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
# BUK Presentation Regime - Shared rendering utilities for regime CLI modules
#
# Provides section-based rendering with gate-aware suppression and
# terminal-adaptive layouts.  Each regime CLI (rbrn_cli, rbrv_cli)
# sources this module and calls the public functions to compose its
# own render output.  Formatting mechanics are shared; editorial
# decisions (section ordering, grouping) stay manual per regime CLI.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBUPR_SOURCED:-}" || buc_die "Module bupr multiply sourced - check sourcing hierarchy"
ZBUPR_SOURCED=1

######################################################################
# Internal Functions (zbupr_*)

zbupr_kindle() {
  test -z "${ZBUPR_KINDLED:-}" || buc_die "Module bupr already kindled"

  # Terminal layout from BURD dispatch (set in bul_launcher before pipe)
  ZBUPR_TERM_COLS=${BURD_TERM_COLS:-80}
  if test "${ZBUPR_TERM_COLS}" -ge 120; then
    ZBUPR_LAYOUT=single
  else
    ZBUPR_LAYOUT=double
  fi


  # Section state
  ZBUPR_SECTION_ACTIVE=1
  ZBUPR_SECTION_GATE_DESC=""
  ZBUPR_SECTION_SUPPRESSED=()

  ZBUPR_KINDLED=1
}

zbupr_sentinel() {
  test "${ZBUPR_KINDLED:-}" = "1" || buc_die "Module bupr not kindled - call zbupr_kindle first"
}

######################################################################
# Public Functions (bupr_*)

# bupr_section_begin TITLE [GATE_VAR GATE_VALUE]
#
# Begin a render section.  Prints section header.
# Optional gate: evaluates ${!GATE_VAR} against GATE_VALUE.
# If gate not satisfied, prints collapsed reminder and suppresses
# subsequent bupr_section_item calls until bupr_section_end.
bupr_section_begin() {
  zbupr_sentinel
  local z_title="$1"
  local z_gate_var=${2:-}
  local z_gate_value=${3:-}

  ZBUPR_SECTION_SUPPRESSED=()
  ZBUPR_SECTION_GATE_DESC=""

  if test -n "${z_gate_var}"; then
    local z_actual=${!z_gate_var:-}
    if test "${z_actual}" = "${z_gate_value}"; then
      ZBUPR_SECTION_ACTIVE=1
    else
      ZBUPR_SECTION_ACTIVE=0
      ZBUPR_SECTION_GATE_DESC="${z_gate_var}=${z_actual}"
    fi
  else
    ZBUPR_SECTION_ACTIVE=1
  fi

  if test "${ZBUPR_SECTION_ACTIVE}" = 1; then
    if test -n "${z_gate_var}"; then
      printf "${ZBUC_YELLOW}%-34s${ZBUC_RESET} ${ZBUC_GREEN}(since %s=%s)${ZBUC_RESET}\n" \
        "${z_title}" "${z_gate_var}" "${z_gate_value}"
    else
      printf "${ZBUC_YELLOW}%s${ZBUC_RESET}\n" "${z_title}"
    fi
  else
    printf "${ZBUC_YELLOW}%-34s${ZBUC_RESET} ${ZBUC_GREEN}(%s)${ZBUC_RESET}\n" "${z_title}" "${ZBUPR_SECTION_GATE_DESC}"
  fi
}

# bupr_section_end
#
# End a render section.  Resets section state for next section.
bupr_section_end() {
  zbupr_sentinel
  ZBUPR_SECTION_ACTIVE=1
  ZBUPR_SECTION_GATE_DESC=""
  ZBUPR_SECTION_SUPPRESSED=()
}

# bupr_item VARNAME TYPE REQ_STATUS DESCRIPTION
#
# Render one regime field outside any section.
# Same layout as bupr_section_item but ignores section state.
bupr_item() {
  zbupr_sentinel
  zbupr_render_field "$@"
}

# bupr_section_item VARNAME TYPE REQ_STATUS DESCRIPTION
#
# Render one regime field within a section.
#   VARNAME:     unquoted regime variable name (e.g., RBRN_ENTRY_MODE)
#   TYPE:        unquoted type badge (xname, string, fqin, port, ipv4, etc.)
#   REQ_STATUS:  unquoted — req, opt, or cond
#   DESCRIPTION: quoted human prose
#
# If section is collapsed (gate not satisfied), appends VARNAME to
# suppressed list and returns silently.
bupr_section_item() {
  zbupr_sentinel

  # Collapsed section — track and skip
  if test "${ZBUPR_SECTION_ACTIVE}" = 0; then
    ZBUPR_SECTION_SUPPRESSED+=("$1")
    return 0
  fi

  zbupr_render_field "$@"
}

# zbupr_render_field VARNAME TYPE REQ_STATUS DESCRIPTION
#
# Shared rendering logic for bupr_item and bupr_section_item.
# Reads ZBUPR_LAYOUT to choose single-line or double-line format.
zbupr_render_field() {
  local z_varname=$1
  local z_type=$2
  local z_req=$3
  local z_desc="$4"
  local z_value=${!z_varname:-}

  # Name color: green when set, yellow when not set
  local z_nc
  if test -n "${z_value}"; then
    z_nc=${ZBUC_GREEN}
  else
    z_nc=${ZBUC_YELLOW}
    z_value="(not set)"
  fi

  if test "${ZBUPR_LAYOUT}" = single; then
    # Wide terminal: name value req type description — one line
    printf "  ${z_nc}%-30s${ZBUC_RESET}  %-24s  ${ZBUC_BLUE}%-4s %-11s${ZBUC_RESET}  ${ZBUC_CYAN}%s${ZBUC_RESET}\n" \
      "${z_varname}" "${z_value}" "${z_req}" "${z_type}" "${z_desc}"
  else
    # Narrow terminal: 3-line short display
    # Line 1: name + req + type
    printf "  ${z_nc}%-30s${ZBUC_RESET}  ${ZBUC_BLUE}%-4s %-11s${ZBUC_RESET}\n" \
      "${z_varname}" "${z_req}" "${z_type}"
    # Line 2: current value
    printf "      ${ZBUC_WHITE}%s${ZBUC_RESET}\n" "${z_value}"
    # Line 3: meaning
    printf "      ${ZBUC_GRAY}meaning => %s${ZBUC_RESET}\n" "${z_desc}"
  fi
}

# eof
