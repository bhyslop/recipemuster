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

source "${BURD_BUK_DIR}/buc_command.sh"

zrbgp_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local -r z_command="${1:-}"

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"
  source "${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}"
  source "${z_rbk_kit_dir}/rbgl_GarLayout.sh"
  source "${z_rbk_kit_dir}/rbgo_OAuth.sh"
  source "${z_rbk_kit_dir}/rbgu_Utility.sh"
  source "${z_rbk_kit_dir}/rbgi_IAM.sh"
  source "${z_rbk_kit_dir}/rbrp_regime.sh"
  source "${z_rbk_kit_dir}/rbgp_Payor.sh"
  source "${z_rbk_kit_dir}/rbndb_base.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"

  buc_log_args 'Initialize modules'
  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrd_kindle
  if test "${z_command}" != "rbgp_depot_list"; then
    zrbrr_enforce
    zrbrd_enforce
  fi
  zrbdc_kindle

  zrbgc_kindle
  zrbgl_kindle

  source "${RBCC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBCC_rbrp_file}"
  zrbrp_kindle
  zrbrp_enforce

  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbndb_kindle

  zbuz_kindle
  zrbz_kindle
}

buc_execute rbgp_ "Recipe Bottle Payor" zrbgp_furnish "$@"

# eof
