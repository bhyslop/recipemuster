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
# BUTCEV - Enrollment validation: numeric types (decimal, ipv4, port)

set -euo pipefail

######################################################################
# butcev_decimal_valid_tcase - Decimal enrollment within range

butcev_decimal_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Numerics"
    buv_decimal_enroll TEST_COUNT 1 100 "Item count"

    buto_info "At minimum"
    export TEST_COUNT="1"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "At maximum"
    export TEST_COUNT="100"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Mid-range"
    export TEST_COUNT="50"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_decimal_below_tcase - Decimal below minimum fails

butcev_decimal_below_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Numerics"
    buv_decimal_enroll TEST_COUNT 1 100 "Item count"

    export TEST_COUNT="0"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_decimal_above_tcase - Decimal above maximum fails

butcev_decimal_above_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Numerics"
    buv_decimal_enroll TEST_COUNT 1 100 "Item count"

    export TEST_COUNT="101"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_decimal_empty_tcase - Decimal empty value fails

butcev_decimal_empty_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Numerics"
    buv_decimal_enroll TEST_COUNT 1 100 "Item count"

    export TEST_COUNT=""
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_ipv4_valid_tcase - IPv4 enrollment with valid address

butcev_ipv4_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Network"
    buv_ipv4_enroll TEST_ADDR "Server address"

    export TEST_ADDR="192.168.1.1"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_ipv4_invalid_tcase - IPv4 enrollment with invalid address

butcev_ipv4_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Network"
    buv_ipv4_enroll TEST_ADDR "Server address"

    buto_info "Not dotted-quad"
    export TEST_ADDR="not-an-ip"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Empty"
    export TEST_ADDR=""
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_port_valid_tcase - Port enrollment with valid values

butcev_port_valid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Network"
    buv_port_enroll TEST_PORT "Service port"

    buto_info "Common port"
    export TEST_PORT="8080"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Minimum port"
    export TEST_PORT="1"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Maximum port"
    export TEST_PORT="65535"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_port_invalid_tcase - Port enrollment with invalid values

butcev_port_invalid_tcase() {
    zbutcev_fresh_kindle
    buv_regime_enroll "TEST"
    buv_group_enroll "Network"
    buv_port_enroll TEST_PORT "Service port"

    buto_info "Zero"
    export TEST_PORT="0"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Above max"
    export TEST_PORT="65536"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Empty"
    export TEST_PORT=""
    buto_unit_expect_fatal buv_vet "TEST"
}

# eof
