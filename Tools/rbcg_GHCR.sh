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
  
  # Media types
  ZRBCG_MTYPE_DLIST="application/vnd.docker.distribution.manifest.list.v2+json"
  ZRBCG_MTYPE_OCI="application/vnd.oci.image.index.v1+json"
  ZRBCG_MTYPE_DV2="application/vnd.docker.distribution.manifest.v2+json"
  ZRBCG_MTYPE_OCM="application/vnd.oci.image.manifest.v1+json"
  ZRBCG_ACCEPT_MANIFEST_MTYPES="${ZRBCG_MTYPE_DV2},${ZRBCG_MTYPE_DLIST},${ZRBCG_MTYPE_OCI},${ZRBCG_MTYPE_OCM}"
  ZRBCG_SCHEMA_V2="2"
  ZRBCG_MTYPE_GHV3="application/vnd.github.v3+json"
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
  local headers="$4"
  local temp_detail="$5"

  local config_digest
  config_digest=$(jq -r '.config.digest' "${manifest_file}")

  test -n "${config_digest}" && test "${config_digest}" != "null" || {
    bcu_warn "null config.digest in manifest"
    return 1
  }

  local config_out="${manifest_file%.json}_config.json"
  curl -sL -H "${headers}" "${ZRBCG_GHCR_V2_API}/blobs/${config_digest}" >"${config_out}" 2>/dev/null && \
    jq . "${config_out}" >/dev/null || {
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
  # Handle documentation mode
  bcu_doc_brief "Login to GHCR"
  bcu_doc_shown || return 0

  bcu_step "Login to GitHub Container Registry"
  podman login "${ZRBCG_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"
  bcu_success "Logged in to ${ZRBCG_REGISTRY}"
}

rbcg_push() {
  local tag="${1:-}"
  local dockerfile_path="${2:-}"
  local platforms="${3:-}"

  # Handle documentation mode
  bcu_doc_brief "Push image to GHCR (called from within GitHub Action)"
  bcu_doc_param "tag" "Image tag"
  bcu_doc_param "dockerfile_path" "Path to Dockerfile (for metadata)"
  bcu_doc_oparm "platforms" "Target platforms (default: linux/amd64)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$dockerfile_path" || bcu_usage_die
  
  # This function is called from within GitHub Actions
  # The actual build/push is handled by docker/build-push-action
  # This is a placeholder for registry abstraction
  
  bcu_info "Push operation for ${tag} would be handled by GitHub Actions"
  bcu_info "Dockerfile: ${dockerfile_path}"
  bcu_info "Platforms: ${platforms:-default}"
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
  podman pull "${fqin}"
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
                               "${delete_url}"                        \
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
  local output_json="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "List all tags in GHCR repository"
  bcu_doc_param "output_json" "Path to write tags JSON (RBC_TAGS_SCHEMA)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$output_json" || bcu_usage_die

  bcu_step "Fetching all tags from GHCR"
  
  # Initialize empty array
  echo "[]" > "${output_json}"
  
  local page=1
  local temp_page="${RBG_TEMP_DIR}/temp_page.json"
  local temp_tags="${RBG_TEMP_DIR}/temp_tags.json"

  while true; do
    local url="${ZRBCG_PACKAGES_URL}?per_page=100&page=${page}"
    zrbcg_curl_get "$url" > "${temp_page}"
    
    local items=$(jq '. | length' "${temp_page}")
    test "${items}" -ne 0 || break
    
    # Extract tags without version_id
    jq -r '[.[] | select(.metadata.container.tags | length > 0) |
            .metadata.container.tags[] as $tag |
            {tag: $tag, fqin: ("'${ZRBCG_IMAGE_PREFIX}':" + $tag)}]' \
      "${temp_page}" > "${temp_tags}"
    
    # Merge with existing
    jq -s '.[0] + .[1]' "${output_json}" "${temp_tags}" > "${output_json}.tmp"
    mv "${output_json}.tmp" "${output_json}"
    
    page=$((page + 1))
  done
  
  local total=$(jq '. | length' "${output_json}")
  bcu_success "Retrieved ${total} tags"
}

rbcg_inspect() {
  local tag="${1:-}"
  local output_json="${2:-}"

  # Handle documentation mode
  bcu_doc_brief "Get raw manifest and config for a tag"
  bcu_doc_param "tag" "Image tag to inspect"
  bcu_doc_param "output_json" "Path to write manifest JSON (RBC_MANIFEST_SCHEMA)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$tag" || bcu_usage_die
  test -n "$output_json" || bcu_usage_die

  bcu_step "Fetching manifest for tag ${tag}"
  
  local bearer_token
  bearer_token=$(zrbcg_get_bearer_token)
  local headers="Authorization: Bearer ${bearer_token}"
  
  curl -sL -H "${headers}" \
       -H "Accept: ${ZRBCG_ACCEPT_MANIFEST_MTYPES}" \
       "${ZRBCG_GHCR_V2_API}/manifests/${tag}" \
       > "${output_json}"
  
  jq . "${output_json}" >/dev/null || bcu_die "Failed to fetch manifest"
  
  bcu_success "Manifest saved to ${output_json}"
}

rbcg_layers() {
  local details_json="${1:-}"
  local stats_json="${2:-}"
  local filter="${3:-}"

  # Handle documentation mode
  bcu_doc_brief "Extract layer information from all tags"
  bcu_doc_param "details_json" "Path to write layer details (RBC_DETAILS_SCHEMA)"
  bcu_doc_param "stats_json" "Path to write layer statistics (RBC_STATS_SCHEMA)"
  bcu_doc_oparm "filter" "Only process tags containing this string"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$details_json" || bcu_usage_die
  test -n "$stats_json" || bcu_usage_die

  bcu_step "Analyzing image layers"
  
  # Get all tags first
  local tags_file="${RBG_TEMP_DIR}/all_tags.json"
  rbcg_tags "${tags_file}"
  
  # Get bearer token
  local bearer_token
  bearer_token=$(zrbcg_get_bearer_token)
  local headers="Authorization: Bearer ${bearer_token}"
  
  # Initialize details file
  echo "[]" > "${details_json}"
  
  # Process each tag
  local tag
  for tag in $(jq -r '.[].tag' "${tags_file}" | sort -u); do
    
    if [ -n "${filter}" ] && [[ "${tag}" != *"${filter}"* ]]; then
      continue
    fi
    
    bcu_info "Processing tag: ${tag}"
    
    local safe_tag="${tag//\//_}"
    local manifest_out="${RBG_TEMP_DIR}/manifest_${safe_tag}.json"
    local temp_detail="${RBG_TEMP_DIR}/temp_detail.json"
    
    # Fetch manifest
    curl -sL -H "${headers}" \
         -H "Accept: ${ZRBCG_ACCEPT_MANIFEST_MTYPES}" \
         "${ZRBCG_GHCR_V2_API}/manifests/${tag}" \
         > "${manifest_out}" 2>/dev/null || continue
    
    # Check media type
    local media_type
    media_type=$(jq -r '.mediaType // .schemaVersion' "${manifest_out}")
    
    if [[ "${media_type}" == "${ZRBCG_MTYPE_DLIST}" ]] || \
       [[ "${media_type}" == "${ZRBCG_MTYPE_OCI}" ]]; then
      # Multi-platform
      local manifests
      manifests=$(jq -c '.manifests[]' "${manifest_out}")
      
      while IFS= read -r platform_manifest; do
        local platform_digest platform_info
        platform_digest=$(echo "${platform_manifest}" | jq -r '.digest')
        platform_info=$(echo "${platform_manifest}" | jq -r '"\(.platform.os)/\(.platform.architecture)"')
        
        local platform_out="${RBG_TEMP_DIR}/platform_${safe_tag}.json"
        
        curl -sL -H "${headers}" \
             -H "Accept: ${ZRBCG_ACCEPT_MANIFEST_MTYPES}" \
             "${ZRBCG_GHCR_V2_API}/manifests/${platform_digest}" \
             > "${platform_out}" 2>/dev/null || continue
        
        if zrbcg_process_single_manifest "${tag}" "${platform_out}" "${platform_info}" "${headers}" "${temp_detail}"; then
          jq -s '.[0] + [.[1]]' "${details_json}" "${temp_detail}" > "${details_json}.tmp"
          mv "${details_json}.tmp" "${details_json}"
        fi
      done <<< "$manifests"
      
    else
      # Single platform
      if zrbcg_process_single_manifest "${tag}" "${manifest_out}" "" "${headers}" "${temp_detail}"; then
        jq -s '.[0] + [.[1]]' "${details_json}" "${temp_detail}" > "${details_json}.tmp"
        mv "${details_json}.tmp" "${details_json}"
      fi
    fi
  done
  
  # Generate stats
  jq '
    .[]
    | {tag} as $t
    | .layers[]
    | {digest, size, tag: $t.tag}
  ' "${details_json}" |
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
  ' > "${stats_json}"
  
  bcu_success "Layer analysis complete"
}

bcu_execute rbcg_ "Recipe Bottle Container GitHub - GHCR Implementation" zrbcg_environment "$@"

