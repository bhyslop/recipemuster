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

  echo ""
  echo "${BUC_white}RBRO - Recipe Bottle OAuth Regime${BUC_reset}"
  echo ""
  buc_step "  RBRO_CLIENT_SECRET=[REDACTED - ${#RBRO_CLIENT_SECRET} chars] [string]"
  buc_step "  RBRO_REFRESH_TOKEN=[REDACTED - ${#RBRO_REFRESH_TOKEN} chars] [string]"
}

######################################################################
# Furnish and Main

zrbro_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"
  source "${z_rbk_kit_dir}/rbro_regime.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zrbcc_kindle

  source "${RBBC_rbrr_file}" || buc_die "Failed to source RBRR: ${RBBC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle

  rbro_load

  zbupr_kindle
}

buc_execute rbro_ "Recipe Bottle OAuth Regime" zrbro_furnish "$@"

# eof
