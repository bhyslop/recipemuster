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
#   start           Start bottle service (sentry + censer + bottle)
#   stop            Stop bottle service
#   connect_sentry  Connect to sentry container
#   connect_censer  Connect to censer container
#   connect_bottle  Connect to bottle container
#   observe         Observe network traffic (tcpdump)
#   info            Show container names, network, and runtime
#   validate        Validate configuration is complete

set -euo pipefail

ZRBOB_CLI_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBOB_CLI_SCRIPT_DIR}/../buk/buc_command.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbrn_regime.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbrr_regime.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbcc_Constants.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rbob_bottle.sh"
source "${ZRBOB_CLI_SCRIPT_DIR}/rboo_observe.sh"

######################################################################
# CLI Commands

rbob_validate() {
  zrbob_sentinel

  buc_doc_brief "Validate that RBOB configuration is complete and runnable"
  buc_doc_shown || return 0

  buc_step "RBOB Validate: ${RBRN_MONIKER}"

  # All values computed at kindle - just verify they exist
  test -n "${ZRBOB_RUNTIME}" || buc_die "ZRBOB_RUNTIME not set"
  test -n "${ZRBOB_SENTRY}" || buc_die "ZRBOB_SENTRY not set"
  test -n "${ZRBOB_CENSER}" || buc_die "ZRBOB_CENSER not set"
  test -n "${ZRBOB_BOTTLE}" || buc_die "ZRBOB_BOTTLE not set"
  test -n "${ZRBOB_NETWORK}" || buc_die "ZRBOB_NETWORK not set"
  test -f "${ZRBOB_SENTRY_SCRIPT}" || buc_die "Sentry script not found: ${ZRBOB_SENTRY_SCRIPT}"

  buc_step "RBOB configuration valid"
  echo "Moniker:   ${RBRN_MONIKER}"
  echo "Runtime:   ${ZRBOB_RUNTIME}"
  echo "Sentry:    ${ZRBOB_SENTRY}"
  echo "Censer:    ${ZRBOB_CENSER}"
  echo "Bottle:    ${ZRBOB_BOTTLE}"
  echo "Network:   ${ZRBOB_NETWORK}"
}

rbob_info() {
  zrbob_sentinel

  buc_doc_brief "Show container names, network, and runtime for kindled nameplate"
  buc_doc_shown || return 0

  buc_step "RBOB Info: ${RBRN_MONIKER}"
  echo "Runtime:   ${ZRBOB_RUNTIME}"
  echo "Sentry:    ${ZRBOB_SENTRY}"
  echo "Censer:    ${ZRBOB_CENSER}"
  echo "Bottle:    ${ZRBOB_BOTTLE}"
  echo "Network:   ${ZRBOB_NETWORK}"
  echo "Sentry IP: ${RBRN_ENCLAVE_SENTRY_IP}"
  echo "Bottle IP: ${RBRN_ENCLAVE_BOTTLE_IP}"
}

rbob_observe() {
  zrbob_sentinel

  buc_doc_brief "Observe network traffic on bottle service containers"
  buc_doc_shown || return 0

  # Kindle observe module and delegate
  zrboo_kindle
  rboo_observe
}

######################################################################
# Furnish and Main

zrbob_furnish() {
  buc_doc_env "RBOB_MONIKER        " "Nameplate moniker (e.g., nsproto)"

  local z_moniker="${RBOB_MONIKER:-}"
  test -n "${z_moniker}" || buc_die "RBOB_MONIKER environment variable required"

  zrbcc_kindle

  # Load nameplate
  rbrn_load_moniker "${z_moniker}"

  # Load RBRR - config loading belongs in furnish per BCG pattern
  rbrr_load

  # Kindle RBOB (validates RBRN and RBRR are ready)
  zrbob_kindle
}

buc_execute rbob_ "Recipe Bottle Orchestration" zrbob_furnish "$@"

# eof
