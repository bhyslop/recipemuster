#!/bin/sh
# RBGJV Step 02: Discover per-platform digests, verify SLSA provenance, compose summary
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL,
#                _RBGV_CONSECRATION, _RBGV_SOURCE_URI,
#                _RBGV_VERIFIER_URL, _RBGV_VERIFIER_SHA256

set -eu
echo "=== Discover digests and verify provenance ==="
apk add --no-cache jq >/dev/null
echo "Authenticating to GAR via metadata server..."
wget -q -O /workspace/token_response.json --header "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token"
TOKEN=$(jq -r '.access_token' /workspace/token_response.json)
test -n "${TOKEN}" || { echo "FATAL: empty metadata server token" >&2; exit 1; }
mkdir -p /root/.docker
AUTH_B64=$(printf "oauth2accesstoken:%s" "${TOKEN}" | base64 | tr -d '\n')
printf '{"auths":{"%s":{"auth":"%s"}}}' "${_RBGV_GAR_HOST}" "${AUTH_B64}" \
  > /root/.docker/config.json
IMAGE_TAG="${_RBGV_CONSECRATION}-image"
MANIFEST_URL="https://${_RBGV_GAR_HOST}/v2/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}/manifests/${IMAGE_TAG}"
echo "Fetching manifest list: ${IMAGE_TAG}"
wget -q -O /workspace/manifest_list.json \
  --header "Authorization: Bearer ${TOKEN}" \
  --header "Accept: application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json" \
  "${MANIFEST_URL}"
DIGEST_COUNT=$(jq '.manifests | length' /workspace/manifest_list.json)
echo "Found ${DIGEST_COUNT} platform entries"
test "${DIGEST_COUNT}" -gt 0 || { echo "FATAL: no platform entries in manifest list" >&2; exit 1; }
IDX=0
while [ "${IDX}" -lt "${DIGEST_COUNT}" ]; do
  DIGEST=$(jq -r ".manifests[${IDX}].digest" /workspace/manifest_list.json)
  ARCH=$(jq -r ".manifests[${IDX}].platform.architecture" /workspace/manifest_list.json)
  VARIANT=$(jq -r ".manifests[${IDX}].platform.variant // empty" /workspace/manifest_list.json)
  if [ -n "${VARIANT}" ]; then
    PLAT_SUFFIX="${ARCH}${VARIANT}"
  else
    PLAT_SUFFIX="${ARCH}"
  fi
  FULL_REF="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}@${DIGEST}"
  echo "Verifying ${PLAT_SUFFIX} (${DIGEST})..."
  echo "  ref:        ${FULL_REF}"
  echo "  source-uri: ${_RBGV_SOURCE_URI}"
  if ! /workspace/slsa-verifier verify-image "${FULL_REF}" \
    --source-uri "${_RBGV_SOURCE_URI}" \
    --print-provenance \
    > "/workspace/verify-${PLAT_SUFFIX}.json" 2>/workspace/verify-error.txt; then
    echo "VERIFY FAILED for ${PLAT_SUFFIX}:" >&2
    cat /workspace/verify-error.txt >&2
    exit 1
  fi
  echo "Platform ${PLAT_SUFFIX} verified"
  IDX=$((IDX + 1))
done
echo "All ${DIGEST_COUNT} platforms verified"
echo "Composing vouch summary..."
VERDICTS="[]"
for f in /workspace/verify-*.json; do
  PLAT=$(basename "${f}" .json)
  PLAT="${PLAT#verify-}"
  VERDICTS=$(echo "${VERDICTS}" | jq --arg p "${PLAT}" '. + [{"platform": $p, "verdict": "pass"}]')
done
jq -n \
  --arg consecration "${_RBGV_CONSECRATION}" \
  --arg vessel "${_RBGV_VESSEL}" \
  --arg verifier_url "${_RBGV_VERIFIER_URL}" \
  --arg verifier_sha "${_RBGV_VERIFIER_SHA256}" \
  --argjson platforms "${VERDICTS}" \
  '{
    consecration: $consecration,
    vessel: $vessel,
    verifier: { url: $verifier_url, sha256: $verifier_sha },
    platforms: $platforms
  }' > /workspace/vouch_summary.json
echo "Vouch summary composed"
