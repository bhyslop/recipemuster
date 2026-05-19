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
# RBRD CLI - Command line interface for RBRD depot regime operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

# Command: validate - enrollment-based validation report
rbrd_validate() {
  buc_doc_brief "Validate RBRD depot regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRD depot regime file: ${RBBC_rbrd_file}"
  buv_report RBRD "Depot Regime"
  buc_step "RBRD depot regime valid"
}

# Command: render - diagnostic display of all RBRD fields
rbrd_render() {
  buc_doc_brief "Display diagnostic view of RBRD depot regime configuration"
  buc_doc_shown || return 0

  buv_render RBRD "RBRD - Recipe Bottle Regime Depot" "${RBBC_rbrd_file}"
}

######################################################################
# Furnish and Main

zrbrd_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh"
  source "${RBBC_rbrd_file}"

  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zrbcc_kindle

  zrbrd_kindle
  zrbrd_enforce

  zbupr_kindle
}

buc_execute rbrd_ "Recipe Bottle Depot Regime" zrbrd_furnish "$@"

# eof
