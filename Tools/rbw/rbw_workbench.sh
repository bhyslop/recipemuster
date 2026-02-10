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
# Regime operations (routed to rbrn_cli.sh / rbrv_cli.sh):
#   rbw-rnr Render nameplate regime (arg: moniker or none to list)
#   rbw-rnv Validate nameplate regime (arg: moniker)
#   rbw-rvr Render vessel regime (arg: sigil or none to list)
#   rbw-rvv Validate vessel regime (arg: sigil)
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
# Local Commands (no nameplate/regime needed)

rbw_local_build() {
  local z_recipe="${1:-}"
  local z_recipe_file="${RBW_SCRIPT_DIR}/../../RBM-recipes/${z_recipe}.recipe"
  local z_image_tag="${z_recipe}:local-${BURD_NOW_STAMP}"

  test -f "${z_recipe_file}" || buc_die "Recipe not found: ${z_recipe_file}"

  buc_step "Building recipe locally: ${z_recipe}"
  rbw_show "Recipe file: ${z_recipe_file}"
  rbw_show "Image tag: ${z_image_tag}"

  docker build -f "${z_recipe_file}" -t "${z_image_tag}" "${RBW_SCRIPT_DIR}/../.." \
    || buc_die "Docker build failed"

  buc_step "Build complete: ${z_image_tag}"
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

    # Local build (handled here - recipe from imprint BURD_TOKEN_3)
    rbw-lB)
      test -n "${BURD_TOKEN_3:-}" || buc_die "rbw-lB requires recipe imprint (BURD_TOKEN_3)"
      rbw_local_build "${BURD_TOKEN_3}"
      ;;

    # Nameplate regime operations (routed to rbrn_cli.sh)
    rbw-rnr|rbw-rnv)
      local z_rbrn_cli="${RBW_SCRIPT_DIR}/rbrn_cli.sh"
      local z_moniker="${1:-}"
      local z_op="render"
      test "${z_command}" = "rbw-rnv" && z_op="validate"

      if test -z "${z_moniker}"; then
        buc_step "Available nameplates:"
        rbrn_list | while read -r z_m; do
          echo "  ${z_m}"
        done
        return 0
      fi

      local z_file="${RBW_SCRIPT_DIR}/rbrn_${z_moniker}.env"
      test -f "${z_file}" || buc_die "Nameplate not found: ${z_file}"
      exec "${z_rbrn_cli}" "${z_op}" "${z_file}"
      ;;

    # Vessel regime operations (routed to rbrv_cli.sh)
    rbw-rvr|rbw-rvv)
      local z_rbrv_cli="${RBW_SCRIPT_DIR}/rbrv_cli.sh"
      local z_sigil="${1:-}"
      local z_op="render"
      test "${z_command}" = "rbw-rvv" && z_op="validate"

      if test -z "${z_sigil}"; then
        buc_step "Available vessels:"
        for z_d in "${RBW_SCRIPT_DIR}/../../rbev-vessels"/*/; do
          test -d "${z_d}" || continue
          test -f "${z_d}/rbrv.env" || continue
          local z_s="${z_d%/}"
          z_s="${z_s##*/}"
          echo "  ${z_s}"
        done
        return 0
      fi

      local z_file="${RBW_SCRIPT_DIR}/../../rbev-vessels/${z_sigil}/rbrv.env"
      test -f "${z_file}" || buc_die "Vessel not found: ${z_file}"
      exec "${z_rbrv_cli}" "${z_op}" "${z_file}"
      ;;

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
