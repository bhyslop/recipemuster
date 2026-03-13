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

test -s .consecration || (echo "consecration not derived" >&2; exit 1)
CONSECRATION="$(cat .consecration)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"

IFS=',' read -ra SUFFIXES <<< "${_RBGY_PLATFORM_SUFFIXES}"

echo "=== Pushing per-platform tags to registry ==="
for SUFFIX in "${SUFFIXES[@]}"; do
  PER_PLAT_TAG="${CONSECRATION}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  echo "Pushing: ${IMAGE_BASE}:${PER_PLAT_TAG}"
  docker push "${IMAGE_BASE}:${PER_PLAT_TAG}"
done
echo "=== Push complete ==="

# Snapshot host daemon cache state after all builds and pushes
# Everything in cache_after that was not in cache_before was pulled for THIS build.
echo "=== Capturing cache_after.json ==="
{
  printf '{"timestamp":"%s","host_daemon_images":[' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  docker images --no-trunc --format '{{json .}}' 2>/dev/null \
    | awk '{if(NR>1) printf ","; print}' || true
  printf ']}'
} > cache_after.json
