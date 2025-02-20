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
