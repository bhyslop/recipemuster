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
# BUTCEV - Enrollment validation: enforce (vet) and report integration

set -euo pipefail

######################################################################
# butcev_enforce_all_pass_tcase - Vet succeeds when all vars valid

butcev_enforce_all_pass_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Core"
    buv_xname_enroll TEST_NAME 2 12 "Name"
    buv_bool_enroll TEST_FLAG "Flag"
    buv_decimal_enroll TEST_COUNT 1 10 "Count"

    export TEST_NAME="myname"
    export TEST_FLAG="1"
    export TEST_COUNT="5"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_enforce_first_bad_tcase - Vet dies on first invalid var

butcev_enforce_first_bad_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Core"
    buv_xname_enroll TEST_NAME 2 12 "Name"
    buv_bool_enroll TEST_FLAG "Flag"
    buv_decimal_enroll TEST_COUNT 1 10 "Count"

    buto_info "First var invalid"
    export TEST_NAME="1"
    export TEST_FLAG="1"
    export TEST_COUNT="5"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Middle var invalid"
    export TEST_NAME="myname"
    export TEST_FLAG="maybe"
    export TEST_COUNT="5"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Last var invalid"
    export TEST_NAME="myname"
    export TEST_FLAG="1"
    export TEST_COUNT="99"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_report_all_pass_tcase - Report returns 0 when all valid

butcev_report_all_pass_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Core"
    buv_xname_enroll TEST_NAME 2 12 "Name"
    buv_bool_enroll TEST_FLAG "Flag"

    export TEST_NAME="myname"
    export TEST_FLAG="0"
    buto_unit_expect_ok buv_report "TEST" "All-pass report"
}

######################################################################
# butcev_report_mixed_tcase - Report returns non-zero with failures

butcev_report_mixed_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Core"
    buv_xname_enroll TEST_NAME 2 12 "Name"
    buv_bool_enroll TEST_FLAG "Flag"

    export TEST_NAME="myname"
    export TEST_FLAG="bad"
    buto_unit_expect_fatal buv_report "TEST" "Mixed report"
}

######################################################################
# butcev_report_gated_tcase - Report shows SKIP for gated-out vars

butcev_report_gated_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Gated"
    buv_enum_enroll TEST_MODE "Mode" on off
    buv_gate_enroll TEST_MODE on
    buv_port_enroll TEST_PORT "Port"

    buto_info "Gate inactive - port gated out, report passes"
    export TEST_MODE="off"
    export TEST_PORT=""
    buto_unit_expect_ok buv_report "TEST" "Gated report"
}

######################################################################
# butcev_multiscope_tcase - Report filters by scope

butcev_multiscope_tcase() {
    zbutcev_fresh_enrollment

    buv_regime_enroll "ALPHA"
    buv_group_enroll "Alpha Vars"
    buv_bool_enroll TEST_ALPHA_FLAG "Alpha flag"

    buv_regime_enroll "BETA"
    buv_group_enroll "Beta Vars"
    buv_bool_enroll TEST_BETA_FLAG "Beta flag"

    buto_info "Alpha valid, Beta invalid - vet ALPHA passes"
    export TEST_ALPHA_FLAG="1"
    export TEST_BETA_FLAG="bad"
    buto_unit_expect_ok buv_vet "ALPHA"

    buto_info "Vet BETA fails"
    buto_unit_expect_fatal buv_vet "BETA"
}

# eof
