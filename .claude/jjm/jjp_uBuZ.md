## Boundary — single tier, office impersonation (cinched 260612)

This heat builds the **one** operator credential model:
workforce federation with **office-SA impersonation**.
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

Unchanged from the canon's D2:
**a human is present at the kickoff of every run; no run outlives a session cap.**
No refresh token anywhere beyond the payor's own;
an orchestrating agent never holds a secret
(device-flow kickoff surfaces only user_code + verification URI).
Belongs in RBS0 as a premise quoin voiced `axk_premise` at civic incorporation.

## The office architecture (cinched 260612)

**Capability-sets instantiate as office SAs at levy; all resource IAM freezes there.**

- Levy creates the office SAs (governor / director / retriever) beside mason
  and grants **every** resource binding (project, GAR repo, mason actAs) to them, once,
  behind a settle gate — levy is long and payor-driven; the gate is cheap there.
- Admission to an office is **one binding**:
  `roles/iam.serviceAccountTokenCreator` on the office SA, naming the operator's `principal://` subject.
  Removal is that binding's deletion.
- Runtime chain: IdP device flow (Leg 1) → STS exchange (Leg 2) →
  `generateAccessToken` on the office SA (Leg 3) → short-lived office token.
  Everything downstream bearer-blind, unchanged.
- **Zero service account keys exist anywhere**; the depot runs with
  `disableServiceAccountKeyCreation` fully enforced.
  The system's sole durable secret is the payor's RBRO refresh token.
- No per-user Google identity ever exists:
  the federated principal is the grantable identity, implicit in the IdP assertion.
  The IdP is the census (cinch carried forward).
- Flap payoff (the architecture's origin): post-levy IAM mutation collapses to
  one binding type at one scope with one gate point;
  the SA-lifecycle flap family confines to levy;
  hot paths perform zero IAM mutation.
  Office tokens are uncapped and self-expiring; the 10-key cap is irrelevant (no keys).
- An impersonated office token is a plain SA token —
  the workforce federated-token per-product support matrix is dodged entirely.
- Blast radius: a token is one office at a time;
  the multi-hat union-credential concern dissolves.

## The civic structure (cinched, carried forward with office recast)

**Payor founds, governors populate, terriers tell, IAM enforces.**

- Admission authority scoped to the polity it admits into; payor outside the citizenry,
  ceremonial after founding; founding gestures gain **establish-federation**
  (workforce pool + provider + attribute mapping, org-level, once per manor)
  and **office establishment** at levy.
- All operator administration is governor-wielded and depot-scoped.
  A governor admits another to the governor office — governors create governors.
  Cross-depot administration does not exist.
- Governor dissolves structurally: an operator holding the governor office.
  Standing governor remains the default posture.
- One-identity-across-depots arrives free:
  the same `principal://` subject is grantable in every depot under the manor.

## Multi-IdP posture (cinched 260612)

One live provider per pool is the norm.
Subjects are **pool-scoped, not provider-scoped** —
every provider is a full root of trust for the whole subject namespace,
so the pool's security floor is its weakest provider.
Adding a provider is a payor ceremony with founding gravity.
Dual-provider windows exist for IdP **migration** only (overlap, re-admit, retire) —
never as an availability hedge; live tokens already ride out IdP outages within the session cap.

## The Terrier (carried forward; entries recast)

Per-polity terriers in the Manor-homed Terrier bucket, managed-folder grain,
payor-created at levy, governor-writeable own-terrier-only,
read population governors-and-above — all unchanged.
Entries now record (principal subject, office held);
key IDs vanish with the keys.
Working verbs never touch terriers; audit is a read-only diff
(terrier intent vs the office SAs' tokenCreator bindings — resource bindings are frozen and need no audit loop).

## Verbs and orderings (recast 260612; words open)

Enfeoff/escheat survive with single-binding bodies;
enfranchise/expel survive as admission/removal compositions;
**recut retires** (the IdP owns rotation; there is nothing to rotate Google-side);
**credential delivery retires whole** (nothing is delivered; the assay machinery vacates with it).

| Verb | Body | Crash leaves |
|---|---|---|
| enfranchise | terrier write → tokenCreator binding | visible deficit; re-run (idempotent) |
| enfeoff | terrier write → tokenCreator binding on further office | same |
| escheat | terrier withdraw → remove binding | visible surplus; report-only |
| expel | escheat all offices → deregister note (IdP-side removal is the IdP admin's) | partial teardown lands as surplus, never resurrection |
| read-Terrier | pure read | — |

The verb *words* above are a working set, now superseded in part:
the four-E set (enfranchise/enfeoff/escheat/expel) fails the sibling-initials rule
MCM Word Selection gained from the vocabulary pace;
see Vocabulary elections below for the replacement words.
The table's *bodies* and crash semantics stand unchanged.

## Vocabulary elections (in progress)

Shape rulings (operator, 260612):
the operator surface carries two daily admission verbs plus one rare whole-person expulsion
(spec grain keeps all four cases as quoins);
*citizen* survives as the operator noun;
one sign-in word serves every human including the payor — the mechanism difference is hearting;
the standing-role noun is ashlar, low-traffic.

Elected:

- **mantle** — the standing-role noun (the governor, director, and retriever mantles);
  supersedes the *office* placeholder, which failed its cold probe
  (heard as place-of-work, not civic role).
- **compear** — the per-session human sign-in act
  (Scots law: to appear formally in answer to a summons; noun *compearance*).
  Not a tabtarget: compearance is an accessor step —
  any cloud tabtarget probes for a live assize, opens the device grant,
  prints the clickable URL + code, and fails loud.
  The pending-grant holder shape (who polls the device code) rides the spike's session-cache election.
- **assize** — the live-window noun: a bounded sitting that is also a fixed regulated measure,
  so the word carries the cap in its own body;
  the premise phrase becomes "a human compears at the kickoff of every run; no run outlives its assize."

Probed once (day-after story), pending operator confirmation:

- **invest / divest** — the daily admission/removal verbs,
  etymologically married to mantle (investiture: robing in the garments of office);
  cult-word reuse under the eviction-then-reuse ruling.
- **attaint** — the rare whole-person expulsion (attainder: the old law's civic death).
- **don** — the impersonation act (donning a mantle; Leg 3); one mantle worn at a time is the blast-radius cinch made audible.
- **muniment** — the terrier-entry noun (deeds preserved as proof of rights).

Mantle homonym guard (transform-scoped):
until the cult colophon family (`rbw-aM`/`arI`/`adI`/`arD`/`adD`) is renamed,
the reused words carry two senses in the repo.
New senses live in heat artifacts only — never in code, colophons, or specs;
the verb movement's **first act** is renaming that family, so the senses never coexist in code;
any agent meeting these words in existing code reads the retiring cult sense.
This guard deletes itself when the rename lands.

Sibling initials within the polity demesne: Invest, Divest, Attaint, read verb open — distinct;
compear consumes no colophon initial.

*Census* stands, correctly spaced against *terrier*:
the census answers who exists and is the IdP's book (we only read assertions from it);
the terrier answers who holds what and is ours.

Candidate ceremony, deliberately unminted (one specimen):
the vocabulary load-test story — a day-after tale told in the candidate words,
testing asterism coherence beyond single-word cold probes;
mint on second use.

Still open: the founding-of-IdP-trust verb; the read-Terrier verb;
the tentative confirmations above;
the classification ledger and the office→mantle prose sweep land at the pace's close.

## Retriever differentiation (cinched 260612 — policy, not mechanism)

Same architecture, three knobs:
retriever-office token lifetime at the 12 h ceiling
(activates the canon's deferred D5 capability-set-keyed lifetime — the trigger case);
machine consumers (CI, crucible charges, runtime pulls) ride their **own workload identities**
granted reader directly — the IdP is never in a machine pull chain;
the reserve-key posture (one key on retriever-office, scoped policy exception)
is documented fallback, **not built**.

## Token custody (cinched MVP shape; upgrades named)

MVP: per-session filesystem scratch behind the accessor seam, 0600, ≤ session cap, never a regime.
First upgrade: OS keystores via container credential-helpers
(one election covers podman and Docker; podman's Linux default is already tmpfs).
Anti-exfiltration layer: VPC-SC ingress with IP access levels (workforce-compatible),
deferred with triggers (first multi-operator org customer or first audit requirement).
Device-bound CBA is currently unavailable to federated principals — named revisit trigger.
Evidence: the two beta-repo memos of 2026-06-11 (custody/context-enforcement; impersonation preference).

## Launcher surface (carried forward, one addition)

`rbw-p` polity and `rbw-m` manor families as elected;
`rbw-m` levy gains establish-federation and office establishment;
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
2. **Capability-sets as named code.** As before — transcription out of invest/mantle bodies;
   these definitions become the office SAs' levy-time grant lists.
3. **Federation spike.** Slated as the heat's first pace; see its docket.
   Gate for movement 4: the legs proven in curl, the banked verification items answered.
4. **Civic verbs + offices + terriers.** The behavior-changing movement:
   its first act renames the cult colophon family (discharging the mantle homonym guard);
   RBS0 civic quoins at its head;
   establish-federation + office establishment into levy;
   the three legs in the accessor with session cache and headless fail-fast;
   admission verbs under `rbw-p`; terriers live.
5. **Manor colophon regroup.** As before, plus the new founding gestures.
6. **Narrative docs.** As before.
7. **RBRA estate retirement.** After federation personas pass the suites:
   cult verbs, key machinery, RBSRA and the keyfile probe specs retire whole.
   The bridge's demolition condition, executed.
8. **Handbook rework: home open** (₣A6 vs a final movement here), unchanged.

Movements 1–2 are no-regret and proceed immediately; nothing waits on the spike.

## MVP purpose (carried forward, recast day-after picture)

The realistic pioneer is the solo evaluator — now with domain + org + IdP tenant,
accepted as a one-time founding hour.
Day-after picture:
payor establishes manor + federation trust, levies depot (offices rise with it),
seats self as governor by one binding;
that governor enfeoffs self into director and retriever offices (two bindings);
every tabtarget works through the accessor on a morning device-flow click;
read-Terrier prints three lines;
invest/divest/mantle/roster and every key file no longer exist.

## Test rig (open, the one place a durable secret could creep back)

Synthetic personas need sessions once keys retire.
Options: per-run human click (12 h session on the project's own test org)
vs a test-org-only secret (Keycloak password grant or caged token — R4's ghost).
Decide at movement-4 slating, deliberately.
Test IdP election rides the spike:
free Entra tenant (real foreign IdP, device flow)
and Keycloak-in-a-crucible (hermetic; JWKS upload makes a private issuer viable for programmatic flow).

## Blast radius (recast)

One operator may hold several offices but a token carries exactly one office's authority;
cross-office union exists only as serial impersonation, never as one credential.
The hijacked-live-session exposure is bounded by the session cap and the lifetime policy.

## What done looks like

- No call site outside the accessor touches credential material (grep-verifiable);
  the accessor's federation branch is the only token mint.
- Offices established at levy with frozen resource IAM;
  admission verbs operate on (actor, office, principal) within a depot;
  terriers function as described; a rights query is a read.
- Zero SA keys in the system; the RBRA estate deleted whole (movement 7).
- Launcher surface per the elected families, including establish-federation.
- Specs migrated contract-first; README and narrative docs tell the office-federation story.
- Suites green at each movement, running on federation personas; `complete` before close.

## Cinched decisions

- Single tier: workforce federation + office impersonation; keyfile citizen tier canceled unbuilt; no mode enum (supersedes canon D1).
- Office SAs at levy instantiate capability-sets; all resource IAM frozen at levy behind a settle gate; admission is one tokenCreator binding.
- Zero SA keys; sole durable secret is the payor's RBRO; human-present premise (D2) bounds the design.
- Payor founds (now incl. federation trust + offices), governors populate, terriers tell, IAM enforces.
- One live IdP provider per pool; provider addition is a payor ceremony; dual-provider for migration only.
- Recut retires; credential delivery retires whole; the word assay vacates rbk.
- Retriever differs by policy only: 12 h lifetime, machine pulls on workload identity, reserve-key posture documented not built.
- Token custody: session scratch behind the accessor at MVP; keystore upgrade and VPC-SC ingress as named deferrals.
- Terrier, documentation strategy, launcher families, audit-as-mirror: carried forward as previously cinched.
- Intent-first verb orderings (terrier write precedes binding) carried forward.
- Capability-set definitions global (code, realized as office grant lists); memberships local (bindings + terrier).

## Retired cinches

- The citizen keyfile tier and everything keyfile-specific:
  per-operator SA minting, key-last enfranchise, recut, verb-internal key delivery,
  RBRA full-auto authorship, the mode enum, the two-tier test matrix.
  (The 260610–11 delivery/assay cinches retire with it — their three jobs have no successor because nothing is delivered.)
- "Identity scope is a tier property / keyfile citizens depot-minted" —
  collapsed: identity is always IdP-scoped; depot-minted identity died with the tier.
- The blast-radius union-key acceptance — dissolved by one-office-per-token.
- Earlier retired cinches stand as recorded.

## Open — resolve within the heat

- **Vocabulary re-mint** (the vocabulary pace, racing):
  elections and shape rulings recorded in Vocabulary elections above;
  remaining — founding-trust verb, read-Terrier verb, tentative confirmations,
  classification ledger and placeholder sweep at close.
- Test-rig synthetic-persona credential (see Test rig).
- Terrier file format, physical bucket name, `rbgb_` allocation.
- Handbook rework home.
- `rbw-gq` disposition at the regroup.
- Spike verification items: audit delegationInfo shape; office-token lifetime ceiling
  and the lifetime-extension org-policy constraint; AR/Cloud Build federated-token status;
  STS egress-rule shape under VPC-SC; Keycloak-crucible JWKS-upload viability.
- Beta-repo evidence memos: migrate to this repo or leave pointed-at.

## Sources

Decision record: memo-20260612-office-federation-conversion (this heat's bedrock).
Federation canon memo-20260609 (banner updated; login legs authoritative, D1 retired).
Evidence (beta repo, 2026-06-11): google-impersonation-preference; token-custody-context-enforcement.
Flap derivation: memo-20260604-credential-churn-leak-and-propagation-races;
memo-20260611-heat-BH-class-c-setiampolicy-write-flap; README Eventual Consistency appendix.
Lineage: memo-20260522 (R1 is the office shape's ancestor; R2 superseded by this conversion);
memo-20260605-citizen-capability-model (canceled tier's mechanism, historical).
Prior paddock revisions through 260610 stand as recorded;
this 260612 revision is the office-federation conversion.
Operator commitment: this paddock and its paces are slated by Fable-class agents; density is calibrated accordingly.