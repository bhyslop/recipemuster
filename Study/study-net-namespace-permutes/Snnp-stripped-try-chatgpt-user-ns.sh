#!/bin/bash

# Simplified Container Network Setup Script (User Network Namespace Version)
# Each step is executed discretely with minimal environment passing

set -e  # Exit on error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "SNNP: Get constants from ${SCRIPT_DIR}"
source "$SCRIPT_DIR/Snnp-constants.sh"

function snnp_podman_exec_sentry() {
    podman -c ${MACHINE} exec ${SENTRY_CONTAINER} "$@"
}

function snnp_podman_exec_bottle() {
    podman -c ${MACHINE} exec ${BOTTLE_CONTAINER} "$@"
}

function snnp_machine_ssh() {
    podman machine ssh ${MACHINE} "$@"
}

function snnp_machine_ssh_sudo() {
    podman machine ssh ${MACHINE} sudo "$@"
}


echo -e "${BOLD}Container Network Setup Script (User Network Namespace)${NC}"
echo "Setting up ${MONIKER} containers with network isolation"
echo ""

echo -e "${BOLD}Checking connection to ${MACHINE}${NC}"
podman -c ${MACHINE} info > /dev/null || { echo "Unable to connect to machine"; exit 1; }
echo -e "${GREEN}${BOLD}Connection successful.${NC}"

echo -e "${BOLD}Stopping any prior containers${NC}"
podman -c ${MACHINE} stop -t 2 ${SENTRY_CONTAINER} || echo "Attempt to stop ${SENTRY_CONTAINER} did nothing"
podman -c ${MACHINE} rm -f     ${SENTRY_CONTAINER} || echo "Attempt to rm   ${SENTRY_CONTAINER} did nothing"
podman -c ${MACHINE} stop -t 2 ${BOTTLE_CONTAINER} || echo "Attempt to stop ${BOTTLE_CONTAINER} did nothing"
podman -c ${MACHINE} rm -f     ${BOTTLE_CONTAINER} || echo "Attempt to rm   ${BOTTLE_CONTAINER} did nothing"

echo -e "${BOLD}Cleaning up old netns and interfaces inside VM${NC}"
echo "RBNC: Beginning network cleanup script"
echo "RBNC: Before cleanup..."
podman machine ssh ${MACHINE} ip link show
podman machine ssh ${MACHINE} ip netns list

echo "RBNC2: Removing prior run elements"
snnp_machine_ssh_sudo ip link del ${ENCLAVE_BRIDGE} || echo "RBNC2: could not delete ${ENCLAVE_BRIDGE}"    
snnp_machine_ssh_sudo ip netns delete ${NET_NAMESPACE} || echo "RBNC2: could not delete ${NET_NAMESPACE}"     

echo "RBNC3: Verifying cleanup"
snnp_machine_ssh "ip link show | grep -E '${ENCLAVE_SENTRY_OUT}|${ENCLAVE_BOTTLE_OUT}|${ENCLAVE_BRIDGE}' || echo 'No matching interfaces found'"
echo "RBNC: Network cleanup complete"

echo "RBNC: After cleanup..."
podman machine ssh ${MACHINE} ip link show
podman machine ssh ${MACHINE} ip netns list

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
# Create the network namespace in user mode; note that we no longer adjust its permissions.
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

echo -e "${BOLD}Verifying network namespace${NC}"
snnp_machine_ssh "echo 'Listing /var/run/netns:' && ls -l /var/run/netns"
snnp_machine_ssh "echo 'Detailed information for ${NET_NAMESPACE}:' && stat /var/run/netns/${NET_NAMESPACE}"

# NOTE: In this user network namespace approach we do not attempt to modify
# the namespace file permissions (the chmod step has been removed).

echo "RBNS-ALT: Starting container with the prepared network namespace"
snnp_machine_ssh podman run -d                    \
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

