#!/bin/bash
# RBGJAM Step 03: Generate per-platform mode-aware build_info.json
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (Cloud Build overrides shebang; bash shebang for shellcheck)
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_VESSEL,
#                _RBGA_CONSECRATION, _RBGA_VESSEL_MODE,
#                _RBGA_GIT_COMMIT, _RBGA_GIT_BRANCH, _RBGA_GIT_REPO,
#                _RBGA_BUILD_ID, _RBGA_INSCRIBE_TIMESTAMP,
#                _RBGA_BIND_SOURCE, _RBGA_GRAFT_SOURCE,
#                _RBGA_DOCKERFILE_CONTENT
#
# Generates one build_info-{arch}{variant}.json per platform with mode-aware fields.
# Shared fields: vessel_mode, vessel_name, platform, image_digest, about_timestamp,
#                git_commit, git_branch, git_repo
# Conjure fields: build_id, inscribe_timestamp, qemu_used, slsa_*
# Bind fields: bind_source
# Graft fields: graft_source
#
# Note: jq is NOT available in gcr.io/cloud-builders/docker or alpine base,
# so this step uses alpine + apk add jq.

# Note: pipefail is not POSIX but is supported by busybox ash (Alpine's /bin/sh).
# This step targets Alpine Cloud Build images which always use busybox ash.
set -euo pipefail

apk add --no-cache jq >/dev/null

test -n "${_RBGA_GAR_HOST}"       || { echo "_RBGA_GAR_HOST missing"       >&2; exit 1; }
test -n "${_RBGA_GAR_PATH}"       || { echo "_RBGA_GAR_PATH missing"       >&2; exit 1; }
test -n "${_RBGA_VESSEL}"         || { echo "_RBGA_VESSEL missing"         >&2; exit 1; }
test -n "${_RBGA_CONSECRATION}"   || { echo "_RBGA_CONSECRATION missing"   >&2; exit 1; }
test -n "${_RBGA_VESSEL_MODE}"    || { echo "_RBGA_VESSEL_MODE missing"    >&2; exit 1; }

test -s platforms.txt         || { echo "platforms.txt not found (step 01)" >&2; exit 1; }
test -s platform_suffixes.txt || { echo "platform_suffixes.txt not found (step 01)" >&2; exit 1; }
test -s platform_digests.txt  || { echo "platform_digests.txt not found (step 01)" >&2; exit 1; }

IMAGE_BASE="${_RBGA_GAR_HOST}/${_RBGA_GAR_PATH}/${_RBGA_VESSEL}"

# Write recipe.txt — prefer -diags extraction (step 01), fall back to substitution variable
if [ -f recipe.txt ]; then
  echo "recipe.txt present from -diags extraction ($(wc -c < recipe.txt | tr -d ' ') bytes) — skipping substitution variable"
elif [ -n "${_RBGA_DOCKERFILE_CONTENT:-}" ]; then
  printf '%s' "${_RBGA_DOCKERFILE_CONTENT}" > recipe.txt
  echo "recipe.txt written from substitution variable ($(wc -c < recipe.txt | tr -d ' ') bytes)"
else
  echo "No Dockerfile content provided — recipe.txt omitted"
fi

TS="$(date -u +%FT%TZ)"

# Host platform for QEMU detection (conjure only)
HOST_PLATFORM="linux/amd64"

# Load per-platform digests into individual files (avoids eval with external data)
while IFS=' ' read -r D_SUFFIX D_DIGEST; do
  D_LABEL="${D_SUFFIX#-}"
  D_LABEL=$(printf '%s' "${D_LABEL}" | tr -cd 'a-z0-9')
  echo "${D_DIGEST}" > "digest-${D_LABEL}.txt"
done < platform_digests.txt

# Process each platform (POSIX sh — no arrays, use positional parsing)
PLATFORMS_CSV="$(cat platforms.txt)"
SUFFIXES_CSV="$(cat platform_suffixes.txt)"

echo "=== Generating per-platform build_info ==="

REMAINING_PLATS="${PLATFORMS_CSV}"
REMAINING_SUFFIXES="${SUFFIXES_CSV}"

while [ -n "${REMAINING_PLATS}" ]; do
  PLAT="${REMAINING_PLATS%%,*}"
  SUFFIX="${REMAINING_SUFFIXES%%,*}"

  if [ "${REMAINING_PLATS}" = "${PLAT}" ]; then
    REMAINING_PLATS=""
    REMAINING_SUFFIXES=""
  else
    REMAINING_PLATS="${REMAINING_PLATS#*,}"
    REMAINING_SUFFIXES="${REMAINING_SUFFIXES#*,}"
  fi

  LABEL="${SUFFIX#-}"
  LABEL=$(printf '%s' "${LABEL}" | tr -cd 'a-z0-9')
  INFO_FILE="build_info-${LABEL}.json"

  # Look up per-platform digest from file written above
  IMAGE_DIGEST=""
  if [ -f "digest-${LABEL}.txt" ]; then
    IMAGE_DIGEST=$(cat "digest-${LABEL}.txt")
  fi

  echo "--- ${PLAT} → ${INFO_FILE} ---"

  case "${_RBGA_VESSEL_MODE}" in
    conjure)
      # Determine QEMU usage
      QEMU_USED="true"
      if [ "${PLAT}" = "${HOST_PLATFORM}" ]; then
        QEMU_USED="false"
      fi

      jq -n \
        --arg consecration  "${_RBGA_CONSECRATION}" \
        --arg vessel_mode   "${_RBGA_VESSEL_MODE}" \
        --arg vessel_name   "${_RBGA_VESSEL}" \
        --arg platform      "${PLAT}" \
        --arg image_digest  "${IMAGE_DIGEST}" \
        --arg about_ts      "${TS}" \
        --arg git_commit    "${_RBGA_GIT_COMMIT}" \
        --arg git_branch    "${_RBGA_GIT_BRANCH}" \
        --arg git_repo      "${_RBGA_GIT_REPO}" \
        --arg build_id      "${_RBGA_BUILD_ID}" \
        --arg inscribe_ts   "${_RBGA_INSCRIBE_TIMESTAMP}" \
        --argjson qemu_used "${QEMU_USED}" \
        '{
          consecration: $consecration,
          vessel_mode: $vessel_mode,
          vessel_name: $vessel_name,
          platform: $platform,
          image_digest: $image_digest,
          about_timestamp: $about_ts,
          git: { repo: $git_repo, branch: $git_branch, commit: $git_commit },
          build: {
            build_id: $build_id,
            inscribe_timestamp: $inscribe_ts,
            qemu_used: $qemu_used
          },
          slsa: {
            build_level: 3,
            build_invocation_id: $build_id,
            provenance_predicate_types: ["https://slsa.dev/provenance/v0.1", "https://slsa.dev/provenance/v1"],
            provenance_builder_id: "https://cloudbuild.googleapis.com/GoogleHostedWorker"
          }
        }' > "${INFO_FILE}"
      ;;

    bind)
      jq -n \
        --arg consecration  "${_RBGA_CONSECRATION}" \
        --arg vessel_mode   "${_RBGA_VESSEL_MODE}" \
        --arg vessel_name   "${_RBGA_VESSEL}" \
        --arg platform      "${PLAT}" \
        --arg image_digest  "${IMAGE_DIGEST}" \
        --arg about_ts      "${TS}" \
        --arg git_commit    "${_RBGA_GIT_COMMIT}" \
        --arg git_branch    "${_RBGA_GIT_BRANCH}" \
        --arg git_repo      "${_RBGA_GIT_REPO}" \
        --arg bind_source   "${_RBGA_BIND_SOURCE}" \
        '{
          consecration: $consecration,
          vessel_mode: $vessel_mode,
          vessel_name: $vessel_name,
          platform: $platform,
          image_digest: $image_digest,
          about_timestamp: $about_ts,
          git: { repo: $git_repo, branch: $git_branch, commit: $git_commit },
          bind: { source: $bind_source }
        }' > "${INFO_FILE}"
      ;;

    graft)
      jq -n \
        --arg consecration  "${_RBGA_CONSECRATION}" \
        --arg vessel_mode   "${_RBGA_VESSEL_MODE}" \
        --arg vessel_name   "${_RBGA_VESSEL}" \
        --arg platform      "${PLAT}" \
        --arg image_digest  "${IMAGE_DIGEST}" \
        --arg about_ts      "${TS}" \
        --arg git_commit    "${_RBGA_GIT_COMMIT}" \
        --arg git_branch    "${_RBGA_GIT_BRANCH}" \
        --arg git_repo      "${_RBGA_GIT_REPO}" \
        --arg graft_source  "${_RBGA_GRAFT_SOURCE}" \
        '{
          consecration: $consecration,
          vessel_mode: $vessel_mode,
          vessel_name: $vessel_name,
          platform: $platform,
          image_digest: $image_digest,
          about_timestamp: $about_ts,
          git: { repo: $git_repo, branch: $git_branch, commit: $git_commit },
          graft: { source: $graft_source }
        }' > "${INFO_FILE}"
      ;;

    *)
      echo "Unknown vessel mode: ${_RBGA_VESSEL_MODE}" >&2
      exit 1
      ;;
  esac

  test -s "${INFO_FILE}" || { echo "build_info output empty for ${PLAT}" >&2; exit 1; }
  echo "Generated: ${INFO_FILE}"
done

echo "=== build_info generation complete ==="
