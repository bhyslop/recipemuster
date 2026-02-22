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
# Future: rbrn-cli-reunification (₢AfAAZ) will add dynamic nameplate listing on no-arg
rbrn_validate() {
  buc_doc_brief "Validate RBRN nameplate regime configuration via enrollment report"
  buc_doc_shown || return 0

  test -n "${RBR0_FOLIO:-}" || buc_die "Nameplate moniker required (e.g., nsproto, srjcl, pluml)"
  buc_step "Validating RBRN nameplate regime"
  buv_report RBRN "Nameplate Regime"
  buc_step "RBRN nameplate valid"
}

# Command: render - diagnostic display
# Future: rbrn-cli-reunification (₢AfAAZ) will add dynamic nameplate listing on no-arg
rbrn_render() {
  buc_doc_brief "Display diagnostic view of RBRN nameplate regime configuration"
  buc_doc_shown || return 0

  test -n "${RBR0_FOLIO:-}" || buc_die "Nameplate moniker required (e.g., nsproto, srjcl, pluml)"
  buv_render RBRN "RBRN - Recipe Bottle Regime Nameplate"
}

######################################################################
# Furnish and Main

zrbrn_furnish() {
  buc_doc_env "RBR0_FOLIO" "Nameplate moniker (e.g., nsproto); empty for list"

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
