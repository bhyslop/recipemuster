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
source "${BUW_SCRIPT_DIR}/buut_tabtarget.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Verbose output if BUD_VERBOSE is set
buw_show() {
  test "${BUD_VERBOSE:-0}" != "1" || echo "BUWSHOW: $*"
}

# Load BURC configuration
buw_load_burc() {
  local z_burc_file="${PWD}/.buk/burc.env"

  test -f "${z_burc_file}" || buc_die "BURC file not found: ${z_burc_file}"

  buw_show "Loading BURC from: ${z_burc_file}"
  # shellcheck disable=SC1090
  source "${z_burc_file}"
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

  # Load BURC configuration
  buw_load_burc

  # Selective kindling for submodules (only create operations need buut)
  case "${z_command}" in
    buw-tt-c*) zbuut_kindle ;;
  esac

  # Route based on command
  case "${z_command}" in

    # TabTarget subsystem (buw-tt-*)
    buw-tt-ll) buut_list_launchers ;;
    buw-tt-cbl) buut_tabtarget_batch_logging "$@" ;;
    buw-tt-cbn) buut_tabtarget_batch_nolog "$@" ;;
    buw-tt-cil) buut_tabtarget_interactive_logging "$@" ;;
    buw-tt-cin) buut_tabtarget_interactive_nolog "$@" ;;
    buw-tt-cl) buut_launcher "$@" ;;

    # Regime management (consolidated)
    buw-rv)
      # Validate both regimes
      buc_step "Validating BURC"
      "${BUW_SCRIPT_DIR}/burc_cli.sh" validate "${PWD}/.buk/burc.env" || buc_die "BURC validation failed"

      buc_step "Validating BURS"
      local z_station_file="${PWD}/${BURC_STATION_FILE}"
      "${BUW_SCRIPT_DIR}/burs_cli.sh" validate "${z_station_file}" || buc_die "BURS validation failed"

      buc_success "All regime validations passed"
      ;;

    buw-rr)
      # Render both regimes
      buc_step "BURC Configuration"
      "${BUW_SCRIPT_DIR}/burc_cli.sh" render "${PWD}/.buk/burc.env" || buc_die "BURC render failed"

      buc_step "BURS Configuration"
      local z_station_file="${PWD}/${BURC_STATION_FILE}"
      "${BUW_SCRIPT_DIR}/burs_cli.sh" render "${z_station_file}" || buc_die "BURS render failed"
      ;;

    buw-ri)
      # Show info for both regimes
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
