#!/bin/sh
echo "RBS: Beginning sentry setup script"

set -e
set -x

echo "RBSp1: Validate parameters"
: ${RBRN_ENCLAVE_BASE_IP:?}        && echo "RBSp0: RBRN_ENCLAVE_BASE_IP        = ${RBRN_ENCLAVE_BASE_IP}"
: ${RBRN_ENCLAVE_NETMASK:?}        && echo "RBSp0: RBRN_ENCLAVE_NETMASK        = ${RBRN_ENCLAVE_NETMASK}"
: ${RBRN_ENCLAVE_SENTRY_IP:?}      && echo "RBSp0: RBRN_ENCLAVE_SENTRY_IP      = ${RBRN_ENCLAVE_SENTRY_IP}"
: ${RBRN_ENCLAVE_BOTTLE_IP:?}      && echo "RBSp0: RBRN_ENCLAVE_BOTTLE_IP      = ${RBRN_ENCLAVE_BOTTLE_IP}"
: ${RBRR_DNS_SERVER:?}             && echo "RBSp0: RBRR_DNS_SERVER             = ${RBRR_DNS_SERVER}"
: ${RBRN_ENTRY_MODE:?}             && echo "RBSp0: RBRN_ENTRY_MODE             = ${RBRN_ENTRY_MODE}"
: ${RBRN_ENTRY_PORT_WORKSTATION:?} && echo "RBSp0: RBRN_ENTRY_PORT_WORKSTATION = ${RBRN_ENTRY_PORT_WORKSTATION}"
: ${RBRN_ENTRY_PORT_ENCLAVE:?}     && echo "RBSp0: RBRN_ENTRY_PORT_ENCLAVE     = ${RBRN_ENTRY_PORT_ENCLAVE}"
: ${RBRN_UPLINK_DNS_MODE:?}        && echo "RBSp0: RBRN_UPLINK_DNS_MODE        = ${RBRN_UPLINK_DNS_MODE}"
: ${RBRN_UPLINK_PORT_MIN:?}        && echo "RBSp0: RBRN_UPLINK_PORT_MIN        = ${RBRN_UPLINK_PORT_MIN}"
: ${RBRN_UPLINK_ACCESS_MODE:?}     && echo "RBSp0: RBRN_UPLINK_ACCESS_MODE     = ${RBRN_UPLINK_ACCESS_MODE}"
: ${RBRN_UPLINK_ALLOWED_CIDRS:?}   && echo "RBSp0: RBRN_UPLINK_ALLOWED_CIDRS   = ${RBRN_UPLINK_ALLOWED_CIDRS}"
: ${RBRN_UPLINK_ALLOWED_DOMAINS:?} && echo "RBSp0: RBRN_UPLINK_ALLOWED_DOMAINS = ${RBRN_UPLINK_ALLOWED_DOMAINS}"

echo "RBSp1: Beginning IPTables initialization"

echo "RBSp1: Set ephemeral port range for uplink connections"
echo "${RBRN_UPLINK_PORT_MIN} 65535" > /proc/sys/net/ipv4/ip_local_port_range || exit 10

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

echo "RBSp2: Allowing ICMP within enclave only"
iptables -A RBM-INGRESS -i eth1 -p icmp -j ACCEPT || exit 20
iptables -A RBM-EGRESS  -o eth1 -p icmp -j ACCEPT || exit 20

echo "RBSp2: Phase 2: Port Setup"
if [ "${RBRN_ENTRY_MODE}" = "enabled" ]; then
    echo "RBSp2: Configuring TCP access for bottled services"

    echo "RBSp2: Allow direct connections from sentry to bottle for the entry port"
    iptables -A RBM-EGRESS  -o eth1 -p tcp -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport ${RBRN_ENTRY_PORT_ENCLAVE} -j ACCEPT || exit 25
    iptables -A RBM-INGRESS -i eth1 -p tcp -s "${RBRN_ENCLAVE_BOTTLE_IP}" --sport ${RBRN_ENTRY_PORT_ENCLAVE} -j ACCEPT || exit 25

    echo "RBSp2: Setting up socat proxy on port ${RBRN_ENTRY_PORT_WORKSTATION} -> ${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}"
    nohup socat TCP-LISTEN:${RBRN_ENTRY_PORT_WORKSTATION},fork,reuseaddr TCP:${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE} >/var/log/socat-proxy.log 2>&1 &

    echo "RBSp2: Give socat a moment to start"
    sleep 1

    echo "RBSp2: Verify socat is running"
    if pgrep -f "socat.*:${RBRN_ENTRY_PORT_WORKSTATION}" >/dev/null; then
        echo "RBSp2: Socat proxy started successfully"
    else
        echo "RBSp2: ERROR - Socat proxy failed to start"
        cat /var/log/socat-proxy.log
        exit 26
    fi

    echo "RBSp2: Allow incoming connections from bridge network (eth0) on entry port"
    iptables -A RBM-INGRESS -i eth0 -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} -j ACCEPT || exit 27
fi

echo "RBSp2b: Blocking ICMP cross-boundary traffic"
iptables -A RBM-FORWARD         -p icmp -j DROP || exit 28
iptables -A RBM-EGRESS  -o eth0 -p icmp -j DROP || exit 28

echo "RBSp3: Phase 3: Access Setup"
if [ "${RBRN_UPLINK_ACCESS_MODE}" = "disabled" ]; then
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
    iptables -t nat -A POSTROUTING -o eth0 -s "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
                                         ! -d "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
                                         -j MASQUERADE || exit 31

    if [ "${RBRN_UPLINK_ACCESS_MODE}" = "global" ]; then
        echo "RBSp3: Enabling global access"
        iptables -A RBM-EGRESS  -o eth0 -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -j ACCEPT || exit 31
    else
        echo "RBSp3: Configuring DNS server access"
        iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d "${RBRR_DNS_SERVER}"        -j ACCEPT || exit 31
        iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d "${RBRR_DNS_SERVER}"        -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -d "${RBRN_ENCLAVE_SENTRY_IP}" -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -d "${RBRN_ENCLAVE_SENTRY_IP}" -j ACCEPT || exit 31
        iptables -A RBM-FORWARD -i eth1 -p udp --dport 53                                -j DROP   || exit 31
        iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53                                -j DROP   || exit 31

        echo "RBSp3: Setting up CIDR-based access control"
        for cidr in ${RBRN_UPLINK_ALLOWED_CIDRS}; do
            iptables -A RBM-EGRESS  -o eth0 -d "${cidr}" -j ACCEPT || exit 32
            iptables -A RBM-FORWARD -i eth1 -d "${cidr}" -j ACCEPT || exit 32
        done
    fi
fi

echo "RBSp4: Configuring DNS services"

echo "RBSp4: Configuring sentry DNS resolution"
echo "nameserver ${RBRR_DNS_SERVER}" > /etc/resolv.conf   || exit 40

if [ "${RBRN_UPLINK_DNS_MODE}" = "disabled" ]; then
    echo "RBSp4: Blocking all DNS traffic"
    iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -j DROP || exit 40
    iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -j DROP || exit 40
    iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -j DROP || exit 40
    iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -j DROP || exit 40
else
    echo "RBSp4: Set up DNS Server"

    echo "RBSp4: Process cleanup"
    killall -9 dnsmasq 2>/dev/null || true
    sleep 1

    echo "RBSp4: Note version in use"
    dnsmasq --version

    echo "RBSp4: Configuring dnsmasq"
    echo "bind-interfaces"                                         > /etc/dnsmasq.conf || exit 41
    echo "interface=eth1"                                         >> /etc/dnsmasq.conf || exit 41
    echo "listen-address=${RBRN_ENCLAVE_SENTRY_IP}"               >> /etc/dnsmasq.conf || exit 41
    echo "no-dhcp-interface=eth1"                                 >> /etc/dnsmasq.conf || exit 41
    echo "dns-forward-max=150"                                    >> /etc/dnsmasq.conf || exit 41
    echo "cache-size=1000"                                        >> /etc/dnsmasq.conf || exit 41
    echo "min-port=4096"                                          >> /etc/dnsmasq.conf || exit 41
    echo "max-port=65535"                                         >> /etc/dnsmasq.conf || exit 41
    echo "min-cache-ttl=600"                                      >> /etc/dnsmasq.conf || exit 41
    echo "max-cache-ttl=3600"                                     >> /etc/dnsmasq.conf || exit 41
    echo "no-resolv"                                              >> /etc/dnsmasq.conf || exit 41
    echo "strict-order"                                           >> /etc/dnsmasq.conf || exit 41
    echo "bogus-priv"                                             >> /etc/dnsmasq.conf || exit 41
    echo "domain-needed"                                          >> /etc/dnsmasq.conf || exit 41
    echo "except-interface=eth0"                                  >> /etc/dnsmasq.conf || exit 41
    echo "log-queries=extra"                                      >> /etc/dnsmasq.conf || exit 41
    echo "log-facility=/var/log/dnsmasq.log"                      >> /etc/dnsmasq.conf || exit 41
    echo "log-dhcp"                                               >> /etc/dnsmasq.conf || exit 41
    echo "log-debug"                                              >> /etc/dnsmasq.conf || exit 41
    echo "log-async=20"                                           >> /etc/dnsmasq.conf || exit 41
    if [ "${RBRN_UPLINK_DNS_MODE}" = "global" ]; then
        echo "RBSp4: Enabling global DNS resolution"
        echo "server=${RBRR_DNS_SERVER}"                          >> /etc/dnsmasq.conf || exit 41
    else
        echo "RBSp4: Configuring domain-based DNS filtering"
        # Add domain-specific forwarding first
        for domain in ${RBRN_UPLINK_ALLOWED_DOMAINS}; do
            echo "server=/${domain}/${RBRR_DNS_SERVER}"           >> /etc/dnsmasq.conf || exit 41
        done
        # Block everything else with NXDOMAIN
        echo "address=/#/"                                        >> /etc/dnsmasq.conf || exit 41
    fi

    echo "RBSp4: Echo back the constructed dnsmasq config file"
    cat                                                              /etc/dnsmasq.conf || exit 41

    echo "RBSp4: Process info before launch (zombie dnsmasq diagnostic)..."
    ps aux
    echo "RBSp4: Starting dnsmasq service"
    dnsmasq
    echo "RBSp4: Mystery dnsmasq setup delay"
    sleep 2
    echo "RBSp4: Process info after launch..."
    ps aux

    echo "RBSp4: Configuring DNS firewall rules"
    iptables -A RBM-INGRESS -i eth1 -p udp --dport 53                         -j ACCEPT || exit 43
    iptables -A RBM-INGRESS -i eth1 -p tcp --dport 53                         -j ACCEPT || exit 43
    iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d "${RBRR_DNS_SERVER}" -j ACCEPT || exit 43
    iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d "${RBRR_DNS_SERVER}" -j ACCEPT || exit 43
fi

echo "RBSp4: Sentry setup complete"

