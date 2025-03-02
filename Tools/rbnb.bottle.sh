#!/bin/sh
echo "RBNS: Beginning bottle namespace setup inside unshare context"

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

echo "RBNS1: Creating network namespace"
ip netns add ${RBM_ENCLAVE_NAMESPACE}

echo "RBNS2: Creating and configuring veth pair"
ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}

echo "RBNS3: Moving veth endpoint to namespace"
ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns ${RBM_ENCLAVE_NAMESPACE}

echo "RBNS4: Configuring interface in namespace"
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set eth0 up
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip route add default via ${RBRN_ENCLAVE_SENTRY_IP} dev eth0
ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set lo up

echo "RBNS5: Connecting namespace veth to bridge"
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE}
ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up

echo "RBNS: Bottle namespace setup complete"

