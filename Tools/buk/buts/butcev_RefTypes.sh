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
# BUTCEV - Enrollment validation: reference types (odref)

set -euo pipefail

######################################################################
# Private constants

ZBUTCEV_VALID_DIGEST="sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"

######################################################################
# butcev_odref_valid_tcase - OCI digest-pinned ref with valid values

butcev_odref_valid_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "References"
    buv_odref_enroll TEST_IMAGE "Container image"

    buto_info "Standard registry"
    export TEST_IMAGE="docker.io/library/alpine@${ZBUTCEV_VALID_DIGEST}"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Multi-level repo path"
    export TEST_IMAGE="us-central1-docker.pkg.dev/my-proj/my-repo/tool@${ZBUTCEV_VALID_DIGEST}"
    buto_unit_expect_ok buv_vet "TEST"

    buto_info "Registry with port"
    export TEST_IMAGE="registry.local:5000/myimage@${ZBUTCEV_VALID_DIGEST}"
    buto_unit_expect_ok buv_vet "TEST"
}

######################################################################
# butcev_odref_no_digest_tcase - OCI ref without digest fails

butcev_odref_no_digest_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "References"
    buv_odref_enroll TEST_IMAGE "Container image"

    buto_info "Tag only, no digest"
    export TEST_IMAGE="docker.io/library/alpine:latest"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_odref_malformed_tcase - OCI ref with malformed digest fails

butcev_odref_malformed_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "References"
    buv_odref_enroll TEST_IMAGE "Container image"

    buto_info "Wrong algorithm"
    export TEST_IMAGE="docker.io/library/alpine@md5:abcdef0123456789"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Short hex"
    export TEST_IMAGE="docker.io/library/alpine@sha256:abcdef"
    buto_unit_expect_fatal buv_vet "TEST"

    buto_info "Uppercase hex"
    export TEST_IMAGE="docker.io/library/alpine@sha256:ABCDEF0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
    buto_unit_expect_fatal buv_vet "TEST"
}

######################################################################
# butcev_odref_empty_tcase - OCI ref empty fails

butcev_odref_empty_tcase() {
    zbutcev_fresh_enrollment
    buv_regime_enroll "TEST"
    buv_group_enroll "References"
    buv_odref_enroll TEST_IMAGE "Container image"

    export TEST_IMAGE=""
    buto_unit_expect_fatal buv_vet "TEST"
}

# eof
