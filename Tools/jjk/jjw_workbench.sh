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
# JJW Workbench - Routes Job Jockey commands

set -euo pipefail

# Get script directory
JJW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${JJW_SCRIPT_DIR}/../buk/buc_command.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verbose output if BUD_VERBOSE is set
jjw_show() {
  test "${BUD_VERBOSE:-0}" != "1" || echo "JJWSHOW: $*"
}

# Simple routing function
jjw_route() {
  local z_command="$1"
  shift
  local z_args="$*"

  jjw_show "Routing command: ${z_command} with args: ${z_args}"

  # Verify BDU environment variables are present
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR not set - must be called from BUD"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP not set - must be called from BUD"

  jjw_show "BDU environment verified"

  local z_cli="${JJW_SCRIPT_DIR}/jju_cli.sh"

  # Route based on command
  case "${z_command}" in

    # Arcanum commands (install/check/uninstall)
    jja-c|jja-i|jja-u)
      jjw_show "Delegating to arcanum: ${z_command}"
      exec "${JJW_SCRIPT_DIR}/jja_arcanum.sh" "${z_command}" "$@"
      ;;

    # Favor encoding/decoding
    jjk-fe) exec "${z_cli}" jju_favor_encode "$@" ;;
    jjk-fd) exec "${z_cli}" jju_favor_decode "$@" ;;

    # Studbook operations
    jjk-m)  exec "${z_cli}" jju_muster "$@" ;;
    jjk-s)  exec "${z_cli}" jju_saddle "$@" ;;
    jjk-n)  exec "${z_cli}" jju_nominate "$@" ;;
    jjk-sl) exec "${z_cli}" jju_slate "$@" ;;
    jjk-rs) exec "${z_cli}" jju_reslate "$@" ;;
    jjk-ra) exec "${z_cli}" jju_rail "$@" ;;
    jjk-t)  exec "${z_cli}" jju_tally "$@" ;;
    jjk-w)  exec "${z_cli}" jju_wrap "$@" ;;
    jjk-re) exec "${z_cli}" jju_retire_extract "$@" ;;

    # Steeplechase operations
    jjk-c)  exec "${z_cli}" jju_chalk "$@" ;;
    jjk-rn) exec "${z_cli}" jju_rein "$@" ;;
    jjk-no) exec "${z_cli}" jju_notch "$@" ;;

    # Help - show all function docs (no command = buc_execute shows help)
    jjk-h) exec "${z_cli}" ;;

    *)
      buc_die "Unknown command: ${z_command}"
      ;;
  esac
}

# Main entry point
jjw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  jjw_route "${z_command}" "$@"
}

jjw_main "$@"

# eof
