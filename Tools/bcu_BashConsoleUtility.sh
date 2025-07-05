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
ZBCU_RED='\033[0;31m'
ZBCU_GREEN='\033[0;32m'
ZBCU_YELLOW='\033[0;33m'
ZBCU_BLUE='\033[0;34m'
ZBCU_RESET='\033[0m'

# Help mode: if ZBCU_HELP_CMD is empty, not in help mode; if set, in help mode for that command
ZBCU_HELP_CMD=""

bcu_info() {
    set +x
    echo "$@"
}

bcu_warn() {
    set +x
    echo -e "${ZBCU_YELLOW}WARNING:${ZBCU_RESET} $@"
}

bcu_die() {
    set +x
    echo -e "${ZBCU_RED}ERROR:${ZBCU_RESET} $@"
    exit 1
}

bcu_step() {
    set +x
    echo -e "${ZBCU_BLUE}===${ZBCU_RESET} $@ ${ZBCU_BLUE}===${ZBCU_RESET}" || bcu_die
}

bcu_pass() {
    set +x
    echo -e "${ZBCU_GREEN}?${ZBCU_RESET} $@" || bcu_die
}

zbcu_do_execute() {
    echo "zbcu_do_execute entered..."
    [[ -z "${ZBCU_HELP_CMD}" ]] || return 0
    echo "zbcu_do_execute exiting with FALSE..."
    return 1
}

bcu_doc_brief() {
    set +x
    echo "bcu_doc_brief entered..."
    zbcu_do_execute && echo "bcu_doc_brief return TRUE" || echo "bcu_doc_brief return FALSE"
    echo "bcu_doc_brief testing..."
    zbcu_do_execute || return 0
    echo "bcu_doc_brief displaying..."
    echo "  ${ZBCU_HELP_CMD}"
    echo "    brief: $1"
}

bcu_doc_lines() {
    set +x
    zbcu_do_execute || return 0
    echo "           $1"
}

bcu_doc_param() {
    set +x
    zbcu_do_execute || return 0
    echo "           $1 - $2"
}


# Idiomatic last step of documentation in the bash api.  
# Usage:
#    bcu_doc_shown || return 0
bcu_doc_shown() {
    zbcu_do_execute
}

bcu_set_help_mode() {
    local cmd_name="${1:-}"
    ZBCU_HELP_CMD="$cmd_name"
}

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

