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

# Test suite for RBG GitHub integration

# Source the libraries from parent directory
ZTRBG_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
source "${ZTRBG_SCRIPT_DIR}/btu_BashTestUtility.sh"

# Helper function to invoke RBG with proper environment
trbg_invoke_rbg() {
    local rbg_temp_dir="$1"; shift
    local rbg_command="$1"; shift
    
    # Generate timestamp for this RBG invocation
    local rbg_now_stamp="$(date +%Y%m%d__%H%M%S)"
    
    RBG_TEMP_DIR="${rbg_temp_dir}" \
    RBG_NOW_STAMP="${rbg_now_stamp}" \
    RBG_RBRR_FILE="rbrr.repo.sh" \
    "${ZTRBG_SCRIPT_DIR}/rbg_RecipeBottleGithub.sh" \
    "${rbg_command}" "$@"
}

trbg_case_github_workflow() {
    local recipe_file="$(dirname "$0")/trbg_dockerfile.recipe"
    test -f "${recipe_file}" || btu_fatal "Recipe file not found: ${recipe_file}"
    
    # Create subdirectories for each RBG operation
    local list1_dir="${BTU_TEMP_DIR}/list_before"
    local build_dir="${BTU_TEMP_DIR}/build"
    local list2_dir="${BTU_TEMP_DIR}/list_during"
    local retrieve_dir="${BTU_TEMP_DIR}/retrieve"
    local delete_dir="${BTU_TEMP_DIR}/delete"
    local list3_dir="${BTU_TEMP_DIR}/list_after"
    
    mkdir -p "${list1_dir}" "${build_dir}" "${list2_dir}" "${retrieve_dir}" "${delete_dir}" "${list3_dir}"
    
    btu_info "Step 1: List registry images before build"
    btu_expect_ok trbg_invoke_rbg "${list1_dir}" rbg_list
    
    btu_info "Step 2: Build container from recipe"
    local fqin_file="${build_dir}/fqin_output.txt"
    RBG_ARG_FQIN_OUTPUT="${fqin_file}" \
    btu_expect_ok trbg_invoke_rbg "${build_dir}" rbg_build "${recipe_file}"
    
    test -f "${fqin_file}" || btu_fatal "FQIN output file not created: ${fqin_file}"
    local fqin="$(cat "${fqin_file}")"
    test -n "${fqin}" || btu_fatal "Empty FQIN in output file"
    
    btu_info "Built image: ${fqin}"
    
    btu_info "Step 3: List registry images after build"
    btu_expect_ok trbg_invoke_rbg "${list2_dir}" rbg_list
    
    btu_info "Step 4: Retrieve image from registry"
    btu_expect_ok trbg_invoke_rbg "${retrieve_dir}" rbg_retrieve "${fqin}"
    
    btu_info "Step 5: Delete image from registry"
    RBG_ARG_SKIP_DELETE_CONFIRMATION="SKIP" \
    btu_expect_ok trbg_invoke_rbg "${delete_dir}" rbg_delete "${fqin}"
    
    btu_info "Step 6: List registry images after deletion"
    btu_expect_ok trbg_invoke_rbg "${list3_dir}" rbg_list
    
    btu_info "GitHub workflow integration test completed successfully"
}

btu_execute "$1" "trbg_case_" "$2"

# eof

