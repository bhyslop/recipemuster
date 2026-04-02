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
source "${BURD_BUK_DIR}/bure_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbdc_DerivedConstants.sh"
source "${RBTB_SCRIPT_DIR}/rbrv_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbrp_regime.sh"
source "${RBTB_SCRIPT_DIR}/rbcc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgc_Constants.sh"
source "${RBTB_SCRIPT_DIR}/rbgd_DepotConstants.sh"
source "${RBTB_SCRIPT_DIR}/rbfd_FoundryDirectorBuild.sh"
source "${RBTB_SCRIPT_DIR}/rbob_bottle.sh"
source "${RBTB_SCRIPT_DIR}/rbgo_OAuth.sh"
source "${RBTB_SCRIPT_DIR}/rbgu_Utility.sh"
source "${RBTB_SCRIPT_DIR}/rbgi_IAM.sh"
source "${RBTB_SCRIPT_DIR}/rbgp_Payor.sh"
source "${RBTB_SCRIPT_DIR}/rbap_AccessProbe.sh"

# Source test case files
source "${RBTB_RBTS_DIR}/rbtckk_KickTires.sh"
source "${RBTB_RBTS_DIR}/rbtcqa_QualifyAll.sh"
  # rbtcap, rbtcfm, rbtcsj, rbtcpl: retired — now run via theurge
  # butcrg_RegimeSmoke: retired — now runs via theurge (tt/rbtd-r.Run.regime-smoke.sh)
source "${RBTB_BUTS_DIR}/butcrg_RegimeCredentials.sh"
  # butcev_*: retired — now runs via theurge (tt/rbtd-r.Run.enrollment-validation.sh)
  # rbtcrv_RegimeValidation: retired — now runs via theurge (tt/rbtd-r.Run.regime-validation.sh)
source "${RBTB_RBTS_DIR}/rbtcbe_BureEnvironment.sh"

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

  local -r z_nameplate_file="${RBBC_dot_dir}/${z_moniker}/${RBCC_rbrn_file}"
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

  rbtb_show "Kindling RBGC/RBGD/RBGO/RBOB"
  zrbgc_kindle
  zrbgd_kindle
  zrbgo_kindle
  zrbob_kindle
}

rbtb_exec_sentry() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_SENTRY}" "$@"
}

rbtb_exec_sentry_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_SENTRY}" "$@"
}

rbtb_exec_pentacle() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_PENTACLE}" "$@"
}

rbtb_exec_pentacle_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_PENTACLE}" "$@"
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

  # access-probe baste: retired — now runs via theurge (tt/rbtd-r.Run.access-probe.sh)
  # four-mode baste: retired — now runs via theurge (tt/rbtd-r.Run.four-mode.sh)
  # srjcl baste: retired — now runs via theurge (tt/rbtd-r.Run.srjcl.sh)
  # pluml baste: retired — now runs via theurge (tt/rbtd-r.Run.pluml.sh)

  # regime-validation baste: retired — now runs via theurge


######################################################################
# Registration

rbtb_kindle() {
  # Suite constants
  readonly BUTR_SUITE_FAST="fast"
  readonly BUTR_SUITE_SERVICE="service"
  readonly BUTR_SUITE_CRUCIBLE="crucible"
  readonly BUTR_SUITE_COMPLETE="complete"

  butr_kindle

  # -- FAST + CRUCIBLE + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_CRUCIBLE}" "${BUTR_SUITE_COMPLETE}"

  # kick-tires fixture
  butr_fixture_enroll "kick-tires" "" "zrbtb_noop_baste"
  butr_case_enroll "kick-tires" rbtckk_false_tcase
  butr_case_enroll "kick-tires" rbtckk_true_tcase

  # qualify-all fixture
  butr_fixture_enroll "qualify-all" "" "zrbtb_qualify_baste"
  butr_case_enroll "qualify-all" rbtcqa_qualify_fast_tcase

  # -- SERVICE + COMPLETE: needs credentials, no containers --
  butr_suite_enroll "${BUTR_SUITE_SERVICE}" "${BUTR_SUITE_COMPLETE}"

  # access-probe: retired — now runs via theurge (tt/rbtd-r.Run.access-probe.sh)
  # four-mode: retired — now runs via theurge (tt/rbtd-r.Run.four-mode.sh)

  # -- COMPLETE only: needs container runtime --
  butr_suite_enroll "${BUTR_SUITE_COMPLETE}"

  # -- FAST + CRUCIBLE + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_CRUCIBLE}" "${BUTR_SUITE_COMPLETE}"

  # -- CRUCIBLE + COMPLETE: needs container runtime, uses existing consecrations --
  butr_suite_enroll "${BUTR_SUITE_CRUCIBLE}" "${BUTR_SUITE_COMPLETE}"

  # tadmor-security: retired — now runs via theurge (tt/rbtd-r.Run.tadmor.sh)

  # srjcl-jupyter: retired — now runs via theurge (tt/rbtd-r.Run.srjcl.sh)
  # pluml-diagram: retired — now runs via theurge (tt/rbtd-r.Run.pluml.sh)

  # -- FAST + CRUCIBLE + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_CRUCIBLE}" "${BUTR_SUITE_COMPLETE}"

  # regime-smoke: retired — now runs via theurge (tt/rbtd-r.Run.regime-smoke.sh)

  # -- SERVICE + COMPLETE: needs all workstation credentials --
  butr_suite_enroll "${BUTR_SUITE_SERVICE}" "${BUTR_SUITE_COMPLETE}"

  # regime-credentials fixture (requires workstation credentials)
  butr_fixture_enroll "regime-credentials" "" "zrbtb_noop_baste"
  butr_case_enroll "regime-credentials" butcrg_rbra_tcase
  butr_case_enroll "regime-credentials" butcrg_rbro_tcase
  butr_case_enroll "regime-credentials" butcrg_rbrs_tcase

  # -- FAST + CRUCIBLE + COMPLETE: no external dependencies --
  butr_suite_enroll "${BUTR_SUITE_FAST}" "${BUTR_SUITE_CRUCIBLE}" "${BUTR_SUITE_COMPLETE}"

  # enrollment-validation: retired — now runs via theurge (tt/rbtd-r.Run.enrollment-validation.sh)

  # regime-validation: retired — now runs via theurge (tt/rbtd-r.Run.regime-validation.sh)

  # bure-tweak fixture
  butr_fixture_enroll "bure-tweak" "" "zrbtb_noop_baste"
  butr_case_enroll "bure-tweak" rbtcbe_tweak_empty_tcase
  butr_case_enroll "bure-tweak" rbtcbe_tweak_both_set_tcase
  butr_case_enroll "bure-tweak" rbtcbe_tweak_name_only_tcase
  butr_case_enroll "bure-tweak" rbtcbe_tweak_value_only_tcase
  butr_case_enroll "bure-tweak" rbtcbe_tweak_name_too_long_tcase
  butr_case_enroll "bure-tweak" rbtcbe_tweak_value_too_long_tcase
  butr_case_enroll "bure-tweak" rbtcbe_unexpected_var_tcase

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
    rbw-tf)
      local -r z_fixture="${BURD_TOKEN_3:-}"
      test -n "${z_fixture}" || buc_die "rbw-tf: fixture imprint required (tab-complete to see options)"
      butd_run_fixture "${z_fixture}"
      ;;
    rbw-ts)
      local -r z_suite="${BURD_TOKEN_3:-}"
      test -n "${z_suite}" || buc_die "rbw-ts: suite imprint required (fast, service, crucible, complete)"
      butd_run_suite "${z_suite}"
      ;;
    rbw-to)
      if test -z "${1:-}"; then
        butd_run_one ""
        buc_die "Test function name required"
      fi
      butd_run_one "${1}"
      ;;
    *)
      if test -n "${z_command}"; then
        buc_warn "Unknown command: ${z_command}"
      fi
      buc_info "Available test commands:"
      buc_info "  rbw-tf   Run single fixture (imprint selects fixture)"
      buc_info "  rbw-ts   Run suite (imprint: fast, service, complete)"
      buc_info "  rbw-to   Run single test function (param: function name)"
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
