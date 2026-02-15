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
# RBRG CLI - Command line interface for RBRG GitHub operations

set -euo pipefail

ZRBRG_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRG_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRG_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRG_CLI_SCRIPT_DIR}/rbrg_regime.sh"
source "${ZRBRG_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRG_CLI_SCRIPT_DIR}/rbcr_render.sh"

######################################################################
# CLI Functions

zrbrg_cli_kindle() {
  test -z "${ZRBRG_CLI_KINDLED:-}" || buc_die "RBRG CLI already kindled"
  ZRBRG_CLI_KINDLED=1
}

# Command: validate - source file and validate (dies on first error)
rbrg_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrg_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrg_validate: file not found: ${z_file}"

  buc_step "Validating RBRG GitHub config file: ${z_file}"

  source "${z_file}" || buc_die "rbrg_validate: failed to source ${z_file}"
  zrbrg_kindle

  buc_step "RBRG GitHub config valid"
}

# Command: render - diagnostic display then validate
rbrg_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrg_render: file argument required"
  test -f "${z_file}" || buc_die "rbrg_render: file not found: ${z_file}"

  source "${z_file}" || buc_die "rbrg_render: failed to source ${z_file}"
  zrbrg_kindle
  zrbcr_kindle

  echo ""
  echo "${ZBUC_WHITE}RBRG - Recipe Bottle GitHub Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  rbcr_section_begin "GitHub Identity"
  rbcr_section_item RBRG_USERNAME xname req "GitHub username"

  # SECURITY: Mask RBRG_PAT - never display the actual value
  local z_pat_status="[NOT SET]"
  local z_pat_color="${ZBUC_RED}"
  if test -n "${RBRG_PAT:-}"; then
    z_pat_status="[REDACTED - ${#RBRG_PAT} chars]"
    z_pat_color="${ZBUC_GREEN}"
  fi
  printf "  ${z_pat_color}%-30s${ZBUC_RESET} %s\n" "RBRG_PAT" "${z_pat_status}"

  rbcr_section_end

  echo "${ZBUC_GREEN}RBRG GitHub config valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbrg_cli_kindle
zrbcc_kindle

z_command="${1:-}"
case "${z_command}" in
  validate)
    z_file="${2:-}"
    test -n "${z_file}" || buc_die "rbrg_cli.sh validate: file argument required"
    rbrg_validate "${z_file}"
    ;;
  render)
    z_file="${2:-}"
    test -n "${z_file}" || buc_die "rbrg_cli.sh render: file argument required"
    rbrg_render "${z_file}"
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrg_cli.sh {validate|render} [file]"
    ;;
esac

# eof
