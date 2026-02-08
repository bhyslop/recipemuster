#!/bin/bash
# RBGJB Step 06: Build multi-arch OCI layout
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GIT_COMMIT, _RBGY_GIT_BRANCH, _RBGY_GIT_REPO
#
# OCI Layout Bridge Phase 1: Export multi-platform build to /workspace/oci-layout
# instead of pushing directly to registry. This avoids the BuildKit credential
# isolation problem. Phase 2 (rbgjb07) will push the OCI layout using Skopeo.

set -euo pipefail

# Required inputs
test -n "${_RBGY_DOCKERFILE}"     || (echo "_RBGY_DOCKERFILE missing"     >&2; exit 1)
test -n "${_RBGY_MONIKER}"        || (echo "_RBGY_MONIKER missing"        >&2; exit 1)
test -n "${_RBGY_PLATFORMS}"      || (echo "_RBGY_PLATFORMS missing"      >&2; exit 1)
test -n "${_RBGY_GAR_LOCATION}"   || (echo "_RBGY_GAR_LOCATION missing"   >&2; exit 1)
test -n "${_RBGY_GAR_PROJECT}"    || (echo "_RBGY_GAR_PROJECT missing"    >&2; exit 1)
test -n "${_RBGY_GAR_REPOSITORY}" || (echo "_RBGY_GAR_REPOSITORY missing" >&2; exit 1)
test -s .tag_base                  || (echo "tag base not derived"         >&2; exit 1)
test -n "${_RBGY_GIT_COMMIT}"     || (echo "_RBGY_GIT_COMMIT missing"     >&2; exit 1)
test -n "${_RBGY_GIT_BRANCH}"     || (echo "_RBGY_GIT_BRANCH missing"     >&2; exit 1)
test -n "${_RBGY_GIT_REPO}"       || (echo "_RBGY_GIT_REPO missing"       >&2; exit 1)

TAG_BASE="$(cat .tag_base)"
IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

docker buildx version
docker version

# Create docker-container driver for multi-platform + OCI export support
# The default docker driver supports neither OCI exporter nor multi-platform builds
# See RBWMBX memo "OCI Output Path Research" section for details
docker buildx create --driver docker-container --name rb-builder --use

# Build multi-platform image and export to OCI archive tarball
# - Output goes to CLIENT filesystem (Cloud Build step container), not BuildKit container
# - BuildKit transfers results back via gRPC
# - Using tar=true (default) because tar=false has known annotation bugs (moby/buildkit#5572)
# - Skopeo can read tarball via oci-archive: transport
# - /workspace persists across Cloud Build steps for Phase 2 (Skopeo push)

docker buildx build \
  --platform="${_RBGY_PLATFORMS}" \
  --tag "${IMAGE_URI}" \
  --output type=oci,dest=/workspace/oci-layout.tar \
  --label "moniker=${_RBGY_MONIKER}" \
  --label "git.commit=${_RBGY_GIT_COMMIT}" \
  --label "git.branch=${_RBGY_GIT_BRANCH}" \
  -f "${_RBGY_DOCKERFILE}" \
  .
