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
# BURH CLI - Command line interface for BURH host regime operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

burh_validate() {
  buc_doc_brief "Validate BURH host regime profile via enrollment report"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"
  buc_step "Validating BURH host regime"
  buv_report BURH "Host Regime"
  buc_step "BURH host regime valid"
}

burh_render() {
  buc_doc_brief "Display diagnostic view of BURH host regime profile"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || buc_die "Profile alias required (e.g., winhost-cyg)"
  buv_render BURH "BURH - Bash Utility Host Regime"
}

burh_list() {
  buc_doc_brief "List available BURH host profiles for current user"
  buc_doc_shown || return 0

  local z_aliases
  z_aliases=$(burh_list_capture) || buc_die "No BURH profiles found for user: ${BURS_USER}"
  buc_step "Available profiles for ${BURS_USER}:"
  local z_alias=""
  for z_alias in ${z_aliases}; do
    buc_bare "        ${z_alias}"
  done
}

######################################################################
# Furnish and Main

zburh_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "BUK config directory (dispatch-provided)"
  buc_doc_env "BUZ_FOLIO             " "Profile alias (e.g., winhost-cyg); empty for list"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/burc_regime.sh"
  source "${BURD_BUK_DIR}/burs_regime.sh"
  source "${BURD_BUK_DIR}/burh_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce

  source "${BURD_REGIME_FILE}" || buc_die "Failed to source BURC: ${BURD_REGIME_FILE}"

  zburc_kindle
  zburc_enforce

  source "${BURD_STATION_FILE}" || buc_die "Failed to source BURS: ${BURD_STATION_FILE}"

  zburs_kindle
  zburs_enforce

  zbupr_kindle

  # If BUZ_FOLIO is set, load and kindle the specified profile
  if test -n "${BUZ_FOLIO:-}"; then
    local -r z_profile_file="${BURD_CONFIG_DIR}/users/${BURS_USER}/${BUZ_FOLIO}/burh.env"
    test -f "${z_profile_file}" || buc_die "BURH profile not found: ${z_profile_file}"
    source "${z_profile_file}" || buc_die "Failed to source BURH: ${z_profile_file}"
    zburh_kindle
    zburh_enforce
  fi
}

buc_execute burh_ "Bash Utility Host Regime" zburh_furnish "$@"

# eof
