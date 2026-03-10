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
# Ark lifecycle integration test
#
# 5-step sequence: conjure → check(present) → retrieve → abjure → check(absent)
# Exercises RBZ zipper dispatch against live GCP environment.
#
# Expects ZRBTB_ARK_VESSEL_SIGIL to contain vessel sigil (set by setup function).

rbtcal_lifecycle_tcase() {
  local z_vessel_sigil="${ZRBTB_ARK_VESSEL_SIGIL:-}"
  test -n "${z_vessel_sigil}" || buto_fatal "No vessel sigil - ZRBTB_ARK_VESSEL_SIGIL empty (setup function must set this)"
  local z_vessel_dir="rbev-vessels/${z_vessel_sigil}"
  test -d "${z_vessel_dir}" || buto_fatal "Vessel directory not found: ${z_vessel_dir}"
  buto_info "Vessel: ${z_vessel_sigil} at ${z_vessel_dir}"

  # Step 1: Conjure ark
  buto_section "Step 1/5: Conjuring ark from vessel ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"
  local z_conjure_dir
  z_conjure_dir=$(buto_tt_previous_output_capture)
  local z_consecration
  z_consecration=$(<"${z_conjure_dir}/${RBF_FACT_CONSECRATION}")
  test -n "${z_consecration}" || buto_fatal "Consecration fact file empty after conjure"
  buto_info "Conjured consecration: ${z_consecration}"

  # Step 2: Check consecrations — verify conjured ark is present
  buto_section "Step 2/5: Checking consecrations (post-conjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_check_dir
  z_check_dir=$(buto_tt_previous_output_capture)
  local z_consec_fact="${z_vessel_sigil}${RBCC_FACT_CONSEC_INFIX}${z_consecration}"
  test -f "${z_check_dir}/${z_consec_fact}" \
    || buto_fatal "Consecration not found after conjure: ${z_consec_fact}"
  buto_info "Confirmed consecration present: ${z_consecration}"

  # Step 3: Retrieve image
  buto_section "Step 3/5: Retrieving image"
  local z_retrieve_locator="${z_vessel_sigil}:${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_info "Retrieving: ${z_retrieve_locator}"
  buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"

  # Step 4: Abjure the conjured ark
  buto_section "Step 4/5: Abjuring ark"
  buto_info "Abjuring consecration: ${z_consecration}"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_vessel_dir}" "${z_consecration}" "--force"

  # Step 5: Check consecrations — verify conjured ark is gone
  buto_section "Step 5/5: Checking consecrations (post-abjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_post_abjure_dir
  z_post_abjure_dir=$(buto_tt_previous_output_capture)
  test ! -f "${z_post_abjure_dir}/${z_consec_fact}" \
    || buto_fatal "Consecration still present after abjure: ${z_consec_fact}"

  buto_success "Ark lifecycle test passed"
}

# eof
