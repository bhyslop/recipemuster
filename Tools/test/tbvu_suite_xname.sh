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

# Test suite for BVU xname validation functions

# Source the libraries from parent directory
ZBUT_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
source "${ZBUT_SCRIPT_DIR}/buc_command.sh"
source "${ZBUT_SCRIPT_DIR}/but_test.sh"
source "${ZBUT_SCRIPT_DIR}/buv_validation.sh"

tbvu_util_echo_bob() { echo "bob"; }

tbvu_case_debug() {
    but_info "HERES AN INFO STRING"

    but_unit_expect_ok_stdout "bob" tbvu_util_echo_bob
}

tbvu_case_xname_valid() {
    but_unit_expect_ok_stdout "abc"        buv_val_xname "var" "abc"        1 10
    but_unit_expect_ok_stdout "Test123"    buv_val_xname "var" "Test123"    1 10
    but_unit_expect_ok_stdout "my_var"     buv_val_xname "var" "my_var"     1 10
    but_unit_expect_ok_stdout "my-name"    buv_val_xname "var" "my-name"    1 10
    but_unit_expect_ok_stdout "A1_2-3"     buv_val_xname "var" "A1_2-3"     1 10
    but_unit_expect_ok_stdout "x"          buv_val_xname "var" "x"          1 10
    but_unit_expect_ok_stdout "abcdefghij" buv_val_xname "var" "abcdefghij" 1 10
}

tbvu_case_xname_invalid_start() {
    but_unit_expect_fatal buv_val_xname "var" "1abc"  1 10
    but_unit_expect_fatal buv_val_xname "var" "_test" 1 10
    but_unit_expect_fatal buv_val_xname "var" "-name" 1 10
    but_unit_expect_fatal buv_val_xname "var" "123"   1 10
    but_unit_expect_fatal buv_val_xname "var" ""      1 10
}

tbvu_case_xname_invalid_chars() {
    but_unit_expect_fatal buv_val_xname "var" "my.name"     1  10
    but_unit_expect_fatal buv_val_xname "var" "test@var"    1  10
    but_unit_expect_fatal buv_val_xname "var" "hello world" 1  10
    but_unit_expect_fatal buv_val_xname "var" 'a$b'         1  10
    but_unit_expect_fatal buv_val_xname "var" "test/path"   1  10
    but_unit_expect_fatal buv_val_xname "var" "name:tag"    1  10
}

tbvu_case_xname_length() {
    but_info "Too short"
    but_unit_expect_fatal buv_val_xname "var" "ab" 3 10

    but_info "Too long"
    but_unit_expect_fatal buv_val_xname "var" "abcdefghijk" 1 10

    but_info "Exactly at limits"
    but_unit_expect_ok_stdout "abc"        buv_val_xname "var" "abc"        3  10
    but_unit_expect_ok_stdout "abcdefghij" buv_val_xname "var" "abcdefghij" 10 10
}

tbvu_case_xname_defaults() {
    but_info "Empty with default"
    but_unit_expect_ok_stdout "mydefault" buv_val_xname "var" "" 1 10 "mydefault"

    but_info "Non-empty ignores default"
    but_unit_expect_ok_stdout "actual" buv_val_xname "var" "actual" 1 10 "mydefault"

    but_info "Empty with min=0 and default"
    but_unit_expect_ok_stdout "mydefault" buv_val_xname "var" "" 0 10 "mydefault"

    but_info "Case completed"
}

tbvu_case_xname_empty_optional() {
    but_info "Empty allowed when min=0"
    but_unit_expect_ok_stdout "" buv_val_xname "var" "" 0 10

    but_info "Empty not allowed when min>0"
    but_unit_expect_fatal buv_val_xname "var" "" 1 10
}

tbvu_case_xname_env_wrapper() {
    but_info "Valid value"
    export TEST_VAR="myname"
    but_unit_expect_ok_stdout "myname" buv_env_xname "TEST_VAR" 1 10

    but_info "Invalid value"
    export TEST_VAR="123invalid"
    but_unit_expect_fatal buv_env_xname "TEST_VAR" 1 10

    but_info "Empty with default"
    export TEST_VAR=""
    but_unit_expect_ok_stdout "default123" buv_env_xname "TEST_VAR" 1 10 "default123"

    but_info "Unset variable"
    unset TEST_VAR
    but_unit_expect_fatal buv_env_xname "TEST_VAR" 1 10
}

# Execute tests
but_execute "$1" "tbvu_case_" "$2"


# eof

