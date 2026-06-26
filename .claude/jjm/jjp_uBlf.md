## Charter — the federation-build stream

This heat is the federation-BUILD stream of the MVP federation work.
It owns the federation/manor region of RBS0-SpecTop and the federation entries of the acronym registry (claude-rbk-acronyms.md), and it carries the federation idea-catalog of still-live strands deliberately deferred from ₣BZ.

The spine is slated: the spec-first recast, the mechanism-gated affiance arm, the Keycloak orchestrator, the programmatic accessor, and the per-vendor setup guide.
The attach-caged-subject tail consumes ₣BZ's admission verbs; with ₣BZ complete those verbs have landed, so the tail is slatable too rather than held.
Five federation-MVP re-derivations (the keyfile-era loose-ends restrung in from the sibling MVP heat) are folded in below, re-deriving against the now-stable federation auth surface this spine builds.

Posture (cinched for this heat): premise-touching ideas are operator-owned — any idea that would amend a cinched ₣BZ premise stays a parked idea until the operator takes it up.
Keep this catalog shape-shaped — each idea a named tension plus a current lean, never a running discussion log.
Genuinely thin or orthogonal ideas belong in itches or the horizon roadmap, not here; this paddock is for ideas worth deliberating with heat context.

## Federation vocabulary — the minted census (chosen; the recast does not re-derive these)

The federation naming is substantially settled; this census is the single prominent home, so a mount agent reads "chosen" before reaching for the mint.
Built-word deliberation stays drained to the heat-memories memo — what lives here is the standing roster and what remains owed: shape, not deliberation.

Chosen (do NOT re-mint — re-deriving these is the failure this census guards against):
- foedus — the configured federation, the noun affiance founds; this recast seats its rbtf_ civic quoin, the word itself is adopted.
- avow / sitting / quash — the human device-flow sign-in, the live window it opens, and the test-only sitting-clear.
- instate / descry — switch which foedus is active, and read a foedus pool's health; minted, owned by the theurge stream where the build is slated (not yet landed).
- interactive / programmatic — the mechanism gate's two values, KEPT as the plain words (260626): test-scoped — read by affiance and the accessor, written only by the test establishment, never a tabtarget argument, so the asterism's weight does not apply.
  Serialized as rbnfe_interactive / rbnfe_programmatic — the rbnfe_ federation enum sprue, derived from the rbnve_ (vessel) / rbnne_ (nameplate) scheme, not a free mint.
- RBRF_MECHANISM — the regime field holding the gate value (260626), modeled on RBRV_VESSEL_MODE.
- RBSFE / RBSFA — the two operation-subdoc acronyms (260626): RBSFE the programmatic establishment, RBSFA the programmatic accessor; fresh RBSF tails (only RBSFH was taken), distinct-over-mnemonic per operator ruling. The operation-quoin verb stays descriptive — the civic verb defers to the build pace.
- canvass — read-only enumeration of every foedus by interrogating the Manor (workforcePools.list), the descry sibling (descry inspects one pool, canvass lists all); chosen 260626 via the Lapidary gates (grep-clean, fair-faced, civic register, sibling-initial c safe against descry-d/instate-i). Colophon rbw-jc, quoin rbtf_canvass; build + test slated as placeholders in the theurge stream (foedus-cardinality family), full docket deferred to a design pass.

Still owed by the recast — the open call this pace settles:
- the active-foedus selector and foedus-identifier field — a TEST-ONLY overlay: the production regime stays single-foedus-clean (effectively singleton, no selector), the multi-config + selector riding on top for the test-bed alone.
  Open question under discussion (260626): lean toward the Manor (GCP) as the authoritative foedus registry — pool-ID as the identifier, "list foedera" a cloud query — so the regime holds the active config, not a local registry.

As each owed item settles it moves up into the chosen list here, so the record never scatters again.

## Conviction — the federation configuration model (settled modeling axis)

The one settled shape in this paddock: the modeling axis is decided, even though the build is not.
What is settled is HOW a federation is modeled; the build spine is now slated, and the vocabulary it needs is minted in-heat (operator decision 260623).

The deciding finding: Google Workforce Identity Federation provider config is vendor-invariant — the same gcloud flags with identical semantics for Microsoft Entra, Okta, or generic OIDC, only the values differ (doc-confirmed against live GCP docs).
So the regime must not enumerate vendors; enumerating them was reasoning from exactly two, and the docs settle that the shape generalizes.

Variation sorts into three buckets, and only one belongs in the regime:
IdP console setup (register the app, configure claims and scopes) is foreign-console human work — it belongs in a per-vendor guide, never a regime field or a tabtarget;
opaque values (issuer URI, client-id, subject claim) belong in one vendor-agnostic core — already the federation regime's stated design, and the docs prove it generalizes past Entra;
the token-acquisition mechanism (interactive device-flow versus programmatic self-supplied JWT) is the one real schema discriminator — the only thing that changes the required-field shape and the code path in affiance and the accessor.

The discriminator is mechanism, not vendor.
Vendors are open-ended but collapse onto a closed, tiny set of acquisition mechanisms; a new vendor never adds a mechanism, it slots into one, so adding a vendor is a guide plus values — no schema change, no code change.

The landing model: the federation regime is a vendor-agnostic trust core plus an acquisition-mechanism gate.
The core is always present — org, pool, provider, session-duration, client-id, attribute-mapping, issuer or JWKS source.
The interactive arm carries the device-authorization and token endpoints and the device scope; the programmatic arm carries an uploaded public JWKS (the caged case); vendor identity is not a regime field at all.

Scope of this model: it governs one foedus's shape (vendor-agnostic core plus mechanism gate).
How MANY foedera the manor holds at once is the separate multiplicity axis below — orthogonal to this per-foedus shape, and now (260622) settled as a GOAL of the heat, not a deferred axis: the manor holds several foedera and a depot draws from a chosen one.
The singleton is no longer the model's boundary; multiplicity rides on top of the per-foedus shape, which is unchanged.

The machinery is precedented in-tree: the vessel regime gates fields per mode via buv_enum_enroll plus buv_gate_enroll.
The federation mechanism gate is the same pattern repurposed from a rejected vendor discriminator onto mechanism, where it is load-bearing.

The 'ships committed, no secrets' invariant is preserved by exactly this seam: the programmatic arm carries the public JWKS (public keys commit fine), while the private signing key never enters the regime — it lives marshal-fenced.
The discriminator splits public config from the one durable secret along the right line.

Boundary: vendor-invariance holds for OIDC only.
The one place a genuine per-protocol fork would reappear is OIDC versus SAML — a structurally different provider — but the manor is OIDC-only, so that is out of scope.

PROVEN end-to-end (260622): the caged programmatic path — a Keycloak-minted OIDC id_token via uploaded JWKS → GCP STS → don → authorized depot-API call — ran GREEN in our own harness, no longer merely doc-confirmed. Full findings in the config-model memo's "Proof result" section.
Decision economy (operator ruling, refined 260622 post-proof): the chain was proven by an expedient gcloud POC (now torn down); the DURABLE establishment + accessor are REST-only — NO direct gcloud — and are the deferred build-units, not this proof.
Three-layer home (cinched): the realm config is baked vessel DATA; the establishment + accessor are station BCG MODULES (orchestration); Keycloak itself is the vessel.
De-lamination (settled 260623): the proof script is a flattened cross-section of all three layers, and productizing de-laminates it.
configure_realm's live admin-REST evaporates into declarative entries in the baked fdkyclk-realm.json (the import mechanism already rides the vessel Dockerfile's --import-realm, and the realm JSON is currently a near-empty shell carrying only the realm envelope), so the durable rig holds no runtime admin-REST.
The JWKS rotates per charge — the realm signing key is generated fresh on import, deliberately committing no secret — so affiance re-syncs the uploaded public keys on every establish rather than once (the JWKS-refresh coupling, made concrete).
The test-facility orchestrator is ONE coherent BCG module: charge then ready-poll then fetch-JWKS then call affiance, and the inverse jilt then quench.
It composes existing verbs plus the one JWKS bridge; it does NOT contain the accessor (that stays a mechanism arm of rba, the production-shared STS-then-don machinery) and it does NOT reimplement affiance or jilt (existing production verbs, called not contained — brevet is admission, owned by the separate deferred attach-caged-subject unit, never this orchestrator).
Membrane: affiance never learns "Keycloak" — all Keycloak knowledge is quarantined in the baked realm DATA and this orchestrator; the orchestrator names its vessel by reference but is BLIND to the realm contents (affiliation by reference, never by ownership), which is what stops the de-lamination from silently re-laminating.
Naming (minted in-heat 260623, gates applied by hand, grep-clean, terminal-exclusivity intact): the module is rbxk_keycloak.sh in a new rbx family — the federation establishment / synthetic-trust test-rig family, pre-earmarked in the proof header as the "rbx_ establishment/accessor units"; its setup/teardown tabtargets take colophons under rbw-qjK (the colophon homing settled below).
The rbx letter doubles as a holding family for other poorly-slotted things until a later mass remint; rby was rejected as it is the yelp/handbook-vocabulary family (rbyk would have been terminal-legal but incoherent).
One bondstone — the keycloak facility, named plainly because a test-scaffold bondstone wants the most cold-probe-able word, not an asterism word; the setup/teardown verbs are toothing on it, settled plainly at module-cut time.

RESOLVED (260622, operator decision — see the config-model memo's "Fork resolution" section): both forks settled.
Fork one (test-manor topology) — MULTIPLICITY IS THE GOAL: the test manor stands up the real Entra foedus (interactive) and a Keycloak test foedus (programmatic) side by side, each its own pool; per-run-switching is rejected, a dedicated second org is noted-unneeded.
Fork two (programmatic JWKS source) — a single UPLOADED field: the local Keycloak crucible is unreachable from Google so its public JWKS must be uploaded regardless; the token behind it may be Keycloak-minted (preferred) or self-signed (fallback), a choice below the regime; issuer-discovered is not modeled (re-cut named if a publicly-hosted automated IdP ever enters).

Minted in-heat (260623): the mechanism discriminator quoin and its value words, and the two new RBS0 subdoc acronyms this model calls for, are minted in-heat applying the gates by hand (grep gate, terminal-exclusivity, family coherence).

Detail and reasoning trail: the federation-config-model memo (Memos/memo-20260618-Bf-federation-config-model.md).

## Idea — foedus, the configured-federation noun (adopted)

Foedus is the adopted civic noun for the configured federation (operator decision 260623): the RBS0 rbtf_ federation-civics quoin the spec-first spine unit homes — the rbtf_foedus quoin is not yet built.
It is a folio, not a colophon: a named foedus is data passed as a parameter to operations homed by actor — the payor founds and dissolves one (affiance/jilt), the governor affiliates a depot with one.
The noun denotes the configured shape — a vendor-agnostic trust core plus an acquisition-mechanism gate, vendor identity deliberately not a field of it — not the raw pool-plus-provider-plus-mapping mechanics.
Why it is the keystone (the model the governor-role and health-assurer ideas below lean on): the healthy payor-governor model collapses to four speakable lines — the payor founds one foedus (affiance, org-scope); the payor sanctions a set of them as eligible (the bound); the governor affiliates a depot with a sanctioned foedus (depot-scope); a depot draws its citizens from the foedus it is affiliated with.
The noun-selection deliberation (foedus vs concordat/covenant, the etymology, the verb-register settlement) is drained to the heat-memories memo.

## Idea — payor federation-setup guides (one per vendor)

Tension: the IdP-side preconditions — standing up the external OIDC tenant and app registration, then authoring the regime from its values — are foreign-console human work with no operator guide today.
Affiance proves only the Google-side, autonomous half of founding; the human precondition half has no home.

Under the conviction above, vendor work is wholly guide-plus-values, so this is not one guide but a guide per vendor.
What unifies them is a contract rather than a shared body: each per-vendor guide must yield the vendor-agnostic core facts the regime reads, so the guides differ only in the console they walk and converge on the same handful of values.
A guide, not a workbench command — the work is in a foreign console and cannot be driven by an API token, exactly as manor establishment is a guide for the same reason.

Current lean: a guide-family procedure under the payor-guide family, with Entra first because it is the live IdP; further vendors are further guides under the same contract, no schema or code change.

Source: the federation spike found the only console work was identity-provider-side; everything Google-side was payor-token REST.

## Idea — freeholds: the three live residuals

A freehold is a durable, deliberately-kept test installation reused day-to-day, set against the ephemeral create→destroy lifecycle fixture (the freehold/leasehold contrast).
It cross-cuts depot and foedus: a muniment binds a foedus principal to a depot mantle, so the standing-citizen roster is the join between them (the manor-wide roll is the foedus view, the per-polity slice the depot view).
₣BZ already built the depot-freehold; quota-flatness rides affiance's refuse-and-rotate, not undelete (the full reconciliation is drained to the heat-memories memo).
₣Bf's three live residuals:
(1) the foedus-freehold-VERIFY fixture — the un-built durable-reuse green target, reserved suite word parley;
(2) the multi-citizen roster slice — a 2nd standing subject, via the headless/degenerate IdP the spine builds or the now-built admission verbs;
(3) the terrier permanent founding-home — below.
Release-cadence refresh: when the quota-touching lifecycle runs (say at releases) it also refreshes the freehold — jilt then re-establish, ordered after the lifecycle's own create→jilt passes so cleanup is proven on a throwaway before it touches the durable pool.

## Idea — the terrier's permanent founding-home

Founding-home RESOLVED (terminal Fable review 260622): OPTION B — Manor-founding provisions the terrier, NOT affiance (affiance touches the terrier nowhere; the bucket name is foedus-independent and its IAM binds the depot-born governor mantle SA, not a foedus principal).
Operator cinched the shape: the terrier-build plus all scriptable manor-setup go into ONE idempotent post-payor-guide manor-setup finisher tabtarget, retiring the rbw-dt/dT scaffold (the option-fork deliberation and the ₣BZ-conflation history are drained to the heat-memories memo).
Live ₣Bf residuals: which B-variant exactly (a standing op vs enlarging a not-yet-existing manor-establish ceremony — lean: standing op); where the per-polity folder step lands (grain says depot-levy, but the folder is in the payor-project bucket so it needs a payor-credentialed actor); and the per-Manor-vs-per-foedus cardinality (default one-per-Manor; per-foedus is operator-owned design).
Settled regardless (RBS0-grounded): the bucket lives in the payor project; the per-polity managed folder is depot-grain; manor establishment's project/OAuth half is manual Console, so any home is for the scriptable remainder.
Cross-note: the caged-federation establishment is another founding-time gesture a synthetic test manor stands up, so this "what founding ensures" shape has one more sibling to weigh — a coupling, not a widening.

## Idea — federation testing collapses onto the real verbs

Tension: ₣BZ's interim terrier proof rides two test-rig tabtargets squatting in the depot family — an interim scaffold (rbw-dt) and an atomicity proof (rbw-dT) — because theurge invokes only tabtargets and qualify rejects an unenrolled one, so a fixture's every step must be an enrolled, operator-visible tabtarget.
The two faked-don proofs (admission, freehold) were already cut in ₣BZ as spent scaffolding; the honest terrier-atomicity survivor still carries this shape.

Current lean: the authentic federation fixture drives the REAL verbs — brevet/unseat/rehearse against a real terrier provisioned by whatever founding ceremony the founding-home idea above settles — not synthetic payor-direct cores.
That dissolves both interim tabtargets: the founding ceremony provisions the terrier so the scaffold retires (as that idea already commits), and the idempotent real verbs hit engross's 412 and expunge's 404 paths while rehearse asserts, so the proof's round-trip falls out as a consequence — and coverage deepens to the real don path and the folder-scoped IAM actually enforcing, neither of which the payor-direct proof touches.
The fixture then invokes only verbs that exist for real operator reasons, so no strange test-rig tabtarget survives and the sole obligation is to call the real verbs; the authentic green target's suite word is the operator-elected parley (grep-clean).

Trap to avoid: the lazy collapse — relocating rbw-dt/rbw-dT into an rbw-t* test group — sheds the wrong-family strangeness but keeps the synthetic-payor shallowness and the obligation; reject it.
Honest residual: real verbs swallow the engross/expunge disposition, so asserting the precise 412/404 idempotency may still want a small rbgft data-layer unit test — homed honestly in test-infra, never a production-family verb.

Composes with the founding-home idea and the build spine's attach-via-admission-verbs unit (this is that unit's operator-avowal sibling); not premise-touching and needs no new vocabulary, so it can graduate freely.

## Idea — release suites lost their credential-heal preamble

Tension: skirmish, dogfight, and blockade each led with the freehold-enrobe fixture as a keyfile credential-heal — re-mantling the governor and defrock/re-enrobing retriever and director so the real fixture found fresh service-account keys against a standing operator-levied depot.
The ₣BZ estate demolition removed freehold-enrobe whole (it minted keys, already dead on the no-keys org) and stripped its leader from those three suites, which now carry no credential-readiness step.

Current lean: the federation replacement is not a re-enrobe dance but a standing-freehold avow-plus-don — open a sitting and confirm the donnable mantles — so the three suites are one more consumer of whatever test-rig credential step the config-model forks and the avowal ideas above settle, not separate machinery.
Until that lands skirmish/dogfight/blockade are credential-incomplete; they were already non-functional on the no-keys org, so this surfaces an existing gap rather than regressing a working state.

Source: the ₣BZ estate-demolition pace and its M7 record in the pace-design memo.

## Idea — the governor's role in federation

Frame under consideration: the payor as the IT department — founds federations and bounds what is permissible — and governors as more-trusted regional stewards who operate within those bounds, still subordinate to the payor's citizen-list and federation-set choices.

Two sub-questions, opposite verdicts on one scope test (creation is org-scoped; selection is depot-scoped):

- Configure or found a federation — declined.
  Founding trust is org-wide: a new provider is a root of trust over every depot under the manor.
  That belongs to the org-owner (payor), not a depot-scoped steward.
  This one is settled: federation founding is a payor job.

- Select which federation a depot is affiliated with — open, and this is the healthy model to aim for.
  Stated in the keystone noun above: the payor founds the configured federations and sanctions which are eligible; the governor affiliates its own depot with one of the sanctioned ones.
  Selection among already-founded, payor-sanctioned trusts affects only the one depot, so it respects the cinch that cross-depot administration does not exist, and it mirrors admission — the governor already decides who is admitted; this adds which trust the depot draws from.
  Depends on the federation-regime-family rework (drained as superseded for the test-bed; it survives only for this premise-gated feature).

The bound — the sanctioned set: which configured federations are eligible for affiliation is a payor-controlled, manor-level artifact, an eligibility the payor sets and governors only read.
This is the IT-department catalog of approved trusts.
Guard: a degenerate or test federation is never eligible for a production depot, so a governor cannot repoint production at a weak or caged pool.

The load-bearing distinction for how the hierarchy is enforced is authentication versus authorization:
- Authentication is the crypto layer, and it already exists — the workforce provider holds the identity provider's public keys (JWKS) while the identity provider holds the private signing key.
  A token is trusted iff signed by that private key; this answers only whether the token is genuinely from the trusted identity provider.
- Authorization is where the payor-governor hierarchy lives, and it is IAM, not the signing key — may this governor admit this principal, and may this governor affiliate this depot with this federation, are IAM questions.
  Founding a pool is an org-level permission the payor holds and a governor structurally lacks; bounding a governor's affiliation to the sanctioned set is an IAM-condition or a provider attribute-condition.

A bespoke payor keypair issuing signed grants would re-introduce a durable private secret — against the zero-keys premise — and duplicate what IAM already enforces; flagged as the path to avoid.
The exact GCP condition mechanism that bounds affiliation to the sanctioned set wants verification before this graduates.

## Idea — payor as the org's health-assurer, and how to represent its authority

Tension: the operator's model is the payor as the IT department — standing veto / health-assurance authority over governors to keep the org healthy, beyond today's narrow powers (brevet the first governor, recover a governorless depot, and — from the governor's-role idea above — bound the sanctioned federation set). How should that standing authority be represented?

Ruled out (firmly): the payor as a federation identity. The payor is the bootstrap that founds the federation — you cannot federate-authenticate into a workforce pool that does not yet exist — and it is the recovery root of last resort: if the pool or the IdP breaks, the OAuth payor must still get in and fix it. An administrator-of-last-resort that depends on the system it administers cannot recover that system, so the payor's OAuth-rootedness, outside the citizenry, is load-bearing, not a gap to close.

The fork (both sides seen):

- Payor wields the IT-department powers directly, via OAuth. Simplest; no new role; fits if org-health administration is rare and founding-adjacent. The default lean.
- A new federated role above governor — a steward-of-the-estate stature, below the payor — carries routine org-health administration as a citizen, so the OAuth root stays last-resort. Earns its existence only if such routine super-governor work actually emerges; defer the mint (the load-bearing-complexity test), do not pre-build it. Not "admin" (overloaded); a fresh civic-asterism word is minted in-heat when the role earns its existence — seneschal (chief steward of a great house, under the lord) is an illustrative register, not a mint.

Composes with the governor's-role idea above (the sanctioned-set bound is the first concrete IT-department power). Premise-touching (the civic hierarchy is a cinched premise), so it stays a parked operator-owned idea until the operator takes it up.

## Idea — headless avowal (restrung in from the federation heat)

Restrung in: reconsider whether avowal must gate on a controlling terminal, or whether a headless-but-human-reachable caller can open a sitting by surfacing the device-flow prompt out-of-band and polling to completion.

It keeps the human-present premise — a human still authenticates each sitting — and only relaxes human-present from terminal-present.
Distinct from the programmatic (caged) mechanism, which removes the human entirely.

Touches a cinched federation-heat premise (human-present, and the headless fail-fast membrane), so it is operator-owned and may surface a paddock amendment rather than land purely in code.
Under the configuration-model conviction this sharpens to a mechanism distinction: the programmatic mechanism is test-only and bypasses sign-in by construction, so it is not this fix — production headless stays an interactive-mechanism question with a human still present.

## Foedus-accessor colophon homing (live build guidance)

The minted words avow / sitting / quash are built and swept tree-wide (260624) — the minted-word record is drained to the heat-memories memo.
The live colophon-homing plan this pass settled, carried here as build guidance:
the access family collapses rbw-ac → rbw-a (rbw-aa avow, rbw-ap payor-credential check; the mantle-access probe relocates to the test bucket);
gird moves to the manor family as rbw-mG, so rbw-m is the payor founding trio (affiance / jilt / gird) and rbw-p stays purely governor;
test-only utilities and qualification take rbw-q (the Keycloak foedus setup is rbw-qjK — uppercase, because it affiances and so mutates cloud).
The foedus NOUN stays a folio with no colophon family.
Casing convention across these families: an uppercase second letter marks an op that mutates cloud or critical manor state; lowercase marks read-only or non-mutating.

## Future build order (spine slated; the attach-caged-subject tail slatable now that ₣BZ is complete)

The configuration-model conviction implies a sequence of buildable units; their dependency order is fixed even though none has graduated to a pace.
The front-of-heat design pace settled the de-lamination, the Keycloak-control home, and the build naming: the six-unit spine below is now slated as build paces; with ₣BZ complete its admission verbs have landed, so the attach-caged-subject tail is slatable too.

The spine, in dependency order:
Spec-first, before any code — evolve the federation-regime and affiance specs to the core-plus-mechanism-gate shape and stand up the two new subdocs as contracts; contract before code is project doctrine, so this is a hard predecessor of every code unit below.
Regime mechanism discriminator and mechanism-conditional affiance — depends on the spec unit. (The affiance soft-deleted-pool handling was settled in ₣BZ as refuse-and-rotate, NOT undelete — the "undelete-on-DELETED fix" this once anticipated is moot; quota-flatness rides never-jilting the durable pool.)
Programmatic-trust establishment bash — the rbxk_keycloak orchestrator (charge, ready-poll, fetch-JWKS, call affiance) over the fattened baked realm; depends on the spec unit and the discriminator; stands up the Keycloak test crucible (preferred) or a self-signed caged trust (fallback) and uploads its public JWKS through affiance; the durable replacement for the throwaway manual spike.
Programmatic accessor (token → STS) — depends on the discriminator and on a programmatic trust existing to acquire against; obtains a token from Keycloak's non-interactive RFC 7523 grant (or self-mints, fallback) and exchanges at STS.
Per-vendor setup guide (Entra first) — depends only on the core-facts contract line, so it runs parallel to the three code units.
Attach a caged subject to a test depot via the admission verbs — strictly last, since it consumes ₣BZ's admission-verb surface (now landed, so it is slatable).

Fold and precedence: all these units stay in ₣Bf; none folds into ₣BZ — the configuration-model evolution and the synthetic test rig that rides on it are this heat's work, while ₣BZ owns the admission verbs and the single-federation implementation, and the attach unit consumes those verbs without defining them.
The spec-first unit is a hard predecessor of every code unit, not merely the first among equals — beginning any code before the specs are recast is a discipline breach, not a sequencing choice.

Source material for cutting these paces: the federation-config-model memo (Memos/memo-20260618-Bf-federation-config-model.md) is the important source to consult — it carries the detailed RBS0 subdoc plan (the per-spec must-contain reference, the marker-scheme note, the MCM mint deferrals) alongside the full reasoning, so the spec-first unit reads it on graduation rather than re-deriving the detail; the front-of-heat design pace's two forks gate it.

## Foedus lifecycle — the two-tier test-bed (settled topology + scoped design pass)

The audit's pool-topology gap — the build spine quietly assumed a manor could hold two foedera at once, which nothing slated builds — resolves here, and it opens a strand that gathers four deferred ideas into one design.

Topology (operator decision 260623): single-active-foedus, switched lightly.
One foedus is live in the regime at a time; a test selects which standing foedus it runs against by pointing the regime at it, never by holding both at once.
Because only one is active, the existing singleton-keyed affiance/accessor/brevet work unchanged — the heavy regime-as-family-of-named-instances rework is NOT needed for the test-bed.
That simultaneous rework is needed only by the future governor-selects feature (premise-gated), so it defers with that feature; this supersedes the earlier (260622) "both at once, each its own pool" framing for the test case.

Build-spine consequence (so the spine reads true): the spine operates on single-active.
The affiance arm founds the Keycloak programmatic foedus as the test regime's one pool, and the orchestrator stands it up as that single active foedus, not beside a live Entra one.
The accessor's end-to-end don-and-call still awaits a standing admission (a brevet), which the attach-caged-subject unit owns; the slated spine reaches the federated token, not the full don-and-call.

Two tiers (operator aim): a light tier — "use the current, switch lightly" — reuses durable standing foedera and changes the active one by selection, mapping onto the existing skirmish/dogfight (freehold-already-standing) suites; a nuclear tier — "full redo everything" — jilts and re-affiances the foedera from scratch, mapping onto the existing gauntlet (levies-fresh) suites.

New vocabulary (minted 260624): the switch toothing instate — select the active foedus — plus the sitting-reset it forces (quash the live sitting, re-sign-in against the new foedus); the switch toothing and the pool-integrity check are built in the sibling theurge stream as the foedus-cardinality verbs.
Founding/dissolving stays affiance/jilt; durable-vs-ephemeral is a tenure policy (freehold = affiance-and-never-jilt), not a verb; the nuclear redo is a named ceremony composing jilt+affiance, not an atom.

This strand gathers four ideas above that are really one design: freehold (durable test-bed), the release-suites' lost credential-heal (a standing-freehold readiness step), multiplicity-as-switching (this topology), and federation-testing-collapses-onto-real-verbs (the authentic-verb fixture, suite word parley). It borders the durable manor-setup finisher.

Relationship to the spine: the spine PROVES one foedus's chain (the de-lamination); this strand MANAGES the test-bed (standing foedera, selection, the two tiers) — two different jobs. The lifecycle layer is mostly downstream of the spine (it composes the spine's verbs), with one upstream touch: the regime must support foedus selection (two configs plus a selector), folded into the spec recast or handed to a lifecycle pace.

Next: a focused design pass settles six forks — the tenure policy; the selection mechanism and what the select act does; the new toothing name(s); the light/nuclear tier mapping onto the existing suite ladder; the light-tier readiness check (the release-heal / parley step); and where regime-selection-support lands relative to the spec recast. It produces this strand's own slate, and runs before the audit's build-docket tightenings are finalized.

Foedus-reuse leg (the affiance-centric reframe): the single-active-foedus, switched-among-standing topology stands, and the everyday reuse path switches rather than affiances — a workforce identity pool is founded once per foedus and kept, never jilted, so the everyday path is cap-flat.
The earlier audit-era reuse-path framing inherited an affiance-centric frame the switch decision supersedes; it is reframed around the switch when this design pass's plan is enrolled.
The switch toothing itself is built in the sibling theurge stream; only its SELECTION mechanism stays PARKED for this design pass to settle.

Signing key — SETTLED (operator decision 260623): a stable key, no per-charge rotation, so affiance's idempotent path needs no jwksJson re-sync arm.
The private half lives as an estate field in the never-committed station file (working name RBRS_ESTATE_KEYCLOAK_SECRET); its public half is validated by matching the already-committed JWKS — the canonical reference — so no secret is ever committed or compared across stations.
Home — SETTLED (operator decision 260624, refining the earlier Estate-nucleation framing): do NOT reinstate the RBK station regime (RBRS) for this.
That regime was shelved intact to Tools/rbk/vov_veiled/FUTURE/ for the later podman feature (heat ₣BW) and WILL return then, so reviving it now for a federation secret would either un-shelve podman work early or load a second, unrelated content domain into the very file thinned for lack of content.
Instead a deliberately-minimal, self-labelled BCG stub whose ONLY job is to source-and-verify this one field from the shelved interim station file — tolerating its absence, fail-loud on present-but-mismatched — while the full RBRS regime, its tabtargets, and its RBS0 section stay shelved for ₣BW.
The shelved FUTURE/RBSRS spec gains the Keycloak field as an optional/future entry, the live stub carries its own short contract plus a pointer to it, and the dangling acronym pointer is repointed to the FUTURE/ path (annotated shelved-for-₣BW), not deleted.
The earlier "nucleate a dedicated Estate regime once 2-3 facts accrue" idea therefore defers WITH the podman regime — a single labelled stub now, not a new concept.
The shared-one-key-versus-per-platform topology softens: the committed JWKS is a key SET, so it can hold several public keys if per-platform keys are ever wanted; that stays the lifecycle pass's to settle.

## Federation-MVP re-derivations (the five keyfile-era loose-ends folded in)

A coherent strand restrung in from the sibling MVP junk-drawer: five keyfile-era pre-MVP loose-ends, each re-deriving against the now-stable federation auth surface (avowal / STS / mantle-don) that the spine above builds.
They belong with the federation vertical, not the MVP junk-drawer they came from, for a load-bearing reason: each was cross-heat gated "do not mount until ₣Bf wraps" or split the federation auth bash (rba_auth.sh / rbgp_payor.sh) across clones, and folding them here converts both to in-clone ordering — the federation paces land first and settle that surface, then these re-derive against it in the same clone.

The five tensions, each by purpose:

- Account-state-invalid tolerance — re-derived against the federation auth path.
  The keyfile-era credential-flap tolerance stranded a healthy build when an account-state-invalid analogue surfaced; the touchpoint it smoothed lived in the demolished keyfile token verb.
  Lean: re-derive whether such an analogue can still strand a healthy build once the auth path is federated — the rebuild re-derives against sign-in / STS / mantle-don, not the old token verb — and tolerate-or-retire on that finding.

- OAuth terminal fast-fail by call context.
  A deterministically-terminal credential failure should stop burning the SA-propagation retry budget; the fail-fast disambiguates terminal-vs-transient on the call context rather than retrying blind.
  Lean: fast-fail the terminal case, distinguished by where the call sits in the federated flow.

- Build-bucket scrub plus the adjacent capability-spec check (lives in the sibling MVP stream).
  Verify-and-retire the GCS build bucket superseded by pouch-context delivery, plus the adjacent capability-spec drift.
  Carried here as a coupling note only — this pace stays in the sibling MVP-cleanup stream; it is named here because it sits in the same rbgp_payor.sh bash neighbourhood the federation arms settle first.

- Clean-tree gate rationale parameter (lives in the sibling MVP stream).
  Give each clean-tree gate call site a locale-specific rationale carried as a new gate parameter, BUG staying kit-agnostic.
  Carried here as a coupling note only — this pace stays in the sibling MVP-cleanup stream; named here because the gate sits in the federation auth bash.

- Payor-install manor-completion next-step.
  The payor-install leaf verb hard-codes the depot-levy next-step; with the manor-setup finisher (the terrier founding-home idea above) it should instead point the operator at finishing the manor.
  Lean: re-aim the install tail's next-step prose at the manor-finisher op, not the depot levy.

In-clone ordering notes: the first two (account-state tolerance, OAuth fast-fail) share the token-capture touchpoint — see the Coupling to watch note below — so mount them adjacent or fold them.
The re-derivations touching rbgp_payor.sh (the heat's busiest bash file) order after the federation-code arms settle that file first.
Keep this band shape-shaped: lean lines, no running log.

## Coupling to watch

The account-state-invalid tolerance and the OAuth terminal fast-fail both touch the federation auth path's token-capture point;
the fast-fail's own docket asks whether it is the same landing as the account-state-tolerance touchpoint,
so mount those two adjacent or fold them together.
The touchpoint is now whatever survives in the federated path (sign-in / STS / mantle-don), not the demolished keyfile token verb — a thing the account-state-rebuild pace must itself first re-derive, so the adjacency is an in-clone ordering note for this stream, not a fixed line.

## Chaining-fact constraint — the standing invariant the re-derivations respect

The federation-MVP re-derivations brush against the cinched chaining-fact discipline: the build/capture verbs (chain HEADS) only ever WRITE a fact; the three durable-leak LINKS (feoff, anoint, yoke) read one chained value and write one durable config field; the read consumers resolve the same express-or-chain fact but write no durable config.
The standing constraint any new clean-tree gate or fact-reading touch must honour is no-relay (the express-or-chain resolver is depth-1 and terminally consumed, never forwarded), the named chaining band (BUBC_band_chain on a broken resolve), and furnish-what-you-LOAD (a CLI whose source closure reaches a fact caller must source the fact module).
This heat carries the discipline as a constraint, not as a work-strand: the durable-leak verb implementations live in the sibling MVP-cleanup stream, and the named-band theurge fixture that asserts the band lives in the sibling theurge stream.
The full discipline — the HEADS/LINKS split, the furnish-vs-coverage two-net requirement, the theurge homing — is the deep shape behind the sibling MVP stream's two durable-leak spec paces and is homed in that stream's paddock.

## Sources

The office-federation heat ₣BZ is the parent; the still-live federation ideas here are its deliberate deferrals.
Federation mechanism and the identity-provider-side console finding: the federation-legs spike findings memo.
The pace-design and divergence record for the parent heat: its pace-design memo and its movement-4 review-findings memo.
Degenerate-test-federation mechanism, sources, and the can-and-cannot-prove boundary: the degenerate-federation test-personas memo.
Workforce-pool quota and soft-delete constraints (the freehold idea's load-bearing facts): Memos/memo-20260617-BZ-workforce-pool-constraints.md.
Federation configuration model — the vendor-agnostic-core plus mechanism-gate conviction, full reasoning: Memos/memo-20260618-Bf-federation-config-model.md.
Spec homes to evolve when this lands: RBSRF (RBSRF-RegimeFederation.adoc) and RBSMA (RBSMA-manor_affiance.adoc).
Dispositioned deliberation drained from this paddock — the ₣BZ-lineage gating, the released Fable gating, the vocabulary-remint dingleberry doctrine, the theurge-test-consolidation audit dispositions, the landed five-suite rename record, and (260625) the foedus noun-selection, the multiplicity-goal framing, the degenerate-test-federation, the freehold reconciliation, the terrier option-fork, the foedus-accessor minted words, and the one-click verification_uri_complete decline: Memos/memo-20260623-Bf-heat-memories.md.

Partition provenance (the cross-heat split that reshaped this heat into the federation-build stream): the five federation-MVP re-derivations were restrung in from the sibling MVP heat — their original junk-drawer home keeps the git history of their dockets; the theurge-crate refactor, the zipper-colophon trio, and the cosmology spec drained OUT to the sibling theurge stream, where the drained cosmology pace supersedes that stream's native cosmology pace.
The full how-we-got-there record is the cross-heat split-study provenance memo.