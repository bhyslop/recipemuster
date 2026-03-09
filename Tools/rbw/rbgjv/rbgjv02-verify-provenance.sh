#!/bin/bash
# RBGJV Step 02: Discover per-platform digests, verify SLSA provenance, compose summary
# Builder: gcloud (via RBRG_GCLOUD_IMAGE_REF)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL,
#                _RBGV_CONSECRATION, _RBGV_SOURCE_URI,
#                _RBGV_VERIFIER_URL, _RBGV_VERIFIER_SHA256

set -euo pipefail
echo "=== Discover digests and verify provenance ==="

FULL_IMAGE="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
IMAGE_TAG="${_RBGV_CONSECRATION}-image"
BUILDER_ID="https://cloudbuild.googleapis.com/GoogleHostedWorker"

echo "Fetching manifest list: ${IMAGE_TAG}"
gcloud artifacts docker images describe "${FULL_IMAGE}:${IMAGE_TAG}" \
  --format json > /workspace/image_describe.json
DIGEST_COUNT=$(jq '.image_summary.media_type' /workspace/image_describe.json | grep -c "manifest.list\|image.index" || true)

# Extract per-platform digests from the manifest list via registry API
TOKEN=$(gcloud auth print-access-token)
MANIFEST_URL="https://${_RBGV_GAR_HOST}/v2/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}/manifests/${IMAGE_TAG}"
curl -sS \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json" \
  "${MANIFEST_URL}" > /workspace/manifest_list.json

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
  FULL_REF="${FULL_IMAGE}@${DIGEST}"
  echo "Verifying ${PLAT_SUFFIX} (${DIGEST})..."

  echo "  Fetching provenance for ${PLAT_SUFFIX}..."
  gcloud artifacts docker images describe "${FULL_REF}" \
    --format json --show-provenance \
    > "/workspace/provenance-${PLAT_SUFFIX}.json"

  echo "  Running slsa-verifier for ${PLAT_SUFFIX}..."
  /workspace/slsa-verifier verify-image "${FULL_REF}" \
    --provenance-path "/workspace/provenance-${PLAT_SUFFIX}.json" \
    --source-uri "${_RBGV_SOURCE_URI}" \
    --builder-id="${BUILDER_ID}" \
    --print-provenance \
    > "/workspace/verify-${PLAT_SUFFIX}.json"
  echo "  Platform ${PLAT_SUFFIX} verified"

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
