#!/bin/sh

set -e
set -x

echo "Checking and displaying environment variables..."
: ${GUARDED_INTERFACE:?}      && echo "GUARDED_INTERFACE=      $GUARDED_INTERFACE"
: ${SENTRY_GUARDED_IP:?}      && echo "SENTRY_GUARDED_IP=      $SENTRY_GUARDED_IP"
: ${HOST_INTERFACE:?}         && echo "HOST_INTERFACE=         $HOST_INTERFACE"
: ${DNS_SERVER:?}             && echo "DNS_SERVER=             $DNS_SERVER"
: ${GUARDED_NETWORK_SUBNET:?} && echo "GUARDED_NETWORK_SUBNET= $GUARDED_NETWORK_SUBNET"

echo "Setting up iptables rules..."
# Flush existing rules
iptables -F
iptables -t nat -F

# Set default policies
iptables -P INPUT ACCEPT
iptables -P FORWARD DROP
iptables -P OUTPUT ACCEPT

# Allow established and related connections
iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT

# Allow ICMP (ping)
iptables -A FORWARD -p icmp -j ACCEPT

# Allow outbound traffic from HOST_INTERFACE to GUARDED_INTERFACE
iptables -A FORWARD -i $HOST_INTERFACE -o $GUARDED_INTERFACE -j ACCEPT

# Allow inbound traffic to specific port (adjust as needed)
iptables -A FORWARD -i $GUARDED_INTERFACE -o $HOST_INTERFACE -p tcp --dport 8889 -j ACCEPT

# Allow traffic to Anthropic CIDR
iptables -A FORWARD -i $GUARDED_INTERFACE -o $HOST_INTERFACE -d 160.79.104.0/23 -j ACCEPT

# Allow DNS queries
iptables -A INPUT -i $GUARDED_INTERFACE -p udp --dport 53 -j ACCEPT
iptables -A INPUT -i $GUARDED_INTERFACE -p tcp --dport 53 -j ACCEPT

# NAT rules
iptables -t nat -A PREROUTING -i $HOST_INTERFACE -p tcp --dport 8889 -j DNAT --to-destination 10.240.0.2:8888
iptables -t nat -A POSTROUTING -o $HOST_INTERFACE -j MASQUERADE
iptables -t nat -A POSTROUTING -s 10.240.0.0/16 ! -d 10.240.0.0/16 -j MASQUERADE
iptables -t nat -A POSTROUTING -o $GUARDED_INTERFACE -p tcp --dport 8888 -j SNAT --to-source 10.240.0.2

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

## echo "Testing connectivity to Anthropic CIDR..."
## ping -c 4 160.79.104.1 || echo "Ping to Anthropic CIDR failed"
## 
## echo "Testing connectivity to non-Anthropic IP..."
## ping -c 4 8.8.8.8 || echo "Ping to non-Anthropic IP failed (expected)"

echo "Snapshot log for posterity..."
cat /var/log/dnsmasq.log

echo "Setup complete."
