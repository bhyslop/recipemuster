#!/bin/bash
# RBGJB Step 07: Assemble build metadata JSON
# Builder: ${_RBGY_JQ_REF} (jq image)
# Entrypoint: sh (not bash)
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_GIT_REPO, _RBGY_GIT_BRANCH, _RBGY_GIT_COMMIT

set -euo pipefail

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"
IMG_URI="$(cat .image_uri)"
cp "${_RBGY_DOCKERFILE}" recipe.txt
TS="$(date -u +%FT%TZ)"

jq -n \
  --arg tag_base  "${TAG_BASE}" \
  --arg image_uri "${IMG_URI}" \
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
