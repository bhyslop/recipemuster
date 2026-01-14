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
# BCG-compliant module for building vvr/vvx Rust binaries.
# Detects available kit features and builds with appropriate flags.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZVOB_SOURCED:-}" || buc_die "Module vob multiply sourced - check sourcing hierarchy"
ZVOB_SOURCED=1

######################################################################
# Internal Functions (zvob_*)

zvob_kindle() {
  test -z "${ZVOB_KINDLED:-}" || buc_die "Module vob already kindled"

  # Validate BUD environment
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR is unset"

  # Validate BURC environment
  test -n "${BURC_TOOLS_DIR:-}" || buc_die "BURC_TOOLS_DIR is unset"

  # Paths
  ZVOB_CARGO_DIR="${BURC_TOOLS_DIR}/vok"
  ZVOB_TARGET_BINARY="${ZVOB_CARGO_DIR}/target/release/vvr"
  ZVOB_CANONICAL_BINARY="${BURC_TOOLS_DIR}/vvk/bin/vvx"
  ZVOB_RELEASE_DIR="${ZVOB_CARGO_DIR}/release"
  ZVOB_LEDGER_FILE="${ZVOB_CARGO_DIR}/vol_ledger.json"

  # Feature detection - build comma-separated list
  ZVOB_FEATURE_LIST=""

  # Detect jjk
  if test -f "${BURC_TOOLS_DIR}/jjk/veiled/Cargo.toml"; then
    ZVOB_FEATURE_LIST="${ZVOB_FEATURE_LIST:+${ZVOB_FEATURE_LIST},}jjk"
  fi

  # Platform detection
  local z_os
  local z_arch
  z_os="$(uname -s)" || buc_die "Failed to detect OS"
  z_arch="$(uname -m)" || buc_die "Failed to detect architecture"

  case "${z_os}-${z_arch}" in
    Darwin-arm64)   ZVOB_PLATFORM="darwin-arm64" ;;
    Darwin-x86_64)  ZVOB_PLATFORM="darwin-x86_64" ;;
    Linux-x86_64)   ZVOB_PLATFORM="linux-x86_64" ;;
    Linux-aarch64)  ZVOB_PLATFORM="linux-aarch64" ;;
    MINGW*-x86_64|MSYS*-x86_64) ZVOB_PLATFORM="windows-x86_64" ;;
    *)              buc_die "Unsupported platform: ${z_os}-${z_arch}" ;;
  esac

  ZVOB_RELEASE_BINARY="${ZVOB_RELEASE_DIR}/${ZVOB_PLATFORM}/vvr"

  ZVOB_KINDLED=1
}

zvob_sentinel() {
  test "${ZVOB_KINDLED:-}" = "1" || buc_die "Module vob not kindled - call zvob_kindle first"
}

######################################################################
# External Functions (vob_*)

vob_build() {
  zvob_sentinel

  buc_doc_brief "Build vvr binary and install to canonical location"
  buc_doc_shown || return 0

  buc_step "Building vvr binary"
  buc_log_args "Cargo dir: ${ZVOB_CARGO_DIR}"
  buc_log_args "Features: ${ZVOB_FEATURE_LIST:-none}"
  buc_log_args "Platform: ${ZVOB_PLATFORM}"

  cargo build --release --manifest-path "${ZVOB_CARGO_DIR}/Cargo.toml" --features "${ZVOB_FEATURE_LIST}" || buc_die "cargo build failed"

  buc_step "Installing to canonical location"
  buc_log_args "Source: ${ZVOB_TARGET_BINARY}"
  buc_log_args "Destination: ${ZVOB_CANONICAL_BINARY}"

  test -f "${ZVOB_TARGET_BINARY}" || buc_die "Binary not found: ${ZVOB_TARGET_BINARY}"

  cp "${ZVOB_TARGET_BINARY}" "${ZVOB_CANONICAL_BINARY}" || buc_die "Failed to copy binary"
  chmod +x "${ZVOB_CANONICAL_BINARY}" || buc_die "Failed to chmod"

  buc_success "Built and installed to ${ZVOB_CANONICAL_BINARY}"
}

vob_release() {
  zvob_sentinel

  buc_doc_brief "Run tests, copy to release dir, update ledger"
  buc_doc_shown || return 0

  buc_step "Running tests"
  buc_log_args "Cargo dir: ${ZVOB_CARGO_DIR}"

  cargo test --manifest-path "${ZVOB_CARGO_DIR}/Cargo.toml" --features "${ZVOB_FEATURE_LIST}" || buc_die "Tests failed"

  buc_step "Copying to release directory"
  buc_log_args "Source: ${ZVOB_TARGET_BINARY}"
  buc_log_args "Destination: ${ZVOB_RELEASE_BINARY}"

  test -f "${ZVOB_TARGET_BINARY}" || buc_die "Binary not found - run vob_build first"
  test -d "${ZVOB_RELEASE_DIR}/${ZVOB_PLATFORM}" || buc_die "Release dir not found: ${ZVOB_RELEASE_DIR}/${ZVOB_PLATFORM}"

  cp "${ZVOB_TARGET_BINARY}" "${ZVOB_RELEASE_BINARY}" || buc_die "Failed to copy binary"
  chmod +x "${ZVOB_RELEASE_BINARY}" || buc_die "Failed to chmod"

  buc_step "Updating ledger"

  local z_hash
  if command -v shasum >/dev/null 2>&1; then
    z_hash=$(shasum -a 256 "${ZVOB_RELEASE_BINARY}" | cut -d' ' -f1) || buc_die "Failed to compute hash"
  elif command -v sha256sum >/dev/null 2>&1; then
    z_hash=$(sha256sum "${ZVOB_RELEASE_BINARY}" | cut -d' ' -f1) || buc_die "Failed to compute hash"
  else
    buc_die "No sha256 tool available"
  fi

  local z_date
  local z_commit
  z_date=$(date +%Y-%m-%d) || buc_die "Failed to get date"
  z_commit=$(git rev-parse --short HEAD 2>/dev/null) || z_commit="unknown"

  buc_log_args "Hash: ${z_hash}"
  buc_log_args "Date: ${z_date}"
  buc_log_args "Commit: ${z_commit}"

  test -f "${ZVOB_LEDGER_FILE}" || buc_die "Ledger not found: ${ZVOB_LEDGER_FILE}"

  local z_ledger_tmp="${BUD_TEMP_DIR}/vob_ledger_update.json"

  jq --arg date "${z_date}" \
     --arg platform "${ZVOB_PLATFORM}" \
     --arg hash "${z_hash}" \
     --arg commit "${z_commit}" \
     '.releases += [{"date": $date, "platform": $platform, "hash": $hash, "commit": $commit}]' \
     "${ZVOB_LEDGER_FILE}" > "${z_ledger_tmp}" || buc_die "jq failed"

  mv "${z_ledger_tmp}" "${ZVOB_LEDGER_FILE}" || buc_die "Failed to update ledger"

  buc_success "Release complete: ${ZVOB_RELEASE_BINARY}"
}

# eof
