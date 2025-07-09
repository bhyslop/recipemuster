#!/bin/bash
# Test suite for BVU xname validation functions

# Source the libraries from parent directory
ZTBTU_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
source "${ZTBTU_SCRIPT_DIR}/bcu_BashConsoleUtility.sh"
source "${ZTBTU_SCRIPT_DIR}/btu_BashTestUtility.sh"
source "${ZTBTU_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

tbvu_case_xname_valid() {
    set -e

    btu_expect_ok_stdout "abc"        bvu_val_xname "var" "abc"        1 10
    btu_expect_ok_stdout "Test123"    bvu_val_xname "var" "Test123"    1 10
    btu_expect_ok_stdout "my_var"     bvu_val_xname "var" "my_var"     1 10
    btu_expect_ok_stdout "my-name"    bvu_val_xname "var" "my-name"    1 10
    btu_expect_ok_stdout "A1_2-3"     bvu_val_xname "var" "A1_2-3"     1 10
    btu_expect_ok_stdout "x"          bvu_val_xname "var" "x"          1 10
    btu_expect_ok_stdout "abcdefghij" bvu_val_xname "var" "abcdefghij" 1 10
}

tbvu_case_xname_invalid_start() {
    set -e

    btu_expect_fatal bvu_val_xname "var" "1abc"  1 10
    btu_expect_fatal bvu_val_xname "var" "_test" 1 10
    btu_expect_fatal bvu_val_xname "var" "-name" 1 10
    btu_expect_fatal bvu_val_xname "var" "123"   1 10
    btu_expect_fatal bvu_val_xname "var" ""      1 10
}

tbvu_case_xname_invalid_chars() {
    set -e

    btu_expect_fatal bvu_val_xname "var" "my.name"     1  10
    btu_expect_fatal bvu_val_xname "var" "test@var"    1  10
    btu_expect_fatal bvu_val_xname "var" "hello world" 1  10
    btu_expect_fatal bvu_val_xname "var" "a$b"         1  10
    btu_expect_fatal bvu_val_xname "var" "test/path"   1  10
    btu_expect_fatal bvu_val_xname "var" "name:tag"    1  10
}

tbvu_case_xname_length() {
    set -e

    btu_info "Too short"
    btu_expect_fatal bvu_val_xname "var" "ab" 3 10

    btu_info "Too long"
    btu_expect_fatal bvu_val_xname "var" "abcdefghijk" 1 10

    btu_info "Exactly at limits"
    btu_expect_ok_stdout "abc"        bvu_val_xname "var" "abc"        3  10
    btu_expect_ok_stdout "abcdefghij" bvu_val_xname "var" "abcdefghij" 10 10
}

tbvu_case_xname_defaults() {
    set -e

    btu_info "Empty with default"
    btu_expect_ok_stdout "mydefault" bvu_val_xname "var" "" 1 10 "mydefault"

    btu_info "Non-empty ignores default"
    btu_expect_ok_stdout "actual" bvu_val_xname "var" "actual" 1 10 "mydefault"

    btu_info "Empty with min=0 and default"
    btu_expect_ok_stdout "mydefault" bvu_val_xname "var" "" 0 10 "mydefault"

    btu_info "Case completed"
}

tbvu_case_xname_empty_optional() {
    set -e

    btu_info "Empty allowed when min=0"
    btu_expect_ok_stdout "" bvu_val_xname "var" "" 0 10

    btu_info "Empty not allowed when min>0"
    btu_expect_fatal bvu_val_xname "var" "" 1 10
}

tbvu_case_xname_env_wrapper() {
    set -e

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
btu_execute "tbvu_case_" "$1"

# eof

