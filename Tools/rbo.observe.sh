#!/bin/sh

echo "RBO DIAG: Script starting, pwd=$(pwd)"
echo "RBO DIAG: Script path=$0"
echo "RBO DIAG: Environment check..."

echo "RBO: Beginning network observation script"

set -e

# Validate required environment variables
: ${RBM_MACHINE:?}              && echo "RBO: RBM_MACHINE              = ${RBM_MACHINE}"
: ${RBM_MONIKER:?}              && echo "RBO: RBM_MONIKER              = ${RBM_MONIKER}"
: ${RBM_ENCLAVE_NETWORK:?}      && echo "RBO: RBM_ENCLAVE_NETWORK      = ${RBM_ENCLAVE_NETWORK}"
: ${RBM_SENTRY_CONTAINER:?}     && echo "RBO: RBM_SENTRY_CONTAINER     = ${RBM_SENTRY_CONTAINER}"
: ${RBM_BOTTLE_CONTAINER:?}     && echo "RBO: RBM_BOTTLE_CONTAINER     = ${RBM_BOTTLE_CONTAINER}"
: ${RBM_CENSER_CONTAINER:?}     && echo "RBO: RBM_CENSER_CONTAINER     = ${RBM_CENSER_CONTAINER}"

echo "RBO: Storing terminal control sequences"
BOLD=$(tput bold)
RED=$(tput setaf 1)
YELLOW=$(tput setaf 3)
BLUE=$(tput setaf 4)
MAGENTA=$(tput setaf 5)
WHITE=$(tput setaf 7)
RESET=$(tput sgr0)

echo "RBO: Setting up signal handling"
cleanup() {
    echo "RBO: Cleaning up observation processes"
    kill 0
    exit 0
}
trap cleanup SIGINT SIGTERM

echo "RBO DIAG: About to setup tcpdump"

echo "RBO: Setting up common tcpdump options"
TCPDUMP_OPTS="-U -l -nn -vvv"

echo "RBO: Discovering bridge interface for enclave network"
BRIDGE_INTERFACE=$(podman --connection ${RBM_MACHINE} network inspect ${RBM_ENCLAVE_NETWORK} --format '{{.NetworkInterface}}')
echo "RBO: Bridge interface: ${BRIDGE_INTERFACE}"

echo "RBO: Defining output prefixing functions"
prefix_bottle() {
    while read -r line; do
        echo "${YELLOW}${BOLD}RBO: [BOTTLE/CENSER]${RESET} $line"
    done
}

prefix_bridge() {
    while read -r line; do
        echo "${BLUE}${BOLD}RBO: [BRIDGE]${RESET} $line"
    done
}

prefix_sentry() {
    while read -r line; do
        echo "${WHITE}${BOLD}RBO: [SENTRY]${RESET} $line"
    done
}

echo "RBO: Starting network capture processes"

echo "RBO: Starting bottle/censer shared namespace capture (using CENSER container)"
podman --connection ${RBM_MACHINE} exec ${RBM_CENSER_CONTAINER} tcpdump ${TCPDUMP_OPTS} -i eth0 2>&1 | 
    prefix_bottle &

echo "RBO: Starting bridge perspective capture"
podman --connection ${RBM_MACHINE} machine ssh "sudo -n tcpdump ${TCPDUMP_OPTS} -i ${BRIDGE_INTERFACE}" 2>&1 | 
    prefix_bridge &

echo "RBO: Starting sentry enclave interface capture"
podman --connection ${RBM_MACHINE} exec ${RBM_SENTRY_CONTAINER} tcpdump ${TCPDUMP_OPTS} -i eth1 2>&1 | 
    prefix_sentry &

echo "RBO: All capture processes started"
echo "RBO: Network topology:"
echo "RBO:   - SENTRY: Bridge (eth0) <-> Internet, Enclave (eth1) <-> Internal"
echo "RBO:   - BOTTLE/CENSER: Shared namespace on Enclave network (eth0)"
echo "RBO: Press Ctrl+C to stop captures"

echo "RBO DIAG: About to wait for processes"

wait

echo "RBO DIAG: Wait completed"

# Cleanup will be handled by the trap

