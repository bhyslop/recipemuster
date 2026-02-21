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
# Recipe Bottle GCP Governor - Command Line Interface

set -euo pipefail

ZRBGG_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
ZRBGG_BUK_DIR="${ZRBGG_CLI_SCRIPT_DIR}/../buk"

# Source all dependencies
source "${ZRBGG_BUK_DIR}/buc_command.sh"
source "${ZRBGG_BUK_DIR}/buv_validation.sh"
source "${ZRBGG_BUK_DIR}/burd_regime.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgu_Utility.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgi_IAM.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBCC_rbrr_file}"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgg_Governor.sh"

zrbgg_furnish() {

  buc_doc_env "BURD_TEMP_DIR   " "Temporary directory for intermediate files"
  buc_doc_env "BURD_OUTPUT_DIR " "Directory for command outputs"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce

  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgg_kindle
}

buc_execute rbgg_ "Governor Procedures" zrbgg_furnish "$@"

# eof

