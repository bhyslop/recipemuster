#!/bin/sh

set -e
set -x

echo "Checking and displaying environment variables..."
: ${GUARDED_INTERFACE:?}      && echo "GUARDED_INTERFACE=      $GUARDED_INTERFACE"
: ${SENTRY_GUARDED_IP:?}      && echo "SENTRY_GUARDED_IP=      $SENTRY_GUARDED_IP"
: ${HOST_INTERFACE:?}         && echo "HOST_INTERFACE=         $HOST_INTERFACE"
: ${DNS_SERVER:?}             && echo "DNS_SERVER=             $DNS_SERVER"
: ${GUARDED_NETWORK_SUBNET:?} && echo "GUARDED_NETWORK_SUBNET= $GUARDED_NETWORK_SUBNET"

echo "Setting up iptables rules for outbound access..."
iptables -A FORWARD -i $GUARDED_INTERFACE -o $HOST_INTERFACE -p udp --dport 53 -j ACCEPT
iptables -A FORWARD -i $GUARDED_INTERFACE -o $HOST_INTERFACE -p tcp --dport 53 -j ACCEPT
iptables -A FORWARD -i $GUARDED_INTERFACE -o $HOST_INTERFACE -d 160.79.104.0/23 -j ACCEPT

echo "Allow incoming DNS queries..."
iptables -A INPUT -i $GUARDED_INTERFACE -p udp --dport 53 -j ACCEPT
iptables -A INPUT -i $GUARDED_INTERFACE -p tcp --dport 53 -j ACCEPT

echo "Setting up dnsmasq..."
cat > /etc/dnsmasq.conf << EOF
server=${DNS_SERVER}
listen-address=${SENTRY_GUARDED_IP}
bind-interfaces
interface=${GUARDED_INTERFACE}
no-hosts
log-queries
log-facility=/var/log/dnsmasq.log
EOF

echo "Starting dnsmasq..."
dnsmasq

echo "Outbound setup complete."

echo "Displaying final network configuration..."
ip addr show
ip route show
iptables -L -v -n
iptables -t nat -L -v -n

echo "Testing DNS resolution..."
nslookup google.com || echo "DNS resolution failed"

echo "Testing connectivity to current gateway..."
CURRENT_GATEWAY=$(ip route | grep default | awk '{print $3}')
ping -c 4 $CURRENT_GATEWAY || echo "Ping to current gateway failed"

echo "Snapshot log for posterity..."
cat /var/log/dnsmasq.log

echo "Setup complete."
