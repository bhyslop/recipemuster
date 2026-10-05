#!/bin/bash

# Copyright 2024 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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

# Configuration Regime Render Library
# Provides core validation functions for configuration regime validators


# Core error handling
crgr_die() {
    echo "ERROR: $*" >&2
    exit 1
}

crgr_render_header() {
    echo "=== $1 ==="
}

crgr_render_group() {
    echo "--- $1 ---"
}

# Render a single value with label
crgr_render_value() {
    local z_varname=$1
    local z_val=${!1}
    printf "%-30s: %s\n" "$z_varname" "$z_val"
}

# Render boolean with enabled/disabled text
crgr_render_boolean() {
    local z_varname=$1
    local z_val=${!1}
    local z_status=$([ "$z_val" = "1" ] && echo "enabled" || echo "disabled")
    printf "%-30s: %s\n" "$z_varname" "$z_status"
}

# Render list with each item on new line
crgr_render_list() {
    local z_varname=$1
    local z_val=${!1}
    echo "$z_varname:"
    for item in $z_val; do
        echo "    $item"
    done
}


# eof
