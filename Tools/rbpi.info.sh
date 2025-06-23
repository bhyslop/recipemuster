#!/bin/sh
set -e

echo "RBPIp1: Pod information for ${RBM_MONIKER}"
podman pod ps --filter name=${RBM_MONIKER}-pod
echo "RBPIp2: Containers in pod"
podman ps --filter pod=${RBM_MONIKER}-pod
echo "RBPIp3: Pod network namespace"
podman pod inspect ${RBM_MONIKER}-pod | jq '.InfraContainerID'

