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
# BUTCVU - Xname validation test cases for RBTB testbench

set -euo pipefail

######################################################################
# Private helper: echo bob

zbutcvu_echo_bob() { echo "bob"; }

######################################################################
# butcvu_debug_tcase - Test info output

butcvu_debug_tcase() {
    buto_info "HERES AN INFO STRING"

    buto_unit_expect_ok_stdout "bob" zbutcvu_echo_bob
}

######################################################################
# butcvu_xname_valid_tcase - Valid xname test cases

butcvu_xname_valid_tcase() {
    buto_unit_expect_ok buv_val_xname "var" "abc"        1 10
    buto_unit_expect_ok buv_val_xname "var" "Test123"    1 10
    buto_unit_expect_ok buv_val_xname "var" "my_var"     1 10
    buto_unit_expect_ok buv_val_xname "var" "my-name"    1 10
    buto_unit_expect_ok buv_val_xname "var" "A1_2-3"     1 10
    buto_unit_expect_ok buv_val_xname "var" "x"          1 10
    buto_unit_expect_ok buv_val_xname "var" "abcdefghij" 1 10
}

######################################################################
# butcvu_xname_invalid_start_tcase - Invalid start character test cases

butcvu_xname_invalid_start_tcase() {
    buto_unit_expect_fatal buv_val_xname "var" "1abc"  1 10
    buto_unit_expect_fatal buv_val_xname "var" "_test" 1 10
    buto_unit_expect_fatal buv_val_xname "var" "-name" 1 10
    buto_unit_expect_fatal buv_val_xname "var" "123"   1 10
    buto_unit_expect_fatal buv_val_xname "var" ""      1 10
}

######################################################################
# butcvu_xname_invalid_chars_tcase - Invalid character test cases

butcvu_xname_invalid_chars_tcase() {
    buto_unit_expect_fatal buv_val_xname "var" "my.name"     1  10
    buto_unit_expect_fatal buv_val_xname "var" "test@var"    1  10
    buto_unit_expect_fatal buv_val_xname "var" "hello world" 1  10
    buto_unit_expect_fatal buv_val_xname "var" 'a$b'         1  10
    buto_unit_expect_fatal buv_val_xname "var" "test/path"   1  10
    buto_unit_expect_fatal buv_val_xname "var" "name:tag"    1  10
}

######################################################################
# butcvu_xname_length_tcase - Length boundary test cases

butcvu_xname_length_tcase() {
    buto_info "Too short"
    buto_unit_expect_fatal buv_val_xname "var" "ab" 3 10

    buto_info "Too long"
    buto_unit_expect_fatal buv_val_xname "var" "abcdefghijk" 1 10

    buto_info "Exactly at limits"
    buto_unit_expect_ok buv_val_xname "var" "abc"        3  10
    buto_unit_expect_ok buv_val_xname "var" "abcdefghij" 10 10
}

######################################################################
# butcvu_xname_defaults_tcase - Default value test cases

butcvu_xname_defaults_tcase() {
    buto_info "Empty with default"
    buto_unit_expect_ok buv_val_xname "var" "" 1 10 "mydefault"

    buto_info "Non-empty ignores default"
    buto_unit_expect_ok buv_val_xname "var" "actual" 1 10 "mydefault"

    buto_info "Empty with min=0 and default"
    buto_unit_expect_ok buv_val_xname "var" "" 0 10 "mydefault"

    buto_info "Case completed"
}

######################################################################
# butcvu_xname_empty_optional_tcase - Empty optional value test cases

butcvu_xname_empty_optional_tcase() {
    buto_info "Empty allowed when min=0"
    buto_unit_expect_ok buv_val_xname "var" "" 0 10

    buto_info "Empty not allowed when min>0"
    buto_unit_expect_fatal buv_val_xname "var" "" 1 10
}

######################################################################
# butcvu_xname_env_wrapper_tcase - Environment variable wrapper test cases

butcvu_xname_env_wrapper_tcase() {
    buto_info "Valid value"
    export TEST_VAR="myname"
    buto_unit_expect_ok buv_env_xname "TEST_VAR" 1 10

    buto_info "Invalid value"
    export TEST_VAR="123invalid"
    buto_unit_expect_fatal buv_env_xname "TEST_VAR" 1 10

    buto_info "Empty with default"
    export TEST_VAR=""
    buto_unit_expect_ok buv_env_xname "TEST_VAR" 1 10 "default123"

    buto_info "Unset variable"
    unset TEST_VAR
    buto_unit_expect_fatal buv_env_xname "TEST_VAR" 1 10
}

# eof
