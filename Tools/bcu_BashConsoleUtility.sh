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
#
# Bash Console Utility Library

# Color codes
BCU_RED='\033[0;31m'
BCU_GREEN='\033[0;32m'
BCU_YELLOW='\033[0;33m'
BCU_BLUE='\033[0;34m'
BCU_RESET='\033[0m'

# Core output functions
bcu_info() {
    echo "$@" >&2
}

bcu_trace() {
    [[ "${BCU_TRACE:-0}" == "1" ]] && echo "${BCU_BLUE}TRACE:${BCU_RESET} $@" >&2
}

bcu_warn() {
    echo -e "${BCU_YELLOW}WARNING:${BCU_RESET} $@" >&2
}

bcu_die() {
    echo -e "${BCU_RED}ERROR:${BCU_RESET} $@" >&2
    exit 1
}

# Progress indicators
bcu_start() {
    echo -e "${BCU_BLUE}===${BCU_RESET} $@ ${BCU_BLUE}===${BCU_RESET}" >&2
}

bcu_step() {
    echo -e "${BCU_YELLOW}---${BCU_RESET} $@" >&2
}

bcu_pass() {
    echo -e "${BCU_GREEN}?${BCU_RESET} $@" >&2
}

# Utility functions
bcu_require_var() {
    local varname="$1"
    if [[ -z "${!varname}" ]]; then
        bcu_die "Required variable $varname is not set"
    fi
}

bcu_require_file() {
    local filepath="$1"
    if [[ ! -f "$filepath" ]]; then
        bcu_die "Required file not found: $filepath"
    fi
}

bcu_require_dir() {
    local dirpath="$1"
    if [[ ! -d "$dirpath" ]]; then
        bcu_die "Required directory not found: $dirpath"
    fi
}

# eof

