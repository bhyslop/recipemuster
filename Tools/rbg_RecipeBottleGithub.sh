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
SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "$SCRIPT_DIR/bcu_BashConsoleUtility.sh"
source "$SCRIPT_DIR/crgv.validate.sh"

# Internal constants
ZRBG_GIT_REGISTRY="ghcr.io"
ZRBG_GITAPI_URL="https://api.github.com"

# All repo variables are needed
source "$SCRIPT_DIR/rbrr.validator.sh"


# Internal helper functions
zrbg_curl_headers() {
    set -e
    echo "-H \"Authorization: token \$RBV_PAT\" -H 'Accept: application/vnd.github.v3+json'"
}

rbg_build() {
    set -e

    bcu_doc_brief "Build container from recipe"
    bcu_doc_param "recipe_file" "Path to recipe file containing build instructions"
    bcu_doc_shown || return 0

    local recipe_file="${1:-}"
    [[ -n "$recipe_file" ]] || bcu_die "Usage: rbg_build <recipe_file>"
    [[ -f "$recipe_file" ]] || bcu_die "Recipe file not found: $recipe_file"
    
    bcu_step "Build container from $recipe_file"
    bcu_warn "Not implemented yet"
    bcu_pass "Build completed"
}

rbg_list() {
    set -e

    bcu_doc_brief "List registry images"
    bcu_doc_shown || return 0
    
    bcu_step "List registry images"
    bcu_warn "Not implemented yet"
    bcu_pass "List completed"
}

rbg_delete() {
    set -e

    bcu_doc_brief "Delete image from registry"
    bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
    bcu_doc_shown || return 0

    local fqin="${1:-}"
    [[ -n "$fqin" ]] || bcu_die "Usage: rbg_delete <fully_qualified_image_name>"
    
    # Validate FQIN format
    FQIN_TEMP="$fqin"
    crgv_fqin "rbg_delete" "FQIN_TEMP" 1 512
    
    bcu_step "Delete image: $fqin"
    bcu_warn "Not implemented yet"
    bcu_pass "Delete completed"
}

rbg_retrieve() {
    set -e

    bcu_doc_brief "Retrieve image from registry"
    bcu_doc_param "fqin" "Fully qualified image name (e.g., ghcr.io/owner/repo:tag)"
    bcu_doc_shown || return 0

    local fqin="${1:-}"
    [[ -n "$fqin" ]] || bcu_die "Usage: rbg_retrieve <fully_qualified_image_name>"
    
    # Validate FQIN format
    FQIN_TEMP="$fqin"
    crgv_fqin "rbg_retrieve" "FQIN_TEMP" 1 512

    bcu_step "Retrieve image: $fqin"
    bcu_warn "Not implemented yet"
    bcu_pass "Retrieve completed"
}

rbg_help() {
    set -e
    
    bcu_doc_brief "Show help for Recipe Bottle GitHub commands"
    bcu_doc_shown || return 0
    
    echo "Recipe Bottle GitHub - Container Registry Management"
    echo
    echo "Commands:"
    
    for cmd in $(declare -F | grep -E '^declare -f rbg_[a-z_]+$' | cut -d' ' -f3); do
        echo
        bcu_set_doc_mode "$cmd"
        $cmd
    done
}

# Main dispatch
cmd="${1:-}"
shift || true

# Check if function exists and matches our pattern
if declare -F "$cmd" >/dev/null && [[ "$cmd" =~ ^rbg_[a-z_]+$ ]]; then
    "$cmd" "$@"
else
    if [[ -n "$cmd" ]]; then
        bcu_warn "Unknown command: $cmd"
        rbg_help
        exit 1
    else
        rbg_help
        exit 0
    fi
fi

# eof

