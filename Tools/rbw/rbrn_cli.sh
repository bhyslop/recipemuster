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
# RBRN CLI - Command line interface for RBRN nameplate operations

set -euo pipefail

ZRBRN_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBRN_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBRN_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBRN_CLI_SCRIPT_DIR}/rbrn_regime.sh"

######################################################################
# CLI Functions

zrbrn_cli_kindle() {
  test -z "${ZRBRN_CLI_KINDLED:-}" || buc_die "RBRN CLI already kindled"
  ZRBRN_CLI_KINDLED=1
}

# Command: validate - source file and validate (dies on first error)
rbrn_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrn_validate: file not found: ${z_file}"

  buc_step "Validating RBRN nameplate file: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrn_validate: failed to source ${z_file}"

  # Prepare state (no dying)
  zrbrn_kindle

  # Strict validation (dies on error; suppress buv echo output)
  zrbrn_validate_fields > /dev/null

  buc_step "RBRN nameplate valid"
}

# Display one field: name, description, value
zrbrn_render_field() {
  local z_name="$1"
  local z_desc="$2"
  local z_value="${!z_name:-}"

  if test -n "${z_value}"; then
    printf "  ${ZBUC_GREEN}%-30s${ZBUC_RESET} %s\n" "${z_name}" "${z_value}"
  else
    printf "  ${ZBUC_YELLOW}%-30s${ZBUC_RESET} ${ZBUC_CYAN}(not set)${ZBUC_RESET}\n" "${z_name}"
  fi
  printf "  ${ZBUC_CYAN}%-30s %s${ZBUC_RESET}\n" "" "${z_desc}"
}

# Command: render - diagnostic display then validate
rbrn_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_render: file argument required"
  test -f "${z_file}" || buc_die "rbrn_render: file not found: ${z_file}"

  # Source and kindle (no dying)
  source "${z_file}" || buc_die "rbrn_render: failed to source ${z_file}"
  zrbrn_kindle

  # Display header
  echo ""
  echo "${ZBUC_CYAN}========================================${ZBUC_RESET}"
  echo "${ZBUC_WHITE}RBRN - Recipe Bottle Regime Nameplate${ZBUC_RESET}"
  echo "${ZBUC_CYAN}========================================${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Core Service Identity
  echo "${ZBUC_YELLOW}Core Service Identity${ZBUC_RESET}"
  zrbrn_render_field RBRN_MONIKER                 "Unique identifier for Bottle Service — xname 2-12, Required"
  zrbrn_render_field RBRN_DESCRIPTION             "Human-readable description — string 0-120, Optional"
  zrbrn_render_field RBRN_RUNTIME                 "Container runtime: docker or podman — string 1-16, Required"
  echo ""

  # Container Image Configuration
  echo "${ZBUC_YELLOW}Container Image Configuration${ZBUC_RESET}"
  zrbrn_render_field RBRN_SENTRY_VESSEL           "Vessel identifier for Sentry Image — fqin 1-128, Required"
  zrbrn_render_field RBRN_BOTTLE_VESSEL           "Vessel identifier for Bottle Image — fqin 1-128, Required"
  zrbrn_render_field RBRN_SENTRY_CONSECRATION     "Consecration tag for Sentry Image — fqin 1-128, Required"
  zrbrn_render_field RBRN_BOTTLE_CONSECRATION     "Consecration tag for Bottle Image — fqin 1-128, Required"
  echo ""

  # Entry Service Configuration
  echo "${ZBUC_YELLOW}Entry Service Configuration${ZBUC_RESET}"
  zrbrn_render_field RBRN_ENTRY_MODE              "Entry functionality: disabled or enabled — Required"
  zrbrn_render_field RBRN_ENTRY_PORT_WORKSTATION  "External port on Transit Network — port, When ENTRY_MODE=enabled"
  zrbrn_render_field RBRN_ENTRY_PORT_ENCLAVE      "Port between Sentry and Bottle — port, When ENTRY_MODE=enabled"
  echo ""

  # Enclave Network Configuration
  echo "${ZBUC_YELLOW}Enclave Network Configuration${ZBUC_RESET}"
  zrbrn_render_field RBRN_ENCLAVE_BASE_IP         "Base IPv4 for enclave network — ipv4, Required"
  zrbrn_render_field RBRN_ENCLAVE_NETMASK         "Network mask width (8-30) — decimal, Required"
  zrbrn_render_field RBRN_ENCLAVE_SENTRY_IP       "IP address for Sentry Container — ipv4, Required"
  zrbrn_render_field RBRN_ENCLAVE_BOTTLE_IP       "IP address for Bottle Container — ipv4, Required"
  echo ""

  # Uplink Configuration
  echo "${ZBUC_YELLOW}Uplink Configuration${ZBUC_RESET}"
  zrbrn_render_field RBRN_UPLINK_PORT_MIN         "Minimum port for outbound connections — port, Required"
  zrbrn_render_field RBRN_UPLINK_DNS_MODE         "DNS mode: disabled, global, or allowlist — Required"
  zrbrn_render_field RBRN_UPLINK_ACCESS_MODE      "IP access mode: disabled, global, or allowlist — Required"
  zrbrn_render_field RBRN_UPLINK_ALLOWED_CIDRS    "Allowed CIDR ranges — cidr_list, When ACCESS_MODE=allowlist"
  zrbrn_render_field RBRN_UPLINK_ALLOWED_DOMAINS  "Allowed DNS domains — domain_list, When DNS_MODE=allowlist"
  echo ""

  # Volume Mount Configuration
  echo "${ZBUC_YELLOW}Volume Mount Configuration${ZBUC_RESET}"
  zrbrn_render_field RBRN_VOLUME_MOUNTS           "Volume mount arguments for Bottle — string 0-240, Optional"
  echo ""

  # Unexpected variables
  if test ${#ZRBRN_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected RBRN_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZRBRN_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate (dies on first error, after full display; suppress buv echo output)
  zrbrn_validate_fields > /dev/null
  echo "${ZBUC_GREEN}RBRN nameplate valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbrn_cli_kindle

z_command="${1:-}"

case "${z_command}" in
  validate)
    shift
    rbrn_validate "${@}"
    ;;
  render)
    shift
    rbrn_render "${@}"
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrn_cli.sh {validate|render} [args]"
    ;;
esac

# eof
