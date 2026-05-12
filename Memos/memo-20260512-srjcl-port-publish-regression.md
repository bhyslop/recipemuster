# srjcl Jupyter Connectivity Regression — Port-Publish Interface Mismatch

**Date:** 2026-05-12
**Status:** Diagnosis complete, repair not yet implemented
**Branch under test:** `pym-₢A_AAP`
**Branch tip when bug observed (both platforms):** `28d1411f0669d048659587bf5ce5fc1b3a0af139`
**Surface:** AAP gauntlet acceptance gate (`tt/rbw-tP.QualifyPristine.sh`)

> The branch tip `28d1411f` carries three fix-inline commits applied during the AAP run (`.gitignore` for the harness lock, marshal-zero state, srjcl readiness-delay bump from 30s → 120s). None of those altered the regression — the bug is rooted in `rbjs_sentry.sh` and `rbob_compose.yml` and predates the AAP branch by several pristine-relevant commits.

## TL;DR

`rbtdrc_srjcl_jupyter_connectivity` (the only crucible case that probes host → bottle HTTP through sentry's entry-port DNAT) reproducibly fails with `HTTP 000`. Docker's port-publish is delivering the packet on the **enclave** interface, but the sentry's DNAT rule scopes match to the **uplink** interface. The packet sails past the DNAT, hits the unused local INPUT chain in sentry, and the curl times out. This is a latent regression introduced in `₢BBABC` and partially-addressed (default-route half) in `₢BBABE`; the port-publish half was never closed.

Verified independently on **linux Docker engine 28.x** and **macOS Docker Desktop 28.x**, both against branch tip `28d1411f`. The macOS reproducer revealed an additional second-order blocker in `RBM-FORWARD` after the DNAT half is fixed — the fix must address both halves or macOS will still fail.

## Symptom

```
tt/rbtd-r.FixtureRun.srjcl.sh
  rbtdrc_srjcl_jupyter_running         PASSED   # ps aux confirms jupyter-lab alive on 0.0.0.0:8000
  rbtdrc_srjcl_jupyter_connectivity    FAILED   # curl http://localhost:7999/lab → HTTP 000
```

The `_running` case execs `bark ps aux` inside the bottle — it confirms `/usr/local/bin/jupyter-lab --ip=0.0.0.0 --port=8000 ...` is up, %CPU > 0, with the listener present (`ss -tlnp` inside bottle's network namespace shows `LISTEN ... 0.0.0.0:8000 jupyter-lab`).

The `_connectivity` case runs from the curia (theurge process, outside any container) with `curl --connect-timeout 5 --max-time 10`:

```
curl http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/lab    # 7999 for srjcl
```

curl reports `Connected to localhost ([::1] / 127.0.0.1) port 7999` — Docker's port-proxy accepts the TCP handshake — then `Operation timed out after 5006 milliseconds with 0 bytes received`. The TCP handshake succeeds against docker-proxy; no HTTP response ever returns; `%{http_code}` resolves to `000`.

The readiness-delay bump from 30s → 120s in `.rbk/srjcl/rbrn.env` (commit `28d1411f` on this branch) was an early hypothesis — it does not address the bug. The connectivity case fails irrespective of how long Jupyter has been up.

## Root Cause

### Network topology of a charged srjcl crucible

```
                   ┌─────────────┐
host:7999 ─────────│ docker-proxy│ -container-ip 10.242.2.2 -container-port 7999
                   └──────┬──────┘
                          │
                          ▼
                   sentry container
                   ─────────────────
                   eth0: 10.242.2.2   (enclave network: ${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}_enclave)
                   eth1: 172.19.0.2   (transit  network: ${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}_transit)

                          │ packets arrive on eth0 (alphabetical: enclave < transit)
                          ▼
                   PREROUTING nat
                   DNAT -i eth1 dpt:7999 → 10.242.2.3:8000   ← MATCHES 0 PACKETS (wrong -i)
                          │
                          ▼
                   INPUT (sentry-local) — nothing listening on 7999 → silent drop, curl times out
```

### The DNAT rule

`rbev-vessels/common-sentry-context/rbjs_sentry.sh:124-126`:

```sh
iptables -t nat -A PREROUTING -i ${RBJ_UPLINK_IF} -p tcp --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         -j DNAT --to-destination "${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}"
```

`RBJ_UPLINK_IF` is computed earlier in the same script as "the non-enclave interface" (i.e., NOT the one carrying `RBRN_ENCLAVE_SENTRY_IP`). In the observed run that resolves to `eth1`. The DNAT therefore matches incoming-on-eth1 traffic only.

### Docker's port-publish delivery

For a container attached to multiple bridge networks, Docker picks **one** network as the `-container-ip` target of the published-port DNAT (on linux) or proxy backend (on Docker Desktop). The selection rule is **alphabetical by network name**:

- `${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}_enclave`
- `${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}_transit`

`enclave` < `transit` lexicographically. Docker therefore targets the enclave network's container IP (10.242.2.2), and the packet arrives on the interface that carries that IP — `eth0` in the observed run. This was **explicitly confirmed empirically** in `₢BBABE` for the default-route case ("both linux Docker engine 28.x AND mac Docker Desktop 28.x ignore compose `services.<svc>.networks.<net>.priority` for default-route selection, picking the alphabetically-first attached network's gateway (= enclave)"). The same alphabetical preference governs port-publish target selection.

### Empirical evidence on macOS (this session)

PREROUTING per-interface LOG counters after one curl probe:

```
Chain PREROUTING (policy ACCEPT 4 packets, 240 bytes)
   4   240 LOG  6  --  eth0  *  ... tcp dpt:7999  LOG prefix "INETH0: "
   0     0 LOG  6  --  eth1  *  ... tcp dpt:7999  LOG prefix "INETH1: "
   0     0 LOG  6  --  lo    *  ... tcp dpt:7999  LOG prefix "INLO:   "
   4   240 LOG  6  --  *     *  ... tcp dpt:7999  LOG prefix "INOTHER:"
```

100% of inbound port-7999 traffic arrives on eth0. The shipped `-i eth1` DNAT matches 0%.

### Empirical evidence on linux (parallel investigation)

```
docker-proxy:  -container-ip 10.242.2.2 -container-port 7999    ← sentry eth0 (enclave)
sentry DNAT:   -i eth1 dpt:7999 -j DNAT --to 10.242.2.3:8000    ← eth1 (transit) only
                                                                  pkts=0 — never matches
```

Parallel investigation added a parallel `-i eth0` DNAT in the running linux sentry; the case immediately returned HTTP 200.

## Lineage — why this only surfaces now

**₢BBABC (`6d836814`)** — *renamed* sentry's `services.networks.default` → `transit`, declared `networks.transit` alongside `networks.enclave` at the top level. The pre-rename `default` name was load-bearing magic: Docker treated `default` as the primary network for both default-route assignment AND port-publish target selection. After the rename, both selections fall through to alphabetical ordering, which picks `enclave` first.

**₢BBABE (`ed898867`)** — diagnosed and fixed the **default-route half** of this fallout: sentry now takes ownership of its own default route after RBJp1 uplink-interface discovery (`ip route replace default via <computed-gw> dev ${RBJ_UPLINK_IF}`). The commit message explicitly names the mechanism: "both linux Docker engine 28.x AND mac Docker Desktop 28.x ignore compose `services.<svc>.networks.<net>.priority` for default-route selection, picking the alphabetically-first attached network's gateway (= enclave). Pre-rename the special `services.networks.default` path made compose pick the right gateway by accident; BBABC removed that luck."

The **port-publish half** has identical root cause and identical environmental sensitivity, but was not addressed by BBABE.

### Why this stayed hidden through BBABC, BBABD, BBABE…

No other crucible case exercises the host → bottle HTTP path through sentry's entry-port DNAT:

- `tadmor`, `moriah`, `ccyolo` — adversarial-network test suites that run attacks from inside the enclave (ifrit container), never from the curia through the published port.
- `pluml` — entry-mode-enabled like srjcl, but its connectivity case has not been exercised on a pristine-conjured image in any recent run (and will fail identically if it is — see "Sibling risk" below).
- The four-mode lifecycle fixtures (`conjure_lifecycle`, `bind_lifecycle`, `graft_lifecycle`, `kludge_lifecycle`) test build/publish/vouch flows, not runtime host-to-bottle networking.

`rbtdrc_srjcl_jupyter_connectivity` is the **only** case in the suite that goes curia → `localhost:<port>` → docker-proxy → sentry → DNAT → forward → bottle. Pristine AAP is the first context that puts srjcl through this path against a freshly-conjured Jupyter image with the post-BBABC network names.

## Second-order blocker observed on macOS — RBM-FORWARD scope

After flushing PREROUTING and installing an interface-agnostic DNAT in the live sentry (`-p tcp --dport 7999 -j DNAT --to 10.242.2.3:8000`, no `-i` constraint), DNAT fired correctly (10 packets matched on macOS) — but curl still timed out at HTTP 000.

The blocker is in `RBM-FORWARD`:

```
Chain RBM-FORWARD (1 references)
 pkts bytes target  prot  in    out   source         destination
   0     0 ACCEPT   tcp   eth1  eth0  0.0.0.0/0      10.242.2.3   dpt:8000
   ...
```

The shipped ACCEPT for entry-port traffic is `-i eth1 -o eth0`. Post-DNAT on macOS, the packet's path is `-i eth0 -o eth0` (hairpin — ingress and egress on the same interface, since Docker delivered to eth0 and the destination 10.242.2.3 is reached via eth0). The eth1-ingress ACCEPT cannot match; FORWARD policy is DROP; packet is dropped.

The linux parallel-DNAT experiment returned HTTP 200 from a parallel investigation. Either linux Docker takes a different post-DNAT path that bypasses FORWARD (some kernel hairpin / `accept_local` behaviour), or the linux experiment also modified RBM-FORWARD without it being noted in the diagnosis statement. Either way, **macOS empirically requires the FORWARD-half fix in addition to the DNAT-half fix.**

`rbjs_sentry.sh:128-130`:

```sh
iptables -A RBM-FORWARD -i ${RBJ_UPLINK_IF} -o ${RBJ_ENCLAVE_IF} -p tcp \
         -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" -j ACCEPT
```

Same `${RBJ_UPLINK_IF}` constraint, same problem. The FORWARD rule needs the same relaxation as the DNAT rule.

## Files Involved

| Path | Lines | Role |
|---|---|---|
| `rbev-vessels/common-sentry-context/rbjs_sentry.sh` | 124-126 | PREROUTING DNAT with `-i ${RBJ_UPLINK_IF}` constraint |
| `rbev-vessels/common-sentry-context/rbjs_sentry.sh` | 128-130 | RBM-FORWARD ACCEPT with `-i ${RBJ_UPLINK_IF} -o ${RBJ_ENCLAVE_IF}` constraint |
| `.rbk/rbob_compose.yml` | 36-37 | `ports: - "${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_WORKSTATION}"` — no `-container-ip` pin, Docker picks alphabetically |
| `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` | 1620 | The test case (correct as written; infrastructure beneath it is broken) |
| `.rbk/srjcl/rbrn.env` | 34 | `RBRN_BOTTLE_READINESS_DELAY_SEC=120` — bumped this run, does not address the bug |

## Severity / Scope

- **Real, reproducible, not a flake.** Confirmed independently on two host platforms against the same branch tip `28d1411f`.
- **Blocks AAP release-qualification.** srjcl is in the pristine gauntlet ladder. Cannot land AAP until srjcl-jupyter-connectivity is green on both platforms.
- **Sibling risk.** `pluml` has the same shape (`RBRN_ENTRY_MODE=enabled`, host → bottle HTTP via published port, same compose template, same sentry image). pluml's connectivity case will fail identically against a pristine-conjured PlantUML image. The repair must cover pluml automatically since both nameplates share the same sentry vessel.

## Fix Shape (proposed — not implemented)

The shape is dictated by BBABE's precedent: **sentry takes ownership of its own networking and does not trust Docker's network-selection heuristics.** That principle was applied to default-route ownership; the same principle applies here. The fix has two halves and both must land:

### Half 1 — DNAT scope

Drop `-i ${RBJ_UPLINK_IF}` from the PREROUTING DNAT rule. Sentry stops caring which interface Docker delivered the packet on; any inbound TCP to the workstation port is DNATed to the bottle.

```sh
iptables -t nat -A PREROUTING -p tcp --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         -j DNAT --to-destination "${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}"
```

Security implication to evaluate: removing the ingress-interface constraint means traffic originating *from inside the enclave* with dest port 7999 would also be DNATed. For srjcl/pluml use cases this is benign (bottles don't send TCP to sentry:7999). For adversarial bottles (tadmor) this is moot because `RBRN_ENTRY_MODE` is `disabled` and the DNAT rule isn't installed at all. If a tighter constraint is wanted, scope by `-s !${RBRN_ENCLAVE_BOTTLE_IP}` or similar — but the simpler interface-agnostic form is consistent with BBABE's "don't trust Docker" philosophy.

### Half 2 — FORWARD scope

Relax `RBM-FORWARD` ACCEPT for entry-port traffic the same way. The shipped rule's `-i eth1 -o eth0` assumes the post-DNAT path is uplink-ingress to enclave-egress; in practice the path is enclave-ingress to enclave-egress (hairpin). Drop both interface constraints:

```sh
iptables -A RBM-FORWARD -p tcp -d "${RBRN_ENCLAVE_BOTTLE_IP}" \
         --dport "${RBRN_ENTRY_PORT_ENCLAVE}" -j ACCEPT
```

Or, if interface scoping is desired for posture, scope by destination only and let any interface deliver — the destination IP (`RBRN_ENCLAVE_BOTTLE_IP`) is the load-bearing constraint, not the interface.

### Vessel rebuild

Both halves edit `rbev-vessels/common-sentry-context/rbjs_sentry.sh`, which is **baked into the sentry image at build time** (per RBS0:2416, referenced in the BBABE commit message). Repair workflow:

1. Edit `rbjs_sentry.sh` (both DNAT and FORWARD rules).
2. `tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether` (rebuild sentry vessel locally).
3. `tt/rbw-cKS.KludgeSentry.sh <nameplate>` to drive the new kludge hallmark into each affected nameplate (srjcl, pluml — and any other entry-mode-enabled nameplate).
4. Re-run the gauntlet on **both linux and macOS**. Verification gate is platform-pair green.

### Spec deltas (anticipated)

`RBSSS` step 2 already carries BBABE's "sentry takes ownership of routing" framing. Extend it to "sentry takes ownership of routing AND of entry-port DNAT scope, decoupled from Docker's choice of published-port delivery interface." `RBS0` quoins around the sentry container narrative may need a sentence on the port-publish-half mirror of the default-route fragility.

> The iptables forms above are superseded by the **Refined Fix Shape** in the Post-Diagnosis Research section below. The principle ("sentry takes ownership") survives unchanged; the concrete rules gain source-CIDR exclusion on the PREROUTING DNAT (classification predicate), conntrack DNAT-state matching on the RBM-FORWARD ACCEPT (authorization predicate), a multi-CIDR rejection guardrail, and a runtime source-IP probe to handle deployment contexts (notably Docker Desktop and Docker Engine) where the source IP visible to the target container is not documented.

## Post-Diagnosis Research

Follow-on web research was commissioned to verify the memo's working presumptions against primary sources and to determine the correct fix under cross-runtime constraints (the project must eventually run under both Docker and Podman, so the original "rely on Docker's alphabetical-first heuristic" is not acceptable as the fix). The research brief framed every presumption as a falsifiable proposition and required source-citable answers. Findings below refine the fix shape; the principle ("sentry owns its networking, don't trust the runtime's selection") is unchanged.

### Documented vs Observed (Docker / Compose)

| Presumption | Status | Notes |
|---|---|---|
| Docker selects port-publish target by alphabetical-first network name | **Partially source-traced** | The alphabetical ordering originates in `compose-go`'s `NetworksByPriority()` (`types/types.go`), which sorts service networks by descending `priority:`, falling through to **lexicographic-by-network-name on ties**. Docker Compose uses `NetworksByPriority()[0]` as `primaryNetworkKey` and sets the container's `NetworkMode` to that primary (`docker/compose/pkg/compose/create.go`). Docker Engine 28 separately documents lexicographic tie-break for default-gateway endpoint selection. **Not source-traced**: the connection from "Compose primary network" to "port-publish DNAT target." Docker's port-publishing docs expose no multi-network destination selector. The chain is `compose-go NetworksByPriority() → Compose primaryNetworkKey → container NetworkMode → Engine published-port DNAT against primary endpoint`, where only the last arrow is undocumented. |
| `default`-name magic confers port-publish/default-route primacy | **Not documented as primacy** | Compose docs describe `<project>_default` as the auto-created service-discovery network; no documented primacy rule for multi-network published-port target selection. The "magic" pre-BBABC was real in effect but undocumented in source. |
| Compose `services.<svc>.networks.<net>.priority:` is ignored for gateway and port-publish | **Confirmed by docs** | Docker's Compose service reference explicitly states `priority:` controls *connection order* and may determine which network gets a service-level `mac_address`. It explicitly does NOT select the default gateway or interface name. |
| Compose long-form `ports:` exposes no `container_ip` selector | **Confirmed** | Compose long-form `ports:` exposes `target`, `published`, `host_ip`, `protocol`, `app_protocol`, `mode`, `name`. No `container_ip` / `network` / `destination_network` field exists. |
| macOS Docker Desktop drops hairpinned post-DNAT | **Observed only** | Desktop runs Engine in a LinuxKit VM with userspace networking interposition; sufficient to treat as materially different but specific failure mode not traced to `accept_local`, vpnkit, or kernel hairpin semantics. |

**New finding the original memo missed**: Docker Compose 2.33.1+ introduces `gw_priority`, which DOES select the default gateway among multiple networks (highest value wins). This does *not* control port-publish target selection, but it provides a Compose-level alternative to BBABE's sentry-side `ip route replace default`. See "Compose-level hardening" below.

**Additional finding — conntrack DNAT virtual state.** `iptables-extensions(8)` documents `-m conntrack --ctstate DNAT` as a virtual state matching when "the original destination differs from the reply source" — i.e., the packet has been DNATed. This is a documented mainline netfilter primitive with stable semantics. For the RBM-FORWARD ACCEPT rule it provides a strictly stronger invariant than source-CIDR exclusion: post-DNAT flows carry conntrack state created by sentry's own PREROUTING DNAT, and that state cannot be forged by an attacker on either bridge. The refined fix below uses conntrack matching on FORWARD and source-CIDR exclusion on PREROUTING; the two compose because PREROUTING cannot match conntrack DNAT state (no DNAT has occurred yet at PREROUTING time — the classification predicate must run first).

### Podman Parity

- **Netavark has the same publish-target ambiguity — confirmed at source level.** Netavark's `src/firewall/nft.rs` creates a `NETAVARK-HOSTPORT-DNAT` chain with per-subnet DNAT chains. `setup_port_forward(...)` and `get_dnat_rules_for_addr_family(...)` build rules that jump from the top-level chain to a subnet-specific DNAT chain, then emit per-port DNAT-to-container-IP rules; rules are appended with `batch.add(rule)`. No documented "primary publish network" selector exists in netavark source. The original discussion-thread claim (`containers/podman#22746`: "first matching rule wins, order varies across recreations") is consistent with the source trace; the exact ordering mechanism for multi-network duplicate same-host-port rules remains implementation-incidental and was not source-confirmed. Engineering conclusion: netavark rule order is implementation-incidental, the same architectural problem as Docker.
- **No `default`-magic in Podman compose paths.** Neither `podman compose` (built-in) nor `podman-compose` (Python wrapper) documents Compose-style `default` primacy beyond ordinary Compose compatibility.
- **`priority:` doesn't help on Podman either.** No Podman/Netavark primary source treats `priority:` as a port-publish selector.
- **No portable runtime-level "primary network" field.** Neither CNI conflist, Netavark JSON, nor Compose YAML offers a field both runtimes honor as "this network is the published-port destination."
- **Rootless source-IP rewriting — load-bearing for the fix.** Podman's rootless networking path through rootlesskit rewrites incoming source IPs to a container-namespace address (commonly `10.0.2.100`). The `pasta` forwarder preserves the original source IP. `slirp4netns` preserves source IP but cannot be used with user-defined networks. This directly affects source-CIDR-based filtering: the source IP visible to sentry's iptables under rootless Podman may not be the host-originating client's IP. Any source-CIDR-exclusion fix must assert non-overlap with runtime rewrite ranges.
- **iptables vs nftables.** Netavark uses nftables or iptables as host-side firewall drivers. Sentry's rules run inside sentry's own network namespace, not on the host, so they are not directly tied to Netavark's choice. The sentry image must carry a working `iptables` frontend backed by kernel netfilter compatibility, or migrate to nftables for future-proofing.

### Source-Rewrite Address Visibility (Cross-Runtime Table)

The source IP visible to sentry's iptables when an external client connects to the published port varies by runtime/proxy combination. Only one concrete rewrite address is documented (`10.0.2.100` on Podman rootlesskit); for Docker Engine and Docker Desktop, the source IP visible to the container is not documented. This table is the basis for the multi-CIDR guardrail in the Refined Fix Shape:

| Runtime / proxy | Source rewrite? | Specific address / range | Confidence |
|---|---|---|---|
| Docker Engine Linux, bridge NAT | Not source-confirmed in docs | Not documented | Docker documents firewall NAT/PAT publishing but not observed source IP. |
| Docker Engine Linux, `docker-proxy` | Likely new-TCP-connection source, exact source not documented | Not documented | `--userland-proxy=true` participates in IPv6-host-to-IPv4-container cases. |
| Docker Desktop macOS | Yes — proxy/backend path | Not documented | Desktop backend listens on host port and forwards into the LinuxKit VM. |
| Docker Desktop Windows WSL2 | Yes — proxy/backend path | Not documented | Same Desktop caveat. |
| Docker Desktop Windows Hyper-V | Yes — proxy/backend path | Not documented | Same Desktop caveat. |
| Podman rootful Netavark nftables | No documented app-level rewrite | Not documented | Source-traced as DNAT to container IP; observed source not documented. |
| Podman rootful CNI legacy | Not source-confirmed | Not documented | CNI backend deprecated; netavark is current default. |
| Podman rootless Netavark + pasta | No — preserves original source IP | Original source IP | Documented in `podman-pod-create(1)`. |
| Podman rootless + slirp4netns | No — preserves source IP; **cannot be used with user-defined networks** | Original source IP | Documented; topology blocker if user-defined networks are required. |
| Podman rootless + rootlesskit | **Yes** | **Usually `10.0.2.100`** | Documented; default for rootless on user-defined networks. |
| Netavark firewalld same-host localhost | Special same-host limitation | IPv4 localhost path only; IPv6 localhost forwarding "not possible" per docs | Documented limitation. |

The table is **not complete enough to enumerate every Docker source range** because Docker's primary docs do not state them. The guardrail therefore combines static CIDR rejection (the known cases) with a runtime probe (for the undocumented cases). See Refined Fix Shape.

### Portability Matrix (Researcher's Evaluation)

Legend: **Works** = architecturally sound; **Caveat** = likely viable with runtime-specific constraints; **No** = incompatible; **Unknown** = source-backed conclusion not available.

| Alternative | Docker Engine Linux | Docker Desktop | Podman rootful Netavark | Podman rootful CNI | Podman rootless Netavark+pasta | Podman rootless Netavark+slirp4netns |
|---|---|---|---|---|---|---|
| A. One-network publisher sidecar | Works | Works | Works | Caveat | Caveat | Caveat |
| B. Sentry self-managed veth | Caveat | Caveat | Caveat | Caveat | No / severe | No / severe |
| C. `network_mode: host` + host iptables | Works | No / caveat | Works | Works | No | No |
| D. Source-CIDR-exclusion in sentry iptables | Works | Caveat | Works | Caveat | Caveat | Caveat |
| E. Conntrack-state-based rules | Caveat | Unknown | Caveat | Caveat | Unknown | Unknown |
| F. Runtime-level primary-publish declaration | No known field | No known field | No known field | No known field | No known field | No known field |

The researcher's primary recommendation is **D** (source-CIDR exclusion) with explicit guardrails. The secondary recommendation is **A** (one-network publisher sidecar) for topology-level certainty if the source-CIDR invariant is too fragile across deployment contexts.

### Refined Fix Shape

The research validates the in-place fix path (alternative D) and refines it further: the RBM-FORWARD rule should match on conntrack DNAT state rather than source-CIDR, since post-DNAT flows carry stronger semantic evidence than source-IP. The PREROUTING DNAT rule itself must still classify on source-CIDR (no conntrack state exists yet at PREROUTING). The "drop `-i ${RBJ_UPLINK_IF}`" proposal earlier in this memo is replaced by:

```sh
# Classification — only workstation-facing ingress (not enclave-originated)
# is DNATed to the bottle. Source-CIDR is the classification predicate.
iptables -t nat -A PREROUTING -p tcp \
         --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         ! -s "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
         -j DNAT --to-destination "${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}"

# Authorization — only flows actually DNATed by sentry's own PREROUTING rule
# may forward to the bottle. Conntrack DNAT-state is the authorization
# predicate. Strictly stronger than source-CIDR exclusion at FORWARD time:
# conntrack state is created by sentry's own NAT and cannot be forged by an
# attacker on either bridge.
iptables -A RBM-FORWARD -p tcp \
         -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" \
         -m conntrack --ctstate DNAT \
         -j ACCEPT
```

The PREROUTING rule replaces interface matching (`-i ${RBJ_UPLINK_IF}`) with source-CIDR exclusion; the interface match was over-specification using interface name as a proxy for "from outside the enclave." The RBM-FORWARD rule replaces interface matching with conntrack-state matching; this is the stronger invariant. Neither rule depends on which interface the runtime chose for port-publish delivery.

**Optional belt-and-suspenders FORWARD form** — adds the source-CIDR predicate to conntrack matching:

```sh
iptables -A RBM-FORWARD -p tcp \
         -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" \
         ! -s "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
         -m conntrack --ctstate DNAT \
         -j ACCEPT
```

The conjunction adds nothing in the normal case (conntrack DNAT state implies sentry's PREROUTING already classified the source as non-enclave), but defends against a hypothetical future bug in the PREROUTING rule. The default is the conntrack-only form; the belt-and-suspenders form should be adopted only after empirical confirmation that no runtime/proxy rewrites legitimate external traffic into a source address that overlaps `${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}`.

**Required guardrail — runtime source-rewrite overlap rejection.** Sentry's startup must refuse to install rules if the enclave CIDR overlaps any known runtime/proxy source-rewrite range. The static rejection list (illustrative bash; the implementation should match project discipline):

```sh
# Refuse enclave CIDRs that overlap runtime/proxy source-rewrite ranges.
# Without this guard, source-CIDR-exclusion on PREROUTING would silently
# misclassify legitimate rewritten external traffic.
reject_overlap() {
  local cidr="$1" forbidden="$2"
  python3 - "${cidr}" "${forbidden}" <<'PY'
import ipaddress, sys
a = ipaddress.ip_network(sys.argv[1], strict=False)
b = ipaddress.ip_network(sys.argv[2], strict=False)
sys.exit(0 if a.overlaps(b) else 1)
PY
}

for forbidden in \
  "10.0.2.100/32" \
  "10.0.2.0/24" \
  "127.0.0.0/8"
do
  if reject_overlap "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" "${forbidden}"; then
    buc_die "Enclave CIDR overlaps runtime/proxy source-rewrite range ${forbidden}"
  fi
done
```

Rationale for the list:
- `10.0.2.100/32` — Podman rootlesskit's documented rewrite address.
- `10.0.2.0/24` — defensive expansion (Podman docs say `10.0.2.100` is "usually" the rewrite address, not the only possibility).
- `127.0.0.0/8` — defensive rejection for same-host localhost forwarding paths; netavark-firewalld documents same-host connections requiring IPv4 localhost on that path.

The list is not exhaustive. Docker Engine and Docker Desktop do not document the exact source IP visible to the target container for published-port traffic; the static rejection must therefore be paired with a runtime probe.

**Required runtime probe — first-time observation of source IP.** Because Docker Engine and Docker Desktop do not document published-port source addresses visible to the target container, sentry should log the source IP of the first matching inbound SYN until empirical confirmation is in hand for each deployment context:

```sh
# Diagnostic probe: log source IPs of inbound published-port SYNs.
# Install during onboarding / first-run; remove after empirical confirmation.
iptables -t raw -I PREROUTING 1 -p tcp \
         --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         -j LOG --log-prefix "RBJ_PUBLISH_SRC: "
```

Pass condition: observed source IP NOT in `${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}`.
Fail condition: observed source IP IN `${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}`. Mitigation: change the enclave CIDR to a non-overlapping range; do not special-case the rule.

**IPv6 — explicit design choice required.** The proposed fix is IPv4-only. If IPv6 published-port ingress is intended, mirror the rules with `ip6tables` (or nftables) using IPv6 versions of the enclave prefix and bottle IP. Note Docker's documented behavior: when the bridge is IPv4-only and `--userland-proxy=true` (the default), host IPv6 addresses can map to the container's IPv4 address via the userland proxy; the source IP visible to the container in that path is not documented. netavark-firewalld documents IPv6 localhost forwarding as "not possible" with the firewalld driver due to kernel limitations. The current recipe-bottle entry-port architecture is IPv4-only; introducing IPv6 published-port ingress is a future-work item, out of scope for this fix.

**Compose-level hardening (optional, conditional on Compose ≥ 2.33.1).** Add `gw_priority` to sentry's network declarations to make default-gateway selection explicit at the Compose level:

```yaml
services:
  sentry:
    networks:
      auplink:
        gw_priority: 100
      enclave:
        gw_priority: 0
```

`gw_priority` does NOT affect port-publish target selection (no documented selector exists). This addition only deduplicates with BBABE's sentry-side default-route override — a Compose-level expression of the same intent. Whether to retain BBABE's iptables-side override after adding `gw_priority` is a defense-in-depth question, not a correctness one. The iptables refined fix above remains necessary regardless.

### Refined Fix — Per-Runtime Coverage

Per-runtime evaluation of the refined fix (source-CIDR PREROUTING + conntrack RBM-FORWARD + multi-CIDR guardrail + runtime probe):

| Runtime | Source-CIDR PREROUTING | Conntrack RBM-FORWARD | Overall refined fix |
|---|---|---|---|
| Docker Engine Linux | Works; source IP not fully documented | Works (sentry's NAT creates state) | **Works with runtime probe** |
| Docker Desktop macOS | Works under proxy/backend path; exact source not documented | Works | **Works with runtime probe** |
| Docker Desktop Windows WSL2 | Same Desktop caveat | Works | **Works with runtime probe** |
| Docker Desktop Windows Hyper-V | Same Desktop caveat | Works | **Works with runtime probe** |
| Podman rootful Netavark nftables | Works unless source overlaps enclave | Works if sentry iptables/nft compat works | **Works with iptables/nft validation** |
| Podman rootful CNI legacy | Source trace not closed; CNI deprecated | Works | **Works with probe** |
| Podman rootless Netavark + pasta | Works; source preserved | Works | **Works** |
| Podman rootless + slirp4netns | Works; source preserved, but slirp4netns cannot be used with user-defined networks | Works | **Topology blocker if user-defined networks are required** |
| Podman rootless + rootlesskit | Works only if enclave avoids `10.0.2.100/10.0.2.0/24` | Works | **Works with static CIDR rejection** |

### Open Empirical Questions

Status after follow-up research; remaining open items below:

1. **Docker exact selector source path** — **Partially resolved.** Alphabetical behavior is source-traced through compose-go's `NetworksByPriority()` (lexicographic tie-break) and Docker Compose's `primaryNetworkKey` derivation. The connection from "Compose primary network" to "port-publish DNAT target" remains empirical; the runtime source-IP probe (in Refined Fix Shape) provides ongoing empirical validation rather than one-time confirmation.
2. **Docker Desktop hairpin failure mechanism** — Still open. Not blocking; the refined fix is independent of which interface Docker Desktop chooses to deliver on.
3. **Podman Netavark current rule order** — **Partially resolved at source level.** Netavark emits per-network DNAT rules via shared chains; the exact ordering mechanism for multi-network duplicate same-host-port rules is implementation-incidental. An empirical reproduction on current Podman 5.x / Netavark 1.x is no longer load-bearing for the fix (conntrack-based RBM-FORWARD is netavark-rule-order-agnostic), but would close the source-traced-but-not-empirically-confirmed gap.
4. **Rootless Podman source-IP under user-defined networks** — **Documented** as rewriting to `10.0.2.100` per Podman docs. The startup guardrail rejects this and the surrounding `/24`. An empirical confirmation on Podman 5.x rootless against sentry remains a cross-runtime acceptance gate.
5. **iptables vs nftables availability inside sentry under Podman** — Still open. Run `iptables -V` and `iptables-save` inside sentry under Podman Netavark; if iptables compatibility is missing or restricted, port sentry's rule installer to nftables. Affects the refined fix's portability claim for Podman rootful Netavark.
6. **Docker Engine / Desktop published-port source-IP visibility** — **Newly surfaced; not documented by Docker.** The runtime source-IP probe (in Refined Fix Shape) is the standing mitigation. Confirm observed source IPs for each deployment context (Linux Engine, Desktop macOS, Desktop Windows WSL2, Desktop Windows Hyper-V) before declaring the cross-platform fix complete.
7. **IPv6 published-port behavior** — Out of scope for the current IPv4-only entry-port architecture; flagged for future-work attention if IPv6 ingress is added. Userland-proxy IPv6-on-IPv4 path source-IP visibility is also undocumented.
8. **Post-restart published-port reachability** — Sentry's own rules are restart-stable, but netavark host-side port-forward state has had teardown/reload drift in past releases. Verify that pristine-charge → quench → re-charge preserves expected reachability on both Docker and Podman.

## Verification Gate

**Both platforms must pass before declaring the fix landed.** This is non-negotiable based on macOS-side empirical evidence:

- linux: `rbtdrc_srjcl_jupyter_connectivity` green via full pristine gauntlet.
- macOS: `rbtdrc_srjcl_jupyter_connectivity` green via full pristine gauntlet.

A fix that only validates on linux (per the parallel investigation's experiment, which got HTTP 200 from the DNAT-half alone) is incomplete — macOS hairpin behaviour means the FORWARD half is also necessary.

## Crucible State (at memo-writing time)

- macOS branch `pym-₢A_AAP` (this host): srjcl crucible is **still charged**. PREROUTING has been mutated in the running container — original `-i eth1` DNAT was flushed and replaced with an interface-agnostic DNAT for investigation. Forwarding still blocked by RBM-FORWARD. Quench with `tt/rbw-cQ.Quench.srjcl.sh` when no longer needed for probing.
- linux branch `cerebro-A_AAP` (parallel investigator's host): srjcl crucible reported still charged with original broken iptables (the eth0-parallel-DNAT experiment was rolled back? — diagnosis statement says "rollback was not run", so the parallel `-i eth0` DNAT may still be in place there).

Both crucibles are non-canonical with respect to the shipped sentry image (post-experiment iptables state). Do not use either for any further verification of the broken-state baseline; charge a fresh crucible if such a baseline is needed.

## References

- `rbev-vessels/common-sentry-context/rbjs_sentry.sh:124-130` — DNAT and FORWARD rules to repair
- `.rbk/rbob_compose.yml:32-37` — sentry networks declaration and port-publish directive
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs:1597-1646` — `_jupyter_running` and `_jupyter_connectivity` case definitions
- Commit `6d836814` (₢BBABC) — `default → transit` rename; introduces the latent regression
- Commit `ed898867` (₢BBABE) — default-route half fix; precedent for sentry-side don't-trust-Docker discipline
- Commit `28d1411f` (₢A_AAP) — branch tip both platforms tested against; readiness-delay bump (unrelated to root cause)

### Research sources (Post-Diagnosis Research section)

- Docker Compose service reference (`priority:`, `gw_priority`, long-form `ports:` fields) — https://docs.docker.com/reference/compose-file/services/
- Docker Compose networking overview — https://docs.docker.com/compose/how-tos/networking/
- Docker port publishing and mapping — https://docs.docker.com/engine/network/port-publishing/
- Docker Desktop networking — https://docs.docker.com/desktop/features/networking/
- Docker host network driver — https://docs.docker.com/engine/network/drivers/host/
- Moby v28.0.0 `daemon/container_operations.go` — port binding sandbox option and network iteration — https://raw.githubusercontent.com/moby/moby/v28.0.0/daemon/container_operations.go
- Podman pod create reference (rootless networking, source-IP rewriting, pasta/slirp4netns) — https://docs.podman.io/en/stable/markdown/podman-pod-create.1.html
- Podman discussion #22746 — DNAT destination when using multiple bridges and publishing ports — https://github.com/containers/podman/discussions/22746
- netavark-firewalld(7) man page — host-side firewall interaction — https://www.mankier.com/7/netavark-firewalld

### Follow-up research sources (source-level traces and refined-fix building blocks)

- `iptables-extensions(8)` — `-m conntrack --ctstate DNAT` virtual state definition — https://man7.org/linux/man-pages/man8/iptables-extensions.8.html
- compose-go `types/types.go` — `NetworksByPriority()` (priority + lexicographic tie-break) — https://github.com/compose-spec/compose-go/blob/master/types/types.go
- Docker Compose `pkg/compose/create.go` — `primaryNetworkKey` derivation and `NetworkMode` assignment — https://github.com/docker/compose/blob/main/pkg/compose/create.go
- Docker Engine 28 release notes — multi-network ordering disclaimer, `gw-priority`, endpoint interface-name label — https://github.com/moby/moby/discussions/49497
- Moby issue #48868 — gateway-endpoint ordering rationale (priority, docker_gwbridge, non-internal, dual-stack, lexicographic) — https://github.com/moby/moby/issues/48868
- Docker Engine 28 release notes (gateway selection) — https://docs.docker.com/engine/release-notes/28/
- Netavark `src/firewall/nft.rs` — `NETAVARK-HOSTPORT-DNAT` chain, `setup_port_forward`, `get_dnat_rules_for_addr_family` — https://raw.githubusercontent.com/containers/netavark/main/src/firewall/nft.rs
- Netavark project README — scope of firewall/NAT/port-forward responsibilities — https://github.com/containers/netavark
- Netavark release notes — port-forward rule removal and firewall reload behavior history — https://raw.githubusercontent.com/containers/netavark/main/RELEASE_NOTES.md
- netavark-firewalld(7) source (current) — same-host localhost forwarding limitation, IPv6 localhost forwarding "not possible" — https://github.com/containers/netavark/blob/main/docs/netavark-firewalld.7.md
- podman-network(1) — Netavark default, CNI deprecated — https://docs.podman.io/en/stable/markdown/podman-network.1.html
