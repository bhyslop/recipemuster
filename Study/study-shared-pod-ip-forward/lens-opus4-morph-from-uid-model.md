# Podman RBM Network Security Architecture Revision: OUTPUT to FORWARD Chain Migration

## Context

We have a Recipe Bottle Makefile (RBM) system that attempts to enforce network security policies using iptables rules within Podman pods. Our current implementation uses the OUTPUT chain with UID-based filtering, but testing revealed that **Podman's pod networking bypasses the OUTPUT chain entirely** for container-initiated traffic.

### Discovery
After extensive testing with Podman 5.3.x, we discovered that container traffic in Podman pods:
- Does NOT traverse the OUTPUT chain (designed for locally-generated host traffic)
- Likely traverses the FORWARD chain as it's treated as forwarded traffic through bridge interfaces
- Can potentially be controlled using FORWARD chain rules with IP-based filtering instead of UID-based

## Current Architecture (Failing)
- **Sentry Container**: Runs as root with NET_ADMIN/NET_RAW, configures iptables
- **Bottle Container**: Runs as UID 1000, intended to be restricted by iptables
- **Shared pod network namespace**: Both containers share the same network stack
- **Enforcement**: OUTPUT chain rules with `-m owner --uid-owner 1000` (NOT WORKING)

## Proposed Solution
Migrate from OUTPUT chain (UID-based) to FORWARD chain (IP-based) filtering:
1. Assign static IP addresses to containers within the pod
2. Use FORWARD chain rules based on source IP instead of UID
3. Move DNS interception from OUTPUT to PREROUTING (nat table)
4. Maintain the same security goals but with a different enforcement mechanism

## Requirements for New Implementation

### Network Configuration
1. Configure the pod with a custom network that allows static IP assignment
2. Assign specific IPs:
   - Sentry container: e.g., 10.88.0.2
   - Bottle container: e.g., 10.88.0.3
   - Pod subnet: e.g., 10.88.0.0/24

### Iptables Rules Migration
Replace all OUTPUT chain rules with FORWARD chain equivalents:
- `OUTPUT -m owner --uid-owner 1000` ? `FORWARD -s 10.88.0.3`
- DNS REDIRECT in nat OUTPUT ? DNAT/REDIRECT in nat PREROUTING
- Maintain the same allow/deny logic but based on source IP

### DNS Interception
- Move DNS interception from nat OUTPUT to nat PREROUTING
- Use DNAT to redirect bottle container's DNS queries to sentry's dnsmasq
- Ensure dnsmasq binds to the pod network interface (not just localhost)

### Testing Requirements
The revised implementation must block:
```bash
# From bottle container (10.88.0.3)
nc -w 2 -zv 8.8.8.8 53  # Should FAIL
dig @8.8.8.8 anthropic.com  # Should FAIL
```

While allowing:
```bash
# From bottle container
dig @<sentry-ip> anthropic.com  # Should work via dnsmasq
curl https://anthropic.com  # Should work if in allowed domains
```

## Current Implementation Script

Find the current `rbss.sentry.sh` script in this chat.

## Task

Please create a revised version of the `rbss.sentry.sh` script that:

1. **Adds new environment variables** for container IPs:
   - `RBRR_SENTRY_IP` (e.g., 10.88.0.2)
   - `RBRR_BOTTLE_IP` (e.g., 10.88.0.3)
   - `RBRR_POD_SUBNET` (e.g., 10.88.0.0/24)

2. **Replaces all OUTPUT chain rules** with FORWARD chain equivalents:
   - Use `-s ${RBRR_BOTTLE_IP}` instead of `-m owner --uid-owner ${RBRR_BOTTLE_UID}`
   - Ensure all bottle traffic control moves to FORWARD chain

3. **Updates DNS interception** to use PREROUTING:
   - Replace nat OUTPUT rules with nat PREROUTING rules
   - Use DNAT to redirect DNS from bottle IP to sentry IP
   - Configure dnsmasq to listen on the pod network interface

4. **Maintains the same security model**:
   - Default DROP for bottle traffic
   - Allow only specified CIDRs
   - DNS filtering through dnsmasq
   - Service port access when enabled

5. **Includes verification commands** to test the FORWARD chain:
   - Add test commands that verify FORWARD chain is processing packets
   - Include packet counter checks to confirm rules are being hit

Please also provide:
- Any necessary Podman pod/container creation commands with static IP configuration
- Explanation of key changes and why they solve the OUTPUT chain bypass issue
- Test commands to verify the security enforcement is working


