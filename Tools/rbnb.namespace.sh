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

# Create a tempfile that we'll use to pass the network namespace FD to the next stage
TEMP_DIR=/tmp/netns-setup
mkdir -p $TEMP_DIR
echo "RBNS1: Creating network namespace reference"

# Create a file with our current PID and NS info
echo $$ > $TEMP_DIR/${RBM_ENCLAVE_NAMESPACE}.pid
readlink /proc/self/ns/net > $TEMP_DIR/${RBM_ENCLAVE_NAMESPACE}.ns

echo "RBNS2: Network namespace created with PID $$ and NS $(readlink /proc/self/ns/net)"
echo "RBNS3: Tempfiles created at $TEMP_DIR/${RBM_ENCLAVE_NAMESPACE}.pid and $TEMP_DIR/${RBM_ENCLAVE_NAMESPACE}.ns"

# Now we keep this process running to maintain the namespace
echo "RBNS4: This process will keep running in background to maintain the namespace..."

# Launch background process to hold namespace open
(sleep 3600 &) # Keep running for an hour

# Sleep briefly so the background process can establish
sleep 5

echo "RBNS5: Namespace setup complete, background process running with PID $$"


