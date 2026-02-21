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
source "${ZRBRN_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRN_CLI_SCRIPT_DIR}/rbcr_render.sh"

######################################################################
# CLI Functions

# Command: validate - source file and validate (dies on first error)
rbrn_validate() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_validate: file argument required"
  test -f "${z_file}" || buc_die "rbrn_validate: file not found: ${z_file}"

  buc_step "Validating RBRN nameplate file: ${z_file}"

  # Use rbrn_load_file for standardized loading
  rbrn_load_file "${z_file}"

  buc_step "RBRN nameplate valid"
}

# Command: render - diagnostic display then validate
rbrn_render() {
  local z_file="${1:-}"
  test -n "${z_file}" || buc_die "rbrn_render: file argument required"
  test -f "${z_file}" || buc_die "rbrn_render: file not found: ${z_file}"

  # Source and kindle (no dying — show all fields before validation)
  source "${z_file}" || buc_die "rbrn_render: failed to source ${z_file}"
  zrbrn_kindle
  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRN - Recipe Bottle Regime Nameplate${ZBUC_RESET}"
  echo "${ZBUC_WHITE}File: ${z_file}${ZBUC_RESET}"
  echo ""

  # Core Service Identity
  rbcr_section_begin "Core Service Identity"
  rbcr_section_item RBRN_MONIKER              xname   req  "Unique identifier for Bottle Service"
  rbcr_section_item RBRN_DESCRIPTION          string  opt  "Human-readable description"
  rbcr_section_item RBRN_RUNTIME              enum    req  "Container runtime: docker or podman"
  rbcr_section_end

  # Container Image Configuration
  rbcr_section_begin "Container Image Configuration"
  rbcr_section_item RBRN_SENTRY_VESSEL        fqin    req  "Vessel identifier for Sentry Image"
  rbcr_section_item RBRN_BOTTLE_VESSEL        fqin    req  "Vessel identifier for Bottle Image"
  rbcr_section_item RBRN_SENTRY_CONSECRATION  fqin    req  "Consecration tag for Sentry Image"
  rbcr_section_item RBRN_BOTTLE_CONSECRATION  fqin    req  "Consecration tag for Bottle Image"
  rbcr_section_end

  # Entry Service Configuration (gated by ENTRY_MODE)
  rbcr_section_begin "Entry Service Configuration" RBRN_ENTRY_MODE enabled
  rbcr_section_item RBRN_ENTRY_MODE              enum  req   "Entry functionality: disabled or enabled"
  rbcr_section_item RBRN_ENTRY_PORT_WORKSTATION  port  cond  "External port on Transit Network"
  rbcr_section_item RBRN_ENTRY_PORT_ENCLAVE      port  cond  "Enclave port between Sentry and Bottle"
  rbcr_section_end

  # Enclave Network Configuration
  rbcr_section_begin "Enclave Network Configuration"
  rbcr_section_item RBRN_ENCLAVE_BASE_IP      ipv4     req  "Base IPv4 for enclave network"
  rbcr_section_item RBRN_ENCLAVE_NETMASK      decimal  req  "Network mask width (8-30)"
  rbcr_section_item RBRN_ENCLAVE_SENTRY_IP    ipv4     req  "IP address for Sentry Container"
  rbcr_section_item RBRN_ENCLAVE_BOTTLE_IP    ipv4     req  "IP address for Bottle Container"
  rbcr_section_end

  # Uplink Core
  rbcr_section_begin "Uplink Core"
  rbcr_section_item RBRN_UPLINK_PORT_MIN      port  req  "Minimum port for outbound connections"
  rbcr_section_item RBRN_UPLINK_DNS_MODE      enum  req  "DNS mode: disabled, global, or allowlist"
  rbcr_section_item RBRN_UPLINK_ACCESS_MODE   enum  req  "IP access mode: disabled, global, or allowlist"
  rbcr_section_end

  # Uplink DNS Allowlist (gated by DNS_MODE)
  rbcr_section_begin "Uplink DNS Allowlist" RBRN_UPLINK_DNS_MODE allowlist
  rbcr_section_item RBRN_UPLINK_ALLOWED_DOMAINS  domain_list  cond  "Allowed DNS domains"
  rbcr_section_end

  # Uplink Access Allowlist (gated by ACCESS_MODE)
  rbcr_section_begin "Uplink Access Allowlist" RBRN_UPLINK_ACCESS_MODE allowlist
  rbcr_section_item RBRN_UPLINK_ALLOWED_CIDRS  cidr_list  cond  "Allowed CIDR ranges"
  rbcr_section_end

  # Volume Mount Configuration
  rbcr_section_begin "Volume Mount Configuration"
  rbcr_section_item RBRN_VOLUME_MOUNTS  string  opt  "Volume mount arguments for Bottle"
  rbcr_section_end

  # Unexpected variables (from kindle, not gated)
  if test ${#ZRBRN_UNEXPECTED[@]} -gt 0; then
    echo "${ZBUC_RED}Unexpected RBRN_ variables:${ZBUC_RESET}"
    local z_var
    for z_var in "${ZRBRN_UNEXPECTED[@]}"; do
      printf "  ${ZBUC_RED}%-30s${ZBUC_RESET} = %s\n" "${z_var}" "${!z_var:-}"
    done
    echo ""
  fi

  # Validate (dies on first error, after full display)
  zrbrn_validate_fields
  echo "${ZBUC_GREEN}RBRN nameplate valid${ZBUC_RESET}"
}

######################################################################
# Main dispatch

zrbcc_kindle

z_command="${1:-}"
z_moniker="${2:-}"

case "${z_command}" in
  validate|render)
    if test -z "${z_moniker}"; then
      buc_step "Available nameplates:"
      rbrn_list | while read -r z_m; do
        echo "  ${z_m}"
      done
    else
      z_file="${RBCC_KIT_DIR}/${RBCC_rbrn_prefix}${z_moniker}${RBCC_rbrn_ext}"
      test -f "${z_file}" || buc_die "Nameplate not found: ${z_file}"
      case "${z_command}" in
        validate) rbrn_validate "${z_file}" ;;
        render)   rbrn_render "${z_file}" ;;
      esac
    fi
    ;;
  survey|audit)
    # Additional dependencies for cross-nameplate operations
    source "${ZRBRN_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
    source "${ZRBRN_CLI_SCRIPT_DIR}/rbgd_DepotConstants.sh"
    source "${ZRBRN_CLI_SCRIPT_DIR}/rbrr_regime.sh"
    source "${RBCC_rbrr_file}"
    source "${ZRBRN_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
    zbuv_kindle
    zrbgc_kindle
    zrbrr_kindle
    zrbrr_enforce
    zrbgo_kindle
    zrbgd_kindle
    case "${z_command}" in
      survey) rbrn_survey ;;
      audit)
        rbrn_audit
        # GCB quota headroom check (requires Director SA token)
        z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") \
          || buc_die "Failed to get token for GCB quota check"
        rbgd_check_gcb_quota "${z_token}"
        buc_step "Full audit passed (nameplates + GCB quota)"
        ;;
    esac
    ;;
  *)
    buc_die "Unknown command: ${z_command}. Usage: rbrn_cli.sh {validate|render|survey|audit} [moniker]"
    ;;
esac

# eof
