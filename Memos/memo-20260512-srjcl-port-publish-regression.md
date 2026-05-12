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

> The iptables forms above were superseded across two rounds of post-diagnosis research, and may themselves be superseded by the topology-level reframing in **"Topology-Level Reframing: Publisher on Pentacle"** below. Round 3 (in Refined Fix Shape, below) drops source-CIDR exclusion at PREROUTING as structurally broken in the bridge-gateway-source-rewrite case, retains conntrack DNAT-state matching at RBM-FORWARD as the authorization predicate, and demotes the runtime source-IP probe to a diagnostic-only role. The pentacle reframing then asks whether sentry should be the publisher at all — if not, the entire entry-port iptables block (PREROUTING DNAT + RBM-FORWARD ACCEPT + POSTROUTING MASQUERADE) becomes deletable.

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
| Source IP at sentry's PREROUTING for hairpinned host-published-port traffic is the enclave bridge gateway IP (e.g., `${ENCLAVE_BASE}.1`) | **Strongly plausible, not formally documented** | Docker docs describe bridge masquerading and `docker-proxy`'s userspace forwarding role (default `--userland-proxy=true`); a `docker/docs#17312` issue clarifies that with userland-proxy enabled, hairpin paths route through the userland proxy. Empirical community evidence (`moby/libnetwork#1994`) shows host-originated traffic appearing at the container with source equal to the docker bridge gateway IP. No formal "source IP seen by target container" contract is published. The bridge gateway is inside the bridge subnet by configuration (defaults to `.1` in both Docker and Podman; both runtimes allow `--gateway` override). This makes source-CIDR exclusion at PREROUTING **structurally broken**: any enclave CIDR contains its own gateway, so the gateway IP is always in `${ENCLAVE_CIDR}` by topological necessity. Renumbering does not help. |

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

**Round 3 simplification.** A third round of research established that source-CIDR exclusion at PREROUTING is structurally broken in the bridge-gateway-source-rewrite case (see the new row in the Documented vs Observed table above), not merely fragile. The Round 2 fix proposed source-CIDR exclusion as the classification predicate at PREROUTING; that predicate cannot work when the source IP of hairpinned published-port traffic is the enclave bridge gateway, because the gateway lies inside any enclave CIDR by topological necessity. Renumbering produces a new gateway at `.1` of the new CIDR. The classification predicate must change.

Round 3's resolution: **drop source-IP filtering at PREROUTING entirely.** Classify by destination port only; authorize via conntrack DNAT-state at RBM-FORWARD. The destination port is the stable classifier; the conntrack DNAT state is the stable authorization predicate.

```sh
# Classification: ANY inbound traffic to the workstation port is DNATed
# to the bottle. No source-IP filter — source IP at PREROUTING is not a
# stable trust label across the bridge-gateway, docker-proxy, Desktop
# backend, and rootlesskit paths.
iptables -t nat -A PREROUTING -p tcp \
         --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         -j DNAT --to-destination "${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}"

# Authorization: only flows actually DNATed by sentry's PREROUTING may
# forward to the bottle. Conntrack DNAT-state cannot be forged by an
# attacker on either bridge.
iptables -A RBM-FORWARD -p tcp \
         -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" \
         -m conntrack --ctstate DNAT \
         -j ACCEPT
```

**What this drops, relative to Round 2:**
- The `! -s ${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}` clause from PREROUTING (structurally broken).
- The multi-CIDR rejection guardrail (no longer necessary — when PREROUTING does not source-filter, no overlap matters; rootlesskit's `10.0.2.100` rewrite, `127.0.0.0/8` localhost forwarding, etc., all just pass through and get DNATed correctly).
- The "belt-and-suspenders" form combining source-CIDR with conntrack (the source-CIDR predicate cannot survive the bridge-gateway case, so the conjunction's failure mode is the same as the prior fix's).

**What this retains:**
- The conntrack DNAT-state RBM-FORWARD rule (the actual authorization gate).
- The runtime source-IP probe, demoted from "required guardrail" to "diagnostic-only" — still useful for understanding what the runtime is actually doing, but no longer load-bearing.

**Why source-CIDR was a mistake, not a defense (Round 3 finding).** The Round 2 fix's source-CIDR exclusion was trying to encode "from outside the enclave" using source IP as the trust label. In bridge NAT and proxy hairpin paths, source IP is not a stable trust label: it may be the bridge gateway, Docker-proxy, Docker Desktop backend, rootlesskit synthetic address, or original client, depending on runtime. The stable classifier is the destination port; the stable authorization predicate is conntrack DNAT state. The defensive value the source-CIDR predicate appeared to provide was illusory — an enclave attacker already has direct enclave access to `${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}`, so denying their workstation-port-to-bottle DNAT path adds no capability gate.

**Diagnostic probe (optional, demoted from required).** Useful for understanding the actual source-IP visibility in a deployment context:

```sh
# Diagnostic probe: log source IPs of inbound workstation-port SYNs.
iptables -t raw -I PREROUTING 1 -p tcp \
         --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         -j LOG --log-prefix "RBJ_PUBLISH_SRC: "
```

The probe is no longer a gate (the fix doesn't depend on the source IP), but the observation is still informative for cross-runtime mental model.

**IPv6 — design choice unchanged.** The fix is IPv4-only. If IPv6 published-port ingress is intended, mirror the rules with `ip6tables` (or nftables) using IPv6 versions of the bottle IP. Docker's IPv6-to-IPv4 userland-proxy mapping with `--userland-proxy=true` has undocumented source-IP visibility; netavark-firewalld documents IPv6 localhost forwarding as "not possible" with the firewalld driver. The current recipe-bottle entry-port architecture is IPv4-only.

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

`gw_priority` does NOT affect port-publish target selection. This addition only deduplicates with BBABE's sentry-side default-route override.

**Multi-port discipline.** If sentry publishes multiple workstation-facing ports, each port needs its own exact DNAT rule. Do not broaden `--dport` to a port range unless every port in the range maps intentionally.

**Sentry-self-loop note.** A local process inside sentry connecting to `${RBRN_ENTRY_PORT_WORKSTATION}` may hit the OUTPUT chain rather than PREROUTING; the rule above does not affect OUTPUT. If sentry-local clients to its own workstation port matter, add an explicit OUTPUT DNAT rule, but only after testing.

### Refined Fix — Per-Runtime Coverage (Round 3)

| Runtime | Status | Notes |
|---|---|---|
| Docker Engine Linux | **Works** | No source-IP dependence; works under both `--userland-proxy=true` and `--userland-proxy=false`. |
| Docker Desktop macOS | **Works** | No source-IP dependence; backend-proxy source rewrite no longer matters. |
| Docker Desktop Windows WSL2 | **Works** | Same as Desktop macOS. |
| Docker Desktop Windows Hyper-V | **Works** | Same as Desktop macOS. |
| Podman rootful Netavark nftables | **Works** | Confirm `iptables`/`nft` frontend availability inside sentry; if missing, port rule installer to nftables. |
| Podman rootful CNI legacy | **Works** | CNI deprecated upstream but rules are runtime-agnostic from sentry's namespace. |
| Podman rootless Netavark + pasta | **Works** | Source preserved; no source filter, so it doesn't matter. |
| Podman rootless + slirp4netns | **Topology blocker** | slirp4netns cannot be used with user-defined networks; topology is incompatible regardless of fix shape. |
| Podman rootless + rootlesskit | **Works** | rootlesskit's `10.0.2.100` source rewrite no longer relevant — no source filter. |

The Round 2 portability matrix is superseded by this Round 3 version; the source-CIDR-driven "Caveat" cells collapse to "Works" because the source-IP question is no longer load-bearing.

**Fallback if Round 3's Candidate A fails empirically:** the publisher-sidecar topology (Round 2's "Alternative A"). See the topology-level reframing below for a project-native form of this fallback that uses the existing pentacle container as the publisher.

### Open Empirical Questions

Status after three rounds of research; remaining open items below:

1. **Docker exact selector source path** — **Partially resolved.** Alphabetical behavior is source-traced through compose-go's `NetworksByPriority()` (lexicographic tie-break) and Docker Compose's `primaryNetworkKey` derivation. The connection from "Compose primary network" to "port-publish DNAT target" remains empirical. After Round 3 the question is moot for the bottle-bound path (no source-IP dependence in the refined fix).
2. **Docker Desktop hairpin failure mechanism** — Still open. Not blocking; the refined fix is independent of which interface Docker Desktop delivers on, and independent of any source rewrite the Desktop backend applies.
3. **Podman Netavark current rule order** — **Partially resolved at source level.** Netavark emits per-network DNAT rules via shared chains; the exact ordering mechanism for multi-network duplicate same-host-port rules is implementation-incidental. After Round 3 the question is moot for the bottle-bound path (conntrack-based RBM-FORWARD is netavark-rule-order-agnostic, and PREROUTING no longer source-filters).
4. **Rootless Podman source-IP under user-defined networks** — **Documented** as rewriting to `10.0.2.100` per Podman docs. After Round 3 this is no longer a fix-blocker (no source filter at PREROUTING), but remains an interesting empirical data point for the diagnostic probe.
5. **iptables vs nftables availability inside sentry under Podman** — Still open. Run `iptables -V` and `iptables-save` inside sentry under Podman Netavark; if iptables compatibility is missing or restricted, port sentry's rule installer to nftables. This is now the primary remaining portability question for the Round 3 fix.
6. **Docker Engine / Desktop published-port source-IP visibility** — **Strongly plausible as the bridge gateway IP** per Round 3's combined source-trace + community-evidence analysis, but not formally documented. After Round 3 this is no longer load-bearing; the diagnostic probe captures the actual value if curiosity demands.
7. **IPv6 published-port behavior** — Out of scope for the current IPv4-only entry-port architecture; flagged for future-work attention if IPv6 ingress is added.
8. **Post-restart published-port reachability** — Sentry's own rules are restart-stable, but runtime-side host port-forward state has had teardown/reload drift in past releases. Verify that pristine-charge → quench → re-charge preserves expected reachability on both Docker and Podman.
9. **Pentacle-publisher topology validity** — Newly surfaced. The Round 3 fix is in-place; the topology reframing below proposes moving the `ports:` directive from sentry to pentacle, eliminating the disambiguation problem at the architectural level rather than mitigating it in iptables rules. Empirical verification pending — see Topology-Level Reframing below.

## Topology-Level Reframing: Publisher on Pentacle

After three rounds of iptables-focused research, the deeper architectural question surfaced: should sentry be the publisher at all? The pentacle is single-network-attached by design (enclave only), and the bottle shares the pentacle's namespace via `network_mode: service:pentacle`. Moving the `ports:` directive from sentry to pentacle eliminates the multi-network publish-target disambiguation problem at its source — no fix-via-iptables is needed because the runtime has only one network to publish to.

**Status: architecturally strong hypothesis, empirically pending.** This section captures the reframe so future readers can evaluate it; the Verification Gate below applies to whichever fix lands.

### The architectural observation

The current topology (per `.rbk/rbob_compose.yml`):

```
sentry   : networks { auplink, enclave }   <- dual-attached (egress gatekeeper)
pentacle : networks { enclave }            <- single-attached (namespace host)
bottle   : network_mode: service:pentacle  <- shares pentacle's namespace
ports:   : declared on sentry              <- publisher on dual-attached container
```

The publisher (sentry) is dual-attached. Docker/Podman must therefore choose one of its networks for port-publish target. Compose offers no portable knob to control that choice. Every iptables fix the prior research rounds proposed is downstream of this single architectural decision: putting the gatekeeper on the publisher.

The pentacle is single-attached precisely because it is the right shape for the publisher role. It has exactly one network to target; there is no ambiguity. The bottle, via shared namespace, is the actual listener on `0.0.0.0:${RBRN_ENTRY_PORT_ENCLAVE}` — Docker's port-publish to pentacle's namespace reaches the bottle's listener directly with no sentry indirection.

The reframed topology:

```
sentry   : networks { auplink, enclave }   <- dual-attached (egress gatekeeper, unchanged)
pentacle : networks { enclave }            <- single-attached + publisher
bottle   : network_mode: service:pentacle  <- shares pentacle's namespace + actual listener
ports:   : declared on pentacle            <- publisher on single-attached container
```

### What becomes deletable

The entire entry-port iptables block in `rbev-vessels/common-sentry-context/rbjs_sentry.sh` disappears:

- **RBJp2 PREROUTING DNAT** (`rbjs_sentry.sh:125`) — no longer needed. Docker port-publish delivers directly to bottle via pentacle namespace.
- **RBM-FORWARD ACCEPT for the entry port** (`rbjs_sentry.sh:129`) — no longer needed. Sentry is not on the inbound path.
- **POSTROUTING MASQUERADE for the entry port** (if present in sentry's startup) — no longer needed. No DNAT happens, no return-path masquerade is required.

The Round 3 Candidate A iptables form (above) becomes moot for the bottle-bound path — there is no DNAT anywhere in sentry's namespace.

### What sentry retains

Sentry's egress-gatekeeper role is unchanged:

- The bottle's default route is still through sentry (via pentacle startup script's route assignment at `rbev-vessels/common-sentry-context/rbjp_pentacle.sh:47`).
- DNS is still through sentry.
- Outbound iptables enforcement (RBJp3 and RBJp4) is untouched.
- The dual-attachment (uplink + enclave) remains correct for the egress role: sentry needs the uplink to forward bottle-initiated outbound traffic, and the enclave to receive it from the bottle.

The "trust-boundary container with two interfaces" concept survives. What changes is the narrative: the dual attachment is for *outbound* traffic the bottle initiates, not for *inbound* traffic the world initiates toward the bottle.

### Honest narrative correction

The prior spec language ("all traffic flows through sentry") was inaccurate even before this reframing. Pre-regression, sentry's entry-port DNAT was a thin shim with no inspection or policy enforcement — packets were rewritten and forwarded, full stop. Removing the shim does not lose any policy enforcement that existed; it just makes the narrative match what was actually happening: **sentry enforces policy on outbound traffic; inbound published-port traffic reaches the bottle's listener via the enclave bridge.**

### Security envelope checks

- **`net-dnat-entry-reflection` invariant** ("enclave-internal sources MUST NOT reach sentry's entry port"): preserved trivially — sentry has no entry port. The crucible test case passes vacuously after the reframe.
- **Ifrit on enclave reaching the bottle directly**: already permitted under the current architecture (`Tools/rbk/rbtd/src/rbida_sorties.rs:2025-2042` documents this as expected — "Connected is expected if a service is listening — that's the bottle's own port"). No new attack surface.
- **External clients reaching the bottle**: same effective surface — host could already reach the bottle's logical service via `sentry:7999`; post-reframe it reaches the same service via the direct path.

No security envelope shrinks under the reframe.

### Spec deltas (post-empirical-verification only)

To be applied after the experiment proves the reframe works on both platforms:

- `RBS0:2429` ("ensuring security policies are enforced from the first packet") — accurate today only for outbound. Rescope to "outbound traffic enforcement from the first packet."
- `RBSSR` step 2 ("dual network attachment") — still correct for sentry; add a clarifying sentence that the dual attachment is for the uplink-control role, not for inbound port publish.
- New entry-mode wording: "Entry-port traffic is published on the pentacle, not the sentry. The pentacle's single-network attachment is the load-bearing simplification — it eliminates the multi-network publish-target disambiguation problem that no portable Compose/runtime API can solve."

Spec edits do NOT block the experiment. Memo first, code experiment second, spec language third.

### Experiment protocol

1. **Move the `ports:` directive** in `.rbk/rbob_compose.yml` from the `sentry` service block to the `pentacle` service block, AND change the mapping to the asymmetric form `"${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_ENCLAVE}"`. The prior `7999:7999` mapping relied on sentry's PREROUTING DNAT to translate port 7999 to bottle's 8000 (srjcl) or 8080 (pluml); without DNAT in the path, the publish itself must perform the port translation. For srjcl this becomes `"7999:8000"`; for pluml `"7999:8080"` (per nameplate `RBRN_ENTRY_PORT_ENCLAVE`).
2. **Strip the entry-port iptables block** from `rbev-vessels/common-sentry-context/rbjs_sentry.sh`: the PREROUTING DNAT at line 125, the RBM-FORWARD ACCEPT at line 129, and any POSTROUTING MASQUERADE associated with the entry port. Leave the egress iptables (RBJp3, RBJp4) untouched.
3. **Lift `ip_forward` enablement out of the entry-port block.** The current entry-port block enables `ip_forward=1` as a side-effect (at `rbjs_sentry.sh:121-122`). For srjcl (allowlist) and ccyolo (global), `ip_forward` is re-enabled later (line ~148), so deletion is functionally a no-op. For pluml (access_mode=disabled), deletion leaves `ip_forward=0`. Lift the enablement to an unconditional earlier line in `rbjs_sentry.sh` so kernel-forward state is independent of which iptables installation block runs. Pluml's outbound remains iptables-blocked by the access-mode disabled path; lifting only normalizes the kernel state.
4. **(Recommended, same change-set) Add `NET_ADMIN` to bottle's `cap_drop:`** in `.rbk/rbob_compose.yml` (the bottle service block, currently `cap_drop: [NET_RAW]` at line ~89). Per Round 4's security envelope refinement.
5. **Kludge the sentry vessel** locally (`tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether`).
6. **Refresh hallmarks on EVERY nameplate exercised in the test iteration.** The sentry vessel is shared across srjcl, pluml, tadmor, moriah, and ccyolo; if any of these nameplates run in the iteration with an unrefreshed hallmark, that fixture exercises the OLD sentry image and its pass/fail tells you nothing about the new image. For the three-fixture iteration (srjcl + pluml + tadmor): refresh all three (`tt/rbw-cKS.KludgeSentry.sh srjcl`, same for pluml, same for tadmor). For the full pristine gauntlet: refresh every nameplate that uses the sentry vessel.
7. **Run the test suite.** For an iteration: sequential three-fixture run (srjcl + pluml + tadmor). For canonical verification: `tt/rbw-tP.QualifyPristine.sh` on both platforms.
8. **Verification gate**: `rbtdrc_srjcl_jupyter_connectivity` green on both platforms via full pristine gauntlet. The three-fixture iteration is an early-signal cycle; canonical pass requires the full gauntlet.

### Empirical observations to record during the experiment

Round 4 specified four observations that should be captured during the experiment, both to confirm the topology behaves as predicted and to firm up cross-runtime understanding for future Podman work:

**A. Guest-publishing rejection (confirm defensive engine behavior).** After charge, inspect the bottle's container configuration:

```sh
docker inspect <bottle-container> --format '{{.HostConfig.NetworkMode}} {{json .HostConfig.PortBindings}}'
```

Expected: bottle's `NetworkMode` references the pentacle's namespace; bottle's `PortBindings` is null/empty; pentacle's `PortBindings` has the expected host port. If bottle has any `PortBindings`, the experiment has a configuration error.

**B. Source-IP visibility at bottle's listener.** While the connectivity test is running:

```sh
# Inside bottle (or pentacle namespace; same thing):
tcpdump -ni any "tcp port ${RBRN_ENTRY_PORT_ENCLAVE}" -c 5
```

Record the observed peer source IP per runtime. Round 4's prediction: Docker Desktop and Docker Engine will show runtime-dependent values (likely bridge gateway or proxy address); Podman rootful Netavark will show a DNAT path source; rootless paths follow the rootlesskit/pasta/slirp4netns split. Not a pass/fail; data collection for future entry-mode use cases.

**C. Pentacle-restart-alone behavior (confirm or refute lifecycle coupling).** After the connectivity test passes, exercise the docker/compose#10263 hazard:

```sh
# Restart pentacle alone (NOT recommended in production; this is a probe):
docker compose -p ${RBRN_MONIKER} restart pentacle
sleep 5
curl -v http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/lab

# Then restart bottle:
docker compose -p ${RBRN_MONIKER} restart bottle
sleep 5
curl -v http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/lab
```

If the first curl fails and the second succeeds, the lifecycle-coupling hazard is real in our deployment context, and rbob_charge / rbob_quench guards may be needed to prevent pentacle-alone-restart.

**D. Startup window (port advertised before listener bound).** During pristine-conjure, observe whether external curl sees connection-refused or timeout during the window between pentacle becoming healthy and bottle's listener binding. The 120s readiness delay in srjcl's nameplate is designed to absorb this. If curl fails BEFORE the delay completes, the window is exposed; the readiness delay handles it. If curl succeeds before the delay completes, the underlying timing has improved post-reframe (because there is no DNAT-shim in the way) — could justify reducing the delay back toward 30s.

### Round 4 cross-runtime findings

A web research round (Round 4) investigated the topology's cross-runtime robustness. Verdict: **architecturally valid and recommended, with two operationalizable caveats: lifecycle coupling and listener-source-IP non-portability.**

**`ports:`-on-namespace-host validity matrix:**

| Runtime / Compose path | Verdict | Notes |
|---|---|---|
| Docker Compose v2 + Docker Engine | **Supported** | `network_mode: service:{name}` is documented in the Compose spec; `ports:` on the namespace owner is the correct location. |
| Docker Compose v2, `ports:` on namespace guest | **Engine-level rejection** | Docker/Moby returns "conflicting options: port publishing and the container type network mode." This is a defensive property — a misconfiguration is caught at engine init rather than silently doing the wrong thing. |
| Docker Desktop macOS / Windows | **Supported with same semantics** | Compose model unchanged; Desktop changes port-forward plumbing only. |
| `podman compose` (built-in wrapper) | **Provider-dependent** | Thin wrapper around `docker-compose` or `podman-compose`; Compose semantics depend on which provider is installed. |
| `podman-compose` (Python) | **Supported-with-caveat** | Project claims Compose Spec implementation; no primary-source guarantee for this exact `network_mode: service:X` + publisher-host pattern. Empirical confirmation required. |
| Podman native `--network container:X` | **Concept supported; publish belongs to namespace owner** | Equivalent semantic — port-publish on the original namespace owner. |
| Podman native pod (`podman pod create`) | **Strongly supported** | Podman docs explicitly: "You must not publish ports of containers in the pod individually, but only by the pod itself." Cleanest semantic for Podman-native deployment. |

**Listener source-IP visibility (key caveat — the topology fixes publish-target determinism, NOT source-IP fidelity):**

| Runtime / proxy | Expected listener source IP |
|---|---|
| Docker Engine Linux, `--userland-proxy=true` | Runtime-dependent; may be proxy/bridge-gateway. Undocumented. |
| Docker Engine Linux, `--userland-proxy=false` | Likely netfilter path; preservation possible for remote clients, hairpin may still show host/bridge-derived source. Undocumented. |
| Docker Desktop macOS / Windows | Backend/VM proxy path; undocumented. Must probe. |
| Podman rootful Netavark | DNAT path; exact host-hairpin source undocumented. |
| Podman rootless + rootlesskit | Usually `10.0.2.100` (documented). |
| Podman rootless + pasta | Original source IP preserved (documented). |
| Podman rootless + slirp4netns | Source preserved, but topology compatibility with user-defined networks is weak. |

If original client IP capture ever matters for an entry-mode use case, add an application-layer proxy upstream of bottle that forwards PROXY protocol or `X-Forwarded-For`. The topology does not solve original-client IP fidelity. The current `rbtdrc_srjcl_jupyter_connectivity` case does not need original-client IP; this is forward guidance for future entry-mode use cases.

**Lifecycle coupling caveat.** If pentacle is recreated alone, bottle's joined network namespace can become stale or broken. Docker Compose issue `docker/compose#10263` reproduces this class of failure with `network_mode: service:pause`: restarting the namespace owner broke the guest until the guest was restarted too. This is empirical evidence rather than spec, but it operationalizes as a working rule:

- **Pentacle and bottle restart as a unit.** Never restart pentacle alone.
- For tooling and operator runbooks: `docker compose restart pentacle bottle` (or `podman compose ...`). Project-side, this may need a guard in rbob_charge / rbob_quench paths to ensure pentacle and bottle are always charged/quenched together.
- For Podman-native deployments, prefer a Podman pod — its infra container coordinates shared-namespace lifecycle and the failure mode is harder to trigger.

**Startup window.** The host port can be advertised before bottle binds its listener. External clients during the window between pentacle healthy and bottle bound observe connection refused or timeout. Same underlying behavior the prior architecture had (sentry's PREROUTING was up before bottle's listener); the prior `RBRN_BOTTLE_READINESS_DELAY_SEC` 30→120s bump on srjcl in ₢A_AAP was for exactly this window. The reframe does not change the underlying timing; it relocates where the "port published but listener not ready" gap shows up. The readiness-delay machinery in rbob_charge remains the correct mitigation.

**Security envelope refinement.** Containers sharing a network namespace share interfaces, IP addresses, port space, routes, iptables/nft rules, and conntrack state for that namespace. This is the same model Kubernetes pods use intentionally. Two project-specific implications:

1. **Drop `CAP_NET_ADMIN` from bottle.** Without it, bottle cannot modify rules pentacle installed in the shared namespace. With it, bottle could alter port-publish forwarding. The current `cap_drop: [NET_RAW]` on bottle (per `.rbk/rbob_compose.yml:89`) is insufficient for this topology; extend to also drop `NET_ADMIN`. This is research-recommended security hygiene; not required to validate the topology empirically, but should land in the same change-set.
2. **Avoid extra port-binding processes in bottle's image.** Two containers in the same namespace see the same port space; an unintended bind in bottle would conflict with the workstation port.

The namespace-sharing model intentionally removes network isolation between pentacle and bottle. They are pod-like peers by design — and were already pod-like peers in the existing architecture via `network_mode: service:pentacle`. Round 4's contribution is identifying the right capability hygiene (`NET_ADMIN` drop).

**Pod-equivalent posture.** Compose's `network_mode: service:X` is a single-host approximation of Kubernetes pod / Podman pod semantics. For Docker-first deployment, the Compose form is the more portable expression. For Podman-native deployment, a Podman pod is semantically cleaner and offers stronger lifecycle coordination via the infra container. Project path: keep the Compose form as the primary YAML (works on Docker and Podman via Compose); evaluate a Podman-pod variant later as an optimization for Podman-native operators if `podman compose` semantics turn out to be too thin.

### Hypothesis posture

The reframe is **research-validated (Round 4) with two operationalizable caveats** (lifecycle coupling, source-IP non-portability), and remains empirically pending. The experiment is cheap to run and easy to revert (one compose-yml edit, one iptables-block deletion, one optional `cap_drop` addition for bottle). If the experiment confirms cross-platform green, the iptables-focused Round 3 fix becomes a fallback rather than the primary recommendation, and the Podman-pod variant joins the future-work list. If the experiment fails, the iptables fix remains in-place as the working solution and the reframe joins the future-work list.

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

### Round 3 research sources (bridge-gateway-source-rewrite finding and Candidate A justification)

- Docker docs issue #17312 — `--userland-proxy` impact on hairpin paths — https://github.com/docker/docs/issues/17312
- moby/libnetwork issue #1994 — empirical evidence of source-IP rewrite to bridge gateway IP — https://github.com/moby/libnetwork/issues/1994
- Docker `docker network create` reference — `--gateway` selection and defaults — https://docs.docker.com/reference/cli/docker/network/create/
- Docker bridge network driver — bridge gateway placement and subnet conventions — https://docs.docker.com/engine/network/drivers/bridge/
- Podman `podman-network-create(1)` — Netavark `managed` mode masquerading and DNAT for published ports; `--gateway` override — https://docs.podman.io/en/stable/markdown/podman-network-create.1.html

### Round 4 research sources (pentacle-publisher topology validity and cross-runtime caveats)

- Compose Spec (`network_mode: service:{name}` definition; `ports:` location rules) — https://github.com/compose-spec/compose-spec/blob/master/spec.md
- Docker Compose issue #10263 — restart behavior when using `network_mode: service:X`; empirical evidence of namespace-guest breakage on namespace-owner restart — https://github.com/docker/compose/issues/10263
- Docker Compose source `pkg/compose/create.go` — `HostConfig.NetworkMode` and `PortBindings` assembly path (engine-level rejection of `ports:` on namespace guest) — https://raw.githubusercontent.com/docker/compose/main/pkg/compose/create.go
- `podman-compose(1)` — Podman's built-in compose wrapper; provider-dependent semantics — https://docs.podman.io/en/latest/markdown/podman-compose.1.html
- containers/podman-compose (Python implementation) — https://github.com/containers/podman-compose
- `podman-run(1)` — `--network container:X` semantics and namespace-sharing security implications — https://docs.podman.io/en/latest/markdown/podman-run.1.html
- Kubernetes networking concepts — pod network namespace model as semantic comparison — https://kubernetes.io/docs/concepts/services-networking/
