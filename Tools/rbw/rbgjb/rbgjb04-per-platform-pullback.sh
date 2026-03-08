#!/bin/bash
# RBGJBM Step 04: Per-platform pullback from intermediate -multi tag
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORMS, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE,
#                _RBGY_INSCRIBE_TIMESTAMP
#
# For each platform: docker pull --platform <plat> from the -multi tag,
# then docker tag to the per-platform consecration tag and the inscribe-time
# alias tag (for CB images: field SLSA provenance generation).

set -euo pipefail

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_INSCRIBE_TIMESTAMP}"  || (echo "_RBGY_INSCRIBE_TIMESTAMP missing"  >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_IMAGE}"    || (echo "_RBGY_ARK_SUFFIX_IMAGE missing"    >&2; exit 1)

test -s .consecration || (echo "consecration not derived" >&2; exit 1)
CONSECRATION="$(cat .consecration)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"
MULTI_TAG="${_RBGY_INSCRIBE_TIMESTAMP}-multi"

# Split platforms and suffixes into parallel arrays
# _RBGY_PLATFORMS is comma-separated: linux/amd64,linux/arm64,linux/arm/v7
# _RBGY_PLATFORM_SUFFIXES is comma-separated: -amd64,-arm64,-armv7
IFS=',' read -ra PLATFORMS <<< "${_RBGY_PLATFORMS}"
IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

test "${#PLATFORMS[@]}" -eq "${#SUFFIXES[@]}" \
  || (echo "Platform/suffix count mismatch: ${#PLATFORMS[@]} vs ${#SUFFIXES[@]}" >&2; exit 1)

echo "=== Per-platform pullback from ${IMAGE_BASE}:${MULTI_TAG} ==="
for IDX in "${!PLATFORMS[@]}"; do
  PLAT="${PLATFORMS[${IDX}]}"
  SUFFIX="${SUFFIXES[${IDX}]}"
  PER_PLAT_TAG="${CONSECRATION}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  SLSA_ALIAS_TAG="${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"

  echo "--- Pulling ${PLAT} ---"
  docker pull --platform "${PLAT}" "${IMAGE_BASE}:${MULTI_TAG}"
  docker tag "${IMAGE_BASE}:${MULTI_TAG}" "${IMAGE_BASE}:${PER_PLAT_TAG}"
  echo "Tagged: ${IMAGE_BASE}:${PER_PLAT_TAG}"

  # Alias tag for CB images: field — inscribe-time-predictable, triggers SLSA provenance
  docker tag "${IMAGE_BASE}:${MULTI_TAG}" "${IMAGE_BASE}:${SLSA_ALIAS_TAG}"
  echo "SLSA alias: ${IMAGE_BASE}:${SLSA_ALIAS_TAG}"
done

echo "=== Pullback complete ==="
