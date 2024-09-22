#!/bin/sh

set -e
set -x

echo "Checking and displaying environment variables..."
: ${GUARDED_INTERFACE:?}      && echo "GUARDED_INTERFACE=      $GUARDED_INTERFACE"
: ${HOST_INTERFACE:?}         && echo "HOST_INTERFACE=         $HOST_INTERFACE"
: ${SENTRY_GUARDED_IP:?}      && echo "SENTRY_GUARDED_IP=      $SENTRY_GUARDED_IP"
: ${NETWORK_MASK:?}           && echo "NETWORK_MASK=           $NETWORK_MASK"
: ${ROGUE_JUPYTER_PORT:?}     && echo "ROGUE_JUPYTER_PORT=     $ROGUE_JUPYTER_PORT"
: ${SENTRY_JUPYTER_PORT:?}    && echo "SENTRY_JUPYTER_PORT=    $SENTRY_JUPYTER_PORT"
: ${GUARDED_NETWORK_SUBNET:?} && echo "GUARDED_NETWORK_SUBNET= $GUARDED_NETWORK_SUBNET"
: ${ROGUE_IP:?}               && echo "ROGUE_IP=               $ROGUE_IP"

echo "Verifying guarded interface..."
if ! ip link show ${GUARDED_INTERFACE} | grep -q "state UP"; then
    echo "Error: ${GUARDED_INTERFACE} is not up or does not exist"
    exit 1
fi

GUARDED_INDEX=$(ip link show ${GUARDED_INTERFACE} | sed -n 's/^[[:space:]]*\([0-9]\+\):.*/\1/p')

echo "Extracted GUARDED_INDEX: ${GUARDED_INDEX}"

test -n "${GUARDED_INDEX}" || (echo "Error: Failed to extract interface indices" && exit 1)

echo "Guarded interface verified successfully"

echo "Displaying guarded adapter configuration..."
ip addr show $GUARDED_INTERFACE

echo "Setting up iptables rules for service..."
iptables -A FORWARD -i $HOST_INTERFACE -o $GUARDED_INTERFACE -j ACCEPT
iptables -A FORWARD -i $GUARDED_INTERFACE -o $HOST_INTERFACE -p tcp --dport $SENTRY_JUPYTER_PORT -j ACCEPT

echo "Set up NAT..."
iptables -t nat -A POSTROUTING -o $HOST_INTERFACE -j MASQUERADE
iptables -t nat -A POSTROUTING -s $GUARDED_NETWORK_SUBNET ! -d $GUARDED_NETWORK_SUBNET -j MASQUERADE

echo "Set up port forwarding for Jupyter..."
iptables -t nat -A PREROUTING  -i $HOST_INTERFACE    -p tcp --dport $SENTRY_JUPYTER_PORT -j DNAT --to-destination $SENTRY_GUARDED_IP:$ROGUE_JUPYTER_PORT
iptables -t nat -A POSTROUTING -o $GUARDED_INTERFACE -p tcp --dport $ROGUE_JUPYTER_PORT  -j SNAT --to-source      $SENTRY_GUARDED_IP

echo "Starting socat for Jupyter port forwarding..."
socat TCP-LISTEN:${SENTRY_JUPYTER_PORT},fork TCP:${ROGUE_IP}:${ROGUE_JUPYTER_PORT} &

echo "Service setup complete."
