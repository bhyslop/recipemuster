#!/bin/bash
# RBGJBM Step 09: Assemble consumer-facing multi-platform manifest list
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE,
#                _RBGY_INSCRIBE_TIMESTAMP
#
# Uses docker buildx imagetools create to assemble per-platform tags
# (pushed in step 05) into a single consumer-facing manifest list tag.
# The consumer tag is platform-transparent: identical format to single-arch.
#
# This runs within the same Cloud Build — experiment 6 (build 6661d0cd)
# proved imagetools create works mid-build after docker push.

set -euo pipefail

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_INSCRIBE_TIMESTAMP}"  || (echo "_RBGY_INSCRIBE_TIMESTAMP missing"  >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_IMAGE}"    || (echo "_RBGY_ARK_SUFFIX_IMAGE missing"    >&2; exit 1)

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"
CONSUMER_TAG="${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}"

IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

# Build source image list for imagetools create
SOURCE_IMAGES=""
for SUFFIX in "${SUFFIXES[@]}"; do
  PER_PLAT_TAG="${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  if test -n "${SOURCE_IMAGES}"; then
    SOURCE_IMAGES="${SOURCE_IMAGES} ${IMAGE_BASE}:${PER_PLAT_TAG}"
  else
    SOURCE_IMAGES="${IMAGE_BASE}:${PER_PLAT_TAG}"
  fi
done

echo "=== Creating multi-platform manifest from per-platform images ==="
echo "Consumer tag: ${IMAGE_BASE}:${CONSUMER_TAG}"
echo "Sources: ${SOURCE_IMAGES}"

# shellcheck disable=SC2086
docker buildx imagetools create \
  -t "${IMAGE_BASE}:${CONSUMER_TAG}" \
  ${SOURCE_IMAGES}

echo "=== Inspecting combined manifest ==="
docker buildx imagetools inspect "${IMAGE_BASE}:${CONSUMER_TAG}"
