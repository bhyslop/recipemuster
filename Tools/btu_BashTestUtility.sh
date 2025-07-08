#!/bin/bash
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

# Source the console utility library
ZBTU_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZBTU_SCRIPT_DIR}/bcu_BashConsoleUtility.sh"

# Print error and return failure
btu_fail() {
    bcu_context "${ZBTU_CONTEXT:-TEST}"
    echo -e "${ZBCU_RED}FAIL:${ZBCU_RESET} $1"
    shift
    while [ $# -gt 0 ]; do
        echo "$1"
        shift
    done
    return 1
}

# Trace function - respects BCU_VERBOSE
btu_trace() {
    test "${BCU_VERBOSE:-0}" -ge 1 && echo "$@"
}

# Run command in subshell, expect success and specific stdout
btu_expect_ok_stdout() {
    local expected="$1"
    shift
    
    # Run in subshell, capture output and status
    local output
    local status
    output=$("$@" 2>&1)
    status=$?
    
    if [ $status -ne 0 ]; then
        btu_fail "Command failed with status $status" \
                 "Command: $*" \
                 "Output: $output"
    fi
    
    if [ "$output" != "$expected" ]; then
        btu_fail "Output mismatch" \
                 "Command: $*" \
                 "Expected: '$expected'" \
                 "Got:      '$output'"
    fi
    
    return 0
}

# Run command in subshell, expect failure
btu_expect_die() {
    # Run in subshell, capture status
    local output
    local status
    output=$("$@" 2>&1)
    status=$?
    
    if [ $status -eq 0 ]; then
        btu_fail "Expected failure but got success" \
                 "Command: $*" \
                 "Output: $output"
    fi
    
    return 0
}

# Run single test case in clean subshell
btu_case() {
    local test_name="$1"
    
    # Check if function exists
    if ! declare -F "$test_name" >/dev/null; then
        bcu_die "Test function not found: $test_name"
    fi
    
    btu_trace "Running: $test_name"
    
    # Run test in subshell with BCU_VERBOSE passed through
    (
        export BCU_VERBOSE="${BCU_VERBOSE:-0}"
        test "${BCU_VERBOSE:-0}" -ge 2 && set -x
        "$test_name"
    )
    local status=$?
    
    if [ $status -ne 0 ]; then
        bcu_context "$test_name"
        bcu_die "Test failed"
    fi
    
    test "${BCU_VERBOSE:-0}" -ge 1 && bcu_pass "PASSED: $test_name"
    return 0
}

# Main test executor
btu_execute() {
    local prefix="$1"
    local specific_test="$2"
    
    if [ -n "$specific_test" ]; then
        # Run specific test
        if ! echo "$specific_test" | grep -q "^${prefix}"; then
            bcu_die "Test '$specific_test' does not start with required prefix '$prefix'"
        fi
        btu_case "$specific_test"
    else
        # Run all tests with prefix
        local found=0
        for test in $(declare -F | grep "^declare -f ${prefix}" | cut -d' ' -f3); do
            found=1
            btu_case "$test"
        done
        
        if [ $found -eq 0 ]; then
            bcu_die "No test functions found with prefix '$prefix'"
        fi
    fi
    
    test "${BCU_VERBOSE:-0}" -ge 1 && bcu_pass "All tests passed"
}

# eof

