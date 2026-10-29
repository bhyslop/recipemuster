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
# BPU Uninstall - Bootstrap script for parcel removal
#
# Thin bootstrap: detect platform, exec the parcel's install binary's vacate.
# All logic (git checks, path safety) is in Rust.
#
# Usage: ./bpu_uninstall.sh /path/to/target/<moorings-dir>/burc.env
#        the path is the consumer's own BURC file — its moorings dirname is
#        that consumer's choice (e.g. rbmm_moorings, jjmm_moorings), never
#        this script's business.
#
# This script runs at the root of an extracted parcel and nowhere else,
# resolving its binary from its own location, never from the process working
# directory: bpx-{platform}, the install binary every parcel carries beside
# it. The vacate reads the parcel's behest, which names the signets the
# target's Claude assets wear and the binary to remove, and only a parcel
# carries one; a copy of this script laid into a target that takes this kit
# finds no binary beside it and refuses.
#
# Note: This is a standalone bootstrap - cannot depend on BUK.
# Local functions follow the same console conventions without sourcing dependencies.

set -euo pipefail

######################################################################
# Constants

# Example relpath for the usage message only — the real path always arrives
# as $1; no consumer moorings dirname is assumed or required.
readonly ZBPU_BURC_RELPATH_EXAMPLE="<moorings-dir>/burc.env"

# The install binary, beside this script at the parcel root under its
# platform suffix, and the verb it uninstalls by.
readonly ZBPU_ENGINE_STEM="bpx-"
readonly ZBPU_ENGINE_VERB="vacate"

######################################################################
# Local console-style functions

zbpu_die() {
  echo "bpu_uninstall: error: ${1}" >&2 || exit 1
  exit 1
}

zbpu_step() {
  echo "${1}" >&2 || zbpu_die "Could not write to stderr"
}

######################################################################
# Internal functions

zbpu_platform_capture() {
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

######################################################################
# Main

zbpu_main() {
  local z_burc_path="${1:-}"

  if test -z "${z_burc_path}"; then
    printf '%s\n' \
      "bpu_uninstall: Remove a parcel's Claude Code integration from a target repository" \
      "" \
      "Usage: ${0} /path/to/target/${ZBPU_BURC_RELPATH_EXAMPLE}" \
      "" \
      "Run from the root of an extracted parcel." \
      >&2 || zbpu_die "Could not write the usage to stderr"
    exit 1
  fi

  test -f "${z_burc_path}" || zbpu_die "BURC file not found: ${z_burc_path}"
  test -r "${z_burc_path}" || zbpu_die "BURC file not readable: ${z_burc_path}"

  local z_platform=""
  local z_platform_status=0
  z_platform=$(zbpu_platform_capture) || z_platform_status=$?
  if test "${z_platform_status}" -ne 0; then
    local z_os=""
    z_os=$(uname -s) || zbpu_die "Unsupported platform, and uname could not name its system"
    local z_machine=""
    z_machine=$(uname -m) || zbpu_die "Unsupported platform, and uname could not name its machine"
    zbpu_die "Unsupported platform: ${z_os}-${z_machine}"
  fi

  local z_script_dir="${BASH_SOURCE[0]%/*}"

  local -r z_binary="${z_script_dir}/${ZBPU_ENGINE_STEM}${z_platform}"

  test -f "${z_binary}" || zbpu_die "Platform binary not found: ${z_binary} - run this script from the root of an extracted parcel"

  test -x "${z_binary}" || chmod +x "${z_binary}" || zbpu_die "Cannot make binary executable: ${z_binary}"

  zbpu_step "Uninstalling parcel integration..."
  zbpu_step "  Platform: ${z_platform}"
  zbpu_step "  Target: ${z_burc_path}"

  exec "${z_binary}" "${ZBPU_ENGINE_VERB}" --parcel "${z_script_dir}" --burc "${z_burc_path}" \
    || zbpu_die "Cannot exec the binary: ${z_binary}"
}

zbpu_main "$@"

# eof
