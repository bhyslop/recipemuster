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
# BURC CLI - Command line interface for BURC regime operations
#
# Requires BURD_REGIME_FILE environment variable (path to burc.env).
# This CLI sources the file and validates/renders/displays BURC configuration.

set -euo pipefail

ZBURC_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZBURC_CLI_SCRIPT_DIR}/buc_command.sh"
source "${ZBURC_CLI_SCRIPT_DIR}/buv_validation.sh"
source "${ZBURC_CLI_SCRIPT_DIR}/burc_regime.sh"
source "${ZBURC_CLI_SCRIPT_DIR}/burd_regime.sh"
source "${ZBURC_CLI_SCRIPT_DIR}/bupr_PresentationRegime.sh"

######################################################################
# CLI Functions

zburc_cli_kindle() {
  test -z "${ZBURC_CLI_KINDLED:-}" || buc_die "BURC CLI already kindled"

  # Verify dispatch environment
  zburd_kindle

  ZBURC_CLI_KINDLED=1
}

# Command: validate - source file and validate
burc_validate() {
  buc_step "Validating BURC: ${BURD_REGIME_FILE}"

  source "${BURD_REGIME_FILE}" || buc_die "Failed to source BURC"
  zburc_kindle
  zburc_validate_fields

  buc_success "BURC configuration valid"
}

# Command: render - diagnostic display then validate
burc_render() {
  local z_file="${BURD_REGIME_FILE}"

  source "${z_file}" || buc_die "burc_render: failed to source ${z_file}"
  zburc_kindle
  zbupr_kindle

  echo ""
  echo "${ZBUC_WHITE}BURC - Bash Utility Configuration Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Station Reference
  bupr_section_begin "Station Reference"
  bupr_section_item BURC_STATION_FILE       path    req  "Path to developer's BURS station file"
  bupr_section_end

  # Tabtarget Infrastructure
  bupr_section_begin "Tabtarget Infrastructure"
  bupr_section_item BURC_TABTARGET_DIR       path    req  "Directory containing launcher scripts"
  bupr_section_item BURC_TABTARGET_DELIMITER string  req  "Token separator in tabtarget filenames"
  bupr_section_end

  # Project Structure
  bupr_section_begin "Project Structure"
  bupr_section_item BURC_TOOLS_DIR           path    req  "Directory containing tool scripts"
  bupr_section_item BURC_PROJECT_ROOT        path    req  "Path from burc.env to project root"
  bupr_section_item BURC_MANAGED_KITS        string  req  "Comma-separated kit list for vvx"
  bupr_section_end

  # Build Output
  bupr_section_begin "Build Output"
  bupr_section_item BURC_TEMP_ROOT_DIR       path    req  "Root directory for temporary files"
  bupr_section_item BURC_OUTPUT_ROOT_DIR     path    req  "Root directory for command output"
  bupr_section_end

  # Logging
  bupr_section_begin "Logging"
  bupr_section_item BURC_LOG_LAST            xname   req  "Filename stem for last-run log"
  bupr_section_item BURC_LOG_EXT             xname   req  "Log file extension (without dot)"
  bupr_section_end

  # Unexpected variables
  if test ${#ZBURC_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected BURC_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZBURC_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate after full display
  zburc_validate_fields
  echo "${ZBUC_GREEN}BURC configuration valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zburc_cli_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    burc_validate
    ;;
  render)
    burc_render
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: burc_cli.sh {validate|render}"
    ;;
esac

# eof
