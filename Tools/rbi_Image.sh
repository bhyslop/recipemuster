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
# Recipe Bottle Container Client - Read-Only GAR Registry Interface

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBC_SOURCED:-}" || bcu_die "Module rbc multiply sourced - check sourcing hierarchy"
ZRBC_SOURCED=1

######################################################################
# Internal Functions (zrbc_*)

zrbc_kindle() {
  test -z "${ZRBC_KINDLED:-}" || bcu_die "Module rbc already kindled"

  # Verify RBGO is available
  test "${ZRBGO_KINDLED:-}" = "1" || bcu_die "Module rbgo not kindled - must kindle rbgo before rbc"

  # Check required environment
  test -n "${RBRR_GAR_PROJECT_ID:-}" || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"   || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}" || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${BDU_TEMP_DIR:-}"        || bcu_die "BDU_TEMP_DIR not set"

  # Verify GAR service account file is configured
  test -n "${RBRR_GAR_RBRA_FILE:-}"   || bcu_die "RBRR_GAR_RBRA_FILE not set"
  test -f "${RBRR_GAR_RBRA_FILE}"     || bcu_die "GAR service env file not found: ${RBRR_GAR_RBRA_FILE}"

  # Module Variables (ZRBC_*)
  ZRBC_REGISTRY_HOST="${RBRR_GAR_LOCATION}-docker.pkg.dev"
  ZRBC_REGISTRY_PATH="${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBC_REGISTRY_API_BASE="https://${ZRBC_REGISTRY_HOST}/v2/${ZRBC_REGISTRY_PATH}"
  ZRBC_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  ZRBC_GAR_PACKAGE_BASE="projects/${RBRR_GAR_PROJECT_ID}/locations/${RBRR_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  # Media types
  ZRBC_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBC_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBC_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBC_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBC_ACCEPT_MANIFEST_MTYPES="${ZRBC_MTYPE_DV2},${ZRBC_MTYPE_DLIST},${ZRBC_MTYPE_OCI},${ZRBC_MTYPE_OCM}"

  # File prefixes for all operations
  ZRBC_MANIFEST_PREFIX="${BDU_TEMP_DIR}/rbc_manifest_"
  ZRBC_CONFIG_PREFIX="${BDU_TEMP_DIR}/rbc_config_"
  ZRBC_DETAIL_PREFIX="${BDU_TEMP_DIR}/rbc_detail_"
  ZRBC_TOKEN_PREFIX="${BDU_TEMP_DIR}/rbc_token_"
  ZRBC_TAGS_PREFIX="${BDU_TEMP_DIR}/rbc_tags_"
  ZRBC_METADATA_PREFIX="${BDU_TEMP_DIR}/rbc_metadata_"

  # Output files
  ZRBC_IMAGE_RECORDS_FILE="${BDU_TEMP_DIR}/rbc_IMAGE_RECORDS.json"
  ZRBC_IMAGE_DETAIL_FILE="${BDU_TEMP_DIR}/rbc_IMAGE_DETAILS.json"
  ZRBC_IMAGE_STATS_FILE="${BDU_TEMP_DIR}/rbc_IMAGE_STATS.json"
  ZRBC_FQIN_FILE="${BDU_TEMP_DIR}/rbc_FQIN.txt"
  ZRBC_TOKEN_FILE="${ZRBC_TOKEN_PREFIX}access.txt"
  ZRBC_METADATA_ARCHIVE="${BDU_TEMP_DIR}/rbc_metadata.tgz"

  # File index counter
  ZRBC_FILE_INDEX=0

  # Initialize detail file
  echo "[]" > "${ZRBC_IMAGE_DETAIL_FILE}" || bcu_die "Failed to initialize detail file"

  # Obtain initial OAuth token
  zrbc_refresh_token || bcu_die "Cannot proceed without OAuth token"

  ZRBC_KINDLED=1
}

zrbc_sentinel() {
  test "${ZRBC_KINDLED:-}" = "1" || bcu_die "Module rbc not kindled - call zrbc_kindle first"
}

zrbc_refresh_token() {
  # No sentinel check - called from kindle before KINDLED=1
  bcu_log_args "Obtaining OAuth token for GAR API"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_GAR_RBRA_FILE}") || bcu_die "Failed to get OAuth token from RBGO"
  echo "${z_token}" > "${ZRBC_TOKEN_FILE}" || bcu_die "Failed to write token file"
}

zrbc_get_next_index_capture() {
  zrbc_sentinel

  ZRBC_FILE_INDEX=$((ZRBC_FILE_INDEX + 1))
  printf "%03d" "${ZRBC_FILE_INDEX}"
}

zrbc_curl_registry() {
  zrbc_sentinel

  local z_url="$1"
  local z_token
  z_token=$(<"${ZRBC_TOKEN_FILE}")
  test -n "${z_token}" || bcu_die "Token is empty"

  curl -sL                                          \
      -H "Authorization: Bearer ${z_token}"         \
      -H "Accept: ${ZRBC_ACCEPT_MANIFEST_MTYPES}"  \
      "${z_url}"                                    \
    || bcu_die "Registry API call failed: ${z_url}"
}

zrbc_process_single_manifest() {
  zrbc_sentinel

  local z_tag="$1"
  local z_manifest_file="$2"
  local z_platform="${3:-}"  # Empty for single-platform

  # Get config digest
  local z_config_digest_file="${ZRBC_CONFIG_PREFIX}digest.txt"
  jq -r '.config.digest' "${z_manifest_file}" > "${z_config_digest_file}" || bcu_die "Failed to extract config digest"

  local z_config_digest
  z_config_digest=$(<"${z_config_digest_file}")
  test -n "${z_config_digest}" || bcu_die "Config digest is empty"
  test "${z_config_digest}" != "null" || {
    bcu_warn "null config.digest in manifest"
    return 0
  }

  # Fetch config blob
  local z_idx
  z_idx=$(zrbc_get_next_index_capture) || bcu_die "Failed to get next index"
  local z_config_out="${ZRBC_CONFIG_PREFIX}${z_idx}.json"

  zrbc_curl_registry "${ZRBC_REGISTRY_API_BASE}/blobs/${z_config_digest}" > "${z_config_out}" \
    || bcu_die "Failed to fetch config blob"

  # Validating config JSON
  jq . "${z_config_out}" > /dev/null || bcu_die "Invalid config JSON"

  # Build detail entry
  local z_detail_idx
  z_detail_idx=$(zrbc_get_next_index_capture) || bcu_die "Failed to get detail index"
  local z_temp_detail="${ZRBC_DETAIL_PREFIX}${z_detail_idx}.json"

  local z_manifest_json=$(<"${z_manifest_file}")
  test -n "${z_manifest_json}" || bcu_die "Manifest JSON is empty"

  # Normalize config with defaults
  local z_config_normalized="${ZRBC_CONFIG_PREFIX}normalized_${z_idx}.json"
  jq '. + {
        created: (.created // "1970-01-01T00:00:00Z"),
        architecture: (.architecture // "unknown"),
        os: (.os // "unknown")
      }' "${z_config_out}" > "${z_config_normalized}" \
    || bcu_die "Failed to normalize config"

  local z_config_json=$(<"${z_config_normalized}")
  test -n "${z_config_json}" || bcu_die "Normalized config is empty"

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
      }' > "${z_temp_detail}" || bcu_die "Failed to build platform detail"
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
      }' > "${z_temp_detail}" || bcu_die "Failed to build single detail"
  fi

  # Append to detail file
  bcu_log_args "Merging image detail"
  local    z_detail_tmp="${ZRBC_IMAGE_DETAIL_FILE}.tmp"
  jq -s '.[0] + [.[1]]' "${ZRBC_IMAGE_DETAIL_FILE}" "${z_temp_detail}" \
    > "${z_detail_tmp}" || bcu_die "Failed to merge image detail"
  mv  "${z_detail_tmp}" "${ZRBC_IMAGE_DETAIL_FILE}" || bcu_die "Failed to move detail file"
}

######################################################################
# External Functions (rbc_*)

rbc_list() {
  zrbc_sentinel

  # Documentation block
  bcu_doc_brief "List all available image tags from registry"
  bcu_doc_shown || return 0

  bcu_step "Fetching image tags from GAR"

  # GAR uses Docker Registry v2 API - list tags endpoint
  local z_tags_response="${ZRBC_TAGS_PREFIX}response.json"

  zrbc_curl_registry "${ZRBC_REGISTRY_API_BASE}/tags/list" > "${z_tags_response}" \
    || bcu_die "Failed to fetch tags list"

  # Transform to records format
  jq -r --arg prefix "${ZRBC_REGISTRY_HOST}/${ZRBC_REGISTRY_PATH}" \
      '[.tags[] | {tag: ., fqin: ($prefix + "/" + .)}]'              \
      "${z_tags_response}" > "${ZRBC_IMAGE_RECORDS_FILE}"           \
    || bcu_die "Failed to transform tags"

  local z_total_file="${ZRBC_TAGS_PREFIX}total.txt"
  jq '. | length' "${ZRBC_IMAGE_RECORDS_FILE}" > "${z_total_file}" || bcu_die "Failed to count tags"

  local z_total
  z_total=$(<"${z_total_file}")
  test -n "${z_total}" || bcu_die "Total count is empty"

  bcu_info "Retrieved ${z_total} total image records"

  # Display tags to stdout
  echo "Repository: ${ZRBC_REGISTRY_HOST}/${ZRBC_REGISTRY_PATH}"
  echo "Images:"
  jq -r '.[] | .tag' "${ZRBC_IMAGE_RECORDS_FILE}" | sort -r | head -20

  test "${z_total}" -gt 20 && echo "... (showing 20 of ${z_total} tags)"

  bcu_success "List complete - ${z_total} images"
}

rbc_show() {
  zrbc_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Fetch and display manifest/config for an image tag"
  bcu_doc_param "tag" "Image tag to fetch manifest for"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  bcu_step "Fetching manifest for: ${z_tag}"

  local z_idx
  z_idx=$(zrbc_get_next_index_capture) || bcu_die "Failed to get index"
  local z_manifest_out="${ZRBC_MANIFEST_PREFIX}${z_idx}.json"

  zrbc_curl_registry "${ZRBC_REGISTRY_API_BASE}/manifests/${z_tag}" > "${z_manifest_out}" \
    || bcu_die "Failed to fetch manifest for ${z_tag}"

  jq . "${z_manifest_out}" >/dev/null || bcu_die "Invalid manifest JSON"

  local z_media_type_file="${ZRBC_MANIFEST_PREFIX}mediatype_${z_idx}.txt"
  jq -r '.mediaType // .schemaVersion' "${z_manifest_out}" > "${z_media_type_file}" \
    || bcu_die "Failed to extract media type"

  local z_media_type=$(<"${z_media_type_file}")
  test -n "${z_media_type}" || bcu_die "Failed to read or empty: ${z_media_type_file}"

  if test "${z_media_type}" = "${ZRBC_MTYPE_DLIST}" || \
     test "${z_media_type}" = "${ZRBC_MTYPE_OCI}"; then

    bcu_info "Multi-platform image detected"

    local z_manifests_file="${ZRBC_MANIFEST_PREFIX}list_${z_idx}.jsonl"
    jq -c '.manifests[]' "${z_manifest_out}" > "${z_manifests_file}" \
      || bcu_die "Failed to extract manifests"

    local z_platform_idx=0
    while IFS= read -r z_platform_manifest; do
      local z_platform_digest_file="${ZRBC_MANIFEST_PREFIX}digest_${z_platform_idx}.txt"
      jq -r '.digest' <<<"${z_platform_manifest}" > "${z_platform_digest_file}" \
        || bcu_die "Failed to extract platform digest"

      local z_platform_digest=$(<"${z_platform_digest_file}")
      test -n "${z_platform_digest}" || bcu_die "Failed to read or empty: ${z_platform_digest_file}"

      local z_platform_info_file="${ZRBC_MANIFEST_PREFIX}info_${z_platform_idx}.txt"
      jq -r '"\(.platform.os)/\(.platform.architecture)"' <<<"${z_platform_manifest}" > "${z_platform_info_file}" \
        || bcu_die "Failed to extract platform info"

      local z_platform_info=$(<"${z_platform_info_file}")
      test -n "${z_platform_info}" || bcu_die "Failed to read or empty: ${z_platform_info_file}"

      bcu_info "Processing platform: ${z_platform_info}"

      local z_platform_idx_str
      z_platform_idx_str=$(zrbc_get_next_index_capture) || bcu_die "Failed to get platform index"
      local z_platform_out="${ZRBC_MANIFEST_PREFIX}${z_platform_idx_str}.json"

      zrbc_curl_registry "${ZRBC_REGISTRY_API_BASE}/manifests/${z_platform_digest}" \
        > "${z_platform_out}" || bcu_die "Failed to fetch platform manifest"

      jq . "${z_platform_out}" > /dev/null || bcu_die "Invalid platform manifest JSON"

      zrbc_process_single_manifest "${z_tag}" "${z_platform_out}" "${z_platform_info}"

      z_platform_idx=$((z_platform_idx + 1))
    done < "${z_manifests_file}"

  else
    bcu_info "Single platform image"
    zrbc_process_single_manifest "${z_tag}" "${z_manifest_out}" ""
  fi

  # Display summary
  echo "Image: ${z_tag}"
  echo "Details saved to: ${ZRBC_IMAGE_DETAIL_FILE}"
  jq '.' "${ZRBC_IMAGE_DETAIL_FILE}" || bcu_warn "Failed to display details"

  bcu_success "Manifest and config retrieved"
}

rbc_metadata() {
  zrbc_sentinel

  local z_tag="${1:-}"

  bcu_doc_brief "Download GAR build metadata archive for a tag"
  bcu_doc_param "tag" "Image tag to fetch metadata for"
  bcu_doc_shown || return 0

  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Refresh token to avoid expiry
  zrbc_refresh_token

  local z_token
  z_token=$(<"${ZRBC_TOKEN_FILE}") || bcu_die "Token read failed"
  test -n "${z_token}" || bcu_die "Empty token"

  local z_package_path="${ZRBC_GAR_API_BASE}/${ZRBC_GAR_PACKAGE_BASE}/packages/${z_tag}"

  bcu_step "Downloading GAR metadata for: ${z_tag}"
  curl -s \
       -H "Authorization: Bearer ${z_token}" \
       "${z_package_path}/versions/metadata:download" \
       -o "${ZRBC_METADATA_ARCHIVE}" \
    || bcu_die "Failed to download metadata"

  test -f "${ZRBC_METADATA_ARCHIVE}" || bcu_die "Metadata archive not created"
  test -s "${ZRBC_METADATA_ARCHIVE}" || bcu_die "Metadata archive is empty"

  local z_extract_dir="${ZRBC_METADATA_PREFIX}${z_tag}"
  rm -rf "${z_extract_dir}" || bcu_warn "Failed to clean previous extract dir"
  mkdir -p "${z_extract_dir}" || bcu_die "Failed to create extract directory"
  tar -xzf "${ZRBC_METADATA_ARCHIVE}" -C "${z_extract_dir}" || bcu_die "Failed to extract metadata"

  if test -f "${z_extract_dir}/package_summary.txt"; then
    bcu_info "Top packages in image:"
    head -5 "${z_extract_dir}/package_summary.txt" || bcu_warn "Failed to show package summary"
  fi

  bcu_success "Metadata retrieved to ${z_extract_dir}"
}

rbc_fqin() {
  zrbc_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Create fully qualified image name"
  bcu_doc_param "tag" "Image tag to qualify"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Write FQIN to file
  local z_fqin="${ZRBC_REGISTRY_HOST}/${ZRBC_REGISTRY_PATH}/${z_tag}"
  echo "${z_fqin}" > "${ZRBC_FQIN_FILE}" || bcu_die "Failed to write FQIN"

  # Also output to stdout for direct use
  echo "${z_fqin}"

  bcu_success "FQIN generated"
}

# eof

