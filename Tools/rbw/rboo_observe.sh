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
# RBOO - Recipe Bottle Orchestration Observe
# Network observation (tcpdump) for bottle services
#
# Requires: buc_command.sh sourced
# Requires: rbob_bottle.sh sourced and kindled

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBOO_SOURCED:-}" || buc_die "Module rboo multiply sourced - check sourcing hierarchy"
ZRBOO_SOURCED=1

######################################################################
# Kindle and Sentinel

zrboo_kindle() {
  test -z "${ZRBOO_KINDLED:-}" || buc_die "Module rboo already kindled"

  # Verify RBOB is kindled (provides container names, runtime)
  zrbob_sentinel

  # Terminal control sequences for colored output
  ZRBOO_BOLD=$(tput bold 2>/dev/null || echo "")
  ZRBOO_YELLOW=$(tput setaf 3 2>/dev/null || echo "")
  ZRBOO_BLUE=$(tput setaf 4 2>/dev/null || echo "")
  ZRBOO_WHITE=$(tput setaf 7 2>/dev/null || echo "")
  ZRBOO_RESET=$(tput sgr0 2>/dev/null || echo "")

  # Common tcpdump options: unbuffered, line-buffered, no name resolution, verbose
  ZRBOO_TCPDUMP_OPTS="-U -l -nn -vvv"

  # Bridge interface (only for podman, discovered at observe time)
  ZRBOO_BRIDGE_INTERFACE=""

  ZRBOO_KINDLED=1
}

zrboo_sentinel() {
  test "${ZRBOO_KINDLED:-}" = "1" || buc_die "Module rboo not kindled - call zrboo_kindle first"
}

######################################################################
# Output Prefixing Functions

zrboo_prefix_censer() {
  while IFS= read -r line; do
    echo "${ZRBOO_YELLOW}${ZRBOO_BOLD}[CENSER/BOTTLE]${ZRBOO_RESET} ${line}"
  done
}

zrboo_prefix_sentry() {
  while IFS= read -r line; do
    echo "${ZRBOO_WHITE}${ZRBOO_BOLD}[SENTRY]${ZRBOO_RESET} ${line}"
  done
}

zrboo_prefix_bridge() {
  while IFS= read -r line; do
    echo "${ZRBOO_BLUE}${ZRBOO_BOLD}[BRIDGE]${ZRBOO_RESET} ${line}"
  done
}

######################################################################
# Public API

# Observe network traffic on bottle service containers
# Runs until Ctrl+C; all captures run in parallel
rboo_observe() {
  zrboo_sentinel

  buc_step "Starting network observation: ${RBRN_MONIKER}"

  # Set up cleanup trap
  trap 'buc_info "Stopping captures..."; kill 0 2>/dev/null; exit 0' SIGINT SIGTERM

  buc_info "Network topology:"
  buc_info "  SENTRY: Bridge (eth0) <-> Internet, Enclave (eth1) <-> Internal"
  buc_info "  CENSER/BOTTLE: Shared namespace on Enclave network (eth0)"

  # Start censer capture (shared namespace with bottle)
  buc_info "Starting censer/bottle capture (eth0)"
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" tcpdump ${ZRBOO_TCPDUMP_OPTS} -i eth0 2>&1 | zrboo_prefix_censer &

  # Start sentry enclave capture
  buc_info "Starting sentry enclave capture (eth1)"
  ${ZRBOB_RUNTIME} exec "${ZRBOB_SENTRY}" tcpdump ${ZRBOO_TCPDUMP_OPTS} -i eth1 2>&1 | zrboo_prefix_sentry &

  # Bridge capture: only for podman (requires podman machine ssh)
  if [[ "${RBRN_RUNTIME}" == "podman" ]]; then
    # Discover bridge interface for enclave network
    ZRBOO_BRIDGE_INTERFACE=$(${ZRBOB_RUNTIME} network inspect "${ZRBOB_NETWORK}" --format '{{.NetworkInterface}}')
    buc_info "Starting bridge capture (${ZRBOO_BRIDGE_INTERFACE}) via podman machine ssh"
    ${ZRBOB_RUNTIME} machine ssh "sudo -n tcpdump ${ZRBOO_TCPDUMP_OPTS} -i ${ZRBOO_BRIDGE_INTERFACE}" 2>&1 | zrboo_prefix_bridge &
  else
    buc_info "Bridge capture not available for Docker runtime (requires podman machine ssh)"
  fi

  buc_info "Press Ctrl+C to stop captures"

  # Wait for all background processes
  wait
}

# eof
