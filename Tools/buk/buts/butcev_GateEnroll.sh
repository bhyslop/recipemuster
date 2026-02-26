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
# BUTCEV - Enrollment validation: gating (gated-in passes, gated-out skips)

set -euo pipefail

######################################################################
# butcev_gate_active_valid_tcase - Gate matches, valid value passes

butcev_gate_active_valid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Gated Features"
    buv_enum_enroll TEST_MODE "Feature mode" enabled disabled
    buv_gate_enroll TEST_MODE enabled
    buv_port_enroll TEST_PORT "Feature port"

    export TEST_MODE="enabled"
    export TEST_PORT="8080"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_gate_active_invalid_tcase - Gate matches, invalid value fails

butcev_gate_active_invalid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Gated Features"
    buv_enum_enroll TEST_MODE "Feature mode" enabled disabled
    buv_gate_enroll TEST_MODE enabled
    buv_port_enroll TEST_PORT "Feature port"

    export TEST_MODE="enabled"
    export TEST_PORT="0"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_gate_inactive_tcase - Gate doesn't match, bad value still passes (skipped)

butcev_gate_inactive_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Gated Features"
    buv_enum_enroll TEST_MODE "Feature mode" enabled disabled
    buv_gate_enroll TEST_MODE enabled
    buv_port_enroll TEST_PORT "Feature port"

    export TEST_MODE="disabled"
    export TEST_PORT="invalid-not-checked"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_gate_multi_tcase - Multiple groups, mixed gate states

butcev_gate_multi_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"

    buv_group_enroll "Core"
    buv_xname_enroll TEST_NAME 2 12 "Service name"

    buv_group_enroll "Feature A"
    buv_enum_enroll TEST_FEAT_A "Feature A mode" on off
    buv_gate_enroll TEST_FEAT_A on
    buv_port_enroll TEST_FEAT_A_PORT "Feature A port"

    buv_group_enroll "Feature B"
    buv_enum_enroll TEST_FEAT_B "Feature B mode" on off
    buv_gate_enroll TEST_FEAT_B on
    buv_string_enroll TEST_FEAT_B_LABEL 1 20 "Feature B label"

    buto_info "Feature A on, Feature B off"
    export TEST_NAME="myservice"
    export TEST_FEAT_A="on"
    export TEST_FEAT_A_PORT="9090"
    export TEST_FEAT_B="off"
    export TEST_FEAT_B_LABEL=""
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Both features on"
    export TEST_FEAT_B="on"
    export TEST_FEAT_B_LABEL="hello"
    buto_unit_expect_ok buv_vet "TEST"
}

# eof
