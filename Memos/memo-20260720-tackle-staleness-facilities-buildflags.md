# Tackle's first consumers: staleness, facility claims, and build-invocation discipline

Design-session record, 2026-07-20 (Fable session, operator Brad), during a JJ
maintenance window.
Provenance, never authority: settled leans live in the tackle aspirant sheaf
(`Tools/jjk/vov_veiled/JJSAT-tackle.adoc`) and its siblings; this memo preserves
one chat's reasoning so it neither gains false authority nor gets lost.

Confidence is graded explicitly throughout, at the operator's direction:

- **RATIFIED** — the operator said yes in-session.
- **DOCTRINE** — replay of already-settled house law, cited to its home.
- **SUPPOSITION** — fresh synthesis with zero adversarial review.
  A future reader must not treat these as settled merely because the prose is fluent.

## 1. Binary staleness: tackle's first mechanical consumer

The operator's wild hair, developed into a design lean: Job Jockey should know
when a tackle-claimed source set has drifted from the binary built from it, and
force a rebuild on a daily backstop cadence regardless.

- **RATIFIED**: the execution and the source→executable mapping live in Job
  Jockey, not in the matricula. Mechanical fingerprinting belongs in the shared
  substrate crate (the JJSAT substrate split); rebuild *policy* — when to act,
  the daily cadence — is JJ dispatch judgment. The matricula may read the same
  fingerprint for a freshness lint but never owns rebuild.
- **DOCTRINE fit**: a tackle's claims already define the input set as rules; the
  derived roster is already specified as station-local, disposable, cacheable
  keyed by (table version, tree state) — JJSVT. A content fingerprint over the
  roster is the same nature, same storage home.
- **SUPPOSITION (mechanics)**: git has already content-hashed every tracked
  file, so a clean-tree fingerprint is a hash over the roster's blob hashes —
  no new hashing machinery on the common path. A stamp beside the binary
  records "built from tree X". Precedent found in-house: the hierophant's
  *cachet* is a tree-hash-keyed fact beside its artifact (RBSHC).
- **Honesty bound**: source-hashing is one-sided detection. "Stale" is
  reliable; "fresh" is approximate (toolchain drift, lockfile drift, undeclared
  inputs escape it). Same failure-asymmetry argument that settled claims
  polarity (the 260707 polarity memo): choose the failure mode you can detect.
  The daily forced rebuild is precisely the backstop that caps the undetectable
  remainder at 24h. Neither half alone is sound; together they are.
- **RATIFIED (direction)**: no build without a commit first — the clean-tree
  gate makes the fingerprint exact (fingerprint = the commit's tree hash over
  the roster) and matches the operator's standing audit-trail preference
  (kin to the notch-before-test discipline; enforcement primitive already
  exists as `bug_require_clean_tree_creed`).
- **Consequence**: this is the first mechanical consumer of `jjottn_check_build`,
  which is exactly what the deferred check-command semantics were waiting on
  (JJSAT settling register). Guardrail stated early: JJ knows *staleness* and
  *which command to run*; it never learns dependency graphs or incremental
  compilation — that stays cargo's. JJ is the "should I bother, and with what"
  layer.
- **Open gaps (enumerated 260720 for the API-sketch settling act; the staleness
  surface joins the MVP only when these settle):**
  1. *Artifact declaration.* No schema member names what `check_build` produces,
     and the stamp needs an artifact (or list of them) to attach to. A new
     tackle-grain member, skip-when-absent — a schema addition, which is why
     this gap alone keeps staleness out of "clearly specified."
  2. *Stamp content and home.* Lean: {roster fingerprint, artifact content hash}
     in station-local scratch — per-clone derived state, so the
     no-station-local-facts rivet bars the studbook; the exact directory rides
     BUK station geography.
  3. *Out-of-band rebuild detection.* If a build can happen without stamping (a
     workbench run directly), stamps rot in the unsafe direction (stale stamp
     reads as fresh). Recording the artifact hash in the stamp closes it
     mechanically: current artifact hash ≠ stamped hash → verdict never fresh.
  4. *Dirty-tree verdict.* Commit-before-build is the preferred discipline, but
     the advisor needs a defined verdict when claimed files are dirty (lean:
     indeterminate, never fresh — one-sidedness preserved).
  5. *Invocation ownership.* Whether JJ wraps `check_build` in a
     build-and-stamp verb, or the workbench cooperates via a stamp hook — the
     rollout seam, and it decides who is allowed to mint a stamp at all.
  The daily forced-rebuild cadence is policy atop the advisor and is explicitly
  not MVP.

## 2. Build-invocation discipline (`--locked`, toolchain pin, path remap)

**RATIFIED (intent)**: the operator wants `--locked` and `--remap-path-prefix`
actively required across the repo's Rust builds.

Facts verified 260720:

- All eleven `Cargo.lock` files are committed (every crate, both ifrit
  contexts, the Study crate). `--locked` therefore costs nothing today and
  turns silent dependency drift into a hard refusal; deliberate upgrades become
  an explicit `cargo update` + commit.
- `rust-toolchain.toml` exists only under `Tools/vok/`. One file at repo root
  would pin every crate (pin channel/version, not target — beast's windows-gnu
  still works).
- No `.cargo/config.toml` exists anywhere.
- RCG carries no build-invocation rules at all (it is a code-authoring guide);
  invocation lives in the workbenches, per the tabtarget-only discipline.
- Cargo's `trim-paths` profile option — the clean fix for path remapping — is
  still nightly-only; the project runs stable (RCG's own rustfmt note records
  the stable posture).

**SUPPOSITION (the design that makes remap workable)**: the tabtarget monopoly
is the enabling structure. Because every build passes through a workbench, and
BUK owns station geography, one **flag-composer** homed BUK-side can compute
`--remap-path-prefix=<absolute-root-at-runtime>=<canonical-alias>` on any
station; the canonical alias is a committed constant defined once. Remap rides
the **release profile only** — dev builds keep real paths for debuggers, and
dev/release cache in separate profile spaces so no rebuild churn arises.

**Why flag mismatch is a hazard at all** (the operator's challenge, answered):
cargo defines and fingerprints all flags correctly; the vulnerability is
*plural entry points into one shared cache* — rust-analyzer runs `cargo check`
outside the workbench with its own flags. A Palisade-shaped neighbor: we cannot
legislate the IDE, only arrange the ground. Two arrangements dissolve the
class: static flags in a committed `.cargo/config.toml` (read by every driver,
IDE included), and dynamic flags confined to the release profile (which the IDE
never builds).

**Emplacement rule (lean)**: a guide states standing rules, so the RCG
build-invocation section and its enactment (workbench flag lines + root
toolchain file) travel together or not at all. The designed-but-unbuilt
remainder (flag-composer, remap rollout) stays memo/aspirant-side until built.
Banked-edit target: JJSAT "Reconciliation obligations" gains the rollout entry.

## 3. Facility claims: what tests need vs. what tests monopolize

The operator named a deeply unsettled area: some suites/fixtures require
exclusive use of a cloud facility; others can share one; nothing declares
which, and the operator personally tracks it (or fails and redoes). Split into
two axes:

- **What a test needs** (nothing / GCP credentials / container runtime) — this
  axis is already declared and mechanized as the suite strata
  (reveille/picket/bivouac, `RBTDRA_SUITES`). Solved.
- **What a test monopolizes** — undeclared, head-resident, perishable. The
  actual problem.

**SUPPOSITION (the shape)** — all of the following awaits operator ratification:

- **Two registries, two homes.** The *inventory* of facilities (depots,
  foedera, the keycloak facility — alongside machines) is operator-private
  operational knowledge and belongs in the mews (JJSAM), whose reservation
  mechanism — soft token-granted ownership, operator-reclaimable — is exactly
  the needed grant shape. One reservation mechanism, two kinds of reservable
  resource (nodes and non-local facilities). The *declarations* of need live
  RB-side beside suite membership (the suites registry is already the
  authoritative hand-written home), keeping the dependency arrow clean: RB
  declares as plain data, never reads the fleet; JJ joins the two at dispatch
  (the posture JJSAM already pins for foray).
- **Role-over-instance.** A declaration says "needs *a depot*, exclusive" —
  never a concrete instance; the inventory binds the role at grant time. The
  blaze argument replayed (DOCTRINE by rhyme): the role outlives any binding;
  two shared-mode consumers may receive the same instance; an exclusive
  consumer waits. Per-facility words (depot, foedus) enter JJ only as data —
  names in data, shape in code.
- **Claim modes: exclusive vs. shared, where shared is a conduct promise.**
  The operator's own example is the definition: two cloud tests share a depot
  if each touches only its own partition (its own builds) and neither mutates
  the commons (reliquaries, admission state). "Shared" = commons-read,
  own-partition-write; "exclusive" = declared by anything mutating the commons.
- **Verification bound.** Whether a fixture honors its shared-mode promise is
  undeducible from test code (the mis-affiliation asymmetry again): violations
  surface by incident. Machinery can shape-audit only: unknown roles, dead
  declarations, a fixture refusing to run without a grant for its declared
  needs. Recorded fancy, not commitment: RB's cloud audit attribution trail
  already logs which subject touched what, so declared-vs-actual could one day
  be retro-audited from the cloud's own records.
- **Gait tie-in.** Gaits declare required facility roles + claim modes exactly
  as they declare required blazes; school validates; the orchestrator acquires
  grants before dispatching a beat. The banked maximal-tackle "actions with
  exclusivity scopes" bullet dissolves into this — the closed scope enum
  (worktree/workstation/project) never gets built; scopes become named
  resources in a registry.
- **Timing**: deferred beyond the tackle MVP (operator suspicion, confirmed by
  placement analysis — exclusivity is a property of *running something*, not of
  a fileset). The tackle schema's skip-when-absent members mean a future
  requires-facility member costs nothing to add later; non-foreclosure is
  already delivered.
- **Perishable asset**: the operator's head-knowledge of which suites
  monopolize what. A census pass (zero code — a column or section in the
  domain-census memo, operator-corrected) is the cheap immediate move.

## 4. Vocabulary ledger (mulling register — gates unrun, falconry to sit beside mews/jess)

| Slot | Meaning | Status |
|---|---|---|
| reservable facility | one inventory entry — a machine *or* a cloud facility | candidates: *perch* (a place one bird occupies; occupancy-flavored), *weathering* (the yard where several hawks are set out; shared-flavored) |
| grant token | temporary, revocable hold issued from the inventory | candidate: *creance* — the training tether, a deliberately temporary line you can always haul back; would give JJSAM's placeholder "reservation" its elected name |
| claim mode | exclusive vs. commons-safe sharing | deliberately unnamed until a consumer proves the need (the verdicts precedent) |
| the declaration | a suite/fixture's stated needs | likely needs no noun — a member on an existing registry row |

Disqualified on sight: *covert* (covert-channel is security prose — poison in
this repo). All candidates re-ratify under MCM Lapidary at election; no gates
were run this session.

## 5. Swingletree tag registry (SUPPOSITION — recommendation awaiting operator yes)

Edge-type words (specifies / tests / crosscuts) should be neither a code enum
nor free text: declared once in the table's own registry (the blaze pattern),
referenced by rows, undeclared tags refusing at the write door. Rationale:
enums are for mechanical consumers; the graph's only consumers are
judgment-tier (school, matricula lint, operator audit), and a frontier session
reads a plain word fine. The engine stays name-blind (names in data, shape in
code; the BURE enforce-shape-never-census pattern). A tag graduates into code
only when a mechanical consumer someday branches on it — the existing
deliberate-graduation rule.

## 6. MVP downscope (SUPPOSITION — recommendation awaiting operator yes)

The refreshed domain census (same-day edit: builds table added to
`memo-20260707-rbm-domain-census.md`) shows *build*, *test*, *lint* recurring
across every domain, with an idiosyncratic tail (deploy, assay, render,
ceremony acts, charge/quench) fitting no small closed enum. Recommendation:
keep the three fixed check members as canon has them; declare the tail out of
tackle's scope (those verbs have homes — tabtargets and workbenches); build no
actions layer until a consumer exists. The verb census is the evidence, not a
vibe.

Gait stays out of the tackle MVP. The confidence in the school/breeze division
constrains only *non-foreclosure* (tags and edges stay data; blaze indirection
stands), which the canon schema already delivers. Pulling gait forward would
invert two standing rulings (tackle-waits-for-footing; boxes before arrows).

## 7. Ratification ledger

Operator-ratified this session: staleness execution lives JJ-side (§1);
commit-before-build preferred (§1); `--locked` + remap actively required as
intent (§2). Everything else in this memo is SUPPOSITION awaiting explicit
operator yes, and the banked sheaf edits must carry the unreviewed-supposition
register until upgraded.

Banked-edit targets identified (not yet applied): JJSAT Reconciliation
obligations (build-flag rollout; actions-bullet dissolution; tag-registry
lean), JJSAM open forks (reservation widens beyond nodes to facilities), RCG
build-invocation section paired with its enactment, domain-census memo
exclusivity column.

## 8. Sources

- This chat (260720, JJ maintenance window; census memo refreshed in-session).
- `Tools/jjk/vov_veiled/JJSAT-tackle.adoc`, `JJSVT-tackle.adoc`,
  `JJSAB-breeze.adoc`, `JJSAM-mews.adoc` — the sheaves read and cited.
- `Tools/vok/vov_veiled/RCG-RustCodingGuide.md` — emplacement survey.
- `Tools/rbk/vov_veiled/RBSHC-hierophant_cosmology.adoc` — the cachet
  precedent.
- `Memos/memo-20260707-tackle-polarity-and-graph.md` — the failure-asymmetry
  argument this memo replays twice.
- Cargo/toolchain facts verified against the tree 260720 (eleven lockfiles,
  one toolchain pin, no cargo config).
