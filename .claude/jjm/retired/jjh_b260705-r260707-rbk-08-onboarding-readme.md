# Heat Trophy: rbk-08-onboarding-readme

**Firemark:** ₣Bt
**Created:** 260705
**Retired:** 260707
**Status:** retired

## Paddock

## Character

Documentation-currency heat with a teaching mandate.
The README and onboarding handbooks are the project's teaching material;
this heat brings them true against the system as it stands
and load-tests them as pedagogy.
Story-first method: a small set of AXLA chapbooks derives the critical operational stories from executable oracles,
then README and handbook prose are reconciled against those stories.

## Shape

Three streams, dependency-ordered:

1. **Chapbooks** — aspirant AXLA chapbooks for the three critical stories:
   provenance (reliquary and Lode capture through the ordain modes to vouch and summon),
   federation (manor founding through admission to attributed action),
   containment (kludge through charge to adversarial sorties).
   Each story's spine is an executable theurge oracle;
   the chapbook stitches actors and interactions and cites governing contracts rather than restating them.
2. **Document reconciliation** — README brought current
   (vocabulary, glossary anchor inventory re-derived from the chapbook casts),
   and the onboarding handbooks reconciled to the keyless federation reality.
3. **Educational load-test** — differential agent-learner evaluation of handbook output,
   closing with a terminal docs-integrity sweep.

## Cinched

- Chapbook genre law is AXLA's: sheaf genre chapbook, hierarchy chapbook markers
  (chapbook opener carrying the document sprue, actor lanes with internal/external kinds,
  interactions citing the governing contract).
  Chapbooks are born aspirant; infusion is a separate operator decision.
- Theurge fixtures are the story oracles:
  a chapbook interaction that contradicts a passing fixture is wrong, not the fixture.
- The handbook rendering form (verified units interleaved with teaching-only units;
  one format serving skim, deep-read, and fix-hunt reading strategies)
  is settled prior art from the predecessor heat and is not redesigned here — content currency, not format.
- Handbook load-testing is differential:
  the signal is the delta between full and teaching-stripped handbook output, never an absolute score.

## Done when

The chapbooks stand as committed aspirants ratified against their oracles,
README and the onboarding handbooks read true against the current system
(a fresh-levy gauntlet satisfies onboarding's gates),
the agent-learner eval has run with findings dispositioned,
and the docs-integrity sweep is green.

## Provenance

Successor to the retired handbook-revamp heat ₣A6,
whose intent-track handbook corpus landed
and whose paddock preserves the design record for the handbook rendering form.
Chapbook apparatus landed in AXLA at b9153977e (sheaf genre axis + hierarchy chapbook markers).

## Paces

### axla-quoin-lane-voicing (₢BtAAJ) [complete]

**[260705-1728] complete**

## Character

Design conversation requiring judgment — Fable-tier.
An AXLA change to weigh, not RB content to author.

## Docket

The current AXLA chapbook law makes every cast lane mint a sprue-local token
(`rbsyp_director`, `jjcbs_operator`) on the backtick stream, and interactions reference lanes by that token.
The sweep applied it faithfully and surfaced the smell: those inlays are quoin-shaped, and in a mature chapbook they are redundant.
Every RBSYP actor lane sits one line above a catalogued quoin naming the same actor —
`rbsyp_director` over `{rbtr_director}`, `rbsyp_cloudbuild` over `{gcb_service}`, `rbsyp_hallmarks` over `{gar_hallmarks_namespace}`, and more.
The law forces a minted alias even where a quoin already names the actor, so the tale cites the alias and the quoin's cross-reference machinery does no work; a reader sees a backtick token shaped exactly like a quoin that is not one.

The question for Fable: should AXLA gain an alternate cast-lane voicing whose lane identity IS a catalogued quoin (the attribute-reference stream), so interactions reference actors by quoin and no parallel sprue token is minted?
Weigh it honestly against the sprue form, which is still needed where no quoin exists — the JJS session chapbook's actors are "pending a catalogued quoin", so the sprue token is their only handle — and which buys a short stable local handle and compact from/to lines a quoin-pair would not.
The likely shape is both forms with a rule for when each applies, not a replacement.

Read first as evidence: `RBSYP-chapbook_provenance.adoc` (mature domain, every actor has a quoin — maximal redundancy) and `JJS-aspirant-chapbook-session.adoc` (early domain, actors lack quoins — the sprue token earns its keep).
The law itself is AXLA "Axial Hierarchy Chapbook Markers" (the `axhca_actor` backtick-vs-attribute streams).

## Done when

A decision is recorded on whether and how AXLA gains a quoin-identity cast-lane voicing.
If adopted, the marker law and both existing chapbooks carry the change.
If declined, the rationale is recorded so the redundancy is understood rather than rediscovered.

### ax-annotation-line-sweep (₢BtAAH) [complete]

**[260705-1902] complete**

## Character

Mechanical repair sweep under corrected AXLA annotation law.
Opus-tier mount; file-scoped chunks may be delegated to sonnet subagents; verification stays with the mount.

## Docket

Annotation lines (`//ax*`) carry ax-universe tokens only — the motif and `axd_` dimensions;
project-universe tokens (rb*, jj*) are malformed on the line and move to the read streams after it:
backtick inlays for sprue-local tokens, attribute references for catalogued quoins.
The corrected law lives in AXLA "Axial Hierarchy Chapbook Markers"
(line grammar, arities, read windows) and in the revised `axvr_variable` voicing definition.
Discovery recipe: `grep -rn "^//ax.* rb\|^//ax.* jj" Tools --include="*.adoc" --include="*.md"`
Three families:
chapbook marker lines in the provenance chapbook (re-author to the backtick-stream form;
also replace the reserved word "beat" with "interaction" throughout that sheaf's prose);
`axvr_variable` lines carrying `rbst_` type tokens (move the type token to the attribute-reference lookahead per the revised law);
legacy `axvd_sheaf` lines carrying imprimatur tokens (rb and jj sheaves — migrate to the current opening-sentence form).

## Done when

The discovery grep returns zero hits;
every touched sheaf's attribute references still resolve against its codex mapping;
the provenance chapbook parses under the revised chapbook nesting law
(opener sprue, lane inlays, interaction from/to inlays, venue and legend inlays each inside their annotation's read window).

### rbst-primitive-type-catalog (₢BtAAI) [complete]

**[260705-1915] complete**

## Character

Type-catalogue completion, mechanical on an established template.

## Docket

Relocating each `axvr_variable`'s project-universe type term from the annotation line
into the definition body (the revised `axvr_variable` law) surfaced six primitive types
that are referenced in bodies but were never catalogued in RBS0's type section:
`rbst_string`, `rbst_url`, `rbst_version`, `rbst_integer`, `rbst_duration`, `rbst_username`.
Those body references currently dangle.
Define each on the established RB subspecialized-type pattern —
the `rbst_ipv4` / `rbst_port` / `rbst_netmask` exemplars in RBS0's type section:
a mapping-section `:rbst_X:` entry, an `[[rbst_X]]` anchor,
an `//axl_voices` line naming an existing AXLA universal motif,
and a one-line definition carrying any RB-specific constraint in prose.
The catalogued motif set is axtu_string / axtu_xname / axtu_path / axtu_ipv4 /
axtu_port / axtu_cidr / axtu_domain / axtu_sha256 — no numeric or temporal motif exists,
so voice to the nearest (string/xname) rather than minting a new motif.
Discovery recipe: `grep -nE "\{rbst_(string|url|version|integer|duration|username)\}" RBS0-SpecTop.adoc`.

## Done when

Each of the six types carries a mapping entry, anchor, voicing, and definition;
every `{rbst_X}` body reference the sweep left resolves against the codex mapping;
no new AXLA motif was minted.

### chapbook-provenance-story (₢BtAAB) [complete]

**[260705-2058] complete**

## Character

Ratification tail of the first chapbook — verification and derived presentation, judgment-light.

## Docket

The provenance chapbook stands committed (`Tools/rbk/vov_veiled/RBSYP-chapbook_provenance.adoc`)
in its venue / vertical-group / legend form,
but its annotation lines predate the corrected AXLA chapbook law;
the preceding annotation-line sweep pace re-authors them.
After that sweep lands:
re-ratify cast and interactions against the onboarding-sequence fixture
(`Tools/rbk/rbtd/src/rbtdro_onboarding.rs` — case order and cross-case witnesses are the oracle;
an interaction that contradicts a passing case is wrong);
verify the reserved word "beat" is gone from the sheaf's prose (the unit word is interaction);
verify every inlay lands inside its annotation's read window per AXLA "Axial Hierarchy Chapbook Markers";
then re-derive the sequence diagram from the marker skeleton
(venues as boxes, open/closed groups as activation bars, legends as arrow inscriptions)
and unfurl it for operator review.
Minted names (RBSY family, `rbsyp` sprue) are already operator-ratified.
The two surfaced contract gaps (uncatalogued anoint, verbless airgap base handoff)
stay recorded in-file and are not fixed here.

## Done when

Cast and interactions ratified against the fixture arc in conformant annotation form;
derived diagram unfurled and operator-accepted;
chapbook committed clean of reserved-word and annotation-law violations.

### chapbook-federation-story (₢BtAAC) [complete]

**[260705-2126] complete**

## Character

Concept-model authoring; second chapbook instance —
file home and marker idiom follow the pattern the provenance chapbook establishes.

## Docket

Draft an aspirant AXLA chapbook telling the federation story:
manor instaurate, affiance, gird, brevet, avow and don, attributed action —
with the withdrawal edges (unseat, attaint, jilt) as the story's shadow side.
Oracles: the parley suite (positive admission round-trip),
the freehold-establish gauntlet arc,
and polity-denial (rejection bands).
Beats cite the manor and polity contracts (RBSM*/RBSP*) and the federation regime specs (RBSRF/RBSRW).

## Done when

Aspirant chapbook committed;
cast and beats ratified against the parley and freehold-establish arcs.

### chapbook-containment-story (₢BtAAD) [complete]

**[260705-2245] complete**

## Character

Concept-model authoring; third chapbook instance.

## Docket

Draft an aspirant AXLA chapbook telling the containment story:
kludge and drive, charge, the sentry/pentacle/bottle composition and its enforcement layers,
adversarial sorties, quench.
Oracles: the tadmor crucible fixtures (siege for the self-contained pair; the bivouac stratum for the running suite).
Beats cite the crucible and security contracts (RBSCC, RBSCN, RBSSR, RBSSS, RBSIP).

## Done when

Aspirant chapbook committed;
cast and beats ratified against the crucible fixture arcs.

### readme-current-sweep (₢BtAAE) [complete]

**[260706-0759] complete**

## Character

Consumer-document reconciliation informed by the chapbooks;
editorial judgment over mechanical sweep.

## Docket

Bring README.md current with the system as it stands:
federation-era vocabulary, current vessel and nameplate roster, current tabtarget families.
Re-derive the glossary anchor inventory from the chapbook casts —
the README glossary is the project broadside,
so every ashlar the chapbooks surface registers there.
Retire stale anchors and stale prose (pre-federation roles, retired operation names);
verify every anchor consumed by handbook linked-term calls still resolves.
Discovery: grep the handbook corpus (Tools/rbk/rbh0/) for buh_tlt and the public-docs URL constant to enumerate consumed anchors.

## Done when

README reads true against the current system and the chapbook stories;
no consumed anchor dangles;
the broadside carries the chapbook-surfaced ashlars.

### readme-identity-single-telling (₢BtAAK) [complete]

**[260706-0832] complete**

## Character

Mechanical README edit under the cinched rule below; Opus-tier — apply the rule, don't redesign the model.

## Docket

README.md tells the federated identity model in three regions:
the Foundry Lifecycle role table and role narrative,
the Establishment and Provisioning / Admission and Access subsections,
and the "Identity and Admission" appendix.
Drift breeds in the retellings —
the Affiance/Jilt staleness fixed at the prior pace lived in exactly this duplication.

## Cinched

The appendix entry is the single home for every identity fact
(token mechanics, binding and role names, session windows, ordering rules, idempotency claims).
Narrative regions keep story flow — who acts, and why it matters — and defer every fact to its linked term:
rewrite narrative fact-sentences down to story plus link; add no new facts anywhere.
One stated exception rides the pass:
add an Escheat landing entry beside Jilt in the admission appendix —
one to two sentences (the payor's manor-hygiene sweep of the terrier:
orphaned polity slices and dead-schema strays, plan-then-confirm, idempotent when already clean)
with its own `<a id="Escheat">` anchor, so a future handbook yawp has a landing.
Never remove or rename an `<a id=...>` anchor — handbooks yawp-link them.

## Done when

Each identity fact is stated in exactly one appendix entry;
narrative regions read as story with linked terms;
the Escheat entry stands with its anchor;
the anchor sweep still passes:
every `](#X)` reference has a matching `<a id="X">`,
and every anchor minted in `Tools/rbk/rbyc_common.sh` resolves in README.md.

### readme-roadmap-promise-trim (₢BtAAL) [complete]

**[260706-0846] complete**

## Character

Mechanical README edit under the cinched rule; Opus-tier — apply the rule, don't redesign the roadmap.

## Docket

The README Roadmap appendix is the only customer-facing statement of future features —
the specs are veiled and never ship.
Its entries over-promise:
the Podman entry explains a dismissal ("no architectural advantage") that no longer reflects intent,
and several entries carry mechanism detail beyond a feature shape.

## Cinched

Promise less while properly articulating intended feature shape:
each roadmap entry either names its feature shape in a sentence or two, or comes out;
no mechanism detail, no implied timeline, no dismissive rationale for deferral.
Add no new entries — announcing unreleased directions is exactly what this pace avoids.
Never remove an `<a id=...>` anchor;
if an entry comes out, keep its anchor on the section or re-point internal refs.

## Done when

Every Roadmap entry passes the rule;
the anchor sweep passes — every `](#X)` reference has a matching `<a id="X">`.

### readme-lode-single-section (₢BtAAM) [complete]

**[260706-0914] complete**

## Character

Mechanical README restructure under the cinched shape; Opus-tier.

## Docket

Lode prose is scattered across README:
Foundry sections (Lode, Touchmark, Reliquary) plus Supply Chain appendix entries (Capture, Conclave, Ensconce).
Consolidate into one section owning all lode prose —
landing descriptions for yawps, never first-class treatment of every verb.

## Cinched

One Lode home section holds every lode term at one-to-two sentences per anchor.
Preserve every existing `<a id=...>` anchor by moving it into the section —
anchors may share a section, so the yawp seam is untouched.
Add a one-sentence roll-call with anchors for the uncovered lode verbs
(Divine, Augur, Presage, Banish, Underpin, Immure)
so any future yawp has a landing without expanding the prose.
Every other README mention of lodes reduces to links into the section.

## Done when

One section owns all lode prose;
every prior lode anchor still resolves;
the roll-call anchors exist;
the anchor sweep passes — every `](#X)` reference has a matching `<a id="X">`.

### handbook-federation-reconcile (₢BtAAA) [complete]

**[260706-1013] complete**

## Character

Handbook reconciliation; a federation-correctness fix first, vocabulary currency second.
Find the real entry points before editing — prior review notes named tracks that do not exist.

## Docket

Reconcile the onboarding handbooks to the keyless federation model.
The core defect (from the ₣BZ terminal-review divergence triage):
keyless freehold-establish dons mantles and writes NO governor key file,
but onboarding's credential-install prose and start-here menu still expect one,
so a fresh-levy gauntlet may not satisfy onboarding's gate.
Entry points: rbho0_onboarding.sh and the start-here menu (rbho0_start_here.sh), plus the rbw-Op payor track.
NOTE: the review's source named tracks rbw-Ocr/Ocd — those tabtargets DO NOT EXIST
(live rbw-O* = Occ/Oda/Odb/Odf/Odg/Ofc/Op/Ots);
locate the real keyfile-expecting prose before editing.
Full context: Memos/memo-20260622-fable-review-queue.md (m7-onboarding-keyless-gap).

Fold in the attest-durability correction salvaged from the predecessor heat:
the -attest-{arch} tags are DURABLE provenance-carrying artifacts —
the only objects with GCB-attested digests
(the classic Docker image store re-serializes manifests, producing digests that differ from buildx-native) —
persisting alongside -image/-about/-vouch/-pouch/-diags and deleted by abjure alongside them.
The first-cloud-build teaching prose was written when they were considered ephemeral scaffolding;
the tag-inventory tour should present them as durable, not skip them.

Sweep remaining handbook prose for pre-federation vocabulary against the federation chapbook's cast
(the chapbook pace earlier in this heat supplies the current story).

## Done when

Onboarding's credential-readiness prose matches the keyless reality (mantle don, not a key file);
a fresh-levy gauntlet satisfies whatever onboarding gate remains;
attest teaching presents the durable tags correctly;
no pre-federation role vocabulary survives in handbook output.

### agent-learner-load-test (₢BtAAF) [complete]

**[260706-1048] complete**

## Character

Evaluation design and execution — a minimal eval artifact, not a harness.
Tests whether handbook output actually teaches.

## Docket

Design and run a differential agent-learner evaluation of the handbook corpus
(design salvaged from the predecessor heat's eval pace):

- Anti-inference persona: the learner agent reasons strictly from what the handbook says —
  if the handbook does not say it, the learner does not know it.
- 3-5 application-focused scenario questions per evaluated track (not recall);
  include misconception probes where surface understanding answers wrong.
- Differential design: full handbook output versus a stripped variant (teaching prose removed);
  the signal is the score delta, not the absolute.
- Synthetic critic alongside the learner:
  a separate agent evaluates the handbook AS pedagogy (gaps, ambiguities, stumble points).
- Goodhart note: eval questions authored outside the track author's working context;
  rotate periodically;
  human override with written justification.

Run against at least the crash course and one director track.
Report whether the teaching prose carries measurable weight;
absorb actionable findings into handbook content or hand them to the docs-integrity sweep.

## Out of scope

Full eval harness or automation;
release-gate mechanics.

## Done when

Eval artifact committed;
runs executed against at least two tracks;
delta reported;
findings dispositioned.

### docs-integrity-tail (₢BtAAG) [complete]

**[260706-1131] complete**

## Character

Terminal mechanical sweep — deliberately last on the rail;
relocate to stay last if later paces enroll.
Carries a small editorial tail from the agent-learner eval handoff.

## Docket

Docs-integrity gates, salvaged from the predecessor heat's finalizer:

- Dangling-reference grep: README and the consumer-facing context docs carry no references
  to retired symbol families or retired tabtargets.
- Tabtarget existence integrity: every tabtarget path mentioned in README and consumer-facing docs exists in tt/;
  no planned-but-never-built references survive.
- Anchor resolution: every anchor consumed by handbook linked-term calls exists in README —
  and home this gate as a standing check in rbq_qualify
  (file-to-file: anchors minted in rbyc_common.sh plus internal `](#X)` refs against `<a id>` targets, no network),
  so the seam cannot regress silently after this heat.

Plus the agent-learner eval handoff:

- Absorb or explicitly decline the findings listed under
  "Handed to the docs-integrity sweep" in
  `Memos/memo-20260706-handbook-agent-learner-eval.md` (Disposition section);
  the ordain success-signal item needs a live ordain run (or its transcript)
  before prose is written.

## Done when

All three gates green in a single pass run after every other pace in this heat has wrapped;
the anchor gate stands in qualify and fails loud on a seeded dangle;
the eval-handoff findings are absorbed or declined.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 J axla-quoin-lane-voicing
  2 H ax-annotation-line-sweep
  3 I rbst-primitive-type-catalog
  4 B chapbook-provenance-story
  5 C chapbook-federation-story
  6 D chapbook-containment-story
  7 E readme-current-sweep
  8 K readme-identity-single-telling
  9 L readme-roadmap-promise-trim
  10 M readme-lode-single-section
  11 A handbook-federation-reconcile
  12 F agent-learner-load-test
  13 G docs-integrity-tail

JHIBCDEKLMAFG
·xxxxx······· RBS0-SpecTop.adoc
x··xxx······x claude-rbk-acronyms.md
······xxxx··· README.md
··········xxx rbhocc_crash_course.sh, rbhodf_director_first_build.sh
xx·x········· RBSYP-chapbook_provenance.adoc
···········xx memo-20260706-handbook-agent-learner-eval.md
····xx······· RBSYF-chapbook_federation.adoc
x··x········· AXLA-Lexicon.adoc
xx··········· JJS-aspirant-chapbook-session.adoc
············x claude-buk-core.md, rbq_qualify.sh
··········x·· claude-rbk-tabtarget-context.md, rbho0_start_here.sh, rbhopw_payor_wrapper.sh, rbz_zipper.sh
······x······ rbyc_common.sh
·····x······· RBSYC-chapbook_containment.adoc
···x········· rbdgp_provenance-tale-dark.svg, rbdgp_provenance-tale.puml, rbdgp_provenance-tale.svg
·x··········· JJS-aspirant-farrier.adoc, JJS-aspirant-mews.adoc, JJS-aspirant-state-repo.adoc, JJS-aspirant-tackle.adoc, RBSCV-crucible_variant.adoc, RBSHR-HorizonRoadmap.adoc, RBSOB-oci_layout_bridge.adoc, RBSPC-podman_crucible.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 48 commits)

  1 J axla-quoin-lane-voicing
  2 H ax-annotation-line-sweep
  3 I rbst-primitive-type-catalog
  4 B chapbook-provenance-story
  5 C chapbook-federation-story
  6 D chapbook-containment-story
  7 E readme-current-sweep
  8 K readme-identity-single-telling
  9 L readme-roadmap-promise-trim
  10 M readme-lode-single-section
  11 A handbook-federation-reconcile
  12 F agent-learner-load-test
  13 G docs-integrity-tail

123456789abcdefghijklmnopqrstuvwxyz
xx·································  J  2c
··xx·······························  H  2c
····xx·····························  I  2c
······xxx··························  B  3c
·········xx························  C  2c
············xx·····················  D  2c
··············x····x···············  E  2c
·····················xx············  K  2c
·······················xx··········  L  2c
·························xx········  M  2c
···························xx······  A  2c
······························xx···  F  2c
································xxx  G  3c
```

## Steeplechase

### 2026-07-06 11:31 - ₢BtAAG - W

Terminal docs-integrity sweep run and greened as the heat's last pace: tabtarget-existence gate fixed three stale examples in the BUK core context doc (retired rack imprint form, tt/foo placeholder, retired TestFixture frontispiece — all replaced with live tabtarget forms); retired-reference gate fixed the acronyms RBH0 entry's never-built rbw-h{o,p}/rbw-H{O,P}* colophon pattern to the live rbw-o + rbw-O* and rbw-gP* families; anchor-resolution gate homed as the standing rbq_qualify_anchors check in fast qualify (file-to-file: README ](#X) refs plus rbyc_common.sh minted anchors against <a id> targets), proven loud on seeded dangles in both consumed sets. Agent-learner eval handoff fully absorbed: ordain success signal and failure path taught in the first-build track after verification against a live ordain transcript, plus the DF-8 mode-check pointer; crash course teaches the expected-vs-actionable validation-failure tell and branches its closing on the depot probe state; both tracks carry a probe-glyph legend at first use; memo disposition updated. Handbook-render fixture 8/8, shellcheck clean, final single-pass all-gates run green.

### 2026-07-06 11:27 - ₢BtAAG - n

Agent-learner eval handoff absorbed, all four findings: director first-build teaches ordain's verified success signal (QUEUED/WORKING/SUCCESS poll ticks, automatic vouch, the closing 'This hallmark feeds:' roster) and the failure path (red ERROR naming phase + Cloud Build status, Cloud Console link at submission, fresh-hallmark re-run safety) — prose written only after verification against the 2026-07-05 ordain transcript — plus the DF-8 ride-along mode-check pointer (render the vessel regime to see its ordain mode); crash course Step 4 teaches the expected-vs-actionable validation-failure tell (expected names a field and its fix; no field named means real problem); the crash course closing now branches on the depot probe state instead of declaring success unconditionally; and both tracks carry a one-line probe-glyph legend at first use. Memo disposition section updated to record the sweep's absorption and the transcript evidence.

### 2026-07-06 11:08 - ₢BtAAG - n

Docs-integrity gates run and greened: tabtarget-existence sweep found three stale examples in the BUK core context doc (imprint-form rbw-cr.Rack.tadmor and the retired TestFixture frontispiece/fixture names replaced with the live rbw-cC.Charge and rbw-tf.FixtureRun param1 forms, tt/foo placeholder replaced with a real quench tabtarget); retired-symbol sweep found the acronyms RBH0 entry claiming the never-built rbw-h{o,p}/rbw-H{O,P}* colophon pattern, corrected to the live rbw-o + rbw-O* onboarding family and rbw-gP* payor guides; README internal refs and rbyc linked-term anchors all resolve. Anchor gate now stands in qualify: new rbq_qualify_anchors (file-to-file, no network) checks README ](#X) refs and rbyc_common.sh minted anchors against <a id> targets, wired into rbq_qualify_fast, proven loud on a seeded dangle in both consumed sets and green after revert.

### 2026-07-06 10:48 - ₢BtAAF - W

Ran the differential agent-learner eval against the crash course and director first-build tracks: eval artifact committed (anti-inference persona, five application/misconception questions per track, two-element rubric, blind scoring, Goodhart rotation note), delta reported (crash course +8.5, first-build +7.5 out of 10 — teaching prose carries the load, step structure and probes supply the stripped residue), sixteen critic findings dispositioned: five absorbed into handbook prose, four handed to the docs-integrity sweep via docket pointer, the rest declined as false positives or deliberate choices in the memo

### 2026-07-06 10:47 - ₢BtAAF - n

Design and first run of the differential agent-learner handbook eval: committed eval artifact (anti-inference learner persona, five application/misconception questions per track, two-element rubric, blind-scoring protocol, Goodhart rotation note) plus results — crash course full 10/10 vs stripped 1.5/10 (+8.5), director first-build full 10/10 vs stripped 2.5/10 (+7.5), so the teaching prose demonstrably carries the load on both tracks, with step structure and probe lines supplying the residual stripped-variant signal. Synthetic critics surfaced sixteen pedagogy findings; five absorbed now (crash course: read-only commands log too, per-cmd/history log filename patterns shown, buw-/rbw- workbench wrinkle added to the letter rule; first-build: GAR expanded on first use, stamp placeholder unified to touchmark), four handed to the docs-integrity sweep via its docket, and the rest dispositioned as false positives or deliberate choices in the memo (notably: the critic-flagged absolute path is live per-machine rendering, and linked-term hyperlinks are the gloss the de-linked capture hid)

### 2026-07-06 10:47 - Heat - d

batch: 1 reslate

### 2026-07-06 10:13 - ₢BtAAA - W

Reconciled onboarding handbooks to keyless federation. Payor track: dropped roles-use-SA-keys framing and the governor-service-account promise, added the missing Affiance-a-foedus step (rbw-gPF walk + rbw-mA) between instaurate and levy, governor admission now brevets citizens onto mantles. Start-here: Director mantle replaces Director credentials; stale probe-aware framing retired. First-build: -attest arks taught as durable (sole carriers of GCB-attested digests, deleted only by abjure), rekon step lists all six durable arks and both rekon invocations fixed to pass hallmark not vessel. Crash course: Manor+Levy populate RBRD. Zipper: rbw-gPI description says OAuth client secret JSON, not key file. Confirmed no keyfile gate exists — probes are filesystem-only, so fresh-levy gauntlet was never blocked; defect was prose. All tracks render clean, shellcheck green.

### 2026-07-06 10:05 - ₢BtAAA - n

Reconcile onboarding handbooks to keyless federation: payor track drops the roles-use-SA-keys framing and the governor-service-account promise, gains the missing Affiance-a-foedus step (rbw-gPF IdP walk + rbw-mA) between instaurate and levy, and teaches governor admission as brevetting citizens onto mantles; start-here menu requires the Director mantle instead of Director credentials and sheds its stale probe-aware framing; first-build track teaches -attest arks as durable (the only GCB-attested digests, persisting until abjure), lists all six durable arks at the rekon step, and fixes both rekon invocations to pass the hallmark rather than the vessel; crash course names Manor+Levy as what populates RBRD; rbw-gPI zipper description now says OAuth client secret JSON, not key file (tabtarget context regenerated)

### 2026-07-06 09:14 - ₢BtAAM - W

README lode single-section: made the `### Supply Chain` appendix the one home for all lode prose. Pulled the three noun definitions (Lode/Touchmark/Reliquary) down out of the Foundry glossary (removed as ### subsections, anchors relocated), collapsed Capture/Conclave/Ensconce to 1-2 sentences each (dropping drift-prone sequencing and a Reliquary restatement), and added a one-clause-each roll-call with fresh anchors for the six uncovered lode verbs (Underpin/Immure/Presage/Divine/Augur/Banish) so any future yawp has a landing. Also reduced the DisappearingUpstream appendix's verbatim Lode re-definition to story-plus-link while preserving its trust-grade story (verified-against-published / recorded-at-acquisition), which the Lode entry defers to. Anchor sweep green throughout: 128 refs / 130 anchors / 0 dangling (was 122/124; +6 roll-call anchors and their self-links, nouns net-zero).

### 2026-07-06 09:14 - ₢BtAAM - n

Federated the four scattered Lode-family definitions (Lode, Touchmark, Reliquary, Capture) into a single Supply Chain subsection in the README glossary, and landed the six remaining Lode operations (Underpin, Immure, Presage, Divine, Augur, Banish) as anchored terms so each handbook step has a glossary home.

### 2026-07-06 08:46 - ₢BtAAL - W

Trimmed the README Roadmap appendix to the cinched promise-less rule: each of the 7 entries reduced to a single feature-shape sentence, stripping mechanism detail (WireGuard, private pools, macOS-VM lifecycle), deferral timelines, and dismissive rationale — notably the Podman 'no architectural advantage' dismissal the docket named. Reframed CDN-Aware IP Gating and Crucible-to-Crucible from problem-statements into feature shapes (preserving their anchors) rather than deleting. No entries added, no <a id> anchors removed. Anchor sweep green: 122 distinct ](#X) refs, 0 dangling against 124 anchors. Appendix ~25 lines to 13.

### 2026-07-06 08:46 - ₢BtAAL - n

README future-features section: collapse each "under consideration" entry to a single-sentence description, stripping the elaboration/rationale paragraphs (Crucible Conduit, Bottle Credential Custody, VPC Service Controls, Cosign Signing, CDN-Aware IP Gating, Podman Support, Crucible-to-Crucible Networking). Anchors and linked terms preserved; the deferred features now read as a scannable roster rather than prose.

### 2026-07-06 08:32 - ₢BtAAK - W

Single-told the federated identity model in README. Stripped drift-prone identity mechanics from the three narrative regions (Foundry Lifecycle role narrative, Establishment & Provisioning, Admission & Access) down to story-plus-link, deferring every fact to its appendix linked term; kept the keyless-posture theses and the org+IdP prerequisite as story. Added an Escheat landing entry beside Jilt with its own <a id="Escheat"> anchor (payor manor-hygiene terrier sweep, Rehearse's mutating counterpart). Both anchor-sweep invariants green: 122 link targets resolve, all 51 minted rbyc_common.sh anchors resolve. Notched as a70c7ce09 before wrap.

### 2026-07-06 08:25 - ₢BtAAK - n

README federated-identity single-telling: strip drift-prone identity mechanics from the three narrative regions (Foundry Lifecycle role narrative, Establishment & Provisioning, Admission & Access) down to story-plus-link, deferring every fact to its appendix linked term (Mantle/Avow/Sitting/Don/Install/Levy/Gird/Brevet/Muniment); keep the keyless-posture theses and the org+IdP prerequisite as story. Add an Escheat landing entry beside Jilt with its own <a id="Escheat"> anchor (payor manor-hygiene terrier sweep, Rehearse's mutating counterpart). Both anchor-sweep invariants green: 122 link targets resolve, all 51 minted rbyc_common.sh anchors resolve.

### 2026-07-06 08:01 - Heat - d

batch: 1 reslate

### 2026-07-06 07:59 - ₢BtAAE - W

README brought current against the system and the three chapbook stories. The one live dangle fixed: the handbook-consumed Reliquary anchor now has its definition section. Thirteen chapbook-surfaced ashlars registered on the broadside: Conclave, Ensconce, Ark, the Chain Links family (Yoke, Feoff, Anoint, Drive), Foedus, Instaurate, Novate, Espy, Recognosce, Attribution, Transit. Two factually stale federation definitions corrected: Affiance seats a provider under the standing pool (Instaurate founds the pool), Jilt deletes one foedus's provider while the pool stands. Rosters reconciled to disk: nineveh and fdkyclk nameplates added, dead rbev-sentry-deb-airgap dropped, kroki/fdkyclk/graft-demo/busybox vessels and the moved ccyolo build-context reflected, rbrw.env and the foedera library added to the tree. RBRW and RBRF registered in the regime list, appendix, and tree. Stale rbw-h tabtarget example fixed to rbw-f. Provenance-tale diagram pair emplaced in Build and Retrieve (deferred here from the diagram pace). Verified programmatically: all 52 handbook-consumed anchors resolve, zero dangling internal refs. rbyc_common.sh: unconsumed pre-federation Charter/Knight constants retired. Adjacent repairs deliberately spun to follow-on paces: identity single-telling, roadmap promise-trim, lode consolidation.

### 2026-07-06 07:50 - Heat - d

batch: 1 reslate

### 2026-07-06 07:38 - Heat - S

readme-lode-single-section

### 2026-07-06 07:38 - Heat - S

readme-roadmap-promise-trim

### 2026-07-06 07:21 - Heat - S

readme-identity-single-telling

### 2026-07-06 07:04 - ₢BtAAE - n

README brought current against the system and the chapbook stories: the consumed-but-missing Reliquary anchor defined (the one real dangle — the bind handbook links it heavily); chapbook-surfaced ashlars registered on the broadside (Conclave, Ensconce, Ark, and a new Chain Links glossary subsection homing Yoke/Feoff/Anoint/Drive from the provenance tale; Foedus, Instaurate, Novate, Espy, Recognosce, Attribution from the federation tale; Transit network from the containment tale); two factually stale federation definitions corrected (Affiance seats a provider under the standing pool — Instaurate founds the pool; Jilt deletes one foedus's provider while the pool stands); nameplate roster completed (nineveh, fdkyclk) and vessel tree reconciled (dead sentry-deb-airgap dropped, kroki/fdkyclk/graft-demo/busybox added, ccyolo build-context); RBRW + RBRF regimes added to list, appendix, and tree; stale rbw-h tabtarget example fixed to rbw-f; provenance-tale diagram pair emplaced in Build and Retrieve (deferred here from the diagram pace); grammar sweeps (an Sitting/a Avowal). rbyc_common.sh: unconsumed pre-federation Charter/Knight constants retired, drift-prone term count dropped from comment. Verified: all 52 handbook-consumed anchors resolve, zero dangling internal refs.

### 2026-07-05 22:45 - ₢BtAAD - W

Authored RBSYC, the aspirant containment chapbook: how a crucible runs foreign code and lets it reach only what its nameplate sanctions — kludge, charge (the ordered health chain that raises the sentry's enforcement before the workload's first packet), the standing guard (deny-by-default iptables, frozen DNS allowlist, namespace inheritance, dropped NET_ADMIN, route ownership), the one sanctioned path, the sorties, and quench closing on the moriah echo. Sixteen interactions across six movements; quoined cast with the bottle as antagonist (the ifrit uncast, named only in prose, because rbsi_ifrit and siblings are dangling mapping refs); the sortie movement compressed to four outer-category interactions (DNS/egress/kernel/integrity) plus the foreign off-path Palisade, per operator direction. Ratified against the siege+tadmor oracle via a five-agent grounding workflow and a four-verifier adversarial ratify workflow. Two correctness blockers fixed: NET_RAW is deliberately KEPT on the tadmor bottle (cap_add) so the ifrit's raw-socket attacks are genuine — containment is the sentry's deny-by-default filter, not a missing capability; and the tadmor allowlist holds two names, not one. Mounted under RBS0 == Aspirants as rbsyc_aspirant; RBSYC acronym registered and legend C un-reserved. Swept 'beat' to 'interaction' across the RBSYF body (4 sites) and the paddock (2 sites) — beat is reserved for JJK's martingale vocabulary; the AXLA unit is interaction. Two surfaced gaps recorded in-file: quench has no operation quoin where charge does; the adversary is uncatalogued (dangling rbsi_ anchors + no sortie/exec operation quoin). Kept one genre-first: a self-interaction (bottle acting on its own cage), operator-ratified at wrap.

### 2026-07-05 22:45 - ₢BtAAD - n

Authored the aspirant containment chapbook RBSYC-chapbook_containment.adoc: the tale of how a crucible cages foreign code — quoined cast across station/ward venues with the bottle as antagonist, six movements from the local kludge through the ordered charge, the standing guard, the one sanctioned path, and the sorties, closing on the quench and the moriah echo (containment as the guard's property, not the supply chain's). Ratified against the siege suite: builder rbtdro_kludge_tadmor_standalone plus the tadmor fixture's security-case registry, with the Windows Docker Desktop off-path reply carried as a surveyed foreign deviation. Mounted under RBS0 Aspirants, RBSYC acronym registered, RBSY legend C unreserved. Two surfaced gaps recorded as graduation conditions: quench has no operation quoin where charge does; the adversary is uncatalogued — rbsi_ifrit and siblings dangle without definition sites and the sortie/exec surface has no operation anchor. Also swept the RBSYF federation chapbook beat->interaction, completing the Lapidary reservation of beat for JJK vocabulary.

### 2026-07-05 22:20 - Heat - d

paddock curried: sweep beat->interaction (Lapidary: beat reserved for JJK martingale vocabulary; AXLA unit is interaction)

### 2026-07-05 21:26 - ₢BtAAC - W

Authored the aspirant federation chapbook RBSYF-chapbook_federation.adoc: quoined cast of ten catalogued lanes across IdP/manor/depot venues, seventeen beats in seven movements telling manor founding through attributed action with the withdrawal shadow side, each beat cited to its axvo_method anchor or carrying an attended/foreign license. Ratified beat-by-beat against the freehold-establish arc, the parley roll round-trip, and polity-denial's band and isolation proofs; every attribute reference mechanically verified against RBS0's mapping. Mounted under RBS0 Aspirants, RBSYF acronym registered, RBSY legend F unreserved. Two surfaced gaps recorded as graduation conditions: the access arc (avow/don, sitting lifecycle, mantle probe, attribution read) has civic nouns but no operation anchors; the IdP-console registration walk lacks a citable guide home. Diagram render deferred — not part of this pace.

### 2026-07-05 21:23 - ₢BtAAC - n

Author the aspirant federation chapbook RBSYF — the tale of keyless attributed action: manor instaurate/affiance, depot levy, the sitting (avow), gird, don, brevet, and the attributed act, with the withdrawal shadow side (unseat/attaint/jilt) rehearsed on the manor roll. Quoined cast of ten catalogued lanes across IdP/manor/depot venues under the rbsyf sprue; seventeen beats across seven movements, each cited to its axvo_method operation anchor or carrying an attended/foreign license; ratified beat-by-beat against the freehold-establish arc, parley round-trip, and polity-denial bands. Two surfaced gaps recorded as graduation conditions: the access arc (avow/don, sitting lifecycle, mantle probe, attribution read) has civic nouns but no operation anchors; the IdP-console registration walk has no citable guide home. Mounted under RBS0 Aspirants beside the provenance chapbook; RBSY legend F unreserved and RBSYF acronym entry registered.

### 2026-07-05 20:58 - ₢BtAAB - W

Ratified the provenance chapbook (RBSYP) against its oracle and delivered the derived diagram, closing the ratification tail the docket held. Fixture walk: all 35 interactions across the six movements checked against the eight cases of rbtdro_onboarding.rs in registered order — no interaction contradicts a passing case; the two declared compressions hold (vouch exercised through the summon/rekon gates rather than the batch verb; the jupyter case as a drive-tail rerun of Movement III), and the verification-tail wrests of the conjure and bind cases are lawfully compressed into Movement VI's single told-once wrest. Mechanical verification: zero hits for reserved word 'beat'; a scratchpad Python checker walked every annotation read window per AXLA chapbook law (arities, sprue lexing, injective cast, lane resolution, LIFO open/closed pairing with from/to reversal validation, legend-first-on-backtick-stream) — sole violation is the Movement II anoint interaction (unlicensed, contract read unfulfilled), exactly the documented Surfaced Gap held out of scope. Diagram: authored diagrams/rbdgp_provenance-tale.puml strictly from the marker skeleton (venues as boxes, the seven open/closed groups as Cloud Build activation bars, the seven legends as bolded arrow inscriptions, contracts lettered with census display texts, both surfaced gaps carried as notes); rendered through the canonical membrane (charged pluml, rbtdrc_pluml_render_diagrams, quenched) with the mojibake gate clean and the three existing diagram pairs re-rendering byte-identical; unfurled light/dark and operator-accepted. New rbdg family member rbdgp_ (provenance) minted mid-pace, grep-clean, ratified by operator acceptance of the committed artifact. README emplacement of the diagram deferred to the README-reconciliation stream; a marker-skeleton-to-puml derive-tool was surfaced as an itch candidate and left with the operator.

### 2026-07-05 20:57 - ₢BtAAB - n

Commit the operator-accepted provenance-tale SVG pair (light + dark), rendered from the committed rbdgp source through the canonical pluml crucible render case — mojibake-gate clean, existing diagram pairs re-rendered byte-identical. Source and SVGs now stand in lockstep per the RBDG convention; README emplacement deferred to the README-reconciliation stream. Size limit raised per operator approval (two ~39KB rendered SVGs, in line with committed rbdg siblings).

### 2026-07-05 20:48 - ₢BtAAB - n

Author the provenance-tale sequence diagram source, derived from the RBSYP chapbook marker skeleton: venues as participant boxes, open/closed vertical groups as Cloud Build activation bars, legends as bolded arrow inscriptions, contract citations lettered with their census display texts, and the two surfaced gaps (uncatalogued anoint, verbless airgap base handoff) carried as notes. New rbdg family member rbdgp_ (provenance) minted provisionally, flagged for operator ratification at diagram review. ASCII-only per PCG.

### 2026-07-05 19:15 - ₢BtAAI - W

Catalogued the six primitive types the ax-annotation sweep surfaced (rbst_string/url/version/integer/duration/username) in RBS0's Type Voicings section, on the established rbst_ipv4/port/netmask template: each gets a mapping-section entry, an [[anchor]], a //axl_voices motif voicing, and a two-line definition placed before the Google-specific Types header. Legacy //axl_voices form used throughout (operator-ratified this pace): the new implicit-voices annotation form has zero instances repo-wide and none exists for type voicings, so //axl_voices remains the sole live convention (370 sites). Motif assignments, reuse-only, no new motif minted: string/url/version/duration -> axtu_string; integer/username -> axtu_xname. Two judgment calls: rbst_integer -> axtu_xname follows the in-section numeric precedent (rbst_port and rbst_netmask are integers voiced to xname); rbst_duration -> axtu_string because a GCB timeout like 3600s carries a unit suffix rather than being a pure scalar. Verified: census confirms all six references now resolve against the codex mapping (the only remaining RBS0 miss is the pre-existing ${project_id} shell-var false positive); AXLA untouched so no motif minted; ax-annotation discovery grep still zero-hit. This closes the caveat carried forward from the ax-annotation-sweep pace (8aabd6ce3): the 9 dangling body references across 6 uncatalogued types now resolve.

### 2026-07-05 19:15 - ₢BtAAI - n

Define the six deferred `rbst_*` primitive types in RBS0's type catalog — `rbst_string`, `rbst_url`, `rbst_version`, `rbst_integer`, `rbst_duration`, `rbst_username` — clearing the documented dangles held for this cantled pace (9 sites across the spec). Each gains a mapping-section attribute reference plus a definition site with format/bounds prose and an `axl_voices` motif (`axtu_string` for the string-family types, `axtu_xname` for the integer and username identifiers).

### 2026-07-05 19:02 - ₢BtAAH - W

Verify-and-wrap: the mechanical annotation sweep landed earlier at 8aabd6ce3 (discovery grep zero-hit, three families swept), and this pace confirmed the two residual Done-when clauses against the current post-quoined-migration state. Clause 2 (chapbook parses under the revised nesting law): traced RBSYP end-to-end against AXLA's quoined law — opener carries one voicing (axd_quoined) + sprue rbsyp; 3 venues each one backtick inlay; 8 actor lanes each one attribute-ref quoin with an injective cast; all ~30 interactions resolve from/to/contract on the attribute stream to prior lanes, every legend readable in-window, every axd_open/axd_closed group depth-1 and LIFO-balanced with each close's from-lane reversing its open's to-lane. The sole covenant breach (Movement II anoint, quoinless-unlicensed) is the file's own documented Surfaced Gap, pre-existing, not a sweep defect. Clause 1 (attribute references resolve): census across all 11 touched sheaves — RBSYP and all 10 siblings fully clean; RBS0 shows only the 6 documented deferred rbst_* primitive-type dangles (9 sites, held for the cantled type-catalog pace ₢BtAAI). One pre-existing latent dangle the caveat had not named — {rbtgi_depot_s} at RBS0:3226, a plural variant undefined since 2026-04-15 (f034436128) — was fixed here by adding the :rbtgi_depot_s: definition (same anchor, pluralized text) per the established variant pattern; re-census confirms it now resolves. The {project_id} at RBS0:5115 is a false positive (${project_id} shell var inside a code span), not a bug.

### 2026-07-05 19:02 - ₢BtAAH - n

Mint `:rbtgi_depot_s:` plural variant for the `rbtgi_depot` quoin — the "RB Depots" replacement text pointing at the existing depot anchor, matching the `_s` variants already present on its siblings (repository, depot_project, image).

### 2026-07-05 17:28 - ₢BtAAJ - W

AXLA gains a required two-valued cast voicing on the chapbook opener — axd_sprued or axd_quoined, no unmarked reading (the internal/external precedent). Quoined: lane identity IS the catalogued quoin, interactions read from/to/contract on the attribute stream, injective fully-catalogued cast demanded, legend reads first on the backtick stream; the mature voicing eligible casts should use. Sprued: the nursery voicing retained for pending or duplicated casts, with quoin-pendency named in lane prose as an aspirant's recorded gap. RBSYP flipped to quoined (8 lane tokens dropped, 28 interactions re-referenced by quoin; anoint breach and attended handoff shapes preserved); JJS session chapbook opener declares axd_sprued; acronym map RBSYP entry trued. Word choice: literal participles rhyming with attended/grouped/inlaid, both already live house prose (buo-sprued, quoined corners).

### 2026-07-05 17:28 - ₢BtAAJ - n

axla-quoin-lane-voicing: mint the chapbook cast-voicing pair axd_sprued/axd_quoined — every axhcb_chapbook opener now declares exactly one, no unmarked reading. Sprued keeps sprue-local lane tokens on the backtick stream; quoined lanes take the catalogued quoin as their identity, interactions reading from/to/contract on the attribute stream, with venues and legends riding the opener's sprue in both voicings (hence the unconditional sprue inlay). Actor/interaction lookahead arities, legend read position, closed-group reversal, and the structural laws (injective quoined cast, voicing required on opener) rewritten per-voicing in AXLA. JJS session chapbook voiced sprued; RBSYP provenance chapbook migrated to quoined — eight lane tokens retired, every interaction re-referenced through the census — and its acronym entry updated.

### 2026-07-05 17:03 - Heat - S

axla-quoin-lane-voicing

### 2026-07-05 17:02 - ₢BtAAH - n

ax-annotation-line-sweep: relocate project-universe tokens off //ax annotation lines into the read streams per revised AXLA law. RBSYP and the JJS session chapbook — chapbook markers (opener sprue, actor lanes, interaction from/to, legends) rewritten to backtick-inlay form; 'beat' purged for 'interaction' in RBSYP prose. Eight axvd_sheaf sheaves (RBSCV/RBSPC/RBSHR/RBSOB plus four JJS aspirants) migrated to the opening-sentence form, imprimatur moved off the line. RBS0 axvr_variable types moved off annotation lines into definition bodies (43 blocks). Discovery grep repo-clean. Caveat: 9 RBS0 blocks reference 6 uncatalogued primitive types (rbst_string/url/version/integer/duration/username) whose body refs dangle, held for the cantled type-catalog pace.

### 2026-07-05 16:57 - Heat - S

rbst-primitive-type-catalog

### 2026-07-05 12:06 - Heat - d

batch: 1 reslate

### 2026-07-05 12:03 - ₢BtAAB - n

AXLA chapbook amendment + annotation-universe definition repairs: annotation lines carry ax-universe tokens only (motif + axd_ dimensions) with project tokens moved to read streams; chapbook marker arities re-specified (opener sprue, lane token, interaction from/to as backtick-stream inlays); new axhcv_venue marker and axd_open/axd_closed/axd_legend dimensions with read-window law; reserved word 'beat' purged for 'interaction'; axvr_variable law revised to the typed/motif split so project type terms read from definition text. RBSYP committed in its venue/group/legend rewrite with knowingly malformed annotation lines — the chivvied ax-annotation-line-sweep pace repairs them to the new law

### 2026-07-05 11:57 - Heat - S

ax-annotation-line-sweep

### 2026-07-05 10:56 - ₢BtAAB - n

Author the aspirant provenance chapbook RBSYP — first AXLA chapbook instance in RB: 8 cast lanes, 28 beats in six movements ratified against the onboarding-sequence fixture; mount it in RBS0 Aspirants with a genre-bearing cartouche; record the RBSY family mint and rbsyp sprue in the acronym map, with the two surfaced contract gaps (uncatalogued anoint, verbless airgap base handoff) flagged in-file

### 2026-07-05 10:34 - Heat - f

racing

### 2026-07-05 10:30 - Heat - T

handbook-federation-reconcile

### 2026-07-05 10:29 - Heat - r

moved ₢BtAAA after ₢BtAAE

### 2026-07-05 10:29 - Heat - d

batch: paddock, 1 reslate, 6 slate

### 2026-07-05 10:28 - Heat - D

restring 1 paces from ₣A6

### 2026-07-05 10:28 - Heat - N

rbk-08-onboarding-readme

