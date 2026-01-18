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
# VVU Uninstall - Remove VVK kit assets from target repo
#
# This script runs from the target repo's root. It verifies installation
# exists, creates a pre-uninstall snapshot, invokes vacate, and commits.
#
# Usage: ./Tools/vvk/vvu_uninstall.sh
#
# Note: This is a kit asset - distributed with VVK, runs in target repo.
# Local functions follow BCG patterns without sourcing dependencies.

set -euo pipefail

######################################################################
# Local BCG-style functions (cannot source BUK - it's being removed!)

zvvu_die() {
  echo "vvu_uninstall: error: ${1}" >&2
  exit 1
}

zvvu_step() {
  echo "${1}" >&2
}

######################################################################
# Internal functions

zvvu_platform_capture() {
  local z_os
  local z_arch
  local z_platform

  z_os=$(uname -s) || return 1
  z_arch=$(uname -m) || return 1

  case "${z_os}-${z_arch}" in
    Darwin-arm64)   z_platform="darwin-arm64" ;;
    Darwin-x86_64)  z_platform="darwin-x86_64" ;;
    Linux-x86_64)   z_platform="linux-x86_64" ;;
    Linux-aarch64)  z_platform="linux-aarch64" ;;
    *)              return 1 ;;
  esac

  echo "${z_platform}"
}

zvvu_git_is_dirty() {
  # Returns 0 if working tree has uncommitted changes, 1 if clean
  ! git diff --quiet HEAD 2>/dev/null || ! git diff --cached --quiet HEAD 2>/dev/null
}

zvvu_git_commit_if_dirty() {
  local z_message="${1}"

  if zvvu_git_is_dirty; then
    zvvu_step "Creating pre-uninstall snapshot..."
    git add -A
    git commit -m "${z_message}" || zvvu_die "Failed to commit snapshot"
  fi
}

######################################################################
# Main

zvvu_main() {
  local z_burc_path=".buk/burc.env"
  local z_brand_path=".vvk/vvbf_brand.json"

  # Verify we're in a repo with VVK installed
  test -f "${z_burc_path}" || zvvu_die "Not in a VVK-enabled repo (missing ${z_burc_path})"
  test -f "${z_brand_path}" || zvvu_die "Nothing installed (missing ${z_brand_path})"

  # Get platform and find binary
  local z_platform
  z_platform=$(zvvu_platform_capture) || zvvu_die "Unsupported platform: $(uname -s)-$(uname -m)"

  local z_binary="Tools/vvk/bin/vvx-${z_platform}"
  test -f "${z_binary}" || zvvu_die "VVX binary not found: ${z_binary}"
  test -x "${z_binary}" || zvvu_die "VVX binary not executable: ${z_binary}"

  zvvu_step "Uninstalling VVK..."
  zvvu_step "  Platform: ${z_platform}"

  # Pre-uninstall snapshot
  zvvu_git_commit_if_dirty "VVK pre-uninstall snapshot"

  # Run vacate
  "${z_binary}" vvx_vacate --burc "${z_burc_path}" || zvvu_die "Vacate failed"

  # Post-uninstall commit
  zvvu_step "Creating post-uninstall commit..."
  git add -A
  git commit -m "VVK uninstall" || zvvu_die "Failed to commit uninstall"

  zvvu_step "Uninstall complete."
}

zvvu_main "$@"

# eof
