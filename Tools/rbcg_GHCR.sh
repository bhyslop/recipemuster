#!/bin/bash
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
# Recipe Bottle Container GitHub - GHCR Registry Implementation

set -euo pipefail

ZRBCG_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBCG_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBCG_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Internal Functions (zrbcg_*)

zrbcg_environment() {
  # Handle documentation mode
  bcu_doc_env "RBG_TEMP_DIR  " "Empty temporary directory"
  bcu_doc_env "RBG_RBRR_FILE " "File containing the RBRR constants"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBG_TEMP_DIR}"
  bvu_dir_empty   "${RBG_TEMP_DIR}"
  bvu_file_exists "${RBG_RBRR_FILE}"

  # Source RBRR configuration (already done by caller)
  source "${ZRBCG_SCRIPT_DIR}/rbrr.validator.sh"

  # Source GitHub PAT credentials
  bvu_file_exists "${RBRR_GITHUB_PAT_ENV}"
  source          "${RBRR_GITHUB_PAT_ENV}"

  # Extract and validate PAT credentials
  test -n "${RBRG_PAT:-}"      || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_USERNAME:-}" || bcu_die "RBRG_USERNAME missing from ${RBRR_GITHUB_PAT_ENV}"

  # Module Variables (ZRBCG_*)
  ZRBCG_REGISTRY="ghcr.io"
  ZRBCG_GITAPI_URL="https://api.github.com"
  ZRBCG_PACKAGES_URL="${ZRBCG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions"
  ZRBCG_IMAGE_PREFIX="${ZRBCG_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBCG_GHCR_V2_API="https://ghcr.io/v2/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  ZRBCG_TOKEN_URL="https://ghcr.io/token?scope=repository:${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}:pull&service=ghcr.io"

  # Container runtime variables (set by rbcg_login)
  ZRBCG_RUNTIME=""
  ZRBCG_CONNECTION=""

  # Media types
  ZRBCG_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBCG_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBCG_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBCG_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBCG_ACCEPT_MANIFEST_MTYPES="${ZRBCG_MTYPE_DV2},${ZRBCG_MTYPE_DLIST},${ZRBCG_MTYPE_OCI},${ZRBCG_MTYPE_OCM}"
  ZRBCG_SCHEMA_V2="2"
  ZRBCG_MTYPE_GHV3="application/vnd.github.v3+json"
}

# Build container command with runtime and connection
zrbcg_build_container_cmd() {
  local base_cmd="$1"

  if [ "${ZRBCG_RUNTIME}" = "podman" ] && [ -n "${ZRBCG_CONNECTION}" ]; then
    echo "podman --connection=${ZRBCG_CONNECTION} ${base_cmd}"
  else
    echo "${ZRBCG_RUNTIME} ${base_cmd}"
  fi
}

# Perform authenticated GET request to GitHub API
zrbcg_curl_get() {
  local url="$1"
  curl -s -H "Authorization: token ${RBRG_PAT}" \
          -H "Accept: ${ZRBCG_MTYPE_GHV3}"      \
          "$url"
}

# Get bearer token for GHCR registry operations
zrbcg_get_bearer_token() {
  local token_out="${RBG_TEMP_DIR}/bearer_token.out"
  local token_err="${RBG_TEMP_DIR}/bearer_token.err"
  local bearer_token

  curl -sL -u "${RBRG_USERNAME}:${RBRG_PAT}" "${ZRBCG_TOKEN_URL}" >"${token_out}" 2>"${token_err}" && \
    bearer_token=$(jq -r '.token' "${token_out}") && \
    test -n "${bearer_token}" && \
    test "${bearer_token}" != "null" || {
      bcu_die "Failed to obtain bearer token"
    }

  echo "${bearer_token}"
}

# Process a single manifest (extracted from original code)
zrbcg_process_single_manifest() {
  local tag="$1"
  local manifest_file="$2"
  local platform="$3"
  local bearer_token="$4"
  local temp_detail="$5"

  local config_digest
  config_digest=$(jq -r '.config.digest' "${manifest_file}")

  test -n "${config_digest}" && test "${config_digest}" != "null" || {
    bcu_warn "null config.digest in manifest"
    return 1
  }

  local config_out="${manifest_file%.json}_config.json"

  # Use proper function call (bug fix)
  rbcg_fetch_config_blob "${config_digest}" "${bearer_token}" "${config_out}" || {
    bcu_warn "Failed to fetch config blob"
    return 1
  }

  local manifest_json config_json
  manifest_json="$(<"${manifest_file}")"
  config_json=$(jq '. + {
    created: (.created // "1970-01-01T00:00:00Z"),
    architecture: (.architecture // "unknown"),
    os: (.os // "unknown")
  }' "${config_out}")

  if [ -n "${platform}" ]; then
    jq -n \
      --arg tag          "${tag}"           \
      --arg platform     "${platform}"      \
      --arg digest       "${config_digest}" \
      --argjson manifest "${manifest_json}" \
      --argjson config   "${config_json}" '
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
      }' > "${temp_detail}"
  else
    jq -n \
      --arg     tag      "${tag}"           \
      --arg     digest   "${config_digest}" \
      --argjson manifest "${manifest_json}" \
      --argjson config   "${config_json}" '
      {
        tag: $tag,
        digest: $digest,
        layers: $manifest.layers,
        config: {
          created: $config.created,
          architecture: $config.architecture,
          os: $config.os
        }
      }' > "${temp_detail}"
  fi

  return 0
}

######################################################################
# External Functions (rbcg_*)

rbcg_login() {
  local runtime="${1:-}"
  local connection_string="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Login to GHCR"
  bcu_doc_param "runtime" "Container runtime to use (docker or podman)"
  bcu_doc_oparm "connection_string" "Connection string for podman remote connections"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$runtime" || bcu_usage_die

  # Store runtime settings
  ZRBCG_RUNTIME="${runtime}"
  ZRBCG_CONNECTION="${connection_string}"

  bcu_step "Login to GitHub Container Registry"
  local login_cmd=$(zrbcg_build_container_cmd "login ${ZRBCG_REGISTRY} -u ${RBRG_USERNAME} -p ${RBRG_PAT}")
  eval "${login_cmd}"
  bcu_success "Logged in to ${ZRBCG_REGISTRY}"
}

rbcg_push() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Push image to GHCR"
  bcu_doc_param "tag" "Image tag to push"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  local fqin="${ZRBCG_IMAGE_PREFIX}:${tag}"
  bcu_step "Push image ${fqin}"

  local push_cmd=$(zrbcg_build_container_cmd "push ${fqin}")
  eval "${push_cmd}"

  bcu_success "Image pushed successfully"
}

rbcg_pull() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from GHCR"
  bcu_doc_param "tag" "Image tag to pull"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  local fqin="${ZRBCG_IMAGE_PREFIX}:${tag}"
  bcu_step "Pull image ${fqin}"

  local pull_cmd=$(zrbcg_build_container_cmd "pull ${fqin}")
  eval "${pull_cmd}"

  bcu_success "Image pulled successfully"
}

rbcg_delete() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from GHCR"
  bcu_doc_param "tag" "Image tag to delete"
  bcu_doc_lines \
    "WARNING: GHCR has known issues with layer reference counting." \
    "Deleting images may orphan or incorrectly delete shared layers."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  bcu_warn "GHCR DELETE HAS KNOWN ISSUES WITH LAYER MANAGEMENT"
  bcu_warn "Shared layers may be incorrectly deleted or orphaned"

  bcu_step "Delete tag ${tag} from GHCR"

  # Find version ID for tag
  local version_id
  version_id=$(zrbcg_curl_get "${ZRBCG_PACKAGES_URL}?per_page=100" | \
    jq -r '.[] | select(.metadata.container.tags[] | contains("'"${tag}"'")) | .id' | head -n1)

  test -n "${version_id}" || bcu_die "Tag ${tag} not found"

  local delete_url="${ZRBCG_PACKAGES_URL}/${version_id}"
  local response
  response=$(curl -X DELETE -s -H "Authorization: token ${RBRG_PAT}" \
                               -H "Accept: ${ZRBCG_MTYPE_GHV3}"      \
                               "${delete_url}"                       \
                               -w "\nHTTP_STATUS:%{http_code}\n")

  echo "${response}" | grep -q "HTTP_STATUS:204" || bcu_die "Delete failed"

  bcu_success "Tag deleted (layer cleanup unreliable on GHCR)"
}

rbcg_exists() {
  local tag="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Check if tag exists in GHCR"
  bcu_doc_param "tag" "Image tag to check"
  bcu_doc_lines "Returns exit code 0 if exists, 1 if not"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die

  # Check using GitHub API
  zrbcg_curl_get "${ZRBCG_PACKAGES_URL}?per_page=100" | \
    jq -e '.[] | select(.metadata.container.tags[] | contains("'"${tag}"'"))' >/dev/null 2>&1
}

rbcg_tags() {
  local output_IMAGE_RECORDS_json="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "List all tags in GHCR repository"
  bcu_doc_param "output_IMAGE_RECORDS_json"  "Path to write tags JSON (IMAGE_RECORDS schema)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$output_IMAGE_RECORDS_json" || bcu_usage_die

  bcu_step "Fetching all tags from GHCR"

  # Initialize empty array
  echo "[]" > "${output_IMAGE_RECORDS_json}"

  local page=1
  local temp_page="${RBG_TEMP_DIR}/temp_page.json"
  local temp_tags="${RBG_TEMP_DIR}/temp_tags.json"

  while true; do
    local url="${ZRBCG_PACKAGES_URL}?per_page=100&page=${page}"
    zrbcg_curl_get "$url" > "${temp_page}"

    local items=$(jq '. | length' "${temp_page}")
    test "${items}" -ne 0 || break

    # Extract tags with version_id for compatibility
    jq -r '[.[] | select(.metadata.container.tags | length > 0) |
            .id as $version_id |
            .metadata.container.tags[] as $tag |
            {version_id: $version_id, tag: $tag, fqin: ("'${ZRBCG_IMAGE_PREFIX}':" + $tag)}]' \
      "${temp_page}" > "${temp_tags}"

    # Merge with existing
    jq -s '.[0] + .[1]' "${output_IMAGE_RECORDS_json}" "${temp_tags}" > "${output_IMAGE_RECORDS_json}.tmp"
    mv "${output_IMAGE_RECORDS_json}.tmp" "${output_IMAGE_RECORDS_json}"

    page=$((page + 1))
  done

  local total=$(jq '. | length' "${output_IMAGE_RECORDS_json}")
  bcu_success "Retrieved ${total} tags"
}

rbcg_fetch_config_blob() {
  local config_digest="${1:-}"
  local bearer_token="${2:-}"
  local output_config_ocijson="${3:-}"

  # Handle documentation mode
  bcu_doc_brief "Fetch OCI config blob from GHCR registry"
  bcu_doc_param "config_digest"          "SHA256 digest of config blob (e.g., sha256:abc123...)"
  bcu_doc_param "bearer_token"           "Bearer token for GHCR authentication"
  bcu_doc_param "output_config_ocijson"  "Path to write OCI config JSON"
  bcu_doc_lines \
    "Retrieves an OCI/Docker image configuration blob from GHCR's blob store." \
    "The output follows OCI Image Configuration Specification." \
    "Key fields: architecture, os, created, rootfs.type, rootfs.diff_ids[]," \
    "config.Env[], config.Cmd[], config.WorkingDir, config.User, history[]" \
    "This blob contains the image metadata and layer history."
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$config_digest" || bcu_usage_die
  test -n "$bearer_token" || bcu_usage_die
  test -n "$output_config_ocijson" || bcu_usage_die

  local headers="Authorization: Bearer ${bearer_token}"

  curl -sL -H "${headers}" \
       "${ZRBCG_GHCR_V2_API}/blobs/${config_digest}" \
       > "${output_config_ocijson}" 2>/dev/null

  # Validate JSON
  jq . "${output_config_ocijson}" >/dev/null 2>&1
}

rbcg_fetch_manifest() {
  local tag="${1:-}"
  local bearer_token="${2:-}"
  local output_manifest_ocijson="${3:-}"

  # Handle documentation mode
  bcu_doc_brief "Fetch OCI manifest from GHCR registry"
  bcu_doc_param "tag"                      "Image tag or manifest digest to fetch"
  bcu_doc_param "bearer_token"             "Bearer token for GHCR authentication"
  bcu_doc_param "output_manifest_ocijson"  "Path to write OCI manifest JSON"
  bcu_doc_lines \
    "Retrieves an OCI/Docker manifest from GHCR's v2 registry API." \
    "The output follows OCI Image Manifest Specification or Docker Manifest v2 Schema." \
    "For multi-platform images, returns a manifest list (mediaType: application/vnd.docker.distribution.manifest.list.v2+json)" \
    "For single-platform images, returns a manifest (mediaType: application/vnd.docker.distribution.manifest.v2+json)" \
    "Key fields: schemaVersion, mediaType, config.digest, layers[], manifests[] (for lists)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$bearer_token" || bcu_usage_die
  test -n "$output_manifest_ocijson" || bcu_usage_die

  local headers="Authorization: Bearer ${bearer_token}"

  curl -sL -H "${headers}" \
       -H "Accept: ${ZRBCG_ACCEPT_MANIFEST_MTYPES}" \
       "${ZRBCG_GHCR_V2_API}/manifests/${tag}" \
       > "${output_manifest_ocijson}" 2>/dev/null

  # Validate JSON
  jq . "${output_manifest_ocijson}" >/dev/null 2>&1
}

rbcg_inspect() {
  local tag="${1:-}"
  local output_MANIFEST_json="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Get raw manifest and config for a tag"
  bcu_doc_param "tag"                   "Image tag to inspect"
  bcu_doc_param "output_MANIFEST_json"  "Path to write manifest JSON"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$output_MANIFEST_json" || bcu_usage_die

  bcu_step "Fetching manifest for tag ${tag}"

  local bearer_token
  bearer_token=$(zrbcg_get_bearer_token)

  # Use new subfunction
  rbcg_fetch_manifest "${tag}" "${bearer_token}" "${output_MANIFEST_json}" || \
    bcu_die "Failed to fetch manifest"

  bcu_success "Manifest saved to ${output_MANIFEST_json}"
}

rbcg_layers() {
  local input_IMAGE_RECORDS_json="${1:-}"
  local output_IMAGE_DETAILS_json="${2:-}"
  local output_IMAGE_STATS_json="${3:-}"

  # Handle documentation mode
  bcu_doc_brief "Extract layer information from specified tags"
  bcu_doc_param "input_IMAGE_RECORDS_json"   "Path to JSON file containing tags to analyze"
  bcu_doc_param "output_IMAGE_DETAILS_json"  "Path to write layer details"
  bcu_doc_param "output_IMAGE_STATS_json"    "Path to write layer statistics"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$input_IMAGE_RECORDS_json"  || bcu_usage_die
  test -n "$output_IMAGE_DETAILS_json" || bcu_usage_die
  test -n "$output_IMAGE_STATS_json"   || bcu_usage_die

  bcu_step "Analyzing image layers"

  # Get bearer token
  local bearer_token
  bearer_token=$(zrbcg_get_bearer_token)

  # Initialize details file
  echo "[]" > "${output_IMAGE_DETAILS_json}"

  # Process each tag from input file
  local tag
  for tag in $(jq -r '.[].tag' "${input_IMAGE_RECORDS_json}" | sort -u); do

    bcu_info "Processing tag: ${tag}"

    local safe_tag="${tag//\//_}"
    local manifest_out="${RBG_TEMP_DIR}/manifest_${safe_tag}.json"
    local temp_detail="${RBG_TEMP_DIR}/temp_detail.json"

    # Use new subfunction to fetch manifest
    rbcg_fetch_manifest "${tag}" "${bearer_token}" "${manifest_out}" || continue

    # Check media type
    local media_type
    media_type=$(jq -r '.mediaType // .schemaVersion' "${manifest_out}")

    if [[ "${media_type}" == "${ZRBCG_MTYPE_DLIST}" ]] || \
       [[ "${media_type}" == "${ZRBCG_MTYPE_OCI}"   ]]; then
      # Multi-platform
      local manifests
      manifests=$(jq -c '.manifests[]' "${manifest_out}")

      while IFS= read -r platform_manifest; do
        local platform_digest platform_info
        platform_digest=$(echo "${platform_manifest}" | jq -r '.digest')
        platform_info=$(echo "${platform_manifest}" | jq -r '"\(.platform.os)/\(.platform.architecture)"')

        local platform_out="${RBG_TEMP_DIR}/platform_${safe_tag}.json"

        # Use subfunction to fetch platform manifest
        rbcg_fetch_manifest "${platform_digest}" "${bearer_token}" "${platform_out}" || continue

        if zrbcg_process_single_manifest "${tag}" "${platform_out}" "${platform_info}" "${bearer_token}" "${temp_detail}"; then
          jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}" "${temp_detail}" > "${output_IMAGE_DETAILS_json}.tmp"
          mv "${output_IMAGE_DETAILS_json}.tmp" "${output_IMAGE_DETAILS_json}"
        fi
      done <<< "$manifests"

    else
      # Single platform
      if zrbcg_process_single_manifest "${tag}" "${manifest_out}" "" "${bearer_token}" "${temp_detail}"; then
        jq -s '.[0] + [.[1]]' "${output_IMAGE_DETAILS_json}" "${temp_detail}" > "${output_IMAGE_DETAILS_json}.tmp"
        mv "${output_IMAGE_DETAILS_json}.tmp" "${output_IMAGE_DETAILS_json}"
      fi
    fi
  done

  # Generate stats
  jq '
    .[]
    | {tag} as $t
    | .layers[]
    | {digest, size, tag: $t.tag}
  ' "${output_IMAGE_DETAILS_json}" |
  jq -s '
    group_by([.digest, .tag])
    | map({
        digest: .[0].digest,
        size: .[0].size,
        tag: .[0].tag,
        count: length
      })
    | group_by(.digest)
    | map({
        digest: .[0].digest,
        size: .[0].size,
        tag_count: length,
        total_usage: (map(.count) | add),
        tag_details: map({tag: .tag, count: .count})
      })
    | sort_by(-.size)
  ' > "${output_IMAGE_STATS_json}"

  bcu_success "Layer analysis complete"
}

bcu_execute rbcg_ "Recipe Bottle Container GitHub - GHCR Implementation" zrbcg_environment "$@"

# eof

