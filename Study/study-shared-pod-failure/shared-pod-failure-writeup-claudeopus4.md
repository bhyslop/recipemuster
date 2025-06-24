# Recipe Bottle Makefile (RBM) Network Security Architecture Failure Analysis

## Executive Summary

The Recipe Bottle Makefile system attempted to enforce network security policies using iptables rules within Podman pods. Testing revealed that **Podman's pod networking implementation does not enforce iptables OUTPUT chain rules for container egress traffic**, rendering the entire security model ineffective. This document captures the precise architecture and failure mode for future reference.

## Current Architecture Overview

### Container Structure
The RBM system creates a security boundary using two containers within a single Podman pod:

1. **Sentry Container**
   - Runs with NET_ADMIN and NET_RAW capabilities
   - Executes `rbss.sentry.sh` to configure iptables rules
   - Intended to enforce all network policies for the pod
   - Runs as root with privileged network access

2. **Bottle Container**
   - Runs as unprivileged user (UID 1000 as configured in `RBRR_BOTTLE_UID`)
   - Contains the actual service/application
   - All network access supposedly controlled by Sentry's iptables rules
   - No network capabilities granted

### Network Model
Both containers share the same network namespace through Podman's pod mechanism:
- Single shared network stack
- Common localhost (127.0.0.1)
- Unified iptables ruleset
- Shared network interfaces

### Security Implementation

The `rbss.sentry.sh` script implements a four-phase security configuration:

#### Phase 1: IPTables Initialization
```bash
# Set default DROP policies
iptables -P INPUT DROP
iptables -P FORWARD DROP
iptables -P OUTPUT DROP

# Accept loopback and established connections
iptables -I INPUT -i lo -j ACCEPT
iptables -A INPUT -m state --state RELATED,ESTABLISHED -j ACCEPT
iptables -A OUTPUT -m state --state RELATED,ESTABLISHED -j ACCEPT

# Create RBM chains
iptables -N RBM-INGRESS
iptables -N RBM-EGRESS
iptables -A INPUT -j RBM-INGRESS
iptables -A OUTPUT -j RBM-EGRESS
```

#### Phase 2: Port Configuration
When `RBRN_ENTRY_ENABLED=true`:
```bash
# Allow incoming service connections
iptables -A RBM-INGRESS -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} -j ACCEPT

# Allow bottle to respond from service port
iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} \
  -p tcp --sport ${RBRN_ENTRY_PORT_ENCLAVE} -j ACCEPT
```

#### Phase 3: Access Control
```bash
# Block all bottle traffic by default
iptables -I RBM-EGRESS 1 -m owner --uid-owner ${RBRR_BOTTLE_UID} -j DROP

# Then allow specific destinations
for cidr in ${RBRN_UPLINK_ALLOWED_CIDRS}; do
    iptables -I RBM-EGRESS 1 -m owner --uid-owner ${RBRR_BOTTLE_UID} \
      -d ${cidr} -j ACCEPT
done
```

#### Phase 4: DNS Configuration
```bash
# Run local dnsmasq for DNS filtering
# Redirect bottle DNS queries to local dnsmasq
iptables -t nat -A OUTPUT -m owner --uid-owner ${RBRR_BOTTLE_UID} \
  -p udp --dport 53 -j REDIRECT --to-ports 53

# Block direct DNS access to unauthorized servers
iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} \
  -p udp --dport 53 ! -d 127.0.0.1 -j DROP
```

## The Failure

### Expected Behavior
The test `ztest_bottle_dns_block_direct_rule` expected:
```bash
# This should fail - bottle shouldn't reach 8.8.8.8:53
! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @8.8.8.8 anthropic.com
! $(MBT_PODMAN_EXEC_BOTTLE_I) nc -w 2 -zv 8.8.8.8 53
```

### Actual Behavior
Both commands succeed. The bottle container can:
- Establish TCP connections to 8.8.8.8:53
- Send DNS queries to 8.8.8.8
- Bypass all iptables OUTPUT chain rules

### Root Cause Analysis

Testing revealed that **Podman's pod networking does not enforce iptables OUTPUT chain rules for container-initiated traffic**. 

#### Evidence from Testing

1. **Rules are correctly configured**:
```bash
# iptables -L OUTPUT -n -v shows:
Chain OUTPUT (policy DROP 0 packets, 0 bytes)
    0     0 ACCEPT     all  --  *      *       0.0.0.0/0   0.0.0.0/0   state RELATED,ESTABLISHED
    0     0 RBM-EGRESS all  --  *      *       0.0.0.0/0   0.0.0.0/0

# iptables -L RBM-EGRESS -n -v shows:
Chain RBM-EGRESS (1 references)
    0     0 ACCEPT     all  --  *      *       0.0.0.0/0   127.0.0.1   owner UID match 1000
    0     0 ACCEPT     all  --  *      *       0.0.0.0/0   160.79.104.0/23   owner UID match 1000
    0     0 DROP       all  --  *      *       0.0.0.0/0   0.0.0.0/0   owner UID match 1000
```

2. **Explicit DROP rule test**:
```bash
# Added explicit rule without owner match
iptables -I OUTPUT 1 -d 8.8.8.8 -p tcp --dport 53 -j DROP

# From bottle container - STILL SUCCEEDS
nc -w 2 -zv 8.8.8.8 53
# Connection to 8.8.8.8 53 port [tcp/domain] succeeded!
```

3. **Packet counters remain at zero**:
   - No packets hit the OUTPUT chain rules
   - No packets hit the RBM-EGRESS rules
   - Traffic bypasses iptables OUTPUT filtering entirely

### Technical Explanation

Podman pods use a network namespace with special handling:
1. Containers in the pod share the network namespace
2. The network stack treats container-initiated traffic differently than host traffic
3. OUTPUT chain rules are designed for locally-generated traffic from processes
4. Container traffic appears to bypass OUTPUT chain processing in Podman's implementation
5. This is likely due to how container networking interfaces with the kernel's netfilter subsystem

### Implications

The entire RBM security model relies on OUTPUT chain filtering, which means:
- DNS blocking doesn't work
- Egress CIDR restrictions don't work
- Port-based filtering doesn't work
- The bottle container has unrestricted network access

## Configuration Context

From `nameplate.nsproto.mk`:
```makefile
RBRN_UPLINK_DNS_ENABLED     = 1
RBRN_UPLINK_ACCESS_ENABLED  = 1
RBRN_UPLINK_DNS_GLOBAL      = 0    # Should limit DNS
RBRN_UPLINK_ACCESS_GLOBAL   = 0    # Should limit access
RBRN_UPLINK_ALLOWED_CIDRS   = 160.79.104.0/23
RBRN_UPLINK_ALLOWED_DOMAINS = anthropic.com
```

These restrictions are completely ineffective due to the OUTPUT chain bypass.

## Conclusion

The RBM pod-based architecture fundamentally cannot enforce egress network policies through iptables OUTPUT chain rules. This is not a configuration error or implementation bug in RBM - it's a limitation of how Podman implements pod networking. The security model's core assumption that iptables OUTPUT rules would control container egress traffic is invalid in the Podman pod context.
