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
# RBRN CLI - Command line interface for RBRN nameplate regime operations
#
# Differential furnish: light deps (validate, render) always loaded;
# heavy deps (survey, audit) loaded only when needed.

set -euo pipefail

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

# Command: validate - enrollment-based validation report
rbrn_validate() {
  buc_doc_brief "Validate RBRN nameplate regime configuration via enrollment report"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Nameplate moniker required (e.g., nsproto, srjcl, pluml)"
  buc_step "Validating RBRN nameplate regime"
  buv_report RBRN "Nameplate Regime"
  buc_step "RBRN nameplate valid"
}

# Command: render - diagnostic display
rbrn_render() {
  buc_doc_brief "Display diagnostic view of RBRN nameplate regime configuration"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Nameplate moniker required (e.g., nsproto, srjcl, pluml)"
  buv_render RBRN "RBRN - Recipe Bottle Regime Nameplate"
}

# Command: survey - fleet info table across all nameplates
rbrn_survey() {
  buc_doc_brief "Display fleet info table for all nameplate configurations"
  buc_doc_shown || return 0

  zrbrn_fleet_survey
}

# Command: audit - survey display then preflight validation
rbrn_audit() {
  buc_doc_brief "Survey all nameplates and run cross-nameplate preflight validation"
  buc_doc_shown || return 0

  zrbrn_fleet_audit

  # GCB quota headroom check (requires Director SA token)
  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_DIRECTOR_RBRA_FILE}") \
    || buc_die "Failed to get token for GCB quota check"
  rbgd_check_gcb_quota "${z_token}"
  buc_step "Full audit passed (nameplates + GCB quota)"
}

# Command: list - show available nameplate monikers
rbrn_list() {
  buc_doc_brief "List available nameplate monikers"
  buc_doc_shown || return 0

  buc_step "Available nameplates:"
  zrbrn_list_monikers
}

######################################################################
# Furnish and Main

zrbrn_furnish() {
  local z_command="${1:-}"

  buc_doc_env "BUZ_FOLIO" "Nameplate moniker (e.g., nsproto); empty for list/survey/audit"

  # Light sources (always)
  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbrn_regime.sh"

  # Heavy sources (survey/audit only)
  case "${z_command}" in
    rbrn_survey|rbrn_audit)
      source "${z_rbw_kit_dir}/rbrr_regime.sh"
      source "${z_rbw_kit_dir}/rbgc_Constants.sh"
      source "${z_rbw_kit_dir}/rbgd_DepotConstants.sh"
      source "${z_rbw_kit_dir}/rbgo_OAuth.sh"
      ;;
  esac

  # Light kindles (always)
  zbuv_kindle
  zburd_kindle
  zbupr_kindle
  zrbcc_kindle
  test "${z_rbw_kit_dir}" = "${RBCC_KIT_DIR}" || buc_die "z_rbw_kit_dir mismatch: ${z_rbw_kit_dir} != ${RBCC_KIT_DIR}"

  # Heavy kindles (survey/audit only)
  case "${z_command}" in
    rbrn_survey|rbrn_audit)
      source "${RBCC_rbrr_file}"
      zrbgc_kindle
      zrbrr_kindle
      zrbrr_enforce
      zrbgo_kindle
      zrbgd_kindle
      ;;
  esac

  # If BUZ_FOLIO is set, load and kindle the specified nameplate
  if test -n "${BUZ_FOLIO:-}"; then
    local z_nameplate_file="${RBCC_KIT_DIR}/${RBCC_rbrn_prefix}${BUZ_FOLIO}${RBCC_rbrn_ext}"
    test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"
    source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"
    zrbrn_kindle
    zrbrn_enforce
  fi
}

buc_execute rbrn_ "Recipe Bottle Nameplate Regime" zrbrn_furnish "$@"

# eof
