# Heat Trophy: rbk-08-mvp-handbook-revamp

**Firemark:** ₣A6
**Created:** 260409
**Retired:** 260705
**Status:** retired

## Paddock

## Character

Iterative discovery work. This paddock is a living document that evolves through **interview-implement-interview cycles**, not a one-shot design phase. Implementation paces teach us things the interview cannot; interview paces integrate what was learned and unblock the next build. Cognitive posture: patient exploration over productive rush, willingness to revise decisions when implementation contradicts them.

Teaching pedagogy is distinct from setup choreography. This heat is about helping learners internalize Recipe Bottle's **novel conceptual apparatus** — vessels, depots, hallmarks, vouching, ordain modes, charters, knights, pentacles, bottles, sentries — not about walking them through prescribed setup steps. Setup steps happen, but they are in service of understanding. General container and GCP knowledge is assumed.

## Prerequisites

`₣A3` (rbk-mvp-3-onboarding-guides) must complete and retire before this heat races:
- `₢A3AAI` (buh-handbook-rename-sweep) — bug→buh module rename
- `₢A3AAJ` (rbgm-split-to-rbho-rbhp) — file structure in place
- `₢A3AAF` (docs-alignment-and-cross-machine-validation) — final A3 validation

After A3 retires, this heat consumes `Tools/rbk/rbho_onboarding.sh` as the home for new handbook content. New tabtargets use a **two-colophon scheme**: `rbw-o` (lowercase, terminal) for the onboarding StartHere entry point, and `rbw-O*` (uppercase family) for individual onboarding handbook tracks (e.g. `rbw-Occ.OnboardingCrashCourse.sh`). This replaces the old `rbw-gO*` onboarding family from A3. The case-split family is a deliberate starting-letter differentiation for clarity. (The initially-proposed `rbw-h?` prefix was found to collide with the existing Hallmark tabtarget family — see Provenance.)

## Context — The Malformation

A3 produced onboarding guides organized around four "role tracks" (retriever, director, governor, payor). The decomposition followed **authorization boundaries** — which roles can perform which actions. But learners don't come to the tool with a role; they come with an **intent**. They want to explore local containers, evaluate the cloud build engine, set up billing, or administer credentials. Role-tracks answered "what can I do next?" but never "what does this mean?" The teaching was setup-oriented, not understanding-oriented.

Breakthrough insight (from the interview distilled in this paddock): **learners decompose along intent and goal, not along authorization**. The same probe can appear in multiple handbooks with different teaching contexts. The same role can appear in multiple tracks for different learner goals. Authorization gates actions; intent shapes learning.

## Design Decision Record

### Frame 4-refined — Handbook as Self-Describing Document

Every handbook tabtarget prints a complete teaching document with live probe results interleaved. Units alternate between:

- **Verified units**: teaching prose + embedded probe status (✓/✗ with concrete values) + tabtarget reference to run for fix
- **Teaching-only units**: pure conceptual explanation, no probes, no checkmarks

A learner reads top-to-bottom for teaching; an expert scrolls for the first red probe; a mid-learner reads around what's broken. **One format, three reading strategies** — no mode switching, no state-based branching logic, no walkthrough-vs-reference conflation.

### Probe-Aware Menu Routing

`tt/rbw-o.OnboardingStartHere.sh` runs quick triage probes and highlights menu items based on repo state. Unpopulated RBRR highlights the payor path. Populated RBRR plus existing RBRA highlights "find your role." Static menu items remain visible so learners can override routing — probe-aware is guidance, not gating.

### Handbook Output Is Ephemeral

**Handbook steps don't log.** Teaching output is ephemeral UX; logs are for commands that change state. A `buh_*` display call produces no log file. Commands invoked from within handbook-guided flow still log normally. The split is between **teaching conversation** (ephemeral) and **persistent operations** (logged).

### No Step Recording, No Mode Switching

Earlier A3 drafts tracked steps taken and supported walkthrough/reference mode switching. Both removed. Retrospection comes from re-running the handbook (which re-displays teaching with current probe state). If a future switchboard layer wants to provide history, it reads logs and probes without any track instrumentation. Tracks stay stateless and idempotent.

### Monotonic vs Cyclic Probes

Some probes check stable state (RBRR populated, SA files present, reliquary inscribed) — once green, they stay green for a long time. Others check cyclic state (OAuth tokens expire, crucible charge state is transient) — red-after-green is routine. Teaching framing adjusts to cadence:
- **Monotonic**: brief "here's what this is" on first encounter, terse diagnostic on re-check
- **Cyclic**: "this resets, here's how to refresh" framing, no shame in running the refresh tabtarget

### Per-Unit Brevity Discipline

Each unit ≈ 3–5 sentences of teaching + 3–6 probe lines + one tabtarget reference. A full track of 8 units renders in ~240 lines — skimmable in 2 minutes, deep-readable in 10. Total track length stays disciplined by per-unit discipline, not by artificial caps.

### Teach the Pattern, Not the Inventory

The regime landscape has 13 regimes and a consistent `rbw-r{letter}{r|v}` tabtarget pattern. The crash course teaches the **pattern**, not the inventory. Once a learner owns the pattern, they can discover any regime's tools by knowing one letter. Six regimes are user-facing (teach on demand); the rest are deferred or invisible.

### Shared Probes, Separate Teaching

The same probe appears in multiple handbooks. RBRR probes appear in the payor handbook ("you are responsible for filling this") *and* the director handbook ("your payor established this; verify before building"). Same ground truth, different teaching context. This repetition is load-bearing — different learners in different sessions, each deserves their own framing.

### Diagnostic Failure as First-Lesson Principle

Running `tt/rbw-rrv.ValidateRepoRegime.sh` against incomplete state **fails diagnostically** — telling the learner exactly which fields need values. This is the crash course's first concrete teaching moment: *"In this system, configuration errors fail diagnostically. Trust the error messages."* Marshal Zero is internal release tooling and must not appear in onboarding content.

## Regime Filtering for Pedagogy

User-facing (teach on demand):
- **BURC** — project-level BUK settings (`.buk/burc.env`)
- **BURS** — per-developer station; `BURS_LOG_DIR` is the anchor
- **RBRR** — repository regime (depot identity, shared across team, monotonic)
- **RBRP** — payor regime (payor's own identity, stable across depot rotations)
- **RBRV** — vessel regime (per-vessel image spec)
- **RBRN** — nameplate regime (per-crucible deployment config)

Deferred / barely mentioned: RBRS (future), RBRM (future), RBRO (invisible validator), RBRA (tooling-created, not hand-edited), BURE/BURD/BURX (infrastructure).

Cleanup action items surfaced by interview:
- `Tools/cccr.env` — delete (cruft)
- `.rbk/rbje_compose_probe.env` — needs scrubbing, out of scope for onboarding

## Rough Track Roster

Working titles, evocative and distinct from the prior role placeholders. **All names are rough** and will be refined in later interview passes. Each track is a single `tt/rbw-O??.Onboarding{TrackName}.sh` tabtarget (uppercase-O family) rendering a complete teaching document in Frame 4-refined format. Final track acronyms (the `??` slots) are assigned during the drafting pace (₢A6AAA Artifact 3).

**Vessel/nameplate assignments**: Tracks that involve hands-on container work should use concrete vessels shipped with the project, not abstract examples. The four shipped vessels and their teaching roles:

| Vessel | Mode | Teaching role |
|--------|------|---------------|
| `rbev-sentry-deb-tether` | conjure (tether) | Sentry firewall image, daily-iteration variant; shared `common-sentry-context/` Dockerfile |
| `rbev-sentry-deb-airgap` | conjure (airgap) | Sentry firewall image, airgap-built variant; same shared sentry context |
| `rbev-bottle-ccyolo` | conjure | First crucible experience — Claude Code in a sandbox (₢A6AAG creates this) |
| `rbev-bottle-ifrit-tether` | conjure (tether) | Adversarial testing vessel — daily-iteration variant, kludge-friendly (`common-ifrit-context/Dockerfile.tether`) |
| `rbev-bottle-ifrit-airgap` | conjure (airgap) | Adversarial testing vessel — airgap-built variant, supply-chain disciplined (`common-ifrit-context/Dockerfile.airgap`, FROM forge) |
| `rbev-bottle-ifrit-forge` | conjure (tether) | Build-time fixture: warms cargo cache + apt deps for airgap-ifrit; consumed input, not a runtime image |
| `rbev-bottle-plantuml` | bind | Upstream image pinned by digest — teaches bind mode |
| `rbev-bottle-graft-demo` | graft | Teaching-only graft vessel — local image pushed to GAR, no Dockerfile in project, no nameplate |

| Nameplate | Sentry | Bottle | Teaching role |
|-----------|--------|--------|---------------|
| `ccyolo` | sentry-deb-tether | bottle-ccyolo | First crucible, kludge-only, Claude OAuth subscription required |
| `tadmor` | sentry-deb-tether | bottle-ifrit-tether | Daily security test authoring, kludge-friendly iteration |
| `moriah` | sentry-deb-airgap | bottle-ifrit-airgap | Airgap chain demonstration target (Building for Crucibles); cloud-built hallmarks; same security suite as tadmor |
| `pluml` | sentry-deb-tether | bottle-plantuml | Bind mode demonstration |

### Foundation (everyone)

1. **Crash Course: Reading the Map** — universal prerequisite. Tabtargets, regimes, log placement, station vs versioned, the `rbw-r{letter}{r|v}` pattern. Local-only, no cloud, no image operations. Teaches BURC, BURS, RBRR at a surface level — enough to navigate the rest. Provisional acronym: `Occ`. **No vessels.**

2. **Take Your Station** — joining an existing configured project. Verify your RBRA is placed correctly, run access probes, confirm you can operate in your role(s). Shortest path, mostly probe-driven with minimal teaching. **No vessels.**

### Role-intent tracks

3. **Kindle Your First Crucible** — crucible explorer. Kludge ccyolo locally, charge, shell in, authenticate Claude Code via OAuth copy/paste between bottle terminal and workstation browser. Teaches: crucible, charge/quench, sentry/pentacle/bottle composition, kludge mode for local hallmark synthesis. *First implementation target — simplest path, zero cloud, tightest feedback loop.* Prerequisite: Claude OAuth subscription. **Nameplate: `ccyolo`.** (Name flagged for "kindle" vocabulary overload — see Open Questions.)

4. **Establish the Depot** — payor ceremony (~15 min). GCP project creation, OAuth consent screen, billing, RBRR initial population. Teaches: depot, payor responsibility, consent dance, the relationship between payor identity (RBRP) and depot identity (RBRR). **No vessels.**

5. **Knight the Realm** — governor. SA administration: create governor/director/retriever SAs, issue RBRA credentials, distribute securely. Teaches: knight, charter, SA lifecycle, secret distribution discipline. **No vessels.**

6. **Receive Your Knighthood** — cross-cutting mini-track for directors and retrievers receiving their RBRA from a governor. Place the file correctly (path derived from RBRR), verify via access probe, understand sensitivity. May appear as a unit within Director/Retriever tracks or as a standalone for experienced users receiving new credentials. **No vessels.**

### Director subtracks (dependency-ordered)

The director subtracks follow a strict dependency chain: inscribe → tethered → enshrine → airgap. Each track requires the previous track's outcome. The learner builds real shipped vessels — the same ones they already met in the crucible track.

7. **Your First Ordination** — conceptual map, **no probes, no vessels**. Teaches the ordination pipeline shape: director pushes a pouch (build context) to GAR, Cloud Build produces an `-image` (the container) and an `-about` (the SBOM), then a separate job produces `-vouch` (provenance attestation). Introduces the tethered vs airgap egress mode fork. All teaching-only units.

8. **Inscribe the Reliquary** — hands-on. Get builder tool images (skopeo, docker, gcloud, syft) into GAR. Prerequisite for all Cloud Build operations — conjure's preflight fails without a reliquary. Teaches: reliquary as co-versioned toolchain, `RBRV_RELIQUARY` in vessel regime. **No vessel target** (reliquary is tool infrastructure, not a vessel build).

9. **Tethered Cloud Builds** — hands-on, first real cloud build. Tether pool has public internet; base images pulled from upstream during build. Wall-clock ~20 min — pedagogy needs to handle wait time (open question). Requires: reliquary inscribed. **Vessel: `rbev-sentry-debian-slim`** (conjure mode, learner already knows this vessel from the crucible track).

10. **Enshrine the Ancestors** — hands-on. Mirror upstream base images into GAR with content-addressed anchors for **all shipped conjure vessels** (sentry-debian-slim, bottle-ccyolo, bottle-ifrit). Produces `RBRV_IMAGE_n_ANCHOR` values per vessel. Enshrine itself requires reliquary tool images. Teaches: supply-chain independence, anchor scheme, air-gap readiness, and the concrete scope of upstream dependencies the project carries. Requires: reliquary inscribed.

11. **Airgap Your Builds** — hands-on, airgap pool with no public internet. All dependencies pre-enshrined in GAR. Most restrictive build mode, full isolation. Requires: enshrined base images. **Vessel: `rbev-sentry-debian-slim`** (rebuild the same vessel from track 9, now airgapped — proving the enshrined bases work).

12. **Building for Crucibles** — ordain the vessels needed to charge a crucible from cloud-built hallmarks instead of kludged ones. The learner ordains the **ifrit forge** tethered (warming a cargo cache + pre-staging apt deps), then ordains **bottle-ifrit-airgap** on the airgap pool from the enshrined forge, drives the airgap hallmark into the **moriah** nameplate, charges, and runs the adversarial test suite — the same suite that runs against tadmor with kludged hallmarks. Closes the loop: the learner built the airgap security testing infrastructure themselves and validated containment against the same attacks. **Nameplate: `moriah`** (sentry-deb-airgap + bottle-ifrit-airgap). Tadmor remains the daily-iteration nameplate; it is not touched by this track.

### Evaluation tracks (split by concern)

Evaluators arrive with specific concerns, not a desire to tour everything. Three discrete tracks, each self-contained. **Framing matters**: these learners are evaluating, not executing. Critical eye, breadth over depth.

13. **Assay: Supply Chain** — evaluator concerned with build provenance. Tours all three ordain modes using concrete shipped vessels: conjure (**sentry-debian-slim**), bind (**bottle-plantuml**), graft (**bottle-graft-demo** — no nameplate, just a local image pushed to GAR). Teaches: SLSA attestation chain, SBOM via plumb, vouch verdicts per mode (full SLSA / digest-pin / GRAFTED), pouch as security boundary, reliquary as toolchain control. The trust hierarchy across modes is the central lesson.

14. **Assay: Containment** — evaluator concerned with runtime security. Tours the crucible architecture: sentry/pentacle/bottle composition, iptables + dnsmasq dual-layer enforcement, namespace isolation. Introduces the Ifrit and Theurge — adversarial AI-authored escape tests that actively validate containment. The evaluator sees the test results, understands the methodology (Claude Code sessions with full visibility authored the attacks, Ifrit delivers them, Theurge coordinates inside/outside observation). **Nameplate: `tadmor`** (the adversarial testing target).

15. **Assay: Operations** — evaluator concerned with operational model. Tours credential lifecycle (payor → governor → director/retriever), regime configuration, depot provisioning/unmake, and day-to-day operations (charge/quench, tally/vouch/summon cycle). Teaches: how a small team stands this up, what ongoing maintenance looks like, recovery procedures. **No vessels** — operational walkthrough, not hands-on building.

### Cross-cutting surfaces

- **Realm Dashboard** — reference surface showing all probes across all regimes. Equivalent to the old `rbw-gOr` reference mode, now in Frame 4-refined format. Reachable from StartHere menu and as its own tabtarget.
- **Secret Handling Primer** — the RBRA distribution lesson, appearing as a unit in Knight the Realm (governor side, distributing) and Receive Your Knighthood (recipient side, placing). Teaches operational handling knowledge — *what it is, how to handle, where it goes* — without teaching *how to read*.
- **Depot Relationships** — the user flagged wanting good intro text explaining "what a Depot is and how Directors dictate its content but Retrievers use it." This is a relational teaching unit that may appear verbatim in multiple tracks. Candidate for a shared snippet consumed by multiple handbooks.

### Remaining gaps

- **Graft demo vessel**: ₢A6AAH enrolled — creates `rbev-graft-demo/` with graft-mode `rbrv.env`.
- **Ifrit airgap pattern**: decided in ₢A6AAI — shared `common-ifrit-context/` with `Dockerfile.tether` and `Dockerfile.airgap`; three-vessel split (tether/airgap/forge); nameplate symmetry with new `moriah` for the airgap chain. Implementation gated on ₢A6AAv (`ifrit-symmetric-rename` — preconditioning rename sweep) → ₢A6AAV (forge + airgap + moriah + theurge fixture). Trajectory: tether variant retires when kludge cycle tolerates the forge dependency; no ETA.
- **Bind mode hands-on**: No director subtrack teaches bind explicitly. Bind is covered conceptually (track 7) and demonstrated in Assay: Supply Chain (track 13). Bind is simple enough (no Dockerfile, no build, just a digest pin in rbrv.env) that the conceptual + eval coverage likely suffices. Revisit if learner feedback says otherwise.

## README Anchor Inventory

Handbook sequences link into `${RBGC_PUBLIC_DOCS_URL}#{Anchor}` via `buh_tlt`. All anchors below are live in `README.md` as of `6c266d09`.

`Vessel`, `Hallmark`, `Vouch`, `Depot`, `Ordain`, `Conjure`, `Bind`, `Graft`, `Kludge`, `Enshrine`, `Reliquary`, `Pouch`, `Tethered`, `Airgap`, `Summon`, `Plumb`, `Tally`, `Levy`, `Payor`, `Governor`, `Director`, `Retriever`, `Charter`, `Knight`, `Nameplate`, `Enclave`, `Sentry`, `Pentacle`, `Bottle`, `Crucible`, `Charge`, `Quench`, `Tabtarget`, `Regime`, `Provenance`, `SBOM`, `RBRP`, `RBRR`, `RBRN`, `RBRV`, `BURC`, `BURS`

The `-image`, `-about`, and `-vouch` artifact suffixes are described within the `Hallmark` entry (plain English, no separate anchors).

**Not anchored** (intentional): Marshal Zero (internal release tooling), DiagnosticFailure (property not noun), ConsentScreen (Google's term), OAuth (described in prose within RBRP track).

## Teaching Voice

Teaching content is **vocabulary-first and relational**. The example the user articulated during interview: *"I want good intro text to what a 'Depot' is and how 'Directors' dictate its content but 'Retrievers' use it."* This is the shape of good teaching — nouns introduced in relation to each other, verbs connecting them, roles emerging from what they do to the nouns rather than abstract job titles.

Principles:
- Introduce nouns in context with concrete referents (a depot IS a GCP project; a vessel IS a configured image spec)
- Connect nouns via verbs (directors *ordain* vessels into depots; retrievers *summon* hallmarks from depots)
- Roles emerge from actions, not labels
- Each new term has a reason to exist at the moment of introduction
- Avoid "X is defined as Y" — prefer "when you X, you Y, which is called Z"

The load-bearing primitive for vocabulary-first teaching is `buh_tlt` from `Tools/buk/buh_handbook.sh`:
```bash
buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
```
Renders the noun as a clickable OSC-8 hyperlink to the public-docs anchor. Every new concept introduced to a learner should be a `buh_tlt` call on its first mention.

## Release Gate: Agent-Learner Evaluation

Every handbook ships with an **agent-learner comprehension test** that functions as a release gate. Design principles (established in interview, mechanics deferred to future pace):

- Test for **application**, not recall. Scenario questions, not definition lookups.
- **Misconception probes**: questions where surface-level understanding gives wrong answers.
- **Differential testing**: run eval against full handbook and against stripped/shuffled variant; signal is in the delta, not absolute score.
- **Persona anchoring**: learner agents use a system prompt that reduces inference-from-general-knowledge advantage — *"reason strictly from what the handbook tells you; if it doesn't say it, you don't know it."*
- **Synthetic critic alongside learner**: separate agent evaluates the handbook AS pedagogy (gaps, ambiguities, places a naive reader would stumble). Gate requires both learner and critic to pass.
- **Goodhart mitigation**: eval questions not in track author's working context; rotate periodically; allow human override with written justification.

Mechanics of the eval harness are **deferred** to a dedicated pace in this heat. The eval will co-evolve with the first implemented handbook.

## Process Rhythm

This heat runs on interview-implement-interview cycles, not linear execution.

| Pace kind | Produces |
|-----------|----------|
| **Interview-paddock** | Paddock refinement + spec for next implementation pace(s) |
| **Implement-{track}** | Working handbook tabtarget for one track, tested via agent-learner eval |
| **Interview-reconcile** | Absorption of implementation findings + spec for next implementation pace |

Interview paces are a **first-class pace type**, not preamble. Each interview pace unblocks N implementation paces; each implementation pace generates questions for the next interview.

**Foundation tracks landed**: First Crucible and Crash Course are implemented and working. The cycle has shifted to infrastructure paces (yelp migration, decomposition) that prepare the codebase for parallel director subtrack implementation.

## Implementation Progress

### Display Layer Migration — Complete

Handbook display converted from BUK combinator style to pure yelp style. Canonical idiom (in BCG §Semicolon Sequencing):
```bash
buyy_text_yawp "  A depot is the ..."; z_depot=$BUYY_CAPTURE
```
Legacy capture API deleted from `buym_yelp.sh` — zero consumers remained. BUS0 yawp count corrected to 8.

**Remaining**: ₢A6AAe converts payor handbook (HREF diastema path), then ₢A6AAf sweeps dead combinators from `buh_handbook.sh`.

### Decomposition Plan

After yelp cleanup, the monolithic handbook files decompose into one-function-per-file under `Tools/rbk/rbh0/` (₢A6AAh). Enables parallel officia on independent handbook tracks without merge conflicts. Positioned after AAe/AAf/AAg/AAA — sweep, clean, audit, and validate the monolith before extraction.

### Critical Path

```
AAe (payor yelp) → AAf (combinator sweep) → AAg (BCG review)
  → AAA (menu cleanup) → AAh (decompose)
                            ↓ parallel-capable
              AAI→AAv→AAV, AAU→AAV, AAW, AAH→AAX, AAZ, AAM
                            ↓ all wrapped
              AAF (final docs hygiene — always last)

(AAv `ifrit-symmetric-rename`: preconditioning rename sweep between AAI's
 design record and AAV's airgap implementation. Parallel-capable with AAU.)
```

## Open Questions

- **Wait-time pedagogy** — what does a learner do during a 20-minute cloud build? Blocking for Tethered Cloud Builds track.
- **Agent-learner eval mechanics** — concrete harness design
- **Shared snippet mechanism** — how do Depot Relationships and Secret Handling Primer appear in multiple handbooks without duplication?
- **Track name refinement** — current roster is rough; final names come from iteration
- **"Kindle" vocabulary overload** — overloads BUK/zipper "kindle" (module kindling). BUK usage is implementer-facing so may be tolerable; deferred to track-name refinement.
- **Building for Crucibles placement** — does this fold into the crucible track (₢A6AAC/AAD) since kludge bypasses cloud, or is it a distinct director subtrack?
- **Crash Course Unit 5 rework** — currently teaches diagnostic failure via Marshal Zero; needs reframing to teach the principle without referencing Marshal (internal release tooling)

## Provenance

Interview origin: `☉260409-1001`. This paddock is the authoritative digest — officia record identity only, not conversation content.

### Lessons

- **Mint-time namespace verification** must sweep the existing tabtarget tree (`ls tt/`), not only the paddock's own proposals — a project-prefix check against CLAUDE.md's registry is insufficient when prefixes have families with reserved subfunction letters. (Caught the `rbw-h?` → Hallmark collision; corrected to two-colophon `rbw-o`/`rbw-O*` scheme.)
- **Officium identity ≠ officium archive**. Treat `☉` references as provenance markers, not as fetchable artifacts. Capture session content explicitly at the time if it matters beyond committed state.

## References

### Code and configuration
- `Tools/rbk/rbho_onboarding.sh` — home for new handbook sequence functions
- `Tools/rbk/rbho_cli.sh` — CLI furnish (sources buh, rbcc, rbgc, rbz, rbho; kindles dependencies)
- `Tools/rbk/rbhp_payor.sh` — payor ceremonies (post-A3)
- `Tools/rbk/rbz_zipper.sh` — rbw colophon registry; new `RBZ__GROUP_ONBOARDING` added here
- `Tools/rbk/rbgc_Constants.sh` — defines `RBGC_PUBLIC_DOCS_URL`
- `Tools/buk/buh_handbook.sh` — display module for handbook tabtargets; `buh_tlt` is the vocabulary-first primitive
- `Tools/buk/buz_zipper.sh` — generic zipper mechanism (`buz_group`, `buz_enroll`, `buz_exec_lookup`)
- `.rbk/rbrr.env` — concrete RBRR example
- `.buk/burc.env` — concrete BURC example
- `Tools/rbk/rbrp_regime.sh` — RBRP validator (reveals payor identity field shape)
- `Tools/rbk/rbgv_AccessProbe.sh` — RBRA consumer (reveals per-role credential file usage)

### Specifications
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` §Configuration Regimes — regime catalog
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` §Regime Configuration — BUK regimes
- `Tools/rbk/vov_veiled/RBSRS-RegimeStation.adoc` — Station regime spec
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — Repository regime spec

### Prior work
- `₣A3` paddock — contains the role-track design this heat supersedes
- `₣A5` paddock — public-docs refresh; coordinates anchor curation with this heat
- `₣AU` paddock — MVP-3 release context from which A3 was spun

### External
- `https://scaleinv.github.io/recipebottle` — public project page, target for cross-linking

## Paces

### revision-onboarding-validation-tail (₢A6AA_) [abandoned]

**[260705-1031] abandoned**

## Character

Interactive design + triage session requiring judgment. Mount-time work is a
conversation, not a mechanical sweep: decide what survives, salvage before
dropping, leave A6 on a clean path to retire on validation.

## Why first on the rail

This pace decides the fate of A6's remaining tail, so every other remaining
pace is downstream of its conclusions. Mount it before touching any of them.

## The work

A6's mission (intent-axis handbook tracks) is largely landed; the tail has
accreted three kinds of leftover. This pace resolves the tail.

### 1. Collapse the validation cluster into one corpus-grounded pace

Four paces are validation-flavored and were all written before the corpus
existed (₢A6AAF's own docket opens with a stale-vocabulary warning):
₢A6AAu, ₢A6AAx, ₢A6AAM, ₢A6AAF. A fifth strand — an end-to-end learner walk —
came from ₢AUAA7, which lived in ₣AU; that heat is now **retired** and the pace
**dropped**. Its learner-walk intent survives as a pointer in ₣BU's release-prep
paddock and in the retired-AU dossier — fold it in from there, no restring
possible.

Salvage-then-drop — author the replacement first, then remove the originals:
- Author one new A6 pace written against the corpus as it stands, folding in
  ₢A6AAF's three concrete gates (dangling-ref grep, tt/ existence integrity,
  ₣A5 anchor coordination), ₢A6AAM's differential agent-learner eval design,
  and the end-to-end learner walk (formerly ₢AUAA7).
- Then drop ₢A6AAu, ₢A6AAx, ₢A6AAM, ₢A6AAF. (₣AU is already retired and its
  walk pace dropped — nothing left to restring or prune there.)

Itches are nonfunctional here, so a drop is permanent docket loss (committed
code is untouched regardless). Salvage the load-bearing text into the new
docket before dropping anything.

### 2. Resolve the two implementation paces by fact

- ₢A6AAW (bind track): the swim-bitmap shows rbhodb_director_bind.sh and
  rbw-Odb already exist — confirm landed state, then wrap or finish. The
  paddock is ambivalent about whether a hands-on bind track is even wanted.
- ₢A6AAU: fold its concrete -attest-{arch} durability correction into the
  validation pass.

### 3. Park Bucket 3 (orthogonal accretion) for this session

₢A6AAZ (plumb→GAR-direct + VOUCHES superdirectory) is a worked-out foundry
redesign worth preserving via rehome, not a drop. ₢A6AA-, ₢A6AAy, ₢A6AA9 are
smaller. Disposition deferred — revisit after the validation re-vision lands.

## Done

A6's tail is re-visioned: the validation paces collapsed into one pace authored
against the actual corpus (folding in the formerly-AU learner walk), the two
implementation paces resolved by fact, Bucket 3 dispositioned or explicitly
parked, and A6 pointed at a clean retire-on-validation termination.

### rbr-env-in-place-edit-pattern (₢A6AA-) [abandoned]

**[260705-1031] abandoned**

## Character

Pattern-application sweep. Pattern is articulated; per-site execution is mount-time work. Methodical, BCG-disciplined, one site at a time.

## Docket

Replace the "tell operator what to type into rbr*.env" pattern with in-place editing across RBR*.env-touching commands.

### Pattern

1. Pre-flight: assert target rbr*.env is clean (no uncommitted changes); fail early naming the file if dirty
2. Determine the new values via the existing work
3. BCG-compliant in-place edit of the rbr*.env file writing those values, then surface the change with a commit reminder

### Trigger case

`rbgp_payor_install` (invoked by `tt/rbw-gPI.PayorInstall.sh`) currently emits "Configuration required in rbrp.env: RBRP_..." instructions rather than writing the values. Four fields involved; `RBRP_BILLING_ACCOUNT_ID` is operator-supplied and may remain a placeholder or get its own prompt.

### Discovery

Grep `Tools/rbk/` for "Configuration required in" and analogous hand-edit emissions to enumerate sites at mount-time.

### Done

Every site emitting hand-edit instructions for an RBR*.env file applies the three-step pattern.

### director-yokes-vessel-to-reliquary (₢A6AA6) [complete]

**[260423-1319] complete**

## Character

Mechanical implementation once the contract is locked. Two validations and a regime field rewrite — no cross-cutting refactors, no new concept work. The design conversation concluded in a separate onboarding-study session; this pace is the code delivery. Cognitive posture: precise, contract-driven, no speculative generality. Do not re-open design questions already settled below.

## Context

`tt/rbw-Odf.OnboardingDirectorFirstBuild.sh` Step 1 currently ends with a manual-edit instruction to the learner:

    Open rbev-sentry-deb-tether/rbrv.env and set the field,
    then commit the change.

This is a teaching seam. A newcomer holding a fresh reliquary datestamp is exactly the person who will typo it, then discover the typo ~15-20 minutes later when a Conjure preflight fails. The remedy is a tabtarget that accepts the stamp and vessel as arguments, validates both, and rewrites the `RBRV_RELIQUARY` line safely.

## Design decisions locked (do not re-litigate)

- **Fact-file capture rejected.** `output-buk/current/` is cleared on every tabtarget entry; intermediate state there vanishes before the next command runs. The stamp MUST come from an explicit argument.
- **Global reliquary (RBRR_RELIQUARY) rejected.** Per-vessel `RBRV_RELIQUARY` stays per-vessel.
- **List-valid-reliquaries subcommand deferred.** If validation fails, the error message names the missing bits; a separate list verb is a future improvement, out of scope for this pace.
- **Verb is Yoke.** Y is unencumbered across Director actions (current taken: E, I, O, A, V, J, r, w). "Yoke A to B" is inherently relational, mirroring the two-argument signature. No phonetic or semantic rhyme with Enshrine.
- **Auto-commit rejected.** Learner commits afterward, matching the Kludge cadence.

## Contract

Colophon `rbw-dY`, frontispiece `DirectorYokesVesselToReliquary`, no imprint. Two positional args (not a single-folio channel; the BUK `imprint`/`param1`/empty vocabulary does not apply here — enroll as a two-param tabtarget per whatever mechanism the zipper supports, or extend the channel if it does not).

Invocation shape:

    tt/rbw-dY.DirectorYokesVesselToReliquary.sh <reliquary-stamp> <vessel>

### Argument order — load-bearing

Stamp FIRST, vessel SECOND. Rationale: keeps the door open to a future "yoke one stamp across multiple vessels" variant without breaking existing callers. Do NOT reverse the order for ergonomic reasons.

### Behavior

1. **Validate the reliquary stamp.** Confirm all four tool images (skopeo, docker, gcloud, syft) are present in GAR under the expected reliquary path for the given stamp. Reuse the path logic that `tt/rbw-dI.DirectorInscribesReliquary.sh` uses when it writes those images — the inscribe path template is the source of truth. On missing-image(s), error naming the stamp AND the specific missing image name(s).

2. **Validate the vessel.** Confirm:
   - Vessel regime directory exists at the expected location
   - `rbrv.env` renders and validates (equivalent to what `tt/rbw-rvv.ValidateVesselRegime.sh <vessel>` checks)
   - Vessel's mode is one that consumes a reliquary — `conjure` and `airgap` accept yoking; `bind` and `graft` do not (bind pins an upstream digest, graft pushes a local build — neither consumes a reliquary). Error clearly if mode is incompatible.

3. **Both validations must pass before any write.** No partial state. If the stamp is valid but the vessel is not (or vice versa), error and exit without modifying any file.

4. **Rewrite `RBRV_RELIQUARY=<stamp>` in the vessel's `rbrv.env`.** Preserve the rest of the file exactly. In-place edit; do NOT regenerate the regime file via a renderer. If the line is absent, add it in an appropriate location; if present, replace the value only.

5. **Report clearly.**
   - Before write: `Yoking <vessel> to reliquary <stamp>`
   - After write: path to the modified file and the new line content
   - Learner commits with the usual git cadence afterward

### Validation errors must be actionable

Sample shapes (not literal strings — match the surrounding operator-message voice):

- `Reliquary stamp 'r260324193327' not found in Depot — expected 4 tool images under <GAR path>; missing: skopeo, gcloud. Re-run tt/rbw-dI.DirectorInscribesReliquary.sh to mint a fresh reliquary, or verify the stamp spelling.`
- `Vessel 'rbev-bottle-plantuml' has mode=bind; Yoke only applies to conjure and airgap vessels. Bind vessels pin an upstream digest; graft vessels push a local build — neither consumes a reliquary.`
- `Vessel 'rbev-does-not-exist' not found — expected regime at <path>.`

No stack traces to the learner; operator-grade messages only.

## What NOT to do

- Do not cache or short-circuit validation (e.g., "skip if stamp looks like a datestamp"). The tabtarget's value IS the validation.
- Do not add a `--force` flag. If validation fails, the fix is to inscribe or correct the input, not to bypass.
- Do not auto-commit.
- Do not add list-valid-reliquaries as a side quest; deferred.
- Do not modify any file other than the target vessel's `rbrv.env`.
- Do not relitigate argument order, verb choice, or scope.

## Handbook update (same pace)

Edit `tt/rbw-Odf.OnboardingDirectorFirstBuild.sh` Step 1. Currently ends with the manual-edit instruction quoted in Context above. Replace with the Yoke invocation:

    tt/rbw-dY.DirectorYokesVesselToReliquary.sh <reliquary-stamp> rbev-sentry-deb-tether

plus a short line reminding the learner to commit `rbrv.env` afterward. Adjust surrounding prose as needed — the step can be a touch shorter.

Sweep the other Director subtracks (`rbw-Oda`, `rbw-Odb`, `rbw-Odg`) for the same pattern. Oda in particular does NOT appear to re-yoke (it assumes Odf ran first; the airgap vessel inherits the stamp) — verify rather than assume.

## Registry updates

- Zipper enrollment for the new tabtarget — see `Tools/rbk/rbz_zipper.sh` for the pattern; the new entry belongs in whichever group owns Director Depot/Foundry actions. Follow the existing `rbw-dI` / `rbw-dE` enrollment as the template.
- Entry in `Tools/rbk/rbk-claude-tabtarget-context.md` under the Depot section: `rbw-dY | DirectorYokesVesselToReliquary | param1+param2 | Yoke vessel's regime to a specific reliquary stamp`.
- Any colophon registry in RBS0 / RBDC / CLAUDE.md that enumerates Director actions.
- `Tools/rbk/vov_veiled/` — check for a spec file covering the Yoke action (pattern: `RBSAY-ark_yoke.adoc` or similar). Create if that pattern is the norm for Director-action specs; otherwise leave for a later spec-hygiene sweep.

## Done criteria

- Tabtarget exists; validates stamp and vessel before any write; errors diagnostically; rewrites cleanly on success.
- `rbw-Odf` Step 1 teaches Yoke instead of the manual edit.
- Other Director subtracks checked for the same pattern and updated if needed.
- `tt/rbw-tf.QualifyFast.sh` green (tabtarget discovery, colophon validation, nameplate health).
- Registry / context files updated.

## References

- `tt/rbw-Odf.OnboardingDirectorFirstBuild.sh` — seam location (Step 1)
- `tt/rbw-dI.DirectorInscribesReliquary.sh` — source of GAR reliquary path template
- `tt/rbw-rvv.ValidateVesselRegime.sh` — vessel regime validator (reusable check)
- `tt/rbw-dE.DirectorEnshrinesVessel.sh` — the analogous self-driving pattern for `RBRV_IMAGE_N_ANCHOR`; Yoke is the explicit (human-chosen) counterpart since reliquary stamps cannot be auto-picked
- `CLAUDE.md` — tabtarget naming conventions, zipper enrollment, BUK folio channel vocabulary
- Design provenance: onboarding-study session ☉260423-1024

### onboarding-menu-navigational-polish (₢A6AA7) [complete]

**[260423-1313] complete**

## Character

Navigational design in a single file. Three small additions that must feel of a piece — if they don't cohere, the menu reads like three patches. A single-sentence "start here" affordance, a time-budget parenthetical per track, and a Foundation concept-intro paragraph. None is mechanical; each wants tight word-economy and consistency with the existing menu voice. Cognitive posture: the menu is read cold by newcomers; every word earns its keep.

## Context

Fresh-reader study of `tt/rbw-o.ONBOARDING.sh` (run against the current menu) produced three concrete gaps:

1. **No explicit "start here if new" path.** The menu has four sections in an intended order (Foundation → Kludged Crucibles → Payor/Depot → Director subtracks) but nothing says so. A newcomer could reasonably try `Op` first and hit GCP billing before they've verified their local environment.

2. **No time budget.** `Ofc` discloses its 15-20 min Cloud Build wait inside the handbook; the menu doesn't surface per-track time estimates up front. A learner planning their session has to guess cadence.

3. **Foundation section lacks a concept intro.** Kludged Crucibles got a reworked 4-line opener (commit `1be9e9f9`) explaining Bottle/Sentry/Pentacle. Foundation just jumps into bullets without a "what is a regime, what is a station, what are credentials" preview. The `Occ` handbook teaches this well, but the menu doesn't scaffold into it.

All three additions live in the start-here menu module. Three parallel officia editing the same file would collide; consolidation into one pace forces coherent navigational design across the additions.

## Target file

`Tools/rbk/rbh0/rbho0_start_here.sh` — home for the start-here menu content rendered by `tt/rbw-o.ONBOARDING.sh`. Recent commits (`1be9e9f9`, `c1430451`, `0f0d3eff`, `b3459cac`) show the current authoring cadence and voice.

## Additions

### 1. "Start here if new" affordance

Add one header sentence immediately above the "Foundation" section, pointing the first-timer at the recommended sequence. Shape (exact wording is the author's judgment):

    New to Recipe Bottle? Run Occ first, then pick Kludged Crucibles
    (local, no cloud) or Director subtracks (requires Payor + Depot).

Constraints:
- One sentence, not a paragraph.
- References tracks by their short handles (`Occ`, etc.) with hyperlinks if the existing menu hyperlinks them — consistency wins.
- Does NOT replace or gate any existing section; it's a signpost, not a restriction. Advanced learners still see the full menu.

### 2. Time-budget parentheticals per track

Append a time estimate to each track's title line. Starting proposals (implementer may calibrate against actual observed wall-clock from their own runs):

| Track | Estimate |
|-------|----------|
| `Occ` | ~5 min |
| `Ocr` / `Ocd` | ~5 min each |
| `Ofc` | ~20 min |
| `Ots` | ~15 min |
| `Op` | ~30 min (incl. GCP provisioning) |
| `Og` | ~10 min |
| `Odf` | ~30 min (~15 min Cloud Build wall-clock) |
| `Oda` | ~60 min (two 15-20 min Cloud Builds) |
| `Odb` | ~10 min |
| `Odg` | ~10 min |

Format consistency matters more than exact minute accuracy. Pick one shape (e.g., `(~20 min)` trailing the track's first line) and use it everywhere. Multi-part estimates like `(~30 min, ~15 of which is Cloud Build wall-clock)` are fine where one component dominates.

### 3. Foundation concept-intro paragraph

Add a 3-5 line concept intro directly under the `Foundation` section header, parallel in shape to the existing Kludged Crucibles opener. It should introduce the three Foundation items as a coherent trio before the bullets, not item-by-item. Shape:

    Foundation sets up three things on your workstation: the repo regime
    (shared project settings that travel with the code), the station
    regime (per-developer local settings that stay on your machine), and
    credentials (the role key your Governor handed you). Local-only —
    no cloud ceremony yet.

Constraints:
- Use the same linked-term attribute references (`:rbrr:` / `:burs:` / `:rbra:` etc.) that the Kludged Crucibles opener uses for its terms, if those attributes exist in the BUK yelp tree. Check `rbho0_start_here.sh` for the current convention.
- ~3-4 sentences max. This is a concept preview, not a handbook section.
- End with a one-clause bridge into the bullets (e.g., "Local-only — no cloud ceremony yet.") to match Kludged Crucibles' narrative transition style.

## What NOT to do

- Do not restructure the menu's section ordering. Foundation → Kludged Crucibles → Payor/Depot → Director subtracks stays as-is.
- Do not rewrite existing section openers (Kludged Crucibles is the reference shape; don't touch it).
- Do not add per-track concept paragraphs beyond Foundation. Kludged Crucibles' opener is enough; other sections don't need new prose.
- Do not gate or hide advanced tracks based on the "start here" line — it's guidance, not enforcement.
- Do not add anchor-chase definitions inside the time-budget parentheticals (keep them bare minutes).

## Done criteria

- `tt/rbw-o.ONBOARDING.sh` renders cleanly with all three additions.
- Time-budget parentheticals appear on every track in consistent shape.
- Foundation concept-intro paragraph renders with correct yelp/hyperlink formatting.
- "Start here if new" sentence appears exactly once, above Foundation.
- `tt/rbw-tf.QualifyFast.sh` green (no tabtarget or colophon regressions).
- Visual spot-check: rendered menu under a reasonable terminal width (~100 cols) has no wrap artifacts on the new content.

## References

- `Tools/rbk/rbh0/rbho0_start_here.sh` — target file
- Recent commits on the menu: `1be9e9f9` (Kludged Crucibles compression — reference shape for concept intro), `c1430451` (consolidated Requires line — reference for single-line affordance), `0f0d3eff` (paren-gloss bullet rework — reference for trailing-paren style)
- Design provenance: onboarding-study session ☉260423-1024

### handbook-pattern-recap-closers (₢A6AA8) [complete]

**[260423-1313] complete**

## Character

Per-handbook authoring requiring judgment. Not mechanical — each closer is the distilled "what the reader should walk away with" for its track, and the right abstraction differs per handbook. A single author handling all four closers will produce more consistent closer shape than parallelizing across files; the convention itself is load-bearing. Cognitive posture: ask "what generative rule does this handbook teach?" and surface that rule as the retained takeaway.

## Context

`tt/rbw-Occ.OnboardingConfigureEnvironment.sh` Step 7 ("The pattern") is the best single teaching moment in the current onboarding. It shows that all regimes follow `rbw-r{letter}{r|v}` tabtargets and lists them in a table. The reader leaves with a *generative rule*, not a memorized list — they can derive any regime's tools by knowing one letter.

Fresh-reader study found that the other handbooks end without equivalent recaps. They walk the learner through a sequence and stop. A closer in Occ's shape would turn each handbook from a walkthrough into a retained mental model.

Not every handbook needs a closer. The Director subtracks (`Odf`, `Oda`, `Odb`) and the primary Crucible track (`Ofc`) each carry a generative rule worth surfacing. Short tracks (`Ocr`, `Ocd`, `Odg`) don't — their whole content is already a short recap.

## Target files

Under `Tools/rbk/rbh0/`. Locate the sequence function for each handbook (per CLAUDE.md the onboarding decomposition lives there). Per recent commits the file naming pattern is likely `rbho*_*.sh` with one function per file.

Handbooks in scope (in priority order):

- `rbw-Ofc.OnboardingFirstCrucible.sh`
- `rbw-Odf.OnboardingDirectorFirstBuild.sh`
- `rbw-Odb.OnboardingDirectorBind.sh`
- `rbw-Oda.OnboardingDirectorAirgap.sh`

## Closer shape (common across all)

New section appended after the existing final step. Consistent header voice — e.g., `Step N — The pattern` or `What you now know` (pick one phrasing and apply uniformly across all four; don't mix). Body:

- 2-4 lines of prose naming the rule
- A compact table or bullet list enacting the rule
- No probes, no tabtarget invocations to run — pure retained-knowledge content
- Close with a one-line bridge back to start-menu or related tracks

The Occ Step 7 body shape is the reference: prose setup → table → one-line closer. Match that rhythm.

## Per-handbook closer content

Each closer's content is distilled per handbook. Do NOT copy a generic template — the whole point is that each track teaches a different generative rule.

### `Ofc` — Crucible lifecycle

Four verbs that describe the full local-crucible iteration loop:

    Kludge → Charge → SSH → Quench

Prose should name these as the iteration primitives: once the learner owns the four verbs, they can repeat the loop against any Nameplate without re-reading the handbook. A 4-row table mapping verb → tabtarget → purpose is the right enactment.

### `Odf` — The six arks

Every Conjured Hallmark produces six artifacts in GAR: `pouch`, `image`, `attest`, `about`, `vouch`, `diags`. These already appear mid-flow in Step 4, which is fine — but the closer should promote them to the retained position as a reference card (a six-row table). The pattern: "every Hallmark is a structured bundle; know the six basenames and you can tour any Hallmark by hand." Consider referencing `Plumb` (full vs compact) as the pattern-driven inspector for these arks.

### `Odb` — Three ordain modes, three trust contracts

This table already appears mid-flow in `Odb` Step 2 (Conjure / Bind / Graft trust-contract comparison). It is arguably the single best architectural summary in the entire onboarding. The closer's job is to promote it to the retained position — either by moving it to the end or duplicating it there. The rule: "three modes, three trust contracts, mode-aware Vouch verdicts." Consider whether moving it breaks Step 2's flow; if so, duplicate.

### `Oda` — Airgap supply chain

The three-link chain the handbook already describes in its opening prose:

    1. Enshrine upstream base into Depot
    2. Conjure forge Tethered, then re-Enshrine
    3. Conjure final Bottle in Airgap mode

The closer names the chain as the reusable pattern: any future airgap build follows these three links. Consider surfacing the Tethered/Airgap/Kludged three-way build-info comparison (already in Step 5) as a secondary micro-card if it fits without bloat.

## Consistency constraints across all four closers

- Same header phrasing (pick one and use everywhere).
- Same rough shape (prose → table/list → one-line closer).
- Same length budget (~10-15 rendered lines each; don't balloon).
- Same voice register — matches Occ Step 7's teacher-voice, not step-voice.
- No probes, no `[*]`/`[ ]` markers (this is pure retained-knowledge content).
- Closer is the LAST thing the reader sees before the "return to start" line; if a handbook has a "Next steps" section, the closer precedes it.

## What NOT to do

- Do not author closers for short handbooks (`Ocr`, `Ocd`, `Odg`, `Op`, `Og`, `Ots`) — they don't need them. If a short handbook genuinely would benefit, flag it but do not add speculative closers.
- Do not invent patterns that aren't already taught in the handbook body. If the pattern isn't in the content, the closer would be introducing new material — out of scope for this pace.
- Do not merge closers across handbooks. Each track has its own generative rule.
- Do not restructure existing step content (exception: the `Odb` three-mode table may be moved from Step 2 to the closer if the flow holds; if moving it damages Step 2, duplicate).
- Do not add cross-track "see also" navigation inside closers beyond a single bridge line.

## Done criteria

- Four handbook closers authored and rendering correctly in their respective tabtargets (`Ofc`, `Odf`, `Odb`, `Oda`).
- Same header phrasing and shape across all four.
- `tt/rbw-o.ONBOARDING.sh` unchanged by this pace (menu is P1's territory).
- `tt/rbw-tf.QualifyFast.sh` green.
- Visual spot-check: each closer renders cleanly under ~100 cols terminal width.
- Occ Step 7 unchanged — it's the reference shape, not the target.

## References

- `tt/rbw-Occ.OnboardingConfigureEnvironment.sh` Step 7 — reference shape
- Existing mid-flow patterns worth promoting:
  - `Odb` Step 2 three-mode trust-contract table
  - `Odf` Step 4 six-ark artifact listing
  - `Oda` opening prose three-link chain
- Onboarding menu module: `Tools/rbk/rbh0/rbho0_start_here.sh` (for context on project voice — do not modify here)
- Design provenance: onboarding-study session ☉260423-1024

### symmetric-sentry-kludge (₢A6AA2) [complete]

**[260420-1501] complete**

## Character

Small infrastructure pace with a downstream pedagogy revision. The bottle has `rbw-cKB` (KludgeBottle) — nameplate-aware, builds and drives the hallmark into the nameplate in one step. The sentry has no such symmetric tool; the learner runs `rbw-fk rbev-sentry-deb-tether`, copies the hallmark tag from terminal output, opens `rbrn.env`, and pastes into `RBRN_SENTRY_HALLMARK` by hand.

The first-crucible handbook (`rbho_first_crucible()` in `rbhofc_first_crucible.sh`) currently rationalizes the asymmetry ("the sentry is shared across nameplates, so you kludge it standalone and drive the hallmark yourself"). In practice this is copy/paste friction the learner hits immediately after meeting the toolchain, and the handbook already teaches "what a hallmark is and where it lives" via the bottle step one screen earlier.

Cognitive posture: mechanical symmetry restoration plus trimming of the teaching prose that only existed to justify the missing tool. Parallel-safe with other A6 work — touches `rbob_bottle.sh`, the zipper, a new tabtarget, and the first-crucible handbook file.

## Docket

### Problem

Step 1.1 + Step 2 of `rbho_first_crucible()` force the learner to transcribe a hallmark tag between terminal output and a config file. Mechanically identical to the bottle path one step earlier, but unautomated.

### Approach

Mirror the bottle pattern:

| Bottle (today) | Sentry (target) |
|---|---|
| `rbob_kludge()` in `rbob_bottle.sh` — kludges bottle vessel, drives hallmark into `RBRN_BOTTLE_HALLMARK` | New `rbob_kludge_sentry()` — kludges sentry vessel (from `RBRN_SENTRY_VESSEL`), drives hallmark into `RBRN_SENTRY_HALLMARK` |
| `rbw-cKB.KludgeBottle.sh` (param1 moniker) | New `rbw-cKS.KludgeSentry.sh` (param1 moniker) |
| Zipper group entry for `rbw-cKB` | Zipper group entry for `rbw-cKS` |

### Commit cadence — load-bearing

`rbfd_kludge()` (at `rbfd_FoundryDirectorBuild.sh:1213-1218`) hard-dies on any unstaged or staged change. Both auto-driving kludges leave the tree dirty after driving their hallmark. Consequence: **with two auto-driving kludges, the learner must commit between them**, not bundle at the end.

**Current flow (manual sentry paste):** clean → `rbw-fk` (tree stays clean) → `rbw-cKB` (tree dirty: bottle hallmark) → manual paste sentry hallmark (tree dirty: both hallmarks, single commit point) → **one commit** → charge.

**New flow (symmetric kludges):** clean → `rbw-cKS` (tree dirty: sentry hallmark) → **commit #1** (forced — next kludge gates on clean tree) → `rbw-cKB` (tree dirty: bottle hallmark) → **commit #2** → charge.

Net change for the learner: manual-paste eliminated, one extra git commit added. The handbook must teach this commit-between-kludges cadence explicitly — it's the one new invariant the symmetry introduces.

### Implementation steps

1. Add `rbob_kludge_sentry()` in `Tools/rbk/rbob_bottle.sh` — structural twin of `rbob_kludge()`, targeting `RBRN_SENTRY_VESSEL` / `RBRN_SENTRY_HALLMARK`. Use `zrbob_drive_hallmark` (already generic on the variable name).
2. Add thin dispatcher `tt/rbw-cKS.KludgeSentry.sh` (nolog launcher pattern matching `rbw-cKB`).
3. Enroll `rbw-cKS` in `Tools/rbk/rbz_zipper.sh` under the Crucible group (next to `rbw-cKB`). Channel: param1.
4. Add linked-term constant (if one exists for `rbw-cKB` — check `rbyc_common.sh` or the `RBZ_CRUCIBLE_KLUDGE_*` family in the zipper) so the handbook can reference it via a named constant, not a raw string.
5. Update `Tools/rbk/rbh0/rbhofc_first_crucible.sh`:
   - Replace Step 1.1's `rbw-fk` invocation and the copy/paste instruction with the new nameplate-aware call.
   - Insert a **commit step between the two kludges** with concrete `git add` / `git commit` lines and a short explanation ("kludge needs a clean tree, so commit the sentry hallmark before kludging the bottle").
   - Collapse old Step 2 (manual RBRN_SENTRY_HALLMARK edit) into a terminal-step commit for the bottle hallmark, symmetric with the post-sentry commit.
   - Reframe the "two kludge commands, two different jobs" comparison (lines ~197–204) — both are now nameplate-aware; the remaining distinction (sentry shared across nameplates vs bottle per-nameplate) can be mentioned briefly or dropped.
   - Keep the existing `buh_warn "${RBYC_KLUDGE} requires a clean git tree."` — its importance *increases* under the new flow.
   - Update probe names/comments if any reference the old flow.
   - Update the "Iteration loop" section at the end (lines ~391–412) — currently describes a one-kludge cycle; with symmetric kludges it may still be one-kludge (the sentry rarely changes), but the commit cadence framing needs a once-over.
6. Regenerate `CLAUDE.consumer.md` / `rbk-claude-tabtarget-context.md` colophon table if auto-generation is wired up (per `₢A6AAF` docket, the regenerator is the source of truth).

### Pedagogy question for mount time

The original asymmetry carried a teaching moment: "here's the file, here's the field, here's how hallmarks live in nameplates." With the symmetric tool in place, decide whether to:

- **(a)** Drop the manual-paste teaching entirely — the bottle step already shows the same machinery via `zrbob_drive_hallmark` output, and a one-line `buh_line` can mention where hallmarks live without a manual exercise.
- **(b)** Keep a vestigial "open the nameplate and verify" probe step — passive inspection instead of active editing.

Default to (a) unless the review at mount time argues for (b).

### Coordination

- `₢A6AAu` (review-onboarding-director-first-kludge, currently earlier on the rail but moving behind this pace) is the natural downstream beneficiary — the review proceeds against the cleaned-up track, not the old copy/paste flow.
- No interaction with director tracks (cloud build, airgap, bind, graft) — those paths don't touch sentry kludge.

### Requires

- Nothing — the bottle path (`rbob_kludge`, `zrbob_drive_hallmark`) is the reference implementation already landed.

### Out of scope

- Any change to `rbw-fk` (LocalKludge) — it remains the standalone vessel-kludge entry point for non-nameplate contexts (and for the sentry-deb-airgap path where no single nameplate owns it).
- Ordain-side symmetric sentry (cloud build). Director cloud-build tracks already drive sentry hallmarks through their own pipeline; this pace is kludge-only.
- Changes to the airgap handbook track (`₢A6AAV`/`₢A6AA1`) — that track uses its own moriah-nameplate flow.

### Validation

End-to-end dry run against a fresh `ccyolo`:

```
# Clean tree
tt/rbw-cKS.KludgeSentry.sh ccyolo
# Expect: sentry image built, RBRN_SENTRY_HALLMARK field rewritten, tree dirty in exactly rbrn.env
git add .rbk/ccyolo/rbrn.env
git commit -m "kludge sentry hallmark into ccyolo nameplate"

tt/rbw-cKB.KludgeBottle.sh ccyolo
# Expect: bottle image built, RBRN_BOTTLE_HALLMARK field rewritten, tree dirty in exactly rbrn.env
git add .rbk/ccyolo/rbrn.env
git commit -m "kludge bottle hallmark into ccyolo nameplate"

tt/rbw-cC.Charge.ccyolo.sh
# Expect: charge succeeds (nameplate is committed)
```

Also run `tt/rbw-Ofc.OnboardingFirstCrucible.sh` end-to-end and confirm handbook output no longer instructs copy/paste and teaches the two-commit cadence clearly.

### Failure mode to verify

Run `rbw-cKS ccyolo` then immediately `rbw-cKB ccyolo` without committing — confirm `rbfd_kludge`'s dirty-tree guard fires with a clear error. The handbook should anticipate this and guide the learner to commit; the runtime guard is the safety net.

### handbook-inline-identifier-hygiene (₢A6AA3) [complete]

**[260420-1546] complete**

## Character

Hygiene pace widened to install the shared infrastructure that makes the hygiene coherent. Three kinds of change in one pace: (1) small infrastructure addition — 7 kindle constants for handbook-facing env var naming; (2) migration of the first consumer (`rbhodf_director_first_build.sh`) from its ad-hoc `ONBOARD_*` pattern to the shared `RBYC_HANDBOOK_*` pattern; (3) two small bash-side substitutions at display-text sites that don't use env vars. Mechanical throughout; no judgment calls remain after the A6AA5 conversation upstream. Parallel-safe with other A6 work.

## Docket

### Problem

`Tools/rbk/rbh0/rbhodf_director_first_build.sh` has three hardcoded `rbev-sentry-deb-tether` sites (lines 43, 94, 171) and invents its own `ONBOARD_VESSEL` / `ONBOARD_HALLMARK` env var names for learner-facing copy-paste interpolation (lines 135, 185, 214, 284, 288, 298, 322, 332, 336). Both concerns are load-bearing:

1. The vessel sigil at lines 94 and 171 is literal text that will silently rot if the vessel renames (the handbook will still render and pass probes while pointing learners at a name that no longer exists).
2. The ad-hoc `ONBOARD_*` naming divergence will compound once ₢A6AA5 (first-crucible-multi-nameplate) begins converting rbhofc and the other director subtracks — each handbook would otherwise invent its own env var names. A shared naming scheme, encoded as kindle constants, prevents this.

The ₢A6AA3 conversation established the prescription: three kindle-constant pairs (`_NAME` + `_REF`) plus a prefix constant, living in `rbyc_common.sh` as a new category alongside the existing linked-term yelp fragments.

### Approach

**Install 7 kindle constants** in `rbyc_common.sh`:

```
RBYC_HANDBOOK_ENV_PREFIX="HANDBOOK_"
RBYC_HANDBOOK_VESSEL_NAME    = "${PREFIX}VESSEL"     → "HANDBOOK_VESSEL"
RBYC_HANDBOOK_VESSEL_REF     = "\${${NAME}}"         → "${HANDBOOK_VESSEL}"
RBYC_HANDBOOK_NAMEPLATE_NAME = "${PREFIX}NAMEPLATE"
RBYC_HANDBOOK_NAMEPLATE_REF  = "\${${NAME}}"
RBYC_HANDBOOK_HALLMARK_NAME  = "${PREFIX}HALLMARK"
RBYC_HANDBOOK_HALLMARK_REF   = "\${${NAME}}"
```

`_NAME` holds the env var name string (for `export ___=...` teaching lines). `_REF` holds the interpolation-ready literal (for command-arg consumption). Flipping `HANDBOOK_`↔`ONBOARDING_` is a one-line change; all downstream values recompute. Discipline: `\${${...NAME}}` at call sites is a smell (the `_REF` constant is missing).

**Migrate `rbhodf_director_first_build.sh`** to consume the new constants:

- Line 135 (vessel export teaching): `ONBOARD_VESSEL` → `${RBYC_HANDBOOK_VESSEL_NAME}`.
- Line 214 (hallmark export teaching): `ONBOARD_HALLMARK` → `${RBYC_HANDBOOK_HALLMARK_NAME}`.
- Lines 185, 284, 288, 298, 322, 332, 336 (command-arg consumption): `' ${ONBOARD_VESSEL}'` → `" ${RBYC_HANDBOOK_VESSEL_REF}"`, same shape for hallmark. Quote change from single to double so the kindle constant expands.

**Apply bash-side `${z_vessel}` substitution** at the two display-text sites (no env var involvement):

- Line 94: `buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Vessel" "rbev-sentry-deb-tether"` → `buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Vessel" "${z_vessel}"`.
- Line 171: `buyy_cmd_yawp "rbev-sentry-deb-tether/rbrv.env"` → `buyy_cmd_yawp "${z_vessel}/rbrv.env"`.

Line 43 (the authorial declaration `local -r z_vessel="rbev-sentry-deb-tether"`) stays — it is the single source of truth for this single-vessel handbook.

### Implementation steps

1. Update `rbyc_common.sh`:
   - Expand module docstring to acknowledge two content categories (linked-term yelp fragments; handbook env var metadata).
   - Add new section inside `zrbyc_kindle()` with the 7 kindle constants.
2. Migrate `rbhodf_director_first_build.sh`:
   - Lines 135, 214 (buh_code exports): use `_NAME` constants.
   - Lines 185, 284, 288, 298, 322, 332, 336 (buh_tt arg consumption): use `_REF` constants with double-quoted interpolation.
   - Lines 94, 171 (display text sites): use bash-side `${z_vessel}`.
3. Render `tt/rbw-Odf.OnboardingDirectorFirstBuild.sh` and inspect: learner-facing output should show `export HANDBOOK_VESSEL=rbev-sentry-deb-tether` (not `ONBOARD_VESSEL=`), command args should show `${HANDBOOK_VESSEL}` / `${HANDBOOK_HALLMARK}`, display text sites render the same literal vessel name as before.
4. Sweep for residual `ONBOARD_VESSEL`/`ONBOARD_HALLMARK` in `rbh0/` — should be zero after migration.

### Validation

- Dry render of `tt/rbw-Odf.OnboardingDirectorFirstBuild.sh` shows the new env var names and renders vessel paths/linked-terms correctly.
- Grep sweep `buyy_*_yawp "rbw-|buyy_*_yawp "rbev-|buyy_*_yawp "rbgp_` across `Tools/rbk/rbh0/` returns zero hits (fix-1 plus the bash-side substitutions close everything).
- Grep sweep `ONBOARD_VESSEL\|ONBOARD_HALLMARK` across `Tools/rbk/` returns zero hits inside handbook files (may remain in comments/docstrings; the bar is zero in live yelp/code).
- `tt/rbw-tf.QualifyFast.sh` passes.

### Requires

Nothing. ₢A6AA2 established the hardcoded-colophon sweep baseline; the ₢A6AA3 conversation established the kindle-constant scheme.

### Out of scope

- Migration of `rbhofc_first_crucible.sh` or any other handbook to the new kindle constants. That is ₢A6AA5's implementation-phase work once the merge/split/middle-path decision is made.
- Horizon Roadmap entry on vessel-sigil registry. Dissolved — the question was whether `rbev-*` sigils need a registry like `RBZ_*`; the kindle-constant scheme clarifies that env var naming is the concern (solved) and vessel sigils themselves remain filesystem-defined (no registry needed).
- Renames of existing `RBYC_*` linked-term constants.
- Changes to `buyy_*_yawp` or `buh_tt`/`buh_code` APIs.

### Done so far (this pace)

- `f131f83f` landed fix 1: `rbhpr_refresh.sh:63` `rbgp_payor_install` hardcode replaced with canonical `buh_tt "${RBZ_PAYOR_INSTALL}" ...` pattern matching `rbhopw_payor_wrapper.sh:81`.

### first-crucible-multi-nameplate (₢A6AA5) [complete]

**[260423-0814] complete**

## Character

Intricate but mechanical. The interview phase landed in ☉260420-1014 — option (B′), two handbooks with a shared mechanical trunk. File names, tabtarget name, shared-helper contracts, tail content shape, and cloud-build exclusion are all locked. No open design questions.

Cognitive posture: careful attention to per-file content role. The evaluator tail is substantially richer than the explorer's — treat the depth asymmetry as intended. Avoid making the shared trunk too thick (voice collapses into one frame) or too thin (factoring fails to earn weight). Mirror the `rbhodf_director_first_build.sh` consumption pattern for `RBYC_HANDBOOK_*` kindle constants (export-teaching via `_NAME`, buh_tt arg expansion via `_REF`).

## Docket

### Landed design decision — option (B′)

A fourth option beyond the original (A)/(B)/(C) in the prior docket. **Two handbooks, shared mechanical trunk, intimate voice-specific top and tail.**

Why (B′) over the prior three:

- **Audience separation is load-bearing.** Explorer ("inhabit the sandbox, feel the walls") and evaluator ("verify the sandbox holds under attack") are distinct framings. Option (A) collapses them into one voice; option (C) lands them in a menu. Neither serves the audience split.
- **Verification is where actions genuinely diverge** — ccyolo's manual `curl`-by-hand against the allowlist vs tadmor's theurge suite run against adversarial attacks. Same goal ("prove the sandbox holds"), different *actions*, not just prose voice.
- **The mechanical trunk (kludge sentry → commit → kludge bottle → commit → charge) is voice-neutral and thick enough to factor.** Status-quo (B) pays duplication cost with no pedagogical benefit.

Result: two handbook files with independent top/tail, one shared trunk helper, one shared quench helper.

### File structure

**Shared helpers (new, siblings in `Tools/rbk/rbh0/`):**

- `rbhoct_crucible_trunk.sh` — factored kludge/commit/charge choreography
- `rbhocq_crucible_quench.sh` — factored quench step surfacing the charge-auto-quench pedagogy at its natural moment (replacing the currently-buried iteration-loop mention at `rbhofc:370-371`)

**Refactored:**

- `rbhofc_first_crucible.sh` — keeps explorer-intimate top (framing + export teaching + docker gate) and explorer-intimate tail (SSH + Claude OAuth + manual curl verification + iteration-loop wrap); delegates middle choreography to the shared helpers

**New evaluator handbook:**

- `rbhots_tadmor_security.sh` — evaluator-intimate top + shared trunk call + evaluator-intimate tail + shared quench call + wrap

**New tabtarget + zipper enrollment:**

- `tt/rbw-Ots.OnboardingTadmorSecurity.sh` — evaluator handbook entry point
- Enroll `rbw-Ots` in `RBZ__GROUP_ONBOARDING` in `rbz_zipper.sh`

### Shared trunk contract (`rbhoct_crucible_trunk`)

Caller signature — authorial bash args carry per-nameplate facts:

```
rbhoct_crucible_trunk "${z_moniker}" "${z_sentry_vessel}" "${z_bottle_vessel}" "${z_nameplate_file}"
```

Trunk body renders, in order:

1. Section header: "Build images locally" + kludge/hallmark/nameplate concept teaching (voice-neutral)
2. Kludge sentry step — `buh_tt` with `${RBYC_HANDBOOK_NAMEPLATE_REF}` for learner-shell expansion
3. Sentry-image-exists + sentry-hallmark-populated probes
4. Commit cadence teaching (sentry)
5. Kludge bottle step — `buh_tt` with `${RBYC_HANDBOOK_NAMEPLATE_REF}`
6. Bottle-image-exists + bottle-hallmark-populated probes
7. Commit cadence teaching (bottle)
8. Charge section — brief sentry/pentacle/bottle mention (architecture deep-dive belongs in evaluator tail)
9. Charge step — `buh_tt` with moniker interpolation
10. Crucible-charged probe

Probe discrimination works naturally — probes key off the `z_sentry_vessel` / `z_bottle_vessel` bash args, which differ per nameplate.

### Shared quench contract (`rbhocq_crucible_quench`)

Caller signature:

```
rbhocq_crucible_quench "${z_moniker}"
```

Quench body renders:

1. Section header: "Quench the crucible"
2. Brief teaching: stops containers, tears down enclave network
3. Quench tabtarget call — `buh_tt` with moniker interpolation
4. **Charge-auto-quench pedagogy at its natural moment:** "you don't need to quench between iterations — charge tears down prior state for you; quench when you're done." (Current location of this insight at `rbhofc:370-371` gets removed from the explorer wrap since the shared quench now carries it for both audiences.)

### Caller structure (both handbooks)

```
1. zrbho_sentinel
2. buc_doc_brief / buc_doc_shown
3. Authorial bash: local -r z_moniker=<nameplate>
                   z_sentry_vessel=...
                   z_bottle_vessel=...
                   z_nameplate_file=...
4. Audience-framing header prose (intimate voice)
5. Export teaching line (using ${RBYC_HANDBOOK_NAMEPLATE_NAME})
6. Docker gate probe + early-return on failure
7. rbhoct_crucible_trunk "${z_moniker}" "${z_sentry_vessel}" "${z_bottle_vessel}" "${z_nameplate_file}"
8. Verification ritual (intimate — curl for ccyolo, architecture + theurge for tadmor)
9. rbhocq_crucible_quench "${z_moniker}"
10. Wrap prose (iteration loop for ccyolo, next-tests guidance for tadmor)
11. Return-to-start buh_tt
```

### Evaluator tail content shape (rich)

The evaluator tail is substantially richer than the explorer's. Do not under-scope. Three parts:

**Architecture deep-dive** — not the brief charge-time mention the trunk carries; a real tour:

- Sentry layer: iptables kernel rules + dnsmasq DNS layer — why both layers, what each enforces, what happens when either fails
- Pentacle layer: network namespace ownership; why a dedicated container rather than bottle-side namespace manipulation
- Bottle layer: unprivileged workload context; lack of network primitives means no circumvention path
- Defense-in-depth across the three containers

**Ifrit/Theurge methodology intro:**

- Ifrit is the attack binary delivered into the bottle
- Claude Code sessions with full visibility authored the attack suite against known primitives
- Theurge coordinates inside (bottle) and outside (host) observation to verify attacks fail
- Test results show both the attack tried AND the sandbox response

**Fixture invocation + result inspection:**

- Full fixture (teach as the default pass): `tt/rbtd-r.Run.tadmor-security.sh` — 34 cases, exercises the complete containment attack surface against a kludged tadmor
- Single case (teach for iterative debugging / deep-dive on one attack): `tt/rbtd-s.SingleCase.tadmor-security.sh [case-name]`
- Result reading: teach the learner how to interpret pass/fail output, what a green case means, what a red case would surface

**Strictly no cloud-build or global-suite invocations.** Do not reference `rbtd-s.TestSuite.crucible.sh` (pulls in moriah, which is cloud-built), `rbtd-s.TestSuite.complete.sh`, or the service suite. If `rbtd-r.Run.tadmor-security.sh` / `rbtd-s.SingleCase.tadmor-security.sh` carry framing that pulls in global theurge-apparatus prose the handbook shouldn't teach, create thin custom wrappers under a new colophon family rather than redirecting learners to the raw rbtd tabtargets.

### Explorer tail content shape (preserve)

Preserve `rbhofc`'s existing tail content, lightly edited:

- SSH into bottle via `tt/rbw-cS.SshTo.ccyolo.sh`
- Claude Code OAuth copy-paste teaching (preserve the v2.1.89 paste-regression warning at current lines 275-279)
- Manual curl verification ritual (api.anthropic.com → 404, claude.ai → 403, www.internic.net → 200, google.com → timeout, registry.npmjs.org → timeout)
- `rbw-cr.Rack` diagnostic fallback guidance
- Iteration-loop summary — **minus** the charge-auto-quench insight (moved into shared quench where it serves both audiences)

### Validation

- `rbhofc` dry-render matches current output closely (small acceptable deltas where shared quench surfaces the auto-quench insight earlier than current iteration-loop summary)
- `rbhots` dry-render produces a coherent evaluator tour: architecture + methodology + fixture invocation + result inspection
- Probes discriminate correctly by nameplate — no false greens/reds (vessel names are the discriminator; already in authorial bash args)
- Zero hardcoded nameplate sigils outside the three authorial declarations per handbook
- Grep sweep for `HANDBOOK_NAMEPLATE` raw literal in yelp args across `Tools/rbk/` returns zero (all consumption via `${RBYC_HANDBOOK_NAMEPLATE_REF}`)
- Zipper entry for `rbw-Ots` present in `RBZ__GROUP_ONBOARDING`; launcher wired; tabtarget file exists and dispatches correctly
- `rbhoct` and `rbhocq` sourced/kindled correctly from both callers
- Fast qualify passes
- No theurge invocations in handbook content call cloud builds or global test suites (verified by grep for `TestSuite.crucible`, `TestSuite.complete`, `TestSuite.service`)

### Requires

₢A6AA3 (landed: `f131f83f`, `33f8d192`, `0bdd62e9`) — installed `RBYC_HANDBOOK_*` kindle constants in `rbyc_common.sh` and proved consumption pattern across 9 sites in `rbhodf_director_first_build.sh`. This pace's consumption follows the same shape: `${RBYC_HANDBOOK_NAMEPLATE_NAME}` in export-teaching lines, `${RBYC_HANDBOOK_NAMEPLATE_REF}` in buh_tt arg positions.

### Out of scope

- **Paddock Rough Track Roster update** (track 3 ↔ track 14 shared-trunk annotation, track 14 promotion from placeholder to implemented) — deferred to a later pace per user direction.
- Changes to the kindle constants themselves (shape is settled).
- Re-deciding infrastructure channel (env var + authorial bash args is the established pattern).
- Any other handbook beyond `rbhofc` and the new `rbhots`.
- New theurge test cases or test suite changes.
- Nameplate regime changes or new nameplates.
- ccyolo/tadmor vessel Dockerfile changes.
- Cloud-build or global-suite theurge invocations (explicitly prohibited in handbook content).

### Interview origin

Design conversation `☉260420-1014` landed the (B′) decision and all structural/naming choices: file names (`rbhoct`, `rbhocq`, `rbhots`), tabtarget name (`rbw-Ots`), shared-helper contracts, tail-depth asymmetry, cloud-build exclusion. Implementation agent should proceed without re-opening these decisions.

### handbook-trash-comment-sweep (₢A6AA4) [complete]

**[260423-0840] complete**

## Character

Small hygiene pace. Surfaced during ₢A6AA2 when step-number section-divider comments (`# ===== / # Step N: / # =====`) forced hand-renumbering after the step ordering shifted — pure maintenance overhead for zero informational gain. The rendered output (`buh_section`, `buh_step1`, `buh_step2`) IS the self-documenting structure; bash comments that parrot it violate BCG and slow down any future edit that touches step numbering or section ordering.

Cognitive posture: mechanical deletion sweep with a clear taxonomy. Parallel-safe with other A6 work — touches only the rbh0/ handbook family.

## Docket

### Problem

The rbh0/ handbook family carries three classes of bash comments that add no information beyond what the rendered output already says. ₢A6AA2 stripped four instances of class (1) in `rbhofc_first_crucible.sh`; the rest of the family still carries all three classes.

### Three classes (taxonomy)

1. **Section-divider blocks** — `# --- Header ---`, `# --- Probes ---`, `# --- Return to start ---`, `# === / # Step N: / # ===`. The structure they mark is already rendered by `buh_section`, `buh_step1`, `buh_step2`. Pure duplication of rendered output.

2. **Inline "what-pointing" comments** — `# Docker runtime`, `# Nameplate exists`, `# Hallmarks populated in nameplate`, `# Kludge-tagged images exist locally (k-prefixed hallmarks)`, `# Crucible charged`, `# Bottle kludge image probe`, `# Hallmark probes`. Each points at an immediately-following block whose identifiers already say the same thing.

3. **Stale numeric references** — `# Step 3: Charge the crucible` that becomes wrong when an earlier step moves or is removed. Same class as (1) but specifically numerically load-bearing against change — they create maintenance work that should not exist.

### Approach

Sweep `Tools/rbk/rbh0/*.sh` for all three classes and delete. Preserve everything else:

- Copyright/license headers — legally required.
- File-top module docstring (one-line "what is this file") — keep case-by-case only if it adds non-obvious context beyond the filename.
- `# [INFRA-NEEDED]` / `# [ASSUMPTION]` / similar load-bearing markers that track future work or document non-obvious constraints.
- Comments explaining non-obvious **WHY** (hidden constraints, subtle invariants, framework quirks, workaround rationales).

**Discrimination criterion:** does the comment carry non-obvious WHY? If no, strip.

### Implementation steps

1. Inventory: for each `Tools/rbk/rbh0/*.sh`, survey `^\s*#` lines and classify each as (delete / keep / judgment) per the taxonomy.
2. For judgment cases, consult BCG Constant Discipline; err toward delete when the identifier already speaks.
3. Strip. File-by-file, small edits. Run `tt/rbw-tf.QualifyFast.sh` after each file.
4. For each affected handbook, render its tabtarget (`rbw-Occ`, `rbw-Ocr`, `rbw-Ocd`, `rbw-Ofc`, `rbw-Odf`, `rbw-Op`, `rbw-Og`, `rbw-o`, `rbw-h0`) and confirm rendered stdout matches pre-sweep output byte-for-byte.
5. After sweep, final grep across remaining files for new trash that may have crept in.

### Validation

- `tt/rbw-tf.QualifyFast.sh` passes.
- Every affected handbook renders byte-identical stdout vs pre-sweep baseline.
- `git diff` shows pure deletions in bash-comment lines — no code changes, no prose shifts.
- Line count drops measurably. Rough estimate: ~10–40 lines per file × ~20 files = 200–800 lines removed.

### Requires

Nothing. ₢A6AA2 established the pattern (strip trash when touched); this pace generalizes it to the full handbook family.

### Out of scope

- Files outside `Tools/rbk/rbh0/`. Broader BCG-compliance sweep across the rest of `Tools/rbk/` is a later pace or horizon item.
- Any code change — this is comments-only.
- Adding NEW comments.
- Identifier renames (if a comment was compensating for a weak name, a strong rename is the right fix — capture as an itch if encountered, don't inline the rename here).

### Lesson captured

Bash comments that parrot the rendered output are a tax, not an aid. `buh_step1`, `buh_section`, `buh_step2` ARE the structure; `# Step 2: Charge` is cost with no benefit. The principle: the printout is the comment.

### ccyolo-workspace-bind-mount (₢A6AA0) [complete]

**[260417-0705] complete**

## Character

Mechanical infrastructure fix with one system-wide enhancement (UID injection). The vessel restructure and compose change are straightforward file moves. The UID/GID alignment touches `rbob_bottle.sh` and `rbob_compose.yml` — shared infrastructure, so verify no regression on tadmor/other nameplates. Onboarding prose update is a light edit. Cognitive posture: careful plumbing, test from both sides.

## Docket

Fix ccyolo workspace architecture: replace the Docker named volume (opaque to the host) with a bind mount to a host-visible directory, and add system-wide UID/GID alignment so bind mounts work without per-developer configuration.

### 1. Vessel directory restructure

Split `rbev-vessels/rbev-bottle-ccyolo/` into two subdirectories:

- `build-context/` — Dockerfile, entrypoint.sh (build-time artifacts)
- `workspace/` — count_words.sh, sample.txt (runtime content, bind-mounted)

Move files accordingly. `rbrv.env` stays at vessel root.

Update `rbrv.env`:
- `RBRV_CONJURE_DOCKERFILE=${RBRR_VESSEL_DIR}/${RBRV_SIGIL}/build-context/Dockerfile`
- `RBRV_CONJURE_BLDCONTEXT=${RBRR_VESSEL_DIR}/${RBRV_SIGIL}/build-context`

### 2. Compose bind mount

In `.rbk/ccyolo/compose.yml`:
- Replace `ccyolo_workspace:/home/${RBRV_USER}/workspace:rw` with a bind mount pointing to the vessel's `workspace/` directory
- Delete the top-level `volumes:` declaration for `ccyolo_workspace`

### 3. UID/GID alignment — system-wide

**`rbob_bottle.sh`**: Export `RBOB_HOST_UID=$(id -u)` and `RBOB_HOST_GID=$(id -g)` before compose up. The person running charge IS the host identity — no configuration needed.

**`rbob_compose.yml`**: Add to the bottle service's `environment:` section:
- `RBOB_HOST_UID`
- `RBOB_HOST_GID`
- `RBRV_USER`

Every bottle gets these automatically. Vessels that don't use bind mounts ignore them.

**ccyolo entrypoint**: Add UID/GID alignment before sshd start:
```
if [ -n "${RBOB_HOST_UID}" ]; then
  usermod -o -u "${RBOB_HOST_UID}" "${RBRV_USER}"
  groupmod -o -g "${RBOB_HOST_GID}" "${RBRV_USER}"
fi
```

### 4. Entrypoint cleanup

Remove the lazy-copy logic (lines 7-13 of current entrypoint.sh) — workspace is bind-mounted, files are already present via the host directory. Keep `chown` and sshd startup.

### 5. Dockerfile cleanup

Remove `COPY sample-project/ /opt/ccyolo-sample/` — the sample project is no longer baked into the image. It lives in `workspace/` and arrives via bind mount at runtime.

### 6. Onboarding content update

Adjust first-kludge handbook prose (`rbhofc_first_crucible.sh` or equivalent) to note that workspace edits inside the bottle are visible from the host filesystem — the repo working tree gets dirty when the learner works, and that's the point.

### 7. Verification

- Kludge ccyolo bottle, charge crucible
- SSH in, confirm `count_words.sh` and `sample.txt` present in `/home/claude/workspace/`
- Edit a file from inside the bottle, confirm change visible on host
- Edit a file from host, confirm change visible inside bottle
- Confirm `id claude` inside container matches host UID
- Charge tadmor (no bind mounts) — confirm no regression from `rbob_compose.yml` changes
- Run `tt/rbtd-s.TestSuite.fast.sh` — confirm no regime validation breakage

### Out of scope

- Theurge fixture for ccyolo
- Other vessel bind mount conversions (this pace establishes the pattern; future vessels adopt it)
- New RBRN/RBRV regime variables (UID injection is runtime, not configuration)

### handbook-render-fixture (₢A6AAt) [complete]

**[260416-0918] complete**

## Character
Infrastructure build. Pure addition — new Rust fixture in Theurge plus a tabtarget. No production bash touched. Lands durable regression coverage for every handbook display before any repair work begins. Cognitive posture: mechanical execution against the design crystallized in the predecessor design conversation; pattern is well-established (mirror existing fast-suite fixtures).

## Docket
Add a `handbook-render` fixture to Theurge that exercises every handbook display tabtarget and reports per-case pass/fail. Wire into the fast suite. Run the baseline; communicate per-handbook results to the user; those results inform the reslated `₢A6AAn`'s ceiling.

### Implementation
- **`Tools/rbk/rbtd/src/rbtdrm_manifest.rs`** — add `pub const RBTDRM_COLOPHON_*` for each of the 16 handbook colophons (one symbolic ref per `RBZ_*` zipper constant); add `pub const RBTDRM_FIXTURE_HANDBOOK_RENDER: &str = "handbook-render"`; extend `rbtdrm_required_colophons()` arm so manifest verification covers the new fixture.
- **`Tools/rbk/rbtd/src/rbtdrf_handbook.rs`** (new file) — one case per colophon. Each case shells out to its tabtarget (via the existing `rbtdrf_run_bash` pattern or equivalent) and returns `Verdict::Pass` iff exit 0. Section assembled and exposed (e.g. `RBTDRF_SECTIONS_HANDBOOK_RENDER`) parallel to `RBTDRF_SECTIONS_REGIME_SMOKE`.
- **`Tools/rbk/rbtd/src/rbtdrc_crucible.rs`** dispatch arm — add `RBTDRM_FIXTURE_HANDBOOK_RENDER => crate::rbtdrf_handbook::RBTDRF_SECTIONS_HANDBOOK_RENDER` to the section selector match.
- **`Tools/rbk/rbtd/src/lib.rs`** — register the new module (`pub mod rbtdrf_handbook;`).
- **`tt/rbtd-r.Run.handbook-render.sh`** (new tabtarget) — byte-for-byte copy of any existing `rbtd-r.Run.*.sh`; imprint `handbook-render` comes from the filename.
- **`Tools/rbk/rbtd/rbte_engine.sh`** — add `"handbook-render"` to `ZRBTE_SUITE_FAST` so the fast suite includes it. Standalone tabtarget invocation continues to work independently.

### Covered colophons (sync discipline — when adding handbooks to rbz_zipper.sh, add a const here)
- Onboarding (8): `RBZ_ONBOARD_START_HERE`, `RBZ_ONBOARD_CRASH_COURSE`, `RBZ_ONBOARD_CRED_RETRIEVER`, `RBZ_ONBOARD_CRED_DIRECTOR`, `RBZ_ONBOARD_FIRST_CRUCIBLE`, `RBZ_ONBOARD_DIR_FIRST_BUILD`, `RBZ_ONBOARD_PAYOR_HB`, `RBZ_ONBOARD_GOVERNOR_HB`
- Windows (5): `RBZ_HANDBOOK_TOP`, `RBZ_HANDBOOK_WINDOWS`, `RBZ_HW_DOCKER_DESKTOP`, `RBZ_HW_DOCKER_WSL_NATIVE`, `RBZ_HW_DOCKER_CONTEXT`
- Payor (3): `RBZ_PAYOR_ESTABLISH`, `RBZ_PAYOR_REFRESH`, `RBZ_QUOTA_BUILD`

### Sync mechanism (free, already-existing pattern)
Bash `rbz_zipper.sh:179` exports `ZRBZ_COLOPHON_MANIFEST` (space-separated colophon string). Theurge's `rbtdrm_verify` checks each `RBTDRM_COLOPHON_*` const value appears in that manifest at startup. Renaming a colophon in `rbz_zipper.sh` without updating the corresponding Rust const causes loud failure at theurge startup — no silent drift.

### Baseline run + report
After fixture builds and wires:
- Run `tt/rbtd-r.Run.handbook-render.sh` standalone
- Capture per-case pass/fail
- Report results to the user; those results populate the "Baseline ceiling" section of reslated `₢A6AAn`'s docket

### Wrap criterion
- Fixture compiles
- Theurge's `rbtdrm_verify` passes for handbook-render fixture
- Standalone tabtarget runs and reports per-case results
- Fast suite continues to run (handbook-render included alongside enrollment-validation, regime-validation, regime-smoke)
- Non-zero exit due to known JJZ crash is **expected and not a wrap blocker** — the fixture doing its job IS the wrap criterion, not all cases passing
- Per-handbook results communicated to user

### repair-bcg-windows-crash (₢A6AAn) [complete]

**[260416-0933] complete**

## Character
Evidence-driven repair pace. Scope determined by the handbook-render fixture's baseline output. Known issues from the original investigation are the floor; baseline findings confirm no new items.

## Docket
Fix every handbook display that the handbook-render fixture reports as failing.

### Known floor (carried from original AAn investigation)
- **JJZ unbound var crash** — `Tools/rbk/rbh0/rbhw0_top.sh:52-53` references `${JJZ_FUNDUS_PHASE1}` and `JJZ_FUNDUS_PHASE2` but the Windows CLI (`rbhw0_cli.sh`) doesn't source `jjz_zipper.sh`. Crashes under `set -euo pipefail`. Fix: source `jjz_zipper.sh` in `rbhw0_cli.sh` furnish (the JJZ references are intentional cross-kit teaching content for the "Phase 3: User Provisioning (JJK — fundus accounts)" section, not bugs).
- **Hardcoded line 53** — `"tt/jjw-tfP2.ProvisionPhase2.{host}.sh"` → replace with `buh_tt` using `${JJZ_FUNDUS_PHASE2}` (depends on the jjz source above).
- **Triple WSL-distro capture** — `rbhw0_top.sh` lines 34, 46, 58 each call `buyy_cmd_yawp "${ZRBHW_WSL_DISTRO}"`. Collapse to one capture at first use, reuse the variable thrice.
- **Furnish hygiene** — `rbhw0_cli.sh` sources/kindles `buv_validation.sh`, `rbcc_Constants.sh`, `rbgc_Constants.sh` — confirmed unreferenced by any `rbhw*` file. Remove sources + kindle calls.

### Baseline ceiling (from ₢A6AAt run on 260416)
Baseline run of `handbook-render` produced 14/15 passing. Only `rbw-hw` (`RBZ_HANDBOOK_WINDOWS` → `rbhw0_top.sh`) failed, and the failure is exactly the JJZ unbound var crash already captured in the floor. No additional handbook failures surfaced. **Ceiling = Floor** — no scope expansion required.

Scope note: `rbw-HWdw` (`RBZ_HW_DOCKER_WSL_NATIVE`) is **not** covered by the baseline because it uses the `param1` channel and the fixture's uniform invoke-with-no-args shape can't supply the required WSL target. That gap is owned by `₢A-AAS` (`handbook-render-param1-coverage`) in `₣A-` and is intentionally out of scope for this repair pace.

### Verification
- `tt/rbtd-r.Run.handbook-render.sh` exits 0 — every covered handbook renders cleanly.
- The fast test suite (`tt/rbtd-s.TestSuite.fast.sh`) passes with handbook-render included.

### repair-bcg-dead-function-sweep (₢A6AAo) [complete]

**[260416-0942] complete**

## Character
Pure deletion — grep-confirm zero callers, then remove. No judgment needed.

## Docket
Delete 6 unreachable legacy role-track functions from `Tools/rbk/rbh0/rbhob_base.sh`:

- `zrbho_probe_role_credentials` (~line 80)
- `zrbho_probe_retriever_units` (~line 104)
- `zrbho_probe_director_units` (~line 154)
- `zrbho_probe_governor_units` (~line 232)
- `zrbho_probe_payor_units` (~line 274)
- `zrbho_triage_role` (~line 313)

Also confirm `zrbho_credential_install` has zero callers and delete if so.

These are remnants of the pre-decomposition role-track approach. The audit confirmed zero callers across the entire codebase.

### Verification
- `grep -r` each function name across all `.sh` files — only the definition site should appear
- Source guard and kindle/sentinel still intact after deletion

### repair-bcg-stderr-to-tempfile (₢A6AAp) [complete]

**[260416-0942] complete**

## Character
Moderate mechanical — replicate an existing correct pattern to 8 call sites. Requires temp file naming decisions.

## Docket
Convert 8 `docker ... 2>/dev/null | grep -q` probe patterns in onboarding to BCG-compliant temp-file stderr capture.

### Sites
- `rbhob_base.sh:124` — `docker images ... 2>/dev/null | grep -q` (inside `zrbho_probe_retriever_units`)
- `rbhob_base.sh:130` — `docker ps ... 2>/dev/null | grep -q`
- `rbhob_base.sh:135` — `docker images ... 2>/dev/null | grep -q`
- `rbhob_base.sh:259` — `docker images ... 2>/dev/null | grep -q` (inside `zrbho_probe_governor_units`)
- `rbhofc_first_crucible.sh:83` — `docker images ... 2>/dev/null`
- `rbhofc_first_crucible.sh:87` — `docker images ... 2>/dev/null`
- `rbhofc_first_crucible.sh:99` — `docker ps ... 2>/dev/null`
- `rbhodf_director_first_build.sh:73` — `docker images ... 2>/dev/null`

### Reference pattern
`zrbho_probe_director_units` (rbhob_base.sh ~line 186) already demonstrates the correct BCG pattern: temp file + stderr redirect + explicit error handling. Replicate this approach.

### Temp file naming
Use kindle `_PREFIX` constants with integer discriminators per BCG stderr capture convention. May need to add kindle constants to `zrbho_kindle` in `rbhob_base.sh`.

### Verification
- No `2>/dev/null` remains on any docker command in `rbh0/`
- Probes still function correctly (return 0/1 based on docker state)

### repair-bcg-mechanical-sweep (₢A6AAq) [complete]

**[260416-0954] complete**

## Character
Wide but shallow — every fix is a 1-line edit with zero design judgment. Mechanical compliance pass.

## Docket
Fix all remaining small BCG violations across all three handbook groups.

### Multi-var declarations → one-per-line (13 sites)
- `rbhob_base.sh:117,252,314`
- `rbhocc_crash_course.sh:40,46`
- `rbhofc_first_crucible.sh:71,72,81`
- `rbhodf_director_first_build.sh:46,67`
- `rbhpe_establish.sh:32`
- `rbhpr_refresh.sh:32`
- `rbhpq_quota_build.sh:32`

### Missing `local -r` on non-mutated locals (3 sites)
- `rbho0_cli.sh:39` — `z_rbk_kit_dir`
- `rbhp0_cli.sh:38` — `z_rbk_kit_dir`
- `rbhw0_cli.sh:36` — `z_rbk_kit_dir`

### `$()` on external command (1 site)
- `rbhpb_base.sh:34` — `case "$(uname -s)" in` — replace with temp file pattern

### Missing `|| buc_die` on source commands (18 sites)
- `rbhp0_cli.sh:40-59` — add `|| buc_die "Failed to source ..."` to each

### Missing sentinel calls in onboarding leaf files (8 sites)
- `rbho0_start_here.sh`, `rbhocc_crash_course.sh`, `rbhocr_credential_retriever.sh`, `rbhocd_credential_director.sh`, `rbhofc_first_crucible.sh`, `rbhodf_director_first_build.sh`, `rbhopw_payor_wrapper.sh`, `rbhogw_governor_wrapper.sh`
- Add `zrbho_sentinel` as first line of each public function

### Redundant enforce check (1 site)
- `rbhpb_base.sh:51-52` — remove the `test ${#...} -gt 0` line (non-load-bearing, `test -n` already covers it)

### Verification
- `grep -rn 'local.*=.*=.*' Tools/rbk/rbh0/` finds no multi-var declarations
- All public functions start with sentinel call
- All source commands in payor CLI have error guards

### repair-bcg-payor-yawp-capture (₢A6AAr) [complete]

**[260416-0940] complete**

## Character
Largest change count, most mechanical. Every edit follows the same transformation: mutable reassignment → `local -r` with unique name.

## Docket
Convert 104 mutable-reuse yawp captures in the three payor ceremony files to BCG-compliant `local -r` with unique names per capture.

### Files
- `rbhpe_establish.sh` — 70 yawp calls
- `rbhpr_refresh.sh` — 6 yawp calls
- `rbhpq_quota_build.sh` — 28 yawp calls

### Transformation pattern
```bash
## Before (mutable reuse — BCG violation)
local z_ui z_cmd z_href        # multi-var declaration at top
...
buyy_ui_yawp "some text"; z_ui="${z_buym_yelp}"
buyy_cmd_yawp "some cmd"; z_cmd="${z_buym_yelp}"

## After (BCG compliant — local -r per capture)
buyy_ui_yawp "some text"; local -r z_intro="${z_buym_yelp}"
buyy_cmd_yawp "some cmd"; local -r z_validate_cmd="${z_buym_yelp}"
```

Each capture needs a unique descriptive name since `local -r` prevents reassignment. Name each capture to reflect what it holds (e.g., `z_depot_intro`, `z_validate_cmd`, `z_consent_href`).

### Also remove
The mutable `local z_ui z_cmd z_href` (etc.) declarations at the top of each function — they become unnecessary once every yawp has its own `local -r`.

### Verification
- No mutable yawp captures remain (`grep` for yawp calls without `local -r`)
- No multi-var declarations remain at function tops
- Functions still render identical output (the variable names changed but the values didn't)

### onboarding-handbook-yelp-conversion (₢A6AAd) [complete]

**[260415-1259] complete**

## Character

Final cleanup sweep. All handbook functions in rbho_onboarding.sh are fully converted to yelp API. The pass/warn/fail yelp infrastructure is in place. What remains is searching for legacy API consumers outside rbho_onboarding.sh, removing dead code from buym_yelp.sh, and final test verification.

## Docket

### Done
- BCG yawp per-function comment template (arrow pattern)
- All 5 buyy_* headerdocs carry capture reminder
- buyy_tt_yawp extended with $3 args parameter
- buh_tt wrapper added to buh_handbook.sh
- RBYC_RBRO and RBYC_RECIPE_BOTTLE added
- rbho_start_here fully converted
- rbho_crash_course fully converted
- zrbho_credential_install, rbho_credential_retriever, rbho_credential_director converted
- rbho_first_crucible fully converted
- rbho_payor_handbook fully converted (buh_tltltlt bug fixed)
- rbho_governor_handbook fully converted
- buyy_pass_yawp (green), buyy_warn_yawp (orange), buyy_fail_yawp (red) added
- BUYC_GREEN and BUYC_ORANGE palette entries added
- RBYC_PROBE_YES and RBYC_PROBE_NO kindled constants added
- zrbho_po_status updated to use kindled probe constants
- Zero old combinators remain in rbho_onboarding.sh

### Remaining
1. Search for legacy API consumers outside rbho_onboarding.sh (buy_*_capture, zbuy_*, ZBUY_*) across Tools/
2. Migrate or remove any remaining consumers found
3. Remove legacy capture functions (buy_cmd_capture, buy_link_capture, buy_ui_capture, buy_tt_capture), compatibility constants (ZBUY_*), and aliases (zbuy_kindle, zbuy_sentinel, buy_configure_*) from buym_yelp.sh
4. Run BUK self-test (tt/buw-st.BukSelfTest.sh) to confirm nothing breaks

### payor-handbook-yelp-conversion (₢A6AAe) [complete]

**[260415-1332] complete**

## Character

Focused conversion of a single file with a distinct yelp vocabulary need. The payor handbook exercises `buyy_href_yawp` heavily (15 Google Console URLs) — the HREF diastema path that the onboarding conversion doesn't touch. Good validation of the HREF pipeline end-to-end.

## Docket

Convert `Tools/rbk/rbhp_payor.sh` from combinator style (`buh_tu`/`buh_tc`/`buh_tut`/`buh_link`) to pure yelp style. The 15 `buh_link` calls become `buyy_href_yawp` captures — the first real exercise of the HREF diastema markers in production code. Vocabulary links use RBYC_* constants. Verify via handbook tabtarget execution. Consider adding HREF-specific test cases to the buym-yelp fixture if the conversion reveals edge cases.

### combinator-sweep-and-deletion (₢A6AAf) [complete]

**[260415-1352] complete**

## Character

Cleanup sweep — mechanical elimination of legacy combinators after all handbook functions are converted to yelp style. Depends on ₢A6AAd (onboarding conversion) and ₢A6AAe (payor conversion) completing first.

## Docket

Sweep all remaining `buh_t` → `buh_line`, `buh_e` → `buh_line ""`, `buh_c` → `buh_code`, `buh_E` → `buh_error`, `buh_W` → `buh_warn` calls across converted functions. Then delete all dead combinators from `Tools/buk/buh_handbook.sh`: the single-element (`buh_t`, `buh_c`, `buh_u`, `buh_E`, `buh_W`), multi-element (`buh_ct`, `buh_tc`, `buh_tct`, etc.), link (`buh_tlt`, `buh_tltlt`, etc.), and tabtarget (`buh_tT`, `buh_tTc`, `buh_tI`, etc.) families. Also delete `zbuh_show`, `zbuh_link_fragment`, `zbuh_tabtarget_fragment`, `zbuh_imprint_fragment` and their associated `ZBUH_LINK_FRAG`/`ZBUH_TT_FRAG` state. Retain `buh_section`, `buh_step1`, `buh_step2`, `buh_step_style`, `buh_e`, `buh_line`, `buh_code`, `buh_warn`, `buh_error`, `buh_link` (if still needed), `buh_critical`, `buh_prompt`, `buh_prompt_required`, `buh_index_buk`. Run BUK self-test suite — update or remove the `buh-link` fixture which tests legacy combinators.

### decompose-rbh-into-rbh0-directory (₢A6AAh) [complete]

**[260415-1426] complete**

## Character
Structural decomposition — mechanical extraction with minting discipline. Parallelized across three agents (one per monolith), each with exclusive file ownership. Opus orchestrates: creates directory, dispatches agents, reviews results as they land, then handles shared-file wiring and cleanup.

## Docket
Decompose the three monolithic handbook files into one-function-per-file under a new `Tools/rbk/rbh0/` directory.

### Source files to decompose (line counts approximate — read the actual files)
- `Tools/rbk/rbho_onboarding.sh` (~1700 lines, 8 public functions + probes) — **heaviest**
- `Tools/rbk/rbhp_payor.sh` (~320 lines, 3 public functions + kindle/sentinel) — **medium**
- `Tools/rbk/rbhw_windows.sh` (~200 lines, 5 public functions + kindle/sentinel) — **lightest**

### Phase 1 — Opus setup (serial)

1. Create `Tools/rbk/rbh0/` directory
2. Read all three monoliths and their CLI files to understand sourcing chains and confirm no cross-monolith private function calls
3. Dispatch three parallel agents (Phase 2)

### Phase 2 — Parallel extraction (three agents)

Each agent owns an exclusive set of files — no overlaps, no worktrees needed.

**Agent 1 — Onboarding** (may READ `Tools/rbk/rbho_onboarding.sh` and `Tools/rbk/rbho_cli.sh`, may only WRITE files below):
- `Tools/rbk/rbh0/rbho_cli.sh` — workbench wiring (moved from `Tools/rbk/rbho_cli.sh`, sources all rbho* below)
- `Tools/rbk/rbh0/rbhob_base.sh` — shared probes, kindle/sentinel, constants
- `Tools/rbk/rbh0/rbho0_start_here.sh` — hub menu (uses probes) — the 0 orchestrator
- `Tools/rbk/rbh0/rbhocc_crash_course.sh`
- `Tools/rbk/rbh0/rbhocr_credential_retriever.sh`
- `Tools/rbk/rbh0/rbhocd_credential_director.sh`
- `Tools/rbk/rbh0/rbhofc_first_crucible.sh`
- `Tools/rbk/rbh0/rbhodf_director_first_build.sh`
- `Tools/rbk/rbh0/rbhopw_payor_wrapper.sh`
- `Tools/rbk/rbh0/rbhogw_governor_wrapper.sh`

**Agent 2 — Payor** (may READ `Tools/rbk/rbhp_payor.sh` and `Tools/rbk/rbhp_cli.sh`, may only WRITE files below):
- `Tools/rbk/rbh0/rbhp_cli.sh` — workbench wiring (moved from `Tools/rbk/rbhp_cli.sh`, sources all rbhp* below)
- `Tools/rbk/rbh0/rbhpb_base.sh` — payor kindle/sentinel/enforce
- `Tools/rbk/rbh0/rbhpe_establish.sh`
- `Tools/rbk/rbh0/rbhpr_refresh.sh`
- `Tools/rbk/rbh0/rbhpq_quota_build.sh`

**Agent 3 — Windows** (may READ `Tools/rbk/rbhw_windows.sh` and `Tools/rbk/rbhw_cli.sh`, may only WRITE files below):
- `Tools/rbk/rbh0/rbhw_cli.sh` — workbench wiring (moved from `Tools/rbk/rbhw_cli.sh`, sources all rbhw* below)
- `Tools/rbk/rbh0/rbhwb_base.sh` — windows kindle/sentinel
- `Tools/rbk/rbh0/rbhw0_top.sh` — windows orchestrator
- `Tools/rbk/rbh0/rbhwht_handbook_top.sh` — cross-group index (rbhw-prefixed but indexes all three handbook groups; lives here because the `rbw-h0` colophon routes through the windows workbench wiring)
- `Tools/rbk/rbh0/rbhwdd_docker_desktop.sh`
- `Tools/rbk/rbh0/rbhwdn_docker_wsl_native.sh`
- `Tools/rbk/rbh0/rbhwcd_docker_context_discipline.sh`

**Each agent's instructions:**
- Read the source monolith AND the existing CLI file (the CLI file shows the full source chain — BUK infrastructure, rbyc_common, etc.)
- Extract each public function into its own file with proper header, `set -euo pipefail`, source guard
- **Extracted files source nothing** — they are sourced by the CLI file and that is the only way they are called. The CLI file is responsible for ensuring all dependencies are available before sourcing the function files.
- Base file (`*b_base.sh`) gets shared private helpers (`zrbho_*`, `zrbhp_*`, `zrbhw_*`), kindle, sentinel
- CLI file gets expanded source list pointing to all files in the group, preserving the existing dependency sourcing (BUK, rbyc, etc.) before the function file sources
- Do NOT touch any file outside your exclusive write list
- Do NOT delete the original monolith (opus does that in Phase 4)

### Phase 3 — Opus review (serial, as agents finish)

Expect Agent 3 (windows) to finish first, then Agent 2 (payor), then Agent 1 (onboarding). Review each agent's output as it lands:

- Verify source guards are correct and unique
- Verify kindle/sentinel wiring in base files
- Verify CLI file sources all extracted files in dependency order (base first, then function files)
- Verify CLI file preserves existing infrastructure sourcing (BUK, rbyc, etc.) before function files
- Verify no function was lost or duplicated
- Verify extracted files do NOT source anything themselves
- Verify no cross-group file edits

### Phase 4 — Opus wiring and cleanup (serial, after all three reviewed)

1. Update workbench source paths to find CLI files in `rbh0/`
2. Delete original monolithic files: `rbho_onboarding.sh`, `rbhp_payor.sh`, `rbhw_windows.sh`
3. Delete original CLI files from `Tools/rbk/`: `rbho_cli.sh`, `rbhp_cli.sh`, `rbhw_cli.sh`
4. Update CLAUDE.md acronym mappings for new file locations
5. Verify all handbook tabtargets print their content: run every `rbw-h*`, `rbw-o`, `rbw-O*`, `rbw-gP*`, `rbw-gq`, and `rbw-HW*` tabtarget. Use best judgement on which to exercise — goal is confidence that all three CLI source chains resolve correctly.
6. Run fast test suite to confirm no regressions

### CLI wiring — no proliferation
Three CLI files already exist. Each currently sources one monolithic file. After the split, each expands its single `source` line into multiple sources for the exploded files. The CLI files themselves move into `rbh0/`. **No new CLI files are created.**

### Naming convention
Each file gets a minting-compliant subprefix. `0` suffix = collective orchestrator (sorts first).

### eliminate-buh-line-yawp-one-liners (₢A6AAj) [complete]

**[260415-1454] complete**

## Character
Code readability audit — find-and-discuss, not mechanical bulk replace. Each instance may reveal a missing accessor.

## Docket
Audit all call sites where `buh_line` and `_yawp` functions are composed on a single line. These one-liners are hard to read and suggest the yelp/handbook API lacks the right higher-level accessors for those use cases.

For each instance:
1. Identify what the call site is trying to express
2. Determine whether an existing accessor already handles it
3. If not, propose a new accessor that would make the intent clear
4. Discuss with user before changing — some may be intentional

Scope: all `Tools/` shell scripts. Start with `buh_handbook.sh` and `buym_yelp.sh` consumers.

### vouches-package-lowercase (₢A6AAm) [complete]

**[260416-0952] complete**

## Character
Two-part remainder: repair a BCG violation in the build-polling loop, then rerun the service suite to verify the vouches-lowercase fix end-to-end.

## Docket

### Background
`RBGC_VOUCHES_PACKAGE` was uppercase `"VOUCHES"` — OCI registries reject uppercase package names. The code fix (₢A6AAk) changed it to `"vouches"` and is committed/pushed.

### Verification attempt (☉260415-1030)
Ran `tt/rbtd-s.TestSuite.service.sh` from a clean, pushed tree. 79/80 passed. The single failure:

- `rbtdrc_four_mode_supply_chain` — bind step for `rbev-bottle-plantuml` exited 28
- Exit 28 is curl's `CURLE_OPERATION_TIMEDOUT`
- Both the mirror and vouch Cloud Builds **succeeded** (confirmed via `gcloud builds describe`)
- The vouch artifact `b260415152036-r260415222036-vouch` **exists** in the `vouches` package in GAR
- Failure was a transient curl timeout during vouch build polling, but `set -e` killed the script before the retry logic could execute

### BCG violation at `rbfc_FoundryCore.sh:247-253`
The build-polling loop uses a bare curl statement followed by `if test $? -ne 0`:

```bash
curl -s ... > "${ZRBFC_BUILD_STATUS_FILE}"
if test $? -ne 0; then
  # consecutive failure counting + retry
```

With `set -euo pipefail` active (line 21), curl returning non-zero triggers errexit immediately — the `$?` check on line 253 never executes. The retry logic (consecutive failure counting, 3 attempts before die) is correct but unreachable.

### Repair
Apply the BCG capture idiom (`|| z_status=$?` pattern, BCG §"set -e is Not Sufficient"):

```bash
local z_curl_rc=0
curl -s ... > "${ZRBFC_BUILD_STATUS_FILE}" || z_curl_rc=$?
if test "${z_curl_rc}" -ne 0; then
```

Audit the same file for other instances of the same anti-pattern (bare curl followed by `$?` check).

### Then
Rerun `tt/rbtd-s.TestSuite.service.sh` from a clean pushed tree. All 80 cases must pass.

### review-rbho-bcg-compliance (₢A6AAg) [complete]

**[260416-0739] complete**

## Character
Parallel audit sweep — mechanical BCG pattern checks across the decomposed `rbh0/` directory. Each file group can be reviewed independently by a separate agent. Read BCG guide first, then fan out.

## Docket
Comprehensive BCG compliance review of the decomposed handbook files in `Tools/rbk/rbh0/`.

**Prerequisite**: ₢A6AAh (decompose-rbh-into-rbh0-directory) must be wrapped — this pace reviews the decomposed structure, not the monolith.

### Parallel review strategy

The decomposed files fall into three independent groups matching the original monoliths. **Spawn three parallel review agents**, one per group:

**Group 1 — Onboarding (`rbho*`)**: `rbhob_base.sh`, `rbho0_start_here.sh`, `rbhocc_crash_course.sh`, `rbhocr_credential_retriever.sh`, `rbhocd_credential_director.sh`, `rbhofc_first_crucible.sh`, `rbhodf_director_first_build.sh`, `rbhopw_payor_wrapper.sh`, `rbhogw_governor_wrapper.sh`

**Group 2 — Payor (`rbhp*`)**: `rbhpb_base.sh`, `rbhpe_establish.sh`, `rbhpr_refresh.sh`, `rbhpq_quota_build.sh`

**Group 3 — Windows (`rbhw*`)**: `rbhwb_base.sh`, `rbhw0_top.sh`, `rbhwht_handbook_top.sh`, `rbhwdd_docker_desktop.sh`, `rbhwdn_docker_wsl_native.sh`, `rbhwcd_docker_context_discipline.sh`

Each reviewer checks the same three categories within its group:

### Review categories (per group)

1. **BCG violations**: `command -v` inline without guards, unguarded subprocess calls, external binary probes that don't follow BCG-compliant patterns. Check every `docker` invocation, every `command -v`, every unguarded subprocess call.

2. **Unused functions**: Functions defined but never called from any tabtarget, workbench dispatch, or other module. Check the full call graph — grep across the codebase for each function name.

3. **Pattern consistency**: Consistent error handling, variable naming (`z_` prefix discipline), `buc_*`/`buh_*` usage patterns. Compare across files within the group and across groups.

### Merge findings

Collect results from all three parallel reviewers into a single categorized list with file paths and recommended fixes.

### Verification

- Every public function in `rbh0/` is reachable from at least one tabtarget
- No BCG-prohibited patterns survive (inline `command -v`, unguarded externals)
- Source guard / kindle / sentinel discipline consistent across all files

### menu-and-crash-course-draft (₢A6AAA) [abandoned]

**[260415-1512] abandoned**

## Character

Cleanup pace — finish the remaining loose ends from the original foundation implementation. Core code is landed and working (commit `9844ce32`). What remains is smoke testing in alternate repo states and paddock housekeeping.

## Docket

### Smoke test States 1 and 2

Run `tt/rbw-o.OnboardingStartHere.sh` and `tt/rbw-Occ.OnboardingCrashCourse.sh` in two additional repo states:

- **State 1 (Bare-fork)**: run `tt/rbw-MZ.MarshalZeroes.sh`, test both tabtargets, then restore
- **State 2 (Joining)**: populated RBRR but credentials removed from `${RBRR_SECRETS_DIR}`, test both tabtargets, then restore

Document results. Fix bugs if any.

### Paddock refinements

- Finalize track acronym table (`rbw-O??` assignments for all tracks)
- Update Anchor Coordination section (github.com/scaleinv/recipebottle/blob/main/README.md, not github-pages)
- Reconcile implementation-sequence statement (Crash Course landed first, not First Crucible)

### Capture user revision notes

User confirmed both tabtargets function but wants revisions — notes not yet captured. Take revision feedback and implement.

### Out of scope

- Agent-learner eval (now ₢A6AAM)
- New track implementation (now ₢A6AAL)

### ifrit-airgap-feasibility (₢A6AAI) [complete]

**[260416-1034] complete**

## Character

Decision recorded. The feasibility investigation expanded into a full architectural blueprint for ifrit's airgap form, after design conversation surfaced the kludge-cycle tension and the nameplate-symmetry opportunity. Implementation flows through a new rename pace (`ifrit-symmetric-rename`) and `₢A6AAV`.

## Decision

Ifrit goes airgap via a **shared-build-context + three-vessel pattern**, mirroring sentry's tether/airgap split with extension for cargo's airgap-incompatibility. The kludge cycle is preserved by keeping the tether variant as the daily-iteration target.

### Vessel layout

```
common-ifrit-context/         shared rbid source + Dockerfile.tether + Dockerfile.airgap
common-ifrit-forge-context/   forge-only Dockerfile (apt + cargo cache warm)

rbev-bottle-ifrit-tether/     conjure tether — FROM rust:slim-bookworm
rbev-bottle-ifrit-airgap/     conjure airgap — FROM ${forge}
rbev-bottle-ifrit-forge/      conjure tether — apt + dummy cargo cache primer
```

The forge is a separate vessel because its lifecycle differs — rebuilt only on toolchain/deps bumps, its hallmark a *consumed input* to airgap-ifrit, not a runtime image.

### Nameplate symmetry

| Nameplate | Sentry | Bottle | Role |
|-----------|--------|--------|------|
| `tadmor` | `rbev-sentry-deb-tether` | `rbev-bottle-ifrit-tether` | Daily iteration, kludge-friendly, security test authoring |
| `moriah` | `rbev-sentry-deb-airgap` | `rbev-bottle-ifrit-airgap` | Airgap chain demonstration, cloud-built hallmarks, Building for Crucibles target |

Moriah inherits tadmor's uplink config (DNS allowlist, allowed CIDRs/domains — required for security test suite identity). Enclave IP block: 10.243.0.0/24 (avoids tadmor 10.242 collision).

The "Mount Moriah" name fits the airgap discipline — it is THE place of binding in the canonical tradition (Akedah, Temple). Most-bound containment for the most-disciplined ifrit deployment.

### Theurge fixture parity

Theurge's "ifrit" coupling is to the binary `rbid` invoked via `bark`, not to a specific vessel name. Fixture↔nameplate binds by name match. Moriah fixture reuses tadmor's Rust test cases via the existing fixture machinery — no Rust code changes. Moriah stays out of `ZRBTE_SUITE_CRUCIBLE` (requires cloud-built airgap hallmark; not routine).

### Bottle vessel rename

Existing `rbev-bottle-ifrit` → `rbev-bottle-ifrit-tether` for symmetric naming. Mechanical sweep — executed in the new `ifrit-symmetric-rename` pace before AAV begins airgap additions.

### Trajectory

"Airgap is future" — eventually retire the tether variant + tadmor when the kludge cycle tolerates the forge dependency (or kludge-cycle pattern itself evolves). No ETA. Both variants live honestly today.

### Rejected alternatives (consolidated)

- **Sentry pattern unchanged** (one Dockerfile, two rbrv.env): cargo crate compilation requires pre-staged Linux-compiled artifacts; airgap pool can mirror apt but not warm cargo cache. Single Dockerfile insufficient for ifrit.
- **Two unrelated vessel directories** (basis [1] original framing): forces source duplication of `Cargo.toml`/`Cargo.lock`/`src/`. Common context with divergent Dockerfiles eliminates duplication.
- **Vendoring crates into airgap build context**: macOS vendoring for Linux target is unsafe; models wrong customer discipline.
- **Permanent two-vessel coexistence with no shared context**: rejected after surfacing the sentry shared-context precedent and accepting the "abandon or morph" framing — both Dockerfiles + shared source is the clean middle path.

### Paddock updates on wrap

A6 paddock receives:
- Vessel table: rows for tether/airgap/forge variants (renames the prior `bottle-ifrit` row); also corrects sentry rows to match actual `sentry-deb-tether`/`sentry-deb-airgap` names
- Nameplate table: row for moriah; tadmor row's bottle column updated to bottle-ifrit-tether
- Track 12 description: forge → airgap-ifrit → moriah chain
- Remaining-gaps section: ifrit airgap entry closed (decision recorded above)

## Out of scope

- Bottle vessel rename — `ifrit-symmetric-rename` pace
- Forge + airgap-ifrit + moriah creation — `₢A6AAV` Phase 1
- Handbook track content — `₢A6AAV` Phase 2

### ifrit-symmetric-rename (₢A6AAv) [complete]

**[260417-0716] complete**

## Character

Pure mechanical rename for symmetry — wide but shallow. No new features, no design judgment. Establishes a clean tether-only baseline for subsequent airgap additions in `₢A6AAV`. The whole pace is grep-then-sed plus one `git mv`; verification is "does the existing tadmor security suite still pass identically."

## Docket

Rename `rbev-bottle-ifrit` → `rbev-bottle-ifrit-tether` and migrate its build artifacts into a shared `common-ifrit-context/` directory. Mirrors the existing `common-sentry-context/` pattern. Decision recorded in `₢A6AAI`.

### Steps

1. Create `rbev-vessels/common-ifrit-context/` directory.
2. Move source artifacts into the shared context:
   - `rbev-vessels/rbev-bottle-ifrit/src/` → `rbev-vessels/common-ifrit-context/src/`
   - `rbev-vessels/rbev-bottle-ifrit/Cargo.toml` → `rbev-vessels/common-ifrit-context/Cargo.toml`
   - `rbev-vessels/rbev-bottle-ifrit/Cargo.lock` → `rbev-vessels/common-ifrit-context/Cargo.lock`
3. Write `rbev-vessels/common-ifrit-context/Dockerfile.tether` — current ifrit Dockerfile content. Build logic byte-identical aside from preamble comments updated to reference the shared-context layout.
4. Rename directory `rbev-vessels/rbev-bottle-ifrit/` → `rbev-vessels/rbev-bottle-ifrit-tether/` (use `git mv` to preserve history).
5. Rewrite `rbev-vessels/rbev-bottle-ifrit-tether/rbrv.env`:
   - `RBRV_SIGIL=rbev-bottle-ifrit-tether`
   - `RBRV_DESCRIPTION` updated to mention "(tethered)"
   - `RBRV_CONJURE_DOCKERFILE=${RBRR_VESSEL_DIR}/common-ifrit-context/Dockerfile.tether`
   - `RBRV_CONJURE_BLDCONTEXT=${RBRR_VESSEL_DIR}/common-ifrit-context`
   - All other fields preserved (mode, egress, reliquary, image origin/anchor, platforms)
6. Update `.rbk/tadmor/rbrn.env`: `RBRN_BOTTLE_VESSEL=rbev-bottle-ifrit-tether`. Hallmark fields preserved (existing tether hallmark remains valid since the underlying image is unchanged — only the vessel name moves).
7. Grep sweep for whole-word `rbev-bottle-ifrit` across `Tools/`, `*.adoc`, `*.md`, `.rbk/`, `tt/`, `rbev-vessels/`. Update each occurrence to `rbev-bottle-ifrit-tether`. Anticipated touch sites:
   - Rust source: `Tools/rbk/rbtd/src/rbtdrc_crucible.rs`, possibly `rbtdri_*` infrastructure files
   - Specs: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md`, possibly `RBSCC-crucible_charge.adoc`, `RBSIP-ifrit_pentester.adoc`
   - Tabtargets and zipper files (verify any vessel-name parameterized lookups)
   - Paddock A6 vessel table row — defer; will be rewritten at AAI wrap
8. Verify (in this order):
   - `tt/rbw-cKB.KludgeBottle.tadmor.sh` succeeds — local kludge path unbroken
   - `tt/rbw-cC.Charge.tadmor.sh` charges cleanly with renamed vessel
   - `tt/rbtd-r.Run.tadmor.sh` passes the security suite — case count and outcomes identical to pre-rename baseline
   - `tt/rbtd-s.TestSuite.fast.sh` green
   - `grep -rn "rbev-bottle-ifrit\b" --include="*.sh" --include="*.env" --include="*.rs" --include="*.md" --include="*.adoc"` returns zero hits (no orphaned old name)

### Requires

- `₢A6AAI` wrapped (decision recorded — provides the architectural rationale for the rename)

### Out of scope

- Forge vessel creation (`₢A6AAV` Phase 1)
- Airgap-ifrit vessel creation (`₢A6AAV` Phase 1)
- `Dockerfile.airgap` creation (`₢A6AAV` Phase 1)
- Moriah nameplate (`₢A6AAV` Phase 1)
- Theurge fixture additions (`₢A6AAV` Phase 1)
- Any change to `Dockerfile.tether` build logic — must be byte-equivalent to current ifrit Dockerfile (preamble comments excepted)
- Paddock vessel/nameplate table rewrites — happen at AAI wrap

### docs-moriah-introduction (₢A6AAw) [complete]

**[260416-1823] complete**

## Character

Prose-judgment-heavy documentation pace. Sits between `₢A6AAv` (mechanical rename) and `₢A6AAV` (airgap implementation). Gates AAV Phase 2 because the handbook will hyperlink to `${RBGC_PUBLIC_DOCS_URL}#moriah` via `buh_tlt`, and that anchor must exist before the handbook content lands. Cognitive posture: careful prose, parallel-structure discipline (moriah definitions mirror tadmor's), spec coherence over speed.

## Docket

Introduce the moriah nameplate as first-class vocabulary across the documentation surface, alongside the existing tadmor. Refine tadmor's prose where the kludge-friendly framing now applies. Acknowledge the tether/airgap/forge vessel family that AAv produced.

### README.md

1. **New `<a id="moriah"></a>` anchor + definitional prose** (insert near tadmor's, line 223 area). Parallel structure to tadmor's two-line definition: identifies moriah as the airgap-built adversarial-validation Crucible, pairs sentry-deb-airgap with bottle-ifrit-airgap, used by the Building-for-Crucibles onboarding track to demonstrate end-to-end airgap supply chain. Theurge runs the same security suite against moriah as tadmor — the cloud-built airgap variant validating containment identically.
2. **Refine tadmor prose** (lines 223-225). One-sentence addition framing tadmor as the daily-iteration / kludge-friendly nameplate, with moriah as its airgap counterpart. Existing semantics preserved.
3. **Glossary mention** (line 191). Optional: extend "(e.g. `tadmor`)" to "(e.g. `tadmor`, `moriah`)" — judgment call at write time.
4. **Directory listing — Nameplates section** (line 505). Add `│   ├── moriah/` row paralleling tadmor's.
5. **Directory listing — Vessels section** (around line 523). Rename `rbev-bottle-ifrit/` row to `rbev-bottle-ifrit-tether/`. Add new rows for `rbev-bottle-ifrit-airgap/`, `rbev-bottle-ifrit-forge/`, `common-ifrit-context/` (shared Dockerfile.tether + Dockerfile.airgap + src), `common-ifrit-forge-context/` (forge-only Dockerfile + cargo cache primer). Use parallel structure to existing `common-sentry-context/` row.

### RBSIP-ifrit_pentester.adoc

1. **Vessel name updates** (mechanical, may already be done by AAv's sweep). Verify line 78 heading and line 102 reference are `rbev-bottle-ifrit-tether` post-AAv. If AAv handled them, no work here.
2. **Vessel family explanation** (insert near §"Ifrit Bottle Vessel"). New paragraph or subsection acknowledging the three-vessel family produced by AAI's decision: tether for daily kludge iteration, airgap for cloud-built supply-chain demo, forge as build-time fixture warming the cargo cache. Cross-reference the AAI decision in `₣A6` paddock if appropriate.
3. **Moriah introduction** (line 101 area — "The Ifrit bottle is the tadmor nameplate's bottle vessel"). Update to acknowledge moriah as the airgap counterpart: "The tether variant of the Ifrit bottle is the tadmor nameplate's bottle vessel for daily-iteration adversarial validation. The airgap variant is the moriah nameplate's bottle vessel for the same security suite run against cloud-built hallmarks." Or similar — judgment at write time.
4. **Compose fragment naming** (line 125). Note that compose fragment naming follows the nameplate (`tadmor.compose.yml`, `moriah.compose.yml`).
5. **Out of scope but noted**: line 196's `Tools/rbk/rbtid/` is stale (actual: `Tools/rbk/rbtd/`). Flagged for separate cleanup, not part of this pace.

### RBS0-SpecTop.adoc

Judgment call: does moriah deserve a `rbsi_moriah` quoin (parallel to `rbsi_ifrit` at lines 500-503)? If yes, add the attribute reference + ensure RBSIP defines `[[rbsi_moriah]]` at moriah's introduction. If no (moriah is treated as a nameplate-name-of-record, not a spec concept), skip. Default lean: skip — moriah is a nameplate moniker, the underlying spec concept is the airgap variant of the existing Ifrit framework, which is already covered.

### RBSHR-HorizonRoadmap.adoc

Line 144 references `.rbk/tadmor/rbrn.env` in a roadmap item. Read the surrounding context at write time. If the roadmap item is about adversarial testing in general, add `.rbk/moriah/rbrn.env` alongside. If it's specifically about a tadmor-anchored feature, leave alone.

### Verification

- `grep -n "moriah" README.md` returns the new anchor + nameplate row + (optional) glossary mention
- `grep -n "moriah" Tools/rbk/vov_veiled/RBSIP-ifrit_pentester.adoc` returns the new vessel-family paragraph
- All `<a id="..."></a>` anchors in README parse cleanly (no duplicates, no broken HTML)
- Render `tt/rbw-h0` and `tt/rbw-Odf` (or any other handbook tabtarget that hyperlinks to README anchors) — confirm no dangling references introduced
- Visual diff: README's tadmor and moriah definitions read in parallel structure (parallel sentence lengths, parallel verb choices, parallel cross-link patterns)

### Requires

- `₢A6AAv` (`ifrit-symmetric-rename`) wrapped — mechanical renames in spec files complete; this pace does prose additions on top of the renamed baseline.

### Out of scope

- Vessel rename (handled by `₢A6AAv`)
- Forge / airgap-ifrit / moriah nameplate **creation** (handled by `₢A6AAV` Phase 1)
- Handbook track **content** (handled by `₢A6AAV` Phase 2 — depends on this pace's anchors existing)
- RBSIP `rbtid` → `rbtd` path correction (separate hygiene, pre-existing drift)
- AAF final docs hygiene sweep (catches anything missed; not a substitute for targeted prep here)

### yelp-and-vocabulary-infrastructure (₢A6AAa) [complete]

**[260415-0906] complete**

## Character

Infrastructure build — new BUK module (`buy_yelp.sh`) and RBK vocabulary module (`rbyc_common.sh`). Mechanical implementation of a design that's fully decided. Validate by converting one onboarding function to the new pattern.

## Docket

### Problem

Handbook procedures compose formatted console output via `buh_` combinators whose function names encode color sequences positionally (e.g., `buh_tltltltlt` consumes 13 args with `l` taking two each). This causes:
- Arg-counting bugs (multiple recent commits fixing `buh_tlt` vs `buh_tltlt` mismatches)
- Combinatorial explosion of function variants (~40 combinators, growing)
- Linked terms repeated verbatim at every call site (`"Depot" "${z_docs}#Depot"`)
- Formatted vocabulary trapped inside `buh_handbook.sh`, unusable in regular command output

### Design (decided)

**BUK layer — `Tools/buk/buy_yelp.sh`**

New module providing capture functions that produce formatted string fragments ("yelps") for subshell capture into local variables. Dispatch-aware: respects terminal capabilities detected at kindle time.

Public capture functions (all produce stdout for `$()` capture):
- `buy_link_capture base_url anchor [display_text]` — OSC-8 hyperlink; display defaults to anchor
- `buy_cmd_capture text` — command/code styling
- `buy_ui_capture text` — UI element styling
- `buy_tt_capture colophon` — resolved tabtarget path

Configurators (called before kindle, control formatting behavior):
- `buy_configure_dispatch` — honor `BURD_*` / `NO_COLOR` / `TERM` (default)
- `buy_configure_unconditional` — always apply ANSI/OSC-8
- `buy_configure_plain` — never format, plain text only

Kindle pattern: sentinel-guarded like `zbuh_kindle`. Configurator sets mode variable; kindle reads it on first capture call.

**RBK layer — `Tools/rbk/rbyc_common.sh`**

Recipe Bottle common vocabulary constants. Kindled after regime loads (needs `RBRR_PUBLIC_DOCS_URL`). Does not source or kindle anything itself — the consuming `_cli.sh` is responsible for sourcing `buy_yelp.sh` and `rbyc_common.sh`, then calling `zrbyc_kindle`.

Constants declared as readonly globals via subshell capture:
```
readonly RBYC_DEPOT=$(buy_link_capture "$z_docs" "Depot")
readonly RBYC_HALLMARK=$(buy_link_capture "$z_docs" "Hallmark")
readonly RBYC_DIRECTOR=$(buy_link_capture "$z_docs" "Director")
readonly RBYC_DIRECTORS=$(buy_link_capture "$z_docs" "Director" "Directors")
```

Initial term inventory seeded from the README Anchor Inventory in the A6 paddock (38 anchors).

**BUH addition — `buh_line`**

New public function in `buh_handbook.sh`. Indent-aware (respects `z_buh_body_indent` from step context), interprets `%b` escapes. Name communicates "this line is pre-composed from yelp fragments."

### Validation

Convert one representative onboarding function in `rbho_onboarding.sh` (the menu display with heavy `buh_tltlt` usage) from combinator style to yelp+constants style. Both patterns coexist — existing combinators are not removed.

### Implementation files

- `Tools/buk/buy_yelp.sh` — new: capture functions + configurators
- `Tools/rbk/rbyc_common.sh` — new: common vocabulary kindle
- `Tools/buk/buh_handbook.sh` — add `buh_line`
- `Tools/rbk/rbho_onboarding.sh` — convert one function as validation
- `Tools/rbk/rbho_cli.sh` — source `buy_yelp.sh` and `rbyc_common.sh`, kindle
- `CLAUDE.md` — add BUY and RBYC acronym entries

### Prefix allocations

| Prefix | Universe | Meaning |
|--------|----------|---------|
| `buy` | BUK primary | Yelp — formatted string capture |
| `rby` | RBK primary (non-terminal) | Recipe Bottle vocabulary |
| `rbyc` | RBK primary | Common vocabulary domain |

### Out of scope

- Removing existing `buh_` combinators (coexist indefinitely)
- Migrating all handbook call sites (incremental, future paces)
- `buy_tt_capture` with imprint variant (inline use only, not kindle constants)
- New onboarding track implementations

### diastema-yawp-infrastructure (₢A6AAc) [complete]

**[260415-1132] complete**

## Character

Infrastructure build — extends the yelp architecture with the diastema wire format, `_yawp` function category, public color constants, and centralized format intelligence. Yelp stampers are pure string assignment (no subshells, no printf, no escape sequences); all terminal capability decisions are resolved by a single format function using BASH_REMATCH. Introduces a new BCG function category (`_yawp`) for pure-computation group return variables. Mechanical implementation of a design refined through extensive conversation.

## Docket

### Problem

The yelp infrastructure from ₢A6AAa introduced `buh_line` for vocabulary-in-prose, but handbook functions still intermingle old positional-color combinators (`buh_tTc`, `buh_ct`, `buh_tW`, etc.) with the new pattern. The combinators cannot express "return to line default color after a yelp" — they hard-reset to terminal default. This prevents yelp-aware semantic line functions (`buh_code`, `buh_warn`, `buh_error`) from maintaining their default color through embedded yelps.

Additionally, the payor handbook (`rbhp_payor.sh`) uses `buh_link` extensively for full web URLs (Google Cloud Console pages). The current yelp vocabulary only has `buy_link_capture` which is vocabulary-anchor-oriented. There is no capture function for arbitrary web hyperlinks.

### Design (decided)

#### File and module

File renamed from `buy_yelp.sh` to `buym_yelp.sh`. Module prefix is `buym`. BCG module infrastructure at the parent level: `zbuym_kindle()`, `zbuym_sentinel()`, `ZBUYM_SOURCED`, `ZBUYM_KINDLED`.

#### Namespace split

The `buy` prefix becomes non-terminal with four children:

| Prefix | Role | Examples |
|--------|------|---------|
| `buyy_` | yelp yawp functions | `buyy_link_yawp`, `buyy_cmd_yawp`, `buyy_href_yawp` |
| `buyc_` | configurators | `buyc_dispatch()`, `buyc_unconditional()`, `buyc_plain()` |
| `buyf_` | format yawp | `buyf_format_yawp(color, string)` |
| `buym_` | module infrastructure | `zbuym_kindle()`, `zbuym_sentinel()` |

#### Yawp function category (new BCG pattern)

**`_yawp`** — a function that produces a value into a shared group return variable via pure bash assignment. No subshells, no stdout, no stderr, cannot fail. The name references Whitman's "barbaric yawp" — a loud, unmistakeable declaration.

Contract:
- Sets `z_«module»_«group»` return variable (mutable kindle state)
- Caller reads immediately; value valid until next `_yawp` call in same group
- Pure bash builtins only — no external commands, no process spawning
- Cannot fail — no `|| buc_die` needed at call site
- First line: sentinel check

Two yawp groups in this module:
- **yelp group**: `buyy_*_yawp` functions set `z_buym_yelp`
- **format group**: `buyf_format_yawp` sets `z_buym_format`

#### Diastema wire format

Yelp yawp functions are **inert marker-stampers**. They embed no colors and no escape sequences — only non-printing byte markers via pure string assignment. All terminal capability decisions are deferred to `buyf_format_yawp`.

**Diastema family** — eight non-printing byte sequences defined as kindle constants:

| Constant | Role |
|----------|------|
| `ZBUYM_DIASTEMA_CMD` | Opens command-styled region |
| `ZBUYM_DIASTEMA_UI` | Opens UI-element region |
| `ZBUYM_DIASTEMA_HREF_URL` | Opens web hyperlink URL data |
| `ZBUYM_DIASTEMA_HREF_TEXT` | Opens web hyperlink display text |
| `ZBUYM_DIASTEMA_LINK_URL` | Opens vocabulary link URL data |
| `ZBUYM_DIASTEMA_LINK_TEXT` | Opens vocabulary link display text |
| `ZBUYM_DIASTEMA_TT` | Opens tabtarget region |
| `ZBUYM_DIASTEMA_END` | Closes any region — return to ambient |

All internal (`ZBUYM_`) kindle constants. HREF and LINK have separate markers because they carry different default colorations: vocabulary links (teaching terms) and web links (external references) serve different pedagogical purposes and deserve independent palette slots.

A yelp yawp is pure string assignment:
```
z_buym_yelp="${ZBUYM_DIASTEMA_CMD}${z_text}${ZBUYM_DIASTEMA_END}"
```

Hyperlink yawps embed URL as structured data between markers:
```
DIASTEMA_HREF_URL  url  DIASTEMA_HREF_TEXT  display  DIASTEMA_END
DIASTEMA_LINK_URL  url  DIASTEMA_LINK_TEXT  display  DIASTEMA_END
```

No OSC-8 at yawp time. `buyf_format_yawp` constructs OSC-8 (or fallback) at format time.

#### Yelp yawp functions (buyy_*_yawp)

All set `z_buym_yelp` via pure assignment. No stdout, no stderr, no process spawning.

- **`buyy_cmd_yawp(text)`** — stamps CMD region
- **`buyy_ui_yawp(text)`** — stamps UI region
- **`buyy_href_yawp(url, display)`** — stamps HREF region with raw URL. For arbitrary web pages (Google Console, etc.).
- **`buyy_link_yawp(base_url, anchor, [display])`** — concatenates `base_url#anchor`, stamps LINK region. Vocabulary convenience; display defaults to anchor.
- **`buyy_tt_yawp(colophon, [imprint])`** — resolves tabtarget path via glob (bash builtin), stamps TT region. Optional second arg for imprint resolution.

Calling pattern:
```bash
buyy_cmd_yawp "git status"
local -r z_cmd="${z_buym_yelp}"

buyy_href_yawp "https://console.cloud.google.com/billing" "Google Cloud Billing"
local -r z_billing="${z_buym_yelp}"

buyy_link_yawp "${z_docs}" "Depot"
local -r z_depot="${z_buym_yelp}"

buh_line "Run ${z_cmd} to see your ${z_depot} status, then go to ${z_billing}."
```

#### Public color constants (BUYC_*)

Configurators set a mode flag. Kindle reads the flag and defines `readonly BUYC_*` constants. Yelp yawps never reference them; only `buyf_format_yawp` reads them.

| Constant | Default value | Semantic use in buh |
|----------|---------------|---------------------|
| `BUYC_RESET` | `\033[0m` | prose default |
| `BUYC_CYAN` | `\033[36m` | command/code |
| `BUYC_MAGENTA` | `\033[35m` | UI element |
| `BUYC_BRIGHT_YELLOW` | `\033[1;33m` | warning |
| `BUYC_BRIGHT_RED` | `\033[1;31m` | error |
| `BUYC_BRIGHT_WHITE` | `\033[1;37m` | section header |
| `BUYC_LINK` | `\033[34;4m` | vocabulary hyperlink |
| `BUYC_HREF` | `\033[34;4m` | web hyperlink |

No-color mode: all set to empty string. LINK and HREF start with the same value but have independent palette slots for future differentiation.

#### Configurators (buyc_*)

Called before kindle. Each sets `ZBUYM_CONFIG_MODE` flag:

- **`buyc_dispatch()`** — sets `"dispatch"` (probe TERM/NO_COLOR/BURD_*)
- **`buyc_unconditional()`** — sets `"unconditional"` (always color)
- **`buyc_plain()`** — sets `"plain"` (never color)

Kindle reads the mode and defines `readonly BUYC_*` palette and hyperlink mode. BCG-compliant: kindle is the sole definer of constants.

#### Format yawp (buyf_*)

**`buyf_format_yawp(color, string)`** — the single intelligence point. Takes a pure `BUYC_*` color constant (the line's ambient color) and a diastema-marked string. Sets `z_buym_format` with the resolved string.

Resolution steps:
1. Simple markers (`DIASTEMA_CMD`, `DIASTEMA_UI`, `DIASTEMA_TT`): replace opener with corresponding `BUYC_*` color
2. Structured markers (HREF and LINK): extract URL and display via BASH_REMATCH. If hyperlinks enabled: construct OSC-8 + type-specific color (`BUYC_HREF` or `BUYC_LINK`). If disabled: display text + ` <url>` in type-specific color.
3. All `DIASTEMA_END` markers: replace with ambient color
4. Prepend ambient color, append `BUYC_RESET`
5. Assign result to `z_buym_format`

BASH_REMATCH regex with pattern stored in variable (Bash 3.2 compatible). Loop for multiple links per line. Non-printing markers interpolated into pattern variable. Pure bash builtins — no external commands.

#### Semantic line functions (buh_*)

- `buh_line` — prose (passes `BUYC_RESET`)
- `buh_code` — command/code (passes `BUYC_CYAN`)
- `buh_warn` — warning (passes `BUYC_BRIGHT_YELLOW`)
- `buh_error` — error (passes `BUYC_BRIGHT_RED`)

Each is a thin wrapper:
```bash
buh_line() {
  zbuh_sentinel
  buyf_format_yawp "${BUYC_RESET}" "${1}"
  printf '%s\n' "${z_buh_body_indent}${z_buym_format}" >&2
}
```

The buh layer owns semantic mapping; the buyf layer owns resolution. buh reads `z_buym_format` as a documented consumer of the format yawp group.

#### Color constant relationship

Existing `ZBUH_*` constants in buh remain for legacy combinators. New semantic line functions route through `BUYC_*` exclusively. No consolidation — coexistence is fine.

### Validation

Convert `rbho_director_first_build()` from mixed combinator+yelp style to pure yelp style. This function has heavy combinator usage across all semantic categories (prose, commands, probes, warnings). No legacy combinators should remain in the converted function.

### Design note: payor coverage

The payor handbook (`rbhp_payor.sh`) was examined to validate the yelp vocabulary's completeness. It uses `buh_tu`/`buh_tc`/`buh_tut` heavily plus 15 `buh_link` calls with full Google Console URLs. The `buyy_href_yawp` function and diastema wire format close the vocabulary gap — all payor primitives are expressible as yelp yawps + semantic line functions. Converting the payor is out of scope for this pace but the infrastructure supports it.

### Implementation files

- `Tools/buk/buym_yelp.sh` — full rework (renamed from `buy_yelp.sh`): `ZBUYM_DIASTEMA_*` markers, `BUYC_*` public color constants, `buyy_*_yawp` functions (inert stampers via assignment), `buyc_*` configurators (mode flag), `buyf_format_yawp` (BASH_REMATCH resolver), `zbuym_kindle`/`zbuym_sentinel`
- `Tools/buk/buh_handbook.sh` — add `buh_code`, `buh_warn`, `buh_error`; update `buh_line` to use `buyf_format_yawp`; update sourcing reference from `buy_yelp.sh` to `buym_yelp.sh`
- `Tools/rbk/rbho_onboarding.sh` — convert `rbho_director_first_build()` to pure yelp style
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` — update yelp quoins: diastema wire format, yawp pattern, format_yawp, href/link distinction, BUYC_* constants, namespace split, file rename
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — add `_yawp` function category to Special Function Patterns, naming conventions, module maturity checklist, and when-to-use guidance
- Update all sourcing references from `buy_yelp.sh` to `buym_yelp.sh` across CLI furnish functions

### Out of scope

- Removing legacy combinators from `buh_handbook.sh` (coexist indefinitely)
- Converting other handbook functions beyond `rbho_director_first_build`
- Converting non-handbook consumers of `buh_*` combinators
- Converting `rbhp_payor.sh` to yelp style
- Consolidating `ZBUH_*` color constants in buh (legacy combinators keep their own)

### reliquary-fact-and-rubric-elimination (₢A6AAb) [complete]

**[260415-0907] complete**

## Character

Mechanical spec-and-code hygiene — eliminate dead quoins, add a new fact file following an established pattern, create a small subdocument. No design judgment needed; every change follows an existing exemplar.

## Docket

### Problem

1. `rbfl_inscribe()` computes a reliquary datestamp and prints it to console but never persists it as a fact file. The handbook tells learners to eyeball the output and manually set `RBRV_RELIQUARY`. Every other foundry pipeline (ordain/kludge) writes fact files for downstream consumers.

2. RBSTB-trigger_build.adoc contains 9 references to eliminated quoins (`rbtgo_rubric_inscribe` × 5, `rbtgr_rubric_repo` × 4). RBS0-SpecTop.adoc still carries the `rbtgr_rubric` quoin (mapping, definition, one prose reference). All dead since the rubric concept was eliminated in ₣Av.

### Changes

**Eliminate rubric quoins:**
- RBS0-SpecTop.adoc: delete `rbtgr_rubric` / `rbtgr_rubric_s` mapping lines (97-98), delete `[[rbtgr_rubric]]` anchor+definition (1899-1901), reword prose at line 2179 to remove `{rbtgr_rubric_s}`
- RBSTB-trigger_build.adoc: reword or delete all 9 `{rbtgo_rubric_inscribe}` and `{rbtgr_rubric_repo}` references — replace with prose describing the current builds.create + pouch mechanism where context is needed, delete where context is not

**Add reliquary fact file:**
- RBS0-SpecTop.adoc: add `:rbf_fact_reliquary:` attribute reference in mapping section after existing `rbf_fact_*` block; add `[[rbf_fact_reliquary]]` definition in §Foundry Fact Files after `rbf_fact_ark_yield`
- rbgc_Constants.sh: add `readonly RBF_FACT_RELIQUARY="rbf_fact_reliquary"` after existing `RBF_FACT_*` constants
- rbfl_FoundryLedger.sh: add `buf_write_fact "${RBF_FACT_RELIQUARY}" "${z_reliquary}"` in `rbfl_inscribe()` after the submit call succeeds

**Create inscribe subdocument:**
- New file: RBSDI-depot_inscribe.adoc — operation spec for reliquary creation, following the axho_operation pattern used by peer subdocuments. Specifies: director authentication, reliquary datestamp derivation, Cloud Build submission (tool image mirror), fact file output (`RBF_FACT_RELIQUARY`). Include from RBS0 in the appropriate section.

### Validation

- `grep -r rbtgr_rubric Tools/rbk/` returns zero hits
- `grep -r rbtgo_rubric_inscribe Tools/rbk/` returns zero hits
- `grep RBF_FACT_RELIQUARY Tools/rbk/rbgc_Constants.sh` shows the new constant
- `grep buf_write_fact Tools/rbk/rbfl_FoundryLedger.sh` includes the reliquary write

### Out of scope

- Updating rbho_onboarding.sh to consume the fact file (consumer side — separate pace)
- Eliminating `rbtgog_rubric` (Build Definition Group) if still live — separate assessment
- Broader RBSTB rewrite beyond dead-reference removal

### conjure-pipeline-spec-revision (₢A6AAT) [complete]

**[260414-1311] complete**

## Character

Specification-first pace — update RBS0 to describe the target conjure pipeline architecture before implementation begins. Design conversation requiring judgment: the spec must accurately describe the new tag lifecycle, hallmark minting, and attestation scaffolding so that implementation paces (₢A6AAR, ₢A6AAQ) have an authoritative reference to build against. Consult BCG for any bash-relevant patterns referenced by the spec.

## Docket

Revise `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` to describe the target conjure pipeline. All changes are spec prose — no code changes in this pace.

### Multi-platform pipeline (~1870-1898)

Rewrite the target multi-platform pipeline section to reflect the new 6-step pipeline:

1. `derive-tag-base` — receive host-minted hallmark via `_RBGY_HALLMARK`, write to `.hallmark` + `/builder/outputs/output`
2. `qemu-binfmt` — register cross-architecture emulation (if needed)
3. `buildx --push` — build all platforms, push consumer-facing `-image` manifest list to GAR (was `-multi`)
4. `per-platform-pullback` — `docker pull --platform` from `-image`, `docker tag` to `{hallmark}-attest-{arch}`
5. `push-per-platform` — push `{hallmark}-attest-{arch}` tags to GAR
6. `push-diags` — diagnostics artifact

`images:` field lists `{hallmark}-attest-{arch}` tags — CB pushes post-steps and generates SLSA provenance per digest. Step 6 (`imagetools-create`) eliminated — buildx output IS the consumer manifest.

Document that `-attest-{arch}` tags are **ephemeral attestation scaffolding**: they exist solely to satisfy the `images:` field for SLSA provenance generation. Provenance attaches to digests in the Container Analysis API. Tags are swept by the host (director credentials, repoAdmin) after vouch completes, as part of the ordain pipeline.

### Hallmark tag inventory (~1654)

Replace the current tag naming table with the target inventory:

| Tag | Lifecycle | Purpose |
|-----|-----------|---------|
| `{hallmark}-image` | Durable | Consumer multiplatform manifest (from buildx) |
| `{hallmark}-about` | Durable | SBOM + build info |
| `{hallmark}-vouch` | Durable | SLSA verification record |
| `{hallmark}-diags` | Durable | Diagnostics |
| `{hallmark}-pouch` | Durable | Build context (FROM SCRATCH OCI image) |
| `{hallmark}-attest-{arch}` | Ephemeral | SLSA attestation scaffolding; swept post-vouch |

Remove references to `-image-{arch}` as durable per-platform tags. Remove `-multi` intermediate tag.

### Hallmark minting (~1064, and throughout)

Update to reflect host-side minting for all modes:

| Mode | Where minted | Format |
|------|-------------|--------|
| Conjure | Host (director) | `c{YYMMDDHHMMSS}-r{YYMMDDHHMMSS}` |
| Bind | Host (director) | `b{YYMMDDHHMMSS}-r{YYMMDDHHMMSS}` |
| Graft | Host (director) | `g{YYMMDDHHMMSS}-r{YYMMDDHHMMSS}` |
| Kludge | Host (local) | `k{YYMMDDHHMMSS}-{git_context}` |

Remove any language about hallmark being derived inside Cloud Build. The `_RBGY_INSCRIBE_TIMESTAMP` substitution is superseded by `_RBGY_HALLMARK`.

### Pouch definition

Add a formal definition for pouch as a hallmark artifact. Currently "pouch" is used descriptively (~1063, 1134, 1619, 1835, 1843) but has no `[[anchor]]` or attribute reference. Either:
- Add a quoin (`rbtga_pouch` or similar) with anchor and definition
- Or add it to the hallmark artifact inventory with clear naming: `{hallmark}-pouch`

Refine all "build context delivered via pouch" references to cite the formal `{hallmark}-pouch` tag naming.

### Single-platform pipeline (~1862-1868)

Update to reflect host-minted hallmark. The single-platform pipeline is simpler (no pullback, no `-attest-` tags, `buildx --load` + `images:` push) but should use the same `_RBGY_HALLMARK` substitution instead of `_RBGY_INSCRIBE_TIMESTAMP`.

### SLSA provenance section (~1846-1898)

Add explanatory text about the attestation scaffolding pattern:
- `buildx --push` produces the consumer `-image` manifest but CB has no awareness of this push
- `-attest-{arch}` tags bridge buildx output to `images:` field for SLSA provenance
- Provenance is keyed by digest in Container Analysis API, not by tag
- Deleting `-attest-{arch}` tags does not delete provenance
- Host sweeps tags after vouch using director credentials (mason lacks delete permission)

### Vouch description

Update to note that vouch verifies by digest via the `-image` manifest list (not by per-platform tag). Add that the ordain pipeline sweeps `-attest-{arch}` tags after vouch completes.

### Rubric definitions (~1830-1843)

Update rubric definition to reference `{hallmark}-pouch` instead of generic "pouch (FROM SCRATCH OCI image in GAR)".

### Out of scope

- Code changes (implementation is ₢A6AAR and ₢A6AAQ)
- MCM normalization of RBS0 (separate concern)
- Sections unrelated to the conjure pipeline

### host-side-conjure-hallmark (₢A6AAR) [complete]

**[260414-1322] complete**

## Character

Architectural refactoring of the conjure pipeline. Three coupled changes: move hallmark minting to host side, promote buildx output to consumer-facing `-image` tag (eliminating `-multi` and `imagetools create`), and make per-platform attestation tags ephemeral (`-attest-{arch}`, swept by host after vouch). Moderate risk — touches GCB step scripts, stitch, vouch flow, and abjure. Consult BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) for bash patterns before writing new functions. Single test verification: build a vessel, confirm SLSA attestation and consumer pull still work.

**Depends on**: ₢A6AAT (conjure-pipeline-spec-revision) — spec describes the target architecture; implementation follows.

## Docket

Refactor the conjure pipeline to produce a cleaner tag inventory and consistent hallmark minting. RBS0 already describes the target state (updated by ₢A6AAT).

### Background: SLSA provenance constraint

Cloud Build generates SLSA provenance ONLY for images listed in the `images:` field of `builds.create`. The `images:` field pushes from the local Docker daemon AFTER all build steps complete. `buildx --push` pushes directly to GAR from within a build step — CB has no awareness of those pushes and generates no provenance for them. The per-platform pullback exists to bridge this gap: pull buildx-pushed images back into the local daemon so they can be declared in `images:` and receive SLSA attestation.

The vouch verifier (`rbgjv02-verify-provenance.py`) already verifies by **digest**, not by tag. It fetches the `-image` manifest list, extracts per-platform digests, and looks up provenance via `gcloud artifacts docker images describe {image}@{digest} --show-provenance`. Provenance is stored in the Container Analysis API keyed by digest. Deleting a tag does not delete provenance.

### 1. Move hallmark minting to host side

Currently `rbgjb01-derive-tag-base.sh` runs `date -u` inside Cloud Build to produce the realized timestamp. Instead:

- Mint the full hallmark on the host in `rbfd_FoundryDirectorBuild.sh` before build submission, using the same `{inscribe_ts}-r{date -u}` pattern that bind and graft use
- Pass the full hallmark into Cloud Build via a new substitution variable `_RBGY_HALLMARK`
- Simplify `rbgjb01-derive-tag-base.sh` — receive hallmark from substitution, write to `.hallmark` + `/builder/outputs/output`
- Remove `_RBGY_INSCRIBE_TIMESTAMP` substitution — no longer needed; step scripts use `_RBGY_HALLMARK` directly
- Simplify post-build hallmark discovery (line ~1132-1151) — host already knows the hallmark; reduce to a consistency assert comparing discovered vs expected, or remove entirely if the assert isn't load-bearing

**Also apply to single-platform pipeline**: the single-platform stitch path has the same inscribe-only hallmark pattern. Update it to use the host-minted hallmark for consistency. Single-platform is the simpler case (no pullback, no `-multi`).

### 2. Promote buildx output to `-image`, eliminate `imagetools create`

- `rbgjb03-buildx-push-multi.sh` → **rename to `rbgjb03-buildx-push-image.sh`**: push to `{hallmark}-image` instead of `{inscribe_ts}-multi`
- **Eliminate `rbgjb06-imagetools-create.sh`** entirely — buildx output IS the consumer manifest list, no reassembly needed
- Remove step 6 from the step definitions array in `zrbfd_stitch_build_json`
- No `RBGC_ARK_SUFFIX_MULTI` constant needed (suffix eliminated entirely)

### 3. Rename per-platform tags to `-attest-{arch}`

Per-platform tags exist solely to satisfy the `images:` field for SLSA provenance generation. They are ephemeral attestation scaffolding.

- Add `RBGC_ARK_SUFFIX_ATTEST="-attest"` constant to `Tools/rbk/rbgc_Constants.sh`
- `rbgjb04-per-platform-pullback.sh`: pull from `-image`, tag as `{hallmark}-attest-{arch}` (was `-image-{arch}`)
- `rbgjb05-push-per-platform.sh`: push `{hallmark}-attest-{arch}` tags
- Stitch `images:` field: list `{hallmark}-attest-{arch}` tags (CB pushes these post-steps and generates SLSA provenance per digest)
- Pass `RBGC_ARK_SUFFIX_ATTEST` into Cloud Build substitutions alongside `RBGC_ARK_SUFFIX_IMAGE`

### 4. Host sweeps `-attest-{arch}` after vouch

Mason (the Cloud Build service account) has `roles/artifactregistry.writer` — no delete permission. The director SA on the host has `roles/artifactregistry.repoAdmin` — can delete. Therefore cleanup happens host-side.

In `rbfd_ordain`, after `rbfv_vouch` returns (line ~1014):

- Add internal function (e.g., `zrbfd_sweep_attest_tags`) that iterates `RBRV_CONJURE_PLATFORMS`, derives each `-attest-{arch}` suffix, and DELETEs via the GAR registry API
- Tolerate 404 (tags may already be gone)
- Not user-facing — no confirmation prompt, no `buc_require`. Silent cleanup as part of the ordain pipeline
- The Docker Registry HTTP API V2 does not support wildcards — each DELETE targets a specific tag: `DELETE /v2/{name}/manifests/{hallmark}-attest-{suffix}`
- Three API calls for a 3-platform vessel, deterministic from `RBRV_CONJURE_PLATFORMS`

### 5. Update abjure and tally

- **Abjure**: remove `-multi` cleanup (gone), remove `-image-{arch}` cleanup (gone), add `-attest-{arch}` cleanup as safety net (in case ordain failed before sweep). Same platform-iteration logic
- **Tally classifier**: add `-attest-{arch}` case — presence means ordain didn't complete (vouch or sweep failed). Do NOT adjust health model for pouch

### 6. Update stitch and step inventory

- Remove step 6 (`imagetools-create`) from step definitions array
- Rename step 3 file (`rbgjb03-buildx-push-multi.sh` → `rbgjb03-buildx-push-image.sh`)
- Pipeline goes from 7 steps to 6 (conjure) or 3 steps to 3 (single-platform, unchanged count but hallmark source changes)
- `buildStepOutputs` index for hallmark consistency check: verify correct index after step removal, or remove the check if host-minted hallmark makes it redundant

### Target tag inventory per hallmark

| Tag | Lifecycle | Purpose |
|-----|-----------|---------|
| `{hallmark}-image` | Durable | Consumer multiplatform manifest (from buildx) |
| `{hallmark}-about` | Durable | SBOM + build info |
| `{hallmark}-vouch` | Durable | SLSA verification record |
| `{hallmark}-diags` | Durable | Diagnostics |
| `{hallmark}-pouch` | Durable | Build context (after ₢A6AAQ) |
| `{hallmark}-attest-{arch}` | Ephemeral | SLSA attestation scaffolding; swept by host after vouch |

### Verification

- Build a test vessel (e.g., `rbev-busybox`, 3-platform, fast)
- Confirm `cosign verify-attestation` works on `-attest-{arch}` tags during vouch (before sweep)
- Confirm `-attest-{arch}` tags are gone after ordain completes
- Confirm `docker pull {hallmark}-image` resolves correctly on the host platform
- Confirm tally and abjure handle the new tag scheme
- Confirm single-platform vessel still builds with host-minted hallmark

### Spec prose

- RBS0 already updated by ₢A6AAT — verify implementation matches spec

### Out of scope

- Pouch rename (₢A6AAQ — depends on this pace)
- Cleanup of legacy `context-*`, `-multi`, and `-image-{arch}` tags in GAR (operational)
- Changing the hallmark format itself

### context-to-pouch-rename (₢A6AAQ) [complete]

**[260414-1333] complete**

## Character

Mechanical refactoring — rename the build context tag from `context-{timestamp}` to `{hallmark}-pouch`, add `RBGC_ARK_SUFFIX_POUCH` constant, and plumb pouch awareness into tally and abjure. Straightforward constant discipline work. Depends on ₢A6AAR (host-side conjure hallmark) so the full hallmark is available at pouch push time.

## Docket

Rename the conjure build context GAR tag from `context-{timestamp}` to `{hallmark}{RBGC_ARK_SUFFIX_POUCH}` so the pouch is hallmark-affiliated and follows the same suffix constant discipline as `-image`, `-about`, `-vouch`, `-diags`.

**Depends on**: ₢A6AAR (host-side-conjure-hallmark) — the full hallmark must be known at pouch push time.

### Add constant

- `Tools/rbk/rbgc_Constants.sh`: add `RBGC_ARK_SUFFIX_POUCH="-pouch"` alongside the existing ark suffix family

### Refactor producer

- `Tools/rbk/rbfd_FoundryDirectorBuild.sh`:
  - Pass the full hallmark (now host-minted after ₢A6AAR) to `zrbfd_push_build_context()`
  - Replace `context-${z_ctx_ts}` tag with `${z_hallmark}${RBGC_ARK_SUFFIX_POUCH}`
  - Delete independent `date -u` timestamp generation (no longer needed)

### Plumb into tally classifier

- `Tools/rbk/rbfl_FoundryLedger.sh` tally tag classifier: add `-pouch` case using `RBGC_ARK_SUFFIX_POUCH` so tally can report pouch presence per hallmark

### Plumb into abjure

- `Tools/rbk/rbfl_FoundryLedger.sh` abjure tag list: add pouch tag to deletion set alongside image/about/vouch/diags

### Spec prose (already done)

RBS0 spec updates — pouch quoin (`rbtga_ark_pouch`), tag inventory table, formal definition with anchor, and all descriptive "pouch" references replaced with quoin — were completed in ₢A6AAT (conjure-pipeline-spec-revision). No spec work remains in this pace.

### Out of scope

- `-multi` suffix constant treatment (done in ₢A6AAR)
- Cleanup of legacy `context-*` tags in GAR (operational, not blocking)
- Cloud build step script changes (GCB receives full tag URI, doesn't parse the name)

### attest-digest-identity-repair (₢A6AAY) [complete]

**[260414-1415] complete**

## Character

Repair pace — fix an architectural misunderstanding surfaced by the first smoke test. The `-attest-{arch}` tags are the only objects carrying GCB provenance; the `-image` manifest's per-platform digests have zero provenance. This is a fundamental property of the classic-store pullback pattern (Docker 20.10 re-serializes manifests on pull/push), not a bug. The tags cost negligible storage (shared layers, ~2KB manifest each) and must be kept as durable artifacts. The vouch verifier must resolve provenance-carrying digests from `-attest-{arch}` tags, not from the `-image` manifest.

Discovered during ₢A6AAS smoke test: conjure succeeded, about succeeded, vouch failed with "No v1.0 envelope for amd64" because the verifier queried provenance on buildx-pushed digests (from `-image` manifest) which have no GCB provenance.

## Docket

Three areas: specification, code, and memo.

### Specification (RBS0-SpecTop.adoc)

1. **Tag inventory table** (~1687): Change `-attest-{arch}` lifecycle from "Ephemeral" to "Durable". Remove "swept post-vouch by ordain pipeline" from Purpose.
2. **Multi-platform pipeline narrative**: Remove sweep references from the attestation scaffolding explanation.
3. **Vouch attestation sweep paragraph** (~1801): Remove entirely — no sweep occurs.
4. **Vouch verification description**: Fix "verifies by digest via the `-image` manifest list" → verifies via `-attest-{arch}` tag digests, which carry GCB provenance.
5. **RBSAA-ark_abjure.adoc**: Add `-attest-{arch}` tag construction and deletion steps (parallel to existing per-suffix patterns).

### Code

1. **Remove sweep function**: Delete `zrbfd_sweep_attest_tags` from `rbfd_FoundryDirectorBuild.sh`.
2. **Remove sweep call**: Remove sweep invocation in `rbfd_ordain` conjure path (after vouch).
3. **Fix vouch verifier** (`rbgjv02-verify-provenance.py`): For conjure mode, resolve per-platform digests from `-attest-{arch}` tags (HEAD request per tag) instead of extracting digests from the `-image` manifest index. Single-platform conjure: resolve `-attest-{arch}` tag directly. Multi-platform: iterate platforms, resolve each `-attest-{arch}` tag.
4. **Add `_RBGV_ARK_SUFFIX_ATTEST`** to vouch substitutions in `rbfv_FoundryVerify.sh` so the verifier script can construct attest tag names.
5. **Update abjure comments**: In `rbfl_FoundryLedger.sh`, change "safety net" language to primary cleanup path for `-attest-{arch}` tags.

### Memo

Add a "Digest Identity Gap" section to `Memos/memo-20260305-provenance-architecture-gap.md` documenting:
- Classic-store pull/push re-serializes manifests → different digest
- GCB provenance attaches to daemon-pushed digests only
- `-attest-{arch}` tags are the provenance-carrying objects
- Decision to keep them as durable (shared layers, ~2KB manifest overhead)
- Reference to smoke test failure (build 090b45c1, "No v1.0 envelope for amd64")

### Out of scope

- Bind/graft vouch paths (different code, not affected)
- Single-platform pipeline changes (same principle applies but separate concern)
- Cleanup of existing `-attest-{arch}` tags in GAR from failed test (operational)

### conjure-pipeline-smoke-test (₢A6AAS) [complete]

**[260414-1501] complete**

## Character

Validation pace — hands-on end-to-end test of the refactored conjure pipeline from ₢A6AAR and ₢A6AAQ. Mechanical execution and observation, not design work. Binary pass/fail against concrete expectations.

## Docket

Conjure a vessel, inspect the tag inventory, summon the image and prove it runs, abjure, and verify cleanup. Confirms that all tag renames (hallmark-minting, `-image` promotion, `-attest-{arch}` ephemerality, `-pouch` affiliation) work together correctly and produce a viable consumer image.

### Test sequence

1. **Conjure**: `tt/rbw-fO.DirectorOrdainsHallmark.sh` against `rbev-busybox` (3-platform, fast build)
2. **Rekon (post-conjure)**: `tt/rbw-ir.DirectorRekonsImages.sh rbev-busybox` — list all tags
3. **Tally (post-conjure)**: `tt/rbw-ft.RetrieverTalliesHallmarks.sh` — verify hallmark health
4. **Summon**: `tt/rbw-fs.RetrieverSummonsHallmark.sh` — pull the consumer `-image` tag locally
5. **Prove viability**: `docker run --rm {summoned-image} echo alive` — confirms the multiplatform manifest resolves to a working image on the host platform
6. **Abjure**: `tt/rbw-fA.DirectorAbjuresHallmark.sh` — remove the hallmark
7. **Rekon (post-abjure)**: list tags again — verify cleanup
8. **Tally (post-abjure)**: verify hallmark gone

### Expected observations

**Post-conjure rekon should show (for this hallmark):**
- `{hallmark}-image` — consumer multiplatform manifest (durable)
- `{hallmark}-about` — SBOM + build info (durable)
- `{hallmark}-vouch` — SLSA verification record (durable)
- `{hallmark}-diags` — diagnostics (durable)
- `{hallmark}-pouch` — build context (durable)
- No `-attest-{arch}` tags (swept by host after vouch)
- No `-multi` tags (eliminated)
- No inscribe-only `{inscribe_ts}-image-{arch}` tags (eliminated)

**Post-conjure tally should show:**
- Hallmark with health state `vouched`

**Summon + run should show:**
- `docker pull` resolves the `-image` manifest list to the host platform
- `echo alive` prints `alive` — image is viable

**Post-abjure rekon should show:**
- None of the above tags remain for this hallmark

**Post-abjure tally should show:**
- Hallmark absent

### Failure modes to watch for

- `-attest-{arch}` tags surviving after ordain (sweep didn't fire)
- Legacy tag patterns appearing (`context-*`, `-multi`, inscribe-only `-image-{arch}`)
- Pouch tag missing (AAQ not plumbed correctly)
- Abjure missing pouch tag (not in deletion set)
- Tally not recognizing new tag scheme
- Summon fails to pull (manifest list broken or digest references invalid)
- Summoned image won't run (buildx output not viable as consumer image)

### Out of scope

- Bind or graft pipeline testing (different code paths)
- Single-platform pipeline (separate concern)
- Performance or timing concerns
- Theurge fixture integration

### ccyolo-vessel-and-nameplate (₢A6AAG) [complete]

**[260413-1757] complete**

## Character

Infrastructure build — new vessel, new nameplate, CCCK retirement. Mechanical once the Dockerfile and nameplate config are settled, but the Dockerfile needs care: npm-friendly base, Claude Code installation, minimal attack surface beyond what Claude Code needs. Debug cycle is kludge→charge→shell-in→verify Claude Code authenticates via OAuth copy/paste. This pace produces the artifact; teaching with it is a separate pace.

## Docket

Create the ccyolo vessel and nameplate for running Claude Code inside a Crucible with network containment scoped to Anthropic API endpoints. Retire CCCK (`Tools/ccck/`).

### Create vessel `rbev-bottle-ccyolo`

New vessel directory under `rbev-vessels/`:

- **Mode**: conjure (has Dockerfile)
- **Base image**: npm-friendly (node:lts-slim or similar — pick the smallest image that gives a working `npm install`)
- **Installed**: Claude Code via npm (`@anthropic-ai/claude-code`), bash, curl, git (Claude Code operational dependencies)
- **NOT installed**: scapy, strace, or other attack tooling (that's ifrit's job)
- **`rbrv.env`**: conjure mode, kludge-oriented (no reliquary or enshrine dependencies needed for first use)

### Create nameplate `ccyolo`

New nameplate directory `.rbk/ccyolo/`:

- **Sentry vessel**: sentry-debian-slim (shared with tadmor)
- **Bottle vessel**: rbev-bottle-ccyolo
- **DNS allowlist**: Anthropic API endpoints (`api.anthropic.com`, and whatever else Claude Code needs for OAuth browser flow)
- **CIDR allowlist**: corresponding IP ranges for the DNS entries
- **Hallmark fields**: blank initially, populated by first kludge

### Verify kludge→charge→authenticate cycle

1. Kludge sentry: `tt/rbw-cKB.KludgeBottle.sh ccyolo` (or kludge both vessels)
2. Charge: `tt/rbw-cC.Charge.ccyolo.sh`
3. Shell into bottle: `tt/rbw-cr.Rack.sh ccyolo`
4. Run `claude` inside the bottle — verify OAuth copy/paste flow works (user copies URL from bottle terminal, pastes in workstation browser, copies code back)
5. Verify Claude Code can reach Anthropic API
6. Verify Claude Code cannot reach anything else (no npm registry, no github, no arbitrary hosts)
7. Quench: `tt/rbw-cQ.Quench.ccyolo.sh`

### Retire CCCK

- Remove `Tools/ccck/` directory (cccw_workbench.sh and any other files)
- Remove CCCK entries from `CLAUDE.md` file acronym mappings
- Remove any CCCK zipper enrollments if they exist
- Update any cross-references

### Out of scope

- Onboarding handbook content using ccyolo (that's the first crucible drafting/implementation paces)
- Cloud Build ordination of ccyolo (kludge-only for now)
- Enshrinement of ccyolo base image
- Aggressive Claude Code permission configuration details (deferred to handbook pace where the learner experience is designed)

### buv-secret-enroll-type (₢A6AAK) [complete]

**[260413-1302] complete**

## Character

Small, mechanical BUK infrastructure addition. Three files, one new type, one conditional. No design ambiguity — the approach is fully specified from the A6AAA assessment.

## Deliverable

Add `buv_secret_enroll` type to the BUK validation/presentation stack so that secret fields (like `RBRA_PRIVATE_KEY`) are validated normally but redacted in render and validate output (terminal and logs).

### Files to change

1. **`Tools/buk/buv_validation.sh`** — Add `buv_secret_enroll()` function, parallel to `buv_string_enroll`. Delegates to `zbuv_enroll` with type `"secret"`. Validation behavior identical to `string` (min/max length).

2. **`Tools/buk/bupr_PresentationRegime.sh`** — In `zbupr_render_field`, add a conditional: if `z_type` is `"secret"`, replace `z_value` with `"(redacted — ${#z_value} chars)"` before rendering. Both single-line and narrow layouts covered by one early substitution.

3. **`Tools/rbk/rbra_regime.sh`** — Change line 45 from `buv_string_enroll RBRA_PRIVATE_KEY ...` to `buv_secret_enroll RBRA_PRIVATE_KEY ...`. No other changes.

### Verification

- `tt/rbw-rar.RenderAuthRegime.sh retriever` — RBRA_PRIVATE_KEY shows `(redacted — N chars)` instead of PEM material
- `tt/rbw-rav.ValidateAuthRegime.sh retriever` — same redaction, validation still passes
- Log files contain redacted output, not key material
- `tt/rbw-tf.QualifyFast.sh` — passes
- `tt/buw-st.BukSelfTest.sh` — passes (no existing test cases exercise secret type, but no regressions)

## Out of Scope

- Adding secret enrollment to any other regime (future paces can adopt it)
- Retroactive log scrubbing (existing logs with leaked keys are a user concern)
- BUS0 spec update for the new type (worth doing but separate)

## References

- `Tools/buk/buv_validation.sh` — enrollment functions, `zbuv_enroll` internal
- `Tools/buk/bupr_PresentationRegime.sh` — `zbupr_render_field` display logic
- `Tools/rbk/rbra_regime.sh` — RBRA field enrollments
- Assessment from ₢A6AAA session ☉260413-1007

### handbook-foundation-implement (₢A6AAB) [abandoned]

**[260409-1234] abandoned**

## Character
First real implementation pace for the handbook heat. Mechanical but high-leverage — this is where we learn whether Frame 4-refined actually works in practice. Depends on Pace `₢A6AAA` (menu-and-crash-course-draft) being reviewed and accepted. The smallest full handbook we can build, which means lessons from this pace inform all subsequent track implementations.

## Docket

Implement the menu tabtarget and the Crash Course handbook from the drafts produced in Pace `₢A6AAA`. This pace produces working code that can be smoke-tested manually and exercised by a (basic, prototype) agent-learner eval.

### Prerequisites
- Pace `₢A6AAA` accepted by user with reviewed drafts
- `₣A3` retired (handbook infrastructure in `rbho_onboarding.sh` available)

### Implement `tt/rbw-h.HandbookStartHere.sh`

Small tabtarget, probe-aware menu routing:
- Dispatch to `rbho_*` function in `Tools/rbk/rbho_onboarding.sh`
- New function: `rbho_start_here` (or similar per-naming-convention)
- Runs the triage probes (BURC present, BURS exists, RBRR populated or empty, RBRA present per-role)
- Renders the full menu with probe-aware highlighting
- Menu text lifted from the Pace `₢A6AAA` draft
- Register in `Tools/rbk/rbz_zipper.sh` with appropriate colophon mapping

Thin dependencies: `buh_handbook.sh`, zipper, constants. No regime, no OAuth. Matches the pattern already established in `rbho_cli.sh` (from Pace `₢A3AAJ`).

### Implement `tt/rbw-Hcc.CrashCourse.sh`

The Crash Course handbook tabtarget:
- Dispatch to new function in `rbho_onboarding.sh`: `rbho_crash_course` (or similar)
- Renders the full Frame 4-refined document from the Pace `₢A6AAA` draft
- Probes embedded inline as designed
- Teaching-only units render their prose; verified units render their prose + probe status lines
- Uses `buh_*` primitives for all display
- Register in `rbz_zipper.sh`

### Probe wiring

Identify and wire any probes the Crash Course references that don't yet exist. Most should already exist (RBRR validate, BURS_LOG_DIR presence) — the crash course is specifically scoped to avoid needing new infrastructure. If a probe is missing, either:
- Reference an existing tabtarget (`tt/rbw-rrv.ValidateRepoRegime.sh`) and let the learner run it manually for status
- Add a minimal probe helper to `rbho_onboarding.sh` scoped to crash-course use

Do NOT build a general probe framework in this pace. Minimum viable wiring only.

### Smoke testing

Manual test cases:
1. **Bare fork simulation**: run `tt/rblm-z.MarshalZeroes.sh` (or equivalent) to produce the invalid state, then run `tt/rbw-h.HandbookStartHere.sh` and `tt/rbw-Hcc.CrashCourse.sh`. Verify menu highlights payor path; verify crash course shows red probes with diagnostic text.
2. **Joining existing simulation**: with populated RBRR, run both tabtargets. Verify menu de-emphasizes payor path; verify crash course shows green probes.
3. **Mid-state**: with some fields populated and others empty, verify the handbook renders a mix of red and green without crashing or rendering oddly.

Document observed output in the pace commit message or a linked artifact.

### Prototype agent-learner eval

Build the minimal eval harness — just enough to validate the concept end-to-end, not a full release gate.

- A simple script or prompt that passes the Crash Course handbook output to a Claude agent with an anti-inference system prompt
- 3–5 scenario questions testing application, not recall
- Differential test: run once with the full handbook, once with teaching prose stripped
- Capture scores and failure modes

This is a prototype. The final eval design is a separate pace. Goal here: prove the loop works.

### Out of Scope
- Full eval harness with question bank, rotation, override mechanism
- First Crucible handbook (Pace `₢A6AAC` drafts, Pace `₢A6AAD` implements)
- Any other track
- Shared snippet mechanism (deferred to when second track needs it)
- Cross-machine or cross-platform validation (defer to A3AAF equivalent later)

### Success criteria
- `tt/rbw-h.HandbookStartHere.sh` runs and prints a probe-aware menu
- `tt/rbw-Hcc.CrashCourse.sh` runs and prints a Frame 4-refined document
- Both tabtargets work in all three smoke-test states without error
- Prototype agent-learner eval runs end-to-end and produces a score
- User reviews and accepts the behavior before pace is wrapped

## References
- Pace `₢A6AAA` drafts (menu + crash course text)
- `₣A6` paddock
- `Tools/rbk/rbho_onboarding.sh` — implementation target
- `Tools/rbk/rbho_cli.sh` — cli pattern to follow
- `Tools/buk/buh_handbook.sh` — display primitives
- `Tools/rbk/rbz_zipper.sh` — colophon registration
- `Tools/rbk/rblm_cli.sh` `rblm_zero()` — produces the bare-fork simulation state

### first-crucible-draft (₢A6AAC) [complete]

**[260414-0723] complete**

## Character
Second drafting pace, informed by real implementation lessons from ₢A6AAA and by the concrete infrastructure built in ₢A6AAG. By the time this pace runs, we have a working ccyolo vessel+nameplate and a verified kludge→charge→SSH cycle to draft against. Harder than the Crash Course draft because a learner track teaches a full role-intent, not just the foundation.

## Docket

Produce a draft of the "Start a Crucible Using Local Builds" handbook (`tt/rbw-Ofc.OnboardingFirstCrucible.sh`) at Frame 4-refined fidelity, ready for implementation review.

### Prerequisites
- ₢A6AAA complete and accepted (foundation: StartHere menu, Crash Course, credential install tabtargets)
- ₢A6AAG complete (ccyolo vessel `rbev-bottle-ccyolo` and nameplate `.rbk/ccyolo/rbrn.env` exist; kludge→charge→SSH cycle verified)
- Crash Course working in real bash code, informing this draft's structure

### Target learner
The **crucible explorer**: local standalone crucibles with kludged hallmarks, fast iteration for security exploration, no cloud. Zero registry, zero cloud — fully self-contained on the developer's workstation. This is the simplest complete learner path and the fastest feedback loop for the pedagogy heat.

### Draft the handbook

Full Frame 4-refined document covering the concrete step sequence established in the StartHere menu:

**Step sequence** (from the StartHere menu bullets, now the canonical procedure shape):
1. **Build images locally** — Kludge Sentry and Bottle vessels
2. **Configure local network** — Amend the ccyolo Nameplate RBRN file with hallmarks
3. **Start the sandbox** — Charge the Crucible
4. **Enter the container via SSH** — Connect to the Bottle, run Claude Code, authenticate via OAuth copy/paste flow
5. **Verify network containment** — Anthropic reachable, example.com reachable, everything else blocked
6. **Quench** — clean shutdown

### Operational lessons from ₢A6AAG (teach these)

The following were discovered and verified during the ccyolo build. The handbook should teach them as first-class content, not footnotes.

**Two kludge commands with different roles:**
- `tt/rbw-fk.LocalKludge.sh rbev-sentry-debian-slim` — standalone vessel kludge. Produces a hallmark but does NOT drive it into any nameplate. The sentry image is shared across all nameplates, so kludging it is rare (only when sentry Dockerfile changes). The handbook teaches the manual sentry kludge once to show the full process.
- `tt/rbw-cKB.KludgeBottle.sh ccyolo` — nameplate-aware bottle kludge. Builds the bottle vessel AND drives the hallmark into the nameplate's `rbrn.env` automatically. This is the common iteration loop — edit Dockerfile, notch, kludge, charge, test.

**Manual hallmark drive for sentry:**
After `rbw-fk` kludges the sentry, the learner must manually edit the nameplate to install the hallmark. The handbook should include a copy/paste command:
```
vi .rbk/ccyolo/rbrn.env
```
Edit `RBRN_SENTRY_HALLMARK=` to the hallmark printed by the kludge output. This is deliberately manual — it teaches what hallmarks are and where they live. The bottle kludge automates this same step, so the learner sees both the manual and automated paths.

**Clean git tree required for kludge:**
Kludge refuses to run with uncommitted changes (`git diff --quiet` guard). The handbook must teach the notch-before-kludge discipline. Include copy/paste git commands for continuity:
```
git add .rbk/ccyolo/rbrn.env
git commit -m "Drive sentry hallmark into ccyolo nameplate"
```
(Or equivalent jjx_record if within a heat context.)

**SSH entry, not docker exec:**
The handbook teaches SSH via `tt/rbw-cS.SshTo.ccyolo.sh` as the primary entry method. Rack (`tt/rbw-cr.Rack.sh ccyolo`) is mentioned only as a diagnostic fallback. The *why* is a teachable moment: `docker exec -it` allocates a pseudo-TTY but does not propagate SIGWINCH (terminal window resize). This is a long-standing Docker limitation (moby/moby#35407, docker/cli#3554). Claude Code's interactive TUI depends on correct terminal dimensions — resizing the window after `docker exec` breaks the display. SSH provides a proper login session with full terminal negotiation, SIGWINCH propagation, and correct environment setup.

**Network containment verification with example.com:**
The ccyolo nameplate includes `example.com` in both `RBRN_UPLINK_ALLOWED_DOMAINS` and `RBRN_UPLINK_ALLOWED_CIDRS` as a deliberate test target. This lets the learner verify that the DNS/CIDR allowlist works for a non-Anthropic domain. The verification step should show:
- `api.anthropic.com` → reachable (404 expected — API wants auth, not GET /)
- `claude.ai` → reachable (403 expected — web app)
- `example.com` → reachable (200)
- `google.com` → blocked (000 / timeout)
- `registry.npmjs.org` → blocked (000 / timeout)

This demonstrates both the allowlist and the blocklist in one pass. The comment in `rbrn.env` explains example.com's test purpose.

### Learning outcomes

After completion, the learner can:
- Kludge a local hallmark for sentry (manual process) and bottle (automated) vessels
- Understand the three-container Crucible composition (sentry + pentacle + bottle) and what each does
- Edit RBRN networking fields and drive hallmarks into a nameplate
- Charge a crucible from a nameplate
- SSH into the bottle and run Claude Code inside the sandbox
- Verify network containment (allowed domains reachable, everything else blocked)
- Quench the crucible cleanly
- Iterate: edit Dockerfile → notch → kludge bottle → notch hallmark → charge → SSH → test

### Key concepts to teach (vocabulary-first, relational)
- Crucible — the composition of sentry + pentacle + bottle containers
- Nameplate / RBRN — the file that defines a specific deployment target, including networking
- Kludge mode — local hallmark synthesis without cloud build
- Hallmark — the version tag that connects a nameplate to a specific built image
- Charge / quench — lifecycle verbs for the crucible
- Rack — docker exec into a running bottle (diagnostic fallback)
- SSH entry — proper terminal access via sentry DNAT port forwarding
- Sentry filtering — how network access is controlled (iptables + dnsmasq allowlist)
- The local-vs-cloud distinction and why this track never touches cloud

### Explicit non-goals
- Vouching (cloud-only concept for this learner)
- Enshrinement or reliquaries
- Ordain modes other than kludge
- SA credentials or RBRA placement
- Any GAR interaction

Draft as actual handbook output — the exact text a learner would see — wrapped in pseudo-bash indicating where `buh_*` calls would produce each line.

### Identify infrastructure requirements

List any probes, tabtargets, or helper functions that don't exist and need to be built in ₢A6AAD. Examples:
- Is there a probe for "crucible is currently charged"? (`rbw-cic.CrucibleIsCharged` exists — verify suitability)
- Is there a probe for "nameplate X exists"?
- Is there a probe for "kludged hallmark is present for nameplate"?
- Is there a probe for "port conflict with other charged crucibles"?

Produce an inventory: "probes existing", "probes needed", "tabtargets needed".

### Flag cross-track shared snippets

If teaching content would naturally appear in multiple tracks (e.g., "what is a vessel," "what is a depot," "how RBRN networking works"), flag as candidates. Do NOT design the mechanism — just note candidates.

### Voice check

Teaching voice should match the crucible learner's energy: exploratory, fast-iteration, slightly irreverent. The learner is here to play with containers, not to set up production infrastructure.

### Explicit assumptions
Mark everything made up without user input. Key areas:
- Unit ordering
- Which concepts need their own unit vs being folded into another
- Whether the RBRN editing step is a verified unit or teaching-only
- How wait-time during charge is handled pedagogically

### Out of Scope
- Actual implementation (₢A6AAD)
- Agent-learner eval design beyond what ₢A6AAA prototyped
- Other learner tracks
- Shared snippet mechanism design
- Creating the ccyolo vessel/nameplate (₢A6AAG's job)

## References
- ₢A6AAA outcomes (StartHere menu step bullets, Crash Course structure, `buh_*` patterns)
- ₢A6AAG outcomes (ccyolo vessel, nameplate, verified kludge→charge→SSH cycle, two spook fixes in rbob_cli.sh furnish)
- `Tools/rbk/rbho_onboarding.sh` — `rbho_start_here()` step bullets as canonical procedure shape
- `Tools/rbk/vov_veiled/RBSCC-crucible_charge.adoc` — crucible charge spec
- `Tools/rbk/vov_veiled/RBSCN-crucible_enjoin.adoc` — crucible enjoin spec
- `Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc` — kludge operation spec
- `.rbk/tadmor/rbrn.env`, `.rbk/pluml/rbrn.env` — existing RBRN examples showing networking field patterns
- `.rbk/ccyolo/rbrn.env` — the nameplate this handbook teaches (from ₢A6AAG)
- `rbev-vessels/rbev-bottle-ccyolo/Dockerfile` — the bottle image this handbook teaches (from ₢A6AAG)
- `tt/rbw-cS.SshTo.ccyolo.sh` — SSH entry tabtarget (from ₢A6AAG)
- moby/moby#35407, docker/cli#3554 — docker exec SIGWINCH limitation (why SSH, not Rack)
- ₣A6 paddock

### ccyolo-yolo-mode-and-sample-content (₢A6AAD) [complete]

**[260414-0838] complete**

## Character
Completing the ccyolo experience. ₢A6AAC delivered a working handbook and probes, but the bottle is inert — the learner SSHs in and finds a bare shell. This pace delivers the "yolo" in ccyolo: Claude Code preconfigured for full autonomy, sample content to work on, and a volume mount so the learner's work persists across charge/quench cycles. The payoff is visceral — the learner sees an AI agent operating freely inside a container that can only reach Anthropic, and understands why the containment makes that safe.

## Docket

### Prerequisites
- ₢A6AAC complete (handbook, probes, README anchors all working)
- ccyolo vessel and nameplate exist (₢A6AAG)

### Preconfigure Claude Code for yolo mode

The ccyolo Dockerfile or entrypoint should preconfigure Claude Code so the learner doesn't face permission prompts on first run. This is the teaching point: inside a contained crucible, full autonomy is the correct posture.

Investigate how Claude Code's permission/acceptance settings work:
- Is there a config file (`~/.claude/settings.json` or similar) that can be baked into the image?
- Can environment variables control acceptance mode?
- What's the minimal configuration to suppress all tool-use permission prompts?

Implement in the Dockerfile or entrypoint for the claude user. The learner should run `claude` and immediately be in a fully autonomous session.

### Add volume mount with sample content

Create sample project content that gives the learner something meaningful to work with inside the bottle. The volume mount should:
- Persist across charge/quench cycles (Docker volume or bind mount)
- Contain a small, self-contained project the learner can ask Claude to modify
- Be interesting enough to demonstrate Claude Code's capabilities in ~2 minutes

Candidates for sample content:
- A tiny bash script with a deliberate bug, plus a README asking Claude to fix it
- A small Python/JS project with a TODO list
- A markdown document asking Claude to restructure it

The volume mount configuration lives in the nameplate's compose definition. Investigate how `rbob_charge` composes containers and where volume mounts are declared.

### Update the handbook

Update `rbho_first_crucible()` Step 4 (Enter the container) to reflect:
- Claude Code starts in yolo mode — no permission prompts
- Sample content is available at a known path
- Suggest a first interaction ("ask Claude to fix the bug in the sample project")

### Smoke test the full flow

Walk through the handbook end-to-end:
1. Kludge sentry + bottle (with new yolo config and sample content)
2. Drive hallmarks, charge
3. SSH in, run `claude`
4. Verify: no permission prompts, sample content visible, Claude can modify files
5. Verify: network containment still holds (Anthropic reachable, google.com blocked)
6. Quench, re-charge, verify sample content persists (if volume-mounted)

### Out of Scope
- Agent-learner eval (defer to dedicated eval pace)
- Other learner tracks
- Shared snippet mechanism

## References
- ₢A6AAC outcomes (working handbook, probes, README anchors)
- ₢A6AAG outcomes (ccyolo vessel, nameplate, Dockerfile, entrypoint)
- `rbev-vessels/rbev-bottle-ccyolo/Dockerfile` — where yolo config goes
- `rbev-vessels/rbev-bottle-ccyolo/entrypoint.sh` — runtime setup
- `.rbk/ccyolo/rbrn.env` — nameplate
- Claude Code documentation for permission/settings configuration

### foundry-image-credential-refactor (₢A6AAP) [complete]

**[260414-0837] complete**

## Character

Small infrastructure refactor with spec changes. Clarifies the credential boundaries between retriever and director for foundry/image operations. Mechanical in code (credential variable swaps), but requires spec updates in RBS0 and the relevant operation adoc files.

## Docket

Three changes to the foundry/image credential model:

### 1. Add `rbw-ir` Rekon — director-only image-level listing

New image-group operation: raw GAR tag listing authenticated as director. Colophon `rbw-ir` (lowercase — read-only). This is the director's credential smoke test.

- Add function in `rbfl_FoundryLedger.sh` (or appropriate file)
- Add zipper enrollment in `rbz_zipper.sh` under `RBZ__GROUP_IMAGE`
- Add thin dispatcher `tt/rbw-ir.DirectorRekonsImages.sh`
- Add operation spec (new adoc file, included from RBS0)
- Run `tt/rbw-MG.MarshalGenerate.sh`

### 2. Move `rbw-iw` Wrest to director credentials

Change `rbfr_wrest()` to authenticate via `RBDC_DIRECTOR_RBRA_FILE` instead of `RBDC_RETRIEVER_RBRA_FILE`. Directors are also retrievers — no capability loss. Wrest becomes a director power operation (raw image pull by arbitrary tag).

- Update `Tools/rbk/rbfr_FoundryRetriever.sh` (or relocate function to director/ledger file)
- Update spec `RBSIW-image_wrest.adoc` to reflect director credential
- Update RBS0 if the operation's role attribution changes

### 3. Move `rbw-ft` Tally to retriever credentials

Change `rbfl_tally()` to authenticate via `RBDC_RETRIEVER_RBRA_FILE` instead of `RBDC_DIRECTOR_RBRA_FILE`. Tally becomes the retriever's credential smoke test — hallmark-level inventory with health status.

- Update `Tools/rbk/rbfl_FoundryLedger.sh` line 628
- Update spec `RBSCL-consecration_tally.adoc` to reflect retriever credential
- Update RBS0 role attribution (currently says "The {rbtr_director} tallies hallmarks")

### Result

| Group | Command | Verb | Role | Operation |
|-------|---------|------|------|-----------|
| foundry | `rbw-ft` | Tally | retriever | Hallmark inventory with health (credential check) |
| foundry | `rbw-fs` | Summon | retriever | Pull vouched hallmark unit |
| foundry | `rbw-fp*` | Plumb | retriever | Read provenance |
| image | `rbw-ir` | Rekon | director | Raw GAR tag listing (credential check) |
| image | `rbw-iw` | Wrest | director | Pull single image by tag |
| image | `rbw-iJ` | Jettison | director | Delete single image tag |

Image group is entirely director-only. Tally is the retriever credential check. Rekon is the director credential check.

### Verification

- Run `tt/rbw-ft.DirectorTalliesHallmarks.sh` with retriever credentials — confirm it works
- Run `tt/rbw-ir.DirectorRekonsImages.sh` with director credentials — confirm it works
- Run `tt/rbw-iw.RetrieverWrestsImage.sh` with director credentials — confirm it works
- Run fast qualify: `tt/rbw-tf.QualifyFast.sh`

### Out of scope

- Onboarding track content (₢A6AAN, ₢A6AAO)
- Dual-credential fallback logic
- Changes to summon, plumb, ordain, vouch, abjure, kludge, or jettison

### onboarding-credential-retriever (₢A6AAN) [complete]

**[260415-0624] complete**

## Character

Small handbook track implementation. Add the retriever credential installation onboarding sequence — place RBRA file, verify, confirm access via hallmark tally (`rbw-ft`). Only relevant if you are not the Payor. Depends on ₢A6AAP having moved tally to retriever credentials.

## Docket

Implement `tt/rbw-Ocr.OnboardingCredentialRetriever.sh` — the retriever credential installation handbook track.

### Teaching content

- Place your RBRA credential file (retriever role)
- Verify the credential is recognized
- Confirm access via `tt/rbw-ft.DirectorTalliesHallmarks.sh` (tally — hallmark inventory, now retriever-authenticated after ₢A6AAP)
- Clarify this track is only needed if you are not the Payor

### Implementation

- Add zipper enrollment, thin dispatcher, sequence function following the Crash Course pattern

### Out of scope

- Director credential installation (₢A6AAO)
- Payor credential flow
- Credential refactoring (₢A6AAP)

### onboarding-credential-director (₢A6AAO) [complete]

**[260415-0631] complete**

## Character

Small handbook track implementation. Add the director credential installation onboarding sequence — place RBRA file, verify, confirm access via image rekon (`rbw-ir`). Only relevant if you are not the Payor. Depends on ₢A6AAP having created the rekon operation.

## Docket

Implement `tt/rbw-Ocd.OnboardingCredentialDirector.sh` — the director credential installation handbook track.

### Teaching content

- Place your RBRA credential file (director role)
- Verify the credential is recognized
- Confirm access via `tt/rbw-ir.DirectorRekonsImages.sh` (rekon — raw GAR tag listing, director-authenticated)
- Clarify this track is only needed if you are not the Payor

### Implementation

- Add zipper enrollment, thin dispatcher, sequence function following the Crash Course pattern

### Out of scope

- Retriever credential installation (₢A6AAN)
- Payor credential flow
- Credential refactoring (₢A6AAP)

### onboarding-director-airgap (₢A6AAV) [complete]

**[260417-0929] complete**

## Character

Infrastructure-only pace: create the airgap vessel chain (forge, airgap-ifrit, moriah nameplate, tabtargets, theurge fixture). Mechanical with one validation gate. Handbook content is a separate pace.

**Depends on**: `₢A6AAU` (onboarding-director-first-cloud-build) — reliquary inscribed, learner has completed a tethered build. `ifrit-symmetric-rename` — `rbev-bottle-ifrit-tether` established as tether baseline with `common-ifrit-context/` in place. `₢A6AAI` — design recorded.

## Docket

Create the vessel, nameplate, and fixture infrastructure for the airgap ifrit chain. Handbook content deferred to a dedicated pace.

### Forge vessel (`rbev-bottle-ifrit-forge`)

- `rbev-vessels/common-ifrit-forge-context/Dockerfile`: FROM rust:slim-bookworm, apt-get install network tools, dummy cargo build to warm cache
- `rbev-vessels/common-ifrit-forge-context/Cargo.toml` + `Cargo.lock`: copied from common-ifrit-context, kept in sync
- `rbev-vessels/rbev-bottle-ifrit-forge/rbrv.env`: conjure mode, tether egress, points to common-ifrit-forge-context
- Enshrine `rust:slim-bookworm` if not already present
- Conjure forge tethered, enshrine forge image — anchor goes into airgap-ifrit's rbrv.env

### Airgap-ifrit vessel (`rbev-bottle-ifrit-airgap`)

- `rbev-vessels/common-ifrit-context/Dockerfile.airgap`: FROM enshrined forge, COPY src + manifests, cargo build, setcap, useradd ifrit, CMD socat
- `rbev-vessels/rbev-bottle-ifrit-airgap/rbrv.env`: conjure mode, airgap egress, IMAGE_1_ORIGIN points to forge, IMAGE_1_ANCHOR populated after forge enshrinement

### Moriah nameplate

- `.rbk/moriah/rbrn.env`: tether sentry (not airgap — sentry is simple enough), airgap bottle, 10.242.4.0/24 enclave, uplink mirrors tadmor for security suite identity, PENDING-ordination hallmarks

### Tabtargets

- Crucible: Charge, Quench, SshTo, Writ, Fiat, Bark, IfritSortie, IfritClient (mechanical clones from tadmor)
- Theurge: Run.moriah, SingleCase.moriah (NOT added to SUITE_CRUCIBLE)

### Validator fix

- `rbrn_regime.sh`: enclave port uniqueness keyed on base_ip:port (network-scoped, not global)

### Validation gate

Forge enshrined; airgap-ifrit conjured and vouched on airgap pool; moriah charges; `tt/rbtd-r.Run.moriah.sh` passes the security suite identically to tadmor.

### Out of scope

- Handbook content (`tt/rbw-Oda.OnboardingDirectorAirgap.sh`, zipper enrollment, teaching arc) — separate pace
- Reliquary inscription (`₢A6AAU`)
- Bind or graft modes (`₢A6AAW`, `₢A6AAX`)
- Migrating tadmor away from tether variant
- Adding moriah to `ZRBTE_SUITE_CRUCIBLE`

### airgap-handbook-draft (₢A6AA1) [complete]

**[260423-0934] complete**

## Character

Voice-driven pedagogical writing, modeled on `rbhodf_director_first_build.sh` and `rbhofc_first_crucible.sh`. The infrastructure (forge vessel, airgap-ifrit vessel, moriah nameplate, tabtargets, theurge fixture) is landed by `₢A6AAV`. This pace writes the teaching content.

**Depends on**: `₢A6AAV` (infrastructure landed), `₢A6AAU` (first cloud build handbook finalized — the airgap track is the next step in the director learning sequence).

## Docket

Implement `tt/rbw-Oda.OnboardingDirectorAirgap.sh` — the airgap cloud build handbook track.

### Teaching arc (5 units, Frame 4-refined)

1. **Enshrine upstream base** — `rust:slim-bookworm` for the forge. Teach: airgap requires a pre-staged supply chain.
2. **Conjure forge tethered, then enshrine** — build a project-authored base image that pre-stages apt packages and warms the cargo cache. Teach: forge as customer-controlled toolchain image; layered enshrinement.
3. **Conjure airgap-ifrit airgapped from enshrined forge** — zero network during the airgap build. Teach: customer code never exposed to internet during build.
4. **Charge moriah, run security suite** — drive the cloud-built airgap hallmark into the moriah nameplate, run `tt/rbtd-r.Run.moriah.sh`, watch the same attack tests pass against the airgap-built bottle. Teach: the learner has built validated security infrastructure end-to-end.
5. **Compare Plumb output** — tethered ifrit (tadmor) vs airgap ifrit (moriah) side-by-side. Teach: functionally equivalent images, distinguished by supply-chain properties; both carry SLSA provenance.

### Implementation

- Zipper enrollment in `rbz_zipper.sh` — colophon `rbw-Oda`, group `RBZ__GROUP_ONBOARDING`
- Thin dispatcher tabtarget `tt/rbw-Oda.OnboardingDirectorAirgap.sh` (nolog launcher pattern)
- Sequence function `rbho_director_airgap()` in `Tools/rbk/rbh0/rbhoda_director_airgap.sh`
- Teaching content using `buh_*` primitives, vocabulary-first via `buh_tlt`
- Probes: reliquary inscribed, rust:slim-bookworm enshrined, forge hallmark exists, airgap ifrit vouched, moriah chargeable, tethered ifrit present (for plumb comparison)

### Model

Follow the patterns in `rbhodf_director_first_build.sh` (probe structure, teaching unit layout, `buc_doc_brief`/`buc_doc_shown` gate, yelp capture idiom) and `rbhofc_first_crucible.sh` (crucible-oriented teaching).

### Out of scope

- Infrastructure creation (₢A6AAV)
- Reliquary inscription (₢A6AAU)
- Bind or graft modes (₢A6AAW, ₢A6AAX)

### review-onboarding-director-airgap (₢A6AAx) [abandoned]

**[260705-1031] abandoned**

## Character

Interactive debug pace. No pre-written docket — work proceeds by running the newly-landed airgap onboarding track, observing output with the user, and iterating on issues as they surface. Mirrors `₢A6AAu`'s relationship to the first-kludge track: implementation lands in `₢A6AA1`, then this pace absorbs user review feedback. Cognitive posture: responsive, hands-on, driven by user judgment at each step.

## Docket

Re-review the airgap onboarding handbook track produced by `₢A6AA1`. Interactive debug until the user is satisfied.

### Requires

- `₢A6AA1` wrapped — handbook track `rbw-Oda.OnboardingDirectorAirgap.sh` landed, sequence `rbho_director_airgap()` in `rbhoda_director_airgap.sh`, zipper entry under `RBZ__GROUP_ONBOARDING`, start-here menu entry, all five teaching units rendering.
- Infrastructure from `₢A6AAV` already in place — forge vessel, airgap-ifrit vessel, moriah nameplate, 10 moriah tabtargets, theurge fixture, enclave validator fix.

### Out of scope

- Vessel / nameplate / fixture creation (handled by `₢A6AAV` Phase 1)
- Teaching arc authoring (handled by `₢A6AA1`)
- Moriah docs/anchor introduction (handled by `₢A6AAw`)
- Final docs hygiene sweep (handled by `₢A6AAF`)

### onboarding-director-first-cloud-build (₢A6AAU) [abandoned]

**[260705-1031] abandoned**

## Character

Refinement pace — the initial implementation landed in prior commits (zipper enrollment, thin dispatcher, `rbho_director_first_build()` with 5-step conjure lifecycle teaching arc). What remains is content refinement informed by the ₢A6AAY durability fix and user review feedback.

## Docket

### Content corrections

1. **Attest tag durability**: The teaching content was written when `-attest-{arch}` tags were considered ephemeral scaffolding. ₢A6AAY established they are **durable** provenance-carrying artifacts — the only objects with GCB-attested digests. Update teaching prose in `rbho_director_first_build()` to reflect:
   - `-attest-{arch}` tags persist alongside `-image`, `-about`, `-vouch`, `-pouch`, `-diags`
   - They carry the GCB-attested digests (classic Docker image store re-serializes manifests, producing different digests than buildx-native)
   - Abjure deletes them alongside all other ark artifacts
   - The tag inventory tour (step 3) should mention them as durable, not skip them as ephemeral

2. **Smoke test docket update**: ₢A6AAS's docket still says "No `-attest-{arch}` tags (swept by host after vouch)" in expected observations. This is now wrong — post-conjure rekon SHOULD show `-attest-{arch}` tags as durable artifacts.

### User revision feedback

Capture and implement revision notes from user review of the handbook output. User confirmed the tabtarget functions but wants content revisions — specifics to be gathered at mount time.

### Probes

- Reliquary inscribed (monotonic)
- Director RBRA present and valid
- Depot provisioned (RBRR populated)
- Latest hallmark health state

### Requires

- ₢A6AAY wrapped (attest durability fix) ✓
- Director credentials and a provisioned Depot

### Out of scope

- Airgap builds (₢A6AAV)
- Bind or graft modes
- Credential installation (₢A6AAO)
- Pipeline refactoring (already done)
- Greenfield implementation (already landed)

### review-onboarding-director-first-kludge (₢A6AAu) [abandoned]

**[260705-1031] abandoned**

## Character

Interactive debug pace. No pre-written docket — work proceeds by running the existing First Kludge onboarding track, observing output with the user, and iterating on issues as they surface. Cognitive posture: responsive, hands-on, driven by user judgment at each step.

## Docket

Re-review the existing onboarding-director-first-kludge handbook track. Interactive debug until the user is satisfied.

### onboarding-director-bind (₢A6AAW) [complete]

**[260705-1028] complete**

## Character

Handbook track implementation — bind mode: pin an upstream image by digest with zero build. Pattern replication from `tt/rbw-Ofc.OnboardingFirstCrucible.sh`. Teaches the simplest ordain mode — no Dockerfile, no build context, no pouch. The learner sees a different vouch verdict (digest-pin, no SLSA) and understands why.

The arc opens with a sentry-kludge step (or a callout to reuse an already-conjured Sentry hallmark from `₢A6AAU`) so the track is self-contained — the learner can complete the bind lesson without having completed First Cloud Build first. The pedagogy explicitly names the mode mixture (kludged Sentry, bound Bottle) inside the single Crucible so learners read it as intentional, not as an oversight.

Uses `rbev-bottle-plantuml` as the teaching Bottle — a real upstream image that renders diagrams but whose Docker Hub source could phone home. Bind pins it; the Sentry blocks all egress.

## Docket

Implement `tt/rbw-Odb.OnboardingDirectorBind.sh` — the bind mode handbook track.

### Teaching arc

0. **Ready a Sentry** — kludge the Sentry locally (primary path, matches the `Ofc` track's pattern the learner already met), or reuse the Cloud-Built Sentry hallmark from First Cloud Build (`₢A6AAU`) if that track has been completed. Teach: the Sentry enforces network policy regardless of its own provenance chain — the policy is load-bearing, not the build mode behind it. A kludged Sentry is fine for learning bind; a Conjured Sentry is fine too; what matters for the bind lesson is what happens to the *Bottle* image.
1. **Bind PlantUML** — pin upstream image by digest into GAR. No Dockerfile in the project, no build. Teach: bind mode mirrors an upstream image by digest, the director trusts the upstream publisher, the depot gets an immutable copy. Contrast with conjure (project builds the image) and graft (local machine builds the image).
2. **Inspect Vouch verdict** — digest-pin, no SLSA. Teach: SLSA provenance requires Cloud Build — bind bypasses Cloud Build entirely, so the vouch verdict honestly reports "digest-pin." The image is exactly what upstream published, pinned by content hash, but the project didn't build it.
3. **Charge the pluml Crucible** — render a diagram, observe blocked egress. Teach: the Sentry enforces network isolation regardless of image provenance. The PlantUML image might phone home if run naked; inside the Crucible it cannot. You get the tool without the risk.

### Mode mixture — name it explicitly

The pluml Crucible in this track pairs a (kludged or conjured) Sentry with a bound PlantUML Bottle. Two different ordain modes cohabiting in one Crucible is the expected shape, not a defect. Teaching prose should say so directly — each Hallmark carries its own Vouch verdict, and a Nameplate can pin Hallmarks of different modes without conflict. This reinforces the trust-hierarchy lesson rather than undermining it.

### Probes

- Director RBRA present and valid
- Depot provisioned
- Sentry hallmark available (kludged locally or conjured in a prior track)
- PlantUML hallmark exists and is vouched (digest-pin)
- pluml Crucible chargeable

### Implementation

- Zipper enrollment in `rbz_zipper.sh` (colophon `rbw-Odb`, group `RBZ__GROUP_ONBOARDING`), positioned alongside `rbw-Odf` and `rbw-Oda` in the Director Subtracks section
- Thin dispatcher tabtarget `tt/rbw-Odb.OnboardingDirectorBind.sh` using the nolog launcher pattern (match the `Oda` dispatcher)
- Sequence function `rbho_director_bind()` in a new `rbhodb_director_bind.sh`, following the post-decomposition pattern of `rbhodf_director_first_build.sh` and `rbhoda_director_airgap.sh`
- Teaching content using `buh_*` primitives and `RBYC_*` linked terms throughout (ORDAIN/BIND/CHARGE, SENTRY/BOTTLE/CRUCIBLE/HALLMARK/NAMEPLATE/VESSEL, KLUDGED/CONJURED/BOUND)
- CLI furnish in `rbho0_cli.sh` to source the new helper
- Regenerate tabtarget context via `tt/rbw-MG.MarshalGenerate.sh`
- Start-here menu entry in `rbho0_start_here.sh` under Director Subtracks — parallel entry to `rbw-Odf` and `rbw-Oda`, briefing the bind mode + mode-mixture pedagogy, five-bullet Steps list mirroring the 0-3 teaching arc, Requires line naming "Your First Crucible (kludged Sentry available)" as prerequisite with "Your First Cloud Build completed" as the acceptable alternative

### Requires

- Director credentials, provisioned Depot
- `rbev-bottle-plantuml` Vessel and `pluml` Nameplate (already shipped)
- Sentry Vessel (already shipped — `rbev-sentry-deb-tether` / `rbev-sentry-deb-airgap`); the track teaches kludging one, no new Vessel work

### Out of scope

- Conjure or graft modes (own tracks — `₢A6AAU`, `₢A6AAX`)
- Reliquary inscription (bind doesn't use Cloud Build)
- PlantUML Vessel creation (already exists)
- Sentry Vessel creation (already exists — the track kludges the shipped Vessel)

### graft-demo-vessel (₢A6AAH) [complete]

**[260423-1038] complete**

## Character

Small infrastructure build. Create a minimal graft-mode vessel for use in the graft onboarding subtrack (₢A6AAX) and Assay: Supply Chain (track 13). Not a bottle — no nameplate, no crucible integration, no Dockerfile in the project. Just an `rbrv.env` declaring graft mode so ordain knows what to do.

## Docket

Create `rbev-vessels/rbev-graft-demo/` with graft-mode configuration.

### Create vessel directory

- `rbrv.env` with `RBRV_VESSEL_MODE=graft` and minimal configuration
- No Dockerfile (graft vessels have no project-side Dockerfile — the image is built locally by the user)
- Comment in rbrv.env noting this vessel is a teaching target for graft mode
- The learner grafts a busybox image — minimal, no distraction from the vouch verdict lesson

### Naming

`rbev-graft-demo` — NOT `rbev-bottle-*`. This vessel never runs in a crucible. No bottle, no nameplate, no sentry.

### Verify graft cycle

1. Build or pull a busybox image locally
2. Graft it via ordain
3. Verify the GRAFTED verdict from vouch
4. Verify tally shows the grafted hallmark

### Out of scope

- Nameplate or crucible integration
- Cloud Build ordination (graft bypasses Cloud Build)
- Onboarding handbook content (₢A6AAX)
- Vessel creation pedagogy (the learner finds this vessel preconfigured)

### onboarding-director-graft (₢A6AAX) [complete]

**[260423-1055] complete**

## Character

Handbook track implementation — graft mode: push a locally-built image to the depot. Pattern replication from `tt/rbw-Ofc.OnboardingFirstCrucible.sh`. Teaches the least-trusted ordain mode — the user owns the entire build, SLSA cannot vouch for this image. Vouch verdict is GRAFTED: an explicit signal that provenance stops at the local machine.

Uses `rbev-graft-demo` (created by ₢A6AAH) as the teaching vessel.

## Docket

Implement `tt/rbw-Odg.OnboardingDirectorGraft.sh` — the graft mode handbook track.

### Teaching arc

1. **Kludge a busybox image locally** — the graft-demo vessel is preconfigured with `RBRV_VESSEL_MODE=graft`. The learner grafts a busybox image — trivially simple so the focus stays on the trust model, not the image contents.
2. **Graft** — push local image to the depot. Teach: graft pushes a locally-built image to GAR. The director takes responsibility for what's in the image. No Cloud Build involvement, no pouch, no SBOM from the project's pipeline.
3. **Inspect Vouch verdict** — GRAFTED, no provenance chain. Teach: the vouch verdict honestly reports "GRAFTED." SLSA provenance requires Cloud Build; graft bypasses it entirely. The verdict is not a failure — it's an explicit signal about trust level. Compare with conjure (full SLSA) and bind (digest-pin). The trust hierarchy across modes is the central lesson.

### Probes

- Director RBRA present and valid
- Depot provisioned
- graft-demo vessel exists (`rbev-graft-demo/rbrv.env` with `RBRV_VESSEL_MODE=graft`)
- Grafted hallmark exists and is vouched (GRAFTED)

### Implementation

- Zipper enrollment in `rbz_zipper.sh` (colophon `rbw-Odg`, group `RBZ__GROUP_ONBOARDING`)
- Thin dispatcher tabtarget `tt/rbw-Odg.OnboardingDirectorGraft.sh`
- Sequence function `rbho_director_graft()` in `rbho_onboarding.sh`
- Teaching content using `buh_*` primitives

### Requires

- Director credentials, provisioned depot
- ₢A6AAH (graft-demo-vessel) must be wrapped — provides `rbev-graft-demo/`

### Out of scope

- Conjure or bind modes
- Graft-demo vessel creation (₢A6AAH)
- Nameplate or crucible integration for graft-demo (no nameplate by design)

### plumb-goes-remote-with-vouches-superdirectory (₢A6AAZ) [abandoned]

**[260705-1032] abandoned**

## Character
Design + implementation — new GAR structural concept (VOUCHES superdirectory), rewrite plumb from local-only to GAR-direct, update vouch to populate the new namespace.

## Docket

### Problem
Plumb currently requires `summon` (local pull) before it can display provenance. This is unnecessarily heavy for a read-only inspection. Additionally, there is no way to reverse-map a hallmark to its vessel without iterating all vessel packages.

### Design

Introduce a VOUCHES superdirectory in GAR — a single package where every vouch artifact is additionally tagged. Since GAR is content-addressable, this is a tag pointer, not data duplication.

- **`RBGC_VOUCHES_PACKAGE`** — new constant in `rbgc_Constants.sh` alongside the `RBGC_ARK_SUFFIX_*` family
- Vouch step additionally tags into `VOUCHES/{hallmark}-vouch` after pushing to `{vessel}:{hallmark}-vouch`
- Plumb queries GAR directly via Retriever credentials (curl), never touches local docker

### Plumb conversion

- Remove all `docker image inspect` / `docker create` / `docker cp` logic
- Authenticate as Retriever (new requirement — plumb was previously unauthenticated)
- Fetch `-about` and `-vouch` manifests via Registry API, extract JSON layers via blob API
- New hallmark-only mode: look up in VOUCHES, read `vouch_summary.json` to discover vessel, then fetch about from the vessel package
- Vessel+hallmark mode still works (skips VOUCHES lookup)

### Vouch amendment

- After pushing vouch artifact to `{vessel}:{hallmark}-vouch`, additionally tag into VOUCHES package
- This is a registry-side tag operation (no re-push of blobs)

### Spec documents to update

| Document | Changes |
|----------|---------|
| **RBSAP** (ark_plumb) | Complete rewrite: GAR-direct, Retriever auth, hallmark-only mode via VOUCHES |
| **RBSAV** (ark_vouch) | New step: tag vouch artifact into VOUCHES superdirectory |
| **RBS0** (SpecTop) | New `RBGC_VOUCHES_PACKAGE` constant definition |
| **RBSGS** (GettingStarted) | Remove "summon first" prerequisite for plumb |

### Implementation files

- `rbgc_Constants.sh` — add `RBGC_VOUCHES_PACKAGE`
- `rbfc_FoundryCore.sh` — rewrite `zrbfc_plumb_core`, `rbfc_plumb_full`, `rbfc_plumb_compact`
- `rbfv_FoundryVerify.sh` — amend `rbfv_vouch` / vouch Cloud Build to tag into VOUCHES
- `rbgjv03-assemble-push-vouch.sh` — additional tag push
- `rbfr_FoundryRetriever.sh` — plumb no longer needs summon; update any coupling

### agent-learner-eval-prototype (₢A6AAM) [abandoned]

**[260705-1032] abandoned**

## Character

Evaluation design pace. Produce a minimal eval artifact that tests whether handbook output actually teaches — can a model with no prior Recipe Bottle knowledge, constrained to reason only from what the handbook says, correctly answer application questions?

## Docket

Create `Tools/rbk/vov_veiled/A6-prototype-eval.md` containing:

- Anti-inference system prompt constraining the model to handbook-only reasoning
- 3–5 application-focused scenario questions (not recall)
- Differential test design: full handbook output vs stripped variant (teaching prose removed), compare scores
- Goodhart mitigation note

Then run the eval against at least the Crash Course output. Report whether the teaching prose carries measurable weight.

### Out of scope

- Full eval harness or automation
- Release-gate mechanics

### bind-mode-coverage-decision (₢A6AAJ) [complete]

**[260414-1310] complete**

## Character

Decision pace — confirm or reject the current assumption that bind mode doesn't need its own hands-on director subtrack. Quick review of what the existing tracks cover, then a recorded decision.

## Docket

Evaluate whether bind mode needs a dedicated hands-on track or is sufficiently covered by existing tracks.

### Review current coverage

1. **Track 7 (Your First Ordination)** — conceptual introduction mentions bind as one of three ordain modes
2. **Track 13 (Assay: Supply Chain)** — evaluator tours bind using bottle-plantuml, sees digest-pin verification verdict

### Assess

Bind mode is operationally simple: no Dockerfile, no build context, no pouch. The director runs ordain, which mirrors an upstream image by digest into GAR. The complexity is in understanding *when* to use bind vs conjure, not in executing it.

Questions to answer:
- Does the evaluator tour in track 13 give enough hands-on exposure?
- Would a director who needs to bind an upstream image in production be served by the conceptual intro + eval tour, or do they need a dedicated walkthrough?
- Is there pedagogical value in a "bind your first upstream image" experience distinct from the evaluation tour?

### Record decision

Update the A6 paddock remaining gaps section with the outcome:
- **No dedicated track**: bind is covered; remove from gaps
- **Add dedicated track**: slate a new director subtrack and update the roster

### Out of scope

- Actually implementing any new track
- Changing existing track dockets

### interview-reconcile-post-fc (₢A6AAE) [abandoned]

**[260414-0725] abandoned**

## Character
Interview pace — distinct from drafting and implementing. This is where we pause after the first complete track lands and integrate what the implementation taught us. Reconciling built reality with planned design. A conversation pace, not a code-producing one, but one that directly unblocks subsequent drafting paces.

## Docket

After First Crucible ships as working code, open an interview session to absorb implementation learnings, refine the paddock, and spec the next track(s).

### Prerequisites
- Pace `₢A6AAD` complete and accepted
- All three handbook tabtargets (`rbw-o.OnboardingStartHere.sh`, `rbw-Occ.OnboardingCrashCourse.sh`, `rbw-Ofc.OnboardingFirstCrucible.sh`) stable and exercised

### Review what was built

Walk through the outcomes of the first implementation cycle:
- What did Frame 4-refined feel like in practice? Any friction?
- What did the prototype eval reveal? Any surprises in agent scores?
- What infrastructure ended up being built vs planned?
- Where did the draft-then-implement gap show up? (i.e., what did the draft assume that implementation proved wrong?)
- Which assumptions flagged in Paces `₢A6AAA` and `₢A6AAC` turned out right, which were wrong?

### Refine the paddock

Based on implementation lessons:
- Update the Frame 4-refined design decision with concrete observations
- Adjust per-unit brevity discipline if the measured length/readability didn't match the prediction
- Refine the rough track roster if Crucible implementation revealed structural issues
- Update open questions list — close answered ones, add newly-surfaced ones
- Record any new technical terms discovered or vocabulary overloads caught

### Pick the next track

Candidates (in rough order of implementation ease):
1. **Take Your Station** — shortest path, probe-heavy, exercises monotonic probe framing, minimal teaching
2. **Establish the Depot** — highest-touch for first-time users, longest wall-clock (~15 min), most external dependencies (GCP, OAuth)
3. **Knight the Realm** — governor administration, introduces Knight the Realm + Receive Your Knighthood pair

Each choice has trade-offs:
- Take Your Station tests probe framing but not teaching depth
- Establish the Depot tests real-world applicability but adds cloud-flake risk
- Knight the Realm exercises cross-track shared snippets (Secret Handling Primer appears in both governor-side and recipient-side tracks)

Pick one or two based on what implementation lessons suggest would advance the heat most. Spec it at the level of the `₢A6AAC` docket.

### Slate the next drafting pace

Produce a gazette entry for the next track's draft pace, to be enrolled at the end of this interview pace. The pattern continues: draft → implement → reconcile, for each track in sequence.

### Consider eval sophistication

By this point, the prototype eval from `₢A6AAA` has run against two tracks. Is it time to graduate from prototype to real release gate? Or does the prototype still need more iteration?

If graduating: slate a dedicated "eval-harness-formalize" pace that builds the release-gate mechanism properly. If not: note what the prototype still needs to learn before we commit to gate mechanics.

### Wait-time pedagogy (defer or engage?)

Establish the Depot and Tethered Cloud Builds both have real wall-clock wait times (15 min and 20 min respectively). The pedagogy for "what does the learner do during the wait" is an open question we haven't addressed. If the next track is one of these, this question becomes blocking. If the next track is Take Your Station, we can defer.

### Record in paddock

At the end of this pace, the paddock should be meaningfully updated with:
- Concrete observations from implementation, not speculation
- Next pace(s) slated and visible on the rail
- A clearer picture of what the release gate looks like
- Refined rough track roster

### Out of Scope
- Drafting the next track (that's the next drafting pace)
- Implementing anything
- Agent-learner eval harness beyond what's already running

## References
- Paces `₢A6AAA` through `₢A6AAD` outputs
- `₣A6` paddock
- Any new paddock/docket/gazette artifacts from subsequent officia working in this heat

### access-probe-5xx-retry (₢A6AAs) [complete]

**[260416-1823] complete**

## Character
Robustness fix with parallel spec+code edits across two probe functions. Low risk, tight scope, clear verification path (rerun service suite).

## Docket

### Phenomenon (2026-04-16, ☉260416-1003, ₢A6AAm rerun)
While rerunning `tt/rbtd-s.TestSuite.service.sh` to verify the graft-composer fix in ₢A6AAm, the suite bailed at the access-probe fixture before four-mode could execute.

- Suite log: `/tmp/service-suite-260416-1030.log`
- Trace dir: `/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-45679`
- Failing case: `rbtdrc_jwt_director` in fixture `access-probe`
- Iteration sequence for director probe (5 iterations, 1500ms delay):
  - 1/5 → HTTP 200 (OK)
  - 2/5 → HTTP 200 (OK)
  - 3/5 → HTTP 200 (OK)
  - 4/5 → **HTTP 503 "The service is currently unavailable."**  → fatal
- Fixture summary: `3 passed, 1 failed, 0 skipped (4 total)`
- Suite exit: 1. four-mode never ran.

The prior 3 successful iterations confirm the JWT-to-OAuth exchange, RBRA key, and role permissions all work. The 503 on iteration 4 is a GCP availability hiccup, not a conformance failure.

### Spec / code lapse assessment — both, matched
- **Spec `RBSAJ-access_jwt_probe.adoc`** step 4.iii.d (Evaluate Response): enumerates 200/401/403 and makes everything else fatal ("unexpected HTTP «HTTP_STATUS»: «RESPONSE_BODY»"). No distinction between transient server-side errors (5xx) and genuine unexpected responses.
- **Code `Tools/rbk/rbgv_AccessProbe.sh:153-169`** (`zrbgv_jwt_ar_probe_once`): faithfully implements the spec — case on 200|206 / 401|403 / *. Default branch is `buc_die "unexpected HTTP ..."`.
- **Second instance**: `zrbgv_payor_crm_probe_once` at lines 175-236 has the identical pattern (lines 219-235). RBSAO (payor OAuth probe) likely has the same spec gap.

Probe purpose per RBSAJ line 121: "Completion demonstrates both that JWT-to-OAuth token exchange is operational for the role and that the resulting token carries the expected Artifact Registry read permissions." This is a token/permission verification, not a liveness/availability SLA — so transient 5xx from the cloud provider should not be conflated with verification failure.

### Proposed resolution
Add a bounded retry with backoff for 5xx responses inside each probe_once function. Retry happens transparently within a single iteration — iteration semantics (sustained access verification) remain unchanged.

- **Retry policy**: up to 3 attempts, backoff 2s / 4s / 8s (total worst-case added latency: 14s per affected iteration).
- **Applies to**: 500, 502, 503, 504 (classic transient 5xx — not 501 Not Implemented, which is a real server capability signal).
- **Final-attempt failure**: same `buc_die` as today, with message distinguishing "repeated 5xx after N retries" from "unexpected HTTP".

### Fix scope (two specs + two functions)
1. `Tools/rbk/rbgv_AccessProbe.sh`
   - `zrbgv_jwt_ar_probe_once` (lines 99-170): wrap the curl + status-evaluation in a retry loop that catches 5xx. Keep 200|206 / 401|403 / other-non-5xx branches unchanged.
   - `zrbgv_payor_crm_probe_once` (lines 175-236): identical treatment.
2. `Tools/rbk/vov_veiled/RBSAJ-access_jwt_probe.adoc` step 4.iii.d: add a new branch "If «HTTP_STATUS» is 5xx (500|502|503|504): retry up to N times with backoff; if still failing, fatal with 'repeated transient error' message".
3. `Tools/rbk/vov_veiled/RBSAO-access_oauth_probe.adoc`: mirror amendment for the payor probe.

### Verification
- `tt/rbtd-r.Run.access-probe.sh` (fast, no side effects) — confirm all 4 cases still pass when GCP is healthy.
- `tt/rbtd-s.TestSuite.service.sh` — full service suite should pass cleanly through a transient 503.
- Manual fault injection (optional, if easy): temporarily alias curl to emit 503, confirm retry kicks in and eventual success.

### Provenance / non-regression concerns
- Does not alter the "fail on genuine unexpected HTTP" contract — only reclassifies 5xx from "unexpected" to "transient, retry-then-fatal".
- Does not alter iteration count, delay, or success criterion for any probe.
- Spec change is additive (new branch before "Otherwise"); no existing language needs reworking.

### handbook-final-docs-hygiene (₢A6AAF) [abandoned]

**[260705-1032] abandoned**

## Character

Final-pace finalizer for `₣A6`. Narrow, mostly mechanical audit of documentation surfaces against the handbook artifacts this heat produced. Runs **after all other A6 paces are wrapped** — the whole point is to catch stale references left behind once the handbook corpus has settled. Cognitive posture: patient, thorough, accepting that the work is sweeping rather than inventing.

## Provenance

This pace carries a three-heat lineage. Originally enrolled in `₣A3` as `₢A3AAF` ("docs-alignment-and-cross-machine-validation") covering five sections. Four sections (Dual-Mode Validation, Term Introduction Audit of current role-tracks, Cross-Machine Testing of role tracks, Dead code removal from `rbgm_onboarding()`) were **obsoleted** by the `₣A6` malformation insight — auditing role-track content is wasted effort if the role-track design itself is being replaced.

Transferred A3 → A5 as `₢A5AAG` ("handbook-split-dangling-refs-audit") with narrowed scope focused on the `bug_*` → `buh_*` and `rbgm_*` → `rbho_*`/`rbhp_*` rename fallout. While in A5 it accreted 9 commits against the `₢A3AAF` coronet — those commits are baked into git history and attributable via the old coronet.

Restrung A5 → A6 as `₢A6AAF` because the gravity had already shifted: the A5-framed docket was already defined in terms of "what did A6 land, what's still stale." That's A6 work wearing A5 clothing. Moving it here lets A5 retire on its own termination condition (readme-rewrite paces landing) without being blocked on A6's handbook corpus maturing.

**Re-verify what the 9 pre-transfer commits already accomplished** before doing new work. `git log --grep 'A3AAF' --oneline` and inspect — the transferred commits may have handled some rename sweeps already.

## Position on the Rail — Last Pace Discipline

This pace is **deliberately positioned last** in `₣A6`. Do not mount it until:

- All handbook implementation paces (`₢A6AAA`, `₢A6AAD`, and any later tracks added during interview-reconcile cycles) are wrapped
- `₣A6`'s interview-reconcile paces have run and produced their paddock refinements
- `₣A5`'s `readme-rewrite-*` paces are complete (the anchors this pace verifies are curated by A5; if A5 hasn't refreshed them, auditing resolution is premature)

If new handbook tracks get enrolled during A6's interview cycles, this pace should be relocated to remain last. Its value is in being the sweep that catches everything the earlier work left behind, and that property only holds if it really is the last thing that runs.

## ⚠ Vocabulary will have shifted by mount time

This docket is written *before* the handbook corpus exists. When mounting, the first action is to **re-audit the vocabulary and reference lists in this docket** — they may be stale. Specifically:

- A6 will have added new onboarding tabtargets under the two-colophon scheme (`rbw-o` + `rbw-O*`). The specific colophons (`rbw-Occ`, `rbw-Ofc`, etc.) will only be known at mount time.
- A6 will have introduced new user-facing vocabulary (intent-based track names, per-track concepts) that wasn't defined at writing time.
- `₣A5`'s `readme-rewrite-*` paces will have executed before this pace runs; the README structure and anchor landscape will reflect their work.
- The old `rbw-gO*` tabtargets may or may not still exist depending on retirement decisions made during A6's later paces.

**Recheck the sections below at mount time and update this docket in place before beginning work if anything is stale.**

## Docket

### Scope — three document surfaces

#### 1. README.md dangling-reference sweep

Search `README.md` for:

- **Old symbol names from A3 renames**: `bug_*`, `zbug_*`, `ZBUG_*`, `bug_guide.sh` — should all have been renamed to `buh_*` / `zbuh_*` / `ZBUH_*` / `buh_handbook.sh`.
- **Old module names from A3 split**: `rbgm_*`, `zrbgm_*`, `ZRBGM_*`, `rbgm_ManualProcedures.sh`, `rbgm_cli.sh` — should all be gone, replaced with `rbho_*` / `rbhp_*` or removed entirely.
- **Obsolete tabtarget references**: any surviving `tt/rbw-gO*` invocation examples. If A6 has retired the old family by this point, they're dangling; if A6 has kept both live during transition, they're valid but stale-looking. Decide per-reference whether to update to the new `rbw-o` / `rbw-O*` scheme or remove.
- **Anchor resolution**: every `buh_tlt` call in `rbho_onboarding.sh` (and anywhere else in the handbook corpus) that targets `${RBGC_PUBLIC_DOCS_URL}#{Anchor}` — verify the anchor exists in the refreshed README. Cross-reference against A6 paddock's "Public-Docs Anchor Coordination with `₣A5`" section, which is the shared truth between the two heats.

#### 2. CLAUDE.consumer.md colophon table

- The table auto-regenerates from `Tools/rbk/rbz_zipper.sh`. Run the regenerator and verify the output matches expectations.
- RBGM-era colophons should be **gone** (`rbw-go`, `rbw-gO*`, `rbw-gP*`, `rbw-gq` — if any still appear, either A6 kept them intentionally during transition or retirement is incomplete; flag for decision).
- New handbook colophons from A6 (`rbw-o`, `rbw-O*` family members) should be **present** with correct frontispieces and purpose text.
- If the colophon table is stale, regenerate it and commit.

#### 3. Tabtarget existence integrity

- Run `ls tt/` and cross-reference against every tabtarget path mentioned in `README.md`, `CLAUDE.consumer.md`, and any public docs that reference Recipe Bottle tabtargets.
- Flag any reference pointing at a file that doesn't exist.
- Fix obvious typos and path errors.
- Any remaining forward-looking references (to tabtargets A6 defined in paddock but never built because the track was deferred or dropped) should be removed — there should be no "planned" tabtarget references surviving this pace, because A6 is the heat responsible for producing them.

### Anchor coordination verification

Cross-reference against the A6 paddock's "Public-Docs Anchor Coordination with `₣A5`" section:

- **Existing anchors** (live at A6 start): verify none were broken by A5's README refresh.
- **New-for-Crash-Course anchors** (`Tabtarget`, `Regime`, `RBRR`, `BURC`, `BURS`, `MarshalZero`, `DiagnosticFailure`): verify A5 added them and A6's handbook consumers resolve to them.
- **New-for-later-tracks anchors** (`Crucible`, `Kludge`, `Ordain`, etc.): verify each one consumed by A6 handbooks has a matching A5 anchor.

This section is the load-bearing coupling between the two heats. If the coupling is broken, this pace is where it gets caught.

### Out of Scope

- **Dual-mode validation** — obsoleted by Frame 4-refined (the whole point of the A6 restart).
- **Term introduction audit of old role-track content** — the old tracks are either retired or coexist with the new tracks as legacy; auditing them is wasted effort.
- **Cross-machine testing of role tracks** — same reason.
- **Dead code removal from `rbgm_onboarding()`** — absorbed by `₢A3AAJ` (already wrapped).
- **Inventing new anchors or concepts** — this pace audits, it does not extend vocabulary.
- **Rewriting README prose** — that's A5's job; this pace checks that what A5 wrote and what A6 consumes agree.

### Validation gates

- `ls tt/` matches all README and CLAUDE.consumer.md references with zero dangling paths.
- `grep -rn 'bug_\|rbgm_\|zbug_\|zrbgm_\|bug_guide' README.md CLAUDE.consumer.md` returns nothing (or only acceptable historical references in changelogs).
- Every anchor consumed by A6 handbooks exists in the refreshed README.
- CLAUDE.consumer.md colophon table reflects current `rbz_zipper.sh` state.

### Inventory questions to answer at mount time

Before deciding what work remains, answer:

1. What did the 9 pre-transfer commits (against `₢A3AAF`) actually accomplish? `git log --grep 'A3AAF' --oneline` and inspect.
2. Which items in sections 1–3 of this docket are already done by those commits or by later A5/A6 work?
3. What's the final state of `₣A6`'s handbook corpus? Which tabtargets actually shipped? Which were planned but dropped?
4. Has `₣A5` retired? Are all `readme-rewrite-*` paces landed? If not, this pace is being mounted too early.
5. Did A6 retire the `rbw-gO*` family, or does it still coexist with the new `rbw-o` / `rbw-O*` scheme?

## References

### Lineage
- `₣A3` (retired) — original home as `₢A3AAF`; commit history attributable via grep
- `₣A5` (retiring or retired) — intermediate home as `₢A5AAG`; scope narrowing happened here
- `₢A3AAI` — `bug_*` → `buh_*` rename, source of one class of rename fallout
- `₢A3AAJ` — `rbgm_*` → `rbho_*` / `rbhp_*` split, source of another class of rename fallout

### A6 coordination
- `₣A6` paddock — "Public-Docs Anchor Coordination with `₣A5`" section is the shared anchor truth
- `₢A6AAA`, `₢A6AAD`, and any later A6 track-implementation paces — produce the handbook corpus this pace audits
- `₢A6AAE` — interview-reconcile-post-fc; this pace runs after that or any later interview-reconcile paces

### Audit targets
- `README.md` — consumer-facing glossary and setup instructions
- `CLAUDE.consumer.md` — auto-generated colophon table, sourced from `Tools/rbk/rbz_zipper.sh`
- `Tools/rbk/rbho_onboarding.sh` — all `buh_tlt` call sites that reference anchors
- `tt/` — authoritative tabtarget inventory
- `Tools/rbk/rbz_zipper.sh` — colophon registry

### External
- Public project page: `https://scaleinv.github.io/recipebottle` — destination of `RBGC_PUBLIC_DOCS_URL` links

### guided-track-buildout (₢A6AAL) [abandoned]

**[260414-1310] abandoned**

## Character

Collaborative pair-programming pace. User drives step by step through adding new tabtargets and building out track text for all items in the StartHere onboarding menu. Pattern replication from the Crash Course implementation — zipper enrollment, thin dispatcher, sequence function, `buh_*` teaching content.

## Docket

User-driven walkthrough: for each track in the StartHere menu, add the tabtarget infrastructure (zipper enrollment, dispatcher, function skeleton) and write the track teaching content. User decides ordering, scope per track, and when to move on.

### Inputs

- Working `rbho_start_here()` menu as the track inventory
- Working `rbho_crash_course()` as the pattern to replicate
- ccyolo vessel and nameplate from ₢A6AAG as hands-on infrastructure

### Per-track steps (user drives sequence)

1. Add zipper enrollment in `rbz_zipper.sh`
2. Create thin dispatcher tabtarget in `tt/`
3. Add sequence function skeleton in `rbho_onboarding.sh`
4. Write teaching content using `buh_*` primitives
5. Run `tt/rbw-MG.MarshalGenerate.sh` to regenerate context
6. Smoke test the new tabtarget
7. Notch

### Out of scope

- Designing the track roster (already in paddock)
- Agent-learner eval (separate pace)

### tadmor-tcp-timeout-debug (₢A6AAy) [abandoned]

**[260705-1032] abandoned**

## Character

Quick verification — the fix is already committed, just need to confirm the 3 cases pass.

## Docket

Verify that adding `172.64.0.0/13` to tadmor's `RBRN_UPLINK_ALLOWED_CIDRS` (committed in 8420c17d) fixes the 3 failing cases.

### Steps

1. Kludge sentry (hallmark may be stale), commit hallmark
2. Charge tadmor
3. Run the 3 previously-failing cases via SingleCase:
   - `rbtdrc_tcp443_allow_example`
   - `rbtdrc_cidr_all_ports_allowed`
   - `rbtdrc_sortie_http_end_to_end`
4. If all 3 pass, run full `tt/rbtd-r.Run.tadmor.sh` to confirm 54/54
5. If any still fail, investigate — the IP may have rotated again or the CIDR range may need further expansion

### Context

Fix propagated from ccyolo nameplate (8b39ea53, ₢A6AAG) where the same Cloudflare rotation was resolved by adding `172.64.0.0/13`. Tadmor was missed in that pass.

### replace-example-com-with-internic (₢A6AAz) [complete]

**[260417-0810] complete**

## Character

Cross-cutting infrastructure change — touches ifrit attack source, theurge orchestrator, both nameplates (tadmor + ccyolo), and needs a spec documentation home in RBS0. Not mechanically hard but the blast radius spans Rust source, regime config, and documentation. Requires a crucible verification cycle for both nameplates.

## Docket

Replace `example.com` with `www.internic.net` as the test connectivity target across the entire security testing infrastructure. Replace the large Cloudflare CIDR allowlist entries with the much smaller ICANN block. Document the rationale in RBS0.

### Problem

example.com moved behind Cloudflare CDN, causing its resolved IPs to rotate across huge ranges. The current allowlist (`93.184.216.0/24 104.16.0.0/12 172.64.0.0/13`) whitelists ~1.57M IPs just to reach one test domain. For a security testing framework where containment is the core value proposition, this is an unacceptable hole. The old IANA-owned IP `93.184.216.34` no longer responds.

### Alternatives considered

| Domain | IP range | CIDR size | CDN | Notes |
|--------|----------|-----------|-----|-------|
| `example.com` (current) | 172.66.x.x, 104.16.x.x | /12 + /13 = ~1.57M IPs | Cloudflare | Rotates unpredictably, triggered this investigation |
| `www.internic.net` | 192.0.46.9 | 192.0.32.0/20 = 4,096 IPs | None (plain Apache) | ICANN-owned since 1993, root infrastructure |
| `www.iana.org` | 192.0.33.8 | Same 192.0.32.0/20 block | None | Also ICANN; same CIDR as internic |
| `connectivity-check.ubuntu.com` | 91.189.88.x + 185.125.188.x | /21 + /22 = ~3K IPs | None | Purpose-built connectivity check (HTTP 204), but two CIDRs needed, Canonical could restructure |

### Decision

**`www.internic.net`** with CIDR `192.0.32.0/20`. ICANN literally runs the internet naming system — these IPs are as stable as anything gets. The domain is an informational reference site (not a production service), appropriate for periodic test traffic. Single CIDR, 383x smaller than current Cloudflare ranges. Responds on both port 80 and 443.

### Touch sites

1. `.rbk/tadmor/rbrn.env`: replace example.com CIDRs and domains with internic
2. `.rbk/ccyolo/rbrn.env`: same replacement
3. `rbev-vessels/common-ifrit-context/src/rbida_attacks.rs`: hardcoded example.com in DNS/TCP/HTTP attack selectors
4. `rbev-vessels/common-ifrit-context/src/rbida_sorties.rs`: example.com in sortie scripts
5. `Tools/rbk/rbtd/src/rbtdrc_crucible.rs`: theurge test case domain references
6. RBS0 or companion spec: document test target selection rationale and the CIDR discipline (exact placement TBD during pace)
7. Grep sweep for remaining `example.com` references in test infrastructure

### Verification

1. Kludge ifrit-tether (source changed), drive hallmark into tadmor
2. Charge + Run.tadmor — all 54 cases green (especially the 3 previously-failing TCP cases)
3. Charge + verify ccyolo connectivity (if ccyolo has example.com-dependent tests)
4. Fast suite green
5. Zero-hit grep for `example.com` in rbrn.env + Rust source (DNS test domains like `example.org` in blocked-DNS tests may intentionally remain — they don't need CIDR allowlisting since they're testing DNS resolution, not TCP connectivity)

### adopt-step-autonumber-across-all-procedures (₢A6AA9) [abandoned]

**[260705-1032] abandoned**

Drafted from ₢A-AAJ in ₣A-.

## Character
Mechanical sweep with judgment — convert existing procedures, evaluate whether the pattern held up.

## Docket
After practicing the Windows procedures with `buh_step1`/`buh_step2`, decide whether to adopt auto-numbered steps across all handbook procedures (onboarding, payor). Convert if yes, document why not if no. Includes PayorEstablish, PayorRefresh, QuotaBuild, and any onboarding procedures still using hardcoded `buh_section "N. ..."`.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 6 director-yokes-vessel-to-reliquary
  2 7 onboarding-menu-navigational-polish
  3 8 handbook-pattern-recap-closers
  4 2 symmetric-sentry-kludge
  5 3 handbook-inline-identifier-hygiene
  6 5 first-crucible-multi-nameplate
  7 4 handbook-trash-comment-sweep
  8 0 ccyolo-workspace-bind-mount
  9 t handbook-render-fixture
  10 n repair-bcg-windows-crash
  11 o repair-bcg-dead-function-sweep
  12 p repair-bcg-stderr-to-tempfile
  13 q repair-bcg-mechanical-sweep
  14 r repair-bcg-payor-yawp-capture
  15 d onboarding-handbook-yelp-conversion
  16 e payor-handbook-yelp-conversion
  17 f combinator-sweep-and-deletion
  18 h decompose-rbh-into-rbh0-directory
  19 j eliminate-buh-line-yawp-one-liners
  20 m vouches-package-lowercase
  21 g review-rbho-bcg-compliance
  22 A menu-and-crash-course-draft
  23 I ifrit-airgap-feasibility
  24 v ifrit-symmetric-rename
  25 w docs-moriah-introduction
  26 a yelp-and-vocabulary-infrastructure
  27 c diastema-yawp-infrastructure
  28 b reliquary-fact-and-rubric-elimination
  29 T conjure-pipeline-spec-revision
  30 R host-side-conjure-hallmark
  31 Q context-to-pouch-rename
  32 Y attest-digest-identity-repair
  33 S conjure-pipeline-smoke-test
  34 G ccyolo-vessel-and-nameplate
  35 K buv-secret-enroll-type
  36 C first-crucible-draft
  37 D ccyolo-yolo-mode-and-sample-content
  38 P foundry-image-credential-refactor
  39 N onboarding-credential-retriever
  40 O onboarding-credential-director
  41 V onboarding-director-airgap
  42 1 airgap-handbook-draft
  43 U onboarding-director-first-cloud-build
  44 u review-onboarding-director-first-kludge
  45 W onboarding-director-bind
  46 H graft-demo-vessel
  47 X onboarding-director-graft
  48 Z plumb-goes-remote-with-vouches-superdirectory
  49 J bind-mode-coverage-decision
  50 s access-probe-5xx-retry
  51 L guided-track-buildout
  52 y tadmor-tcp-timeout-debug
  53 z replace-example-com-with-internic

67823540tnopqrdefhjmgAIvwacbTRQYSGKCDPNOV1UuWHXZJsLyz
··············x··x···x···xx···x····xx·xx··x·······x·· rbho_onboarding.sh
x··x·x···········x···x········x·····xx···x··x·x···x·· rbz_zipper.sh
··xx·xxx···xx····x·························x········x rbhofc_first_crucible.sh
x··x·x···············x··············xx···xx·x·x······ rbk-claude-tabtarget-context.md
x·x·x·x····xx····xx························x········· rbhodf_director_first_build.sh
·····················x·x·········x·xx·x···xx········· README.md
·······x···············x·········x··x···x·x········xx rbrn.env
·x···xx·····x····x·······················x··x·x······ rbho0_start_here.sh
x··························x·xxx·····x·x··x·········· rbfl_FoundryLedger.sh
x··························xx··x····xx·········x····x RBS0-SpecTop.adoc
··············xxx·x··x···xx·························· buh_handbook.sh
·······x···············x·········x··x···x·x··x······· rbrv.env
·····xx·····x····x·······················x··x·x······ rbho0_cli.sh
x················x·····x··xx·····x···x··············· CLAUDE.md
·······x···············x·········x··x···x·x·········· Dockerfile
····xx········x··········xx················x········· rbyc_common.sh
···························x·xx················x····· rbgc_Constants.sh
···················x············x·········x····x····· rbfc_FoundryCore.sh
·········x··x····x·························x········· rbhw0_cli.sh
······x···xx·····x··································· rbhob_base.sh
·····························xxx····················· rbfd_FoundryDirectorBuild.sh
···················x···········x···············x····· rbfv_FoundryVerify.sh
·················x·······xx·························· rbho_cli.sh
··············x···········x················x········· buym_yelp.sh
··············x··········xx·························· BUS0-BashUtilitiesSpec.adoc
·········x·······xx·································· rbhw0_top.sh
········x····································x······x rbtdrc_crucible.rs
·······x·························x··x················ compose.yml, entrypoint.sh
······x·····x····x··································· rbhocc_crash_course.sh, rbhocd_credential_director.sh, rbhocr_credential_retriever.sh, rbhogw_governor_wrapper.sh, rbhopw_payor_wrapper.sh, rbhp0_cli.sh, rbhpb_base.sh
····x········x···x··································· rbhpr_refresh.sh
···x···x····························x················ rbob_bottle.sh
·································x··x················ rbw-cS.SshTo.ccyolo.sh
································x····x··············· rbfr_FoundryRetriever.sh
······························xx····················· RBSAA-ark_abjure.adoc
·························xx·························· buy_yelp.sh
·······················x····························x Dockerfile.tether, RBSIP-ifrit_pentester.adoc, rbida_attacks.rs, rbida_sorties.rs
·······················x················x············ Cargo.lock, Cargo.toml
·······················x·········x··················· .dockerignore
·················xx·································· rbhwcd_docker_context_discipline.sh, rbhwdd_docker_desktop.sh, rbhwdn_docker_wsl_native.sh
················x·········x·························· butt_testbench.sh
················x·x·································· buhw_windows.sh, burh_cli.sh, rbgp_Payor.sh
················xx··································· rbhw_windows.sh
···············x·x··································· rbhp_cli.sh, rbhp_payor.sh
·············x···x··································· rbhpe_establish.sh, rbhpq_quota_build.sh
········x··············x····························· lib.rs
······x··········x··································· rbhwb_base.sh
·····xx·············································· rbhocq_crucible_quench.sh, rbhoct_crucible_trunk.sh, rbhots_tadmor_security.sh
···x·····························x··················· rbob_cli.sh
··x·········································x········ rbhodb_director_bind.sh
··x······································x··········· rbhoda_director_airgap.sh
··················································x·· rbw-gOD.OnboardDirector.sh, rbw-gOG.OnboardGovernor.sh, rbw-gOP.OnboardPayor.sh, rbw-gOR.OnboardRetriever.sh, rbw-gOr.OnboardReference.sh, rbw-go.OnboardMAIN.sh
·················································x··· RBSAJ-access_jwt_probe.adoc, RBSAO-access_oauth_probe.adoc, rbgv_AccessProbe.sh
···············································x····· RBSAP-ark_plumb.adoc, RBSAV-ark_vouch.adoc, RBSGS-GettingStarted.adoc, rbfc_cli.sh, rbgjv03-assemble-push-vouch.sh
··············································x······ rbhodg_director_graft.sh, rbw-Odg.OnboardingDirectorGraft.sh
············································x········ rbw-Odb.OnboardingDirectorBind.sh
··········································x·········· rbjp_pentacle.sh, rbjs_sentry.sh
·········································x··········· rbw-Oda.OnboardingDirectorAirgap.sh
········································x············ Dockerfile.airgap, rbrn_regime.sh, rbtd-r.Run.moriah.sh, rbtd-s.SingleCase.moriah.sh, rbw-Ic.IfritClient.moriah.sh, rbw-Is.IfritSortie.moriah.sh, rbw-cC.Charge.moriah.sh, rbw-cQ.Quench.moriah.sh, rbw-cS.SshTo.moriah.sh, rbw-cb.Bark.moriah.sh, rbw-cf.Fiat.moriah.sh, rbw-cw.Writ.moriah.sh
·····································x··············· RBSCL-consecration_tally.adoc, RBSIR-image_rekon.adoc, RBSIW-image_wrest.adoc, rbw-ir.DirectorRekonsImages.sh
····································x················ RBSRV-RegimeVessel.adoc, count_words.sh, rbrv_regime.sh, rbw-ft.DirectorTalliesHallmarks.sh, rbw-ft.RetrieverTalliesHallmarks.sh, rbw-iw.DirectorWrestsImage.sh, rbw-iw.RetrieverWrestsImage.sh, sample.txt
··································x·················· bupr_PresentationRegime.sh, buv_validation.sh, rbra_regime.sh
·································x··················· ccck-s.ConnectShell.sh, cccw_workbench.sh, docker-compose.yml, launcher.cccw_workbench.sh, rbw-cC.Charge.ccyolo.sh, rbw-cQ.Quench.ccyolo.sh, requirements.md
································x···················· memo-20260305-provenance-architecture-gap.md, rbfr_cli.sh, rbgja02-syft-per-platform.sh
·······························x····················· rbgjv02-verify-provenance.py
······························x······················ rbw-Odf.OnboardingDirectorFirstBuild.sh
·····························x······················· RBSAB-ark_about.adoc, rbgjb01-derive-tag-base.sh, rbgjb03-buildx-push-image.sh, rbgjb03-buildx-push-multi.sh, rbgjb04-per-platform-pullback.sh, rbgjb05-push-per-platform.sh, rbgjb06-imagetools-create.sh, rbgjb06-push-diags.sh, rbgjb07-push-diags.sh
···························x························· RBSDI-depot_inscribe.adoc, RBSTB-trigger_build.adoc
··························x·························· butcym_YelpModule.sh
·······················x····························· dockerfile-ifrit-setcap.patch, main.rs, rbk-claude-theurge-ifrit-context.md
·····················x······························· bud_dispatch.sh, bul_nolog_launcher.sh, launcher_nolog.rbw_workbench.sh, rbw-Occ.OnboardingConfigureEnvironment.sh, rbw-Occ.OnboardingCrashCourse.sh, rbw-Ocd.OnboardingCredentialDirector.sh, rbw-Ocr.OnboardingCredentialRetriever.sh, rbw-Ofc.OnboardingFirstCrucible.sh, rbw-o.ONBOARDING.sh, rbw-o.OnboardingStartHere.sh
·················x··································· rbhw_cli.sh, rbhwht_handbook_top.sh
················x···································· butclc_LinkCombinator.sh, rblm_cli.sh
··············x······································ BCG-BashConsoleGuide.md
········x············································ rbtd-r.Run.handbook-render.sh, rbtdrf_handbook.rs, rbtdrm_manifest.rs, rbtdtm_manifest.rs, rbte_engine.sh
·······x············································· rbob_compose.yml
·····x··············································· rbw-Ots.OnboardingTadmorSecurity.sh
···x················································· rbw-cKS.KludgeSentry.sh
x···················································· RBSDY-director_yoke.adoc, rbw-dY.DirectorYokesReliquaryInVessel.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 388 commits)

  1 W onboarding-director-bind

123456789abcdefghijklmnopqrstuvwxyz
·······················x···········  W  1c
```

## Steeplechase

### 2026-07-05 10:32 - Heat - T

adopt-step-autonumber-across-all-procedures

### 2026-07-05 10:32 - Heat - T

tadmor-tcp-timeout-debug

### 2026-07-05 10:32 - Heat - T

handbook-final-docs-hygiene

### 2026-07-05 10:32 - Heat - T

agent-learner-eval-prototype

### 2026-07-05 10:32 - Heat - T

plumb-goes-remote-with-vouches-superdirectory

### 2026-07-05 10:31 - Heat - T

review-onboarding-director-first-kludge

### 2026-07-05 10:31 - Heat - T

onboarding-director-first-cloud-build

### 2026-07-05 10:31 - Heat - T

review-onboarding-director-airgap

### 2026-07-05 10:31 - Heat - T

rbr-env-in-place-edit-pattern

### 2026-07-05 10:31 - Heat - T

revision-onboarding-validation-tail

### 2026-07-05 10:31 - Heat - n

Salvage the VOUCHES-superdirectory/GAR-direct-plumb design into RBSHR at heat retirement — the worked foundry redesign moves to the horizon roadmap so its pace can drop with nothing lost

### 2026-07-05 10:28 - ₢A6AAW - W

Bind-mode onboarding track verified landed by fact at heat-retirement groom: rbhodb_director_bind.sh sequence function, rbw-Odb tabtarget, zipper enrollment (RBZ_ONBOARD_DIR_BIND), rbho0_cli.sh furnish, and start-here menu entry all live in tree. Wrapped without further work.

### 2026-07-05 09:48 - Heat - f

silks=rbk-08-mvp-handbook-revamp

### 2026-06-26 07:22 - Heat - f

silks=rbk-17-mvp-handbook-revamp

### 2026-06-22 07:40 - Heat - S

onboarding-keyless-precondition-gap

### 2026-06-21 12:45 - Heat - f

silks=rbk-13-mvp-FABLE-handbook-restart

### 2026-06-18 13:08 - Heat - f

silks=rbk-12-mvp-FABLE-handbook-restart

### 2026-06-02 06:35 - Heat - f

silks=rbk-12-mvp-handbook-restart

### 2026-06-01 06:11 - Heat - f

silks=rbk-mvp-handbook-restart

### 2026-05-29 09:06 - Heat - f

silks=rbk-17-mvp-handbook-restart

### 2026-05-29 09:02 - Heat - S

revision-onboarding-validation-tail

### 2026-05-21 11:03 - Heat - n

Add dated risk-management premise memo characterizing two structural cloud-vendor risks across GCP/AWS/Azure/OCI/GitHub: (Axis A) unbounded cost-overrun liability and the absence of a clean native dollar hard-cap on pay-as-you-go consumption, with quotas as the real enforced brake; (Axis B) uneven delivery of terminal-state operation-completion contracts, leaving eventual-consistency races in the customer's lap. Documents that both capabilities provably exist and are selectively withheld, names a citable premise per axis, and ties implications to Recipe Bottle's onboarding cost exposure, RBSCIP IAM propagation, and the Cloud Build quota wedge.

### 2026-05-21 10:14 - Heat - n

Correct RBRP_OPERATOR_EMAIL to the authorized payor account discovered by the OAuth flow (bhyslop@gmail.com -> bhyslop@scaleinvariant.org), aligning the operator identity in rbrp.env with the credentials actually installed.

### 2026-05-21 09:38 - Heat - n

Replace payor_install's unconditional 'Configuration required in rbrp.env' dump with an actual verification of the live rbrp.env against the installed credentials. Each of RBRP_PAYOR_PROJECT_ID, RBRP_OAUTH_CLIENT_ID, and RBRP_OPERATOR_EMAIL is compared to its discovered value (JSON project_id, gated client_id, OAuth userinfo email); RBRP_BILLING_ACCOUNT_ID is a presence check. Matches print as buc_info confirmations; any mismatch or unset value emits buc_warn naming actual-vs-expected, and gates the 'Next: levy the depot' tabtarget behind an all-clear flag (otherwise warns to resolve first). Fixes the confusing imperative output that looked like pending work even when config already matched.

### 2026-05-21 09:33 - Heat - n

Repair two operator-facing printouts to surface real tabtargets instead of internal function names. PayorEstablish step 10 now uses buh_tt with RBZ_PAYOR_INSTALL (renders tt/rbw-gPI.PayorInstall.sh, mirroring the rbhpr_refresh sibling and correcting the placeholder filename to the real client_secret_*.json). The payor_install completion 'Next' guidance now uses buc_tabtarget with RBZ_LEVY_DEPOT (renders tt/rbw-dL.PayorLeviesDepot.sh) instead of naming rbgp_depot_levy. Both resolve via the zipper, so they cannot drift from the colophon-to-filename mapping.

### 2026-05-21 09:29 - Heat - n

Make the payor_install OAuth token-exchange failure actionable: precede the die with guidance that authorization codes are single-use and short-lived and that the user should re-run and paste the full code promptly. Also surface Google's raw 'error' field (e.g. invalid_grant) alongside 'error_description' so a bare 'Bad Request' is no longer the only signal.

### 2026-05-21 09:24 - Heat - n

Sync RBRP_OAUTH_CLIENT_ID to the new payor's OAuth client (297222692580-... -> 727189059962-ddv3midird5bvukmca6na61d2r8d4i4j.apps.googleusercontent.com) ahead of re-running payor_install, so the client-ID match gate passes for the freshly created payor.

### 2026-05-21 09:24 - Heat - n

Fix payor_install client-ID mismatch guidance to use the RBCC_rbrp_file kindle constant instead of a bare 'rbrp.env'. The printed sed/echo fix commands hardcoded the basename, so copy-pasting them from the repo root failed with 'No such file or directory' (real path is rbmm_moorings/rbrp.env). All four references — missing-var echo, mismatch sed, and two prose lines — now resolve via ${RBCC_rbrp_file}.

### 2026-05-21 09:21 - Heat - n

Collapse the five per-site RBGC_PAYOR_APP_NAME yawps in PayorEstablish into a single readonly z_cmd_app_name captured once at the top of rbhp_establish and referenced everywhere (eliminates repeated yawp+local boilerplate; value already single-sourced so no behavior change). Also lowercase the OAuth-client create button label 'CREATE' -> 'Create' in step 5 to match the console.

### 2026-05-21 09:16 - Heat - n

Advise reading the terms before agreeing in the PayorEstablish OAuth consent step: reword 'Check I agree...' to 'Read the linked terms, then (if you accept) check I agree to the Google API Services: User Data Policy'.

### 2026-05-21 09:12 - Heat - n

Match the enable-APIs UI label casing to the console: '+ ENABLE APIS AND SERVICES' -> '+ Enable APIs and services' in the PayorEstablish handbook.

### 2026-05-21 09:11 - Heat - n

Match billing-link UI label casing to the console: 'Link a Billing Account' -> 'Link a billing account' in the PayorEstablish handbook.

### 2026-05-21 09:08 - Heat - n

Fix readonly-variable collision in PayorEstablish: the Project Picker verify step reused the local -r name z_cmd_app_name_verify already declared by the billing-table verify step, aborting the procedure. Rename the picker-step variable to z_cmd_app_name_picker.

### 2026-05-21 09:03 - Heat - n

Match the billing-account UI label casing to the console: 'CREATE ACCOUNT' -> 'Create account' in the PayorEstablish handbook.

### 2026-05-21 09:03 - Heat - n

Clarify the Verify Project Creation step 2: instead of the vague 'page loads and shows your project', give the concrete proof — the Project Picker button text reads the payor app name. Reuse RBGC_PAYOR_APP_NAME so the check tracks the configured name; style 'Project Picker' as a magenta UI element.

### 2026-05-21 08:59 - Heat - n

Clarify and restructure the Create Payor Project steps in the PayorEstablish handbook: reword the Location bullet so 'No organization' reads as the choice this guide describes (org affiliation also works, advanced path); split clicking Edit and entering the chosen project ID into their own numbered steps 4 and 5 (renumbering CREATE/Wait to 6/7); style 'Edit' as a magenta UI element via buyy_ui_yawp.

### 2026-05-21 08:52 - Heat - n

Roll RBRP_PAYOR_PROJECT_ID to a freshly-created payor project (rbwg-p-251228075220 → rbwg-p-260521084625) ahead of establishing a new payor while exercising the onboarding/payor handbook flow.

### 2026-05-21 08:50 - Heat - n

Drop the deferred 'Generic OS Procedures (Bash Utility Kit)' group from the rbw-h0 HandbookTOP index — removed the buh_index_buk call and a redundant blank-line separator from rbhw_handbook_top. The buh_index_buk renderer remains in buh_handbook.sh, unreferenced, so the section can be reinstated trivially when the BUK jurisdiction handbook is undeferred.

### 2026-05-13 07:47 - Heat - S

rbr-env-in-place-edit-pattern

### 2026-05-11 15:40 - Heat - f

silks=rbk-13-mvp-handbook-restart

### 2026-04-29 11:10 - Heat - D

restring 1 paces from ₣A-

### 2026-04-23 13:19 - ₢A6AA6 - W

Yoke operation delivered. New tabtarget tt/rbw-dY.DirectorYokesReliquaryInVessel.sh (user-renamed from the docket's ...VesselToReliquary) delegating to rbfl_yoke in Tools/rbk/rbfl_FoundryLedger.sh. The function takes positional args (stamp, vessel), validates the stamp by listing GAR packages under ${RBGL_RELIQUARIES_ROOT}/<stamp>/ and requiring all six tool names (gcloud, docker, alpine, syft, binfmt, skopeo) to be present, validates the vessel by resolving via zrbfc_resolve_vessel + loading via zrbfc_load_vessel + zrbrv_kindle/enforce + gating on RBRV_VESSEL_MODE=conjure (bind and graft error diagnostically), then rewrites rbrv.env via load-then-iterate + printf — preserving all other lines, substituting the RBRV_RELIQUARY= line in place, or appending under a '# Tool Image Reliquary' header if absent. Initial implementation used awk/grep/cat which violate BCG's Evicted Utilities; refactored to builtins-only (while-read + case + printf) with index-iteration array safety and ': > file' for explicit truncation after user flagged. Zipper enrollment RBZ_YOKE_RELIQUARY added to Depot group in rbz_zipper.sh. Tabtarget context doc regenerated via tt/rbw-MG (initial manual edit was wrong — generator regenerates from zipper). Handbook Odf Step 1 integration landed concurrently by another officium in commit 839c79b5 (₢A6AA8): my Edit call to rbhodf was idempotent against the already-in-HEAD content. Sweep of Oda/Odb/Odg confirmed no other handbook re-yokes (Oda inherits the stamp; Odb is bind; Odg is graft). Post-delivery repairs: (1) Created RBSDY-director_yoke.adoc (8-step spec modeled on RBSDI using rbtr_director / rbrv_vessel_mode_{bind,conjure,graft} / rbtoe_director_authenticate / rbbc_{require,fatal,store,call} / rbrr_{vessel_dir,depot_project_id,gcp_region,gar_repository} linked terms); wired rbtgo_director_yoke into RBS0 mapping section (after rbtgo_director_vouch) and body (after rbtgo_depot_inscribe include); added RBSDY row to CLAUDE.md RBK File Acronym Mappings. (2) Corrected Odf prose tool-count from 'four tool images (skopeo, docker, gcloud, syft)' to 'six builder tool images (gcloud, docker, alpine, syft, binfmt, skopeo)' and 'This mirrors four tool images' to 'six' — matching the authoritative list in rbfc_FoundryCore.sh:120-125 and rbgji01-inscribe-mirror.sh. Pace commit sequence: 3f648c10 (main delivery), c3d583f9 (post-delivery repairs), plus this wrap. Qualify-fast green at every notch. Done criteria met: tabtarget exists and validates diagnostically before write; rbw-Odf teaches Yoke in place of the manual-edit instruction; other director subtracks swept; registry/context/spec updated.

### 2026-04-23 13:18 - ₢A6AA6 - n

A6AA6 post-delivery repairs: (1) Create RBSDY-director_yoke.adoc modeled on RBSDI — 8 steps covering arg validation, vessel resolve+load, mode gate (conjure only, bind/graft fatal), director auth, GAR packages.list, tool-set completeness check, in-place rbrv.env rewrite, completion. Wire rbtgo_director_yoke into RBS0 mapping section (after rbtgo_director_vouch) and body section (after rbtgo_depot_inscribe include). Add RBSDY entry to CLAUDE.md RBK file acronym mappings. (2) Correct Odf Step 1 prose tool count from 'four tool images (skopeo, docker, gcloud, syft)' to 'six builder tool images (gcloud, docker, alpine, syft, binfmt, skopeo)' matching the authoritative list in rbfc_FoundryCore.sh:120-125 and rbgji01-inscribe-mirror.sh; update 'This mirrors four tool images' on line 141 to 'six'. Qualify-fast green.

### 2026-04-23 13:13 - ₢A6AA7 - W

Three navigational-polish additions to Tools/rbk/rbh0/rbho0_start_here.sh landed as a coherent set: (1) 'New to Recipe Bottle? Run Occ first, then pick Kludged Crucibles (local, no cloud) or Director subtracks (requires Payor + Depot).' signpost sentence inserted above the Foundation section header — parenthetical acknowledges the Create-Payor-and-Depot intermediate section without naming it explicitly, keeping the sentence a single navigational thought; four RBYC linked-term refs carry inline hyperlinks. (2) Four-line Foundation concept-intro paragraph under the Foundation section header parallel in shape to the Kludged Crucibles opener (1be9e9f9) — introduces repository Regime (RBRR) / station Regime (BURS) / credentials (RBRA) as a coherent trio before the existing bullets, closes with the 'Local-only — no cloud ceremony yet.' bridge phrase matching the Kludged Crucibles narrative transition; uses RBYC_REGIME, RBYC_RBRR, RBYC_BURS, RBYC_RBRA, RBYC_GOVERNOR as inline hyperlinks. (3) Per-track time-budget parentheticals on all eleven tracks: Occ (~5 min), Ocr/Ocd (merged into the existing '(existing projects only, ~5 min)' paren to avoid awkward double-parens), Ofc (~20 min), Ots (~15 min), Op (~30 min), Og (~10 min), Odf (~30 min, ~15 of which is Cloud Build wall-clock), Oda (~60 min, two 15-20 min Cloud Builds), Odb (~10 min), Odg (~10 min); multi-part estimates reserved for tracks where one component (Cloud Build wall-clock) dominates, consistent placement trailing the title line throughout. Done criteria met: bash -n clean, tt/rbw-o.ONBOARDING.sh renders cleanly with all hyperlinks intact and visible widths ≤ 80 cols (longest new line is the Foundation concept-intro's fourth line at 70 visible chars), tt/rbw-tf.QualifyFast.sh green across tabtarget structure / RBW colophon registrations / generated context freshness, 'Start here if new' sentence appears exactly once above Foundation, time-budget parens consistent across all eleven tracks, Foundation concept-intro renders with correct yelp/hyperlink formatting. Notched as commit 94110857 with explicit file list (Tools/rbk/rbh0/rbho0_start_here.sh only) per multi-officium discipline — five concurrent A6AA6 files (rbfd_FoundryDirectorBuild.sh, rbtdrc_crucible.rs, RBS0-SpecTop.adoc, RBSAG-ark_graft.adoc, new tt/rbw-dY.DirectorYokesReliquaryInVessel.sh) left untouched and have since landed independently through commits 3f648c10 (A6AA6 Yoke implementation) and fa8c434b (A6AA8 wrap). Wrap confirms working tree clean before close.

### 2026-04-23 13:13 - ₢A6AA8 - W

Four 'Step N — The pattern' closers authored across the onboarding handbook family, each distilling its track's generative rule as a retained-knowledge reference card in Occ Step 7's rhythm (prose → table → one-line bridge). Ofc teaches the four-verb local-Crucible iteration loop (Kludge → Charge → SSH → Quench) with a verb→tabtarget→purpose table keyed to ccyolo; Odf teaches the six-ark structured bundle (pouch/image/attest/about/vouch/diags under hallmarks/{hallmark}/) with a Plumb bridge; Odb teaches three Ordain modes as three trust contracts (Conjure SLSA / Bind digest-pin / Graft GRAFTED); Oda teaches the three-link airgap supply chain (Enshrine → forge Tethered + re-Enshrine → final Airgap) plus the Tethered/Airgap/Kludged build-info mini-card. Uniform header 'Step N — The pattern' across all four, continuing each handbook's buh_step1 numbering rather than introducing a new buh_section (Step 6 Ofc/Oda, Step 7 Odf, Step 5 Odb). Pre-existing 'iteration loop' buh_section in Ofc and 'What you learned' buh_sections in Odf/Odb/Oda were replaced — they carried prose lifecycle recaps rather than the docket's generative-rule reference shape. Odb's Step 2 three-mode prose preserved (duplicated, not moved) because it anchors bind-as-one-of-three at first invocation. Banked as commit 839c79b5; diff on rbhodf expanded beyond my closer edit because a concurrent A6AA6 officium's Yoke integration into Step 1 rode along during the jjx_record window (system-reminders confirmed intentional, not to revert). Occ Step 7 unchanged; all four closers rendered under 100 cols with column alignment holding through OSC-8 hyperlink wrapping; bash -n clean. Done criteria met except the fast-suite green gate — which was red on a generated-context staleness owned by the concurrent A6AA6 officium (their regenerated context file already resolves it in their uncommitted tree, greens on their next commit).

### 2026-04-23 13:12 - ₢A6AA6 - n

Implement Yoke: new rbw-dY tabtarget, rbfl_yoke function validating reliquary stamp (6 tool images in GAR) and vessel (regime + conjure mode) before in-place rewrite of RBRV_RELIQUARY, zipper enrollment in Depot group, regenerated tabtarget context. BCG-compliant builtins-only (no awk/grep/cat; while-read + case + printf; zrbfc_load_vessel for sourcing). Handbook Odf Yoke edit landed concurrently in commit 839c79b5 (₢A6AA8). Spec file RBSDY-director_yoke.adoc + RBS0 rbtgo_director_yoke mapping + CLAUDE.md entry deferred per docket's later-sweep escape. Qualify-fast green.

### 2026-04-23 13:07 - ₢A6AA8 - n

Add 'Step N — The pattern' closers to four onboarding handbooks (Ofc, Odf, Odb, Oda), each distilling that track's generative rule as a retained-knowledge reference card in Occ Step 7's shape (prose setup → table → one-line bridge). Ofc teaches the four-verb local-Crucible lifecycle (Kludge → Charge → SSH → Quench) as iteration primitives, with a verb→tabtarget→purpose table keyed to the ccyolo Nameplate the learner just drove; the pre-existing 'The iteration loop' buh_section with its 7-step procedural walkthrough was deleted — commit discipline it recapped is already taught in the trunk's Step 1.2/1.4. Odf teaches the six-ark structured bundle (pouch, image, attest, about, vouch, diags under hallmarks/{hallmark}/) as the retained reference card, closing with a one-line Plumb bridge naming tt/rbw-fpf and tt/rbw-fpc as full and compact inspectors of the pattern. Odb teaches three Ordain modes as three trust contracts (Conjure / Bind / Graft) in a column-aligned table, noting mode-aware Vouch verdicts (full-SLSA / digest-pin / GRAFTED) and mode-mixing tolerance at the Nameplate layer — the Step 2 mid-flow prose version is preserved (duplicated, not moved) because it anchors why bind-as-one-of-three registers at the moment of first bind invocation. Oda teaches the three-link airgap supply chain (Enshrine upstream → Conjure forge Tethered + re-Enshrine → Conjure final Airgap from enshrined forge) and adds the Tethered/Airgap/Kludged build-info mini-card the docket flagged as a worthwhile secondary surface, fitting within the ~10-15 line per-closer budget. In all four, the existing 'Return to start:' line sits unchanged as the terminal line; the new step continues the handbook's buh_step1 numbering (Step 6 in Ofc and Oda, Step 7 in Odf, Step 5 in Odb) without introducing a new buh_section. Three of the four files had pre-existing 'What you learned' buh_section blocks in the closer slot; these carried prose lifecycle-recap content rather than the docket-specified generative-rule reference shape and were replaced (not preserved alongside) per the docket's explicit reference-to-Occ-Step-7 rhythm. Implementation uses existing BUK primitives throughout: buyy_tt_yawp resolves RBZ_CRUCIBLE_* colophons to full tabtarget paths (imprint channels take nameplate moniker as second arg for imprint-baked filenames, param1 channels omit it); RBYC_* yelp-wrapped linked terms carry README.md anchor hyperlinks for every project vocabulary word; RBGC_ARK_BASENAME_* and RBGC_GAR_CATEGORY_HALLMARKS are plain string constants interpolated directly. Column alignment is hand-tuned per-row with trailing spaces compensating for display-width variance (notably 'SSH' vs 'Kludge/Charge/Quench' at 3 vs 6 display chars, and 'attest' at 6 vs the other 5-char basenames). Header phrasing uniform 'Step N — The pattern' across all four, matching Occ's exact phrasing rather than the docket's alternative 'What you now know' — the step-numbering mechanism was already in place via buh_step_style calls and continuing the count was the structurally cleanest option. Verification: bash -n clean on all four files; rendered output inspected via each rbw-O*.sh tabtarget — all closers fit under 100 cols with column alignment holding under ANSI/OSC-8 hyperlink rendering; Occ Step 7 unchanged. tt/rbw-tf.QualifyFast.sh currently flags Tools/rbk/rbk-claude-tabtarget-context.md stale, but that is a concurrent officium's work (their in-flight rbz_zipper.sh edits + new tt/rbw-dY.DirectorYokesReliquaryInVessel.sh tabtarget belong to ₢A6AA6 director-yokes-vessel-to-reliquary); not regenerating to avoid merge hazard with their uncommitted state.

### 2026-04-23 12:58 - ₢A6AA7 - n

Three coherent navigational-polish additions to the onboarding start-here menu (rbho0_start_here.sh). (1) Start-here signpost: one two-line sentence immediately above the Foundation section header — 'New to Recipe Bottle? Run Occ first, then pick Kludged Crucibles (local, no cloud) or Director subtracks (requires Payor + Depot).' — gives first-time readers a recommended sequence without gating; parenthetical 'requires Payor + Depot' acknowledges the intermediate Create-Payor-and-Depot section without naming it explicitly so the sentence stays a single navigational thought. Uses RBYC linked-term refs (RBYC_RECIPE_BOTTLE, RBYC_CRUCIBLES, RBYC_PAYOR, RBYC_DEPOT) so the four terms resolve as docs hyperlinks in-line. (2) Foundation concept-intro paragraph: four prose lines under the Foundation section header introducing the three Foundation items (repository Regime / station Regime / credentials) as a coherent trio before the existing bullets — parallel in shape to the Kludged Crucibles opener landed in 1be9e9f9. Uses RBYC_REGIME twice, RBYC_RBRR, RBYC_BURS, RBYC_RBRA, RBYC_GOVERNOR as inline hyperlinks; closes with the bridge phrase 'Local-only — no cloud ceremony yet.' matching the Kludged Crucibles narrative transition. (3) Per-track time-budget parentheticals on all eleven tracks: Occ (~5 min), Ocr/Ocd (~5 min merged into the existing '(existing projects only, ~5 min)' paren to avoid awkward double-parens), Ofc (~20 min), Ots (~15 min), Op (~30 min), Og (~10 min), Odf (~30 min, ~15 of which is Cloud Build wall-clock), Oda (~60 min, two 15-20 min Cloud Builds), Odb (~10 min), Odg (~10 min). Multi-part estimates used on Odf and Oda where Cloud Build wall-clock dominates the total, per docket guidance that multi-part form is fine where one component dominates. Placement: trailing the track's title line (the same line the entry opens with), consistent across the eleven tracks. Rendered via tt/rbw-o.ONBOARDING.sh: all hyperlinks survive the RBYC linked-term expansion on the new content, all visible line widths remain under ~80 cols at a reasonable terminal width (longest new line is the Foundation concept-intro's fourth line at 70 visible chars). Verification: bash -n clean; tt/rbw-tf.QualifyFast.sh green across tabtarget structure, RBW colophon registrations, and generated context freshness. Other modified files in the working tree (rbfd_FoundryDirectorBuild.sh, rbtdrc_crucible.rs, RBS0-SpecTop.adoc, RBSAG-ark_graft.adoc, new tt/rbw-dY.DirectorYokesReliquaryInVessel.sh) left untouched per multi-officium discipline — they belong to another officium's concurrent work on the next pace (A6AA6 director-yokes-vessel-to-reliquary).

### 2026-04-23 12:51 - Heat - S

handbook-pattern-recap-closers

### 2026-04-23 12:50 - Heat - S

onboarding-menu-navigational-polish

### 2026-04-23 12:46 - Heat - S

director-yokes-vessel-to-reliquary

### 2026-04-23 12:07 - Heat - n

Compress the Kludged Crucibles concept intro in rbho0_start_here.sh from 9 prose lines to 4, preserving all linked vocabulary. Previous form unfolded the three-container architecture across 4-5 sentences with a parenthetical aside for the Pentacle ('(A third container, the Pentacle, owns the network namespace the two share, making the containment structural rather than policy-alone.)') plus a separate 'Each Crucible is described by a Nameplate' sentence plus a 'Two tracks locally build... same mechanical middle' intro paragraph before the bullets. New form collapses the three-container definition into a single parallel-structure sentence: 'a Bottle runs the workload, a Sentry gates egress, and a Pentacle owns their shared network namespace.' Then one transition sentence names the two shipped nameplates and leads into the bullets: 'ccyolo and tadmor below both start the same way:'. All six linked terms carry hyperlinks as before — RBYC_CRUCIBLE, RBYC_BOTTLE, RBYC_SENTRY, RBYC_PENTACLE via attribute references; z_ccyolo and z_tadmor via the buyy_link_yawp captures at L37-38. Dropped from menu: the word 'Nameplate' itself (per-track handbooks rbhofc/rbhots can introduce the term in context where it's earned by the narrative), the 'making the containment structural rather than policy-alone' pedagogical gloss on the Pentacle's role (advanced framing that belongs in the deep handbooks not the navigation menu), and the 'Two tracks... same mechanical middle' redundant intro paragraph (the single-sentence transition 'both start the same way:' carries the same load). 'gates egress' replaces 'enforces what can leave' — slightly more verb-y but punchier. Bullets (L83-84) left untouched — they continue to use em-dash form as the golden-pattern shared-middle bullets. bash -n clean; rendered output verified via rbw-o.ONBOARDING.sh showing 4 prose lines + blank + 2 bullets in the Kludged Crucibles section, all vocabulary rendered with hyperlinks intact.

### 2026-04-23 12:01 - Heat - n

Two polish changes to the onboarding start-here menu (rbho0_start_here.sh), following on from 0f0d3eff's paren-gloss conversion. (1) Bind track's first step bullet — the 'Prepare the guard image' parenthetical was too long to fit on one line in a reasonable terminal: '(Kludge a Sentry locally, or reuse the Conjured Hallmark from Your First Cloud Build)' consumed ~80 chars of paren content on top of 23 chars of left side plus 18 spaces of alignment padding plus the 8-space indent, pushing total well past 120 cols. Shortened per user direction to '(Kludge Sentry or reuse Conjured)' — three linked terms (RBYC_KLUDGE, RBYC_SENTRY, RBYC_CONJURED) act as the gloss; 'reuse Conjured' reads as 'reuse the Conjured one' with the Hallmark noun implicit. Left-side column and all-bullet alignment preserved since only paren content changed. (2) All per-track 'Requires:' clauses removed from the four Director Subtracks and replaced with a single consolidated line under the section header: 'All tracks below require Director credentials and a provisioned Depot.' Removed clauses: Your First Cloud Build L128 ('Director credentials and a provisioned Depot'), Airgap L142 ('Director credentials, a provisioned Depot, and Your First Cloud Build completed'), Bind L159-162 (four-line block: credentials, provisioned Depot, 'First Crucible or First Cloud Build completed'), Graft L175 ('Director credentials, a provisioned Depot, and the rbev-graft-demo Vessel present'). The common Director+Depot gate surfaces once at the top of the section where a learner first encounters the subtracks. Track-specific prerequisites that used to be advertised in the per-track Requires lines — Airgap's 'Your First Cloud Build completed', Bind's 'First Crucible kludged OR YFCB completed', Graft's 'rbev-graft-demo Vessel present' — are no longer surfaced in the menu; per-track tabtarget probes catch these at runtime. Information loss documented to the user for potential re-surfacing as short inline hints if desired. bash -n clean; rendered output verified via rbw-o.ONBOARDING.sh.

### 2026-04-23 11:57 - Heat - n

Rework step-bullet shape across five tracks in the onboarding start-here menu (rbho0_start_here.sh). Previous form was jargon-first with em-dash separator (e.g. 'Inscribe the Reliquary — provision builder tool images'); the reader met project vocabulary before the common-language action it named, forcing them to guess the action from the word. New form inverts the order and uses parens as the gloss: 'Provision the builder toolchain (Inscribe the Reliquary)'. The parenthesized terms are already hyperlinked via RBYC_* attribute references, so readers who want definitions chase the anchor; readers scanning the menu see the action directly. Five sections converted: (1) ccyolo After-Charge bullets L92-93 — 'Run Claude Code in the sandbox (SSH into the Bottle)' and 'Verify network containment (manual curl against the allowlist)'; (2) Your First Cloud Build L120-130 — 2-line track description compressed to 1 line ('Build your first image in the cloud with verified provenance'), 5 step bullets converted (Provision builder toolchain→Inscribe Reliquary; Run your first tethered build→Conjure a Sentry; Inspect images and SLSA→Tally Vouch Plumb Pouch; Pull the image locally→Summon the Hallmark; Clean up→Abjure Rekon); (3) Airgap Cloud Build L138-142 — 5 step bullets converted (Mirror the upstream base→Enshrine into the Depot; Build the toolchain as the new base→Conjure forge Tethered then re-Enshrine; Build the final image with zero network→Conjure airgap Bottle in Airgap mode; Start the sandbox and attack it→Charge moriah and run adversarial suite; Compare provenance side by side→airgap vs Tethered Plumb); (4) Bind L156-159 — 4 step bullets converted (Prepare the guard image→Kludge/reuse Hallmark; Pin the upstream image by digest→Bind PlantUML; Check the provenance verdict→Vouch reads digest-pin; Render a diagram observe blocked egress→Charge pluml Crucible); (5) Graft L173-175 — 3 step bullets converted (Build a local image→busybox tag; Publish the local image→Graft to Depot; Check the provenance verdict→Vouch reads GRAFTED). Golden-pattern bullets at L83-84 (Kludged Crucibles shared-middle) and L101-102 (tadmor explorer After-Charge) still use em-dash — they already had correct word order so conversion would be purely stylistic, deferred pending user decision on whether to homogenize style across the file. Concept-paragraph migration (moving the Crucible-is-a-sandbox intro at L72-78 down into the per-track handbooks rbhofc/rbhots themselves) also deferred as separate concern. Track titles for Bind and Graft (L146 L166) still use em-dash as subtitle separator — structurally distinct role, left untouched. bash -n clean; rendered output verified via rbw-o.ONBOARDING.sh showing all five converted sections aligned with paren-gloss in consistent columns.

### 2026-04-23 11:35 - Heat - n

Mint handbook-track display-name constants and replace bare string literals across the onboarding handbook family. Problem: the same track names — 'Your First Cloud Build' (11 occurrences), 'Airgap Cloud Build' (3), 'Bind Cloud Build' (2), 'Install Retriever Credentials' (2), 'Install Director Credentials' (2), plus singletons 'Your First Crucible' / 'Graft Cloud Build' / 'Tadmor Security' / 'Crash Course' — appeared as bare string literals across the handbook, making any future display-name edit a grep-and-hope exercise and leaving the relationship between cross-referenced tracks invisible at the source level. Fix: add nine RBHO_TRACK_* readonly constants to zrbho_kindle() in rbhob_base.sh, declared right after the existing ZRBHO_DOCKER_*_PREFIX block under a 'Handbook track display names' comment header naming them plain strings (not linked-term yelps) and explaining their purpose. Track names do not correspond to public-docs anchors, so the yelp wrapper earns nothing here — the test from CLAUDE.md 'Load-Bearing Complexity' principle says plain constant is the right shape; if docs later grow per-track anchors, each constant can convert readonly→zrbyc_yk as a one-line swap. Constants minted: RBHO_TRACK_CRASH_COURSE, RBHO_TRACK_RET_CREDS, RBHO_TRACK_DIR_CREDS, RBHO_TRACK_FIRST_CRUCIBLE, RBHO_TRACK_TADMOR, RBHO_TRACK_FIRST_BUILD, RBHO_TRACK_AIRGAP, RBHO_TRACK_BIND, RBHO_TRACK_GRAFT. Replacement sites across 7 files, 14 runtime-display points total: rbho0_start_here.sh 4 (menu entry heading line 120, Airgap Requires line 143, Bind Steps cross-ref line 156, Bind Requires sub-bullet line 163 which also picks up FIRST_CRUCIBLE dropping the bare 'Your First' + RBYC_CRUCIBLE link mix — accepted link-loss on 'Crucible' inside the track name as a consequence of the plain-string design since the track name is a proper-noun label, not a concept reference); rbhocd_credential_director.sh 1 (buh_section line 32); rbhocr_credential_retriever.sh 1 (buh_section line 32); rbhodb_director_bind.sh 5 (buc_doc_brief line 35, buh_section line 83, 'Your First Cloud Build taught' cross-ref line 85, 'Your First Crucible' cross-ref line 144, 'Your First Cloud Build' alternative-path line 150); rbhoda_director_airgap.sh 3 (buc_doc_brief line 34, buh_section line 96 + 'In Your First Cloud Build' line 98 atomically, 'This track assumes' line 132); rbhodf_director_first_build.sh 2 (buc_doc_brief line 29, buh_section line 78); rbhodg_director_graft.sh 1 (buc_doc_brief line 37). Intentional non-replacements documented for future readers: rbz_zipper.sh enrollment-description strings for the same tracks left as bare literals because the dependency order is zipper-kindled-first (zrbho_kindle calls zrbz_sentinel, so rbho kindles after rbz), meaning ${RBHO_TRACK_*} would expand to empty at zipper enrollment time — this is load-bearing constraint, not oversight; generated rbk-claude-tabtarget-context.md left as-is because MarshalGenerate regenerates from zipper (mechanical regeneration); rbhocc_crash_course.sh / rbhots_tadmor_security.sh / rbhofc_first_crucible.sh local section headers ('Recipe Bottle — Configure your Repo's Environment', 'Verify Crucible Containment Under Attack', 'Start a Crucible Using Local Builds') left as descriptive local titles rather than forcibly renaming to track-name strings — these files describe what the track does in-situ using content-oriented titles, a conscious authorial choice distinct from the cross-reference name-tag usage RBHO_TRACK_* exists for; file-header shell comments naming tracks left untouched (not runtime display). Singleton-use constants RBHO_TRACK_TADMOR and RBHO_TRACK_CRASH_COURSE currently have no runtime reference site — minted for parallel structure and future cross-references. Verification: bash -n passes clean on all 8 touched files (rbhob_base.sh plus the 7 consumer files); grep for 'Your First Cloud Build|Your First Crucible|Airgap Cloud Build|Bind Cloud Build|Graft Cloud Build|Install Retriever Credentials|Install Director Credentials|Tadmor Security|Crash Course' across rbh0/*.sh returns only the 9 readonly declarations in rbhob_base.sh itself (all runtime occurrences converted); other modified files in the tree (rbfc_*, rbfd_*, rbfl_*, rbfr_*, rbfv_*, rbgc_Constants.sh, rbgl_GarLayout.sh, rbob_*, rbrn_cli.sh) left untouched per multi-officium discipline — they belong to another officium's concurrent work.

### 2026-04-23 11:14 - Heat - n

Polish the onboarding start-here menu: define Crucible for first-time readers, flag Retriever/Director credential tracks as for-existing-projects-only, and link the Nameplate monikers referenced in the Crucible definition's closing sentence. rbyc_common.sh mints RBYC_CRUCIBLES as a plural variant alongside RBYC_VESSELS (display text 'Crucibles', same 'Crucible' anchor) — covers the new section header and any future plural uses. rbho0_start_here.sh picks up z_ccyolo / z_tadmor buyy_link_yawp captures at the top of the function (alongside the preexisting z_docs capture, before the first buh_section call) pointing at ${RBRR_PUBLIC_DOCS_URL}#ccyolo and #tadmor respectively — matching the lowercase anchor IDs at README.md lines 223 and 228 (a id="ccyolo" and a id="tadmor"), reached via the same anchor-link convention RBS0 anchors use. Kludged Crucibles section rewritten from a bare section header plus one-line intro into a proper concept introduction: header now 'Kludged ${RBYC_CRUCIBLES}' (picks up the new plural link), new 3-sentence definition paragraph follows — a Crucible is a Bottle-running-workload paired with a Sentry-enforcing-egress (Bottle's only network path is through the Sentry's allowlist), Pentacle surfaced as a parenthetical owning the shared network namespace that makes the containment structural rather than policy-alone, closing sentence names the two shipped Nameplates as ${z_ccyolo} and ${z_tadmor} so the tracks below are already anchored before the learner sees them. Foundation section Retriever and Director credential entries each pick up a parenthetical '(existing projects only)' flag on their heading line plus a 2-3 line role definition between heading and the Place-your-RBRA action line: Retriever reads as 'retrieves assets from the Depot for an established project — read-only access to pull existing images from the project's registry; no build, no publish' (anchored on RBS0 §rbtr_retriever's read-only-access-to-repository-for-pulling-images); Director reads as 'conducts cloud operations on an established project — submits Cloud Builds, publishes and manages images in the Depot' (anchored on RBS0 §rbtr_director's full-build-lifecycle-permissions-including-Cloud-Build-submission-and-repository-management). Both entries keep the existing Place-your-RBRA closing line and buh_tt link unchanged, and the parenthetical heading flag makes the scope visible at a glance for a learner scanning the menu before they read the bodies. Section-header yelp rendering verified visually deferred to user review (buh_section wraps the header through buyf_format_yawp with BUYC_BRIGHT_WHITE — if the link wire doesn't survive the color wrap, fallback is plain 'Kludged Crucibles' with RBYC_CRUCIBLE carrying the link from the in-body first sentence). Extensive RBYC_* linked-term usage in the new prose: CRUCIBLE/CRUCIBLES, BOTTLE/SENTRY/PENTACLE, NAMEPLATE, RETRIEVER/DIRECTOR, DEPOT, RBRA.

### 2026-04-23 11:06 - Heat - n

Thicken the Bind track entry in the onboarding start-here menu: install RBYC linked-term coverage across the body prose and break the single-line Requires clause into a bulleted sub-list. Hoisted the z_plantuml / z_pluml buyy_link_yawp captures above the paragraph so PlantUML and pluml read as docs links in the body (previously they were defined only just-in-time above the Steps list, leaving the two body mentions bare). Body substitutions: the standalone 'PlantUML' now renders via ${z_plantuml}; 'Bind pins it' picks up ${RBYC_BIND}; 'the Sentry blocks all egress' picks up ${RBYC_SENTRY}; 'The pluml Crucible' becomes ${z_pluml} ${RBYC_CRUCIBLE}; 'a kludged Sentry' becomes ${RBYC_KLUDGE_D} ${RBYC_SENTRY} (accepting the 'kludged'→'Kludged' capitalization drift that already exists elsewhere in the file — line 73's '${RBYC_CHARGE_D}' rendering 'Charged' mid-sentence is the established precedent); 'a bound Bottle' links only Bottle since RBYC has no 'Bound' past-participle variant (only Kludged/Charged/Conjured/Vouched exist); 'two ordain modes in one Crucible' links Ordain and Crucible. Requires clause reformatted from a single prose line into three bullets matching the Steps list shape and indentation: ${RBYC_DIRECTOR} credentials / a provisioned ${RBYC_DEPOT} / Your First ${RBYC_CRUCIBLE} (${RBYC_KLUDGE_D} ${RBYC_SENTRY} available) or Your First Cloud Build completed. Proper-noun track names ('Your First Crucible', 'Your First Cloud Build') left intact as plain prose — linking them would read as over-markup for what are effectively handbook-section titles. Parallel entries (airgap at line 130, graft at line 160) still carry single-line Requires prose; this pace tightens Bind only per explicit user direction. No structural or behavioral changes — dry-render against current repo state is the visible surface of this edit.

### 2026-04-23 10:55 - ₢A6AAX - W

Landed the graft-mode onboarding handbook track — rbw-Odg. New rbho_director_graft() in rbhodg_director_graft.sh implements a three-step teaching arc (build a local busybox image and point RBRV_GRAFT_IMAGE at it, graft via ordain to push the local image to the Depot, inspect the GRAFTED Vouch verdict) against rbev-graft-demo (landed ₢A6AAH). Track frames graft as the least-trusted ordain mode — SLSA cannot cover the image because Cloud Build never ran, and Vouch reports GRAFTED as an honest signal that provenance stops at the local machine. Dedicated 'Why Graft exists' section between Prerequisites and Step 1 presents the three-mode trust hierarchy side by side (Conjure/SLSA, Bind/digest-pin, Graft/GRAFTED), and 'What you learned' returns to the hierarchy as a Vouch-verdict-readable summary while calling out that rbev-graft-demo has no Nameplate and never runs in a Crucible — pure teaching contrast material. Four filesystem probes render inline: director credential, depot configured, rbev-graft-demo vessel regime with mode=graft, and grafted hallmark in local docker cache via g-prefix tag match. Thin dispatcher tt/rbw-Odg.OnboardingDirectorGraft.sh uses the nolog launcher byte-identical to the Oda/Odb dispatchers (159 bytes). Zipper enrollment in rbz_zipper.sh adds RBZ_ONBOARD_DIR_GRAFT (colophon rbw-Odg, empty channel) between RBZ_ONBOARD_DIR_BIND and RBZ_ONBOARD_PAYOR_HB. CLI furnish in rbho0_cli.sh sources the new helper after rbhodb. Start-here menu entry in rbho0_start_here.sh rewrites the prior stub Graft entry into a fully-wired entry parallel to Bind: refined brief naming Director as build owner, three-bullet Steps list mirroring the teaching arc, Requires line naming Director credentials + provisioned Depot + rbev-graft-demo vessel present, buh_tt linking RBZ_ONBOARD_DIR_GRAFT. Docket staleness resolved: docket called for rbho_onboarding.sh but the ₢A6AAh decomposition split that file into Tools/rbk/rbh0/ one-per-file — followed the current rbhod{f,a,b}_ pattern producing rbhodg_director_graft.sh. Generated tabtarget context regenerated via MarshalGenerate shows rbw-Odg 'Graft Cloud Build — push locally-built image, inspect GRAFTED Vouch verdict' in the Onboarding group between Oda and Odf. Dry-renders of both tt/rbw-Odg.OnboardingDirectorGraft.sh and tt/rbw-o.ONBOARDING.sh confirmed teaching prose, three-mode trust-contract table, and menu entry land cleanly with all RBYC_* linked terms resolving against current repo state. Fast qualify (tabtargets, RBW colophons, generated context freshness) passes. Live end-to-end graft cycle (docker pull busybox → ordain → vouch GRAFTED → tally → abjure) deferred at user review — it would push a real graft hallmark into this station's GAR, which the docket called out as the credentialed-station portion. Extensive RBYC_* linked-term usage throughout: ORDAIN/CONJURE/BIND/GRAFT/VOUCH/TALLY/HALLMARK/HALLMARKS/VESSEL/DEPOT/POUCH/NAMEPLATE/CRUCIBLE/SENTRY/DIRECTOR/PAYOR/RBRV/REGIME/PROBE_YES/PROBE_NO/SBOM/PROVENANCE/CONJURED.

### 2026-04-23 10:55 - ₢A6AAX - n

Land the graft-mode onboarding handbook track: new rbho_director_graft() in rbhodg_director_graft.sh implements the 3-step teaching arc (build a local busybox image and edit RBRV_GRAFT_IMAGE, graft via ordain to push the local image to the Depot, inspect the GRAFTED Vouch verdict). Teaching target rbev-graft-demo (landed ₢A6AAH) carries RBRV_VESSEL_MODE=graft and no project Dockerfile — the track frames graft as the least-trusted ordain mode where SLSA cannot cover the image because Cloud Build never ran, and the Vouch verdict reads GRAFTED as an honest signal that provenance stops at the local machine. Four filesystem probes (director credential, depot configured, rbev-graft-demo vessel regime present with mode=graft, grafted hallmark in local docker cache via g-prefix tag match) render inline as the learner progresses. Dedicated 'Why Graft exists' section between Prerequisites and Step 1 positions graft as the development/prototyping path (local image, you want it in the Depot, Cloud Build roundtrip would slow you down) and presents the three-mode trust hierarchy side by side: Conjure (Cloud Build, SLSA), Bind (upstream digest pin), Graft (local machine, GRAFTED). Closing 'What you learned' section returns to the trust hierarchy as a Vouch-verdict-readable summary and calls out explicitly that rbev-graft-demo has no Nameplate and never runs in a Crucible — it is pure teaching contrast, and production graft usage follows the same pattern (RBRV_VESSEL_MODE=graft, RBRV_GRAFT_IMAGE at a local tag, ordain, vouch records GRAFTED). Extensive RBYC_* linked-term usage throughout: ORDAIN/CONJURE/BIND/GRAFT/VOUCH/TALLY operations, VESSEL/HALLMARK/HALLMARKS/DEPOT/POUCH/NAMEPLATE/CRUCIBLE/SENTRY artifacts, DIRECTOR/PAYOR roles, RBRV/REGIME regime references, PROBE_YES/NO, SBOM/PROVENANCE/CONJURED. Thin dispatcher tt/rbw-Odg.OnboardingDirectorGraft.sh uses the nolog launcher (byte-identical to the Oda/Odb dispatchers — 159 bytes). Zipper enrollment in rbz_zipper.sh adds RBZ_ONBOARD_DIR_GRAFT (colophon rbw-Odg, empty channel) between RBZ_ONBOARD_DIR_BIND and RBZ_ONBOARD_PAYOR_HB. CLI furnish in rbho0_cli.sh sources the new helper after rbhodb. Start-here menu entry in rbho0_start_here.sh rewrites the prior stub Graft entry into a fully-wired entry parallel to Bind: refines the brief to name Director as the build owner and frame Vouch verdict GRAFTED inline, three-bullet Steps list mirroring the teaching arc (Build a local image / Graft — push to Depot / Inspect Vouch verdict), Requires line names Director credentials + provisioned Depot + rbev-graft-demo vessel present as the concrete prerequisites (matching the in-handbook probe set), and ends with buh_tt linking RBZ_ONBOARD_DIR_GRAFT so learners can launch the handbook directly from the menu. Docket staleness noted and handled: docket called for sequence in rbho_onboarding.sh but decomposition (₢A6AAh) split that file into Tools/rbk/rbh0/ one-per-file — followed the current rbhod{f,a,b}_ pattern so the new file is rbhodg_director_graft.sh. Generated tabtarget context regenerated via MarshalGenerate shows rbw-Odg as 'Graft Cloud Build — push locally-built image, inspect GRAFTED Vouch verdict' in the Onboarding group between Oda and Odf. Dry-renders of both tt/rbw-Odg.OnboardingDirectorGraft.sh and tt/rbw-o.ONBOARDING.sh confirm the teaching prose, three-mode trust-contract table, and menu entry land cleanly; all RBYC_* linked terms resolve against current repo state; the three prereq probes fire correctly (director credential, depot, vessel regime all PASS on this station) and the grafted-hallmark probe correctly renders PROBE_NO (no graft has yet been pushed to this station's Depot). Fast qualify (tt/rbw-tf.QualifyFast.sh) passes across tabtarget structure, RBW colophon registrations, and generated context freshness. Live graft-cycle verification (docker pull busybox → ordain → vouch → tally GRAFTED verdict → abjure) intentionally deferred from this notch to user review since it would push a real graft hallmark into the station's GAR — docket called this out as the credentialed-station portion and explicit user decision point before wrap.

### 2026-04-23 10:38 - ₢A6AAH - W

Subsumed rbev-vessels/rbev-busybox-graft/ into rbev-vessels/rbev-graft-demo/ as the graft-mode teaching vessel for Assay: Supply Chain (onboarding track 13). git mv preserves rbrv.env history; header comment reframed from 'BusyBox (Grafting)' test-orientation to 'Graft Demo' teaching-orientation with an explicit scope block (no Dockerfile, no nameplate, no crucible integration — this vessel never runs in a Crucible; the learner builds a local busybox image, grafts it via ordain, inspects GRAFTED vouch verdict). RBRV_SIGIL updated to rbev-graft-demo; RBRV_DESCRIPTION reframed to 'Teaching vessel for graft mode — local image pushed to GAR, no Dockerfile'; RBRV_GRAFT_IMAGE annotated with a comment naming BURE_TWEAK as the fixture's override path and suggesting the forthcoming handbook pace either override similarly or document replacing the line. Static RBRV_GRAFT_IMAGE value retained at the preexisting test-build tag per explicit user guidance — the four-mode fixture overrides at runtime via BURE_TWEAK (rbtdrc_crucible.rs step 3), and handbook pace ₢A6AAX will refine the placeholder when it authors the teaching flow. Naming tension resolved per docket: paddock Vessel table still lists 'rbev-bottle-graft-demo' but paddock remaining-gaps and ₢A6AAH docket both specify 'rbev-graft-demo' with the docket explicit ('NOT rbev-bottle-*') — docket wins, paddock table update deferred as stale-paddock follow-up. Four-mode fixture at Tools/rbk/rbtd/src/rbtdrc_crucible.rs:2216 graft_vessel literal updated to match; fixture behavior unchanged. Sweep across Tools/, rbev-vessels/, .rbk/, .buk/, tt/, Memos/ returned zero stale 'rbev-busybox-graft' references. RCG magic-literal question raised during review: the three single-occurrence vessel-name let bindings (lines 2214-2216) satisfy RCG String Boundary Discipline (single definition site, not magic by the 2+-occurrence threshold); the adjacent 'rbev-vessels/' format-string prefix used three times at lines 2217-2219 IS magic per RCG Constant Discipline (same value, same semantic, 3 occurrences — extractable to const) but is preexisting debt outside ₢A6AAH scope, flagged for follow-up. Validation chain: tt/rbw-rvv.ValidateVesselRegime.sh rbev-graft-demo passes (all PASS/SKIP with graft-gated fields resolving correctly), tt/rbtd-b.Build.sh compiles theurge cleanly, tt/rbw-tf.QualifyFast.sh green across tabtarget structure, RBW colophon registrations, generated context freshness. Credentialed verify (ordain → vouch GRAFTED → tally) deferred to credentialed station per scope-match with my local environment. Design evolution from original docket: docket called for creating rbev-vessels/rbev-graft-demo/ from scratch; collaboration surfaced that rbev-vessels/rbev-busybox-graft/ already existed (used by the four-mode fixture) and user authorized subsuming rather than parallel creation — resulting in single-directory ownership, preserved git history via rename, and one fixture-literal touch point instead of a fork. Scope note: AAH is parallel-capable infrastructure feeding the forthcoming ₢A6AAX graft handbook; handbook authoring (teaching arc, tabtarget, zipper entry, start-here menu entry) is explicitly out of scope per docket. Notch 1565d116 captured the vessel+fixture change; wrap finalizes the pace.

### 2026-04-23 10:38 - ₢A6AAH - n

Subsume rbev-busybox-graft into rbev-graft-demo as the teaching vessel for graft mode. git mv the vessel directory so the rbrv.env file preserves history; rewrite the header comment from 'BusyBox (Grafting)' to 'Graft Demo' with a teaching-intent block naming the scope (no Dockerfile, no nameplate, no crucible integration — this vessel never runs in a Crucible; the learner builds a local busybox image, grafts it via ordain, and inspects the GRAFTED vouch verdict), positioning the vessel as contrast material in the forthcoming Assay: Supply Chain onboarding track (track 13) alongside conjure (rbev-busybox) and bind (rbev-bottle-plantuml). RBRV_SIGIL updated from 'rbev-busybox-graft' to 'rbev-graft-demo' matching the new directory name; RBRV_DESCRIPTION reframed from test-oriented 'Busybox graft test - local image pushed to GAR' to teaching-oriented 'Teaching vessel for graft mode — local image pushed to GAR, no Dockerfile'; RBRV_GRAFT_IMAGE annotated with a comment naming BURE_TWEAK as the override path used by the four-mode fixture and suggesting the forthcoming handbook pace either override similarly or document replacing the line. Static RBRV_GRAFT_IMAGE value left at the preexisting test-build tag (test_busybox:local-20251231-081227-44143-75) per explicit user guidance — the four-mode fixture overrides it at runtime via BURE_TWEAK (see rbtdrc_crucible.rs step 3 where 'threemodegraft'/local_image_ref are set), and the handbook pace ₢A6AAX will refine the placeholder when it authors the teaching flow. Naming tension resolved per docket: paddock Vessel table still lists 'rbev-bottle-graft-demo' but paddock remaining-gaps and ₢A6AAH docket both specify 'rbev-graft-demo' with the docket explicit ('NOT rbev-bottle-*. This vessel never runs in a crucible. No bottle, no nameplate, no sentry.') — docket wins, paddock table update deferred as stale-paddock follow-up (not this pace's scope). Four-mode fixture at Tools/rbk/rbtd/src/rbtdrc_crucible.rs:2216 updated graft_vessel literal from 'rbev-busybox-graft' to 'rbev-graft-demo' to match the renamed directory — fixture behavior unchanged, the literal binds a local identifier used by format!() calls that construct the vessel-directory path ('rbev-vessels/{}') and passed to ordain/wrest/vouch dispatches. Sweep for stale 'rbev-busybox-graft' references across Tools/, rbev-vessels/, .rbk/, .buk/, tt/, Memos/ returned zero hits after the update. RCG magic-literal question surfaced during review: the three single-occurrence vessel-name let bindings (conjure_vessel/bind_vessel/graft_vessel on lines 2214-2216) each appear exactly once and are bound to named locals before downstream use — satisfies RCG's String Boundary Discipline grep-based smell test (single definition site), not magic by RCG's 2+-occurrence threshold. The adjacent 'rbev-vessels/' format-string prefix used three times at lines 2217-2219 IS magic per RCG Constant Discipline (same value, same semantic, 3 occurrences — extractable to const RBTDRC_VESSELS_DIR or similar) but is preexisting debt outside ₢A6AAH scope; flagged for follow-up itch or dedicated sweep pace. Validation: tt/rbw-rvv.ValidateVesselRegime.sh rbev-graft-demo passes (RBRV_SIGIL xname PASS, RBRV_VESSEL_MODE enum PASS, RBRV_GRAFT_IMAGE string PASS, all bind/conjure-gated fields correctly SKIP, RBRV_GRAFT_OPTIONAL_DOCKERFILE empty-string PASS); tt/rbtd-b.Build.sh compiles theurge cleanly with the updated literal; tt/rbw-tf.QualifyFast.sh green across tabtarget structure, RBW colophon registrations, and generated context freshness.

### 2026-04-23 10:16 - ₢A6AAW - n

Land the bind-mode onboarding handbook track: new rbho_director_bind() in rbhodb_director_bind.sh implements the 4-step teaching arc (ready Sentry via kludge or reuse Cloud-Built hallmark, bind PlantUML by digest, inspect digest-pin Vouch verdict, charge pluml Crucible). Step 1 presents both Sentry-readying paths so the track is self-contained — kludge is primary (writes nameplate automatically), Cloud-Built reuse is the alternative for learners arriving from rbw-Odf. Dedicated 'Mode mixture — name it explicitly' section between Prerequisites and Step 1 frames kludged-Sentry + bound-Bottle in one Crucible as intentional pedagogy, not an oversight: each Hallmark carries its own Vouch verdict, a Nameplate can pin Hallmarks of different modes without conflict, runtime containment is enforced by the Sentry policy not by the build chain. Four filesystem probes (director cred, depot, pluml RBRN_SENTRY_HALLMARK non-PENDING, pluml RBRN_BOTTLE_HALLMARK non-PENDING) render inline as the learner progresses. Trust-hierarchy framing in Step 2 contrasts the three ordain modes side-by-side: Conjure (project builds, SLSA), Bind (upstream by digest), Graft (local builds, no provenance). Extensive RBYC_* linked-term usage throughout: ORDAIN/CONJURE/BIND/GRAFT/KLUDGE/KLUDGE_D/CONJURED/CHARGE/QUENCH/VOUCH operations, VESSEL/HALLMARK/HALLMARKS/DEPOT/POUCH/NAMEPLATE/BOTTLE/SENTRY/CRUCIBLE artifacts, DIRECTOR/PAYOR roles, RBRN/RBRV regimes, PROBE_YES/NO. Thin dispatcher tt/rbw-Odb.OnboardingDirectorBind.sh uses nolog launcher (matches Oda dispatcher byte-pattern). Zipper enrollment in rbz_zipper.sh adds RBZ_ONBOARD_DIR_BIND (colophon rbw-Odb, empty channel) between RBZ_ONBOARD_DIR_AIRGAP and RBZ_ONBOARD_PAYOR_HB. CLI furnish in rbho0_cli.sh sources the new helper after rbhoda. Start-here menu entry in rbho0_start_here.sh rewrites the prior stub Bind entry into a fully-wired entry parallel to Airgap: adds the mode-mixture sentence to the entry brief, four-bullet Steps list mirroring the 0-3 teaching arc (Ready Sentry / Bind PlantUML / Inspect Vouch / Charge pluml), Requires line names 'Your First Crucible (kludged Sentry available) or Your First Cloud Build completed' as the dual prerequisite, and ends with buh_tt linking RBZ_ONBOARD_DIR_BIND so learners can launch the handbook directly from the menu. Generated tabtarget context regenerated via MarshalGenerate shows the new entry between Odf and Oda in the Onboarding group. Dry-renders of both tt/rbw-Odb.OnboardingDirectorBind.sh and tt/rbw-o.ONBOARDING.sh confirm Step 0 sentry-kludge prose and mode-mixture section land cleanly with all probes resolving green against current repo state. Fast qualify (tabtargets, RBW colophons, generated context freshness) passes.

### 2026-04-23 09:50 - Heat - n

Tighten Nameplate/Crucible term discipline in README. Charge/Quench appendix no longer says 'Charged Nameplate' — a Crucible is what's Charged; the Crucible definition reads 'triad... defined by a Nameplate' (not 'for a Nameplate'), matching how the Charge line now reads. Swept parallel drift: Hallmark values pin a Nameplate (not a Crucible — the Crucible is derived at runtime); the Director 'Ordains a Hallmark' not 'Ordains a build' (per Ordain's own definition); the workstation Charges the Crucible (not 'starts' it) in the credential-confinement roadmap entry; a Hallmark's image is what gets used in a Crucible. Reference Nameplates section now treats tadmor/moriah/ccyolo strictly as Nameplates — the section heading already says 'Reference Nameplates', and these are the shipped configurations, so the entry ledes and the ccyolo description all read Nameplate rather than Crucible. Reference Project tree rows drop the trailing 'Crucible' qualifier — the directory itself is the Nameplate; describing it further as 'a Crucible' was redundant. No content/structure changes beyond term tightening — 11 edits total, all local substitutions preserving surrounding prose.

### 2026-04-23 09:34 - ₢A6AA1 - W

Landed the airgap cloud build handbook track. rbho_director_airgap() in rbhoda_director_airgap.sh implements the 5-unit Frame 4-refined teaching arc per docket: enshrine rust:slim-bookworm upstream, conjure forge tethered + re-enshrine to populate the airgap vessel's IMAGE_1_ANCHOR, conjure airgap-ifrit airgapped from the enshrined forge, drive the cloud-built hallmark into moriah + charge + run the 34-case adversarial suite, side-by-side plumb comparison against tadmor's tethered ifrit. Two hard-gate probes (director credential, depot) mirror rbhodf's prerequisite block with early-return on failure; three dynamic probes render inline (airgap anchor populated via RBRV_IMAGE_1_ANCHOR, moriah bottle hallmark installed vs PENDING-ordination, tadmor tether hallmark available for comparison) so the handbook is self-describing regardless of where the learner is in the track. Nameplate-facing interpolation uses HANDBOOK_NAMEPLATE env var following rbhots; vessel names are bash-substituted (forge/airgap) since they shift mid-track — explicitly documented in the opening configure-your-session block. No new ordain imprint tabtargets — ordain goes through rbw-fO.DirectorOrdainsHallmark.sh with vessel-arg, matching rbhodf's discipline (imprint-channel reserved for non-ordain like rbw-cC charge and rbtd-r suite). Extensive RBYC_* linked-term usage throughout: ORDAIN/CONJURE/ENSHRINE/VOUCH/PLUMB operations, VESSEL/HALLMARK/DEPOT/RELIQUARY/NAMEPLATE/BOTTLE artifacts, TETHERED/AIRGAP modes, CRUCIBLE/SENTRY/PENTACLE components, DIRECTOR/PAYOR roles, PROBE_YES/NO/SBOM/PROVENANCE/HALLMARKS/VOUCHED/CONJURED variants. Thin dispatcher tt/rbw-Oda.OnboardingDirectorAirgap.sh uses the nolog launcher pattern. Zipper enrollment RBZ_ONBOARD_DIR_AIRGAP added to rbz_zipper.sh (colophon rbw-Oda, empty channel) in the Onboarding group immediately after RBZ_ONBOARD_DIR_FIRST_BUILD; CLI furnish in rbho0_cli.sh sources the new helper; generated tabtarget context regenerated via MarshalGenerate. Onboarding menu wired: the stale Airgap Cloud Build placeholder in rbho0_start_here.sh Director Subtracks (no tabtarget link, 'same vessel, full isolation' copy that described a sentry-rebuild rather than the forge+ifrit chain) replaced with a parallel-to-Odf entry whose 5 bullet steps mirror the teaching arc and whose Requires line names 'Your First Cloud Build completed' as the intellectual prerequisite. Single quality fix during review: dropped an unused z_lk_tether_v link capture that was declared but never consumed (z_tether_vessel itself is still referenced bare in the plumb comparison command). Fast qualify passes; dry-renders of both tt/rbw-o.ONBOARDING.sh and tt/rbw-Oda.OnboardingDirectorAirgap.sh confirm the menu entry resolves between Odf and the Bind demo stub, and the handbook renders cleanly with live probes firing and HANDBOOK_HALLMARK interpolation working in the plumb commands.

### 2026-04-23 09:32 - ₢A6AA1 - n

Wire rbw-Oda into the onboarding start-here menu and drop an unused link capture from the airgap handbook. The menu's Director Subtracks section already carried an Airgap Cloud Build placeholder (four-line stub with no tabtarget link and stale copy — 'same vessel, full isolation' described a sentry-debian-slim rebuild, not the forge+ifrit chain the track actually teaches). Rewrote the entry to match rbhoda's 5-unit arc: brief frames zero-external-network builds and the pre-stage-inputs-in-Depot discipline, five bullet steps mirror the teaching arc (Enshrine rust upstream, Conjure forge Tethered + re-Enshrine, Conjure airgap Bottle Airgap from forge, Charge moriah and run the adversarial suite, Compare Plumb airgap vs Tethered Provenance), Requires line names director credentials + provisioned Depot + Your First Cloud Build completed as the intellectual prerequisite (matching the paddock's director-track dependency chain and the handbook's own 'This track assumes you have completed Your First Cloud Build' framing). buh_tt now references RBZ_ONBOARD_DIR_AIRGAP so learners can launch the handbook directly from the menu, matching the rbw-Odf parallel entry above it. Structural and voice patterns match the existing Director Subtracks entries (brief, Steps: list, Requires: line, tabtarget reference). Also dropped z_lk_tether_v from rbhoda_director_airgap.sh — the tether vessel's #Vessel-anchored link was captured but never consumed (z_tether_vessel itself is still used bare in the Plumb comparison command). Load-bearing discipline — remove elements that do not earn their keep. Dry-renders of both tt/rbw-o.ONBOARDING.sh and tt/rbw-Oda.OnboardingDirectorAirgap.sh confirm the menu entry appears with correct tabtarget resolution between rbw-Odf and the Bind demo stub, and the handbook renders cleanly with probes firing (airgap anchor empty on this repo, tadmor tether hallmark k260417074933-cf716423 surfaced inline). Fast qualify (tabtargets, RBW colophons, generated context freshness) passes.

### 2026-04-23 09:29 - ₢A6AA1 - n

Land the airgap cloud build handbook draft: new rbho_director_airgap() in rbhoda_director_airgap.sh implements the 5-unit Frame 4-refined teaching arc (enshrine rust upstream, conjure forge tethered + re-enshrine, conjure airgap ifrit airgapped from enshrined forge, drive hallmark into moriah + charge + run 34-case security suite, plumb comparison tethered vs airgap). Two hard-gate probes (director credential, depot) mirror rbhodf's prerequisite block; three dynamic probes (airgap-anchor populated via RBRV_IMAGE_1_ANCHOR, moriah hallmark installed vs PENDING-ordination, tether hallmark available for comparison) render inline as the learner progresses. Nameplate-facing interpolation uses HANDBOOK_NAMEPLATE env var following rbhots; vessel names bash-substituted (forge/airgap) since they shift mid-track. No new ordain imprint tabtargets — ordain goes through rbw-fO DirectorOrdainsHallmark with vessel-arg (matches rbhodf discipline). Extensive RBYC_* linked-term usage throughout: ORDAIN/CONJURE/ENSHRINE/VOUCH/PLUMB operations, VESSEL/HALLMARK/DEPOT/RELIQUARY/POUCH/NAMEPLATE/BOTTLE artifacts, TETHERED/AIRGAP modes, CRUCIBLE/SENTRY/PENTACLE components, DIRECTOR/PAYOR roles, PROBE_YES/NO/SBOM/PROVENANCE/HALLMARKS/VOUCHED/CONJURED variants. Thin dispatcher tabtarget tt/rbw-Oda.OnboardingDirectorAirgap.sh uses nolog launcher. Zipper enrollment in rbz_zipper.sh adds RBZ_ONBOARD_DIR_AIRGAP (colophon rbw-Oda, empty channel) after RBZ_ONBOARD_DIR_FIRST_BUILD. CLI furnish sources the new helper. Generated tabtarget context regenerated via MarshalGenerate shows the new entry between Odf and Op in the Onboarding group. Menu wiring in rbho0_start_here.sh and dead-var cleanup (z_lk_tether_v unused) deferred to next notch.

### 2026-04-23 08:53 - Heat - r

moved A6AA1 before A6AAx

### 2026-04-23 08:40 - ₢A6AA4 - W

BCG-compliance comment sweep across the rbh0/ handbook family. Stripped three classes of trash comments — section-divider blocks (#### bars, # --- Header ---, # === / # Step N / # ===), inline what-pointing comments (# Docker runtime, # Depot configured, # Director credential present, # Summoned probe, etc.), and stale numeric references (# Step 2: Install, # Step 3: Validate, etc.) — all pure duplication of rendered output from buh_section/buh_step1/buh_step2. Touched 16 of 25 rbh0/ files across three notches: main sweep (8d1d1cb3) covered 15 files (rbho0_cli, rbho0_start_here, rbhob_base, rbhocc_crash_course, rbhocd_credential_director, rbhocq_crucible_quench, rbhocr_credential_retriever, rbhoct_crucible_trunk, rbhodf_director_first_build, rbhofc_first_crucible, rbhogw_governor_wrapper, rbhopw_payor_wrapper, rbhots_tadmor_security, rbhp0_cli, rbhpb_base), windows batch (6f78906c) covered rbhwb_base.sh, and this wrap commit carries the user's prose tightening in rbho0_start_here.sh (Kludged Crucibles section: one-line preamble, SENTRY/PENTACLE compound in the build step, >30 case count generalization, crucible-explorer one-liner). Cumulative impact: ~176 lines net removed, zero code changes, zero behavioral change. Judgment calls kept: architectural shape notes in rbhofc/rbhots/rbhoct/rbhocq (Args signatures for bash functions with no type system, 'Explorer-intimate top/tail ... mechanical middle delegates to rbhoct_crucible_trunk + rbhocq_crucible_quench', 'No cloud-build or global-suite references ... Airgap/moriah coverage belongs in Building for Crucibles' scope boundary); thin/full furnish architectural family across rbho0_cli/rbhp0_cli/rbhw0_cli (each names what its furnish deliberately includes and excludes); BCG stderr-discriminator notes in rbhob_base/rbhpb_base (discriminator-appended-at-use-site protocol + BURD_TEMP_DIR dispatcher-provided); rbho_po_extract_capture 'No sourcing' security contract; zrbho_credential_install 'Callers append role-specific verification and closing steps' compositional contract. Judgment calls stripped: stale 4-arg signature on zrbho_credential_install (function uses only $1 — docs were actively misleading); 'Legacy role-track functions removed ... see ₣A6 paddock' historical marker (recoverable from git, paddock reference thin); all #### decorative bars; all # Multiple inclusion detection pointers (test+die line is self-documenting); all # Internal Functions / # Kindle — xxx section headers (z-prefix already signals internal). Full grep sweep across all 25 rbh0/ files for '# ---', '# ===', '# Step [0-9]', '#####' confirmed zero stragglers. All 10 affected handbook tabtargets (rbw-h0, rbw-o, rbw-Occ, rbw-Ocr, rbw-Ocd, rbw-Ofc, rbw-Ots, rbw-Odf, rbw-Op, rbw-Og) render byte-identical stdout+stderr vs pre-sweep baselines (captured at /tmp/rbhsweep-baseline/, verified after every batch). Fast qualify (tabtarget structure, RBW colophons, generated context freshness) passes. Lesson captured: comment-strip paces are agent-delegation-friendly because (a) classification taxonomy is well-defined, (b) render byte-identical is a mechanical verification, (c) main context only needs pass/fail per file — for future BCG-sweep paces, structure as main-context classification + judgment-call review + parallel agents per file for mechanical strips.

### 2026-04-23 08:40 - ₢A6AA4 - n

Tighten Kludged Crucibles prose in rbho0_start_here: compress the opener, add pentacle to the build-images line, merge the OAuth note, and soften the tadmor case count from "34" to ">30" so the copy stays correct as cases evolve.

### 2026-04-23 08:38 - ₢A6AA4 - n

Finish ₢A6AA4 handbook-trash-comment-sweep with the windows batch — the last file carrying class-1/2 violations after the 15-file main sweep landed in 8d1d1cb3. Strip three comments from rbhwb_base.sh: L23 '# Multiple inclusion detection' (class 2 inline what-pointer above the ZRBHW_SOURCED guard; the test+die line already names itself), L27-28 '######' bar + '# Internal: Kindle and Sentinel' (class 1 section divider; z-prefix on zrbhw_kindle/zrbhw_sentinel already signals internal), and L31 '# Kindle — project-specific constants for Windows test infrastructure' (class 2 what-pointer above three readonly assignments whose names — ZRBHW_WSL_DISTRO, ZRBHW_DOCKER_CONTEXT, ZRBHW_DOCKER_HOST — already describe them). 5 lines removed (3 comment lines + 2 blank lines that surrounded the bar). Matches the treatment applied to rbhob_base.sh and rbhpb_base.sh in the main sweep; same rationale (strip the 'Multiple inclusion detection' pattern uniformly, strip the #### internal-functions bar, strip the 'Kindle — ...' what-pointer while keeping any BCG stderr-discriminator notes that rbhwb_base does not carry). rbhw0_cli.sh preserved unchanged — the 'Furnish: handbook display procedures ... No regime, no OAuth, no IAM.' docstring at L21-23 carries the same architectural-rationale load as 'Thin furnish' in rbho0_cli.sh and 'Full furnish' in rbhp0_cli.sh (both kept in the main sweep); this family of three furnish-intent docstrings earns its existence by naming what each CLI deliberately includes and excludes. Full grep sweep across Tools/rbk/rbh0/*.sh for '# ---', '# ===', '# Step [0-9]', and '#####' confirms zero stragglers remain across all 25 files. All 10 affected handbook tabtargets (rbw-h0, rbw-o, rbw-Occ, rbw-Ocr, rbw-Ocd, rbw-Ofc, rbw-Ots, rbw-Odf, rbw-Op, rbw-Og) render byte-identical stdout+stderr vs pre-sweep baselines at /tmp/rbhsweep-baseline/. Fast qualify (tabtarget structure, RBW colophons, generated context freshness) passes. Cumulative ₢A6AA4 impact across both notches: 16 of 25 rbh0/ handbook files touched, ~176 lines net removed, zero behavioral change, zero code edits.

### 2026-04-23 08:33 - ₢A6AA4 - n

Strip class-1/2/3 BCG-violating bash comments across 15 of 25 rbh0/ handbook family files (batches 1-5 of planned 6). 171 net lines removed (173 deletions, 2 insertions from re-flowing rbhob_base 'Probe utilities' docstring). Three deletion classes per the docket taxonomy: section-divider blocks (#### bars, # --- Header ---, # === / # Step N: / # ===), inline what-pointing comments (# Docker runtime, # Depot configured, # Director credential present, # Summoned probe, etc.), and stale numeric references (# Step 2: Install, # Step 3: Validate, etc.) — all pure duplication of rendered output from buh_section/buh_step1/buh_step2. Judgment calls kept: architectural shape notes in rbhofc/rbhots/rbhoct/rbhocq (Args signatures for bash functions with no type system, 'Explorer-intimate top/tail ... mechanical middle delegates to rbhoct_crucible_trunk + rbhocq_crucible_quench', 'No cloud-build or global-suite references ... Airgap/moriah coverage belongs in Building for Crucibles' scope boundary); 'thin-deps concession' RBRR-kindle-only comment in rbho0_cli.sh; 'Full furnish' architectural rationale in rbhp0_cli.sh (paired with rbho0_cli's 'Thin furnish'); BCG stderr-discriminator + dispatcher-provided notes in rbhob_base.sh and rbhpb_base.sh; rbho_po_extract_capture security contract ('No sourcing'); 'Callers append role-specific verification and closing steps' compositional contract in zrbho_credential_install. Judgment calls stripped: stale 4-arg signature block in zrbho_credential_install (function only uses $1, docs were actively misleading); 'Legacy role-track functions removed ... see ₣A6 paddock' historical marker (deletion is recoverable from git, paddock reference thin); all file-top ######## decorative bars. All 10 affected handbook tabtargets (rbw-h0, rbw-o, rbw-Occ, rbw-Ocr, rbw-Ocd, rbw-Ofc, rbw-Ots, rbw-Odf, rbw-Op, rbw-Og) render byte-identical stdout+stderr vs pre-sweep baselines captured at /tmp/rbhsweep-baseline/ — validated after every batch. Remaining ₢A6AA4 work: batch 6 windows handbooks (rbhw0_cli.sh 1 strip, rbhwb_base.sh 3 strips — 4 tiny strips total), final grep sweep across all 25 files for trash stragglers, fast qualify, then wrap.

### 2026-04-23 08:14 - ₢A6AA5 - W

Landed option (B') — two kludged-crucible handbooks sharing a mechanical trunk. Extracted kludge-sentry→commit→kludge-bottle→commit→charge choreography into rbhoct_crucible_trunk.sh and quench (with charge-auto-quench pedagogy) into rbhocq_crucible_quench.sh. Refactored rbhofc_first_crucible.sh to preserve explorer-intimate top (Claude OAuth prereq, export-teaching) and tail (SSH + Claude OAuth copy/paste + v2.1.89 paste-regression warning + manual curl verification + rbw-cr.Rack fallback + iteration-loop wrap minus auto-quench line now in shared quench). Added rbhots_tadmor_security.sh evaluator handbook with rich three-part tail: architecture deep-dive (sentry iptables+dnsmasq dual-layer, pentacle namespace-ownership rationale, bottle unprivileged-capability posture), Ifrit+Theurge methodology intro, and tt/rbtd-r.Run.tadmor.sh + tt/rbtd-s.SingleCase.tadmor.sh invocation with per-case result-reading teaching. No cloud-build or TestSuite.{crucible,complete,service} references per docket. Wired new RBZ_ONBOARD_TADMOR_SECURITY (rbw-Ots) in zipper; created tt/rbw-Ots.OnboardingTadmorSecurity.sh launcher; sourced new helpers and rbhots in rbho0_cli.sh furnish; regenerated rbk-claude-tabtarget-context.md. Surfaced both handbooks in the onboarding start-here menu: pulled them out of Foundation into a new top-level 'Kludged Crucibles' section with a DRY'd preamble listing the two shared mechanical steps once (Build images locally / Start the sandbox), each track now showing only its audience-specific post-charge activities. Added RBYC_IFRIT/RBYC_THEURGE linked-term constants and wired them through rbho0_start_here.sh (menu evaluator entry) and rbhots (every prose mention) as OSC-8 hyperlinks to README anchors. Added RBYC_KLUDGE_D/RBYC_CHARGE_D variant constants for 'Kludged'/'Charged' display forms (user-introduced) and repaired them from the 3-arg form (dead README#Kludged/#Charged anchors) to the 4-arg variant pattern (live README#Kludge/#Charge anchors with 'Kludged'/'Charged' display text) matching RBYC_CONJURED/RBYC_VOUCHED. All nameplate-facing kludge args use ${RBYC_HANDBOOK_NAMEPLATE_REF} for learner-shell interpolation; imprint-channel charge/quench use bash-side ${z_moniker} substitution. Grep sweeps confirm zero HANDBOOK_NAMEPLATE raw literals outside rbyc_common.sh and zero TestSuite.{crucible,complete,service} references across the four handbook files. Fast qualify and theurge fast suite (handbook-render exercising post-refactor rbhofc) both pass. Paddock Rough-Track-Roster update and theurge manifest enrollment of rbw-Ots remain explicitly out of scope per docket.

### 2026-04-23 08:09 - ₢A6AA5 - n

Wire the Ifrit and Theurge linked terms through the evaluator track now that they are confirmed README anchors, and repair the anchor side of RBYC_KLUDGE_D/RBYC_CHARGE_D. Add two new 3-arg kindle constants in rbyc_common.sh (RBYC_IFRIT, RBYC_THEURGE) following the established paddock-anchor pattern; both render as OSC-8 hyperlinks to README.md#Ifrit and README.md#Theurge. Update the start-here menu's kludged-crucible evaluator entry to carry the new linked terms ('Ifrit attack binary', 'Theurge + Ifrit' in the adversarial-suite step). Update rbhots_tadmor_security.sh to link every prose occurrence of Ifrit and Theurge: the top framing ('Ifrit attack binary'), the step4 heading ('Meet Ifrit and Theurge'), the methodology prose (three Ifrit mentions + two Theurge mentions describing roles and orchestration), and the result-reading section ('output shows what Ifrit tried'). Vessel-name fragments (rbev-bottle-ifrit-tether, 'bottle-ifrit image') intentionally remain plain since they are not referring to the concept but to an image artifact whose name happens to contain the substring. Repair RBYC_KLUDGE_D and RBYC_CHARGE_D from the 3-arg form (which wired the OSC-8 link to README.md#Kludged and #Charged — anchors that do not exist, producing dead-link hrefs) to the 4-arg variant form matching RBYC_CONJURED/RBYC_VOUCHED: anchor stays #Kludge / #Charge while display text renders as 'Kludged' / 'Charged'. Dry-render confirms the start-here menu's 'Kludged Crucibles' section header and the 'is Charged' line now produce live hrefs to the Kludge/Charge definitions; tadmor handbook renders with Ifrit/Theurge hyperlinks throughout; fast qualify passes.

### 2026-04-23 08:01 - ₢A6AA5 - n

Promote the two kludged-crucible tracks out of the Foundation section into a dedicated top-level 'Kludged Crucibles' section, and DRY the menu entries against the shared mechanical middle. The shared preamble now lists the two voice-neutral steps both tracks take ('Build images locally' + 'Start the sandbox') exactly once, under the section heading; each track's remaining bullet list shrinks to its audience-specific post-charge activities (explorer: shell in + verify containment; evaluator: tour architecture + run adversarial suite). Section framing signals the DRY explicitly with 'Both share the same mechanical middle' and 'They diverge on what happens once the Crucible is Charged'. Foundation shrinks back to its three mechanical prerequisites (Configure Environment / Install Retriever Credentials / Install Director Credentials). Add two linked-term variants to rbyc_common.sh (RBYC_KLUDGE_D, RBYC_CHARGE_D) for 'Kludged'/'Charged' display forms consumed by the new preamble, mirroring the existing RBYC_CONJURED/RBYC_VOUCHED variant pattern. Dry-render confirms the four sections flow (Foundation -> Kludged Crucibles -> Create Payor and Depot -> Director Subtracks) with both tracks resolving correct tabtargets; fast qualify passes.

### 2026-04-23 07:52 - ₢A6AA5 - n

Expose both kludged-crucible handbooks in the onboarding menu Foundation section. Add a parallel evaluator entry for the new RBZ_ONBOARD_TADMOR_SECURITY (rbw-Ots) handbook directly below the existing first-crucible entry. Each entry now carries a distinct audience tag line at the top ('Explorer track: inhabit the sandbox, feel the walls.' / 'Evaluator track: prove the sandbox holds under active attack.') so learners can self-select at a glance. Tighten the explorer step list to match what rbhofc actually teaches post-refactor (drop 'Configure local network' since kludge drives the hallmark into the nameplate directly; drop 'Rack the bottle' since the handbook SSHes rather than racks; add 'Verify network containment' since manual curl against the allowlist is the explorer's verification ritual). Evaluator step list mirrors rbhots: kludge sentry + ifrit bottle, charge, tour the sentry/pentacle/bottle defense-in-depth architecture, run the adversarial suite with per-case result reading. Both entries note no cloud / no credentials; the explorer entry alone flags the Claude OAuth subscription prerequisite. Dry-render confirms both entries display with correct tabtarget resolution; fast qualify passes.

### 2026-04-23 07:47 - ₢A6AA5 - n

Land option (B') two-handbook crucible fork. Extract mechanical kludge-sentry→commit→kludge-bottle→commit→charge choreography from rbhofc into new rbhoct_crucible_trunk.sh (voice-neutral middle, caller supplies nameplate/vessel authorial bash args); extract quench into rbhocq_crucible_quench.sh carrying the charge-auto-quench pedagogy at its natural moment. Refactor rbhofc to delegate the middle + quench to the helpers while preserving explorer-intimate top (Claude OAuth prereq, export-teaching, docker gate) and tail (SSH, Claude OAuth copy/paste with v2.1.89 paste-regression warning, manual curl verification, rbw-cr.Rack fallback, iteration-loop summary minus the auto-quench line now carried by shared quench). Add new rbhots_tadmor_security.sh evaluator handbook: evaluator-intimate top frames critical-eye posture against tadmor (sentry-deb-tether + bottle-ifrit-tether), shared trunk call builds the crucible, then rich evaluator tail tours the containment architecture (sentry iptables+dnsmasq dual-layer, pentacle namespace ownership rationale, bottle unprivileged-capability posture), meets Ifrit+Theurge (attack binary + orchestration methodology), and runs the security suite via rbtd-r.Run.tadmor.sh (full 34) plus rbtd-s.SingleCase.tadmor.sh [case-name] (debug one) with per-case result-reading teaching; strictly no cloud-build or TestSuite.{crucible,complete,service} references per docket. Wire new RBZ_ONBOARD_TADMOR_SECURITY (rbw-Ots) in zipper and tt/rbw-Ots.OnboardingTadmorSecurity.sh launcher; source new helpers and rbhots in rbho0_cli.sh furnish. Regenerate rbk-claude-tabtarget-context.md via MarshalGenerate. All nameplate-facing buh_tt args and commit-message bodies in the trunk use ${RBYC_HANDBOOK_NAMEPLATE_REF} for learner-shell interpolation; imprint-channel charge/quench use ${z_moniker} bash-side substitution (imprint is baked into filename). Grep sweeps confirm zero HANDBOOK_NAMEPLATE raw literals outside rbyc_common.sh and zero TestSuite.{crucible,complete,service} references across the four handbook-track files. Fast qualify passes; fast suite (4 fixtures, 22 cases incl. handbook-render exercising rbhofc post-refactor) passes.

### 2026-04-20 15:46 - ₢A6AA3 - W

Handbook-inline-identifier-hygiene widened mid-pace to install the shared infrastructure that makes the hygiene coherent. Three-part delivery: (1) f131f83f landed fix 1 — rbhpr_refresh.sh:63 rbgp_payor_install hardcode replaced with canonical buh_tt/${RBZ_PAYOR_INSTALL} pattern matching rbhopw_payor_wrapper.sh:81. (2) 33f8d192 installed 7 kindle constants in rbyc_common.sh (RBYC_HANDBOOK_ENV_PREFIX + _NAME/_REF pairs for VESSEL/NAMEPLATE/HALLMARK) with expanded module docstring acknowledging three content categories (linked-term yelp fragments, probe markers, handbook env var metadata). _NAME constants hold the env var name for buh_code export-teaching lines; _REF constants hold the interpolation-ready ${HANDBOOK_*} literal for buh_tt arg consumption; prefix centralizes HANDBOOK_↔ONBOARDING_ swap as a one-line change. (3) 0bdd62e9 migrated rbhodf_director_first_build.sh to consume the kindle constants (9 sites: lines 135/214 buh_code exports use _NAME; lines 185/284/288/298/322/332/336 buh_tt arg consumption converted from single-quoted ONBOARD_* literals to double-quoted ${RBYC_HANDBOOK_*_REF} interpolation) plus bash-side ${z_vessel} at display-text sites (lines 94 linked-term display and 171 file path). Line 43 authorial declaration remains the sole vessel-sigil literal. Horizon Roadmap entry dissolved — the vessel-sigil-registry architectural question collapsed once the kindle-constant scheme clarified that env var naming (not vessel sigils) was the actual concern. Validation: buyy_*_yawp sweep for rbw-/rbev-/rbgp_ returns 0 hits across rbh0/; ONBOARD_VESSEL/ONBOARD_HALLMARK sweep across Tools/rbk/ returns 0 hits; fast qualify passes; handbook renders with HANDBOOK_VESSEL/HANDBOOK_HALLMARK env vars exposed to the learner. Downstream: ₢A6AA5 reslated to consume the settled infrastructure — its interview phase narrows to pure pedagogy (merge/split/middle-path) and its implementation phase references RBYC_HANDBOOK_NAMEPLATE_NAME/_REF concretely.

### 2026-04-20 15:41 - ₢A6AA3 - n

Migrate rbhodf_director_first_build.sh to consume the new RBYC_HANDBOOK_* kindle constants and apply bash-side ${z_vessel} substitution at display-text sites. Lines 135/214 (buh_code export teaching lines) now use ${RBYC_HANDBOOK_VESSEL_NAME} / ${RBYC_HANDBOOK_HALLMARK_NAME}; lines 185/284/288/298/322/332/336 (buh_tt arg consumption) converted from single-quoted ONBOARD_* literals to double-quoted ${RBYC_HANDBOOK_*_REF} interpolation; lines 94/171 (linked-term display text and file-path yawp) use ${z_vessel} bash-side substitution. Line 43 authorial declaration is the sole remaining literal for the vessel sigil. Grep sweep across Tools/rbk/ confirms zero residual ONBOARD_VESSEL/ONBOARD_HALLMARK and zero hardcoded-identifier yawp args in rbh0/. Fast qualify passes; handbook renders with HANDBOOK_VESSEL/HANDBOOK_HALLMARK env vars exposed to the learner.

### 2026-04-20 15:39 - ₢A6AA3 - n

Install 7 kindle constants in rbyc_common.sh for handbook-facing env var naming: RBYC_HANDBOOK_ENV_PREFIX plus matched _NAME/_REF pairs for VESSEL, NAMEPLATE, HALLMARK. _NAME holds the env var name (for buh_code "export ___=..." teaching lines); _REF holds the interpolation-ready "${HANDBOOK_*}" literal (for buh_tt arg consumption). Prefix centralizes the family-name swap as a one-line change. Module docstring expanded to acknowledge three content categories (linked-term yelp fragments, probe markers, handbook env var metadata). Additive only; no existing consumers touched.

### 2026-04-20 15:24 - Heat - S

first-crucible-multi-nameplate

### 2026-04-20 15:10 - ₢A6AA3 - n

Replace hardcoded rbgp_payor_install function name in rbhpr_refresh.sh:63 with ${RBZ_PAYOR_INSTALL} via buh_tt, aligning the payor-refresh handbook with the canonical pattern already used in rbhopw_payor_wrapper.sh:81. Removes one of two hardcoded persistent identifiers flagged in the ₢A6AA3 docket; the second (vessel-sigil hardcode in rbhodf_director_first_build.sh) is pending user discussion.

### 2026-04-20 15:01 - ₢A6AA2 - W

Symmetric sentry kludge landed. Added rbob_kludge_sentry() as structural twin of rbob_kludge(), plus tt/rbw-cKS.KludgeSentry.sh tabtarget (param1) and RBZ_CRUCIBLE_KLUDGE_SENTRY zipper entry. Rewrote first-crucible handbook (rbhofc_first_crucible.sh): replaced copy/paste hallmark transcription with symmetric kludges; inserted explicit commit-between-kludges substeps teaching the cadence (rbfd_kludge gates on clean tree — new invariant); dropped manual-paste teaching per docket option (a). Stripped four step-number section-divider trash-comment blocks I had introduced (BCG violation — the printout is the comment). Regenerated rbk-claude-tabtarget-context.md. Two follow-on paces slated: ₢A6AA3 handbook-inline-identifier-hygiene (fix remaining hardcoded function-name and vessel-sigil yelps discovered during the hardcoded-colophon sweep) and ₢A6AA4 handbook-trash-comment-sweep (generalize the section-divider strip to the full rbh0/ family). Heavy validation (full ccyolo kludge cycle) deferred; handbook renders correctly and fast qualifier passes.

### 2026-04-20 14:58 - Heat - S

handbook-trash-comment-sweep

### 2026-04-20 14:56 - ₢A6AA2 - n

Symmetric sentry kludge: add rbob_kludge_sentry() + rbw-cKS tabtarget mirroring bottle kludge; rewrite first-crucible handbook to teach commit-between-kludges cadence (dropping manual-paste teaching moment per docket option a); strip step-number section-divider trash comments (BCG violation, rendered output is the self-documenting structure)

### 2026-04-20 14:48 - Heat - S

handbook-inline-identifier-hygiene

### 2026-04-20 14:30 - Heat - S

symmetric-sentry-kludge

### 2026-04-20 11:37 - Heat - n

remove cmk vaporware (cmsa-normalizer subagent, cma-normalize/render/validate/doctor slash commands) and normalize RBSHR + RBSCB subdocuments to MCM line-break discipline

### 2026-04-20 11:20 - Heat - n

add existing quoin references (depot, ark, bottle, nameplate) to hoard roadmap entry

### 2026-04-20 11:16 - Heat - n

repair stale RBSCB-CloudBuildRoadmap reference and add hoard (external artifact mirror) roadmap entry

### 2026-04-17 13:28 - Heat - n

Fix bash 5.2 ANSI escape corruption: replace \033 literal strings with real ESC bytes via $'\033' ANSI-C quoting, eliminating backslash collapse in ${var/pat/rep} that caused 033[0m residue on Cygwin

### 2026-04-17 09:29 - ₢A6AAV - W

Phase 1 airgap infrastructure: forge vessel (common-ifrit-forge-context Dockerfile + Cargo files, rbrv.env), airgap-ifrit vessel (Dockerfile.airgap, rbrv.env with pending forge anchor), moriah nameplate (tether sentry, 10.242.4.0/24, PENDING-ordination hallmarks, mirrors tadmor uplink for security suite identity), 10 moriah tabtargets (8 crucible + 2 theurge), enclave port validator fix (network-scoped uniqueness). Handbook content split to ₢A6AA1.

### 2026-04-17 09:29 - ₢A6AAu - n

Add RBYC_PROVENANCE and RBYC_SBOM constants, replace 13 bare occurrences in director first build handbook, update paddock anchor inventory

### 2026-04-17 08:49 - Heat - S

airgap-handbook-draft

### 2026-04-17 08:42 - ₢A6AAu - n

Director First Build yelp sweep: replace ~25 bare vocabulary words with RBYC linked-term constants, add RBYC_CONJURED variant, restructure probes from buyy_*_yawp+buh_ternary to buh_line+RBYC_PROBE_* to avoid LINK-inside-FAIL color bleed

### 2026-04-17 08:33 - ₢A6AAV - n

Phase 1 airgap infrastructure: forge vessel (common-ifrit-forge-context + rbrv.env), airgap-ifrit vessel (Dockerfile.airgap + rbrv.env), moriah nameplate (tether sentry, airgap bottle, 10.242.4.0/24), 10 moriah tabtargets (crucible + theurge), enclave port validator fix (network-scoped uniqueness)

### 2026-04-17 08:25 - ₢A6AAu - n

Complete yelp sweep: replace all 33 bare vocabulary words with RBYC linked-term constants, add RBYC_NAMEPLATES variant, extract z_test_domain local, fix underline bleed bug in diastema LINK/HREF resolver (BUYC_RESET before ambient restore)

### 2026-04-17 08:18 - ₢A6AAu - n

Add Enclave anchor to README, RBYC_ENCLAVE constant, replace bare enclave words with linked terms in First Crucible handbook, update paddock anchor inventory

### 2026-04-17 08:14 - ₢A6AAu - n

Switch linked-term yelp color from blue to bright white for handbook readability

### 2026-04-17 08:10 - ₢A6AAz - W

Replaced example.com with www.internic.net as connectivity test target across the security testing infrastructure. CIDR allowlist reduced from ~1.57M IPs (Cloudflare 104.16.0.0/12 + 172.64.0.0/13) to 4,096 IPs (ICANN 192.0.32.0/20). RCG-compliant: RBIDA_CONNECTIVITY_DOMAIN const in ifrit (pub, imported by sorties), RBTDRC_CONNECTIVITY_DOMAIN in theurge, RBIDA_HTTP_BODY_MARKER_INTERNIC for HTTP body check. Updated both nameplates (tadmor + ccyolo), handbook prose, Dockerfile (added curl + inline backtick comment pattern). Fixed HTTP end-to-end test: HTTP/1.0→1.1, added User-Agent header (InterNIC Apache requires it). Verified: tadmor security suite 54/54 green (basic-infra 2, ifrit-attacks 16, observation 2, correlated 6, sortie-attacks 16, unilateral-novel 3, coordinated-attacks 3, coordinated-integrity 6), fast suite 75/75 green (enrollment-validation 47, regime-validation 21, regime-smoke 7, handbook-render 15).

### 2026-04-17 08:10 - ₢A6AAu - n

First Crucible handbook review: vocabulary-first prose (use linked terms instead of bare words), add Sentry variant constants, alignment cleanup, remove redundant section comments, clarify kludge as host-platform-only

### 2026-04-17 07:49 - ₢A6AAz - n

Drive kludge hallmark k260417074933-cf716423 into tadmor (curl + inline comments)

### 2026-04-17 07:49 - ₢A6AAz - n

Add curl to ifrit bottle for ad-hoc diagnosis, adopt inline backtick comment pattern for apt package documentation

### 2026-04-17 07:46 - ₢A6AAz - n

Drive kludge hallmark k260417074554-1f2e3152 into tadmor after User-Agent fix

### 2026-04-17 07:45 - ₢A6AAz - n

Add User-Agent header to HTTP end-to-end test — InterNIC Apache returns 403 without it

### 2026-04-17 07:42 - ₢A6AAz - n

Drive kludge hallmark k260417074208-f35fc3b0 into tadmor after HTTP/1.1 fix

### 2026-04-17 07:42 - ₢A6AAz - n

Fix HTTP end-to-end test: switch from HTTP/1.0 to HTTP/1.1 with Connection: close — InterNIC returns 403 on HTTP/1.0

### 2026-04-17 07:32 - ₢A6AAz - n

Drive kludge hallmark k260417073148-25088288 into tadmor after internic domain migration

### 2026-04-17 07:31 - ₢A6AAz - n

Replace example.com with www.internic.net as connectivity test target — ICANN-owned single /20 CIDR replaces ~1.57M Cloudflare IPs. RCG-compliant: RBIDA_CONNECTIVITY_DOMAIN const in ifrit, RBTDRC_CONNECTIVITY_DOMAIN in theurge, RBIDA_HTTP_BODY_MARKER_INTERNIC for HTTP body check. Updated both nameplates and handbook prose.

### 2026-04-17 07:23 - ₢A6AAz - n

Document test connectivity target switchover from example.com to www.internic.net in RBSIP spec (new subsection with alternatives trade study and CIDR discipline), update rbst_domain_list format example in RBS0

### 2026-04-17 07:16 - ₢A6AAv - W

Completed ifrit-symmetric-rename verification. Prior commits (04f21003, e29568f4, ab576d58) performed the mechanical rename sweep (rbev-bottle-ifrit → rbev-bottle-ifrit-tether), created common-ifrit-context/ with shared build artifacts, rewrote rbrv.env/rbrn.env, updated all specs and Rust source references, and kludged+drove hallmarks for both bottle and sentry. This session verified: tadmor charge clean with renamed vessel, tadmor security suite 54/54 green (basic-infra 2, ifrit-attacks 16, observation 2, correlated 6, sortie-attacks 16, unilateral-novel 3, coordinated-attacks 3, coordinated-integrity 6), fast suite 75/75 green (enrollment-validation 47, regime-validation 21, regime-smoke 7, handbook-render 15), grep sweep zero stale rbev-bottle-ifrit references in live code.

### 2026-04-17 07:05 - ₢A6AA0 - W

Restructured ccyolo vessel into build-context/workspace split, replaced Docker named volume with host bind mount for bidirectional file visibility, added system-wide UID/GID alignment (rbob_bottle.sh exports host identity via tempfile, rbob_compose.yml forwards to all bottles, ccyolo entrypoint aligns container user). Removed baked-in sample-project from Dockerfile and lazy-copy from entrypoint. Updated handbook prose. Verified: workspace files present, UID match, bidirectional edits, tadmor no-regression charge, 75/75 fast suite green.

### 2026-04-17 07:02 - ₢A6AA0 - n

Align ccyolo sentry hallmark to k260416190047-e29568f4 (matches tadmor, locally available after prior kludge)

### 2026-04-17 07:01 - ₢A6AA0 - n

Drive kludge hallmark k260417070129-f973810d into ccyolo nameplate after vessel restructure

### 2026-04-17 07:01 - ₢A6AA0 - n

Restructure ccyolo vessel into build-context/workspace split, replace Docker named volume with host bind mount, add system-wide UID/GID alignment for bind-mount ownership, clean entrypoint and Dockerfile of baked-in sample content, update handbook prose for workspace visibility

### 2026-04-17 06:51 - Heat - S

ccyolo-workspace-bind-mount

### 2026-04-17 06:02 - Heat - S

replace-example-com-with-internic

### 2026-04-17 05:52 - ₢A6AAy - n

Add 172.64.0.0/13 to tadmor RBRN_UPLINK_ALLOWED_CIDRS — propagating the Cloudflare CIDR fix from ccyolo (8b39ea53, ¢A6AAG) that was never applied to tadmor. example.com now resolves to 172.66.147.243 which falls in this range. Fixes 3 failing tadmor cases (tcp443_allow_example, cidr_all_ports_allowed, http_end_to_end).

### 2026-04-17 05:50 - Heat - S

tadmor-tcp-timeout-debug

### 2026-04-16 19:01 - ₢A6AAv - n

Drive kludge sentry hallmark k260416190047-e29568f4 into tadmor rbrn.env. Prior conjured hallmark c260403180049-r260404010221 was no longer available in GAR; fresh local kludge of rbev-sentry-deb-tether provides a working sentry image for crucible charge verification.

### 2026-04-16 18:31 - ₢A6AAv - n

Drive kludge hallmark k260416183104-f4091b62 into tadmor rbrn.env after ifrit-symmetric-rename kludge of rbev-bottle-ifrit-tether. Local build of new vessel name succeeded (rust:slim-bookworm base, layer cache intact, 9-step build, image+vouch tagged in GAR namespace under new sigil). Clean tree required for upcoming charge — capturing hallmark drive as separate commit from the rename sweep for audit clarity.

### 2026-04-16 18:23 - ₢A6AAs - W

Added bounded 5xx retry to JWT SA and Payor OAuth access probes via new zrbgv_http_get_with_5xx_retry helper (factored out of both probe_once sites per BCG load-bearing-complexity). Retry policy: 3 attempts with 2s→4s exponential backoff on 500/502/503/504, buc_die on exhausted retries or curl-network failure. RBSAJ and RBSAO specs amended with a new branch in step 4.iii.d Evaluate Response (before Otherwise), mirrored between JWT and OAuth probes. Verification: access-probe fixture 4/4 green; full service suite 80/80 green end-to-end (6 fixtures including four-mode-integration, which had transiently failed on an earlier run). The AAm trigger — a 503 bailing the suite before four-mode could execute — is now resolved; the suite rides through transient 5xx without operator intervention.

### 2026-04-16 18:23 - ₢A6AAw - W

Documentation skeleton for moriah nameplate landed bundled into AAv's commit 04f21003 (see commit message for explicit attribution of 'accompanying edit' / 'companion edit'). Surfaces touched: README #moriah anchor + parallel definitional prose + tadmor refinement (consuming Kludged Hallmarks for daily iteration framing) + 4 new directory rows (moriah/, common-ifrit-forge-context, ifrit-airgap, ifrit-forge, plus Dockerfile.airgap line); RBSIP three-vessel family expansion + tether/airgap nameplate split (moriah introduced paralleling tadmor) + moriah.compose.yml in compose-fragment example. RBS0 rbsi_moriah quoin skipped per docket judgment (nameplate moniker, not spec concept). RBSHR line 144 moriah row skipped per docket judgment (tadmor-specific CIDR config, not general adversarial testing). README line 191 glossary single-example preserved (Reference Nameplates section now lists both prominently). Anchors now in place to gate AAV Phase 2 handbook content; handbook tabtargets can hyperlink to ${RBGC_PUBLIC_DOCS_URL}#moriah via buh_tlt. Working tree clean at wrap; this is a pure state-marker commit.

### 2026-04-16 17:24 - ₢A6AAs - n

Add bounded 5xx retry to access probes via new zrbgv_http_get_with_5xx_retry helper (factored per BCG load-bearing-complexity — same curl invariants, same failure class, same recovery across the two probe_once sites). Retry policy as kindle constants in rbgv: 3 attempts, initial 2s backoff doubling (2s→4s, worst-case +6s added latency). Helper takes label/url/token/resp-file/code-file/stderr-file; preserves all forensic stderr capture; buc_die on curl-network failure or exhausted 5xx retries; returns to caller for normal case-switch on any non-5xx response. Caller blocks in zrbgv_jwt_ar_probe_once and zrbgv_payor_crm_probe_once reduced to a single helper call — 200/401|403/other case-switches at call sites unchanged. RBSAJ and RBSAO specs amended with a new branch in step 4.iii.d Evaluate Response (before Otherwise): transient 5xx triggers retry with exponential backoff, buc_die after 3 repeated 5xx; existing 200/401/403/404 branches untouched. Verification: tt/rbtd-r.Run.access-probe.sh green 4/4 (healthy path through the helper). Service suite ran access-probe 4/4 (the AAm blocker is resolved) and now proceeds to four-mode, which hit a separate unrelated failure (rbtdrc_four_mode_supply_chain) — orthogonal to this pace, out of scope.

### 2026-04-16 17:22 - Heat - S

review-onboarding-director-airgap

### 2026-04-16 15:17 - ₢A6AAv - n

ifrit-symmetric-rename mechanical sweep: migrated ifrit source into new shared rbev-vessels/common-ifrit-context/ (mirrors common-sentry-context pattern) with Dockerfile.tether (body byte-identical to former ifrit Dockerfile, preamble updated to reference shared context and note the coming airgap companion). Renamed vessel directory rbev-bottle-ifrit → rbev-bottle-ifrit-tether and rewrote its rbrv.env (SIGIL, DESCRIPTION marked tethered, CONJURE_DOCKERFILE/BLDCONTEXT point to common-ifrit-context/Dockerfile.tether). Drove new vessel name into .rbk/tadmor/rbrn.env (hallmark fields preserved — underlying image unchanged, only vessel name moves). Whole-word grep sweep updated CLAUDE.md RBID mapping (now points to common-ifrit-context/), README.md vessel tree (added common-ifrit-context rows and renamed ifrit entry to rbev-bottle-ifrit-tether with rbrv.env-only layout; moriah nameplate rows landed via accompanying edit), Tools/rbk/rbk-claude-theurge-ifrit-context.md (source path references, fO/fs tabtarget examples), Tools/rbk/rbtid/dockerfile-ifrit-setcap.patch (diff headers repointed to new location — stale artifact but cleaned for consistency), RBSIP-ifrit_pentester.adoc (Ifrit Bottle Vessel section expanded to describe the three-vessel family — tether/airgap/forge — and shared build context; companion edit also documents nameplate usage for tadmor+moriah). Paddock vessel table intentionally not touched here — deferred to AAI wrap per docket. Retired jjh_* history files left untouched per convention; jjp_uAl6.md paddock already carries new names from the prior AAI curry. Fast suite green (15/15 handbook-render + 7 regime-smoke + all other fast fixtures). Kludge/charge/tadmor-suite verification pending in next step.

### 2026-04-16 10:41 - Heat - S

docs-moriah-introduction

### 2026-04-16 10:39 - Heat - r

moved A6AAV before A6AAU

### 2026-04-16 10:34 - Heat - r

moved A6AAs before A6AAF

### 2026-04-16 10:34 - ₢A6AAI - W

Recorded architectural blueprint for ifrit airgap form: shared-build-context + three-vessel pattern with nameplate symmetry. Vessels: `common-ifrit-context/` houses shared rbid source plus Dockerfile.tether and Dockerfile.airgap; `common-ifrit-forge-context/` houses the forge; three vessel directories (rbev-bottle-ifrit-tether, rbev-bottle-ifrit-airgap, rbev-bottle-ifrit-forge) consume them. Nameplates: tadmor stays kludge-friendly daily-iteration target (repointed to bottle-ifrit-tether); new moriah is the airgap chain demonstration target (sentry-deb-airgap + bottle-ifrit-airgap, enclave 10.243.0.0/24, uplink mirrors tadmor for security-suite identity). Theurge fixture parity by name match — no Rust changes; moriah stays out of ZRBTE_SUITE_CRUCIBLE (cloud-built airgap hallmark required, not routine). Implementation flows through new ₢A6AAv (ifrit-symmetric-rename mechanical sweep) → ₢A6AAV (forge + airgap-ifrit + moriah + tabtargets + handbook content). Paddock landed in 5e0d6135 with vessel/nameplate tables refreshed, track 12 narrative rewritten for forge → airgap-ifrit → moriah chain, remaining-gaps entry closed. Basis [2] supersedes basis [1]'s narrower 'two unrelated vessels' framing after design conversation surfaced the sentry shared-context precedent and the kludge-cycle tension.

### 2026-04-16 10:30 - Heat - d

paddock curried: record AAI ifrit-airgap decision: shared context, 3-vessel split, tadmor/moriah nameplate symmetry

### 2026-04-16 10:27 - Heat - S

ifrit-symmetric-rename

### 2026-04-16 09:54 - ₢A6AAq - W

Mechanical BCG compliance sweep across rbh0/ tree completed via three parallel subagents on file-disjoint targets (rbho* / rbhp* / rbhw*). 11 files modified across 6 docket buckets: multi-var local splits (7 sites in onboarding files; payor/rbhob sites already cleaned by prior AAr/AAo paces, no-op), `local -r` promotion on `z_rbk_kit_dir` (3 sites across the three *0_cli.sh files), `$(uname -s)` → BCG-compliant temp-file capture in rbhpb_base.sh:34 using local discriminated paths (no kindle prefix — single-site usage doesn't earn factoring per load-bearing-complexity discipline; contrast 8-site ZRBHO_DOCKER_*_PREFIX), `|| buc_die` source guards on 20 source lines in rbhp0_cli.sh:40-59 (line 26 buc_command.sh bootstrap left unwrapped — circular), `zrbho_sentinel` first-line insertion in 8 onboarding leaf files (2 stale `# No sentinel — works pre-kindle` comments removed; verified safe because zrbho_furnish unconditionally kindles before public dispatch), and removal of redundant `test ${#...} -gt 0` enforce check in rbhpb_base.sh:51-52 (non-load-bearing — `test -n` covers it). Heat-level verification: zero multi-var hits across rbh0/, 22 wrapped sources in rbhp0_cli.sh, all 8 public leaf functions begin with sentinel call. Fast suite 90/90 (enrollment-validation 47, regime-validation 21, regime-smoke 7, handbook-render 15 — all 8 onboarding + 4 windows + 3 payor handbooks render clean). Notch 04128d32 captured the working tree.

### 2026-04-16 09:52 - ₢A6AAm - W

Verified vouches-package-lowercase end-to-end: service suite 80/80 pass on commit 0df61c4c (clean+pushed tree). Includes the original RBGC_VOUCHES_PACKAGE uppercase→lowercase fix, the BCG curl capture idiom repair in rbfc_FoundryCore.sh's build-polling loop, and the missing _RBGV_VOUCHES_PACKAGE substitution added to graft's combined about+vouch composer. four-mode-integration case rbtdrc_four_mode_supply_chain: PASS.

### 2026-04-16 09:51 - ₢A6AAq - n

Mechanical BCG compliance sweep across rbh0/ tree — three parallel subagents on file-disjoint targets (rbho*, rbhp*, rbhw*). Bucket 1 (multi-var local → one-per-line): 7 sites in rbhocc/rbhofc/rbhodf onboarding files split into single-var lines preserving initializers; payor sites (rbhpe/rbhpr/rbhpq:32) and rbhob_base.sh sites already cleaned by ₢A6AAr/₢A6AAo (no-op). Bucket 2 (`local -r` on non-mutated): 3 sites — `z_rbk_kit_dir` in rbho0_cli.sh:39, rbhp0_cli.sh:38, rbhw0_cli.sh:36 promoted (read-only path prefix used in source statements). Bucket 3 (`$()` on external command): rbhpb_base.sh:34 `case "$(uname -s)"` replaced with BCG-compliant temp-file capture (local discriminated paths `zrbhp_uname_{1,2}_kernel.txt` under BURD_TEMP_DIR; no kindle prefix constant — single-site usage doesn't earn the indirection per load-bearing-complexity discipline, contrast 8-site ZRBHO_DOCKER_*_PREFIX). Bucket 4 (`|| buc_die` on source): 20 source lines in rbhp0_cli.sh:40-59 wrapped with diagnostic `|| buc_die "Failed to source ..."` matching rbho0_cli.sh convention; line 26 buc_command.sh bootstrap left unwrapped (defines buc_die — circular). Bucket 5 (zrbho_sentinel first-line): 8 onboarding leaf files (rbho0_start_here, rbhocc, rbhocr, rbhocd, rbhofc, rbhodf, rbhopw, rbhogw) — sentinel call inserted as first executable line of public function; 2 stale `# No sentinel — works pre-kindle` comments removed (zrbho_furnish unconditionally calls zrbho_kindle before public dispatch, so guards are safe). Bucket 6 (redundant enforce check): rbhpb_base.sh:51-52 `test ${#...} -gt 0` line removed (non-load-bearing — `test -n` covers it). Verification: fast suite 90/90 (enrollment-validation 47, regime-validation 21, regime-smoke 7, handbook-render 15 with all 8 onboarding + 4 windows + 3 payor handbooks rendering clean). Total 11 files modified.

### 2026-04-16 09:47 - Heat - S

review-onboarding-director-first-kludge

### 2026-04-16 09:42 - ₢A6AAp - W

Converted 8 `docker ... 2>/dev/null | grep -q` probe sites across rbhob_base.sh (lines 124/130/135/259), rbhofc_first_crucible.sh (83/87/99), and rbhodf_director_first_build.sh (73) to BCG-compliant temp-file stderr capture with while/case parsing. Added kindle prefixes ZRBHO_DOCKER_IMAGES_PREFIX, ZRBHO_DOCKER_PS_PREFIX, ZRBHO_DOCKER_STDERR_PREFIX in zrbho_kindle using BURD_TEMP_DIR; sites use integer discriminators per BCG §Stderr file naming. Reference pattern from existing zrbho_probe_director_units. Verification: grep finds 0 `docker.*2>/dev/null` sites in rbh0/; rbw-o, rbw-Ofc, rbw-Odf all exit 0. Work was completed and committed as 0f5dd529 in an earlier chat; pace left rough and is now being formally wrapped.

### 2026-04-16 09:42 - ₢A6AAo - W

Deleted 6 unreachable legacy role-track functions from Tools/rbk/rbh0/rbhob_base.sh (zrbho_probe_role_credentials, zrbho_probe_retriever_units, zrbho_probe_director_units, zrbho_probe_governor_units, zrbho_probe_payor_units, zrbho_triage_role) — all grep-confirmed zero-caller across .sh files. This finishes half-done cleanup: a placeholder comment banner ("Legacy role-track functions... removed — replaced by intent-organized handbook tracks below") had been added earlier but the actual function bodies were never deleted; that banner now accurately describes file state. zrbho_credential_install retained (2 live callers: rbhocd_credential_director.sh:39, rbhocr_credential_retriever.sh:39). Shared helpers (zrbho_po_status, zrbho_po_extract_capture, zrbho_kindle, zrbho_sentinel) preserved. Verification: fast suite 90/90 pass (enrollment-validation 47, regime-validation 21, regime-smoke 7, handbook-render 15). Pure deletion — ~270 lines removed, no new code.

### 2026-04-16 09:41 - ₢A6AAo - n

Delete 6 unreachable legacy role-track functions from rbh0/rbhob_base.sh — zrbho_probe_role_credentials, zrbho_probe_retriever_units, zrbho_probe_director_units, zrbho_probe_governor_units, zrbho_probe_payor_units, zrbho_triage_role. Grep across all .sh files confirmed zero callers. The existing placeholder banner at lines 354-357 ("Legacy role-track functions... removed — replaced by intent-organized handbook tracks below") now accurately reflects file state; previously the comment had been added but the functions never actually deleted. zrbho_credential_install retained — 2 live callers (rbhocd_credential_director.sh:39, rbhocr_credential_retriever.sh:39). Shared helpers zrbho_po_status, zrbho_po_extract_capture, zrbho_kindle, zrbho_sentinel preserved. Verification: fast suite 90/90 (enrollment-validation 47, regime-validation 21, regime-smoke 7, handbook-render 15).

### 2026-04-16 09:40 - ₢A6AAr - W

Converted 104 yawp capture sites across three payor ceremony handbooks (rbhpe_establish 70, rbhpq_quota_build 28, rbhpr_refresh 6) from mutable-reuse z_ui/z_cmd/z_href pattern to BCG-compliant `local -r` with unique descriptive names. Top-level multi-var declarations removed. Downstream buh_line references updated to match new names. Render output byte-identical vs pre-edit baseline (verified via diff; only expected timestamp and dynamic project-id example vary). Grep hygiene: 0 mutable captures, exactly 104 `local -r` sites, 0 residual declarations. Fast suite green (4 fixtures / 90 cases including all 15 handbook-render cases with all 3 payor handbooks). Executed in parallel with ₢A6AAn via three subagents on file-disjoint targets.

### 2026-04-16 09:40 - ₢A6AAr - n

Convert 104 yawp capture sites in payor ceremony handbooks (rbhpe 70, rbhpq 28, rbhpr 6) from mutable-reuse z_ui/z_cmd/z_href pattern to BCG-compliant `local -r` with unique descriptive names per site; downstream buh_line references updated to new names; top-level multi-var declarations (`local z_ui z_cmd z_href` and establish.sh's five-var variant `z_ui z_ui2 z_cmd z_href z_warn`) removed. Name scheme preserves original type prefix (ui/cmd/href) and appends section/role descriptors to disambiguate repeated literals (e.g. z_ui_next_step1/step2/step3 across wizard steps; z_cmd_app_name_{create,verify,consent,client} for four RBGC_PAYOR_APP_NAME sites; z_href_apis_dashboard_{verify,enable} for twin console hrefs). Render output byte-identical: diff against pre-edit baseline shows only expected timestamp and dynamic project-id example variance. Grep hygiene clean: 0 mutable captures, exactly 104 `local -r z_.*=${z_buym_yelp}` sites across the three files, 0 residual multi-var declarations. Fast suite green (4 fixtures, 90 cases including 15/15 handbook-render with all 3 payor cases). Executed in parallel with ₢A6AAn via three subagents on file-disjoint targets; git status confirms only the three AAr payor files modified.

### 2026-04-16 09:33 - ₢A6AAn - W

Repaired Windows handbook BCG crash and hygiene. Fix 1: rbhw0_cli.sh now sources jjk/jjz_zipper.sh and calls zjjz_kindle, unblocking JJZ_FUNDUS_PHASE1/PHASE2 references in rbhw0_top.sh Phase 3 teaching section (prior unbound-variable crash under set -euo pipefail). Fix 2: rbhw0_top.sh:52 hardcoded string replaced with buh_tt using ${JJZ_FUNDUS_PHASE2}. Fix 3: triple buyy_cmd_yawp ${ZRBHW_WSL_DISTRO} capture at lines 34/46/58 collapsed to single capture at first use, reused thrice via shared z_wsl_distro_yelp. Fix 4: removed unreferenced sources+kindles (buv_validation.sh, rbcc_Constants.sh, rbgc_Constants.sh) from rbhw0_cli.sh furnish (confirmed zero consumers in rbhw* tree via grep). Verification: handbook-render fixture now 15/15 (was 14/15; rbw-hw went green), fast suite (4 fixtures) exits 0, tt/rbw-hw.HandbookWindows.sh renders all four phases cleanly. Observable side effect flagged separately: buh_tt on imprint-channel JJZ_FUNDUS_PHASE2 resolves its glob to jjw-tfP2.ProvisionPhase2.cerebro.sh (first alphabetical match) rather than preserving the prior {host} placeholder teaching signal — captured as ₢A-AAT handbook-render-imprint-coverage in ₣A- for separate treatment, not expanded into this pace's scope.

### 2026-04-16 09:29 - ₢A6AAn - n

Repair Windows handbook BCG crash and hygiene — source jjk/jjz_zipper.sh in rbhw0_cli.sh furnish (unblocks JJZ_FUNDUS_PHASE1/PHASE2 refs in Phase 3 teaching section of rbhw0_top.sh:52-53), remove unreferenced buv/rbcc/rbgc sources+kindles (confirmed zero consumers in rbhw* tree), replace hardcoded tt/jjw-tfP2.ProvisionPhase2.{host}.sh with buh_tt using ${JJZ_FUNDUS_PHASE2}, collapse triple buyy_cmd_yawp ${ZRBHW_WSL_DISTRO} captures at lines 34/46/58 to single capture reused thrice. Verification: handbook-render fixture now 15/15 (was 14/15), fast suite exits 0, rbw-hw renders cleanly. Post-note: buh_tt on imprint-channel JJZ_FUNDUS_PHASE2 resolves to jjw-tfP2.ProvisionPhase2.cerebro.sh (first alphabetical glob match) — observable behavior change vs prior {host} placeholder, flagged for review.

### 2026-04-16 09:18 - ₢A6AAt - W

Landed handbook-render fast-suite fixture covering 15 of 16 handbook colophons (8 onboarding + 4 windows + 3 payor); rbw-HWdw deferred to ₢A-AAS (param1 channel). 15 RBTDRM_COLOPHON_* consts enforce sync with rbz_zipper.sh via rbtdrm_verify at theurge startup. New module rbtdrf_handbook.rs with per-colophon case + 3-section array; wired into lib.rs, dispatch arm in rbtdrc_crucible.rs, fast suite in rbte_engine.sh, tabtarget tt/rbtd-r.Run.handbook-render.sh. Added 2 unit tests in rbtdtm_manifest.rs for the new fixture (53 passing total). Baseline: 14/15 pass; only rbw-hw fails with known JJZ_FUNDUS_PHASE1 unbound variable at rbhw0_top.sh:52 — this failure is the fixture doing its job, not a wrap blocker. Baseline fed into ₢A6AAn docket (Ceiling=Floor; no scope expansion). Fixture runs standalone in ~1.3s and is now gating future handbook changes.

### 2026-04-16 09:13 - ₢A6AAt - n

Add handbook-render Theurge fixture — exercises 15 of 16 handbook display tabtargets (rbw-HWdw deferred to ₢A-AAS for param1 channel), reports per-case pass/fail, wired into fast suite. Baseline: 14/15 pass; rbw-hw fails with known JJZ_FUNDUS_PHASE1 unbound variable at rbhw0_top.sh:52.

### 2026-04-16 09:11 - ₢A6AAp - n

Convert 8 docker probe sites in rbh0/ from `2>/dev/null | grep -q` anti-pattern to BCG-compliant temp-file stderr capture with while/case parsing — preserves forensic evidence when docker fails. Added kindle prefixes (ZRBHO_DOCKER_IMAGES_PREFIX, ZRBHO_DOCKER_PS_PREFIX, ZRBHO_DOCKER_STDERR_PREFIX) in zrbho_kindle using BURD_TEMP_DIR; sites use integer discriminators per BCG §Stderr file naming. Reference pattern from existing zrbho_probe_director_units. rbw-o, rbw-Ofc, rbw-Odf all exit 0.

### 2026-04-16 09:01 - Heat - S

handbook-render-fixture

### 2026-04-16 08:49 - Heat - S

access-probe-5xx-retry

### 2026-04-16 08:33 - ₢A6AAm - n

Add missing _RBGV_VOUCHES_PACKAGE substitution to graft's combined about+vouch build composer — standalone vouch composer had it; combined composer (introduced in ₢A6AAZ) was missing it, causing assemble-push-vouch to fail with unbound variable

### 2026-04-16 07:39 - ₢A6AAg - W

BCG compliance audit conducted across the three decomposed rbh0 groups (onboarding, payor, windows). Findings were transcribed into five actionable fix paces rail-ordered before this one: ₢A6AAn (Windows crash + yawp collapse), ₢A6AAo (delete 6 unreachable legacy role-track functions — audit confirmed zero callers), ₢A6AAp (8 docker stderr-capture sites), ₢A6AAq (mechanical BCG violations across all groups — multi-var, local -r, sentinels, source guards), ₢A6AAr (104 mutable yawp captures → local -r in payor ceremonies). The audit itself produced no code commits; its output lives in the descendant paces.

### 2026-04-16 07:38 - ₢A6AAm - n

Apply BCG capture idiom to curl in build-polling loop — bare curl + \$? check was unreachable under set -euo pipefail, blocking retry logic

### 2026-04-15 15:26 - Heat - S

repair-bcg-payor-yawp-capture

### 2026-04-15 15:26 - Heat - S

repair-bcg-mechanical-sweep

### 2026-04-15 15:25 - Heat - S

repair-bcg-stderr-to-tempfile

### 2026-04-15 15:25 - Heat - S

repair-bcg-dead-function-sweep

### 2026-04-15 15:25 - Heat - S

repair-bcg-windows-crash

### 2026-04-15 15:12 - Heat - T

menu-and-crash-course-draft

### 2026-04-15 15:11 - Heat - r

moved A6AAm to first

### 2026-04-15 15:07 - Heat - D

restring 1 paces from ₣A_

### 2026-04-15 14:54 - ₢A6AAj - W

Audited all buyy_*_yawp + buh_line compositions across Tools/. Added buh_ternary conditional display function. Fixed rbhodf (21 cross-line violations): standalone tabtargets to buh_tt, tabtarget+args to buh_tt with degenerate imprint, probe checkboxes to buh_ternary, all z_buym_yelp to local -r capture. Fixed 27 semicolon one-liners across 7 files (buhw_windows, rbhw0_top, rbgp_Payor, rbhwdd, rbhwcd, rbhwdn, burh_cli). Updated buh_handbook.sh header comment. Zero yawp-capture violations remain.

### 2026-04-15 14:54 - ₢A6AAk - n

Fix RBGC_VOUCHES_PACKAGE from uppercase VOUCHES to lowercase vouches — OCI registries reject uppercase package names, breaking vouch step of every conjure

### 2026-04-15 14:51 - Heat - r

moved A6AAl to first

### 2026-04-15 14:51 - Heat - S

rbro-payor-subdirectory-migration

### 2026-04-15 14:48 - ₢A6AAj - n

Fix all remaining BCG yawp-capture violations across 7 files — every buyy_*_yawp now captures to local -r on the same semicolon line; update buh_handbook.sh header comment to show correct pattern

### 2026-04-15 14:45 - ₢A6AAj - n

Add buh_ternary conditional display function; fix all BCG yawp-capture violations in rbhodf — standalone tabtargets converted to buh_tt, tabtarget+args use degenerate imprint, probe checkboxes use buh_ternary with pass/fail yawps, all remaining cross-line z_buym_yelp usage captured to local -r

### 2026-04-15 14:43 - ₢A6AAi - n

Rename rbap (Access Probe) to rbgv (Google Verification) — file, all internal functions/constants/guards, three consumers, spec linked terms, CLAUDE.md mapping, paddock reference

### 2026-04-15 14:43 - Heat - S

vouches-package-lowercase

### 2026-04-15 14:26 - ₢A6AAh - W

Decomposed three monolithic handbook files (rbho_onboarding.sh ~1700 lines, rbhp_payor.sh ~430 lines, rbhw_windows.sh ~200 lines) into 22 one-function-per-file under Tools/rbk/rbh0/. Three parallel sonnet agents extracted with exclusive file ownership. CLI files renamed with 0 suffix (rbho0_cli.sh, rbhp0_cli.sh, rbhw0_cli.sh) to fix terminal exclusivity violations. Windows CLI gained missing buym_yelp and zipper dependencies exposed by testing (pre-existing gaps from yelp migration). Zipper paths updated, originals deleted, CLAUDE.md mappings refreshed. All handbook tabtargets render correctly; fast suite 75/75 green. Known pre-existing spook: rbhw_top references JJZ_FUNDUS_PHASE1 cross-kit constant not wired in windows CLI.

### 2026-04-15 14:25 - ₢A6AAh - n

Decompose three monolithic handbook files into 22 one-function-per-file under Tools/rbk/rbh0/; fix CLI minting violations with 0 suffix; wire zipper paths; fix pre-existing windows CLI dependency gaps exposed by testing

### 2026-04-15 14:10 - Heat - r

moved A6AAj after A6AAh

### 2026-04-15 14:09 - Heat - S

eliminate-buh-line-yawp-one-liners

### 2026-04-15 14:06 - Heat - n

Add crucible conduit and credential confinement roadmap items, augment VPC Service Controls with conduit cross-reference, fix 15 one-sentence-per-line violations across entire README

### 2026-04-15 13:55 - Heat - r

moved A6AAh to first

### 2026-04-15 13:52 - ₢A6AAf - W

Swept all legacy combinator callers across 6 files (rblm_cli, rbgp_Payor, burh_cli, rbhw_windows, buhw_windows, buh_handbook) converting buh_t/c/tc/tu/tut/tct/tT/tTc to yelp-style buh_line/code/tt + yawp captures. Deleted 26 combinator functions, 4 internal helpers (zbuh_show, zbuh_link_fragment, zbuh_tabtarget_fragment, zbuh_imprint_fragment), associated state variables, and 5 unused color constants from buh_handbook.sh. Updated buh_critical and buh_prompt_required to use direct printf. Replaced all hardcoded colophon strings in buh_tt calls with zipper constants. Rewrote butclc_LinkCombinator test fixture to exercise retained buh_link. BUK self-test 25/25, fast suite 75/75.

### 2026-04-15 13:52 - ₢A6AAf - n

Remove BUH text/link/tabtarget combinator system; convert all handbook callers to yelp-aware buh_line/buh_code/buh_tt with yawp captures. Eliminates zbuh_show, zbuh_link_fragment, zbuh_tabtarget_fragment, 40+ combinator functions, and 5 unused color constants. Test cases updated to exercise buh_link directly.

### 2026-04-15 13:41 - Heat - n

crucible conduit architecture: horizon roadmap nodes for WireGuard tunnel, cloud service nameplate, credential confinement; design exploration memo with approach comparison and service compatibility survey

### 2026-04-15 13:32 - ₢A6AAe - W

Converted payor handbook (rbhp_payor.sh) from combinator style to pure yelp style. All 15 buh_link calls now use buyy_href_yawp captures — first production exercise of the HREF diastema path. Fixed ~10 silently-dropped suffixes from buh_tu 3-arg misuse. Added zbuym_sentinel to zbuh_kindle to prevent missing-kindle bugs across all handbook CLIs.

### 2026-04-15 13:31 - ₢A6AAe - n

Convert payor handbook from combinator style to pure yelp style: 15 buh_link calls become buyy_href_yawp captures, all buh_t/tc/tu/tut/tctu/tutu/tW converted to yawp+buh_line idiom, buh_c to buh_code. Fixed ~10 silently-dropped suffixes from buh_tu 3-arg bugs. Added buym_yelp sourcing to rbhp_cli.sh and zbuym_sentinel to zbuh_kindle to prevent missing-kindle class of bug.

### 2026-04-15 13:16 - Heat - S

rename-rbap-to-rbgv

### 2026-04-15 13:12 - Heat - d

paddock curried: yelp migration status, decomposition strategy, critical path, compress provenance

### 2026-04-15 13:07 - Heat - r

moved A6AAM after A6AAZ

### 2026-04-15 13:07 - Heat - r

moved A6AAH after A6AAW

### 2026-04-15 13:07 - Heat - r

moved A6AAZ after A6AAX

### 2026-04-15 13:07 - Heat - r

moved A6AAI after A6AAh

### 2026-04-15 13:07 - Heat - r

moved A6AAh after A6AAA

### 2026-04-15 13:07 - Heat - r

moved A6AAA after A6AAg

### 2026-04-15 13:06 - Heat - r

moved A6AAg after A6AAf

### 2026-04-15 12:59 - Heat - S

decompose-rbh-into-rbh0-directory

### 2026-04-15 12:59 - ₢A6AAd - W

Remove legacy capture API (buy_*_capture, ZBUY_*, zbuy_* aliases) from buym_yelp.sh — zero external consumers remained; update BUS0 spec to remove capture quoin and correct yawp count to 8; canonize one-line semicolon yawp+capture idiom across rbho_onboarding.sh (16 pairs converted), buym_yelp.sh headerdocs, and BCG pattern section; BUK self-test 25/25 pass

### 2026-04-15 12:57 - ₢A6AAd - n

Canonize one-line semicolon yawp+capture idiom: convert 16 multiline pairs in rbho_onboarding.sh, update buym_yelp.sh headerdocs, add BCG pattern with AVOID example for old two-line form

### 2026-04-15 12:48 - ₢A6AAd - n

Remove legacy capture API (buy_*_capture, ZBUY_*, zbuy_* aliases, buy_configure_*) from buym_yelp.sh — zero external consumers remain; update spec to remove capture quoin and correct yawp count to 8

### 2026-04-15 12:39 - ₢A6AAd - n

Convert rbho_payor_handbook and rbho_governor_handbook to yelp API; fix buh_tltltlt bug dropping Retriever/Director text; eliminate z_docs locals in favor of RBYC_* kindled constants

### 2026-04-15 12:29 - ₢A6AAd - n

Add pass/warn/fail yelp yawps (green/orange/red) with BUYC palette and diastema resolution; kindle RBYC_PROBE_YES and RBYC_PROBE_NO constants; convert rbho_first_crucible old combinators to yelp API; update zrbho_po_status to use kindled probe constants

### 2026-04-15 12:05 - ₢A6AAd - n

Add buh_tt wrapper and buyy_tt_yawp args parameter; fix capture discipline in converted functions (start_here, crash_course, credential_install, credential_retriever, credential_director); first_crucible buy_cmd_capture migrated but old combinators remain

### 2026-04-15 11:56 - ₢A6AAd - n

Yawp capture discipline: BCG per-function template with → arrow, all 5 buyy_* headerdocs updated, section header trimmed; RBYC_RBRO and RBYC_RECIPE_BOTTLE added; rbho_start_here, rbho_crash_course, credential functions converted to yelp (capture discipline violations remain — pending buh_ helper discussion)

### 2026-04-15 11:39 - Heat - S

review-rbho-bcg-compliance

### 2026-04-15 11:32 - ₢A6AAc - W

Diastema wire format and yawp infrastructure: buym_yelp.sh with 8 STX-prefixed markers, 5 yawp functions, buyf_format_yawp BASH_REMATCH resolver, BUYC_* palette, buyc_* configurators; buh_line/code/warn/error semantic line functions; buh_section/step1/step2 yelp-aware for linked headers; RBYC vocabulary converted from buy_link_capture to buyy_link_yawp diastema constants; rbho_director_first_build fully converted to pure yelp style; 7-case buym-yelp test fixture (25/25 suite green); discovered bash SOH reservation in BASH_REMATCH

### 2026-04-15 11:32 - ₢A6AAc - n

Convert handbook section/step headers to yelp format via buyf_format_yawp; wire linked terms into director-first-build step1

### 2026-04-15 11:32 - Heat - S

combinator-sweep-and-deletion

### 2026-04-15 11:27 - Heat - S

payor-handbook-yelp-conversion

### 2026-04-15 11:26 - Heat - S

onboarding-handbook-yelp-conversion

### 2026-04-15 11:18 - ₢A6AAc - n

Add 7-case buym-yelp test fixture (cmd resolve, link OSC-8, link fallback, ambient preservation, fast path, multi-marker, plain mode); fix diastema prefix byte from \x01 to \x02 — bash uses SOH internally for BASH_REMATCH group delimiting; drop dead z_docs local

### 2026-04-15 11:10 - Heat - n

Add Appendix: Build Isolation section covering airgap security rationale (exfiltration prevention, curated gate principle, base image honesty, regulatory alignment), link egress-locked mentions to new #BuildIsolation anchor

### 2026-04-15 11:07 - ₢A6AAc - n

Fix buyf_format_yawp: remove inner quotes from bash parameter substitution patterns (literal quote chars were appearing in output)

### 2026-04-15 11:06 - Heat - n

Add Appendix: Software Bill of Materials section defining SBOM and container hygiene value, link all 8 existing SBOM mentions to new #SBOM anchor

### 2026-04-15 11:04 - ₢A6AAc - n

Convert RBYC vocabulary from buy_link_capture to buyy_link_yawp (diastema-marked constants), add RBYC_OUTPUT, replace local z_lk_* captures in rbho_director_first_build with RBYC_* constants

### 2026-04-15 11:01 - Heat - n

Add Appendix: Supply Chain Provenance section defining SLSA provenance guarantees, link all 18 existing provenance/SLSA mentions to new #Provenance anchor

### 2026-04-15 11:01 - ₢A6AAc - n

Diastema wire format and yawp infrastructure: buym_yelp.sh with 8 diastema markers, 5 yawp functions, buyf_format_yawp resolver, BUYC_* palette, buyc_* configurators, legacy capture compatibility; buh_line routes through format_yawp, add buh_code/warn/error semantic line functions; convert rbho_director_first_build to pure yelp style; delete dead buy_yelp.sh; update BUS0 quoins

### 2026-04-15 10:43 - Heat - n

Add _yawp function category to BCG (pure-computation group return via assignment), evict unimplemented _litmus_predicate pattern

### 2026-04-15 10:38 - Heat - T

diastema-yawp-infrastructure

### 2026-04-15 09:07 - ₢A6AAb - W

Eliminate dead rubric quoins from RBS0 and RBSTB, add RBF_FACT_RELIQUARY constant and buf_write_fact emission in rbfl_inscribe, create RBSDI-depot_inscribe.adoc operation spec, add CLAUDE.md acronym entry

### 2026-04-15 09:06 - ₢A6AAa - W

Built yelp infrastructure: buy_yelp.sh (4 capture functions, 3 configurators, no-% premise), rbyc_common.sh (46 base + 10 variant vocabulary constants), buh_line with direct interpolation. Converted rbho_first_crucible to pure yelp+constants style. Added Yelp/Capture quoins to BUS0. Discovery: printf %b double-interprets OSC-8 ST sequences — resolved by direct interpolation design. Slated ₢A6AAc for sentinel/format_capture architecture and legacy combinator elimination.

### 2026-04-15 09:04 - Heat - S

sentinel-format-and-legacy-elimination

### 2026-04-15 08:38 - ₢A6AAa - n

Simplify buh_line to direct interpolation, add Yelp/Capture quoins to BUS0 with no-% premise, remove format-string machinery

### 2026-04-15 08:25 - ₢A6AAa - n

yelp-and-vocabulary-infrastructure: add buy_yelp.sh capture functions, rbyc_common.sh vocabulary constants (46 base + 10 variants), buh_line with %b-to-%s safety mapping, convert rbho_first_crucible to yelp+constants style

### 2026-04-15 08:22 - ₢A6AAb - n

Eliminate dead rubric quoins from RBS0 and RBSTB, add RBF_FACT_RELIQUARY constant and buf_write_fact call in rbfl_inscribe, create RBSDI-depot_inscribe.adoc operation spec for reliquary creation

### 2026-04-15 08:11 - Heat - S

reliquary-fact-and-rubric-elimination

### 2026-04-15 08:05 - Heat - S

yelp-and-vocabulary-infrastructure

### 2026-04-15 06:58 - ₢A6AAZ - n

VOUCHES superdirectory and GAR-direct plumb: add RBGC_VOUCHES_PACKAGE constant, dual-tag vouch into VOUCHES package, rewrite plumb from local docker to Registry API with Retriever auth and hallmark-only mode, update four spec documents

### 2026-04-15 06:51 - Heat - n

Revise First Crucible handbook: add Nameplate/Hallmark/Bottle/Kludge links, reword kludge description and clean-tree warning, restructure nameplate definition with container role links

### 2026-04-15 06:38 - Heat - S

plumb-goes-remote-with-vouches-superdirectory

### 2026-04-15 06:31 - ₢A6AAO - W

Fixed director credential onboarding track: added rbev-busybox vessel argument to rekon command so learner sees complete working invocation

### 2026-04-15 06:30 - ₢A6AAO - n

Fix director credential track rekon command: add rbev-busybox vessel argument so learner sees complete working command

### 2026-04-15 06:24 - ₢A6AAN - W

Implemented rbw-Ocr OnboardingCredentialRetriever handbook track — retriever RBRA placement, verification, and tally access confirmation following Crash Course pattern

### 2026-04-15 06:16 - Heat - n

Demote rbfl_tally per-vessel query and no-hallmarks messages from step/info to debug level

### 2026-04-14 15:52 - ₢A6AAU - n

Revise Summon description (set of images, Vouched Hallmark, image cache), add Charge/Hallmarks/Summon links, remove redundant step comments

### 2026-04-14 15:45 - ₢A6AAU - n

Remove redundant step-numbering comments that duplicate buh_step1/buh_step2 output (BCG: no non-load-bearing elements)

### 2026-04-14 15:33 - ₢A6AAU - n

Add step 3 (Capture the hallmark) with ONBOARD_VESSEL/ONBOARD_HALLMARK env vars, update Plumb/Summon/Abjure displays to use them, renumber steps 3→4 4→5 5→6

### 2026-04-14 15:27 - ₢A6AAU - n

Move pouch first in artifact tour, expand definition: FROM SCRATCH OCI image, contents, tether/airgap invariance

### 2026-04-14 15:24 - ₢A6AAU - n

Reword vouch recovery line: reattempt Vouch on untreated Hallmarks, both linked

### 2026-04-14 15:23 - ₢A6AAU - n

Fix multi-link buh_tlt calls (RBRR/Depot, Hallmark/Vouch) and capitalize+link all remaining unlinked vocabulary terms in director first build handbook

### 2026-04-14 15:20 - ₢A6AAU - n

Fix buh_tlt→buh_tltlt for two-link lines in step 5 (Rekon/Vessel, Abjure/Hallmark) that were silently dropping second linked term

### 2026-04-14 15:18 - ₢A6AAU - n

Capitalize all vocabulary anchor link display text (Crucible, Vessel, Hallmark, Sentry, Bottle, Nameplate, Conjure, Abjure) across onboarding handbook tracks

### 2026-04-14 15:13 - ₢A6AAU - n

Add vocabulary links for hallmark, vessel, abjure, and Depot in step 5 (Abjure and Rekon) of director first build handbook

### 2026-04-14 15:05 - ₢A6AAU - n

Add hallmark and conjure vocabulary links in step 3 tally/vouch substeps

### 2026-04-14 15:03 - Heat - n

Fix tally platform display: read platforms from attest-type tags instead of defunct image-{arch} tags, so multi-platform hallmarks show amd64,arm64,armv7 instead of single

### 2026-04-14 15:01 - ₢A6AAS - W

End-to-end conjure pipeline smoke test passed: ordain rbev-busybox (3-platform, conjure mode), verified 8-tag inventory (image, about, vouch, diags, pouch, 3x attest-{arch}), tally shows vouched health, summon pulls consumer image, docker run proves viability, abjure deletes all 8 tags cleanly. No legacy tag patterns. Attest tags confirmed durable per AAY fix. Pipeline is correct.

### 2026-04-14 14:59 - ₢A6AAU - n

Split rbev-sentry-debian-slim into tether/airgap vessel variants sharing common-sentry-context build directory, update all nameplate and onboarding references

### 2026-04-14 14:39 - ₢A6AAU - n

Refine director first build handbook step 3: add -attest-{arch} to artifact tour (durable, shared layers, GCB-attested digests), move after -image, use constants for cyan display, fix vouch description to teach standalone recovery for interrupted builds

### 2026-04-14 14:39 - ₢A6AAS - n

BCG-compliant hallmark listing in summon: temp files instead of pipelines, case instead of grep, parameter expansion instead of sed, local -r for constants, stderr capture; fix buc_bare misuse for file output

### 2026-04-14 14:33 - ₢A6AAS - n

Add rbfc_require_vessel_sigil() to list available vessels on missing/invalid vessel arg; replace raw buc_die in rbfr_summon and zrbfc_plumb_core; source rbrv_regime.sh in rbfr_cli.sh

### 2026-04-14 14:15 - ₢A6AAY - W

Fix vouch verifier to resolve provenance-carrying digests from -attest-{arch} tags instead of -image manifest index; remove sweep function and call (attest tags are durable, not ephemeral); add _RBGV_ARK_SUFFIX_ATTEST substitution; update RBS0 spec (lifecycle Durable, vouch provenance resolution, abjure includes attest); add attest tag steps to RBSAA abjure spec; update comments across foundry modules

### 2026-04-14 14:15 - ₢A6AAY - n

Promote -attest-{arch} tags from ephemeral to durable: remove post-vouch sweep, resolve vouch provenance from attest tags (not -image manifest index) to get GCB-attested digests, extend abjure to delete attest artifacts for conjure vessels

### 2026-04-14 14:07 - ₢A6AAS - n

Document digest identity gap: classic-store pull/push re-serializes manifests, GCB provenance attaches to daemon-pushed digests only, decision to keep -attest-{arch} as durable tags

### 2026-04-14 14:06 - Heat - S

attest-digest-identity-repair

### 2026-04-14 13:41 - ₢A6AAS - n

Fix about pipeline syft step: unify multi-platform scan to digest pinning for all modes (conjure included), eliminating dependency on per-platform -image-{arch} tags that no longer exist

### 2026-04-14 13:41 - ₢A6AAU - n

Fix five nits: link Depot/Vessel/Sentry terms, add reliquary refresh recommendation, add RBRV_RELIQUARY installation substep after inscribe

### 2026-04-14 13:35 - ₢A6AAU - n

Implement director first cloud build onboarding track (rbw-Odf): zipper enrollment, thin dispatcher, rbho_director_first_build() with 5-step conjure lifecycle teaching arc, prerequisite probes, RBGC_ARK_SUFFIX constants throughout, tabtarget link in start_here menu

### 2026-04-14 13:33 - ₢A6AAQ - W

Renamed build context tag from context-{timestamp} to {hallmark}-pouch via RBGC_ARK_SUFFIX_POUCH constant, plumbed pouch into tally classifier and abjure (check/delete/confirm/display), reordered hallmark minting before context push, fixed BCG violation in sweep function (temp file instead of $() capture), updated RBSAA-ark_abjure.adoc sub-spec with pouch artifact.

### 2026-04-14 13:33 - ₢A6AAQ - n

Add director first-build onboarding walkthrough — enroll rbw-Odf tabtarget, wire into onboarding start-here menu

### 2026-04-14 13:32 - ₢A6AAQ - n

Fix BCG violation in sweep function (temp file instead of command substitution), add pouch to RBSAA-ark_abjure.adoc sub-spec (tag construction, existence check, deletion step)

### 2026-04-14 13:27 - ₢A6AAQ - n

Rename build context tag from context-{timestamp} to {hallmark}-pouch via RBGC_ARK_SUFFIX_POUCH constant, plumb pouch into tally classifier and abjure deletion set, reorder hallmark minting before context push

### 2026-04-14 13:22 - ₢A6AAR - W

Refactored conjure pipeline: host-side hallmark minting via _RBGY_HALLMARK substitution, promoted buildx --push output to consumer-facing -image tag (eliminated -multi intermediate and imagetools-create step), renamed per-platform tags to ephemeral -attest-{arch} with post-vouch sweep by director credentials, updated abjure/tally for new tag scheme, reduced pipeline from 7 to 6 steps. Added RBGC_ARK_SUFFIX_ATTEST constant.

### 2026-04-14 13:21 - ₢A6AAR - n

Refactor conjure pipeline: host-side hallmark minting via _RBGY_HALLMARK, promote buildx output to -image (eliminate -multi and imagetools-create), rename per-platform tags to ephemeral -attest-{arch} with post-vouch sweep, update abjure/tally for new tag scheme, reduce pipeline from 7 to 6 steps

### 2026-04-14 13:13 - Heat - r

moved A6AAH before A6AAU

### 2026-04-14 13:13 - Heat - S

onboarding-director-graft

### 2026-04-14 13:12 - Heat - S

onboarding-director-bind

### 2026-04-14 13:12 - Heat - S

onboarding-director-airgap

### 2026-04-14 13:11 - Heat - S

onboarding-director-first-cloud-build

### 2026-04-14 13:11 - ₢A6AAT - W

Revised RBS0 conjure pipeline spec: added rbtga_ark_pouch and rbtga_ark_diags quoins with formal definitions, added kludge hallmark mode (k-prefix with git-context), replaced tag inventory with 6-entry table (5 durable + ephemeral attest-arch scaffolding), rewrote multi-platform pipeline from imagetools-create to buildx-push+pullback, documented _RBGY_HALLMARK host-minting throughout, formalized SLSA attestation scaffolding pattern and post-vouch sweep, updated abjure to cover all 5 artifact suffixes, replaced all bare pouch references with quoin. Reslated A6AAQ to note spec prose already complete.

### 2026-04-14 13:10 - Heat - T

guided-track-buildout

### 2026-04-14 13:10 - ₢A6AAJ - W

Decision: bind mode gets a dedicated director subtrack (rbw-Odb) using bottle-plantuml. Evaluator coverage in Assay: Supply Chain (track 13) is insufficient alone — directors need a hands-on walkthrough. Resolves the bind-mode-coverage gap in the paddock.

### 2026-04-14 13:04 - ₢A6AAT - n

Revise RBS0 conjure pipeline spec: add pouch/diags quoins, kludge hallmark mode, target tag inventory with ephemeral attest-arch scaffolding, rewrite multi-platform pipeline from imagetools-create to buildx-push+pullback, formalize _RBGY_HALLMARK host-minting

### 2026-04-14 12:54 - Heat - S

conjure-pipeline-spec-revision

### 2026-04-14 12:23 - Heat - S

conjure-pipeline-smoke-test

### 2026-04-14 12:16 - Heat - S

host-side-conjure-hallmark

### 2026-04-14 12:09 - Heat - S

context-to-pouch-rename

### 2026-04-14 12:02 - ₢A6AAO - n

Enhance rekon error path to show available vessels when moniker is omitted, fix buc_error→buc_warn

### 2026-04-14 11:55 - ₢A6AAN - n

Soften SLSA provenance bullet to 'inspect images and SLSA' for broader accuracy

### 2026-04-14 11:47 - Heat - n

Headline multiplatform images (x86 + ARM) in Foundry bullet, soften SLSA to 'optional'

### 2026-04-14 10:45 - ₢A6AAL - n

Delete legacy role-track onboarding (6 tabtargets, 6 zipper enrollments, ~600 lines of triage/reference/walkthrough functions), replace Director subtracks with four intent-organized tracks (First Cloud Build, Airgap, Bind/PlantUML, Graft), reorder StartHere menu items (description before tabtarget, credential caveats, bullet points, linked terms), add Pouch to tour line

### 2026-04-14 10:16 - Heat - n

StartHere menu: fix Station→BURS broken anchor, add Charter/Knight parenthesized links in governor handbook, indent crucible steps with 'Steps:' header

### 2026-04-14 10:00 - Heat - n

Tighten governor handbook: collapse header into Step 1, simplify Charter/Knight lines to 'Create a Retriever/Director', fix buh_tlt→buh_tltlt for two-link line

### 2026-04-14 09:53 - Heat - n

Rename rbro-payor.env to rbro.env, create RBCC_rbro_file constant as single source of truth for filename, eliminate all hardcoded references

### 2026-04-14 09:48 - ₢A6AAL - n

Fix payor handbook step 3: use 'container repository' in intro vs 'Artifact Registry repository' in action line for precision, fix buh_tlt→buh_tltltlt bug that was dropping Governor link and 'can create' text

### 2026-04-14 09:43 - ₢A6AAL - n

Rebalance payor handbook step 3: introduce all terms (Depot, Governor, Retriever, Director) relationally at top, then pure action blocks with linked role names, clean handoff paragraph

### 2026-04-14 09:38 - Heat - n

Fix payor handbook Step 2 wording: connect OAuth credential install to the JSON file downloaded in Step 1 rather than sounding like a separate task

### 2026-04-14 09:36 - ₢A6AAL - n

Simplify payor handbook step 3+4: merge into single 'Provision the Depot' step, replace levy/charter/knight verbs with plain create language, cut payor-funds-infrastructure phrase

### 2026-04-14 09:32 - Heat - n

Introduce Manor term: Payor's administrative seat holding billing, OAuth, and operator identity. Updated README (anchor + Payor/Depot/RBRP/Establish definitions), payor and governor handbooks, StartHere menu, and zipper descriptions to use Manor consistently as 'has a GCP project' not 'is one'

### 2026-04-14 09:12 - Heat - n

Regenerated tabtarget context to include new rbw-Op and rbw-Og colophons

### 2026-04-14 09:08 - Heat - n

Fix payor/governor handbooks: reorder payor steps (project creation before OAuth), reinstate 15-minute estimate, capitalize all link display text, convert bare prose references to buh_tlt links

### 2026-04-14 09:04 - Heat - n

Add payor and governor handbook tracks (rbw-Op, rbw-Og) in linear step style with no conditional probes, update StartHere menu to reference them

### 2026-04-14 09:01 - ₢A6AAN - n

Refine credential onboarding teaching: replace Governor tabtarget reference with relational sentence (Governor produces RBRA for Directors and Retrievers), add Rekon anchor to README.md, consolidate multi-line linked text into single sentences

### 2026-04-14 08:51 - ₢A6AAN - n

Add live-access verification steps to credential onboarding tracks: retriever confirms via Tally (rbw-ft), director confirms via Rekon (rbw-ir). Extracted generic 'Next steps' from shared helper so each track owns its closing steps. Added Rekon anchor to README.md public docs.

### 2026-04-14 08:49 - Heat - n

Fix StartHere menu 'Create Payor and Depot' section: replaced three unlinked descriptive items with two direct buh_tT links to existing guided tracks (RBZ_ONBOARD_PAYOR, RBZ_ONBOARD_GOVERNOR)

### 2026-04-14 08:38 - ₢A6AAD - W

Delivered ccyolo yolo mode (bypassPermissions settings.json baked into image), sample project (buggy word counter + README seeded into persistent Docker volume via entrypoint), handbook Step 4 update with autonomy explanation and first interaction suggestion. Pinned Claude Code to v2.1.89 to work around OAuth paste regression in later versions (documented in handbook with issue link). Enrolled rbw-cS SSH tabtarget in zipper with rbob_ssh function (fixed pre-existing qualification failure, used nolog launcher for TTY preservation). Added platform.claude.com to DNS allowlist. Also notched RBRV_USER vessel regime parameter (user's work) — single source of truth for container runtime user consumed by compose, SSH, and rack.

### 2026-04-14 08:37 - ₢A6AAP - W

Foundry-image credential refactor: moved tally to retriever credentials, wrest to director credentials, added new rekon operation (director-only raw GAR tag listing). Updated specs (RBSCL, RBSIW, RBS0 section reorganization, new RBSIR), code (rbfl/rbfr credential swaps, rbfl_rekon function), zipper enrollment, tabtarget renames. Image group now fully director-only; tally is the retriever credential check. All verified live against GCP.

### 2026-04-14 08:33 - ₢A6AAD - n

Drive kludge hallmark k260414083313-ce8b8648 (with RBRV_USER) into ccyolo nameplate

### 2026-04-14 08:31 - ₢A6AAD - n

Add RBRV_USER vessel regime parameter: single source of truth for container runtime user, consumed by compose fragments, SSH, and rack. Eliminates hardcoded usernames from compose.yml and rbob_ssh.

### 2026-04-14 08:18 - ₢A6AAP - n

Code changes for credential refactor: tally to retriever, wrest to director, new rekon function and tabtarget, tabtarget renames

### 2026-04-14 08:17 - ₢A6AAD - n

Document Claude Code version pin and paste regression in handbook Step 4 with issue link

### 2026-04-14 08:13 - ₢A6AAD - n

Drive kludge hallmark k260414081338-cda96f9e (pinned v2.1.89) into ccyolo nameplate

### 2026-04-14 08:13 - ₢A6AAD - n

Pin Claude Code to v2.1.89 — v2.1.105 has bracketed paste regression breaking OAuth entry through SSH

### 2026-04-14 07:59 - Heat - n

Vessel render/validate now show available sigil list on missing folio, matching nameplate UX pattern

### 2026-04-14 07:58 - ₢A6AAD - n

Add platform.claude.com to DNS allowlist for Claude Code startup

### 2026-04-14 07:57 - ₢A6AAD - n

Fix SSH tabtarget TTY: use nolog launcher to bypass dispatch tee pipe, add -t flag to ssh

### 2026-04-14 07:51 - ₢A6AAD - n

Enroll rbw-cS SSH tabtarget in zipper with rbob_ssh function, fixing pre-existing qualification failure

### 2026-04-14 07:47 - ₢A6AAD - n

Drive kludge hallmark k260414074732-1cc3e403 into ccyolo nameplate for smoke test

### 2026-04-14 07:32 - ₢A6AAP - n

Spec updates for foundry-image credential refactor: tally to retriever, wrest to director, new rekon operation, RBS0 section reorganization

### 2026-04-14 07:31 - ₢A6AAD - n

Add yolo mode (bypassPermissions settings.json), sample project with buggy word counter, persistent workspace volume, and handbook Step 4 update for ccyolo bottle

### 2026-04-14 07:25 - Heat - T

interview-reconcile-post-fc

### 2026-04-14 07:24 - Heat - T

ccyolo-yolo-mode-and-sample-content

### 2026-04-14 07:23 - ₢A6AAC - W

Drafted and implemented First Crucible handbook (rbho_first_crucible) as working Frame 4-refined function: 6-step teaching flow (kludge sentry/bottle, drive hallmarks, charge, SSH, verify containment, quench), inline probes for Docker/nameplate/hallmark/charged state, vocabulary-first linked terms throughout. Added Reference Nameplates section to README.md with ccyolo and tadmor anchors. Updated reference tree with ccyolo vessel and nameplate. Implementation went beyond draft into working code — tabtarget runs clean, probes functional.

### 2026-04-14 07:19 - ₢A6AAC - n

Simplify docker exec vs SSH explanation: add laggy note, drop SIGWINCH details

### 2026-04-14 07:18 - ₢A6AAC - n

Capitalize and link Kludge in iteration loop step 2

### 2026-04-14 07:16 - ₢A6AAC - n

Step 5: capitalize and link Bottle, soften to 'you can test'

### 2026-04-14 07:14 - ₢A6AAC - n

Split SSH tabtarget to indented sub-line matching iteration loop pattern

### 2026-04-14 07:13 - ₢A6AAC - n

Drop terminal Quench from iteration loop — Charge auto-quenches prior state (compose down before up)

### 2026-04-14 07:08 - ₢A6AAC - n

Iteration loop: link Kludge, Bottle, Hallmark, Charge, Quench, Sentry as capitalized anchors; remove redundant cloud disclaimer

### 2026-04-14 07:07 - ₢A6AAC - n

Capitalize Sentry and make Nameplate a linked term in containment verification paragraph

### 2026-04-14 07:05 - ₢A6AAC - n

Convert remaining bare ccyolo references in onboarding teaching text to buh_tlt linked terms

### 2026-04-14 07:04 - ₢A6AAC - n

Add Reference Nameplates section (ccyolo, tadmor) to README with anchors; add ccyolo vessel to reference tree; upgrade handbook to use buh_tlt links for ccyolo

### 2026-04-14 07:03 - Heat - S

foundry-image-credential-refactor

### 2026-04-14 06:49 - ₢A6AAC - n

Fix iteration loop: rbw-cKB does not auto-commit, learner must commit hallmark drive explicitly

### 2026-04-13 18:05 - ₢A6AAC - n

Draft First Crucible handbook (rbho_first_crucible) — Frame 4-refined format with 6 steps, inline probes, vocabulary-first teaching, iteration loop summary

### 2026-04-13 18:04 - Heat - S

onboarding-credential-director

### 2026-04-13 18:02 - Heat - S

onboarding-credential-retriever

### 2026-04-13 17:59 - Heat - r

moved A6AAD after A6AAC

### 2026-04-13 17:59 - Heat - r

moved A6AAC to first

### 2026-04-13 17:57 - ₢A6AAG - W

Created ccyolo vessel (conjure mode, node:lts-slim base, Claude Code via npm) and nameplate (sentry-debian-slim + ccyolo bottle, Anthropic DNS/CIDR allowlist). Verified kludge→charge→OAuth authenticate cycle. Retired CCCK (directory, CLAUDE.md mappings, zipper enrollments).

### 2026-04-13 17:52 - Heat - S

agent-learner-eval-prototype

### 2026-04-13 17:52 - Heat - S

guided-track-buildout

### 2026-04-13 17:50 - Heat - r

moved A6AAA to last

### 2026-04-13 17:50 - Heat - r

moved A6AAG to first

### 2026-04-13 17:29 - ₢A6AAA - n

Fix buh_tltl typo in rbho_start_here — nonexistent function call, should be buh_tltlt with trailing empty arg

### 2026-04-13 17:17 - ₢A6AAG - n

Add SSH entry tabtarget for ccyolo bottle (reads port from RBRN, hardcoded claude user)

### 2026-04-13 16:12 - ₢A6AAG - n

Drive bottle hallmark k260413161216-99e62002 (with ca-certificates) into ccyolo nameplate

### 2026-04-13 16:12 - ₢A6AAG - n

Document each apt package with inline comments per sentry Dockerfile convention

### 2026-04-13 16:10 - ₢A6AAG - n

Add ca-certificates to Dockerfile, fix example.com CIDRs to Cloudflare ranges

### 2026-04-13 16:06 - ₢A6AAG - n

Fix CIDR allowlist to Anthropic range (160.79.104.0/23), add example.com as containment test target

### 2026-04-13 15:55 - ₢A6AAG - n

Drive sentry hallmark k260413155458-f9dcb6d9 into ccyolo nameplate

### 2026-04-13 15:54 - ₢A6AAG - n

Drive bottle hallmark k260413155220-2418e726 into ccyolo nameplate

### 2026-04-13 15:52 - ₢A6AAG - n

Fix rbob furnish: source buf_fact.sh so buf_write_fact is available for kludge/ordain paths

### 2026-04-13 15:16 - ₢A6AAG - n

Fix rbob furnish: zrbfd_kindle already calls zrbfc_kindle internally, consolidate kludge+ordain cases

### 2026-04-13 15:16 - ₢A6AAG - n

Fix rbob furnish: add missing zrbfd_kindle for kludge path (rbfd_kludge calls zrbfd_sentinel)

### 2026-04-13 15:13 - ₢A6AAG - n

Remove CCCK launcher, tabtarget, and directory (retirement)

### 2026-04-13 15:13 - ₢A6AAG - n

Create ccyolo vessel (node:22-slim, SSH, Claude Code via npm) and nameplate (entry 2222→22, Anthropic DNS/CIDR allowlist, 10.242.3.0/24), retire CCCK

### 2026-04-13 14:49 - Heat - r

moved A6AAG before A6AAC

### 2026-04-13 14:38 - ₢A6AAA - n

Add Rack anchor, RBRR anchor in payor description, First Crucible tabtarget reference in StartHere menu

### 2026-04-13 14:31 - ₢A6AAA - n

Add First Crucible onboarding tabtarget/zipper/stub, add Configure local network substep, fix ccyolo Bottle to ccyolo Crucible

### 2026-04-13 14:28 - ₢A6AAA - n

Column-align em dashes in Director subtracks menu section

### 2026-04-13 14:26 - ₢A6AAA - n

Add buh_tltltltlt 4-link combinator, rewrite Crucible substeps with linked Sentry/Pentacle/Bottle and aligned em dashes

### 2026-04-13 14:12 - ₢A6AAA - n

Rewrite Crucible menu item: clearer title, expanded description with kludge/charge/bottle sequence, drop Claude Code link

### 2026-04-13 13:04 - ₢A6AAA - n

Add cloud builds characterization to Director credential install description

### 2026-04-13 13:02 - ₢A6AAK - W

Added buv_secret_enroll type to BUK validation/presentation stack. Secret fields validate identically to string (min/max length) but render as '(redacted — N chars)' in both regime render and validate output, keeping PEM key material out of terminals and log files. Switched RBRA_PRIVATE_KEY enrollment to use the new type. All four verification targets green: render, validate, BUK self-test (18 cases), QualifyFast.

### 2026-04-13 13:01 - ₢A6AAA - n

Add linked terms to credential install descriptions, move header/description rendering from shared zrbho_credential_install into callers, fix RBRA credential file description from JSON to linked RBRA regime file

### 2026-04-13 13:01 - ₢A6AAK - n

Add buv_secret_enroll type: validates like string but redacts value in render, validate, and log output. Switch RBRA_PRIVATE_KEY to secret enrollment.

### 2026-04-13 12:51 - Heat - S

buv-secret-enroll-type

### 2026-04-13 12:45 - ₢A6AAA - n

Add retriever and director credential install tabtargets with shared zrbho_credential_install utility, remove Run: prefix from StartHere menu, rename tabtargets to ONBOARDING and OnboardingConfigureEnvironment

### 2026-04-13 12:33 - ₢A6AAA - n

Add great for Claude note to stable log path description

### 2026-04-13 12:32 - ₢A6AAA - n

Generalize Orchestration commands to Some commands in Step 6 transcript description

### 2026-04-13 12:27 - ₢A6AAA - n

Rewrite Step 6 logs as bulleted list with concrete stable-name path from BURS_LOG_DIR probe, cyan paths on stable log line and station file status

### 2026-04-13 12:22 - ₢A6AAA - n

Column-align validate tabtargets in Step 7 regime table by padding render-to-validate separators per row

### 2026-04-13 12:18 - ₢A6AAA - n

Add buh_tltTtT combinator, expand Step 7 from 3 abstract patterns to 8 concrete regime rows with linked names and resolved tabtargets, add Output anchor to README

### 2026-04-13 11:54 - ₢A6AAA - n

Link plain-text regime references in Crash Course: BURC/BURS in split line, Regime in Step 4, RBRR/Payor/Depot in bare-fork line, BURS prefix in BURS_LOG_DIR, three regime letters in Step 7

### 2026-04-13 11:46 - ₢A6AAA - n

Add buh_step_style() for configurable step prefix/separator, convert Crash Course from manual buh_section numbering to auto-numbered buh_step1 with Step N — format

### 2026-04-13 11:37 - ₢A6AAA - n

Add Log and Transcript linked terms to legacy onboarding walkthroughs (retriever, director, governor, payor)

### 2026-04-13 11:35 - ₢A6AAA - n

Add Log and Transcript anchors to README.md, link them in StartHere menu and Crash Course Step 6/Step 8

### 2026-04-13 11:32 - ₢A6AAA - n

Replace hardcoded tt/ with ${BURC_TABTARGET_DIR} in Crash Course teaching text

### 2026-04-13 11:22 - ₢A6AAA - n

Trim Crash Course: remove intro paragraph, colophon/frontispiece example, launcher-absence sentence; move station-validate failure warning before the command

### 2026-04-13 11:11 - ₢A6AAA - n

Move Crucible to Foundation, restructure cloud section as Create Payor and Depot with Depot definition as section intro, meld 15-minute timing into Payor account creation

### 2026-04-13 11:05 - ₢A6AAA - n

Restructure StartHere menu: drop arrows, split Retriever/Director credential installs into Foundation, add Create Payor account before Depot creation, link RBRA in credential descriptions

### 2026-04-13 10:54 - ₢A6AAA - n

Fix Step 3 stale claim about station file creation, replace Step 6 grep hack with plain-text reference to validator log output

### 2026-04-13 10:52 - ₢A6AAA - n

Rewrite Crash Course as hands-on walk-through, consolidate Take Your Station into it, rename to Configure your Repo's Environment, rename role-intent tracks to actor-action pattern, add no-station launcher so handbooks work before station file exists

### 2026-04-13 10:07 - ₢A6AAA - n

Remove all [planned] markers from StartHere menu — tracks speak for themselves without roadmap scaffolding

### 2026-04-10 09:54 - Heat - S

bind-mode-coverage-decision

### 2026-04-10 09:53 - Heat - S

ifrit-airgap-feasibility

### 2026-04-10 09:53 - Heat - S

graft-demo-vessel

### 2026-04-10 09:49 - Heat - d

paddock curried: split Assay into 3 concern tracks, add graft demo vessel, enshrine all shipped vessels, tadmor as conjured build target, resolve gaps

### 2026-04-10 09:28 - Heat - d

paddock curried: vessel/nameplate assignments per track, gaps identified

### 2026-04-10 09:25 - Heat - S

ccyolo-vessel-and-nameplate

### 2026-04-10 08:22 - Heat - d

paddock curried: anchor inventory refresh, director subtrack reorder, marshal removal from onboarding

### 2026-04-10 08:17 - Heat - n

Fold -image and -about into Hallmark entry as plain English (container image, software bill of materials), remove HallmarkImage and HallmarkAbout compound anchors

### 2026-04-10 08:12 - Heat - n

Add 12 new README anchors for handbook onboarding tracks: HallmarkImage, HallmarkAbout, Reliquary, Pouch, Tethered, Airgap, Tabtarget, Regime, RBRP, RBRR, RBRN, RBRV, BURC, BURS. Identified during A6 groom session reviewing director subtrack dependency chain and anchor coordination with A5.

### 2026-04-09 14:28 - ₢A6AAA - n

Handbook foundation revisions: Marshal Zero removed from Crash Course (replaced with teaching-only Diagnostic Failure unit, no user-facing MZ invocation); Unit 2/3 restructured so BURC and BURS (project config, personal station) come before the Recipe Bottle regime family; user-facing regime count stated explicitly (seven) with RBRA included as placed-but-not-edited; RBRN rendered as Crucible Nameplate regime; pattern teaching generalized to {W}-r{L}{r|v} covering both buw and rbw workbenches; BURC tabtargets (buw-rcr/rcv) and BURS tabtargets (buw-rsr/rsv) now listed under their respective descriptions in Unit 2; BURC probe dropped (always present — committed); every Recipe Bottle vocabulary term linked and capitalized across both rbho_start_here and rbho_crash_course (Payor, Vessel, Crucible, Nameplate, Kludge, Hallmark, Depot, Governor, Director, Retriever, Ordain, Enshrine, Knight, and more); new buh_tltltlt three-link combinator added to buh_handbook.sh to preserve the director-subtracks table layout under dense linking; zipper description for RBZ_ONBOARD_CRASH_COURSE updated away from Marshal Zero; tabtarget context regenerated. QualifyFast passes, both handbook tabtargets exit 0, station + RBRR probes green in Ready state.

### 2026-04-09 12:50 - ₢A6AAA - n

land handbook-restart foundation: rbho_start_here + rbho_crash_course sequences, tt/rbw-o and tt/rbw-Occ dispatchers, RBZ__GROUP_ONBOARDING zipper group + two enrollments

### 2026-04-09 12:34 - Heat - T

handbook-foundation-implement

### 2026-04-09 12:31 - Heat - T

handbook-final-docs-hygiene

### 2026-04-09 12:31 - Heat - D

restring 1 paces from ₣A5

### 2026-04-09 12:18 - Heat - d

paddock curried: remove interview-transcript phantom; add A5 anchor coordination section; expand References

### 2026-04-09 11:50 - Heat - d

paddock curried: sweep rbw-h? → rbw-o/rbw-O* (Hallmark collision caught)

### 2026-04-09 10:59 - Heat - f

racing, silks=rbk-mvp-3-handbook-restart

### 2026-04-09 10:59 - Heat - S

interview-reconcile-post-fc

### 2026-04-09 10:58 - Heat - S

first-crucible-implement

### 2026-04-09 10:58 - Heat - S

first-crucible-draft

### 2026-04-09 10:57 - Heat - S

handbook-foundation-implement

### 2026-04-09 10:56 - Heat - S

menu-and-crash-course-draft

### 2026-04-09 10:44 - Heat - d

paddock curried: initial paddock from interview ☉260409-1001

### 2026-04-09 10:42 - Heat - N

rbk-handbook-pedagogy

