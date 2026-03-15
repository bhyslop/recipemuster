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
# RBTCTM - Three-mode integration test cases for RBTB testbench

set -euo pipefail

######################################################################
# Three-mode integration test
#
# 12-step sequence exercising all three delivery modes end-to-end:
#   conjure(busybox) → bind(plantuml) → graft(busybox-graft) →
#   check(all vouched) → vouch_gate → retrieve → run → cleanup →
#   abjure(×3) → check(all gone)
#
# Each rbf_create call produces image + about + vouch (chained internally).
# Consecration check verifies health=vouched for all three modes.
# Steps 5-8 exercise consumer-side operations on the conjured busybox.
#
# Key invariant: three arks coexist in the registry with different trust paths.
#   conjure → SLSA provenance
#   bind    → digest-pin trust
#   graft   → GRAFTED trust
#
# Supersedes rbtcal_ArkLifecycle (single-mode conjure test).
# Expects foundry kindled in baste.

rbtctm_three_mode_tcase() {
  local -r z_conjure_vessel="rbev-busybox"
  local -r z_bind_vessel="rbev-bottle-plantuml"
  local -r z_graft_vessel="rbev-busybox-graft"
  local -r z_conjure_dir="rbev-vessels/${z_conjure_vessel}"
  local -r z_bind_dir="rbev-vessels/${z_bind_vessel}"
  local -r z_graft_dir="rbev-vessels/${z_graft_vessel}"

  test -d "${z_conjure_dir}" || buto_fatal "Vessel directory not found: ${z_conjure_dir}"
  test -d "${z_bind_dir}"    || buto_fatal "Vessel directory not found: ${z_bind_dir}"
  test -d "${z_graft_dir}"   || buto_fatal "Vessel directory not found: ${z_graft_dir}"

  # Step 1: Conjure busybox
  buto_section "Step 1/12: Conjuring ark from vessel ${z_conjure_vessel}"
  buto_tt_expect_ok "${RBZ_CREATE_ARK}" "${z_conjure_dir}"
  local z_conjure_output
  z_conjure_output=$(buto_tt_previous_output_capture)
  local z_conjure_consec
  z_conjure_consec=$(<"${z_conjure_output}/${RBF_FACT_CONSECRATION}")
  test -n "${z_conjure_consec}" || buto_fatal "Empty consecration after conjure"
  local z_conjure_gar_root
  z_conjure_gar_root=$(<"${z_conjure_output}/${RBF_FACT_GAR_ROOT}")
  local z_conjure_ark_stem
  z_conjure_ark_stem=$(<"${z_conjure_output}/${RBF_FACT_ARK_STEM}")
  buto_info "Conjured: ${z_conjure_consec}"

  # Step 2: Bind plantuml
  buto_section "Step 2/12: Binding ark from vessel ${z_bind_vessel}"
  buto_tt_expect_ok "${RBZ_CREATE_ARK}" "${z_bind_dir}"
  local z_bind_output
  z_bind_output=$(buto_tt_previous_output_capture)
  local z_bind_consec
  z_bind_consec=$(<"${z_bind_output}/${RBF_FACT_CONSECRATION}")
  test -n "${z_bind_consec}" || buto_fatal "Empty consecration after bind"
  buto_info "Bound: ${z_bind_consec}"

  # Retrieve conjured image for graft input
  buto_section "Retrieving conjured busybox for graft input"
  local -r z_graft_retrieve_locator="${z_conjure_ark_stem}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_graft_retrieve_locator}"
  local -r z_local_image_ref="${z_conjure_gar_root}/${z_graft_retrieve_locator}"
  export BURE_TWEAK_NAME=threemodegraft
  export BURE_TWEAK_VALUE="${z_local_image_ref}"
  buto_info "Tweak: ${BURE_TWEAK_NAME}=${BURE_TWEAK_VALUE}"

  # Step 3: Graft busybox
  buto_section "Step 3/12: Grafting ark from vessel ${z_graft_vessel}"
  buto_tt_expect_ok "${RBZ_CREATE_ARK}" "${z_graft_dir}"
  local z_graft_output
  z_graft_output=$(buto_tt_previous_output_capture)
  local z_graft_consec
  z_graft_consec=$(<"${z_graft_output}/${RBF_FACT_CONSECRATION}")
  test -n "${z_graft_consec}" || buto_fatal "Empty consecration after graft"
  buto_info "Grafted: ${z_graft_consec}"

  # Step 4: Consecration check — verify all three show vouched health
  buto_section "Step 4/12: Checking consecrations (all three should be vouched)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}"

  local z_check_dir
  z_check_dir=$(buto_tt_previous_output_capture)

  local -r z_conjure_fact="${z_conjure_vessel}${RBCC_FACT_CONSEC_INFIX}${z_conjure_consec}"
  local -r z_bind_fact="${z_bind_vessel}${RBCC_FACT_CONSEC_INFIX}${z_bind_consec}"
  local -r z_graft_fact="${z_graft_vessel}${RBCC_FACT_CONSEC_INFIX}${z_graft_consec}"

  test -f "${z_check_dir}/${z_conjure_fact}" \
    || buto_fatal "Conjure consecration not found: ${z_conjure_fact}"
  test -f "${z_check_dir}/${z_bind_fact}" \
    || buto_fatal "Bind consecration not found: ${z_bind_fact}"
  test -f "${z_check_dir}/${z_graft_fact}" \
    || buto_fatal "Graft consecration not found: ${z_graft_fact}"

  buto_info "All three consecrations present and vouched"

  # Step 5: Verify consumer vouch gate — confirm consumer-side enforcement passes
  buto_section "Step 5/12: Verifying vouch gate (conjured busybox)"
  rbf_vouch_gate "${z_conjure_vessel}" "${z_conjure_consec}"
  buto_info "Vouch gate passed: ${z_conjure_consec}"

  # Step 6: Retrieve image
  buto_section "Step 6/12: Retrieving image (conjured busybox)"
  local -r z_retrieve_locator="${z_conjure_vessel}:${z_conjure_consec}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_info "Retrieving: ${z_retrieve_locator}"
  buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"

  # Step 7: Run and verify — docker run the image, assert expected output
  buto_section "Step 7/12: Running image and verifying output"
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
  buto_section "Step 8/12: Cleaning up local image"
  docker rmi "${z_full_image_ref}" \
    || buto_fatal "Failed to remove image: ${z_full_image_ref}"
  buto_info "Image removed: ${z_full_image_ref}"

  # Step 9: Abjure conjure ark
  buto_section "Step 9/12: Abjuring conjure ark"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_conjure_dir}" "${z_conjure_consec}" "--force"
  buto_info "Abjured conjure: ${z_conjure_consec}"

  # Step 10: Abjure bind ark
  buto_section "Step 10/12: Abjuring bind ark"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_bind_dir}" "${z_bind_consec}" "--force"
  buto_info "Abjured bind: ${z_bind_consec}"

  # Step 11: Abjure graft ark
  buto_section "Step 11/12: Abjuring graft ark"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_graft_dir}" "${z_graft_consec}" "--force"
  buto_info "Abjured graft: ${z_graft_consec}"

  # Step 12: Consecration check — verify all three are gone
  buto_section "Step 12/12: Checking consecrations (post-abjure)"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}"

  local z_post_abjure_dir
  z_post_abjure_dir=$(buto_tt_previous_output_capture)

  test ! -f "${z_post_abjure_dir}/${z_conjure_fact}" \
    || buto_fatal "Conjure consecration still present after abjure: ${z_conjure_fact}"
  test ! -f "${z_post_abjure_dir}/${z_bind_fact}" \
    || buto_fatal "Bind consecration still present after abjure: ${z_bind_fact}"
  test ! -f "${z_post_abjure_dir}/${z_graft_fact}" \
    || buto_fatal "Graft consecration still present after abjure: ${z_graft_fact}"

  buto_success "Three-mode integration test passed (12-step conjure→bind→graft supply chain)"
}

# eof
