#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Container Registry - Registry interface

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBCR_INCLUDED:-}" || bcu_die "Module rbcr multiply included - check sourcing hierarchy"
ZRBCR_INCLUDED=1

######################################################################
# Internal Functions (zrbcr_*)

zrbcr_kindle() {
  # Check required environment
  test -n "${RBRR_REGISTRY:-}"       || bcu_die "RBRR_REGISTRY not set"
  test -n "${RBRR_REGISTRY_OWNER:-}" || bcu_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}"  || bcu_die "RBRR_REGISTRY_NAME not set"
  test -n "${RBG_RUNTIME:-}"         || bcu_die "RBG_RUNTIME not set"
  test -n "${RBG_TEMP_DIR:-}"        || bcu_die "RBG_TEMP_DIR not set"

  # Detect environment and set auth variables
  if test -n "${GITHUB_ACTIONS:-}"; then
    # GitHub Actions mode - use environment variables directly
    bcu_info "Running in GitHub Actions - using GITHUB_TOKEN"
    test -n "${GITHUB_TOKEN:-}" || bcu_die "GITHUB_TOKEN not set in GitHub Actions"
    ZRBCR_GITHUB_TOKEN="${GITHUB_TOKEN}"
    ZRBCR_REGISTRY_USERNAME="${GITHUB_ACTOR:-github-actions}"
  else
    # Local mode - source PAT file
    bcu_info "Running locally - sourcing PAT file"
    test -n "${RBRR_GITHUB_PAT_ENV:-}" || bcu_die "RBRR_GITHUB_PAT_ENV not set"
    test -f "${RBRR_GITHUB_PAT_ENV}" || bcu_die "PAT file not found: ${RBRR_GITHUB_PAT_ENV}"
    source "${RBRR_GITHUB_PAT_ENV}"
    test -n "${RBRG_PAT:-}" || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
    test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"
    ZRBCR_GITHUB_TOKEN="${RBRG_PAT}"
    ZRBCR_REGISTRY_USERNAME="${RBRG_USERNAME}"
  fi

  # Module Variables (ZRBCR_*)
  ZRBCR_REGISTRY_HOST="ghcr.io"
  ZRBCR_REGISTRY_API_BASE="https://ghcr.io/v2/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBCR_TOKEN_URL="https://ghcr.io/token?scope=repository:${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:pull&service=ghcr.io"

  # Media types
  ZRBCR_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBCR_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBCR_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBCR_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBCR_ACCEPT_MANIFEST_MTYPES="${ZRBCR_MTYPE_DV2},${ZRBCR_MTYPE_DLIST},${ZRBCR_MTYPE_OCI},${ZRBCR_MTYPE_OCM}"
  ZRBCR_SCHEMA_V2="2"
  ZRBCR_MTYPE_GHV3="application/vnd.github.v3+json"

  # Curl headers
  ZRBCR_HEADER_AUTH_TOKEN="Authorization: token ${ZRBCR_GITHUB_TOKEN}"
  ZRBCR_HEADER_ACCEPT_GH="Accept: ${ZRBCR_MTYPE_GHV3}"
  ZRBCR_HEADER_ACCEPT_MANIFEST="Accept: ${ZRBCR_ACCEPT_MANIFEST_MTYPES}"

  # File prefixes for all operations
  ZRBCR_LIST_PAGE_PREFIX="${RBG_TEMP_DIR}/list_page_"
  ZRBCR_LIST_RECORDS_PREFIX="${RBG_TEMP_DIR}/list_records_"
  ZRBCR_MANIFEST_PREFIX="${RBG_TEMP_DIR}/manifest_"
  ZRBCR_CONFIG_PREFIX="${RBG_TEMP_DIR}/config_"
  ZRBCR_DELETE_PREFIX="${RBG_TEMP_DIR}/delete_"
  ZRBCR_VERSION_PREFIX="${RBG_TEMP_DIR}/version_"
  ZRBCR_DETAIL_PREFIX="${RBG_TEMP_DIR}/detail_"

  # Output files
  ZRBCR_IMAGE_RECORDS_FILE="${RBG_TEMP_DIR}/IMAGE_RECORDS.json"
  ZRBCR_IMAGE_DETAIL_FILE="${RBG_TEMP_DIR}/IMAGE_DETAILS.json"
  ZRBCR_IMAGE_STATS_FILE="${RBG_TEMP_DIR}/IMAGE_STATS.json"
  ZRBCR_FQIN_FILE="${RBG_TEMP_DIR}/FQIN.txt"

  # File index counter
  ZRBCR_FILE_INDEX=0

  bcu_step "Obtaining bearer token for registry API"
  local z_bearer_token
  z_bearer_token=$(zrbcr_get_bearer_token_subshell) || bcu_die "Cannot proceed without bearer token"
  ZRBCR_REGISTRY_TOKEN="${z_bearer_token}"

  # Registry auth header
  ZRBCR_HEADER_AUTH_BEARER="Authorization: Bearer ${ZRBCR_REGISTRY_TOKEN}"

  # Login to registry
  bcu_step "Log in to container registry"
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} login "${ZRBCR_REGISTRY_HOST}" -u "${ZRBCR_REGISTRY_USERNAME}" -p "${ZRBCR_GITHUB_TOKEN}"

  ZRBCR_KINDLED=1
}

zrbcr_sentinel() {
  test "${ZRBCR_KINDLED:-}" = "1" || bcu_die "Module rbcr not kindled - call zrbcr_kindle first"
}

zrbcr_get_bearer_token_subshell() {
  # Fetch token and extract in memory only
  local z_response
  z_response=$(curl -sL -u "${ZRBCR_REGISTRY_USERNAME}:${ZRBCR_GITHUB_TOKEN}" \
    "${ZRBCR_TOKEN_URL}" 2>/dev/null) || return 1

  local z_token
  z_token=$(echo "${z_response}" | jq -r '.token' 2>/dev/null) || return 1

  test -n "${z_token}"           || return 1
  test    "${z_token}" != "null" || return 1
  echo    "${z_token}"
}

zrbcr_curl_github_api() {
  local z_url="$1"

  curl -s                              \
       -H "${ZRBCR_HEADER_AUTH_TOKEN}" \
       -H "${ZRBCR_HEADER_ACCEPT_GH}"  \
       "${z_url}"
}

zrbcr_get_next_index() {
  ZRBCR_FILE_INDEX=$((ZRBCR_FILE_INDEX + 1))
  printf "%03d" "${ZRBCR_FILE_INDEX}"
}

zrbcr_process_single_manifest() {
  local z_tag="$1"
  local z_manifest_file="$2"
  local z_platform="$3"  # Empty for single-platform

  # Get config digest
  local z_config_digest
  z_config_digest=$(jq -r '.config.digest' "${z_manifest_file}")

  test -n "${z_config_digest}" || bcu_die "Missing config.digest"
  test "${z_config_digest}" != "null" || {
    bcu_warn "null config.digest in manifest"
    return 0
  }

  # Fetch config blob
  local z_idx
  z_idx=$(zrbcr_get_next_index)
  local z_config_out="${ZRBCR_CONFIG_PREFIX}${z_idx}.json"
  local z_config_err="${ZRBCR_CONFIG_PREFIX}${z_idx}.err"

  curl -sL -H "${ZRBCR_HEADER_AUTH_BEARER}" "${ZRBCR_REGISTRY_API_BASE}/blobs/${z_config_digest}" \
        >"${z_config_out}" 2>"${z_config_err}" && \
    jq . "${z_config_out}" >/dev/null || {
      bcu_warn "Failed to fetch config blob"
      bcu_die "Failed to retrieve config blob from registry"
    }

  # Build detail entry
  local z_temp_detail="${ZRBCR_DETAIL_PREFIX}$(zrbcr_get_next_index).json"
  local z_manifest_json z_config_json
  z_manifest_json="$(<"${z_manifest_file}")"
  z_config_json=$(jq '. + {
    created: (.created // "1970-01-01T00:00:00Z"),
    architecture: (.architecture // "unknown"),
    os: (.os // "unknown")
  }' "${z_config_out}")

  if test -n "${z_platform}"; then
    jq -n \
      --arg tag          "${z_tag}"           \
      --arg platform     "${z_platform}"      \
      --arg digest       "${z_config_digest}" \
      --argjson manifest "${z_manifest_json}" \
      --argjson config   "${z_config_json}" '
      {
        tag: $tag,
        platform: $platform,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${z_temp_detail}"
  else
    jq -n \
      --arg     tag      "${z_tag}"           \
      --arg     digest   "${z_config_digest}" \
      --argjson manifest "${z_manifest_json}" \
      --argjson config   "${z_config_json}" '
      {
        tag: $tag,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${z_temp_detail}"
  fi

  # Append to detail file
  jq -s '.[0] + [.[1]]' "${ZRBCR_IMAGE_DETAIL_FILE}" "${z_temp_detail}" \
    > "${ZRBCR_IMAGE_DETAIL_FILE}.tmp" || bcu_die "Failed to merge image detail"
  mv "${ZRBCR_IMAGE_DETAIL_FILE}.tmp" "${ZRBCR_IMAGE_DETAIL_FILE}"
}

######################################################################
# External Functions (rbcr_*)

rbcr_make_fqin() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Write FQIN to file
  echo "${ZRBCR_REGISTRY_HOST}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:${z_tag}" > "${ZRBCR_FQIN_FILE}"
}

rbcr_list_tags() {
  # Ensure module started
  zrbcr_sentinel

  bcu_step "Fetching all image records with pagination"

  # Initialize empty array
  echo "[]" > "${ZRBCR_IMAGE_RECORDS_FILE}"

  local z_page=1

  while true; do
    bcu_info "Fetching page ${z_page}..."

    local z_temp_page="${ZRBCR_LIST_PAGE_PREFIX}${z_page}.json"
    local z_temp_records="${ZRBCR_LIST_RECORDS_PREFIX}${z_page}.json"

    local z_url="https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100&page=${z_page}"
    zrbcr_curl_github_api "${z_url}" > "${z_temp_page}"

    local z_items
    z_items=$(jq '. | length' "${z_temp_page}")
    bcu_info "Saw ${z_items} items on page ${z_page}"

    test "${z_items}" -ne 0 || break

    # Transform to simplified records
    jq -r --arg prefix "${ZRBCR_REGISTRY_HOST}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}" \
      '[.[] | select(.metadata.container.tags | length > 0) |
       .id as $id | .metadata.container.tags[] as $tag |
       {version_id: $id, tag: $tag, fqin: ($prefix + ":" + $tag)}]' \
      "${z_temp_page}" > "${z_temp_records}"

    # Merge with existing
    jq -s '.[0] + .[1]' "${ZRBCR_IMAGE_RECORDS_FILE}" "${z_temp_records}" > \
       "${ZRBCR_IMAGE_RECORDS_FILE}.tmp"
    mv "${ZRBCR_IMAGE_RECORDS_FILE}.tmp" "${ZRBCR_IMAGE_RECORDS_FILE}"

    z_page=$((z_page + 1))
  done

  local z_total
  z_total=$(jq '. | length' "${ZRBCR_IMAGE_RECORDS_FILE}")
  bcu_info "Retrieved ${z_total} total image records"
}

rbcr_get_manifest() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  local z_idx
  z_idx=$(zrbcr_get_next_index)
  local z_manifest_out="${ZRBCR_MANIFEST_PREFIX}${z_idx}.json"
  local z_manifest_err="${ZRBCR_MANIFEST_PREFIX}${z_idx}.err"

  curl -sL \
       -H "${ZRBCR_HEADER_AUTH_BEARER}" \
       -H "${ZRBCR_HEADER_ACCEPT_MANIFEST}" \
       "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_tag}" \
       >"${z_manifest_out}" 2>"${z_manifest_err}" \
    && jq . "${z_manifest_out}" >/dev/null \
    || {
      bcu_warn "Failed to fetch manifest for ${z_tag}"
      bcu_die "This image appears corrupted"
    }

  local z_media_type
  z_media_type=$(jq -r '.mediaType // .schemaVersion' "${z_manifest_out}")

  if [[ "${z_media_type}" == "${ZRBCR_MTYPE_DLIST}" ]] || \
     [[ "${z_media_type}" == "${ZRBCR_MTYPE_OCI}" ]]; then

    bcu_info "Multi-platform image detected"

    local z_platform_idx=1
    local z_manifests
    z_manifests=$(jq -c '.manifests[]' "${z_manifest_out}")

    while IFS= read -r z_platform_manifest; do
      local z_platform_digest z_platform_info
      z_platform_digest=$(echo "${z_platform_manifest}" | jq -r '.digest')
      z_platform_info=$(echo "${z_platform_manifest}" | jq -r '"\(.platform.os)/\(.platform.architecture)"')

      bcu_info "Processing platform: ${z_platform_info}"

      local z_platform_idx_str
      z_platform_idx_str=$(zrbcr_get_next_index)
      local z_platform_out="${ZRBCR_MANIFEST_PREFIX}${z_platform_idx_str}.json"
      local z_platform_err="${ZRBCR_MANIFEST_PREFIX}${z_platform_idx_str}.err"

      curl -sL \
           -H "${ZRBCR_HEADER_AUTH_BEARER}" \
           -H "${ZRBCR_HEADER_ACCEPT_MANIFEST}" \
           "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_platform_digest}" \
           >"${z_platform_out}" 2>"${z_platform_err}" \
        && jq . "${z_platform_out}" >/dev/null \
        || {
          bcu_warn "Failed to fetch platform manifest"
          ((z_platform_idx++))
          continue
        }

      zrbcr_process_single_manifest "${z_tag}" "${z_platform_out}" "${z_platform_info}"

      ((z_platform_idx++))
    done <<< "${z_manifests}"

  else
    bcu_info "Single platform image"
    zrbcr_process_single_manifest "${z_tag}" "${z_manifest_out}" ""
  fi
}

rbcr_get_config() {
  # Name parameters
  local z_digest="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_digest}" || bcu_die "Digest parameter required"

  # Fetch config blob
  local z_idx
  z_idx=$(zrbcr_get_next_index)
  local z_config_out="${ZRBCR_CONFIG_PREFIX}${z_idx}.json"

  curl -sL -H "${ZRBCR_HEADER_AUTH_BEARER}" \
       "${ZRBCR_REGISTRY_API_BASE}/blobs/${z_digest}" > "${z_config_out}"
}

rbcr_delete() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Get version ID for tag
  rbcr_get_version_id "${z_tag}"
  local z_version_id
  z_version_id=$(<"${ZRBCR_VERSION_ID_FILE}")

  # Delete via GitHub API
  local z_delete_url="https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions/${z_version_id}"
  local z_result="${ZRBCR_DELETE_PREFIX}result.txt"
  local z_status_file="${ZRBCR_DELETE_PREFIX}status.txt"

  # Use -w to write status to separate file to avoid subshell
  curl -X DELETE -s \
    -H "${ZRBCR_HEADER_AUTH_TOKEN}" \
    -H "${ZRBCR_HEADER_ACCEPT_GH}" \
    -w "%{http_code}" \
    -o "${z_result}" \
    "${z_delete_url}" > "${z_status_file}"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test "${z_http_code}" = "204" || bcu_die "Delete failed with HTTP ${z_http_code}"
}

rbcr_pull() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Construct FQIN from tag
  rbcr_make_fqin "${z_tag}"
  local z_fqin
  z_fqin=$(<"${ZRBCR_FQIN_FILE}")

  bcu_info "Fetch image..."
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} pull "${z_fqin}"
}

rbcr_exists_predicate() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Check if tag exists
  local z_url="https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100"
  zrbcr_curl_github_api "${z_url}" | \
    jq -e '.[] | select(.metadata.container.tags[] | contains("'"${z_tag}"'"))' > /dev/null
}

rbcr_get_version_id() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Find version ID
  ZRBCR_VERSION_ID_FILE="${ZRBCR_VERSION_PREFIX}id.txt"

  rbcr_list_tags

  jq -r '.[] | select(.tag == "'"${z_tag}"'") | .version_id' \
    "${ZRBCR_IMAGE_RECORDS_FILE}" > "${ZRBCR_VERSION_ID_FILE}"

  test -s "${ZRBCR_VERSION_ID_FILE}" || bcu_die "Version ID not found for tag: ${z_tag}"
}

# eof


