#!/bin/sh
echo "RBNC: Beginning network configuration script"

set -e
set -x

# Validate required environment variables
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "RBNC0: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"
: ${RBM_ENCLAVE_BOTTLE_IN:?}    && echo "RBNC0: RBM_ENCLAVE_BOTTLE_IN    = ${RBM_ENCLAVE_BOTTLE_IN}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "RBNC0: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RBNC0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBRN_ENCLAVE_BOTTLE_IP:?}   && echo "RBNC0: RBRN_ENCLAVE_BOTTLE_IP   = ${RBRN_ENCLAVE_BOTTLE_IP}"
: ${RBRN_ENCLAVE_NETMASK:?}     && echo "RBNC0: RBRN_ENCLAVE_NETMASK     = ${RBRN_ENCLAVE_NETMASK}"
: ${RBRN_ENCLAVE_SENTRY_IP:?}   && echo "RBNC0: RBRN_ENCLAVE_SENTRY_IP   = ${RBRN_ENCLAVE_SENTRY_IP}"

# Define the namespace file path
USER_NETNS_DIR=/home/user/users-netns
NAMESPACE_FILE=$USER_NETNS_DIR/${RBM_ENCLAVE_NAMESPACE}

# Check if namespace file exists
if [ ! -e "$NAMESPACE_FILE" ]; then
  echo "RBNC-ERROR: Namespace file not found at $NAMESPACE_FILE"
  exit 1
fi

echo "RBNC1: Creating and configuring veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}

echo "RBNC2: Moving veth endpoint to namespace"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns $NAMESPACE_FILE

echo "RBNC3: Configuring interface in namespace"
sudo ip netns exec $NAMESPACE_FILE ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0
sudo ip netns exec $NAMESPACE_FILE ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0
sudo ip netns exec $NAMESPACE_FILE ip link set eth0 up
sudo ip netns exec $NAMESPACE_FILE ip route add default via ${RBRN_ENCLAVE_SENTRY_IP} dev eth0
sudo ip netns exec $NAMESPACE_FILE ip link set lo up

echo "RBNC4: Connecting namespace veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE}
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up

echo "RBNC5: Network configuration complete"

