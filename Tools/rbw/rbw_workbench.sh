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
# RBW Workbench - Routes Recipe Bottle commands to CLIs
#
# Commands routed to rbob_cli.sh (require moniker imprint):
#   rbw-s   Start service (sentry + censer + bottle)
#   rbw-z   Stop service
#   rbw-S   Connect to sentry container
#   rbw-C   Connect to censer container
#   rbw-B   Connect to bottle container
#   rbw-o   Observe network traffic (tcpdump)
#
# Regime operations (pure routing to CLI scripts):
#   rbw-rnr Render nameplate regime  → rbrn_cli.sh render
#   rbw-rnv Validate nameplate regime → rbrn_cli.sh validate
#   rbw-rvr Render vessel regime      → rbrv_cli.sh render
#   rbw-rvv Validate vessel regime    → rbrv_cli.sh validate
#   rbw-rrr Render repo regime        → rbrr_cli.sh render
#   rbw-rrv Validate repo regime      → rbrr_cli.sh validate
#
# Commands handled locally:
#   rbw-lB  Local build from recipe (no moniker needed)

set -euo pipefail

# Get script directory
RBW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBW_SCRIPT_DIR}/../buk/buc_command.sh"
# Show filename on each displayed line
buc_context "${0##*/}"

# Verbose output if BURD_VERBOSE is set
rbw_show() {
  test "${BURD_VERBOSE:-0}" != "1" || echo "RBWSHOW: $*"
}

######################################################################
# Routing

rbw_route() {
  local z_command="$1"
  shift

  rbw_show "Routing command: ${z_command} with args: $*"

  # Verify BUD environment
  test -n "${BURD_TEMP_DIR:-}" || buc_die "BURD_TEMP_DIR not set - must be called from BUD"
  test -n "${BURD_NOW_STAMP:-}" || buc_die "BURD_NOW_STAMP not set - must be called from BUD"

  rbw_show "BUD environment verified"

  # Route based on colophon
  local z_rbob_cli="${RBW_SCRIPT_DIR}/rbob_cli.sh"

  case "${z_command}" in

    # Bottle operations (routed to rbob_cli.sh)
    # Workbench translates BURD_TOKEN_3 (imprint) to RBOB_MONIKER for CLI
    rbw-s|rbw-z|rbw-S|rbw-C|rbw-B|rbw-o)
      test -n "${BURD_TOKEN_3:-}" || buc_die "${z_command} requires moniker imprint (BURD_TOKEN_3)"
      export RBOB_MONIKER="${BURD_TOKEN_3}"
      case "${z_command}" in
        rbw-s)  exec "${z_rbob_cli}" rbob_start          ;;
        rbw-z)  exec "${z_rbob_cli}" rbob_stop           ;;
        rbw-S)  exec "${z_rbob_cli}" rbob_connect_sentry ;;
        rbw-C)  exec "${z_rbob_cli}" rbob_connect_censer ;;
        rbw-B)  exec "${z_rbob_cli}" rbob_connect_bottle ;;
        rbw-o)  exec "${z_rbob_cli}" rbob_observe        ;;
      esac
      ;;

    # Nameplate regime operations (routed to rbrn_cli.sh)
    rbw-rnr) exec "${RBW_SCRIPT_DIR}/rbrn_cli.sh" render   ${1+"$@"} ;;
    rbw-rnv) exec "${RBW_SCRIPT_DIR}/rbrn_cli.sh" validate ${1+"$@"} ;;

    # Vessel regime operations (routed to rbrv_cli.sh)
    rbw-rvr) exec "${RBW_SCRIPT_DIR}/rbrv_cli.sh" render   ${1+"$@"} ;;
    rbw-rvv) exec "${RBW_SCRIPT_DIR}/rbrv_cli.sh" validate ${1+"$@"} ;;

    # Repo regime operations (routed to rbrr_cli.sh)
    rbw-rrr) exec "${RBW_SCRIPT_DIR}/rbrr_cli.sh" render   ${1+"$@"} ;;
    rbw-rrv) exec "${RBW_SCRIPT_DIR}/rbrr_cli.sh" validate ${1+"$@"} ;;

    # Cross-nameplate operations (routed to rbrn_cli.sh)
    rbw-ni) exec "${RBW_SCRIPT_DIR}/rbrn_cli.sh" survey ${1+"$@"} ;;
    rbw-nv) exec "${RBW_SCRIPT_DIR}/rbrn_cli.sh" audit  ${1+"$@"} ;;

    # Unknown command
    *)   buc_die "Unknown command: ${z_command}" ;;
  esac
}

rbw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  rbw_route "${z_command}" "$@"
}

rbw_main "$@"

# eof
