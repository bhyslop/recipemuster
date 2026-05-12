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

> The iptables forms above are superseded by the **Refined Fix Shape** in the Post-Diagnosis Research section below. The principle ("sentry takes ownership") survives unchanged; the concrete rules gain explicit source-CIDR exclusion and a rootless-overlap guardrail.

## Post-Diagnosis Research

Follow-on web research was commissioned to verify the memo's working presumptions against primary sources and to determine the correct fix under cross-runtime constraints (the project must eventually run under both Docker and Podman, so the original "rely on Docker's alphabetical-first heuristic" is not acceptable as the fix). The research brief framed every presumption as a falsifiable proposition and required source-citable answers. Findings below refine the fix shape; the principle ("sentry owns its networking, don't trust the runtime's selection") is unchanged.

### Documented vs Observed (Docker / Compose)

| Presumption | Status | Notes |
|---|---|---|
| Docker selects port-publish target by alphabetical-first network name | **Observed only — not documented** | Moby v28.0.0 source builds port bindings as a sandbox-level libnetwork option and iterates `ctr.NetworkSettings.Networks` (a Go map). No user-facing ordering contract found. Observed "alphabetical" behavior could be Compose serialization, daemon map iteration, libnetwork endpoint order, or Desktop proxy behavior. Not a contract. |
| `default`-name magic confers port-publish/default-route primacy | **Not documented as primacy** | Compose docs describe `<project>_default` as the auto-created service-discovery network; no documented primacy rule for multi-network published-port target selection. The "magic" pre-BBABC was real in effect but undocumented in source. |
| Compose `services.<svc>.networks.<net>.priority:` is ignored for gateway and port-publish | **Confirmed by docs** | Docker's Compose service reference explicitly states `priority:` controls *connection order* and may determine which network gets a service-level `mac_address`. It explicitly does NOT select the default gateway or interface name. |
| Compose long-form `ports:` exposes no `container_ip` selector | **Confirmed** | Compose long-form `ports:` exposes `target`, `published`, `host_ip`, `protocol`, `app_protocol`, `mode`, `name`. No `container_ip` / `network` / `destination_network` field exists. |
| macOS Docker Desktop drops hairpinned post-DNAT | **Observed only** | Desktop runs Engine in a LinuxKit VM with userspace networking interposition; sufficient to treat as materially different but specific failure mode not traced to `accept_local`, vpnkit, or kernel hairpin semantics. |

**New finding the original memo missed**: Docker Compose 2.33.1+ introduces `gw_priority`, which DOES select the default gateway among multiple networks (highest value wins). This does *not* control port-publish target selection, but it provides a Compose-level alternative to BBABE's sentry-side `ip route replace default`. See "Compose-level hardening" below.

### Podman Parity

- **Netavark has the same publish-target ambiguity.** Podman discussion `containers/podman#22746` documents that on multi-bridge containers with published ports, Netavark adds DNAT rules for every (port, network) pair, and the first matching rule wins. Rule order was observed to vary across recreations. No documented selector exists. The architectural problem is shared, not Docker-specific.
- **No `default`-magic in Podman compose paths.** Neither `podman compose` (built-in) nor `podman-compose` (Python wrapper) documents Compose-style `default` primacy beyond ordinary Compose compatibility.
- **`priority:` doesn't help on Podman either.** No Podman/Netavark primary source treats `priority:` as a port-publish selector.
- **No portable runtime-level "primary network" field.** Neither CNI conflist, Netavark JSON, nor Compose YAML offers a field both runtimes honor as "this network is the published-port destination."
- **Rootless source-IP rewriting — load-bearing for the fix.** Podman's rootless networking path through rootlesskit rewrites incoming source IPs to a container-namespace address (commonly `10.0.2.100`). The `pasta` forwarder preserves the original source IP. `slirp4netns` preserves source IP but cannot be used with user-defined networks. This directly affects source-CIDR-based filtering: the source IP visible to sentry's iptables under rootless Podman may not be the host-originating client's IP. Any source-CIDR-exclusion fix must assert non-overlap with runtime rewrite ranges.
- **iptables vs nftables.** Netavark uses nftables or iptables as host-side firewall drivers. Sentry's rules run inside sentry's own network namespace, not on the host, so they are not directly tied to Netavark's choice. The sentry image must carry a working `iptables` frontend backed by kernel netfilter compatibility, or migrate to nftables for future-proofing.

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

The research validates the in-place fix path (alternative D) and refines the iptables form. The "drop `-i ${RBJ_UPLINK_IF}`" proposal earlier in this memo is replaced by:

```sh
# DNAT: entry-port traffic from anywhere not in the enclave goes to the bottle
iptables -t nat -A PREROUTING -p tcp \
         --dport "${RBRN_ENTRY_PORT_WORKSTATION}" \
         ! -s "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
         -j DNAT --to-destination "${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}"

# FORWARD: entry-port-bound traffic to bottle, from outside enclave, allowed
iptables -A RBM-FORWARD -p tcp \
         -d "${RBRN_ENCLAVE_BOTTLE_IP}" --dport "${RBRN_ENTRY_PORT_ENCLAVE}" \
         ! -s "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" \
         -j ACCEPT
```

This form replaces interface matching (`-i ${RBJ_UPLINK_IF}`) with source-CIDR exclusion. The interface match was over-specification using interface name as a proxy for "from outside the enclave"; the source-CIDR form expresses that invariant directly in terms of project-controlled nameplate values, with no dependence on the runtime's interface-selection choice.

**Required guardrail — rootless overlap discipline.** When the project runs under rootless Podman, `${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}` must not overlap rootlesskit's source-rewrite address (`10.0.2.100` per Podman docs) or any other runtime/proxy source range. Sentry's startup should refuse to install rules under an overlapping enclave CIDR:

```sh
# Refuse enclave CIDRs that overlap known runtime source-rewrite ranges.
# Without this guard, source-CIDR-exclusion would silently filter
# legitimate rewritten external traffic on rootless Podman.
case "${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}" in
  10.0.2.0/24)
    buc_die "Enclave CIDR overlaps rootlesskit source-rewrite range"
    ;;
esac
```

The check should be generalized to all known runtime source-rewrite ranges discovered in deployment; the `10.0.2.0/24` case is the documented one but is not exhaustive.

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

`gw_priority` does NOT affect port-publish target selection (no documented selector exists). This addition only deduplicates with BBABE's sentry-side default-route override — providing a Compose-level expression of the same intent. Whether to retain BBABE's iptables-side override after adding `gw_priority` is a defense-in-depth question, not a correctness one. The iptables source-CIDR fix above remains necessary regardless.

### Open Empirical Questions

The research flagged five empirical tests that should be run before declaring the fix understood (not before applying it — the fix is independent of runtime selection by construction):

1. **Docker exact selector source path.** The "alphabetical-first" claim is empirical only. Test with three networks, randomized creation order, randomized Compose YAML order; inspect `docker inspect` plus `iptables-save` to determine the actual selection rule.
2. **Docker Desktop hairpin failure mechanism.** Need packet capture inside sentry and inside the LinuxKit VM to trace the DROP. Not blocking — the source-CIDR fix removes the dependence — but a known unknown.
3. **Podman Netavark current rule order.** The `#22746` discussion describes nondeterministic first-rule-wins. A current Podman 5.x / Netavark 1.x reproduction should confirm.
4. **Rootless Podman source-IP under user-defined networks.** Confirm sentry sees rootlesskit's rewritten source as `10.0.2.100` (or another runtime-internal address) when accessed via published port under rootless Podman. This is the key test for the source-CIDR-exclusion fix's robustness.
5. **iptables vs nftables availability inside sentry under Podman.** Test `iptables -V` and `iptables-save` inside sentry under both Docker and Podman Netavark to confirm the rule installer's frontend assumptions hold.

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
