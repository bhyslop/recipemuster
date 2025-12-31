#!/bin/bash
# RBGJB Step 06: Build multi-arch image and push to GAR
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_GIT_COMMIT, _RBGY_GIT_BRANCH, _RBGY_GIT_REPO
#
# Note: Buildx setup is done inline because Cloud Build steps run in isolated
# containers - a builder created in a previous step won't persist here.

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
IMAGE_URI="${_RBGY_GAR_LOCATION}-docker.pkg.dev/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}-img"

docker buildx version
docker version

# Use default buildx builder (docker driver = host docker daemon with GAR credentials from step 3)
# No need to create a builder - the default builder uses the authenticated docker daemon

docker buildx build \
  --platform="${_RBGY_PLATFORMS}" \
  --tag "${IMAGE_URI}" \
  --push \
  --label "moniker=${_RBGY_MONIKER}" \
  --label "git.commit=${_RBGY_GIT_COMMIT}" \
  --label "git.branch=${_RBGY_GIT_BRANCH}" \
  -f "${_RBGY_DOCKERFILE}" \
  .

echo "${IMAGE_URI}" > .image_uri
