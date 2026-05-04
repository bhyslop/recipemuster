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

# bujb_knock - probe workload reachability (workload SSH + remote no-op).
bujb_knock() {
  buc_doc_brief "Probe workload SSH reachability for an investiture"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || burp_die_no_folio

  bujb_resolve_investiture

  buc_step "Knocking ${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST} (${BUJB_RESOLVED_VICEROYALTY})"

  ssh -i "${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"     \
      -o IdentitiesOnly=yes                       \
      -o BatchMode=yes                            \
      -o StrictHostKeyChecking=accept-new         \
      -o ConnectTimeout=10                        \
      "${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST}" \
      true                                        \
    || buc_die "Knock failed for ${BUJB_RESOLVED_VICEROYALTY}"

  buc_step "Knock succeeded"
}

# bujb_command_file - stream a local command file's contents to the workload
# remote shell. Captures stdout/stderr/exit to BURD_OUTPUT_DIR. Per spec:
# returns 0 when dispatch + capture complete (independent of remote exit);
# non-zero on SSH-level failure (exit 255).
bujb_command_file() {
  buc_doc_brief "Execute a local command file as workload; capture stdout/stderr/exit"
  buc_doc_param "command_file" "Local file streamed as the remote command body"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || burp_die_no_folio

  local z_command_file="${1:-}"
  test -n "${z_command_file}"   || buc_usage_die
  test -f "${z_command_file}"   || buc_die "Command file not found: ${z_command_file}"
  test -r "${z_command_file}"   || buc_die "Command file not readable: ${z_command_file}"

  bujb_resolve_investiture

  buc_step "Streaming ${z_command_file} to ${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST}"
  buc_step "Output dir: ${BURD_OUTPUT_DIR}"

  local z_exit=0
  ssh -i "${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"     \
      -o IdentitiesOnly=yes                       \
      -o BatchMode=yes                            \
      -o StrictHostKeyChecking=accept-new         \
      -o ConnectTimeout=10                        \
      "${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST}" \
      'bash -s'                                   \
      < "${z_command_file}"                       \
      > "${BURD_OUTPUT_DIR}/stdout.log"           \
      2> "${BURD_OUTPUT_DIR}/stderr.log"          \
    || z_exit=$?

  echo "${z_exit}" > "${BURD_OUTPUT_DIR}/exitcode"

  test "${z_exit}" -ne 255 \
    || buc_die "SSH connection or authentication failed (exit 255). See ${BURD_OUTPUT_DIR}/stderr.log"

  buc_step "Remote exit code: ${z_exit}"
  buc_step "Capture complete"
}

# bujb_interactive_session - hand control to ssh as workload.
bujb_interactive_session() {
  buc_doc_brief "Interactive SSH session as workload"
  buc_doc_shown || return 0

  test -n "${BUZ_FOLIO:-}" || burp_die_no_folio

  bujb_resolve_investiture

  buc_step "Opening interactive session: ${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST} (${BUJB_RESOLVED_VICEROYALTY})"

  exec ssh -t                                     \
      -i "${BUJB_RESOLVED_WORKLOAD_KEY_FILE}"     \
      -o IdentitiesOnly=yes                       \
      -o StrictHostKeyChecking=accept-new         \
      "${BUJB_RESOLVED_WORKLOAD_USER}@${BUJB_RESOLVED_HOST}" \
      "bash -i"
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
