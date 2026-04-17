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
# Recipe Bottle Windows Handbook - Command Line Interface
#
# Furnish: handbook display procedures need buh_* display, buym yelp,
# and zipper constants (BUWZ_*, RBZ_*) for tabtarget references.
# No regime, no OAuth, no IAM.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbhw_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "Configuration directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"  || buc_die "Failed to source rbbc_constants.sh"
  local -r z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"
  local -r z_jjk_kit_dir="${BURD_TOOLS_DIR}/jjk"

  source "${BURD_BUK_DIR}/buym_yelp.sh"                                || buc_die "Failed to source buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"                             || buc_die "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"                               || buc_die "Failed to source buz_zipper.sh"
  source "${BURD_BUK_DIR}/buwz_zipper.sh"                              || buc_die "Failed to source buwz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"                              || buc_die "Failed to source rbz_zipper.sh"
  source "${z_jjk_kit_dir}/jjz_zipper.sh"                              || buc_die "Failed to source jjz_zipper.sh"
  zbuz_kindle
  zbuwz_kindle
  zrbz_kindle
  zjjz_kindle
  source "${z_rbk_kit_dir}/rbh0/rbhwb_base.sh"                         || buc_die "Failed to source rbhwb_base.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhwht_handbook_top.sh"                || buc_die "Failed to source rbhwht_handbook_top.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhw0_top.sh"                          || buc_die "Failed to source rbhw0_top.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhwdd_docker_desktop.sh"              || buc_die "Failed to source rbhwdd_docker_desktop.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhwdn_docker_wsl_native.sh"           || buc_die "Failed to source rbhwdn_docker_wsl_native.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhwcd_docker_context_discipline.sh"   || buc_die "Failed to source rbhwcd_docker_context_discipline.sh"

  zrbhw_kindle
}

buc_execute rbhw_ "Windows Handbook" zrbhw_furnish "$@"

# eof
