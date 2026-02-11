#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# RBTCAL - Ark lifecycle test cases for RBTB testbench

set -euo pipefail

######################################################################
# Private helpers

# zrbtcal_count_locators - Count locator lines from captured list output
# Locator lines are `moniker:tag` with no spaces; skip "Repository:" header
zrbtcal_count_locators() {
  grep -cE '^[^[:space:]:]+:[^[:space:]]+$' "$1" || echo 0
}

######################################################################
# Ark lifecycle integration test
#
# 6-step sequence: list(baseline) → conjure → list(verify+1) → retrieve → delete → list(verify=baseline)
# Exercises RBZ zipper dispatch against live GCP environment.
#
# Expects ZRBTB_ARK_VESSEL_SIGIL to contain vessel sigil (set by setup function).

rbtcal_lifecycle() {
  # Resolve vessel directory from global setup variable
  local z_vessel_sigil="${ZRBTB_ARK_VESSEL_SIGIL:-}"
  test -n "${z_vessel_sigil}" || buto_fatal "No vessel sigil - ZRBTB_ARK_VESSEL_SIGIL empty (setup function must set this)"
  local z_vessel_dir="rbev-vessels/${z_vessel_sigil}"
  test -d "${z_vessel_dir}" || buto_fatal "Vessel directory not found: ${z_vessel_dir}"
  buto_info "Vessel: ${z_vessel_sigil} at ${z_vessel_dir}"

  # Step 1: Capture baseline image list
  buto_section "Step 1/6: Listing images (baseline)"
  local z_baseline_file="${BUT_TEMP_DIR}/rbtcal_list_baseline.txt"
  buto_tt_expect_ok "${RBZ_LIST_IMAGES}"
  echo "${ZBUTO_STDOUT}" > "${z_baseline_file}"
  local z_baseline_count
  z_baseline_count=$(zrbtcal_count_locators "${z_baseline_file}")
  buto_info "Baseline locator count: ${z_baseline_count}"

  # Step 2: Conjure ark
  buto_section "Step 2/6: Conjuring ark from vessel ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"

  # Step 3: Verify image count increased
  buto_section "Step 3/6: Listing images (post-conjure)"
  local z_post_conjure_file="${BUT_TEMP_DIR}/rbtcal_list_post_conjure.txt"
  buto_tt_expect_ok "${RBZ_LIST_IMAGES}"
  echo "${ZBUTO_STDOUT}" > "${z_post_conjure_file}"
  local z_post_conjure_count
  z_post_conjure_count=$(zrbtcal_count_locators "${z_post_conjure_file}")
  buto_info "Post-conjure locator count: ${z_post_conjure_count}"
  test "${z_post_conjure_count}" -gt "${z_baseline_count}" \
    || buto_fatal "Locator count did not increase: baseline=${z_baseline_count} post=${z_post_conjure_count}"

  # Harvest new locators by diffing baseline vs post-conjure
  local z_new_locators_file="${BUT_TEMP_DIR}/rbtcal_new_locators.txt"
  comm -13 \
    <(grep -E '^[^[:space:]:]+:[^[:space:]]+$' "${z_baseline_file}" | sort) \
    <(grep -E '^[^[:space:]:]+:[^[:space:]]+$' "${z_post_conjure_file}" | sort) \
    > "${z_new_locators_file}"
  local z_new_count
  z_new_count=$(wc -l < "${z_new_locators_file}" | tr -d ' ')
  test "${z_new_count}" -gt 0 || buto_fatal "No new locators found after conjure"
  buto_info "New locators (${z_new_count}):"
  buto_info "$(cat "${z_new_locators_file}")"

  # Step 4: Retrieve image using first new locator
  buto_section "Step 4/6: Retrieving image"
  local z_retrieve_locator
  z_retrieve_locator=$(head -1 "${z_new_locators_file}")
  test -n "${z_retrieve_locator}" || buto_fatal "No locator available for retrieve"
  buto_info "Retrieving: ${z_retrieve_locator}"
  buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"

  # Step 5: Delete all new locators to restore baseline
  buto_section "Step 5/6: Deleting new images"
  while IFS= read -r z_locator; do
    buto_info "Deleting: ${z_locator}"
    buto_tt_expect_ok "${RBZ_DELETE_IMAGE}" "${z_locator}" "--force"
  done < "${z_new_locators_file}"

  # Step 6: Verify count restored to baseline
  buto_section "Step 6/6: Listing images (post-delete)"
  local z_post_delete_file="${BUT_TEMP_DIR}/rbtcal_list_post_delete.txt"
  buto_tt_expect_ok "${RBZ_LIST_IMAGES}"
  echo "${ZBUTO_STDOUT}" > "${z_post_delete_file}"
  local z_final_count
  z_final_count=$(zrbtcal_count_locators "${z_post_delete_file}")
  buto_info "Final locator count: ${z_final_count}"
  test "${z_final_count}" -eq "${z_baseline_count}" \
    || buto_fatal "Count not restored: baseline=${z_baseline_count} final=${z_final_count}"

  buto_success "Ark lifecycle test passed"
}

# eof
