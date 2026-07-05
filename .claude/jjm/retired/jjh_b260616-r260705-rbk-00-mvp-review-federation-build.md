# Heat Trophy: rbk-00-mvp-review-federation-build

**Firemark:** ₣Bf
**Created:** 260616
**Retired:** 260705
**Status:** retired

## Paddock

## Charter — the federation-build stream

This heat is the federation-BUILD stream of the MVP federation work.
It owns the federation/manor region of RBS0-SpecTop and the federation entries of the acronym registry (claude-rbk-acronyms.md), and it carries the federation idea-catalog of still-live strands deliberately deferred from ₣BZ.

The spine is slated: the spec-first recast, the mechanism-gated affiance arm, the Keycloak orchestrator, the programmatic accessor, and the per-vendor setup guide.
The attach-caged-subject tail consumes ₣BZ's admission verbs; with ₣BZ complete those verbs have landed, so the tail is slatable too rather than held.
Five federation-MVP re-derivations (the keyfile-era loose-ends restrung in from the sibling MVP heat) are folded in below, re-deriving against the now-stable federation auth surface this spine builds.

Posture (cinched for this heat): premise-touching ideas are operator-owned — any idea that would amend a cinched ₣BZ premise stays a parked idea until the operator takes it up.
Keep this catalog shape-shaped — each idea a named tension plus a current lean, never a running discussion log.
Genuinely thin or orthogonal ideas belong in itches or the horizon roadmap, not here; this paddock is for ideas worth deliberating with heat context.

## Model — the one-pool identity substrate (operator decision 260630; GOVERNS)

This section is the governing federation-identity model.
It supersedes the per-foedus-pool framing the heat carried through 260623–260627; that reversal record and its reasoning live in git history and the heat-memories memo, not here.

The model, in five lines:
- ONE manor pool, manor lifecycle — founded once as a manor-setup act (the scriptable manor-finisher, extended, or a sibling founding step beside it), never by affiance/jilt. Pool lifecycle and foedus lifecycle are distinct.
- A foedus IS a PROVIDER under that one pool. affiance creates the provider (from the foedus's issuer / client-id / attribute-mapping / oidc); jilt deletes the provider.
- A Manor lists its foedera by providers.list under its one pool; the foedus↔provider link rides the provider id/displayName.
- Membership is per-individual on the stable pool — principal://…/subject/{S} kept, but S is an IdP-stable canonical claim. Normalize S across co-trusted PRODUCTION IdPs (the multi-real-IdP future); namespace-disjoint the synthetic TEST provider's subjects so a test token can never satisfy a production binding. That namespacing is LOAD-BEARING — the price of co-tenancy, carrying the isolation a separate test pool would have given for free.
- Substrate-only scope: this fixes the identity substrate and UNBLOCKS, but does not build, the governor-selects / sanctioned-set authz model (the "governor's role in federation" idea below, the deferred premise-gated feature).

Why (the IT-maintainability lens + sole-operator simplicity): a workforce pool is the org-stable identity container; IdP variety belongs in PROVIDERS under it, not in per-IdP pools. One pool with providers is how GCP WIF is meant to be run, and one pool for everything — the synthetic test rig included — is the simplest shape; the co-tenancy impersonation risk is closed by the subject-namespacing rule above.

Contract-first (the heat's own hard rule): the spec re-cut leads, before any code — RBSRF (pool→manor-level home, foedus≡provider), RBSMA (affiance→add-provider), RBSFD (descry verdicts), and the RBS0 federation civics. No affiance/jilt/canvass/terrier code before the specs land.

Code-re-cut ownership (integrity audit 260701): the regime-buildout pace owns the RBRW module AND, atomic in the same landing, the mechanical re-point of every stripped-field consumer — the rbof descry/canvass file-parses, the rba sitting-file and STS-audience composition, the rbgp read sites (name-only; semantics stay with the composer and verb re-cut paces), and the theurge patrol poison const (one line, safety-critical: left inert, the lifecycle fixture's jilt would aim at the REAL manor pool).
The verb re-cut pace owns affiance, jilt, AND descry's verdict re-cut to the re-cut RBSFD.
The foedus-lifecycle fixture's provider-grain re-cut stays theurge-stream-owned (₣Bl), delivered behind that stream's one-pool barrier.

Still valid (the change touches pool + membership only): the vendor-agnostic-core conviction, avow/sitting/quash, the Keycloak orchestrator (now provisioning a PROVIDER, not a pool), the accessor STS layer (already provider-aware — audience and sitting-cache key on pool AND provider), and the vocabulary census all stand.

New sub-works this substrate adds: the terrier muniment provider-dimension + on-disk migration; RBPC's freehold-subject goes multi; the subject-namespacing discipline.

Source for cutting the re-cut paces: the four 260630 spec studies (founding, membership, terrier, test-bed/accessor) carry the per-surface change-lists. Pace triage — re-cut the model-touching paces, keep the orthogonal loose-ends — is the next groom step.

## Landed structure (stands under the Model)

These structural facts from the wrapped RBSRR / RBSFI / RBSFD predate 260630 and still hold:
- Library: one rbrf.env per standing foedus, each in its own moorings subdirectory bearing the rbef_ sprue — rbef_entrada (interactive Entra), rbef_keycloak (programmatic test); the subdirectory name is the foedus identity and the instate/descry folio.
- Selector: RBRR_ACTIVE_FOEDUS names the active foedus, a cross-domain reference whose value the rbrf library owns.
- RBRF_COGNOMEN evicted whole — the subdirectory name plus the rbef_ sprue carry the identity.
- instate (RBSFI): a single-field rewrite of the selector — own band, no self-gate, operator commits.
- descry (RBSFD): a read of a named foedus's health — own band, no durable write. The Model re-cuts its verdict vocabulary: a foedus is a provider, so descry reads provider presence, not pool absence / soft-delete.

## Federation vocabulary — the minted census (chosen; the recast does not re-derive these)

The federation naming is substantially settled; this census is the single prominent home, so a mount agent reads "chosen" before reaching for the mint.
Built-word deliberation stays drained to the heat-memories memo — what lives here is the standing roster and what remains owed: shape, not deliberation.

Chosen (do NOT re-mint — re-deriving these is the failure this census guards against):
- foedus — the configured federation, the noun affiance founds; this recast seats its rbtf_ civic quoin, the word itself is adopted.
- avow / sitting / quash — the human device-flow sign-in, the live window it opens, and the test-only sitting-clear.
- instate / descry — switch which foedus is active, and read a foedus's health; minted, owned by the theurge stream where the build is slated (not yet landed).
- interactive / programmatic — the mechanism gate's two values, KEPT as the plain words (260626): test-scoped — read by affiance and the accessor, written only by the test establishment, never a tabtarget argument, so the asterism's weight does not apply.
  Serialized as rbnfe_interactive / rbnfe_programmatic — the rbnfe_ federation enum sprue, derived from the rbnve_ (vessel) / rbnne_ (nameplate) scheme, not a free mint.
- RBRF_MECHANISM — the regime field holding the gate value (260626), modeled on RBRV_VESSEL_MODE.
- RBSFE / RBSFA — the two operation-subdoc acronyms (260626): RBSFE the programmatic establishment, RBSFA the programmatic accessor; fresh RBSF tails (only RBSFH was taken), distinct-over-mnemonic per operator ruling. The operation-quoin verb stays descriptive — the civic verb defers to the build pace.
- canvass — read-only enumeration of every foedus by interrogating the Manor (providers.list under the one pool), the descry sibling (descry inspects one foedus, canvass lists all); chosen 260626 via the Lapidary gates (grep-clean, fair-faced, civic register, sibling-initial c safe against descry-d/instate-i). Colophon rbw-jc, quoin rbtf_canvass; build + test slated as placeholders in the theurge stream (foedus-cardinality family).

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
The core is always present — org, provider, client-id, attribute-mapping, issuer or JWKS source (the workforce-pool id and its session-duration are manor-level under the Model, not per-foedus fields).
The interactive arm carries the device-authorization and token endpoints and the device scope; the programmatic arm carries an uploaded public JWKS (the caged case); vendor identity is not a regime field at all.

Scope of this model: it governs one foedus's shape (vendor-agnostic core plus mechanism gate), orthogonal to the pool topology the Model above settles.

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

Minted in-heat (260623): the mechanism discriminator quoin and its value words, and the two new RBS0 subdoc acronyms this model calls for, are minted in-heat applying the gates by hand (grep gate, terminal-exclusivity, family coherence).

Detail and reasoning trail: the federation-config-model memo (Memos/memo-20260618-Bf-federation-config-model.md).

## Idea — foedus, the configured-federation noun (adopted)

Foedus is the adopted civic noun for the configured federation (operator decision 260623): the RBS0 rbtf_ federation-civics quoin the spec-first spine unit homes — the rbtf_foedus quoin is not yet built.
It is a folio, not a colophon: a named foedus is data passed as a parameter to operations homed by actor — the payor founds and dissolves one (affiance/jilt), the governor affiliates a depot with one.
The noun denotes the configured shape — a vendor-agnostic trust core plus an acquisition-mechanism gate, vendor identity deliberately not a field of it — not the raw provider-plus-mapping mechanics.
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
₣BZ already built the depot-freehold; the full reconciliation is drained to the heat-memories memo.
₣Bf's three live residuals:
(1) the foedus-freehold-VERIFY fixture — the un-built durable-reuse green target, reserved suite word parley;
(2) the multi-citizen roster slice — a 2nd standing subject, via the headless/degenerate IdP the spine builds or the now-built admission verbs;
(3) the terrier permanent founding-home — below.
Release-cadence refresh: when the quota-touching lifecycle runs (say at releases) it also refreshes the freehold — jilt then re-establish, ordered after the lifecycle's own create→jilt passes so cleanup is proven on a throwaway before it touches the standing trust.

## Idea — the terrier's permanent founding-home

Founding-home RESOLVED (terminal Fable review 260622): OPTION B — Manor-founding provisions the terrier, NOT affiance (affiance touches the terrier nowhere; the bucket name is foedus-independent and its IAM binds the depot-born governor mantle SA, not a foedus principal).
Operator cinched the shape: the terrier-build plus all scriptable manor-setup go into ONE idempotent post-payor-guide manor-setup finisher tabtarget, retiring the rbw-dt/dT scaffold (the option-fork deliberation and the ₣BZ-conflation history are drained to the heat-memories memo).
Live ₣Bf residuals: which B-variant exactly (a standing op vs enlarging a not-yet-existing manor-establish ceremony — lean: standing op); where the per-polity folder step lands (grain says depot-levy, but the folder is in the payor-project bucket so it needs a payor-credentialed actor).
Settled regardless (RBS0-grounded): the bucket lives in the payor project; the per-polity managed folder is depot-grain; manor establishment's project/OAuth half is manual Console, so any home is for the scriptable remainder.
Model touch (260630): the manor pool's one-time founding now wants a home here too (the finisher, or a sibling founding step), and the muniment gains a provider dimension — see the Model.
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

Escheat residual (260702): the terrier-hygiene sweep (RBSME, rbw-mE) landed live-proven on the dead-schema arm —
a real provider-blind stray swept from the standing terrier, verify re-survey green, second run idempotent-clean, rehearse roll all-live.
The orphan-slice arm (dead-depot liveness verdict + dead-folder purge) awaits the next real depot churn,
per the instaurate billing-arm gauntlet-clause precedent —
never a synthetic payor-direct writer; that shape stays retired (operator ruling at the exercise).
The rbgft classification unit-test residual above now also covers the escheat survey verdicts and its band, still theurge-stream-homed.

## Idea — release suites lost their credential-heal preamble

Tension: skirmish, dogfight, and blockade each led with the freehold-enrobe fixture as a keyfile credential-heal — re-mantling the governor and defrock/re-enrobing retriever and director so the real fixture found fresh service-account keys against a standing operator-levied depot.
The ₣BZ estate demolition removed freehold-enrobe whole (it minted keys, already dead on the no-keys org) and stripped its leader from those three suites, which now carry no credential-readiness step.

Current lean: the federation replacement is not a re-enrobe dance but a standing-freehold avow-plus-don — open a sitting and confirm the donnable mantles — so the three suites are one more consumer of whatever test-rig credential step the config-model forks and the avowal ideas above settle, not separate machinery.
Until that lands skirmish/dogfight/blockade are credential-incomplete; they were already non-functional on the no-keys org, so this surfaces an existing gap rather than regressing a working state.

Source: the ₣BZ estate-demolition pace and its M7 record in the pace-design memo.

## Idea — the governor's role in federation

Frame under consideration: the payor as the IT department — founds federations and bounds what is permissible — and governors as more-trusted regional stewards who operate within those bounds, still subordinate to the payor's citizen-list and federation-set choices.
This is the premise-gated feature the 260630 Model UNBLOCKS but does not build (governor-selects); it stays parked and operator-owned.

Two sub-questions, opposite verdicts on one scope test (creation is org-scoped; selection is depot-scoped):

- Configure or found a federation — declined.
  Founding trust is org-wide: a new provider is a root of trust over every depot under the manor.
  That belongs to the org-owner (payor), not a depot-scoped steward.
  This one is settled: federation founding is a payor job.

- Select which federation a depot is affiliated with — open, and this is the healthy model to aim for.
  Stated in the keystone noun above: the payor founds the configured federations and sanctions which are eligible; the governor affiliates its own depot with one of the sanctioned ones.
  Selection among already-founded, payor-sanctioned trusts affects only the one depot, so it respects the cinch that cross-depot administration does not exist, and it mirrors admission — the governor already decides who is admitted; this adds which trust the depot draws from.

The bound — the sanctioned set: which configured federations are eligible for affiliation is a payor-controlled, manor-level artifact, an eligibility the payor sets and governors only read.
This is the IT-department catalog of approved trusts.
Guard: a degenerate or test federation is never eligible for a production depot, so a governor cannot repoint production at a weak or caged provider.

The load-bearing distinction for how the hierarchy is enforced is authentication versus authorization:
- Authentication is the crypto layer, and it already exists — the workforce provider holds the identity provider's public keys (JWKS) while the identity provider holds the private signing key.
  A token is trusted iff signed by that private key; this answers only whether the token is genuinely from the trusted identity provider.
- Authorization is where the payor-governor hierarchy lives, and it is IAM, not the signing key — may this governor admit this principal, and may this governor affiliate this depot with this federation, are IAM questions.
  Founding the pool is an org-level permission the payor holds and a governor structurally lacks; bounding a governor's affiliation to the sanctioned set is an IAM-condition or a provider attribute-condition.

A bespoke payor keypair issuing signed grants would re-introduce a durable private secret — against the zero-keys premise — and duplicate what IAM already enforces; flagged as the path to avoid.
The exact GCP condition mechanism that bounds affiliation to the sanctioned set wants verification before this graduates.

## Idea — payor as the org's health-assurer, and how to represent its authority

Tension: the operator's model is the payor as the IT department — standing veto / health-assurance authority over governors to keep the org healthy, beyond today's narrow powers (brevet the first governor, recover a governorless depot, and — from the governor's-role idea above — bound the sanctioned federation set). How should that standing authority be represented?

Ruled out (firmly): the payor as a federation identity. The payor is the bootstrap that founds the federation — you cannot federate-authenticate into a workforce pool that does not yet exist — and it is the recovery root of last resort: if the pool or the IdP breaks, the OAuth payor must still get in and fix it. An administrator-of-last-resort that depends on the system it administers cannot recover that system, so the payor's OAuth-rootedness, outside the citizenry, is load-bearing, not a gap to close.

The fork (both sides seen):

- Payor wields the IT-department powers directly, via OAuth. Simplest; no new role; fits if org-health administration is rare and founding-adjacent. The default lean.
- A new federated role above governor — a steward-of-the-estate stature, below the payor — carries routine org-health administration as a citizen, so the OAuth root stays last-resort. Earns its existence only if such routine super-governor work actually emerges; defer the mint (the load-bearing-complexity test), do not pre-build it. Not "admin" (overloaded); a fresh civic-asterism word is minted in-heat when the role earns its existence — seneschal (chief steward of a great house, under the lord) is an illustrative register, not a mint.

Composes with the governor's-role idea above (the sanctioned-set bound is the first concrete IT-department power). Premise-touching (the civic hierarchy is a cinched premise), so it stays a parked operator-owned idea until the operator takes it up.

## Settled — headless avowal (the tty gate retired, 260701)

The terminal gate on avow is retired (operator ruling 260701): human presence is enforced by the IdP sign-in itself, never by terminal possession, so the gate defended a mechanism that needed no defense at production strength.
The device-flow prompt now rides the shared progress stream as a yawp — console, log, and any watching relay alike — so a terminal operator and a headless-but-human-reachable relay complete the same sign-in; no opt-in, no new config, no prompt file.
The user code rides the stream deliberately: RFC 8628 designs it for open display (possession grants nothing without the human's own IdP sign-in; a substituted sign-in cannot pass admission).
Accepted cost: a truly unattended cache-miss polls to the bounded device-code expiry (~15 min) and dies loud instead of failing instantly — unattended operation belongs to the programmatic mechanism, never this path.
The human-present premise stands untouched; the "headless fail-fast membrane" language is retired with the gate.
Spec home: RBS0 rbtf_avow (the definition site carries the rationale).

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
Spec-first, before any code — evolve the federation-regime and affiance specs to the core-plus-mechanism-gate shape, fold in the 260630 Model's pool→provider re-cut, and stand up the two new subdocs as contracts; contract before code is project doctrine, so this is a hard predecessor of every code unit below.
Regime mechanism discriminator and mechanism-conditional affiance — depends on the spec unit.
Programmatic-trust establishment bash — the rbxk_keycloak orchestrator (charge, ready-poll, fetch-JWKS, call affiance) over the fattened baked realm; depends on the spec unit and the discriminator; stands up the Keycloak test crucible (preferred) or a self-signed caged trust (fallback) and uploads its public JWKS through affiance; the durable replacement for the throwaway manual spike.
Programmatic accessor (token → STS) — depends on the discriminator and on a programmatic trust existing to acquire against; obtains a token from the IdP's non-interactive RFC 7523 grant against the RBRF programmatic self-supply fields and exchanges at STS.
No self-mint fallback: the realm signing key is ephemeral and held by no caller, so a self-mint would mean re-establishing the trust, not falling back — the grant is the one programmatic path (RBSFA/RBSFK).
Per-vendor setup guide (Entra first) — depends only on the core-facts contract line, so it runs parallel to the three code units.
Attach a caged subject to a test depot via the admission verbs — strictly last, since it consumes ₣BZ's admission-verb surface (now landed, so it is slatable).

Fold and precedence: all these units stay in ₣Bf; none folds into ₣BZ — the configuration-model evolution and the synthetic test rig that rides on it are this heat's work, while ₣BZ owns the admission verbs and the single-federation implementation, and the attach unit consumes those verbs without defining them.
The spec-first unit is a hard predecessor of every code unit, not merely the first among equals — beginning any code before the specs are recast is a discipline breach, not a sequencing choice.

Source material for cutting these paces: the federation-config-model memo (Memos/memo-20260618-Bf-federation-config-model.md) is the important source to consult — it carries the detailed RBS0 subdoc plan (the per-spec must-contain reference, the marker-scheme note, the MCM mint deferrals) alongside the full reasoning, so the spec-first unit reads it on graduation rather than re-deriving the detail; the 260630 spec studies carry the pool→provider re-cut on top.

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
Workforce-pool quota and soft-delete constraints: Memos/memo-20260617-BZ-workforce-pool-constraints.md.
Federation configuration model — the vendor-agnostic-core plus mechanism-gate conviction, full reasoning: Memos/memo-20260618-Bf-federation-config-model.md.
Spec homes to evolve when this lands: RBSRF (RBSRF-RegimeFederation.adoc), RBSMA (RBSMA-manor_affiance.adoc), RBSFD (RBSFD-foedus_descry.adoc).
Dispositioned deliberation drained from this paddock — the ₣BZ-lineage gating, the released Fable gating, the vocabulary-remint dingleberry doctrine, the theurge-test-consolidation audit dispositions, the landed five-suite rename record, and (260625) the foedus noun-selection, the multiplicity-goal framing, the degenerate-test-federation, the freehold reconciliation, the terrier option-fork, the foedus-accessor minted words, and the one-click verification_uri_complete decline: Memos/memo-20260623-Bf-heat-memories.md.
The 260630 model reversal (per-foedus-pool → one-pool identity substrate) and its four spec studies: to drain into the heat-memories memo when next edited.

Partition provenance (the cross-heat split that reshaped this heat into the federation-build stream): the five federation-MVP re-derivations were restrung in from the sibling MVP heat — their original junk-drawer home keeps the git history of their dockets; the theurge-crate refactor, the zipper-colophon trio, and the cosmology spec drained OUT to the sibling theurge stream, where the drained cosmology pace supersedes that stream's native cosmology pace.
The full how-we-got-there record is the cross-heat split-study provenance memo.

## Paces

### fed-syncs-theurge-zipper-crucible (₢BfAAo) [abandoned]

**[260624-2332] abandoned**

Cross-clone coordination gate for the federation-build stream (its own repo clone).
No code change of its own — the standing sync contract for this stream's writes into ₣Bl-owned files.
Pace order within each heat is the dependency order and is fixed, so it already encodes every in-clone wait; this gate names only the CROSS-clone waits that separate-clone order cannot express.

## Spec of needed change
This stream owns RBS0-SpecTop and the acronym registry and leads on those; its only cross-clone WAITS are on ₣Bl, which owns the zipper trio and the crucible fixture/suite registry:
- Zipper baton (₣Bl → ₣Bf → ₣Bi): ₢BfAAF (manor-finisher, rbw-dXX) and ₢BfAAL (Keycloak facility, rbw-qjK/rbw-qj) — plus ₢BfAAJ's Entra colophon IF that guide mints one — take the baton AFTER ₣Bl's ₢BlAAN and ₢BlAAE have pushed the trio. Sync ₣Bl, edit rbz_zipper.sh, run the theurge build to regenerate the pair, commit the three files as one unit.
- Crucible delta: ₢BfAAF retires two crucible fixtures — a write into ₣Bl's rbtdrc_crucible.rs — and lands only AFTER ₢BlAAC has settled the relocated RBTDRC_FIXTURES / RBTDRC_SUITES home, as a one-shot delta against that relocated home.

## Done when
₢BfAAF and ₢BfAAL synced ₣Bl's zipper and crucible pushes before writing them, with no ₣Bl work overwritten.

## Cinched
Coordination-only, no substantive code.
The zipper baton order is ₣Bl (theurge) then ₣Bf (federation) then ₣Bi (MVP).

## Character
Cross-clone coordination gate; mechanical.

### accessor-vocabulary-pass (₢BfAAS) [complete]

**[260624-0211] complete**

Mint the foedus-accessor toothing/bondstone vocabulary in-heat as one coupled register pass — the sign-in verb (compear → heed), the live sign-in-window noun (assize → a new word), the verb that clears the live window (forces a fresh sign-in), and the active-foedus switch toothing — then sweep the two remints tree-wide.

## Spec of needed change
Pick each word by hand against the gates — grep-clean, no trodden words, terminal-exclusivity, the fair-faced floor for the operator-facing ashlars, one coherent civic register. The four are coupled and picked together: the switch forces the window-clear, the clear verb keys off whatever the window noun becomes, and heed and the window noun are sibling sign-in remints. Settle the open fork on the window noun — keep an operator-facing noun, or demote it to hearting with the error text going plain prose. Mint only the switch NAME here; its selection mechanism stays with the parked foedus-reuse leg.
Then run the eviction sweeps: compear → heed and assize → the new noun across every live site, all forms moving together.

Source: the paddock ideas — the heed/compear remint, the assize rename, the clear-the-live-window verb, and the foedus-lifecycle switch toothing — carry the tensions and current leans.

## Done when
Each word clears the gates and is minted; compear → heed and the assize remint are swept tree-wide; the clear verb and the switch toothing are named; and the new names are incorporated throughout the ₣Bf paddock and every remaining ₣Bf docket.

## Cinched
₣BZ is treated as functionally complete (operator decision 260623): the eviction-sweep gate is lifted — sweep now, do not wait on Fable. Only the switch name is minted here; the switch mechanism and the foedus-reuse leg stay parked. Words are operator-elected in-heat.

## Character
In-heat mint plus tree-wide eviction sweep; design judgment on the words and the window-noun fork; mount with the operator present.

### suite-asterism-rename (₢BfAAR) [complete]

**[260624-1203] complete**

Rename the test suites to the elected military asterism across every surface: fast → reveille, service → picket, crucible → bivouac, complete → echelon. Keepers (gauntlet / skirmish / dogfight / siege / blockade) and parley unchanged.

## Spec of needed change
crucible → bivouac is the forced fix — the suite name shadowed the production crucible runtime noun (the charged sentry + pentacle + bottle), a one-meaning-per-word collision; the other three are the elected coherence renames.
Sweep every home of each name: the suite tabtarget filenames, the RBTDRC_SUITES composition table, the zipper/colophon registry that drives them, the CLAUDE.md suite tables, and the CLI / onboarding text; then regenerate the build-generated context + consts and confirm qualify is green.
Discover the homes by grep, not a baked list.

Source: the naming slate in Memos/memo-20260623-theurge-test-consolidation-audit.md carries the elected words and their grep verification.

## Done when
The four suites are renamed across every surface; the build regenerates clean and qualify passes; and the new suite names are incorporated throughout the ₣Bf paddock and every remaining ₣Bf docket.

## Cinched
The elected words are settled (operator election 260623), not reopened; crucible → bivouac is mandatory; keepers and parley untouched.

## Character
Mechanical multi-surface rename sweep plus regenerate; the one judgment is catching every home of each name.

### heat-integrity-and-coverage-audit (₢BfAAh) [complete]

**[260624-1358] complete**

Mount FRESH — a clean instance, not the long planning chat that produced this heat. Run a workflow-heavy audit that verifies the plan and closes its coverage gaps, producing a verified gap-list and a closure slate.

## Spec of needed change
A read-only audit (mount fresh — the value is fresh eyes on work a long, heavy chat produced and cannot reliably self-check):
- VERIFY: adversarially re-check every slated pace's docket, its dependencies, and the heat ordering for subtle errors, contradictions, stale references, or mis-scopings introduced during planning.
- PACE COVERAGE: map every paddock-defined change to a pace home; propose paces for the uncovered ones — beyond this point, every change has a pace or an explicit deferral/decline.
- SPEC COVERAGE: map every code-changing pace to its RBS0*.adoc spec-sync need; propose the spec-update/synchronisation paces.
- FIXTURE COVERAGE: check that the theurge suites and major fixtures have characterisation homes (the cosmology subdoc and beyond).

Output: a verified gap-list plus a proposed closure slate the operator reviews and enrolls — the test-consolidation audit's shape, turned on the heat itself.

## Done when
The plan is adversarially verified; every paddock change has a pace home or explicit disposition; every code change has a spec-sync pace; the suite/fixture characterisation gaps are mapped; the closure slate is produced for operator review.

## Cinched
MOUNT FRESH — a tired instance cannot reliably audit its own tired work. Read-only / audit-not-mutate (it proposes; the operator enrolls). Workflow-heavy by design.

## Character
Workflow orchestration; the heat's trust-but-verify gate after a heavy planning chat.

### cross-heat-parallelization-split-study (₢BfAAk) [complete]

**[260624-2317] complete**

Run a workflow-driven cross-heat split study: partition the remaining work of ₣Bf, ₣Bi, and ₣Bl into about three parallel streams that can run in separate repo clones with occasional git syncs, and propose the retained-heat set (₣Bl the candidate third stream), the pace transfers, and the paddock restitching that realize it.

## Spec of needed change
Mount FRESH in a clean chat; this is a workflow-heavy planning study, read-only until the operator approves the transfers.
Exclude THIS study pace itself from the partition.

SCOPE — updated 260624 (operator ruling): the inclusion list is ₣Bf, ₣Bi, and ₣Bl ONLY.
₣Bb is OUT — it does not belong; the earlier "coupled ₣Bb" framing is retired.
The one real residue is that ₣Bi's nameplate-hallmark-drive pace couples to a ₣Bb chaining-consolidation pace — honor that as an EXTERNAL dependency to an out-of-scope heat, never pull ₣Bb in.

₣Bl drains into the retained set and is the candidate THIRD-STREAM ANCHOR.
Its single pace (the theurge-cosmology RBS0 subdoc) is pure spec / RBS0 / theurge-crate territory, so a stream that OWNS RBS0 anchors naturally on ₣Bl: prefer relabelling + un-stabling ₣Bl and draining the spec/RBS0/theurge paces of ₣Bf and ₣Bi into it over nominating a fresh third heat.
Restitch items ₣Bl carries: an empty template paddock, a narrow silks, and a stabled state.

Method:
- FOOTPRINT: for each remaining (non-done, non-abandoned) pace of ₣Bf, ₣Bi, ₣Bl, derive its likely file footprint and its hard dependencies (cross-pace, cross-heat) from the docket plus grep — no jjx surface gives this, and the file-touch bitmap covers committed work only, not future paces. Discovery recipe: jjx_coronets {firemark, remaining:true} per heat, then jjx_brief per coronet.
- CONTENTION: build the file-contention graph. The binding constraint is NOT dependency-chain depth but the small set of hot shared spine files every heat writes — RBS0-SpecTop.adoc, the zipper plus its two build-generated files (rbtdgc_consts.rs, claude-rbk-tabtarget-context.md), claude-rbk-acronyms.md, and the theurge crate (Tools/rbk/rbtd/). Identify every remaining pace that writes each.
- PARTITION: propose about three streams whose file territories are as disjoint as possible (the ₣Bl-anchored spec stream is a strong candidate for one), plus a choreography for serializing edits to the unavoidable hot files (who owns RBS0 / the zipper in which window). Honor the hard cross-heat blocks (the ₣Bi account-state and payor-install paces wait on ₣Bf; the ₣Bi nameplate-drive pace's coupling to the out-of-scope ₣Bb consolidation).
- PADDOCK RESTITCH (first-class deliverable, operator-flagged): the partition moves paces between heats, so the paddock prose carrying their shape must move, merge, or be written fresh. For each retained stream-heat, specify which paddock shape moves in (from which source paddock sections), what is written fresh (especially ₣Bl's empty paddock), and what drains to a provenance memo — honoring the no-coronets / no-silks-in-paddock-prose discipline.
- SLATE: propose the retained-heat set, the relabels / alters, the exact pace transfers (jjx_transfer), and the paddock-restitch operations that realize the partition — for operator review.

## Done when
A workflow-verified partition over ₣Bf, ₣Bi, ₣Bl exists: each remaining pace assigned to a stream, the hot-file contention choreography specified, the hard cross-heat blocks honored (including the external ₣Bb coupling), the paddock-restitch plan written, and a concrete proposal (reuse/relabel ₣Bl as stream three, nominate any further heat, transfer the named paces, restitch the paddocks) produced for operator review — nothing nominated, relabelled, or transferred without approval.

## Cinched
MOUNT FRESH in a clean chat (the audit chat that birthed this is saturated, and the operator deploys the workflow there).
Workflow-driven. Read-only / propose-not-mutate — the operator approves the nominate + relabels + transfers.
Scope is ₣Bf + ₣Bi + ₣Bl; ₣Bb is excluded (operator ruling 260624).
The real constraint is file contention on the spine files, not dependency depth — design the partition by file territory, not by chain length.
₣Bl is the candidate third-stream anchor (spec / RBS0 / theurge territory); prefer reusing it over nominating a fresh third heat.
Paddock restitching is a required deliverable, not an afterthought.

## Character
Workflow orchestration; a cross-heat planning study. Motivated by the ₣Bf heat-integrity audit's finding that file contention, not chain depth, is the binding constraint on parallelism.

### paddock-refocus-and-drain (₢BfAAi) [complete]

**[260625-1524] complete**

Drain the dispositioned deliberation from the ₣Bf paddock to the heat-memories memo and refocus the paddock on remaining work — the first use of the drain practice.

## Spec of needed change
Per the paddock's drain direction and Memos/memo-20260623-Bf-heat-memories.md: move the DISPOSITIONED material out to the memo — the ₣BZ lineage and its now-released gating, the dingleberry doctrine (its gate released by treating ₣BZ complete), and any built-or-declined idea — de-staling (not draining) the references that now resolve to existing work. Leave the paddock the shape of remaining work only.
Explicit de-stale targets (surfaced by the heat-integrity audit): the retired verbs compear/assize still in idea bodies (→ avow/sitting), and the superseded colophon earmark rbw-xkX in the Conviction section (→ the live rbw-qjK) — de-stale these specific tokens, not only generic references.

## Done when
The dispositioned material lives in the memories memo as provenance; the paddock holds only live, remaining-work shape; stale ₣BZ-gating and dingleberry references are gone or de-staled; the named stale tokens (compear/assize, rbw-xkX) are de-staled; deferred-but-live ideas remain in the paddock.

## Cinched
Drain only DISPOSITIONED ideas (built / declined / superseded), never deferred-but-live ones. Runs after the renames (which sweep the paddock) and after the integrity audit (whose coverage-mapping informs what is truly dispositioned).

## Character
Paddock hygiene — the experimental drain practice's first run; judgment on what is dispositioned versus still-live.

### federation-spec-first-recast (₢BfAAO) [complete]

**[260626-1029] complete**

Recast the federation specs to the vendor-agnostic-core-plus-mechanism-gate shape, contract-first, before any build code — and add the foedus-selection regime support the switch design leans on.

Evolve RBSRF (RBSRF-RegimeFederation.adoc) and RBSMA (RBSMA-manor_affiance.adoc) so the regime is a vendor-agnostic trust core plus an acquisition-mechanism gate (interactive vs programmatic), and stand up the two new RBS0 subdoc contracts the model calls for.
Mint the mechanism-discriminator quoin, its value words, and the two subdoc acronyms in-heat, applying the selection gates by hand (grep, no trodden words, terminal-exclusivity, family coherence) — the heat lifted the Fable gate for its build vocabulary.

Add the foedus-SELECTION regime support: the single-active-foedus topology needs the regime to hold two (or more) foedus configs plus a selector naming the active one.
This is the regime shape the switch toothing re-points; its SUPPORT lives here in the recast, while the switch's behaviour lives in the foedus-reuse design pace.
Settled selector-home (operator decision 260623).

## Additional spec sites (surfaced by the heat-integrity audit — outside the original RBSRF/RBSMA scope but belonging to this spec-first pace)
Seat the foedus civic quoin in RBS0 — the configured-federation noun the whole model leans on, in the rbtf_ federation-civics category, with a fair-faced first-contact gloss for the cold-probe; it is homed in no spec today.
Make the human-present premise and the three governor/director/retriever authenticate patterns in RBS0-SpecTop mechanism-conditional: the interactive arm keeps the human-avow / TTY / fails-loud rule, the programmatic arm voids it — so the programmatic accessor has a contract to build against (config-model memo).
Assess RBSMA's sibling RBSMJ (manor_jilt) for a mechanism arm: jilt carries interactive behaviour and is the orchestrator's teardown, so the interactive/programmatic split likely reaches it; sync it or record why it stays mechanism-blind.
Register the missing RBSMA (manor_affiance) and RBSMJ (manor_jilt) acronym entries in claude-rbk-acronyms.md — both specs exist on disk but were never catalogued (pre-existing gap; this pace already touches RBSMA/RBSMJ and the acronym registry, so it is the natural home).

Source: the config-model memo (Memos/memo-20260618-Bf-federation-config-model.md) for the subdoc plan and the per-spec must-contain reference; the paddock foedus-lifecycle strand for the selection topology.

## Done when
The two specs voice the core-plus-mechanism-gate model, the two subdoc contracts exist, the regime carries the multi-config + selector shape the switch re-points, the minted vocabulary clears the gates, the foedus quoin is seated, the human-present premise and the three authenticate patterns are mechanism-conditional, jilt's mechanism-arm need is settled, and the RBSMA/RBSMJ acronym entries are registered.

## Cinched
Contract before code (project doctrine): a hard predecessor of every build pace below — no establishment, accessor, orchestrator, or foedus-toothing code starts before the specs are recast.
Selector regime-SUPPORT lives here; switch behaviour lives in the foedus-reuse design.

## Character
Spec authoring plus in-heat mint; judgment-heavy, reads the config-model memo first.

### foedus-reuse-design (₢BfAAT) [complete]

**[260627-1134] complete**

Author the foedus test-bed switching mechanism into contract-first spec and the supporting regime restructure — the committed library of standing foedera, the active-selector, and the atomic toothings.
The design is landed (cinched below); sprue terminology and the normalization dispositions arrive as decided inputs from the attended chat, so this pace does not reopen them.

## Cinched (the design, landed)
Library: one subdirectory per standing foedus (the RBRR vessel-dir pattern), each holding its own rbrf.env; config stored once, no copied active rbrf.env; the subdirectory name is the foedus identity.
Selector: committed, a new RBRR field naming the active foedus subdirectory; production trivially names its single foedus; RBSRR prose gains the cross-domain-reference and the production-rarely-changes notes.
RBRF_COGNOMEN is evicted entirely (the field has no code consumers); the subdirectory name carries the identity; the rbrf_cognomen quoin is retired or redefined in RBS0.
instate: a single param verb taking the foedus subdirectory (omit the arg → fail and list discovered foedera); it writes the RBRR field and leaves it uncommitted for the operator (enchase-adjacent write discipline: one field, no self-gate, operator commits), reusing the feoff/anoint/yoke field-rewrite mechanics; it is NOT a chaining link and uses its own failure band, aligned per the ₣Bi chaining-roles boundary.
descry: a single param verb reading a named foedus's pool health for transient use, no durable write (palpate-adjacent), its own band, not a chaining consumer.
Gating: the authenticate-against-active consumers (avow, the accessor, the federated-access and mantle-access probes) gate on the selector being committed, via a new correctly-formed bug_require_* taking a BCG tinder/kindle constant; instate does not self-gate; a separate later pace migrates the existing bug_require_clean_tree call sites and retires the malformed original.
Fixture composition: reuse-if-valid-else-create lives in the fixture (the freehold-ensure shape), composing the atoms — descry, then reuse-instate or establish-then-instate — never a fat verb; share the ensure-combinator with depot-ensure.
Deferred: canvass and the name-as-folio richer apparatus, until foedera proliferate or the premise-gated governor-selects feature lands.

## Done when
descry's and instate's operation subdocs are authored contract-first (the RBSDC / RBSAO precedent); the RBRR selector, the RBRF library restructure, and the cognomen eviction are spec'd (RBS0 / RBSRR / RBSRF); the spec text aligns with the ₣Bi chaining-roles boundary; the fixture-composition pattern is specified so the build and reuse-build paces can build against it.

## Character
Authoring of an already-landed design — contract-first spec work, parallelizable without the operator. The open deliberations (parley, the normalization dispositions, the rbgft idempotency test, sprue terminology) are settled in the attended chat and arrive as inputs; do not reopen them here.

### butcfc-seed-previous-wrapper (₢BfAAb) [complete]

**[260630-0829] complete**

Fold the repeated mkdir-then-seed-previous idiom in the BUK fact-chaining self-test (butcfc_facts.sh) into a dedicated zbutcfc_seed_previous wrapper.

## Done when
The five mkdir+seed pairs become single calls; the OUTPUT_DIR seed untouched; BUK self-test fact-chaining green.

## Cinched
Hardcode BURD_PREVIOUS_DIR; do NOT route the OUTPUT_DIR seed through it; keep the mkdir inline (readonly-path constraint). Detail: the audit memo's pace-slate.

## Character
Trivial BUK-self-test fold; framework-self-test stratum.

### federation-buildout-orientation (₢BfAAI) [complete]

**[260623-0910] complete**

A reminder-and-convene pace: before any federation build-unit is cut, the operator and agent reach shared understanding of the work the ride-or-die proof left open.
Mount this fresh — it is orientation and design conversation, not implementation.

The proven ground (read first): the ride-or-die Keycloak->GCP->don chain is harness-proven green; the full result, the empirical Palisade findings, the JWKS-refresh coupling, and the three-layer architecture are recorded in the config-model memo's "Proof result" section (Memos/memo-20260618-Bf-federation-config-model.md).
The working artifacts are the fdkyclk crucible (vessel rbev-bottle-fdkyclk + nameplate rbmm_moorings/fdkyclk/) and the idempotent POC driver + teardown (rbmm_moorings/fdkyclk/fdkyclk-proof.sh, fdkyclk-teardown.sh).

Two deferrals the proof consciously made — the "wrap issues" to understand:
1. Grant mechanism — the proof minted the Keycloak id_token via the PASSWORD grant as a de-risk stepping-stone, NOT the RFC 7523 JWT Authorization Grant the proof was meant to use.
   GCP cannot distinguish the grant (it validates the resulting id_token's signature / iss / aud / sub), so the cloud chain is proven regardless — but the durable rig must mint via RFC 7523, whose Keycloak-side setup (a trusted asserting key) is un-built and un-understood here.
2. Implementation form — the proof was driven by a gcloud POC, not durable code.
   The operator cinch is that the durable establishment + accessor are REST-only (the payor OAuth token + rbuh/rbge patterns + the rba STS/don shapes), NO direct gcloud.
   Every gcloud call in the POC has a REST sibling already in-tree (affiance for the pool/provider, rbgp_brevet for the IAM), so the transform is assembly not invention — but it is the establishment/accessor build-units, gated on the spec-first recast, which is Fable-gated.

The open design question to settle here — Keycloak control: how is the IdP configured durably?
The proof applied the realm config (client, subject, frontendUrl, eventually the RFC 7523 grant) live via station admin-REST.
Its proper home is one of: baked vessel realm-import JSON (config data), a rbw-* tabtarget family for Keycloak control, or a station BCG module — and the choice cuts across the three-layer split (vessel data vs station orchestration).
This is the BCG / gcloud / tabtargets-for-Keycloak-control question flagged at wrap.

## Done when
The operator and agent have walked the proven state and the two deferrals to shared understanding.
The Keycloak-control approach (realm-config home) is decided, or its decision-owner (operator vs Fable) is named.
The next concrete federation build pace(s) are slated — or explicitly deferred behind the spec-first / Fable gate, with that gate named.

## Cinched
The durable establishment + accessor are REST-only — no direct gcloud (operator cinch).
The build-units sit behind the spec-first recast (contract-before-code doctrine), Fable-gated for vocabulary; this pace plans, it does not jump that gate.

## Character
Orientation and design conversation; mount fresh, with the operator present.
Not implementation — its output is shared understanding plus slated next paces.

### keycloak-estate-key-genesis (₢BfAAr) [abandoned]

**[260627-1312] abandoned**

Generate the stable Keycloak signing keypair once, fence its private half in the never-committed station file, and commit its public half as the canonical JWKS reference — the producer the estate-secret stub, the baked realm, and affiance all consume.

## Why this pace exists
The 260623 stable-key decision and the 260624 stub-not-regime decision left a producer gap (surfaced by the heat-integrity audit): three paces reference "the already-committed JWKS (the canonical reference)" — the estate-secret stub verifies against it, the baked realm bakes it into its RFC 7523 trusted-key, affiance uploads it to the workforce provider — but no pace produces it. The original "public JWKS committed" step (config-model memo subdoc #3) was orphaned by those two decisions and never re-homed. This pace is that home.

## Spec of needed change
The stable keypair is generated ONCE (no per-charge rotation, operator decision 260623):
- generate the keypair — the programmatic/caged path's root of trust for the Keycloak foedus;
- the PRIVATE half is operator-fenced: this pace provides the procedure that emits it for the operator to place in the never-committed station file (working name RBRS_ESTATE_KEYCLOAK_SECRET); the secret is never committed, mirroring the manor OAuth secret seam (public config commits, the one durable secret is operator-held);
- the PUBLIC half is committed as the canonical JWKS reference at the location the federation regime names — the artifact the stub matches against, the realm bakes, and affiance uploads.

## Done when
The stable keypair is generated, the private-half placement procedure exists (operator runs the secret-fencing), and the public JWKS is committed as the canonical reference at the regime-named location — so the estate-secret stub, the baked realm, and affiance all have a real artifact to consume.

## Cinched
Stable key, generated once — regenerate only on a deliberate rotation, never per charge (operator 260623).
The private half is NEVER committed (operator-fenced in the station file); only the public JWKS commits — the ships-committed-no-secrets seam.
This pace PRODUCES the canonical JWKS; the estate-secret stub later in this heat only sources-and-verifies against it, its source-plus-verify cinch intact.

## Character
Thin establishment / bootstrap over a settled design (stable key, fenced private half); one-time keypair genesis, not recurring runtime. Needs the regime's JWKS-location contract (the spec recast) before it; sits before the stub/realm/affiance consumers.

### estate-secret-verification (₢BfAAP) [abandoned]

**[260627-1312] abandoned**

Build the minimal estate-secret stub — a self-labelled BCG module that sources-and-verifies the Keycloak signing key from the shelved interim station file, so the realm and orchestrator have a stable key to consume.

The estate-secret HOME is settled (operator 260624, paddock Foedus-lifecycle section): do NOT reinstate the RBK station regime (RBRS) — it stays shelved in Tools/rbk/vov_veiled/FUTURE/ for the podman feature (₣BW).
Instead a deliberately-minimal, self-labelled stub whose ONLY job is to source-and-verify this one field.

## Spec of needed change
A small BCG module that:
- sources the private signing key from the never-committed interim station file (working name RBRS_ESTATE_KEYCLOAK_SECRET), TOLERATING the file's absence (skip the programmatic path when absent);
- verifies it by matching its public half against the already-committed JWKS (the canonical reference), failing LOUD on present-but-mismatched — never a silent pass;
- is self-labelled as a stub: a header noting the full RBRS station regime is shelved for ₣BW, with a pointer to FUTURE/RBSRS.
Also: add the Keycloak field to the shelved FUTURE/RBSRS spec as an optional/future entry, give the live stub its own short contract, and repoint the dangling claude-rbk-acronyms.md RBSRS pointer to the FUTURE/ path (annotated shelved-for-₣BW), not deleted.

## Producer — RESOLVED (260625, the integrity-audit gap closed)
The canonical public JWKS this stub verifies against is now produced by the estate-key genesis pace slated earlier in this heat: it generates the stable keypair once, fences the private half in the station file, and commits the public JWKS.
So this stub verifies against a real, committed artifact; its source-plus-verify cinch is intact — it does NOT produce or commit the JWKS.
(Before the genesis pace was slated, no ₣Bf pace produced this JWKS — the original producer step had been orphaned by the 260623 stable-key reversal and the 260624 stub-not-regime decision.)

## Done when
The stub sources-and-verifies the one estate field (present → match-or-fail-loud; absent → skip), is self-labelled with the ₣BW-shelved pointer, the FUTURE/RBSRS spec carries the optional field, and the acronym pointer resolves; it verifies against the canonical JWKS produced by the earlier estate-key genesis pace.

## Cinched
NOT a station-regime reinstatement and NOT the Estate-concept nucleation — a single labelled stub; the full RBRS regime and any dedicated-estate-regime graduation defer with podman (₣BW).
Source-plus-verify ONLY (operator 260624) — no podman anticipation, no extra fields; the canonical JWKS is produced by the separate earlier genesis pace, NOT this stub.
The private half is never committed; the match is always against the committed JWKS, never a cross-station secret comparison.

## Character
Small BCG build over a settled design; sits after the estate-key genesis pace (its producer) and before the realm/orchestrator paces that consume the stable key (the stub name is an in-pace hearting mint).

### rfc7523-grant-and-baked-realm (₢BfAAN) [complete]

**[260630-0946] complete**

Work out the un-understood Keycloak-side RFC 7523 JWT-authorization-grant setup, and bake the durable realm so configure_realm evaporates.

The proof minted the id_token via the PASSWORD grant as a de-risk stepping-stone; the durable rig mints via the RFC 7523 grant, whose Keycloak-side setup (a trusted asserting key registered in the realm) is un-built and un-understood — understand it first, then bake it.
Fatten the baked realm at rbmm_moorings/rbmv_vessels/rbev-bottle-fdkyclk/fdkyclk-realm.json with the client, audience mapper, user, frontendUrl, and the RFC 7523 trusted-key config as declarative import entries, until the proof's configure_realm has nothing left to apply live.

Source: fdkyclk-proof.sh (configure_realm + mint_idtoken) is the live-applied form being de-laminated into baked DATA; the vessel Dockerfile already loads it via --import-realm; the paddock de-lamination section is the model.

## Done when
The fdkyclk vessel boots a fully-configured realm via --import-realm with no runtime admin-REST and mints an id_token via the RFC 7523 grant.
The realm signing key is EPHEMERAL — generated fresh in the crucible on each charge, persisted nowhere; the orchestrator fetches the fresh public JWKS and uploads it to the provider at establish (the twiddle).

## Cinched
The realm signing key is EPHEMERAL, generated fresh per charge and committed nowhere (paddock "Foedus switching — AUTHORED + residual shape"; supersedes the earlier stable-key decisions 260623/260624 for the fed key).
There is NO never-committed station estate secret and NO committed JWKS reference — both belonged to the overturned stable model; the station-key paces that carried them were dropped.
Because the key is fresh each charge, affiance re-uploads the fetched public JWKS on every establish — the twiddle is a per-charge re-sync, not a one-time upload.
The full RBRS station regime stays shelved for ₣BW; this pace introduces no station-key stub.

## Character
Investigation (RFC 7523 on Keycloak) plus mechanical realm-JSON authoring.

### affiance-programmatic-mechanism-arm (₢BfAAM) [complete]

**[260630-1012] complete**

Add the programmatic (uploaded-JWKS) arm to affiance, gated by the mechanism discriminator from the spec recast.

affiance today founds the interactive (device-flow) trust; add the programmatic arm that creates the workforce provider from an uploaded public JWKS plus the id-token web-sso flags.
affiance stays vendor-agnostic — it consumes a JWKS file and an issuer string as opaque inputs, never reads realm contents, never learns "Keycloak."

Source: fdkyclk-proof.sh ensure_gcp_provider is the proven gcloud shape to transform to REST (gcloud-to-REST per the no-gcloud cinch); rbgp_manor_affiance is the existing verb home.

## Done when
affiance founds a programmatic foedus from an uploaded JWKS via REST, mechanism-gated, with no direct gcloud.

## Cinched
REST-only, no direct gcloud (operator cinch).
The discriminator is mechanism, not vendor — the programmatic arm is the same gate pattern the vessel regime already uses (buv_enum_enroll + buv_gate_enroll).

## Character
Bash; REST transform of a proven gcloud flow.

### onepool-spec-recut (₢BfAAv) [complete]

**[260630-2127] complete**

Re-cut the federation specs to the 260630 one-pool identity substrate (paddock "Model" section) — the contract-first hard predecessor of every model-touching code pace.

## Spec of needed change
A foedus becomes a PROVIDER under one manor-lifetime pool (was: a pool per foedus). Re-cut, contract-first:
- RBSRF: the workforce-pool id moves to a manor-level home; RBRF_PROVIDER_ID is the per-foedus discriminator; the library/selector (RBSRR) stand.
- RBSMA: affiance founds a PROVIDER under the standing pool, not the pool; the pool's one-time founding moves to manor setup. Drop the pre-existing-pool fast-fail / refuse-and-rotate framing (moot). The namespacing attribute-mapping (below) is set provider-side here.
- RBSMJ: jilt deletes a PROVIDER (dissolves a foedus), not the pool. The old pool-delete relocates to the manor-teardown force-delete (the finisher's inverse, its own pace) — deliberately distinct so an everyday foedus-jilt can never nuke the pool.
- RBSFD: descry reads provider presence under the pool, not pool absence / soft-delete.
- RBSPB (and the polity admission specs): membership is per-individual on the stable pool by an IdP-stable canonical subject; the synthetic test provider's subjects are namespaced DISJOINT from production (the load-bearing co-tenancy rule).
- The canvass operation sheaf: re-cut to enumerate providers under the one pool (providers.list) — the contract ₢BlAAU consumes.
- The manor found/teardown contract: the finisher founds the one pool (ensure-exists); a distinct manor-teardown force-deletes it (buc_require-gated). Homed in the finisher's new operation subdoc (₢BfAAF), referenced here.
- RBS0 federation civics: foedus≡provider, one manor pool; coordinate with the manor/org canon scrub (₢BfAAE).

## Source
The paddock "Model — the one-pool identity substrate" section and the four 260630 spec studies (founding, membership, terrier, test-bed/accessor).

## Done when
RBSRF / RBSMA / RBSMJ / RBSFD / RBSPB, the canvass sheaf, the manor found/teardown contract, and the RBS0 federation civics express the one-pool / foedus≡provider model with the per-individual-normalized + test-namespacing membership rule; no code re-cut yet. The membership-composer code re-cut (₢BfAAw) and the verb code follow.

## Character
Spec authoring over a settled model (the Model section is the decision); contract-first hard predecessor — no model-touching code lands before this.

### clean-tree-gate-precision-variant (₢BfAAu) [complete]

**[260630-1945] complete**

Add a well-formed bug_require_* clean-tree gate variant — one that rejects with a named BCG precision-exit-code band and takes the detailed error condition as a tinder/kindle constant plus the RB rationale as a parameter — as standalone BUK/BCG infrastructure (no call site wired here).

## Spec of needed change
Today's bug_require_clean_tree (Tools/buk/bug_git.sh) is malformed as a deliberate-rejection gate: it takes a free-string operation context and calls buc_die (generic fatal), with no precision band. A deliberate-rejection gate per BCG should buc_reject a named band and carry the detailed error condition as a structured constant.
Add a NEW variant alongside it (do NOT modify the existing one — its migration is a separate later pace on the loose-ends heat). The variant rejects on a dirty tree with a named clean-tree band (mint it in the bubc precision block — take the next free code, grep BUBC_band_ to confirm), takes the error-condition tinder/kindle constant, and takes the RB rationale as a PARAMETER so BUG stays kit-agnostic.
The RB opinion the gate states — clean commits are demanded because building a container image from uncommitted state causes confusion — rides that rationale parameter as an RBCC creed constant (no such constant exists today; mint it). This keeps the opinion RB-side and out of kit-agnostic BUK.
This pace builds the machinery only and wires no call site. The first live consumer is the Keycloak-facility orchestrator pace (later in this heat), which places the gate; the existing ~9 RBK call sites stay on the old gate for the loose-ends migration pace (₢BiAAl).

## Done when
A well-formed bug_require_* variant exists (named bubc clean-tree band + error-condition tinder/kindle constant + RB-rationale parameter), the existing malformed gate is untouched, and BUK self-test is green. No call site is migrated or wired here.

## Cinched
BUG stays kit-agnostic — the RB opinion rides the rationale parameter (an RBCC creed constant), never baked into BUG.
The existing bug_require_clean_tree is NOT modified here; its call-site migration and retirement is the loose-ends heat's pace (₢BiAAl).
Infra-only: the first live consumer is the orchestrator pace, not this one — so this pace carries no obligation to wire a site.

## Character
BUK/BCG infrastructure: a new precision-band gate variant plus a minted bubc band and RBCC creed constant. Orthogonal to the federation model — spec-first-exempt, rides early and parallel to the spec re-cut.

### await-thg-zipper-baton (₢BfAAp) [complete]

**[260701-1750] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BlAAb (heat ₣Bl — the colophon-completeness check, ₣Bl's LAST zipper write under the 260630 rework, after canvass added rbw-jc) is complete, pushed to origin, and origin merged back into this clone.
The zipper trio (rbz_zipper.sh plus its two build-regenerated files) is theurge-owned and re-derives wholesale, so the federation stream takes the baton only after ₣Bl finishes ALL its zipper writes — now canvass (rbw-jc) plus the colophon-check, both post-dating the old ₢BlAAN trigger.
Positioned after ₢BfAAv (the spec re-cut, non-zipper) so the spec lands and unblocks ₣Bl's canvass before this baton waits — only then may the following ₣Bf zipper paces (force-delete, finisher, Keycloak-facility) edit the trio.

## Done when
₢BlAAb is wrapped and pushed to origin, and this clone has pulled origin so the full theurge zipper state (incl. canvass rbw-jc + the colophon-check) is present before this stream edits the trio.

## Character
Cross-clone barrier (zipper baton ₣Bl → ₣Bf → ₣Bi); retargeted 260630 from the stale ₢BlAAN to ₣Bl's actual last zipper write ₢BlAAb, and repositioned after ₢BfAAv to avoid the spec-re-cut cycle.

### rbrw-regime-buildout (₢BfAA0) [complete]

**[260701-1827] complete**

Build the RBRW Workforce regime module and relocate the manor pool identity out of RBRF — the code home for the org / pool-id / session-duration fields the one-pool spec recast (₢BfAAv) moved to a manor-level regime — re-pointing every consumer of the stripped fields in the same landing.

## Spec of needed change
RBSRW (new, ₢BfAAv) specifies the regime; nothing builds it yet. Build the code, strip the vacated fields, and re-point the consumers atomically:
- New module rbrw_regime.sh (+ CLI) enrolling RBRW_ORG_ID, RBRW_WORKFORCE_POOL_ID, RBRW_SESSION_DURATION under one group, per RBSRW's kindle/validate/render (org numeric; pool-id lowercase-alnum-hyphen with the gcp- prefix reserved; session NNNs bounded 900s–43200s). Follow the RBRR/RBRP regime-module pattern.
- Moorings home: a single manor-level committed file (RBRW is axrd_singleton, one per manor) — settle its path in the moorings tree, sibling to the rbmf_foedera library, not per-foedus.
- Strip RBRF: remove RBRF_ORG_ID / RBRF_WORKFORCE_POOL_ID / RBRF_SESSION_DURATION from rbrf_regime.sh and from every rbef_*/rbrf.env in the moorings foedera library — they now live in RBRW.
- Re-point the stripped fields' consumers IN THE SAME LANDING (the strip alone leaves the tree broken in ways the gates below cannot see). Discovery: grep -rn 'RBRF_ORG_ID\|RBRF_WORKFORCE_POOL_ID\|RBRF_SESSION_DURATION' Tools/. Known at reslate time (re-grep, don't trust):
  - rbof_foedus.sh — descry PARSES org/pool from the inspected foedus's rbrf.env and canvass PARSES pool from the active one (file-parse, not kindled vars); both now read the manor-level RBRW file. Provider fields stay RBRF. Mechanical source-move only — descry's verdict semantics are the verb re-cut pace's (₢BfAA1).
  - rba_auth.sh — the sitting-file name and the STS audience compose pool+provider; re-point the pool read to RBRW. Breaking this silently breaks avow, the whole credential path.
  - rbgp_payor.sh — mechanical name re-point of the field reads only; affiance/jilt/composer semantics stay with ₢BfAA1 / ₢BfAAw.
  - rbtdrv_patrol.rs — the lifecycle fixture's poison const targets RBRF_WORKFORCE_POOL_ID; re-point it to RBRW_WORKFORCE_POOL_ID (one line, safety-critical: left inert, the poison stops masking the pool id and the fixture's affiance/jilt would target the REAL manor pool). The fixture's provider-grain re-cut proper stays ₣Bl's.
- Tabtargets rbw-rwr (render) / rbw-rwv (validate), per the regime-tabtarget pattern — a zipper write, so this rides BEHIND the ₢BfAAp baton — plus the zipper enrollment for the new colophons.

## Done when
rbrw_regime.sh kindles/validates/renders the three fields per RBSRW; the manor-level moorings file exists; rbrf_regime.sh and the rbef_*/rbrf.env files no longer carry org/pool/session; every stripped-field consumer reads RBRW (the discovery grep lands clean outside specs); rbw-rwr/rbw-rwv are enrolled; reveille (regime-validation) and the fast qualify stay green — AND, because those gates cannot see rbof/rba breakage, verify live: rbw-aa (avow/STS) and rbw-jd (descry of the standing foedus) both still work.

## Cinched
RBRW is the manor pool identity's sole home (RBSRW); RBRF keeps only the per-foedus provider fields.
The strip and the consumer re-point are ONE landing — the tree never holds the strip without the re-point.
Hard predecessor of the finisher (₢BfAAF), the membership composer (₢BfAAw), and the affiance/jilt verb re-cut — they all READ RBRW.
Tabtarget enrollment rides behind the ₢BfAAp zipper baton.

## Character
Regime-module build over a settled spec (RBSRW) plus a mechanical field strip AND consumer re-point across RBRF, the rbef_ library, rbof, rba, rbgp, and the one patrol const; the moorings-path choice is the one open judgment.

### manor-teardown-force-delete (₢BfAAx) [complete]

**[260701-1849] complete**

Build the manor-teardown — force-delete the manor workforce pool, the deliberate inverse of the ensure-exists finisher, so a release ladder can start from a clean manor.

## Spec of needed change
A distinct, dangerous manor-level op (NOT foedus-jilt, which is the everyday provider-delete). Deletes the one manor pool; the next finisher run recreates it clean. Re-enrolls every citizen by construction (bindings key on the pool) — which is exactly why it is separate and gated, never the finisher's default.
- buc_require-gated, matching manor-jilt / depot-unmake: `buc_require "DANGER: Force-delete the manor workforce pool … re-enrolls every citizen" "<pool-id>"` (TTY-typed confirmation, honors BURE_CONFIRM=skip for tests).
- Payor-credentialed (org-level, like affiance/jilt).
- Internal test infra: the CODE ships, the tabtarget accelerator is WITHHELD from delivery — consumers never get a one-keystroke pool-destroyer.

## Done when
A payor-credentialed, buc_require-gated tabtarget force-deletes the manor pool; the release ladder runs it before the finisher to start clean; the tabtarget is marked withheld-from-delivery (code ships, accelerator does not).

## Cinched
Distinct from foedus-jilt (provider-delete) — the danger lives in a separate, scary-gated verb so everyday jilt can never nuke the pool.
buc_require gate, BURE_CONFIRM=skip seam.
Withheld from delivery (internal only); code ships.

## Character
Bash — one dangerous payor verb reusing the manor-jilt confirm pattern, plus the delivery-withhold marking.

### membership-composer-recut (₢BfAAw) [complete]

**[260701-1857] complete**

Re-cut the membership-principal composer to the one-pool model — per-individual normalized subject on the stable manor pool, test-provider subjects namespaced disjoint from production. The code follow-on of the ₢BfAAv spec re-cut.

## Spec of needed change
zrbgp_principal_member_capture (rbgp_payor.sh) builds principal://…/workforcePools/{pool}/subject/{S} from the per-foedus pool id and the raw IdP subject. Re-cut per RBSPB (re-cut by ₢BfAAv):
- The pool is the one manor pool, sourced from RBRW_WORKFORCE_POOL_ID (the RBRW Workforce regime), not the retired per-foedus RBRF pool field.
- S is an IdP-stable canonical subject; the synthetic test provider's subjects are namespaced disjoint from production (the attribute-mapping that yields this is set provider-side in affiance, per RBSMA).
- The composer is the single home — brevet / gird / unseat / attaint all route through it.

## Done when
The principal composer and its polity-verb consumers build membership on the one manor pool (RBRW-sourced) with the normalized + namespaced subject per RBSPB; the exact-string unseat/attaint revoke still matches (no binding orphaned by the pool-id change).

## Cinched
Per-individual membership, NOT group — the terrier muniment's 1:1 record↔grant mirror is preserved (a group binding would sever it).
Contract-first: builds against RBSPB as re-cut by ₢BfAAv; reads the pool from RBRW, not RBRF.

## Character
Surgical bash re-cut at one composer + its polity-verb consumers; sweeping any existing per-foedus-pool bindings (the exact-string-revoke leak the membership study flagged) is the in-pace judgment call.

### terrier-founding-home-finisher (₢BfAAF) [complete]

**[260701-2003] complete**

Build the manor-setup finisher — a single idempotent post-payor-guide tabtarget that founds the scriptable manor: the workforce pool (once) plus the terrier bucket + per-polity folders. ENSURE-EXISTS (operator 260630).

## Spec of needed change
Founding-home is OPTION B (terminal Fable 260622): the Manor provisions these, not affiance.
- The ONE manor pool's one-time founding lives here (founded once at manor setup, never by affiance/jilt), reading its identity from the RBRW Workforce regime (org / pool-id / session-duration; RBSRW).
- Idempotency is ENSURE-EXISTS, but NOT a bare get-by-id: RBSRW's drift guard (₢BfAAv-contracted) makes the finisher the pool_id half of the sync. Before founding, LIST the RB-marked pools under RBRW_ORG_ID; if one stands under a different id than RBRW_WORKFORCE_POOL_ID, REFUSE (coordinate drift) rather than create — a plain get-by-id would silently found a second, empty pool. No RB-marked pool → create; present at the expected id → PATCH-reconcile the reconcilable field (session-duration, non-destructive). An org_id drift is unguardable here (the org is the search scope) and rests on org-invariance, per RBSRW.
- The finisher NEVER deletes the pool — clean-slate is the separate force-delete sibling (₢BfAAx manor-teardown), run before the finisher in the release ladder.
- The terrier bucket + per-polity managed folders + grain IAM.

Out of this pace (siblings): the terrier muniment provider-dimension (₢BfAAy) and the rbw-dt/dT scaffold retirement (₢BfAAz, behind ₢BfAAq). This finisher is the founding tabtarget only.

Contract-first: rides behind ₢BfAAv (the found/teardown contract in the new finisher subdoc; the pool sync-guard shape in RBSRW) and behind the RBRW regime module (the finisher READS RBRW, does not build it). Sequenced AHEAD of ₢BfAAL — the Keycloak orchestrator's affiance-add-provider needs the pool standing.

## Spec-sync
A NEW operation subdoc for the manor found/teardown pair (provisional acronym minted in-heat), authored before the code (the contract is re-cut in ₢BfAAv; the pool sync-guard shape is in RBSRW). NOT RBSTR (terrier ACCESS) / RBSMF (depot-grain).

## Done when
A standing manor-setup-finisher tabtarget idempotently ensures the manor pool via the RBSRW drift guard (list-and-match under the org, refuse on id-mismatch, create-if-absent, PATCH-reconcile session-duration — never a bare get-by-id, never a delete) reading RBRW; plus the terrier bucket (payor-project-grain) + per-polity folders (depot-grain) + grain IAM; the new operation subdoc exists; gauntlet stays green.

## Cinched
Founding-home = Option B (Manor, not affiance); one idempotent finisher after manual manor setup.
Ensure-exists, never destructive — clean-slate is the force-delete sibling.
The pool guard is list-and-match (RBSRW), not a bare get-by-id — a drifted pool_id must refuse, never silently found a second pool.
Spec home = a NEW operation subdoc, not RBSTR/RBSMF.

## Character
Implementation over a settled design; the B-variant, the folder-step credential boundary, the PATCH-reconcile scope, and the list-and-match drift guard are the in-pace judgment calls.

### affiance-jilt-provider-recut (₢BfAA1) [complete]

**[260701-2142] complete**

Re-cut the foedus verb code to the one-pool provider model per the re-cut RBSMA / RBSMJ / RBSFD (₢BfAAv) — affiance creates a provider under the standing pool (no longer founds it); jilt deletes the provider (no longer the pool); descry reads provider-grain health.

## Spec of needed change
rbgp_manor_affiance / rbgp_manor_jilt (rbgp_payor.sh) still found and delete the workforce POOL, and rbof_descry (rbof_foedus.sh) still reads pool-grain health. Re-cut to the provider model:
- affiance (RBSMA): drop the pool-ensure and the org-level workforcePoolAdmin self-grant (both move to the finisher ₢BfAAF); REQUIRE the manor pool present (fatal if absent, directing to the finisher); create the provider under RBRW_WORKFORCE_POOL_ID; drop the soft-delete / refuse-and-rotate handling (moot); write the attribute-mapping verbatim (the subject-namespacing rides the regime value, provider-side).
- jilt (RBSMJ): delete the PROVIDER under the standing pool (workforcePools.providers.delete), not the pool; drop the pool soft-delete/undelete/graveyard handling (the pool force-delete is the separate ₢BfAAx teardown); keep the idempotent 404 short-circuit; provider-scoped confirmation.
- descry (RBSFD): re-cut the verdict read to provider-grain per the re-cut RBSFD — a foedus is a provider, so descry reads provider presence under the one manor pool; align the verdict tokens with RBSFD, keeping 'healthy' stable (the foedus-reuse fixture branches on it).
- All three read the pool id from RBRW (₢BfAA0's relocation, already re-pointed mechanically); this pace owns the SEMANTICS.

## Done when
affiance creates a provider under the standing manor pool and fatally requires that pool present; jilt deletes only the provider; descry reports provider-grain verdicts per the re-cut RBSFD and the foedus-reuse fixture's healthy branch still passes; all three source the pool from RBRW; the paths verify against the re-cut RBSMA/RBSMJ/RBSFD; the relevant test tier (picket) stays green.
Known-broken residue, by design: the foedus-lifecycle fixture still asserts pool-grain terminals after this lands — its provider-grain re-cut is ₣Bl's canvass-weave pace, behind that stream's one-pool barrier; the fixture is operator-invoked (no suite member), and with the poison const re-pointed at RBRW it fails LOUD at affiance's pool-present gate rather than touching the real pool.

## Cinched
Affiance never founds the pool and jilt never deletes it — both are provider-grain now (RBSMA/RBSMJ); the pool is the finisher's (₢BfAAF) and the teardown's (₢BfAAx).
descry's verdict re-cut lands HERE (verb semantics), not in the regime-buildout pace (mechanical re-points only).
Reads the pool from RBRW, not RBRF.

## Character
Surgical bash re-cut of three foedus verbs (two payor, one rbof) against the settled RBSMA/RBSMJ/RBSFD; how much of the affiance founding-step logic moves wholesale to the finisher is the in-pace judgment.

### affiance-jilt-folio-recut (₢BfAA3) [complete]

**[260702-0207] complete**

Re-cut affiance and jilt to folio-address the foedus — take the foedus identity as a folio and act on rbef_<folio>, mirroring descry — instead of reading the entrada-pinned RBCC_rbrf_file constant.

## Spec of needed change
Contract-first (the heat's hard rule): re-cut RBSMA (affiance) and RBSMJ (jilt) to the folio-addressed model before any code — the verb takes the foedus name as a folio and acts on that named foedus's provider, not the active/pinned foedus.
rbof_descry (RBSFD) is the in-tree precedent: it folio-addresses any named foedus and parses that foedus's own rbrf.env, independent of RBCC_rbrf_file and the active selector.
Code: the affiance/jilt CLI furnish (rbgp_cli.sh) sources the folio-derived rbrf.env path instead of the constant-folded RBCC_rbrf_file; the verb bodies stay foedus-blind (they already read RBRF_ from the kindled regime).
Reserve the active-foedus selector (RBRR_ACTIVE_FOEDUS / instate) for the human accessor (avow/don), completing the addressing split descry already half-embodies.

## Done when
rbw-mA and rbw-mJ take a foedus folio (param1) like rbw-jd, resolving rbef_<folio>/rbrf.env directly; RBSMA/RBSMJ describe the folio-addressed model; rbw-mA rbef_entrada still affiances the standing spike trust — proven, not regressed.

## Cinched
Folio-addressing reverses the prior "no CLI folio for affiance/jilt" decision (operator 260702); descry is the precedent.
Provider-management verbs are folio-addressed; the active-foedus selector is reserved for the accessor.

## Character
Contract-first spec re-cut plus a mechanical CLI change; the verb bodies are largely untouched.

Source: rbof_descry (rbof_foedus.sh) for the folio-resolution precedent; RBSMA / RBSMJ; RBCC_rbrf_file (rbcc_constants.sh) for the constant being bypassed.

### rbxk-keycloak-orchestrator (₢BfAAL) [complete]

**[260702-1025] complete**

Build the one BCG module orchestrating the Keycloak test-facility lifecycle: rbxk_keycloak.sh in the rbx family, composing the folio-addressed provider verbs.

## Spec of needed change
Create the rbef_keycloak foedus (programmatic mechanism) in the moorings foedera library: a committed rbrf.env TEMPLATE carrying the vendor-agnostic core plus the namespaced attribute-mapping, and an IGNORED live rbrf.env the orchestrator renders per setup with the fresh JWKS.
The ephemeral key is "committed nowhere" (RBRF's own stated intent, rbrf_regime.sh); only keycloak deviates from RBSRF's ships-committed, entrada untouched — spec-note the deviation in RBSRF as part of this pace.

Setup (rbw-qjK): precision-band clean-tree gate FIRST (bug_require_clean_tree_creed, its first live consumer; pass the RB rationale creed), then charge fdkyclk, poll Keycloak ready, fetch its public JWKS (the bridge — strip to the standard RSA members kty/kid/use/alg/n/e; GCP's uploaded-JWKS parser rejects Keycloak's x5c/x5t extras) into the ignored live rbrf.env, then affiance the named foedus (rbw-mA rbef_keycloak).
Teardown (rbw-qj sibling, named at cut-time): jilt the named foedus (rbw-mJ rbef_keycloak), then quench.
Because the JWKS lands in the ignored carrier, the tree stays clean through setup, so affiance's own clean-tree gate passes untouched — no gate hoist, no skip. Verify this holds at mount (untracked files are ungated per BUG; the live file must never become tracked).

Compose existing verbs plus the one JWKS bridge — do NOT contain the accessor (rba's job), do NOT reimplement affiance or jilt (call them).
The module is blind to the realm JSON — affiliation by reference, never ownership; affiance never learns "Keycloak."
The orchestrator never speaks GCP REST and never holds the payor credential — its only HTTP is the local-crucible JWKS fetch; cloud mutation flows solely through the called verbs (the uppercase-K rationale: setup mutates cloud BY CALLING affiance).

Standing-provider staleness (SETTLED, 260702 terminal review): affiance grows the narrow programmatic re-sync arm, contract-first — re-cut RBSMA before the code.
The re-seat fork (jilt-then-affiance) is dead three ways: affiance's provider 200-branch is state-blind (unlike its pool gate), so a soft-deleted provider reads as present and create is skipped against a dead trust — the provider-grain twin of the latent-affiance-bug finding in Memos/memo-20260617-BZ-workforce-pool-constraints.md; soft-delete holds the id (pool-documented, provider-extrapolated), so recreate collides; and undelete alone restores the OLD key snapshot, so no delete/undelete cycle can converge the twiddle — only a patch can.
The arm, scoped to the twiddle: under the programmatic mechanism, a standing live provider gets providers.patch of oidc.jwksJson from the regime value (updateMask discipline per the instaurate pool-reconcile precedent); a soft-deleted provider is undeleted then patched; 404 creates as today; whether a soft-deleted provider surfaces as 200-DELETED or 404 on GET is an impl-confirm at this pace's first live cycle (jilt's own verify loop already tolerates both).
Full drift-reconcile (attributeMapping / issuer / clientId) stays the named deferred follow-up — RBSMA's drift NOTE narrows, it does not dissolve.
An orchestrator-owned direct patch was weighed at this review and rejected as re-lamination.
RBSMA's webSsoConfig impl-confirm NOTE (does the REST create demand webSsoConfig under programmatic?) settles at this pace's first live create.

Subject-namespacing (the one-pool co-tenancy rule, RBSPB / rbtf_citizen): under the shared manor pool this test provider is co-resident with production, so its subjects MUST be namespace-disjoint from production — a test token can never satisfy a production `principal://…/subject/{S}` binding (load-bearing security).
Author it in the foedus attribute-mapping (a CEL prefix on `google.subject` placing subjects in a reserved namespace), NOT the realm JSON — preserving realm-blindness; only if the realm must mint a namespaceable claim does this coordinate a RBSFK / fdkyclk-realm.json touch, never the orchestrator owning realm internals.

Source: fdkyclk-proof.sh (fetch_jwks plus the main staging) and fdkyclk-teardown.sh (the jilt-then-quench inverse); the paddock de-lamination and naming section; RBSPB plus rbtf_citizen for the namespacing rule; RBSMA/RBSMJ for the folio-addressed contracts; Memos/memo-20260617-BZ-workforce-pool-constraints.md for the soft-delete findings the staleness settlement rests on.

## Done when
rbw-qjK stands up the programmatic facility (charged Keycloak + affianced rbef_keycloak) and its teardown sibling tears it down, idempotently, composing the folio-addressed rbw-mA/rbw-mJ plus charge/quench;
the ephemeral JWKS lives in the ignored carrier so the tree stays clean through setup;
a rerun against a standing provider converges on the fresh JWKS via affiance's programmatic re-sync arm (RBSMA re-cut first), and a setup after teardown restores-and-converges the soft-deleted provider rather than mistaking it for standing;
the setup's gate is bug_require_clean_tree_creed with an RB rationale, not the old bug_require_clean_tree;
the test foedus's subjects are namespace-disjoint from production per RBSPB.

## Cinched
rbx family, rbxk_keycloak.sh module; colophon rbw-qjK (setup — uppercase K because setup CALLS AFFIANCE and so mutates cloud) plus an rbw-qj teardown sibling named at cut-time; the module letter need not match the colophon.
The module is blind to the realm JSON — affiliation by reference, never ownership.
Composes the folio-addressed affiance/jilt — the orchestrator names rbef_keycloak and never touches the active-foedus selector.
The orchestrator never speaks GCP REST and never holds the payor credential (260702): cloud mutation flows only through the called manor verbs; the staleness settlement lives in affiance's programmatic re-sync arm, never in the orchestrator.
Keycloak's ephemeral JWKS lives in an IGNORED live rbrf.env rendered from a committed template; entrada untouched (the one sanctioned deviation from RBSRF ships-committed — spec-note it).
The setup gate is the precision-band variant's first live consumer; wiring the gate lands here, not in the infra pace.
Test-provider subjects are namespace-disjoint from production (RBSPB), authored in the foedus attribute-mapping.

## Character
Bash orchestration plus one narrow verb arm; the heart of the de-lamination.
The RBSMA re-cut for the re-sync arm leads, then composition of existing verbs plus the JWKS bridge.
The subject-namespacing mapping is a small config addition riding the same facility setup.

### programmatic-self-supply-spec-recut (₢BfAA6) [complete]

**[260702-1234] complete**

Contract-first predecessor of the programmatic accessor arm:
re-cut the federation specs so the accessor's RFC 7523 grant inputs have a regime home before any code moves.

RBSRF (Tools/rbk/vov_veiled/RBSRF-RegimeFederation.adoc): extend the programmatic mechanism group with generic self-supply fields —
a reachable grant token-endpoint (live-rendered per charge by the orchestrator beside the JWKS; validator admits https or loopback-http, cleartext tolerated on loopback only — the test IdP is a local crucible, and RBSFK pins the POST target as local, distinct from the fictional https frontend issuer);
asserter key-file and client-secret-file PATH fields, repo-root-relative (the regime carries references, never secret values — sourced regime vars live in process env, so secret material must never ride them; the ships-committed / no-secrets premise stands un-amended);
and the public assertion facts (asserter kid, asserter issuer, asserter subject) as committed template values, dual-homed by contract with the realm DATA exactly as client-id already is.
The assertion aud is cinched to the existing issuer field — no new field.
Keep the reachable endpoint a distinct programmatic-gated field, not a promotion of the interactive token-endpoint to core: commit-vs-render provenance and https-vs-loopback validation are load-bearing differences.
Extend the one-sanctioned-deviation NOTE: the live render now carries two fields (JWKS + grant endpoint), both composed from the orchestrator's own knowledge (nameplate port, facility constants), never from realm contents.

RBSFA (Tools/rbk/vov_veiled/RBSFA-foedus_acquire.adoc): re-cut the stale precondition (the marshal-fenced-key / estate-stub line) to the two-key reality per RBSFK —
the accessor reaches the committed caged asserter key and client secret by regime path reference;
remove the self-mint fallback everywhere it appears (the accessor cannot hold the ephemeral realm key, so a self-mint would mean re-establishing the trust — not a fallback).

RBSFK (Tools/rbk/vov_veiled/RBSFK-foedus_realm.adoc): one correspondence line homing the caller-side grant inputs in the RBRF programmatic self-supply fields, and the client-secret file joining the caged-scaffolding census beside the asserter key.

Exact field names are minted in this pace under the Lapidary/grep gates; the shapes above are the contract.

## Done when
RBSRF specifies the programmatic self-supply fields (reachable endpoint, two secret-path references, three assertion facts) with validators, including the loopback-http escape from the https gate;
RBSFA's precondition reads the two-key reality with no self-mint fallback;
RBSFK names the correspondence.
No code moves in this pace.

## Cinched
Secrets enter the regime only as path references; the committed caged home keeps the material (two-key doctrine, RBSFK).
The reachable grant endpoint is orchestrator-rendered into the git-ignored live rbrf.env, composed never from realm JSON (the orchestrator stays blind to realm contents).
rba stays IdP-blind: it reads only generic RBRF_ fields and never learns "Keycloak".

## Character
Spec authoring (MCM/AsciiDoc) with minting judgment; no bash.

### programmatic-accessor-rba-arm (₢BfAAK) [complete]

**[260702-1306] complete**

Add the programmatic token-acquisition arm to the rba accessor:
mint the id_token via the test IdP's RFC 7523 grant, then exchange at STS for a federated token —
mechanism-gated, a sibling of the interactive (device-flow) arm, never a new module and never folded into the orchestrator.

The config-home is settled by the predecessor spec pace (the RBSRF programmatic self-supply fields):
the arm reads its grant inputs ONLY from generic RBRF_ fields — reachable grant endpoint, asserter key-file and client-secret-file paths, assertion kid/issuer/subject, aud = the existing issuer field — and never learns "Keycloak".

This pace lands the whole read surface with the arm, atomically:
- Tools/rbk/rbrf_regime.sh — enroll + validate the new programmatic fields per the re-cut RBSRF, including the loopback-http endpoint escape from the https gate.
- rbmm_moorings/rbmf_foedera/rbef_keycloak/rbrf.env.template — the committed assertion facts + path references; the client-secret file joins the caged home rbmm_moorings/fdkyclk/ beside the asserter key.
- Tools/rbk/rbxk_keycloak.sh — extend the live render with the reachable grant endpoint, composed from the nameplate port + facility constants (never realm contents).
- Tools/rbk/rba_auth.sh — the mint arm: assertion signed by openssl reading the key path (key never in a shell var), client secret delivered to curl by file reference, tokens jq-straight-to-stdout; reuse zrba_leg2_federated_capture wholesale; sitting-cache reuse is mount's call (the cache key is already pool+provider).

Source proof: rbmm_moorings/fdkyclk/fdkyclk-proof.sh (mint_idtoken + sts_exchange).
The civic verb for the programmatic acquisition act is decided at mount per RBSFA's deferral; staying descriptive is acceptable.

Ceiling: reaches the federated token — the end-to-end don needs a standing admission (a brevet of the test subject), owned by the deferred attach-caged-subject unit, not this pace.
The don path is shared and unchanged.
Do NOT reintroduce the proof's payor-direct brevet into the production-shared rba arm to force a green don here.

## Done when
With the Keycloak facility up, the accessor acquires a federated token over the instated programmatic foedus unattended, via the RFC 7523 grant and the interactive arm's shared STS leg;
the interactive arm's behavior is unchanged and regime validation passes on both standing foedera.
The don completes end-to-end only once the deferred attach-caged-subject unit provides a standing admission.

## Cinched
Stays an arm of rba (production-shared machinery), never absorbed into the rbxk orchestrator.
REST-only, no direct gcloud.
Grant inputs only via generic RBRF_ fields; secret material only by path reference — no secret value in any rbrf.env or shell var (BCG custody; the predecessor spec pace's contract).
No self-mint fallback: the RFC 7523 grant is the one programmatic path (RBSFK two-key doctrine).
The end-to-end don is deferred-by-design behind the post-₣BZ attach unit; no spine-local brevet.

## Character
Bash across four surfaces (regime, template, orchestrator render, accessor arm); consumes the contract the predecessor spec pace re-cut; intricate but mechanical once that contract stands.

### accessor-foedus-selector-resolve (₢BfAA7) [complete]

**[260702-1354] complete**

Make the singleton federation accessor selector-derived, so it reads the INSTATED foedus rather than the constant-folded entrada — the "federation family-of-named-instances rework" the RBCC_rbrf_file comment (rbcc_constants.sh), CLAUDE.md, and the paddock all defer.
This unblocks the programmatic accessor arm landed in the prior pace: with the arm reachable, this pace carries that pace's deferred end-to-end Done-when clause.

The crux (discovered at the accessor-arm mount, not stated in the RBCC comment):
RBCC_rbrf_file is a source-time literal pinned to rbef_entrada, but RBRR_ACTIVE_FOEDUS is only populated when rbrr.env is sourced during furnish/kindle — after rbcc_constants.sh.
So the selector-derived form must move RBCC_rbrf_file from a source-time constant to a resolution that runs after rbrr.env is sourced, re-pointing every consumer (rbgv_cli, rba's callers, rbw-rfv) and keeping the no-repo-regime contexts (no RBRR_ACTIVE_FOEDUS) working.

## Done when
With the selector-derivation in place and the Keycloak facility up (rbw-qjK), instating rbef_keycloak makes the accessor resolve the programmatic foedus, so:
rbw-rfv validates the keycloak programmatic regime through the standard path (no synthetic-file workaround);
and the programmatic arm acquires a federated token over the instated foedus unattended, via the RFC 7523 grant — the deferred Done-when clause of the accessor-arm pace.
The interactive path (entrada) still resolves and behaves unchanged when instated.

## Cinched
Production stays single-foedus-clean: rbef_entrada remains the committed default active foedus.
Contexts with no repo regime must not break — RBCC_rbrf_file must still resolve, or be shown unconsumed, where RBRR_ACTIVE_FOEDUS is unset.
Ceiling unchanged from the accessor-arm pace: the proof reaches the federated token only; the full don still awaits the deferred attach-caged-subject unit.

## Character
Sourcing-topology rework (source-time constant to post-RBRR computation) across RBCC_rbrf_file's consumers, plus a facility-up integration proof; structural judgment, not mechanical.

### entra-federation-setup-guide (₢BfAAJ) [complete]

**[260701-1719] complete**

Author the per-vendor federation setup guide for Microsoft Entra — the foreign-console human work that yields the vendor-agnostic core facts the regime reads.

A guide, not a workbench command (foreign console, no API token can drive it), under the payor-guide family: walk the Entra app registration and claims/scopes, ending at the core values affiance consumes (issuer, client-id, subject claim, JWKS source).
Entra first because it is the live IdP; further vendors become further guides under the same core-facts contract, with no schema or code change.

Source: the config-model conviction that vendor work is wholly guide-plus-values; HCG for durable-procedure craft.

## Done when
An operator can stand up the Entra side from the guide and author the federation regime from its values.

## Cinched
A guide, never a tabtarget; converges on the same core facts every per-vendor guide yields.
Needs only the spec recast's core-facts contract, so it runs parallel to the code paces.

## Character
Procedure authoring (HCG); foreign-console human steps.

### settle-federation-config-design-forks (₢BfAAC) [complete]

**[260622-1008] complete**

Two design forks the federation configuration model leaves open need collaborative resolution before the spec-first build unit can proceed; neither is doc-resolvable, so this pace holds them at the front of the heat as a reminder that they want a design conversation, not a lookup.

Fork one — the single-test-manor collision: can a synthetic test manor host both the preserved human-click proof and the headless caged suites on one active federation, or does the caged trust's own-pool requirement force the multiplicity axis (or per-run switching of the singleton regime)?
This is our test-rig topology, not a fact lookup — it turns on how the freehold and the device-flow proof are installed.

Fork two — the programmatic JWKS-source shape: is the JWKS source a programmatic-only uploaded field (fits the caged case alone), or a core field with two sub-modes (uploaded self-held versus issuer-discovered) so a real non-interactive IdP also fits?
This decides whether the mechanism gate is a clean two-value, and whether a Keycloak-style real-but-programmatic IdP is in scope for this heat.

## Character
Design conversation requiring operator judgment, and Fable where it touches the premise; the blocker the spec-first work waits on.

## Done when
Both forks are settled with rationale recorded in the config-model memo: the test-manor topology is chosen (single federation, or multiplicity, or per-run switch), and the JWKS-source shape is fixed (one field or two sub-modes, with the real-programmatic IdP scoped in or out).
The rework each implies for the spec-first build unit is named, not left implicit.

## Cinched
The configuration model itself — vendor-agnostic core plus mechanism gate, mechanism-not-vendor, OIDC-only — is settled and not reopened here; these forks live within it.

### heed-headless (₢BfAAA) [complete]

**[260701-1832] complete**

Reconsider whether avow (the sign-in verb) must gate on a controlling terminal, and if it relaxes, build the yawp channel as the headless mechanism — surface the device-flow prompt out-of-band so a relay can complete it, preserving the human-present premise without requiring a TTY.

## Spec of needed change
Two halves of one question, formerly split across two paces (the yawp exploration folded in here):
- The DECISION (premise-touching, operator-owned): does avow keep its TTY fail-fast, or relax to headless-but-human-reachable? The suspected design error is conflating "a human authenticates each sitting" (the premise) with "a human is at a controlling terminal" (the current enforcement) — which fails a headless-but-human-reachable caller (a long-running agent whose operator watches an output channel but holds no TTY).
- The MECHANISM (if it relaxes): emit the device-flow prompt (verification URL + user code) as a yawp (BUYM diastema wire format) instead of writing it to /dev/tty, so a relay surfaces it and polls to completion; the tty's real purposes — keep the single-use code off the persistent log, and serve as the fail-fast presence proxy — are preserved through the new channel.

## Done when
The TTY-gating question is settled either way with rationale recorded; if relaxed, avow can open a sitting from a non-tty context by surfacing its prompt as a yawp a relay completes, the human-present guarantee preserved; the rework for the accessor and the Leg-1 step is slated or folded.

## Cinched
The human-present premise holds — a human still authenticates each sitting; the device flow stays the mechanism; zero durable secrets beyond the payor's RBRO. This touches cinched paddock material (the human-present premise and the headless fail-fast membrane), so resolving it may surface an operator-sanctioned paddock amendment. Distinct from the programmatic no-human path — that is the foedus-reuse spine's, not this.

## Character
Design reconsideration plus a contained channel build at the shared accessor membrane (avow, used by every cloud tabtarget) and the BUYM yawp wire format; premise-touching, operator-owned.

### compear-tty-to-yawp (₢BfAAD) [abandoned]

**[260623-2235] abandoned**

## Explore: replace the /dev/tty compearance channel with a yawp

Compearance (rba_compear, the accessor's federation Leg-1, Tools/rbk/rba_auth.sh) writes the device-flow prompt (verification URL + user code) to /dev/tty and gates its headless fail-fast on a `: >/dev/tty` presence proxy.
That proxy blocks compearance from every non-tty channel (Claude Code's `!`, an agent relay, CI) — yet the device flow reads NOTHING from the terminal: it writes the prompt, then polls the IdP over the network; approval is out-of-band in a browser.

The tty is NOT needed for visibility — buc_step/buc_info already reach the screen (the tabtarget tees output to both screen and ../logs-buk/).
Its two real purposes: keeping the single-use code off the persistent log (the tee writes buc output to disk; /dev/tty reaches the screen only), and serving as a presence proxy for the fail-fast.

Explore emitting the prompt as a yawp (BUYM diastema wire format) so a relay can surface it and open an assize without a controlling terminal — preserving the spike's human-present guarantee (canon D2/D3) through the new channel rather than the tty proxy.

## Done when
Compearance can open an assize from a non-tty context by surfacing its prompt as a yawp a relay completes, the human-present guarantee preserved through the new channel — or a recorded decision to keep the tty with documented rationale.

## Character
Design exploration at the shared accessor membrane (rba_compear, used by every cloud tabtarget) and the BUYM yawp wire format.

### depot-freehold-roster (₢BfAAB) [abandoned]

**[260617-0938] abandoned**

Define the depot-freehold and its standing-citizen roster-verify, once ₣BZ has settled the citizen relationship under mantle-impersonation.

The freehold concept (minted in ₣BZ conversation, 260617): a freehold is the durable, deliberately-kept test installation we reuse day-to-day to dodge the create/destroy churn quota — the freehold/leasehold contrast against the ephemeral create→destroy lifecycle fixture.
It cross-cuts depot and foedus.

The foedus-freehold (org-level pool/provider trust) is nucleated in ₣BZ and carries no citizen roster: federation principals are the IdP's census, not a GCP-side list, so its verify is pool/provider/attribute-mapping config only.

The depot-freehold is this heat's work, deliberately deferred because its nature is unsettled until ₣BZ finishes redefining the citizen relationship under impersonation stature (mantle membership, not keyfile identity).
The standing-citizen roster lives here: muniments are (principal subject, mantle held), depot-scoped, read by the rehearse verb over the terrier — both built in ₣BZ.
This pace consumes that machinery ready-made; it does not rebuild it.

## Done when
The depot-freehold is defined against the post-impersonation citizen model, and its verify iterates the standing-citizen roster (rehearse over the terrier muniments) to confirm the expected citizens are present and hold the expected mantles.
The freehold/leasehold vocabulary is grep-gated and seated wherever depot test infrastructure names it — including any canonical→freehold rename, which carries blast radius across the gauntlet/skirmish/dogfight/onboarding suites and is a deliberate decision for this heat.

## Cinched
The roster head-count rides ₣BZ's rehearse/terrier/brevet; this heat does not rebuild the counting machinery.
Citizens are depot-scoped (muniment = principal + mantle); the foedus-freehold has no roster.
Rationale: Memos/memo-20260617-BZ-workforce-pool-constraints.md and the ₣BZ pace-design memo.

## Character
Design + fixture definition, gated on ₣BZ's citizen-relationship outcome; carries the cross-cutting freehold mint and the depot-side roster-verify.

### rbs0-manor-org-canon-scrub (₢BfAAE) [complete]

**[260701-1730] complete**

## Scrub canonical RBS0 of the org-level "manor" leak — disambiguate Manor (the holding / payor project) from the GCP organization it commands.

The terminal Fable review ratified the composite-Manor model (Manor = a holding that has-a payor/manor project + clustered resources + commands-an-org; the "deed / pure-org" sense is overturned). That overturned sense already leaked into canonical RBS0 prose, leaving the spec self-contradictory: one definition says Manor = one project, while other sites say "Manor-scoped (org-level)" / "every depot under the manor."

Authoritative model + leak-site list: Memos/memo-20260622-fable-review-queue.md (composite-manor-model, rbs0-canon-scrub-framing) and the review ledger. The ledger's site enumeration (RBS0 lines near 1890/1971/1980/1985/1331/2030/3486/3497 + RBSRF + claude-rbk-acronyms) is provenance — re-grep before trusting.

## 260630 coordination
₢BfAAv re-cuts the same RBS0 federation/manor region first (foedus≡provider, one manor pool). Adopt that framing here and avoid double-rewriting the same lines — this scrub layers on top of the re-cut, it does not contradict it.

## Done when
Each leaked site reads "Manor" where it means the project/holding and "organization"/"org-scoped" where it means the org; the formal Manor quoin carries the composite framing (has-a-project + commands-an-org); the Depot↔Manor cardinality (each Depot belongs to exactly one Manor; a Manor owns zero-or-more Depots) is stated; and the RBS0:1774 "payor manages infra without org-level permissions" contradiction is reconciled against the federation layer's org-admin requirement.

## Cinched
The composite-Manor model is ratified (operator-sanctioned 260622) and not reopened here — this is mechanical disambiguation, not a re-decision.

## Character
Mechanical spec disambiguation over a settled model; coordinate with ₢BfAAv (same federation region, re-cut first) and the FABLE-REANCHOR-RBS0 worklist to avoid dangling citations.

### fable-terminal-review (₢BfAAG) [abandoned]

**[260623-0911] abandoned**

## TERMINAL pace — model Fable works through the standing Fable-review queue: ratify-or-sweep every provisional mint made in her absence, and answer every deferred vocabulary/premise question.

This is the ₣Bf analogue of ₣BZ's terminal Fable review. The federation-evolution work proceeded under operator sanction while Fable was unavailable; the names and premise-touching calls made meanwhile are PROVISIONAL and eviction-sweepable.

Entry point: Memos/memo-20260622-fable-review-queue.md — Bucket 1 (provisional decisions to ratify-or-sweep) and Bucket 2 (open questions deferred to Fable: the foedus mint + cold-probe, the heed/assize remints, the mechanism-discriminator quoin, the RBST0 fresh quoin stem + theurge word + asterism call, the FABLE-REANCHOR worklist, jilt/gird ratification, the federation premise calls).

## Done when
Fable has worked through the queue memo: every Bucket-1 provisional decision is ratified or eviction-swept; every Bucket-2 question is answered; confirmed outcomes land in their spec homes; the queue memo retires.

## Cinched
Requires model Fable (the queue was assembled at provisional, operator-sanctioned density). Runs LAST — every other ₣Bf movement slates before it.

## Character
Terminal reconciliation under Fable; the pace that lifts the provisional-mint posture for ₣Bf. Requires model Fable.

### keycloak-programmatic-foedus-proof (₢BfAAH) [complete]

**[260622-1251] complete**

Prove end-to-end — ride or die, durable BCG bash, not a throwaway spike — that a Keycloak-minted OIDC token federates into GCP via uploaded JWKS and dons a mantle from a clean headless shell.
This is the one link the planning corpus only ever paper-confirmed: ₢BZAAA accepted "paper answer acceptable" for Keycloak-uploaded-JWKS viability and it was never harness-run, yet the whole Keycloak-automated test strategy rests on it.

Walk the chain: a LOCAL Keycloak crucible (kludged or pulled from upstream, never through the federated GAR/summon path — that one-way dependency is what keeps the chicken-and-egg from biting) mints a token via its non-interactive RFC 7523 JWT Authorization Grant (the supported path per the degenerate-personas memo — NOT the deprecated password grant the static-negative-test-rig memo still names); upload Keycloak's public JWKS to a programmatic workforce provider; STS exchange → generateAccessToken → bearer call against a depot API / don a mantle.

## Done when
A Keycloak-minted token, from a clean shell with no human, dons a mantle and makes an authorized depot-API call.
The uploaded-JWKS refresh coupling (Keycloak signing-key change → re-upload) is characterized.
Result recorded: works → it feeds the spec-first recast; fails → it is a real blocker, surfaced not silently absorbed.

## Cinched
Ride or die — durable BCG bash, not throwaway (the config-model memo's decision-economy ruling).
Keycloak is the preferred minter; self-signed-caged is the fallback only if the grant disappoints.
Proves the PROGRAMMATIC arm only — the interactive device-flow arm stays human-gated by construction.

## Character
Ride-or-die foreign-API proof; expect Palisade surprises; characterize the exact signature rather than tolerate it silently.

Sources: degenerate-personas memo (RFC 7523 grant, can/cannot-prove boundary), static-negative-test-rig memo (degenerate-IdP spectrum, chicken-and-egg-doesn't-bite), config-model memo (uploaded-JWKS, decision-economy, the 260622 fork resolution).

### token-mint-failfast-terminal-iam (₢BfAAm) [abandoned]

**[260702-1332] abandoned**

Drafted from ₢BiAAB in ₣Bi.

Make the federation accessor fail fast on deterministically-terminal credential failures rather than burning the SA-propagation retry budget on dead credentials — re-derived against the post-federation auth path.

## Demolished — re-derive, do not extend
The original drafting targeted `rbgo_get_token_capture` in `rbgo_OAuth.sh`, its "Discriminate failure" / `TEMP-FORENSIC` blocks, and the ~104s/10-attempt SA-propagation loop.
This heat's own federation recast demolished all of it: `grep rbgo_get_token_capture` is empty, the file is `rbgo_oauth.sh`, those blocks are gone.
Do not hunt or extend them — sibling ₢BfAAl already carries this federation-vs-keyfile warning; this docket now states it too.

## The surviving surface
The token-mint / don home is `rba_auth.sh` (`rba_token_capture`, `rba_avow`, `rba_don_capture`); `rba_don_capture` is already single-attempt fail-fast by design — the 403 admission-deficit is logged and returned, never retried as a propagation race.
The retry-budget consts (`RBGC_SA_KEY_CONSUMER_RETRY_*`, `rbgc_constants.sh`) are vestigial — zero source consumers today.
The cold access probes are `rbgv_payor_oauth_probe` and the live `rbgv_check_*` family (`rbgv_probe.sh` / `rbgv_cli.sh`).

## Done when
A determination, against the settled federation accessor, of whether a deterministically-terminal credential failure can still burn a propagation budget:
call-context fail-fast added at the surviving touchpoint where it can, or the pace retired with that finding recorded where the surviving code lives.
Tolerate-or-retire.

## Cinched
The disambiguator is call context (did a mint just happen?), not the response body.
Do NOT reintroduce or extend the demolished keyfile SA-propagation loop; keep genuine post-mint propagation tolerance wherever it survives.

## Character
Surgical re-derivation gated on one design question — judgment, not mechanical.
Shares its surviving-touchpoint question with sibling ₢BfAAl; mount adjacent or fold.

### payor-install-finish-manor-guidance (₢BfAAn) [abandoned]

**[260702-1135] abandoned**

Drafted from ₢BiAAJ in ₣Bi.

The payor-install verb closes by prescribing one hard-coded next step — "Next: levy the depot" — and the fix is to make its tail point the operator at finishing the manor rather than at a single federation/depot command.

## Character
Reconfirm-then-shape, deferred on purpose: the mechanical change is small, but the canonical manor-completion sequence it must point at is mid-evolution in the federation heats and gated on Fable's vocabulary mints, so every premise below is a hypothesis to re-derive from the live tree at mount, not authority. The timing is the whole point of parking it here.

## The need
`rbgp_payor_install` (Tools/rbk/rbgp_payor.sh) ends by hard-coding a pointer at the depot-levy tabtarget.
A leaf credential verb should not route workflow: the pointer is blind to the credential-refresh path (where depots already exist), it skips the manor-affiance step that actually finishes a manor, and levy is a per-depot act, not a manor-completion act.
The operator finishing payor-install wants "here is how to finish setting up your manor" — not federation-flavored, not a single command.

## Shape (verify before adopting — see Reconfirm)
- Stop the verb prescribing levy; have its tail report state and point at the manor onboarding handbook (rbw-Op) as the sequence of record — the not-federation-dedicated "finish your manor" home.
- Clean the post-success "Verifying ... rbrp.env" tail in the same pass: give it an honest label (it reconciles the committed manor-identity regime against what the install just discovered), and drop any arm an earlier step already enforces with a hard die — suspected unreachable: the billing and client-id checks. Confirm, do not assume.
- Separable, larger: weave the affiance step into the manor onboarding handbook (Tools/rbk/rbh0/rbhopw_payor_wrapper.sh) as a manor-completion step — "connect your manor to its identity provider" — sequenced establish → install → affiance → then provision depots.

## Reconfirm premises first
The federation surface this points at is mid-evolution in ₣Bf and ₣BZ, with vocabulary gated on Fable.
Before writing any guidance prose, re-derive from the live tree:
- Does the affiance verb still exist by that name and colophon, or did a Fable mint rename it (the affiance/heed and assize remints are live ideas in ₣Bf)?
- Has the manor-establishment reshape landed — is there now a unified manor-provision op (the terrier founding-home shape in ₣Bf) instead of a bare affiance, changing what "finish the manor" means?
- What is the canonical manor-completion sequence at mount, and does the handbook already reflect it?
Do not wire prose that contradicts whatever ₣Bf/₣BZ landed; if the sequence is still unsettled, the safe subset is the leaf-verb-stops-routing move alone.

## Cinched
The leaf verb must not hard-code "the next command"; the manor handbook is the sequence of record.
The post-install guidance is framed as manor-completion, never as a federation-specialist detour.

## Done when
Payor-install no longer prescribes levy; its tail reports state and points the operator at finishing the manor, with the "Verifying" block honestly labelled and free of unreachable checks.
The manor-completion sequence pointed at is confirmed against the live tree at mount, not taken from this docket.
Whether the handbook affiance-weave lands here or splits to its own pace is decided at mount.
Fast suite green.

### foedus-pool-state-classifier (₢BfAAs) [abandoned]

**[260630-1428] abandoned**

## Spec of needed change
Three operations interrogate a GCP workforce pool and must classify its state identically — live, soft-deleted (the ~30-day purge window), or absent: affiance (creates pools and must refuse a soft-deleted id), descry (reports a foedus's pool health), and canvass (enumerates foedera).
The classification lives inline today in affiance (rbgp_payor.sh, the workforcePools.get 200/404/DELETED branch).
Extract it into one shared helper so the manager (affiance) and the watcher (descry) share a single notion of soft-deleted rather than re-deriving it and drifting apart.
While here, give affiance its missing live-pool guard: on a live pre-existing pool, affiance FAST-FAILS ("exists — reuse, don't recreate") instead of silently leaving it in place, completing affiance's tri-state — 404 create / live fast-fail / soft-deleted refuse-and-rotate.

## Done when
The live/soft-deleted/absent classification is defined once in a shared helper; affiance consumes it in place of its inline branch and fast-fails on a live pre-existing pool (no silent leave-in-place); the helper is the mandated home for descry's pool-state read (descry's config-drift "valid" check rides on top of it) and any future enumerator's.

## Character
Mechanical extraction plus re-wire — the soft-delete semantics already exist in affiance and are not being redesigned; the live-pool fast-fail is a small added guard. The descry-side consumption is a forward constraint, satisfied when descry is built.

### terrier-band-discrimination (₢BfAAt) [complete]

**[260701-2024] complete**

Give the terrier data layer (rbgft) precise, testable failure discrimination, closing the wrong-reason hole on its error paths (today: generic buc_die / bare nonzero).
Mint a terrier rejection band in the bubc precision-exit-code block (one code per gate, per BCG); convert rbgft's error-path buc_die to buc_reject of that band; add a buo-sprued fault-injection seam in the rbuh HTTP layer so a forced HTTP code can drive each error path (the regime-poison analogue for HTTP, which rbuh lacks today).
The idempotency SUCCESS paths (412 to present, 404 to absent) stay exit-0 stdout dispositions, untouched.
The theurge poison fixture that asserts these bands is the sibling pace on ₣Bl.

## Done when
rbgft error paths reject in a named bubc terrier band (no generic buc_die); the band is minted in the bubc tinder block (one code per gate); rbuh carries a fault-injection seam that forces an HTTP code for negative testing.

## Character
Data-layer hygiene (BCG precision-band) plus a small rbuh test seam; the band semantics are new but follow the established band discipline.

### terrier-muniment-provider-dim (₢BfAAy) [complete]

**[260702-0153] complete**

Add a provider dimension to the terrier muniment — so co-resident providers under one pool no longer conflate in a manor-wide rehearse, and a pool-independent subject can't collide silently (the membership study's structural-collision finding).

## Spec of needed change
The muniment records only (subject, mantle) today and is pool/provider/foedus-blind (rbgft_terrier.sh; RBSTR). Under co-resident providers, two foedera admitting the same human write a byte-identical muniment while holding two distinct grants. Add a provider segment:
- Key composer + body builder (rbgft_terrier.sh) gain a provider dimension; engross/expunge signatures + the lister emit thread it; the rbgp_ cores already read RBRF_PROVIDER_ID, so the source is in scope.
- On-disk MIGRATION: existing depot/mantle/subject objects have no provider segment — re-key or migrate the live terrier.
- Spec home is RBSTR/RBGFT (the terrier ACCESS layer), NOT the finisher subdoc.

## Done when
The muniment carries a provider dimension; engross/expunge/peruse/rehearse thread it; a manor-wide rehearse distinguishes co-resident foedera; the live terrier is migrated; the RBSTR/RBGFT contract is updated.

## Cinched
Per-individual muniment (1:1 record↔grant mirror preserved — depends on the per-individual, not group, membership choice).
Spec home = RBSTR/RBGFT, not the finisher.

## Character
Contained code membrane (one terrier module) plus a one-time on-disk muniment migration; the migration is the real cost.

### terrier-noun-sheaf-carve (₢BfAA4) [complete]

**[260702-0410] complete**

Carve the terrier noun-internals out of RBS0 into the companion normative sheaf RBSTN,
so the terrier quoin reads as its definition plus two include::s (noun, access) —
symmetric with the realm→RBSFK and access→RBSTR sheaf precedents.

## Spec of needed change
Create `Tools/rbk/vov_veiled/RBSTN-terrier_noun.adoc` mirroring the RBSTR/RBSFK sheaf apparatus:
copy RBSTR's first two lines exactly (the `//axvd_sheaf axd_normative` marker + the rendered imprimatur banner sentence),
add a comment header naming the sheaf's scope (the terrier noun internals — resource contract, muniment shape, key format; access stays RBSTR),
then a `== Terrier noun` heading (leveloffset in the include renders it one level down, matching RBSTR's `== Terrier access`).

The sheaf body is four blocks relocated VERBATIM from RBS0, in this order:
(1) the terrier quoin's resource-internals continuation block (begins "The terrier's *resource* internals are settled");
(2) the terrier quoin's muniment-internals continuation block (begins "The terrier's *muniment* internals are settled");
(3) the muniment quoin's provider collision-rationale (the sentences from "The provider dimension is load-bearing" through "…the foedus it was admitted through.");
(4) the terrier quoin's access pointer sentence ("Its access sub-operations … are homed in the Terrier access subdoc below." — still true, RBSTR follows).
Drop the `+` list-continuation markers — list syntax, not content; every sentence moves with zero word changes.

In RBS0: the rbtf_terrier quoin keeps its anchor + opening definition sentence ("A polity's durable … Manor-homed bucket."),
followed by a new `//axvd_cartouche axd_normative` + `include::RBSTN-terrier_noun.adoc[leveloffset=+1]`,
then the existing cartouche + RBSTR include, unchanged.
The rbtf_muniment quoin keeps its anchor and definitional sentences ("A single … proof of rights." and "A muniment names a holding, not a secret …"); only the collision-rationale between them moves.

Update RBSTR's two stale self-pointers that home the noun internals "in the RBS0 terrier quoin"
(its header-comment tail and its "Settled — the terrier noun's internals (in RBS0)" section)
to point at the RBSTN sheaf instead — pointer maintenance tracking the relocation, not rewording.
Register RBSTN in claude-rbk-acronyms.md beside RBSTR.

## Done when
RBSTN exists with the sheaf apparatus and the four verbatim blocks;
RBS0's terrier quoin carries only its definition plus the two cartouched includes (noun, access);
the muniment quoin retains only its definitional sentences;
every attribute reference used in RBSTN is registered in RBS0's mapping section (grep-verify);
all quoin anchors and axvd_ markers resolve unchanged;
RBSTN is registered in the acronym file.

## Cinched
Fork resolved (Fable 260702): "byte-faithful" means the normative sentences relocate verbatim — zero rewording — NOT a byte-identical render;
the render legitimately gains the sheaf apparatus (banner sentence, one heading, cartouche), exactly as the RBSFK carve did.
RBSTN / `RBSTN-terrier_noun.adoc` is the minted acronym and filename (grep-clean 260702); do not re-mint.
The muniment collision-rationale moves (the spec-of-change enumeration governs); its definitional sentences stay.
Do NOT fold the noun into RBSTR; no RBSTR edits beyond the two pointer updates.
If the split demands any further rewording to cohere, stop and surface it.

## Character
Mechanical AsciiDoc relocation; the care is verbatim sentence preservation and resolving refs, not judgment.

### await-thg-registry-relocation (₢BfAAq) [complete]

**[260702-0348] complete**

BARRIER — cross-clone wait. No code work of its own.
This pace cannot complete until ₢BlAAY (heat ₣Bl — the terrier-poison fixture, ₣Bl's LAST registry write under the 260630 rework: it adds the poison fixture to the roster AND replaces the terrier-atomicity fixture that ₢BfAAz retires) is complete, pushed to origin, and origin merged back into this clone.
The next pace ₢BfAAz (terrier-scaffold-retirement) retires the terrier fixtures from the registry, so it must apply that delta against the FULLY-restructured roster — including ₢BlAAY's replacement fixture — not against a stale roster.
Positioned after ₢BfAAt (which ₢BlAAY depends on for its bands), so the terrier zigzag ₢BfAAt → ₢BlAAY → ₢BfAAz resolves in order without a cycle.

## Done when
₢BlAAY is wrapped and pushed to origin, and this clone has pulled origin so the fully-restructured fixture/suite registry (incl. the terrier-poison replacement) is present here before ₢BfAAz retires the old fixtures.

## Character
Cross-clone barrier; retargeted 260630 from the stale ₢BlAAM to ₣Bl's actual last registry write ₢BlAAY, repositioned after ₢BfAAt for the terrier zigzag. The retirement it gates is ₢BfAAz (split out of the former ₢BfAAF).

### terrier-scaffold-retirement (₢BfAAz) [complete]

**[260702-0405] complete**

Retire the interim terrier scaffold — the rbw-dt/dT tabtargets, the terrier-scaffold / terrier-atomicity fixtures (picket/echelon), and their rbtdrm_manifest.rs name-consts + colophon-dependency-map arms — now that the finisher provisions the terrier for real.

## Spec of needed change
Split out of the finisher (₢BfAAF) because it genuinely depends on the fully-restructured fixture registry (behind the now-lifted ₢BfAAq cross-clone barrier), while the finisher itself does not.
- Retire rbw-dt / rbw-dT, the terrier-scaffold + terrier-atomicity fixtures, and the rbtdrm_manifest.rs RBTDRM_FIXTURE_TERRIER_* consts + colophon-dependency-map arms (the sixth ₣Bl-owned file — include it so a hot-file-only read doesn't miss it).
- Key the terrier-atomicity retirement off ₣Bl's polity-denial fixture (₢BlAAY): its terrier arc — folded in and renamed from the old mantle-denial fixture — now carries the terrier-band coverage that the retired proof held, so retiring terrier-atomicity leaves 2 terrier picket fixtures, not 3. There is NO standalone fixture named terrier-poison (₢BlAAY's earlier plan; it renamed/folded instead). Coordinate off the polity-denial terrier arc; do not duplicate coverage.
- Update the RBS0 terrier cross-project line (the finisher makes the "interim scaffold" framing false).

## Done when
The rbw-dt/dT scaffold + its two fixtures are retired/migrated onto the finisher, including the rbtdrm_manifest.rs arms; the RBS0 terrier line is corrected; gauntlet stays green (no terrier fixture there).

## Cinched
Rode behind ₢BfAAq (the fully-restructured fixture registry is now merged here).
The terrier-band coverage lives in ₢BlAAY's polity-denial terrier arc (renamed from mantle-denial), NOT in a fixture named terrier-poison — none was landed. Coordinate the terrier-atomicity retirement off that arc; do not duplicate.

## Character
Mechanical retirement across the registry + manifest + RBS0 line; the cross-clone barrier (₢BfAAq) that gated it is now lifted.

### guide-to-tabtarget-automation-lift (₢BfAA2) [complete]

**[260702-1204] complete**

## Character
Design + cross-cutting surgery across the guide/tabtarget membrane — Fable-suited (strong reasoning, few-error strange surgery).

## Spec of needed change
Migrate the automatable manor-setup steps out of the human payor guide into the tabtarget layer (chiefly the manor-setup finisher, rbgp_manor_instaurate), shrinking the guide to the irreducible pre-credential genesis — because humans follow instructions poorly and idempotent scripts raise adoption.
Sorting rule — the credential boundary: before the payor holds its first OAuth credential there is no token to automate with (first project, OAuth client, billing linkage — irreducibly manual); everything downstream of that first credential is automatable and belongs in a tabtarget.

Known movable work (audit rbhpe_establish / rbhpf_entra / the onboarding tracks for more):
- Enable-required-APIs (RBSPE step 3, the serviceusage set) folds into the finisher as a step-0 enable-then-wait via the existing rbge_api_enable idempotent helper — which also resolves the finisher's own dependency on those APIs being on.
- Re-aim the onboarding handoff so payor-install points at the finisher, then depot-levy (guide -> install -> instaurate -> levy); this SUBSUMES the paddock's "payor-install manor-completion next-step" re-derivation — do not double-slate it.
- Investigate billing-linkage automatability (API exists; the payor OAuth may lack billing-account scope — verify, do not assume).

## Done when
The human guide holds only the irreducible pre-credential genesis;
every automatable manor-setup step runs from a tabtarget;
the guide -> install -> instaurate -> levy handoff is wired end-to-end;
each migrated step owns its failure modes (GCP eventual-consistency, enable-then-propagate, partial-run re-entry) so a half-founded manor is safely re-runnable, never a worse state than a checklist;
gauntlet stays green.

## Cinched
The credential boundary is the sorting rule.
The genesis (first payor project + OAuth client) is irreducibly manual, stays in the guide.
The finisher hosts the automatable substrate (ensure-exists, idempotent).

### terrier-hygiene-sweep (₢BfAA5) [complete]

**[260702-1321] complete**

## Character
Bounded terrier-hygiene design-and-build — the need is witnessed; the mechanism is open (ride depot unmake / freehold churn, or a standalone payor-credentialed verb).

## The debt
The payor-grain terrier accretes muniments no live admission path owns.
Unmaking a depot project runs no terrier sweep, so a retired levy's polity slice persists as orphans.
Muniment-schema churn strands old-shape objects the current verbs cannot expunge
(witnessed live at the provider-grain re-cut: a provider-blind stray aliased the parley probe's roll assertion until the scaffold-purge migration removed it — the accretion path stays open).
rehearse's roll is depot-attributed (RBSPO depot-attributed emission), so strays are visible and attributable — but nothing removes them.

## Done when
A ruled sweep path exists and is exercised:
unmade-depot slices and dead-schema strays can be removed from the terrier,
so every standing roll line names a live depot in the current muniment schema.

## Cinched
Terrier hygiene is this heat's work, not the parley probe's (ruled at the theurge heat's read-surface pace).
rehearse stays a pure read; the sweep is a separate mutating act, never folded into a read.

### paddock-scrub-and-retire (₢BfAA8) [complete]

**[260705-0806] complete**

Terminal pace: disposition every live strand of this heat's paddock, then retire the heat.
The build work is complete; the paddock still holds live planning content that must be re-homed or dispositioned before archive.
Inventory taken 260702 — verify at mount rather than re-derive:

- To RBSHR (operator-owned parked ideas, premise-touching): the governor's-role idea (sanctioned-set affiliation bound), the payor-health-assurer representation fork, further per-vendor federation setup guides.
- Re-home: the federation vocabulary census (most words are built RBS0 quoins already; the instate/descry/canvass records belong with the theurge stream); the escheat orphan-arm gauntlet-clause (to the RBSME spec or the fixture notes); the colophon casing convention (uppercase second letter marks cloud-mutating) if not already recorded at the zipper or BUS0.
- Drain: the 260630 model-reversal record into Memos/memo-20260623-Bf-heat-memories.md, per the paddock's own Sources note.
- Verify, then slate-or-decline with the operator:
  the attach-caged-subject spine unit (brevet the caged Keycloak subject onto a test depot via the admission verbs, yielding the second roster citizen — the Keycloak proof reached token acquisition, admission unconfirmed);
  the rbgft data-layer unit test (terrier 412/404 idempotency + escheat survey verdicts — paddock calls it theurge-stream-homed but that stream has no remaining paces);
  the mantle-access probe relocation out of the access colophon family (the colophon-homing plan's unexecuted line);
  the payor-install next-step re-aim at the manor finisher (probably landed via the guide-automation pace — confirm);
  the build-bucket scrub coupling said to live in the sibling MVP stream — confirm it is homed there or itch it.
- Already re-homed, no action: the release-suites credential preamble (slated at the tail of ₣Bq); the credential-fault disposition and the sitting-runway feature (₣Bq whole).

Then retire this heat as the final act, operator-confirmed.

## Done when
Every paddock idea is dispositioned — a pace elsewhere, RBSHR, an itch, or an explicit decline recorded;
the re-homings and the memo drain are done;
the five verifications resolved with the operator;
the heat retired.

## Cinched
Premise-touching ideas move to RBSHR as operator-owned parked ideas — never silently dropped, never quietly built.
Retirement is the last act, after every strand has a home.

## Character
Disposition sweep with the operator in the loop; judgment on homes, mechanical on moves.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 S accessor-vocabulary-pass
  2 R suite-asterism-rename
  3 h heat-integrity-and-coverage-audit
  4 k cross-heat-parallelization-split-study
  5 i paddock-refocus-and-drain
  6 O federation-spec-first-recast
  7 T foedus-reuse-design
  8 b butcfc-seed-previous-wrapper
  9 I federation-buildout-orientation
  10 N rfc7523-grant-and-baked-realm
  11 M affiance-programmatic-mechanism-arm
  12 v onepool-spec-recut
  13 u clean-tree-gate-precision-variant
  14 p await-thg-zipper-baton
  15 0 rbrw-regime-buildout
  16 x manor-teardown-force-delete
  17 w membership-composer-recut
  18 F terrier-founding-home-finisher
  19 1 affiance-jilt-provider-recut
  20 3 affiance-jilt-folio-recut
  21 L rbxk-keycloak-orchestrator
  22 6 programmatic-self-supply-spec-recut
  23 K programmatic-accessor-rba-arm
  24 7 accessor-foedus-selector-resolve
  25 J entra-federation-setup-guide
  26 C settle-federation-config-design-forks
  27 A heed-headless
  28 E rbs0-manor-org-canon-scrub
  29 H keycloak-programmatic-foedus-proof
  30 t terrier-band-discrimination
  31 y terrier-muniment-provider-dim
  32 4 terrier-noun-sheaf-carve
  33 q await-thg-registry-relocation
  34 z terrier-scaffold-retirement
  35 2 guide-to-tabtarget-automation-lift
  36 5 terrier-hygiene-sweep
  37 8 paddock-scrub-and-retire

SRhkiOTbINMvup0xwF13L6K7JCAEHty4qz258
x··x·xx··x·······x·x·x·x···x··xx·x·x· claude-rbk-acronyms.md
xx········x···xx·xxxx······x··x··xxx· rbgp_payor.sh
x····xx··x·x·····x···x····xx··xx·x·x· RBS0-SpecTop.adoc
xx············xx·xxxx···x········xxx· claude-rbk-tabtarget-context.md, rbz_zipper.sh
x·············xx·x··x··xx····x···x·x· rbtdgc_consts.rs
x·············xx·x·x···x···········x· rbgp_cli.sh
x····xx··x·x········xx··············· RBSRF-RegimeFederation.adoc
·x··········x·x···x····x·····x······· rbcc_constants.sh
··············x···xx···x·········x··· rbtdrv_patrol.rs
x····x·····x·······xx················ RBSMA-manor_affiance.adoc
xx············x·······x···x·········· rba_auth.sh
·x··········x················x·····x· bubc_constants.sh
x·········x···x·······x·············· rbrf_regime.sh
x····x·····x·······x················· RBSMJ-manor_jilt.adoc
······························xx···x· RBSTR-Terrier.adoc
·····························xx····x· rbgft_terrier.sh
·················x···············xx·· RBSMS-manor_instaurate.adoc
·········x··········x·······x········ rbrn.env
···xx·······························x memo-20260623-Bf-heat-memories.md
x·············x········x············· rbgv_cli.sh
x·········x···x······················ rbrf.env
x·····x····x························· RBSRR-RegimeRepo.adoc
xx·······························x··· rbtdrm_manifest.rs
xx···························x······· BUS0-BashUtilitiesSpec.adoc
···································xx RBSME-manor_escheat.adoc
·························x··x········ memo-20260618-Bf-federation-config-model.md
·······················xx············ rbhpf_entra.sh
····················x·x·············· rbrf.env.template, rbxk_keycloak.sh
··············x········x············· rbfc0_cli.sh, rbfd_cli.sh, rbfl0_cli.sh, rbfr_cli.sh, rbfv_cli.sh, rbgg_cli.sh, rbld0_cli.sh, rbob_cli.sh
··············x···x·················· rbof_foedus.sh
·········x··················x········ fdkyclk-proof.sh, fdkyclk-realm.json
·········x···········x··············· RBSFK-foedus_realm.adoc
······x····x························· RBSFD-foedus_descry.adoc
·····x···············x··············· RBSFA-foedus_acquire.adoc
·····x···x··························· RBSFE-foedus_establish.adoc
·x···························x······· CLAUDE.md
x··································x· RBSPO-terrier_rehearse.adoc
x·································x·· rbhopw_payor_wrapper.sh
x··········x························· RBSPB-citizen_brevet.adoc
xx··································· claude-rbk-theurge-ifrit-context.md, rbtdrc_crucible.rs, rbtdrf_fast.rs
····································x RBSHR-HorizonRoadmap.adoc
···································x· rbgb_buckets.sh, rbw-mE.PayorEscheatsTerrier.sh
··································x·· RBSPE-payor_establish.adoc, RBSPI-payor_install.adoc, rbhpe_establish.sh
·································x··· rbtdra_almanac.rs, rbw-dT.TerrierProof.sh, rbw-dt.TerrierScaffold.sh
·······························x····· RBSTN-terrier_noun.adoc
·····························x······· rbuh_http.sh
····························x········ Dockerfile, fdkyclk-teardown.sh, rbrv.env, rbw-cC.Charge.fdkyclk.sh, rbw-cQ.Quench.fdkyclk.sh
························x············ rbhp0_cli.sh, rbw-gPF.PayorFederationEntra.sh
·······················x············· rbrf_cli.sh
······················x·············· fdkyclk-client-secret.txt
····················x················ .gitignore, rbdgl_federation-login-dark.svg, rbdgl_federation-login.svg, rbdgm_federation-seam-dark.svg, rbdgm_federation-seam.svg, rbdgs_federation-setup-dark.svg, rbdgs_federation-setup.svg, rbw-qjK.KeycloakSetup.sh, rbw-qjQ.KeycloakTeardown.sh, rbxk_cli.sh
···················x················· memo-20260702-jjk-gallops-coronet-collision-incident.md
·················x··················· RBSMR-manor_raze.adoc, RBSMS-manor_found.adoc, rbgc_constants.sh, rbw-mF.PayorFoundsManor.sh, rbw-mI.PayorInstauratesManor.sh
···············x····················· rbk-prep-release.md, rbw-mR.PayorRazesManor.sh
··············x······················ rbrw.env, rbrw_cli.sh, rbrw_regime.sh, rbw-rwr.RenderWorkforceRegime.sh, rbw-rwv.ValidateWorkforceRegime.sh
············x························ bug_git.sh
···········x························· RBSRW-RegimeWorkforce.adoc
·········x··························· fdkyclk-asserter-key.pem
·······x····························· butcfc_facts.sh
······x······························ RBSFI-foedus_instate.adoc
·····x······························· MCM-MetaConceptModel.adoc
···x································· memo-20260624-Bf-split-study.md
·x··································· ACG-AllocationCodingGuide.md, RBSLE-lode_ensconce.adoc, buv_validation.sh, claude-buk-core.md, rblds_spine.sh, rbldv_immure.sh, rbq_qualify.sh, rbtdre_engine.rs, rbtdrf_handbook.rs, rbtdri_invocation.rs, rbtdrn_conformance.rs, rbte_engine.sh
x···································· RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc, RBSGS-GettingStarted.adoc, RBSPA-citizen_attaint.adoc, RBSPG-governor_gird.adoc, RBSPU-citizen_unseat.adoc, README.md, rbgi_iam.sh, rbtdrk_depot.rs, rbtdro_onboarding.rs, rbtdtk_freehold.rs

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 293 commits)

  1 z terrier-scaffold-retirement
  2 L rbxk-keycloak-orchestrator
  3 4 terrier-noun-sheaf-carve
  4 2 guide-to-tabtarget-automation-lift
  5 6 programmatic-self-supply-spec-recut
  6 K programmatic-accessor-rba-arm
  7 5 terrier-hygiene-sweep
  8 7 accessor-foedus-selector-resolve
  9 8 paddock-scrub-and-retire

123456789abcdefghijklmnopqrstuvwxyz
·xx································  z  2c
····xxx··xxxx······················  L  7c
·······xx··························  4  2c
··············xx···················  2  2c
·················x·x···············  6  2c
····················x·x············  K  2c
·······················x·x·········  5  2c
·····························xx····  7  2c
·································xx  8  2c
```

## Steeplechase

### 2026-07-05 08:06 - ₢BfAA8 - W

Terminal disposition sweep, inventory verified at mount rather than trusted: six strands had already resolved (memo drain done 260630; casing convention zipper-homed; payor-install re-aim landed; build-bucket scrub complete in the MVP stream; credential preamble complete in the sitting-lifecycle heat; vocabulary census all quoin-built, quash superseded by novate/espy). Moved the three premise-touching ideas to RBSHR as operator-owned entries and rewrote its stale keyfile-era Operator-federation entry IMPLEMENTED; homed the escheat orphan-arm proof status as an RBSME NOTE; drained the terminal-dispositions ledger to the heat-memories memo; declined the rbw-am test-bucket relocation. Mid-sweep the operator nominated rebaseline heat ₣Bs (iterate-before-destroy cinch, 8 paces): the escheat orphan-arm live-proof, the rbgft data-layer test residue, and the attach-caged-subject admission graduated there as paces instead of itches. Every paddock strand homed; heat retires.

### 2026-07-05 08:05 - ₢BfAA8 - n

Paddock-scrub dispositions for heat retirement: RBSHR gains the three operator-owned premise-touching entries (governor sanctioned-set affiliation bound, payor health-assurer representation fork, per-vendor federation setup guides) and its keyfile-era Operator-federation entry is rewritten IMPLEMENTED against the built foedus/avow/sitting surface; RBSME gains the orphan-arm proof-status NOTE (dead-schema arm live-proven 260702, orphan arm proven at next real depot churn, synthetic payor-direct writers stay retired); the heat-memories memo gains the terminal-dispositions entry (census closed all-built with quash superseded by novate/espy, the rbw-am relocation declined, the escheat proof and rbgft residue and attach-caged-subject admission graduated to the new rebaseline heat, the already-done verifications recorded); the two transient itch entries removed as graduated to paces.

### 2026-07-02 15:19 - Heat - S

paddock-scrub-and-retire

### 2026-07-02 15:08 - Heat - d

batch: 1 reslate

### 2026-07-02 13:54 - ₢BfAA7 - W

Made the singleton federation accessor selector-derived: retired the constant-folded RBCC_rbrf_file for a post-rbrr resolution off RBRR_ACTIVE_FOEDUS. rbcc_constants.sh gained rbcc_rbrf_file_capture (composes <moorings>/<foedera>/<foedus>/rbrf.env, optional explicit-foedus arg) and rbcc_source_active_rbrf (guarded resolve-and-source helper, BCG two-line capture pattern), and dropped RBCC_rbrf_file from rbcc_emit_consts. Eleven bash consumers re-pointed to the one-line helper; rbrf_cli (rbw-rfv/rfr) gained rbrr.env sourcing so it validates the ACTIVE foedus through the standard path; the Entra guide resolves the entrada path explicitly; the Rust patrol (zrbtdrv_active_provider_capture) mirrors the compose from the selector and RBTDGC_RBRF_FILE dropped from the regenerated consts. Proven credlessly (build, shellcheck 228, reveille 117/0, regime-poison 29/0; rbw-rfv/rfr green on entrada) AND end-to-end via the operator-gated facility proof: rbw-qjK -> instate keycloak -> rbw-rfv validates the programmatic regime through the standard path (no synthetic file) -> rbw-aa acquires a federated token unattended via the RFC 7523 grant (392 chars) -> re-instate entrada, clean tree. The mechanism-gate mirror (entrada interactive-PASS/programmatic-SKIP vs keycloak programmatic-PASS/interactive-SKIP through one resolver) is the proof. Facility torn down. Code notched at cad03870c.

### 2026-07-02 13:42 - ₢BfAA7 - n

Make the singleton federation accessor selector-derived: retire the source-time RBCC_rbrf_file constant (constant-folded to rbef_entrada) for a post-rbrr resolution off RBRR_ACTIVE_FOEDUS. rbcc_constants.sh gains rbcc_rbrf_file_capture (composes <moorings>/<foedera>/<foedus>/rbrf.env from the selector, optional explicit-foedus arg) and rbcc_source_active_rbrf (the guarded resolve-and-source helper, BCG two-line capture pattern), and drops RBCC_rbrf_file from the rbcc_emit_consts projection. Eleven bash consumers (rbld0/rbfl0/rbob/rbfv/rbfd/rbfr/rbfc0/rbgg/rbgv x2/rbgp active-branch) re-point to the one-line helper. rbrf_cli (rbw-rfv/rbw-rfr) gains rbrr.env sourcing so it resolves and validates the ACTIVE foedus through the standard path (no synthetic-file workaround); the Entra guide (rbhpf_entra) resolves the entrada path explicitly since it teaches the interactive foedus. The Rust patrol (zrbtdrv_active_provider_capture) mirrors the compose from RBRR_ACTIVE_FOEDUS instead of the frozen RBTDGC_RBRF_FILE (matching what it already does 2x in the same file); RBTDGC_RBRF_FILE drops from the regenerated rbtdgc_consts.rs. Ceiling honored: the accessor resolves the instated foedus, no end-to-end proof yet. Build green, shellcheck 228 clean.

### 2026-07-02 13:32 - Heat - T

token-mint-failfast-terminal-iam

### 2026-07-02 13:32 - Heat - T

federation-credential-fault-disposition

### 2026-07-02 13:32 - Heat - d

batch: 1 reslate

### 2026-07-02 13:21 - ₢BfAA5 - W

Terrier-hygiene sweep, contract-first and live-proven on real debt. The mechanism fork resolved to a standalone payor-credentialed manor verb (dead-schema strays and already-orphaned slices exist independent of any unmake event; cross-slice deletion exceeds governor folder scope). Mint: escheat — manorial reversion of ownerless holdings, grep-clean on all durable surfaces (the bedrock-quire memo pre-claim adjudicated by operator go). Spec RBSME seats the contract: plan-then-confirm with an already-clean idempotent exit-0 short-circuit, the depot-liveness rule (ACTIVE-with-anchor live; DELETE_REQUESTED/403/404 dead — waiting out the purge window would leave fresh unmakes standing; ACTIVE-without-anchor flagged-and-kept), provider-liveness deliberately out of scope (jilt churn never erases admission records), and the raw-grain data layer beneath the RBSTR muniment contract. Wired: RBS0 rbtgo_manor_escheat after raze, RBSTR hygiene-access NOTE, RBSPO orphan-removal pointer, acronym entry. Code: BUBC_band_escheat=114 (last free code, own gate — hygiene refusal, never the admission-path gates); rbgb_managed_folders_capture; rbgft escheat_survey (tolerant classify: key shape, mantle enum, body JSON, rbgft_ fields, key-body agreement; benign-vanish skip; per-run forensics) + escheat_expunge_raw; rbgp_manor_escheat with depots/liveness/plan helpers (verify re-runs the plan against held liveness verdicts and requires zero strikes); CLI arm on the depot_list posture; rbw-mE enrolled and generated files re-derived. BCG audited by checklist per operator directive; shellcheck 227 clean; reveille 117/0. Live exercise found REAL debt: a provider-blind 3-segment retriever stray the scaffold-purge migration missed — baseline run displayed the plan and refused on EOF (gate proven), the seam-skipped run swept it, verify green, second run Terrier-clean exit 0, rehearse roll all-live in current schema. Exercise posture ruled mid-pace: no synthetic payor-direct stray writer (that shape stays retired) — the orphan-slice arm (dead-depot verdict + folder purge) awaits the next real depot churn per the instaurate billing-arm gauntlet-clause precedent; rbgft classification unit-test residual flagged to the theurge stream. Both recorded in the paddock.

### 2026-07-02 13:20 - Heat - d

paddock curried: escheat residual: dead-schema arm live-proven, orphan arm gauntlet-claused

### 2026-07-02 12:59 - ₢BfAA5 - n

Escheat, contract-first: the manor-hygiene sweep of the terrier. Spec RBSME-manor_escheat.adoc (plan-then-confirm; depot-liveness rule ACTIVE-with-anchor live / DELETE_REQUESTED-403-404 dead / ACTIVE-without-anchor flagged-and-kept; provider-liveness deliberately out of scope; raw-grain data layer beneath the muniment contract) wired into RBS0 as rbtgo_manor_escheat after raze, with the RBSTR hygiene-access NOTE, the RBSPO orphan-removal pointer, and the acronym-registry entry. Code: BUBC_band_escheat=114 (the band's last free code); rbgb_managed_folders_capture (paged managed-folder list); rbgft hygiene pair — escheat_survey (tolerant classifying read: key shape, mantle enum, body JSON, rbgft_ fields, key-body agreement; benign-vanish 404 skip; per-run forensic files) and escheat_expunge_raw (raw delete by listed name, 204/404 clean); rbgp_manor_escheat verb (survey -> folder list -> distinct-depot dedupe -> CRM liveness probe -> plan display -> already-clean exit-0 short-circuit -> bucket-name confirm gate -> sweep objects then purge dead folders -> re-survey verify against held liveness) with three helpers; CLI arm rides the depot_list no-depot-regime posture; zipper enrolls rbw-mE PayorEscheatsTerrier, generated consts + tabtarget context re-derived. Shellcheck 227 clean. Not yet live-exercised — that follows this notch.

### 2026-07-02 13:06 - ₢BfAAK - W

Landed the programmatic RFC 7523 token-acquisition arm on the rba accessor plus its whole read surface, atomically (commit 36e292734), across four surfaces. rba_auth.sh: a mechanism-gated programmatic Leg-1 sibling (zrba_leg1_programmatic_idtoken_capture) mints an id_token via the jwt-bearer grant — BCG custody honored (asserter private key read only by openssl via its regime PATH, client secret delivered to curl only by --data-urlencode name@file, id_token jq-straight-to-stdout, never a shell var), base64url via openssl+parameter-expansion (tr/base64 evicted), a small zrba_b64url_capture helper DRYing the three encodings; rba_avow gates Leg 1 on RBRF_MECHANISM and reuses Leg 2/sitting/don wholesale. rbrf_regime.sh: six programmatic self-supply fields enrolled+validated (grant endpoint https-or-loopback-http escape, two repo-root-relative path guards, three assertion facts). rbxk_keycloak.sh: zrbxk_render_live renders the reachable grant endpoint beside the JWKS from the nameplate port (accessor-facing, never uploaded). Template gained the five committed lines; the caged client-secret file joined fdkyclk/ (no trailing newline, RBSFK two-keys). Verified: shellcheck 227 files clean, reveille 117/0, regime-poison 29/0, entrada validates with the six fields correctly gated, the programmatic enforce-branch validates via a synthetic live regime, and the RS256 mint crypto verifies locally against the asserter public key with the cinched payload (aud=RBRF_IDP_ISSUER). Ceiling honored: reaches the federated token, no spine-local brevet. The end-to-end acquisition over the instated foedus is blocked by the constant-folded RBCC_rbrf_file (deferred selector-derivation) and is homed in the follow-on pace ₢BfAA7 (accessor-foedus-selector-resolve).

### 2026-07-02 13:05 - Heat - S

accessor-foedus-selector-resolve

### 2026-07-02 12:55 - ₢BfAAK - n

Land the programmatic RFC 7523 token-acquisition arm on the rba accessor plus its whole read surface, atomically, consuming the ₢BfAA6 RBSRF re-cut. rba_auth.sh: a new mechanism-gated Leg-1 sibling zrba_leg1_programmatic_idtoken_capture mints an id_token via the RFC 7523 jwt-bearer grant — BCG custody honored (asserter private key read only by openssl via its regime PATH, client secret delivered to curl only by --data-urlencode name@file reference, id_token jq-straight-to-stdout; never a shell var), base64url via openssl enc -base64 + parameter expansion (tr/base64 are evicted), a small zrba_b64url_capture helper DRYs the three encodings; rba_avow now gates Leg 1 on RBRF_MECHANISM (interactive device-flow vs programmatic grant) and reuses Leg 2 (STS)/sitting-cache/don wholesale, the cache key already pool+provider. rbrf_regime.sh: enroll+validate the six programmatic self-supply fields (RBRF_GRANT_ENDPOINT with the https-or-loopback-http escape, RBRF_ASSERTER_KEY_FILE/RBRF_CLIENT_SECRET_FILE repo-root-relative-path guards, RBRF_ASSERTER_KID/ISSUER/SUBJECT present-non-empty). rbxk_keycloak.sh: zrbxk_render_live now renders RBRF_GRANT_ENDPOINT beside the JWKS, composed from the nameplate port (accessor-facing, never uploaded to GCP). Template gains the five committed lines (2 path refs + 3 assertion facts); the caged client-secret file joins fdkyclk/ beside the asserter key (no trailing newline, RBSFK two-keys). Ceiling respected: reaches the federated token, no spine-local payor-direct brevet. Crypto proven locally (RS256 assertion verifies against the asserter public key); shellcheck 227 files clean.

### 2026-07-02 12:34 - ₢BfAA6 - W

Recut RBSRF/RBSFA/RBSFK (+ RBS0 mapping/definitions, acronyms entry) to home the programmatic accessor's RFC 7523 grant inputs: six new RBRF programmatic self-supply fields (grant endpoint with loopback-http escape, asserter-key and client-secret path references, three committed assertion facts; assertion aud cinched to the existing issuer field), sanctioned-deviation NOTE extended to two rendered fields, RBSFA precondition recut to the two-key reality with the self-mint fallback removed as impossible-by-construction, RBSFK correspondence line + client secret into the caged census. A live crucible probe settled the open fork: Keycloak refuses the jwt-authorization-grant on a public client, so the confidential client + secret is load-bearing — recorded as a fourth RBSFK first-contact trip. Field names grep-gated; paddock build-order line updated; accessor build docket verified already aligned. No code moved.

### 2026-07-02 12:33 - Heat - d

paddock curried: build-order accessor line: self-mint fallback removed per the spec recut

### 2026-07-02 12:32 - ₢BfAA6 - n

Recut the federation specs to home the programmatic accessor's RFC 7523 grant inputs in the regime, contract before code. RBSRF programmatic group gains the six self-supply fields: RBRF_GRANT_ENDPOINT (orchestrator-rendered per charge beside the JWKS, https-or-loopback-http validator, deliberately distinct from the interactive token endpoint on commit-vs-render provenance), RBRF_ASSERTER_KEY_FILE + RBRF_CLIENT_SECRET_FILE (repo-root-relative path references — secret values never ride sourced regime vars; ships-committed premise un-amended), and RBRF_ASSERTER_KID/ISSUER/SUBJECT (committed assertion facts, dual-homed with the realm DATA like client-id; assertion aud cinched to RBRF_IDP_ISSUER, no new field). Sanctioned-deviation NOTE extended to the two rendered fields; validate section extended. RBS0 mapping + definition sites for the six quoins; programmatic-group definition widened. RBSFA precondition recut to the two-key reality (caged asserter key + client secret by path reference, nothing marshal-fenced) and the self-mint fallback removed with its impossibility stated (ephemeral realm key held by no caller — self-mint would re-establish the trust). RBSFK: client secret joins the caged-scaffolding census, completion homes the caller-side correspondence with the RBRF self-supply fields, and a fourth first-contact trip records the harness-probed public-client refusal (Keycloak forbids the jwt-authorization-grant on a public client), so the confidential client + load-bearing secret is a surveyed verdict, not a guess. Acronyms RBSFA entry updated to match. Field names grep-gated clean before adoption.

### 2026-07-02 11:48 - Heat - d

batch: 1 reslate, 1 slate

### 2026-07-02 12:04 - ₢BfAA2 - W

Guide-to-tabtarget automation lift, landed and live-proven. The credential-boundary sort found exactly two automatable steps in the payor guide — enable-required-APIs and billing linkage — and billing automatability was verified, not assumed: the payor OAuth grant carries cloud-billing scope and the updateBillingInfo PUT already ships in depot_levy. Contract-first: RBSMS gained the credential-boundary NOTE and two opening steps (8-service payor-project API set with provenance and the serviceusage bootstrap edge; GET-then-PUT billing ensure with drift-reconcile-to-RBRP); RBSPE shrank to the pre-credential genesis with a pointer to the finisher; RBSPI completion repoints install at instaurate then levy. Code: rbgp_manor_instaurate opens with the idempotent rbge_api_enable loop (guide's six APIs plus iam+storage, closing its formerly-silent preconditions) and the billing ensure (empty capture folds to not-linked so a fresh manor links rather than dies); install completion emits instaurate-then-levy tabtargets; the payor onboarding wrapper gained an Instaurate step; the establish guide dropped its two migrated sections (stale payor-project Cloud Build rationale removed, not copied) and renumbered to 8; rbw-mI zipper description recut, context regenerated. BCG pass caught and fixed two violations before commit (loop-body || buc_die, explicit loop-var initialization). Shellcheck 226 clean; reveille 116/0; live rbw-mI run against the standing manor walked all 8 APIs to confirmed-enabled, hit the billing already-linked no-op, and rode the rest idempotently. Billing PUT arm (fresh manor) and the gauntlet clause await the next gauntlet run.

### 2026-07-02 12:00 - ₢BfAA2 - n

Guide-to-tabtarget automation lift: migrate the two automatable manor-setup steps (enable-required-APIs, billing linkage) out of the manual payor guide into the manor-setup finisher rbgp_manor_instaurate, and wire the guide -> install -> instaurate -> levy handoff end-to-end. Contract-first: RBSMS gains the credential-boundary NOTE and two opening steps (payor-project API set with provenance + bootstrap-edge failure mode; GET-then-PUT billing ensure with drift-reconcile-to-RBRP posture); RBSPE shrinks to the pre-credential genesis (link sub-steps and enable-APIs step removed, pointer to the finisher seated); RBSPI completion repoints install at instaurate then levy. Code: finisher opens with the idempotent rbge_api_enable loop (8 services: the guide's six plus iam+storage, closing the finisher's formerly-silent preconditions) and the billing ensure (empty-capture folded to not-linked so a fresh manor links cleanly); install's completion emits instaurate then levy tabtargets; the payor onboarding wrapper gains an Instaurate step between install and depot; the establish guide drops its link-billing and enable-APIs sections (stale payor-project Cloud Build rationale removed with them) and renumbers to 8 steps; rbw-mI zipper description covers the new scope, tabtarget context regenerated. Billing automatability verified, not assumed: payor OAuth carries cloud-billing scope (rbgp_payor.sh OAuth URL) and the updateBillingInfo PUT is already shipped in depot_levy. Build green.

### 2026-07-02 11:35 - Heat - T

payor-install-finish-manor-guidance

### 2026-07-02 10:25 - ₢BfAAL - W

Live-proved the Keycloak federation facility end-to-end on this clone (rbm_gamma: container runtime + payor cred). rbw-qjK stands up charged Keycloak + affianced rbef_keycloak with the JWKS in the git-ignored carrier (tree clean through setup, affiance's own clean-tree gate untouched); a rerun converges via affiance's programmatic re-sync arm (providers.patch of oidc.jwksJson, updateMask); teardown jilts+quenches; setup-after-teardown restores the soft-deleted provider. All 4 impl-confirms observed, code matched (no fix): (1) a jilted provider surfaces on GET as HTTP 200 + state DELETED -> undelete-then-patch (the code's primary assumption; the 404->create branch and its 409-tombstone residual were never exercised); (2) programmatic providers.create succeeded WITH webSsoConfig in the body; (3) ready-poll hit 200 on attempt 1 (25s charge delay, 150s budget unstressed); (4) nested-tabtarget composition (rbxk_cli -> rbw-cC/mA/mJ/cQ as subprocesses) ran clean. Regression: credless build + shellcheck 226 + reveille 116/0/0; picket 154/0/2 (the 2 skips polity-denial/parley were RAPT expiry mid-run, then both PASSED individually with fresh cred + cached sitting); bivouac 215/0/0 (all tadmor/srjcl/pluml crucibles green). All six Done-when met. Incidental commits kept out of this wrap: 2 fdkyclk kludge-hallmark notches + the pluml-rerendered federation SVGs (font-metric float drift, semantically identical).

### 2026-07-02 10:24 - ₢BfAAL - n

Commit the federation SVGs as re-rendered by the bivouac pluml crucible case (rbtdrc_pluml_render_diagrams) on this clone. Diff is sub-pixel textLength/coordinate float drift from font-metric render portability (same PlantUML 1.2026.2 both sides) — semantically identical diagrams, not a content change. Incidental to the ₢BfAAL live-test regression; committed per operator directive to clean the tree before wrap. size_limit raised to 250000 per operator confirmation (six full ~35-44KB SVGs; legitimate content).

### 2026-07-02 07:56 - ₢BfAAL - n

Live-cycle kludge: drive the fdkyclk BOTTLE hallmark (k260702075602-f23e899fa, Keycloak 26.6.3 + baked fdkyclk-realm.json) into rbrn.env from a local kludge on this clone. Second of the two kludge notches; tree is now clean so the Keycloak setup's clean-tree gate (RBCC_creed_clean_affiance) passes. Separate from the pace wrap per the docket.

### 2026-07-02 07:55 - ₢BfAAL - n

Live-cycle kludge: drive the fdkyclk SENTRY hallmark (k260702075534-30e84c65b) into rbrn.env from a local kludge on this clone (rbm_gamma). The prior committed hallmark (k260630, from the retired rbm_beta clone) references an image that exists only on that machine's local runtime, so re-kludging here is required to charge the crucible. Committed separately from the pace deliverable per the docket (keeps the unscoped wrap from sweeping it); the second kludge's clean-tree gate needs this committed first.

### 2026-07-02 04:10 - ₢BfAA4 - W

Carved the terrier noun-internals out of RBS0 into the new normative sheaf RBSTN-terrier_noun.adoc, symmetric with the realm->RBSFK and access->RBSTR carves. RBSTN mirrors the sheaf apparatus (copied //axvd_sheaf banner + scope comment + one == Terrier noun heading) and holds four blocks relocated byte-verbatim in docket order: terrier resource-internals, muniment-internals, the muniment quoin's provider collision-rationale, and the access-pointer sentence (+ list-continuation markers dropped as pure syntax). RBS0's rbtf_terrier quoin now reads as its definition plus two cartouched includes (RBSTN then RBSTR), structurally symmetric with the rbtf_realm quoin; rbtf_muniment keeps only its two definitional sentences. RBSTR got exactly two pointer swaps (header-comment tail + the === Settled heading/body) repointed from 'the RBS0 terrier quoin' to 'the RBSTN Terrier noun sheaf'. RBSTN registered in claude-rbk-acronyms.md between RBSTC and RBSTR. Verified: reconstructed the expected sheaf body from git-HEAD blocks and diffed against RBSTN = byte-identical (order 1,2,3,4); all 8 attribute refs used in RBSTN resolve to :name: definitions in RBS0's mapping section; anchors unique; no orphaned + ; moved prose gone from RBS0. Pure .adoc/.md relocation, no test tier applies.

### 2026-07-02 04:10 - ₢BfAA4 - n

Carve the Terrier noun internals out of the RBS0 terrier quoin into a new standing subdoc RBSTN-terrier_noun.adoc. The quoin now reads as its definition plus two cartouched include::s — the RBSTN noun sheaf (resource contract, muniment JSON shape + rbgft_ sprue, object-name key format, and the load-bearing provider dimension under the one-pool model), then the RBSTR access subdoc. Moved the provider-collision rationale out of the muniment quoin into RBSTN, added the RBSTN acronym mapping, and repointed RBSTR's two "settled in RBS0" references to RBSTN. Sibling of the realm carve (RBSFK) and the access carve (RBSTR); access verbs (engross/expunge/peruse) stay in RBSTR.

### 2026-07-02 04:04 - ₢BfAAL - n

Fix BCG command-discipline violation flagged by reveille's rbtdru_kit_bash fixture: replace the evicted grep -v in zrbxk_render_live's eof-stripping with a pure-bash read loop (a builtin per BCG Prefer Bash Builtins). Shellcheck stays 226 clean.

### 2026-07-02 04:01 - ₢BfAAL - n

Complete the Keycloak orchestrator wiring (post origin/main merge). Reconcile setup's clean-tree gate to reuse RBCC_creed_clean_affiance — origin's BiAAl migration had already added that creed and retired the old bug_require_clean_tree, so the orchestrator gates up-front precisely to honor affiance's own committed-name requirement before charging (ACG: reference the home, no near-duplicate creed). Enroll the rbw-q facility group: rbw-qjK (rbxk_setup) and rbw-qjQ (rbxk_teardown), channel-less, dispatching to rbxk_cli.sh; add the two trampoline tabtargets; regenerate rbtdgc_consts.rs + claude-rbk-tabtarget-context.md from the zipper. RBSRF gains the one sanctioned ships-committed deviation note for rbef_keycloak (committed template + git-ignored live carrier). Build green; shellcheck 226 clean.

### 2026-07-02 03:55 - ₢BfAAL - n

Keycloak orchestrator (interim, before integrating origin/main): RBSMA re-cut adds affiance's programmatic JWKS re-sync arm (200-branch — live provider gets providers.patch of oidc.jwksJson via updateMask; soft-deleted undeletes-then-patches; interactive stays a no-op; drift-reconcile NOTE narrowed to jwksJson-exempt). Arm built in rbgp_manor_affiance. Created rbef_keycloak foedus: committed rbrf.env.template (programmatic core + RBSPB namespace-disjoint attribute mapping) plus a git-ignored live rbrf.env carrier. New rbx-family orchestrator rbxk_keycloak.sh + rbxk_cli.sh (BCG): setup gates a clean tree, charges fdkyclk, polls ready, fetches+strips the JWKS bridge into the ignored live regime, affiances rbef_keycloak; teardown jilts then quenches — composing charge/quench/affiance/jilt through their tabtargets, never reimplementing them. Zipper enrollment, tabtargets, and the RBSRF ships-committed deviation note still pending.

### 2026-07-02 03:21 - Heat - d

batch: 1 reslate

### 2026-07-02 04:05 - ₢BfAAz - W

Retired the interim terrier scaffold now that the manor finisher (rbgp_manor_instaurate, RBSMS) provisions the terrier for real. Removed: rbw-dt/dT tabtargets + zipper enrollments; rbgp_terrier_scaffold/rbgp_terrier_proof verb bodies + their two scaffold-exclusive ZRBGP_INFIX_TERRIER_* consts (shared RBGP_TERRIER_BUCKET + rbgb_managed_folder_* helpers kept — used by the finisher and polity verbs); the terrier-scaffold + terrier-atomicity fixtures across rbtdra_almanac.rs (roster/picket/echelon), rbtdrv_patrol.rs (statics + fn bodies), and rbtdrm_manifest.rs (name-consts + colophon-dependency-map arms). Verified the terrier-band coverage lives in polity-denial's terrier arc (real brevet/unseat/rehearse under the rbuh http-fault seam, exact bands) with the positive round-trip on parley — no duplication; the docket's no-standalone-terrier-poison-fixture trap held. Corrected the RBS0 terrier cross-project line, neutralized two stale scaffold refs in RBSMS, and reframed the RBGFT acronym entry. rbtdgc_consts.rs + claude-rbk-tabtarget-context.md regenerated by the build. Verification: build compiles (const-eval suite guards pass), theurge units 156/0 (incl. model_classifies_every_suite), reveille 116/0/0, shellcheck 224 clean; gauntlet provably unaffected (never contained the terrier fixtures, still compiles). picket/echelon live runs deferred to operator (post main-merge). Honest residual: the old proof's 412/404 idempotency assertion is not reproduced — left as the paddock's parked rbgft-unit-test idea, out of this pace's Done-when. Scope note: removed the rbgp verb bodies + exclusive consts though the docket bullets named only the tabtargets/fixtures/manifest arms — retiring the tabtargets orphans them, and their own comments said 'retired when Bf consolidates the founding-home'.

### 2026-07-02 04:02 - ₢BfAAz - n

Retire the interim terrier scaffold now that the manor finisher (rbgp_manor_instaurate, RBSMS) provisions the terrier for real. Removed the rbw-dt/dT tabtargets + their zipper enrollments + the rbgp_terrier_scaffold/rbgp_terrier_proof verb bodies (and their two scaffold-exclusive ZRBGP_INFIX_TERRIER_* consts; the shared RBGP_TERRIER_BUCKET and rbgb_managed_folder_* helpers stay, used by the finisher and polity verbs). Retired the terrier-scaffold + terrier-atomicity fixtures from the roster/picket/echelon in rbtdra_almanac.rs, their statics + function bodies in rbtdrv_patrol.rs, and their name-consts + colophon-dependency-map arms in rbtdrm_manifest.rs. The terrier-band rejection coverage lives in polity-denial's terrier arc (real brevet/unseat/rehearse under the rbuh http-fault seam); the positive round-trip rides parley — no duplication. Corrected the RBS0 terrier cross-project line (was 'provisioned by an interim scaffold, never folded into a founding ceremony'), neutralized two stale scaffold references in RBSMS, and reframed the RBGFT acronym entry. rbtdgc_consts.rs + claude-rbk-tabtarget-context.md regenerated by the build. Build compiles (const-eval suite guards pass), shellcheck 224 clean.

### 2026-07-02 04:00 - Heat - d

batch: 1 reslate

### 2026-07-02 03:48 - Heat - d

batch: 1 reslate

### 2026-07-02 03:48 - ₢BfAAq - W

Barrier satisfied and verified — no code work of its own. ₢BlAAY (₣Bl's last registry write) is wrapped (W commit c360dcdc7) and confirmed in this clone's HEAD ancestry (merge-base --is-ancestor); its :n: commit 8ef0ecec9 was pushed to origin. The fully-restructured fixture/suite registry is present here in rbtdra_almanac.rs (roster/picket/echelon lines 52/140/191). Nuance recorded: ₢BlAAY did NOT land a standalone 'terrier-poison' fixture as this pace's docket predicted — it folded the terrier-band coverage into the old mantle-denial fixture and renamed it polity-denial (one picket fixture, two arcs). terrier-atomicity still stands (lines 51/139/190), awaiting retirement under the next pace ₢BfAAz — which was reslated in the same session to swap its stale terrier-poison references for polity-denial's terrier arc.

### 2026-07-02 03:12 - Heat - S

terrier-hygiene-sweep

### 2026-07-02 03:05 - ₢BfAA3 - n

Incident report: the ₢BfAA3 gallops coronet-collision (alpha affiance vs beta terrier-noun-sheaf-carve, both allocated the same next-free coronet across divergent clones) and the full convergence — Fable's exotic heat-tombstone transfer (nominate ₣Bp, transfer ₢BfAA3→₢BpAAA to evacuate the coronet, re-coronet terrier to ₢BfAA4, retire ₣Bp), alpha's merge-not-rebase convergence, and the operator-authorized one-off gallops hand-edit. Records root cause (distributed next-free allocation), why rebase re-fights gallops while merge finalizes off MERGE_HEAD, the manual-edit gap (no merge driver; jjx_validate can't parse marker-mangled JSON), and recommended durable JJK fixes.

### 2026-07-02 02:59 - Heat - d

batch: 1 reslate

### 2026-07-02 02:58 - Heat - d

batch: 1 reslate

### 2026-07-02 02:37 - Heat - T

terrier-noun-sheaf-carve

### 2026-07-02 02:36 - Heat - T

terrier-noun-sheaf-carve

### 2026-07-02 02:35 - Heat - S

terrier-noun-sheaf-carve

### 2026-07-02 02:07 - ₢BfAA3 - W

Folio-addressed affiance and jilt: each takes the foedus as a param1 folio (like descry) and acts on the NAMED foedus's rbrf.env, reserving the active-foedus selector (RBRR_ACTIVE_FOEDUS/instate) for the credential accessor. Contract-first RBSMA/RBSMJ re-cut to the folio-addressed model; rbgp_cli furnish folio-resolves the rbrf.env via an idiom-clean in-arm case (matching rbrd_cli — no cross-module dependency, no foreign kindle mid-source, after a first cut that over-coupled and was reverted on operator catch); verb bodies stay foedus-blind (comments re-cut); zipper channel ''->param1 + purpose; acronym entries noted; tabtarget-context.md regenerated. Atomic consumer re-point: the foedus folio threaded through all five affiance/jilt call sites in the two foedus fixtures (rbtdrv_patrol.rs) — reuse leg matches its own descry/instate, lifecycle derives the folio from RBRR_ACTIVE_FOEDUS with poison overriding only the pool; the mechanical folio-pass, distinct from the provider-grain semantic re-cut left to the theurge stream. Verified: theurge build, shellcheck 224, theurge unit 156/0, fast-qualify (incl. generated-file freshness), reveille 116/0, credless folio-guard proofs (no-arg + rbef_bogus reject in the furnish before auth), and live rbw-mA rbef_entrada green — idempotent no-op on the already-seated spike-entra provider under pool spike-office-test (org 247899326218), folio-addressed and not regressed. Residual: the two foedus fixtures' folio-pass is compile/unit-verified but runtime-unrun (reuse needs an interactive avow click; lifecycle is quota-touching) — runtime green lands with the theurge-stream re-cut.

### 2026-07-02 02:02 - ₢BfAA3 - n

Folio-address affiance and jilt: each now takes the foedus as a param1 folio (like descry) and acts on the NAMED foedus's provider, resolving rbef_<folio>/rbrf.env directly; the active-foedus selector (RBRR_ACTIVE_FOEDUS/instate) is reserved for the credential accessor. Contract-first: RBSMA/RBSMJ re-cut to the folio-addressed model (an unresolvable folio is a precondition rejection naming the absent foedus). CLI: rbgp_cli furnish sources the folio-derived rbrf.env for affiance/jilt via an in-arm case (matching the rbrd_cli idiom), active foedus otherwise — no cross-module dependency, no mid-source foreign kindle; verb bodies stay foedus-blind (comments re-cut). Zipper channel ""→param1 + purpose (args: foedus), regenerating claude-rbk-tabtarget-context.md. Atomic consumer re-point of the CLI contract change: threaded the foedus folio through both foedus fixtures' affiance/jilt calls in rbtdrv_patrol.rs (reuse leg matches its own descry/instate; lifecycle derives the folio from RBRR_ACTIVE_FOEDUS, poison overriding only the pool) — the mechanical folio-pass, distinct from the provider-grain semantic re-cut reserved for the theurge stream. Acronym entries note the folio-addressing. Theurge build green.

### 2026-07-02 01:53 - ₢BfAAy - W

Added the provider dimension to the terrier muniment so co-resident foedera under the one pool no longer conflate. Muniment key is now <depot>/<mantle>/<provider>/<subject> and content gains rbgft_provider; the read path emits a 3-column <mantle>\t<provider>\t<subject> line from content so a manor-wide rehearse attributes each holding to its admitting foedus. Threaded through rbgft_ (key composer, engross/expunge signatures + content builder, peruse/peruse_manor emit) and the rbgp_ brevet/unseat cores (RBRF_PROVIDER_ID, enforced by the CLI arm); the interim proof uses a synthetic provider. Specs re-cut contract-first: RBS0 terrier/muniment/rehearse quoins + RBSTR access contract + RBGFT acronym entry. Cantled BfAA3 for the RBS0 terrier-noun sheaf carve-out (cinched cosmetic-only, per operator). Migration was plan-A with no artifact: the interim scaffold's folder purge wiped old-scheme muniments and freehold-establish rebrevetted new-scheme. VALIDATED LIVE: terrier-atomicity (new key create, 412/present + 404/absent idempotency, 3-column peruse) + freehold-establish 6/6 (gird/brevet -> core -> RRBF -> engross, don-after-brevet) + rehearse showing the restored freehold roll with provider 'spike-entra' across governor/director/retriever. Shellcheck 224 clean; notched dbcf52573.

### 2026-07-02 01:45 - ₢BfAAy - n

Add the provider dimension to the terrier muniment so co-resident foedera under the one pool no longer conflate. Muniment key becomes <depot>/<mantle>/<provider>/<subject> and content gains rbgft_provider; the read path emits a 3-column <mantle>\t<provider>\t<subject> line from content so a manor-wide rehearse attributes each holding to its admitting foedus. Threaded through rbgft_ (key composer, engross/expunge signatures + content builder, peruse/peruse_manor emit) and the rbgp_ brevet/unseat cores (pass RBRF_PROVIDER_ID, enforced by the CLI arm); the interim proof uses a synthetic provider + updated 3-column assertion. Specs re-cut contract-first: RBS0 terrier/muniment/rehearse quoins + RBSTR access contract + RBGFT acronym entry. Shellcheck 224 clean. Live-terrier migration is plan-A with no artifact: the scaffold's folder purge wipes old-scheme muniments and freehold-establish rewrites new-scheme at the next picket run.

### 2026-07-02 01:41 - Heat - S

terrier-noun-sheaf-carve

### 2026-07-02 01:35 - Heat - d

batch: 1 reslate

### 2026-07-02 01:34 - Heat - S

affiance-jilt-folio-recut

### 2026-07-01 21:42 - ₢BfAA1 - W

Re-cut the three foedus verbs to the one-pool provider model per the settled RBSMA/RBSMJ/RBSFD. Affiance: dropped the F1 org self-grant and the whole pool-ensure (both the instaurate finisher's now) for the fatal pool-present gate (workforcePools.get, 404/DELETED dies naming the finisher tabtarget via buyy_tt_yawp); the mechanism-conditional provider-ensure is the verb's whole remaining substance; clean-tree gate kept with provider-grain rationale. Jilt: provider-grain inverse — typed confirmation on RBRF_PROVIDER_ID, providers.get probe with idempotent 404/soft-deleted short-circuits, providers.delete, DELETED-or-404 verify poll on the provider; all pool graveyard handling gone (raze's). Descry: verdict vocabulary aligned to RBSFD — healthy stable, provider-absent, and the new coordinate-drift deficit (pool absent/soft-deleted collapse into it as descry's half of the RBRW sync guard, a reported verdict not an error). Rode along: zipper purposes rbw-mA/mJ/jd re-cut provider-grain (context regenerated), RBCC token comment, two stale pool-grain prose lines in the reuse fixture, BCG Exception-2 hoist of jilt's verify-loop locals. PROVEN: picket green post-clone-sync at 23 fixtures / 157 passed / 0 failed (suite doubled mid-session via the sync), and descry proven LIVE against the real manor — rbef_entrada HEALTHY, provider spike-entra present under pool spike-office-test, the exact re-cut coordinate-confirm→provider-presence shape. Not live-run by choice: affiance idempotent path (operator declined the optional one-shot), jilt (would delete the real standing provider), and the foedus-reuse fixture full run (needs human avow; its branch token healthy is live-proven). Known-broken residue as designed: foedus-lifecycle fixture stays pool-grain for ₣Bl, failing loud at the new pool-present gate. Raze still carries in-loop verify locals (pre-existing, flagged not touched).

### 2026-07-01 20:25 - ₢BfAA1 - n

Re-cut the three foedus verbs to the one-pool provider model per the re-cut RBSMA/RBSMJ/RBSFD. AFFIANCE (RBSMA): dropped the org-level workforcePoolAdmin self-grant (spike F1) and the whole pool-ensure block (soft-delete refuse-and-rotate included) — both now the instaurate finisher's — and replaced them with the pool-present gate: workforcePools.get on RBRW_WORKFORCE_POOL_ID, fatal on 404 or state DELETED with the message directing the operator to the finisher tabtarget (resolved via buyy_tt_yawp RBZ_INSTAURATE_MANOR); the provider ensure-exists block (mechanism-conditional oidc body) stands unchanged as the verb's whole remaining substance. Clean-tree gate retained, rationale re-worded provider-grain (provider id is the committed RBRF value). JILT (RBSMJ): re-cut from pool delete to provider delete — probes workforcePools.providers.get (404 idempotent no-op short-circuit; 200+DELETED already-dissolved, consistent with the verify terminal), provider-scoped typed confirmation (operator types RBRF_PROVIDER_ID, message states the pool stands), providers.delete, then the DELETED-or-404 poll on the provider resource; all pool soft-delete/undelete/graveyard prose gone (that is raze's, ₢BfAAx). DESCRY (RBSFD): verdict vocabulary re-cut to provider-grain — the pool get is now the folded coordinate-confirmation (descry's half of the RBRW sync guard): pool absent/soft-deleted collapse into the single coordinate-drift deficit verdict (was pool-absent/pool-deleted), provider read yields healthy/provider-absent; 'healthy' stable for the foedus-reuse fixture branch; broken reads still reject in BUBC_band_descry. All three read the pool from RBRW (semantics now match ₢BfAA0's mechanical re-point). Rode along: zipper purpose strings for rbw-mA/rbw-mJ/rbw-jd re-cut provider-grain (tabtarget-context.md regenerated by the theurge build), RBCC foedus-health token comment updated to healthy/provider-absent/coordinate-drift, and two stale pool-grain prose lines in the foedus-reuse fixture (rbtdrv_patrol.rs comments + one Fail message string). BCG pass at operator prompt: hoisted jilt's verify-loop per-iteration locals (z_verify_infix/code/state) to the pre-loop declaration block per Exception 2 (the old pool-grain body declared them in-loop; raze still carries that shape — flagged, not touched). Green so far: shellcheck 224 clean, fast qualify pass. Theurge rebuild after the patrol comment edit pending — the launcher hit a stale burx.env in the shared output dir left by a parallel-clone run (rbm_alpha rbw-fhv), a cross-officium artifact, not cleaned; rebuild + picket tier still owed after this notch.

### 2026-07-01 20:24 - ₢BfAAt - W

Terrier band discrimination: minted the three terrier gate codes in the bubc precision band (BUBC_band_engross=111 / BUBC_band_expunge=112 / BUBC_band_peruse=113 — distinct per the allocation rule since the sub-ops chain along the terrier-proof spawn path; the read's list/fetch/body-parse deficits are rules of the one peruse gate per the descry precedent), converted rbgft's error-path buc_die to buc_reject of the matching band (engross/expunge unexpected-code arms + the read core's list, fetch, and missing-fields paths; harness-breakage checks and arg preconditions stay imprecise buc_die per the BCG carve-out; the 412/present and 404/absent idempotent successes untouched), and added the HTTP fault-injection seam rbuh lacked: zrbuh_fault_apply, one membrane in rbuh_json under buorb_http_fault (RBCC_tweak_http_fault tinder) with BURE_TWEAK_VALUE 'INFIX=CODE' forcing the captured code for one named request infix — the regime-poison analogue for HTTP. Band codes + tweak name projected to Rust consts (RBTDGC_BAND_ENGROSS/EXPUNGE/PERUSE, RBTDGC_TWEAK_HTTP_FAULT) for the sibling ₣Bl poison fixture; both live tweak censuses (BUS0, CLAUDE.md) extended. Green: shellcheck 224, reveille 116/0, BUK self-test 49/0, fast qualify.

### 2026-07-01 20:22 - ₢BfAAt - n

Gave the terrier data layer (rbgft) precise, testable failure discrimination, closing the wrong-reason hole on its error paths. Minted three terrier gate codes in the bubc precision-exit-code band — BUBC_band_engross=111 (unexpected HTTP on the conditioned create), BUBC_band_expunge=112 (unexpected HTTP on the conditioned delete), BUBC_band_peruse=113 (list/fetch deficit or malformed muniment body, shared by peruse and peruse_manor) — distinct codes per the allocation rule because the three sub-operations chain along the terrier-proof spawn path; within the read gate, list/fetch/body-parse deficits are rules of one gate (the descry precedent). Converted rbgft's error-path buc_die to buc_reject of the matching band: the engross and expunge unexpected-code case arms, and the read core's list non-OK (formerly rbuh_require_ok), fetch non-OK (formerly rbuh_require_ok), and missing-rbgft_-fields paths; harness-breakage checks (bad code file, jq compose, urlencode) and arg-validation preconditions stay imprecise buc_die per the BCG carve-out, and the idempotent SUCCESS dispositions (engross 412 present, expunge 404 absent) stay exit-0 stdout outcomes, untouched. Added the buo-sprued HTTP fault-injection seam rbuh lacked: zrbuh_fault_apply, one membrane in rbuh_json applied after transport success and before code-file registration, under tweak name buorb_http_fault (RBCC_tweak_http_fault tinder) with BURE_TWEAK_VALUE 'INFIX=CODE' — when the request's infix matches, the captured HTTP code file is overwritten so every downstream reader (rbuh_code_capture, rbuh_require_ok, rbuh_code_ok_predicate) sees the forced code and the caller's error path runs for real; the spec validates before the infix match so a malformed value dies loud even when aimed elsewhere, and any other infix or tweak rides inert (the regime-poison analogue for HTTP). Projected the three band codes and the tweak name through rbcc_emit_consts (RBTDGC_BAND_ENGROSS/EXPUNGE/PERUSE, RBTDGC_TWEAK_HTTP_FAULT) and regenerated rbtdgc_consts.rs via the theurge build for the sibling poison fixture on the theurge stream. Extended the two live tweak censuses (BUS0 Tweak Mechanism examples-and-census line, CLAUDE.md BURE tweak-name detail) so the grep-buo census claims stay true. Shellcheck 224 clean.

### 2026-07-01 20:03 - ₢BfAAF - W

Built the manor-setup finisher rbgp_manor_instaurate (RBSMS, colophon rbw-mI) — the idempotent, payor-credentialed ensure-exists founding of the manor's scriptable substrate, the inverse of raze: (1) org-level workforcePoolAdmin self-grant (spike F1); (2) the one workforce pool via a list-and-match drift guard — id-match for the expected coordinate, RB description-marker for the different-id drift check, PATCH-reconcile session-duration + marker backfill masking only drifted fields, never a bare get-by-id, never a delete; (3) the terrier bucket (payor-project-grain); (4) the per-polity managed folder + grain IAM (ensure-only, governor-mantle objectAdmin own-folder / objectViewer manor-wide, getIamPolicy read-back). Contract-first: authored two sibling operation subdocs — RBSMS (finisher) and RBSMR (raze's owed retroactive spec, closing the debt BfAAx left) — wired into RBS0 (mapping + operation blocks), acronyms registered; no new rbtf_ civic-act quoins per the descry/instate deferred-verb precedent. Homed the shared RBGC_WORKFORCE_POOL_MARKER constant so affiance's marker and the finisher's filter cannot drift. Verb 'instaurate' operator-chosen over the provisional 'found' and renamed across every namespace. PROVEN LIVE end-to-end against the real standing manor, twice (idempotent-convergent): the live run caught a real bug (the id-match was wrapped inside the marker guard, so the pre-marker spike pool 409'd on create) which was fixed to coordinate-match + marker-backfill. Green: shellcheck 224, fast qualify, reveille 120/0. Also slated the sibling guide-to-tabtarget automation-lift pace (BfAA2, Fable-suited). Gauntlet ladder deferred (heavy fresh-project levy; the finisher's own live proof exceeds it, and finisher-in-ladder wiring is downstream).

### 2026-07-01 20:00 - ₢BfAAF - n

Fix a real list-and-match bug the live run exposed, and reconcile the spec to the proven behavior. BUG: the loop wrapped the id-match inside the description==marker guard, so the standing spike-office-test pool — which predates the marker convention and carries no marker — was skipped entirely, fell through to the create branch, and 409'd ('pool already exists'). FIX: match the pool at the expected id by COORDINATE (id alone, regardless of description); reserve the marker solely for recognizing OUR pool among unrelated org pools in the DIFFERENT-id drift check. On reconcile, build the updateMask from only the drifted fields — sessionDuration when it differs AND description when the marker is absent (backfilling it so a pre-convention pool becomes conformant); a fully-conformant pool is left untouched (empty mask, no PATCH). BCG: hoisted the mutable z_mask accumulator to the function-scope declaration block (Exception 4, not an in-branch local) and expanded the dense one-line nested if to multi-line per the operator's BCG reminder. Reconciled RBSMS-manor_instaurate.adoc (the list-and-match NOTE + the ensure-pool step) to describe id-match-for-expected / marker-for-drift / mask-only-drifted-fields, contract-first. VERIFIED LIVE end-to-end against the real standing manor (operator-permitted, post clone-resync): run 1 matched spike-office-test by id and reconciled updateMask=description (backfilled the missing marker; session already matched) — green; run 2 hit the fully-conformant no-op path ('already conformant, leaving in place') — green, proving idempotent convergence and that the backfill persisted. Bucket/folder/IAM all idempotent with getIamPolicy read-back. Shellcheck 224 clean.

### 2026-07-01 19:51 - ₢BfAAF - n

Renamed the finisher verb from the provisional 'found' to the operator-chosen civic-asterism 'instaurate' across every namespace: rbgp_manor_instaurate (verb), rbw-mI / PayorInstauratesManor (colophon + tabtarget, tt/rbw-mF renamed to tt/rbw-mI, 755 preserved), rbtgo_manor_instaurate (operation quoin — RBS0 mapping + operation block + the RBSMR raze cross-refs), RBSMS-manor_instaurate.adoc (subdoc renamed from RBSMS-manor_found.adoc), RBZ_INSTAURATE_MANOR (zipper const), and the doc-facing act labels (doc_brief, 'Confirm Instaurate' typed output, 'Manor instaurated' banner, 'Instaurate operation' acronym entry, and the internal instaurate_* rbuh infixes + temp-file names). Deliberately KEPT descriptive 'founds/founding' prose as the plain-English gloss of instaurate (matching how RBSMA affiance mixes the minted verb with plain description); a blanket sweep would have clobbered Foundry / 'not found' / 'founding' throughout, so only precise identifier tokens + explicit act-labels were swept, then a repo-wide grep confirmed zero stragglers (caught one in the rbgc_constants.sh marker comment). Re-aligned the rbgp_cli case arm to the shared column. Theurge build regenerated rbtdgc_consts.rs (RBTDGC_INSTAURATE_MANOR='rbw-mI') + tabtarget-context.md. Green: shellcheck 224 clean, fast qualify pass, reveille unaffected (name-only change). The live end-to-end run is deliberately HELD at operator request pending a resync of the parallel clones.

### 2026-07-01 19:44 - Heat - S

guide-to-tabtarget-automation-lift

### 2026-07-01 19:26 - ₢BfAAF - n

Built the manor-setup finisher rbgp_manor_found (RBSMS) — the idempotent, payor-credentialed ensure-exists founding of the manor's scriptable substrate, the inverse of rbgp_manor_raze. Founds four things: (1) the org-level roles/iam.workforcePoolAdmin grant on the payor (spike F1, idempotent etag) that affiance assumes present; (2) the ONE workforce pool via a LIST-AND-MATCH drift guard adapted from rbof_canvass — pages workforcePools.list?parent=org&showDeleted=true, keeps RB-marked pools (description == the new shared RBGC_WORKFORCE_POOL_MARKER constant affiance also writes), then classifies: expected-id+live -> PATCH-reconcile sessionDuration (updateMask, non-destructive) or leave if matching; expected-id+DELETED -> refuse (squatting, coordinate drift); a live RB pool at a different id -> refuse (coordinate drift); none -> create via rbge_lro_ok. Never a bare get-by-id, never a delete. (3) the terrier bucket (rbgb_bucket_ensure, payor-project-grain); (4) the per-polity managed folder + grain IAM (rbgb_managed_folder_ensure ENSURE-ONLY dropping the scaffold's destroy-then-create purge, + governor-mantle objectAdmin own-folder / objectViewer manor-wide, with getIamPolicy read-back verify). Homed RBGC_WORKFORCE_POOL_MARKER in rbgc_constants.sh and re-pointed affiance's inline literal at it (non-behavioral, same string) so the finisher's list filter and affiance's marker cannot silently diverge. Enforce set zrbrw+zrbrd (pool + depot coords the folder is named by; no provider, no repo) via a new rbgp_cli case-arm. Enrolled colophon rbw-mF (RBZ_FOUND_MANOR, ships to consumers, not withheld) + tt/rbw-mF.PayorFoundsManor.sh (755); theurge build regenerated rbtdgc_consts.rs (RBTDGC_FOUND_MANOR) + tabtarget-context.md. Shellcheck 224 clean. No clean-tree gate (writes no local config, matching raze/jilt). Verb 'found' still provisional pending operator confirm.

### 2026-07-01 19:17 - ₢BfAAF - n

Contract-first: authored the manor found/teardown operation subdoc pair, mirroring the affiance/jilt (RBSMA/RBSMJ) precedent of one operation quoin per subdoc. RBSMS-manor_found.adoc specs the manor-setup finisher (rbtgo_manor_found): payor-credentialed, ensure-exists, founding the org-level workforcePoolAdmin grant (spike F1), the ONE workforce pool via the list-and-match drift guard (list RB-marked pools under RBRW_ORG_ID by the description marker affiance writes, refuse on id-mismatch as coordinate drift, create-if-absent, PATCH-reconcile session-duration, never a bare get-by-id, never a delete), the terrier bucket (payor-project-grain), and the per-polity managed folder + grain IAM (depot-grain but payor-provisioned since the folder lives in the payor-project bucket — the folder-step credential boundary resolved to the payor, ensure-only, no destroy-then-create). RBSMR-manor_raze.adoc gives the already-built rbgp_manor_raze (BfAAx) its owed spec home (rbtgo_manor_raze): the dangerous force-delete pool teardown, the destructive inverse of found, distinct from the everyday provider-delete jilt, idempotent no-op on 404/DELETED, safety-gated, reads RBRW alone. Wired both into RBS0 (two rbtgo_ mapping entries + two operation blocks after the jilt include); no new rbtf_ civic-act quoins per the descry/instate deferred-verb precedent. Registered RBSMS/RBSMR in claude-rbk-acronyms.md. Verb 'found' (rbw-mF) is provisional pending operator confirm vs constitute/instaurate.

### 2026-07-01 18:57 - ₢BfAAw - W

Verified the membership-principal composer's one-pool re-cut is already fully landed — no code change needed. zrbgp_principal_member_capture (rbgp_payor.sh) emits principal://…/workforcePools/${RBRW_WORKFORCE_POOL_ID}/subject/${subject}, byte-for-byte the RBSPB:77 re-cut form (one manor pool, RBRW-sourced, subject verbatim, pool-named-not-provider). Done-when confirmed clause-by-clause: (1) all four polity verbs route through the single composer — brevet/gird via zrbgp_brevet_core, unseat via zrbgp_unseat_core, attaint via zrbgp_attaint_core — and ₢BfAA0 (rbrw-regime-buildout) already did the RBRW pool re-point in the same landing; (2) subject normalization/namespacing lives provider-side in the RBRF_ATTRIBUTE_MAPPING affiance sets (RBSPB:79 'the member names the pool, never the provider'), which the docket itself scopes to affiance/RBSMA, not this composer — so the composer correctly takes the subject verbatim; (3) exact-string revoke matches and NO binding is orphaned — the git trace shows the pool-id VALUE never changed (RBRF_WORKFORCE_POOL_ID=spike-office-test moved verbatim to RBRW_WORKFORCE_POOL_ID=spike-office-test), so the flagged 'sweep existing per-foedus-pool bindings' judgment call is moot: there was only ever the one pool value, now correctly homed manor-level. The pace's composer deliverable was subsumed by the combination of ₢BfAA0 (pool re-point) + the pre-existing single-home composer + ₢BfAAv (the spec re-cut it built against). Wrapped as verified-already-satisfied; the verification is the deliverable.

### 2026-07-01 18:49 - ₢BfAAx - W

Built rbgp_manor_raze — a dangerous, payor-credentialed force-delete of the manor's one workforce pool, the deliberate inverse of the ensure-exists finisher so a release ladder starts from a clean manor. Distinct from foedus-jilt (the everyday provider-delete): the danger lives in this separate, scary-gated verb so everyday jilt can never nuke the pool. Modeled line-faithfully on rbgp_manor_jilt — buc_require gate (types the pool id; message warns it re-enrolls EVERY citizen; honors BURE_CONFIRM=skip), payor OAuth, idempotent no-op on absent (404) / already soft-deleted pool, LRO delete with provider cascade, poll to DELETED/404 terminal. rbgp_cli enforces RBRW alone (no provider field read). Enrolled colophon rbw-mR in the Manor group + tabtarget rbw-mR.PayorRazesManor.sh (755). WITHHELD from delivery: prep-release Step 9c strip line + survive-list exception, mirroring the rbw-MZ/MP marshal precedent — the verb ships in the surviving rbgp_payor.sh, only the one-keystroke accelerator is withheld. Regenerated rbtdgc_consts.rs (RBTDGC_RAZE_MANOR) + tabtarget-context.md. Green: shellcheck 224 clean, fast qualify, reveille 120/0. Two caveats reported: the delete/poll path is verified-by-construction (a faithful clone of live-proven jilt) not end-to-end, since driving rbw-mR live would destroy the real manor pool; and Done-when clause 2 (release ladder runs raze before the finisher) is deferred to the finisher pace (raze is #1, finisher #3), clauses 1 and 3 met.

### 2026-07-01 18:45 - ₢BfAAx - n

Build the manor-raze verb (rbgp_manor_raze) — a dangerous, payor-credentialed force-delete of the manor's one workforce pool, the deliberate inverse of the ensure-exists finisher so a release ladder can start from a clean manor. Distinct from foedus-jilt (the everyday provider-delete): the danger lives in this separate, scary-gated verb so everyday jilt can never nuke the pool. Modeled on rbgp_manor_jilt — buc_require gate (types the pool id; message warns it re-enrolls EVERY citizen; honors BURE_CONFIRM=skip), payor OAuth, idempotent no-op on an absent (404) or already soft-deleted (state DELETED) pool, LRO delete with provider cascade, poll to DELETED/404 terminal. rbgp_cli enforces RBRW alone (raze reads the pool org/id, no provider field). Enrolled colophon rbw-mR in the Manor group beside affiance/jilt/gird, with tabtarget rbw-mR.PayorRazesManor.sh. WITHHELD from delivery: prep-release Step 9c strips the tabtarget + a survive-list exception (mirroring the rbw-MZ/MP marshal precedent) — the verb ships in the surviving rbgp_payor.sh, only the one-keystroke accelerator is withheld. Regenerated rbtdgc_consts.rs (RBTDGC_RAZE_MANOR) + claude-rbk-tabtarget-context.md via the theurge build.

### 2026-07-01 18:32 - ₢BfAAA - W

Settled the TTY-gating question by retiring the gate: human presence is enforced by the IdP sign-in itself, never terminal possession — the headless fail-fast membrane defended a mechanism needing no defense at production strength (operator ruling 260701, grounded in RFC 8628 research and code survey across a Fable/Opus advisory cycle). Spec-first: RBS0 rbtf_avow definition recut to carry the retirement rationale, three rbtoe_*_authenticate parentheticals recut to match. Code: zrba_tty_present_predicate and probe file deleted whole; the device-flow prompt (verification URL + user code) now rides the shared progress stream as a yawp (buyy_href_yawp/buyy_ui_yawp through buc_step) reaching console, log, and any watching relay — no opt-in, no new config, no prompt file; the user code rides deliberately (RFC 8628 open-display design; substituted sign-in cannot pass admission). Accepted cost recorded: truly unattended cache-miss polls to bounded device-code expiry (~15 min) instead of instant fail — unattended operation belongs to the programmatic mechanism. Paddock idea re-cut to Settled; membrane language retired, human-present premise untouched. Verified: shellcheck 222 clean, reveille 120/0, and a LIVE relay proof — this TTY-less agent session quashed the sitting, ran rbw-aa in background, lifted the prompt off the stream, the operator signed in on their own device, sitting opened (Legs 1+2, 12h) exit 0. The run also live-validated the concurrent RBRW pool-id re-point at Leg 2. The mechanism-design deliberation (opt-in env var, prompt-to-file, BURE naming precedent hunt) dissolved when the operator spotted the gate defended nothing — recorded here as provenance. BUK housekeeping itches surfaced en route, not folded: BURE_CONFIRM is read/validated in buc_require but unenrolled in bure_regime.sh (scope sentinel would reject it in bure-kindling paths), and BUS0's BURE section says kindled-at-buc-source which no longer matches practice (kindle runs only in bure_cli + self-test). Pre-existing cosmetic: rbw-aa preamble emits WARNING: BUZ_FOLIO is not set.

### 2026-07-01 18:27 - ₢BfAA0 - W

Built the RBRW workforce regime (rbrw_regime.sh + rbrw_cli.sh, tabtargets rbw-rwr/rbw-rwv) homing the manor's one workforce pool identity — org/pool/session — per RBSRW, in the flat manor-level rbmm_moorings/rbrw.env (RBCC_rbrw_file), and relocated those three fields out of RBRF and rbef_entrada/rbrf.env, conforming rbrf_regime.sh to the re-cut RBSRF five-section shape. Re-pointed every stripped-field consumer in one atomic landing: rbof descry/canvass file-parse org/pool from the manor RBRW file (provider stays per-foedus); rba_auth sitting-path + STS audience read RBRW_WORKFORCE_POOL_ID behind a new zrbrw_sentinel; rbgp affiance/jilt composers + principal:// read RBRW_*; the theurge patrol poison const became RBRW_WORKFORCE_POOL_ID. Furnish-what-you-load forced parallel RBRW source/kindle/enforce into the 9 other don-path CLIs plus rbgp_cli. All Done-when verified: shellcheck 224 clean, fast qualify green, reveille 120/0, rbw-rwv/rbw-rwr green, and live rbw-jd (HEALTHY) + rbw-aa (live token). Caveats reported: rbw-aa reused a cached sitting so the STS-audience leg is verified-by-construction not end-to-end, and no RBRW-specific reveille fixture yet exists.

### 2026-07-01 18:17 - ₢BfAA0 - n

Set the 755 executable bit on the two new RBRW bash files to match their rbrf siblings — the Write tool created them 644, and the zipper dispatch executes rbrw_cli.sh directly (Permission denied on rbw-rwv/rbw-rwr).

### 2026-07-01 18:15 - ₢BfAA0 - n

Restore the 755 executable bit on the 9 don-path CLIs that carried the RBRW furnish re-point — the awk rewrite-and-mv had reset them to 644, and the credless-guard fixture executes rbfl0_cli.sh directly (Permission denied).

### 2026-07-01 18:13 - ₢BfAA0 - n

Build the RBRW workforce regime (rbrw_regime.sh + rbrw_cli.sh) homing the manor's one workforce pool identity — RBRW_ORG_ID/RBRW_WORKFORCE_POOL_ID/RBRW_SESSION_DURATION — per RBSRW, with the manor-level moorings file rbmm_moorings/rbrw.env (flat sibling of the rbmf_foedera library) and RBCC_rbrw_file. Strip org/pool/session out of rbrf_regime.sh and rbef_entrada/rbrf.env, conforming the RBRF module to the re-cut RBSRF's five sections (Provider Identity / Acquisition Mechanism / IdP Trust (core) / Interactive / Programmatic). Re-point every stripped-field consumer in the same landing: rbof descry/canvass now file-parse org/pool from the manor RBRW file (provider stays per-foedus); rba_auth sitting-file name + STS audience read RBRW_WORKFORCE_POOL_ID with a zrbrw_sentinel guard; rbgp affiance/jilt composers + principal:// read RBRW_*; the theurge patrol poison const becomes RBRW_WORKFORCE_POOL_ID (RBTDRV_RBRW_POOL_VAR). Furnish-what-you-load: add parallel RBRW source/kindle/enforce to the 9 other don-path CLIs (rbgv/rbfr/rbfd/rbgg/rbob/rbld0/rbfl0/rbfv/rbfc0) plus rbgp_cli. Enroll rbw-rwr/rbw-rwv in the zipper; generated rbtdgc_consts.rs + tabtarget-context.md re-derived by the theurge build.

### 2026-07-01 18:21 - ₢BfAAA - n

Retire avow's tty gate: the human-present premise is enforced by the IdP sign-in itself, never terminal possession, so the headless fail-fast membrane defended a mechanism needing no defense at production strength (operator ruling 260701). The device-flow prompt (verification URL + user code) now rides the shared progress stream as a yawp — buyy_href_yawp/buyy_ui_yawp through buc_step — reaching console, log, and any watching relay alike, so a terminal operator and a headless-but-human-reachable agent complete the same sign-in; no opt-in, no new config, no prompt file. The user code rides the stream deliberately: RFC 8628 designs it for open display (possession grants nothing without the human's own IdP sign-in; a substituted sign-in cannot pass admission). Accepted cost recorded: a truly unattended cache-miss polls to the bounded device-code expiry and dies loud rather than failing instantly — unattended operation belongs to the programmatic mechanism. Deleted zrba_tty_present_predicate and its probe stderr file whole (single-site). Spec-first honored: RBS0 rbtf_avow definition site recut to carry the retirement rationale; the three rbtoe_*_authenticate parentheticals recut to match. Reveille-tier credless guard untouched (precedes the retired gate). Shellcheck: 222 files clean.

### 2026-07-01 18:21 - Heat - d

paddock curried: headless-avowal idea settled — tty gate retired, prompt rides the progress stream

### 2026-07-01 17:50 - ₢BfAAp - W

Barrier cleared — the zipper baton ₣Bl→₣Bf handed off cleanly. Verified all three Done-when conditions in this clone: ₢BlAAb (colophon-completeness relocation onto rbw-MZ) wrapped at b8bad3cbf and present as an ancestor of origin/main (pushed), main is behind origin/main by 0 (full theurge zipper state pulled), and the build-regenerated claude-rbk-tabtarget-context.md already lists rbw-jc FoedusCanvass. The following ₣Bf zipper paces (force-delete, finisher, Keycloak-facility) may now edit the theurge-owned zipper trio.

### 2026-07-01 17:30 - ₢BfAAE - W

Scrubbed canonical RBS0 of the overturned pure-org 'manor' sense, layered on the landed ₢BfAAv one-pool re-cut. RBS0: rewrote the Manor quoin to the ratified composite framing (has-a payor project hosting OAuth/billing/terrier bucket + commands-a GCP organization hosting the one workforce pool and org-level trust authority; Manor != its project) and dropped its now-false axig_project voicing; reconciled the old RBS0:1774 payor contradiction per the commands-org asymmetry (everyday depot work needs no org perms; the payor's sole org-scoped authority is the Manor's command of its org — pool founding + affiance/jilt — which keeps depots org-blind); stated Depot<->Manor cardinality on both quoins (each Depot in exactly one Manor, org-blind; a Manor owns zero-or-more depots); re-aimed the terrier 'Manor is the Payor Project' equation to 'lives in the Manor's own payor project'. acronyms.md: rewrote RBSMA/RBSMJ/RBSRF entries carrying both the org-level leak AND pre-one-pool staleness (affiance-seats-pool, jilt-dissolves-pool, refuse-and-rotate) to the landed specs, and registered the missing RBSRW workforce-regime acronym that ₢BfAAv created a spec for but never enrolled. rbgp_payor.sh: re-aimed a dangling terrier-bucket comment citation (comment-only, no behavior change). Deliberately left the affiance/jilt one-line zipper/help briefs saying 'org-level workforce pool' since that is what the un-re-cut code does today — the verb re-cut pace owns changing behavior+description together. Verified: leak-phrase sweep clean except the deliberate code leave-alones; RBS0 'deed' hits are the muniment etymology (correct sense); frozen BZ paddock already carries its composite correction.

### 2026-07-01 17:30 - ₢BfAAE - n

Recast the Manor as a composite holding (has-a payor project, commands-an organization) and split its federation identity along the pool/provider seam: the workforce pool is manor-level and org-hosted (new RBSRW Regime Workforce, founded once at manor setup), while each foedus is a provider hanging beneath it (RBSRF now per-foedus, provider-side). Affiance creates/dissolves a single foedus provider under the standing pool rather than the pool itself; jilt deletes one provider (pool teardown is the manor-teardown act, not jilt). Depots are org-blind — organization-level authority is the Manor's alone, exercised only via affiance/jilt. RBS0 Manor and depot quoins reworked to match; terrier bucket comment reworded to "homed in the Manor's payor project." Acronyms updated for RBSMA/RBSMJ/RBSRF and the new RBSRW.

### 2026-07-01 17:19 - ₢BfAAJ - W

Authored the Entra federation setup guide (rbw-gPF, rbhpf_entra.sh) under the payor-guide family: free-tenant acquisition, app registration, device-flow enablement, and the eight vendor-agnostic core values ending at RBRF regime fields. Sourced from the ₣BZ spike-findings memo and verified against current Microsoft/Google docs (caught two 2026 doc drifts: account-types is now a drop-down, assertion.oid is now Google's recommended Entra mapping). Wired via rbhp0_cli/zipper/tabtarget, shellcheck clean, fast-qualify green, notched at 7678218f2.

### 2026-07-01 17:14 - ₢BfAAJ - n

Author the Entra federation setup guide (rbw-gPF): new rbhpf_entra.sh handbook body under the payor-guide family walking the Entra console work — free-tenant acquisition, single-tenant app registration, public-client device-flow enablement — and ending at the vendor-agnostic core values the foedus regime reads (issuer from the tenant metadata document, client-id, oid subject mapping, device/token endpoints, scope). Console steps verified against current Microsoft and Google docs (2026-06 refresh: account-types drop-down, assertion.oid now Google-recommended). Wired via rbhp0_cli furnish, zipper enrollment, and tabtarget; generated consts and tabtarget context re-derived.

### 2026-07-01 16:54 - Heat - d

batch: paddock, 2 reslate

### 2026-06-30 21:27 - Heat - r

moved ₢BfAA1 after ₢BfAAF

### 2026-06-30 21:27 - ₢BfAAv - W

Re-cut the federation specs to the 260630 one-pool identity substrate (a foedus is now a PROVIDER under one manor-lifetime workforce pool). Mid-pace, minted the RBRW Workforce regime (operator-approved) as the manor-pool-identity home — reifying GCP's Workforce(pool)/Federation(providers) split as two regimes — carrying a committed-record-vs-live-pool sync as a mutability split (org+pool-id pool-time-immutable, session reconcilable) with a strong drift guard folded into the finisher (list-and-match, refuse on id-mismatch) and descry, no separate tripwire. Authored RBSRW; re-cut RBSRF (provider-side per-foedus config), RBSMA (affiance creates a provider under the standing pool, requires it present, sets subject-namespacing), RBSMJ (jilt deletes the provider, pool stands), RBSFD (provider presence + coordinate guard), RBSPB (membership rule homed at rbtf_citizen: IdP-stable canonical subject, test subjects namespace-disjoint), the RBS0 federation civics, and canvass (providers.list). Contract-first, no code. In the same session, sharpened the code follow-on: reslated AAF (drift guard), AAw (RBRW source), AAL (subject-namespacing); slated AA0 (RBRW regime module) + AA1 (affiance/jilt verb re-cut).

### 2026-06-30 21:24 - Heat - d

batch: 1 reslate

### 2026-06-30 20:43 - Heat - d

batch: 2 reslate, 2 slate

### 2026-06-30 20:34 - ₢BfAAv - n

Re-cut the federation specs to the 260630 one-pool identity substrate: a foedus is now a PROVIDER under one manor-lifetime workforce pool. Seated a new RBRW Workforce regime (RBSRW + RBS0 regime/mapping/civics) homing the manor pool identity (org/pool/session) that left the per-foedus RBRF; RBRW carries the committed-record-vs-live-pool sync as a mutability split (org+pool-id pool-time-immutable, session reconcilable) with a strong drift guard folded into the manor-setup finisher (list-and-match, refuse on id-mismatch) and descry — no separate tripwire. RBSRF recut to provider-side per-foedus config. Affiance (RBSMA) now creates a provider under the standing pool, requires the pool present (finisher founds it), drops the pool-ensure/org-admin-self-grant/refuse-and-rotate steps, and sets subject-namespacing provider-side. Jilt (RBSMJ) inverted to delete the provider while the pool stands, pool teardown relocated to the manor-teardown act. Descry (RBSFD) reads provider presence plus its half of the coordinate guard. Canvass recut workforcePools.list -> providers.list. Membership rule (IdP-stable canonical subject, test-provider subjects namespace-disjoint from production) homed at rbtf_citizen and cited from RBSPB's principal:// construction. Contract-first, no code re-cut. RBS0 Manor-vs-org disambiguation left to the sibling scrub pace, per its deferral.

### 2026-06-30 19:45 - ₢BfAAu - W

Built bug_require_clean_tree_creed — the well-formed precision-band clean-tree gate: buc_rejects a new BUBC_band_clean_tree=108, states the condition from BUG_clean_tree_condition tinder, and takes the RB rationale as a parameter homed in RBCC_creed_clean_build (opinion kit-side, BUG kit-agnostic). Old malformed bug_require_clean_tree untouched; no call site wired. Shellcheck clean (221 files), BUK self-test green (7 fixtures / 49 cases, band-survival included).

### 2026-06-30 19:44 - ₢BfAAu - n

Rewrite the clean-tree check to explicit if-form (! git diff --quiet || ! git diff --cached --quiet) — clears shellcheck SC2015 (A && B || C is not if-then-else) for BCG compliance.

### 2026-06-30 19:43 - ₢BfAAu - n

Add well-formed bug_require_clean_tree_creed precision-band clean-tree gate: mint BUBC_band_clean_tree=108, BUG_clean_tree_condition error-condition tinder, and RBCC_creed_clean_build creed rationale (RB opinion kit-side, BUG kit-agnostic). Old malformed bug_require_clean_tree untouched; no call site wired.

### 2026-06-30 19:33 - Heat - d

batch: 2 reslate

### 2026-06-30 19:33 - Heat - r

moved ₢BfAAv to first

### 2026-06-30 15:36 - Heat - r

moved ₢BfAAq before ₢BfAAz

### 2026-06-30 15:35 - Heat - r

moved ₢BfAAp after ₢BfAAv

### 2026-06-30 15:35 - Heat - d

batch: 2 reslate

### 2026-06-30 15:19 - Heat - r

moved ₢BfAAF before ₢BfAAL

### 2026-06-30 15:19 - Heat - d

batch: 2 reslate, 3 slate

### 2026-06-30 14:52 - Heat - d

paddock curried: gap 7: drop pool + session-duration from the per-foedus regime core (manor-level under the Model)

### 2026-06-30 14:51 - Heat - d

batch: 2 reslate, 1 slate

### 2026-06-30 14:37 - Heat - n

Drain the 260630 one-pool model reversal (per-foedus-pool topology superseded by the one-pool identity substrate) into the heat-memories memo: prior framing, replacement, reasoning, and the slated rework disposition

### 2026-06-30 14:28 - Heat - T

foedus-pool-state-classifier

### 2026-06-30 14:28 - Heat - d

batch: 1 reslate, 1 slate

### 2026-06-30 14:20 - Heat - d

paddock curried: 260630 cleanup: remove superseded sections + reversal changelog (skidmarks); state one-pool model present-tense; sweep stale per-foedus-pool lines; reversal record to git history + heat-memories memo

### 2026-06-30 14:10 - Heat - d

paddock curried: Model rewrite 260630: seat the one-pool identity substrate as the governing federation-identity model; supersede per-foedus-pool; mark reversed cinches; banner the two reversed sections

### 2026-06-30 10:12 - ₢BfAAM - W

Built the programmatic (uploaded-JWKS) arm of affiance, mechanism-gated, REST-only. rbrf_regime.sh recast from flat interactive-only to the RBSRF core+gate model: RBRF_MECHANISM enum (rbnfe_interactive/rbnfe_programmatic, modeled on RBRV_VESSEL_MODE) as a core field; device-flow group (scope/device/token endpoints) gated on rbnfe_interactive and the programmatic group (RBRF_IDP_JWKS_JSON) on rbnfe_programmatic via buv_gate_enroll; interactive-only custom format checks moved under a mechanism case so they never touch unset gated fields, https-issuer + google.subject kept core, JWKS-parse check added under programmatic. rbgp_manor_affiance's provider-create body branched on RBRF_MECHANISM — interactive keeps the proven issuer-discovery+webSsoConfig body, programmatic adds opaque oidc.jwksJson (read verbatim, vendor-blind) plus the inert webSsoConfig carried proof-aligned (RBSMA impl-confirm, tied to the orchestrator's first live create). RBRF_MECHANISM=rbnfe_interactive declared in rbef_entrada. Verified: shellcheck clean (219), rbw-rfv PASS (JWKS gated/skipped under interactive), enrollment-validation 47/47 incl. all gate tests, regime-poison 30/30, programmatic jq snippets unit-checked. Live cloud create rides BfAAL (the orchestrator) by design. Scope kept off affiance's pool-state fast-fail (BfAAs). One file: notch ee536ef.

### 2026-06-30 10:07 - ₢BfAAM - n

Add the programmatic (uploaded-JWKS) arm to affiance, mechanism-gated. Recast rbrf_regime.sh from a flat interactive-only shape to the RBSRF core+gate model: enrolled RBRF_MECHANISM (rbnfe_interactive/rbnfe_programmatic enum, modeled on RBRV_VESSEL_MODE) as a core field, gated the device-flow group (scope/device-endpoint/token-endpoint) on rbnfe_interactive via buv_gate_enroll, and added a programmatic group (RBRF_IDP_JWKS_JSON) gated on rbnfe_programmatic; moved the interactive-only custom format checks under a mechanism case so they never touch the unset gated fields, kept https-issuer + google.subject as core checks, and added a JWKS-parse check under programmatic. Branched rbgp_manor_affiance's provider-create body on RBRF_MECHANISM: interactive keeps the proven issuer-discovery+webSsoConfig body; programmatic adds opaque oidc.jwksJson (read verbatim from the regime, vendor-blind) and carries the inert webSsoConfig proof-aligned (RBSMA impl-confirm to settle at the orchestrator's first live create). Declared RBRF_MECHANISM=rbnfe_interactive in rbef_entrada so the standing trust validates under the gated regime. Pool/org-grant steps unchanged (mechanism-invariant); affiance's live-pool fast-fail left to BfAAs.

### 2026-06-30 09:46 - ₢BfAAN - W

Proved Keycloak 26.6.3's RFC 7523 JWT Authorization Grant live, then de-laminated configure_realm into a fully baked fdkyclk-realm.json (jwt-authorization-grant asserting IdP + committed-asserter public key; GCP-facing confidential client + oauth2.jwt.authorization.grant.* + audience mapper; federated-linked pinned user; https frontendUrl issuer; no baked signing key -> ephemeral). Committed a caged test asserter keypair; de-laminated fdkyclk-proof.sh (configure_realm + kc_admin_token removed, mint swapped password->RFC 7523, local `mint` mode added). Done-when VERIFIED: the baked vessel boots via --import-realm and mints an id_token (iss=frontend issuer, aud=fdkyclk-gcp, sub=pinned subject) with zero runtime admin-REST and zero GCP. Beyond the docket (operator-directed): minted RBSFK + the rbtf_realm quoin — the baked-realm DATA contract, IdP-side companion to rbtf_foedus, homing the two-keys doctrine and the harness-surveyed first-contact trips — and de-staled the overturned stable-key/estate-secret/committed-JWKS model to ephemeral across RBSFE/RBSRF/RBS0/registry. Flagged residual for the establishment pace BfAAL: whether RBRF_IDP_JWKS_JSON commits a value vs establish-populated under ephemeral keys.

### 2026-06-30 09:42 - ₢BfAAN - n

Local mint verification surfaced a frontendUrl audience trip the dev run could not: once frontendUrl is baked, the assertion aud must be the realm's FRONTEND issuer (https://fdkyclk.test/realms/fdkyclk), not the localhost token endpoint the caller POSTs to — the latter draws 'Invalid token audience'. Fixed fdkyclk-proof.sh mint_idtoken to set aud=$ISSUER (frontend issuer) while still POSTing to the localhost endpoint, and sharpened the RBSFK first-contact-trips aud line from the loose 'token endpoint or issuer' to the proven frontend-issuer fact. Done-when now VERIFIED: bash fdkyclk-proof.sh mint boots the baked realm via --import-realm and mints via RFC 7523 with zero admin-REST / zero GCP, yielding iss=frontend issuer, aud=fdkyclk-gcp, sub=pinned subject (the federatedIdentities link imported correctly; the user-list repr just omits the sub-resource).

### 2026-06-30 09:37 - ₢BfAAN - n

Drove RBRN_BOTTLE_HALLMARK=k260630093652-c4ec99f0c from re-kludging the fdkyclk bottle with the baked RFC 7523 realm — so the crucible charges the new image carrying the fully-baked realm for the local mint verification.

### 2026-06-30 09:36 - ₢BfAAN - n

De-laminate the fdkyclk realm config into baked DATA per RBSFK. fdkyclk-realm.json now carries every entry configure_realm applied live: the jwt-authorization-grant asserting IdP (publicKeySignatureVerifier = the committed test asserter's public half, kid, RS256), the GCP-facing confidential client (oauth2.jwt.authorization.grant.{enabled,idp} + gcp-aud audience mapper), the federated-linked federate user (pinned id = the teardown SUBJECT for a stable GCP subject), and the https frontendUrl issuer — no baked signing key, so Keycloak mints it ephemeral on import. Committed fdkyclk-asserter-key.pem as caged test scaffolding (its public half is the baked publicKeySignatureVerifier). fdkyclk-proof.sh de-laminated: configure_realm + kc_admin_token removed (zero runtime admin-REST), mint_idtoken swapped password-grant -> RFC 7523 jwt-bearer (signs an assertion with the asserter key), added a `mint` local-only mode for the Done-when check (boots baked realm + mints, no GCP). fetch_jwks kept as the per-charge twiddle. Pending verification: re-kludge bottle + charge + local mint check.

### 2026-06-30 09:29 - ₢BfAAN - n

Mint RBSFK + the rbtf_realm quoin: a baked-realm DATA contract capturing the RFC 7523 JWT-authorization-grant realm shape proven live against Keycloak 26.6.3. rbtf_realm is the IdP-side companion to rbtf_foedus (GCP-side trust); the subdoc contracts the five required --import-realm entries (no baked signing key -> ephemeral; jwt-authorization-grant asserting IdP + publicKeySignatureVerifier; GCP-facing confidential client + oauth2.jwt.authorization.grant.* + audience mapper; federated-linked pinned subject; https frontendUrl issuer), the two-keys doctrine, and the harness-surveyed first-contact trips (issued sub = internal user id so pin it; jti/exp/aud; GCP strict-JWKS RSA-member strip). Registered via the Terrier noun+subdoc pattern (mapping entry, Term-Definitions definition, leveloffset include) + acronym registry. De-staled the overturned stable-key/committed-JWKS/estate-secret model to the ephemeral two-key model wherever it contradicted RBSFK: RBSFE (keystone NOTE, step 1, completion, membrane ref, RBS0 mount-intro), RBSRF rbrf_idp_jwks_json definition, and both registry entries. Residual flagged for the establishment pace: whether RBRF_IDP_JWKS_JSON commits any value vs establish-populated under ephemeral keys. Spec validated by grep: all 11 RBSFK quoin refs resolve, anchor unique, NOTE blocks delimiter-balanced, no residual stable-key phrases corpus-wide.

### 2026-06-30 08:39 - ₢BfAAN - n

Drove RBRN_SENTRY_HALLMARK=k260630083919-80d0794f0 from a fresh fdkyclk sentry kludge — completes the nameplate hallmark pair so the fdkyclk crucible can charge for live RFC 7523 realm development.

### 2026-06-30 08:39 - ₢BfAAN - n

Drove RBRN_BOTTLE_HALLMARK=k260630083845-535203e3f from a fresh fdkyclk bottle kludge — populates the nameplate's bottle hallmark so the sentry kludge's clean-tree gate clears and the crucible can charge for live RFC 7523 realm development.

### 2026-06-30 08:29 - ₢BfAAb - W

Folded the five mkdir+seed-into-previous pairs in butcfc_facts.sh into a dedicated zbutcfc_seed_previous(name, value) wrapper that hardcodes BURD_PREVIOUS_DIR and keeps the mkdir inline (readonly-path constraint). OUTPUT_DIR seed left on the bare zbutcfc_seed helper; the two unseeded cases untouched. Exactly one mkdir survives (in the wrapper); five call sites collapsed to single calls. Green: shellcheck 219 clean, BUK self-test 7 fixtures/49 cases, fact-chaining 7/7.

### 2026-06-30 08:29 - ₢BfAAb - n

Fold the five mkdir+seed-into-previous pairs in the BUK fact-chaining self-test into a dedicated zbutcfc_seed_previous(name, value) wrapper that hardcodes BURD_PREVIOUS_DIR and keeps the mkdir inline (readonly-path constraint). The OUTPUT_DIR seed stays on the bare zbutcfc_seed helper; the two unseeded cases untouched. Shellcheck 219 clean.

### 2026-06-27 14:09 - Heat - S

clean-tree-gate-precision-variant

### 2026-06-27 14:08 - Heat - d

batch: 1 reslate

### 2026-06-27 13:14 - Heat - d

batch: 1 reslate

### 2026-06-27 13:14 - Heat - d

paddock curried: finalize: signing key -> ephemeral+twiddle, no persistence (drops BfAAr/BfAAP, supersedes 260623/260624 for the fed key); canvass -> active (not deferred); name-as-folio settled by rbef_ sprue

### 2026-06-27 13:12 - Heat - T

estate-secret-verification

### 2026-06-27 13:12 - Heat - T

keycloak-estate-key-genesis

### 2026-06-27 13:08 - Heat - S

terrier-band-discrimination

### 2026-06-27 12:37 - Heat - d

paddock curried: staleness repair: ₢BfAAT landed the switching design (RBSFD/RBSFI/RBSRR/RBSRF) — reframe settled-block as AUTHORED, correct cognomen to evicted-whole, add rbef_ sprue (rbef_entrada/rbef_keycloak) + RBRR_ACTIVE_FOEDUS, note code-enrollment owed, mark 2c declined-YAGNI

### 2026-06-27 12:05 - Heat - S

foedus-pool-state-classifier

### 2026-06-27 12:04 - Heat - d

paddock curried: record settled foedus-switching design (RBRR selector + committed library, cognomen evicted, instate/descry atomic own-band, RBRS-only key, affiance fast-fail, terrier-rigor to theurge, canvass deferred); overturns the selector open-question and the committed-JWKS clause

### 2026-06-27 11:34 - ₢BfAAT - W

Authored the foedus test-bed switching mechanism contract-first. Two new operation subdocs: RBSFD (descry — palpate-adjacent read of a named foedus's pool health, own precision band, not a chaining consumer) and RBSFI (instate — enchase-adjacent rewrite of the RBRR_ACTIVE_FOEDUS selector, own band, not a chaining link, carrying the never-a-fat-verb fixture-composition contract). RBS0: wired both operation quoins, retired the rbrf_cognomen quoin, flipped RBRF singleton->manifold, repointed rbtf_foedus identity to its subdirectory name, cross-linked canvass to descry, seated rbrr_active_foedus + the federation-selection group. RBSRR: added the RBRR_ACTIVE_FOEDUS selector field under a new Federation Selection group. RBSRF: restructured to a library (one rbrf.env per foedus subdirectory, subdir name = identity). Evicted RBRF_COGNOMEN whole (spec-only, zero code consumers). Minted the rbef_ foedus-instance sprue with standing members rbef_entrada (interactive Entra trust) and rbef_keycloak (programmatic test trust), documented as the operator-facing folios passed to instate/descry. Registered RBSFD/RBSFI acronyms. Conflict-checked via groom of Bf/Bi/Bl: BfAAT is the paddock-assigned owner; downstream foedus-reuse-leg/canvass build paces consume these subdocs behind the BlAAR barrier; the chaining-roles boundary text stays owned by BiAAk (cited, not edited).

### 2026-06-27 11:29 - ₢BfAAT - n

Refine foedus naming per operator decision. Rename the active-foedus selector RBRR_FOEDUS -> RBRR_ACTIVE_FOEDUS (quoin rbrr_active_foedus) across RBS0/RBSRR/RBSRF/RBSFD/RBSFI + the acronym registry. Mint the rbef_ foedus-instance sprue (the well-formed underscore cousin of the malformed rbev- vessel-instance sprue) and document the two standing members rbef_entrada (the interactive Entra trust) and rbef_keycloak (the programmatic test trust). Document the foedus identity as the operator-facing folio passed to the instate/descry tabtargets, in RBSRF's library description, the RBRR_ACTIVE_FOEDUS field (RBSRR + RBS0 civic def), and both operation subdocs. Conflict-checked against heats Bf/Bi/Bl via groom: BfAAT is the paddock-assigned owner of the selector/identity minting, downstream foedus-reuse-leg and canvass build paces consume these subdocs behind the BlAAR barrier, and the chaining-roles boundary text stays owned by BiAAk (cited, not edited here).

### 2026-06-27 10:38 - ₢BfAAT - n

Author the foedus test-bed switching mechanism contract-first. New operation subdocs RBSFD (foedus_descry) and RBSFI (foedus_instate) in the deferred-build register: descry reads a named foedus's pool health (palpate-adjacent, resolves no chained fact so NOT a rbch_palpate member, own precision band); instate rewrites the RBRR_FOEDUS selector (enchase-adjacent, reuses feoff/yoke/anoint atomic-rename, no self-gate, operator commits, NOT a rbch_enchase member, own band distinct from descry's since the two co-occur in the fixture's reuse-or-establish spawn path). RBSFI carries the fixture-composition contract (reuse-if-valid-else-create lives in the fixture composing descry+instate, sharing depot-ensure's combinator, never a fat verb). RBS0: wired both operation quoins (mapping refs + anchor/heading/include stanzas), retired the rbrf_cognomen quoin, flipped RBRF axrd_singleton->axrd_manifold, repointed rbtf_foedus identity to its subdirectory name, cross-linked canvass to descry, seated rbrr_foedus + rbrr_group_federation_selection quoins, updated moorings layout. RBSRR: added the RBRR_FOEDUS selector field under a new Federation Selection group with cross-domain-reference and production-rarely-changes notes. RBSRF: restructured to a library (one rbrf.env per foedus subdirectory, subdir name = identity), evicted RBRF_COGNOMEN whole (spec-only, zero code consumers). Registered RBSFD/RBSFI acronym entries. Aligns with the Bf-i chaining-roles boundary.

### 2026-06-27 10:07 - Heat - d

batch: 1 reslate

### 2026-06-27 09:30 - Heat - d

batch: 1 reslate

### 2026-06-26 10:55 - Heat - f

silks=rbk-00-mvp-review-federation-build

### 2026-06-26 10:29 - ₢BfAAO - W

Federation spec-first recast (contract-before-code). Recast RBSRF + RBSMA to the vendor-agnostic-core + acquisition-mechanism-gate model: RBRF_MECHANISM gating interactive/programmatic (rbnfe_ wire tokens), modeled on RBRV_VESSEL_MODE; device endpoints/scope interactive-gated, uploaded public JWKS programmatic-gated. Seated rbtf_foedus + rbtf_canvass civics. Created the RBSFE (programmatic establishment) and RBSFA (programmatic accessor) contract subdocs with descriptive operation quoins (civic verbs deferred), wired into RBS0. Made rbsk_human_present and the three governor/director/retriever authenticate patterns mechanism-conditional. Recorded jilt mechanism-blind and fixed its stale undelete->refuse-and-rotate drift. Registered the RBSMA/RBSMJ/RBSFE/RBSFA acronym entries. Landed cognomen as the foedus's stable identity (RBRF_COGNOMEN, voiced //axrg_variable axtu_xname), demoting RBRF_WORKFORCE_POOL_ID to the rotating realization. Added moniker to the MCM fallow-word seeds. Deliberately deferred: the key-role voicing (axt_key/axd_key) + rbst_ identifier-shelf retrofit to a CMK pass, and the foedus-switch mechanism to the foedus-reuse design.

### 2026-06-26 10:22 - ₢BfAAO - n

Add moniker to the MCM fallow-word seed list (Soil Family) — the editor reserves it as freely-grazable explanatory vocabulary, withheld from minting, alongside mesentery and constellation. Operator decision made during the foedus-identifier discussion, which is why it rode this pace.

### 2026-06-26 10:18 - ₢BfAAO - n

Land cognomen as the foedus's stable identity: add RBRF_COGNOMEN (operator-assigned durable name, voiced axrg_variable axtu_xname per RBRF's direct-axtu convention — bare identifier format, key-role voicing deferred to the CMK pass) and demote RBRF_WORKFORCE_POOL_ID from 'the foedus identifier' to 'the pool that realizes it and may rotate' (refuse-and-rotate). Flip every pool-id-as-identity site: the active-foedus regime note, the rbrf_regime header, the rbtf_foedus civic def. How the cognomen is carried in the Manor for canvass is held at contract altitude (the deferred switch mechanism).

### 2026-06-26 08:49 - ₢BfAAO - n

Register the missing acronym entries: RBSMA (manor_affiance) and RBSMJ (manor_jilt) — pre-existing gap, both specs existed uncatalogued — plus the two new RBSFA (foedus_acquire) and RBSFE (foedus_establish) contract subdocs, in alphabetical RBS-family order.

### 2026-06-26 08:47 - ₢BfAAO - n

Record jilt as mechanism-blind (no RBRF_MECHANISM arm — pool teardown is identical for interactive and programmatic foedera; its one interactive touch, the operator confirm, rides the orthogonal confirm-skip seam). Fix the stale soft-delete-graveyard drift: a later affiance against a soft-deleted same-id pool meets 200/DELETED and REFUSES (refuse-and-rotate), never undeletes — superseding the old 404/409-then-undelete claim, matching RBSMA; undelete stays operator-manual recovery.

### 2026-06-26 08:46 - ₢BfAAO - n

Create the two programmatic-foedus contract subdocs: RBSFE (rbtgo_foedus_establish — marshal-only establishment: stable keypair + uploaded public JWKS + regime facts, hands off to affiance's programmatic branch, durable-secret quarantine keystone, harness-proven, REST-only) and RBSFA (rbtgo_foedus_acquire — programmatic accessor: RFC 7523 self-supplied JWT → STS, arm of rba, reaches federated token only, don deferred to attach-caged-subject unit). Descriptive operation quoins, civic verbs deferred. Wired into RBS0 with mapping refs + body anchor/heading/include sections after manor_jilt.

### 2026-06-26 08:42 - ₢BfAAO - n

Seat rbtf_foedus (configured-federation noun, fair-faced first-contact gloss) and rbtf_canvass (read-only foedera enumeration via workforcePools.list, Manor-as-registry) in the federation civics + mapping refs. Make rbsk_human_present mechanism-conditional (interactive keeps the human-avow/no-refresh-token premise; programmatic voids it, bounded to the test-bed) and fork the three governor/director/retriever authenticate patterns (interactive: live sitting via avow / TTY / fail-loud; programmatic: self-supplied-JWT STS exchange, no sitting, no human).

### 2026-06-26 08:39 - ₢BfAAO - n

Make affiance mechanism-conditional: the vendor-agnostic core reads unconditionally, RBRF_MECHANISM selects the provider oidc body — interactive builds issuerUri + webSsoConfig (issuer-discovered), programmatic builds issuerUri + clientId + oidc.jwksJson (uploaded public JWKS). Vendor-neutralize the attribute-mapping example, make webSsoConfig interactive-meaningful/programmatic-inert with the gcloud-demands-it impl-confirm flag, add the strict-JWKS-parser RSA-member constraint, and fork the completion follow-up (avow vs self-minted STS exchange).

### 2026-06-26 08:37 - ₢BfAAO - n

Recast RBRF to vendor-agnostic core + acquisition-mechanism gate: seat RBRF_MECHANISM discriminator (rbnfe_interactive/rbnfe_programmatic) modeled on RBRV_VESSEL_MODE, re-home device endpoints/scope into the interactive group and the uploaded public JWKS into the programmatic group, degrade issuer validation under programmatic, and add the Manor-as-registry foedus-selection note + per-vendor-guide contract line. Mapping refs and RBS0 quoin defs (mechanism, value quoins, group quoins, jwks field) seated.

### 2026-06-26 08:22 - Heat - f

racing

### 2026-06-26 08:04 - Heat - d

paddock curried: census: record canvass (foedus read-all verb, descry sibling) as chosen — Lapidary-gated, rbw-jc / rbtf_canvass, build+test slated as placeholders in the theurge stream

### 2026-06-26 07:48 - Heat - d

paddock curried: census: record RBRF_MECHANISM (Call 1) + RBSFE/RBSFA subdoc acronyms (Call 2) as chosen; selector (Call 3) under discussion with Manor-as-registry lean noted

### 2026-06-26 07:40 - Heat - d

paddock curried: census: record keep interactive/programmatic + rbnfe_ sprue as chosen; correct instate/descry (minted, not built); reframe selector as test-only overlay

### 2026-06-26 07:20 - Heat - f

stabled, silks=rbk-40-fbl-review-federation-build

### 2026-06-26 07:05 - Heat - f

silks=rbk-04-mvp-federation-build

### 2026-06-26 06:59 - Heat - d

paddock curried: add Federation vocabulary census after Charter — single prominent chosen+owed roster, closing the scatter that caused a fresh-instance re-derive

### 2026-06-25 15:24 - ₢BfAAi - W

First run of the drain practice: drained the ₣Bf paddock 394→308 lines. Moved 7 dispositioned ideas' deliberation to the heat-memories memo (foedus noun-selection, multiplicity-as-goal, degenerate-test-federation, freehold reconciliation, terrier option-fork, foedus-accessor minted words, one-click verification_uri_complete decline) — deliberation preserved, not lost. 3 sections removed, 4 thinned to live residuals, ~14 de-stales (compear→avow, assize→sitting, compearance→avowal, rbw-xkX→rbw-qjK, ₣BZ-gating on the attach tail released as slatable, spent Fable-gating clauses trimmed, Sources extended). Caught and avoided over-draining the live keystone four-line model that the governor's-role and health-assurer ideas lean on.

### 2026-06-25 15:22 - ₢BfAAi - n

Drain ₣Bf dispositioned deliberation to heat-memories: foedus noun-selection, multiplicity-goal framing, degenerate-test-federation, freehold reconciliation, terrier option-fork, foedus-accessor minted words, one-click verification_uri_complete decline

### 2026-06-25 15:22 - Heat - d

paddock curried: drain pass ₢BfAAi: dispositioned deliberation moved to heat-memories memo; ₣BZ-gating/compear/assize/rbw-xkX de-staled

### 2026-06-25 16:43 - Heat - d

batch: 1 reslate, 1 slate

### 2026-06-25 16:42 - Heat - n

Process retrospective for the three-heat (₣Bf/₣Bi/₣Bl) plan-integrity audit: the cold re-derive + adversarial-verify methodology, why it earned confidence (it refuted the auditor's own two scariest HIGH guesses), the merge-semantics reasoning, the reusable playbook, and honest residuals (the JWKS-producer gap, the rbq_qualify dead colophon-check).

### 2026-06-25 16:29 - Heat - d

paddock curried: integrity-audit fix: de-silks the reuse-path framing line (was the ₣Bl reuse-build pace's display name)

### 2026-06-25 16:23 - Heat - d

batch: 5 reslate

### 2026-06-25 14:21 - Heat - n

Add the three-heat plan-integrity audit directive — a read-only, fresh-chat audit prompt (full re-derive + reconcile) for the Bf/Bi/Bl parallelization plan: re-footprint every remaining pace cold, rebuild the cross-clone dependency graph, and reconcile it against the installed barriers/order/assignments to catch the invisible class (barrier gaps), plus referent dangles, topology staleness, ordering/cycles, coverage, and paddock-vs-inventory drift. Encodes the operator-confirmed ground truths (monotonic no-recycle coronets so dangles never collide; the hot-file contention/ownership model) and the read-only, severity-ranked, propose-not-mutate posture.

### 2026-06-25 06:02 - Heat - d

batch: 1 reslate

### 2026-06-24 23:33 - Heat - S

await-thg-registry-relocation

### 2026-06-24 23:33 - Heat - S

await-thg-zipper-baton

### 2026-06-24 23:32 - Heat - T

fed-syncs-theurge-zipper-crucible

### 2026-06-24 23:17 - ₢BfAAk - W

Workflow-verified cross-heat split of ₣Bf/₣Bi/₣Bl into three clone-able streams by file territory (16/16/7): FED federation vertical (owns RBS0-SpecTop + acronym registry), THG theurge-refactor (relabelled+un-stabled ₣Bl, owns rbtdrc_crucible.rs + the zipper trio), MVP the narrowed ₣Bi remainder. Realized end-to-end on operator approval: 3 jjx_transfers, the ₣Bl un-stable+relabel and ₣Bf relabel, all three paddocks restitched under one drain map, the landed suite-rename + Fable-gate + dingleberry deliberation drained to the heat-memories memo (parley left live in the FED paddock), and three cross-clone coordination gate paces installed (the zipper baton THG->FED->MVP, the RBS0 forward-sync, the crucible delta, the foedus spec-first window). Resolved the three operator-deferred calls: ₢BiAAH/₢BiAAI stay in MVP on soft-contention grounds (rebalancing 18/16/5 -> 16/16/7); the suite-rename record drains to the memo while the live parley reservation stays in FED; the occurrencesViewer premise is false on three counts (RBSRK was deleted 260619, RBSDC already homes the grant correctly, the camelCase token mis-renders a correctly-granted dotted role) so the clause was dropped from ₢BiAAH along with its parallel stale RBSDK references. Also excised the 7 dangling cult-triad acronym pointers the 260619 demolition sweep missed.

### 2026-06-24 23:12 - Heat - d

batch: 1 reslate

### 2026-06-24 23:09 - Heat - S

fed-syncs-theurge-zipper-crucible

### 2026-06-24 23:01 - ₢BfAAk - n

Drain the dispositioned federation-paddock deliberation into the heat-memories provenance memo as part of the split-study restitch: the landed five-suite rename record (with parley left live in the federation paddock), the lifted Fable-adjudicator gate and the in-heat-minting decision that replaced it, the now-spent vocabulary-remint dingleberry doctrine, and a forward-note that the theurge-test-consolidation audit dispositions relocated into the theurge-refactor stream's charter rather than this memo.

### 2026-06-24 22:58 - Heat - d

paddock curried: restitch into the federation-build stream (split study): charter reframed, theurge/cosmology drained to the sibling theurge stream, dingleberry/Fable framing drained to heat-memories, federation-MVP band + coupling-to-watch + chaining-fact constraint folded in

### 2026-06-24 22:44 - Heat - f

silks=rbk-14-mvp-federation-build

### 2026-06-24 22:44 - Heat - D

restring 3 paces from ₣Bi

### 2026-06-24 22:40 - ₢BfAAk - n

Record the cross-heat split study: the 16/16/7 partition of Bf/Bi/Bl by file territory (FED federation vertical owns RBS0+acronyms; THG theurge-crate+zipper anchored on relabelled/un-stabled Bl; MVP the Bi remainder), the hot-file serialization choreography, the heat realization (3 transfers + relabels), and the three deferred-call resolutions (BiAAH/BiAAI stay in MVP on soft-contention grounds, rebalancing to 16/16/7; suite-rename record drains to heat-memories while the live parley reservation stays in the FED paddock; occurrencesViewer premise is false on three counts so the clause is dropped from BiAAH). Also excised 7 dangling cult-triad acronym pointers (RBSDK/RBSRK/RBSDD/RBSRD/RBSDR/RBSRL/RBSGM) the 260619 demolition sweep missed in claude-rbk-acronyms.md.

### 2026-06-24 14:11 - Heat - d

batch: 1 reslate

### 2026-06-24 13:58 - ₢BfAAh - W

Ran the workflow-driven Bf heat-integrity audit (49 agents; 6 dimensions; adversarial per-finding verification + completeness critic). Verdict: plan sound (clean dependency DAG, dense coverage), one HIGH correctness defect + a cluster of coverage/spec-home gaps. Closure enrolled on operator review — reslated BfAAN (stable-key cinch reversing the superseded per-charge JWKS rotation), BfAAW (service->picket), BfAAO (+foedus quoin, mechanism-conditional human-present premise/rbtoe, jilt assessment), BfAAF (+new operation subdoc, RBS0:2036 fix, dropped wrong RBSTR/RBSMF targets), BfAAT/BfAAg (parley ownership + descry/instate own-subdoc, no exemption), BfAAe (parley characterization), BfAAi (named de-stale targets compear/assize/rbw-xkX), BfAAP (reshaped to the source-plus-verify estate stub over the shelved interim station file, railed before BfAAN); slated BfAAj (colophon-relocation-260624, order 4); recorded the estate-stub decision + multi-citizen-roster deferral in the paddock; cantled BfAAk (cross-heat parallelization split study). RBSRS regime confirmed shelved in FUTURE/ for podman/BW, not revived.

### 2026-06-24 13:54 - Heat - S

cross-heat-parallelization-split-study

### 2026-06-24 13:44 - Heat - d

batch: 1 reslate

### 2026-06-24 13:41 - Heat - d

batch: 2 reslate

### 2026-06-24 13:33 - Heat - d

batch: 2 reslate

### 2026-06-24 13:29 - Heat - d

batch: 1 reslate

### 2026-06-24 13:28 - Heat - r

moved BfAAP before BfAAN

### 2026-06-24 13:24 - Heat - d

paddock curried: estate-secret home settled as a minimal source-plus-verify stub over the shelved interim station file (RBRS stays shelved for podman/₣BW); supersedes the Estate-regime-nucleation framing; + record the multi-citizen roster-slice deferral

### 2026-06-24 13:22 - Heat - S

colophon-relocation-260624

### 2026-06-24 13:22 - Heat - d

batch: 4 reslate

### 2026-06-24 12:03 - ₢BfAAR - W

Renamed the four test suites to the military asterism — fast→reveille, service→picket, crucible→bivouac, complete→echelon — across every surface: the RBTDRC_SUITES table, the zipper help string (regenerating tabtarget-context.md), CLI/onboarding text, the CLAUDE.md suite tables + prose, and the ACG / theurge-ifrit context docs. Per operator full-coherence ruling, the sweep also took the entire fast-tier→reveille-tier dependency-tier vocabulary, including the two ashlar credless-guard error strings (rba_auth, rbgp_payor) and the BUS0/RBSLE specs. crucible→bivouac was the forced fix (it shadowed the production crucible runtime noun). Tabtargets git-mv'd; 5 dockets reslated (BfAAF/U/V/Z/c); paddock suite-naming disposition marked DONE + fast-base→reveille-base; the stale BfAAV silks relabeled to reveille-base-set-equality-assertion. Disambiguation preserved runtime-crucible, the fail-fast idiom, fast-qualify (a separate op), and English 'complete'. JJK jjrm_mcp.rs kept its TestSuite.fast.sh format-example (foreign-kit illustration, not a suite home). Verified green: build clean, shellcheck 217 clean, fast-qualify green incl. consts/context freshness, and the reveille suite 113/0 (credless guard + conformance among them).

### 2026-06-24 12:01 - ₢BfAAR - n

suite-asterism-rename: sweep the four test-suite names to the military asterism (fast→reveille, service→picket, crucible→bivouac, complete→echelon) across every surface — RBTDRC_SUITES table, the zipper help string (regenerating tabtarget-context.md), CLI/onboarding text, CLAUDE.md tables/prose, ACG/theurge-ifrit context docs; and per operator full-coherence ruling, the whole fast-tier→reveille-tier dependency-tier vocabulary including the two ashlar credless-guard error strings (rba_auth, rbgp_payor) and the BUS0/RBSLE specs. crucible→bivouac is the forced one (collided with the production crucible runtime noun). Build + shellcheck (217 clean) + fast-qualify green. Tabtarget renames, 5 docket reslates, and the paddock disposition landed in prior commits this pace.

### 2026-06-24 12:01 - Heat - T

reveille-base-set-equality-assertion

### 2026-06-24 11:59 - Heat - d

paddock curried: suite-asterism-rename: mark suite-naming disposition DONE (renamed 260624, full coherence) + fast-base→reveille-base

### 2026-06-24 11:56 - Heat - d

batch: 5 reslate

### 2026-06-24 02:11 - ₢BfAAS - W

Minted the foedus-accessor vocabulary in-heat and swept the two evicted words tree-wide. Elected: avow (sign-in, rbw-aa, cognitive-rhymes OIDC 'assertion'); sitting (live-window noun, rhymes vendor 'session', keep-a-noun fork resolved to keep); quash (clear-the-sitting, test bucket rbw-q); instate (active-foedus switch, rbw-j, casing fork carried to the switch-design pace); descry (foedus check, rbw-jd, rhymes Google 'describe'). Swept compear→avow and assize→sitting across 33 files (accessor identifiers, federation specs/quoins rbtf_compear→rbtf_avow + rbtf_assize→rbtf_sitting, error text, README, rbrf.env, theurge Rust); build + shellcheck (214 clean) + fast suite (113/0) green. Settled colophon homing (rbw-ac→rbw-a; gird→rbw-mG leaving rbw-p purely governor; rbw-j foedus-cardinality; rbw-q test/qualify; rbw-qjK keycloak) and the casing convention (uppercase 2nd letter = mutates), formalized in BUS0 signet_case. Folded names into the ₣Bf paddock (MINTED section) and the AAg/AAA/AAT dockets. Deferred (infra-gated): the rbdgm_federation-seam diagram puml sweep + svg re-render via the pluml crucible — the lone tree remnant on the old words. The ₣Bi dockets ₢BiAAA/₢BiAAJ still reference the old words — left for ₣Bi grooming.

### 2026-06-24 02:08 - ₢BfAAS - n

Amend BUS0 Tabtarget Vesture (signet_case) to formally permit case-significant colophon suffixes: the cipher prefix stays lowercase, but suffix letters may be upper or lower case and the case distinguishes colophons (kept filesystem-safe by their differing epithets); a consumer may assign the case a meaning by convention. Formalizes the casing convention the foedus-accessor colophon pass relies on (RBK: an uppercase suffix letter marks a state-mutating op).

### 2026-06-24 02:02 - Heat - d

batch: 3 reslate

### 2026-06-24 01:59 - Heat - d

paddock curried: Fold the minted foedus-accessor vocabulary (avow/sitting/quash/instate/descry) + colophon map; consolidate the three decided remint ideas into one MINTED record; de-stale the directly-affected leans. Comprehensive narrative de-stale left to the terminal-groom refocus per the drain doctrine.

### 2026-06-24 01:49 - Heat - n

Track the two ₣Bf provenance memos, untracked since creation: the heat-memories paddock-drain memo and the theurge-test-consolidation audit memo (audit findings, inventory-by-stratum, duplication map, suite-naming slate, draft RBS0 cosmology subdoc, dependency-ordered pace plan). Committing to clean the working tree for theurge suite runs.

### 2026-06-24 01:47 - ₢BfAAS - n

Eviction sweep: compear→avow, assize→sitting tree-wide. Renames the federation sign-in verb and live-window noun across the accessor (rba_auth.sh identifiers ZRBA_*_ASSIZE_*→*_SITTING_*, rba_compear→rba_avow), the federation specs/quoins (rbtf_compear→rbtf_avow, rbtf_assize→rbtf_sitting + all refs), error text (assize lapsed, compear → sitting lapsed, avow), README broadside, rbrf.env comments, and the theurge Rust. Case-preserving, with compearance→avowal and article (a/an) fixes. Regenerated zipper-derived rbtdgc_consts.rs + claude-rbk-tabtarget-context.md. Build + shellcheck green. Diagram puml/svg and paddock/docket fold deferred.

### 2026-06-24 00:57 - Heat - d

batch: 2 reslate

### 2026-06-23 22:41 - Heat - S

paddock-refocus-and-drain

### 2026-06-23 22:40 - Heat - S

heat-integrity-and-coverage-audit

### 2026-06-23 22:35 - Heat - T

compear-tty-to-yawp

### 2026-06-23 22:35 - Heat - T

heed-headless

### 2026-06-23 22:35 - Heat - d

batch: 1 reslate

### 2026-06-23 22:27 - Heat - d

batch: 1 reslate

### 2026-06-23 22:18 - Heat - d

batch: 1 reslate

### 2026-06-23 22:17 - Heat - r

moved ₢BfAAO before ₢BfAAT

### 2026-06-23 22:13 - Heat - d

batch: 1 reslate

### 2026-06-23 22:07 - Heat - S

foedus-reuse-leg

### 2026-06-23 22:06 - Heat - T

foedus-reuse-design

### 2026-06-23 22:06 - Heat - d

batch: 1 reslate

### 2026-06-23 21:53 - Heat - S

freehold-wrapper-combinator-lift

### 2026-06-23 21:52 - Heat - S

theurge-cosmology-subdoc-finalize

### 2026-06-23 21:52 - Heat - r

moved ₢BfAAZ after ₢BfAAT

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAV after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAW after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAX after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAY after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAZ after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAa after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAb after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAc after ₢BfAAU

### 2026-06-23 21:45 - Heat - r

moved ₢BfAAd after ₢BfAAU

### 2026-06-23 21:35 - Heat - d

batch: 1 reslate, 10 slate

### 2026-06-23 21:34 - Heat - S

foedus-fixture-seam-and-pool-check

### 2026-06-23 21:23 - Heat - d

paddock curried: teach paddock the heat-memories drain practice (experimental): dispositioned-only, wrap-considered + terminal-groom-judged

### 2026-06-23 20:29 - Heat - S

accessor-vocabulary-pass

### 2026-06-23 20:28 - Heat - S

suite-asterism-rename

### 2026-06-23 19:38 - Heat - d

paddock curried: store theurge-audit dispositions: suite asterism elected (reveille/picket/bivouac/echelon), chaining-fact-band restored, switch name + foedus-reuse parked

### 2026-06-23 11:33 - Heat - S

theurge-crucible-module-split

### 2026-06-23 10:49 - Heat - S

estate-secret-verification

### 2026-06-23 10:48 - Heat - d

paddock curried: Decision 2 settled: stable Keycloak key homed as an estate-wide field in the never-committed rbrs.env (RBRS_ESTATE_KEYCLOAK_SECRET), validated by match against the committed JWKS; nucleates the Estate concept (estate-wide facts in the station regime + must-match validator, graduate to a dedicated estate regime at 2-3 facts). Both audit decisions now settled.

### 2026-06-23 10:37 - Heat - d

paddock curried: cinch foedus (adopt in-heat; affiance/jilt coexist, betrothal-impurity accepted); Fable scrub: broaden in-heat mint to ALL vocabulary incl. remints, reframe premise-touching ideas as operator-owned (no Fable adjudicator), neutralize ~15 Fable-gate phrases; record stable-key inclination (global-regime home) with topology+naming open in the lifecycle pass

### 2026-06-23 10:14 - Heat - d

batch: 1 reslate

### 2026-06-23 10:14 - Heat - d

paddock curried: audit integral pass: record Decision-1 single-active-foedus topology + the foedus-lifecycle two-tier strand (new section); fix brevet->jilt in the orchestrator membrane; de-stale the BZ-gate + future-build-order header; record Decisions 2 (signing key) and 3 (foedus mint) as open

### 2026-06-23 09:38 - Heat - d

paddock curried: restore: header-only getter-vs-setter slip blanked the body; recovered from e5f490bda^

### 2026-06-23 09:37 - Heat - d

paddock curried

### 2026-06-23 09:35 - Heat - d

paddock curried: restore: prior header-only set had blanked the body

### 2026-06-23 09:34 - Heat - d

paddock curried

### 2026-06-23 09:31 - Heat - d

paddock curried: restore paddock content erroneously emptied by an audit-lens officium's stray setter call

### 2026-06-23 09:30 - Heat - d

paddock curried

### 2026-06-23 09:11 - Heat - T

fable-terminal-review

### 2026-06-23 09:10 - ₢BfAAI - W

Orientation complete. Walked the proven Keycloak->GCP->don chain and its two conscious deferrals (password-grant->RFC 7523; gcloud POC->durable REST) to shared understanding. Settled the Keycloak-control home as baked vessel DATA (realm-import) via the de-lamination model: the proof script is a flattened cross-section of three layers, and productizing de-laminates realm-config DOWN to vessel data and the Google-half UP to BCG station modules, with affiance held vendor-agnostic behind a membrane (never learns 'Keycloak'; blind to realm contents). Decided to finish the heat WITHOUT Fable (in-heat mint, gates applied by hand). Named the test-facility module rbxk_keycloak.sh in a new rbx family with rbw-xkX colophons (one bondstone, plainly named; rby rejected as the yelp family). Recorded all into the paddock and slated the 6-pace build spine (AAO spec-first -> AAN rfc7523+realm -> AAM affiance-arm -> AAL rbxk-orchestrator -> AAK accessor -> AAJ entra-guide).

### 2026-06-23 09:04 - Heat - S

federation-spec-first-recast

### 2026-06-23 09:03 - Heat - S

rfc7523-grant-and-baked-realm

### 2026-06-23 09:03 - Heat - S

affiance-programmatic-mechanism-arm

### 2026-06-23 09:03 - Heat - S

rbxk-keycloak-orchestrator

### 2026-06-23 09:02 - Heat - S

programmatic-accessor-rba-arm

### 2026-06-23 09:02 - Heat - S

entra-federation-setup-guide

### 2026-06-23 08:55 - Heat - d

paddock curried: record Fable-decoupling (in-heat mint), the de-lamination model, the affiance/Keycloak membrane, the one-module-with-accessor-boundary shape, and the rbx/rbxk_keycloak/rbw-xkX naming; prune a stray coronet

### 2026-06-22 12:52 - Heat - S

federation-buildout-orientation

### 2026-06-22 12:51 - ₢BfAAH - W

Proved the ride-or-die chain GREEN end-to-end from a clean headless shell: a Keycloak-minted OIDC id_token (fdkyclk crucible — kludge-built conjure vessel + nameplate, boring tether sentry) federates into GCP via uploaded JWKS (new programmatic workforce provider) -> STS -> don director mantle -> authorized Artifact Registry call. Captured as idempotent fdkyclk-proof.sh + fdkyclk-teardown.sh (POC, gcloud); live state torn down, recreate = charge + run proof. Recorded the result, empirical Palisade findings (https-issuer required, JWKS x5c/x5t strip, web-sso-required-even-with-uploaded-JWKS, confirmed oidc.jwksJson field, ~70s IAM propagation), the JWKS-refresh coupling, and the three-layer architecture in the config-model memo; curried the paddock to PROVEN. Conscious deferrals carried to the establishment/accessor build-units: minted via Keycloak password grant not the docket's RFC 7523 (GCP is grant-agnostic so the chain is proven regardless), and proven via a gcloud POC not durable REST (operator cinch: durable transform is REST-only, no gcloud).

### 2026-06-22 12:49 - Heat - d

paddock curried: curry ₢BfAAH proof result: flip config-model conviction to PROVEN; cinch durable establishment+accessor REST-only no-gcloud; three-layer home; durable naming deferred to Fable

### 2026-06-22 12:46 - ₢BfAAH - n

Record the ride-or-die proof result in the config-model memo: chain harness-proven green end-to-end (caveat flipped from paper-confirmed to proven); the conscious deferrals (password-grant stepping-stone vs RFC 7523, gcloud POC vs durable REST/no-gcloud transform); the empirical Palisade findings (https-issuer, JWKS field-strip, web-sso-required-with-uploaded-JWKS, confirmed oidc.jwksJson field, ~70s IAM propagation); the JWKS-refresh coupling; the three-layer architecture; and the committed artifacts that recreate the chain.

### 2026-06-22 12:38 - ₢BfAAH - n

Durable idempotent POC teardown, inverse of fdkyclk-proof.sh: attaint the brevet IAM bindings (tolerant of already-gone), jilt the workforce pool (--quiet, cascades provider), quench the crucible via the existing tabtarget. Header maps the sequence onto the real federation verbs (jilt/attaint) so the BCG conversion folds it into the establishment module's un-establish path (REST, no gcloud). Stashed beside the proof script where the conversion finds both halves.

### 2026-06-22 12:11 - ₢BfAAH - n

Trial: Stage B brevet tokenCreator binding failed — gcloud service-accounts add-iam-policy-binding could not infer the project from a bare access token, so the grant never landed and the don failed all retries. Fix: pass --project=$DEPOT_PROJECT on the SA binding.

### 2026-06-22 12:08 - ₢BfAAH - n

Stage A proven (FEDERATED-TOKEN-OK); add Stage B to the driver: brevet payor-direct (grant the fdkyclk-test principal tokenCreator on the director mantle SA + serviceUsageConsumer on the depot), don (generateAccessToken with the federated token, retry loop for IAM propagation), and the authorized depot-API call (Artifact Registry repositories.list). Payor-direct brevet is a deliberate proof deviation from the governor admission verbs, which assume the single manor pool — multiplicity-aware verbs are a separate build unit.

### 2026-06-22 12:03 - ₢BfAAH - n

Trial: GCP accepted the https issuer but rejected Keycloak's JWKS with a misleading 'Only RSA, EC key types are supported'. Strip the JWK to standard RSA public-key members (kty/kid/use/alg/n/e), dropping Keycloak's x5c/x5t cert fields that GCP's strict parser chokes on.

### 2026-06-22 12:01 - ₢BfAAH - n

Capture the ride-or-die proof chain as a durable, idempotent driver (fdkyclk-proof.sh): payor-token mint via refresh-token exchange, Keycloak realm config (client + audience mapper + user + frontendUrl=https for GCP's https-issuer requirement), signing-JWKS fetch, GCP pool + programmatic-provider ensure (uploaded JWKS), id_token mint (password grant stepping-stone), and STS exchange. Baseline for notch-per-trial traceability; provisional pool/provider/client names live in the constants block pending the pre-wrap naming pass. Findings encoded: GCP requires an https issuer scheme and requires web-sso flags even alongside --jwk-json-path.

### 2026-06-22 11:31 - ₢BfAAH - n

Re-kludge fdkyclk bottle with the corrected minimal realm; drove RBRN_BOTTLE_HALLMARK=k260622113135-9a1800167 into the nameplate.

### 2026-06-22 11:28 - ₢BfAAH - n

Fix fdkyclk realm import: Keycloak strict-parses RealmRepresentation and rejected the JSON _comment field, aborting start-dev --import-realm. Strip to a minimal-bootable realm (realm + token lifespans); the client, RFC 7523 grant, and mappers will be built live via kcadm and re-baked from export at the end.

### 2026-06-22 11:21 - ₢BfAAH - n

Kludge the fdkyclk sentry: local conjure build of the boring tether sentry rbev-sentry-deb-tether (cached layers), drove RBRN_SENTRY_HALLMARK=k260622112125-f45aecc4a into the nameplate. Both vessels now kludged; ready to charge.

### 2026-06-22 11:18 - ₢BfAAH - n

Kludge the fdkyclk Keycloak bottle: local conjure build of rbev-bottle-fdkyclk (quay keycloak 26.6.3 pulled by digest, hygiene clean, realm copied in), drove RBRN_BOTTLE_HALLMARK=k260622110611-044f732f9 into the nameplate.

### 2026-06-22 11:03 - ₢BfAAH - n

Scaffold the fdkyclk Keycloak test-IdP crucible for the ride-or-die proof: new conjure bottle vessel rbev-bottle-fdkyclk (FROM ${RBF_IMAGE_1} -> quay keycloak 26.6.3, digest-pinned, direct upstream pass-through per the dominant vessel precedent; first-cut realm baked for import, to be tuned live in Stage A), new fdkyclk nameplate (boring tether sentry, fresh enclave subnet 10.242.5.0/24, host 8088 -> enclave 8080, egress disabled), and charge/quench tabtargets. Nameplate validates clean.

### 2026-06-22 10:08 - ₢BfAAC - W

Settled both federation config-forks via design conversation. Fork one (test-manor topology): multiplicity-is-the-goal — the test manor holds the Entra interactive foedus and a Keycloak programmatic foedus side by side, each its own pool; per-run-switch rejected, dedicated-org noted-unneeded. Fork two (programmatic JWKS-source): single uploaded field — the local Keycloak crucible is unreachable from Google so its public JWKS uploads regardless; Keycloak-minted token preferred over self-signed-caged, the minter choice below-the-regime; issuer-discovered not modeled (re-cut named). Keycloak elevated over self-signed-caged as the automated lean; release-vs-routine test layering recorded; uploaded-JWKS confirmed load-bearing (only stubs Google's key-discovery, covered at the release gate). Resolution recorded in the config-model memo, paddock curried to match, and the ride-or-die Keycloak->GCP->don proof slated as the next pace (BfAAH).

### 2026-06-22 10:07 - Heat - d

paddock curried: 260622 fork resolution: flip degenerate lean to Keycloak-preferred, elevate multiplicity to goal, mark both config-forks RESOLVED, correct build-order units

### 2026-06-22 10:03 - Heat - S

keycloak-programmatic-foedus-proof

### 2026-06-22 10:03 - ₢BfAAC - n

Record fork resolution: fork-one test-manor topology = multiplicity-is-the-goal (swappable foedera; per-run-switch rejected; dedicated-org noted-unneeded); fork-two JWKS-source = single uploaded field (Keycloak-minted preferred over self-signed-caged, minter is below-the-regime; issuer-discovered not modeled, re-cut named); Keycloak supersedes self-signed-caged as the automated lean; release-vs-routine test layering; uploaded-JWKS load-bearing analysis; groom finding that the path was raised 4x but never elevated nor harness-proven

### 2026-06-22 07:40 - Heat - S

fable-terminal-review

### 2026-06-22 07:39 - Heat - S

terrier-founding-home-finisher

### 2026-06-22 07:39 - Heat - S

rbs0-manor-org-canon-scrub

### 2026-06-22 07:38 - Heat - d

paddock curried: ₢BZAAN reconciliation: re-aim freehold split to as-built (depot-freehold built in BZ; only foedus-lifecycle a built fixture; undelete declined -> refuse-and-rotate, prerequisite struck; 3 genuine residuals named); record terrier founding-home RESOLVED (option B, post-payor-guide manor-setup finisher)

### 2026-06-22 14:01 - Heat - r

moved ₢BfAAD after ₢BfAAA

### 2026-06-22 04:51 - Heat - f

racing, silks=rbk-12-mvp-federation-evolution

### 2026-06-21 12:47 - Heat - f

silks=rbk-12-mvp-FABLE-federation-evolution

### 2026-06-21 08:49 - Heat - d

paddock curried: record release-suite credential-heal deferral (freehold-enrobe demolished in BZ; skirmish/dogfight/blockade need federation compear+don readiness) — don't-forget tracking for the BZ estate demolition

### 2026-06-21 08:39 - Heat - d

paddock curried: add vocabulary-remint dingleberry doctrine to charter (canonical-word-is-safe; tree-wide post-BZ eviction sweep) — settled while mounting BZ estate demolition

### 2026-06-20 09:19 - Heat - n

Clarify the Itch concept in JJK core doctrine: an itch is a human reminder only, never load-bearing or agent-actionable — an agent must not cut paces or implement against itch content as if it were authority; load-bearing guidance homes in a paddock (heat-shape) or a spec (durable facts), not an itch. Parallels the existing 'memos are provenance, never authority' posture. Prompted by an itch-misplacement this session: load-bearing design guidance was wrongly parked in jji_itch.md, since relocated to the ₣Bf paddock.

### 2026-06-20 09:15 - Heat - n

Remove the bf-authentic-federation-green-target itch — its design guidance (authentic fixture drives the real verbs; the interim test-rig tabtargets rbw-dt/rbw-dT collapse; the lazy-relocate trap; the suspension-vs-erasure coverage debt; the reserved suite word parley) moved to the ₣Bf paddock as load-bearing authority where pace-cutting reads it. Operator doctrine: itches are human reminders only, never agent-actionable guidance, so load-bearing design content does not belong here. The terrier-atomicity-nature-discussion itch (a genuine human reminder) stays.

### 2026-06-20 09:13 - Heat - d

paddock curried: Add 'federation testing collapses onto the real verbs' idea (₣BZ payor-proof-cut follow-through); homes the test-collapse design lean as paddock authority, not in the human-only itch

### 2026-06-19 11:26 - Heat - S

compear-tty-to-yawp

### 2026-06-19 08:40 - Heat - n

Record the ₣Bf static negative-authorization test-rig haul from the ₣BZ ₢BZAAY mount conversation: payor boundary as a credential-possession test (one identity suffices) vs inter-citizen/inter-mantle denial (needs distinct standing subjects); the never-churn doctrine carried from ₣BZ; the self-signed-caged vs Keycloak degenerate-IdP spectrum; and the container chicken-and-egg resolution (one-way rig→IdP dependency, kludge/upstream-pull dodges it, self-signed needs no container).

### 2026-06-18 16:07 - Heat - d

paddock curried: correct the undelete-fix home: ₣BZ's foedus-freehold pace owns affiance undelete-on-DELETED as an in-pace prerequisite; ₣Bf consumes it, does not re-home it (removes a double-home)

### 2026-06-18 16:00 - Heat - d

paddock curried: name the config-model memo as the source material for cutting the federation build-order paces (it now carries the detailed RBS0 subdoc plan)

### 2026-06-18 15:59 - Heat - n

Append the detailed RBS0 subdoc plan (spec-authoring reference) produced by the incorporation workflow: marker-scheme note (grouped-not-gated, flat enum-enroll, the rbnve_-parallel wire-token mint), and per-spec must-contain detail for RBSRF/RBSMA updates and the two new subdocs (caged establishment, programmatic accessor), the per-vendor-guide contract, and the MCM quoin/sub-letter implications for Fable. Durably homes the detail that previously lived only in chat + an ephemeral workflow output file, so the future spec-first pace inherits it

### 2026-06-18 15:44 - Heat - S

settle-federation-config-design-forks

### 2026-06-18 15:43 - Heat - d

paddock curried: incorporate the federation-config-model conviction: keystone settled-modeling-axis block, re-home degenerate/payor-guide/multiple-federations ideas, foedus+terrier+headless cross-thread clauses, future build-order spine, sources; Q2/Q4 forks carried to a front-of-heat design pace

### 2026-06-18 15:03 - Heat - n

Record the federation configuration-model conviction: GCP workforce-provider config is vendor-invariant (doc-confirmed), so the regime carries a vendor-agnostic trust core + a single acquisition-mechanism gate (interactive|programmatic), NOT vendor identity; vendors live in per-vendor setup guides + opaque values; caged degenerate federation is mechanism=programmatic with an uploaded public JWKS, no vendor. Scores the four candidate models, grounds the gate in the vessel-regime precedent, preserves rbrf's no-secrets-committed invariant, bounds to OIDC, and names the RBS0 subdocs the heat must provide

### 2026-06-18 14:33 - Heat - d

paddock curried: tighten degenerate-federation confidence line after 2026-06-18 live-doc re-verification

### 2026-06-18 14:32 - Heat - n

Re-verify degenerate-federation mechanism against live GCP/Keycloak docs (2026-06-18): record the --jwk-json-path config door, the aud<->client-id spike gotcha, the explicit headless blessing, and Keycloak's JWT Authorization Grant promotion preview->supported; downgrade the honesty caveat to harness-composition-only with supporting URLs

### 2026-06-18 13:08 - Heat - f

silks=rbk-14-mvp-FABLE-federation-evolution

### 2026-06-18 11:10 - Heat - d

paddock curried: add 'payor as health-assurer' idea — payor-as-federation-identity ruled out, payor-direct vs new steward-role fork, 260618

### 2026-06-18 10:53 - Heat - d

paddock curried: add 'terrier's permanent founding-home' idea — terrier founding-integration deferred from BZ, 260618

### 2026-06-17 16:36 - Heat - d

paddock curried: Add three federation-evolution ideas deferred from BZ during the live-proof walkthrough: rename the assize live-window noun (fair-faced remint — leading ass- register wart + memorability miss, ashlar/hearting fork open); a verb to clear the live assize (force fresh compearance, capability trivial + name coupled to the rename, disposition open); IdP-agnostic one-click compearance via verification_uri_complete (Entra empirically dark + otc-prefill dead, lights up for a conformant IdP).

### 2026-06-17 09:38 - Heat - T

depot-freehold-roster

### 2026-06-17 09:38 - Heat - d

paddock curried: add freehold idea (durable reused test installations); cite workforce-pool-constraints memo

### 2026-06-17 08:19 - Heat - S

depot-freehold-roster

### 2026-06-17 07:56 - Heat - d

paddock curried: add heed idea: fair-faced remint of the compear sign-in verb (proffer runner-up)

### 2026-06-17 07:27 - Heat - d

paddock curried: swap entente -> foedus as the configured-federation noun lean; record folio-not-colophon correction

### 2026-06-16 16:19 - Heat - d

paddock curried: bake entente as the configured-federation noun (working lean)

### 2026-06-16 15:20 - Heat - d

paddock curried: keystone configured-federation noun + healthy payor/governor model (authn-vs-authz, sanctioned set)

### 2026-06-16 15:12 - Heat - n

Record degenerate-federation test-personas finding: doc-confirmed programmatic STS token-exchange flow lets test personas acquire federated tokens without a human browser click; two degenerate shapes, can/cannot-prove boundary, GCP/Keycloak/RFC sources. Referenced from the Bf holding paddock's degenerate-test-federation idea.

### 2026-06-16 15:12 - Heat - d

paddock curried: link degenerate-federation test-personas memo into the degenerate-test-federation idea + Sources

### 2026-06-16 15:08 - Heat - d

paddock curried: first-draft idea-catalog paddock for rbk-14

### 2026-06-16 15:07 - Heat - D

restring 1 paces from ₣BZ

### 2026-06-16 15:07 - Heat - f

stabled

### 2026-06-16 15:06 - Heat - N

rbk-14-mvp-federation-evolution

