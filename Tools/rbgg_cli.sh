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

# Source all dependencies
source "${ZRBGG_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgu_Utility.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgi_IAM.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rgbs_ServiceAccounts.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgb_Buckets.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbga_ArtifactRegistry.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgp_Payor.sh"
source "${ZRBGG_CLI_SCRIPT_DIR}/rbgg_Governor.sh"

zrbgg_furnish() {

  bcu_doc_env "BDU_TEMP_DIR   " "Temporary directory for intermediate files"
  bcu_doc_env "BDU_OUTPUT_DIR " "Directory for command outputs"

  zrbl_kindle
  bvu_file_exists "${RBL_RBRR_FILE}"
  source          "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR regime file"

  zrbgc_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrgbs_kindle
  zrbgb_kindle
  zrbga_kindle
  zrbgp_kindle
  zrbgg_kindle
}

bcu_execute rbgg_ "Governor Procedures" zrbgg_furnish "$@"

# eof

