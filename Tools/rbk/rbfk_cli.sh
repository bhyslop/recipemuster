#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buym_yelp.sh" || exit 1

zrbfk_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env_row "BURD_NOW_STAMP        " "Bash Dispatch Utility provided string unique between invocations"
  buc_doc_env_done || return 0

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buf_fact.sh" || buc_die_now "Failed to source buf_fact.sh"
  source "${BURD_BUK_DIR}/bug_git.sh" || buc_die_now "Failed to source bug_git.sh"
  source "${z_rbk_kit_dir}/rbcc_constants.sh" || buc_die_now "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rbgc_constants.sh" || buc_die_now "Failed to source rbgc_constants.sh"
  source "${z_rbk_kit_dir}/rbgl_layout.sh" || buc_die_now "Failed to source rbgl_layout.sh"
  source "${z_rbk_kit_dir}/rbgd_depot.sh" || buc_die_now "Failed to source rbgd_depot.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh" || buc_die_now "Failed to source rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh" || buc_die_now "Failed to source rbrd_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_derived.sh" || buc_die_now "Failed to source rbdc_derived.sh"
  source "${RBCC_rbrr_file}" || buc_die_now "Failed to source ${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}" || buc_die_now "Failed to source ${RBCC_rbrd_file}"
  source "${z_rbk_kit_dir}/rbgo_oauth.sh" || buc_die_now "Failed to source rbgo_oauth.sh"
  source "${z_rbk_kit_dir}/rbuh_http.sh" || buc_die_now "Failed to source rbuh_http.sh"
  source "${z_rbk_kit_dir}/rbge_rest.sh" || buc_die_now "Failed to source rbge_rest.sh"
  source "${z_rbk_kit_dir}/rbau_auth.sh" || buc_die_now "Failed to source rbau_auth.sh"
  source "${z_rbk_kit_dir}/rbfh_hygiene.sh" || buc_die_now "Failed to source rbfh_hygiene.sh"
  source "${z_rbk_kit_dir}/rbfc0_core.sh" || buc_die_now "Failed to source rbfc0_core.sh"
  source "${z_rbk_kit_dir}/rbfk_kludge.sh" || buc_die_now "Failed to source rbfk_kludge.sh"
  source "${z_rbk_kit_dir}/rbfb_beckon.sh" || buc_die_now "Failed to source rbfb_beckon.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh" || buc_die_now "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh" || buc_die_now "Failed to source rbz_zipper.sh"


  buc_log_args 'Validate BUD environment'

  buc_log_args 'Validate required tools'
  command -v git >/dev/null || buc_die_now "git not found - required for assuring controlled build context"

  buc_log_args 'Container runtime settings'
  RBG_RUNTIME="${RBG_RUNTIME:-podman}"
  RBG_RUNTIME_ARG="${RBG_RUNTIME_ARG:-}"

  zrbcc_kindle

  zrbrr_kindle
  zrbrd_kindle
  zrbrr_enforce
  # No zrbrd_enforce: kludge is depot-free — its output refs live in the
  # local kludge namespace (RBGC_KLUDGE_REGISTRY_ROOT), and an anchored
  # slot's GAR ref miss fails into the wrest remediation, whose own verb
  # enforces RBRD. RBRD stays kindled: enrolled, locked, inert strings.
  zrbdc_kindle

  source "${z_rbk_kit_dir}/rbrv_regime.sh" || buc_die_now "Failed to source rbrv_regime.sh"

  buc_log_args 'Kindle modules in dependency order'
  zrbgc_kindle
  zrbgl_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbuh_kindle
  zrbge_kindle
  zrbau_kindle
  zrbfh_kindle
  zrbfk_kindle

  zbuz_kindle
  zrbz_kindle
}

buc_execute rbfk_ "Recipe Bottle Foundry Kludge" zrbfk_furnish "$@"

# eof
