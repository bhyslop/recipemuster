#!/bin/bash
#
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
# Bash Utility Guide - Always-visible user interaction output
#
# This module provides formatted output for interactive procedures where
# the user MUST see the output to proceed (OAuth flows, manual steps, etc).
# All output goes to stderr and is NOT subject to verbosity control.
#
# Function naming convention encodes color sequence for positional args:
#   t = text (default color)
#   c = command/code (cyan)
#   u = UI element (magenta) - text user sees on screen
#   W = warning (yellow) - uppercase for visual pop
#   E = error (red) - uppercase for visual pop
#   l = link (blue underline, OSC-8 hyperlink) - consumes TWO args
#
# Each letter consumes one positional argument, except 'l' which consumes
# two (display_text then url). This is the only multi-arg letter.
#
# Example: bug_tut "Click the " "Save" " button"
#   Renders: "Click the " + magenta("Save") + " button"
#
# Link arg mapping example for bug_tlt:
#   bug_tlt  "A "  "hallmark"  "https://...#hallmark"  " is a named artifact."
#            ^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
#            t(1)  l(1:text)   l(2:url)                t(1)

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUG_INCLUDED:-}" || return 0
ZBUG_INCLUDED=1

######################################################################
# Internal: Kindle and Sentinel

zbug_kindle() {
  test -z "${ZBUG_KINDLED:-}" || return 0

  # Color support detection
  local z_use_color=0
  if test -z "${NO_COLOR:-}" && test -n "${TERM:-}" && test "${TERM}" != "dumb"; then
    z_use_color=1
  fi

  if test "${z_use_color}" = "1"; then
    readonly ZBUG_R="\033[0m"          # Reset
    readonly ZBUG_T=""                 # Text (default - no color change)
    readonly ZBUG_C="\033[36m"         # Command (cyan)
    readonly ZBUG_U="\033[35m"         # UI element (magenta)
    readonly ZBUG_W="\033[1;33m"       # Warning (bright yellow)
    readonly ZBUG_E="\033[1;31m"       # Error (bright red)
    readonly ZBUG_S="\033[1;37m"       # Section (bright white)
    readonly ZBUG_L="\033[34;4m"       # Link (blue + underline)
  else
    readonly ZBUG_R=""
    readonly ZBUG_T=""
    readonly ZBUG_C=""
    readonly ZBUG_U=""
    readonly ZBUG_W=""
    readonly ZBUG_E=""
    readonly ZBUG_S=""
    readonly ZBUG_L=""
  fi

  readonly ZBUG_KINDLED=1
}

zbug_sentinel() {
  test "${ZBUG_KINDLED:-}" = "1" || { zbug_kindle; }
}

######################################################################
# Internal: Core output

zbug_show() {
  zbug_sentinel
  printf '%b\n' "${1:-}" >&2
}

######################################################################
# Internal: Link fragment builder
#
# Sets ZBUG_LINK_FRAG to a formatted link fragment for embedding in
# combinator output.  Respects BURD_NO_HYPERLINKS fallback.
# Args: display_text url

zbug_link_fragment() {
  zbug_sentinel
  local -r z_text="${1:-}"
  local -r z_url="${2:-}"
  if test -n "${BURD_NO_HYPERLINKS:-}"; then
    ZBUG_LINK_FRAG="${ZBUG_L}${z_text}${ZBUG_R} <${z_url}>"
  else
    ZBUG_LINK_FRAG="${ZBUG_L}\033]8;;${z_url}\033\\\\${z_text}\033]8;;\033\\\\${ZBUG_R}"
  fi
}

######################################################################
# Public: Section headers

bug_section() { zbug_sentinel; printf '%b\n' "${ZBUG_S}${1}${ZBUG_R}" >&2; }
bug_e()       { echo "" >&2; }

######################################################################
# Public: Text combinators
#
# Naming: sequence of t/c/u/W/E describes positional arg colors
# Each letter consumes one positional argument

# Single element (sorted)
bug_c()       { zbug_show "${ZBUG_C}${1}${ZBUG_R}"; }
bug_E()       { zbug_show "${ZBUG_E}${1}${ZBUG_R}"; }
bug_t()       { zbug_show "${1}"; }
bug_u()       { zbug_show "${ZBUG_U}${1}${ZBUG_R}"; }
bug_W()       { zbug_show "${ZBUG_W}${1}${ZBUG_R}"; }

# Two elements (sorted)
bug_ct()      { zbug_show "${ZBUG_C}${1}${ZBUG_R}${2}"; }
bug_tc()      { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}"; }
bug_tE()      { zbug_show "${1}${ZBUG_E}${2}${ZBUG_R}"; }
bug_tu()      { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}"; }
bug_tW()      { zbug_show "${1}${ZBUG_W}${2}${ZBUG_R}"; }
bug_ut()      { zbug_show "${ZBUG_U}${1}${ZBUG_R}${2}"; }

# Three elements (sorted)
bug_tct()     { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}${3}"; }
bug_tcu()     { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}${ZBUG_U}${3}${ZBUG_R}"; }
bug_tEt()     { zbug_show "${1}${ZBUG_E}${2}${ZBUG_R}${3}"; }
bug_tuc()     { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}${ZBUG_C}${3}${ZBUG_R}"; }
bug_tut()     { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}${3}"; }
bug_tWt()     { zbug_show "${1}${ZBUG_W}${2}${ZBUG_R}${3}"; }

# Four elements (sorted)
bug_tctc()    { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}${3}${ZBUG_C}${4}${ZBUG_R}"; }
bug_tctu()    { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}${3}${ZBUG_U}${4}${ZBUG_R}"; }
bug_tcut()    { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}${ZBUG_U}${3}${ZBUG_R}${4}"; }
bug_tuct()    { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}${ZBUG_C}${3}${ZBUG_R}${4}"; }
bug_tutE()    { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}${3}${ZBUG_E}${4}${ZBUG_R}"; }
bug_tutu()    { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}${3}${ZBUG_U}${4}${ZBUG_R}"; }

# Five elements (sorted)
bug_tctct()   { zbug_show "${1}${ZBUG_C}${2}${ZBUG_R}${3}${ZBUG_C}${4}${ZBUG_R}${5}"; }
bug_tutut()   { zbug_show "${1}${ZBUG_U}${2}${ZBUG_R}${3}${ZBUG_U}${4}${ZBUG_R}${5}"; }

######################################################################
# Public: Hyperlinks
#
# bug_link "prefix text" "link text" "url" "suffix text"
# Renders clickable hyperlink with OSC-8 escape sequences

bug_link() {
  zbug_sentinel
  local z_prefix="${1:-}"
  local z_text="${2:-}"
  local z_url="${3:-}"
  local z_suffix="${4:-}"

  # Blue + underline style
  local z_link_style="\033[34;4m"

  if test -n "${BURD_NO_HYPERLINKS:-}"; then
    # Fallback: styled text with URL in angle brackets
    printf '%s%b%s%b <%s>%s\n' \
      "${z_prefix}" "${z_link_style}" "${z_text}" "${ZBUG_R}" "${z_url}" "${z_suffix}" >&2
  else
    # OSC-8 hyperlink with styling
    printf '%s%b\033]8;;%s\033\\%s\033]8;;\033\\%b%s\n' \
      "${z_prefix}" "${z_link_style}" "${z_url}" "${z_text}" "${ZBUG_R}" "${z_suffix}" >&2
  fi
}

######################################################################
# Public: Link combinators
#
# The 'l' letter consumes TWO positional args (display_text, url) and
# renders an OSC-8 terminal hyperlink.  All other letters consume one.
#
# Arg mapping example for bug_tlt:
#   bug_tlt  "A "  "hallmark"  "https://...#hallmark"  " is a named artifact."
#            ^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
#            t(1)  l(1:text)   l(2:url)                t(1)

bug_lt()      { zbug_link_fragment "${1}" "${2}"; zbug_show "${ZBUG_LINK_FRAG}${3}"; }
bug_tl()      { zbug_link_fragment "${2}" "${3}"; zbug_show "${1}${ZBUG_LINK_FRAG}"; }
bug_tlt()     { zbug_link_fragment "${2}" "${3}"; zbug_show "${1}${ZBUG_LINK_FRAG}${4}"; }
bug_tlc()     { zbug_link_fragment "${2}" "${3}"; zbug_show "${1}${ZBUG_LINK_FRAG}${ZBUG_C}${4}${ZBUG_R}"; }

bug_tltlt() {
  zbug_link_fragment "${2}" "${3}"
  local -r z_frag1="${ZBUG_LINK_FRAG}"
  zbug_link_fragment "${5}" "${6}"
  zbug_show "${1}${z_frag1}${4}${ZBUG_LINK_FRAG}${7}"
}

######################################################################
# Internal: Tabtarget fragment builder
#
# Sets ZBUG_TT_FRAG to a resolved tabtarget path in cyan for embedding
# in combinator output.  Resolves colophon via glob.
# Args: colophon

zbug_tabtarget_fragment() {
  zbug_sentinel
  local z_matches=("${BURD_TABTARGET_DIR}"/${1}.*)
  test -e "${z_matches[0]}" || buc_die "bug: no tabtarget for colophon '${1}'"
  ZBUG_TT_FRAG="${ZBUG_C}${z_matches[0]}${ZBUG_R}"
}

######################################################################
# Public: Tabtarget combinators
#
# T consumes one positional arg (colophon), resolves to tabtarget path,
# renders in cyan.  Parallel to l (link) combinator.

bug_T()       { zbug_tabtarget_fragment "${1}"; zbug_show "${ZBUG_TT_FRAG}"; }
bug_tT()      { zbug_tabtarget_fragment "${2}"; zbug_show "${1}${ZBUG_TT_FRAG}"; }
bug_cT()      { zbug_tabtarget_fragment "${2}"; zbug_show "${ZBUG_C}${1}${ZBUG_R}${ZBUG_TT_FRAG}"; }
bug_tlT()     { zbug_link_fragment "${2}" "${3}"; zbug_tabtarget_fragment "${4}"; zbug_show "${1}${ZBUG_LINK_FRAG} ${ZBUG_TT_FRAG}"; }

######################################################################
# Public: User prompts

# bug_prompt "prompt text"
# Displays prompt and reads user input, returns via stdout
bug_prompt() {
  zbug_sentinel
  printf '%s' "${1:-}" >&2
  local z_input
  read -r z_input
  printf '%s' "${z_input}"
}

# bug_prompt_required "prompt text" "error message"
# Like bug_prompt but dies if input is empty
bug_prompt_required() {
  local z_input
  z_input=$(bug_prompt "${1:-}")
  if test -z "${z_input}"; then
    zbug_show "${ZBUG_E}ERROR:${ZBUG_R} ${2:-Input required}"
    return 1
  fi
  printf '%s' "${z_input}"
}

######################################################################
# Public: Critical warnings (box format)

bug_critical() {
  zbug_sentinel
  zbug_show ""
  zbug_show "${ZBUG_E}===============================================${ZBUG_R}"
  zbug_show "${ZBUG_E}  CRITICAL: ${1}${ZBUG_R}"
  zbug_show "${ZBUG_E}===============================================${ZBUG_R}"
  zbug_show ""
}

# eof
