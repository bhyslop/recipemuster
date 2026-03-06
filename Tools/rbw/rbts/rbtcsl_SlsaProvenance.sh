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
# RBTCSL - SLSA provenance test cases for RBTB testbench

set -euo pipefail

######################################################################
# SLSA provenance integration test
#
# 4-step sequence: conjure → check → vouch → cleanup
# Exercises full SLSA v1.0 provenance pipeline against live GCP environment.
#
# Expects ZRBTB_ARK_VESSEL_SIGIL to contain vessel sigil (set by setup function).

rbtcsl_provenance_tcase() {
  # Resolve vessel directory from global setup variable
  local z_vessel_sigil="${ZRBTB_ARK_VESSEL_SIGIL:-}"
  test -n "${z_vessel_sigil}" || buto_fatal "No vessel sigil - ZRBTB_ARK_VESSEL_SIGIL empty (setup function must set this)"
  local z_vessel_dir="rbev-vessels/${z_vessel_sigil}"
  test -d "${z_vessel_dir}" || buto_fatal "Vessel directory not found: ${z_vessel_dir}"
  buto_info "Vessel: ${z_vessel_sigil} at ${z_vessel_dir}"

  # Step 1: Conjure ark
  buto_section "Step 1/4: Conjuring ark from vessel ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"

  # Read fact files from BURV output
  local z_conjure_burv="${ZBUTO_BURV_OUTPUT}"
  test -n "${z_conjure_burv}" || buto_fatal "ZBUTO_BURV_OUTPUT empty after conjure"

  local z_image_ref=$(<"${z_conjure_burv}/current/${RBF_FACT_IMAGE_REF}")
  test -n "${z_image_ref}" || buto_fatal "Image ref fact file empty"
  buto_info "Image ref: ${z_image_ref}"

  local z_consecration=$(<"${z_conjure_burv}/current/rbf_consecration.txt")
  test -n "${z_consecration}" || buto_fatal "Consecration output empty"
  buto_info "Consecration: ${z_consecration}"

  # Step 2: Check consecrations — verify conjured image appears
  buto_section "Step 2/4: Checking consecrations for ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_CHECK_CONSECRATIONS}" "${z_vessel_dir}"
  buto_info "Consecration check output available"

  # Step 3: Vouch provenance — assert SLSA Level 3
  buto_section "Step 3/4: Vouching provenance for ${z_vessel_sigil}"
  buto_tt_expect_ok "${RBZ_VOUCH_ARK}" "${z_vessel_dir}" "${z_consecration}"

  local z_vouch_burv="${ZBUTO_BURV_OUTPUT}"
  test -n "${z_vouch_burv}" || buto_fatal "ZBUTO_BURV_OUTPUT empty after vouch"

  local z_slsa_level=$(<"${z_vouch_burv}/current/${RBF_FACT_SLSA_LEVEL}")
  test -n "${z_slsa_level}" || buto_fatal "SLSA level fact file empty"
  buto_info "SLSA build level: ${z_slsa_level}"

  test "${z_slsa_level}" = "3" \
    || buto_fatal "Expected SLSA Level 3 but got: ${z_slsa_level}"

  # Step 4: Cleanup — abjure the conjured ark
  buto_section "Step 4/4: Abjuring ark to restore baseline"
  buto_tt_expect_ok "${RBZ_ABJURE_ARK}" "${z_vessel_dir}" "${z_consecration}" "--force"

  buto_success "SLSA provenance test passed — Level 3 verified"
}

# eof
