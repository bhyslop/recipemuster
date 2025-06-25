# Recipe Bottle Network Architecture: Post-Mortem Analysis

**Date**: June 2025  
**Author**: Brad Hyslop  
**Subject**: Recipe Bottle (RBM) Network Namespace Architecture Failure

## Executive Summary

Recipe Bottle (RBM) implemented a sophisticated container network isolation strategy using manual network namespace manipulation to create a DMZ-style architecture with fine-grained traffic control. This approach, which worked in early Podman 5.x releases, has been systematically blocked by security hardening in Podman 5.3+ across all operational modes (rootless and rootful).

**Core Finding**: Podman now prohibits containers from joining externally-created network namespaces, regardless of privileges or creation method. The `--network ns:PATH` syntax remains but is effectively unusable.

## Architecture Overview

### Design Goals
- Air-gapped container isolation with selective internet access
- Fine-grained traffic control (DNS filtering, CIDR restrictions)
- Two-tier security model: SENTRY (gateway) + BOTTLE (isolated service)

### Implementation Strategy

The architecture relied on three key technical capabilities:
1. Creating network namespaces outside Podman's control
2. Manually configuring veth pairs and bridges
3. Attaching containers to these pre-configured namespaces

### Network Topology

```
Internet ← → SENTRY Container (privileged)
              ├─ eth0: Bridge network (internet access)
              └─ eth1: Custom veth → Virtual Bridge
                                           │
                                           ↓
                      BOTTLE Container (--network none → custom namespace)
                        └─ eth1: Custom veth (isolated, filtered access)
```

### Key Implementation Steps

1. **SENTRY Setup** (rbns.sentry.sh):
   - Launch privileged container with bridge networking
   - Extract container PID: `podman inspect -f '{{.State.Pid}}'`
   - Create virtual bridge and veth pairs
   - Move veth into container namespace: `ip link set <veth> netns <PID>`
   - Configure iptables for filtering

2. **BOTTLE Setup** (rbnb.bottle.sh):
   - Create container with `--network none`
   - Manually create network namespace
   - Configure veth pair and routing
   - Attempt to move interface into BOTTLE container's namespace

## Failure Analysis

### Test Methodology

Systematic testing across multiple configurations documented in Snnp study scripts:
- **Podman versions**: 5.3.2, 5.5.2
- **VM modes**: Rootless and Rootful
- **Multiple approaches**: See individual Snnp-*.sh scripts

### Universal Failure Pattern

All attempts to attach containers to external namespaces failed with:
```
Error: crun: cannot setns '/var/run/netns/<namespace>': Operation not permitted: OCI permission denied
```

### Specific Test Results

| Test Script | Approach | Result |
|------------|----------|---------|
| Snnp-base-netns.sh | Basic `ip netns add` + `--network ns:` | Failed: Operation not permitted |
| Snnp-cap-add.sh | Added CAP_SYS_ADMIN, CAP_NET_ADMIN | Failed: Same error |
| Snnp-privileged-bottle.sh | Used --privileged flag | Failed: Same error |
| Snnp-podman-unshare*.sh | Podman's own unshare command | Failed: Permission denied on /var/run/netns |
| Snnp-unshare-*.sh | Linux unshare utility | Failed: Various permission errors |

**Note**: See individual Snnp study scripts for detailed command sequences and version-specific results.

### Root Cause

Podman has implemented strict security controls that:
1. Prevent containers from using `setns()` on arbitrary network namespaces
2. Restrict namespace operations even in rootful mode
3. Enforce that containers only use Podman-managed namespaces

This represents a fundamental security philosophy change - prioritizing container isolation integrity over operational flexibility.

## Lessons Learned

### 1. Dependency on Container Runtime Internals
Recipe Bottle's architecture depended on low-level container runtime behaviors that were never part of Podman's stable API contract. The ability to manipulate namespaces was an implementation detail, not a feature.

### 2. Security Model Evolution
Container runtimes are progressively restricting namespace operations to prevent:
- Container escape attacks
- Privilege escalation through namespace manipulation
- Cross-container network access violations

### 3. Podman's Network Architecture Direction
Podman is moving toward plugin-based networking (CNI/Netavark) as the only supported method for complex network topologies. Direct namespace manipulation is considered a security anti-pattern.

### 4. Documentation Gaps
The `--network ns:PATH` option remains documented but its practical limitations are not clearly stated. This creates false expectations about architectural possibilities.

## Technical Debt

The Recipe Bottle codebase contains significant infrastructure for:
- Network namespace lifecycle management
- Complex iptables rule generation
- Multi-stage container initialization

This code is now effectively obsolete but represents considerable implementation effort.

## Conclusion

Recipe Bottle's network isolation strategy, while technically sophisticated, relied on container runtime capabilities that Podman has deliberately removed. The architecture cannot be salvaged in its current form - any future implementation must work within Podman's supported networking models.

The systematic failure across all tested approaches, modes, and privilege levels confirms this is not a configuration issue but a fundamental incompatibility with modern Podman's security model.

## References

- Recipe Bottle implementation: rbp.podman.mk, rbnb.bottle.sh, rbns.sentry.sh
- Network namespace study: Study/study-net-namespace-permutes/Snnp-*.sh
- Test methodology: Study/study-net-namespace-permutes/README.md