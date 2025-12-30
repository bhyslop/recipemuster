#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# RBT Testbench - Recipe Bottle test execution
#
# Commands:
#   rbt-to  Run test suite for a nameplate (e.g., rbt-to nsproto)

set -euo pipefail

# Get script directory
RBT_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBT_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBT_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${RBT_SCRIPT_DIR}/../buk/but_test.sh"
source "${RBT_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBT_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBT_SCRIPT_DIR}/rbob_bottle.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

######################################################################
# Helper Functions

# Verbose output if BUD_VERBOSE is set
rbt_show() {
  test "${BUD_VERBOSE:-0}" != "1" || echo "RBTSHOW: $*"
}

# Load nameplate configuration by moniker and kindle RBOB
# Usage: rbt_load_nameplate <moniker>
rbt_load_nameplate() {
  local z_moniker="${1:-}"
  test -n "${z_moniker}" || buc_die "rbt_load_nameplate: moniker argument required"

  local z_nameplate_file="${RBT_SCRIPT_DIR}/rbrn_${z_moniker}.env"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"

  rbt_show "Loading nameplate: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"

  rbt_show "Kindling nameplate regime"
  zrbrn_kindle

  rbt_show "Nameplate loaded: RBRN_MONIKER=${RBRN_MONIKER}, RBRN_RUNTIME=${RBRN_RUNTIME}"

  # Load RBRR (repository regime)
  rbt_show "Loading RBRR"
  local z_rbrr_file="${RBT_SCRIPT_DIR}/../../rbrr_RecipeBottleRegimeRepo.sh"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"
  source "${z_rbrr_file}" || buc_die "Failed to source RBRR config: ${z_rbrr_file}"
  zrbrr_kindle

  # Kindle RBOB (validates RBRN and RBRR are ready, sets container names)
  rbt_show "Kindling RBOB"
  zrbob_kindle
}

######################################################################
# Container Exec Helpers
#
# Two variants per container:
#   rbt_exec_*   - Simple exec (no -i), for commands that just output and exit
#   rbt_exec_*_i - With -i flag, for commands that may read stdin (dig, traceroute, apt-get)

rbt_exec_sentry() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_SENTRY}" "$@"
}

rbt_exec_sentry_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_SENTRY}" "$@"
}

rbt_exec_censer() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" "$@"
}

rbt_exec_censer_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_CENSER}" "$@"
}

rbt_exec_bottle() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_BOTTLE}" "$@"
}

rbt_exec_bottle_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_BOTTLE}" "$@"
}

######################################################################
# Test Cases - nsproto security tests

test_nsproto_dns_allow_anthropic() {
  but_expect_ok rbt_exec_bottle nslookup anthropic.com
}

######################################################################
# Test Suites

rbt_suite_nsproto() {
  buc_step "Running nsproto security test suite"
  local z_test_dir="${BUD_TEMP_DIR}/tests"
  mkdir -p "${z_test_dir}"
  but_execute "${z_test_dir}" "test_nsproto_" ""
}

rbt_suite_srjcl() {
  buc_step "Running srjcl Jupyter test suite"
  but_fatal "srjcl test suite not yet implemented"
}

rbt_suite_pluml() {
  buc_step "Running pluml PlantUML test suite"
  but_fatal "pluml test suite not yet implemented"
}

######################################################################
# Routing

rbt_route() {
  local z_command="${1:-}"
  shift || true
  local z_moniker="${1:-}"

  rbt_show "Routing command: ${z_command} with moniker: ${z_moniker}"

  # Verify BUD environment
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR not set - must be called from BUD"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP not set - must be called from BUD"

  case "${z_command}" in
    rbt-to)
      test -n "${z_moniker}" || buc_die "rbt-to requires moniker argument"
      rbt_load_nameplate "${z_moniker}"
      case "${z_moniker}" in
        nsproto) rbt_suite_nsproto ;;
        srjcl)   rbt_suite_srjcl ;;
        pluml)   rbt_suite_pluml ;;
        *)       buc_die "Unknown test suite: ${z_moniker}" ;;
      esac
      ;;
    *) buc_die "Unknown command: ${z_command}" ;;
  esac
}

rbt_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  rbt_route "${z_command}" "$@"
}

rbt_main "$@"

# eof
