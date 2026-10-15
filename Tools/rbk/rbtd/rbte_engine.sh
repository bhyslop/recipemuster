#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# Module providing kindle/sentinel/public functions for theurge
# Rust build, test, and orchestration commands.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZRBTE_SOURCED:-}" || return 0
ZRBTE_SOURCED=1

######################################################################
# Kindle

zrbte_kindle() {
  test -z "${ZRBTE_KINDLED:-}" || buc_die_now "rbte already kindled"

  local z_dir="${BASH_SOURCE[0]%/*}"

  # The theurge's two collars (rbmm_moorings/bkd_weaned/). Every cargo
  # invocation is the kennel's, through the door each collar is handed to: the
  # app converged by the heel, the suite launched by the mush.
  readonly ZRBTE_APP_COLLAR="rbtdn_app"
  readonly ZRBTE_SUITE_COLLAR="rbtdn_suite"

  # The kennel's doors, resolved by COLOPHON rather than by whole filename: the
  # frontispiece is a label the kit may re-word.
  local z_heel=("${BURD_TABTARGET_DIR}/bkw-h."*)
  test -x "${z_heel[0]}" || buc_die_now "rbte: no kennel heel door stands at ${BURD_TABTARGET_DIR}/bkw-h.*"
  readonly ZRBTE_HEEL_DOOR="${z_heel[0]}"
  local z_mush=("${BURD_TABTARGET_DIR}/bkw-m."*)
  test -x "${z_mush[0]}" || buc_die_now "rbte: no kennel mush door stands at ${BURD_TABTARGET_DIR}/bkw-m.*"
  readonly ZRBTE_MUSH_DOOR="${z_mush[0]}"

  # Where the heel installs the binary: the app collar's residence and byname.
  readonly ZRBTE_BINARY="${z_dir}/target/debug/rbtd"

  # Suite→fixture composition is owned by theurge (RBTDRC_SUITES in
  # rbtdrc_crucible.rs), not bash. Suite names survive here only as tabtarget
  # imprints passed straight through to the binary's `suite` mode.

  readonly ZRBTE_KINDLED=1
}

######################################################################
# Sentinel

zrbte_sentinel() {
  test "${ZRBTE_KINDLED:-}" = "1" || buc_die_now "Module rbte not kindled - call zrbte_kindle first"
}

######################################################################
# Internal helpers

# zrbte_codegen() - Refresh all zipper-derived artifacts before compiling.
# Write-on-change, so unchanged files keep their mtime and cargo skips rebuilds.
# The build is the sole producer — there is no standalone generate command.
zrbte_codegen() {
  zrbte_sentinel

  rbz_generate_context "${BURD_TABTARGET_DIR}" "${RBCC_tabtarget_context_file}" \
    || buc_die_now "Failed to generate tabtarget context"
  rbz_generate_consts "${RBCC_rbtdgc_consts_file}" \
    || buc_die_now "Failed to generate colophon consts"
}

# The strike, then the kennel's converge of the app collar. A strike that moved
# a committed file leaves the tree dirty, and the heel refuses it by the door law.
zrbte_build_binary() {
  zrbte_sentinel

  zrbte_codegen
  buc_step "Building theurge"
  "${ZRBTE_HEEL_DOOR}" "${ZRBTE_APP_COLLAR}" || buc_die_now "The kennel's heel of ${ZRBTE_APP_COLLAR} failed"
  test -x "${ZRBTE_BINARY}" || buc_die_now "Theurge binary not found: ${ZRBTE_BINARY}"
}

######################################################################
# Public functions

rbte_build() {
  zrbte_sentinel

  test "$#" -eq 0 || buc_die_now "The theurge build takes no argument; its shape is the collar ${ZRBTE_APP_COLLAR}'s"

  zrbte_build_binary
  buc_success "Theurge built"
}

# The strike, then the kennel's mush over the suite collar; any argument is the
# kennel's selection pattern, handed through untouched.
rbte_test() {
  zrbte_sentinel

  zrbte_codegen
  buc_step "Testing theurge"
  local z_mush_status=0
  "${ZRBTE_MUSH_DOOR}" "${ZRBTE_SUITE_COLLAR}" "$@" || z_mush_status=$?
  test "${z_mush_status}" -eq 0 || buc_die_now "The kennel's mush of ${ZRBTE_SUITE_COLLAR} failed (status ${z_mush_status})"
  buc_success "All theurge tests passed"
}

rbte_run() {
  zrbte_sentinel

  local z_fixture="${BUZ_FOLIO:-}"
  test -n "${z_fixture}" || buc_die_now "No fixture — pass one as the folio (e.g. rbw-tf.FixtureRun.sh tadmor)"

  zrbte_build_binary

  # Extra CLI args (e.g. --keep-going) pass straight through to the binary,
  # which owns flag parsing and the disposition policy gate.
  buc_step "Running theurge fixture '${z_fixture}'"
  local z_theurge_status=0
  "${ZRBTE_BINARY}" "${z_fixture}" "$@" || z_theurge_status=$?
  return "${z_theurge_status}"
}

rbte_suite() {
  zrbte_sentinel

  local z_suite="${BUZ_FOLIO:-}"
  test -n "${z_suite}" || buc_die_now "No suite imprint — use tabtarget with imprint (e.g. rbw-ts.TestSuite.reveille.sh)"

  zrbte_build_binary

  # Composition is owned by theurge: pass the suite imprint straight through to
  # the binary's `suite` mode, which resolves and runs its fixtures. Extra CLI
  # args (e.g. --keep-going) pass through with it.
  buc_step "Running theurge suite '${z_suite}'"
  local z_theurge_status=0
  "${ZRBTE_BINARY}" suite "${z_suite}" "$@" || z_theurge_status=$?
  return "${z_theurge_status}"
}

rbte_single() {
  zrbte_sentinel

  zrbte_build_binary

  local z_fixture="${BUZ_FOLIO:-}"
  local z_case="${1:-}"
  local z_theurge_status=0
  "${ZRBTE_BINARY}" single ${z_fixture:+"${z_fixture}"} ${z_case:+"${z_case}"} || z_theurge_status=$?
  return "${z_theurge_status}"
}

rbte_dowse() {
  zrbte_sentinel

  zrbte_build_binary

  # Read-only census over the station's self-logs. The log dir is a station
  # regime value (BURS_LOG_DIR), sourced from the launcher-exported station file
  # by the furnish for this command — the burs_cli pattern.
  test -n "${BURS_LOG_DIR:-}" || buc_die_now "BURS_LOG_DIR not set in ${BURD_STATION_FILE:-}"

  buc_step "Dowsing observed tariff history"
  local z_theurge_status=0
  "${ZRBTE_BINARY}" dowse "${BURS_LOG_DIR}" || z_theurge_status=$?
  return "${z_theurge_status}"
}

rbte_nihil() {
  zrbte_sentinel

  buc_step "Nihil: synthetic colophon for the calibrant census coverage cases"
  buc_success "Nihil: complete"
}

# eof
