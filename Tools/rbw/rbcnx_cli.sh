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
# RBCNX CLI - Extended command line interface for RBRN cross-nameplate operations
#
# Heavy furnish: loads GCP/OAuth/RBRR dep stack required for fleet-wide operations.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

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

######################################################################
# Furnish and Main

zrbcnx_furnish() {
  buc_doc_env "BUZ_FOLIO" "Nameplate moniker (e.g., nsproto); empty for survey/audit"

  # Sources
  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbrn_regime.sh"
  source "${z_rbw_kit_dir}/rbrr_regime.sh"
  source "${z_rbw_kit_dir}/rbgc_Constants.sh"
  source "${z_rbw_kit_dir}/rbgd_DepotConstants.sh"
  source "${z_rbw_kit_dir}/rbgo_OAuth.sh"

  # Kindles
  zbuv_kindle
  zburd_kindle
  zbupr_kindle
  zrbcc_kindle
  test "${z_rbw_kit_dir}" = "${RBCC_KIT_DIR}" || buc_die "z_rbw_kit_dir mismatch: ${z_rbw_kit_dir} != ${RBCC_KIT_DIR}"
  source "${RBCC_rbrr_file}"
  zrbgc_kindle
  zrbrr_kindle
  zrbrr_enforce
  zrbgo_kindle
  zrbgd_kindle
}

buc_execute rbrn_ "Recipe Bottle Nameplate Extended" zrbcnx_furnish "$@"

# eof
