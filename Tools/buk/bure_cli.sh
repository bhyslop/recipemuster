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
# BURE CLI - Command line interface for BURE regime operations
#
# BURE is an ambient regime — variables are read from the current environment.
# No file sourcing is required; callers export BURE_* variables before invoking.

set -euo pipefail

ZBURE_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZBURE_CLI_SCRIPT_DIR}/buc_command.sh"
source "${ZBURE_CLI_SCRIPT_DIR}/buv_validation.sh"
source "${ZBURE_CLI_SCRIPT_DIR}/bure_regime.sh"
source "${ZBURE_CLI_SCRIPT_DIR}/bupr_PresentationRegime.sh"

######################################################################
# CLI Functions

# Command: validate - kindle and validate ambient environment
bure_validate() {
  buc_step "Validating BURE ambient environment"

  zbure_kindle
  zbure_validate_fields

  buc_success "BURE configuration valid"
}

# Command: render - diagnostic display then validate
bure_render() {
  zbure_kindle
  zbupr_kindle

  echo ""
  echo "${ZBUC_WHITE}BURE - Bash Utility Environment Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}Source: ambient (caller environment)${ZBUC_RESET}"
  echo ""

  # Behavioral Overrides
  bupr_section_begin "Behavioral Overrides"
  bupr_section_item BURE_COUNTDOWN  enum  opt  "Countdown behavior override (skip)"
  bupr_section_end

  # Unexpected variables
  if test ${#ZBURE_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected BURE_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZBURE_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate after full display
  zbure_validate_fields
  echo "${ZBUC_GREEN}BURE configuration valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

z_command="${1:-}"

case "${z_command}" in
  validate)
    bure_validate
    ;;
  render)
    bure_render
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: bure_cli.sh {validate|render}"
    ;;
esac

# eof
