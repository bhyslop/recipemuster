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