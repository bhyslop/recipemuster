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
# BPI Install - Bootstrap script for parcel installation
#
# This script lives at the root of an extracted parcel. It validates
# the target environment and invokes the parcel's own install binary,
# bpx, which stands beside it platform-suffixed in every parcel.
#
# Usage: ./bpi_install.sh /path/to/target/repo/burc.env
#        the path is the consumer's own BURC file — its moorings dirname is
#        that consumer's choice (e.g. rbmm_moorings, jjmm_moorings), never
#        this script's business.
#    or: ./bpi_install.sh /path/to/target
#        a target carrying no repository, named by its directory: the install
#        empties it and lays it whole from the parcel on every run, refusing a
#        non-empty directory that holds no record. The directory must already
#        stand; the install stands none.
#
# Note: This is a standalone bootstrap - cannot depend on BUK.
# Local functions follow the same console conventions without sourcing dependencies.

set -euo pipefail

######################################################################
# Constants

# Example relpath for the usage message only — the real path always arrives
# as $1; no consumer moorings dirname is assumed or required.
readonly ZBPI_BURC_RELPATH_EXAMPLE="<moorings-dir>/burc.env"

# The install binary, beside this script at the parcel root under its
# platform suffix, and the verb it installs by.
readonly ZBPI_ENGINE_STEM="bpx-"
readonly ZBPI_ENGINE_VERB="emplace"

######################################################################
# Local console-style functions (cannot source BUK)

zbpi_die() {
  echo "bpi_install: error: ${1}" >&2 || exit 1
  exit 1
}

zbpi_step() {
  echo "${1}" >&2 || zbpi_die "Could not write to stderr"
}

######################################################################
# Internal functions

zbpi_platform_capture() {
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

zbpi_main() {
  local z_burc_path="${1:-}"

  if test -z "${z_burc_path}"; then
    printf '%s\n' \
      "bpi_install: Install a parcel into a target repository" \
      "" \
      "Usage: ${0} /path/to/target/${ZBPI_BURC_RELPATH_EXAMPLE}" \
      "   or: ${0} /path/to/target    (a target carrying no repository)" \
      "" \
      "A target repository must have BURC configured (its own burc.env)." \
      >&2 || zbpi_die "Could not write the usage to stderr"
    exit 1
  fi

  if ! test -d "${z_burc_path}"; then
    test -f "${z_burc_path}" || zbpi_die "BURC file not found: ${z_burc_path}"
    test -r "${z_burc_path}" || zbpi_die "BURC file not readable: ${z_burc_path}"
  fi

  local z_platform=""
  local z_platform_status=0
  z_platform=$(zbpi_platform_capture) || z_platform_status=$?
  if test "${z_platform_status}" -ne 0; then
    local z_os=""
    z_os=$(uname -s) || zbpi_die "Unsupported platform, and uname could not name its system"
    local z_machine=""
    z_machine=$(uname -m) || zbpi_die "Unsupported platform, and uname could not name its machine"
    zbpi_die "Unsupported platform: ${z_os}-${z_machine}"
  fi

  local z_script_dir="${BASH_SOURCE[0]%/*}"

  local z_binary="${z_script_dir}/${ZBPI_ENGINE_STEM}${z_platform}"

  test -f "${z_binary}" || zbpi_die "Platform binary not found: ${z_binary}"

  test -x "${z_binary}" || chmod +x "${z_binary}" || zbpi_die "Cannot make binary executable: ${z_binary}"

  zbpi_step "Installing parcel..."
  zbpi_step "  Platform: ${z_platform}"
  zbpi_step "  Target: ${z_burc_path}"

  exec "${z_binary}" "${ZBPI_ENGINE_VERB}" --parcel "${z_script_dir}" --burc "${z_burc_path}" \
    || zbpi_die "Cannot exec the platform binary: ${z_binary}"
}

zbpi_main "$@"

# eof
