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
# BUK Test Engine - _tcase boundary runner, dispatch, and evidence machinery

set -euo pipefail

# Source test-case API
ZBUTE_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZBUTE_SCRIPT_DIR}/buto_operations.sh"

######################################################################
# _tcase boundary — case isolation subshell

# _tcase boundary runner: execute case function in isolation subshell
zbute_tcase() {
  set -e

  local z_case_name="${1}"
  declare -F "${z_case_name}" >/dev/null || buto_fatal "Test function not found: ${z_case_name}"

  buto_section "START: ${z_case_name}"

  local z_case_temp_dir="${ZBUTE_ROOT_TEMP_DIR}/${z_case_name}"
  mkdir -p "${z_case_temp_dir}" || buto_fatal "Failed to create test temp dir: ${z_case_temp_dir}"

  local z_status=0
  (
    set -e
    export BUT_TEMP_DIR="${z_case_temp_dir}"
    export BUTE_BURV_ROOT="${z_case_temp_dir}/burv"
    "${z_case_name}"
  ) || z_status=$?

  buto_trace "Ran: ${z_case_name} and got status:${z_status}"
  buto_fatal_on_error "${z_status}" "Test failed: ${z_case_name}"

  buto_trace "Finished: ${z_case_name} with status: ${z_status}"
  test "${BUT_VERBOSE:-0}" -le 0 || echo "${ZBUTO_GREEN}PASSED:${ZBUTO_RESET} ${z_case_name}" >&2
}

# bute_execute removed - dispatch now iterates cases directly via butr_cases_recite

######################################################################
# Dispatch and evidence infrastructure

# bute_init_dispatch() - Initialize step tracking arrays
bute_init_dispatch() {
  test -z "${ZBUTE_DISPATCH_READY:-}" || buto_fatal "bute dispatch already initialized"
  zbute_step_colophons=()
  zbute_step_exit_status=()
  zbute_step_output_dir=()
  ZBUTE_DISPATCH_READY=1
}

zbute_dispatch_sentinel() {
  test "${ZBUTE_DISPATCH_READY:-}" = "1" || buto_fatal "bute dispatch not initialized - call bute_init_dispatch first"
}

# Non-fatal tabtarget resolution (returns 1 on failure instead of dying)
zbute_resolve_tabtarget_capture() {
  zbute_dispatch_sentinel

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || return 1

  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)

  # Bash 3.2: no-match glob returns literal — check with test -e
  test -e "${z_matches[0]}" || return 1
  test "${#z_matches[@]}" -eq 1 || return 1

  echo "${z_matches[0]}"
}

# bute_init_evidence() - Create evidence root dir under testbench temp
bute_init_evidence() {
  zbute_dispatch_sentinel
  zburd_sentinel

  ZBUTE_EVIDENCE_ROOT="${BURD_TEMP_DIR}/evidence"
  mkdir -p "${ZBUTE_EVIDENCE_ROOT}"
  buc_log_args "Evidence root: ${ZBUTE_EVIDENCE_ROOT}"
}

# bute_dispatch() - Invoke tabtarget via BURV-isolated environment
# Args: colophon [extra_args...]
# Step index available via bute_last_step_capture after return
bute_dispatch() {
  zbute_dispatch_sentinel
  test -n "${ZBUTE_EVIDENCE_ROOT:-}" || buto_fatal "Evidence not initialized - call bute_init_evidence first"

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || buto_fatal "bute_dispatch requires colophon"
  shift

  local z_tabtarget
  z_tabtarget=$(zbute_resolve_tabtarget_capture "${z_colophon}") || buto_fatal "Cannot resolve tabtarget for '${z_colophon}'"

  local z_step_idx="${#zbute_step_colophons[@]}"

  local z_step_dir="${ZBUTE_EVIDENCE_ROOT}/step-${z_step_idx}"
  local z_burv_output="${z_step_dir}/burv-output"
  local z_burv_temp="${z_step_dir}/burv-temp"
  local z_evidence_dir="${z_step_dir}/evidence"
  mkdir -p "${z_burv_output}" "${z_burv_temp}" "${z_evidence_dir}"

  buc_log_args "Dispatching step ${z_step_idx}: colophon=${z_colophon} tabtarget=${z_tabtarget}"

  local z_exit_status=0
  BURD_NO_LOG= \
  BURV_OUTPUT_ROOT_DIR="${z_burv_output}" \
  BURV_TEMP_ROOT_DIR="${z_burv_temp}" \
    "${z_tabtarget}" "$@" || z_exit_status=$?

  buc_log_args "Step ${z_step_idx} exit status: ${z_exit_status}"
  buc_log_args "Step ${z_step_idx} inner BURD output: ${z_burv_output}"
  buc_log_args "Step ${z_step_idx} inner BURD temp: ${z_burv_temp}"
  buc_log_args "Step ${z_step_idx} evidence dir: ${z_evidence_dir}"

  if test -d "${z_burv_output}/current"; then
    cp -r "${z_burv_output}/current/." "${z_evidence_dir}/" || buc_warn "Evidence harvest failed for step ${z_step_idx}"
  fi

  if test "${z_exit_status}" -ne 0; then
    zbuto_render_lines "FAIL" "${ZBUTO_RED}" \
      "Step ${z_step_idx} FAILED - inner process artifacts:" \
      "  BURD output: ${z_burv_output}" \
      "  BURD temp:   ${z_burv_temp}" \
      "  Evidence:    ${z_evidence_dir}"
  fi

  zbute_step_colophons+=("${z_colophon}")
  zbute_step_exit_status+=("${z_exit_status}")
  zbute_step_output_dir+=("${z_evidence_dir}")
}

######################################################################
# Step result _capture functions

bute_last_step_capture() {
  zbute_dispatch_sentinel
  local z_count="${#zbute_step_colophons[@]}"
  test "${z_count}" -gt 0 || return 1
  echo "$((z_count - 1))"
}

bute_get_step_exit_capture() {
  zbute_dispatch_sentinel
  local z_idx="${1:-}"
  test -n "${z_idx}" || return 1
  echo "${zbute_step_exit_status[$z_idx]}"
}

bute_get_step_output_capture() {
  zbute_dispatch_sentinel
  local z_idx="${1:-}"
  test -n "${z_idx}" || return 1
  echo "${zbute_step_output_dir[$z_idx]}"
}

# eof
