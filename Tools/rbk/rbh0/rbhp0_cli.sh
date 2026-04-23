#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Handbook Payor Ceremonies - Command Line Interface
#
# Full furnish: payor-only ceremonies require the complete regime +
# OAuth + IAM dependency stack.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbhp_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "Configuration directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for intermediate files"
  buc_doc_env "BURD_OUTPUT_DIR       " "Directory for command outputs"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"  || buc_die "Failed to source rbbc_constants.sh"
  local -r z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  source "${BURD_BUK_DIR}/burd_regime.sh"                  || buc_die "Failed to source burd_regime.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh"               || buc_die "Failed to source buv_validation.sh"
  source "${BURD_BUK_DIR}/buym_yelp.sh"                    || buc_die "Failed to source buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"                 || buc_die "Failed to source buh_handbook.sh"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"              || buc_die "Failed to source rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"              || buc_die "Failed to source rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"                 || buc_die "Failed to source rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"       || buc_die "Failed to source rbdc_DerivedConstants.sh"
  source "${RBBC_rbrr_file}"                               || buc_die "Failed to source ${RBBC_rbrr_file}"
  source "${z_rbk_kit_dir}/rbrp_regime.sh"                 || buc_die "Failed to source rbrp_regime.sh"
  source "${z_rbk_kit_dir}/rbgo_OAuth.sh"                  || buc_die "Failed to source rbgo_OAuth.sh"
  source "${z_rbk_kit_dir}/rbgu_Utility.sh"                || buc_die "Failed to source rbgu_Utility.sh"
  source "${z_rbk_kit_dir}/rbgi_IAM.sh"                    || buc_die "Failed to source rbgi_IAM.sh"
  source "${z_rbk_kit_dir}/rbra_regime.sh"                 || buc_die "Failed to source rbra_regime.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhpb_base.sh"             || buc_die "Failed to source rbhpb_base.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhpe_establish.sh"        || buc_die "Failed to source rbhpe_establish.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhpr_refresh.sh"          || buc_die "Failed to source rbhpr_refresh.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhpq_quota_build.sh"      || buc_die "Failed to source rbhpq_quota_build.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"                   || buc_die "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"                  || buc_die "Failed to source rbz_zipper.sh"

  zbuv_kindle
  zburd_kindle
  zrbcc_kindle

  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle

  zrbgc_kindle

  source "${RBBC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBBC_rbrp_file}"
  zrbrp_kindle
  zrbrp_enforce

  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbhp_kindle
  zrbhp_enforce

  zbuz_kindle
  zrbz_kindle
}

buc_execute rbhp_ "Payor Ceremonies" zrbhp_furnish "$@"

# eof
