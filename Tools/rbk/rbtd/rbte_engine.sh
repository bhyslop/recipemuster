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
# RBTE Engine - Theurge test engine implementation module
#
# BCG module providing kindle/sentinel/public functions for theurge
# Rust build, test, and orchestration commands.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZRBTE_SOURCED:-}" || return 0
ZRBTE_SOURCED=1

######################################################################
# Kindle

zrbte_kindle() {
  test -z "${ZRBTE_KINDLED:-}" || buc_die "rbte already kindled"

  local z_dir="${BASH_SOURCE[0]%/*}"

  readonly RBTE_MANIFEST="${z_dir}/Cargo.toml"
  readonly ZRBTE_BINARY="${z_dir}/target/debug/rbtd"

  # Theurge-own colophons (routed by rbte_dispatch, not the RBK zipper)
  readonly ZRBTE_COLOPHONS="rbtd-ap"

  # Combined manifest: RBK zipper colophons + theurge-own colophons
  readonly ZRBTE_FULL_MANIFEST="${ZRBZ_COLOPHON_MANIFEST} ${ZRBTE_COLOPHONS}"

  # Suite-to-fixture mappings
  ZRBTE_SUITE_FAST=("enrollment-validation" "regime-validation" "regime-smoke")
  ZRBTE_SUITE_SERVICE=("${ZRBTE_SUITE_FAST[@]}" "access-probe" "four-mode")
  ZRBTE_SUITE_CRUCIBLE=("${ZRBTE_SUITE_FAST[@]}" "tadmor" "srjcl" "pluml")
  ZRBTE_SUITE_COMPLETE=("${ZRBTE_SUITE_FAST[@]}" "access-probe" "four-mode" "tadmor" "srjcl" "pluml")

  readonly ZRBTE_KINDLED=1
}

######################################################################
# Sentinel

zrbte_sentinel() {
  test "${ZRBTE_KINDLED:-}" = "1" || buc_die "Module rbte not kindled - call zrbte_kindle first"
}

######################################################################
# Internal helpers

zrbte_build_binary() {
  zrbte_sentinel

  buc_step "Building theurge"
  cargo build --manifest-path "${RBTE_MANIFEST}" || buc_die "cargo build failed"
  test -x "${ZRBTE_BINARY}" || buc_die "Theurge binary not found: ${ZRBTE_BINARY}"
}

zrbte_resolve_suite() {
  local z_suite="$1"
  case "${z_suite}" in
    fast)     echo "${ZRBTE_SUITE_FAST[*]}" ;;
    service)  echo "${ZRBTE_SUITE_SERVICE[*]}" ;;
    crucible) echo "${ZRBTE_SUITE_CRUCIBLE[*]}" ;;
    complete) echo "${ZRBTE_SUITE_COMPLETE[*]}" ;;
    *)        buc_die "Unknown suite: ${z_suite} (expected fast|service|crucible|complete)" ;;
  esac
}

######################################################################
# Public functions

rbte_build() {
  zrbte_sentinel

  buc_step "Building theurge"
  buc_log_args "Manifest: ${RBTE_MANIFEST}"
  cargo build --manifest-path "${RBTE_MANIFEST}" "$@" || buc_die "cargo build failed"
  buc_success "Theurge built"
}

rbte_test() {
  zrbte_sentinel

  buc_step "Testing theurge"
  buc_log_args "Manifest: ${RBTE_MANIFEST}"
  cargo test --manifest-path "${RBTE_MANIFEST}" "$@" || buc_die "cargo test failed"
  buc_success "All theurge tests passed"
}

rbte_run() {
  zrbte_sentinel

  local z_fixture="${BURD_TOKEN_3:-}"
  test -n "${z_fixture}" || buc_die "No fixture imprint — use tabtarget with imprint (e.g. rbtd-r.Run.tadmor.sh)"

  zrbte_build_binary

  buc_step "Running theurge fixture '${z_fixture}'"
  "${ZRBTE_BINARY}" "${ZRBTE_FULL_MANIFEST}" "${z_fixture}"
}

rbte_suite() {
  zrbte_sentinel

  local z_suite="${BURD_TOKEN_3:-}"
  test -n "${z_suite}" || buc_die "No suite imprint — use tabtarget with imprint (e.g. rbtd-s.TestSuite.fast.sh)"

  local z_fixture_list
  z_fixture_list="$(zrbte_resolve_suite "${z_suite}")"

  zrbte_build_binary

  local z_count=0
  local z_fixture
  for z_fixture in ${z_fixture_list}; do
    buc_step "Running theurge fixture '${z_fixture}'"
    "${ZRBTE_BINARY}" "${ZRBTE_FULL_MANIFEST}" "${z_fixture}"
    z_count=$((z_count + 1))
  done

  buc_success "Suite '${z_suite}' complete (${z_count} fixtures)"
}

rbte_single() {
  zrbte_sentinel

  local z_fixture="${BURD_TOKEN_3:-}"
  test -n "${z_fixture}" || buc_die "No fixture imprint — use tabtarget with imprint (e.g. rbtd-s.SingleCase.tadmor.sh)"

  zrbte_build_binary

  local z_case="${1:-}"
  "${ZRBTE_BINARY}" single "${ZRBTE_FULL_MANIFEST}" "${z_fixture}" ${z_case:+"${z_case}"}
}

rbte_probe() {
  zrbte_sentinel

  local z_role="${BURD_TOKEN_3:-}"
  test -n "${z_role}" || buc_die "No role imprint — use tabtarget with imprint (e.g. rbtd-ap.AccessProbe.governor.sh)"

  local z_iterations=5
  local z_delay_ms=1500

  case "${z_role}" in
    governor|director|retriever)
      buc_step "JWT SA access probe: ${z_role}"
      rbap_jwt_sa_probe "${z_role}" "${z_iterations}" "${z_delay_ms}"
      buc_success "${z_role} JWT access probe passed"
      ;;
    payor)
      buc_step "Payor OAuth access probe"
      source "${RBBC_rbrp_file}" || buc_die "Failed to source RBRP: ${RBBC_rbrp_file}"
      zrbrp_kindle
      zrbrp_enforce
      rbap_payor_oauth_probe "${z_iterations}" "${z_delay_ms}"
      buc_success "Payor OAuth access probe passed"
      ;;
    *)
      buc_die "Unknown access-probe role: ${z_role} (expected governor|director|retriever|payor)"
      ;;
  esac
}

# eof
