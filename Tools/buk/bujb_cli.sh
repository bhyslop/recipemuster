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
# BUJB CLI - Command line interface for the jurisdiction module
#
# Furnish loads BURC, BURS, BURP (per investiture folio), and the cross-
# referenced BURN profile, then kindles bujb. Command functions call
# bujb_resolve_investiture before doing any work; per-verb platform
# invariants are asserted inline at the command body.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Command Functions

# bujb_resolve - diagnostic: load investiture and print resolved fields.
# No remote action; verifies regime + key-file health for a target.
bujb_resolve() {
  buc_doc_brief "Resolve an investiture and display the BUJB_RESOLVED_* state"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || burp_die_no_folio

  bujb_resolve_investiture

  buc_step "Resolved investiture ${BUZ_FOLIO}:"
  buc_bare "  BUJB_RESOLVED_VICEROYALTY        = ${BUJB_RESOLVED_VICEROYALTY}"
  buc_bare "  BUJB_RESOLVED_HOST               = ${BUJB_RESOLVED_HOST}"
  buc_bare "  BUJB_RESOLVED_PLATFORM           = ${BUJB_RESOLVED_PLATFORM}"
  buc_bare "  BUJB_RESOLVED_PRIVILEGED_USER    = ${BUJB_RESOLVED_PRIVILEGED_USER}"
  buc_bare "  BUJB_RESOLVED_PRIVILEGED_KEY_FILE= ${BUJB_RESOLVED_PRIVILEGED_KEY_FILE}"
  buc_bare "  BUJB_RESOLVED_WORKLOAD_KEY_FILE  = ${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"
  buc_bare "  BUJB_RESOLVED_WORKLOAD_USER      = ${BUJB_RESOLVED_WORKLOAD_USER}"
}

######################################################################
# Furnish and Main

zbujb_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "BUK config directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/burc_regime.sh"
  source "${BURD_BUK_DIR}/burs_regime.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh"
  source "${BURD_BUK_DIR}/bubc_constants.sh"
  source "${BURD_BUK_DIR}/burn_regime.sh"
  source "${BURD_BUK_DIR}/burp_regime.sh"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"
  source "${BURD_BUK_DIR}/bujb_jurisdiction.sh"

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
  zbujb_kindle

  test -n "${BUZ_FOLIO:-}" || burp_die_no_folio

  local -r z_burp_file="${BURD_CONFIG_DIR}/${BUBC_rbmu_users_subdir}/${BURS_USER}/${BUZ_FOLIO}/burp.env"
  test -f "${z_burp_file}" || buc_die "BURP profile not found: ${z_burp_file}"
  source "${z_burp_file}"  || buc_die "Failed to source BURP: ${z_burp_file}"
  zburp_kindle
  zburp_enforce

  # Cross-reference: BURP_VICEROYALTY must resolve to a registered BURN profile.
  local -r z_burn_file="${BURD_CONFIG_DIR}/${BUBC_rbmn_nodes_subdir}/${BURP_VICEROYALTY}/burn.env"
  test -f "${z_burn_file}" || buc_die "BURN profile referenced by BURP_VICEROYALTY=${BURP_VICEROYALTY} not found: ${z_burn_file}"
  source "${z_burn_file}"  || buc_die "Failed to source BURN: ${z_burn_file}"
  zburn_kindle
  zburn_enforce
}

buc_execute bujb_ "Bash Utility Jurisdiction" zbujb_furnish "$@"

# eof
