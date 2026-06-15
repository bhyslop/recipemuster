# ACG Pilot 2 — Access-Probe Pair Audit Findings

## Origin

Second ACG audit pilot (₣Bb rbk-10-mvp-acg-scrub, pace ₢BbAAJ), run 2026-06-15
in the operator's chat. Pair audited: `Tools/rbk/rbgv_probe.sh` (RBGV) against
RBSAJ (access_jwt_probe) + RBSAO (access_oauth_probe).

**Tier note.** The pace Gate names a top-tier ("Fable-class") model. Fable was
unavailable; the pilot ran on Opus 4.8 (1M) after the gate was flagged to the
operator and the operator authorized proceeding. Recorded because the gate asks
that a lesser-tier mount be surfaced, not silently taken.

Line numbers below are pinned to commit `4b22ed898`. This memo retires when its
routed items land (see Disposition).

## Headline

The detect rules **fire** — both planted specimens are caught — but the
access-probe pair's landable surface is **thin**: it sits heavily in ₣BZ's
report-only lane (Payor / Manor / account-role vocabulary), and its source-side
collapses defer behind the cited-constraint anchor. The pilot's value is not
landed moves; it is two guide-boundary resolutions and one first-principles
finding that **ACG had overreached into spec form**. The access-probe NOTE is the
worked specimen behind that finding.

## Findings (all detect-only / report-only this pace)

### F1 — "bounded propagation-absorbing retry," three sites

The idea is worded at RBSAJ:9 (spec NOTE), in RBSAJ's own steps (RBSAJ:76,
105-113), and in the RBGV comment at rbgv_probe.sh:287-291 (header of
`rbgv_jwt_sa_probe`).

- **Move:** ACGm_104 citation-collapse on the RBGV comment (the spec homes the
  content); the coined phrase "bounded propagation-absorbing retry" is an
  ACGm_105 corpus item.
- **Disposition:** bash collapse deferred to the anchor pace. **Open question
  raised:** RBGV's residue would cite a plain spec section, not a security
  tripwire (contrast pilot 1's sentry firewall, where the opaque anchor protects
  closed-source security reasoning). So this collapse may *not* need the opaque
  anchor at all — it may land with a plain "see RBSAJ" citation. Whether the
  anchor-deferral applies to non-security collapses is itself an anchor-design
  question; recorded for that pace.
- **Comment extra with no spec home:** "Layers cleanly with
  `rbgo_get_token_capture`'s JWT-signature retry — each layer recognizes its own
  'not yet'." A cross-function compositional property. Candidate either for a
  system-level constraint statement (the two retry layers must compose without
  double-counting) or to stay as edit-time mechanics. Recorded; not dispositioned.

### F2 — role→permission mapping (Governor=Owner, Director=repoAdmin, Retriever=reader)

At RBSAJ:13-19 (spec) and rbgv_probe.sh:293-296 (comment).

- **Move:** ACGm_104/105 cross-medium duplication; the spec is the home.
- **Disposition:** **triage — account-role vocabulary is ₣BZ's lane. Report,
  never touch.**

### F3 — RBSAJ NOTE restates its own steps

The NOTE duplicates normative step content: "returns immediately" at RBSAJ:9 vs
the step at RBSAJ:105; the `--count 1` / `--delay_ms 0` defaults at RBSAJ:11 vs
the steps at RBSAJ:30,32.

- **Move:** ACGm_105 *internal* (one document). A rationale marker does not excuse
  forking a forced constraint — the steps are authoritative; the NOTE should
  trim or cite.
- **Disposition:** report-only (RBSAJ role/Manor vocabulary, ₣BZ lane). Feeds the
  spec-form question below.

### F4 — RBSAJ NOTE genuine rationale: "403 means propagation-not-yet, not denied"

RBSAJ:10 explains *why* the 403-retry constraint exists (the post-grant IAM
propagation window). This is the highest-value sentence in the NOTE: without it
an editor "fixes" the 403-retry into 403-fatal, since 403 normally means denied.
A deliberate-deviation tripwire.

- **Move:** wants the cited-constraint anchor — rationale earns a canon home only
  when bound to a forced constraint (a test probing a freshly-granted role
  defends it). Until the anchor exists, report-only.
- **Significance:** the worked proof that rationale *can* be load-bearing — but
  the load is carried by the constraint it guards, not by free prose.

### F5 — RBSAO NOTE rationale (payor distinct-path; sustained-check posture)

RBSAO:4-13. The payor-uses-refresh-token-not-SA-key rationale (RBSAO:9-12) and
the "enabling sustained access verification and OAuth token lifecycle testing"
clause (RBSAO:7) — the latter the closest thing in either spec to deletable
ornament.

- **Move:** ACGm_104/106 family, lighter than RBSAJ.
- **Disposition:** **triage — RBSAO is entirely Payor. ₣BZ lane. Report, never
  touch.**

### F6 — RBGV payor header restates the procedure

rbgv_probe.sh:357-361 numbers the procedure ("1. Authenticate… 2. Call CRM…
3. Sleep") that RBSAO already homes.

- **Move:** ACGm_104 citation-collapse (RBSAO is the home).
- **Disposition:** Payor vocabulary + bash → ₣BZ lane / anchor pace. Report-only.

## Leftovers and positive controls

- **Correctly-allocated edit-time comments (not findings):** RBGV's contract
  headers at rbgv_probe.sh:100-103, 154-156, and especially 173-177 ("verdict
  logic lives in the caller… loop context to discriminate") carry
  design-of-the-code knowledge with no spec home — and *should* have none (specs
  do not spec private helpers). The audit distinguishes legitimate edit-time
  mechanics from misallocated design-time knowledge; these are the positive
  control.
- **ACGm_101 already satisfied:** the transient-5xx policy values appear in both
  specs (RBSAJ:93, RBSAO:69-73) and in RBGV constants (rbgv_probe.sh:48-49). The
  values resolve through `ZRBGV_HTTP_RETRY_*` constants — homed correctly; the
  spec carries the design value. Not a finding.
- **ACGm_107 (skidmark): clean.** No version commentary in either spec or the
  source (consistent with pilot 1).

## Rule-2 check

Both planted specimens caught: specimen 1 (the retry idea worded twice) → F1;
specimen 2 (rationale-as-normative in RBSAJ) → F3 + F4. The detect rules are not
contradicted on this pair. (Honest bound: the specimens were named in the docket,
so this is a floor test — the rules are not *broken* — not a blind-recall proof.
Independent signal that they find real sites beyond the planted ones: F2, F3's
defaults duplication, F6, and the three-site nature of F1.)

## Fork resolutions

### Fork 1 — the ACGm_104 / ACGm_105 seam (resolved: unit-based; landed in ACG)

They are not rivals; they tag different *units*. ACGm_104 acts on a misallocated
*comment* (collapse to residue); ACGm_105 tracks a recurring *wording*
corpus-wide (one law, cite the rest). They coincide on one comment without
conflict. Pilot 1 practiced this without writing it down; the groom mis-filing
this pilot's specimen under 105 was the evidence the seam needed a sentence.
Landed as the unit-boundary note under ACGm_105 with a cross-ref at ACGm_104.

### Fork 2 — does an AsciiDoc `NOTE:` mark rationale? (resolved: wrong question)

First lean was to bless `NOTE:` as a dialect-specific rationale marker. Reading
the diptych vision (`memo-20260209-diptych-vision.md`) overturned it: the
destination is a *restrictive declared grammar* with no free-form admonition, so
blessing `NOTE:` would enshrine a construct with a demolition date and protect
exactly the dumping-ground prose the destination sheds. The deeper turn (below)
is that this is not ACG's question at all.

## The first principle (feedstock for the spec-form layer — MCM / diptych)

The pilot's durable finding, seeded here for the document-form design:

1. **A durable document's contents each need a forcing function or they rot** —
   ACG's own trace principle (the three-homes execution-time note).
2. This cuts *across* the constraint/rationale line. A **constraint** is forced
   (the implementation, or a regenerate-from-specs diff, pins it). **Rationale**
   is not — nothing consumes it mechanically, so it drifts like trace. RBSAJ's
   NOTE is the proof (F3).
3. Under regenerate-from-specs, free rationale is also **non-generative** — the
   regenerator looks through it. Unforced *and* non-generative is the worst
   quadrant: costs tokens, untrustworthy, produces nothing.
4. A NOTE's legitimate cargo **redistributes to four homes**, each forced or
   consumed: forced constraint (drain the restatement); rationale bound to a
   **cited-constraint anchor** (forced by the defending test); a structured
   **operation-summary** (generative — the regenerator reads it); and the
   **lectio** layer (human onboarding, derived from the canon, never in it).
   No canon-residue was found that wants free unforced prose. *Caveat:* not
   proven exhaustive from one pair — a true residue would be the exception to
   characterize, not the rule.
5. **Therefore:** rationale earns a place in the canon only when bound to a
   constraint with a forcing function. This is spec-form doctrine — MCM and the
   diptych design own it. ACG governs allocation and should not legislate it,
   which is why the normative-register clause was pared back this pace.

## Rulings landed in ACG (this pace)

1. **ACGm_104/105 unit-boundary note** added (fork 1).
2. **Normative-register clause pared to its allocation core.** The spec-form
   half ("every spec sentence constrains or is marked rationale"; the
   `**Gestalt**:`-is-the-blessed-form enshrinement) removed; a one-line boundary
   added — spec form is MCM/diptych's domain, ACG governs allocation. The
   allocation residue (a code comment restating design-time knowledge is a
   cross-medium fork) stays.
3. **ACGm_106 recast detect-only, authority-pending** — its authority pointer
   moved from the in-ACG normative-register rule to the spec-form layer (MCM /
   diptych), held externally until rehomed. The move stays catalogued.
4. **Acronym Registry** "Normative register" row updated to record the migration.

## Routed / deferred

- **Full normative-register + ACGm_106 re-home into MCM / diptych:** recommended
  **follow-on pace** — surfaced here, not executed (junk-drawer discipline; the
  migration is a spec-form design task, not an ACG-scrub move).
- **Forcing-function first principle:** this memo, plus a one-line pointer in the
  diptych removal-conditions ledger so the migration finds it.
- **Spec-side findings (F2, F3, F4, F5):** report-only, ₣BZ lane
  (role / Manor / Payor vocabulary).
- **Bash collapses (F1, F6):** deferred to the anchor pace. F1 carries the open
  question of whether non-security collapses need the opaque anchor at all (they
  may collapse with a plain spec citation).
- **F1 comment extra** (retry-layer composition): recorded, awaiting the
  system-level-constraint vs edit-time judgment.

## Disposition

- ACG amendments (104/105 seam; normative-register pare-back; ACGm_106 recast;
  registry row): **landed this pace.**
- Diptych ledger pointer: **landed this pace.**
- Full normative-register → MCM migration: **slate a follow-on pace.**
- Report-only spec findings and deferred bash collapses ride ₣BZ and the anchor
  pace; this memo retires when those routed items land.
