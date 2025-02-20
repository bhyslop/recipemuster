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
# Provides core validation functions for configuration regime validators


# Core error handling
crgv_print_and_die() {
    echo "ERROR: $*" >&2
    exit 1
}

# Basic type validators
crgv_is_boolean() {
    local val=${!1}
    test "$val" = "0" -o "$val" = "1" || crgv_print_and_die "$1 must be 0 or 1, got: $val"
}

crgv_is_boolean_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgv_is_boolean $1
}

crgv_in_range() {
    local val=${!1}
    test -n "$val" || crgv_print_and_die "$1 must not be empty"
    test $val -ge $2 -a $val -le $3 || crgv_print_and_die "$1 value $val must be between $2 and $3"
}

crgv_in_range_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgv_in_range $1 $2 $3
}

crgv_is_ipv4() {
    local val=${!1}
    test -n "$val" || crgv_print_and_die "$1 must not be empty"
    test $(echo $val | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || crgv_print_and_die "$1 has invalid IPv4: $val"
}

crgv_is_ipv4_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgv_is_ipv4 $1
}

crgv_is_cidr() {
    local val=${!1}
    test -n "$val" || crgv_print_and_die "$1 must not be empty"
    test $(echo $val | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || crgv_print_and_die "$1 has invalid CIDR: $val"
}

crgv_is_cidr_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgv_is_cidr $1
}

crgv_is_domain() {
    local val=${!1}
    test -n "$val" || crgv_print_and_die "$1 must not be empty"
    test $(echo $val | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || crgv_print_and_die "$1 has invalid domain: $val"
}

crgv_is_domain_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgv_is_domain $1
}

crgv_is_port() {
    crgv_in_range $1 1 65535
}

crgv_is_port_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgv_is_port $1
}

crgv_is_ipv4_list() {
    local val=${!1}
    test -z "$val" && return 0  # Empty lists allowed
    
    for item in $val; do
        crgv_is_ipv4 item
    done
}

crgv_is_cidr_list() {
    local val=${!1}
    test -z "$val" && return 0  # Empty lists allowed
    
    for item in $val; do
        crgv_is_cidr item
    done
}

crgv_is_domain_list() {
    local val=${!1}
    test -z "$val" && return 0  # Empty lists allowed
    
    for item in $val; do
        crgv_is_domain item
    done
}


# eof
