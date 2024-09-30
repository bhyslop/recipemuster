#!/bin/sh

set -e
set -x

echo "Checking and displaying needed environment variables..."
: ${RBEV_SENTRY_HOST_INTERFACE:?}      && echo "RBEV_SENTRY_HOST_INTERFACE = $RBEV_SENTRY_HOST_INTERFACE"
: ${RBEV_DNS_SERVER:?}                 && echo "RBEV_DNS_SERVER            = $RBEV_DNS_SERVER"

echo "Verifying host interface..."
if ! ip link show ${RBEV_SENTRY_HOST_INTERFACE} | grep -q "state UP"; then
    echo  "Error: ${RBEV_SENTRY_HOST_INTERFACE} is not up or does not exist"
    exit 1
fi

HOST_INDEX=$(ip link show ${RBEV_SENTRY_HOST_INTERFACE} | sed -n 's/^[[:space:]]*\([0-9]\+\):.*/\1/p')

echo "Extracted HOST_INDEX:    ${HOST_INDEX}"

test -n "${HOST_INDEX}"    || (echo "Error: Failed to extract interface indices" && exit 1)

echo "Host interface verified successfully"

echo "Displaying host adapter configuration..."
ip addr show $RBEV_SENTRY_HOST_INTERFACE

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
echo "nameserver $RBEV_DNS_SERVER" > /etc/resolv.conf

echo "Host setup complete."
