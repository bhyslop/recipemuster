#!/bin/bash
# Copyright 2025 Scale Invariant, Inc.
# Licensed under the Apache License, Version 2.0
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Recipe Bottle Container Registry - Registry interface

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHCR_INCLUDED:-}" || buc_die "Module rbcr multiply included - check sourcing hierarchy"
ZRBHCR_INCLUDED=1

######################################################################
# Internal Functions (zrbhcr_*)

zrbhcr_kindle() {
  # Check required environment
  test -n "${RBRR_REGISTRY_OWNER:-}" || buc_die "RBRR_REGISTRY_OWNER not set"
  test -n "${RBRR_REGISTRY_NAME:-}"  || buc_die "RBRR_REGISTRY_NAME not set"
  test -n "${RBG_RUNTIME:-}"         || buc_die "RBG_RUNTIME not set"
  test -n "${BURD_TEMP_DIR:-}"        || buc_die "BURD_TEMP_DIR not set"

  # Detect environment and set auth variables
  if test -n "${GITHUB_ACTIONS:-}"; then
    # GitHub Actions mode - use environment variables directly
    buc_info "Running in GitHub Actions - using GITHUB_TOKEN"
    test -n "${GITHUB_TOKEN:-}" || buc_die "GITHUB_TOKEN not set in GitHub Actions"
    ZRBHCR_GITHUB_TOKEN="${GITHUB_TOKEN}"
    ZRBHCR_REGISTRY_USERNAME="${GITHUB_ACTOR:-github-actions}"
  else
    # Local mode - source PAT file
    buc_info "Running locally - sourcing PAT file"
    test -n "${RBRR_GITHUB_PAT_ENV:-}" || buc_die "RBRR_GITHUB_PAT_ENV not set"
    test -f "${RBRR_GITHUB_PAT_ENV}" || buc_die "PAT file not found: ${RBRR_GITHUB_PAT_ENV}"
    source "${RBRR_GITHUB_PAT_ENV}"
    test -n "${RBRG_PAT:-}" || buc_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
    test -n "${RBRG_USERNAME:-}" || buc_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"
    ZRBHCR_GITHUB_TOKEN="${RBRG_PAT}"
    ZRBHCR_REGISTRY_USERNAME="${RBRG_USERNAME}"
  fi

  # Module Variables (ZRBHCR_*)
  ZRBHCR_REGISTRY_HOST="ghcr.io"
  ZRBHCR_REGISTRY_API_BASE="https://ghcr.io/v2/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBHCR_TOKEN_URL="https://ghcr.io/token?scope=repository:${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:pull&service=ghcr.io"

  # Media types
  ZRBHCR_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBHCR_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBHCR_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBHCR_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBHCR_ACCEPT_MANIFEST_MTYPES="${ZRBHCR_MTYPE_DV2},${ZRBHCR_MTYPE_DLIST},${ZRBHCR_MTYPE_OCI},${ZRBHCR_MTYPE_OCM}"
  ZRBHCR_SCHEMA_V2="2"
  ZRBHCR_MTYPE_GHV3="application/vnd.github.v3+json"

  # Curl headers
  ZRBHCR_HEADER_AUTH_TOKEN="Authorization: token ${ZRBHCR_GITHUB_TOKEN}"
  ZRBHCR_HEADER_ACCEPT_GH="Accept: ${ZRBHCR_MTYPE_GHV3}"
  ZRBHCR_HEADER_ACCEPT_MANIFEST="Accept: ${ZRBHCR_ACCEPT_MANIFEST_MTYPES}"

  # File prefixes for all operations
  ZRBHCR_LIST_PAGE_PREFIX="${BURD_TEMP_DIR}/list_page_"
  ZRBHCR_LIST_RECORDS_PREFIX="${BURD_TEMP_DIR}/list_records_"
  ZRBHCR_MANIFEST_PREFIX="${BURD_TEMP_DIR}/manifest_"
  ZRBHCR_CONFIG_PREFIX="${BURD_TEMP_DIR}/config_"
  ZRBHCR_DELETE_PREFIX="${BURD_TEMP_DIR}/delete_"
  ZRBHCR_VERSION_PREFIX="${BURD_TEMP_DIR}/version_"
  ZRBHCR_DETAIL_PREFIX="${BURD_TEMP_DIR}/detail_"

  # Output files
  ZRBHCR_IMAGE_RECORDS_FILE="${BURD_TEMP_DIR}/IMAGE_RECORDS.json"
  ZRBHCR_IMAGE_DETAIL_FILE="${BURD_TEMP_DIR}/IMAGE_DETAILS.json"
  ZRBHCR_IMAGE_STATS_FILE="${BURD_TEMP_DIR}/IMAGE_STATS.json"
  ZRBHCR_FQIN_FILE="${BURD_TEMP_DIR}/FQIN.txt"

  # File index counter
  ZRBHCR_FILE_INDEX=0

  buc_step "Obtaining bearer token for registry API"
  local z_bearer_token
  z_bearer_token=$(zrbhcr_get_bearer_token_subshell) || buc_die "Cannot proceed without bearer token"
  ZRBHCR_REGISTRY_TOKEN="${z_bearer_token}"

  # Registry auth header
  ZRBHCR_HEADER_AUTH_BEARER="Authorization: Bearer ${ZRBHCR_REGISTRY_TOKEN}"

  # Login to registry
  buc_step "Log in to container registry"
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} login "${ZRBHCR_REGISTRY_HOST}" -u "${ZRBHCR_REGISTRY_USERNAME}" -p "${ZRBHCR_GITHUB_TOKEN}"

  ZRBHCR_KINDLED=1
}

zrbhcr_sentinel() {
  test "${ZRBHCR_KINDLED:-}" = "1" || buc_die "Module rbhcr not kindled - call zrbhcr_kindle first"
}

zrbhcr_get_bearer_token_subshell() {
  # Fetch token and extract in memory only
  local z_response
  z_response=$(curl -sL -u "${ZRBHCR_REGISTRY_USERNAME}:${ZRBHCR_GITHUB_TOKEN}" \
    "${ZRBHCR_TOKEN_URL}" 2>/dev/null) || return 1

  local z_token
  z_token=$(echo "${z_response}" | jq -r '.token' 2>/dev/null) || return 1

  test -n "${z_token}"           || return 1
  test    "${z_token}" != "null" || return 1
  echo    "${z_token}"
}

zrbhcr_curl_github_api() {
  local z_url="$1"

  curl -s                              \
       -H "${ZRBHCR_HEADER_AUTH_TOKEN}" \
       -H "${ZRBHCR_HEADER_ACCEPT_GH}"  \
       "${z_url}"
}

zrbhcr_get_next_index() {
  ZRBHCR_FILE_INDEX=$((ZRBHCR_FILE_INDEX + 1))
  printf "%03d" "${ZRBHCR_FILE_INDEX}"
}

zrbhcr_process_single_manifest() {
  local z_tag="$1"
  local z_manifest_file="$2"
  local z_platform="$3"  # Empty for single-platform

  # Get config digest
  local z_config_digest
  z_config_digest=$(jq -r '.config.digest' "${z_manifest_file}")

  test -n "${z_config_digest}" || buc_die "Missing config.digest"
  test "${z_config_digest}" != "null" || {
    buc_warn "null config.digest in manifest"
    return 0
  }

  # Fetch config blob
  local z_idx
  z_idx=$(zrbhcr_get_next_index)
  local z_config_out="${ZRBHCR_CONFIG_PREFIX}${z_idx}.json"
  local z_config_err="${ZRBHCR_CONFIG_PREFIX}${z_idx}.err"

  curl -sL -H "${ZRBHCR_HEADER_AUTH_BEARER}" "${ZRBHCR_REGISTRY_API_BASE}/blobs/${z_config_digest}" \
        >"${z_config_out}" 2>"${z_config_err}" && \
    jq . "${z_config_out}" >/dev/null || {
      buc_warn "Failed to fetch config blob"
      buc_die "Failed to retrieve config blob from registry"
    }

  # Build detail entry
  local z_temp_detail="${ZRBHCR_DETAIL_PREFIX}$(zrbhcr_get_next_index).json"
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
  jq -s '.[0] + [.[1]]' "${ZRBHCR_IMAGE_DETAIL_FILE}" "${z_temp_detail}" \
    > "${ZRBHCR_IMAGE_DETAIL_FILE}.tmp" || buc_die "Failed to merge image detail"
  mv "${ZRBHCR_IMAGE_DETAIL_FILE}.tmp" "${ZRBHCR_IMAGE_DETAIL_FILE}"
}

######################################################################
# External Functions (rbhcr_*)

rbhcr_make_fqin() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Write FQIN to file
  echo "${ZRBHCR_REGISTRY_HOST}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:${z_tag}" > "${ZRBHCR_FQIN_FILE}"
}

rbhcr_list_tags() {
  # Ensure module started
  zrbhcr_sentinel

  buc_step "Fetching all image records with pagination"

  # Initialize empty array
  echo "[]" > "${ZRBHCR_IMAGE_RECORDS_FILE}"

  local z_page=1

  while true; do
    buc_info "Fetching page ${z_page}..."

    local z_temp_page="${ZRBHCR_LIST_PAGE_PREFIX}${z_page}.json"
    local z_temp_records="${ZRBHCR_LIST_RECORDS_PREFIX}${z_page}.json"

    local z_url="https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100&page=${z_page}"
    zrbhcr_curl_github_api "${z_url}" > "${z_temp_page}"

    local z_items
    z_items=$(jq '. | length' "${z_temp_page}")
    buc_info "Saw ${z_items} items on page ${z_page}"

    test "${z_items}" -ne 0 || break

    # Transform to simplified records
    jq -r --arg prefix "${ZRBHCR_REGISTRY_HOST}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}" \
      '[.[] | select(.metadata.container.tags | length > 0) |
       .id as $id | .metadata.container.tags[] as $tag |
       {version_id: $id, tag: $tag, fqin: ($prefix + ":" + $tag)}]' \
      "${z_temp_page}" > "${z_temp_records}"

    # Merge with existing
    jq -s '.[0] + .[1]' "${ZRBHCR_IMAGE_RECORDS_FILE}" "${z_temp_records}" > \
       "${ZRBHCR_IMAGE_RECORDS_FILE}.tmp"
    mv "${ZRBHCR_IMAGE_RECORDS_FILE}.tmp" "${ZRBHCR_IMAGE_RECORDS_FILE}"

    z_page=$((z_page + 1))
  done

  local z_total
  z_total=$(jq '. | length' "${ZRBHCR_IMAGE_RECORDS_FILE}")
  buc_info "Retrieved ${z_total} total image records"
}

rbhcr_get_manifest() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  local z_idx
  z_idx=$(zrbhcr_get_next_index)
  local z_manifest_out="${ZRBHCR_MANIFEST_PREFIX}${z_idx}.json"
  local z_manifest_err="${ZRBHCR_MANIFEST_PREFIX}${z_idx}.err"

  curl -sL \
       -H "${ZRBHCR_HEADER_AUTH_BEARER}" \
       -H "${ZRBHCR_HEADER_ACCEPT_MANIFEST}" \
       "${ZRBHCR_REGISTRY_API_BASE}/manifests/${z_tag}" \
       >"${z_manifest_out}" 2>"${z_manifest_err}" \
    && jq . "${z_manifest_out}" >/dev/null \
    || {
      buc_warn "Failed to fetch manifest for ${z_tag}"
      buc_die "This image appears corrupted"
    }

  local z_media_type
  z_media_type=$(jq -r '.mediaType // .schemaVersion' "${z_manifest_out}")

  if [[ "${z_media_type}" == "${ZRBHCR_MTYPE_DLIST}" ]] || \
     [[ "${z_media_type}" == "${ZRBHCR_MTYPE_OCI}" ]]; then

    buc_info "Multi-platform image detected"

    local z_platform_idx=1
    local z_manifests
    z_manifests=$(jq -c '.manifests[]' "${z_manifest_out}")

    while IFS= read -r z_platform_manifest; do
      local z_platform_digest z_platform_info
      z_platform_digest=$(echo "${z_platform_manifest}" | jq -r '.digest')
      z_platform_info=$(echo "${z_platform_manifest}" | jq -r '"\(.platform.os)/\(.platform.architecture)"')

      buc_info "Processing platform: ${z_platform_info}"

      local z_platform_idx_str
      z_platform_idx_str=$(zrbhcr_get_next_index)
      local z_platform_out="${ZRBHCR_MANIFEST_PREFIX}${z_platform_idx_str}.json"
      local z_platform_err="${ZRBHCR_MANIFEST_PREFIX}${z_platform_idx_str}.err"

      curl -sL \
           -H "${ZRBHCR_HEADER_AUTH_BEARER}" \
           -H "${ZRBHCR_HEADER_ACCEPT_MANIFEST}" \
           "${ZRBHCR_REGISTRY_API_BASE}/manifests/${z_platform_digest}" \
           >"${z_platform_out}" 2>"${z_platform_err}" \
        && jq . "${z_platform_out}" >/dev/null \
        || {
          buc_warn "Failed to fetch platform manifest"
          ((z_platform_idx++))
          continue
        }

      zrbhcr_process_single_manifest "${z_tag}" "${z_platform_out}" "${z_platform_info}"

      ((z_platform_idx++))
    done <<< "${z_manifests}"

  else
    buc_info "Single platform image"
    zrbhcr_process_single_manifest "${z_tag}" "${z_manifest_out}" ""
  fi
}

rbhcr_get_config() {
  # Name parameters
  local z_digest="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_digest}" || buc_die "Digest parameter required"

  # Fetch config blob
  local z_idx
  z_idx=$(zrbhcr_get_next_index)
  local z_config_out="${ZRBHCR_CONFIG_PREFIX}${z_idx}.json"

  curl -sL -H "${ZRBHCR_HEADER_AUTH_BEARER}" \
       "${ZRBHCR_REGISTRY_API_BASE}/blobs/${z_digest}" > "${z_config_out}"
}

rbhcr_delete() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Get version ID for tag
  rbhcr_get_version_id "${z_tag}"
  local z_version_id
  z_version_id=$(<"${ZRBHCR_VERSION_ID_FILE}")

  # Delete via GitHub API
  local z_delete_url="https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions/${z_version_id}"
  local z_result="${ZRBHCR_DELETE_PREFIX}result.txt"
  local z_status_file="${ZRBHCR_DELETE_PREFIX}status.txt"

  # Use -w to write status to separate file to avoid subshell
  curl -X DELETE -s \
    -H "${ZRBHCR_HEADER_AUTH_TOKEN}" \
    -H "${ZRBHCR_HEADER_ACCEPT_GH}" \
    -w "%{http_code}" \
    -o "${z_result}" \
    "${z_delete_url}" > "${z_status_file}"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test "${z_http_code}" = "204" || buc_die "Delete failed with HTTP ${z_http_code}"
}

rbhcr_pull() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Construct FQIN from tag
  rbhcr_make_fqin "${z_tag}"
  local z_fqin
  z_fqin=$(<"${ZRBHCR_FQIN_FILE}")

  buc_info "Fetch image..."
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} pull "${z_fqin}"
}

rbhcr_exists_predicate() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Check if tag exists
  local z_url="https://api.github.com/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100"
  zrbhcr_curl_github_api "${z_url}" | \
    jq -e '.[] | select(.metadata.container.tags[] | contains("'"${z_tag}"'"))' > /dev/null
}

rbhcr_get_version_id() {
  # Name parameters
  local z_tag="${1:-}"

  # Ensure module started
  zrbhcr_sentinel

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Find version ID
  ZRBHCR_VERSION_ID_FILE="${ZRBHCR_VERSION_PREFIX}id.txt"

  rbhcr_list_tags

  jq -r '.[] | select(.tag == "'"${z_tag}"'") | .version_id' \
    "${ZRBHCR_IMAGE_RECORDS_FILE}" > "${ZRBHCR_VERSION_ID_FILE}"

  test -s "${ZRBHCR_VERSION_ID_FILE}" || buc_die "Version ID not found for tag: ${z_tag}"
}

# eof


