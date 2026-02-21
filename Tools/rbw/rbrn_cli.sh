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
source "${ZRBRN_CLI_SCRIPT_DIR}/../buk/burd_regime.sh"
source "${ZRBRN_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBRN_CLI_SCRIPT_DIR}/rbrn_regime.sh"
source "${ZRBRN_CLI_SCRIPT_DIR}/rbcr_render.sh"

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

  zrbcr_kindle

  # Display header
  echo ""
  echo "${ZBUC_WHITE}RBRN - Recipe Bottle Regime Nameplate${ZBUC_RESET}"
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

  echo "${ZBUC_GREEN}RBRN nameplate valid${ZBUC_RESET}"
}

# Command: survey - fleet info table across all nameplates
rbrn_survey() {
  buc_doc_brief "Display fleet info table for all nameplate configurations"
  buc_doc_shown || return 0

  # Additional dependencies for cross-nameplate operations
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbgd_DepotConstants.sh"
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbrr_regime.sh"
  source "${RBCC_rbrr_file}"
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
  zrbgc_kindle
  zrbrr_kindle
  zrbrr_enforce
  zrbgo_kindle
  zrbgd_kindle

  zrbrn_fleet_survey
}

# Command: audit - survey display then preflight validation
rbrn_audit() {
  buc_doc_brief "Survey all nameplates and run cross-nameplate preflight validation"
  buc_doc_shown || return 0

  # Additional dependencies for cross-nameplate operations
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbgd_DepotConstants.sh"
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbrr_regime.sh"
  source "${RBCC_rbrr_file}"
  source "${ZRBRN_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
  zrbgc_kindle
  zrbrr_kindle
  zrbrr_enforce
  zrbgo_kindle
  zrbgd_kindle

  zrbrn_fleet_audit

  # GCB quota headroom check (requires Director SA token)
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get token for GCB quota check"
  rbgd_check_gcb_quota "${z_token}"
  buc_step "Full audit passed (nameplates + GCB quota)"
}

######################################################################
# Furnish and Main

zrbrn_furnish() {
  buc_doc_env "RBR0_FOLIO" "Nameplate moniker (e.g., nsproto); empty for list/survey/audit"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

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
