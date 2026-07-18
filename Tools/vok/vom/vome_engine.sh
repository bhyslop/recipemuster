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
# VOME Engine - Matricula (vom) build/test/run implementation module
#
# BCG module: kindle/sentinel + public vome_build/test/run over the vom crate's
# own manifest. The vom crate never ships and is never folded into the vvr build
# (VOr_q4f), so this engine drives its own crate-local target/.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZVOME_SOURCED:-}" || return 0
ZVOME_SOURCED=1

######################################################################
# Kindle

zvome_kindle() {
  test -z "${ZVOME_KINDLED:-}" || buc_die "vome already kindled"

  local z_dir="${BASH_SOURCE[0]%/*}"

  readonly VOME_MANIFEST="${z_dir}/Cargo.toml"

  # Windows-native cargo needs a drive-letter path, not a /cygdrive one; the vom
  # crate is run from the mac station for now, so that translation (see
  # rbte_engine.sh zrbte_kindle) is intentionally omitted until a Cygwin
  # matricula run is needed.
  readonly VOME_MANIFEST_ARG="${VOME_MANIFEST}"

  readonly ZVOME_BINARY="${z_dir}/target/debug/vom"

  readonly ZVOME_KINDLED=1
}

######################################################################
# Sentinel

zvome_sentinel() {
  test "${ZVOME_KINDLED:-}" = "1" || buc_die "Module vome not kindled - call zvome_kindle first"
}

######################################################################
# Internal helpers

zvome_build_binary() {
  zvome_sentinel

  buc_step "Building matricula"
  cargo build --manifest-path "${VOME_MANIFEST_ARG}" || buc_die "cargo build failed"
  test -x "${ZVOME_BINARY}" || buc_die "Matricula binary not found: ${ZVOME_BINARY}"
}

######################################################################
# Public functions

vome_build() {
  zvome_sentinel

  buc_step "Building matricula"
  buc_log_args "Manifest: ${VOME_MANIFEST}"
  cargo build --manifest-path "${VOME_MANIFEST_ARG}" "$@" || buc_die "cargo build failed"
  buc_success "Matricula built"
}

vome_test() {
  zvome_sentinel

  buc_step "Testing matricula"
  buc_log_args "Manifest: ${VOME_MANIFEST}"
  cargo test --manifest-path "${VOME_MANIFEST_ARG}" "$@" || buc_die "cargo test failed"
  buc_success "All matricula tests passed"
}

vome_run() {
  zvome_sentinel

  zvome_build_binary

  buc_step "Running matricula (degenerate)"
  "${ZVOME_BINARY}" "$@"
}

# eof
