#!/bin/sh
echo "RBNS: Beginning bottle namespace setup script for unshare use"

set -e
set -x

# Validate required environment variables
: ${RBM_BOTTLE_CONTAINER:?}     && echo "RBNS0: RBM_BOTTLE_CONTAINER     = ${RBM_BOTTLE_CONTAINER}"
: ${RBRN_ENCLAVE_NETMASK:?}     && echo "RBNS0: RBRN_ENCLAVE_NETMASK     = ${RBRN_ENCLAVE_NETMASK}"
: ${RBRN_ENCLAVE_SENTRY_IP:?}   && echo "RBNS0: RBRN_ENCLAVE_SENTRY_IP   = ${RBRN_ENCLAVE_SENTRY_IP}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RBNS0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBM_ENCLAVE_BOTTLE_IN:?}    && echo "RBNS0: RBM_ENCLAVE_BOTTLE_IN    = ${RBM_ENCLAVE_BOTTLE_IN}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "RBNS0: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"
: ${RBRN_ENCLAVE_BOTTLE_IP:?}   && echo "RBNS0: RBRN_ENCLAVE_BOTTLE_IP   = ${RBRN_ENCLAVE_BOTTLE_IP}"
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "RBNS0: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"

# Check if we're running as root in a user namespace
if [ "$(id -u)" -eq 0 ]; then
  echo "RBNS-NS: Running as root in user namespace, good."
else
  echo "RBNS-NS: ERROR - Not running as root in a user namespace."
  echo "RBNS-NS: This script should be run inside 'unshare -r -n -m' context."
  exit 1
fi

# Setup the network namespace
echo "RBNS1: Setting up network namespace directory"
mkdir -p /run/netns
mount -t tmpfs netns /run/netns

echo "RBNS2: Creating network namespace"
ip netns add ${RBM_ENCLAVE_NAMESPACE}

echo "RBNS3: Creating and configuring veth pair"
ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}

echo "RBNS4: Moving veth endpoint to namespace"
ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns ${RBM_ENCLAVE_NAMESPACE}

echo "RBNS5: Configuring interface in namespace"
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set eth0 up
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip route add default via ${RBRN_ENCLAVE_SENTRY_IP} dev eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set lo up

echo "RBNS6: Connecting namespace veth to bridge"
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE}
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up

echo "RBNS7: Verifying namespace exists"
ls -la /run/netns/${RBM_ENCLAVE_NAMESPACE} || echo "WARNING: Namespace not found!"

echo "RBNS8: Making namespace visible to host system"
# We need to make the namespace created in our user namespace 
# accessible to the host system for container attachment
mkdir -p /var/run/netns
mount --bind /run/netns/${RBM_ENCLAVE_NAMESPACE} /var/run/netns/${RBM_ENCLAVE_NAMESPACE}

echo "RBNS: Bottle namespace setup complete"

