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
hosted together in the **Terrier bucket**, a Manor-homed GCS bucket
(concept name elected; the physical GCS name and `rbgb_` allocation are mount-time).
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

## Credential delivery — assay retires (settled)

Canon D4 already condemns today's delivery flow:
RBRA is full-auto (sole authors are the identity verbs; operator never edits),
so the assay staging directory plus the manual operator `mv` into a role directory is human authorship of RBRA placement —
an XOR-rule violation the civic verbs close.

- **Delivery is verb-internal.** Enfranchise and recut write the final identity-keyed RBRA path directly:
  mint → write → verify (the access probe is the prove-before-trust step, folded in) → done.
  No staging slot, no operator move.
- **The yoke precedent applies, in its cleaner form.**
  Tool-authored regime mutation is proven by the reliquary yoke (temp file, atomic swap, BCG discipline);
  RBRA is wholly infra-owned, so delivery is a whole-file write — no surgical field rewrite needed.
- **Mid-verb intermediates ride `BURD_TEMP_DIR`** (per-invocation, cleaned).
  Today's decoded-key JSON parked in the semi-durable secrets/assay directory is a lingering-plaintext leak this closes.
  `BURD_OUTPUT_DIR` is rejected for credentials:
  its current/previous recycling clobbers on a two-run horizon, and secrets do not ride the run-output channel.
- **Test personas are ordinary citizens.**
  The dev/test rig's personas are plain citizen identities under the accessor identity override (Ordering, movement 3);
  no special slot, no special word.
- **The word assay vacates rbk**, resolving its standing double-binding with APCK's detection-pipeline assay
  (semantic uniqueness restored by attrition).
  Theurge's *cupel* is a separate clean mint and is unaffected.
- Out-of-heat note (roadmap-grade): the yoke's surgical write puts an infra-authored field inside the human-edited `rbrv.env` —
  in tension with canon D4's per-file XOR read literally; not this heat's to fix.

## Launcher surface — colophon families (elected)

Two family mints, one rejection:

- **`rbw-p` — the polity family** (minted): governor-wielded citizen administration —
  enfranchise, enfeoff, escheat, expel, recut, terrier read, audit.
  The family letter names the tier-blind domain noun,
  so federation changes verb bodies, never colophons — the convergence test applied to the launcher surface.
- **`rbw-m` — the manor family** (minted; regroup executes this heat): the payor's founding surface,
  gathered from its current scatter across `rbw-g` and `rbw-d` (see map).
  Levy gains the create-terrier and seat-first-governor founding gestures here.
  Director-wielded depot verbs (`rbw-di`, `rbw-dI`, `rbw-dY`) stay in `rbw-d`.
- **`rbw-P` rejected**: payor and polity are semantically adjacent,
  and a case-only family discriminator is unreliable across the fleet
  (Git Bash ships completion-ignore-case; macOS and Windows filesystems resolve wrong-case paths).
  Case-pair families are reserved for semantically unrelated pairs —
  the `rbw-i`/`rbw-I` precedent; `rbw-m` vs `rbw-M` (Marshal) rides the same allowance.
- **`rbw-a`'s admin colophons retire** with the cult verbs.
  The `rbw-ac` access probes survive in place
  (accessor-seam diagnostics, not polity administration), rekeyed to identity in movement 1;
  the payor probe keeps payor wording — probes are per-credential-kind, and the payor is no citizen.
  The probes' long-term family belongs to the accessor seam.

Retirement map:

| Today | Fate |
|---|---|
| `rbw-arI` / `rbw-adI` (invest) | `rbw-p` enfranchise + enfeoff |
| `rbw-arD` / `rbw-adD` (divest) | `rbw-p` escheat / expel |
| `rbw-arr` / `rbw-adr` (roster) | `rbw-p` terrier read |
| `rbw-aM` (mantle) | retires whole; founding seat rides `rbw-m` levy |
| `rbw-gPI` / `rbw-gPE` / `rbw-gPR` (payor OAuth ceremonies) | `rbw-m` (move; verbs unchanged) |
| `rbw-dL` / `rbw-dU` / `rbw-dl` (depot levy/unmake/list) | `rbw-m` (move; levy gains terrier + seat gestures) |
| — | `rbw-p` recut, audit (new surface) |

Sub-letter caution for the `rbw-p` mint: the manorial verbs pile on 'e'
(enfranchise, enfeoff, escheat, expel) —
second letters must diverge early, minted at the zipper with its legend.

## Documentation strategy (cinched)

Split by document kind; the heat's own doctrine decides each:

- **Contract specs ride their pace, contract-first.**
  Movement 3's paces are verb-shaped;
  each pace writes the successor .adoc as the contract, implements against it, and retires the old spec in the same pace —
  never two authoritative wordings live at once (the word-cancer rule at spec grain).
  This is intent-first applied to documentation:
  the spec is the intent ledger, code converges, suites audit.
  Docs-after-the-fact is the retired "roster derives from IAM" cinch applied to documentation — rejected.
- **Successor spec acronyms mint at slate-time.**
  Seven verb specs retire: RBSDK, RBSRK, RBSDD, RBSRD, RBSDR, RBSRL, RBSGM;
  the accessor seam touches RBSAX/RBSAJ/RBSAO and the credential-format RBSRA.
  Mint the successors when the paces are cut, so dockets reference specs by name before they exist.
- **RBS0 civic quoins land at the head of movement 3** —
  the verb specs need citizen/terrier/capability-set quoins to voice;
  the incorporation-timing question resolves by dependency, not preference.
- **Narrative docs trail the surfaces they describe.**
  README.md is the most public cult-verb surface
  (Mantle/Invest/Divest/Roster glossary anchors, the role table, the getting-started ceremony);
  it updates as its own pace after movements 3–4, never before the regroup it would name.
  RBSCO cosmology and RBSGS getting-started ride the same pace.
  Diagram repoint rides the already-queued pluml fixture run.
- **Independent-then-compare is rejected for authoring** (the spec is the derivation);
  its verification cousin survives: after movement 3, re-run the cold-probe —
  a fresh agent reading only the new specs and README, reporting what fails to bind.

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
   One accessor (likely homed in rba) resolves every credential —
   all five `RBDC_*` credential constants are in seam scope
   (governor/director/retriever/assay RBRA **and** the payor's RBRO; the accessor branches by credential-kind: signed-JWT vs payor OAuth).
   The surface is tokens **plus identity facts** (call sites read fields like the client email today), not tokens alone.
   The gate is read-side at this movement:
   no file outside the accessor names an `RBDC_*` credential file or sources one — grep is the gate.
   (Write-side delivery arrives verb-internal at movement 3 — see Credential delivery.)
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
   RBS0 civic quoins incorporate at its head (see Documentation strategy);
   then enfranchise/expel/enfeoff/escheat/recut under the `rbw-p` family, terriers in the Terrier bucket, identity-only SA names,
   `RBRS_CITIZEN` station selector,
   verb-internal credential delivery (assay staging retires — see Credential delivery),
   accessor identity override for multi-identity stations
   (the dev/test rig holds several personas; today's assay test-credential slot already proves the need.
   Canon D4's "RBRA is a true per-home singleton" describes the shipped operator posture;
   the rig is the named exception, served by the override — this paddock supersedes the canon's singleton phrasing for multi-identity stations).
4. **Manor colophon regroup.**
   The payor surface gathers into `rbw-m` per the retirement map;
   mechanical (zipper + tabtarget renames + reference sweep);
   sequenced ahead of the handbook movement so the credential tracks unfreeze against final names.
5. **Narrative docs.**
   README glossary/table/ceremony, RBSCO cosmology, RBSGS getting-started — one pace, after the surfaces exist (see Documentation strategy).
6. **Handbook rework to the civic ceremony: home open** —
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
- Credential delivery is verb-internal; the assay staging directory no longer exists.
- The launcher surface reflects the elected families:
  `rbw-a` admin colophons gone, `rbw-p` live, the payor verbs under `rbw-m`.
- The cult-verb surfaces migrated:
  workstation bash, admin verbs, cloud-side naming, the .adoc verb specs (successors written contract-first, predecessors retired in-pace),
  README and the narrative docs, the theurge cases.
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
- Colophon families: `rbw-p` polity (tier-blind — federation never moves colophons);
  `rbw-m` manor regroup executes this heat;
  `rbw-P` rejected on case-adjacency;
  case-pair families only for semantically unrelated meanings.
- Accessor seam scope: all five `RBDC_*` credential constants including the payor's RBRO;
  the surface is tokens plus identity facts;
  read-side gate at movement 1, write-side delivery verb-internal at movement 3.
- Assay retires whole per canon D4: verb-internal delivery, probe-as-verify, intermediates in `BURD_TEMP_DIR`,
  test personas are ordinary citizens, the word vacates rbk to APCK.
- Documentation strategy: contract specs ride their pace contract-first with in-pace predecessor retirement;
  successor spec acronyms mint at slate-time;
  RBS0 civic quoins at the head of movement 3;
  narrative docs trail as their own movement;
  post-movement-3 cold-probe is the verification step.
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
- The assay staging mechanism — retired by canon D4 application (see Credential delivery); its three jobs survive in successors.

## Open — resolve within the heat

- Handbook rework home: ₣A6 vs a final movement here.
- Terrier file format, physical bucket name, and `rbgb_` allocation (concept naming and ACL shape settled above).
- Whether enfranchise+enfeoff get a convenience wrapper for the one-hat citizen.
- Whether recut ships in MVP or immediately after.
- `rbw-gq` (quota capacity review) disposition: guide vs manor — decide at the regroup.
- At RBS0 incorporation (timing settled: head of movement 3):
  whether the citizen's terrier entry gets a noun (the prebend's manorial dual — candidate *feoffment*)
  and the read-Terrier verb word (candidate *consult*).
- Breadcrumbs owed: the canon's diagram repoint item stands
  (the bedrock memo's verb-line breadcrumb is discharged).
- `RBRA_TOKEN_LIFETIME_SEC` eviction to RBRD lifetime policy (canon D4/D5) — small, slot it where convenient.

## Sources

The prior credential-repairs heat's revoke layer and Class-C propagation tolerance are assumed throughout
(its record: memo-20260604-credential-churn-leak-and-propagation-races — the lifecycle split, the enabler);
memo-20260527-operator-credential-models (folded into the federation canon);
memo-20260605-governor-mantle-tombstone-leak (standalone fallback only);
memo-20260605-citizen-capability-model (mechanism, partially superseded as noted);
memo-20260605-citizen-model-ultracode-process;
memo-20260609-federation-canon (D1–D6; the tier boundary and the premise);
memo-20260609-bedrock-quire-shaping (the consume-side Quire/Cloister civic homogenization: choristers, prebend, the endow/enfeoff dual);
MCM "Word Selection" + `mcm_asterism`/`mcm_trodden_word` (the verb re-mint doctrine);
RBSHR "Operator federation".
Prior revisions: 260605 design conversation + two review passes; 260606 Manor/verb dispositions; 260609 morning Terrier naming; 260609 evening civic structure, per-depot Terrier, substrate ordering.
The 260610 groom revision: civic verb re-mint (enfeoff/escheat/recut),
terrier singular reframe + Terrier bucket + managed-folder grain,
read-population settlement, muniment reserve, colophon family election (`rbw-p` polity, `rbw-m` manor regroup, `rbw-P` rejected).
The 260610 second pass (standalone-probe findings + assay research):
movement-1 seam scope tightened, assay retirement per canon D4, D4-singleton supersession flagged,
Quire memo added to Sources, stale silks pointer replaced.
The 260610 third pass: documentation strategy
(contract-first in-pace specs, slate-time acronym mint, RBS0 quoins at movement-3 head, narrative-docs movement, post-build cold-probe);
ordering renumbered to six movements.
Operator commitment: this paddock and its paces are slated by Fable-class agents; density is calibrated accordingly.