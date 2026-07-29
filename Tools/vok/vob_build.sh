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
# VOB - VOK Build Module
#
# BCG-compliant module for building and testing vvr/vvx Rust binaries.
# Uses VOF_VOK_FEATURES from vof_features.sh for kit feature flags.
# Uses VVB_BIN_DIR from vvb_bash.sh for binary installation location.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZVOB_SOURCED:-}" || buc_die "Module vob multiply sourced - check sourcing hierarchy"
ZVOB_SOURCED=1

######################################################################
# Internal Functions (zvob_*)

zvob_kindle() {
  test -z "${ZVOB_KINDLED:-}" || buc_die "Module vob already kindled"

  # Validate BUD environment
  zburd_sentinel

  # Validate BURC environment
  test -n "${BURC_TOOLS_DIR:-}" || buc_die "BURC_TOOLS_DIR is unset"
  test -n "${BURC_MANAGED_KITS:-}" || buc_die "BURC_MANAGED_KITS is unset"

  # Paths (VVB_BIN_DIR and VVB_PLATFORM come from vvb_bash.sh)
  readonly ZVOB_CARGO_DIR="${BURC_TOOLS_DIR}/vok"
  readonly ZVOB_TARGET_BINARY="${ZVOB_CARGO_DIR}/target/release/vvr"
  readonly ZVOB_RELEASE_DIR="${ZVOB_CARGO_DIR}/release"
  readonly ZVOB_LEDGER_FILE="${ZVOB_CARGO_DIR}/vol_ledger.json"

  readonly ZVOB_RELEASE_BINARY="${ZVOB_RELEASE_DIR}/${VVB_PLATFORM}/vvr"

  readonly ZVOB_KINDLED=1
}

zvob_sentinel() {
  test "${ZVOB_KINDLED:-}" = "1" || buc_die "Module vob not kindled - call zvob_kindle first"
}

zvob_hash_capture() {
  local z_file="$1"
  local z_hash
  z_hash=$(openssl dgst -sha256 -r "${z_file}") || return 1
  read -r z_hash _ <<< "${z_hash}"
  echo "${z_hash}"
}

######################################################################
# External Functions (vob_*)

vob_build() {
  zvob_sentinel

  buc_doc_brief "Build vvr binary and install to canonical location"
  buc_doc_shown || return 0

  # VOr_q4f: the shipped vvr binary must not link the matricula crate (vom).
  # Mechanical guard — assert vom is absent from vvr's resolved dependency
  # closure (feature-activated edges included); cargo tree resolves without
  # compiling, so this fails fast before the release build. Rationale: grep VOr_q4f.
  buc_step "Guarding vvr never links matricula (VOr_q4f)"
  if cargo tree --manifest-path "${ZVOB_CARGO_DIR}/Cargo.toml" --features "${VOF_VOK_FEATURES}" --prefix none 2>/dev/null \
       | grep -qE '^vom[[:space:]]'; then
    buc_die "VOr_q4f violated: shipped vvr links matricula crate 'vom' — it must never ship; sever the dependency"
  fi

  vof_clean

  buc_step "Building vvr binary"
  buc_log_args "Features: ${VOF_VOK_FEATURES:-none}"
  buc_log_args "Platform: ${VVB_PLATFORM}"

  cargo build --release --manifest-path "${ZVOB_CARGO_DIR}/Cargo.toml" --features "${VOF_VOK_FEATURES}" || buc_die "cargo build failed"

  buc_step "Installing to VVK bin directory"

  buc_log_args "Source: ${ZVOB_TARGET_BINARY}"
  buc_log_args "Destination: ${VVB_VVX_BINARY}"

  test -f "${ZVOB_TARGET_BINARY}" || buc_die "Binary not found: ${ZVOB_TARGET_BINARY}"
  test -d "${VVB_BIN_DIR}" || mkdir -p "${VVB_BIN_DIR}" || buc_die "Failed to create: ${VVB_BIN_DIR}"

  local z_tmp
  z_tmp="$(mktemp "${VVB_VVX_BINARY}.XXXXXX")" || buc_die "Failed to create temp file for atomic install"
  cp "${ZVOB_TARGET_BINARY}" "${z_tmp}" || buc_die "Failed to copy binary"
  chmod +x "${z_tmp}" || buc_die "Failed to chmod"

  # Ad-hoc codesign for macOS (prevents quarantine kills)
  if command -v codesign >/dev/null 2>&1; then
    codesign --force --sign - "${z_tmp}" 2>/dev/null || buc_warn "codesign failed (non-fatal)"
  fi

  # Atomic rename: repoints the directory entry without touching the inode a live
  # MCP server holds open, avoiding ETXTBSY on an in-place overwrite.
  mv -f "${z_tmp}" "${VVB_VVX_BINARY}" || buc_die "Failed to install binary"

  buc_success "Built and installed to ${VVB_VVX_BINARY}"
}

vob_test() {
  zvob_sentinel

  buc_doc_brief "Run tests for all detected kit manifests"
  buc_doc_shown || return 0

  vof_clean

  buc_step "Testing vok"
  buc_log_args "Manifest: ${VOF_VOK_MANIFEST}"
  buc_log_args "Features: ${VOF_VOK_FEATURES:-none}"

  cargo test --manifest-path "${VOF_VOK_MANIFEST}" --features "${VOF_VOK_FEATURES}" || buc_die "Tests failed: vok"

  local z_manifest=""
  for z_manifest in ${VOF_TEST_MANIFESTS}; do
    buc_step "Testing ${z_manifest##*/}"
    buc_log_args "Manifest: ${z_manifest}"

    cargo test --manifest-path "${z_manifest}" || buc_die "Tests failed: ${z_manifest}"
  done

  buc_success "All tests passed"
}

vob_clean() {
  zvob_sentinel

  buc_doc_brief "Remove all Rust build artifacts from kit target directories"
  buc_doc_shown || return 0

  local z_dirs=(
    "${BURC_TOOLS_DIR}/vok/target"
    "${BURC_TOOLS_DIR}/vok/vof/target"
    "${BURC_TOOLS_DIR}/vvc/target"
    "${BURC_TOOLS_DIR}/jjk/vov_veiled/target"
  )

  local z_total=0
  local z_dir=""
  for z_dir in "${z_dirs[@]}"; do
    if [ -d "${z_dir}" ]; then
      local z_size=""
      z_size=$(du -sm "${z_dir}" 2>/dev/null | awk '{print $1}') || z_size=0
      z_total=$((z_total + z_size))
      buc_step "Removing ${z_dir##*/} (${z_size}MB)"
      buc_log_args "Path: ${z_dir}"
      rm -rf "${z_dir}" || buc_die "Failed to remove: ${z_dir}"
    else
      buc_step "Skipping ${z_dir} (not found)"
    fi
  done

  buc_success "Cleaned ${z_total}MB of build artifacts"
}

# eof
