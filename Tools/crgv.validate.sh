#!/bin/bash

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

# Configuration Regime Validate Library

# Core error handling
crgv_print_and_die() {
    echo "ERROR: $*" >&2
    exit 1
}

# String validator with optional length constraints
crgv_string() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"
    local min=$3
    local max=$4

    # Allow empty if min=0
    test "$min" = "0" -a -z "$val" && return 0

    # Otherwise must not be empty
    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"

    # Check length constraints if max provided
    if [ -n "$max" ]; then
        test ${#val} -ge $min || crgv_print_and_die "[$context] $varname must be at least $min chars, got '${val}' (${#val})"
        test ${#val} -le $max || crgv_print_and_die "[$context] $varname must be no more than $max chars, got '${val}' (${#val})"
    fi
}

# Cross-context name validator (system-safe identifier)
crgv_xname() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"
    local min=$3
    local max=$4

    # Allow empty if min=0
    test "$min" = "0" -a -z "$val" && return 0

    # Otherwise must not be empty
    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"

    # Check length constraints
    test ${#val} -ge $min || crgv_print_and_die "[$context] $varname must be at least $min chars, got '${val}' (${#val})"
    test ${#val} -le $max || crgv_print_and_die "[$context] $varname must be no more than $max chars, got '${val}' (${#val})"

    # Must start with letter and contain only allowed chars
    test $(echo "$val" | grep -E '^[a-zA-Z][a-zA-Z0-9_-]*$') || \
        crgv_print_and_die "[$context] $varname must start with letter and contain only letters, numbers, underscore, hyphen, got '$val'"
}

# Fully Qualified Image Name component validator
crgv_fqin() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"
    local min=$3
    local max=$4

    # Allow empty if min=0
    test "$min" = "0" -a -z "$val" && return 0

    # Otherwise must not be empty
    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"

    # Check length constraints
    test ${#val} -ge $min || crgv_print_and_die "[$context] $varname must be at least $min chars, got '${val}' (${#val})"
    test ${#val} -le $max || crgv_print_and_die "[$context] $varname must be no more than $max chars, got '${val}' (${#val})"

    # Allow letters, numbers, dots, hyphens, underscores, forward slashes, colons
    test $(echo "$val" | grep -E '^[a-zA-Z0-9][a-zA-Z0-9:._/-]*$') || \
        crgv_print_and_die "[$context] $varname must start with letter/number and contain only letters, numbers, colons, dots, underscores, hyphens, forward slashes, got '$val'"
}

crgv_bool() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test "$val" = "0" -o "$val" = "1" || crgv_print_and_die "[$context] $varname must be 0 or 1, got: '$val'"
}

crgv_opt_bool() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0
    test "$val" = "0" -o "$val" = "1" || crgv_print_and_die "[$context] $varname must be 0 or 1, got: '$val'"
}

crgv_decimal() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"
    local min=$3
    local max=$4

    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"
    test $val -ge $min -a $val -le $max || crgv_print_and_die "[$context] $varname value '$val' must be between $min and $max"
}

crgv_opt_range() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"
    local min=$3
    local max=$4

    test -z "$val" && return 0
    test $val -ge $min -a $val -le $max || crgv_print_and_die "[$context] $varname value '$val' must be between $min and $max"
}

crgv_ipv4() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"
    test $(echo $val | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || crgv_print_and_die "[$context] $varname has invalid IPv4 format: '$val'"
}

crgv_opt_ipv4() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0
    test $(echo $val | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || crgv_print_and_die "[$context] $varname has invalid IPv4 format: '$val'"
}

crgv_cidr() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"
    test $(echo $val | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || crgv_print_and_die "[$context] $varname has invalid CIDR format: '$val'"
}

crgv_opt_cidr() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0
    test $(echo $val | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || crgv_print_and_die "[$context] $varname has invalid CIDR format: '$val'"
}

crgv_domain() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"
    test $(echo $val | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || crgv_print_and_die "[$context] $varname has invalid domain format: '$val'"
}

crgv_opt_domain() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0
    test $(echo $val | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || crgv_print_and_die "[$context] $varname has invalid domain format: '$val'"
}

crgv_port() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -n "$val" || crgv_print_and_die "[$context] $varname must not be empty"
    test $val -ge 1 -a $val -le 65535 || crgv_print_and_die "[$context] $varname value '$val' must be between 1 and 65535"
}

crgv_opt_port() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0
    test $val -ge 1 -a $val -le 65535 || crgv_print_and_die "[$context] $varname value '$val' must be between 1 and 65535"
}

crgv_list_ipv4() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0  # Empty lists allowed

    local item_num=0
    for item in $val; do
        item_num=$((item_num + 1))
        test $(echo $item | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || crgv_print_and_die "[$context] $varname item #$item_num has invalid IPv4 format: '$item'"
    done
}

crgv_list_cidr() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0  # Empty lists allowed

    local item_num=0
    for item in $val; do
        item_num=$((item_num + 1))
        test $(echo $item | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || crgv_print_and_die "[$context] $varname item #$item_num has invalid CIDR format: '$item'"
    done
}

crgv_list_domain() {
    local context=$1
    local varname=$2
    eval "local val=\${$varname:-}" || crgv_print_and_die "Variable '$varname' is not defined in '$context'"

    test -z "$val" && return 0  # Empty lists allowed

    local item_num=0
    for item in $val; do
        item_num=$((item_num + 1))
        test $(echo $item | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || crgv_print_and_die "[$context] $varname item #$item_num has invalid domain format: '$item'"
    done
}

# eof