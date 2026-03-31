#!/bin/sh
echo "RBJ: Beginning sentry setup script"

set -e
test "${RBJ_VERBOSE:-0}" -ge 1 && set -x

echo "RBJp0: Validate compose env-file quoting"
: "${RBJE_PROBE:?}"
z_probe_count=0
for z_word in ${RBJE_PROBE}; do
  z_probe_count=$((z_probe_count + 1))
done
test "${z_probe_count}" -eq 2                || { echo "FATAL: RBJE_PROBE token count ${z_probe_count}, expected 2 — value: '${RBJE_PROBE:-}'"; exit 1; }
test "$(echo ${RBJE_PROBE} | cut -d' ' -f1)" = "alpha" || { echo "FATAL: RBJE_PROBE first token not 'alpha' — value: '${RBJE_PROBE:-}'"; exit 1; }
test "$(echo ${RBJE_PROBE} | cut -d' ' -f2)" = "bravo" || { echo "FATAL: RBJE_PROBE second token not 'bravo' — value: '${RBJE_PROBE:-}'"; exit 1; }
echo "RBJp0: Compose env-file quoting validated — RBJE_PROBE = '${RBJE_PROBE}'"

echo "RBJp1: Validate parameters"
: "${RBRN_ENCLAVE_BASE_IP:?}"        && echo "RBJp0: RBRN_ENCLAVE_BASE_IP        = ${RBRN_ENCLAVE_BASE_IP}"
: "${RBRN_ENCLAVE_NETMASK:?}"        && echo "RBJp0: RBRN_ENCLAVE_NETMASK        = ${RBRN_ENCLAVE_NETMASK}"
: "${RBRN_ENCLAVE_SENTRY_IP:?}"      && echo "RBJp0: RBRN_ENCLAVE_SENTRY_IP      = ${RBRN_ENCLAVE_SENTRY_IP}"
: "${RBRN_ENCLAVE_BOTTLE_IP:?}"      && echo "RBJp0: RBRN_ENCLAVE_BOTTLE_IP      = ${RBRN_ENCLAVE_BOTTLE_IP}"
: "${RBRR_DNS_SERVER:?}"             && echo "RBJp0: RBRR_DNS_SERVER             = ${RBRR_DNS_SERVER}"
: "${RBRN_ENTRY_MODE:?}"             && echo "RBJp0: RBRN_ENTRY_MODE             = ${RBRN_ENTRY_MODE}"
: "${RBRN_ENTRY_PORT_WORKSTATION:?}" && echo "RBJp0: RBRN_ENTRY_PORT_WORKSTATION = ${RBRN_ENTRY_PORT_WORKSTATION}"
: "${RBRN_ENTRY_PORT_ENCLAVE:?}"     && echo "RBJp0: RBRN_ENTRY_PORT_ENCLAVE     = ${RBRN_ENTRY_PORT_ENCLAVE}"
: "${RBRN_UPLINK_DNS_MODE:?}"        && echo "RBJp0: RBRN_UPLINK_DNS_MODE        = ${RBRN_UPLINK_DNS_MODE}"
: "${RBRN_UPLINK_PORT_MIN:?}"        && echo "RBJp0: RBRN_UPLINK_PORT_MIN        = ${RBRN_UPLINK_PORT_MIN}"
: "${RBRN_UPLINK_ACCESS_MODE:?}"     && echo "RBJp0: RBRN_UPLINK_ACCESS_MODE     = ${RBRN_UPLINK_ACCESS_MODE}"
: "${RBRN_UPLINK_ALLOWED_CIDRS:?}"   && echo "RBJp0: RBRN_UPLINK_ALLOWED_CIDRS   = ${RBRN_UPLINK_ALLOWED_CIDRS}"
: "${RBRN_UPLINK_ALLOWED_DOMAINS:?}" && echo "RBJp0: RBRN_UPLINK_ALLOWED_DOMAINS = ${RBRN_UPLINK_ALLOWED_DOMAINS}"

echo "RBJp1: Discovering network interfaces by IP (Docker does not guarantee eth0/eth1 ordering)"
z_temp_file="/tmp/rbj_iface_discovery.txt"

ip -o addr show to "${RBRN_ENCLAVE_SENTRY_IP}" > "${z_temp_file}" || exit 11
read z_num RBJ_ENCLAVE_IF z_rest < "${z_temp_file}"
test -n "${RBJ_ENCLAVE_IF}" || { echo "FATAL: No interface found with IP ${RBRN_ENCLAVE_SENTRY_IP}"; exit 11; }

ip -o -4 addr show scope global > "${z_temp_file}" || exit 11
RBJ_UPLINK_IF=""
while read z_num z_ifname z_rest; do
  test "${z_ifname}" = "${RBJ_ENCLAVE_IF}" && continue
  RBJ_UPLINK_IF="${z_ifname}"
  break
done < "${z_temp_file}"
test -n "${RBJ_UPLINK_IF}" || { echo "FATAL: No uplink interface found"; exit 11; }

rm -f "${z_temp_file}"
echo "RBJp1: Enclave interface = ${RBJ_ENCLAVE_IF}"
echo "RBJp1: Uplink interface  = ${RBJ_UPLINK_IF}"

echo "RBJp1: Beginning IPTables initialization"

echo "RBJp1: Set ephemeral port range for uplink connections"
echo "${RBRN_UPLINK_PORT_MIN} 65535" > /proc/sys/net/ipv4/ip_local_port_range || exit 10

echo "RBJp1: Flushing existing rules"
iptables -F        || exit 10
iptables -t nat -F || exit 10

echo "RBJp1: Setting default policies"
iptables -P INPUT   DROP || exit 10
iptables -P FORWARD DROP || exit 10
iptables -P OUTPUT  DROP || exit 10

echo "RBJp1: Configuring loopback access"
iptables -A INPUT  -i lo -j ACCEPT || exit 10
iptables -A OUTPUT -o lo -j ACCEPT || exit 10

echo "RBJp1: Setting up connection tracking"
iptables -A INPUT   -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -A OUTPUT  -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10

echo "RBJp1: Creating RBM chains"
iptables -N RBM-INGRESS || exit 10
iptables -N RBM-EGRESS  || exit 10
iptables -N RBM-FORWARD || exit 10

echo "RBJp1: Setting up chain jumps"
iptables -A INPUT   -j RBM-INGRESS || exit 10
iptables -A OUTPUT  -j RBM-EGRESS  || exit 10
iptables -A FORWARD -j RBM-FORWARD || exit 10

echo "RBJp2: Allowing ICMP within enclave only"
iptables -A RBM-INGRESS -i ${RBJ_ENCLAVE_IF} -p icmp -j ACCEPT || exit 20
iptables -A RBM-EGRESS  -o ${RBJ_ENCLAVE_IF} -p icmp -j ACCEPT || exit 20

echo "RBJp2: Phase 2: Port Setup"
if test "${RBRN_ENTRY_MODE}" = "enabled"; then
  echo "RBJp2: Configuring entry port via iptables DNAT (no userspace proxy)"

  echo "RBJp2: Enabling IP forwarding for entry port DNAT"
  echo 1 > /proc/sys/net/ipv4/ip_forward || exit 25

  echo "RBJp2: DNAT incoming connections on entry port to bottle"
  iptables -t nat -A PREROUTING -i ${RBJ_UPLINK_IF} -p tcp --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
           -j DNAT --to-destination "${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}" || exit 25

  echo "RBJp2: Allow forwarding of DNATted entry traffic to bottle"
  iptables -A RBM-FORWARD -i ${RBJ_UPLINK_IF} -o ${RBJ_ENCLAVE_IF} -p tcp \
           -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" -j ACCEPT || exit 25

  echo "RBJp2: MASQUERADE return traffic so bottle replies to sentry"
  iptables -t nat -A POSTROUTING -o ${RBJ_ENCLAVE_IF} -p tcp \
           -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" -j MASQUERADE || exit 25
fi

echo "RBJp2b: Blocking ICMP cross-boundary traffic"
iptables -A RBM-FORWARD         -p icmp -j DROP || exit 28
iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p icmp -j DROP || exit 28

echo "RBJp3: Phase 3: Access Setup"
if test "${RBRN_UPLINK_ACCESS_MODE}" = "disabled"; then
  echo "RBJp3: Blocking all non-port traffic"
  iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -j DROP || exit 30
  iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -j DROP || exit 30
else
  echo "RBJp3: Setting up network forwarding"
  echo 1 > /proc/sys/net/ipv4/ip_forward               || exit 31
  echo 0 > /proc/sys/net/ipv6/conf/all/disable_ipv6    || exit 31
  echo 1 > /proc/sys/net/ipv4/conf/all/rp_filter       || exit 31
  echo 1 > /proc/sys/net/ipv4/conf/${RBJ_UPLINK_IF}/route_localnet || exit 31

  echo "RBJp3: Configuring NAT"
  iptables -t nat -A POSTROUTING -o ${RBJ_UPLINK_IF} -s "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
                                       ! -d "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
                                       -j MASQUERADE || exit 31

  if test "${RBRN_UPLINK_ACCESS_MODE}" = "global"; then
    echo "RBJp3: Enabling global access"
    iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -j ACCEPT || exit 31
    iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -j ACCEPT || exit 31
  else
    echo "RBJp3: Configuring DNS server access"
    iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p udp --dport 53 -d "${RBRR_DNS_SERVER}"        -j ACCEPT || exit 31
    iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p tcp --dport 53 -d "${RBRR_DNS_SERVER}"        -j ACCEPT || exit 31
    iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -p udp --dport 53 -d "${RBRN_ENCLAVE_SENTRY_IP}" -j ACCEPT || exit 31
    iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -p tcp --dport 53 -d "${RBRN_ENCLAVE_SENTRY_IP}" -j ACCEPT || exit 31
    iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -p udp --dport 53                                -j DROP   || exit 31
    iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -p tcp --dport 53                                -j DROP   || exit 31

    echo "RBJp3: Setting up CIDR-based access control"
    for cidr in ${RBRN_UPLINK_ALLOWED_CIDRS}; do
      iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -d "${cidr}" -j ACCEPT || exit 32
      iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -d "${cidr}" -j ACCEPT || exit 32
    done
  fi
fi

echo "RBJp4: Configuring DNS services"

echo "RBJp4: Configuring sentry DNS resolution"
echo "nameserver ${RBRR_DNS_SERVER}" > /etc/resolv.conf   || exit 40

if test "${RBRN_UPLINK_DNS_MODE}" = "disabled"; then
  echo "RBJp4: Blocking all DNS traffic"
  iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -p udp --dport 53 -j DROP || exit 40
  iptables -A RBM-FORWARD -i ${RBJ_ENCLAVE_IF} -p tcp --dport 53 -j DROP || exit 40
  iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p udp --dport 53 -j DROP || exit 40
  iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p tcp --dport 53 -j DROP || exit 40
else
  echo "RBJp4: Set up DNS Server"

  echo "RBJp4: Note version in use"
  dnsmasq --version

  echo "RBJp4: Configuring dnsmasq"
  echo "bind-interfaces"                                         > /etc/dnsmasq.conf || exit 41
  echo "interface=${RBJ_ENCLAVE_IF}"                             >> /etc/dnsmasq.conf || exit 41
  echo "listen-address=${RBRN_ENCLAVE_SENTRY_IP}"               >> /etc/dnsmasq.conf || exit 41
  echo "no-dhcp-interface=${RBJ_ENCLAVE_IF}"                     >> /etc/dnsmasq.conf || exit 41
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
  echo "except-interface=${RBJ_UPLINK_IF}"                       >> /etc/dnsmasq.conf || exit 41
  echo "log-queries=extra"                                      >> /etc/dnsmasq.conf || exit 41
  echo "log-facility=/var/log/dnsmasq.log"                      >> /etc/dnsmasq.conf || exit 41
  echo "log-dhcp"                                               >> /etc/dnsmasq.conf || exit 41
  echo "log-debug"                                              >> /etc/dnsmasq.conf || exit 41
  echo "log-async=20"                                           >> /etc/dnsmasq.conf || exit 41
  if test "${RBRN_UPLINK_DNS_MODE}" = "global"; then
    echo "RBJp4: Enabling global DNS resolution"
    echo "server=${RBRR_DNS_SERVER}"                          >> /etc/dnsmasq.conf || exit 41
  else
    echo "RBJp4: Configuring domain-based DNS filtering"
    # Add domain-specific forwarding first
    for domain in ${RBRN_UPLINK_ALLOWED_DOMAINS}; do
      echo "server=/${domain}/${RBRR_DNS_SERVER}"           >> /etc/dnsmasq.conf || exit 41
    done
    # Block everything else with NXDOMAIN
    echo "address=/#/"                                        >> /etc/dnsmasq.conf || exit 41
  fi

  echo "RBJp4: Echo back the constructed dnsmasq config file"
  cat                                                              /etc/dnsmasq.conf || exit 41

  echo "RBJp4: Starting dnsmasq service"
  dnsmasq
  echo "RBJp4: Waiting for dnsmasq to initialize"
  sleep 2

  echo "RBJp4: Configuring DNS firewall rules"
  iptables -A RBM-INGRESS -i ${RBJ_ENCLAVE_IF} -p udp --dport 53                         -j ACCEPT || exit 43
  iptables -A RBM-INGRESS -i ${RBJ_ENCLAVE_IF} -p tcp --dport 53                         -j ACCEPT || exit 43
  iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p udp --dport 53 -d "${RBRR_DNS_SERVER}" -j ACCEPT || exit 43
  iptables -A RBM-EGRESS  -o ${RBJ_UPLINK_IF} -p tcp --dport 53 -d "${RBRR_DNS_SERVER}" -j ACCEPT || exit 43
fi

echo "RBJp5: Signaling health"
touch /tmp/rbj_healthy || exit 50

echo "RBJp5: Sentry setup complete, entering hold"
exec sleep infinity

