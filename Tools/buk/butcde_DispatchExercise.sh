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
# BUTCDE - Dispatch exercise test cases for RBTB testbench

set -euo pipefail

######################################################################
# Private helper: init dispatch and evidence scoped to this test case
# Uses BUT_TEMP_DIR (set per-case by zbute_case) for isolated evidence

zbutcde_init() {
  bute_init_dispatch
  BURD_TEMP_DIR="${BUT_TEMP_DIR}"
  bute_init_evidence
}

######################################################################
# butcde_evidence_created - Dispatch butctt colophon, verify step
# arrays populated and evidence directory exists

butcde_evidence_created() {
  zbutcde_init
  buto_section "Dispatching test target colophon"
  bute_dispatch "${ZBUTCDE_TEST_COLOPHON}"

  buto_section "Verifying dispatch recorded a step"
  local z_step
  z_step=$(bute_last_step_capture) || buto_fatal "No step recorded after dispatch"

  buto_section "Verifying evidence directory exists"
  local z_output_dir
  z_output_dir=$(bute_get_step_output_capture "${z_step}") || buto_fatal "Failed to get step output dir"
  test -d "${z_output_dir}" || buto_fatal "Evidence directory not created: ${z_output_dir}"
  buto_info "Evidence dir: ${z_output_dir}"

  buto_success "Evidence creation verified"
}

######################################################################
# butcde_burv_isolation - Verify BURV temp dirs created by inner dispatch

butcde_burv_isolation() {
  zbutcde_init
  buto_section "Dispatching test target colophon"
  bute_dispatch "${ZBUTCDE_TEST_COLOPHON}"

  local z_step
  z_step=$(bute_last_step_capture) || buto_fatal "No step recorded after dispatch"

  buto_section "Verifying BURV isolation"
  local z_burv_temp="${ZBUTE_EVIDENCE_ROOT}/step-${z_step}/burv-temp"
  test -d "${z_burv_temp}" || buto_fatal "BURV temp directory not created: ${z_burv_temp}"
  buto_info "BURV temp dir: ${z_burv_temp}"

  buto_success "BURV isolation verified"
}

######################################################################
# butcde_exit_capture - Verify bute_get_step_exit_capture returns
# correct status after successful dispatch

butcde_exit_capture() {
  zbutcde_init
  buto_section "Dispatching test target colophon"
  bute_dispatch "${ZBUTCDE_TEST_COLOPHON}"

  local z_step
  z_step=$(bute_last_step_capture) || buto_fatal "No step recorded after dispatch"

  buto_section "Verifying exit capture"
  local z_status
  z_status=$(bute_get_step_exit_capture "${z_step}")
  buto_fatal_on_error "${z_status}" "dispatch failed" "Colophon: ${ZBUTCDE_TEST_COLOPHON}"
  buto_info "Exit status: ${z_status}"

  buto_success "Exit capture verified"
}

# eof
