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
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle GitHub - Image Registry Management

set -e

ZRBG_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBG_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBG_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

######################################################################
# Module Variables (ZRBG_*)
# These variables are used across multiple functions within this module
# Naming convention: ZRBG_<PURPOSE>
ZRBG_GIT_REGISTRY="ghcr.io"
ZRBG_GITAPI_URL="https://api.github.com"
ZRBG_REPO_PREFIX="${ZRBG_GITAPI_URL}/repos"
ZRBG_COLLECT_FULL_JSON="${RBG_TEMP_DIR}/RBG_COMBINED__${RBG_NOW_STAMP}.json"
ZRBG_COLLECT_TEMP_PAGE="${RBG_TEMP_DIR}/RBG_PAGE__${RBG_NOW_STAMP}.json"
ZRBG_CURRENT_WORKFLOW_RUN_CACHE="${RBG_TEMP_DIR}/CURR_WORKFLOW_RUN__${RBG_NOW_STAMP}.txt"
ZRBG_DELETE_VERSION_ID_CACHE="${RBG_TEMP_DIR}/RBG_VERSION_ID__${RBG_NOW_STAMP}.txt"
ZRBG_DELETE_RESULT_CACHE="${RBG_TEMP_DIR}/RBG_DELETE__${RBG_NOW_STAMP}.txt"


######################################################################
# Internal Functions (zrbg_*)
# These are helper functions used internally by the module
# Naming convention: zrbg_<action>_<object>

# Document, establish, validate environment
zrbg_validate_envvars() {
  # Handle documentation mode
  bcu_doc_env "RBG_TEMP_DIR  " "Empty temporary directory"
  bcu_doc_env "RBG_NOW_STAMP " "Timestamp for per run branding"
  bcu_doc_env "RBG_RBRR_FILE " "File containing the RBRR constants"

  bcu_env_done || return 0

  # Validate environment
  bvu_dir_exists  "${RBG_TEMP_DIR}"
  bvu_dir_empty   "${RBG_TEMP_DIR}"
  bvu_env_string     RBG_NOW_STAMP   1 128   # weak validation but infrastructure managed
  bvu_file_exists "${RBG_RBRR_FILE}"
  source          "${RBG_RBRR_FILE}"
  source "${ZRBG_SCRIPT_DIR}/rbrr.validator.sh"
}

# Validate GitHub PAT environment
zrbg_validate_pat() {
  test -f "${RBRR_GITHUB_PAT_ENV}" || bcu_die "GitHub PAT env file not found at ${RBRR_GITHUB_PAT_ENV}"

  # Load and check PAT exists
  source "${RBRR_GITHUB_PAT_ENV}"
  test -n "${RBRG_PAT:-}" || bcu_die "RBRG_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
}

# Perform authenticated GET request
# Usage: zrbg_curl_get <url>
zrbg_curl_get() {
  local url="$1"

  source "${RBRR_GITHUB_PAT_ENV}"
  curl -s -H "Authorization: token ${RBRG_PAT}"       \
          -H 'Accept: application/vnd.github.v3+json' \
          "$url"
}

# Perform authenticated POST request
# Usage: zrbg_curl_post <url> <data>
zrbg_curl_post() {
  local url="$1"
  local data="$2"

  source "${RBRR_GITHUB_PAT_ENV}"
  curl                                             \
       -s                                          \
       -X POST                                     \
       -H "Authorization: token ${RBRG_PAT}"       \
       -H 'Accept: application/vnd.github.v3+json' \
       "$url"                                      \
       -d "$data"                                  \
    || bcu_die "Curl failed."
}

# Perform authenticated DELETE request
# Usage: zrbg_curl_delete <url>
zrbg_curl_delete() {
  local url="$1"

  source "${RBRR_GITHUB_PAT_ENV}"
  curl -X DELETE -s -H "Authorization: token ${RBRG_PAT}"       \
                    -H 'Accept: application/vnd.github.v3+json' \
                    "$url"                                      \
                    -w "\nHTTP_STATUS:%{http_code}\n"
}

# Collect all package versions with pagination
# Outputs: Combined JSON file at ZRBG_COLLECT_FULL_JSON
zrbg_collect_all_versions() {
  bcu_step "Fetching all registry images with pagination to ${ZRBG_COLLECT_FULL_JSON}"

  # Initialize empty array
  echo "[]" > "${ZRBG_COLLECT_FULL_JSON}"

  bcu_info "Retrieving paged results..."

  local page=1
  while true; do
    bcu_info "  Fetching page ${page}..."

    # Get page of results
    local url="${ZRBG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100&page=${page}"
    zrbg_curl_get "$url" > "${ZRBG_COLLECT_TEMP_PAGE}"

    # Count items
    local items=$(jq '. | length' "${ZRBG_COLLECT_TEMP_PAGE}")
    bcu_info "  Saw ${items} items on page ${page}..."

    # Break if no items
    test "${items}" -ne 0 || break

    # Append to combined JSON
    bcu_info "  Appending page ${page} to combined JSON..."
    jq -s '.[0] + .[1]' "${ZRBG_COLLECT_FULL_JSON}" "${ZRBG_COLLECT_TEMP_PAGE}" > \
        "${ZRBG_COLLECT_FULL_JSON}.tmp"
    mv "${ZRBG_COLLECT_FULL_JSON}.tmp" "${ZRBG_COLLECT_FULL_JSON}"

    page=$((page + 1))
  done

  local total=$(jq '. | length' "${ZRBG_COLLECT_FULL_JSON}")
  bcu_info "  Retrieved ${total} total items"
  bcu_success "Pagination complete."
}

# Check git repository status
zrbg_check_git_status() {
  bcu_info "Make sure your local repo is up to date with github variant..."

  git fetch

  # Check if branch is up to date
  git status -uno | grep -q 'Your branch is up to date' || \
      bcu_die "ERROR: Your repo is not cleanly aligned with github variant.\n       Commit or otherwise match to proceed (prevents merge\n       conflicts with image history tracking)."

  # Check for uncommitted changes
  git diff-index --quiet HEAD -- || \
      bcu_die "ERROR: Your repo is not cleanly aligned with github variant.\n       Commit or otherwise match to proceed (prevents merge\n       conflicts with image history tracking)."
}

# Find the latest build directory for a recipe
# Usage: zrbg_get_latest_build_dir <recipe_basename>
zrbg_get_latest_build_dir() {
  local recipe_basename="$1"
  local basename_no_ext="${recipe_basename%.*}"

  find "${RBRR_HISTORY_DIR}" -name "${basename_no_ext}*" -type d -print | sort -r | head -n1
}

# Prompt for confirmation
# Usage: zrbg_confirm_action <prompt_message>
# Returns 0 if confirmed, 1 if not
zrbg_confirm_action() {
  local prompt="$1"

  echo -e "${ZBCU_YELLOW}${prompt}${ZBCU_RESET}"
  read -p "Type YES: " confirm
  test "$confirm" = "YES"
}

# Login to container registry
zrbg_registry_login() {
  bcu_step "Log in to container registry"

  source "${RBRR_GITHUB_PAT_ENV}"
  podman login "${ZRBG_GIT_REGISTRY}" -u "${RBRG_USERNAME}" -p "${RBRG_PAT}"
}

######################################################################
# External Functions (rbg_*)
# These are functions used from outside this module
# Naming convention: rbg_<action>

rbg_build() {
  # Name parameters
  local recipe_file="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Build image from recipe"
  bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$recipe_file" || bcu_usage_die
  test -f "$recipe_file" || bcu_die "Recipe file not found: $recipe_file"

  # Perform command
  local recipe_basename=$(basename "$recipe_file")
  echo "$recipe_basename" | grep -q '[A-Z]' && \
      bcu_die "Basename of '$recipe_file' contains uppercase letters so cannot use in image name"

  zrbg_validate_pat

  bcu_step "Trigger image build from $recipe_file"

  zrbg_check_git_status

  bcu_step "Triggering GitHub Actions workflow for image build"
  local dispatch_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/dispatches"
  local dispatch_data='{"event_type": "build_images", "client_payload": {"dockerfile": "'$recipe_file'"}}'
  zrbg_curl_post "$dispatch_url" "$dispatch_data"

  bcu_info "Polling for completion..."
  sleep 5

  bcu_info "Retrieve workflow run ID..."
  local runs_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs?event=repository_dispatch&branch=main&per_page=1"
  zrbg_curl_get "$runs_url" | jq -r '.workflow_runs[0].id' > "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"
  test -s "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}" || bcu_die "Failed to get workflow run ID"

  local run_id=$(cat "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}")
  bcu_info "Workflow online at:"
  echo -e "${ZBCU_YELLOW}   https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}${ZBCU_RESET}"

  bcu_info "Polling to completion..."
  local status=""
  local conclusion=""
  while true; do
    local run_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}"
    local response=$(zrbg_curl_get "$run_url")

    status=$(echo "$response" | jq -r '.status')
    conclusion=$(echo "$response" | jq -r '.conclusion')

    echo "  Status: $status    Conclusion: $conclusion"

    test "$status" != "completed" || break
    sleep 3
  done

  test "$conclusion" = "success" || bcu_die "Workflow fail: $conclusion"

  bcu_info "Git Pull for artifacts with retry..."
  local i
  for i in 9 8 7 6 5 4 3 2 1 0; do
    echo "  Attempt $i: Checking for remote changes..."
    git fetch --quiet

    if [ $(git rev-list --count HEAD..origin/main 2>/dev/null) -gt 0 ]; then
      echo "  Found new commits, pulling..."
      git pull
      echo "  Pull successful, breaking loop"
      break
    fi

    echo "  No new commits yet, waiting 5 seconds (attempt $i)"
    [ $i -eq 0 ] && bcu_die "No new commits after many attempts"
    sleep 5
  done

  bcu_info "Verifying build output..."
  local build_dir=$(zrbg_get_latest_build_dir "$recipe_basename")
  test -n "$build_dir" || bcu_die "Missing build directory"
  test -d "$build_dir" || bcu_die "Invalid build directory"
  test -f "$build_dir/recipe.txt" || bcu_die "recipe.txt not found"
  cmp "$recipe_file" "$build_dir/recipe.txt" || bcu_die "recipe mismatch"

  bcu_info "Extracting FQIN..."
  local fqin_file="$build_dir/docker_inspect_RepoTags_0.txt"
  test -f "$fqin_file" || bcu_die "Could not find FQIN in build output"

  local fqin_contents=$(cat "$fqin_file")
  bcu_info "Built image FQIN: $fqin_contents"

  if [ -n "${RBG_ARG_FQIN_OUTPUT:-}" ]; then
    cp "$fqin_file" "${RBG_ARG_FQIN_OUTPUT}"
    bcu_info "Wrote FQIN to ${RBG_ARG_FQIN_OUTPUT}"
  fi

  bcu_info "Verifying image availability in registry..."
  local tag=$(echo "$fqin_contents" | cut -d: -f2)
  echo "Waiting for tag: $tag to become available..."
  for i in 1 2 3 4 5; do
    zrbg_curl_get "${ZRBG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100" | \
      jq -e '.[] | select(.metadata.container.tags[] | contains("'"$tag"'"))' > /dev/null && break

    echo "  Image not yet available, attempt $i of 5"
    [ $i -eq 5 ] && bcu_die "Image '$tag' not available in registry after 5 attempts"
    sleep 5
  done

  bcu_info "Pull logs..."
  local logs_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}/logs"
  zrbg_curl_get "$logs_url" > "${RBG_TEMP_DIR}/workflow_logs__${RBG_NOW_STAMP}.txt"

  bcu_info "Everything went right, delete the run cache..."
  rm "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"

  bcu_success "No errors."
}

rbg_list() {
  # Handle documentation mode
  bcu_doc_brief "List registry images"
  bcu_doc_shown || return 0

  # Perform command
  zrbg_validate_pat
  zrbg_collect_all_versions

  bcu_step "List Current Registry Images"
  echo "Package: ${RBRR_REGISTRY_NAME}"
  echo -e "${ZBCU_YELLOW}    https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/pkgs/container/${RBRR_REGISTRY_NAME}${ZBCU_RESET}"
  echo "Versions:"

  # Format header
  printf "%-13s %-70s\n" "Version ID" "Fully Qualified Image Name"

  # Process and display versions
  jq -r '.[] | select(.metadata.container.tags | length > 0) | .id as $id | .metadata.container.tags[] as $tag | [$id, "'"${ZRBG_GIT_REGISTRY}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"':" + $tag] | @tsv' \
    "${ZRBG_COLLECT_FULL_JSON}" | sort -k2 -r | while IFS=$'\t' read -r id tag; do
    printf "%-13s %s\n" "$id" "$tag"
  done

  echo "${ZBCU_RESET}"

  # Count total versions
  local total=$(jq '[.[] | select(.metadata.container.tags | length > 0) | .metadata.container.tags | length] | add // 0' "${ZRBG_COLLECT_FULL_JSON}")
  bcu_info "Total image versions: ${total}"

  bcu_success "No errors."
}

rbg_delete() {
  # Name parameters
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Delete image from registry and clean orphans"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Perform command
  zrbg_validate_pat
  bcu_step "Delete image from GitHub Container Registry"
  zrbg_check_git_status

  # Confirm deletion unless skipped
  if [ "${RBG_ARG_SKIP_DELETE_CONFIRMATION:-}" != "SKIP" ]; then
      zrbg_confirm_action "Confirm delete image ${fqin}?" || bcu_die "WONT DELETE"
  fi

  bcu_step "Triggering GitHub Actions workflow for image deletion"
  local dispatch_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/dispatches"
  local dispatch_data='{"event_type": "delete_image", "client_payload": {"fqin": "'$fqin'"}}'
  zrbg_curl_post "$dispatch_url" "$dispatch_data"
  bcu_info "Delete dispatch submitted"
  sleep 5

  bcu_info "Retrieve workflow run ID..."
  local runs_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs?event=repository_dispatch&branch=main&per_page=1"
  zrbg_curl_get "$runs_url" | jq -r '.workflow_runs[0].id' > "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"
  test -s "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}" || bcu_die "Failed to get workflow run ID"

  local run_id=$(cat "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}")
  bcu_info "Delete workflow online at:"
  echo -e "${ZBCU_YELLOW}   https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}${ZBCU_RESET}"

  bcu_info "Polling to completion..."
  local status=""
  local conclusion=""
  while true; do
    local run_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}"
    local response=$(zrbg_curl_get "$run_url")

    status=$(echo "$response" | jq -r '.status')
    conclusion=$(echo "$response" | jq -r '.conclusion')

    echo "  Status: $status    Conclusion: $conclusion"

    test "$status" != "completed" || break
    sleep 3
  done

  test "$conclusion" = "success" || bcu_die "Workflow fail: $conclusion"

  bcu_info "Git Pull for deletion history..."
  local i
  for i in 9 8 7 6 5 4 3 2 1 0; do
    echo "  Attempt $i: Checking for remote changes..."
    git fetch --quiet

    if [ $(git rev-list --count HEAD..origin/main 2>/dev/null) -gt 0 ]; then
      echo "  Found new commits, pulling..."
      git pull
      echo "  Pull successful"
      break
    fi

    echo "  No new commits yet, waiting 3 seconds (attempt $i)"
    [ $i -eq 0 ] && echo "  Note: No deletion history recorded (might be expected for external images)"
    sleep 3
  done

  bcu_info "Pull logs..."
  local logs_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}/logs"
  zrbg_curl_get "$logs_url" > "${RBG_TEMP_DIR}/workflow_logs__${RBG_NOW_STAMP}.txt"

  bcu_info "Verifying deletion..."
  local tag=$(echo "$fqin" | sed 's/.*://')

  echo "  Checking that tag '$tag' is gone..."
  if zrbg_curl_get "${ZRBG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100" | \
      jq -e '.[] | select(.metadata.container.tags[] | contains("'"$tag"'"))' > /dev/null 2>&1; then
      bcu_die "Tag '$tag' still exists in registry after deletion"
  fi

  echo "  Confirmed: Tag '$tag' has been deleted"

  bcu_info "Cleanup..."
  rm "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"

  bcu_success "No errors."
}

rbg_retrieve() {
  # Name parameters
  local fqin="${1:-}"

  # Handle documentation mode
  bcu_doc_brief "Pull image from registry"
  bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
  bcu_doc_shown || return 0

  # Argument validation
  test -n "$fqin" || bcu_usage_die
  bvu_val_fqin "fqin" "$fqin" 1 512

  # Perform command
  zrbg_validate_pat
  bcu_step "Pull image from GitHub Container Registry"
  zrbg_registry_login
  bcu_info "Fetch image..."
  podman pull "$fqin"
  bcu_success "No errors."
}

zrbg_extract_manifest_info() {
  local tag="$1"
  local digest="$2"
  local platform="$3"
  local safe_tag="${tag//\//_}"
  local suffix="${platform//\//_}"
  local manifest_file="$RBG_TEMP_DIR/manifest__${safe_tag}_${suffix}.json"
  local config_file="$RBG_TEMP_DIR/config__${safe_tag}_${suffix}.json"
  local imageinfo_file="$RBG_TEMP_DIR/imageinfo__${safe_tag}_${suffix}.json"

  local repo="${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  local api="https://ghcr.io/v2/${repo}"
  local token_url="https://ghcr.io/token?scope=repository:${repo}:pull&service=ghcr.io"
  local bearer_token
  bearer_token=$(curl -sfL -u "${RBRG_USERNAME}:${RBRG_PAT}" "${token_url}" | jq -r '.token') || \
    bcu_die "Failed to obtain bearer token"
  local headers="Authorization: Bearer ${bearer_token}"

  curl -sfL -H "${headers}" \
    -H "Accept: application/vnd.docker.distribution.manifest.v2+json" \
    "$api/manifests/${digest}" -o "$manifest_file" || {
      bcu_warn "  Skipping $tag [$platform]: failed to fetch manifest"
      return
    }

  local config_digest
  config_digest=$(jq -r '.config.digest' "$manifest_file")
  if [ -z "$config_digest" ] || [ "$config_digest" = "null" ]; then
    bcu_warn "  Skipping $tag [$platform]: missing config digest"
    return
  fi

  curl -sfL -H "${headers}" "$api/blobs/${config_digest}" -o "$config_file" || {
    bcu_warn "  Skipping $tag [$platform]: failed to fetch config blob"
    return
  }

  local manifest_json config_json
  manifest_json=$(<"$manifest_file")
  config_json=$(<"$config_file")

  jq -n \
    --arg tag "$tag" \
    --arg digest "$config_digest" \
    --arg platform "$platform" \
    --argjson manifest "$manifest_json" \
    --argjson config "$config_json" '
    {
      tag: $tag,
      digest: $digest,
      platform: $platform,
      layers: $manifest.layers,
      config: {
        created: $config.created,
        architecture: $config.architecture,
        os: $config.os
      }
    }' > "$imageinfo_file"
}

# Gather image info from GHCR tags only (Bash 3.2 compliant)
rbg_image_info() {
  bcu_doc_brief "Extracts per-image and per-layer info from GHCR tags using GitHub API"
  bcu_doc_lines \
    "Creates manifest/config JSON files for each image tag, extracts creation date," \
    "layers, and layer sizes. Handles image indexes by recursively processing each platform manifest."
  bcu_doc_shown || return 0

  bcu_step "Assure PAT prepared"
  zrbg_validate_pat

  local combined_json="$RBG_TEMP_DIR/RBG_COMBINED__${RBG_NOW_STAMP}.json"
  local repo="${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}"
  local api="https://ghcr.io/v2/${repo}"
  local token_url="https://ghcr.io/token?scope=repository:${repo}:pull&service=ghcr.io"
  local accept_header="Accept: application/vnd.oci.image.index.v1+json, application/vnd.docker.distribution.manifest.list.v2+json, application/vnd.docker.distribution.manifest.v2+json"

  local bearer_token
  bearer_token=$(curl -sfL -u "${RBRG_USERNAME}:${RBRG_PAT}" "${token_url}" | jq -r '.token') || \
    bcu_die "Failed to obtain bearer token"
  local headers="Authorization: Bearer ${bearer_token}"

  bcu_step "Fetching all registry images with pagination to $combined_json"
  curl -sfL -H "${headers}" "https://ghcr.io/v2/${repo}/tags/list" -o "$combined_json" || \
    bcu_die "Failed to fetch tag list"

  bcu_info "Pagination complete."
  bcu_step "Collecting manifest + config JSON for each tag"

  local tag
  for tag in $(jq -r '.tags[]' "$combined_json"); do
    local safe_tag="${tag//\//_}"
    local manifest_file="$RBG_TEMP_DIR/manifest__${safe_tag}.json"

    bcu_info "  Tag: $tag"
    curl -sfL -H "${headers}" -H "${accept_header}" \
      "$api/manifests/${tag}" -o "$manifest_file" || {
        bcu_warn "  Skipping $tag: failed to fetch manifest"
        continue
      }

    local media_type
    media_type=$(jq -r '.mediaType' "$manifest_file")

    local digest arch os

    if [[ "$media_type" == "application/vnd.oci.image.index.v1+json" ||
          "$media_type" == "application/vnd.docker.distribution.manifest.list.v2+json" ]]; then
      jq -c '.manifests[]' "$manifest_file" | while read -r platform_manifest; do
        digest=$(jq -r '.digest'              <<<"$platform_manifest")
        arch=$(jq -r '.platform.architecture' <<<"$platform_manifest")
        os=$(jq -r '.platform.os'             <<<"$platform_manifest")
        zrbg_extract_manifest_info "$tag" "$digest" "$arch/$os"
      done
    else
      digest=$(jq -r '.config.digest' "$manifest_file")
      zrbg_extract_manifest_info "$tag" "$digest" ""
    fi
  done

  bcu_success "No errors."
}

bcu_execute rbg_ "Recipe Bottle GitHub - Image Registry Management" zrbg_validate_envvars "$@"

# eof

