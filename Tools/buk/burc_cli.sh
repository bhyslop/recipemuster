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
source "${ZBURC_CLI_SCRIPT_DIR}/burc_regime.sh"

######################################################################
# CLI Functions

zburc_cli_kindle() {
  test -z "${ZBURC_CLI_KINDLED:-}" || buc_die "BURC CLI already kindled"

  # Verify environment
  test -n "${BURD_REGIME_FILE:-}" || buc_die "BURD_REGIME_FILE not set - must be called via launcher"

  ZBURC_CLI_KINDLED=1
}

# Command: validate - source file and validate
burc_validate() {
  buc_step "Validating BURC: ${BURD_REGIME_FILE}"

  source "${BURD_REGIME_FILE}" || buc_die "Failed to source BURC"
  zburc_kindle

  buc_success "BURC configuration valid"
}

# Command: render - display configuration values
burc_render() {
  buc_step "BURC Configuration: ${BURD_REGIME_FILE}"

  source "${BURD_REGIME_FILE}" || buc_die "Failed to source BURC"

  # Render with aligned columns
  printf "%-25s %s\n" "BURC_STATION_FILE" "${BURC_STATION_FILE:-<not set>}"
  printf "%-25s %s\n" "BURC_TABTARGET_DIR" "${BURC_TABTARGET_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_TABTARGET_DELIMITER" "${BURC_TABTARGET_DELIMITER:-<not set>}"
  printf "%-25s %s\n" "BURC_TOOLS_DIR" "${BURC_TOOLS_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_TEMP_ROOT_DIR" "${BURC_TEMP_ROOT_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_OUTPUT_ROOT_DIR" "${BURC_OUTPUT_ROOT_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_LOG_LAST" "${BURC_LOG_LAST:-<not set>}"
  printf "%-25s %s\n" "BURC_LOG_EXT" "${BURC_LOG_EXT:-<not set>}"
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
