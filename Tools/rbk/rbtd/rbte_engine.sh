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

  # EVERY CARGO INVOCATION IS SPELLED HERE, by the crate's manifest, locked,
  # under the channel the repository's root pin states. These doors carry no
  # stale-binary gate and refuse no uncommitted tree: cargo's own fingerprint
  # answers currency, and every run door builds first.
  readonly ZRBTE_REPO_ROOT="${PWD}"
  readonly ZRBTE_MANIFEST="${z_dir}/Cargo.toml"
  readonly ZRBTE_PIN_FILE="${ZRBTE_REPO_ROOT}/rust-toolchain.toml"

  # The position's roots: the crate's own source and manifests and the root pin,
  # its cfg(test) modules carved out. The position is the seat's newest
  # first-parent commit touching them: the seat's own line, not a landed walk,
  # so a billet carrying unlanded commits states where it actually stands.
  ZRBTE_ROOTS=(
    "Tools/rbk/rbtd/src"
    "Tools/rbk/rbtd/build.rs"
    "Tools/rbk/rbtd/Cargo.toml"
    "Tools/rbk/rbtd/Cargo.lock"
    "rust-toolchain.toml"
    ":(exclude)Tools/rbk/rbtd/src/rbtdt*"
  )
  readonly ZRBTE_ROOTS

  # The lint's whole ask, every lint capped at warning and the one barred-call
  # lint raised to be read. The verdict is read from that lint's help-URL mark
  # in the output, never the exit code, which the cap silences.
  ZRBTE_CLIPPY_ASKED=(
    --all-targets
    --color never
    --
    --cap-lints warn
    -W clippy::disallowed_methods
  )
  readonly ZRBTE_CLIPPY_ASKED
  readonly ZRBTE_CLIPPY_MARK="index.html#disallowed_methods"
  readonly ZRBTE_LINT_REPORT="${BURD_TEMP_DIR}/rbte_lint_report.txt"
  readonly ZRBTE_CHANNEL_PATTERN='^channel *= *"([^"]+)"'

  # Where cargo's dev profile lands the binary, on either road.
  readonly ZRBTE_BINARY="${z_dir}/target/debug/rbt"

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

# The channel the root pin states, stated on every invocation.
zrbte_channel_capture() {
  zrbte_sentinel

  local z_channel=""
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    if [[ "${z_line}" =~ ${ZRBTE_CHANNEL_PATTERN} ]]; then
      z_channel="${BASH_REMATCH[1]}"
      break
    fi
  done < "${ZRBTE_PIN_FILE}" || return 1
  test -n "${z_channel}" || return 1
  echo "${z_channel}" || return 1
}

# The seat's position over the roots, the one line the binary will answer.
zrbte_position_capture() {
  zrbte_sentinel

  local z_position=""
  z_position=$(git -C "${ZRBTE_REPO_ROOT}" log -n 1 --first-parent --format=%H HEAD -- "${ZRBTE_ROOTS[@]}") || return 1
  test -n "${z_position}" || return 1
  echo "${z_position}" || return 1
}

# Run cargo for the theurge: the pinned channel, the lock flag ahead of the
# verb, the position stated, the manifest named after the verb.
#
# THE POSITION'S NAME IS THE BUILD SCRIPT'S CONTRACT, not a choice made here,
# and it is the binary's answer to --exergue. Cargo reruns the script when the
# value changes, so every road building into this target directory must state
# the same position under the same definition, or each rebuilds what the other
# left.
zrbte_cargo() {
  zrbte_sentinel

  local -r z_verb="${1:-}"
  test -n "${z_verb}" || buc_die_now "zrbte_cargo needs a cargo verb"
  shift || buc_die_now "zrbte_cargo could not shift its verb"

  local z_channel=""
  z_channel=$(zrbte_channel_capture) || buc_die_now "The root pin at ${ZRBTE_PIN_FILE} names no channel"

  local z_position=""
  z_position=$(zrbte_position_capture) || buc_die_now "Could not read the seat's position over the theurge's roots in ${ZRBTE_REPO_ROOT}"

  local z_cargo_status=0
  BKCH_COLLAR_POSITION="${z_position}" \
    cargo "+${z_channel}" --locked "${z_verb}" --manifest-path "${ZRBTE_MANIFEST}" "$@" \
    || z_cargo_status=$?
  return "${z_cargo_status}"
}

# The codegen, then cargo's build. Cargo decides whether anything recompiles.
zrbte_build_binary() {
  zrbte_sentinel

  zrbte_codegen
  buc_step "Building theurge"
  zrbte_cargo build || buc_die_now "The theurge did not build"
  test -x "${ZRBTE_BINARY}" || buc_die_now "Theurge binary not found: ${ZRBTE_BINARY}"
}

######################################################################
# Public functions

rbte_build() {
  zrbte_sentinel

  test "$#" -eq 0 || buc_die_now "The theurge build takes no argument"

  zrbte_build_binary
  buc_success "Theurge built"
}

# The codegen, then cargo's test; the one optional argument is cargo's own
# test-name filter.
rbte_test() {
  zrbte_sentinel

  test "$#" -le 1 || buc_die_now "The theurge test takes at most one argument, a test-name filter"
  local -r z_filter="${1:-}"

  zrbte_codegen
  buc_step "Testing theurge"
  zrbte_cargo test ${z_filter:+"${z_filter}"} || buc_die_now "Theurge tests failed"
  buc_success "All theurge tests passed"
}

# Lints the test engine's crate with every lint held to a warning, refusing only on a call the repository's lint list bars.
rbte_lint() {
  zrbte_sentinel

  buc_doc_brief "Lints the test engine's crate with every lint held to a warning, refusing only on a call the repository's lint list bars."
  buc_doc_shown || return 0

  test "$#" -eq 0 || buc_die_now "The theurge lint takes no argument"

  zrbte_codegen
  buc_step "Linting theurge"
  local z_lint_status=0
  CLIPPY_CONF_DIR="${ZRBTE_REPO_ROOT}" zrbte_cargo clippy "${ZRBTE_CLIPPY_ASKED[@]}" \
    > "${ZRBTE_LINT_REPORT}" 2>&1 || z_lint_status=$?

  local z_report=""
  z_report=$(<"${ZRBTE_LINT_REPORT}") || buc_die_now "Could not read the lint report at ${ZRBTE_LINT_REPORT}"
  printf '%s\n' "${z_report}" || buc_die_now "Could not print the lint report"

  [[ "${z_report}" != *"${ZRBTE_CLIPPY_MARK}"* ]] \
    || buc_die_now "The theurge calls what the root clippy.toml bars; each finding above names the surface to adopt"
  test "${z_lint_status}" -eq 0 || buc_die_now "The theurge did not compile under clippy (status ${z_lint_status})"
  buc_success "Theurge lint found no barred call"
}

# Removes the test engine's build output, so the next door that builds starts from nothing.
rbte_clean() {
  zrbte_sentinel

  buc_doc_brief "Removes the test engine's build output, so the next door that builds starts from nothing."
  buc_doc_shown || return 0

  test "$#" -eq 0 || buc_die_now "The theurge clean takes no argument"

  buc_step "Cleaning theurge"
  zrbte_cargo clean || buc_die_now "The theurge clean failed"
  buc_success "Theurge build output removed"
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
