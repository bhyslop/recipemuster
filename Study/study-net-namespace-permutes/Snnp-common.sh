#!/bin/bash

# Color codes for output
GREEN='\033[0;32m'
BOLD='\033[1m'
NC='\033[0m' # No Color


MACHINE="pdvm-rbw"
MONIKER="nsproto"
SENTRY_CONTAINER="${MONIKER}-sentry"
BOTTLE_CONTAINER="${MONIKER}-bottle"
ENCLAVE_BRIDGE="vbr_${MONIKER}"
ENCLAVE_SENTRY_IN="vsi_${MONIKER}"
ENCLAVE_SENTRY_OUT="vso_${MONIKER}"
ENCLAVE_BOTTLE_IN="vbi_${MONIKER}"
ENCLAVE_BOTTLE_OUT="vbo_${MONIKER}"
ENCLAVE_BASE_IP="10.242.0.0"
ENCLAVE_NETMASK="24"
ENCLAVE_SENTRY_IP="10.242.0.2"
ENCLAVE_BOTTLE_IP="10.242.0.3"
ENTRY_PORT_WORKSTATION="8890"
ENTRY_PORT_ENCLAVE="8888"
SENTRY_REPO_PATH="ghcr.io/bhyslop/recipemuster"
BOTTLE_REPO_PATH="ghcr.io/bhyslop/recipemuster"
SENTRY_IMAGE_TAG="sentry_ubuntu_large.20241022__130547"
BOTTLE_IMAGE_TAG="bottle_ubuntu_test.20241207__190758"
DNS_SERVER="8.8.8.8"
UPLINK_ALLOWED_CIDRS="160.79.104.0/23"
UPLINK_ALLOWED_DOMAINS="anthropic.com"
NET_NAMESPACE="${MONIKER}-ns"
UNSHARE_PID_DIR="/tmp/unshare_pid_dir"
USER_NETNS_DIR="/tmp/user_netns"


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


function snnp_cleanup_all() {
    echo -e "${BOLD}Starting comprehensive cleanup...${NC}"
    
    echo -e "${BOLD}Stopping any prior containers${NC}"
    podman -c ${MACHINE} stop -t 2 ${SENTRY_CONTAINER} || echo "No running ${SENTRY_CONTAINER} to stop"
    podman -c ${MACHINE} rm -f     ${SENTRY_CONTAINER} || echo "No ${SENTRY_CONTAINER} to remove"
    podman -c ${MACHINE} stop -t 2 ${BOTTLE_CONTAINER} || echo "No running ${BOTTLE_CONTAINER} to stop"
    podman -c ${MACHINE} rm -f     ${BOTTLE_CONTAINER} || echo "No ${BOTTLE_CONTAINER} to remove"
    
    echo -e "${BOLD}Killing unshare and sleep processes${NC}"
    snnp_machine_ssh_sudo "pkill -9 -f 'sleep infinity'" || echo "No sleep infinity processes found"
    snnp_machine_ssh_sudo "pkill -9 -f 'unshare'"        || echo "No unshare processes found"
    
    echo -e "${BOLD}Cleaning up PID files${NC}"
    snnp_machine_ssh "rm -rf ${UNSHARE_PID_DIR}/*.pid"                     || echo "No PID files to remove"
    snnp_machine_ssh "rm -rf ${USER_NETNS_DIR}/*.pid"                      || echo "No user netns PID files to remove"
    snnp_machine_ssh "rm -f ${USER_NETNS_DIR}/cleanup_${NET_NAMESPACE}.sh" || echo "No cleanup script to remove"
    snnp_machine_ssh "rm -f ${USER_NETNS_DIR}/${NET_NAMESPACE}"            || echo "No namespace file to remove"
    
    echo -e "${BOLD}Network state before cleanup${NC}"
    echo "Network interfaces:"
    snnp_machine_ssh "ip link show"
    echo "Network namespaces:"
    snnp_machine_ssh "ip netns list"
    
    echo -e "${BOLD}Cleaning up network interfaces${NC}"
    snnp_machine_ssh_sudo "ip link del ${ENCLAVE_BOTTLE_OUT}"      || echo "Could not delete ${ENCLAVE_BOTTLE_OUT}"
    snnp_machine_ssh_sudo "ip link del ${ENCLAVE_SENTRY_OUT}"      || echo "Could not delete ${ENCLAVE_SENTRY_OUT}"
    snnp_machine_ssh_sudo "ip link del eth1@${ENCLAVE_BOTTLE_OUT}" || echo "Could not delete eth1@${ENCLAVE_BOTTLE_OUT}"
    snnp_machine_ssh_sudo "ip link del ${ENCLAVE_BOTTLE_OUT}@eth1" || echo "Could not delete ${ENCLAVE_BOTTLE_OUT}@eth1"
    snnp_machine_ssh_sudo "ip link del ${ENCLAVE_BRIDGE}"          || echo "Could not delete ${ENCLAVE_BRIDGE}"
    
    echo -e "${BOLD}Removing network namespaces${NC}"
    snnp_machine_ssh_sudo "ip netns delete ${NET_NAMESPACE}" || echo "Could not delete namespace ${NET_NAMESPACE}"
    
    echo -e "${BOLD}Verifying process cleanup${NC}"
    snnp_machine_ssh "ps -ef | grep -E 'unshare|sleep infinity' | grep -v grep || echo 'All processes successfully cleaned up'"
    
    echo -e "${BOLD}Verifying network cleanup${NC}"
    snnp_machine_ssh "ip link show | grep -E '${ENCLAVE_SENTRY_OUT}|${ENCLAVE_BOTTLE_OUT}|${ENCLAVE_BRIDGE}' || echo 'No matching interfaces found'"
    
    echo -e "${BOLD}Network state after cleanup${NC}"
    echo "Network interfaces:"
    snnp_machine_ssh "ip link show"
    echo "Network namespaces:"
    snnp_machine_ssh "ip netns list"
    
    echo -e "${GREEN}${BOLD}Cleanup complete${NC}"
}

function snnp_verify_machine_connection() {
    echo -e "${BOLD}Checking connection to ${MACHINE}${NC}"
    podman -c ${MACHINE} info > /dev/null || { echo "Unable to connect to machine"; exit 1; }
    echo -e "${GREEN}${BOLD}Connection successful.${NC}"
}

