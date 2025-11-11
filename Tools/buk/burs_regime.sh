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
# BURS Regime - Multi-call script for BURS (Station) regime operations
#

set -euo pipefail

ZBURS_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZBURS_SCRIPT_DIR}/buc_command.sh"
source "${ZBURS_SCRIPT_DIR}/buv_validation.sh"

# Module state
ZBURS_KINDLED=""

zburs_kindle() {
  test -z "${ZBURS_KINDLED:-}" || buc_die "zburs_kindle: already kindled"

  ZBURS_SPEC_FILE="${ZBURS_SCRIPT_DIR}/burs_specification.md"

  ZBURS_KINDLED="1"
}

zburs_sentinel() {
  test "${ZBURS_KINDLED:-}" = "1" || buc_die "zburs_sentinel: not kindled"
}

# Predicate: validate loaded BURS variables (returns 0=valid, 1=invalid, no output)
zburs_validate_predicate() {
  zburs_sentinel

  # Check BURS_LOG_DIR exists and is non-empty
  test -n "${BURS_LOG_DIR:-}" || return 1

  return 0
}

# Command: validate - source file and validate
burs_validate() {
  zburs_sentinel

  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "burs_validate: file argument required"
  test -f "${z_file}" || buc_die "burs_validate: file not found: ${z_file}"

  buc_step "Validating BURS assignment file: ${z_file}"

  # Source the assignment file
  # shellcheck disable=SC1090
  source "${z_file}" || buc_die "burs_validate: failed to source ${z_file}"

  # Run validation predicate
  if ! zburs_validate_predicate; then
    buc_log_burs "Validation failed"
    burs_info
    buc_die "BURS validation failed for ${z_file}"
  fi

  buc_step "BURS configuration valid"
}

# Command: render - display configuration values
burs_render() {
  zburs_sentinel

  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "burs_render: file argument required"
  test -f "${z_file}" || buc_die "burs_render: file not found: ${z_file}"

  buc_step "BURS Configuration: ${z_file}"

  # Source the assignment file
  # shellcheck disable=SC1090
  source "${z_file}" || buc_die "burs_render: failed to source ${z_file}"

  # Render with aligned columns
  printf "%-25s %s\n" "BURS_LOG_DIR" "${BURS_LOG_DIR:-<not set>}"
}

# Command: info - display specification (formatted for terminal)
burs_info() {
  zburs_sentinel

  # Source BCU for colors
  # shellcheck disable=SC1091
  source "${ZBURS_SCRIPT_DIR}/buc_command.sh"

  cat <<EOF

${ZBUC_CYAN}========================================${ZBUC_RESET}
${ZBUC_WHITE}BURS - Bash Utility Regime Station${ZBUC_RESET}
${ZBUC_CYAN}========================================${ZBUC_RESET}

${ZBUC_YELLOW}Overview${ZBUC_RESET}
Developer/machine-level configuration for personal preferences.
NOT checked into git - each developer has their own BURS file.

${ZBUC_YELLOW}Variables${ZBUC_RESET}

  ${ZBUC_GREEN}BURS_LOG_DIR${ZBUC_RESET}
    Where this developer stores logs
    Type: string
    Example: ../_logs_buk

${ZBUC_CYAN}For full specification, see: ${ZBURS_SPEC_FILE}${ZBUC_RESET}

EOF
}

# Main dispatch
zburs_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    shift
    burs_validate "${@}"
    ;;
  render)
    shift
    burs_render "${@}"
    ;;
  info)
    burs_info
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: burs_regime.sh {validate|render|info} [args]"
    ;;
esac
