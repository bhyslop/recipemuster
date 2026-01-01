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
#   rbw-o   Observe network traffic (tcpdump)
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
source "${RBW_SCRIPT_DIR}/rboo_observe.sh"

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

  # Load RBRR (repository regime) - config loading belongs in workbench per BCG pattern
  rbw_show "Loading RBRR"
  local z_rbrr_file="${RBW_SCRIPT_DIR}/../../rbrr_RecipeBottleRegimeRepo.sh"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"
  source "${z_rbrr_file}" || buc_die "Failed to source RBRR config: ${z_rbrr_file}"
  zrbrr_kindle

  # Kindle RBOB (validates RBRN and RBRR are ready)
  rbw_show "Kindling RBOB"
  zrbob_kindle
}

######################################################################
# Non-RBOB Commands

rbw_local_build() {
  local z_recipe="${1:-}"
  local z_recipe_file="${RBW_SCRIPT_DIR}/../../RBM-recipes/${z_recipe}.recipe"
  local z_image_tag="${z_recipe}:local-${BUD_NOW_STAMP}"

  test -f "${z_recipe_file}" || buc_die "Recipe not found: ${z_recipe_file}"

  buc_step "Building recipe locally: ${z_recipe}"
  rbw_show "Recipe file: ${z_recipe_file}"
  rbw_show "Image tag: ${z_image_tag}"

  docker build -f "${z_recipe_file}" -t "${z_image_tag}" "${RBW_SCRIPT_DIR}/../.." \
    || buc_die "Docker build failed"

  buc_step "Build complete: ${z_image_tag}"
}

######################################################################
# Routing (two-phase: load config, then execute)

rbw_route() {
  local z_command="${1:-}"
  shift || true
  local z_moniker="${1:-}"

  rbw_show "Routing command: ${z_command} with moniker: ${z_moniker}"

  # Verify BUD environment
  test -n "${BUD_TEMP_DIR:-}" || buc_die "BUD_TEMP_DIR not set - must be called from BUD"
  test -n "${BUD_NOW_STAMP:-}" || buc_die "BUD_NOW_STAMP not set - must be called from BUD"

  # Phase 1: Load nameplate for commands that need it
  case "${z_command}" in
    rbw-s|rbw-z|rbw-S|rbw-C|rbw-B|rbw-o)
      test -n "${z_moniker}" || buc_die "${z_command} requires moniker argument"
      rbw_load_nameplate "${z_moniker}"
      ;;
    rbw-lB)
      test -n "${z_moniker}" || buc_die "rbw-lB requires recipe argument"
      ;;
    *) buc_die "Unknown command: ${z_command}" ;;
  esac

  # Phase 1b: Kindle additional modules as needed
  case "${z_command}" in
    rbw-o) zrboo_kindle ;;
  esac

  # Phase 2: Execute command
  case "${z_command}" in
    rbw-s)  rbob_start ;;
    rbw-z)  rbob_stop ;;
    rbw-S)  rbob_connect_sentry ;;
    rbw-C)  rbob_connect_censer ;;
    rbw-B)  rbob_connect_bottle ;;
    rbw-o)  rboo_observe ;;
    rbw-lB) rbw_local_build "${z_moniker}" ;;
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
