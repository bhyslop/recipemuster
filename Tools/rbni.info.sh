#!/bin/sh
echo "RBNI: Beginning network information report"

set -e
set -x

# Validate required environment variables
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "RBNI0: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RBNI0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "RBNI0: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"

# Define custom netns location
USER_NETNS_DIR=/home/user/users-netns
USER_NETNS_FILE=$USER_NETNS_DIR/${RBM_ENCLAVE_NAMESPACE}

echo "RBNI1: Network namespace information"
echo "RBNI1: Listing all network namespaces:"
sudo ip netns list || echo "RBNI1-INFO: No system namespaces found"

echo "RBNI2: Namespace file system details"
echo "RBNI2a: Checking system namespace file permissions:"
if [ -e "/var/run/netns/${RBM_ENCLAVE_NAMESPACE}" ]; then
  ls -l /var/run/netns/${RBM_ENCLAVE_NAMESPACE}
else
  echo "RBNI2a-INFO: System namespace file not found"
fi

echo "RBNI2b: Checking custom namespace file permissions:"
if [ -e "$USER_NETNS_FILE" ]; then
  ls -l $USER_NETNS_FILE
  echo "RBNI2b-INFO: PID of namespace holder:"
  if [ -e "$USER_NETNS_FILE.pid" ]; then
    cat $USER_NETNS_FILE.pid
    ps -p $(cat $USER_NETNS_FILE.pid) || echo "Process not running"
  else
    echo "No PID file found"
  fi
else
  echo "RBNI2b-ERROR: Custom namespace file not found"
fi

echo "RBNI3: Bridge interface details"
echo "RBNI3: Checking bridge interface status:"
sudo ip link show ${RBM_ENCLAVE_BRIDGE} || echo "RBNI3-ERROR: Bridge interface not found"

echo "RBNI4: Bottle interface details"
echo "RBNI4: Checking bottle outbound interface:"
sudo ip link show ${RBM_ENCLAVE_BOTTLE_OUT} || echo "RBNI4-ERROR: Bottle interface not found"

echo "RBNI5: Namespace interface list"
echo "RBNI5a: Listing interfaces in system namespace:"
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link list 2>/dev/null || echo "RBNI5a-INFO: Cannot list system namespace interfaces"

echo "RBNI5b: Listing interfaces in custom namespace:"
if [ -e "$USER_NETNS_FILE" ]; then
  sudo ip netns exec $USER_NETNS_FILE ip link list 2>/dev/null || echo "RBNI5b-ERROR: Cannot list custom namespace interfaces"
else
  echo "RBNI5b-ERROR: Custom namespace file not found"
fi

echo "RBNI6: Additional interface details"
echo "RBNI6: Bridge interface details:"
sudo ip -d link show ${RBM_ENCLAVE_BRIDGE} || echo "RBNI6-ERROR: Cannot get detailed bridge info"
echo "RBNI6: Bridge forwarding table:"
sudo bridge fdb show dev ${RBM_ENCLAVE_BRIDGE} || echo "RBNI6-ERROR: Cannot show bridge forwarding table"

echo "RBNI: Network information report complete"

