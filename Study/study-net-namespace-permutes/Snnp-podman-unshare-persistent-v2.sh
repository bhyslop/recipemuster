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

echo -e "${BOLD}Using podman unshare to create persistent network namespace (v2)${NC}"
echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Creating persistent network namespace using /proc/PID/ns/net"

# Create a script that will run inside podman unshare and keep the namespace alive
snnp_machine_ssh "cat > /tmp/persistent_netns_v2.sh << 'EOF'
#!/bin/bash
set -e

echo 'Starting network namespace setup in podman unshare context'
echo 'Current PID: \$\$'
echo 'Network namespace: /proc/\$\$/ns/net'

# Create veth pair in the current (unshared) namespace
echo 'Creating veth pair'
ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}

# Configure the interface that will stay in this namespace
echo 'Configuring interface in current namespace'
ip link set ${ENCLAVE_BOTTLE_IN} name eth1
ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
ip link set eth1 up
ip link set lo up

# Set default route
echo 'Setting default route'
ip route add default via ${ENCLAVE_SENTRY_IP}

echo 'Network namespace setup complete'
echo 'Namespace available at: /proc/\$\$/ns/net'
echo 'PID: \$\$'

# Save the PID for later reference
echo \$\$ > /tmp/unshare_pid_${NET_NAMESPACE}
echo 'PID saved to /tmp/unshare_pid_${NET_NAMESPACE}'

echo 'Keeping namespace alive with sleep infinity...'
# Keep the namespace alive by running sleep infinity
exec sleep infinity
EOF"

snnp_machine_ssh "chmod +x /tmp/persistent_netns_v2.sh"

echo -e "${CYAN}EXPECT NEXT TO FAIL WITH -> RTNETLINK answers: Operation not permitted (when creating veth pair inside user namespace)${NC}"

# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: 2025-06-24 17:06:59 PDT
# 
# Podman Version: 5.3.2 (client) / 5.3.1 (server) (Built: Wed Jan 22 05:42:46 2025)
# VM Build Date: 2024-11-17 16:00:00.000000000 -0800
# 
# Command: podman unshare /tmp/persistent_netns_v2.sh
# 
# Expected Error from Next Command:
# RTNETLINK answers: Operation not permitted
# 
# Note: This test failed at the veth pair creation step inside the user namespace,
# indicating that even with Podman unshare, network interface creation is restricted
# in rootless mode
# =============================================================================

echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Starting persistent namespace in background"
snnp_machine_ssh "podman unshare /tmp/persistent_netns_v2.sh &"
sleep 5  # Give it more time to set up

echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Getting the unshare process PID"
UNSHARE_PID=$(snnp_machine_ssh "cat /tmp/unshare_pid_${NET_NAMESPACE}")
echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Unshare PID: ${UNSHARE_PID}"

echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Connecting veth to bridge from host namespace"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} up

echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Verifying namespace setup"
snnp_machine_ssh "ls -la /proc/${UNSHARE_PID}/ns/"
snnp_machine_ssh "echo 'Process still running:' && ps -p ${UNSHARE_PID}"

echo "RBNS-PODMAN-UNSHARE-PERSISTENT-V2: Starting container with the prepared network namespace"
podman -c ${MACHINE} run -d                       \
    --name ${BOTTLE_CONTAINER}                    \
    --network ns:/proc/${UNSHARE_PID}/ns/net      \
    ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}

echo -e "${BOLD}Visualizing network setup in podman machine...${NC}"
echo "RBNI: Network interface information"
snnp_machine_ssh ip a

echo "RBNI: Network bridge information" 
snnp_machine_ssh ip link show type bridge

echo "RBNI: Network namespace information"
snnp_machine_ssh "echo 'Unshare process:' && ps -p ${UNSHARE_PID}"
snnp_machine_ssh "echo 'Namespace file:' && ls -la /proc/${UNSHARE_PID}/ns/net"

echo "RBNI: Route information"
snnp_machine_ssh ip route

echo -e "${BOLD}Verifying containers${NC}"
podman -c ${MACHINE} ps -a

echo -e "${GREEN}${BOLD}Setup script execution complete${NC}"
echo "Note: The persistent namespace process (PID: ${UNSHARE_PID}) is running in the background"
echo "Namespace available at: /proc/${UNSHARE_PID}/ns/net"
echo "To clean up, kill process ${UNSHARE_PID}" 