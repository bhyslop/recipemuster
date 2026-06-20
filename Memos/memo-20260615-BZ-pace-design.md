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

Why jilt (MCM Lapidary):

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

## Terrier — settled design (260618)

Consolidates the terrier design settled across the 260617–18 conversation. Intermediate positions explored and withdrawn along the way — a levy/founding graft, then a dedicated manor-provision op — are not narrated here; git history holds them.

Identity and home (RBS0-grounded, operator-sanctioned):

- Manor ≡ Payor Project. RBS0 typedefs `:rbtgi_payor_project:` as an alias of `:rbtgi_manor:` (RBS0:70); the manor quoin (RBS0:2068) defines a single control-plane GCP project; establish (manual Console) creates it. The operator's long-held understanding was right.
- The terrier bucket lives in the payor project. There is no separate manor project.
- The `rbgp_payor.sh` "Cannot create Governor in Payor project" guard stays: it keeps depot-scoped SAs out of the control plane, and the terrier is manor-grain control-plane *data*, so it honors the guard rather than crossing it. (The guard may retire with the enrobe estate at M7, but the separation it encodes should survive.)

₣BZ scope vs ₣Bf:

- ₣BZ builds the terrier capability — the three sub-ops, plus the net-new managed-folder layer: GCS `storage.managedFolders` REST (create/delete), a teardown that deletes folders (today's object-only emptying does not), and a managed-folder IAM wrapper mirroring the AR repo-IAM idiom over a new rbgi primitive shaped like the bucket-IAM one — and provisions the freehold's terrier via an interim scaffold tabtarget. ₣BZ's own paces (admission verbs, foedus-freehold roster, attribution proof) need only a working terrier on the freehold.
- The terrier's permanent founding-home — dedicated op vs enlarged affiance vs folded into a foedus-shaped establishment — is deferred to ₣Bf, because it is entangled with affiance's own evolution there (the foedus verb-register question, multiple federations). Captured as the ₣Bf "terrier's permanent founding-home" idea; the interim scaffold stands until ₣Bf consolidates. Signposts against re-conflation are in place: the abandoned graft-pace tombstone, the build pace's Cinched boundary line, and that ₣Bf idea.

Grain (settled):

- Write is folder-scoped, own-polity — a governor's mantle writes only its own depot's managed folder (managed-folder IAM).
- Read is bucket-level, manor-wide — every governor mantle reads all folders (operator ruling 260618: read across all terriers is fair at governor stature), so read is one bucket grant, not per-folder. The payor reads inherently as owner of the payor project (no grant).

Pace split (260618): the build work splits into two paces, each carrying its own service-tier test. Testing is deliberately NOT a separate pace — these tests are service-tier and inseparable from their code (provisioning is tested by provisioning; sub-ops by running them against live GCS preconditions), so there is no unit seam worth deferring and a coding-only pace would carry no meaningful local test.

- Provisioning pace (the reslated terrier build pace): idempotent bucket-ensure in the payor project, the net-new managed-folder REST (create/delete) and IAM wrapper (write folder-scoped, read bucket-wide), and the destroy-then-create scaffold tabtarget. Settles the bucket name + constant home and the managed-folder grain. Test: a service fixture that provisions via the scaffold and asserts bucket + per-polity folder + the write/read IAM, then asserts an idempotent reset; self-skips credless.
- Sub-ops pace (new, immediately after): engross / expunge / peruse against the RBSTR atomic contract. Settles the muniment JSON shape under a fresh terrier sprue and object granularity (per-entry vs per-subject). Test: the atomicity fixture (ifGenerationMatch=0 create → 412-on-conflict treated as idempotent), charging the terrier via the provisioning scaffold; self-skips credless.

## Manor-identity divergence — for Fable's review (260618)

Kept for the terminal Fable review to adjudicate; the terrier design above is otherwise settled.

RBS0 makes the Manor the Payor Project (a control-plane project — a building). The federation conversion introduced a second, org-level sense of "Manor" without reconciling RBS0: the paddock's "org anchor, a deed not a building" (grep-empty in the specs — paddock-only), and the federation machinery operating on an organization (`RBRF_ORG_ID`, affiance) rather than the project. The paddock body line is left in Fable's original wording under the freeze; the divergence is flagged by a freeze-banner pointer to this memo, and the terminal Fable review reconciles it in the paddock body. This section is the divergence's standing record.

Provenance hypothesis (two candidates, neutral):

1. Fable confabulation — the federation-conversion author reconceived the manor as an org-level "deed," a polished-but-ungrounded reframing (the "deed not a building" coinage is the tell, nowhere in the specs).
2. Editor spec/impl drift (the operator's own candidate) — conformance between spec, implementation, and paddock was not enforced, so the org-level sense accreted and the paddock voiced it.

Likely a blend. The practical correction stands regardless; the provenance call is Fable's, and it bears on a process question — whether paddock authoring needs a spec-conformance gate — larger than this heat.

## Test-rig doctrine — static admission, no in-test IAM churn (260619)

Settled in conversation 260619 (a long design walk during the ₢BZAAY mount). This is the conclusion the freehold test rig is built on; it supersedes the in-test grant/revoke ambition the same conversation briefly entertained. Per the freeze, it lives here, not in the paddock.

The scar that drove it: revocation-propagation pain. `setIamPolicy` is eventually consistent and the team has been bitten hard by it; the operator is very reluctant to make in-test role changes depend on propagation, now or ever.

The reframe that makes the static model strong (not a retreat):

- Propagation races live entirely in the TRANSITION (the moment of grant or revoke). A standing state — always-granted or never-granted — has no transition and is race-free.
- Negatives are therefore tested as NEVER-GRANTED standing states, not as revocation transitions. "Can't do X" = a principal that was never granted X — the strong, deterministic form; revocation-as-a-test was always the weak form.
- Revocation CORRECTNESS (did unseat issue the right delete?) is asserted by IAM read-back — `getIamPolicy` shows the binding gone, the `rbgp_admission_proof` pattern — never by attempting a don and waiting for a 403. Whether a removed binding eventually denies is Google's propagation: a Palisade, not our code's contract.
- "Sleep once" is really "absorb the grant-direction propagation once at freehold setup" via the existing poll-until-200 budget (the `rbgv` 60×3s pattern, the direction proven at spike F2). The unreliable revoke direction never appears.

Why this mirrors the architecture: the mantle design already pulls IAM mutation out of hot paths (frozen-at-levy + admission-only) and bounds exposure by token self-expiry. The historical propagation pain was the hot-path version; the test rig should mirror the architecture and keep all IAM mutation at provisioning time, never per-test.

The identity-layers model (the clarifying frame) — three layers, three durabilities, three homes:

- Citizen definition (name → Entra subject + mantle): PERMANENT, bare, pool-independent → `rbpc`.
- Foedus instance (workforce pool id + provider): EVOLVING, bumped on re-mint → `rbrf.env`.
- Depot instance (moniker + prefix): EVOLVING, rotated by churn → `rbrd.env`.

A citizen's definition is pool-independent (pinned to the Entra `oid` + a mantle), but its live admission is pool-scoped — the grantable `principal://…/workforcePools/{POOL_ID}/subject/{oid}` embeds the live pool id, so a foedus re-mint re-brevets the same permanent citizens into the new instance. Definitions survive; bindings re-establish.

Consequence — ₢BZAAY collapses to its invariant kernel (reslated 260619): `rbpc` homes a SINGLE durable freehold subject (the operator's standing Entra `oid` `9657166c-8a2d-4f5d-bcd1-ef481ee31f3e`, recorded at the federation-legs spike) projected to `RBTDGC_FREEHOLD_*`, plus one parameterized don-mantle probe (compear as the subject, don a named mantle, or surface the admission-deficit 403). The multi-citizen roster and the freehold depot moniker are DROPPED: with one real federated identity a roster is fiction, and depot identity is `rbrd.env`'s evolving concern (the depot-freehold is ₣Bf). The mantles are already RBCC constants.

Consequence — the next pace (the freehold fixture) carries the provision-once / standing-don / never-granted-negatives / read-back-revocation pattern. The full grant/revoke scenario matrix is NOT built; it is replaced by standing differentiated admission.

₣Bf coupling: full inter-mantle negative differentiation (a director-mantle citizen denied governor work) needs multiple standing subjects, which needs the degenerate/headless IdP — ₣Bf work, where the same static-no-churn doctrine applies. Recorded in `memo-20260619-Bf-static-negative-test-rig.md`.

## Manor colophon regroup (M5) — intent recovered, demesne consolidation routed to Fable (260620)

Groom of the M5 placeholder (the reslated ₢BZAAO). The recovered original intent, the tension the one-liner hid, and the health-driven scoping decision — recorded here for the terminal Fable review to ratify. The operator did not recall the movement's intent on mount; this is the reconstruction from the as-built surface, accepted in conversation.

Recovered intent. Movement 5's "manor colophon regroup. As before, plus the new founding gestures" meant: consolidate the **manor demesne** — the founding/un-founding verbs Levy / Establish / Affiance / Jilt — into one coherent colophon family. The demesne is real and spec-named: RBSMF's acronym entry tags levy "Manor demesne — depot levy ceremony," and the paddock's vocabulary elections cinch L/E/A(/J) as a sibling-initials set (the jilt mint extended it explicitly). But as-built the four are scattered across three colophon families: levy `rbw-dL` (depot), establish `rbw-gPE` (payor guide), affiance `rbw-mA` + jilt `rbw-mJ` (manor). The regroup would pull levy and establish into `rbw-m` so the demesne reads as siblings (`rbw-mL/mE/mA/mJ`). Movement 4 already landed affiance, jilt, and mantle-establishment-at-levy, so the "fold in the new founding gestures" half of M5 is spent; only the consolidation and the orphan `rbw-gq` cleanup remained.

The tension the one-liner hid. The consolidation is not mechanical — it forces a which-axis choice the movement never confronted. Levy is *both* a manor-demesne founding gesture *and* the depot **create** verb (sibling to unmake / list / info / recognosce in `rbw-d`); establish is *both* manor-demesne *and* a payor **guide** procedure (sibling to install / refresh in `rbw-gP`). A colophon family can encode only one axis — who acts (manor) or what is acted on (depot / guide) — so the regroup cannot seat levy and establish in `rbw-m` without orphaning them from their object-lifecycle siblings. Levy additionally carries operator-facing blast radius: literal `rbw-dL` hints live in `rbndb_base.sh` operator text, not only in const-insulated fixtures.

Decision (260620, operator-accepted). M5's reslated pace (₢BZAAO) executes only the uncontested kernel — `rbw-gq → rbw-gPQ`, the orphan QuotaBuild homing already resolved in the slate-picks above. The manor-demesne consolidation (levy / establish → `rbw-m`) is **routed to the terminal Fable review** as an explicit divergence, not executed at heat's-end: it is contested (the which-axis call), costlier than when planned (levy is now deeply wired), purely cosmetic (no behavior change), and exactly the family-coherence legibility judgment the terminal review owns — while the tail's real urgency is M7's live keyless-rewrite blocker, not M5's colophon legibility. Fable's charge: rule the which-axis question, then either author the demesne regroup into a fresh pace or affirm the scatter as load-bearing (levy and establish earning their depot / guide homes by object-lifecycle siblinghood, the demesne living only in the spec's prose and the verbs' shared register).

## RBRA estate retirement (M7) — gate forced early by the no-keys org, split into bootstrap + demolition (260620)

Groom of the M7 placeholder (₢BZAAQ), continuing the 260620 session. Recovered intent, the live-blocker reconsideration, the estate map, and the operator-accepted decomposition — recorded for the terminal Fable review and the two successor paces' mount agents.

Recovered intent. Movement 7 retires the RBRA keyfile-credential estate whole — "the bridge's demolition condition, executed," the heat's "zero SA keys, RBRA estate deleted whole" done-criterion. The paddock gates it: "suites stay green on keyfile until federation personas pass the same suites — then the RBRA estate retires whole." The M4/M7 review (THE OPEN QUESTION in memo-20260615-BZ-m4-review-findings) read that gate as M7-coupled at medium confidence — prove the verbs on the spike trust in M4, defer the keyless canonical-fixture rewrite to M7 to avoid double work — and named a revisit trigger: overturn to an earlier rewrite if anything comes to depend on a suite-green federation persona.

The live-blocker reconsideration (the trigger fired, from outside). The org now enforces disableServiceAccountKeyCreation, so the keyfile governor-enrobe case 400s on any fresh levy and canonical-establish dies there — the keyfile bootstrap fixture is already broken, not merely scheduled to retire. This fires the review's revisit trigger, but from an external policy change rather than from M5/M6 work. The keyless canonical rewrite is therefore forced now, ahead of the gate; and building it IS meeting the gate (canonical-establish on federation admission), which retires the review's one recorded residual risk — the federation surface unproven on personas until M7 — early rather than late.

The estate map (260620 Explore survey). The accessor seam localized the token mint: rba_token_capture is the sole non-probe caller of rbgo_get_token_capture, so the mint rip is one function. But ~20 consumers still carry a test-f/source guard against the raw RBDC_*_RBRA_FILE constant, so the constant retirement is a ~20-file mechanical sweep. Five clusters retire: (a) accessor keyfile branch + RBDC constants + rbra regime/CLI + the guard sweep (small core, medium fan-out); (b) the enrobe/defrock cult verbs rbw-aE/adE/arE/adF/arF + their bodies + the key-creation engine zrbgg_create_service_account_with_key (large); (c) the keyfile JWT probes rbw-acg/acr/acd + rbgv bodies + RBSAJ, replaced by the already-built federated rbw-acm/acf (medium); (d) RBSRA + the RBS0 quoin seats/includes (small, clean); (e) the Rust fixture coupling — the pristine full-SA-cycle case, the fast-tier credless guard, manifest/consts (medium). The rbgw capability-set definitions stay (they realize the mantle grant lists, not keyfile machinery); whether the cult roster verbs rbw-adr/arr retire or survive (federation rehearse may replace them) is a Pace-2 mount decision.

The decomposition (operator-accepted). M7 splits along the gate sentence's own seam:
- Pace 1 — keyless canonical bootstrap (the reslated ₢BZAAQ): rewrite canonical-establish's enrobe cases to admit federation personas (affiance rbw-mA, brevet rbw-pB, compear rbw-acf, don rbw-acm), every replacement verb already enrolled. Unblocks fresh-levy gauntlet; meets the gate; leaves the keyfile estate present as dead bridge legacy. The test-rig compearance step (per-run human click, memo-20260619) is the one live wrinkle for an autonomous gauntlet.
- Pace 2 — RBRA estate demolition whole (new, slated after the freehold-collapse pace): delete clusters (a)–(e); zero SA keys; the heat's done-criterion.

Sequencing with the freehold-collapse pace. Both the keyless bootstrap and the freehold-collapse pace rewrite rbtdrk_canonical.rs, but in disjoint regions (collapse owns Case 1 + the prefix/moniker helpers; the bootstrap owns the enrobe cases), so they compose. The freehold-collapse docket defers its enrobe-idempotency to this keyless rewrite, so order is keyless bootstrap → freehold collapse → estate demolition → terminal Fable review.
