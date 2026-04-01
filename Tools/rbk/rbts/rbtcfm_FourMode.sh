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
# RBTCFM - Four-mode integration test cases for RBTB testbench

set -euo pipefail

######################################################################
# Four-mode integration test
#
# 15-step sequence exercising all four delivery modes end-to-end:
#   conjure(busybox) → bind(plantuml) → graft(busybox-graft) →
#   kludge(busybox) → tally(all vouched) → vouch_gate → retrieve →
#   run → cleanup → abjure(×3) → tally(all gone)
#
# Each rbfd_ordain call produces image + about + vouch (chained internally).
# Kludge produces a local-only image + fake vouch tag (no GAR artifacts).
# Tally verifies health=vouched for the three GAR modes.
# Steps 6-9 exercise consumer-side operations on the conjured busybox.
#
# Key invariant: four modes coexist with different trust paths.
#   conjure → SLSA provenance (Cloud Build, multi-platform)
#   bind    → digest-pin trust (Cloud Build, skopeo mirror)
#   graft   → GRAFTED trust (local push to GAR)
#   kludge  → local-only dev build (docker build, fake vouch, no GAR)
#
# Supersedes rbtcal_ArkLifecycle (single-mode conjure test).
# Expects foundry kindled in baste.

rbtcfm_four_mode_tcase() {
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
  buto_section "Step 1/15: Conjuring consecration from vessel ${z_conjure_vessel}"
  buto_tt_expect_ok "${RBZ_ORDAIN_CONSECRATION}" "${z_conjure_dir}"
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
  buto_section "Step 2/15: Binding consecration from vessel ${z_bind_vessel}"
  buto_tt_expect_ok "${RBZ_ORDAIN_CONSECRATION}" "${z_bind_dir}"
  local z_bind_output
  z_bind_output=$(buto_tt_previous_output_capture)
  local z_bind_consec
  z_bind_consec=$(<"${z_bind_output}/${RBF_FACT_CONSECRATION}")
  test -n "${z_bind_consec}" || buto_fatal "Empty consecration after bind"
  buto_info "Bound: ${z_bind_consec}"

  # Retrieve conjured image for graft input
  buto_section "Retrieving conjured busybox for graft input"
  local -r z_graft_retrieve_locator="${z_conjure_ark_stem}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_tt_expect_ok "${RBZ_WREST_IMAGE}" "${z_graft_retrieve_locator}"
  local -r z_local_image_ref="${z_conjure_gar_root}/${z_graft_retrieve_locator}"
  export BURE_TWEAK_NAME=threemodegraft
  export BURE_TWEAK_VALUE="${z_local_image_ref}"
  buto_info "Tweak: ${BURE_TWEAK_NAME}=${BURE_TWEAK_VALUE}"

  # Step 3: Graft busybox
  buto_section "Step 3/15: Grafting consecration from vessel ${z_graft_vessel}"
  buto_tt_expect_ok "${RBZ_ORDAIN_CONSECRATION}" "${z_graft_dir}"
  local z_graft_output
  z_graft_output=$(buto_tt_previous_output_capture)
  local z_graft_consec
  z_graft_consec=$(<"${z_graft_output}/${RBF_FACT_CONSECRATION}")
  test -n "${z_graft_consec}" || buto_fatal "Empty consecration after graft"
  buto_info "Grafted: ${z_graft_consec}"

  # Step 4: Kludge busybox (local-only dev build — no GAR artifacts)
  buto_section "Step 4/15: Kludging local build from vessel ${z_conjure_vessel}"
  buto_tt_expect_ok "${RBZ_KLUDGE_VESSEL}" "${z_conjure_dir}"
  local z_kludge_output
  z_kludge_output=$(buto_tt_previous_output_capture)
  local z_kludge_consec
  z_kludge_consec=$(<"${z_kludge_output}/${RBF_FACT_CONSECRATION}")
  test -n "${z_kludge_consec}" || buto_fatal "Empty consecration after kludge"
  buto_info "Kludged: ${z_kludge_consec}"

  # Verify kludge image and vouch tag exist locally
  local -r z_kludge_image_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_conjure_vessel}:${z_kludge_consec}${RBGC_ARK_SUFFIX_IMAGE}"
  local -r z_kludge_vouch_ref="${ZRBFC_REGISTRY_HOST}/${ZRBFC_REGISTRY_PATH}/${z_conjure_vessel}:${z_kludge_consec}${RBGC_ARK_SUFFIX_VOUCH}"

  docker inspect "${z_kludge_image_ref}" >/dev/null 2>&1 \
    || buto_fatal "Kludge image not found in local docker store: ${z_kludge_image_ref}"
  buto_info "Kludge image verified: ${z_kludge_image_ref}"

  docker inspect "${z_kludge_vouch_ref}" >/dev/null 2>&1 \
    || buto_fatal "Kludge vouch tag not found in local docker store: ${z_kludge_vouch_ref}"
  buto_info "Kludge vouch tag verified: ${z_kludge_vouch_ref}"

  # Run kludge image and verify output
  local z_kludge_run_output
  z_kludge_run_output=$(docker run --rm "${z_kludge_image_ref}" 2>&1) \
    || buto_fatal "docker run failed for kludge image: ${z_kludge_image_ref}"
  local -r z_kludge_expected="BusyBox container is running!"
  case "${z_kludge_run_output}" in
    *"${z_kludge_expected}"*) buto_info "Kludge output verified: ${z_kludge_expected}" ;;
    *) buto_fatal "Expected kludge output containing '${z_kludge_expected}', got: ${z_kludge_run_output}" ;;
  esac

  # Cleanup kludge images from local store
  docker rmi "${z_kludge_image_ref}" "${z_kludge_vouch_ref}" \
    || buto_fatal "Failed to remove kludge images"
  buto_info "Kludge images cleaned up"

  # Step 5: Tally — verify all three GAR modes show vouched health
  buto_section "Step 5/15: Tallying consecrations (all three GAR modes should be vouched)"
  buto_tt_expect_ok "${RBZ_TALLY_CONSECRATIONS}"

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

  buto_info "All three GAR consecrations present and vouched"

  # Step 6: Verify consumer vouch gate — confirm consumer-side enforcement passes
  buto_section "Step 6/15: Verifying vouch gate (conjured busybox)"
  rbfv_vouch_gate "${z_conjure_vessel}" "${z_conjure_consec}"
  buto_info "Vouch gate passed: ${z_conjure_consec}"

  # Step 7: Retrieve image
  buto_section "Step 7/15: Retrieving image (conjured busybox)"
  local -r z_retrieve_locator="${z_conjure_vessel}:${z_conjure_consec}${RBGC_ARK_SUFFIX_IMAGE}"
  buto_info "Retrieving: ${z_retrieve_locator}"
  buto_tt_expect_ok "${RBZ_WREST_IMAGE}" "${z_retrieve_locator}"

  # Step 8: Run and verify — docker run the image, assert expected output
  buto_section "Step 8/15: Running image and verifying output"
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

  # Step 9: Cleanup image — remove pulled image from local store
  buto_section "Step 9/15: Cleaning up local image"
  docker rmi "${z_full_image_ref}" \
    || buto_fatal "Failed to remove image: ${z_full_image_ref}"
  buto_info "Image removed: ${z_full_image_ref}"

  # Step 10: Abjure conjure ark
  buto_section "Step 10/15: Abjuring conjure consecration"
  buto_tt_expect_ok "${RBZ_ABJURE_CONSECRATION}" "${z_conjure_dir}" "${z_conjure_consec}" "--force"
  buto_info "Abjured conjure: ${z_conjure_consec}"

  # Step 11: Abjure bind ark
  buto_section "Step 11/15: Abjuring bind consecration"
  buto_tt_expect_ok "${RBZ_ABJURE_CONSECRATION}" "${z_bind_dir}" "${z_bind_consec}" "--force"
  buto_info "Abjured bind: ${z_bind_consec}"

  # Step 12: Abjure graft ark
  buto_section "Step 12/15: Abjuring graft consecration"
  buto_tt_expect_ok "${RBZ_ABJURE_CONSECRATION}" "${z_graft_dir}" "${z_graft_consec}" "--force"
  buto_info "Abjured graft: ${z_graft_consec}"

  # Step 13: Tally — verify all three GAR arks are gone
  buto_section "Step 13/15: Tallying consecrations (post-abjure)"
  buto_tt_expect_ok "${RBZ_TALLY_CONSECRATIONS}"

  local z_post_abjure_dir
  z_post_abjure_dir=$(buto_tt_previous_output_capture)

  test ! -f "${z_post_abjure_dir}/${z_conjure_fact}" \
    || buto_fatal "Conjure consecration still present after abjure: ${z_conjure_fact}"
  test ! -f "${z_post_abjure_dir}/${z_bind_fact}" \
    || buto_fatal "Bind consecration still present after abjure: ${z_bind_fact}"
  test ! -f "${z_post_abjure_dir}/${z_graft_fact}" \
    || buto_fatal "Graft consecration still present after abjure: ${z_graft_fact}"

  buto_success "Four-mode integration test passed (15-step conjure→bind→graft→kludge supply chain)"
}

# eof
