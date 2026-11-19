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
# RBFH CLI - Recipe Bottle Foundry Hygiene command-line interface
#
# Surfaces the Dockerfile FROM-line constraint enforced by the rbfh
# library module so that theurge fixtures (and operators) can drive
# the contract from outside the kludge/conjure code paths.
#
# Commands:
#   check         Check a Dockerfile path for FROM-line hygiene
#   check_vessel  Check a vessel's RBRV_CONJURE_DOCKERFILE for hygiene

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh" || exit 1
source "${BURD_BUK_DIR}/buym_yelp.sh" || exit 1

######################################################################
# CLI Commands

rbfh_check() {
  zrbfh_sentinel

  buc_doc_brief "Check a Dockerfile against the FROM-line hygiene contract"
  buc_doc_param "dockerfile" "Path to the Dockerfile to check"
  buc_doc_shown || return 0

  local -r z_path="${BUZ_FOLIO:-}"
  test -n "${z_path}" || buc_die_now "Dockerfile path required (free-form path argument)"

  buc_step "RBFH Check: ${z_path}"
  rbfh_dockerfile_check "${z_path}"
  buc_success "Dockerfile passes hygiene contract: ${z_path}"
}

rbfh_check_vessel() {
  zrbfh_sentinel
  zrbfc0_sentinel

  buc_doc_brief "Check a vessel's conjure Dockerfile against the FROM-line hygiene contract"
  buc_doc_param "vessel" "Vessel sigil or path to vessel directory"
  buc_doc_shown || return 0

  # Resolve vessel argument (sigil or path) — lists-and-dies on missing/invalid
  zrbfcv_resolve_vessel "${BUZ_FOLIO:-}"
  local z_vessel_dir
  z_vessel_dir=$(<"${ZRBFC0_VESSEL_RESOLVED_DIR_FILE}") || buc_die_now "Failed to read: ${ZRBFC0_VESSEL_RESOLVED_DIR_FILE}"
  test -n "${z_vessel_dir}" || buc_die_now "Empty resolved vessel path"

  # The vessel's rbrv.env supplies RBRV_VESSEL_MODE and RBRV_CONJURE_DOCKERFILE,
  # contained so the vessel's fields do not outlive the check.
  local -r z_rbrv_file="${z_vessel_dir}/${RBCC_rbrv_file}"
  (
    source "${z_rbrv_file}" || buc_die_now "Failed to source vessel rbrv.env: ${z_rbrv_file}"

    # Hygiene is a property of the FROM line; non-conjure vessels have no
    # local Dockerfile, so the contract is vacuously satisfied. Exit silently
    # so callers iterating the whole fleet need not pre-filter by mode.
    if test "${RBRV_VESSEL_MODE:-}" != "rbnve_conjure"; then
      buc_info "Vessel '${RBRV_SIGIL:-${z_vessel_dir}}' is mode '${RBRV_VESSEL_MODE:-<unset>}' — no Dockerfile to check"
    else
      test -n "${RBRV_CONJURE_DOCKERFILE:-}" \
        || buc_die_now "Vessel '${RBRV_SIGIL:-${z_vessel_dir}}' has no RBRV_CONJURE_DOCKERFILE"

      buc_step "RBFH Check Vessel: ${RBRV_SIGIL:-${z_vessel_dir}} → ${RBRV_CONJURE_DOCKERFILE}"
      rbfh_dockerfile_check "${RBRV_CONJURE_DOCKERFILE}"
      buc_success "Vessel Dockerfile passes hygiene contract: ${RBRV_CONJURE_DOCKERFILE}"
    fi
  ) || buc_die_now "Vessel hygiene check failed: ${z_rbrv_file}"
}

######################################################################
# Furnish and Main

zrbfh_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env_row "BUZ_FOLIO             " "Dockerfile path (rbfh_check) or vessel sigil/path (rbfh_check_vessel)"
  buc_doc_env_done || return 0

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buf_fact.sh" || buc_die_now "Failed to source buf_fact.sh"
  source "${z_rbk_kit_dir}/rbcc_constants.sh" || buc_die_now "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rbgc_constants.sh" || buc_die_now "Failed to source rbgc_constants.sh"
  source "${z_rbk_kit_dir}/rbgl_layout.sh" || buc_die_now "Failed to source rbgl_layout.sh"
  source "${z_rbk_kit_dir}/rbgd_depot.sh" || buc_die_now "Failed to source rbgd_depot.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh" || buc_die_now "Failed to source rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh" || buc_die_now "Failed to source rbrd_regime.sh"
  source "${z_rbk_kit_dir}/rbrv_regime.sh" || buc_die_now "Failed to source rbrv_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_derived.sh" || buc_die_now "Failed to source rbdc_derived.sh"
  source "${RBCC_rbrr_file}" || buc_die_now "Failed to source ${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}" || buc_die_now "Failed to source ${RBCC_rbrd_file}"
  source "${z_rbk_kit_dir}/rbgo_oauth.sh" || buc_die_now "Failed to source rbgo_oauth.sh"
  source "${z_rbk_kit_dir}/rbfc0_core.sh" || buc_die_now "Failed to source rbfc0_core.sh"
  source "${z_rbk_kit_dir}/rbfh_hygiene.sh" || buc_die_now "Failed to source rbfh_hygiene.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh" || buc_die_now "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh" || buc_die_now "Failed to source rbz_zipper.sh"

  zrbcc_kindle

  zrbrr_kindle
  zrbrd_kindle
  zrbrr_enforce
  zrbrd_enforce
  zrbdc_kindle

  zrbgc_kindle
  zrbgl_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbfc0_kindle
  zrbfh_kindle

  zbuz_kindle
  zrbz_kindle
}

buc_execute rbfh_ "Recipe Bottle Foundry Hygiene" zrbfh_furnish "$@"

# eof
