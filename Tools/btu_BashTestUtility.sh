#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# Bash Test Utility Library

# Multiple inclusion guard
test -z "${ZBTU_INCLUDED:-}" || return 0
ZBTU_INCLUDED=1

# Color codes
btu_color() { test -n "$TERM" && test "$TERM" != "dumb" && printf '\033[%sm' "$1" || printf ''; }
ZBTU_WHITE=$(  btu_color '1;37' )
ZBTU_RED=$(    btu_color '1;31' )
ZBTU_GREEN=$(  btu_color '1;32' )
ZBTU_RESET=$(  btu_color '0'    )

# Generic renderer for aligned multi-line messages
# Usage: zbtu_render_lines PREFIX [COLOR] [STACK_DEPTH] LINES...
zbtu_render_lines() {
  local label="$1"; shift
  local color="$1"; shift

  local prefix="$label:"

  local visible_prefix="$prefix"
  test -z "$color" || prefix="${color}${prefix}${ZBTU_RESET}"
  local indent="$(printf '%*s' "$(echo -e "$visible_prefix" | sed 's/\x1b\[[0-9;]*m//g' | wc -c)" '')"

  local first=1
  for line in "$@"; do
    if test $first -eq 1; then
      echo "$prefix $line" >&2
      first=0
    else
      echo "$indent $line" >&2
    fi
  done
}

btu_section() {
  test "${BTU_VERBOSE:-0}" -ge 1 || return 0
  zbtu_render_lines "info " "${ZBTU_WHITE}" "$@"
}

btu_info() {
  test "${BTU_VERBOSE:-0}" -ge 1 || return 0
  zbtu_render_lines "info " "" "$@"
}

btu_trace() {
  test "${BTU_VERBOSE:-0}" -ge 2 || return 0
  zbtu_render_lines "trace" "" "$@"
}

btu_fatal() {
  zbtu_render_lines "ERROR" "${ZBTU_RED}" "$@"
  exit 1
}

btu_fatal_on_error() {
  set -e
  local condition="$1"; shift
  test "${condition}" -eq 0 && return 0
  btu_fatal "$@"
}

btu_fatal_on_success() {
  set -e
  local condition="$1"; shift
  test "${condition}" -ne 0 && return 0
  btu_fatal "$@"
}

# Safely invoke a command under 'set -e', capturing stdout, stderr, and exit status
# Globals set:
#   ZBTU_STDOUT  — command stdout
#   ZBTU_STDERR  — command stderr
#   ZBTU_STATUS  — command exit code
zbtu_invoke() {
  btu_trace "Invoking: $*"

  local tmp_stdout="$(mktemp)"
  local tmp_stderr="$(mktemp)"

  ZBTU_STATUS=$( (
      set +e
      "$@" >"${tmp_stdout}" 2>"${tmp_stderr}"
      printf '%s' "$?"
      exit 0
    ) || printf '__subshell_failed__' )

  if [[ "${ZBTU_STATUS}" == "__subshell_failed__" || -z "${ZBTU_STATUS}" ]]; then
    ZBTU_STATUS=127
    ZBTU_STDOUT=""
    ZBTU_STDERR="zbtu_invoke: command caused shell to exit before status could be captured"
  else
    ZBTU_STDOUT=$(<"${tmp_stdout}")
    ZBTU_STDERR=$(<"${tmp_stderr}")
  fi

  rm -f "${tmp_stdout}" "${tmp_stderr}"
}

btu_expect_ok_stdout() {
  set -e

  local expected="$1"; shift

  zbtu_invoke "$@"

  btu_fatal_on_error "${ZBTU_STATUS}" "Command failed with status ${ZBTU_STATUS}" \
                                      "Command: $*"                               \
                                      "STDERR: ${ZBTU_STDERR}"

  test "${ZBTU_STDOUT}" = "${expected}" || btu_fatal "Output mismatch"            \
                                                     "Command: $*"                \
                                                     "Expected: '${expected}'"    \
                                                     "Got:      '${ZBTU_STDOUT}'"
}

btu_expect_ok() {
  set -e

  zbtu_invoke "$@"

  btu_fatal_on_error "${ZBTU_STATUS}" "Command failed with status ${ZBTU_STATUS}" \
                                      "Command: $*"                               \
                                      "STDERR: ${ZBTU_STDERR}"
}

btu_expect_fatal() {
  set -e

  zbtu_invoke "$@"

  btu_fatal_on_success "${ZBTU_STATUS}" "Expected failure but got success" \
                                        "Command: $*"                      \
                                        "STDOUT: ${ZBTU_STDOUT}"           \
                                        "STDERR: ${ZBTU_STDERR}"
}

# Run single test case in subshell
zbtu_case() {
  set -e

  local test_name="$1"
  declare -F "${test_name}" >/dev/null || btu_fatal "Test function not found: ${test_name}"

  btu_section "START: ${test_name}"

  local status
  (
    set -e
    "${test_name}"
  )

  status=$?
  btu_trace "Ran: ${test_name} and got status:${status}"
  btu_fatal_on_error "${status}" "Test failed: ${test_name}"

  btu_trace "Finished: ${test_name} with status: ${status}"
  test "${BTU_VERBOSE:-0}" -le 0 || echo "${ZBTU_GREEN}PASSED:${ZBTU_RESET} ${test_name}" >&2
}

# Run all or specific tests
btu_execute() {
  set -e

  export BTU_VERBOSE="${BTU_VERBOSE:-0}"

  # Enable bash trace to stderr if BTU_VERBOSE is 3 or higher and bash supports
  if [[ "${BTU_VERBOSE}" -ge 3 ]]; then
    if [[ "${BASH_VERSINFO[0]}" -gt 4 ]] || [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 1 ]]; then
      export PS4='+ ${BASH_SOURCE##*/}:${LINENO}: '
      export BASH_XTRACEFD=2
      set -x
    fi
  fi

  local prefix="$1"
  local specific_test="$2"
  local count=0

  if [[ -n "${specific_test}" ]]; then
    echo "${specific_test}" | grep -q "^${prefix}" || btu_fatal \
      "Test '${specific_test}' does not start with required prefix '${prefix}'"
    zbtu_case "${specific_test}"
    count=1
  else
    local found=0
    for one_case in $(declare -F | grep "^declare -f ${prefix}" | cut -d' ' -f3); do
      found=1
      zbtu_case "${one_case}"
      ((count++))
    done
    btu_fatal_on_success "${found}" "No test functions found with prefix '${prefix}'"
  fi

  echo "${ZBTU_GREEN}All tests passed (${count} case$(test ${count} -eq 1 || echo 's'))${ZBTU_RESET}" >&2
}

# eof

