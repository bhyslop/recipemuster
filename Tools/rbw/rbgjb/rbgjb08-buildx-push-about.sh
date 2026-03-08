#!/bin/bash
# RBGJBM Step 08: Build and push multi-platform metadata (-about) container
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_ABOUT
#
# Builds a multi-platform FROM scratch container where TARGETARCH/TARGETVARIANT
# auto-args select the per-platform SBOM and build_info files. Uses the buildx
# builder instance created in step 03.
#
# File naming convention (matches step 06/07 outputs):
#   sbom-{arch}{variant}.json       → /sbom.json
#   build_info-{arch}{variant}.json → /build_info.json
#
# No QEMU needed: scratch images have no executables — just file copies with
# platform annotations.

set -euo pipefail

test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_ARK_SUFFIX_ABOUT}"    || (echo "_RBGY_ARK_SUFFIX_ABOUT missing"    >&2; exit 1)

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

META_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_ABOUT}"

# Generate multi-platform Dockerfile.meta
# TARGETARCH and TARGETVARIANT are automatic buildx args
{
  echo 'FROM scratch'
  echo 'ARG TARGETARCH'
  echo 'ARG TARGETVARIANT'
  echo 'LABEL org.opencontainers.image.title="rbia-metadata"'
  echo 'COPY sbom-${TARGETARCH}${TARGETVARIANT}.json /sbom.json'
  echo 'COPY build_info-${TARGETARCH}${TARGETVARIANT}.json /build_info.json'
  echo 'COPY recipe.txt /recipe.txt'
} > Dockerfile.meta

echo "=== Building multi-platform -about container ==="
echo "Platforms: ${_RBGY_PLATFORMS}"
echo "Target: ${META_URI}"

# Re-use the builder created in step 03 (rb-builder)
# If it was garbage-collected, recreate it
docker buildx inspect rb-builder >/dev/null 2>&1 \
  || docker buildx create --driver docker-container --name rb-builder
docker buildx use rb-builder

docker buildx build \
  --push \
  --platform="${_RBGY_PLATFORMS}" \
  --tag "${META_URI}" \
  -f Dockerfile.meta \
  .

echo "Multi-platform -about pushed: ${META_URI}"
