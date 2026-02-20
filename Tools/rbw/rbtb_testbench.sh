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
source "${BURC_BUK_DIR}/burd_regime.sh"
source "${RBTB_SCRIPT_DIR}/../buk/bute_engine.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butr_registry.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butd_dispatch.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/rbz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${RBTB_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrv_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbcc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${RBTB_SCRIPT_DIR}/rbob_bottle.sh"

# Source test case files
source "${RBTB_SCRIPT_DIR}/rbtckk_KickTires.sh"
source "${RBTB_SCRIPT_DIR}/rbtcqa_QualifyAll.sh"
source "${RBTB_SCRIPT_DIR}/rbtcal_ArkLifecycle.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcde_DispatchExercise.sh"
source "${RBTB_SCRIPT_DIR}/rbtcns_NsproSecurity.sh"
source "${RBTB_SCRIPT_DIR}/rbtcsj_SrjclJupyter.sh"
source "${RBTB_SCRIPT_DIR}/rbtcpl_PlumlDiagram.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcvu_XnameValidation.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcrg_RegimeSmoke.sh"
source "${RBTB_SCRIPT_DIR}/../buk/butcrg_RegimeCredentials.sh"

buc_context "${0##*/}"
zburd_kindle
zrbcc_kindle
zbuz_kindle
zrbz_kindle

######################################################################
# Shared test helpers (migrated from rbt_testbench.sh)

rbtb_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "RBTSHOW: $*"
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

zrbtb_kick_tsuite_setup() {
  buto_trace "Setup for kick-tires suite (no-op)"
}

zrbtb_qualify_tsuite_setup() {
  buto_trace "Setup for qualify-all suite"
  source "${RBTB_SCRIPT_DIR}/rbq_Qualify.sh"
  zrbq_kindle
}

zrbtb_ark_tsuite_setup() {
  buto_trace "Setup for ark-lifecycle suite"
  ZRBTB_ARK_VESSEL_SIGIL="trbim-macos"
}

zrbtb_dispatch_tsuite_setup() {
  buto_trace "Setup for dispatch-exercise suite"
  buz_blazon ZBUTCDE_TEST_COLOPHON "butctt" "butcde_DispatchExercise" "butcde_run"
}

zrbtb_nsproto_tsuite_setup() {
  buto_trace "Setup for nsproto-security suite"
  rbtb_load_nameplate "nsproto"
}

zrbtb_srjcl_tsuite_setup() {
  buto_trace "Setup for srjcl-jupyter suite"
  rbtb_load_nameplate "srjcl"
}

zrbtb_pluml_tsuite_setup() {
  buto_trace "Setup for pluml-diagram suite"
  rbtb_load_nameplate "pluml"
}

zrbtb_xname_tsuite_setup() {
  buto_trace "Setup for xname-validation suite (no-op)"
}

zrbtb_regime_tsuite_setup() {
  buto_trace "Setup for regime-smoke suite (no-op)"
}

zrbtb_credentials_tsuite_setup() {
  buto_trace "Setup for regime-credentials suite (no-op)"
}


######################################################################
# Registration

rbtb_kindle() {
  butr_kindle

  # kick-tires suite
  butr_suite_enroll "kick-tires" "" "zrbtb_kick_tsuite_setup"
  butr_case_enroll "kick-tires" rbtckk_false_tcase
  butr_case_enroll "kick-tires" rbtckk_true_tcase

  # qualify-all suite
  butr_suite_enroll "qualify-all" "" "zrbtb_qualify_tsuite_setup"
  butr_case_enroll "qualify-all" rbtcqa_qualify_all_tcase

  # ark-lifecycle suite
  butr_suite_enroll "ark-lifecycle" "" "zrbtb_ark_tsuite_setup"
  butr_case_enroll "ark-lifecycle" rbtcal_lifecycle_tcase

  # dispatch-exercise suite
  butr_suite_enroll "dispatch-exercise" "" "zrbtb_dispatch_tsuite_setup"
  butr_case_enroll "dispatch-exercise" butcde_burv_isolation_tcase
  butr_case_enroll "dispatch-exercise" butcde_evidence_created_tcase
  butr_case_enroll "dispatch-exercise" butcde_exit_capture_tcase

  # nsproto-security suite
  butr_suite_enroll "nsproto-security" "" "zrbtb_nsproto_tsuite_setup"
  butr_case_enroll "nsproto-security" rbtcns_basic_dnsmasq_tcase
  butr_case_enroll "nsproto-security" rbtcns_basic_iptables_tcase
  butr_case_enroll "nsproto-security" rbtcns_basic_ping_sentry_tcase
  butr_case_enroll "nsproto-security" rbtcns_block_packages_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_allow_anthropic_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_altport_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_cloudflare_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_direct_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_google_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_ipv6_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_multicast_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_quad9_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_spoofing_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_tunneling_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_block_zonetransfer_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_nonexist_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_notcp_tcase
  butr_case_enroll "nsproto-security" rbtcns_dns_tcp_tcase
  butr_case_enroll "nsproto-security" rbtcns_icmp_block_beyond_tcase
  butr_case_enroll "nsproto-security" rbtcns_icmp_sentry_only_tcase
  butr_case_enroll "nsproto-security" rbtcns_tcp443_allow_anthropic_tcase
  butr_case_enroll "nsproto-security" rbtcns_tcp443_block_google_tcase

  # srjcl-jupyter suite
  butr_suite_enroll "srjcl-jupyter" "" "zrbtb_srjcl_tsuite_setup"
  butr_case_enroll "srjcl-jupyter" rbtcsj_jupyter_connectivity_tcase
  butr_case_enroll "srjcl-jupyter" rbtcsj_jupyter_running_tcase
  butr_case_enroll "srjcl-jupyter" rbtcsj_websocket_kernel_tcase

  # pluml-diagram suite
  butr_suite_enroll "pluml-diagram" "" "zrbtb_pluml_tsuite_setup"
  butr_case_enroll "pluml-diagram" rbtcpl_http_headers_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_invalid_hash_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_local_diagram_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_malformed_diagram_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_text_rendering_tcase

  # regime-smoke suite
  butr_suite_enroll "regime-smoke" "" "zrbtb_regime_tsuite_setup"
  butr_case_enroll "regime-smoke" butcrg_burc_tcase
  butr_case_enroll "regime-smoke" butcrg_burs_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrn_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrr_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrv_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrp_tcase
  butr_case_enroll "regime-smoke" butcrg_burd_tcase

  # regime-credentials suite (requires workstation credentials)
  butr_suite_enroll "regime-credentials" "" "zrbtb_credentials_tsuite_setup"
  butr_case_enroll "regime-credentials" butcrg_rbra_tcase
  butr_case_enroll "regime-credentials" butcrg_rbro_tcase
  butr_case_enroll "regime-credentials" butcrg_rbrs_tcase

  # xname-validation suite
  butr_suite_enroll "xname-validation" "" "zrbtb_xname_tsuite_setup"
  butr_case_enroll "xname-validation" butcvu_debug_tcase
  butr_case_enroll "xname-validation" butcvu_xname_defaults_tcase
  butr_case_enroll "xname-validation" butcvu_xname_empty_optional_tcase
  butr_case_enroll "xname-validation" butcvu_xname_env_wrapper_tcase
  butr_case_enroll "xname-validation" butcvu_xname_invalid_chars_tcase
  butr_case_enroll "xname-validation" butcvu_xname_invalid_start_tcase
  butr_case_enroll "xname-validation" butcvu_xname_length_tcase
  butr_case_enroll "xname-validation" butcvu_xname_valid_tcase

}

######################################################################
# Routing

rbtb_route() {
  local z_command="${1:-}"
  shift || true

  zburd_sentinel

  rbtb_kindle

  export ZBUTE_ROOT_TEMP_DIR="${BURD_TEMP_DIR}"
  export BUT_VERBOSE="${BUT_VERBOSE:-0}"

  case "${z_command}" in
    rbw-ta) butd_run_all ;;
    rbw-ts) butd_run_suite "${1:-}" ;;
    rbw-to) butd_run_one "${1:-}" ;;
    rbw-tn)
      local z_imprint="${BURD_TOKEN_3:-}"
      case "${z_imprint}" in
        nsproto) butd_run_suite "nsproto-security" ;;
        srjcl)  butd_run_suite "srjcl-jupyter" ;;
        pluml)  butd_run_suite "pluml-diagram" ;;
        *)      buc_die "rbw-tn: unknown nameplate imprint '${z_imprint}' (expected: nsproto, srjcl, pluml)" ;;
      esac
      ;;
    rbw-trg) butd_run_suite "regime-smoke" ;;
    rbw-trc) butd_run_suite "regime-credentials" ;;
    *)
      if [ -n "${z_command}" ]; then
        buc_warn "Unknown command: ${z_command}"
      fi
      buc_info "Available test commands:"
      buc_info "  rbw-ta   Run all suites"
      buc_info "  rbw-ts   Run single suite (pass suite name)"
      buc_info "  rbw-to   Run single test function (pass function name)"
      buc_info "  rbw-tn   Run nameplate suite (imprint: nsproto, srjcl, pluml)"
      buc_info "  rbw-trg  Run regime-smoke suite"
      buc_info "  rbw-trc  Run regime-credentials suite"
      return 0
      ;;
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
