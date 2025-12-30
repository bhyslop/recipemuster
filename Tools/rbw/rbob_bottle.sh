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

  ZRBOB_KINDLED=1
}

zrbob_sentinel() {
  test "${ZRBOB_KINDLED:-}" = "1" || buc_die "Module rbob not kindled - call zrbob_kindle first"
}

######################################################################
# Naming Helpers

# Get container name for a given type
# Usage: zrbob_container_name <type>  # type: sentry, censer, bottle
zrbob_container_name() {
  zrbob_sentinel
  local z_type="${1:-}"
  test -n "${z_type}" || buc_die "zrbob_container_name: type argument required"
  echo "${RBRN_MONIKER}-${z_type}"
}

# Get enclave network name
zrbob_network_name() {
  zrbob_sentinel
  echo "${RBRN_MONIKER}-enclave"
}

# Get runtime command (docker or podman)
zrbob_runtime_cmd() {
  zrbob_sentinel
  case "${RBRN_RUNTIME}" in
    docker) echo "docker" ;;
    podman) echo "podman" ;;  # Future: add -c connection if needed
    *) buc_die "Unknown RBRN_RUNTIME: ${RBRN_RUNTIME}" ;;
  esac
}

######################################################################
# Variable Setters (populate module variables without subshell)

# Set ZRBOB_CENSER_NETWORK_ARGS for runtime-specific network syntax
# Docker uses --ip separately, Podman uses network:ip= syntax
zrbob_set_censer_network_args() {
  zrbob_sentinel

  local z_network="${1:-}"
  local z_ip="${2:-}"

  test -n "${z_network}" || buc_die "zrbob_set_censer_network_args: network required"
  test -n "${z_ip}" || buc_die "zrbob_set_censer_network_args: ip required"

  case "${RBRN_RUNTIME}" in
    docker) ZRBOB_CENSER_NETWORK_ARGS="--network ${z_network} --ip ${z_ip}" ;;
    podman) ZRBOB_CENSER_NETWORK_ARGS="--network ${z_network}:ip=${z_ip}" ;;
    *) buc_die "Unknown RBRN_RUNTIME: ${RBRN_RUNTIME}" ;;
  esac
}

######################################################################
# Wait Helper

# Wait for container to be running
# Usage: zrbob_wait_container <container_name> [timeout_seconds]
zrbob_wait_container() {
  local z_container="${1:-}"
  local z_timeout="${2:-10}"
  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  test -n "${z_container}" || buc_die "zrbob_wait_container: container name required"

  buc_info "Waiting for container: ${z_container} (timeout: ${z_timeout}s)"

  local z_elapsed=0
  while [[ ${z_elapsed} -lt ${z_timeout} ]]; do
    if ${z_runtime} ps --format '{{.Names}}' 2>/dev/null | grep -q "^${z_container}$"; then
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
  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  local z_sentry z_censer z_bottle
  z_sentry="$(zrbob_container_name sentry)"
  z_censer="$(zrbob_container_name censer)"
  z_bottle="$(zrbob_container_name bottle)"

  buc_step "Stopping any prior containers"

  # Stop with short timeout, then force remove (ignore errors for missing containers)
  ${z_runtime} stop -t 2 "${z_sentry}" 2>/dev/null || true
  ${z_runtime} rm   -f    "${z_sentry}" 2>/dev/null || true

  ${z_runtime} stop -t 2 "${z_bottle}" 2>/dev/null || true
  ${z_runtime} rm   -f    "${z_bottle}" 2>/dev/null || true

  ${z_runtime} stop -t 2 "${z_censer}" 2>/dev/null || true
  ${z_runtime} rm   -f    "${z_censer}" 2>/dev/null || true
}

# Remove enclave network (tolerates missing)
zrbob_cleanup_network() {
  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  local z_network
  z_network="$(zrbob_network_name)"

  buc_step "Removing any existing enclave network"
  ${z_runtime} network rm -f "${z_network}" 2>/dev/null || true
}

######################################################################
# Network Creation

# Create the internal enclave network with subnet
zrbob_create_network() {
  zrbob_sentinel

  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  local z_network
  z_network="$(zrbob_network_name)"

  buc_step "Creating enclave network: ${z_network}"

  ${z_runtime} network create \
    --internal \
    --subnet="${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
    "${z_network}" \
    || buc_die "Failed to create enclave network"

  buc_info "Enclave network created: ${z_network} (${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK})"
}

######################################################################
# Sentry Launch

# Launch sentry container: bridge network, privileged, env vars, connect to enclave, configure security
zrbob_launch_sentry() {
  zrbob_sentinel

  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  local z_sentry z_network
  z_sentry="$(zrbob_container_name sentry)"
  z_network="$(zrbob_network_name)"

  local z_image="${RBRN_SENTRY_REPO_PATH}:${RBRN_SENTRY_IMAGE_TAG}"

  buc_step "Launching sentry container: ${z_sentry}"

  # Build port mapping args if entry is enabled
  local z_port_args=()
  if [[ "${RBRN_ENTRY_ENABLED}" == "1" ]]; then
    z_port_args+=("-p" "${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_WORKSTATION}")
  fi

  # Run sentry on bridge network with env vars
  ${z_runtime} run -d \
    --name "${z_sentry}" \
    --network bridge \
    --privileged \
    "${z_port_args[@]}" \
    "${ZRBRR_DOCKER_ENV[@]}" \
    "${ZRBRN_DOCKER_ENV[@]}" \
    "${z_image}" \
    || buc_die "Failed to launch sentry"

  # Wait for sentry to be running
  zrbob_wait_container "${z_sentry}" 5

  # Connect sentry to enclave network with specific IP
  buc_step "Connecting sentry to enclave network"
  ${z_runtime} network connect \
    --ip "${RBRN_ENCLAVE_SENTRY_IP}" \
    "${z_network}" \
    "${z_sentry}" \
    || buc_die "Failed to connect sentry to enclave"

  # Verify sentry got expected IP
  buc_step "Verifying sentry enclave IP"
  local z_actual_ip
  z_actual_ip=$(${z_runtime} inspect "${z_sentry}" \
    --format "{{(index .NetworkSettings.Networks \"${z_network}\").IPAddress}}")

  if [[ "${z_actual_ip}" != "${RBRN_ENCLAVE_SENTRY_IP}" ]]; then
    buc_die "Sentry IP mismatch. Expected ${RBRN_ENCLAVE_SENTRY_IP}, got ${z_actual_ip}"
  fi
  buc_info "Sentry enclave IP verified: ${z_actual_ip}"

  # Configure sentry security by exec'ing rbss.sentry.sh
  buc_step "Configuring sentry security"
  local z_sentry_script="${ZRBOB_SCRIPT_DIR}/rbss.sentry.sh"
  test -f "${z_sentry_script}" || buc_die "Sentry script not found: ${z_sentry_script}"

  ${z_runtime} exec -i "${z_sentry}" /bin/sh < "${z_sentry_script}" \
    || buc_die "Failed to configure sentry security"

  buc_info "Sentry launched and configured: ${z_sentry}"
}

######################################################################
# Censer Launch

# Launch censer container: enclave network, configure routing
zrbob_launch_censer() {
  zrbob_sentinel

  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  local z_censer z_network
  z_censer="$(zrbob_container_name censer)"
  z_network="$(zrbob_network_name)"

  local z_image="${RBRN_SENTRY_REPO_PATH}:${RBRN_SENTRY_IMAGE_TAG}"

  buc_step "Launching censer container: ${z_censer}"

  # Set runtime-specific network args (populates ZRBOB_CENSER_NETWORK_ARGS)
  zrbob_set_censer_network_args "${z_network}" "${RBRN_ENCLAVE_BOTTLE_IP}"

  # Run censer on enclave network with bottle IP, sleep infinity
  ${z_runtime} run -d \
    --name "${z_censer}" \
    ${ZRBOB_CENSER_NETWORK_ARGS} \
    --privileged \
    --entrypoint /bin/sleep \
    "${z_image}" \
    infinity \
    || buc_die "Failed to launch censer"

  # Wait for censer to be running
  zrbob_wait_container "${z_censer}" 5

  # Configure censer: set resolv.conf to use sentry as DNS
  buc_step "Configuring censer DNS"
  ${z_runtime} exec "${z_censer}" sh -c \
    "echo 'nameserver ${RBRN_ENCLAVE_SENTRY_IP}' > /etc/resolv.conf" \
    || buc_die "Failed to configure censer DNS"

  # Flush ARP and restart networking
  buc_step "Flushing censer ARP entries"
  ${z_runtime} exec "${z_censer}" sh -c \
    "ip link set eth0 down && ip link set eth0 up && ip -s -s neigh flush all" \
    || buc_die "Failed to flush censer ARP"

  # Configure default route through sentry
  buc_step "Configuring censer default route"
  ${z_runtime} exec "${z_censer}" sh -c \
    "ip route add default via ${RBRN_ENCLAVE_SENTRY_IP}" \
    || buc_die "Failed to set censer default route"

  # Verify default route
  ${z_runtime} exec "${z_censer}" sh -c \
    "ip route | grep -q '^default via ${RBRN_ENCLAVE_SENTRY_IP}'" \
    || buc_die "Failed to verify censer default route"

  buc_info "Censer launched and configured: ${z_censer}"
}

######################################################################
# Bottle Launch

# Create and start bottle container using censer's network namespace
zrbob_launch_bottle() {
  zrbob_sentinel

  local z_runtime
  z_runtime="$(zrbob_runtime_cmd)"

  local z_bottle z_censer
  z_bottle="$(zrbob_container_name bottle)"
  z_censer="$(zrbob_container_name censer)"

  local z_image="${RBRN_BOTTLE_REPO_PATH}:${RBRN_BOTTLE_IMAGE_TAG}"

  buc_step "Creating bottle container: ${z_bottle}"

  # Create bottle sharing censer's network namespace
  ${z_runtime} create \
    --name "${z_bottle}" \
    --net=container:"${z_censer}" \
    --security-opt label=disable \
    "${z_image}" \
    || buc_die "Failed to create bottle"

  buc_step "Starting bottle container"
  ${z_runtime} start "${z_bottle}" \
    || buc_die "Failed to start bottle"

  # Wait for bottle to be running
  zrbob_wait_container "${z_bottle}" 5

  buc_info "Bottle launched: ${z_bottle}"
}

######################################################################
# Public API

# Start the complete bottle service (sentry + censer + bottle)
# Requires: RBOB kindled (which requires RBRN and RBRR)
rbob_start() {
  zrbob_sentinel

  buc_step "Starting bottle service: ${RBRN_MONIKER}"

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

# eof
