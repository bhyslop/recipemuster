## Purpose

Durable memory for **final release preparation** of Recipe Bottle. This heat is
deliberately **paddock-only and stabled** until the project is close to actually
cutting a release. It exists so the hard-won release model — and the specific
runs and doc work that gate a release — are not lost while the release itself
is deferred.

When release time comes: re-read this paddock, slate fresh paces for the runs
and doc revision described below (dockets in git history of the dropped ₣BB
paces if detail is needed), confirm the precursor engineering has landed, and
race the heat.

## What already shipped (do not re-do)

The release-qualification **machinery** is built and lives under ₣BB:

- `rbw-tP` QualifyPristine — the release gate tabtarget.
- The **gauntlet suite** — a TestSuite (not a custom orchestrator) composing
  fixtures in a state ladder from marshal-zero through canonical-credentialed
  to crucible verification.
- The marshal-zero signature gate baked into `rblm_zero` (`rbw-MZ`).

This heat does not rebuild any of that. It is about *running* it and finishing
the operator-facing documentation around it.

## The release-qualification model (the idea worth keeping)

`rbw-tP` is THE release gate. It **refuses to run** unless marshal-zero state was
just committed — the refusal is enforced by the test itself, not by ceremony or
operator discipline. This is what catches the silent-first-build bug class
(k-prefixed hallmarks, project-name prefix omissions, reliquary-integrity on
negative tests) that every accumulated-state tier silently tolerates.

**Entry contract** (all must hold or `rbw-tP` fails fast): working tree clean;
HEAD is a marshal-zero commit; RBRR fields blank; no RBRA credential files
present (governor/retriever/director — minted by the qualification, not restored
from backup); no hallmark fields in nameplates; no depot-scoped fields in vessel
rbrv.env files. Only **Payor OAuth** is the operator's prerequisite; everything
else is self-minted.

**Release-branch execution contract** — run on a release branch, not main; the
branch accumulates first-class commits during the run (the growing commit trail
is the operator's mental anchor against losing their place in a long sequence);
stop on the very first failure, cleanly. Recovery is `rbw-MZ` + retry on a fresh
marshal-zero — **mid-qualification failure means start over from zero, not
patch-and-continue**. No graceful degradation, no recovery branches. Any
scaffolding for partial-run cleanup or multi-mode failure handling is
non-load-bearing and does not belong in pristine-tier code.

**No automatic teardown.** After a green tally the canonical depot, SAs,
hallmarks, and RBRA files persist for operator inspection. Cleanup is
operator-driven: `rbw-fA` per vessel + `rbw-arD` + `rbw-adD` + `rbw-dU`.

## Pending release work (slate as paces when close)

### Pristine first run

First end-to-end run of the gauntlet suite via `rbw-tP` against actual fresh
state — where cross-section bugs that case-level probes can't see finally
surface. **Wrap criterion is a green tally**, not "ran to completion" — an
explicit guardrail against the agent failure mode of wrapping at "I built it"
rather than "it works against reality." Cost ≈ 1 hour wall-clock + 2 GCP
projects per attempt. Capture every surprise; triage tiny fixes inline + re-run,
larger ones into follow-up paces. Re-runs draw fresh quota; budget accordingly.

### Pristine branched re-run (validation)

Re-run `rbw-tP` end-to-end on a release branch to validate the first run's
repairs. With BURS_TINCTURE composition landed, parallel mac+linux execution
against a shared Payor manor is first-class — distinct tincture values give every
RBDC-derived name (depot project_id, GAR repo, GCS bucket, SA emails) per-station
disjointness by construction. Operator picks the machine; no foray. Same
green-tally guardrail.

**Collision-aware diagnostic** (both runs): any GCP "resource already exists"
error should name BURS_TINCTURE and the colliding term with a hypothesis pointing
at peer-station tincture overlap — distinguishing tincture-composition collision
from a transient GCP race.

### RELEASE.md revision

Re-read RELEASE.md cold and bring it into alignment with what the gauntlet suite
became:

- Gauntlet-suite vocabulary (suite ≠ fixture ≠ case; "pristine" reserved for the
  §1 fixture and tier vocabulary).
- §3 Sequence rewritten as suite-of-fixtures composed in order, not a custom
  orchestrator script.
- No-teardown semantics documented explicitly.
- The post-success cleanup ceremony (`rbw-fA` per vessel + `rbw-arD` +
  `rbw-adD` + `rbw-dU`) documented for when operators are ready to release
  resources.
- Verify rbw-tP / fixture-name / marshal-zero-signature references match what
  shipped; confirm RELEASE.md ↔ README.md cross-doc anchors resolve.
- Source deferred corrections from ₣BB commit messages (grep RELEASE.md refs)
  and the pristine-walkthrough findings.

The `/rbk-prep-release` ceremony treats a pristine pass as a precondition; the
README points into the cleanup-ceremony section. Keep those cross-doc links live.

### Onboarding end-to-end walk (carried from ₣AU)

Walk every handbook track in sequence from a learner's perspective — Crash
Course, Credential Retriever, Credential Director, First Crucible, Director First
Cloud Build, Payor, Governor — confirming windows render with prefixed resource
names and probe outputs report the correct prefixed container/image identifiers.
This is a pre-release user-perspective validation and a natural companion to the
pristine runs.

> **A6 overlap to reconcile:** ₣A6 (rbk-17-mvp-handbook-restart) covers handbook
> territory and may be the better owner of, or already cover, this walk. Decide
> the ownership split when ₣A6 is groomed before slating this here.

## Precursor engineering (lands in ₣BB, not here)

₣BB stays racing for three tail items that gate a clean pristine run but are
ordinary engineering, not release ceremony: a foundry account-state-invalid
retry-tolerance bug fix (401/`ACCOUNT_STATE_INVALID` flap), the fast-tier
shellcheck fold + marshal-zero gate, and an RBS0 spec-sync sweep. Confirm these
have landed before racing this heat.

## References

- `tt/rbw-tP.QualifyPristine.sh` — the release gate
- `tt/rbtd-s.TestSuite.gauntlet.sh` — the gauntlet suite
- `Tools/rbk/rblm_cli.sh` — `rbw-MZ`; marshal-zero signature baked here
- `RELEASE.md` — the runbook under revision
- `.claude/commands/rbk-prep-release.md` — upstream contribution ceremony;
  pristine pass is a precondition
- ₣BB rbk-15-mvp-release-qualification — built the machinery; holds precursor
  engineering and the git history of the dropped run/doc dockets
- ₣A6 rbk-17-mvp-handbook-restart — handbook territory; reconcile onboarding-walk
  ownership