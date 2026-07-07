# Tackle: inclusion polarity, the SlickEdit autopsy, and the association graph

Design-session record, 2026-07-07 (Fable session, operator Brad).
Provenance, never authority: the settled leans live in the tackle aspirant sheaf
(`Tools/jjk/vov_veiled/JJS-aspirant-tackle.adoc`); this memo preserves the reasoning,
the specimen evidence, and the explorations that did not graduate.

## 1. The question

How should a tackle fileset be defined?

- **Positive inclusion** (allowlist): the set is named by membership globs; anything unlisted is out.
  Explicit, safe-by-default-empty; must be maintained as files are added; can silently *under*-cover.
- **Negative inclusion** (denylist): the set is a universe minus carve-outs; new files fall in automatically.
  Auto-covering; can silently *mis*-affiliate — a new file lands under a discipline nobody chose.

Stakes: tackle affiliates files to the rules that govern them, so mis-affiliation applies the
wrong discipline — potentially the wrong security posture.

## 2. Position

Positive claims, at domain grain, closed by a mechanical coverage check whose remainder is loud.
The dichotomy fuses two separable things: *how a set is defined* (polarity) and *what happens
to a file nothing claims* (the default).
Sets are defined positively; there is no default; the universe negative inclusion wanted still
exists, but it moves from the binder to the auditor.

## 3. The decisive argument: the failure modes are not symmetric

- **Under-coverage is mechanically detectable.**
  Git-tracked files minus the union of claims is a set subtraction any validator runs, with no
  judgment required.
  The remainder is a stray report; each stray is repairable the moment it is seen.
- **Mis-affiliation is undetectable by construction.**
  The table *is* the definition of "right", so no checker can know a file landed under a
  discipline nobody chose for it.
  The error surfaces only when the wrong discipline misbehaves — for security-posture
  bindings, discovery by incident.

Given teeth, choose the failure mode you can automate away.
This is the same reasoning already settled elsewhere in the house doctrine: BUK's tweak-shape
gate exists so a typo'd tweak fails loud instead of silently no-op'ing; the ROE's Palisade
conduct absorbs only surveyed signatures and fails fast on everything else.

## 4. Prior art

- **Firewalls**: the identical debate, settled decades ago as default-deny / explicit-allow,
  because default-accept's failure mode is silent exposure.
- **gitattributes**: path pattern → attribute set; positive; evaluated lazily per-path;
  scales to million-file repos.
  The closest structural relative of tackle.
- **CODEOWNERS**: path pattern → owner; positive, last-match-wins; unowned files are surfaced
  *visibly* rather than defaulted — the loud-remainder pattern.
- **gitignore**: negative-with-renegation (`!`), last-match-wins; famously confusing —
  evidence that mixed-polarity rule stacks carry high cognitive cost.

Note: gitignore/gitattributes scale partly via *in-tree distributed* rule files.
The vanilla-hippodrome premise (hippodromes carry no JJ inserts) forecloses that trick for
tackle, which must be centralized-external (pedigree-side) — making the intensional-table +
live-evaluator design mandatory rather than optional.

## 5. The SlickEdit autopsy (specimen: `Tools/vslf-rbw/`, `Tools/vslk/`)

A prior attempt at the same theme — IDE workspace construction — studied as evidence, not
as a system to fix.

### Structure

- `vsp-rbw-main.vpj`: positively includes `*.sh`/`*.mk`/`*.recipe` recursively from root,
  *negatively* excludes `tt/*` and `.buk/*` — mixed polarity in one set.
- `vsp-rbw-tabtargets.vpj` (`tt/*.sh`, non-recursive), `vsp-rbw-launchers.vpj` (`.buk/*.sh`),
  `vsp-rbw-markdown.vpj` (`*.md`), `vsp-rbw-asciidoc.vpj` (`*.adoc`): pure positive globs,
  one per discipline-ish slice.
- `vsw-rbw-ALL.vpw` / `vsw-rbw-source-only.vpw`: workspaces bundling projects — a two-level
  grouping (files → project → workspace).
- Installer (`Tools/vslk/`): template-copy with `__VSLW_PROJECT_DIR__` substitution at
  install time.

### Diagnosis: the mixture is accidental to polarity, essential to disjointness

Main's `Excludes="tt/*;.buk/*"` are not denylist philosophy — they are *shadows of sibling
sets' positive claims* (tabtargets and launchers are separate projects).
In a flat per-set language with no cross-set awareness, disjointness must be hand-encoded as
exclusions that mirror another set's inclusions.
Generalization: in any family of hand-authored sets over one universe, either the language
provides cross-set resolution or authors hand-encode set-complement fragments of each other,
which drift.

### Live rot evidence (verified against the tree, 2026-07-07)

- `.buk/` no longer exists; launchers moved to `rbmm_moorings/rbml_launchers/`.
  The launchers project's claim went dead **and** main's exclusion went dead — so main's
  recursive `*.sh` now silently absorbs the launcher domain under the wrong grouping.
  Live mis-affiliation, caused precisely by hand-mirrored disjointness, silent in the
  dangerous direction (the general set swallows the moved domain).
- `RBM-recipes/` does not exist; zero `*.recipe` files tracked.
  Zero `*.mk` files tracked.
  Dead positive claims rot silently but fail *safe* (missing files in an editor view) — the
  asymmetry argument in miniature.

### Lessons carried into the sheaf

1. The table must be family-aware: one table with resolution semantics, never independent
   per-set definitions.
2. Overlap between a general claim and a nested specific claim must be resolved by the
   system, deterministically and visibly — never by hand-mirrored exclusions.
3. Dead-row detection (a claim matching zero files) is a cheap checker that would have
   caught all three rots.
4. The two-level grouping splits as: project ≈ tackle row (load-bearing);
   workspace ≈ consumer projection (belongs to consumers, not the table).
5. Within each `.vpj`, membership (Files globs) and presentation (CustomFolders filters) are
   two hand-maintained copies *inside one file* — even a single consumer drifts against
   itself.
   One authoritative table, mechanical projection.
6. Materialized extensions with no re-derivation path (template-copy at install) are
   staleness by design.

## 6. Domain grain: why positive inclusion's maintenance burden mostly evaporates

The classic objection — allowlists must be maintained as files are added — conflates file
churn with domain churn.
A tackle claim is domain-grain: an anchored subtree × shape conjunction.
New files inside a claimed domain auto-fall-in (auto-coverage exactly where it is correct,
because the domain owner chose the criterion once); a *new domain* is ceremonial (the
pedigree posture one level down).
Corollary: with domain-grain claims, **placing a file is affiliating it** — the location
decision that already happens is the discipline decision.

Guards that keep the ground positive:

- No catch-all row; humble disciplines (generated, inert asset, provenance-only) are
  first-class rows, so totality is reached deliberately, never by bucket.
- Recursion is preserved in claim semantics (placement-is-affiliation depends on it); the
  hazard is *anchor width*, so root-anchored recursive claims and bare-`*` shapes are audit
  flags, not prohibitions.
- Off-shape files inside claimed territory (a `.py` in a bash-claimed subtree) are strays
  (loud), not silently governed — shape constraints are load-bearing.

## 7. Scale: intension vs extension, and the three verdicts

The recursion/cost question splits into claim semantics vs evaluation.
The table is intensional — rules, never lists; every fileset is a derived projection.
Two query directions scale differently:

- **path → discipline** (saddle-time): O(rules) per path, no tree walk — scales to any tree
  size (the gitattributes model).
- **discipline → fileset / coverage audit** (census): needs universe enumeration; batch;
  cacheable keyed (table version, tree state).
  A materialized roster is derived per-clone state → station-local scratch, never the
  studbook (same law as worktree paths).
  Caching at scale is the deliberate supersession VOSMM's transience invariant (VOr_k3p)
  already anticipates.

For large vanilla upstreams, total coverage at founding is absurd, so the governed universe
is itself a declared, positive, per-upstream scope (plausibly pedigree-seated), yielding
three mechanical verdicts per path: claimed / unclaimed-inside-universe (defect, loud) /
beyond-universe (known-unknown; a session touching one is told so).
The same boundary bounds census cost: the coverage walk prunes beyond-universe subtrees.
Governance honesty and traversal cost are one boundary.

## 8. The association graph (the "RAG tree, declared not deduced")

The worked RB census (see sheaf) forces two discoveries:
the operator thinks domains-first (globs are how a domain grabs its files, so the binding
factors globs → domain → everything else), and disciplines attach to edges, not only nodes
(ACG is intelligible only as governing the code↔spec *relation*; tests-subject and
specifies are further typed edges).
Scope containment is a tree; declared interrelations make it a graph.

"Declared, not deduced" is load-bearing: every edge is authored and traversal is mechanical —
no similarity inference (VOSMM VOr_m7w ethos applied to context provisioning).
The economics: a frontier-tier planner's judgment ("this implementation needs balancing
against this spec") is captured once as an edge and replayed mechanically; a weak model
cannot discover missing context, so the declared graph is the *precondition* for handing
work down-tier.
Same freeze pattern as the grimoire: authored once with full wisdom, read forever by exact
match.

Related confirmations from the session:

- The CLAUDE.md `@`-include tree, suites table, acronym maps, VOSMM allowlist, and SlickEdit
  files are all hand-maintained projections of the not-yet-existing table — tackle is
  normalization, not new machinery.
- The farrier sheaf's honing repairs (conduct core + pull door) ride along into tackle's
  consumer story: honing fails silent, and selection is safe only when paired with
  discoverability.
- The grimoire deliberately went one flat cross-project table (words travel); tackle is
  deliberately per-upstream (file domains do not).
  The two sit on opposite sides of the per-upstream split for stated reasons — recorded so
  nobody later "unifies" them.

## 9. Naming ledger (explorations, disqualifications, holds)

Recorded so dead ends are not re-tried; all candidate words re-ratify under MCM Lapidary at
graduation, gates unrun today.

- **palisade** (for beyond-universe territory) — REJECTED: the ROE Palisade names
  jurisdiction-impossible (cannot edit the source of failure); tackle's boundary is
  jurisdiction-not-yet-extended (can edit, haven't legislated).
  Overloading a load-bearing ROE word breaks one-word-one-concept.
- **trace** (for the interrelation) — DISQUALIFIED: trodden (stack trace, tracing).
- **yoke** — DISQUALIFIED: RBK-minted (RBSDY, `rbfly_yoke`).
- **harness** — DISQUALIFIED: trodden in ambient agent/tooling prose.
- **girth** (for the claim rule) — SET ASIDE: near-neighbor of RBK's minted *gird*
  (RBSPG `rbgp_gird`); the census precedent (seat/unseat) treats near-neighbors as
  disqualifying.
- **scope** (for the claim rule) — serves as prose placeholder; trodden, must not mint
  as-is.
- **remuda** (for the derived roster) — HELD: the working herd a ranch draws mounts from;
  rare, register-true; gate unrun.
- **hame** (for the interrelation) — HELD: the collar-piece traces attach to; harness
  register; obscure, but edges are not ashlar-tier.
- **swingletree** (for the interrelation) — HELD: the crossbar that balances the pull of two
  traces; meaning-apt for "balance implementation against specification"; long.
- **estray question** — OPEN: file-level unclaimed-inside-universe rhymes exactly with
  VOSMM's token-level `vosme_estray` (unclaimed-but-ours, surfaced as product).
  One generalizing concept or two words: a genuine Lapidary fork, resolves at tackle
  graduation.

## 10. Banked nudges to sibling sheaves

Mirrored in the sheaf's Reconciliation section (that copy is the live one): JJSAB pointer
from its tackle-execution-layer bullet to the association graph; JJSVS pedigree bullet
widens to the two-part charter and possibly seats the governed universe; farrier states
conduct-core/pull-door as tackle-consumer obligations; VOSMM estray naming and file-role ↔
bundle-facet alignment at graduation.

## 11. Session sources

- `Tools/jjk/vov_veiled/JJS-aspirant-tackle.adoc` (restructured this session)
- `Tools/jjk/vov_veiled/JJSVS-studbook.adoc` (pedigree anchor)
- `Tools/jjk/vov_veiled/JJSAB-breeze.adoc`, `JJS-aspirant-farrier.adoc` (consumption sites)
- `Tools/vok/vov_veiled/VOSMM-entity.adoc` (matricula: allowlist, VOr_k3p/m7w/q4f, file-role
  layer, grimoire residence)
- `Tools/vslf-rbw/*`, `Tools/vslk/*` (specimen)
- Operator wording 260707: the two-part charter; the dynamic-context future; gaits as
  vision, filesets + interrelating rulesets as base.
