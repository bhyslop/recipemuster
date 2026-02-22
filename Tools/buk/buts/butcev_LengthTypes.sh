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
# BUTCEV - Enrollment validation: length-bounded types (string, xname, gname, fqin)

set -euo pipefail

######################################################################
# Private helper: fresh kindle for enrollment tests

zbutcev_fresh_kindle() {
  ZBUV_KINDLED=""
  zbuv_kindle
}

######################################################################
# butcev_string_valid_tcase - String enrollment with valid values

butcev_string_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Strings"
    buv_string_enroll TEST_NAME 1 20 "Test name"
    buv_string_enroll TEST_DESC 3 50 "Test description"

    export TEST_NAME="hello"
    export TEST_DESC="a valid description"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_string_empty_optional_tcase - String with min=0 allows empty

butcev_string_empty_optional_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Strings"
    buv_string_enroll TEST_OPT 0 20 "Optional field"

    export TEST_OPT=""
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_string_too_short_tcase - String below min length fails

butcev_string_too_short_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Strings"
    buv_string_enroll TEST_NAME 5 20 "Test name"

    export TEST_NAME="ab"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_string_too_long_tcase - String above max length fails

butcev_string_too_long_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Strings"
    buv_string_enroll TEST_NAME 1 5 "Test name"

    export TEST_NAME="toolongvalue"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_string_empty_required_tcase - Required string empty fails

butcev_string_empty_required_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Strings"
    buv_string_enroll TEST_NAME 1 20 "Test name"

    export TEST_NAME=""
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_xname_enrolled_valid_tcase - Xname enrollment with valid values

butcev_xname_enrolled_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Xnames"
    buv_xname_enroll TEST_IDENT 2 12 "Identifier"

    buto_info "Standard xname"
    export TEST_IDENT="myName"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Xname with underscore and hyphen"
    export TEST_IDENT="my_var-1"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_xname_enrolled_invalid_tcase - Xname enrollment with invalid values

butcev_xname_enrolled_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Xnames"
    buv_xname_enroll TEST_IDENT 2 12 "Identifier"

    buto_info "Starts with digit"
    export TEST_IDENT="1bad"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Contains dot"
    export TEST_IDENT="my.name"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Too short"
    export TEST_IDENT="x"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Too long"
    export TEST_IDENT="abcdefghijklm"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_gname_enrolled_valid_tcase - Gname enrollment with valid values

butcev_gname_enrolled_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Gnames"
    buv_gname_enroll TEST_PROJECT 3 20 "Project ID"

    export TEST_PROJECT="my-project-01"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_gname_enrolled_invalid_tcase - Gname enrollment with invalid values

butcev_gname_enrolled_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Gnames"
    buv_gname_enroll TEST_PROJECT 3 20 "Project ID"

    buto_info "Uppercase letters"
    export TEST_PROJECT="MyProject"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Ends with hyphen"
    export TEST_PROJECT="my-project-"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Starts with digit"
    export TEST_PROJECT="1project"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_fqin_enrolled_valid_tcase - FQIN enrollment with valid values

butcev_fqin_enrolled_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "FQINs"
    buv_fqin_enroll TEST_IMAGE 5 100 "Image reference"

    export TEST_IMAGE="us-central1-docker.pkg.dev/my-proj/repo/image:latest"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_fqin_enrolled_invalid_tcase - FQIN enrollment with invalid values

butcev_fqin_enrolled_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "FQINs"
    buv_fqin_enroll TEST_IMAGE 5 100 "Image reference"

    buto_info "Starts with special char"
    export TEST_IMAGE=".invalid/path"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Empty value"
    export TEST_IMAGE=""
    buto_unit_expect_fatal buv_vet "TEST"
}

# eof
