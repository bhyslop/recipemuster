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
[[ -n "${ZBTU_INCLUDED:-}" ]] && return 0
ZBTU_INCLUDED=1

# Color codes
btu_color() { test -n "$TERM" && test "$TERM" != "dumb" && printf '\033[%sm' "$1" || printf ''; }
ZBTU_RED=$(    btu_color '1;31' )
ZBTU_GREEN=$(  btu_color '1;32' )
ZBTU_RESET=$(  btu_color '0'    )

# Generic renderer for aligned multi-line messages
# Usage: zbtu_render_lines PREFIX [COLOR] [STACK_DEPTH] LINES...
zbtu_render_lines() {
  local label="$1"; shift
  local color="$1"; shift
  local depth="$1"; shift

  local prefix file line indent
  if test "${BTU_VERBOSE:-0}" -eq 1; then
    prefix="$label:"
  else
    file="${BASH_SOURCE[$depth]}"
    line="${BASH_LINENO[$((depth - 1))]}"
    prefix="$label:$(basename "$file"):$line"
  fi

  local visible_prefix="$prefix"
  test -n "$color" && prefix="${color}${prefix}${ZBTU_RESET}"
  indent="$(printf '%*s' "$(echo -e "$visible_prefix" | sed 's/\x1b\[[0-9;]*m//g' | wc -c)" '')"

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

btu_trace() {
  test "${BTU_VERBOSE:-0}" -ge 1 || return 0
  zbtu_render_lines "trace" "" 2 "$@"
}

btu_fatal() {
  zbtu_render_lines "ERROR" "$ZBTU_RED" 2 "$@"
  exit 1
}

# Fatal if condition is true (non-zero)
btu_fatal_on_error() {
  set -e
  local condition="$1"
  shift
  test "$condition" -eq 0 || { ZBTU_STACK_DEPTH=3 btu_fatal "$@"; }
}

# Fatal unless condition is true (zero)
btu_fatal_on_success() {
  set -e
  local condition="$1"
  shift
  test "$condition" -ne 0 || { ZBTU_STACK_DEPTH=3 btu_fatal "$@"; }
}

# Safely invoke a command under 'set -e', capturing stdout, stderr, and exit status
# Globals set:
#   ZBTU_STDOUT  — command stdout
#   ZBTU_STDERR  — command stderr
#   ZBTU_STATUS  — command exit code
zbtu_invoke() {
  local tmp_stdout tmp_stderr
  tmp_stdout="$(mktemp)"
  tmp_stderr="$(mktemp)"

  set +e
  "$@" >"$tmp_stdout" 2>"$tmp_stderr"
  ZBTU_STATUS=$?
  set -e

  ZBTU_STDOUT=$(<"$tmp_stdout")
  ZBTU_STDERR=$(<"$tmp_stderr")

  rm -f "$tmp_stdout" "$tmp_stderr"
}

# Expect success and specific stdout
btu_expect_ok_stdout() {
  set -e

  local expected="$1"
  shift

  zbtu_invoke "$@"

  btu_fatal_on_error $ZBTU_STATUS "Command failed with status $ZBTU_STATUS" \
                            "Command: $*"                             \
                            "STDERR: $ZBTU_STDERR"

  test "$ZBTU_STDOUT" = "$expected" || btu_fatal "Output mismatch"       \
                                                 "Command: $*"           \
                                                 "Expected: '$expected'" \
                                                 "Got:      '$ZBTU_STDOUT'"
}

# Expect success (ignore stdout)
btu_expect_ok() {
  set -e

  zbtu_invoke "$@"

  btu_fatal_on_error $ZBTU_STATUS "Command failed with status $ZBTU_STATUS" \
                            "Command: $*"                             \
                            "STDERR: $ZBTU_STDERR"
}

# Expect failure
btu_expect_fatal() {
  set -e

  zbtu_invoke "$@"

  btu_fatal_on_success $ZBTU_STATUS "Expected failure but got success" \
                                "Command: $*"                      \
                                "STDOUT: $ZBTU_STDOUT"             \
                                "STDERR: $ZBTU_STDERR"
}

# Run single test case in subshell
btu_case() {
  set -e

  local test_name="$1"

  declare -F "$test_name" >/dev/null || btu_fatal "Test function not found: $test_name"
  btu_trace "Running: $test_name"

  (
    export BTU_VERBOSE="${BTU_VERBOSE:-0}"
    "$test_name"
  )
  local status=$?
  btu_fatal_on_error $status "Test failed: $test_name"

  btu_trace "Finished: $test_name with status: $status"
  test "${BTU_VERBOSE:-0}" -ge 1 && echo "${ZBTU_GREEN}PASSED:${ZBTU_RESET} $test_name" >&2
}

# Run all or specific tests
btu_execute() {
  set -e

  local prefix="$1"
  local specific_test="$2"

  if [ -n "$specific_test" ]; then
    echo "$specific_test" | grep -q "^${prefix}" || btu_fatal \
      "Test '$specific_test' does not start with required prefix '$prefix'"
    btu_case "$specific_test"
  else
    local found=0
    for test in $(declare -F | grep "^declare -f ${prefix}" | cut -d' ' -f3); do
      found=1
      btu_case "$test"
    done
    btu_fatal_on_success $found "No test functions found with prefix '$prefix'"
  fi

  test "${BTU_VERBOSE:-0}" -ge 1 && echo "${ZBTU_GREEN}All tests passed${ZBTU_RESET}" >&2
}


# eof
