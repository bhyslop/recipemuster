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
# Recipe Bottle Container Registry - Google Artifact Registry interface

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBCR_SOURCED:-}" || bcu_die "Module rbcr multiply sourced - check sourcing hierarchy"
ZRBCR_SOURCED=1

######################################################################
# Internal Functions (zrbcr_*)

zrbcr_kindle() {
  test -z "${ZRBCR_KINDLED:-}" || bcu_die "Module rbcr already kindled"

  # Verify RBGO is available
  test "${ZRBGO_KINDLED:-}" = "1" || bcu_die "Module rbgo not kindled - must kindle rbgo before rbcr"

  # Check required environment
  test -n "${RBRR_GAR_PROJECT_ID:-}" || bcu_die "RBRR_GAR_PROJECT_ID not set"
  test -n "${RBRR_GAR_LOCATION:-}"   || bcu_die "RBRR_GAR_LOCATION not set"
  test -n "${RBRR_GAR_REPOSITORY:-}" || bcu_die "RBRR_GAR_REPOSITORY not set"
  test -n "${RBG_RUNTIME:-}"         || bcu_die "RBG_RUNTIME not set"
  test -n "${BDU_TEMP_DIR:-}"        || bcu_die "BDU_TEMP_DIR not set"

  # Verify GAR service account file is configured
  test -n "${RBRR_GAR_RBRA_FILE:-}"   || bcu_die "RBRR_GAR_RBRA_FILE not set"
  test -f "${RBRR_GAR_RBRA_FILE}"     || bcu_die "GAR service env file not found: ${RBRR_GAR_RBRA_FILE}"

  # Module Variables (ZRBCR_*)
  ZRBCR_REGISTRY_HOST="${RBRR_GAR_LOCATION}-docker.pkg.dev"
  ZRBCR_REGISTRY_PATH="${RBRR_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBCR_REGISTRY_API_BASE="https://${ZRBCR_REGISTRY_HOST}/v2/${ZRBCR_REGISTRY_PATH}"
  ZRBCR_GAR_API_BASE="https://artifactregistry.googleapis.com/v1"
  ZRBCR_GAR_PACKAGE_BASE="projects/${RBRR_GAR_PROJECT_ID}/locations/${RBRR_GAR_LOCATION}/repositories/${RBRR_GAR_REPOSITORY}"

  # Media types
  ZRBCR_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBCR_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBCR_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBCR_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBCR_ACCEPT_MANIFEST_MTYPES="${ZRBCR_MTYPE_DV2},${ZRBCR_MTYPE_DLIST},${ZRBCR_MTYPE_OCI},${ZRBCR_MTYPE_OCM}"

  # File prefixes for all operations
  ZRBCR_MANIFEST_PREFIX="${BDU_TEMP_DIR}/rbcr_manifest_"
  ZRBCR_CONFIG_PREFIX="${BDU_TEMP_DIR}/rbcr_config_"
  ZRBCR_DELETE_PREFIX="${BDU_TEMP_DIR}/rbcr_delete_"
  ZRBCR_DETAIL_PREFIX="${BDU_TEMP_DIR}/rbcr_detail_"
  ZRBCR_TOKEN_PREFIX="${BDU_TEMP_DIR}/rbcr_token_"
  ZRBCR_TAGS_PREFIX="${BDU_TEMP_DIR}/rbcr_tags_"
  ZRBCR_EXISTS_PREFIX="${BDU_TEMP_DIR}/rbcr_exists_"

  # Output files
  ZRBCR_IMAGE_RECORDS_FILE="${BDU_TEMP_DIR}/rbcr_IMAGE_RECORDS.json"
  ZRBCR_IMAGE_DETAIL_FILE="${BDU_TEMP_DIR}/rbcr_IMAGE_DETAILS.json"
  ZRBCR_IMAGE_STATS_FILE="${BDU_TEMP_DIR}/rbcr_IMAGE_STATS.json"
  ZRBCR_FQIN_FILE="${BDU_TEMP_DIR}/rbcr_FQIN.txt"
  ZRBCR_TOKEN_FILE="${ZRBCR_TOKEN_PREFIX}access.txt"
  ZRBCR_METADATA_ARCHIVE="${BDU_TEMP_DIR}/rbcr_metadata.tgz"

  # File index counter
  ZRBCR_FILE_INDEX=0

  # Initialize detail file
  echo "[]" > "${ZRBCR_IMAGE_DETAIL_FILE}" || bcu_die "Failed to initialize detail file"

  # Obtain initial OAuth token before sentinel is active
  zrbcr_refresh_token_bootstrap || bcu_die "Cannot proceed without OAuth token"

  bcu_step "Log in to container registry"
  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}")
  test -n "${z_token}" || bcu_die "Token file is empty"

  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} login "${ZRBCR_REGISTRY_HOST}" \
                                            -u oauth2accesstoken     \
                                            -p "${z_token}"          \
    || bcu_die "Failed to login to registry"

  ZRBCR_KINDLED=1
}

zrbcr_sentinel() {
  test "${ZRBCR_KINDLED:-}" = "1" || bcu_die "Module rbcr not kindled - call zrbcr_kindle first"
}

zrbcr_refresh_token_bootstrap() {
  # No sentinel: used only from kindle before KINDLED=1
  bcu_log_args "Obtaining OAuth token for GAR API (bootstrap)"
  local z_token=""
  z_token=$(rbgo_get_token_capture "${RBRR_GAR_RBRA_FILE}") || bcu_die "Failed to get OAuth token from RBGO"
  echo "${z_token}" > "${ZRBCR_TOKEN_FILE}" || bcu_die "Failed to write token file"
}

# Download the GAR metadata archive for a given tag
rbcr_download_metadata() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  bcu_doc_brief "Download GAR build metadata archive for a tag"
  bcu_doc_param "tag" "Image tag to fetch metadata for"
  bcu_doc_shown || return 0

  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Refresh token (post-kindle) to avoid expiry surprises
  zrbcr_refresh_token

  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}") || bcu_die "Token read failed"
  test -n "${z_token}" || bcu_die "Empty token"

  local z_package_path="${ZRBCR_GAR_API_BASE}/${ZRBCR_GAR_PACKAGE_BASE}/packages/${z_tag}"

  bcu_step "Downloading GAR metadata for: ${z_tag}"
  curl -s \
       -H "Authorization: Bearer ${z_token}" \
       "${z_package_path}/versions/metadata:download" \
       -o "${ZRBCR_METADATA_ARCHIVE}" \
    || bcu_die "Failed to download metadata"

  test -f "${ZRBCR_METADATA_ARCHIVE}" || bcu_die "Metadata archive not created"
  test -s "${ZRBCR_METADATA_ARCHIVE}" || bcu_die "Metadata archive is empty"

  local z_extract_dir="${BDU_TEMP_DIR}/rbcr_metadata_${z_tag}"
  rm -rf "${z_extract_dir}" || bcu_warn "Failed to clean previous extract dir"
  mkdir -p "${z_extract_dir}" || bcu_die "Failed to create extract directory"
  tar -xzf "${ZRBCR_METADATA_ARCHIVE}" -C "${z_extract_dir}" || bcu_die "Failed to extract metadata"

  if test -f "${z_extract_dir}/package_summary.txt"; then
    bcu_info "Top packages in image:"
    head -5 "${z_extract_dir}/package_summary.txt" || bcu_warn "Failed to show package summary"
  fi

  bcu_success "Metadata retrieved to ${z_extract_dir}"
}



zrbcr_refresh_token() {
  zrbcr_sentinel

  bcu_step "Obtaining OAuth token for GAR API"

  # Use RBGO to get the token
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_GAR_RBRA_FILE}") || bcu_die "Failed to get OAuth token from RBGO"

  # Store token in RBCR's expected location
  echo "${z_token}" > "${ZRBCR_TOKEN_FILE}" || bcu_die "Failed to write token file"

  bcu_success "OAuth token obtained"
}

zrbcr_get_next_index_capture() {
  zrbcr_sentinel

  ZRBCR_FILE_INDEX=$((ZRBCR_FILE_INDEX + 1))
  printf "%03d" "${ZRBCR_FILE_INDEX}"
}

zrbcr_curl_registry() {
  zrbcr_sentinel

  local z_url="$1"
  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}")
  test -n "${z_token}" || bcu_die "Token is empty"

  curl -sL                                          \
      -H "Authorization: Bearer ${z_token}"         \
      -H "Accept: ${ZRBCR_ACCEPT_MANIFEST_MTYPES}"  \
      "${z_url}"                                    \
    || bcu_die "Registry API call failed: ${z_url}"
}

zrbcr_process_single_manifest() {
  zrbcr_sentinel

  local z_tag="$1"
  local z_manifest_file="$2"
  local z_platform="${3:-}"  # Empty for single-platform

  # Get config digest
  local z_config_digest_file="${ZRBCR_CONFIG_PREFIX}digest.txt"
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
  z_idx=$(zrbcr_get_next_index_capture) || bcu_die "Failed to get next index"
  local z_config_out="${ZRBCR_CONFIG_PREFIX}${z_idx}.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/blobs/${z_config_digest}" > "${z_config_out}" \
    || bcu_die "Failed to fetch config blob"

  # Validating config JSON
  jq . "${z_config_out}" > /dev/null || bcu_die "Invalid config JSON"

  # Build detail entry
  local z_detail_idx
  z_detail_idx=$(zrbcr_get_next_index_capture) || bcu_die "Failed to get detail index"
  local z_temp_detail="${ZRBCR_DETAIL_PREFIX}${z_detail_idx}.json"

  local z_manifest_json=$(<"${z_manifest_file}")
  test -n "${z_manifest_json}" || bcu_die "Manifest JSON is empty"

  # Normalize config with defaults
  local z_config_normalized="${ZRBCR_CONFIG_PREFIX}normalized_${z_idx}.json"
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
  bcu_info "Merging image detail"
  local    z_detail_tmp="${ZRBCR_IMAGE_DETAIL_FILE}.tmp"
  jq -s '.[0] + [.[1]]' "${ZRBCR_IMAGE_DETAIL_FILE}" "${z_temp_detail}" \
    > "${z_detail_tmp}" || bcu_die "Failed to merge image detail"
  mv  "${z_detail_tmp}" "${ZRBCR_IMAGE_DETAIL_FILE}" || bcu_die "Failed to move detail file"
}

zrbcr_exists_predicate() {
  zrbcr_sentinel

  local z_tag="$1"

  # Check if tag exists
  local z_tags_response="${ZRBCR_EXISTS_PREFIX}check.json"

  # Don't die on curl failure for predicate
  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}")
  test -n "${z_token}" || return 1

  curl -sL                                         \
      -H "Authorization: Bearer ${z_token}"        \
      -H "Accept: ${ZRBCR_ACCEPT_MANIFEST_MTYPES}" \
      "${ZRBCR_REGISTRY_API_BASE}/tags/list"       \
      > "${z_tags_response}" 2>/dev/null           \
    || return 1

  jq -e '.tags[] | select(. == "'"${z_tag}"'")' "${z_tags_response}" > /dev/null
}

######################################################################
# External Functions (rbcr_*)

rbcr_make_fqin() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Create fully qualified image name"
  bcu_doc_param "tag" "Image tag to qualify"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Write FQIN to file
  echo "${ZRBCR_REGISTRY_HOST}/${ZRBCR_REGISTRY_PATH}/${z_tag}" > "${ZRBCR_FQIN_FILE}" \
    || bcu_die "Failed to write FQIN"
}

rbcr_list_tags() {
  zrbcr_sentinel

  # Documentation block
  bcu_doc_brief "List all available image tags from registry"
  bcu_doc_shown || return 0

  bcu_step "Fetching image tags from GAR"

  # GAR uses Docker Registry v2 API - list tags endpoint
  local z_tags_response="${ZRBCR_TAGS_PREFIX}response.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/tags/list" > "${z_tags_response}" \
    || bcu_die "Failed to fetch tags list"

  # Transform to records format matching GHCR output
  jq -r --arg prefix "${ZRBCR_REGISTRY_HOST}/${ZRBCR_REGISTRY_PATH}" \
      '[.tags[] | {tag: ., fqin: ($prefix + "/" + .)}]'              \
      "${z_tags_response}" > "${ZRBCR_IMAGE_RECORDS_FILE}"           \
    || bcu_die "Failed to transform tags"

  local z_total_file="${ZRBCR_TAGS_PREFIX}total.txt"
  jq '. | length' "${ZRBCR_IMAGE_RECORDS_FILE}" > "${z_total_file}" || bcu_die "Failed to count tags"

  local z_total
  z_total=$(<"${z_total_file}")
  test -n "${z_total}" || bcu_die "Total count is empty"

  bcu_info "Retrieved ${z_total} total image records"
}

rbcr_get_manifest() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Fetch and process manifest for an image tag"
  bcu_doc_param "tag" "Image tag to fetch manifest for"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  local z_idx
  z_idx=$(zrbcr_get_next_index_capture) || bcu_die "Failed to get index"
  local z_manifest_out="${ZRBCR_MANIFEST_PREFIX}${z_idx}.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_tag}" > "${z_manifest_out}" \
    || bcu_die "Failed to fetch manifest for ${z_tag}"

  jq . "${z_manifest_out}" >/dev/null || bcu_die "Invalid manifest JSON"

  local z_media_type_file="${ZRBCR_MANIFEST_PREFIX}mediatype_${z_idx}.txt"
  jq -r '.mediaType // .schemaVersion' "${z_manifest_out}" > "${z_media_type_file}" \
    || bcu_die "Failed to extract media type"

  local z_media_type=$(<"${z_media_type_file}")
  test -n "${z_media_type}" || bcu_die "Failed to read or empty: ${z_media_type_file}"

  if test "${z_media_type}" = "${ZRBCR_MTYPE_DLIST}" || \
     test "${z_media_type}" = "${ZRBCR_MTYPE_OCI}"; then

    bcu_info "Multi-platform image detected"

    local z_manifests_file="${ZRBCR_MANIFEST_PREFIX}list_${z_idx}.jsonl"
    jq -c '.manifests[]' "${z_manifest_out}" > "${z_manifests_file}" \
      || bcu_die "Failed to extract manifests"

    local z_platform_idx=0
    while IFS= read -r z_platform_manifest; do
      local z_platform_digest_file="${ZRBCR_MANIFEST_PREFIX}digest_${z_platform_idx}.txt"
      jq -r '.digest' <<<"${z_platform_manifest}" > "${z_platform_digest_file}" \
        || bcu_die "Failed to extract platform digest"

      local z_platform_digest=$(<"${z_platform_digest_file}")
      test -n "${z_platform_digest}" || bcu_die "Failed to read or empty: ${z_platform_digest_file}"

      local z_platform_info_file="${ZRBCR_MANIFEST_PREFIX}info_${z_platform_idx}.txt"
      jq -r '"\(.platform.os)/\(.platform.architecture)"' <<<"${z_platform_manifest}" > "${z_platform_info_file}" \
        || bcu_die "Failed to extract platform info"

      local z_platform_info=$(<"${z_platform_info_file}")
      test -n "${z_platform_info}" || bcu_die "Failed to read or empty: ${z_platform_info_file}"

      bcu_info "Processing platform: ${z_platform_info}"

      local z_platform_idx_str
      z_platform_idx_str=$(zrbcr_get_next_index_capture) || bcu_die "Failed to get platform index"
      local z_platform_out="${ZRBCR_MANIFEST_PREFIX}${z_platform_idx_str}.json"

      zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_platform_digest}" \
        > "${z_platform_out}" || bcu_die "Failed to fetch platform manifest"

      jq . "${z_platform_out}" > /dev/null || bcu_die "Invalid platform manifest JSON"

      zrbcr_process_single_manifest "${z_tag}" "${z_platform_out}" "${z_platform_info}"

      z_platform_idx=$((z_platform_idx + 1))
    done < "${z_manifests_file}"

  else
    bcu_info "Single platform image"
    zrbcr_process_single_manifest "${z_tag}" "${z_manifest_out}" ""
  fi
}

rbcr_get_config() {
  zrbcr_sentinel

  local z_digest="${1:-}"

  # Documentation block
  bcu_doc_brief "Fetch configuration blob for a digest"
  bcu_doc_param "digest" "Digest of config blob to fetch"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_digest}" || bcu_die "Digest parameter required"

  # Fetch config blob
  local z_idx
  z_idx=$(zrbcr_get_next_index_capture) || bcu_die "Failed to get index"
  local z_config_out="${ZRBCR_CONFIG_PREFIX}${z_idx}.json"

  zrbcr_curl_registry "${ZRBCR_REGISTRY_API_BASE}/blobs/${z_digest}" \
    > "${z_config_out}" || bcu_die "Failed to fetch config blob"

  jq . "${z_config_out}" > /dev/null || bcu_die "Invalid config JSON"

  bcu_success "Config blob fetched to ${z_config_out}"
}

rbcr_delete() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Delete an image tag from the registry"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  bcu_step "Fetching manifest digest for deletion"

  # Get manifest with digest header
  local z_manifest_headers="${ZRBCR_DELETE_PREFIX}headers.txt"
  local z_token
  z_token=$(<"${ZRBCR_TOKEN_FILE}")
  test -n "${z_token}" || bcu_die "Token is empty"

  curl -sL -I \
    -H "Authorization: Bearer ${z_token}" \
    -H "Accept: ${ZRBCR_ACCEPT_MANIFEST_MTYPES}" \
    "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_tag}" \
    > "${z_manifest_headers}" || bcu_die "Failed to fetch manifest headers"

  # Extract digest from Docker-Content-Digest header
  local z_digest_file="${ZRBCR_DELETE_PREFIX}digest.txt"
  grep -i "docker-content-digest:" "${z_manifest_headers}" | \
    sed 's/.*: //' | tr -d '\r\n' > "${z_digest_file}" || bcu_die "Failed to extract digest header"

  local z_digest
  z_digest=$(<"${z_digest_file}")
  test -n "${z_digest}" || bcu_die "Manifest digest is empty"

  bcu_info "Deleting manifest: ${z_digest}"

  # Delete by digest
  local z_status_file="${ZRBCR_DELETE_PREFIX}status.txt"

  curl -X DELETE -s \
    -H "Authorization: Bearer ${z_token}" \
    -w "%{http_code}" \
    -o /dev/null \
    "${ZRBCR_REGISTRY_API_BASE}/manifests/${z_digest}" \
    > "${z_status_file}" || bcu_die "DELETE request failed"

  local z_http_code
  z_http_code=$(<"${z_status_file}")
  test -n "${z_http_code}" || bcu_die "HTTP status code is empty"
  test "${z_http_code}" = "202" || test "${z_http_code}" = "204" || \
    bcu_die "Delete failed with HTTP ${z_http_code}"
}

rbcr_pull() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Pull an image from the registry"
  bcu_doc_param "tag" "Image tag to pull"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  # Construct FQIN from tag
  rbcr_make_fqin "${z_tag}"
  local z_fqin
  z_fqin=$(<"${ZRBCR_FQIN_FILE}")
  test -n "${z_fqin}" || bcu_die "FQIN is empty"

  bcu_step "Pull image: ${z_tag}"
  ${RBG_RUNTIME} ${RBG_RUNTIME_ARG:-} pull "${z_fqin}" || bcu_die "Failed to pull image"
}

rbcr_exists() {
  zrbcr_sentinel

  local z_tag="${1:-}"

  # Documentation block
  bcu_doc_brief "Check if an image tag exists in the registry"
  bcu_doc_param "tag" "Image tag to check"
  bcu_doc_shown || return 0

  # Validate parameters
  test -n "${z_tag}" || bcu_die "Tag parameter required"

  if zrbcr_exists_predicate "${z_tag}"; then
    bcu_success "Tag exists: ${z_tag}"
  else
    bcu_warn "Tag does not exist: ${z_tag}"
    return 1
  fi
}

# eof

