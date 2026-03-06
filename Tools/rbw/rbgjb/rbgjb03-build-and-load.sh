#!/bin/bash
# RBGJB Step 03: Build single-arch image with dual tags and load into local Docker daemon
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_IMAGE,
#                _RBGY_INSCRIBE_TIMESTAMP, _RBGY_PLATFORM_SUFFIXES,
#                _RBGY_GIT_COMMIT, _RBGY_GIT_BRANCH
#
# Builds a single-platform image and loads it into the local Docker daemon
# via --load. Tags the image with BOTH the per-platform suffixed tag and the
# bare consumer tag. The images: field in cloudbuild.json lists both tags so
# Cloud Build pushes both to GAR and generates SLSA v1.0 provenance for each.
#
# Image tags use _RBGY_INSCRIBE_TIMESTAMP (CB substitution) so the images:
# field can reference them statically. TAG_BASE (inscribe + build timestamp)
# is used only for metadata.

set -euo pipefail

# Required inputs
test -n "${_RBGY_DOCKERFILE}"          || (echo "_RBGY_DOCKERFILE missing"          >&2; exit 1)
test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_INSCRIBE_TIMESTAMP}"  || (echo "_RBGY_INSCRIBE_TIMESTAMP missing"  >&2; exit 1)
test -n "${_RBGY_PLATFORM_SUFFIXES}"   || (echo "_RBGY_PLATFORM_SUFFIXES missing"   >&2; exit 1)
test -n "${_RBGY_GIT_COMMIT}"          || (echo "_RBGY_GIT_COMMIT missing"          >&2; exit 1)
test -n "${_RBGY_GIT_BRANCH}"          || (echo "_RBGY_GIT_BRANCH missing"          >&2; exit 1)

# Per-platform suffixed tag (for SLSA provenance via images: field)
PLATFORM_SUFFIX="${_RBGY_PLATFORM_SUFFIXES}"  # single value for single-platform
IMAGE_URI_SUFFIXED="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}${PLATFORM_SUFFIX}"

# Bare consumer tag (also pushed via images: field)
IMAGE_URI_BARE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${_RBGY_INSCRIBE_TIMESTAMP}${_RBGY_ARK_SUFFIX_IMAGE}"

docker buildx version
docker version

# Create docker-container driver for cross-arch builds (QEMU emulation)
# Harmless for native-arch builds; required when platform != host arch
docker buildx create --driver docker-container --name rb-builder --use

# Build single-platform image and load into local Docker daemon
# - --load puts the image in the local daemon for CB images: field
# - Two tags: suffixed (SLSA provenance) + bare (consumer-facing)
# - Single platform only (stitch validates single-arch at inscribe time)
# - CB-native push via images: field generates SLSA v1.0 provenance for both tags
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
