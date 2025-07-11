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
source "${ZRBG_SCRIPT_DIR}/crgv.validate.sh"

######################################################################
# Module Variables (ZRBG_*)
# These variables are used across multiple functions within this module
# Naming convention: ZRBG_<PURPOSE>
ZRBG_GIT_REGISTRY="ghcr.io"
ZRBG_GITAPI_URL="https://api.github.com"
ZRBG_REPO_PREFIX="${ZRBG_GITAPI_URL}/repos"
ZRBG_COLLECT_FULL_JSON="${RBG_TEMP_DIR}/RBG_COMBINED__${RBG_NOW_STAMP}.json"
ZRBG_COLLECT_TEMP_PAGE="${RBG_TEMP_DIR}/RBG_PAGE__${RBG_NOW_STAMP}.json"


######################################################################
# Internal Functions (zrbg_*)
# These are helper functions used internally by the module
# Naming convention: zrbg_<action>_<object>

# Document, establish, validate environment
zrbg_validate_envvars() {
    set -e

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
    set -e

    test -f "${RBRR_GITHUB_PAT_ENV}" || bcu_die "GitHub PAT env file not found at ${RBRR_GITHUB_PAT_ENV}"

    # Load and check PAT exists
    source "${RBRR_GITHUB_PAT_ENV}"
    test -n "${RBV_PAT:-}" || bcu_die "RBV_PAT missing from ${RBRR_GITHUB_PAT_ENV}"
}

# Perform authenticated GET request
# Usage: zrbg_curl_get <url>
zrbg_curl_get() {
    set -e
    local url="$1"

    source "${RBRR_GITHUB_PAT_ENV}"
    curl -s -H "Authorization: token ${RBV_PAT}" \
            -H 'Accept: application/vnd.github.v3+json' \
            "$url"
}

# Collect all package versions with pagination
# Outputs: Combined JSON file at ZRBG_COLLECT_FULL_JSON
zrbg_collect_all_versions() {
    set -e

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

# Internal helper functions
zrbg_curl_headers() {
    set -e

    echo "-H \"Authorization: token \$RBV_PAT\" -H 'Accept: application/vnd.github.v3+json'"
}

######################################################################
# External Functions (rbg_*)
# These are functions used from outside this module
# Naming convention: rbg_<action>

rbg_build() {
    set -e

    # Name parameters, perhaps provide defaults for optional ones
    local recipe_file="${1:-}"

    # Handle documentation mode
    bcu_doc_brief "Build container from recipe"
    bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
    bcu_doc_shown || return 0

    # Argument validation
    test -n "$recipe_file" || bcu_usage_die
    test -f "$recipe_file" || bcu_die "Recipe file not found: $recipe_file"

    # Command execution
    bcu_step "Build container from $recipe_file"
    bcu_warn "Not implemented yet"
    bcu_success "Build completed"
}

rbg_list() {
    set -e

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
    set -e

    # Name parameters, perhaps provide defaults for optional ones
    local     fqin="${1:-}"

    # Handle documentation mode
    bcu_doc_brief "Delete image from registry"
    bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
    bcu_doc_shown || return 0

    # Argument validation
    test -n "$fqin" || bcu_usage_die
    crgv_fqin "rbg_delete" "$fqin" 1 512

    # Command execution
    bcu_step "Delete image: $fqin"
    bcu_warn "Not implemented yet"
    bcu_success "Delete completed"
}

rbg_retrieve() {
    set -e

    # Name parameters, perhaps provide defaults for optional ones
    local     fqin="${1:-}"

    # Handle documentation mode
    bcu_doc_brief "Retrieve image from registry"
    bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
    bcu_doc_shown || return 0

    # Argument validation
    test -n "$fqin" || bcu_die "Usage: rbg_retrieve <fully_qualified_image_name>"
    crgv_fqin "rbg_retrieve" "$fqin" 1 512

    # Command execution
    bcu_step "Retrieve image: $fqin"
    bcu_warn "Not implemented yet"
    bcu_success "Retrieve completed"
}

bcu_execute rbg_ "Recipe Bottle GitHub - Container Registry Management" zrbg_validate_envvars "$@"

# eof
