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

echo "RBNS-ALT: Setting up user-accessible network namespace"
snnp_machine_ssh "mkdir -p ${UNSHARE_PID_DIR}"

echo "RBNS-ALT: Create a small script on the remote machine to handle the PID capture correctly"
snnp_machine_ssh "cat > /tmp/create_netns.sh << 'EOF'
#!/bin/bash
echo \$\$ > ${UNSHARE_PID_DIR}/${NET_NAMESPACE}.pid
exec sleep infinity
EOF"
snnp_machine_ssh "chmod +x /tmp/create_netns.sh"

echo "RBNS-ALT: Assure creation command well formed"
snnp_machine_ssh "cat /tmp/create_netns.sh"

echo "RBNS-ALT: First capture the sudo process PID"
SUDO_PID=$(snnp_machine_ssh_sudo "nohup unshare --net --fork --pid --mount-proc /bin/sleep infinity > /tmp/unshare.log 2>&1 & echo \$!")
echo "RBNS-ALT: Sudo process PID: ${SUDO_PID}"

echo "RBNS-ALT: Wait a moment for child processes to spawn"
sleep 2

echo "RBNS-ALT: Now find the sleep infinity PID which is what we actually need"
UNSHARE_PID=$(snnp_machine_ssh "ps --ppid \$(ps --ppid ${SUDO_PID} -o pid= | tr -d ' ') -o pid= | tr -d ' '")
echo "RBNS-ALT: Actual sleep process PID: ${UNSHARE_PID}"

echo "RBNS-ALT: Detailed process info for PID ${UNSHARE_PID}:"
snnp_machine_ssh "ps -p ${UNSHARE_PID} -o pid,ppid,stat,cmd= || echo 'Process not found'"
snnp_machine_ssh "ps -ef | grep 'sleep infinity' | grep -v grep || echo 'Sleep process not found'"

echo "RBNS-ALT: Checking unshare log for errors:"
snnp_machine_ssh "cat /tmp/unshare.log" || echo 'No log file found'

echo "RBNS-ALT: Verify the PID is valid"
snnp_machine_ssh "ps -p ${UNSHARE_PID} -o cmd="

echo "RBNS-ALT: Creating veths"
podman machine ssh ${MACHINE} ip link show

echo "RBNS-ALT: Creating veth pair -> ${ENCLAVE_BOTTLE_OUT} ${ENCLAVE_BOTTLE_IN}"
snnp_machine_ssh_sudo ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}

echo "RBNS-ALT: Moving veth endpoint to namespace"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_IN} netns ${UNSHARE_PID}

echo "RBNS-ALT: Configuring interfaces in namespace"
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip link set ${ENCLAVE_BOTTLE_IN} name eth1
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip link set eth1 up
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip link set lo up

echo "RBNS-ALT: Connecting veth to bridge"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} up

echo "RBNS-ALT: Collect diagnostic info: ip route show shows"
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip route show

echo "RBNS-ALT: Collect diagnostic info: sdhow pid file written"
snnp_machine_ssh_sudo "ps -p ${UNSHARE_PID} -o cmd="

echo "RBNS-ALT: Setting default route in namespace"
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip route add default via ${ENCLAVE_SENTRY_IP}

echo "RBNS-ALT: Check interfaces after namespace setup..."
snnp_machine_ssh ip link show
snnp_machine_ssh_sudo nsenter -t ${UNSHARE_PID} -n ip link show

echo "RBNS-ALT: Checking permissions on network namespace"
snnp_machine_ssh_sudo ls -la /proc/${UNSHARE_PID}/ns/net

echo -e "${CYAN}EXPECT NEXT TO FAIL WITH -> Error: cannot find specified network namespace path: faccessat /proc/${UNSHARE_PID}/ns/net: permission denied"

echo "RBNS-ALT: Starting container with the prepared user network namespace"
podman -c ${MACHINE} run -d                  \
    --name ${BOTTLE_CONTAINER}               \
    --network ns:/proc/${UNSHARE_PID}/ns/net \
    ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}

echo -e "${BOLD}Visualizing network setup in podman machine...${NC}"
echo "RBNI: Network interface information"
snnp_machine_ssh ip a

echo "RBNI: Network bridge information" 
snnp_machine_ssh ip link show type bridge

echo "RBNI: Network namespace information"
snnp_machine_ssh "ps aux | grep 'sleep infinity' | grep -v grep"
snnp_machine_ssh "ls -la ${UNSHARE_PID_DIR}"

echo "RBNI: Route information"
snnp_machine_ssh ip route

echo -e "${BOLD}Verifying containers${NC}"
podman -c ${MACHINE} ps -a

echo -e "${GREEN}${BOLD}Setup script execution complete${NC}"
echo "The unshare process (PID: ${UNSHARE_PID}) will be cleaned up automatically on next script run"



