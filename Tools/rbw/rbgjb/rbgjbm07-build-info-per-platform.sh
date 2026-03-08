#!/bin/sh
# RBGJBM Step 07: Generate per-platform build_info.json with SLSA summary
# Builder: alpine (via RBRR_GCB_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGY_DOCKERFILE, _RBGY_MONIKER, _RBGY_PLATFORMS,
#                _RBGY_PLATFORM_SUFFIXES, _RBGY_GIT_REPO, _RBGY_GIT_BRANCH,
#                _RBGY_GIT_COMMIT, _RBGY_GAR_LOCATION, _RBGY_GAR_HOST_SUFFIX,
#                _RBGY_GAR_PROJECT, _RBGY_GAR_REPOSITORY,
#                _RBGY_ARK_SUFFIX_IMAGE, _RBGY_INSCRIBE_TIMESTAMP
#
# Generates one build_info-{arch}{variant}.json per platform.
# Per-platform fields: platform, image_digest, qemu_used (boolean)
# Shared fields: build_id, build_timestamp, inscribe_timestamp, git_commit, vessel_name
# SLSA summary fields: slsa_build_level, build_invocation_id,
#                       provenance_predicate_types, provenance_builder_id
#
# Note: jq is NOT available in gcr.io/cloud-builders/docker, so this step
# uses alpine + apk add jq.

set -euo pipefail

apk add --no-cache jq >/dev/null

test -s .tag_base || (echo "tag base not derived" >&2; exit 1)
TAG_BASE="$(cat .tag_base)"

IMAGE_BASE="${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}/${_RBGY_GAR_PROJECT}/${_RBGY_GAR_REPOSITORY}/${_RBGY_MONIKER}"

# Copy Dockerfile as recipe artifact
cp "${_RBGY_DOCKERFILE}" recipe.txt

TS="$(date -u +%FT%TZ)"

# Cloud Build provides BUILD_ID as an environment variable in all steps
BUILD_ID="${BUILD_ID:-unknown}"

# Determine host platform for QEMU detection
HOST_PLATFORM="linux/amd64"

# Split platforms and suffixes (POSIX sh — no arrays, use positional parsing)
PLATFORMS_CSV="${_RBGY_PLATFORMS}"
SUFFIXES_CSV="${_RBGY_PLATFORM_SUFFIXES}"

echo "=== Generating per-platform build_info ==="

# Process each platform by consuming CSV fields
REMAINING_PLATS="${PLATFORMS_CSV}"
REMAINING_SUFFIXES="${SUFFIXES_CSV}"

while [ -n "${REMAINING_PLATS}" ]; do
  # Extract first platform and suffix
  PLAT="${REMAINING_PLATS%%,*}"
  SUFFIX="${REMAINING_SUFFIXES%%,*}"

  # Advance to next
  if [ "${REMAINING_PLATS}" = "${PLAT}" ]; then
    REMAINING_PLATS=""
    REMAINING_SUFFIXES=""
  else
    REMAINING_PLATS="${REMAINING_PLATS#*,}"
    REMAINING_SUFFIXES="${REMAINING_SUFFIXES#*,}"
  fi

  # Derive filenames: strip leading dash from suffix
  LABEL="${SUFFIX#-}"
  INFO_FILE="build_info-${LABEL}.json"
  PER_PLAT_TAG="${TAG_BASE}${_RBGY_ARK_SUFFIX_IMAGE}${SUFFIX}"
  IMAGE_URI="${IMAGE_BASE}:${PER_PLAT_TAG}"

  # Determine if QEMU is used for this platform
  QEMU_USED="true"
  if [ "${PLAT}" = "${HOST_PLATFORM}" ]; then
    QEMU_USED="false"
  fi

  echo "--- ${PLAT} → ${INFO_FILE} ---"

  jq -n \
    --arg tag_base       "${TAG_BASE}" \
    --arg image_uri      "${IMAGE_URI}" \
    --arg moniker        "${_RBGY_MONIKER}" \
    --arg repo           "${_RBGY_GIT_REPO}" \
    --arg branch         "${_RBGY_GIT_BRANCH}" \
    --arg commit         "${_RBGY_GIT_COMMIT}" \
    --arg ts             "${TS}" \
    --arg platform       "${PLAT}" \
    --arg inscribe_ts    "${_RBGY_INSCRIBE_TIMESTAMP}" \
    --arg build_id       "${BUILD_ID}" \
    --argjson qemu_used  "${QEMU_USED}" \
    '{
      tag_base: $tag_base,
      image: { uri: $image_uri },
      moniker: $moniker,
      platform: $platform,
      qemu_used: $qemu_used,
      git: { repo: $repo, branch: $branch, commit: $commit },
      build: {
        timestamp: $ts,
        build_id: $build_id,
        inscribe_timestamp: $inscribe_ts
      },
      slsa: {
        build_level: 3,
        build_invocation_id: $build_id,
        provenance_predicate_types: ["https://slsa.dev/provenance/v0.1", "https://slsa.dev/provenance/v1"],
        provenance_builder_id: "https://cloudbuild.googleapis.com/GoogleHostedWorker"
      }
    }' > "${INFO_FILE}"

  test -s "${INFO_FILE}" || (echo "build_info output empty for ${PLAT}" >&2; exit 1)
  echo "Generated: ${INFO_FILE}"
done

echo "=== build_info generation complete ==="
