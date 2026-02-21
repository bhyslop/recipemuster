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
# BUTCRG - Regime credential test cases
#
# Tests external regime files that require credential files on the
# developer workstation. These are NOT CI-safe — they require a fully
# configured workstation with credential files present.

set -euo pipefail

######################################################################
# Private helper: init dispatch and evidence scoped to this test case

zbutcrg_cred_init() {
  bute_init_dispatch
  BURD_TEMP_DIR="${BUT_TEMP_DIR}"
  bute_init_evidence
}

zbutcrg_cred_dispatch_ok() {
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
# RBRA credential cases (requires RBRR with RBRA file paths)

butcrg_rbra_tcase() {
  zbutcrg_cred_init

  source "${RBCC_rbrr_file}" || buc_die "Failed to source ${RBCC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce

  local z_roles=("governor" "retriever" "director")
  local z_vars=("RBRR_GOVERNOR_RBRA_FILE" "RBRR_RETRIEVER_RBRA_FILE" "RBRR_DIRECTOR_RBRA_FILE")

  local z_i
  for z_i in "${!z_roles[@]}"; do
    local z_role="${z_roles[$z_i]}"
    local z_var="${z_vars[$z_i]}"
    local z_path="${!z_var:-}"

    test -n "${z_path}" || buto_fatal "RBRR variable ${z_var} is empty"
    test -f "${z_path}" || buto_fatal "RBRA credential file not found for ${z_role}: ${z_path}. This suite requires a fully configured workstation."

    zbutcrg_cred_dispatch_ok "rbw-rav" "${z_role}"
    zbutcrg_cred_dispatch_ok "rbw-rar" "${z_role}"
  done

  buto_success "RBRA credential render+validate passed (all roles)"
}

######################################################################
# RBRO OAuth credential case

butcrg_rbro_tcase() {
  zbutcrg_cred_init

  local z_file="${HOME}/.rbw/rbro.env"
  test -f "${z_file}" || buto_fatal "RBRO credential file not found: ${z_file}. This suite requires a fully configured workstation."

  zbutcrg_cred_dispatch_ok "rbw-rov"
  zbutcrg_cred_dispatch_ok "rbw-ror"

  buto_success "RBRO OAuth regime render+validate passed"
}

######################################################################
# RBRS station credential case

butcrg_rbrs_tcase() {
  zbutcrg_cred_init

  local z_file="../station-files/rbrs.env"
  test -f "${z_file}" || buto_fatal "RBRS station file not found: ${z_file}. This suite requires a fully configured workstation."

  zbutcrg_cred_dispatch_ok "rbw-rsv"
  zbutcrg_cred_dispatch_ok "rbw-rsr"

  buto_success "RBRS station regime render+validate passed"
}

# eof
