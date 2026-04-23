#!/bin/bash
# RBGJAM Step 02: Syft SBOM scan for each platform of image
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_HALLMARKS_ROOT,
#                _RBGA_HALLMARK, _RBGA_VESSEL_MODE
#
# Scans each platform of image via registry: transport.
# Two scan modes based on platform count:
#   Single-platform: scan main image tag directly
#   Multi-platform: scan via @digest from manifest list (all modes)
# Auth via GCB metadata server OAuth2 token — no Docker daemon coupling.
# Produces one SBOM per platform: sbom-{arch}{variant}.json

set -euo pipefail

SYFT_IMAGE="${ZRBF_TOOL_SYFT}"

test -n "${_RBGA_GAR_HOST}"        || { echo "_RBGA_GAR_HOST missing"        >&2; exit 1; }
test -n "${_RBGA_GAR_PATH}"        || { echo "_RBGA_GAR_PATH missing"        >&2; exit 1; }
test -n "${_RBGA_HALLMARKS_ROOT}"  || { echo "_RBGA_HALLMARKS_ROOT missing"  >&2; exit 1; }
test -n "${_RBGA_HALLMARK}"        || { echo "_RBGA_HALLMARK missing"        >&2; exit 1; }
test -n "${_RBGA_VESSEL_MODE}"     || { echo "_RBGA_VESSEL_MODE missing"     >&2; exit 1; }

test -s platforms.txt         || { echo "platforms.txt not found (step 01)" >&2; exit 1; }
test -s platform_suffixes.txt || { echo "platform_suffixes.txt not found (step 01)" >&2; exit 1; }
test -s platform_count.txt    || { echo "platform_count.txt not found (step 01)" >&2; exit 1; }

IMAGE_URI="${_RBGA_GAR_HOST}/${_RBGA_GAR_PATH}/${_RBGA_HALLMARKS_ROOT}/${_RBGA_HALLMARK}/image:${_RBGA_HALLMARK}"
PLATFORM_COUNT=$(cat platform_count.txt)
GAR_AUTHORITY="${_RBGA_GAR_HOST}"

# Fetch OAuth2 token from GCB metadata server (no gcloud/jq dependency)
echo "Fetching OAuth2 token from metadata server"
TOKEN_JSON=$(curl -sf -H "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token") \
  || { echo "Failed to fetch OAuth2 token from metadata server" >&2; exit 1; }
TOKEN=$(printf '%s' "${TOKEN_JSON}" | sed 's/.*"access_token":"\([^"]*\)".*/\1/') \
  || { echo "Failed to parse access_token" >&2; exit 1; }
test -n "${TOKEN}" || { echo "OAuth2 token empty" >&2; exit 1; }

# Split platforms and suffixes
IFS=',' read -ra PLATFORMS <<< "$(cat platforms.txt)"
IFS=',' read -ra SUFFIXES <<< "$(cat platform_suffixes.txt)"

test "${#PLATFORMS[@]}" -eq "${#SUFFIXES[@]}" \
  || { echo "Platform/suffix count mismatch" >&2; exit 1; }

# Load per-platform digests (for multi-platform scanning via @digest pinning)
declare -A DIGEST_MAP
if test -f platform_digests.txt; then
  while IFS=' ' read -r D_SUFFIX D_DIGEST; do
    DIGEST_MAP["${D_SUFFIX}"]="${D_DIGEST}"
  done < platform_digests.txt
fi

echo "=== Per-platform SBOM generation (registry transport) ==="
for IDX in "${!PLATFORMS[@]}"; do
  PLAT="${PLATFORMS[${IDX}]}"
  SUFFIX="${SUFFIXES[${IDX}]}"
  SBOM_LABEL="${SUFFIX#-}"
  SBOM_FILE="sbom-${SBOM_LABEL}.json"

  # Determine scan target based on platform count.
  # Multi-platform uses @digest pinning for all modes: OCI indexes may contain
  # attestation manifests (unknown/unknown platform) that cause syft to fail on
  # auto-selection. platform_digests.txt is always written by discover-platforms.
  if test "${PLATFORM_COUNT}" = "1"; then
    SCAN_TARGET="registry:${IMAGE_URI}"
  else
    DIGEST="${DIGEST_MAP[${SUFFIX}]:-}"
    test -n "${DIGEST}" || { echo "No digest found for suffix ${SUFFIX}" >&2; exit 1; }
    SCAN_TARGET="registry:${IMAGE_URI}@${DIGEST}"
  fi

  echo "--- Scanning ${PLAT} (${SCAN_TARGET}) → ${SBOM_FILE} ---"
  docker run --rm \
    -e SYFT_REGISTRY_AUTH_AUTHORITY="${GAR_AUTHORITY}" \
    -e SYFT_REGISTRY_AUTH_USERNAME=oauth2accesstoken \
    -e SYFT_REGISTRY_AUTH_PASSWORD="${TOKEN}" \
    "${SYFT_IMAGE}" "${SCAN_TARGET}" -o json > "${SBOM_FILE}" \
    || { echo "Syft JSON generation failed for ${PLAT}" >&2; exit 1; }

  test -s "${SBOM_FILE}" || { echo "SBOM output empty for ${PLAT}" >&2; exit 1; }
  echo "SBOM generated: ${SBOM_FILE}"
done
echo "=== SBOM generation complete ==="
