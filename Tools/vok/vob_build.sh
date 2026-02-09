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
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR is unset"

  # Validate BURC environment
  test -n "${BURC_TOOLS_DIR:-}" || buc_die "BURC_TOOLS_DIR is unset"
  test -n "${BURC_MANAGED_KITS:-}" || buc_die "BURC_MANAGED_KITS is unset"

  # Paths (VVB_BIN_DIR and VVB_PLATFORM come from vvb_bash.sh)
  ZVOB_CARGO_DIR="${BURC_TOOLS_DIR}/vok"
  ZVOB_TARGET_BINARY="${ZVOB_CARGO_DIR}/target/release/vvr"
  ZVOB_RELEASE_DIR="${ZVOB_CARGO_DIR}/release"
  ZVOB_LEDGER_FILE="${ZVOB_CARGO_DIR}/vol_ledger.json"

  ZVOB_RELEASE_BINARY="${ZVOB_RELEASE_DIR}/${VVB_PLATFORM}/vvr"

  ZVOB_KINDLED=1
}

zvob_sentinel() {
  test "${ZVOB_KINDLED:-}" = "1" || buc_die "Module vob not kindled - call zvob_kindle first"
}

zvob_hash_capture() {
  local z_file="$1"

  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "${z_file}" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "${z_file}" | awk '{print $1}'
  else
    return 1
  fi
}

zvob_commit_capture() {
  git rev-parse HEAD 2>/dev/null || echo "unknown"
}

######################################################################
# External Functions (vob_*)

vob_build() {
  zvob_sentinel

  buc_doc_brief "Build vvr binary and install to canonical location"
  buc_doc_shown || return 0

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

  cp "${ZVOB_TARGET_BINARY}" "${VVB_VVX_BINARY}" || buc_die "Failed to copy binary"
  chmod +x "${VVB_VVX_BINARY}" || buc_die "Failed to chmod"

  # Ad-hoc codesign for macOS (prevents quarantine kills)
  if command -v codesign >/dev/null 2>&1; then
    codesign --force --sign - "${VVB_VVX_BINARY}" 2>/dev/null || buc_warn "codesign failed (non-fatal)"
  fi

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

vob_release() {
  zvob_sentinel

  buc_doc_brief "Create VVK parcel: test, build, collect, brand, tarball"
  buc_doc_shown || return 0

  local z_staging="${BUD_TEMP_DIR}/staging"
  local z_install_script="${BURC_TOOLS_DIR}/vvk/vvi_install.sh"
  local z_registry="${BURC_TOOLS_DIR}/vok/vov_veiled/vovr_registry.json"
  local z_vvx="${VVB_VVX_BINARY}"

  buc_step "Running tests"
  buc_log_args "Cargo dir: ${ZVOB_CARGO_DIR}"
  buc_log_args "Features: ${VOF_VOK_FEATURES:-none}"

  cargo test --manifest-path "${ZVOB_CARGO_DIR}/Cargo.toml" --features "${VOF_VOK_FEATURES}" || buc_die "Tests failed"

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
    --install-script "${z_install_script}" \
    --managed-kits "${BURC_MANAGED_KITS}" || buc_die "release_collect failed"

  buc_step "Copying platform binary"

  local z_bin_dest="${z_staging}/kits/vvk/bin"
  mkdir -p "${z_bin_dest}" || buc_die "Failed to create bin dir"

  cp "${z_vvx}" "${z_bin_dest}/vvx-${VVB_PLATFORM}" || buc_die "Failed to copy binary"
  chmod +x "${z_bin_dest}/vvx-${VVB_PLATFORM}" || buc_die "Failed to chmod"

  buc_log_args "Binary: ${z_bin_dest}/vvx-${VVB_PLATFORM}"

  buc_step "Branding parcel"

  local z_commit=""
  z_commit=$(zvob_commit_capture)

  buc_log_args "Registry: ${z_registry}"
  buc_log_args "Commit: ${z_commit}"

  local z_hallmark_file="${BUD_TEMP_DIR}/hallmark.txt"
  local z_stderr_file="${BUD_TEMP_DIR}/release_brand_stderr.txt"

  "${z_vvx}" release_brand \
    --staging "${z_staging}" \
    --registry "${z_registry}" \
    --commit "${z_commit}" \
    --managed-kits "${BURC_MANAGED_KITS}" > "${z_hallmark_file}" 2>"${z_stderr_file}" || buc_die "release_brand failed"

  local z_hallmark=""
  z_hallmark=$(<"${z_hallmark_file}")
  test -n "${z_hallmark}" || buc_die "Failed to read hallmark"

  buc_log_args "Hallmark: ${z_hallmark}"

  # Check if a new hallmark was allocated by examining stderr
  local z_is_new=0
  if grep -q "Allocated new hallmark: ${z_hallmark}" "${z_stderr_file}" 2>/dev/null; then
    z_is_new=1
  fi

  # If new hallmark was allocated, commit the registry
  if [ "${z_is_new}" -eq 1 ]; then
    buc_step "Committing registry for new hallmark"
    buc_log_args "New hallmark: ${z_hallmark}"

    local z_kit_list="${BURC_MANAGED_KITS// /, }"
    "${z_vvx}" commit \
      --message "vvb:${z_hallmark}::A: allocate hallmark for ${z_kit_list}" \
      --file "${z_registry}" || buc_die "Failed to commit registry"

    buc_log_args "Registry committed"
  fi

  buc_step "Creating tarball"

  local z_parcel_name="vvk-parcel-${z_hallmark}"
  local z_parcels_dir=".jjk/parcels"
  local z_tarball="${z_parcels_dir}/${z_parcel_name}.tar.gz"

  # Create parcels directory if it doesn't exist
  mkdir -p "${z_parcels_dir}" || buc_die "Failed to create parcels directory: ${z_parcels_dir}"

  buc_log_args "Tarball: ${z_tarball}"

  tar -czf "${z_tarball}" -C "${z_staging}" . || buc_die "tar failed"

  buc_success "Parcel created: ${z_tarball}"
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

vob_freshen() {
  zvob_sentinel

  buc_doc_brief "Freshen CLAUDE.md managed sections from kit forge templates"
  buc_doc_shown || return 0

  local z_burc_file="${PWD}/.buk/burc.env"
  local z_vvx="${VVB_VVX_BINARY}"

  buv_file_exists "${z_burc_file}"
  buv_file_exists "${z_vvx}"

  buc_step "Freshening CLAUDE.md"
  buc_log_args "burc: ${z_burc_file}"

  "${z_vvx}" vvx_freshen --burc "${z_burc_file}" || buc_die "vvx_freshen failed"

  buc_success "CLAUDE.md managed sections updated"
}

# eof
