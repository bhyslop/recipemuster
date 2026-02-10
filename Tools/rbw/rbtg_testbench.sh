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
# RBTG Testbench - Recipe Bottle dispatch exercise testbench

set -euo pipefail

RBTG_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBTG_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBTG_SCRIPT_DIR}/../buk/but_test.sh"
source "${RBTG_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${RBTG_SCRIPT_DIR}/rbz_zipper.sh"

buc_context "${0##*/}"

rbtg_case_dispatch_exercise() {
  buc_step "Kindling buz"
  zbuz_kindle

  buc_step "Kindling rbz"
  zrbz_kindle

  buc_step "Initializing evidence collection"
  buz_init_evidence

  buc_step "Dispatching RBZ_LIST_IMAGES"
  buz_dispatch "${RBZ_LIST_IMAGES}"

  buc_step "Verifying dispatch and evidence collection"
  local z_step
  z_step=$(buz_last_step_capture) || buc_die "No step recorded after dispatch"
  local z_status
  z_status=$(buz_get_step_exit_capture "$z_step")
  but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_LIST_IMAGES}"
  local z_output_dir
  z_output_dir=$(buz_get_step_output_capture "${z_step}") || buc_die "Failed to get step output dir"
  test -d "${z_output_dir}" || buc_die "Evidence directory not created: ${z_output_dir}"
  buc_log_args "Evidence dir: ${z_output_dir}"
  buc_log_args "Testbench output dir: ${BURD_OUTPUT_DIR}"

  buc_step "Verifying BURV isolation"
  local z_burv_temp="${ZBUZ_EVIDENCE_ROOT}/step-${z_step}/burv-temp"
  local z_inner_temp_count=0
  z_inner_temp_count=$(find "${z_burv_temp}" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | wc -l) || true
  test "${z_inner_temp_count}" -gt 0 || buc_die "Inner dispatch did not create temp under BURV root: ${z_burv_temp}"
  buc_log_args "Inner dispatch temp dirs under BURV root: ${z_inner_temp_count}"

  buc_success "Dispatch exercise passed"
}

######################################################################
# rbtg_case_ark_lifecycle - Full ark lifecycle integration test
#
# 6-step sequence: list(baseline) → conjure → list(verify+1) → retrieve → delete → list(verify=baseline)
# Exercises RBZ zipper dispatch against live GCP environment.
#
# Expects BURD_TOKEN_3 to contain vessel sigil (from tabtarget imprint).

rbtg_case_ark_lifecycle() {
  buc_step "Kindling buz"
  zbuz_kindle

  buc_step "Kindling rbz"
  zrbz_kindle

  buc_step "Initializing evidence collection"
  buz_init_evidence

  # Resolve vessel directory from tabtarget imprint
  local z_vessel_sigil="${BURD_TOKEN_3:-}"
  test -n "${z_vessel_sigil}" || buc_die "No vessel sigil - BURD_TOKEN_3 empty (tabtarget imprint required)"
  local z_vessel_dir="rbev-vessels/${z_vessel_sigil}"
  test -d "${z_vessel_dir}" || buc_die "Vessel directory not found: ${z_vessel_dir}"
  buc_log_args "Vessel: ${z_vessel_sigil} at ${z_vessel_dir}"
  local z_step z_status

  # Helper: count locator lines from captured list output
  # Locator lines are `moniker:tag` with no spaces; skip "Repository:" header
  zrbtg_count_locators() {
    grep -cE '^[^[:space:]:]+:[^[:space:]]+$' "$1" || echo 0
  }

  # Step 1: Capture baseline image list
  buc_step "Step 1/6: Listing images (baseline)"
  local z_baseline_file="${BURD_TEMP_DIR}/rbtg_list_baseline.txt"
  buz_dispatch "${RBZ_LIST_IMAGES}" > "${z_baseline_file}"
  z_step=$(buz_last_step_capture)
  z_status=$(buz_get_step_exit_capture "$z_step")
  but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_LIST_IMAGES}"
  local z_baseline_count
  z_baseline_count=$(zrbtg_count_locators "${z_baseline_file}")
  buc_info "Baseline locator count: ${z_baseline_count}"

  # Step 2: Conjure ark
  buc_step "Step 2/6: Conjuring ark from vessel ${z_vessel_sigil}"
  buz_dispatch "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"
  z_step=$(buz_last_step_capture)
  z_status=$(buz_get_step_exit_capture "$z_step")
  but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_CONJURE_ARK}"

  # Step 3: Verify image count increased
  buc_step "Step 3/6: Listing images (post-conjure)"
  local z_post_conjure_file="${BURD_TEMP_DIR}/rbtg_list_post_conjure.txt"
  buz_dispatch "${RBZ_LIST_IMAGES}" > "${z_post_conjure_file}"
  z_step=$(buz_last_step_capture)
  z_status=$(buz_get_step_exit_capture "$z_step")
  but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_LIST_IMAGES}"
  local z_post_conjure_count
  z_post_conjure_count=$(zrbtg_count_locators "${z_post_conjure_file}")
  buc_info "Post-conjure locator count: ${z_post_conjure_count}"
  test "${z_post_conjure_count}" -gt "${z_baseline_count}" \
    || buc_die "Locator count did not increase: baseline=${z_baseline_count} post=${z_post_conjure_count}"

  # Harvest new locators by diffing baseline vs post-conjure
  local z_new_locators_file="${BURD_TEMP_DIR}/rbtg_new_locators.txt"
  comm -13 \
    <(grep -E '^[^[:space:]:]+:[^[:space:]]+$' "${z_baseline_file}" | sort) \
    <(grep -E '^[^[:space:]:]+:[^[:space:]]+$' "${z_post_conjure_file}" | sort) \
    > "${z_new_locators_file}"
  local z_new_count
  z_new_count=$(wc -l < "${z_new_locators_file}" | tr -d ' ')
  test "${z_new_count}" -gt 0 || buc_die "No new locators found after conjure"
  buc_log_args "New locators (${z_new_count}):"
  buc_log_args "$(cat "${z_new_locators_file}")"

  # Step 4: Retrieve image using first new locator
  buc_step "Step 4/6: Retrieving image"
  local z_retrieve_locator
  z_retrieve_locator=$(head -1 "${z_new_locators_file}")
  test -n "${z_retrieve_locator}" || buc_die "No locator available for retrieve"
  buc_info "Retrieving: ${z_retrieve_locator}"
  buz_dispatch "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"
  z_step=$(buz_last_step_capture)
  z_status=$(buz_get_step_exit_capture "$z_step")
  but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_RETRIEVE_IMAGE}"

  # Step 5: Delete all new locators to restore baseline
  buc_step "Step 5/6: Deleting new images"
  while IFS= read -r z_locator; do
    buc_info "Deleting: ${z_locator}"
    buz_dispatch "${RBZ_DELETE_IMAGE}" "${z_locator}" "--force"
    z_step=$(buz_last_step_capture)
    z_status=$(buz_get_step_exit_capture "$z_step")
    but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_DELETE_IMAGE}"
  done < "${z_new_locators_file}"

  # Step 6: Verify count restored to baseline
  buc_step "Step 6/6: Listing images (post-delete)"
  local z_post_delete_file="${BURD_TEMP_DIR}/rbtg_list_post_delete.txt"
  buz_dispatch "${RBZ_LIST_IMAGES}" > "${z_post_delete_file}"
  z_step=$(buz_last_step_capture)
  z_status=$(buz_get_step_exit_capture "$z_step")
  but_fatal_on_error "$z_status" "dispatch failed" "Colophon: ${RBZ_LIST_IMAGES}"
  local z_final_count
  z_final_count=$(zrbtg_count_locators "${z_post_delete_file}")
  buc_info "Final locator count: ${z_final_count}"
  test "${z_final_count}" -eq "${z_baseline_count}" \
    || buc_die "Count not restored: baseline=${z_baseline_count} final=${z_final_count}"

  buc_success "Ark lifecycle test passed"
}

rbtg_route() {
  local z_command="${1:-}"
  shift || true

  test -n "${BURD_TEMP_DIR:-}" || buc_die "BURD_TEMP_DIR not set - must be called from BURD"
  test -n "${BURD_NOW_STAMP:-}" || buc_die "BURD_NOW_STAMP not set - must be called from BURD"

  case "${z_command}" in
    rbtg-de) rbtg_case_dispatch_exercise ;;
    rbtg-al) rbtg_case_ark_lifecycle ;;
    *) buc_die "Unknown command: ${z_command}" ;;
  esac
}

rbtg_main() {
  local z_command="${1:-}"
  shift || true
  test -n "${z_command}" || buc_die "No command specified"
  rbtg_route "${z_command}" "$@"
}

rbtg_main "$@"

# eof
