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
# Test Wrappers
#
# These wrap internal zjju_* functions for testing

jjt_favor_encode() {
  zjju_favor_encode "$@"
}

jjt_favor_decode() {
  zjju_favor_decode "$@"
}

jjt_studbook_validate() {
  zjju_studbook_validate "$@"
}

######################################################################
# Test Suites

jjt_test_favor_encoding() {
  but_section "=== Favor Encoding Tests ==="

  # Test 1: Minimum values
  but_info "Test 1: heat=0, pace=0 (minimum)"
  but_expect_ok_stdout "AAAAA" jjt_favor_encode 0 0
  but_expect_ok_stdout "0	0" jjt_favor_decode "AAAAA"

  # Test 2: Heat-only reference (heat=667, pace=0)
  but_info "Test 2: heat=667, pace=0 (heat-only reference)"
  but_expect_ok_stdout "KbAAA" jjt_favor_encode 667 0
  but_expect_ok_stdout "667	0" jjt_favor_decode "KbAAA"

  # Test 3: Heat 667, Pace 1
  but_info "Test 3: heat=667, pace=1"
  but_expect_ok_stdout "KbAAB" jjt_favor_encode 667 1
  but_expect_ok_stdout "667	1" jjt_favor_decode "KbAAB"

  # Test 4: Heat 667, Pace 2
  but_info "Test 4: heat=667, pace=2"
  but_expect_ok_stdout "KbAAC" jjt_favor_encode 667 2
  but_expect_ok_stdout "667	2" jjt_favor_decode "KbAAC"

  # Test 5: Maximum values
  but_info "Test 5: heat=4095, pace=262143 (maximum)"
  but_expect_ok_stdout "_____" jjt_favor_encode 4095 262143
  but_expect_ok_stdout "4095	262143" jjt_favor_decode "_____"

  # Test 6: Round-trip multiple values
  but_info "Test 6: Round-trip various values"
  local z_favor
  z_favor=$(jjt_favor_encode 100 500)
  but_expect_ok_stdout "100	500" jjt_favor_decode "${z_favor}"

  z_favor=$(jjt_favor_encode 2048 131072)
  but_expect_ok_stdout "2048	131072" jjt_favor_decode "${z_favor}"

  # Test 7: Invalid inputs (should fail)
  but_info "Test 7: Invalid inputs (expect failures)"
  but_expect_fatal jjt_favor_encode 4096 0  # heat too large
  but_expect_fatal jjt_favor_encode 0 262144  # pace too large
  but_expect_fatal jjt_favor_encode -1 0  # heat negative
  but_expect_fatal jjt_favor_decode "ABCD"  # too short
  but_expect_fatal jjt_favor_decode "ABCDEF"  # too long
  but_expect_fatal jjt_favor_decode "ABC@D"  # invalid char

  but_section "=== All favor encoding tests passed ==="
}

jjt_test_studbook_validation() {
  but_section "=== Studbook Validation Tests ==="

  # Test 1: Valid empty studbook
  but_info "Test 1: Valid empty studbook"
  but_expect_ok jjt_studbook_validate '{"heats":{},"saddled":"","next_heat_seed":"AA"}'

  # Test 2: Valid studbook with heat and paces
  but_info "Test 2: Valid studbook with heat and paces"
  local z_valid_full='{
    "heats": {
      "₣Kb": {
        "datestamp": "260101",
        "display": "Test Heat",
        "silks": "test-heat",
        "status": "current",
        "paces": [
          {"id": "001", "display": "First pace", "status": "current"},
          {"id": "002", "display": "Second pace\nWith newlines\nIn description", "status": "pending"}
        ]
      }
    },
    "saddled": "₣Kb",
    "next_heat_seed": "Kc"
  }'
  but_expect_ok jjt_studbook_validate "${z_valid_full}"

  # Test 3: Valid with all pace statuses
  but_info "Test 3: Valid with all pace statuses"
  local z_all_statuses='{
    "heats": {
      "₣AA": {
        "datestamp": "260101",
        "display": "Status Test",
        "silks": "status-test",
        "status": "current",
        "paces": [
          {"id": "001", "display": "Current", "status": "current"},
          {"id": "002", "display": "Pending", "status": "pending"},
          {"id": "003", "display": "Complete", "status": "complete"},
          {"id": "004", "display": "Abandoned", "status": "abandoned"},
          {"id": "005", "display": "Malformed", "status": "malformed"}
        ]
      }
    },
    "saddled": "",
    "next_heat_seed": "AB"
  }'
  but_expect_ok jjt_studbook_validate "${z_all_statuses}"

  # Test 4: Invalid - missing saddled field
  but_info "Test 4: Invalid - missing saddled field"
  but_expect_fatal jjt_studbook_validate '{"heats":{},"next_heat_seed":"AA"}'

  # Test 5: Invalid - bad saddled format (too long - pace included)
  but_info "Test 5: Invalid - bad saddled format (pace included)"
  but_expect_fatal jjt_studbook_validate '{"heats":{},"saddled":"₣Kb001","next_heat_seed":"AA"}'

  # Test 6: Invalid - bad heat key (missing ₣ prefix)
  but_info "Test 6: Invalid - bad heat key (missing ₣)"
  but_expect_fatal jjt_studbook_validate '{"heats":{"Kb":{"datestamp":"260101","display":"X","silks":"x","status":"current","paces":[]}},"saddled":"","next_heat_seed":"AA"}'

  # Test 7: Invalid - bad pace status
  but_info "Test 7: Invalid - bad pace status"
  local z_bad_status='{
    "heats": {
      "₣Kb": {
        "datestamp": "260101",
        "display": "Test",
        "silks": "test",
        "status": "current",
        "paces": [{"id": "001", "display": "Bad", "status": "invalid"}]
      }
    },
    "saddled": "",
    "next_heat_seed": "Kc"
  }'
  but_expect_fatal jjt_studbook_validate "${z_bad_status}"

  # Test 8: Invalid - bad datestamp format
  but_info "Test 8: Invalid - bad datestamp format"
  local z_bad_date='{
    "heats": {
      "₣Kb": {
        "datestamp": "2026-01-01",
        "display": "Test",
        "silks": "test",
        "status": "current",
        "paces": []
      }
    },
    "saddled": "",
    "next_heat_seed": "Kc"
  }'
  but_expect_fatal jjt_studbook_validate "${z_bad_date}"

  # Test 9: Invalid - bad silks format (uppercase)
  but_info "Test 9: Invalid - bad silks format"
  local z_bad_silks='{
    "heats": {
      "₣Kb": {
        "datestamp": "260101",
        "display": "Test",
        "silks": "Test_Heat",
        "status": "current",
        "paces": []
      }
    },
    "saddled": "",
    "next_heat_seed": "Kc"
  }'
  but_expect_fatal jjt_studbook_validate "${z_bad_silks}"

  but_section "=== All studbook validation tests passed ==="
}

######################################################################
# Main

# When called via BUD dispatch: $1=tabtarget-stem, $2=suite-name
# When called directly: $1=suite-name
z_suite="${2:-${1:-}}"

case "${z_suite}" in
  favor)
    jjt_test_favor_encoding
    ;;
  studbook)
    jjt_test_studbook_validation
    ;;
  all)
    jjt_test_favor_encoding
    jjt_test_studbook_validation
    ;;
  *)
    echo "JJT Testbench - Job Jockey test execution"
    echo ""
    echo "Usage: ${0##*/} <suite>"
    echo ""
    echo "Test Suites:"
    echo "  favor     Test favor encoding/decoding"
    echo "  studbook  Test studbook validation"
    echo "  all       Run all test suites"
    exit 1
    ;;
esac

# eof
