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
# BUTCEV - Enrollment validation: choice types (bool, enum)

set -euo pipefail

######################################################################
# butcev_bool_valid_tcase - Bool enrollment with valid values

butcev_bool_valid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Booleans"
    buv_bool_enroll TEST_ENABLED "Feature enabled"

    buto_info "Value 1"
    export TEST_ENABLED="1"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Value 0"
    export TEST_ENABLED="0"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_bool_invalid_tcase - Bool enrollment with invalid values

butcev_bool_invalid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Booleans"
    buv_bool_enroll TEST_ENABLED "Feature enabled"

    buto_info "String true"
    export TEST_ENABLED="true"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "String yes"
    export TEST_ENABLED="yes"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Number 2"
    export TEST_ENABLED="2"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_bool_empty_tcase - Bool enrollment with empty value fails

butcev_bool_empty_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Booleans"
    buv_bool_enroll TEST_ENABLED "Feature enabled"

    export TEST_ENABLED=""
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_enum_valid_tcase - Enum enrollment with valid choice

butcev_enum_valid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Enums"
    buv_enum_enroll TEST_MODE "Operating mode" debug release test

    buto_info "First choice"
    export TEST_MODE="debug"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Last choice"
    export TEST_MODE="test"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_enum_invalid_tcase - Enum enrollment with invalid choice

butcev_enum_invalid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Enums"
    buv_enum_enroll TEST_MODE "Operating mode" debug release test

    buto_info "Not a valid choice"
    export TEST_MODE="production"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Case mismatch"
    export TEST_MODE="Debug"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_enum_empty_tcase - Enum enrollment with empty value fails

butcev_enum_empty_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "Enums"
    buv_enum_enroll TEST_MODE "Operating mode" debug release test

    export TEST_MODE=""
    buto_unit_expect_fatal buv_vet "TEST"
}

# eof
