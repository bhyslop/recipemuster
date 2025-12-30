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
# RBW Workbench - Recipe Bottle container lifecycle management
#
# Commands:
#   rbw-s   Start service (sentry + censer + bottle)
#   rbw-z   Stop service
#   rbw-S   Connect to sentry container
#   rbw-C   Connect to censer container
#   rbw-B   Connect to bottle container
#   rbw-lB  Local build from recipe

set -euo pipefail

# Get script directory
RBW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBW_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBW_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${RBW_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBW_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBW_SCRIPT_DIR}/rbob_bottle.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

######################################################################
# Helper Functions

# Verbose output if BUD_VERBOSE is set
rbw_show() {
  test "${BUD_VERBOSE:-0}" != "1" || echo "RBWSHOW: $*"
}

# Load nameplate configuration by moniker and kindle RBOB
# Usage: rbw_load_nameplate <moniker>
rbw_load_nameplate() {
  local z_moniker="${1:-}"
  test -n "${z_moniker}" || buc_die "rbw_load_nameplate: moniker argument required"

  local z_nameplate_file="${RBW_SCRIPT_DIR}/rbrn_${z_moniker}.env"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"

  rbw_show "Loading nameplate: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"

  rbw_show "Kindling nameplate regime"
  zrbrn_kindle

  rbw_show "Nameplate loaded: RBRN_MONIKER=${RBRN_MONIKER}, RBRN_RUNTIME=${RBRN_RUNTIME}"

  # Load RBRR (repository regime) via RBOB helper
  rbw_show "Loading RBRR via RBOB"
  zrbob_load_rbrr

  # Kindle RBOB (validates RBRN and RBRR are ready)
  rbw_show "Kindling RBOB"
  zrbob_kindle
}

######################################################################
# Command Implementations (stubs for future paces)

rbw_cmd_start() {
  local z_moniker="${1:-}"
  buc_step "Starting service: ${z_moniker}"
  rbob_start
}

rbw_cmd_stop() {
  local z_moniker="${1:-}"
  buc_step "Stopping service: ${z_moniker}"
  buc_die "rbw-z not yet implemented"
}

rbw_cmd_connect_sentry() {
  local z_moniker="${1:-}"
  buc_step "Connecting to sentry: ${z_moniker}"
  buc_die "rbw-S not yet implemented"
}

rbw_cmd_connect_censer() {
  local z_moniker="${1:-}"
  buc_step "Connecting to censer: ${z_moniker}"
  buc_die "rbw-C not yet implemented"
}

rbw_cmd_connect_bottle() {
  local z_moniker="${1:-}"
  buc_step "Connecting to bottle: ${z_moniker}"
  buc_die "rbw-B not yet implemented"
}

rbw_cmd_local_build() {
  local z_recipe="${1:-}"
  local z_recipe_file="${RBW_SCRIPT_DIR}/../../RBM-recipes/${z_recipe}.recipe"
  local z_image_tag="${z_recipe}:local-${BUD_NOW_STAMP}"

  # Validate recipe file exists
  test -f "${z_recipe_file}" || buc_die "Recipe not found: ${z_recipe_file}"

  buc_step "Building recipe locally: ${z_recipe}"
  rbw_show "Recipe file: ${z_recipe_file}"
  rbw_show "Image tag: ${z_image_tag}"

  # Build with docker (hardcoded for now - this heat is Docker-first)
  buc_step "Running: docker build -f ${z_recipe_file} -t ${z_image_tag} ."
  docker build -f "${z_recipe_file}" -t "${z_image_tag}" "${RBW_SCRIPT_DIR}/../.." \
    || buc_die "Docker build failed"

  buc_step "Build complete: ${z_image_tag}"
}

######################################################################
# Routing

rbw_route() {
  local z_command="${1:-}"
  shift || true
  local z_moniker="${1:-}"

  rbw_show "Routing command: ${z_command} with moniker: ${z_moniker}"

  # Verify BUD environment variables are present
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR not set - must be called from BUD"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP not set - must be called from BUD"

  rbw_show "BUD environment verified: TEMP_DIR=${BUD_TEMP_DIR}, NOW_STAMP=${BUD_NOW_STAMP}"

  # Route based on command
  case "${z_command}" in

    # Service lifecycle
    rbw-s)
      test -n "${z_moniker}" || buc_die "rbw-s requires moniker argument"
      rbw_load_nameplate "${z_moniker}"
      rbw_cmd_start "${z_moniker}"
      ;;

    rbw-z)
      test -n "${z_moniker}" || buc_die "rbw-z requires moniker argument"
      rbw_load_nameplate "${z_moniker}"
      rbw_cmd_stop "${z_moniker}"
      ;;

    # Container connections
    rbw-S)
      test -n "${z_moniker}" || buc_die "rbw-S requires moniker argument"
      rbw_load_nameplate "${z_moniker}"
      rbw_cmd_connect_sentry "${z_moniker}"
      ;;

    rbw-C)
      test -n "${z_moniker}" || buc_die "rbw-C requires moniker argument"
      rbw_load_nameplate "${z_moniker}"
      rbw_cmd_connect_censer "${z_moniker}"
      ;;

    rbw-B)
      test -n "${z_moniker}" || buc_die "rbw-B requires moniker argument"
      rbw_load_nameplate "${z_moniker}"
      rbw_cmd_connect_bottle "${z_moniker}"
      ;;

    # Local build (uses recipe name, not moniker)
    rbw-lB)
      local z_recipe="${z_moniker}"  # Repurpose arg as recipe name
      test -n "${z_recipe}" || buc_die "rbw-lB requires recipe argument"
      rbw_cmd_local_build "${z_recipe}"
      ;;

    # Unknown command
    *)
      buc_die "Unknown command: ${z_command}\nAvailable: rbw-s, rbw-z, rbw-S, rbw-C, rbw-B, rbw-lB"
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
