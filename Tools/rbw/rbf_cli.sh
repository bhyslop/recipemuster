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

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbf_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env "BURD_NOW_STAMP        " "Bash Dispatch Utility provided string unique between invocations"
  buc_doc_env_done || return 0

  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbgc_Constants.sh"
  source "${z_rbw_kit_dir}/rbgd_DepotConstants.sh"
  source "${z_rbw_kit_dir}/rbrr_regime.sh"
  source "${RBCC_rbrr_file}"
  source "${z_rbw_kit_dir}/rbgo_OAuth.sh"
  source "${z_rbw_kit_dir}/rbf_Foundry.sh"

  buc_log_args 'Validate BUD environment'
  zburd_kindle

  buc_log_args 'Validate required tools'
  command -v git >/dev/null 2>&1 || buc_die "git not found - required for assuring controlled build context"

  buc_log_args 'Container runtime settings'
  RBG_RUNTIME="${RBG_RUNTIME:-podman}"
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  zbuv_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce
  zrbrr_lock

  buc_log_args 'Kindle modules in dependency order'
  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbf_kindle
}

buc_execute rbf_ "Recipe Bottle Foundry" zrbf_furnish "$@"

# eof
