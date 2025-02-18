#!/bin/sh
echo "RBNI: Beginning network information report"

set -e
set -x

# Validate required environment variables
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "RBNI0: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RBNI0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "RBNI0: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"

echo "RBNI1: Network namespace information"
echo "RBNI1: Listing all network namespaces:"
sudo ip netns list || echo "RBNI1-ERROR: No namespaces found"

echo "RBNI2: Namespace file system details"
echo "RBNI2: Checking namespace file permissions:"
ls -l /var/run/netns/${RBM_ENCLAVE_NAMESPACE} || echo "RBNI2-ERROR: Namespace file not found"

echo "RBNI3: Bridge interface details"
echo "RBNI3: Checking bridge interface status:"
sudo ip link show ${RBM_ENCLAVE_BRIDGE} || echo "RBNI3-ERROR: Bridge interface not found"

echo "RBNI4: Bottle interface details"
echo "RBNI4: Checking bottle outbound interface:"
sudo ip link show ${RBM_ENCLAVE_BOTTLE_OUT} || echo "RBNI4-ERROR: Bottle interface not found"

echo "RBNI5: Namespace interface list"
echo "RBNI5: Listing interfaces in namespace:"
sudo ip netns exec ${RBM_ENCLAVE_NAMESPACE} ip link list || echo "RBNI5-ERROR: Cannot list namespace interfaces"

echo "RBNI6: Additional interface details"
echo "RBNI6: Bridge interface details:"
sudo ip -d link show ${RBM_ENCLAVE_BRIDGE} || echo "RBNI6-ERROR: Cannot get detailed bridge info"
echo "RBNI6: Bridge forwarding table:"
sudo bridge fdb show dev ${RBM_ENCLAVE_BRIDGE} || echo "RBNI6-ERROR: Cannot show bridge forwarding table"

echo "RBNI: Network information report complete"

