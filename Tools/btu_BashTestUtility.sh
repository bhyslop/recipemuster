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

# ============================================================================
# Multiple inclusion guard
# ----------------------------------------------------------------------------
# A simple, POSIX?compatible header?guard to prevent accidental re?sourcing.
# ============================================================================

test -z "${ZBTU_INCLUDED:-}" || return 0
ZBTU_INCLUDED=1

# ============================================================================
# Colours
# ----------------------------------------------------------------------------
# These are only emitted if stdout is a TTY and TERM is not "dumb" so that the
# library degrades gracefully in scripts and CI logs.
# ============================================================================

btu_color() {
  test -t 1 && test "${TERM:-}" != "dumb" && printf '\033[%sm' "$1" || printf ''
}

ZBTU_WHITE=$(  btu_color '1;37' )
ZBTU_RED=$(    btu_color '1;31' )
ZBTU_GREEN=$(  btu_color '1;32' )
ZBTU_RESET=$(  btu_color '0'    )

# ============================================================================
# Lightweight call?stack tracker
# ----------------------------------------------------------------------------
#   * zbtu_stack_push – records the current file:line into $ZBTU_LOC if empty
#   * zbtu_stack_pop  – clears $ZBTU_LOC when the matching handle is popped
#
#  NOTE: These functions *must* run in the current shell.  Do **not** wrap
#        them in $( ) or back?ticks, otherwise they execute in a subshell and
#        the global variable is lost.  Call them directly, then capture the
#        resulting value from $ZBTU_LOC.
# ============================================================================

zbtu_stack_push() {
  local file="${BASH_SOURCE[0]}"
  local line="${BASH_LINENO[1]}"
  local ident="${file}:${line}"

  echo "BRADTRACE: PUSH ZBTU_LOC:(${ZBTU_LOC}) ident:(${ident})" >&2
  if [[ -z "${ZBTU_LOC}" ]]; then
    export ZBTU_LOC="${ident}"
    echo "BRADTRACE: SET LOCALE ${ident} is ${ZBTU_LOC}" >&2
  else
    echo "BRADTRACE: skipped locale ${ident}" >&2
  fi

  # The caller captures this from $ZBTU_LOC, so also return it for convenience.
  printf '%s\n' "${ident}"
}

zbtu_stack_pop() {
  local handle="$1"
  echo "BRADTRACE: POP  ZBTU_LOC:(${ZBTU_LOC}) handle:(${handle})" >&2
  if [[ "${handle}" == "${ZBTU_LOC}" ]]; then
    unset ZBTU_LOC
    echo "BRADTRACE: CLEARED LOCALE ${handle}" >&2
  else
    echo "BRADTRACE: uncleared locale ${handle}" >&2
  fi
}

# ============================================================================
# Pretty printing helpers
# ============================================================================

# Render aligned, possibly coloured, multi?line messages to stderr.
# Usage: zbtu_render_lines PREFIX [COLOUR] LINES…
zbtu_render_lines() {
  local label="$1"; shift
  local colour="$1"; shift

  local prefix visible_prefix indent line file frame
  frame="${ZBTU_STACK_DEPTH:-2}"
  file="${BASH_SOURCE[${frame}]}"
  line="${BASH_LINENO[${frame}]}"

  if test "${BTU_VERBOSE:-0}" -eq 1; then
    prefix="${label}:"
  else
    prefix="${label}:$(basename "${file}"):${line}"
  fi

  visible_prefix="${prefix}"
  test -z "${colour}" || prefix="${colour}${prefix}${ZBTU_RESET}"
  indent="$(printf '%*s' "$(echo -e "${visible_prefix}" | sed 's/\x1b\[[0-9;]*m//g' | wc -c)" '')"

  local first=1
  for line in "$@"; do
    if test ${first} -eq 1; then
      echo "${prefix} ${line}" >&2
      first=0
    else
      echo "${indent} ${line}" >&2
    fi
  done
}

# ============================================================================
# Public logging helpers (section/info/trace/fatal)
# ============================================================================

btu_section() {
  test "${BTU_VERBOSE:-0}" -ge 1 || return 0
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"
  zbtu_render_lines "info " "${ZBTU_WHITE}" "$@"
  zbtu_stack_pop "${zbtu_token}"
}

btu_info() {
  test "${BTU_VERBOSE:-0}" -ge 1 || return 0
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"
  zbtu_render_lines "info " "" "$@"
  zbtu_stack_pop "${zbtu_token}"
}

btu_trace() {
  test "${BTU_VERBOSE:-0}" -ge 2 || return 0
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"
  zbtu_render_lines "trace" "" "$@"
  zbtu_stack_pop "${zbtu_token}"
}

btu_fatal() {
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"
  zbtu_render_lines "ERROR" "${ZBTU_RED}" "$@"
  zbtu_stack_pop "${zbtu_token}"
  exit 1
}

btu_fatal_on_error() {
  set -e
  local condition="$1"; shift
  test "${condition}" -eq 0 && return 0
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"
  btu_fatal "$@"
}

btu_fatal_on_success() {
  set -e
  local condition="$1"; shift
  test "${condition}" -ne 0 && return 0
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"
  btu_fatal "$@"
}

# ============================================================================
# Command invocation helpers
# ============================================================================
#   zbtu_invoke – runs a command under set ?e, capturing stdout/stderr/status
# ============================================================================

zbtu_invoke() {
  btu_trace "Invoking: $*"

  local tmp_stdout tmp_stderr
  tmp_stdout="$(mktemp)"
  tmp_stderr="$(mktemp)"

  ZBTU_STATUS=$( (
      set +e
      export ZBTU_LOC="${ZBTU_LOC}"
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

# ============================================================================
# Expectations helpers
# ============================================================================

btu_expect_ok_stdout() {
  set -e
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"

  local expected="$1"; shift

  zbtu_invoke "$@"

  btu_fatal_on_error "${ZBTU_STATUS}" "Command failed with status ${ZBTU_STATUS}" \
                                   "Command: $*"                             \
                                   "STDERR: ${ZBTU_STDERR}"

  test "${ZBTU_STDOUT}" = "${expected}" || btu_fatal "Output mismatch"       \
                                                 "Command: $*"           \
                                                 "Expected: '${expected}'" \
                                                 "Got:      '${ZBTU_STDOUT}'"

  zbtu_stack_pop "${zbtu_token}"
}

btu_expect_ok() {
  set -e
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"

  zbtu_invoke "$@"

  btu_fatal_on_error "${ZBTU_STATUS}" "Command failed with status ${ZBTU_STATUS}" \
                                   "Command: $*"                             \
                                   "STDERR: ${ZBTU_STDERR}"

  zbtu_stack_pop "${zbtu_token}"
}

btu_expect_fatal() {
  set -e
  zbtu_stack_push; local zbtu_token="${ZBTU_LOC}"

  zbtu_invoke "$@"

  btu_fatal_on_success "${ZBTU_STATUS}" "Expected failure but got success" \
                                     "Command: $*"                      \
                                     "STDOUT: ${ZBTU_STDOUT}"             \
                                     "STDERR: ${ZBTU_STDERR}"

  zbtu_stack_pop "${zbtu_token}"
}

# ============================================================================
# Test?case harness
# ============================================================================

# Run a single test case in a subshell so that 'set -e' only affects it.
zbtu_case() {
  set -e

  local test_name="$1"
  declare -F "${test_name}" >/dev/null || btu_fatal "Test function not found: ${test_name}"
  btu_section "START: ${test_name}"

  local status
  (
    set -e
    export ZBTU_LOC="${ZBTU_LOC}"
    "${test_name}"
  )
  status=$?
  btu_trace "Ran: ${test_name} and got status:${status}"
  btu_fatal_on_error "${status}" "Test failed: ${test_name}"

  btu_trace "Finished: ${test_name} with status: ${status}"
  test "${BTU_VERBOSE:-0}" -le 0 || echo "${ZBTU_GREEN}PASSED:${ZBTU_RESET} ${test_name}" >&2
}

# Discover and run test cases with the given prefix (or a specific test name).
btu_execute() {
  set -e

  export BTU_VERBOSE="${BTU_VERBOSE:-0}"

  local prefix="$1"
  local specific_test="$2"

  if [[ -n "${specific_test}" ]]; then
    echo "${specific_test}" | grep -q "^${prefix}" || btu_fatal \
      "Test '${specific_test}' does not start with required prefix '${prefix}'"
    zbtu_case "${specific_test}"
  else
    local found=0
    for one_case in $(declare -F | grep "^declare -f ${prefix}" | cut -d' ' -f3); do
      found=1
      zbtu_case "${one_case}"
    done
    btu_fatal_on_success "${found}" "No test functions found with prefix '${prefix}'"
  fi

  echo "${ZBTU_GREEN}All tests passed${ZBTU_RESET}" >&2
}

# eof
