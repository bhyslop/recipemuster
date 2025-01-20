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

echo "RBNS1: Getting BOTTLE PID"
BOTTLE_PID=$(podman inspect -f '{{.State.Pid}}' ${RBM_BOTTLE_CONTAINER}) || exit 50
[ -n "$BOTTLE_PID" ] || exit 51
echo "RBNS1: BOTTLE PID: $BOTTLE_PID"

echo "RBNS2: Creating and configuring BOTTLE veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN}              || exit 60
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns $BOTTLE_PID                                          || exit 61
sudo nsenter -t $BOTTLE_PID -n ip link set ${RBM_ENCLAVE_BOTTLE_IN} name eth0                        || exit 62
sudo nsenter -t $BOTTLE_PID -n ip addr add ${RBN_ENCLAVE_BOTTLE_IP}/${RBN_ENCLAVE_NETMASK} dev eth0  || exit 63
sudo nsenter -t $BOTTLE_PID -n ip link set eth0 up                                                   || exit 64

echo "RBNS3: Connecting BOTTLE veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE} || exit 70
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up                           || exit 71

echo "RBNS4: Configuring BOTTLE routing"
sudo nsenter -t $BOTTLE_PID -n ip route add default via ${RBN_ENCLAVE_SENTRY_IP} dev eth0 || exit 80

echo "RBNS: Bottle namespace setup complete"
