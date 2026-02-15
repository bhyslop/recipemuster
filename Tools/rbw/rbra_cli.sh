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
# RBRA CLI - Command line interface for RBRA credential operations

set -euo pipefail

ZRBRA_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRA_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRA_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRA_CLI_SCRIPT_DIR}/rbra_regime.sh"
source "${ZRBRA_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRA_CLI_SCRIPT_DIR}/rbcr_render.sh"
source "${ZRBRA_CLI_SCRIPT_DIR}/rbrr_regime.sh"

######################################################################
# CLI Functions

zrbra_cli_kindle() {
  test -z "${ZRBRA_CLI_KINDLED:-}" || buc_die "RBRA CLI already kindled"
  ZRBRA_CLI_KINDLED=1
}

# Command: validate - source file and validate (dies on first error)
rbra_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbra_validate: file argument required"
  test -f "${z_file}" || buc_die "rbra_validate: file not found: ${z_file}"

  buc_step "Validating RBRA credential file: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbra_validate: failed to source ${z_file}"

  # Kindle and validate
  zrbra_kindle
  zrbra_validate_fields

  buc_step "RBRA credential valid"
}

# Command: render - diagnostic display then validate
rbra_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbra_render: file argument required"
  test -f "${z_file}" || buc_die "rbra_render: file not found: ${z_file}"

  # Source and kindle (no dying — show all fields before validation)
  source "${z_file}" || buc_die "rbra_render: failed to source ${z_file}"
  zrbra_kindle
  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRA - Recipe Bottle Authentication Regime${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Service Account Identity
  rbcr_section_begin "Service Account Identity"
  rbcr_section_item RBRA_CLIENT_EMAIL       string  req  "Service account email address"
  rbcr_section_item RBRA_PROJECT_ID         string  req  "GCP project owning the service account"
  rbcr_section_end

  # Authentication Material
  rbcr_section_begin "Authentication Material"
  # CRITICAL SECURITY: Mask private key — show presence and length only
  local z_key_status="[NOT SET]"
  local z_key_color="${ZBUC_RED}"
  if test -n "${RBRA_PRIVATE_KEY:-}"; then
    z_key_status="[REDACTED - ${#RBRA_PRIVATE_KEY} chars]"
    z_key_color="${ZBUC_GREEN}"
  fi
  printf "  ${z_key_color}%-30s${ZBUC_RESET} %s\n" "RBRA_PRIVATE_KEY" "${z_key_status}"
  rbcr_section_end

  # Token Configuration
  rbcr_section_begin "Token Configuration"
  rbcr_section_item RBRA_TOKEN_LIFETIME_SEC  decimal  req  "OAuth token lifetime in seconds (300-3600)"
  rbcr_section_end

  # Unexpected variables (from kindle, not gated)
  if test ${#ZRBRA_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected RBRA_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZRBRA_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate after full render
  zrbra_validate_fields
  echo "${ZBUC_GREEN}RBRA credential file valid${ZBUC_RESET}"
}

# Command: list - show all RBRA file paths from RBRR
rbra_list() {
  # Kindle constants and load RBRR
  zrbcc_sentinel
  local z_rbrr_file="${RBCC_RBRR_FILE}"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"
  source "${z_rbrr_file}" || buc_die "Failed to source RBRR: ${z_rbrr_file}"
  zrbrr_kindle

  echo ""
  echo "${ZBUC_WHITE}RBRA Credential Files (from RBRR)${ZBUC_RESET}"
  echo ""

  local z_roles=("governor" "retriever" "director")
  local z_vars=("RBRR_GOVERNOR_RBRA_FILE" "RBRR_RETRIEVER_RBRA_FILE" "RBRR_DIRECTOR_RBRA_FILE")

  local z_i
  for z_i in "${!z_roles[@]}"; do
    local z_role="${z_roles[$z_i]}"
    local z_var="${z_vars[$z_i]}"
    local z_path="${!z_var:-}"
    local z_exists="missing"
    local z_color="${ZBUC_RED}"
    if test -f "${z_path}"; then
      z_exists="ok"
      z_color="${ZBUC_GREEN}"
    fi
    printf "  %-12s ${z_color}%-8s${ZBUC_RESET} %s\n" "${z_role}" "[${z_exists}]" "${z_path}"
  done
  echo ""
}

######################################################################
# Main dispatch

zrbra_cli_kindle
zrbcc_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    z_file="${2:-}"
    test -n "${z_file}" || buc_die "rbra_cli.sh validate: file argument required"
    rbra_validate "${z_file}"
    ;;
  render)
    z_file="${2:-}"
    test -n "${z_file}" || buc_die "rbra_cli.sh render: file argument required"
    rbra_render "${z_file}"
    ;;
  list)
    rbra_list
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbra_cli.sh {validate|render|list} [file]"
    ;;
esac

# eof
