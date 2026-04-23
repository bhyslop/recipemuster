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
# Recipe Bottle Handbook Onboarding - Command Line Interface
#
# Thin furnish: onboarding walkthroughs need only display infrastructure
# (buh handbook, buz/rbz zippers, rbcc/rbgc constants) — no regime, no
# OAuth, no full GCP stack. All probes work on the filesystem directly.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zrbho_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "Configuration directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for intermediate files"
  buc_doc_env "BURD_OUTPUT_DIR       " "Directory for command outputs"
  buc_doc_env_done || return 0

  # Light sources (always)
  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"  || buc_die "Failed to source rbbc_constants.sh"
  local -r z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  source "${BURD_BUK_DIR}/buh_handbook.sh"           || buc_die "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buym_yelp.sh"             || buc_die "Failed to source buym_yelp.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh"         || buc_die "Failed to source buv_validation.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"             || buc_die "Failed to source buz_zipper.sh"
  source "${BURD_BUK_DIR}/buwz_zipper.sh"            || buc_die "Failed to source buwz_zipper.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"        || buc_die "Failed to source rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"        || buc_die "Failed to source rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"           || buc_die "Failed to source rbrr_regime.sh"
  source "${RBBC_rbrr_file}"                         || buc_die "Failed to source ${RBBC_rbrr_file}"
  source "${z_rbk_kit_dir}/rbyc_common.sh"            || buc_die "Failed to source rbyc_common.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"            || buc_die "Failed to source rbz_zipper.sh"
  zbuv_kindle
  zrbgc_kindle
  zbuz_kindle
  zbuwz_kindle
  zrbz_kindle
  # RBRR kindle only — thin-deps concession: enforce would fail on fresh installs
  # (filesystem gates for vessel/secrets dirs), blocking onboarding entry.
  zrbrr_kindle
  zrbyc_kindle
  source "${z_rbk_kit_dir}/rbh0/rbhob_base.sh"               || buc_die "Failed to source rbhob_base.sh"
  source "${z_rbk_kit_dir}/rbh0/rbho0_start_here.sh"         || buc_die "Failed to source rbho0_start_here.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhocc_crash_course.sh"      || buc_die "Failed to source rbhocc_crash_course.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhocr_credential_retriever.sh" || buc_die "Failed to source rbhocr_credential_retriever.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhocd_credential_director.sh"  || buc_die "Failed to source rbhocd_credential_director.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhoct_crucible_trunk.sh"    || buc_die "Failed to source rbhoct_crucible_trunk.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhocq_crucible_quench.sh"   || buc_die "Failed to source rbhocq_crucible_quench.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhofc_first_crucible.sh"    || buc_die "Failed to source rbhofc_first_crucible.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhots_tadmor_security.sh"   || buc_die "Failed to source rbhots_tadmor_security.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhodf_director_first_build.sh" || buc_die "Failed to source rbhodf_director_first_build.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhopw_payor_wrapper.sh"     || buc_die "Failed to source rbhopw_payor_wrapper.sh"
  source "${z_rbk_kit_dir}/rbh0/rbhogw_governor_wrapper.sh"  || buc_die "Failed to source rbhogw_governor_wrapper.sh"
  zrbho_kindle
}

buc_execute rbho_ "Onboarding Guides" zrbho_furnish "$@"

# eof
