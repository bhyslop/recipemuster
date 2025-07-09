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

# Emit caller context information: filename:line in function
# Accepts one optional arg: stack depth (default=2)
zbtu_localize() {
  local frame="${1:-${ZBTU_STACK_DEPTH:-2}}"
  local file="${BASH_SOURCE[$frame]}"
  local line="${BASH_LINENO[$((frame - 1))]}"
  local func="${FUNCNAME[$frame]}"
  echo "$file($line): in $func()" >&2
}

# Verbosity-controlled trace
btu_trace() {
  test "${BTU_VERBOSE:-0}" -ge 1 || return 0
  echo "$@" >&2
  if test "${BTU_VERBOSE:-0}" -ge 2; then
    zbtu_localize 2
  fi
}

# Fatal error message and exit
btu_fatal() {
  echo "${ZBTU_RED}FATAL:${ZBTU_RESET} $1" >&2
  zbtu_localize 2
  shift
  for line in "$@"; do echo "$line" >&2; done
  exit 1
}


# Fatal if condition is true (non-zero)
btu_fatal_if() {
  local condition="$1"
  shift
  test "$condition" -ne 0 && ZBTU_STACK_DEPTH=3 btu_fatal "$@"
}

# Fatal unless condition is true (zero)
btu_fatal_unless() {
  local condition="$1"
  shift
  test "$condition" -eq 0 || ZBTU_STACK_DEPTH=3 btu_fatal "$@"
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
  local expected="$1"
  shift

  zbtu_invoke "$@"

  btu_fatal_if $ZBTU_STATUS "Command failed with status $ZBTU_STATUS" \
                            "Command: $*"                             \
                            "STDERR: $ZBTU_STDERR"

  test "$ZBTU_STDOUT" = "$expected" || btu_fatal "Output mismatch"       \
                                                 "Command: $*"           \
                                                 "Expected: '$expected'" \
                                                 "Got:      '$ZBTU_STDOUT'"
}

# Expect success (ignore stdout)
btu_expect_ok() {
  zbtu_invoke "$@"

  btu_fatal_if $ZBTU_STATUS "Command failed with status $ZBTU_STATUS" \
                            "Command: $*"                             \
                            "STDERR: $ZBTU_STDERR"
}

# Expect failure
btu_expect_fatal() {
  zbtu_invoke "$@"

  btu_fatal_unless $ZBTU_STATUS "Expected failure but got success" \
                                "Command: $*"                      \
                                "STDOUT: $ZBTU_STDOUT"             \
                                "STDERR: $ZBTU_STDERR"
}

# Run single test case in subshell
btu_case() {
  local test_name="$1"

  declare -F "$test_name" >/dev/null || btu_fatal "Test function not found: $test_name"
  btu_trace "Running: $test_name"

  (
    export BTU_VERBOSE="${BTU_VERBOSE:-0}"
    "$test_name"
  )
  local status=$?
  btu_fatal_if $status "Test failed: $test_name"

  btu_trace "Finished: $test_name with status: $status"
  test "${BTU_VERBOSE:-0}" -ge 1 && echo "${ZBTU_GREEN}PASSED:${ZBTU_RESET} $test_name" >&2
}

# Run all or specific tests
btu_execute() {
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
    btu_fatal_unless $found "No test functions found with prefix '$prefix'"
  fi

  test "${BTU_VERBOSE:-0}" -ge 1 && echo "${ZBTU_GREEN}All tests passed${ZBTU_RESET}" >&2
}

# eof
