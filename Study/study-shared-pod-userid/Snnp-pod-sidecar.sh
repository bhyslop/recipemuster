#!/bin/bash

# Copyright 2024 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
# Gemini Co-Author: Gemini

set -e

# --- Podman connection configuration ---
PODMAN_CMD="podman -c pdvm-rbw"
# ---

# --- Load common constants and functions ---
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "SNNP: Get constants from" ${SCRIPT_DIR}
source "$SCRIPT_DIR/Snnp-common.sh"
# ---

# --- Define Pod-specific names ---
POD_NAME="enclave-pod"
SENTRY_SETUP_SCRIPT="/tmp/sentry_setup.sh"
# ---

# --- Comprehensive Cleanup ---
echo -e "${BOLD}Cleaning up previous Pods and Containers...${NC}"
${PODMAN_CMD} pod stop ${POD_NAME} 2>/dev/null || echo "Pod ${POD_NAME} not running."
${PODMAN_CMD} pod rm -f ${POD_NAME} 2>/dev/null || echo "Pod ${POD_NAME} not found."
snnp_cleanup_all # This function from Snnp-common.sh cleans up containers
echo -e "${GREEN}Cleanup complete.${NC}"
# ---

# --- Wait for Podman machine to be ready ---
echo -e "${BOLD}Verifying Podman machine connection...${NC}"
sleep 3
snnp_verify_machine_connection
echo -e "${GREEN}Connection successful.${NC}"
# ---

# --- Pod Creation ---
echo -e "${BOLD}Creating Pod: ${POD_NAME}...${NC}"
${PODMAN_CMD} pod create --name ${POD_NAME} -p ${ENTRY_PORT_WORKSTATION}:${ENTRY_PORT_WORKSTATION}
echo -e "${GREEN}Pod created successfully.${NC}"
# ---

# --- Sentry Container Start ---
echo -e "${BOLD}Starting SENTRY container in Pod...${NC}"
# SENTRY needs NET_ADMIN to configure the shared network namespace's firewall
${PODMAN_CMD} run -d --name ${SENTRY_CONTAINER} \
  --pod ${POD_NAME} \
  --cap-add NET_ADMIN,NET_RAW \
  ${SENTRY_REPO_PATH}:${SENTRY_IMAGE_TAG}
echo -e "${GREEN}SENTRY container started.${NC}"
# ---

# --- Sentry Configuration Script Injection ---
echo -e "${BOLD}Injecting SENTRY setup script...${NC}"
# This script will run inside the Sentry container to set up the firewall
${PODMAN_CMD} exec ${SENTRY_CONTAINER} bash -c "cat > ${SENTRY_SETUP_SCRIPT}" <<EOF
#!/bin/bash
set -ex

echo "--- Sentry Internal Setup ---"

# 1. Create users and groups
groupadd --force net-allowed
useradd --system --shell /usr/sbin/nologin --gid net-allowed sentry-proxy || echo "User sentry-proxy exists"
useradd --system --shell /usr/sbin/nologin bottle-app || echo "User bottle-app exists"
BOTTLE_UID=\$(id -u bottle-app)
echo "Bottle app UID is: \$BOTTLE_UID"

# 2. Configure iptables firewall (Mandatory Access Control)
echo "Configuring firewall..."
iptables -t nat -F  # Flush NAT table
iptables -F         # Flush Filter table
iptables -X         # Delete all user-chains

# Default Deny Policy
iptables -P INPUT   DROP
iptables -P FORWARD DROP
iptables -P OUTPUT  DROP

# Allow loopback traffic (essential for redirects)
iptables -A INPUT  -i lo -j ACCEPT
iptables -A OUTPUT -o lo -j ACCEPT

# Allow established connections
iptables -A INPUT  -m state --state ESTABLISHED,RELATED -j ACCEPT
iptables -A OUTPUT -m state --state ESTABLISHED,RELATED -j ACCEPT

# Allow SENTRY proxy to access the internet
iptables -A OUTPUT -m owner --gid-owner net-allowed -j ACCEPT

# Redirect BOTTLE traffic to local Sentry listeners
# Hijack DNS traffic from the bottle user and redirect to our local DNS proxy
iptables -t nat -A OUTPUT -p udp --dport 53 -m owner --uid-owner \$BOTTLE_UID -j REDIRECT --to-ports 5353
iptables -t nat -A OUTPUT -p tcp --dport 53 -m owner --uid-owner \$BOTTLE_UID -j REDIRECT --to-ports 5353

# Hijack all other TCP traffic from the bottle user and redirect to our TCP proxy
iptables -t nat -A OUTPUT -p tcp -m owner --uid-owner \$BOTTLE_UID -j REDIRECT --to-ports 8080

echo "Firewall configured."
iptables-save

# 3. Start mock listeners as the sentry-proxy user
echo "Starting mock listeners..."
# Mock DNS server - replies to any query with a fixed IP
su -s /bin/bash -c "socat -v UDP-LISTEN:5353,fork,reuseaddr SYSTEM:'echo \"\$\$ > /tmp/dns.pid\"; echo -e \"\n\n- -. 300 IN A 1.2.3.4\"; sleep 1' &" sentry-proxy

# Mock TCP proxy - just logs connections and closes them
su -s /bin/bash -c "socat -v TCP-LISTEN:8080,fork,reuseaddr SYSTEM:'echo \"\$\$ > /tmp/tcp.pid\"; echo -e \"HTTP/1.1 501 Not Implemented\r\n\r\n\"' &" sentry-proxy

echo "--- Sentry Internal Setup Complete ---"
EOF

${PODMAN_CMD} exec ${SENTRY_CONTAINER} chmod +x ${SENTRY_SETUP_SCRIPT}
echo -e "${BOLD}Executing SENTRY setup script...${NC}"
${PODMAN_CMD} exec ${SENTRY_CONTAINER} bash ${SENTRY_SETUP_SCRIPT}
# ---

# --- Bottle Container Start ---
echo -e "${BOLD}Starting BOTTLE container in Pod...${NC}"
# BOTTLE runs as an unprivileged user with NO capabilities.
${PODMAN_CMD} run -d --name ${BOTTLE_CONTAINER} \
  --pod ${POD_NAME} \
  --cap-drop=ALL \
  ${BOTTLE_REPO_PATH}:${BOTTLE_IMAGE_TAG}
echo -e "${GREEN}BOTTLE container started.${NC}"
# ---

# --- Verification ---
sleep 3
echo -e "${BOLD}--- VERIFICATION ---${NC}"

echo -e "${BOLD}Pod Status:${NC}"
${PODMAN_CMD} pod ps

echo -e "\n${BOLD}Sentry Firewall Rules:${NC}"
${PODMAN_CMD} exec ${SENTRY_CONTAINER} iptables-save

echo -e "\n${BOLD}Testing BOTTLE's network access...${NC}"

echo -e "\n${CYAN}1. Test basic network with nc (netcat)${NC}"
# Try to connect to Google DNS - should timeout due to firewall
${PODMAN_CMD} exec ${BOTTLE_CONTAINER} timeout 2 nc -v 8.8.8.8 53 || echo -e "${GREEN}Connection blocked as expected.${NC}"

echo -e "\n${CYAN}2. Test with wget (usually pre-installed)${NC}"
# Try HTTP - should be redirected to local proxy
${PODMAN_CMD} exec ${BOTTLE_CONTAINER} timeout 2 wget -O- http://example.com 2>&1 || echo -e "${GREEN}HTTP blocked/redirected as expected.${NC}"

echo -e "\n${CYAN}3. Check if we can see the network from BOTTLE${NC}"
${PODMAN_CMD} exec ${BOTTLE_CONTAINER} cat /proc/net/tcp || echo "Cannot read network info"

echo -e "\n${CYAN}4. Verify SENTRY listeners are running${NC}"
${PODMAN_CMD} exec ${SENTRY_CONTAINER} ss -tlnp | grep -E "5353|8080" || echo "No listeners found"

echo -e "\n${CYAN}5. Show network namespace info${NC}"
echo "BOTTLE container:"
${PODMAN_CMD} exec ${BOTTLE_CONTAINER} ip addr show || echo "Cannot show IP info"
echo -e "\nSENTRY container:"
${PODMAN_CMD} exec ${SENTRY_CONTAINER} ip addr show

echo -e "\n${GREEN}${BOLD}--- VERIFICATION COMPLETE ---${NC}"
# ---

