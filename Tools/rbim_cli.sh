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

ZRBIM_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBIM_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbgo_GoogleOAuth.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbcr_ContainerRegistry.sh"
source "${ZRBIM_CLI_SCRIPT_DIR}/rbim_ImageManagement.sh"

zrbim_furnish() {
  bcu_doc_env "BDU_TEMP_DIR  " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  bcu_doc_env "BDU_NOW_STAMP " "Bash Dispatch Utility provided string unique between invocations"

  # Validate BDU environment
  bvu_dir_exists "${BDU_TEMP_DIR}"
  bvu_dir_empty  "${BDU_TEMP_DIR}"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  # Container runtime settings
  RBG_RUNTIME="${RBG_RUNTIME:-podman}"
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  # Use RBL to locate and source RBRR file
  zrbl_kindle
  test -f "${RBL_RBRR_FILE}" || bcu_die "RBRR file not found: ${RBL_RBRR_FILE}"
  source  "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR file"

  # Validate RBRR variables using validator
  source "${ZRBIM_CLI_SCRIPT_DIR}/rbrr.validator.sh" || bcu_die "Failed to validate RBRR variables"

  # Kindle modules in dependency order
  zrbgo_kindle
  zrbcr_kindle
  zrbim_kindle
}

bcu_execute rbim_ "Recipe Bottle Image Management" zrbim_furnish "$@"

# eof

