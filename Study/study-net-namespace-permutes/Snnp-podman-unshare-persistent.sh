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

echo -e "${BOLD}Using podman unshare to create persistent network namespace${NC}"
echo "RBNS-PODMAN-UNSHARE-PERSISTENT: Creating persistent network namespace"

# Create a script that will run inside podman unshare and keep the namespace alive
snnp_machine_ssh "cat > /tmp/persistent_netns.sh << 'EOF'
#!/bin/bash
set -e

echo 'Creating network namespace: ${NET_NAMESPACE}'
ip netns add ${NET_NAMESPACE}

echo 'Creating veth pair'
ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}

echo 'Moving veth endpoint to namespace'
ip link set ${ENCLAVE_BOTTLE_IN} netns ${NET_NAMESPACE}

echo 'Configuring interfaces in namespace'
ip netns exec ${NET_NAMESPACE} ip link set ${ENCLAVE_BOTTLE_IN} name eth1
ip netns exec ${NET_NAMESPACE} ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
ip netns exec ${NET_NAMESPACE} ip link set eth1 up
ip netns exec ${NET_NAMESPACE} ip link set lo up

echo 'Connecting veth to bridge'
ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
ip link set ${ENCLAVE_BOTTLE_OUT} up

echo 'Setting default route in namespace'
ip netns exec ${NET_NAMESPACE} ip route add default via ${ENCLAVE_SENTRY_IP}

echo 'Network namespace setup complete'
echo 'Namespace path: /var/run/netns/${NET_NAMESPACE}'
ls -la /var/run/netns/

echo 'Keeping namespace alive...'
# Keep the namespace alive by holding a reference to it
exec ip netns exec ${NET_NAMESPACE} sleep infinity
EOF"

snnp_machine_ssh "chmod +x /tmp/persistent_netns.sh"

echo -e "${CYAN}EXPECT NEXT TO FAIL WITH -> mkdir /var/run/netns failed: Permission denied (user namespace can't write to /var/run/netns)${NC}"

# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: 2025-06-24 17:04:22 PDT
# 
# Podman Version: 5.3.2 (client) / 5.3.1 (server) (Built: Wed Jan 22 05:42:46 2025)
# VM Build Date: 2024-11-17 16:00:00.000000000 -0800
# Mode: Rootless (Rootful: false)
# 
# Command: podman unshare /tmp/persistent_netns.sh
# 
# Expected Error from Next Command:
# mkdir /var/run/netns failed: Permission denied
# 
# =============================================================================

# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: 2025-06-24 18:09:18 PDT
# 
# Podman Version: 5.3.2 (client) / 5.3.1 (server) (Built: Wed Jan 22 05:42:46 2025)
# VM Build Date: 2024-11-17 16:00:00.000000000 -0800
# Mode: Rootful (Rootful: true)
# 
# Command: podman unshare /tmp/persistent_netns.sh
# 
# Expected Error from Next Command:
# mkdir /var/run/netns failed: Permission denied
# =============================================================================

# =============================================================================
# VERSION STUDY DOCUMENTATION BLOCK
# =============================================================================
# Date: 2025-06-24 19:15:00 PDT
# 
# Podman Version: 5.5.2 (client) / 5.5.1 (server) (Built: Tue Jun 24 09:13:04 2025)
# VM Build Date: 2025-04-22 17:00:00.000000000 -0700
# Mode: Rootless (Rootful: false)
# 
# Command: podman unshare /tmp/persistent_netns.sh
# 
# Expected Error from Next Command:
# mkdir /var/run/netns failed: Permission denied
# =============================================================================

echo "RBNS-PODMAN-UNSHARE-PERSISTENT: Starting persistent namespace in background"
snnp_machine_ssh "podman unshare /tmp/persistent_netns.sh &"
sleep 3  # Give it time to set up

echo "RBNS-PODMAN-UNSHARE-PERSISTENT: Verifying namespace was created"
snnp_machine_ssh "ls -la /var/run/netns/"

echo "RBNS-PODMAN-UNSHARE-PERSISTENT: Starting container with the prepared network namespace"
podman -c ${MACHINE} run -d                       \
    --name ${BOTTLE_CONTAINER}                    \
    --network ns:/var/run/netns/${NET_NAMESPACE}  \
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
echo "Note: The persistent namespace process is running in the background"
echo "To clean up, you may need to kill the podman unshare process manually" 
