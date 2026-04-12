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
# Recipe Bottle Windows Handbook - Command Line Interface
#
# Thin furnish: handbook display procedures need only buh_* combinators
# and the rbhw module kindle constants. No regime, no OAuth, no IAM.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbhw_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "Configuration directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"  || buc_die "Failed to source rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  source "${BURD_BUK_DIR}/buh_handbook.sh"       || buc_die "Failed to source buh_handbook.sh"
  source "${z_rbk_kit_dir}/rbhw_windows.sh"      || buc_die "Failed to source rbhw_windows.sh"

  zrbhw_kindle
}

buc_execute rbhw_ "Windows Handbook" zrbhw_furnish "$@"

# eof
