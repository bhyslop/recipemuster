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
# Recipe Bottle Manual Procedures - Command Line Interface

set -euo pipefail

ZRBGA_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBGA_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBGA_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBGA_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBGA_SCRIPT_DIR}/rbga_GoogleAdmin.sh"
source "${ZRBGA_SCRIPT_DIR}/rbgo_GoogleOAuth.sh"

zrbga_furnish() {

  bcu_doc_env "BDU_TEMP_DIR   " "Temporary directory for intermediate files"
  bcu_doc_env "BDU_OUTPUT_DIR " "Directory for command outputs"

  zrbl_kindle
  bvu_file_exists "${RBL_RBRR_FILE}"
  source          "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR regime file"

  zrbgo_kindle
  zrbga_kindle
}

bcu_execute rbga_ "Google Admin Procedures" zrbga_furnish "$@"


# eof
