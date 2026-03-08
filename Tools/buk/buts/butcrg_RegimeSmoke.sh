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
# Invokes all regime render and validate tabtargets via buto_tt_expect_ok
# and verifies each exits cleanly.

set -euo pipefail

######################################################################
# BUK regime cases

butcrg_burc_tcase() {
  buto_tt_expect_ok "buw-rcr"
  buto_tt_expect_ok "buw-rcv"
  buto_success "BURC regime render+validate passed"
}

butcrg_burs_tcase() {
  buto_tt_expect_ok "buw-rsr"
  buto_tt_expect_ok "buw-rsv"
  buto_success "BURS regime render+validate passed"
}

######################################################################
# RBW regime cases

butcrg_rbrn_tcase() {
  local z_monikers
  z_monikers=$(rbrn_list_capture) || buto_fatal "No nameplates found"

  buto_section "RBRN nameplate iteration"
  local z_moniker=""
  for z_moniker in ${z_monikers}; do
    buto_info "Render+validate nameplate: ${z_moniker}"
    buto_tt_expect_ok "rbw-rnr" "${z_moniker}"
    buto_tt_expect_ok "rbw-rnv" "${z_moniker}"
  done

  buto_success "RBRN regime render+validate passed"
}

butcrg_rbrr_tcase() {
  buto_tt_expect_ok "rbw-rrr"
  buto_tt_expect_ok "rbw-rrv"
  buto_success "RBRR regime render+validate passed"
}

butcrg_rbrv_tcase() {
  # Load RBRR for rbrv_list_capture (needs RBRR_VESSEL_DIR)
  source "${RBBC_rbrr_file}" || buc_die "Failed to source ${RBBC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle

  local z_sigils
  z_sigils=$(rbrv_list_capture) || buto_fatal "No vessels found"

  buto_section "RBRV vessel iteration"
  local z_sigil=""
  for z_sigil in ${z_sigils}; do
    buto_info "Render+validate vessel: ${z_sigil}"
    buto_tt_expect_ok "rbw-rvr" "${z_sigil}"
    buto_tt_expect_ok "rbw-rvv" "${z_sigil}"
  done

  buto_success "RBRV regime render+validate passed"
}

######################################################################
# Payor regime case

butcrg_rbrp_tcase() {
  buto_tt_expect_ok "rbw-rpr"
  buto_tt_expect_ok "rbw-rpv"
  buto_success "RBRP regime render+validate passed"
}

######################################################################
# BURD regime case

butcrg_burd_tcase() {
  buto_section "Verifying BURD dispatch environment"
  zburd_sentinel
  zburd_enforce
  test -d "${BURD_TEMP_DIR}"      || buto_fatal "BURD_TEMP_DIR does not exist: ${BURD_TEMP_DIR}"
  buto_success "BURD dispatch environment verified"
}

# eof
