#!/bin/sh
echo "RBS: Beginning sentry setup script"

set -e
set -x

: ${RBN_ENCLAVE_BASE_IP:?}        && echo "RBSp0: RBN_ENCLAVE_BASE_IP        = ${RBN_ENCLAVE_BASE_IP}"
: ${RBN_ENCLAVE_NETMASK:?}        && echo "RBSp0: RBN_ENCLAVE_NETMASK        = ${RBN_ENCLAVE_NETMASK}"
: ${RBN_ENCLAVE_SENTRY_IP:?}      && echo "RBSp0: RBN_ENCLAVE_SENTRY_IP      = ${RBN_ENCLAVE_SENTRY_IP}"
: ${RBN_ENCLAVE_BOTTLE_IP:?}      && echo "RBSp0: RBN_ENCLAVE_BOTTLE_IP      = ${RBN_ENCLAVE_BOTTLE_IP}"
: ${RBB_DNS_SERVER:?}             && echo "RBSp0: RBB_DNS_SERVER             = ${RBB_DNS_SERVER}"
: ${RBN_PORT_ENABLED:?}           && echo "RBSp0: RBN_PORT_ENABLED           = ${RBN_PORT_ENABLED}"
: ${RBN_ENTRY_PORT_WORKSTATION:?} && echo "RBSp0: RBN_ENTRY_PORT_WORKSTATION = ${RBN_ENTRY_PORT_WORKSTATION}"
: ${RBN_ENTRY_PORT_ENCLAVE:?}     && echo "RBSp0: RBN_ENTRY_PORT_ENCLAVE     = ${RBN_ENTRY_PORT_ENCLAVE}"
: ${RBN_UPLINK_DNS_ENABLED:?}     && echo "RBSp0: RBN_UPLINK_DNS_ENABLED     = ${RBN_UPLINK_DNS_ENABLED}"
: ${RBN_UPLINK_ACCESS_ENABLED:?}  && echo "RBSp0: RBN_UPLINK_ACCESS_ENABLED  = ${RBN_UPLINK_ACCESS_ENABLED}"
: ${RBN_UPLINK_DNS_GLOBAL:?}      && echo "RBSp0: RBN_UPLINK_DNS_GLOBAL      = ${RBN_UPLINK_DNS_GLOBAL}"
: ${RBN_UPLINK_ACCESS_GLOBAL:?}   && echo "RBSp0: RBN_UPLINK_ACCESS_GLOBAL   = ${RBN_UPLINK_ACCESS_GLOBAL}"
: ${RBN_UPLINK_ALLOWED_CIDRS:?}   && echo "RBSp0: RBN_UPLINK_ALLOWED_CIDRS   = ${RBN_UPLINK_ALLOWED_CIDRS}"
: ${RBN_UPLINK_ALLOWED_DOMAINS:?} && echo "RBSp0: RBN_UPLINK_ALLOWED_DOMAINS = ${RBN_UPLINK_ALLOWED_DOMAINS}"

echo "RBSp1: Beginning IPTables initialization"

echo "RBSp1: Flushing existing rules"
iptables -F        || exit 10
iptables -t nat -F || exit 10

echo "RBSp1: Setting default policies"
iptables -P INPUT   DROP || exit 10
iptables -P FORWARD DROP || exit 10
iptables -P OUTPUT  DROP || exit 10

echo "RBSp1: Configuring loopback access"
iptables -A INPUT  -i lo -j ACCEPT || exit 10
iptables -A OUTPUT -o lo -j ACCEPT || exit 10

echo "RBSp1: Setting up connection tracking"
iptables -A INPUT   -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -A OUTPUT  -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10

echo "RBSp1: Creating RBM chains"
iptables -N RBM-INGRESS || exit 10
iptables -N RBM-EGRESS  || exit 10
iptables -N RBM-FORWARD || exit 10

echo "RBSp1: Setting up chain jumps"
iptables -A INPUT   -j RBM-INGRESS || exit 10
iptables -A OUTPUT  -j RBM-EGRESS  || exit 10
iptables -A FORWARD -j RBM-FORWARD || exit 10

echo "RBSp2: Phase 2: Port Setup"
if [ "${RBN_PORT_ENABLED}" = "1" ]; then
    echo "RBSp2: Configuring port forwarding"
    
    echo "RBSp2: Setting up DNAT rules"
    iptables -t nat -A PREROUTING -i eth0 -p tcp --dport "${RBN_ENTRY_PORT_WORKSTATION}"   \
             -j DNAT --to-destination "${RBN_ENCLAVE_SENTRY_IP}:${RBN_ENTRY_PORT_ENCLAVE}" \
             -m comment --comment "RBM-PORT-FORWARD" || exit 20

    echo "RBSp2: Configuring port filter rules"
    iptables -A RBM-INGRESS -i eth0 -p tcp --dport "${RBN_ENTRY_PORT_WORKSTATION}" -m state --state NEW                     -j ACCEPT || exit 20
    iptables -A RBM-FORWARD -i eth1 -p tcp --sport "${RBN_ENTRY_PORT_ENCLAVE}"     -m state --state NEW,RELATED,ESTABLISHED -j ACCEPT || exit 20
fi

echo "RBSp3: Phase 3: Access Setup"
if [ "${RBN_UPLINK_ACCESS_ENABLED}" = "0" ]; then
    echo "RBSp3: Blocking all non-port traffic"
    iptables -A RBM-EGRESS  -o eth0 -j DROP || exit 30
    iptables -A RBM-FORWARD -i eth1 -j DROP || exit 30
else
    echo "RBSp3: Setting up network forwarding"
    echo 1 > /proc/sys/net/ipv4/ip_forward               || exit 31
    echo 0 > /proc/sys/net/ipv6/conf/all/disable_ipv6    || exit 31
    echo 1 > /proc/sys/net/ipv4/conf/all/rp_filter       || exit 31
    echo 1 > /proc/sys/net/ipv4/conf/eth0/route_localnet || exit 31

    echo "RBSp3: Configuring NAT"
    iptables -t nat -A POSTROUTING -o eth0 -s "${RBN_ENCLAVE_BASE_IP}/${RBN_ENCLAVE_NETMASK}" -j MASQUERADE || exit 31

    if [ "${RBN_UPLINK_ACCESS_GLOBAL}" = "1" ]; then
        echo "RBSp3: Enabling global access"
        iptables -A RBM-EGRESS  -o eth0 -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -j ACCEPT || exit 31
    else
        echo "RBSp3: Configuring DNS server access"
        iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d "${RBB_DNS_SERVER}" -j ACCEPT || exit 31
        iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d "${RBB_DNS_SERVER}" -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -d "${RBB_DNS_SERVER}" -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -d "${RBB_DNS_SERVER}" -j ACCEPT || exit 31

        echo "RBSp3: Setting up CIDR-based access control"
        for cidr in ${RBN_UPLINK_ALLOWED_CIDRS}; do
            iptables -A RBM-EGRESS  -o eth0 -d "${cidr}" -j ACCEPT || exit 32
            iptables -A RBM-FORWARD -i eth1 -d "${cidr}" -j ACCEPT || exit 32
        done
    fi
fi

echo "RBSp4: Configuring DNS services"

echo "RBSp4: Configuring sentry DNS resolution"
echo "nameserver ${RBB_DNS_SERVER}" > /etc/resolv.conf   || exit 40

if [ "${RBN_UPLINK_DNS_ENABLED}" = "0" ]; then
    echo "RBSp4: Blocking all DNS traffic"
    iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -j DROP || exit 40
    iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -j DROP || exit 40
    iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -j DROP || exit 40
    iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -j DROP || exit 40
else
    echo "RBSp4: Testing DNS server connectivity"
    timeout 5s nc -z "${RBB_DNS_SERVER}" 53                   || exit 40
    timeout 5s dig  @"${RBB_DNS_SERVER}" .                    || exit 40

    echo "RBSp4: Process cleanup"
    killall -9 dnsmasq 2>/dev/null || true
    sleep 1

    echo "RBSp4: Note version in use"
    dnsmasq --version

    echo "RBSp4: Configuring dnsmasq"
    echo "bind-interfaces"                                         > /etc/dnsmasq.conf || exit 41
    echo "interface=eth1"                                         >> /etc/dnsmasq.conf || exit 41
    echo "listen-address=${RBN_ENCLAVE_SENTRY_IP}"                >> /etc/dnsmasq.conf || exit 41
    echo "no-dhcp-interface=eth1"                                 >> /etc/dnsmasq.conf || exit 41
    echo "cache-size=1000"                                        >> /etc/dnsmasq.conf || exit 41
    echo "min-cache-ttl=600"                                      >> /etc/dnsmasq.conf || exit 41
    echo "max-cache-ttl=3600"                                     >> /etc/dnsmasq.conf || exit 41
    echo "log-queries=extra"                                      >> /etc/dnsmasq.conf || exit 41
    echo "log-facility=/var/log/dnsmasq.log"                      >> /etc/dnsmasq.conf || exit 41
    echo "log-dhcp"                                               >> /etc/dnsmasq.conf || exit 41
    echo "log-debug"                                              >> /etc/dnsmasq.conf || exit 41
    echo "log-async=20"                                           >> /etc/dnsmasq.conf || exit 41
    if [ "${RBN_UPLINK_DNS_GLOBAL}" = "1" ]; then
        echo "RBSp4: Enabling global DNS resolution"
        echo "server=${RBB_DNS_SERVER}"                           >> /etc/dnsmasq.conf || exit 41
    else
        echo "RBSp4: Configuring domain-based DNS filtering"
        # Add domain-specific forwarding first
        for domain in ${RBN_UPLINK_ALLOWED_DOMAINS}; do
            echo "server=/${domain}/${RBB_DNS_SERVER}"            >> /etc/dnsmasq.conf || exit 41
        done
        # Block everything else with NXDOMAIN
        echo "server=/#/"                                         >> /etc/dnsmasq.conf || exit 41
    fi
    echo "RBSp4: Echo back the constructed dnsmasq config file"
    cat                                                              /etc/dnsmasq.conf || exit 41

    #  FOR NOW I'M COMMENTING THIS OUT: we seem to get zombie processes and while
    #  I'm trying to riddle the failure of bottle to see sentry first time, that
    #  is a distraction.  For now we'll manually run in its own console.
    #
    # echo "RBSp4: Process info before launch (zombie dnsmasq diagnostic)..."
    # ps aux
    # echo "RBSp4: Starting dnsmasq service"
    # dnsmasq -d
    # sleep 1
    # echo "RBSp4: Process info after launch..."
    # ps aux

    echo "RBSp4: Configuring DNS firewall rules"
    iptables -A RBM-INGRESS -i eth1 -p udp --dport 53                        -j ACCEPT || exit 43
    iptables -A RBM-INGRESS -i eth1 -p tcp --dport 53                        -j ACCEPT || exit 43
    iptables -A RBM-FORWARD -i eth1 -p udp --dport 53                        -j ACCEPT || exit 43
    iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53                        -j ACCEPT || exit 43
    iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d "${RBB_DNS_SERVER}" -j ACCEPT || exit 43
    iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d "${RBB_DNS_SERVER}" -j ACCEPT || exit 43

    echo "RBSp4: Setting up DNS NAT rules"
    iptables -t nat -A PREROUTING -i eth1 -p udp --dport 53 -j DNAT --to ${RBN_ENCLAVE_SENTRY_IP}:53
    iptables -t nat -A PREROUTING -i eth1 -p tcp --dport 53 -j DNAT --to ${RBN_ENCLAVE_SENTRY_IP}:53
fi

echo "RBSp4: Sentry setup complete"
