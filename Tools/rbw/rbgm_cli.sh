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

ZRBGM_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
ZRBGM_BUK_DIR="${ZRBGM_SCRIPT_DIR}/../buk"

# Source all dependencies
source "${ZRBGM_BUK_DIR}/buc_command.sh"
source "${ZRBGM_BUK_DIR}/buv_validation.sh"
source "${ZRBGM_BUK_DIR}/bug_guide.sh"
source "${ZRBGM_SCRIPT_DIR}/rbgc_Constants.sh"
source "${ZRBGM_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBGM_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBCC_rbrr_file}"
source "${ZRBGM_SCRIPT_DIR}/rbrp_regime.sh"
source "${ZRBGM_SCRIPT_DIR}/rbgm_ManualProcedures.sh"

zrbgm_furnish() {

  buc_doc_env "BURD_TEMP_DIR   " "Temporary directory for intermediate files"
  buc_doc_env "BURD_OUTPUT_DIR " "Directory for command outputs"

  zbuv_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce

  zrbgc_kindle
  rbrp_load

  zrbgm_kindle
}

buc_execute rbgm_ "Manual Procedures" zrbgm_furnish "$@"


# eof

