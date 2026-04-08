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
# BUTT Testbench - BUK test framework self-test
#
# Exercises the BUK test framework (bute/butr/butd/buto) with kick-tires
# and BURE tweak cases.  Pure local — no GCP, no containers, no network.

set -euo pipefail

BUTT_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
BUTT_BUTS_DIR="${BUTT_SCRIPT_DIR}/buts"

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/burd_regime.sh"
source "${BURD_BUK_DIR}/buv_validation.sh"
source "${BURD_BUK_DIR}/bute_engine.sh"
source "${BURD_BUK_DIR}/butr_registry.sh"
source "${BURD_BUK_DIR}/butd_dispatch.sh"
source "${BURD_BUK_DIR}/bure_regime.sh"
source "${BURD_BUK_DIR}/buf_fact.sh"
source "${BURD_BUK_DIR}/bug_guide.sh"

# Source test case files
source "${BUTT_BUTS_DIR}/butckk_KickTires.sh"
source "${BUTT_BUTS_DIR}/butcbe_BureEnvironment.sh"
source "${BUTT_BUTS_DIR}/butcbx_BurxExchange.sh"
source "${BUTT_BUTS_DIR}/butclc_LinkCombinator.sh"

buc_context "${0##*/}"
zbuv_kindle
zburd_kindle

######################################################################
# Registration

butt_kindle() {
  butr_kindle

  # All fixtures are pure local
  butr_suite_enroll "self-test"

  # kick-tires fixture (2 cases)
  butr_fixture_enroll "kick-tires" "" "zbutt_noop_baste"
  butr_case_enroll "kick-tires" butckk_false_tcase
  butr_case_enroll "kick-tires" butckk_true_tcase

  # bure-tweak fixture (9 cases)
  butr_fixture_enroll "bure-tweak" "" "zbutt_noop_baste"
  butr_case_enroll "bure-tweak" butcbe_tweak_empty_tcase
  butr_case_enroll "bure-tweak" butcbe_tweak_both_set_tcase
  butr_case_enroll "bure-tweak" butcbe_tweak_name_only_tcase
  butr_case_enroll "bure-tweak" butcbe_tweak_value_only_tcase
  butr_case_enroll "bure-tweak" butcbe_tweak_name_too_long_tcase
  butr_case_enroll "bure-tweak" butcbe_tweak_value_too_long_tcase
  butr_case_enroll "bure-tweak" butcbe_label_valid_tcase
  butr_case_enroll "bure-tweak" butcbe_label_too_long_tcase
  butr_case_enroll "bure-tweak" butcbe_unexpected_var_tcase

  # burx-exchange fixture (4 cases)
  butr_fixture_enroll "burx-exchange" "" "zbutt_noop_baste"
  butr_case_enroll "burx-exchange" butcbx_burx_dual_write_tcase
  butr_case_enroll "burx-exchange" butcbx_burx_fields_tcase
  butr_case_enroll "burx-exchange" butcbx_burx_preexist_tcase
  butr_case_enroll "burx-exchange" butcbx_burx_timestamp_format_tcase

  # bug-link fixture (3 cases)
  butr_fixture_enroll "bug-link" "" "zbutt_noop_baste"
  butr_case_enroll "bug-link" butclc_tlt_osc8_tcase
  butr_case_enroll "bug-link" butclc_tlt_fallback_tcase
  butr_case_enroll "bug-link" butclc_all_combinators_tcase
}

zbutt_noop_baste() {
  buto_trace "Baste (no-op)"
}

######################################################################
# Routing

butt_route() {
  local -r z_command="${1:-}"
  shift || true

  zburd_sentinel

  butt_kindle

  export ZBUTE_ROOT_TEMP_DIR="${BURD_TEMP_DIR}"
  export BUT_VERBOSE="${BUT_VERBOSE:-0}"

  case "${z_command}" in
    buw-st)
      butd_run_suite "self-test"
      ;;
    *)
      buc_die "Unknown command: ${z_command}"
      ;;
  esac
}

butt_main() {
  local -r z_command="${1:-}"
  shift || true
  test -n "${z_command}" || buc_die "No command specified"
  butt_route "${z_command}" "$@"
}

butt_main "$@"

# eof
