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
RBTB_RBTS_DIR="${RBTB_SCRIPT_DIR}/rbts"
RBTB_BUTS_DIR="${BURD_BUK_DIR}/buts"

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"
source "${BURD_BUK_DIR}/bute_engine.sh"
source "${BURD_BUK_DIR}/butr_registry.sh"
source "${BURD_BUK_DIR}/butd_dispatch.sh"
source "${BURD_BUK_DIR}/buz_zipper.sh"
source "${RBTB_SCRIPT_DIR}/rbz_zipper.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${RBTB_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbdc_DerivedConstants.sh"
source "${RBTB_SCRIPT_DIR}/rbrv_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrp_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbcc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${RBTB_SCRIPT_DIR}/rbob_bottle.sh"
source "${RBTB_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${RBTB_SCRIPT_DIR}/rbgu_Utility.sh"
source "${RBTB_SCRIPT_DIR}/rbgi_IAM.sh"
source "${RBTB_SCRIPT_DIR}/rbgp_Payor.sh"
source "${RBTB_SCRIPT_DIR}/rbap_AccessProbe.sh"

# Source test case files
source "${RBTB_RBTS_DIR}/rbtckk_KickTires.sh"
source "${RBTB_RBTS_DIR}/rbtcqa_QualifyAll.sh"
source "${RBTB_RBTS_DIR}/rbtcap_AccessProbe.sh"
source "${RBTB_RBTS_DIR}/rbtcal_ArkLifecycle.sh"
source "${RBTB_RBTS_DIR}/rbtcsl_SlsaProvenance.sh"
source "${RBTB_RBTS_DIR}/rbtcns_NsproSecurity.sh"
source "${RBTB_RBTS_DIR}/rbtcsj_SrjclJupyter.sh"
source "${RBTB_RBTS_DIR}/rbtcpl_PlumlDiagram.sh"
source "${RBTB_BUTS_DIR}/butcrg_RegimeSmoke.sh"
source "${RBTB_BUTS_DIR}/butcrg_RegimeCredentials.sh"
source "${RBTB_BUTS_DIR}/butcev_LengthTypes.sh"
source "${RBTB_BUTS_DIR}/butcev_ChoiceTypes.sh"
source "${RBTB_BUTS_DIR}/butcev_NumericTypes.sh"
source "${RBTB_BUTS_DIR}/butcev_RefTypes.sh"
source "${RBTB_BUTS_DIR}/butcev_ListTypes.sh"
source "${RBTB_BUTS_DIR}/butcev_GateEnroll.sh"
source "${RBTB_BUTS_DIR}/butcev_EnforceReport.sh"

buc_context "${0##*/}"
zbuv_kindle
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
  local -r z_moniker="${1:-}"
  test -n "${z_moniker}" || buc_die "rbtb_load_nameplate: moniker argument required"

  rbtb_show "Loading nameplate: ${z_moniker}"

  local -r z_nameplate_file="${RBBC_dot_dir}/${RBCC_rbrn_prefix}${z_moniker}${RBCC_rbrn_ext}"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"
  zrbrn_kindle
  zrbrn_enforce

  rbtb_show "Nameplate loaded: RBRN_MONIKER=${RBRN_MONIKER}, RBRN_RUNTIME=${RBRN_RUNTIME}"

  rbtb_show "Loading RBRR via enrollment"
  source "${RBBC_rbrr_file}" || buc_die "Failed to source ${RBBC_rbrr_file}"
  zrbrr_kindle
  zrbrr_enforce
  zrbdc_kindle

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
# Litmus predicates (BCG _litmus_predicate: 0=proceed, 1=skip, never dies, no output)

zrbtb_container_runtime_litmus_predicate() {
  timeout 5 docker version >/dev/null 2>/dev/null || return 1
  return 0
}

zrbtb_clean_git_litmus_predicate() {
  git diff-index --quiet HEAD -- || return 1
  return 0
}

zrbtb_container_clean_git_litmus_predicate() {
  zrbtb_container_runtime_litmus_predicate || return 1
  zrbtb_clean_git_litmus_predicate || return 1
  return 0
}

######################################################################
# Baste functions (BCG _baste: kindle, source, configure inside fixture subshell)

zrbtb_noop_baste() {
  buto_trace "Baste (no-op)"
}

zrbtb_qualify_baste() {
  buto_trace "Baste for qualify-all fixture"
  source "${RBTB_SCRIPT_DIR}/rbq_Qualify.sh"
  zrbq_kindle
}

zrbtb_access_probe_baste() {
  buto_trace "Baste for access-probe fixture"
  # 5 iterations with 1500ms delay between calls
  # Total runtime: ~4 roles × 5 × 1.5s ≈ 30 seconds
  ZRBTCAP_ITERATIONS=5
  ZRBTCAP_DELAY_MS=1500
}

zrbtb_ark_baste() {
  buto_trace "Baste for ark-lifecycle fixture"
  ZRBTB_ARK_VESSEL_SIGIL="rbev-busybox"
  zrbgc_kindle
}

zrbtb_slsa_baste() {
  buto_trace "Baste for slsa-provenance fixture"
  ZRBTB_ARK_VESSEL_SIGIL="rbev-busybox"
  zrbgc_kindle
}

zrbtb_nsproto_baste() {
  buto_trace "Baste for nsproto-security fixture"
  rbtb_load_nameplate "nsproto"
}

zrbtb_srjcl_baste() {
  buto_trace "Baste for srjcl-jupyter fixture"
  rbtb_load_nameplate "srjcl"
}

zrbtb_pluml_baste() {
  buto_trace "Baste for pluml-diagram fixture"
  rbtb_load_nameplate "pluml"
}


######################################################################
# Registration

rbtb_kindle() {
  # Sweep suite constants
  readonly BUTR_SUITE_FAST="fast"
  readonly BUTR_SUITE_SERVICE="service"
  readonly BUTR_SUITE_COMPLETE="complete"

  butr_kindle

  # -- FAST + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_COMPLETE}"

  # kick-tires fixture
  butr_fixture_enroll "kick-tires" "" "zrbtb_noop_baste"
  butr_case_enroll "kick-tires" rbtckk_false_tcase
  butr_case_enroll "kick-tires" rbtckk_true_tcase

  # qualify-all fixture
  butr_fixture_enroll "qualify-all" "" "zrbtb_qualify_baste"
  butr_case_enroll "qualify-all" rbtcqa_qualify_all_tcase

  # -- SERVICE + COMPLETE: needs credentials, no containers --
  butr_suite_enroll "${BUTR_SUITE_SERVICE}" "${BUTR_SUITE_COMPLETE}"

  # access-probe fixture (runs before ark-lifecycle; ~30s smoke test for OAuth/credential issues)
  # Regression tests for rbgo_OAuth.sh stderr-capture fix (pace AfAAR)
  butr_fixture_enroll "access-probe" "" "zrbtb_access_probe_baste"
  butr_case_enroll "access-probe" rbtcap_jwt_governor_tcase
  butr_case_enroll "access-probe" rbtcap_jwt_director_tcase
  butr_case_enroll "access-probe" rbtcap_jwt_retriever_tcase
  butr_case_enroll "access-probe" rbtcap_payor_oauth_tcase

  # -- COMPLETE only: needs container runtime --
  butr_suite_enroll "${BUTR_SUITE_COMPLETE}"

  # ark-lifecycle fixture
  butr_fixture_enroll "ark-lifecycle" "zrbtb_container_clean_git_litmus_predicate" "zrbtb_ark_baste"
  butr_case_enroll "ark-lifecycle" rbtcal_lifecycle_tcase

  # slsa-provenance fixture (rbev-busybox 3-platform, exercises multi-platform provenance)
  butr_fixture_enroll "slsa-provenance" "zrbtb_container_clean_git_litmus_predicate" "zrbtb_slsa_baste"
  butr_case_enroll "slsa-provenance" rbtcsl_provenance_tcase

  # -- FAST + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_COMPLETE}"

  # -- COMPLETE only: needs container runtime --
  butr_suite_enroll "${BUTR_SUITE_COMPLETE}"

  # nsproto-security fixture
  butr_fixture_enroll "nsproto-security" "zrbtb_container_runtime_litmus_predicate" "zrbtb_nsproto_baste"
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

  # srjcl-jupyter fixture
  butr_fixture_enroll "srjcl-jupyter" "zrbtb_container_runtime_litmus_predicate" "zrbtb_srjcl_baste"
  butr_case_enroll "srjcl-jupyter" rbtcsj_jupyter_connectivity_tcase
  butr_case_enroll "srjcl-jupyter" rbtcsj_jupyter_running_tcase
  butr_case_enroll "srjcl-jupyter" rbtcsj_websocket_kernel_tcase

  # pluml-diagram fixture
  butr_fixture_enroll "pluml-diagram" "zrbtb_container_runtime_litmus_predicate" "zrbtb_pluml_baste"
  butr_case_enroll "pluml-diagram" rbtcpl_http_headers_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_invalid_hash_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_local_diagram_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_malformed_diagram_tcase
  butr_case_enroll "pluml-diagram" rbtcpl_text_rendering_tcase

  # -- FAST + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_COMPLETE}"

  # regime-smoke fixture
  butr_fixture_enroll "regime-smoke" "" "zrbtb_noop_baste"
  butr_case_enroll "regime-smoke" butcrg_burc_tcase
  butr_case_enroll "regime-smoke" butcrg_burs_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrn_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrr_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrv_tcase
  butr_case_enroll "regime-smoke" butcrg_rbrp_tcase
  butr_case_enroll "regime-smoke" butcrg_burd_tcase

  # -- SERVICE + COMPLETE: needs credentials, no containers --
  butr_suite_enroll "${BUTR_SUITE_SERVICE}" "${BUTR_SUITE_COMPLETE}"

  # regime-credentials fixture (requires workstation credentials)
  butr_fixture_enroll "regime-credentials" "" "zrbtb_noop_baste"
  butr_case_enroll "regime-credentials" butcrg_rbra_tcase
  butr_case_enroll "regime-credentials" butcrg_rbro_tcase
  butr_case_enroll "regime-credentials" butcrg_rbrs_tcase

  # -- FAST + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_COMPLETE}"

  # enrollment-validation fixture
  butr_fixture_enroll "enrollment-validation" "" "zrbtb_noop_baste"
  butr_case_enroll "enrollment-validation" butcev_string_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_string_empty_optional_tcase
  butr_case_enroll "enrollment-validation" butcev_string_too_short_tcase
  butr_case_enroll "enrollment-validation" butcev_string_too_long_tcase
  butr_case_enroll "enrollment-validation" butcev_string_empty_required_tcase
  butr_case_enroll "enrollment-validation" butcev_xname_enrolled_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_xname_enrolled_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_gname_enrolled_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_gname_enrolled_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_fqin_enrolled_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_fqin_enrolled_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_bool_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_bool_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_bool_empty_tcase
  butr_case_enroll "enrollment-validation" butcev_enum_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_enum_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_enum_empty_tcase
  butr_case_enroll "enrollment-validation" butcev_decimal_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_decimal_below_tcase
  butr_case_enroll "enrollment-validation" butcev_decimal_above_tcase
  butr_case_enroll "enrollment-validation" butcev_decimal_empty_tcase
  butr_case_enroll "enrollment-validation" butcev_ipv4_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_ipv4_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_port_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_port_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_odref_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_odref_no_digest_tcase
  butr_case_enroll "enrollment-validation" butcev_odref_malformed_tcase
  butr_case_enroll "enrollment-validation" butcev_odref_empty_tcase
  butr_case_enroll "enrollment-validation" butcev_list_string_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_list_string_empty_tcase
  butr_case_enroll "enrollment-validation" butcev_list_string_bad_item_tcase
  butr_case_enroll "enrollment-validation" butcev_list_ipv4_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_list_ipv4_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_list_ipv4_empty_tcase
  butr_case_enroll "enrollment-validation" butcev_list_gname_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_list_gname_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_gate_active_valid_tcase
  butr_case_enroll "enrollment-validation" butcev_gate_active_invalid_tcase
  butr_case_enroll "enrollment-validation" butcev_gate_inactive_tcase
  butr_case_enroll "enrollment-validation" butcev_gate_multi_tcase
  butr_case_enroll "enrollment-validation" butcev_enforce_all_pass_tcase
  butr_case_enroll "enrollment-validation" butcev_enforce_first_bad_tcase
  butr_case_enroll "enrollment-validation" butcev_report_all_pass_tcase
  butr_case_enroll "enrollment-validation" butcev_report_mixed_tcase
  butr_case_enroll "enrollment-validation" butcev_report_gated_tcase
  butr_case_enroll "enrollment-validation" butcev_multiscope_tcase

}

######################################################################
# Routing

rbtb_route() {
  local -r z_command="${1:-}"
  shift || true

  zburd_sentinel

  rbtb_kindle

  export ZBUTE_ROOT_TEMP_DIR="${BURD_TEMP_DIR}"
  export BUT_VERBOSE="${BUT_VERBOSE:-0}"

  case "${z_command}" in
    rbw-ta) butd_run_all ;;
    rbw-ts) butd_run_fixture "${1:-}" ;;
    rbw-to) butd_run_one "${1:-}" ;;
    rbw-tw) butd_run_sweep "${1:-}" ;;
    rbw-tn)
      local -r z_imprint="${BURD_TOKEN_3:-}"
      case "${z_imprint}" in
        nsproto) butd_run_fixture "nsproto-security" ;;
        srjcl)  butd_run_fixture "srjcl-jupyter" ;;
        pluml)  butd_run_fixture "pluml-diagram" ;;
        *)      buc_die "rbw-tn: unknown nameplate imprint '${z_imprint}' (expected: nsproto, srjcl, pluml)" ;;
      esac
      ;;
    rbw-tap) butd_run_fixture "access-probe" ;;
    rbw-trg) butd_run_fixture "regime-smoke" ;;
    rbw-trc) butd_run_fixture "regime-credentials" ;;
    *)
      if [ -n "${z_command}" ]; then
        buc_warn "Unknown command: ${z_command}"
      fi
      buc_info "Available test commands:"
      buc_info "  rbw-ta   Run all fixtures"
      buc_info "  rbw-ts   Run single fixture (pass fixture name)"
      buc_info "  rbw-to   Run single test function (pass function name)"
      buc_info "  rbw-tw   Run sweep suite (pass sweep name: fast, service, complete)"
      buc_info "  rbw-tn   Run nameplate fixture (imprint: nsproto, srjcl, pluml)"
      buc_info "  rbw-tap  Run access-probe fixture (OAuth/credential smoke test)"
      buc_info "  rbw-trg  Run regime-smoke fixture"
      buc_info "  rbw-trc  Run regime-credentials fixture"
      return 0
      ;;
  esac
}

rbtb_main() {
  local -r z_command="${1:-}"
  shift || true
  test -n "${z_command}" || buc_die "No command specified"
  rbtb_route "${z_command}" "$@"
}

rbtb_main "$@"

# eof
