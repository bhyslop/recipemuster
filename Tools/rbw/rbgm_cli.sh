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
# Recipe Bottle GCP Manual Procedures - Command Line Interface

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbgm_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for intermediate files"
  buc_doc_env "BURD_OUTPUT_DIR       " "Directory for command outputs"
  buc_doc_env_done || return 0

  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/bug_guide.sh"
  source "${z_rbw_kit_dir}/rbgc_Constants.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbrr_regime.sh"
  source "${RBCC_rbrr_file}"
  source "${z_rbw_kit_dir}/rbrp_regime.sh"
  source "${z_rbw_kit_dir}/rbgo_OAuth.sh"
  source "${z_rbw_kit_dir}/rbgu_Utility.sh"
  source "${z_rbw_kit_dir}/rbra_regime.sh"
  source "${z_rbw_kit_dir}/rbgm_ManualProcedures.sh"

  zbuv_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce
  zrbrr_lock

  zrbgc_kindle

  source "${RBCC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBCC_rbrp_file}"
  zrbrp_kindle
  zrbrp_enforce

  zrbgo_kindle
  zrbgu_kindle
  zrbgm_kindle
}

buc_execute rbgm_ "Manual Procedures" zrbgm_furnish "$@"


# eof
