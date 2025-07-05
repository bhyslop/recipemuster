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

# Multiple inclusion guard
[[ -n "${ZBCU_INCLUDED:-}" ]] && return 0
ZBCU_INCLUDED=1

# Color codes
zbcu_color() {
  # More robust terminal detection for Cygwin and other environments
  if [[ -n "$TERM" && "$TERM" != "dumb" ]]; then
    printf '\033[%sm' "$1"
  else
    printf ''
  fi
}
ZBCU_RED=$(    zbcu_color '0;31' )
ZBCU_GREEN=$(  zbcu_color '0;32' )
ZBCU_YELLOW=$( zbcu_color '0;33' )
ZBCU_BLUE=$(   zbcu_color '0;34' )
ZBCU_RESET=$(  zbcu_color '0'    )

# Help mode: if ZBCU_HELP_CMD is empty, not in help mode; if set, in help mode for that command

# Global context variable for error messages
ZBCU_CONTEXT=""

# Help mode flag
ZBCU_DOC_MODE=false

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
    local context="${ZBCU_CONTEXT:-}"
    echo -e "${ZBCU_RED}ERROR:${ZBCU_RESET} [$context] $@"
    exit 1
}

bcu_set_context() {
    local old_context="$ZBCU_CONTEXT"
    ZBCU_CONTEXT="$1"
    echo "$old_context"
}

bcu_get_context() {
    echo "$ZBCU_CONTEXT"
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
    [[ "$ZBCU_DOC_MODE" == "true" ]] && return 0  # Help mode
    return 1  # Normal mode
}


bcu_doc_brief() {
    set +x
    zbcu_do_execute || return 0
    echo "  ${ZBCU_CONTEXT}"
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
    zbcu_do_execute || return 0
    return 1
}

bcu_set_doc_mode() {
    ZBCU_DOC_MODE=true
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

