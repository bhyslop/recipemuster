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
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  source "${BURD_BUK_DIR}/buh_handbook.sh"           || buc_die "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buv_validation.sh"         || buc_die "Failed to source buv_validation.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"             || buc_die "Failed to source buz_zipper.sh"
  source "${BURD_BUK_DIR}/buwz_zipper.sh"            || buc_die "Failed to source buwz_zipper.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"        || buc_die "Failed to source rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"        || buc_die "Failed to source rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"           || buc_die "Failed to source rbrr_regime.sh"
  source "${RBBC_rbrr_file}"                         || buc_die "Failed to source ${RBBC_rbrr_file}"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"            || buc_die "Failed to source rbz_zipper.sh"
  zbuv_kindle
  zrbgc_kindle
  zbuz_kindle
  zbuwz_kindle
  zrbz_kindle
  # RBRR kindle only — thin-deps concession: enforce would fail on fresh installs
  # (filesystem gates for vessel/secrets dirs), blocking onboarding entry.
  zrbrr_kindle
  source "${z_rbk_kit_dir}/rbho_onboarding.sh"       || buc_die "Failed to source rbho_onboarding.sh"
  zrbho_kindle
}

buc_execute rbho_ "Onboarding Guides" zrbho_furnish "$@"

# eof
