#!/bin/sh
echo "RBNS: Beginning bottle namespace setup script using container PID"

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

echo "RBNS1: Getting BOTTLE PID"
BOTTLE_PID=$(podman inspect -f '{{.State.Pid}}' ${RBM_BOTTLE_CONTAINER}) || exit 50
[ -n "$BOTTLE_PID" ] || exit 51
echo "RBNS1: BOTTLE PID: $BOTTLE_PID"

echo "RBNS2: Creating and configuring veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN} || exit 60

echo "RBNS3: Moving veth endpoint to container namespace"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns $BOTTLE_PID || exit 61

echo "RBNS4: Configuring interface in container namespace"
sudo nsenter -t $BOTTLE_PID -n ip link set ${RBM_ENCLAVE_BOTTLE_IN}                         name eth0    || exit 62
sudo nsenter -t $BOTTLE_PID -n ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0    || exit 63
sudo nsenter -t $BOTTLE_PID -n ip link set                                                       eth0 up || exit 64
sudo nsenter -t $BOTTLE_PID -n ip route add default via ${RBRN_ENCLAVE_SENTRY_IP}            dev eth0    || exit 65
sudo nsenter -t $BOTTLE_PID -n ip link set lo up                                                         || exit 66

echo "RBNS5: Connecting namespace veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE} || exit 70
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up                           || exit 71

echo "RBNS6: Setting DNS configuration in container namespace"
sudo nsenter -t $BOTTLE_PID -n mkdir -p /etc/netns || true
sudo nsenter -t $BOTTLE_PID -n bash -c "echo 'nameserver ${RBRN_ENCLAVE_SENTRY_IP}' > /etc/resolv.conf" || echo "WARNING: Failed to set DNS configuration"

echo "RBNS: Bottle namespace setup complete"

