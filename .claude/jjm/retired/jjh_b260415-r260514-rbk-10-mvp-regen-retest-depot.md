# Heat Trophy: rbk-10-mvp-regen-retest-depot

**Firemark:** ₣A_
**Created:** 260415
**Retired:** 260514
**Status:** retired

## Paddock

## Context

Three interwoven infrastructure cleanups culminating in a depot regeneration:

1. **Resource prefixing** (`₢A_AAA`/`₢A_AAB`/`₢A_AAC`) — introduce `RBRR_CLOUD_PREFIX` (cloud-visible names: GAR, Cloud Build, GCP) and `RBRR_RUNTIME_PREFIX` (local container and network names). Enables multi-regime isolation on shared infrastructure.

2. **GAR categorical layout migration** (`₢A_AAK`/`₢A_AAL`) — reorganize GAR top-level namespace into three prefixed categories. Dissolves the `<vessel>:<hallmark>-<suffix>` hyphen-suffix scheme and the `vouches/` aggregator. Tabtargets taking `(vessel, hallmark)` simplify to `(hallmark)`.

3. **Payor credential subdirectory** (`₢A_AAD`) — move `rbro.env` into a `payor/` subdirectory to match the role-subdirectory pattern already established for RBRA credentials.

The depot regen at the tail (`₢A_AAE`) is the hard cutover point. All three threads land before it; depot regen validates the new infrastructure in regenerated form.

## Design decisions captured

### GAR categorical layout

Three top-level namespaces under the cloud prefix:

```
<prefix>hallmarks/<hallmark>/{image, vouch, pouch, about, attest, diags}
<prefix>reliquaries/<date>/<tool>
<prefix>enshrines/<base>
```

Each immediate child of `<prefix>hallmarks` IS a hallmark; the six ark types become plain basename siblings. No more vessel-in-path or suffix-on-tag string grammar.

### Vessel identity

No longer lives in the GAR path. Resolved on demand from the `vouch` or `about` metadata of the hallmark in question. `ls <prefix>hallmarks/` no longer surfaces vessel at a glance; tally and plumb already read metadata for health, and extend the pattern where operation prose needs the vessel name.

### Attest vs vouch

Both persist as peer basenames under `<hallmark>/`. Attestations advertise during build for SLSA provenance; vouches check them at consumption. Load-bearing split — retained by design.

### Pouch

A `FROM scratch` OCI image carrying build context (Dockerfile + supporting files) to Cloud Build's first step. Present for conjure/bind, absent for graft (graft is local push — no GCB involved). Now lives at `<prefix>hallmarks/<hallmark>/pouch` in the new layout.

### SUFFIX → BASENAME constant rename

`RBGC_ARK_SUFFIX_*` constants persist but rename to `RBGC_ARK_BASENAME_*`; their values drop the leading hyphen. Role shifts from "suffix to hyphen-concat" to "literal basename of a sibling file." The concat grammar disappears entirely from ark path construction.

### `vouches/` aggregator removal

`RBGC_VOUCHES_PACKAGE` existed to enable hallmark-only vouch lookup without knowing the vessel. The new layout delivers the same property naturally — every hallmark has its own `vouch` basename under `<prefix>hallmarks/<hallmark>/`. The aggregator becomes redundant and is deleted.

### Cutover discipline

Hard cutover — old and new path shapes do not coexist in live code. Depot regen at `₢A_AAE` is the forcing function; all layout work lands before it.

## References

- `₢A_AAJ` — filesystem-side config-directory naming research (orthogonal to this heat). Decisions recorded in that pace: `rbmm_moorings` umbrella, `rbm*_` prefix family, hard-cutover at depot regen. Filesystem rename itself flows to a separate follow-up heat and is out of this heat's scope.
- `Tools/rbk/rbgc_Constants.sh:122-131` — current ark-suffix constants and `RBGC_VOUCHES_PACKAGE`; this heat renames to `_BASENAME_*` and removes the aggregator.
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:712-733` — pouch definition and current concat-based path construction.
- `Tools/rbk/rbfl_FoundryLedger.sh:440-650` — current per-suffix HEAD/DELETE dance; collapses to subtree operations in `₢A_AAK`.

## Paces

### depot-fact-files-namespaced-by-cloud-prefix (₢A_AAo) [complete]

**[260513-1218] complete**

## Character

Done-pending hybrid: structural fix landed this session; data-state
cleanup remains. Open this pace next session to do the cleanup.

## What landed

Depot fact files moved to `<cloud_prefix>/<moniker>.depot` layout to
prevent same-moniker collisions across cloud_prefixes. Surfaced when
`rbw-dl` FATAL'd on `canest2bhm100002` existing under both `cancbhl`
and `cancbhm`. See the notch that affiliates with this pace for the
diff and rationale.

## What remains: delete all existing depots

The current depot population (39 projects) predates the fix and
includes the live moniker duplicate. Wiping it gives future levies a
clean substrate matching the new invariants.

- Re-list: `rbw-dl.PayorListsDepots.sh`
- For each ACTIVE depot: `rbw-dU.PayorUnmakesDepot.sh <project_id>`
- 24 are already in DELETE_REQUESTED; ~15 to unmake by hand

Done when `rbw-dl` reports zero COMPLETE depots (only
DELETE_REQUESTED remain).

**[260513-0823] rough**

## Character

Done-pending hybrid: structural fix landed this session; data-state
cleanup remains. Open this pace next session to do the cleanup.

## What landed

Depot fact files moved to `<cloud_prefix>/<moniker>.depot` layout to
prevent same-moniker collisions across cloud_prefixes. Surfaced when
`rbw-dl` FATAL'd on `canest2bhm100002` existing under both `cancbhl`
and `cancbhm`. See the notch that affiliates with this pace for the
diff and rationale.

## What remains: delete all existing depots

The current depot population (39 projects) predates the fix and
includes the live moniker duplicate. Wiping it gives future levies a
clean substrate matching the new invariants.

- Re-list: `rbw-dl.PayorListsDepots.sh`
- For each ACTIVE depot: `rbw-dU.PayorUnmakesDepot.sh <project_id>`
- 24 are already in DELETE_REQUESTED; ~15 to unmake by hand

Done when `rbw-dl` reports zero COMPLETE depots (only
DELETE_REQUESTED remain).

### sentry-entry-port-dnat-forward-rescope (₢A_AAb) [complete]

**[260513-0644] complete**

## Character

Repair execution — architecture empirically validated cross-platform at three-fixture level (six attempts in the anchor memo's lineage; Attempt 6 per-IP exclusion form is the validated shape). Pristine gauntlet is the remaining canonical-declaration gate. A fresh agent picks up the anchor memo cold and runs the pristine gauntlet on both platforms.

## Docket

Apply the sentry-entry-port repair per `Memos/memo-20260512-sentry-entry-port-repair-anchor.md`. The Fix Recipe specifies the empirically-validated per-IP exclusion form (PREROUTING per-IP RETURN short-circuit for the two enclave-internal IPs + unconditional DNAT + POSTROUTING MASQUERADE + RBM-FORWARD conntrack-DNAT ACCEPT + ESTABLISHED,RELATED baseline + rp_filter=2 loose).

**Current state** (load-bearing for cold mount):

- Source tree: per-IP exclusion form is committed and validated. Cross-platform three-fixture green confirmed:
  - macOS Docker Desktop 28.x: srjcl 3/3, pluml 5/5, tadmor 54/54.
  - Linux Docker Engine 28.x: srjcl 3/3 (including the prior-regression target), pluml 5/5, tadmor 54/54. Mechanism-level confirmation from cerebro (DNAT counter incrementing; source IP flow traced through MASQUERADE creating symmetric return path).
- Memo and source tree are aligned. No in-flight hypothesis remaining.
- Remaining work in this pace: full pristine gauntlet (`tt/rbw-tP.QualifyPristine.sh`) on BOTH platforms for canonical declaration. Pristine adds the four-mode lifecycle fixtures (build/publish paths, orthogonal to runtime network topology) — hardening margin, not load-bearing for architectural correctness, but it's the canonical gate per project discipline.

**Next bounded step (cold mount):**

1. Verify source tree per-IP form is in place: `grep -n 'RBRN_ENCLAVE_SENTRY_IP.*RETURN' rbev-vessels/common-sentry-context/rbjs_sentry.sh`. Expected: matches present.
2. Run `tt/rbw-tP.QualifyPristine.sh` on this platform.
3. Coordinate cross-platform pristine on the other-platform operator if not local.
4. Both green: ₢A_AAb is ready to wrap; ₢A_AAd (spec deltas) becomes mountable.
5. Either red: diagnose specifically; likely a regression in something other than the entry-port architecture (four-mode lifecycle tests build paths, not runtime networking). Don't reflexively re-iterate the entry-port fix.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — primary source: action plan, validated fix recipe, empirical validation status (cross-platform three-fixture green), six-attempt lineage, spec alignment findings (input contract for ₢A_AAd), reset handoff notes.
- `Memos/memo-20260512-srjcl-port-publish-regression.md` — companion: full diagnostic history.
- `tt/rbw-cs.Scry.sh` — diagnostic infrastructure (institutional value, retain).
- ₢A_AAd (slated, mount-gated on canonical pass) — RBS spec deltas.
- ₢A_AAc (slated, after ₢A_AAd by current ordering) — Ifrit sortie for residual spoof-as-arbitrary-external gap.

**[260512-1139] rough**

## Character

Repair execution — architecture empirically validated cross-platform at three-fixture level (six attempts in the anchor memo's lineage; Attempt 6 per-IP exclusion form is the validated shape). Pristine gauntlet is the remaining canonical-declaration gate. A fresh agent picks up the anchor memo cold and runs the pristine gauntlet on both platforms.

## Docket

Apply the sentry-entry-port repair per `Memos/memo-20260512-sentry-entry-port-repair-anchor.md`. The Fix Recipe specifies the empirically-validated per-IP exclusion form (PREROUTING per-IP RETURN short-circuit for the two enclave-internal IPs + unconditional DNAT + POSTROUTING MASQUERADE + RBM-FORWARD conntrack-DNAT ACCEPT + ESTABLISHED,RELATED baseline + rp_filter=2 loose).

**Current state** (load-bearing for cold mount):

- Source tree: per-IP exclusion form is committed and validated. Cross-platform three-fixture green confirmed:
  - macOS Docker Desktop 28.x: srjcl 3/3, pluml 5/5, tadmor 54/54.
  - Linux Docker Engine 28.x: srjcl 3/3 (including the prior-regression target), pluml 5/5, tadmor 54/54. Mechanism-level confirmation from cerebro (DNAT counter incrementing; source IP flow traced through MASQUERADE creating symmetric return path).
- Memo and source tree are aligned. No in-flight hypothesis remaining.
- Remaining work in this pace: full pristine gauntlet (`tt/rbw-tP.QualifyPristine.sh`) on BOTH platforms for canonical declaration. Pristine adds the four-mode lifecycle fixtures (build/publish paths, orthogonal to runtime network topology) — hardening margin, not load-bearing for architectural correctness, but it's the canonical gate per project discipline.

**Next bounded step (cold mount):**

1. Verify source tree per-IP form is in place: `grep -n 'RBRN_ENCLAVE_SENTRY_IP.*RETURN' rbev-vessels/common-sentry-context/rbjs_sentry.sh`. Expected: matches present.
2. Run `tt/rbw-tP.QualifyPristine.sh` on this platform.
3. Coordinate cross-platform pristine on the other-platform operator if not local.
4. Both green: ₢A_AAb is ready to wrap; ₢A_AAd (spec deltas) becomes mountable.
5. Either red: diagnose specifically; likely a regression in something other than the entry-port architecture (four-mode lifecycle tests build paths, not runtime networking). Don't reflexively re-iterate the entry-port fix.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — primary source: action plan, validated fix recipe, empirical validation status (cross-platform three-fixture green), six-attempt lineage, spec alignment findings (input contract for ₢A_AAd), reset handoff notes.
- `Memos/memo-20260512-srjcl-port-publish-regression.md` — companion: full diagnostic history.
- `tt/rbw-cs.Scry.sh` — diagnostic infrastructure (institutional value, retain).
- ₢A_AAd (slated, mount-gated on canonical pass) — RBS spec deltas.
- ₢A_AAc (slated, after ₢A_AAd by current ordering) — Ifrit sortie for residual spoof-as-arbitrary-external gap.

**[260512-1112] rough**

## Character

Repair execution — six repair attempts documented in the anchor memo; current hypothesis (Attempt 6, per-IP exclusion via RETURN short-circuit) is empirically pending cross-platform verification. A fresh agent picks up the anchor memo cold and runs the next experiment.

## Docket

Apply the sentry-entry-port repair per `Memos/memo-20260512-sentry-entry-port-repair-anchor.md`. The anchor memo's Fix Recipe specifies the current per-IP-exclusion hypothesis (PREROUTING per-IP RETURN short-circuit for the two enclave-internal IPs + unconditional DNAT + POSTROUTING MASQUERADE + RBM-FORWARD conntrack-DNAT ACCEPT + ESTABLISHED,RELATED baseline + rp_filter=2 loose).

**Current in-flight state** (load-bearing for cold mount):

- Source tree has the PRIOR whole-CIDR exclusion form. Empirically falsified on Linux Docker Engine 28.x per cerebro diagnostic (host attaches to enclave bridge with `.1` IP — inside enclave CIDR by Docker convention — so `! -s ENCLAVE_CIDR` rejects every legitimate host SYN). macOS Docker Desktop 28.x was green under this form (three-fixture 54/54 tadmor including the decisive security cases).
- Memo Fix Recipe specifies the per-IP-exclusion form. NOT yet tested on any platform.
- Next experiment: implement per-IP exclusion in `rbev-vessels/common-sentry-context/rbjs_sentry.sh` (RETURN short-circuit pattern per the Fix Recipe), kludge sentry, refresh hallmarks on srjcl + pluml + tadmor, run three-fixture parallel on BOTH macOS Docker Desktop AND Linux Docker Engine. Coordinate with the other-platform operator if you only have access to one.

Cross-platform three-fixture green AND pristine gauntlet green on both platforms under the per-IP form is the canonical declaration gate. NO platform meets the gate under the current hypothesis.

If the per-IP experiment surfaces another structural Docker fact (a fourth such surprise in this work's arc — see anchor memo's meta-lesson section for the running tally), apply the iteration discipline: process the result, update the memo with the new hypothesis, run the next experiment. Do NOT abandon the architecture or regress to "clean" architectural reasoning.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — primary source: action plan, fix recipe (per-IP form), empirical validation status table, six-attempt lineage with meta-lesson, spec alignment findings, verification protocol.
- `Memos/memo-20260512-srjcl-port-publish-regression.md` — companion: full diagnostic history and four rounds of web research.
- `tt/rbw-cs.Scry.sh` — diagnostic infrastructure (institutional value, retain).
- ₢A_AAc (slated, not started) — Ifrit sortie for residual spoof-as-arbitrary-external gap.
- ₢A_AAd (slated, not started) — RBS spec deltas, mount-gated on canonical cross-platform pass.

**[260512-0942] rough**

## Character

Repair execution — three repair-attempt iterations are now documented; this pace applies the final fix path anchored in the new repair-anchor memo. A fresh agent should pick up the anchor memo cold and execute.

## Docket

Apply the sentry-entry-port repair per `Memos/memo-20260512-sentry-entry-port-repair-anchor.md`. The anchor memo carries the revert plan (undo the topology reframe), the fix recipe (architectural form — agent reads `rbjs_sentry.sh` for exact syntax), the retain-from-failed-attempt list (`ip_forward` lift, `NET_ADMIN` cap_drop, the new `rbw-cs.Scry` tabtarget), and the verification protocol (scry capture + three-fixture iteration + cross-platform pristine gauntlet).

The companion diagnostic memo `Memos/memo-20260512-srjcl-port-publish-regression.md` carries full lineage (four rounds of web research, two prior failed repair attempts) for context but is not required for execution.

Cross-platform pristine-gauntlet `rbtdrc_srjcl_jupyter_connectivity` green on BOTH macOS Docker Desktop 28.x and Linux Docker Engine 28.x is the non-negotiable verification gate.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — primary source: action plan, revert + fix recipe, verification protocol.
- `Memos/memo-20260512-srjcl-port-publish-regression.md` — companion: full lineage and research history.

**[260512-0736] rough**

## Character

Repair execution — diagnosis is complete in the memo; this pace
applies the proposed sentry-side fix and verifies on both platforms.

## Docket

Apply the sentry-side repair described in
`Memos/memo-20260512-srjcl-port-publish-regression.md`. Both halves
must land — the PREROUTING DNAT rescope AND the RBM-FORWARD rescope.
macOS empirical evidence in the memo shows the DNAT half alone is
insufficient.

Pristine-gauntlet srjcl connectivity green on BOTH macOS and linux is
the verification gate. Non-negotiable — see the memo's "Verification
Gate" section.

Sentry image is shared between srjcl and pluml; the vessel re-kludge
covers both nameplates. Hallmark refresh on each affected nameplate
follows.

### References

- `Memos/memo-20260512-srjcl-port-publish-regression.md` — primary
  source: symptom, root cause, lineage (₢BBABC + ₢BBABE), fix shape,
  files, verification gate, crucible state at memo-writing time.

### rbs-spec-deltas-post-sentry-fix (₢A_AAd) [complete]

**[260513-0736] complete**

## Character

Spec hygiene. Apply spec deltas from ₢A_AAb / anchor-memo's alignment findings. Mount ONLY after ₢A_AAb canonical pristine-gauntlet green on both platforms.

Treat as fragile, structural work — not a quick cleanup. The prior pace's arc demonstrated how easily plausible-looking architectural reasoning produces subtle empirical errors. Spec edits don't have an empirical safety net of the same kind that the Ifrit suite provided for code; the rigor has to live in the editing PROCESS itself.

## Mount-time gate check (before reading anything else)

1. Confirm git log shows ₢A_AAb's landing commit.
2. Run `tt/rbw-tP.QualifyPristine.sh` once on this platform to confirm canonical green still holds. If it does not, do not proceed — surface what changed before any spec edit.

## Pre-work — structural inventory + operator checkpoint

Before opening any RBS*.adoc file in edit mode:

1. Read the anchor memo's "Spec alignment findings" subsection in full — that is the deltas contract.
2. For each delta, produce in a scratch file:
   - File + exact line range being changed.
   - Every `{attribute_reference}` used in or near those lines.
   - Every `[[anchor]]` defined in or near those lines.
   - Every `axhob_operation` / `axhos_step` / `axhoc_completion` annotation that participates.
   - Every cross-file reference (other RBS* docs / RBS0 mapping section).
3. For each attribute reference, confirm its quoin is defined in RBS0's mapping section. Flag NEW attribute references the deltas introduce — those need quoin definitions added separately.
4. Define explicit out-of-scope boundaries: for each spec file touched, list the section ranges that MUST NOT change. Any post-edit diff touching outside those boundaries is a sign of accidental damage.
5. **Operator checkpoint:** surface the inventory + boundaries to the operator BEFORE editing. Do not start editing until reviewed. This is the cheapest place to catch "your delta scope is wider than I thought" or "this attribute reference doesn't exist."

## During edit — atomic, verified, individually committed

1. One delta = one edit cycle = one commit. The cycle:
   a. **Render before**: produce the rendered output of the affected file via the project's AsciiDoc render path. Save baseline.
   b. **Apply delta** as a precise old_string → new_string edit. No `replace_all`. No regex sweeps. Exact string substitution only.
   c. **Render after**: produce the rendered output again. Diff against baseline.
   d. **Verify the rendered diff**: only the intended change appears; no raw `{attribute}` markers in output (would indicate broken ref); no dangling anchor warnings; the surrounding paragraphs still read coherently (read them, don't just diff them).
   e. **Notch the delta**: `jjx_record` with the specific file and an intent describing this single delta and its rationale. One delta per commit.
2. If a delta requires touching multiple files atomically (e.g., RBSSS prose + RBS0 mapping section addition), keep atomic but treat as a single delta — one cycle, one render-diff, one notch.
3. Do NOT batch deltas. Each delta completes the full cycle before the next begins.
4. **Forbidden: prose invention beyond the deltas list.** If a delta seems to require adding a sentence not in the anchor memo's deltas — STOP. Surface as a question to the operator. "I needed to add a sentence to make the flow work" is precisely the class of decision that wallowed in the prior chat.

## After all deltas — sweep verification

1. Render the entire RBS0-SpecTop.adoc transitive include tree.
2. For each spec file edited, diff the rendered output against the pre-edit baseline from step 1.a. Walk the diff:
   - In-scope changes are expected and match the deltas list.
   - Out-of-scope sections (the boundaries defined in pre-work step 4) MUST show zero differences. Any difference there is accidental damage; investigate before declaring done.
3. Run `tt/rbw-tf.TestFixture.regime-validation.sh` and any nameplate-validation tabtargets to confirm spec edits haven't broken downstream consumers (CLAUDE.md interpolation, generated tabtarget-context docs, etc.).
4. Produce a "deltas-applied verification" scratch document: for each delta in the anchor memo's list, cite the commit(s) that applied it. Any unapplied deltas get explicit justification or are marked as future-work for a follow-up pace.

## Independent-eyes check (recommended)

After all deltas land but before close: spawn a fresh Explore agent (no chat context) to read each edited spec file in isolation and report on architectural coherence. The fresh agent has no investment in the changes and will catch phrasing that's clearly fishy from outside the context that produced it. If the fresh agent flags anything, treat it as a wallow-avoidance signal — investigate before close.

## Wallow-avoidance discipline (the meta-rule)

When in doubt, do not improvise. The deltas list in the anchor memo is the contract. Deviations from it require explicit operator approval. The cost of one "should I do X?" question is small; the cost of one wrong improvised sentence in the source-of-truth spec is potentially weeks of confused downstream readers.

This chat demonstrated how plausible-but-wrong architectural moves wallow when committed without empirical check (Option E "drop source-CIDR" → empirical Ifrit catch hours later; topology reframe → empirical asymmetric-return catch hours later). The render-diff cycle in "During edit" above is the empirical check for spec edits. Trust it; do not skip it; do not batch deltas to "save time."

## References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — Spec alignment findings subsection contains the deltas list and per-delta rationale. This is the contract.
- `Memos/memo-20260512-srjcl-port-publish-regression.md` — companion lineage.
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` — MCM patterns for attribute references, anchors, linked terms. Consult before editing if any delta introduces new linked terms.
- ₢A_AAb landing commit — the canonical architecture this spec reflects.
- ₢A_AAc — if landed before this pace mounts, may have produced architectural changes (per its "If empirically allowed" branch); incorporate into the pre-work inventory.

**[260512-1037] rough**

## Character

Spec hygiene. Apply spec deltas from ₢A_AAb / anchor-memo's alignment findings. Mount ONLY after ₢A_AAb canonical pristine-gauntlet green on both platforms.

Treat as fragile, structural work — not a quick cleanup. The prior pace's arc demonstrated how easily plausible-looking architectural reasoning produces subtle empirical errors. Spec edits don't have an empirical safety net of the same kind that the Ifrit suite provided for code; the rigor has to live in the editing PROCESS itself.

## Mount-time gate check (before reading anything else)

1. Confirm git log shows ₢A_AAb's landing commit.
2. Run `tt/rbw-tP.QualifyPristine.sh` once on this platform to confirm canonical green still holds. If it does not, do not proceed — surface what changed before any spec edit.

## Pre-work — structural inventory + operator checkpoint

Before opening any RBS*.adoc file in edit mode:

1. Read the anchor memo's "Spec alignment findings" subsection in full — that is the deltas contract.
2. For each delta, produce in a scratch file:
   - File + exact line range being changed.
   - Every `{attribute_reference}` used in or near those lines.
   - Every `[[anchor]]` defined in or near those lines.
   - Every `axhob_operation` / `axhos_step` / `axhoc_completion` annotation that participates.
   - Every cross-file reference (other RBS* docs / RBS0 mapping section).
3. For each attribute reference, confirm its quoin is defined in RBS0's mapping section. Flag NEW attribute references the deltas introduce — those need quoin definitions added separately.
4. Define explicit out-of-scope boundaries: for each spec file touched, list the section ranges that MUST NOT change. Any post-edit diff touching outside those boundaries is a sign of accidental damage.
5. **Operator checkpoint:** surface the inventory + boundaries to the operator BEFORE editing. Do not start editing until reviewed. This is the cheapest place to catch "your delta scope is wider than I thought" or "this attribute reference doesn't exist."

## During edit — atomic, verified, individually committed

1. One delta = one edit cycle = one commit. The cycle:
   a. **Render before**: produce the rendered output of the affected file via the project's AsciiDoc render path. Save baseline.
   b. **Apply delta** as a precise old_string → new_string edit. No `replace_all`. No regex sweeps. Exact string substitution only.
   c. **Render after**: produce the rendered output again. Diff against baseline.
   d. **Verify the rendered diff**: only the intended change appears; no raw `{attribute}` markers in output (would indicate broken ref); no dangling anchor warnings; the surrounding paragraphs still read coherently (read them, don't just diff them).
   e. **Notch the delta**: `jjx_record` with the specific file and an intent describing this single delta and its rationale. One delta per commit.
2. If a delta requires touching multiple files atomically (e.g., RBSSS prose + RBS0 mapping section addition), keep atomic but treat as a single delta — one cycle, one render-diff, one notch.
3. Do NOT batch deltas. Each delta completes the full cycle before the next begins.
4. **Forbidden: prose invention beyond the deltas list.** If a delta seems to require adding a sentence not in the anchor memo's deltas — STOP. Surface as a question to the operator. "I needed to add a sentence to make the flow work" is precisely the class of decision that wallowed in the prior chat.

## After all deltas — sweep verification

1. Render the entire RBS0-SpecTop.adoc transitive include tree.
2. For each spec file edited, diff the rendered output against the pre-edit baseline from step 1.a. Walk the diff:
   - In-scope changes are expected and match the deltas list.
   - Out-of-scope sections (the boundaries defined in pre-work step 4) MUST show zero differences. Any difference there is accidental damage; investigate before declaring done.
3. Run `tt/rbw-tf.TestFixture.regime-validation.sh` and any nameplate-validation tabtargets to confirm spec edits haven't broken downstream consumers (CLAUDE.md interpolation, generated tabtarget-context docs, etc.).
4. Produce a "deltas-applied verification" scratch document: for each delta in the anchor memo's list, cite the commit(s) that applied it. Any unapplied deltas get explicit justification or are marked as future-work for a follow-up pace.

## Independent-eyes check (recommended)

After all deltas land but before close: spawn a fresh Explore agent (no chat context) to read each edited spec file in isolation and report on architectural coherence. The fresh agent has no investment in the changes and will catch phrasing that's clearly fishy from outside the context that produced it. If the fresh agent flags anything, treat it as a wallow-avoidance signal — investigate before close.

## Wallow-avoidance discipline (the meta-rule)

When in doubt, do not improvise. The deltas list in the anchor memo is the contract. Deviations from it require explicit operator approval. The cost of one "should I do X?" question is small; the cost of one wrong improvised sentence in the source-of-truth spec is potentially weeks of confused downstream readers.

This chat demonstrated how plausible-but-wrong architectural moves wallow when committed without empirical check (Option E "drop source-CIDR" → empirical Ifrit catch hours later; topology reframe → empirical asymmetric-return catch hours later). The render-diff cycle in "During edit" above is the empirical check for spec edits. Trust it; do not skip it; do not batch deltas to "save time."

## References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — Spec alignment findings subsection contains the deltas list and per-delta rationale. This is the contract.
- `Memos/memo-20260512-srjcl-port-publish-regression.md` — companion lineage.
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` — MCM patterns for attribute references, anchors, linked terms. Consult before editing if any delta introduces new linked terms.
- ₢A_AAb landing commit — the canonical architecture this spec reflects.
- ₢A_AAc — if landed before this pace mounts, may have produced architectural changes (per its "If empirically allowed" branch); incorporate into the pre-work inventory.

### rekon-hallmark-catalog (₢A_AAQ) [complete]

**[260423-1355] complete**

## Character

Narrow endpoint-rewrite. The `rbfl_rekon` function (exposed as `rbw-ir` DirectorRekonsImages) still queries `<prefix><moniker>/tags/list` — the pre-AAK vessel-indexed package grammar that no longer exists after the categorical-layout migration. Discovered during ₢A_AAL's vessel-param-drop sweep but is orthogonal to vessel-param-drop scope; captured here as its own pace.

## Docket

Post-AAK there is no single "vessel package" to list tags for. The hallmark subtree `<prefix>hallmarks/<hallmark>/` contains basename packages (image, vouch, pouch, about, attest, diags), each with `<hallmark>` as its sole tag by the hallmark-as-tag discipline. The pre-migration `rbw-ir` semantic — "raw GAR tag listing for a vessel's package" — does not cleanly translate; there's no vessel-indexed container any more.

### Decision needed

Reconcile `rbw-ir`'s purpose with the post-AAK layout. Three viable shapes:

1. **Raw hallmark listing** — `rbw-ir` takes no arg and lists every hallmark in GAR as raw rows (hallmark, basenames-present). Parallel to `rbw-ft` (tally) but without health-state classification. Consumes `zrbfc_list_hallmarks_capture` (landed in ₢A_AAO, commit b5603dc6).

2. **Hallmark-indexed basename listing** — `rbw-ir <hallmark>` lists the package basenames under that hallmark's subtree. Director-scoped diagnostic for "what artifacts exist under this hallmark, ignoring health classification."

3. **Dissolve** — if `rbw-ft` (tally) plus direct GAR console inspection covers the use cases, delete `rbw-ir` entirely along with its RBZ_REKON_IMAGE enrollment and COLOPHON_REKON Rust constant.

Preference: option 2. Raw per-hallmark basename surfacing is useful for debugging partial-ark situations where a hallmark's vouch or about is missing and the user wants to see exactly which basenames are present without the tally's three-state classifier coloring the signal. Option 1 duplicates tally's scope.

### If option 2

Signature: `rbfl_rekon(hallmark)`. Queries GAR packages endpoint with subtree filter `<prefix>hallmarks/<hallmark>/`. Emits a three-column table (BASENAME, EXISTS-AT-HALLMARK-TAG, PACKAGE-PATH). Authenticates as Director (retains current credential posture). Drops the `curl <prefix><moniker>/tags/list` entirely.

### Cross-cutting

- `rbfl_rekon` signature change in rbfl_FoundryLedger.sh
- Zipper `RBZ_REKON_IMAGE` enrollment unchanged (channel already empty)
- Rust `RBTDRM_COLOPHON_REKON` constant unchanged (string-level stable)
- Tabtarget shim unchanged
- Docstring and Usage prose

### Verification

- `rbw-ir <hallmark>` against a known hallmark lists present basenames correctly
- Error path on missing hallmark returns buc_die with a diagnostic
- Fast suite 90/90 green (structural — rbfl module unit-tested via enrollment/regime validation)
- Crucible suite exercises `rbfl_rekon` if the four-mode fixture uses it (check with grep for RBTDRM_COLOPHON_REKON in rbtdrc_crucible.rs)

### Out of scope

- Jettison grammar rewrite — separate pace
- Health-state classification — belongs to tally
- Deeper GAR catalog restructuring — not contemplated

**[260423-1336] rough**

## Character

Narrow endpoint-rewrite. The `rbfl_rekon` function (exposed as `rbw-ir` DirectorRekonsImages) still queries `<prefix><moniker>/tags/list` — the pre-AAK vessel-indexed package grammar that no longer exists after the categorical-layout migration. Discovered during ₢A_AAL's vessel-param-drop sweep but is orthogonal to vessel-param-drop scope; captured here as its own pace.

## Docket

Post-AAK there is no single "vessel package" to list tags for. The hallmark subtree `<prefix>hallmarks/<hallmark>/` contains basename packages (image, vouch, pouch, about, attest, diags), each with `<hallmark>` as its sole tag by the hallmark-as-tag discipline. The pre-migration `rbw-ir` semantic — "raw GAR tag listing for a vessel's package" — does not cleanly translate; there's no vessel-indexed container any more.

### Decision needed

Reconcile `rbw-ir`'s purpose with the post-AAK layout. Three viable shapes:

1. **Raw hallmark listing** — `rbw-ir` takes no arg and lists every hallmark in GAR as raw rows (hallmark, basenames-present). Parallel to `rbw-ft` (tally) but without health-state classification. Consumes `zrbfc_list_hallmarks_capture` (landed in ₢A_AAO, commit b5603dc6).

2. **Hallmark-indexed basename listing** — `rbw-ir <hallmark>` lists the package basenames under that hallmark's subtree. Director-scoped diagnostic for "what artifacts exist under this hallmark, ignoring health classification."

3. **Dissolve** — if `rbw-ft` (tally) plus direct GAR console inspection covers the use cases, delete `rbw-ir` entirely along with its RBZ_REKON_IMAGE enrollment and COLOPHON_REKON Rust constant.

Preference: option 2. Raw per-hallmark basename surfacing is useful for debugging partial-ark situations where a hallmark's vouch or about is missing and the user wants to see exactly which basenames are present without the tally's three-state classifier coloring the signal. Option 1 duplicates tally's scope.

### If option 2

Signature: `rbfl_rekon(hallmark)`. Queries GAR packages endpoint with subtree filter `<prefix>hallmarks/<hallmark>/`. Emits a three-column table (BASENAME, EXISTS-AT-HALLMARK-TAG, PACKAGE-PATH). Authenticates as Director (retains current credential posture). Drops the `curl <prefix><moniker>/tags/list` entirely.

### Cross-cutting

- `rbfl_rekon` signature change in rbfl_FoundryLedger.sh
- Zipper `RBZ_REKON_IMAGE` enrollment unchanged (channel already empty)
- Rust `RBTDRM_COLOPHON_REKON` constant unchanged (string-level stable)
- Tabtarget shim unchanged
- Docstring and Usage prose

### Verification

- `rbw-ir <hallmark>` against a known hallmark lists present basenames correctly
- Error path on missing hallmark returns buc_die with a diagnostic
- Fast suite 90/90 green (structural — rbfl module unit-tested via enrollment/regime validation)
- Crucible suite exercises `rbfl_rekon` if the four-mode fixture uses it (check with grep for RBTDRM_COLOPHON_REKON in rbtdrc_crucible.rs)

### Out of scope

- Jettison grammar rewrite — separate pace
- Health-state classification — belongs to tally
- Deeper GAR catalog restructuring — not contemplated

### jettison-locator-grammar (₢A_AAR) [complete]

**[260423-1355] complete**

## Character

Narrow parsing-and-endpoint rewrite. The `rbfl_jettison` function (exposed as `rbw-iJ` DirectorJettisonsImage) still parses its locator as `moniker:tag` with first-colon split and hits `<prefix><moniker>/manifests/<tag>` — pre-AAK grammar that no longer matches live packages. Discovered during ₢A_AAL's vessel-param-drop sweep but is orthogonal to vessel-param-drop scope; captured here as its own pace.

## Docket

Post-AAK, package paths contain colons in the semantic sense only as the `<path>:<tag>` separator; the `<path>` itself is `hallmarks/<hallmark>/<basename>` or `reliquaries/<date>/<tool>` or `enshrines/<anchor>` — all colon-free. So first-colon-split would still technically work for parsing a locator if callers supply the new grammar. But the constructed URL `<prefix><moniker>/manifests/<tag>` treats `moniker` as a bare top-level package name — which works for enshrines (`enshrines/<anchor>`) but mis-targets hallmarks and reliquaries because the package path has slashes.

### What needs to change

1. **Locator parsing** — use last-colon split (same idiom `rbfr_wrest` adopted in AAK Notch 2a) so `path/with/slashes:tag` parses correctly. Current `z_moniker="${z_locator%%:*}"` and `z_tag="${z_locator#*:}"` splits on first colon; need `z_pkg_path="${z_locator%:*}"` and `z_tag="${z_locator##*:}"`.

2. **URL construction** — change `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_moniker}/manifests/${z_tag}` to `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_pkg_path}/manifests/${z_tag}`. The package path already contains whatever sub-namespacing is needed; the prefix prepends once.

3. **Docstring example** — current `rbev-busybox:20251231T160211Z-img` is pre-AAK. Replace with post-AAK example: `hallmarks/c260305133650-r260305160530/image:c260305133650-r260305160530` (or a shortened realistic form).

4. **Validation prose** — error messages reference "moniker" (pre-AAK concept); update to "package path" to match the new semantic.

### Cross-cutting

- `rbfl_jettison` body edit in rbfl_FoundryLedger.sh
- Zipper `RBZ_JETTISON_IMAGE` enrollment unchanged (channel already empty)
- Rust `RBTDRM_COLOPHON_JETTISON` constant and call sites — check for any that pass pre-AAK locator strings
- Tabtarget shim unchanged

### Relationship to abjure

`rbw-fA` (abjure) deletes every package under a hallmark subtree (wholesale cleanup of one hallmark). `rbw-iJ` (jettison) deletes a single tag from a single package (surgical cleanup of a stray artifact). They don't overlap functionally — abjure is hallmark-scoped, jettison is manifest-scoped. Keep both.

### Verification

- `rbw-iJ hallmarks/H/image:H --force` against a known single-ark succeeds (or fails diagnostically if the ark is missing)
- First-colon-containing paths parse correctly (e.g., a future reliquary path like `reliquaries/260305:260305` if that ever happens)
- bash -n clean on rbfl_FoundryLedger.sh
- Fast suite 90/90 green (structural)
- Crucible suite exercises path if the four-mode fixture uses jettison (grep rbtdrc_crucible.rs)

### Out of scope

- Rekon rewrite — separate pace (₢A_AAQ)
- Broader locator-format standardization across the foundry CLI — not contemplated

**[260423-1337] rough**

## Character

Narrow parsing-and-endpoint rewrite. The `rbfl_jettison` function (exposed as `rbw-iJ` DirectorJettisonsImage) still parses its locator as `moniker:tag` with first-colon split and hits `<prefix><moniker>/manifests/<tag>` — pre-AAK grammar that no longer matches live packages. Discovered during ₢A_AAL's vessel-param-drop sweep but is orthogonal to vessel-param-drop scope; captured here as its own pace.

## Docket

Post-AAK, package paths contain colons in the semantic sense only as the `<path>:<tag>` separator; the `<path>` itself is `hallmarks/<hallmark>/<basename>` or `reliquaries/<date>/<tool>` or `enshrines/<anchor>` — all colon-free. So first-colon-split would still technically work for parsing a locator if callers supply the new grammar. But the constructed URL `<prefix><moniker>/manifests/<tag>` treats `moniker` as a bare top-level package name — which works for enshrines (`enshrines/<anchor>`) but mis-targets hallmarks and reliquaries because the package path has slashes.

### What needs to change

1. **Locator parsing** — use last-colon split (same idiom `rbfr_wrest` adopted in AAK Notch 2a) so `path/with/slashes:tag` parses correctly. Current `z_moniker="${z_locator%%:*}"` and `z_tag="${z_locator#*:}"` splits on first colon; need `z_pkg_path="${z_locator%:*}"` and `z_tag="${z_locator##*:}"`.

2. **URL construction** — change `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_moniker}/manifests/${z_tag}` to `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_pkg_path}/manifests/${z_tag}`. The package path already contains whatever sub-namespacing is needed; the prefix prepends once.

3. **Docstring example** — current `rbev-busybox:20251231T160211Z-img` is pre-AAK. Replace with post-AAK example: `hallmarks/c260305133650-r260305160530/image:c260305133650-r260305160530` (or a shortened realistic form).

4. **Validation prose** — error messages reference "moniker" (pre-AAK concept); update to "package path" to match the new semantic.

### Cross-cutting

- `rbfl_jettison` body edit in rbfl_FoundryLedger.sh
- Zipper `RBZ_JETTISON_IMAGE` enrollment unchanged (channel already empty)
- Rust `RBTDRM_COLOPHON_JETTISON` constant and call sites — check for any that pass pre-AAK locator strings
- Tabtarget shim unchanged

### Relationship to abjure

`rbw-fA` (abjure) deletes every package under a hallmark subtree (wholesale cleanup of one hallmark). `rbw-iJ` (jettison) deletes a single tag from a single package (surgical cleanup of a stray artifact). They don't overlap functionally — abjure is hallmark-scoped, jettison is manifest-scoped. Keep both.

### Verification

- `rbw-iJ hallmarks/H/image:H --force` against a known single-ark succeeds (or fails diagnostically if the ark is missing)
- First-colon-containing paths parse correctly (e.g., a future reliquary path like `reliquaries/260305:260305` if that ever happens)
- bash -n clean on rbfl_FoundryLedger.sh
- Fast suite 90/90 green (structural)
- Crucible suite exercises path if the four-mode fixture uses jettison (grep rbtdrc_crucible.rs)

### Out of scope

- Rekon rewrite — separate pace (₢A_AAQ)
- Broader locator-format standardization across the foundry CLI — not contemplated

### rename-rbap-to-rbgv (₢A_AAI) [complete]

**[260423-0747] complete**

Drafted from ₢A6AAi in ₣A6.

## Character
Mechanical rename — straightforward but touches many files. Grep-driven, no design judgment needed.

## Docket
Rename `rbap` (Access Probe) to `rbgv` (Google Verification) to free the `rba` prefix for future Amazon features.

### BCG naming contract (all must transform)

| Element | BCG Pattern | Old | New |
|---------|------------|-----|-----|
| Implementation file | `«prefix»_«name».sh` | `rbap_AccessProbe.sh` | `rbgv_AccessProbe.sh` |
| Source guard | `Z«PREFIX»_SOURCED` | `ZRBAP_SOURCED` | `ZRBGV_SOURCED` |
| Kindle guard | `Z«PREFIX»_KINDLED` | `ZRBAP_KINDLED` | `ZRBGV_KINDLED` |
| Kindle function | `z«prefix»_kindle()` | `zrbap_kindle()` | `zrbgv_kindle()` |
| Sentinel function | `z«prefix»_sentinel()` | `zrbap_sentinel()` | `zrbgv_sentinel()` |
| Public functions | `«prefix»_*()` | `rbap_*()` | `rbgv_*()` |
| Internal functions | `z«prefix»_*()` | `zrbap_*()` | `zrbgv_*()` |
| Internal constants | `Z«PREFIX»_*` | `ZRBAP_*` | `ZRBGV_*` |
| Temp file paths | `«prefix»_*` in strings | `rbap_` | `rbgv_` |

### Note: no standalone CLI
`rbap` has no `rbap_cli.sh` — it is a library module sourced by `rbte_cli.sh` (theurge test engine). No CLI file to rename. The `rbte_cli.sh` furnish sources it directly and calls `zrbap_kindle` — both references must update.

### Files to modify (exhaustive)
1. `Tools/rbk/rbap_AccessProbe.sh` — rename file, transform all internals per table above
2. `Tools/rbk/rbtd/rbte_cli.sh` — update source path and `zrbap_kindle` call
3. `Tools/rbk/rbtd/rbte_engine.sh` — update `rbap_jwt_sa_probe` and `rbap_payor_oauth_probe` calls
4. `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — update linked term display text referencing `rbap_*` function names
5. `CLAUDE.md` — no RBAP entry exists today; add RBGV entry to acronym mappings

### Verification
- Grep entire repo for any remaining `rbap` / `RBAP` / `zrbap` / `ZRBAP` (retired heat files in `.claude/jjm/retired/` are historical and do not need updating)
- Run fast test suite to confirm no regressions

**[260415-1507] rough**

Drafted from ₢A6AAi in ₣A6.

## Character
Mechanical rename — straightforward but touches many files. Grep-driven, no design judgment needed.

## Docket
Rename `rbap` (Access Probe) to `rbgv` (Google Verification) to free the `rba` prefix for future Amazon features.

### BCG naming contract (all must transform)

| Element | BCG Pattern | Old | New |
|---------|------------|-----|-----|
| Implementation file | `«prefix»_«name».sh` | `rbap_AccessProbe.sh` | `rbgv_AccessProbe.sh` |
| Source guard | `Z«PREFIX»_SOURCED` | `ZRBAP_SOURCED` | `ZRBGV_SOURCED` |
| Kindle guard | `Z«PREFIX»_KINDLED` | `ZRBAP_KINDLED` | `ZRBGV_KINDLED` |
| Kindle function | `z«prefix»_kindle()` | `zrbap_kindle()` | `zrbgv_kindle()` |
| Sentinel function | `z«prefix»_sentinel()` | `zrbap_sentinel()` | `zrbgv_sentinel()` |
| Public functions | `«prefix»_*()` | `rbap_*()` | `rbgv_*()` |
| Internal functions | `z«prefix»_*()` | `zrbap_*()` | `zrbgv_*()` |
| Internal constants | `Z«PREFIX»_*` | `ZRBAP_*` | `ZRBGV_*` |
| Temp file paths | `«prefix»_*` in strings | `rbap_` | `rbgv_` |

### Note: no standalone CLI
`rbap` has no `rbap_cli.sh` — it is a library module sourced by `rbte_cli.sh` (theurge test engine). No CLI file to rename. The `rbte_cli.sh` furnish sources it directly and calls `zrbap_kindle` — both references must update.

### Files to modify (exhaustive)
1. `Tools/rbk/rbap_AccessProbe.sh` — rename file, transform all internals per table above
2. `Tools/rbk/rbtd/rbte_cli.sh` — update source path and `zrbap_kindle` call
3. `Tools/rbk/rbtd/rbte_engine.sh` — update `rbap_jwt_sa_probe` and `rbap_payor_oauth_probe` calls
4. `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — update linked term display text referencing `rbap_*` function names
5. `CLAUDE.md` — no RBAP entry exists today; add RBGV entry to acronym mappings

### Verification
- Grep entire repo for any remaining `rbap` / `RBAP` / `zrbap` / `ZRBAP` (retired heat files in `.claude/jjm/retired/` are historical and do not need updating)
- Run fast test suite to confirm no regressions

**[260415-1318] rough**

## Character
Mechanical rename — straightforward but touches many files. Grep-driven, no design judgment needed.

## Docket
Rename `rbap` (Access Probe) to `rbgv` (Google Verification) to free the `rba` prefix for future Amazon features.

### BCG naming contract (all must transform)

| Element | BCG Pattern | Old | New |
|---------|------------|-----|-----|
| Implementation file | `«prefix»_«name».sh` | `rbap_AccessProbe.sh` | `rbgv_AccessProbe.sh` |
| Source guard | `Z«PREFIX»_SOURCED` | `ZRBAP_SOURCED` | `ZRBGV_SOURCED` |
| Kindle guard | `Z«PREFIX»_KINDLED` | `ZRBAP_KINDLED` | `ZRBGV_KINDLED` |
| Kindle function | `z«prefix»_kindle()` | `zrbap_kindle()` | `zrbgv_kindle()` |
| Sentinel function | `z«prefix»_sentinel()` | `zrbap_sentinel()` | `zrbgv_sentinel()` |
| Public functions | `«prefix»_*()` | `rbap_*()` | `rbgv_*()` |
| Internal functions | `z«prefix»_*()` | `zrbap_*()` | `zrbgv_*()` |
| Internal constants | `Z«PREFIX»_*` | `ZRBAP_*` | `ZRBGV_*` |
| Temp file paths | `«prefix»_*` in strings | `rbap_` | `rbgv_` |

### Note: no standalone CLI
`rbap` has no `rbap_cli.sh` — it is a library module sourced by `rbte_cli.sh` (theurge test engine). No CLI file to rename. The `rbte_cli.sh` furnish sources it directly and calls `zrbap_kindle` — both references must update.

### Files to modify (exhaustive)
1. `Tools/rbk/rbap_AccessProbe.sh` — rename file, transform all internals per table above
2. `Tools/rbk/rbtd/rbte_cli.sh` — update source path and `zrbap_kindle` call
3. `Tools/rbk/rbtd/rbte_engine.sh` — update `rbap_jwt_sa_probe` and `rbap_payor_oauth_probe` calls
4. `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — update linked term display text referencing `rbap_*` function names
5. `CLAUDE.md` — no RBAP entry exists today; add RBGV entry to acronym mappings

### Verification
- Grep entire repo for any remaining `rbap` / `RBAP` / `zrbap` / `ZRBAP` (retired heat files in `.claude/jjm/retired/` are historical and do not need updating)
- Run fast test suite to confirm no regressions

**[260415-1316] rough**

## Character
Mechanical rename — straightforward but touches many files. Grep-driven, no design judgment needed.

## Docket
Rename `rbap` (Access Probe) to `rbgv` (Google Verification) to free the `rba` prefix for future Amazon features.

### What moves
- `Tools/rbk/rbap_AccessProbe.sh` → `Tools/rbk/rbgv_AccessProbe.sh`
- All public functions: `rbap_*` → `rbgv_*`
- All private functions: `zrbap_*` → `zrbgv_*`
- All constants: `ZRBAP_*` → `ZRBGV_*`
- Source guard variable: `ZRBAP_SOURCED` → `ZRBGV_SOURCED`
- Kindle guard: `ZRBAP_KINDLED` → `ZRBGV_KINDLED`

### Scope of changes
1. The file itself (rename + all internal references)
2. CLI file that sources it (find via grep for `rbap`)
3. Workbench/zipper enrollment referencing `rbap`
4. Any tabtargets referencing `rbap` functions
5. Spec documents (`vov_veiled/RBSAJ-*.adoc`, `RBSAO-*.adoc`) if they reference `rbap` identifiers
6. CLAUDE.md acronym mappings — remove RBAP, add RBGV
7. Grep entire repo for `rbap` / `RBAP` / `zrbap` / `ZRBAP` to catch stragglers

### Verification
- Run `tt/rbw-rav.ValidateAuthRegime.sh` (or equivalent) to confirm probes still work
- Run fast test suite

### rbrr-image-and-runtime-prefix-variables (₢A_AAA) [complete]

**[260423-0935] complete**

## Character
Mechanical plumbing — add two new RBRR variables and wire through the enrollment / enforce / render machinery.

## Docket
Add `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` to the RBRR regime.

**`RBRR_CLOUD_PREFIX`** — prepended to all cloud-visible resource names: GAR image paths, vessel package names, Cloud Build substitutions, depot-scoped identifiers. Everything that lives in the GCP project.

**`RBRR_RUNTIME_PREFIX`** — prepended to local container names, Docker network names, and other machine-local resources. Enables multi-regime isolation on a single developer workstation.

### Real module shape (verify before coding)

The RBRR regime is split across two files. Enrollment is the single source of truth; `rbrr_validate` and `rbrr_render` are thin CLI wrappers around `buv_report` / `buv_render` that pick up enrolled variables automatically.

| File | Relevant function | Role |
|------|------------------|------|
| `Tools/rbk/rbrr_regime.sh` | `zrbrr_kindle` | enrollment declarations (`buv_*_enroll` calls), scope sentinel, lock |
| `Tools/rbk/rbrr_regime.sh` | `zrbrr_enforce` | `buv_vet` + custom regex / path checks beyond enroll types |
| `Tools/rbk/rbrr_cli.sh` | `rbrr_validate` | `buv_report RBRR "Repository Regime"` — automatic, no edit needed |
| `Tools/rbk/rbrr_cli.sh` | `rbrr_render` | `buv_render RBRR "RBRR - Recipe Bottle Regime Repo"` — automatic, no edit needed |

### Length / format policy

- **Min length: 0** (empty is valid — represents "no prefix"). Enforced by `buv_string_enroll` min arg of 0.
- **Max length: 11** characters. Allows realistic dev-team prefixes like `production-` or `bhyslop-` while keeping full image paths readable in terminal output.
- **Format (when non-empty):** lowercase letter start, lowercase/digit/hyphen body, ending in hyphen. Regex: `^$|^[a-z][a-z0-9-]*-$`. The regex layers atop buv length: empty OR ≥2 chars ending in hyphen.

### Changes

1. **`Tools/rbk/rbrr_regime.sh` — `zrbrr_kindle`:**
   Add a new group `"Resource Prefixing"` (sibling to existing groups). Enroll both variables with `buv_string_enroll`, allowing empty as valid:
   ```
   buv_group_enroll "Resource Prefixing"
   buv_string_enroll  RBRR_CLOUD_PREFIX    0  11  "Prefix prepended to cloud-visible resource names (GAR, Cloud Build, GCP)"
   buv_string_enroll  RBRR_RUNTIME_PREFIX  0  11  "Prefix prepended to local container and network names"
   ```

2. **`Tools/rbk/rbrr_regime.sh` — `zrbrr_enforce`:**
   Add custom regex check mirroring the existing `RBRR_GCB_POOL_STEM` pattern:
   ```
   [[ "${RBRR_CLOUD_PREFIX}"   =~ ^$|^[a-z][a-z0-9-]*-$ ]] \
     || buc_die "Invalid RBRR_CLOUD_PREFIX format: ${RBRR_CLOUD_PREFIX}   (expected empty or lowercase ending in hyphen)"
   [[ "${RBRR_RUNTIME_PREFIX}" =~ ^$|^[a-z][a-z0-9-]*-$ ]] \
     || buc_die "Invalid RBRR_RUNTIME_PREFIX format: ${RBRR_RUNTIME_PREFIX} (expected empty or lowercase ending in hyphen)"
   ```

3. **`Tools/rbk/rbrr_cli.sh`** — no edit needed. `buv_report` and `buv_render` discover new enrollments automatically.

4. **`Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc`** — document both variables: purpose, format rule (0–11 chars, lowercase, trailing hyphen if non-empty), empty-default behavior, and the cloud-vs-runtime split that ₢A_AAB will consume.

5. **On-disk regime files** — add both new variables (defaulted to empty string) to keep enforce green:
   - `.rbk/rbrr.env` — the only on-disk rbrr.env in the repo. Test fixtures in `rbtdrf_fast.rs:768,915,938,1083` source from this same file.

   No other rbrr.env files exist (verified by `find . -name rbrr.env`). Marshal-zero template emission of the new vars is ₢A_AAC's scope.

### Verification

- `tt/rbw-rrv.ValidateRepoRegime.sh` — validate passes with empty prefixes
- `tt/rbw-rrr.RenderRepoRegime.sh` — both new vars appear under the "Resource Prefixing" group
- Set each to a valid sample (`test-`), confirm validate still passes
- Set each to an invalid sample (`bad`, `TOOLONG12345-`, `trailing` without hyphen), confirm enforce dies with the expected message
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime-validation exercises the enrollment machinery)

### Out of scope

- Wiring the prefixes into derived constants and consumers (that is ₢A_AAB)
- Marshal-zero template changes (that is ₢A_AAC)
- Any depot-side changes (that is ₢A_AAE)

**[260423-0901] rough**

## Character
Mechanical plumbing — add two new RBRR variables and wire through the enrollment / enforce / render machinery.

## Docket
Add `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` to the RBRR regime.

**`RBRR_CLOUD_PREFIX`** — prepended to all cloud-visible resource names: GAR image paths, vessel package names, Cloud Build substitutions, depot-scoped identifiers. Everything that lives in the GCP project.

**`RBRR_RUNTIME_PREFIX`** — prepended to local container names, Docker network names, and other machine-local resources. Enables multi-regime isolation on a single developer workstation.

### Real module shape (verify before coding)

The RBRR regime is split across two files. Enrollment is the single source of truth; `rbrr_validate` and `rbrr_render` are thin CLI wrappers around `buv_report` / `buv_render` that pick up enrolled variables automatically.

| File | Relevant function | Role |
|------|------------------|------|
| `Tools/rbk/rbrr_regime.sh` | `zrbrr_kindle` | enrollment declarations (`buv_*_enroll` calls), scope sentinel, lock |
| `Tools/rbk/rbrr_regime.sh` | `zrbrr_enforce` | `buv_vet` + custom regex / path checks beyond enroll types |
| `Tools/rbk/rbrr_cli.sh` | `rbrr_validate` | `buv_report RBRR "Repository Regime"` — automatic, no edit needed |
| `Tools/rbk/rbrr_cli.sh` | `rbrr_render` | `buv_render RBRR "RBRR - Recipe Bottle Regime Repo"` — automatic, no edit needed |

### Length / format policy

- **Min length: 0** (empty is valid — represents "no prefix"). Enforced by `buv_string_enroll` min arg of 0.
- **Max length: 11** characters. Allows realistic dev-team prefixes like `production-` or `bhyslop-` while keeping full image paths readable in terminal output.
- **Format (when non-empty):** lowercase letter start, lowercase/digit/hyphen body, ending in hyphen. Regex: `^$|^[a-z][a-z0-9-]*-$`. The regex layers atop buv length: empty OR ≥2 chars ending in hyphen.

### Changes

1. **`Tools/rbk/rbrr_regime.sh` — `zrbrr_kindle`:**
   Add a new group `"Resource Prefixing"` (sibling to existing groups). Enroll both variables with `buv_string_enroll`, allowing empty as valid:
   ```
   buv_group_enroll "Resource Prefixing"
   buv_string_enroll  RBRR_CLOUD_PREFIX    0  11  "Prefix prepended to cloud-visible resource names (GAR, Cloud Build, GCP)"
   buv_string_enroll  RBRR_RUNTIME_PREFIX  0  11  "Prefix prepended to local container and network names"
   ```

2. **`Tools/rbk/rbrr_regime.sh` — `zrbrr_enforce`:**
   Add custom regex check mirroring the existing `RBRR_GCB_POOL_STEM` pattern:
   ```
   [[ "${RBRR_CLOUD_PREFIX}"   =~ ^$|^[a-z][a-z0-9-]*-$ ]] \
     || buc_die "Invalid RBRR_CLOUD_PREFIX format: ${RBRR_CLOUD_PREFIX}   (expected empty or lowercase ending in hyphen)"
   [[ "${RBRR_RUNTIME_PREFIX}" =~ ^$|^[a-z][a-z0-9-]*-$ ]] \
     || buc_die "Invalid RBRR_RUNTIME_PREFIX format: ${RBRR_RUNTIME_PREFIX} (expected empty or lowercase ending in hyphen)"
   ```

3. **`Tools/rbk/rbrr_cli.sh`** — no edit needed. `buv_report` and `buv_render` discover new enrollments automatically.

4. **`Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc`** — document both variables: purpose, format rule (0–11 chars, lowercase, trailing hyphen if non-empty), empty-default behavior, and the cloud-vs-runtime split that ₢A_AAB will consume.

5. **On-disk regime files** — add both new variables (defaulted to empty string) to keep enforce green:
   - `.rbk/rbrr.env` — the only on-disk rbrr.env in the repo. Test fixtures in `rbtdrf_fast.rs:768,915,938,1083` source from this same file.

   No other rbrr.env files exist (verified by `find . -name rbrr.env`). Marshal-zero template emission of the new vars is ₢A_AAC's scope.

### Verification

- `tt/rbw-rrv.ValidateRepoRegime.sh` — validate passes with empty prefixes
- `tt/rbw-rrr.RenderRepoRegime.sh` — both new vars appear under the "Resource Prefixing" group
- Set each to a valid sample (`test-`), confirm validate still passes
- Set each to an invalid sample (`bad`, `TOOLONG12345-`, `trailing` without hyphen), confirm enforce dies with the expected message
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime-validation exercises the enrollment machinery)

### Out of scope

- Wiring the prefixes into derived constants and consumers (that is ₢A_AAB)
- Marshal-zero template changes (that is ₢A_AAC)
- Any depot-side changes (that is ₢A_AAE)

**[260423-0747] rough**

## Character
Mechanical plumbing — add two new RBRR variables and wire through the enrollment / enforce / render machinery.

## Docket
Add `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` to the RBRR regime.

**`RBRR_CLOUD_PREFIX`** — prepended to all cloud-visible resource names: GAR image paths, vessel package names, Cloud Build substitutions, depot-scoped identifiers. Everything that lives in the GCP project.

**`RBRR_RUNTIME_PREFIX`** — prepended to local container names, Docker network names, and other machine-local resources. Enables multi-regime isolation on a single developer workstation.

### Real module shape (verify before coding)

The RBRR regime is split across two files — there are no `rbkro_*` or `rbkrr_*` functions (phantom names from prior draft). Enrollment is the single source of truth; `rbrr_validate` and `rbrr_render` are thin CLI wrappers around `buv_report` / `buv_render` that pick up enrolled variables automatically.

| File | Relevant function | Role |
|------|------------------|------|
| `Tools/rbk/rbrr_regime.sh` | `zrbrr_kindle` | enrollment declarations (`buv_*_enroll` calls), scope sentinel, lock |
| `Tools/rbk/rbrr_regime.sh` | `zrbrr_enforce` | `buv_vet` + custom regex / path checks beyond enroll types |
| `Tools/rbk/rbrr_cli.sh` | `rbrr_validate` | `buv_report RBRR "Repository Regime"` — automatic, no edit needed |
| `Tools/rbk/rbrr_cli.sh` | `rbrr_render` | `buv_render RBRR "RBRR - Recipe Bottle Regime Repo"` — automatic, no edit needed |

### Changes

1. **`Tools/rbk/rbrr_regime.sh` — `zrbrr_kindle`:**
   Add a new group `"Resource Prefixing"` (sibling to the existing groups). Enroll both variables with `buv_string_enroll`, allowing empty as valid:
   ```
   buv_group_enroll "Resource Prefixing"
   buv_string_enroll  RBRR_CLOUD_PREFIX    0  6  "Prefix prepended to cloud-visible resource names (GAR, Cloud Build, GCP)"
   buv_string_enroll  RBRR_RUNTIME_PREFIX  0  6  "Prefix prepended to local container and network names"
   ```

2. **`Tools/rbk/rbrr_regime.sh` — `zrbrr_enforce`:**
   Add custom regex check mirroring the existing `RBRR_GCB_POOL_STEM` pattern — empty OR lowercase/digit/hyphen ending in hyphen:
   ```
   [[ "${RBRR_CLOUD_PREFIX}"   =~ ^$|^[a-z][a-z0-9-]*-$ ]] \
     || buc_die "Invalid RBRR_CLOUD_PREFIX format: ${RBRR_CLOUD_PREFIX}   (expected empty or lowercase ending in hyphen)"
   [[ "${RBRR_RUNTIME_PREFIX}" =~ ^$|^[a-z][a-z0-9-]*-$ ]] \
     || buc_die "Invalid RBRR_RUNTIME_PREFIX format: ${RBRR_RUNTIME_PREFIX} (expected empty or lowercase ending in hyphen)"
   ```

3. **`Tools/rbk/rbrr_cli.sh`** — no edit needed. `buv_report` and `buv_render` discover new enrollments automatically.

4. **`Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc`** — document both variables: purpose, format rule (0-6 chars, lowercase, trailing hyphen if non-empty), empty-default behavior, and the cloud-vs-runtime split that ₢A_AAB will consume.

5. **Repo regime files** — add both new variables (defaulted to empty string) to any on-disk RBRR regime files used by tests or default templates, so enforce passes. Marshal-zero template updates are ₢A_AAC's scope; this pace only needs whatever keeps fast-qualify green.

### Verification

- `tt/rbw-rrv.ValidateRepoRegime.sh` — validate passes with empty prefixes
- `tt/rbw-rrr.RenderRepoRegime.sh` — both new vars appear under the "Resource Prefixing" group
- Set each to a valid sample (`test-`), confirm validate still passes
- Set each to an invalid sample (`bad`, `TOOLONG-`, `trailing` without hyphen), confirm enforce dies with the expected message
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime-validation exercises the enrollment machinery)

### Out of scope

- Wiring the prefixes into derived constants and consumers (that is ₢A_AAB)
- Marshal-zero template changes (that is ₢A_AAC)
- Any depot-side changes (that is ₢A_AAE)

**[260415-1525] rough**

## Character
Mechanical plumbing — add two new RBRR variables and wire through kindle/validate/render.

## Docket
Add `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` to RBRR regime.

**`RBRR_CLOUD_PREFIX`** — prepended to all cloud-visible resource names: GAR image paths, vessel package names, Cloud Build substitutions, depot-scoped identifiers. Everything that lives in the GCP project.

**`RBRR_RUNTIME_PREFIX`** — prepended to local container names, Docker network names, and other machine-local resources. Enables multi-regime isolation on a single developer workstation.

Update RBSRR spec with variable definitions. Wire through `rbkro_kindle` (default to empty string), `rbkro_validate` (xname format, max 6 chars, must end with hyphen if non-empty), `rbkro_render` (display in Container Runtime Configuration section). Add to ZRBRR_ROLLUP serialization.

**[260415-1504] rough**

## Character
Mechanical plumbing — add two new RBRR variables and wire through kindle/validate/render.

## Work
Add `RBRR_IMAGE_PREFIX` and `RBRR_RUNTIME_PREFIX` to RBRR regime. Update RBSRR spec with variable definitions. Wire through `rbkro_kindle` (default to empty string), `rbkro_validate` (xname format, max 6 chars, must end with hyphen if non-empty), `rbkro_render` (display in Container Runtime Configuration section). Add to ZRBRR_ROLLUP serialization.

### derived-constants-and-consumer-wiring (₢A_AAB) [complete]

**[260423-1006] complete**

## Character
Mechanical breadth — touches container/network names, image paths, and compose interpolation across 6+ modules. Every site that constructs a container name or image reference must pick up the prefix variables. Risk: silent miss of a hardcoded site that bypasses the constants.

## Docket

Wire `RBRR_RUNTIME_PREFIX` into local container/network names and `RBRR_CLOUD_PREFIX` into GAR image paths and Cloud Build substitutions. Verify compose.yml interpolates the prefixed variables correctly.

### Runtime-prefix wiring (container/network names)

`Tools/rbk/rbob_bottle.sh` — `zrbob_kindle()` lines 104–109:
```
readonly ZRBOB_SENTRY="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}-sentry"
readonly ZRBOB_PENTACLE="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}-pentacle"
readonly ZRBOB_BOTTLE="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}-bottle"
readonly ZRBOB_NETWORK="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}_enclave"
```
Network uses underscore separator (Docker network rule); the hyphen-trailed prefix concatenates cleanly against either character.

### Cloud-prefix wiring (image paths)

Every site currently constructing `${z_gar_base}/${...VESSEL}:${...HALLMARK}${RBGC_ARK_SUFFIX_*}` prepends `${RBRR_CLOUD_PREFIX}` to the GAR-internal path segment. Concrete sites (lines from current grep — verify on touch):

| File | Lines | Sites |
|------|-------|-------|
| `rbob_bottle.sh` | 126–131 | `ZRBOB_SENTRY_IMAGE`, `ZRBOB_BOTTLE_IMAGE`, `ZRBOB_SENTRY_VOUCH`, `ZRBOB_BOTTLE_VOUCH` |
| `rbrn_cli.sh` | 55–56 | render-time image refs (`z_sentry_img`, `z_bottle_img`) |
| `rbfr_FoundryRetriever.sh` | 150, 171–173 | retriever pull paths |
| `rbfv_FoundryVerify.sh` | 75, 130, 154, 212, 395–396, 605, 629 + jq args 317–321, 510–512, 729–732 | verify reads |
| `rbfc_FoundryCore.sh` | 654, 681, 691, 705 | core foundry path construction |
| `rbfd_FoundryDirectorBuild.sh` | 602, 733, 1178–1180, 1258–1259, 1369–1370, 1488 | producer-side path construction + jq args |
| `rbga_ArtifactRegistry.sh` | (full file scan) | GAR API wrappers |

**Note:** AAK reshapes these paths entirely (basenames replace hyphen-suffix). The AAB→AAK relationship is sequential: AAB threads the prefix through current paths; AAK then reshapes the path grammar. Doing both at once risks losing track of which change is which. Land AAB first, run fast suite green, then AAK.

### Compose interpolation

`.rbk/rbob_compose.yml:23,57,77` currently hardcodes:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```
Update to prepend `${RBRR_CLOUD_PREFIX}`:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```
(AAK will further reshape this when the path grammar changes; AAB just inserts the prefix into the existing shape.)

Verify `RBRR_CLOUD_PREFIX` is exported into compose's environment (likely already is via `--env-file`, but confirm in `zrbob_compose()` at `rbob_bottle.sh:187`).

### Grep sweep for hardcoded sites

- `grep -rnE 'docker\.pkg\.dev|gcr\.io' Tools/ .rbk/` — find any hardcoded GAR hostnames/paths bypassing the constants
- `grep -rnE '"\$\{RBRN_MONIKER\}-(sentry|pentacle|bottle)"' Tools/` — find any container-name reconstruction outside RBOB
- `grep -rnE 'enclave"' Tools/` — find any network-name reconstruction outside RBOB

Any hits are candidates for prefix-wiring or fold-into-helper.

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime-validation exercises new var wiring)
- `tt/rbtd-s.TestSuite.crucible.sh` — 117/117 green (exercises actual container/network names with empty prefix)
- Render a nameplate with empty prefixes: container/network names match current shape exactly
- Render with `RBRR_RUNTIME_PREFIX="test-"`: container names become `test-tadmor-sentry`, network becomes `test-tadmor_enclave`
- Render with `RBRR_CLOUD_PREFIX="test-"`: image refs include `test-` between repo and vessel
- Manual: `tt/rbw-cC.Charge.tadmor.sh` succeeds with empty prefix, no behavior change

### Out of scope

- GAR layout grammar reshape (that is ₢A_AAK — basenames replace hyphen-suffix)
- Marshal-zero blank-template emission of new vars (that is ₢A_AAC)

**[260423-0858] rough**

## Character
Mechanical breadth — touches container/network names, image paths, and compose interpolation across 6+ modules. Every site that constructs a container name or image reference must pick up the prefix variables. Risk: silent miss of a hardcoded site that bypasses the constants.

## Docket

Wire `RBRR_RUNTIME_PREFIX` into local container/network names and `RBRR_CLOUD_PREFIX` into GAR image paths and Cloud Build substitutions. Verify compose.yml interpolates the prefixed variables correctly.

### Runtime-prefix wiring (container/network names)

`Tools/rbk/rbob_bottle.sh` — `zrbob_kindle()` lines 104–109:
```
readonly ZRBOB_SENTRY="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}-sentry"
readonly ZRBOB_PENTACLE="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}-pentacle"
readonly ZRBOB_BOTTLE="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}-bottle"
readonly ZRBOB_NETWORK="${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}_enclave"
```
Network uses underscore separator (Docker network rule); the hyphen-trailed prefix concatenates cleanly against either character.

### Cloud-prefix wiring (image paths)

Every site currently constructing `${z_gar_base}/${...VESSEL}:${...HALLMARK}${RBGC_ARK_SUFFIX_*}` prepends `${RBRR_CLOUD_PREFIX}` to the GAR-internal path segment. Concrete sites (lines from current grep — verify on touch):

| File | Lines | Sites |
|------|-------|-------|
| `rbob_bottle.sh` | 126–131 | `ZRBOB_SENTRY_IMAGE`, `ZRBOB_BOTTLE_IMAGE`, `ZRBOB_SENTRY_VOUCH`, `ZRBOB_BOTTLE_VOUCH` |
| `rbrn_cli.sh` | 55–56 | render-time image refs (`z_sentry_img`, `z_bottle_img`) |
| `rbfr_FoundryRetriever.sh` | 150, 171–173 | retriever pull paths |
| `rbfv_FoundryVerify.sh` | 75, 130, 154, 212, 395–396, 605, 629 + jq args 317–321, 510–512, 729–732 | verify reads |
| `rbfc_FoundryCore.sh` | 654, 681, 691, 705 | core foundry path construction |
| `rbfd_FoundryDirectorBuild.sh` | 602, 733, 1178–1180, 1258–1259, 1369–1370, 1488 | producer-side path construction + jq args |
| `rbga_ArtifactRegistry.sh` | (full file scan) | GAR API wrappers |

**Note:** AAK reshapes these paths entirely (basenames replace hyphen-suffix). The AAB→AAK relationship is sequential: AAB threads the prefix through current paths; AAK then reshapes the path grammar. Doing both at once risks losing track of which change is which. Land AAB first, run fast suite green, then AAK.

### Compose interpolation

`.rbk/rbob_compose.yml:23,57,77` currently hardcodes:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```
Update to prepend `${RBRR_CLOUD_PREFIX}`:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```
(AAK will further reshape this when the path grammar changes; AAB just inserts the prefix into the existing shape.)

Verify `RBRR_CLOUD_PREFIX` is exported into compose's environment (likely already is via `--env-file`, but confirm in `zrbob_compose()` at `rbob_bottle.sh:187`).

### Grep sweep for hardcoded sites

- `grep -rnE 'docker\.pkg\.dev|gcr\.io' Tools/ .rbk/` — find any hardcoded GAR hostnames/paths bypassing the constants
- `grep -rnE '"\$\{RBRN_MONIKER\}-(sentry|pentacle|bottle)"' Tools/` — find any container-name reconstruction outside RBOB
- `grep -rnE 'enclave"' Tools/` — find any network-name reconstruction outside RBOB

Any hits are candidates for prefix-wiring or fold-into-helper.

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime-validation exercises new var wiring)
- `tt/rbtd-s.TestSuite.crucible.sh` — 117/117 green (exercises actual container/network names with empty prefix)
- Render a nameplate with empty prefixes: container/network names match current shape exactly
- Render with `RBRR_RUNTIME_PREFIX="test-"`: container names become `test-tadmor-sentry`, network becomes `test-tadmor_enclave`
- Render with `RBRR_CLOUD_PREFIX="test-"`: image refs include `test-` between repo and vessel
- Manual: `tt/rbw-cC.Charge.tadmor.sh` succeeds with empty prefix, no behavior change

### Out of scope

- GAR layout grammar reshape (that is ₢A_AAK — basenames replace hyphen-suffix)
- Marshal-zero blank-template emission of new vars (that is ₢A_AAC)

**[260415-1525] rough**

## Character
Mechanical — trace every consumer of container names, network names, and image paths.

## Docket
Update RBDC derived constants to prepend `RBRR_RUNTIME_PREFIX` to container/network name bases. Update RBOB (bottle kindle) so `ZRBOB_SENTRY`, `ZRBOB_PENTACLE`, `ZRBOB_BOTTLE`, `ZRBOB_NETWORK` incorporate the runtime prefix. Update image path construction in RBOB and foundry to prepend `RBRR_CLOUD_PREFIX` to vessel package names and GAR image references. Verify compose.yml interpolation still works. Grep for any hardcoded container/network/image name patterns that bypass the prefixed variables.

**[260415-1504] rough**

## Character
Mechanical — trace every consumer of container names, network names, and image paths.

## Work
Update RBDC derived constants to prepend `RBRR_RUNTIME_PREFIX` to container/network name bases. Update RBOB (bottle kindle) so `ZRBOB_SENTRY`, `ZRBOB_PENTACLE`, `ZRBOB_BOTTLE`, `ZRBOB_NETWORK` incorporate the runtime prefix. Update image path construction in RBOB and foundry to prepend `RBRR_IMAGE_PREFIX` to vessel package names. Verify compose.yml interpolation still works. Grep for any hardcoded container/network/image name patterns that bypass the prefixed variables.

### marshal-release-zeroes-prefixes (₢A_AAC) [complete]

**[260423-1030] complete**

## Character
Small and mechanical — one marshal edit plus a blank-template alignment.

## Docket
Update the RBLM marshal so that zeroing a regime to blank template carries both `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` (from ₢A_AAA) as empty-string assignments. The zeroed repo regime must continue to pass `rbrr_validate` — empty is valid for both prefixes by design.

### Changes

1. **`Tools/rbk/rblm_cli.sh`** — ensure the marshal-zero path emits both `RBRR_CLOUD_PREFIX=""` and `RBRR_RUNTIME_PREFIX=""` into the blank `rbrr.env` template. Match whatever emission pattern the existing `RBRR_*` blank-template entries use.

2. **Any `rbrr.env` template source files** referenced by the marshal — add both variables defaulted to empty, so a fresh marshal cycle produces a regime file that passes `zrbrr_enforce`.

### Verification

- `tt/rbw-MZ.MarshalZeroes.sh` (or whatever the marshal-zero tabtarget is named) — run against a scratch regime
- `tt/rbw-rrv.ValidateRepoRegime.sh` — validate the zeroed template passes
- `tt/rbw-rrr.RenderRepoRegime.sh` — both prefix vars render with empty values under the "Resource Prefixing" group
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green

### Depends on

₢A_AAA — requires `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` to be enrolled in the RBRR regime before the blank template can reference them.

### Out of scope

- Derived-constant wiring through consumers (that is ₢A_AAB)
- Non-empty prefix values in the blank template (by design: blank means no prefix)
- Any other marshal-zero content beyond the two new prefix entries

**[260423-0750] rough**

## Character
Small and mechanical — one marshal edit plus a blank-template alignment.

## Docket
Update the RBLM marshal so that zeroing a regime to blank template carries both `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` (from ₢A_AAA) as empty-string assignments. The zeroed repo regime must continue to pass `rbrr_validate` — empty is valid for both prefixes by design.

### Changes

1. **`Tools/rbk/rblm_cli.sh`** — ensure the marshal-zero path emits both `RBRR_CLOUD_PREFIX=""` and `RBRR_RUNTIME_PREFIX=""` into the blank `rbrr.env` template. Match whatever emission pattern the existing `RBRR_*` blank-template entries use.

2. **Any `rbrr.env` template source files** referenced by the marshal — add both variables defaulted to empty, so a fresh marshal cycle produces a regime file that passes `zrbrr_enforce`.

### Verification

- `tt/rbw-MZ.MarshalZeroes.sh` (or whatever the marshal-zero tabtarget is named) — run against a scratch regime
- `tt/rbw-rrv.ValidateRepoRegime.sh` — validate the zeroed template passes
- `tt/rbw-rrr.RenderRepoRegime.sh` — both prefix vars render with empty values under the "Resource Prefixing" group
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green

### Depends on

₢A_AAA — requires `RBRR_CLOUD_PREFIX` and `RBRR_RUNTIME_PREFIX` to be enrolled in the RBRR regime before the blank template can reference them.

### Out of scope

- Derived-constant wiring through consumers (that is ₢A_AAB)
- Non-empty prefix values in the blank template (by design: blank means no prefix)
- Any other marshal-zero content beyond the two new prefix entries

**[260415-1504] rough**

## Character
Small and mechanical — one function edit.

## Work
Update RBLM marshal zeroes to clear both `RBRR_IMAGE_PREFIX` and `RBRR_RUNTIME_PREFIX` to empty string in the blank template. Verify the zeroed rbrr.env passes validation (empty prefix is valid).

### gar-categorical-layout-migration (₢A_AAK) [complete]

**[260423-1313] complete**

## Character
Structural — reshapes ark storage in GAR and the path grammar all producer/consumer code uses. Atomic cutover: old and new shapes do not coexist. Notches 1 and 2 are complete; Notch 3 (spec sweep) remains. Grep-zero on `RBGC_ARK_SUFFIX_` and `RBGC_VOUCHES_PACKAGE` is the binding verification.

## Docket

Migrate GAR top-level namespace from vessel-scattered hyphen-suffix (`<vessel>:<hallmark>-<suffix>`) to three prefixed categorical namespaces with basename arks under each hallmark. **Notches 1 and 2 landed; only the spec sweep (Notch 3) remains.**

### Post-Notch-2 actual path shape (binding)

Three root constants in `rbgl_GarLayout.sh`, kindled from `RBRR_CLOUD_PREFIX` + category:

```
RBGL_HALLMARKS_ROOT   = "${RBRR_CLOUD_PREFIX}hallmarks"
RBGL_RELIQUARIES_ROOT = "${RBRR_CLOUD_PREFIX}reliquaries"
RBGL_ENSHRINES_ROOT   = "${RBRR_CLOUD_PREFIX}enshrines"
```

Call-site interpolation (NOT a helper function — "no subshell, no function call overhead" per rbgl_GarLayout.sh:23):

```
${z_gar_base}/${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename>:<hallmark>
${z_gar_base}/${RBGL_RELIQUARIES_ROOT}/<date>/<tool>:<date>
${z_gar_base}/${RBGL_ENSHRINES_ROOT}/<anchor>:<anchor>
```

Tag matches path-terminal segment in all three shapes (hallmark | date | anchor). Redundant by construction — divergence is a visible bug.

Basename constants in `rbgc_Constants.sh`:
```
RBGC_ARK_BASENAME_IMAGE   = "image"
RBGC_ARK_BASENAME_ABOUT   = "about"
RBGC_ARK_BASENAME_VOUCH   = "vouch"
RBGC_ARK_BASENAME_DIAGS   = "diags"
RBGC_ARK_BASENAME_ATTEST  = "attest"
RBGC_ARK_BASENAME_POUCH   = "pouch"
```

Category constants in `rbgc_Constants.sh`:
```
RBGC_GAR_CATEGORY_HALLMARKS   = "hallmarks"
RBGC_GAR_CATEGORY_RELIQUARIES = "reliquaries"
RBGC_GAR_CATEGORY_ENSHRINES   = "enshrines"
```

### Vocabulary conversions (applies uniformly across all spec files)

| Old | New |
|-----|-----|
| `<vessel>:<hallmark>-image` | `<cloud_prefix>hallmarks/<hallmark>/image:<hallmark>` |
| `<vessel>:<hallmark>-about` | `<cloud_prefix>hallmarks/<hallmark>/about:<hallmark>` |
| `<vessel>:<hallmark>-vouch` | `<cloud_prefix>hallmarks/<hallmark>/vouch:<hallmark>` |
| `<vessel>:<hallmark>-pouch` | `<cloud_prefix>hallmarks/<hallmark>/pouch:<hallmark>` |
| `<vessel>:<hallmark>-diags` | `<cloud_prefix>hallmarks/<hallmark>/diags:<hallmark>` |
| `<vessel>:<hallmark>-attest-<ARCH>` | `<cloud_prefix>hallmarks/<hallmark>/attest:<hallmark>-<ARCH>` (attest remains per-arch-tagged under one basename; see note below) |
| `{repo}/enshrine:{anchor}` | `<cloud_prefix>enshrines/<anchor>:<anchor>` |
| `vouches:<hallmark>-vouch` (aggregator) | dissolved — hallmark-only lookup comes free via subtree navigation |
| "vouches superdirectory" phrase | "hallmark subtree" / remove as redundant |
| `RBGC_ARK_SUFFIX_*` | `RBGC_ARK_BASENAME_*` |
| `_RBGA_ARK_SUFFIX_*` attribute refs | `_RBGA_ARK_BASENAME_*` (where they appear in spec attribute tables) |
| `_RBGY_ARK_SUFFIX_*` attribute refs | `_RBGY_ARK_BASENAME_*` (where they appear in spec attribute tables) |
| `RBGC_VOUCHES_PACKAGE` | DELETED — remove attribute and any reference |

Attest note: confirm with the code — the `attest:<hallmark>-<ARCH>` shape is a working hypothesis. If code actually emits `attest-<ARCH>:<hallmark>` or another shape, specs reflect the code. Verify by reading `rbfd_FoundryDirectorBuild.sh` attest push site before committing attest prose.

### Enshrine shift (binding, per-agent handoff)

Old: `{repo}/enshrine:{anchor}` — singular `enshrine` namespace, anchor as tag on a shared package.

New: `<cloud_prefix>enshrines/<anchor>:<anchor>` — plural `enshrines/` for namespace symmetry; each anchor is a peer package under the namespace, not a tag on a shared package. Spec prose in RBSAE and any other file referencing enshrine layout must update.

### Abjure rewrite (RBSAA)

Old: per-suffix HEAD/DELETE dance — one HEAD and one DELETE per suffix, sequential, hardcoded list of 5-6 suffixes.

New: enumerate-then-delete via `${RBGL_HALLMARKS_ROOT}/<hallmark>` subtree. Loop iterates over discovered children rather than a hardcoded list — naturally tolerates graft's missing pouch and any future ark additions. Still N deletes; the iteration shape changes, not the operation count.

### File-by-file edit list (~17 files)

**From original docket (10):**
- `RBSAC-ark_conjure.adoc` — 5 hits: `-image/-about` tags, `VESSEL:HALLMARK-pouch`, `enshrine:ANCHOR`
- `RBSAE-ark_enshrine.adoc` — 2 hits: enshrine-tag→enshrines-path-segment shift; heavy prose rewrite
- `RBSDI-depot_inscribe.adoc` — no direct stale refs, but ADD: `_RBGN_CLOUD_PREFIX` substitution docs + `<prefix>reliquaries/<date>/<tool>:<date>` reliquary path
- `RBSAA-ark_abjure.adoc` — 25+ hits: per-suffix algorithm → enumerate-then-delete (see Abjure rewrite above)
- `RBSAV-ark_vouch.adoc` — 15 hits: `-about/-vouch/-image` tags; dual-tag `vouches:HALLMARK-vouch` dissolves; `_RBGV_VOUCHES_PACKAGE` attribute deleted
- `RBSAS-ark_summon.adoc` — 3 hits: `VESSEL:HALLMARK-{image,about,vouch}` → basename shape
- `RBSAP-ark_plumb.adoc` — ~7 hits: "vouches hallmark-only mode" → hallmark subtree navigation
- `RBSCL-consecration_tally.adoc` — ~7 hits: tag-suffix classification → basename-under-hallmark enumeration
- `RBS0-SpecTop.adoc` — 40+ hits: heaviest; dense attestation section; vouches superdirectory explanation; enshrine namespace ref
- `RBSCO-CosmologyIntro.adoc` — CLEAN. Scanned, no stale layout refs. Listed for completeness; no edits needed.

**Additions from grep sweep (7):**
- `RBSAB-ark_about.adoc` — 20+ hits: many `-image/-about/-diags` refs; `_RBGA_ARK_SUFFIX_*` attribute table pointing at renamed constants
- `RBSAG-ark_graft.adoc` — 3 hits: `{VESSEL}:{HALLMARK}-image`, `-about` refs
- `RBSAK-ark_kludge.adoc` — 3 hits: `{SIGIL}:{HALLMARK}-image`, `-vouch` grammar
- `RBSCB-CloudBuildPosture.adoc` — dedicated tag table with `{HALLMARK}-image`, `{HALLMARK}-about`, `{HALLMARK}-diags`; plus `-diags` prose
- `RBSCC-crucible_charge.adoc` — 2 hits: `{vessel}:{hallmark}-vouch` in prerequisite checks (local image store refs)
- `RBSDV-director_vouch.adoc` — 7 hits: classification algo refs `-about/-vouch/-image`
- `RBSGS-GettingStarted.adoc` — 1 hit: "vouches superdirectory" phrase
- `RBSIJ-image_jettison.adoc` — 1 hit: "three artifacts: `-image`, `-about`, `-vouch`"
- `RBSTB-trigger_build.adoc` — 2 hits: `_RBGY_ARK_SUFFIX_IMAGE` attribute + `ARK_SUFFIX_IMAGE` usage

### Scope notes

- `.rbk/` is clean (compose.yml and fixtures already updated in Notch 2b).
- `Tools/rbk/rbgc_Constants.sh` already defines `RBGC_ARK_BASENAME_*` and `RBGC_GAR_CATEGORY_*` (Notch 1). No code-side changes needed in Notch 3.
- `RBSHR-HorizonRoadmap.adoc` and `RBSIP-ifrit_pentester.adoc` matched grep but are false positives (image-layer metaphor; URL fragments in bibliography).

### Execution plan (parallel agent dispatch)

Five general-purpose agents, each receiving the same conversion-rules brief (Vocabulary conversions + Enshrine shift + Abjure rewrite + path shape), working on disjoint file sets:

- **Agent A** (heavy lifecycle): RBS0, RBSAA, RBSAV
- **Agent B** (ark mid): RBSAB, RBSAC, RBSAE, RBSAS
- **Agent C** (ark light + local): RBSAG, RBSAK, RBSAP, RBSIJ
- **Agent D** (aggregators + infra): RBSCL, RBSDV, RBSCB, RBSTB
- **Agent E** (prose + inscribe): RBSCC, RBSGS, RBSDI

After all agents return, main session does final grep-zero verification and reads each modified file's diff for consistency before notch.

### Verification

- `grep -nE 'RBGC_ARK_SUFFIX_' Tools/` returns zero (renamed constants; no spec references)
- `grep -nE 'RBGC_VOUCHES_PACKAGE' Tools/` returns zero (deleted; no spec references)
- `grep -nE ':\$\{[^}]*HALLMARK[^}]*\}-(image|vouch|pouch|about|attest|diags)' Tools/` returns zero
- `grep -nE 'enshrine:\{?[Aa]nchor' Tools/` returns zero (old enshrine tag grammar gone)
- `grep -rnE 'vouches (superdirectory|package|aggregator)' Tools/rbk/vov_veiled/` returns zero
- Each edited spec reads cleanly with new path shape and vocabulary

### Out of scope

- Tabtarget signature simplification — `₢A_AAL`
- Depot regen execution — `₢A_AAE`
- Reliquary garbage collection — deferred
- Filesystem moorings rename — separate heat per `₢A_AAJ`

**[260423-1236] rough**

## Character
Structural — reshapes ark storage in GAR and the path grammar all producer/consumer code uses. Atomic cutover: old and new shapes do not coexist. Notches 1 and 2 are complete; Notch 3 (spec sweep) remains. Grep-zero on `RBGC_ARK_SUFFIX_` and `RBGC_VOUCHES_PACKAGE` is the binding verification.

## Docket

Migrate GAR top-level namespace from vessel-scattered hyphen-suffix (`<vessel>:<hallmark>-<suffix>`) to three prefixed categorical namespaces with basename arks under each hallmark. **Notches 1 and 2 landed; only the spec sweep (Notch 3) remains.**

### Post-Notch-2 actual path shape (binding)

Three root constants in `rbgl_GarLayout.sh`, kindled from `RBRR_CLOUD_PREFIX` + category:

```
RBGL_HALLMARKS_ROOT   = "${RBRR_CLOUD_PREFIX}hallmarks"
RBGL_RELIQUARIES_ROOT = "${RBRR_CLOUD_PREFIX}reliquaries"
RBGL_ENSHRINES_ROOT   = "${RBRR_CLOUD_PREFIX}enshrines"
```

Call-site interpolation (NOT a helper function — "no subshell, no function call overhead" per rbgl_GarLayout.sh:23):

```
${z_gar_base}/${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename>:<hallmark>
${z_gar_base}/${RBGL_RELIQUARIES_ROOT}/<date>/<tool>:<date>
${z_gar_base}/${RBGL_ENSHRINES_ROOT}/<anchor>:<anchor>
```

Tag matches path-terminal segment in all three shapes (hallmark | date | anchor). Redundant by construction — divergence is a visible bug.

Basename constants in `rbgc_Constants.sh`:
```
RBGC_ARK_BASENAME_IMAGE   = "image"
RBGC_ARK_BASENAME_ABOUT   = "about"
RBGC_ARK_BASENAME_VOUCH   = "vouch"
RBGC_ARK_BASENAME_DIAGS   = "diags"
RBGC_ARK_BASENAME_ATTEST  = "attest"
RBGC_ARK_BASENAME_POUCH   = "pouch"
```

Category constants in `rbgc_Constants.sh`:
```
RBGC_GAR_CATEGORY_HALLMARKS   = "hallmarks"
RBGC_GAR_CATEGORY_RELIQUARIES = "reliquaries"
RBGC_GAR_CATEGORY_ENSHRINES   = "enshrines"
```

### Vocabulary conversions (applies uniformly across all spec files)

| Old | New |
|-----|-----|
| `<vessel>:<hallmark>-image` | `<cloud_prefix>hallmarks/<hallmark>/image:<hallmark>` |
| `<vessel>:<hallmark>-about` | `<cloud_prefix>hallmarks/<hallmark>/about:<hallmark>` |
| `<vessel>:<hallmark>-vouch` | `<cloud_prefix>hallmarks/<hallmark>/vouch:<hallmark>` |
| `<vessel>:<hallmark>-pouch` | `<cloud_prefix>hallmarks/<hallmark>/pouch:<hallmark>` |
| `<vessel>:<hallmark>-diags` | `<cloud_prefix>hallmarks/<hallmark>/diags:<hallmark>` |
| `<vessel>:<hallmark>-attest-<ARCH>` | `<cloud_prefix>hallmarks/<hallmark>/attest:<hallmark>-<ARCH>` (attest remains per-arch-tagged under one basename; see note below) |
| `{repo}/enshrine:{anchor}` | `<cloud_prefix>enshrines/<anchor>:<anchor>` |
| `vouches:<hallmark>-vouch` (aggregator) | dissolved — hallmark-only lookup comes free via subtree navigation |
| "vouches superdirectory" phrase | "hallmark subtree" / remove as redundant |
| `RBGC_ARK_SUFFIX_*` | `RBGC_ARK_BASENAME_*` |
| `_RBGA_ARK_SUFFIX_*` attribute refs | `_RBGA_ARK_BASENAME_*` (where they appear in spec attribute tables) |
| `_RBGY_ARK_SUFFIX_*` attribute refs | `_RBGY_ARK_BASENAME_*` (where they appear in spec attribute tables) |
| `RBGC_VOUCHES_PACKAGE` | DELETED — remove attribute and any reference |

Attest note: confirm with the code — the `attest:<hallmark>-<ARCH>` shape is a working hypothesis. If code actually emits `attest-<ARCH>:<hallmark>` or another shape, specs reflect the code. Verify by reading `rbfd_FoundryDirectorBuild.sh` attest push site before committing attest prose.

### Enshrine shift (binding, per-agent handoff)

Old: `{repo}/enshrine:{anchor}` — singular `enshrine` namespace, anchor as tag on a shared package.

New: `<cloud_prefix>enshrines/<anchor>:<anchor>` — plural `enshrines/` for namespace symmetry; each anchor is a peer package under the namespace, not a tag on a shared package. Spec prose in RBSAE and any other file referencing enshrine layout must update.

### Abjure rewrite (RBSAA)

Old: per-suffix HEAD/DELETE dance — one HEAD and one DELETE per suffix, sequential, hardcoded list of 5-6 suffixes.

New: enumerate-then-delete via `${RBGL_HALLMARKS_ROOT}/<hallmark>` subtree. Loop iterates over discovered children rather than a hardcoded list — naturally tolerates graft's missing pouch and any future ark additions. Still N deletes; the iteration shape changes, not the operation count.

### File-by-file edit list (~17 files)

**From original docket (10):**
- `RBSAC-ark_conjure.adoc` — 5 hits: `-image/-about` tags, `VESSEL:HALLMARK-pouch`, `enshrine:ANCHOR`
- `RBSAE-ark_enshrine.adoc` — 2 hits: enshrine-tag→enshrines-path-segment shift; heavy prose rewrite
- `RBSDI-depot_inscribe.adoc` — no direct stale refs, but ADD: `_RBGN_CLOUD_PREFIX` substitution docs + `<prefix>reliquaries/<date>/<tool>:<date>` reliquary path
- `RBSAA-ark_abjure.adoc` — 25+ hits: per-suffix algorithm → enumerate-then-delete (see Abjure rewrite above)
- `RBSAV-ark_vouch.adoc` — 15 hits: `-about/-vouch/-image` tags; dual-tag `vouches:HALLMARK-vouch` dissolves; `_RBGV_VOUCHES_PACKAGE` attribute deleted
- `RBSAS-ark_summon.adoc` — 3 hits: `VESSEL:HALLMARK-{image,about,vouch}` → basename shape
- `RBSAP-ark_plumb.adoc` — ~7 hits: "vouches hallmark-only mode" → hallmark subtree navigation
- `RBSCL-consecration_tally.adoc` — ~7 hits: tag-suffix classification → basename-under-hallmark enumeration
- `RBS0-SpecTop.adoc` — 40+ hits: heaviest; dense attestation section; vouches superdirectory explanation; enshrine namespace ref
- `RBSCO-CosmologyIntro.adoc` — CLEAN. Scanned, no stale layout refs. Listed for completeness; no edits needed.

**Additions from grep sweep (7):**
- `RBSAB-ark_about.adoc` — 20+ hits: many `-image/-about/-diags` refs; `_RBGA_ARK_SUFFIX_*` attribute table pointing at renamed constants
- `RBSAG-ark_graft.adoc` — 3 hits: `{VESSEL}:{HALLMARK}-image`, `-about` refs
- `RBSAK-ark_kludge.adoc` — 3 hits: `{SIGIL}:{HALLMARK}-image`, `-vouch` grammar
- `RBSCB-CloudBuildPosture.adoc` — dedicated tag table with `{HALLMARK}-image`, `{HALLMARK}-about`, `{HALLMARK}-diags`; plus `-diags` prose
- `RBSCC-crucible_charge.adoc` — 2 hits: `{vessel}:{hallmark}-vouch` in prerequisite checks (local image store refs)
- `RBSDV-director_vouch.adoc` — 7 hits: classification algo refs `-about/-vouch/-image`
- `RBSGS-GettingStarted.adoc` — 1 hit: "vouches superdirectory" phrase
- `RBSIJ-image_jettison.adoc` — 1 hit: "three artifacts: `-image`, `-about`, `-vouch`"
- `RBSTB-trigger_build.adoc` — 2 hits: `_RBGY_ARK_SUFFIX_IMAGE` attribute + `ARK_SUFFIX_IMAGE` usage

### Scope notes

- `.rbk/` is clean (compose.yml and fixtures already updated in Notch 2b).
- `Tools/rbk/rbgc_Constants.sh` already defines `RBGC_ARK_BASENAME_*` and `RBGC_GAR_CATEGORY_*` (Notch 1). No code-side changes needed in Notch 3.
- `RBSHR-HorizonRoadmap.adoc` and `RBSIP-ifrit_pentester.adoc` matched grep but are false positives (image-layer metaphor; URL fragments in bibliography).

### Execution plan (parallel agent dispatch)

Five general-purpose agents, each receiving the same conversion-rules brief (Vocabulary conversions + Enshrine shift + Abjure rewrite + path shape), working on disjoint file sets:

- **Agent A** (heavy lifecycle): RBS0, RBSAA, RBSAV
- **Agent B** (ark mid): RBSAB, RBSAC, RBSAE, RBSAS
- **Agent C** (ark light + local): RBSAG, RBSAK, RBSAP, RBSIJ
- **Agent D** (aggregators + infra): RBSCL, RBSDV, RBSCB, RBSTB
- **Agent E** (prose + inscribe): RBSCC, RBSGS, RBSDI

After all agents return, main session does final grep-zero verification and reads each modified file's diff for consistency before notch.

### Verification

- `grep -nE 'RBGC_ARK_SUFFIX_' Tools/` returns zero (renamed constants; no spec references)
- `grep -nE 'RBGC_VOUCHES_PACKAGE' Tools/` returns zero (deleted; no spec references)
- `grep -nE ':\$\{[^}]*HALLMARK[^}]*\}-(image|vouch|pouch|about|attest|diags)' Tools/` returns zero
- `grep -nE 'enshrine:\{?[Aa]nchor' Tools/` returns zero (old enshrine tag grammar gone)
- `grep -rnE 'vouches (superdirectory|package|aggregator)' Tools/rbk/vov_veiled/` returns zero
- Each edited spec reads cleanly with new path shape and vocabulary

### Out of scope

- Tabtarget signature simplification — `₢A_AAL`
- Depot regen execution — `₢A_AAE`
- Reliquary garbage collection — deferred
- Filesystem moorings rename — separate heat per `₢A_AAJ`

**[260423-1040] rough**

## Character
Structural — reshapes ark storage in GAR and the path grammar all producer/consumer code uses. Atomic cutover: old and new shapes do not coexist. The path-construction helper minted here is the single point of truth that makes the cutover tractable and the abjure cleanup possible.

## Docket

Migrate GAR top-level namespace from vessel-scattered hyphen-suffix (`<vessel>:<hallmark>-<suffix>`) to three prefixed categorical namespaces with basename arks under each hallmark:

```
${RBRR_CLOUD_PREFIX}hallmarks/<hallmark>/{image, vouch, pouch, about, attest, diags}
${RBRR_CLOUD_PREFIX}reliquaries/<date>/{gcloud, docker, alpine, syft, binfmt, skopeo}
${RBRR_CLOUD_PREFIX}enshrines/<anchor>
```

Each immediate child of `<prefix>hallmarks` IS a hallmark; the six ark types become plain basename siblings under it. Each immediate child of `<prefix>reliquaries` IS a reliquary (datestamp); all six tool images mirrored by one inscribe operation become plain basename siblings under it. Each immediate child of `<prefix>enshrines` IS an anchor (content hash) for one base image. The `<vessel>:<hallmark>-<suffix>` scheme and the `vouches/` aggregator package both dissolve.

### Reliquary precision

Today (`Tools/rbk/rbgji/rbgji01-inscribe-mirror.sh:36`):
```
${_RBGN_GAR_HOST}/${_RBGN_GAR_PATH}/${_RBGN_RELIQUARY}/${NAME}:latest
```
Tool manifest at lines 18–25: `gcloud, docker, alpine, syft, binfmt, skopeo`. All six mirror into one datestamped reliquary directory in a single Cloud Build pass — co-versioned by design.

After AAK:
```
${_RBGN_GAR_HOST}/${_RBGN_GAR_PATH}/${_RBGN_CLOUD_PREFIX}reliquaries/${_RBGN_RELIQUARY}/${NAME}:latest
```

The bare datestamp stays in `_RBGN_RELIQUARY` (no semantic drift on the existing variable). A new substitution `_RBGN_CLOUD_PREFIX` is added to the inscribe Cloud Build invocation (alongside the existing `_RBGN_GAR_HOST`, `_RBGN_GAR_PATH`, `_RBGN_RELIQUARY`). The mirror script constructs the prefixed-namespace path explicitly. Same pattern propagates to the enshrine and conjure builders that need the cloud prefix.

### Enshrine precision

Today (per `RBSAE-ark_enshrine.adoc`): `{repo}/enshrine:{anchor}` — singular `enshrine` namespace, anchor as Docker tag.

After AAK: `${RBRR_CLOUD_PREFIX}enshrines/<anchor>` — plural `enshrines/` for symmetry with `hallmarks/` and `reliquaries/`; anchor moves from tag to path segment so each anchor is a peer package under the namespace, not a tag on a shared package. Multiple anchors no longer share one package's tag space.

Update enshrine producer (`rbgje01-enshrine-copy.sh` and its dispatcher) and any consumer that resolves `{repo}/enshrine:{anchor}` paths to the new shape. Document the schema change in RBSAE.

### Vessel identity (no producer change required)

Verified: `rbfv_FoundryVerify.sh:310,335,499,520,724,746` already emits `_RBGA_VESSEL` and `_RBGV_VESSEL` fields into vouch/about JSON. AAL reads these on demand; no producer change in AAK.

### Constants — `Tools/rbk/rbgc_Constants.sh`

- Rename `RBGC_ARK_SUFFIX_{IMAGE,VOUCH,POUCH,ABOUT,ATTEST,DIAGS}` -> `RBGC_ARK_BASENAME_*`; values drop the leading hyphen (line 122–127).
- Delete `RBGC_VOUCHES_PACKAGE` (line 131) — hallmark-only vouch lookup comes free via subtree navigation.
- Add categorical-namespace constants:
  ```
  readonly RBGC_GAR_CATEGORY_HALLMARKS="hallmarks"
  readonly RBGC_GAR_CATEGORY_RELIQUARIES="reliquaries"
  readonly RBGC_GAR_CATEGORY_ENSHRINES="enshrines"
  ```

### Path-construction helper — single source of truth

Mint helpers in new module `Tools/rbk/rbgl_GarLayout.sh` (rbgl = GAR Layout; terminal-exclusive slot, rbgp_ is Payor):

```
rbgl_hallmark_ark_path()    # (hallmark, ark_basename) -> "${CLOUD_PREFIX}hallmarks/${hallmark}/${ark_basename}"
rbgl_hallmark_subtree()     # (hallmark)               -> "${CLOUD_PREFIX}hallmarks/${hallmark}"
rbgl_reliquary_path()       # (date, tool)             -> "${CLOUD_PREFIX}reliquaries/${date}/${tool}"
rbgl_reliquary_subtree()    # (date)                   -> "${CLOUD_PREFIX}reliquaries/${date}"
rbgl_enshrine_path()        # (anchor)                 -> "${CLOUD_PREFIX}enshrines/${anchor}"
```

Helpers emit **prefix-rooted relative path** (from GAR repo root), not full URL. Full URL construction keeps existing `${host}/${path}/` prefix intact; only the vessel-suffix segment changes. Call sites do `"${host}/${path}/$(rbgl_hallmark_ark_path hallmark image)"`.

**Discipline:** ALL producer and consumer sites route through these helpers. No more direct concat. Verification step `grep -nE 'RBGC_ARK_BASENAME_' Tools/ | grep -v rbgl_` should return zero outside the helpers — every other reference goes through the helper.

### Producer sites (route through helper)

- `rbfd_FoundryDirectorBuild.sh` — conjure (image, vouch, pouch, about, attest, diags), bind, graft (no pouch). Affected lines: 602, 733, 1178–1180, 1258–1259, 1369–1370, 1488.
- `rbfc_FoundryCore.sh` — lines 654, 681, 691, 705.
- `rbfv_FoundryVerify.sh` — vouch/about emission. Lines 75, 130, 154, 212, 317–321, 395–396, 510–512, 605, 629, 729–732.
- `rbga_ArtifactRegistry.sh` — GAR API wrappers; reshape any path construction.
- `rbgji/rbgji01-inscribe-mirror.sh:36` — inscribe mirror script; consumes new `_RBGN_CLOUD_PREFIX` substitution.
- Enshrine pipeline (rbgje01-enshrine-copy.sh and dispatcher in rbfd) — emit under `<prefix>enshrines/<anchor>` with anchor as path segment.

### Consumer sites (route through helper)

- `rbob_bottle.sh:126–131` — `ZRBOB_*_IMAGE`, `ZRBOB_*_VOUCH` construction.
- `rbrn_cli.sh:55–56` — render-time image refs.
- `rbfr_FoundryRetriever.sh:150,171–173` — pull paths.
- `rbfv_FoundryVerify.sh` (consumer half) — verify reads.
- `rbfl_FoundryLedger.sh:440–650` — abjure per-suffix HEAD/DELETE dance collapses to enumerate-then-delete via `rbgl_hallmark_subtree()`. Loop iterates over discovered children rather than a hardcoded suffix list — free tolerance for graft's missing pouch and any future ark additions.
- `rbfl_FoundryLedger.sh` — Jettison/Rekons/Wrest implementations (image-management tabtargets currently live here).

### Compose.yml reshape

`.rbk/rbob_compose.yml:23,57,77` — replace the vessel-suffix shape with the basename shape. Tag = hallmark itself (immutable by operator convention; tag and path segment both equal the hallmark, so any divergence is a visible bug):

Before (post-AAB):
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```

After:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}hallmarks/${RBRN_SENTRY_HALLMARK}/image:${RBRN_SENTRY_HALLMARK}
```

Vessel disappears from compose path.

### Spec updates

- `RBSAC-ark_conjure.adoc` — new basename layout
- `RBSAE-ark_enshrine.adoc` — enshrines plural namespace; anchor moves from tag to path segment
- `RBSDI-depot_inscribe.adoc` — reliquary lands under `<prefix>reliquaries/<date>/{tools}`; document `_RBGN_CLOUD_PREFIX` substitution
- `RBSAA-ark_abjure.adoc` — enumerate-then-delete replaces per-suffix dance
- `RBSAV-ark_vouch.adoc` — vouch path/lookup pattern
- `RBSAS-ark_summon.adoc` — summon path
- `RBSAP-ark_plumb.adoc` — plumb path
- `RBSCL-consecration_tally.adoc` — tally traversal pattern (subtree under `<prefix>hallmarks/`)
- `RBS0-SpecTop.adoc` — ark-layout prose
- `RBSCO-CosmologyIntro.adoc` — if it references the old layout

### Reliquary accumulation policy

Each inscribe creates a new datestamped reliquary; old reliquaries persist until explicitly cleaned. No GC mechanism exists today; out of scope for AAK. Captured here so the eventual GC question isn't lost.

### Pouch-on-graft

Conjure and bind produce pouch; graft does not. A grafted hallmark has 5 children (image, vouch, about, attest, diags), not 6. Abjure's enumerate-then-delete handles this naturally — loop iterates over what exists, no explicit missing-pouch branch.

### Decisions (groomed)

Binding stances from grooming. Research above stands as context; this section is the contract.

1. **Helper module:** `Tools/rbk/rbgl_GarLayout.sh` (rbgl = GAR Layout). `rbgc_Constants.sh` stays declaration-only.
2. **Helper return shape:** prefix-rooted relative path from GAR repo root. Callers prepend `${host}/${path}/`.
3. **Abjure mechanism:** enumerate-then-delete via subtree root from `rbgl_hallmark_subtree()`. N deletes still, but loop iterates over discovered children.
4. **Tag on basename arks:** the hallmark itself — `image:${HALLMARK}`, etc. Redundant with path segment by construction; divergence = visible bug. No floating tag name.
5. **Graft's missing pouch:** tolerated naturally by enumerate-loop; no special case.
6. **Commit sequence — 3 notches:**
   - Notch 1 (additive, green): mint `rbgl_GarLayout.sh` + add category constants. No consumers yet.
   - Notch 2 (atomic cutover): rename suffix->basename constants, rewrite every producer/consumer concat site through helpers, reshape compose.yml, delete `RBGC_VOUCHES_PACKAGE`. Grep-zero checks pass here.
   - Notch 3 (specs): 10 spec files.
7. **First producer routed:** `rbfd_FoundryDirectorBuild.sh` conjure path — exercises all 6 ark basenames in one pass.
8. **Verification gating:** fast suite mandatory pre-notch. Crucible suite via foray to fundus if live credentials; otherwise explicit flag in close summary for user-side run.
9. **`_RBGN_CLOUD_PREFIX` substitution:** inscribe builder. Enshrine and conjure builders follow their own substitution-prefix family (`_RBGE_*`, `_RBGA_*`/`_RBGV_*`) — no cross-builder naming coupling forced.
10. **AAK/AAL boundary:** AAK stops at locator construction. Tabtarget signature `(vessel, hallmark) -> (hallmark)` simplification deferred to AAL.

### Verification

- `grep -nE 'RBGC_ARK_SUFFIX_' Tools/` returns zero (renamed)
- `grep -nE 'RBGC_VOUCHES_PACKAGE' Tools/` returns zero
- `grep -nE ':\$\{[^}]*HALLMARK[^}]*\}-' Tools/ .rbk/` returns zero (old hyphen-suffix concat gone)
- `grep -nE 'RBGC_ARK_BASENAME_' Tools/ | grep -v rbgl_` — all hits go through path helpers, not direct concat
- After conjure of one hallmark: render subtree, confirm `image, vouch, pouch, about, attest, diags` as basenames
- After graft of one hallmark: render subtree, confirm 5 basenames (no pouch), abjure clean
- After inscribe: `<prefix>reliquaries/<date>/` contains all 6 tool basenames as siblings
- After enshrine: `<prefix>enshrines/<anchor>` is a package, not a tag on a shared package
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green
- `tt/rbtd-s.TestSuite.crucible.sh` — 117/117 green (via foray or user-side)

### Out of scope

- Tabtarget signature simplification (₢A_AAL)
- Depot regen execution (₢A_AAE)
- Reliquary garbage collection (deferred — captured above)
- Filesystem moorings rename (separate follow-up heat per ₢A_AAJ)

**[260423-0911] rough**

## Character
Structural — reshapes ark storage in GAR and the path grammar all producer/consumer code uses. Atomic cutover: old and new shapes do not coexist. The path-construction helper minted here is the single point of truth that makes the cutover tractable and the abjure cleanup possible.

## Docket

Migrate GAR top-level namespace from vessel-scattered hyphen-suffix (`<vessel>:<hallmark>-<suffix>`) to three prefixed categorical namespaces with basename arks under each hallmark:

```
${RBRR_CLOUD_PREFIX}hallmarks/<hallmark>/{image, vouch, pouch, about, attest, diags}
${RBRR_CLOUD_PREFIX}reliquaries/<date>/{gcloud, docker, alpine, syft, binfmt, skopeo}
${RBRR_CLOUD_PREFIX}enshrines/<anchor>
```

Each immediate child of `<prefix>hallmarks` IS a hallmark; the six ark types become plain basename siblings under it. Each immediate child of `<prefix>reliquaries` IS a reliquary (datestamp); all six tool images mirrored by one inscribe operation become plain basename siblings under it. Each immediate child of `<prefix>enshrines` IS an anchor (content hash) for one base image. The `<vessel>:<hallmark>-<suffix>` scheme and the `vouches/` aggregator package both dissolve.

### Reliquary precision

Today (`Tools/rbk/rbgji/rbgji01-inscribe-mirror.sh:36`):
```
${_RBGN_GAR_HOST}/${_RBGN_GAR_PATH}/${_RBGN_RELIQUARY}/${NAME}:latest
```
Tool manifest at lines 18–25: `gcloud, docker, alpine, syft, binfmt, skopeo`. All six mirror into one datestamped reliquary directory in a single Cloud Build pass — co-versioned by design.

After AAK:
```
${_RBGN_GAR_HOST}/${_RBGN_GAR_PATH}/${_RBGN_CLOUD_PREFIX}reliquaries/${_RBGN_RELIQUARY}/${NAME}:latest
```

The bare datestamp stays in `_RBGN_RELIQUARY` (no semantic drift on the existing variable). A new substitution `_RBGN_CLOUD_PREFIX` is added to the inscribe Cloud Build invocation (alongside the existing `_RBGN_GAR_HOST`, `_RBGN_GAR_PATH`, `_RBGN_RELIQUARY`). The mirror script constructs the prefixed-namespace path explicitly. Same pattern propagates to the enshrine and conjure builders that need the cloud prefix.

### Enshrine precision

Today (per `RBSAE-ark_enshrine.adoc`): `{repo}/enshrine:{anchor}` — singular `enshrine` namespace, anchor as Docker tag.

After AAK: `${RBRR_CLOUD_PREFIX}enshrines/<anchor>` — plural `enshrines/` for symmetry with `hallmarks/` and `reliquaries/`; anchor moves from tag to path segment so each anchor is a peer package under the namespace, not a tag on a shared package. Multiple anchors no longer share one package's tag space.

Update enshrine producer (`rbgje01-enshrine-copy.sh` and its dispatcher) and any consumer that resolves `{repo}/enshrine:{anchor}` paths to the new shape. Document the schema change in RBSAE.

### Vessel identity (no producer change required)

Verified: `rbfv_FoundryVerify.sh:310,335,499,520,724,746` already emits `_RBGA_VESSEL` and `_RBGV_VESSEL` fields into vouch/about JSON. AAL reads these on demand; no producer change in AAK.

### Constants — `Tools/rbk/rbgc_Constants.sh`

- Rename `RBGC_ARK_SUFFIX_{IMAGE,VOUCH,POUCH,ABOUT,ATTEST,DIAGS}` -> `RBGC_ARK_BASENAME_*`; values drop the leading hyphen (line 122–127).
- Delete `RBGC_VOUCHES_PACKAGE` (line 131) — hallmark-only vouch lookup comes free via subtree navigation.
- Add categorical-namespace constants:
  ```
  readonly RBGC_GAR_CATEGORY_HALLMARKS="hallmarks"
  readonly RBGC_GAR_CATEGORY_RELIQUARIES="reliquaries"
  readonly RBGC_GAR_CATEGORY_ENSHRINES="enshrines"
  ```

### Path-construction helper — single source of truth

Mint helper(s) in a stable location (suggest `Tools/rbk/rbgc_Constants.sh` or new `rbgp_GarPath.sh`):

```
rbgc_hallmark_ark_path()    # (hallmark, ark_basename) -> "${cloud_prefix}hallmarks/${hallmark}/${ark_basename}"
rbgc_hallmark_subtree()     # (hallmark) -> "${cloud_prefix}hallmarks/${hallmark}"
rbgc_reliquary_path()       # (date, tool) -> "${cloud_prefix}reliquaries/${date}/${tool}"
rbgc_reliquary_subtree()    # (date) -> "${cloud_prefix}reliquaries/${date}"
rbgc_enshrine_path()        # (anchor) -> "${cloud_prefix}enshrines/${anchor}"
```

**Discipline:** ALL producer and consumer sites route through these helpers. No more direct concat. Verification step `grep -nE 'RBGC_ARK_BASENAME_' Tools/ | grep -v rbgc_` should return zero outside the helpers — every other reference goes through the helper.

### Producer sites (route through helper)

- `rbfd_FoundryDirectorBuild.sh` — conjure (image, vouch, pouch, about, attest, diags), bind, graft (no pouch). Affected lines: 602, 733, 1178–1180, 1258–1259, 1369–1370, 1488.
- `rbfc_FoundryCore.sh` — lines 654, 681, 691, 705.
- `rbfv_FoundryVerify.sh` — vouch/about emission. Lines 75, 130, 154, 212, 317–321, 395–396, 510–512, 605, 629, 729–732.
- `rbga_ArtifactRegistry.sh` — GAR API wrappers; reshape any path construction.
- `rbgji/rbgji01-inscribe-mirror.sh:36` — inscribe mirror script; consumes new `_RBGN_CLOUD_PREFIX` substitution.
- Enshrine pipeline (rbgje01-enshrine-copy.sh and dispatcher in rbfd) — emit under `<prefix>enshrines/<anchor>` with anchor as path segment.

### Consumer sites (route through helper)

- `rbob_bottle.sh:126–131` — `ZRBOB_*_IMAGE`, `ZRBOB_*_VOUCH` construction.
- `rbrn_cli.sh:55–56` — render-time image refs.
- `rbfr_FoundryRetriever.sh:150,171–173` — pull paths.
- `rbfv_FoundryVerify.sh` (consumer half) — verify reads.
- `rbfl_FoundryLedger.sh:440–650` — abjure per-suffix HEAD/DELETE dance collapses to single subtree enumerate + delete via `rbgc_hallmark_subtree()`.
- `rbfl_FoundryLedger.sh` — Jettison/Rekons/Wrest implementations (image-management tabtargets currently live here).

### Compose.yml reshape

`.rbk/rbob_compose.yml:23,57,77` — replace the vessel-suffix shape with the basename shape:

Before (post-AAB):
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```

After:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}hallmarks/${RBRN_SENTRY_HALLMARK}/image
```

Vessel disappears from compose path. Confirm `latest` tag default (or explicit) on the basename-style refs.

### Spec updates

- `RBSAC-ark_conjure.adoc` — new basename layout
- `RBSAE-ark_enshrine.adoc` — enshrines plural namespace; anchor moves from tag to path segment
- `RBSDI-depot_inscribe.adoc` — reliquary lands under `<prefix>reliquaries/<date>/{tools}`; document `_RBGN_CLOUD_PREFIX` substitution
- `RBSAA-ark_abjure.adoc` — subtree delete replaces per-suffix dance
- `RBSAV-ark_vouch.adoc` — vouch path/lookup pattern
- `RBSAS-ark_summon.adoc` — summon path
- `RBSAP-ark_plumb.adoc` — plumb path
- `RBSCL-consecration_tally.adoc` — tally traversal pattern (subtree under `<prefix>hallmarks/`)
- `RBS0-SpecTop.adoc` — ark-layout prose
- `RBSCO-CosmologyIntro.adoc` — if it references the old layout

### Reliquary accumulation policy

Each inscribe creates a new datestamped reliquary; old reliquaries persist until explicitly cleaned. No GC mechanism exists today; out of scope for AAK. Captured here so the eventual GC question isn't lost.

### Pouch-on-graft

Conjure and bind produce pouch; graft does not. A grafted hallmark has 5 children (image, vouch, about, attest, diags), not 6. Abjure subtree-delete must handle missing arks gracefully (probably already does — confirm).

### Verification

- `grep -nE 'RBGC_ARK_SUFFIX_' Tools/` returns zero (renamed)
- `grep -nE 'RBGC_VOUCHES_PACKAGE' Tools/` returns zero
- `grep -nE ':\$\{[^}]*HALLMARK[^}]*\}-' Tools/ .rbk/` returns zero (old hyphen-suffix concat gone)
- `grep -nE 'RBGC_ARK_BASENAME_' Tools/ | grep -v rbgc_` — all hits go through path helpers, not direct concat
- After conjure of one hallmark: render subtree, confirm `image, vouch, pouch, about, attest, diags` as basenames
- After graft of one hallmark: render subtree, confirm 5 basenames (no pouch), abjure clean
- After inscribe: `<prefix>reliquaries/<date>/` contains all 6 tool basenames as siblings
- After enshrine: `<prefix>enshrines/<anchor>` is a package, not a tag on a shared package
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green
- `tt/rbtd-s.TestSuite.crucible.sh` — 117/117 green

### Out of scope

- Tabtarget signature simplification (₢A_AAL)
- Depot regen execution (₢A_AAE)
- Reliquary garbage collection (deferred — captured above)
- Filesystem moorings rename (separate follow-up heat per ₢A_AAJ)

**[260423-0858] rough**

## Character
Structural — reshapes ark storage in GAR and the path grammar all producer/consumer code uses. Atomic cutover: old and new shapes do not coexist. The path-construction helper minted here is the single point of truth that makes the cutover tractable and the abjure cleanup possible.

## Docket

Migrate GAR top-level namespace from vessel-scattered hyphen-suffix (`<vessel>:<hallmark>-<suffix>`) to three prefixed categorical namespaces with basename arks under each hallmark:

```
${RBRR_CLOUD_PREFIX}hallmarks/<hallmark>/{image, vouch, pouch, about, attest, diags}
${RBRR_CLOUD_PREFIX}reliquaries/<date>/<tool>
${RBRR_CLOUD_PREFIX}enshrines/<base>
```

Each immediate child of `<prefix>hallmarks` IS a hallmark; the six ark types become plain basename siblings. The `<vessel>:<hallmark>-<suffix>` scheme and the `vouches/` aggregator package both dissolve.

### Vessel identity (no producer change required)

Verified: `rbfv_FoundryVerify.sh:310,335,499,520,724,746` already emits `_RBGA_VESSEL` and `_RBGV_VESSEL` fields into vouch/about JSON. AAL reads these on demand; no producer change in AAK.

### Constants — `Tools/rbk/rbgc_Constants.sh`

- Rename `RBGC_ARK_SUFFIX_{IMAGE,VOUCH,POUCH,ABOUT,ATTEST,DIAGS}` → `RBGC_ARK_BASENAME_*`; values drop the leading hyphen (line 122–127).
- Delete `RBGC_VOUCHES_PACKAGE` (line 131) — hallmark-only vouch lookup comes free via subtree navigation.
- Add categorical-namespace constants:
  ```
  readonly RBGC_GAR_CATEGORY_HALLMARKS="hallmarks"
  readonly RBGC_GAR_CATEGORY_RELIQUARIES="reliquaries"
  readonly RBGC_GAR_CATEGORY_ENSHRINES="enshrines"
  ```

### Path-construction helper — single source of truth

Mint helper(s) in a stable location (suggest `Tools/rbk/rbgc_Constants.sh` or new `rbgp_GarPath.sh`):

```
rbgc_hallmark_ark_path()    # (hallmark, ark_basename) -> "${cloud_prefix}hallmarks/${hallmark}/${ark_basename}"
rbgc_hallmark_subtree()     # (hallmark) -> "${cloud_prefix}hallmarks/${hallmark}"
rbgc_reliquary_path()       # (date, tool) -> "${cloud_prefix}reliquaries/${date}/${tool}"
rbgc_enshrine_path()        # (base) -> "${cloud_prefix}enshrines/${base}"
```

**Discipline:** ALL producer and consumer sites route through these helpers. No more direct concat. Verification step `grep -nE 'RBGC_ARK_BASENAME_' Tools/ | grep -v rbgc_` should return zero outside the helpers — every other reference goes through the helper.

### Producer sites (route through helper)

- `rbfd_FoundryDirectorBuild.sh` — conjure (image, vouch, pouch, about, attest, diags), bind, graft (no pouch). Affected lines: 602, 733, 1178–1180, 1258–1259, 1369–1370, 1488.
- `rbfc_FoundryCore.sh` — lines 654, 681, 691, 705.
- `rbfv_FoundryVerify.sh` — vouch/about emission. Lines 75, 130, 154, 212, 317–321, 395–396, 510–512, 605, 629, 729–732.
- `rbga_ArtifactRegistry.sh` — GAR API wrappers; reshape any path construction.
- Enshrine pipeline (locate by grep `enshrine` in foundry) — emit under `<prefix>enshrines/<base>`.
- Inscribe pipeline (locate via `rbw-dI` -> `RBSDI`) — emit under `<prefix>reliquaries/<date>/<tool>`.

### Consumer sites (route through helper)

- `rbob_bottle.sh:126–131` — `ZRBOB_*_IMAGE`, `ZRBOB_*_VOUCH` construction.
- `rbrn_cli.sh:55–56` — render-time image refs.
- `rbfr_FoundryRetriever.sh:150,171–173` — pull paths.
- `rbfv_FoundryVerify.sh` (consumer half) — verify reads.
- `rbfl_FoundryLedger.sh:440–650` — abjure per-suffix HEAD/DELETE dance collapses to single subtree enumerate + delete via `rbgc_hallmark_subtree()`.
- `rbfl_FoundryLedger.sh` — Jettison/Rekons/Wrest implementations (image-management tabtargets currently live here).

### Compose.yml reshape

`.rbk/rbob_compose.yml:23,57,77` — replace the vessel-suffix shape with the basename shape:

Before (post-AAB):
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_HALLMARK}-image
```

After:
```
${RBRR_GCP_REGION}-docker.pkg.dev/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRR_CLOUD_PREFIX}hallmarks/${RBRN_SENTRY_HALLMARK}/image
```

Vessel disappears from compose path. Confirm `latest` tag default (or explicit) on the basename-style refs.

### Spec updates

- `RBSAC-ark_conjure.adoc` — new basename layout
- `RBSAE-ark_enshrine.adoc` — enshrine lands under `<prefix>enshrines`
- `RBSDI-depot_inscribe.adoc` — reliquary lands under `<prefix>reliquaries`
- `RBSAA-ark_abjure.adoc` — subtree delete replaces per-suffix dance
- `RBSAV-ark_vouch.adoc` — vouch path/lookup pattern
- `RBSAS-ark_summon.adoc` — summon path
- `RBSAP-ark_plumb.adoc` — plumb path
- `RBSCL-consecration_tally.adoc` — tally traversal pattern (subtree under `<prefix>hallmarks/`)
- `RBS0-SpecTop.adoc` — ark-layout prose
- `RBSCO-CosmologyIntro.adoc` — if it references the old layout

### Pouch-on-graft

Conjure and bind produce pouch; graft does not. A grafted hallmark has 5 children (image, vouch, about, attest, diags), not 6. Abjure subtree-delete must handle missing arks gracefully (probably already does — confirm).

### Verification

- `grep -nE 'RBGC_ARK_SUFFIX_' Tools/` returns zero (renamed)
- `grep -nE 'RBGC_VOUCHES_PACKAGE' Tools/` returns zero
- `grep -nE ':\$\{[^}]*HALLMARK[^}]*\}-' Tools/ .rbk/` returns zero (old hyphen-suffix concat gone)
- `grep -nE 'RBGC_ARK_BASENAME_' Tools/ | grep -v rbgc_` — all hits go through path helpers, not direct concat
- After conjure of one hallmark: render subtree, confirm `image, vouch, pouch, about, attest, diags` as basenames
- After graft of one hallmark: render subtree, confirm 5 basenames (no pouch), abjure clean
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green
- `tt/rbtd-s.TestSuite.crucible.sh` — 117/117 green

### Out of scope

- Tabtarget signature simplification (₢A_AAL)
- Depot regen execution (₢A_AAE)
- Filesystem moorings rename (separate heat per ₢A_AAJ)

**[260423-0821] rough**

## Character

Structural — reshapes how arks are stored in GAR and how consumer code constructs paths. Requires attention to the split between producer sites (foundry conjure, enshrine, inscribe) and consumer sites (tally, summon, plumb, abjure). Not mechanical — touches the core abstraction of ark storage. Atomic cutover: old and new path shapes do not coexist in live code.

## Docket

Migrate GAR top-level namespace from the current vessel-scattered, hyphen-suffix layout to three prefixed categorical namespaces:

```
<prefix>hallmarks/<hallmark>/{image, vouch, pouch, about, attest, diags}
<prefix>reliquaries/<date>/<tool>
<prefix>enshrines/<base>
```

Each immediate child of `<prefix>hallmarks` IS a hallmark; the six ark types become plain basename siblings. The `<vessel>:<hallmark>-<suffix>` scheme and the `vouches/` aggregator package both dissolve.

### Constants (`Tools/rbk/rbgc_Constants.sh`)

- Rename `RBGC_ARK_SUFFIX_{IMAGE,VOUCH,POUCH,ABOUT,ATTEST,DIAGS}` to `RBGC_ARK_BASENAME_*`; values drop the leading hyphen.
- Delete `RBGC_VOUCHES_PACKAGE` — hallmark-only vouch lookup now comes free via subtree navigation.
- Add categorical-namespace constants: `RBGC_GAR_CATEGORY_HALLMARKS="hallmarks"`, `_RELIQUARIES="reliquaries"`, `_ENSHRINES="enshrines"`.

### Vessel identity

Not in the GAR path. Resolved from the `vouch` or `about` metadata for the hallmark in question. Tally already reads metadata for health; other commands extend the pattern where operation prose or logging requires vessel name.

### Attest-vs-vouch split

Both persist as peer basenames under `<hallmark>/`. Attestations advertise during build for SLSA provenance; vouches check them at consumption. Load-bearing split retained.

### Producer-site updates

Every site that pushes an ark — conjure (image, vouch, pouch, about, attest, diags), enshrine, inscribe — constructs paths in the new shape. Concentrate path construction in a helper if current code scatters it.

### Consumer-site updates

Summon, plumb, vouch-verify, tally — switch to new path shape. Abjure simplifies considerably: the per-suffix HEAD/DELETE dance in `rbfl_FoundryLedger.sh:440-650` collapses to a single subtree enumeration + delete loop.

### Spec updates

- `RBSAC-ark_conjure.adoc` — new basename layout
- `RBSAE-ark_enshrine.adoc` — enshrine lands under `<prefix>enshrines`
- `RBSDI-depot_inscribe.adoc` — reliquary lands under `<prefix>reliquaries`
- `RBSAA-ark_abjure.adoc` — subtree delete replaces per-suffix dance
- `RBS0-SpecTop.adoc` — update ark-layout prose
- Any spec referencing the `vouches/` aggregator — rewrite for hallmark-only lookup via subtree

### Depends on

- ₢A_AAA — prefix variables enrolled
- ₢A_AAB — prefix wired through consumers; this pace reshapes the paths AAB wires

### Verification

- `grep -nE 'RBGC_ARK_SUFFIX_' Tools/` returns zero (renamed to `_BASENAME_`)
- `grep -nE 'RBGC_VOUCHES_PACKAGE' Tools/` returns zero
- `grep -nE ':\$\{[^}]*hallmark[^}]*\}-' Tools/` returns zero for the old concat pattern
- Render one hallmark's subtree after conjure — confirms image/vouch/pouch/about/attest/diags as basenames under `<prefix>hallmarks/<hallmark>/`
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green

### Out of scope

- Tabtarget signature simplification — follow-up pace
- Depot regen execution — ₢A_AAE
- Filesystem moorings rename — separate heat per ₢A_AAJ decision

### tally-hallmark-enumeration-rewrite (₢A_AAO) [complete]

**[260423-1341] complete**

## Character

Architectural rewrite — both `rbfl_tally` and `rbfv_batch_vouch` shared a per-vessel tag-enumeration shape (Docker Registry v2 `/tags/list` keyed on vessel sigil + case-on-suffix classifier) that dissolved with the GAR categorical-layout migration. They now need a per-hallmark package-enumeration implementation (GAR REST `/packages` keyed under `${RBGL_HALLMARKS_ROOT}/<hallmark>/`). Health/state classification moves from per-tag suffix matching to per-hallmark basename presence in the package set. Vessel column drops in AAK; restoration via vouch/about metadata is AAL territory per the heat paddock.

## Docket

Replace two stubbed function bodies (`rbfl_tally` and `rbfv_batch_vouch`, both stubbed in ₢A_AAK Notch 2b with `buc_die` pointing here) with hallmark-enumeration implementations against the new GAR categorical layout.

### Why these are stubbed

Both functions were stubbed in ₢A_AAK Notch 2b because the Notch 2a constant deletion broke their shared shape:

1. **URL target** — `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_sigil}/tags/list` no longer returns anything; packages live under `hallmarks/<H>/<basename>` not `<sigil>/...`
2. **Case-on-suffix classifier** — references deleted `RBGC_ARK_SUFFIX_*` constants; even renaming to `RBGC_ARK_BASENAME_*` doesn't restore semantics (basenames are package directories, not tag suffixes)

Stubbing was correct in 2b — the rewrite is architecturally substantial and deserves its own focused notch rather than getting jammed into the cross-file 2b cutover.

### Shared subroutine: list hallmarks

Both functions need: list all packages in repo, filter to `${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename>` shape, group by hallmark, build map<hallmark, set<basename>>. Likely lift to a shared helper in `rbfc_FoundryCore.sh`:

```
zrbfc_list_hallmarks_capture <token>  # writes hallmark→basenames map to scratch file
```

API call (one per invocation, regardless of vessel count):
```
GET ${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000
```
Pagination via `pageToken` deferred until evidence of >1000 packages. GAR returns names URL-encoded (slashes as `%2F`); decode then prefix-match.

### `rbfl_tally` shape

1. Authenticate as Retriever (existing read-only credential — unchanged).
2. Call shared list-hallmarks subroutine.
3. For each hallmark, classify health from basename presence:
   - `vouched`: image + about + vouch all present
   - `pending`: image + about, no vouch
   - `incomplete`: image only (or any other non-healthy combination)
4. Per-hallmark fact emission: revisit `buf_write_fact "${z_sigil}${RBCC_FACT_CONSEC_INFIX}${z_hallmark}" "${z_sigil}"` pattern. Without vessel in path, fact key shape needs adjustment — check fixture consumers (`grep -nE 'RBCC_FACT_CONSEC_INFIX' Tools/rbtd/`).
5. Display table: hallmark, basenames-present, health.
6. Tabtarget recommendations (RBZ_VOUCH_HALLMARKS for pending, RBZ_ABJURE_HALLMARK for incomplete) — unchanged.

### `rbfv_batch_vouch` shape

1. Authenticate as Director (existing).
2. Call shared list-hallmarks subroutine.
3. Filter to "pending" hallmarks (about present, vouch absent).
4. For each pending hallmark, resolve vessel directory (read vouch/about metadata for vessel name → map to `${RBRR_VESSEL_DIR}/<vessel>` — but vouch metadata only exists for already-vouched hallmarks, so for pending hallmarks vessel must come from about metadata) and call `rbfv_vouch <vessel_dir> <hallmark>`.
5. Track vouched/already/failed counts; print summary.

The vessel-resolution step is the AAL-territory work that the heat paddock defers — but batch_vouch needs vessel to call rbfv_vouch. Either (a) read vessel from about metadata in batch_vouch (anticipating AAL), or (b) further-defer batch_vouch until AAL lands the vessel-from-metadata pattern, or (c) simplify batch_vouch to not need vessel (rewrite rbfv_vouch signature to take only hallmark).

### Vessel column

Drops in AAK. Restoration is AAL scope per the heat paddock — AAL adds vessel-from-vouch-metadata reading on demand via the `_RBGV_VESSEL` field that rbfv already emits into vouch JSON.

### Files

- `Tools/rbk/rbfl_FoundryLedger.sh` — replace `rbfl_tally` body
- `Tools/rbk/rbfv_FoundryVerify.sh` — replace `rbfv_batch_vouch` body
- `Tools/rbk/rbfc_FoundryCore.sh` — add shared list-hallmarks helper

### Verification

- Fast suite 90/90 green
- Crucible suite tally fixture (if exists) green
- Manual: tally empty repo, tally a single conjure hallmark, tally mix of vouched/pending/incomplete
- Manual: batch_vouch with no pending, batch_vouch with multiple pending across multiple vessels

### References

- `Tools/rbk/rbfl_FoundryLedger.sh` — `rbfl_tally` stubbed in 2b
- `Tools/rbk/rbfv_FoundryVerify.sh` — `rbfv_batch_vouch` stubbed in 2b
- Pace ₢A_AAK — stubs both functions with pointers to this pace
- `RBSCL-consecration_tally.adoc` — tally spec target (Notch 3 of ₢A_AAK)
- `RBSAV-ark_vouch.adoc` — vouch/batch_vouch spec target (Notch 3 of ₢A_AAK)
- `Tools/rbk/rbfc_FoundryCore.sh:41,45` — `ZRBFC_GAR_API_BASE`, `ZRBFC_GAR_PACKAGE_BASE`
- `Tools/rbk/rbgl_GarLayout.sh:48` — `RBGL_HALLMARKS_ROOT`

**[260423-1213] rough**

## Character

Architectural rewrite — both `rbfl_tally` and `rbfv_batch_vouch` shared a per-vessel tag-enumeration shape (Docker Registry v2 `/tags/list` keyed on vessel sigil + case-on-suffix classifier) that dissolved with the GAR categorical-layout migration. They now need a per-hallmark package-enumeration implementation (GAR REST `/packages` keyed under `${RBGL_HALLMARKS_ROOT}/<hallmark>/`). Health/state classification moves from per-tag suffix matching to per-hallmark basename presence in the package set. Vessel column drops in AAK; restoration via vouch/about metadata is AAL territory per the heat paddock.

## Docket

Replace two stubbed function bodies (`rbfl_tally` and `rbfv_batch_vouch`, both stubbed in ₢A_AAK Notch 2b with `buc_die` pointing here) with hallmark-enumeration implementations against the new GAR categorical layout.

### Why these are stubbed

Both functions were stubbed in ₢A_AAK Notch 2b because the Notch 2a constant deletion broke their shared shape:

1. **URL target** — `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_sigil}/tags/list` no longer returns anything; packages live under `hallmarks/<H>/<basename>` not `<sigil>/...`
2. **Case-on-suffix classifier** — references deleted `RBGC_ARK_SUFFIX_*` constants; even renaming to `RBGC_ARK_BASENAME_*` doesn't restore semantics (basenames are package directories, not tag suffixes)

Stubbing was correct in 2b — the rewrite is architecturally substantial and deserves its own focused notch rather than getting jammed into the cross-file 2b cutover.

### Shared subroutine: list hallmarks

Both functions need: list all packages in repo, filter to `${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename>` shape, group by hallmark, build map<hallmark, set<basename>>. Likely lift to a shared helper in `rbfc_FoundryCore.sh`:

```
zrbfc_list_hallmarks_capture <token>  # writes hallmark→basenames map to scratch file
```

API call (one per invocation, regardless of vessel count):
```
GET ${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000
```
Pagination via `pageToken` deferred until evidence of >1000 packages. GAR returns names URL-encoded (slashes as `%2F`); decode then prefix-match.

### `rbfl_tally` shape

1. Authenticate as Retriever (existing read-only credential — unchanged).
2. Call shared list-hallmarks subroutine.
3. For each hallmark, classify health from basename presence:
   - `vouched`: image + about + vouch all present
   - `pending`: image + about, no vouch
   - `incomplete`: image only (or any other non-healthy combination)
4. Per-hallmark fact emission: revisit `buf_write_fact "${z_sigil}${RBCC_FACT_CONSEC_INFIX}${z_hallmark}" "${z_sigil}"` pattern. Without vessel in path, fact key shape needs adjustment — check fixture consumers (`grep -nE 'RBCC_FACT_CONSEC_INFIX' Tools/rbtd/`).
5. Display table: hallmark, basenames-present, health.
6. Tabtarget recommendations (RBZ_VOUCH_HALLMARKS for pending, RBZ_ABJURE_HALLMARK for incomplete) — unchanged.

### `rbfv_batch_vouch` shape

1. Authenticate as Director (existing).
2. Call shared list-hallmarks subroutine.
3. Filter to "pending" hallmarks (about present, vouch absent).
4. For each pending hallmark, resolve vessel directory (read vouch/about metadata for vessel name → map to `${RBRR_VESSEL_DIR}/<vessel>` — but vouch metadata only exists for already-vouched hallmarks, so for pending hallmarks vessel must come from about metadata) and call `rbfv_vouch <vessel_dir> <hallmark>`.
5. Track vouched/already/failed counts; print summary.

The vessel-resolution step is the AAL-territory work that the heat paddock defers — but batch_vouch needs vessel to call rbfv_vouch. Either (a) read vessel from about metadata in batch_vouch (anticipating AAL), or (b) further-defer batch_vouch until AAL lands the vessel-from-metadata pattern, or (c) simplify batch_vouch to not need vessel (rewrite rbfv_vouch signature to take only hallmark).

### Vessel column

Drops in AAK. Restoration is AAL scope per the heat paddock — AAL adds vessel-from-vouch-metadata reading on demand via the `_RBGV_VESSEL` field that rbfv already emits into vouch JSON.

### Files

- `Tools/rbk/rbfl_FoundryLedger.sh` — replace `rbfl_tally` body
- `Tools/rbk/rbfv_FoundryVerify.sh` — replace `rbfv_batch_vouch` body
- `Tools/rbk/rbfc_FoundryCore.sh` — add shared list-hallmarks helper

### Verification

- Fast suite 90/90 green
- Crucible suite tally fixture (if exists) green
- Manual: tally empty repo, tally a single conjure hallmark, tally mix of vouched/pending/incomplete
- Manual: batch_vouch with no pending, batch_vouch with multiple pending across multiple vessels

### References

- `Tools/rbk/rbfl_FoundryLedger.sh` — `rbfl_tally` stubbed in 2b
- `Tools/rbk/rbfv_FoundryVerify.sh` — `rbfv_batch_vouch` stubbed in 2b
- Pace ₢A_AAK — stubs both functions with pointers to this pace
- `RBSCL-consecration_tally.adoc` — tally spec target (Notch 3 of ₢A_AAK)
- `RBSAV-ark_vouch.adoc` — vouch/batch_vouch spec target (Notch 3 of ₢A_AAK)
- `Tools/rbk/rbfc_FoundryCore.sh:41,45` — `ZRBFC_GAR_API_BASE`, `ZRBFC_GAR_PACKAGE_BASE`
- `Tools/rbk/rbgl_GarLayout.sh:48` — `RBGL_HALLMARKS_ROOT`

**[260423-1150] rough**

## Character

Architectural rewrite — tally's enumeration shape changes from per-vessel tag listing (Docker Registry v2 `/tags/list`) to per-hallmark package listing (GAR REST API `/packages`). The basename-keyed layout means health is determined from the SET of basenames present under each hallmark subtree, not from per-tag suffix matching. Vessel column drops in AAK; restoration via vouch/about metadata is AAL territory per the heat paddock.

## Docket

Replace the `rbfl_tally` body (stubbed in ₢A_AAK Notch 2b with `buc_die` pointing here) with a hallmark-enumeration implementation against the new GAR categorical layout.

### Why this exists

`rbfl_tally` was stubbed in ₢A_AAK Notch 2b because the Notch 2a constant deletion broke both:

1. **URL target** — `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_sigil}/tags/list` no longer returns anything; packages live under `hallmarks/<H>/<basename>` not `<sigil>/...`
2. **Case-on-suffix classifier** — references deleted `RBGC_ARK_SUFFIX_*` constants; even renaming to `RBGC_ARK_BASENAME_*` doesn't restore semantics (basenames are package directories, not tag suffixes)

Stubbing was correct in 2b — the rewrite is architecturally substantial and deserves its own focused notch rather than getting jammed into the cross-file 2b cutover.

### New shape

1. Authenticate as Retriever (existing read-only credential — unchanged).
2. List all packages in the GAR repo via GAR REST API:
   ```
   GET ${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000
   ```
   Pagination via `pageToken` deferred until evidence of >1000 packages.
3. Filter package names to `${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename>` shape:
   - GAR returns names URL-encoded (slashes as `%2F`); decode before matching
   - Strip the `projects/.../packages/` prefix
4. Group decoded names by hallmark (parent directory): map<hallmark, set<basename>>.
5. For each hallmark, classify health from basename presence:
   - `vouched`: image + about + vouch all present
   - `pending`: image + about, no vouch
   - `incomplete`: image only (or any other non-healthy combination)
6. Per-hallmark fact emission: revisit `buf_write_fact "${z_sigil}${RBCC_FACT_CONSEC_INFIX}${z_hallmark}" "${z_sigil}"` pattern. Without vessel in path, fact key shape needs adjustment — check fixture consumers (`grep -nE 'RBCC_FACT_CONSEC_INFIX' Tools/rbtd/`).
7. Display table: hallmark, basenames-present, health.
8. Tabtarget recommendations (RBZ_VOUCH_HALLMARKS for pending, RBZ_ABJURE_HALLMARK for incomplete) — unchanged.

### Vessel column

Drops in AAK. Restoration is AAL scope per the heat paddock — AAL adds vessel-from-vouch-metadata reading on demand via the `_RBGV_VESSEL` field that rbfv already emits into vouch JSON.

### Files

- `Tools/rbk/rbfl_FoundryLedger.sh` — replace `rbfl_tally` body
- Possibly `Tools/rbk/rbfc_FoundryCore.sh` — extract a GAR REST list-packages helper if rekon (which currently uses Docker v2 `/tags/list`) also migrates

### Verification

- Fast suite 90/90 green
- Crucible suite tally fixture (if exists) green
- Manual: tally empty repo, tally a single conjure hallmark, tally mix of vouched/pending/incomplete

### References

- `Tools/rbk/rbfl_FoundryLedger.sh:675-834` — current `rbfl_tally` (broken since Notch 2a; stubbed in 2b)
- Pace ₢A_AAK — stubs tally with pointer to this pace
- `RBSCL-consecration_tally.adoc` — spec target (Notch 3 of ₢A_AAK covers spec sweep)
- `Tools/rbk/rbfc_FoundryCore.sh:41,45` — `ZRBFC_GAR_API_BASE`, `ZRBFC_GAR_PACKAGE_BASE`
- `Tools/rbk/rbgl_GarLayout.sh:48` — `RBGL_HALLMARKS_ROOT`

### tabtarget-vessel-param-removal (₢A_AAL) [complete]

**[260423-1341] complete**

## Character

Mechanical once ₢A_AAK lands. CLI UX simplification — vessel parameter drops from tabtargets whose folio was `(vessel, hallmark)`. Review attention: confirm the metadata-resolution path (vessel from vouch/about) is exercised and yields correct vessel identity for every affected command.

## Docket

After the GAR categorical layout migration (₢A_AAK) lands, vessel is no longer part of the GAR path. Tabtargets that previously took `(vessel, hallmark)` to construct ark paths simplify to `(hallmark)` alone. Internal vessel identity is resolved on demand from the hallmark's `vouch` or `about` metadata.

### Affected tabtargets (verify and refine)

Foundry and image families — confirm the exact list by grepping for dispatch channels that take both params. Expected targets:

- `rbw-fA` DirectorAbjuresHallmark
- `rbw-fV` DirectorVouchesHallmarks
- `rbw-fs` RetrieverSummonsHallmark
- `rbw-fpf` RetrieverPlumbsFull
- `rbw-fpc` RetrieverPlumbsCompact
- `rbw-ft` RetrieverTalliesHallmarks (if vessel-filtered)
- `rbw-iJ` DirectorJettisonsImage
- `rbw-ir` DirectorRekonsImages
- `rbw-iw` DirectorWrestsImage

### NOT affected

Nameplate-moniker tabtargets (Charge, Quench, Rack, KludgeBottle, Hail, Writ, Fiat, Bark, SshTo, Ifrit-*) pass a nameplate moniker — not a vessel for path construction. Different axis; unchanged.

### Per-tabtarget changes

- Folio channel: `param1`+`param2` collapses to `param1` (hallmark). Hallmarks are dynamic (datestamp + sha) so `imprint` does not apply — param1 is the answer for every affected tabtarget.
- Workbench dispatch simplifies accordingly (channel declarations in zipper enrollments, dispatch tables in workbench routing).
- Underlying command-function signatures in foundry/image modules lose the vessel parameter.

### Vessel-resolution helper

Mint `rbfc_vessel_for_hallmark(hallmark)` in `Tools/rbk/rbfc_FoundryCore.sh`. Reads the `_RBGA_VESSEL` field from the hallmark's vouch metadata (already emitted per AAK verification — see `rbfv_FoundryVerify.sh:310,335,499,520`). Single home prevents reinvention at each call site.

Call sites that need vessel name for operation prose or logging route through the helper. Resolution is metadata-read only — no second source of truth.

### Cross-cutting

- Zipper enrollment updates for affected colophons (channel declaration changes from `param1+param2` to `param1`)
- `tt/rbw-MG.MarshalGenerate.sh` regenerates `Tools/rbk/rbk-claude-tabtarget-context.md` to reflect the new signatures

### Depends on

- ₢A_AAK — vessel must resolve from metadata in the new GAR layout before it can leave the CLI surface

### Verification

- Affected tabtargets take hallmark-only params; none reference the old vessel param
- A representative flow (summon -> plumb -> abjure against a known hallmark) runs end-to-end without the user specifying vessel
- `rbfc_vessel_for_hallmark()` returns the correct vessel for at least one conjure-mode hallmark and one graft-mode hallmark
- Regenerated `rbk-claude-tabtarget-context.md` reflects the new signatures
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green
- `tt/rbtd-s.TestSuite.service.sh` — 80/80 green (if credentials available)

### Out of scope

- Layout migration itself — predecessor ₢A_AAK
- Nameplate-moniker tabtargets — different axis
- Depot regen — ₢A_AAE

**[260423-0911] rough**

## Character

Mechanical once ₢A_AAK lands. CLI UX simplification — vessel parameter drops from tabtargets whose folio was `(vessel, hallmark)`. Review attention: confirm the metadata-resolution path (vessel from vouch/about) is exercised and yields correct vessel identity for every affected command.

## Docket

After the GAR categorical layout migration (₢A_AAK) lands, vessel is no longer part of the GAR path. Tabtargets that previously took `(vessel, hallmark)` to construct ark paths simplify to `(hallmark)` alone. Internal vessel identity is resolved on demand from the hallmark's `vouch` or `about` metadata.

### Affected tabtargets (verify and refine)

Foundry and image families — confirm the exact list by grepping for dispatch channels that take both params. Expected targets:

- `rbw-fA` DirectorAbjuresHallmark
- `rbw-fV` DirectorVouchesHallmarks
- `rbw-fs` RetrieverSummonsHallmark
- `rbw-fpf` RetrieverPlumbsFull
- `rbw-fpc` RetrieverPlumbsCompact
- `rbw-ft` RetrieverTalliesHallmarks (if vessel-filtered)
- `rbw-iJ` DirectorJettisonsImage
- `rbw-ir` DirectorRekonsImages
- `rbw-iw` DirectorWrestsImage

### NOT affected

Nameplate-moniker tabtargets (Charge, Quench, Rack, KludgeBottle, Hail, Writ, Fiat, Bark, SshTo, Ifrit-*) pass a nameplate moniker — not a vessel for path construction. Different axis; unchanged.

### Per-tabtarget changes

- Folio channel: `param1`+`param2` collapses to `param1` (hallmark). Hallmarks are dynamic (datestamp + sha) so `imprint` does not apply — param1 is the answer for every affected tabtarget.
- Workbench dispatch simplifies accordingly (channel declarations in zipper enrollments, dispatch tables in workbench routing).
- Underlying command-function signatures in foundry/image modules lose the vessel parameter.

### Vessel-resolution helper

Mint `rbfc_vessel_for_hallmark(hallmark)` in `Tools/rbk/rbfc_FoundryCore.sh`. Reads the `_RBGA_VESSEL` field from the hallmark's vouch metadata (already emitted per AAK verification — see `rbfv_FoundryVerify.sh:310,335,499,520`). Single home prevents reinvention at each call site.

Call sites that need vessel name for operation prose or logging route through the helper. Resolution is metadata-read only — no second source of truth.

### Cross-cutting

- Zipper enrollment updates for affected colophons (channel declaration changes from `param1+param2` to `param1`)
- `tt/rbw-MG.MarshalGenerate.sh` regenerates `Tools/rbk/rbk-claude-tabtarget-context.md` to reflect the new signatures

### Depends on

- ₢A_AAK — vessel must resolve from metadata in the new GAR layout before it can leave the CLI surface

### Verification

- Affected tabtargets take hallmark-only params; none reference the old vessel param
- A representative flow (summon -> plumb -> abjure against a known hallmark) runs end-to-end without the user specifying vessel
- `rbfc_vessel_for_hallmark()` returns the correct vessel for at least one conjure-mode hallmark and one graft-mode hallmark
- Regenerated `rbk-claude-tabtarget-context.md` reflects the new signatures
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green
- `tt/rbtd-s.TestSuite.service.sh` — 80/80 green (if credentials available)

### Out of scope

- Layout migration itself — predecessor ₢A_AAK
- Nameplate-moniker tabtargets — different axis
- Depot regen — ₢A_AAE

**[260423-0822] rough**

## Character

Mechanical once ₢A_AAK lands. CLI UX simplification — vessel parameter drops from tabtargets whose folio was `(vessel, hallmark)`. Review attention: confirm the metadata-resolution path (vessel from vouch/about) is exercised and yields correct vessel identity for every affected command.

## Docket

After the GAR categorical layout migration (₢A_AAK) lands, vessel is no longer part of the GAR path. Tabtargets that previously took `(vessel, hallmark)` to construct ark paths simplify to `(hallmark)` alone. Internal vessel identity is resolved on demand from the hallmark's `vouch` or `about` metadata.

### Affected tabtargets (verify and refine)

Foundry and image families — confirm the exact list by grepping for dispatch channels that take both params. Expected targets:

- `rbw-fA` DirectorAbjuresHallmark
- `rbw-fV` DirectorVouchesHallmarks
- `rbw-fs` RetrieverSummonsHallmark
- `rbw-fpf` RetrieverPlumbsFull
- `rbw-fpc` RetrieverPlumbsCompact
- `rbw-ft` RetrieverTalliesHallmarks (if vessel-filtered)
- `rbw-iJ` DirectorJettisonsImage
- `rbw-ir` DirectorRekonsImages
- `rbw-iw` DirectorWrestsImage

### NOT affected

Nameplate-moniker tabtargets (Charge, Quench, Rack, KludgeBottle, Hail, Writ, Fiat, Bark, SshTo, Ifrit-*) pass a nameplate moniker — not a vessel for path construction. Different axis; unchanged.

### Per-tabtarget changes

- Folio channel: `param1`+`param2` collapses to `param1` (hallmark) — or `imprint` if the hallmark is bakeable into the filename for the given colophon
- Workbench dispatch simplifies accordingly
- Underlying command-function signatures in foundry/image modules lose the vessel parameter
- Internal vessel-resolution reads vouch/about metadata when operation prose or logging requires the vessel name

### Cross-cutting

- Zipper enrollment updates for affected colophons (channel declaration changes)
- `tt/rbw-MG.MarshalGenerate.sh` regenerates `Tools/rbk/rbk-claude-tabtarget-context.md` to reflect the new signatures

### Depends on

- ₢A_AAK — vessel must resolve from metadata in the new GAR layout before it can leave the CLI surface

### Verification

- Affected tabtargets take hallmark-only params; none reference the old vessel param
- A representative flow (summon → plumb → abjure against a known hallmark) runs end-to-end without the user specifying vessel
- Regenerated `rbk-claude-tabtarget-context.md` reflects the new signatures
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green
- `tt/rbtd-s.TestSuite.service.sh` — 80/80 green (if credentials available)

### Out of scope

- Layout migration itself — predecessor ₢A_AAK
- Nameplate-moniker tabtargets — different axis
- Depot regen — ₢A_AAE

### rbro-payor-subdirectory-migration (₢A_AAD) [complete]

**[260423-1343] complete**

Drafted from ₢A6AAl in ₣A6.

## Character
Mechanical refactoring with a migration shim — straightforward path surgery plus a tinder constant mint, verified end-to-end through governor reset proving payor operations survive the move. Includes a small retrofit of the existing RBRA migration to fail-loud on both-exist (consistency with the new payor migration policy).

## Docket
Move `rbro.env` from flat `RBRR_SECRETS_DIR/rbro.env` into `RBRR_SECRETS_DIR/payor/rbro.env` to match the established `{role}/` subdirectory pattern used by RBRA credentials (governor, retriever, director each have their own subdirectory).

### Mint tinder constant
Add `RBCC_role_payor="payor"` to `Tools/rbk/rbcc_Constants.sh` alongside the existing `RBCC_role_governor`, `RBCC_role_retriever`, `RBCC_role_director` tinder constants.

### Path change
In `Tools/rbk/rbdc_DerivedConstants.sh`, change:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
```
to:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
```

### Migration policy: fail-loud on both-exist

Both the new payor migration AND the existing RBRA migration in `zrbdc_kindle()` (lines 40–56) currently silently leave both files if both old and new exist. Replace with hard-fail, forcing human resolution.

**New payor migration block** (parallel to existing RBRA, but with fail-loud):
```bash
# Payor migration: rbro.env -> payor/rbro.env
local z_mig_payor_old="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
local z_mig_payor_new="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
if test -f "${z_mig_payor_old}" && test -f "${z_mig_payor_new}"; then
  buc_die "Migration ambiguous: both ${z_mig_payor_old} and ${z_mig_payor_new} exist; delete the obsolete file before continuing"
fi
if test -f "${z_mig_payor_old}"; then
  mv "${z_mig_payor_old}" "${z_mig_payor_new}" || buc_die "Failed to migrate: ${z_mig_payor_old} -> ${z_mig_payor_new}"
fi
```

**Retrofit existing RBRA migration** (lines 43–56) — add the both-exist guard to match the new policy. Inside the `for z_mig_role` loop, before the existing `if test -f "${z_mig_old}" && ! test -f "${z_mig_new}"; then` block, add:
```bash
if test -f "${z_mig_old}" && test -f "${z_mig_new}"; then
  buc_die "Migration ambiguous for role ${z_mig_role}: both ${z_mig_old} and ${z_mig_new} exist; delete the obsolete file before continuing"
fi
```

### Ensure payor directory creation
Add `${RBCC_role_payor}` to the `mkdir -p` block at lines 35–38 that already creates governor/retriever/director subdirectories.

### Doc/spec updates
Update literal `~/.rbw/rbro.env` references to `~/.rbw/payor/rbro.env` in:
- `RBSRO-RegimeOauth.adoc`
- `RBSGS-GettingStarted.adoc`
- `RBSPI-payor_install.adoc`
- `RBSPR-payor_refresh.adoc` (added — payor refresh handbook spec)
- `RBS0-SpecTop.adoc` (prose references to the file path)
- `CLAUDE.consumer.md`
- `rbhpr_refresh.sh` (handbook display text)
- `rblm_cli.sh` (marshal display text)

### Verification — governor reset cycle
After code changes, prove payor operations survive the migration by running a governor reset cycle:
1. `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime validation exercises RBDC paths)
2. `tt/rbw-aM.PayorMantlesGovernor.sh` — reset governor SA
3. `tt/rbw-aC.GovernorChartersRetriever.sh` — re-issue retriever RBRA
4. `tt/rbw-aK.GovernorKnightsDirector.sh` — re-issue director RBRA
5. Confirm all three RBRA files land in their role subdirectories and payor OAuth (`payor/rbro.env`) is still functional throughout

### Both-exist policy verification
- Stage a fake "both exist" scenario for payor: create both `~/.rbw/rbro.env` and `~/.rbw/payor/rbro.env`. Run any tabtarget that triggers `zrbdc_kindle`. Confirm `buc_die` with the expected message.
- Repeat for one RBRA role (e.g., governor): confirm retrofitted guard fires.

**[260423-0901] rough**

Drafted from ₢A6AAl in ₣A6.

## Character
Mechanical refactoring with a migration shim — straightforward path surgery plus a tinder constant mint, verified end-to-end through governor reset proving payor operations survive the move. Includes a small retrofit of the existing RBRA migration to fail-loud on both-exist (consistency with the new payor migration policy).

## Docket
Move `rbro.env` from flat `RBRR_SECRETS_DIR/rbro.env` into `RBRR_SECRETS_DIR/payor/rbro.env` to match the established `{role}/` subdirectory pattern used by RBRA credentials (governor, retriever, director each have their own subdirectory).

### Mint tinder constant
Add `RBCC_role_payor="payor"` to `Tools/rbk/rbcc_Constants.sh` alongside the existing `RBCC_role_governor`, `RBCC_role_retriever`, `RBCC_role_director` tinder constants.

### Path change
In `Tools/rbk/rbdc_DerivedConstants.sh`, change:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
```
to:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
```

### Migration policy: fail-loud on both-exist

Both the new payor migration AND the existing RBRA migration in `zrbdc_kindle()` (lines 40–56) currently silently leave both files if both old and new exist. Replace with hard-fail, forcing human resolution.

**New payor migration block** (parallel to existing RBRA, but with fail-loud):
```bash
# Payor migration: rbro.env -> payor/rbro.env
local z_mig_payor_old="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
local z_mig_payor_new="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
if test -f "${z_mig_payor_old}" && test -f "${z_mig_payor_new}"; then
  buc_die "Migration ambiguous: both ${z_mig_payor_old} and ${z_mig_payor_new} exist; delete the obsolete file before continuing"
fi
if test -f "${z_mig_payor_old}"; then
  mv "${z_mig_payor_old}" "${z_mig_payor_new}" || buc_die "Failed to migrate: ${z_mig_payor_old} -> ${z_mig_payor_new}"
fi
```

**Retrofit existing RBRA migration** (lines 43–56) — add the both-exist guard to match the new policy. Inside the `for z_mig_role` loop, before the existing `if test -f "${z_mig_old}" && ! test -f "${z_mig_new}"; then` block, add:
```bash
if test -f "${z_mig_old}" && test -f "${z_mig_new}"; then
  buc_die "Migration ambiguous for role ${z_mig_role}: both ${z_mig_old} and ${z_mig_new} exist; delete the obsolete file before continuing"
fi
```

### Ensure payor directory creation
Add `${RBCC_role_payor}` to the `mkdir -p` block at lines 35–38 that already creates governor/retriever/director subdirectories.

### Doc/spec updates
Update literal `~/.rbw/rbro.env` references to `~/.rbw/payor/rbro.env` in:
- `RBSRO-RegimeOauth.adoc`
- `RBSGS-GettingStarted.adoc`
- `RBSPI-payor_install.adoc`
- `RBSPR-payor_refresh.adoc` (added — payor refresh handbook spec)
- `RBS0-SpecTop.adoc` (prose references to the file path)
- `CLAUDE.consumer.md`
- `rbhpr_refresh.sh` (handbook display text)
- `rblm_cli.sh` (marshal display text)

### Verification — governor reset cycle
After code changes, prove payor operations survive the migration by running a governor reset cycle:
1. `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime validation exercises RBDC paths)
2. `tt/rbw-aM.PayorMantlesGovernor.sh` — reset governor SA
3. `tt/rbw-aC.GovernorChartersRetriever.sh` — re-issue retriever RBRA
4. `tt/rbw-aK.GovernorKnightsDirector.sh` — re-issue director RBRA
5. Confirm all three RBRA files land in their role subdirectories and payor OAuth (`payor/rbro.env`) is still functional throughout

### Both-exist policy verification
- Stage a fake "both exist" scenario for payor: create both `~/.rbw/rbro.env` and `~/.rbw/payor/rbro.env`. Run any tabtarget that triggers `zrbdc_kindle`. Confirm `buc_die` with the expected message.
- Repeat for one RBRA role (e.g., governor): confirm retrofitted guard fires.

**[260415-1504] rough**

Drafted from ₢A6AAl in ₣A6.

## Character
Mechanical refactoring with a migration shim — straightforward path surgery plus a tinder constant mint, verified end-to-end through governor reset proving payor operations survive the move.

## Docket
Move `rbro.env` from flat `RBRR_SECRETS_DIR/rbro.env` into `RBRR_SECRETS_DIR/payor/rbro.env` to match the established `{role}/` subdirectory pattern used by RBRA credentials (governor, retriever, director each have their own subdirectory).

### Mint tinder constant
Add `RBCC_role_payor="payor"` to `Tools/rbk/rbcc_Constants.sh` alongside the existing `RBCC_role_governor`, `RBCC_role_retriever`, `RBCC_role_director` tinder constants.

### Path change
In `Tools/rbk/rbdc_DerivedConstants.sh`, change:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
```
to:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
```

### One-shot migration
Add a migration block in `zrbdc_kindle()` (parallel to the existing RBRA migration at lines 40-56) that moves `RBRR_SECRETS_DIR/rbro.env` → `RBRR_SECRETS_DIR/payor/rbro.env` if the old path exists and the new path does not.

### Ensure payor directory creation
Add `${RBCC_role_payor}` to the `mkdir -p` block that already creates governor/retriever/director subdirectories.

### Doc/spec updates
Update literal `~/.rbw/rbro.env` references to `~/.rbw/payor/rbro.env` in:
- `RBSRO-RegimeOauth.adoc`
- `RBSGS-GettingStarted.adoc`
- `RBSPI-payor_install.adoc`
- `RBS0-SpecTop.adoc` (prose references to the file path)
- `CLAUDE.consumer.md`
- `rbhpr_refresh.sh` (handbook display text)
- `rblm_cli.sh` (marshal display text)

### Verification — governor reset cycle
After code changes, prove payor operations survive the migration by running a governor reset cycle:
1. `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime validation exercises RBDC paths)
2. `tt/rbw-aM.PayorMantlesGovernor.sh` — reset governor SA
3. `tt/rbw-aC.GovernorChartersRetriever.sh` — re-issue retriever RBRA
4. `tt/rbw-aK.GovernorKnightsDirector.sh` — re-issue director RBRA
5. Confirm all three RBRA files land in their role subdirectories and payor OAuth (`payor/rbro.env`) is still functional throughout

**[260415-1457] rough**

## Character
Mechanical refactoring with a migration shim — straightforward path surgery plus a tinder constant mint, verified end-to-end through governor reset proving payor operations survive the move.

## Docket
Move `rbro.env` from flat `RBRR_SECRETS_DIR/rbro.env` into `RBRR_SECRETS_DIR/payor/rbro.env` to match the established `{role}/` subdirectory pattern used by RBRA credentials (governor, retriever, director each have their own subdirectory).

### Mint tinder constant
Add `RBCC_role_payor="payor"` to `Tools/rbk/rbcc_Constants.sh` alongside the existing `RBCC_role_governor`, `RBCC_role_retriever`, `RBCC_role_director` tinder constants.

### Path change
In `Tools/rbk/rbdc_DerivedConstants.sh`, change:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
```
to:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
```

### One-shot migration
Add a migration block in `zrbdc_kindle()` (parallel to the existing RBRA migration at lines 40-56) that moves `RBRR_SECRETS_DIR/rbro.env` → `RBRR_SECRETS_DIR/payor/rbro.env` if the old path exists and the new path does not.

### Ensure payor directory creation
Add `${RBCC_role_payor}` to the `mkdir -p` block that already creates governor/retriever/director subdirectories.

### Doc/spec updates
Update literal `~/.rbw/rbro.env` references to `~/.rbw/payor/rbro.env` in:
- `RBSRO-RegimeOauth.adoc`
- `RBSGS-GettingStarted.adoc`
- `RBSPI-payor_install.adoc`
- `RBS0-SpecTop.adoc` (prose references to the file path)
- `CLAUDE.consumer.md`
- `rbhpr_refresh.sh` (handbook display text)
- `rblm_cli.sh` (marshal display text)

### Verification — governor reset cycle
After code changes, prove payor operations survive the migration by running a governor reset cycle:
1. `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime validation exercises RBDC paths)
2. `tt/rbw-aM.PayorMantlesGovernor.sh` — reset governor SA
3. `tt/rbw-aC.GovernorChartersRetriever.sh` — re-issue retriever RBRA
4. `tt/rbw-aK.GovernorKnightsDirector.sh` — re-issue director RBRA
5. Confirm all three RBRA files land in their role subdirectories and payor OAuth (`payor/rbro.env`) is still functional throughout

**[260415-1451] rough**

## Character
Mechanical refactoring with a migration shim — straightforward path surgery plus a tinder constant mint, verified by existing test suites.

## Docket
Move `rbro.env` from flat `RBRR_SECRETS_DIR/rbro.env` into `RBRR_SECRETS_DIR/payor/rbro.env` to match the established `{role}/` subdirectory pattern used by RBRA credentials (governor, retriever, director each have their own subdirectory).

### Mint tinder constant
Add `RBCC_role_payor="payor"` to `Tools/rbk/rbcc_Constants.sh` alongside the existing `RBCC_role_governor`, `RBCC_role_retriever`, `RBCC_role_director` tinder constants.

### Path change
In `Tools/rbk/rbdc_DerivedConstants.sh`, change:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_rbro_file}"
```
to:
```
readonly RBDC_PAYOR_RBRO_FILE="${RBRR_SECRETS_DIR}/${RBCC_role_payor}/${RBCC_rbro_file}"
```

### One-shot migration
Add a migration block in `zrbdc_kindle()` (parallel to the existing RBRA migration at lines 40-56) that moves `RBRR_SECRETS_DIR/rbro.env` → `RBRR_SECRETS_DIR/payor/rbro.env` if the old path exists and the new path does not.

### Ensure payor directory creation
Add `${RBCC_role_payor}` to the `mkdir -p` block that already creates governor/retriever/director subdirectories.

### Doc/spec updates
Update literal `~/.rbw/rbro.env` references to `~/.rbw/payor/rbro.env` in:
- `RBSRO-RegimeOauth.adoc`
- `RBSGS-GettingStarted.adoc`
- `RBSPI-payor_install.adoc`
- `RBS0-SpecTop.adoc` (prose references to the file path)
- `CLAUDE.consumer.md`
- `rbhpr_refresh.sh` (handbook display text)
- `rblm_cli.sh` (marshal display text)

### Verification
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (regime validation exercises RBDC paths)

### four-mode-case-split (₢A_AAT) [complete]

**[260423-1500] complete**

## Character

Mechanical refactor. Independent per-mode lifecycle cases — no inter-case state, no sequential ordering requirement. Each case is a self-contained observation of one delivery mode.

## Docket

`rbtdrc_four_mode_supply_chain` (Tools/rbk/rbtd/src/rbtdrc_crucible.rs:2217) is currently one `rbtdre_Case` with a 15-step sequential pipeline whose later steps depend on earlier steps' world-state (three hallmarks co-resident in GAR, kludge refs local). Decompose into four independent per-mode cases, each doing its own setup and teardown. Single-case reruns via `tt/rbtd-s.SingleCase.four-mode.sh <case>` become meaningful because every case stands alone; failures attribute to the mode under test; engine array order carries no load.

The four modes exercised are the three ordain modes (conjure, bind, graft — all dispatched via `RBTDRM_COLOPHON_ORDAIN`, vessel-mode-selected in `rbrv.env`) plus kludge (`RBTDRM_COLOPHON_KLUDGE`, local-only, never touches GAR). Each gets its own lifecycle case.

### Shape of each case

Every case is fully self-contained:

1. Invoke ordain (or kludge) on the relevant vessel directory
2. Read the hallmark and ark_stem facts from the BURV output
3. For GAR-backed modes (conjure/bind/graft): wrest the image locally, docker run, assert expected output, docker rmi
4. For kludge: docker inspect local image+vouch refs, docker run, assert expected output, docker rmi
5. For GAR-backed modes: abjure the hallmark (`--force`)
6. For kludge: no GAR teardown needed (no GAR footprint)

No locals cross case boundaries. No ctx struct fields added — all state is per-case local.

### Graft setup

Graft's vessel needs a local image ref to graft against. The current fixture uses the conjure hallmark. Decide during implementation:

- **Option A**: Graft's case conjures its own fresh base hallmark, grafts on it, abjures both at the end. Two abjures per graft run. Honest and standalone; doubles graft's Cloud Build cost.
- **Option B**: Graft uses a pre-existing upstream image as its base (e.g., docker pull busybox:latest fresh). No conjure dependency. Cheaper but shifts what the test exercises (graft-on-upstream vs graft-on-conjured).

Pick based on what `rbev-graft-demo`'s `BURE_TWEAK_NAME=threemodegraft` can realistically accept. If the tweak expects a GAR-formed ref, Option A is forced.

### Work items

- Extract four functions: `fn rbtdrc_fourmode_conjure_lifecycle(dir: &Path) -> rbtdre_Verdict`, same shape for `bind_lifecycle`, `graft_lifecycle`, `kludge_lifecycle`
- Replace monolithic `cases: &[case!(rbtdrc_four_mode_supply_chain)]` at `rbtdrc_crucible.rs:2457` with four `case!` entries — one per mode
- Each case: own `dir` for step markers; no shared locals; no ctx state additions
- Rename step-marker files from `step-NN-*` to per-case names (e.g., `conjure-ordain.txt`, `conjure-wrest.txt`, `conjure-run.txt`, `conjure-abjure.txt`)
- Delete `rbtdrc_four_mode_supply_chain` after extraction
- Drop tally-pre and tally-post cross-checks from this fixture — tally's enumeration is exercised live by other flows; a dedicated tally fixture is a separate pace if coverage is wanted
- Update the `///` doc comment to reflect the independent-case shape (replace the "15-step sequence" framing with "four independent per-mode lifecycle cases")

### Verification

- `tt/rbtd-b.Build.sh` clean (no cargo warnings beyond baseline)
- `tt/rbtd-t.Test.sh` — theurge unit tests green
- `tt/rbtd-s.TestSuite.fast.sh` — 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render)
- Each case individually runnable via `tt/rbtd-s.SingleCase.four-mode.sh <case-name>` against a charged crucible
- Full four-mode run (all four cases via array order; no dependency) produces green verdict surface
- Live crucible run deferred unless explicitly requested — no container runtime guaranteed in this officium

### Out of scope

- Adding new test calls (summon, plumb, rekon, jettison) — those remain under AAS
- Splitting other fixtures (tadmor-security, srjcl-jupyter, pluml-diagram) — separate judgment calls
- Engine-level changes to case model or execution — plural `cases` already exists and is used by tadmor/srjcl/pluml
- Creating a dedicated tally fixture to cover the dropped tally-pre/tally-post cross-checks — separate pace if wanted
- `rbtdri_Context` struct field additions — the independent shape makes ctx plumbing unnecessary

**[260423-1416] rough**

## Character

Mechanical refactor. Independent per-mode lifecycle cases — no inter-case state, no sequential ordering requirement. Each case is a self-contained observation of one delivery mode.

## Docket

`rbtdrc_four_mode_supply_chain` (Tools/rbk/rbtd/src/rbtdrc_crucible.rs:2217) is currently one `rbtdre_Case` with a 15-step sequential pipeline whose later steps depend on earlier steps' world-state (three hallmarks co-resident in GAR, kludge refs local). Decompose into four independent per-mode cases, each doing its own setup and teardown. Single-case reruns via `tt/rbtd-s.SingleCase.four-mode.sh <case>` become meaningful because every case stands alone; failures attribute to the mode under test; engine array order carries no load.

The four modes exercised are the three ordain modes (conjure, bind, graft — all dispatched via `RBTDRM_COLOPHON_ORDAIN`, vessel-mode-selected in `rbrv.env`) plus kludge (`RBTDRM_COLOPHON_KLUDGE`, local-only, never touches GAR). Each gets its own lifecycle case.

### Shape of each case

Every case is fully self-contained:

1. Invoke ordain (or kludge) on the relevant vessel directory
2. Read the hallmark and ark_stem facts from the BURV output
3. For GAR-backed modes (conjure/bind/graft): wrest the image locally, docker run, assert expected output, docker rmi
4. For kludge: docker inspect local image+vouch refs, docker run, assert expected output, docker rmi
5. For GAR-backed modes: abjure the hallmark (`--force`)
6. For kludge: no GAR teardown needed (no GAR footprint)

No locals cross case boundaries. No ctx struct fields added — all state is per-case local.

### Graft setup

Graft's vessel needs a local image ref to graft against. The current fixture uses the conjure hallmark. Decide during implementation:

- **Option A**: Graft's case conjures its own fresh base hallmark, grafts on it, abjures both at the end. Two abjures per graft run. Honest and standalone; doubles graft's Cloud Build cost.
- **Option B**: Graft uses a pre-existing upstream image as its base (e.g., docker pull busybox:latest fresh). No conjure dependency. Cheaper but shifts what the test exercises (graft-on-upstream vs graft-on-conjured).

Pick based on what `rbev-graft-demo`'s `BURE_TWEAK_NAME=threemodegraft` can realistically accept. If the tweak expects a GAR-formed ref, Option A is forced.

### Work items

- Extract four functions: `fn rbtdrc_fourmode_conjure_lifecycle(dir: &Path) -> rbtdre_Verdict`, same shape for `bind_lifecycle`, `graft_lifecycle`, `kludge_lifecycle`
- Replace monolithic `cases: &[case!(rbtdrc_four_mode_supply_chain)]` at `rbtdrc_crucible.rs:2457` with four `case!` entries — one per mode
- Each case: own `dir` for step markers; no shared locals; no ctx state additions
- Rename step-marker files from `step-NN-*` to per-case names (e.g., `conjure-ordain.txt`, `conjure-wrest.txt`, `conjure-run.txt`, `conjure-abjure.txt`)
- Delete `rbtdrc_four_mode_supply_chain` after extraction
- Drop tally-pre and tally-post cross-checks from this fixture — tally's enumeration is exercised live by other flows; a dedicated tally fixture is a separate pace if coverage is wanted
- Update the `///` doc comment to reflect the independent-case shape (replace the "15-step sequence" framing with "four independent per-mode lifecycle cases")

### Verification

- `tt/rbtd-b.Build.sh` clean (no cargo warnings beyond baseline)
- `tt/rbtd-t.Test.sh` — theurge unit tests green
- `tt/rbtd-s.TestSuite.fast.sh` — 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render)
- Each case individually runnable via `tt/rbtd-s.SingleCase.four-mode.sh <case-name>` against a charged crucible
- Full four-mode run (all four cases via array order; no dependency) produces green verdict surface
- Live crucible run deferred unless explicitly requested — no container runtime guaranteed in this officium

### Out of scope

- Adding new test calls (summon, plumb, rekon, jettison) — those remain under AAS
- Splitting other fixtures (tadmor-security, srjcl-jupyter, pluml-diagram) — separate judgment calls
- Engine-level changes to case model or execution — plural `cases` already exists and is used by tadmor/srjcl/pluml
- Creating a dedicated tally fixture to cover the dropped tally-pre/tally-post cross-checks — separate pace if wanted
- `rbtdri_Context` struct field additions — the independent shape makes ctx plumbing unnecessary

**[260423-1404] rough**

## Character

Mechanical refactor. Single-fixture surgery with clear pre-condition (current monolithic fixture green) and post-condition (same behavior, decomposed into individually runnable cases). Each step's body already exists — the work is extraction and state-flow plumbing.

## Docket

`rbtdrc_four_mode_supply_chain` is currently one `rbtdre_Case` with a 15-step sequential pipeline and a single pass/fail verdict. The engine's `rbtdre_Section` already supports multi-case sections (`cases: &[…]` is plural) — four-mode just doesn't use it. Decompose so single-case reruns via `tt/rbtd-s.SingleCase.four-mode.sh <name>` actually target meaningful sub-units, failures attribute precisely ("bind failed" vs. the current "four-mode step 2"), and future fixture additions land as new cases rather than step insertions into an ever-lengthening monolith.

### Work items

- Extract each step (or cohesive step group) from `rbtdrc_four_mode_supply_chain` into its own `fn rbtdrc_fourmode_<name>(dir: &Path) -> rbtdre_Verdict`
- Replace the monolithic `cases: &[case!(rbtdrc_four_mode_supply_chain)]` in `RBTDRC_SECTIONS_FOUR_MODE` with the decomposed array; engine runs cases in array order, so sequential dependencies still hold for full-fixture runs
- Resolve state handoff: scratch `dir: &Path` is per-case (isolated); GAR state persists naturally across calls; any in-memory handoff (image digests, etc.) flows through the shared `ctx` — add fields to the ctx struct if needed
- Rename step-marker files from `step-NN-…` to per-case names so single-case reruns produce clean trace output
- Update the `///` doc comment from "15-step sequence" to the case-decomposed description

### Case decomposition (proposal — refine during work)

1. `conjure` — ordain conjure busybox
2. `bind` — ordain bind plantuml
3. `graft_setup` — wrest for graft (if separable)
4. `graft` — ordain graft busybox
5. `kludge` — local kludge busybox
6. `tally_pre` — tally expect three vouched
7. `retrieve` — wrest conjured busybox
8. `run` — bark the retrieved image
9. `cleanup` — rmi after run
10. `abjure_conjure`, `abjure_bind`, `abjure_graft` — three abjure calls
11. `tally_post` — tally expect empty

Consolidate where a step alone is too thin (e.g., `graft_setup` might merge with `graft`).

### Verification

- Each case runnable individually via `tt/rbtd-s.SingleCase.four-mode.sh <case>` against a charged crucible
- Full four-mode run (all cases sequentially) produces the same green verdict surface as pre-split
- bash -n and cargo build clean; fast suite 90/90 green (no regression in regime-validation)

### Out of scope

- Adding new test calls (summon, plumb, rekon, jettison) — those land under AAS
- Splitting other fixtures (tadmor-security, srjcl-jupyter, pluml-diagram) — separate judgment calls
- Engine-level changes to case model or execution — plural `cases` already exists
- Converting sequential dependency into parallel-safe cases — current shape is sequential by necessity; preserve

### theurge-coverage-catchup-aak-aal-aao (₢A_AAS) [complete]

**[260426-0905] complete**

## Character

Resume in a clean chat. Today's work landed five notches under AAS plus a repo-sync sidebar; the original docket scope is **substantially complete** except for a live verification gap on the batch_vouch fixture and stale-hallmark cleanup. The remount should orient on the gap, not redo verification.

## Docket

### Notches landed under AAS (today, after rebase — SHAs are post-rebase)

- `e53c7268` (saddle, was `c9883c06`)
- `ce6e54c0` OAuth exchange diagnostics in zrbgo_exchange_jwt_capture (was `7050a06f`)
- `76f20e9b` graft layer-DiffID round-trip via rbtdrc_docker_layers_capture, replacing the run-with-stdout assertion that was unreachable against busybox:latest's default sh CMD (was `c97804d8`)
- `2d8986d2` fast-suite RBRR_CLOUD_PREFIX/RUNTIME_PREFIX negatives × 3 modes each (uppercase / no-trailing-hyphen / too-long), plus rs_rbrr_nonempty_prefix smoke; RCG constization of var names (was `7375f110`)

### Verified GREEN (today)

- conjure_lifecycle SingleCase (HTTP 400 + plumb multi-layer fixes confirmed end-to-end through the wrapper)
- bind_lifecycle SingleCase
- graft_lifecycle SingleCase (layer-DiffID — empirical confirmation that graft preserves RootFS layer DiffIDs while normalizing the manifest envelope multi-arch index → single-platform manifest)
- kludge_lifecycle SingleCase
- Fast suite: enrollment-validation 47/47, regime-validation **27/27** (was 21 — added 6 new prefix negatives), regime-smoke **8/8** (was 7 — rs_rbrr_nonempty_prefix counted), handbook-render 15/15

### Original AAS docket items — completion status

1. **Fast-suite negatives + smoke** — DONE (notch `2d8986d2`)
2. **Four-mode new cases** (summon, plumb_compact, jettison, plumb_full, rekon) — DONE prior to this chat; all five integrated into the existing per-mode lifecycles per AAT shape (kept embedded rather than extracted; valid per docket's "Decide: keep or extract" clause)
3. **Batch-vouch standalone fixture** — fixture IMPLEMENTED prior to this chat (`rbtdrc_batch_vouch_lifecycle` + `RBTDRC_SECTIONS_BATCH_VOUCH` + `tt/rbtd-{r,s}.{Run,SingleCase}.batch-vouch.sh` + manifest registration). Live verification **FAILED today** — see Verification gap below.
4. **Manifest updates** — DONE prior to this chat (RBTDRM_COLOPHON_SUMMON / PLUMB_FULL / PLUMB_COMPACT / VOUCH all present; REKON + JETTISON registered; RBTDRM_FIXTURE_BATCH_VOUCH declared with required colophons)

### Verification gap — batch_vouch live failure

Live `tt/rbtd-s.SingleCase.batch-vouch.sh rbtdrc_batch_vouch_lifecycle` ran today with these outcomes:

- Step 01 ordain conjure on rbev-busybox → GREEN, hallmark `c260426074346-r260426144348` minted
- Step 02 jettison vouch ark via `rbw-iJ <category>/<hallmark>/vouch:<hallmark> --force` → exit 0
- Step 03 tally pending check → **FAIL**: hallmark still shows `vouched` health with all six basenames (`about attest diags image pouch vouch`)

The vouch ark was **not actually removed** despite jettison reporting success. Test bailed before abjure (steps 04-06 unreached). Trace + invoke transcripts at `/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-54491/` (path is per-machine; expect garbage collection after reboot — recover by re-running the case for fresh traces).

### Hypotheses to investigate at remount

1. **Locator-grammar mismatch (low likelihood)** — bind_lifecycle's jettison uses the same shape and successfully removes the pouch ark. Grammar isn't broken across the board. But vouch-specific path could differ.
2. **Vouch-specific guard in rbfl_jettison** — vouch arks may be protected (digest-pinned? attestation-aggregator preserved?). Read `rbfl_jettison` and look for vouch-specific branches.
3. **Re-creation path** — some downstream operation may auto-recreate the vouch ark (vouch-on-write? lazy regeneration?). Read jettison's full path through `zrbfl_jettison_*` for any recreate-after-delete.
4. **Eventual consistency** — the tally ran almost immediately after jettison; possible GAR cache lag. Counter-evidence: tally elsewhere reflects deletions promptly.

Read `Tools/rbk/rbfl_FoundryLedger.sh` (jettison implementation) and `Tools/rbk/rbfv_FoundryVerify.sh` (batch_vouch / vouch-creation implementation) before any code change. Check `Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc` for the documented contract on vouch jettison.

### Cleanup pending

- Hallmark `c260426074346-r260426144348` (rbev-busybox conjure, minted today) is live in GAR. Abjure manually before pace close: `tt/rbw-fA.DirectorAbjuresHallmark.sh c260426074346-r260426144348 --force`. Cheap.

### Repo-sync sidebar (informational; not pace-blocking)

Today this chat also reconfigured beta's `origin` from filesystem-pointing-at-alpha to `git@github.com:bhyslop/recipemuster.git` directly, with backup branches (`backup/main-prerebase-260426`, `backup/bth-heat-BA-prerebase-260426`, `backup/bth-heat-BA-rebased-260426`) on GitHub for safety. Beta's main + rebased bth-heat-BA published. Alpha repo not touched — needs separate alignment in its own session (a self-contained alert prompt was generated for the alpha-side Claude). All this happened around resolving a saddle-commit gallops conflict in jjg_gallops.json — phantom IDs (AAM/AAV/AAW/AAX) in heat A_'s order array were dropped; legitimate `AAa` preserved. AAS pace itself isn't affected by the sync.

### Cadence (suggested for next chat)

1. Mount ₣A_, parade ₢A_AAS, read this docket
2. Abjure the stale hallmark `c260426074346-r260426144348`
3. Read `rbfl_FoundryLedger.sh` jettison path with vouch-specific guard hypothesis in mind
4. Read `rbfv_FoundryVerify.sh` for any vouch re-creation path
5. Re-run `tt/rbtd-s.SingleCase.batch-vouch.sh rbtdrc_fourmode_batch_vouch_lifecycle` (correct case name: `rbtdrc_batch_vouch_lifecycle`) with the failure narrowed by reading first
6. Once batch_vouch is GREEN, the AAS pace is ready to wrap

### Verification gates (for pace close)

- conjure_lifecycle GREEN — DONE
- bind/graft/kludge cases GREEN — DONE
- Fast suite 90 → 93 cases GREEN — DONE (now 97 actually: 47 + 27 + 8 + 15 = 97; the +6 negatives bumped regime-validation from 21 to 27)
- Crucible suite: four-mode GREEN; batch_vouch fixture GREEN — **batch_vouch OPEN**
- Optional: complete suite GREEN

### Out-of-scope itches surfaced today (flag, do not fix here)

(All from prior chats, still open) plus:

- **Vouch-jettison silent no-op** — if investigation reveals this is a real defect (not a guard or re-creation path), flag for a separate pace under AAP. The fixture's failure mode is the diagnostic.

**[260426-0814] rough**

## Character

Resume in a clean chat. Today's work landed five notches under AAS plus a repo-sync sidebar; the original docket scope is **substantially complete** except for a live verification gap on the batch_vouch fixture and stale-hallmark cleanup. The remount should orient on the gap, not redo verification.

## Docket

### Notches landed under AAS (today, after rebase — SHAs are post-rebase)

- `e53c7268` (saddle, was `c9883c06`)
- `ce6e54c0` OAuth exchange diagnostics in zrbgo_exchange_jwt_capture (was `7050a06f`)
- `76f20e9b` graft layer-DiffID round-trip via rbtdrc_docker_layers_capture, replacing the run-with-stdout assertion that was unreachable against busybox:latest's default sh CMD (was `c97804d8`)
- `2d8986d2` fast-suite RBRR_CLOUD_PREFIX/RUNTIME_PREFIX negatives × 3 modes each (uppercase / no-trailing-hyphen / too-long), plus rs_rbrr_nonempty_prefix smoke; RCG constization of var names (was `7375f110`)

### Verified GREEN (today)

- conjure_lifecycle SingleCase (HTTP 400 + plumb multi-layer fixes confirmed end-to-end through the wrapper)
- bind_lifecycle SingleCase
- graft_lifecycle SingleCase (layer-DiffID — empirical confirmation that graft preserves RootFS layer DiffIDs while normalizing the manifest envelope multi-arch index → single-platform manifest)
- kludge_lifecycle SingleCase
- Fast suite: enrollment-validation 47/47, regime-validation **27/27** (was 21 — added 6 new prefix negatives), regime-smoke **8/8** (was 7 — rs_rbrr_nonempty_prefix counted), handbook-render 15/15

### Original AAS docket items — completion status

1. **Fast-suite negatives + smoke** — DONE (notch `2d8986d2`)
2. **Four-mode new cases** (summon, plumb_compact, jettison, plumb_full, rekon) — DONE prior to this chat; all five integrated into the existing per-mode lifecycles per AAT shape (kept embedded rather than extracted; valid per docket's "Decide: keep or extract" clause)
3. **Batch-vouch standalone fixture** — fixture IMPLEMENTED prior to this chat (`rbtdrc_batch_vouch_lifecycle` + `RBTDRC_SECTIONS_BATCH_VOUCH` + `tt/rbtd-{r,s}.{Run,SingleCase}.batch-vouch.sh` + manifest registration). Live verification **FAILED today** — see Verification gap below.
4. **Manifest updates** — DONE prior to this chat (RBTDRM_COLOPHON_SUMMON / PLUMB_FULL / PLUMB_COMPACT / VOUCH all present; REKON + JETTISON registered; RBTDRM_FIXTURE_BATCH_VOUCH declared with required colophons)

### Verification gap — batch_vouch live failure

Live `tt/rbtd-s.SingleCase.batch-vouch.sh rbtdrc_batch_vouch_lifecycle` ran today with these outcomes:

- Step 01 ordain conjure on rbev-busybox → GREEN, hallmark `c260426074346-r260426144348` minted
- Step 02 jettison vouch ark via `rbw-iJ <category>/<hallmark>/vouch:<hallmark> --force` → exit 0
- Step 03 tally pending check → **FAIL**: hallmark still shows `vouched` health with all six basenames (`about attest diags image pouch vouch`)

The vouch ark was **not actually removed** despite jettison reporting success. Test bailed before abjure (steps 04-06 unreached). Trace + invoke transcripts at `/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-54491/` (path is per-machine; expect garbage collection after reboot — recover by re-running the case for fresh traces).

### Hypotheses to investigate at remount

1. **Locator-grammar mismatch (low likelihood)** — bind_lifecycle's jettison uses the same shape and successfully removes the pouch ark. Grammar isn't broken across the board. But vouch-specific path could differ.
2. **Vouch-specific guard in rbfl_jettison** — vouch arks may be protected (digest-pinned? attestation-aggregator preserved?). Read `rbfl_jettison` and look for vouch-specific branches.
3. **Re-creation path** — some downstream operation may auto-recreate the vouch ark (vouch-on-write? lazy regeneration?). Read jettison's full path through `zrbfl_jettison_*` for any recreate-after-delete.
4. **Eventual consistency** — the tally ran almost immediately after jettison; possible GAR cache lag. Counter-evidence: tally elsewhere reflects deletions promptly.

Read `Tools/rbk/rbfl_FoundryLedger.sh` (jettison implementation) and `Tools/rbk/rbfv_FoundryVerify.sh` (batch_vouch / vouch-creation implementation) before any code change. Check `Tools/rbk/vov_veiled/RBSIJ-image_jettison.adoc` for the documented contract on vouch jettison.

### Cleanup pending

- Hallmark `c260426074346-r260426144348` (rbev-busybox conjure, minted today) is live in GAR. Abjure manually before pace close: `tt/rbw-fA.DirectorAbjuresHallmark.sh c260426074346-r260426144348 --force`. Cheap.

### Repo-sync sidebar (informational; not pace-blocking)

Today this chat also reconfigured beta's `origin` from filesystem-pointing-at-alpha to `git@github.com:bhyslop/recipemuster.git` directly, with backup branches (`backup/main-prerebase-260426`, `backup/bth-heat-BA-prerebase-260426`, `backup/bth-heat-BA-rebased-260426`) on GitHub for safety. Beta's main + rebased bth-heat-BA published. Alpha repo not touched — needs separate alignment in its own session (a self-contained alert prompt was generated for the alpha-side Claude). All this happened around resolving a saddle-commit gallops conflict in jjg_gallops.json — phantom IDs (AAM/AAV/AAW/AAX) in heat A_'s order array were dropped; legitimate `AAa` preserved. AAS pace itself isn't affected by the sync.

### Cadence (suggested for next chat)

1. Mount ₣A_, parade ₢A_AAS, read this docket
2. Abjure the stale hallmark `c260426074346-r260426144348`
3. Read `rbfl_FoundryLedger.sh` jettison path with vouch-specific guard hypothesis in mind
4. Read `rbfv_FoundryVerify.sh` for any vouch re-creation path
5. Re-run `tt/rbtd-s.SingleCase.batch-vouch.sh rbtdrc_fourmode_batch_vouch_lifecycle` (correct case name: `rbtdrc_batch_vouch_lifecycle`) with the failure narrowed by reading first
6. Once batch_vouch is GREEN, the AAS pace is ready to wrap

### Verification gates (for pace close)

- conjure_lifecycle GREEN — DONE
- bind/graft/kludge cases GREEN — DONE
- Fast suite 90 → 93 cases GREEN — DONE (now 97 actually: 47 + 27 + 8 + 15 = 97; the +6 negatives bumped regime-validation from 21 to 27)
- Crucible suite: four-mode GREEN; batch_vouch fixture GREEN — **batch_vouch OPEN**
- Optional: complete suite GREEN

### Out-of-scope itches surfaced today (flag, do not fix here)

(All from prior chats, still open) plus:

- **Vouch-jettison silent no-op** — if investigation reveals this is a real defect (not a guard or re-creation path), flag for a separate pace under AAP. The fixture's failure mode is the diagnostic.

**[260425-1009] rough**

## Character

Verification-driven completion in a clean chat. Today's session pivoted from the original additive-coverage scope into AAK-completion fixing — six notches landed, two of those today (HTTP 400 + plumb multi-layer) verified GREEN against live infrastructure but the full SingleCase lifecycle isn't end-to-end clean yet. Pick up at "verify what's already fixed before extending coverage." Mechanical until the original docket items are reached, then back to the additive shape.

## Docket

This pace has accreted six notches under ₢A_AAS:

- `b602b7c5` AAK preflight + inscribe submitter (zrbfd_preflight_reliquary, zrbfl_inscribe_submit)
- `14242011` enshrine sweep across conjure vessels (busybox, rust:slim-bookworm, debian:bookworm-slim)
- `160ceec7` yoke remaining 6 conjure vessels to reliquary r260425082412
- `73f3f818` bind AAK-shape (rbgjm01-mirror-image.sh) + graft platform-normalization (rbgja01 _normalize_variant)
- `98b80250` conjure HTTP 400 (rbfd:541 dead-substitution stripping `${_RBGA_HALLMARK}` while line 679 still declares it under automapSubstitutions=true) + SingleCase non-crucible context (main.rs run_single moved ctx creation outside the needs_charge gate)
- `23a93bb0` plumb multi-layer extraction (rbfc:zrbfc_gar_extract_artifact — buildkit FROM-scratch images emit one layer per COPY, not a single accumulated layer)

### Verified GREEN (today)

1. **Conjure HTTP 400 fix** — live `tt/rbw-fO` against rbev-busybox produced end-to-end conjure (image+about+vouch builds, hallmark `c260425094751-r260425164754` in GAR) with no HTTP 400 substitution-mismatch. Build IDs for forensics: `6893dd5c-86a1-4304-bee9-025bebcd633b` (image+about), `3b9ccf2b-4877-4d10-a2ff-9065b52e5062` (vouch).
2. **Plumb multi-layer extraction** — live `tt/rbw-fpf c260425094751-r260425164754` emits all sections (Vessel Type / Source / Builder / SLSA / Base Image / Build Output / Build Cache Delta / SBOM / Package Inventory / Package Licensing / Recipe). All three SingleCase markers present: SLSA, SBOM, Dockerfile.

### Verification gap

The full lifecycle SingleCase failed at plumb_full (now fixed) before reaching:

- **06-rekon** — `rbw-ir` against the conjured hallmark, expects image/about/vouch/attest basenames marked yes
- **07-abjure** — `rbw-fA --force` against the conjured hallmark

The hallmark `c260425094751-r260425164754` remains live in GAR (abjure didn't run). Two paths to close:

- **(A)** Re-run `tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_conjure_lifecycle` — ~10-12 min, full lifecycle from fresh ordain. Recommended — also re-confirms HTTP 400 + plumb fixes through the case wrapper, not just the underlying tabtargets.
- **(B)** Run `tt/rbw-ir <hallmark>` + `tt/rbw-fA --force <hallmark>` manually against the live hallmark — ~1 min, but doesn't exercise the SingleCase wrapper or rbgja04→plumb continuity.

Then re-run bind + graft + kludge SingleCases (or full `tt/rbtd-r.Run.four-mode.sh`) to confirm the 73f3f818 bind/graft fixes still hold.

### Original AAS docket scope (unstarted — diverted by today's AAK-completion work)

None of these were touched today. Listed in declared dependency order:

1. **Fast-suite negatives + smoke** (`rbtdrf_fast.rs`):
   - `rv_rbrr_bad_cloud_prefix` — invalid RBRR_CLOUD_PREFIX formats (uppercase, trailing non-hyphen, >11 chars)
   - `rv_rbrr_bad_runtime_prefix` — same on RBRR_RUNTIME_PREFIX
   - `rs_rbrr_nonempty_prefix` — smoke with both prefixes populated, confirm kindle + derived-constant propagation

2. **Four-mode new cases** (decomposed `RBTDRC_SECTIONS_FOUR_MODE.cases` per AAT):
   - `summon` — `rbw-fs` against conjured busybox; verify AAL single-arg signature; image appears locally. Place after `tally_pre`.
   - `plumb_full` — **already implicitly added** in the conjure_lifecycle case (markers SLSA / SBOM / Dockerfile). Decide: keep there, or extract as standalone case in the decomposed array.
   - `plumb_compact` — `rbw-fpc` against bind plantuml hallmark; verify compact output shape.
   - `rekon` — already implicitly added in conjure_lifecycle. Decide: extract or leave.
   - `jettison` — `rbw-iJ` on a single ark of the bind plantuml hallmark; expect tally to show `incomplete` before abjure clears it. Place between `cleanup` and `abjure_bind`.

3. **Batch-vouch standalone fixture** (`rbtdrc_batch_vouch` — new section):
   - `plant_pending` — ordain conjure, then jettison vouch package surgically (relies on AAR jettison grammar)
   - `tally_expect_pending` — confirm pending classification + `rbw-fV` recommendation
   - `batch_vouch` — invoke `rbw-fV`
   - `tally_expect_vouched` — confirm transition to vouched
   - `cleanup` — abjure
   - Fallback: focused unit test of `rbfv_batch_vouch`'s two-pass state machine against mock `zrbfc_list_hallmarks_capture` output.

4. **Manifest updates** (`rbtdrm_manifest.rs`):
   - Add `RBTDRM_COLOPHON_SUMMON` / `PLUMB_FULL` / `PLUMB_COMPACT` / `BATCH_VOUCH` if absent
   - Verify `REKON` + `JETTISON` constants already registered

5. **Yoke** — deferred to AAP's live burn-in scope. No code in this pace.

### Out-of-scope itches surfaced today (flag, do not fix here)

1. **Pre-existing `2>/dev/null` at `rbfc_FoundryCore.sh:619`** — `jq -r '.mediaType // empty' ... 2>/dev/null || true` violates BCG "Stderr Capture — Never Suppress." Out of scope for the multi-layer fix; candidate for a BCG sweep pace.
2. **Graft test-expectation defect** (`rbtdrc_crucible.rs:2585`) — asserts "BusyBox container is running!" from upstream busybox:latest; only `rbev-busybox/Dockerfile:15` CMD produces it. Vessel-tier issue. Flagged in 73f3f818 for AAP.
3. **rbfl_yoke mode-gate** — `rbfd:1325` and `rbfd:1576` prove bind/graft consume the reliquary, but `rbfl_yoke` refuses non-conjure modes. Flagged for AAP.
4. **rbhodf inscribe handbook framing** (`rbhodf_director_first_build.sh:123-154`) — positive-path-only, oversells reliquary durability ("stays in the depot until you choose to refresh it"), frames yoke as single-vessel. Flagged for AAP.
5. **Plumb data-field gaps** — live plumb output shows `?` for Moniker / Build time / Image URI / Vouch URL / Vouch SHA256. Upstream data not being written into build_info.json or vouch_summary.json — separate from extraction. Itch, not this pace.

### Cadence (suggested for next chat)

1. Mount ₣A_, parade ₢A_AAS, read this docket
2. Close the verification gap (path A recommended)
3. Re-run bind + graft + kludge SingleCases
4. Address original docket items 1 → 2 → 3 → 4 in order
5. Fast suite GREEN, crucible suite GREEN, optional complete suite GREEN at pace close
6. Wrap

### Verification gates (for pace close)

- `tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_conjure_lifecycle` GREEN end-to-end
- Bind + graft + kludge cases GREEN (or full `tt/rbtd-r.Run.four-mode.sh` GREEN)
- Fast suite: 90 → 93 cases GREEN
- Crucible suite: four-mode + new `batch_vouch` fixture GREEN
- Optional: `tt/rbtd-s.TestSuite.complete.sh` GREEN

**[260423-1405] rough**

## Character

Additive test-code catchup. Each work item is a short, mechanical extension — new fast-suite cases, new four-mode cases for recently rewritten functions, a new standalone fixture for batch_vouch. Code all, run suites, notch once; second notch only on test-driven fix.

## Docket

Today's A_/A6 sweep landed runtime-behavior changes with partial theurge coverage:

- AAK categorical-layout migration — happy path covered by four-mode; rekon, jettison, batch-vouch, summon, plumb, yoke surfaces untested
- AAL vessel-param drop — abjure + tally signatures exercised; summon (`rbw-fs`) and plumb (`rbw-fpf`/`rbw-fpc`) not
- AAO tally + batch_vouch rewrites — tally exercised pre- and post-abjure; new two-pass `rbfv_batch_vouch` pending→vouched path untested
- AAA/AAB resource prefixing — enrollment covered at empty prefix; non-empty-prefix propagation and bad-format negatives absent
- AAQ/AAR — rekon + jettison rewrites landed but no fixture exercises them

**Pre-requisite**: ₢A_AAT (four-mode-case-split) lands first. New four-mode work here adds cases to the decomposed section rather than inserting steps into a monolith.

### Work items

1. **Fast-suite negatives + smoke** (`rbtdrf_fast.rs`):
   - `rv_rbrr_bad_cloud_prefix` — invalid RBRR_CLOUD_PREFIX formats (uppercase, trailing non-hyphen, >11 chars)
   - `rv_rbrr_bad_runtime_prefix` — same on RBRR_RUNTIME_PREFIX
   - `rs_rbrr_nonempty_prefix` — smoke with both prefixes populated (e.g., `acme-`), confirm kindle + derived-constant propagation

2. **Four-mode new cases** (appended to the decomposed `RBTDRC_SECTIONS_FOUR_MODE.cases` array from AAT):
   - `summon` — invoke `rbw-fs` against the conjured busybox hallmark; verify AAL single-arg signature; image appears in local daemon. Place after `tally_pre`.
   - `plumb_full` — invoke `rbw-fpf` against the same hallmark; verify SBOM + build_info + Dockerfile sections present.
   - `plumb_compact` — invoke `rbw-fpc` against the bind plantuml hallmark; verify compact output shape.
   - `rekon` — invoke `rbw-ir` against the conjured busybox hallmark; verify basename listing includes image + about + vouch. Place after `tally_pre` (any time before that hallmark's abjure).
   - `jettison` — invoke `rbw-iJ` on a single ark of the bind plantuml hallmark (pouch or about); verify surgical delete succeeds; expect a subsequent `tally` case to show that hallmark degraded to `incomplete` before the abjure case clears it. Place between `cleanup` and `abjure_bind`.

3. **Batch-vouch standalone fixture** (`rbtdrc_batch_vouch` — new section):
   - `plant_pending` — ordain conjure, then jettison the vouch package surgically (relies on post-AAR jettison grammar), leaving image+about
   - `tally_expect_pending` — confirm `pending` classification + `rbw-fV` recommendation emitted
   - `batch_vouch` — invoke `rbw-fV`
   - `tally_expect_vouched` — confirm transition to `vouched`
   - `cleanup` — abjure

   Fallback if plant-via-jettison is entangled: focused unit test of `rbfv_batch_vouch`'s two-pass state machine against mock `zrbfc_list_hallmarks_capture` output. Less integration value but decoupled.

4. **Manifest updates** (`rbtdrm_manifest.rs`):
   - Add RBTDRM_COLOPHON_SUMMON / PLUMB_FULL / PLUMB_COMPACT / BATCH_VOUCH if absent
   - Verify REKON + JETTISON constants already registered

5. **Yoke** — deferred to AAP's live burn-in scope (reliquary-inscribe is there). No code in this pace.

### Cadence

Code all four touchpoints, run fast suite → crucible suite, notch once when green. Second notch only if test feedback requires a substantive fix.

### Verification

- Fast suite: 90 → 93 cases green (2 negatives + 1 smoke)
- Crucible suite: four-mode green with new cases each individually runnable; new batch_vouch fixture green
- Full `tt/rbtd-s.TestSuite.complete.sh` green at pace close (or deferred if container runtime unavailable in the working officium)

### Out of scope

- Four-mode decomposition (landed under AAT)
- New test infrastructure beyond additive extensions
- Handbook prose churn coverage (already covered by `rbtdrf_hb_render` exit-0 assertion)
- Service-tier coverage (access-probe unchanged)
- Yoke code (handshaken into AAP)

### Affiliated commits today

- ₢A_AAK N1–N4: GAR categorical layout
- ₢A_AAL: vessel-param drop
- ₢A_AAO: tally + batch_vouch rewrites
- ₢A_AAA/AAB/AAC: RBRR_CLOUD_PREFIX / RUNTIME_PREFIX
- ₢A6AA6: rbfl_yoke / rbw-dY (deferred to AAP)
- ₢A_AAQ: rekon rewrite
- ₢A_AAR: jettison rewrite

**[260423-1350] rough**

## Character

Intricate but mechanical. Each notch is additive — new fixtures or new calls inserted into existing fixtures — with clear pre-conditions and verification shape. The hardest judgment is notch 3's fixture design (how to plant a `pending` hallmark cheaply); everything else is routine extension.

## Docket

Today's A_/A6 sweep landed runtime-behavior changes whose theurge coverage is partial:

- AAK categorical-layout migration — four-mode fixture covers the happy path (conjure/bind/graft/kludge/tally/abjure), but rekon, jettison, batch-vouch, summon, plumb, and yoke surfaces are untested
- AAL vessel-param drop — abjure and tally signature changes are exercised, but summon (`rbw-fs`) and plumb (`rbw-fpf`/`rbw-fpc`) are not
- AAO tally + batch_vouch rewrites — tally is exercised pre- and post-abjure, but the new two-pass `rbfv_batch_vouch` pending→vouched path is never walked (four-mode's ordain already emits vouched state)
- AAA/AAB resource prefixing — enrollment is covered by regime-validation/smoke at empty prefix, but non-empty-prefix propagation and bad-format negatives are absent

Land the catchup in five notches, ordered fast-suite-first, then independent crucible work, then AAQ/AAR-gated crucible work, with yoke deferred into AAP's scope.

### Notch 1 — Regime-validation negatives + non-empty-prefix smoke

Fast suite, zero dependencies. Mint two negative cases under `rbtdrf_fast.rs` following the existing `rv_rbrr_*` shape:

- `rv_rbrr_bad_cloud_prefix` — RBRR_CLOUD_PREFIX with invalid format (uppercase, trailing non-hyphen, >11 chars)
- `rv_rbrr_bad_runtime_prefix` — same negatives on RBRR_RUNTIME_PREFIX

Add one positive smoke: `rs_rbrr_nonempty_prefix` — render a regime with both prefixes populated (e.g., `acme-`), confirm kindle succeeds and the prefixes appear in derived constants where they should.

### Notch 2 — Four-mode weave: summon + plumb (AAL signature coverage)

Independent of AAQ/AAR. In `rbtdrc_four_mode_supply_chain` (`rbtdrc_crucible.rs:2217`), after step 9 (cleanup) and before step 10 (abjure loop), insert:

- `rbw-fs` summon against the conjured busybox hallmark — verify single-arg signature works, image appears in local daemon
- `rbw-fpf` plumb-full against the same hallmark — verify single-arg signature, output contains SBOM + build_info + Dockerfile sections
- `rbw-fpc` plumb-compact against the bind-mode plantuml hallmark — verify single-arg signature, output is the compact shape

These cover the AAL signature drops and implicitly exercise `rbfc_vessel_for_hallmark_capture` (consumed only by plumb). Add RBTDRM_COLOPHON_SUMMON / PLUMB_FULL / PLUMB_COMPACT to `rbtdrm_manifest.rs` if absent.

### Notch 3 — `rbfv_batch_vouch` standalone fixture

New crucible fixture `rbtdrc_batch_vouch`. Cannot cleanly fold into four-mode because four-mode's ordain already vouches. Shape:

1. Plant a hallmark in `pending` state (image + about packages present, no vouch package). Cheapest path: invoke `rbw-fO` for conjure mode, then surgically jettison the vouch package via `rbw-iJ`, leaving image+about. (This also exercises post-AAR jettison grammar — coordinate verification.)
2. Tally — confirm `pending` classification, confirm `rbw-fV` recommendation emitted
3. Invoke `rbw-fV` batch-vouch
4. Tally — confirm the hallmark transitioned to `vouched`
5. Cleanup via abjure

Add RBTDRM_COLOPHON_BATCH_VOUCH to the manifest. If the plant-via-jettison path is too entangled, alternative: a focused unit test of `rbfv_batch_vouch`'s two-pass state machine against a mock `zrbfc_list_hallmarks_capture` output — less integration value but decoupled.

### Notch 4 — Four-mode weave: rekon + jettison (gated on AAQ, AAR)

Must land after AAQ and AAR rewrite `rbfl_rekon` and `rbfl_jettison` to the new GAR grammar. In `rbtdrc_four_mode_supply_chain`:

- After step 5 tally, invoke `rbw-ir` rekon against the conjured busybox hallmark — verify basename listing includes image + about + vouch
- Between step 9 (cleanup) and step 10 (abjure loop), invoke `rbw-iJ` jettison against one ark of the bind-mode plantuml hallmark (e.g., the pouch package if present, else about) — verify surgical delete succeeds, tally shows plantuml degraded to `incomplete`, then let the abjure loop clear it

RBTDRM_COLOPHON_REKON and JETTISON constants already exist. Verify their tabtarget shims are registered in the manifest.

This notch retroactively validates AAQ and AAR — their dockets' Verification sections note the four-mode-fixture gap without committing to fill it; this closes that loop.

### Notch 5 — Yoke coordination with AAP (deferred marker)

`rbfl_yoke` / `rbw-dY` need a produced reliquary to test against. Reliquary-Inscribe (`rbw-dI`) has no theurge coverage and is in AAP's live-burn-in scope. Coordinate with AAP to add:

- Reliquary inscribe → yoke-stamp-into-conjure-vessel → ordain → tally happy path
- Yoke negative: missing-vessel, missing-reliquary, wrong-mode vessel (bind/graft refused)

Notch 5 is a handshake marker: the actual code lands under AAP. If AAP elects to scope yoke out, re-slate notch 5 under this pace with a canned-reliquary stub path.

## Verification

- Notch 1: fast suite green with 2 new negatives + 1 new smoke (90 → 93 cases, roughly)
- Notch 2: four-mode fixture green with summon+plumb insertions
- Notch 3: new batch_vouch fixture green in crucible suite
- Notch 4: four-mode fixture green with rekon+jettison insertions (post-AAQ/AAR)
- Notch 5: handshake documented; code under AAP

Full `tt/rbtd-s.TestSuite.complete.sh` green at pace close (or deferred to AAP's live burn-in if container runtime unavailable in this officium).

## Out of scope

- New test infrastructure (fixtures manager, shared fakes) — additive only, follow existing patterns
- Rewriting the four-mode fixture's overall shape — insertions only, no restructure
- Coverage of onboarding handbook prose churn — `rbtdrf_hb_render` already covers via exit-0 assertion
- Service-tier coverage (access-probe variants) — unchanged by today's work

## Affiliated commits today

- ₢A_AAK notches 1–4: GAR categorical layout
- ₢A_AAL: vessel-param drop (abjure, summon, plumb signatures)
- ₢A_AAO: tally + batch_vouch rewrites
- ₢A_AAA, ₢A_AAB, ₢A_AAC: RBRR_CLOUD_PREFIX/RUNTIME_PREFIX
- ₢A6AA6: rbfl_yoke / rbw-dY (new surface)
- Slated: ₢A_AAQ (rekon rewrite), ₢A_AAR (jettison rewrite), ₢A_AAP (live burn-in)

### cloudbuild-ark-basename-passthrough (₢A_AAU) [complete]

**[260426-0957] complete**

## Character

Mechanical extraction across a producer-consumer language boundary. Director-side changes thread new env vars through three jq substitution blocks; consumer-side changes (CB step bash + Python) replace literals with env-var reads. Each site is small and uniform; the design thought is the boundary contract — not the per-site edits.

Pre-AAP positioning: the basename literals must come into discipline before the live burn-in so AAP exercises the corrected pass-through end-to-end. Slotting after AAS keeps the theurge-coverage-catchup additive work uninterrupted.

## Docket

GAR ark basenames (`image`, `vouch`, `pouch`, `about`, `attest`, `diags`) currently appear as bare literals inside Cloud Build step scripts. The Director-side `RBGC_ARK_BASENAME_*` constants do not propagate across the boundary; only `_RBG?_HALLMARKS_ROOT` does. A future rename on the Director side would silently mismatch the CB-side literals — producer and consumer would write/read different paths, builds would report success, vouch would surface nothing.

### Pattern

Pass-through, matching the existing `_RBG?_HALLMARKS_ROOT` discipline. The Director already builds three env-var groups via `automapSubstitutions` (`_RBGA_*`, `_RBGV_*`, `_RBGY_*`); add `_RBG?_ARK_BASENAME_*` entries to whichever groups need them.

### Per-group basename needs (verify on touch — line numbers will drift)

- `_RBGY_*` (rbgjb buildx/push group):  IMAGE (write), ATTEST (write), DIAGS (write)
- `_RBGA_*` (rbgja about/syft group):    IMAGE (read), ABOUT (write), DIAGS (read)
- `_RBGV_*` (rbgjv vouch/verify group):  IMAGE (read), VOUCH (write), ATTEST (read)

Pouch only appears at the Director construction site (`rbfd_FoundryDirectorBuild.sh:728`) where `RBGC_ARK_BASENAME_POUCH` is already in use. Pouch is GCB input, not output — no pass-through needed.

### Director-side jq blocks to extend

- `rbfd_FoundryDirectorBuild.sh:638-687` — conjure stitcher (sets `_RBGY_*` and `_RBGA_*`)
- `rbfv_FoundryVerify.sh:302-357` — graft vouch substitutions (`_RBGA_*` and `_RBGV_*`)
- `rbfv_FoundryVerify.sh:727-...` — multi-mode vouch substitutions (`_RBGA_*` and `_RBGV_*`)

Each block adds a small block of `--arg zjq_basename_<type> "${RBGC_ARK_BASENAME_<TYPE>}"` lines and corresponding `_RBG?_ARK_BASENAME_<TYPE>: $zjq_basename_<type>` substitution entries — only those needed by that group's consumer scripts.

### Consumer-side sites

Bash CB steps:
- `rbgja/rbgja02-syft-per-platform.sh:28` — bare `image` → `${_RBGA_ARK_BASENAME_IMAGE}`
- `rbgja/rbgja04-assemble-push-about.sh:31` — bare `about` → `${_RBGA_ARK_BASENAME_ABOUT}`
- `rbgjb/rbgjb03-buildx-push-image.sh:39` — bare `image` → `${_RBGY_ARK_BASENAME_IMAGE}`
- `rbgjb/rbgjb04-per-platform-pullback.sh:28-29` — bare `image`, `attest`
- `rbgjb/rbgjb05-push-per-platform.sh:25` — bare `attest` → `${_RBGY_ARK_BASENAME_ATTEST}`
- `rbgjb/rbgjb06-push-diags.sh:35` — bare `diags` → `${_RBGY_ARK_BASENAME_DIAGS}`
- `rbgjv/rbgjv03-assemble-push-vouch.sh:25` — bare `vouch` → `${_RBGV_ARK_BASENAME_VOUCH}`

Python CB steps:
- `rbgja/rbgja01-discover-platforms.py:93,94` — bare `image`, `diags`; replace with `os.environ["_RBGA_ARK_BASENAME_IMAGE"]` etc. via the existing `require_env` helper
- `rbgjv/rbgjv02-verify-provenance.py:93,155` — bare `image`, `attest`; same pattern with `_RBGV_*`

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (no behavior change to fast suite)
- `tt/rbtd-s.TestSuite.crucible.sh` — green (exercises producer→consumer end-to-end)
- One conjure ordain against `rbev-busybox` succeeds against the current depot, with `tt/rbw-ir.DirectorRekonsImages.sh` confirming all 5-6 ark packages land correctly under `<prefix>hallmarks/<H>/`
- One graft ordain against `rbev-graft-demo` succeeds (graft path lacks pouch but still pushes image+vouch+about+attest)
- Spot-check that CB build logs from a conjure show env vars resolved (the substitutions surface in the rendered cloudbuild.json which `rbfd` writes locally before submission — diff against current shape to confirm new substitution lines appear)

### Out of scope

- The three-prefix question (`_RBGA_` vs `_RBGV_` vs `_RBGY_`). The scoping may be load-bearing per-step-group isolation or it may be vestigial. Investigate as a separate pace if it surfaces during this work.
- The `_RBGY_HALLMARKS_ROOT` vs `_RBGA_HALLMARKS_ROOT` overlap — same value passed under two names. Out of scope unless this work reveals a clean unification.
- Director-side `RBGC_ARK_BASENAME_*` constants themselves — already in discipline.
- OCI protocol keywords (`manifests`, `blobs`, `tags/list`, `v2`) — separate pace ₢A_AAM.
- Theurge-side equivalent in `rbtdrc_crucible.rs` — already disciplined via `RBTDRC_ARK_BASENAME_IMAGE`/`_VOUCH` constants.

### Cadence

Code all sites, run fast suite → crucible suite, notch once when green. Second notch only if test feedback requires a substantive fix.

### Discovery context

Surfaced during ₣A_ groom (260425) when reviewing trust signals for hallmark/reliquaries/enshrines magic strings. Bash-side modules and theurge-side Rust use named constants throughout; the Cloud Build submission boundary turned out to be the exception. Flagged as the kind of integration-bug class that AAP's burn-in is set up to catch — moving it ahead of AAP keeps AAP focused on layout-grammar surprises rather than this known gap.

**[260425-0756] rough**

## Character

Mechanical extraction across a producer-consumer language boundary. Director-side changes thread new env vars through three jq substitution blocks; consumer-side changes (CB step bash + Python) replace literals with env-var reads. Each site is small and uniform; the design thought is the boundary contract — not the per-site edits.

Pre-AAP positioning: the basename literals must come into discipline before the live burn-in so AAP exercises the corrected pass-through end-to-end. Slotting after AAS keeps the theurge-coverage-catchup additive work uninterrupted.

## Docket

GAR ark basenames (`image`, `vouch`, `pouch`, `about`, `attest`, `diags`) currently appear as bare literals inside Cloud Build step scripts. The Director-side `RBGC_ARK_BASENAME_*` constants do not propagate across the boundary; only `_RBG?_HALLMARKS_ROOT` does. A future rename on the Director side would silently mismatch the CB-side literals — producer and consumer would write/read different paths, builds would report success, vouch would surface nothing.

### Pattern

Pass-through, matching the existing `_RBG?_HALLMARKS_ROOT` discipline. The Director already builds three env-var groups via `automapSubstitutions` (`_RBGA_*`, `_RBGV_*`, `_RBGY_*`); add `_RBG?_ARK_BASENAME_*` entries to whichever groups need them.

### Per-group basename needs (verify on touch — line numbers will drift)

- `_RBGY_*` (rbgjb buildx/push group):  IMAGE (write), ATTEST (write), DIAGS (write)
- `_RBGA_*` (rbgja about/syft group):    IMAGE (read), ABOUT (write), DIAGS (read)
- `_RBGV_*` (rbgjv vouch/verify group):  IMAGE (read), VOUCH (write), ATTEST (read)

Pouch only appears at the Director construction site (`rbfd_FoundryDirectorBuild.sh:728`) where `RBGC_ARK_BASENAME_POUCH` is already in use. Pouch is GCB input, not output — no pass-through needed.

### Director-side jq blocks to extend

- `rbfd_FoundryDirectorBuild.sh:638-687` — conjure stitcher (sets `_RBGY_*` and `_RBGA_*`)
- `rbfv_FoundryVerify.sh:302-357` — graft vouch substitutions (`_RBGA_*` and `_RBGV_*`)
- `rbfv_FoundryVerify.sh:727-...` — multi-mode vouch substitutions (`_RBGA_*` and `_RBGV_*`)

Each block adds a small block of `--arg zjq_basename_<type> "${RBGC_ARK_BASENAME_<TYPE>}"` lines and corresponding `_RBG?_ARK_BASENAME_<TYPE>: $zjq_basename_<type>` substitution entries — only those needed by that group's consumer scripts.

### Consumer-side sites

Bash CB steps:
- `rbgja/rbgja02-syft-per-platform.sh:28` — bare `image` → `${_RBGA_ARK_BASENAME_IMAGE}`
- `rbgja/rbgja04-assemble-push-about.sh:31` — bare `about` → `${_RBGA_ARK_BASENAME_ABOUT}`
- `rbgjb/rbgjb03-buildx-push-image.sh:39` — bare `image` → `${_RBGY_ARK_BASENAME_IMAGE}`
- `rbgjb/rbgjb04-per-platform-pullback.sh:28-29` — bare `image`, `attest`
- `rbgjb/rbgjb05-push-per-platform.sh:25` — bare `attest` → `${_RBGY_ARK_BASENAME_ATTEST}`
- `rbgjb/rbgjb06-push-diags.sh:35` — bare `diags` → `${_RBGY_ARK_BASENAME_DIAGS}`
- `rbgjv/rbgjv03-assemble-push-vouch.sh:25` — bare `vouch` → `${_RBGV_ARK_BASENAME_VOUCH}`

Python CB steps:
- `rbgja/rbgja01-discover-platforms.py:93,94` — bare `image`, `diags`; replace with `os.environ["_RBGA_ARK_BASENAME_IMAGE"]` etc. via the existing `require_env` helper
- `rbgjv/rbgjv02-verify-provenance.py:93,155` — bare `image`, `attest`; same pattern with `_RBGV_*`

### Verification

- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green (no behavior change to fast suite)
- `tt/rbtd-s.TestSuite.crucible.sh` — green (exercises producer→consumer end-to-end)
- One conjure ordain against `rbev-busybox` succeeds against the current depot, with `tt/rbw-ir.DirectorRekonsImages.sh` confirming all 5-6 ark packages land correctly under `<prefix>hallmarks/<H>/`
- One graft ordain against `rbev-graft-demo` succeeds (graft path lacks pouch but still pushes image+vouch+about+attest)
- Spot-check that CB build logs from a conjure show env vars resolved (the substitutions surface in the rendered cloudbuild.json which `rbfd` writes locally before submission — diff against current shape to confirm new substitution lines appear)

### Out of scope

- The three-prefix question (`_RBGA_` vs `_RBGV_` vs `_RBGY_`). The scoping may be load-bearing per-step-group isolation or it may be vestigial. Investigate as a separate pace if it surfaces during this work.
- The `_RBGY_HALLMARKS_ROOT` vs `_RBGA_HALLMARKS_ROOT` overlap — same value passed under two names. Out of scope unless this work reveals a clean unification.
- Director-side `RBGC_ARK_BASENAME_*` constants themselves — already in discipline.
- OCI protocol keywords (`manifests`, `blobs`, `tags/list`, `v2`) — separate pace ₢A_AAM.
- Theurge-side equivalent in `rbtdrc_crucible.rs` — already disciplined via `RBTDRC_ARK_BASENAME_IMAGE`/`_VOUCH` constants.

### Cadence

Code all sites, run fast suite → crucible suite, notch once when green. Second notch only if test feedback requires a substantive fix.

### Discovery context

Surfaced during ₣A_ groom (260425) when reviewing trust signals for hallmark/reliquaries/enshrines magic strings. Bash-side modules and theurge-side Rust use named constants throughout; the Cloud Build submission boundary turned out to be the exception. Flagged as the kind of integration-bug class that AAP's burn-in is set up to catch — moving it ahead of AAP keeps AAP focused on layout-grammar surprises rather than this known gap.

### regenerate-depot (₢A_AAE) [abandoned]

**[260511-1618] abandoned**

## Character
Operational — live GCP infrastructure change, requires payor credentials. The forcing function for the entire heat: AAA–AAD all land before this; this regen validates the new prefix wiring and GAR layout end-to-end against fresh infrastructure.

## Preconditions

Before running, set non-empty prefixes in `.rbk/rbrr.env` (so the regen exercises non-empty paths, not the empty-prefix shortcut):

```
RBRR_CLOUD_PREFIX="rbm-"
RBRR_RUNTIME_PREFIX="dev-"
```

Distinct values for cloud and runtime are deliberate — proves the variables are wired distinctly, not aliased through any common derivation. `rbm-` matches the project family prefix; `dev-` signals local-machine scope. Both are 4 chars (well under the 11-char ceiling) and pass the regex.

Also confirm before running:
- AAA–AAD all landed and committed
- `tt/rbw-rrv.ValidateRepoRegime.sh` — passes
- `tt/rbw-rrr.RenderRepoRegime.sh` — shows both prefixes rendered with `rbm-` / `dev-`
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green

## Work

Run depot unmake + depot levy to regenerate the depot with the new prefix-aware configuration:

1. `tt/rbw-dU.PayorUnmakesDepot.sh` — unmake the existing depot
2. `tt/rbw-dL.PayorLeviesDepot.sh` — provision fresh depot with the new prefix-aware GAR layout

## Verification

- `tt/rbw-dl.PayorListsDepots.sh` — confirms new depot is active
- GAR repository created, worker pools provisioned (verify via gcloud or via tabtarget output)
- `tt/rbw-fO.DirectorOrdainsHallmark.sh` against a small vessel (suggest sentry-deb-tether or similar small target) — confirms hallmark publication into `${RBRR_CLOUD_PREFIX}hallmarks/<hallmark>/...` layout
- `tt/rbw-dE.DirectorEnshrinesVessel.sh` — confirms enshrine lands under `${RBRR_CLOUD_PREFIX}enshrines/`
- `tt/rbw-dI.DirectorInscribesReliquary.sh` — confirms reliquary lands under `${RBRR_CLOUD_PREFIX}reliquaries/<date>/<tool>`
- `tt/rbw-ft.RetrieverTalliesHallmarks.sh` — tally surfaces the published hallmark with prefixed paths

If any verification step fails, the failure is informative: the prefix wiring or layout migration has a gap that needs to be back-fixed in AAB or AAK before depot-regen can succeed.

**[260423-0901] rough**

## Character
Operational — live GCP infrastructure change, requires payor credentials. The forcing function for the entire heat: AAA–AAD all land before this; this regen validates the new prefix wiring and GAR layout end-to-end against fresh infrastructure.

## Preconditions

Before running, set non-empty prefixes in `.rbk/rbrr.env` (so the regen exercises non-empty paths, not the empty-prefix shortcut):

```
RBRR_CLOUD_PREFIX="rbm-"
RBRR_RUNTIME_PREFIX="dev-"
```

Distinct values for cloud and runtime are deliberate — proves the variables are wired distinctly, not aliased through any common derivation. `rbm-` matches the project family prefix; `dev-` signals local-machine scope. Both are 4 chars (well under the 11-char ceiling) and pass the regex.

Also confirm before running:
- AAA–AAD all landed and committed
- `tt/rbw-rrv.ValidateRepoRegime.sh` — passes
- `tt/rbw-rrr.RenderRepoRegime.sh` — shows both prefixes rendered with `rbm-` / `dev-`
- `tt/rbtd-s.TestSuite.fast.sh` — 75/75 green

## Work

Run depot unmake + depot levy to regenerate the depot with the new prefix-aware configuration:

1. `tt/rbw-dU.PayorUnmakesDepot.sh` — unmake the existing depot
2. `tt/rbw-dL.PayorLeviesDepot.sh` — provision fresh depot with the new prefix-aware GAR layout

## Verification

- `tt/rbw-dl.PayorListsDepots.sh` — confirms new depot is active
- GAR repository created, worker pools provisioned (verify via gcloud or via tabtarget output)
- `tt/rbw-fO.DirectorOrdainsHallmark.sh` against a small vessel (suggest sentry-deb-tether or similar small target) — confirms hallmark publication into `${RBRR_CLOUD_PREFIX}hallmarks/<hallmark>/...` layout
- `tt/rbw-dE.DirectorEnshrinesVessel.sh` — confirms enshrine lands under `${RBRR_CLOUD_PREFIX}enshrines/`
- `tt/rbw-dI.DirectorInscribesReliquary.sh` — confirms reliquary lands under `${RBRR_CLOUD_PREFIX}reliquaries/<date>/<tool>`
- `tt/rbw-ft.RetrieverTalliesHallmarks.sh` — tally surfaces the published hallmark with prefixed paths

If any verification step fails, the failure is informative: the prefix wiring or layout migration has a gap that needs to be back-fixed in AAB or AAK before depot-regen can succeed.

**[260415-1504] rough**

## Character
Operational — live GCP infrastructure change, requires payor credentials.

## Work
Run depot unmake + depot levy to regenerate the depot with the new prefix-aware configuration. Verify GAR repository is created, worker pools are provisioned, and the prefixed image namespace is functional. Confirm enshrine and inscribe operations work against the fresh depot.

### run-complete-test-suite (₢A_AAF) [complete]

**[260513-1324] complete**

## Character
Narrower validation, macOS only — AAP carries the two-host gauntlet exit. This pace exercises operator-config prefixes (rbm-/dev-) against the live substrate (canest2bhm100003), which AAP's throwaway-prefix posture won't cover.

## Work

Run sequentially on macOS:

- `tt/rbtd-s.TestSuite.fast.sh`
- `tt/rbtd-s.TestSuite.service.sh`
- Kludge tadmor's vessels: `rbw-cKS tadmor`, `rbw-cKB tadmor` (bottle vessel name resolved from tadmor's nameplate at mount-time).
- `tt/rbtd-r.FixtureRun.tadmor.sh` (charge → 34 security cases → quench).

Diagnose failures into AAB (prefix gap) or AAK (layout gap) back-fixes. srjcl, pluml, moriah deliberately out of scope. Linux coverage deferred to AAP.

**[260513-1228] rough**

## Character
Narrower validation, macOS only — AAP carries the two-host gauntlet exit. This pace exercises operator-config prefixes (rbm-/dev-) against the live substrate (canest2bhm100003), which AAP's throwaway-prefix posture won't cover.

## Work

Run sequentially on macOS:

- `tt/rbtd-s.TestSuite.fast.sh`
- `tt/rbtd-s.TestSuite.service.sh`
- Kludge tadmor's vessels: `rbw-cKS tadmor`, `rbw-cKB tadmor` (bottle vessel name resolved from tadmor's nameplate at mount-time).
- `tt/rbtd-r.FixtureRun.tadmor.sh` (charge → 34 security cases → quench).

Diagnose failures into AAB (prefix gap) or AAK (layout gap) back-fixes. srjcl, pluml, moriah deliberately out of scope. Linux coverage deferred to AAP.

**[260423-0911] rough**

## Character
Validation — patience and diagnostic attention. Failures are the point. Preflight rebuilds the kit-shipped vessels that AAE's depot regen wiped, then runs the complete suite against the freshly stocked depot.

## Preflight: rebuild kit-shipped vessels

AAE wipes the depot. The complete test suite includes `tadmor-security`, `srjcl-jupyter`, and `pluml-diagram` fixtures, each requiring built vessel images that no longer exist after regen. AAE's verification only exercises one small vessel (sentry-deb-tether or similar) — it doesn't restock the suite-needed vessels.

Before running the suite, ordain every vessel referenced by complete-suite fixtures:

- Tadmor: sentry vessel (`rbev-sentry-deb-tether`) + bottle vessel (verify exact name in nameplate)
- Srjcl: bottle vessel (`rbev-bottle-anthropic-jupyter`)
- Pluml: bottle vessel (`rbev-bottle-plantuml`)
- Plus any additional vessels referenced by enrollment-validation, regime-validation, regime-smoke, access-probe, four-mode (most of these don't need built images, but verify)

For each vessel: `tt/rbw-fO.DirectorOrdainsHallmark.sh <vessel>` (or vessel-mode-appropriate ordain pathway). Confirms the new prefix-aware GAR layout from AAB/AAK works end-to-end against the regenerated depot.

## Work

Run `tt/rbtd-s.TestSuite.complete.sh` — the full 122-case suite covering enrollment-validation, regime-validation, regime-smoke, access-probe, four-mode, tadmor-security, srjcl-jupyter, and pluml-diagram. All container names and image references should now carry the configured prefixes (`rbm-` cloud, `dev-` runtime per AAE preconditions).

Diagnose and fix any failures caused by the prefix wiring. Failures fall into two buckets:
- **Prefix gap** — a hardcoded path or container name slipped past AAB's grep sweep. Back-fix in AAB and re-verify.
- **Layout gap** — a producer or consumer site wasn't updated to the new GAR layout in AAK. Back-fix in AAK and re-verify.

The complete-suite failure surface is the safety net for both AAB and AAK gaps.

**[260415-1505] rough**

## Character
Validation — patience and diagnostic attention. Failures are the point.

## Work
Run `tt/rbtd-s.TestSuite.complete.sh` — the full 122-case suite covering enrollment-validation, regime-validation, regime-smoke, access-probe, four-mode, tadmor-security, srjcl-jupyter, and pluml-diagram. All container names and image references should now carry the configured prefixes. Diagnose and fix any failures caused by the prefix wiring.

### rbgo-oauth-retry-and-diagnostics-completion (₢A_AAa) [complete]

**[260511-1613] complete**

## Character

Design conversation requiring judgment. Re-study before implementing.

## Docket

### Issue

`zrbgo_exchange_jwt_capture` runs a single curl POST to `oauth2.googleapis.com` and returns 1 on any failure with no diagnostic surfaced — the wrapper `buc_die "Failed to get Director OAuth token"` identically covers credentials-bad, endpoint-down, JWT-malformed, network-blip, DNS-fail, TLS-handshake-fail. Observed in ₢A_AAS: a brief connect-phase timeout (curl 28) on a single director op killed a multi-minute test. The same operation succeeded on retry minutes later.

Two halves:

- **Diagnostic visibility** — already landed in ₢A_AAS. `zrbgo_exchange_jwt_capture` now `buc_warn`s the curl stderr on network failure and the redacted response body on missing-access-token, before returning 1. The wrapper `buc_die` still fires; the cause now appears above it.
- **Retry on transient failures** — open. Single attempt is fragile against ordinary network blips. Roughly 11 call sites go through `rbgo_get_token_capture` from rbfd / rbfl / rbfv.

### Survey of retry mechanisms in the codebase

- **`zrbgv_http_get_with_5xx_retry` in `rbgv_AccessProbe.sh`** — bounded exponential backoff (3 attempts, 2s doubling). Retries HTTP 5xx only. Curl-network failures (`curl_status != 0`) → immediate `buc_die`. Surfaces curl stderr each iteration. Closest existing pattern; would not have helped with a curl exit 28. GET-only.
- **No POST retry helper exists.** The OAuth exchange is the only POST site that needs auth-aware retry.
- **`zrbhh_pull_with_retry` in `vov_veiled/ABANDONED-github/`** — abandoned, not in use.
- **Counter-precedent** — `rbcc_Constants.sh` carries a comment expressing aversion to retry-loop complexity in bash, scoped to *bottle test fixture readiness* (waiting for local services to start). Different problem from outbound HTTP; not strictly authoritative for OAuth.

### Open

Whether to extend / generalize / inline the retry, what curl exit codes count as "transient" vs configuration-deterministic, and whether to extend diagnostic surfacing to the JWT-build openssl path and the RBRA-validation paths in the same family. Decide via conversation when this pace mounts.

**[260426-0659] rough**

## Character

Design conversation requiring judgment. Re-study before implementing.

## Docket

### Issue

`zrbgo_exchange_jwt_capture` runs a single curl POST to `oauth2.googleapis.com` and returns 1 on any failure with no diagnostic surfaced — the wrapper `buc_die "Failed to get Director OAuth token"` identically covers credentials-bad, endpoint-down, JWT-malformed, network-blip, DNS-fail, TLS-handshake-fail. Observed in ₢A_AAS: a brief connect-phase timeout (curl 28) on a single director op killed a multi-minute test. The same operation succeeded on retry minutes later.

Two halves:

- **Diagnostic visibility** — already landed in ₢A_AAS. `zrbgo_exchange_jwt_capture` now `buc_warn`s the curl stderr on network failure and the redacted response body on missing-access-token, before returning 1. The wrapper `buc_die` still fires; the cause now appears above it.
- **Retry on transient failures** — open. Single attempt is fragile against ordinary network blips. Roughly 11 call sites go through `rbgo_get_token_capture` from rbfd / rbfl / rbfv.

### Survey of retry mechanisms in the codebase

- **`zrbgv_http_get_with_5xx_retry` in `rbgv_AccessProbe.sh`** — bounded exponential backoff (3 attempts, 2s doubling). Retries HTTP 5xx only. Curl-network failures (`curl_status != 0`) → immediate `buc_die`. Surfaces curl stderr each iteration. Closest existing pattern; would not have helped with a curl exit 28. GET-only.
- **No POST retry helper exists.** The OAuth exchange is the only POST site that needs auth-aware retry.
- **`zrbhh_pull_with_retry` in `vov_veiled/ABANDONED-github/`** — abandoned, not in use.
- **Counter-precedent** — `rbcc_Constants.sh` carries a comment expressing aversion to retry-loop complexity in bash, scoped to *bottle test fixture readiness* (waiting for local services to start). Different problem from outbound HTTP; not strictly authoritative for OAuth.

### Open

Whether to extend / generalize / inline the retry, what curl exit codes count as "transient" vs configuration-deterministic, and whether to extend diagnostic surfacing to the JWT-build openssl path and the RBRA-validation paths in the same family. Decide via conversation when this pace mounts.

### research-config-directory-naming (₢A_AAJ) [abandoned]

**[260514-1129] abandoned**

## Character

Heat ₣BK (rbk-12-mvp-moorings-cutover) was nominated during the agent conversation that would have been this pace's reslate-to-heat-construction-directive moment. The decision content captured below stands as the canonical record of the design that drove ₣BK's pace decomposition.

This pace now serves three purposes, all **post-₣BK**:

1. **Block on ₣BK completion.** Do not mount until ₣BK has wrapped all ten paces and merged. Operator-managed gate; ₣BK runs ahead of A_'s closure and that is expected.
2. **Review the cutover.** Walk the merged ₣BK work against the decisions captured below — confirm renames landed as specified, `rbm*_` prefix family registered, BCG "Tabtarget Path Indirection" subsection codified, trampoline pattern correct, symbolic-constant discipline applied. Extra review is the point.
3. **Update A_'s paddock.** Record the rbm*_ prefix family, hard-cutover discipline, BCG-codification commitment, trampoline's named goal (execution-context normalization), and symbolic-constant discipline — the items the original docket's "Paddock update on wrap" section flagged.

If review surfaces drift between ₣BK's outcome and the captured design, file findings as new paces (in ₣BK or a follow-up heat per scope) rather than redoing work in AAJ. AAJ's wrap closes ₣A_; the review verdict becomes part of A_'s wrap commit message.

The original deferred-reslate pattern (preserve below as historical context) accomplished its purpose: design captured at the moment of surfacing, then synthesized into a heat decomposition without forcing the heat-sized rename into ₣A_'s GCP-side scope.

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | Consumer-side bootstrap file; layout standardization removes its reason to exist (distinct from kit-side `Tools/buk/bubc_constants.sh` — see Symbolic-constant discipline below) |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns. (BUBC already uses `rbmn_` for `rbmn_nodes_subdir` — disposition is ₣BK pace 1.)

### Final layout

```
rbmm_moorings/                      <- single consumer-owned entry at repo root
  burc.env                          <- BUK regime (authored)
  rbrr.env                          <- RBK repo regime (authored)
  rbrp.env                          <- RBK payor regime (authored)
  tadmor/
    rbrn.env                        <- nameplate regime (authored)
    compose.yml                     <- per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              <- kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                <- base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     <- trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      <- tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

**Execution-context normalization (named goal).** z-launcher.sh `cd`s to the repo root (resolved via `${BASH_SOURCE[0]%/*}/..`) before exec'ing the named launcher. The single `../` in the system lives here, used once at trampoline entry to establish a stable cwd. Two consequences fall out:

- Tabtargets carry zero `../` — each invokes its sibling `z-launcher.sh` directly (`exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <id> "${@}"`). Today's pervasive per-tabtarget `${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}` pattern dissolves.
- Workbenches always start with cwd = repo root, regardless of where the user typed the tabtarget from. Downstream relative paths resolve identically. Today's `BURD_LAUNCHER` pattern leaves cwd at the user's invocation directory, which the trampoline closes.

This is the load-bearing reason to introduce z-launcher.sh — not just rename consolidation, but execution-context determinism.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover is cleaner.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).
- **Launchers fold into `tt/` to eliminate `../` entirely** — considered, rejected. Reading: launchers are kit-shipped boilerplate (regenerated by `rbw-MG`), so co-locating them with tabtargets under `tt/_launchers/` would buy zero `../` in the system. Rejected because: (a) the docket places launchers under moorings on the basis that consumers occasionally tune them, (b) the single `../` inside z-launcher is well-localized and load-bearing for the chdir-to-root mechanic above — hiding it inside `tt/` doesn't simplify the model, it just relocates it. The named goal is execution-context normalization, not `../` minimization.
- **No kit-side fact locale needed (original "kit-side constants suffice" stance)** — superseded. The original wording was directionally right but understated the discipline. See Symbolic-constant discipline below.

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (executed under ₣BK)

The actual filesystem rename is the work of ₣BK. The migration scope below is the inventory that drove ₣BK's pace decomposition; preserved here as the canonical reference against which to review ₣BK's outcome.

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Symbolic-constant discipline

Embedded `.buk`/`.rbk` literals in kit code are magic strings and the rename heat must sweep them to symbolic constants — not just retarget the literal. Symbolic naming is flat better, and bash's lack of compile-time checks makes the discipline more important here, not less. The rename is the natural moment to apply it because the constant's *value* changes exactly once while call sites stay clean across the cutover.

The kit-side fact locale on the BUK side is `Tools/buk/bubc_constants.sh`, established during ₣A_ (concurrent with this pace, separate scope). It holds path fragments and other crosscutting strings as proper symbols.

The RBK-side question was resolved during ₣BK planning: `Tools/rbk/rbcc_Constants.sh` is the existing RBK fact locale (already holds file-path literals like `RBCC_rbrn_file`, `RBCC_rbra_file`); ₣BK extends it rather than creating a new file. Review verifies this landed as specified.

The Renames table line "`.buk/rbbc_constants.sh` Eliminated" is unchanged: that file was *consumer-side bootstrap* whose reason for existing dissolves with layout standardization. The kit-side locale (`Tools/buk/bubc_constants.sh` and the extended `rbcc_Constants.sh`) is a different beast and is load-bearing for symbolic-constant discipline going forward.

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### BCG codification

The trampoline (`tt/z-launcher.sh`) is a **new pattern** — single-point-of-customization for moorings location at the tabtarget layer plus execution-context normalization (cwd = repo root before workbench dispatch), parallel to BURC providing it at the CLI layer (BCG:171-181 dispatch-provided `BURD_*`). Without BCG codification it remains tribal knowledge.

Add a subsection peer to "Dispatch-Provided Directory Variables" titled **"Tabtarget Path Indirection"** stating:

> Tabtargets must not hardcode moorings/config-directory paths. The single tabtarget `tt/z-launcher.sh` resolves the moorings location, normalizes cwd to the repo root, and dispatches to the named launcher; all other tabtargets invoke it with a workbench identifier. This makes the moorings location a single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer) and guarantees workbenches start with a deterministic cwd regardless of user invocation directory.

The BCG update lands in ₣BK **alongside** the trampoline introduction, not before — BCG should not document vapor. The pattern also extends BCG's CLI-as-Module-Gateway principle (BCG:44-77) one level further: tabtargets delegate path resolution and execution-context setup through a stable indirection layer rather than hardcoding implementation details.

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete consumer-side `.buk/rbbc_constants.sh` (reason-to-exist absorbed by layout standardization)
- Move `rbmP_laterSecurityAudit.md` -> `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope (within this pace's review)

- Re-executing any work that ₣BK already performed.
- Extending moorings pattern to other kits.
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored).
- Redesigning the launcher infrastructure beyond what ₣BK landed.

## Paddock update on wrap (the canonical list to fold into A_)

A_ paddock should record (additive to what's already there):

- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The hard-cutover discipline as executed under ₣BK
- The BCG-codification outcome: trampoline pattern documented as "Tabtarget Path Indirection" subsection
- The trampoline's named goal: execution-context normalization (chdir to repo root before workbench dispatch), not just rename consolidation
- The symbolic-constant discipline: `.buk`/`.rbk` literals were magic strings; ₣BK swept them to constants from kit-side fact locales (BUBC and the extended RBCC)
- Pointer to ₣BK as the heat that executed the moorings cutover

The filesystem rename was NOT a prerequisite to ₢A_AAE — confirmed correct as executed.

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — relevant sections: lines 21-30 (Load-Bearing Complexity), 44-77 (CLI as Module Gateway), 171-181 (Dispatch-Provided Directory Variables)
- ₣BK / rbk-12-mvp-moorings-cutover — the heat that executed the cutover this pace reviews

**[260513-1305] rough**

## Character

Heat ₣BK (rbk-12-mvp-moorings-cutover) was nominated during the agent conversation that would have been this pace's reslate-to-heat-construction-directive moment. The decision content captured below stands as the canonical record of the design that drove ₣BK's pace decomposition.

This pace now serves three purposes, all **post-₣BK**:

1. **Block on ₣BK completion.** Do not mount until ₣BK has wrapped all ten paces and merged. Operator-managed gate; ₣BK runs ahead of A_'s closure and that is expected.
2. **Review the cutover.** Walk the merged ₣BK work against the decisions captured below — confirm renames landed as specified, `rbm*_` prefix family registered, BCG "Tabtarget Path Indirection" subsection codified, trampoline pattern correct, symbolic-constant discipline applied. Extra review is the point.
3. **Update A_'s paddock.** Record the rbm*_ prefix family, hard-cutover discipline, BCG-codification commitment, trampoline's named goal (execution-context normalization), and symbolic-constant discipline — the items the original docket's "Paddock update on wrap" section flagged.

If review surfaces drift between ₣BK's outcome and the captured design, file findings as new paces (in ₣BK or a follow-up heat per scope) rather than redoing work in AAJ. AAJ's wrap closes ₣A_; the review verdict becomes part of A_'s wrap commit message.

The original deferred-reslate pattern (preserve below as historical context) accomplished its purpose: design captured at the moment of surfacing, then synthesized into a heat decomposition without forcing the heat-sized rename into ₣A_'s GCP-side scope.

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | Consumer-side bootstrap file; layout standardization removes its reason to exist (distinct from kit-side `Tools/buk/bubc_constants.sh` — see Symbolic-constant discipline below) |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns. (BUBC already uses `rbmn_` for `rbmn_nodes_subdir` — disposition is ₣BK pace 1.)

### Final layout

```
rbmm_moorings/                      <- single consumer-owned entry at repo root
  burc.env                          <- BUK regime (authored)
  rbrr.env                          <- RBK repo regime (authored)
  rbrp.env                          <- RBK payor regime (authored)
  tadmor/
    rbrn.env                        <- nameplate regime (authored)
    compose.yml                     <- per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              <- kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                <- base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     <- trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      <- tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

**Execution-context normalization (named goal).** z-launcher.sh `cd`s to the repo root (resolved via `${BASH_SOURCE[0]%/*}/..`) before exec'ing the named launcher. The single `../` in the system lives here, used once at trampoline entry to establish a stable cwd. Two consequences fall out:

- Tabtargets carry zero `../` — each invokes its sibling `z-launcher.sh` directly (`exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <id> "${@}"`). Today's pervasive per-tabtarget `${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}` pattern dissolves.
- Workbenches always start with cwd = repo root, regardless of where the user typed the tabtarget from. Downstream relative paths resolve identically. Today's `BURD_LAUNCHER` pattern leaves cwd at the user's invocation directory, which the trampoline closes.

This is the load-bearing reason to introduce z-launcher.sh — not just rename consolidation, but execution-context determinism.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover is cleaner.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).
- **Launchers fold into `tt/` to eliminate `../` entirely** — considered, rejected. Reading: launchers are kit-shipped boilerplate (regenerated by `rbw-MG`), so co-locating them with tabtargets under `tt/_launchers/` would buy zero `../` in the system. Rejected because: (a) the docket places launchers under moorings on the basis that consumers occasionally tune them, (b) the single `../` inside z-launcher is well-localized and load-bearing for the chdir-to-root mechanic above — hiding it inside `tt/` doesn't simplify the model, it just relocates it. The named goal is execution-context normalization, not `../` minimization.
- **No kit-side fact locale needed (original "kit-side constants suffice" stance)** — superseded. The original wording was directionally right but understated the discipline. See Symbolic-constant discipline below.

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (executed under ₣BK)

The actual filesystem rename is the work of ₣BK. The migration scope below is the inventory that drove ₣BK's pace decomposition; preserved here as the canonical reference against which to review ₣BK's outcome.

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Symbolic-constant discipline

Embedded `.buk`/`.rbk` literals in kit code are magic strings and the rename heat must sweep them to symbolic constants — not just retarget the literal. Symbolic naming is flat better, and bash's lack of compile-time checks makes the discipline more important here, not less. The rename is the natural moment to apply it because the constant's *value* changes exactly once while call sites stay clean across the cutover.

The kit-side fact locale on the BUK side is `Tools/buk/bubc_constants.sh`, established during ₣A_ (concurrent with this pace, separate scope). It holds path fragments and other crosscutting strings as proper symbols.

The RBK-side question was resolved during ₣BK planning: `Tools/rbk/rbcc_Constants.sh` is the existing RBK fact locale (already holds file-path literals like `RBCC_rbrn_file`, `RBCC_rbra_file`); ₣BK extends it rather than creating a new file. Review verifies this landed as specified.

The Renames table line "`.buk/rbbc_constants.sh` Eliminated" is unchanged: that file was *consumer-side bootstrap* whose reason for existing dissolves with layout standardization. The kit-side locale (`Tools/buk/bubc_constants.sh` and the extended `rbcc_Constants.sh`) is a different beast and is load-bearing for symbolic-constant discipline going forward.

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### BCG codification

The trampoline (`tt/z-launcher.sh`) is a **new pattern** — single-point-of-customization for moorings location at the tabtarget layer plus execution-context normalization (cwd = repo root before workbench dispatch), parallel to BURC providing it at the CLI layer (BCG:171-181 dispatch-provided `BURD_*`). Without BCG codification it remains tribal knowledge.

Add a subsection peer to "Dispatch-Provided Directory Variables" titled **"Tabtarget Path Indirection"** stating:

> Tabtargets must not hardcode moorings/config-directory paths. The single tabtarget `tt/z-launcher.sh` resolves the moorings location, normalizes cwd to the repo root, and dispatches to the named launcher; all other tabtargets invoke it with a workbench identifier. This makes the moorings location a single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer) and guarantees workbenches start with a deterministic cwd regardless of user invocation directory.

The BCG update lands in ₣BK **alongside** the trampoline introduction, not before — BCG should not document vapor. The pattern also extends BCG's CLI-as-Module-Gateway principle (BCG:44-77) one level further: tabtargets delegate path resolution and execution-context setup through a stable indirection layer rather than hardcoding implementation details.

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete consumer-side `.buk/rbbc_constants.sh` (reason-to-exist absorbed by layout standardization)
- Move `rbmP_laterSecurityAudit.md` -> `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope (within this pace's review)

- Re-executing any work that ₣BK already performed.
- Extending moorings pattern to other kits.
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored).
- Redesigning the launcher infrastructure beyond what ₣BK landed.

## Paddock update on wrap (the canonical list to fold into A_)

A_ paddock should record (additive to what's already there):

- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The hard-cutover discipline as executed under ₣BK
- The BCG-codification outcome: trampoline pattern documented as "Tabtarget Path Indirection" subsection
- The trampoline's named goal: execution-context normalization (chdir to repo root before workbench dispatch), not just rename consolidation
- The symbolic-constant discipline: `.buk`/`.rbk` literals were magic strings; ₣BK swept them to constants from kit-side fact locales (BUBC and the extended RBCC)
- Pointer to ₣BK as the heat that executed the moorings cutover

The filesystem rename was NOT a prerequisite to ₢A_AAE — confirmed correct as executed.

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — relevant sections: lines 21-30 (Load-Bearing Complexity), 44-77 (CLI as Module Gateway), 171-181 (Dispatch-Provided Directory Variables)
- ₣BK / rbk-12-mvp-moorings-cutover — the heat that executed the cutover this pace reviews

**[260502-0731] rough**

## Character

The moorings concept below is locked-in commitment, no longer tentative research. Pace remains **railed last** in ₣A_ by deliberate plan: when its turn comes (after AAS through AAM all wrap), reslate from this directive form into a **heat-construction directive** that synthesizes the inventory below into a pace decomposition, then nominates the moorings rename heat. AAJ's wrap closes ₣A_ and the rename heat exists as a freshly-nominated sibling.

This deferred-reslate pattern keeps the finding tied to the heat that surfaced it (no orphan future-heat drift) without forcing the heat-sized rename into ₣A_'s scope (which is GCP/cloud-side path grammar, orthogonal from filesystem moorings).

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | Consumer-side bootstrap file; layout standardization removes its reason to exist (distinct from kit-side `Tools/buk/bubc_constants.sh` — see Symbolic-constant discipline below) |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns.

### Final layout

```
rbmm_moorings/                      <- single consumer-owned entry at repo root
  burc.env                          <- BUK regime (authored)
  rbrr.env                          <- RBK repo regime (authored)
  rbrp.env                          <- RBK payor regime (authored)
  tadmor/
    rbrn.env                        <- nameplate regime (authored)
    compose.yml                     <- per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              <- kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                <- base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     <- trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      <- tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

**Execution-context normalization (named goal).** z-launcher.sh `cd`s to the repo root (resolved via `${BASH_SOURCE[0]%/*}/..`) before exec'ing the named launcher. The single `../` in the system lives here, used once at trampoline entry to establish a stable cwd. Two consequences fall out:

- Tabtargets carry zero `../` — each invokes its sibling `z-launcher.sh` directly (`exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <id> "${@}"`). Today's pervasive per-tabtarget `${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}` pattern dissolves.
- Workbenches always start with cwd = repo root, regardless of where the user typed the tabtarget from. Downstream relative paths resolve identically. Today's `BURD_LAUNCHER` pattern leaves cwd at the user's invocation directory, which the trampoline closes.

This is the load-bearing reason to introduce z-launcher.sh — not just rename consolidation, but execution-context determinism.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover is cleaner.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).
- **Launchers fold into `tt/` to eliminate `../` entirely** — considered, rejected. Reading: launchers are kit-shipped boilerplate (regenerated by `rbw-MG`), so co-locating them with tabtargets under `tt/_launchers/` would buy zero `../` in the system. Rejected because: (a) the docket places launchers under moorings on the basis that consumers occasionally tune them, (b) the single `../` inside z-launcher is well-localized and load-bearing for the chdir-to-root mechanic above — hiding it inside `tt/` doesn't simplify the model, it just relocates it. The named goal is execution-context normalization, not `../` minimization.
- **No kit-side fact locale needed (original "kit-side constants suffice" stance)** — superseded. The original wording was directionally right but understated the discipline. See Symbolic-constant discipline below.

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (for the follow-up rename heat)

The actual filesystem rename is out of scope for this heat (₣A_). It is intended as a separate follow-up heat. The migration scope below is captured here so the follow-up heat has a starting inventory.

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Symbolic-constant discipline

Embedded `.buk`/`.rbk` literals in kit code are magic strings and the rename heat must sweep them to symbolic constants — not just retarget the literal. Symbolic naming is flat better, and bash's lack of compile-time checks makes the discipline more important here, not less. The rename is the natural moment to apply it because the constant's *value* changes exactly once while call sites stay clean across the cutover.

The kit-side fact locale on the BUK side is `Tools/buk/bubc_constants.sh`, established during ₣A_ (concurrent with this pace, separate scope). It holds path fragments and other crosscutting strings as proper symbols.

The RBK-side question is open and the rename heat must decide it explicitly:

- BUK and RBK have somewhat different gravitational pulls — RBK has more cross-file path indirection history (the now-eliminated consumer-side `.buk/rbbc_constants.sh`); BUK is starting fresh with a kit-side analog.
- Whether RBK gains an analogous kit-side `Tools/rbk/rb??_constants.sh` locale, reuses BUK's, or genuinely needs no constants at all (because the trampoline + standardized layout removes all RBK-side path indirection), is a design decision — not a default-by-omission. The rename heat must make this call explicitly and record the rationale.

The Renames table line "`.buk/rbbc_constants.sh` Eliminated" is unchanged: that file was *consumer-side bootstrap* whose reason for existing dissolves with layout standardization. The kit-side locale (`Tools/buk/bubc_constants.sh` and any RBK analog) is a different beast and is load-bearing for symbolic-constant discipline going forward.

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### BCG codification

The trampoline (`tt/z-launcher.sh`) is a **new pattern** — single-point-of-customization for moorings location at the tabtarget layer plus execution-context normalization (cwd = repo root before workbench dispatch), parallel to BURC providing it at the CLI layer (BCG:171-181 dispatch-provided `BURD_*`). Without BCG codification it remains tribal knowledge.

Add a subsection peer to "Dispatch-Provided Directory Variables" titled **"Tabtarget Path Indirection"** stating:

> Tabtargets must not hardcode moorings/config-directory paths. The single tabtarget `tt/z-launcher.sh` resolves the moorings location, normalizes cwd to the repo root, and dispatches to the named launcher; all other tabtargets invoke it with a workbench identifier. This makes the moorings location a single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer) and guarantees workbenches start with a deterministic cwd regardless of user invocation directory.

The BCG update lands in the rename heat **alongside** the trampoline introduction, not before — BCG should not document vapor. The pattern also extends BCG's CLI-as-Module-Gateway principle (BCG:44-77) one level further: tabtargets delegate path resolution and execution-context setup through a stable indirection layer rather than hardcoding implementation details.

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete consumer-side `.buk/rbbc_constants.sh` (reason-to-exist absorbed by layout standardization)
- Move `rbmP_laterSecurityAudit.md` -> `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope

- Actually executing the renames (separate follow-up heat, not a pace in this heat)
- Extending moorings pattern to other kits
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored)
- Redesigning the launcher infrastructure beyond introducing `z-launcher.sh`
- The BUK kit-side fact locale itself (`Tools/buk/bubc_constants.sh`) — being established in a separate ₣A_ pace, not here

## Paddock update on wrap

A_ paddock should record (additive to what's already there):

- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The hard-cutover discipline for the moorings rename (when its own heat eventually runs)
- The BCG-codification commitment: rename heat lands trampoline AND adds BCG "Tabtarget Path Indirection" subsection
- The trampoline's named goal: execution-context normalization (chdir to repo root before workbench dispatch), not just rename consolidation
- The symbolic-constant discipline: `.buk`/`.rbk` literals are magic strings; the rename heat sweeps them to constants from kit-side fact locales (BUK's `Tools/buk/bubc_constants.sh` established in ₣A_; RBK analog is an open decision for the rename heat)

The filesystem rename is NOT a prerequisite to this heat's ₢A_AAE — it's a separate follow-up heat. Paddock should not claim otherwise.

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — relevant sections: lines 21-30 (Load-Bearing Complexity), 44-77 (CLI as Module Gateway), 171-181 (Dispatch-Provided Directory Variables)

**[260427-1455] rough**

## Character

Research completed — decisions recorded. Pace is **railed last** in ₣A_ by deliberate plan: when its turn comes (after AAS through AAM all wrap), reslate this pace from research into a **heat-construction directive** that synthesizes the inventory below into a pace decomposition, then nominates the moorings rename heat. AAJ's wrap closes ₣A_ and the rename heat exists as a freshly-nominated sibling.

This deferred-reslate pattern keeps the research finding tied to the heat that surfaced it (no orphan future-heat drift) without forcing the heat-sized rename into ₣A_'s scope (which is GCP/cloud-side path grammar, orthogonal from filesystem moorings).

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | No longer needed — layout is standardized, kit-side constants suffice |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns.

### Final layout

```
rbmm_moorings/                      <- single consumer-owned entry at repo root
  burc.env                          <- BUK regime (authored)
  rbrr.env                          <- RBK repo regime (authored)
  rbrp.env                          <- RBK payor regime (authored)
  tadmor/
    rbrn.env                        <- nameplate regime (authored)
    compose.yml                     <- per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              <- kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                <- base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     <- trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      <- tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

**Execution-context normalization (named goal).** z-launcher.sh `cd`s to the repo root (resolved via `${BASH_SOURCE[0]%/*}/..`) before exec'ing the named launcher. The single `../` in the system lives here, used once at trampoline entry to establish a stable cwd. Two consequences fall out:

- Tabtargets carry zero `../` — each invokes its sibling `z-launcher.sh` directly (`exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" <id> "${@}"`). Today's pervasive per-tabtarget `${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}` pattern dissolves.
- Workbenches always start with cwd = repo root, regardless of where the user typed the tabtarget from. Downstream relative paths resolve identically. Today's `BURD_LAUNCHER` pattern leaves cwd at the user's invocation directory, which the trampoline closes.

This is the load-bearing reason to introduce z-launcher.sh — not just rename consolidation, but execution-context determinism.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **`rbmb_` bootstrap-constants module** — rejected. Original `rbbc_constants.sh` existed to single-source the `.rbk` literal across many files. With `rbmm_moorings/` as the standardized, z-launcher-hardcoded name, no cross-file indirection is needed.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover is cleaner.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).
- **Launchers fold into `tt/` to eliminate `../` entirely** — considered, rejected. Reading: launchers are kit-shipped boilerplate (regenerated by `rbw-MG`), so co-locating them with tabtargets under `tt/_launchers/` would buy zero `../` in the system. Rejected because: (a) the docket places launchers under moorings on the basis that consumers occasionally tune them, (b) the single `../` inside z-launcher is well-localized and load-bearing for the chdir-to-root mechanic above — hiding it inside `tt/` doesn't simplify the model, it just relocates it. The named goal is execution-context normalization, not `../` minimization.

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (for the follow-up rename heat)

The actual filesystem rename is out of scope for this heat (₣A_). It is intended as a separate follow-up heat. The migration scope below is captured here so the follow-up heat has a starting inventory.

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### BCG codification

The trampoline (`tt/z-launcher.sh`) is a **new pattern** — single-point-of-customization for moorings location at the tabtarget layer plus execution-context normalization (cwd = repo root before workbench dispatch), parallel to BURC providing it at the CLI layer (BCG:171-181 dispatch-provided `BURD_*`). Without BCG codification it remains tribal knowledge.

Add a subsection peer to "Dispatch-Provided Directory Variables" titled **"Tabtarget Path Indirection"** stating:

> Tabtargets must not hardcode moorings/config-directory paths. The single tabtarget `tt/z-launcher.sh` resolves the moorings location, normalizes cwd to the repo root, and dispatches to the named launcher; all other tabtargets invoke it with a workbench identifier. This makes the moorings location a single-point-of-customization at the tabtarget layer (parallel to BURC at the CLI layer) and guarantees workbenches start with a deterministic cwd regardless of user invocation directory.

The BCG update lands in the rename heat **alongside** the trampoline introduction, not before — BCG should not document vapor. The pattern also extends BCG's CLI-as-Module-Gateway principle (BCG:44-77) one level further: tabtargets delegate path resolution and execution-context setup through a stable indirection layer rather than hardcoding implementation details.

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete `rbbc_constants.sh` (absorbed into kit code)
- Move `rbmP_laterSecurityAudit.md` -> `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope

- Actually executing the renames (separate follow-up heat, not a pace in this heat)
- Extending moorings pattern to other kits
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored)
- Redesigning the launcher infrastructure beyond introducing `z-launcher.sh`

## Paddock update on wrap

A_ paddock should record (additive to what's already there):
- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The hard-cutover discipline for the moorings rename (when its own heat eventually runs)
- The BCG-codification commitment: rename heat lands trampoline AND adds BCG "Tabtarget Path Indirection" subsection
- The trampoline's named goal: execution-context normalization (chdir to repo root before workbench dispatch), not just rename consolidation

The filesystem rename is NOT a prerequisite to this heat's ₢A_AAE — it's a separate follow-up heat. Paddock should not claim otherwise.

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — relevant sections: lines 21-30 (Load-Bearing Complexity), 44-77 (CLI as Module Gateway), 171-181 (Dispatch-Provided Directory Variables)

**[260425-0811] rough**

## Character

Research completed — decisions recorded. Pace is **railed last** in ₣A_ by deliberate plan: when its turn comes (after AAS through AAM all wrap), reslate this pace from research into a **heat-construction directive** that synthesizes the inventory below into a pace decomposition, then nominates the moorings rename heat. AAJ's wrap closes ₣A_ and the rename heat exists as a freshly-nominated sibling.

This deferred-reslate pattern keeps the research finding tied to the heat that surfaced it (no orphan future-heat drift) without forcing the heat-sized rename into ₣A_'s scope (which is GCP/cloud-side path grammar, orthogonal from filesystem moorings).

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | No longer needed — layout is standardized, kit-side constants suffice |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns.

### Final layout

```
rbmm_moorings/                      <- single consumer-owned entry at repo root
  burc.env                          <- BUK regime (authored)
  rbrr.env                          <- RBK repo regime (authored)
  rbrp.env                          <- RBK payor regime (authored)
  tadmor/
    rbrn.env                        <- nameplate regime (authored)
    compose.yml                     <- per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              <- kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                <- base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     <- trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      <- tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **`rbmb_` bootstrap-constants module** — rejected. Original `rbbc_constants.sh` existed to single-source the `.rbk` literal across many files. With `rbmm_moorings/` as the standardized, z-launcher-hardcoded name, no cross-file indirection is needed.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover is cleaner.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (for the follow-up rename heat)

The actual filesystem rename is out of scope for this heat (₣A_). It is intended as a separate follow-up heat. The migration scope below is captured here so the follow-up heat has a starting inventory.

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### BCG codification

The trampoline (`tt/z-launcher.sh`) is a **new pattern** — single-point-of-customization for moorings location at the tabtarget layer, parallel to BURC providing it at the CLI layer (BCG:171-181 dispatch-provided `BURD_*`). Without BCG codification it remains tribal knowledge.

Add a subsection peer to "Dispatch-Provided Directory Variables" titled **"Tabtarget Path Indirection"** stating:

> Tabtargets must not hardcode moorings/config-directory paths. The single tabtarget `tt/z-launcher.sh` resolves the moorings location and dispatches to the named launcher; all other tabtargets invoke it with a workbench identifier. This makes the moorings location a single-point-of-customization at the tabtarget layer, parallel to BURC providing it at the CLI layer.

The BCG update lands in the rename heat **alongside** the trampoline introduction, not before — BCG should not document vapor. The pattern also extends BCG's CLI-as-Module-Gateway principle (BCG:44-77) one level further: tabtargets delegate path resolution through a stable indirection layer rather than hardcoding implementation details.

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete `rbbc_constants.sh` (absorbed into kit code)
- Move `rbmP_laterSecurityAudit.md` -> `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope

- Actually executing the renames (separate follow-up heat, not a pace in this heat)
- Extending moorings pattern to other kits
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored)
- Redesigning the launcher infrastructure beyond introducing `z-launcher.sh`

## Paddock update on wrap

A_ paddock should record (additive to what's already there):
- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The hard-cutover discipline for the moorings rename (when its own heat eventually runs)
- The BCG-codification commitment: rename heat lands trampoline AND adds BCG "Tabtarget Path Indirection" subsection

The filesystem rename is NOT a prerequisite to this heat's ₢A_AAE — it's a separate follow-up heat. Paddock should not claim otherwise.

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)
- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — relevant sections: lines 21-30 (Load-Bearing Complexity), 44-77 (CLI as Module Gateway), 171-181 (Dispatch-Provided Directory Variables)

**[260423-0903] rough**

## Character

Research completed — decisions recorded. Pace is wrap-ready.

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | No longer needed — layout is standardized, kit-side constants suffice |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns.

### Final layout

```
rbmm_moorings/                      <- single consumer-owned entry at repo root
  burc.env                          <- BUK regime (authored)
  rbrr.env                          <- RBK repo regime (authored)
  rbrp.env                          <- RBK payor regime (authored)
  tadmor/
    rbrn.env                        <- nameplate regime (authored)
    compose.yml                     <- per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              <- kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                <- base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     <- trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      <- tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **`rbmb_` bootstrap-constants module** — rejected. Original `rbbc_constants.sh` existed to single-source the `.rbk` literal across many files. With `rbmm_moorings/` as the standardized, z-launcher-hardcoded name, no cross-file indirection is needed.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover is cleaner.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (for the follow-up rename heat)

The actual filesystem rename is out of scope for this heat (₣A_). It is intended as a separate follow-up heat. The migration scope below is captured here so the follow-up heat has a starting inventory.

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete `rbbc_constants.sh` (absorbed into kit code)
- Move `rbmP_laterSecurityAudit.md` -> `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope

- Actually executing the renames (separate follow-up heat, not a pace in this heat)
- Extending moorings pattern to other kits
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored)
- Redesigning the launcher infrastructure beyond introducing `z-launcher.sh`

## Paddock update on wrap

A_ paddock should record (additive to what's already there):
- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The hard-cutover discipline for the moorings rename (when its own heat eventually runs)

The filesystem rename is NOT a prerequisite to this heat's ₢A_AAE — it's a separate follow-up heat. Paddock should not claim otherwise.

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)

**[260416-0856] rough**

## Character

Research completed — decisions recorded. Pace is wrap-ready. Implementation flows to a follow-up rename pace to be enrolled before `₢A_AAE` (depot regeneration) so the hard cutover aligns with depot regen.

## Decision

### Principle

**Visibility aligns with authorship.** Consumer-authored content is visible at top level; kit machinery is hidden or lives under `Tools/`. Dot-prefix on consumer-authored directories (`.rbk/`, `.buk/`) contradicts the handbook's explicit teaching that learners should read those files. Visibility signals importance to the human reader ([Lobsters — "Use `.config` to store your project configs"](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)).

### Renames

| Current | Becomes | Notes |
|---------|---------|-------|
| `.rbk/` + `.buk/` | `rbmm_moorings/` | Unified consumer configuration umbrella |
| `.buk/launcher.*.sh` | `rbmm_moorings/rbml_launchers/` | All launchers merged, no kit segregation |
| `.buk/users/` | `rbmm_moorings/rbmu_users/` | Remote device profile registry |
| `rbev-vessels/` | `rbmm_moorings/rbmv_vessels/` | Default for `RBRR_VESSEL_DIR`; customer can override |
| `.rbk/rbob_compose.yml` | `Tools/rbk/rbob_compose.yml` | Kit machinery (crucible topology), not consumer tunable |
| `.buk/rbbc_constants.sh` | Eliminated | No longer needed — layout is standardized, kit-side constants suffice |
| `Tools/rbk/rbmP_laterSecurityAudit.md` | `Memos/` | Orphan memo; `rbm*` namespace reclaimed for moorings family |

### Prefix family: `rbm*_`

Gestalt: user-facing content inside moorings. A new concept-prefix family under RB.

| Prefix | Meaning | Location |
|--------|---------|----------|
| `rbmm_` | Moorings umbrella (the directory itself) | `rbmm_moorings/` |
| `rbml_` | Moorings launchers | `rbmm_moorings/rbml_launchers/` |
| `rbmu_` | Moorings users (remote profiles) | `rbmm_moorings/rbmu_users/` |
| `rbmv_` | Moorings vessels | `rbmm_moorings/rbmv_vessels/` |

`rbm` is non-terminal. Future `rbm?_` slots remain open for other user-facing moorings concerns.

### Final layout

```
rbmm_moorings/                      ← single consumer-owned entry at repo root
  burc.env                          ← BUK regime (authored)
  rbrr.env                          ← RBK repo regime (authored)
  rbrp.env                          ← RBK payor regime (authored)
  tadmor/
    rbrn.env                        ← nameplate regime (authored)
    compose.yml                     ← per-nameplate overlay (authored)
  ccyolo/ ...
  pluml/ ...
  srjcl/ ...
  rbml_launchers/
    launcher.rbw_workbench.sh
    launcher_nolog.rbw_workbench.sh
    launcher.jjw_workbench.sh
    launcher.buw_workbench.sh
    launcher.cmw_workbench.sh
    launcher.study_workbench.sh
    launcher.vow_workbench.sh
    launcher.vvw_workbench.sh
    launcher.vslw_workbench.sh
    launcher.apcw_workbench.sh
    launcher.rbtw_workbench.sh
  rbmu_users/
    <BURH-style profiles>
  rbmv_vessels/
    common-sentry-context/
    rbev-bottle-ifrit/              ← kit-shipped, keeps rbev- provenance marker
    rbev-bottle-plantuml/ ...
    <user vessels without rbev- prefix>
Tools/
  rbk/
    rbob_compose.yml                ← base crucible topology (kit-shipped)
    rbob_bottle.sh
    ...
  buk/ ...
tt/
  z-launcher.sh                     ← trampoline, hardcodes ../rbmm_moorings/rbml_launchers/
  rbw-*.sh ...                      ← tabtargets route through z-launcher
```

### Trampoline design

Every tabtarget currently hardcodes `.buk/launcher_*_workbench.sh` — the single most pervasive path reference in the system. Introducing `tt/z-launcher.sh` as a stable indirection means future moorings-directory renames touch exactly one file. Tabtargets call the trampoline with a workbench identifier; the trampoline resolves `../rbmm_moorings/rbml_launchers/launcher.{id}.sh` (or equivalent).

The `rbmm_moorings/` path inside `z-launcher.sh` is hardcoded. Customers adjust it at project start if they choose to rename the moorings directory. That's a deliberate single-point-of-customization.

### Rejected / decided alternatives

- **Kit subdirectories inside moorings** (`rbmm_moorings/rbk/`, `rbmm_moorings/buk/`) — rejected. File prefixes (`burc.`, `rbrr.`, `rbrn.`) already encode kit; subdirs would be redundant nesting, and the kit boundary is a producer concern not a consumer one.
- **Separate `regime/` and `vessels/` top-level dirs** — rejected. Single `rbmm_moorings/` umbrella is cleaner. Vessels are consumer-configured (even if rarely edited), belong with the rest of the configuration surface.
- **`rbmb_` bootstrap-constants module** — rejected. Original `rbbc_constants.sh` existed to single-source the `.rbk` literal across many files. With `rbmm_moorings/` as the standardized, z-launcher-hardcoded name, no cross-file indirection is needed.
- **Bridge period (both old and new paths work transiently)** — rejected. Hard cutover at depot regen is cleaner; the depot regeneration is already a fresh-state moment.
- **Extend moorings to JJK/CMK/VVK** — rejected. `rbmm_moorings/` is RBK+BUK only. JJK uses `.claude/jjm/` for agent-side state (different kettle). Other kits keep their own conventions.
- **Visible `config/` or `regime/`** — rejected. `config/` throws away the project's distinctive vocabulary; `regime/` is too narrow (regimes are formal configurations; moorings also contains bootstrap glue and vessels).

### Prefix coexistence note

Inside `rbmm_moorings/`, regime files retain their original family prefixes (`burc_`, `rbrr_`, `rbrn_`, `rbrp_`). The `rbm*_` family is the moorings-infrastructure prefix — directories and any new moorings-level code. This is a feature, not a conflict: regime file prefixes tell you which kit's regime you're reading; `rbm*_` tells you it's moorings infrastructure.

## Migration scope (for the follow-up rename pace)

### Code references

- ~33 files in `Tools/` reference paths starting with `.rbk/`, `.buk/`, or `rbev-vessels/`
- Every tabtarget in `tt/` hardcodes `.buk/launcher...` — mechanical sed sweep, then tabtargets route through `z-launcher.sh` going forward
- `BURD_LAUNCHER` value in each tabtarget → replaced with workbench identifier passed to trampoline
- `RBRR_VESSEL_DIR` default flips to `rbmm_moorings/rbmv_vessels/`

### Handbook and specs

- Every handbook track that teaches `.rbk/`, `.buk/`, `rbev-vessels/` — update vocabulary and examples
- Specs: RBS0, RBRN, RBSRR, RBSRP, RBSRV, and others that reference these paths
- README.md, CLAUDE.md, consumer CLAUDE.md (`Tools/rbk/vov_veiled/CLAUDE.consumer.md`)
- AsciiDoc linked terms that reference the old dir names

### Infrastructure

- Marshal zero template (currently creates `.rbk/` and `.buk/` for new consumers) — regenerate to create `rbmm_moorings/` layout
- Test fixtures that create these directories
- Any diagrams showing the directory structure

### Cleanup

- Delete `rbbc_constants.sh` (absorbed into kit code)
- Move `rbmP_laterSecurityAudit.md` → `Memos/`
- Move `rbob_compose.yml` from `.rbk/` to `Tools/rbk/`
- Register new `rbm*_` prefixes in CLAUDE.md File Acronym Mappings

### Observed non-issue

**gitignore patterns** — nothing in `.rbk/`, `.buk/`, or `rbev-vessels/` is currently gitignored. Migration doesn't touch ignore files.

## Out of scope

- Actually executing the renames (follow-up pace)
- Extending moorings pattern to other kits
- Eliminating per-nameplate overlay `compose.yml` files (they ARE consumer-authored)
- Redesigning the launcher infrastructure beyond introducing `z-launcher.sh`

## Paddock update on wrap

A_ paddock should record:
- The `rbm*_` prefix family and its gestalt ("user-facing content inside moorings")
- The decision to hard-cutover at depot regen
- The follow-up rename pace as a prerequisite to `₢A_AAE`

## Sources

- [Lobsters — Use `.config` to store your project configs](https://lobste.rs/s/wac58n/use_config_store_your_project_configs)
- [GitHub — Folder Structure Conventions](https://github.com/kriasoft/Folder-Structure-Conventions)
- [monorepo.tools](https://monorepo.tools/)
- [mise — Monorepo Tasks](https://mise.jdx.dev/tasks/monorepo.html)

**[260416-0746] rough**

Drafted from ₢AUAA2 in ₣AU.

## Character

Research pace — survey how other projects name their local configuration directories, then propose a pattern for replacing `.buk/` and `.rbk/` with something visible and unsurprising. The hidden-dot semantic actively hinders discoverability, which conflicts with the handbook's teaching goals. No code changes — just research and a recommendation.

## Docket

### Problem

`.buk/` and `.rbk/` use the Unix dot-prefix convention to hide directories from `ls`. This made sense when they were implementation details, but the handbook explicitly teaches learners to find and read these files (RBRR, RBRN, BURC). Hiding them contradicts discoverability.

### Research

Survey configuration directory naming conventions across well-known projects:

- How do Rust, Node, Python, Go, Ruby projects name their config dirs?
- What do tools like `.github/`, `.vscode/`, `.cargo/`, `.npm/` do — and do any visible-directory conventions exist?
- Are there projects that deliberately chose visible config directories? What pattern did they use?
- What does `config/` vs `conf/` vs `cfg/` vs project-prefixed (`config_buk/`) look like in practice?
- How do monorepo tools handle per-tool config directories?

### Evaluate candidates

For each candidate pattern, assess:
- Discoverability: does `ls` show it without flags?
- Sort position: does it cluster with related dirs or scatter?
- Collision risk: could it conflict with common directory names?
- Familiarity: would a new contributor guess the convention?
- Grep/glob friendliness: does the prefix pattern work well for searching?

### Propose

Recommend a naming pattern with rationale. Include:
- Proposed names for both BUK and RBK config dirs
- Migration considerations (what references need updating)
- Whether the pattern extends to other kits (JJK, CMK, etc.)

### Out of scope

- Actually renaming anything
- Writing migration scripts
- Updating code references

**[260414-0711] rough**

## Character

Research pace — survey how other projects name their local configuration directories, then propose a pattern for replacing `.buk/` and `.rbk/` with something visible and unsurprising. The hidden-dot semantic actively hinders discoverability, which conflicts with the handbook's teaching goals. No code changes — just research and a recommendation.

## Docket

### Problem

`.buk/` and `.rbk/` use the Unix dot-prefix convention to hide directories from `ls`. This made sense when they were implementation details, but the handbook explicitly teaches learners to find and read these files (RBRR, RBRN, BURC). Hiding them contradicts discoverability.

### Research

Survey configuration directory naming conventions across well-known projects:

- How do Rust, Node, Python, Go, Ruby projects name their config dirs?
- What do tools like `.github/`, `.vscode/`, `.cargo/`, `.npm/` do — and do any visible-directory conventions exist?
- Are there projects that deliberately chose visible config directories? What pattern did they use?
- What does `config/` vs `conf/` vs `cfg/` vs project-prefixed (`config_buk/`) look like in practice?
- How do monorepo tools handle per-tool config directories?

### Evaluate candidates

For each candidate pattern, assess:
- Discoverability: does `ls` show it without flags?
- Sort position: does it cluster with related dirs or scatter?
- Collision risk: could it conflict with common directory names?
- Familiarity: would a new contributor guess the convention?
- Grep/glob friendliness: does the prefix pattern work well for searching?

### Propose

Recommend a naming pattern with rationale. Include:
- Proposed names for both BUK and RBK config dirs
- Migration considerations (what references need updating)
- Whether the pattern extends to other kits (JJK, CMK, etc.)

### Out of scope

- Actually renaming anything
- Writing migration scripts
- Updating code references

### ifrit-sortie-enclave-spoof-external (₢A_AAc) [complete]

**[260512-1216] complete**

## Character

Adversarial test for a residual security envelope gap identified during ₢A_AAb. Mount after ₢A_AAb canonical green on both platforms.

## Docket

Add Ifrit sortie(s) covering enclave-internal source spoofing as arbitrary external-routable IPs. Existing `sortie_net_srcip_spoof` covers spoof-as-sentry, spoof-as-allowed-cidr, spoof-as-loopback — none cover spoof-as-external-routable-IP (e.g., Docker Desktop gateway `192.168.65.1`, or any non-enclave non-loopback non-sentry IP that has a route in sentry's routing table).

Under ₢A_AAb's Option E architecture (source-CIDR exclusion at PREROUTING + rp_filter loose), this spoof variant is theoretically reachable: the claimed source is outside enclave CIDR (so source-CIDR exclusion allows it), the spoofed IP has a route (so loose rp_filter allows it), DNAT fires, packet reaches bottle. Strict rp_filter would have blocked this at kernel layer; the architecture trades that for source-IP-flexibility on Docker Desktop delivery.

The new sortie verifies the iptables-layer enforcement holds without rp_filter=1's kernel-layer protection.

If empirically blocked: security envelope is sound; the sortie becomes regression backstop and the architecture's "iptables, not rp_filter, is the load-bearing spoof gate" claim is now empirically validated for this attack class too.

If empirically allowed: architecture needs targeted fix. Candidate mitigations include (a) selectively re-implementing rp_filter-strict-for-enclave-interface at iptables raw table (`iptables -t raw -A PREROUTING -i ENCLAVE_IF ! -s ENCLAVE_CIDR -j DROP`), or (b) adding an explicit "drop non-enclave-source on enclave interface" rule. Either preserves rp_filter loose for legitimate Docker Desktop delivery while restoring the spoof block.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — full architectural context for why this gap exists post-Option E.
- `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs:1042-1102` — `sortie_net_srcip_spoof` as template for the new sortie shape.
- `rbev-vessels/common-ifrit-context/src/rbida_attacks.rs` — adversary registration entry to add.
- Pace ₢A_AAb — the architecture this sortie validates.

**[260512-1025] rough**

## Character

Adversarial test for a residual security envelope gap identified during ₢A_AAb. Mount after ₢A_AAb canonical green on both platforms.

## Docket

Add Ifrit sortie(s) covering enclave-internal source spoofing as arbitrary external-routable IPs. Existing `sortie_net_srcip_spoof` covers spoof-as-sentry, spoof-as-allowed-cidr, spoof-as-loopback — none cover spoof-as-external-routable-IP (e.g., Docker Desktop gateway `192.168.65.1`, or any non-enclave non-loopback non-sentry IP that has a route in sentry's routing table).

Under ₢A_AAb's Option E architecture (source-CIDR exclusion at PREROUTING + rp_filter loose), this spoof variant is theoretically reachable: the claimed source is outside enclave CIDR (so source-CIDR exclusion allows it), the spoofed IP has a route (so loose rp_filter allows it), DNAT fires, packet reaches bottle. Strict rp_filter would have blocked this at kernel layer; the architecture trades that for source-IP-flexibility on Docker Desktop delivery.

The new sortie verifies the iptables-layer enforcement holds without rp_filter=1's kernel-layer protection.

If empirically blocked: security envelope is sound; the sortie becomes regression backstop and the architecture's "iptables, not rp_filter, is the load-bearing spoof gate" claim is now empirically validated for this attack class too.

If empirically allowed: architecture needs targeted fix. Candidate mitigations include (a) selectively re-implementing rp_filter-strict-for-enclave-interface at iptables raw table (`iptables -t raw -A PREROUTING -i ENCLAVE_IF ! -s ENCLAVE_CIDR -j DROP`), or (b) adding an explicit "drop non-enclave-source on enclave interface" rule. Either preserves rp_filter loose for legitimate Docker Desktop delivery while restoring the spoof block.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — full architectural context for why this gap exists post-Option E.
- `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs:1042-1102` — `sortie_net_srcip_spoof` as template for the new sortie shape.
- `rbev-vessels/common-ifrit-context/src/rbida_attacks.rs` — adversary registration entry to add.
- Pace ₢A_AAb — the architecture this sortie validates.

### sentry-config-presence-assertions (₢A_AAn) [complete]

**[260513-0737] complete**

## Character

Structural backstops for the load-bearing iptables clauses and kernel sysctls identified by ₢A_AAb's iteration arc. Each assertion is a fast structural check that catches a specific class of regression — the kind where a future architect (human or agent) decides a defensive clause is "redundant" and removes it. The current adversarial Ifrit suite catches behavioral regressions; these add fast structural-presence checks that fail loudly before the behavioral suite even has to run.

Mount only after ₢A_AAb canonical pristine pass.

## Docket

Add structural/configuration-presence assertion tests covering the load-bearing clauses the anchor memo's Lessons captured section identifies. Each assertion targets a specific class of regression the iteration arc taught us to defend against.

Venue (mount-time decision): probably new crucible cases under `Tools/rbk/rbtd/src/rbtdrc_*.rs` (since they probe live container state, not file content). Alternative venue: new Ifrit sortie family. Inspect existing patterns and choose; document the decision in the implementation.

**Assertions to add:**

1. **rp_filter sysctl assertion.** Inside sentry's network namespace when `RBRN_ENTRY_MODE=enabled`: read `/proc/sys/net/ipv4/conf/all/rp_filter` and assert value is `2` (loose). Catches regression where ₢A_AAd's spec edits or a future cleanup re-introduces strict mode (RBSAX's pre-existing commitment); strict mode silently breaks Linux Docker Engine delivery per the cerebro empirical evidence.

2. **PREROUTING per-IP RETURN structural assertion.** Parse sentry's `iptables -t nat -L PREROUTING -n` output; assert two RETURN rules for `${RBRN_ENCLAVE_SENTRY_IP}` and `${RBRN_ENCLAVE_BOTTLE_IP}` are present and positioned BEFORE the DNAT rule. Catches regression where someone "simplifies" back to whole-CIDR exclusion (which re-breaks Linux per cerebro evidence) or removes the RETURN clauses entirely (which would let enclave-internal sources reach the bottle via entry-port DNAT, violating `net-dnat-entry-reflection`).

3. **POSTROUTING MASQUERADE structural assertion.** Parse sentry's `iptables -t nat -L POSTROUTING -n` output; assert MASQUERADE rule on the entry-port path (matching `-d ${RBRN_ENCLAVE_BOTTLE_IP} --dport ${RBRN_ENTRY_PORT_ENCLAVE}`) is present. Complements ₢A_AAk's behavioral check (bottle sees sentry's enclave IP); ₢A_AAk tests the property, this tests the implementation. Both should fail under MASQUERADE removal, but they fail in different ways — useful for diagnosis.

4. **RBM-FORWARD ESTABLISHED,RELATED assertion.** Parse sentry's `iptables -L RBM-FORWARD -n` output; assert `-m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT` rule is present and positioned EARLY in the chain (before per-destination access-mode filtering). Catches regression where this baseline rule is removed or repositioned late; return paths through sentry would break asymmetrically.

**Optional fifth assertion (lower priority):**

5. **NET_ADMIN capability drop assertion.** From inside the bottle, attempt `iptables -L` (or equivalent capability probe); assert operation-not-permitted. Catches regression where bottle's `cap_drop: [NET_ADMIN]` in `rbob_compose.yml` is reverted (the namespace-sharing security hygiene Round 4 recommended). Lower priority because the failure mode is "compromised bottle could alter pentacle/sentry rules," which is escalation rather than direct invariant violation.

**Implementation discipline:**

- Each assertion should produce a clear failure message naming which clause regressed and pointing at the anchor memo section explaining why the clause is load-bearing. The point of these tests is to catch regressions early AND educate the agent who triggered them about why their change broke things.
- Assertions can run as part of the existing tadmor fixture (extending its 54-case suite) or as a new dedicated fixture. Operator picks at mount.
- These are CHEAP to run (read /proc, parse iptables output). They should be fast-fail early-warning, not slow gauntlet additions.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — Lessons captured section identifies each load-bearing clause these assertions backstop; Fix Recipe section names the exact form expected.
- `rbev-vessels/common-sentry-context/rbjs_sentry.sh` — the file these assertions verify the state of (post-startup).
- `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs` — pattern reference if using Ifrit framework.
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — pattern reference if using crucible test cases.
- ₢A_AAk (slated) — behavioral counterpart to assertion 3.

**[260512-1207] rough**

## Character

Structural backstops for the load-bearing iptables clauses and kernel sysctls identified by ₢A_AAb's iteration arc. Each assertion is a fast structural check that catches a specific class of regression — the kind where a future architect (human or agent) decides a defensive clause is "redundant" and removes it. The current adversarial Ifrit suite catches behavioral regressions; these add fast structural-presence checks that fail loudly before the behavioral suite even has to run.

Mount only after ₢A_AAb canonical pristine pass.

## Docket

Add structural/configuration-presence assertion tests covering the load-bearing clauses the anchor memo's Lessons captured section identifies. Each assertion targets a specific class of regression the iteration arc taught us to defend against.

Venue (mount-time decision): probably new crucible cases under `Tools/rbk/rbtd/src/rbtdrc_*.rs` (since they probe live container state, not file content). Alternative venue: new Ifrit sortie family. Inspect existing patterns and choose; document the decision in the implementation.

**Assertions to add:**

1. **rp_filter sysctl assertion.** Inside sentry's network namespace when `RBRN_ENTRY_MODE=enabled`: read `/proc/sys/net/ipv4/conf/all/rp_filter` and assert value is `2` (loose). Catches regression where ₢A_AAd's spec edits or a future cleanup re-introduces strict mode (RBSAX's pre-existing commitment); strict mode silently breaks Linux Docker Engine delivery per the cerebro empirical evidence.

2. **PREROUTING per-IP RETURN structural assertion.** Parse sentry's `iptables -t nat -L PREROUTING -n` output; assert two RETURN rules for `${RBRN_ENCLAVE_SENTRY_IP}` and `${RBRN_ENCLAVE_BOTTLE_IP}` are present and positioned BEFORE the DNAT rule. Catches regression where someone "simplifies" back to whole-CIDR exclusion (which re-breaks Linux per cerebro evidence) or removes the RETURN clauses entirely (which would let enclave-internal sources reach the bottle via entry-port DNAT, violating `net-dnat-entry-reflection`).

3. **POSTROUTING MASQUERADE structural assertion.** Parse sentry's `iptables -t nat -L POSTROUTING -n` output; assert MASQUERADE rule on the entry-port path (matching `-d ${RBRN_ENCLAVE_BOTTLE_IP} --dport ${RBRN_ENTRY_PORT_ENCLAVE}`) is present. Complements ₢A_AAk's behavioral check (bottle sees sentry's enclave IP); ₢A_AAk tests the property, this tests the implementation. Both should fail under MASQUERADE removal, but they fail in different ways — useful for diagnosis.

4. **RBM-FORWARD ESTABLISHED,RELATED assertion.** Parse sentry's `iptables -L RBM-FORWARD -n` output; assert `-m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT` rule is present and positioned EARLY in the chain (before per-destination access-mode filtering). Catches regression where this baseline rule is removed or repositioned late; return paths through sentry would break asymmetrically.

**Optional fifth assertion (lower priority):**

5. **NET_ADMIN capability drop assertion.** From inside the bottle, attempt `iptables -L` (or equivalent capability probe); assert operation-not-permitted. Catches regression where bottle's `cap_drop: [NET_ADMIN]` in `rbob_compose.yml` is reverted (the namespace-sharing security hygiene Round 4 recommended). Lower priority because the failure mode is "compromised bottle could alter pentacle/sentry rules," which is escalation rather than direct invariant violation.

**Implementation discipline:**

- Each assertion should produce a clear failure message naming which clause regressed and pointing at the anchor memo section explaining why the clause is load-bearing. The point of these tests is to catch regressions early AND educate the agent who triggered them about why their change broke things.
- Assertions can run as part of the existing tadmor fixture (extending its 54-case suite) or as a new dedicated fixture. Operator picks at mount.
- These are CHEAP to run (read /proc, parse iptables output). They should be fast-fail early-warning, not slow gauntlet additions.

### References

- `Memos/memo-20260512-sentry-entry-port-repair-anchor.md` — Lessons captured section identifies each load-bearing clause these assertions backstop; Fix Recipe section names the exact form expected.
- `rbev-vessels/common-sentry-context/rbjs_sentry.sh` — the file these assertions verify the state of (post-startup).
- `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs` — pattern reference if using Ifrit framework.
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` — pattern reference if using crucible test cases.
- ₢A_AAk (slated) — behavioral counterpart to assertion 3.

### aak-acceptance-tP-macos-linux (₢A_AAP) [abandoned]

**[260514-1129] abandoned**

## Character

Two-platform AAK acceptance gate via the gauntlet. Mechanical
execution; a failure here is a real AAK-cutover bug, not exploration.

## Docket

Run `tt/rbw-tP.QualifyPristine.sh` on macOS and linux, each on its
own branch. The gauntlet handles depot regen + qualification in one
pass — no separate AAE step is needed if both runs go green.

- One branch per platform — keeps each run's canonical-depot
  commits isolated from main
- Sequential, never concurrent — canonical-establish picks a fresh
  moniker but two concurrent runs could still collide on
  station/GAR identifiers
- Triage on failure: grammar (fix-inline), contract (fix-inline),
  or spinoff pace with explicit handoff

### References

- `Tools/rbk/rbq_Qualify.sh:187-200` — pristine entry point
- `Tools/rbk/rbtd/rbte_engine.sh:57-69` — gauntlet suite contents
- `Tools/rbk/rbtd/src/rbtdrk_canonical.rs:382-501` — canonical-
  establish performs the depot regen

**[260511-1552] rough**

## Character

Two-platform AAK acceptance gate via the gauntlet. Mechanical
execution; a failure here is a real AAK-cutover bug, not exploration.

## Docket

Run `tt/rbw-tP.QualifyPristine.sh` on macOS and linux, each on its
own branch. The gauntlet handles depot regen + qualification in one
pass — no separate AAE step is needed if both runs go green.

- One branch per platform — keeps each run's canonical-depot
  commits isolated from main
- Sequential, never concurrent — canonical-establish picks a fresh
  moniker but two concurrent runs could still collide on
  station/GAR identifiers
- Triage on failure: grammar (fix-inline), contract (fix-inline),
  or spinoff pace with explicit handoff

### References

- `Tools/rbk/rbq_Qualify.sh:187-200` — pristine entry point
- `Tools/rbk/rbtd/rbte_engine.sh:57-69` — gauntlet suite contents
- `Tools/rbk/rbtd/src/rbtdrk_canonical.rs:382-501` — canonical-
  establish performs the depot regen

**[260511-1552] rough**

## Character

Live-infrastructure verification. The AAK migration was all grammar until now — bash string construction, Rust constants, spec prose, fast-suite regime checks. None of it has touched real GAR, real Cloud Build, real containers. This pace is the first-time burn-in: exercise every code path that matters, against the current depot (mixed-shape content is fine — no backward compat obligation), and surface any gaps before `₢A_AAE` (depot regen, irreversible).

Posture: investigative. Expect some surprises. Fix what's small inline; surface what's big for a follow-up pace. Don't force green — honest failures here are the whole point of running.

## Docket

Exercise the full AAK-migrated code surface against the current depot. Work through the tiers in order; escalate on failure.

### Tier 1 — Automated suites

- `tt/rbtd-s.TestSuite.service.sh` — 80 cases (fast + access-probe + four-mode foundry integration). Requires GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` and `RBDC_RETRIEVER_RBRA_FILE`.
- `tt/rbtd-s.TestSuite.crucible.sh` — 117 cases (fast + tadmor-security + srjcl-jupyter + pluml-diagram). Requires container runtime.

Sequential only — fixtures share regime state and container namespaces.

### Tier 2 — Hand-run producer paths

Once suites are green (or triaged), exercise the director-side producer lifecycle against a real vessel:

- **Inscribe** a reliquary: `tt/rbw-dI.DirectorInscribesReliquary.sh` — verifies `_RBGN_CLOUD_PREFIX` substitution threads correctly into the inscribe builder; check resulting GAR shape under `<prefix>reliquaries/<date>/<tool>:<date>`.
- **Conjure** busybox: `tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-busybox` — hits the heavy rewrite: `rbfd_FoundryDirectorBuild.sh` (conjure jq block, pouch construction, per-platform attest sweep, fact emissions) and the Cloud Build step scripts (`rbgjb/`, `rbgja/`). Watch the Cloud Build console — URL construction failures surface as build-submission errors.
- **Enshrine** a base (if any vessel has `RBRV_IMAGE_ORIGIN` set): exercises `rbgje01-enshrine-copy.sh` and the anchor-as-path-segment shift.
- **Graft** a local image: `tt/rbw-fO.*` against `rbev-graft-demo` — exercises `rbfd` graft path + `rbgja01-graft-push.sh` if applicable.

After each: confirm artifact layout under `<prefix>hallmarks/<hallmark>/` has `image`, `vouch`, `pouch`/none-for-graft, `about`, `attest`, `diags` as sibling basenames — via `tt/rbw-ir.DirectorRekonsImages.sh` (raw GAR tag listing).

### Tier 3 — Hand-run consumer paths

- **Tally**: `tt/rbw-ft.RetrieverTalliesHallmarks.sh` — confirms tally's new basename-enumeration classification works against mixed-shape depot content. Look for the vouched/pending/incomplete counts and per-hallmark listings; any hallmark misclassified surfaces here.
- **Plumb compact + full**: `tt/rbw-fpc.*` and `tt/rbw-fpf.*` on a vouched hallmark — confirms metadata fetch from new `<hallmark>/vouch` and `<hallmark>/about` packages.
- **Summon**: `tt/rbw-fs.RetrieverSummonsHallmark.sh` — confirms retriever pull path against new layout.
- **Charge tadmor**: `tt/rbw-cC.Charge.tadmor.sh` — compose.yml interpolation against new basename shape. Sentry + pentacle + bottle come up cleanly or they don't.
- **Quench tadmor**: `tt/rbw-cQ.Quench.tadmor.sh` — tests cleanup path.

### Tier 4 — Abjure

- **Abjure a freshly-conjured hallmark**: `tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-vessels/rbev-busybox «hallmark»` — exercises the new enumerate-then-delete via GAR REST `/packages` (the biggest algorithmic rewrite in Notch 2b). Confirm all 5-6 basename packages deleted cleanly; confirm `tt/rbw-ir` shows empty for that hallmark.
- **Abjure a pre-AAK hallmark** (if any exist with old-shape content): exercises enumerate-then-delete's topology-agnosticism. Should Just Work because it enumerates the subtree — but worth confirming rather than assuming.

### Triage protocol

On failure:
1. Record the exact tabtarget invocation, stderr, and GAR state (screenshot or `rbw-ir` listing)
2. Classify: **grammar bug** (stale path or tag in bash/Rust), **contract bug** (fact emission mismatch), **integration bug** (new code is right but an adjacent old assumption broke), **AAL-scope** (signature-related, legitimate to defer), or **AAE-scope** (depot-state related, legitimate to defer)
3. Fix grammar and contract bugs inline in this pace — they're part of cutover completion
4. Integration bugs: assess scope; small means fix here, large means spinoff pace with explicit handoff docket
5. AAL/AAE-scope: document and move on; these are out of this pace's contract

### Verification

- Service suite green or triaged
- Crucible suite green or triaged
- One full ordain → summon → charge → quench cycle passes against current depot
- One abjure cycle passes (delete → confirm empty)
- Tally/plumb/rekon reflect new layout correctly on at least one vessel's hallmarks
- No stale-grammar failure modes surface (if they do, they return to AAK for a Notch 5; this pace is consumer-side)

### Out of scope

- Depot regen (`₢A_AAE`) — explicitly gates on this pace completing
- Tabtarget signature simplification (`₢A_AAL`) — orthogonal to path grammar
- Filesystem moorings rename (`₢A_AAJ`) — separate heat
- Any fact-contract harmonization (the `ark_stem` includes-prefix vs. wrest locator excludes-prefix asymmetry flagged in AAK close summary) — surface-level tax, not broken

### References

- `₢A_AAK` — parent migration, 4 notches landed: a9e95201 (1 + 2a), 6c362750 (2b), 93c651f2 (specs), 739b4a35 (Rust catchup)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — producer; per-mode fact emission now uniform
- `Tools/rbk/rbfl_FoundryLedger.sh:264–359` — abjure new algorithm
- `Tools/rbk/rbgl_GarLayout.sh` — root constants contract
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs:2210–2414` — four-mode fixture
- `.rbk/rbob_compose.yml:23,57,77` — compose interpolation

**[260423-1304] rough**

## Character

Live-infrastructure verification. The AAK migration was all grammar until now — bash string construction, Rust constants, spec prose, fast-suite regime checks. None of it has touched real GAR, real Cloud Build, real containers. This pace is the first-time burn-in: exercise every code path that matters, against the current depot (mixed-shape content is fine — no backward compat obligation), and surface any gaps before `₢A_AAE` (depot regen, irreversible).

Posture: investigative. Expect some surprises. Fix what's small inline; surface what's big for a follow-up pace. Don't force green — honest failures here are the whole point of running.

## Docket

Exercise the full AAK-migrated code surface against the current depot. Work through the tiers in order; escalate on failure.

### Tier 1 — Automated suites

- `tt/rbtd-s.TestSuite.service.sh` — 80 cases (fast + access-probe + four-mode foundry integration). Requires GCP credentials at `RBDC_DIRECTOR_RBRA_FILE` and `RBDC_RETRIEVER_RBRA_FILE`.
- `tt/rbtd-s.TestSuite.crucible.sh` — 117 cases (fast + tadmor-security + srjcl-jupyter + pluml-diagram). Requires container runtime.

Sequential only — fixtures share regime state and container namespaces.

### Tier 2 — Hand-run producer paths

Once suites are green (or triaged), exercise the director-side producer lifecycle against a real vessel:

- **Inscribe** a reliquary: `tt/rbw-dI.DirectorInscribesReliquary.sh` — verifies `_RBGN_CLOUD_PREFIX` substitution threads correctly into the inscribe builder; check resulting GAR shape under `<prefix>reliquaries/<date>/<tool>:<date>`.
- **Conjure** busybox: `tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-busybox` — hits the heavy rewrite: `rbfd_FoundryDirectorBuild.sh` (conjure jq block, pouch construction, per-platform attest sweep, fact emissions) and the Cloud Build step scripts (`rbgjb/`, `rbgja/`). Watch the Cloud Build console — URL construction failures surface as build-submission errors.
- **Enshrine** a base (if any vessel has `RBRV_IMAGE_ORIGIN` set): exercises `rbgje01-enshrine-copy.sh` and the anchor-as-path-segment shift.
- **Graft** a local image: `tt/rbw-fO.*` against `rbev-graft-demo` — exercises `rbfd` graft path + `rbgja01-graft-push.sh` if applicable.

After each: confirm artifact layout under `<prefix>hallmarks/<hallmark>/` has `image`, `vouch`, `pouch`/none-for-graft, `about`, `attest`, `diags` as sibling basenames — via `tt/rbw-ir.DirectorRekonsImages.sh` (raw GAR tag listing).

### Tier 3 — Hand-run consumer paths

- **Tally**: `tt/rbw-ft.RetrieverTalliesHallmarks.sh` — confirms tally's new basename-enumeration classification works against mixed-shape depot content. Look for the vouched/pending/incomplete counts and per-hallmark listings; any hallmark misclassified surfaces here.
- **Plumb compact + full**: `tt/rbw-fpc.*` and `tt/rbw-fpf.*` on a vouched hallmark — confirms metadata fetch from new `<hallmark>/vouch` and `<hallmark>/about` packages.
- **Summon**: `tt/rbw-fs.RetrieverSummonsHallmark.sh` — confirms retriever pull path against new layout.
- **Charge tadmor**: `tt/rbw-cC.Charge.tadmor.sh` — compose.yml interpolation against new basename shape. Sentry + pentacle + bottle come up cleanly or they don't.
- **Quench tadmor**: `tt/rbw-cQ.Quench.tadmor.sh` — tests cleanup path.

### Tier 4 — Abjure

- **Abjure a freshly-conjured hallmark**: `tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-vessels/rbev-busybox «hallmark»` — exercises the new enumerate-then-delete via GAR REST `/packages` (the biggest algorithmic rewrite in Notch 2b). Confirm all 5-6 basename packages deleted cleanly; confirm `tt/rbw-ir` shows empty for that hallmark.
- **Abjure a pre-AAK hallmark** (if any exist with old-shape content): exercises enumerate-then-delete's topology-agnosticism. Should Just Work because it enumerates the subtree — but worth confirming rather than assuming.

### Triage protocol

On failure:
1. Record the exact tabtarget invocation, stderr, and GAR state (screenshot or `rbw-ir` listing)
2. Classify: **grammar bug** (stale path or tag in bash/Rust), **contract bug** (fact emission mismatch), **integration bug** (new code is right but an adjacent old assumption broke), **AAL-scope** (signature-related, legitimate to defer), or **AAE-scope** (depot-state related, legitimate to defer)
3. Fix grammar and contract bugs inline in this pace — they're part of cutover completion
4. Integration bugs: assess scope; small means fix here, large means spinoff pace with explicit handoff docket
5. AAL/AAE-scope: document and move on; these are out of this pace's contract

### Verification

- Service suite green or triaged
- Crucible suite green or triaged
- One full ordain → summon → charge → quench cycle passes against current depot
- One abjure cycle passes (delete → confirm empty)
- Tally/plumb/rekon reflect new layout correctly on at least one vessel's hallmarks
- No stale-grammar failure modes surface (if they do, they return to AAK for a Notch 5; this pace is consumer-side)

### Out of scope

- Depot regen (`₢A_AAE`) — explicitly gates on this pace completing
- Tabtarget signature simplification (`₢A_AAL`) — orthogonal to path grammar
- Filesystem moorings rename (`₢A_AAJ`) — separate heat
- Any fact-contract harmonization (the `ark_stem` includes-prefix vs. wrest locator excludes-prefix asymmetry flagged in AAK close summary) — surface-level tax, not broken

### References

- `₢A_AAK` — parent migration, 4 notches landed: a9e95201 (1 + 2a), 6c362750 (2b), 93c651f2 (specs), 739b4a35 (Rust catchup)
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — producer; per-mode fact emission now uniform
- `Tools/rbk/rbfl_FoundryLedger.sh:264–359` — abjure new algorithm
- `Tools/rbk/rbgl_GarLayout.sh` — root constants contract
- `Tools/rbk/rbtd/src/rbtdrc_crucible.rs:2210–2414` — four-mode fixture
- `.rbk/rbob_compose.yml:23,57,77` — compose interpolation

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 o depot-fact-files-namespaced-by-cloud-prefix
  2 b sentry-entry-port-dnat-forward-rescope
  3 d rbs-spec-deltas-post-sentry-fix
  4 Q rekon-hallmark-catalog
  5 R jettison-locator-grammar
  6 I rename-rbap-to-rbgv
  7 A rbrr-image-and-runtime-prefix-variables
  8 B derived-constants-and-consumer-wiring
  9 C marshal-release-zeroes-prefixes
  10 K gar-categorical-layout-migration
  11 O tally-hallmark-enumeration-rewrite
  12 L tabtarget-vessel-param-removal
  13 D rbro-payor-subdirectory-migration
  14 T four-mode-case-split
  15 S theurge-coverage-catchup-aak-aal-aao
  16 U cloudbuild-ark-basename-passthrough
  17 F run-complete-test-suite
  18 a rbgo-oauth-retry-and-diagnostics-completion
  19 c ifrit-sortie-enclave-spoof-external
  20 n sentry-config-presence-assertions
  21 P aak-acceptance-tP-macos-linux

obdQRIABCKOLDTSUFacnP
·········x·x·xx···xx· rbtdrc_crucible.rs
·x·············xx·xxx rbrn.env
·······x·x····xx····x rbfd_FoundryDirectorBuild.sh
····x··x·xx···x······ rbfl_FoundryLedger.sh
··x···x··xx·x········ RBS0-SpecTop.adoc
·······x·xx····x····· rbfv_FoundryVerify.sh
·······x·xx···x······ rbfc_FoundryCore.sh
·x·····x·x·····x····· rbob_bottle.sh
··············x·x···x rbrv.env
·········x····xx····· rbgja01-discover-platforms.py
·······x·x·x········· rbfr_FoundryRetriever.sh
······x·x···x········ RBSRR-RegimeRepo.adoc
·x·····x·x··········· rbob_compose.yml
x········x·······x··· rbgc_Constants.sh
x·····x·x············ rbrr.env
··············x··x··· rbgo_OAuth.sh
·············xx······ rbtdrm_manifest.rs
··········x·x········ rbcc_Constants.sh
·········x·····x····· rbgja02-syft-per-platform.sh, rbgja04-assemble-push-about.sh, rbgjb03-buildx-push-image.sh, rbgjb04-per-platform-pullback.sh, rbgjb05-push-per-platform.sh, rbgjb06-push-diags.sh, rbgjv02-verify-provenance.py, rbgjv03-assemble-push-vouch.sh
·········x··x········ RBSGS-GettingStarted.adoc
·········xx·········· RBSCL-consecration_tally.adoc, RBSDV-director_vouch.adoc
········x···x········ rblm_cli.sh
·······x·x··········· rbgje01-enshrine-copy.sh, rbrn_cli.sh
······x·x············ rbrr_regime.sh
·x··················x memo-20260512-srjcl-port-publish-regression.md
····················x .gitignore
··················x·· rbida_attacks.rs, rbida_sorties.rs
·················x··· rbgu_Utility.sh
················x···· memo-20260513-iam-propagation-race-director-invest-gar.md
··············x······ main.rs, rbgjm01-mirror-image.sh, rbm-abstract-drawio.svg, rbtd-r.Run.batch-vouch.sh, rbtd-s.SingleCase.batch-vouch.sh, rbtdrf_fast.rs, rbtdtm_manifest.rs, rbte_engine.sh
·············x······· rbtd-s.SingleCase.four-mode.sh
············x········ CLAUDE.consumer.md, RBSPI-payor_install.adoc, RBSPR-payor_refresh.adoc, RBSRO-RegimeOauth.adoc, rbdc_DerivedConstants.sh, rbhpr_refresh.sh
·········x··········· RBSAA-ark_abjure.adoc, RBSAB-ark_about.adoc, RBSAC-ark_conjure.adoc, RBSAE-ark_enshrine.adoc, RBSAG-ark_graft.adoc, RBSAK-ark_kludge.adoc, RBSAP-ark_plumb.adoc, RBSAS-ark_summon.adoc, RBSAV-ark_vouch.adoc, RBSCB-CloudBuildPosture.adoc, RBSCC-crucible_charge.adoc, RBSDI-depot_inscribe.adoc, RBSIJ-image_jettison.adoc, RBSTB-trigger_build.adoc, rbfc_cli.sh, rbfd_cli.sh, rbfl_cli.sh, rbfr_cli.sh, rbfv_cli.sh, rbgja03-build-info-per-platform.py, rbgji01-inscribe-mirror.sh, rbgl_GarLayout.sh, rbob_cli.sh
···x················· rbk-claude-tabtarget-context.md, rbz_zipper.sh
··x·················· RBSAX-access_setup.adoc, RBSBL-bottle_launch.adoc, RBSDS-dns_step.adoc, RBSII-iptables_init.adoc, RBSIP-ifrit_pentester.adoc, RBSPT-port_setup.adoc, RBSSS-sentry_start.adoc
·x··················· Dockerfile, memo-20260512-sentry-entry-port-repair-anchor.md, rbjs_sentry.sh
x···················· rbgp_Payor.sh, rbtdrk_canonical.rs, rbtdrp_pristine.rs

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 167 commits)

  1 b sentry-entry-port-dnat-forward-rescope
  2 c ifrit-sortie-enclave-spoof-external
  3 d rbs-spec-deltas-post-sentry-fix
  4 n sentry-config-presence-assertions
  5 o depot-fact-files-namespaced-by-cloud-prefix
  6 F run-complete-test-suite

123456789abcdefghijklmnopqrstuvwxyz
xxxxxx·········x···················  b  7c
·········xx·xx·····················  c  4c
················x·x··xx············  d  4c
·················x·xx··x···········  n  4c
··························xxx······  o  3c
·····························xxxx··  F  4c
```

## Steeplechase

### 2026-05-14 11:29 - Heat - T

aak-acceptance-tP-macos-linux

### 2026-05-14 11:29 - Heat - T

research-config-directory-naming

### 2026-05-13 13:24 - ₢A_AAF - W

macOS validation against the freshly-cut canest2bhm100003 substrate ran end-to-end green: 261 cases across fast suite (98), service suite (104), and tadmor security fixture (59), with sentry+bottle vessels locally kludged in between. The pace's docket assumed it would walk a substrate that ₢A_AAo had handed off whole; the first service-suite run surfaced that the swap-and-defer in ₢A_AAo had deferred more than just the test cycle — five canonical post-levy steps from the Payor and Governor handbooks were skipped: mantle governor, invest retriever, invest director, inscribe reliquary, yoke vessels. This pace executed all five inline as setup, including a divest+re-invest recovery from an HTTP 403 IAM-propagation race on director invest's GAR getIamPolicy call (governor SA had been minted with roles/owner ~30s prior — project-scope and SA-scope IAM caches were warm, AR-resource-scope cache was not). The race is captured in Memos/memo-20260513-iam-propagation-race-director-invest-gar.md and the structural repair is tracked under ₢BBABF (per the linter-added cross-ref). Three tracked-tree commits landed under this pace: 80257e11 (yoke 9 vessels to r260513125123 + race memo), 2c1bbbc2 (sentry hallmark k260513131344-9798a40a into tadmor nameplate), b233bdee (bottle hallmark k260513131436-2c1bbbc2 into tadmor nameplate). RBRA credential files for all three roles (governor, retriever, director) now reference canest2bhm100003 — these live outside the tracked tree in ../station-files/secrets/ and were rotated in passing. Linux coverage stays deferred to AAP per the docket's explicit out-of-scope clause; srjcl, pluml, moriah likewise out of scope. Operator question worth carrying forward (not resolved here): whether 'depot regen requires SA chain + reliquary reconstruction' should be captured as a documented post-levy procedure pace, since the swap-and-defer convention loses these steps silently.

### 2026-05-13 13:14 - ₢A_AAF - n

Drive freshly-kludged bottle hallmark k260513131436-2c1bbbc2 into tadmor's nameplate. Local build of rbev-bottle-ifrit-tether against the post-yoke reliquary; build was fully cached (rust:slim-bookworm base, ifrit binary cargo build, runtime apt layers all hit cache from the prior k260513072351 build), so the new hallmark differs only in timestamp and content-hash digest from the predecessor. Tadmor's nameplate now points at this commit's bottle and the prior commit's sentry — both vessels stand ready for the tadmor security fixture next in ₢A_AAF's docket sequence.

### 2026-05-13 13:14 - ₢A_AAF - n

Drive freshly-kludged sentry hallmark k260513131344-9798a40a into tadmor's nameplate. Local build of rbev-sentry-deb-tether against the post-yoke reliquary; build was fully cached (Dockerfile + COPY layers from prior k260513072313 build), so the new hallmark differs only in timestamp and content-hash digest from the predecessor. Required as a separate commit because rbob_kludge_bottle's preflight refuses to run with an unstaged tree, and the tadmor bottle kludge is the next step in ₢A_AAF's tadmor-fixture sequence.

### 2026-05-13 13:09 - ₢A_AAF - n

Reconstruct SA chain and reliquary on canest2bhm100003 after the swap-and-defer in ₢A_AAo left the new depot bare. The depot regen via rbw-dL provisioned the project + GAR repo + GCS bucket but did not chain forward to governor mantle, retriever invest, director invest, reliquary inscribe, or vessel yoke — all canonical post-levy steps per the Payor and Governor handbooks. Service suite's first run after the swap surfaced this as rbtdrc_jwt_governor failing with 'Invalid grant: account not found' (governor SA literally absent on 100003 since 100002's governor was destroyed during rbw-dU teardown), then rbtdrc_hallmark_lifecycle failing with 'Registry preflight failed — 6 of 6 reliquary tool images missing from GAR' (reliquary stamp r260512130920 yoked into all 9 rbrv.env files but never inscribed onto 100003). Repair sequence executed: (1) rbw-aM minted governor-202605131246, (2) rbw-arI t3r minted retriever, (3) rbw-adI t3d minted director — first attempt failed with HTTP 403 on artifactregistry.repositories.getIamPolicy ~30s after governor mantle (IAM propagation race documented in the memo committed here), divest+re-invest cleared it, (4) rbw-dI inscribed reliquary r260513125123 (5min 6s Cloud Build, 6 tool images: gcloud, docker, alpine, syft, binfmt, skopeo), (5) rbw-dY yoked the new stamp across all 9 vessel rbrv.env files. RBRA credential files (governor, retriever, director) live in ../station-files/secrets/ outside the tracked tree, so this commit captures only the rbrv.env churn from yoke + the propagation-race memo. Service suite re-run is in flight in the background to validate the reconstructed substrate. Memo characterizes the GAR-IAM 403 as an asymmetric propagation lag — same governor token successfully wrote project-scope and SA-scope IAM seconds before, but AR-resource-scope IAM cache hadn't caught up; the script's existing SA-key propagation retry doesn't cover this call site. Out-of-scope repair vectors (propagation-retry wrapper around getIamPolicy/setIamPolicy on AR, or a deliberate post-mantle wait covering AR-IAM cache) noted for future work.

### 2026-05-13 12:18 - ₢A_AAo - W

Pace done in two heads. Head 1 (structural, notch 3ba22d5d, last session): depot fact files moved to <cloud_prefix>/<moniker>.depot layout — writer (zrbgp_depot_state_emit), renderer (rbgp_depot_list), and theurge readers (rbtdrp_pristine, rbtdrk_canonical) all picked up the cloud_prefix dimension; collision between cancbhl-d-canest2bhm100002 and cancbhm-d-canest2bhm100002 became structurally impossible. Head 2 (data-state, notch 4b49b92d, this session): levied fresh cancbhl-d-canest2bhm100003 — first depot in this regime to come up with ₢BAAAG's rb-delete-untagged retention policy (UNTAGGED, 86400s, cleanupPolicyDryRun: false) baked into the create-repo body — and unmade cancbhl-d-canest2bhm100002 (governor SA removed, billing unlinked, project to DELETE_REQUESTED). RBRR_DEPOT_MONIKER bumped 100002 → 100003. Posture change vs as-written docket: docket criterion was 'zero COMPLETE depots'; operator-directed reality is 'one COMPLETE (the active substrate) plus 39 DELETE_REQUESTED finishing on GCP's schedule' — the literal criterion is inverted, the intent (clean substrate for future levies) is achieved. Side verification arriving from concurrent ₢BAAAG officium: gcloud describe on the fresh 100003 confirmed cleanupPolicies.rb-delete-untagged is attached with the expected DELETE/UNTAGGED/86400s shape (proto3 default-stripping accounts for cleanupPolicyDryRun absence in read-back). ₢BAAAG's heavier verification gates (ordain→jettison→tally cycle, batch_vouch target shift) remain that pace's obligation under ₣BA's test-suite reservation, not ₢A_AAo's.

### 2026-05-13 12:14 - ₢A_AAo - n

Cut RBRR over from the depot that predated ₢BAAAG's retention-policy landing (canest2bhm100002, in COMPLETE) to a freshly-levied successor (canest2bhm100003) under the cancbhl- cloud_prefix. rbw-dL minted cancbhl-d-canest2bhm100003 with ₢BAAAG's rb-delete-untagged policy (UNTAGGED tagState, 86400s olderThan, cleanupPolicyDryRun: false) baked into the create-repo jq body — first depot in this regime to come up with autonomous orphan-children reclamation in force. Abandoned cancbhl-d-canest2bhm100002 unmade via rbw-dU (BURE_CONFIRM=skip): governor SA removed, billing unlinked (quota released immediately), worker pools deleted, project transitioned to DELETE_REQUESTED with the standard 30-day grace. RBRR_DEPOT_MONIKER bump from 100002 to 100003 is the only repo-tracked change; depot fact files (BURD_OUTPUT_DIR, outside the repo) updated in passing by rbw-dL and rbw-dU's tracking-emit steps. Posture shift relative to the as-written ₢A_AAo docket: 'wipe all 39 depots to zero ACTIVE' becomes 'swap one and let the rest finish reaping' — the 38 already in DELETE_REQUESTED finish on GCP's schedule; the 100002 unmade here joins them. Live-infra verification gates from ₢BAAAG's notch (gcloud describe on fresh depot to inspect attached cleanupPolicies, ordain→jettison→tally cycle to exercise the V2-DELETE + filter contract end-to-end, batch_vouch assertion-target shift) remain deferred to follow-on work under ₣BA's test-suite reservation against ₣A_. Pace docket reslate to capture the swap-and-defer posture is the next conversation step per operator direction.

### 2026-05-13 08:23 - ₢A_AAo - n

Namespace depot fact files by cloud_prefix (`<cloud_prefix>/<moniker>.depot` layout) to prevent same-moniker collisions across cloud_prefixes. Surfaced when rbw-dl FATAL'd with `buf_write_fact_multi: preexists` on canest2bhm100002 existing under both cancbhl-d- and cancbhm-d- project IDs — two distinct GCP projects with the same displayName-derived moniker. Root cause was the fact-file layout assuming moniker-globally-unique; the cloud_prefix dimension was implicit in projectId but invisible to the per-moniker file naming. Structural fix: namespace the storage. Writer (zrbgp_depot_state_emit) now derives prefix subdir from projectId via `${project_id%%-d-*}`, mkdirs the subdir in both BURD_OUTPUT_DIR and BURD_TEMP_DIR, then emits `<cloud_prefix>/<moniker>.depot` and `.depot-project`. Renderer (rbgp_depot_list) glob changed from `*.depot` to `*/*.depot`; parent dir derived from path for sidecar lookup. Theurge readers (rbtdrp_pristine, rbtdrk_canonical) gain cloud_prefix_subdir helpers (read RBRR_CLOUD_PREFIX from rbrr.env, strip trailing `-` to match filesystem layout); pick_next_moniker now takes `root` and walks only the current cloud_prefix subdir — allocation is collision-safe because foreign-prefix names are correctly ignored. Direct fact-path reads in case 2 (stand-up) and case 5 (tear-down) join the prefix subdir into the path. Doc-comment refresh in rbgp_Payor.sh header, rbgc_Constants.sh, rbtdrp_pristine.rs, and rbtdrk_canonical.rs. Lister verified live: 39 depots list cleanly with both duplicates standing under separate cancbhl/ and cancbhm/ subdirs; collision is now structurally impossible. Theurge build clean. Fast suite deferred to next session per pace plan (cleanup work remains: delete all existing depots to give future levies a clean substrate under the new invariants).

### 2026-05-13 08:23 - Heat - S

depot-fact-files-namespaced-by-cloud-prefix

### 2026-05-13 07:48 - Heat - r

moved A_AAP to last

### 2026-05-13 07:37 - ₢A_AAn - W

Added four sentry-config presence-assertion crucible cases backstopping the load-bearing iptables clauses and kernel sysctl identified by ₢A_AAb's iteration arc: rbtdrc_sentry_config_rp_filter (branches on RBRN_ENTRY_MODE — asserts /proc/sys/net/ipv4/conf/all/rp_filter == 2 when enabled, == 1 when disabled), rbtdrc_sentry_config_prerouting_dnat (skips when disabled; otherwise asserts exactly 2 RETURN rules positioned before the DNAT rule in nat-table PREROUTING), rbtdrc_sentry_config_postrouting_masquerade (skips when disabled; otherwise asserts a MASQUERADE rule with -p tcp + --dport is present in nat-table POSTROUTING, discriminating from the egress MASQUERADE), rbtdrc_sentry_config_forward_estab_related (unconditional; asserts an ACCEPT rule matching RELATED+ESTABLISHED is present on the parent FORWARD chain at a position before the -j RBM-FORWARD jump). Plus rbtdrc_read_sentry_env helper for reading RBRN_* values from sentry's process environment via printenv. Cases register in RBTDRC_CASES_SECURITY (shared by tadmor + moriah fixtures), extending coverage from 54 → 58 security cases on those fixtures. Shape checks only — no env-var substitution into iptables output for failure-message decoration; plain failure messages name what regressed and the probe output goes to a per-case file in the trace dir for forensics. Correction relative to the docket prose: FORWARD case targets the parent FORWARD chain rather than RBM-FORWARD as the docket said, because rbjs_sentry.sh installs the ESTABLISHED,RELATED ACCEPT on FORWARD at line 100 before the chain-jump to RBM-FORWARD at line 111 (the script's own comment at lines 166-167 codifies this as load-bearing). Same regression class is caught — the docket's intent (catch removal/repositioning of the return-path baseline) is preserved at the right chain. Integration verification: full tadmor fixture 59/59 PASS against a live charged crucible (54 existing security cases + 4 new + 1 pentacle infra case; sortie cases including direct_sentry_probe, net_dnat_entry_reflection, net_srcip_spoof all green). Tadmor runs entry-mode=enabled, so all four new cases exercised their full assertion paths rather than skipping. The cases backstop regressions to rbjs_sentry.sh's iptables/sysctl shape — fast structural reads that fail loudly before the slower behavioral Ifrit suite has to run. They are early-warning gates, not duplicating behavioral coverage. Pace closed with three commits affiliated: 3594d08e (theurge cases + helper + RBTDRC_CASES_SECURITY registration), 833b9149 (tadmor sentry hallmark refresh — local kludge for this machine; prior hallmark was from cross-platform validation work not in local cache), 27c48b2f (tadmor bottle hallmark refresh — same reason). The two hallmark refreshes were operational prerequisites for the integration check, affiliated with this pace rather than the heat per the precedent set by ₢A_AAc's bottle-hallmark refresh.

### 2026-05-13 07:36 - ₢A_AAd - W

Wave D independent-verification pass on the spec-deltas pace. Performed a fresh cross-reference of the current state of the spec corpus against the anchor memo's Spec alignment findings subsection (Memos/memo-20260512-sentry-entry-port-repair-anchor.md lines 257-297), classifying each of the seven memo deltas without re-reading prior Wave A/Wave C diffs. All seven classified ADDRESSED: Delta 1 (rp_filter strict->loose) addressed via Wave A's structurally-correct relocation from RBSAX to RBSPT under entry-mode condition; Delta 2 (MASQUERADE dual-role) addressed via Wave A's RBSPT entry-port MASQUERADE + Wave C's RBSAX cross-reference; Delta 3 (entry-port DNAT predicate enumeration) addressed via Wave A's RBSPT enumeration + Wave C's RBSSS summary cross-ref; Delta 4 (don't-trust-Docker elevation) addressed via Wave C's RBSSS:26 architectural-principle substep; Delta 5 (topological grounding) addressed via Wave C's RBS0:609 first-packet topology + RBS0:679 architectural-commitments framing; Delta 6 (RBSIP front-to-rule linkage) addressed via Wave C's six-front per-front enforcement pointers; Delta 7 (defense-in-depth framing) addressed via Wave C's RBS0:679 layered-defense framing. Independent sweep then surfaced four documentation-only follow-up issues: (A) RBSPT operation guard over-claimed entry-mode conditionality — IP forwarding and ip_local_port_range are unconditional in rbjs_sentry.sh:82-83 and :117-118 (outside the entry-mode if-block at :120-171), only rp_filter and the iptables rules are truly entry-mode-conditional; (B) RBSSS:35 'four-clause iptables block' count didn't match RBSPT's 3 iptables-rule-step enumeration; (C) RBSSS:26 'ARCHITECTURAL PRINCIPLE:' inline label jarred vs sibling NOTE: admonitions in the same step and didn't appear elsewhere in the spec corpus; (D) RBSIP Direct Attack Front had description/enforcement mismatch — description matched the rbtdrc_sortie_direct_sentry_probe sortie while enforcement pointer matched the different rbtdrc_sortie_net_dnat_entry_reflection sortie, conflating two distinct attack classes. All four are documentation-only — code at HEAD is correct in all four cases. Operator confirmed scope ('How many affect code? -> Zero') and authorized 'repair all and notch'. All four repairs landed in commit 20a5c57: RBSPT operation guard honestly states which steps are unconditional vs entry-mode-conditional; RBSSS:35 reworded to count-free 'iptables rules and kernel sysctls (rp_filter loose, ip_local_port_range)' preserving entry-port-required-sysctls visibility; RBSSS:26 prefix changed from ARCHITECTURAL PRINCIPLE: to NOTE: matching sibling admonitions, content otherwise unchanged; RBSIP Direct Attack Front enforcement expanded to name both mechanisms (sentry's INPUT-chain default DROP with selective RBM-INGRESS allows for the direct-sentry-probe invariant + PREROUTING per-IP source-RETURN exclusion for the net-dnat-entry-reflection invariant) with cross-refs to scr_iptables_init Base Chain Configuration and scr_dns_step Filter Rules added for the INPUT-DROP path. Verification: tt/rbtd-s.TestSuite.fast.sh 98 passed / 0 failed / 0 skipped (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15) — spec edits don't break CLAUDE.md interpolation, handbook rendering, or tabtarget context generation. All attribute references used in new prose confirmed present in RBS0-SpecTop.adoc mapping section; no new quoins introduced. Multi-officium discipline observed: bud_dispatch.sh uncommitted change in another officium's working tree left untouched per additive-only rule.

### 2026-05-13 07:34 - ₢A_AAd - n

Wave D independent-verification repairs on the four documentation-only findings surfaced by a fresh cross-reference of the spec corpus against the anchor memo's Spec alignment findings subsection. (A) RBSPT operation guard at the top of the file previously said 'If entry_mode is disabled, this sequence is skipped' — but Step 1 (IP Forwarding via /proc/sys/net/ipv4/ip_forward) and Step 2 (Ephemeral Port Range via /proc/sys/net/ipv4/ip_local_port_range) are unconditional in code at HEAD (rbjs_sentry.sh:82-83 and :117-118 both sit outside the entry-mode if-block at :120-171), only Step 3 (rp_filter loose) and Steps 4-6 (DNAT Rules, Forward Filter, MASQUERADE Rule) are truly entry-mode-conditional in rbjs_sentry.sh:120-171. Wave A's structural-decomposition sweep introduced the over-broad operation guard when it removed the IP-forwarding duplication from RBSAX, landing both unconditional sysctls under an entry-mode-only guard. Operation guard rewritten to honestly state which steps are unconditional vs entry-mode-conditional. (B) RBSSS:35 had 'see scr_port_setup for the full four-clause iptables block' but RBSPT enumerates 3 iptables-rule steps (DNAT Rules with 3 rules, Forward Filter, MASQUERADE Rule), not 4 clauses. Reworded to 'iptables rules' (count-free), preserving the kernel sysctls list (rp_filter loose, ip_local_port_range) which a reader needs to know are entry-port-required even though ip_local_port_range is set unconditionally in code (its rationale ties it to entry-port port-clearance per RBSPT step 2). (C) RBSSS:26 'ARCHITECTURAL PRINCIPLE: sentry takes ownership...' used a custom ALL-CAPS inline label that appears nowhere else in the spec corpus and jars vs the sibling NOTE: admonitions in the same step at lines 23 and 25. The anchor memo's Delta 4 suggested wording had no prefix. Changed to 'NOTE:' to match sibling admonitions; content unchanged (still the architectural principle elevation memo Delta 4 wanted). (D) RBSIP Direct Attack Front had a description/enforcement mismatch — description 'Probe the sentry container for listening services, open ports' matched the rbtdrc_sortie_direct_sentry_probe sortie while the enforcement pointer (PREROUTING per-IP source-RETURN exclusion) matched the different rbtdrc_sortie_net_dnat_entry_reflection sortie. Two related-but-distinct attack classes were conflated under one front pointer. Enforcement expanded to name both mechanisms: sentry's INPUT-chain default DROP with selective RBM-INGRESS allows defends against 'no listening services on sentry' (direct-sentry-probe invariant), AND the PREROUTING per-IP source-RETURN exclusion defends against entry-port reflection (net-dnat-entry-reflection invariant). Cross-refs to scr_iptables_init Base Chain Configuration and scr_dns_step Filter Rules added for the INPUT-DROP path; pre-existing cross-ref to scr_port_setup DNAT Rules preserved for the reflection path. Verification: tt/rbtd-s.TestSuite.fast.sh 98 passed / 0 failed / 0 skipped (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15) — spec edits don't break downstream consumers (CLAUDE.md interpolation, handbook rendering, tabtarget context generation). All attribute references used in new prose (rbrn_entry_mode, rbrn_uplink_port_min, rbrn_enclave_sentry_ip, rbrn_enclave_bottle_ip, scr_iptables_init, scr_dns_step, scr_port_setup) confirmed present in RBS0-SpecTop.adoc mapping section — no new quoins. All four findings are documentation-only; no code changes implied. Memo coverage post-Wave-D: all seven deltas substantively ADDRESSED with the four precision/style follow-ups now closed.

### 2026-05-13 07:24 - ₢A_AAn - n

Refresh tadmor bottle hallmark to k260513072351-833b9149 — local kludge alongside the sentry refresh, same local-cache-empty reason. Both vessels now buildable on this host for integration testing of the ₢A_AAn sentry-config presence-assertion cases.

### 2026-05-13 07:23 - ₢A_AAn - n

Refresh tadmor sentry hallmark to k260513072313-bcc29f58 — local kludge for integration testing of the four new sentry-config presence-assertion cases on this machine. Tadmor's nameplate previously referenced k260512131447-b914bcc66 from cross-platform validation work that wasn't on this host's image cache.

### 2026-05-13 07:19 - ₢A_AAd - n

Wave C anchor-memo gap pass on spec corpus. Read the sentry-entry-port-repair anchor memo's 'Spec alignment findings' subsection and cross-referenced its seven proposed deltas against what Wave A's code-vs-spec drift sweep already shipped. Delta 1 (rp_filter strict->loose with rationale) and Delta 2 partial (MASQUERADE dual-role) and Delta 3 partial (entry-port DNAT predicate enumeration) were already landed in Wave A by virtue of the code-vs-spec methodology. Wave C addresses the remaining structural and architectural-framing gaps. Four spec files edited: (1) RBSAX NAT MASQUERADE step gains a cross-reference sentence pointing to the entry-port MASQUERADE in RBSPT, surfacing the dual-role split that the memo's Delta 2 wanted articulated in a single location but which Wave A's structural choice put across two locations (outbound-from-enclave in RBSAX, entry-port-return-path-symmetry in RBSPT) — the cross-ref makes the dual-role visible from RBSAX without duplicating prose. (2) RBSSS step 2 (interface discovery / default-route ownership) gains a new substep tagged 'ARCHITECTURAL PRINCIPLE' that elevates the don't-trust-Docker discipline from default-route-specific to a class principle — sentry takes ownership of any runtime-implicit behavior its security envelope depends on, including interface naming, default-route selection, published-port target selection, and any future runtime-implicit ordering that emerges; kernel state and sentry's own iptables are source of truth, framework-implicit ordering is not load-bearing for the security envelope (memo Delta 4 wording cribbed with minor edits). (3) RBSSS step 3 (sentry configures iptables) tightens the previously one-line 'Entry port DNAT if enabled' bullet into 'Entry-port DNAT plus POSTROUTING MASQUERADE and conntrack-DNAT-state FORWARD authorization if {rbrn_entry_mode} is enabled — see {scr_port_setup} for the full four-clause iptables block and the kernel sysctls (rp_filter loose, ip_local_port_range) the entry-port path requires'; also corrects 'ICMP allowed within enclave only' to 'ICMP allowed within enclave only; ICMP cross-boundary blocked' (memo Delta 3 condensed: the four-clause detail lives in RBSPT post-Wave-A, so RBSSS gets a tight parent-overview summary with cross-ref rather than the full enumeration the memo proposed). (4) RBS0:609 (first-packet claim) gains topological grounding: the 'first packet' guarantee is enforceable because sentry's iptables are installed before the pentacle is healthy, the pentacle's namespace is shared with the bottle so the bottle inherits routing-and-firewall state already configured, and sentry sits on every inbound path by virtue of the published-port directive being on the sentry service (not pentacle) and the bottle's default route being via sentry; topology changes that violate any of these commitments require re-validation against the Ifrit suite before declaring the security properties preserved (memo Delta 5 first-packet part). (5) RBS0:675 (all-operations-and-failure-modes claim) gains the layered-defense framing that the memo Delta 7 wanted: the properties depend on a layered-defense structure in which each layer is independently load-bearing for at least one invariant — network namespace isolation, capability bounds on the bottle (cap_drop: NET_RAW, NET_ADMIN), kernel reverse-path filtering, iptables filter chains (RBM-INGRESS, RBM-EGRESS, RBM-FORWARD), iptables NAT (PREROUTING DNAT, POSTROUTING MASQUERADE), and conntrack state tracking; removing or weakening a layer requires Ifrit-suite empirical validation to confirm the security envelope remains intact, architectural reasoning alone has been empirically insufficient (memo Deltas 5-second-part and 7 combined; the memo offered RBSCO or RBS0 as the location and RBS0 was chosen since the Security Properties listing it adjoins is the right structural neighbor). (6) RBSIP Fronts subsection gets per-front rule-linkage as memo Delta 6 requested: each of six fronts (DNS, ICMP, Direct attack, Port floor, Metadata, Namespace) now names the specific spec clause(s) and {scr_*} / {mkr_*} cross-references that enforce the invariant the sortie probes; rationale paragraph added stating the spec is self-checking — future maintainers can trace from sortie to enforcing rule, and a missing or weakened rule surfaces as a sortie failure rather than a silent regression. Namespace-front prose careful to attribute correctly: cap_drop NET_ADMIN prevents reconfiguring the network namespace the bottle shares with the pentacle, cap_drop NET_RAW prevents raw-socket crafting; container runtime namespace isolation is between (pentacle+bottle) and sentry and host, not between bottle and pentacle (which intentionally share). Metadata-front prose carefully scoped: link-local 169.254.0.0/16 passes only if the operator includes it in the allowlist (no speculation about canonical regimes). Verification: every attribute reference used in new prose (rbrn_uplink_allowed_domains, rbrn_uplink_allowed_cidrs, rbrn_uplink_port_min, rbrn_enclave_sentry_ip, rbrn_enclave_bottle_ip, rbrn_entry_mode, scr_dns_step, scr_access_setup, scr_iptables_init, scr_port_setup, mkr_bottle_launch, at_recipe_bottle, at_bottle_container, at_transit_network, at_enclave_network) confirmed defined in RBS0 mapping section — no new quoins. tt/rbtd-s.TestSuite.fast.sh: 98/98 green (enrollment-validation 47, regime-validation 27, regime-smoke 9, handbook-render 15). Memo coverage status post-Wave-C: Delta 1 done (Wave A); Delta 2 done (RBSPT in Wave A + RBSAX cross-ref in Wave C); Delta 3 done (RBSPT detail in Wave A + RBSSS summary+cross-ref in Wave C); Delta 4 done (Wave C RBSSS architectural-principle elevation); Delta 5 done (Wave C RBS0:609 topological grounding + RBS0:675 architectural-commitments framing); Delta 6 done (Wave C RBSIP front-to-rule linkage); Delta 7 done (Wave C RBS0:675 layered-defense framing — memo said RBSCO or RBS0, RBS0 chosen for adjacency to Security Properties listing). All seven deltas addressed.

### 2026-05-13 07:16 - ₢A_AAn - n

Add four sentry-config presence-assertion crucible cases backstopping the load-bearing iptables clauses and kernel sysctl identified by ₢A_AAb's iteration arc — fast structural reads that fail loudly before the slower behavioral Ifrit suite runs. Cases: rbtdrc_sentry_config_rp_filter (branches on RBRN_ENTRY_MODE: asserts /proc/sys/net/ipv4/conf/all/rp_filter == 2 when enabled, == 1 when disabled), rbtdrc_sentry_config_prerouting_dnat (skips when disabled; otherwise asserts exactly 2 RETURN rules positioned before the DNAT rule in nat-table PREROUTING), rbtdrc_sentry_config_postrouting_masquerade (skips when disabled; otherwise asserts a MASQUERADE rule with -p tcp + --dport is present in nat-table POSTROUTING, discriminating it from the egress MASQUERADE which has neither), rbtdrc_sentry_config_forward_estab_related (unconditional; asserts an ACCEPT rule matching RELATED+ESTABLISHED is present on the parent FORWARD chain at a position before the -j RBM-FORWARD jump). Plus rbtdrc_read_sentry_env helper for reading RBRN_* values from sentry's process environment via printenv. Cases register in RBTDRC_CASES_SECURITY (shared by tadmor + moriah fixtures). Targeting the parent FORWARD chain rather than RBM-FORWARD as the docket prose said: rbjs_sentry.sh installs the ESTABLISHED,RELATED ACCEPT on FORWARD at line 100 before the chain-jump to RBM-FORWARD at line 111 (the script's own comment at lines 166-167 codifies this as load-bearing). Shape checks only — no env-var substitution into iptables output for failure-message decoration. Plain failure messages name what was missing; one probe-output file per case written to the trace dir for forensics. Theurge unit tests (94 cases) pass; live-crucible integration check against a charged tadmor crucible remains as the gate before wrap.

### 2026-05-13 07:11 - ₢A_AAd - n

Wave A code-vs-spec drift repair on RBS sentry/bottle/port/dns/iptables specs. Scope: lineage footprint of A_AAb (sentry entry-port DNAT/FORWARD rescope) and A_AAc (Ifrit external-spoof sortie). Universe: all orchestration steps in rbjs_sentry.sh, rbjp_pentacle.sh, rbob_bottle.sh, rbob_compose.yml, common-sentry-context/Dockerfile, .rbk/<moniker>/rbrn.env. Drift classes addressed: (A) Repaired-mechanism drift in RBSPT — DNAT rule rewritten from single -i eth0 + dport+DNAT shape to three-rule per-IP RETURN exclusion of sentry/bottle IPs + unconditional DNAT (with whole-CIDR-is-incorrect rationale and load-bearing classification axis statement); FORWARD rule rewritten from -i eth0 -o eth1 shape to conntrack --ctstate DNAT authorization with unforgeable-from-bridge rationale; MASQUERADE wording cleaned to drop eth1 literal and add return-path-symmetry rationale. (B) Interface-naming sweep in RBSPT/RBSAX/RBSDS — every literal eth0/eth1 replaced with 'interface attached to {at_transit_network}' or '...{at_enclave_network}', consistent with the IP-based discovery discipline already documented in RBSSS:22-25 and RBSNX:12-14 (sentry's rbjs_sentry.sh:33-51 enforces discover-by-IP for both interfaces). (C) Silent-omissions filled — added Ephemeral Port Range step to RBSPT (ip_local_port_range, with cross-ref to entry-port-must-be-below uplink-port-min invariant); added ICMP Within Enclave and ICMP Cross-Boundary Block steps to RBSII (matching rbjs_sentry.sh:113-115 and 173-175); added Capability and label bounds step to RBSBL describing cap_drop NET_RAW/NET_ADMIN and security_opt label:disable as kernel-layer defense-in-depth peer to iptables-layer enforcement at sentry (claim deliberately scoped: does NOT attribute closure of any specific attack class to cap_drop, since A_AAc empirical finding showed the Docker bridge per-veth source-IP enforcement is what actually blocks the spoofed-SYN attack class on Docker Desktop runtime, regardless of NET_RAW state inside the bottle namespace). (D) Structural decomposition correctness — moved rp_filter step from RBSAX (incorrectly conditioned on uplink-access-mode) to RBSPT (correctly conditioned on entry-mode) with corrected value strict-mode-1 → loose-mode-2 and rewritten rationale (port-publish delivery interface mismatch, framework gateway reverse-route asymmetry); removed duplicated 'Enable IPv4 forwarding' bullet from RBSAX (already correctly placed in RBSPT step 1). Verification: post-edit grep confirms zero remaining eth0/eth1 literals and zero remaining 'rp_filter strict' / 'rp_filter 1' claims in any of the five files; all attribute references used in edits (at_transit_network, at_enclave_network, at_sentry_container, rbrn_entry_mode, rbrn_entry_port_workstation, rbrn_entry_port_enclave, rbrn_enclave_sentry_ip, rbrn_enclave_bottle_ip, rbrn_enclave_base_ip, rbrn_enclave_netmask, rbrn_uplink_port_min, rbrn_uplink_access_mode, rbrn_uplink_dns_mode, rbrn_uplink_allowed_cidrs, rbrn_uplink_allowed_domains, rbrr_dns_server, scr_iptables_init, scr_port_setup, scr_access_setup, scr_dns_step, mkr_bottle_launch) confirmed present in RBS0-SpecTop.adoc mapping section — no new quoins introduced. tt/rbtd-s.TestSuite.fast.sh: 98 passes / 0 fails / 0 skipped across enrollment-validation (47), regime-validation (27), regime-smoke (9), handbook-render (15). Approach reframed mid-pace per operator: code-at-HEAD (canonical pristine green) is the authoritative contract; anchor-memo Spec alignment findings to be consulted in Wave C as a coverage backstop rather than primary contract. Single-notch-after-batch per operator preference; one delta per commit discipline from original docket relaxed by operator approval after inventory review. Wave C (anchor-memo gap pass) and Wave D (sweep verification) remain queued.

### 2026-05-13 06:44 - ₢A_AAb - W

Sentry entry-port DNAT/FORWARD rescope complete. Per-IP exclusion form (PREROUTING per-IP RETURN short-circuit for the two enclave-internal IPs + unconditional DNAT + POSTROUTING MASQUERADE + RBM-FORWARD conntrack-DNAT ACCEPT + ESTABLISHED,RELATED baseline + rp_filter=2 loose) is committed to rbev-vessels/common-sentry-context/rbjs_sentry.sh. Canonical-declaration gate satisfied via cross-platform pristine PASS recorded in commits a30823a8 (macOS, 1h 39m, merge) and 8afa7ff6 (Linux/cerebro, 95m 58s); synthesis line in commit 1743b347. Source tree at HEAD contains the validated per-IP form. Wrap attributes those pristine runs as the canonical evidence for this pace; no fresh gauntlet runs needed. Architecture spec deltas remain queued in ₢A_AAd; structural-presence assertions queued in ₢A_AAn. Anchor memo: Memos/memo-20260512-sentry-entry-port-repair-anchor.md.

### 2026-05-12 20:17 - Heat - n

Broaden OAuth token-mint retry to cover 'Invalid grant: account not found' (fresh-SA propagation lag) alongside existing 'Invalid JWT Signature.' shape; rename log messages to 'SA propagation race' and emit error_description per attempt.

### 2026-05-12 12:16 - ₢A_AAc - W

Added sortie_net_srcip_spoof_external + theurge case rbtdrc_sortie_net_srcip_spoof_external probing the residual spoof-as-arbitrary-external attack class left open by the per-IP RETURN PREROUTING exclusion + rp_filter=2 loose. Ifrit crafts a TCP SYN with spoofed source 8.8.8.8 (external-routable, not enclave/sentry/bottle/loopback/allowed-CIDR) targeting sentry's workstation entry port; raw IPPROTO_TCP listener in bottle's namespace watches for DNAT-reflected SYN matching distinctive src-port=40021 and dst-port=RBRN_ENTRY_PORT_ENCLAVE. Empirical result on macOS Docker Desktop 28.x against tadmor: PASS (SECURE) — but mechanism is NOT the iptables layer the docket reasoned about. Sentry's PREROUTING NAT chain showed 0 packets matched at dport=8890 after the sortie ran (all three rules — RETURN sentry-IP, RETURN bottle-IP, DNAT — at 0/0); the spoofed SYN never reached sentry. The block is consistent with Docker bridge's per-veth source-IP enforcement at egress, a default Docker security control that drops packets whose source IP does not match the originating container's IP regardless of CAP_NET_RAW + IP_HDRINCL inside the container's namespace. This is a structural Docker fact in the same family as the anchor memo's 'host attaches to enclave bridge as a peer with bridge gateway IP' finding from the per-IP transition arc — surfaced by empirical testing rather than predicted by architectural reasoning. Consequence: the architectural argument that traded rp_filter=1 for source-IP-flexibility on Docker Desktop delivery is mooted at this specific attack class because Docker's bridge enforcement already blocks below iptables; the iptables layer is not load-bearing for THIS attack class on this runtime (it remains load-bearing for other attack classes such as the spoof-as-allowed-cidr → forbidden-dst case the existing sortie_net_srcip_spoof covers). The sortie retains regression-backstop value: if a future change (alternative container runtime, custom network mode, raw bridge configuration) removes the Docker bridge enforcement, the sortie would report BREACH and surface the regression. Empirical finding captured in sortie's leading doc comment (load-bearing for future maintainers — the predicted mechanism differs from what's actually load-bearing) and a brief pointer comment on the theurge case fn. Cross-platform verification on Linux Docker Engine and Podman remains open before declaring canonical PASS across the matrix. Branch pym-₢A_AAP fast-forwarded to main this session; main now at 75f2bc8d, 88 commits ahead of origin/main, not yet pushed.

### 2026-05-12 12:13 - ₢A_AAc - n

Capture empirical finding in the sortie's leading comment and add a pointer comment on the theurge case fn. On macOS Docker Desktop 28.x against tadmor, the sortie returned PASS (SECURE) but mechanism is NOT the iptables layer the docket reasoned about: sentry's PREROUTING NAT chain showed 0 packets matched at dport=8890 after the sortie ran (all three rules — RETURN sentry-IP, RETURN bottle-IP, DNAT — at 0/0). The spoofed SYN never reached sentry. The block is consistent with Docker bridge's per-veth source-IP enforcement at egress (a default Docker security control that drops packets whose source IP does not match the originating container's IP, regardless of CAP_NET_RAW + IP_HDRINCL inside the container's namespace). This is a structural Docker fact in the same family as the anchor memo's 'host attaches to enclave bridge as a peer with bridge gateway IP' finding from the per-IP transition arc — surfaced by empirical testing rather than predicted by architectural reasoning. Consequence captured in the comment: the architectural argument that traded rp_filter=1 for source-IP-flexibility on Docker Desktop delivery is mooted at this specific attack class because Docker's bridge enforcement already blocks below iptables; the iptables layer is not load-bearing for THIS attack class on this runtime, though it remains load-bearing for other attack classes (e.g., the spoof-as-allowed-cidr → forbidden-dst case the existing sortie_net_srcip_spoof covers). The sortie retains regression-backstop value: if a future change (alternative container runtime, custom network mode, raw bridge configuration) removes the Docker bridge enforcement, this sortie would report BREACH and surface the regression. Open: cross-platform verification on Linux Docker Engine and Podman before declaring canonical PASS across the matrix. Notes added in sortie's doc comment (load-bearing — the empirical mechanism differs from docket prediction and a future maintainer reading the docstring needs that context) and a brief pointer comment on the theurge case fn referring back.

### 2026-05-12 12:07 - Heat - S

sentry-config-presence-assertions

### 2026-05-12 12:06 - ₢A_AAc - n

Refresh tadmor bottle hallmark to k260512120611-e25b5fe9 — the kludge of e25b5fe9 baked the new sortie_net_srcip_spoof_external (raw IPPROTO_TCP listener + spoofed-SYN sender) into the ifrit binary inside the bottle image.

### 2026-05-12 12:06 - ₢A_AAc - n

Add sortie_net_srcip_spoof_external and theurge case rbtdrc_sortie_net_srcip_spoof_external probing the residual security envelope gap left open by the per-IP RETURN PREROUTING exclusion + rp_filter=2 loose. An enclave-internal source crafts a TCP SYN with spoofed source IP 8.8.8.8 (arbitrary external-routable, not enclave, not sentry, not bottle, not loopback, not in allowed CIDR) targeting sentry's workstation entry port; the spoof matches neither RETURN rule, DNAT fires, FORWARD chain accepts via conntrack-DNAT-state, MASQUERADE rewrites source to sentry's bridge IP, and the packet arrives at the bottle's enclave entry port. Detection uses a raw IPPROTO_TCP listener inside bottle's namespace, filtering inbound packets by dst-port==RBRN_ENTRY_PORT_ENCLAVE AND src-port==40021 (distinctive ephemeral); src-port survives MASQUERADE unchanged. Strict rp_filter would have blocked the egress of the spoofed packet at the bottle's kernel layer, but loose rp_filter for legitimate Docker Desktop delivery removed that defense. Per the pace docket, this sortie verifies whether the iptables-layer enforcement holds without rp_filter=1's kernel-layer protection: empirically blocked => security envelope sound + regression backstop established; empirically allowed => architecture needs targeted fix (raw-table source-CIDR drop on enclave interface or selective rp_filter restoration). Companion to existing sortie_net_srcip_spoof (which tests spoof-as-sentry/spoof-as-allowed-cidr/spoof-as-loopback targeting forbidden external dst) and net_dnat_entry_reflection (which tests real-source enclave reflection). Also includes the bottle hallmark refresh from re-kludge (k260512115633-73695c39) carried out earlier this session.

### 2026-05-12 12:02 - Heat - S

consolidate-srcip-spoof-parameterization

### 2026-05-12 12:02 - Heat - S

ifrit-sortie-spoof-from-outside-as-enclave-ip

### 2026-05-12 12:02 - Heat - S

ifrit-sortie-dnat-source-observation

### 2026-05-12 11:39 - ₢A_AAb - n

Capture empirical cross-platform validation of the per-IP exclusion form. Both macOS Docker Desktop 28.x and Linux Docker Engine 28.x report three-fixture parallel green: srjcl 3/3 (including the prior-regression target rbtdrc_srjcl_jupyter_connectivity on Linux), pluml 5/5, tadmor 54/54 each including the decisive security cases (direct_sentry_probe, net_dnat_entry_reflection, net_srcip_spoof). Cerebro provided mechanism-level confirmation: DNAT counter incremented to 1 packet under per-IP form against the same host where whole-CIDR showed 0 packets across 20 SYN retransmits; source IP flow traced as 10.242.2.1 (host bridge gateway) → sentry PREROUTING → DNAT to 10.242.2.3:8000 → POSTROUTING MASQUERADE rewrites source to 10.242.2.2 (sentry's enclave IP) → bottle sees enclave-internal peer. Return-path symmetry working exactly as the architectural argument predicted. Memo updates: (1) TL;DR empirical status section restructured to show cross-platform green under per-IP form with mechanism-level confirmation; (2) Reset handoff notes updated from 'in-flight hypothesis' framing to 'post-empirical-validation' framing; first move on cold mount is now 'verify per-IP RETURN pattern is in place' rather than 'implement per-IP'; what done looks like simplifies to 'pristine gauntlet on both platforms'; anti-patterns retained; new operational note added about cerebro's stale-hallmark workflow observation (not architectural, but worth flagging); (3) Empirical validation table dual-column format updated: per-IP column now shows green on both platforms with mechanism details; whole-CIDR column shows the historical state for lineage; (4) Lessons captured Attempt 6 marked 'empirically validated cross-platform at three-fixture level' with mechanism-level confirmation note; pristine gauntlet remains pending for canonical declaration; (5) Open questions: items 2 and 3 (Linux verification, macOS re-verification) marked resolved (green); new item 3a added for pristine gauntlet as the remaining canonical-declaration gate. Architecture is now empirically validated; pristine gauntlet is hardening margin for canonical pass. The work has crossed the threshold from 'hypothesis with empirical evidence' to 'empirically validated architecture'.

### 2026-05-12 11:15 - ₢A_AAb - n

Refresh tadmor sentry hallmark to k260512111528-64aab808 (per-IP form).

### 2026-05-12 11:15 - ₢A_AAb - n

Refresh pluml sentry hallmark to k260512111518-6f3544a3 (per-IP form).

### 2026-05-12 11:15 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512111508-4c2d418a (per-IP RETURN short-circuit form).

### 2026-05-12 11:14 - ₢A_AAb - n

Transition PREROUTING entry-port classifier from whole-CIDR exclusion to per-IP RETURN short-circuit per anchor memo's current Fix Recipe. The whole-CIDR form (! -s ${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}) was empirically falsified on linux Docker Engine (cerebro diagnostic) — the host attaches to the enclave bridge as a peer with bridge gateway IP (.1), which is inside the enclave CIDR by Docker convention, so the predicate rejected every legitimate host SYN and sentry's PREROUTING DNAT fired 0 times. Replacement is three rules: -s SENTRY_IP -j RETURN; -s BOTTLE_IP -j RETURN; unconditional DNAT. The RETURN short-circuit pattern is the unambiguous iptables idiom; the single-rule multi-! -s form has uncertain semantics (some references say only the last -s is honored) and is reserved as an optional empirical follow-up. The per-IP form expresses the threat model directly — the enclave-internal containers are exactly two (sentry + bottle/pentacle, the latter share namespace) — and is platform-agnostic: bridge gateway, Docker Desktop VM gateway 192.168.65.1, rootlesskit synthetic source 10.0.2.100, and external clients all pass naturally because none equals sentry-IP or bottle-IP. Other entry-mode-block clauses unchanged (MASQUERADE return-path symmetry, conntrack-DNAT-state FORWARD authorization, rp_filter=2 loose, parent FORWARD ESTABLISHED,RELATED). Maintenance burden noted: if future architecture adds enclave-internal containers beyond sentry+bottle (e.g., a sidecar), the RETURN list must grow — whole-CIDR didn't have that burden. Next: kludge sentry vessel, refresh srjcl/pluml/tadmor hallmarks, three-fixture parallel verification on macOS (Linux verification follows via separate session).

### 2026-05-12 11:12 - ₢A_AAb - n

Add 'Reset handoff notes' subsection to the anchor memo positioned right after TL;DR. This is reset-readiness prep — if the architect chat resets while the per-IP-exclusion hypothesis is still empirically pending, a fresh-cold agent reading the memo can pick up the in-flight state cleanly. The subsection covers: where we are right now (source tree has prior whole-CIDR form; memo Fix Recipe has per-IP hypothesis; the divergence is by design); first-move discipline on cold mount (verify divergence, read Fix Recipe, run next experiment); the bounded next experiment (per-IP implementation, kludge + refresh, cross-platform three-fixture parallel); what 'done' looks like (both platforms green under per-IP form + full pristine gauntlet for canonical); iteration discipline if the experiment surfaces another structural surprise (process result, update memo, iterate within the architecture); and four explicit anti-patterns (do not simplify, do not skip cross-platform, do not abandon architecture, do not change source tree before committing per-IP form). Pairs with the parallel jjx_redocket update to ₢A_AAb's docket that surfaces the same in-flight state at navigation level. Together, the docket update + memo subsection give a cold-mount agent the orientation needed to execute the bounded next step without losing the iteration context this chat built up.

### 2026-05-12 11:06 - ₢A_AAb - n

Capture third structural surprise and per-IP-exclusion hypothesis. Cerebro diagnostic on linux Docker Engine empirically falsified the whole-CIDR exclusion form: the host attaches to the enclave bridge as a peer with bridge gateway IP (.1), which is inside the enclave CIDR by Docker convention; `! -s ENCLAVE_CIDR` rejects every legitimate host SYN; sentry's PREROUTING DNAT fires 0 times. macOS-only testing missed this because Docker Desktop's VM-proxy structure puts the host outside the bridge (source IP 192.168.65.1 in its own subnet). Updates to anchor memo: (1) TL;DR replaced with current per-IP-exclusion hypothesis + honest empirical status (macOS green under PRIOR whole-CIDR form, linux FAILED under whole-CIDR, per-IP form not yet tested on either platform). (2) Fix recipe PREROUTING clause replaced: per-IP exclusion via RETURN short-circuit pattern (`-s SENTRY_IP -j RETURN`; `-s BOTTLE_IP -j RETURN`; unconditional DNAT) instead of whole-CIDR exclusion; rationale articulated; multi-`! -s` semantic uncertainty flagged; maintenance burden noted (sidecar additions need explicit list update). (3) Empirical validation table restructured: dual columns comparing prior whole-CIDR shape vs current per-IP hypothesis per platform; honest 'no platform meets canonical declaration gate under current hypothesis' framing. (4) Lessons captured: Attempt 5 (Option E corrected, whole-CIDR) now documented as 'failed at linux'; Attempt 6 (Option E corrected v2, per-IP) added as current hypothesis. Meta-lesson section expanded: five empirical surprises now enumerated (interface filter, default-route accident, MASQUERADE removal in topology reframe, source-CIDR removal in Option E, whole-CIDR-vs-per-IP); pattern reframed from 'load-bearing defenses' to 'structural Docker facts surface via cross-platform testing'; explicit prediction that Podman testing will surface a third class of structural fact. (5) Spec Alignment Findings Delta 3 updated: enumerate per-IP RETURN short-circuit form rather than whole-CIDR; note maintenance burden. (6) Open Questions: ESTABLISHED,RELATED marked resolved (already was); Linux verification under per-IP form added as load-bearing next experiment; macOS re-verification under per-IP form added; trailing items renumbered; new item 9 added for multi-`! -s` semantic empirical confirmation as minor open. (7) 'What in companion memo is superseded' updated: Round 3's whole-CIDR was MORE right than 'drop source-CIDR' but itself empirically falsified on linux; per-IP is the current correction. Memo now reflects honest current state: per-IP-exclusion is hypothesis pending cross-platform empirical confirmation; canonical declaration awaits both-platform green under the current form.

### 2026-05-12 10:57 - Heat - S

scry-plain-output-mode

### 2026-05-12 10:57 - Heat - S

scry-docker-bridge-capture

### 2026-05-12 10:57 - Heat - S

scry-filter-argument

### 2026-05-12 10:57 - Heat - S

scry-finite-duration-exit

### 2026-05-12 10:57 - Heat - S

scry-charge-aware-lifecycle

### 2026-05-12 10:56 - Heat - S

scry-sentry-interface-coverage

### 2026-05-12 10:43 - ₢A_AAb - n

Capture post-empirical-correction state across both memos. Anchor memo updates: (1) TL;DR reflects empirically-validated five-clause fix recipe (source-CIDR PREROUTING + entry-port MASQUERADE + conntrack-DNAT FORWARD + ESTABLISHED,RELATED baseline + rp_filter=2 loose) and empirical status (macOS three-fixture green: srjcl 3/3, pluml 5/5, tadmor 54/54 including direct_sentry_probe, net_dnat_entry_reflection, net_srcip_spoof; Linux in progress per operator). (2) 'What is superseded' section corrected: Round 3's source-CIDR was right; this anchor memo's earlier 'drop source-CIDR' reasoning was empirically falsified by Ifrit direct_sentry_probe. (3) Fix recipe rewritten with the four-clause + sysctl form, each clause with per-clause rationale; the destination-port-only earlier draft is explicitly marked as empirically-falsified. (4) New 'Empirical validation status' table covering macOS, Linux, Podman. (5) New 'Ifrit coverage limitation' subsection linking the residual spoof-as-arbitrary-external-routable-IP gap to pace ₢A_AAc. (6) Lessons captured updated: attempt 2 stage 2 failure now explained as rp_filter=1 strict mode dropping at routing time (not missing MASQUERADE as earlier framing said); Option E added as Attempt 4 with the architect's contractual-vs-capability confusion documented; Option E corrected added as Attempt 5; new meta-lesson 'layered defenses are load-bearing by default' added with four-removal-pattern listing. (7) New 'Spec alignment findings' subsection added as contract for pace ₢A_AAd: seven specific gaps (entry-port DNAT predicate enumeration thin in RBSSS; rp_filter strict commitment wrong in RBSAX; MASQUERADE two-role gap in RBSAX; don't-trust-Docker should be elevated; first-packet-and-all-failure-modes need topological grounding; defense-in-depth implicit; Ifrit Direct Attack Front not linked to specific rules) plus seven concrete deltas to apply (specific files/regions/intents) plus mount-time-inheritance note for incorporating ₢A_AAc findings. (8) Open questions updated: ESTABLISHED,RELATED verification marked resolved; Linux verification marked in-progress with operator-reported test-infrastructure issues; new items added for ₢A_AAc and ₢A_AAd contractual scope. Companion memo update: post-implementation-update banner added to top of 'Refined Fix Shape' (Round 3) section noting empirical refutation of the 'source-CIDR structurally broken' claim and pointing forward at anchor memo for the empirically-corrected shape. Both memos now coherent with empirical state; ready for chat reset and ₢A_AAc/₢A_AAd execution downstream.

### 2026-05-12 10:37 - Heat - S

rbs-spec-deltas-post-sentry-fix

### 2026-05-12 10:25 - Heat - S

ifrit-sortie-enclave-spoof-external

### 2026-05-12 10:07 - ₢A_AAb - n

Refresh tadmor sentry hallmark to k260512100702-6f22efba (Option E corrected).

### 2026-05-12 10:06 - ₢A_AAb - n

Refresh pluml sentry hallmark to k260512100651-e08c8da3 (Option E corrected).

### 2026-05-12 10:06 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512100642-81688036 (Option E corrected — source-CIDR exclusion restored).

### 2026-05-12 10:06 - ₢A_AAb - n

Restore source-CIDR exclusion (! -s ENCLAVE_CIDR) to the PREROUTING DNAT rule — corrects Option E after empirical falsification at tadmor's direct_sentry_probe. The prior 'destination-port-only' classifier from commit ac06284254e5 allowed ifrit (inside enclave) to dial sentry:7999 and reach the bottle's listener via sentry's DNAT — a BREACH per ifrit's contractual invariant 'enclave-internal sources MUST NOT reach sentry's entry port' (enforced by both rbtdrc_sortie_direct_sentry_probe and rbtdrc_sortie_net_dnat_entry_reflection cases in the tadmor suite). The architect's original rationale for dropping source-CIDR was twofold: (a) 'unnecessary overconstraint on Docker Desktop' — true for legitimate external traffic from 192.168.65.1, but missed that the rule's actual job is keeping enclave SOURCES out; (b) 'structurally broken on rootless Podman rootlesskit 10.0.2.100' — false here because our enclave CIDR is 10.242.x.0/24 with explicit non-overlap with rootlesskit's range, and the companion memo's startup guardrail enforces this. Empirically validated by attempt-2-stage-1 (tadmor 54/54 with this exact PREROUTING shape in place). The architect has explicitly accepted the correction. Other Option E elements unchanged: MASQUERADE on POSTROUTING (return-path symmetry), conntrack-DNAT-state on RBM-FORWARD (authorization), rp_filter=2 (interface-agnostic kernel routing), parent FORWARD ESTABLISHED,RELATED (return-path authorization at the layer-above-RBM level). Larger pattern surfaced by this iteration arc: four 'redundant' defensive layers (interface filter, default-route accident, entry-port block, source-CIDR exclusion) have each been independently shown to be load-bearing — the original architecture was genuine defense-in-depth, not redundancy. Operating posture: keep every defense by default; remove only with empirical falsification, never with pure architectural reasoning.

### 2026-05-12 09:54 - ₢A_AAb - n

Add post-implementation-update note at the top of the companion memo's 'Topology-Level Reframing' section pointing forward to the anchor memo. The reframe was implemented and empirically refuted via scry packet capture (RBM-FORWARD counters zero, Docker Desktop NAT reflection through transit, asymmetric return-path failure). The note prevents a fresh agent who lands in the companion memo first (rather than via docket → anchor) from misreading the reframe as still the recommended path. The reframe content below the note is retained as historical record of the hypothesis. Closes the bidirectional cross-reference: anchor memo's 'What is now superseded' section already points back at the companion's reframe section; this update points forward from the companion's reframe section to the anchor memo. Final retention edit before chat reset.

### 2026-05-12 09:53 - ₢A_AAb - n

Refresh tadmor sentry hallmark to k260512095338-c96c91b6 (Option E + rp_filter relaxation).

### 2026-05-12 09:53 - ₢A_AAb - n

Refresh pluml sentry hallmark to k260512095330-bcf5647b (Option E + rp_filter relaxation).

### 2026-05-12 09:53 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512095318-421ce0ec (Option E + rp_filter relaxation).

### 2026-05-12 09:53 - ₢A_AAb - n

Relax rp_filter to loose mode (2) when entry mode is enabled. Discovered during Option E scry verification: with rp_filter=1 (strict), inbound published-port SYNs from Docker Desktop gateway (192.168.65.1) arriving on the enclave interface get dropped at routing time because the reverse path for the source is only via the uplink default route. The packet never reaches the FORWARD chain — all FORWARD counters stay at zero — making Option E's 'destination-port-only classifier, interface-agnostic' design impossible to implement at the kernel-routing layer with strict rp_filter. Empirical confirmation: `ip route get 10.242.2.3 from 192.168.65.1` returned 'Network is unreachable' under rp_filter=1; setting rp_filter=2 live in the running container made curl http://localhost:7999/lab return HTTP 200 immediately with all expected iptables counters (DNAT, MASQUERADE, RBM-FORWARD conntrack-DNAT-ACCEPT, parent FORWARD ESTABLISHED,RELATED) incrementing as designed. Two-part edit: (1) added rp_filter=2 setter inside the entry-mode block at RBJp2c — this is the natural home because entry-mode is the load-bearing reason for loose rp_filter (without entry mode, traffic doesn't enter via 'wrong' interfaces); (2) removed the previously-existing `echo 1 > /proc/sys/net/ipv4/conf/all/rp_filter` from the access-mode-enabled branch at RBJp3, which would have overwritten back to strict and broken Option E for srjcl/tadmor/moriah/ccyolo (entry=enabled AND access=allowlist). Pluml (entry=enabled, access=disabled) wasn't affected by the strict setter anyway (disabled branch never set rp_filter), but is now consistently set to loose via the entry-mode block. Security trade-off: rp_filter is a defense against spoofed source IPs at the kernel layer; loose mode (2) allows any source reachable via any interface. The iptables FORWARD/INGRESS/EGRESS chains do explicit per-interface and per-destination filtering — those remain the load-bearing security controls. The tadmor adversarial security suite (including rbtdrc_sortie_net_srcip_spoof) will be validated by the upcoming three-fixture parallel run; if any spoof case regresses, we'll re-evaluate.

### 2026-05-12 09:48 - ₢A_AAb - n

Refresh tadmor sentry hallmark to k260512094819-b14cbeda (Option E image) — so tadmor security suite validates egress posture under the NEW sentry script.

### 2026-05-12 09:48 - ₢A_AAb - n

Refresh pluml sentry hallmark to k260512094810-40d054e4 (Option E image).

### 2026-05-12 09:48 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512094800-ac062842 (Option E image with restored entry-port iptables block per anchor memo recipe).

### 2026-05-12 09:47 - ₢A_AAb - n

Revert topology reframe and apply Option E (anchor memo Memos/memo-20260512-sentry-entry-port-repair-anchor.md). Compose: move ports back to sentry with symmetric ${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_WORKSTATION} mapping (sentry's DNAT does port translation, so publish maps symmetrically). Remove ports from pentacle. Add NET_ADMIN to bottle's cap_drop (defense in depth — without it, a compromised bottle in pentacle's shared namespace could alter pentacle's iptables state; the existing NET_RAW drop was insufficient for the namespace-sharing topology). Sentry script: re-introduce the entry-port iptables block, in the form the anchor memo prescribes. PREROUTING DNAT classified by destination port ONLY — no -i interface filter (which caused the original BBABC regression) and no ! -s source-CIDR filter (which would have broken on rootless Podman's rootlesskit 10.0.2.100 rewrite). POSTROUTING MASQUERADE on the enclave-egress entry-port path — load-bearing per the scry diagnostic: rewrites the bottle's perceived source to sentry's enclave IP, keeping the reply destination in-enclave so it routes via directly-connected enclave back to sentry (avoiding the asymmetric default-via-sentry-into-transit path that empirically broke under the topology reframe). RBM-FORWARD ACCEPT scoped to destination + conntrack DNAT-state — unforgeable inbound authorization. Block gated on RBRN_ENTRY_MODE=enabled (only srjcl and pluml install the rules; tadmor/moriah/ccyolo with entry_mode=disabled skip). Parent FORWARD chain's ESTABLISHED,RELATED ACCEPT at line 100 satisfies return-path authorization before reaching RBM-FORWARD (verified — no additional rule needed in RBM-FORWARD per architect's read-and-verify directive). Retained from the failed reframe: ip_forward lift to unconditional position at RBJp2 (hygiene, independent of architecture choice); tcpdump in Dockerfile (already committed); readiness-delay revert to 30 (already committed). Next: kludge sentry, refresh srjcl/pluml/tadmor hallmarks, scry verify, three-fixture parallel.

### 2026-05-12 09:41 - ₢A_AAb - n

Add anchor memo for next sentry-entry-port repair attempt after the topology-reframe failure and scry-empirical confirmation. The companion diagnostic memo (memo-20260512-srjcl-port-publish-regression.md) carries full lineage — four rounds of web research, two failed repair attempts, hypothesis evolution; this anchor memo carries forward action — revert plan, fix recipe (architectural, no snippets), verification protocol, and lessons. Decisive scry evidence: source IP at bottle's listener on Docker Desktop is 192.168.65.1 (Desktop's host-to-VM gateway, NOT enclave bridge gateway as the companion memo hypothesized); sentry's RBM-FORWARD counters all zero during topology-reframe attempt; SYN-ACK reappears on sentry's transit interface post-Docker-Desktop NAT reflection. Architectural conclusion: POSTROUTING MASQUERADE was structurally load-bearing in the pre-regression design (it kept the bottle's reply destination in-enclave for clean conntrack-mediated return); the topology reframe broke this; sentry must be in the inbound path. Fix recipe: restore sentry-as-publisher with symmetric 7999:7999 mapping; re-introduce entry-port iptables block in interface-agnostic, source-IP-agnostic form (PREROUTING DNAT classified by dport only + POSTROUTING MASQUERADE retained + RBM-FORWARD ACCEPT with conntrack DNAT-state + verified ESTABLISHED,RELATED baseline rule); retain ip_forward lift, NET_ADMIN cap_drop, and rbw-cs.Scry tabtarget from the failed reframe attempt. The new memo is self-contained for execution but points at the companion for lineage, at rbjs_sentry.sh for exact syntax (no snippet duplication per operator preference), and at scry for empirical verification. Positions both this chat and the coding agent's chat for clean reset — next agent picks up the anchor memo cold and executes.

### 2026-05-12 09:27 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512092705-bd33909c (tcpdump-equipped image) for the scry diagnostic. Only srjcl refreshed — pluml and tadmor will be refreshed once we understand the failure and have a real fix.

### 2026-05-12 09:26 - ₢A_AAb - n

Add tcpdump to the sentry/pentacle image so scry (Tools/rbk/rboo_observe.sh, tt/rbw-cs.Scry.sh) actually works. Discovered during the pentacle-publisher diagnostic: scry's tcpdump exec failed with 'No such file or directory' because tcpdump was never in the image apt-install list. The scry tooling appears to have been written against an assumption that tcpdump would be present, but the Dockerfile installs only bash/iptables/dnsmasq/iproute2/iputils-ping/dnsutils/procps. Adding tcpdump completes the existing diagnostic infrastructure. Without it the only probes available are iptables counter deltas, ss state inspection, and /proc/net/nf_conntrack — which got us to 'connection in SYN_RECV state with src=192.168.65.1' but cannot show packet-level flow at sentry's interfaces. Next: kludge sentry vessel and refresh srjcl hallmark to test the new image.

### 2026-05-12 09:01 - ₢A_AAb - n

Refresh tadmor sentry hallmark to k260512090057-3c5bac2d (pentacle-publisher topology image) — so tadmor's adversarial security suite validates egress posture under the NEW sentry script, not the previous one.

### 2026-05-12 09:00 - ₢A_AAb - n

Refresh pluml sentry hallmark to k260512090045-8798c6a4 (pentacle-publisher topology image).

### 2026-05-12 09:00 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512090035-c8b218ef carrying the pentacle-publisher topology reframe (entry-port iptables block deleted, ip_forward lifted).

### 2026-05-12 09:00 - ₢A_AAb - n

Topology-level reframe of the entry port: move port publish from sentry (dual-network) to pentacle (single-network), and delete the entry-port iptables block from sentry. The architectural flaw being repaired: making sentry the publisher coupled the gatekeeper role with a multi-network disambiguation problem that has no portable Compose/runtime API. Pentacle is single-attached (enclave only); Docker has one network to target, no disambiguation. Pentacle and bottle share a network namespace, so the publish lands on the bottle's listener directly. Compose change: ports directive moves from sentry to pentacle with asymmetric form ${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_ENCLAVE} (host 7999 → in-namespace 8000 for srjcl, 7999 → 8080 for pluml). The asymmetric form is necessary because without sentry's PREROUTING DNAT the publish itself must carry the port translation. Sentry script change: the entire entry-mode if-block at lines 117-142 deleted (PREROUTING DNAT, RBM-FORWARD ACCEPT, POSTROUTING MASQUERADE — all three become moot when sentry is not in the inbound path). ip_forward enablement was previously latent in the entry-mode block; lifted to an unconditional position at RBJp2 since sentry forwards on behalf of the enclave whenever access mode permits, independent of entry mode. Removed the redundant ip_forward set in the access-mode-enabled branch (was duplicated). Security posture: ifrit's net-dnat-entry-reflection invariant ('enclave-internal sources MUST NOT reach sentry entry port') is preserved trivially — sentry no longer has an entry port. Sentry's egress-gatekeeper role unchanged (default route, DNS, FORWARD/EGRESS rules for outbound traffic all untouched). Next: kludge sentry vessel, refresh hallmarks on srjcl + pluml + tadmor (all three exercised in parallel verification), run the three fixtures in parallel.

### 2026-05-12 08:58 - ₢A_AAb - n

Correct the memo's experiment protocol in response to coding-agent clarifying questions. (1) Port mapping was wrong: said the directive value was 'unchanged' (7999:7999), but without sentry's PREROUTING DNAT translating 7999→8000(/8080), the publish itself must translate. Corrected to asymmetric form `"${RBRN_ENTRY_PORT_WORKSTATION}:${RBRN_ENTRY_PORT_ENCLAVE}"` — for srjcl `7999:8000`, for pluml `7999:8080`. (2) Added new Step 3: lift `ip_forward=1` enablement out of the entry-port block in rbjs_sentry.sh to an unconditional earlier line, so kernel-forward state is independent of which access-mode iptables block runs. The current entry-port block at line 121-122 enables ip_forward as a side-effect; deletion leaves pluml (access_mode=disabled) with ip_forward=0 because pluml's access-mode block doesn't touch ip_forward. Lifting normalizes kernel state across access modes; functional behavior unchanged (pluml outbound remains iptables-blocked). (3) Expanded hallmark refresh from 'srjcl and pluml' to 'every nameplate exercised in the test iteration' — sentry vessel is shared across srjcl/pluml/tadmor/moriah/ccyolo, so any unrefreshed nameplate in the iteration runs against the OLD sentry image and its pass/fail validates the old image rather than the new one. For the three-fixture iteration (srjcl + pluml + tadmor), refresh all three. (4) Split verification into iteration-cycle (three-fixture, early signal) and canonical-cycle (full pristine gauntlet on both platforms, declaration gate). Memo principle and topology reframe conclusion unchanged; this is housekeeping on the experiment protocol following coding-agent triage.

### 2026-05-12 08:54 - ₢A_AAb - n

Incorporate Round 4 research into the memo's pentacle-publisher topology section. Round 4 (cross-runtime robustness investigation) validates the topology with two operationalizable caveats: (1) lifecycle coupling — pentacle-recreated-alone can break bottle's joined network namespace per docker/compose#10263; mitigation is treat pentacle+bottle as one lifecycle unit (`docker compose restart pentacle bottle`, possibly enforced via rbob_charge/rbob_quench guards); (2) listener source-IP non-portability — the topology fixes publish-target determinism, NOT source-IP fidelity, so bottle's listener sees runtime-dependent peer addresses (bridge gateway / proxy / rootlesskit `10.0.2.100` / pasta-preserved / etc.); if original-client IP ever matters for future entry-mode use cases, add an application-layer proxy with PROXY protocol or `X-Forwarded-For`. Replaces the placeholder 'Cross-runtime portability' subsection with a full validity matrix (Docker Compose v2/Engine supported; `ports:` on namespace guest rejected at engine level as 'conflicting options: port publishing and the container type network mode' — defensive property; Docker Desktop supported; `podman compose` provider-dependent; `podman-compose` Python supported-with-caveat; Podman native pod strongly supported with explicit 'You must not publish ports of containers in the pod individually, but only by the pod itself'). Adds listener source-IP table covering all nine runtime/proxy contexts. Adds Round 4-recommended security envelope refinement: drop `CAP_NET_ADMIN` from bottle (current `cap_drop: [NET_RAW]` is insufficient for the namespace-sharing topology — without NET_ADMIN drop, bottle could alter pentacle's port-publish rules in the shared netns). Adds pod-equivalent posture comparison (Kubernetes pods use same model intentionally; Compose `network_mode: service:X` is single-host approximation; for Docker-first deployment Compose form is more portable; for Podman-native deployment a Podman pod is semantically cleaner; project keeps Compose as primary, evaluates Podman-pod variant later as future-work). Adds four empirical observations to the experiment protocol (Round 4 specified): (A) confirm guest-publishing rejection via `docker inspect ... PortBindings` showing bottle null/empty and pentacle expected; (B) record source-IP visibility at bottle's listener via tcpdump for cross-runtime data; (C) probe pentacle-restart-alone behavior to confirm/refute docker/compose#10263 hazard in our deployment context; (D) observe startup window (port advertised before listener bound) to validate whether the existing `RBRN_BOTTLE_READINESS_DELAY_SEC=120` on srjcl is still needed post-reframe or can be reduced. Adds Step 3 to experiment protocol: optional same-change-set addition of NET_ADMIN to bottle's `cap_drop`. Updates Hypothesis posture from 'architecturally strong but empirically pending' to 'research-validated with two operationalizable caveats, empirically pending'. Extends References with 7 new Round 4 URLs (Compose Spec, docker/compose#10263, Compose create.go raw, podman-compose(1), containers/podman-compose, podman-run(1), Kubernetes networking concepts).

### 2026-05-12 08:21 - ₢A_AAb - n

Capture two rounds of web research into the memo's Post-Diagnosis Research section. First round established documented-vs-observed status for the original presumptions (Docker's alphabetical port-publish target selection is empirical-only, `default`-name magic is not documented as primacy, Compose `priority:` does not select port-publish target, long-form `ports:` exposes no `container_ip`, macOS Desktop hairpin DROP is observed-only); identified Podman rootless source-IP rewriting through rootlesskit (`10.0.2.100`) as load-bearing for any source-CIDR-based fix; surfaced `gw_priority` (Compose 2.33.1+) as a Compose-level alternative to BBABE's sentry-side default-route override. Second round source-traced Docker's alphabetical behavior through compose-go `NetworksByPriority()` (priority + lexicographic tie-break) and Docker Compose's `primaryNetworkKey` derivation in `pkg/compose/create.go` — chain traced up to the final 'Compose primary network → port-publish DNAT target' hop, which remains undocumented; source-traced Netavark's DNAT chain emission via `src/firewall/nft.rs`; established `iptables-extensions(8)` `-m conntrack --ctstate DNAT` virtual state as a documented mainline netfilter primitive providing a strictly stronger FORWARD invariant than source-CIDR exclusion. Refined Fix Shape upgrades RBM-FORWARD from source-CIDR exclusion to conntrack DNAT-state matching (PREROUTING retains source-CIDR as classification predicate since no conntrack state exists yet at PREROUTING time); expands rootless overlap guardrail from one CIDR to three (`10.0.2.100/32`, `10.0.2.0/24`, `127.0.0.0/8`) with rationale; adds runtime source-IP probe because Docker Engine and Desktop do not document target-container source-IP visibility; flags IPv6 published-port ingress as out-of-scope but tracked. Adds Source-Rewrite Address Visibility cross-runtime table and Refined Fix Per-Runtime Coverage table covering nine runtime contexts. Updates Open Empirical Questions (items 1/3/4 partially-resolved or documented; adds new items for Docker Engine/Desktop source-IP visibility, IPv6 behavior, post-restart reachability). Extends References with 14 primary-source URLs from the two research rounds. Memo principle ('sentry takes ownership; don't trust the runtime's selection') and conclusion path (cross-platform verification gate non-negotiable) unchanged; the research refines the concrete iptables shape and adds operational guardrails.

### 2026-05-12 08:21 - ₢A_AAb - n

Revert the experimental transit→auplink rename. Stage 1 verification (rename + source-IP-exclusion fix together) showed all three fixtures green: srjcl 3/3, pluml 5/5, tadmor 54/54 including the rbtdrc_sortie_net_dnat_entry_reflection regression gate. Now we revert the rename to verify the fix is rename-independent — the 'renaming has no impact' proof. With this revert, Docker will go back to delivering port-publish via the enclave interface (alphabetically-first), and the source-IP-exclusion DNAT must allow it through purely on source-IP-not-in-enclave-CIDR grounds. If stage 2 stays green, the fix has proven its independence from Docker's interface-selection heuristic — which was the entire point of choosing source-IP-exclusion over the memo's bare -i drop or compose-rename alternatives. The .rbk/rbob_compose.yml and Tools/rbk/rbob_bottle.sh contents are now byte-identical to their pre-experiment state (just before commit 65359863); only the experiment markers in commit history remain.

### 2026-05-12 08:09 - ₢A_AAb - n

Refresh pluml sentry hallmark to k260512080941-1522dab7. Same underlying source-IP-exclusion rbjs_sentry.sh; pluml's hallmark identifier differs from srjcl's only because each kludge cycle produces a fresh timestamped identifier. Pluml and srjcl now both carry the corrected entry-port DNAT/FORWARD rules.

### 2026-05-12 08:09 - ₢A_AAb - n

Refresh srjcl sentry hallmark to k260512080911-257a027f after kludging rbev-sentry-deb-tether with the source-IP-exclusion DNAT/FORWARD rules. Required to commit before continuing to pluml's hallmark refresh (kludge guard refuses dirty tree).

### 2026-05-12 08:08 - ₢A_AAb - n

User-authored research import: Post-Diagnosis Research section added to the regression memo. Committing as-is at user direction to clear the kludge guard's clean-tree precondition. Content is the user's; not reviewed line-by-line in this commit.

### 2026-05-12 08:07 - ₢A_AAb - n

Apply source-IP-exclusion repair to sentry's entry-port DNAT and RBM-FORWARD rules. Replaces the broken -i ${RBJ_UPLINK_IF} interface proxy with the actual semantic invariant: ! -s ${RBRN_ENCLAVE_BASE_IP}/${RBRN_ENCLAVE_NETMASK}. Three reasons this form is correct rather than the memo's bare -i drop: (1) the active ifrit sortie net-dnat-entry-reflection (registered at rbida_attacks.rs:344, dispatched in tadmor at rbtdrc_crucible.rs:2252) asserts that enclave-internal sources MUST NOT reach the entry port through sentry's DNAT — the shipped -i UPLINK_IF rule passed this only by interface accident, and a bare -i drop would silently regress the case; source-IP exclusion preserves the invariant explicitly in nameplate vocabulary. (2) Syntax precedent exists in the same script at line 154-156 where MASQUERADE uses ! -d on the same CIDR variables, so the new rules introduce no new patterns. (3) Extends BBABE's 'sentry takes ownership of its networking' precedent to the entry-port half: instead of trusting Docker's choice of port-publish delivery interface (alphabetical fallback per the compose header comment), the rule expresses what it actually means — entry-port traffic originating outside the enclave subnet — using only project-controlled constants. srjcl and pluml share the same sentry vessel + hallmark (c260511164345-r260511234349), so one kludge cycle covers both. Comment added to the rule block points at rbob_compose.yml's header for context on why the proxy was wrong. Next steps: kludge sentry vessel, refresh hallmarks on both nameplates, run srjcl fixture (expect green: rename + fix), run tadmor suite (regression gate for net-dnat-entry-reflection), then revert the experimental rename and re-run srjcl (proves fix is rename-independent — the renaming-no-impact proof of correctness).

### 2026-05-12 07:51 - ₢A_AAb - n

EXPERIMENT continuation (revert after observation): restore srjcl's RBRN_BOTTLE_READINESS_DELAY_SEC from 120 back to 30 (the original value introduced in ₢BBAAt commit 0a850d57). The 120 bump in ₢A_AAP commit 28d1411f was a 'Jupyter takes longer to bind' hypothesis from before the port-publish-routing root cause was understood. Now that the compose-rename experiment confirms the regression was a routing problem and not a readiness problem, we predict 30s is still sufficient because Jupyter has always bound its listener well within 30s — the curl was failing because DNAT didn't match the delivered packet, not because Jupyter wasn't listening. This re-run at delay=30 with the rename in place either: (a) passes — confirming the routing fix is robust at the original timing budget and the 120 bump was masking; or (b) fails on the connectivity case — meaning there's a genuine cold-conjure readiness issue independent of routing, in which case the 120 bump would need to land permanently regardless of routing fix choice. Required nameplate-side commit per the charge tabtarget's uncommitted-changes guard.

### 2026-05-12 07:46 - ₢A_AAb - n

EXPERIMENT (revert after observation): rename compose network 'transit' → 'auplink' so it sorts alphabetically before 'enclave'. Memo memo-20260512-srjcl-port-publish-regression.md asserts Docker's port-publish target is the alphabetically-first attached network — extrapolated from BBABE's empirically-established alphabetical rule for default-route assignment, but not independently verified for port-publish. This experiment falsifies-or-confirms that extrapolation as a binary outcome: if alphabetical hypothesis holds, srjcl jupyter-connectivity goes green (HTTP 200) with the renamed network because the shipped DNAT (-i ${RBJ_UPLINK_IF}) and RBM-FORWARD ACCEPT (-i uplink -o enclave) rules now match the path Docker actually delivers packets on; if false, curl stays at HTTP 000 and we need a different theory of the regression. Two files touched: (1) .rbk/rbob_compose.yml renames the network at top-level declaration (line 99) and at sentry's services.networks ref (line 33), plus header-comment update; (2) Tools/rbk/rbob_bottle.sh:345 collision-guard allowlist swaps 'transit' → 'auplink' so the guard recognises the renamed network as ours. No image rebuild required — sentry's rbjs_sentry.sh does interface discovery by IP (against RBRN_ENCLAVE_SENTRY_IP), never reads docker network names, so the baked entrypoint is unaffected. Test: tt/rbtd-r.FixtureRun.srjcl.sh; expected to flip from HTTP 000 (broken in baseline) to HTTP 200 if alphabetical-ordering hypothesis is causal for port-publish target selection. Edits are surgical and explicitly tagged with ₢A_AAb experiment marker in both files for trivial revertability.

### 2026-05-12 07:36 - Heat - S

sentry-entry-port-dnat-forward-rescope

### 2026-05-12 07:35 - ₢A_AAP - n

Capture the srjcl jupyter-connectivity regression diagnosis surfaced by the AAP gauntlet acceptance run. The memo records: symptom (HTTP 000 from curl localhost:7999/lab against pristine-conjured srjcl crucible); root cause (Docker port-publishes via alphabetically-first attached network, which is the enclave — packets arrive on eth0, but rbjs_sentry.sh:124-126 scopes the PREROUTING DNAT to eth1 via ${RBJ_UPLINK_IF}, so DNAT matches 0 packets); lineage (₢BBABC commit 6d836814 renamed sentry's services.networks.default → transit which removed the load-bearing 'default' magic that made compose pick the right gateway and port-publish target by accident; ₢BBABE commit ed898867 caught and fixed the default-route half of this same fragility, but the port-publish half was missed); cross-platform empirical confirmation (linux Docker engine 28.x and macOS Docker Desktop 28.x, both against branch tip 28d1411f); macOS-specific second-order blocker (after relaxing the DNAT scope, RBM-FORWARD's eth1-ingress/eth0-egress ACCEPT still drops the hairpinned post-DNAT path — linux's parallel investigation reported HTTP 200 from DNAT-half alone but the macOS evidence shows the FORWARD half is empirically necessary too); fix shape (sentry-side, both halves, drop ${RBJ_UPLINK_IF} from both PREROUTING DNAT at line 125 and RBM-FORWARD ACCEPT at line 129; consistent with BBABE's 'sentry takes ownership of its networking' precedent; requires sentry vessel re-kludge + nameplate hallmark refresh on srjcl and pluml since the sentry image is shared); sibling risk (pluml shares the sentry image and entry-mode-enabled posture — same bug will surface there); files involved (rbjs_sentry.sh lines 124-130, rbob_compose.yml lines 32-37, rbtdrc_crucible.rs line 1620); verification gate (rbtdrc_srjcl_jupyter_connectivity green via full pristine gauntlet on BOTH platforms, non-negotiable). The memo is structured to be self-contained for a clean-chat mount in a separate pace — it points at sources rather than restating them where possible, but captures enough empirical detail (per-interface PREROUTING log counters, both platforms' DNAT outputs, FORWARD chain dump) to support implementation without re-probing the running crucibles.

### 2026-05-11 19:07 - ₢A_AAP - n

Bump srjcl's RBRN_BOTTLE_READINESS_DELAY_SEC from 30 to 120. Surfaced during the AAP gauntlet acceptance run: pristine-lifecycle reached srjcl after 54/54 tadmor cases passed, but rbtdrc_srjcl_jupyter_connectivity got HTTP 000 (curl couldn't establish TCP) against a freshly-conjured Jupyter image. Re-running the fixture in isolation reproduced HTTP 000 every time. The jupyter process was up inside the bottle (ps aux confirmed at +1min into the container's lifetime, bound to 0.0.0.0:8000), and the 30s readiness delay did execute per the rbob_charge log line 'Waiting 30s for bottle service readiness' — but a freshly-conjured Jupyter image takes longer than 30s to bind its HTTP listener. Triage class: grammar — the value introduced in ₢BBAAt (commit 0a850d57) was calibrated against cached/warm Jupyter images and is insufficient for the pristine-cutover path the gauntlet exercises. 120s is the operator-chosen value from the 60/90/120 menu, providing strong margin for the cold-conjure path. RBRN_BOTTLE_READINESS_DELAY_SEC survives marshal-zero (which only blanks hallmark fields), so this value persists across the next depot regen cycle. The charge tabtarget refused to proceed with the file uncommitted ('Nameplate has uncommitted changes: .rbk/srjcl/rbrn.env — commit before charging'), forcing this commit before fixture re-run. Pluml carries the same 30s value and will need a parallel bump if its connectivity case fails the same way later in the gauntlet.

### 2026-05-11 16:23 - ₢A_AAP - n

Ignore .claude/scheduled_tasks.lock — runtime artifact from Claude Code's ScheduleWakeup machinery, parallel to the existing .claude/settings.local.json and .claude/settings.json entries. Surfaced when rblm_zero's working-tree-clean precheck refused to proceed at the start of the AAP gauntlet acceptance run because the file was untracked.

### 2026-05-11 16:18 - Heat - T

regenerate-depot

### 2026-05-11 16:17 - Heat - r

moved A_AAP to first

### 2026-05-11 16:13 - ₢A_AAa - W

Added bounded transient-network retry to the OAuth token-mint POST and completed diagnostic surfacing across the token-mint family. Centralized the transient-curl-exit classification in a new stateless rbgu_curl_status_is_transient_predicate that rbgu_http_json now delegates to; promoted the shared retry policy (3 attempts × 3s) to RBGC_HTTP_TRANSIENT_RETRY_{ATTEMPTS,SLEEP_SEC} so both retry sites reference one documented decision. Three openssl failure paths in zrbgo_build_jwt_capture (base64url-encode helper shared by header/claims, RSA256 sign, signature base64 encode) now buc_warn captured openssl stderr before returning 1, finishing the diagnostic discipline started in ₢A_AAS. Stale-by-mount: the rbcc retry-aversion counter-precedent flagged in the docket no longer exists, resolved by deletion. Fast suite 98/98 green; service-tier OAuth path exercised on first live director or retriever call.

### 2026-05-11 16:12 - ₢A_AAa - n

Complete the OAuth retry-and-diagnostics pace. Add bounded transient-network retry to zrbgo_exchange_jwt_capture: curl POST is now wrapped in a loop that retries on curl exits 7/28/35/56 (connection refused, timeout, TLS error, recv failure) up to RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS=3 with RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC=3 between attempts, returning 1 on non-transient curl exits or retry-budget exhaustion so the outer JWT-propagation loop in rbgo_get_token_capture composes cleanly. The transient-class decision is centralized in a new rbgu_curl_status_is_transient_predicate (rbgu_Utility.sh) — stateless, no sentinel, so any module can call it regardless of kindle order. rbgu_http_json (its previous owner) now delegates to the predicate and replaces its local magic numbers (3 attempts, 3s sleep) with the new RBGC constants, so both retry sites share one documented policy and 'what counts as transient' lives in exactly one place. Diagnostic-surfacing in zrbgo_build_jwt_capture extends the discipline started in ₢A_AAS to three openssl failure paths that previously returned 1 silently: the base64url-encode helper zrbgo_base64url_encode_capture (shared between header and claims encoding), the RSA256 sign openssl dgst call, and the signature base64 encode. All three now capture openssl stderr to ZRBGO_OPENSSL_STDERR_FILE and buc_warn the captured output before returning 1 — a key-parse error, openssl misconfigure, or signing failure now reports its cause above the wrapper's 'Failed to get OAuth token' buc_die. Counter-precedent investigation: the rbcc retry-aversion comment flagged in the original docket no longer exists in rbcc_Constants.sh (resolved by deletion since the docket was written, not by decision). Architectural shape (shared predicate vs duplicated case vs generalize rbgu_http_json), family scope (token-mint only vs include RBRA-load), and constants placement (promote vs leave magic) all settled via design conversation on mount. Fast suite 98/98 green; service-tier OAuth verification deferred — first call through rbgu_http_json or rbgo on any live op will exercise the new code paths.

### 2026-05-11 15:52 - Heat - r

moved A_AAP to last

### 2026-05-11 15:52 - Heat - T

aak-acceptance-tP-macos-linux

### 2026-05-11 15:29 - Heat - f

silks=rbk-10-mvp-regen-retest-depot

### 2026-04-26 10:38 - ₢A_AAP - n

AAU passthrough completion: thread _RBGA_ARK_BASENAME_{IMAGE,ABOUT,DIAGS} into the bind/mirror jq block in zrbfd_mirror_submit. AAU updated three jq blocks (rbfd conjure stitcher; rbfv graft combined about+vouch; rbfv multi-mode vouch) but missed this fourth one — bind/mirror has its own combined-steps jq composition at L1516 (mirror step + about steps), not shared with conjure. Surfaced via T1 four-mode re-run after the reliquary refresh: conjure_lifecycle GREEN (AAU + reliquary fix verified end-to-end), graft_lifecycle GREEN, kludge_lifecycle GREEN, but bind_lifecycle failed at GCB step #1 'discover-platforms' with stderr '_RBGA_ARK_BASENAME_IMAGE missing'. Build 79f3dc0a-23df-4d86-b599-d8a54c507740: mirror-image SUCCESS (12s), discover-platforms FAILURE (78s, exit 1), syft-per-platform / build-info-per-platform / assemble-push-about all stuck QUEUED. The shared about-pipeline scripts (rbgja01-discover-platforms.py:90-91, rbgja02-syft-per-platform.sh:24, rbgja04-assemble-push-about.sh:26) require IMAGE/DIAGS/ABOUT basenames — exactly what conjure's stitcher emits at L724-726. Fix is symmetric: 3 --arg + 3 substitutions added to bind block (post-zjq_timeout, after _RBGA_DOCKERFILE_CONTENT). bash -n clean. Triage class: AAU-scope grammar bug, fix-inline per docket. Bind verification re-run pending.

### 2026-04-26 10:38 - ₢A_AAP - n

AAU passthrough completion: thread _RBGA_ARK_BASENAME_{IMAGE,ABOUT,DIAGS} into the bind/mirror jq block in zrbfd_mirror_submit. AAU updated three jq blocks (rbfd conjure stitcher; rbfv graft combined about+vouch; rbfv multi-mode vouch) but missed this fourth one — bind/mirror has its own combined-steps jq composition at L1516 (mirror step + about steps), not shared with conjure. Surfaced via T1 four-mode re-run after the reliquary refresh: conjure_lifecycle GREEN (AAU + reliquary fix verified end-to-end), graft_lifecycle GREEN, kludge_lifecycle GREEN, but bind_lifecycle failed at GCB step #1 'discover-platforms' with stderr '_RBGA_ARK_BASENAME_IMAGE missing'. Build 79f3dc0a-23df-4d86-b599-d8a54c507740: mirror-image SUCCESS (12s), discover-platforms FAILURE (78s, exit 1), syft-per-platform / build-info-per-platform / assemble-push-about all stuck QUEUED. The shared about-pipeline scripts (rbgja01-discover-platforms.py:90-91, rbgja02-syft-per-platform.sh:24, rbgja04-assemble-push-about.sh:26) require IMAGE/DIAGS/ABOUT basenames — exactly what conjure's stitcher emits at L724-726. Fix is symmetric: 3 --arg + 3 substitutions added to bind block (post-zjq_timeout, after _RBGA_DOCKERFILE_CONTENT). bash -n clean. Triage class: AAU-scope grammar bug, fix-inline per docket. Bind verification re-run pending.

### 2026-04-26 10:16 - ₢A_AAP - n

Refresh reliquary across all 13 vessels under r260426100632 (newly inscribed via tt/rbw-dI). Surfaced during AAP T1 service-suite run: four-mode conjure_lifecycle and bind_lifecycle failed at zrbfd_preflight_reliquary's HEAD on `<prefix>reliquaries/r260425082412/skopeo:r260425082412` — the old reliquary's skopeo package exists but lacks the stamp tag (rbw-irr's exists-yes is package-level not tag-level, a soft mismatch worth a separate look). Both Cloud Build modes failed identically; both local modes passed. Triage: not an AAU-passthrough bug — precheck fired before any AAU code path ran. Real cause was depot-state drift: r260425082412 partially-jettisoned (skopeo tag missing), r260327172456 never inscribed at all (referenced by 4 vessels but absent from rbxc-reliquaries/). Fix: inscribed fresh r260426100632 (Cloud Build SUCCESS at 1m16s; rbw-dY validates 6/6 tools present at the new stamp). Yoked 10 conjure vessels via rbw-dY; manually re-pointed rbev-bottle-plantuml (mode=bind) and rbev-graft-demo (mode=graft) — yoke refuses non-conjure vessels but rbfd_mirror line 1384 and rbfd_graft's about/vouch step images both consume the reliquary, so the 'no reliquary needed' policy is itself the defect (already flagged in ₢A_AAS commit b602b7c5 as follow-up; not yet captured as a formal itch — capture pending end-of-pace). Verification re-run held until user signal.

### 2026-04-26 09:57 - ₢A_AAU - W

Threaded RBGC_ARK_BASENAME_* across the Director→Cloud Build boundary so renaming a basename Director-side stops silently desyncing producer/consumer. Three Director jq blocks (rbfd conjure stitcher; rbfv graft combined about+vouch; rbfv multi-mode vouch) gained --arg/substitution pairs for the basenames each consumer group needs (_RBGY_*: image+attest+diags; _RBGA_*: image+about+diags; _RBGV_*: image+vouch+attest). Nine consumer scripts (7 bash, 2 python) replaced bare 'image'/'vouch'/'about'/'attest'/'diags' literals with ${_RBG?_ARK_BASENAME_*} reads, each gating on the new env var at its required-inputs check. Pouch unchanged — it's GCB input, never a producer write. Verification surfaced four side-effect notches: kludged sentry + bottle for tadmor (depot's k260416190047-... was missing from local AND from GAR), spook-fixed rbob_charged_predicate which had been left behind in the ₢A_AAA-AAC RBRR_RUNTIME_PREFIX rollout (compose -p was using bare ${RBRN_MONIKER}), and extracted ZRBOB_PROJECT constant to eliminate the bug class (six hand-rolled ${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER} sites collapsed to one). Fast suite 97/97 green; tadmor crucible fixture 54/54 green; srjcl/pluml fixtures need their own kludge bootstrap (same depot-state pattern) — deferred. Cloud Build burn-in (the actual end-to-end signal for this work) deferred to AAP per docket. Spawned ₢BAAAH kludge-aware-charge-prereq on ₣BA to address the diagnostic gap that made the sentry-kludge dance discoverable only by hand: the existing missing-vouch warning suggests summon for k-prefixed hallmarks which is guaranteed to fail because they never go to GAR. That pace will mint the RBGC_HALLMARK_PREFIX_{KLUDGE,CONJURE,BIND,GRAFT} family (currently four bare literals at rbfd:1137,1293,1391,1648) and discriminate at the charge-time prerequisite check.

### 2026-04-26 09:43 - ₢A_AAU - n

Extract ZRBOB_PROJECT constant for the runtime-prefixed compose project name (was ${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER} hand-rolled at six sites). Eliminates the bug class that produced the prior spook — a future site author cannot now forget to thread the prefix because the concept has a name. Sites collapsed: ZRBOB_SENTRY/PENTACLE/BOTTLE container names, ZRBOB_NETWORK enclave name, zrbob_compose -p arg, rbob_charged_predicate -p arg. Constant placed at the top of the existing kindle block with a docstring naming it as the authoritative project identity from which container/network names derive. Verified with a charge → rbw-cic probe (exit=0) → quench cycle on tadmor: containers labeled rbxr-tadmor-{sentry,pentacle,bottle}, networks rbxr-tadmor_{enclave,default}, probe matches — all consistent with the new single source of truth.

### 2026-04-26 09:30 - ₢A_AAU - n

Spook fix surfaced during AAU crucible verification: rbob_charged_predicate at rbob_bottle.sh:345 missed the RBRR_RUNTIME_PREFIX rollout from the AAA-AAC prefix-introduction work. The probe queried 'compose -p ${RBRN_MONIKER}' (e.g. 'tadmor') while every other compose invocation — including the actual charge at line 202 — uses 'compose -p ${RBRR_RUNTIME_PREFIX}${RBRN_MONIKER}' (e.g. 'rbxr-tadmor'). Verified: rbw-cic.CrucibleIsCharged.sh tadmor now exits 0 against the live charged crucible (was exit 1 despite three healthy containers). Single-line addition; no other use of bare ${RBRN_MONIKER} as a compose project name in the file.

### 2026-04-26 09:26 - ₢A_AAU - n

Bottle kludge for tadmor: drive RBRN_BOTTLE_HALLMARK=k260426091932-4dcb0aa2 (rbev-bottle-ifrit-tether). Pairs with the prior sentry kludge to give tadmor a fully-local crucible state for AAU verification.

### 2026-04-26 09:19 - ₢A_AAU - n

Sentry kludge for tadmor: drive RBRN_SENTRY_HALLMARK=k260426091808-33111a93. Local-built hallmark replaces the missing-from-GAR k260416190047-e29568f4 sentry artifact that blocked crucible suite at AAU verification.

### 2026-04-26 09:17 - ₢A_AAU - n

Thread RBGC_ARK_BASENAME_* across the Director→Cloud Build boundary so renaming a basename Director-side stops silently desyncing producer/consumer. Three Director jq blocks (rbfd conjure stitcher, rbfv graft combined about+vouch, rbfv multi-mode vouch) gain --arg/substitution pairs for the basenames each consumer group needs (_RBGY_*: image+attest+diags; _RBGA_*: image+about+diags; _RBGV_*: image+vouch+attest). Nine consumer scripts replace bare 'image'/'vouch'/'about'/'attest'/'diags' literals with ${_RBG?_ARK_BASENAME_*} reads (bash) or os.environ via existing require_env helper (python rbgja01, rbgjv02). Each consumer adds the new env var to its required-inputs gate. Pouch unchanged — it's GCB input, never crosses the boundary as a write. Fast suite green (97/97). Crucible suite blocked separately by missing local sentry vouch artifact (depot state, unrelated to this pace) — surfaced as needing a separate spook; Cloud Build burn-in (the actual end-to-end check for this work) deferred to AAP.

### 2026-04-26 09:05 - Heat - r

moved A_AAa before A_AAJ

### 2026-04-26 09:05 - ₢A_AAS - W

AAS theurge-coverage-catchup wrap (option i — batch_vouch GREEN gate deferred to forthcoming ₣BA pace).

### 2026-04-26 07:40 - ₢A_AAS - n

Expand RBRR_CLOUD_PREFIX/RUNTIME_PREFIX negative coverage in regime-validation: per-failure-mode decomposition. Each prefix's docket-spec'd failure modes (uppercase / no-trailing-hyphen / too-long, against the rbrr_regime.sh:99-103 enforce regex ^[a-z][a-z0-9-]*-$ length 2..=11) get their own case, replacing the prior single-shot bad_cloud_prefix=BAD- and bad_runtime_prefix=acme that each exercised only one mode by accident. Six new cases total; rv-rbrr-negative section now 14 (was 8). RCG Constant Discipline applied: extracted RBTDRF_VAR_RBRR_CLOUD_PREFIX/RUNTIME_PREFIX consts (5+/4+ refs after expansion) and threaded through both the negative cases and the rs_rbrr_nonempty_prefix smoke (named-arg format string for the smoke's ${{var}} bash interpolation). Fast suite GREEN: enrollment-validation 47/47, regime-validation 27/27 (was 21), regime-smoke 8/8 (was 7), handbook-render 15/15. Manifest update item from the AAS docket already complete from prior work — RBTDRM_COLOPHON_SUMMON / PLUMB_FULL / PLUMB_COMPACT / VOUCH and RBTDRM_FIXTURE_BATCH_VOUCH all present in rbtdrm_manifest.rs with REKON + JETTISON registered.

### 2026-04-26 07:32 - ₢A_AAS - n

Redesign rbtdrc_fourmode_graft_lifecycle assertion: replace docker-run + 'BusyBox container is running!' stdout match (test-expectation defect — that string only comes from rbev-busybox/Dockerfile:15 CMD echo, while graft's source busybox:latest defaults to sh which exits clean with no stdout, so the assertion was unreachable) with a layer-DiffID round-trip comparison via new helper rbtdrc_docker_layers_capture (docker inspect --format={{json .RootFS.Layers}}). Initial implementation used Image-ID equality per the chat brief; first run revealed the premise had shifted: notch 73f3f818's graft platform-normalization (rbgja01 _normalize_variant) intentionally collapses the multi-arch index (vnd.oci.image.index.v1+json, 9535B) to a single-platform manifest (vnd.oci.image.manifest.v1+json, 610B) on push, changing the config blob and thus the Image ID. RootFS.Layers DiffIDs (uncompressed-content SHA256) are byte-preserved across the round-trip — confirmed empirically: source busybox:latest and wrested image both report layer sha256:900b8517...db2ebbf despite divergent Image IDs. Layer-DiffID is the right fingerprint: tests graft's actual contract (payload byte-preservation), robust to intentional envelope normalization, still catches silent re-pack of layer content. Rejected re-considerations: run-with-marker (functional but blind to silent re-pack), exit-0-only (loses round-trip signal), wrapper-Dockerfile (contradicts graft↔kludge boundary). Verification: tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_graft_lifecycle GREEN. Kludge case re-run pending — failed first attempt because rbfd_kludge:1213 enforces clean working tree, blocked by this in-flight edit; will re-run post-notch.

### 2026-04-26 07:00 - ₢A_AAS - n

Surface OAuth exchange failure diagnostics in zrbgo_exchange_jwt_capture: emit curl stderr verbatim on network-failure path, dump redacted response body (token/secret/key/password keys filtered) on missing-access-token path, both via buc_warn before return 1. The 11 call sites' wrapper buc_die 'Failed to get Director OAuth token' continues to fire; the actual cause now appears above it. Surfaced during AAS verification re-run when bind_lifecycle SingleCase failed at rekon with the opaque wrapper — diagnostic excavation required deep-dig through theurge-preserved per-invoke temp dirs (burv/invoke-NNNNN/temp/temp-*/rbgo_curl_stderr.txt) to recover the actual curl 28 connect-timeout. The retry-on-transient half is open and slated to AAa for design conversation.

### 2026-04-26 06:59 - Heat - S

rbgo-oauth-retry-and-diagnostics-completion

### 2026-04-25 10:08 - Heat - S

cloudbuild-machinetype-ab-bench

### 2026-04-25 10:04 - ₢A_AAS - n

Multi-layer about-artifact extraction in zrbfc_gar_extract_artifact (test-driven by the post-98b80250 SingleCase four-mode conjure verification, which now ran end-to-end and exposed a separate plumb_full assertion failure: stdout missing the 'Dockerfile' marker).

### 2026-04-25 09:51 - ₢A_AAS - n

Post-b602b7c5 conjure HTTP 400 fix in rbfd_stitch_build_json + SingleCase non-crucible context-init defect surfaced while setting up verification.

### 2026-04-25 09:40 - Heat - S

bcg-shellcheck-cross-module-discipline

### 2026-04-25 09:21 - Heat - S

imageops-fixtures

### 2026-04-25 09:20 - Heat - S

imageops-precheck

### 2026-04-25 09:20 - Heat - S

imageops-broaden

### 2026-04-25 09:16 - ₢A_AAS - n

Bind+graft AAK-shape and platform-normalization fixes (test-driven from the post-b602b7c5 four-mode service-suite re-run that surfaced two failures distinct from the conjure preflight fix).

### 2026-04-25 08:59 - ₢A_AAS - n

Sweep enshrine completion across remaining conjure vessels: re-enshrined rust:slim-bookworm (build caf819b9, ifrit-forge slot, anchor 5ae2d2ef98 → caaf9ca7ac — upstream digest moved since the prior enshrine 2026-04-04) and debian:bookworm-slim (build c6112e1f, sentry-deb-tether slot, anchor f06537653a → f9c6a2fd2d — upstream digest also moved); both wrote new anchors to their owning vessel's rbrv.env, leaving sibling vessels (ifrit-tether shares rust anchor with ifrit-forge; sentry-deb-airgap shares debian anchor with sentry-deb-tether) pointing at the dead anchors. Manually synced the two siblings to the new anchors per the handbook principle 'multiple vessels sharing the same base image anchor need only one enshrine' (rbhoda:176) — sharing only works if siblings stay in lock-step, which the rbw-dE write-path doesn't enforce (it only updates the invoked vessel). All three depot anchors (busybox-latest-1487d0af5f, rust-slim-bookworm-caaf9ca7ac, debian-bookworm-slim-f9c6a2fd2d) are now live in GAR under the post-AAK ${RBRR_CLOUD_PREFIX}enshrines/<anchor>:<anchor> shape. Combined with reliquary r260425082412 yoked into all 7 vessels (busybox + 6 from notch 160ceec7), conjure mode preflight should now pass. Bind and graft four-mode failures are unrelated to enshrines: bind dies in rbgjm01-mirror-image.sh which still uses the pre-AAK ${vessel}:${tag} layout (needs swap to ${HALLMARKS_ROOT}/${hallmark}/image:${hallmark} per rbgja02/04/rbgjv03); graft dies at sbom-arm64.json missing in about Step #3, separate from AAK. The pre-AAK vs post-AAK layout split in rbgjm01 makes the missing-substitution symptom into a layout-completion fix. ifrit-airgap RBRV_IMAGE_1_ANCHOR remains empty by design — it sources from rbev-bottle-ifrit-forge as a vessel-to-vessel image (RBRV_IMAGE_1_ORIGIN=rbev-bottle-ifrit-forge), no upstream enshrine; rbev-busybox-airgap-negative-canary RBRV_IMAGE_1_ANCHOR is also empty by design (canary tests pass-through behavior). About to re-run four-mode fixture to verify conjure now passes; bind/graft still expected red.

### 2026-04-25 08:55 - ₢A_AAS - n

Yoke remaining 6 conjure vessels to reliquary r260425082412 (live four-mode/enshrine surfaced that the b602b7c5 notch only re-pointed busybox/bottle-plantuml/graft-demo; all other conjure vessels still pinned the dead r260327172456). Direct symptom: rbw-dE rbev-bottle-ifrit-forge died at 'Reliquary not found: r260327172456' during the post-b602b7c5 enshrine sweep. All six were yokable via rbw-dY (RBRV_VESSEL_MODE=conjure across the set). rbw-dY validated r260425082412 in GAR before each rewrite (six redundant validations — no fix needed, the cost is negligible vs the safety of the gate). Stamp transition: r260327172456 → r260425082412 across forge, ifrit-tether, sentry-tether, sentry-airgap, ifrit-airgap, busybox-airgap-negative-canary. No code changed; this is operational re-pointing only. Tree dirty for 6 rbrv.env, captured here. Enshrine of busybox already succeeded standalone (build 517fb360, 18s wall); forge/sentry-tether enshrines pending after this notch — they need the new reliquary because rbfd_enshrine's preflight HEADs the reliquary canary at zrbfd_preflight_reliquary path. Out-of-scope discoveries flagged for AAP/follow-up: (a) rbgjm01-mirror-image.sh:21 still uses pre-AAK shape ${vessel}:${hallmark}${suffix} instead of post-AAK ${HALLMARKS_ROOT}/${hallmark}/image:${hallmark} per rbgja02/rbgja04/rbgjv03 — bind ordain mirror step dies at '_RBGA_ARK_SUFFIX_IMAGE: unbound variable'; the substitution is missing in rbfd:1496-1511 AND the script itself uses the wrong layout, so the right fix is updating rbgjm01 to the canonical hallmarks_root shape and dropping the suffix substitution rather than adding it; (b) graft about pipeline (build 122db554) failed at Step #3 'assemble-push-about' with 'sbom-arm64.json not found' — graft-demo vessel is single-platform (linux/amd64 per RBRV_CONJURE_PLATFORMS, though graft-demo is graft mode so different vessel envelope) but the about Dockerfile.meta unconditionally COPYs sbom-${TARGETARCH}${TARGETVARIANT}.json for both arch outputs, separate from AAK migration.

### 2026-04-25 08:39 - ₢A_AAS - n

Repo asset: drawio SVG re-export with minor layout adjustment (1-line diff), captured alongside heat A_ work. No associated context in this officium — same shape as prior re-exports under A8AAK 2b7f4c0a.

### 2026-04-25 08:37 - ₢A_AAS - n

AAK-migration completion in foundry preflight + inscribe submitter (test-driven fix surfaced by service-suite four-mode failures): zrbfl_inscribe_submit was missing _RBGN_RELIQUARIES_ROOT substitution and over-prefixed _RBGN_RELIQUARY (passing ${RBRR_CLOUD_PREFIX}${stamp} instead of bare stamp), causing the inscribe step script to die in <1s on `set -u` unbound-var dereference at the docker-push DEST construction; symmetrically zrbfd_preflight_reliquary was looking under the pre-AAK path shape `${RBRR_CLOUD_PREFIX}<stamp>/docker:latest` with the wrong tag, so even with a freshly-inscribed reliquary every conjure/bind/graft ordain would 404 in the canary HEAD. Both functions now use the canonical post-AAK shape: inscribe pushes to `${RBGL_RELIQUARIES_ROOT}/<stamp>/<tool>:<stamp>` and preflight HEADs `${RBGL_RELIQUARIES_ROOT}/<stamp>/${RBGC_RELIQUARY_TOOL_DOCKER}:<stamp>` (canary tool name pulled from the same constant zrbfc_resolve_tool_images consumes). Preflight buc_die recommendation also extended with the missing yoke step — operators following the prior message (inscribe → ordain) would have re-failed because the vessel still pinned the dead stamp; the corrected sequence is inscribe → yoke <new-stamp> <vessel> → ordain. Vessel rbrv.env updates: rbev-busybox yoked via rbw-dY (only conjure mode is yoke-eligible per current rbfl_yoke restriction); rbev-bottle-plantuml and rbev-graft-demo manually re-pointed because rbfl_yoke refuses non-conjure modes — though rbfd lines 1325/1576 prove bind and graft DO consume the reliquary (mirror skopeo + about/vouch step images), so the yoke restriction is itself a defect (separate from this fix; flagged for follow-up). Reliquary stamp transitioned r260327172456 → r260425082412 (fresh inscribe under the corrected submitter; verified via gcloud build log showing all 6 tools pushed at canonical paths and rbfl_yoke's 'Reliquary valid — all 6 tool images present' confirmation on busybox). bash -n clean on both edited shell files. Adjacent observations not addressed in this notch: the handbook's Step 1 inscribe procedure (rbhodf_director_first_build.sh:123-154) is positive-path only — no recovery guidance, oversells reliquary durability ('stays in the depot until you choose to refresh it'), and frames yoke as single-vessel; rbfl_yoke's mode-gate for bind/graft contradicts rbfd's actual reliquary consumption in those modes; bind and graft vessels should be yoke-eligible. These are AAP-territory or follow-up itches, not this pace's scope. Live four-mode + batch-vouch suite re-run pending after this notch (kludge case requires clean tree).

### 2026-04-25 08:09 - Heat - r

moved A_AAJ to last

### 2026-04-25 08:01 - ₢A_AAS - n

Theurge coverage catchup: 3 fast-suite prefix cases (90→93 green), four-mode extended with summon/plumb_full/rekon (conjure) and plumb_compact/jettison/post-jettison-rekon (bind), new batch-vouch standalone fixture exercising pending→vouched two-pass via plant-by-vouch-jettison.

### 2026-04-25 07:56 - Heat - S

cloudbuild-ark-basename-passthrough

### 2026-04-23 15:00 - ₢A_AAT - W

Decomposed rbtdrc_four_mode_supply_chain (Tools/rbk/rbtd/src/rbtdrc_crucible.rs) from a 15-step monolithic case with cross-step dependencies into four independent per-mode lifecycle cases: rbtdrc_fourmode_conjure_lifecycle, _bind_lifecycle, _graft_lifecycle, _kludge_lifecycle. Each case is fully self-contained — own setup, own observation, own teardown, no inter-case state, no ordering requirement. Engine runs them in array order but order carries no load. Reslated the docket before starting because the original Out-of-scope clause explicitly forbade converting sequential dependency into parallel-safe cases — the reshape was the whole point after the user rejected ctx-threaded sequential decomposition. Design choice on graft setup: Option B (docker pull busybox:latest as graft source) rather than Option A (conjure then graft) — preserves graft's 'local image → GAR' semantic while keeping the case independent and fast. Added rbtdrc_docker_pull helper alongside the existing docker_run/inspect/rmi wrappers. Graft lifecycle: docker pull busybox:latest → ordain with BURE_TWEAK_NAME=threemodegraft and BURE_TWEAK_VALUE=busybox:latest → wrest → docker run asserting 'BusyBox container is running!' (safe because we know the source payload) → rmi → abjure --force. Conjure lifecycle: ordain rbev-busybox → wrest → run+assert → rmi → abjure. Bind lifecycle: ordain rbev-bottle-plantuml → wrest → rmi → abjure (no run — plantuml runtime isn't a CLI-output assertion; the pluml fixture covers runtime semantics, this fixture observes supply-chain completion). Kludge lifecycle: kludge rbev-busybox → inspect image+vouch refs exist locally → run+assert → rmi (no GAR footprint, no abjure). Each case reads its own RBF_FACT_HALLMARK/GAR_ROOT/ARK_STEM from the ordain or kludge BURV output (all four modes emit all three facts per rbfd_FoundryDirectorBuild.sh:1160-1166,1279-1281,1361-1367,1650-1656). RBTDRC_SECTIONS_FOUR_MODE now has a four-element cases array. Tally cross-checks (tally-pre asserting three hallmarks co-resident, tally-post asserting all three gone) dropped from the fixture per the reslated docket — tally's multi-hallmark enumeration is exercised live by other flows, and a dedicated tally fixture is a separate pace if coverage is wanted. Manifest cleanup: RBTDRM_COLOPHON_TALLY removed from four-mode's required colophons in rbtdrm_manifest.rs. Dead code removed: RBTDRC_FACT_CONSEC_INFIX constant (used only by the dropped tally cross-checks), RBTDRI_BURV_OUTPUT_SUBDIR and RBTDRM_COLOPHON_TALLY imports from rbtdrc_crucible.rs. Step-marker filename scheme changed from step-NN-* to per-case NN-phase.txt (e.g., conjure's 01-ordain.txt, 02-wrest.txt, 03-run.txt, 04-abjure.txt, 05-passed.txt) so single-case reruns produce clean per-case trace output. No rbtdri_Context struct fields added — the independent-case shape makes all ctx plumbing unnecessary, confirming the reslated docket's 'Out of scope: Context struct field additions' clause. New tabtarget: tt/rbtd-s.SingleCase.four-mode.sh (previously absent; mirrors tadmor's three-line stub exactly, making case reruns possible via tt/rbtd-s.SingleCase.four-mode.sh <case-name>). Verified at end: chmod +x applied, the tabtarget lists all four cases correctly under the 'four-mode-integration' section header. The rbtdtm_accepts_valid_fourmode_manifest unit test self-validates because rbtdtm_manifest_for builds the test manifest by calling rbtdrm_required_colophons(fixture) — no hardcoded list, so the TALLY removal propagated through cleanly. Verification: tt/rbtd-b.Build.sh clean (1.02s, no cargo warnings), tt/rbtd-t.Test.sh 53/53 theurge unit tests green, tt/rbtd-s.TestSuite.fast.sh 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) matching baseline. Not run from this officium: service and crucible suites (no live GCP credentials, container runtime not guaranteed), live four-mode full-run against a charged crucible, live single-case reruns. The per-case invocations are mechanically identical to the operations the old monolithic fixture used (ordain, wrest, docker run, docker rmi, abjure, kludge) — the refactor relocates case boundaries without changing any underlying invocation, so live behavior is preserved by construction. Multi-officium discipline observed: rbtdrc_crucible.rs, rbtdrm_manifest.rs, and the new tabtarget are this officium's work; no other files touched.

### 2026-04-23 15:00 - ₢A_AAT - n

split four-mode fixture into four independent per-mode lifecycle cases

### 2026-04-23 14:04 - Heat - S

four-mode-case-split

### 2026-04-23 13:55 - ₢A_AAR - W

Jettison locator grammar rewritten to post-AAK form in a single file (Tools/rbk/rbfl_FoundryLedger.sh lines 326-395). Locator parsing switched from first-colon split (%%:* / #*:) to last-colon split (%:* / ##*:), mirroring the rbfr_wrest idiom adopted in AAK Notch 2a — required because post-AAK package paths contain slashes (hallmarks/<H>/<basename>, reliquaries/<date>/<tool>) and the pre-AAK bare-moniker URL construction would mis-target any multi-segment path. URL `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_moniker}/manifests/${z_tag}` rewritten to `${ZRBFC_REGISTRY_API_BASE}/${RBRR_CLOUD_PREFIX}${z_pkg_path}/manifests/${z_tag}` so the package path's sub-namespacing survives to the DELETE endpoint. Local `z_moniker` renamed to `z_pkg_path` throughout the function body. Docstring example updated from pre-AAK `rbev-busybox:20251231T160211Z-img` to post-AAK `hallmarks/H/image:H` (mirrors rbfr_wrest's docstring style). Validation prose: required-param die message 'moniker:tag' → 'package-path:tag', case-label error 'Expected moniker:tag' → 'Expected package-path:tag', empty-check 'Moniker is empty' → 'Package path is empty'. Docket-flagged Rust call-site audit was a no-op — grep for RBTDRM_COLOPHON_JETTISON / jettison across Tools/rbk/rbtd/src/ returned zero hits; theurge has no jettison consumers, so the 'check for any that pass pre-AAK locator strings' concern in the docket was speculative and there was nothing to fix. Scope came in narrower than the docket anticipated: a single file, no Rust, no tabtarget or zipper churn (channel already empty per the docket's cross-cutting note). BCG review performed pre-commit at user's explicit request: braced quoted expansion throughout, patterns in %:* and ##*: are literal colon-and-star so the BCG §381-417 'inner expansion quoted separately' rule is satisfied trivially (no variable-in-pattern glob risk), no new $() or stderr captures introduced since the edit is pure string manipulation, and local declarations stayed without -r to match the surrounding function's existing style (z_locator, z_force, z_status_file are all plain local in this function). Verification: bash -n clean on rbfl_FoundryLedger.sh; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) matching baseline. Service and crucible suites not run from this officium (no live credentials, container runtime not required for a parsing-only change), and no live jettison against a known hallmark was exercised (no disposable test ark available in this session); live verification deferred to downstream — the parsing is mechanically equivalent to rbfr_wrest which is already exercised in live pull flows. Attribution clean: notched as commit 7ede2fd6 with only Tools/rbk/rbfl_FoundryLedger.sh in the file list; rbk-claude-tabtarget-context.md and rbz_zipper.sh were dirty at record time from another concurrent officium and were correctly left out of this pace's commit per multi-officium discipline. Heat state: with AAR closed, the two post-AAK orthogonal bugs that AAL flagged (AAQ rekon-hallmark-catalog and AAR jettison-locator-grammar) have split — AAR landed, AAQ remains the next actionable rough pace awaiting decision between the three shape options (raw hallmark listing / hallmark-indexed basename listing / dissolve) that its docket enumerates.

### 2026-04-23 13:55 - ₢A_AAQ - W

Rewrote rbfl_rekon (rbw-ir) from pre-AAK <prefix><moniker>/tags/list Docker V2 query to post-AAK GAR packages.list categorical-layout enumeration. Signature collapsed from (moniker) to (hallmark); emits three-column BASENAME/EXISTS/PACKAGE-PATH table over the six canonical basenames (image, about, vouch, attest, pouch, diags) with yes/no presence plus absolute package path or (absent). Implementation consumes the shared zrbfc_list_hallmarks_capture helper (landed in A_AAO b5603dc6), filters its full enumeration to rows for the target hallmark using a bash-3.2 space-padded substring membership idiom, retains Director auth. BCG-clean: braced quoted expansion, local -r on single-assign locals, two-line capture for z_token, test not [[, trailing-newline guard on while read, no heredocs, no pipelines in $(). Zipper RBZ_REKON_IMAGE help text updated to match new semantic; tabtarget context regenerated (one-line diff on Image row). Docket-flagged RBTDRM_COLOPHON_REKON audit was a no-op — grep across Tools/rbk/rbtd/src returned zero hits, so no Rust call-site changes needed (crucible has no rekon consumer). Fast suite 90/90 green (47+21+7+15). Service and crucible suites not run from this officium; live rbw-ir deferred to downstream but rides on enumeration infrastructure already exercised by rbfl_tally and rbfv_batch_vouch. Multi-officium overlap: rbfl_FoundryLedger.sh edit landed under A_AAR's 7ede2fd6 (jettison-grammar pace was staging the same file concurrently); AAQ's own 0fde7757 notch captures the residual files (rbz_zipper.sh + regen). All work is live; attribution is cross-cutting per the documented multi-officium discipline.

### 2026-04-23 13:55 - ₢A_AAQ - n

Post-AAK endpoint rewrite for rbfl_rekon: dropped pre-AAK <prefix><moniker>/tags/list Docker V2 registry query in favor of the GAR packages.list categorical-layout enumeration via shared helper zrbfc_list_hallmarks_capture (landed in ₢A_AAO b5603dc6). Signature changed from (moniker) to (hallmark); output reshaped to the three-column docket-specified BASENAME/EXISTS/PACKAGE-PATH table iterating the six canonical basenames (image, about, vouch, attest, pouch, diags) with yes/no presence and absolute package path or (absent). Director auth posture preserved. Implementation hews to rbfl_tally's shape (authenticate, enumerate, filter, classify) with the classifier collapsed to per-basename membership since this is a single-hallmark diagnostic rather than tally's fleet-wide classifier; present-basenames accumulator uses the space-padded substring membership idiom (case match on " ${z_present} ") to stay bash 3.2 compatible without associative arrays. BCG review pre-commit: braced-quoted expansion throughout; local -r on the single hallmark param; loop-synthesized z_line/z_h/z_b and z_canon/z_mark/z_path use plain local per Exception 2; two-line capture pattern for z_token; test not [[; no heredocs; while IFS= read has trailing-newline guard; zrbfc_list_hallmarks_capture invoked bare (not inside $()) matching rbfl_tally's usage; no new stderr captures introduced (the shared helper owns its curl/jq error handling). Zipper help text for RBZ_REKON_IMAGE updated from 'Raw GAR tag listing for a vessel package' to 'List ark basenames present under a hallmark's GAR subtree' reflecting the new per-hallmark semantic; tabtarget context regenerated via tt/rbw-MG.MarshalGenerate.sh produced a one-line diff on the Image row. Multi-officium overlap: my rbfl_FoundryLedger.sh edit (the rbfl_rekon function rewrite) was staged in the working tree when ₢A_AAR's jjx_record for jettison-grammar fired; the record named rbfl_FoundryLedger.sh in its file list so my rekon function body rode along under commit 7ede2fd6's A_AAR attribution. git show HEAD:Tools/rbk/rbfl_FoundryLedger.sh confirms rbfl_rekon at line 597 with the new (hallmark) signature, zrbfc_list_hallmarks_capture consumption, and three-column printf table. Same overlap phenomenon AAK Notch 2a and AAL both documented — not destructive, work is live in the tree, only commit-log attribution is cross-cutting. Items explicitly landing under AAQ attribution: (1) rbz_zipper.sh help-text tweak, (2) rbk-claude-tabtarget-context.md regen. Out-of-scope docket items verified as no-ops: RBTDRM_COLOPHON_REKON does not exist in Tools/ (grep zero hits across the theurge crate) — the docket speculated the four-mode fixture might exercise rekon, but crucible has no rekon consumer, so no Rust call-site changes were needed. Verification: bash -n clean on both edited files; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) matching baseline. Service and crucible suites not run from this officium (no live credentials / container runtime not guaranteed); live rbw-ir against a known hallmark deferred to downstream verification — the enumeration helper it consumes is already exercised live by rbfl_tally and rbfv_batch_vouch, so the new code path rides on already-validated infrastructure. Not updated as out-of-scope: RBSIR-image_rekon.adoc spec (the docket listed no spec-alignment task; spec can be swept in an RBS follow-up pace if needed).

### 2026-04-23 13:52 - ₢A_AAR - n

Rewrite rbfl_jettison locator grammar to post-AAK form. Locator parsing switched from first-colon split (%%:* / #*:) to last-colon split (%:* / ##*:), matching the rbfr_wrest idiom adopted in AAK Notch 2a — needed because post-AAK package paths contain slashes (hallmarks/<H>/<basename>, reliquaries/<date>/<tool>) and first-colon split combined with a bare-moniker URL would mis-target any multi-segment path. URL construction changed from ${RBRR_CLOUD_PREFIX}${z_moniker}/manifests/${z_tag} to ${RBRR_CLOUD_PREFIX}${z_pkg_path}/manifests/${z_tag} so the package path's sub-namespacing survives intact. Local renamed z_moniker → z_pkg_path throughout. Docstring example updated from pre-AAK rbev-busybox:20251231T160211Z-img to post-AAK hallmarks/H/image:H (mirrors rbfr_wrest's docstring style). Validation prose: 'moniker' → 'package path' in the required-param die message, the case-label 'Expected moniker:tag' → 'Expected package-path:tag', and the empty-check 'Moniker is empty' → 'Package path is empty'. Docket-flagged Rust call-site audit was a no-op: grep for RBTDRM_COLOPHON_JETTISON across Tools/rbk/rbtd/src/ returned zero hits — jettison has no theurge consumers, so the 'check for any that pass pre-AAK locator strings' line in the docket was speculative and there was nothing to fix. BCG review pre-commit: braced quoted expansion throughout; patterns in %:* and ##*: are literal colon-and-star (no variable-in-pattern glob risk per BCG §381-417); no new $() or stderr captures introduced (edit is pure string manipulation); local/local style matches surrounding function (plain local, not local -r — consistent with existing z_locator, z_force, z_status_file). Verification: bash -n clean on rbfl_FoundryLedger.sh; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) matching baseline. Not run from this officium: service suite (no live GCP credentials), crucible suite (container runtime not required for this parsing-only change), live jettison against a known hallmark (no test ark available in this session) — live verification deferred to downstream; the parsing is mechanically equivalent to rbfr_wrest which is already exercised in live pull flows.

### 2026-04-23 13:50 - Heat - S

theurge-coverage-catchup-aak-aal-aao

### 2026-04-23 13:43 - ₢A_AAD - W

Migrated payor OAuth credentials from flat RBRR_SECRETS_DIR/rbro.env to RBRR_SECRETS_DIR/payor/rbro.env, matching the role-subdirectory pattern already established for RBRA credentials. Minted RBCC_role_payor tinder constant; updated RBDC_PAYOR_RBRO_FILE derivation in rbdc_DerivedConstants.sh; added payor to the kindle mkdir -p block alongside governor/retriever/director. Added new payor migration block with fail-loud on both-exist. Retrofitted the existing RBRA migration loop (lines 43-56) with a parallel both-exist guard for policy consistency — both migrations now hard-fail instead of silently leaving both files. Doc/spec sweep across 8 files updated literal '~/.rbw/rbro.env' to '~/.rbw/payor/rbro.env' (RBSGS, RBSPI, RBSRO, RBSPR, RBS0, CLAUDE.consumer.md) and spec 'as rbro.env' to 'as payor/rbro.env' (RBS0, RBSRR), plus handbook display text in rbhpr_refresh.sh (literal payor matches surrounding RBRR_SECRETS_DIR literal style) and kindle-constant refactor in rblm_cli.sh:100 — user caught that my initial literal 'payor/rbro.env' broke consistency with sibling lines 63/132 that use ${RBCC_role_governor}/${RBCC_rbra_file} pattern; fixed to ${RBCC_role_payor}/${RBCC_rbro_file}. Verified via three tiers: (1) migration mv ran cleanly against the live ../station-files/secrets/ directory with 600 perms preserved; (2) both-exist guards fire correctly for both payor (touch flat rbro.env + ValidateOauthRegime produces 'Migration ambiguous: both...') and governor RBRA retrofit (touch rbra-governor.env + ValidateOauthRegime produces 'Migration ambiguous for role governor: both...'); (3) live Payor OAuth round-trip via rbw-dl.PayorListsDepots authenticated and returned 1 COMPLETE depot from the migrated payor/rbro.env path. Fast suite 90/90 green throughout. Notched as 1381068b mid-pace; wrap captures no additional source changes (verification was runtime-only against external station-files). Governor reset cycle (aM/aC/aK) deferred to user since it reissues RBRA files destructively and the retrofitted RBRA guard was already verified in isolation; not a hard gate for AAD completion.

### 2026-04-23 13:41 - ₢A_AAL - W

AAL vessel-param-drop pace shipped clean but asymmetrically attributed due to multi-officium overlap. Real code changes delivered: (1) rbfc_vessel_for_hallmark_capture helper minted in rbfc_FoundryCore.sh, authenticates as Retriever and extracts vouch ark to read .vessel from vouch_summary.json; single home prevents reinvention at each call site; BCG-mandated _capture suffix (not the bare name the docket proposed) because $() invocation requires it. (2) zrbfc_plumb_core refactored to hallmark-only signature (hallmark, mode) — removed the test-for-empty-vessel single-arg fallback that was AAK-era forward-compat scaffolding; vessel is now always resolved from vouch via the new helper, then rbfc_require_vessel_sigil + zrbfc_load_vessel for RBRV_VESSEL_MODE. (3) rbfc_plumb_full and rbfc_plumb_compact collapsed from (vessel, hallmark) to (hallmark) and pass through to the simplified plumb_core. (4) rbfl_abjure signature dropped $1 vessel — zrbfc_resolve_vessel + zrbfc_load_vessel calls removed (post-AAK they were dead weight; the registry path is hallmark-indexed and vessel doesn't participate); param-shift $2 hallmark → $1 and $3 force → $2; docstring updated. (5) rbfr_summon signature dropped $1 vessel — removed rbfc_require_vessel_sigil validation call (was gating on the dropped arg); removed path-prefix strip (accept-directory-path normalization no longer needed); final success prose changed from 'Hallmark summoned: ${vessel}/${hallmark}' to 'Hallmark summoned: ${hallmark}' since the bulleted per-artifact retrieval lines below show the full hallmark subtree paths (vessel would require a full vouch-ark extract just for display, not worth it). (6) rbtdrc_crucible.rs:2424 Rust call-site updated: rbw-fA invocation loop tuple destructure changed from (label, vdir, consec) to (label, consec), args slice from [vdir, consec, --force] to [consec, --force]; conjure_dir/bind_dir/graft_dir locals preserved because ORDAIN (rbw-fO) and KLUDGE (rbw-fk) still take vessel. Multi-officium overlap: commits 1-4 above landed in A_AAO's b5603dc6 because that officium's jjx_record file list named rbfc_FoundryCore.sh and rbfl_FoundryLedger.sh — my WIP edits to those files were already in the index at that moment and rode along under A_AAO's attribution. Same phenomenon AAK Notch 2a's paddock documented for rbhodf. Not destructive — all work is live in the repo, just the commit-log attribution is cross-cutting. Items 5-6 landed explicitly under AAL as commit addae3a1. Scope refinement from docket's expected 9 affected tabtargets: real vessel-param drops hit only rbw-fA/fs/fpf/fpc. The rbw-fV (batch_vouch) and rbw-ft (tally) functions were stubbed by AAK Notch 2b and A_AAO reimplemented them in the same b5603dc6 overlap window. The rbw-iJ/ir/iw image-family tabtargets already take single-arg locators from AAK Notch 2a. Two orthogonal post-AAK bugs discovered during the grep audit but flagged out-of-scope and slated as fresh paces: ₢A_AAQ rekon-hallmark-catalog (rbfl_rekon still hits <prefix><moniker>/tags/list — pre-AAK vessel-indexed package grammar that no longer exists; needs endpoint rewrite, likely consuming A_AAO's new zrbfc_list_hallmarks_capture helper) and ₢A_AAR jettison-locator-grammar (rbfl_jettison parses first-colon split into moniker:tag and constructs <prefix><moniker>/manifests/<tag> URL — needs last-colon split like rbfr_wrest adopted in AAK Notch 2a, and URL fix to interpret the path as containing slashes). Both slated first in heat per user request. Verification at close: git show HEAD:rbfc_FoundryCore.sh confirms all AAL structural changes present at expected line numbers (helper at 679, plumb_core hallmark-only at 716, plumb_full single-arg at 1332); rbfl_FoundryLedger.sh:397 shows the simplified abjure signature; bash -n clean on all edited files; theurge crate builds clean with 53 Rust unit tests passing; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) matching baseline with no regression. tt/rbw-MG.MarshalGenerate.sh regen produced zero diff on rbk-claude-tabtarget-context.md because channel declarations for affected colophons were already empty at the zipper level — the generator shows colophon/frontispiece/channel but not underlying function signatures, so the change was invisible at that layer. Service and crucible suites not run from this officium (no live credentials / container runtime not guaranteed); deferred to downstream verification. Paddock and heat state advanced: AAL's docket-spec verification items all green except the service/crucible suites which require live infrastructure.

### 2026-04-23 13:41 - ₢A_AAO - W

Replaced two stubbed function bodies (rbfl_tally, rbfv_batch_vouch) with hallmark-enumeration implementations against the new GAR categorical layout, then landed the two followups that emerged from the rewrite. Main implementation (commit b5603dc6): rbfc_FoundryCore.sh added ZRBFC_HALLMARK_LIST_FILE + shared helper zrbfc_list_hallmarks_capture (single GAR REST packages.list call, jq-decoded %2F, filtered to ${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename> shape via split length==2, sorted pair output). rbfl_tally: Retriever auth, shared helper, load-then-iterate with synthetic sentinel element so the final hallmark flushes through the same boundary branch as intermediates (single flush site), state machine tracking z_prev_img/abt/vch flag bits and z_prev_bns basenames, classifier (vouched=img+abt+vch, pending=img+abt without vch, incomplete=else), three-column HALLMARK/HEALTH/BASENAMES table, rbw-fV/rbw-fA recommendations. rbfv_batch_vouch: Director auth, shared helper, two-pass shape — pass-1 state machine accumulates pending into a bash array AND writes forensic file, pass-2 iterates the array (not file — critical BCG load-then-iterate compliance since rbfv_vouch issues curl/source/Cloud Build that would silently consume loop stdin), per-hallmark extract about ark via zrbfc_gar_extract_artifact, read .vessel_name from build_info.json (narrow AAL-anticipation inside this one function, not the broader vessel-column-everywhere work AAL defers), resolve vessel_dir, call rbfv_vouch; fail-fast via buc_die at broken hallmark rather than summary-count-masking. BCG discipline throughout: bash 3.2 compatible (refactored initial declare -A draft to sorted-pair state machine with sentinel), load-then-iterate, explicit ':>' truncation, no pipelines inside $(), all $(<file) validated. Followups (commit fbba7777): (F1) deleted dead RBCC_FACT_CONSEC_INFIX at three sites (rbcc_Constants.sh readonly+comment, RBS0 mapping entry, RBS0 ==== Hallmark Check Fact Map ==== section); full-tree grep zero residuals. (F2) aligned RBSCL (tally) and RBSDV (batch_vouch) specs to landed shape — both had described pre-AAK per-vessel tag-enumeration; rewrote RBSCL modeled on RBSAA's GAR-REST-API idiom (removed Enumerate Vessels step, fixed classifier from 'about+vouch=vouched' to 'img+abt+vch=vouched', replaced per-vessel tables with single three-column table, dropped {rbcc_fact_consec_infix} typed_output); rewrote RBSDV (removed vessel enumeration + pre-check/skip-with-warning pattern, added two-pass shape with BCG rationale note, added narrow vessel-from-about-metadata framing explicitly marked AAL-anticipation, added 'no mode pre-check' historical-change note); RBS0 surface fixes at lines 1237/1253/1836 (stale 'across all vessel repositories', 'across all regime vessels', 'Vouch column yes/no'). Fast suite 90/90 green at both checkpoints. bash -n clean.

### 2026-04-23 13:41 - ₢A_AAO - n

Land the two AAO-close followups flagged in b5603dc6. (F1) Delete the dead RBCC_FACT_CONSEC_INFIX constant — removed the readonly+comment block from rbcc_Constants.sh (was keyed on vessel+infix+hallmark for per-vessel-hallmark presence files; the old rbfl_tally emitted one fact per pair but AAO's rewrite dropped the emission since no fixture consumed them, and vessel is no longer in the GAR path anyway), plus the :rbcc_fact_consec_infix: mapping entry and the ==== Hallmark Check Fact Map ==== section (anchor + definition) from RBS0. Full-tree grep returns zero residuals after edit. (F2) Spec alignment for RBSCL (tally) and RBSDV (batch_vouch) — both specs described the pre-AAK per-vessel tag-enumeration shape. RBSCL fully rewritten modeled on RBSAA's GAR-REST-API idiom: removed Enumerate Vessels step (vessel no longer in path), removed per-vessel iteration, added single packages.list call framing with shared-helper note, fixed classifier to match landed triplet (vouched=img+abt+vch, pending=img+abt without vch, incomplete=else — old spec had two-basename vouched which was wrong), replaced per-vessel-tables step with single three-column HALLMARK/HEALTH/BASENAMES table, removed the {rbcc_fact_consec_infix} typed_output block, added completion prose noting AAL-scope vessel restoration. RBSDV fully rewritten: removed vessel enumeration + per-vessel iteration, removed the 4-state vouched/vouchable/unvouchable/incomplete classification, removed the separate SLSA provenance pre-check step (AAO deliberately dropped skip-with-warning in favor of fail-fast via rbfv_vouch's buc_die), added two-pass shape (pass-1 collect-pending-in-memory, pass-2 dispatch) with BCG rationale note about loop stdin consumption, added narrow vessel-from-about-metadata framing explicitly marked as AAL-anticipation scoped to this function only, added an explicit 'no mode pre-check' closing note documenting the historical change. RBS0 surface fixes: {rbtgo_director_vouch} intro at 1237 (was 'across all vessel repositories... conjure requires SLSA pre-check'), {rbtgo_hallmark_tally} intro at 1253 (was 'across all regime vessels'), tally-display note at 1836 (was 'Vouch column yes/no' which didn't match the three-state HEALTH column). Fast suite 90/90 green (47+21+7+15). bash -n clean on rbcc_Constants.sh. Closes both followups that b5603dc6 flagged.

### 2026-04-23 13:37 - Heat - S

jettison-locator-grammar

### 2026-04-23 13:36 - Heat - S

rekon-hallmark-catalog

### 2026-04-23 13:35 - ₢A_AAL - n

AAL vessel-param drop — residual files after multi-officium overlap. The helper mint (rbfc_vessel_for_hallmark_capture), the zrbfc_plumb_core hallmark-only refactor, and the rbfl_abjure vessel-arg drop all rode along into A_AAO's commit b5603dc6 because that officium's jjx_record file list named rbfc_FoundryCore.sh and rbfl_FoundryLedger.sh — my WIP edits to those files were staged at that moment and committed under A_AAO's attribution. Same phenomenon the AAK Notch 2a paddock already documented for rbhodf. Tree confirms: git show HEAD:rbfc_FoundryCore.sh contains rbfc_vessel_for_hallmark_capture at line 679, zrbfc_plumb_core's hallmark-only signature, and rbfc_plumb_full/rbfc_plumb_compact single-arg collapse. Two files remain for explicit AAL attribution: (1) rbfr_FoundryRetriever.sh — rbfr_summon signature drops $1 vessel (was vessel, hallmark → now just hallmark); removed the rbfc_require_vessel_sigil validation call that was gating on the dropped arg; removed the path-prefix strip that normalized vessel path→sigil; final success prose changed from 'Hallmark summoned: ${z_vessel}/${z_hallmark}' to just 'Hallmark summoned: ${z_hallmark}' since the bulleted per-artifact retrieval lines below already show the full hallmark subtree paths. (2) rbtd/src/rbtdrc_crucible.rs:2424 — the crucible four-mode fixture was invoking rbw-fA with [vdir, consec, --force] matching the pre-AAL two-arg abjure signature; updated to [consec, --force]. Loop-tuple destructure changed from (label, vdir, consec) to (label, consec) with conjure_dir/bind_dir/graft_dir locals preserved because they're still used by ORDAIN (line 2236, 2257, 2293) and KLUDGE (line 2308) which take vessel and are unaffected by AAL. Scope refinement from docket's expected 9 affected tabtargets: real vessel-param drops hit only rbw-fA/fs/fpf/fpc. The rbw-fV (batch_vouch) and rbw-ft (tally) functions were stubbed by AAK Notch 2b and reimplemented in A_AAO; the rbw-iJ/ir/iw image-family tabtargets already take single-arg locators from AAK Notch 2b. Two orthogonal post-AAK bugs discovered but flagged out-of-scope (to be slated as follow-up paces): rbfl_rekon points at <prefix><moniker>/tags/list which is the pre-AAK vessel-package grammar (no longer valid post-migration); rbfl_jettison interprets its locator against <prefix><moniker>/manifests/<tag> similarly. Neither is a vessel-param-drop issue; both need endpoint-rewrite work along the lines of zrbfc_list_hallmarks_capture. Verification: bash -n clean on rbfr_FoundryRetriever.sh; theurge crate builds clean (cargo dev profile); 53 theurge unit tests pass; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render); tt/rbw-MG.MarshalGenerate.sh regen produced zero diff on rbk-claude-tabtarget-context.md because channel declarations for affected colophons were already '' (empty — pass all args through to function) at the zipper level — the generator shows colophon/frontispiece/channel and the underlying function signature change is invisible at that layer. Service and crucible suites not run from this officium (no live credentials / container runtime not guaranteed); deferred to downstream verification. AAL's docket-specified helper rbfc_vessel_for_hallmark exists as rbfc_vessel_for_hallmark_capture (BCG-mandated _capture suffix for $()-invoked public functions) and is consumed by zrbfc_plumb_core; abjure and summon don't route through it because abjure has no vessel-consumption post-simplification and summon's prose simplification dropped the vessel identifier rather than resolving it (the vouch ark extraction cost wasn't worth it for a display-only use).

### 2026-04-23 13:30 - ₢A_AAO - n

Replace both stubbed bodies (rbfl_tally, rbfv_batch_vouch) from AAK Notch 2b with hallmark-enumeration implementations against the new GAR categorical layout. (1) rbfc_FoundryCore.sh — added ZRBFC_HALLMARK_LIST_FILE kindle constant and new shared helper zrbfc_list_hallmarks_capture: single GAR REST GET ${ZRBFC_GAR_API_BASE}/${ZRBFC_GAR_PACKAGE_BASE}/packages?pageSize=1000, jq decode %2F, filter to ${RBGL_HALLMARKS_ROOT}/<hallmark>/<basename> shape (split length==2 excludes anything deeper or shallower), emit sorted <hallmark> <basename> pairs. Pagination deferred until evidence of >1000 packages. (2) rbfl_tally — Retriever auth, shared helper, load-then-iterate with synthetic '__SENTINEL__ __SENTINEL__' element appended to lines array so the final hallmark flushes through the same boundary branch as every intermediate one (single flush site). State machine tracks z_prev_img / z_prev_abt / z_prev_vch flag bits and z_prev_bns accumulated-basenames; classifier: vouched (image+about+vouch), pending (image+about, no vouch), else incomplete. Displays three-column table (HALLMARK, HEALTH, BASENAMES), emits buc_tabtarget recommendations to rbw-fV (pending) and rbw-fA (incomplete). Dropped the orphan 'buf_write_fact ${vessel}${RBCC_FACT_CONSEC_INFIX}${hallmark}' emission — grep confirms zero fixture consumers of RBCC_FACT_CONSEC_INFIX (only the definition in rbcc_Constants.sh:63-65 and the RBS0:96 spec entry); constant left in place out of pace scope, flagged as followup. (3) rbfv_batch_vouch — Director auth, shared helper, two-pass: pass-1 state machine accumulates pending hallmarks (image+about, no vouch) into a bash array AND writes pending list to a forensic file; pass-2 iterates the array (NOT while-read over the file — critical BCG load-then-iterate compliance since rbfv_vouch issues curl/source/Cloud Build that would silently consume loop stdin), for each pending hallmark extracts about ark via zrbfc_gar_extract_artifact, reads .vessel_name from build_info.json (option (a) per docket — narrow AAL-anticipation inside one function, not the broader vessel-column-everywhere work AAL defers), resolves vessel_dir=${RBRR_VESSEL_DIR}/<vessel>, calls rbfv_vouch. Fail-fast: rbfv_vouch's buc_die terminates batch at the broken hallmark rather than masking problems behind a summary count. BCG discipline: bash 3.2 compatible (no associative arrays — initial draft used declare -A, refactored to sorted-pair state machine with sentinel); load-then-iterate throughout; explicit : > truncation; no pipelines inside $(); all $(<file) validated; only sort and cp used from POSIX allowlist (all jq/curl go through existing rbgu_http_json helpers). Fast suite 90/90 green post-refactor. Followups captured for next pace, neither blocking: (a) dead RBCC_FACT_CONSEC_INFIX constant + RBS0 linked-term entry — candidates for deletion; (b) spec alignment in RBSCL (tally) and RBSAV (batch_vouch) — docket referenced these as AAK Notch 3 targets but AAK's 18-file sweep may or may not have covered the new shapes.

### 2026-04-23 13:26 - ₢A_AAD - n

Migrate payor OAuth credentials from flat rbro.env to payor/rbro.env subdirectory, matching the role-subdirectory pattern already used for RBRA credentials. Mint RBCC_role_payor tinder constant, update RBDC_PAYOR_RBRO_FILE derivation, add payor to mkdir -p block, add new payor migration block with fail-loud on both-exist, and retrofit the existing RBRA migration loop with a parallel both-exist guard for policy consistency. Doc sweep across 8 files updates ~/.rbw/rbro.env literals to ~/.rbw/payor/rbro.env and spec prose from as rbro.env to as payor/rbro.env. Fast suite 90/90 green throughout. Governor reset cycle deferred to user (requires credentials).

### 2026-04-23 13:13 - ₢A_AAK - W

Delivered the full GAR categorical-layout migration across 4 notches: (1) a9e95201 Notch 1+2a — minted rbgl_GarLayout.sh as kindle-constants-only module (RBGL_HALLMARKS_ROOT / RBGL_RELIQUARIES_ROOT / RBGL_ENSHRINES_ROOT), renamed RBGC_ARK_SUFFIX_* to RBGC_ARK_BASENAME_* with values dropping the hyphen, added RBGC_GAR_CATEGORY_* and RBGC_RELIQUARY_TOOL_* constants, deleted RBGC_VOUCHES_PACKAGE, wired 7 CLI scripts and first-wave consumer modules (rbob, rbrn, rbfr summon, rbfc) to the new grammar. Original 8-function helper design rejected as BCG violation (subshell on non-_capture functions); pure-interpolation-at-callsite adopted instead. (2) 6c362750 Notch 2b — atomic cutover of the remaining producer surface: rbfd conjure/bind/graft/enshrine jq blocks with _RBGA_*/_RBGV_*/_RBGY_*/_RBGE_* substitutions threading, rbfv verify 3 jq blocks, rbfl abjure rewrite (per-suffix HEAD/DELETE dance → enumerate-then-delete via GAR REST /packages naturally tolerating graft's missing pouch), compose.yml 3 image refs reshaped, 6 rbgjb + 2 rbgja + 2 rbgjv + rbgje01 + rbgji01 Cloud Build step scripts reshaped with _RBG?_HALLMARKS_ROOT substitution, enshrine shifted to plural enshrines/ with anchor-as-path-segment. rbfl_tally and rbfv_batch_vouch stubbed with buc_die pointing to ₢A_AAO for the architecturally substantial rewrite. Grep-zero achieved on RBGC_ARK_SUFFIX_, RBGC_VOUCHES_PACKAGE, hyphen-suffix concat across all code paths. Fast suite 90/90 green. (3) 93c651f2 Notch 3 — 18 spec files swept via 5 parallel general-purpose agents dispatched against disjoint file sets: 10 from the original docket plus 7 found via grep expansion beyond the original list (RBSAB, RBSAG, RBSAK, RBSCB, RBSCC, RBSDV, RBSGS, RBSIJ, RBSTB). Heaviest rewrites: RBSAA abjure algorithm collapsed from 241 lines of per-suffix HEAD/DELETE dance to 122 lines of enumerate-then-delete against GAR REST /packages; RBSAV dissolved the vouches aggregator entirely (_RBGV_VOUCHES_PACKAGE attribute row deleted, dual-tag push grammar dropped, subtree-navigation prose added); RBSAE captured the enshrine anchor-as-path-segment shift; RBSCB rebuilt its Cloud Build tag-scheme table with full <prefix>hallmarks/<H>/<basename>:<H> shape plus a new attest row; RBSCL and RBSDV classification algorithms key off basename presence instead of tag-suffix matching. Two fixes applied after the agent pass: RBS0:3147 enshrine-namespace reference at the regime variable definition site (Agent A focused on the attestation section, missed it); RBSAK:58 awkward 'No about basename' phrasing rephrased to 'No about artifact'. Commit size 77555 bytes required size_limit override above the 50000 default (user-approved) — the 18-file sweep is homogeneous work that belongs in one logical commit to preserve cross-file consistency. (4) 739b4a35 Notch 4 — Rust-side catchup discovered during Notch 3 review: rbtdrc_crucible.rs carried its own RBTDRC_ARK_SUFFIX_{IMAGE,VOUCH} constants invisible to Notch 2b's bash-prefix grep sweep. Four call sites (graft-input wrest, kludge image/vouch local refs, final conjured-image wrest+run) were emitting invalid post-migration locators. Fixed: removed SUFFIX constants, added RBTDRC_ARK_BASENAME_{IMAGE,VOUCH} and RBTDRC_GAR_CATEGORY_HALLMARKS, rewrote the 4 sites with the new locator grammar respecting wrest's relative-to-cloud-prefix input convention (rbfr_wrest prepends RBRR_CLOUD_PREFIX itself). Also added GAR_ROOT and ARK_STEM fact emissions to rbfd_kludge for uniform per-mode fact contract (kludge was the odd one out emitting only HALLMARK). Spec alignment: RBS0:1884 (stale 'Ark stem (vessel:hallmark)' missed by Agent A) and RBSAG:114 (Agent B and C disagreed on whether the stem includes cloud prefix; bash truth matches Agent B, aligned RBSAG accordingly). Fast suite 90/90 green confirmed post-Notch-4 with no regression. Two small flagged follow-ups captured for future attention, neither blocking: (a) Agent A's shape-level 'probe' verb in RBS0 ~1832 re: hallmark_tally detection mechanism (RBSCL is the authoritative description and reads consistent); (b) the wrest locator contract asymmetry — fact_ark_stem includes cloud prefix while wrest's locator input does not — surfaces as a Rust-side conceptual tax (two different string forms from the same semantic concept, more format! calls than strictly necessary). Scope-out for AAK; a future cleanup could unify (e.g., split into RELATIVE_STEM + PREFIX facts or fold cloud prefix into GAR_ROOT). Service suite (80 cases) and crucible suite (117 cases) not run from this officium — deferred to the just-slated ₢A_AAP aak-live-depot-burn-in testing pace which will exercise the migration end-to-end against the current depot before ₢A_AAE depot regen. AAK's hard-cutover contract is genuinely complete: grep-zero across all code and spec paths, no old grammar surviving in live code, spec/code alignment maintained.

### 2026-04-23 13:09 - Heat - r

moved A_AAP after A_AAD

### 2026-04-23 13:04 - Heat - S

aak-live-depot-burn-in

### 2026-04-23 12:59 - ₢A_AAK - n

Notch 4 of ₢A_AAK — Rust-side catchup for the GAR categorical-layout migration. Discovered during Notch 3 spec review: rbtdrc_crucible.rs (the four-mode foundry integration test fixture) still carried the old hyphen-suffix grammar via its own Rust-side RBTDRC_ARK_SUFFIX_{IMAGE,VOUCH} constants, invisible to Notch 2b's grep-zero verification which targeted the bash-side RBGC_ARK_SUFFIX_ prefix. Four call sites were emitting invalid post-migration locators: wrest-for-graft input, kludge image/vouch local docker-store refs, and final conjured-image wrest+run. This would surface as crucible-suite failures since the fixture's locators no longer matched what rbfd_kludge and compose.yml actually produce; fast suite didn't catch it because rbtdrc_crucible.rs is only exercised by the crucible suite (117 cases, requires container runtime). Root cause for why Notch 2b missed it: the Rust binary defined its own constants mirroring the bash values; the 2b sweep greped for the bash-prefix symbols (RBGC_/RBGL_) and for bash-level ':${HALLMARK}-' concat patterns, neither of which touches Rust code. Fix scope: three code files plus two spec alignment fixes. (1) rbtdrc_crucible.rs: removed RBTDRC_ARK_SUFFIX_{IMAGE,VOUCH}; added RBTDRC_ARK_BASENAME_{IMAGE,VOUCH} and RBTDRC_GAR_CATEGORY_HALLMARKS matching their RBGC_ counterparts; rewrote the four call sites to emit the new locator grammar. Wrest locators (relative-to-cloud-prefix since rbfr_wrest prepends RBRR_CLOUD_PREFIX itself): 'hallmarks/<hallmark>/<basename>:<hallmark>'. Full local refs (prefixed, match what wrest pulled to local cache): '<gar_root>/<prefixed_ark_stem>/<basename>:<hallmark>'. Per-mode symmetry enforced: each mode now reads its own HALLMARK + GAR_ROOT + ARK_STEM facts and builds refs uniformly. (2) rbfd_FoundryDirectorBuild.sh: added GAR_ROOT and ARK_STEM fact emissions to rbfd_kludge to match conjure/bind/graft's fact contract — previously kludge only emitted HALLMARK, forcing Rust consumers to reconstruct the stem via string manipulation on conjure's fact. Per-mode fact emission is now uniform; downstream consumers read the same three facts regardless of mode. (3) RBS0-SpecTop.adoc:1884: fixed stale 'Ark stem (vessel:hallmark)' description that Agent A missed during Notch 3 — rewrote to match the actual emission shape '«PREFIX»hallmarks/«HALLMARK»'. (4) RBSAG-ark_graft.adoc:114: aligned ark_stem description with RBSAC's wording — Agent C described the stem without cloud prefix, Agent B described it with; the bash truth (includes prefix via RBGL_HALLMARKS_ROOT) matches Agent B. Verification: cargo build succeeded (theurge crate, dev profile); tt/rbtd-s.TestSuite.fast.sh returns 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) matching Notch 2b baseline with no regression. Crucible suite (which exercises the edited fixture) not run — requires container runtime and live GAR credentials; flagging for user-side or foray-based verification in the close summary of AAK. The hard-cutover contract is now genuinely complete: no old grammar survives in live code paths. Two flagged follow-ups from Notch 3 remain: (a) Agent A's 'probe' verb choice in RBS0 ~1832 (shape-level; RBSCL is the authoritative description so cross-reference reads consistent); (b) the wrest locator contract split — fact_ark_stem includes cloud prefix, wrest's locator input does not — surfaces as a Rust-side conceptual tax (Rust builds two different string forms from the same semantic concept). A future cleanup could unify (e.g., split into RELATIVE_STEM + PREFIX facts or fold cloud prefix into GAR_ROOT). Scope-out for AAK: the contract asymmetry doesn't produce incorrect behavior, just more format! calls. Closes ₢A_AAK's cutover-discipline contract.

### 2026-04-23 12:48 - ₢A_AAK - n

Notch 3 of ₢A_AAK — spec sweep completing the GAR categorical-layout migration. 18 spec files updated to match the Notch 2b code cutover. Heaviest change is RBSAA: the abjure algorithm description collapses from a per-suffix HEAD/DELETE dance (241 lines, 5-6 hardcoded suffixes, per-artifact existence + orphan-detection branches + individual DELETE steps) to enumerate-then-delete via GAR REST /packages (122 lines, single list-filter-loop-delete flow that naturally tolerates graft's missing pouch). RBSAV dissolves the vouches aggregator: _RBGV_VOUCHES_PACKAGE attribute row deleted entirely, dual-tag push grammar (VESSEL:HALLMARK-vouch + vouches:HALLMARK-vouch) replaced by single package under hallmarks/HALLMARK/vouch with subtree navigation for hallmark-only lookup. RBSAE reflects the enshrine anchor-as-path-segment shift — the `enshrine:anchor` tag-on-shared-package grammar becomes `enshrines/anchor:anchor` peer packages under a plural namespace, symmetric with hallmarks/ and reliquaries/. RBSCB rebuilds its Cloud Build tag-scheme table to show full <prefix>hallmarks/HALLMARK/<basename>:HALLMARK shape with a new attest row capturing the single-package-with-per-platform-tags pattern. RBSCL and RBSDV classification algorithms key off basename presence under hallmark subtrees instead of tag-suffix matching; health-state semantics (vouched/pending/incomplete) preserved. RBSDI gains `_RBGN_CLOUD_PREFIX` Cloud Build substitution docs plus the new reliquaries/<date>/<tool>:<date> path example. The remaining 12 files receive proportionally lighter edits: basename-shape tag grammar in RBSAC/RBSAS/RBSAG/RBSAK/RBSIJ/RBSCC, subtree-navigation prose in RBSAP/RBSGS replacing the vouches-superdirectory mechanism, attribute renames in RBSAB (_RBGA_ARK_SUFFIX_* → _RBGA_ARK_BASENAME_*) and RBSTB (_RBGY_ARK_SUFFIX_IMAGE → _RBGY_ARK_BASENAME_IMAGE) with their RBGC_* back-references. Two fixes applied after the agent pass: RBS0:3147 where the regime variable definition site still referenced `enshrine:«anchor»` (missed by Agent A who focused on the attestation section), and RBSAK:58 where agent output read 'No about basename, no provenance chain' — rephrased to 'No about artifact' since structural-location terminology ('basename') doesn't fit an absence statement. All 7 binding verification greps return zero: RBGC_ARK_SUFFIX_, RBGC_VOUCHES_PACKAGE, :${HALLMARK}-(image|vouch|pouch|about|diags) hyphen-suffix concat, enshrine:anchor old tag grammar, 'vouches (superdirectory|package|aggregator)' aggregator phrasing, «HALLMARK»-(image|vouch|pouch|about|diags) stale substitution grammar, and {vessel}:{hallmark}-X. Two flagged follow-ups captured but not blocking: Agent B reshaped the {rbf_fact_ark_stem} fact description to match the new grammar; if the code-side rbf_fact value actually stores something different, spec/code could drift (worth a future spot check). Agent A used verb 'probe' in RBS0 ~1832 re: tally detection mechanism without inspecting rbfl_tally implementation; RBSCL (Agent D) is the authoritative description and reads consistently. Spec-only change — no bash/Rust modified, no test suite run (Notch 2b already banked 90/90 fast-suite green). Closes Notch 3 of the 3-notch commit sequence in the groomed AAK plan; AAK's spec-side contract is now complete. Commit size 77555 bytes required raising size_limit above the 50000 default — user-approved; the 18-file sweep is homogeneous spec-prose work that belongs in one logical commit to preserve the cross-file consistency that motivated the parallel agent dispatch.

### 2026-04-23 12:18 - ₢A_AAK - n

Notch 2b (atomic cutover) of ₢A_AAK — completes the GAR categorical-layout migration that Notch 2a started. 16 files reshaped to the new <prefix>hallmarks/<H>/<basename>:<H> | <prefix>reliquaries/<D>/<tool>:<D> | <prefix>enshrines/<A>:<A> URL grammar, restoring grep-zero on RBGC_ARK_SUFFIX_/RBGC_VOUCHES_PACKAGE across all code paths (only spec files retain references; sweep deferred to Notch 3). Fast suite 90/90 green; service and crucible suites unblocked but not run from this officium (no live credentials).

### 2026-04-23 11:50 - Heat - S

tally-hallmark-enumeration-rewrite

### 2026-04-23 11:39 - ₢A_AAK - n

Notch 2a (partial cutover, infrastructure + safe-path consumers) of ₢A_AAK. 13 files landed; foundry producer/verifier/ledger and Cloud Build step scripts deferred to a follow-up Notch 2b due to session context scale. LANDED: (1) rbgl_GarLayout.sh rewritten as kindle-constants-only module exposing RBGL_HALLMARKS_ROOT / RBGL_RELIQUARIES_ROOT / RBGL_ENSHRINES_ROOT (each = cloud-prefix + category string). Original 8-function design (rbgl_hallmark_ark_path etc.) rejected per BCG line 488: '$() on functions is only permitted for _capture (with guard) or _recite' — callers would have needed $(rbgl_...) subshell invocation which is a BCG violation. Kindle constants with pure interpolation at call sites is the BCG-compliant pattern; replaces all the originally-planned helper calls. (2) rbgc_Constants.sh shape rewrite: deleted RBGC_ARK_SUFFIX_{IMAGE,VOUCH,POUCH,ABOUT,ATTEST,DIAGS} and RBGC_VOUCHES_PACKAGE; added RBGC_ARK_BASENAME_{IMAGE,VOUCH,POUCH,ABOUT,ATTEST,DIAGS} (no hyphen prefix — they are now plain basenames), RBGC_GAR_CATEGORY_{HALLMARKS,RELIQUARIES,ENSHRINES}, and RBGC_RELIQUARY_TOOL_{GCLOUD,DOCKER,ALPINE,SYFT,BINFMT,SKOPEO} (canonical tool manifest in authoritative rbgji01-inscribe-mirror.sh surfaces as named constants). (3) 7 CLI scripts (rbfc_cli, rbfd_cli, rbfl_cli, rbfr_cli, rbfv_cli, rbob_cli, rbrn_cli) source rbgl_GarLayout.sh immediately after rbgc_Constants.sh and invoke zrbgl_kindle immediately after zrbgc_kindle (ordering matters — rbgl kindle takes zrbgc_sentinel dependency for category constants). rbrn_cli places source+kindle inside the heavy survey/audit conditional branch. (4) Consumer module rewrites for sites that don't involve Cloud Build substitution plumbing: rbob_bottle.sh ZRBOB_SENTRY/BOTTLE_IMAGE/VOUCH construction uses $\{RBGL_HALLMARKS_ROOT\}/$\{HALLMARK\}/$\{RBGC_ARK_BASENAME_X\}:$\{HALLMARK\} pattern (hallmark-as-tag); rbrn_cli.sh z_sentry_img/z_bottle_img same pattern; rbfr_FoundryRetriever.sh summon rewrote to: (a) ark package paths as z_image_pkg/z_about_pkg/z_vouch_pkg locals built inline from constants, (b) HEAD URLs use package path + manifests/$\{hallmark\}, (c) pull refs use full URL with tag=hallmark, (d) locator parsing in rbfr_wrest uses last-colon split (%:* and ##*:) for path-may-contain-colons shapes, (e) simplified list-vouched-hallmarks branch replaced by die-with-pointer-to-rbw-ft (the hallmark-discovery feature needed _catalog-based rewrite — deferred as scope, tally command handles the use case). rbfc_FoundryCore.sh: zrbfc_resolve_tool_images assigns tool refs from RBGL_RELIQUARIES_ROOT + RBGC_RELIQUARY_TOOL_* constants with tag=$\{reliquary_date\} (immutable-datestamp-as-tag, matches hallmark-as-tag discipline); plumb hallmark-only resolution branch dissolves vouches-superdirectory fetch in favor of fetching hallmark's own vouch ark (vouch_summary.json still contains vessel field — verified no producer change needed); plumb about/vouch fetches use $\{RBGL_HALLMARKS_ROOT\}/$\{hallmark\}/$\{basename\} package paths with tag=hallmark. (5) BCG-compliance cleanup collateral: ZRBFC_TOOL_{GCLOUD,DOCKER,ALPINE,SYFT,BINFMT,SKOPEO} renamed to z_rbfc_tool_* across rbfc (13 sites) and rbfd (10 sites). These variables are assigned in zrbfc_resolve_tool_images (not kindle — they depend on RBRV_RELIQUARY which comes from vessel-load), so per BCG 'Mutable kindle state' rule they must be lowercase z_-prefixed (SCREAMING_SNAKE is reserved for readonly kindle constants). Initialized empty in zrbfc_kindle under a 'Tool image refs — mutable kindle state' comment. This was a LATENT pre-existing BCG violation that became my scope because I was editing those specific lines for AAK. NOT TOUCHED IN THIS NOTCH (deferred to Notch 2b): rbfd_FoundryDirectorBuild.sh 15+ sites (image_base construction line 596, per-platform attest URI loop line 602, conjure jq substitution block lines 622-692, pouch context tag line 733, bind image/vouch refs lines 1260-1261, bind image tag 1371, per-platform attest fact writes 1180-1182, mirror/bind jq substitution block 1474-1522, graft image tag 1632, graft fact write 1665, enshrine jq block 856-884 including _RBGE_ENSHRINE_NAME that currently carries $\{CLOUD_PREFIX\}enshrine and needs reshape to plural enshrines/ with anchor-as-path-segment not tag); rbfv_FoundryVerify.sh 22+ sites (tag construction lines 75/130/154/212/371/374/395-396/605/629, 3 verify jq substitution blocks for graft/bind/conjure at lines 317-358/510-533/729-754 — each block currently passes _RBGV_VOUCHES_PACKAGE which dissolves, catalog-filter case statement lines 852/855 that switches on SUFFIX patterns); rbfl_FoundryLedger.sh abjure per-suffix HEAD/DELETE dance at lines 282-341 collapses to enumerate-then-delete via gcloud artifacts packages list against hallmark subtree (naturally tolerates graft's missing pouch — no special case); .rbk/rbob_compose.yml 3 image references at lines 23/57/77 reshape to $\{CLOUD_PREFIX\}hallmarks/$\{HALLMARK\}/image:$\{HALLMARK\}; Cloud Build step scripts (rbgji/rbgji01-inscribe-mirror.sh line 36 + dispatcher — needs _RBGN_CLOUD_PREFIX substitution threading and reliquary tag change from :latest to :$\{RELIQUARY_DATE\}; rbgje/rbgje01-enshrine-copy.sh line 62 — _RBGE_ENSHRINE_NAME reshape to plural + anchor-as-tag; 6 rbgjb/ scripts and 2 rbgja/ and 2 rbgjv/ scripts — drop _RBG?_ARK_SUFFIX_* substitutions, add _RBG?_HALLMARKS_ROOT substitution, consumer scripts build URL as $\{_RBG?_HALLMARKS_ROOT\}/$\{_RBG?_HALLMARK\}/<basename-literal>:$\{_RBG?_HALLMARK\}). Grep-zero discipline currently UNSATISFIED: grep -nE 'RBGC_ARK_SUFFIX_|RBGC_VOUCHES_PACKAGE' hits ~50 sites across rbfd/rbfv/rbfl pointing to now-undefined readonly constants; the foundry modules will die at function-body execution time if their path-construction code is reached. Fast suite doesn't exercise those paths so tree is green on fast (47+21+7+15 = 90/90). Service and crucible suites will fail until Notch 2b lands. rbhodf tutorial was swept into ₣A6 commit 5c9522d5 via multi-officium concurrency overlap (another officium's jjx_record explicit file list included rbhodf and my WIP edits rode along) — the BASENAME/CATEGORY_HALLMARKS rewrites are live in the tree but attributed to the other heat; no action needed in this notch or in 2b. Grooming decisions preserved in pace docket: rbgl is constants-only (deviated from 5-helper-functions plan — docket Decisions section 1-2 now effectively describes constants instead of functions); hallmark-as-tag with 'tag = enclosing directory's immutable identifier' pattern extended to reliquary datestamp and enshrine anchor; abjure converts to enumerate-then-delete (gcloud API level, not subtree delete — GAR doesn't offer that); Notch 3 handles specs (RBSAC/RBSAE/RBSDI/RBSAA/RBSAV/RBSAS/RBSAP/RBSCL/RBS0/RBSCO). BCG lessons discovered mid-flight that must be forward-applied to Notch 2b: (a) no $() invocation on non-_capture functions — use pure-interpolation kindle constants or guarded _capture; (b) yawp is for human-readable text fragments only, NOT internal computation; (c) SCREAMING_SNAKE is for readonly kindle constants — mutable kindle state assigned outside kindle must be z_-lowercase; (d) named RBGC_* constants for system-vocabulary literals (image/vouch/about/pouch/attest/diags/gcloud/docker/alpine/syft/binfmt/skopeo) — magic strings hide meaning; (e) for Cloud Build consumer scripts that can't source rbgl, pass HALLMARKS_ROOT as a Cloud Build substitution (not function-call result) and let consumer do pure interpolation. Verification landed: bash -n clean on all 13 files; fast suite 90/90 green; shellcheck -S error clean on rbgl + rbgc.

### 2026-04-23 11:07 - Heat - S

oauth-trust-hygiene-sweep

### 2026-04-23 10:44 - ₢A_AAK - n

Notch 1 of ₢A_AAK: mint rbgl_GarLayout.sh + category constants (additive, no consumers). New module Tools/rbk/rbgl_GarLayout.sh exposes 5 path-construction helpers — rbgl_hallmark_ark_path/rbgl_hallmark_subtree/rbgl_reliquary_path/rbgl_reliquary_subtree/rbgl_enshrine_path — emitting prefix-rooted relative paths from GAR repo root (callers prepend ${host}/${path}/). Each takes mandatory positional args via bash :? expansion, sentinels against kindle, and printf-emits the composed path. Kindle takes a zrbgc_sentinel dependency for the category constants and is otherwise stateless. Added RBGC_GAR_CATEGORY_{HALLMARKS,RELIQUARIES,ENSHRINES} readonly constants to rbgc_Constants.sh co-located with the existing ark-suffix block; category values are plain strings (hallmarks/reliquaries/enshrines). Design follows BCG precedent of rbgu_Utility.sh — set -euo pipefail, ZRBGL_SOURCED guard, zrbgl_kindle/zrbgl_sentinel boundary, zrbgc_sentinel dependency declaration. No consumer sources rbgl yet — consumers and the RBGC_ARK_SUFFIX→RBGC_ARK_BASENAME rename land atomically in Notch 2 so grep-zero discipline never passes through a broken intermediate state. Module prefix rbgl chosen from free rbg* slots (rbgp_ is Payor, rbga-rbgu are taken elsewhere); terminal-exclusive. Tag shape on basename arks decided during grooming: tag = hallmark itself (image:${HALLMARK}), redundant with path segment by construction so divergence is visible bug. Verification: bash -n on both files OK; shellcheck -S error clean; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render) — the additive shape leaves runtime behavior unchanged.

### 2026-04-23 10:30 - ₢A_AAC - W

Tightened RBRR_CLOUD_PREFIX and RBRR_RUNTIME_PREFIX into the 'blanked-on-zero, populated-before-enforce' precedent shared with RBRR_DEPOT_PROJECT_ID et al. Enrollment min-length lifted 0→2 (natural floor of the ^[a-z][a-z0-9-]*-$ regex), regex drops the ^$| empty-alternative, error messages drop 'empty or'. Marshal-zero path emits both fields empty via two case entries merged into the existing 'fields blanked' announcement group — prefix fields now share semantics with the other site-specific blanks rather than standing alone. RBSRR-RegimeRepo.adoc reframes the Resource Prefixing group narrative from 'both default to empty' to 'both must be populated'; per-variable 'Empty by default' claims removed. Live .rbk/rbrr.env gets rbxc-/rbxr- placeholder values — regex-valid, 5 chars each, carry a visible 'change-me' hint via the 'x' marker without needing a tripwire mechanism. Design evolution from original docket: docket called for 'empty is valid by design' but collaboration surfaced that empty-as-default silently accepts a risky state on shared infrastructure; tightening to min=2 forces an intentional operator decision on marshal-zero while keeping the enrollment-level contract simple. Verification: rbrr_validate passes with rbxc-/rbxr-, Resource Prefixing group renders both vars, fast suite 90/90 green (47+21+7+15). Marshal-zero not exercised in place (destructive against live depot config); scratch-filter simulation confirms non-empty prefix inputs are force-reset to empty under the new case entries. Scope note: AAK-disposable — the enrollment+regex changes and rbrr.env shape persist across AAK's GAR categorical layout migration; the marshal-zero case entries are orthogonal to path grammar. Proof-clone exercise of full marshal-zero ceremony remains available for release-qualify cycles but was deferred as not load-bearing for this pace.

### 2026-04-23 10:29 - ₢A_AAC - n

Tighten RBRR_CLOUD_PREFIX and RBRR_RUNTIME_PREFIX to the 'blanked-on-zero, populated-before-enforce' precedent shared with RBRR_DEPOT_PROJECT_ID et al. Enrollment min-length lifted from 0 to 2 (natural minimum of the ^[a-z][a-z0-9-]*-$ format regex), regex loses the ^$| empty-alternative, error messages drop the 'empty or' phrasing. Marshal-zero path emits both fields empty via two new case entries grouped with the other site-specific blanks (dropped the separate 'reset to empty default' announcement block in favor of merging into the existing 'fields blanked' group — semantics now parallel). RBSRR-RegimeRepo.adoc reframes the Resource Prefixing group narrative: 'both must be populated' replaces 'both default to empty'; per-variable 'Empty by default' claims removed. Live .rbk/rbrr.env gets rbxc-/rbxr- placeholder values — regex-valid, 5 chars each, carry a visible 'change-me' hint via the 'x' marker without needing a tripwire mechanism; comment reframed to describe the placeholder contract. Design evolution from the original docket: the docket called for 'empty is valid by design' but collaboration surfaced the concern that empty-as-default silently accepts a risky state on shared infrastructure; tightening to min=2 makes the field behave like the other depot-tied requireds and forces an intentional operator decision on marshal-zero, while the 'x' placeholder carries the hint forward into the live working regime. Verification: rbw-rrv passes with rbxc-/rbxr-; rbw-rrr surfaces both in the Resource Prefixing group; fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render). Marshal-zero not exercised in place (would destroy live depot config); scratch-filter simulation confirms non-empty prefix inputs are force-reset to empty under the new case entries.

### 2026-04-23 10:06 - ₢A_AAB - W

Threaded RBRR_RUNTIME_PREFIX into local container/network names (ZRBOB_SENTRY/PENTACLE/BOTTLE/NETWORK, compose -p project arg, compose.yml container_name directives) and RBRR_CLOUD_PREFIX into GAR path construction at vessel position across the foundry family: rbob image/vouch refs, rbrn survey refs, rbfr wrest + 4 API URLs + 3 image refs, rbfv 9 manifests URLs + z_vi_gar_prefix x2, rbfc reliquary prefix + zrbfc_gar_extract_artifact callers, rbfd reliquary canary + enshrine HEAD/prefix + sigil path + context tag + kludge/graft refs, rbfl 5 HEAD + 5 DELETE + 2 tags/list + jettison, compose.yml image refs. Baked prefix into jq-arg substitutions feeding Cloud Build step scripts (_RBGA_VESSEL, _RBGV_VESSEL, _RBGY_MONIKER, _RBGN_RELIQUARY, _RBGV_VOUCHES_PACKAGE) so step scripts remain unchanged. New _RBGE_ENSHRINE_NAME substitution in rbfd enshrine jq block carries ${RBRR_CLOUD_PREFIX}enshrine; rbgje01-enshrine-copy.sh consumes it instead of hardcoded 'enshrine' literal (parallel to _RBGN_RELIQUARY pattern). All threading is AAK-disposable — AAK reshapes path grammar into <prefix>hallmarks/<hallmark>/... categorical layout; AAB keeps current vessel:hallmark-suffix shape and inserts prefix at vessel position only. Adjacent fix: RBGC_vouches_PACKAGE -> RBGC_VOUCHES_PACKAGE in rbfc_FoundryCore.sh:653 (misspelled variable in hallmark-only plumb lookup, silently fell through under set -u until surfaced while threading prefix through that line). Also slated ₢A_AAM oci-magic-string-extraction at end of heat for future follow-up on OCI Distribution Spec protocol keywords (manifests, blobs, tags/list, v2) — scoped deliberately narrower than category namespaces which AAK owns. Verification: fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render); nameplate render and info survey exercise RBRN image-ref computation cleanly with empty prefix. Crucible suite not run (requires container runtime) — baseline-preservation property of empty-default makes regression risk low.

### 2026-04-23 10:05 - ₢A_AAB - n

Thread RBRR_RUNTIME_PREFIX into local container/network names and RBRR_CLOUD_PREFIX into GAR path construction at vessel position across the foundry family. Runtime side: rbob_bottle.sh kindle constants (ZRBOB_SENTRY/PENTACLE/BOTTLE/NETWORK) and compose -p project arg carry the prefix so compose's generated {project}_enclave network name matches ZRBOB_NETWORK; compose.yml container_name directives interpolate RBRR_RUNTIME_PREFIX. Cloud side: RBRR_CLOUD_PREFIX inserted at GAR-internal vessel-path position across bash sites (rbob GAR image/vouch refs, rbrn survey image refs, rbfr wrest locator + 4 API URLs + 3 image refs, rbfv 9 manifests URLs + z_vi_gar_prefix x2, rbfc zrbfc_resolve_tool_images reliquary prefix + zrbfc_gar_extract_artifact callers, rbfd reliquary canary + enshrine HEAD + enshrine image prefix + sigil path + context tag + kludge/graft image refs, rbfl 5 HEAD + 5 DELETE + 2 tags/list + jettison manifest) and baked into jq-arg substitutions feeding Cloud Build step scripts (_RBGA_VESSEL via zjq_vessel/zjq_moniker, _RBGV_VESSEL, _RBGY_MONIKER, _RBGN_RELIQUARY, _RBGV_VOUCHES_PACKAGE). Compose.yml image refs prepend RBRR_CLOUD_PREFIX at vessel segment. Enshrine category: new _RBGE_ENSHRINE_NAME substitution in rbfd's enshrine jq block carrying ${RBRR_CLOUD_PREFIX}enshrine; rbgje01-enshrine-copy.sh consumes it to replace the bare 'enshrine' literal (parallel to how _RBGN_RELIQUARY carries the prefixed reliquary datestamp). All path threading is AAK-disposable — AAK reshapes path grammar entirely into <prefix>hallmarks/<hallmark>/... categorical layout, so AAB keeps the current vessel:hallmark-suffix shape and inserts prefix at vessel position only, zero structural change. Fixed adjacent pre-existing typo in rbfc_FoundryCore.sh:653: RBGC_vouches_PACKAGE -> RBGC_VOUCHES_PACKAGE (misspelled variable name in hallmark-only plumb lookup — silently fell through to 'Hallmark not found in vouches superdirectory' die branch under set -u, surfaced while threading the prefix through that line). Verification: fast suite 90/90 green (47 enrollment-validation + 21 regime-validation + 7 regime-smoke + 15 handbook-render); nameplate render and nameplate-info survey exercise RBRN image-ref computation cleanly with empty prefix; RBRR repo regime render surfaces both prefix vars with '(not set)' indicator. Crucible suite not run here (requires container runtime) — baseline-preservation property of empty-default makes regression risk low, but worth running against a live fundus before final sign-off.

### 2026-04-23 09:47 - Heat - S

oci-magic-string-extraction

### 2026-04-23 09:35 - ₢A_AAA - W

Added RBRR_CLOUD_PREFIX and RBRR_RUNTIME_PREFIX under a new 'Resource Prefixing' enrollment group. Both optional (0-11 chars), defaulted empty, format-checked when non-empty (^[a-z][a-z0-9-]*-$). Cloud prefix scopes GAR/Cloud Build/GCP names; runtime prefix scopes local container/network names. Enforce regex mirrors existing RBRR_GCB_POOL_STEM pattern. Documented as a new group in RBSRR with attribute refs in RBS0. Verified: validate green with empty defaults and with valid samples (test-, bhy-); enforce dies on bad format (bad) with expected message; buv length check catches over-length (toolong12345-) at 13 chars. Fast suite 90/90 green (exceeds docket's 75 target — handbook-render added post-docket). Scope-correct: no consumer wiring (that is A_AAB), no marshal-zero template changes (that is A_AAC), no depot-side work (that is A_AAE).

### 2026-04-23 09:23 - ₢A_AAA - n

Enroll RBRR_CLOUD_PREFIX and RBRR_RUNTIME_PREFIX under new Resource Prefixing group — 0–11 chars, empty default, regex enforces lowercase-ending-in-hyphen when non-empty. Cloud prefix scopes GAR/Cloud Build/GCP names; runtime prefix scopes local container/network names. Documented in RBSRR with attribute refs in RBS0.

### 2026-04-23 09:17 - Heat - f

racing

### 2026-04-23 08:23 - Heat - d

paddock curried: captured GAR categorical layout decisions and heat coherence story

### 2026-04-23 08:22 - Heat - S

tabtarget-vessel-param-removal

### 2026-04-23 08:21 - Heat - S

gar-categorical-layout-migration

### 2026-04-23 07:47 - ₢A_AAI - W

Pace work was already landed in commit bb268dbf (as ₢A6AAi in ₣A6) and carried into ₣A_ by the e268c4bd restring; no rbap residue remains in the repo, RBS0 linked terms read rbgv_*, and CLAUDE.md lists RBGV. Fast suite's single handbook-onboard-first-crucible failure is unrelated — another officium was actively modifying rbhofc_first_crucible.sh at mount time.

### 2026-04-16 07:47 - Heat - r

moved A_AAJ before A_AAE

### 2026-04-16 07:46 - Heat - D

restring 1 paces from ₣AU

### 2026-04-15 15:08 - Heat - r

moved A_AAI to first

### 2026-04-15 15:07 - Heat - D

restring 2 paces from ₣A6

### 2026-04-15 15:05 - Heat - S

run-all-onboarding-tracks

### 2026-04-15 15:05 - Heat - S

run-complete-test-suite

### 2026-04-15 15:04 - Heat - S

regenerate-depot

### 2026-04-15 15:04 - Heat - D

restring 1 paces from ₣A6

### 2026-04-15 15:04 - Heat - S

marshal-release-zeroes-prefixes

### 2026-04-15 15:04 - Heat - S

derived-constants-and-consumer-wiring

### 2026-04-15 15:04 - Heat - S

rbrr-image-and-runtime-prefix-variables

### 2026-04-15 15:03 - Heat - N

rbk-mvp-3-resource-prefix-and-depot-regen

