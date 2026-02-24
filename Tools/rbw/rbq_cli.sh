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
# RBQ CLI - Command line interface for RBQ qualification operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Furnish and Main

zrbq_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"
  source "${z_rbw_kit_dir}/rbz_zipper.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbrn_regime.sh"
  source "${z_rbw_kit_dir}/rbq_Qualify.sh"

  zbuz_kindle
  zrbz_kindle
  zrbcc_kindle
  zrbq_kindle
}

buc_execute rbq_ "Recipe Bottle Qualification" zrbq_furnish "$@"

# eof
