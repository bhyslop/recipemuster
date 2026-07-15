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
  pace states snapshot through the journal sheaf's read bracket — lock, advance, read, release —
  then reaping proceeds lock-free.
  Re-grounded 260714 (the sweep-lock ruling pace): the snapshot is an acting read deciding from
  the store's truth; the monotonicity justification retired under the layer-independence rule
  (JJSVB Read posture) — store correctness never leans on pace-state reasoning.
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
- Unsaddle removed; muck folds into the spine (260708): the stiles collapse to two — saddle and lunge.
  muck ceases to be a door and becomes the billet-clearing behavior itself:
  the leading step of the shared entrance spine, run implicitly on both stiles, never directly invoked;
  its concept-home moves from its own operation to a step within the spine.
  It stays plan-then-confirm but is silent when its reap set (closed-pace × past-retention) is empty —
  forced by the entrance-must-be-pleasant invariant.
  unsaddle's refuse-if-dirty was redundant: muck already owns the dirty guard
  (refuse-with-advice by default, auto-commit-push behind explicit confirm), so the door vanishes whole,
  the word released unminted (never infused; retirement ledgered in the farrier sheaf).
  Deliberately sacrificed for MVP: immediate reap of a not-yet-closed billet —
  such a billet waits for pace-close plus retention, and the next stile crossing takes it.
  Refines the Billet sweep cinch's at-dispatch-time framing (the mechanics stand; the door does not)
  and re-cuts the census ashlar roster — saddle/lunge/break/refit typed, muck ashlar by confirm-output alone.
  The Dispatch shell surface cinch already names only the two jjy_ trampolines and stands unchanged.
- Sheaf register (260708, operator ruling): sheaves — aspirant and canon alike — carry the latest
  best concept only, never the steps that led there;
  a date-stamped process trail in a sheaf is a skidmark.
  Dated records belong to the paddock (this convention) and to git;
  a resolved open fork simply leaves the mulling list.
- Dispatch-sheaf authoring decisions (260708, settled at authoring):
  billet dirnames are jjqb_{coronet} for pace billets and jjqb_{firemark} for groom billets
  (the yard-split reconciliation superseded the earlier branch-name-verbatim lean;
  branch = the bare coronet, dirname = the jjqb_ signet, typed by length);
  groom billets reap on retention alone under the same liveness join (they carry nothing durable);
  the break-all door is catalogued with the dispatch family, citing the journal break sequence per lock;
  the trampoline installer verb stays unnamed — a boring hearting name mints at implementation;
  saddle is infused with its grep gate still OPEN by contingency, completed by the interior-remint pace;
  broadside registration for the dispatch ashlars defers to cutover alongside the cosmology nouns.
- Registered-identity durability (260709): the registered external body of work is transport-independent identity —
  it may arrive as a git clone today and a tarball someday under another driver kind,
  so the pedigree keys on a minted immutable identity,
  and the origin URL is a kind-owned mutable address attribute, never the key.
  Identify is untouched (it still derives the kind-canonical key from ground, studbook-blind);
  the spine's pedigree lookup gains one indirection: derived key → registered identity → pedigree.
  Supersedes only the "the pedigree key" clause of the identify cinch.
  The word ELECTED 260711: *sire*, under a re-run clean grep gate,
  and the wording sweep landed the same act —
  JJS0's pedigree glossary row and JJSVC/JJSVS/JJSVF/JJSVD re-worded
  carrying the key-indirection re-cut (derived key → sire → pedigree)
  and the unrecorded-sire rejection rename,
  JJSAT's per-upstream and governed-body placeholders swept
  with its discharged reconciliation bullet removed,
  JJSAS re-worded with the election ledgered.
  Deliberately untouched by judgment:
  identify's ground-level upstream-key vocabulary (identify is sire-blind),
  JJSAB's directional "flows upstream", and RB's Pale mint;
  the journal/blotter "work repo" phrasing was ruled outside the recorded scope.
  Plain prose, no quoin — cataloguing a sire quoin is a deliberate later act.
- Footprint posture (260709): work products (commits, branches — the code changes themselves)
  are the registered body's property, delivered in the form its owner tolerates;
  knowledge products (records, counterfoils, chat captures, guides, grimoire) are JJ's
  and never land there.
  The pedigree's remotes-posture parameter generalizes to a per-pedigree footprint posture —
  default: personal branches;
  constrained: one personal branch carrying a prepared patch series (format-patch lineage),
  spoken by a future driver kind.
- Engagement inversion (260709): a pedigree records a bounded engagement —
  JJ arrives, delivers traction on a body of work, and evaporates when done;
  the bodies of work outlive JJ.
  Entries conclude by status, never deletion (the studbook keeps history);
  a disengagement ceremony (footprint retirement per the posture) is post-MVP.
- Chat-capture parking (260709): the recurring chat-capture work (formerly a fast-tweaks pace, now parked)
  was overtaken by the footprint-posture cinch —
  chat captures are JJ knowledge products and never land in the work repo;
  its redo (store home: the studbook or a blotter instance, capture riding the post-MVP ceremony surface)
  is reconsidered once this heat lands.
  Standing obligation meanwhile: the host-separated rough-backfill chat store in the work repo
  is the sole copy of off-machine chat history —
  it must not be flattened, migrated, or discarded before the successor design lands.
- Tackle model urged (260711): the pace's three done-when items settled and banked into JJSAT.
  Trappings dissolved into typed governance members
  (guides / conduct / specs / balances / check_build|test|lint) —
  each with its own consumer and its own value type (documents vs commands);
  growth is member-wise, never kinds threaded through a generic list.
  Blaze registries: guides and codices are ₿-sigiled role-word entries
  (₿bash-guide, never a document acronym) — declared variables, not identities;
  one blaze namespace per table, and late binding keeps gaits portable across governed
  bodies and makes a crossbred row bind correctly to the receiving table.
  Names-not-identities: tackle rows key on plain spoken names, resolution is files-first,
  and nothing durable references a tackle — no minted insignia, no seed, no glyph
  (the identity glyph election retired from the settling register);
  identity re-enters only with the first durable referencer
  (swingletree normalization, per-heat overlays), for that channel only.
  remuda and silks released alongside trappings; claims is the fileset member.
  Same-session re-elections: swingletree supersedes hame (the crossbar, not the anchor
  hardware), and ambit releases to plain bounds (jjotrn_bounds) with in-bounds /
  out-of-bounds verdicts.
  The bounds seat in the tackle table, never the pedigree.
  Blaze-usage moratorium: no docket or operative prose references a ₿ word until the
  table lands and the glyph-carriage amendment legalizes the sigil.
  Corpus generalized to named corpora: reserved jjotrn_corpora registry, plural by
  design, plain-name keys; entry shape settles with the central-corpus fork.
  JJ is the tackle table's sole author (verbs under the journal ceremony, gallops posture);
  the matricula reads and lints, never writes.
  Document members are declared flattened edges; ACG is the first migrant when
  swingletrees land.
  The AXLA glyph-carriage amendment widens with a blaze-sigil companion clause;
  the corpus member is skip-when-absent (absent = the governed body's own tree),
  its tokens electing with the central-corpus-home fork;
  JJSAT joins the sire wording-sweep scope.
- Minted-mark re-cut (260711): at the glyph-carriage edit the insignia axis re-split
  on referential standing —
  axd_insignia narrowed to the constitutive identity mark
  (the mint creates the identity; mint natures axd_seeded | axd_temporal),
  axd_ordinal re-seated as the sibling annotative order-label dimension
  (a denormalized alias of a pre-existing, often foreign-minted identity — never truth),
  and axd_algebraic renamed axd_seeded — the mint-discipline contrast carries the nature axis,
  with radix, width policy, and boundedness demoted to consequences, never the distinction.
  The surface-keyed carriage law homes once at the AXLA section intro
  (retitled Minted-Mark Dimensions):
  type sentinel mandatory in operator-facing output and in project-authored structured wires
  (self-typing keys; a wrong-type value fails at the parse boundary),
  forbidden on foreign-traversed surfaces, tolerant on input,
  with the role-word companion clause covering ₿-sigiled blaze keys.
  Consequences: the gallops store's sigiled JSON keys are doctrine-compliant
  (the old forbidden-in-wire-keys clause was contradicted by standing practice);
  the wire-vs-display cinch's machine-context enumeration (260706) is superseded
  for project-authored JSON stores — the identity stays the bare body on every surface,
  carriage of the sentinel differs by surface, and git refs / paths / branch names stay bare;
  the ₣Bc gallops-key glyph-strip pace, which worked against the new law, was dispositioned by abandonment;
  JJS0's jjdt_insignia carriage sentence and the jjrf_favor.rs comment sweep were verified
  already conformant at the convergence audit (260713) —
  discharged in the re-cut act itself, nothing owed to JJSAT.
  JJS0 swept for the rename in the same act (Seeded section title, attr refs, annotations).
- Convergence audit (260713): the chapbook's two missing cast homes minted as jjdw_ quoins
  in the cosmology sheaf's new two-parties section —
  jjdw_operator catalogued from the corpus's standing vocabulary,
  jjdw_engine blessed as the living interior word by operator ruling
  (the Docker/Podman-engine foreign-proper-noun collision deliberate and tolerated,
  ledgered at the definition site);
  the chapbook's pending-citation register reads zero.
  Aspirant genre rule applied as the docket pre-ratified — genre names what the sheaf now holds:
  JJSAS/JJSAF/JJSAT/JJSAB axd_roadmap (post-drain and banked remainders),
  JJSAM axd_entity (a live noun mulling),
  lintels and JJS0 cartouches moved in one act.
  The revision-insignia cinch above is now canon at JJSVB
  (Revision ordinals; jjdb_catchword/jjdb_varvel catalogued, JJSAS re-pointed) —
  but its code remains unbuilt: the blotter commits plain messages today,
  so the ordinal bake (allocate under lock at journal time, founding value at jjdb_found)
  needs its own small pace before the rehearsal gate.
- Founded studbook is disposable scaffolding (260713, operator ruling at the founding pace):
  the real jjqs_studbook was founded against its GitHub remote with a scratch empty gallops —
  rehearsal substrate only, and it may absorb rehearsal noise freely;
  the operator's working gallops does not migrate this heat and current JJ stays untouched
  (both enablement seams pinned false).
  The bare remote may be deleted and recreated by the operator for a clean slate at any time —
  the founding ceremony reruns from nothing, so nothing founded today is load-bearing.
  The conversion heat performs its own founding import per JJSAS Founding-and-cutover
  and decides then whether to reuse the standing instance additively or start from a
  recreated-clean remote; today's scratch founding forecloses neither.
- Pedigree keys on the address, not a minted sire id (260713, operator ruling at the rehearsal pace):
  `jjop_sire` dropped from the pedigree wire — it had no consumer
  (lookup scans `jjop_addresses` for the derived key directly),
  and the registered-identity indirection the 260709 cinch describes was never built.
  Seeding it would write a meaningless value into durable committed studbook records,
  where the first real sire becomes indistinguishable from the junk seeded before it.
  Supersedes only the key clause of the 260709 registered-identity-durability cinch;
  the scenario that cinch names — a body of work that moves hosts, so its address is mutable — stands,
  and the field lands the day that scenario is real,
  as an optional serde-default field (a non-breaking add, unlike the removal it would otherwise cost).
  Until then the single operator's discipline over the addresses serves.
- Founding seeds the pedigrees file (260713, found at the rehearsal pace):
  `jjrds_plan`'s first studbook read is `pedigrees.json`,
  so a station without it cannot dispatch (the refusal is the fair-faced unrecorded-sire rejection);
  the ₢BrAAU ceremony seeded only `gallops.json`, one file short of what JJSVS Founding-and-cutover describes.
  The gap is in the seed the ceremony was handed, never in the ceremony itself —
  a founding writes whatever seed it is given.

## Rehearsal findings (260713, ₢BrAAW against the real remote)

Evidence for the conversion heat's entry gate. Two stations are two clones of the real studbook remote,
so GitHub itself arbitrates every lock — the in-process guard registry cannot be what refuses.

- SUBSTRATE HOLDS. Real GitHub accepts the whole protocol, which no scratch bare remote could vouch for:
  the custom `refs/jjv/guidon` namespace (stake pushes a blob there, sight reads it back
  via ls-remote + fetch + cat-file), and the atomic two-ref consign under `--force-with-lease`.
  The ordinal bake works on the real store; the genesis lacking its mark costs nothing downstream,
  since the ordinal derives from history length rather than from the genesis value.
- THREE CHECKLIST ITEMS PASS: two-station contention (the second station is refused LockHeld
  off the remote's own compare-and-swap, and its mutate never runs), stale-lock break
  (the second station reads the crashed holder's guidon, breaks it, and completes its own ceremony),
  and atomic-push lease failure (a lock broken mid-ceremony refuses the consign with LockBroken
  and lands NOTHING on the remote — proven twice, once by accident when a harness bug
  panicked a ceremony mid-flight and the remote tip did not move).
- DEFECT FOUND, and it is why ₢BrAAf exists: a lease-refused commit strands in the local clone,
  nothing reconciles it, and the station's next authorized ceremony pushes it onto the shared store.
  Advance fast-forwards toward the remote tip, so a clone merely *ahead* of the remote is left alone;
  lodge then stacks on the stranded commit and consign carries both, a clean fast-forward from the
  remote's side with a lease that passes because the station holds the lock *this* time.
  Content the lock refused thus arrives under a later, unrelated lock, with nothing recording the refusal.
  Observed on the real remote, not reasoned about.
  Second face, same cause: had another station written in between, the clone is both ahead and behind,
  the fast-forward fails, and the station wedges with a Diverged it has no verb to recover from.
  Which face you get is pure timing.
- NO OPERATOR DOOR TO THE BREAK. A crashed writer leaves a stale lock on the shared remote
  and the break sequence is its only cure, but nothing on the operator's surface calls that sequence.
  We needed it for real within the hour, and had to reach past the surface to get it.
  A recovery tool stands in; a door is owed before the store carries live records.

## Rehearsal findings (260714, ₢BrAAW re-run against the real remote)

The re-run after ₢BrAAf, ₢BrAAk, and the two harness fixes landed.
All four checklist items now pass against the real GitHub studbook.

- THE THREE LOCK ITEMS PASS CLEAN, and the JJr_b52 regression is proven where the defect was found:
  the aftermath probe pushed exactly one commit and the lock-refused commit did not ride along —
  advance equalizes, so the stranded content is retrenched, on the real remote, not a fixture.
- TWO HARNESS DEFECTS the re-run's own first attempt surfaced, both fixed before it could report:
  the harness was not re-runnable against a store that accumulates —
  fixed content wrote nothing once advance equalized to a tip already holding it,
  so every rehearsal write now carries a per-run mark and the aftermath assertion is range-scoped;
  and the unclassified-git-failure panic rendered its detail from stderr alone,
  reporting an empty reason for a `commit`-with-nothing-staged that explains itself on stdout —
  the panic now says exit code, stderr, and stdout at every unclassified site.
- DISPATCH ROUND-TRIP PASSES — the fourth item, exercised for the first time, by hand through the real stiles.
  Both doors resolved their plan against the real studbook (identify, hippodrome, infield,
  pedigree read off the store by the canonicalized upstream key, target typing, tier),
  refused all three fair-faced ways (a coronet to lunge, a closed pace to saddle, foreign ground),
  and each launched a real session inside its own billet with its own MCP config,
  which opened an officium and worked — a groom in `jjqb_Br` detached, a mount in `jjqb_BrAAW` on branch.
- FINDING A, the door's report reaches the operator only after the launched session exits.
  The door accumulates its billet/tier/prompt/staleness report and prints it after the child's status returns,
  so every line it has to say lands below the session it was meant to precede.
  Contained timing defect; its fix pace is cut into this heat ahead of the conversion-heat cut.
- FINDING B, and it is evidence for the switchover, not a defect in the door.
  A billet anchors at trunk's remote counterpart by design (the no-exfiltration posture:
  a billet never inherits the hippodrome's private local work).
  That is correct for the deployed single-hippodrome model, but it presumes origin carries the finished work.
  In the current multi-clone dev setup gamma's local trunk ran unpushed-ahead of origin by a whole heat's tail,
  so both billets were born from origin missing every recent pace,
  and the dispatched sessions read a gallops and paddock that predated ₢BrAAk —
  the groom session trusted it and reported the break door as not-yet-existing,
  the mount session recovered only by exploring the shared object store with `git log --all`.
  The staleness detector is structurally blind to this: it compares billet-against-origin,
  never origin-against-the-operator's-local-trunk, so it can only warn about drift acquired after birth.
  This is the concrete cost the single-hippodrome cutover retires, and it belongs to the conversion heat's shape.

  CROSS-MACHINE COHERENCE CLUSTER (traced this session; the seams the switchover owes).
  The operator works from more than one machine (macmini, cerebro), so the same sire is cloned
  as two hippodromes — the studbook keeps the RECORD coherent (locked shared store, proven by
  the rehearsal's two-station tests), but the CODE half is the operator's by the
  trunk-never-pushed-by-JJ premise, and the seams below are what would make it seamless.
  Two are conversion-scope (they ride the cutover the conversion heat performs):
  - RECORD-READ REPOINT (verified built, awaiting the flip): the gallops-over-studbook surface
    (`jjdb_gallops_journal_load`, reads from the studbook clone) is complete and tested behind
    `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED`; the flip is the cutover's act (JJSVS Founding-and-cutover).
    TWO readers must repoint, not one: every `jjx_*` command's read AND the dispatch spine's own
    target-resolution read (`jjrds_plan`, which today loads `hippodrome/.claude/jjm/jjg_gallops.json`
    while the pedigree it reads beside it already comes from the studbook).
    Currency-at-read — when a session gleans its local studbook clone — is the cutover's to settle.
  - WARN-AT-OPEN, owed to match the chapbook: JJSAC Act II already renders the beat
    ("on a stale trunk the open leads with the staleness warning and names refit"), but `jjx_open`
    has no staleness check — the warn fires only from the dispatch board today, so a session
    RE-ENTERING an existing billet is never alerted. The probe is cheap (billet behind
    `origin/<trunk>`, a local ancestry check after a glean) and rides identify-at-open.
    Detect-and-alert, never auto-run (the advice-not-automatic cinch). The full refit sequence
    is the already-named-deferred stale-trunk-resync sibling tale in JJSAC.
  Two are post-conversion (footprint-delivery, and NOT retired by the cutover):
  - REMOTE-AWARE SEAT: because branch = coronet and dirname = `jjqb_<coronet>` are both derived,
    the billet path is derivable per-station (regional coronet + station-local infield root) and
    never needs recording — so the no-worktree-paths rivet is respected AND self-justified by
    derivability (a why-clause JJSVS could adopt). The seam: extend the existence check from local
    `refs/heads/<coronet>` to also consult `origin/<coronet>` after a glean, and seat FROM the
    remote branch when found, so a second machine adopts the first's pushed work instead of forking.
    Refit already pushes the coronet branch (its lease-free consign), so the branch to adopt exists.
  - DELIVERY-TARGET FIELD, only if base ever differs from target: `jjop_trunk` already names the
    per-sire trunk (built) and doubles as the stable integration base — choose it stable and refit
    stays rare. A separate delivery-target earns a field only when born-from differs from
    delivered-to; until then it is footprint-posture supposition, not owed.
  Also owed, small: the refit ENGINE is built and tested but has no operator DOOR
  (the advice names refit; nothing invokes `jjrrf_refit`), and that module's own comment
  ("the dispatch spine's consumers, which does not exist yet") is now stale — the spine exists.
- FINDING C, minor: each billet is a fresh project directory,
  so Claude Code asks for MCP-server trust on first contact into every new billet.
  This is Claude Code's own first-contact behavior at the harness Palisade, not JJ's to legislate;
  recorded so the cutover surface anticipates the prompt rather than treating it as a fault.

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
- Dispatch and billet (operation family): the two doors, the shared entrance spine with muck leading,
  refit, the break door, the billet entity, BUK meld, launch-time provisioning, session launch.
  LANDED 260708: `JJSVD-dispatch.adoc` (executory operation family), mounted in JJS0's Revision Control region;
  homes the `jjdd_` quoins (saddle, lunge, billet, entrance spine, muck, refit, break door);
  the farrier and cosmology forward-reference notes retired, the journal break-door pointer hardened,
  and the studbook sheaf's refit/billet mentions converted to citations.
- Tackle MVP core (entity): the scope-to-discipline binding beside the pedigree —
  the tackle node, its claims, its bounds and three verdicts, the blaze registries, the schema,
  and saddle-time provisioning.
  LANDED 260711: `JJSVT-tackle.adoc` (executory entity), mounted last in JJS0's Revision Control
  region so every citation reads backward;
  homes the `jjdl_` quoins (tackle, claims, bounds, blaze) and the `jjdk_no_catch_all` premise,
  and allocates the `jjo` JSON-sprue container with `jjot` for the tackle wire.
  Two laws classified as rivets with tails deferred to implementation
  (positive-at-domain-grain; derived-roster-never-in-the-studbook) —
  following the landed sibling practice over JJSAT's mint-at-infusion phrasing, which it supersedes.
  The drain closed the family's tackle forward references:
  JJSVS's forward-reference NOTE retired and its pedigree bullet widened to filesets-and-interrelations
  (with the no-duplicate-bounds rule stated), JJSVD's binding reference hardened to a citation,
  JJSVC's sheaf roster and derivation chain re-worded, JJSAF's drained-marker updated,
  and VOSMM re-pointed at canon with its stale until-representation-settles clause corrected.
  JJSAT keeps the aspirant remainder: the relation layer (swingletrees, the association graph,
  the consumer horizon), the banked maximal layer, the substrate-split interface direction,
  the open forks, the settling register, and the still-owed reconciliation nudges.
- Typical-JJ-session chapbook — AUTHORED 260705, aspirant: `Tools/jjk/vov_veiled/JJSAC-chapbook-session.adoc`,
  mounted in the codex's Aspirant Nucleations with the first two-token cartouche.
  Its pending-citation register is the canon convergence gauge:
  each destination sheaf that lands hardens the tale's flagged beats to citations;
  the tale is expected to reach zero pending flags by heat end.
  260708: the dispatch flags hardened (Act I, teardown, cast lanes, resync);
  the register now holds only the party and engine entity quoins.
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
UNSADDLE RETIREMENT (260708): unsaddle removed before infusion — the reap folded into the
entrance spine as muck's own behavior, leaving no door to name; the word releases unminted.
muck re-cut from sweep door to the spine's leading step — never typed, ashlar by confirm-output alone;
the ashlar roster now reads saddle/lunge/break/refit typed plus muck by output
(superseding the census line's six-verb count);
retirement and re-cut ledgered in the farrier sheaf.
DISPATCH INFUSION (260708): saddle, lunge, billet, muck, refit, the entrance spine, and the break door
catalogued as `jjdd_` quoins in `JJSVD-dispatch.adoc`;
their ledger entries marked infused (saddle with its grep gate still open by contingency —
completion rides the interior-remint pace);
stirrup stays hearting;
broadside registration for the dispatch ashlars deferred to cutover with the cosmology nouns.
SIRE ELECTION (260711): sire elected for the registered external body of work
under a re-run clean grep gate (held since 260709, no alternates banked);
plain prose, no quoin minted;
rationale ledgered in the state-repo sheaf's naming ledger.
MINTED-MARK RE-CUT (260711): axd_algebraic renamed axd_seeded and axd_ordinal re-seated
as the sibling annotative mark at the glyph-carriage landing;
the carriage law homed at the AXLA section intro (Minted-Mark Dimensions) with the
role-word companion clause; rationale in the minted-mark cinch and the AXLA edit itself.
TACKLE INFUSION (260711): tackle, claims, bounds, and blaze catalogued as `jjdl_` quoins in
`JJSVT-tackle.adoc` — the `jjdl_` category minted because `jjdt_` is spent on Types
(`a`/`c`/`k` likewise spent, so `l` is the only free letter inside the word);
the `jjdk_no_catch_all` premise voiced (totality by humble first-class rows, never a bucket);
the `jjo` JSON-sprue container allocated with `jjot` for the tackle wire.
The retirements ledger in JJSAT's vocabulary table so they are not re-tried
(trappings, remuda, ambit, hame, silks-as-handle; palisade and pale rejected with rationale);
swingletree stands elected but deferred whole with the relation layer.
Two obligations discharged from JJSAT's list: the JJSVS pedigree widening and the
rivet/premise voicing.
The aspirant-lintel upgrade proves family-wide, not tackle-local — every aspirant sheaf but
the chapbook carries the legacy one-token form — so it re-banks as one sweep, with this
sheaf's own genre re-opened by the drain (it is a deferred-remainder sheaf now, not an entity).
OFFICIUM-OPEN INFUSION (260712): the officium-open toothing drained to canon with no word minted —
the composition is the point, not new vocabulary.
Billet resolution homes at `JJSVF-farrier.adoc` ("Toothing: officium open"):
identify at the officium's captured cwd supplies the seat,
station and session are the two members the tree cannot supply,
and the composition adds no rejection kind of its own — the farrier taxonomy stays tree-shaped.
No hippodrome-or-billet enum minted: the seat is carried as-is,
and reading `Primary` as the hippodrome and `Partition` as a billet is a premise
the dispatch doors underwrite (the sole-entrance law — a hand-launched session in a hand-made
partition is a non-JJ session and opens no officium),
which the deferred billet-validation composition will enforce;
canon states it as that premise, never as a fact identify asserts.
Exchange/record co-location homes at `JJSVS-studbook.adoc` ("Officium exchange and record").
The record's entry rule binds NOW, before any record write exists:
graduation of the record half is only a gitignore-line change, so it forces no schema review of its own —
the hippodrome-or-billet member enters as the seat's role reading alone,
and a `Partition`'s primary root, a station-local worktree path, never serializes
(the no-worktree-paths rivet already covers it; its tail still mints at the record write).
The station takes no stand-in name: a station that reports no name opens no officium,
since two unnamed stations would record the same station member
and collapse the distinction it exists to draw, durably, in a record slated for committed history;
the refusal is officium-open's own, wired at the conversion heat.
Landed inert behind `JJRM_OFFICIUM_STUDBOOK_ENABLED` with a guard test pinning it false,
mirroring the blotter seam.
Deliberately not drained, and still aspirant:
the `sync_state`-from-last-fetch staleness line,
and validating the resolved billet against the work being mounted
(that composition wants the pedigree/sire infrastructure this heat does not build).
Elections, alternates, and rationales live in the sheaf ledgers; this roster is the summary.

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
  the store-shape/insignia half is DONE 260713 —
  drained to the blotter sheaf's Revision ordinals section at the convergence audit,
  JJSAS re-pointed and its resolved fork removed.
- Session-tiers paragraph (farrier sheaf, dispatch layer): DONE 260708 — drained into the
  dispatch sheaf's Session launch section per the session-launch cinch;
  the haiku orchestration line stays breeze-era.
- Unsaddle removal: DONE 260708 — banked into the farrier sheaf (toothing table, dispatch-layer
  section, lifecycle doctrine, naming ledger, exposure tiers), the chapbook's Act I and teardown
  interactions, and the state-repo ledger pointer, per the 260708 cinch;
  the dispatch-layer content then drained whole into the dispatch sheaf the same day.

## Held for discussion

Settle before the affected change, not before the heat; each item names its settling pace.
Nothing currently held — the billet dirname convention and the stirruped-entry protocol seam
both settled at dispatch-sheaf authoring (260708);
the kit-core protocol-text updates the seam implies ride the implementation tail.

## Done when

The new surface is complete and inert behind its enablement seam under the frozen, still-authoritative old path,
and the scratch-studbook rehearsal checklist passes:
two-station lock contention, stale-lock break, atomic-push lease failure, dispatch round-trip.