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
# RBRS CLI - Command line interface for RBRS station operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

rbrs_validate() {
  buc_doc_brief "Validate RBRS station regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRS station regime file: ${RBCC_rbrs_file}"
  buv_report RBRS "Station Regime"
  buc_step "RBRS station regime valid"
}

rbrs_render() {
  buc_doc_brief "Display diagnostic view of RBRS station regime configuration"
  buc_doc_shown || return 0

  buv_render RBRS "RBRS - Recipe Bottle Station Regime"
}

######################################################################
# Furnish and Main

zrbrs_furnish() {
  local z_rbw_kit_dir="${BURD_TOOLS_DIR}/rbw"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbw_kit_dir}/rbcc_Constants.sh"
  source "${z_rbw_kit_dir}/rbrs_regime.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zrbcc_kindle

  source "${RBCC_rbrs_file}" || buc_die "Failed to source RBRS: ${RBCC_rbrs_file}"

  zrbrs_kindle
  zrbrs_enforce

  zbupr_kindle
}

buc_execute rbrs_ "Recipe Bottle Station Regime" zrbrs_furnish "$@"

# eof
