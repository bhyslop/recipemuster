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
  buz_dispatch_expect_ok "${RBZ_LIST_IMAGES}"

  buc_step "Verifying evidence collection"
  local z_step
  z_step=$(buz_last_step_capture) || buc_die "No step recorded after dispatch"
  local z_output_dir
  z_output_dir=$(buz_get_step_output_capture "${z_step}") || buc_die "Failed to get step output dir"
  test -d "${z_output_dir}" || buc_die "Evidence directory not created: ${z_output_dir}"
  buc_log_args "Evidence dir: ${z_output_dir}"
  buc_log_args "Testbench output dir: ${BUD_OUTPUT_DIR}"

  buc_step "Verifying BURV isolation"
  local z_burv_temp="${ZBUZ_EVIDENCE_ROOT}/step-${z_step}/burv-temp"
  local z_inner_temp_count=0
  z_inner_temp_count=$(find "${z_burv_temp}" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | wc -l) || true
  test "${z_inner_temp_count}" -gt 0 || buc_die "Inner dispatch did not create temp under BURV root: ${z_burv_temp}"
  buc_log_args "Inner dispatch temp dirs under BURV root: ${z_inner_temp_count}"

  buc_success "Dispatch exercise passed"
}

rbtg_route() {
  local z_command="${1:-}"
  shift || true

  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR not set - must be called from BUD"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP not set - must be called from BUD"

  case "${z_command}" in
    rbtg-de) rbtg_case_dispatch_exercise ;;
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
