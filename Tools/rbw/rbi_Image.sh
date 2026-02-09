#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle Image Management - Read-Only GAR Registry Operations

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBI_SOURCED:-}" || buc_die "Module rbi multiply sourced - check sourcing hierarchy"
ZRBI_SOURCED=1

######################################################################
# Internal Functions (zrbi_*)

zrbi_kindle() {
  test -z "${ZRBI_KINDLED:-}" || buc_die "Module rbi already kindled"

  # Verify RBGO is available
  test "${ZRBGO_KINDLED:-}" = "1" || buc_die "Module rbgo not kindled - must kindle rbgo before rbi"

  # Check required environment
  test -n "${RBRR_GAR_REPOSITORY:-}" || buc_die "RBRR_GAR_REPOSITORY not set"
  test -n "${BURD_TEMP_DIR:-}"        || buc_die "BURD_TEMP_DIR not set"

  # Verify GAR service account file is configured
  test -n "${RBRR_RETRIEVER_RBRA_FILE:-}"   || buc_die "RBRR_RETRIEVER_RBRA_FILE not set"
  test -f "${RBRR_RETRIEVER_RBRA_FILE}"     || buc_die "GAR service env file not found: ${RBRR_RETRIEVER_RBRA_FILE}"

  # Module Variables (ZRBI_*)
  ZRBI_REGISTRY_HOST="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  ZRBI_REGISTRY_PATH="${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBI_REGISTRY_API_BASE="https://${ZRBI_REGISTRY_HOST}/v2/${ZRBI_REGISTRY_PATH}"
  ZRBI_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  ZRBI_GAR_PACKAGE_BASE="projects/${RBGD_GAR_PROJECT_ID}/locations/${RBGD_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  # Media types
  ZRBI_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBI_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBI_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBI_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBI_ACCEPT_MANIFEST_MTYPES="${ZRBI_MTYPE_DV2},${ZRBI_MTYPE_DLIST},${ZRBI_MTYPE_OCI},${ZRBI_MTYPE_OCM}"

  # File prefixes for all operations
  ZRBI_MANIFEST_PREFIX="${BURD_TEMP_DIR}/rbi_manifest_"
  ZRBI_CONFIG_PREFIX="${BURD_TEMP_DIR}/rbi_config_"
  ZRBI_DETAIL_PREFIX="${BURD_TEMP_DIR}/rbi_detail_"
  ZRBI_TOKEN_PREFIX="${BURD_TEMP_DIR}/rbi_token_"
  ZRBI_TAGS_PREFIX="${BURD_TEMP_DIR}/rbi_tags_"
  ZRBI_METADATA_PREFIX="${BURD_TEMP_DIR}/rbi_metadata_"

  # Output files
  ZRBI_IMAGE_RECORDS_FILE="${BURD_TEMP_DIR}/rbi_IMAGE_RECORDS.json"
  ZRBI_IMAGE_DETAIL_FILE="${BURD_TEMP_DIR}/rbi_IMAGE_DETAILS.json"
  ZRBI_IMAGE_STATS_FILE="${BURD_TEMP_DIR}/rbi_IMAGE_STATS.json"
  ZRBI_FQIN_FILE="${BURD_TEMP_DIR}/rbi_FQIN.txt"
  ZRBI_TOKEN_FILE="${ZRBI_TOKEN_PREFIX}access.txt"
  ZRBI_METADATA_ARCHIVE="${BURD_TEMP_DIR}/rbi_metadata.tgz"

  # File index counter
  ZRBI_FILE_INDEX=0

  # Initialize detail file
  echo "[]" > "${ZRBI_IMAGE_DETAIL_FILE}" || buc_die "Failed to initialize detail file"

  # Obtain initial OAuth token
  zrbi_refresh_token || buc_die "Cannot proceed without OAuth token"

  ZRBI_KINDLED=1
}

zrbi_sentinel() {
  test "${ZRBI_KINDLED:-}" = "1" || buc_die "Module rbi not kindled - call zrbi_kindle first"
}

zrbi_refresh_token() {
  # No sentinel check - called from kindle before KINDLED=1
  buc_log_args "Obtaining OAuth token for GAR API"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_RETRIEVER_RBRA_FILE}") || buc_die "Failed to get OAuth token from RBGO"
  echo "${z_token}" > "${ZRBI_TOKEN_FILE}" || buc_die "Failed to write token file"
}

zrbi_get_next_index_capture() {
  zrbi_sentinel

  ZRBI_FILE_INDEX=$((ZRBI_FILE_INDEX + 1))
  printf "%03d" "${ZRBI_FILE_INDEX}"
}

zrbi_curl_registry() {
  zrbi_sentinel

  local z_url="$1"
  local z_token
  z_token=$(<"${ZRBI_TOKEN_FILE}")
  test -n "${z_token}" || buc_die "Token is empty"

  curl -sL                                          \
      -H "Authorization: Bearer ${z_token}"         \
      -H "Accept: ${ZRBI_ACCEPT_MANIFEST_MTYPES}"   \
      "${z_url}"                                    \
    || buc_die "Registry API call failed: ${z_url}"
}

zrbi_process_single_manifest() {
  zrbi_sentinel

  local z_tag="$1"
  local z_manifest_file="$2"
  local z_platform="${3:-}"  # Empty for single-platform

  # Get config digest
  local z_config_digest_file="${ZRBI_CONFIG_PREFIX}digest.txt"
  jq -r '.config.digest' "${z_manifest_file}" > "${z_config_digest_file}" || buc_die "Failed to extract config digest"

  local z_config_digest
  z_config_digest=$(<"${z_config_digest_file}")
  test -n "${z_config_digest}" || buc_die "Config digest is empty"
  test "${z_config_digest}" != "null" || {
    buc_warn "null config.digest in manifest"
    return 0
  }

  # Fetch config blob
  local z_idx
  z_idx=$(zrbi_get_next_index_capture) || buc_die "Failed to get next index"
  local z_config_out="${ZRBI_CONFIG_PREFIX}${z_idx}.json"

  zrbi_curl_registry "${ZRBI_REGISTRY_API_BASE}/blobs/${z_config_digest}" > "${z_config_out}" \
    || buc_die "Failed to fetch config blob"

  # Validating config JSON
  jq . "${z_config_out}" > /dev/null || buc_die "Invalid config JSON"

  # Build detail entry
  local z_detail_idx
  z_detail_idx=$(zrbi_get_next_index_capture) || buc_die "Failed to get detail index"
  local z_temp_detail="${ZRBI_DETAIL_PREFIX}${z_detail_idx}.json"

  local z_manifest_json=$(<"${z_manifest_file}")
  test -n "${z_manifest_json}" || buc_die "Manifest JSON is empty"

  # Normalize config with defaults
  local z_config_normalized="${ZRBI_CONFIG_PREFIX}normalized_${z_idx}.json"
  jq '. + {
        created: (.created // "1970-01-01T00:00:00Z"),
        architecture: (.architecture // "unknown"),
        os: (.os // "unknown")
      }' "${z_config_out}" > "${z_config_normalized}" \
    || buc_die "Failed to normalize config"

  local z_config_json=$(<"${z_config_normalized}")
  test -n "${z_config_json}" || buc_die "Normalized config is empty"

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
      }' > "${z_temp_detail}" || buc_die "Failed to build platform detail"
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
      }' > "${z_temp_detail}" || buc_die "Failed to build single detail"
  fi

  # Append to detail file
  buc_log_args "Merging image detail"
  local    z_detail_tmp="${ZRBI_IMAGE_DETAIL_FILE}.tmp"
  jq -s '.[0] + [.[1]]' "${ZRBI_IMAGE_DETAIL_FILE}" "${z_temp_detail}" \
    > "${z_detail_tmp}" || buc_die "Failed to merge image detail"
  mv  "${z_detail_tmp}" "${ZRBI_IMAGE_DETAIL_FILE}" || buc_die "Failed to move detail file"
}

######################################################################
# External Functions (rbi_*)

rbi_list() {
  zrbi_sentinel

  # Documentation block
  buc_doc_brief "List all available image tags from registry"
  buc_doc_shown || return 0

  buc_step "Fetching image tags from GAR"

  # GAR uses Docker Registry v2 API - list tags endpoint
  local z_tags_response="${ZRBI_TAGS_PREFIX}response.json"

  zrbi_curl_registry "${ZRBI_REGISTRY_API_BASE}/tags/list" > "${z_tags_response}" \
    || buc_die "Failed to fetch tags list"

  # Transform to records format
  jq -r --arg prefix "${ZRBI_REGISTRY_HOST}/${ZRBI_REGISTRY_PATH}" \
      '[.tags[] | {tag: ., fqin: ($prefix + "/" + .)}]'              \
      "${z_tags_response}" > "${ZRBI_IMAGE_RECORDS_FILE}"           \
    || buc_die "Failed to transform tags"

  local z_total_file="${ZRBI_TAGS_PREFIX}total.txt"
  jq '. | length' "${ZRBI_IMAGE_RECORDS_FILE}" > "${z_total_file}" || buc_die "Failed to count tags"

  local z_total
  z_total=$(<"${z_total_file}")
  test -n "${z_total}" || buc_die "Total count is empty"

  buc_info "Retrieved ${z_total} total image records"

  # Display tags to stdout
  echo "Repository: ${ZRBI_REGISTRY_HOST}/${ZRBI_REGISTRY_PATH}"
  echo "Images:"
  jq -r '.[] | .tag' "${ZRBI_IMAGE_RECORDS_FILE}" | sort -r | head -20

  test "${z_total}" -gt 20 && echo "... (showing 20 of ${z_total} tags)"

  buc_success "List complete - ${z_total} images"
}

rbi_show() {
  zrbi_sentinel

  local z_tag="${1:-}"

  # Documentation block
  buc_doc_brief "Fetch and display manifest/config for an image tag"
  buc_doc_param "tag" "Image tag to fetch manifest for"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  buc_step "Fetching manifest for: ${z_tag}"

  local z_idx
  z_idx=$(zrbi_get_next_index_capture) || buc_die "Failed to get index"
  local z_manifest_out="${ZRBI_MANIFEST_PREFIX}${z_idx}.json"

  zrbi_curl_registry "${ZRBI_REGISTRY_API_BASE}/manifests/${z_tag}" > "${z_manifest_out}" \
    || buc_die "Failed to fetch manifest for ${z_tag}"

  jq . "${z_manifest_out}" >/dev/null || buc_die "Invalid manifest JSON"

  local z_media_type_file="${ZRBI_MANIFEST_PREFIX}mediatype_${z_idx}.txt"
  jq -r '.mediaType // .schemaVersion' "${z_manifest_out}" > "${z_media_type_file}" \
    || buc_die "Failed to extract media type"

  local z_media_type=$(<"${z_media_type_file}")
  test -n "${z_media_type}" || buc_die "Failed to read or empty: ${z_media_type_file}"

  if test "${z_media_type}" = "${ZRBI_MTYPE_DLIST}" || \
     test "${z_media_type}" = "${ZRBI_MTYPE_OCI}"; then

    buc_info "Multi-platform image detected"

    local z_manifests_file="${ZRBI_MANIFEST_PREFIX}list_${z_idx}.jsonl"
    jq -c '.manifests[]' "${z_manifest_out}" > "${z_manifests_file}" \
      || buc_die "Failed to extract manifests"

    local z_platform_idx=0
    while IFS= read -r z_platform_manifest; do
      local z_platform_digest_file="${ZRBI_MANIFEST_PREFIX}digest_${z_platform_idx}.txt"
      jq -r '.digest' <<<"${z_platform_manifest}" > "${z_platform_digest_file}" \
        || buc_die "Failed to extract platform digest"

      local z_platform_digest=$(<"${z_platform_digest_file}")
      test -n "${z_platform_digest}" || buc_die "Failed to read or empty: ${z_platform_digest_file}"

      local z_platform_info_file="${ZRBI_MANIFEST_PREFIX}info_${z_platform_idx}.txt"
      jq -r '"\(.platform.os)/\(.platform.architecture)"' <<<"${z_platform_manifest}" > "${z_platform_info_file}" \
        || buc_die "Failed to extract platform info"

      local z_platform_info=$(<"${z_platform_info_file}")
      test -n "${z_platform_info}" || buc_die "Failed to read or empty: ${z_platform_info_file}"

      buc_info "Processing platform: ${z_platform_info}"

      local z_platform_idx_str
      z_platform_idx_str=$(zrbi_get_next_index_capture) || buc_die "Failed to get platform index"
      local z_platform_out="${ZRBI_MANIFEST_PREFIX}${z_platform_idx_str}.json"

      zrbi_curl_registry "${ZRBI_REGISTRY_API_BASE}/manifests/${z_platform_digest}" \
        > "${z_platform_out}" || buc_die "Failed to fetch platform manifest"

      jq . "${z_platform_out}" > /dev/null || buc_die "Invalid platform manifest JSON"

      zrbi_process_single_manifest "${z_tag}" "${z_platform_out}" "${z_platform_info}"

      z_platform_idx=$((z_platform_idx + 1))
    done < "${z_manifests_file}"

  else
    buc_info "Single platform image"
    zrbi_process_single_manifest "${z_tag}" "${z_manifest_out}" ""
  fi

  # Display summary
  echo "Image: ${z_tag}"
  echo "Details saved to: ${ZRBI_IMAGE_DETAIL_FILE}"
  jq '.' "${ZRBI_IMAGE_DETAIL_FILE}" || buc_warn "Failed to display details"

  buc_success "Manifest and config retrieved"
}

rbi_metadata() {
  zrbi_sentinel

  local z_tag="${1:-}"

  buc_doc_brief "Download GAR build metadata archive for a tag"
  buc_doc_param "tag" "Image tag to fetch metadata for"
  buc_doc_shown || return 0

  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Refresh token to avoid expiry
  zrbi_refresh_token

  local z_token
  z_token=$(<"${ZRBI_TOKEN_FILE}") || buc_die "Token read failed"
  test -n "${z_token}" || buc_die "Empty token"

  local z_package_path="${ZRBI_GAR_API_BASE}/${ZRBI_GAR_PACKAGE_BASE}/packages/${z_tag}"

  buc_step "Downloading GAR metadata for: ${z_tag}"
  curl -s                                             \
       -H "Authorization: Bearer ${z_token}"          \
       "${z_package_path}/versions/metadata:download" \
       -o "${ZRBI_METADATA_ARCHIVE}"                  \
    || buc_die "Failed to download metadata"

  test -f "${ZRBI_METADATA_ARCHIVE}" || buc_die "Metadata archive not created"
  test -s "${ZRBI_METADATA_ARCHIVE}" || buc_die "Metadata archive is empty"

  local z_extract_dir="${ZRBI_METADATA_PREFIX}${z_tag}"
  rm -rf "${z_extract_dir}" || buc_warn "Failed to clean previous extract dir"
  mkdir -p "${z_extract_dir}" || buc_die "Failed to create extract directory"
  tar -xzf "${ZRBI_METADATA_ARCHIVE}" -C "${z_extract_dir}" || buc_die "Failed to extract metadata"

  if test -f "${z_extract_dir}/package_summary.txt"; then
    buc_info "Top packages in image:"
    head -5 "${z_extract_dir}/package_summary.txt" || buc_warn "Failed to show package summary"
  fi

  buc_success "Metadata retrieved to ${z_extract_dir}"
}

rbi_fqin() {
  zrbi_sentinel

  local z_tag="${1:-}"

  # Documentation block
  buc_doc_brief "Create fully qualified image name"
  buc_doc_param "tag" "Image tag to qualify"
  buc_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || buc_die "Tag parameter required"

  # Write FQIN to file
  local z_fqin="${ZRBI_REGISTRY_HOST}/${ZRBI_REGISTRY_PATH}/${z_tag}"
  echo "${z_fqin}" > "${ZRBI_FQIN_FILE}" || buc_die "Failed to write FQIN"

  # Also output to stdout for direct use
  echo "${z_fqin}"

  buc_success "FQIN generated"
}

# eof

