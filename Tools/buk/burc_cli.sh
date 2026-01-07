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
# BOOTSTRAP REGIME CLI
# ====================
# This is a bootstrap regime CLI - it reads configuration from environment
# variables set by the launcher, NOT from file arguments. BURC and BURS are
# foundational regimes that the launcher loads before any workbench runs.
# Unlike application regime CLIs which may take file arguments, bootstrap
# regime CLIs always rely on launcher-provided environment.
#
# Required environment: BUD_REGIME_FILE (path to .buk/burc.env)
# The BURC_* variables must already be set by launcher sourcing this file.

set -euo pipefail

ZBURC_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZBURC_CLI_SCRIPT_DIR}/buc_command.sh"
source "${ZBURC_CLI_SCRIPT_DIR}/burc_regime.sh"

######################################################################
# CLI Functions

zburc_cli_kindle() {
  test -z "${ZBURC_CLI_KINDLED:-}" || buc_die "BURC CLI already kindled"

  # Verify bootstrap environment
  test -n "${BUD_REGIME_FILE:-}" || buc_die "BUD_REGIME_FILE not set - must be called via launcher"

  ZBURC_SPEC_FILE="${ZBURC_CLI_SCRIPT_DIR}/burc_specification.md"
  ZBURC_SPEC_FILE_ABSOLUTE="$(cd "${ZBURC_CLI_SCRIPT_DIR}" && pwd)/burc_specification.md"

  ZBURC_CLI_KINDLED=1
}

# Command: bootstrap_validate - validate BURC from launcher environment
burc_bootstrap_validate() {
  buc_step "Validating BURC: ${BUD_REGIME_FILE}"

  # BURC_* variables already set by launcher - just validate via kindle
  zburc_kindle

  buc_success "BURC configuration valid"
}

# Command: bootstrap_render - display configuration values from environment
burc_bootstrap_render() {
  buc_step "BURC Configuration: ${BUD_REGIME_FILE}"

  # Render with aligned columns - values already in environment
  printf "%-25s %s\n" "BURC_STATION_FILE" "${BURC_STATION_FILE:-<not set>}"
  printf "%-25s %s\n" "BURC_TABTARGET_DIR" "${BURC_TABTARGET_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_TABTARGET_DELIMITER" "${BURC_TABTARGET_DELIMITER:-<not set>}"
  printf "%-25s %s\n" "BURC_TOOLS_DIR" "${BURC_TOOLS_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_TEMP_ROOT_DIR" "${BURC_TEMP_ROOT_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_OUTPUT_ROOT_DIR" "${BURC_OUTPUT_ROOT_DIR:-<not set>}"
  printf "%-25s %s\n" "BURC_LOG_LAST" "${BURC_LOG_LAST:-<not set>}"
  printf "%-25s %s\n" "BURC_LOG_EXT" "${BURC_LOG_EXT:-<not set>}"
}

# Command: bootstrap_info - display specification (formatted for terminal)
burc_bootstrap_info() {
  cat <<EOF

${ZBUC_CYAN}========================================${ZBUC_RESET}
${ZBUC_WHITE}BURC - Bash Utility Regime Configuration${ZBUC_RESET}
${ZBUC_CYAN}========================================${ZBUC_RESET}

${ZBUC_YELLOW}Overview${ZBUC_RESET}
Project-level configuration that defines repository structure for BUK.
Checked into git and shared by all developers on the team.

${ZBUC_YELLOW}Variables${ZBUC_RESET}

  ${ZBUC_GREEN}BURC_STATION_FILE${ZBUC_RESET}
    Path to developer's BURS file (relative to project root)
    Type: string
    Example: ../station-files/burs.env

  ${ZBUC_GREEN}BURC_TABTARGET_DIR${ZBUC_RESET}
    Directory containing tabtarget scripts
    Type: string
    Example: tt

  ${ZBUC_GREEN}BURC_TABTARGET_DELIMITER${ZBUC_RESET}
    Token separator in tabtarget filenames
    Type: string
    Example: .

  ${ZBUC_GREEN}BURC_TOOLS_DIR${ZBUC_RESET}
    Directory containing tool scripts
    Type: string
    Example: Tools

  ${ZBUC_GREEN}BURC_TEMP_ROOT_DIR${ZBUC_RESET}
    Parent directory for temp directories
    Type: string
    Example: ../temp-buk

  ${ZBUC_GREEN}BURC_OUTPUT_ROOT_DIR${ZBUC_RESET}
    Parent directory for output directories
    Type: string
    Example: ../output-buk

  ${ZBUC_GREEN}BURC_LOG_LAST${ZBUC_RESET}
    Basename for "last run" log file
    Type: xname
    Example: last

  ${ZBUC_GREEN}BURC_LOG_EXT${ZBUC_RESET}
    Extension for log files (without dot)
    Type: xname
    Example: txt

EOF

  printf "${ZBUC_CYAN}For full specification, see: \033]8;;file://${ZBURC_SPEC_FILE_ABSOLUTE}\033\\${ZBURC_SPEC_FILE}\033]8;;\033\\${ZBUC_RESET}\n"
}

######################################################################
# Main dispatch

zburc_cli_kindle

z_command="${1:-}"

case "${z_command}" in
  bootstrap_validate)
    burc_bootstrap_validate
    ;;
  bootstrap_render)
    burc_bootstrap_render
    ;;
  bootstrap_info)
    burc_bootstrap_info
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: burc_cli.sh {bootstrap_validate|bootstrap_render|bootstrap_info}"
    ;;
esac

# eof
