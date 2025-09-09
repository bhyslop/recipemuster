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

ZRBV_CLI_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

# Source all dependencies (CLI files handle all sourcing)
source "${ZRBV_CLI_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBV_CLI_SCRIPT_DIR}/bvu_BashValidationUtility.sh"
source "${ZRBV_CLI_SCRIPT_DIR}/rbv_PodmanVM.sh"

# CLI-specific environment function
zrbv_furnish() {
  # Handle documentation mode
  bcu_doc_env "RBV_TEMP_DIR  " "Empty temporary directory"
  bcu_doc_env "RBV_RBRR_FILE " "File containing the RBRR constants"
  bcu_doc_env "RBV_RBRS_FILE " "File containing the RBRS constants"

  # Validate environment
  bvu_dir_exists  "${RBV_TEMP_DIR}"
  bvu_dir_empty   "${RBV_TEMP_DIR}"
  bvu_file_exists "${RBV_RBRR_FILE}"
  bvu_file_exists "${RBV_RBRS_FILE}"

  # Source config files (CLI handles all sourcing)
  source              "${RBV_RBRR_FILE}" || bcu_die "Failed to source RBRR config"
  source "${ZRBV_CLI_SCRIPT_DIR}/rbrr.validator.sh" || bcu_die "Failed to source RBRR validator"

  source              "${RBV_RBRS_FILE}" || bcu_die "Failed to source RBRS config"
  source "${ZRBV_CLI_SCRIPT_DIR}/rbrs.validator.sh" || bcu_die "Failed to source RBRS validator"

  # Start implementation module
  zrbv_kindle
}

# Execute command
bcu_execute rbv_ "Recipe Bottle VM - Podman Virtual Machine Management" zrbv_furnish "$@"

# eof

