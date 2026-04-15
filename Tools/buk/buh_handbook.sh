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
# Bash Utility Handbook - Always-visible user interaction output
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
# Example: buh_tut "Click the " "Save" " button"
#   Renders: "Click the " + magenta("Save") + " button"
#
# Link arg mapping example for buh_tlt:
#   buh_tlt  "A "  "hallmark"  "https://...#hallmark"  " is a named artifact."
#            ^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
#            t(1)  l(1:text)   l(2:url)                t(1)

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUH_INCLUDED:-}" || return 0
ZBUH_INCLUDED=1

######################################################################
# Internal: Kindle and Sentinel

zbuh_kindle() {
  test -z "${ZBUH_KINDLED:-}" || return 0

  # Color support detection
  local z_use_color=0
  if test -z "${NO_COLOR:-}" && test -n "${TERM:-}" && test "${TERM}" != "dumb"; then
    z_use_color=1
  fi

  if test "${z_use_color}" = "1"; then
    readonly ZBUH_R="\033[0m"          # Reset
    readonly ZBUH_T=""                 # Text (default - no color change)
    readonly ZBUH_C="\033[36m"         # Command (cyan)
    readonly ZBUH_U="\033[35m"         # UI element (magenta)
    readonly ZBUH_W="\033[1;33m"       # Warning (bright yellow)
    readonly ZBUH_E="\033[1;31m"       # Error (bright red)
    readonly ZBUH_S="\033[1;37m"       # Section (bright white)
    readonly ZBUH_L="\033[34;4m"       # Link (blue + underline)
  else
    readonly ZBUH_R=""
    readonly ZBUH_T=""
    readonly ZBUH_C=""
    readonly ZBUH_U=""
    readonly ZBUH_W=""
    readonly ZBUH_E=""
    readonly ZBUH_S=""
    readonly ZBUH_L=""
  fi

  # Mutable kindle state — step counters, body indent, step format
  z_buh_step1_n=0
  z_buh_step2_n=0
  z_buh_body_indent=""
  z_buh_step_prefix=""
  z_buh_step_separator=". "

  readonly ZBUH_KINDLED=1
}

zbuh_sentinel() {
  test "${ZBUH_KINDLED:-}" = "1" || { zbuh_kindle; }
}

######################################################################
# Internal: Core output

zbuh_show() {
  zbuh_sentinel
  printf '%b\n' "${z_buh_body_indent}${1:-}" >&2
}

######################################################################
# Internal: Link fragment builder
#
# Sets ZBUH_LINK_FRAG to a formatted link fragment for embedding in
# combinator output.  Respects BURD_NO_HYPERLINKS fallback.
# Args: display_text url

zbuh_link_fragment() {
  zbuh_sentinel
  local -r z_text="${1:-}"
  local -r z_url="${2:-}"
  if test -n "${BURD_NO_HYPERLINKS:-}"; then
    ZBUH_LINK_FRAG="${ZBUH_L}${z_text}${ZBUH_R} <${z_url}>"
  else
    ZBUH_LINK_FRAG="${ZBUH_L}\033]8;;${z_url}\033\\\\${z_text}\033]8;;\033\\\\${ZBUH_R}"
  fi
}

######################################################################
# Public: Section headers and numbered steps

buh_section() { zbuh_sentinel; z_buh_body_indent=""; printf '%b\n' "${ZBUH_S}${1}${ZBUH_R}" >&2; }
buh_e()       { echo "" >&2; }

buh_step_style() {
  zbuh_sentinel
  z_buh_step_prefix="${1:-}"
  z_buh_step_separator="${2:-". "}"
}

buh_step1() {
  zbuh_sentinel
  z_buh_step1_n=$((z_buh_step1_n + 1))
  z_buh_step2_n=0
  z_buh_body_indent="   "
  printf '%b\n' "${ZBUH_S}${z_buh_step_prefix}${z_buh_step1_n}${z_buh_step_separator}${1}${ZBUH_R}" >&2
}

buh_step2() {
  zbuh_sentinel
  z_buh_step2_n=$((z_buh_step2_n + 1))
  z_buh_body_indent="      "
  printf '%b\n' "   ${ZBUH_S}${z_buh_step_prefix}${z_buh_step1_n}.${z_buh_step2_n}${z_buh_step_separator}${1}${ZBUH_R}" >&2
}

######################################################################
# Public: Text combinators
#
# Naming: sequence of t/c/u/W/E describes positional arg colors
# Each letter consumes one positional argument

# Single element (sorted)
buh_c()       { zbuh_show "${ZBUH_C}${1}${ZBUH_R}"; }
buh_E()       { zbuh_show "${ZBUH_E}${1}${ZBUH_R}"; }
buh_t()       { zbuh_show "${1}"; }
buh_u()       { zbuh_show "${ZBUH_U}${1}${ZBUH_R}"; }
buh_W()       { zbuh_show "${ZBUH_W}${1}${ZBUH_R}"; }

# Two elements (sorted)
buh_ct()      { zbuh_show "${ZBUH_C}${1}${ZBUH_R}${2}"; }
buh_tc()      { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}"; }
buh_tE()      { zbuh_show "${1}${ZBUH_E}${2}${ZBUH_R}"; }
buh_tu()      { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}"; }
buh_tW()      { zbuh_show "${1}${ZBUH_W}${2}${ZBUH_R}"; }
buh_ut()      { zbuh_show "${ZBUH_U}${1}${ZBUH_R}${2}"; }

# Three elements (sorted)
buh_tct()     { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}${3}"; }
buh_tcu()     { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}${ZBUH_U}${3}${ZBUH_R}"; }
buh_tEt()     { zbuh_show "${1}${ZBUH_E}${2}${ZBUH_R}${3}"; }
buh_tuc()     { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}${ZBUH_C}${3}${ZBUH_R}"; }
buh_tut()     { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}${3}"; }
buh_tWt()     { zbuh_show "${1}${ZBUH_W}${2}${ZBUH_R}${3}"; }

# Four elements (sorted)
buh_tctc()    { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}${3}${ZBUH_C}${4}${ZBUH_R}"; }
buh_tctu()    { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}${3}${ZBUH_U}${4}${ZBUH_R}"; }
buh_tcut()    { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}${ZBUH_U}${3}${ZBUH_R}${4}"; }
buh_tuct()    { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}${ZBUH_C}${3}${ZBUH_R}${4}"; }
buh_tutE()    { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}${3}${ZBUH_E}${4}${ZBUH_R}"; }
buh_tutu()    { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}${3}${ZBUH_U}${4}${ZBUH_R}"; }

# Five elements (sorted)
buh_tctct()   { zbuh_show "${1}${ZBUH_C}${2}${ZBUH_R}${3}${ZBUH_C}${4}${ZBUH_R}${5}"; }
buh_tutut()   { zbuh_show "${1}${ZBUH_U}${2}${ZBUH_R}${3}${ZBUH_U}${4}${ZBUH_R}${5}"; }

######################################################################
# Public: Hyperlinks
#
# buh_link "prefix text" "link text" "url" "suffix text"
# Renders clickable hyperlink with OSC-8 escape sequences

buh_link() {
  zbuh_sentinel
  local z_prefix="${1:-}"
  local z_text="${2:-}"
  local z_url="${3:-}"
  local z_suffix="${4:-}"

  # Blue + underline style
  local z_link_style="\033[34;4m"

  if test -n "${BURD_NO_HYPERLINKS:-}"; then
    # Fallback: styled text with URL in angle brackets
    printf '%s%s%b%s%b <%s>%s\n' \
      "${z_buh_body_indent}" "${z_prefix}" "${z_link_style}" "${z_text}" "${ZBUH_R}" "${z_url}" "${z_suffix}" >&2
  else
    # OSC-8 hyperlink with styling
    printf '%s%s%b\033]8;;%s\033\\%s\033]8;;\033\\%b%s\n' \
      "${z_buh_body_indent}" "${z_prefix}" "${z_link_style}" "${z_url}" "${z_text}" "${ZBUH_R}" "${z_suffix}" >&2
  fi
}

######################################################################
# Public: Link combinators
#
# The 'l' letter consumes TWO positional args (display_text, url) and
# renders an OSC-8 terminal hyperlink.  All other letters consume one.
#
# Arg mapping example for buh_tlt:
#   buh_tlt  "A "  "hallmark"  "https://...#hallmark"  " is a named artifact."
#            ^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^
#            t(1)  l(1:text)   l(2:url)                t(1)

buh_lt()      { zbuh_link_fragment "${1}" "${2}"; zbuh_show "${ZBUH_LINK_FRAG}${3}"; }
buh_tl()      { zbuh_link_fragment "${2}" "${3}"; zbuh_show "${1}${ZBUH_LINK_FRAG}"; }
buh_tlt()     { zbuh_link_fragment "${2}" "${3}"; zbuh_show "${1}${ZBUH_LINK_FRAG}${4}"; }
buh_tlc()     { zbuh_link_fragment "${2}" "${3}"; zbuh_show "${1}${ZBUH_LINK_FRAG}${ZBUH_C}${4}${ZBUH_R}"; }

buh_tltlt() {
  zbuh_link_fragment "${2}" "${3}"
  local -r z_frag1="${ZBUH_LINK_FRAG}"
  zbuh_link_fragment "${5}" "${6}"
  zbuh_show "${1}${z_frag1}${4}${ZBUH_LINK_FRAG}${7}"
}

buh_tltltlt() {
  zbuh_link_fragment "${2}" "${3}"
  local -r z_frag1="${ZBUH_LINK_FRAG}"
  zbuh_link_fragment "${5}" "${6}"
  local -r z_frag2="${ZBUH_LINK_FRAG}"
  zbuh_link_fragment "${8}" "${9}"
  zbuh_show "${1}${z_frag1}${4}${z_frag2}${7}${ZBUH_LINK_FRAG}${10}"
}

buh_tltltltlt() {
  zbuh_link_fragment "${2}" "${3}"
  local -r z_frag1="${ZBUH_LINK_FRAG}"
  zbuh_link_fragment "${5}" "${6}"
  local -r z_frag2="${ZBUH_LINK_FRAG}"
  zbuh_link_fragment "${8}" "${9}"
  local -r z_frag3="${ZBUH_LINK_FRAG}"
  zbuh_link_fragment "${11}" "${12}"
  zbuh_show "${1}${z_frag1}${4}${z_frag2}${7}${z_frag3}${10}${ZBUH_LINK_FRAG}${13}"
}

buh_tltTtT() {
  zbuh_link_fragment "${2}" "${3}"
  local -r z_frag1="${ZBUH_LINK_FRAG}"
  zbuh_tabtarget_fragment "${5}"
  local -r z_frag2="${ZBUH_TT_FRAG}"
  zbuh_tabtarget_fragment "${7}"
  zbuh_show "${1}${z_frag1}${4}${z_frag2}${6}${ZBUH_TT_FRAG}${8}"
}

######################################################################
# Internal: Tabtarget fragment builder
#
# Sets ZBUH_TT_FRAG to a resolved tabtarget path in cyan for embedding
# in combinator output.  Resolves colophon via glob.
# Args: colophon

zbuh_tabtarget_fragment() {
  zbuh_sentinel
  local z_matches=("${BURD_TABTARGET_DIR}"/${1}.*)
  test -e "${z_matches[0]}" || buc_die "buh: no tabtarget for colophon '${1}'"
  ZBUH_TT_FRAG="${ZBUH_C}${z_matches[0]}${ZBUH_R}"
}

# Imprint tabtarget fragment: resolves colophon + imprint to a specific
# imprint tabtarget.  Glob: ${colophon}.*.${imprint}.sh
# Args: colophon imprint

zbuh_imprint_fragment() {
  zbuh_sentinel
  local z_matches=("${BURD_TABTARGET_DIR}"/${1}.*.${2}.sh)
  test -e "${z_matches[0]}" || buc_die "buh: no tabtarget for colophon '${1}' imprint '${2}'"
  ZBUH_TT_FRAG="${ZBUH_C}${z_matches[0]}${ZBUH_R}"
}

######################################################################
# Public: Tabtarget combinators
#
# T consumes one positional arg (colophon), resolves to tabtarget path,
# renders in cyan.  Parallel to l (link) combinator.
# I consumes two positional args (colophon, imprint), resolves to a
# specific imprint tabtarget.
# Trailing c after T or I appends cyan text (arguments) to the resolved
# path, keeping the whole command visually unified.

buh_T()       { zbuh_tabtarget_fragment "${1}"; zbuh_show "${ZBUH_TT_FRAG}"; }
buh_tT()      { zbuh_tabtarget_fragment "${2}"; zbuh_show "${1}${ZBUH_TT_FRAG}"; }
buh_tTc()     { zbuh_tabtarget_fragment "${2}"; zbuh_show "${1}${ZBUH_TT_FRAG}${ZBUH_C}${3}${ZBUH_R}"; }
buh_tI()      { zbuh_imprint_fragment "${2}" "${3}"; zbuh_show "${1}${ZBUH_TT_FRAG}"; }
buh_tIc()     { zbuh_imprint_fragment "${2}" "${3}"; zbuh_show "${1}${ZBUH_TT_FRAG}${ZBUH_C}${4}${ZBUH_R}"; }
buh_cT()      { zbuh_tabtarget_fragment "${2}"; zbuh_show "${ZBUH_C}${1}${ZBUH_R}${ZBUH_TT_FRAG}"; }
buh_tlT()     { zbuh_link_fragment "${2}" "${3}"; zbuh_tabtarget_fragment "${4}"; zbuh_show "${1}${ZBUH_LINK_FRAG} ${ZBUH_TT_FRAG}"; }
buh_tltT()    { zbuh_link_fragment "${2}" "${3}"; zbuh_tabtarget_fragment "${5}"; zbuh_show "${1}${ZBUH_LINK_FRAG}${4}${ZBUH_TT_FRAG}"; }

######################################################################
# Public: Index renderers
#
# Group-level handbook indexes shared across kits.
# Each function renders one kit's handbook group tops.

buh_index_buk() {
  buh_section  "Generic OS Procedures (Bash Utility Kit)"
  buh_t        "  Project-independent OS-level mechanisms."
  buh_tT       "  Windows:  " "buw-hw"
}

######################################################################
# Public: Pre-composed line output
#
# buh_line string
#   Indent-aware (respects z_buh_body_indent from step context).
#   Designed for lines pre-composed from yelp capture fragments
#   via direct bash variable interpolation:
#     buh_line "A ${RBYC_DEPOT} is where images live."
#   Yelps are pre-rendered (literal ESC bytes) and must not contain %.

buh_line() {
  zbuh_sentinel
  buyf_format_yawp "${BUYC_RESET}" "${1:-}"
  printf '%b\n' "${z_buh_body_indent}${z_buym_format}" >&2
}

######################################################################
# Public: Semantic line functions
#
# Each routes through buyf_format_yawp with a BUYC_* ambient color,
# then prints via indent-aware printf.  Diastema markers in the string
# are resolved to ANSI/OSC-8 at display time.
#
# buh_line   — prose default (BUYC_RESET ambient)
# buh_code   — command/code (BUYC_CYAN ambient)
# buh_warn   — warning (BUYC_BRIGHT_YELLOW ambient)
# buh_error  — error (BUYC_BRIGHT_RED ambient)

buh_code() {
  zbuh_sentinel
  buyf_format_yawp "${BUYC_CYAN}" "${1:-}"
  printf '%b\n' "${z_buh_body_indent}${z_buym_format}" >&2
}

buh_warn() {
  zbuh_sentinel
  buyf_format_yawp "${BUYC_BRIGHT_YELLOW}" "${1:-}"
  printf '%b\n' "${z_buh_body_indent}${z_buym_format}" >&2
}

buh_error() {
  zbuh_sentinel
  buyf_format_yawp "${BUYC_BRIGHT_RED}" "${1:-}"
  printf '%b\n' "${z_buh_body_indent}${z_buym_format}" >&2
}

######################################################################
# Public: User prompts

# buh_prompt "prompt text"
# Displays prompt and reads user input, returns via stdout
buh_prompt() {
  zbuh_sentinel
  printf '%s' "${1:-}" >&2
  local z_input
  read -r z_input
  printf '%s' "${z_input}"
}

# buh_prompt_required "prompt text" "error message"
# Like buh_prompt but dies if input is empty
buh_prompt_required() {
  local z_input
  z_input=$(buh_prompt "${1:-}")
  if test -z "${z_input}"; then
    zbuh_show "${ZBUH_E}ERROR:${ZBUH_R} ${2:-Input required}"
    return 1
  fi
  printf '%s' "${z_input}"
}

######################################################################
# Public: Critical warnings (box format)

buh_critical() {
  zbuh_sentinel
  zbuh_show ""
  zbuh_show "${ZBUH_E}===============================================${ZBUH_R}"
  zbuh_show "${ZBUH_E}  CRITICAL: ${1}${ZBUH_R}"
  zbuh_show "${ZBUH_E}===============================================${ZBUH_R}"
  zbuh_show ""
}

# eof
