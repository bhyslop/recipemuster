#!/bin/sh

echo "OBSN DIAG: Script starting, pwd=$(pwd)"
echo "OBSN DIAG: Script path=$0"
echo "OBSN DIAG: Environment check..."

echo "OBSN: Beginning network observation script"

set -e

# Validate required environment variables
: ${RBM_MACHINE:?}              && echo "OBSN: RBM_MACHINE              = ${RBM_MACHINE}"
: ${RBM_MONIKER:?}              && echo "OBSN: RBM_MONIKER              = ${RBM_MONIKER}"
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "OBSN: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "OBSN: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "OBSN: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"
: ${RBM_SENTRY_CONTAINER:?}     && echo "OBSN: RBM_SENTRY_CONTAINER     = ${RBM_SENTRY_CONTAINER}"
: ${RBRN_ENCLAVE_BOTTLE_IP:?}   && echo "OBSN: RBRN_ENCLAVE_BOTTLE_IP   = ${RBRN_ENCLAVE_BOTTLE_IP}"
: ${RBRN_ENCLAVE_SENTRY_IP:?}   && echo "OBSN: RBRN_ENCLAVE_SENTRY_IP   = ${RBRN_ENCLAVE_SENTRY_IP}"

echo "OBSN: Storing terminal control sequences"
BOLD=$(tput bold)
RED=$(tput setaf 1)
YELLOW=$(tput setaf 3)
BLUE=$(tput setaf 4)
MAGENTA=$(tput setaf 5)
WHITE=$(tput setaf 7)
RESET=$(tput sgr0)

echo "OBSN: Setting up signal handling"
cleanup() {
    echo "OBSN: Cleaning up observation processes"
    kill 0
    exit 0
}
trap cleanup SIGINT SIGTERM

echo "OBSN DIAG: About to setup tcpdump"

echo "OBSN: Setting up common tcpdump options"
TCPDUMP_OPTS="-U -l -nn -vvv"
FILTER="host ${RBRN_ENCLAVE_BOTTLE_IP} or host ${RBRN_ENCLAVE_SENTRY_IP}"

echo "OBSN: Defining output prefixing functions"
prefix_bottle() {
    while read -r line; do
        echo "${YELLOW}${BOLD}OBSN: [BOTTLE]${RESET} $line"
    done
}

prefix_bridge() {
    while read -r line; do
        echo "${BLUE}${BOLD}OBSN: [BRIDGE]${RESET} $line"
    done
}

prefix_veth() {
    while read -r line; do
        echo "${MAGENTA}${BOLD}OBSN: [VETH]${RESET} $line"
    done
}

prefix_sentry() {
    while read -r line; do
        echo "${WHITE}${BOLD}OBSN: [SENTRY]${RESET} $line"
    done
}

echo "OBSN DIAG: About to start captures"

echo "OBSN: Starting network capture processes"
echo "OBSN: Starting bottle perspective capture"
podman machine ssh ${RBM_MACHINE} "sudo -n ip netns exec ${RBM_ENCLAVE_NAMESPACE} tcpdump ${TCPDUMP_OPTS} -i eth0 '${FILTER}'" 2>&1 | 
    prefix_bottle &

echo "OBSN: Starting bridge perspective capture"
podman machine ssh ${RBM_MACHINE} "sudo -n tcpdump ${TCPDUMP_OPTS} -i ${RBM_ENCLAVE_BRIDGE} '${FILTER}'" 2>&1 | 
    prefix_bridge &

echo "OBSN: Starting veth perspective capture"
podman machine ssh ${RBM_MACHINE} "sudo -n tcpdump ${TCPDUMP_OPTS} -i ${RBM_ENCLAVE_BOTTLE_OUT} '${FILTER}'" 2>&1 | 
    prefix_veth &

echo "OBSN: Starting sentry perspective capture"
podman --connection ${RBM_MACHINE} exec ${RBM_SENTRY_CONTAINER} tcpdump ${TCPDUMP_OPTS} -i eth1 "${FILTER}" 2>&1 | 
    prefix_sentry &

echo "OBSN: All capture processes started"
echo "OBSN: Press Ctrl+C to stop captures"

echo "OBSN DIAG: About to wait for processes"

wait

echo "OBSN DIAG: Wait completed"

# Cleanup will be handled by the trap
