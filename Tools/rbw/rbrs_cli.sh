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
# RBRS CLI - Command line interface for RBRS station operations

set -euo pipefail

ZRBRS_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRS_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRS_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRS_CLI_SCRIPT_DIR}/rbrs_regime.sh"
source "${ZRBRS_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRS_CLI_SCRIPT_DIR}/rbcr_render.sh"

######################################################################
# CLI Functions

# Command: validate - source file and validate (dies on first error)
rbrs_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrs_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrs_validate: file not found: ${z_file}"

  buc_step "Validating RBRS station config file: ${z_file}"

  source "${z_file}" || buc_die "rbrs_validate: failed to source ${z_file}"
  zrbrs_kindle

  buc_step "RBRS station config valid"
}

# Command: render - diagnostic display then validate
rbrs_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrs_render: file argument required"
  test -f "${z_file}" || buc_die "rbrs_render: file not found: ${z_file}"

  source "${z_file}" || buc_die "rbrs_render: failed to source ${z_file}"
  zrbrs_kindle
  zrbcr_kindle

  echo ""
  echo "${ZBUC_WHITE}RBRS - Recipe Bottle Station Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  rbcr_section_begin "Podman Configuration"
  rbcr_section_item RBRS_PODMAN_ROOT_DIR    string  req  "Podman machine root directory"
  rbcr_section_item RBRS_VMIMAGE_CACHE_DIR  string  req  "VM image cache directory"
  rbcr_section_item RBRS_VM_PLATFORM        string  req  "VM platform architecture"
  rbcr_section_end

  echo "${ZBUC_GREEN}RBRS station config valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbcc_kindle

z_command="${1:-}"
ZRBRS_CLI_DEFAULT_FILE="../station-files/rbrs.env"

case "${z_command}" in
  validate)
    z_file="${2:-${ZRBRS_CLI_DEFAULT_FILE}}"
    test -f "${z_file}" || buc_die "RBRS file not found: ${z_file}"
    rbrs_validate "${z_file}"
    ;;
  render)
    z_file="${2:-${ZRBRS_CLI_DEFAULT_FILE}}"
    test -f "${z_file}" || buc_die "RBRS file not found: ${z_file}"
    rbrs_render "${z_file}"
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrs_cli.sh {validate|render} [file]"
    ;;
esac

# eof
