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
# Regime CLI Render - Shared rendering utilities for regime CLI modules
#
# Provides section-based rendering with gate-aware suppression and
# terminal-adaptive layouts.  Each regime CLI (rbrn_cli, rbrv_cli)
# sources this module and calls the public functions to compose its
# own render output.  Formatting mechanics are shared; editorial
# decisions (section ordering, grouping) stay manual per regime CLI.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBCR_SOURCED:-}" || buc_die "Module rbcr multiply sourced - check sourcing hierarchy"
ZRBCR_SOURCED=1

######################################################################
# Internal Functions (zrbcr_*)

zrbcr_kindle() {
  test -z "${ZRBCR_KINDLED:-}" || buc_die "Module rbcr already kindled"

  # Terminal layout from BURD dispatch (set in bul_launcher before pipe)
  ZRBCR_TERM_COLS=${BURD_TERM_COLS:-80}
  if test "${ZRBCR_TERM_COLS}" -ge 120; then
    ZRBCR_LAYOUT=single
  else
    ZRBCR_LAYOUT=double
  fi


  # Section state
  ZRBCR_SECTION_ACTIVE=1
  ZRBCR_SECTION_GATE_DESC=""
  ZRBCR_SECTION_SUPPRESSED=()

  ZRBCR_KINDLED=1
}

zrbcr_sentinel() {
  test "${ZRBCR_KINDLED:-}" = "1" || buc_die "Module rbcr not kindled - call zrbcr_kindle first"
}

######################################################################
# Public Functions (rbcr_*)

# rbcr_section_begin TITLE [GATE_VAR GATE_VALUE]
#
# Begin a render section.  Prints section header.
# Optional gate: evaluates ${!GATE_VAR} against GATE_VALUE.
# If gate not satisfied, prints collapsed reminder and suppresses
# subsequent rbcr_line calls until rbcr_section_end.
rbcr_section_begin() {
  zrbcr_sentinel
  local z_title="$1"
  local z_gate_var=${2:-}
  local z_gate_value=${3:-}

  ZRBCR_SECTION_SUPPRESSED=()
  ZRBCR_SECTION_GATE_DESC=""

  if test -n "${z_gate_var}"; then
    local z_actual=${!z_gate_var:-}
    if test "${z_actual}" = "${z_gate_value}"; then
      ZRBCR_SECTION_ACTIVE=1
    else
      ZRBCR_SECTION_ACTIVE=0
      ZRBCR_SECTION_GATE_DESC="${z_gate_var}=${z_actual}"
    fi
  else
    ZRBCR_SECTION_ACTIVE=1
  fi

  if test "${ZRBCR_SECTION_ACTIVE}" = 1; then
    printf "${ZBUC_YELLOW}%s${ZBUC_RESET}\n" "${z_title}"
  else
    printf "${ZBUC_GRAY}%s [%s]${ZBUC_RESET}\n" "${z_title}" "${ZRBCR_SECTION_GATE_DESC}"
  fi
}

# rbcr_section_end
#
# End a render section.  Prints blank line for spacing.
# Resets section state for next section.
rbcr_section_end() {
  zrbcr_sentinel
  echo ""
  ZRBCR_SECTION_ACTIVE=1
  ZRBCR_SECTION_GATE_DESC=""
  ZRBCR_SECTION_SUPPRESSED=()
}

# rbcr_line VARNAME TYPE REQ_STATUS DESCRIPTION
#
# Render one regime field.
#   VARNAME:     unquoted regime variable name (e.g., RBRN_ENTRY_MODE)
#   TYPE:        unquoted type badge (xname, string, fqin, port, ipv4, etc.)
#   REQ_STATUS:  unquoted — req, opt, or cond
#   DESCRIPTION: quoted human prose
#
# If section is collapsed (gate not satisfied), appends VARNAME to
# suppressed list and returns silently.
# Reads ZRBCR_LAYOUT to choose single-line or double-line format.
rbcr_line() {
  zrbcr_sentinel
  local z_varname=$1
  local z_type=$2
  local z_req=$3
  local z_desc="$4"
  local z_value=${!z_varname:-}

  # Collapsed section — track and skip
  if test "${ZRBCR_SECTION_ACTIVE}" = 0; then
    ZRBCR_SECTION_SUPPRESSED+=("${z_varname}")
    return 0
  fi

  # Name color: green when set, yellow when not set
  local z_nc
  if test -n "${z_value}"; then
    z_nc=${ZBUC_GREEN}
  else
    z_nc=${ZBUC_YELLOW}
    z_value="(not set)"
  fi

  if test "${ZRBCR_LAYOUT}" = single; then
    # Wide terminal: name value [type] [req] description — one line
    printf "  ${z_nc}%-30s${ZBUC_RESET}  %-24s  ${ZBUC_GRAY}[%-11s] [%-4s]${ZBUC_RESET}  ${ZBUC_CYAN}%s${ZBUC_RESET}\n" \
      "${z_varname}" "${z_value}" "${z_type}" "${z_req}" "${z_desc}"
  else
    # Narrow terminal: name+value line 1, badges+description line 2
    printf "  ${z_nc}%-30s${ZBUC_RESET}  %s\n" \
      "${z_varname}" "${z_value}"
    printf "  %-30s  ${ZBUC_GRAY}[%-11s] [%-4s]${ZBUC_RESET}  ${ZBUC_CYAN}%s${ZBUC_RESET}\n" \
      "" "${z_type}" "${z_req}" "${z_desc}"
  fi
}

# eof
