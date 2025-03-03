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
  echo "RBNS-NS: This script should be run inside 'unshare -r -m' context."
  exit 1
fi

# This script needs to be modified to run WITHOUT the -n flag in unshare
# Update the makefile command to:
# $(zRBM_PODMAN_SSH_CMD) "unshare -r -m --propagation private /bin/sh" < $(MBV_TOOLS_DIR)/rbnb.bottle.sh

# Setup the user-accessible network namespace directory
echo "RBNS1: Setting up user-accessible network namespace directory"
USER_NETNS_DIR=/home/user/users-netns
mkdir -p $USER_NETNS_DIR
mount -t tmpfs netns $USER_NETNS_DIR

# First, check if the bridge exists
echo "RBNS2: Checking for bridge existence"
if ! ip link show ${RBM_ENCLAVE_BRIDGE} > /dev/null 2>&1; then
  echo "ERROR: Bridge ${RBM_ENCLAVE_BRIDGE} does not exist!"
  exit 1
fi

# Create veth pair in the current namespace
echo "RBNS3: Creating veth pair"
ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}

# Connect one end to the bridge
echo "RBNS4: Attaching veth to bridge"
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE}
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up

# Now create a new network namespace
echo "RBNS5: Creating new network namespace"
# Create network namespace
ip netns add ${RBM_ENCLAVE_NAMESPACE}

# Move the other veth end to this namespace
echo "RBNS6: Moving veth to new namespace"
ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns ${RBM_ENCLAVE_NAMESPACE}

# Configure the interface inside the namespace
echo "RBNS7: Configuring interface in namespace"
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set eth0 up
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip route add default via ${RBRN_ENCLAVE_SENTRY_IP} dev eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set lo up

# Create our namespace reference file for podman
NAMESPACE_FILE=$USER_NETNS_DIR/${RBM_ENCLAVE_NAMESPACE}
echo "RBNS8: Creating namespace file at $NAMESPACE_FILE"
touch $NAMESPACE_FILE
mount --bind /var/run/netns/${RBM_ENCLAVE_NAMESPACE} $NAMESPACE_FILE

echo "RBNS9: Verifying namespace file exists"
ls -la $NAMESPACE_FILE || echo "WARNING: Namespace file not found!"

echo "RBNS: Bottle namespace setup complete"

