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
# Recipe Bottle GitHub - Container Registry Management

set -e

# Find script directory and source utilities
ZRBG_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBG_SCRIPT_DIR}/bcu_BashConsoleUtility.sh"
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
    test -n "${RBV_PAT:-}" || bcu_die "RBV_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
}

# Perform authenticated GET request
# Usage: zrbg_curl_get <url>
zrbg_curl_get() {
    local url="$1"

    source "${RBRR_GITHUB_PAT_ENV}"
    curl -s -H "Authorization: token ${RBV_PAT}" \
            -H 'Accept: application/vnd.github.v3+json' \
            "$url"
}

# Perform authenticated POST request
# Usage: zrbg_curl_post <url> <data>
zrbg_curl_post() {
    local url="$1"
    local data="$2"

    source "${RBRR_GITHUB_PAT_ENV}"
    curl -X POST -H "Authorization: token ${RBV_PAT}" \
                 -H 'Accept: application/vnd.github.v3+json' \
                 "$url" \
                 -d "$data"
}

# Perform authenticated DELETE request
# Usage: zrbg_curl_delete <url>
zrbg_curl_delete() {
    local url="$1"

    source "${RBRR_GITHUB_PAT_ENV}"
    curl -X DELETE -s -H "Authorization: token ${RBV_PAT}" \
                      -H 'Accept: application/vnd.github.v3+json' \
                      "$url" \
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
    podman login "${ZRBG_GIT_REGISTRY}" -u "${RBV_USERNAME}" -p "${RBV_PAT}"
}

######################################################################
# External Functions (rbg_*)
# These are functions used from outside this module
# Naming convention: rbg_<action>

rbg_build() {
    # Name parameters
    local recipe_file="${1:-}"

    # Handle documentation mode
    bcu_doc_brief "Build container from recipe"
    bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
    bcu_doc_shown || return 0

    # Argument validation
    test -n "$recipe_file" || bcu_usage_die
    test -f "$recipe_file" || bcu_die "Recipe file not found: $recipe_file"

    local recipe_basename=$(basename "$recipe_file")
    # Check for uppercase letters in basename
    echo "$recipe_basename" | grep -q '[A-Z]' && \
        bcu_die "Basename of '$recipe_file' contains uppercase letters"

    # Validate GitHub PAT
    zrbg_validate_pat

    # Command execution
    bcu_step "Trigger Build of $recipe_file"

    # Check git status
    zrbg_check_git_status

    # Trigger workflow dispatch
    local dispatch_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/dispatches"
    local dispatch_data='{"event_type": "build_containers", "client_payload": {"dockerfile": "'"$recipe_file"'"}}'
    zrbg_curl_post "$dispatch_url" "$dispatch_data"

    # Pause for GitHub to process
    bcu_info "Pausing for GitHub to process the dispatch event..."
    sleep 5

    # Get workflow run ID
    bcu_info "Retrieve workflow run ID..."
    local runs_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs?event=repository_dispatch&branch=main&per_page=1"
    zrbg_curl_get "$runs_url" | jq -r '.workflow_runs[0].id' > "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"
    test -s "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}" || bcu_die "Failed to get workflow run ID"

    local run_id=$(cat "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}")
    bcu_info "Workflow online at:"
    echo -e "${ZBCU_YELLOW}   https://github.com/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}${ZBCU_RESET}"

    # Poll for completion
    bcu_info "Polling to completion..."
    local status=""
    local conclusion=""
    while true; do
        local run_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}"
        local response=$(zrbg_curl_get "$run_url")

        echo "  TRACE: $response" | grep -i -e "status" -e "conclusion" -e "TRACE"

        status=$(echo "$response" | jq -r '.status')
        conclusion=$(echo "$response" | jq -r '.conclusion')

        echo "  Status: $status    Conclusion: $conclusion"

        test "$status" != "completed" || break
        sleep 3
    done

    test "$conclusion" = "success" || bcu_die "Workflow fail: $conclusion"

    # Git pull with retry
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

    # Verify build output
    bcu_info "Verifying build output..."

    # Find latest build directory
    local build_dir=$(zrbg_get_latest_build_dir "$recipe_basename")
    test -n "$build_dir" || bcu_die "Missing build directory - No directory found matching pattern '${RBRR_HISTORY_DIR}/${recipe_basename%.*}*'"
    test -d "$build_dir" || bcu_die "Build directory '$build_dir' is not a valid directory"

    # Compare recipes
    test -f "$build_dir/recipe.txt" || bcu_die "recipe.txt not found in $build_dir"
    cmp "$recipe_file" "$build_dir/recipe.txt" || bcu_die "recipe mismatch"

    # Extract FQIN
    bcu_info "Extracting FQIN..."
    local fqin_file="$build_dir/docker_inspect_RepoTags_0.txt"
    test -f "$fqin_file" || bcu_die "Could not find FQIN in build output"

    local fqin_contents=$(cat "$fqin_file")
    echo -e "${ZBCU_YELLOW}Built container FQIN: $fqin_contents${ZBCU_RESET}"

    # Output FQIN if requested
    if [ -n "${RBG_ARG_FQIN_OUTPUT:-}" ]; then
        cp "$fqin_file" "${RBG_ARG_FQIN_OUTPUT}"
        echo -e "${ZBCU_YELLOW}Wrote FQIN to ${RBG_ARG_FQIN_OUTPUT}${ZBCU_RESET}"
    fi

    # Verify image availability
    bcu_info "Verifying image availability in registry..."
    local tag=$(echo "$fqin_contents" | cut -d: -f2)
    echo "  Waiting for tag: $tag to become available..."

    for i in 1 2 3 4 5; do
        zrbg_curl_get "${ZRBG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions?per_page=100" | \
            jq -e '.[] | select(.metadata.container.tags[] | contains("'"$tag"'"))' > /dev/null && break

        echo "  Image not yet available, attempt $i of 5"
        [ $i -eq 5 ] && bcu_die "Image '$tag' not available in registry after 5 attempts"
        sleep 5
    done

    # Get logs
    bcu_info "Pull logs..."
    local logs_url="${ZRBG_REPO_PREFIX}/${RBRR_REGISTRY_OWNER}/${RBRR_REGISTRY_NAME}/actions/runs/${run_id}/logs"
    zrbg_curl_get "$logs_url" > "${RBG_TEMP_DIR}/workflow_logs__${RBG_NOW_STAMP}.txt"

    # Cleanup
    bcu_info "Everything went right, delete the run cache..."
    rm "${ZRBG_CURRENT_WORKFLOW_RUN_CACHE}"

    bcu_success "No errors."
}

rbg_list() {
    # Handle documentation mode
    bcu_doc_brief "List registry images"
    bcu_doc_shown || return 0

    # Validate GitHub PAT
    zrbg_validate_pat

    # Collect all versions with pagination
    zrbg_collect_all_versions

    # Display results
    bcu_step "List Current Registry Images"
    bcu_info "Processing collected JSON data..."

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
    bcu_doc_brief "Delete image from registry"
    bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
    bcu_doc_shown || return 0

    # Argument validation
    test -n "$fqin" || bcu_usage_die
    bvu_val_fqin "fqin" "$fqin" 1 512

    # Validate GitHub PAT
    zrbg_validate_pat

    # Command execution
    bcu_step "Delete Container Registry Image"
    echo "Deleting image: $fqin"

    # Extract tag
    echo "Extracting tag from FQIN..."
    local tag=$(echo "$fqin" | sed 's/.*://')
    echo "Using tag: '$tag'"
    test -n "$tag" || bcu_die "Could not extract a valid tag from FQIN $fqin"

    # Collect all versions
    zrbg_collect_all_versions

    # Find version ID for tag
    echo "DEBUG: Collecting tag and ID mapping..."
    jq -r '.[] | select(.metadata.container.tags != null) | .id as $id | .metadata.container.tags[] as $tag | [$id, $tag] | @tsv' \
        "${ZRBG_COLLECT_FULL_JSON}" > "${RBG_TEMP_DIR}/all_tags_with_ids.txt"

    echo "DEBUG: Searching for tag '$tag' in extracted mapping..."
    grep "${tag}$" "${RBG_TEMP_DIR}/all_tags_with_ids.txt" | cut -f1 > "${ZRBG_DELETE_VERSION_ID_CACHE}" || true

    local match_count=$(wc -l < "${ZRBG_DELETE_VERSION_ID_CACHE}" | tr -d ' ')
    echo "DEBUG: Found ${match_count} exact matching version(s)"
    echo "DEBUG: Matching version IDs:"
    cat "${ZRBG_DELETE_VERSION_ID_CACHE}"

    test "$match_count" -eq 1 || bcu_die "Expected exactly 1 matching version, found $match_count"

    local version_id=$(cat "${ZRBG_DELETE_VERSION_ID_CACHE}")
    echo "Found version ID: $version_id"

    # Confirm deletion unless skipped
    if [ "${RBG_ARG_SKIP_DELETE_CONFIRMATION:-}" != "SKIP" ]; then
        zrbg_confirm_action "Confirm delete image?" || bcu_die "WONT DELETE"
    fi

    # Delete image version
    bcu_info "Deleting image version..."
    local delete_url="${ZRBG_GITAPI_URL}/user/packages/container/${RBRR_REGISTRY_NAME}/versions/${version_id}"
    zrbg_curl_delete "$delete_url" > "${ZRBG_DELETE_RESULT_CACHE}" 2>&1

    # Check result
    grep -q "HTTP_STATUS:204" "${ZRBG_DELETE_RESULT_CACHE}" || \
        bcu_die "Failed to delete image version. HTTP Status: $(grep 'HTTP_STATUS' "${ZRBG_DELETE_RESULT_CACHE}" || echo 'unknown')"

    echo "Successfully deleted image version."

    # Cleanup
    rm "${ZRBG_DELETE_VERSION_ID_CACHE}" "${ZRBG_DELETE_RESULT_CACHE}"

    bcu_success "No errors."
}

rbg_retrieve() {
    # Name parameters
    local fqin="${1:-}"

    # Handle documentation mode
    bcu_doc_brief "Retrieve image from registry"
    bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
    bcu_doc_shown || return 0

    # Argument validation
    test -n "$fqin" || bcu_usage_die
    bvu_val_fqin "fqin" "$fqin" 1 512

    # Validate GitHub PAT
    zrbg_validate_pat

    # Command execution
    bcu_step "Retrieve Container Registry Image"

    # Login to registry
    zrbg_registry_login

    # Pull image
    bcu_info "Fetch image..."
    podman pull "$fqin"

    bcu_success "No errors."
}

bcu_execute rbg_ "Recipe Bottle GitHub - Container Registry Management" zrbg_validate_envvars "$@"

# eof

