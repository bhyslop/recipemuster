# Crucible Conduit Architecture — Design Exploration

Date: 2026-04-15

## Problem

A crucible bottle needs to reach external cloud services (LLM inference,
Git hosting, etc.) through the sentry's network enforcement.  Today the
sentry supports CIDR + domain allowlists, which work but have two
weaknesses:

1. **Floating CIDRs.**  Cloud providers publish IP ranges that change.
   AWS publishes theirs at `ip-ranges.amazonaws.com/ip-ranges.json`;
   GCP and Azure similarly.  A static `RBRN_UPLINK_ALLOWED_CIDRS` entry
   drifts over time or must be overly broad.

2. **Workstation surface area.**  Credentials for cloud services
   (AWS IAM keys, GCP service account JSON, SSH keys) currently reside
   on the operator's workstation — a large, hard-to-harden attack
   surface.  The crucible is a minimal, auditable environment; credentials
   belong there, not on the workstation.

## Approaches Evaluated

### A. Sentry allowlist only (no tunnel)

Use existing `RBRN_UPLINK_ACCESS_MODE=allowlist` with cloud provider
CIDRs and domains.  Bottle reaches the public endpoint of the service.

- **Pros:** Zero development.  Works today.
- **Cons:** Floating CIDRs.  Traffic crosses public internet (TLS
  protects content but not metadata).  Broad CIDRs may include
  unrelated services (same CDN/CIDR sprawl problem as the existing
  roadmap item on CDN-hosted domains).

Verdict: viable first step for proving a Bedrock nameplate, but not
a long-term architecture.

### B. AWS Client VPN

OpenVPN-based managed service.  Terminates in your VPC.  Bottle traffic
exits the sentry, reaches the VPN client on the workstation (or in a
sidecar), enters the tunnel.

- **Pros:** Managed service, mutual TLS or SAML auth.
- **Cons:** VPN client runs on the workstation (doesn't help the
  surface-area concern) or requires a sidecar container (new
  architecture).  ~$0.10/hr + $0.05/GB.

Verdict: doesn't move the trust boundary to the right place.

### C. WireGuard in the sentry (recommended)

Add WireGuard to the sentry image.  The sentry terminates the tunnel
directly.  Bottle traffic routes through `wg0` to a peer in the
target VPC.  The VPC hosts PrivateLink endpoints for cloud services.

- **Pros:**
  - Sentry is already privileged, already the network gateway.
  - WireGuard is ~400KB on Alpine, ~4000 lines of kernel code.
  - Static keypair exchange (no PKI, no CA, no certificate expiry).
  - One stable CIDR (the VPC range) replaces floating provider ranges.
  - Credentials can live in the crucible, not the workstation.
  - Cloud-agnostic: works with any VPC that has a WireGuard peer.
- **Cons:**
  - Sentry image development (new interface, new iptables rules).
  - VPC infrastructure required on the provider side.
  - Key management is simple but still needs a regime story.

#### WireGuard packet flow

```
Bottle
  → enclave network
  → Sentry iptables FORWARD
  → wg0 (encrypted UDP)
  → public internet (single UDP stream to known endpoint)
  → VPC WireGuard peer (small EC2/VM)
  → VPC internal routing
  → PrivateLink endpoint
  → Cloud service (Bedrock, GitLab, etc.)
```

#### Sentry changes

Already present:
- Privileged container mode (required for iptables; also required for wg0)
- iptables FORWARD chain (add rules for wg0 interface)
- Allowlist CIDR machinery (VPC CIDR replaces floating cloud ranges)

New:
- `wg-tools` Alpine package (~400KB)
- `wg0` interface setup in sentry startup, after iptables, before health check
- Nameplate regime fields for tunnel configuration

#### Proposed nameplate fields

```
RBRN_TUNNEL_MODE=wireguard|disabled
RBRN_TUNNEL_PRIVATE_KEY_PATH=<path within regime>
RBRN_TUNNEL_PEER_PUBLIC_KEY=<base64 string>
RBRN_TUNNEL_PEER_ENDPOINT=<host:port>
RBRN_TUNNEL_PEER_ALLOWED_IPS=<CIDR>
RBRN_TUNNEL_ADDRESS=<sentry tunnel IP/mask>
```

#### VPC peer requirements

- Small instance (t3.nano or equivalent) running WireGuard
- Security group: UDP on WireGuard port from operator's IP only
- Routing to PrivateLink endpoints within the VPC
- Static Elastic IP for stable `RBRN_TUNNEL_PEER_ENDPOINT`

### D. Site-to-Site VPN

IPsec tunnel, designed for office routers connecting to cloud VPCs.
Overkill for a single workstation/crucible.  Dismissed.

## Service Compatibility Survey

Which services support PrivateLink (making the tunnel worthwhile)?

| Service | PrivateLink | Notes |
|---------|-------------|-------|
| AWS Bedrock | Yes | Interface endpoint in VPC.  Known requirement. |
| AWS SageMaker | Yes | Alternative to Bedrock for custom models. |
| GitLab Dedicated | Yes (AWS) | GitLab runs isolated instance, PrivateLink available. |
| GitLab.com (SaaS) | No | Public endpoints only.  Sentry allowlist + TLS ceiling. |
| GitHub.com | No | Public endpoints only.  No VPN/PrivateLink offering. |
| GitHub Enterprise Server | N/A | Self-hosted, so you control the network. |
| GCP Vertex AI | Yes (Private Service Connect) | GCP equivalent of PrivateLink. |
| Azure OpenAI | Yes | Azure Private Endpoint. |

For SaaS services without PrivateLink (GitHub.com, GitLab.com), the
sentry allowlist + TLS is the realistic ceiling.  The sentry still
provides the critical value: the bottle can't reach anything the sentry
doesn't allow, and the workstation is cut out of the data path.

## Credential Confinement

Orthogonal to the tunnel but naturally paired with it.

Today: AWS credentials on the workstation → workstation compromise
exposes cloud service access.

Proposed: credentials injected into the crucible via nameplate/regime
configuration.  The workstation starts the crucible but never holds
the service credentials.  The sentry or bottle holds them; the
workstation's only credential is "permission to start this crucible."

This works for:
- AWS IAM keys (env vars or mounted credential file in bottle)
- GCP service account JSON (already has a regime path pattern via RBRA)
- SSH keys for Git hosting
- API tokens for any service

The regime already has `RBRA` (credential file) patterns.  Extending
to tunnel keys and service credentials follows the same model.

## Threat Model Summary

| Threat | Sentry allowlist | + WireGuard tunnel | + Credential confinement |
|--------|------------------|--------------------|--------------------------|
| Bottle escape to unexpected destination | Blocked | Blocked | Blocked |
| Credential theft from workstation | Exposed | Exposed | **Blocked** |
| MITM on service connection | TLS only | **Encrypted tunnel + TLS** | Encrypted tunnel + TLS |
| Lateral movement from compromised app | Blocked (bottle isolated) | Blocked | Blocked |
| Floating CIDR drift | Manual tracking | **Eliminated** (stable VPC CIDR) | Eliminated |
| CDN CIDR sprawl (existing roadmap item) | Vulnerable | **Eliminated** for tunnel-capable services | Eliminated |

## Relationship to Existing Roadmap Items

- **CIDR allowlist granularity vs CDN-hosted domains** (RBSHR Network
  Security Model): the tunnel eliminates this problem for any service
  reachable via PrivateLink.  For SaaS-only services, the existing
  mitigations (dynamic CIDR from DNS, TLS SNI enforcement) remain
  relevant.

- **VPC Service Controls perimeter** (RBSHR Build Pipeline Security):
  if a VPC is stood up for conduit purposes, the VPC-SC perimeter for
  Cloud Build could share it.  Design the VPC to serve both consumers.

- **Hairpin peer access** (RBSHR Container Runtime): orthogonal.
  Peer access is intra-host; conduit is bottle-to-external.
