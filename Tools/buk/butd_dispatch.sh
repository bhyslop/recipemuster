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
# BUK Test Dispatch - Suite runner and reporting

set -euo pipefail

# Source test engine
ZBUTD_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZBUTD_SCRIPT_DIR}/bute_engine.sh"

# Multiple inclusion guard
test -z "${ZBUTD_INCLUDED:-}" || buto_fatal "butd_dispatch multiply sourced"
ZBUTD_INCLUDED=1

######################################################################
# Suite execution

# butd_run_suite() - Run a single registered suite by name
# Args: suite_name
butd_run_suite() {
  local z_suite="${1:-}"
  if test -z "${z_suite}"; then
    buto_info "Available suites:"
    local z_suite_list
    z_suite_list=$(mktemp)
    butr_suites_recite > "${z_suite_list}"
    while IFS= read -r z_suite_name; do
      test -n "${z_suite_name}" || continue
      buto_info "  ${z_suite_name}"
    done < "${z_suite_list}"
    rm -f "${z_suite_list}"
    buto_fatal "butd_run_suite: suite_name required"
  fi

  local z_init
  z_init=$(butr_init_recite "${z_suite}") || buto_fatal "butd_run_suite: failed to get init for '${z_suite}'"
  local z_setup
  z_setup=$(butr_setup_recite "${z_suite}") || buto_fatal "butd_run_suite: failed to get setup for '${z_suite}'"

  buto_section "Suite: ${z_suite}"

  # Run init function if specified (status capture pattern)
  if test -n "${z_init}"; then
    buto_trace "Running init: ${z_init}"
    declare -F "${z_init}" >/dev/null || buto_fatal "Init function not found: ${z_init}"
    local z_status=0
    "${z_init}" || z_status=$?
    if test "${z_status}" -ne 0; then
      buc_warn "Suite '${z_suite}' not ready (init returned ${z_status}), skipping"
      return 2
    fi
  fi

  # Run setup function if specified
  if test -n "${z_setup}"; then
    buto_trace "Running setup: ${z_setup}"
    declare -F "${z_setup}" >/dev/null || buto_fatal "Setup function not found: ${z_setup}"
    "${z_setup}"
  fi

  # Create per-suite temp dir
  local z_suite_dir="${ZBUTE_ROOT_TEMP_DIR}/${z_suite}"
  mkdir -p "${z_suite_dir}"

  # Load case list into array before execution (BCG: load-then-iterate)
  # Prevents stdin consumption by test commands (docker exec -i, etc.)
  local z_cases=()
  local z_case_fn=""
  local z_case_count=0
  local z_cases_temp
  z_cases_temp=$(mktemp)
  butr_cases_recite "${z_suite}" > "${z_cases_temp}" || buto_fatal "Failed to get cases for suite '${z_suite}'"
  while IFS= read -r z_case_fn; do
    z_cases+=("${z_case_fn}")
  done < "${z_cases_temp}"
  rm -f "${z_cases_temp}"

  local z_ci
  for z_ci in "${!z_cases[@]}"; do
    test -n "${z_cases[$z_ci]}" || continue
    zbute_case "${z_cases[$z_ci]}"
    z_case_count=$((z_case_count + 1))
  done

  test "${z_case_count}" -gt 0 || buto_fatal "No test cases found for suite '${z_suite}'"

  echo "${ZBUTO_GREEN}Suite passed: ${z_suite} (${z_case_count} case$(test "${z_case_count}" -eq 1 || echo 's'))${ZBUTO_RESET}" >&2
}

# butd_run_one() - Run a single test function by name
# Args: function_name
butd_run_one() {
  local z_func="${1:-}"
  if test -z "${z_func}"; then
    buto_info "Available test functions:"
    local z_suites_list
    z_suites_list=$(mktemp)
    butr_suites_recite > "${z_suites_list}"
    while IFS= read -r z_suite_name; do
      test -n "${z_suite_name}" || continue
      local z_cases_list
      z_cases_list=$(mktemp)
      butr_cases_recite "${z_suite_name}" > "${z_cases_list}"
      while IFS= read -r z_case_fn; do
        test -n "${z_case_fn}" || continue
        buto_info "  ${z_suite_name}: ${z_case_fn}"
      done < "${z_cases_list}"
      rm -f "${z_cases_list}"
    done < "${z_suites_list}"
    rm -f "${z_suites_list}"
    buto_fatal "butd_run_one: function_name required"
  fi

  local z_suite
  z_suite=$(butr_suite_for_case_recite "${z_func}") || buto_fatal "butd_run_one: no suite matches function '${z_func}'"

  local z_init
  z_init=$(butr_init_recite "${z_suite}") || buto_fatal "butd_run_one: failed to get init for '${z_suite}'"
  local z_setup
  z_setup=$(butr_setup_recite "${z_suite}") || buto_fatal "butd_run_one: failed to get setup for '${z_suite}'"

  buto_section "Suite: ${z_suite} (single case: ${z_func})"

  # Run init function if specified (status capture pattern)
  if test -n "${z_init}"; then
    buto_trace "Running init: ${z_init}"
    declare -F "${z_init}" >/dev/null || buto_fatal "Init function not found: ${z_init}"
    local z_status=0
    "${z_init}" || z_status=$?
    if test "${z_status}" -ne 0; then
      buto_fatal "Suite '${z_suite}' not ready (init returned ${z_status})"
    fi
  fi

  # Run setup function if specified
  if test -n "${z_setup}"; then
    buto_trace "Running setup: ${z_setup}"
    declare -F "${z_setup}" >/dev/null || buto_fatal "Setup function not found: ${z_setup}"
    "${z_setup}"
  fi

  # Create per-suite temp dir
  local z_suite_dir="${ZBUTE_ROOT_TEMP_DIR}/${z_suite}"
  mkdir -p "${z_suite_dir}"

  # Run the single case
  zbute_case "${z_func}"

  echo "${ZBUTO_GREEN}Test passed: ${z_func}${ZBUTO_RESET}" >&2
}

# butd_run_all() - Run all registered suites
# Args: none
butd_run_all() {
  local z_total_suites=0
  local z_total_skipped=0
  local z_total_failed=0
  local z_suite=""
  local z_suites_temp
  z_suites_temp=$(mktemp)

  # Load suite list into array before execution (BCG: load-then-iterate)
  # Prevents stdin consumption by suite test commands
  local z_suites=()
  butr_suites_recite > "${z_suites_temp}" || buto_fatal "Failed to get suites"
  while IFS= read -r z_suite; do
    z_suites+=("${z_suite}")
  done < "${z_suites_temp}"
  rm -f "${z_suites_temp}"

  local z_si
  for z_si in "${!z_suites[@]}"; do
    z_suite="${z_suites[$z_si]}"
    test -n "${z_suite}" || continue

    local z_suite_status=0
    butd_run_suite "${z_suite}" || z_suite_status=$?

    if test "${z_suite_status}" -eq 0; then
      z_total_suites=$((z_total_suites + 1))
    elif test "${z_suite_status}" -eq 2; then
      z_total_skipped=$((z_total_skipped + 1))
    else
      z_total_failed=$((z_total_failed + 1))
      buc_warn "Suite '${z_suite}' failed with status ${z_suite_status}"
    fi
  done

  local z_total_ran=$((z_total_suites + z_total_failed))
  test "${z_total_ran}" -gt 0 || buto_fatal "No suites ran (${z_total_skipped} skipped)"

  if test "${z_total_failed}" -gt 0; then
    echo "${ZBUTO_RED}Some suites failed: ${z_total_suites} passed, ${z_total_failed} failed, ${z_total_skipped} skipped${ZBUTO_RESET}" >&2
    exit 1
  fi

  local z_skip_note=""
  test "${z_total_skipped}" -eq 0 || z_skip_note=", ${z_total_skipped} skipped"

  echo "${ZBUTO_GREEN}All suites passed (${z_total_suites} suite$(test "${z_total_suites}" -eq 1 || echo 's')${z_skip_note})${ZBUTO_RESET}" >&2
}

# eof
