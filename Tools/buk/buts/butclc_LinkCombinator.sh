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
# BUTCLC - Link combinator test cases for BUK self-test
#
# Exercises bug_* link combinators: OSC-8 hyperlink output and
# BURD_NO_HYPERLINKS fallback.  Pure local — no GCP, no containers.

set -euo pipefail

######################################################################
# Helpers

zbutclc_tlt_osc8() {
  bug_tlt "A " "hallmark" "https://example.com#hallmark" " is a named artifact."
}

zbutclc_tlt_fallback() {
  export BURD_NO_HYPERLINKS=1
  bug_tlt "A " "hallmark" "https://example.com#hallmark" " is a named artifact."
}

zbutclc_all_combinators() {
  bug_lt    "click here" "https://example.com" " for details"
  bug_tl    "See " "docs" "https://example.com/docs"
  bug_tlt   "A " "vessel" "https://example.com#vessel" " is a container image."
  bug_tlc   "Run " "setup" "https://example.com#setup" "tt/rbw-gO.sh"
  bug_tltlt "A " "vessel" "https://example.com#vessel" " contains a " "bottle" "https://example.com#bottle" " runtime."
}

######################################################################
# Test cases

butclc_tlt_osc8_tcase() {
  buto_trace "bug_tlt: OSC-8 hyperlink present in output"
  zbuto_invoke zbutclc_tlt_osc8
  buto_fatal_on_error "${ZBUTO_STATUS}" "bug_tlt failed" "STDERR: ${ZBUTO_STDERR}"
  local z_osc_marker
  z_osc_marker=$(printf '\033]8;;')
  case "${ZBUTO_STDERR}" in
    *"${z_osc_marker}"*) ;;
    *) buto_fatal "OSC-8 sequence not found in output" "Got: ${ZBUTO_STDERR}" ;;
  esac
}

butclc_tlt_fallback_tcase() {
  buto_trace "bug_tlt: BURD_NO_HYPERLINKS falls back to angle-bracket URL"
  zbuto_invoke zbutclc_tlt_fallback
  buto_fatal_on_error "${ZBUTO_STATUS}" "bug_tlt fallback failed" "STDERR: ${ZBUTO_STDERR}"
  case "${ZBUTO_STDERR}" in
    *"<https://example.com#hallmark>"*) ;;
    *) buto_fatal "Fallback URL not found in output" "Got: ${ZBUTO_STDERR}" ;;
  esac
  local z_osc_marker
  z_osc_marker=$(printf '\033]8;;')
  case "${ZBUTO_STDERR}" in
    *"${z_osc_marker}"*) buto_fatal "OSC-8 should not appear in fallback mode" ;;
    *) ;;
  esac
}

butclc_all_combinators_tcase() {
  buto_trace "All link combinators succeed with correct arg counts"
  buto_unit_expect_ok zbutclc_all_combinators
}

# eof
