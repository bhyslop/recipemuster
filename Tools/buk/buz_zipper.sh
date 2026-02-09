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
# BUK Zipper - Colophon registry via parallel arrays

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUZ_SOURCED:-}" || return 0
ZBUZ_SOURCED=1

######################################################################
# Internal kindle boilerplate

zbuz_kindle() {
  test -z "${ZBUZ_KINDLED:-}" || buc_die "buz already kindled"

  # Registry arrays (populated by coordinator kindle, same-process only)
  zbuz_colophons=()
  zbuz_modules=()
  zbuz_commands=()
  zbuz_tabtargets=()

  # Step result arrays (populated by buz_dispatch)
  zbuz_step_colophons=()
  zbuz_step_exit_status=()
  zbuz_step_output_dir=()

  ZBUZ_KINDLED=1
}

######################################################################
# Internal sentinel

zbuz_sentinel() {
  test "${ZBUZ_KINDLED:-}" = "1" || buc_die "Module buz not kindled - call zbuz_kindle first"
}

######################################################################
# Internal helpers

# zbuz_resolve_tabtarget_capture() - Resolve colophon to tabtarget path
# Args: colophon
# Returns: tabtarget path or exit 1
zbuz_resolve_tabtarget_capture() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || return 1

  local z_matches=("${BURC_TABTARGET_DIR}/${z_colophon}."*.sh)

  # Bash 3.2: no-match glob returns literal — check with test -e
  test -e "${z_matches[0]}" || return 1

  test "${#z_matches[@]}" -eq 1 || return 1

  echo "${z_matches[0]}"
}

######################################################################
# Public registry operations

# buz_register() - Register colophon tuple in parallel arrays
# Args: colophon, module, command
# Sets: z1z_buz_colophon (colophon string for caller to store as constant)
# Side effects: populates registry arrays (must be called in same process, NOT inside $())
buz_register() {
  zbuz_sentinel

  local z_colophon="${1:-}"
  local z_module="${2:-}"
  local z_command="${3:-}"
  test -n "${z_colophon}" || return 1
  test -n "${z_module}"   || return 1
  test -n "${z_command}"  || return 1

  # Validate tabtarget resolution (die on 0 or >1 matches)
  local z_tabtarget
  z_tabtarget=$(zbuz_resolve_tabtarget_capture "${z_colophon}") || buc_die "No unique tabtarget for colophon '${z_colophon}' in ${BURC_TABTARGET_DIR}/"

  # Registry population (only persists in same-process context, lost in $() subshell)
  zbuz_colophons+=("${z_colophon}")
  zbuz_modules+=("${z_module}")
  zbuz_commands+=("${z_command}")
  zbuz_tabtargets+=("${z_tabtarget}")

  # shellcheck disable=SC2034
  z1z_buz_colophon="${z_colophon}"
}

######################################################################
# Dispatch and evidence infrastructure

# buz_init_evidence() - Create evidence root dir under testbench temp
buz_init_evidence() {
  zbuz_sentinel
  test -n "${BURD_TEMP_DIR:-}" || buc_die "BURD_TEMP_DIR not set - buz_init_evidence requires BURD context"

  ZBUZ_EVIDENCE_ROOT="${BURD_TEMP_DIR}/evidence"
  mkdir -p "${ZBUZ_EVIDENCE_ROOT}"
  buc_log_args "Evidence root: ${ZBUZ_EVIDENCE_ROOT}"
}

# buz_dispatch() - Invoke tabtarget via BURV-isolated environment
# Args: colophon [extra_args...]
# NOT a _capture function — has side effects (step arrays, dirs)
# Step index available via buz_last_step_capture after return
buz_dispatch() {
  zbuz_sentinel
  test -n "${ZBUZ_EVIDENCE_ROOT:-}" || buc_die "Evidence not initialized - call buz_init_evidence first"

  local z_colophon="${1:-}"
  test -n "${z_colophon}" || buc_die "buz_dispatch requires colophon"
  shift

  # Resolve tabtarget on-demand
  local z_tabtarget
  z_tabtarget=$(zbuz_resolve_tabtarget_capture "${z_colophon}") || buc_die "Cannot resolve tabtarget for '${z_colophon}'"

  local z_step_idx="${#zbuz_step_colophons[@]}"

  local z_step_dir="${ZBUZ_EVIDENCE_ROOT}/step-${z_step_idx}"
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

  zbuz_step_colophons+=("${z_colophon}")
  zbuz_step_exit_status+=("${z_exit_status}")
  zbuz_step_output_dir+=("${z_evidence_dir}")
}

# buz_dispatch_expect_ok() - dispatch + die on non-zero exit
# Args: colophon [extra_args...]
buz_dispatch_expect_ok() {
  zbuz_sentinel

  buz_dispatch "$@"

  local z_step_idx
  z_step_idx=$(buz_last_step_capture) || buc_die "No step recorded after dispatch"

  local z_status="${zbuz_step_exit_status[$z_step_idx]}"
  if test "${z_status}" -ne 0; then
    buc_die "Dispatch '${zbuz_step_colophons[$z_step_idx]}' expected exit 0, got ${z_status}"
  fi
}

# buz_dispatch_expect_fail() - dispatch + die on zero exit
# Args: colophon [extra_args...]
buz_dispatch_expect_fail() {
  zbuz_sentinel

  buz_dispatch "$@"

  local z_step_idx
  z_step_idx=$(buz_last_step_capture) || buc_die "No step recorded after dispatch"

  local z_status="${zbuz_step_exit_status[$z_step_idx]}"
  if test "${z_status}" -eq 0; then
    buc_die "Dispatch '${zbuz_step_colophons[$z_step_idx]}' expected non-zero exit, got 0"
  fi
}

######################################################################
# Step result _capture functions

# buz_last_step_capture() - Return index of most recent step
buz_last_step_capture() {
  zbuz_sentinel
  local z_count="${#zbuz_step_colophons[@]}"
  test "${z_count}" -gt 0 || return 1
  echo "$((z_count - 1))"
}

# buz_get_step_exit_capture() - Return exit status for step
# Args: step_index
buz_get_step_exit_capture() {
  zbuz_sentinel
  local z_idx="${1:-}"
  test -n "${z_idx}" || return 1
  echo "${zbuz_step_exit_status[$z_idx]}"
}

# buz_get_step_output_capture() - Return evidence dir for step
# Args: step_index
buz_get_step_output_capture() {
  zbuz_sentinel
  local z_idx="${1:-}"
  test -n "${z_idx}" || return 1
  echo "${zbuz_step_output_dir[$z_idx]}"
}

# eof
