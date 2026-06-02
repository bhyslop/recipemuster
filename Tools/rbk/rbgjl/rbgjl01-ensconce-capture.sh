#!/bin/bash
# RBGJL Step 01: Ensconce a base image into a Lode (capture) via skopeo
# Builder: skopeo (from reliquary)
# Substitutions: _RBGL_GAR_HOST, _RBGL_GAR_PATH, _RBGL_LODES_ROOT,
#                _RBGL_TAG_BOLE, _RBGL_TAG_DIGEST_PREFIX,
#                _RBGL_TRUST_GRADE, _RBGL_VOUCH_SCHEMA, _RBGL_ACQUIRED_BY,
#                _RBGL_IMAGE_1_ORIGIN, _RBGL_IMAGE_2_ORIGIN, _RBGL_IMAGE_3_ORIGIN,
#                _RBGL_LODE_1_STAMP,   _RBGL_LODE_2_STAMP,   _RBGL_LODE_3_STAMP
#
# For each non-empty (ORIGIN, STAMP) slot: inspect upstream, measure the canonical
# digest, copy --all into ONE GAR package rbi_ld/<stamp>, then apply the member
# tags by GAR->GAR retag (dedups blobs). Author the provenance envelope and stage
# it for step 02 (the :rbi_vouch artifact) and for the host capture-file via
# /builder/outputs/output. Mason SA ambient auth via Cloud Build metadata server.
#
# Package shape:  <host>/<path>/<LODES_ROOT>/<stamp>            (one package = one Lode)
# Member tags on that package, all pointing at the base manifest:
#   :<TAG_DIGEST_PREFIX><full-hex>   canonical OCI digest (exact cross-Lode dedup)
#   :<TAG_BOLE>                      uniform greppable handle
#   :<sanitized-origin>-<sha10>      UNSPRUED — name + glance-fingerprint (= legacy enshrine anchor)
# The :rbi_vouch tag is a separate manifest pushed by step 02.

set -euo pipefail
echo "=== Ensconce base images into Lodes ==="

# Obtain OAuth2 token from metadata server (Mason SA)
echo "Fetching OAuth2 token from metadata server"
TOKEN_JSON=$(curl -sf -H "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token") \
  || { echo "Failed to fetch OAuth2 token from metadata server" >&2; exit 1; }

TOKEN=$(printf '%s' "${TOKEN_JSON}" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
test -n "${TOKEN}" || { echo "Failed to extract access_token from metadata response" >&2; exit 1; }

# CB substitutions are expanded at submit time, not available as shell variables.
# Capture each into a runtime variable so we can loop.
SLOT_1_ORIGIN="${_RBGL_IMAGE_1_ORIGIN}"
SLOT_2_ORIGIN="${_RBGL_IMAGE_2_ORIGIN}"
SLOT_3_ORIGIN="${_RBGL_IMAGE_3_ORIGIN}"
SLOT_1_STAMP="${_RBGL_LODE_1_STAMP}"
SLOT_2_STAMP="${_RBGL_LODE_2_STAMP}"
SLOT_3_STAMP="${_RBGL_LODE_3_STAMP}"

# Acquisition moment, attested once for the whole build.
ACQUIRED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)

# Stamp roster for step 02 (one line per captured Lode).
: > /workspace/lode_stamps.txt

# Initialize result JSON (host reads this back as the capture-file source)
RESULT='{'
FIRST=true

for SLOT in 1 2 3; do
  case "${SLOT}" in
    1) ORIGIN="${SLOT_1_ORIGIN}"; STAMP="${SLOT_1_STAMP}" ;;
    2) ORIGIN="${SLOT_2_ORIGIN}"; STAMP="${SLOT_2_STAMP}" ;;
    3) ORIGIN="${SLOT_3_ORIGIN}"; STAMP="${SLOT_3_STAMP}" ;;
  esac
  test -n "${ORIGIN}" || continue
  test -n "${STAMP}"  || { echo "FATAL: slot ${SLOT} has ORIGIN but no STAMP" >&2; exit 1; }

  echo "--- Slot ${SLOT}: ${ORIGIN} -> ${_RBGL_LODES_ROOT}/${STAMP} ---"

  # Inspect upstream for raw manifest (manifest list or single manifest).
  RAW_FILE="/workspace/ensconce_raw_${SLOT}.json"
  skopeo inspect --raw "docker://${ORIGIN}" > "${RAW_FILE}" \
    || { echo "FATAL: Failed to inspect upstream: ${ORIGIN}" >&2; exit 1; }

  # Canonical digest = sha256 of the raw manifest (matches what every tool reports).
  # sha256sum (coreutils) — this runs inside the skopeo reliquary container.
  SHA=$(sha256sum "${RAW_FILE}" | cut -d' ' -f1)
  test -n "${SHA}" || { echo "FATAL: Empty digest for slot ${SLOT}" >&2; exit 1; }

  # Sanitize origin (: and / become -), append first 10 hex chars (legacy anchor form).
  SANITIZED=$(printf '%s' "${ORIGIN}" | tr ':/' '--')
  SHORT="${SHA:0:10}"
  FINGERPRINT_TAG="${SANITIZED}-${SHORT}"

  DIGEST_TAG="${_RBGL_TAG_DIGEST_PREFIX}${SHA}"
  PKG="${_RBGL_GAR_HOST}/${_RBGL_GAR_PATH}/${_RBGL_LODES_ROOT}/${STAMP}"

  echo "Package: ${PKG}"
  echo "Digest:  sha256:${SHA}"
  echo "Tags:    ${DIGEST_TAG}, ${_RBGL_TAG_BOLE}, ${FINGERPRINT_TAG}"

  # Copy upstream into the Lode package under the canonical digest tag.
  skopeo copy --all \
    "docker://${ORIGIN}" \
    "docker://${PKG}:${DIGEST_TAG}" \
    --dest-creds "oauth2accesstoken:${TOKEN}" \
    || { echo "FATAL: skopeo copy failed for slot ${SLOT}" >&2; exit 1; }

  # Apply remaining member tags by GAR->GAR retag (same blobs, manifest re-tag only).
  for MEMBER_TAG in "${_RBGL_TAG_BOLE}" "${FINGERPRINT_TAG}"; do
    skopeo copy --all \
      "docker://${PKG}:${DIGEST_TAG}" \
      "docker://${PKG}:${MEMBER_TAG}" \
      --src-creds  "oauth2accesstoken:${TOKEN}" \
      --dest-creds "oauth2accesstoken:${TOKEN}" \
      || { echo "FATAL: skopeo retag ${MEMBER_TAG} failed for slot ${SLOT}" >&2; exit 1; }
  done

  echo "Slot ${SLOT} ensconced: ${STAMP}"

  # Author the provenance envelope (identical content lands in :rbi_vouch and the
  # host capture-file). No jq dependency — values are controlled (sanitized origin,
  # hex digest, SA email, build id, ISO timestamp); none can carry a literal quote.
  # members[] is the cardinality axis — length 1 for the bole singleton.
  ENVELOPE='{'
  ENVELOPE="${ENVELOPE}\"schema\":\"${_RBGL_VOUCH_SCHEMA}\","
  ENVELOPE="${ENVELOPE}\"kind\":\"bole\","
  ENVELOPE="${ENVELOPE}\"lode\":\"${STAMP}\","
  ENVELOPE="${ENVELOPE}\"acquired_at\":\"${ACQUIRED_AT}\","
  ENVELOPE="${ENVELOPE}\"acquired_by\":\"${_RBGL_ACQUIRED_BY}\","
  ENVELOPE="${ENVELOPE}\"capture_build\":\"${BUILD_ID:-}\","
  ENVELOPE="${ENVELOPE}\"trust_grade\":\"${_RBGL_TRUST_GRADE}\","
  ENVELOPE="${ENVELOPE}\"signature\":null,"
  ENVELOPE="${ENVELOPE}\"members\":[{"
  ENVELOPE="${ENVELOPE}\"name\":\"${_RBGL_TAG_BOLE}\","
  ENVELOPE="${ENVELOPE}\"origin\":\"${ORIGIN}\","
  ENVELOPE="${ENVELOPE}\"digest\":\"sha256:${SHA}\","
  ENVELOPE="${ENVELOPE}\"verification\":\"oci-digest\","
  ENVELOPE="${ENVELOPE}\"tags\":[\"${_RBGL_TAG_BOLE}\",\"${DIGEST_TAG}\",\"${FINGERPRINT_TAG}\"]"
  ENVELOPE="${ENVELOPE}}]}"

  # Stage the envelope for step 02 (pushes it as the :rbi_vouch artifact).
  printf '%s' "${ENVELOPE}" > "/workspace/lode_${STAMP}_vouch.json"
  echo "${STAMP}" >> /workspace/lode_stamps.txt

  # Accumulate host-facing result (the capture-file carries the same envelope).
  if [ "${FIRST}" = "true" ]; then
    FIRST=false
  else
    RESULT="${RESULT},"
  fi
  RESULT="${RESULT}\"slot_${SLOT}\":{\"stamp\":\"${STAMP}\",\"vouch\":${ENVELOPE}}"
done

RESULT="${RESULT}}"

echo "=== Writing capture results ==="
echo "${RESULT}"

# Write to buildStepOutputs channel (host extracts per-Lode envelope -> capture-file)
mkdir -p /builder/outputs
printf '%s' "${RESULT}" > /builder/outputs/output

echo "=== Ensconce capture step complete ==="
