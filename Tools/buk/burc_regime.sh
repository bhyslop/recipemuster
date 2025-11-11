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
# BURC Regime - Multi-call script for BURC (Config) regime operations
#

set -euo pipefail

ZBURC_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZBURC_SCRIPT_DIR}/buc_BashCommandUtility.sh"
source "${ZBURC_SCRIPT_DIR}/buv_BashValidationUtility.sh"

# Module state
ZBURC_KINDLED=""

zburc_kindle() {
  test -z "${ZBURC_KINDLED:-}" || buc_die "zburc_kindle: already kindled"

  ZBURC_SPEC_FILE="${ZBURC_SCRIPT_DIR}/burc_specification.md"
  ZBURC_SPEC_FILE_ABSOLUTE="$(cd "${ZBURC_SCRIPT_DIR}" && pwd)/burc_specification.md"

  ZBURC_KINDLED="1"
}

zburc_sentinel() {
  test "${ZBURC_KINDLED:-}" = "1" || buc_die "zburc_sentinel: not kindled"
}

# Predicate: validate loaded BURC variables (returns 0=valid, 1=invalid, no output)
zburc_validate_predicate() {
  zburc_sentinel

  # Check all required BURC variables exist and are non-empty
  test -n "${BURC_STATION_FILE:-}" || return 1
  test -n "${BURC_TABTARGET_DIR:-}" || return 1
  test -n "${BURC_TABTARGET_DELIMITER:-}" || return 1
  test -n "${BURC_TOOLS_DIR:-}" || return 1
  test -n "${BURC_TEMP_ROOT_DIR:-}" || return 1
  test -n "${BURC_OUTPUT_ROOT_DIR:-}" || return 1
  test -n "${BURC_LOG_LAST:-}" || return 1
  test -n "${BURC_LOG_EXT:-}" || return 1

  # Check delimiter is exactly one character
  test "${#BURC_TABTARGET_DELIMITER}" -eq 1 || return 1

  return 0
}

# Command: validate - source file and validate
burc_validate() {
  zburc_sentinel

  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "burc_validate: file argument required"
  test -f "${z_file}" || buc_die "burc_validate: file not found: ${z_file}"

  buc_step "Validating BURC assignment file: ${z_file}"

  # Source the assignment file
  # shellcheck disable=SC1090
  source "${z_file}" || buc_die "burc_validate: failed to source ${z_file}"

  # Run validation predicate
  if ! zburc_validate_predicate; then
    buc_log_burc "Validation failed"
    burc_info
    buc_die "BURC validation failed for ${z_file}"
  fi

  buc_step "BURC configuration valid"
}

# Command: render - display configuration values
burc_render() {
  zburc_sentinel

  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "burc_render: file argument required"
  test -f "${z_file}" || buc_die "burc_render: file not found: ${z_file}"

  buc_step "BURC Configuration: ${z_file}"

  # Source the assignment file
  # shellcheck disable=SC1090
  source "${z_file}" || buc_die "burc_render: failed to source ${z_file}"

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

# Command: info - display specification (formatted for terminal)
burc_info() {
  zburc_sentinel

  # Source BCU for colors
  # shellcheck disable=SC1091
  source "${ZBURC_SCRIPT_DIR}/buc_BashCommandUtility.sh"

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

# Main dispatch
zburc_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    shift
    burc_validate "${@}"
    ;;
  render)
    shift
    burc_render "${@}"
    ;;
  info)
    burc_info
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: burc_regime.sh {validate|render|info} [args]"
    ;;
esac
