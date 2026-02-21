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
# RBRP CLI - Command line interface for RBRP payor operations

set -euo pipefail

ZRBRP_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRP_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRP_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRP_CLI_SCRIPT_DIR}/rbrp_regime.sh"
source "${ZRBRP_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRP_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBRP_CLI_SCRIPT_DIR}/rbcr_render.sh"

######################################################################
# CLI Functions

# Command: validate - source file and validate (dies on first error)
rbrp_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrp_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrp_validate: file not found: ${z_file}"

  buc_step "Validating RBRP payor file: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrp_validate: failed to source ${z_file}"

  # Kindle (requires RBGC)
  zrbgc_kindle
  zrbrp_kindle

  buc_step "RBRP payor valid"
}

# Command: render - diagnostic display then validate
rbrp_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrp_render: file argument required"
  test -f "${z_file}" || buc_die "rbrp_render: file not found: ${z_file}"

  # Source and kindle (no dying — show all fields before validation)
  source "${z_file}" || buc_die "rbrp_render: failed to source ${z_file}"
  zrbgc_kindle
  zrbrp_kindle
  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRP - Recipe Bottle Regime Payor${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Payor Project Identity
  rbcr_section_begin "Payor Project Identity"
  rbcr_section_item RBRP_PAYOR_PROJECT_ID  string  req  "GCP project hosting OAuth client"
  rbcr_section_end

  # Billing Configuration
  rbcr_section_begin "Billing Configuration"
  rbcr_section_item RBRP_BILLING_ACCOUNT_ID  string  opt  "Billing account for depot projects"
  rbcr_section_end

  # OAuth Configuration
  rbcr_section_begin "OAuth Configuration"
  rbcr_section_item RBRP_OAUTH_CLIENT_ID  string  opt  "OAuth 2.0 client identifier"
  rbcr_section_end

  # Validate after full render
  echo "${ZBUC_GREEN}RBRP payor valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbcc_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    z_file="${RBCC_rbrp_file}"
    test -f "${z_file}" || buc_die "RBRP payor file not found: ${z_file}"
    rbrp_validate "${z_file}"
    ;;
  render)
    z_file="${RBCC_rbrp_file}"
    test -f "${z_file}" || buc_die "RBRP payor file not found: ${z_file}"
    rbrp_render "${z_file}"
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrp_cli.sh {validate|render}"
    ;;
esac

# eof
