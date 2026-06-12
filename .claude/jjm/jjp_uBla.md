## Shape

This heat founds a **new kit** — the strongly-controlled home for fleet provisioning
and shared operator state — sitting between BUK and JJK in the dependency stack:
the kit depends on BUK (bash/tabtarget surface only),
JJK depends on the kit,
and BUK never depends on it.
Its charter has two halves:

1. The **mews** (name soaking — falconry register, see Naming in Held;
   replaces the retired "worksite" placeholder):
   an operator-scoped, repo-shaped home that any number of discrete projects share,
   with **atomic Rust manipulators** owning its read/write/commit.
2. **Fleet provisioning**, absorbed from BUK jurisdiction:
   creating precise users on remote nodes,
   managing their keying environment for code access,
   and normalizing them for extremely reproducible testing and simultaneous operation.

It carries an unresolved design core — chiefly the mews git-database representation —
and is stood up now to **hold** the decisions and open questions below until the
operator can focus on resolving them. Today's act is capture, not construction; the
build waits until the design core settles.

Spine is the mews model. BURN/BURP (fleet topology) and curia/fundus (operation
roles) reorganize **into the kit** — absorption, revised from the earlier
co-residence stance (see Cinched).

## Cinched

- **Decouple JJ state from work repos** into a shared singleton mews. The married
  model contaminates the work repo's git preconditions — the pensum-seed eviction is
  the prior symptom (JJ bookkeeping commits advanced HEAD past origin and tripped
  foray's curia-readiness guard). Decoupling dissolves that contamination class, not
  just one instance. This is the case for decoupling, weighted above merge relief.
- **Git stays source-of-truth and journal — not a transactional store.** The covenant
  survives: additive-only history, parseable commit-as-intent, jjx_log reconstruction,
  free immutability. A transactional store would force re-establishing all of them.
- **Central/shared across stations, synced by a JJ-owned pull-rebase-push loop.** The
  human never hand-manages the mews's git. Git push is an atomic compare-and-swap
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
- **Asymmetric sync.** The mews (authoritative) is tight: gallops lifecycle ops
  bracket as pull-rebase -> lock -> mutate -> push -> unlock. Repos-under-test are
  SHA-referenced only and sync lazily (eventually-consistent).
- **Generalize, don't invent, for cross-repo references.** The basis field (per-tack
  HEAD SHA, with the unknown-sentinel for "can't determine") is already a (repo, SHA)
  reference — lift it to (work-repo, SHA). The legatio/BURN registry already names
  external targets — the work-repo registry is its sibling.
- **The legatio (alias, reldir) pair is the factoring:** alias -> fleet topology
  (leaves the repo, operator-scoped); reldir -> project location (stays with project).
- **The kit absorbs BUK jurisdiction** (revised 260612; formerly "co-residence, not
  absorption").
  The jurisdiction verbs (caparison / invigilate / garrison) and their regimes
  (BURN, BURP) migrate out of BUK into the kit;
  BUK returns to a pure utility kit holding no fleet semantics.
  Rationale: the charter — creating users and installing keys on remote machines —
  IS garrison; owning the responsibility without owning its verbs would split the
  audit perimeter the kit exists to close.
  JJ still resolves by alias and owns nothing of the regime semantics;
  the deferring party simply changes from BUK to the kit.
  ₣A- remains jurisdiction's heat of record until the migration pace lands.
- **Containment is the kit's founding argument.** Code that creates remote users and
  installs key material is the most privileged code in the estate;
  the kit confines it to one audit perimeter with one declared caliber bar.
  Caliber by surface: the state core (git database, lock, CAS sync, manipulators)
  is Rust under RCG;
  remote-transport surfaces stay bash under BCG+WSG —
  the Windows transport stack is a Palisade membrane of hard-won empirical
  knowledge and is not rewritten in Rust for purity.
  "BCG or better" is the floor everywhere.
- **Mews git-engine = vvx (Rust); fleet-artifact semantics live in the kit, git
  mutation does not leave the engine.** Verified grain: BUK does no git mutation today
  (only git status); all mutation plus the lock (refs/vvg/locks/vvx) lives in the Rust
  vvc crate; vvx already reads burn.env artifacts directly. One engine, one lock — two
  git drivers on a co-tenant store would race on a single station the way two stations
  would. Kit identity and binary identity stay decoupled: the kit's machinery is
  delivered through the vvx binary exactly as jjx already is; VVK remains the generic
  engine kit, this kit owns the fleet semantics riding it.
- **Read/write asymmetry — many readers, one writer.** Other kits may peek the mews,
  but only their own domain (fleet topology, reservation tokens) directly. Gallops
  awareness routes through a vvx read path — never raw JSON (Exclusive JSON
  Ownership). Reads take no lock and tolerate slight staleness.
- **Kit + JJ core are project-agnostic, zero RBK coupling.** JJ is more general
  than RBK (used on any tracked project); the right relayering is extraction/delivery
  boundaries, NOT a runtime RBK->JJ dependency. RBK and JJ stay dependency-independent
  siblings. The cross-project mews forces this. JJ's core stays BUK-independent by the
  same split the kit itself has: Rust core free of BUK, bash/tabtarget surface on BUK.
- **Parallel fleet testing is a chartered consumer, not a side effect.** Reservation
  tokens + normalized nodes are the substrate for running test ladders simultaneously
  across windows/macos/linux fleet nodes —
  each node serial within itself (the per-station regime/namespace constraint is
  unchanged), nodes parallel with each other.
  This makes theurge/RBK suites a consumer of the kit;
  the dependency points RBK -> kit, never the reverse.
- **Falconry asterism elected for soak** (260612). Fixed anchors: **mews** = the
  operator-scoped home; **jess** = the key tether binding a provisioned user.
  The legend is otherwise deliberately unminted — no hawk mapping, no verbs, no kit
  prefix until the register survives daily voice.
  Deliberation record, register survey, and grep gates:
  `Memos/memo-20260612-heat-Ba-falconry-asterism-soak.md`.

## Held (refine before cutting paces)

- **Mews git-database representation — the design core that gates the build.** A
  peer to the gallops JSON (further JSON stores committed alongside), or something more
  primal (git objects/refs as the database directly). Git-stays-truth holds either way;
  this decision sets the data shape and subsumes the sharding mechanism above.
- **Scope boundary of Ba vs follow-on heats.** Which of {gallops sharding rollout,
  station-partitioned minting, the full cross-station pull-rebase-push sync loop,
  the jurisdiction migration out of BUK} land
  in this heat versus later. Settled once the git-database fork resolves.
- **Mews clone location.** A gitignored subdirectory of each work repo (one clone per
  work repo — clones as disposable caches, remote CAS the only authority, relative
  paths simple) vs one operator-scoped clone at a station path. Interacts with the
  project-dimension fork below.
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
  (work execution), git to the mews (state). With the kit founded, the structural
  question resolves toward: the mews is kit-owned infrastructure the curia rides.
  The role *words* (curia, fundus, legatio, pensum) are a naming question — folded
  into Naming below.
- **Project dimension in gallops.** With many projects sharing one mews: does a
  heat carry an explicit project, or is project implicit via the work-repo its paces'
  SHAs reference?
- **Naming.** Falconry register is SOAKING (see Cinched anchor entry + memo). Open
  within it: whether the register survives daily voice across several paddock
  revisions; the hawk mapping (settle before any verb mints — sibling-initials binds
  the verb family from birth); the kit's own name and 2-4 char prefix (full minting
  workflow: namespace enumeration, terminal exclusivity); the ashlar budget.
  Standing anti-constraint independent of register: no `work-` stem (workbench
  collision). Disposition of the living foray words (fundus, curia, legatio, pensum):
  untouched during the soak — MCM constrains births, not the living; a deliberate
  remint pace decides migrate / translate / stand-as-dialect when the responsibility
  actually moves.
- **Lock-as-protocol escape hatch.** If a real "fleet op must run where vvx is absent"
  context appears, define the mews lock as a language-agnostic protocol (lock-ref +
  CAS-on-push discipline) so a thin bash driver could participate without code-sharing
  or inverting the layer stack. Hold; do not build speculatively.
- **vvx/JJ extraction from the RBK-centric monorepo** — delivery/workspace decoupling.
  The kit boundary is the natural extraction seam; still deferred.

## Done when

The kit exists with the mews repo and its atomic Rust manipulators (validated
read/write/commit plus lock) built and tested; the fundus/curia/BURP/BURN files are
rebalanced into the kit; the jurisdiction-absorption boundary with BUK is recorded
(even where the migration itself is deferred to a follow-on heat); the git-database
representation and the other Held forks are resolved-and-recorded; and the
Ba-vs-follow-on scope boundary is decided. No construction begins until the design
core (the git-database approach) is settled.

## Character

Design requiring judgment, then a careful build — architectural model-setting plus
the mews repo and its atomic manipulators, under a containment charter that holds the
estate's most privileged code to its highest caliber bar. Standing now as a holding
pen for settled decisions and open forks; construction waits until the operator can
focus and resolve the git-database core. Resist building before the model sets.
Resist minting beyond the two soaking anchors before the register proves itself.

## Sources

- `Memos/memo-20260612-heat-Ba-falconry-asterism-soak.md` — naming deliberation,
  register survey, grep gates, soak protocol.
- ₣A- — jurisdiction's heat of record (caparison/invigilate/garrison, BURN/BURP)
  until the migration pace lands.
- MCM § Word Selection — asterism doctrine governing the soak.