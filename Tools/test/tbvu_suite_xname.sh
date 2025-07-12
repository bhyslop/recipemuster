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
ZTBTU_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
source "${ZTBTU_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZTBTU_SCRIPT_DIR}/btu_BashTestUtility.sh"
source "${ZTBTU_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

tbvu_util_echo_bob() { echo "bob"; }

tbvu_case_debug() {
    btu_info "HERES AN INFO STRING"

    btu_expect_ok_stdout "bob" tbvu_util_echo_bob
}

tbvu_case_xname_valid() {
    btu_expect_ok_stdout "abc"        bvu_val_xname "var" "abc"        1 10
    btu_expect_ok_stdout "Test123"    bvu_val_xname "var" "Test123"    1 10
    btu_expect_ok_stdout "my_var"     bvu_val_xname "var" "my_var"     1 10
    btu_expect_ok_stdout "my-name"    bvu_val_xname "var" "my-name"    1 10
    btu_expect_ok_stdout "A1_2-3"     bvu_val_xname "var" "A1_2-3"     1 10
    btu_expect_ok_stdout "x"          bvu_val_xname "var" "x"          1 10
    btu_expect_ok_stdout "abcdefghij" bvu_val_xname "var" "abcdefghij" 1 10
}

tbvu_case_xname_invalid_start() {
    btu_expect_fatal bvu_val_xname "var" "1abc"  1 10
    btu_expect_fatal bvu_val_xname "var" "_test" 1 10
    btu_expect_fatal bvu_val_xname "var" "-name" 1 10
    btu_expect_fatal bvu_val_xname "var" "123"   1 10
    btu_expect_fatal bvu_val_xname "var" ""      1 10
}

tbvu_case_xname_invalid_chars() {
    btu_expect_fatal bvu_val_xname "var" "my.name"     1  10
    btu_expect_fatal bvu_val_xname "var" "test@var"    1  10
    btu_expect_fatal bvu_val_xname "var" "hello world" 1  10
    btu_expect_fatal bvu_val_xname "var" 'a$b'         1  10
    btu_expect_fatal bvu_val_xname "var" "test/path"   1  10
    btu_expect_fatal bvu_val_xname "var" "name:tag"    1  10
}

tbvu_case_xname_length() {
    btu_info "Too short"
    btu_expect_fatal bvu_val_xname "var" "ab" 3 10

    btu_info "Too long"
    btu_expect_fatal bvu_val_xname "var" "abcdefghijk" 1 10

    btu_info "Exactly at limits"
    btu_expect_ok_stdout "abc"        bvu_val_xname "var" "abc"        3  10
    btu_expect_ok_stdout "abcdefghij" bvu_val_xname "var" "abcdefghij" 10 10
}

tbvu_case_xname_defaults() {
    btu_info "Empty with default"
    btu_expect_ok_stdout "mydefault" bvu_val_xname "var" "" 1 10 "mydefault"

    btu_info "Non-empty ignores default"
    btu_expect_ok_stdout "actual" bvu_val_xname "var" "actual" 1 10 "mydefault"

    btu_info "Empty with min=0 and default"
    btu_expect_ok_stdout "mydefault" bvu_val_xname "var" "" 0 10 "mydefault"

    btu_info "Case completed"
}

tbvu_case_xname_empty_optional() {
    btu_info "Empty allowed when min=0"
    btu_expect_ok_stdout "" bvu_val_xname "var" "" 0 10

    btu_info "Empty not allowed when min>0"
    btu_expect_fatal bvu_val_xname "var" "" 1 10
}

tbvu_case_xname_env_wrapper() {
    btu_info "Valid value"
    export TEST_VAR="myname"
    btu_expect_ok_stdout "myname" bvu_env_xname "TEST_VAR" 1 10

    btu_info "Invalid value"
    export TEST_VAR="123invalid"
    btu_expect_fatal bvu_env_xname "TEST_VAR" 1 10

    btu_info "Empty with default"
    export TEST_VAR=""
    btu_expect_ok_stdout "default123" bvu_env_xname "TEST_VAR" 1 10 "default123"

    btu_info "Unset variable"
    unset TEST_VAR
    btu_expect_fatal bvu_env_xname "TEST_VAR" 1 10
}

# Execute tests
btu_execute "$1" "tbvu_case_" "$2"


# eof

