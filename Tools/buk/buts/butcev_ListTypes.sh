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
# BUTCEV - Enrollment validation: list types (list_string, list_ipv4, list_gname)

set -euo pipefail

######################################################################
# butcev_list_string_valid_tcase - List string with valid items

butcev_list_string_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_string_enroll TEST_TAGS 2 10 "Tags"

    export TEST_TAGS="foo bar baz"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_list_string_empty_tcase - Empty list passes (no items to validate)

butcev_list_string_empty_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_string_enroll TEST_TAGS 2 10 "Tags"

    export TEST_TAGS=""
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_list_string_bad_item_tcase - List with item too short fails

butcev_list_string_bad_item_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_string_enroll TEST_TAGS 3 10 "Tags"

    buto_info "Second item too short"
    export TEST_TAGS="good ab okay"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Item too long"
    export TEST_TAGS="good toolongvalue okay"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_list_ipv4_valid_tcase - List IPv4 with valid addresses

butcev_list_ipv4_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_ipv4_enroll TEST_SERVERS "Server addresses"

    export TEST_SERVERS="192.168.1.1 10.0.0.1 172.16.0.1"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_list_ipv4_invalid_tcase - List IPv4 with bad address fails

butcev_list_ipv4_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_ipv4_enroll TEST_SERVERS "Server addresses"

    export TEST_SERVERS="192.168.1.1 not-an-ip 10.0.0.1"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_list_ipv4_empty_tcase - Empty IPv4 list passes

butcev_list_ipv4_empty_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_ipv4_enroll TEST_SERVERS "Server addresses"

    export TEST_SERVERS=""
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_list_gname_valid_tcase - List gname with valid names

butcev_list_gname_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_gname_enroll TEST_PROJECTS 3 20 "Project IDs"

    export TEST_PROJECTS="my-project other-proj test-01"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_list_gname_invalid_tcase - List gname with bad name fails

butcev_list_gname_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Lists"
    buv_list_gname_enroll TEST_PROJECTS 3 20 "Project IDs"

    buto_info "Uppercase in second item"
    export TEST_PROJECTS="my-project BadName test-01"
    buto_unit_expect_fatal buv_vet "TEST"
}

# eof
