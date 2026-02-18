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
# BUK Workbench - Routes BUK management commands

set -euo pipefail

# Get script directory
BUW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${BUW_SCRIPT_DIR}/buc_command.sh"
source "${BURC_BUK_DIR}/burd_regime.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify dispatch completed
zburd_kindle

# Verbose output if BURD_VERBOSE is set
buw_show() {
  test "${BURD_VERBOSE:-0}" != "1" || echo "BUWSHOW: $*"
}

# Simple routing function
buw_route() {
  local z_command="$1"
  shift

  buw_show "Routing command: ${z_command} with args: $*"

  zburd_sentinel

  buw_show "BURD environment verified"

  # Route based on command
  local z_buut_cli="${BUW_SCRIPT_DIR}/buut_cli.sh"
  local z_burc_cli="${BUW_SCRIPT_DIR}/burc_cli.sh"
  local z_burs_cli="${BUW_SCRIPT_DIR}/burs_cli.sh"
  local z_bure_cli="${BUW_SCRIPT_DIR}/bure_cli.sh"

  case "${z_command}" in

    # TabTarget subsystem (buw-tt-*)
    buw-tt-ll)  exec "${z_buut_cli}" buut_list_launchers                 "$@" ;;
    buw-tt-cbl) exec "${z_buut_cli}" buut_tabtarget_batch_logging        "$@" ;;
    buw-tt-cbn) exec "${z_buut_cli}" buut_tabtarget_batch_nolog          "$@" ;;
    buw-tt-cil) exec "${z_buut_cli}" buut_tabtarget_interactive_logging  "$@" ;;
    buw-tt-cin) exec "${z_buut_cli}" buut_tabtarget_interactive_nolog    "$@" ;;
    buw-tt-cl)  exec "${z_buut_cli}" buut_launcher                       "$@" ;;

    # Config Regime subsystem
    buw-rcv) exec "${z_burc_cli}" validate ;;
    buw-rcr) exec "${z_burc_cli}" render ;;

    # Station Regime subsystem
    buw-rsv) exec "${z_burs_cli}" validate ;;
    buw-rsr) exec "${z_burs_cli}" render ;;

    # Environment Regime subsystem
    buw-rev) exec "${z_bure_cli}" validate ;;
    buw-rer) exec "${z_bure_cli}" render ;;

    # Unknown command
    *)   buc_die "Unknown command: ${z_command}" ;;
  esac
}

buw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  buw_route "${z_command}" "$@"
}

buw_main "$@"

# eof
