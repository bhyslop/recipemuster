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
# RBA CLI - Recipe Bottle Auth command-line interface
#
# Surfaces the sederunt-lifecycle operator verbs as tabtargets — where avow
# itself never is: novate, the one mutating surface, and
# espy, the read-only probe. Thin arm over the rba library:
# the furnish carries only the avowal-path stack (trust + manor pool + OAuth
# transport), none of the depot/don machinery the probe CLI (rbgv_cli.sh) pulls.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# CLI Commands

# Novate the sederunt — force-fresh renewal: bypass the sederunt-reuse branch
# and atomically overwrite any standing sederunt with a freshly-opened,
# full-window one. The remedy the avow runway gate names when it turns a
# short sederunt away. Mechanism-gated exactly as avowal is: device-flow
# interactive or RFC 7523 programmatic per the trust's RBRF_MECHANISM.
# Depot-agnostic like the avowal probe: needs only the RBRF trust + manor pool.
rba_novate_sederunt() {
  zrba_sentinel
  buc_doc_brief "Novate the ${RBCC_noun_sederunt} — open a fresh full-window ${RBCC_noun_sederunt}, extinguishing any standing one (the runway gate's named remedy)"
  buc_doc_shown || return 0

  buc_step "Novation — force-fresh ${RBCC_noun_sederunt} against the RBRF trust"
  rbcc_source_active_rbrf
  source "${RBCC_rbrw_file}" || buc_die_now "Failed to source RBRW: ${RBCC_rbrw_file}"
  zrbrf_kindle
  zrbrw_kindle
  zrbrf_enforce
  zrbrw_enforce

  rba_novate

  local z_token
  z_token=$(zrba_sederunt_read_capture) || buc_die_now "${RBCC_noun_sederunt^} not readable after novation"
  test -n "${z_token}" || buc_die_now "${RBCC_noun_sederunt^} holds an empty federated token"
  buc_success "${RBCC_noun_sederunt^} novated — fresh full-window federated token obtained (${#z_token} chars)"
}

# Espy the sederunt — the read-only probe: report whether a
# sederunt is live and how much runway remains, from the cache alone — never
# opening one, never prompting, no network. An absent or lapsed sederunt is a
# reported verdict, exit 0 (the descry precedent); only a broken read
# dies. Liveness and sufficiency judgments belong to the callers: the verdict
# rides a fact file keyed by the active foedus, the branch point for the
# theurge gate arc's fail-fast before its may-prompt baseline avow.
rba_espy_sederunt() {
  zrba_sentinel
  buc_doc_brief "Espy the ${RBCC_noun_sederunt} — report liveness and remaining runway from the cache alone (read-only: never opens, never prompts, no network)"
  buc_doc_shown || return 0

  buc_step "Espy — ${RBCC_noun_sederunt} state against the RBRF trust"
  rbcc_source_active_rbrf
  source "${RBCC_rbrw_file}" || buc_die_now "Failed to source RBRW: ${RBCC_rbrw_file}"
  zrbrf_kindle
  zrbrw_kindle
  zrbrf_enforce
  zrbrw_enforce

  local z_path
  z_path=$(zrba_sederunt_path_capture) || buc_die_now "Failed to resolve the ${RBCC_noun_sederunt} cache path"

  # Verdict: absent (no cache), else live/lapsed by the skew-gated predicate.
  # Runway is reported raw (a lapsed sederunt inside the skew window may still
  # show a few seconds) — the probe reports, it never judges sufficiency.
  local z_verdict=""
  local z_runway=""
  if test ! -f "${z_path}"; then
    z_verdict="absent"
  else
    z_runway=$(zrba_sederunt_runway_capture) || buc_die_now "${RBCC_noun_sederunt^} cache present but unreadable: ${z_path}"
    if zrba_sederunt_live_predicate; then
      z_verdict="live"
    else
      z_verdict="lapsed"
    fi
  fi

  local z_value="verdict=${z_verdict}"
  test -z "${z_runway}" || z_value="${z_value}
runway=${z_runway}"
  buf_write_fact_multi "${RBRR_ACTIVE_FOEDUS}" "${RBCC_fact_ext_sederunt}" "${z_value}" \
    || buc_die_now "Failed to write the ${RBCC_noun_sederunt} fact"

  if test "${z_verdict}" = "live"; then
    buc_success "${RBCC_noun_sederunt^} LIVE — runway ${z_runway}s (~$(( z_runway / 3600 ))h$(( (z_runway % 3600) / 60 ))m remaining)"
  else
    buc_warn "No live ${RBCC_noun_sederunt} — verdict '${z_verdict}'; open one with any federated command or rbw-aN (fresh full window)"
  fi
}

######################################################################
# Furnish and Main

zrba_furnish() {
  buc_doc_env_row "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env_row "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env_done || return 0

  local z_rbk="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh"
  source "${z_rbk}/rbrr_regime.sh"
  source "${z_rbk}/rbrf_regime.sh"
  source "${z_rbk}/rbrw_regime.sh"
  source "${z_rbk}/rbcc_constants.sh"
  source "${z_rbk}/rbgc_constants.sh"
  source "${z_rbk}/rbgo_oauth.sh"
  source "${z_rbk}/rba_auth.sh"

  zbuv_kindle
  zburd_kindle

  # RBRR is sourced for the RBRR_ACTIVE_FOEDUS selector alone (the trust
  # resolve in rbcc_source_active_rbrf); depot-agnostic, so no RBRR
  # enforcement — mirroring the avowal probe's furnish posture.
  source "${RBCC_rbrr_file}" || buc_die_now "Failed to source ${RBCC_rbrr_file}"
  zrbrr_kindle
  zrbcc_kindle
  zrbgc_kindle
  zrbgo_kindle
  zrba_kindle
}

buc_execute rba_ "Recipe Bottle Auth" zrba_furnish "$@"

# eof
