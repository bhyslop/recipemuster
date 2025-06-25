#!/bin/bash

# Simplified Container Network Setup Script
# Each step is executed discretely with minimal environment passing

set -e  # Exit on error

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "SNNP: Get constants from" ${SCRIPT_DIR}
source "$SCRIPT_DIR/Snnp-constants.sh"

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

podman machine ssh ${MACHINE} sudo ip link del ${ENCLAVE_SENTRY_OUT} || true
podman machine ssh ${MACHINE} sudo ip link del ${ENCLAVE_SENTRY_IN} || true

podman machine ssh ${MACHINE} sudo ip link del ${ENCLAVE_BOTTLE_OUT} || true
podman machine ssh ${MACHINE} sudo ip link del ${ENCLAVE_BOTTLE_IN} || true

podman machine ssh ${MACHINE} sudo ip link del ${ENCLAVE_BRIDGE} || true

echo "RBNC3: Verifying cleanup"
podman machine ssh ${MACHINE} "ip link show | grep -E '${ENCLAVE_SENTRY_OUT}|${ENCLAVE_BOTTLE_OUT}|${ENCLAVE_BRIDGE}' || echo 'No matching interfaces found'"
echo "RBNC: Network cleanup complete"

echo -e "${BOLD}Launching SENTRY container with bridging for internet${NC}"
podman -c ${MACHINE} run -d \
  --name ${SENTRY_CONTAINER} \
  --network bridge \
  --privileged \
  -p ${ENTRY_PORT_WORKSTATION}:${ENTRY_PORT_WORKSTATION} \
  -e "RBRN_MONIKER=${MONIKER}" \
  -e "RBRN_ENTRY_ENABLED=1" \
  -e "RBRN_ENTRY_PORT_WORKSTATION=${ENTRY_PORT_WORKSTATION}" \
  -e "RBRN_ENTRY_PORT_ENCLAVE=${ENTRY_PORT_ENCLAVE}" \
  -e "RBRN_UPLINK_PORT_MIN=10000" \
  -e "RBRN_UPLINK_DNS_ENABLED=1" \
  -e "RBRN_UPLINK_ACCESS_ENABLED=1" \
  -e "RBRN_UPLINK_DNS_GLOBAL=0" \
  -e "RBRN_UPLINK_ACCESS_GLOBAL=0" \
  -e "RBRN_UPLINK_ALLOWED_CIDRS=${UPLINK_ALLOWED_CIDRS}" \
  -e "RBRN_UPLINK_ALLOWED_DOMAINS=${UPLINK_ALLOWED_DOMAINS}" \
  -e "RBRR_DNS_SERVER=${DNS_SERVER}" \
  ${SENTRY_REPO_PATH}:${SENTRY_IMAGE_TAG}

echo -e "${BOLD}Waiting for SENTRY container${NC}"
sleep 2
podman -c ${MACHINE} ps | grep ${SENTRY_CONTAINER} || (echo 'Container not running' && exit 1)

echo -e "${BOLD}Executing SENTRY namespace setup script${NC}"
echo "RSNS: Beginning sentry namespace setup"
echo "RSNS0: Getting SENTRY PID"

SENTRY_PID=$(podman -c ${MACHINE} inspect -f '{{.State.Pid}}' ${SENTRY_CONTAINER})
echo "RSNS1: SENTRY PID: ${SENTRY_PID}"

echo "RSNS2: Creating and configuring bridge"
podman machine ssh ${MACHINE} sudo ip link add name ${ENCLAVE_BRIDGE} type bridge
podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BRIDGE} up

echo "RSNS3: Creating and configuring SENTRY veth pair"
podman machine ssh ${MACHINE} sudo ip link add ${ENCLAVE_SENTRY_OUT} type veth peer name ${ENCLAVE_SENTRY_IN}
podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_SENTRY_IN} netns ${SENTRY_PID}
podman machine ssh ${MACHINE} sudo nsenter -t ${SENTRY_PID} -n ip link set ${ENCLAVE_SENTRY_IN} name eth1
podman machine ssh ${MACHINE} sudo nsenter -t ${SENTRY_PID} -n ip addr add ${ENCLAVE_SENTRY_IP}/${ENCLAVE_NETMASK} dev eth1
podman machine ssh ${MACHINE} sudo nsenter -t ${SENTRY_PID} -n ip link set eth1 up

echo "RSNS4: Connecting SENTRY veth to bridge"
podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_SENTRY_OUT} master ${ENCLAVE_BRIDGE}
podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_SENTRY_OUT} up
echo "RSNS: Sentry namespace setup complete"

echo -e "${BOLD}Configuring SENTRY security${NC}"
echo "RBS: Beginning sentry setup script"

echo "RBS1: Beginning IPTables initialization"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "echo 10000 65535 > /proc/sys/net/ipv4/ip_local_port_range"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -F
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -t nat -F

echo "RBS2: Setting default policies"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -P INPUT DROP
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -P FORWARD DROP
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -P OUTPUT DROP

echo "RBS3: Configuring loopback access"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A INPUT -i lo -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A OUTPUT -o lo -j ACCEPT

echo "RBS4: Setting up connection tracking"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A INPUT -m state --state RELATED,ESTABLISHED -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A OUTPUT -m state --state RELATED,ESTABLISHED -j ACCEPT

echo "RBS5: Creating RBM chains"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -N RBM-INGRESS
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -N RBM-EGRESS
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -N RBM-FORWARD

echo "RBS6: Setting up chain jumps"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A INPUT -j RBM-INGRESS
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A OUTPUT -j RBM-EGRESS
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A FORWARD -j RBM-FORWARD

echo "RBS7: Allowing ICMP within enclave only"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-INGRESS -i eth1 -p icmp -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS -o eth1 -p icmp -j ACCEPT

echo "RBS8: Configuring TCP access for bottled services"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS -o eth1 -p tcp -d ${ENCLAVE_BOTTLE_IP} --dport ${ENTRY_PORT_ENCLAVE} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-INGRESS -i eth1 -p tcp -s ${ENCLAVE_BOTTLE_IP} --sport ${ENTRY_PORT_ENCLAVE} -j ACCEPT

echo "RBS9: Setting up socat proxy"
podman -c ${MACHINE} exec -d ${SENTRY_CONTAINER} socat TCP-LISTEN:${ENTRY_PORT_WORKSTATION},fork,reuseaddr TCP:${ENCLAVE_BOTTLE_IP}:${ENTRY_PORT_ENCLAVE}
sleep 1
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} pgrep -f "socat.*:${ENTRY_PORT_WORKSTATION}" || echo "Socat proxy not started"

echo "RBS10: Blocking ICMP cross-boundary traffic"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-FORWARD -p icmp -j DROP
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS -o eth0 -p icmp -j DROP

echo "RBS11: Setting up network forwarding"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "echo 1 > /proc/sys/net/ipv4/ip_forward"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "echo 0 > /proc/sys/net/ipv4/conf/default/rp_filter"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "echo 1 > /proc/sys/net/ipv4/conf/all/forwarding"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "echo 1 > /proc/sys/net/ipv4/conf/eth0/forwarding"

echo "RBS12: Configuring NAT"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -t nat -A POSTROUTING -o eth0 -s ${ENCLAVE_BASE_IP}/${ENCLAVE_NETMASK} ! -d ${ENCLAVE_BASE_IP}/${ENCLAVE_NETMASK} -j MASQUERADE

echo "RBS13: Configuring DNS server access"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d ${DNS_SERVER} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d ${DNS_SERVER} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -d ${ENCLAVE_SENTRY_IP} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -d ${ENCLAVE_SENTRY_IP} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-FORWARD -i eth1 -p udp --dport 53 -j DROP
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-FORWARD -i eth1 -p tcp --dport 53 -j DROP

echo "RBS14: Setting up CIDR-based access control"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS  -o eth0 -d ${UPLINK_ALLOWED_CIDRS} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-FORWARD -i eth1 -d ${UPLINK_ALLOWED_CIDRS} -j ACCEPT

echo "RBS15: Configuring sentry DNS resolution"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "echo 'nameserver ${DNS_SERVER}' > /etc/resolv.conf"

echo "RBS16: Setting up DNS Server"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} killall -9 dnsmasq 2>/dev/null || true
sleep 1

echo "RBS17: Configuring dnsmasq"
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} sh -c "cat > /etc/dnsmasq.conf << EOC
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
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-INGRESS -i eth1 -p udp --dport 53 -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-INGRESS -i eth1 -p tcp --dport 53 -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS  -o eth0 -p udp --dport 53 -d ${DNS_SERVER} -j ACCEPT
podman -c ${MACHINE} exec ${SENTRY_CONTAINER} iptables -A RBM-EGRESS  -o eth0 -p tcp --dport 53 -d ${DNS_SERVER} -j ACCEPT
echo "RBS: Sentry setup complete"

echo -e "${BOLD}Creating BOTTLE container${NC}"
podman -c ${MACHINE} create \
  --name ${BOTTLE_CONTAINER} \
  --privileged \
  --network none \
  --cap-add net_raw \
  --security-opt label=disable \
  ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}

echo -e "${BOLD}Starting BOTTLE container to get a valid PID${NC}"
podman -c ${MACHINE} start ${BOTTLE_CONTAINER}
sleep 2

echo "RBNS: Beginning bottle namespace setup"
echo "RBNS0: Getting BOTTLE PID"
BOTTLE_PID=$(podman -c ${MACHINE} inspect -f '{{.State.Pid}}' ${BOTTLE_CONTAINER})
echo "RBNS1: BOTTLE PID: ${BOTTLE_PID}"

if [ -n "${BOTTLE_PID}" ] && [ "${BOTTLE_PID}" != "0" ]; then
  echo "RBNS2: Creating and configuring veth pair"
  podman machine ssh ${MACHINE} sudo ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}
  
  echo "RBNS3: Moving veth endpoint to container namespace"
  podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BOTTLE_IN} netns ${BOTTLE_PID}
  podman machine ssh ${MACHINE} sudo nsenter -t ${BOTTLE_PID} -n ip link set ${ENCLAVE_BOTTLE_IN} name eth1
  podman machine ssh ${MACHINE} sudo nsenter -t ${BOTTLE_PID} -n ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
  podman machine ssh ${MACHINE} sudo nsenter -t ${BOTTLE_PID} -n ip link set eth1 up
  
  echo "RBNS4: Connecting BOTTLE veth to bridge"
  podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
  podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BOTTLE_OUT} up
  
  echo "RBNS5: Setting default route in BOTTLE"
  podman machine ssh ${MACHINE} sudo nsenter -t ${BOTTLE_PID} -n ip route add default via ${ENCLAVE_SENTRY_IP}
  podman machine ssh ${MACHINE} sudo nsenter -t ${BOTTLE_PID} -n bash -c "echo 'nameserver ${ENCLAVE_SENTRY_IP}' > /etc/resolv.conf"
  
  echo "RBNS: Bottle namespace setup complete"
else
  echo "ERROR: Could not retrieve BOTTLE PID or PID is 0. Got: ${BOTTLE_PID}"
  echo "Stopping container and setting up a manual network namespace approach"
  
  podman -c ${MACHINE} stop ${BOTTLE_CONTAINER}
  
  echo "RBNS-ALT: Creating network namespace manually"
  podman machine ssh ${MACHINE} sudo ip netns add ${MONIKER}-ns
  
  echo "RBNS-ALT: Creating veth pair"
  podman machine ssh ${MACHINE} sudo ip link add ${ENCLAVE_BOTTLE_OUT} type veth peer name ${ENCLAVE_BOTTLE_IN}
  
  echo "RBNS-ALT: Moving veth endpoint to namespace"
  podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BOTTLE_IN} netns ${MONIKER}-ns
  
  echo "RBNS-ALT: Configuring interfaces in namespace"
  podman machine ssh ${MACHINE} sudo ip netns exec ${MONIKER}-ns ip link set ${ENCLAVE_BOTTLE_IN} name eth1
  podman machine ssh ${MACHINE} sudo ip netns exec ${MONIKER}-ns ip addr add ${ENCLAVE_BOTTLE_IP}/${ENCLAVE_NETMASK} dev eth1
  podman machine ssh ${MACHINE} sudo ip netns exec ${MONIKER}-ns ip link set eth1 up
  podman machine ssh ${MACHINE} sudo ip netns exec ${MONIKER}-ns ip link set lo up
  
  echo "RBNS-ALT: Connecting veth to bridge"
  podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BOTTLE_OUT} master ${ENCLAVE_BRIDGE}
  podman machine ssh ${MACHINE} sudo ip link set ${ENCLAVE_BOTTLE_OUT} up
  
  echo "RBNS-ALT: Setting default route in namespace"
  podman machine ssh ${MACHINE} sudo ip netns exec ${MONIKER}-ns ip route add default via ${ENCLAVE_SENTRY_IP}
  
  echo "RBNS-ALT: Starting container with the prepared network namespace"
  podman machine ssh ${MACHINE} podman run -d --name ${BOTTLE_CONTAINER} --network ns:/var/run/netns/${MONIKER}-ns --privileged ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}
  
  echo "RBNS-ALT: Bottle namespace setup complete"
fi

echo -e "${BOLD}Visualizing network setup in podman machine...${NC}"
echo "RBNI: Network interface information"
podman machine ssh ${MACHINE} ip a

echo "RBNI: Network bridge information" 
podman machine ssh ${MACHINE} ip link show type bridge

echo "RBNI: Network namespace information"
podman machine ssh ${MACHINE} ip netns list

echo "RBNI: Route information"
podman machine ssh ${MACHINE} ip route

echo -e "${BOLD}Verifying containers${NC}"
podman -c ${MACHINE} ps -a

echo -e "${GREEN}${BOLD}Setup script execution complete${NC}"

