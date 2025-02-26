#!/bin/sh
echo "RBNS: Beginning bottle namespace setup script"

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
sudo ip netns add ${RBM_ENCLAVE_NAMESPACE} || exit 50

echo "RBNS1-DEBUG2: Make namespace accessible to the podman user"
sudo chmod 755 /run/netns/${RBM_ENCLAVE_NAMESPACE} || echo "Warning: Could not change ns file permissions"

echo "RBNS1-DEBUG2: Also try changing ownership if chmod alone doesn't fix it"
sudo chown user:user /run/netns/${RBM_ENCLAVE_NAMESPACE} || echo "Warning: Could not change ns file ownership"

echo "RBNS1-DEBUG: Checking namespace creation results..."
echo "RBNS1-DEBUG: Namespace list:"
sudo ip netns list
echo "RBNS1-DEBUG: Netns directory contents:"
sudo ls -la /var/run/netns/ || echo "No /var/run/netns directory"
sudo ls -la /run/netns/     || echo "No /run/netns directory"
echo "RBNS1-DEBUG: End namespace check"

echo "RBNS2: Creating and configuring veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN} || exit 60

echo "RBNS3: Moving veth endpoint to namespace"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns ${RBM_ENCLAVE_NAMESPACE} || exit 61

echo "RBNS4: Configuring interface in namespace"
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set ${RBM_ENCLAVE_BOTTLE_IN}                         name eth0    || exit 62
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0    || exit 63
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set                                                       eth0 up || exit 64
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip route add default via ${RBRN_ENCLAVE_SENTRY_IP}            dev eth0    || exit 65
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link set lo up                                                         || exit 66

echo "RBNS5: Connecting namespace veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE} || exit 70
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up                           || exit 71

echo "RBNS: Bottle namespace setup complete"

