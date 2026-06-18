# ₣BZ Pace-Design Memo — Opus stewarding a Fable-authored heat

Working surface for designing the remaining paces of ₣BZ (rbk-14-office-federation).
Provenance, not authority: the paddock is the design; this memo is where the Opus
pace-cutting forms and where Opus/Fable divergences are recorded.
Created 260615.

## The relationship this memo manages

The ₣BZ paddock was authored at Fable density and calibration.
The working agents are now Opus-class.
A first Opus read (this session) mistook load-bearing precision for over-treatment —
it manufactured a "movement-4 is gated by the Open items" blocker that reading the code dissolved.
Lesson carried forward: read the paddock as authority; suspect the divergence is *Opus's* first.

The paddock is frozen — the banner landed 260615 at the paddock head.
Corrections and tightenings are carried here and into the paces — never folded back into the
paddock — until Fable re-authors or the operator sanctions a change in conversation.

## Pace succession (forming)

Skeleton is the paddock's Ordering section (movements 1–8).
Movement 3 (the federation spike) is done as ₢BZAAA; the vocabulary mint is done as ₢BZAAB.
Remaining, to be cut:

- M1 — Accessor seam. Identity-keyed accessor swallowing the RBRA-file plumbing; behavior-preserving; grep-gate + suites as arbiter. No-regret. **Slated** as the accessor-seam pace (₢BZAAD).
- M2 — Capability-sets as named code. Lift the three grant lists (governor / director / retriever) into named definitions. No-regret. **Slated** as the capability-sets-as-code pace (₢BZAAE).
- M4 — Civic verbs + mantles + terriers. The behavior-changing movement; multi-pace; its first act renames the cult colophon family. Needs operator slate-time picks (see Open).
- M5 — Manor colophon regroup (+ founding gestures).
- M6 — Narrative docs (+ broadside registration of the elected words).
- M7 — RBRA estate retirement (after federation personas pass the suites).
- M8 — Handbook rework (home open: ₣A6 vs a final movement here).

How M4 decomposes into paces is the hard judgment; M1 and M2 are cuttable now.

## Code-grounding (from the 260615 read)

- Credential surface (M1 target): ~50 call sites, one shape —
  `rbgo_get_token_capture "${RBDC_<ROLE>_RBRA_FILE}"` — plus the governor wrapper
  `rba_get_governor_token_capture`.
  The sole durable secret that survives is the payor's RBRO.
  `rba_auth.sh` is the natural accessor home.
- Capability-sets (M2 target): grant lists live inside `rbgg_invest_director` /
  `rbgg_invest_retriever` and the governor mantle in `rbgg_governor.sh`, expressed via
  `rbgi_add_{project,repo,sa,bucket}_iam_role`.
  The key-minting body `zrbgg_create_service_account_with_key` is what eventually retires.

## Opus/Fable divergences (candidates — paddock stays as-is)

Held lightly; each is a place where Opus execution would tighten what Fable wrote, carried
in the pace and here, not in the paddock.

1. The paddock's "Open — resolve within the heat" list reads as five open architecture
   questions but is mostly a closed slate-picklist (test-rig credential, terrier
   bucket/`rbgb_` allocation, beta-memo disposition).
   Reframing them as picks-not-blockers is what unblocked M1/M2.
   The relevant dockets collapse each to a one-line slate pick.
2. The mantle homonym guard (~6 paddock lines) carries one load-bearing sentence —
   "M4's first act renames the cult colophon family."
   A single docket line absorbs it.

Add as we find more.
Resist the pull to call precision over-treatment — divergence is Opus-first suspect.

## Slate-time picks — resolved in the 260615 trot

- **Test-rig synthetic-persona credential → per-run human click.**
  A 12 h assize on the project's own test org.
  The zero-secret invariant then holds even in the test estate,
  and the suites exercise the real Leg-1 compearance rather than a secret-shortcut.
  The Keycloak-in-a-crucible password grant stays paper-only (as the paddock already records it) —
  documented-viable, not a planned fallback.

- **Terrier — write-semantics and structural home settled; internals parked.**
  Governor terrier writes are atomic at GCS itself via object preconditions
  (`ifGenerationMatch=0` create, `ifGenerationMatch=<gen>` update; 412-on-conflict; no cloud invocation, no external lock) —
  a single conditioned REST call, within bash's BCG-good case (web-confirmed).
  Homed in the new `RBSTR-Terrier.adoc` (acronym registered), reframed across the trot to its settled identity:
  a **standing terrier sub-operation subdoc** defining the atomic write / withdraw / read of a muniment,
  with invest / divest / rehearse as thin RBS0-side wrappers that reference it (ACG — reference the home, don't recreate).
  The terrier **noun** (object format, bucket, managed-folder IAM grain) is a separate RBS0 civic quoin seated at M4;
  RBSTR's sub-op quoins + `include::` into RBS0 are contract-first work of the M4 terrier pace.
  Internals — per-entry vs per-subject granularity, bucket name, managed-folder IAM — parked to that pace.

- **Cult-rename target → garment-of-office mint: enrobe / defrock.**
  M4's first act renames the legacy keyfile-SA family off the words federation claims (invest / divest / mantle).
  Operator chose a proper mint (rare-and-colorful) over a cheap marker:
  **enrobe** = create a key-backed SA — unifies cult `invest` (director/retriever) and `mantle` (governor),
  subject differing (payor enrobes the governor; governor enrobes director/retriever);
  **defrock** = decommission one (replaces cult `divest`);
  `roster` stays (carries no homonym).
  Grep-gated clean; transitional ashlar (doomed at M7) — lands in specs/acronym map but NOT the README broadside.
  `mantle` is reserved to federation and cannot be reused;
  the garment register deliberately rhymes with federation's (mantle / don) while the words differ.

## Slate-time picks — resolved (260615 trot, cont.)

- **Handbook rework home → ₣A6.**
  When federation lands and the keyfile estate retires, onboarding is rewritten
  (install-RBRA-key-files → morning compearance, one device-flow click).
  That rework is chivvied into ₣A6 (the handbook-restart heat) as a separate pace — not authored here.

- **`rbw-gq` (QuotaBuild) → stays a guide, regroups under Payor as `rbw-gPQ`.**
  Cloud Build quota/capacity is a Payor concern — the Payor owns the project, billing, and quota;
  the Director only consumes builds — so QuotaBuild joins the Payor guide subfamily (`rbw-gPI/gPE/gPR`).
  The orphan `rbw-gq` becomes `rbw-gPQ`; the colophon rename rides the M5 regroup, not authored here.

- **Beta-repo evidence memos → already in this repo; no action.**
  Both 2026-06-11 memos (`memo-20260611-google-impersonation-preference`,
  `memo-20260611-token-custody-context-enforcement`) are present in `Memos/`; migrate-vs-point-at is moot.
  Their load-bearing facts are already absorbed into the paddock cinches and the token-custody section,
  so they stand as pure provenance.

## Movement-4 review findings (260615) — see the dedicated memo

The deep adversarial review of the seven movement-4 paces (run by the pace-design
membrane pace under a balanced-merits workflow) is recorded in
`Memos/memo-20260615-BZ-m4-review-findings.md`.
That memo is the entry point for model Fable's terminal review of the heat (the
terminal Fable-review pace calls it out by name).
Headline: four of the five judgment calls held the first cut; the levy-founding pace
split into two (a net-new affiance pace plus a reshaped levy/mantle-establishment
pace), taking movement 4 to eight paces.
The one medium-confidence call — when federation personas must pass the suites — took
the M7-coupled reading, with its residual risk recorded for Fable to overturn.

## Colophon ground-truth inventory + homonym-guard vestige (260616)

Opus grep inventory this session of the admission/cult verbs across code, specs, and tabtargets —
the as-built reality against the paddock's mantle-homonym-guard (divergence #2 above),
recorded for Fable's terminal review and the remaining M4 cult/admission paces.

As-built:

- The cult/legacy keyfile-SA colophons are live as enrobe/defrock and pervasive —
  bash (rbgg_governor, rbgp_payor, rbgi_iam, rbgw_capabilities, rbcc_/rbgc_constants),
  rbz_zipper, generated rbtdgc_consts.rs, seven theurge fixtures, both handbook wrappers,
  the enrobe/defrock specs (RBSDK/RBSGM/RBSDD/RBSRD), and the five rbw-aE/adE/arE/adF/arF tabtargets.
  ₢BZAAF [complete] is the rename that landed this; its brief confirms the cult sense of invest/divest/mantle was vacated repo-wide.
- brevet/unseat have landed as RBS0 civic quoins (:rbtf_brevet:/:rbtf_unseat:), referenced across RBS0, RBSMA, RBSMF, RBSTR —
  but not in code or tabtargets (no rbw-p*; the admission-verb pace ₢BZAAL is unbuilt).
  The 260616 re-election moved the federation admission verbs invest/divest -> brevet/unseat in the paddock and these specs.
- No live invest/divest cult residue (₢BZAAF cleared it).
  The one straggler was a stale federation-verb reference in a derived context mirror —
  claude-rbk-acronyms.md's RBSTR entry still said "invest/divest/rehearse wrappers" (the 260615 names), not the 260616 brevet/unseat;
  fixed this session to brevet/unseat/rehearse to match RBSTR.
  (A historical memo-filename citation in RBSCIP, "canonical-divest-delete-flap", is provenance, left as-is.)

Homonym-guard vestige (extends divergence #2):

- The guard names the cult family rbw-aM/arI/adI/arD/adD (mantle/invest/divest) — the PRE-₢BZAAF state.
  That family no longer exists; it is enrobe/defrock now.
- ₢BZAAF's stated purpose was to "discharge the homonym guard"; ₢BZAAF is complete, so the guard's resolution condition
  ("deletes itself when the rename lands") is MET.
  The guard survives only because the paddock froze before it could self-delete — a vestige the freeze pins in place.
- Doubly moot: the 260616 invest/divest -> brevet/unseat re-election means even the words the guard protects are no longer the admission verbs;
  brevet/unseat sit in the civic specs with no live cult collision.
- Not corrected in the frozen paddock: there is no purely-factual fix (re-aim / rewrite / retire the guard is a vocabulary judgment, and the text is Fable's),
  and the freeze routes corrections here.
  Natural cleanup rides the M4 admission/cult work (where the guard was always meant to self-delete) or Fable's reconciliation.

Note: line 80 above ("invest/divest/rehearse as thin RBS0-side wrappers") is the faithful 260615-trot record;
it is superseded by the 260616 brevet/unseat re-election noted here, left as history rather than rewritten.

## Manor jilt verb — operator-sanctioned mint (260616)

Operator sanctioned, in conversation 260616, a new manor verb: **jilt** —
the inverse of affiance, dissolving a workforce pool (breaking the manor↔IdP betrothal).
Colophon `rbw-mJ`, frontispiece PayorJiltsManor, payor-credentialed.
An operator verb, not a test-only teardown gesture:
the create/delete round-trip becomes test cases, so jilt earns first-class verb status with its own contract.

Why jilt (MCM Word Selection):

- **Asterism — the decider.** Affiance is the betrothal/diplomatic-marriage register
  ("the pledged faith between manor and IdP; *fiancé* keeps it warm").
  To jilt is to break off a betrothal — affiance ↔ jilt is a true antonym pair *inside the same asterism*,
  mirroring create-trust ↔ dissolve-trust.
  Sunder (the runner-up) severs any union; jilt severs *this* one — the tighter fit, chosen over sunder.
- **Sibling initials — clean.** Manor demesne is Levy / Establish / Affiance (L/E/A);
  +Jilt keeps all four distinct.
  J also clears the polity Brevet/Unseat/Attaint/Rehearse and the cult-garment Enrobe/deFrock/roster.
- **Grep gate:** 0 hits repo-wide (jilt, jilts). Clean landing.
- **Vocabulary isolation:** courtship vocabulary, zero ambient software/cloud/CI presence — not a trodden word.
- **Husbandry:** a pool teardown is rarer than affiance itself;
  husbandry prescribes a rare, colorful word for such low-traffic ceremonies.

Exposure: ashlar (operator-triggered; surfaces in error output).
Unlike the transitional enrobe/defrock (doomed at M7), jilt is a permanent capability —
so it IS broadside-eligible and registers with the federation vocabulary at the M6 narrative-docs movement.
Contract spec: **RBSMJ** (M-seat, verified free), contract-first per the documentation-strategy cinch;
RBS0 civic-quoin seat in the federation category alongside affiance.

Scope note for Fable: this adds a permanent un-founding verb beyond the paddock's frozen
founding triad (Levy / Establish / Affiance).
The paddock does not cinch *against* a manor delete — it omits one (founding is rare; un-founding was outside the MVP first cut);
the operator pulled it into ₣BZ in conversation because the affiance create-path de-risk needs a real delete and the round-trip is worth testing.
A full manor-lifecycle re-architecture would route to ₣Bf; this single sanctioned verb stays in-heat.

Pace structure: jilt gets its own contract-first build pace (silks manor-jilt),
slated immediately before the affiance-proof pace, which depends on it for the Tier B teardown.

## Freehold doctrine (260617 — post-freeze sharpening, operator-sanctioned)

This section records design that landed after the paddock froze; per the freeze banner it lives here, not in the paddock. The terminal Fable heat-review reconciles it.

A *freehold* (minted 260617, grep-clean) is the durable, deliberately-kept test installation reused day-to-day to dodge the create/destroy churn quota — the freehold/leasehold contrast against an ephemeral create→destroy lifecycle fixture. The concept cross-cuts depot and foedus. (The configured-federation noun *foedus* it builds on was already the ₣Bf working lean, flagged for Fable as an RBS0 civic quoin; this session re-derived it independently and adopted it.)

Two foedus fixtures, mirroring the gauntlet(churn)/skirmish(reuse) split:
- foedus-lifecycle (the reslated affiance-proof pace): ephemeral create→jilt round-trip, the repeatable autonomous proof of affiance+jilt. Quota-touching (a genuine create cannot reuse a soft-deleted id), so operator-invoked, never a routine auto-suite member; self-skips on the payor cred. The create-shape bug (the org parent is a body field, not a query param) was found and fixed proving this by hand. NOT YET BUILT — the fixture is staged but unwritten; this pace is where it gets authored.
- foedus-freehold (its own pace): durable establish-if-absent + verify, reusing one standing pool (quota-flat). Carries the affiance undelete-on-DELETED fix as prerequisite.

Quota constraint (memo-20260617-BZ-workforce-pool-constraints): 100 workforce pools per org; soft-deleted pools hold their id-namespace AND count against the cap for ~30 days; a soft-deleted id cannot be re-created, only undeleted. So fresh-per-run is quota-threatening (the depot-project pain, recurring); the durable freehold must reuse one pool via undelete.

Latent affiance bug surfaced by the jilt proof: GET on a soft-deleted pool returns 200 / state DELETED, so affiance's "200 → already present, skip create" treats a dead pool as live. The fix (undelete-on-DELETED) is folded into the foedus-freehold pace as its prerequisite.

The roster correction (retracting an earlier Opus claim, prompted by the operator): the foedus freehold is NOT roster-free. The IdP owns only who-can-authenticate; the *admitted* standing citizens are recorded in the manor-homed terrier and readable via rehearse. So the foedus-freehold verify head-counts a roster — and the machinery (terrier, brevet, rehearse) all lands in ₣BZ, making it a BZ-era capability. ("Providers are a roster" was a weak prop and is dropped — a pool's providers are an enumerable list but usually a singleton; the citizens are the substantive roster.)

The freeholds are intertwined: a muniment binds a foedus principal to a depot mantle, so the standing-citizen roster IS the join — the manor-wide roll is the foedus view, the per-polity slice the depot view. Post-impersonation a depot's mantles are donnable only through a foedus, so a depot freehold presupposes a foedus freehold. The depot-side (per-polity slice, post-impersonation depot nature, the canonical→freehold rename of depot test infrastructure, and the parked multi-foedus-per-depot capability) is deferred to ₣Bf, captured there as the freehold paddock idea.

Release-cadence refresh: when the quota-touching lifecycle does run (e.g. at releases), it also refreshes the freehold — jilt then re-establish, ordered after the lifecycle's own create→jilt passes, so cleanup is proven on a throwaway before it touches the durable pool. Buys staleness-isolation.

## Terrier homing correction — levy is depots, the bucket is the manor's (260617, operator-sanctioned)

Surfaced in conversation 260617 while orienting the terrier-live pace around the operator's idempotent-tabtarget-then-graft strategy; the operator caught the slip and sanctioned the correction in-heat.

The paddock's Terrier section says the per-polity terriers are "payor-created at levy." But levy creates a *depot* (`rbgp_depot_levy`; RBSMF is "Manor demesne — depot levy"), while the Terrier bucket is **Manor-homed** — one bucket shared across every depot under the manor (same paddock sentence). A manor-shared bucket cannot be born at depot levy: it would have no durable home (the build bucket lives in the depot project and dies with it), and a second depot's levy would collide with the first depot's bucket.

The correction (a grain-sharpening, not an architecture change — stays in-heat per the freeze's correction/evolution split):

- The **Manor-homed bucket** is an idempotent *ensure* grafted into **manor founding** (establish/affiance — the payor founding acts), not into levy.
- The **per-polity managed folder + its managed-folder IAM** is depot-grain and grafts into **levy**, one folder added per depot founded.
- Code consequence: the terrier bucket-create must NOT copy the build-bucket's 409-is-fatal pristine guard (the "Create build bucket" step in `rbgp_depot_levy`); a manor-shared bucket's second-depot founding legitimately finds it present.

Why "at levy" reads true in the frozen paddock: for the MVP one-depot-per-manor freehold, manor founding and the single depot's levy coincide, so the grain distinction is invisible. It bites only at the second depot — exactly the multi-foedus / multi-depot-per-manor capability the freehold doctrine above already defers to ₣Bf. So the split is consistent: the homing *sharpening* is a BZ correction (recorded here); the multi-depot *generalization* that makes it load-bearing is ₣Bf.

Pace shape (260617): the terrier-live pace is reslated to build-and-verify-via-tabtarget — an idempotent destroy-then-create terrier provisioning tabtarget run against the freehold, so the noun internals (muniment shape, managed-folder grain, bucket name/home) settle through cheap iteration with zero depot/manor quota churn. A new graft-and-demolish pace folds the proven sequence into founding (bucket→manor founding, folder→levy) and deletes the scaffold tabtarget — the demolition condition as a committed pace. The tabtarget deliberately outlives the terrier-consuming dev paces (admission verbs, foedus-freehold) so they get cheap resets; demolition is last.

Net-new surface the build pace carries (grounded 260617): GCS `storage.managedFolders` REST (create/delete) is absent repo-wide; a managed-folder IAM wrapper mirrors the AR repo-IAM idiom in the registry module over a new rbgi primitive shaped like the bucket-IAM primitive; and teardown must delete managed folders, which today's object-only bucket-emptying does not.

## Terrier pace review — confusions surfaced and ameliorations (260617)

A cold-read review of the two terrier dockets (the reslated build-and-verify pace and the new graft-and-demolish pace) surfaced gaps the first drafts papered over. Recorded here so the corrections survive the chat reset; the dockets were amended the same session.

Confusions found:

- **The bucket's hosting project was never settled, and the graft docket pre-committed a target it could not know.** "Manor-homed" implies a durable manor-scoped project, but the repo has only the payor project and per-depot projects (the build bucket dies with its depot). RBSTR and the paddock park "bucket name + constant home" yet never name "which project hosts it." The graft docket wrote "establish/affiance" as if decided — and affiance has no GCS project at all (org-level pool only), so it was simply wrong as a bucket target, while establish-vs-first-levy was an undecided fork. Amelioration: the build pace now carries the project-home as an explicit decision it must make and record (payor project vs a dedicated manor project — note the latter may be net-new infra), and the graft pace's target is written as contingent on that decision, with affiance excluded.

- **The read-population grain was conflated with own-polity write.** The first Done-when paired "own-polity write IAM" with "read population governors-and-above" as one binding. They are two grains: write is folder-scoped, but the foedus-freehold roster head-count needs governors-and-above to read manor-wide across polities. There is also a latent source conflict (early design prose said governors read own-depot; the paddock says governors-and-above) and an unsettled payor read mechanism (OAuth user vs a mantle). Amelioration: the build pace's first criterion now asserts only own-polity write; the read grain is a named decision feeding the foedus-freehold pace.

- **The terminal Fable review did not list this divergence.** The homing-correction is a divergence from the frozen paddock's "payor-created at levy," but the Fable-review pace's reconciliation checklist did not name it, risking a silent paddock/code contradiction. Amelioration: the Fable-review pace is reslated to ratify-or-overturn the homing-correction and note its open questions.

Smaller ameliorations folded into the dockets:

- The provisioning tabtarget churns at folder-grain by default (delete the managed folder + its muniments, keep the bucket) — recreating the whole bucket each iteration risks GCS's same-name reuse lag, which would undercut the cheap-iteration premise.
- "the standing freehold" disambiguated to the standing depot freehold (the one whose governor mantle the write IAM binds); the manor/foedus freehold is the later foedus-freehold pace.
- The tabtarget's colophon is transitional ashlar, not broadside-registered (the enrobe/defrock precedent).
- The graft pace's own verification (a fresh keyless founding) is quota-touching — a full manor+depot stand-up — so it is operator-invoked / release-cadence, never a routine auto-suite member; the build-pace fixture self-skips without service credentials.

Load-bearing reconciliation note: the frozen paddock still reads "payor-created at levy." Because the paddock is read-only until the Fable review, that line contradicts the dockets on its face; the only bridge is the docket pointer to this memo's terrier-homing-correction. A reader who skips the memo will read the contradiction as real. This is unavoidable while the freeze holds — it resolves only when the Fable review re-authors the paddock.

## Manor identity — the Manor IS the Payor Project (260618, operator-sanctioned; paddock corrected in conversation)

Research this session (RBS0, RBSRF, RBSPE, RBSMA, source) resolved decision #1 (the terrier bucket's home) and surfaced a spec/paddock divergence the terminal Fable review must adjudicate.

Finding — the authoritative noun model makes them one entity:

- RBS0 typedefs `:rbtgi_payor_project:` as an ALIAS pointing at the same anchor as `:rbtgi_manor:` (RBS0:70) — Manor and Payor Project are the same entity, two names.
- The manor quoin (RBS0:2068) defines it as "the singular administrative GCP project that hosts the Payor SA, the OAuth client, and the billing account ... the control plane from which depot projects are created." A project, not an org.
- RBSPE's establish step 1 is titled "Create the Manor" and creates the payor project; the handbook says "creating the Manor's GCP project."
- So Manor ≡ Payor Project (`RBRP_PAYOR_PROJECT_ID`). The operator's long-held understanding was correct.

The divergence (for Fable review) — the federation conversion introduced a SECOND, org-level sense of "Manor" without reconciling RBS0:

- The paddock asserted "every manor brings a domain (org anchor, a deed not a building), a Cloud Identity org ..." — calling the manor a deed (org-level), contradicting RBS0's "the building" (a project).
- The phrase "org anchor, a deed not a building" exists ONLY in the paddock (grep-empty in the specs).
- The federation regime (RBSRF) and affiance operate on an organization (`RBRF_ORG_ID`), never the payor project — consistent with the org-level sense, and never reconciled back to RBS0's project definition.

Provenance hypothesis (two candidates, recorded neutrally for Fable to adjudicate):

1. **Fable confabulation.** The federation-conversion paddock author (Fable-class) reconceived the manor as an org-level "deed" — a plausible-but-unspecified reframing not grounded in RBS0, of the kind a confident author introduces when the source is not re-read alongside the writing. The "deed not a building" coinage is the tell: rhetorically polished, load-bearing to the reframing, and nowhere in the specs.
2. **Editor spec/impl drift (the operator's own candidate).** The editor did not enforce high spec↔implementation↔paddock conformance, so the org-level federation machinery (`RBRF_ORG_ID`, affiance) accreted a divergent "manor" sense that was never reconciled to RBS0's project definition; the paddock then voiced the drift.

Both fit the evidence; the truth may be a blend — a confabulated coinage that lax conformance left unchallenged. The practical correction stands regardless of which; the provenance call is the Fable review's, and it bears on a process question (does paddock authoring need a spec-conformance gate) larger than this heat.

Correction applied (operator-sanctioned in conversation, 260618):

- The paddock Boundary line is corrected in place: the manor is the payor project (RBS0's control-plane project — a building, not a deed); federation additionally requires that project sit under a Cloud Identity org with an IdP, the org hosting the workforce pool but not itself being the manor. The freeze banner records the sanction.
- This is the one paddock-body edit made under operator sanction; all other corrections remain memo/pace-homed per the freeze.

Consequences settled:

- **Decision #1 (terrier bucket home) resolves: the bucket lives in the payor project (= the manor).** No separate manor project exists or is warranted; "dedicated manor project" is rejected as net-new infra for no gain.
- **The `rbgp_payor.sh` guard "Cannot create Governor in Payor project" is KEPT, not removed.** It enforces the control-plane/workload separation RBS0 intends: depot-scoped SAs (governors, and the mantles) belong in depot projects, never the control-plane manor project. The terrier does NOT conflict with it — the terrier is manor-grain control-plane *data*, not depot infra, so it lives in the manor/payor project honoring the guard. (The earlier "counter-precedent" flag was imprecise: the guard's spirit is "depot infra out of the control plane," and the terrier is control-plane, so it is aligned, not crossing.) The guard may retire with the enrobe estate at M7, but the separation it encodes should survive that retirement.

Ceremony decision — a dedicated scriptable manor-provision step (project integrity):

- The Manor (payor project) is created MANUALLY via Console (RBSPE). There is today NO scriptable operation that provisions the manor's programmatic cloud resources — establish is hand-done, affiance does only the org-level IdP trust, levy does depot resources.
- The terrier bucket is the FIRST manor-grain cloud resource needing programmatic management, so it forces the question: (A) bolt the bucket-ensure onto affiance, or (B) mint a dedicated scriptable manor-provision operation.
- **Chosen: (B), ruled by the operator as required for project integrity now.** Bolting onto affiance would make affiance lie about its scope (it is "establish the manor's IdP trust," not "...and provision its storage"), mixing org-work and project-work. A dedicated manor-provision operation is the honest home, fills the real gap (the manor has no scriptable resource-provisioning act), and is the seam where future manor-grain resources land. The MVP-surface cost — a new manor-demesne colophon, since L/E/A/J are taken — is outweighed by the integrity gain.
- Shape: the manor-provision op is permanent (idempotent ensure of the terrier bucket in the payor project; payor-credentialed; run after the manual establish, joined to the onboarding founding sequence). It is NOT the demolished scaffold — what the graft pace retires is only the dev-only reset/destroy convenience used to iterate noun internals; the ensure op persists. The strategy's quota-flat iteration still holds: the reset capability rides alongside through the terrier-consuming dev paces and is retired at the graft.

## Re-cut — affiance stays narrow in ₣BZ; the terrier's founding-home defers to ₣Bf (260618, operator-sanctioned)

Supersedes the ceremony decision in the manor-identity section above (the dedicated manor-provision op, "option B"). That decision is withdrawn — not wrong, but premature for ₣BZ.

Trigger: re-reading ₣Bf (rbk-14-mvp-federation-evolution, the federation-evolution holding heat). ₣Bf already holds the manor/affiance consolidation as deliberate deferrals — the foedus noun (with the open "do the verbs affiance/jilt migrate to match it" verb-register question, flagged for Fable), multiple federations (affiance keyed to named instances), and the payor-as-IT-department governor-selects model. So affiance's very shape and scope are Fable's call in ₣Bf.

Consequence: enlarging affiance in ₣BZ — or even building a permanent dedicated manor-provision op — pre-empts that consolidation. The terrier's permanent founding-home (dedicated op vs enlarged affiance vs folded into a foedus-shaped manor establishment) IS the consolidation ₣Bf owns.

The cut:

- ₣BZ builds the terrier CAPABILITY (sub-ops, managed-folder REST + IAM) and provisions the freehold's terrier via the interim scaffold tabtarget. Its own paces (admission verbs, foedus-freehold roster, attribution proof) need only a working terrier on the freehold, which the scaffold gives. No permanent manor-provision op, no founding graft in ₣BZ.
- The scaffold is interim — the standing terrier-provisioning tool until ₣Bf consolidates — retired there, not demolished in ₣BZ.
- The graft-and-demolish pace (the former founding-integration pace) is abandoned in ₣BZ; its intent moves to ₣Bf as a named-tension idea ("the terrier's permanent founding-home"), entangled with foedus / multiple-federations / affiance-evolution so Fable decides the whole manor-establishment shape at once.

Unchanged (RBS0-grounded ₣BZ corrections, not consolidation-entangled): manor ≡ payor project; the terrier bucket lives in the payor project; the `rbgp_payor.sh` guard stays.

Anti-conflation note: the ₣BZ↔₣Bf boundary keeps blurring because the heats touch the same machinery and "MVP vs evolution" is fuzzy under pressure — it bit this session twice (the manor-provision op baked in, then withdrawn). The durable mitigation is signposting at the points of stumble, placed in the artifacts: the abandoned graft-pace tombstone (where a future groom asks "where is the founding-integration pace?"), the terrier build pace's Cinched boundary line ("provision via the interim scaffold; do not graft"), and the ₣Bf named-tension idea written so any re-derivation of "enlarge affiance now" lands on a ready answer rather than blank space. Deliberately NOT signposted into the frozen ₣BZ paddock — its banner already routes out-of-scope ideas to ₣Bf, and the specific deflection lives in the docket where it is read.
