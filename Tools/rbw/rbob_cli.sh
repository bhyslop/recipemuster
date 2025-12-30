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
# RBOB CLI - Recipe Bottle Orchestration Bottle command-line interface
#
# Commands:
#   info <moniker>  Show container names, network, and runtime for nameplate

set -euo pipefail

ZRBOB_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBOB_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbrn_regime.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbob_bottle.sh"

######################################################################
# CLI Commands

rbob_validate() {
  zrbob_sentinel

  buc_doc_brief "Validate that RBOB configuration is complete and runnable"
  buc_doc_shown || return 0

  buc_step "RBOB Validate: ${RBRN_MONIKER}"

  # Verify container naming works (two-line pattern per BCG)
  local z_sentry
  z_sentry="$(zrbob_container_name sentry)" || buc_die "Failed to get sentry name"

  local z_censer
  z_censer="$(zrbob_container_name censer)" || buc_die "Failed to get censer name"

  local z_bottle
  z_bottle="$(zrbob_container_name bottle)" || buc_die "Failed to get bottle name"

  local z_network
  z_network="$(zrbob_network_name)" || buc_die "Failed to get network name"

  # Verify runtime command
  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)" || buc_die "Failed to get runtime command"

  # Verify sentry script exists
  local z_sentry_script="${ZRBOB_CLI_SCRIPT_DIR}/rbss.sentry.sh"
  test -f "${z_sentry_script}" || buc_die "Sentry script not found: ${z_sentry_script}"

  buc_step "RBOB configuration valid"
  echo "Moniker:   ${RBRN_MONIKER}"
  echo "Runtime:   ${z_runtime}"
  echo "Sentry:    ${z_sentry}"
  echo "Censer:    ${z_censer}"
  echo "Bottle:    ${z_bottle}"
  echo "Network:   ${z_network}"
}

rbob_info() {
  zrbob_sentinel

  buc_doc_brief "Show container names, network, and runtime for kindled nameplate"
  buc_doc_shown || return 0

  buc_step "RBOB Info: ${RBRN_MONIKER}"
  echo "Runtime:   ${RBRN_RUNTIME}"
  echo "Sentry:    $(zrbob_container_name sentry)"
  echo "Censer:    $(zrbob_container_name censer)"
  echo "Bottle:    $(zrbob_container_name bottle)"
  echo "Network:   $(zrbob_network_name)"
  echo "Sentry IP: ${RBRN_ENCLAVE_SENTRY_IP}"
  echo "Bottle IP: ${RBRN_ENCLAVE_BOTTLE_IP}"
}

######################################################################
# Furnish and Main

zrbob_furnish() {
  buc_doc_env "RBOB_MONIKER        " "Nameplate moniker (e.g., nsproto)"

  local z_moniker="${RBOB_MONIKER:-}"
  test -n "${z_moniker}" || buc_die "RBOB_MONIKER environment variable required"

  # Load nameplate
  local z_nameplate_file="${ZRBOB_CLI_SCRIPT_DIR}/rbrn_${z_moniker}.env"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die "Failed to source nameplate"
  zrbrn_kindle

  # Load RBRR - config loading belongs in furnish per BCG pattern
  local z_rbrr_file="${ZRBOB_CLI_SCRIPT_DIR}/../../rbrr_RecipeBottleRegimeRepo.sh"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"
  source "${z_rbrr_file}" || buc_die "Failed to source RBRR config: ${z_rbrr_file}"
  zrbrr_kindle

  # Kindle RBOB (validates RBRN and RBRR are ready)
  zrbob_kindle
}

buc_execute rbob_ "Recipe Bottle Orchestration" zrbob_furnish "$@"

# eof
