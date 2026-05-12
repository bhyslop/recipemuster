# Sentry Entry-Port Repair — Anchor for Next Attempt

**Date:** 2026-05-12
**Pace:** ₢A_AAb (sentry-entry-port-dnat-forward-rescope)
**Status:** Action anchor after topology-reframe failure and decisive scry-empirical evidence.
**Companion memo:** `Memos/memo-20260512-srjcl-port-publish-regression.md` — full diagnostic history, four rounds of web research, two prior repair attempts and their analyses. Read for lineage; this memo is the forward action plan.

## TL;DR

Revert the topology reframe. Restore sentry-as-publisher in `.rbk/rbob_compose.yml` (symmetric `7999:7999` port mapping). Re-introduce the entry-port iptables block in `rbev-vessels/common-sentry-context/rbjs_sentry.sh`, in interface-agnostic, source-IP-agnostic form: classification by destination port only at PREROUTING; MASQUERADE retained on the entry-port path; FORWARD authorization via conntrack DNAT-state; baseline ESTABLISHED,RELATED rule confirmed in RBM-FORWARD (read source to verify; add if absent). Keep three things from the failed reframe: the `ip_forward` lift (independent hygiene), the `cap_drop: [NET_ADMIN]` addition on bottle (defense in depth), the `tt/rbw-cs.Scry.sh` tabtarget (new diagnostic capability). Cross-platform pristine gauntlet on macOS Docker Desktop 28.x and Linux Docker Engine 28.x is the verification gate.

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

The companion memo's "Refined Fix Shape" (Round 3, permissive PREROUTING + conntrack RBM-FORWARD) was *close* to correct but had two omissions corrected here:

1. **MASQUERADE on the entry-port path was missing.** Round 3's reasoning held that "conntrack DNAT-state alone authorizes the return" — true for the FORWARD authorization predicate, but the MASQUERADE serves a different purpose (keeping the bottle's reply destination in-enclave so it routes cleanly back through sentry's conntrack). Without MASQUERADE, the reply destination is whatever the runtime put as the source IP, which on Docker Desktop is in hostile address space.
2. **An ESTABLISHED,RELATED FORWARD rule is required** for the return path through sentry's egress filter. Round 3's conntrack DNAT-state ACCEPT matches inbound direction only. The return needs its own pass-through.

The Round 3 fix's principled stance — interface-agnostic, source-IP-agnostic rules — is retained. The omissions are filled in this memo.

## The fix recipe (architectural; agent reads source for exact syntax)

In `rbev-vessels/common-sentry-context/rbjs_sentry.sh`, restore the entry-port iptables block in the following architectural form. Read the file for current variable names, function structure, and surrounding rule order; do not blind-copy from the companion memo's superseded snippets.

**PREROUTING DNAT — classification:** match on destination port (`${RBRN_ENTRY_PORT_WORKSTATION}`) only. No interface filter (`-i ...` removed). No source-IP filter (`! -s ...` removed). Destination NAT to `${RBRN_ENCLAVE_BOTTLE_IP}:${RBRN_ENTRY_PORT_ENCLAVE}` (port translation is performed by the DNAT, not by the compose `ports:` mapping). Rationale: destination port is the stable classifier; runtime-dependent source IP and runtime-chosen ingress interface are not trust labels and should not appear in the predicate.

**POSTROUTING MASQUERADE — return-path-symmetry preservation:** match the entry-port path on the outgoing direction (`-d ${RBRN_ENCLAVE_BOTTLE_IP} --dport ${RBRN_ENTRY_PORT_ENCLAVE}`). Rationale: makes the bottle see the inbound as originating from sentry's enclave IP, which routes the reply back through sentry's directly-connected enclave route (not through the default-via-sentry route into transit). This is the load-bearing clause the prior repair attempts missed.

**RBM-FORWARD ACCEPT for inbound — authorization:** match destination (`-d ${RBRN_ENCLAVE_BOTTLE_IP} --dport ${RBRN_ENTRY_PORT_ENCLAVE}`) AND conntrack DNAT-state (`-m conntrack --ctstate DNAT`). Rationale: only flows that sentry's own PREROUTING DNAT created conntrack state for are authorized. The conntrack state cannot be forged by an attacker on either bridge.

**RBM-FORWARD ESTABLISHED,RELATED ACCEPT — return-path authorization (verify exists, add if absent):** match by conntrack state (`-m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT`), positioned early in the chain so return traffic passes before any per-destination access-mode filtering. Rationale: the bottle's SYN-ACK and subsequent return packets need to traverse RBM-FORWARD to leave sentry's transit interface. Without this rule, return traffic hits the default DROP. **Read `rbjs_sentry.sh` to check whether a baseline ESTABLISHED,RELATED rule already exists in RBM-FORWARD** (the egress-mode blocks may rely on it). If present, no change needed here. If absent, this clause is required and is likely the explanation for the companion memo's attempt-2-stage-2 failure.

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

## Lessons captured

The three-attempt arc, briefly (full lineage in the companion memo):

1. **Attempt 1 — compose rename `transit → auplink` (experiment, reverted).** Worked, exploited Docker's undocumented alphabetical-first network-name heuristic. Rejected as canonical for cross-runtime portability.

2. **Attempt 2 — source-CIDR exclusion at PREROUTING + FORWARD.** Stage 1 (with rename) green; stage 2 (no rename) red. Stage 2 failure most likely explained by missing MASQUERADE or missing ESTABLISHED,RELATED rule in RBM-FORWARD (or both); the bridge-gateway-source-rewrite hypothesis that Round 3 used to justify abandoning source-CIDR was empirically wrong on Docker Desktop (actual source is `192.168.65.1`, outside enclave CIDR).

3. **Attempt 3 — topology reframe (publisher on pentacle).** Failed empirically. Root cause: bottle's default-via-sentry routing makes inbound published-port traffic produce asymmetric return paths through sentry's transit interface, where Docker Desktop's NAT reflection breaks the symmetry hostilely. Architecturally clean in isolation but fundamentally incompatible with our egress-gate-via-default-route security model.

The fix in this memo is **what pre-BBABC's design would have looked like if written defensively against future framework changes**: the same DNAT + MASQUERADE + FORWARD-ACCEPT shape, but with classification by destination port (not interface or source IP) and authorization by conntrack state (not interface or source IP). The original design's principle — sentry takes ownership of its security envelope — is preserved.

## Open questions

Narrowed considerably after the scry capture. Remaining:

1. **Empirically confirm an ESTABLISHED,RELATED rule exists in RBM-FORWARD** during this implementation. Read `rbjs_sentry.sh`; if absent, add it. This is likely the missing piece in attempt 2 stage 2.
2. **Docker Desktop reflection-through-transit mechanism** — interesting but not blocking. The fix is independent of this behavior. Defer to a future Round 5 if Podman parity work needs deeper trace of Desktop networking internals.
3. **iptables vs nftables inside sentry under Podman** — still open from the companion memo. Run `iptables -V` and `iptables-save` inside sentry under Podman Netavark when Podman validation begins.
4. **Future entry-mode use cases requiring original-client IP** — out of scope. The fix preserves the same source-IP-visibility property the original architecture had (bottle sees sentry's enclave IP, not the original client); if a future use case needs original-client IP, add an application-layer proxy. Documented in the companion memo's Round 4 listener-source-IP table.

## References

- Companion memo: `Memos/memo-20260512-srjcl-port-publish-regression.md` — full diagnostic history, four rounds of web research, source URLs.
- `rbev-vessels/common-sentry-context/rbjs_sentry.sh` — sentry startup script; entry-port iptables to be restored here.
- `.rbk/rbob_compose.yml` — sentry/pentacle/bottle service definitions; `ports:` directive to be moved back to sentry.
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` (around lines 1597-1646) — srjcl test cases including `_jupyter_running` and `_jupyter_connectivity`.
- `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs` (around lines 1961-2048) — `net-dnat-entry-reflection` ifrit invariant; must remain preserved.
- `Tools/rbk/rboo_observe.sh` and `tt/rbw-cs.Scry.sh` — scry diagnostic infrastructure to be retained.
- Pace ₢A_AAb (sentry-entry-port-dnat-forward-rescope) — this work.
- Commit `c8b218ef` (and successors) — the topology reframe to be reverted. Reference for what to undo.
