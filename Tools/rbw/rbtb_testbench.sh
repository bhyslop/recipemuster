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
source "${RBTB_SCRIPT_DIR}/../buk/bute_engine.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butr_registry.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butd_dispatch.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/rbz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${RBTB_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbcc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${RBTB_SCRIPT_DIR}/rbob_bottle.sh"

# Source test case files
source "${RBTB_SCRIPT_DIR}/rbtckk_KickTires.sh"
source "${RBTB_SCRIPT_DIR}/rbtcal_ArkLifecycle.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcde_DispatchExercise.sh"
source "${RBTB_SCRIPT_DIR}/rbtcns_NsproSecurity.sh"
source "${RBTB_SCRIPT_DIR}/rbtcsj_SrjclJupyter.sh"
source "${RBTB_SCRIPT_DIR}/rbtcpl_PlumlDiagram.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcvu_XnameValidation.sh"

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

  rbtb_show "Kindling RBGC/RBGD/RBOB"
  zrbgc_kindle
  zrbgd_kindle
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
  bute_init_dispatch
  bute_init_evidence
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


######################################################################
# Registration

rbtb_kindle() {
  butr_kindle

  # kick-tires suite
  butr_suite_enroll "kick-tires" "" "zrbtb_setup_kick"
  butr_case_enroll "kick-tires" rbtckk_false
  butr_case_enroll "kick-tires" rbtckk_true

  # ark-lifecycle suite
  butr_suite_enroll "ark-lifecycle" "" "zrbtb_setup_ark"
  butr_case_enroll "ark-lifecycle" rbtcal_lifecycle

  # dispatch-exercise suite
  butr_suite_enroll "dispatch-exercise" "" "zrbtb_setup_dispatch"
  butr_case_enroll "dispatch-exercise" butcde_burv_isolation
  butr_case_enroll "dispatch-exercise" butcde_evidence_created
  butr_case_enroll "dispatch-exercise" butcde_exit_capture

  # nsproto-security suite
  butr_suite_enroll "nsproto-security" "" "zrbtb_setup_nsproto"
  butr_case_enroll "nsproto-security" rbtcns_basic_dnsmasq
  butr_case_enroll "nsproto-security" rbtcns_basic_iptables
  butr_case_enroll "nsproto-security" rbtcns_basic_ping_sentry
  butr_case_enroll "nsproto-security" rbtcns_block_packages
  butr_case_enroll "nsproto-security" rbtcns_dns_allow_anthropic
  butr_case_enroll "nsproto-security" rbtcns_dns_block_altport
  butr_case_enroll "nsproto-security" rbtcns_dns_block_cloudflare
  butr_case_enroll "nsproto-security" rbtcns_dns_block_direct
  butr_case_enroll "nsproto-security" rbtcns_dns_block_google
  butr_case_enroll "nsproto-security" rbtcns_dns_block_ipv6
  butr_case_enroll "nsproto-security" rbtcns_dns_block_multicast
  butr_case_enroll "nsproto-security" rbtcns_dns_block_quad9
  butr_case_enroll "nsproto-security" rbtcns_dns_block_spoofing
  butr_case_enroll "nsproto-security" rbtcns_dns_block_tunneling
  butr_case_enroll "nsproto-security" rbtcns_dns_block_zonetransfer
  butr_case_enroll "nsproto-security" rbtcns_dns_nonexist
  butr_case_enroll "nsproto-security" rbtcns_dns_notcp
  butr_case_enroll "nsproto-security" rbtcns_dns_tcp
  butr_case_enroll "nsproto-security" rbtcns_icmp_block_beyond
  butr_case_enroll "nsproto-security" rbtcns_icmp_sentry_only
  butr_case_enroll "nsproto-security" rbtcns_tcp443_allow_anthropic
  butr_case_enroll "nsproto-security" rbtcns_tcp443_block_google

  # srjcl-jupyter suite
  butr_suite_enroll "srjcl-jupyter" "" "zrbtb_setup_srjcl"
  butr_case_enroll "srjcl-jupyter" rbtcsj_jupyter_connectivity
  butr_case_enroll "srjcl-jupyter" rbtcsj_jupyter_running
  butr_case_enroll "srjcl-jupyter" rbtcsj_websocket_kernel

  # pluml-diagram suite
  butr_suite_enroll "pluml-diagram" "" "zrbtb_setup_pluml"
  butr_case_enroll "pluml-diagram" rbtcpl_http_headers
  butr_case_enroll "pluml-diagram" rbtcpl_invalid_hash
  butr_case_enroll "pluml-diagram" rbtcpl_local_diagram
  butr_case_enroll "pluml-diagram" rbtcpl_malformed_diagram
  butr_case_enroll "pluml-diagram" rbtcpl_text_rendering

  # xname-validation suite
  butr_suite_enroll "xname-validation" "" "zrbtb_setup_xname"
  butr_case_enroll "xname-validation" butcvu_debug
  butr_case_enroll "xname-validation" butcvu_xname_defaults
  butr_case_enroll "xname-validation" butcvu_xname_empty_optional
  butr_case_enroll "xname-validation" butcvu_xname_env_wrapper
  butr_case_enroll "xname-validation" butcvu_xname_invalid_chars
  butr_case_enroll "xname-validation" butcvu_xname_invalid_start
  butr_case_enroll "xname-validation" butcvu_xname_length
  butr_case_enroll "xname-validation" butcvu_xname_valid

}

######################################################################
# Routing

rbtb_route() {
  local z_command="${1:-}"
  shift || true

  test -n "${BURD_TEMP_DIR:-}"   || buc_die "BURD_TEMP_DIR not set - must be called from BURD"
  test -n "${BURD_NOW_STAMP:-}"  || buc_die "BURD_NOW_STAMP not set - must be called from BURD"

  rbtb_kindle

  export ZBUTE_ROOT_TEMP_DIR="${BURD_TEMP_DIR}"
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
