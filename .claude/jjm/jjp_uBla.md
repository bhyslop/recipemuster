## Shape

This heat creates the **worksite repo** and the **atomic Rust manipulators** that own
its read/write/commit, and **rebalances the fundus/curia/BURP/BURN files** into the
worksite model (name provisional): an operator-scoped, repo-shaped home that any number
of discrete projects share. It carries an unresolved design core — chiefly the
worksite's git-database representation — and is stood up now to **hold** the decisions
and open questions below until the operator can focus on resolving them. Today's act is
capture, not construction; the build waits until the design core settles.

Spine is the worksite model. BURN/BURP (fleet topology) and curia/fundus (operation
roles) are reorganized into it, not merely referenced.

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
- **Writers must be isolatable; single-blob storage is disqualifying.** Single-blob is
  the shared root of merge pressure, global-lock contention, and round-trip-validation
  cost — and under shared-sync it fails outright (every write contends on one file;
  every rebase is a 3-way merge of canonical JSON). Two stations on different heats must
  push without conflict. Per-heat sharding is the natural mechanism; the exact
  representation is the git-database fork (Held) — principle is cinched, mechanism open.
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
- **BURN/BURP is BUK substrate JJ defers to**, not JJ-owned. The rebalance relocates
  these files into the worksite home (co-residence), but JJ resolves by alias and owns
  nothing of their semantics — co-residence, not absorption.
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

- **Worksite git-database representation — the design core that gates the build.** A
  peer to the gallops JSON (further JSON stores committed alongside), or something more
  primal (git objects/refs as the database directly). Git-stays-truth holds either way;
  this decision sets the data shape and subsumes the sharding mechanism above.
- **Scope boundary of Ba vs follow-on heats.** Which of {gallops sharding rollout,
  station-partitioned minting, the full cross-station pull-rebase-push sync loop} land
  in this heat versus later. Settled once the git-database fork resolves.
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

The worksite repo exists and its atomic Rust manipulators (validated read/write/commit
plus lock) are built and tested; the fundus/curia/BURP/BURN files are rebalanced into
the worksite model; the git-database representation and the other Held forks are
resolved-and-recorded; and the Ba-vs-follow-on scope boundary is decided. No
construction begins until the design core (the git-database approach) is settled.

## Character

Design requiring judgment, then a careful build — architectural model-setting plus the
worksite repo and its atomic manipulators. Standing now as a holding pen for settled
decisions and open forks; construction waits until the operator can focus and resolve
the git-database core. Resist building before the model sets.