#!/bin/bash
# RBGJBM Step 06: Syft SBOM scan for each per-platform image
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORMS, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE
#
# Scans each per-platform image from GAR via registry: transport.
# Images were pushed by step 05. Produces one SBOM per platform:
#   sbom-{arch}{variant}.json (e.g., sbom-amd64.json, sbom-armv7.json)
# Auth via GCB metadata server OAuth2 token — no Docker daemon coupling.

set -euo pipefail

SYFT_IMAGE="${RBRG_SYFT_IMAGE_REF}"

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_IMAGE}"    || (echo "_RBGY_ARK_SUFFIX_IMAGE missing"    >&2; exit 1)
test -n "${_RBGY_GAR_HOST_SUFFIX}"     || (echo "_RBGY_GAR_HOST_SUFFIX missing"     >&2; exit 1)

test -s .consecration || (echo "consecration not derived" >&2; exit 1)
CONSECRATION="$(cat .consecration)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"
GAR_AUTHORITY="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}"

# Fetch OAuth2 token from GCB metadata server (no gcloud/jq dependency)
# Response: {"access_token":"ya29...","expires_in":3600,"token_type":"Bearer"}
echo "Fetching OAuth2 token from metadata server"
TOKEN_JSON=$(curl -sf -H "Metadata-Flavor: Google" \
  "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token") \
  || { echo "Failed to fetch OAuth2 token from metadata server" >&2; exit 1; }
TOKEN=$(printf '%s' "${TOKEN_JSON}" | sed 's/.*"access_token":"\([^"]*\)".*/\1/') \
  || { echo "Failed to parse access_token from metadata response" >&2; exit 1; }
test -n "${TOKEN}" || { echo "OAuth2 token empty" >&2; exit 1; }

# Split platforms and suffixes
IFS=',' read -ra PLATFORMS <<< "${_RBGY_PLATFORMS}"
IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

test "${#PLATFORMS[@]}" -eq "${#SUFFIXES[@]}" \
  || (echo "Platform/suffix count mismatch" >&2; exit 1)

echo "=== Per-platform SBOM generation (registry transport) ==="
for IDX in "${!PLATFORMS[@]}"; do
  PLAT="${PLATFORMS[${IDX}]}"
  SUFFIX="${SUFFIXES[${IDX}]}"
  PER_PLAT_TAG="${CONSECRATION}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  IMAGE_URI="${IMAGE_BASE}:${PER_PLAT_TAG}"

  # Derive SBOM filename: strip leading dash from suffix → sbom-amd64.json
  SBOM_LABEL="${SUFFIX#-}"
  SBOM_FILE="sbom-${SBOM_LABEL}.json"

  echo "--- Scanning ${PLAT} (${IMAGE_URI}) → ${SBOM_FILE} ---"
  docker run --rm \
    -e SYFT_REGISTRY_AUTH_AUTHORITY="${GAR_AUTHORITY}" \
    -e SYFT_REGISTRY_AUTH_USERNAME=oauth2accesstoken \
    -e SYFT_REGISTRY_AUTH_PASSWORD="${TOKEN}" \
    "${SYFT_IMAGE}" "registry:${IMAGE_URI}" -o json > "${SBOM_FILE}" \
    || { echo "Syft JSON generation failed for ${PLAT}" >&2; exit 1; }

  test -s "${SBOM_FILE}" || { echo "SBOM output empty for ${PLAT}" >&2; exit 1; }
  echo "SBOM generated: ${SBOM_FILE}"
done
echo "=== SBOM generation complete ==="
