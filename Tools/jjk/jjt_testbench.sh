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
# JJT Testbench - Job Jockey test execution
#
# NOTE: Gallops JSON operations are now tested in Rust.
# Run: cd Tools/jjk/veiled && cargo test
#
# This testbench provides minimal bash wrapper verification only.

set -euo pipefail

# Get script directory
JJT_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${JJT_SCRIPT_DIR}/../buk/buc_command.sh"
source "${JJT_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${JJT_SCRIPT_DIR}/../buk/but_test.sh"
source "${JJT_SCRIPT_DIR}/jju_utility.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Kindle JJU (assumes BUD environment is already set by dispatcher)
zjju_kindle

######################################################################
# Test: Verify vvx is available

jjt_test_vvx_available() {
  but_section "=== VVX Availability Test ==="

  but_info "Test: vvx binary is available"
  if test -n "${ZJJU_VVX_BIN:-}"; then
    but_expect_ok "${ZJJU_VVX_BIN}" --version
    but_info "vvx found at: ${ZJJU_VVX_BIN}"
  else
    but_fatal "vvx binary not found"
  fi

  but_section "=== VVX availability verified ==="
}

######################################################################
# Test: Wrapper functions call vvx without error

jjt_test_wrapper_help() {
  but_section "=== Wrapper Help Tests ==="

  # These test that wrappers produce doc output when called without args
  # (via buc_doc_shown returning early)

  but_info "Test: jju_muster shows doc"
  but_expect_ok jju_muster

  but_info "Test: jju_saddle shows doc when no args"
  but_expect_ok jju_saddle

  but_info "Test: jju_nominate shows doc when no args"
  but_expect_ok jju_nominate

  but_section "=== Wrapper help tests passed ==="
}

######################################################################
# Main

# When called via BUD dispatch: $1=tabtarget-stem, $2=suite-name
# When called directly: $1=suite-name
z_suite="${2:-${1:-}}"

case "${z_suite}" in
  vvx)
    jjt_test_vvx_available
    ;;
  wrappers)
    jjt_test_wrapper_help
    ;;
  all)
    jjt_test_vvx_available
    jjt_test_wrapper_help
    ;;
  rust)
    echo "Running Rust tests..."
    cd "${JJT_SCRIPT_DIR}/veiled" && cargo test
    ;;
  *)
    echo "JJT Testbench - Job Jockey test execution"
    echo ""
    echo "Usage: ${0##*/} <suite>"
    echo ""
    echo "Test Suites:"
    echo "  vvx       Verify vvx binary is available"
    echo "  wrappers  Test bash wrapper functions"
    echo "  rust      Run Rust tests (cargo test)"
    echo "  all       Run bash tests (vvx + wrappers)"
    echo ""
    echo "NOTE: Gallops JSON operations are tested in Rust."
    echo "      Run 'cargo test' in Tools/jjk/veiled/ for full coverage."
    exit 1
    ;;
esac

# eof
