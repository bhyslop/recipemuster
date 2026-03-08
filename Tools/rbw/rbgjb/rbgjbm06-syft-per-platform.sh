#!/bin/bash
# RBGJBM Step 06: Syft SBOM scan for each per-platform image
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORMS, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE
#
# Scans each per-platform image from the local Docker daemon (docker: transport).
# Images were loaded by step 04 (pullback). Produces one SBOM per platform:
#   sbom-{arch}{variant}.json (e.g., sbom-amd64.json, sbom-armv7.json)
# Syft runs as a sibling container sharing the Docker daemon socket.

set -euo pipefail

SYFT_IMAGE="${RBRG_SYFT_IMAGE_REF}"

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_IMAGE}"    || (echo "_RBGY_ARK_SUFFIX_IMAGE missing"    >&2; exit 1)

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"

# Split platforms and suffixes
IFS=',' read -ra PLATFORMS <<< "${_RBGY_PLATFORMS}"
IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

test "${#PLATFORMS[@]}" -eq "${#SUFFIXES[@]}" \
  || (echo "Platform/suffix count mismatch" >&2; exit 1)

echo "=== Per-platform SBOM generation ==="
for IDX in "${!PLATFORMS[@]}"; do
  PLAT="${PLATFORMS[${IDX}]}"
  SUFFIX="${SUFFIXES[${IDX}]}"
  PER_PLAT_TAG="${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  IMAGE_URI="${IMAGE_BASE}:${PER_PLAT_TAG}"

  # Derive SBOM filename: strip leading dash from suffix → sbom-amd64.json
  SBOM_LABEL="${SUFFIX#-}"
  SBOM_FILE="sbom-${SBOM_LABEL}.json"

  echo "--- Scanning ${PLAT} (${IMAGE_URI}) → ${SBOM_FILE} ---"
  docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
    "${SYFT_IMAGE}" "docker:${IMAGE_URI}" -o json > "${SBOM_FILE}" \
    || { echo "Syft JSON generation failed for ${PLAT}" >&2; exit 1; }

  test -s "${SBOM_FILE}" || { echo "SBOM output empty for ${PLAT}" >&2; exit 1; }
  echo "SBOM generated: ${SBOM_FILE}"
done
echo "=== SBOM generation complete ==="
