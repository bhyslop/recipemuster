#!/bin/sh
set -e
echo 'Getting BOTTLE PID...'
BOTTLE_PID=$(podman inspect -f '{{.State.Pid}}' nsproto-bottle)
[ -n "$BOTTLE_PID" ] || exit 31
echo 'BOTTLE PID: '$BOTTLE_PID
echo 'Setting up BOTTLE networking...'
sudo ip link add vbo_nsproto type veth peer name vbi_nsproto
sudo ip link set vbi_nsproto netns $BOTTLE_PID
sudo nsenter -t $BOTTLE_PID -n ip link set vbi_nsproto name eth1
sudo nsenter -t $BOTTLE_PID -n ip addr add 10.242.0.3/24 dev eth1
sudo nsenter -t $BOTTLE_PID -n ip link set eth1 up
sudo ip link set vbo_nsproto master vbr_nsproto
sudo ip link set vbo_nsproto up
echo 'Configuring BOTTLE routing...'
sudo nsenter -t $BOTTLE_PID -n ip route add default via 10.242.0.2 dev eth1
