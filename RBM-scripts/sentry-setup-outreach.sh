#!/bin/sh

set -e
set -x

echo "Checking and displaying environment variables..."
: ${RBEV_SENTRY_GUARDED_INTERFACE:?} && echo "RBEV_SENTRY_GUARDED_INTERFACE = $RBEV_SENTRY_GUARDED_INTERFACE"
: ${RBEV_SENTRY_HOST_INTERFACE:?}    && echo "RBEV_SENTRY_HOST_INTERFACE    = $RBEV_SENTRY_HOST_INTERFACE"
: ${RBEV_SENTRY_GUARDED_IP:?}        && echo "RBEV_SENTRY_GUARDED_IP        = $RBEV_SENTRY_GUARDED_IP"
: ${RBEV_GUARDED_NETMASK:?}          && echo "RBEV_GUARDED_NETMASK          = $RBEV_GUARDED_NETMASK"
: ${RBEV_ROGUE_JUPYTER_PORT:?}       && echo "RBEV_ROGUE_JUPYTER_PORT       = $RBEV_ROGUE_JUPYTER_PORT"
: ${RBEV_SENTRY_JUPYTER_PORT:?}      && echo "RBEV_SENTRY_JUPYTER_PORT      = $RBEV_SENTRY_JUPYTER_PORT"
: ${RBEV_GUARDED_NETWORK_SUBNET:?}   && echo "RBEV_GUARDED_NETWORK_SUBNET   = $RBEV_GUARDED_NETWORK_SUBNET"
: ${RBEV_ROGUE_IP:?}                 && echo "RBEV_ROGUE_IP                 = $RBEV_ROGUE_IP"

echo "Verifying guarded interface..."
if ! ip link show ${RBEV_SENTRY_GUARDED_INTERFACE} | grep -q "state UP"; then
    echo  "Error: ${RBEV_SENTRY_GUARDED_INTERFACE} is not up or does not exist"
    exit 1
fi

GUARDED_INDEX=$(ip link show ${RBEV_SENTRY_GUARDED_INTERFACE} | sed -n 's/^[[:space:]]*\([0-9]\+\):.*/\1/p')

echo "Extracted GUARDED_INDEX: ${GUARDED_INDEX}"

test -n "${GUARDED_INDEX}" || (echo "Error: Failed to extract interface indices" && exit 1)

echo "Guarded interface verified successfully"

echo "Displaying guarded adapter configuration..."
ip addr show $RBEV_SENTRY_GUARDED_INTERFACE

echo "Setting up iptables rules for service..."
iptables -A FORWARD -i $RBEV_SENTRY_HOST_INTERFACE -o $RBEV_SENTRY_GUARDED_INTERFACE -j ACCEPT
iptables -A FORWARD -i $RBEV_SENTRY_GUARDED_INTERFACE -o $RBEV_SENTRY_HOST_INTERFACE -p tcp --dport $RBEV_SENTRY_JUPYTER_PORT -j ACCEPT

echo "Set up NAT..."
iptables -t nat -A POSTROUTING -o $RBEV_SENTRY_HOST_INTERFACE -j MASQUERADE
iptables -t nat -A POSTROUTING -s $RBEV_GUARDED_NETWORK_SUBNET ! -d $RBEV_GUARDED_NETWORK_SUBNET -j MASQUERADE

echo "Set up port forwarding for Jupyter..."
iptables -t nat -A PREROUTING  -i $RBEV_SENTRY_HOST_INTERFACE    -p tcp --dport $RBEV_SENTRY_JUPYTER_PORT -j DNAT --to-destination $RBEV_SENTRY_GUARDED_IP:$RBEV_ROGUE_JUPYTER_PORT
iptables -t nat -A POSTROUTING -o $RBEV_SENTRY_GUARDED_INTERFACE -p tcp --dport $RBEV_ROGUE_JUPYTER_PORT  -j SNAT --to-source      $RBEV_SENTRY_GUARDED_IP

echo "Starting socat for Jupyter port forwarding..."
socat TCP-LISTEN:${RBEV_SENTRY_JUPYTER_PORT},fork TCP:${RBEV_ROGUE_IP}:${RBEV_ROGUE_JUPYTER_PORT} &

echo "Service setup complete."
