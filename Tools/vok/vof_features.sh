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
# VOF - VOK Features Module
#
# BCG-compliant library module for detecting available kit features for Rust builds.
# Provides manifest paths, feature flags, and cargo helpers (clean).
# No CLI - sourced and kindled by VOB (build) and called from VOK test operations.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZVOF_SOURCED:-}" || buc_die "Module vof multiply sourced - check sourcing hierarchy"
ZVOF_SOURCED=1

######################################################################
# Internal Functions (zvof_*)

zvof_kindle() {
  test -z "${ZVOF_KINDLED:-}" || buc_die "Module vof already kindled"

  # Validate BURC environment
  test -n "${BURC_TOOLS_DIR:-}" || buc_die "BURC_TOOLS_DIR is unset"

  # Public exports - VOK is special (integrator crate, gets features)
  VOF_VOK_MANIFEST="${BURC_TOOLS_DIR}/vok/Cargo.toml"
  VOF_VOK_FEATURES=""

  # Other test manifests - space-separated list of Cargo.toml paths (no features)
  VOF_TEST_MANIFESTS="${BURC_TOOLS_DIR}/vvc/Cargo.toml"

  # Detect jjk
  if test -f "${BURC_TOOLS_DIR}/jjk/vov_veiled/Cargo.toml"; then
    VOF_VOK_FEATURES="${VOF_VOK_FEATURES:+${VOF_VOK_FEATURES},}jjk"
    VOF_TEST_MANIFESTS="${VOF_TEST_MANIFESTS} ${BURC_TOOLS_DIR}/jjk/vov_veiled/Cargo.toml"
  fi

  # Add more kit detection here as needed
  # Example pattern:
  # if test -f "${BURC_TOOLS_DIR}/«kit»/vov_veiled/Cargo.toml"; then
  #   VOF_VOK_FEATURES="${VOF_VOK_FEATURES:+${VOF_VOK_FEATURES},}«kit»"
  #   VOF_TEST_MANIFESTS="${VOF_TEST_MANIFESTS} ${BURC_TOOLS_DIR}/«kit»/vov_veiled/Cargo.toml"
  # fi

  ZVOF_KINDLED=1
}

zvof_sentinel() {
  test "${ZVOF_KINDLED:-}" = "1" || buc_die "Module vof not kindled - call zvof_kindle first"
}

######################################################################
# External Functions (vof_*)

vof_clean() {
  zvof_sentinel

  buc_step "Cleaning vok"
  cargo clean --manifest-path "${VOF_VOK_MANIFEST}" || buc_die "cargo clean failed: vok"

  local z_manifest=""
  for z_manifest in ${VOF_TEST_MANIFESTS}; do
    buc_step "Cleaning ${z_manifest##*/}"
    cargo clean --manifest-path "${z_manifest}" || buc_die "cargo clean failed: ${z_manifest}"
  done
}

# eof
