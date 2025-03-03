#!/bin/sh
echo "RBNS: Beginning network namespace setup script"

set -e
set -x

# Validate required environment variables
: ${RBM_ENCLAVE_NAMESPACE:?}    && echo "RBNS0: RBM_ENCLAVE_NAMESPACE    = ${RBM_ENCLAVE_NAMESPACE}"

# Check if we're running as root in a user namespace
if [ "$(id -u)" -eq 0 ]; then
  echo "RBNS-NS: Running as root in user namespace, good."
else
  echo "RBNS-NS: ERROR - Not running as root in a user namespace."
  echo "RBNS-NS: This script should be run inside 'unshare -r -n -m' context."
  exit 1
fi

# Setup the user-accessible network namespace directory
echo "RBNS1: Setting up user-accessible network namespace directory"
USER_NETNS_DIR=/home/user/users-netns
mkdir -p $USER_NETNS_DIR
mount -t tmpfs netns $USER_NETNS_DIR

# Create our namespace reference file
NAMESPACE_FILE=$USER_NETNS_DIR/${RBM_ENCLAVE_NAMESPACE}
echo "RBNS2: Creating network namespace file at $NAMESPACE_FILE"

# We're already in a network namespace (from unshare -n)
# So we just need to save a reference to the current namespace
echo "RBNS3: Saving reference to current network namespace"
touch $NAMESPACE_FILE
mount --bind /proc/self/ns/net $NAMESPACE_FILE

echo "RBNS4: Verifying namespace file exists"
ls -la $NAMESPACE_FILE || echo "WARNING: Namespace file not found!"

echo "RBNS5: Network namespace creation complete"
echo "RBNS5: Namespace location: $NAMESPACE_FILE"

