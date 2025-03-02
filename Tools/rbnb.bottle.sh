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

# Setup the user-accessible network namespace directory
echo "RBNS1: Setting up user-accessible network namespace directory"
USER_NETNS_DIR=/home/user/users-netns
mkdir -p $USER_NETNS_DIR
mount -t tmpfs netns $USER_NETNS_DIR

# Create our namespace reference file
NAMESPACE_FILE=$USER_NETNS_DIR/${RBM_ENCLAVE_NAMESPACE}
echo "RBNS2: Creating network namespace file at $NAMESPACE_FILE"

# We're already in a network namespace (from unshare -n)
# So we just need to save a reference to the current namespace
echo "RBNS3: Saving reference to current network namespace"
touch $NAMESPACE_FILE
mount --bind /proc/self/ns/net $NAMESPACE_FILE

echo "RBNS4: Creating and configuring veth pair"
ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}

echo "RBNS5: Configuring interfaces"
# We're already in the namespace, so no need to specify it
ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0
ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0
ip link set eth0 up
ip route add default via ${RBRN_ENCLAVE_SENTRY_IP} dev eth0
ip link set lo up

echo "RBNS6: Connecting namespace veth to bridge"
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE}
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up

echo "RBNS7: Verifying namespace file exists"
ls -la $NAMESPACE_FILE || echo "WARNING: Namespace file not found!"

echo "RBNS8: Network namespace is ready at $NAMESPACE_FILE"

echo "RBNS: Bottle namespace setup complete"

