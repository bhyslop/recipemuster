#!/bin/sh
echo "RBNC: Beginning network cleanup script"

set -e
set -x

# Validate required environment variables
: ${RBM_ENCLAVE_SENTRY_OUT:?}   && echo "RBNC0: RBM_ENCLAVE_SENTRY_OUT   = ${RBM_ENCLAVE_SENTRY_OUT}"
: ${RBM_ENCLAVE_SENTRY_IN:?}    && echo "RBNC0: RBM_ENCLAVE_SENTRY_IN    = ${RBM_ENCLAVE_SENTRY_IN}"
: ${RBM_ENCLAVE_BOTTLE_OUT:?}   && echo "RBNC0: RBM_ENCLAVE_BOTTLE_OUT   = ${RBM_ENCLAVE_BOTTLE_OUT}"
: ${RBM_ENCLAVE_BOTTLE_IN:?}    && echo "RBNC0: RBM_ENCLAVE_BOTTLE_IN    = ${RBM_ENCLAVE_BOTTLE_IN}"
: ${RBM_ENCLAVE_BRIDGE:?}       && echo "RBNC0: RBM_ENCLAVE_BRIDGE       = ${RBM_ENCLAVE_BRIDGE}"

# We no longer need to remove network namespace as it's managed by Podman
# echo "RBNC1: Removing network namespace"
# sudo ip netns del ${RBM_ENCLAVE_NAMESPACE} 2>/dev/null || true

echo "RBNC2: Cleaning up SENTRY interfaces"
sudo ip link del ${RBM_ENCLAVE_SENTRY_OUT} 2>/dev/null || true
sudo ip link del ${RBM_ENCLAVE_SENTRY_IN} 2>/dev/null || true

echo "RBNC3: Cleaning up BOTTLE interfaces"
sudo ip link del ${RBM_ENCLAVE_BOTTLE_OUT} 2>/dev/null || true
sudo ip link del ${RBM_ENCLAVE_BOTTLE_IN} 2>/dev/null || true

echo "RBNC4: Removing bridge"
sudo ip link del ${RBM_ENCLAVE_BRIDGE} 2>/dev/null || true

echo "RBNC5: Verifying cleanup"
echo "RBNC5-DEBUG: Remaining interfaces:"
ip link show | grep -E "${RBM_ENCLAVE_SENTRY_OUT}|${RBM_ENCLAVE_BOTTLE_OUT}|${RBM_ENCLAVE_BRIDGE}" || echo "No matching interfaces found"

echo "RBNC: Network cleanup complete"
