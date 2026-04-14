#!/bin/bash
# RBGJB Step 04: Per-platform pullback from consumer -image manifest list
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORMS, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE,
#                _RBGY_ARK_SUFFIX_ATTEST
#
# For each platform: docker pull --platform <plat> from the -image manifest,
# then docker tag to {hallmark}-attest-{arch} (ephemeral attestation scaffolding
# for CB images: field SLSA provenance generation).

set -euo pipefail

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_IMAGE}"    || (echo "_RBGY_ARK_SUFFIX_IMAGE missing"    >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_ATTEST}"   || (echo "_RBGY_ARK_SUFFIX_ATTEST missing"   >&2; exit 1)

test -s .hallmark || (echo "hallmark not derived" >&2; exit 1)
HALLMARK="$(cat .hallmark)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"
IMAGE_TAG="${HALLMARK}${_RBGY_ARK_SUFFIX_IMAGE}"

# Split platforms and suffixes into parallel arrays
# _RBGY_PLATFORMS is comma-separated: linux/amd64,linux/arm64,linux/arm/v7
# _RBGY_PLATFORM_SUFFIXES is comma-separated: -amd64,-arm64,-armv7
IFS=',' read -ra PLATFORMS <<< "${_RBGY_PLATFORMS}"
IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

test "${#PLATFORMS[@]}" -eq "${#SUFFIXES[@]}" \
  || (echo "Platform/suffix count mismatch: ${#PLATFORMS[@]} vs ${#SUFFIXES[@]}" >&2; exit 1)

echo "=== Per-platform pullback from ${IMAGE_BASE}:${IMAGE_TAG} ==="
for IDX in "${!PLATFORMS[@]}"; do
  PLAT="${PLATFORMS[${IDX}]}"
  SUFFIX="${SUFFIXES[${IDX}]}"
  ATTEST_TAG="${HALLMARK}${_RBGY_ARK_SUFFIX_ATTEST}${SUFFIX}"

  echo "--- Pulling ${PLAT} ---"
  docker pull --platform "${PLAT}" "${IMAGE_BASE}:${IMAGE_TAG}"
  docker tag "${IMAGE_BASE}:${IMAGE_TAG}" "${IMAGE_BASE}:${ATTEST_TAG}"
  echo "Tagged: ${IMAGE_BASE}:${ATTEST_TAG}"
done

echo "=== Pullback complete ==="
