#!/bin/bash
# RBGJB Step 03: Build single-arch image with dual tags and load into local Docker daemon
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE,
#                _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GIT_COMMIT, _RBGY_GIT_BRANCH
#
# Builds a single-platform image, loads it into the local Docker daemon
# via --load, then pushes both tags to GAR explicitly.
# Tags the image with BOTH the per-platform suffixed tag and the
# bare consumer tag, using TAG_BASE (full consecration).

set -euo pipefail

# Required inputs
test -n "${_RBGY_DOCKERFILE}"          || (echo "_RBGY_DOCKERFILE missing"          >&2; exit 1)
test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GIT_COMMIT}"          || (echo "_RBGY_GIT_COMMIT missing"          >&2; exit 1)
test -n "${_RBGY_GIT_BRANCH}"          || (echo "_RBGY_GIT_BRANCH missing"          >&2; exit 1)

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

# Per-platform suffixed tag
PLATFORM_SUFFIX="${_RBGY_PLATFORM_SUFFIXES}"  # single value for single-platform
IMAGE_URI_SUFFIXED="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}${PLATFORM_SUFFIX}"

# Bare consumer tag
IMAGE_URI_BARE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

docker buildx version
docker version

# Create docker-container driver for cross-arch builds (QEMU emulation)
# Harmless for native-arch builds; required when platform != host arch
docker buildx create --driver docker-container --name rb-builder --use

# Build single-platform image and load into local Docker daemon
# - --load puts the image in the local daemon for syft scanning (step 04)
# - Two tags: suffixed + bare (consumer-facing)
# - Single platform only (stitch validates single-arch at inscribe time)
# - Explicit push after load (images: field removed since TAG_BASE is runtime-derived)
docker buildx build \
  --platform="${_RBGY_PLATFORMS}" \
  --tag "${IMAGE_URI_SUFFIXED}" \
  --tag "${IMAGE_URI_BARE}" \
  --load \
  --label "moniker=${_RBGY_MONIKER}" \
  --label "git.commit=${_RBGY_GIT_COMMIT}" \
  --label "git.branch=${_RBGY_GIT_BRANCH}" \
  -f "${_RBGY_DOCKERFILE}" \
  .

echo "Image loaded with tags: ${IMAGE_URI_SUFFIXED} and ${IMAGE_URI_BARE}"

# Push both tags explicitly (previously handled by CB images: field)
docker push "${IMAGE_URI_SUFFIXED}"
docker push "${IMAGE_URI_BARE}"
echo "Both tags pushed to registry"
