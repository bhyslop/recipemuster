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
# VOP Prepare Release - Build and package vvr binary for current platform

set -euo pipefail

# Capture script directory at source time
ZVOP_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source BUC for messaging
source "${ZVOP_SCRIPT_DIR}/../buk/buc_command.sh"

######################################################################
# Internal Functions (zvop_*)

zvop_kindle() {
  test -z "${ZVOP_KINDLED:-}" || buc_die "Module vop already kindled"

  # Paths
  ZVOP_CARGO_DIR="${ZVOP_SCRIPT_DIR}"
  ZVOP_RELEASE_DIR="${ZVOP_SCRIPT_DIR}/release"
  ZVOP_LEDGER_FILE="${ZVOP_SCRIPT_DIR}/vol_ledger.json"
  ZVOP_TARGET_BINARY="${ZVOP_CARGO_DIR}/target/release/vvr"

  # Temp files for intermediate results
  ZVOP_TEMP_DIR="${TMPDIR:-/tmp}/vop_$$"
  mkdir -p "${ZVOP_TEMP_DIR}" || buc_die "Failed to create temp dir"
  ZVOP_HASH_FILE="${ZVOP_TEMP_DIR}/hash.txt"
  ZVOP_LEDGER_TMP="${ZVOP_TEMP_DIR}/ledger.json"

  ZVOP_KINDLED=1
}

zvop_sentinel() {
  test "${ZVOP_KINDLED:-}" = "1" || buc_die "Module vop not kindled"
}

zvop_cleanup() {
  test -d "${ZVOP_TEMP_DIR:-}" && rm -rf "${ZVOP_TEMP_DIR}"
}

zvop_detect_platform_capture() {
  local z_os
  local z_arch

  z_os="$(uname -s)" || return 1
  z_arch="$(uname -m)" || return 1

  case "${z_os}-${z_arch}" in
    Darwin-arm64)   echo "darwin-arm64" ;;
    Darwin-x86_64)  echo "darwin-x86_64" ;;
    Linux-x86_64)   echo "linux-x86_64" ;;
    Linux-aarch64)  echo "linux-aarch64" ;;
    MINGW*-x86_64|MSYS*-x86_64) echo "windows-x86_64" ;;
    *)              return 1 ;;
  esac
}

zvop_build_release() {
  zvop_sentinel

  buc_step "Building release binary"

  cd "${ZVOP_CARGO_DIR}" || buc_die "Cannot cd to cargo dir"

  cargo build --release || buc_die "cargo build failed"

  buc_log_args "Build complete"
}

zvop_run_tests() {
  zvop_sentinel

  buc_step "Running tests"

  cd "${ZVOP_CARGO_DIR}" || buc_die "Cannot cd to cargo dir"

  cargo test || buc_die "Tests failed"

  buc_log_args "Tests passed"
}

zvop_copy_binary() {
  zvop_sentinel

  local z_platform="${1:-}"

  test -n "${z_platform}" || buc_die "Platform required"

  local z_dest_dir="${ZVOP_RELEASE_DIR}/${z_platform}"
  local z_dest="${z_dest_dir}/vvr"

  test -f "${ZVOP_TARGET_BINARY}" || buc_die "Binary not found: ${ZVOP_TARGET_BINARY}"
  test -d "${z_dest_dir}" || buc_die "Release dir not found: ${z_dest_dir}"

  buc_step "Installing binary to ${z_dest}"

  cp "${ZVOP_TARGET_BINARY}" "${z_dest}" || buc_die "Copy failed"
  chmod +x "${z_dest}" || buc_die "chmod failed"

  buc_log_args "Binary installed"
}

zvop_compute_hash_capture() {
  zvop_sentinel

  local z_platform="${1:-}"
  local z_binary="${ZVOP_RELEASE_DIR}/${z_platform}/vvr"

  test -f "${z_binary}" || return 1

  # Use shasum on macOS, sha256sum on Linux
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "${z_binary}" | cut -d' ' -f1
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "${z_binary}" | cut -d' ' -f1
  else
    return 1
  fi
}

zvop_update_ledger() {
  zvop_sentinel

  local z_platform="${1:-}"
  local z_hash="${2:-}"

  test -n "${z_platform}" || buc_die "Platform required"
  test -n "${z_hash}" || buc_die "Hash required"
  test -f "${ZVOP_LEDGER_FILE}" || buc_die "Ledger not found: ${ZVOP_LEDGER_FILE}"

  buc_step "Updating ledger"

  local z_date
  local z_commit
  z_date=$(date +%Y-%m-%d) || buc_die "Failed to get date"
  z_commit=$(git rev-parse --short HEAD 2>/dev/null) || z_commit="unknown"

  # Use jq to append entry - write to temp file first
  jq --arg date "${z_date}" \
     --arg platform "${z_platform}" \
     --arg hash "${z_hash}" \
     --arg commit "${z_commit}" \
     '.releases += [{"date": $date, "platform": $platform, "hash": $hash, "commit": $commit}]' \
     "${ZVOP_LEDGER_FILE}" > "${ZVOP_LEDGER_TMP}" || buc_die "jq failed"

  mv "${ZVOP_LEDGER_TMP}" "${ZVOP_LEDGER_FILE}" || buc_die "Failed to update ledger"

  buc_log_args "Ledger updated: ${z_date} ${z_platform} ${z_commit}"
}

######################################################################
# Public Functions (vop_*)

vop_release() {
  zvop_kindle

  buc_step "=== VOK Prepare Release ==="

  local z_platform
  z_platform=$(zvop_detect_platform_capture) || buc_die "Unsupported platform: $(uname -s)-$(uname -m)"
  buc_log_args "Platform: ${z_platform}"

  zvop_build_release
  zvop_run_tests
  zvop_copy_binary "${z_platform}"

  local z_hash
  z_hash=$(zvop_compute_hash_capture "${z_platform}") || buc_die "Failed to compute hash"
  buc_log_args "Hash: ${z_hash}"

  zvop_update_ledger "${z_platform}" "${z_hash}"

  buc_step "=== Release Complete ==="
  buc_log_args "Binary: ${ZVOP_RELEASE_DIR}/${z_platform}/vvr"
  buc_log_args "Ledger: ${ZVOP_LEDGER_FILE}"

  zvop_cleanup
}

# Run if executed directly
if test "${BASH_SOURCE[0]}" = "${0}"; then
  vop_release "$@"
fi

# eof
