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

# zrbtcal_read_burv_fact - Read a fact file from ZBUTO_BURV_OUTPUT after tabtarget
zrbtcal_read_burv_fact() {
  local z_fact_name="${1:-}"
  test -n "${ZBUTO_BURV_OUTPUT_DIR}" || buto_fatal "ZBUTO_BURV_OUTPUT_DIR empty after tabtarget"
  local z_path="${ZBUTO_BURV_OUTPUT_DIR}/${z_fact_name}"
  test -f "${z_path}" || buto_fatal "Fact file not found: ${z_path}"
  cat "${z_path}"
}

# zrbtcal_count_consecrations - Count lines in consecrations fact file content
zrbtcal_count_consecrations() {
  local z_content="${1:-}"
  if test -z "${z_content}"; then
    echo 0
  else
    echo "${z_content}" | wc -l | tr -d ' '
  fi
}

######################################################################
# Ark lifecycle integration test
#
# 6-step sequence: check(baseline) → conjure → check(verify+1) → retrieve → abjure → check(verify=baseline)
# Exercises RBZ zipper dispatch against live GCP environment.
#
# Expects ZRBTB_ARK_VESSEL_SIGIL to contain vessel sigil (set by setup function).

rbtcal_lifecycle_tcase() {
  # Resolve vessel directory from global setup variable
  local z_vessel_sigil="${ZRBTB_ARK_VESSEL_SIGIL:-}"
  test -n "${z_vessel_sigil}" || buto_fatal "No vessel sigil - ZRBTB_ARK_VESSEL_SIGIL empty (setup function must set this)"
  local z_vessel_dir="rbev-vessels/${z_vessel_sigil}"
  test -d "${z_vessel_dir}" || buto_fatal "Vessel directory not found: ${z_vessel_dir}"
  buto_info "Vessel: ${z_vessel_sigil} at ${z_vessel_dir}"

  # Step 1: Capture baseline consecration count
  buto_section "Step 1/6: Checking consecrations (baseline)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_baseline_consecrations
  z_baseline_consecrations=$(zrbtcal_read_burv_fact "${RBF_FACT_CONSECRATIONS}")
  local z_baseline_count
  z_baseline_count=$(zrbtcal_count_consecrations "${z_baseline_consecrations}")
  buto_info "Baseline consecration count: ${z_baseline_count}"

  # Step 2: Conjure ark
  buto_section "Step 2/6: Conjuring ark from vessel ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"
  local z_consecration
  z_consecration=$(zrbtcal_read_burv_fact "${RBF_FACT_CONSECRATION}")
  test -n "${z_consecration}" || buto_fatal "Consecration fact file empty after conjure"
  buto_info "Conjured consecration: ${z_consecration}"

  # Step 3: Verify consecration count increased
  buto_section "Step 3/6: Checking consecrations (post-conjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_post_conjure_consecrations
  z_post_conjure_consecrations=$(zrbtcal_read_burv_fact "${RBF_FACT_CONSECRATIONS}")
  local z_post_conjure_count
  z_post_conjure_count=$(zrbtcal_count_consecrations "${z_post_conjure_consecrations}")
  buto_info "Post-conjure consecration count: ${z_post_conjure_count}"
  test "${z_post_conjure_count}" -gt "${z_baseline_count}" \
    || buto_fatal "Consecration count did not increase: baseline=${z_baseline_count} post=${z_post_conjure_count}"

  # Step 4: Retrieve image using conjured consecration
  buto_section "Step 4/6: Retrieving image"
  local z_retrieve_locator="${z_vessel_sigil}:${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_info "Retrieving: ${z_retrieve_locator}"
  buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"

  # Step 5: Abjure the conjured ark
  buto_section "Step 5/6: Abjuring ark"
  buto_info "Abjuring consecration: ${z_consecration}"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_vessel_dir}" "${z_consecration}" "--force"

  # Step 6: Verify consecration count restored to baseline
  buto_section "Step 6/6: Checking consecrations (post-abjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_post_abjure_consecrations
  z_post_abjure_consecrations=$(zrbtcal_read_burv_fact "${RBF_FACT_CONSECRATIONS}")
  local z_final_count
  z_final_count=$(zrbtcal_count_consecrations "${z_post_abjure_consecrations}")
  buto_info "Final consecration count: ${z_final_count}"
  test "${z_final_count}" -eq "${z_baseline_count}" \
    || buto_fatal "Count not restored: baseline=${z_baseline_count} final=${z_final_count}"

  buto_success "Ark lifecycle test passed"
}

# eof
