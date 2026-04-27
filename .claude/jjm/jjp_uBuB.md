## Context

Release qualification gap surfaced by the ₣A_ depot-regen heat. Every existing test tier (`rbw-tf` fast, `rbw-tr` release) tolerates accumulated depot state, so silent first-build assumptions slip through. Recent spooks — kludge-aware-charge-prereq (k-prefixed hallmarks at GAR), ZRBOB_PROJECT (compose project name without runtime prefix), rbob_charged_predicate prefix omission, reliquary integrity-broken on negative test — all share a property: they only manifest on first-build paths, and none were caught until live verification surfaced them.

This heat constructs `rbw-tP` QualifyPristine — a third tier that **refuses to run** unless marshal-zero state was just committed. The refusal is enforced by the test itself, not by ceremony or operator discipline.

## Single operator prerequisite chain

Operator has exactly one prerequisite chain: confirm Payor health → marshal-zero → commit → run `rbw-tP`. Payor OAuth is the only credential the operator must have ready; everything else (governor, retriever, director SAs and their RBRA files) is **minted by the qualification itself**, not restored from backup.

## Entry contract — load-bearing

`rbw-tP` fails-fast unless ALL of the following hold:

- Working tree clean
- HEAD commit is a marshal-zero commit (detectable signature baked into `rblm_zero` per BBAAB)
- RBRR fields are blank (prefixes, depot project ID, etc.)
- No RBRA credential files present (governor, retriever, director — all deleted by marshal-zero, recreated by the qualification)
- No hallmark fields populated in nameplates
- No depot-scoped fields populated in vessel rbrv.env files

Operator cannot skip the prerequisite. This is the property that makes the tier catch the silent-first-build bug class by construction.

## Failure mode contract

Mid-qualification failure means start-over-from-zero, not patch-and-continue. Documented explicitly in runbook to prevent the very accumulated-state bug class this tier exists to catch.

## Release-branch execution contract

Three properties bind every fixture and phase in this tier.

**Run on a release branch.** The release machinery is engineered for branch execution, not main. The branch will accumulate many commits across a successful pristine run. After a failed run, the branch holds bygone commits that `rbw-MZ` does not touch — they remain as history; the next run starts from a fresh marshal-zero commit on top.

**Commits during the run are first-class.** Each step that mutates regime state (rbrr.env, rbra files, vessel rbrv.env, etc.) commits the change. Container-recipe work has a documented cognitive failure mode: humans lose track of where they are in long sequences. The growing commit trail is the operator's mental anchor against that.

**Stop on very first failure, cleanly and clearly.** A step fails → fixture stops → operator stops → debugging happens immediately on the failed branch. Stderr names the failed step; fixture exits non-zero. Recovery is `rbw-MZ` + retry on a fresh marshal-zero. No graceful degradation, no recovery branches.

Engineering scaffolding for any other shape — partial-run cleanup, soft-delete tolerance, multi-mode failure handling, recovery-time orphan inspection, diagnostic-redundancy — is non-load-bearing and does not belong in pristine-tier code. The single mechanical path is the entire surface.

## Tier layering

Three tiers, escalating cost:

| Colophon | Frontispiece | Cost | Entry contract |
|----------|--------------|------|----------------|
| `rbw-tf` | QualifyFast | seconds | none |
| `rbw-tr` | QualifyRelease | minutes | none (accumulated state OK) |
| `rbw-tP` | QualifyPristine *(new)* | ~1 hour + cloud $ | marshal-zero just committed |

`rbw-tP` is THE release gate. `rbw-tr` becomes the pre-pristine smoke test — cheap-and-frequent for development confidence; pristine for actual release.

## Operator scope

Single operator (project lead). Not designed for multi-operator workflow. Runbook lives in README.md release section.

## Coupling to ₣A_

Runs concurrently with ₣A_'s remaining paces; ₣A_'s one-time cutover informs `rbw-tP`'s sequence.

## References

- ₣A_ rbk-mvp-3-resource-prefix-and-depot-regen — surfaced the gap
- `tt/rbw-tr.QualifyRelease.sh` — current release qualify; layered alongside, not replaced
- `Tools/rbk/rblm_cli.sh` — `rbw-MZ` zeroes local regime; marshal-zero signature baked here per BBAAB
- `.claude/commands/rbk-prep-release.md` — upstream contribution ceremony; pristine-pass becomes a precondition (BBAAH)