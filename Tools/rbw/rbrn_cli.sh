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

# Command: validate - source file and validate
rbrn_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrn_validate: file not found: ${z_file}"

  buc_step "Validating RBRN nameplate file: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrn_validate: failed to source ${z_file}"

  # Validate via kindle
  zrbrn_kindle

  buc_step "RBRN nameplate valid"
}

# Command: render - display configuration values
rbrn_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_render: file argument required"
  test -f "${z_file}" || buc_die "rbrn_render: file not found: ${z_file}"

  buc_step "RBRN Nameplate: ${z_file}"

  # Source the assignment file
  source "${z_file}" || buc_die "rbrn_render: failed to source ${z_file}"

  # Core Service Identity
  printf "%-30s %s\n" "RBRN_MONIKER" "${RBRN_MONIKER:-<not set>}"
  printf "%-30s %s\n" "RBRN_DESCRIPTION" "${RBRN_DESCRIPTION:-<not set>}"
  printf "%-30s %s\n" "RBRN_RUNTIME" "${RBRN_RUNTIME:-<not set>}"

  # Container Image Configuration
  printf "%-30s %s\n" "RBRN_SENTRY_VESSEL" "${RBRN_SENTRY_VESSEL:-<not set>}"
  printf "%-30s %s\n" "RBRN_BOTTLE_VESSEL" "${RBRN_BOTTLE_VESSEL:-<not set>}"
  printf "%-30s %s\n" "RBRN_SENTRY_CONSECRATION" "${RBRN_SENTRY_CONSECRATION:-<not set>}"
  printf "%-30s %s\n" "RBRN_BOTTLE_CONSECRATION" "${RBRN_BOTTLE_CONSECRATION:-<not set>}"

  # Entry Service Configuration
  printf "%-30s %s\n" "RBRN_ENTRY_MODE" "${RBRN_ENTRY_MODE:-<not set>}"
  printf "%-30s %s\n" "RBRN_ENTRY_PORT_WORKSTATION" "${RBRN_ENTRY_PORT_WORKSTATION:-<not set>}"
  printf "%-30s %s\n" "RBRN_ENTRY_PORT_ENCLAVE" "${RBRN_ENTRY_PORT_ENCLAVE:-<not set>}"

  # Enclave Network Configuration
  printf "%-30s %s\n" "RBRN_ENCLAVE_BASE_IP" "${RBRN_ENCLAVE_BASE_IP:-<not set>}"
  printf "%-30s %s\n" "RBRN_ENCLAVE_NETMASK" "${RBRN_ENCLAVE_NETMASK:-<not set>}"
  printf "%-30s %s\n" "RBRN_ENCLAVE_SENTRY_IP" "${RBRN_ENCLAVE_SENTRY_IP:-<not set>}"
  printf "%-30s %s\n" "RBRN_ENCLAVE_BOTTLE_IP" "${RBRN_ENCLAVE_BOTTLE_IP:-<not set>}"

  # Uplink Configuration
  printf "%-30s %s\n" "RBRN_UPLINK_PORT_MIN" "${RBRN_UPLINK_PORT_MIN:-<not set>}"
  printf "%-30s %s\n" "RBRN_UPLINK_DNS_MODE" "${RBRN_UPLINK_DNS_MODE:-<not set>}"
  printf "%-30s %s\n" "RBRN_UPLINK_ACCESS_MODE" "${RBRN_UPLINK_ACCESS_MODE:-<not set>}"
  printf "%-30s %s\n" "RBRN_UPLINK_ALLOWED_CIDRS" "${RBRN_UPLINK_ALLOWED_CIDRS:-<not set>}"
  printf "%-30s %s\n" "RBRN_UPLINK_ALLOWED_DOMAINS" "${RBRN_UPLINK_ALLOWED_DOMAINS:-<not set>}"

  # Volume Mount Configuration
  printf "%-30s %s\n" "RBRN_VOLUME_MOUNTS" "${RBRN_VOLUME_MOUNTS:-<not set>}"
}

# Command: info - display specification (formatted for terminal)
rbrn_info() {
  cat <<EOF

${ZBUC_CYAN}========================================${ZBUC_RESET}
${ZBUC_WHITE}RBRN - Recipe Bottle Regime Nameplate${ZBUC_RESET}
${ZBUC_CYAN}========================================${ZBUC_RESET}

${ZBUC_YELLOW}Overview${ZBUC_RESET}
Defines a Bottle Service deployment: container images, network security,
and entry/uplink configuration. Each nameplate is a complete service definition.

${ZBUC_YELLOW}Core Service Identity${ZBUC_RESET}

  ${ZBUC_GREEN}RBRN_MONIKER${ZBUC_RESET}
    Unique identifier for this Bottle Service instance
    Type: xname (2-12 chars), Required: Yes

  ${ZBUC_GREEN}RBRN_DESCRIPTION${ZBUC_RESET}
    Human-readable description of service purpose
    Type: string (0-120 chars), Required: No

  ${ZBUC_GREEN}RBRN_RUNTIME${ZBUC_RESET}
    Container runtime to use for service deployment
    Type: string ("docker" or "podman"), Required: Yes

${ZBUC_YELLOW}Container Image Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRN_SENTRY_VESSEL${ZBUC_RESET}
    Vessel identifier for Sentry Image
    Type: fqin (1-128 chars), Required: Yes

  ${ZBUC_GREEN}RBRN_BOTTLE_VESSEL${ZBUC_RESET}
    Vessel identifier for Bottle Image
    Type: fqin (1-128 chars), Required: Yes

  ${ZBUC_GREEN}RBRN_SENTRY_CONSECRATION${ZBUC_RESET}
    Consecration tag for Sentry Image
    Type: fqin (1-128 chars), Required: Yes

  ${ZBUC_GREEN}RBRN_BOTTLE_CONSECRATION${ZBUC_RESET}
    Consecration tag for Bottle Image
    Type: fqin (1-128 chars), Required: Yes

${ZBUC_YELLOW}Entry Service Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRN_ENTRY_MODE${ZBUC_RESET}
    Mode for user-initiated entry functionality
    Type: string ("disabled" or "enabled"), Required: Yes

  ${ZBUC_GREEN}RBRN_ENTRY_PORT_WORKSTATION${ZBUC_RESET}
    External port on Transit Network for user entry
    Type: port, Required: When ENTRY_MODE=enabled

  ${ZBUC_GREEN}RBRN_ENTRY_PORT_ENCLAVE${ZBUC_RESET}
    Port between Sentry and Bottle for entry traffic
    Type: port, Required: When ENTRY_MODE=enabled

${ZBUC_YELLOW}Enclave Network Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRN_ENCLAVE_BASE_IP${ZBUC_RESET}
    Base IPv4 address for enclave network range
    Type: ipv4, Required: Yes

  ${ZBUC_GREEN}RBRN_ENCLAVE_NETMASK${ZBUC_RESET}
    Network mask width (8-30)
    Type: decimal, Required: Yes

  ${ZBUC_GREEN}RBRN_ENCLAVE_SENTRY_IP${ZBUC_RESET}
    IP address for Sentry Container
    Type: ipv4, Required: Yes

  ${ZBUC_GREEN}RBRN_ENCLAVE_BOTTLE_IP${ZBUC_RESET}
    IP address for Bottle Container
    Type: ipv4, Required: Yes

${ZBUC_YELLOW}Uplink Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRN_UPLINK_PORT_MIN${ZBUC_RESET}
    Minimum port for bottle-initiated outbound connections
    Type: port, Required: Yes

  ${ZBUC_GREEN}RBRN_UPLINK_DNS_MODE${ZBUC_RESET}
    DNS resolution mode for bottle-initiated requests
    Type: string ("disabled", "global", or "allowlist"), Required: Yes

  ${ZBUC_GREEN}RBRN_UPLINK_ACCESS_MODE${ZBUC_RESET}
    IP access mode for bottle-initiated connections
    Type: string ("disabled", "global", or "allowlist"), Required: Yes

  ${ZBUC_GREEN}RBRN_UPLINK_ALLOWED_CIDRS${ZBUC_RESET}
    CIDR ranges for allowed outbound traffic
    Type: cidr_list, Required: When ACCESS_MODE=allowlist

  ${ZBUC_GREEN}RBRN_UPLINK_ALLOWED_DOMAINS${ZBUC_RESET}
    Domains allowed for DNS resolution
    Type: domain_list, Required: When DNS_MODE=allowlist

${ZBUC_YELLOW}Volume Mount Configuration${ZBUC_RESET}

  ${ZBUC_GREEN}RBRN_VOLUME_MOUNTS${ZBUC_RESET}
    Volume mount arguments for Bottle Container
    Type: string (0-240 chars), Required: No

EOF
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
  info)
    rbrn_info
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrn_cli.sh {validate|render|info} [args]"
    ;;
esac

# eof
