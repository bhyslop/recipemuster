#!/bin/sh
set -e

: ${RBRR_BOTTLE_UID:?}             && echo "RBSp0: RBRR_BOTTLE_UID             = ${RBRR_BOTTLE_UID}"
: ${RBRR_DNS_SERVER:?}             && echo "RBSp0: RBRR_DNS_SERVER             = ${RBRR_DNS_SERVER}"
: ${RBRN_ENTRY_ENABLED:?}          && echo "RBSp0: RBRN_ENTRY_ENABLED          = ${RBRN_ENTRY_ENABLED}"
: ${RBRN_ENTRY_PORT_WORKSTATION:?} && echo "RBSp0: RBRN_ENTRY_PORT_WORKSTATION = ${RBRN_ENTRY_PORT_WORKSTATION}"
: ${RBRN_ENTRY_PORT_ENCLAVE:?}     && echo "RBSp0: RBRN_ENTRY_PORT_ENCLAVE     = ${RBRN_ENTRY_PORT_ENCLAVE}"
: ${RBRN_UPLINK_DNS_ENABLED:?}     && echo "RBSp0: RBRN_UPLINK_DNS_ENABLED     = ${RBRN_UPLINK_DNS_ENABLED}"
: ${RBRN_UPLINK_ACCESS_ENABLED:?}  && echo "RBSp0: RBRN_UPLINK_ACCESS_ENABLED  = ${RBRN_UPLINK_ACCESS_ENABLED}"
: ${RBRN_UPLINK_DNS_GLOBAL:?}      && echo "RBSp0: RBRN_UPLINK_DNS_GLOBAL      = ${RBRN_UPLINK_DNS_GLOBAL}"
: ${RBRN_UPLINK_ACCESS_GLOBAL:?}   && echo "RBSp0: RBRN_UPLINK_ACCESS_GLOBAL   = ${RBRN_UPLINK_ACCESS_GLOBAL}"
: ${RBRN_UPLINK_ALLOWED_CIDRS:?}   && echo "RBSp0: RBRN_UPLINK_ALLOWED_CIDRS   = ${RBRN_UPLINK_ALLOWED_CIDRS}"
: ${RBRN_UPLINK_ALLOWED_DOMAINS:?} && echo "RBSp0: RBRN_UPLINK_ALLOWED_DOMAINS = ${RBRN_UPLINK_ALLOWED_DOMAINS}"

echo "RBSp1: Phase 1 - IPTables Initialization"

echo "RBSp1: Flushing existing rules"
iptables -F        || exit 10
iptables -t nat -F || exit 10

echo "RBSp1: Setting default policies to DROP"
iptables -P INPUT DROP   || exit 10
iptables -P FORWARD DROP || exit 10
iptables -P OUTPUT DROP  || exit 10

echo "RBSp1: Accepting loopback traffic"
iptables -I INPUT -i lo -j ACCEPT || exit 10

echo "RBSp1: Setting up connection tracking"
iptables -I INPUT   2 -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -I FORWARD 2 -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -I OUTPUT  2 -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10

echo "RBSp1: Creating RBM chains"
iptables -N RBM-INGRESS || exit 10
iptables -N RBM-EGRESS  || exit 10
iptables -N RBM-FORWARD || exit 10

echo "RBSp1: Setting up chain jumps"
iptables -I INPUT   3 -j RBM-INGRESS || exit 10
iptables -I OUTPUT  3 -j RBM-EGRESS  || exit 10
iptables -I FORWARD 3 -j RBM-FORWARD || exit 10

echo "RBSp2: Phase 2 - Port Setup"
if [ "${RBRN_ENTRY_ENABLED}" = "true" ]; then
    echo "RBSp2: Configuring service port access"
    iptables -A RBM-INGRESS -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} -j ACCEPT || exit 20
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --sport ${RBRN_ENTRY_PORT_ENCLAVE} -j ACCEPT || exit 20
else
    echo "RBSp2: Port access disabled"
fi

echo "RBSp3: Phase 3 - Access Setup"
if [ "${RBRN_UPLINK_ACCESS_ENABLED}" = "false" ]; then
    echo "RBSp3: Blocking all bottle network access"
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -j DROP || exit 30
else
    echo "RBSp3: Configuring network access"
    
    echo "RBSp3: Allowing DNS server access for bottle"
    if [ -n "${RBRR_DNS_SERVER}" ]; then
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp -d ${RBRR_DNS_SERVER} --dport 53 -j ACCEPT || exit 30
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp -d ${RBRR_DNS_SERVER} --dport 53 -j ACCEPT || exit 30
    fi

    if [ "${RBRN_UPLINK_ACCESS_GLOBAL}" = "true" ]; then
        echo "RBSp3: Enabling global access for bottle"
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -j ACCEPT || exit 31
    else
        echo "RBSp3: Configuring CIDR-based access"
        for cidr in ${RBRN_UPLINK_ALLOWED_CIDRS}; do
            echo "RBSp3: Allowing access to ${cidr}"
            iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -d ${cidr} -j ACCEPT || exit 32
        done
        echo "RBSp3: Dropping all other bottle traffic"
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -j DROP || exit 30
    fi
fi

echo "RBSp4: Phase 4 - DNS Configuration"
if [ "${RBRN_UPLINK_DNS_ENABLED}" = "false" ]; then
    echo "RBSp4: Blocking all DNS traffic from bottle"
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp --dport 53 -j DROP || exit 40
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --dport 53 -j DROP || exit 40
else
    echo "RBSp4: Validating DNS server connectivity"
    if [ -n "${RBRR_DNS_SERVER}" ]; then
        echo "RBSp4: Testing TCP connection to DNS server"
        timeout 5 nc -zv ${RBRR_DNS_SERVER} 53 || exit 40
        echo "RBSp4: Testing UDP DNS query"
        timeout 6 nslookup -type=NS . ${RBRR_DNS_SERVER} || exit 40
    fi

    echo "RBSp4: Configuring dnsmasq"
    echo "interface=*"                                     > /etc/dnsmasq.conf || exit 41
    echo "listen-address=127.0.0.1"                       >> /etc/dnsmasq.conf || exit 41
    echo "bind-interfaces"                                >> /etc/dnsmasq.conf || exit 41
    echo "port=53"                                        >> /etc/dnsmasq.conf || exit 41
    echo "no-dhcp-interface=*"                            >> /etc/dnsmasq.conf || exit 41
    echo "cache-size=1000"                                >> /etc/dnsmasq.conf || exit 41
    echo "min-cache-ttl=600"                              >> /etc/dnsmasq.conf || exit 41
    echo "max-cache-ttl=3600"                             >> /etc/dnsmasq.conf || exit 41
    echo "no-resolv"                                      >> /etc/dnsmasq.conf || exit 41
    echo "strict-order"                                   >> /etc/dnsmasq.conf || exit 41
    echo "log-queries=extra"                              >> /etc/dnsmasq.conf || exit 41
    echo "log-facility=/var/log/dnsmasq.log"              >> /etc/dnsmasq.conf || exit 41
    echo "log-dhcp"                                       >> /etc/dnsmasq.conf || exit 41
    echo "log-debug"                                      >> /etc/dnsmasq.conf || exit 41
    echo "log-async=20"                                   >> /etc/dnsmasq.conf || exit 41

    if [ "${RBRN_UPLINK_DNS_GLOBAL}" = "true" ]; then
        echo "RBSp4: Enabling global DNS resolution"
        echo "server=${RBRR_DNS_SERVER}" >> /etc/dnsmasq.conf || exit 41
    else
        echo "RBSp4: Configuring domain-based DNS filtering"
        for domain in ${RBRN_UPLINK_ALLOWED_DOMAINS}; do
            echo "server=/${domain}/${RBRR_DNS_SERVER}" >> /etc/dnsmasq.conf || exit 41
        done
        echo "address=/#/" >> /etc/dnsmasq.conf || exit 41
    fi

    echo "RBSp4: Starting dnsmasq"
    killall dnsmasq 2>/dev/null || true
    sleep 1
    dnsmasq || exit 42

    echo "RBSp4: Setting up DNS interception"
    iptables -A RBM-EGRESS    -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp --dport 53 -d 127.0.0.1 -j ACCEPT    || exit 43
    iptables -A RBM-EGRESS    -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --dport 53 -d 127.0.0.1 -j ACCEPT    || exit 43
    iptables -t nat -A OUTPUT -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp --dport 53 -j REDIRECT --to-ports 53 || exit 43
    iptables -t nat -A OUTPUT -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --dport 53 -j REDIRECT --to-ports 53 || exit 43
fi

echo "RBSp5: Security configuration complete"

