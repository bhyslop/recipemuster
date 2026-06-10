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
(this revision supersedes its Terrier-homing, census, mantle, and migration passages — breadcrumb owed)
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

## The civic structure (cinched this revision)

**Payor founds, governors populate, ledgers tell, IAM enforces.**

- **Admission authority is scoped to the polity it admits into.**
  The payor's founding gestures: establish manor, levy depot, create the depot's ledger file, seat the first governor.
  The payor is outside the citizenry — no ledger entry; its authority is constitutive, not grantable or revocable from inside.
  After founding, the payor is ceremonial:
  routine administration never exercises the payor credential,
  keeping the highest-blast-radius credential cold.
- **All citizen administration is governor-wielded and depot-scoped**:
  enfranchise, expel, grant, revoke, rekey.
  A governor may grant the governor capability-set to another citizen of its own depot — governors create governors.
  Cross-depot administration does not exist and needs no rule: depot project scoping already forbids it.
  Revisit trigger: an org running many depots wanting delegated cross-depot administration gets a manor-scope steward seated by the payor — federation-era.
- **Governor keeps its quoin.**
  It dissolves structurally: a governor is a citizen holding the governor capability-set;
  mantle ceases to exist as a verb.
  Singleton-governor is a posture, never a code-enforced constraint.
  Standing governor is the default posture for both tiers
  (regression-testable administration, cold payor, tier symmetry);
  an operator may expel theirs between uses
  (known caveat: the on-disk key outlives the revoke by the propagation lag).
- **Identity scope is a tier property.**
  Keyfile citizens are depot-minted: the governor mints the SA in the depot project.
  Federates are IdP-scoped.
  A person in two keyfile depots is two citizens with two keys;
  the one-identity-across-depots story belongs to federation, where the IdP is the census.
  We never build a census.

## The Terrier — per-depot ledger files (reshaped this revision)

The Terrier is a **collection**: one ledger file per depot, hosted in a Manor bucket, created by the payor at levy.

- Write ACL: each depot's governors write exactly their own depot's file, plus the payor.
  Ledger-write thereby co-locates with grant authority on the same principal — the write-authority invariant satisfied trivially.
- Read population: governors + payor initially (admin-plane);
  widen only when a working verb demonstrates need.
  Never `allUsers`/`allAuthenticatedUsers` — citizen names are reconnaissance data.
- A rights query ("what does X hold in depot Y") is a pure read of one file.
- **The Terrier is admin-plane only — working verbs never touch it.**
  Directors and retrievers neither read nor write it;
  routine operations go straight to IAM-enforced resources.
- Object versioning on: grant history for free.
- No global bookkeeping: N independent files, no cross-depot consistency, no "already registered?" lookup.
- File format, bucket naming, managed-folder vs per-depot-prefix mechanics: mount-time.

## Verbs and orderings (cinched)

Civic verbs, all governor-wielded:

| Verb | Order | Crash leaves |
|---|---|---|
| enfranchise | create SA → register in ledger → mint key **last** | keyless SA (sweepable) or registered keyless citizen (rekey completes); never an unregistered key |
| grant | write ledger → apply bindings | visible deficit; re-run grant (idempotent) |
| revoke | withdraw ledger → remove bindings | visible surplus; report-only, safe |
| expel | revoke all held → delete key + SA | partial teardown lands as surplus, never resurrection |
| rekey | new key → deliver → verify → delete old | ledger untouched — keys are not its business |
| read-Terrier | pure read | — |

Robustness of the verb set against the two open futures
(the convergence test: change lands only in named, pre-fenced places):

| Verb | Federation arrives | RBRA dropped entirely |
|---|---|---|
| enfranchise / expel | branch by tier (designed for it) | body swaps, name survives |
| grant / revoke | untouched | untouched |
| read-Terrier | untouched | untouched |
| rekey | untouched (keyfile-side only) | retires |

RBRA is **demoted, not deleted**: one credential-kind behind the accessor.
Keyfile-tier permanence is itself not promised; the verb set survives either resolution.
Capability-set definitions are code, global across depots; memberships are data, local per depot.
Grant/revoke operate only on named sets, never raw bindings (contamination guard unchanged).

## The audit — MVP is a mirror, not a surgeon (resized this revision)

MVP audit: read ledgers, read IAM, print the diff.
Report-only — no auto-converge in either direction;
the heal for a deficit is re-running the idempotent grant.
With nothing auto-converging from the ledger, ledger-write is not an escalation path,
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
3. **Civic verbs + ledger.**
   The behavior-changing movement:
   enfranchise/expel/grant/revoke/rekey, ledger files, identity-only SA names,
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
that governor grants director + retriever sets to the same citizen;
`RBRS_CITIZEN` brands the station;
every tabtarget works through the accessor;
read-Terrier prints one line.
invest/divest/mantle/roster no longer exist.

## Inherited concern — governor teardown leak (resolved by dissolution)

With mantle gone and the governor a citizen under the generic verbs,
teardown is expel (revoke-all-first — no tombstone)
and replacement is rekey (no delete at all).
The standalone fallback (memo-20260605-governor-mantle-tombstone-leak)
applies only if the civic verbs are descoped.

## Blast radius (cinched, updated)

One citizen = one SA = one key carrying the union of held capability-sets **within its depot** — accepted (memo reasoning stands).
Depot-minted identity bounds the union at the depot:
cross-depot union exposure is federation-tier-shaped (a hijacked live session), never keyfile-shaped (a file at rest).

## What done looks like

- No call site outside the accessor touches credential files or paths (grep-verifiable).
- Capability-sets are named code;
  civic verbs operate on (actor, capability-set, citizen) within a depot;
  ledger files function as described;
  a rights query is a read.
- The cult-verb surfaces migrated:
  workstation bash, admin verbs, cloud-side naming, the .adoc specs voicing the old verbs, the theurge cases.
  Handbook migration per the open home decision.
- Suites green at each movement; `complete` before close.

## Cinched decisions

- Enforcement is server-side IAM; the ledger is intent; no file or client-side check is ever the boundary.
- Human-present premise (canon D2) bounds the design;
  no refresh tokens anywhere beyond the payor's own;
  earmarked for RBS0 as an `axk_premise` voicing.
- Payor founds, governors populate, ledgers tell, IAM enforces;
  admission authority is scoped to the polity it admits into.
- Governor keeps the quoin;
  dissolves to citizen + governor capability-set;
  mantle retires;
  singleton and standing-governor are postures, not constraints.
- Governors create governors within their own depot; cross-depot administration does not exist.
- Identity scope is a tier property:
  keyfile citizens depot-minted; federates IdP-scoped; no census, ever.
- The Terrier is per-depot ledger files in a Manor bucket;
  payor creates at levy;
  governor-writeable own-depot-only;
  admin-plane reads;
  working verbs never touch it.
- Capability-set definitions global (code); memberships local (data);
  grant/revoke only on named sets, never raw bindings.
- Intent-first orderings as tabled; key-last enfranchise.
- MVP audit is a read-only diff; the asymmetric-healing doctrine is deferred intact, not diluted.
- RBRA is demoted to one credential-kind behind the accessor — not deleted; permanence not promised either.
- Multi-role is first-class; the solo evaluator's union citizen is the MVP case.
- No migration machinery: pre-MVP has no compatibility surface.
- The accessor is identity-keyed at the interface from day one; D3's no-per-call-mint assumption honored at the interface only.

## Retired cinches

- "Roster derives from IAM, not a stored list" — retired earlier (see memo); the Terrier + audit replaced it.
  The true invariant (no enforcement state outside IAM) survives.
- "Manor hosts the Terrier" as a singular registry — retired this revision for per-depot ledger files
  (still physically Manor-hosted as a bucket, but no global census;
  both the singular-registry framing and a briefly-explored manor-scoped-citizen-SA model are dropped).
- "Keyfile/solo drops or demotes the standing governor" — retired:
  standing governor is the default posture;
  the true invariant (minimize standing privilege) survives as the posture knob.
- "One SA per person" globally — narrowed: one SA per person **per depot**;
  the cross-depot one-identity property is federation's, via the IdP.
- The memo-20260527 stepping-stone question (role-keyed vs identity-keyed accessor) — resolved:
  identity-keyed interface, role-shaped contents.

## Open — resolve within the heat

- Handbook rework home: ₣A6 vs a final movement here.
- Ledger reader-population mechanics, file format, managed-folder vs per-depot-prefix layout.
- Whether enfranchise+grant get a convenience wrapper for the one-hat citizen.
- Whether rekey ships in MVP or immediately after.
- RBS0 incorporation timing for the civic quoins (citizen, Terrier, capability-set, the human-present premise).
- Breadcrumbs owed:
  memo-20260605 (Terrier homing, census, mantle, migration passages superseded by this revision);
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
RBSHR "Operator federation".
Prior revisions: 260605 design conversation + two review passes; 260606 Manor/verb dispositions; 260609 morning Terrier naming.
This 260609 evening revision folds in a long design conversation (first Fable 5 session):
the civic structure and admission scoping,
governor dissolution and postures,
per-depot ledger Terrier replacing the singular registry,
read-only MVP audit,
human-present premise propagation,
demote-don't-delete RBRA and the verb robustness matrix,
substrate-first ordering,
and the solo-evaluator MVP framing.
Operator commitment: this paddock and its paces are slated by Fable-class agents; density is calibrated accordingly.