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
  ZVOB_VVK_BIN_DIR="${BURC_TOOLS_DIR}/vvk/bin"
  ZVOB_RELEASE_DIR="${ZVOB_CARGO_DIR}/release"
  ZVOB_LEDGER_FILE="${ZVOB_CARGO_DIR}/vol_ledger.json"

  # Feature detection - build comma-separated list
  ZVOB_FEATURE_LIST=""

  # Detect jjk
  if test -f "${BURC_TOOLS_DIR}/jjk/vov_veiled/Cargo.toml"; then
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

  buc_step "Installing to VVK bin directory"

  local z_dest="${ZVOB_VVK_BIN_DIR}/vvx-${ZVOB_PLATFORM}"

  buc_log_args "Source: ${ZVOB_TARGET_BINARY}"
  buc_log_args "Destination: ${z_dest}"

  test -f "${ZVOB_TARGET_BINARY}" || buc_die "Binary not found: ${ZVOB_TARGET_BINARY}"
  test -d "${ZVOB_VVK_BIN_DIR}" || mkdir -p "${ZVOB_VVK_BIN_DIR}" || buc_die "Failed to create: ${ZVOB_VVK_BIN_DIR}"

  cp "${ZVOB_TARGET_BINARY}" "${z_dest}" || buc_die "Failed to copy binary"
  chmod +x "${z_dest}" || buc_die "Failed to chmod"

  # Ad-hoc codesign for macOS (prevents quarantine kills)
  if command -v codesign >/dev/null 2>&1; then
    codesign --force --sign - "${z_dest}" 2>/dev/null || buc_warn "codesign failed (non-fatal)"
  fi

  buc_success "Built and installed to ${z_dest}"
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

vob_parcel() {
  zvob_sentinel

  buc_doc_brief "Create VVK parcel: test, build, collect, brand, tarball"
  buc_doc_shown || return 0

  local z_staging="${BUD_TEMP_DIR}/staging"
  local z_install_script="${BURC_TOOLS_DIR}/vvk/vvi_install.sh"
  local z_registry="${BURC_TOOLS_DIR}/vok/vov_veiled/vovr_registry.json"
  local z_vvx="${ZVOB_VVK_BIN_DIR}/vvx-${ZVOB_PLATFORM}"

  buc_step "Running tests"
  buc_log_args "Cargo dir: ${ZVOB_CARGO_DIR}"
  buc_log_args "Features: ${ZVOB_FEATURE_LIST:-none}"

  cargo test --manifest-path "${ZVOB_CARGO_DIR}/Cargo.toml" --features "${ZVOB_FEATURE_LIST}" || buc_die "Tests failed"

  buc_step "Building binary"

  vob_build

  buc_step "Creating staging directory"
  buc_log_args "Staging: ${z_staging}"

  rm -rf "${z_staging}" || buc_die "Failed to clean staging"
  mkdir -p "${z_staging}" || buc_die "Failed to create staging"

  buc_step "Collecting assets"
  buc_log_args "Tools dir: ${BURC_TOOLS_DIR}"
  buc_log_args "Install script: ${z_install_script}"

  test -f "${z_install_script}" || buc_die "Install script not found: ${z_install_script}"
  test -f "${z_vvx}" || buc_die "VVX binary not found: ${z_vvx}"

  "${z_vvx}" release_collect \
    --staging "${z_staging}" \
    --tools-dir "${BURC_TOOLS_DIR}" \
    --install-script "${z_install_script}" || buc_die "release_collect failed"

  buc_step "Copying platform binary"

  local z_bin_dest="${z_staging}/kits/vvk/bin"
  mkdir -p "${z_bin_dest}" || buc_die "Failed to create bin dir"

  cp "${z_vvx}" "${z_bin_dest}/vvx-${ZVOB_PLATFORM}" || buc_die "Failed to copy binary"
  chmod +x "${z_bin_dest}/vvx-${ZVOB_PLATFORM}" || buc_die "Failed to chmod"

  buc_log_args "Binary: ${z_bin_dest}/vvx-${ZVOB_PLATFORM}"

  buc_step "Branding parcel"

  local z_commit
  z_commit=$(git rev-parse HEAD 2>/dev/null) || z_commit="unknown"

  buc_log_args "Registry: ${z_registry}"
  buc_log_args "Commit: ${z_commit}"

  local z_hallmark_file="${BUD_TEMP_DIR}/hallmark.txt"

  "${z_vvx}" release_brand \
    --staging "${z_staging}" \
    --registry "${z_registry}" \
    --commit "${z_commit}" > "${z_hallmark_file}" || buc_die "release_brand failed"

  local z_hallmark
  z_hallmark=$(<"${z_hallmark_file}")
  test -n "${z_hallmark}" || buc_die "Failed to read hallmark"

  buc_log_args "Hallmark: ${z_hallmark}"

  buc_step "Creating tarball"

  local z_parcel_name="vvk-parcel-${z_hallmark}"
  local z_project_root="${BURC_PROJECT_ROOT:-${PWD}}"
  local z_tarball="${z_project_root}/${z_parcel_name}.tar.gz"

  buc_log_args "Tarball: ${z_tarball}"

  tar -czf "${z_tarball}" -C "${z_staging}" . || buc_die "tar failed"

  buc_success "Parcel created: ${z_tarball}"
}

# eof
