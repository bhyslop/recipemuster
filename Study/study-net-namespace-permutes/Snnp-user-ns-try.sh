#!/bin/bash

# Simplified Container Network Setup Script
# Each step is executed discretely with minimal environment passing

set -e  # Exit on error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "SNNP: Get constants from" ${SCRIPT_DIR}
source "$SCRIPT_DIR/Snnp-constants.sh"

function snnp_podman_exec_sentry() {
    podman -c ${MACHINE} exec ${SENTRY_CONTAINER} "$@"
}

function snnp_podman_exec_bottle() {
    podman -c ${MACHINE} exec ${BOTTLE_CONTAINER} "$@"
}

function snnp_machine_ssh() {
    podman machine ssh ${MACHINE} "$@"
}

function snnp_machine_ssh_sudo() {
    podman machine ssh ${MACHINE} sudo "$@"
}


echo -e "${BOLD}Container Network Setup Script${NC}"
echo "Setting up ${MONIKER} containers with network isolation"
echo ""

echo -e "${BOLD}Checking connection to ${MACHINE}${NC}"
podman -c ${MACHINE} info > /dev/null || { echo "Unable to connect to machine"; exit 1; }
echo -e "${GREEN}${BOLD}Connection successful.${NC}"

echo -e "${BOLD}Stopping any prior containers${NC}"
podman -c ${MACHINE} stop -t 2 ${SENTRY_CONTAINER} || true
podman -c ${MACHINE} rm -f     ${SENTRY_CONTAINER} || true
podman -c ${MACHINE} stop -t 2 ${BOTTLE_CONTAINER} || true
podman -c ${MACHINE} rm -f     ${BOTTLE_CONTAINER} || true

echo -e "${BOLD}Cleaning up old netns and interfaces inside VM${NC}"
echo "RBNC: Beginning network cleanup script"
echo "RBNC0: Cleaning up SENTRY interfaces: ${ENCLAVE_SENTRY_OUT}, ${ENCLAVE_SENTRY_IN}"
echo "RBNC1: Cleaning up BOTTLE interfaces: ${ENCLAVE_BOTTLE_OUT}, ${ENCLAVE_BOTTLE_IN}"
echo "RBNC2: Removing bridge: ${ENCLAVE_BRIDGE}"

snnp_machine_ssh_sudo ip link  del    ${ENCLAVE_SENTRY_OUT} || echo "Deleted" ${ENCLAVE_SENTRY_OUT}
snnp_machine_ssh_sudo ip link  del    ${ENCLAVE_SENTRY_IN}  || echo "Deleted" ${ENCLAVE_SENTRY_IN} 
snnp_machine_ssh_sudo ip link  del    ${ENCLAVE_BOTTLE_OUT} || echo "Deleted" ${ENCLAVE_BOTTLE_OUT}
snnp_machine_ssh_sudo ip link  del    ${ENCLAVE_BOTTLE_IN}  || echo "Deleted" ${ENCLAVE_BOTTLE_IN} 
snnp_machine_ssh_sudo ip link  del    ${ENCLAVE_BRIDGE}     || echo "Deleted" ${ENCLAVE_BRIDGE}    
snnp_machine_ssh_sudo ip netns delete ${NET_NAMESPACE}      || echo "Deleted" ${NET_NAMESPACE}     

echo "RBNC3: Verifying cleanup"
snnp_machine_ssh "ip link show | grep -E '${ENCLAVE_SENTRY_OUT}|${ENCLAVE_BOTTLE_OUT}|${ENCLAVE_BRIDGE}' || echo 'No matching interfaces found'"
echo "RBNC: Network cleanup complete"

echo -e "${BOLD}Launching SENTRY container with bridging for internet${NC}"
podman -c ${MACHINE} run -d                              \
  --name ${SENTRY_CONTAINER}                             \
  --network bridge                                       \
  --privileged                                           \
  -p ${ENTRY_PORT_WORKSTATION}:${ENTRY_PORT_WORKSTATION} \
  ${SENTRY_REPO_PATH}:${SENTRY_IMAGE_TAG}

echo -e "${BOLD}Waiting for SENTRY container${NC}"
sleep 2
snnp_machine_ssh "podman ps | grep ${SENTRY_CONTAINER} || (echo 'Container not running' && exit 1)"

echo -e "${BOLD}Executing SENTRY namespace setup script${NC}"
echo "RSNS: Beginning sentry namespace setup"
echo "RSNS0: Getting SENTRY PID"

SENTRY_PID=$(podman -c ${MACHINE} inspect -f '{{.State.Pid}}' ${SENTRY_CONTAINER})
echo "RSNS1: SENTRY PID: ${SENTRY_PID}"

echo "RSNS2: Creating and configuring bridge"
snnp_machine_ssh_sudo ip link add name ${ENCLAVE_BRIDGE} type bridge
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BRIDGE} up

echo "RSNS3: Creating and configuring SENTRY veth pair"
snnp_machine_ssh_sudo ip link add ${ENCLAVE_SENTRY_OUT} type veth peer name ${ENCLAVE_SENTRY_IN}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_SENTRY_IN} netns ${SENTRY_PID}
snnp_machine_ssh_sudo nsenter -t ${SENTRY_PID} -n ip link set ${ENCLAVE_SENTRY_IN} name eth1
snnp_machine_ssh_sudo nsenter -t ${SENTRY_PID} -n ip addr add ${ENCLAVE_SENTRY_IP}/${ENCLAVE_NETMASK} dev eth1
snnp_machine_ssh_sudo nsenter -t ${SENTRY_PID} -n ip link set eth1 up

echo "RSNS4: Connecting SENTRY veth to bridge"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_SENTRY_OUT} master ${ENCLAVE_BRIDGE}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_SENTRY_OUT} up
echo "RSNS: Sentry namespace setup complete"

echo -e "${BOLD}Configuring SENTRY security${NC}"
echo "RBS: Beginning sentry setup script"

echo "RBS1: Beginning IPTables initialization"
snnp_podman_exec_sentry sh -c "echo 10000 65535 > /proc/sys/net/ipv4/ip_local_port_range"
snnp_podman_exec_sentry iptables -F
snnp_podman_exec_sentry iptables -t nat -F

echo "RBS2: Setting default policies"
snnp_podman_exec_sentry iptables -P INPUT   DROP
snnp_podman_exec_sentry iptables -P FORWARD DROP
snnp_podman_exec_sentry iptables -P OUTPUT  DROP

echo "RBS3: Configuring loopback access"
snnp_podman_exec_sentry iptables -A INPUT  -i lo -j ACCEPT
snnp_podman_exec_sentry iptables -A OUTPUT -o lo -j ACCEPT

echo "RBS4: Setting up connection tracking"
snnp_podman_exec_sentry iptables -A INPUT   -m state --state RELATED,ESTABLISHED -j ACCEPT
snnp_podman_exec_sentry iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT
snnp_podman_exec_sentry iptables -A OUTPUT  -m state --state RELATED,ESTABLISHED -j ACCEPT

echo "RBS5: Creating RBM chains"
snnp_podman_exec_sentry iptables -N RBM-INGRESS
snnp_podman_exec_sentry iptables -N RBM-EGRESS
snnp_podman_exec_sentry iptables -N RBM-FORWARD

echo "RBS6: Setting up chain jumps"
snnp_podman_exec_sentry iptables -A INPUT   -j RBM-INGRESS
snnp_podman_exec_sentry iptables -A OUTPUT  -j RBM-EGRESS
snnp_podman_exec_sentry iptables -A FORWARD -j RBM-FORWARD

echo "RBS7: Allowing ICMP within enclave only"
snnp_podman_exec_sentry iptables -A RBM-INGRESS -i eth1 -p icmp -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth1 -p icmp -j ACCEPT

echo "RBS8: Configuring TCP access for bottled services"
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth1 -p tcp -d ${ENCLAVE_BOTTLE_IP} --dport ${ENTRY_PORT_ENCLAVE} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-INGRESS -i eth1 -p tcp -s ${ENCLAVE_BOTTLE_IP} --sport ${ENTRY_PORT_ENCLAVE} -j ACCEPT

echo "RBS9: Setting up socat proxy"
podman -c ${MACHINE} exec -d ${SENTRY_CONTAINER} socat TCP-LISTEN:${ENTRY_PORT_WORKSTATION},fork,reuseaddr TCP:${ENCLAVE_BOTTLE_IP}:${ENTRY_PORT_ENCLAVE}
sleep 1
snnp_podman_exec_sentry pgrep -f "socat.*:${ENTRY_PORT_WORKSTATION}" || echo "Socat proxy not started"

echo "RBS10: Blocking ICMP cross-boundary traffic"
snnp_podman_exec_sentry iptables -A RBM-FORWARD         -p icmp -j DROP
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth0 -p icmp -j DROP

echo "RBS11: Setting up network forwarding"
snnp_podman_exec_sentry sh -c "echo 1 > /proc/sys/net/ipv4/ip_forward"
snnp_podman_exec_sentry sh -c "echo 0 > /proc/sys/net/ipv4/conf/default/rp_filter"
snnp_podman_exec_sentry sh -c "echo 1 > /proc/sys/net/ipv4/conf/all/forwarding"
snnp_podman_exec_sentry sh -c "echo 1 > /proc/sys/net/ipv4/conf/eth0/forwarding"

echo "RBS12: Configuring NAT"
snnp_podman_exec_sentry iptables -t nat -A POSTROUTING -o eth0 -s ${ENCLAVE_BASE_IP}/${ENCLAVE_NETMASK} ! -d ${ENCLAVE_BASE_IP}/${ENCLAVE_NETMASK} -j MASQUERADE

echo "RBS13: Configuring DNS server access"
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d ${DNS_SERVER} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d ${DNS_SERVER} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -d ${ENCLAVE_SENTRY_IP} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -d ${ENCLAVE_SENTRY_IP} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -j DROP
snnp_podman_exec_sentry iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -j DROP

echo "RBS14: Setting up CIDR-based access control"
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth0 -d ${UPLINK_ALLOWED_CIDRS} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-FORWARD -i eth1 -d ${UPLINK_ALLOWED_CIDRS} -j ACCEPT

echo "RBS15: Configuring sentry DNS resolution"
snnp_podman_exec_sentry sh -c "echo 'nameserver ${DNS_SERVER}' > /etc/resolv.conf"

echo "RBS16: Setting up DNS Server"
snnp_podman_exec_sentry killall -9 dnsmasq 2>/dev/null || true
sleep 1

echo "RBS17: Configuring dnsmasq"
snnp_podman_exec_sentry sh -c "cat > /etc/dnsmasq.conf << EOC
bind-interfaces
interface=eth1
listen-address=${ENCLAVE_SENTRY_IP}
no-dhcp-interface=eth1
dns-forward-max=150
cache-size=1000
min-port=4096
max-port=65535
min-cache-ttl=600
max-cache-ttl=3600
no-resolv
strict-order
bogus-priv
domain-needed
except-interface=eth0
log-queries=extra
log-facility=/var/log/dnsmasq.log
log-dhcp
log-debug
log-async=20
server=/${UPLINK_ALLOWED_DOMAINS}/${DNS_SERVER}
address=/#/
EOC"

echo "RBS18: Starting dnsmasq service"
podman -c ${MACHINE} exec -d ${SENTRY_CONTAINER} dnsmasq
sleep 2

echo "RBS19: Configuring DNS firewall rules"
snnp_podman_exec_sentry iptables -A RBM-INGRESS -i eth1 -p udp --dport 53 -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-INGRESS -i eth1 -p tcp --dport 53 -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d ${DNS_SERVER} -j ACCEPT
snnp_podman_exec_sentry iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d ${DNS_SERVER} -j ACCEPT
echo "RBS: Sentry setup complete"

echo "RBNS-ALT: Creating network namespace manually"
snnp_machine_ssh_sudo ip netns add ${NET_NAMESPACE}

echo "RBNS-ALT: Creating veth pair"
snnp_machine_ssh_sudo ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}

echo "RBNS-ALT: Moving veth endpoint to namespace"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_IN} netns ${NET_NAMESPACE}

echo "RBNS-ALT: Configuring interfaces in namespace"
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip link set ${ENCLAVE_BOTTLE_IN} name eth1
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip link set eth1 up
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip link set lo up

echo "RBNS-ALT: Connecting veth to bridge"
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
snnp_machine_ssh_sudo ip link set ${ENCLAVE_BOTTLE_OUT} up

echo "RBNS-ALT: Setting default route in namespace"
snnp_machine_ssh_sudo ip netns exec ${NET_NAMESPACE} ip route add default via ${ENCLAVE_SENTRY_IP}

echo "RBNS-ALT: Starting container with the prepared network namespace"
snnp_machine_ssh podman run -d                    \
    --name ${BOTTLE_CONTAINER}                    \
    --network ns:/var/run/netns/${NET_NAMESPACE}  \
    ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}

echo -e "${BOLD}Visualizing network setup in podman machine...${NC}"
echo "RBNI: Network interface information"
snnp_machine_ssh ip a

echo "RBNI: Network bridge information" 
snnp_machine_ssh ip link show type bridge

echo "RBNI: Network namespace information"
snnp_machine_ssh ip netns list

echo "RBNI: Route information"
snnp_machine_ssh ip route

echo -e "${BOLD}Verifying containers${NC}"
podman -c ${MACHINE} ps -a

echo -e "${GREEN}${BOLD}Setup script execution complete${NC}"

