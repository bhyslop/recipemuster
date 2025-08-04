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

# Test suite for RBIM Image Management integration

# Source the libraries from parent directory
ZTRBIM_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
source "${ZTRBIM_SCRIPT_DIR}/btu_BashTestUtility.sh"

# Helper function to invoke RBIM with proper environment
trbim_invoke_rbim() {
    local rbg_temp_dir="$1"; shift
    local rbim_command="$1"; shift

    # Generate timestamp for this RBIM invocation
    local rbg_now_stamp="$(date +%Y%m%d__%H%M%S)"

    RBG_TEMP_DIR="${rbg_temp_dir}"                    \
      RBG_NOW_STAMP="${rbg_now_stamp}"                \
      RBG_RUNTIME="${RBG_RUNTIME:-podman}"            \
      "${ZTRBIM_SCRIPT_DIR}/rbim_cli.sh"              \
      "${rbim_command}" "$@"
}

trbim_case_image_management_workflow() {
    local recipe_file="$(dirname "$0")/trbim_dockerfile.recipe"
    test -f "${recipe_file}" || btu_fatal "Recipe file not found: ${recipe_file}"

    # Create subdirectories for each RBIM operation
    local list1_dir="${BTU_TEMP_DIR}/list_before"
    local build_dir="${BTU_TEMP_DIR}/build"
    local list2_dir="${BTU_TEMP_DIR}/list_during"
    local retrieve_dir="${BTU_TEMP_DIR}/retrieve"
    local delete_dir="${BTU_TEMP_DIR}/delete"
    local list3_dir="${BTU_TEMP_DIR}/list_after"

    mkdir -p "${list1_dir}" "${build_dir}" "${list2_dir}" "${retrieve_dir}" "${delete_dir}" "${list3_dir}"

    btu_info "Step 1: List registry images before build"
    btu_expect_ok trbim_invoke_rbim "${list1_dir}" rbim_list

    btu_info "Step 2: Build container from recipe"
    local fqin_file="${build_dir}/fqin_output.txt"
    RBG_ARG_FQIN_OUTPUT="${fqin_file}" \
    btu_expect_ok trbim_invoke_rbim "${build_dir}" rbim_build "${recipe_file}"

    test -f "${fqin_file}" || btu_fatal "FQIN output file not created: ${fqin_file}"
    local fqin="$(cat "${fqin_file}")"
    test -n "${fqin}" || btu_fatal "Empty FQIN in output file"

    btu_info "Built image: ${fqin}"

    btu_info "Step 3: List registry images after build"
    btu_expect_ok trbim_invoke_rbim "${list2_dir}" rbim_list

    btu_info "Step 4: Retrieve image from registry"
    # Extract tag from FQIN for rbim_retrieve
    local tag="${fqin##*:}"
    btu_expect_ok trbim_invoke_rbim "${retrieve_dir}" rbim_retrieve "${tag}"

    btu_info "Step 5: Delete image from registry"
    RBG_ARG_SKIP_DELETE_CONFIRMATION="SKIP" \
    btu_expect_ok trbim_invoke_rbim "${delete_dir}" rbim_delete "${fqin}"

    btu_info "Step 6: List registry images after deletion"
    btu_expect_ok trbim_invoke_rbim "${list3_dir}" rbim_list

    btu_info "RBIM image management workflow integration test completed successfully"
}

btu_execute "$1" "trbim_case_" "$2"

# eof

