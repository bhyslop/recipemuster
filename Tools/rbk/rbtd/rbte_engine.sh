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

  # Suite-to-fixture mappings
  ZRBTE_SUITE_FAST=(
    "enrollment-validation"
    "regime-validation"
    "regime-smoke"
    "handbook-render"
    "dockerfile-hygiene"
  )
  ZRBTE_SUITE_SERVICE=("${ZRBTE_SUITE_FAST[@]}" "access-probe" "hallmark-lifecycle" "batch-vouch")
  ZRBTE_SUITE_CRUCIBLE=("${ZRBTE_SUITE_FAST[@]}" "tadmor" "srjcl" "pluml")
  ZRBTE_SUITE_COMPLETE=("${ZRBTE_SUITE_FAST[@]}" "access-probe" "hallmark-lifecycle" "batch-vouch" "tadmor" "srjcl" "pluml")
  # Gauntlet — release-qualification ladder, walks marshal-zero state through
  # canonical-credentialed state to crucible verification. Pristine-lifecycle
  # case 1 is the entry-contract gate; preceding enrollment-validation runs
  # state-indifferent and is harmless on broken state. Fail-fast across
  # fixtures is provided by the for-loop's set -e.
  ZRBTE_SUITE_GAUNTLET=(
    "enrollment-validation"
    "pristine-lifecycle"
    "canonical-establish"
    "onboarding-sequence"
    "regime-validation"
    "regime-smoke"
    "dockerfile-hygiene"
    "hallmark-lifecycle"
    "tadmor"
    "moriah"
    "srjcl"
    "pluml"
  )
  # Skirmish — the "mini gauntlet": the depot->build->crucible chain WITHOUT
  # project-ID churn. canonical-invest reuses a standing operator-levied depot
  # (no levy, no unmake) where the gauntlet's pristine-lifecycle/canonical
  # -establish each levy a fresh project; pristine-lifecycle is dropped
  # entirely. onboarding-sequence then builds the crucible images (local kludge
  # + cloud ordain into the standing depot) and the four crucibles charge+run.
  # OPERATOR PRECONDITION: a canonical depot already levied — install canonical
  # prefixes and run rbw-dL by hand before this suite. Spends cloud build/GAR
  # but creates no GCP project per run.
  ZRBTE_SUITE_SKIRMISH=(
    "enrollment-validation"
    "canonical-invest"
    "onboarding-sequence"
    "regime-validation"
    "regime-smoke"
    "dockerfile-hygiene"
    "tadmor"
    "moriah"
    "srjcl"
    "pluml"
  )
  # Dogfight — standing-depot cloud-build viability probe. Sibling to skirmish
  # in the operator-precondition family (reuses a hand-levied depot, no levy,
  # no unmake) but charges NO crucible: it proves only the cloud-build → summon
  # → run path yields a runnable artifact. Single fixture; the suite name is
  # the operator's memorable batch handle. OPERATOR PRECONDITION: a canonical
  # depot already levied with a director + retriever invested (the same
  # standing-depot setup skirmish assumes).
  ZRBTE_SUITE_DOGFIGHT=(
    "dogfight"
  )

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
    gauntlet) echo "${ZRBTE_SUITE_GAUNTLET[*]}" ;;
    skirmish) echo "${ZRBTE_SUITE_SKIRMISH[*]}" ;;
    dogfight) echo "${ZRBTE_SUITE_DOGFIGHT[*]}" ;;
    *)        buc_die "Unknown suite: ${z_suite} (expected fast|service|crucible|complete|gauntlet|skirmish|dogfight)" ;;
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
  test -n "${z_fixture}" || buc_die "No fixture imprint — use tabtarget with imprint (e.g. rbtd-r.FixtureRun.tadmor.sh)"

  zrbte_build_binary

  buc_step "Running theurge fixture '${z_fixture}'"
  "${ZRBTE_BINARY}" "${ZRBZ_COLOPHON_MANIFEST}" "${z_fixture}"
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
    "${ZRBTE_BINARY}" "${ZRBZ_COLOPHON_MANIFEST}" "${z_fixture}"
    z_count=$((z_count + 1))
  done

  buc_success "Suite '${z_suite}' complete (${z_count} fixtures)"
}

rbte_single() {
  zrbte_sentinel

  zrbte_build_binary

  local z_fixture="${1:-}"
  local z_case="${2:-}"
  "${ZRBTE_BINARY}" single "${ZRBZ_COLOPHON_MANIFEST}" ${z_fixture:+"${z_fixture}"} ${z_case:+"${z_case}"}
}

# eof
