# Heat ₣BZ — Movement-4 Review Findings (260615)

Provenance, not authority. This memo records the findings of the high-effort
multi-agent review run by the pace-design membrane pace of heat ₣BZ
(rbk-14-office-federation). It is the **entry point for model Fable's terminal
review** of the heat — the terminal Fable-review pace calls it out by name.

The paddock stayed **frozen** through this review (Opus-stewardship banner). Per
the freeze discipline, findings land here and in the firmed paces — never folded
back into the paddock. The operator considered a sanctioned paddock amendment and
chose instead to carry the Fable hook as a terminal pace, leaving the freeze whole.

Method: a balanced-merits adversarial workflow — 27 agents, ~3.1M tokens, every
structural claim code-grounded (the agents grepped the live kit, counted the levy
function's steps, verified the spec-acronym seats against both disk and the map).
Posture (operator-elected): balanced merits — the first cut earned no deference and
no suspicion.

---

## THE OPEN QUESTION (the focus): when must federation personas pass the suites?

This is the one judgment that resolved at **medium** confidence, and the one place
the review needed an operator-or-architect ruling. It is recorded here in full for
Fable.

**The question.** The paddock's ordering couples movement 7 (RBRA-estate
retirement) to a gate: "suites stay green on [keyfile] until federation personas
pass the same suites — then the RBRA estate retires whole." Reading that literally
raises a chicken-and-egg: to run the canonical test suite on *federation personas*
you need a **keyless** credential-bootstrap fixture, because today's canonical
fixture (`rbtdrk_canonical`) writes key-backed RBRA — the very artifact federation
eliminates. So: does "federation personas pass the suites" bind **during movement
4** (which would pull the keyless-fixture rewrite into M4 as a real, mount-sized,
separately-failing pace), or is it the **heat-close gate that legitimately waits
for movement 7**, where that fixture must change regardless?

**The ruling taken (operator-delegated, 260615): the M7-coupled reading.**
- Movement 4 proves the federation verbs **live on the standing spike trust** (the
  three-leg chain is already spike-proven end-to-end; the spike rig stands as the
  project's standing test trust).
- The **keyless rewrite of the canonical credential-bootstrap fixture** lands at
  **movement 7**, coupled to the RBRA-estate retirement — where `rbtdrk_canonical`
  must change anyway.
- Movement 4 therefore stays **eight paces**; no keyless-fixture standup pace is
  added.

**Why this reading.**
- The spike is *already* proven live, so M4 does not need the canonical-suite
  keyless fixture to prove the verbs work — the spike trust is a sufficient M4
  arbiter.
- Building a keyless fixture in M4 only to re-cut it at M7 is **double work** the
  Load-Bearing Complexity principle says to avoid; the fixture changes at M7
  regardless of M4.
- A dedicated *resource*-standup pace (the form the open question originally
  floated) is a **verified no-op**: grep of `Tools/rbk/rbtd/src` for every
  federation token returns zero — the kept fixtures are cloud *resources*, not a
  theurge fixture awaiting a sibling, so a standup pace would build nothing.

**Residual risk (recorded for Fable to overturn).** Under this reading the
federation surface is **not canonical-suite-proven on personas until M7** — three
movements after it is built. Integration bugs in the verb surface that the spike
trust does not exercise could sleep through M5/M6 and surface only at the M7 swap.

**Revisit trigger.** If, at M4 close, the spike-trust arbiter feels too thin to
trust the verb surface for three more movements — or if M5/M6 work depends on a
suite-green federation persona — overturn to the M4 reading: cantle a dedicated
**keyless-fixture pace after the don-leg pace** (explicitly *not* the resource-
standup no-op). The admission-verbs docket already names this coupling honestly,
so the seam is visible either way.

**What the operator should know they decided.** The paddock ordering step 7
("personas must pass the same suites — then the RBRA estate retires") reads to me
as "personas pass *before/at* M7," which this ruling satisfies by proving on the
spike trust in M4 and doing the canonical-suite-on-personas proof as part of the
M7 estate swap. A stricter "must pass the *canonical service suite* within M4"
reading would add the keyless-fixture pace. This is the call Fable should sanity-
check first.

---

## The five judgment calls

| Call | Ruling | Confidence |
|---|---|---|
| Levy-founding split | **SPLIT into two paces** | high |
| Admission-verbs split | **KEEP one pace** | high |
| Compearance/don boundary | **KEEP the two-pace cut** | high |
| Rename depth | **KEEP full-depth, one pace** | high |
| Test substrate | **KEEP — no standup pace** | medium (see above) |

### 1. Levy-founding split → SPLIT into two
The first-cut founding-gestures pace (was ₢BZAAH) splits into a net-new
**affiance** pace and a reshaped **levy/mantle-establishment** pace.
- **Affiance** (org-scoped IdP-trust founding: workforce pool + provider +
  attribute mapping, seating the payor's org-level `workforcePoolAdmin` first per
  spike F1) is wholly net-new — grep for `workforcePool`/`sts.googleapis`/
  `attributeMapping` across `Tools/rbk/*.sh` returns zero. It is **M2-independent**
  and sits **first** in heat order among the two.
- **Levy/mantle-establishment** (three mantle SAs beside mason, every resource
  binding frozen behind the settle gate; AR Data-Access audit-log enablement) is
  **M2-gated** — its grant lists are the capability-sets lifted by movement 2.
- **Why two, not one:** scope (org vs depot-project), API surface (workforce-pool
  REST vs SA-create+IAM vs `auditConfigs`), and M2-dependency all fall on the same
  seam — affiance vs the remainder. When three boundaries coincide, the seam is a
  pace boundary. `rbgp_depot_levy()` is already ~289 lines / 19 steps with a
  maximal contract; folding three more net-new gesture-families onto it makes one
  coronet a two-to-three-mount unit.
- **Why not three:** AR audit-log enablement is a single `auditConfigs` gesture
  keyed to the AR resource the same levy creates — a ceremony step, not a mount.
  A three-way split is over-fragmentation (the named hazard inverted).
- **The keep-side's best point, answered:** the single settle gate is a runtime
  IAM-freeze *invariant*, not a work-decomposition boundary. Two paces preserve it
  fully — the reshaped levy pace still freezes all depot-scoped IAM behind the one
  gate; affiance's org-scoped work was never a resource-IAM-freeze concern.

### 2. Admission-verbs split → KEEP one
The four verbs (invest/divest/attaint/rehearse) stay in one operator-surface pace.
They are thin compositions over existing `rbgi_iam` primitives plus the terrier
sub-ops, differing only in the depot-scoped binding; `attaint` is definitionally
`divest`-all plus the sweep. They share one (actor, mantle, principal) tuple, one
test arbiter (the family round-trip), and one contract family. The four distinct
crash-residue contracts are real — they become **docket content** (named in
Done-when), not four paces. A per-verb split would scatter one binding-mapping
decision across four dockets and force cross-order coronet refs — the
over-fragmentation mirror of the heat's named over-treatment hazard.

### 3. Compearance/don boundary → KEEP the two-pace cut
The Leg-1+2 (compearance + STS + federated-token cache) vs Leg-3 (the don +
overhang) cut lands on a real object boundary the paddock itself cinches: two
artifacts, two lifetimes, two endpoints (federated token from STS vs mantle token
from iamcredentials), with **opposite cache rules flipping exactly at the seam**
(federated cached 0600/tmpfs per-assize; mantle never cached, in-process). Only
Leg 3 carries the F2 403 Palisade membrane — matching the codebase's
one-membrane-one-pace grain. The V1 overhang couples the halves through the
persisted cache as a clean producer-consumer interface, not entanglement.

### 4. Rename depth → KEEP full-depth, one pace
The cult homonym (invest/divest/mantle) lives in the **substrate**, not just the
operator-facing colophons: cult-word identifiers ride durable names across **six
theurge Rust source files** (`rbtdrk`/`rbtdtk_canonical`, `rbtdrm_manifest`,
`rbtdrp_pristine`, `rbtdro_onboarding`, `rbtdrc_crucible`) plus the acronym map
plus RBS0 op-quoins plus cult-spec prose. The federation civic quoins collide one
pace later under MCM one-meaning-per-word, so colophons-only is the forbidden
half-sweep — it leaves the homonym live in exactly the layers the federation verbs
reference. The cult functions die at M7, but the homonym **binds at the
civic-quoin birth three movements earlier**: a dying name a new birth is about to
collide with is not shielded.

### 5. Test substrate → KEEP (see THE OPEN QUESTION above)
No federation-test-fixture standup pace; M4 stays seven... — corrected to **eight**
paces after the levy split. Two docket corrections, not a new pace: the compearance
pace scope-clarified to own the suite-head assize-injection seam, and the
admission-verbs arbiter corrected to name the M7-coupled keyless-fixture dependency
honestly.

---

## Docket changes (first-cut → firmed)

Eight firmed paces (seven first-cut + the affiance carve). Each docket honors JJK
craft (Done-when / Cinched / Character; semantic line breaks; point-don't-paraphrase;
no plan-step labels). The firmed dockets are the reslated paces themselves; the
material deltas:

- **cult-rename**: blast-radius recipe broadened to one repo-wide grep across bash +
  theurge-Rust + spec + zipper + acronym-map (first cut omitted the Rust canon and
  acronym map); the full-depth hedge promoted to a settled cinch; regime-poison gate
  added to Done-when.
- **rbs0-civic-quoins**: Character recast from "sub-letter discipline" to
  flat-category authoring (RBS0 is single-prefix-per-category, not a JJS0-style
  sub-lettered scheme); the human-present premise added to Done-when; the
  Terrier-noun-as-shell vs internals-parked split made explicit.
- **manor-affiance** (NEW): carved from the founding-gestures pace; carries affiance
  only, org-scoped, M2-independent, heat-order first; owns the spike-F1 ordering and
  the multi-IdP affiance cinch.
- **levy/mantle-establishment** (reshaped, relabel pending): affiance struck out;
  audit-log Done-when names both ADMIN_READ and DATA_READ and excludes iamcredentials
  (400-rejects service-level auditConfig, spike V3); the "movement 2" plan-step
  cross-ref replaced with a concept pointer.
- **accessor-compearance-assize**: cache mechanics collapsed to a pointer at the
  token-custody cinch; the suite-head assize-injection seam folded in (homes the
  assize bridge without minting a pace); Character reframed around the
  federated-vs-mantle-token seam.
- **accessor-don-leg3**: the F2 403 Palisade signature added as a Cinched line (the
  structural "unregistered callers" 403 is failed loud, never pattern-matched to the
  SA-propagation retry loops the accessor already carries) — load-bearing for this
  pace, omitted in the first cut.
- **terrier-live**: parked-internals restatement replaced with a pointer to the
  Terrier subdoc's Forthcoming section; the one load-bearing delta kept —
  managed-folder IAM is net-new on the buckets module.
- **admission-verbs-polity**: arbiter corrected — the first cut's "service suite green
  on federation personas; complete green before close" silently presupposed a keyless
  canonical-fixture rewrite no M4 docket builds; now names the M7 coupling honestly;
  the may-split-per-verb tail struck.

---

## The mint sequence (verified against disk AND the acronym map)

Contract-first; successor acronyms mint at each pace's slate-time, never pinned into
upstream dockets (documentation-strategy cinch). The dockets reference successor
specs by role, not code.

1. **Premise quoin** (human-present) — RBS0 premise category, a further sibling
   voiced `axk_premise`. Authored by the civic-quoins pace.
2. **Civic-quoin category** — one new RBS0 federation prefix seating the elected
   vocabulary as flat quoins (mantle, compear/compearance, assize, don, invest,
   divest, attaint, affiance, rehearse, muniment, citizen, census). The category
   prefix is the terminal-exclusivity mint. **The rename pace must precede this** so
   the civic invest/divest quoins do not collide under MCM one-meaning-per-word.
3. **RBSTR** (existing — no new file acronym) — the Terrier noun quoin rides the new
   civic category; the RBSTR sub-op quoins register in RBS0; `include::` added.
4. **RBSMF** — levy/mantle-establishment contract spec. The M-seat (manor demesne) is
   empty on both disk and map (verified), so RBSMF opens it cleanly.
5. **RBSMA** — affiance contract spec, org-scoped founding successor. M-seat, clean.
6. **RBSPN invest / RBSPD divest / RBSPA attaint / RBSPO rehearse** — polity
   admission-verb contract specs, successors to the renamed cult specs
   (RBSDK/RBSDD/RBSRK/RBSRD). All four free on disk and in map. Mnemonic mismatch is
   deliberate and flagged: invest cannot be RBSPI (taken by payor_install), rehearse
   cannot be RBSPR (payor_refresh). RBSPH/RBSPV are file-present but map-absent — they
   still consume the P-seat; do not mint onto them.
7. **Terrier-muniment sprue** (JSON wire-key, not an RBS* file acronym) — the one new
   persistent wire-key name, owned by the terrier-live pace; joins the RB-authored
   sprue families block in RBS0.

Seat census: M-seat (RBSM*) entirely empty → RBSMF/RBSMA open it cleanly, RBSMC free
if audit-log ever splits out. P-seat taken on disk = E,H,I,R,T,V; in map = E,I,R,T —
RBSPH/RBSPV are file-present/map-absent and still consume the seat. RBSDN
(depot_initialize) is in map + on disk but NOT wired into RBS0; RBSDE is the wired
levy home.

---

## For Fable's terminal review

The five decisions above are Opus's balanced-merits calls on a Fable-authored heat.
The places to scrutinize first, in order of leverage:
1. **The open question** — the M4/M7 reading is the one medium-confidence call and
   the one with a recorded residual risk. Sanity-check whether the paddock's "same
   suites" gate really tolerates spike-trust-only proof through M5/M6.
2. **Rename depth** — confirm the six-Rust-file blast radius is real and that
   full-depth is right (vs the operator-facing half-sweep the first cut hedged on).
3. **Levy split** — confirm the affiance/remainder seam is load-bearing and that the
   single settle gate is preserved across the two paces.
4. Whether any firmed docket over-treats (the named hazard) or under-specifies a
   load-bearing contract.

The full workflow result (all five for/against/judge panels, the seven docket
mount-readiness reviews, the sequencer output) is in the membrane pace's run
transcript; this memo is the distilled, reviewable record.
