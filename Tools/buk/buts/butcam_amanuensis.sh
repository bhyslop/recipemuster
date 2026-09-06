#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# BUTCAM - amanuensis-mode test cases for BUK self-test
#
# Proves the third log mode of the Log Family Law: the dispatch composes
# the log family's three names, exports them across the exec boundary, creates
# no file and tees nothing, so the coordinator writes the record itself.
#
# Each case drives a whole sub-dispatch of its own, because the mode cannot be
# observed from inside the testbench's own dispatch — that one is logged, and a
# dispatch declares its mode before any case runs.  The sub-dispatch is
# isolated onto BURV roots (the composed inlet, innermost wins), so the outer
# dispatch's log directory, output directory and temp directory are untouched.
#
# The logged control beside the flagged case is not decoration.  The flagged
# case asserts an ABSENCE — that no member was created — and an absence reads
# green whether the mode worked or the probe looked in the wrong directory.
# The control drives the same probe through the same helper with the flag
# empty and asserts all three members appear.
#
# All tests are pure local — no GCP, no containers, no network.

set -euo pipefail

######################################################################
# Fixture

# Write a coordinator reporting the dispatch environment it was handed.
zbutcam_write_probe() {
  local -r z_path="${1}"
  local -r z_report="${2}"
  printf '%s\n'                                                \
    '#!/bin/bash'                                              \
    '{'                                                        \
    '  printf "zbutcam_last=\"%s\"\n" "${BURD_LOG_LAST:-}"'    \
    '  printf "zbutcam_same=\"%s\"\n" "${BURD_LOG_SAME:-}"'    \
    '  printf "zbutcam_hist=\"%s\"\n" "${BURD_LOG_HIST:-}"'    \
    '  printf "zbutcam_temp=\"%s\"\n" "${BURD_TEMP_DIR:-}"'    \
    "} > \"${z_report}\""                                      \
    'echo "butcam probe coordinator ran"'                      \
    > "${z_path}"
  chmod +x "${z_path}"
}

# Drive one sub-dispatch. The three mode values are passed on every call, empty
# standing for a flag the tabtarget does not carry — which is how the dispatch
# itself reads them, so an empty value exercises the same branch an absent one
# would.  Sets zbutcam_log_dir and zbutcam_report for the caller, and leaves
# zbuto_invoke's own capture globals standing.
zbutcam_drive() {
  local -r z_tag="${1}"
  local -r z_amanuensis="${2}"
  local -r z_no_log="${3}"
  local -r z_interactive="${4}"

  zbutcam_log_dir="${BUT_TEMP_DIR}/${z_tag}-logs"
  zbutcam_report="${BUT_TEMP_DIR}/${z_tag}-report.env"
  local -r z_probe="${BUT_TEMP_DIR}/${z_tag}-coordinator.sh"

  mkdir -p "${zbutcam_log_dir}"
  zbutcam_write_probe "${z_probe}" "${zbutcam_report}"

  zbuto_invoke env                                 \
    "BURV_LOG_DIR=${zbutcam_log_dir}"              \
    "BURD_COORDINATOR_SCRIPT=${z_probe}"           \
    "BURD_AMANUENSIS=${z_amanuensis}"              \
    "BURD_NO_LOG=${z_no_log}"                      \
    "BURD_INTERACTIVE=${z_interactive}"            \
    bash "${BURD_BUK_DIR}/bud_dispatch.sh" "butcam-probe.LogFamily.sh"
}

# Count the members of the log family standing in the driven log directory.
zbutcam_member_count() {
  local z_count=0
  local z_entry
  for z_entry in "${zbutcam_log_dir}"/*; do
    test -e "${z_entry}" || continue
    z_count=$((z_count + 1))
  done
  printf '%s' "${z_count}"
}

# Assert a substring stands in the refusal, naming what was sought when absent.
zbutcam_stderr_carries() {
  local -r z_needle="${1}"
  local -r z_what="${2}"
  case "${ZBUTO_STDERR}" in
    *"${z_needle}"*) return 0 ;;
  esac
  buto_fatal_now "Refusal does not state ${z_what}"  \
                 "Sought: ${z_needle}"               \
                 "STDERR: ${ZBUTO_STDERR}"
}

######################################################################
# Cases

butcam_exports_the_family_tcase() {
  buto_trace "Amanuensis-mode: the coordinator's environment carries all three log paths"

  zbutcam_drive "exports" 1 "" ""
  test "${ZBUTO_STATUS}" -eq 0 || buto_fatal_now "Sub-dispatch failed (status ${ZBUTO_STATUS})" \
                                                 "STDERR: ${ZBUTO_STDERR}"
  test -f "${zbutcam_report}" || buto_fatal_now "Probe coordinator wrote no report"
  source "${zbutcam_report}"

  test -n "${zbutcam_last:-}" || buto_fatal_now "BURD_LOG_LAST did not reach the coordinator"
  test -n "${zbutcam_same:-}" || buto_fatal_now "BURD_LOG_SAME did not reach the coordinator"
  test -n "${zbutcam_hist:-}" || buto_fatal_now "BURD_LOG_HIST did not reach the coordinator"

  # The names are the dispatch's: each must stand under the directory this
  # dispatch was pointed at, never one the coordinator could have composed.
  local z_name
  for z_name in "${zbutcam_last}" "${zbutcam_same}" "${zbutcam_hist}"; do
    case "${z_name}" in
      "${zbutcam_log_dir}"/*) ;;
      *) buto_fatal_now "Handed path stands outside the dispatch's log directory" \
                        "Path: ${z_name}"                                         \
                        "Directory: ${zbutcam_log_dir}" ;;
    esac
  done
}

butcam_writes_no_member_tcase() {
  buto_trace "Amanuensis-mode: the dispatch creates the log directory and none of its members"

  zbutcam_drive "writes-none" 1 "" ""
  test "${ZBUTO_STATUS}" -eq 0 || buto_fatal_now "Sub-dispatch failed (status ${ZBUTO_STATUS})" \
                                                 "STDERR: ${ZBUTO_STDERR}"

  test -d "${zbutcam_log_dir}" || buto_fatal_now "Log directory absent: ${zbutcam_log_dir}"

  local -r z_count=$(zbutcam_member_count)
  test "${z_count}" -eq 0 \
    || buto_fatal_now "Dispatch created ${z_count} log member(s) where it owed none" \
                      "Directory: ${zbutcam_log_dir}"
}

butcam_logged_control_writes_all_tcase() {
  buto_trace "Amanuensis-mode control: the same drive with the flag empty writes all three members"

  zbutcam_drive "logged-control" "" "" ""
  test "${ZBUTO_STATUS}" -eq 0 || buto_fatal_now "Sub-dispatch failed (status ${ZBUTO_STATUS})" \
                                                 "STDERR: ${ZBUTO_STDERR}"

  local -r z_count=$(zbutcam_member_count)
  test "${z_count}" -eq 3 \
    || buto_fatal_now "A logged dispatch wrote ${z_count} log member(s), expected 3" \
                      "The absence the flagged case asserts is unproven without this" \
                      "Directory: ${zbutcam_log_dir}"
}

butcam_burx_carries_hist_tcase() {
  buto_trace "Amanuensis-mode: burx.env carries the historical member's path, as under a logged dispatch"

  zbutcam_drive "burx" 1 "" ""
  test "${ZBUTO_STATUS}" -eq 0 || buto_fatal_now "Sub-dispatch failed (status ${ZBUTO_STATUS})" \
                                                 "STDERR: ${ZBUTO_STDERR}"
  test -f "${zbutcam_report}" || buto_fatal_now "Probe coordinator wrote no report"
  source "${zbutcam_report}"

  test -n "${zbutcam_temp:-}" || buto_fatal_now "BURD_TEMP_DIR did not reach the coordinator"
  local -r z_burx="${zbutcam_temp}/${BUF_burx_env}"
  test -f "${z_burx}" || buto_fatal_now "No fact file in the sub-dispatch's temp dir: ${z_burx}"

  source "${z_burx}"
  test -n "${BURX_LOG_HIST:-}" \
    || buto_fatal_now "BURX_LOG_HIST is empty where a path was composed; empty reads as no-log"
  test "${BURX_LOG_HIST}" = "${zbutcam_hist}" \
    || buto_fatal_now "BURX_LOG_HIST names a different path than the coordinator was handed" \
                      "Fact file: ${BURX_LOG_HIST}"                                          \
                      "Coordinator: ${zbutcam_hist}"
}

butcam_refuses_beside_no_log_tcase() {
  buto_trace "Amanuensis-mode: the mode refuses to stand beside no-log, before the coordinator runs"

  zbutcam_drive "with-no-log" 1 1 ""
  test "${ZBUTO_STATUS}" -ne 0 || buto_fatal_now "Dispatch admitted BURD_AMANUENSIS beside BURD_NO_LOG"
  test ! -f "${zbutcam_report}" || buto_fatal_now "The coordinator ran despite the refusal"

  zbutcam_stderr_carries "BURD_AMANUENSIS" "the mode flag it refused"
  zbutcam_stderr_carries "BURD_NO_LOG"     "the flag it refused to stand beside"
}

butcam_refuses_beside_interactive_tcase() {
  buto_trace "Amanuensis-mode: the mode refuses to stand beside interactive, before the coordinator runs"

  zbutcam_drive "with-interactive" 1 "" 1
  test "${ZBUTO_STATUS}" -ne 0 || buto_fatal_now "Dispatch admitted BURD_AMANUENSIS beside BURD_INTERACTIVE"
  test ! -f "${zbutcam_report}" || buto_fatal_now "The coordinator ran despite the refusal"

  zbutcam_stderr_carries "BURD_AMANUENSIS"  "the mode flag it refused"
  zbutcam_stderr_carries "BURD_INTERACTIVE" "the flag it refused to stand beside"
}

# eof
