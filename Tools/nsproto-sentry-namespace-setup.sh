#!/bin/sh
set -e
echo 'Can we see sentry here, inspect intimate...'
podman inspect -f '{{.State.Pid}}' nsproto-sentry
echo 'Getting SENTRY PID...'
SENTRY_PID=$(PODMAN_IGNORE_CGROUPSV1_WARNING=1 podman inspect -f '{{.State.Pid}}' nsproto-sentry)
[ -n "$SENTRY_PID" ] || exit 31
echo 'SENTRY PID: '$SENTRY_PID
echo 'Setting up bridge and SENTRY networking...'
echo 'Creating bridge vbr_nsproto...'
sudo ip link add name vbr_nsproto type bridge
sudo ip link set vbr_nsproto up
sudo ip link add vso_nsproto type veth peer name vsi_nsproto
sudo ip link set vsi_nsproto netns $SENTRY_PID
sudo nsenter -t $SENTRY_PID -n ip link set vsi_nsproto name eth1
sudo nsenter -t $SENTRY_PID -n ip addr add 10.242.0.2/24 dev eth1
sudo nsenter -t $SENTRY_PID -n ip link set eth1 up
sudo ip link set vso_nsproto master vbr_nsproto
sudo ip link set vso_nsproto up
