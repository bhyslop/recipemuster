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
# BUTCRG - Regime smoke test cases
#
# Dispatches all regime render and validate tabtargets via bute_dispatch
# and verifies each exits cleanly.

set -euo pipefail

######################################################################
# Private helper: init dispatch and evidence scoped to this test case

zbutcrg_init() {
  bute_init_dispatch
  BURD_TEMP_DIR="${BUT_TEMP_DIR}"
  bute_init_evidence
}

zbutcrg_dispatch_ok() {
  local z_colophon="${1}"
  shift || true
  local z_label="${z_colophon}${*:+ $*}"
  buto_section "Dispatching: ${z_label}"
  bute_dispatch "${z_colophon}" "$@"
  local z_step
  z_step=$(bute_last_step_capture) || buto_fatal "No step recorded for ${z_label}"
  local z_status
  z_status=$(bute_get_step_exit_capture "${z_step}")
  buto_fatal_on_error "${z_status}" "Tabtarget failed" "Colophon: ${z_label}"
  buto_info "OK: ${z_label} (exit ${z_status})"
}

######################################################################
# BUK regime cases

butcrg_burc_tcase() {
  zbutcrg_init
  zbutcrg_dispatch_ok "buw-rcr"
  zbutcrg_dispatch_ok "buw-rcv"
  buto_success "BURC regime render+validate passed"
}

butcrg_burs_tcase() {
  zbutcrg_init
  zbutcrg_dispatch_ok "buw-rsr"
  zbutcrg_dispatch_ok "buw-rsv"
  buto_success "BURS regime render+validate passed"
}

######################################################################
# RBW regime cases

butcrg_rbrn_tcase() {
  zbutcrg_init

  local z_monikers
  z_monikers=$(rbrn_list_capture) || buto_fatal "No nameplates found"

  buto_section "RBRN nameplate iteration"
  local z_moniker=""
  for z_moniker in ${z_monikers}; do
    buto_info "Render+validate nameplate: ${z_moniker}"
    zbutcrg_dispatch_ok "rbw-rnr" "${z_moniker}"
    zbutcrg_dispatch_ok "rbw-rnv" "${z_moniker}"
  done

  buto_success "RBRN regime render+validate passed"
}

butcrg_rbrr_tcase() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rrr"
  zbutcrg_dispatch_ok "rbw-rrv"
  buto_success "RBRR regime render+validate passed"
}

butcrg_rbrv_tcase() {
  zbutcrg_init

  # Load RBRR for rbrv_list_capture (needs RBRR_VESSEL_DIR)
  source "${RBCC_rbrr_file}" || buc_die "Failed to source ${RBCC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce
  zrbrr_lock

  local z_sigils
  z_sigils=$(rbrv_list_capture) || buto_fatal "No vessels found"

  buto_section "RBRV vessel iteration"
  local z_sigil=""
  for z_sigil in ${z_sigils}; do
    buto_info "Render+validate vessel: ${z_sigil}"
    zbutcrg_dispatch_ok "rbw-rvr" "${z_sigil}"
    zbutcrg_dispatch_ok "rbw-rvv" "${z_sigil}"
  done

  buto_success "RBRV regime render+validate passed"
}

######################################################################
# Payor regime case

butcrg_rbrp_tcase() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rpr"
  zbutcrg_dispatch_ok "rbw-rpv"
  buto_success "RBRP regime render+validate passed"
}

######################################################################
# BURD dispatch regime case

butcrg_burd_tcase() {
  zbutcrg_init
  buto_section "Verifying BURD dispatch environment"
  zburd_sentinel
  zburd_enforce
  zburd_lock
  test -d "${BURD_TEMP_DIR}"      || buto_fatal "BURD_TEMP_DIR does not exist: ${BURD_TEMP_DIR}"
  buto_success "BURD dispatch environment verified"
}

# eof
