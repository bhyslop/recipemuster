## Freeze — Opus stewarding a Fable-authored paddock (260615)

This paddock was authored at Fable density and calibration.
The working agents are now Opus-class; a first Opus read mistook load-bearing precision for over-treatment.
Until Fable is available to re-author, or the operator sanctions a change in conversation, this paddock is read-only.
Corrections and tightenings — including over-treatments Opus believes it spots — are carried in the paces and in the pace-design memo, never folded back here.
Live pace-design work and the Opus/Fable divergence record: `Memos/memo-20260615-BZ-pace-design.md`.
A further routing, for evolution rather than correction: an idea that would change this paddock's cinched logic, or reach past this heat's scope, is not a correction — it goes to the holding heat for federation ideas deferred from this one (₣Bf), never folded back here.
A mid-pace urge to re-architect is evolution — route it to ₣Bf, not into this frozen body.
Note (260618): the manor-prerequisite line below is a known divergence under review — RBS0 makes the manor the payor project (a building), not an org-level "deed"; see the pace-design memo's manor-identity section, which the terminal Fable review reconciles. Left in Fable's original wording per the freeze; not corrected in place.

## Boundary — single tier, mantle impersonation (cinched 260612)

This heat builds the **one** operator credential model:
workforce federation with **mantle-SA impersonation**.
The keyfile citizen tier is **canceled unbuilt** —
no mode enum, no second tier, no migration machinery
(decision record: `Memos/memo-20260612-office-federation-conversion.md`;
canon banner updated same day).
Prerequisite ladder accepted: every manor brings a domain (org anchor, a deed not a building),
a Cloud Identity org, and a conformant external OIDC IdP.
Today's keyfile machinery is **bridge legacy**:
untouched, riding under the accessor seam, suites stay green on it
until federation personas pass the same suites —
then the RBRA estate retires whole as its own movement.
Scaffolding with a demolition condition, never a tier.

## The premise — human present (cinched; earmarked for RBS0)

Unchanged in substance from the canon's D2, wording per the vocabulary elections
and the overhang ruling:
**a human compears at the kickoff of every run; no run begins outside a live assize; every token self-expires.**
No refresh token anywhere beyond the payor's own;
an orchestrating agent never holds a secret
(device-flow kickoff surfaces only user_code + verification URI —
spike-verified live: no offline_access scope, no refresh token in the response).
Belongs in RBS0 as a premise quoin voiced `axk_premise` at civic incorporation.

## The mantle architecture (cinched 260612; spike-proven, two amendments absorbed)

**Capability-sets instantiate as mantle SAs at levy; all resource IAM freezes there.**
The full three-leg chain is proven live end-to-end in pure curl
(`Memos/memo-20260612-federation-legs-spike-findings.md`).

- Levy creates the mantle SAs (governor / director / retriever) beside mason
  and grants **every** resource binding (project, GAR repo, mason actAs) to them, once,
  behind a settle gate — levy is long and payor-driven; the gate is cheap there.
- Admission to a mantle writes the tokenCreator binding —
  plus, on a citizen's **first** admission to a depot, one depot-scoped binding (spike F2):
  `roles/iam.serviceAccountTokenCreator` on the mantle SA naming the `principal://` subject,
  and `roles/serviceusage.serviceUsageConsumer` on the depot project
  (Leg 3 needs a quota project; further mantles in the same depot need only tokenCreator).
  Brevet is an idempotent ensure of both.
  Unseat removes the tokenCreator binding and leaves the depot-scoped binding in place —
  a citizen unseated of every mantle is *suspended*, not erased, and cheap to re-brevet;
  **attaint alone sweeps the depot-scoped binding** (cinched at the review walk).
- Runtime chain: IdP device flow (Leg 1) → STS exchange (Leg 2, nothing extra — spike F3) →
  `generateAccessToken` on the mantle SA (Leg 3, depot as quota project via `x-goog-user-project`) →
  short-lived mantle token.
  Leg 3 is the **don**.
  Everything downstream bearer-blind, unchanged — no quota-project ceremony downstream (spike-verified).
- **Zero service account keys exist anywhere**; the depot runs with
  `disableServiceAccountKeyCreation` fully enforced.
  The system's sole durable secret is the payor's RBRO refresh token.
- No per-user Google identity ever exists:
  the federated principal is the grantable identity, implicit in the IdP assertion.
  The IdP is the census (cinch carried forward).
- Flap payoff (the architecture's origin): post-levy IAM mutation collapses to
  two binding types at two scopes with one gate point (F2 widened the original one-binding claim);
  the SA-lifecycle flap family confines to levy;
  hot paths perform zero IAM mutation.
  Mantle tokens accrue no count cap and self-expire
  (the 10-key cap is irrelevant — no keys; lifetimes per the two-ceiling note in Token custody).
- A donned mantle's token is a plain SA token —
  the workforce federated-token per-product support matrix is dodged entirely
  (and AR + Cloud Build are GA with no known limitations regardless — spike V2; belt and braces).
- Blast radius: a token carries one mantle's authority at a time;
  the multi-hat union-credential concern dissolves.

## The civic structure (cinched, carried forward with the mantle recast)

**Payor founds, governors populate, terriers tell, IAM enforces.**

- Admission authority scoped to the polity it admits into; payor outside the citizenry,
  ceremonial after founding; founding gestures gain **affiance**
  (the IdP-trust betrothal: workforce pool + provider + attribute mapping, org-level —
  and seating the payor's org-level `workforcePoolAdmin` role first, spike F1)
  and **mantle establishment** at levy.
- All operator administration is governor-wielded and depot-scoped.
  A governor brevets another with the governor mantle — governors create governors.
  The founding exception: levy's last act is the payor breveting the first governor —
  the one admission gesture outside governor wielding.
  Removing the last governor is **legal** (cinched at the review walk):
  a governorless depot is visible and recoverable, never broken —
  running citizens keep working, only administration pauses;
  recovery is the payor's founding exception re-exercised.
  Cross-depot administration does not exist.
- Governor dissolves structurally: an operator wearing the governor mantle.
  Standing governor remains the default posture.
- One-identity-across-depots arrives free:
  the same `principal://` subject is grantable in every depot under the manor.

## Multi-IdP posture (cinched 260612)

One live provider per pool is the norm.
Subjects are **pool-scoped, not provider-scoped** —
every provider is a full root of trust for the whole subject namespace,
so the pool's security floor is its weakest provider.
Adding a provider is a payor ceremony with founding gravity — a second affiancing:
**affiance covers first or further provider alike** (cinched at the review walk, parallel to brevet);
the machinery difference beneath is hearting.
Dual-provider windows exist for IdP **migration** only (overlap, re-admit, retire) —
never as an availability hedge; live tokens already ride out IdP outages within the assize.

## The Terrier (carried forward; entries recast)

Per-polity terriers in the Manor-homed Terrier bucket, managed-folder grain,
payor-created at levy, governor-writeable own-terrier-only,
read population governors-and-above — all unchanged.
Entries — **muniments** — record (principal subject, mantle held);
key IDs vanish with the keys.
Runtime verbs (compear, don, the bearer-blind verbs) never touch terriers;
admission verbs write muniments first.
The reconciliation diff is read-only
(terrier intent vs the mantle SAs' tokenCreator bindings — resource bindings are frozen and need no reconciliation loop)
and is **spec-interior machinery, not an operator verb**:
the word *audit* is demoted (cinched at the review walk — trodden, and initial-collides with attaint);
a word is minted afresh only if an operator surface ever appears.
Reconciliation and rehearse are spaced two distinctions apart:
rehearse recounts what the record says; reconciliation checks the record against reality.
Attribution trail (spike V3): the always-on token-mint log names the mantle SA and caller IP, **not** the human;
the federate appears downstream via `serviceAccountDelegationInfo` —
so levy enables per-service Data Access audit logs as a ceremony step
(Google's term; AR needs ADMIN_READ + DATA_READ for full retriever-trail coverage).

## Verbs and orderings (recast 260612; words elected)

Brevet and unseat carry small idempotent bodies —
first-vs-further admission differs only in the depot-scoped F2 binding,
which brevet ensures alongside the tokenCreator
(spec grain keeps the four admission/removal cases as quoins);
attaint composes unseat-all plus the F2 sweep;
**recut retires** (the IdP owns rotation; there is nothing to rotate Google-side);
**credential delivery retires whole** (nothing is delivered; the assay machinery vacates with it).

| Verb | Body | Crash leaves |
|---|---|---|
| brevet | muniment write → ensure tokenCreator on the mantle SA + serviceUsageConsumer on the depot | visible deficit; re-run |
| unseat | muniment withdraw → remove tokenCreator binding (depot-scoped binding stays: suspension, not erasure) | visible surplus; report-only |
| attaint | unseat all mantles → sweep serviceUsageConsumer → deregister note (IdP-side removal is the IdP admin's) | partial teardown lands as surplus, never resurrection |
| rehearse | pure read of the terrier | — |

(The earlier four-E working set — enfranchise/enfeoff/escheat/expel — was superseded at the
vocabulary pace: it failed MCM's sibling-initials rule;
bodies and crash semantics carried over unchanged, then F2 widened brevet's ensure set.)

## Vocabulary elections (complete 260612)

Shape rulings (operator):
the operator surface carries two daily admission verbs plus one rare whole-person expulsion
(spec grain keeps all four cases as quoins);
*citizen* survives as the operator noun;
one sign-in word serves every human including the payor — the mechanism difference is hearting;
the standing-role noun is ashlar, low-traffic.

Elected and operator-confirmed:

- **mantle** — the standing-role noun (the governor, director, and retriever mantles);
  supersedes the *office* placeholder, which failed its cold probe
  (heard as place-of-work, not civic role).
- **compear** — the per-session human sign-in act
  (Scots law: to appear formally in answer to a summons; noun *compearance*).
  Not a tabtarget: compearance is an accessor step —
  any cloud tabtarget probes for a live assize and on a miss with a TTY
  runs Leg 1 inline (prints the clickable URL + user code, polls);
  headless miss fails loud (elected at the spike).
- **assize** — the live-window noun: a bounded sitting that is also a fixed regulated measure,
  so the word carries the cap in its own body.
  Concretely: the assize is the workforce-pool session window (15 min–12 h, spike V1).
- **don** — the impersonation act (Leg 3);
  one mantle worn at a time is the blast-radius cinch made audible.
- **brevet / unseat** — the daily admission/removal verbs,
  etymologically married to mantle (investiture: robing in the garments of office);
  cult-word reuse under the eviction-then-reuse ruling.
- **attaint** — the rare whole-person expulsion (attainder: the old law's civic death).
- **affiance** — the founding-of-IdP-trust verb (the pledged faith between manor and IdP;
  *fiancé* keeps it warm despite rarity — husbandry's prescription for a founding-rare ceremony;
  first or further provider alike).
- **rehearse** — the read-Terrier verb (medieval sense: to recount a record formally, in order).
- **muniment** — the terrier-entry noun (deeds preserved as proof of rights).

Classification ledger (word → exposure → traffic → verdict):

| Word | Slot | Exposure | Traffic | Verdict |
|---|---|---|---|---|
| mantle | standing-role noun | ashlar | low: founding + error text | story-probed; confirmed (office failed its probe) |
| compear | sign-in act | ashlar | daily: instruction/error text | sentence- and story-probed; confirmed |
| assize | live-window noun | ashlar | daily: error text, cap prose | sentence- and story-probed; confirmed |
| don | impersonation act | ashlar | low: narrative, verbose output | story-probed; confirmed |
| brevet | admission verb | ashlar | governor-occasional | story-probed; confirmed (reuse) |
| unseat | removal verb | ashlar | governor-occasional | story-probed; confirmed (reuse) |
| attaint | whole-person expulsion | ashlar | rare | story-probed; confirmed |
| affiance | IdP-trust founding verb | ashlar | founding-rare (first or further provider) | sentence-probed; confirmed |
| rehearse | read-Terrier verb | ashlar | governor-occasional | sentence-probed; confirmed; spaced against the reconciliation diff |
| muniment | terrier-entry noun | ashlar | low: rehearsal output | story-probed; confirmed |
| citizen | operator noun | ashlar | narrative docs | shape ruling: survives the canceled tier |
| census | IdP-as-population concept | quoin (spec/narrative) | — | spaced against terrier (who exists vs who holds what); never operator-triggered |

Sibling initials: polity demesne **B**revet / **U**nseat / **A**ttaint / **R**ehearse — distinct;
manor demesne **L**evy / **E**stablish / **A**ffiance — distinct;
compear consumes no colophon initial (accessor step, not a tabtarget).

Mantle homonym guard (transform-scoped):
until the cult colophon family (`rbw-aM`/`arI`/`adI`/`arD`/`adD`) is renamed,
the reused words carry two senses in the repo.
New senses live in heat artifacts only — never in code, colophons, or specs;
the verb movement's **first act** is renaming that family, so the senses never coexist in code;
any agent meeting these words in existing code reads the retiring cult sense.
This guard deletes itself when the rename lands.

Broadside: the README glossary is the project broadside;
registration of the elected words rides the narrative-docs movement;
agent-context mirrors (CLAUDE.md inserts) are derived, never canonical.

MCM Word Selection gained from this pace:
one word per concept (no alias quoins, near neighbors spaced),
word husbandry (commonness matches traffic),
sibling initials (verbs orbiting one critical noun are siblings from birth),
and the broadside-mirror line.

Candidate ceremony, deliberately unminted (one specimen):
the vocabulary load-test story — a day-after tale told in the candidate words,
testing asterism coherence beyond single-word cold probes;
mint on second use.

## Retriever differentiation (cinched 260612 — policy, not mechanism)

Same architecture, three knobs:
retriever-mantle token lifetime at the 12 h ceiling —
mechanism spike-confirmed (V1): 1 h default and max,
12 h only via `constraints/iam.allowServiceAccountCredentialLifetimeExtension`
listing the retriever-mantle SA
(activates the canon's deferred D5 capability-set-keyed lifetime — the trigger case);
machine consumers (CI, crucible charges, runtime pulls) ride their **own workload identities**
granted reader directly — the IdP is never in a machine pull chain;
the reserve-key posture (one key on retriever-mantle, scoped policy exception)
is documented fallback, **not built**.

## Token custody (cinched MVP shape; spike election absorbed)

MVP, elected at the spike: the per-assize scratch caches the **federated token only**
(plus expiry and subject) — 0600 in a 0700 dir,
tmpfs-preferred (`$XDG_RUNTIME_DIR` on Linux, `$TMPDIR` on macOS), written atomically,
never a regime, never `BURD_OUTPUT_DIR`, never `BURD_TEMP_DIR`, never outliving the assize.
Mantle tokens are never cached: each verb-run mints one from the federated token
and holds it in process memory only — one-mantle-per-token blast radius automatic.
The IdP token is never persisted at all (consumed by Leg 2 in-process).
Two ceilings compose (spike V1): the pool session caps the federated token — that window is the assize;
the mantle token is independently capped (1 h; 12 h by org-policy listing)
and can outlive the assize that minted it.
**Overhang embraced** (cinched at the review walk): tokens self-expire;
a long run re-mints mid-flight from the cached federated token while the assize lives,
and when the assize lapses the next mint fails loud — "assize lapsed, compear."
Duration-aware admission control (commands predicting their runtime against the assize remainder)
is a named deferral — trigger: assize-lapse mid-flight failures biting in practice.
First upgrade: OS keystores via container credential-helpers
(one election covers podman and Docker; podman's Linux default is already tmpfs).
Anti-exfiltration layer: VPC-SC ingress with IP access levels (workforce-compatible),
deferred with triggers (first multi-operator org customer or first audit requirement);
the STS egress shape under a perimeter is answered on paper (spike V4: ANY_IDENTITY egress to sts.googleapis.com).
Device-bound CBA is currently unavailable to federated principals — named revisit trigger.
Evidence: the two beta-repo memos of 2026-06-11 (custody/context-enforcement; impersonation preference);
the spike findings memo.

## Launcher surface (carried forward, one addition)

`rbw-p` polity and `rbw-m` manor families as elected;
`rbw-m` levy gains affiance and mantle establishment;
recut leaves the `rbw-p` surface;
the retirement map otherwise stands.

## Documentation strategy (cinched, carried forward)

Contract specs ride their pace contract-first with in-pace predecessor retirement;
successor acronyms mint at slate-time;
RBS0 civic quoins at the head of the verb movement;
narrative docs trail as their own movement;
post-build cold-probe verifies.

## Ordering — substrate first, spike before verbs (recast 260612)

1. **Accessor seam.** As before: one accessor resolves every credential,
   read-side grep gate, identity-keyed interface, zero behavior change, suites arbiter.
   Keyfile inside during the bridge; the federation branch lands at movement 4.
2. **Capability-sets as named code.** As before — transcription out of the legacy
   cult-verb bodies (the SA-creation machinery riding under the seam);
   these definitions become the mantle SAs' levy-time grant lists.
3. **Federation spike.** **Done** — chain proven live, all verification items answered,
   session-cache shape elected, two paddock amendments absorbed (F2 two-binding admission,
   V3 downstream attribution): `Memos/memo-20260612-federation-legs-spike-findings.md`.
   The movement-4 gate is open.
4. **Civic verbs + mantles + terriers.** The behavior-changing movement:
   its first act renames the cult colophon family (discharging the mantle homonym guard);
   RBS0 civic quoins at its head;
   affiance + mantle establishment + audit-log enablement into levy;
   the three legs in the accessor with the assize cache and headless fail-fast
   (compearance as an accessor step, never a tabtarget);
   admission verbs under `rbw-p`; terriers live.
5. **Manor colophon regroup.** As before, plus the new founding gestures.
6. **Narrative docs.** As before, plus broadside registration of the elected words.
7. **RBRA estate retirement.** After federation personas pass the suites:
   the cult estate, key machinery, RBSRA and the keyfile probe specs retire whole.
   The bridge's demolition condition, executed.
8. **Handbook rework: home open** (₣A6 vs a final movement here), unchanged.

Movements 1–2 are no-regret and proceed immediately; the spike gate is now open.

## MVP purpose (carried forward, recast day-after picture)

The realistic pioneer is the solo evaluator — now with domain + org + IdP tenant,
accepted as a one-time founding hour
(spike-observed Entra path: free Azure signup provisions a real tenant, ~15 min, $0).
Day-after picture:
payor establishes the manor and affiances it to the IdP,
levies the depot (mantles rise with it),
brevets the first governor — self — as levy's last act;
that governor brevets self with the director and retriever mantles;
every tabtarget works through the accessor on a morning compearance (one device-flow click);
a terrier rehearsal prints three muniments;
the cult verb estate and every key file no longer exist.

## Test rig (the one place a durable secret could creep back)

Synthetic personas need assizes once keys retire.
Options: per-run human click (a 12 h assize on the project's own test org)
vs a test-org-only secret (Keycloak password grant or caged token — R4's ghost).
Decide at movement-4 slating, deliberately.
Spike fixtures **kept** (cinched at the review walk):
the pool/provider/SA rig stands as the project's standing test trust,
with the recorded caveat that the pool is a real root of trust on the production org
and its security floor is the operator-owned Entra tenant;
the payor's org-level `workforcePoolAdmin` stays (affiance formally seats it later);
the depot's AR audit config stays as known-good reference for the levy ceremony.
Keycloak-in-a-crucible is paper-confirmed viable for the programmatic flow
(uploaded JWKS serve STS exchange exactly; console sign-in cannot ride them — spike V5).

## Blast radius (recast)

One operator may hold several mantles but a token carries exactly one mantle's authority;
cross-mantle union exists only as serial donning, never as one credential.
The hijacked-live-assize exposure is bounded by the assize and the lifetime policy.

## What done looks like

- No call site outside the accessor touches credential material (grep-verifiable);
  the accessor's federation branch is the only token mint.
- Mantles established at levy with frozen resource IAM;
  admission verbs operate on (actor, mantle, principal) within a depot;
  terriers function as described; a rights query is a rehearsal — pure read.
- Zero SA keys in the system; the RBRA estate deleted whole (movement 7).
- Launcher surface per the elected families, including affiance.
- Specs migrated contract-first; README and narrative docs tell the federation story
  in the elected vocabulary, registered on the broadside.
- Suites green at each movement, running on federation personas; `complete` before close.

## Cinched decisions

- Single tier: workforce federation + mantle impersonation; keyfile citizen tier canceled unbuilt; no mode enum (supersedes canon D1).
- Mantle SAs at levy instantiate capability-sets; all resource IAM frozen at levy behind a settle gate.
- Admission is tokenCreator on the mantle SA + (first admission per depot) serviceUsageConsumer on the depot project, idempotent-ensured (spike F2 amendment).
- Zero SA keys; sole durable secret is the payor's RBRO; human-present premise (D2) bounds the design — spike-verified live (no refresh token).
- Payor founds (now incl. affiance + mantle establishment + first-governor brevet + audit-log enablement), governors populate, terriers tell, IAM enforces.
- One live IdP provider per pool; provider addition is a payor ceremony; dual-provider for migration only; affiance covers first or further provider alike.
- Recut retires; credential delivery retires whole; the word assay vacates rbk.
- Retriever differs by policy only: 12 h lifetime via the V1 org-policy listing, machine pulls on workload identity, reserve-key posture documented not built.
- Token custody: federated-token-only assize scratch (tmpfs-preferred) behind the accessor at MVP; mantle tokens per-verb in-process; keystore upgrade and VPC-SC ingress as named deferrals.
- Review-walk rulings (260612): mantle-token overhang embraced (duration-aware admission deferred with trigger);
  *audit* demoted to spec-interior prose;
  last-governor removal legal with payor recovery;
  attaint alone sweeps the depot-scoped F2 binding (unseated-of-all is suspension);
  spike fixtures kept as standing test trust.
- Terrier, documentation strategy, launcher families, reconciliation-as-mirror: carried forward as previously cinched.
- Intent-first verb orderings (muniment write precedes binding) carried forward.
- Capability-set definitions global (code, realized as mantle grant lists); memberships local (bindings + muniments).
- The civic word constellation elected and operator-confirmed — see Vocabulary elections.

## Retired cinches

- The citizen keyfile tier and everything keyfile-specific:
  per-operator SA minting, key-last admission, recut, verb-internal key delivery,
  RBRA full-auto authorship, the mode enum, the two-tier test matrix.
  (The 260610–11 delivery/assay cinches retire with it — their three jobs have no successor because nothing is delivered.)
- "Identity scope is a tier property / keyfile citizens depot-minted" —
  collapsed: identity is always IdP-scoped; depot-minted identity died with the tier.
- The blast-radius union-key acceptance — dissolved by one-mantle-per-token.
- The four-E civic verb working set — superseded by the elected words (sibling-initials rule).
- "Admission is one binding" — widened to two by spike F2; the flap payoff survives (one gate point, hot paths still zero-IAM).
- Earlier retired cinches stand as recorded.

## Open — resolve within the heat

- Test-rig synthetic-persona credential (see Test rig).
- Terrier file format, physical bucket name, `rbgb_` allocation.
- Handbook rework home.
- `rbw-gq` disposition at the regroup.
- Beta-repo evidence memos: migrate to this repo or leave pointed-at.

## Sources

Decision record: memo-20260612-office-federation-conversion (this heat's bedrock).
Spike evidence: memo-20260612-federation-legs-spike-findings —
the chain proven live, the five verification items, the session-cache election, the F1/F2/V3 amendments absorbed above.
Federation canon memo-20260609 (banner updated; login legs authoritative, D1 retired).
Evidence (beta repo, 2026-06-11): google-impersonation-preference; token-custody-context-enforcement.
Flap derivation: memo-20260604-credential-churn-leak-and-propagation-races;
memo-20260611-heat-BH-class-c-setiampolicy-write-flap; README Eventual Consistency appendix.
Lineage: memo-20260522 (R1 is the mantle shape's ancestor; R2 superseded by this conversion);
memo-20260605-citizen-capability-model (canceled tier's mechanism, historical).
Vocabulary: elected in conversation at the vocabulary pace under MCM Word Selection,
which gained four rules back from the same conversation.
Prior paddock revisions through 260610 stand as recorded;
this 260612 revision is the office-federation conversion, the vocabulary elections,
the spike-findings absorption, and the review-walk rulings.
Operator commitment: this paddock and its paces are slated by Fable-class agents; density is calibrated accordingly.