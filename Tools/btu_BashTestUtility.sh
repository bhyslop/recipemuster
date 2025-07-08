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

# Internal constants
ZRBG_GIT_REGISTRY="ghcr.io"
ZRBG_GITAPI_URL="https://api.github.com"

# Document, establish, validate environment
zrbg_env() {

    # Handle documentation mode
    bcu_doc_env "RBG_TEMP_DIR  " "Empty temporary directory"
    bcu_doc_env "RBG_NOW_STAMP " "Timestamp for per run branding"
    bcu_doc_env "RBG_RBRR_FILE " "File containing the RBRR constants"

    bcu_env_done || return 0

    # Validate environment
    bvu_file_exists "${RBG_RBRR_FILE}"
    source          "${RBG_RBRR_FILE}"
    source "${ZRBG_SCRIPT_DIR}/rbrr.validator.sh"
}

# Internal helper functions
zrbg_curl_headers() {
    set -e

    echo "-H \"Authorization: token \$RBV_PAT\" -H 'Accept: application/vnd.github.v3+json'"
}

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
    bcu_pass "Build completed"
}

rbg_list() {
    set -e

    # Handle documentation mode
    bcu_doc_brief "List registry images"
    bcu_doc_shown || return 0

    # Command execution
    bcu_step "List registry images"
    bcu_warn "Not implemented yet"
    bcu_pass "List completed"
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
    bcu_pass "Delete completed"
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
    bcu_pass "Retrieve completed"
}

rbg_help() {
    set -e

    # Handle documentation mode
    bcu_doc_brief "Show help for available commands"
    bcu_doc_shown || return 0

    # Perform documentation
    shift $#
    bcu_set_doc_mode

    echo "Recipe Bottle GitHub - Container Registry Management"
    echo
    echo "Environment vars needed:"
    zrbg_env

    echo "Commands:"

    for zrbg_command in $(declare -F | grep -E '^declare -f rbg_[a-z_]+$' | cut -d' ' -f3); do
        bcu_context "$zrbg_command"
        $zrbg_command
    done
}

# Detect command, if any
zrbg_command="${1:-}"
shift || true

# Attempt execution
if declare -F   "$zrbg_command" >/dev/null &&\
           echo "$zrbg_command" | grep -q '^rbg_[a-z_]*$'; then
    bcu_context "$zrbg_command"

    zrbg_env
    "$zrbg_command" "$@"
else
    # Emit documentation
    test -z     "$zrbg_command" || bcu_warn "Unknown command: $zrbg_command"
    rbg_help
    exit 1
fi


# eof

