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
# RBRA CLI - Command line interface for RBRA credential operations
#
# Manifold regime: BUZ_FOLIO carries the minted role sprue (rbnae_governor,
# rbnae_retriever, rbnae_director). Role resolves to RBRA file path via RBRR
# references — the resolve case is the translation point where the minted folio
# maps to a bare secret-directory path.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# Internal Helpers

# Resolve role name to RBRA file path via RBRR
zrbra_resolve_role() {
  local z_role="${1:-}"
  test -n "${z_role}" || buc_die "RBRA role required (rbnae_governor|rbnae_retriever|rbnae_director)"

  zrbrr_sentinel

  # Minted folio → bare secret-directory path: the case arms match the rbnae_
  # enum sprue while the RBDC targets resolve to bare-named directories.
  case "${z_role}" in
    rbnae_governor)  echo "${RBDC_GOVERNOR_RBRA_FILE}" ;;
    rbnae_retriever) echo "${RBDC_RETRIEVER_RBRA_FILE}" ;;
    rbnae_director)  echo "${RBDC_DIRECTOR_RBRA_FILE}" ;;
    *)               buc_die "Unknown RBRA role: ${z_role}. Valid roles: rbnae_governor, rbnae_retriever, rbnae_director" ;;
  esac
}

######################################################################
# Command Functions

rbra_validate() {
  buc_doc_brief "Validate RBRA credential regime configuration via enrollment report"
  buc_doc_shown || return 0

  if test -z "${BUZ_FOLIO:-}"; then
    rbra_list
    buc_die "RBRA role required"
  fi
  buc_step "Validating RBRA credential regime"
  buv_report RBRA "Credential Regime"
  buc_step "RBRA credential valid"
}

rbra_render() {
  buc_doc_brief "Display diagnostic view of RBRA credential regime configuration"
  buc_doc_shown || return 0

  if test -z "${BUZ_FOLIO:-}"; then
    rbra_list
    buc_die "RBRA role required"
  fi
  local z_rbra_file
  z_rbra_file=$(zrbra_resolve_role "${BUZ_FOLIO}")
  buv_render RBRA "RBRA - Recipe Bottle Authentication Regime" "${z_rbra_file}"
}

rbra_list() {
  buc_doc_brief "List RBRA credential roles and file paths"
  buc_doc_shown || return 0

  zrbrr_sentinel

  buc_step "RBRA credential roles (from RBRR):"
  local z_roles=("rbnae_governor" "rbnae_retriever" "rbnae_director")
  local z_vars=("RBDC_GOVERNOR_RBRA_FILE" "RBDC_RETRIEVER_RBRA_FILE" "RBDC_DIRECTOR_RBRA_FILE")

  local z_i
  for z_i in "${!z_roles[@]}"; do
    local z_role="${z_roles[$z_i]}"
    local z_path="${!z_vars[$z_i]:-}"
    local z_status="missing"
    test -f "${z_path}" && z_status="ok"
    buc_step "  ${z_role} [${z_status}] ${z_path}"
  done
}

######################################################################
# Furnish and Main

zrbra_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local z_command="${1:-}"

  # Light sources (always)
  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbgc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrd_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"
  source "${z_rbk_kit_dir}/rbra_regime.sh"

  # Light kindles (always)
  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zbupr_kindle
  zrbcc_kindle

  # Load RBRR (needed for role resolution and list)
  source "${RBCC_rbrr_file}"
  source "${RBCC_rbrd_file}"
  zrbrr_kindle
  zrbrd_kindle
  zrbrr_enforce
  zrbrd_enforce
  zrbdc_kindle

  # If BUZ_FOLIO is set, load and kindle the specified role
  if test -n "${BUZ_FOLIO:-}"; then
    local z_rbra_file
    z_rbra_file=$(zrbra_resolve_role "${BUZ_FOLIO}")
    test -f "${z_rbra_file}" || buc_die "RBRA file not found for role ${BUZ_FOLIO}: ${z_rbra_file}"
    source "${z_rbra_file}" || buc_die "Failed to source RBRA: ${z_rbra_file}"
    zrbra_kindle
    zrbra_enforce

    # Swizzle guard: RBRA_ROLE must match the expected role from folio
    test "${RBRA_ROLE}" = "${BUZ_FOLIO}" \
      || buc_die "RBRA_ROLE swizzle: file declares role '${RBRA_ROLE}' but loaded as '${BUZ_FOLIO}'"
  fi
}

buc_execute rbra_ "Recipe Bottle Authentication Regime" zrbra_furnish "$@"

# eof
