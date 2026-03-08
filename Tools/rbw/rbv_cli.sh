#!/bin/bash
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
# Recipe Bottle VM - Command Line Interface

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

# CLI-specific environment function
zrbv_furnish() {
  # Handle documentation mode
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "RBV_TEMP_DIR          " "Empty temporary directory"
  buc_doc_env "RBV_RBRR_FILE         " "File containing the RBRR constants"
  buc_doc_env "RBV_RBRS_FILE         " "File containing the RBRS constants"
  buc_doc_env_done || return 0

  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${z_rbw_kit_dir}/rbrr_regime.sh"
  source "${z_rbw_kit_dir}/rbdc_DerivedConstants.sh"
  source "${z_rbw_kit_dir}/rbrs_regime.sh"
  source "${z_rbw_kit_dir}/rbv_PodmanVM.sh"

  # Initialize enrollment and validate environment
  zbuv_kindle
  buv_dir_exists  "${RBV_TEMP_DIR}"
  buv_dir_empty   "${RBV_TEMP_DIR}"
  buv_file_exists "${RBV_RBRR_FILE}"
  buv_file_exists "${RBV_RBRS_FILE}"

  # Source config files (CLI handles all sourcing)
  source              "${RBV_RBRR_FILE}" || buc_die "Failed to source RBRR config"
  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle

  source              "${RBV_RBRS_FILE}" || buc_die "Failed to source RBRS config"
  zrbrs_kindle
  zrbrs_enforce

  # Start implementation module
  zrbv_kindle
}

# Execute command
buc_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_furnish "$@"

# eof
