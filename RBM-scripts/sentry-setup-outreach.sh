#!/bin/sh

set -e
set -x

echo "Checking and displaying environment variables..."
: ${RBEV_SENTRY_GUARDED_INTERFACE:?} && echo "RBEV_SENTRY_GUARDED_INTERFACE = $RBEV_SENTRY_GUARDED_INTERFACE"
: ${RBEV_SENTRY_GUARDED_IP:?}        && echo "RBEV_SENTRY_GUARDED_IP        = $RBEV_SENTRY_GUARDED_IP"
: ${RBEV_SENTRY_HOST_INTERFACE:?}    && echo "RBEV_SENTRY_HOST_INTERFACE    = $RBEV_SENTRY_HOST_INTERFACE"
: ${RBEV_DNS_SERVER:?}               && echo "RBEV_DNS_SERVER               = $RBEV_DNS_SERVER"
: ${RBEV_GUARDED_NETWORK_SUBNET:?}   && echo "RBEV_GUARDED_NETWORK_SUBNET   = $RBEV_GUARDED_NETWORK_SUBNET"

echo "Setting up iptables rules for anthropic specific access..."
iptables -A FORWARD -i $RBEV_SENTRY_GUARDED_INTERFACE -o $RBEV_SENTRY_HOST_INTERFACE -d 160.79.104.0/23 -j ACCEPT

echo "Allow incoming DNS queries..."
iptables -A INPUT -i $RBEV_SENTRY_GUARDED_INTERFACE -p udp --dport 53 -j ACCEPT
iptables -A INPUT -i $RBEV_SENTRY_GUARDED_INTERFACE -p tcp --dport 53 -j ACCEPT

echo "Setting up dnsmasq..."
cat > /etc/dnsmasq.conf << EOF
server=${RBEV_DNS_SERVER}
listen-address=${RBEV_SENTRY_GUARDED_IP}
bind-interfaces
interface=${RBEV_SENTRY_GUARDED_INTERFACE}
no-hosts
log-queries
log-facility=/var/log/dnsmasq.log
EOF

echo "Starting dnsmasq..."
dnsmasq -d -q &> /var/log/dnsmasq_verbose.log
sleep 2  # Give dnsmasq time to start

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
