# Sentry Entry-Port Repair — Anchor for Next Attempt

**Date:** 2026-05-12
**Pace:** ₢A_AAb (sentry-entry-port-dnat-forward-rescope)
**Status:** Action anchor after topology-reframe failure and decisive scry-empirical evidence.
**Companion memo:** `Memos/memo-20260512-srjcl-port-publish-regression.md` — full diagnostic history, four rounds of web research, two prior repair attempts and their analyses. Read for lineage; this memo is the forward action plan.

## TL;DR

Revert the topology reframe. Restore sentry-as-publisher in `.rbk/rbob_compose.yml` (symmetric `7999:7999` port mapping). Re-introduce the entry-port iptables block in `rbev-vessels/common-sentry-context/rbjs_sentry.sh`. Fix shape (current hypothesis, per the empirical correction history below):

- PREROUTING classification by destination port + **per-IP source exclusion of the two enclave-internal containers** (sentry's enclave IP and bottle's enclave IP) via RETURN-short-circuit pattern. NOT whole-CIDR exclusion — the latter collides with Linux Docker Engine's host-as-bridge-peer model (host appears with bridge gateway IP `.1`, which IS inside the enclave CIDR by construction).
- POSTROUTING MASQUERADE on the entry-port path.
- RBM-FORWARD ACCEPT via conntrack DNAT-state.
- Baseline ESTABLISHED,RELATED rule confirmed in RBM-FORWARD.
- `rp_filter=2` (loose) on sentry's network namespace when `RBRN_ENTRY_MODE=enabled`.

Keep three things from the failed reframe: the `ip_forward` lift (independent hygiene), the `cap_drop: [NET_ADMIN]` addition on bottle (defense in depth), the `tt/rbw-cs.Scry.sh` tabtarget (new diagnostic capability).

**Empirical status:**
- macOS Docker Desktop 28.x: three-fixture parallel **green under PRIOR fix shape (whole-CIDR exclusion)** — srjcl 3/3, pluml 5/5, tadmor 54/54 including the decisive `direct_sentry_probe`, `net_dnat_entry_reflection`, `net_srcip_spoof` cases. NOT yet re-verified under the current per-IP-exclusion hypothesis.
- Linux Docker Engine 28.x: **whole-CIDR exclusion FAILS** — cerebro diagnostic confirmed sentry's PREROUTING DNAT fires 0 times because the host's source IP (`10.242.2.1`, the enclave bridge gateway) is inside the enclave CIDR and the `! -s ENCLAVE_CIDR` predicate rejects every legitimate host SYN. Per-IP exclusion hypothesis is the proposed fix; not yet tested.

**Canonical declaration gate**: three-fixture green AND pristine gauntlet green on BOTH platforms under the per-IP-exclusion form. Currently no platform is canonically green under the current hypothesis; both need re-verification.

## Decisive empirical evidence

A `tt/rbw-cs.Scry.sh srjcl` capture during the topology-reframe failure produced the data that resolved every prior hypothesis. The captured packet path on macOS Docker Desktop:

- Source IP at bottle's listener: **`192.168.65.1`** (Docker Desktop's host-to-VM gateway). NOT the enclave bridge gateway. The prior "bridge gateway as source" hypothesis from the companion memo was wrong on Desktop.
- Bottle's SYN-ACK departs bottle's namespace correctly (destined for `192.168.65.1`).
- Sentry's RBM-FORWARD counters: **ALL ZERO**. The SYN-ACK never traversed sentry's iptables FORWARD chain.
- The SYN-ACK reappears on sentry's transit interface (eth1, "In" direction) with source rewritten from `10.242.2.3:8000` to `192.168.65.3:7999` — Docker Desktop's networking layer SNAT-reflected the post-DNAT return through sentry's transit network.
- Connection never completes; curl times out at HTTP 000.

The bottle was listening correctly (`_jupyter_running` passed). The asymmetric return path is the failure mechanism. Tadmor green (54/54) confirmed the sentry script changes preserved egress-gatekeeper posture; the failure is specific to inbound published-port traffic.

## Architectural conclusion

The original pre-regression design's POSTROUTING MASQUERADE on entry-port traffic was **structurally load-bearing, not decorative.** It made the bottle see the inbound connection as originating from sentry's enclave IP, which produced two crucial properties:

1. The bottle's reply destination is in-enclave (sentry's enclave IP), reached via the bottle's directly-connected route — no traversal through sentry's default-via-sentry route for the return.
2. Sentry's conntrack table holds the NAT entry from the inbound; the return packet matches REPLY direction, and the NAT is reversed cleanly before egress filtering sees the packet.

The topology reframe (publisher on pentacle) broke this because the bottle saw the original Docker Desktop source IP (`192.168.65.1`), which routes through bottle's default-via-sentry route into sentry's transit interface, where Docker Desktop's NAT reflection then loops it back into sentry hostilely. Sentry's iptables never see the post-DNAT reply as a normal forward operation (counters zero).

**Load-bearing requirement: sentry must be in the inbound path.** That's what makes conntrack + MASQUERADE keep the return path symmetric and inside our security envelope.

## What in the companion memo is now superseded

The companion memo's "Topology-Level Reframing: Publisher on Pentacle" section: **empirically refuted.** The pattern is valid in isolation (Round 4 confirmed), but incompatible with our egress-gate-via-default-route security model. Mark in the companion memo when convenient; not blocking.

The companion memo's "Refined Fix Shape" (Round 3) included **source-CIDR exclusion at PREROUTING** in *whole-CIDR* form. This was *more right* than this anchor memo's earlier "drop source-CIDR" reasoning (which the Ifrit `direct_sentry_probe` empirically falsified — confusing capability calculation with contractual invariant). Source-IP-based exclusion of enclave-internal containers IS the load-bearing predicate enforcing the `net-dnat-entry-reflection` invariant.

But Round 3's *whole-CIDR* form is itself empirically falsified on Linux Docker Engine (cerebro diagnostic): the host attaches to the enclave bridge as a peer with bridge gateway IP (`.1`), which is inside the enclave CIDR by Docker convention. Whole-CIDR exclusion rejects every legitimate host SYN. The current hypothesis (per-IP exclusion) replaces whole-CIDR with per-container-IP RETURN short-circuits.

Round 3 also missed two things the empirical experiment surfaced:

1. **MASQUERADE on the entry-port path was missing in Round 3's description.** Round 3's reasoning held that "conntrack DNAT-state alone authorizes the return" — true for the FORWARD authorization predicate, but MASQUERADE serves a different purpose (keeping the bottle's reply destination in-enclave so it routes cleanly back through sentry's conntrack). Without MASQUERADE, the reply destination is whatever the runtime put as the source IP, which on Docker Desktop is in hostile address space.
2. **`rp_filter=2` (loose mode) was not in Round 3's recipe.** Strict rp_filter (the kernel default and the project's pre-existing spec commitment in RBSAX) drops legitimate Docker-Desktop-delivered entry-port traffic at routing time, before iptables can act. Loose mode is required for the post-BBABC delivery topology. Discovered during empirical scry-based diagnosis; not anticipated by any prior research round.

Round 3's principled stance — interface-agnostic rules, conntrack DNAT-state for FORWARD authorization, no dependence on undocumented Docker heuristics — is retained. The two omissions are filled, and the CIDR-vs-per-IP choice is the latest empirical correction (still pending cross-platform validation under the per-IP form).

## The fix recipe (architectural; agent reads source for exact syntax)

In `rbev-vessels/common-sentry-context/rbjs_sentry.sh`, the entry-port iptables block has four clauses plus one kernel sysctl. Read the file for current variable names, function structure, and surrounding rule order; do not blind-copy snippets. All clauses below are empirically validated by the three-fixture parallel green on macOS Docker Desktop 28.x.

**PREROUTING — classification via per-IP source exclusion + DNAT (current hypothesis):**

The whole-CIDR exclusion form (`! -s ENCLAVE_CIDR`) was empirically falsified on Linux Docker Engine — the host attaches to the enclave bridge as a peer with `.1` IP, which is inside the enclave CIDR by Docker convention, so CIDR exclusion rejects every legitimate host SYN. The replacement is per-IP source exclusion of the two enclave-internal container IPs only, implemented as a RETURN short-circuit pattern (the standard iptables idiom for "block these specific sources, allow everything else"):

```
iptables -t nat -A PREROUTING -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} \
         -s ${RBRN_ENCLAVE_SENTRY_IP} -j RETURN
iptables -t nat -A PREROUTING -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} \
         -s ${RBRN_ENCLAVE_BOTTLE_IP} -j RETURN
iptables -t nat -A PREROUTING -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} \
         -j DNAT --to-destination ${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}
```

Rationale: the `net-dnat-entry-reflection` invariant says "enclave-internal sources MUST NOT reach sentry's entry port via DNAT reflection." The enclave-internal containers are exactly two (sentry and bottle; pentacle shares bottle's namespace, so they share the IP). Excluding those two specific IPs expresses the threat model directly. The bridge gateway (`.1` on linux), the Docker Desktop VM gateway (`192.168.65.1`), the rootlesskit synthetic source (`10.0.2.100`), and any external LAN client all pass the predicate naturally because none equals sentry or bottle IP.

Implementation note — single-rule alternative: `iptables ... ! -s SENTRY_IP ! -s BOTTLE_IP -j DNAT ...` may work but `iptables-extensions(8)` semantics of multiple `! -s` predicates in a single rule are uncertain (some references say only the last `-s` is honored). The RETURN short-circuit pattern above is the unambiguous iptables idiom and should be the implementation form unless empirically verified that the single-rule multi-`! -s` form works on all target runtimes.

Maintenance note: if future architecture adds a third enclave-internal container (e.g., a sidecar to the bottle), its IP must be added to the RETURN list. The whole-CIDR form didn't have this burden but was empirically broken; this is the trade-off.

No interface filter (`-i ...` removed — that was the BBABC-broken assumption). Destination NAT performs the port translation (`7999 → 8000` for srjcl, `7999 → 8080` for pluml).

**POSTROUTING MASQUERADE — return-path-symmetry preservation:**
match the entry-port path on the outgoing direction (`-d ${RBRN_ENCLAVE_BOTTLE_IP} --dport ${RBRN_ENTRY_PORT_ENCLAVE}`).

Rationale: makes the bottle see the inbound as originating from sentry's enclave IP, which routes the reply back through sentry's directly-connected enclave route (not through the default-via-sentry route into transit). This is the load-bearing clause the prior repair attempts missed; the scry-empirical evidence from the topology reframe confirmed asymmetric return-path was the failure mode that omitting MASQUERADE produced.

**RBM-FORWARD ACCEPT for inbound — authorization:**
match destination (`-d ${RBRN_ENCLAVE_BOTTLE_IP} --dport ${RBRN_ENTRY_PORT_ENCLAVE}`) AND conntrack DNAT-state (`-m conntrack --ctstate DNAT`).

Rationale: only flows that sentry's own PREROUTING DNAT created conntrack state for are authorized. The conntrack state cannot be forged by an attacker on either bridge.

**RBM-FORWARD ESTABLISHED,RELATED ACCEPT — return-path authorization:**
match by conntrack state (`-m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT`), positioned early in the chain so return traffic passes before any per-destination access-mode filtering. **Empirically verified present** in the current shipping `rbjs_sentry.sh` (the egress-mode blocks rely on it); no change needed in this implementation. Documented here because it's load-bearing for the return path; removal would break the architecture.

**Kernel sysctl — `rp_filter=2` (loose mode) when `RBRN_ENTRY_MODE=enabled`:**
inside the entry-mode block, set `/proc/sys/net/ipv4/conf/all/rp_filter` to 2. Remove any earlier strict-mode setter from the access-mode-enabled branch (it would overwrite this back to 1 and break Docker-Desktop-delivered traffic).

Rationale: post-BBABC, Docker delivers entry-port traffic on the enclave interface (alphabetically-first selection); the inbound source IP has its reverse path via the uplink/transit interface; strict rp_filter (mode 1, the kernel default and the project's pre-existing spec commitment in RBSAX) sees this as a spoofed packet and silently drops at routing time, before iptables can act. Loose mode (2) accepts "any route exists" as sufficient.

Security trade-off: kernel-layer spoof protection moves to iptables. The Ifrit `net_srcip_spoof` sortie empirically validates that the existing spoof patterns (spoof-as-sentry, spoof-as-allowed-cidr, spoof-as-loopback) remain blocked at iptables layer. One residual gap — spoof-as-arbitrary-external-routable-IP — is the subject of ₢A_AAc (see "Ifrit coverage limitation" below).

## The revert plan

### `.rbk/rbob_compose.yml`

- Move the `ports:` directive from the `pentacle` service back to the `sentry` service.
- Restore the symmetric mapping form `"${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_WORKSTATION}"` (e.g., `7999:7999`). The asymmetric form (`7999:8000`) was a consequence of the topology reframe (no DNAT in the path); with sentry's DNAT restored, the publish maps symmetrically and the DNAT does the port translation.
- **Retain** the `cap_drop: [NET_ADMIN]` addition on the `bottle` service if it was added during the failed reframe (it's defense-in-depth orthogonal to the topology choice). If it wasn't added, add it now — it costs nothing and hardens against a compromised bottle altering pentacle's rules.

### `rbev-vessels/common-sentry-context/rbjs_sentry.sh`

- Re-introduce the entry-port block (deleted during the reframe), in the form described in "The fix recipe" above. Read the file's current state for variable names, function structure, comment style.
- **Retain** the `ip_forward` lift from the failed reframe attempt: keep `echo 1 > /proc/sys/net/ipv4/ip_forward` at an unconditional early position (currently at RBJp2). Do NOT couple ip_forward enablement to the entry-port block again; that was a latent hygiene issue and the lift fixed it.

### Tabtargets

- **Retain** `tt/rbw-cs.Scry.sh` and the underlying `Tools/rbk/rboo_observe.sh` machinery introduced during the failed reframe. The scry tabtarget produced the decisive diagnostic data and is new institutional capability; it should outlive the specific investigation.

## Verification protocol

### Step 1 — apply the revert + fix

Edit the two files per the revert plan and fix recipe above.

### Step 2 — kludge sentry, refresh hallmarks

```
tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether
tt/rbw-cKS.KludgeSentry.sh srjcl
tt/rbw-cKS.KludgeSentry.sh pluml
tt/rbw-cKS.KludgeSentry.sh tadmor
```

The sentry vessel is shared across srjcl, pluml, tadmor, moriah, and ccyolo. Refresh every nameplate exercised in the iteration; per the companion memo, an unrefreshed nameplate runs against the OLD sentry image and its pass/fail validates nothing about the new image.

### Step 3 — scry verification before declaring success

After charging srjcl, run `tt/rbw-cs.Scry.sh srjcl` in one terminal and issue a single curl probe from another. Confirm the packet path:

- **Inbound:** SYN arrives at sentry's transit interface (or wherever Docker chose) with source `192.168.65.1` (or runtime equivalent). PREROUTING DNAT fires. POSTROUTING MASQUERADE fires. Packet arrives at bottle with source = sentry's enclave IP (e.g., `10.242.2.2`).
- **Listener:** bottle responds.
- **Return:** bottle's SYN-ACK destination = sentry's enclave IP (the MASQUERADE'd source). Routed via directly-connected enclave route to sentry's enclave interface. Sentry's RBM-FORWARD counters increment (specifically the ESTABLISHED,RELATED rule). Conntrack reverses the NAT. Packet leaves sentry via transit. curl receives HTTP 200.

This is the symmetric return path. If scry shows any of these steps missing — particularly if RBM-FORWARD counters remain zero or the bottle sees a non-enclave source — diagnosis before declaring landed.

### Step 4 — three-fixture iteration on this platform

Sequential `srjcl + pluml + tadmor`. Decisive case: `rbtdrc_srjcl_jupyter_connectivity`. Tadmor green confirms the egress-gatekeeper posture is preserved.

### Step 5 — coordinate with linux operator for cross-platform verification

The companion memo's verification gate (cross-platform pristine gauntlet green) stands. The fix is interface-agnostic and source-IP-agnostic by construction; it should work uniformly on Linux Docker Engine 28.x and macOS Docker Desktop 28.x. Confirm before declaring canonical landing.

### Step 6 — full pristine gauntlet for canonical pass

`tt/rbw-tP.QualifyPristine.sh` on both platforms. This is the canonical gate; the three-fixture iteration is the early-signal cycle.

## Empirical validation status

The fix shape has evolved through three empirical corrections. The current hypothesis (per-IP exclusion via RETURN short-circuit) has not yet been tested on any platform. Status table:

| Platform | Whole-CIDR exclusion (PRIOR fix shape) | Per-IP exclusion (current hypothesis) |
|---|---|---|
| macOS Docker Desktop 28.x | **Three-fixture green** — srjcl 3/3, pluml 5/5, tadmor 54/54 including `direct_sentry_probe`, `net_dnat_entry_reflection`, `net_srcip_spoof`. Pristine gauntlet not yet exercised. | **Not yet tested.** Re-verification needed under the new form; expected green by analysis (per-IP exclusion is a strict subset of whole-CIDR in terms of what it blocks, and macOS host source is `192.168.65.1` which is outside enclave CIDR and outside the per-IP block list, so it passes both). |
| Linux Docker Engine 28.x | **FAILED** — cerebro empirical diagnostic: sentry's PREROUTING DNAT fires 0 times because the host attaches to the enclave bridge as a peer with IP `.1` (inside enclave CIDR by Docker convention); `! -s ENCLAVE_CIDR` rejects every legitimate host SYN. | **Not yet tested.** Expected green by analysis: host source `.1` is neither sentry IP nor bottle IP, so per-IP exclusion allows it. |
| Podman rootful (netavark) | Not exercised | Not exercised. Plausible by analysis but no Podman cross-runtime testing has been done. |
| Podman rootless (rootlesskit / pasta / slirp4netns) | Not exercised | Not exercised. Rootlesskit's `10.0.2.100` source is outside both enclave CIDR and the per-IP block list, so per-IP exclusion would allow it; pasta preserves original source IP, also outside; slirp4netns can't be used with user-defined networks (topology incompatible). |

**Canonical declaration gate**: cross-platform three-fixture parallel green AND full pristine gauntlet green on BOTH macOS Docker Desktop AND Linux Docker Engine, under the per-IP-exclusion form. Currently NO platform meets the gate under the current hypothesis. The next experiment (per-IP exclusion implementation, then re-run three-fixture cross-platform) is what produces the empirical data that either confirms the hypothesis or surfaces another structural surprise.

## Ifrit coverage limitation (subject of ₢A_AAc)

The empirically-validated Ifrit spoof tests cover three patterns: spoof-as-sentry-IP, spoof-as-allowed-CIDR-IP, spoof-as-loopback. They do NOT cover **spoof-as-arbitrary-external-routable-IP** (e.g., enclave attacker spoofing Docker Desktop's host-to-VM gateway `192.168.65.1` as source).

Under the Option E-corrected architecture (source-CIDR exclusion + rp_filter=2 loose), this spoof variant is theoretically reachable: the claimed source is outside enclave CIDR (so source-CIDR exclusion allows it), the spoofed IP has a route in sentry's routing table (so loose rp_filter allows it), DNAT fires, packet reaches bottle. Strict rp_filter (mode 1) would have blocked this at kernel layer; the architecture trades kernel-layer protection for source-IP-flexibility on Docker Desktop delivery.

This isn't a regression against the project's current test gates — those tests all pass — but it IS a known limitation in the empirically-validated security envelope. Pace **₢A_AAc** (ifrit-sortie-enclave-spoof-external) is slated to add an Ifrit sortie that empirically verifies whether this attack vector is blocked by other iptables rules (RBM-INGRESS, the source-CIDR clause if it catches the spoof somehow, etc.) or requires an additional explicit mitigation. If the sortie reveals the attack succeeds, candidate mitigations include: (a) selectively re-implementing rp_filter-strict-for-enclave-interface at iptables raw table (`iptables -t raw -A PREROUTING -i ENCLAVE_IF ! -s ENCLAVE_CIDR -j DROP`); (b) adding an explicit "drop non-enclave-source on enclave interface" rule. Either preserves rp_filter loose for legitimate Docker Desktop delivery while restoring the spoof block.

## Lessons captured

The three-attempt arc plus the empirical correction, briefly (full lineage in the companion memo):

1. **Attempt 1 — compose rename `transit → auplink` (experiment, reverted).** Worked, exploited Docker's undocumented alphabetical-first network-name heuristic. Rejected as canonical for cross-runtime portability.

2. **Attempt 2 — source-CIDR exclusion at PREROUTING + FORWARD.** Stage 1 (with rename) green; stage 2 (no rename) red. Now-understood failure mechanism: **rp_filter=1 (strict, the kernel default)** dropped legitimate Docker-Desktop-delivered traffic at routing time before PREROUTING fired. The "missing MASQUERADE / missing ESTABLISHED,RELATED" hypothesis in earlier versions of this memo was wrong; the actual failure was kernel-layer rp_filter, discovered during scry-based diagnosis. The bridge-gateway-source-rewrite hypothesis (companion memo Round 3) was also wrong on Docker Desktop — the actual source is `192.168.65.1` (Desktop's host-to-VM gateway), outside enclave CIDR.

3. **Attempt 3 — topology reframe (publisher on pentacle).** Failed empirically. Root cause: bottle's default-via-sentry routing makes inbound published-port traffic produce asymmetric return paths through sentry's transit interface, where Docker Desktop's NAT reflection breaks the symmetry hostilely. Architecturally clean in isolation but fundamentally incompatible with our egress-gate-via-default-route security model.

4. **Option E (this memo's first proposal) — destination-port-only DNAT + MASQUERADE + conntrack FORWARD + rp_filter=2.** Failed at tadmor `rbtdrc_sortie_direct_sentry_probe`: an enclave attacker dialing sentry's entry port matched the dport-only DNAT, redirected to bottle, BREACH. The architect's "drop source-CIDR" reasoning ("enclave already has bottle access so DNAT'ing adds no capability") confused capability calculation with contractual invariant. The `net-dnat-entry-reflection` invariant is a contract, empirically tested; source-CIDR exclusion is the load-bearing predicate that enforces it.

5. **Option E corrected (whole-CIDR exclusion) — source-CIDR + MASQUERADE + conntrack FORWARD + ESTABLISHED,RELATED + rp_filter=2.** Empirically green on macOS three-fixture parallel (54/54 tadmor). Reached Linux cross-platform verification — **FAILED**. Cerebro diagnostic: the host attaches to the enclave bridge as a peer with IP `.1` (Docker's bridge-gateway convention), which is inside the enclave CIDR by construction; `! -s ENCLAVE_CIDR` rejects every legitimate host SYN. This was a third structural Docker fact not anticipated by any prior round.

6. **Option E corrected v2 (per-IP exclusion) — destination port + RETURN short-circuit for sentry/bottle IPs + DNAT + MASQUERADE + conntrack FORWARD + ESTABLISHED,RELATED + rp_filter=2.** Current hypothesis. Threat model expressed directly (the only enclave-internal sources are sentry and bottle; block exactly those, allow everything else). Cross-platform analysis suggests it works on all target runtimes; empirical confirmation pending.

The fix in this memo is **what pre-BBABC's design would have looked like if written defensively against the structural Docker facts we've now learned about**: the same DNAT + MASQUERADE + FORWARD-ACCEPT shape, but with classification by destination port + per-IP source exclusion (not interface, not whole-CIDR), authorization by conntrack state (not interface), and explicit kernel sysctl for rp_filter to accommodate the post-BBABC delivery interface. The original design's principle — sentry takes ownership of its security envelope — is preserved.

### Meta-lesson: layered defenses are load-bearing by default

The arc exposed a pattern. Architectural reasoning produced clean-looking fixes; each got empirically falsified by a structural Docker fact we hadn't anticipated. Five empirical surprises (counted now), each invalidating a "this is the clean shape" argument:

- Interface filter (`-i UPLINK_IF`) — broken by BBABC's alphabetical-first network selection. Original architecture relied on it via accident.
- Default-route ownership accident (compose `default` magic) — also broken by BBABC. Recovered explicitly by BBABE.
- The entire entry-port block (topology reframe) — failed at return-path asymmetry. Sentry's MASQUERADE was structurally load-bearing for symmetric return, not decorative.
- Source-CIDR exclusion (in destination-port-only Option E) — failed at the `direct_sentry_probe` Ifrit invariant. Architect's "drop source-CIDR because clean" was empirically wrong; source-CIDR was load-bearing.
- Whole-CIDR exclusion (in CIDR-form Option E corrected) — failed on Linux because the host is a peer on the enclave bridge with `.1` IP, which is inside the enclave CIDR by Docker convention. Architect's "CIDR exclusion is the obvious form of source-IP filter" was empirically wrong on Linux Engine.

Plus one defense that wasn't stripped but needed re-evaluation:

- rp_filter strict — empirically incompatible with the post-BBABC delivery topology. Relaxed to loose; iptables-layer enforcement compensates.

The lesson is now sharper than "load-bearing defenses." It's about **the structural facts of the framework**: every time architectural reasoning has produced a "clean" shape, an undocumented structural Docker fact has invalidated it. The Ifrit framework + cross-platform empirical validation are the discipline that catches this; architectural reasoning without empirical validation has failed every time in this work.

The pattern itself is the institutional value to capture: **future architecture changes touching network configuration must be empirically validated on Linux Docker Engine AND macOS Docker Desktop AND (eventually) Podman before declaring confidence.** Each platform exposes different structural facts. macOS Desktop's VM-proxy structure hides Linux's host-bridge-peer fact. Linux's host-bridge-peer fact is hidden by Desktop's VM. Only cross-platform testing surfaces both. Probably Podman will surface a third class of structural fact when tested; expect more surprises, design the iteration discipline for them.

## Spec alignment findings (input for ₢A_AAd)

This section is the contract for pace **₢A_AAd** (rbs-spec-deltas-post-sentry-fix). It documents specific gaps in the RBS* spec corpus that this work has surfaced, and the targeted spec deltas to apply post-canonical-pass. Each delta names a specific file + region + intent; ₢A_AAd's docket explicitly forbids prose invention beyond this list.

### The seven gaps

1. **Entry-port DNAT predicate enumeration is too thin in RBSSS.** RBSSS:32 reads "Entry port DNAT if enabled" — one line. The actual empirically-validated implementation requires four cooperating iptables rules plus one kernel sysctl. Each clause is load-bearing for a different invariant. The spec's one-line treatment invites the kind of "what's the minimum sufficient rule" reasoning that produced the failed Option E.

2. **rp_filter strict commitment is empirically wrong post-BBABC.** RBSAX:16 reads "Set rp_filter to strict mode (1) on all interfaces." Empirically: strict mode drops legitimate Docker-Desktop-delivered entry-port traffic at routing time before iptables can act. The fix uses loose mode (2) when `RBRN_ENTRY_MODE=enabled`. RBSAX should be updated with the security trade-off explained (kernel-layer spoof protection moves to iptables layer).

3. **MASQUERADE's two roles aren't distinguished in RBSAX.** RBSAX:21-24 describes POSTROUTING MASQUERADE only for outbound from enclave (source NAT for internet-bound bottle traffic). The entry-port path also requires MASQUERADE (to make the bottle see sentry as source, keeping the reply destination in-enclave for symmetric return through conntrack). The spec doesn't articulate this second role — which is why the topology reframe attempt got past architectural review without anyone catching that removing MASQUERADE would break return-path symmetry.

4. **"Don't trust Docker" should be elevated to architectural principle.** RBSSS:25 names this discipline for default-route selection only. It applies equally to: interface naming (RBSSS:20-22 handles), published-port target selection (broke here), source-IP visibility (broke at attempt 2 stage 2 + topology reframe), bridge gateway addressing, network creation ordering. Each new appearance has been a surprise. The spec should articulate the principle as a class, not enumerate workarounds case-by-case.

5. **"First packet" and "all failure modes" claims need topological grounding.** RBS0:609 ("the bottle cannot send traffic before security policies are enforced") and RBS0:675 ("These properties hold across all operations and failure modes") are aspirational. The topology reframe attempt showed they hold only under specific architectural choices (sentry on the inbound path, with conntrack creating symmetric return). The spec should ground these aspirations in concrete topological commitments.

6. **Defense-in-depth structure is implicit and got the architect wrong.** Each layer (rp_filter, source-CIDR exclusion, MASQUERADE, conntrack-DNAT-state, ESTABLISHED,RELATED, network namespace isolation, capability drop) does real work. The spec describes each in isolation but doesn't articulate the layered structure. Option E reasoned "iptables is the gate, why have redundant predicates" — which conflated "iptables is the security layer" (spec-correct) with "iptables can be reduced to minimum rules" (spec-misaligned). The defense-in-depth structure should be named.

7. **Ifrit Direct Attack Front isn't linked to specific iptables rules.** RBSIP:323-330 names the attack class ("probe sentry for listening services"). The implementation (`rbtdrc_sortie_direct_sentry_probe`) tests this empirically. The spec doesn't say which iptables rule is responsible for enforcing the invariant. Future maintainers reading the spec to understand "what blocks ifrit's direct probe of sentry" have to infer from the test that fails when the rule is missing.

### Specific spec deltas to apply (₢A_AAd contract)

These are the concrete changes ₢A_AAd applies. Each delta names file + region + intent. The implementer follows ₢A_AAd's docket discipline (render-diff per delta, out-of-scope boundary verification, etc.); this list is the "what" not the "how."

**Delta 1 — RBSAX:16 (rp_filter):** Replace "Set rp_filter to strict mode (1) on all interfaces" with: "When `RBRN_ENTRY_MODE=enabled`, set `/proc/sys/net/ipv4/conf/all/rp_filter` to loose mode (2). Required because post-BBABC, Docker delivers entry-port traffic on the enclave interface while the reverse path for the source IP is via uplink — strict rp_filter sees this as a spoofed packet and drops at routing time before iptables can act. The security trade-off: kernel-layer spoof protection for transit-side spoofing moves to iptables (RBM-INGRESS rules + source-CIDR exclusion at PREROUTING DNAT)."

**Delta 2 — RBSAX:21-24 (MASQUERADE):** Extend the POSTROUTING MASQUERADE description to articulate BOTH roles: (a) outbound from enclave to internet (current spec content); (b) entry-port path inbound, so the bottle sees sentry's enclave IP as source, keeping the reply destination in-enclave for conntrack-mediated symmetric return. The second role is load-bearing for the post-BBABC delivery topology.

**Delta 3 — RBSSS:32 (entry-port DNAT predicates):** Replace "Entry port DNAT if enabled" with enumerated predicates: (a) PREROUTING per-IP source exclusion of the enclave-internal containers via RETURN short-circuit pattern (`-s ${RBRN_ENCLAVE_SENTRY_IP} -j RETURN`; `-s ${RBRN_ENCLAVE_BOTTLE_IP} -j RETURN`) followed by unconditional DNAT on destination port; (b) POSTROUTING MASQUERADE on the entry-port path to enclave; (c) RBM-FORWARD ACCEPT matched on destination + conntrack `--ctstate DNAT`; (d) RBM-FORWARD baseline `--ctstate ESTABLISHED,RELATED` ACCEPT (already covered by RBSII state tracking but cross-reference here); (e) kernel sysctl `rp_filter=2` per Delta 1. Each clause enforces a distinct invariant; removing any clause requires empirical re-validation, not architectural reasoning. The per-IP RETURN pattern (instead of whole-CIDR exclusion) is required because Linux Docker Engine attaches the host to bridge networks as a peer with the bridge gateway IP — which is inside the enclave CIDR by construction, so whole-CIDR exclusion rejects legitimate host SYNs. The per-IP list MUST be maintained as the enclave-internal container set evolves (sidecar additions, etc.).

**Delta 4 — RBSSS:19-25 (don't-trust-Docker elevation):** Elevate the "don't trust Docker" framing from default-route-specific to architectural principle. Suggested addition after the existing default-route discussion: "Sentry takes ownership of any runtime-implicit behavior its security envelope depends on. This applies to: interface naming (IP-based discovery, already articulated), default-route selection (explicit override post-discovery, already articulated), published-port target selection (iptables rules that don't depend on which interface Docker chose for delivery), and any future runtime-implicit ordering that emerges. The principle: kernel + sentry's own iptables are the source of truth; framework-implicit ordering is not load-bearing."

**Delta 5 — RBS0:609 and RBS0:675 (topological grounding):** Add concrete commitments after the existing claims. Suggested form: "These properties depend on sentry being on every inbound path. The published-port directive on sentry (not pentacle), the bottle's default route through sentry, and sentry's iptables installation before the bottle's namespace is shared are the architectural commitments that make 'first packet' enforceable. Topology changes that violate these commitments require re-validation against the Ifrit suite before declaring the security properties preserved."

**Delta 6 — RBSIP:323-330 (front-to-rule linkage):** For each anticipated front, add a one-line pointer to the specific iptables rule or kernel mechanism that enforces the property the sortie tests. Direct Attack Front → "PREROUTING DNAT source-CIDR exclusion (RBSAX/RBSSS)"; DNS front → "RBM-FORWARD DNS rules (RBSAX allowlist branch)"; etc. This makes the spec self-checking — future maintainers can see which spec clause enforces which security invariant.

**Delta 7 — defense-in-depth framing (location TBD by implementer):** Add a short subsection (likely in RBSCO or RBS0) naming the layered-defense structure explicitly: the security envelope is enforced jointly by network namespace isolation, capability drops, kernel rp_filter (now loose, with iptables compensation), iptables filter chains (RBM-INGRESS, RBM-EGRESS, RBM-FORWARD), iptables NAT (PREROUTING DNAT, POSTROUTING MASQUERADE), and conntrack state tracking. Each layer is independently load-bearing for at least one invariant. Removing a layer requires Ifrit-suite empirical validation, not architectural reasoning.

### Mount-time inheritance

If ₢A_AAc (ifrit-sortie-enclave-spoof-external) lands before ₢A_AAd mounts and surfaces an architectural mitigation (e.g., the targeted "drop non-enclave-source on enclave interface" rule), Delta 3's enumeration should incorporate that mitigation as a sixth clause. Update the deltas list in this memo before starting ₢A_AAd's edit cycle.

## Open questions

Narrowed considerably after empirical validation. Status:

1. **ESTABLISHED,RELATED rule in RBM-FORWARD** — **resolved.** Empirically verified present in current `rbjs_sentry.sh`; egress-mode blocks rely on it; the empirical three-fixture green confirms return-path traversal works. No action needed in this implementation; documented in the fix recipe for posterity.

2. **Linux Docker Engine 28.x verification under per-IP-exclusion form** — **load-bearing next experiment.** Whole-CIDR exclusion was empirically falsified on Linux (cerebro diagnostic). Per-IP exclusion is the current hypothesis. The next experiment runs the three-fixture parallel under the new form. If green: macOS re-verification under same form, then canonical declaration. If red: another structural surprise has surfaced; iterate the hypothesis.

3. **macOS re-verification under per-IP-exclusion form** — pending. The prior macOS green was under whole-CIDR; per-IP is a strict subset of CIDR's block set (only blocks two specific IPs vs the whole subnet), so macOS green is expected by analysis, but empirical confirmation required before canonical declaration.

4. **Docker Desktop reflection-through-transit mechanism** — interesting but not blocking. The fix is independent of this behavior. Defer to a future research round if Podman parity work needs deeper trace of Desktop networking internals.

5. **iptables vs nftables inside sentry under Podman** — still open from the companion memo. Run `iptables -V` and `iptables-save` inside sentry under Podman Netavark when Podman validation begins.

6. **Spoof-as-arbitrary-external-routable-IP attack vector** — subject of pace **₢A_AAc**. See "Ifrit coverage limitation" section above. Either resolves via the existing iptables rules (RBM-INGRESS or related), or requires a targeted mitigation (selective rp_filter-strict at iptables raw table, or "drop non-enclave-source on enclave interface" rule). Decision deferred to empirical result.

7. **Spec deltas application** — subject of pace **₢A_AAd**. See "Spec alignment findings" section above. Hold until canonical cross-platform pass. Deltas list is the contract; ₢A_AAd's docket has the editing discipline. Note: Delta 3's per-IP-exclusion form depends on the current hypothesis being empirically validated; if a future structural surprise changes the PREROUTING shape, Delta 3 must be updated before ₢A_AAd mounts.

8. **Future entry-mode use cases requiring original-client IP** — out of scope. The fix preserves the same source-IP-visibility property the original architecture had (bottle sees sentry's enclave IP, not the original client); if a future use case needs original-client IP, add an application-layer proxy. Documented in the companion memo's Round 4 listener-source-IP table.

9. **Multi-`! -s` iptables semantics empirical confirmation** — minor open item. The RETURN short-circuit pattern in the Fix Recipe is unambiguous. The single-rule form (`iptables ... ! -s SENTRY_IP ! -s BOTTLE_IP -j DNAT ...`) MAY work cleaner but is not empirically confirmed. If during ₢A_AAd or follow-up work someone wants to simplify, verify the single-rule form's behavior on Linux + macOS Docker first; default is the RETURN pattern.

## References

- Companion memo: `Memos/memo-20260512-srjcl-port-publish-regression.md` — full diagnostic history, four rounds of web research, source URLs.
- `rbev-vessels/common-sentry-context/rbjs_sentry.sh` — sentry startup script; entry-port iptables to be restored here.
- `.rbk/rbob_compose.yml` — sentry/pentacle/bottle service definitions; `ports:` directive to be moved back to sentry.
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` (around lines 1597-1646) — srjcl test cases including `_jupyter_running` and `_jupyter_connectivity`.
- `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs` (around lines 1961-2048) — `net-dnat-entry-reflection` ifrit invariant; must remain preserved.
- `Tools/rbk/rboo_observe.sh` and `tt/rbw-cs.Scry.sh` — scry diagnostic infrastructure to be retained.
- Pace ₢A_AAb (sentry-entry-port-dnat-forward-rescope) — this work.
- Commit `c8b218ef` (and successors) — the topology reframe to be reverted. Reference for what to undo.
