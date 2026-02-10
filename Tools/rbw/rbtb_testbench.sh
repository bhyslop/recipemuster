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
# RBTB Testbench - Recipe Bottle test framework testbench

set -euo pipefail

RBTB_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBTB_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buto_operations.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butr_registry.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butd_dispatch.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/rbz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${RBTB_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbcc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbob_bottle.sh"

# Source test case files
source "${RBTB_SCRIPT_DIR}/rbtckk_KickTires.sh"
source "${RBTB_SCRIPT_DIR}/rbtcal_ArkLifecycle.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcde_DispatchExercise.sh"
source "${RBTB_SCRIPT_DIR}/rbtcns_NsproSecurity.sh"
source "${RBTB_SCRIPT_DIR}/rbtcsj_SrjclJupyter.sh"
source "${RBTB_SCRIPT_DIR}/rbtcpl_PlumlDiagram.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcvu_XnameValidation.sh"
source "${RBTB_SCRIPT_DIR}/rbtcim_ImageManagement.sh"

buc_context "${0##*/}"
zrbcc_kindle

######################################################################
# Shared test helpers (migrated from rbt_testbench.sh)

rbtb_show() {
  test "${BURD_VERBOSE:-0}" != "1" || echo "RBTSHOW: $*"
}

rbtb_load_nameplate() {
  local z_moniker="${1:-}"
  test -n "${z_moniker}" || buc_die "rbtb_load_nameplate: moniker argument required"

  rbtb_show "Loading nameplate: ${z_moniker}"
  rbrn_load_moniker "${z_moniker}"

  rbtb_show "Nameplate loaded: RBRN_MONIKER=${RBRN_MONIKER}, RBRN_RUNTIME=${RBRN_RUNTIME}"

  rbtb_show "Loading RBRR"
  rbrr_load

  rbtb_show "Kindling RBOB"
  zrbob_kindle
}

rbtb_exec_sentry() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_SENTRY}" "$@"
}

rbtb_exec_sentry_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_SENTRY}" "$@"
}

rbtb_exec_censer() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" "$@"
}

rbtb_exec_censer_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_CENSER}" "$@"
}

rbtb_exec_bottle() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_BOTTLE}" "$@"
}

rbtb_exec_bottle_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_BOTTLE}" "$@"
}

######################################################################
# Setup functions

zrbtb_setup_kick() {
  buto_trace "Setup for kick-tires suite (no-op)"
}

zrbtb_setup_ark() {
  buto_trace "Setup for ark-lifecycle suite"
  zbuz_kindle
  zrbz_kindle
  buto_init_dispatch
  buto_init_evidence
  ZRBTB_ARK_VESSEL_SIGIL="trbim-macos"
}

zrbtb_setup_dispatch() {
  buto_trace "Setup for dispatch-exercise suite"
  zbuz_kindle
  buz_register "butctt" "butcde_DispatchExercise" "butcde_run"
  ZBUTCDE_TEST_COLOPHON="${z1z_buz_colophon}"
}

zrbtb_setup_nsproto() {
  buto_trace "Setup for nsproto-security suite"
  rbtb_load_nameplate "nsproto"
}

zrbtb_setup_srjcl() {
  buto_trace "Setup for srjcl-jupyter suite"
  rbtb_load_nameplate "srjcl"
}

zrbtb_setup_pluml() {
  buto_trace "Setup for pluml-diagram suite"
  rbtb_load_nameplate "pluml"
}

zrbtb_setup_xname() {
  buto_trace "Setup for xname-validation suite (no-op)"
}

zrbtb_setup_image() {
  buto_trace "Setup for image-management suite"
  rbrr_load || buto_fatal "Failed to load RBRR configuration"
}

######################################################################
# Registration

rbtb_kindle() {
  butr_kindle
  butr_register "kick-tires" "rbtckk_" "zrbtb_setup_kick" "fast"
  butr_register "ark-lifecycle" "rbtcal_" "zrbtb_setup_ark" "slow"
  butr_register "dispatch-exercise" "butcde_" "zrbtb_setup_dispatch" "fast"
  butr_register "nsproto-security" "rbtcns_" "zrbtb_setup_nsproto" "slow"
  butr_register "srjcl-jupyter" "rbtcsj_" "zrbtb_setup_srjcl" "slow"
  butr_register "pluml-diagram" "rbtcpl_" "zrbtb_setup_pluml" "slow"
  butr_register "xname-validation" "butcvu_" "zrbtb_setup_xname" "fast"
  butr_register "image-management" "rbtcim_" "zrbtb_setup_image" "slow"
}

######################################################################
# Routing

rbtb_route() {
  local z_command="${1:-}"
  shift || true

  test -n "${BURD_TEMP_DIR:-}"   || buc_die "BURD_TEMP_DIR not set - must be called from BURD"
  test -n "${BURD_NOW_STAMP:-}"  || buc_die "BURD_NOW_STAMP not set - must be called from BURD"

  rbtb_kindle

  export ZBUTO_ROOT_TEMP_DIR="${BURD_TEMP_DIR}"
  export BUT_VERBOSE="${BUT_VERBOSE:-0}"

  case "${z_command}" in
    rbtb-ta) butd_run_all ;;
    rbtb-ts) butd_run_suite "${1:-}" ;;
    rbtb-to) butd_run_one "${1:-}" ;;
    rbtb-ns) butd_run_suite "nsproto-security" ;;
    rbtb-sj) butd_run_suite "srjcl-jupyter" ;;
    rbtb-pl) butd_run_suite "pluml-diagram" ;;
    *)       buc_die "Unknown command: ${z_command}" ;;
  esac
}

rbtb_main() {
  local z_command="${1:-}"
  shift || true
  test -n "${z_command}" || buc_die "No command specified"
  rbtb_route "${z_command}" "$@"
}

rbtb_main "$@"

# eof
