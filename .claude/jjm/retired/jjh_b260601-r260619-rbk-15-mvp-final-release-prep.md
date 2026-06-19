# Heat Trophy: rbk-15-mvp-final-release-prep

**Firemark:** ₣BU
**Created:** 260601
**Retired:** 260619
**Status:** retired

## Paddock

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

### Handbook learner-walk validation — owned by ₣A6

A pre-release release would want the operator-facing onboarding handbook tracks
(`rbh*`) confirmed against working code. **That validation is ₣A6's work, not
this heat's** — ₣A6 (handbook-restart) owns the intent-axis handbook corpus, and
its tail-resolution pace folds the end-to-end learner walk (carried over from the
retired ₣AU) into a single corpus-grounded validation pace. When releasing,
confirm ₣A6's learner-walk validation has run rather than duplicating it here.

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
- ₣BB rbk-mvp-release-qualification — built the machinery; holds precursor
  engineering and the git history of the dropped run/doc dockets
- ₣A6 rbk-mvp-handbook-restart — owns the handbook learner-walk validation

## Paces

### bcg-module-decomposition-0-trick (₢BUAAA) [complete]

**[260604-1414] complete**

## Character
BCG authoring + whole-document consistency review. Judgment on where the decomposition pattern fits and how it reconciles with the existing single-module guidance.

## Goal
Incorporate the module-decomposition "0-trick" into BCG: when a module outgrows one file, the prefix becomes a container naming no file, with a two-tier layout — a `«prefix»0_` top tier (CLI `«prefix»0_cli.sh` + entry `«prefix»0_«ModuleName».sh`, the sourcing hub holding the single inclusion-guard + the whole kindle) and a gesture-letter cluster tier (guard-free bodies the entry sources). Dispatch functions keep the container prefix — terminal exclusivity is file-level, not function-level.

## Cinched
- `«prefix»0_«ModuleName».sh` is THE entry form (e.g. `rbld0_Lode.sh`) — retires the `k`-postfix entry.
- The `0` tier is the module's top/entry surface (cli, entry, rbh0-style landings); the cluster tier uses gesture letters; the `_cli.sh`-must-be-executable marker still governs (entry is non-executable).

## Scope
1. Whole-document review pass so the decomposition pattern reconciles with Module Architecture, Module-CLI Prefix Unity, the Templates, the Naming conventions, and the filesystem marker.
2. Module Maturity Checklist updated to cover decomposed modules.
3. FM-002 retiring the `k`-entry form (`k` → `0_«ModuleName»`).

## Found
- BCG today documents only single modules (Module-CLI Prefix Unity: `rbrn_` + `rbrn_cli`); no decomposition pattern exists.
- Precedent: the rbh0 handbook (`rbho0_cli` + `rbho0_start_here` top tier). Live exemplar: the rbk lode-module-explosion (`rbld0_cli` + `rbld` clusters) — its `rbldk_Kindle.sh` renames to `rbld0_Lode.sh` under FM-002 (that rename is rbk-heat work, not this pace).

## Done
BCG carries a Module Decomposition section consistent with the rest of the document; the Maturity Checklist covers decomposed modules; FM-002 records the `k` → `0_«ModuleName»` retirement; the whole-document review is complete; `0_«ModuleName»` is cinched.

**[260604-1132] rough**

## Character
BCG authoring + whole-document consistency review. Judgment on where the decomposition pattern fits and how it reconciles with the existing single-module guidance.

## Goal
Incorporate the module-decomposition "0-trick" into BCG: when a module outgrows one file, the prefix becomes a container naming no file, with a two-tier layout — a `«prefix»0_` top tier (CLI `«prefix»0_cli.sh` + entry `«prefix»0_«ModuleName».sh`, the sourcing hub holding the single inclusion-guard + the whole kindle) and a gesture-letter cluster tier (guard-free bodies the entry sources). Dispatch functions keep the container prefix — terminal exclusivity is file-level, not function-level.

## Cinched
- `«prefix»0_«ModuleName».sh` is THE entry form (e.g. `rbld0_Lode.sh`) — retires the `k`-postfix entry.
- The `0` tier is the module's top/entry surface (cli, entry, rbh0-style landings); the cluster tier uses gesture letters; the `_cli.sh`-must-be-executable marker still governs (entry is non-executable).

## Scope
1. Whole-document review pass so the decomposition pattern reconciles with Module Architecture, Module-CLI Prefix Unity, the Templates, the Naming conventions, and the filesystem marker.
2. Module Maturity Checklist updated to cover decomposed modules.
3. FM-002 retiring the `k`-entry form (`k` → `0_«ModuleName»`).

## Found
- BCG today documents only single modules (Module-CLI Prefix Unity: `rbrn_` + `rbrn_cli`); no decomposition pattern exists.
- Precedent: the rbh0 handbook (`rbho0_cli` + `rbho0_start_here` top tier). Live exemplar: the rbk lode-module-explosion (`rbld0_cli` + `rbld` clusters) — its `rbldk_Kindle.sh` renames to `rbld0_Lode.sh` under FM-002 (that rename is rbk-heat work, not this pace).

## Done
BCG carries a Module Decomposition section consistent with the rest of the document; the Maturity Checklist covers decomposed modules; FM-002 records the `k` → `0_«ModuleName»` retirement; the whole-document review is complete; `0_«ModuleName»` is cinched.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A bcg-module-decomposition-0-trick

A
x BCG-BashConsoleGuide.md

Commit swim lanes (x = commit affiliated with pace):

  1 A bcg-module-decomposition-0-trick

123456789abcd
······xx·····  A  2c
```

## Steeplechase

### 2026-06-18 13:58 - Heat - f

stabled

### 2026-06-16 08:26 - Heat - f

silks=rbk-15-mvp-final-release-prep

### 2026-06-16 10:59 - Heat - S

build-bucket-vestige-scrub

### 2026-06-15 18:33 - Heat - f

racing, silks=rbk-08-mvp-final-release-prep

### 2026-06-04 19:57 - Heat - f

silks=rbk-15-mvp-final-release-prep

### 2026-06-04 14:14 - ₢BUAAA - W

BCG gained a Module Decomposition section (synthetic notation, no rb-grafting): container prefix names no file; two-tier layout — 0_ModuleName entry-hub (single inclusion-guard + whole kindle, non-executable) over guard-free gesture-letter clusters; single-guard rule with the multiply-sourced-cluster exception; filesystem-level (not function-level) terminal exclusivity; flat-default topology. Reconciled across the document: Sourcing Rules entry-hub carve-out, Naming-table decomposed file-forms, Maturity Checklist (three corrected single-module bullets + new Decomposed Modules block), Module Architecture forward pointer, Decision Matrix note. FM-002 records the k-entry -> 0_ModuleName retirement. Per operator steer: one clean archetype (kindle-hub) documented; rbh0's kindle-in-base divergence left rbk-local rather than canonized; three judgment calls flagged for later review (cluster-file PascalCase vs lowercase, rbh0 silence, FM-002 without a sites list). Committed 687e0213f.

### 2026-06-04 14:13 - ₢BUAAA - n

BCG: add Module Decomposition section (0-trick). Container prefix names no file; two-tier layout — 0_ModuleName entry hub (single inclusion-guard + whole kindle, non-executable) over guard-free gesture-letter clusters; single-guard rule with multiply-sourced-cluster exception; filesystem-level (not function-level) terminal exclusivity; flat-default topology. Reconciled across the document: Sourcing Rules gains the entry-hub top-level-sourcing carve-out; Naming table gains decomposed file-form rows; Maturity Checklist corrects the three single-module bullets and adds a Decomposed Modules block; forward pointer from Module Architecture; Decision Matrix note. FM-002 records the k-entry -> 0_ModuleName retirement in synthetic notation. All synthetic — no rb-grafting.

### 2026-06-04 11:32 - Heat - S

bcg-module-decomposition-0-trick

### 2026-06-02 06:35 - Heat - f

silks=rbk-14-mvp-final-release-prep

### 2026-06-01 06:12 - Heat - d

paddock curried: learner-walk ownership → ₣A6; drop stale heat numbers

### 2026-06-01 05:31 - Heat - f

stabled

### 2026-06-01 05:31 - Heat - d

paddock curried: initial release-memory paddock; distilled from ₣BB

### 2026-06-01 05:30 - Heat - N

rbk-mvp-final-release-prep

