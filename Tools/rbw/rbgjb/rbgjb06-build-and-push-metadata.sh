#!/bin/bash
# RBGJB Step 06: Build and push metadata container
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_MONIKER, _RBGY_GAR_HOST_SUFFIX, _RBGY_ARK_SUFFIX_ABOUT,
#                _RBGY_PLATFORMS

set -euo pipefail

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"
META_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_ABOUT}"

# Platform label for SBOM filenames (linux/amd64 → linux_amd64)
PLATFORM_LABEL="$(echo "${_RBGY_PLATFORMS}" | tr '/' '_')"

{
  echo 'FROM scratch'
  echo 'LABEL org.opencontainers.image.title="rbia-metadata"'
  echo 'ADD build_info.json /build_info.json'
  echo "ADD sbom.${PLATFORM_LABEL}.spdx.json /sbom.${PLATFORM_LABEL}.spdx.json"
  echo "ADD package_summary.${PLATFORM_LABEL}.txt /package_summary.${PLATFORM_LABEL}.txt"
  echo 'ADD recipe.txt /recipe.txt'
} > Dockerfile.meta

docker build -f Dockerfile.meta -t "${META_URI}" .
docker push "${META_URI}"
