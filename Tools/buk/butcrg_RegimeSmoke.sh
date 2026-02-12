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
  buto_section "Dispatching: ${z_colophon}"
  bute_dispatch "${z_colophon}"
  local z_step
  z_step=$(bute_last_step_capture) || buto_fatal "No step recorded for ${z_colophon}"
  local z_status
  z_status=$(bute_get_step_exit_capture "${z_step}")
  buto_fatal_on_error "${z_status}" "Tabtarget failed" "Colophon: ${z_colophon}"
  buto_info "OK: ${z_colophon} (exit ${z_status})"
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
  buto_success "RBRN regime render+validate passed"
}

butcrg_rbrr() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rrr"
  zbutcrg_dispatch_ok "rbw-rrv"
  buto_success "RBRR regime render+validate passed"
}

butcrg_rbrv() {
  zbutcrg_init
  zbutcrg_dispatch_ok "rbw-rvr"
  zbutcrg_dispatch_ok "rbw-rvv"
  buto_success "RBRV regime render+validate passed"
}

# eof
