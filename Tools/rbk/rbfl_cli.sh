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

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbfl_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env "BURD_NOW_STAMP        " "Bash Dispatch Utility provided string unique between invocations"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbgd_DepotConstants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"
  source "${RBBC_rbrr_file}"
  source "${z_rbk_kit_dir}/rbgo_OAuth.sh"
  source "${z_rbk_kit_dir}/rbgu_Utility.sh"
  source "${z_rbk_kit_dir}/rbfl_FoundryLedger.sh"

  zbuv_kindle

  buc_log_args 'Validate BUD environment'
  zburd_kindle

  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle

  source "${z_rbk_kit_dir}/rbrv_regime.sh"

  buc_log_args 'Kindle modules in dependency order'
  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbfc_kindle
  zrbfl_kindle
}

buc_execute rbfl_ "Recipe Bottle Foundry Ledger" zrbfl_furnish "$@"

# eof
