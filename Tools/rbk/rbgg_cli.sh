#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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

source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buym_yelp.sh" || exit 1

zrbgg_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Temporary directory for intermediate files"
  buc_doc_env_row "BURD_OUTPUT_DIR       " "Directory for command outputs"
  buc_doc_env_done || return 0

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/bubc_constants.sh" || buc_die_now "Failed to source bubc_constants.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh" || buc_die_now "Failed to source buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh" || buc_die_now "Failed to source burd_regime.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh" || buc_die_now "Failed to source buf_fact.sh"
  source "${z_rbk_kit_dir}/rbcc_constants.sh" || buc_die_now "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rbgc_constants.sh" || buc_die_now "Failed to source rbgc_constants.sh"
  source "${z_rbk_kit_dir}/rbgd_depot.sh" || buc_die_now "Failed to source rbgd_depot.sh"
  source "${z_rbk_kit_dir}/rbgo_oauth.sh" || buc_die_now "Failed to source rbgo_oauth.sh"
  source "${z_rbk_kit_dir}/rbuh_http.sh" || buc_die_now "Failed to source rbuh_http.sh"
  source "${z_rbk_kit_dir}/rbge_rest.sh" || buc_die_now "Failed to source rbge_rest.sh"
  source "${z_rbk_kit_dir}/rba_auth.sh" || buc_die_now "Failed to source rba_auth.sh"
  source "${z_rbk_kit_dir}/rbgi_iam.sh" || buc_die_now "Failed to source rbgi_iam.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh" || buc_die_now "Failed to source rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh" || buc_die_now "Failed to source rbrd_regime.sh"
  source "${z_rbk_kit_dir}/rbrf_regime.sh" || buc_die_now "Failed to source rbrf_regime.sh"
  source "${z_rbk_kit_dir}/rbrw_regime.sh" || buc_die_now "Failed to source rbrw_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_derived.sh" || buc_die_now "Failed to source rbdc_derived.sh"
  source "${RBCC_rbrr_file}" || buc_die_now "Failed to source ${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}" || buc_die_now "Failed to source ${RBCC_rbrd_file}"
  rbcc_source_active_rbrf_flint "${RBRR_ACTIVE_FOEDUS:-}"
  source "${RBCC_rbrw_file}" || buc_die_now "Failed to source ${RBCC_rbrw_file}"
  source "${z_rbk_kit_dir}/rbgg_governor.sh" || buc_die_now "Failed to source rbgg_governor.sh"
  source "${z_rbk_kit_dir}/rbgw_capabilities.sh" || buc_die_now "Failed to source rbgw_capabilities.sh"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrd_kindle
  zrbrf_kindle
  zrbrw_kindle
  zrbrr_enforce
  zrbrd_enforce
  zrbrf_enforce
  zrbrw_enforce
  zrbdc_kindle

  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbuh_kindle
  zrbge_kindle
  zrba_kindle
  zrbgi_kindle
  zrbgg_kindle
  zrbgw_kindle
}

buc_execute rbgg_ "Governor Procedures" zrbgg_furnish "$@"

# eof
