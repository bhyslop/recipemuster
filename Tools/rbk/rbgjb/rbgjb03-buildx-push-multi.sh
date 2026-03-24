#!/bin/bash
# RBGJBM Step 03: Build all platforms and push intermediate -multi tag to GAR
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GAR_HOST_SUFFIX, _RBGY_INSCRIBE_TIMESTAMP,
#                _RBGY_GIT_COMMIT, _RBGY_GIT_BRANCH,
#                _RBGY_IMAGE_1, _RBGY_IMAGE_2, _RBGY_IMAGE_3 (optional base image refs)
#
# Uses buildx --push with docker-container driver. Cloud Build pre-populates
# Docker credentials in the host daemon; buildx inherits them via the
# docker-container driver's config propagation.
#
# The -multi tag is an intermediate artifact: per-platform pullback (step 04)
# extracts individual platform images from it.

set -euo pipefail

# Required inputs
test -n "${_RBGY_DOCKERFILE}"          || (echo "_RBGY_DOCKERFILE missing"          >&2; exit 1)
test -n "${_RBGY_MONIKER}"             || (echo "_RBGY_MONIKER missing"             >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"           || (echo "_RBGY_PLATFORMS missing"           >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"        || (echo "_RBGY_GAR_LOCATION missing"        >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"         || (echo "_RBGY_GAR_PROJECT missing"         >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}"      || (echo "_RBGY_GAR_REPOSITORY missing"      >&2; exit 1)
test -n "${_RBGY_INSCRIBE_TIMESTAMP}"  || (echo "_RBGY_INSCRIBE_TIMESTAMP missing"  >&2; exit 1)
test -n "${_RBGY_GIT_COMMIT}"          || (echo "_RBGY_GIT_COMMIT missing"          >&2; exit 1)
test -n "${_RBGY_GIT_BRANCH}"          || (echo "_RBGY_GIT_BRANCH missing"          >&2; exit 1)

# Resolve base image build-args (anchored GAR refs or upstream pass-through)
BUILD_ARGS=""
test -z "${_RBGY_IMAGE_1}" || BUILD_ARGS="${BUILD_ARGS} --build-arg RBF_IMAGE_1=${_RBGY_IMAGE_1}"
test -z "${_RBGY_IMAGE_2}" || BUILD_ARGS="${BUILD_ARGS} --build-arg RBF_IMAGE_2=${_RBGY_IMAGE_2}"
test -z "${_RBGY_IMAGE_3}" || BUILD_ARGS="${BUILD_ARGS} --build-arg RBF_IMAGE_3=${_RBGY_IMAGE_3}"

MULTI_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${_RBGY_INSCRIBE_TIMESTAMP}-multi"

docker buildx version
docker version

# Capture Docker daemon state before build (forwarded to about via -diags)
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

# Build all platforms and push intermediate -multi tag
# --metadata-file captures BuildKit's resolved base image references, digests,
# and build parameters for base image provenance in inspect.
docker buildx build \
  --push \
  --platform="${_RBGY_PLATFORMS}" \
  --tag "${MULTI_URI}" \
  --metadata-file buildkit_metadata.json \
  --label "moniker=${_RBGY_MONIKER}" \
  --label "git.commit=${_RBGY_GIT_COMMIT}" \
  --label "git.branch=${_RBGY_GIT_BRANCH}" \
  ${BUILD_ARGS} \
  -f "${_RBGY_DOCKERFILE}" \
  .

echo "Multi-platform push complete: ${MULTI_URI}"
