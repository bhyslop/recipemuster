#!/bin/bash
# RBGJB Step 05: Assemble build metadata JSON
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash)
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GIT_REPO, _RBGY_GIT_BRANCH, _RBGY_GIT_COMMIT,
#                _RBGY_GAR_LOCATION, _RBGY_GAR_HOST_SUFFIX,
#                _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_ARK_SUFFIX_IMAGE

set -euo pipefail

apk add --no-cache jq >/dev/null

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

# Image URI uses TAG_BASE (full consecration)
IMAGE_URI="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}:${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}"

cp "${_RBGY_DOCKERFILE}" recipe.txt
TS="$(date -u +%FT%TZ)"

jq -n \
  --arg tag_base  "${TAG_BASE}" \
  --arg image_uri "${IMAGE_URI}" \
  --arg moniker   "${_RBGY_MONIKER}" \
  --arg repo      "${_RBGY_GIT_REPO}" \
  --arg branch    "${_RBGY_GIT_BRANCH}" \
  --arg commit    "${_RBGY_GIT_COMMIT}" \
  --arg ts        "${TS}" \
  --arg platforms "${_RBGY_PLATFORMS}" \
  '{
    tag_base: $tag_base,
    image: { uri: $image_uri },
    moniker: $moniker,
    git: { repo: $repo, branch: $branch, commit: $commit },
    build: { timestamp: $ts },
    platforms: $platforms
  }' > build_info.json
