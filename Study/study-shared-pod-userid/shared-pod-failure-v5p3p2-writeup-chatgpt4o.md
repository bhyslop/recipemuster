# DNS Blocking Failure in Podman Pod Architecture

## Overview

This document records the architecture of the RBM Bottle Service DNS enforcement model and documents the root cause of a critical security failure observed during testing. It is intended as a reference for future system design decisions, specifically to avoid repeating assumptions about network enforcement capabilities within container runtimes.

## Version Information

**Test Environment:**
- **Podman Version**: 5.3.2 (client) / 5.3.1 (server) (Built: Wed Jan 22 05:42:46 2025)
- **VM Build Date**: 2024-11-17 16:00:00.000000000 -0800
- **VM Image SHA**: da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16
- **Mode**: Rootless (`Rootful: false`)
- **Test Date**: 2025-06-24

## Current Architecture: Shared Network Namespace

The Bottle Service architecture relies on a Podman **pod** with a **shared network namespace**. Within this pod:

- **SENTRY container**:
  - Runs as `root`
  - Granted `NET_ADMIN` and `NET_RAW`
  - Installs UID-based `iptables` rules for traffic enforcement
  - Runs a `dnsmasq` instance for DNS resolution and filtering

- **BOTTLE container**:
  - Runs as unprivileged user (`UID 1000`)
  - Has **no NET capabilities**
  - Shares the network namespace with SENTRY
  - All its egress is intended to be filtered by `iptables` rules authored by SENTRY

The system depends on iptables `OUTPUT` chain filtering with `-m owner --uid-owner 1000` to control BOTTLE's outbound traffic.

## Enforcement Strategy

SENTRY sets:
- A **default DROP policy** in the `OUTPUT` chain
- Explicit **ACCEPT rules** for:
  - Loopback
  - Allowed CIDRs
  - dnsmasq on `127.0.0.1`
- A **DROP fallback** for UID 1000 to block all other destinations
- `REDIRECT` rules in the `nat OUTPUT` chain to intercept BOTTLE's DNS traffic and force it through dnsmasq

## Failure Mode

Despite a correctly configured `iptables` rule set, the test:

```sh
nc -w 2 -zv 8.8.8.8 53
```

executed **from within the BOTTLE container** succeeds—indicating an outbound TCP connection to a blocked IP and port.

### Key Observations

- The rule `iptables -I OUTPUT 1 -d 8.8.8.8 -p tcp --dport 53 -j DROP` **does not block** the traffic.
- This failure **is not UID-dependent**—even non-owner-matching DROP rules are bypassed.
- This confirms that the `OUTPUT` chain is **not enforced for container egress traffic**, despite being defined in the shared namespace.

## Root Cause

Podman's pod networking model, particularly in environments using Podman Machine (e.g., WSL2 or macOS virtualization), does **not enforce `OUTPUT` chain rules** for container-initiated traffic.

This bypass undermines all attempts to control container egress using:
- `-m owner --uid-owner` rules
- `DROP` rules in `OUTPUT` targeting specific destinations or protocols
- Any enforcement strategy depending on container egress traversing the shared namespace's `OUTPUT` chain

### Mechanism Hypothesis

Container traffic may:
- Egress through a host-bridge or virtual NIC that **bypasses iptables OUTPUT**
- Rely on NAT or forwarding rules **inserted outside** the container's iptables filter path

## Conclusion

**The assumption that `