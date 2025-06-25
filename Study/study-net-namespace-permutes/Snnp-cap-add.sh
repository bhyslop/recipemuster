#!/bin/bash

# Copyright 2024 Scale Invariant, Inc.
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

set -e  # Exit on error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "SNNP: Get constants from" ${SCRIPT_DIR}
source "$SCRIPT_DIR/Snnp-common.sh"

snnp_verify_machine_connection
snnp_cleanup_all

echo -e "${BOLD}Launching SENTRY container with bridging for internet${NC}"
podman -c ${MACHINE} run -d                              \
  --name ${SENTRY_CONTAINER}                             \
  --network bridge                                       \
  --privileged                                           \
  -p ${ENTRY_PORT_WORKSTATION}:${ENTRY_PORT_WORKSTATION} \
  ${SENTRY_REPO_PATH}:${SENTRY_IMAGE_TAG}

echo -e "${BOLD}Waiting for SENTRY container${NC}"
sleep 2
snnp_machine_ssh "podman ps | grep ${SENTRY_CONTAINER} || (echo 'Container not running' && exit 1)"

echo -e "${BOLD}Executing SENTRY namespace setup script${NC}"
echo "RSNS: Beginning sentry namespace setup"
echo "RSNS0: Getting SENTRY PID"

SENTRY_PID=$(podman -c ${MACHINE} inspect -f '{{.State.Pid}}' ${SENTRY_CONTAINER})
echo "RSNS1: SENTRY PID: ${SENTRY_PID}"

echo "RSNS2: Creating and configuring bridge"
snnp_machine_ssh_sudo ip link add name ${ENCLAVE_BRIDGE} type bridge
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BRIDGE} up

echo "RSNS3: Creating and configuring SENTRY veth pair"
snnp_machine_ssh_sudo ip link add ${ENCLAVE_SENTRY_OUT} type veth peer name ${ENCLAVE_SENTRY_IN}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_SENTRY_IN} netns ${SENTRY_PID}
snnp_machine_ssh_sudo nsenter -t ${SENTRY_PID} -n ip link set ${ENCLAVE_SENTRY_IN} name eth1
snnp_machine_ssh_sudo nsenter -t ${SENTRY_PID} -n ip addr add ${ENCLAVE_SENTRY_IP}/${ENCLAVE_NETMASK} dev eth1
snnp_machine_ssh_sudo nsenter -t ${SENTRY_PID} -n ip link set eth1 up

echo "RSNS4: Connecting SENTRY veth to bridge"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_SENTRY_OUT} master ${ENCLAVE_BRIDGE}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_SENTRY_OUT} up
echo "RSNS: Sentry namespace setup complete"

echo -e "${BOLD}Configuring SENTRY security${NC}"
echo "RBS: SKIPPING sentry setup script"

echo "RBNS-ALT: Creating network namespace manually"
snnp_machine_ssh_sudo ip netns add ${NET_NAMESPACE}

echo "RBNS-ALT: Creating veth pair"
snnp_machine_ssh_sudo ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}

echo "RBNS-ALT: Moving veth endpoint to namespace"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_IN} netns ${NET_NAMESPACE}

echo "RBNS-ALT: Configuring interfaces in namespace"
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip link set ${ENCLAVE_BOTTLE_IN} name eth1
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip link set eth1 up
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip link set lo up

echo "RBNS-ALT: Connecting veth to bridge"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} up

echo "RBNS-ALT: Setting default route in namespace"
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip route add default via ${ENCLAVE_SENTRY_IP}

echo "RBNS-ALT: Check names after..."
podman machine ssh ${MACHINE} ip link show
podman machine ssh ${MACHINE} ip netns list

echo -e "${BOLD}Verifying network namespace permissions${NC}"
snnp_machine_ssh "echo 'Listing /var/run/netns:' && ls -l /var/run/netns"
snnp_machine_ssh "echo 'Detailed permissions for ${NET_NAMESPACE}:' && stat /var/run/netns/${NET_NAMESPACE}"

echo -e "${CYAN}EXPECT NEXT TO FAIL WITH -> Error: crun: cannot setns '/var/run/netns/nsproto-ns': Operation not permitted: OCI permission denied${NC}"

# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: 2025-06-24 16:45:32 PDT
# 
# Podman Version: 5.3.2 (client) / 5.3.1 (server) (Built: Wed Jan 22 05:42:46 2025)
# VM Build Date: 2024-11-17 16:00:00.000000000 -0800
# Mode: Rootless (Rootful: false)
# 
# Command: podman -c pdvm-rbw run -d --name nsproto-bottle --network ns:/var/run/netns/nsproto-ns --cap-add=SYS_ADMIN --cap-add=NET_ADMIN ghcr.io/bhyslop/recipemuster:bottle_ubuntu_test.20241207__190758
# 
# Expected Error from Next Command:
# Error: crun: cannot setns '/var/run/netns/nsproto-ns': Operation not permitted: OCI permission denied
# =============================================================================

# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: 2025-06-24 17:44:15 PDT
# 
# Podman Version: 5.3.2 (client) / 5.3.1 (server) (Built: Wed Jan 22 05:42:46 2025)
# VM Build Date: 2024-11-17 16:00:00.000000000 -0800
# Mode: Rootful (Rootful: true)
# 
# Command: podman -c pdvm-rbw run -d --name nsproto-bottle --network ns:/var/run/netns/nsproto-ns --cap-add=SYS_ADMIN --cap-add=NET_ADMIN ghcr.io/bhyslop/recipemuster:bottle_ubuntu_test.20241207__190758
# 
# Expected Error from Next Command:
# Error: crun: cannot setns '/var/run/netns/nsproto-ns': Operation not permitted: OCI permission denied
# =============================================================================

echo "RBNS-ALT: Starting container with the prepared network namespace"
podman -c ${MACHINE} run -d                       \
    --name ${BOTTLE_CONTAINER}                    \
    --network ns:/var/run/netns/${NET_NAMESPACE}  \
    --cap-add=SYS_ADMIN --cap-add=NET_ADMIN       \
    ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}

echo -e "${BOLD}Visualizing network setup in podman machine...${NC}"
echo "RBNI: Network interface information"
snnp_machine_ssh ip a

echo "RBNI: Network bridge information" 
snnp_machine_ssh ip link show type bridge

echo "RBNI: Network namespace information"
snnp_machine_ssh ip netns list

echo "RBNI: Route information"
snnp_machine_ssh ip route

echo -e "${BOLD}Verifying containers${NC}"
podman -c ${MACHINE} ps -a

echo -e "${GREEN}${BOLD}Setup script execution complete${NC}"

