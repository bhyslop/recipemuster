#!/bin/sh

set -e
set -x

echo "Checking and displaying environment..."
env

echo "Checking and displaying needed environment variables..."
: ${HOST_INTERFACE:?}      && echo "HOST_INTERFACE=      $HOST_INTERFACE"
: ${DNS_SERVER:?}          && echo "DNS_SERVER=          $DNS_SERVER"

echo "Verifying host interface..."
if ! ip link show ${HOST_INTERFACE} | grep -q "state UP"; then
    echo "Error: ${HOST_INTERFACE} is not up or does not exist"
    exit 1
fi

HOST_INDEX=$(ip link show ${HOST_INTERFACE} | sed -n 's/^[[:space:]]*\([0-9]\+\):.*/\1/p')

echo "Extracted HOST_INDEX:    ${HOST_INDEX}"

test -n "${HOST_INDEX}"    || (echo "Error: Failed to extract interface indices" && exit 1)

echo "Host interface verified successfully"

echo "Displaying host adapter configuration..."
ip addr show $HOST_INTERFACE

echo "Displaying all addr info..."
ip addr show

echo "Displaying all route info..."
ip route show

echo "Enabling IP forwarding..."
echo 1 > /proc/sys/net/ipv4/ip_forward

echo "Determining current default gateway..."
CURRENT_GATEWAY=$(ip route | grep default | awk '{print $3}')
CURRENT_GATEWAY_DEV=$(ip route | grep default | awk '{print $5}')

echo "Current gateway: $CURRENT_GATEWAY via $CURRENT_GATEWAY_DEV"

echo "Setting up initial iptables rules..."
iptables -F
iptables -t nat -F

echo "Allow established and related connections..."
iptables -A FORWARD -m state --state ESTABLISHED,RELATED -j ACCEPT

echo "Allow ping passthrough (OUCH diagnostic only?)..."
iptables -A FORWARD -p icmp -j ACCEPT

echo "Setting DNS server in resolv.conf..."
echo "nameserver $DNS_SERVER" > /etc/resolv.conf

echo "Host setup complete."