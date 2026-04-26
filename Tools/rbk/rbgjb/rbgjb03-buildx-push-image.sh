#!/bin/bash
# RBGJB Step 03: Build all platforms and push consumer-facing image to GAR
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_DOCKERFILE, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_HALLMARKS_ROOT, _RBGY_HALLMARK,
#                _RBGY_GIT_COMMIT, _RBGY_GIT_BRANCH,
#                _RBGY_ARK_BASENAME_IMAGE,
#                _RBGY_IMAGE_1, _RBGY_IMAGE_2, _RBGY_IMAGE_3 (optional base image refs)
#
# Uses buildx --push with docker-container driver. Cloud Build pre-populates
# Docker credentials in the host daemon; buildx inherits them via the
# docker-container driver's config propagation.
#
# Image URI shape: <host>/<project>/<repo>/<HALLMARKS_ROOT>/<HALLMARK>/<image-basename>:<HALLMARK>

set -euo pipefail

# Required inputs
test -n "${_RBGY_DOCKERFILE}"          || (echo "_RBGY_DOCKERFILE missing"          >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_HALLMARKS_ROOT}"      || (echo "_RBGY_HALLMARKS_ROOT missing"      >&2; exit 1)
test -n "${_RBGY_HALLMARK}"            || (echo "_RBGY_HALLMARK missing"            >&2; exit 1)
test -n "${_RBGY_GIT_COMMIT}"          || (echo "_RBGY_GIT_COMMIT missing"          >&2; exit 1)
test -n "${_RBGY_GIT_BRANCH}"          || (echo "_RBGY_GIT_BRANCH missing"          >&2; exit 1)
test -n "${_RBGY_ARK_BASENAME_IMAGE}"  || (echo "_RBGY_ARK_BASENAME_IMAGE missing"  >&2; exit 1)

# Resolve base image build-args (anchored GAR refs or upstream pass-through)
BUILD_ARGS=""
test -z "${_RBGY_IMAGE_1}" || BUILD_ARGS="${BUILD_ARGS} --build-arg RBF_IMAGE_1=${_RBGY_IMAGE_1}"
test -z "${_RBGY_IMAGE_2}" || BUILD_ARGS="${BUILD_ARGS} --build-arg RBF_IMAGE_2=${_RBGY_IMAGE_2}"
test -z "${_RBGY_IMAGE_3}" || BUILD_ARGS="${BUILD_ARGS} --build-arg RBF_IMAGE_3=${_RBGY_IMAGE_3}"

test -s .hallmark || (echo "hallmark not derived" >&2; exit 1)
HALLMARK="$(cat .hallmark)"

GAR_REPO_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}"
IMAGE_URI="${GAR_REPO_BASE}/${_RBGY_HALLMARKS_ROOT}/${HALLMARK}/${_RBGY_ARK_BASENAME_IMAGE}:${HALLMARK}"

docker buildx version
docker version

# Capture Docker daemon state before build (forwarded to about via diags)
# Format: {"timestamp":"...","host_daemon_images":[...]} matching inspect's jq queries
CACHE_TS_BEFORE="$(date -u +%FT%TZ)"
docker images --no-trunc --format '{{json .}}' > cache_before_raw.txt
{
  printf '{"timestamp":"%s","host_daemon_images":[' "${CACHE_TS_BEFORE}"
  awk 'NR>1{printf ","}{printf "%s",$0}' cache_before_raw.txt
  printf ']}'
} > cache_before.json
rm -f cache_before_raw.txt
echo "cache_before.json written ($(wc -c < cache_before.json | tr -d ' ') bytes)"

# Create docker-container driver for multi-platform builds (QEMU emulation)
docker buildx create --driver docker-container --name rb-builder --use

# Build all platforms and push consumer-facing image manifest list
# --metadata-file captures BuildKit's resolved base image references, digests,
# and build parameters for base image provenance in inspect.
docker buildx build \
  --push \
  --platform="${_RBGY_PLATFORMS}" \
  --tag "${IMAGE_URI}" \
  --metadata-file buildkit_metadata.json \
  --label "hallmark=${HALLMARK}" \
  --label "git.commit=${_RBGY_GIT_COMMIT}" \
  --label "git.branch=${_RBGY_GIT_BRANCH}" \
  ${BUILD_ARGS} \
  -f "${_RBGY_DOCKERFILE}" \
  .

echo "Consumer manifest push complete: ${IMAGE_URI}"
