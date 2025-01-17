#!/bin/sh
echo "zNSs: Beginning sentry namespace setup script"

set -e
set -x

# Validate required environment variables
: ${RBM_MONIKER:?}              && echo "zNSs0: RBM_MONIKER              = ${RBM_MONIKER}"
: ${RBN_ENCLAVE_SENTRY_IP:?}    && echo "zNSs0: RBN_ENCLAVE_SENTRY_IP    = ${RBN_ENCLAVE_SENTRY_IP}"
: ${RBN_ENCLAVE_NETMASK:?}      && echo "zNSs0: RBN_ENCLAVE_NETMASK      = ${RBN_ENCLAVE_NETMASK}"
: ${zRBM_BRIDGE:?}              && echo "zNSs0: zRBM_BRIDGE              = ${zRBM_BRIDGE}"
: ${zRBM_VETH_SENTRY_IN:?}      && echo "zNSs0: zRBM_VETH_SENTRY_IN      = ${zRBM_VETH_SENTRY_IN}"
: ${zRBM_VETH_SENTRY_OUT:?}     && echo "zNSs0: zRBM_VETH_SENTRY_OUT     = ${zRBM_VETH_SENTRY_OUT}"

echo "zNSs1: Getting SENTRY PID"
SENTRY_PID=$(podman inspect -f '{{.State.Pid}}' ${RBM_MONIKER}-sentry) || exit 50
[ -n "$SENTRY_PID" ] || exit 51
echo "zNSs1: SENTRY PID: $SENTRY_PID"

echo "zNSs2: Creating and configuring bridge"
sudo ip link add name ${zRBM_BRIDGE} type bridge || exit 60
sudo ip link set ${zRBM_BRIDGE} up               || exit 61

echo "zNSs3: Creating and configuring SENTRY veth pair"
sudo ip link add ${zRBM_VETH_SENTRY_OUT} type veth peer name ${zRBM_VETH_SENTRY_IN}                 || exit 70
sudo ip link set ${zRBM_VETH_SENTRY_IN} netns $SENTRY_PID                                           || exit 71
sudo nsenter -t $SENTRY_PID -n ip link set ${zRBM_VETH_SENTRY_IN} name eth1                         || exit 72
sudo nsenter -t $SENTRY_PID -n ip addr add ${RBN_ENCLAVE_SENTRY_IP}/${RBN_ENCLAVE_NETMASK} dev eth1 || exit 73
sudo nsenter -t $SENTRY_PID -n ip link set eth1 up                                                  || exit 74

echo "zNSs4: Connecting SENTRY veth to bridge"
sudo ip link set ${zRBM_VETH_SENTRY_OUT} master ${zRBM_BRIDGE} || exit 80
sudo ip link set ${zRBM_VETH_SENTRY_OUT} up                    || exit 81

echo "zNSs: Sentry namespace setup complete"

