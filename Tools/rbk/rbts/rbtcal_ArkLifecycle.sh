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
# 10-step sequence exercising the full supply chain:
#   conjure → check(present) → vouch → check(health) → vouch_gate →
#   retrieve → run → cleanup → abjure → check(absent)
#
# Exercises RBZ zipper dispatch against live GCP environment.
# Steps 3 (vouch) and 5 (vouch gate) call foundry functions directly.
# Step 7 (run) uses hardcoded docker (not regime runtime).
#
# Expects ZRBTB_ARK_VESSEL_SIGIL to contain vessel sigil (set by baste).
# Expects foundry kindled in baste (for direct rbf_vouch/rbf_vouch_gate calls).

rbtcal_lifecycle_tcase() {
  local -r z_vessel_sigil="${ZRBTB_ARK_VESSEL_SIGIL:-}"
  test -n "${z_vessel_sigil}" || buto_fatal "No vessel sigil - ZRBTB_ARK_VESSEL_SIGIL empty (baste must set this)"
  local -r z_vessel_dir="rbev-vessels/${z_vessel_sigil}"
  test -d "${z_vessel_dir}" || buto_fatal "Vessel directory not found: ${z_vessel_dir}"
  buto_info "Vessel: ${z_vessel_sigil} at ${z_vessel_dir}"

  # Step 1: Conjure ark
  buto_section "Step 1/10: Conjuring ark from vessel ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"
  local z_conjure_dir
  z_conjure_dir=$(buto_tt_previous_output_capture)
  local z_consecration
  z_consecration=$(<"${z_conjure_dir}/${RBF_FACT_CONSECRATION}")
  test -n "${z_consecration}" || buto_fatal "Consecration fact file empty after conjure"
  buto_info "Conjured consecration: ${z_consecration}"

  # Step 2: Check consecrations — verify conjured ark is present
  buto_section "Step 2/10: Checking consecrations (post-conjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_check_dir
  z_check_dir=$(buto_tt_previous_output_capture)
  local -r z_consec_fact="${z_vessel_sigil}${RBCC_FACT_CONSEC_INFIX}${z_consecration}"
  test -f "${z_check_dir}/${z_consec_fact}" \
    || buto_fatal "Consecration not found after conjure: ${z_consec_fact}"
  buto_info "Confirmed consecration present: ${z_consecration}"

  # Step 3: Vouch — verify SLSA provenance and publish -vouch artifact
  buto_section "Step 3/10: Vouching for ark (may take minutes — Cloud Build)"
  rbf_vouch "${z_vessel_dir}" "${z_consecration}"
  buto_info "Vouch complete: ${z_consecration}"

  # Step 4: Check health classification — verify health shows vouched
  buto_section "Step 4/10: Checking consecrations (post-vouch)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_post_vouch_dir
  z_post_vouch_dir=$(buto_tt_previous_output_capture)
  test -f "${z_post_vouch_dir}/${z_consec_fact}" \
    || buto_fatal "Consecration not found after vouch: ${z_consec_fact}"
  buto_info "Confirmed consecration present with vouch: ${z_consecration}"

  # Step 5: Verify consumer vouch gate — confirm consumer-side enforcement passes
  buto_section "Step 5/10: Verifying vouch gate"
  rbf_vouch_gate "${z_vessel_sigil}" "${z_consecration}"
  buto_info "Vouch gate passed: ${z_consecration}"

  # Step 6: Retrieve image
  buto_section "Step 6/10: Retrieving image"
  local -r z_retrieve_locator="${z_vessel_sigil}:${z_consecration}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_info "Retrieving: ${z_retrieve_locator}"
  buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"

  # Step 7: Run and verify — docker run the image, assert expected output
  buto_section "Step 7/10: Running image and verifying output"
  local -r z_gar_host="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}"
  local -r z_full_image_ref="${z_gar_host}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${z_retrieve_locator}"
  local z_run_output
  z_run_output=$(docker run --rm "${z_full_image_ref}" 2>&1) \
    || buto_fatal "docker run failed for ${z_full_image_ref}"
  local -r z_expected="BusyBox container is running!"
  case "${z_run_output}" in
    *"${z_expected}"*) buto_info "Output verified: ${z_expected}" ;;
    *) buto_fatal "Expected output containing '${z_expected}', got: ${z_run_output}" ;;
  esac

  # Step 8: Cleanup image — remove pulled image from local store
  buto_section "Step 8/10: Cleaning up local image"
  docker rmi "${z_full_image_ref}" \
    || buto_fatal "Failed to remove image: ${z_full_image_ref}"
  buto_info "Image removed: ${z_full_image_ref}"

  # Step 9: Abjure the conjured ark
  buto_section "Step 9/10: Abjuring ark"
  buto_info "Abjuring consecration: ${z_consecration}"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_vessel_dir}" "${z_consecration}" "--force"

  # Step 10: Check consecrations — verify conjured ark is gone
  buto_section "Step 10/10: Checking consecrations (post-abjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  local z_post_abjure_dir
  z_post_abjure_dir=$(buto_tt_previous_output_capture)
  test ! -f "${z_post_abjure_dir}/${z_consec_fact}" \
    || buto_fatal "Consecration still present after abjure: ${z_consec_fact}"

  buto_success "Ark lifecycle test passed (10-step supply chain)"
}

# eof
