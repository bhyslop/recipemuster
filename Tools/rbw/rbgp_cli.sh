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
# Recipe Bottle GCP Payor - Billing and Destructive Operations CLI

set -euo pipefail

ZRBGP_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
ZRBGP_BUK_DIR="${ZRBGP_CLI_SCRIPT_DIR}/../buk"

# Source all dependencies
source "${ZRBGP_BUK_DIR}/buc_command.sh"
source "${ZRBGP_BUK_DIR}/buv_validation.sh"
source "${ZRBGP_BUK_DIR}/bug_guide.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBCC_rbrr_file}"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgu_Utility.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgi_IAM.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbrp_regime.sh"
source "${ZRBGP_CLI_SCRIPT_DIR}/rbgp_Payor.sh"

zrbgp_furnish() {
  buc_log_args 'Initialize modules'
  zbuv_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce

  zrbgc_kindle
  rbrp_load

  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
}

buc_execute rbgp_ "Recipe Bottle Payor" zrbgp_furnish "$@"

# eof
