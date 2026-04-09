## Character

Iterative discovery work. This paddock is a living document that evolves through **interview-implement-interview cycles**, not a one-shot design phase. Implementation paces teach us things the interview cannot; interview paces integrate what was learned and unblock the next build. Cognitive posture: patient exploration over productive rush, willingness to revise decisions when implementation contradicts them.

Teaching pedagogy is distinct from setup choreography. This heat is about helping learners internalize Recipe Bottle's **novel conceptual apparatus** — vessels, depots, hallmarks, vouching, ordain modes, charters, knights, pentacles, bottles, sentries — not about walking them through prescribed setup steps. Setup steps happen, but they are in service of understanding. General container and GCP knowledge is assumed.

## Prerequisites

`₣A3` (rbk-mvp-3-onboarding-guides) must complete and retire before this heat races:
- `₢A3AAI` (buh-handbook-rename-sweep) — bug→buh module rename
- `₢A3AAJ` (rbgm-split-to-rbho-rbhp) — file structure in place
- `₢A3AAF` (docs-alignment-and-cross-machine-validation) — final A3 validation

After A3 retires, this heat consumes `Tools/rbk/rbho_onboarding.sh` as the home for new handbook content. All new tabtargets use the `rbw-h?` prefix (handbook), replacing the old `rbw-gO*` onboarding family from A3.

## Context — The Malformation

A3 produced onboarding guides organized around four "role tracks" (retriever, director, governor, payor). The decomposition followed **authorization boundaries** — which roles can perform which actions. But learners don't come to the tool with a role; they come with an **intent**. They want to explore local containers, evaluate the cloud build engine, set up billing, or administer credentials. Role-tracks answered "what can I do next?" but never "what does this mean?" The teaching was setup-oriented, not understanding-oriented.

Breakthrough insight (from interview in `☉260409-1001`): **learners decompose along intent and goal, not along authorization**. The same probe can appear in multiple handbooks with different teaching contexts. The same role can appear in multiple tracks for different learner goals. Authorization gates actions; intent shapes learning.

## Design Decision Record

### Frame 4-refined — Handbook as Self-Describing Document

Every handbook tabtarget prints a complete teaching document with live probe results interleaved. Units alternate between:

- **Verified units**: teaching prose + embedded probe status (✓/✗ with concrete values) + tabtarget reference to run for fix
- **Teaching-only units**: pure conceptual explanation, no probes, no checkmarks

A learner reads top-to-bottom for teaching; an expert scrolls for the first red probe; a mid-learner reads around what's broken. **One format, three reading strategies** — no mode switching, no state-based branching logic, no walkthrough-vs-reference conflation.

### Probe-Aware Menu Routing

`tt/rbw-h?.StartHere.sh` runs quick triage probes and highlights menu items based on repo state. Unpopulated RBRR highlights the payor path. Populated RBRR plus existing RBRA highlights "find your role." Static menu items remain visible so learners can override routing — probe-aware is guidance, not gating.

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

### Marshal Zero as First-Lesson Gift

`rblm_zero()` produces the invalid-baseline state bare-fork learners encounter: RBRR identity fields blanked, RBRA credentials deleted, hallmarks blanked, reliquary/anchor values blanked, payor OAuth preserved. Running `tt/rbw-rrv.ValidateRepoRegime.sh` against this state **fails diagnostically** — telling the learner exactly which fields need values. This failure is the crash course's first concrete teaching moment: *"In this system, configuration errors fail diagnostically. Trust the error messages."*

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

Working titles, evocative and distinct from the prior role placeholders. **All names are rough** and will be refined in later interview passes. Each track is a single `tt/rbw-h?.{TrackName}.sh` tabtarget rendering a complete teaching document in Frame 4-refined format.

### Foundation (everyone)

1. **Crash Course: Reading the Map** — universal prerequisite. Tabtargets, regimes, log placement, station vs versioned, the `rbw-r{letter}{r|v}` pattern. Local-only, no cloud, no image operations. Teaches BURC, BURS, RBRR at a surface level — enough to navigate the rest.

2. **Take Your Station** — joining an existing configured project. Verify your RBRA is placed correctly, run access probes, confirm you can operate in your role(s). Shortest path, mostly probe-driven with minimal teaching.

### Role-intent tracks

3. **Kindle Your First Crucible** — crucible explorer. Local sandbox with kludged hallmarks, security exploration, fast iteration, no cloud. Teaches: crucible, charge/quench, sentry/pentacle/bottle composition, kludge mode for local hallmark synthesis. *First implementation target — simplest path, zero cloud, tightest feedback loop.*

4. **Establish the Depot** — payor ceremony (~15 min). GCP project creation, OAuth consent screen, billing, RBRR initial population. Teaches: depot, payor responsibility, consent dance, the relationship between payor identity (RBRP) and depot identity (RBRR).

5. **Knight the Realm** — governor. SA administration: create governor/director/retriever SAs, issue RBRA credentials, distribute securely. Teaches: knight, charter, SA lifecycle, secret distribution discipline.

6. **Receive Your Knighthood** — cross-cutting mini-track for directors and retrievers receiving their RBRA from a governor. Place the file correctly (path derived from RBRR), verify via access probe, understand sensitivity. May appear as a unit within Director/Retriever tracks or as a standalone for experienced users receiving new credentials.

### Director subtracks (rough — may decompose or merge)

7. **Your First Ordination** — director, one vessel, one ordain mode. Generic path teaching what ordination means, what gets produced, how vouching works, where hallmarks land. *May decompose by mode (conjure/bind/graft).*

8. **Enshrine the Ancestors** — base image enshrinement. Mirror upstream tool images into GAR, understand supply-chain intent, observe the resulting reliquary contents.

9. **Inscribe the Reliquary** — reliquary management. Create reliquary, understand its role as canonical image reference, understand `RBRV_RELIQUARY`. *May merge with #8 into "Stock the Armory" or similar.*

10. **Tethered Cloud Builds** — cloud build integration from workstation. Understand tethered vs airgapped distinction. Wall-clock ~20 min — pedagogy needs to handle wait time (open question).

11. **Airgap Your Builds** — airgapped mode. Most restrictive ordain mode, teaches isolation. Presumes tethered is understood.

12. **Building for Crucibles** — building crucible targets for testing. May be distinct from tethered/airgapped or variant of Your First Ordination. Needs interview refinement.

### Evaluation tracks

13. **Assay the Realm** — enterprise evaluator. Tour of all ordain modes, enshrinement, reliquaries, airgap. For someone deciding whether to trust Recipe Bottle as a corporate build engine. **Framing matters**: this learner is evaluating, not executing. Critical eye, breadth over depth.

### Cross-cutting surfaces

- **Realm Dashboard** — reference surface showing all probes across all regimes. Equivalent to the old `rbw-gOr` reference mode, now in Frame 4-refined format. Reachable from StartHere menu and as its own tabtarget.
- **Secret Handling Primer** — the RBRA distribution lesson, appearing as a unit in Knight the Realm (governor side, distributing) and Receive Your Knighthood (recipient side, placing). Teaches operational handling knowledge — *what it is, how to handle, where it goes* — without teaching *how to read*.
- **Depot Relationships** — the user flagged wanting good intro text explaining "what a Depot is and how Directors dictate its content but Retrievers use it." This is a relational teaching unit that may appear verbatim in multiple tracks. Candidate for a shared snippet consumed by multiple handbooks.

## Teaching Voice

Teaching content is **vocabulary-first and relational**. The example the user articulated during interview: *"I want good intro text to what a 'Depot' is and how 'Directors' dictate its content but 'Retrievers' use it."* This is the shape of good teaching — nouns introduced in relation to each other, verbs connecting them, roles emerging from what they do to the nouns rather than abstract job titles.

Principles:
- Introduce nouns in context with concrete referents (a depot IS a GCP project; a vessel IS a configured image spec)
- Connect nouns via verbs (directors *ordain* vessels into depots; retrievers *summon* hallmarks from depots)
- Roles emerge from actions, not labels
- Each new term has a reason to exist at the moment of introduction
- Avoid "X is defined as Y" — prefer "when you X, you Y, which is called Z"

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
| **Interview-paddock** | Interview transcript + paddock refinement + spec for next implementation pace(s) |
| **Implement-{track}** | Working handbook tabtarget for one track, tested via agent-learner eval |
| **Interview-reconcile** | Absorption of implementation findings + spec for next implementation pace |

Interview paces are a **first-class pace type**, not preamble. Each interview pace unblocks N implementation paces; each implementation pace generates questions for the next interview.

**First implementation target: Kindle Your First Crucible.** Rationale: local-only, zero cloud flakiness, tightest feedback loop, smallest scope. Lessons about how to write handbooks in Frame 4-refined format will generalize to harder tracks.

**Second implementation candidates** (after first is stable): Crash Course (universal foundation) or Take Your Station (shortest path, probe-heavy, exercises the monotonic-probe side of the framing).

## Open Questions (Interview Work Remaining)

Items to resolve before implementation can start:

- **StartHere menu text** — exact words (in progress as Q3 when interview resumes)
- **Crash Course unit breakdown** — what units, what each teaches, what each probes, learning outcomes
- **Kindle Your First Crucible unit breakdown** — the first complete learner path spec at implementation fidelity

Items that can wait until later interview passes (between implementations):

- **Wait-time pedagogy** — what does a learner do during a 20-minute cloud build? Teaching moment or drop-off risk?
- **Agent-learner eval mechanics** — concrete harness design
- **Director subtrack decomposition** — are tethered/airgapped/crucible builds distinct paths or variants of one path?
- **Shared snippet mechanism** — how do Depot Relationships and Secret Handling Primer appear in multiple handbooks without duplication?
- **Probe-aware menu implementation** — mechanism for triage probes and dynamic highlighting
- **Track name refinement** — current roster is rough; final names come from iteration
- **Wait-time pedagogy for the first long-running track implementation**

## Provenance

Interview conducted in officium `☉260409-1001`. Key turning points:
- User articulated that role tracks are a malformation — authorization ≠ intent
- User articulated concern for teaching NOVEL vocabulary specifically (not general containers/GCP)
- User proposed probe-aware handbook with checkmark redisplay, refining interviewer's Frame 4 into Frame 4-refined
- User confirmed `tt/rbw-h?` as new tabtarget prefix family
- User identified implementation burden (not learner burden) as reason to drop step recording
- User surfaced additional learner profiles through the interview exercise ("crumbs I forgot")
- User asked for intimate track names distinct from prior role placeholders

## References

### Code and configuration
- `Tools/rbk/rbho_onboarding.sh` — new home for handbook functions (post-A3)
- `Tools/rbk/rbhp_payor.sh` — payor ceremonies (post-A3)
- `Tools/rbk/rblm_cli.sh` — `rblm_zero()` produces the invalid-state baseline that teaches diagnostic failure
- `Tools/buk/buh_handbook.sh` — display module for handbook tabtargets (post-α rename)
- `.rbk/rbrr.env` — concrete RBRR example
- `.buk/burc.env` — concrete BURC example
- `Tools/rbk/rbrp_regime.sh` — RBRP validator (reveals payor identity field shape)
- `Tools/rbk/rbap_AccessProbe.sh` — RBRA consumer (reveals per-role credential file usage)

### Specifications
- `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` §Configuration Regimes — regime catalog
- `Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc` §Regime Configuration — BUK regimes
- `Tools/rbk/vov_veiled/RBSRS-RegimeStation.adoc` — Station regime spec
- `Tools/rbk/vov_veiled/RBSRR-RegimeRepo.adoc` — Repository regime spec

### Prior work
- `₣A3` paddock — contains the role-track design this heat supersedes
- `₣AU` paddock — MVP-3 release context from which A3 was spun

### External
- `https://scaleinv.github.io/recipebottle` — public project page, target for cross-linking