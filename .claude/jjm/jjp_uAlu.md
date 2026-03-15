## Hairpin Peer Access — Design Context

### Problem
Bottled containers cannot reach each other's host ports. Each bottle is isolated behind its own sentry on its own enclave network. There is no mechanism for Bottle A to connect to Bottle B's published entry port.

### Traffic Path (Proven Feasible)
```
Bottle A → enclave A → Sentry A (eth0, MASQUERADE) → transit-A bridge
    → host routing table →
transit-B bridge → Sentry B (eth0, socat) → enclave B → Bottle B
```

Key insight: both transit bridges have external connectivity enabled, so the host kernel can route between them. Sentry B's existing entry path (socat + RBM-INGRESS rules) handles the inbound connection identically whether it came from the host user or from another sentry. No changes needed on the receiving side.

### Security Model — Unbroken
The sentry remains the sole policy enforcement point. The bottle gains no new interfaces, capabilities, or bypass paths. Peer access is a new category of allowed destination in the sentry's iptables ruleset, alongside the existing entry (inbound) and uplink (outbound internet) categories. Same mechanism, same enforcement architecture.

### Nameplate Configuration Model (Decided)
Two new variables, following the existing mode+gated-group pattern:

```
RBRN_PEER_ACCESS_MODE=disabled          # or: allowlist
RBRN_PEER_ALLOWED_MONIKERS=srjcl,nsproto  # only when mode=allowlist
```

No global mode. Peer access must always be explicit. Moniker is sufficient — port comes from the peer's own rbrn_entry_port_workstation, IP is resolved at runtime. Follows existing pattern of rbrn_uplink_access_mode + gated rbrn_uplink_allowed_cidrs.

### RBRN Spec Additions
```
rbrn_peer_access_mode:        disabled | allowlist
rbrn_group_peer_allowlist:    (gated on peer_access_mode=allowlist)
  rbrn_peer_allowed_monikers: comma-separated moniker list
```

### Implementation Surface (Three Things)
1. Nameplate variables — add to RBRN spec + kindle/validate/render
2. Iptables rules — new peer-access step in security_config that for each allowed peer adds:
   - RBM-EGRESS: ACCEPT on eth0 to peer's transit IP + entry port
   - RBM-FORWARD: ACCEPT from eth1 to peer's transit IP + entry port
3. Peer IP discovery — host-side orchestration resolves moniker to transit IP via podman inspect on peer's sentry container, injects into sentry before security_config runs

### Audit Cross-Instance Check
For each moniker in rbrn_peer_allowed_monikers, verify:
- Named peer nameplate exists
- Peer has rbrn_entry_mode=enabled (can't peer-connect to a bottle with no entry port)

### Design Decision: Unilateral vs Bilateral
Decided unilateral: A declares it can reach B. B doesn't need to consent — it already accepts connections on its entry port from anyone on the transit side. Bilateral would require cross-nameplate validation and adds complexity without clear security benefit (the transit network is already internal).

### Docker Compatibility
The mechanism is pure IP networking on Linux bridges. Works identically with Docker (bridge driver, docker network connect for dual-homing, same iptables inside privileged container). No runtime-specific concerns.

### Open Questions for Refinement
- Type voicing for the moniker list (comma-separated like CIDRs, or different?)
- Whether peer discovery should happen in sentry_start orchestration or in security_config script
- Test fixture design for validating peer connectivity