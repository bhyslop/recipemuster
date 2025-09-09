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

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBCU_INCLUDED:-}" || return 0
ZBCU_INCLUDED=1

# Color codes
zbcu_color() {
  # More robust terminal detection for Cygwin and other environments
  test -n "${TERM}" && test "${TERM}" != "dumb" && printf '\033[%sm' "$1" || printf ''
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


######################################################################
# Internal logging helpers

# Usage: zbcu_make_tag <depth> "<label>"
#   Computes ZBCU_TAG for the given stack depth/label (no I/O).
# Usage: zbcu_tag_args <depth> "<label>" [arg...]
#   Computes ZBCU_TAG and logs args directly to the transcript.
#
# Bash stack quirk:
#   BASH_SOURCE[i] / FUNCNAME[i] index the current frame,
#   but BASH_LINENO[i] reports the line where FUNCNAME[i+1] was called.
#   For depth=N callers:
#       file = BASH_SOURCE[N]
#       line = BASH_LINENO[N-1]
# Usage: zbcu_make_tag <depth> "<label>"
#   Computes ZBCU_TAG for the given stack depth/label (no I/O).
#   Note: With depth=0 or too-deep stacks, file/line may be empty (by design).
zbcu_make_tag() {
  local -i z_d="${1:-1}"
  local    z_label="${2:-}"
  local    z_file="${BASH_SOURCE[z_d]##*/}"
  local    z_line="${BASH_LINENO[z_d-1]}"
  ZBCU_TAG="${z_label}${z_file}:${z_line}: "
}

zbcu_tag_args() {
  local z_d="${1:-1}"
  shift
  local z_label="${1:-}"
  shift
  zbcu_make_tag "${z_d}" "${z_label}"
  printf '%s\n' "$@" | zbcu_log "${ZBCU_TAG}" " ---- "
}

######################################################################
# Public logging wrappers

bcu_log_args() { zbcu_tag_args 3 "bcu_log_args " "$@"; }
bcu_log_pipe() { zbcu_make_tag 3 "bcu_log_pipe "; zbcu_log "${ZBCU_TAG}" " ---- "; }

bcu_step()     { zbcu_tag_args 3 "bcu_step     " "$@"; zbcu_print 0 "${ZBCU_WHITE}$*${ZBCU_RESET}"; }
bcu_code()     { zbcu_tag_args 3 "bcu_code     " "$@"; zbcu_print 0 "${ZBCU_CYAN}$*${ZBCU_RESET}"; }
bcu_info()     { zbcu_tag_args 3 "bcu_info     " "$@"; zbcu_print 1 "$@"; }
bcu_debug()    { zbcu_tag_args 3 "bcu_debug    " "$@"; zbcu_print 2 "$@"; }
bcu_trace()    { zbcu_tag_args 3 "bcu_trace    " "$@"; zbcu_print 3 "$@"; }
bcu_warn()     { zbcu_tag_args 3 "bcu_warn     " "$@"; zbcu_print 0 "${ZBCU_YELLOW}WARNING:${ZBCU_RESET} $*"; }
bcu_success()  { zbcu_tag_args 3 "bcu_success  " "$@"; printf '%b\n' "${ZBCU_GREEN}$*${ZBCU_RESET}" >&2 || bcu_die; }
bcu_die() {
  zbcu_tag_args                3 "bcu_die      " "ERROR: [${ZBCU_CONTEXT:-}] $*"
  zbcu_print -1          "${ZBCU_RED}ERROR:${ZBCU_RESET} [${ZBCU_CONTEXT:-}] $*"
  exit 1
}

bcu_context() {
  ZBCU_CONTEXT="$1"
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
  test "${ZBCU_DOC_MODE}" = "true" && return 0 || return 1
}

bcu_doc_env() {
  set -e

  local env_var_name="${1}"
  local env_var_info="${2}"

  # Trim trailing spaces from variable name
  env_var_name="${env_var_name%% *}"

  # In doc mode, show documentation first
  if zbcu_do_execute; then
    echo "  ${ZBCU_MAGENTA}${1}${ZBCU_RESET}:  ${env_var_info}"
  fi

  # Always check if variable is set (using trimmed name)
  eval "test -n \"\${${env_var_name}:-}\"" || bcu_warn "${env_var_name} is not set"
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
zbcu_print() {
  local min_verbosity="$1"
  shift

  # Always print if min_verbosity is -1, otherwise check BCU_VERBOSE
  if [ "${min_verbosity}" -eq -1 ] || [ "${BCU_VERBOSE:-0}" -ge "${min_verbosity}" ]; then
    while [ $# -gt 0 ]; do
      echo "$1" >&2
      shift
    done
  fi
}

# Core logging implementation - always reads from stdin
zbcu_log() {
  test -n "${BDU_TRANSCRIPT:-}" || return 0

  local z_prefix="$1"
  local z_rest_prefix="$2"
  local z_outfile="${BDU_TRANSCRIPT}"

  while IFS= read -r z_line; do
    printf '%s%s\n' "${z_prefix}" "${z_line}" >> "${z_outfile}"
    z_prefix="${z_rest_prefix}"
  done
}


# Die if condition is true (non-zero)
# Usage: bcu_die_if <condition> <message1> [<message2> ...]
bcu_die_if() {
  local condition="$1"
  shift

  test "${condition}" -ne 0 || return 0

  set -e
  local context="${ZBCU_CONTEXT:-}"
  zbcu_print -1 "${ZBCU_RED}ERROR:${ZBCU_RESET} [$context] $1"
  shift
  zbcu_print -1 "$@"
  exit 1
}

# Die unless condition is true (zero)
# Usage: bcu_die_unless <condition> <message1> [<message2> ...]
bcu_die_unless() {
  local condition="$1"
  shift

  test "${condition}" -eq 0 || return 0

  set -e
  local context="${ZBCU_CONTEXT:-}"
  zbcu_print -1 "${ZBCU_RED}ERROR:${ZBCU_RESET} [$context] $1"
  shift
  zbcu_print -1 "$@"
  exit 1
}

zbcu_show_help() {
  local prefix="$1"
  local title="$2"
  local env_func="$3"

  echo "$title"
  echo

  if [ -n "$env_func" ]; then
    echo "Environment Variables:"
    "$env_func"
    echo
  fi

  echo "Commands:"

  for cmd in $(declare -F | grep -E "^declare -f ${prefix}[a-z][a-z0-9_]*$" | cut -d' ' -f3); do
    bcu_context "$cmd"
    "$cmd"
  done
}

bcu_require() {
  local prompt="$1"
  local required_value="$2"

  echo -e "${ZBCU_YELLOW}${prompt}${ZBCU_RESET}"
  read -p "Type ${required_value}: " input
  test "$input" = "$required_value" || bcu_die "prompt not confirmed."
}

bcu_execute() {
  set -e
  local prefix="$1"
  local title="$2"
  local env_func="$3"
  local command="${4:-}"
  shift 3; [ -n "$command" ] && shift || true

  export BCU_VERBOSE="${BCU_VERBOSE:-0}"

  # Enable bash trace to stderr if BCU_VERBOSE is 3 or higher and bash >= 4.1
  if [[ "${BCU_VERBOSE}" -ge 3 ]]; then
    if [[ "${BASH_VERSINFO[0]}" -gt 4 ]] || [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 1 ]]; then
      export PS4='+ ${BASH_SOURCE##*/}:${LINENO}: '
      export BASH_XTRACEFD=2
      set -x
    fi
  fi

  # Validate and execute command if named, else show help
  if [ -n       "${command}" ]            &&\
    declare -F  "${command}" >/dev/null   &&\
    echo        "${command}" | grep -q "^${prefix}[a-z][a-z0-9_]*$"; then
    bcu_context "${command}"
    [ -n "${env_func}" ] && "${env_func}"
    "${command}" "$@"
  else
    test -z "${command}" || bcu_warn "Unknown command: ${command}"
    bcu_set_doc_mode
    zbcu_show_help "${prefix}" "${title}" "${env_func}"
    echo
    exit 1
  fi
}

# --- Hyperlink helpers (OSC-8), falls back to plain text when disabled ---
# Disable with: export BDU_NO_HYPERLINKS=1
zbcu_hyperlink() {
  local z_text="${1:-}"
  local z_url="${2:-}"

  # ANSI codes for blue underlined text
  local z_blue_underline='\033[34m\033[4m'
  local z_reset='\033[0m'

  if [ -n "${BDU_NO_HYPERLINKS:-}" ]; then
    # Fallback: blue underlined text with URL in angle brackets
    printf '%s%s%s <%s>' "${z_blue_underline}" "${z_text}" "${z_reset}" "${z_url}"
    return 0
  fi

  # OSC-8 with blue underline formatting
  printf '%s\033]8;;%s\033\\%s\033]8;;\033\\%s' \
    "${z_blue_underline}" "${z_url}" "${z_text}" "${z_reset}"
}

# bcu_link "Prefix text" "Link text" "URL"  (prints to stderr like other user-visible messages)
bcu_link() {
  local z_prefix="${1:-}"
  local z_text="${2:-}"
  local z_url="${3:-}"

  zbcu_tag_args 3 "bcu_link    " "${z_prefix} ${z_text} -> ${z_url}"

  # Always show at verbosity >= 0 (same visibility as bcu_step)
  if [ "${BCU_VERBOSE:-0}" -ge 0 ]; then
    # Print prefix text if provided
    test -n "${z_prefix}" && printf '%s ' "${z_prefix}" >&2

    # Print formatted hyperlink
    zbcu_hyperlink "${z_text}" "${z_url}" >&2
    echo >&2
  fi
}

# eof
