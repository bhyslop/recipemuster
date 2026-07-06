## Shape

The first implementation heat of the JJ revision-control MVP:
the studbook (regional record repo), the farrier (revision-control driver), and the dispatch layer,
built as one binary with parallel code paths under current JJ —
the in-repo gallops stays authoritative until the cutover ceremony, which is NOT this heat.

Authority is the three aspirant sheaves; dated cinches (260704) are settled and not re-litigated:

- `Tools/jjk/vov_veiled/JJS-aspirant-state-repo.adoc` — studbook, journal ceremony, counterfoil, guidon, pace-identity re-gestalt, founding-and-cutover.
- `Tools/jjk/vov_veiled/JJS-aspirant-farrier.adoc` — driver op census under the vocabulary-Palisade naming rule, lock guard, saddle/lunge dispatch, BUK meld.
- `Tools/jjk/vov_veiled/JJS-aspirant-tackle.adoc` — MVP tackle only; scope layering is post-MVP supposition, do not act on it.

The 260630 revision-control memo and 260702 collision memo are provenance, never authority.
A canon-workup groom (260705) added the specification architecture below;
its AXLA half is landed canon (sheaf genre axis, two-token axvd_sheaf grammar, chapbook marker family).

The roster is ordered judgment-first:
mint, design, and sheaf-authoring paces front-load for frontier-tier sessions;
the implementation tail runs at work tier.

## Cinched

- One heat at a time: the conversion heat is cut only after this heat lands,
  from the state-repo sheaf's Founding-and-cutover section, refreshed by build learnings.
- The pace-identity re-gestalt is a gallops schema change: reprieve-gated
  (JJS0 `jjdz_reprieve`; CLAUDE.md §E branch delivery, §F branch naming).
- The scratch-studbook rehearsal gate is this heat's final pace and the conversion heat's entry gate.
- Farrier op names are git-free above the trait (vocabulary Palisade, farrier sheaf);
  all word-picking under MCM Lapidary with repo-wide grep gates.
- Blotter (260705): the locked-store entity — a linear, single-writer-under-lock, engine-driven git repo.
  The journal ceremony and break sequence are its methods;
  the studbook and the mews fleet store are its instances;
  its own config is engine-known, never pedigree-resolved (a store cannot look itself up in itself).
  Word elected under grep gate; ratchet and cadastre held as alternates.
- Sheaf genres (260705): every sheaf this heat authors carries the two-token axvd_sheaf line
  (normativeness then genre) per AXLA Sheaf Genre Dimensions.
- Chapbook (260705): the stitched happy-path tale genre, landed in AXLA —
  actors with required internal/external kind, interactions citing the governing contract at either grain
  (operations at doors, routines within), attended/foreign licenses for the rare quoinless beat.
  First instance planned below.
- Billet sweep (260705): reaping is nuke-on-start — a plan-then-confirm sweep at dispatch time
  retires worktrees whose pace is closed, behind an age-retention window rhyming with officium exsanguination;
  refuse-with-advice on dirty anomalies by default, auto-commit-push behind explicit confirm;
  pace states snapshot under the studbook lock, then reaping proceeds lock-free (completion is monotonic).
  Supersedes the farrier sheaf's reaped-at-next-saddle-if-clean line (bank at drain time).
- Sweep liveness and retention (260705, slate-time): the liveness guard is the pure JJ-data join —
  live officia × billets, no platform process-probe (a probe only ever as belt-and-braces),
  which sequences the sweep pace after the officium re-gestalt amendment;
  the retention window equals the officium exsanguination window — one constant, one rhyme.
- Trunk resync (260705, softened same-day): promoted from spine-internal auto-merge to an explicit session-facing operation —
  the dispatch spine fetches but never merges;
  NO verb refuses on a stale trunk: trunk staleness is drift, never corruption
  (the billet branch is additive and the studbook always advances under its own lock),
  and refusing a notch would block the safe act of recording work.
  Instead jjx_open leads its report with the staleness warning and the recommended remedy,
  and notch/wrap append the same recommendation whenever trunk has moved.
  The remedy is the resync operation — a merge of trunk into the billet, never rebase
  (journaled counterfoils survive; rebase would dangle them);
  offline degrades to warn-and-proceed.
  The freshen re-mint (VOK collision) mints the new word for this operation, not the spine op.
- Resync push timing (260705, slate-time): the resync merge commit pushes immediately —
  the operation exists to clear staleness for every station and runs online by construction;
  offline stays warn-and-proceed.
- Journal ceremony home (260705): the ceremony and break sequence are spec-homed as their own routine-genre sheaf —
  one genre token per sheaf decides it;
  the blotter entity sheaf summarizes its methods and cites, never restates.
- Farrier facets (260705): the capability model is structural, not declarative —
  facet-split traits (core / lock / billet), a blotter carrier implementing core+lock, a hippodrome kind core+billet,
  so a kind lacking a facet is a compile-time fact;
  no runtime capability machinery in the MVP,
  and the degradation ladder stays spec prose until a second farrier kind arrives.
- Dispatch shell surface (260705): the doors are infield-resident trampolines under the jjy_ brand —
  jjy_saddle and jjy_lunge, two-line scripts (never symlinks: Windows stations) that exec into the kit repo's tabtargets,
  stamped by an idempotent kit-repo installer verb at founding.
  Invoked from within the elected clone: cwd elects the clone — the pace cannot,
  since parallel clones of one upstream are all legitimate hippodromes;
  works from trunk or billet alike (identify resolves either).
  Nothing ever lands in a hippodrome — the vanilla premise and the no-clone-local-state ruling both forbid it,
  and no gitignore is needed anywhere because the infield is not a repo.
  The jjy_ family charter widens from infield-resident repos to ALL JJ-owned infield residents,
  giving the yard-side/wire-side legibility split: jjy_ names what the operator touches on disk, jjx_ names the MCP wire.

## Canon workup

How the aspirant sheaves become canon, and what gets minted on the way.

### Disposition doctrine

Aspirant content drains as the build proceeds, durable-first:
the binding destination commits before the aspirant source trims,
so authority migrates fact-by-fact and never gaps.
Each section has one of three fates:
contract content drains into binding sheaves;
gestalt and rationale drain into the cosmology sheaf;
decision records (naming ledgers, dated cinches, open-fork lists) retire to provenance.
The sheaves are asymmetric:
the studbook and farrier sheaves drain fully this heat;
the tackle sheaf drains only its MVP core and keeps an aspirant remainder;
the founding-and-cutover section stays aspirant deliberately — it is the conversion heat's sole plan home.

### Destination sheaves (genre in parentheses)

- Revision-control cosmology (cosmology): infield peer ring, hippodrome, why-decouple, the world model.
- Blotter (entity): the locked-store entity — lock-ref namespace, guidon, read posture, instances, bootstrap config.
- Journal ceremony and break sequence (routine): the shared write bracket, ONE home,
  cited by the blotter sheaf and every studbook-writing operation.
- Studbook (entity): tenant content — pedigree, scope at birth.
- Farrier (entity): the driver trait, op census as method contracts, error-kind taxonomy, the capability seam.
- Dispatch and billet (operation family): saddle, lunge, unsaddle, the entrance spine, the sweep,
  BUK meld consequences; launch-time provisioning folds in (lean 260705: fold, split only if it dominates the sheaf at authoring).
- Tackle MVP core: scope-to-discipline binding beside the pedigree.
- Typical-JJ-session chapbook — AUTHORED 260705, aspirant: `Tools/jjk/vov_veiled/JJS-aspirant-chapbook-session.adoc`,
  mounted in the codex's Aspirant Nucleations with the first two-token cartouche.
  Its pending-citation register is the canon convergence gauge:
  each destination sheaf that lands hardens the tale's flagged beats to citations;
  the tale is expected to reach zero pending flags by heat end.
  Unhappy-path sibling tales (two-station contention, stale-lock break, stale-trunk resync) are named there and deferred.

Amendment streams, not new sheaves:
officium re-gestalt (exchange relocation, identify at open);
pace-identity re-gestalt (reprieve-gated per the standing cinch);
the interior saddle-routine rename freeing the saddle word.

### Mint roster

Catalogued at infusion (candidate-minted 260704):
studbook, pedigree, hippodrome, infield, counterfoil, guidon, farrier, saddle, unsaddle, lunge, billet, tackle,
and the jjy_ dirname family.
Elected 260705 under grep gate: blotter; chapbook (landed in AXLA);
the jjy_ widening carrying the launcher names jjy_saddle and jjy_lunge.
CENSUS DISCHARGED (260705): the candidate battery re-ran clean;
billet settled by eviction — the window-overlay quoin yielded, re-worded jjdxw_hatchment
(both held alternates were dead: livery foreign-minted in RBK, stall trodden);
farrier ops re-minted comb/lodge/glean/consign;
lock trio settled as guidon verbs stake/pluck/sight;
freshen resolved — refit names the session-facing resync operation, enfold the bare merge primitive,
the VOK collision banked in the farrier ledger;
the sweep door is muck;
weigh advance retained as the boring-register proof-word;
the RBSGS pedigree stray reworded to provenance,
plus a JJK-README ghost excision clearing studbook's only foreign hits.
Allocations landed:
refs/jjv/* lock-ref namespace (rationale in the state-repo ledger);
JJS0 legend reserves jjdb_/jjdd_/jjdf_ for the store/dispatch/driver vocabularies;
invariants classified — rivets never-force / no-worktree-paths / durable-first,
premises trunk-never-pushed / reads-take-no-lock / sole-door;
exposure tiers recorded — the six dispatch verbs ashlar, toothing-tier centrality,
broadside registration owed at infusion.
Saddle/unsaddle ratified with the gate OPEN by contingency until the interior rename frees the word.
Elections, alternates, and rationales live in the two sheaf ledgers; this roster is the summary.

### Deltas to bank into the aspirant sheaves at drain time

- Farrier ledger: DONE 260705 — the freshen collision recorded and resolved (refit/enfold);
  spine and doctrine wording re-cinched to glean-never-merge per the trunk-resync cinch.
- Billet definition: re-anchor on the partition axis (a root with a lifecycle) —
  worktree is the plain-git implementation; a full clone is the fallback for kinds without worktrees.
- The sweep supersedes the reaped-at-next-saddle line;
  the identify census row sheds its plain-git-only vocabulary.
- Studbook sheaf naming ledger: widen the jjy_ entry from infield-resident repos
  to all JJ-owned infield residents (the launcher trampolines join the family).
- Mews sheaf (sibling heat ₣Ba): DONE 260705 — the graduation fork closed in the sheaf itself;
  the fleet store is a jjy_ peer repo manipulated by the jj app, the blotter's anticipated second instance; no separate kit.

## Held for discussion

Settle before the affected change, not before the heat; each item names its settling pace:

- Identify contract shape — the kind-invariant core of its return;
  calcifying: settles at its design pace, before any farrier trait code.
- Billet dirname convention — lean on record (dirname = the §F branch name verbatim);
  adopted at dispatch-sheaf authoring.
- Wire-vs-display coronet form (gazette halter typing depends on it) —
  settles at the pace-identity design pace, coupled to the insignia sub-mint.

## Done when

The new surface is complete and inert behind its enablement seam under the frozen, still-authoritative old path,
and the scratch-studbook rehearsal checklist passes:
two-station lock contention, stale-lock break, atomic-push lease failure, dispatch round-trip.