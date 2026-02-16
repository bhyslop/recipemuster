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
# RBRO CLI - Command line interface for RBRO OAuth operations

set -euo pipefail

ZRBRO_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRO_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRO_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRO_CLI_SCRIPT_DIR}/rbro_regime.sh"
source "${ZRBRO_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRO_CLI_SCRIPT_DIR}/rbcr_render.sh"

######################################################################
# CLI Functions

zrbro_cli_kindle() {
  test -z "${ZRBRO_CLI_KINDLED:-}" || buc_die "RBRO CLI already kindled"
  ZRBRO_CLI_KINDLED=1
}

# Command: validate - source file and validate (dies on first error)
rbro_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbro_validate: file argument required"
  test -f "${z_file}" || buc_die "rbro_validate: file not found: ${z_file}"

  buc_step "Validating RBRO OAuth credential file: ${z_file}"

  source "${z_file}" || buc_die "rbro_validate: failed to source ${z_file}"
  zrbro_kindle
  zrbro_validate_fields

  buc_step "RBRO OAuth credential file valid"
}

# Command: render - diagnostic display then validate
rbro_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbro_render: file argument required"
  test -f "${z_file}" || buc_die "rbro_render: file not found: ${z_file}"

  source "${z_file}" || buc_die "rbro_render: failed to source ${z_file}"
  zrbro_kindle
  zrbcr_kindle

  echo ""
  echo "${ZBUC_WHITE}RBRO - Recipe Bottle OAuth Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  rbcr_section_begin "OAuth Credentials"

  # SECURITY: Mask client secret - never display the actual value
  local z_secret_status="[NOT SET]"
  local z_secret_color="${ZBUC_RED}"
  if test -n "${RBRO_CLIENT_SECRET:-}"; then
    z_secret_status="[REDACTED - ${#RBRO_CLIENT_SECRET} chars]"
    z_secret_color="${ZBUC_GREEN}"
  fi
  printf "  ${z_secret_color}%-30s${ZBUC_RESET} %s\n" "RBRO_CLIENT_SECRET" "${z_secret_status}"

  # SECURITY: Mask refresh token - never display the actual value
  local z_token_status="[NOT SET]"
  local z_token_color="${ZBUC_RED}"
  if test -n "${RBRO_REFRESH_TOKEN:-}"; then
    z_token_status="[REDACTED - ${#RBRO_REFRESH_TOKEN} chars]"
    z_token_color="${ZBUC_GREEN}"
  fi
  printf "  ${z_token_color}%-30s${ZBUC_RESET} %s\n" "RBRO_REFRESH_TOKEN" "${z_token_status}"

  rbcr_section_end

  # Display unexpected RBRO_ variables if present
  if test ${#ZRBRO_UNEXPECTED[@]} -gt 0; then
    echo ""
    echo "${ZBUC_RED}Unexpected RBRO_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZRBRO_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  zrbro_validate_fields
  echo "${ZBUC_GREEN}RBRO OAuth credential file valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbro_cli_kindle
zrbcc_kindle

z_command="${1:-}"
ZRBRO_CLI_DEFAULT_FILE="${HOME}/.rbw/rbro.env"

case "${z_command}" in
  validate)
    z_file="${2:-${ZRBRO_CLI_DEFAULT_FILE}}"
    test -f "${z_file}" || buc_die "RBRO file not found: ${z_file}"
    rbro_validate "${z_file}"
    ;;
  render)
    z_file="${2:-${ZRBRO_CLI_DEFAULT_FILE}}"
    test -f "${z_file}" || buc_die "RBRO file not found: ${z_file}"
    rbro_render "${z_file}"
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbro_cli.sh {validate|render} [file]"
    ;;
esac

# eof
