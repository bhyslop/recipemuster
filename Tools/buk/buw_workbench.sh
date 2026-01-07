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

# Show filename on each displayed line
buc_context "${0##*/}"

# Verify launcher provided regime environment
test -n "${BUD_REGIME_FILE:-}"    || buc_die "BUD_REGIME_FILE not set - must be called via launcher"
test -n "${BURC_STATION_FILE:-}"  || buc_die "BURC_STATION_FILE not set - launcher failed to load BURC"
test -n "${BURC_TOOLS_DIR:-}"     || buc_die "BURC_TOOLS_DIR not set - launcher failed to load BURC"

# Verbose output if BUD_VERBOSE is set
buw_show() {
  test "${BUD_VERBOSE:-0}" != "1" || echo "BUWSHOW: $*"
}

# Simple routing function
buw_route() {
  local z_command="$1"
  shift
  local z_args="$*"

  buw_show "Routing command: ${z_command} with args: ${z_args}"

  # Verify BDU environment variables are present
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR not set - must be called from BUD"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP not set - must be called from BUD"

  buw_show "BDU environment verified"

  # Route based on command
  local z_buut_cli="${BUW_SCRIPT_DIR}/buut_cli.sh"

  case "${z_command}" in

    # TabTarget subsystem (buw-tt-*) - delegate to buut_cli.sh
    buw-tt-ll)  exec "${z_buut_cli}" buut_list_launchers                 $z_args ;;
    buw-tt-cbl) exec "${z_buut_cli}" buut_tabtarget_batch_logging        $z_args ;;
    buw-tt-cbn) exec "${z_buut_cli}" buut_tabtarget_batch_nolog          $z_args ;;
    buw-tt-cil) exec "${z_buut_cli}" buut_tabtarget_interactive_logging  $z_args ;;
    buw-tt-cin) exec "${z_buut_cli}" buut_tabtarget_interactive_nolog    $z_args ;;
    buw-tt-cl)  exec "${z_buut_cli}" buut_launcher                       $z_args ;;

    # Regime management - delegate to regime CLIs
    buw-rv)
      buc_step "Validating BURC"
      "${BUW_SCRIPT_DIR}/burc_cli.sh" validate "${BUD_REGIME_FILE}" || buc_die "BURC validation failed"
      buc_step "Validating BURS"
      "${BUW_SCRIPT_DIR}/burs_cli.sh" validate "${BURC_STATION_FILE}" || buc_die "BURS validation failed"
      buc_success "All regime validations passed"
      ;;

    buw-rr)
      buc_step "BURC Configuration"
      "${BUW_SCRIPT_DIR}/burc_cli.sh" render "${BUD_REGIME_FILE}" || buc_die "BURC render failed"
      buc_step "BURS Configuration"
      "${BUW_SCRIPT_DIR}/burs_cli.sh" render "${BURC_STATION_FILE}" || buc_die "BURS render failed"
      ;;

    buw-ri)
      buc_step "BURC Specification"
      "${BUW_SCRIPT_DIR}/burc_cli.sh" info || buc_die "BURC info failed"
      buc_step "BURS Specification"
      "${BUW_SCRIPT_DIR}/burs_cli.sh" info || buc_die "BURS info failed"
      ;;

    # Unknown command
    *)
      buc_die "Unknown command: ${z_command}\nAvailable commands:\n  TabTarget: buw-tt-ll, buw-tt-cbl, buw-tt-cbn, buw-tt-cil, buw-tt-cin, buw-tt-cl\n  Regime:    buw-rv, buw-rr, buw-ri"
      ;;
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
