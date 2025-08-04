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

# Test suite for RBIM Image Management

# Source the libraries from parent directory
ZTRBIM_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
source "${ZTRBIM_SCRIPT_DIR}/btu_BashTestUtility.sh"

# Helper function to invoke RBIM with proper environment
ztrbim_invoke_rbim() {
  local z_rbg_temp_dir="$1"; shift
  local z_rbim_command="$1"; shift

  # Generate timestamp for this RBIM invocation
  local z_rbg_now_stamp
  z_rbg_now_stamp="$(date +%Y%m%d__%H%M%S)"

  RBG_TEMP_DIR="${z_rbg_temp_dir}"     \
  RBG_NOW_STAMP="${z_rbg_now_stamp}"   \
  RBG_RUNTIME="podman"                 \
    "${ZTRBIM_SCRIPT_DIR}/rbim_cli.sh" \
    "${z_rbim_command}" "$@"
}

trbim_case_github_workflow() {
  local z_recipe_file="$(dirname "$0")/trbim_dockerfile.recipe"
  test -f "${z_recipe_file}" || btu_fatal "Recipe file not found: ${z_recipe_file}"

  # Create subdirectories for each RBIM operation
  local z_list1_dir="${BTU_TEMP_DIR}/list_before"
  local z_build_dir="${BTU_TEMP_DIR}/build"
  local z_list2_dir="${BTU_TEMP_DIR}/list_during"
  local z_retrieve_dir="${BTU_TEMP_DIR}/retrieve"
  local z_delete_dir="${BTU_TEMP_DIR}/delete"
  local z_list3_dir="${BTU_TEMP_DIR}/list_after"

  mkdir -p "${z_list1_dir}" "${z_build_dir}" "${z_list2_dir}" \
           "${z_retrieve_dir}" "${z_delete_dir}" "${z_list3_dir}"

  btu_info "Step 1: List registry images before build"
  btu_expect_ok ztrbim_invoke_rbim "${z_list1_dir}" list

  btu_info "Step 2: Build container from recipe"
  local z_fqin_file="${z_build_dir}/fqin_output.txt"
  RBG_ARG_FQIN_OUTPUT="${z_fqin_file}" \
  btu_expect_ok ztrbim_invoke_rbim "${z_build_dir}" build "${z_recipe_file}"

  test -f "${z_fqin_file}" || btu_fatal "FQIN output file not created: ${z_fqin_file}"
  local z_fqin
  z_fqin="$(<"${z_fqin_file}")"
  test -n "${z_fqin}" || btu_fatal "Empty FQIN in output file"

  btu_info "Built image: ${z_fqin}"

  # Extract tag from FQIN for retrieve command
  local z_tag="${z_fqin##*:}"

  btu_info "Step 3: List registry images after build"
  btu_expect_ok ztrbim_invoke_rbim "${z_list2_dir}" list

  btu_info "Step 4: Retrieve image from registry"
  btu_expect_ok ztrbim_invoke_rbim "${z_retrieve_dir}" retrieve "${z_tag}"

  btu_info "Step 5: Delete image from registry"
  RBG_ARG_SKIP_DELETE_CONFIRMATION="SKIP" \
  btu_expect_ok ztrbim_invoke_rbim "${z_delete_dir}" delete "${z_fqin}"

  btu_info "Step 6: List registry images after deletion"
  btu_expect_ok ztrbim_invoke_rbim "${z_list3_dir}" list

  btu_info "GitHub workflow integration test completed successfully"
}

trbim_case_image_info() {
  btu_info "Test image_info command"

  local z_info_dir="${BTU_TEMP_DIR}/image_info"
  mkdir -p "${z_info_dir}"

  # Run image_info without filter
  btu_expect_ok ztrbim_invoke_rbim "${z_info_dir}" image_info

  # Run image_info with filter (if images exist with 'test' in the name)
  local z_info_filtered_dir="${BTU_TEMP_DIR}/image_info_filtered"
  mkdir -p "${z_info_filtered_dir}"

  btu_info "Running image_info with filter 'test'"
  btu_expect_ok ztrbim_invoke_rbim "${z_info_filtered_dir}" image_info "test"

  btu_info "Image info test completed successfully"
}

btu_execute "$1" "trbim_case_" "$2"

# eof

