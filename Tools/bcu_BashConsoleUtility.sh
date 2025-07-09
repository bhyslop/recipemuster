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
test -z "${ZBCU_INCLUDED:-}" || return 0
ZBCU_INCLUDED=1

# Color codes
zbcu_color() {
  # More robust terminal detection for Cygwin and other environments
  test -n "$TERM" && test "$TERM" != "dumb" && printf '\033[%sm' "$1" || printf ''
}
ZBCU_BLACK=$(   zbcu_color '1;30' )
ZBCU_RED=$(     zbcu_color '1;31' )
ZBCU_GREEN=$(   zbcu_color '1;32' )
ZBCU_YELLOW=$(  zbcu_color '1;33' )
ZBCU_BLUE=$(    zbcu_color '1;34' )
ZBCU_MAGENTA=$( zbcu_color '1;35' )
ZBCU_CYAN=$(    zbcu_color '1;36' )
ZBCU_WHITE=$(   zbcu_color '1;37' )
ZBCU_RESET=$(   zbcu_color '0'    )

# Global context variable for info and error messages
ZBCU_CONTEXT=""

# Help mode flag
ZBCU_DOC_MODE=false

bcu_info() {
    set -e
    bcu_print 0 "$@"
}

bcu_warn() {
    set -e
    bcu_print 0 "${ZBCU_YELLOW}WARNING:${ZBCU_RESET} $@"
}

bcu_die() {
    set -e
    local context="${ZBCU_CONTEXT:-}"
    bcu_print -1 "${ZBCU_RED}ERROR:${ZBCU_RESET} [$context] $@"
    exit 1
}

bcu_context() {
    ZBCU_CONTEXT="$1"
}

bcu_step() {
    set -e
    echo -e "${ZBCU_BLUE}===${ZBCU_RESET} $@ ${ZBCU_BLUE}===${ZBCU_RESET}" || bcu_die
}

bcu_success() {
    set -e
    echo -e "${ZBCU_GREEN}$@${ZBCU_RESET}" >&2 || bcu_die
}

# Enable trace to stderr safely if supported
zbcu_enable_trace() {
    # Only supported in Bash >= 4.1
    if [[ ${BASH_VERSINFO[0]} -gt 4 ]] || { [[ ${BASH_VERSINFO[0]} -eq 4 ]] && [[ ${BASH_VERSINFO[1]} -ge 1 ]]; }; then
        export BASH_XTRACEFD=2
    fi
    set -x
}

# Disable trace
zbcu_disable_trace() {
    set +x
}

zbcu_do_execute() {
    test "$ZBCU_DOC_MODE" = "true" && return 0 || return 1
}

bcu_doc_env() {
    set -e
    local env_var_name="${1}"
    local env_var_info="${2}"

    echo "  ${ZBCU_MAGENTA}${env_var_name}${ZBCU_RESET}:  ${env_var_info}"
}

bcu_env_done() {
    zbcu_do_execute || return 0
    echo
    return 1
}

ZBCU_USAGE_STRING="UNFILLED"

bcu_doc_brief() {
    set -e
    ZBCU_USAGE_STRING="${ZBCU_CONTEXT}"
    zbcu_do_execute || return 0
    echo
    echo "  ${ZBCU_WHITE}${ZBCU_CONTEXT}${ZBCU_RESET}"
    echo "    brief: $1"
}

bcu_doc_lines() {
    set -e
    zbcu_do_execute || return 0
    echo "           $1"
}

bcu_doc_param() {
    set -e
    ZBCU_USAGE_STRING="${ZBCU_USAGE_STRING} <<$1>>"
    zbcu_do_execute || return 0
    echo "    required: $1 - $2"
}

bcu_doc_oparm() {
    set -e
    ZBCU_USAGE_STRING="${ZBCU_USAGE_STRING} [<<$1>>]"
    zbcu_do_execute || return 0
    echo "    optional: $1 - $2"
}

zbcu_usage() {
    echo -e "    usage: ${ZBCU_CYAN}${ZBCU_USAGE_STRING}${ZBCU_RESET}"
}

# Idiomatic last step of documentation in the bash api.
# Usage:
#    bcu_doc_shown || return 0
bcu_doc_shown() {
    zbcu_do_execute || return 0
    zbcu_usage
    return 1
}

bcu_set_doc_mode() {
    ZBCU_DOC_MODE=true
}

bcu_usage_die() {
    set -e
    local context="${ZBCU_CONTEXT:-}"
    local usage=$(zbcu_usage)
    echo -e "${ZBCU_RED}ERROR:${ZBCU_RESET} $usage"
    exit 1
}

# Multi-line print function with verbosity control
# Sends output to stderr to avoid interfering with stdout returns
bcu_print() {
    local min_verbosity="$1"
    shift

    # Always print if min_verbosity is -1, otherwise check BCU_VERBOSE
    if [ "$min_verbosity" -eq -1 ] || [ "${BCU_VERBOSE:-0}" -ge "$min_verbosity" ]; then
        while [ $# -gt 0 ]; do
            echo "$1" >&2
            shift
        done
    fi
}

# Die if condition is true (non-zero)
# Usage: bcu_die_if <condition> <message1> [<message2> ...]
bcu_die_if() {
    local condition="$1"
    shift

    test "$condition" -ne 0 || return 0

    set -e
    local context="${ZBCU_CONTEXT:-}"
    bcu_print -1 "${ZBCU_RED}ERROR:${ZBCU_RESET} [$context] $1"
    shift
    bcu_print -1 "$@"
    exit 1
}

# Die unless condition is true (zero)
# Usage: bcu_die_unless <condition> <message1> [<message2> ...]
bcu_die_unless() {
    local condition="$1"
    shift

    test "$condition" -eq 0 || return 0

    set -e
    local context="${ZBCU_CONTEXT:-}"
    bcu_print -1 "${ZBCU_RED}ERROR:${ZBCU_RESET} [$context] $1"
    shift
    bcu_print -1 "$@"
    exit 1
}


# eof

