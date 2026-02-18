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
ZRBF_BUK_DIR="${ZRBF_CLI_SCRIPT_DIR}/../buk"

# Source all dependencies
source "${ZRBF_BUK_DIR}/buc_command.sh"
source "${ZRBF_BUK_DIR}/buv_validation.sh"
source "${ZRBF_BUK_DIR}/burd_regime.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${ZRBF_CLI_SCRIPT_DIR}/rbf_Foundry.sh"

zrbf_furnish() {
  buc_doc_env "BURD_TEMP_DIR  " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env "BURD_NOW_STAMP " "Bash Dispatch Utility provided string unique between invocations"

  buc_log_args 'Validate BUD environment'
  zburd_kindle

  buc_log_args 'Validate required tools'
  command -v git >/dev/null 2>&1 || buc_die "git not found - required for assuring controlled build context"

  buc_log_args 'Container runtime settings'
  RBG_RUNTIME="${RBG_RUNTIME:-podman}"
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  zrbcc_kindle

  buc_log_args 'Load RBRR using canonical loader'
  rbrr_load

  buc_log_args 'Validate RBRR variables using validator'
  source "${ZRBF_CLI_SCRIPT_DIR}/rbrr.validator.sh" || buc_die "Failed to validate RBRR variables"

  buc_log_args 'Kindle modules in dependency order'
  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbf_kindle
}

buc_execute rbf_ "Recipe Bottle Foundry" zrbf_furnish "$@"

# eof

