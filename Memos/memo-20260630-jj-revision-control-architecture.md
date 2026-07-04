# Memo — Job Jockey Revision-Control Architecture (nucleation)

**Date:** 2026-06-30
**Status:** Superseded as mulling home (2026-07-04). The architecture matured into three
aspirant sheaves — `Tools/jjk/vov_veiled/JJS-aspirant-state-repo.adoc` (studbook),
`JJS-aspirant-farrier.adoc` (driver + dispatch), `JJS-aspirant-tackle.adoc` — which carry the
settled vocabulary (studbook, pedigree, farrier, saddle/billet/unsaddle, hippodrome) and
current design. Read this memo as provenance for the lock/insignia reasoning and the naming
alternates; the sheaves are the living surface. Aspirant, not authority.
Scope grew across the 2026-06-30 chats from "a lock service" to the whole JJ
revision-control + state-persistence architecture for the mews heat (₣Ba) and the
gallops heat (₣Bk). No sheaf yet — its name awaits more clarity. Both heats' paddocks
point here. Read as provenance, never authority; released code must cite nothing it mints.

## The three repos

| Repo | Holds | Concurrency posture |
|------|-------|---------------------|
| **gallops** (state) / its repo (candidate: *cartulary*) | the journalled planning record + full history | unpartitioned single record → **locked** |
| **mews** | the configured-servers-at-a-worksite fleet store | unpartitioned single record → **locked** |
| **sourcebase** (candidate: *turf* / *heath* / …) | the code being jockeyed | partitioned (branches/worktrees) → **lock-free** |

One JJ engine lifecycle-manages all three; the LLM and operator reach them only through
JJ's revision-control verbs (the membrane), and barely even then.

## The three-serializer closure (the locking picture is complete)

Every shared-state point has exactly one serializer — no unhandled shared state:

1. **gallops / mews** — automated **git-CAS lock** (an unpartitioned single record git's branch model does not protect).
2. **source feature work** — **no lock**; git's own ref-CAS + per-officium worktrees + one-branch-per-unit-of-work partition the writers.
3. **source trunk** — **the operator, manually, at review/approval.** Feature worktrees merge to the first-class sourcebase under the operator's eye; the human is the trunk's lock.

Rationale (failure-mode asymmetry): source without a lock fails *loud* (git non-fast-forward
rejection, recoverable by rebase); the gallops without a lock fails *silent* (divergent
writes / merge hell). Git protects history; it does not protect a single shared blob edited
by many. Lock the blob; leave history to git.

## The lock (gallops / mews only)

Pure git, no other service.

- **CAS = a held lock ref** in a dedicated namespace (provisionally `refs/jj/locks/<repo>`),
  outside `refs/heads/*` so lock churn never touches history. Whole-repo scope, one lock per
  repo (held only briefly).
- **Acquire = pull / push / pull.** pull (sync lock state) → push (the CAS: atomically create
  the stamped lock ref, or be rejected) → pull (confirm the held stamp is ours).
- **Stamp** on the lock object: officium, station/clone, acquire-time, operation — what makes
  break-on-error *safe* (VVG's lock carries none; this is the new surface).
- **Object model = RAII guard:** object-lifetime = lock-lifetime; `Drop` releases on
  stack-frame exit. **Panic on nested acquire** (thread-local held-flag set in the
  constructor, cleared on drop) — non-nesting is unforgeable, not merely documented. Remote
  release can fail and `Drop` cannot propagate it → best-effort delete + lean on the warden to reap.
- **Break-safety (load-bearing):** giving the warden break power creates a slow-but-alive-holder
  race. Two-part pure-git fix: (1) the warden breaks with a **lease** (`--force-with-lease` to
  the exact stale stamp SHA), never blind; (2) the holder's content push **re-asserts ownership
  atomically** (`git push --atomic` binding the content update to a `--force-with-lease` on its
  own lock ref). If the lock was broken under the holder, the whole push fails. No corruption, no service but git.
- **Break is a separate entity (the "warden")** — confirmed. The holder's *guard*
  (acquire/release/RAII) is one entity; observe-and-break is a *different actor* acting on
  someone else's lock, so it is its own entity, not a method on the guard instance.
  Observe = `ls-remote refs/jj/locks/*` + read stamp; break = lease-guarded delete.

## The VCS driver (source + gallops, coherent on *pins*, not locking)

JJ exposes a polymorphic revision-control layer over backends (plain git | submodules |
subtrees | Android `repo`). The gallops repo is the degenerate single-repo case.

- **Coherence is on the pin manifest, not the lock.** Both drivers "drive a git backend + emit
  a pin (member→SHA)"; locking is a property of *what kind of state the driver fronts*
  (unpartitioned → locked gallops/mews; partitioned → lock-free source), not of the interface.
- **Worktree-ops and lock-ops are complementary facets** — worktrees serve *partitioned* state
  (source), the lock serves *unpartitioned* state (gallops/mews). A repo uses one facet or the
  other by its nature.
- **The source driver's real job is to enforce the discipline that substitutes for the lock** —
  per-officium worktrees, one branch per unit of work (branch-naming §F), fall back to git
  push-CAS + rebase on collision. Discipline made structural, not hoped-for.
- **The membrane:** the LLM uses JJ's revision-control verbs and is cautioned heavily not to
  reach past the layer (raw `git` on the source). Transparency-plus-don't-reach-past, not a
  blindfold — a consistent extension of the existing "never reach past the JJK interface to raw
  storage / never read regime files directly, go through the CLI." Lives as a managed CLAUDE.md section.

**Op list (kept deliberately tight):**

| Op | Facet | Options / shape |
|----|-------|-----------------|
| `status` | inspect | short/porcelain |
| `diff` | inspect | range; `--stat` vs full; pathspec |
| `pin` | inspect | snapshot member→SHA (= HEAD for a single repo) |
| `commit` | mutate | explicit file list + message; no `-a`, no amend (additive discipline) |
| `fetch` / `push` | sync | push lock-facet adds `--atomic` + lease |
| `worktree_create` | worktree | **`path`, `branch=yyy`, `at=XX`** — keystone; births the partition; `at` defaults to trunk tip |
| `worktree_remove` | worktree | `path` |
| `ref_cas_create` / `ref_cas_delete` / `ref_read` | lock | gallops/mews only; consumed by the guard + warden |

Deliberately omitted: `merge`/`rebase` (trunk merge = operator; feature rebase added only if
needed), `amend`/`reset`/`clean` (additive discipline), `log`/`blame` (operator's git or
`jjx_log`), `clone`/`init` (rare admin). Coarser chunking of these ops is a flagged later topic.

## Cross-repo SHA and transaction ordering

- No cross-repo atomicity → **sequence, durable-first.** `record` commits the work-code repo
  first (its branch) to get the SHA, then journals it into gallops under the gallops lock. The
  durable artifact commits first; the journal references it second; it never owns it. Orphaned
  work-commit = recoverable; dangling gallops record = the worse failure.
- **`jjdcm_basis` generalizes** from scalar SHA → constellation **pin manifest** (member→SHA),
  with single-repo as the one-element degenerate. A submodule gitlink / repo-tool manifest /
  subtree-split set are all the same pin object. The driver translates to/from the backend's
  native form; the gallops stores the logical pin. Constellation backends are a future driver effort, not today's.

## Revision insignia (linear repos, aliased hashes)

- The gallops and mews repos are **linear — they never branch** — and are
  **single-writer-under-lock**, so a monotonic SVN-style ordinal over each is clean and
  authoritative (the property that forces the lock is what makes the clean sequence possible).
- Each repo's commits get a glyph-prefixed **insignia** (`axt_insignia` — the family of
  Firemark ₣ / Coronet ₢ / Officium ☉) aliasing the underlying SHA. The ordinal is a
  **denormalized label, never the identity** — the SHA stays truth (the `jjdcm_basis` rule).
- **The glyph's presence is the sourcebase/tooling discriminator:** a glyphed revision = a
  JJ-managed tooling repo; a bare git SHA = the sourcebase; *which* glyph = *which* tooling repo.
- These are **ashlar** (they surface only on failure — "words in failure output are ashlar");
  otherwise hearting (JJ-lifecycle-managed, invisible to LLM and operator).
- **Deferred sub-mint:** two glyphs (codepoint-distinct from ₣₢☉) + two names — archival
  register for the cartulary's revision, falconry for the mews's ("folio" is taken by `BUZ_FOLIO`).

## Naming candidates (grep-gated, NOT adopted)

Principle: **name each repo in the register of what it holds**; the set is **bondstones**
(reasoned-through daily) → first-letter separation + fair-faced.

- **Record repo — recommend `cartulary`** (archival: a bound register of charters/records — a
  git repo of planning history literally is one). `gallops` *stays* the live state-concept; this
  splits the double-duty the operator flagged (logical state vs. physical vessel — a real
  two-concept split, not an alias). Warmer alternative: `daybook`.
- **Sourcebase — open fork (highest-traffic bondstone, wants a warm word).**
  - *Ground / substance* family: **`turf`** (operator's favorite), **`heath`** (a real racing
    place that is also actual ground), `glebe` (richest meaning, rarer), `sward`, `steading`.
  - *Venue / contest-space* family: `tiltyard` (the jousting arena), `hippodrome`, `arena`,
    `the lists` (leached — unmintable).
  - The two families encode different mental models — the sourcebase as *ground you cultivate*
    vs. *arena where work contends*. Husbandry favors warm (turf/heath); audible-register favors
    a non-equestrian register (glebe) to mark the object-layer apart from the planning layer.
- **Mews — keep** (already a clean falconry bondstone; promoted from concept to repo).
- **Peer home** (the projects-level enclosure peers ring, `../XXX`): seed **`garth`** (cloister
  garth). `close` is perfect-but-leached. Light flag; awaits layout firming. JJ's own code
  probably needs no new bondstone (it is the kit).
- First-letter check on the held set: Cartulary (C), Turf/Heath/Glebe (T/H/G), Mews (M),
  Garth (G) — distinct except glebe + garth share G.

## Open forks (the mulling surface)

- Sourcebase register choice (ground vs. venue; warm vs. distinct-register).
- The sheaf's name and shape (deferred — "more clarity after a chat or two").
- Insignia glyphs + names (the deferred sub-mint).
- Whether `record` *pushes* the work repo (curia-readiness interaction), and recording the
  `jjdk_sole_operator` "reads take no lock" change as a deliberate premise-edit.
- Coarser-chunked driver ops; auto-branch / auto-push behaviors; mews-assured remote-clone
  sync; SVN-style sequence ergonomics.

## Provenance (not authority)

- VVG lock — `Tools/vok/vov_veiled/VOSRL-lock.adoc`, `Tools/vvc/src/vvcc_commit.rs`
  (`vvcc_CommitLock`), `Tools/vok/src/vorm_main.rs` — the local-CAS template generalized to remote.
- `Tools/jjk/vov_veiled/JJS-aspirant-state-repo.adoc`, `JJS-aspirant-mews.adoc` — the two sheaves.
- JJS0 — Crash-Safe Architecture (`lock → load → transform → save → unlock`),
  `jjdk_sole_operator`, the `jjdcm_basis` tack member, the `pensum_seed` eviction (JJS0:1547),
  the entity hierarchy worked in `JJSCGZ-gazette.adoc`.
- AXLA — `axo_entity`, `axt_insignia`, the `axhe*` entity hierarchy; MCM Lapidary (bondstone,
  asterism, husbandry, the Soil Family).
