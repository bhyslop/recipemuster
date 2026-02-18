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
# BURS CLI - Command line interface for BURS regime operations
#
# Requires BURD_STATION_FILE environment variable (path to burs.env).
# This CLI sources the file and validates/renders/displays BURS configuration.

set -euo pipefail

ZBURS_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZBURS_CLI_SCRIPT_DIR}/buc_command.sh"
source "${ZBURS_CLI_SCRIPT_DIR}/buv_validation.sh"
source "${ZBURS_CLI_SCRIPT_DIR}/burs_regime.sh"
source "${ZBURS_CLI_SCRIPT_DIR}/burd_regime.sh"
source "${ZBURS_CLI_SCRIPT_DIR}/bupr_PresentationRegime.sh"

######################################################################
# CLI Functions

zburs_cli_kindle() {
  test -z "${ZBURS_CLI_KINDLED:-}" || buc_die "BURS CLI already kindled"

  # Verify dispatch environment
  zburd_kindle

  ZBURS_CLI_KINDLED=1
}

# Command: validate - source file and validate
burs_validate() {
  buc_step "Validating BURS: ${BURD_STATION_FILE}"

  source "${BURD_STATION_FILE}" || buc_die "Failed to source BURS"
  zburs_kindle
  zburs_validate_fields

  buc_success "BURS configuration valid"
}

# Command: render - diagnostic display then validate
burs_render() {
  local z_file="${BURD_STATION_FILE}"

  source "${z_file}" || buc_die "burs_render: failed to source ${z_file}"
  zburs_kindle
  zbupr_kindle

  echo ""
  echo "${ZBUC_WHITE}BURS - Bash Utility Station Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Developer Logging
  bupr_section_begin "Developer Logging"
  bupr_section_item BURS_LOG_DIR  path  req  "Directory for BUK operation logs"
  bupr_section_end

  # Unexpected variables
  if test ${#ZBURS_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected BURS_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZBURS_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate after full display
  zburs_validate_fields
  echo "${ZBUC_GREEN}BURS configuration valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zburs_cli_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    burs_validate
    ;;
  render)
    burs_render
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: burs_cli.sh {validate|render}"
    ;;
esac

# eof
