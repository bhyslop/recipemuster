# Heat Trophy: rbk-42-fbl-federation-create

**Firemark:** ₣BZ
**Created:** 260604
**Retired:** 260705
**Status:** retired

## Paddock

## Freeze LIFTED — operator-sanctioned reconciliation (260622)

The freeze is RESOLVED. The terminal Fable review (the heat's last pace) ran under OPERATOR SANCTION in Fable's absence (the freeze always lifted to "Fable's re-authoring OR operator sanction in conversation").
The review affirmed this heat's build sound — 17 recorded Opus/Fable divergences, ZERO overturns — and applied the record corrections below in place.
Full reconciliation lives in the review ledger and `Memos/memo-20260622-fable-review-queue.md` (the standing Fable terminal-review queue; every mint made meanwhile is provisional and eviction-sweepable).
Manor-identity divergence RESOLVED: the Manor is a COMPOSITE HOLDING — a payor/manor GCP project plus clustered resources plus a commanded organization — not an org-level "deed"; that sense was confabulated (paddock-only, zero spec/code) and is overturned. The federation-quoin scrub of canonical RBS0 is tracked as its own pace.
Historical freeze regime (260615, retained for the record): this paddock was authored at Fable density; a first Opus read mistook load-bearing precision for over-treatment; corrections rode the paces and `Memos/memo-20260615-BZ-pace-design.md` rather than being folded back. Evolution still routes to ₣Bf, never into this body.

## Boundary — single tier, mantle impersonation (cinched 260612)

This heat builds the **one** operator credential model:
workforce federation with **mantle-SA impersonation**.
The keyfile citizen tier is **canceled unbuilt** —
no mode enum, no second tier, no migration machinery
(decision record: `Memos/memo-20260612-office-federation-conversion.md`;
canon banner updated same day).
Prerequisite ladder accepted: every manor (its one payor/control-plane project) sits under a Cloud Identity organization — the organization brings the domain and is the federation anchor the manor commands; the manor is the project it governs.
Required: that Cloud Identity org, and a conformant external OIDC IdP.
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
payor-created by an interim scaffold (NOT at levy — the permanent founding-home is ₣Bf's, via a post-payor-guide manor-setup finisher; the rbw-dt/dT scaffold retires when it lands), governor-writeable own-terrier-only,
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

## Paces

### known-good-floor (₢BZAAR) [complete]

**[260615-2014] complete**

Run the credential-surface suites at heat HEAD and record their dispositions as the heat's known-good floor, before any movement-4 code lands.
Only `fast` is green at current HEAD; `service` and `complete` last ran before the regime-poison conversion and the sentry citation-collapses landed, so no verified floor exists on the very surface this heat rewires.
Run `complete` then `dogfight` sequentially (never parallel — fixtures share regime and container state); `blockade` is optional at operator call.
Depot is levied — the cloud-probe precondition — confirmed at slate.

## Done when
- `complete` and `dogfight` have run green at the heat-HEAD commit; the wrap summary records each suite's verdict and the commit SHA as the durable floor marker.
- Any red result is dispositioned — fix-first or accept-and-note — before the movement-1 credential-accessor refactor mounts, never silently carried.

## Cinched
- This pace makes no code change; its sole deliverable is the recorded disposition in the wrap summary (the heat's journal, since the paddock is frozen).
- A red baseline blocks the movement-1 mount until resolved.

## Character
Verification gate, not a change — run the suites, read the logs, record the floor. Mechanical; the only judgment is dispositioning a red.

### federation-legs-spike (₢BZAAA) [complete]

**[260612-1003] complete**

Prove the office-federation runtime chain end-to-end in pure curl/openssl/jq before the verb movement is slated,
and answer the banked verification items.

Stand up on the existing org: a workforce pool, an OIDC provider against a test IdP
(free Entra tenant; also assess Keycloak-in-a-crucible viability via uploaded JWKS for the hermetic case),
and walk: device flow (Leg 1) → STS exchange (Leg 2) → `generateAccessToken` on a test office SA (Leg 3) →
bearer call against a depot API.
No gcloud at runtime; setup ceremony may use console/gcloud freely.
Elect the session-cache wiring shape (token shared across tabtarget processes,
per-session scratch, ≤ session cap, fail-loud when absent and no human can click).

Sources: `Memos/memo-20260612-office-federation-conversion.md` (decision record),
`Memos/memo-20260609-federation-canon.md` (Leg 1/2 mechanics),
beta-repo evidence memos of 2026-06-11 (custody, impersonation preference).

## Done when

- A federated principal mints an office-SA token from a clean shell and successfully calls a depot API with it.
- Session-cache shape elected and recorded (memo or paddock note).
- Verification items answered with evidence: audit-log delegationInfo shape under impersonation;
  office-token lifetime ceiling and the lifetime-extension org-policy constraint;
  Artifact Registry and Cloud Build status in the workforce federated-token support matrix;
  STS egress-rule shape under a VPC-SC perimeter (paper answer acceptable);
  Keycloak-uploaded-JWKS programmatic flow viability (paper answer acceptable if Entra leg consumed the budget).
- Findings land as a memo; surprises that change the paddock's office-architecture section are flagged, not silently absorbed.

## Cinched

- This is evidence-gathering, not production code: throwaway scripts acceptable, BCG discipline not required.
- The pool/provider created here may serve as the project's standing test trust if it proves clean.

## Character

Exploratory spike against foreign APIs; expect Palisade surprises; characterize precisely rather than tolerate silently.

### ashlar-vocabulary-mint (₢BZAAB) [complete]

**[260612-1046] complete**

Elect the operator-facing (ashlar) vocabulary for the office-federation model,
in design conversation with the operator —
words are elected here, applied later.

Slots to fill:
the generic standing-role noun (current placeholder *office* — first decide whether it is ashlar at all or spec-only);
the impersonation act;
the per-session human sign-in act and the live-window noun (highest traffic — daily words on trodden ground);
the founding-of-IdP-trust verb (joins the Establish/Levy founding family);
the user-facing admission verb surface;
the terrier-entry noun and read-Terrier verb (carried opens).

Discipline: MCM Word Selection including the Exposure ladder
(`mcm_ashlar`, `mcm_fair_faced`, `mcm_coffer`, `mcm_broadside`, `mcm_cold_probe`);
the project broadside is the README glossary (registration itself may ride the narrative-docs movement);
retired cult words (mantle, invest, divest, roster) are reusable per the operator's eviction-then-reuse ruling —
eviction sweep precedes any re-mint, the grep gate proves it;
the credential asterism is manorial-civic, audible against foundry (ecclesiastical) and crucible (alchemical).

Resolve with the operator before electing words:
how many admission verbs operators face (spec grain may stay finer);
whether *citizen* survives as the operator noun;
whether one sign-in word serves all humans including the payor;
each candidate cold-probed, verdicts recorded.

Sources: ₣BZ paddock (office architecture, verb table, vocabulary Open item);
`Memos/memo-20260612-office-federation-conversion.md`;
MCM Word Selection.

## Done when

- Elections recorded in the paddock; the *office* placeholder prose replaced by the elected words.
- A classification ledger (word → exposure tier → traffic class) lands in the paddock or a memo.
- Cold-probe verdicts recorded for every ashlar election.
- No code or spec-file changes — application rides the verb movement's paces.

## Character

Design conversation requiring operator judgment throughout; the agent proposes, probes, and records — the operator rules.

### pace-design-membrane (₢BZAAC) [complete]

**[260615-1806] complete**

Deeply review and firm the seven movement-4 paces — first-cut at this pace's slate-time at recommended grain, dockets deliberately thin — in a clean chat, using a high-effort multi-agent workflow, then hand them to execution.
This pace was the Opus/Fable membrane that cut the first draft; it now owns the deep shake-out, deliberately moved off the heavy planning conversation that produced it.
The current succession is the heat's coronet list following the movement-1/2 paces; read it fresh rather than trusting any enumeration here.

## Done when
- The movement-4 paces' boundaries, ordering, and splits are reviewed and firmed: does levy-founding split (affiance vs mantle establishment)? do the admission verbs split per-verb? is the compearance/don boundary right?
- The first-cut assumptions are confirmed or overturned: full-depth rename (vs operator-facing only), and the spike rig as the sole test substrate (vs a federation-test-fixture standup pace).
- The contract-first successor spec-acronym mints are sequenced: civic-verb specs succeeding the renamed cult specs, a levy/affiance spec, the Terrier noun quoin.
- Each movement-4 docket is firmed from first-cut to mount-ready.

## Cinched
- The paddock stays frozen; corrections route into the paces and the BZ pace-design memo — the membrane role persists.
- The review runs in a clean chat with a high-effort workflow, not by extending the planning conversation that produced the first cut.

## Character
High-effort adversarial review of a first-cut plan; workflow-driven.
The membrane role persists — divergences land in the memo, not the paddock.

### accessor-seam (₢BZAAD) [complete]

**[260615-2141] complete**

Collapse every credential resolution behind one identity-keyed accessor — paddock Ordering movement 1.
Today ~50 call sites mint tokens directly via `rbgo_get_token_capture "${RBDC_<ROLE>_RBRA_FILE}"`, plus the governor wrapper `rba_get_governor_token_capture`.
After this pace, one accessor keyed by identity (governor / director / retriever) is the only place credential material is touched.

Behavior is unchanged: the accessor wraps today's keyfile mint during the bridge; the federation branch lands at movement 4.
Home: `rba_auth.sh`. The source-side grep gate is the verification; the test suites are the arbiter.

Sources: ₣BZ paddock "Ordering" movement 1 and "What done looks like" (first bullet); code-grounding in `Memos/memo-20260615-BZ-pace-design.md`.

## Done when
- A single accessor resolves credentials by identity; no call site outside it touches credential material (grep-verifiable).
- Every former direct-mint site routes through the accessor; behavior unchanged.
- `service` suite green (credential paths) — the arbiter of the no-behavior-change claim.

## Cinched
- Identity-keyed interface, not RBRA-path-keyed; the RBRA-file plumbing moves inside the accessor.
- The keyfile mint stays inside the accessor during the bridge — no federation here; that branch is movement 4.

## Character
Mechanical, behavior-preserving refactor across ~50 sites; the grep gate and suites make correctness checkable, not a matter of judgment.

### capability-sets-as-code (₢BZAAE) [complete]

**[260616-0126] complete**

Lift the capability-sets — the per-role resource-grant lists — out of the legacy cult-verb bodies into named code — paddock Ordering movement 2.
Today the grant lists live inline inside `rbgg_invest_director` / `rbgg_invest_retriever` and the governor mantle, expressed through `rbgi_add_{project,repo,sa,bucket}_iam_role`.
After this pace, each role's grant list is a named definition; these definitions become the mantle SAs' levy-time grant lists at movement 4.

No behavior change: this is transcription into named form, not a re-grant.
The key-minting body `zrbgg_create_service_account_with_key` is untouched here — it retires at movement 7.

Sources: ₣BZ paddock "Ordering" movement 2 (capability-set definitions become the mantle SAs' levy-time grant lists) and the "Cinched" line on definitions-global / memberships-local; code-grounding in `Memos/memo-20260615-BZ-pace-design.md`.

## Done when
- Each role's resource-grant list (governor / director / retriever) exists as a named definition in code.
- The legacy invest / mantle bodies consume the named definitions rather than inlining the grants — behavior unchanged.
- `service` suite green (the IAM-grant paths).

## Cinched
- Definitions are global (code); memberships and bindings stay local — the capability-set definition is distinct from its realization (paddock Cinched).
- Transcription only; no grant added or removed; the key machinery is untouched (it retires at movement 7).

## Character
Near-mechanical extraction; the only judgment is cluster boundaries — which grants belong to which set — already implied by the existing per-role invest bodies.

### cult-rename-enrobe-defrock (₢BZAAF) [complete]

**[260616-0954] complete**

Rename the legacy keyfile-SA cult family off the words federation claims — invest / divest / mantle — to the garment-of-office mint enrobe / defrock.
The verb movement's first act; discharges the homonym guard so the elected words enter code in one sense only.
enrobe = create a key-backed SA (unifies the cult create-verbs across governor and director/retriever); defrock = decommission one; roster stays (carries no homonym).
Mint rationale and register: the BZ pace-design memo.
Blast radius is discovery, not enumeration: grep the cult words across Tools/rbk in one sweep — the rbw-a colophon family and rbz_zipper enroll lines, the governor-auth and payor-auth mantle functions, the cult specs with their RBS0 op-quoins and include:: sites, the acronym map, and the theurge Rust canonical-establish fixtures that drive the cult colophons by cult-word function name.
The generated consts re-derive via the build, not by hand.

## Done when
invest/divest/mantle no longer carry the cult sense anywhere in Tools/rbk — bash, colophons, specs, acronym map, and theurge fixtures (grep-verifiable); roster unchanged.
Generated consts re-derive via build; fast suite plus regime-poison green.

## Cinched
Full depth: vacate the cult words from functions, specs, op-quoins, acronym map, and Rust fixtures — not operator-facing colophons only, since the homonym otherwise stays live in the layers the federation verbs reference.
enrobe/defrock are transitional ashlar — specs and acronym map only, never the README broadside (doomed at the estate-retirement movement).

## Character
Wide but mechanical grep-gated rename crossing four surfaces — bash, specs, acronym map, theurge Rust — and two auth paths in two files.

### rbs0-civic-quoins (₢BZAAG) [complete]

**[260616-1019] complete**

Seat the elected federation civic vocabulary as RBS0 quoins at the head of the verb movement — the contract-first foundation the levy, accessor, terrier, and verb paces all reference.
Declare one new federation category prefix in the RBS0 mapping-section header — RBS0 categorizes by flat single-prefix categories, not a sub-lettered scheme — then seat the elected civic nouns and acts as flat quoins under it.
The human-present premise rides the existing premise category as a further quoin voiced axk_premise; clone the form from a sibling premise.
Also seat the Terrier noun quoin as a shell — its internals are parked to the terrier-live pace — and wire in the already-minted Terrier sub-operation subdoc: register its sub-op quoins in the RBS0 mapping section and add its include:: proximal to the registration.
The elected word set and the noun/sub-op split are authoritative in the frozen paddock; do not re-derive them here.

## Done when
The elected civic words are RBS0 quoins with definition sites, under one new federation category prefix declared in the mapping header.
The human-present premise is a premise-category quoin voiced axk_premise.
The Terrier noun quoin exists as a shell; the Terrier sub-op quoins are registered in the mapping section and the subdoc is include::'d into RBS0.

## Cinched
Concepts are RBS0 quoins; the Terrier access sub-ops stay homed in their subdoc (ACG: reference the home, don't recreate).
Terrier noun internals — object granularity, muniment JSON shape, bucket name, managed-folder IAM grain — are NOT decided here; they are parked to the terrier-live pace.

## Character
Flat-category MCM quoin authoring in RBS0 plus subdoc wiring — additive spec work, no code, no runtime behavior.

### manor-affiance (₢BZAAM) [complete]

**[260616-1302] complete**

Affiance the manor to its IdP — the org-scoped IdP-trust founding: workforce pool + provider + attribute mapping.
Seat the payor's org-level workforcePoolAdmin role BEFORE creating the pool or provider, or the ceremony 403s — see the federation-legs-spike-findings memo (F1).
Wholly net-new behavior: no affiance machinery exists in the kit today; greenfield REST against the workforce-pool API, org-scoped.
A successor affiance contract spec carries it; acronym minted when firmed.

## Done when
A payor affiances the manor — the pool, provider, and attribute mapping exist and the payor's org-level workforcePoolAdmin self-grant has landed.
Leg-1 device flow reaches the provider against the standing spike fixture trust.
The successor affiance contract spec carries it, seated under the manor demesne.

## Cinched
Affiance covers first or further provider alike; a second affiancing is a payor ceremony with founding gravity.
workforcePoolAdmin is seated before pool or provider creation.

## Character
Net-new federation founding, org-scoped, no existing code — wholly greenfield REST against the workforce-pool API.

### manor-jilt (₢BZAAX) [complete]

**[260616-1700] complete**

Build the manor jilt verb — the inverse of affiance: dissolve a workforce pool (break the manor↔IdP betrothal), payor-credentialed.
Colophon rbw-mJ, frontispiece PayorJiltsManor.
Contract-first per the documentation-strategy cinch: author the RBSMJ contract spec, then the impl.

The create-side mirror is rbgp_manor_affiance (RBSMA): jilt is its structural inverse, a payor DELETE on the workforce pool.
The delete-pattern precedent is depot unmake (rbw-dU, PayorUnmakesDepot): payor token, REST DELETE, LRO poll to terminal state, soft-delete graveyard.
Delete the pool only — the provider is namespaced under it and cascades; undelete is the recovery path (per the RBSMA soft-delete NOTE).
Seat jilt as a federation civic quoin in RBS0 alongside affiance; broadside registration rides the M6 narrative-docs movement.

## Done when
RBSMJ contract spec authored (M-seat, contract-first).
rbw-mJ colophon enrolled in the zipper — consts and tabtarget-context regenerate from the build.
The verb dissolves a named workforce pool under payor credentials and exits clean on an already-absent pool (404 → idempotent no-op).
Grep-clean: jilt carries one meaning repo-wide.

## Cinched
Operator-sanctioned in conversation (260616) as an in-heat manor verb beyond the paddock's frozen founding triad — recorded in the pace-design memo with the scope note for Fable.
jilt is a permanent operator verb (ashlar, broadside-eligible), not a test-only teardown gesture.
Pool-only delete; provider cascades; soft-delete debris accepted.

## Character
Contract-first manor verb — affiance's create mirrored as its delete.
Intricate-but-mechanical: the REST/LRO shape follows the depot-unmake precedent.

### foedus-lifecycle (₢BZAAW) [complete]

**[260617-1011] complete**

Nucleate the foedus-lifecycle theurge fixture: the repeatable autonomous proof of the affiance→jilt create/destroy round-trip on an ephemeral workforce pool.
This supersedes the one-off manual proof — affiance and jilt are already proven live by hand, and the create-shape bug found in doing so (the org parent was sent as a query parameter; it must be a body field) is fixed and notched.

The fixture mirrors the reliquary-lifecycle precedent structurally — a single self-contained round-trip case, no charge/quench.
First it probes the payor credential and fails loud (Fail, not Skip) when it is not green.
Then it mints a unique throwaway pool id, affiances under the regime-poison tweak overriding RBRF_WORKFORCE_POOL_ID, asserts the create banners, jilts under the same override plus confirm-skip, asserts the dissolved/DELETED terminal, and re-jilts to assert the idempotent no-op.

## The payor-credential gate is Fail, not Skip (operator ruling, this conversation)
The reliquary/regime-poison self-skip pattern is passenger-protection: it keeps a broadly-run suite from going red on a machine that legitimately lacks an operator-local prerequisite.
This fixture is never a passenger — it is quota-touching, so operator-invoked only, in no auto-suite.
The only way it runs is an operator typing the FixtureRun tabtarget, which is an explicit "prove the round-trip now."
A missing or expired payor credential at that moment means the one thing it was invoked to do cannot happen — that is a failure of the run, reported loud, never a benign skip.
The same fail-loud rule covers every precondition (the RBRF federation regime included): if you invoked it, you meant it.

## Placement
The fixture creates a real workforce pool every run, and soft-deleted pools count against the 100-per-org cap for ~30 days (workforce-pool-constraints memo) — so it is quota-touching and runs operator-invoked, never in the routinely-run service/complete suites.
Registered in the fixture registry for discovery (runnable via FixtureRun), a member of no suite.

## Done when
foedus-lifecycle is registered and discoverable, theurge compiles, and one live operator-invoked run passes end to end: create → DELETED → re-jilt no-op.
The fixture fails loud (Fail, not Skip) when the payor credential is absent.

## Freehold counterpart (separate pace)
The durable day-to-day counterpart is the foedus-freehold (establish-if-absent + verify) — its own pace, slated after the terrier and admission-verb work.
Correction to an earlier framing in this docket: the foedus freehold DOES carry a standing-citizen roster (admitted citizens in the manor-homed terrier, read via rehearse); the IdP owns only who-can-authenticate, not who's-admitted.
That roster is the join binding a foedus principal to a depot mantle, so the foedus and depot freeholds are intertwined.
See that pace and the pace-design memo's freehold doctrine for the full shape.

## Cinched
Quota-touching by nature (a genuine create cannot reuse a soft-deleted id) → operator-invoked, never a routine auto-suite member.
The create-shape fix and the manual live proof are already landed; this pace codifies them as the repeatable artifact.
Missing precondition = Fail, never Skip (operator ruling): Skip is suite-passenger protection, and this fixture is never a passenger.

## Character
Codify a proven manual round-trip as a repeatable, quota-aware, fail-loud fixture; mirror reliquary-lifecycle's single-case round-trip shape.

### levy-founding-proof (₢BZAAU) [complete]

**[260617-1531] complete**

Add the eyes to the depot levy's federation founding, and make the freehold reusable — delivered and proven live.
Three pieces shipped:
the recognosce verb (RBSDC contract + rbgp_depot_recognosce + rbw-dr + the canonical-establish recognosce case) — a read-only survey confirming the three mantle SAs, every capability-set binding across the project / GAR-repo / Mason / director policies, and the AR Data-Access audit config, exit 0 only when whole and fatal-naming otherwise;
the idempotent freehold-ensure (canonical-establish reuses an ACTIVE freehold, creating no depot on a routine run — the quota fix — and pick-next-creates only when absent or graveyarded);
the canonical-churn fixture (fixture-driven rotate-moniker + unmake of the canonical freehold, so the churn → create → recognosce cycle runs as fixtures, no hand-edited regime surgery).

## Done when
MET. On a fresh conformant depot (churn the stale pre-mantle freehold, then create-path levy), recognosce passed all seven live checks: "Depot founding recognosced whole." Reuse-detect, churn, and the create-path levy-with-founding all proven live and fixture-driven.

## Cinched
Independent live-GCP re-query, judged by exit code and resource facts, never banner prose.
Reuse-not-recreate is the quota fix; churn is deliberate (member of no suite); DELETE_REQUESTED is treated as gone, so a fresh create needs no 30-day wait.
recognosce mints no keys, so it survives the org no-keys policy the live run surfaced.

## Deferred — worked through here, owned by later BZ paces
The bridge-legacy collision: the org disableServiceAccountKeyCreation policy fails the legacy keyfile governor-enrobe (HTTP 400) on a fresh levy, breaking canonical-establish at case 2 before recognosce can run — routed to a heat pace by the groom.
Enrobe-idempotency, the canonical/throwaway prefix collapse, the rbtdrk_depot_levy fn rename (it ensures now, not just levies), and the rbtdrk_canonical.rs >800-line split.

### federation-live-proof (₢BZAAT) [complete]

**[260617-1637] complete**

Walk the operator through the live, human-in-the-loop proof of federation compearance — the one thing the build paces cannot verify without a terminal and a browser click.
Everything machine-verifiable (BCG, shellcheck, the credless and headless gates, the jq-emit expression checks) is already green in the build paces; this pace is only the live-trust + human-click portion they defer.
Agent drives it as a trot: operator at a terminal, Leg 1 device flow (verification URL + user code, then browser approval), Leg 2 STS exchange, federated token cached per-assize; then confirm the headless cases consume that one compeared assize at suite head.
Run against the trust affiance founds, not only the standing spike fixture.

## Done when
A live assize opens end to end: Leg 1 under a TTY, Leg 2 yields the federated token, that token alone is cached.
The suite-head seam is shown: one compeared assize fed at suite head, the otherwise-headless cases consume it thereafter.

## Cinched
All human-in-the-loop federation proof consolidates here; the build paces wrap on machine-verifiable criteria and hand their live-proof obligation to this pace.
Runs after affiance founds the trust.

## Character
Operator-paced live walkthrough — interactive, needs a terminal and browser, not autonomous.

### levy-mantle-establishment (₢BZAAH) [complete]

**[260616-1302] complete**

Fold mantle establishment and audit-log enablement into levy.
Establish the three mantle SAs beside mason and grant every resource binding to them once, behind the settle gate; the mantle SAs reuse the named capability-set grant definitions lifted into code earlier in the substrate work.
Enable AR Data-Access audit logs at levy as a ceremony step.
Both gestures are depot-project-scoped, payor-authenticated, and share crash-leaves-visible-deficit, re-run semantics.

## Done when
Levy establishes the three mantle SAs with all resource IAM frozen behind the settle gate.
AR audit logs are enabled at levy for both ADMIN_READ and DATA_READ on the using service — not on iamcredentials, which rejects service-level auditConfig (see the federation-legs-spike-findings memo, V3).
A successor levy contract spec carries it, succeeding the live levy spec contract-first; acronym minted when firmed.

## Cinched
All resource IAM freezes at levy; hot paths zero-IAM.
Mantle SAs instantiate the capability-sets at levy behind the settle gate.

## Character
Founding-ceremony assembly — long, payor-driven, settle gate cheap there.

### accessor-compearance-assize (₢BZAAI) [complete]

**[260616-1303] complete**

Extend the identity-keyed credential accessor from the substrate work with its federation branch through Leg 2 — build only; the live human-in-the-loop proof and the curl-response custody gap are deferred (below).
Device-flow compearance is Leg 1, an accessor step never a tabtarget: on a live-assize miss with a TTY, run Leg 1 inline (verification_uri + user_code, poll); a headless miss fails loud — the fail-fast membrane, also the suite-head seam an automated run injects a compeared assize into.
The STS exchange is Leg 2 — a bare unauthenticated POST, audience = the provider resource name, nothing extra (federation-legs-spike-findings memo F3).
Cache the STS federated token only, per-assize; the IdP token is consumed by Leg 2 in-process.

## Done when
Leg 1 (inline, TTY-gated) and Leg 2 (STS) exist as accessor steps and are BCG-clean — shellcheck green.
The headless miss fails loud and the credless guard rejects, both machine-verified by the cloud probe tabtarget.
The assize caches the federated token alone (its shape lives in the token-custody cinch — do not restate); compearance consumes no colophon.

## Cinched
The branch extends the identity-keyed accessor — a token-mint path, not a second credential entry point.
compearance is an accessor step, not a tabtarget; only the federated token is cached in the assize.
This pace wraps on machine-verifiable criteria; the live end-to-end proof — Leg 1 browser approval, Leg 2 yielding the token, suite-head consumption — is handed to the human-in-the-loop federation proof railed after affiance.
The curl-response files still land the id/access/federated tokens in forensic temp — the IdP-token-never-persisted custody gap — a deferred design decision that rides a later custody/headless reconsideration, not this pace.

## Character
The behavior-heavy heart, first half — net-new runtime, no federation machinery existed in the kit. Build complete and machine-verified; runtime proof and custody gap deferred.

### accessor-don-leg3 (₢BZAAJ) [complete]

**[260617-1949] complete**

Complete the three-leg chain in the credential accessor: the don, Leg 3 — generateAccessToken on the mantle SA, naming the depot as quota project via the x-goog-user-project header; the short-lived mantle token lives in process memory only, never cached, carrying exactly one mantle's authority.
Overhang: a long run re-mints the mantle token mid-flight from the still-live cached federated token, and fails loud — "assize lapsed, compear" — once the assize lapses.
The two lifetime ceilings compose independently (see the federation-legs-spike-findings memo, V1); the mantle token outlives nothing here, and it is the assize that caps re-minting.

## Done when
A donned mantle token mints from the cached federated token; downstream bearer calls stay quota-project-blind and unchanged.
Mid-run re-mint works while the assize lives; assize lapse fails loud with the compear instruction.

## Cinched
A token carries exactly one mantle's authority; mantle tokens never cached.
The Leg-3 403 is the structural Palisade signature recorded in the spike findings (F2: missing quota-project header / serviceUsageConsumer grant) — recognized and failed loud, never retried as a propagation race, unlike the SA-propagation loops the accessor already carries.
Duration-aware admission control is a named deferral, not built here.

## Character
The behavior-heavy heart, second half — net-new federation runtime in the accessor, no don code exists today.

### terrier-provision (₢BZAAK) [complete]

**[260618-1302] complete**

Provision the terrier resource: build the idempotent bucket-ensure, the net-new managed-folder layer, and the destroy-then-create scaffold tabtarget that stands up and resets the freehold's terrier — with zero depot or manor quota churn.
The terrier bucket lives in the payor project, which RBS0 makes the manor itself; the bucket-create is an idempotent ensure (manor-shared), never the build-bucket's pristine 409-fatal, and enables Uniform Bucket-Level Access — managed folders require it.
Net-new, and the first survey target: GCS storage.managedFolders REST (create/delete, plus a teardown that deletes folders — today's object-only emptying does not), and a managed-folder IAM wrapper mirroring the AR repo-IAM idiom in the registry module over a new rbgi primitive shaped like the bucket-IAM one.
The scaffold provisions and resets at folder grain (recreating the whole bucket risks GCS's same-name reuse lag); its colophon is transitional — retired when ₣Bf consolidates, not broadside-registered. The later terrier-consuming paces charge against the terrier it stands up.
The manor=payor-project finding and the net-new surface are grounded in the pace-design memo's terrier-settled-design section; graft to it, do not re-derive.

## Done when
The scaffold stands up a live terrier on the standing depot freehold — bucket (UBLA enabled) in the payor project, a per-polity managed folder, write IAM folder-scoped to the depot's governor mantle SA (a cross-project grant onto the payor-project bucket), bucket-level read for every governor mantle — and resets it idempotently (a re-run reaches the same clean state).
A service fixture provisions via the scaffold and asserts the bucket, the per-polity folder, and the write/read IAM policy are present — a getIamPolicy check, not impersonation-enforcement (donning the mantle to prove own-folder-only belongs to the admission/foedus paces) — then asserts the idempotent reset; it self-skips without service credentials.
The noun internals this pace owns are settled and recorded in the RBS0 Terrier noun quoin: the bucket name and its constant home, and the managed-folder grain.

## Cinched
The terrier bucket lives in the payor project — RBS0 makes the manor and the payor project one entity (pace-design memo, terrier-settled-design); there is no separate manor project.
The grain: write is folder-scoped own-polity (managed-folder IAM granting the depot's governor mantle SA, cross-project); read is bucket-level manor-wide for every governor mantle (one bucket grant); the payor reads inherently as owner of the payor project.
The terrier's permanent founding-home is NOT this heat's: provision via the interim scaffold only — do not graft into affiance, levy, or any founding ceremony (that consolidation is ₣Bf's). The scaffold is interim, retired by ₣Bf, not demolished here.

## Character
The resource layer: idempotent provisioning plus the net-new managed-folder REST and IAM. The sub-ops that read and write muniments are the next pace.

### terrier-subops (₢BZAAb) [complete]

**[260618-1337] complete**

Realize the three muniment sub-operations against the provisioned terrier: engross (write), expunge (withdraw), and peruse (read), atomic per the GCS-precondition contract RBSTR settles.
Each mutation is a single conditioned REST call — ifGenerationMatch=0 create, generation-conditional update — with the bucket adjudicating atomicity; peruse is the unconditioned list-and-fetch.
This pace acts on a terrier the provisioning pace's scaffold has already stood up; it provisions nothing itself.
The atomic contract is settled in RBSTR; the muniment shape is this pace's to settle. Grounding: the pace-design memo's terrier-settled-design section.

## Done when
engross / expunge / peruse work against a scaffold-provisioned terrier and are atomic per the RBSTR contract.
An atomicity fixture asserts the 412-on-conflict precondition (a duplicate create receives HTTP 412 and is treated as idempotent success), charging the terrier via the provisioning scaffold; it self-skips without service credentials.
The noun internals this pace owns are settled and recorded in the RBS0 Terrier noun quoin: the muniment JSON shape under a fresh terrier sprue, and object granularity (per-entry versus per-subject).

## Cinched
Atomicity is GCS object preconditions per RBSTR — no external lock, no cloud invocation.
The sub-ops only read and write muniments; they never provision (that is the predecessor pace) and never touch founding.

## Character
The data layer over the provisioned resource: glue over a conditioned REST call, carrying no lock logic of its own.

### admission-verbs-polity (₢BZAAL) [complete]

**[260618-1610] complete**

Build the operator-facing federation admission surface — brevet, unseat, attaint, rehearse — under a new rbw-p polity launcher family, operating on (actor, mantle, principal) within a depot.
Thin idempotent compositions over the terrier sub-ops and the two IAM binding types, authored to the paddock Verbs-and-orderings table — do not re-derive the bodies or crash residues.

## Done when
brevet/unseat/attaint/rehearse operate over (actor, mantle, principal); the muniment write precedes every binding; first-vs-further admission differs only in the depot-scoped binding.
The depot-scoped binding survives unseat as suspension and is swept only by attaint; rehearse mutates nothing.
Four polity-verb contract specs carry the verbs contract-first; predecessor retirement is documentation succession (Forthcoming→Settled pointer per the paddock documentation-strategy), never deletion — the enrobe/defrock bridge specs stay live for M7.
A focused service-tier proof asserts the IAM composition (muniment-before-binding, idempotency, unseat-suspension vs attaint-sweep, rehearse-reads-nothing) by getIamPolicy read-back plus rehearse; self-skips credless.

## Cinched
Credential: the verbs run as a donned governor mantle (rba_compear then rba_don_capture governor — this pace is that accessor's first consumer); the founding-exception brevet (payor brevets the first governor, inside levy) is payor-credentialed over a shared brevet-core.
Roles are named constants in rbgc_constants.sh's Canonical Role IDs block (tokenCreator and serviceUsageConsumer); the one existing inline tokenCreator literal folds in.
The principal:// tokenCreator binding gets a focused new SA-scope add/revoke pair — a distinct canonical path, never a normalizer over the serviceAccount: functions (BCG Interface Contamination); serviceUsageConsumer uses the existing project-scope add/revoke with a verbatim member.
Verb bodies live in rbgp_payor.sh dispatched via rbgp_cli.sh, per the manor and terrier precedent.
rehearse's manor-wide read factors peruse's list-and-fetch into a shared lister with per-polity and manor-wide entry points; the manor-wide read is depot-blind, emits the (mantle, subject) record, changes no muniment content (depot stays recoverable from the key, unemitted), and skips an object that vanishes between list and fetch.
The live-don payoff proof (the grant actually enables impersonation, unseat denies it) is deferred to M7 with the keyless fixture rewrite, where that fixture must change regardless.
Design record: Memos/memo-20260615-BZ-pace-design.md.

## Character
Small idempotent operator bodies over the sub-ops and the two binding types; the design is settled — careful composition, not architecture.

### freehold-rbpc-substrate (₢BZAAY) [complete]

**[260619-0914] complete**

Build the rbpc freehold test-constants substrate and the manual don-mantle probe, in the static / no-in-test-IAM-churn shape settled in conversation 260619 (recorded in the pace-design memo's 260619 section).

Scope settled this session — supersedes the original roster+depot-moniker framing:
- rbpc homes a SINGLE durable freehold subject: the operator's standing Entra oid on the spike/freehold trust (the only real federated identity — multi-subject rosters presuppose the degenerate federation, deferred to the federation-evolution heat). The mantles are already RBCC constants; no citizen roster, no freehold depot moniker (depot identity is rbrd.env's evolving concern).
- A new segregated bash constants module under the rbpc prefix (proving-constants — grep-gated; the rbcc/rbgc pattern), homing the freehold subject. Test gestalt, deliberately NOT in RBCC.
- Project to Rust as a peer emit source: rbpc_emit_consts wired into rbz_emit_consts (which already composes the colophons + rbcc_emit_consts), landing RBTDGC_FREEHOLD_* in the generated rbtdgc_consts.rs. Do not fold into RBCC.
- One parameterized don-mantle probe tabtarget in the rbw-ac access family: compear as the freehold subject, then don a named mantle (governor|director|retriever) and report — or surface the admission-deficit 403 the accessor already emits. Couples to the depot regime (the don derives the depot project), unlike the depot-agnostic compearance probe (rbw-acf / rbgv_check_compearance is the model).

## Done when
The rbpc module homes the freehold subject and projects through rbz_emit_consts so the theurge build regenerates rbtdgc_consts.rs and rbq fast-qualify stays green; the don-mantle probe resolves the freehold subject, opens an assize, and dons the named mantle (or reports the admission deficit). Shellcheck clean.

## Cinched
rbpc is the chosen prefix (operator-elected; rbfc/rbtc/rbtf were grep collisions). Test constants stay segregated from RBCC (operator ruling); the module rides the existing codegen as a third peer emit source.
Static-admission doctrine (260619): the freehold test rig never mutates IAM mid-test. This pace provides only the substrate + the standing-don probe; the provision-once fixture, the never-granted negatives, and read-back revocation are the next pace's work — not built here.
Single freehold subject this heat; multi-subject negative differentiation (and the headless IdP behind it) is federation-evolution-heat work.

## Character
Mechanical build on settled design (260619). Sources: the pace-design memo (the static-admission doctrine + the identity-layers model + the literal subject oid), the config-model memo, affiance commit 217b592a5, RBSMA, and the rbw-acf / rbgv_check_compearance compearance model.

### foedus-freehold (₢BZAAc) [complete]

**[260619-1020] complete**

Nucleate the foedus-freehold fixture: durable, quota-flat verification that the standing freehold trust stands and carries its known citizen roster — the counterpart to the ephemeral foedus-lifecycle round-trip.

Consumes the freehold rbpc substrate (the preceding pace): the well-known citizens (RBTDGC_FREEHOLD_*) are the roster this fixture asserts.

Automatable scope (payor-credentialed, headless, self-skips credless):
- Establish-if-absent: affiance against the standing pool, asserting the idempotent no-op on the live pool (affiance is now fail-fast on soft-delete) plus pool/provider config.
- Seed: ensure the well-known citizens are breveted onto the freehold mantles, payor-credentialed through the token-agnostic cores (the admission-proof precedent — no compearance).
- Roster: read the manor-wide muniment roll payor-credentialed (the terrier is payor-project data, as admission-proof does — NOT the governor-donned rehearse verb, which needs compearance) and assert the well-known (subject, mantle) pairs are present.
- Service + complete suite member.

Out of scope — a genuine hole, owned by ₣Bf: the runtime/sign-in proof (a citizen compearing, donning a mantle, making an attributed call) needs headless compearance, which is open in ₣Bf (the compearance-headless reconsideration and the caged test-manor topology forks). Live runtime attribution is the separate operator-paced attribution-verification pace.

## Done when
The fixture verifies the standing freehold headless: affiance no-op + config, well-known citizens ensured-breveted (payor-credentialed), and the manor-wide roster asserted to carry them; it self-skips credless; the service and complete suites stay green.

## Cinched
Reuses one durable standing pool (quota-flat) — distinct from foedus-lifecycle's throwaway churn.
The roster read is payor-credentialed, not the rehearse verb (compearance-free).
Runtime/compearance verification is out of scope — it is the ₣Bf hole and the operator-paced attribution pace.

## Character
Rust fixture authoring on settled design; consumes and sequences after the substrate pace. Sources: the config-model and pace-design memos.

### citizen-attribution-proof (₢BZAAV) [complete]

**[260619-1127] complete**

Close the federation's true end to end: a brevetted citizen dons a levy mantle and the Artifact Registry Data-Access log attributes the act to the human.
This is the payoff of the levy's audit-log enablement and the spike's V3 attribution finding — the live audit-log read the build paces defer.
It rides the whole runtime: levied mantles plus audit config, a compeared federated token, the Leg-3 don, and a brevet binding the citizen (tokenCreator on the mantle, serviceUsageConsumer on the depot) — so it lands only once the don and the admission surface are both up.

## Done when
A brevetted citizen mints a mantle access token via the Leg-3 don, makes an Artifact Registry call under it, and the resulting Data-Access log entry carries serviceAccountDelegationInfo[].principalSubject equal to the federate's immutable IdP subject.
The mint-hop quirk the spike flagged — principalEmail is the mantle SA, the subject absent from the mint record — is observed at the using-service log and not mistaken for a failure.

## Cinched
The live audit-attribution read, distinct from the admission-verb suites: it asserts the using-service log shape, not the binding behavior.
Rides the standing runtime — the don and the admission surface must both be up, against a levied depot reached by a compeared assize.

## Character
Operator-paced live verification reading real Cloud Logging audit entries — interactive, follows the federation surface; read the federation-legs-spike findings memo (V3) for the exact log shape.

### payor-cred-federation-suite (₢BZAAd) [complete]

**[260620-0756] complete**

Add a named theurge test-suite composing the payor-credentialed federation fixtures, so a federation pace has a green-able confirmation target that excludes the bridge-legacy keyfile estate — then retire the standalone proof tabtargets the suite renders redundant.

Motivation (surfaced live this heat): the service and complete suites cannot go green while the org enforces disableServiceAccountKeyCreation.
Service reds at access-probe on the keyfile director/retriever, which can no longer be enrobed — the keyfile-estate collision the RBRA-estate-retirement movement owns.
The payor-credentialed federation fixtures (foedus-freehold, admission-proof, terrier-scaffold, terrier-atomicity) ride above that estate and pass on payor credentials alone, but they live only inside service/complete, so today there is no suite a federation pace can confirm green against.

Scope: a new entry in the theurge suite registry (the RBTDRC_SUITES home in rbtdrc_crucible.rs), plus its rbw-ts tabtarget and zipper enrollment, mirroring the existing suite-plus-tabtarget pattern.
Compose the payor-credentialed federation fixtures only; the exact membership, the suite word, and whether the fast base rides along are mount-time calls under MCM Word Selection.

Once the suite gives those fixtures a green-able home, the three standalone proof drivers — rbw-pF, rbw-pP, rbw-dT — are duplicative operator surface that bakes the freehold test rig into the production rbw-p/rbw-d families; each already has its backing fixture, so retire the three as operator CLI while keeping their proofs reachable through the fixture/suite layer (whether a fixture drives its verb directly or keeps an internal driver is a mount-time call).
The genuine diagnostics stay — rbw-acm and rbw-da are real operator reads, not drivers — and the terrier scaffold rbw-dt stays until ₣Bf homes the terrier permanently.

## Done when
A named suite runs the payor-credentialed federation fixtures, self-skips credless, and is green-able on a levied freehold without any keyfile-estate fixture (no access-probe, cloud-build, or GAR fixtures the key-creation policy breaks).
The three interim proof tabtargets (rbw-pF, rbw-pP, rbw-dT) are gone from the operator CLI, their proofs still reached through the fixture/suite layer.
The suite word and tabtarget are minted, the generated consts/context are re-derived by the build, and fast-qualify is green.

## Cinched
Membership is the payor-credentialed slice only — it deliberately excludes the keyfile-dependent fixtures (access-probe, the hallmark/lode/reliquary/wsl/podvm lifecycles, batch-vouch) the org's disableServiceAccountKeyCreation currently breaks.
This is the interim green target until the RBRA-estate-retirement movement lands the keyless cloud-build credential path and service/complete are whole again.
Retirement is scoped to the three interim proof drivers only; the diagnostic surface (rbw-acm, rbw-da) stays, and the terrier scaffold rbw-dt is out of scope — its removal is the terrier-home work deferred to ₣Bf.

## Character
Test-infra; mechanical — a suite-registry entry plus tabtarget mint mirroring existing patterns, then a tabtarget retirement (zipper de-enrollment + build re-derivation).
Sources: the foedus-freehold wrap rationale and the live keyfile-estate finding recorded in the RBRA-estate-retirement pace's docket.

### quotabuild-payor-regroup (₢BZAAO) [complete]

**[260620-0959] complete**

Regroup the orphan rbw-gq (QuotaBuild) colophon under the Payor guide subfamily as rbw-gPQ.
Cloud Build quota/capacity is a Payor concern — the Payor owns the project, billing, and quota; the Director only consumes builds — so QuotaBuild joins rbw-gPE/gPI/gPR.
Source: the resolved slate-pick in Memos/memo-20260615-BZ-pace-design.md ("rbw-gq (QuotaBuild)" disposition).

The colophon string is hand-edited in one place — the rbz_zipper.sh enrollment — plus the tabtarget filename; rbtdgc_consts.rs and claude-rbk-tabtarget-context.md re-derive from the theurge build, never hand-edited.
The op/function name rbhp_quota_build and the RBSQB / RBS0 references key on the function, not the colophon, so they do not move.
Discovery gate: grep rbw-gq repo-wide before finishing — catch any operator-facing literal hints in guide/handbook output.

Out of scope, routed away: the manor-demesne consolidation movement 5 also originally implied — pulling levy and establish into rbw-m beside affiance/jilt — is a contested which-axis legibility call handed to the terminal Fable review, recorded as a divergence in the pace-design memo. Do not undertake it here.

## Done when
rbw-gq is renamed to rbw-gPQ: tabtarget file renamed, zipper enrollment updated, theurge build re-run so the generated consts and tabtarget-context re-derive.
grep rbw-gq lands clean repo-wide.
Fast-qualify (rbw-tq) green.

## Cinched
Scope is the gq->gPQ rename only; the manor-demesne (levy/establish) consolidation is routed to the terminal Fable review, not executed here.
QuotaBuild stays a guide — rbhp_quota_build body unchanged; only its colophon home moves.

## Character
Mechanical colophon rename — one zipper line, one file rename, a build, a grep gate.

### keyless-canonical-bootstrap (₢BZAAQ) [complete]

**[260620-1225] complete**

Rewrite the canonical credential-bootstrap fixture (rbtdrk_canonical.rs) so a fresh levy admits federation personas — compear, gird the first governor, brevet+don each mantle — instead of enrobing keyfile SAs.
This makes canonical-establish pass on the no-keys org and meets the paddock's "federation personas pass the suites" gate.

Forced now, ahead of M7's gate: the org enforces disableServiceAccountKeyCreation, so the keyfile governor-enrobe case 400s on any fresh levy and canonical-establish dies there (live-reproduced: "Key creation is not allowed on this service account").
This is the M4/M7 review's named revisit trigger firing from an external policy change — Memos/memo-20260615-BZ-m4-review-findings.md (THE OPEN QUESTION) and the pace-design memo's M7 record carry the reasoning and the estate map.

As built, the chicken-and-egg the docket's "replacement verbs already enrolled" premise missed: brevet wields a donned governor, so it cannot seat the FIRST governor on a fresh depot.
The founding verb gird (rbw-pE, contract RBSPG) was minted to fill it — the payor seats the first governor with no key, driving the shared brevet core with the payor's OAuth token; mint recorded in the pace-design memo, flagged for the terminal Fable review.
The other replacement verbs were already enrolled — affiance (rbw-mA), brevet (rbw-pB), compear (rbw-acf), don (rbw-acm).

## Done when
The canonical-establish enrobe cases admit federation personas (compear / gird / brevet / don), not keyfile enrobe + JWT probe.
A fresh-levy canonical-establish completes on the no-keys org — proven 6/6: levy → compear → gird governor → brevet+don director → brevet+don retriever → recognosce.
The keyfile estate stays present and untouched — canonical-enrobe keeps the keyfile cases for skirmish/dogfight/blockade.

## Cinched
Keyless rewrite only; the estate teardown is the following movement (RBRA estate demolition).
The full gauntlet greening on the no-keys org is NOT this pace's gate — the gauntlet's downstream foundry runtime still authenticates director/retriever through the keyfile accessor branch (rba_token_capture), which the RBRA estate demolition retires; that movement owns "complete green," and it surfaces first at hallmark-lifecycle.
Federation personas passing canonical-establish IS the paddock's M7 gate — this pace meets it, retiring the review's recorded residual risk early.
Composes with the freehold-collapse pace (disjoint regions of the same fixture), which defers its enrobe-idempotency here and follows this pace.

## Character
Behavior-changing fixture rewrite plus a founding-verb mint (gird); credless-green, federation path proven 6/6 live on the no-keys org.

### terrier-graft-demolish (₢BZAAa) [abandoned]

**[260618-1054] abandoned**

Abandoned 260618. The terrier's founding-integration — wiring the bucket-ensure and the per-depot folder into manor establishment, and retiring the scaffold — is consolidation work that belongs to ₣Bf, not this heat.
₣BZ provisions the terrier via the interim scaffold only (the terrier build pace); the permanent founding-home turns on the foedus and multiple-federations questions reserved for Fable.
See ₣Bf's "terrier's permanent founding-home" idea and the pace-design memo's re-cut section.

## Done when
Abandoned — no completion. Superseded by the ₣Bf consolidation idea.

## Cinched
The terrier's permanent founding-home is ₣Bf's, decided alongside foedus and multiple-federations.

## Character
Tombstone — records that founding-integration was considered in ₣BZ and moved to ₣Bf, so a future groom finds the answer here, not blank space.

### freehold-collapse-cleanup (₢BZAAZ) [complete]

**[260620-1546] complete**

Finish the collapse of the depot test-installation to a single freehold scheme — the cleanup the quota-driven reuse decision set in motion.
The proof pace realized the first pieces (idempotent reuse-or-create ensure, the deliberate churn fixture); two test-prefix schemes still coexist — the canonical canc-/canr- family and pristine's throwaway family — each with its own duplicated autoincrement pick-next.
Collapse to one freehold scheme: one prefix family, one pick-next, no "which scheme is live" to track — the path-dependence that bit during the quota stall.

## Done when
One depot test-prefix scheme remains and the duplicated autoincrement logic is unified.
The fixtures and the depot_levy function (which reuses-or-levies now, not just levies) are renamed to the freehold vocabulary — exact names minted at mount under MCM Word Selection.
rbtdrk_canonical.rs (~1075 lines) is split at a natural seam, back under the RCG 800-line threshold.
The suites exercising these fixtures stay green.

## Cinched
Collapse to one freehold is the operator decision — the quota wound made two-scheme tracking too costly.
Churn stays deliberate and a member of no suite; the safety property — a routine fixture never silently destroys the freehold — is preserved by that deliberate-churn separation, not by disjoint prefixes.
Enrobe-idempotency is out of scope here: the keyfile enrobe is rewritten keyless by the RBRA-estate-retirement movement, which subsumes it.

## Character
Test-infra refactor — mostly mechanical rename/unify/split, but it touches shared fixture machinery; sequence against the RBRA-estate-retirement movement's keyless rewrite of the canonical credential-bootstrap fixture, since both touch the same fixtures.

### accessor-don-rewire (₢BZAAf) [complete]

**[260621-1307] complete**

Wire the production credential accessor onto the federation don, so the foundry/lode/bottle surface mints mantle tokens instead of keyfile JWTs.
This is the accessor federation branch the paddock's movement-4 ordering expected ("the federation branch lands at movement 4") but movement 4 left unbuilt — and it must land before the RBRA estate can be torn down.

`rba_token_capture` (rba_auth.sh) is today a keyfile-only bridge with 55 production callers; `rba_don_capture` + `rba_compear` exist but reach only the polity admission verbs (rbgp_payor.sh) and the two access probes — no foundry path dons.
Route `rba_token_capture`'s body through `rba_compear` (ensure-assize) then `rba_don_capture <identity>`, leaving the 55 callers and their bearer-blind downstream unchanged.
Reconcile the direct keyfile reads (the `source RBDC_DIRECTOR_RBRA_FILE` SA-identity loads in rbfd_director.sh) and the consumer `test -f RBRA_FILE` precondition guards onto mantle-SA derivation (RBCC_account_mantle_* in the depot project), and re-kindle the foundry furnish graphs that source only rbgo/rba for the keyfile mint.
Discovery: `git grep -n 'rba_token_capture\|RBRA_FILE' Tools/rbk/`, minus the RBRA estate files the successor teardown owns (RBDC_*_RBRA_FILE derivation, rbra regime/CLI, RBSRA).

## Done when
The foundry mints via the don: `rba_token_capture` reads no keyfile, `rbgo_get_token_capture` has no live caller, and a service-tier run plus a live dogfight (ordain → summon under a donned director mantle) is green on federation personas.
The RBRA estate constants/regime/spec stay standing — grep RBRA_FILE clean is the successor teardown's gate, not this pace's.

## Cinched
The live credential swap the estate-demolition pace explicitly is not — its own movement, proven by its own live run before the teardown sweeps the now-dead keyfile path.
The mantle-SA identity model (RBCC_account_mantle_* in the depot) is settled; this pace wires the foundry to it, mints no new credential concept.

## Character
Architectural and wide: production blast radius — every foundry op's credential path changes — proven only by a live run.

### assize-duration-12h (₢BZAAg) [complete]

**[260621-1447] complete**

Bump the assize cap from 1h to 12h so a long suite run (gauntlet, skirmish, service) completes on a single morning compearance — no mid-run re-compear to babysit.

The 1h cap is the workforce pool's `sessionDuration`, not a runtime token lifetime: the STS leg requests no lifetime, so the assize equals whatever the pool was provisioned with at create (currently `RBRF_SESSION_DURATION=3600s`).

Two parts, and the second is the catch:
- Bump `RBRF_SESSION_DURATION` in rbrf.env to 43200s (12h — the pool `sessionDuration` ceiling).
- PATCH the *existing* pool's `sessionDuration` to match. Affiance is ensure-exists-only and leaves a live pool unpatched — the pool-PATCH-`sessionDuration` reconciliation is a named-deferred follow-up (grep `sessionDuration` in rbgp_payor.sh). Apply a one-shot PATCH to the standing test-trust pool, or wire the deferred reconciliation.

## Done when
A long suite run on the standing test trust completes on one compearance — the assize outlives the run, no mid-flight re-compear.
`rbw-acf` reports the federated token expiring in ~12h, not ~1h.

## Cinched
12h is within the already-cinched assize range (900s–43200s) — a config bump, not a design change.
The minimal form (regime bump + one-shot pool PATCH of the standing trust) is this pace; generalizing affiance to reconcile `sessionDuration` drift is affiance-evolution — route that to ₣Bf.
If wiring the deferred reconciliation (bash in rbgp_payor.sh), it follows BCG (Tools/buk/vov_veiled/BCG-BashConsoleGuide.md) — read it first; shellcheck-green is necessary but not sufficient. The one-shot PATCH form touches no committed bash.

## Character
Small config pace with one wrinkle — the live pool needs a `sessionDuration` PATCH that affiance won't apply to an existing pool.

### rbra-estate-demolition (₢BZAAe) [complete]

**[260621-1544] complete**

Sweep the orphaned RBRA keyfile estate once the foundry mints via the federation don — the bridge's demolition condition, executed.
The dead-legacy half of this estate is already done (the cult-verb estate and the keyfile JWT probes); the live half waited on the preceding pace, which reroutes rba_token_capture and the foundry consumers onto the don and leaves the estate below unreferenced.

Remaining estate (rediscover with grep RBRA_FILE and grep rbgo_get_token_capture):
- the keyfile JWT mint rbgo_get_token_capture + its rbgo keyfile machinery, now caller-less;
- the RBDC_*_RBRA_FILE constants + derivation (rbdc_derived.sh), the rbra regime + rbra_cli.sh, and the rbra regime file;
- RBSRA + its RBS0 quoin seats/includes, and the rbtoe_*_authenticate keyfile JWT-auth patterns in RBS0 (+ the RBSAP ref);
- the Rust remainder — rbtdgc_consts RBTDGC_RBRA_FILE, the rbtdrk_freehold_rbra helper + its import, rbtdrp_attest RBRA refs, the fast-tier credless guard's keyfile aspect, and the dogfight/onboarding RBRA-presence probes (likely removed — those suites' federation credential-readiness is already deferred to the test-rig heat; confirm at mount).

## Done when
grep RBRA_FILE and grep rbgo_get_token_capture land clean repo-wide; no keyfile mint and no RBRA estate anywhere in the kit.
RBSRA is gone and RBS0 is internally consistent with the seats removed.
The theurge build regenerates the consts and tabtarget-context; shellcheck and the suites stay green; complete green before the heat closes.
A live gauntlet plus skirmish on federation personas verifies the rewire and this teardown together before close, never assumed from complete-green.

## Cinched
Runs only after the preceding pace proves the foundry mints via the don — teardown of now-dead keyfile machinery, not a credential swap (the swap is that preceding pace).
The rbgw capability-set definitions stay (they realize the mantle grant lists, not keyfile machinery).
Bash edits follow BCG (Tools/buk/vov_veiled/BCG-BashConsoleGuide.md) — read it before authoring; shellcheck-green is necessary but not sufficient (it cannot see capture-purity, $()-on-externals, heredoc avoidance, or commit-message comments).

## Character
Large mechanical-but-wide demolition of an orphaned estate across bash, theurge-Rust, and specs; the risk is reach, not depth — the preceding rewire has already de-risked the live behavior.

### federation-narrative-broadside (₢BZAAP) [complete]

**[260622-0106] complete**

Invert the README federation narrative and register the elected vocabulary on the broadside (the README glossary). Today the README frames keyfile as the whole current system and federation as a planned future tier; this pace flips it — federation is the model, keyfile is retired bridge legacy now gone, the two-tier framing collapses to one. Scope: README + broadside glossary + diagrams; the handbook rework is ₣A6, not here.

Findings, full elected-word list, and rationale are in the pace-design memo's M6 record. Rediscover live: grep the README for the keyfile/RBRA/"planned" framing, the elected words, and the diagrams/ embeds.

Two traps a vocabulary top-up would miss: the glossary's "Mantle — create the Governor service account" is the cult verb sense and must flip to the federation standing-role noun (meaning and part of speech); the federation-seam diagram's "mode-enum branch" caption is stale — the mode enum was canceled (single tier).

## Done when
The README tells federation as the model — no keyfile tier presented as current, no two-tier framing.
The broadside registers the permanent elected words (mantle, compear, assize, don, brevet, unseat, attaint, affiance, jilt, rehearse, muniment, citizen, census) and retires the keyfile entries (RBRA, the cult Mantle verb); transitional enrobe/defrock never reach the broadside; RBRO stays.
The keyfile-login diagram (rbdgk) is gone and the federation-seam diagram drops the mode-enum claim; the pluml fixture re-renders the diagram set.
A post-build cold-probe confirms the elected words read first-contact actionable.

## Cinched
One narrative pace — README, broadside, diagrams; not split.
Runs after the estate demolition so the narrative describes the real keyless state; handbook rework stays in ₣A6.

## Character
Wide rewrite of one public document and its diagrams — judgment in prose and vocabulary, mechanical in the glossary/diagram plumbing.

### fable-heat-review (₢BZAAN) [complete]

**[260622-0759] complete**

Full review by model Fable of heat ₣BZ as executed — reconcile the frozen paddock against what was built, with the movement-4 review-findings memo as the divergence record and entry point.
The paddock was frozen through execution under the Opus-stewardship banner; this is where the freeze lifts — Fable re-reads the paddock against the landed paces and the findings memo, then re-authors the paddock or ratifies the divergences.
The manor-identity line is left in Fable's original wording under the freeze, flagged only by a banner pointer to the memo; this review reconciles it in the body along with the rest.
Terminal by design: it runs after the federation surface lands, when the as-built reality and the recorded divergences are both complete; later movements slate before it.

## Done when
Model Fable has read the movement-4 review-findings memo and re-read the frozen paddock against the heat as built.
Each recorded Opus/Fable divergence is ratified or overturned; the medium-confidence M4-vs-M7 suite-gate reading is sanity-checked.
The freehold doctrine arrived at post-freeze (under operator sanction) is reconciled into the paddock: the freehold concept, the foedus-lifecycle/foedus-freehold split, the standing-citizen roster correction, and the foedus↔depot intertwining.
The terrier homing is reconciled: the paddock's "payor-created at levy" is superseded — ₣BZ provisions the terrier via an interim scaffold and defers the permanent founding-home to ₣Bf's consolidation (foedus / multiple-federations / affiance-evolution), so confirm this heat-split rather than expecting a ₣BZ founding-integration; the read-population grain is settled (write folder-scoped own-polity, read bucket-level manor-wide) by the terrier paces.
The manor-identity divergence is adjudicated: RBS0 makes the Manor the Payor Project (a building), while the federation conversion introduced an org-level "deed" sense; the provenance hypothesis (Fable confabulation vs editor spec/impl drift) is weighed, and the manor-prerequisite line — left in original under the freeze, banner-flagged — is corrected in the body and the paddock made internally consistent on this point.
The paddock is re-authored or explicitly affirmed and its freeze banner resolved.

## Cinched
This pace requires model Fable — the paddock was authored at Fable density and the freeze lifts only to Fable's re-authoring (or operator sanction in conversation).
The entry point is Memos/memo-20260615-BZ-m4-review-findings.md; it carries the five movement-4 decisions, the M4-vs-M7 reading, and the per-docket divergences for cold review.
Also read, in the pace-design memo, the freehold doctrine, the terrier-settled-design section, and the manor-identity section, plus Memos/memo-20260617-BZ-workforce-pool-constraints.md — the post-freeze sharpenings the paddock does not yet fully reflect.

## As-built baseline (shelved 260622)
The heat was shelved (stabled, not retired) before this Fable review, so work continues on the repo meanwhile — read the federation surface as it stood at shelve time, not as later commits leave it.
Baseline HEAD at shelve: af0e08eee (the silks-rename commit; the federation surface itself landed through ₢BZAAP at 28ed94a0a).
Recover the full affiliated commit set with `jjx_log ₣BZ` — the heat→commit affiliation is committed gallops state and survives the shelving immutably; do not trust any hand-copied enumeration over it.
As-built caveats not captured durably in any spec (carried only in commit-message PENDING notes, pinned here against erosion):
- The pluml SVG re-renders (rbdgl / rbdgm / rbdgs) are stale — blocked on an interactive compearance this headless work could not perform; the README still displays the pre-rewrite federation diagrams until someone runs `tt/rbw-tf.FixtureRun.sh pluml` after a compearance on a clean tree.
- The 12h assize cap (RBRF_SESSION_DURATION 43200s) is live in regime and on the standing test pool, but only a fresh compearance picks it up.
- `rbw-acf` federated-access verification is deferred — it needs a human device-flow click, not run during this work.

## Character
Terminal reconciliation under Fable; the one pace that lifts the paddock freeze.

### fable-sequence-retrospective (₢BZAAh) [complete]

**[260701-2129] complete**

A terminal, Fable-only retrospective that mines heat ₣BZ's pace sequence and divergence arc for insights that inform how future heats of comparable scope and duration are planned.
Not a redo of the paddock reconciliation (that landed under operator sanction) and not the queue memo's ratify-or-sweep agenda — this studies the SHAPE of the execution: where the planned sequence held, where it bent, and why.

## Done when
Model Fable has read the heat's affiliated commit sequence via `jjx_log ₣BZ` (the immutable affiliation record, authoritative over any hand-copied enumeration) and the as-built pace ordering.
Fable has characterized where execution diverged from the original plan — the freeze-under-stewardship arc, the mid-heat reslates, the forcing functions (e.g. the org keyless-policy rewrite that pulled work ahead of plan), and the heat-splits that pushed work into sibling heats.
The findings land in a forward-looking memo: what this heat's sequence teaches about planning future heats — segmentation, freeze discipline, divergence budgeting, when to split a heat versus absorb the change.
The memo points at decision homes rather than restating them; any durable planning doctrine that should outlive the memo is flagged for a real spec home.

## Cinched
Requires model Fable — hard gate; this pace does not open to a lesser tier even under time pressure.
Entry points: `jjx_log ₣BZ`, the heat paddock (`jjx_paddock ₣BZ`), `Memos/memo-20260615-BZ-m4-review-findings.md`, `Memos/memo-20260615-BZ-pace-design.md`, and `Memos/memo-20260622-fable-review-queue.md` (read the last to stay OFF the already-tracked ratify-or-sweep work).
Distinct charter from the completed terminal reconciliation pace — that record stays untouched; this is additive process-retrospective, not a redo.

## Character
Forward-looking retrospective; genuine Fable judgment over the heat's shape, not a mechanical pass.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 R known-good-floor
  2 A federation-legs-spike
  3 B ashlar-vocabulary-mint
  4 C pace-design-membrane
  5 D accessor-seam
  6 E capability-sets-as-code
  7 F cult-rename-enrobe-defrock
  8 G rbs0-civic-quoins
  9 M manor-affiance
  10 X manor-jilt
  11 W foedus-lifecycle
  12 U levy-founding-proof
  13 T federation-live-proof
  14 H levy-mantle-establishment
  15 I accessor-compearance-assize
  16 J accessor-don-leg3
  17 K terrier-provision
  18 b terrier-subops
  19 L admission-verbs-polity
  20 Y freehold-rbpc-substrate
  21 c foedus-freehold
  22 V citizen-attribution-proof
  23 d payor-cred-federation-suite
  24 O quotabuild-payor-regroup
  25 Q keyless-canonical-bootstrap
  26 Z freehold-collapse-cleanup
  27 f accessor-don-rewire
  28 g assize-duration-12h
  29 e rbra-estate-demolition
  30 P federation-narrative-broadside
  31 N fable-heat-review
  32 h fable-sequence-retrospective

RABCDEFGMXWUTHIJKbLYcVdOQZfgePNh
····xxx·xxxx·x··xxxxxxx·x···x··· rbgp_payor.sh
······x·xx·x··x·xxxxxxxxx···x··· claude-rbk-tabtarget-context.md, rbtdgc_consts.rs, rbz_zipper.sh
······xxxx·x·xx·xxx·····x···x··· RBS0-SpecTop.adoc
···x··x······xx··xxxx·x·x···x··· claude-rbk-acronyms.md
······x···xx····xxx·x·x··x··x··· rbtdrc_crucible.rs, rbtdrm_manifest.rs
········xx···x··xxx·x·x·x······· rbgp_cli.sh
······x······x·xx·xx·x·········· rbgc_constants.sh
····x·········xx···x······x·x··· rba_auth.sh
····xxx······x·····x········x··· rbgg_governor.sh
······x······xx····x········x··· rbcc_constants.sh
··············x····x·x······x··· rbgv_cli.sh
······x······x··x···········x··· RBSCIG-IamGrantContracts.adoc
······x····x············xx······ rbtdrk_canonical.rs
···x····················xx··x··· memo-20260615-BZ-pace-design.md
········x·x········x············ RBSMA-manor_affiance.adoc
······x··················x··x··· rbtdrd_dogfight.rs, rbtdro_onboarding.rs
······x···········x·········x··· RBSDD-director_defrock.adoc, RBSDK-director_enrobe.adoc, RBSRD-retriever_defrock.adoc, RBSRK-retriever_enrobe.adoc
······x·········x·x············· rbgi_iam.sh
······x······x··············x··· RBSHR-HorizonRoadmap.adoc
······x····x·············x······ rbtdtk_canonical.rs
····x····················xx····· rbob_cli.sh
····x················x······x··· rbgv_probe.sh
····x··············x······x····· rbfd_director.sh, rbldb_bole.sh, rbldr_reliquary.sh, rbldv_immure.sh, rbldw_underpin.sh
···x···x·········x·············· RBSTR-Terrier.adoc
··························x·x··· rbldd_delete.sh
·························x··x··· lib.rs, rbtdrk_depot.rs, rbtdrk_enrobe.rs, rbtdrk_freehold.rs, rbtdrp_attest.rs, rbtdrp_lifecycle.rs, rbtdtp_lifecycle.rs
····················x·x········· rbw-pF.FreeholdProof.sh
··················x···x········· rbw-pP.AdmissionProof.sh
·················xx············· rbgft_terrier.sh
··············x············x···· rbrf.env
··············x····x············ rbrf_regime.sh
·············x··············x··· RBSMF-depot_levy.adoc
·············x············x····· rbgg_cli.sh
······x·····················x··· RBSCIP-IamPropagation.adoc, RBSGM-governor_enrobe.adoc, RBSGS-GettingStarted.adoc, rbhogw_governor_wrapper.sh, rbhopw_payor_wrapper.sh, rbw-aE.PayorEnrobesGovernor.sh, rbw-adE.GovernorEnrobesDirector.sh, rbw-adF.GovernorDefrocksDirector.sh, rbw-arE.GovernorEnrobesRetriever.sh, rbw-arF.GovernorDefrocksRetriever.sh, rbyc_common.sh
······x··················x······ rbtdrp_pristine.rs
····x·····················x····· rbfc0_cli.sh, rbfcg_gar.sh, rbfcp_plumb.sh, rbfln_inventory.sh, rbflw_wrest.sh, rbfr_cli.sh, rbfr_retriever.sh, rbfv_verify.sh, rbldl_lifecycle.sh, rbob_bottle.sh
····x···········x··············· rbgb_buckets.sh
·······························x memo-20260701-BZ-sequence-retrospective.md
······························x· memo-20260622-fable-review-queue.md
·····························x·· README.md, rbdgk_keyfile-login-dark.svg, rbdgk_keyfile-login.puml, rbdgk_keyfile-login.svg, rbdgl_federation-login.puml, rbdgm_federation-seam.puml, rbdgs_federation-setup.puml
····························x··· BUS0-BashUtilitiesSpec.adoc, RBSAJ-access_jwt_probe.adoc, RBSAP-ark_plumb.adoc, RBSDR-director_roster.adoc, RBSIP-ifrit_pentester.adoc, RBSRA-CredentialFormat.adoc, RBSRL-retriever_roster.adoc, RBSRR-RegimeRepo.adoc, claude-rbk-theurge-ifrit-context.md, rbdc_derived.sh, rbgo_oauth.sh, rbho0_cli.sh, rbho0_onboarding.sh, rbho0_start_here.sh, rbhocc_crash_course.sh, rbhocd_credential_director.sh, rbhocr_credential_retriever.sh, rbhoda_director_airgap.sh, rbhodb_director_bind.sh, rbhodf_director_first_build.sh, rbhodg_director_graft.sh, rbhp0_cli.sh, rblm_cli.sh, rbra_cli.sh, rbra_regime.sh, rbtdrf_fast.rs, rbtdrf_handbook.rs, rbtdrs_poison.rs, rbw-Ocd.OnboardingCredentialDirector.sh, rbw-Ocr.OnboardingCredentialRetriever.sh, rbw-Og.OnboardingGovernor.sh, rbw-acd.CheckDirectorCredential.sh, rbw-acg.CheckGovernorCredential.sh, rbw-acr.CheckRetrieverCredential.sh, rbw-adr.GovernorRostersDirectors.sh, rbw-arr.GovernorRostersRetrievers.sh, rbw-ral.ListAuthRegimes.sh, rbw-rar.RenderAuthRegime.sh, rbw-rav.ValidateAuthRegime.sh
··························x····· rbfd_cli.sh, rbfl0_cli.sh, rbfl0_ledger.sh, rbfv_cli.sh, rbld0_cli.sh, rbld0_lode.sh
·························x······ CLAUDE.md, RBSDL-depot_list.adoc, VMS-VoxMatriculaSpec.adoc, rbrd_regime.sh, rbtdtk_freehold.rs, rbtdtp_pristine.rs
························x······· RBSPG-governor_gird.adoc, rbw-pE.PayorGirdsGovernor.sh
·······················x········ rbw-gPQ.QuotaBuild.sh, rbw-gq.QuotaBuild.sh
·····················x·········· rbw-da.DepotAttribution.sh
···················x············ rbfk_kludge.sh, rbndb_base.sh, rbpc_constants.sh, rbq_cli.sh, rbte_cli.sh, rbw-acm.CheckMantleDon.sh
··················x············· RBSPA-citizen_attaint.adoc, RBSPB-citizen_brevet.adoc, RBSPO-terrier_rehearse.adoc, RBSPU-citizen_unseat.adoc, rbw-pA.GovernorAttaintsCitizen.sh, rbw-pB.GovernorBrevetsCitizen.sh, rbw-pU.GovernorUnseatsCitizen.sh, rbw-pr.GovernorRehearsesTerrier.sh
·················x·············· rbw-dT.TerrierProof.sh
················x··············· rbw-dt.TerrierScaffold.sh
··············x················· RBSRF-RegimeFederation.adoc, rbrf_cli.sh, rbw-acf.CheckFederatedAccess.sh, rbw-rfr.RenderFederationRegime.sh, rbw-rfv.ValidateFederationRegime.sh
·············x·················· rbgd_depot.sh, rbgw_capabilities.sh
···········x···················· RBSDC-depot_recognosce.adoc, rbw-dr.DepotRecognosce.sh
·········x······················ RBSMJ-manor_jilt.adoc, rbw-mJ.PayorJiltsManor.sh
········x······················· rbw-mA.PayorAffiancesManor.sh
······x························· RBSCB-CloudBuildPosture.adoc, RBSDD-director_divest.adoc, RBSDE-depot_levy.adoc, RBSDK-director_invest.adoc, RBSGM-governor_mantle.adoc, RBSRD-retriever_divest.adoc, RBSRK-retriever_invest.adoc, rbw-aM.PayorMantlesGovernor.sh, rbw-adD.GovernorDivestsDirector.sh, rbw-adI.GovernorInvestsDirector.sh, rbw-arD.GovernorDivestsRetriever.sh, rbw-arI.GovernorInvestsRetriever.sh
····x··························· rbfcb_host.sh, rbfld_delete.sh, rbfly_yoke.sh, rbga_registry.sh
···x···························· memo-20260612-acg-pilot-sentry-findings.md, memo-20260615-BZ-m4-review-findings.md
··x····························· MCM-MetaConceptModel.adoc
·x······························ memo-20260612-federation-legs-spike-findings.md

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 218 commits)

  1 O quotabuild-payor-regroup
  2 Q keyless-canonical-bootstrap
  3 Z freehold-collapse-cleanup
  4 e rbra-estate-demolition
  5 f accessor-don-rewire
  6 g assize-duration-12h
  7 P federation-narrative-broadside
  8 N fable-heat-review
  9 h fable-sequence-retrospective

123456789abcdefghijklmnopqrstuvwxyz
···xx······························  O  2c
·····xx····························  Q  2c
·······xxx·························  Z  3c
··········xxx······xxx·············  e  6c
··············x·x··················  f  2c
·················xx················  g  2c
······················xx···········  P  2c
····························xx·····  N  2c
·································xx  h  2c
```

## Steeplechase

### 2026-07-01 21:29 - ₢BZAAh - W

Fable sequence retrospective executed; findings landed in Memos/memo-20260701-BZ-sequence-retrospective.md — the four-phase arc characterized from the 215-commit affiliation record, nine forward-looking planning lessons (movement-grain planning with just-in-time wave slating, two-sentence placeholder minimum, spike-within-hours, structured thaw for freezes, named revisit triggers, critical-path split-vs-absorb rule, proof-pace lifecycle, deference/challenge channel separation, traffic-tiered mint review), the standing manor-identity provenance question answered (confabulation-dominant; cite-or-flag rule proposed), and five durable-doctrine flags pointed at JJK/MCM spec homes

### 2026-07-01 21:27 - ₢BZAAh - n

Fable sequence retrospective of heat ₣BZ: characterized the four-phase arc (gestation / conversion-day / design-day / execution) from the 215-commit affiliation record; found the movement skeleton was the only plan artifact that never needed repair while half the pace roster accreted just-in-time; confirmed the freeze/membrane ran bidirectionally (17 divergences zero overturns, plus the one genuine Fable confabulation caught and overturned); nine forward-looking planning lessons (movement-grain planning, two-sentence placeholder minimum, spike-within-hours, structured thaw for freezes, named revisit triggers, critical-path split-vs-absorb rule, proof-pace lifecycle, deference/challenge channel separation, traffic-tiered mint review); answered the standing manor-identity provenance question (confabulation-dominant; cite-or-flag rule for identity-recasting curries); five durable-doctrine flags pointed at JJK/MCM homes

### 2026-07-01 21:13 - Heat - f

racing

### 2026-06-26 10:56 - Heat - f

silks=rbk-42-fbl-federation-create

### 2026-06-26 07:16 - Heat - S

fable-sequence-retrospective

### 2026-06-22 07:59 - ₢BZAAN - W

Executed the terminal Fable review of ₣BZ under operator sanction (Fable unavailable), across five trotted chunks. Ratified the heat's build sound — 17 recorded Opus/Fable divergences, ZERO overturns. Resolved manor-identity to the composite-holding model (Manor = a payor/manor project + clustered resources + a commanded organization; the org-level 'deed' sense overturned as confabulated). Reconciled the freehold doctrine (only foedus-lifecycle is a built fixture; undelete declined for refuse-and-rotate; three genuine BF residuals) and the terrier homing (Option B — Manor founds it; operator cinched the post-payor-guide manor-setup finisher). Sanity-checked the M4-vs-M7 suite-gate (resolved early by the keyless-policy rewrite). Lifted the paddock freeze with the record corrections applied in place. Stood up the standing Fable-review-queue memo (provisional, eviction-sweepable mints), and slated the follow-on work: canon-scrub, terrier-finisher, and terminal-Fable-review paces in BF; the onboarding-keyless gap in A6; and a skimpy theurge RBS0-subdoc pace in new heat BL. Pre-wrap audit verdict: safe to wrap.

### 2026-06-22 07:42 - ₢BZAAN - n

Stand up the Fable-review-queue memo (standing terminal-review agenda, operator-sanctioned in Fable's absence) and capture the RBST0 theurge-test-cosmology-spec itch — products of the BZAAN terminal review

### 2026-06-22 07:35 - Heat - d

paddock curried: operator-sanctioned freeze lift + record corrections (terminal Fable review executed under operator sanction): manor-identity resolved to composite-holding, deed-sense overturned, terrier at-levy corrected to interim-scaffold

### 2026-06-22 04:49 - Heat - f

stabled

### 2026-06-22 04:49 - Heat - d

batch: 1 reslate

### 2026-06-22 04:45 - Heat - f

silks=rbk-11-mvp-FABLE-federation-create

### 2026-06-22 01:06 - ₢BZAAP - W

Inverted the README federation narrative (M6): federation is now THE credential model — keyfile gone, two-tier framing collapsed. Lifted the federation story + diagram trio from the Roadmap into the Foundry body; rewrote the role/auth table, the credential paragraph (+ the honest GCP-org / external-OIDC-IdP founding prerequisite), Establishment (affiance -> levy raises the mantle SAs -> gird the first governor) and Admission/Access (brevet / compear / don). Registered the federation Identity-and-Admission vocabulary on the broadside (mantle, citizen, compear, assize, don, affiance, jilt, gird, brevet, unseat, attaint, rehearse, terrier, muniment, census); retired the RBRA entry, kept RBRO, enrobe/defrock barred; JWT-Probe -> CheckFederatedAccess/CheckMantleDon; swept residual keyfile phrasing. Diagram sources: deleted rbdgk (keyfile-login), rewrote rbdgm (mode-enum gone, single federation flow incl. the Leg-3 don), de-Tiered rbdgl/rbdgs titles, fixed rbdgs's stale mode=federation->RBRD to federation-facts->RBRF. Verified by a multi-agent pass: zero keyfile residue, all 15 glossary defs spec-accurate, anchor-link integrity clean, 3-reader cold-probe confirms first-contact actionability. Committed in 28ed94a0a. OUTSTANDING (done-when 3b): the pluml SVG re-render is NOT done — blocked on an interactive compearance (this headless session has no TTY for the device flow), so rbdgl/rbdgm/rbdgs .svg renders are stale and the README still displays the pre-rewrite federation-seam diagram until someone runs tt/rbw-tf.FixtureRun.sh pluml after a compearance (tree is clean, so only the device-flow click gates it). Routed to Fable's terminal review: gird/affiance/jilt headword-obliqueness (cold-probe consensus; M6 already flagged gird), and the pre-three-leg-model internals of rbdgl/rbdgs (bounded fixes landed, deeper flow rewrites deferred).

### 2026-06-22 01:02 - ₢BZAAP - n

Invert the README federation narrative (M6 of office-federation): federation is now THE credential model, keyfile is gone, the two-tier framing collapses to one. Lift the federation story out of the Roadmap into the Foundry body — rewrite the role/auth table (governor/director/retriever now authenticate via Federated sign-in -> Mantle), replace the two-tier paragraph with the federation model plus the honest GCP-org + external-OIDC-IdP founding prerequisite, rework Establishment (affiance -> levy raises the three mantle SAs -> gird the first governor) and Credential Distribution -> Admission and Access (brevet/compear/don). Replace the keyfile-verb glossary (Mantle-verb/Invest/Divest/Roster) with the federation Identity-and-Admission vocabulary (Mantle-noun, Citizen, Compear, Assize, Don, Affiance, Jilt, Gird, Brevet, Unseat, Attaint, Rehearse, Terrier, Muniment, Census); retire the RBRA regime entry; keep RBRO; replace the JWT-Probe diagnostic with CheckFederatedAccess/CheckMantleDon; sweep residual keyfile phrasing (Governor 'creates service accounts', ccyolo, Eventual-Consistency Divest reference). Diagrams: delete rbdgk keyfile-login (puml + both svg); rewrite rbdgm federation-seam dropping the canceled mode-enum (single federation flow incl. the Leg-3 don, RBRA/GOOG participants removed); drop 'Tier' from rbdgl/rbdgs titles; fix rbdgs's stale 'mode=federation -> RBRD' to 'federation facts -> RBRF'. Verified: anchor-link integrity clean (no dangling #Invest/#Divest/#Roster/#JWTProbe/#OperatorFederation); a multi-agent verify pass found zero keyfile residue, all 15 glossary definitions spec-accurate, and a 3-reader cold-probe confirmed the entries first-contact actionable (gird/affiance/jilt oblique on the bare headword but rescued by the gloss -> routed to Fable's terminal review). PENDING: the pluml SVG re-render is blocked on an interactive compearance (this headless session has no TTY for the device flow); rbdgl/rbdgm/rbdgs .svg renders are stale until re-rendered via tt/rbw-tf.FixtureRun.sh pluml on a clean tree.

### 2026-06-21 15:44 - ₢BZAAe - W

Demolished the orphaned RBRA keyfile estate across bash, theurge-Rust, and specs (the bridge's demolition condition, executed). Bash: removed the caller-less JWT mint + rba RBRA load/extract + rbra regime/cli/tabtargets + the rbra constants/enum/assay; collapsed rbdc to payor-RBRO. Handbook (operator: demolish here): deleted the keyfile cred-onboarding tracks + RBYC_RBRA and stripped the four director-tracks' keyfile-readiness probes (federation compearance is just-in-time at the accessor). Rust: dropped the attest no-RBRA check, freehold-rbra helper, poison case, and dogfight/onboarding presence-probes; rebuilt theurge regenerates consts+context. Specs (operator: full Opus reframe): deleted RBSRA + the RBS0 RBRA quoin estate and redefined the three rbtoe_*_authenticate patterns from keyfile-JWT to the federation compear+don model, so the ~20 referencing operation specs inherit federation semantics; reframed RBSRR/RBSAP/RBSGS/RBSHR/RBSIP/BUS0. Deferred to owners: RBSCIG/RBSCIP RBRA + the orphaned RBGC_SA_KEY_*_RETRY constants -> Fable's IAM-propagation reanchor (M7 worklist updated); RBSHR/RBSGS narrative inversion -> A6 (de-dangled only). Green: theurge build, shellcheck 212, fast-qualify, fast suite 106/0, regime-poison 31/0, both grep gates (RBRA_FILE, rbgo_get_token_capture) clean; credless guard still fires via rba_compear. Close-time complete + live gauntlet/skirmish on federation personas remain per the docket.

### 2026-06-21 15:42 - ₢BZAAe - n

Scrub the last two stale references to the deleted keyfile membrane: rbtdro_onboarding probe doc (governor RBRA -> governor compearance) and the theurge-ifrit context doc's credless-guard membrane list (rbgo_get_token_capture -> rba_compear, the live federation chokepoint). grep rbgo_get_token_capture now lands clean repo-wide, completing the demolition's done-when grep gates.

### 2026-06-21 15:40 - ₢BZAAe - n

Demolish the orphaned RBRA keyfile estate across bash, theurge-Rust, and specs (the bridge's demolition condition, executed). Bash: strip the caller-less JWT mint (rbgo_get_token_capture + zrbgo build/exchange/base64url + JWT temp consts) keeping rbgo base64/docker-login/curl-predicate; delete rba_extract_json_to_rbra + rba_rbra_load (keep rba_rbro_load); delete rbra_cli.sh + rbra_regime.sh + the rbw-rar/rav/ral tabtargets + zipper enrollments; collapse rbdc_derived to the payor RBRO path only; drop RBCC_rbra_file, the rbnae_ role enum (RBCC_role_*), and RBCC_account_assay from rbcc_constants + emit list; strip the marshal-zero RBRA preview/delete blocks in rblm_cli. Handbook (operator ruling: demolish here): delete the keyfile-onboarding cred tracks (rbhocr/rbhocd + rbw-Ocr/Ocd + zrbho_credential_install + RBYC_RBRA term), excise the start-here credential subtracks and crash-course auth row, and strip the now-obsolete director-keyfile readiness probes from the four director tracks (federation compearance is just-in-time at the accessor). Rust: drop the attest Class-C no-RBRA check + RBTDRP_RBRA_ROLES, the rbtdrk_freehold_rbra helper, the poison rbra-bad-private-key case, the dogfight/onboarding governor/director-RBRA presence probes (readiness deferred to Bf), and refresh the fast/manifest/handbook fixtures; rebuilt theurge regenerates rbtdgc_consts + tabtarget-context (AUTH/ASSAY/RBRA_FILE/CRED consts gone). Specs (operator ruling: full Opus reframe): delete RBSRA-CredentialFormat.adoc + the RBS0 RBRA quoin estate (rbra_regime/RBRA_* env, rbrr_*_rbra_file, rbtgi_rbra_file, rbtoe_rbra_generate/load/jwt_oauth_exchange), redefine the three rbtoe_*_authenticate patterns from keyfile-JWT to the federation compear+don model so the ~20 referencing operation specs inherit federation semantics, reframe the role-model prose + RBSRR/RBSAP/RBSGS/RBSHR/RBSIP referrers, and drop the BUS0 rbnae example + the RBSRA acronym entry (RBSCIG/RBSCIP RBRA mentions left behind FABLE-REANCHOR for the deferred IAM-propagation reconciliation). Memo: appended the now-orphaned RBGC_SA_KEY_*_RETRY constants (consumerless, gated on the RBSCIG/RBSCIP reanchor) to the M7 Fable terminal-review worklist. Green: theurge build, shellcheck 212 clean, fast-qualify (consts+context fresh). grep RBRA_FILE and grep rbgo_get_token_capture land clean repo-wide.

### 2026-06-21 14:47 - ₢BZAAg - W

Assize cap raised 1h -> 12h. RBRF_SESSION_DURATION 3600s -> 43200s in rbrf.env; standing test-trust pool spike-office-test PATCHed to 43200s (live re-read confirms ACTIVE) via the kit's own RBRO refresh-grant -> payor OAuth -> IAM workforcePools PATCH path, no committed bash. rbw-acf 12h-expiry verification deferred per operator (needs a fresh human compearance; existing assizes keep the old 1h cap).

### 2026-06-21 14:46 - ₢BZAAg - n

Bump the assize cap from 1h to 12h. Regime: RBRF_SESSION_DURATION 3600s -> 43200s (the pool sessionDuration ceiling), so a fresh affiance provisions 12h. Live pool: one-shot PATCH of the standing test-trust pool spike-office-test sessionDuration 3600s -> 43200s via the kit's own auth path (RBRO refresh-grant -> payor OAuth -> IAM workforcePools PATCH, mirroring zrbgp_refresh_capture); re-read confirms 43200s ACTIVE. No committed bash touched (affiance stays ensure-exists-only; reconciling sessionDuration drift is affiance-evolution routed to Bf). Only a fresh compearance picks up the 12h cap; rbw-acf verification deferred (needs a human device-flow click, not run per operator).

### 2026-06-21 13:07 - ₢BZAAf - W

Wired the production credential accessor onto the federation don — proven live on federation personas. rba_token_capture now validates identity, compears (ensure-assize), then dons the matching mantle SA via rba_don_capture, retiring its keyfile JWT mint; rbgo_get_token_capture is now caller-less. Swept the keyfile reads across the foundry/lode/bottle surface: 9 vestigial `source RBDC_*_RBRA_FILE` lines (rbfd x3, rbfv x4, rbldb/r/v/w) and ~17 `test -f RBRA_FILE` precondition guards (the don's assize + admission checks supersede them); reconciled rbldd_delete's Cloud Build run-as identity onto director mantle-SA derivation (RBCC_account_mantle_director) replacing the keyfile RBRA_CLIENT_EMAIL extraction. Re-kindled the 8 minting CLIs (rbfc0/rbfd/rbfl0/rbfr/rbfv/rbgg/rbld0/rbob) with the rbrf federation regime (source rbrf_regime.sh + RBCC_rbrf_file + zrbrf_kindle + unconditional zrbrf_enforce), matching the rbgp/rbgv shape; rbfk/rbhp0 stay keyfile-only (verified non-minting). RBRA estate (constants/regime/RBSAP prose, dead rbgo_get_token_capture) deliberately left standing for the successor teardown BZAAe. 24-file commit 6b65d15b4. Static-green: shellcheck 216 clean, fast-qualify, fast suite 108/108 incl. the credless-guard mint refusal surviving the compear fold. Live-green on the standing federation trust: dogfight (ordain->summon->run->abjure, 10:19) and the full service tier (20 fixtures, 150 passed, 0 failed) drove the rewritten accessor through foundry ordain+vouch, lode capture (conclave's 6-tool cohort), reliquary/wsl/podvm captures, batch-vouch, and terrier on the rbma-* mantles — plus the assize-lapse re-compear path exercised live. Surfaced two follow-ups: the assize 1h->12h bump (cantled as the next pace) and the duration-aware-admission named-deferral trigger (routed to Bf).

### 2026-06-21 12:49 - Heat - S

assize-duration-12h

### 2026-06-21 11:31 - ₢BZAAf - n

Wire the production credential accessor onto the federation don (the movement-4 accessor branch). rba_token_capture now validates identity, compears (rba_compear, ensure-assize), then dons the matching mantle SA (rba_don_capture) — retiring its keyfile JWT mint; rbgo_get_token_capture now has zero live callers. Stripped 9 vestigial `source RBDC_*_RBRA_FILE` lines (rbfd x3, rbfv x4, rbldb/r/v/w) and ~17 `test -f RBRA_FILE` keyfile precondition guards across the foundry/lode/bottle surface — the don's assize + admission checks supersede them. Reconciled rbldd_delete's Cloud Build run-as identity onto director mantle-SA derivation (RBCC_account_mantle_director@DEPOT.iam) replacing the keyfile RBRA_CLIENT_EMAIL subshell extraction (live-run-provable: mantle SA needs build actAs). Added rbrf federation regime (source rbrf_regime.sh + RBCC_rbrf_file + zrbrf_kindle + unconditional zrbrf_enforce) to the 8 minting CLIs (rbfc0/rbfd/rbfl0/rbfr/rbfv/rbgg/rbld0/rbob), matching the rbgp/rbgv shape; rbfk/rbhp0 stay keyfile-only (verified non-minting). RBRA estate (constants/regime/RBSAP prose, dead rbgo_get_token_capture) left standing for the successor teardown per the done-when. Green: shellcheck 216 clean, fast-qualify.

### 2026-06-21 10:46 - Heat - S

accessor-don-rewire

### 2026-06-21 10:32 - ₢BZAAe - n

Demolish the keyfile JWT access-probes (M7 Task 2 of the office-federation RBRA estate demolition). The federated rbw-acm (mantle don) and rbw-acf (compearance) already replace the governor/director/retriever JWT probes; the payor OAuth probe rbw-acp survives. Code: deleted tabtargets rbw-acg/acr/acd + their zipper enrollments (RBZ_CHECK_GOVERNOR/RETRIEVER/DIRECTOR); stripped the JWT-probe bodies from rbgv_probe.sh (rbgv_jwt_sa_probe, zrbgv_jwt_ar_probe_once_capture, the JWT-only zrbgv_jq_error_message_capture) and rbgv_cli.sh (zrbgv_jwt_check + the three rbgv_check_{governor,retriever,director} dispatch wrappers), keeping the payor/compearance/mantle surface and the shared helpers (zrbgv_ms_to_sleep_capture, zrbgv_http_get_with_5xx_retry, the mantle/payor capture pair). Manifest: rbtdrm_credential_check_colophon collapsed to the single payor row; access-probe fixture reduced to its one surviving case (oauth_payor) across RBTDRC_CASES_ACCESS_PROBE and the RBTDRM_FIXTURE_ACCESS_PROBE colophon list, now-unused RBTDGC_ACCOUNT_{GOVERNOR,DIRECTOR,RETRIEVER} imports trimmed from rbtdrc_crucible.rs. Spec: deleted RBSAJ-access_jwt_probe.adoc whole + its RBS0 mapping quoin, the rbtgo_access_jwt_probe definition, and the now-empty == Multi Role Operations section header (pure-dead, no marker); removed the stale RBSAJ acronym-map entry. Two surviving-spec evidence citations of the deleted JWT probe (RBSCIG class-C invalid_grant harness, RBSCIP class-C model) unlinked to past-tense plain text + FABLE-REANCHOR markers, enumerated in the M7 worklist. Theurge rebuilt (rbtdgc_consts.rs drops the three RBTDGC_CHECK_* consts; tabtarget-context re-derived); 159 unit tests, shellcheck 216 clean, fast-qualify green.

### 2026-06-21 10:13 - ₢BZAAe - n

Demolish the cult-verb estate spec half (M7 office-federation), completing the BZAAe cult-verb-estate demolition. Deleted the 7 cult-triad spec files (RBSDK/RBSRK enrobe, RBSDD/RBSRD defrock, RBSDR/RBSRL roster, RBSGM governor_enrobe). RBS0: removed their mapping-section quoins, the verb-definition subsections + includes, and the rbsk_enrobe_serialized assay premise; restored the == {rbtr_governor} Operations section header (deleted with the cult block, now holding gird + the federation admission verbs) with a FABLE-REANCHOR marker on its framing. Unlinked or re-pointed every surviving cult-quoin reference repo-wide: clean gird re-points where a successor exists (RBSMF's two post-levy governor-enrobe hints + RBSGS's governor-creation passage -> rbtgo_governor_gird; gird-prose/rbtf_gird successor refs -> plain text); FABLE-REANCHOR deferrals where federation IAM-model reframing is Fable's (RBS0 director workerPoolUser grant -> levy; RBSCIG revoke-contract + retry-list; RBSCIP keyfile-churn race bullets -> flap relocates to levy/unmake; RBSMF cache-lag; RBSGS getting-started credential-admin + rotation/revocation -> A6). M7 memo record appended: as-built + the full Fable terminal-review worklist the markers point to, plus the A6-inherited onboarding remnants. Verified: every cult-triad quoin gone repo-wide; rbtgo_governor_gird attr defined so re-points resolve; dangling-attribute sweep across RBS0 + includes shows zero new dangling refs (only pre-existing MCM _s variants + code placeholders). Build unaffected (specs not compiled).

### 2026-06-21 09:41 - ₢BZAAe - n

Demolish the keyfile cult-verb estate (code half of M7 office-federation). Bash: delete enrobe/defrock/roster verbs (rbgg bodies + zrbgg_create_service_account_with_key key engine + 22 orphaned ZRBGG infixes + secrets_json helper), rbgp_enrobe_governor, the rbw-a Accounts colophon group + its 7 tabtargets, and the whole keyfile governor onboarding handbook (rbhogw + rbw-Og + start-here/cli wiring) + rbyc enrobe/defrock/roster terms. Repoint the surviving payor handbook + post-levy hint from enrobe-governor to gird (rbw-pE federation successor). Rust: delete the freehold-enrobe fixture (rbtdrk_enrobe whole) and the depot-lifecycle sa_cycle case; strip freehold-enrobe from gauntlet/skirmish/dogfight/blockade (federation credential-readiness deferred to Bf, recorded in its paddock); fix the freehold-establish manifest colophon list to the federation personas gird/brevet/check-mantle/compearance (the BZAAZ handed-down finding); drop the governor-hb handbook-render case. Roster verbs retire with the triad (mount decision); governor handbook demolished whole (operator ruling). theurge build (consts + tabtarget-context regenerated), 159 unit tests, shellcheck 216 clean, fast-qualify all green. Spec sub-unit (cult-triad specs RBSDK/RBSRK/RBSDD/RBSRD/RBSDR/RBSRL/RBSGM, RBS0 seats, RBSGS, surviving-spec FABLE-REANCHOR markers) still pending.

### 2026-06-20 15:46 - ₢BZAAZ - W

Collapsed the two depot test-prefix schemes (canonical + pristine) into one freehold scheme. Unified the duplicated autoincrement pick-next and ~9 env/git helpers into a single home (rbtdrk_freehold); both fixture families now ride it. Operator option A: kept the deployed prefix/stem VALUES (canc/canr/canest3) as opaque strings so the live standing freehold keeps working with zero churn — only Rust identifiers, fixtures, and the depot_levy fn (->freehold_ensure) took freehold vocabulary. Fixtures: canonical-establish->freehold-establish, canonical-enrobe->freehold-enrobe, canonical-churn->freehold-churn, pristine-lifecycle->depot-lifecycle (kept in the gauntlet as the ephemeral create->destroy proof; pick_next's max+1 guarantees it tears down only the fresh leasehold it mints, never the standing freehold — the safety cinch holds by mechanism, not disjoint prefixes). RCG-clean split (every file <800): rbtdrk_canonical.rs->freehold/depot/enrobe (enrobe isolated whole as a clean keyfile-delete target), rbtdrp_pristine.rs->attest(gate)/lifecycle(arc). Fixed a stale unit test the keyless-bootstrap pace left red (rbtdtk_cases_registered: 5 enrobe cases -> 6 federation cases). All cross-refs updated (crucible suites, dogfight/onboarding, manifest, RBSDL, CLAUDE.md). Green: build, 159 unit tests, shellcheck 217. Live-suite verification (gauntlet/skirmish on federation personas, which complete-green omits) handed to the demolition pace, whose docket was reslated to direct it; two BZAAQ-era findings (manifest colophon list; onboarding governor-RBRA precondition) captured in the M7 memo record for that mount agent.

### 2026-06-20 15:45 - ₢BZAAZ - n

Capture the freehold-collapse as-built state and two BZAAQ-era correctness findings in the M7 record, for the estate-demolition mount agent: the manifest's freehold-establish colophon list still names keyfile enrobe colophons rather than the federation ones the cases now invoke (harmless existence-check, reconcile when sweeping keyfile colophons); onboarding's governor-RBRA precondition expects a key file keyless federation establish no longer writes (the demolition's gauntlet run forces it open). Both left untouched by the collapse as out-of-scope; the demolition's gauntlet/skirmish verification (added to its docket this session) is where they bite.

### 2026-06-20 15:29 - ₢BZAAZ - n

Collapse the two depot test-prefix schemes (canonical + pristine) into one freehold scheme. The duplicated autoincrement pick-next and ~9 env/git helpers unify into a single home, rbtdrk_freehold.rs; both fixture families now ride it. Operator decision (option A): keep the deployed prefix/stem VALUES (canc/canr/canest3) as opaque strings so the live standing freehold keeps working with zero churn — only the Rust identifiers, fixtures, and the depot_levy fn rename to freehold vocabulary. Fixtures: canonical-establish->freehold-establish, canonical-enrobe->freehold-enrobe (bridge-legacy keyfile estate, isolated in rbtdrk_enrobe for clean later demolition), canonical-churn->freehold-churn, pristine-lifecycle->depot-lifecycle (the ephemeral create->destroy proof, kept in the gauntlet; pick_next's max+1 guarantees it tears down only the fresh leasehold it mints, never the standing freehold — so the safety cinch holds by mechanism, not disjoint prefixes). The depot_levy reuse-or-levy fn -> rbtdrk_freehold_ensure. File splits keep every file under the RCG 800 line threshold: rbtdrk_canonical.rs (1255) -> rbtdrk_freehold (machinery+probes) / rbtdrk_depot (keyless establish+churn) / rbtdrk_enrobe (keyfile estate); rbtdrp_pristine.rs (1320) -> rbtdrp_attest (marshal-zero gate) / rbtdrp_lifecycle (the arc). Corrected a stale unit test the keyless-bootstrap pace left red: rbtdtk_cases_registered asserted the old 5 enrobe-based establish cases; now asserts the 6 federation-persona cases. Crucible suite registrations + comments, dogfight/onboarding cross-refs, manifest fixture-name consts + arms, RBSDL spec, and the CLAUDE.md release-suite table all updated to freehold vocabulary. Green: theurge build, 159 unit tests, shellcheck 217 clean.

### 2026-06-20 12:25 - ₢BZAAQ - W

Minted the payor-wielded founding verb gird (rbw-pE / RBSPG) — the chicken-and-egg the 'verbs already enrolled' premise missed (brevet wields a donned governor, so it cannot seat the FIRST governor on a fresh depot) — and rewrote canonical-establish onto federation personas: levy -> compear -> gird governor -> brevet+don director -> brevet+don retriever -> recognosce. Proven 6/6 live on the no-keys org (beta), where the old keyfile governor-enrobe 400'd ('Key creation is not allowed on this service account'). RBS0 quoins rbtf_gird/rbtgo_governor_gird minted, RBSPG authored and included, acronym map updated, theurge regenerated; credless-green (build, shellcheck 217 clean, quoin-resolution 19 refs+anchors+include, fast-qualify). Done-when reslated to the delivered scope: the full-gauntlet complete-green and the keyfile-suite-green belong to the RBRA-estate-demolition movement, not this pace — its accessor keyfile-branch retirement (rba_token_capture) is the gauntlet's downstream blocker, surfacing first at hallmark-lifecycle. canonical-enrobe (skirmish/dogfight/blockade) deliberately untouched on keyfile.

### 2026-06-20 11:58 - ₢BZAAQ - n

Mint the payor-wielded founding verb gird (colophon rbw-pE, contract RBSPG) and rewrite the gauntlet canonical-establish fixture onto federation personas. gird seats the first governor without a key — the founding admission the paddock cinched but never built — driving the shared zrbgp_brevet_core with the payor's OAuth token (rbgp_gird in rbgp_payor.sh, dispatched via rbgp_cli, governor-only/single-target). canonical-establish's cases become levy -> compear -> gird governor -> brevet+don director -> brevet+don retriever -> recognosce, replacing the keyfile governor/retriever/director enrobe + JWT-probe cases that 400 under the org's disableServiceAccountKeyCreation (live-reproduced this session: 'Key creation is not allowed on this service account'). canonical-enrobe keeps the keyfile cases for skirmish/dogfight/blockade. RBS0 quoins rbtf_gird + rbtgo_governor_gird minted, RBSPG authored and included, acronym map updated; theurge rebuilt so rbtdgc_consts.rs (RBTDGC_GIRD_POLITY) and the tabtarget-context re-derive. gird mint recorded in the pace-design memo and flagged for Fable's terminal review per the jilt precedent. Credless-green: build, shellcheck (217 clean), quoin-resolution (19 refs + anchors + include), fast-qualify. Live federation-persona verification pending (needs the no-keys org + a live assize).

### 2026-06-20 09:59 - ₢BZAAO - W

Regrouped the orphan rbw-gq (QuotaBuild) colophon under the Payor guide subfamily as rbw-gPQ. One zipper-enrollment colophon-string edit + tabtarget rename, theurge rebuilt so rbtdgc_consts.rs and claude-rbk-tabtarget-context.md re-derived; rbhp_quota_build function and RBSQB/RBS0 refs unchanged, RBZ_QUOTA_BUILD const left as concept-keyed (out of the gq->gPQ-only cinched scope). Manor-demesne (levy/establish) consolidation deliberately not undertaken — routed to the terminal Fable review per the cinch. Grep gate clean on the operative axis: only provenance mentions remain in memo-20260615-BZ-pace-design.md, left intact per operator. Fast-qualify green.

### 2026-06-20 09:58 - ₢BZAAO - n

Regroup the orphan rbw-gq (QuotaBuild) colophon under the Payor guide subfamily as rbw-gPQ. Zipper enrollment colophon string updated (RBZ_QUOTA_BUILD now rbw-gPQ, joining gPI/gPE/gPR), tabtarget renamed tt/rbw-gq.QuotaBuild.sh -> tt/rbw-gPQ.QuotaBuild.sh; theurge rebuilt so the generated rbtdgc_consts.rs (RBTDGC_QUOTA_BUILD) and claude-rbk-tabtarget-context.md re-derive. Function rbhp_quota_build and the RBSQB/RBS0 references key on the function, not the colophon, so unchanged; const name RBZ_QUOTA_BUILD keys on concept, out of the gq->gPQ-only cinched scope. memo-20260615-BZ-pace-design.md rbw-gq mentions left intact as provenance per operator. Fast-qualify green.

### 2026-06-20 09:12 - Heat - r

moved BZAAP after BZAAe

### 2026-06-20 09:12 - Heat - T

federation-narrative-broadside

### 2026-06-20 09:11 - Heat - n

Record the M6 narrative-docs groom: recovered intent (federation narrative-docs movement = README story + broadside glossary registration of the elected words + cold-probe; contract specs and RBS0 quoins were M4, handbook rework is A6), the as-built inversion finding (the README still frames keyfile as today's whole system and federation as a planned future tier — M6 inverts it to federation-as-model, two tiers collapse to one), three sharper traps (the broadside mantle homonym: glossary 'Mantle = create Governor SA' cult-verb sense must flip to the federation standing-role noun; the federation-seam diagram's stale 'mode-enum branch' caption since the mode enum was canceled; RBRA glossary entry + rbdgk keyfile-login diagram retire while RBRO stays), the one-coherent-pace scope, and the operator-accepted sequencing fix (rail M6 to after the estate demolition so the narrative describes the real keyless state: keyless bootstrap -> freehold collapse -> estate demolition -> narrative docs -> terminal Fable review).

### 2026-06-20 09:03 - Heat - S

rbra-estate-demolition

### 2026-06-20 09:03 - Heat - T

keyless-canonical-bootstrap

### 2026-06-20 09:02 - Heat - n

Record the M7 RBRA-estate-retirement groom: recovered intent (retire the keyfile estate whole, gated on 'federation personas pass the suites'), the live-blocker reconsideration (the org's disableServiceAccountKeyCreation broke keyfile fresh-levy enrobe — the M4/M7 review's named revisit trigger firing from an external policy change, so the keyless canonical rewrite is forced now and building it MEETS the gate, retiring the review's recorded residual risk early), the 260620 Explore estate map (mint localized to rba_token_capture; ~20-file guard sweep; five sized clusters a-e; rbgw capability-sets stay, roster-verb fate a Pace-2 decision), and the operator-accepted 2-pace decomposition (Pace 1 keyless canonical bootstrap = reslated BZAAQ, meets the gate; Pace 2 estate demolition whole = new pace after the freehold-collapse pace) with the freehold-collapse sequencing (disjoint fixture regions, compose; order keyless-bootstrap -> freehold-collapse -> demolition -> terminal Fable review).

### 2026-06-20 08:43 - Heat - n

Record the M5 manor-colophon-regroup groom finding: recover Fable's original intent (consolidate the manor demesne Levy/Establish/Affiance/Jilt into one colophon family, currently scattered across rbw-d/rbw-gP/rbw-m; RBSMF tags levy 'Manor demesne', the vocabulary elections cinch L/E/A/J as sibling-initials), surface the which-axis tension the one-liner hid (levy is both manor-demesne and depot-create; establish is both manor-demesne and payor-guide; a colophon family encodes one axis only), and the operator-accepted decision: M5's reslated pace BZAAO executes only the uncontested rbw-gq->rbw-gPQ kernel, the contested levy/establish consolidation routed to the terminal Fable review (BZAAN) as an explicit divergence — contested, costlier now that levy is deeply wired, purely cosmetic, and the family-coherence call the terminal review owns, while the tail's real urgency is M7's live keyless-rewrite blocker.

### 2026-06-20 08:42 - Heat - T

quotabuild-payor-regroup

### 2026-06-20 07:56 - ₢BZAAd - W

Retired the two faked-don payor-credentialed proofs (admission-proof, foedus-freehold) as spent construction scaffolding — verbs + exclusive IAM-readback helpers, rbw-pP/rbw-pF tabtargets + zipper enrollments, rbgp_cli enforce alternatives, and the theurge fixtures (bodies, cases, registry, service+complete membership, manifest). Kept terrier-atomicity and rbw-dT as the one honest, credential-path-agnostic survivor (GCS 412/404 muniment atomicity). Path (b), chosen in conversation: the payor-proofs tested only the admission write-side via the payor's owner authority, bypassing the compear->don->act runtime path that is the federation thesis; the authentic compearance-driven green target and the reserved suite word 'parley' deferred to the federation-evolution heat, with admission-proof's suspension-vs-erasure coverage recorded as a debt owed there (jji_itch.md). Build, 162 theurge tests, fast-qualify, shellcheck (217 files) all green.

### 2026-06-20 07:48 - ₢BZAAd - n

Retire the two faked-don payor-credentialed proofs (admission-proof, foedus-freehold) as spent construction scaffolding; keep terrier-atomicity as the one honest, credential-path-agnostic survivor. Path (b), chosen in conversation: the payor-proofs tested only the admission write-side via the payor's owner authority, deliberately bypassing the compear->don->act runtime path that is the federation thesis — a reasonable construction-time scaffold now spent; enshrining them as a named green target would report 'federation works' when it means 'the plumbing composes under payor authority.' Removed: verbs rbgp_admission_proof/rbgp_freehold_proof plus their three exclusive IAM-readback helpers (zrbgp_proof_forbid_binding/_fetch_sa_policy/_fetch_project_policy) in rbgp_payor.sh; the rbw-pP/rbw-pF tabtargets and their zipper enrollments; the two federation-enforce alternatives in rbgp_cli; the admission-proof/foedus-freehold theurge fixtures (bodies, cases, RBTDRC_FIXTURES registry, service+complete suite membership, manifest consts + dep-arms); the RBGP acronym mentions. Kept untouched: terrier-atomicity, the terrier scaffold, and rbw-dT (terrier proof) — terrier-atomicity's GCS 412/404 muniment-atomicity claim is credential-path-agnostic. Generated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md re-derived by the build (PROOF_POLITY/FREEHOLD dropped, TERRIER_PROOF kept). The authentic compearance-driven federation green target — and the operator-reserved suite word 'parley' — deferred to the federation-evolution heat where runtime/compearance is already routed; the deleted admission-proof's suspension-vs-erasure coverage recorded as a debt owed there. Build, 162 theurge unit tests, fast-qualify, and shellcheck (217 files) all green. Two reminders captured in jji_itch.md: the operator-requested terrier-atomicity nature discussion (post-pace) and the deferred authentic-green-target work. Supersedes the original 'build parley suite + retire three proofs' docket.

### 2026-06-19 11:27 - ₢BZAAV - W

Federation attribution proven live, end-to-end: a brevetted citizen donned the retriever mantle, made an AR ListRepositories call under it, and the resulting Cloud Logging Data-Access entry carries serviceAccountDelegationInfo[].principalSubject = the freehold subject's Entra oid (the human, by immutable IdP claim). The mint-hop quirk observed and not mistaken for failure. Shipped the payor attribution-trail reader (rbw-da / rbgp_attribution_trail) reading entries:list with three honest cases (federated use-hop carries the subject / iamcredentials mint hop names only the SA / direct-SA use hop carries no delegation). Extended rbw-acm to make the attributed AR call under the donned token (new zrbgv_mantle_ar_call_capture); added Cloud Logging constants. Fixed a latent BZAAY bug the live run surfaced: acm read the mantle from positional $1 instead of the BUZ_FOLIO param1 channel. The flagged privateLogViewer risk closed itself (payor read the private log at HTTP 200). shellcheck 217 clean, build green, fast-qualify green. tty-removal/yawp follow-on parked as BfAAD.

### 2026-06-19 11:23 - ₢BZAAV - n

citizen-attribution-proof — proven live, end-to-end.

### 2026-06-19 10:22 - Heat - S

payor-cred-federation-suite

### 2026-06-19 10:20 - ₢BZAAc - W

foedus-freehold fixture + rbw-pF freehold-proof verb: built, qualified (build/shellcheck-217/fast-qualify green), and PROVEN LIVE end-to-end against the standing freehold. rbw-pF seeded the freehold subject onto all three mantles (muniment engrossed + breveted each) and asserted the manor-wide roster carries them; the full Rust fixture ran green (payor probe -> affiance no-op assert -> terrier charge -> freehold proof). Idempotent confirmed — the proof ran twice green (direct + via fixture), demonstrating the provision-once static-admission pattern. Done-when's 'service/complete stays green' clause is NOT directly demonstrable in this environment, and that is honestly a keyfile-estate breakage, not a foedus-freehold defect: service reds at access-probe on the broken bridge-legacy keyfile director (the org now enforces disableServiceAccountKeyCreation, so the keyfile director/retriever cannot be re-enrobed) — the ₢BZAAQ-owned collision, surfaced live here. foedus-freehold rides ABOVE the keyfile estate (payor-credentialed, self-skipping, Independent disposition), so its own verification is conclusive and it is non-regressing in-suite. A follow-on pace is slated to give the payor-credentialed federation fixtures a green-able sub-suite that excludes the keyfile estate. Separately, the rbw-ac* furnish regression that blocked the payor probe (rbgv_cli kindled rbgp without loading RBRP, latent since the terrier pace BZAAK) was fixed and notched standalone (9ee2ff6cc2).

### 2026-06-19 09:56 - Heat - n

Fix the rbw-ac* probe-family regression: rbgv_cli furnish kindles rbgp (rbgv needs zrbgp_authenticate_capture/zrbgp_sentinel) but never loaded RBRP values, so zrbgp_kindle's RBGP_TERRIER_BUCKET derivation — added by the terrier resource-layer pace BZAAK — died with 'RBRP_PAYOR_PROJECT_ID not set' before any probe ran, taking down every rbw-ac* probe (acp/acg/acr/acd/acf/acm). Source RBCC_rbrp_file in the furnish value-source batch alongside RBRR/RBRD so the furnish-time zrbgp_kindle finds the payor project; rbrp's kindle+enforce stay per-probe in rbgv_check_payor, since the payor probe enforces RBRP while the depot probes enforce RBRR/RBRD (opposite conditionality, load-bearing). Latent since BZAAK because the terrier/admission paces were never exercised through rbw-ac*; surfaced on the first live rbw-acp run. rbw-acp now passes 5/5 OAuth iterations; shellcheck 217 files clean.

### 2026-06-19 09:43 - ₢BZAAc - n

Build the foedus-freehold fixture + rbw-pF freehold-proof verb. New rbgp_freehold_proof (payor-credentialed, dispatched via rbgp_cli with rbpc_constants.sh sourced + the RBRF/RBRR/RBRD enforce arm): ensure-brevet the single freehold subject (RBPC_freehold_subject) onto all three mantles via the token-agnostic zrbgp_brevet_core (the admission-proof precedent — no compearance), then assert the manor-wide muniment roll carries each (mantle, subject) pair via rbgft_peruse_manor. Leaves the subject seeded — standing admission, idempotent re-runs, the provision-once static-admission doctrine — unlike admission_proof's throwaway-subject + attaint cleanup. New rbw-pF FreeholdProof colophon enrolled in the zipper (sibling to rbw-pP, transitional/payor-credentialed, uppercase because it ensure-writes), launcher byte-identical to rbw-pP; generated rbtdgc_consts.rs (RBTDGC_PROOF_FREEHOLD) + claude-rbk-tabtarget-context.md re-derived by the theurge build. New rbtdrc_foedus_freehold theurge fixture (service + complete member, self-skips credless): payor probe -> affiance the standing pool with NO regime-poison asserting the idempotent no-op (already-present + Manor-affianced terminal) -> charge terrier via rbw-dt -> run rbw-pF. Manifest fixture-name const + colophons-it-invokes entry; RBGP acronym entry names the freehold proof. Build green, shellcheck 217 clean, fast-qualify green. Not live-exercised this pace by design (needs service creds + levied depot + clean tree — static-admission doctrine); noted: affiance's bug_require_clean_tree gate couples the service/complete suites to a clean tree at this one fixture.

### 2026-06-19 09:14 - ₢BZAAY - W

Freehold rbpc substrate + don-mantle probe landed and qualified. New kindle-less rbpc_constants.sh homes the single durable freehold subject (operator Entra oid) as a tinder constant; rbpc_emit_consts is the third peer emit source in rbz_emit_consts, landing RBTDGC_FREEHOLD_SUBJECT in the generated rbtdgc_consts.rs; sourced in the three furnish sites that reach the emit or probe (rbq_cli/rbte_cli/rbgv_cli). New rbw-acm/CheckMantleDon probe (rbgv_check_mantle) resolves the freehold subject, compears, confirms the compeared identity (warn-only, best-effort), then dons a named mantle or surfaces the accessor's admission-deficit 403; depot-coupled. New zrba_assize_subject_capture read-only getter in the accessor. BCG-audited against the full guide (dropped a $()-pipeline and an if-VAR=$(capture) form); shellcheck 217 clean, fast-qualify green. Probe not live-exercised here by design (static-admission doctrine) — substrate auto-tested by foedus-freehold (₢BZAAc), live compear+don by the operator-paced attribution proof (₢BZAAV), headless automation deferred to ₣Bf.

### 2026-06-19 09:10 - ₢BZAAY - n

Build the rbpc freehold test-constants substrate + don-mantle probe. New segregated rbpc_constants.sh homes the single durable freehold subject (operator Entra oid) as a tinder constant (module deliberately kindle-less — pure tinder); rbpc_emit_consts wired as the third peer emit source in rbz_emit_consts, landing RBTDGC_FREEHOLD_SUBJECT in the generated rbtdgc_consts.rs; sourced in rbq_cli/rbte_cli/rbgv_cli (the three furnish sites that reach the emit or the probe). New rbw-acm/CheckMantleDon don-mantle probe (rbgv_check_mantle): resolves the freehold subject, compears, confirms the compeared identity (warn-only, best-effort per accessor), then dons the named mantle (governor|director|retriever) or surfaces the accessor's admission-deficit 403; depot-coupled via furnish RBRR/RBRD enforcement, unlike the depot-agnostic rbw-acf. New zrba_assize_subject_capture read-only getter in rba_auth.sh (the accessor is the sole assize-cache reader). RBPC acronym entry added; generated consts + tabtarget-context re-derived by the build. BCG-audited against the full guide: dropped a $()-pipeline in rbpc_emit_consts (emit direct for one const) and an if-VAR=$(capture) form in the probe (two-line capture + test-branch). Shellcheck 217 clean, fast-qualify green. Probe not live-exercised this pace (needs compearance + levied depot + brevetted subject — next-pace fixture work per docket Cinched).

### 2026-06-19 08:40 - Heat - n

Capture the static-admission / no-in-test-IAM-churn test-rig doctrine and the identity-layers model settled in conversation 260619, and record the resulting ₢BZAAY scope collapse (single freehold subject + don-mantle probe; multi-citizen roster and freehold depot moniker dropped). Negatives tested as never-granted standing states; revocation correctness by IAM read-back, never don-and-wait; grant-direction propagation absorbed once at freehold setup. Notes the ₣Bf coupling for inter-mantle negatives.

### 2026-06-19 07:45 - Heat - S

foedus-freehold

### 2026-06-19 07:42 - Heat - T

freehold-rbpc-substrate

### 2026-06-19 06:59 - ₢BZAAY - n

Affiance prerequisite reshaped + ACG constant homing for the foedus freehold. Affiance's soft-deleted-pool handling changes from undelete to fail-fast: clean-tree gate (bug_require_clean_tree) up front, then 404 create / 200-ACTIVE no-op / 200-DELETED refuse with a copy-paste sed bumping RBRF_WORKFORCE_POOL_ID — lifecycle certainty, a dissolved trust is never resurrected and a fresh trust takes a fresh name (no undelete, no commingled lifecycles). Home the nine bug_require_clean_tree operation labels in the RBCC_verb_* tinder family (7 new: affiance/conclave/conjure/ensconce/immure/mirror/underpin) and the GCP resource-lifecycle state enum in new RBGC_STATE_* kindle constants (ACTIVE/DELETED/STATE_UNSPECIFIED), sweeping both across affiance/jilt/project-state sites; kept out of the Rust projection (no consumer). Tighten the RBRF workforce-pool and provider id regex to GCP's real rule (letter-led, [a-z0-9-], no trailing hyphen). RBSMA updated to the verified 200/DELETED-refuse behavior. Shellcheck 216 clean; fast-qualify green.

### 2026-06-18 16:10 - ₢BZAAL - W

Built the operator-facing federation admission surface — brevet / unseat / attaint / rehearse — under the rbw-p polity launcher family. Substrate: tokenCreator + serviceUsageConsumer role constants and the principal:// SA-binding add/revoke pair (distinct canonical path per BCG). Manor-wide terrier read (rbgft_peruse_manor over a shared depot-blind lister, read-after-list 404 tolerated). Verb bodies in rbgp_payor.sh over token-agnostic zrbgp_*_core helpers wielded as a donned governor mantle (compear then don governor; cores stay token-agnostic so the levy founding-exception and the proof drive them payor-credentialed): brevet engrosses the muniment then ensures tokenCreator + first-admission serviceUsageConsumer; unseat expunges then revokes only tokenCreator (depot binding survives = suspension); attaint unseats every mantle then sweeps the depot binding; rehearse reads manor-wide, mutating nothing. rbw-pB/pU/pA/pr tabtargets + generated consts/context. Interim payor-credentialed admission-composition proof (rbw-pP) + service-tier admission-proof fixture (getIamPolicy read-back + peruse, self-skips credless). Four contract specs (RBSPB/RBSPU/RBSPA/RBSPO) authored contract-first to the RBSMA/RBSMJ precedent and wired into RBS0; documentation succession (not deletion) on the enrobe/defrock predecessors, bridge specs live to M7; acronyms map updated. All seams shellcheck-clean, theurge builds, fast qualify passes; spec quoins all resolve. The live-don payoff proof and keyless fixture rewrite are deferred to M7 as cinched.

### 2026-06-18 16:10 - ₢BZAAL - n

Federation admission contract specs (seam 4, final seam). Author the four polity-verb operation specs — RBSPB brevet / RBSPU unseat / RBSPA attaint / RBSPO rehearse — contract-first to the RBSMA/RBSMJ precedent, documenting the built rbgp_ bodies: donned-governor credential (compear then don governor), muniment-write-precedes-binding, principal:// tokenCreator on the mantle SA + first-admission depot serviceUsageConsumer, unseat-as-suspension vs attaint-sweep, manor-wide depot-blind rehearse with read-after-list-404 tolerance. Mint rbtgo_citizen_brevet/unseat/attaint + rbtgo_terrier_rehearse operation quoins and wire the four includes into RBS0 under Governor Operations, framed to separate the federation surface from the bridge-legacy cult triad. Acronyms RBSPB/RBSPU re-derived for the 260616 brevet/unseat re-election (the M4-review's RBSPN/RBSPD were for the superseded invest/divest); RBSPA/RBSPO carry over; all P-seats verified free on disk and in the map. Documentation succession (not deletion) on the enrobe/defrock predecessors — RBSDK/RBSRK -> brevet, RBSDD/RBSRD -> unseat/attaint — via superseded-by NOTEs, bridge specs left live to M7. Acronyms map: four spec entries, RBGFT refreshed (rbgft_peruse_manor + shared zrbgft_list_fetch_emit lister), RBGP noted with the polity verbs + rbw-p*/rbw-pP. All quoin references resolve; heading depth uniform. Doc-only — no code, generated files, or suites touched. The rtoe_-quoin-vs-inline-call choice for the new IAM primitives went inline per the RBSMA federation-era precedent (operator-content).

### 2026-06-18 15:06 - ₢BZAAL - n

Admission-composition proof + service fixture (seam 6). rbgp_admission_proof (rbw-pP, transitional, payor-credentialed) drives the brevet/unseat/attaint cores against a levied depot + scaffolded terrier and asserts the IAM composition by getIamPolicy read-back plus peruse: brevet writes the muniment + tokenCreator (mantle SA) + serviceUsageConsumer (depot); brevet is idempotent; unseat withdraws the muniment and tokenCreator but LEAVES serviceUsageConsumer (suspension); attaint sweeps it; the manor-wide read is clean. New zrbgp_proof_forbid_binding (absence assertion, inverse of recognosce_require_binding) + two getIamPolicy fetch helpers. New terrier-atomicity-style 'admission-proof' theurge fixture (probe payor -> charge rbw-dt scaffold -> run rbw-pP), member of service + complete suites, self-skips credless. Generated consts/context re-derived from the zipper. Build green, shellcheck 216 clean. Read-back is immediate-after-write per the scaffold posture (principal-member poll a future hardening).

### 2026-06-18 14:52 - ₢BZAAL - n

rbw-p polity launcher family (seam 5). New zipper group enrolling the four admission verbs: rbw-pB brevet / rbw-pU unseat / rbw-pA attaint (mutating, uppercase, param1 channel carrying subject as folio + mantle passed through) and rbw-pr rehearse (read-only, lowercase, no folio), all dispatched through rbgp_cli.sh. Four tabtarget stubs (Actor+Verb+Object frontispieces, governor-wielded). Generated rbtdgc_consts.rs (RBTDGC_*_POLITY) and claude-rbk-tabtarget-context.md re-derived from the zipper by the theurge build. Build green, fast qualify passed (tabtarget structure, colophon registrations, generated-file freshness).

### 2026-06-18 14:49 - ₢BZAAL - n

Polity admission verb bodies (seam 3). rbgp_brevet/unseat/attaint/rehearse over token-agnostic zrbgp_*_core helpers: brevet engrosses the muniment then idempotently ensures tokenCreator on the mantle SA + serviceUsageConsumer on the depot (spike F2, muniment-write-precedes-binding); unseat withdraws the muniment then revokes only tokenCreator (depot-scoped binding survives = suspension); attaint unseats every mantle then sweeps the serviceUsageConsumer; rehearse reads manor-wide via rbgft_peruse_manor, mutating nothing. The verbs wield a donned governor mantle (rba_compear then rba_don_capture governor — first consumer of that accessor; compear stays outside the capture so its device flow keeps the terminal); the cores stay token-agnostic so the levy founding-exception and the proof can drive them payor-credentialed. Two capture helpers (zrbgp_mantle_sa_email_capture, zrbgp_principal_member_capture) home the mantle-SA-email and principal:// member composition. rbgp_cli.sh enforce-case gains a polity-verb branch (federation + repo + depot regimes for the don). BCG capture contract honored — the _capture helpers return 1, never buc_die (operator-caught fix; re-audited clean). Shellcheck 216 clean.

### 2026-06-18 14:24 - ₢BZAAL - n

Manor-wide terrier read (seam 2). Factor rbgft_peruse's list-and-fetch into a shared internal zrbgft_list_fetch_emit (prefix empty = manor-wide whole-terrier, '<depot>/' = one polity slice); rbgft_peruse keeps its per-polity contract as a thin wrapper, new rbgft_peruse_manor reads the whole bucket. Manor-wide read is depot-blind, emits the same (mantle, subject) TSV as peruse — the depot stays the key index, recoverable but unemitted (no consumer needs per-entry depot; the muniment placement already is the principal-by-depot join). The shared lister now treats a read-after-list 404 as a benign vanish (skip, not fatal) — a pure read must not crash when a concurrent unseat withdraws an entry, and the manor sweep widens that window; per-polity peruse inherits the refinement. No muniment content-schema change. rehearse (seam 3) composes peruse_manor. Shellcheck 216 clean.

### 2026-06-18 14:20 - ₢BZAAL - n

Federation admission substrate (seam 1 of the admission-verbs pace). Two RBGC_ROLE_ named constants — tokenCreator and serviceUsageConsumer — in rbgc_constants.sh's Canonical Role IDs block, folding the one existing inline tokenCreator literal at the Cloud Build agent grant. New rbgi_add_sa_principal_iam_role / rbgi_revoke_sa_principal_member: the federated workforce principal:// member variants of the SA-binding add/revoke pair, passing the member verbatim (no serviceAccount: prefix) — a distinct canonical path per BCG Interface Contamination, never a normalizer over the serviceAccount: siblings, matching the house per-scope-full-function convention (shared only at the member-agnostic jq transforms). This is the IAM binding primitive brevet (grant tokenCreator on the mantle SA) and unseat (revoke it) compose. Shellcheck 216 files clean.

### 2026-06-18 13:37 - ₢BZAAb - W

Built the three terrier muniment sub-ops (rbgft_terrier.sh: engross/expunge/peruse over GCS object preconditions, caller-authenticates token-first, no lock logic). Settled the muniment internals the pace owned: per-entry granularity, object-name index <depot>/<mantle>/<subject>, content {rbgft_subject, rbgft_mantle} under the new rbgft_ sprue as authoritative record; per-entry immutability resolved RBSTR's open update-clause question to create-and-delete-only. RBS0 terrier-noun NOTE filled; RBSTR Forthcoming retired to a Settled pointer (ACG). Interim payor-credentialed rbgp_terrier_proof (rbw-dT) asserts the 412/404 idempotency end-to-end; terrier-atomicity service fixture (charge-via-scaffold, self-skips credless) in service+complete. Verified: build green, shellcheck 216 clean, fast suite 109/0/0, fixture self-skips credless. Folded muniment keys under rbgft_ (dropped the proposed rbtm_ sprue). Downstream seam recorded into the admission-verbs pace: rbgft_peruse is per-polity, so rehearse must extend it to a manor-wide read.

### 2026-06-18 13:30 - ₢BZAAb - n

Swap the two grep presence-checks in rbgp_terrier_proof for the BCG-blessed [[ == *needle* ]] substring test — cupel evicts grep in kit-bash. Behavior identical: assert the (mantle, subject) pair is present after engross and absent after expunge.

### 2026-06-18 13:29 - ₢BZAAb - n

Realize the three terrier muniment sub-operations against the RBSTR atomic contract. New rbgft_terrier.sh (Federation Terrier) library module: engross (ifGenerationMatch=0 media upload, 412->idempotent 'present'), expunge (DELETE, 404->idempotent 'absent'), peruse (list-and-fetch under the polity prefix, echoing mantle/subject from object content). Caller-authenticates token-first like rbgb_; carries no lock logic or IAM. Settled the muniment internals the pace owns: per-entry granularity (one object per (subject, mantle) pair), object name <depot>/<mantle>/<subject> as index, content {rbgft_subject, rbgft_mantle} under the new rbgft_ sprue as authoritative record; per-entry immutability resolves RBSTR's open update-clause question to create-and-delete-only. RBS0 terrier-noun NOTE filled with the settled internals; RBSTR Forthcoming retired to a Settled pointer (ACG: noun internals home in RBS0) keeping only the access-bearing immutability resolution. Interim payor-credentialed rbgp_terrier_proof verb (rbw-dT) drives the full round-trip and asserts the 412/404 idempotency end-to-end (exit 0 = atomicity proven); proves GCS precondition mechanics as project-owner, mantle-scoped enforcement deferred to admission paces. New terrier-atomicity service fixture (charges via the rbw-dt scaffold, runs the proof, self-skips credless) in service+complete suites. rbgft wired into rbgp_cli; RBGFT acronym added; build green, shellcheck 216 files clean; generated rbtdgc_consts.rs + tabtarget-context re-derived from the zipper.

### 2026-06-18 13:02 - ₢BZAAK - W

Built the terrier resource layer (provisioning). rbgb_: first idempotent bucket-ensure (UBLA on, 409->success), managed-folder create/purge (folder-aware teardown), IAM module wrapper. rbgi_: rbgi_add_managed_folder_iam_role mirroring the bucket-IAM 412 GCS family. rbgp_terrier_scaffold: payor-credentialed interim scaffold (rbw-dt, transitional/off-broadside) — ensures the payor-project terrier bucket, destroy-then-creates the per-polity managed folder, grants folder-scoped write + bucket-level read to the depot governor mantle, verifies via getIamPolicy read-back; RBGP_TERRIER_BUCKET const; RBGC_ROLE_STORAGE_OBJECT_{ADMIN,VIEWER}. Specs: RBS0 Terrier noun quoin settled (bucket <payor-project-id>-terrier, constant home, grain: write folder-scoped own-polity / read bucket-level manor-wide), RBSCIG managed-folder IAM contract row. Test: terrier-scaffold service fixture (probe payor -> provision -> idempotent reset; self-skips credless) in service+complete suites. Locally green: build, shellcheck (215 clean), fast (109 passed incl. cupel+conformance), fixture self-skips cleanly. Live provision/reset deferred to a credentialed environment (service suite). BCG-compliant; notched 8e0aa7f7a.

### 2026-06-18 12:59 - ₢BZAAK - n

Build the terrier resource layer (provisioning pace). rbgb_: idempotent bucket-ensure (UBLA enabled, 409->success, contrasting the build-bucket's pristine 409-fatal), managed-folder create/purge (folder-aware teardown vs today's object-only emptying), and a managed-folder IAM module wrapper. rbgi_: rbgi_add_managed_folder_iam_role mirroring the bucket-IAM 412-family loop. rbgp_: rbgp_terrier_scaffold — a payor-credentialed interim scaffold (rbw-dt, transitional/off-broadside) that ensures the payor-project terrier bucket, destroy-then-creates the per-polity managed folder, grants folder-scoped write + bucket-level read to the depot's governor mantle, and verifies via getIamPolicy read-back; RBGP_TERRIER_BUCKET constant; rbgp_cli sources/kindles rbgb. RBGC_ROLE_STORAGE_OBJECT_{ADMIN,VIEWER} constants. Specs: RBS0 Terrier noun quoin filled (bucket <payor-project-id>-terrier, constant home, managed-folder grain: write folder-scoped own-polity / read bucket-level manor-wide), RBSCIG managed-folder IAM contract row. Test: terrier-scaffold service fixture (probe payor -> provision -> idempotent reset; self-skips credless) in service+complete suites. Build green; shellcheck 215 files clean. Generated rbtdgc_consts.rs + tabtarget-context regenerated from the zipper.

### 2026-06-18 11:53 - Heat - n

Reconcile the memo's manor-identity section to the paddock revert: the manor-prerequisite body line is left in Fable's original wording under the freeze (not corrected in place), the divergence flagged only by a banner pointer to this memo, and the terminal Fable review reconciles it in the body.

### 2026-06-18 11:52 - Heat - d

paddock curried: revert the manor-identity body edit to Fable's original; downgrade banner note to a divergence-pointer — freeze integrity restored, correction lives in memo+dockets, 260618

### 2026-06-18 11:31 - Heat - S

terrier-subops

### 2026-06-18 11:31 - Heat - T

terrier-provision

### 2026-06-18 11:30 - Heat - n

Record the terrier pace split in the settled-design section: two paces (provisioning + sub-ops), each carrying its own service-tier test, with testing deliberately not separated because these tests are service-tier and inseparable from their code (no unit seam worth deferring; a coding-only pace would carry no meaningful local test). Assigns the noun-internals per pace (provisioning settles bucket name/home + managed-folder grain; sub-ops settles muniment shape + granularity) and names each pace's fixture, so the test-context is durable in the memo given the immutable BZ paddock.

### 2026-06-18 11:08 - Heat - n

Consolidate this chat's four appended terrier sections in the pace-design memo into two, removing the waffle the conversation accreted (the levy/founding-graft arc and the dedicated-manor-provision-op arc, both explored and withdrawn) while keeping everything load-bearing. New 'Terrier — settled design' section: manor=payor-project (RBS0-grounded), bucket in the payor project, the guard stays, BZ-builds-capability-plus-interim-scaffold vs Bf-owns-the-permanent-founding-home, and the now-settled grain (write folder-scoped own-polity; read bucket-level manor-wide for governor mantles per the 260618 operator ruling; payor reads as project owner). New 'Manor-identity divergence — for Fable' section: the confabulation-vs-editor-drift hypothesis kept for the terminal review. The withdrawn intermediate positions remain recoverable in git history; the live memo now reads the landing, not the wandering.

### 2026-06-18 10:54 - Heat - T

terrier-graft-demolish

### 2026-06-18 10:50 - Heat - n

Record the re-cut: affiance stays narrow in BZ, the terrier's permanent founding-home defers to Bf. Supersedes the manor-identity section's dedicated-manor-provision-op decision (withdrawn as premature). Trigger: re-reading Bf (federation-evolution holding heat), which already holds the manor/affiance consolidation as deferrals — the foedus noun with its open affiance/jilt verb-register question flagged for Fable, multiple-federations (affiance keyed to named instances), and the payor-as-IT-dept governor-selects model. So enlarging affiance, or even a permanent manor-provision op, pre-empts Fable's consolidation. The cut: BZ builds the terrier capability + provisions the freehold via interim scaffold (its own paces need only a working terrier); no permanent op, no founding graft; the graft pace is abandoned and its intent moves to Bf as a named-tension idea. Unchanged RBS0-grounded BZ corrections: manor=payor-project, bucket in payor project, the guard stays. Plus the anti-conflation signposting scheme (tombstone, build-pace Cinched boundary, Bf named-tension idea; deliberately not the frozen paddock).

### 2026-06-18 06:54 - Heat - d

paddock curried: operator-sanctioned manor-identity correction (manor = payor project), 260618

### 2026-06-18 06:53 - Heat - n

Record the manor-identity finding and its consequences in the BZ pace-design memo. RBS0 makes the Manor and the Payor Project one entity (rbtgi_payor_project aliases rbtgi_manor; the manor quoin defines a control-plane GCP project; establish creates it) — the operator's understanding was right. The federation conversion introduced a second org-level 'Manor' sense (paddock's 'org anchor, a deed not a building', grep-empty in specs; affiance operates on RBRF_ORG_ID) never reconciled to RBS0. Provenance recorded neutrally as two candidates for Fable: Fable confabulation (the polished 'deed not a building' coinage, ungrounded in RBS0) vs editor spec/impl-conformance drift (operator's own candidate) — likely a blend; the call bears on a process question about paddock authoring gates. Consequences settled: terrier bucket home = the payor project (no separate manor project); the rbgp_payor.sh 'Cannot create Governor in Payor project' guard is KEPT (control-plane/workload separation; terrier is manor-grain data not depot infra, so aligned not crossing); and a dedicated scriptable manor-provision operation is chosen over bolting onto affiance (the manor has no scriptable resource-provisioning act today, establish is manual Console; integrity over MVP surface), permanent, with only the dev reset retired at graft.

### 2026-06-17 21:51 - Heat - n

Append the terrier-pace cold-read review to the BZ pace-design memo: record the confusions the first docket drafts papered over and the ameliorations chosen, so they survive the chat reset. Three substantive confusions — (1) the bucket's hosting project was never settled and the graft docket pre-committed 'establish/affiance' (affiance has no GCS project, so wrong; establish-vs-first-levy undecided); (2) read-population grain conflated with own-polity write (manor-wide roster read vs folder-scoped write, plus a latent own-depot-vs-governors-and-above source conflict and unsettled payor read mechanism); (3) the terminal Fable-review pace did not list the homing-correction as a divergence to reconcile. Plus smaller folded ameliorations (folder-grain churn default to dodge GCS same-name reuse lag, depot-freehold disambiguation, transitional colophon, operator-invoked quota-touching graft verification) and the load-bearing note that the frozen paddock's 'payor-created at levy' contradicts the dockets until the Fable review re-authors it, bridged only by the memo pointer.

### 2026-06-17 21:23 - Heat - S

terrier-graft-demolish

### 2026-06-17 21:22 - Heat - n

Record the terrier homing correction in the BZ pace-design memo, per the paddock freeze that routes post-freeze corrections here rather than into the frozen paddock. The slip: the paddock says terriers are 'payor-created at levy', but levy creates a depot while the Terrier bucket is Manor-homed (shared across every depot under the manor) — a manor-shared bucket cannot be born at depot levy (no durable home, second-depot collision). Correction (grain-sharpening, in-heat): the bucket is an idempotent ensure grafted into manor founding (establish/affiance); the per-polity managed folder + its IAM is depot-grain and grafts into levy. Code consequence: the terrier bucket-create must not copy the build-bucket's 409-is-fatal pristine guard. 'At levy' reads true only because MVP one-depot-per-manor collapses the grains; the multi-depot generalization stays deferred to Bf. Also records the idempotent-tabtarget-then-graft pace shape (build-and-verify via a destroy-then-create provisioning tabtarget against the freehold, quota-flat; demolition as a later committed pace) and the net-new surface (storage.managedFolders REST absent repo-wide; managed-folder IAM wrapper mirrors the AR repo-IAM idiom; teardown must delete managed folders).

### 2026-06-17 19:33 - Heat - f

silks=rbk-11-mvp-office-federation

### 2026-06-17 19:49 - ₢BZAAJ - W

Built the don (Leg 3) in the credential accessor as rba_don_capture (rba_auth.sh): mints a mantle-SA access token from the cached federated token via iamcredentials generateAccessToken, naming the depot as quota project (x-goog-user-project, spike F2). BCG capture contract throughout — emits the mantle token to stdout once on success, returns 1 with buc_log_args forensics on failure (buc_die only for the unknown-identity programming guard, matching the rba_token_capture sibling), never stderr; the consuming verb supplies the loud buc_die. Custody: mantle token reaches only stdout + the per-invocation curl response (BURD_TEMP_DIR, like the Leg-2 STS response), never the persistent assize cache; one mantle's authority, self-expiring. Single attempt by design: the Leg-3 403 admission-deficit Palisade signature is logged and returned, never retried as a propagation race; the cached-federated-token read caps re-minting and carries the compear instruction on assize lapse. New RBGC constants for the distinct iamcredentials service (root/v1/generateAccessToken suffix). An initial draft that buc_die'd inside the stdout-returning function was caught and rewritten to a true _capture after operator BCG flag. Shellcheck clean (215 files); fast suite 109/109. The live 200-success mint is deferred by the heat's own sequencing to citizen-attribution-proof, which needs a brevet binding from admission-verbs-polity; the Done-when is met at the contract level, the live clauses ride BZAAV.

### 2026-06-17 19:47 - ₢BZAAJ - n

Implement the don (Leg 3) in the credential accessor as rba_don_capture: mints a mantle-SA access token from the cached federated token via iamcredentials generateAccessToken, naming the depot as quota project (x-goog-user-project, spike F2). BCG capture contract — emits the mantle token to stdout once on success, returns 1 with buc_log_args forensics on failure (no buc_die except the unknown-identity programming guard, matching the rba_token_capture sibling), never stderr; the consuming verb supplies the loud buc_die. Custody: the mantle token reaches only stdout + the per-invocation curl response (BURD_TEMP_DIR, like the Leg-2 STS response), never the persistent assize cache; one mantle's authority, self-expiring (1h ceiling). Single attempt by design: the Leg-3 403 admission-deficit Palisade signature is logged and returned, never retried as a propagation race; the cached-federated-token read caps re-minting, returning 1 with the compear instruction once the assize lapses. New RBGC constants for the distinct iamcredentials service (RBGC_API_ROOT_IAMCREDENTIALS, RBGC_IAMCREDENTIALS_V1, RBGC_IAMCREDENTIALS_GENERATE_ACCESS_TOKEN_SUFFIX). Shellcheck clean (215 files). Live mint proof is deferred to citizen-attribution-proof, which needs a brevet binding from admission-verbs-polity.

### 2026-06-17 16:37 - ₢BZAAT - W

Federation compearance proven live end-to-end against the affiance-founded trust (standing spike pool spike-office-test/spike-entra, org 247899326218; affiance run first to seat workforcePoolAdmin and adopt the pool). Done-when both met: (1) live assize opened end-to-end — Leg 1 device flow under a TTY (operator compeared as my-microsoft@bhyslop.com via passkey/QR), Leg 2 STS yielded a 393-char federated token; the per-assize cache holds ONLY {federated_token, expiry_epoch, subject} at 0600 in a 0700 tmpfs dir — no id_token or mantle token — subject oid 9657166c-... matching the bound spike principal. (2) Suite-head seam — a headless no-TTY run took the cache-hit path and passed exit 0 with no device flow, as a headless suite case consumes the one compeared assize. Discharges the consolidated live-trust + human-click obligation the build paces deferred here. Side-work: affiance compearance hint repaired to yawp the rbw-acf colophon (e23bc47a2); three federation-evolution ideas homed in Bf (548437ef) — assize-rename, a clear-the-live-assize verb, and IdP-agnostic one-click compearance via verification_uri_complete (Entra empirically dark, otc-prefill dead).

### 2026-06-17 15:45 - Heat - n

Route affiance's compearance hint through buyy_tt_yawp instead of a hardcoded tabtarget filename. rbgp_manor_affiance's closing 'Verify the trust by compearing' line printed the literal 'tt/rbw-acf.CheckFederatedAccess.sh' inside a buc_info — plain text, no TT diastema span, and rename-fragile (duplicated filename rather than resolved colophon). Now yawps the RBZ_CHECK_COMPEARANCE colophon constant ('rbw-acf') through buyy_tt_yawp, resolving filename via the single colophon-glob and rendering in the proper TT span, matching the lone house-style call site rbfly_yoke.sh:103. RBZ_CHECK_COMPEARANCE is in scope (rbgp_cli.sh sources rbz_zipper + zrbz_kindle before dispatch). Surfaced by the operator while running affiance as the federation-live-proof (BZAAT) precondition; the code is BZAAM (manor-affiance) territory, so homed heat-level. The lone sibling hardcode rbfk_kludge.sh:238 is left for its own pace. Shellcheck clean (215 files).

### 2026-06-17 15:31 - ₢BZAAU - W

Added the eyes to the depot levy's federation founding and made the freehold reusable, proven live. Three pieces shipped: (1) the recognosce verb — RBSDC contract + rbgp_depot_recognosce + rbw-dr + the canonical-establish recognosce case — a read-only payor-authenticated survey confirming the three mantle SAs, every capability-set binding across project/GAR-repo/Mason/director policies, and the AR Data-Access audit config, exit 0 only when whole, fatal-naming otherwise; (2) the idempotent freehold-ensure — canonical-establish reuses an ACTIVE freehold (the quota fix, no depot created on a routine run) and pick-next-creates only when absent or graveyarded (DELETE_REQUESTED treated as gone); (3) the canonical-churn fixture — fixture-driven rotate-moniker + unmake of the canonical freehold (member of no suite, deliberate). Done-when MET: on a fresh conformant depot, recognosce passed all seven live checks ('Depot founding recognosced whole'); reuse-detect, churn, and create-path levy-with-founding all proven live and fixture-driven. recognosce mints no keys, surviving the org no-keys policy the live run surfaced. Deferred to later BZ paces: the bridge-legacy keyfile-enrobe collision (routed to RBRA-estate retirement), enrobe-idempotency, the canonical/throwaway prefix collapse, the _levy fn rename, and the rbtdrk_canonical.rs >800-line split (the latter three owned by freehold-collapse-cleanup).

### 2026-06-17 13:50 - Heat - S

freehold-collapse-cleanup

### 2026-06-17 13:28 - ₢BZAAU - n

Build the canonical-churn fixture — fixture-driven teardown of the canonical freehold, so the whole detect->churn->create->recognosce cycle runs as fixtures (no hand-edited regime surgery). New case rbtdrk_depot_churn (rbtdrk_canonical.rs): reads the freehold RBRD names, composes its project_id, rotates RBRD_DEPOT_MONIKER to a placeholder ('churned') so rbgp_depot_unmake's live-disqualify guard releases the real project, invokes rbw-dU with BURE_CONFIRM=skip, then confirms via a fresh list that the churned moniker's .depot state fact is no longer COMPLETE (DELETE_REQUESTED or absent = gone). Mirrors pristine-lifecycle's rotate-then-unmake teardown but applied to the canonical freehold. New single-case Independent fixture RBTDRK_FIXTURE_CANONICAL_CHURN (member of NO suite — operator-invoked, quota-reclaiming), registered in RBTDRC_FIXTURES + RBTDRM (name const 'canonical-churn' + required colophons UNMAKE_DEPOT/LIST_DEPOT). New const RBTDRK_CHURN_PLACEHOLDER_MONIKER; imports for RBTDGC_UNMAKE_DEPOT + RBTDRI_BURE_CONFIRM_KEY/SKIP (reformatted the rbtdri_invocation use-block one-per-line per RCG while touching it). Guard test rbtdtk_churn_case_registered. Build clean; 162/162 unit tests pass; fixture discoverable via FixtureRun/FixtureCase. Note: rbtdrk_canonical.rs now ~1075 lines, further over the RCG 800 threshold flagged earlier — the deferred collapse-cleanup split will address it.

### 2026-06-17 12:51 - ₢BZAAU - n

RCG Comment Discipline pass on the freehold-ensure comments: drop two task-flavored asides RCG prohibits — '(the quota fix)' (rationale already carried by 'no depot is created on a routine run') and '(Fn name keeps _levy pending the collapse-cleanup rename.)' (referenced future planned work). Replaced the latter with a permanent statement that the fn retains _levy though it now reuses or levies. Comments now describe permanent behavior only, no session/future-work references. Build + 161 unit tests green.

### 2026-06-17 12:44 - ₢BZAAU - n

Make canonical-establish's depot case an idempotent freehold-ensure — the quota fix from rediscovering the depot-creation pain. rbtdrk_depot_levy_impl now: install canonical prefixes, list, then READ the freehold RBRD already names and its .depot state fact — if ACTIVE (RBTDRK_DEPOT_STATE_COMPLETE='COMPLETE'), REUSE it (no depot created on a routine run); otherwise (blank, absent, or DELETE_REQUESTED — a graveyarded id is treated as gone per operator ruling, enabling immediate re-create under a fresh moniker more often than the 30-day window) pick-next moniker + install + levy. Validity is NOT judged here — recognosce (the unconditional fifth case) remains the freehold's validity gate, so a stale-but-ACTIVE freehold is reused here and fails there, prompting deliberate churn. project-id cross-check runs both paths (reuse reads list_pre; create re-lists). Writes a freehold-decision.txt trace (reused/levied <moniker>). New const RBTDRK_DEPOT_STATE_COMPLETE; case-1 doc comment updated to reflect ensure-not-always-levy (fn name keeps _levy pending the collapse-cleanup rename, noted for the end redocket). Builds clean; 161/161 unit tests pass. Pure code — no live GCP. Deferred to later BZ paces (per operator): enrobe-idempotency, throwaway-prefix retirement, the fn rename, and updating (not running) the create-delete lifecycle fixture.

### 2026-06-17 12:02 - ₢BZAAU - n

Tidy the recognosce mantle-existence check labels: drop the word 'present' from the three rbuh_require_ok labels (governor/director/retriever mantle SA), since on a 404 failure the label folds into the error and 'SA present (...) (HTTP 404): Unknown service account' read self-contradictory. Now reads 'recognosce: governor mantle SA (<email>) (HTTP 404): ...' — clean. Surfaced by the first live run against the pre-mantle freehold depot. Shellcheck clean (215).

### 2026-06-17 11:51 - ₢BZAAU - n

Wire the recognosce verify case into the gauntlet's canonical-establish fixture. New case rbtdrk_depot_recognosce (rbtdrk_canonical.rs): precondition probe (canonical depot moniker installed), then invoke RBTDGC_RECOGNOSCE_DEPOT (rbw-dr) and gate purely on exit_code==0 — the verb does the whole founding check and dies fatally naming any absent piece, so the case asserts only 'founding whole'. Registered as the fifth/last case of RBTDRK_CASES_CANONICAL_ESTABLISH (after the levy+enrobes; a pass also confirms the enrobes left the mantle founding intact) and added RBTDGC_RECOGNOSCE_DEPOT to canonical-establish's required-colophons (rbtdrm_manifest.rs) + the rbtdgc_consts import list. Updated the guard test rbtdtk_cases_registered to expect five cases including recognosce. Theurge builds clean, case discoverable via FixtureCase, all 161 unit tests pass. Live run (against a real levied depot with payor creds) remains the operator-invoked proof, like foedus-lifecycle.

### 2026-06-17 11:44 - ₢BZAAU - n

Implement the recognosce verb per the RBSDC contract. rbgp_depot_recognosce (rbgp_payor.sh): payor-authenticated read-only survey — confirms the three rbma- mantle SAs exist (serviceAccounts.get x3), reads the project IAM policy v3 and requires every project-scoped capability-set binding (governor owner; retriever reader+occurrences-viewer; director builds-editor+viewer+workerPoolUser) plus the artifactregistry Data-Access audit config (ADMIN_READ+DATA_READ), then reads the GAR repo / Mason SA / director SA policies and requires the director's off-project bindings (repoAdmin, actAs on Mason, self-actAs). Helper zrbgp_recognosce_require_binding asserts a (role,member) present in a captured policy via jq field-capture or buc_die naming the absent piece; exit 0 means only 'founding whole'. Expected-binding lists mirror rbgw_grant_*_capabilities (static; cross-ref comment marks the drift seam per the single-list resolution). Pass/fail by exit code + resource facts, no banner parsing, no precision-band codes (positive survey). Wiring: zipper enrolls RBZ_RECOGNOSCE_DEPOT -> rbw-dr -> rbgp_depot_recognosce ('' channel, like depot_info; no rbgp_cli change — recognosce rides the default depot-regime arm); tabtarget tt/rbw-dr.DepotRecognosce.sh; theurge consts (RBTDGC_RECOGNOSCE_DEPOT) + tabtarget-context regenerated by the build. Shellcheck clean (215), fast qualify clean (tabtarget/colophon/generated-freshness).

### 2026-06-17 11:27 - ₢BZAAU - n

Contract-first mint of the recognosce verb (depot founding-survey). RBSDC-depot_recognosce.adoc: read-only operation — payor authenticates, confirms the three rbma- mantle SAs exist (serviceAccounts.get x3), then reads four IAM policies (project getIamPolicy v3, GAR repo, Mason SA, director SA) and requires every capability-set binding present plus the artifactregistry Data-Access audit config (ADMIN_READ+DATA_READ); names the scope and points at rbgw_capabilities for the binding inventory (ACG, no drift); exit 0 means only 'founding whole', any miss fatal and named. Seated in RBS0 as {rbtgo_depot_recognosce} -> rbgp_depot_recognosce, declared in the mapping section and given its operation block in the depot family after depot_list. Verb name minted under MCM Word Selection: Scots-law inspection register (recognosce = examine/inspect officially), harmonizing with the heat's compear; grep-clean repo-wide; colophon rbw-dr (free), contract acronym RBSDC (RBSDR taken by director_roster). Payor-credentialed, governor-capable noted not built. Pass/fail by exit code + resource facts, never banner prose; precision-band codes reserved for any future negative case, not minted for this positive survey.

### 2026-06-17 10:11 - ₢BZAAW - W

foedus-lifecycle theurge fixture landed and proven live. Single-case round-trip mirroring reliquary-lifecycle (no charge/quench): payor-credential gate that FAILS LOUD (not Skip) per operator ruling — this fixture is quota-touching/operator-invoked-only, never a suite passenger, so a missing precondition is a failure of the run; mint a unique throwaway pool id (foedus-<epoch_millis>); affiance the manor onto it under the regime-poison tweak overriding RBRF_WORKFORCE_POOL_ID (RBRF_ scope prefix drives zbuv_poison_apply); assert create+affianced banners; jilt under the same override + confirm-skip, assert the DELETED terminal; re-jilt, assert the idempotent no-op. Best-effort cleanup jilt on failure. Registered in RBTDRC_FIXTURES for discovery, member of NO suite. Docket reslated Skip->Fail to record the operator ruling. Live operator-invoked run PASSED end to end on pool foedus-1781716116436: created -> dissolved (DELETED) -> re-jilt no-op (1 passed, 0 failed). Code notched 0c70ac67b. Note: the live run consumed one soft-deleted pool slot against the org 100-cap for ~30 days, as designed.

### 2026-06-17 10:06 - ₢BZAAW - n

Nucleate the foedus-lifecycle theurge fixture — the repeatable autonomous proof of the affiance->jilt create/destroy round-trip on an ephemeral workforce pool, codifying the manual proof the create-shape fix was found by. Mirrors reliquary-lifecycle's single-case round-trip shape (no charge/quench): probe the payor credential and FAIL LOUD (not Skip) when it is not green — operator ruling this conversation, since this fixture is quota-touching/operator-invoked-only and never a suite passenger, so a missing precondition is a failure of the run; then mint a unique throwaway pool id (foedus-<epoch_millis>, within the RBRF_WORKFORCE_POOL_ID regex), affiance the manor onto it under the regime-poison tweak overriding RBRF_WORKFORCE_POOL_ID (the RBRF_ scope prefix makes zbuv_poison_apply rewrite that one field at kindle), assert the create + affianced banners, jilt under the same override plus confirm-skip and assert the dissolved/DELETED terminal, then re-jilt and assert the idempotent no-op. Banner assertions match stdout+stderr concatenated (BUK folds the log stream). Best-effort cleanup jilt on any round-trip failure so a leaked LIVE pool is at least soft-deleted rather than left counting against the 100-per-org cap as active. Registered in RBTDRC_FIXTURES for discovery (runnable via FixtureRun) but a member of NO suite — quota-touching by design. Manifest: RBTDRM_FIXTURE_FOEDUS_LIFECYCLE const + required-colophons arm (CHECK_PAYOR/AFFIANCE_MANOR/JILT_MANOR). Compiles clean; discoverable.

### 2026-06-17 09:40 - Heat - n

Capture the 260617 freehold-doctrine sharpening (post-freeze, operator-sanctioned) into the two ₣BZ design memos. New workforce-pool-constraints memo: verified via websearch that workforce pools cap at 100/org, soft-deleted pools hold their id-namespace and count against the cap for ~30 days, a soft-deleted id cannot be re-created (only undeleted) — so fresh-create-per-run is quota-threatening (the depot-project pain recurring); records the latent affiance bug (GET on a soft-deleted pool returns 200/DELETED, so affiance skips create treating dead as live) and the reuse-one-durable-pool mitigation. Pace-design memo gains a Freehold doctrine section: the freehold concept (durable reused test installation, freehold/leasehold), the foedus-lifecycle (ephemeral, quota-touching, not-yet-built) vs foedus-freehold (durable, quota-flat) split, the quota constraint, the affiance undelete-on-DELETED fix, the roster correction (the foedus freehold DOES carry a standing-citizen roster via the manor-homed terrier/rehearse — retracting the earlier no-roster claim; providers-are-a-roster dropped as a weak prop), and the foedus×depot intertwining (the roster is the join). Recorded as the freeze-sanctioned corrections home; the terminal Fable heat-review reconciles.

### 2026-06-17 09:39 - Heat - S

foedus-freehold

### 2026-06-17 08:20 - Heat - T

foedus-lifecycle

### 2026-06-17 07:28 - ₢BZAAW - n

Fix the affiance workforce-pool create REST shape, found by debugging the affiance-proof Tier B create path (unrun since the spike). rbgp_manor_affiance passed the org parent as a query parameter; workforcePools.create rejects that (HTTP 400 'Unknown name parent: Cannot bind query parameter') — parent is an immutable body field, and only workforcePoolId is a query param (the URL path parent is locations/global; the pool's resource name carries no org). Moved parent into the jq body, dropped it from the query string. Proven live against org 247899326218: pool + provider create REST shapes both succeed, then jilt dissolves the throwaway pool to the DELETED terminal, and a re-jilt no-ops on the soft-deleted pool. RBSMA contract corrected to the empirically-confirmed create shape (parent body field, workforcePoolId sole query param; dropped the never-sent disabled:false line). Shellcheck clean (215).

### 2026-06-16 17:00 - ₢BZAAX - W

Built the manor jilt verb (rbw-mJ / PayorJiltsManor), affiance's structural inverse — a payor-credentialed DELETE that dissolves the org-level workforce pool (provider cascades), breaking the manor↔IdP betrothal. Contract-first: RBSMJ spec mirrors RBSMA delete-shaped (confirm → payor auth → probe → DELETE → poll to soft-deleted terminal), with the soft-delete/undelete graveyard NOTE, pool-only/provider-cascades cinch, and depot-scoped-bindings-untouched scope boundary. Impl rbgp_manor_jilt() reads RBRF (no folio), idempotent no-op on absent (404) or already-DELETED pool, tolerates DELETED-state or 404 as the dissolved terminal. rbgp_cli enforces RBRF for jilt beside affiance; zipper enrolls RBZ_JILT_MANOR; theurge consts + tabtarget-context regenerated. RBS0 seats rbtgo_manor_jilt (include RBSMJ) and the rbtf_jilt civic verb beside affiance. BCG-compliant (read BCG in full; inverted one test-&&-buc_die to ||; shellcheck clean 215). Verified: fast qualify clean, fast suite 10/109 green (conformance + unmake-refusal), grep gate clean (one meaning repo-wide). Live create→jilt round-trip deferred by design to the affiance-proof pace's Tier B teardown — the same not-yet-live-exercised posture affiance shipped in.

### 2026-06-16 16:44 - ₢BZAAX - n

Build the manor jilt verb (rbw-mJ / PayorJiltsManor), affiance's structural inverse: a payor-credentialed DELETE that dissolves the org-level workforce pool (the provider cascades beneath it), breaking the manor↔IdP betrothal. Contract-first per the documentation-strategy cinch. RBSMJ contract spec authored mirroring RBSMA's structure but delete-shaped (safety confirm → payor auth → probe pool → DELETE → poll to soft-deleted terminal), with the soft-delete/undelete graveyard NOTE, the pool-only/provider-cascades cinch, and the scope boundary (depot-scoped mantle bindings untouched). Impl rbgp_manor_jilt() reads RBRF (no folio, like affiance), idempotent no-op on an absent (404) or already soft-deleted (200/DELETED) pool, and tolerates either DELETED-state or 404 as the dissolved terminal (robust to whether GET surfaces soft-deleted pools). rbgp_cli enforces RBRF for jilt alongside affiance. Zipper enrolls RBZ_JILT_MANOR; theurge consts and tabtarget-context regenerated from the build. RBS0 seats the rbtgo_manor_jilt operation (include RBSMJ) and the rbtf_jilt civic verb beside affiance. BCG-compliant (shellcheck clean 215 files; one test-&&-buc_die inverted to ||-buc_die per BCG). Fast qualify clean (tabtarget/colophon/generated-freshness); grep gate clean (one meaning repo-wide). Live create→jilt round-trip deferred by design to the affiance-proof pace's Tier B teardown.

### 2026-06-16 16:18 - Heat - S

manor-jilt

### 2026-06-16 16:18 - Heat - n

Record the operator-sanctioned manor jilt verb mint (260616): jilt is the affiance inverse (dissolve a workforce pool / break the manor-IdP betrothal), colophon rbw-mJ, frontispiece PayorJiltsManor, payor-credentialed. Full MCM Word Selection rationale in the pace-design memo — asterism decider (affiance<->jilt betrothal antonym pair, sunder runner-up), sibling-initials clean (manor demesne L/E/A +J), grep-gate 0 hits, vocabulary isolation, husbandry; exposure ashlar/broadside-eligible (permanent capability, unlike transitional enrobe/defrock), RBSMJ contract-first seat (verified free), RBS0 civic-quoin alongside affiance; scope note for Fable that this adds a permanent un-founding verb beyond the frozen founding triad, operator-pulled in-heat. m4-review memo: one-line RBSMJ seat-reservation addendum so Fable's M-seat census stays accurate. jilt is its own contract-first build pace ordered immediately before affiance-proof.

### 2026-06-16 15:45 - Heat - n

Record the 260616 colophon ground-truth inventory and the mantle-homonym-guard vestige finding in the pace-design memo (extends divergence #2): live cult colophons are enrobe/defrock per the complete cult-rename pace, brevet/unseat landed as RBS0 quoins but not yet in code/tabtargets, and the guard names the pre-rename invest/divest/mantle family the freeze pinned in place after its discharge condition was met; flagged for Fable's terminal review. Fix the lone stale federation-verb reference in the derived acronym mirror (RBSTR entry: invest/divest/rehearse -> brevet/unseat/rehearse, matching RBSTR-Terrier).

### 2026-06-16 15:44 - Heat - d

paddock curried: operator-sanctioned: freeze-banner routing note — evolution ideas go to ₣Bf, corrections to pace-design memo

### 2026-06-16 09:42 - Heat - f

silks=rbk-11-office-federation

### 2026-06-16 13:03 - ₢BZAAI - W

Accessor federation branch through Leg 2 — build complete and machine-verified; wraps on the redocketed build-only criteria, live human-in-the-loop proof handed to ₢BZAAT. Functional draft (3849d168f): device-flow Leg 1 + STS Leg 2 as accessor steps, the per-session assize cache (federated-token-only producer/consumer seam), rba_compear ensure-membrane (credless→104, cache-hit reuse, TTY-gate, headless fail-loud), rbw-acf probe. BCG-discipline polish (f8ee048bd, 10dbc1e9b): every $() on an external removed — non-secret leg scalars go jq/date→temp file→$(<file); the federated/id/access tokens emit straight to stdout via a final/branch jq (select(length>0)+-e reproduce the prior non-empty guards), so no token touches a var or temp file; subject decode folded into one capture-final printf|openssl|jq pipeline; zrbrf_enforce glob [[]] tests → case (openid/offline_access/google.subject), two non-load-bearing gcp- checks dropped. Verified: shellcheck 215 clean, rbw-rfv 10/10 exit 0, credless guard 104, headless fail-loud exit 1 no hang, jq emit-expressions unit-checked. Deferred (not regressions): live end-to-end proof — Leg 1 browser approval, Leg 2 yielding a token, suite-head consumption — owned by ₢BZAAT (railed after affiance); the curl-response-file id-token custody gap rides a later custody/headless reconsideration. Out of scope, untouched: rba_extract_json_to_rbra still carries pre-existing $(jq) on externals.

### 2026-06-16 13:02 - ₢BZAAH - W

Folded the federation founding into depot levy: establish the three impersonatable mantle SAs (rbma-governor/director/retriever) beside Mason, grant each its shared rbgw capability-set, mark the settle gate, and enable Artifact Registry Data-Access audit logs. Carried the rbgw capability-set relocation (shared module reached by both enrobe and levy) and the RBSMF contract. Review-hardened this session: added the audit-set content gate (assert the returned policy carries the artifactregistry auditConfigs entry, not just HTTP 200); moved the AR audit getIamPolicy/setIamPolicy to CRM v3 with updateMask auditConfigs,etag (web-confirmed canonical form, v1 lifecycle constants left surgical); corrected RBSCIG's stale no-audit-config claim and added the RBSHR v1->v3 migration horizon entry. Adversarial review found no levy-breaking bug and no BCG violation; shellcheck 215 clean. Runtime proof (fresh-levy gauntlet) handed to the levy-founding-proof pace; live citizen-attribution proof handed to its capstone pace.

### 2026-06-16 13:02 - ₢BZAAM - W

Manor affiance built contract-first and wired end-to-end. RBSMA spec: the payor's IdP-trust founding — seat org-level workforcePoolAdmin first (spike F1), then idempotent ensure-exists of the workforce pool, provider, and attribute mapping under organizations/RBRF_ORG_ID, reading the RBRF federation regime; seated under the manor demesne in RBS0. Implementation rbgp_manor_affiance (rbgp_payor.sh): BCG-clean, reuses the generic rbgi IAM read-modify-write helper for the F1 org self-grant, GET-then-create for pool/provider, attributeMapping parsed in jq, webSsoConfig fixed as a protocol constant. Wiring: new rbw-m manor colophon group + rbw-mA tabtarget; furnish enforces rbrf and skips the depot/repo regimes (affiance runs pre-levy); zipper-regenerated rbtdgc_consts.rs + tabtarget-context. Build/shellcheck(215)/qualify-fast all green. Contract aligned to the ensure-exists impl with drift-reconcile flagged as a deferred horizon item. Live verification deferred by design to the slated ₢BZAAW affiance-proof (Tier A reconcile against the standing spike trust + Tier B throwaway-pool create-path de-risk) and the downstream Leg-1 capstone.

### 2026-06-16 13:01 - ₢BZAAM - n

Align the RBSMA affiance contract to the shipped ensure-exists implementation, resolving the contract/impl divergence flagged before wrap. The two ensure steps now describe GET-then-create (workforcePools.get / providers.get -> create only on 404) matching the code, instead of the prior submit-create + 409-ALREADY_EXISTS-tolerant framing the code never uses. Dropped both drift-reconcile-via-PATCH claims (pool sessionDuration; provider attributeMapping/oidc) and added an explicit NOTE that drift reconciliation is a deferred follow-up (horizon item): affiance ensures existence, not field convergence — an already-present pool/provider is left as it stands. Idempotent NOTE reworded ensure -> ensure-exists; soft-delete graveyard NOTE merged (dropped the duplicate) and reframed to the 404-GET-then-409-create signal with the undelete pointer. Prose-only; no quoin/anchor changes, no code change. Resolves the last open concern on the affiance pace; ₢BZAAM is ready to wrap. Verification deferred by design to the slated ₢BZAAW affiance-proof pace (Tier A reconcile + Tier B create-path) and the downstream Leg-1 capstone.

### 2026-06-16 12:55 - Heat - S

affiance-proof

### 2026-06-16 12:47 - ₢BZAAH - n

Documentation tail of the levy audit-log change. RBSCIG: correct the now-stale claim in the updateMask-default section that 'Recipe Bottle does not configure audit logging via IAM policy' — the levy now enables AR Data-Access audit logs; document the canonical masked-write shape (CRM v3 getIamPolicy/setIamPolicy, updateMask auditConfigs,etag, bindings left untouched outside the mask as the safe side of the full-replace footgun, etag-in-mask keeping the concurrency check load-bearing). RBSHR: new horizon entry for migrating the remaining CRM v1 callers (rbgi_add_project_iam_role + RBGD_API_CRM_GET/DELETE/UNDELETE_PROJECT) to v3 — web-evidenced deprecation path; the audit call already moved, the rest left surgical with trigger and reference recorded.

### 2026-06-16 12:46 - Heat - r

moved BZAAV after BZAAL

### 2026-06-16 12:39 - ₢BZAAM - n

Wire rbgp_manor_affiance into the payor CLI as colophon rbw-mA. New rbw-m manor colophon group (Manor — IdP federation founding) seeded by RBZ_AFFIANCE_MANOR rbw-mA -> rbgp_manor_affiance (channel empty, no folio) in rbz_zipper.sh; seeds the elected manor family that the M5 regroup gathers levy/establish into. Correctness core in zrbgp_furnish (rbgp_cli.sh): affiance runs at manor founding BEFORE any depot exists, so it must NOT enforce the depot/repo regimes — replaced the depot_list-only enforce guard with a per-command case (depot_list enforces nothing; manor_affiance enforces RBRF only; every other command enforces rbrr+rbrd). Added rbrf to the furnish graph (source rbrf_regime.sh + RBCC_rbrf_file before kindle so the locked values are present; zrbrf_kindle unconditional like the other regimes), so affiance reads the federation trust config and RBRP (operator email) but not the depot regime. New tt/rbw-mA.PayorAffiancesManor.sh launcher stub (byte-identical BURD_LAUNCHER pattern). Regenerated the two zipper-derived files via tt/rbw-tb.Build.sh (NOT hand-edited): rbtdgc_consts.rs (RBTDGC_AFFIANCE_MANOR) and claude-rbk-tabtarget-context.md (Manor group + rbw-mA row). Verified: theurge build green, shellcheck 215 clean, qualify-fast passes all four (tabtarget structure, colophon registration, context freshness, Rust consts freshness). Committed only these five files. Still pending: live verification (rbw-acf device-flow proof needs a human browser click); the REST create shapes remain doc-derived/unexercised and drift-reconcile is deferred (both flagged on the prior notch).

### 2026-06-16 12:37 - Heat - S

citizen-attribution-proof

### 2026-06-16 12:36 - Heat - S

levy-founding-proof

### 2026-06-16 12:32 - ₢BZAAH - n

Move the AR Data-Access audit-log call to CRM v3 and complete the updateMask, per the authoritative Google guidance (web-confirmed). Google documents the Data-Access audit-log procedure only against v3 getIamPolicy/setIamPolicy and CRM v1 is on the deprecation path, so RBGD_API_CRM_GET/SET_IAM_POLICY now ride a new RBGD_API_BASE_CRM_PROJECT_V3 base — matching the spec's v3 links and rbgp_payor.sh's v3 project-lifecycle calls; the v1 GET/DELETE/UNDELETE_PROJECT lifecycle constants stay on the proven v1 base (used by rbgg_governor). updateMask 'auditConfigs' -> 'auditConfigs,etag' (canonical form): the etag the code already captures and rides is now in the mask, making the optimistic-concurrency check load-bearing instead of a no-op. Bindings remain protected (outside the mask); the masked write is the safe side of Google's 'might lose access' full-replace warning. Function comment updated. Shellcheck 215 clean; runtime proof remains the gauntlet.

### 2026-06-16 12:30 - ₢BZAAM - n

Implement rbgp_manor_affiance (the RBSMA affiance ceremony) in rbgp_payor.sh, mirroring rbgp_depot_levy's house style. Payor authenticates via zrbgp_authenticate_capture, then: (F1) seats org-level roles/iam.workforcePoolAdmin on user:RBRP_OPERATOR_EMAIL by reusing the generic rbgi_add_project_iam_role helper against organizations/RBRF_ORG_ID (etag read-modify-write, idempotent) — must precede the creates per spike F1; (pool) idempotent GET-then-create of the workforce identity pool under organizations/RBRF_ORG_ID via rbge_lro_ok; (provider) idempotent GET-then-create of the pool provider, attributeMapping parsed from the regime's comma-separated key=value string inside jq, webSsoConfig fixed (ID_TOKEN/ONLY_ID_TOKEN_CLAIMS) as an affiance-local protocol constant. Reads the committed RBRF federation regime as founding inputs. BCG-compliant: two-line captures, jq->file redirects (no command substitution), no stderr suppression; shellcheck 215 files clean. Function is inert — not yet wired into the payor CLI/zipper (no colophon), so qualification-invisible. FLAGGED for review: (1) workforce-pool/provider REST create shapes are IAM-v1-doc-derived and NOT yet exercised live (the device-flow proof rides tt/rbw-acf, needs a human browser click); (2) drift-reconcile on an existing pool/provider (PATCH) is deferred as a named follow-up, so the impl is ensure-exists where the RBSMA contract says reconcile-on-drift — contract/impl divergence to resolve. Remaining: CLI furnish (kindle+enforce rbrf) + tabtarget (rbw-mA) + zipper registration + build, then live verification.

### 2026-06-16 12:24 - ₢BZAAH - n

Audit-log content gate: after the AR Data-Access setIamPolicy, assert the returned policy actually carries the artifactregistry.googleapis.com auditConfigs entry (rbuh_json_field_capture on .auditConfigs[]?|select(.service==...), buc_die on miss). Closes the gap where HTTP 200 alone could mask a silently-dropped auditConfigs on the unrun masked-write path — satisfies RBSMF's returned-policy require, which the prior code under-implemented as a bare status check. Shellcheck 215 clean.

### 2026-06-16 12:22 - Heat - S

federation-live-proof

### 2026-06-16 12:11 - ₢BZAAI - n

BCG comment-smell follow-up: reword the three token-emit comments in rba_auth.sh that read 'matching the prior test -n ...' — a mild commit-message-comment (BCG §Commit-Message Comments), since a future reader has no 'prior' form for context. Now describe the code's own behavior (select(length>0) -> non-zero jq exit for an absent/empty token = the capture miss) with no change-history reference. Comment-only; shellcheck 215 clean.

### 2026-06-16 12:07 - ₢BZAAI - n

BCG-discipline polish on the ₢BZAAI federation bash — no behavior change. rba_auth.sh: every $() on an external command removed. Non-secret leg scalars (expiry_epoch, device_code, user_code, verification_uri, interval, poll error, expires_in, and both date +%s clock reads) now go jq/date -> dedicated temp file -> $(<file), with new ZRBA_FED_*_FILE kindle constants. The federated/id/access tokens are emitted STRAIGHT to each function's stdout by a final/branch jq (jq -er '... // empty | select(length > 0)' for assize-read and leg1 id_token; jq -er --argjson e for leg2's '<token> <expires_in>'), so no token passes through a shell var or temp file — select(length>0)+(-e) reproduce the prior test -n guards exactly, and leg2's no-access_token forensic buc_log_args rides the captured exit status. zrba_idtoken_subject_capture folded its $(openssl) into one capture-final printf|openssl|jq pipeline (pipefail-guarded) so the decoded id_token payload never lands on disk; added ZRBA_FED_OPENSSL_STDERR_FILE. assize-read reordered (non-secret expiry/now checks first, token emit last) — same return semantics. rbrf_regime.sh zrbrf_enforce: the four glob [[ ]] tests -> case (openid required, offline_access forbidden, google.subject mapped); the two gcp- reservation [[ ]] checks dropped as non-load-bearing (Google enforces server-side). All =~ regex [[ ]] tests kept (BCG-blessed). rbgv_cli.sh reviewed, left unchanged (its only secret-touching line is $(zrba_assize_read_capture), a $() on a _capture function — allowed). Verified: shellcheck 215 clean; rbw-rfv 10/10 PASS exit 0; credless guard -> 104; headless -> fail-loud exit 1, no hang. NOT addressed (surfaced to operator as the standing design flag): the curl response files still land id_token/access_token/federated_token in forensic BURD_TEMP_DIR, in tension with the id_token-never-persisted cinch — a custody design call deferred toward the headless-reconsider pace, not a mechanical BCG fix.

### 2026-06-16 12:06 - ₢BZAAM - n

Author the manor-affiance contract spec (RBSMA) and seat it in RBS0. New RBSMA-manor_affiance.adoc: the affiance operation contract — payor seats the org-level roles/iam.workforcePoolAdmin self-grant first (spike F1, must precede or both creates 403), then idempotent-ensures the workforce pool, its provider, and the attribute mapping under organizations/RBRF_ORG_ID, reading the committed RBRF federation regime as founding inputs (org/pool/provider/issuer/client/mapping/session). Idempotent-ensure with reconcile-on-409 (patch drift back to regime) makes the standing spike trust the reference instance; webSsoConfig (ID_TOKEN/ONLY_ID_TOKEN_CLAIMS) is an affiance-local protocol constant, not regime config; scope bounded to F1 only — the depot-scoped F2 serviceUsageConsumer and the mantle tokenCreator are left to levy/admission. Grantee is user:RBRP_OPERATOR_EMAIL. RBS0 wiring: rbtgo_manor_affiance mapping entry + operation seating under Payor ops in founding order (establish -> affiance -> levy) + include::RBSMA. References RBRF_* as literal regime fields, decoupled from the concurrently-churning rbrf_* quoins. Verified anchor/ref balance (1 mapping / 1 anchor / 1 header usage / 1 subfile usage), include present. Committed only these two files; rba_auth.sh + rbrf_regime.sh left untouched for the concurrent BCG-polish officium. Implementation (rbgp_manor_affiance) and live rbw-acf Leg-1 verification deferred per the trot.

### 2026-06-16 12:01 - ₢BZAAH - n

Levy code-body: fold the federation founding steps into rbgp_depot_levy(). Added RBCC_account_mantle_{governor,director,retriever} (hardcoded rbma-<role>, hyphen — GCP SA account-ids forbid underscore per RFC1035). New zrbgp_establish_mantle_sa (create + propagation-poll one mantle SA, mirroring the Mason block) and zrbgp_enable_ar_audit_logs (getIamPolicy -> add auditConfigs for artifactregistry.googleapis.com ADMIN_READ+DATA_READ -> setIamPolicy masked to auditConfigs riding the read etag, never iamcredentials per spike V3). Levy now establishes the three mantle SAs beside Mason, grants each its shared rbgw capability-set, marks the settle-gate freeze, and enables AR audit logs — slotted after the Cloud Build agent step. All three Done-when criteria met in code+spec. Shellcheck 215 clean; runtime proof is the gauntlet (fresh levy), deferred to operator orchestration.

### 2026-06-16 11:50 - ₢BZAAI - n

Accessor federation branch through Leg 2 — FUNCTIONAL DRAFT (BCG-discipline polish handed to a focused follow-up chat). In rba_auth.sh: federation protocol constants in kindle (STS/device-flow URNs, endpoints, poll ceiling, skew, scratch files); zrba_tty_present_predicate (/dev/tty open-probe via `:` redirect, no subshell); the assize cache as the producer/consumer seam — path/read/live-predicate/write, session-scoped tmpfs ($XDG_RUNTIME_DIR/$TMPDIR), 0700 dir + 0600 atomic temp-then-rename, federated-token-only; zrba_leg1_idtoken_capture (RFC 8628 device flow: device-code request, prompt surfaced to /dev/tty, poll with transient-retry + slow_down backoff); zrba_leg2_federated_capture (RFC 8693 STS exchange, audience = provider resource name, F3 no-extras); zrba_idtoken_subject_capture (best-effort subject decode); rba_compear as the ensure-assize membrane (credless guard -> band 104, cache-hit reuse, TTY-gate, headless fail-loud — matches the cinched docket). rbw-acf CheckFederatedAccess probe (rbgv_check_compearance, depot-agnostic like the payor probe; sources+kindles+enforces rbrf, calls rba_compear, reads the token from cache) + zipper registration + regenerated tabtarget-context. Verified: shellcheck 215 clean; rbw-rfv validate clean; credless probe -> 104; headless probe -> fail-loud exit 1, no 15-min hang. NOT done: live device-flow proof against the standing spike trust (needs a human browser click). KNOWN BCG DEBT for the follow-up: (1) $(jq ...) and $(date +%s) command substitutions in the federation functions -> temp-file + $(<file)/read, with secret return-values (federated_token/id_token/access_token) emitted directly by jq to stdout (never captured to a var or extra temp file); (2) rbrf_regime.sh zrbrf_enforce glob [[ ]] tests (gcp-/openid/offline_access/google.subject) -> case form, gcp- checks droppable as non-load-bearing. FLAGGED design concern (not pure BCG): the curl response files (ZRBA_FED_*_RESPONSE_FILE) land id_token/access_token/federated_token in forensic BURD_TEMP_DIR, in tension with the id_token-never-persisted custody cinch — may couple to the headless-reconsider pace.

### 2026-06-16 11:46 - ₢BZAAH - n

Levy federation contract content + rbgw capability-set shared module. (The bare RBSDE->RBSMF rename was swept early into concurrent commit 9a41c1c40, leaving HEAD's RBS0 include dangling; this notch carries the real content and repairs the include.) Folded three founding steps into the manor-seated levy spec RBSMF — mantle SA establishment, settle gate, AR Data-Access audit-log enablement — referencing the code-homed capability-sets per ACG; RBS0 include repointed to RBSMF + section intro updated; acronym map RBSMF relocated to manor seat + RBGW seated; rbgc RBSDE->RBSMF comment. Relocated the three per-role capability-set functions out of rbgg/rbgp into new shared library rbgw_capabilities.sh (public rbgw_grant_{governor,director,retriever}_capabilities, verbatim bodies); repointed three enrobe callers; wired both CLI furnish graphs + added rbgd_depot to payor CLI. Shellcheck 215 clean. Additive; levy code-body steps follow.

### 2026-06-16 11:37 - Heat - S

compearance-headless-reconsider

### 2026-06-16 10:57 - ₢BZAAI - n

Seat the RBRF federation regime in the specs, contract-first per the documentation-strategy cinch. New RBSRF-RegimeFederation.adoc subdoc mirrors the RBSRP payor-regime shape (//axhrb_regime + per-variable //axhrv_variable + //axhro_kindle/validate/render). In RBS0: a Federation Regime (RBRF) section under the regime family seats [[rbrf_regime]] (//axvr_regime axf_bash axrd_file_sourced axrd_singleton) plus all 10 variable quoins inline, each voiced current-form //axrg_variable axtu_string (NOT the legacy //axl_voices its frozen RBRA/RBRP siblings carry — same MCM-current discipline ₢BZAAG used for the premise); 11 mapping-section attribute refs added; RBSRF include::'d proximal to the payor regime. Acronym-map RBSRF entry added for new-file findability (flagged: the broader acronym-map vocabulary refresh remains the M6 narrative-docs concern per ₢BZAAG's agent-context-mirror strategy; this is a net-new registration, not a stale-vocabulary edit). Verified: 11 rbrf anchors balanced against 11 mapping refs, every {rbrf_*} usage mapped (no dangling), shared {rbkro_*} terms resolve, include present.

### 2026-06-16 10:50 - ₢BZAAI - n

Stand up the RBRF federation regime — the manor-scoped (org-level) IdP-trust + workforce-pool config home the accessor's federation legs will read. New rbrf.env seeded to the standing spike trust (committed: zero secrets, every value a public identifier); rbrf_regime.sh with a 10-field enroll + enforce (pool/provider naming with gcp- reservation, https endpoints, numeric org, and the no-offline_access scope guard wiring the human-present premise into the regime boundary); rbrf_cli.sh + rbw-rfr/rbw-rfv render/validate tabtargets; zipper registration; RBCC_rbrf_file constant + Rust projection. Regenerated rbtdgc_consts.rs (RBTDGC_RBRF_FILE) and the tabtarget-context md via the build. Authored as the COMPLETE manor founding set (accessor-subset pool/provider/client/scope/endpoints + affiance's org/issuer/mapping/session-duration) so affiance later rewrites values, never schema; IdP endpoints are explicit fields so a non-Entra IdP needs only new values, no code change. The RBRM seat was already taken (Regime Machine / Podman VM supply chain); minted RBRF (Regime Federation). Validates 10/10 PASS, shellcheck 214 clean.

### 2026-06-16 10:19 - ₢BZAAG - W

Seated the federation civic vocabulary as RBS0 quoins (head of the verb movement). New rbtf_ category (RB Term Federation); === Federation Civics section seating all 13 elected words as quoins with definition sites (mantle, citizen, compear, assize, don, brevet, unseat, attaint, affiance, rehearse, muniment, census, plus the terrier noun as a shell with internals parked to terrier-live). rbsk_human_present premise voiced //axk_premise. In RBSTR: minted and registered the three Terrier-access sub-op quoins engross/expunge/peruse (grep-gate clean, muniment-room register), fixed stale invest/divest -> brevet/unseat, wired the subdoc into RBS0 via include::[leveloffset=+1] proximal to the terrier noun. Voicing: mantle->axo_role, citizen->axo_actor, premise->axk_premise; acts and session/record/population nouns left unvoiced rather than guess a motif. Verified: 16 rbtf anchors balanced against 16 xref targets, 0 unresolved usages, no duplicate anchors. Deferred (flagged): claude-rbk-acronyms.md RBSTR entry stays stale until the M6 narrative-docs movement per the paddock's agent-context-mirror strategy. Code committed in notch 5c212954b.

### 2026-06-16 10:19 - ₢BZAAG - n

Seat the federation civic vocabulary as RBS0 quoins (head of the verb movement). New rbtf_ category (RB Term Federation), declared inline in the mapping section per the rbsi_/rbsq_ precedent. New === Federation Civics section under Term Definitions seating all 13 elected words as quoins with definition sites: mantle, citizen, compear, assize, don, brevet, unseat, attaint, affiance, rehearse, muniment, census, plus the terrier noun as a shell (internals NOTE'd as parked to the terrier-live pace). rbsk_human_present premise under Key Premises, voiced //axk_premise (MCM-current form, not the legacy //axl_voices of its frozen siblings). In RBSTR: minted and registered the three Terrier-access sub-op quoins engross/expunge/peruse (grep-gate clean, muniment-room register), fixed stale invest/divest -> brevet/unseat per operator ruling, updated the not-yet-wired note to wired(M4); RBS0 include::'s RBSTR with leveloffset=+1 proximal to the terrier noun. Voicing policy: only the two unambiguous identity nouns voiced (mantle->axo_role, citizen->axo_actor) and the required premise; acts and session/record/population nouns left unvoiced rather than guess an AXLA motif. Verified: 16 rbtf anchors balanced against 16 xref targets, 0 unresolved {rbtf_*} usages, no cross-file duplicate anchors, premise anchor+ref present. Flagged not-edited: claude-rbk-acronyms.md RBSTR entry remains stale (invest/divest + write/withdraw/read) -> left for the M6 narrative-docs movement per the paddock's agent-context-mirror strategy.

### 2026-06-16 09:54 - ₢BZAAF - W

Full-depth cult-rename: vacated invest/divest/mantle (cult keyfile-SA sense) for the enrobe/defrock garment register across every surface — bash (rbgg_governor, rbgp_payor z_govsa_* locals, rbgi_iam, rbyc_common, rbcc_constants, rbgc_constants, handbook wrappers), zipper consts + 5-tabtarget colophon family rbw-a{M,rI,adI,rD,adD}→{E,rE,adE,rF,adF}, regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md, 7 theurge Rust fixtures, 5 git-mv'd cult specs (RBSDK/RBSRK _enrobe, RBSDD/RBSRD _defrock, RBSGM governor_enrobe) + RBS0 op-quoins/colophon-doctrine(E/F)/includes, acronym map. Roster left intact. Preserved as non-cult: federation mantle role-noun, English investigat*, Windows <investiture> arg, canonical-divest-delete-flap memo citation. Verified: grep-gate clean of cult sense, shellcheck 212 clean, theurge unit tests 161/0, fast 109/0, regime-poison 32/0. Work committed at dd7756423 (notch, size_limit 130000 operator-approved for legitimate wide-rename diff). Deferred & flagged for separate passes: (1) RBSTR-Terrier.adoc + its acronym entry — stale federation invest/divest→brevet/unseat; (2) CLAUDE.consumer.md — divergent diplomatic verb scheme (mantle/knight/charter/forfeit), pre-existing mismatch with code, needs its own vocabulary reconciliation.

### 2026-06-16 09:49 - ₢BZAAF - n

₢BZAAF cult-rename (full-depth): vacate invest/divest/mantle (cult keyfile-SA sense) for the enrobe/defrock garment register across every surface. Bash: rbgg_governor, rbgp_payor (z_govsa locals), rbgi_iam, rbyc_common (RBYC_ENROBE/DEFROCK), rbcc_constants (RBCC_verb_enrobe/defrock), rbgc_constants, handbook wrappers rbhogw/rbhopw. Zipper consts + colophon family rbw-a{M,rI,adI,rD,adD} -> rbw-a{E,rE,adE,rF,adF} with 5 tabtarget git-mv (PayorEnrobesGovernor, Governor{Enrobes,Defrocks}{Director,Retriever}). Regenerated rbtdgc_consts.rs + claude-rbk-tabtarget-context.md via the build. Theurge Rust fixtures (rbtdrk/rbtdrp/rbtdrc/rbtdrm/rbtdrd/rbtdtk/rbtdro). Specs: git-mv 5 cult specs (RBSDK/RBSRK ..._enrobe, RBSDD/RBSRD ..._defrock, RBSGM ...governor_enrobe) + bodies, RBS0 op-quoins (rbtgo_*, rbsk_enrobe_serialized) + colophon-letter doctrine (E/F) + include:: lines, surgical fixes in RBSCIP/RBSCIG/RBSCB/RBSHR, RBSGS/RBSDE op-quoin refs. Acronym map 5 entries. Preserved as non-cult: federation mantle role-noun (rbgg/rbgp capability-set comments), English investigat*, Windows <investiture> arg, canonical-divest-delete-flap memo citation. Deferred for separate cleanup (flagged): RBSTR federation-stale invest/divest, CLAUDE.consumer.md divergent diplomatic verb scheme. Grep-gate clean of cult sense; shellcheck 212 clean; theurge builds green. (size_limit raised to 130000 with operator approval — legitimate wide-rename diff, no binary/bulk.)

### 2026-06-16 08:32 - Heat - d

paddock curried: brevet/unseat word replacement — operator-sanctioned paddock edit

### 2026-06-16 01:26 - ₢BZAAE - W

Capability-sets as named code (movement 2). The three per-role resource-grant lists are now named functions, each (token, member_email): zrbgg_grant_retriever_capabilities + zrbgg_grant_director_capabilities (rbgg_governor.sh), zrbgp_grant_governor_capabilities (rbgp_payor.sh). The invest bodies (rbgg_invest_retriever/director) and the governor mantle (rbgp_governor_mantle) consume the named definitions instead of inlining grants. Behavior-preserving transcription: member email is the only per-call variable; SA-creation and the RBRA mv message stay in the invest bodies (M7 estate); no grant added or removed; key machinery untouched. Form decision (operator-approved): function-per-role, not a declarative roll — the director set is heterogeneous (3 project grants + Mason/self actAs + self-actAs read-back poll + the complete AR repo-policy ceremony) and a flat list cannot hold it whole. Home: extract-in-place (director/retriever in rbgg, governor in rbgp); co-location into one capability-sets home deferred to M4 when levy wires them and mantle-SA homing is decided. At M4 these functions become the mantle SAs' levy-time grant lists. Verification at HEAD 9bb827f96: shellcheck 212 clean; service 152 passed / 0 failed / 0 skipped (18 fixtures). Coverage note (operator-accepted): service tests resulting access, not re-invest, so the new functions are unexecuted until the heat's next gauntlet (release-qualification or M4 levy) — a conscious low-risk deferral for a statically-verified, behavior-identical transcription; gauntlet not fired specially for this pace.

### 2026-06-15 22:19 - ₢BZAAE - n

Lift the three per-role capability-sets (resource-grant lists) into named functions: zrbgg_grant_{retriever,director}_capabilities in rbgg_governor.sh, zrbgp_grant_governor_capabilities in rbgp_payor.sh, each signature (token, member_email). The invest bodies (rbgg_invest_retriever/director) and the governor mantle (rbgp_governor_mantle) now call the named definitions instead of inlining grants. Behavior-preserving transcription — member email is the only per-call variable; SA-creation and the RBRA mv message stay in the invest bodies (M7 estate). The director set is kept whole as a function (heterogeneous: 3 project grants + Mason/self actAs + self-actAs read-back poll + the complete AR repo-policy ceremony) rather than a flat list — a roll could not hold it. At M4 these become the mantle SAs' levy-time grant lists. Shellcheck 212 clean; no orphaned locals (functions use only params + kindle constants). service verification follows this notch to clear the theurge clean-tree gate; note service exercises the resulting access not re-invest, so gauntlet is where the functions truly run.

### 2026-06-15 21:41 - ₢BZAAD - W

Accessor seam landed (movement 1). One identity-keyed accessor — rba_token_capture <governor|director|retriever> in rba_auth.sh — is now the sole token mint (only call to rbgo_get_token_capture outside its definition). 55 direct-mint sites routed through it (23 director, 6 retriever, 26 governor), collapsing the old rba_get_governor_token_capture wrapper and dropping the dead path-keyed rba_authenticate_role_capture. The rbgv access-probe routed through the accessor; its redundant zrbgv_role_rbra_file_capture removed. rba wired into the rbfc0/rbfr/rbob CLI kindle graphs (their mint modules previously kindled only rbgo). Behavior-preserving keyfile-bridge wrapper; credless return-$? band-code discipline preserved (rbtdrf_rs_credless_guard_mint_refusal green). BCG-grounded: capture-function shape, defensive ${1:-} param, plan-vocabulary kept out of code comments. Verification at HEAD 3016c72ab: mint grep gate clean, shellcheck 212 clean, service 152 passed / 0 failed / 0 skipped (18 fixtures — all three identity mints, lifecycles, batch-vouch, regime-poison). Scope: M1 cut at the mint; ~40 source/test-f RBDC_*_RBRA_FILE reads (RBRA_CLIENT_EMAIL/PROJECT_ID, key-file management) left to the M7 RBRA-estate retirement, per docket and pace-design memo.

### 2026-06-15 20:46 - ₢BZAAD - n

Collapse every credential token-mint behind one identity-keyed accessor (rba_token_capture) in rba_auth.sh; the accessor is the sole rbgo_get_token_capture call. Route 55 sites through it (23 director, 6 retriever, 26 governor), collapsing the old rba_get_governor_token_capture wrapper and dropping the dead path-keyed rba_authenticate_role_capture. Route the rbgv access-probe through the accessor and remove its redundant zrbgv_role_rbra_file_capture. Wire rba into the rbfc0/rbfr/rbob CLI kindle graphs (those mint modules previously kindled only rbgo). Behavior-preserving bridge wrapper of the keyfile mint; credless return-$? band-code discipline preserved. Mint grep gate clean (rbgo_get_token_capture only in accessor + definition) and shellcheck 212 clean. RBRA-estate source/test-f reads left to M7 per scope. service verification follows this notch to clear the theurge clean-tree gate.

### 2026-06-15 20:14 - ₢BZAAR - W

Known-good floor recorded at heat HEAD e9ce49924 (clean tree, verified no drift between the two runs). complete: GREEN — 222 passed / 0 failed / 0 skipped across 21 fixtures (fast validation tier, service lifecycle tier incl. the JWT governor/director/retriever + payor OAuth credential probes, and the crucible tier: tadmor 61-case security suite, srjcl 3, pluml 6). dogfight: GREEN — 6 passed / 0 failed / 0 skipped across 2 fixtures (cult-verb mantle/invest/divest lifecycle + rbtdrd build_run cloud-build ordain->summon->run). No red to disposition; the red-baseline-blocks-movement-1 clause does not trigger. blockade left out of scope (operator call). This is the durable floor on the credential surface the heat rewires; movement-1 credential-accessor refactor is cleared to mount against this baseline.

### 2026-06-15 18:47 - Heat - S

known-good-floor

### 2026-06-15 18:07 - Heat - S

placeholder-m7-rbra-retirement

### 2026-06-15 18:07 - Heat - S

placeholder-m6-narrative-docs

### 2026-06-15 18:06 - Heat - S

placeholder-m5-manor-regroup

### 2026-06-15 18:06 - ₢BZAAC - W

Ran the high-effort balanced-merits adversarial review of the seven first-cut movement-4 paces (27-agent workflow). Recorded findings in memo-20260615-BZ-m4-review-findings.md (the Fable-review entry point). Four of five judgment calls held the first cut; the levy-founding pace split into manor-affiance + levy-mantle-establishment, taking M4 to eight paces. Firmed all eight dockets to mount-ready and reslated them. Resolved the M4-vs-M7 suite-gate open question to the M7-coupled reading, with residual risk recorded for Fable. Seated the terminal fable-heat-review pace. Paddock left frozen per discipline; the Fable hook rides the terminal pace.

### 2026-06-15 17:59 - Heat - S

fable-heat-review

### 2026-06-15 17:59 - Heat - S

manor-affiance

### 2026-06-15 17:58 - Heat - T

levy-mantle-establishment

### 2026-06-15 17:56 - ₢BZAAC - n

Record the movement-4 review findings memo (balanced-merits adversarial workflow output): four of five judgment calls held the first cut; levy-founding split into two paces; the M4/M7 keyless-fixture open question resolved to the M7-coupled reading with residual risk recorded for Fable. Pointer added from the pace-design memo. Paddock left frozen per discipline; the Fable hook rides a terminal review pace instead.

### 2026-06-15 11:44 - Heat - S

admission-verbs-polity

### 2026-06-15 11:44 - Heat - S

terrier-live

### 2026-06-15 11:44 - Heat - S

accessor-don-leg3

### 2026-06-15 11:44 - Heat - S

accessor-compearance-assize

### 2026-06-15 11:43 - Heat - S

levy-founding-gestures

### 2026-06-15 11:43 - Heat - S

rbs0-civic-quoins

### 2026-06-15 11:43 - Heat - S

cult-rename-enrobe-defrock

### 2026-06-15 11:34 - ₢BZAAC - n

Record the closing three slate-time picks in the BZ pace-design memo: handbook rework homed to A6 (chivvied separately), rbw-gq kept a guide but regrouped under Payor as rbw-gPQ at the M5 regroup, and the two 2026-06-11 beta evidence memos confirmed already present in this repo (migrate-vs-point-at moot). Closes the 260615 slate-time-pick trot — 6/6 resolved.

### 2026-06-15 10:47 - ₢BZAAC - n

BZ pace-design trot (260615): record the terrier write-semantics and evolve RBSTR from a contract-first stub (parallel-minted under BeAAD) to a standing terrier sub-operation subdoc — atomic write/withdraw/read of a muniment via GCS preconditions, with invest/divest/rehearse as thin RBS0-side wrappers and the terrier noun as a separate RBS0 civic quoin at M4; mint enrobe/defrock (garment-of-office register) for the M4 cult-colophon rename; resolve the test-rig credential to per-run human click. Memo records all three resolutions; acronym-map RBSTR entry updated to the sub-op framing. Touches only this session's three files; leaves the parallel chat's ACG/AXLA/MCM/RBS0 work untouched.

### 2026-06-15 08:42 - ₢BZAAC - n

Update pace-design memo: freeze landed at paddock head; M1 (accessor-seam) and M2 (capability-sets-as-code) now slated

### 2026-06-15 08:41 - Heat - S

capability-sets-as-code

### 2026-06-15 08:40 - Heat - S

accessor-seam

### 2026-06-15 08:40 - Heat - d

paddock curried: freeze banner — Opus stewarding Fable-authored paddock; read-only pending Fable/operator sanction

### 2026-06-15 08:30 - ₢BZAAC - n

Seed the BZ pace-design memo: the Opus/Fable membrane, forming pace succession (M1-M2 cuttable now, M4+ after slate-time picks), code-grounding from the credential/cult-verb read, and the divergence-candidate ledger

### 2026-06-15 08:30 - Heat - S

pace-design-membrane

### 2026-06-12 11:13 - Heat - d

paddock curried: review-walk rulings absorbed: overhang embraced with deferral trigger, audit demoted to spec-interior, affiance first-or-further, last-governor removal legal with payor recovery, attaint alone sweeps the F2 binding, spike fixtures kept as standing test trust; six Open items resolved and cleared

### 2026-06-12 11:01 - Heat - d

paddock curried: spike-findings absorption + review repairs: F2 two-binding admission swept through architecture/verbs/cinches, V1 two-ceiling assize model, V3 downstream attribution + levy audit-log enablement, F1 workforcePoolAdmin seating, session-cache election absorbed into custody, spike verification items cleared from Open, six new decision items opened

### 2026-06-12 10:46 - ₢BZAAB - W

Operator-facing vocabulary for the federation model elected in design conversation, every candidate grep-gated and cold-probed: mantle (standing-role noun, after office failed its probe), compear/assize (the daily pair, re-minted rarer after the word-husbandry ruling), don (impersonation act), invest/divest/attaint (admission surface, 2+1 shape, cult-word reuse), affiance (IdP-trust founding verb), rehearse (read-Terrier), muniment (terrier-entry noun); citizen survives, census stands spaced. Classification ledger and full placeholder sweep landed in the paddock with the mantle homonym guard. MCM Word Selection gained four rules discovered live: one word per concept, word husbandry, sibling initials (verbs orbiting one noun are siblings from birth — caught the four-E verb set), broadside mirror

### 2026-06-12 10:44 - Heat - d

paddock curried: vocabulary pace close-out: all seven remaining words confirmed, classification ledger landed, office/session placeholder prose swept to the elected constellation throughout

### 2026-06-12 10:40 - ₢BZAAB - n

Sibling-initials rule strengthened per operator catch: verbs orbiting one critical noun are siblings from birth, abbreviation namespace or not — verb families acquire one eventually, and by then the words are minted. Closes the gap where a verb family minted before its colophons arrive would dodge the distinct-initials check

### 2026-06-12 10:35 - Heat - d

paddock curried: vocabulary elections: mantle/compear/assize elected, invest/divest/attaint/don/muniment probed, homonym guard, sibling-initials supersedes four-E verb set

### 2026-06-12 10:31 - ₢BZAAB - n

MCM Word Selection gains four rules discovered live during the office-federation vocabulary mint: one-word-per-concept (the dual of semantic uniqueness — no alias quoins, near neighbors spaced by stated distinction), word husbandry (commonness matches traffic — plain words conserved for daily surfaces, rare ceremonies take colorful words; ceiling not floor), sibling initials (distinct first letters wherever siblings share an abbreviation namespace — colophon suffixes, sub-letters; word-side twin of sub-letter monosemy), and the broadside mirror line (agent context files may mirror the broadside; derived, never canonical)

### 2026-06-12 10:03 - ₢BZAAA - W

Three-leg office-federation chain proven live end-to-end in pure curl/jq against the standing depot: Entra device flow (no refresh token, D2 live-verified), STS exchange, generateAccessToken, office-token bearer call listing depot GAR. All five verification items answered with evidence; session-cache shape elected (federated token per-session in tmpfs scratch, office tokens per-verb in-process, headless fail-loud). Standing test trust candidate created: pool spike-office-test + provider spike-entra + office SA. Two paddock-grade flags recorded, not absorbed: admission requires a second binding (serviceUsageConsumer on depot for the quota project) and mint audit records omit the federated caller — attribution lives downstream via serviceAccountDelegationInfo, gated on per-service audit-log enablement at levy and correct log class. Findings memo: memo-20260612-federation-legs-spike-findings.md

### 2026-06-12 09:58 - ₢BZAAA - n

Findings memo gains a Why-this-spike section: the conversion bet the single-tier model on a never-executed runtime chain, and this spike is the ordering's gate for the verb movement — wobbles surface at throwaway-script cost before the civic verbs and accessor branch are slated

### 2026-06-12 09:55 - ₢BZAAA - n

Federation legs spike findings banked: the three-leg office-federation chain proven live end-to-end in pure curl/jq against the standing depot (Entra device flow with no refresh token, STS exchange needing nothing extra, generateAccessToken, office-token bearer call dodging the quota-project ceremony downstream). All five verification items answered with evidence; session-cache shape elected (cache the federated token per-session, mint office tokens per verb-run, headless fail-loud). Two surprises flagged against the paddock rather than absorbed: admission is two bindings not one (serviceUsageConsumer on the depot is required for Leg 3's quota project), and the always-on mint audit record omits the federated caller — attribution lives in the downstream serviceAccountDelegationInfo entry, which requires per-service audit-log enablement at levy and the right log class (ListRepositories is ADMIN_READ).

### 2026-06-12 09:15 - Heat - S

ashlar-vocabulary-mint

### 2026-06-12 08:08 - Heat - n

Two evidence memos from the 260611 federation-architecture conversation banked: Google's published preference ordering for SA impersonation over keys (last-resort framing, default org-policy enforcement since May 2024 — the office-SA proposal composes two recommended mechanisms), and the token-custody layer model L1-L4 with server-side context enforcement (podman credHelpers parity confirmed; correction flagged: device-based access levels unavailable under Workforce Identity Federation, so the CBA device-bound layer is off the table for federates — IP/time conditions and VPC-SC remain the working server-side layers)

### 2026-06-12 08:06 - Heat - n

Office-federation conversion decision record; canon banner: D1 retired, direct grants superseded by office impersonation

### 2026-06-12 08:06 - Heat - f

silks=rbk-14-office-federation

### 2026-06-12 08:06 - Heat - S

federation-legs-spike

### 2026-06-12 08:05 - Heat - d

paddock curried: office-federation conversion: single tier, citizen keyfile tier canceled unbuilt

### 2026-06-10 11:00 - Heat - d

paddock curried: documentation strategy cinched: contract-first in-pace specs, slate-time acronym mint, RBS0 quoins at movement-3 head, narrative docs as movement 5; ordering now six movements

### 2026-06-10 10:45 - Heat - d

paddock curried: standalone-probe fixes + assay retirement per canon D4 + movement-1 seam scope (five constants, RBRO, facts+tokens, read-side gate)

### 2026-06-10 10:20 - Heat - d

paddock curried: colophon family election: rbw-p polity + rbw-m manor regroup in-heat, rbw-P rejected; ordering gains the regroup movement

### 2026-06-10 10:13 - Heat - n

Discharge the civic-verb breadcrumb owed to the bedrock Quire memo: §12's chorister verb list reads enfranchised/enfeoffed/escheated/expelled with a re-mint pointer at the paddock (MCM Word Selection, 260610), the establishment-verbs parenthetical drops the retired cult verbs for the civic set, the recut analog line follows, and the §15 ₣BZ source bullet carries the re-minted words. Closes the staleness the paddock's 260610 revision queued.

### 2026-06-10 10:08 - Heat - d

paddock curried: civic verb re-mint under MCM Word Selection (enfeoff/escheat/recut) + terrier singular reframe, Terrier bucket, managed-folder grain, read-population settlement

### 2026-06-10 09:19 - Heat - n

Civic homogenization election + standalone deployment ladder + owed breadcrumbs, closing the bedrock-quire shaping session. Quire memo: §9's instance-vs-menu question is answered and deleted as an open item — Quire is the endpoint-side envelope, prebend the citizen-side envelope, a concrete invocation falls in their intersection. §12 gains the Cloister (consume-side AWS account premises noun; Close rejected as overloaded; blast-radius/quota/billing boundary, sovereign-setup sibling of depot and embassy, cardinality lean one-per-operator) and the elected civic homogenization: the Cloister is a polity in the BZ civic structure whose enforcement plane is AWS IAM, its ledger file joining the Manor-bucket Terrier collection (admin-plane-only cross-cloud writes priced — working verbs never touch GCP); choristers are enfranchised/granted/revoked/expelled (invest/divest corrected away per the civic verb retirement); precentor follows the governor dissolution (keeps quoin, structurally citizen + capability-set); prebend resolves to the chorister's ledger entry, honoring the code-vs-data seam (set definitions code/global, held-sets-plus-allowances data/local). New §14 records the standalone posture: a three-rung deployment ladder (bare Cloister / Manor+Cloister no-Depot / full) riding federation canon D1's tier enum across the cloud boundary, the chorister-as-retriever provenance answer with named residuals, legibility-before-tunnel sequencing for the Outpost, and the decoupling invariant (Quire/Cloister kindle chain never sources a GCP regime). §10 queues the probe verb (candidate: intone); Sources gain the authoritative BZ-paddock citation. RBSHR: the conduit/chantry/credential-custody entries are breadcrumbed to the Quire memo (WireGuard-in-sentry marked rejected by §11's terminus; chantry's native-consume half marked renamed to Quire) and the VPC-SC cross-reference is corrected (AWS network vs GCP perimeter — neither serves the other). The two federation memos gain the supersession breadcrumbs the BZ paddock owed: memo-20260605 a superseded-in-part banner (Terrier homing, census, mantle, migration), federation-canon an authoritative-homing note at its depot-remains-the-home line — closing the staleness trap that misled this session's first Terrier search.

### 2026-06-10 08:37 - Heat - n

Transport-seam addendum (§13) to the Bedrock Quire memo: why bash-for-AWS-auth is harder than the GCP precedent and where the difficulty dissolves. Records the GCP baseline (rbgo's JWT-bearer flow front-loads all crypto into one per-session mint — document-coupled, already at its minimal seam), then SigV4's four strains (per-request crypto cardinality, binary HMAC intermediates vs NUL-free bash strings, canonicalization byte-exactness failing as remote opaque 403s, sign-the-reconstruction two-sources-of-truth with curl building the wire bytes). The probable resolution is curl's native --aws-sigv4 signing: the wire-byte builder signs, strains 2-4 dissolve, bash never touches an HMAC — priced honestly as a curl version floor, a Palisade dependence with surveyed signer bugs (#10129, #11007, wildcard paths), and imperfect trust transfer to a less-trodden surface within an already-trusted neighbor. Names ARN-in-path on the runtime probe as the one RB-relevant risk shape. Cinches spike-before-build (two curl calls: clean control-plane + ARN-bearing probe shape) with hand-rolled hex-chained SigV4 as named-fallback-not-built, testable against AWS's published vectors. Closes with the seam principle: transport-coupled crypto belongs in the transport tool, document-coupled crypto at openssl — the two clouds correctly get different seams, and curl offers nothing that would simplify the existing GCP path (--oauth2-bearer is cosmetic; --retry cannot see the response bodies the load-bearing rbgo retry loops discriminate on). §10 gains the spike as a cinched open item; §12 resident-program paragraph points at §13; Sources renumbered §14 and gain the curl/AWS verification.

### 2026-06-10 08:29 - Heat - n

Consume-side governance addendum (§12) to the Bedrock Quire memo: the Precentor/Chorister/Prebend role nouns riding the Quire's ecclesiastical register (precentor directs the quire; chorister voices within bounds; prebend is the medieval endowed-allowance, here the per-chorister envelope of model list + agency ceiling + spend allowances). Frames the hierarchy as RBSHR's pre-authorized 'deliberate parallel role hierarchy' arriving consume-side, with the diplomatic register (embassy/envoy) reserved for delivery-side so the two AWS efforts stay distinguishable by ear; shape is one administering role + N citizen principals dividing people/workloads (₣BZ citizen model on AWS), unlike the function-dividing GCP trio. Grounds enforcement in verified AWS facilities as a four-rung reaction-time ladder (AIP cost-attribution tags with the non-retroactivity hazard, Budgets auto-deny actions at day grain, CloudWatch token metrics + Lambda deny at minutes grain privacy-clean, counting proxy as named non-goal whose body would be the §11 terminus) and keeps cap vocabulary honest: prebends declare soft/firm, with hard reserved for the nonexistent proxy tier. Records the anti-crap-wrap rationale (ceremony codification — AWS supplies primitives, composes none; establishment-order guarantees, roster, declared-vs-actual drift audit) and the no-resident-program finding, naming SigV4 per-request signing rather than the caps as the real bash strain. Prebend-as-menu strengthens §9's envelope resolution. Sources gain the 2026-06-10 AWS verification; old §12 Sources renumbered §13; §10 minting queue gains the three nouns.

### 2026-06-09 18:30 - Heat - d

paddock curried: 260609 Fable session: civic structure, per-depot ledger Terrier, substrate ordering, human-present premise

### 2026-06-09 11:10 - Heat - d

paddock curried: Rename Census -> Terrier; purge 'ledger' as a competing name (Option 2); append 260609 naming breadcrumb

### 2026-06-09 11:05 - Heat - n

Rename the federation declared-ledger concept Census -> Terrier across the three federation memos, and purge 'ledger' as a competing name (Option 2, elected this groom): Terrier is the sole name, the redundant 'declared' modifier drops, and the term-of-art compounds convert (Terrier-write, read-Terrier, Terrier-read, withdraw-Terrier, non-Terrier). The citizen-capability-model memo is the bulk (40 ledger sites -> Terrier; the short-form scaffolding dropped). federation-canon swaps its 2 'Census declared ledger' refs. bedrock-quire records the Census/Crucible C-initial collision as resolved (Census -> Terrier) rather than carrying it as deferred minting work. Driver: Census is semantically apt but C-initial (collides with Crucible); Terrier (manorial register of holdings, pairs with the Manor) carries the citizen-roll-with-rank meaning without the collision. No code, spec-quoin, or config touched -- the concept is pre-MVP and doc-only.

### 2026-06-09 10:50 - Heat - n

Addendum (§11) to the Bedrock Quire memo: the Outpost's local tunnel terminus as a second launch shape beside the crucible. Rejects two tempting homes for the tunnel's local end — WireGuard-in-sentry (inverts the sentry's containment-harness purpose: trusted infra vs. jailed prisoner) and a bottle-less crucible (category error: a jail with no prisoner, abstraction strain to borrow lifecycle plumbing). Adopts the local terminus as the client-side counterpart of the EC2 peer, built as its own vessel and launched standalone, so the Outpost gains two ends both under raise/strike with no new verbs or top-level concept — finishing the migration away from WireGuard-in-sentry. Honors the operator's 'no host WireGuard' constraint: the tunnel client lives in a container, config riding inside the hallmark, host never joins a VPN. Fast-follow only; MVP (allowlist reach, sentry egress-lockdown, no Outpost/VPC) untouched per §9's no-touch-the-MVP rule. Names the one open question in §10 — terminus consumer class: container-only (plain container networking) vs. host-resident (needs a local forward-proxy/endpoint seam, still no host VPN), with container-only the lean. Renumbers Sources to §12.

### 2026-06-09 10:23 - Heat - n

Capture Bedrock consume-side shaping as a design memo riding the citizen/capability identity model: the consume-not-host scope and the 'Bedrock is not an instance' correction, why RB holds the persistent state (Bedrock statelessness), the six-boundary privacy model with the new server-side-agency boundary F and the Palisade framing, the no-submit rule (RB establishes the binding, the workload invokes), the Quire/Outpost vocabulary (endow/dissolve, raise/strike) with the two-tier lifecycle, the three-layer config with the agency clearance lattice (none/internal/internet) as boundary F's IAM-enforced home, the MVP allowlist -> Outpost/PrivateLink fast-follow staging with the three VPC triggers, the cost assessment (~$15/mo Outpost floor, NAT as the one trap, inference dwarfs it), and the full JJK-choreography horizon (Quire identity as governed invocation principal, the menu-vs-instance question with the menu/envelope resolution, and privacy-as-graph-property with the agency lattice as clearance lattice). Includes an RBSHR VPC-SC cloud-conflation correction note.

### 2026-06-09 07:07 - Heat - n

Consolidate the federation tier into a single canon memo (memo-20260609-federation-canon). Folds memo-20260527 in whole and supersedes R4 of memo-20260522 per the operator's human-always-present decision: the federation tier now persists no refresh token and is fully secret-free (keyfile holds one durable secret, the SA key; federation holds zero). Canon records D1-D6 incl. the identity-keyed accessor, the regime-home XOR authorship rule, RBRS_CITIZEN as the human-edited identity selector, RBRA going full-auto, and Governor-administered token lifetime in tamper-evident RBRD. Added superseded banners to memo-20260527 and memo-20260522 R4; repointed the RBSHR Operator-federation roadmap reference at the canon. Diagram Source: repointing deferred (container re-render).

### 2026-06-09 07:07 - Heat - n

Consolidate the federation tier into a single canon memo (memo-20260609-federation-canon). Folds memo-20260527 in whole and supersedes R4 of memo-20260522 per the operator's human-always-present decision: the federation tier now persists no refresh token and is fully secret-free (keyfile holds one durable secret, the SA key; federation holds zero). Canon records D1-D6 incl. the identity-keyed accessor, the regime-home XOR authorship rule, RBRS_CITIZEN as the human-edited identity selector, RBRA going full-auto, and Governor-administered token lifetime in tamper-evident RBRD. Added superseded banners to memo-20260527 and memo-20260522 R4; repointed the RBSHR Operator-federation roadmap reference at the canon. Diagram Source: repointing deferred (container re-render).

### 2026-06-08 20:21 - Heat - n

Nest the four federation-diagram <details> blocks inside the Operator Federation list item by indenting them to the bullet's content column (2 spaces), so they render under the bullet instead of fully outdented at the page margin. Collapse each block to one contiguous HTML block (drop the now-unneeded internal blank lines, since the summary holds the label and nothing inside needs markdown rendering).

### 2026-06-08 20:06 - Heat - n

Split the federation-diagram embeds into four independent default-folded <details> blocks (one per diagram) instead of one combined expander. Each summary now carries the diagram's label, so the folded view still names all four sequences; the <picture> theme-aware embed sits inside its own block and renders on expand.

### 2026-06-08 20:01 - Heat - n

Mint the rbdg diagram family and give the federation diagrams light/dark theme-aware rendering. Rename the four federation-tier PlantUML pairs to rbdg-prefixed handles (rbdgl federation-login, rbdgs federation-setup, rbdgk keyfile-login, rbdgm federation-seam) and register RBDG as a non-terminal family in claude-rbk-acronyms.md. Add a pure zrbtdrc_darken_svg recolor (surveyed PlantUML default-skin palette to dark-canvas equivalents, page background dropped to transparent) invoked in the rbtdrc_pluml_render_diagrams crucible case right after each render to emit *-dark.svg siblings with no second container trip, plus a crucible-free unit test proving the map and its characterized pass-through. README: the four diagram links become inline theme-aware <picture> embeds (light default, dark via prefers-color-scheme), folded in a <details> block to keep the Roadmap appendix scannable. Dark SVGs bootstrapped via the identical transform; the pluml case is the canonical generator going forward, glob-driven so a new rbdgX_*.puml renders both modes with no second list.

### 2026-06-08 12:15 - Heat - n

Add federation-model critical-sequence diagrams (PlantUML source + rendered SVGs) and a generative pluml crucible fixture case that renders them. Four sequences drawn from memo-20260527: federation login (two-leg device-flow + STS exchange), federation setup (Payor-side workforce pool/provider), keyfile login (contrast), and the single code seam (mode-enum branch). New case rbtdrc_pluml_render_diagrams re-renders diagrams/*.puml to committed SVGs via the live PlantUML server and asserts well-formed SVG. README Operator Federation roadmap entry links the four diagrams. Experiment scaffolding for BZ citizen-tier design orientation toward the federation target.

### 2026-06-08 08:54 - Heat - n

RBS0: introduce Manor as a first-class quoin (rbtgi_manor) — the singular, one-of Payor-project administrative seat (hosts Payor SA, OAuth client, billing; control plane funding Depots), rbtgi_payor_project kept as a resolving alias. Swept payor-project references to {rbtgi_manor} across RBSDE, RBSGS, RBSPE, RBSAO, RBSPI, RBSPR.

### 2026-06-08 08:52 - Heat - d

paddock curried: 260606 naming: Manor=Payor-project (rbtgi_manor quoin), Census=ledger, verb dispositions

### 2026-06-06 12:04 - Heat - f

silks=rbk-14-mvp-citizen-model

### 2026-06-05 18:43 - Heat - n

Educational record of the ultracode multi-agent process that produced the citizen-model revision: the two background workflows (9-lens review -> adversarial verify -> synthesize, 45->26; then a 4-check self-verification that caught 2 residuals), the fan-out/adversarial-verify/synthesize patterns, the solo-editing vs fan-out division of labor, the args-bug recovery, costs (~61 agents, ~4.5M tokens), and honest limits

### 2026-06-05 18:41 - Heat - n

Fix two verification residuals in the verb-dissolution table: roster verb is retired (not mapped to ledger-read) with read-ledger added as the routine intent read and the actual-state read folded into the audit; attach the intent-first invariant to capability verbs (grant/revoke) and reduce remove-citizen to the SA delete, resolving the divest-row ordering inversion

### 2026-06-05 18:41 - Heat - d

paddock curried: verification residuals: audit reads actual IAM state vs ledger (not actual-state roster); intent-first attributed to capability verbs (grant/revoke), not remove-citizen

### 2026-06-05 18:29 - Heat - n

Apply ultracode review sweep to the citizen model: rename declared roster -> declared ledger (disambiguate from the actual-reading roster verb); add SA-naming migration section, member-first audit axis, divest ledger-withdraw-first invariant, definition-expansion human-gate, 10-key rekey constraint, concurrency etag re-read, half-failed-revoke handling, integrity-advisory orphan sweep, ledger-home-not-build-bucket; reword RBSHR artifact-citizen prose to holding; breadcrumb memo-20260527

### 2026-06-05 18:29 - Heat - d

paddock curried: apply ultracode review sweep: declared roster -> declared ledger rename; member-first audit axis, divest ledger-withdraw-first, definition-expansion gate, ledger home not build-bucket; hygiene compressions

### 2026-06-05 18:01 - Heat - n

Amend citizen-capability mechanics per review pass: add roster-write-authority invariant under auto-heal; correct repoAdmin-bears-setIamPolicy scoping; qualify surplus-as-bypass for definition-shrink; precise resting-key vs union blast-radius; idempotent-governor not-free; byte-identical -> modulo principal handle; ontology covers operators only; record third (seam role->identity keyed) divergence

### 2026-06-05 18:00 - Heat - d

paddock curried: apply review findings #1-5,7-9,11-13: roster-write invariant, repoAdmin setIamPolicy scoping, surplus-claim qualifier, compress governor+authority to shape, blast-radius precision, seam divergence, Open rephrasings

### 2026-06-05 17:32 - Heat - n

Capture citizen-tier identity/capability mechanics as the paddock's mechanism companion: vocabulary (citizen/federate/payor, capability-set, declared roster, holdings), verb dissolution, the audit drift taxonomy + asymmetric healing, orphan marker, authority topology with verified modifiedGrantsByRole facts, blast-radius reasoning

### 2026-06-05 17:32 - Heat - d

paddock curried: refactor paddock to pure shape; citizen scoped to keyfile tier (federate = federation peer); capability layer tier-blind; point to mechanics memo

### 2026-06-05 17:24 - Heat - f

silks=rbk-14-citizen-model

### 2026-06-05 16:57 - Heat - d

paddock curried: fold 260605 design conversation: identity/capability decoupling, declared roster, asymmetric audit, authority topology; retire no-stored-list cinch

### 2026-06-05 03:07 - Heat - d

paddock curried: add Inherited concern — governor teardown leak (memo-20260605 ref)

### 2026-06-05 03:06 - Heat - n

Capture the governor-mantle tombstone-leak concept as input to rbk-14: rbgp_governor_mantle deletes each outgoing governor-* SA without revoking its roles/owner first, so every re-mantle accrues a deleted:...?uid= project tombstone (8 and counting on the canonical depot) — H1's mechanism on the governor tier, deliberately scoped out of rbk-08. Frames the three-layer subsumption (targeted lean project-scope revoke-before-delete if no remodel; rbk-14's generic teardown or an idempotent governor closes it for free; federation obviates it) and the caveat that rbk-14's governor teardown must reuse the rbk-08 revoke layer + Class-C tolerance rather than reinvent.

### 2026-06-04 19:57 - Heat - f

silks=rbk-14-mvp-prepare-federation-idea

### 2026-06-04 19:40 - Heat - f

racing

### 2026-06-04 19:38 - Heat - d

paddock curried: Seed paddock: identity-credential convergence target model, federation-shape-not-mechanism cinch

### 2026-06-04 19:36 - Heat - N

rbk-15-identity-credential-convergence

