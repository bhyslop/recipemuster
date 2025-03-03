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

# Get namespace info from the temp files
TEMP_DIR=/tmp/netns-setup
NS_PID_FILE=$TEMP_DIR/${RBM_ENCLAVE_NAMESPACE}.pid
NS_INFO_FILE=$TEMP_DIR/${RBM_ENCLAVE_NAMESPACE}.ns

# Check if namespace info exists
if [ ! -e "$NS_PID_FILE" ]; then
  echo "RBNC-ERROR: Namespace PID file not found at $NS_PID_FILE"
  exit 1
fi

NS_PID=$(cat $NS_PID_FILE)
echo "RBNC1: Using namespace from PID $NS_PID"

# Check if process is still running
if ! ps -p $NS_PID > /dev/null; then
  echo "RBNC-ERROR: Namespace process $NS_PID is not running"
  exit 1
fi

# Create and set up the standard netns directory
echo "RBNC2: Setting up system netns directory"
sudo mkdir -p /var/run/netns
sudo touch /var/run/netns/${RBM_ENCLAVE_NAMESPACE}
sudo mount --bind /proc/$NS_PID/ns/net /var/run/netns/${RBM_ENCLAVE_NAMESPACE}

echo "RBNC3: Creating and configuring veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}

echo "RBNC4: Moving veth endpoint to namespace"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns ${RBM_ENCLAVE_NAMESPACE}

echo "RBNC5: Configuring interface in namespace"
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set eth0 up
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip route add default via ${RBRN_ENCLAVE_SENTRY_IP} dev eth0
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set lo up

echo "RBNC6: Connecting namespace veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE}
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up

echo "RBNC7: Network configuration complete"

