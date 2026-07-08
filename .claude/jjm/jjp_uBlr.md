## Shape

The first implementation heat of the JJ revision-control MVP:
the studbook (regional record repo), the farrier (revision-control driver), and the dispatch layer,
built as one binary with parallel code paths under current JJ —
the in-repo gallops stays authoritative until the cutover ceremony, which is NOT this heat.

Authority is the three aspirant sheaves; dated cinches (260704) are settled and not re-litigated:

- `Tools/jjk/vov_veiled/JJSAS-state-repo.adoc` — studbook, journal ceremony, counterfoil, guidon, pace-identity re-gestalt, founding-and-cutover.
- `Tools/jjk/vov_veiled/JJSAF-farrier.adoc` — driver op census under the vocabulary-Palisade naming rule, lock guard, saddle/lunge dispatch, BUK meld.
- `Tools/jjk/vov_veiled/JJSAT-tackle.adoc` — MVP tackle only; scope layering is post-MVP supposition, do not act on it.

The 260630 revision-control memo and 260702 collision memo are provenance, never authority.
A canon-workup groom (260705) added the specification architecture below;
its AXLA half is landed canon (sheaf genre axis, two-token axvd_sheaf grammar, chapbook marker family).

The roster is ordered judgment-first:
mint, design, and sheaf-authoring paces front-load for frontier-tier sessions;
the implementation tail runs at work tier.

## Cinched

- One heat at a time: the conversion heat is cut only after this heat lands,
  from the state-repo sheaf's Founding-and-cutover section, refreshed by build learnings.
- Pace-identity re-gestalt relocated (260708): the implementation moved to the
  isolated-schema-changes heat (₣Bc) as its own reprieve-gated pace — this heat does not depend on it.
  The billet layer needs only the git-ref- and dirname-safe id shape,
  which grandfathering preserves verbatim, and the scratch rehearsal exercises no transfer or restring.
  The 260706 pace-id-shape, wire-vs-display, and halter-typing cinches travel as that pace's contract
  (it banks them into the state-repo sheaf, closing its open fork);
  they remain recorded below as dated design record.
  The insignia cinches stay here — this heat's blotter stores consume them;
  they merely shared the design pace.
  The conversion heat's entry gate widens: it requires the relocated re-gestalt landed and converged
  in addition to this heat's rehearsal pass — live billet branches must not rot on transfer.
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
- Identify contract (260706): the kind-invariant core of the farrier's identify is settled —
  studbook-blind, network-silent, lock-free, the first link of the derivation chain,
  taking an explicit probe path (the door captures cwd once; identify itself honors the census no-cwd rule)
  and answering claim-or-decline: identify doubles as ground detection —
  the spine probes the kind roster, the claim names the kind,
  and the spine cross-checks the claiming kind against the pedigree's recorded kind downstream
  (mismatch is a named rejection: record/ground drift).
  A claimed tree resolves four things:
  root (station-local absolute path, transient — never journaled, per the no-worktree-paths rivet;
  the identity dirname derives as its basename),
  canonical upstream key (the pedigree key; the kind owns canonicalization,
  one key per tree — a constellation kind keys on its own upstream),
  seat (primary or partition; a partition seat carries its primary's root,
  with linkage mechanics the kind's own: worktree gitdir for plain git,
  a planted station-local marker for full-clone fallbacks),
  and line of work (branch or detached, one seat-grain designation even for constellations).
  Total for a claimed tree: detached, mid-merge, or dirty states report faithfully, never reject;
  the sole failure is foreign ground (nothing claims), and all refusal policy lives in consumers.
  Deliberately excluded from the return:
  position — the member→SHA manifest shape has one home, the counterfoil op;
  trunk-ness — pedigree-relative, classified downstream, so the seat says primary, never trunk;
  dirt and staleness — comb's and sync_state's.
- Wire-vs-display coronet form (260706): the bare immutable pace id is the identity in every machine context —
  gallops keys, MCP params, billet branch names, git refs;
  heat qualification is emission-only — glyph + current heat + interpunct + immutable id —
  rendered from live affiliation at output time
  (transfer changes tomorrow's rendering, never the identity);
  input tolerates emitted forms: glyph and heat qualifier stripped on ingest, the bare id looked up;
  frozen emissions (commit messages, historical prose) legitimately carry the qualified form
  as record-of-the-moment, never as a live reference;
  the full-form never-abbreviate display discipline carries over, and firemark display is untouched.
- Pace-id shape (260706): five charset characters minted from the one global studbook seed under the lock;
  existing coronets grandfather verbatim — immutable-for-life includes the already-living —
  and the global seed founds above the highest grandfathered body,
  so collision with the legacy heat-embedded space is structurally impossible
  (one founding-time comparison, no reserved-set check);
  a grandfathered body renders like any other, its leading pair merely spelling the birth heat.
- Halter typing (260706): strip the glyph if present;
  a token containing the interpunct is a qualified coronet — the five-character tail resolves, the qualifier is ignored;
  otherwise type by length exactly as today (2 firemark, 5 coronet);
  the %-sentinel pensum is untouched, its heat embedding a frozen at-dispatch correlation fact, not a live affiliation.
- Revision insignia (260706): the two stores' commits carry decimal ordinal insignia, entirely emission-layer —
  catchword ₶ for the studbook (founding value 200000), varvel ₰ for the mews (founding value 10000);
  the ordinal is a denormalized label allocated under the store's lock and baked into the commit message,
  the SHA stays the identity, and the genesis commit takes the founding value (no zeroth special case);
  leading-digit founding pins width without zero-padding so lexicographic and numeric order agree
  (horizons: 800K studbook / 90K mews revisions before width growth),
  and a future store founds at a fresh leading-digit × width pair;
  bare decimal is charset-valid, so an ordinal never circulates glyphless —
  machine contexts carry the SHA, and any future ingestion convenience must require the glyph.
- AXLA insignia natures (260706): axd_ordinal minted as the third encoding nature
  (decimal, monotonic per store, a denormalized label aliasing an underlying identity, no hierarchical embedding),
  and axd_algebraic tightened to name its encoding precisely —
  the 64-character URL-safe base64 alphabet, RFC 4648 §5, positional values `A`=0 through `_`=63 —
  eliminating the loose fixed-alphabet wording a decimal ordinal would otherwise satisfy;
  the algebraic word retained, re-quoining deferred;
  the insignia-family ADT work in ₣BD picks the natures up code-side.
- Session-launch configuration (260707): designed whole; the former held-for-discussion fork is settled.
  The tier vocabulary is the vendor family words, ratified —
  a Palisade admission at one membrane, never a JJ mint (the farrier's git-word posture applied to model families);
  no tier categories are hardlined: a tier is just a name,
  and category judgment lives at each policy site as an explicit name-set, never as an attribute a family IS.
  Effort anchors transparently on Anthropic's own effort classification, likewise unminted;
  effort never rides the MCP wire, so it is designation-and-dispatch data — the guard reads tier alone.
  The bridle designation carries the pair: tier required, effort optional
  (folded into the sibling schema heat's bridle docket by operator ruling 260707).
  One roster table, baked into the engine as a constant (the blotter's engine-known posture):
  family name → launch model ID + valid effort set; no default columns;
  every kind of vendor drift lands as an edit to this one table, spook-on-surprise;
  fable stays a roster row no launch policy names until its pricing settles — opus is the working judgment tier.
  The launch primitive is stirrup (hearting per the enfold precedent):
  parameterized (billet, tier, opening prompt), pace-blind — pace-coupling lives in the caller
  (the dispatch doors first, per-beat breeze dispatch later, the same spine, no parallel launcher);
  stirrup is the table's one launch consumer — callers speak tier words, never model IDs,
  and an invalid (family, effort) pair refuses fair-facedly.
  The (tier, effort) choice has exactly two sources, split by door:
  lunge, having no pace, launches the judgment constant — opus/xhigh, one named cell;
  saddle launches the pace's designation exactly;
  saddle on an unbridled pace launches the same judgment constant —
  undesignated work is judgment work: such a session plans toward a bridle or simply finishes fast;
  a designation without effort launches with the effort knob omitted — the vendor default governs, JJ invents nothing.
  Guard permanence: every command registers its guard bucket explicitly, no default — absence fails loud;
  the docket-authoring gate is an explicit accept-set of family names beside the roster (fable + opus today);
  unknown families sit in no accept-set by construction — OPEN reads only, until a deliberate roster-and-set edit.

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
  LANDED 260705: `JJSVC-cosmology.adoc` (normative cosmology), mounted in JJS0's new Revision Control region;
  the `jjdw_` quoin category catalogues infield, hippodrome, and the jjy_ family (charter widened per the delta);
  the JJSV* file family allocated for the destination sheaves (V: revision-control);
  broadside registration for the two nouns deliberately deferred to cutover —
  the README speaks to consumers of the live surface, and the new surface is inert until then.
- Blotter (entity): the locked-store entity — lock-ref namespace, guidon, read posture, instances, bootstrap config.
  LANDED 260706: `JJSVB-blotter.adoc` (normative entity), mounted in JJS0's Revision Control region.
- Journal ceremony and break sequence (routine): the shared write bracket, ONE home,
  cited by the blotter sheaf and every studbook-writing operation.
  LANDED 260706: `JJSVJ-journal.adoc` (normative routine), mounted beside the blotter sheaf.
- Studbook (entity): tenant content — pedigree, scope at birth.
  LANDED 260706: `JJSVS-studbook.adoc` (normative entity), mounted in JJS0's Revision Control region;
  homes the studbook, pedigree, and counterfoil quoins and the no-worktree-paths rivet.
- Farrier (entity): the driver trait, op census as method contracts, error-kind taxonomy, the capability seam.
  LANDED 260706: `JJSVF-farrier.adoc` (normative entity), mounted in JJS0's Revision Control region;
  homes the `jjdf_` quoins (entity, three facets, identify, vocabulary Palisade)
  and the never-force rivet at the consign contract.
- Dispatch and billet (operation family): saddle, lunge, unsaddle, the entrance spine, the sweep,
  BUK meld consequences; launch-time provisioning folds in (lean 260705: fold, split only if it dominates the sheaf at authoring).
- Tackle MVP core: scope-to-discipline binding beside the pedigree.
- Typical-JJ-session chapbook — AUTHORED 260705, aspirant: `Tools/jjk/vov_veiled/JJSAC-chapbook-session.adoc`,
  mounted in the codex's Aspirant Nucleations with the first two-token cartouche.
  Its pending-citation register is the canon convergence gauge:
  each destination sheaf that lands hardens the tale's flagged beats to citations;
  the tale is expected to reach zero pending flags by heat end.
  Unhappy-path sibling tales (two-station contention, stale-lock break, stale-trunk resync) are named there and deferred.

Amendment streams, not new sheaves:
officium re-gestalt (exchange relocation, identify at open);
the interior saddle-routine rename freeing the saddle word.
The pace-identity re-gestalt amendment stream relocated with its implementation
to the isolated-schema-changes heat (₣Bc) per the 260708 cinch.

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
COSMOLOGY INFUSION (260705): hippodrome, infield, and the jjy_ family catalogued as `jjdw_` quoins
in `JJSVC-cosmology.adoc`; their ledger entries marked infused;
the chapbook's hippodrome/infield flags hardened to citations.
BLOTTER AND JOURNAL INFUSION (260706): blotter, guidon, journal, and break catalogued as `jjdb_` quoins
in `JJSVB-blotter.adoc`/`JJSVJ-journal.adoc`;
guidon and jjv ledger entries marked infused (interior shape settled: refs/jjv/guidon, flat, per-blotter);
lockless-reads and sole-door premises voiced; durable-first marked infused;
the chapbook's blotter lane and journal-ceremony interactions hardened to citations.
STUDBOOK INFUSION (260706): studbook, pedigree, and counterfoil catalogued as `jjdb_` quoins
in `JJSVS-studbook.adoc`; their ledger entries marked infused;
the no-worktree-paths rivet homed at the sheaf (tail mints at implementation);
the chapbook's studbook lane and counterfoil mentions hardened to citations.
FARRIER INFUSION (260706): farrier, the three facets (core/lock/billet), identify, and the
vocabulary Palisade catalogued as `jjdf_` quoins in `JJSVF-farrier.adoc`;
farrier, comb/lodge/glean/consign, stake/pluck/sight, and enfold ledger entries marked infused
(refit, muck, saddle/unsaddle, lunge, and billet stay aspirant with the dispatch family);
never-force marked infused (homed at the consign contract, tail mints at implementation);
the chapbook's farrier interactions and identify-at-open hardened to citations,
its open-staleness line re-worded to the softened no-verb-refuses cinch.
INSIGNIA SUB-MINT DISCHARGED (260706): the revision-insignia glyph space allocated whole at the pace-identity design pace —
catchword ₶ (U+20B6, livre tournois) for studbook revisions, varvel ₰ (U+20B0, pfennig) for mews revisions,
decimal-ordinal bodies under leading-digit founding;
tattoo deliberately banked unminted, freezemark rejected for the firemark rhyme, chestnut/hip/accession held as alternates;
AXLA gains axd_ordinal beside the tightened axd_algebraic.
SESSION-LAUNCH MINT DISCHARGED (260707): stirrup elected for the launch primitive at the entrance spine's end —
hearting per the enfold precedent, gate clean;
launch pre-ruled trodden, launcher BUK-bound, compound forms barred by operator ruling (leg-up and put-up died there);
spur held as alternate with a false-friend note;
gee released to deliberate fallow as the operator's trot-advance cry;
rationale ledgered in the farrier sheaf.
Elections, alternates, and rationales live in the two sheaf ledgers; this roster is the summary.

### Deltas to bank into the aspirant sheaves at drain time

- Farrier ledger: DONE 260705 — the freshen collision recorded and resolved (refit/enfold);
  spine and doctrine wording re-cinched to glean-never-merge per the trunk-resync cinch.
- Billet definition: DONE 260706 — re-anchored on the partition axis (a root with a lifecycle)
  in both the canon Two-axes section and the remaining aspirant billet bullet;
  worktree the plain-git implementation, full clone the fallback for kinds without worktrees.
- Sweep and identify rows: DONE 260706 — the muck-sweep cinch superseded the
  reaped-at-next-saddle line in the aspirant lifecycle doctrine;
  the census drained into the farrier sheaf with identify carrying the cinched contract
  (plain-git vocabulary and HEAD shed — position's one home is the counterfoil op),
  and the officium-open composition row re-worded to the four identify resolutions.
- Studbook sheaf naming ledger: DONE 260705 — the jjy_ entry marked infused with the widened
  charter (all JJ-owned infield residents), the charter itself canon in the cosmology sheaf.
- Mews sheaf (sibling heat ₣Ba): DONE 260705 — the graduation fork closed in the sheaf itself;
  the fleet store is a jjy_ peer repo manipulated by the jj app, the blotter's anticipated second instance; no separate kit.
- Pace-identity and store-shape sections: the 260706 design cinches settle the sheaf's
  wire-vs-display open fork and deferred insignia sub-mint.
  The pace-identity banking travels with the relocated ₣Bc re-gestalt pace (its implementing surface);
  the store-shape/insignia settlement still banks here at drain time.
- Session-tiers paragraph (farrier sheaf, dispatch layer): the session-launch cinch supersedes its wording —
  lunge and unbridled-saddle launch the opus/xhigh judgment constant, bridled-saddle consumes the designation,
  and the verb-carries-the-tier line narrows to the two-source choice function;
  the haiku orchestration line stays breeze-era; bank at drain time.

## Held for discussion

Settle before the affected change, not before the heat; each item names its settling pace:

- Billet dirname convention — lean on record (dirname = the §F branch name verbatim);
  adopted at dispatch-sheaf authoring.
- Stirruped-entry protocol seam — how the mount and groom protocol texts behave when a session
  arrives through the dispatch doors rather than a hand-opened chat:
  the opening prompt carries the engagement verb, so target election and its gazette dance shift form;
  a session on a sub-frontier-designated pace follows the designee protocol landing with the bridle work
  (orient, work, record, landing, never wrap), not the full mount ceremony;
  a judgment session keeps the ceremony;
  settles at dispatch-sheaf authoring alongside the kit-core protocol-text updates.

## Done when

The new surface is complete and inert behind its enablement seam under the frozen, still-authoritative old path,
and the scratch-studbook rehearsal checklist passes:
two-station lock contention, stale-lock break, atomic-push lease failure, dispatch round-trip.