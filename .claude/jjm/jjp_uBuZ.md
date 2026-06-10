## Boundary — shape, not mechanism (cinched)

This heat builds the **citizen tier** — the keyfile, no-org operator-credential tier —
restructured so its capability layer is identical to the future federation tier's.
The backing stays keyfile; a no-org operator runs it unchanged.
We do **not** build federation here.
OUT of scope: workforce pool/provider setup, STS exchange, IdP device flow,
RBRD mode-enum activation,
the full drift-audit machinery,
and `modifiedGrantsByRole` topology conditioning —
all post-MVP (RBSHR "Operator federation"; `Memos/memo-20260609-federation-canon.md`; audit section below).

Pre-MVP there is **no compatibility surface**: no depots exist that we must migrate.
The grandfather bootstrap (role-from-SA-name recovery) is retired unbuilt.

Mechanism lives in `Memos/memo-20260605-citizen-capability-model.md`
(superseded-in-part banner landed; consult the paddock first on Terrier homing, census, mantle, migration — and on the verb words, re-minted below)
and `Memos/memo-20260609-federation-canon.md`; this paddock holds shape.

## The premise — human present (cinched; earmarked for RBS0)

Canon D2, restated as the bounding premise:
**a human is present at the kickoff of every run; no run outlives a session cap.**
Therefore the system holds no refresh token anywhere beyond the payor's own,
ships no unattended-run support,
and an orchestrating agent never holds a secret
(device-flow kickoff surfaces only user_code + verification URI).
This belongs in RBS0 as a premise quoin voiced `axk_premise` when the civic vocabulary incorporates —
it is the constraint that bounds the entire credential design.
The unchosen middle credential path — a bash-managed per-operator OAuth refresh token —
is recorded as declined under this premise
(a durable secret at rest, plus the OAuth-client ceremony multiplied across operators);
no current revisit trigger.

## The civic structure (cinched)

**Payor founds, governors populate, terriers tell, IAM enforces.**

- **Admission authority is scoped to the polity it admits into.**
  The payor's founding gestures: establish manor, levy depot, create the depot's terrier, seat the first governor.
  The payor is outside the citizenry — no terrier entry; its authority is constitutive, not grantable or revocable from inside.
  After founding, the payor is ceremonial:
  routine administration never exercises the payor credential,
  keeping the highest-blast-radius credential cold.
- **All citizen administration is governor-wielded and depot-scoped**:
  enfranchise, expel, enfeoff, escheat, recut.
  A governor may enfeoff another citizen of its own depot with the governor capability-set — governors create governors.
  Cross-depot administration does not exist and needs no rule: depot project scoping already forbids it.
  Revisit trigger: an org running many depots wanting delegated cross-depot administration gets a manor-scope steward seated by the payor — federation-era
  (naming caution on that day: *steward* is bound in the cnmp WRC lenses).
- **Governor keeps its quoin.**
  It dissolves structurally: a governor is a citizen holding the governor capability-set;
  mantle ceases to exist as a verb.
  Singleton-governor is a posture, never a code-enforced constraint.
  Standing governor is the default posture for both tiers
  (regression-testable administration, cold payor, tier symmetry);
  an operator may expel theirs between uses
  (known caveat: the on-disk key outlives the escheat by the propagation lag).
- **Identity scope is a tier property.**
  Keyfile citizens are depot-minted: the governor mints the SA in the depot project.
  Federates are IdP-scoped.
  A person in two keyfile depots is two citizens with two keys;
  the one-identity-across-depots story belongs to federation, where the IdP is the census.
  We never build a census.

## The Terrier — a terrier per polity (singular reframe)

Each polity keeps **its own terrier** — one ledger file per depot, payor-created at levy —
hosted together in the **Terrier bucket**, a Manor-homed GCS bucket.
The singular is the metaphor working:
historically every manor kept its own terrier and no realm-wide terrier existed,
so the word itself encodes the no-global-bookkeeping cinch.
Reserve word, not minted: *muniment* (the manorial evidence-of-rights store)
waits for the day the bucket holds non-terrier records; grep-clean when reserved.

- Write ACL: each depot's governors write exactly their own terrier, plus the payor.
  Terrier-write thereby co-locates with enfeoff authority on the same principal — the write-authority invariant satisfied trivially.
- Read population (settled): governors (own-depot) + payor — the admin plane and above.
  Directors, retrievers, and guarded consumers (the Quire memo's choristers) are all outside it;
  widen only when a working verb demonstrates need.
  A citizen holding no permission on the Terrier bucket sees no evidence it exists.
  Never `allUsers`/`allAuthenticatedUsers` — citizen names are reconnaissance data.
- Grain (elected): **managed folders** — per-depot folder IAM delivers own-terrier write and read scoping with one mechanism;
  payor reads all.
- A rights query ("what does X hold in depot Y") is a pure read of one terrier.
- **Terriers are admin-plane only — working verbs never touch them.**
  Directors and retrievers neither read nor write them;
  routine operations go straight to IAM-enforced resources.
- Object versioning on: grant history for free.
- No global bookkeeping: N independent terriers, no cross-depot consistency, no "already registered?" lookup.
- File format: mount-time.

## Verbs and orderings (cinched; words re-minted under MCM Word Selection)

Civic verbs, all governor-wielded.
The 260609 words grant/revoke/rekey were trodden words (grant/revoke are IAM's own vocabulary, maximally grep-hostile);
re-minted into the manorial register that Manor/levy/Terrier already anchor.
Enfeoff/escheat are a genuine tenure pair — creation and lawful reversion of a holding —
so the pair-relationship mirrors the verbs' symmetry.
The endow/enfeoff adjacency is elected consciously:
you *endow* an institution (Quire-side), you *enfeoff* a person (citizen-side);
register separation keeps them audible.

| Verb | Order | Crash leaves |
|---|---|---|
| enfranchise | create SA → register in terrier → mint key **last** | keyless SA (sweepable) or registered keyless citizen (recut completes); never an unregistered key |
| enfeoff | write terrier → apply bindings | visible deficit; re-run enfeoff (idempotent) |
| escheat | withdraw terrier → remove bindings | visible surplus; report-only, safe |
| expel | escheat all held → delete key + SA | partial teardown lands as surplus, never resurrection |
| recut | new key → deliver → verify → delete old | terrier untouched — keys are not its business |
| read-Terrier | pure read | — |

Robustness of the verb set against the two open futures
(the convergence test: change lands only in named, pre-fenced places):

| Verb | Federation arrives | RBRA dropped entirely |
|---|---|---|
| enfranchise / expel | branch by tier (designed for it) | body swaps, name survives |
| enfeoff / escheat | untouched | untouched |
| read-Terrier | untouched | untouched |
| recut | untouched (keyfile-side only) | retires |

RBRA is **demoted, not deleted**: one credential-kind behind the accessor.
Keyfile-tier permanence is itself not promised; the verb set survives either resolution.
Capability-set definitions are code, global across depots; memberships are data, local per depot.
Enfeoff/escheat operate only on named sets, never raw bindings (contamination guard unchanged).

## The audit — MVP is a mirror, not a surgeon

MVP audit: read terriers, read IAM, print the diff.
Report-only — no auto-converge in either direction;
the heal for a deficit is re-running the idempotent enfeoff.
With nothing auto-converging from the terrier, terrier-write is not an escalation path,
which defers the write-authority-vs-grant-authority machinery whole.
The full asymmetric-healing doctrine
(deficit auto-converge, surplus adjudication, member-first axis, expansion gating, etag concurrency — memo-20260605)
remains the cinched future shape, post-MVP.

## Ordering — substrate first (cinched)

1. **Accessor seam.**
   One accessor (likely homed in rba) resolves every token;
   no file outside it names an `RBDC_*_RBRA_FILE` or sources an RBRA file — grep is the gate.
   Identity-keyed interface, role-shaped contents initially:
   call sites keep passing today's role word, reread as a citizen name.
   Honors canon D3 at the interface only:
   contract silent about mint freshness; no cache built (no consumer yet).
   Zero behavior change; suites are the arbiter.
2. **Capability-sets as named code.**
   Transcribe the binding lists out of the invest/mantle bodies; verbs become thin compositions.
   Transcription, not design; zero behavior change.
3. **Civic verbs + terriers.**
   The behavior-changing movement:
   enfranchise/expel/enfeoff/escheat/recut, terriers in the Terrier bucket, identity-only SA names,
   `RBRS_CITIZEN` station selector,
   accessor identity override for multi-identity stations
   (the dev/test rig holds several personas; the assay axis already proves the need).
4. **Handbook rework to the civic ceremony: home open** —
   the stalled handbook heat (₣A6) vs a final movement here; decide once the verbs exist.
   Credential handbook tracks stay frozen meanwhile.

The substrate movements (1, 2) are no-regret under every model variant discussed;
design wobbles cannot reach them.
Their correctness is mechanical (suites + grep), not judgment —
deliberately robust to agent variance across sessions.

## MVP purpose (framing, cinched)

The realistic pioneer is the solo evaluator with their own GCP billing — one human wearing every hat.
The multi-role union citizen
(one person, one identity, one rbra.env, all capability-sets)
**is** the evaluator onboarding experience; that is this heat's MVP case, not an architectural nicety.
Day-after picture:
payor establishes manor, levies depot, seats self-citizen as governor;
that governor enfeoffs the same citizen with director + retriever sets;
`RBRS_CITIZEN` brands the station;
every tabtarget works through the accessor;
read-Terrier prints one line.
invest/divest/mantle/roster no longer exist.

## Inherited concern — governor teardown leak (resolved by dissolution)

With mantle gone and the governor a citizen under the generic verbs,
teardown is expel (escheat-all-first — no tombstone)
and replacement is recut (no delete at all).
The standalone fallback (memo-20260605-governor-mantle-tombstone-leak)
applies only if the civic verbs are descoped.

## Blast radius (cinched)

One citizen = one SA = one key carrying the union of held capability-sets **within its depot** — accepted (memo reasoning stands).
Depot-minted identity bounds the union at the depot:
cross-depot union exposure is federation-tier-shaped (a hijacked live session), never keyfile-shaped (a file at rest).

## What done looks like

- No call site outside the accessor touches credential files or paths (grep-verifiable).
- Capability-sets are named code;
  civic verbs operate on (actor, capability-set, citizen) within a depot;
  terriers function as described;
  a rights query is a read.
- The cult-verb surfaces migrated:
  workstation bash, admin verbs, cloud-side naming, the .adoc specs voicing the old verbs, the theurge cases.
  Handbook migration per the open home decision.
- Suites green at each movement; `complete` before close.

## Cinched decisions

- Enforcement is server-side IAM; the terrier is intent; no file or client-side check is ever the boundary.
- Human-present premise (canon D2) bounds the design;
  no refresh tokens anywhere beyond the payor's own;
  earmarked for RBS0 as an `axk_premise` voicing.
- Payor founds, governors populate, terriers tell, IAM enforces;
  admission authority is scoped to the polity it admits into.
- Governor keeps the quoin;
  dissolves to citizen + governor capability-set;
  mantle retires;
  singleton and standing-governor are postures, not constraints.
- Governors create governors within their own depot; cross-depot administration does not exist.
- Identity scope is a tier property:
  keyfile citizens depot-minted; federates IdP-scoped; no census, ever.
- Civic verb words re-minted under MCM Word Selection: enfeoff (was grant), escheat (was revoke), recut (was rekey);
  enfranchise and expel pass the trodden-word gate unchanged;
  endow/enfeoff adjacency elected consciously (institution vs person).
- A terrier per polity, hosted in the Terrier bucket (Manor-homed);
  payor creates at levy;
  governor-writeable own-terrier-only;
  read population governors-and-above, exclusion list explicit (directors, retrievers, guarded consumers);
  managed-folder grain;
  working verbs never touch terriers;
  *muniment* held in reserve, unminted.
- Capability-set definitions global (code); memberships local (data);
  enfeoff/escheat only on named sets, never raw bindings.
- Intent-first orderings as tabled; key-last enfranchise.
- MVP audit is a read-only diff; the asymmetric-healing doctrine is deferred intact, not diluted.
- RBRA is demoted to one credential-kind behind the accessor — not deleted; permanence not promised either.
- Multi-role is first-class; the solo evaluator's union citizen is the MVP case.
- No migration machinery: pre-MVP has no compatibility surface.
- The accessor is identity-keyed at the interface from day one; D3's no-per-call-mint assumption honored at the interface only.

## Retired cinches

- "Roster derives from IAM, not a stored list" — retired earlier (see memo); the Terrier + audit replaced it.
  The true invariant (no enforcement state outside IAM) survives.
- "Manor hosts the Terrier" as a singular registry — retired for per-depot terriers
  (still physically Manor-hosted as a bucket, but no global census;
  both the singular-registry framing and a briefly-explored manor-scoped-citizen-SA model are dropped).
- "Keyfile/solo drops or demotes the standing governor" — retired:
  standing governor is the default posture;
  the true invariant (minimize standing privilege) survives as the posture knob.
- "One SA per person" globally — narrowed: one SA per person **per depot**;
  the cross-depot one-identity property is federation's, via the IdP.
- The memo-20260527 stepping-stone question (role-keyed vs identity-keyed accessor) — resolved:
  identity-keyed interface, role-shaped contents.
- The 260609 verb words grant/revoke/rekey — re-minted (see Verbs section); the verb *semantics* carried over unchanged.

## Open — resolve within the heat

- Handbook rework home: ₣A6 vs a final movement here.
- Terrier file format (reader population, grain, and bucket naming are settled above).
- Whether enfranchise+enfeoff get a convenience wrapper for the one-hat citizen.
- Whether recut ships in MVP or immediately after.
- RBS0 incorporation timing for the civic quoins (citizen, terrier, capability-set, the human-present premise);
  at incorporation, decide whether the citizen's terrier entry gets a noun
  (the prebend's manorial dual — candidate *feoffment*)
  and the read-Terrier verb word (candidate *consult*).
- Breadcrumbs owed:
  the bedrock Quire memo's verb lines predate the re-mint
  (§12 "enfranchised, granted, revoked, expelled" and "Establishment (invest/divest/roster/endow)") — owe it the civic-verb breadcrumb;
  the canon's diagram repoint item stands.
- `RBRA_TOKEN_LIFETIME_SEC` eviction to RBRD lifetime policy (canon D4/D5) — small, slot it where convenient.

## Sources

rbk-08-credential-repairs;
memo-20260604-credential-churn-leak-and-propagation-races (the lifecycle split — the enabler);
memo-20260527-operator-credential-models (folded into the federation canon);
memo-20260605-governor-mantle-tombstone-leak (standalone fallback only);
memo-20260605-citizen-capability-model (mechanism, partially superseded as noted);
memo-20260605-citizen-model-ultracode-process;
memo-20260609-federation-canon (D1–D6; the tier boundary and the premise);
MCM "Word Selection" + `mcm_asterism`/`mcm_trodden_word` (the verb re-mint doctrine);
RBSHR "Operator federation".
Prior revisions: 260605 design conversation + two review passes; 260606 Manor/verb dispositions; 260609 morning Terrier naming; 260609 evening civic structure, per-depot Terrier, substrate ordering.
This 260610 revision (groom session following the MCM Word Selection landing):
civic verb re-mint (enfeoff/escheat/recut),
terrier singular reframe + Terrier bucket + managed-folder grain,
read-population settlement (governors and above, explicit exclusions),
muniment reserve, endow/enfeoff conscious adjacency, feoffment/consult queued for RBS0 incorporation,
bedrock-memo breadcrumb queued.
Operator commitment: this paddock and its paces are slated by Fable-class agents; density is calibrated accordingly.