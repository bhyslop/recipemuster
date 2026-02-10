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

# Multiple inclusion guard
test -z "${ZBUTD_INCLUDED:-}" || buto_fatal "butd_dispatch multiply sourced"
ZBUTD_INCLUDED=1

######################################################################
# Suite execution

# butd_run_suite() - Run a single registered suite by name
# Args: suite_name [single_test_function]
butd_run_suite() {
  local z_suite="${1:-}"
  test -n "${z_suite}" || buto_fatal "butd_run_suite: suite_name required"
  shift
  local z_single="${1:-}"

  local z_glob
  z_glob=$(butr_get_glob "${z_suite}") || buto_fatal "butd_run_suite: unknown suite '${z_suite}'"
  local z_setup
  z_setup=$(butr_get_setup "${z_suite}") || buto_fatal "butd_run_suite: failed to get setup for '${z_suite}'"

  buto_section "Suite: ${z_suite} (glob: ${z_glob})"

  # Run setup function if specified
  if test -n "${z_setup}"; then
    buto_trace "Running setup: ${z_setup}"
    declare -F "${z_setup}" >/dev/null || buto_fatal "Setup function not found: ${z_setup}"
    "${z_setup}"
  fi

  # Create per-suite temp dir
  local z_suite_dir="${ZBUTO_ROOT_TEMP_DIR}/${z_suite}"
  mkdir -p "${z_suite_dir}"

  # Delegate to buto_execute for test discovery and execution
  buto_execute "${z_suite_dir}" "${z_glob}" "${z_single}"
}

# butd_run_one() - Run a single test function by name
# Args: function_name
butd_run_one() {
  local z_func="${1:-}"
  test -n "${z_func}" || buto_fatal "butd_run_one: function_name required"

  local z_suite=""
  local z_glob=""
  local z_found=""
  for z_suite in $(butr_get_suites); do
    z_glob=$(butr_get_glob "${z_suite}") || continue
    case "${z_func}" in
      ${z_glob}*) z_found="${z_suite}"; break ;;
    esac
  done

  test -n "${z_found}" || buto_fatal "butd_run_one: no suite matches function '${z_func}'"

  butd_run_suite "${z_found}" "${z_func}"
}

# butd_run_all() - Run all registered suites
# Args: [--fast|--slow] (optional tier filter)
butd_run_all() {
  local z_tier_filter=""

  case "${1:-}" in
    --fast) z_tier_filter="fast" ;;
    --slow) z_tier_filter="slow" ;;
    "")     ;;
    *)      buto_fatal "butd_run_all: unknown flag '${1}'" ;;
  esac

  local z_total_suites=0
  local z_suite=""
  local z_tier=""

  for z_suite in $(butr_get_suites); do
    if test -n "${z_tier_filter}"; then
      z_tier=$(butr_get_tier "${z_suite}") || continue
      test "${z_tier}" = "${z_tier_filter}" || continue
    fi

    butd_run_suite "${z_suite}"
    z_total_suites=$((z_total_suites + 1))
  done

  test "${z_total_suites}" -gt 0 || buto_fatal "No suites matched${z_tier_filter:+ tier '${z_tier_filter}'}"

  echo "${ZBUTO_GREEN}All suites passed (${z_total_suites} suite$(test ${z_total_suites} -eq 1 || echo 's'))${ZBUTO_RESET}" >&2
}

# eof
