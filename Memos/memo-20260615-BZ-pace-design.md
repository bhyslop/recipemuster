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
