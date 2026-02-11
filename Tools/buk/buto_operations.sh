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
# BUK Test Operations - Assertions, invocations, dispatch, and evidence

set -euo pipefail

# Guard die — available before inclusion guard (buto_fatal not yet defined)
zbuto_guard_die() { echo "FATAL: $*" >&2; exit 1; }

# Multiple inclusion guard
test -z "${ZBUTO_INCLUDED:-}" || zbuto_guard_die "buto_operations multiply sourced"
ZBUTO_INCLUDED=1

######################################################################
# Color codes

buto_color() { test -n "${TERM:-}" && test "${TERM}" != "dumb" && printf '\033[%sm' "${1}" || printf ''; }
ZBUTO_WHITE=$(  buto_color '1;37' )
ZBUTO_RED=$(    buto_color '1;31' )
ZBUTO_GREEN=$(  buto_color '1;32' )
ZBUTO_RESET=$(  buto_color '0'    )

######################################################################
# Generic renderer for aligned multi-line messages
# Usage: zbuto_render_lines PREFIX COLOR LINES...

zbuto_render_lines() {
  local z_label="${1}"; shift
  local z_color="${1}"; shift

  local z_prefix="${z_label}:"

  local z_visible_prefix="${z_prefix}"
  test -z "${z_color}" || z_prefix="${z_color}${z_prefix}${ZBUTO_RESET}"
  local z_indent
  z_indent=$(printf '%*s' "$(echo -e "${z_visible_prefix}" | sed 's/\x1b\[[0-9;]*m//g' | wc -c)" '')

  local z_first=1
  local z_line=""
  for z_line in "$@"; do
    if test "${z_first}" -eq 1; then
      echo "${z_prefix} ${z_line}" >&2
      z_first=0
    else
      echo "${z_indent} ${z_line}" >&2
    fi
  done
}

######################################################################
# Output functions

buto_section() {
  test "${BUT_VERBOSE:-0}" -ge 1 || return 0
  zbuto_render_lines "info " "${ZBUTO_WHITE}" "$@"
}

buto_info() {
  test "${BUT_VERBOSE:-0}" -ge 1 || return 0
  zbuto_render_lines "info " "" "$@"
}

buto_trace() {
  test "${BUT_VERBOSE:-0}" -ge 2 || return 0
  zbuto_render_lines "trace" "" "$@"
}

buto_fatal() {
  zbuto_render_lines "ERROR" "${ZBUTO_RED}" "$@"
  exit 1
}

buto_fatal_on_error() {
  set -e
  local z_condition="${1}"; shift
  test "${z_condition}" -eq 0 && return 0
  buto_fatal "$@"
}

buto_fatal_on_success() {
  set -e
  local z_condition="${1}"; shift
  test "${z_condition}" -ne 0 && return 0
  buto_fatal "$@"
}

buto_success() {
  echo "${ZBUTO_GREEN}PASSED:${ZBUTO_RESET} $*" >&2
}

######################################################################
# Safely invoke a command under 'set -e', capturing stdout, stderr, and exit status
# Globals set:
#   ZBUTO_STDOUT  - command stdout
#   ZBUTO_STDERR  - command stderr
#   ZBUTO_STATUS  - command exit code

zbuto_invoke() {
  buto_trace "Invoking: $*"

  local z_tmp_stdout
  z_tmp_stdout=$(mktemp)
  local z_tmp_stderr
  z_tmp_stderr=$(mktemp)

  ZBUTO_STATUS=$( (
      set +e
      "$@" >"${z_tmp_stdout}" 2>"${z_tmp_stderr}"
      printf '%s' "$?"
      exit 0
    ) || printf '__subshell_failed__' )

  if test "${ZBUTO_STATUS}" = "__subshell_failed__" || test -z "${ZBUTO_STATUS}"; then
    ZBUTO_STATUS=127
    ZBUTO_STDOUT=""
    ZBUTO_STDERR="zbuto_invoke: command caused shell to exit before status could be captured"
  else
    ZBUTO_STDOUT=$(<"${z_tmp_stdout}")
    ZBUTO_STDERR=$(<"${z_tmp_stderr}")
  fi

  rm -f "${z_tmp_stdout}" "${z_tmp_stderr}"
}

######################################################################
# buto_unit_* - Raw command invocation via zbuto_invoke

buto_unit_expect_ok_stdout() {
  set -e

  local z_expected="${1}"; shift

  zbuto_invoke "$@"

  buto_fatal_on_error "${ZBUTO_STATUS}" "Command failed with status ${ZBUTO_STATUS}" \
                                        "Command: $*"                               \
                                        "STDERR: ${ZBUTO_STDERR}"

  test "${ZBUTO_STDOUT}" = "${z_expected}" || buto_fatal "Output mismatch"            \
                                                         "Command: $*"                \
                                                         "Expected: '${z_expected}'"  \
                                                         "Got:      '${ZBUTO_STDOUT}'"
}

buto_unit_expect_ok() {
  set -e

  zbuto_invoke "$@"

  buto_fatal_on_error "${ZBUTO_STATUS}" "Command failed with status ${ZBUTO_STATUS}" \
                                        "Command: $*"                               \
                                        "STDERR: ${ZBUTO_STDERR}"
}

buto_unit_expect_fatal() {
  set -e

  zbuto_invoke "$@"

  buto_fatal_on_success "${ZBUTO_STATUS}" "Expected failure but got success" \
                                          "Command: $*"                      \
                                          "STDOUT: ${ZBUTO_STDOUT}"          \
                                          "STDERR: ${ZBUTO_STDERR}"
}

######################################################################
# buto_tt_* - Tabtarget file invocation (requires tabtarget exists)
#
# Resolves colophon to tt/{colophon}.*.sh file, dies if missing.
# Extra args pass through to tabtarget script.

zbuto_resolve_tabtarget() {
  local z_colophon="${1:-}"
  test -n "${z_colophon}" || buto_fatal "zbuto_resolve_tabtarget: colophon required"

  local z_tt_dir="${BURC_TABTARGET_DIR:-}"
  test -n "${z_tt_dir}" || buto_fatal "BURC_TABTARGET_DIR not set -- buto_tt requires BUK environment"
  local z_matches=("${z_tt_dir}/${z_colophon}."*.sh)

  # Bash 3.2: no-match glob returns literal — check with test -e
  test -e "${z_matches[0]}" || buto_fatal "No tabtarget found for colophon '${z_colophon}' in ${z_tt_dir}/"

  test "${#z_matches[@]}" -eq 1 || buto_fatal "Multiple tabtargets found for colophon '${z_colophon}' in ${z_tt_dir}/"

  echo "${z_matches[0]}"
}

buto_tt_expect_ok() {
  set -e

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || buto_fatal "buto_tt_expect_ok: colophon required"
  shift

  local z_tabtarget
  z_tabtarget=$(zbuto_resolve_tabtarget "${z_colophon}")

  zbuto_invoke "${z_tabtarget}" "$@"

  buto_fatal_on_error "${ZBUTO_STATUS}" "Tabtarget failed with status ${ZBUTO_STATUS}" \
                                        "Colophon: ${z_colophon}"                      \
                                        "Tabtarget: ${z_tabtarget}"                    \
                                        "STDERR: ${ZBUTO_STDERR}"
}

buto_tt_expect_fatal() {
  set -e

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || buto_fatal "buto_tt_expect_fatal: colophon required"
  shift

  local z_tabtarget
  z_tabtarget=$(zbuto_resolve_tabtarget "${z_colophon}")

  zbuto_invoke "${z_tabtarget}" "$@"

  buto_fatal_on_success "${ZBUTO_STATUS}" "Expected failure but got success"    \
                                          "Colophon: ${z_colophon}"             \
                                          "Tabtarget: ${z_tabtarget}"           \
                                          "STDOUT: ${ZBUTO_STDOUT}"             \
                                          "STDERR: ${ZBUTO_STDERR}"
}

######################################################################
# buto_launch_* - Workbench dispatch (no tabtarget file required)
#
# First arg is launcher (workbench), second is colophon, rest are args.
# Invokes launcher directly with colophon+args.

buto_launch_expect_ok() {
  set -e

  local z_launcher="${1:-}"
  local z_colophon="${2:-}"
  test -n "${z_launcher}" || buto_fatal "buto_launch_expect_ok: launcher required"
  test -n "${z_colophon}" || buto_fatal "buto_launch_expect_ok: colophon required"
  shift 2

  zbuto_invoke "${z_launcher}" "${z_colophon}" "$@"

  buto_fatal_on_error "${ZBUTO_STATUS}" "Launch failed with status ${ZBUTO_STATUS}" \
                                        "Launcher: ${z_launcher}"                   \
                                        "Colophon: ${z_colophon}"                   \
                                        "STDERR: ${ZBUTO_STDERR}"
}

buto_launch_expect_fatal() {
  set -e

  local z_launcher="${1:-}"
  local z_colophon="${2:-}"
  test -n "${z_launcher}" || buto_fatal "buto_launch_expect_fatal: launcher required"
  test -n "${z_colophon}" || buto_fatal "buto_launch_expect_fatal: colophon required"
  shift 2

  zbuto_invoke "${z_launcher}" "${z_colophon}" "$@"

  buto_fatal_on_success "${ZBUTO_STATUS}" "Expected failure but got success" \
                                          "Launcher: ${z_launcher}"          \
                                          "Colophon: ${z_colophon}"          \
                                          "STDOUT: ${ZBUTO_STDOUT}"          \
                                          "STDERR: ${ZBUTO_STDERR}"
}

######################################################################
# Test case execution

# Run single test case in subshell
zbuto_case() {
  set -e

  local z_case_name="${1}"
  declare -F "${z_case_name}" >/dev/null || buto_fatal "Test function not found: ${z_case_name}"

  buto_section "START: ${z_case_name}"

  local z_case_temp_dir="${ZBUTO_ROOT_TEMP_DIR}/${z_case_name}"
  mkdir -p "${z_case_temp_dir}" || buto_fatal "Failed to create test temp dir: ${z_case_temp_dir}"

  local z_status=0
  (
    set -e
    export BUT_TEMP_DIR="${z_case_temp_dir}"
    "${z_case_name}"
  ) || z_status=$?

  buto_trace "Ran: ${z_case_name} and got status:${z_status}"
  buto_fatal_on_error "${z_status}" "Test failed: ${z_case_name}"

  buto_trace "Finished: ${z_case_name} with status: ${z_status}"
  test "${BUT_VERBOSE:-0}" -le 0 || echo "${ZBUTO_GREEN}PASSED:${ZBUTO_RESET} ${z_case_name}" >&2
}

# buto_execute removed - dispatch now iterates cases directly via butr_cases_recite

######################################################################
# Dispatch and evidence infrastructure

# buto_init_dispatch() - Initialize step tracking arrays
buto_init_dispatch() {
  test -z "${ZBUTO_DISPATCH_READY:-}" || buto_fatal "buto dispatch already initialized"
  zbuto_step_colophons=()
  zbuto_step_exit_status=()
  zbuto_step_output_dir=()
  ZBUTO_DISPATCH_READY=1
}

zbuto_dispatch_sentinel() {
  test "${ZBUTO_DISPATCH_READY:-}" = "1" || buto_fatal "buto dispatch not initialized - call buto_init_dispatch first"
}

# Non-fatal tabtarget resolution (returns 1 on failure instead of dying)
zbuto_resolve_tabtarget_capture() {
  zbuto_dispatch_sentinel

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || return 1

  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)

  # Bash 3.2: no-match glob returns literal — check with test -e
  test -e "${z_matches[0]}" || return 1
  test "${#z_matches[@]}" -eq 1 || return 1

  echo "${z_matches[0]}"
}

# buto_init_evidence() - Create evidence root dir under testbench temp
buto_init_evidence() {
  zbuto_dispatch_sentinel
  test -n "${BURD_TEMP_DIR:-}" || buto_fatal "BURD_TEMP_DIR not set - buto_init_evidence requires BURD context"

  ZBUTO_EVIDENCE_ROOT="${BURD_TEMP_DIR}/evidence"
  mkdir -p "${ZBUTO_EVIDENCE_ROOT}"
  buc_log_args "Evidence root: ${ZBUTO_EVIDENCE_ROOT}"
}

# buto_dispatch() - Invoke tabtarget via BURV-isolated environment
# Args: colophon [extra_args...]
# Step index available via buto_last_step_capture after return
buto_dispatch() {
  zbuto_dispatch_sentinel
  test -n "${ZBUTO_EVIDENCE_ROOT:-}" || buto_fatal "Evidence not initialized - call buto_init_evidence first"

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || buto_fatal "buto_dispatch requires colophon"
  shift

  local z_tabtarget
  z_tabtarget=$(zbuto_resolve_tabtarget_capture "${z_colophon}") || buto_fatal "Cannot resolve tabtarget for '${z_colophon}'"

  local z_step_idx="${#zbuto_step_colophons[@]}"

  local z_step_dir="${ZBUTO_EVIDENCE_ROOT}/step-${z_step_idx}"
  local z_burv_output="${z_step_dir}/burv-output"
  local z_burv_temp="${z_step_dir}/burv-temp"
  local z_evidence_dir="${z_step_dir}/evidence"
  mkdir -p "${z_burv_output}" "${z_burv_temp}" "${z_evidence_dir}"

  buc_log_args "Dispatching step ${z_step_idx}: colophon=${z_colophon} tabtarget=${z_tabtarget}"

  local z_exit_status=0
  BURV_OUTPUT_ROOT_DIR="${z_burv_output}" \
  BURV_TEMP_ROOT_DIR="${z_burv_temp}" \
  BURD_NO_LOG=1 \
    "${z_tabtarget}" "$@" || z_exit_status=$?

  buc_log_args "Step ${z_step_idx} exit status: ${z_exit_status}"

  if test -d "${z_burv_output}/current"; then
    cp -r "${z_burv_output}/current/." "${z_evidence_dir}/" || buc_warn "Evidence harvest failed for step ${z_step_idx}"
  fi

  zbuto_step_colophons+=("${z_colophon}")
  zbuto_step_exit_status+=("${z_exit_status}")
  zbuto_step_output_dir+=("${z_evidence_dir}")
}

######################################################################
# Step result _capture functions

buto_last_step_capture() {
  zbuto_dispatch_sentinel
  local z_count="${#zbuto_step_colophons[@]}"
  test "${z_count}" -gt 0 || return 1
  echo "$((z_count - 1))"
}

buto_get_step_exit_capture() {
  zbuto_dispatch_sentinel
  local z_idx="${1:-}"
  test -n "${z_idx}" || return 1
  echo "${zbuto_step_exit_status[$z_idx]}"
}

buto_get_step_output_capture() {
  zbuto_dispatch_sentinel
  local z_idx="${1:-}"
  test -n "${z_idx}" || return 1
  echo "${zbuto_step_output_dir[$z_idx]}"
}

# eof
