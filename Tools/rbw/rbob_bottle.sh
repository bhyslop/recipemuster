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
# RBOB - Recipe Bottle Orchestration Bottle
# Container lifecycle management for bottle services (sentry + censer + bottle)
#
# Requires: buc_command.sh sourced
# Requires: rbrn_regime.sh sourced
# Requires: rbrr_regime.sh sourced

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBOB_SOURCED:-}" || buc_die "Module rbob multiply sourced - check sourcing hierarchy"
ZRBOB_SOURCED=1

# Store script directory for locating sibling files (rbss.sentry.sh)
ZRBOB_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Project root directory (two levels up from Tools/rbw/)
ZRBOB_PROJECT_ROOT="${ZRBOB_SCRIPT_DIR}/../.."

######################################################################
# Kindle and Sentinel

zrbob_kindle() {
  test -z "${ZRBOB_KINDLED:-}" || buc_die "Module rbob already kindled"

  # Verify RBRN regime is kindled (provides nameplate config)
  zrbrn_sentinel

  # Verify RBRR regime is kindled (provides repo config like DNS_SERVER)
  zrbrr_sentinel

  # Runtime command (docker or podman)
  case "${RBRN_RUNTIME}" in
    docker) ZRBOB_RUNTIME="docker" ;;
    podman) ZRBOB_RUNTIME="podman" ;;
    *) buc_die "Unknown RBRN_RUNTIME: ${RBRN_RUNTIME}" ;;
  esac

  # Container names
  ZRBOB_SENTRY="${RBRN_MONIKER}-sentry"
  ZRBOB_CENSER="${RBRN_MONIKER}-censer"
  ZRBOB_BOTTLE="${RBRN_MONIKER}-bottle"

  # Network name
  ZRBOB_NETWORK="${RBRN_MONIKER}-enclave"

  # Sentry configuration script
  ZRBOB_SENTRY_SCRIPT="${ZRBOB_SCRIPT_DIR}/rbss.sentry.sh"
  test -f "${ZRBOB_SENTRY_SCRIPT}" || buc_die "Sentry script not found: ${ZRBOB_SENTRY_SCRIPT}"

  # Runtime-specific censer network args
  # Docker uses --ip separately, Podman uses network:ip= syntax
  case "${RBRN_RUNTIME}" in
    docker) ZRBOB_CENSER_NETWORK_ARGS="--network ${ZRBOB_NETWORK} --ip ${RBRN_ENCLAVE_BOTTLE_IP}" ;;
    podman) ZRBOB_CENSER_NETWORK_ARGS="--network ${ZRBOB_NETWORK}:ip=${RBRN_ENCLAVE_BOTTLE_IP}" ;;
  esac

  # GAR image references (computed once, used by launch and preflight)
  local z_gar_base="${RBGD_GAR_LOCATION}${RBGC_GAR_HOST_SUFFIX}/${RBGD_GAR_PROJECT_ID}/${RBRR_GAR_REPOSITORY}"
  ZRBOB_SENTRY_IMAGE="${z_gar_base}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_CONSECRATION}${RBGC_ARK_SUFFIX_IMAGE}"
  ZRBOB_BOTTLE_IMAGE="${z_gar_base}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_CONSECRATION}${RBGC_ARK_SUFFIX_IMAGE}"

  ZRBOB_KINDLED=1
}

zrbob_sentinel() {
  test "${ZRBOB_KINDLED:-}" = "1" || buc_die "Module rbob not kindled - call zrbob_kindle first"
}

######################################################################
# Wait Helper

# Wait for container to be running
# Usage: zrbob_wait_container <container_name> [timeout_seconds]
zrbob_wait_container() {
  local z_container="${1:-}"
  local z_timeout="${2:-10}"

  test -n "${z_container}" || buc_die "zrbob_wait_container: container name required"

  buc_info "Waiting for container: ${z_container} (timeout: ${z_timeout}s)"

  local z_elapsed=0
  while [[ ${z_elapsed} -lt ${z_timeout} ]]; do
    if ${ZRBOB_RUNTIME} ps --format '{{.Names}}' 2>/dev/null | grep -q "^${z_container}$"; then
      buc_info "Container running: ${z_container}"
      return 0
    fi
    sleep 1
    z_elapsed=$((z_elapsed + 1))
  done

  buc_die "Timeout waiting for container: ${z_container}"
}

######################################################################
# Cleanup Helpers

# Stop and remove containers (tolerates missing)
zrbob_cleanup_containers() {
  buc_step "Stopping any prior containers"

  # Stop with short timeout, then force remove (ignore errors for missing containers)
  ${ZRBOB_RUNTIME} stop -t 2 "${ZRBOB_SENTRY}" 2>/dev/null || true
  ${ZRBOB_RUNTIME} rm   -f    "${ZRBOB_SENTRY}" 2>/dev/null || true

  ${ZRBOB_RUNTIME} stop -t 2 "${ZRBOB_BOTTLE}" 2>/dev/null || true
  ${ZRBOB_RUNTIME} rm   -f    "${ZRBOB_BOTTLE}" 2>/dev/null || true

  ${ZRBOB_RUNTIME} stop -t 2 "${ZRBOB_CENSER}" 2>/dev/null || true
  ${ZRBOB_RUNTIME} rm   -f    "${ZRBOB_CENSER}" 2>/dev/null || true
}

# Remove enclave network (tolerates missing)
zrbob_cleanup_network() {
  buc_step "Removing any existing enclave network"
  ${ZRBOB_RUNTIME} network rm -f "${ZRBOB_NETWORK}" 2>/dev/null || true
}

######################################################################
# Network Creation

# Create the enclave network with subnet
# Note: Docker's --internal blocks ALL traffic leaving the network, including through
# dual-homed containers. Podman's --internal allows forwarding through containers.
# For Docker, we omit --internal and rely on sentry's iptables for isolation.
zrbob_create_network() {
  zrbob_sentinel

  buc_step "Creating enclave network: ${ZRBOB_NETWORK}"

  local z_internal_flag=""
  if [[ "${RBRN_RUNTIME}" == "podman" ]]; then
    z_internal_flag="--internal"
  fi

  ${ZRBOB_RUNTIME} network create \
    ${z_internal_flag} \
    --subnet="${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
    "${ZRBOB_NETWORK}" \
    || buc_die "Failed to create enclave network"

  buc_info "Enclave network created: ${ZRBOB_NETWORK} (${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK})"
}

######################################################################
# Sentry Launch

# Launch sentry container: bridge network, privileged, env vars, connect to enclave, configure security
zrbob_launch_sentry() {
  zrbob_sentinel

  buc_step "Launching sentry container: ${ZRBOB_SENTRY}"

  # Build port mapping args if entry is enabled
  local z_port_args=()
  if [[ "${RBRN_ENTRY_MODE}" == "enabled" ]]; then
    z_port_args+=("-p" "${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_WORKSTATION}")
  fi

  # Run sentry on bridge network with env vars
  ${ZRBOB_RUNTIME} run -d \
    --name "${ZRBOB_SENTRY}" \
    --network bridge \
    --privileged \
    "${z_port_args[@]}" \
    "${ZRBRR_DOCKER_ENV[@]}" \
    "${ZRBRN_DOCKER_ENV[@]}" \
    "${ZRBOB_SENTRY_IMAGE}" \
    || buc_die "Failed to launch sentry"

  # Wait for sentry to be running
  zrbob_wait_container "${ZRBOB_SENTRY}" 5

  # Connect sentry to enclave network with specific IP
  buc_step "Connecting sentry to enclave network"
  ${ZRBOB_RUNTIME} network connect \
    --ip "${RBRN_ENCLAVE_SENTRY_IP}" \
    "${ZRBOB_NETWORK}" \
    "${ZRBOB_SENTRY}" \
    || buc_die "Failed to connect sentry to enclave"

  # Verify sentry got expected IP
  buc_step "Verifying sentry enclave IP"
  local z_actual_ip
  z_actual_ip=$(${ZRBOB_RUNTIME} inspect "${ZRBOB_SENTRY}" \
    --format "{{(index .NetworkSettings.Networks \"${ZRBOB_NETWORK}\").IPAddress}}")

  if [[ "${z_actual_ip}" != "${RBRN_ENCLAVE_SENTRY_IP}" ]]; then
    buc_die "Sentry IP mismatch. Expected ${RBRN_ENCLAVE_SENTRY_IP}, got ${z_actual_ip}"
  fi
  buc_info "Sentry enclave IP verified: ${z_actual_ip}"

  # Configure sentry security by exec'ing rbss.sentry.sh
  buc_step "Configuring sentry security"
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_SENTRY}" /bin/sh < "${ZRBOB_SENTRY_SCRIPT}" \
    || buc_die "Failed to configure sentry security"

  buc_info "Sentry launched and configured: ${ZRBOB_SENTRY}"
}

######################################################################
# Censer Launch

# Launch censer container: enclave network, configure routing
zrbob_launch_censer() {
  zrbob_sentinel

  buc_step "Launching censer container: ${ZRBOB_CENSER}"

  # Run censer on enclave network with bottle IP, sleep infinity
  # ZRBOB_CENSER_NETWORK_ARGS computed at kindle time
  ${ZRBOB_RUNTIME} run -d \
    --name "${ZRBOB_CENSER}" \
    ${ZRBOB_CENSER_NETWORK_ARGS} \
    --privileged \
    --entrypoint /bin/sleep \
    "${ZRBOB_SENTRY_IMAGE}" \
    infinity \
    || buc_die "Failed to launch censer"

  # Wait for censer to be running
  zrbob_wait_container "${ZRBOB_CENSER}" 5

  # Configure censer: set resolv.conf to use sentry as DNS
  buc_step "Configuring censer DNS"
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" sh -c \
    "echo 'nameserver ${RBRN_ENCLAVE_SENTRY_IP}' > /etc/resolv.conf" \
    || buc_die "Failed to configure censer DNS"

  # Flush ARP and restart networking
  buc_step "Flushing censer ARP entries"
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" sh -c \
    "ip link set eth0 down && ip link set eth0 up && ip -s -s neigh flush all" \
    || buc_die "Failed to flush censer ARP"

  # Configure default route through sentry
  buc_step "Configuring censer default route"
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" sh -c \
    "ip route add default via ${RBRN_ENCLAVE_SENTRY_IP}" \
    || buc_die "Failed to set censer default route"

  # Verify default route
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" sh -c \
    "ip route | grep -q '^default via ${RBRN_ENCLAVE_SENTRY_IP}'" \
    || buc_die "Failed to verify censer default route"

  buc_info "Censer launched and configured: ${ZRBOB_CENSER}"
}

######################################################################
# Bottle Launch

# Create and start bottle container using censer's network namespace
zrbob_launch_bottle() {
  zrbob_sentinel

  buc_step "Creating bottle container: ${ZRBOB_BOTTLE}"

  # Create bottle sharing censer's network namespace
  ${ZRBOB_RUNTIME} create \
    --name "${ZRBOB_BOTTLE}" \
    --net=container:"${ZRBOB_CENSER}" \
    --security-opt label=disable \
    "${ZRBOB_BOTTLE_IMAGE}" \
    || buc_die "Failed to create bottle"

  buc_step "Starting bottle container"
  ${ZRBOB_RUNTIME} start "${ZRBOB_BOTTLE}" \
    || buc_die "Failed to start bottle"

  # Wait for bottle to be running
  zrbob_wait_container "${ZRBOB_BOTTLE}" 5

  buc_info "Bottle launched: ${ZRBOB_BOTTLE}"
}

######################################################################
# Public API

# Start the complete bottle service (sentry + censer + bottle)
# Requires: RBOB kindled (which requires RBRN and RBRR)
rbob_start() {
  zrbob_sentinel

  buc_step "Starting bottle service: ${RBRN_MONIKER}"

  # Cross-nameplate validation (silent on success, dies on conflict)
  rbrn_preflight

  # Preflight: verify container images exist locally before touching anything
  ${ZRBOB_RUNTIME} image inspect "${ZRBOB_SENTRY_IMAGE}" >/dev/null 2>&1 \
    || buc_die "Sentry image not found locally: ${ZRBOB_SENTRY_IMAGE}
  Run: tt/rbw-as.SummonArk.sh ${RBRN_SENTRY_VESSEL} ${RBRN_SENTRY_CONSECRATION}"

  ${ZRBOB_RUNTIME} image inspect "${ZRBOB_BOTTLE_IMAGE}" >/dev/null 2>&1 \
    || buc_die "Bottle image not found locally: ${ZRBOB_BOTTLE_IMAGE}
  Run: tt/rbw-as.SummonArk.sh ${RBRN_BOTTLE_VESSEL} ${RBRN_BOTTLE_CONSECRATION}"

  # Cleanup any prior state
  zrbob_cleanup_containers
  zrbob_cleanup_network

  # Create enclave network
  zrbob_create_network

  # Launch containers in sequence
  zrbob_launch_sentry
  zrbob_launch_censer
  zrbob_launch_bottle

  buc_step "Bottle service started: ${RBRN_MONIKER}"
}

# Stop the complete bottle service
rbob_stop() {
  zrbob_sentinel

  buc_step "Stopping bottle service: ${RBRN_MONIKER}"

  zrbob_cleanup_containers
  zrbob_cleanup_network

  buc_step "Bottle service stopped: ${RBRN_MONIKER}"
}

# Connect to sentry container (interactive shell)
rbob_connect_sentry() {
  zrbob_sentinel
  buc_step "Connecting to sentry: ${ZRBOB_SENTRY}"
  exec ${ZRBOB_RUNTIME} exec -it "${ZRBOB_SENTRY}" /bin/bash
}

# Connect to censer container (interactive shell)
rbob_connect_censer() {
  zrbob_sentinel
  buc_step "Connecting to censer: ${ZRBOB_CENSER}"
  exec ${ZRBOB_RUNTIME} exec -it "${ZRBOB_CENSER}" /bin/bash
}

# Connect to bottle container (interactive shell)
rbob_connect_bottle() {
  zrbob_sentinel
  buc_step "Connecting to bottle: ${ZRBOB_BOTTLE}"
  exec ${ZRBOB_RUNTIME} exec -it "${ZRBOB_BOTTLE}" /bin/bash
}

# eof
