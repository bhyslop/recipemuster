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

ZRBF_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBF_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbf_Foundry.sh"

zrbf_furnish() {
  bcu_doc_env "BDU_TEMP_DIR  " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  bcu_doc_env "BDU_NOW_STAMP " "Bash Dispatch Utility provided string unique between invocations"

  bcu_log_args 'Validate BDU environment'
  bvu_dir_exists "${BDU_TEMP_DIR}"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  bcu_log_args 'Validate required tools'
  command -v git >/dev/null 2>&1 || bcu_die "git not found - required for assuring controlled build context"

  bcu_log_args 'Container runtime settings'
  RBG_RUNTIME="${RBG_RUNTIME:-podman}"
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  bcu_log_args 'Use RBL to locate and source RBRR file'
  zrbl_kindle
  test -f "${RBL_RBRR_FILE}" || bcu_die "RBRR file not found: ${RBL_RBRR_FILE}"
  source  "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR file"

  bcu_log_args 'Validate RBRR variables using validator'
  source "${ZRBF_CLI_SCRIPT_DIR}/rbrr.validator.sh" || bcu_die "Failed to validate RBRR variables"

  bcu_log_args 'Kindle modules in dependency order'
  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbf_kindle
}

bcu_execute rbf_ "Recipe Bottle Foundry" zrbf_furnish "$@"

# eof

