#!/bin/sh
echo "zNSb: Beginning bottle namespace setup script"

set -e
set -x

# Validate required environment variables
: ${RBM_MONIKER:?}              && echo "zNSb0: RBM_MONIKER              = ${RBM_MONIKER}"
: ${RBN_ENCLAVE_INITIAL_IP:?}   && echo "zNSb0: RBN_ENCLAVE_INITIAL_IP   = ${RBN_ENCLAVE_INITIAL_IP}"
: ${RBN_ENCLAVE_NETMASK:?}      && echo "zNSb0: RBN_ENCLAVE_NETMASK      = ${RBN_ENCLAVE_NETMASK}"
: ${RBN_ENCLAVE_SENTRY_IP:?}    && echo "zNSb0: RBN_ENCLAVE_SENTRY_IP    = ${RBN_ENCLAVE_SENTRY_IP}"
: ${zRBM_BRIDGE:?}              && echo "zNSb0: zRBM_BRIDGE              = ${zRBM_BRIDGE}"
: ${zRBM_VETH_BOTTLE_IN:?}      && echo "zNSb0: zRBM_VETH_BOTTLE_IN      = ${zRBM_VETH_BOTTLE_IN}"
: ${zRBM_VETH_BOTTLE_OUT:?}     && echo "zNSb0: zRBM_VETH_BOTTLE_OUT     = ${zRBM_VETH_BOTTLE_OUT}"

echo "zNSb1: Getting BOTTLE PID"
BOTTLE_PID=$(podman inspect -f '{{.State.Pid}}' ${RBM_MONIKER}-bottle) || exit 50
[ -n "$BOTTLE_PID" ] || exit 51
echo "zNSb1: BOTTLE PID: $BOTTLE_PID"

echo "zNSb2: Creating and configuring BOTTLE veth pair"
sudo ip link add ${zRBM_VETH_BOTTLE_OUT} type veth peer name ${zRBM_VETH_BOTTLE_IN}                  || exit 60
sudo ip link set ${zRBM_VETH_BOTTLE_IN} netns $BOTTLE_PID                                            || exit 61
sudo nsenter -t $BOTTLE_PID -n ip link set ${zRBM_VETH_BOTTLE_IN} name eth0                          || exit 62
sudo nsenter -t $BOTTLE_PID -n ip addr add ${RBN_ENCLAVE_INITIAL_IP}/${RBN_ENCLAVE_NETMASK} dev eth0 || exit 63
sudo nsenter -t $BOTTLE_PID -n ip link set eth0 up                                                   || exit 64

echo "zNSb3: Connecting BOTTLE veth to bridge"
sudo ip link set ${zRBM_VETH_BOTTLE_OUT} master ${zRBM_BRIDGE} || exit 70
sudo ip link set ${zRBM_VETH_BOTTLE_OUT} up                    || exit 71

echo "zNSb4: Configuring BOTTLE routing"
sudo nsenter -t $BOTTLE_PID -n ip route add default via ${RBN_ENCLAVE_SENTRY_IP} dev eth0 || exit 80

echo "zNSb: Bottle namespace setup complete"
