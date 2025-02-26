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
: ${RBM_ENCLAVE_NS_DIR:?}       && echo "RBNS0: RBM_ENCLAVE_NS_DIR       = ${RBM_ENCLAVE_NS_DIR}"

echo "RBNS1: Creating network namespace directory usable by users"
sudo mkdir -p                  ${RBM_ENCLAVE_NS_DIR} || exit 40
sudo chown $(whoami):$(whoami) ${RBM_ENCLAVE_NS_DIR} || exit 41
sudo chmod 755                 ${RBM_ENCLAVE_NS_DIR} || exit 42

# Create the network namespace file path
NS_PATH="${RBM_ENCLAVE_NS_DIR}/${RBM_ENCLAVE_NAMESPACE}"

echo "RBNS1: Creating network namespace with unshare"
unshare                  --net=${NS_PATH} --mount-proc -f /bin/true || exit 50
sudo chown $(whoami):$(whoami) ${NS_PATH}                           || exit 51
sudo chmod 644                 ${NS_PATH}                           || exit 52

echo "RBNS1-DEBUG: Checking namespace creation results..."
echo "RBNS1-DEBUG: Namespace file:"
ls -la ${NS_PATH} || exit 55

echo "RBNS2: Creating and configuring veth pair"
sudo ip link add ${RBM_ENCLAVE_BOTTLE_OUT} type veth peer name ${RBM_ENCLAVE_BOTTLE_IN} || exit 60

echo "RBNS3: Moving veth endpoint to namespace"
# Use nsenter instead of ip netns exec
sudo ip link set ${RBM_ENCLAVE_BOTTLE_IN} netns $(readlink -f ${NS_PATH}) || exit 61

echo "RBNS4: Configuring interface in namespace"
sudo nsenter --net=${NS_PATH} ip link                         set ${RBM_ENCLAVE_BOTTLE_IN} name eth0    || exit 62
sudo nsenter --net=${NS_PATH} ip addr add ${RBRN_ENCLAVE_BOTTLE_IP}/${RBRN_ENCLAVE_NETMASK} dev eth0    || exit 63
sudo nsenter --net=${NS_PATH} ip link                                                       set eth0 up || exit 64
sudo nsenter --net=${NS_PATH} ip route add default via ${RBRN_ENCLAVE_SENTRY_IP}            dev eth0    || exit 65
sudo nsenter --net=${NS_PATH} ip link set lo up || exit 66

echo "RBNS5: Connecting namespace veth to bridge"
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} master ${RBM_ENCLAVE_BRIDGE} || exit 70
sudo ip link set ${RBM_ENCLAVE_BOTTLE_OUT} up || exit 71

echo "RBNS5: Create a symlink from /var/run/netns/ to our namespace for compatibility"
if [ ! -d /var/run/netns ]; then
  sudo mkdir -p /var/run/netns
fi
sudo ln -sf ${NS_PATH} /var/run/netns/${RBM_ENCLAVE_NAMESPACE} 2>/dev/null || true

echo "RBNS6: Namespace permissions check"
ls -la ${NS_PATH}
echo "Mount point info:"
findmnt -t nsfs

echo "RBNS: Bottle namespace setup complete"

