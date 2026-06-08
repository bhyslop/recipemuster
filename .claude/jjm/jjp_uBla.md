## Shape

Decouple Job Jockey's state from the work repo and re-home it — together with the
operator's fleet topology — into a single shared **worksite** store (name
provisional): an operator-scoped, repo-shaped home that any number of discrete
projects share. This is a **spec-first** heat. Settle the model and voice it in the
spec(s). Store reshape, minting change, sync machinery, and moorings relocation are
deferred to follow-on heats that cannot be clear-and-present until the model sets.

Spine is the worksite model. BURN/BURP (fleet topology) and curia/fundus (operation
roles) are two tributaries it unifies — kept conceptual here, not refactored.

## Cinched

- **Decouple JJ state from work repos** into a shared singleton worksite. The married
  model contaminates the work repo's git preconditions — the pensum-seed eviction is
  the prior symptom (JJ bookkeeping commits advanced HEAD past origin and tripped
  foray's curia-readiness guard). Decoupling dissolves that contamination class, not
  just one instance. This is the case for decoupling, weighted above merge relief.
- **Git stays source-of-truth and journal — not a transactional store.** The covenant
  survives: additive-only history, parseable commit-as-intent, jjx_log reconstruction,
  free immutability. A transactional store would force re-establishing all of them.
- **Central/shared across stations, synced by a JJ-owned pull-rebase-push loop.** The
  human never hand-manages the worksite's git. Git push is an atomic compare-and-swap
  on the remote; that CAS is the cross-station lock.
- **Per-heat sharding is the enabling precondition, not an optimization.** Single-blob
  storage is the shared root of merge pressure, global-lock contention, and
  round-trip-validation cost — and is disqualifying under shared-sync (every write
  contends on one file; every rebase is a 3-way merge of canonical JSON). Shards let
  two stations on different heats push non-conflicting changes.
- **Exclusivity is soft.** Reservation tokens grant node ownership; operations
  expire/reissue them; an operator can administratively reclaim a forgotten
  reservation. No distributed-consensus machinery — fits sole-operator.
- **Asymmetric sync.** The worksite (authoritative) is tight: gallops lifecycle ops
  bracket as pull-rebase -> lock -> mutate -> push -> unlock. Repos-under-test are
  SHA-referenced only and sync lazily (eventually-consistent).
- **Generalize, don't invent, for cross-repo references.** The basis field (per-tack
  HEAD SHA, with the unknown-sentinel for "can't determine") is already a (repo, SHA)
  reference — lift it to (work-repo, SHA). The legatio/BURN registry already names
  external targets — the work-repo registry is its sibling.
- **The legatio (alias, reldir) pair is the factoring:** alias -> fleet topology
  (leaves the repo, operator-scoped); reldir -> project location (stays with project).
- **BURN/BURP is BUK substrate JJ defers to**, not JJ-owned. It may co-reside in the
  worksite home; it is not absorbed. JJ resolves by alias and owns nothing of it.
- **Worksite git-engine = vvx (Rust); BUK owns fleet-artifact semantics, not git
  mutation.** Verified grain: BUK does no git mutation today (only git status); all
  mutation plus the lock (refs/vvg/locks/vvx) lives in the Rust vvc crate; vvx already
  reads BUK's burn.env artifacts directly. One engine, one lock — two git drivers on a
  co-tenant store would race on a single station the way two stations would.
- **Read/write asymmetry — many readers, one writer.** BUK may peek the worksite, but
  only its own domain (fleet topology, reservation tokens) directly. Gallops awareness
  for BUK routes through a vvx read path — never raw JSON (Exclusive JSON Ownership).
  Reads take no lock and tolerate slight staleness.
- **Worksite + JJ core are project-agnostic, zero RBK coupling.** JJ is more general
  than RBK (used on any tracked project); the right relayering is extraction/delivery
  boundaries, NOT a runtime RBK->JJ dependency. RBK and JJ stay dependency-independent
  siblings on BUK. The cross-project worksite forces this. JJ's core is even
  BUK-independent; only foray reaches BUK artifacts.

## Held (refine before cutting paces)

- **Advisory vs. authoritative tracked SHA.** Lean: advisory by default (degrades
  gracefully under rebase-away, as basis does today), with foray's curia-readiness
  guard the only authoritative point. Governs the entire reconciliation burden.
- **Station-partitioned firemark/coronet minting.** Root state (next_heat_seed,
  heat_order) is the irreducible shared point even with per-heat shards. Needs a
  partition scheme so heat creation requires no cross-station coordination; the
  pensum-seed "cross-officium collision acceptable" stance is the template.
- **Conflict policy** when two stations touch the same shard: CAS rejects, JJ retries a
  small number of times, then recommends an administrative action (still within the
  system). Open: what happens to a genuine content conflict on one heat-file.
- **curia/fundus re-focus.** The curia is a hub with two transports: foray/SSH to fundi
  (work execution), git to the worksite (state). Open: is the worksite a new role
  alongside curia/fundus, or curia-owned infrastructure? Reclaiming/reusing
  curia/fundus is in scope.
- **Project dimension in gallops.** With many projects sharing one worksite: does a
  heat carry an explicit project, or is project implicit via the work-repo its paces'
  SHAs reference?
- **Naming.** "worksite" and any new role term are provisional. Minting (namespace
  enumeration, terminal exclusivity, equestrian/nautical fit — note fundus already =
  Latin "estate") happens once the model sets, not now.
- **Lock-as-protocol escape hatch.** If a real "fleet op must run where vvx is absent"
  context appears, define the worksite lock as a language-agnostic protocol (lock-ref +
  CAS-on-push discipline) so a thin bash driver could participate without code-sharing
  or inverting the layer stack. Hold; do not build speculatively.
- **vvx/JJ extraction from the RBK-centric monorepo** — delivery/workspace decoupling.
  On the operator's mind; deferred.

## Done when

The worksite model is voiced in the spec(s) (JJS0 and/or a new worksite spec): the
Held forks resolved-and-recorded, new concepts minted, the curia<->worksite<->fundus
layering restated, BUK's deferral relationship documented, and implementation
explicitly carved out to named follow-on heats. No store reshape, minting change, sync
machinery, or moorings relocation lands in this heat.

## Character

Design conversation requiring judgment — architectural model-setting, spec-first.
Resist pre-baking implementation; the payoff is a model that sets cleanly so the
downstream implementation heats can be clear-and-present.