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

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

# Command: validate - enrollment-based validation (dies on first error)
# SECURITY: buv_report shows raw values — use enforce + manual report for secrets
rbro_validate() {
  buc_doc_brief "Validate RBRO OAuth credential file via enrollment enforcement"
  buc_doc_shown || return 0

  buc_step "Validating RBRO OAuth credential file: ${RBDC_PAYOR_RBRO_FILE}"
  buc_step "  PASS  RBRO_CLIENT_SECRET=[REDACTED - ${#RBRO_CLIENT_SECRET} chars] [string]"
  buc_step "  PASS  RBRO_REFRESH_TOKEN=[REDACTED - ${#RBRO_REFRESH_TOKEN} chars] [string]"
  buc_step "RBRO OAuth credential file valid"
}

# Command: render - diagnostic display with secret masking
# SECURITY: Temporarily masks secret values before calling buv_render
rbro_render() {
  buc_doc_brief "Display diagnostic view of RBRO OAuth regime configuration"
  buc_doc_shown || return 0

  local z_real_secret="${RBRO_CLIENT_SECRET}"
  local z_real_token="${RBRO_REFRESH_TOKEN}"
  RBRO_CLIENT_SECRET="[REDACTED - ${#z_real_secret} chars]"
  RBRO_REFRESH_TOKEN="[REDACTED - ${#z_real_token} chars]"

  buv_render RBRO "RBRO - Recipe Bottle OAuth Regime"

  RBRO_CLIENT_SECRET="${z_real_secret}"
  RBRO_REFRESH_TOKEN="${z_real_token}"
}

######################################################################
# Furnish and Main

zrbro_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbrr_regime.sh"
  source "${z_rbw_kit_dir}/rbdc_DerivedConstants.sh"
  source "${z_rbw_kit_dir}/rbro_regime.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zburd_lock
  zrbcc_kindle

  source "${RBBC_rbrr_file}" || buc_die "Failed to source RBRR: ${RBBC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce
  zrbrr_lock
  zrbdc_kindle

  rbro_load

  zbupr_kindle
}

buc_execute rbro_ "Recipe Bottle OAuth Regime" zrbro_furnish "$@"

# eof
