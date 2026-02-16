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

butcrg_burc() {
  zbutcrg_init
  zbutcrg_dispatch_ok "buw-rcr"
  zbutcrg_dispatch_ok "buw-rcv"
  buto_success "BURC regime render+validate passed"
}

butcrg_burs() {
  zbutcrg_init
  zbutcrg_dispatch_ok "buw-rsr"
  zbutcrg_dispatch_ok "buw-rsv"
  buto_success "BURS regime render+validate passed"
}

######################################################################
# RBW regime cases

butcrg_rbrn() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rnr"
  zbutcrg_dispatch_ok "rbw-rnv"

  # List operation: verify rbrn_list returns non-empty, validate each moniker
  buto_section "RBRN list operation"
  local z_monikers
  z_monikers=$(rbrn_list) || buto_fatal "rbrn_list failed"
  test -n "${z_monikers}" || buto_fatal "rbrn_list returned empty"
  local z_moniker
  while read -r z_moniker; do
    buto_info "Validating moniker: ${z_moniker}"
    zbutcrg_dispatch_ok "rbw-rnv" "${z_moniker}"
  done <<< "${z_monikers}"

  buto_success "RBRN regime render+validate+list passed"
}

butcrg_rbrr() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rrr"
  zbutcrg_dispatch_ok "rbw-rrv"
  buto_success "RBRR regime render+validate passed"
}

butcrg_rbrv() {
  zbutcrg_init

  # Load RBRR (needed for RBRR_VESSEL_DIR used by rbrv_list)
  rbrr_load

  zbutcrg_dispatch_ok "rbw-rvr"
  zbutcrg_dispatch_ok "rbw-rvv"

  # List operation: verify rbrv_list returns non-empty, validate each sigil
  buto_section "RBRV list operation"
  local z_sigils
  z_sigils=$(rbrv_list) || buto_fatal "rbrv_list failed"
  test -n "${z_sigils}" || buto_fatal "rbrv_list returned empty"
  local z_sigil
  while read -r z_sigil; do
    buto_info "Validating sigil: ${z_sigil}"
    zbutcrg_dispatch_ok "rbw-rvv" "${z_sigil}"
  done <<< "${z_sigils}"

  buto_success "RBRV regime render+validate+list passed"
}

######################################################################
# Payor regime case

butcrg_rbrp() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rpr"
  zbutcrg_dispatch_ok "rbw-rpv"
  buto_success "RBRP regime render+validate passed"
}

######################################################################
# BURD dispatch regime case

butcrg_burd() {
  zbutcrg_init
  buto_section "Verifying BURD dispatch environment"
  test -n "${BURD_TEMP_DIR:-}"    || buto_fatal "BURD_TEMP_DIR not set"
  test -n "${BURD_NOW_STAMP:-}"   || buto_fatal "BURD_NOW_STAMP not set"
  test -n "${BURD_OUTPUT_DIR:-}"  || buto_fatal "BURD_OUTPUT_DIR not set"
  test -n "${BURD_GIT_CONTEXT:-}" || buto_fatal "BURD_GIT_CONTEXT not set"
  test -d "${BURD_TEMP_DIR}"      || buto_fatal "BURD_TEMP_DIR does not exist: ${BURD_TEMP_DIR}"
  buto_success "BURD dispatch environment verified"
}

# eof
