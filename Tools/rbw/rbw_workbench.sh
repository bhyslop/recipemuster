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
# Bottle operations (require moniker imprint, explicit case arms):
#   rbw-s   Start service (sentry + censer + bottle)
#   rbw-z   Stop service
#   rbw-S   Connect to sentry container
#   rbw-C   Connect to censer container
#   rbw-B   Connect to bottle container
#   rbw-o   Observe network traffic (tcpdump)
#
# All other commands dispatch via zipper registry (zbuz_exec_lookup).
# See rbz_zipper.sh for the complete colophon→CLI→command mapping.

set -euo pipefail

# Get script directory
RBW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBW_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBW_SCRIPT_DIR}/../buk/buz_zipper.sh"
source "${RBW_SCRIPT_DIR}/rbz_zipper.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Kindle zipper registry
zbuz_kindle
zrbz_kindle

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
  case "${z_command}" in

    # Bottle operations (non-degenerate: translate imprint to RBOB_MONIKER)
    rbw-s|rbw-z|rbw-S|rbw-C|rbw-B|rbw-o)
      local z_rbob_cli="${RBW_SCRIPT_DIR}/rbob_cli.sh"
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

    # All other commands: resolve via zipper registry
    *)
      zbuz_exec_lookup "${z_command}" "${RBW_SCRIPT_DIR}" "$@" || {
        if [ -n "${z_command}" ]; then
          buc_warn "Unknown command: ${z_command}"
        fi
        buc_info "Available command groups:"
        buc_info ""
        buc_info "Bottle operations (require moniker imprint):"
        buc_info "  rbw-s   Start service (sentry + censer + bottle)"
        buc_info "  rbw-z   Stop service"
        buc_info "  rbw-S   Connect to sentry container"
        buc_info "  rbw-C   Connect to censer container"
        buc_info "  rbw-B   Connect to bottle container"
        buc_info "  rbw-o   Observe network traffic (tcpdump)"
        buc_info ""
        buc_info "Payor commands:"
        buc_info "  rbw-PC  Create depot          rbw-PI  Install payor"
        buc_info "  rbw-PD  Destroy depot          rbw-PE  Establish payor"
        buc_info "  rbw-PG  Reset governor         rbw-PR  Refresh payor"
        buc_info "  rbw-QB  Quota build"
        buc_info ""
        buc_info "General operations:"
        buc_info "  rbw-ld  List depots"
        buc_info ""
        buc_info "Governor commands:"
        buc_info "  rbw-GR  Create retriever       rbw-GD  Create director"
        buc_info ""
        buc_info "Admin commands:"
        buc_info "  rbw-ps  Establish payor (admin)"
        buc_info "  rbw-Gl  List service accounts  rbw-GS  Delete service account"
        buc_info ""
        buc_info "Ark commands:"
        buc_info "  rbw-aA  Abjure ark             rbw-ab  Beseech ark"
        buc_info "  rbw-aC  Conjure ark            rbw-as  Summon ark"
        buc_info ""
        buc_info "Image commands:"
        buc_info "  rbw-iB  Build image            rbw-iD  Delete image"
        buc_info "  rbw-il  List images             rbw-ir  Retrieve image"
        buc_info ""
        buc_info "Nameplate regime operations:"
        buc_info "  rbw-rnr Render nameplate regime"
        buc_info "  rbw-rnv Validate nameplate regime"
        buc_info ""
        buc_info "Vessel regime operations:"
        buc_info "  rbw-rvr Render vessel regime"
        buc_info "  rbw-rvv Validate vessel regime"
        buc_info ""
        buc_info "Repo regime operations:"
        buc_info "  rbw-rrr Render repo regime"
        buc_info "  rbw-rrv Validate repo regime"
        buc_info "  rbw-rrg Refresh GCB image pins"
        buc_info ""
        buc_info "Payor regime operations:"
        buc_info "  rbw-rpr Render payor regime"
        buc_info "  rbw-rpv Validate payor regime"
        buc_info ""
        buc_info "OAuth regime operations:"
        buc_info "  rbw-ror Render OAuth regime"
        buc_info "  rbw-rov Validate OAuth regime"
        buc_info ""
        buc_info "Station regime operations:"
        buc_info "  rbw-rsr Render station regime"
        buc_info "  rbw-rsv Validate station regime"
        buc_info ""
        buc_info "Auth regime operations (imprint: governor, retriever, director):"
        buc_info "  rbw-rar Render auth regime     rbw-ral  List auth regimes"
        buc_info "  rbw-rav Validate auth regime"
        buc_info ""
        buc_info "Cross-nameplate operations:"
        buc_info "  rbw-ni  Survey nameplates"
        buc_info "  rbw-nv  Audit nameplates"
        buc_info ""
        buc_info "Test operations:"
        buc_info "  rbw-ta  Run all test suites"
        buc_info "  rbw-ts  Run single test suite"
        buc_info "  rbw-to  Run single test function"
        buc_info "  rbw-tn  Run nameplate suite (imprint: nsproto, srjcl, pluml)"
        buc_info "  rbw-trg Run regime-smoke suite"
        return 0
      }
      ;;
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
