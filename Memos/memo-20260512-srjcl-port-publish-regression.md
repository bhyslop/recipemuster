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
