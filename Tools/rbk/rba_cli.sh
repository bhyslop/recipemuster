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
# Surfaces the sitting-lifecycle operator verb (novate) as a tabtarget — the
# one operator-invoked surface on the sitting lifecycle, where avow itself
# never is (RBS0 rbtf_novate). Thin arm over the rba library: the furnish
# carries only the avowal-path stack (trust + manor pool + OAuth transport),
# none of the depot/don machinery the probe CLI (rbgv_cli.sh) pulls.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# CLI Commands

# Novate the sitting — force-fresh renewal: bypass the sitting-reuse branch
# and atomically overwrite any standing sitting with a freshly-opened,
# full-window one. The remedy the avow runway gate names when it turns a
# short sitting away. Mechanism-gated exactly as avowal is: device-flow
# interactive or RFC 7523 programmatic per the trust's RBRF_MECHANISM.
# Depot-agnostic like the avowal probe: needs only the RBRF trust + manor pool.
rba_novate_sitting() {
  zrba_sentinel
  buc_doc_brief "Novate the sitting — open a fresh full-window sitting, extinguishing any standing one (the runway gate's named remedy)"
  buc_doc_shown || return 0

  buc_step "Novation — force-fresh sitting against the RBRF trust"
  rbcc_source_active_rbrf
  source "${RBCC_rbrw_file}" || buc_die "Failed to source RBRW: ${RBCC_rbrw_file}"
  zrbrf_kindle
  zrbrw_kindle
  zrbrf_enforce
  zrbrw_enforce

  rba_novate

  local z_token
  z_token=$(zrba_sitting_read_capture) || buc_die "Sitting not readable after novation"
  test -n "${z_token}" || buc_die "Sitting holds an empty federated token"
  buc_success "Sitting novated — fresh full-window federated token obtained (${#z_token} chars)"
}

######################################################################
# Furnish and Main

zrba_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Bash Dispatch Utility provided temporary directory, empty at start of command"
  buc_doc_env_done || return 0

  local z_rbk="${BASH_SOURCE[0]%/*}"
  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
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
  source "${RBCC_rbrr_file}" || buc_die "Failed to source ${RBCC_rbrr_file}"
  zrbrr_kindle
  zrbcc_kindle
  zrbgc_kindle
  zrbgo_kindle
  zrba_kindle
}

buc_execute rba_ "Recipe Bottle Auth" zrba_furnish "$@"

# eof
