#!/bin/bash

# Copyright 2024 Scale Invariant, Inc.
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

# Configuration Regime Library (crgl)
# Provides core validation functions for configuration regime validators


# Core error handling
crgl_die() {
    echo "ERROR: $1" >&2
    exit 1
}

# Basic type validators
crgl_is_boolean() {
    local val=${!1}
    test "$val" = "0" -o "$val" = "1" || crgl_die "$1 must be 0 or 1, got: $val"
}

crgl_is_boolean_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgl_is_boolean $1
}

crgl_in_range() {
    local val=${!1}
    test -n "$val" || crgl_die "$1 must not be empty"
    test $val -ge $2 -a $val -le $3 || crgl_die "$1 value $val must be between $2 and $3"
}

crgl_in_range_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgl_in_range $1 $2 $3
}

# Network-related validators
crgl_is_ipv4() {
    local val=${!1}
    test -n "$val" || crgl_die "$1 must not be empty"
    test $(echo $val | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$') || crgl_die "$1 has invalid IPv4: $val"
}

crgl_is_ipv4_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgl_is_ipv4 $1
}

crgl_is_cidr() {
    local val=${!1}
    test -n "$val" || crgl_die "$1 must not be empty"
    test $(echo $val | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$') || crgl_die "$1 has invalid CIDR: $val"
}

crgl_is_cidr_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgl_is_cidr $1
}

crgl_is_domain() {
    local val=${!1}
    test -n "$val" || crgl_die "$1 must not be empty"
    test $(echo $val | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$') || crgl_die "$1 has invalid domain: $val"
}

crgl_is_domain_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgl_is_domain $1
}

crgl_is_port() {
    crgl_in_range $1 1 65535
}

crgl_is_port_opt() {
    local val=${!1}
    test -z "$val" && return 0
    crgl_is_port $1
}

# List validators
crgl_is_ipv4_list() {
    local val=${!1}
    test -z "$val" && return 0  # Empty lists allowed
    
    for item in $val; do
        crgl_is_ipv4 item
    done
}

crgl_is_cidr_list() {
    local val=${!1}
    test -z "$val" && return 0  # Empty lists allowed
    
    for item in $val; do
        crgl_is_cidr item
    done
}

crgl_is_domain_list() {
    local val=${!1}
    test -z "$val" && return 0  # Empty lists allowed
    
    for item in $val; do
        crgl_is_domain item
    done
}

crgl_render_header() {
    echo "=== $1 ==="
}

crgl_render_group() {
    echo "--- $1 ---"
}

# Render a single value with label
crgl_render_value() {
    local varname=$1
    local val=${!1}
    printf "%-30s: %s\n" "$varname" "$val"
}

# Render boolean with enabled/disabled text
crgl_render_boolean() {
    local varname=$1
    local val=${!1}
    local status=$([ "$val" = "1" ] && echo "enabled" || echo "disabled")
    printf "%-30s: %s\n" "$varname" "$status"
}

# Render list with each item on new line
crgl_render_list() {
    local varname=$1
    local val=${!1}
    echo "$varname:"
    for item in $val; do
        echo "    $item"
    done
}


# eof
