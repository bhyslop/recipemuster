#!/bin/bash
# RBGJBM Step 05: Push each per-platform tag to registry
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE
#
# Pre-pushes each per-platform tag so that imagetools create (step 09)
# can reference them as registry-resident images.

set -euo pipefail

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_IMAGE}"    || (echo "_RBGY_ARK_SUFFIX_IMAGE missing"    >&2; exit 1)

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"

IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

echo "=== Pushing per-platform tags to registry ==="
for SUFFIX in "${SUFFIXES[@]}"; do
  PER_PLAT_TAG="${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  echo "Pushing: ${IMAGE_BASE}:${PER_PLAT_TAG}"
  docker push "${IMAGE_BASE}:${PER_PLAT_TAG}"
done
echo "=== Push complete ==="
