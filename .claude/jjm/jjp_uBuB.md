## Context

Release qualification gap surfaced by the ₣A_ depot-regen heat. Every existing test tier (`rbw-tf` fast, `rbw-tr` release) tolerates accumulated depot state, so silent first-build assumptions slip through. Recent spooks — kludge-aware-charge-prereq (k-prefixed hallmarks at GAR), ZRBOB_PROJECT (compose project name without runtime prefix), rbob_charged_predicate prefix omission, reliquary integrity-broken on negative test — all share a property: they only manifest on first-build paths, and none were caught until live verification surfaced them.

This heat constructs `rbw-tP` QualifyPristine — a third tier that **refuses to run** unless marshal-zero state was just committed. The refusal is enforced by the test itself, not by ceremony or operator discipline.

## Design decisions

### Entry contract — load-bearing

`rbw-tP` fails-fast unless ALL of the following hold:
- Working tree clean
- HEAD commit is a marshal-zero commit (detectable signature)
- RBRR fields are blank (prefixes, depot project ID, etc.)
- No RBRA credential files present (governor, retriever, director)
- No hallmark fields populated in nameplates
- No depot-scoped fields populated in vessel rbrv.env files

Operator cannot skip the prerequisite. This is the property that makes the tier catch the silent-first-build bug class by construction. Without Phase 0, every later phase silently tolerates accumulated state and the whole tier loses its discipline.

### Cloud-side ceremony owned by the test

`rbw-MZ` zeroes only local state. The pristine tier OWNS its cloud-side wipe — `rbw-dU` then `rbw-dL` runs as the first executable phase, so the operator has only one prerequisite chain: marshal-zero, commit, install RBRAs, run `rbw-tP`.

### Discrete quota-aware checkpoint at depot create

GCP project creation has daily quota caps. Phase 1 (depot lifecycle) is a deliberate discrete checkpoint — bulk qualification can re-run against an already-created depot without re-triggering quota. Operator pauses here consciously.

### Failure mode contract

Mid-qualification failure means start-over-from-zero, not patch-and-continue. Documented explicitly in runbook to prevent the very accumulated-state bug class this tier exists to catch.

### Tier layering

Three tiers, escalating cost:

| Colophon | Frontispiece | Cost | Entry contract |
|----------|--------------|------|----------------|
| `rbw-tf` | QualifyFast | seconds | none |
| `rbw-tr` | QualifyRelease | minutes | none (accumulated state OK) |
| `rbw-tP` | QualifyPristine *(new)* | ~1 hour + cloud $ | marshal-zero just committed |

`rbw-tP` is THE release gate. `rbw-tr` becomes the pre-pristine smoke test — cheap-and-frequent for development confidence; pristine for actual release.

### Operator scope

Single operator (project lead). Not designed for multi-operator workflow. Runbook lives in README.md release section (4-5 human steps).

### Coupling to ₣A_

This heat runs concurrently with ₣A_'s remaining paces. ₣A_'s AAE+AAF+AAG sequence is the one-time cutover that informs `rbw-tP`'s sequence; pristine-tier construction proceeds in parallel and codifies the lessons. Phase 5's first end-to-end run is the natural validation point for both heats together.

## References

- ₣A_ rbk-mvp-3-resource-prefix-and-depot-regen — surfaced the gap; AAE+AAF+AAG one-time burn-in informs but doesn't codify
- `Tools/rbk/rblm_cli.sh` — RBLM Lifecycle Marshal; `rbw-MZ` zeroes local regime, deletes RBRAs, blanks hallmark/depot-scoped vessel fields
- `tt/rbw-tr.QualifyRelease.sh` — current release qualify; layered alongside, not replaced
- `.claude/commands/rbk-prep-release.md` — upstream contribution ceremony; pristine-pass becomes a precondition (Phase 6)
- Recent spook commits informing the bug class: kludge-aware-charge-prereq (₢BAAAH), ZRBOB_PROJECT extraction (f7146d1d), rbob_charged_predicate fix (a8eb7311), reliquary integrity-broken negative-test residue (recent ₣BA verification)