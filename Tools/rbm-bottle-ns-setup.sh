#!/bin/sh
echo "RBNS: Beginning bottle namespace setup script"

set -e
set -x

# Validate required environment variables
: ${RBM_BOTTLE_CONTAINER:?}     && echo "RBNS0: RBM_BOTTLE_CONTAINER     = ${RBM_BOTTLE_CONTAINER}"
: ${RBN_ENCLAVE_NETMASK:?}      && echo "RBNS0: RBN_ENCLAVE_NETMASK      = ${RBN_ENCLAVE_NETMASK}"
: ${RBN_ENCLAVE_SENTRY_IP:?}    && echo "RBNS0: RBN_ENCLAVE_SENTRY_IP    = ${RBN_ENCLAVE_SENTRY_IP}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RBNS0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBM_ENCLAVE_BOTTLE_IN:?}    && echo "RBNS0: RBM_ENCLAVE_BOTTLE_IN    = ${RBM_ENCLAVE_BOTTLE_IN}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "RBNS0: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"
: ${RBN_ENCLAVE_BOTTLE_IP:?}    && echo "RBNS0: RBN_ENCLAVE_BOTTLE_IP    = ${RBN_ENCLAVE_BOTTLE_IP}"
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "RBNS0: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"

echo "RBNS1: Creating network namespace"
sudo ip netns add ${RBM_ENCLAVE_NAMESPACE} || exit 50

echo "RBNS2: Creating and configuring veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN} || exit 60

echo "RBNS3: Moving veth endpoint to namespace"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns ${RBM_ENCLAVE_NAMESPACE} || exit 61

echo "RBNS4: Configuring interface in namespace"
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0                         || exit 62
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip addr add ${RBN_ENCLAVE_BOTTLE_IP}/${RBN_ENCLAVE_NETMASK} dev eth0   || exit 63
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set eth0 up                                                    || exit 64
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip route add default via ${RBN_ENCLAVE_SENTRY_IP} dev eth0             || exit 65

echo "RBNS5: Connecting namespace veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE} || exit 70
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up                           || exit 71

echo "RBNS: Bottle namespace setup complete"

