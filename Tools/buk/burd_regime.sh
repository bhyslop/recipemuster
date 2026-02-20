#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# BURD Regime - Bash Utility Regime Dispatch Module
#
# BURD is an ephemeral regime — variables are constructed by bud_dispatch.sh during
# tabtarget execution, not sourced from a file.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZBURD_SOURCED:-}" || buc_die "Module burd multiply sourced - check sourcing hierarchy"
ZBURD_SOURCED=1

######################################################################
# Internal Functions (zburd_*)

zburd_kindle() {
  test -z "${ZBURD_KINDLED:-}" || buc_die "Module burd already kindled"

  # Set defaults for all optional fields only (required ones are set by dispatch — don't overwrite)
  BURD_NO_LOG="${BURD_NO_LOG:-}"
  BURD_INTERACTIVE="${BURD_INTERACTIVE:-}"
  BURD_TOKEN_3="${BURD_TOKEN_3:-}"
  BURD_TOKEN_4="${BURD_TOKEN_4:-}"
  BURD_TOKEN_5="${BURD_TOKEN_5:-}"

  # Detect unexpected BURD_ variables
  local z_known="BURD_REGIME_FILE BURD_STATION_FILE BURD_COORDINATOR_SCRIPT BURD_LAUNCHER BURD_TERM_COLS BURD_NOW_STAMP BURD_TEMP_DIR BURD_OUTPUT_DIR BURD_TRANSCRIPT BURD_GIT_CONTEXT BURD_TARGET BURD_COMMAND BURD_TOKEN_1 BURD_TOKEN_2 BURD_CLI_ARGS BURD_NO_LOG BURD_INTERACTIVE BURD_TOKEN_3 BURD_TOKEN_4 BURD_TOKEN_5 BURD_LOG_LAST BURD_LOG_SAME BURD_LOG_HIST"
  ZBURD_UNEXPECTED=()
  local z_var
  for z_var in $(compgen -v BURD_); do
    case " ${z_known} " in
      *" ${z_var} "*) : ;;
      *) ZBURD_UNEXPECTED+=("${z_var}") ;;
    esac
  done

  ZBURD_KINDLED=1
}

zburd_sentinel() {
  test "${ZBURD_KINDLED:-}" = "1" || buc_die "Module burd not kindled - call zburd_kindle first"
}

# Validate BURD variables via buv_* (dies on first error)
# Prerequisite: kindle must have been called; buv_validation.sh must be sourced
zburd_validate_fields() {
  zburd_sentinel

  # Die on unexpected variables
  if test ${#ZBURD_UNEXPECTED[@]} -gt 0; then
    buc_die "Unexpected BURD_ variables: ${ZBURD_UNEXPECTED[*]}"
  fi

  # Launcher Configuration
  buv_env_string BURD_REGIME_FILE           1 256
  buv_env_string BURD_STATION_FILE          1 256
  buv_env_string BURD_COORDINATOR_SCRIPT    1 256
  buv_env_string BURD_LAUNCHER              1 256
  buv_env_string BURD_TERM_COLS             1   8

  # Computed State
  buv_env_string BURD_NOW_STAMP             1  64
  buv_env_string BURD_TEMP_DIR              1 256
  buv_env_string BURD_OUTPUT_DIR            1 256
  buv_env_string BURD_TRANSCRIPT            1 256
  buv_env_string BURD_GIT_CONTEXT           1 128
  # Parsed Tabtarget
  buv_env_string BURD_TARGET                1 256
  buv_env_string BURD_COMMAND               1  64
  buv_env_string BURD_TOKEN_1               1  64
  buv_env_string BURD_TOKEN_2               1 128

  # BURD_CLI_ARGS is an array — skip validation (no buv_ for arrays)

  # Conditional: log paths are set by dispatch but NOT exported, so they're only
  # available in the dispatch process itself (not exec'd children).
  # Validate only when both logging is active AND the variables are present.
  if test -z "${BURD_NO_LOG:-}" && test -n "${BURD_LOG_LAST:-}"; then
    buv_env_string BURD_LOG_LAST            1 256
    buv_env_string BURD_LOG_SAME            1 256
    buv_env_string BURD_LOG_HIST            1 256
  fi

  # BURD_NO_LOG, BURD_INTERACTIVE, BURD_TOKEN_3-5 are optional — not validated as required
}

# Render BURD regime state for diagnostic display
# Prerequisite: zburd_kindle must have been called; bupr (PresentationRegime) must be sourced
zburd_render() {
  zburd_sentinel
  type bupr_section_begin >/dev/null 2>&1 || buc_die "zburd_render requires bupr (PresentationRegime) to be sourced"

  echo ""
  echo "${ZBUC_WHITE}BURD - Dispatch Runtime Regime${ZBUC_RESET}"
  echo ""

  # Launcher Configuration
  bupr_section_begin "Launcher Configuration"
  bupr_section_item BURD_REGIME_FILE           path   req  "Path to the BURC regime configuration file"
  bupr_section_item BURD_STATION_FILE          path   req  "Path to the developer's BURS station file"
  bupr_section_item BURD_COORDINATOR_SCRIPT    path   req  "Path to the coordinator script for this tabtarget"
  bupr_section_item BURD_LAUNCHER              path   req  "Path to the tabtarget launcher script"
  bupr_section_item BURD_TERM_COLS             string req  "Terminal column width at dispatch time"
  bupr_section_end

  # Computed State
  bupr_section_begin "Computed State"
  bupr_section_item BURD_NOW_STAMP             string req  "Timestamp string computed at dispatch time"
  bupr_section_item BURD_TEMP_DIR              path   req  "Temporary directory for this invocation"
  bupr_section_item BURD_OUTPUT_DIR            path   req  "Output directory for this invocation"
  bupr_section_item BURD_TRANSCRIPT            path   req  "Path to transcript file for this invocation"
  bupr_section_item BURD_GIT_CONTEXT           string req  "Git context string at dispatch time"
  bupr_section_end

  # Parsed Tabtarget
  bupr_section_begin "Parsed Tabtarget"
  bupr_section_item BURD_COMMAND               string req  "Command parsed from tabtarget filename"
  bupr_section_item BURD_TARGET                string req  "Target parsed from tabtarget filename"
  bupr_section_item BURD_TOKEN_1               string req  "First token parsed from tabtarget filename"
  bupr_section_item BURD_TOKEN_2               string req  "Second token parsed from tabtarget filename"
  bupr_section_item BURD_TOKEN_3               string opt  "Third token parsed from tabtarget filename (optional)"
  bupr_section_item BURD_TOKEN_4               string opt  "Fourth token parsed from tabtarget filename (optional)"
  bupr_section_item BURD_TOKEN_5               string opt  "Fifth token parsed from tabtarget filename (optional)"
  bupr_section_end

  # Caller Options
  bupr_section_begin "Caller Options"
  bupr_section_item BURD_NO_LOG                string opt  "Disable logging when set (empty means logging active)"
  bupr_section_item BURD_INTERACTIVE           string opt  "Interactive mode flag when set"
  bupr_section_end

  # Log Paths (conditional: only when logging is active)
  if test -z "${BURD_NO_LOG:-}"; then
    bupr_section_begin "Log Paths"
    bupr_section_item BURD_LOG_LAST            path   req  "Path to last-run log file"
    bupr_section_item BURD_LOG_SAME            path   req  "Path to same-command log file"
    bupr_section_item BURD_LOG_HIST            path   req  "Path to historical log file"
    bupr_section_end
  fi

  # Unexpected variables
  if test ${#ZBURD_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected BURD_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZBURD_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi
}

# eof
