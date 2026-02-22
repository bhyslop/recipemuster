#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# RBCNC CLI - Common command line interface for RBRN nameplate operations
#
# Light furnish: stock ops (validate, render, list).

set -euo pipefail

ZRBCNC_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/rbrn_regime.sh"
source "${ZRBCNC_CLI_SCRIPT_DIR}/../buk/bupr_PresentationRegime.sh"

######################################################################
# Command Functions

# Command: validate - enrollment-based validation report
rbrn_validate() {
  buc_doc_brief "Validate RBRN nameplate regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRN nameplate regime"
  buv_report RBRN "Nameplate Regime"
  buc_step "RBRN nameplate valid"
}

# Command: render - diagnostic display then validate
rbrn_render() {
  buc_doc_brief "Display diagnostic view of RBRN nameplate regime configuration"
  buc_doc_shown || return 0

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRN - Recipe Bottle Regime Nameplate${ZBUC_RESET}"
  echo ""

  # Core Service Identity
  bupr_section_begin "Core Service Identity"
  bupr_section_item RBRN_MONIKER              xname   req  "Unique identifier for Bottle Service"
  bupr_section_item RBRN_DESCRIPTION          string  opt  "Human-readable description"
  bupr_section_item RBRN_RUNTIME              enum    req  "Container runtime: docker or podman"
  bupr_section_end

  # Container Image Configuration
  bupr_section_begin "Container Image Configuration"
  bupr_section_item RBRN_SENTRY_VESSEL        fqin    req  "Vessel identifier for Sentry Image"
  bupr_section_item RBRN_BOTTLE_VESSEL        fqin    req  "Vessel identifier for Bottle Image"
  bupr_section_item RBRN_SENTRY_CONSECRATION  fqin    req  "Consecration tag for Sentry Image"
  bupr_section_item RBRN_BOTTLE_CONSECRATION  fqin    req  "Consecration tag for Bottle Image"
  bupr_section_end

  # Entry Service Configuration (gated by ENTRY_MODE)
  bupr_section_begin "Entry Service Configuration" RBRN_ENTRY_MODE enabled
  bupr_section_item RBRN_ENTRY_MODE              enum  req   "Entry functionality: disabled or enabled"
  bupr_section_item RBRN_ENTRY_PORT_WORKSTATION  port  cond  "External port on Transit Network"
  bupr_section_item RBRN_ENTRY_PORT_ENCLAVE      port  cond  "Enclave port between Sentry and Bottle"
  bupr_section_end

  # Enclave Network Configuration
  bupr_section_begin "Enclave Network Configuration"
  bupr_section_item RBRN_ENCLAVE_BASE_IP      ipv4     req  "Base IPv4 for enclave network"
  bupr_section_item RBRN_ENCLAVE_NETMASK      decimal  req  "Network mask width (8-30)"
  bupr_section_item RBRN_ENCLAVE_SENTRY_IP    ipv4     req  "IP address for Sentry Container"
  bupr_section_item RBRN_ENCLAVE_BOTTLE_IP    ipv4     req  "IP address for Bottle Container"
  bupr_section_end

  # Uplink Core
  bupr_section_begin "Uplink Core"
  bupr_section_item RBRN_UPLINK_PORT_MIN      port  req  "Minimum port for outbound connections"
  bupr_section_item RBRN_UPLINK_DNS_MODE      enum  req  "DNS mode: disabled, global, or allowlist"
  bupr_section_item RBRN_UPLINK_ACCESS_MODE   enum  req  "IP access mode: disabled, global, or allowlist"
  bupr_section_end

  # Uplink DNS Allowlist (gated by DNS_MODE)
  bupr_section_begin "Uplink DNS Allowlist" RBRN_UPLINK_DNS_MODE allowlist
  bupr_section_item RBRN_UPLINK_ALLOWED_DOMAINS  domain_list  cond  "Allowed DNS domains"
  bupr_section_end

  # Uplink Access Allowlist (gated by ACCESS_MODE)
  bupr_section_begin "Uplink Access Allowlist" RBRN_UPLINK_ACCESS_MODE allowlist
  bupr_section_item RBRN_UPLINK_ALLOWED_CIDRS  cidr_list  cond  "Allowed CIDR ranges"
  bupr_section_end

  # Volume Mount Configuration
  bupr_section_begin "Volume Mount Configuration"
  bupr_section_item RBRN_VOLUME_MOUNTS  string  opt  "Volume mount arguments for Bottle"
  bupr_section_end

  echo "${ZBUC_GREEN}RBRN nameplate valid${ZBUC_RESET}"
}

######################################################################
# Furnish and Main

zrbrn_furnish() {
  buc_doc_env "RBR0_FOLIO" "Nameplate moniker (e.g., nsproto); empty for list/survey/audit"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle
  zbupr_kindle

  # If RBR0_FOLIO is set, load and kindle the specified nameplate
  if test -n "${RBR0_FOLIO:-}"; then
    local z_nameplate_file="${RBCC_KIT_DIR}/${RBCC_rbrn_prefix}${RBR0_FOLIO}${RBCC_rbrn_ext}"
    test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"
    source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"
    zrbrn_kindle
    zrbrn_enforce
  fi
}

buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"

# eof
