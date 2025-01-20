#!/bin/sh
echo "RSNS: Beginning sentry namespace setup script"

set -e
set -x

# Validate required environment variables
: ${RBM_SENTRY_CONTAINER:?}     && echo "RSNS0: RBM_SENTRY_CONTAINER     = ${RBM_SENTRY_CONTAINER}"
: ${RBN_ENCLAVE_SENTRY_IP:?}    && echo "RSNS0: RBN_ENCLAVE_SENTRY_IP    = ${RBN_ENCLAVE_SENTRY_IP}"
: ${RBN_ENCLAVE_NETMASK:?}      && echo "RSNS0: RBN_ENCLAVE_NETMASK      = ${RBN_ENCLAVE_NETMASK}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RSNS0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBM_ENCLAVE_SENTRY_IN:?}    && echo "RSNS0: RBM_ENCLAVE_SENTRY_IN    = ${RBM_ENCLAVE_SENTRY_IN}"
: ${RBM_ENCLAVE_SENTRY_OUT:?}   && echo "RSNS0: RBM_ENCLAVE_SENTRY_OUT   = ${RBM_ENCLAVE_SENTRY_OUT}"

echo "RSNS1: Getting SENTRY PID"
SENTRY_PID=$(podman inspect -f '{{.State.Pid}}' ${RBM_SENTRY_CONTAINER}) || exit 50
[ -n "$SENTRY_PID" ] || exit 51
echo "RSNS1: SENTRY PID: $SENTRY_PID"

echo "RSNS2: Creating and configuring bridge"
sudo ip link add name ${RBM_ENCLAVE_BRIDGE} type bridge || exit 60
sudo ip link set      ${RBM_ENCLAVE_BRIDGE} up          || exit 61

echo "RSNS3: Creating and configuring SENTRY veth pair"
sudo ip link add ${RBM_ENCLAVE_SENTRY_OUT} type veth peer name ${RBM_ENCLAVE_SENTRY_IN}             || exit 70
sudo ip link set ${RBM_ENCLAVE_SENTRY_IN} netns $SENTRY_PID                                         || exit 71
sudo nsenter -t $SENTRY_PID -n ip link set ${RBM_ENCLAVE_SENTRY_IN} name eth1                       || exit 72
sudo nsenter -t $SENTRY_PID -n ip addr add ${RBN_ENCLAVE_SENTRY_IP}/${RBN_ENCLAVE_NETMASK} dev eth1 || exit 73
sudo nsenter -t $SENTRY_PID -n ip link set eth1 up                                                  || exit 74

echo "RSNS4: Connecting SENTRY veth to bridge"
sudo ip link set ${RBM_ENCLAVE_SENTRY_OUT} master ${RBM_ENCLAVE_BRIDGE} || exit 80
sudo ip link set ${RBM_ENCLAVE_SENTRY_OUT} up                           || exit 81

echo "RSNS: Sentry namespace setup complete"

