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
Programmatic accessor (token → STS) — depends on the discriminator and on a programmatic trust existing to acquire against; obtains a token from Keycloak's non-interactive RFC 7523 grant (or self-mints, fallback) and exchanges at STS.
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