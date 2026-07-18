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
# RBTHE CLI - the hierophant's thin bash front. Builds the veiled rbthd crate
# and runs its subcommands. All ceremony logic is Rust (RBSHC "as the theurge
# conducts the tests"); this file only builds and launches the binary.
#
# VEILED, withheld from delivery. Dispatched by the hierophant's own veiled
# workbench (rbthw_workbench.sh) via its own zipper (rbthz_zipper.sh) — never the
# shipped rbw workbench, so no shipped manifest references the veiled crate.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"
source "${BURD_BUK_DIR}/buym_yelp.sh"

######################################################################
# Build helper

# Build the crate and resolve its binary. The crate is standalone (its own
# Cargo.toml, no workspace), built by manifest path. No codegen: essai reaches
# its workers by tabtarget, needing no generated colophon constants.
ZRBTHE_BINARY=""
zrbthe_cargo_build() {
  local z_dir="${BASH_SOURCE[0]%/*}"
  local z_manifest="${z_dir}/Cargo.toml"
  buc_step "Building hierophant"
  buc_log_args "Manifest: ${z_manifest}"
  cargo build --manifest-path "${z_manifest}" || buc_die "cargo build failed"
  ZRBTHE_BINARY="${z_dir}/target/debug/rbthd"
  test -x "${ZRBTHE_BINARY}" || buc_die "Hierophant binary not found: ${ZRBTHE_BINARY}"
}

######################################################################
# Commands

rbthe_build() {
  buc_doc_brief "Build the hierophant crate"
  buc_doc_shown || return 0

  zrbthe_cargo_build
  buc_success "Hierophant built"
}

rbthe_essai() {
  buc_doc_brief "Essai — the reversible repair lap (gate, cut, prove, rig; zero remote acts)"
  buc_doc_shown || return 0

  zrbthe_cargo_build
  buc_step "Conducting essai"
  "${ZRBTHE_BINARY}" essai
}

rbthe_docimasy() {
  buc_doc_brief "Docimasy — the reveal's reversible proving act (quarantine gate, freshness, preview, credential preflight, gauntlet; grants the cachet)"
  buc_doc_oparm "rehearse" "Prove the reversible stages only: skip credential preflight, the gauntlet, and the cachet grant"
  buc_doc_shown || return 0

  zrbthe_cargo_build
  buc_step "Conducting docimasy"
  "${ZRBTHE_BINARY}" docimasy "$@"
}

rbthe_ostend() {
  buc_doc_brief "Ostend — the reveal's irreversible showing (re-asserted ground, file-list eyes, disclosure, promotion)"
  buc_doc_oparm "rehearse" "Prove the reversible stages only: stop before the disclosure line"
  buc_doc_shown || return 0

  zrbthe_cargo_build
  buc_step "Conducting ostend"
  "${ZRBTHE_BINARY}" ostend "$@"
}

rbthe_harbinger() {
  buc_doc_brief "Harbinger — the stranger rig against promoted public main (zero remote acts)"
  buc_doc_shown || return 0

  zrbthe_cargo_build
  buc_step "Conducting harbinger"
  "${ZRBTHE_BINARY}" harbinger
}

rbthe_test() {
  buc_doc_brief "Run the hierophant crate tests — the cut's self-proofs (totality, planted-leak sweep)"
  buc_doc_shown || return 0

  local z_dir="${BASH_SOURCE[0]%/*}"
  local z_manifest="${z_dir}/Cargo.toml"
  buc_step "Testing hierophant"
  cargo test --manifest-path "${z_manifest}" || buc_die "cargo test failed"
  buc_success "Hierophant tests green"
}

######################################################################
# Furnish and Main

zrbthe_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temp directory (dispatch-provided)"
  buc_doc_env_done || return 0
}

buc_execute rbthe_ "Hierophant ceremony conductor" zrbthe_furnish "$@"

# eof
