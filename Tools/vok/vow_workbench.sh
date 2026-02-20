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
# VOW Workbench - Routes VOK management commands

set -euo pipefail

# Get script directory
VOW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${VOW_SCRIPT_DIR}/../buk/buc_command.sh"
source "${VOW_SCRIPT_DIR}/../buk/burd_regime.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify dispatch completed
zburd_kindle

# Verbose output if BURE_VERBOSE is set
vow_show() {
  test "${BURE_VERBOSE:-0}" != "1" || echo "VOWSHOW: $*"
}

# Simple routing function
vow_route() {
  local z_command="$1"
  shift

  vow_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel

  vow_show "BUD environment verified"

  # Route based on command
  local z_vob_cli="${VOW_SCRIPT_DIR}/vob_cli.sh"

  case "${z_command}" in

    # Build subsystem
    vow-b)  exec "${z_vob_cli}" vob_build   "$@" ;;
    vow-c)  exec "${z_vob_cli}" vob_clean   "$@" ;;
    vow-t)  exec "${z_vob_cli}" vob_test    "$@" ;;
    vow-R)  exec "${z_vob_cli}" vob_release "$@" ;;  # capital R = big action
    vow-F)  exec "${z_vob_cli}" vob_freshen "$@" ;;  # capital F = CLAUDE.md freshen

    # Run VVX binary directly
    vow-r)  exec "${VOW_SCRIPT_DIR}/target/release/vvr" "$@" ;;

    # Unknown command
    *)   buc_die "Unknown command: ${z_command}" ;;
  esac
}

vow_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  vow_route "${z_command}" "$@"
}

vow_main "$@"

# eof
