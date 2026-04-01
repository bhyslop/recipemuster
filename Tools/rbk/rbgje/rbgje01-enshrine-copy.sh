#!/bin/bash
# RBGJE Step 01: Enshrine upstream base images to GAR via skopeo
# Builder: skopeo (from reliquary)
# Substitutions: _RBGE_GAR_HOST, _RBGE_GAR_PATH,
#                _RBGE_IMAGE_1_ORIGIN, _RBGE_IMAGE_2_ORIGIN, _RBGE_IMAGE_3_ORIGIN
#
# For each non-empty ORIGIN slot: inspect upstream manifest, compute anchor,
# skopeo copy --all to GAR. Write JSON anchor results to /builder/outputs/output.
# Mason SA ambient auth via Cloud Build metadata server for GAR destination.

set -euo pipefail
echo "=== Enshrine base images to GAR ==="

# Obtain OAuth2 token from metadata server (Mason SA)
echo "Fetching OAuth2 token from metadata server"
TOKEN_JSON=$(curl -sf -H "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token") \
  || { echo "Failed to fetch OAuth2 token from metadata server" >&2; exit 1; }

TOKEN=$(printf '%s' "${TOKEN_JSON}" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
test -n "${TOKEN}" || { echo "Failed to extract access_token from metadata response" >&2; exit 1; }

# CB substitutions are expanded at submit time, not available as shell variables.
# Capture each into a runtime variable so we can loop.
SLOT_1_ORIGIN="${_RBGE_IMAGE_1_ORIGIN}"
SLOT_2_ORIGIN="${_RBGE_IMAGE_2_ORIGIN}"
SLOT_3_ORIGIN="${_RBGE_IMAGE_3_ORIGIN}"

# Initialize result JSON
RESULT='{'
FIRST=true

for SLOT in 1 2 3; do
  case "${SLOT}" in
    1) ORIGIN="${SLOT_1_ORIGIN}" ;;
    2) ORIGIN="${SLOT_2_ORIGIN}" ;;
    3) ORIGIN="${SLOT_3_ORIGIN}" ;;
  esac
  test -n "${ORIGIN}" || continue

  echo "--- Slot ${SLOT}: ${ORIGIN} ---"

  # Inspect upstream for raw manifest (manifest list or single manifest)
  RAW_FILE="/workspace/enshrine_raw_${SLOT}.json"
  skopeo inspect --raw "docker://${ORIGIN}" > "${RAW_FILE}" \
    || { echo "FATAL: Failed to inspect upstream: ${ORIGIN}" >&2; exit 1; }

  # Compute sha256 of the raw manifest
  SHA=$(openssl dgst -sha256 -r "${RAW_FILE}") || { echo "FATAL: Hash failed for slot ${SLOT}" >&2; exit 1; }
  read -r SHA _ <<< "${SHA}"
  test -n "${SHA}" || { echo "FATAL: Empty digest for slot ${SLOT}" >&2; exit 1; }

  # Construct anchor: sanitize origin (: and / become -), append first 10 hex chars
  SANITIZED=$(printf '%s' "${ORIGIN}" | tr ':/' '--')
  SHORT="${SHA:0:10}"
  ANCHOR="${SANITIZED}-${SHORT}"

  echo "Anchor: ${ANCHOR}"
  echo "Digest: sha256:${SHA}"

  # Construct GAR destination
  DEST_REF="${_RBGE_GAR_HOST}/${_RBGE_GAR_PATH}/enshrine:${ANCHOR}"
  echo "Dest: ${DEST_REF}"

  # Copy upstream to GAR with anchor tag, preserving manifest list
  skopeo copy --all \
    "docker://${ORIGIN}" \
    "docker://${DEST_REF}" \
    --dest-creds "oauth2accesstoken:${TOKEN}" \
    || { echo "FATAL: skopeo copy failed for slot ${SLOT}" >&2; exit 1; }

  echo "Slot ${SLOT} enshrined: ${ANCHOR}"

  # Accumulate JSON result (no jq dependency — printf safe for sanitized anchor values)
  if [ "${FIRST}" = "true" ]; then
    FIRST=false
  else
    RESULT="${RESULT},"
  fi
  RESULT="${RESULT}\"slot_${SLOT}\":{\"anchor\":\"${ANCHOR}\",\"origin\":\"${ORIGIN}\",\"digest\":\"sha256:${SHA}\"}"
done

RESULT="${RESULT}}"

echo "=== Writing anchor results ==="
echo "${RESULT}"

# Write to buildStepOutputs channel
mkdir -p /builder/outputs
printf '%s' "${RESULT}" > /builder/outputs/output

echo "=== Enshrine step complete ==="
