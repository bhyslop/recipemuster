# Heat Trophy: jjk-v4-4-school-breeze-founding

**Firemark:** ₣Ah
**Created:** 260222
**Retired:** 260706
**Status:** retired

## Paddock

## Paddock: jjk-v4-vision

## Current Design

**Authoritative design seeds**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline. Partially superseded by cchat-20260301 sessions and cchat-20260317 execution model refinements.

Key decisions: no leg layer, worktrees for isolation (branches persist after worktree disposal), composable gaits, four-phase pipeline (longe → school → breeze → corral), warrants as per-beat fields in gallops, jjx as LLM orchestrator. Git-as-mutex proven under concurrent pressure; V4 extends git protection to work product (beat branches).

## Identity System

| Symbol | Name | Digits | Structure | Slots |
|--------|------|--------|-----------|-------|
| `₣` | Firemark | 2 | 2 heat | 4,096 heats |
| `Ꝗ` | Quirt | 3 | 3 global | 262,144 gaits |
| `₼` | Martingale | 4 | 2 heat + 2 index | 4,096/heat |
| `₢` | Coronet | 5 | 2 heat + 3 index | ~262K/heat |
| `₭` | Kimberwick | 6 | 4 martingale + 2 index | 4,096/martingale |

Length alone disambiguates: 2=firemark, 3=quirt, 4=martingale, 5=coronet, 6=kimberwick.

Quirt `ꝖABC` identifies an immutable gait snapshot. When a gait evolves, a new quirt is minted with the same silks. "Latest version" = highest quirt with matching silks. No lineage fields needed.

## Vocabulary

| Term | Definition |
|------|------------|
| **Beat** | Abstract step within a gait: defined by a piaffe specifying purpose, approach, and nosebag declarations. In gallops execution tables, beat rows are concrete instances carrying resolved warrants and (upon completion) provender. Context disambiguates: gait definition → abstract template with piaffe, gallops table → concrete dispatch unit. |
| **Gait** | Reusable recipe of beats organized by chukker. Gaits can compose other gaits (practical depth limit: ~2 levels). Stored as data in gallops with quirt identity. School-time resource: school reads gaits for decomposition guidance and confidence gates, then produces concrete beat table entries. Breeze never looks up a gait. |
| **Piaffe** | The prose field of an abstract gait beat template. Contains the beat's purpose, approach, and nosebag declarations (produces/expects). School reads piaffes to write concrete warrants. Docket → piaffe → warrant: intent → template → concrete. Dressage term: precise, deliberate, contained movement — trotting in place with controlled energy before moving forward. |
| **Warrant** | The resolved prompt school writes into a single beat table entry. Contains concrete instructions for one model dispatch. Fully resolved — no ₿ symbols, no gait references, only concrete values. Each warrant cites its source quirt for audit. |
| **Provender** | A beat's informational output — a markdown document produced during execution, indelibly recorded on the beat row when status → complete. One provender per beat, optional (beats may produce zero or one). Labeled with a single `#` header, containing zero or more nosebag subsections. jjx reads markdown structure for routing but never interprets prose content. Scoped to the martingale — lives and dies with the volte. |
| **Nosebag** | A `##`-labeled subsection within a provender document. The addressable unit of informational output. Consuming beats request nosebags by name in their piaffe; jjx matches `##` headers across all prior completed beats' provender and delivers matching sections at dispatch time. Multiple beats may produce same-named nosebags; consumer gets all matches. Multiple beats may expect the same name; each gets the same set. |
| **Volte** | An attempt at executing one or more paces. One active volte per heat at a time. School creates it (populating beat table entries in gallops); breeze executes it (dispatching beats per chukker); corral reviews per-pace (accept/refine/reject). Identified by martingale. Dressage term: a precise, controlled circle back to the same point with refined intent. |
| **Martingale** | Volte identity. `₼` (U+20BC, manat) + 4 base64 characters (2 heat + 2 index). Named for the control strap that keeps the horse from going off course — what holds execution on track. Replaces "caracole" (cchat-20260317). |
| **Kimberwick** | Beat instance identifier. `₭` (U+20AD, kip) + 6 base64 characters (4 martingale + 2 index). 4,096 slots per martingale. Named for a type of bit providing precise, engineered control. Scoped to its martingale. If school exhausts slots, breeze and corral what exists — not an error condition. |
| **Chukker** | Numbered concurrency layer within a volte. All beats in chukker N execute concurrently; chukkers execute in sequence. School assigns each beat a chukker number. Named for a numbered period in polo — distinctive, won't bleed into generic usage. |
| **Quirt** | Gait identity. `Ꝗ` + 3 base64 characters. Named after a short riding whip — the thing that sets a gait in motion. |
| **Longe** | Heat-level readiness assessment. Classifies remaining paces as breezable / needs-refinement / blocked. Read-only. See Four Phases for detail. |
| **School** | Per-pace planning phase. Produces concrete beat table entries from docket + codebase investigation. Opus-tier, high human attention. See Four Phases for detail. |
| **Breeze** | Execution phase. jjx dispatches beats per chukker via OAuth-authenticated Claude Code instances. Zero human attention. See Four Phases for detail. |
| **Corral** | Per-pace review verb (parallels mount). Reviews composed pace outcome (not individual beats). Three outcomes: accept, refine + accept, or reject. No rebreeze — rejection flows upstream. See Four Phases for detail. |
| **Tackle** | Core definition homed in the aspirant sheaf `Tools/jjk/vov_veiled/JJS-aspirant-tackle.adoc` (scope→discipline binding; one word, one concept — that MVP core is the floor this heat's maximal tackle extends, and this paddock no longer restates it). ₣Ah's extension layer, homed here: ₿-named file scopes (blazes), ₿-named actions (command + exclusivity), gait affinities, read-also entries. School resolves ₿ references to concrete paths when building beat warrants; tackle's build/test/integrity-check declarations inform school's chukker boundary decisions. |
| **Blaze** | Symbolic file/resource reference defined by a tackle. Notation: `₿guide-doc`. Tackles own blaze definitions; dockets and gaits reference them. School resolves blazes to concrete paths in warrants — breeze never sees ₿ symbols, only resolved paths. Global uniqueness within a heat. |

### Three Naming Regimes

| Regime | Scope | Example | Where used |
|--------|-------|---------|------------|
| Minted prefixes | Code, specs, cross-chat precision | `jjep_`, `jjfg_` | Rust types, AsciiDoc anchors, AXLA |
| Silks | Gallops entities (heats, paces, gaits, tackles) | `design-v4-data-model` | JSON data, human display |
| Blazes (₿) | Tackle slot references | `₿guide-doc` | Dockets, gaits — resolved by school |

### Prose Field Escalation

Three named prose fields form an escalation from intent to execution:

| Field | Lives on | Written by | Purpose |
|-------|----------|------------|---------|
| **Docket** | Pace | Human (at slate/reslate) | States intent — what needs to be done |
| **Piaffe** | Gait beat template | Gait author | Abstract instruction — how school should plan this kind of beat |
| **Warrant** | Beat row (gallops) | School | Concrete prompt — fully resolved, dispatched to a model |

### Beat: Abstract and Concrete

"Beat" serves as both abstract template (in gait definitions) and concrete instance (in gallops execution tables). The structural context disambiguates: a beat in a gait's recipe is a template with a piaffe declaring purpose, approach, and nosebag flow; a beat row in gallops is a concrete dispatch unit with a resolved warrant, model, file scope, chukker number, status, and (upon completion) provender. This dual usage is intentional — the word maps naturally to both contexts, and the home (gait library vs gallops table) eliminates ambiguity.

## Execution Model

### Four Phases

1. **Longe** (parallel, read-only) — assesses all remaining paces in a heat simultaneously. For each pace: reads docket, reads codebase to understand scope, identifies file types and potential gaits, classifies as breezable / needs-refinement / blocked. Output is a heat-level readiness report. Guides where to focus groom/reslate before committing to school.
2. **School** (opus, per-pace, conversational) — reads pace docket and codebase thoroughly (file reads, grep, structural understanding). Two internal phases: (a) assess docket quality against codebase reality, evaluate confidence gates; (b) produce concrete beat table entries in gallops, each carrying a resolved warrant with chukker assignment and model selection. School reads gait piaffes to understand beat purpose and nosebag flow, then writes warrants that account for what provender each beat will receive. May refuse to warrant ("this docket has gaps — here's what's wrong"). Does NOT web-search or produce work artifacts. Human Q&A refines the beat plan. One pace at a time.
3. **Breeze** (jjx-orchestrated, concurrent) — jjx reads beat table from gallops. For each chukker in sequence: merges prior-chukker branches, collects provender from completed prior-chukker beats, creates per-beat branches and worktrees, dispatches warrants (with matching nosebags assembled per piaffe declarations) as bare prompts to specified models via OAuth-authenticated Claude Code instances. Multiple beats within a chukker execute concurrently. jjx collects results, records provender, and updates beat status in gallops. Haiku can orchestrate — precision, not judgment. Staggering beat launches by 1-2 seconds mitigates RPM throttle. Different model tiers have independent rate limit pools.
4. **Corral** (human + LLM, per-pace) — invoked per pace like mount, targeting next candidate (no param) or a specific pace. Reviews composed pace outcome against docket, diffs against base. Individual beats are internal detail — corral evaluates the whole. Provender from the volte's beats is visible to corral for context. Three outcomes: (a) accept as-is → merge to main, pace complete; (b) refine interactively → agent and human adjust near-miss output, then merge; (c) reject → pace returns to pool for groom/reslate, eventual re-school produces new volte (fresh provender). No rebreeze — the fix is always upstream.

### Beat Table in Gallops

Beats are a **flat collection** in gallops, each carrying an **immutable coronet reference** (pace). Paces don't contain beats; beats point to their pace.

Each beat row carries:
- **Kimberwick** (key) — beat instance identity, scoped to martingale
- **Coronet** — immutable pace reference
- **Warrant** — resolved prompt for this beat
- **Model** — which model tier to dispatch to
- **File scope** — files this beat is expected to modify (empty = read-only/reviewer beat)
- **Chukker** — integer concurrency layer assignment
- **Quirt** — source gait beat for audit
- **Status** — pending / complete / failed
- **Provender** — markdown document (optional, empty until status → complete, immutable thereafter)

Three status states only. No "dispatched" — idempotent re-dispatch on crash recovery. Failed = critical issue encountered during execution, needs human attention.

Provender is the only field that changes after beat creation — set once, indelibly, on completion.

### Provender and Nosebag Routing

At each chukker boundary, jjx assembles nosebags for each beat about to dispatch:

1. Scan all completed beats in prior chukkers within this martingale
2. For each completed beat with provender, parse `##` headers
3. Match against the consuming beat's piaffe `expects` declarations
4. Deliver matched nosebag sections as context alongside the warrant

jjx reads markdown structure (headers) but never interprets prose content between headers. The LLM produces nosebags; the LLM consumes them. jjx is the switchboard.

Provender is scoped to the martingale. If corral rejects the volte and a new one is schooled/breezed, fresh provender starts from scratch.

### Chukker Model

School assigns each beat a **chukker number** (integer). jjx dispatches all beats in chukker 0 concurrently, waits for all to complete (or fail), then chukker 1, etc.

Simpler than arbitrary DAGs. Loses some expressiveness (can't say "C depends on A but not B" within the same chukker) but in practice school decomposes work into layers, not irregular graphs. One integer field per beat. No dependency lists. No cycle detection needed.

```
Chukker 0:  beat-write-spec, beat-write-code    (concurrent)
Chukker 1:  beat-cross-align                     (waits for chukker 0)
Chukker 2:  beat-integration-check               (waits for chukker 1)
```

### Branch-Per-Beat Model

Branches are execution artifacts, created just-in-time when a beat's chukker becomes active:

1. jjx merges all completed prior-chukker branches into a staging point
2. If merge succeeds → create beat branch from staging point, dispatch into worktree
3. If merge fails (conflicts) → fail-fast, human attention needed

Branch naming: `jj/MARTINGALE_ID/KIMBERWICK_ID` — created at dispatch time, not planning time.

**"State from branch existence" revised**: Gallops carries volte execution state (beat table with status). Branches are execution containers for beat worktrees, not the primary state carrier. Heat stores `next_martingale` seed for allocation.

### Merge Model

Two kinds of merge at chukker boundaries:

1. **Mechanical merge** (jjx infrastructure): Git merge of prior-chukker beat branches before creating next-chukker branches. Different files = trivial. Conflict = fail-fast signal, human attention needed. Always happens automatically at chukker boundaries.
2. **Semantic merge** (explicit beat): Alignment/cross-reference verification for multidisciplinary work. An LLM reads parallel outputs and checks consistency. Gaits CAN name merge/alignment beats but aren't required to.

**Tackle's build/test/integrity-check declarations** inform school's chukker boundary decisions — whether auto-merge with tackle-defined checks suffices or explicit alignment beats are needed.

Merger beats receive assembled context from the post-mechanical-merge staging point: warrants (intent per beat), beat diffs (actual outputs), docket (human's original goal), and nosebags from prior beats. jjx constructs this context.

### Longe → School Handoff

Longe assesses the whole heat; school plans one pace at a time. The refinement cycle: longe identifies readiness → human grooms unready paces (reslate) → school warrants ready paces → breeze executes → corral reviews → longe reassesses.

School's primary discipline is **refusing to plan on weak foundations**. Its most valuable output may be "I cannot warrant this docket — here's what's wrong."

### Concurrent Execution

- jjx dispatches **multiple Claude Code instances** (OAuth-authenticated) per chukker, one per beat, each in its own worktree.
- Staggering launches by 1-2 seconds and right-sizing models per beat mitigates throughput contention. Different models have independent rate limit pools — beats mixing haiku/sonnet/opus get more effective throughput than a single model.
- **API-key direct REST dispatch** deferred to future work.

### jjx as LLM Orchestrator

V4 jjx doesn't just manage JSON — it emits prompts, directives, and context that shape what the LLM does next. Slash commands establish the interpretation contract (LLM treats jjx output as imperative, not advisory). jjx output design becomes a first-class concern: quality of directive text matters as much as correctness of JSON mutations.

Pattern: jjx handles state/sequencing, LLM handles git operations and judgment. Co-routine. jjx is the conductor — it never interprets the warrants, just routes them.

jjx orchestration is haiku-tier work. Reading beat tables, creating worktrees, dispatching per chukker, collecting results, routing nosebags — precision, not judgment. Reserve opus for school and corral; haiku conducts.

**Model-tier enforcement**: jjx requires a model-identity parameter on every invocation. The invoking model self-reports its identity (e.g., `claude-haiku-4-5`). jjx enforces tier constraints mechanically — breeze-phase orchestration commands refuse to execute unless invoked by haiku. No honor system; hard gate.

**Bare prompt reproducibility**: Same prompt + same model + same worktree state = comparable output. Multi-turn conversations are path-dependent and unreproducible; a bare prompt is a function call. This makes process improvement scientific: when a beat produces bad output, change one variable (the prompt, or the model) and rerun. The closed loop — prompt → output → evaluate → refine prompt — is what makes the gait library a learning system. Each quirt version encodes lessons from prior bare-prompt outcomes. Tool use during beat execution is expected (file reads, searches, etc.) but opaque to the orchestrator — jjx dispatches one warrant and collects commits.

### Attention Model

V4 is designed around human attention as the bottleneck:
- **Longe**: low attention — read report, decide where to focus groom effort
- **School**: high attention — shaping beat plans, making stop/go judgments on ambiguous paces
- **Breeze**: zero attention — pure execution, go do something else
- **Corral**: medium attention — reviewing diffs per pace, optionally interactive refinement for near-miss candidates

The system optimizes for the serial path being frictionless, not for raw parallelism.

## Gait Library

Gaits live in gallops as data, identified by quirt (`ꝖABC`). They are a **school-time resource** — school reads them for decomposition guidance and confidence gates, then produces concrete beat table entries. Breeze never looks up a gait.

The gait library is like a playbook a coach consults before writing the game plan. The game plan doesn't say "run play #47" — it says exactly what each player does.

Gaits evolve through practice: start with a few simple single-beat gaits, use them, see what works, crystallize recurring compositions as named gaits. The library grows from practice, not from design.

### Well-Formed Gait Beats

Abstract beats in gait definitions carry a **piaffe** — the prose field declaring:
- **Purpose**: What the beat accomplishes
- **Approach**: How school should plan this kind of work
- **Nosebag production**: What `##`-labeled nosebags this beat is expected to emit in its provender
- **Nosebag expectations**: What nosebag names this beat needs from prior chukkers
- **Resources**: File scopes, ₿ blaze references

Additionally, well-formed beats may include:
- **Auto-tool gaits**: Builds, test runs, linters — automatic tool invocations as beat steps
- **Quality requirements**: Retry semantics ("try and fix up to N times") or fail-fast ("stop on any failure")

**Gait record fields (TBD)**: At minimum needs silks (shared across versions), beat templates with piaffes, default model preferences, confidence gates, and required ₿ blazes. Exact schema deferred until first gaits are built. Gaits are immutable — evolution mints new quirts with the same silks.

## Schema Decisions

Accumulated across groom sessions.

### Core V4 Type Changes
- **Tack eliminated**: Flat mutable fields on pace (state, silks, docket, basis, ts, chain). Chain is inter-pace dependency set at slate/reslate time — structural metadata, not discovered by school. Longe depends on chains being pre-set to assess pace readiness.
- **Field renames**: V3 `text` → `docket`. V3 `direction` → no longer a pace-level field; warrants live on beat rows.
- **New state enum**: green → ready/reined → candidate → complete/abandoned. Reined = interactive-required (school decides ready vs reined).
- **Bridling and markers eliminated**: School/breeze/corral replaces arm-and-fly. Restore markers on need.

### V4 Execution Infrastructure (cchat-20260317)
- **Caracole → Martingale**: Volte identity renamed to avoid C-collision with Coronet. Symbol: `₼` (U+20BC, manat).
- **Kimberwick**: Beat instance identity. 6 chars (4 martingale + 2 index). 4,096 slots per martingale. Symbol: `₭` (U+20AD, kip).
- **Beat table in gallops**: Flat collection of beat rows with immutable coronet refs, replacing monolithic warrant JSON. Warrants in gallops (not git) for lane isolation, natural state store, granular status tracking.
- **Chukker model**: Integer concurrency layers replace arbitrary DAG `depends` lists.
- **Branch-per-beat JIT**: Branches created at dispatch time. Mechanical merge at chukker boundaries. Naming: `jj/MARTINGALE_ID/KIMBERWICK_ID`.
- **Beat status**: pending/complete/failed. Three states, idempotent re-dispatch.
- **next_martingale seed**: Heat-level field for martingale allocation.

### V4 Informational Output (cchat-20260317b)
- **Provender**: Markdown document on beat row, set indelibly at completion. `#` header labels the document. No separate table — lives on beat row as string field.
- **Nosebag**: `##`-labeled subsection within provender. Addressable unit for routing. jjx matches headers mechanically, never interprets content.
- **Piaffe**: Named prose field on abstract gait beat templates. Replaces unnamed "inputs/outputs/resources/goals" facet list with a single prose field carrying purpose, approach, and nosebag declarations.
- **Routing model**: Consuming beat's piaffe declares expected nosebag names. jjx scans completed prior-chukker beats, matches `##` headers, delivers matching sections. Name-matched, not ID-matched.
- **Provender lifecycle**: Empty until complete, immutable thereafter. Scoped to martingale — rejected volte means fresh provender on re-school/rebreeze.

### Gallops Registries
- **Gaits registry**: Top-level gallops key, keyed by quirt. Fields TBD.
- **Tackles as resource bundles**: Top-level gallops key. Silks-identified, mutable. Contains blaze definitions, actions, gait affinities. Extension-layer schema only: the standing scope→discipline base is pedigree-side per the aspirant tackle sheaf's provisional layering — reconcile at the tackle/blaze design pace, which inherits the sheaf as its floor.
- **Blaze identity (₿)**: Not a global identity — scoped to tackle, resolved by school.

## Still Open

- **Gait data model fields**: Beat templates with piaffes, confidence gates, model preferences, auto-tool gaits, quality requirements — exact schema deferred.
- **Provender emission convention**: How does the LLM emit provender during beat execution? Structured block, tool call, delimiter convention? Deferred until first gaits are built.
- **Provender size limits**: Same size-guard philosophy as commits? Deferred.
- **Memory curation**: How does jjx curate prior-volte context for school prompts? Deferred — likely ₣Am scope.
- **Longe output format and CLI**: Readiness report structure and new upper API verb. Needs spec.
- **Ready vs reined distinction**: Both in state enum — is the distinction clear enough?
- **Beat table gallops location**: Top-level keyed by kimberwick, or scoped under martingale?
- **Chukker 0 semantics**: Always first execution layer, or can school use it for setup beats?

## Resolved

### Pre-20260317
- **School scope** (cchat-20260306): Reads codebase thoroughly — planning intelligence, not "research." Two phases: assess and plan.
- **Longe concept** (cchat-20260306): Heat-level readiness assessment, separate from per-pace school. Longe → groom → school refinement cycle.
- **No concurrent voltes** (cchat-20260306): One active per heat. Parallax via concurrent voltes deferred.
- **Volte history synthesis deferred** (cchat-20260306): School starts fresh from docket. Machine-curated volte history is V4.1+ scope.
- **Quirt immutability** (cchat-20260306): Immutable snapshots, new quirt per evolution with same silks. No lineage chain.
- **Corral as per-pace verb** (cchat-20260311): Three outcomes: accept / refine + accept / reject. No rebreeze — fix is always upstream.

### cchat-20260317: Execution Model Refinements
Martingale replaces caracole (C-collision avoidance). Kimberwick beat identity (6-char, 4096/martingale). Chukker concurrency layers replace DAG. Warrants in gallops not git (lane isolation). Flat beat table with immutable coronet refs. Branch-per-beat JIT with mechanical merge at chukker boundaries. Three-state beat status (pending/complete/failed). Corral at pace boundary. Spur concept killed (structural context disambiguates). Well-formed gait beats (inputs/outputs/resources/goals, auto-tool gaits, quality requirements). Lean beat records. Chain set at slate/reslate not school. Volte state in gallops supersedes "state from branch existence."

### cchat-20260318: Symbol Selection
Martingale symbol: `₼` (U+20BC, Azerbaijani manat). Kimberwick symbol: `₭` (U+20AD, Laotian kip). Continues accidental currency-symbol pattern: ₣ franc, ₢ cruzeiro, ₼ manat, ₭ kip. All in Currency Symbols block (U+20A0–U+20CF).

### cchat-20260317b: Provender, Nosebag, and Piaffe
Beat informational output as provender (markdown doc on beat row, indelible at completion). Nosebags as `##`-labeled addressable subsections. Piaffe as named prose field on gait beat templates (docket → piaffe → warrant escalation). Name-matched routing: piaffe declares produces/expects, jjx matches `##` headers mechanically. No separate findings table — provender lives on beat row. Provender scoped to martingale (dies with rejected volte). LLM produces, LLM consumes, jjx routes.

## Gait Design Principles

Distilled from cchat-20260302 (₢AiAAz retrospective), updated cchat-20260317.

A gait contains: **beat templates with piaffes** (declaring purpose, approach, nosebag flow, organized by chukker), **codebase investigation steps** (school reads files to understand scope before creating beat entries), and **confidence gates** (conditions that cause school to stop and flag). Confidence gates are the most valuable part — they encode "where this kind of work goes wrong."

Well-formed gait beats may also include **auto-tool gaits** (builds, test runs, linters) and **quality requirements** (retry semantics or fail-fast declarations). Tackle declarations inform which checks are available.

Key decisions:
- Gaits are checklists for school, not plan templates for breeze.
- Gait selection belongs in school, not at slate time.
- Review is multi-phase: self-review → fix → human review (corral) → fix.
- School's primary discipline is refusing to plan on weak foundations.

## V3→V4 Migration Discipline

### Strategy

V4 introduces breaking schema changes. A work installation runs V3 jjx against V3 gallops and must continue functioning until explicitly upgraded.

**Detection:** `schema_version` field in `jjrg_Gallops`. Absent = V3. Present with value `4` = V4. Uses `#[serde(default)]` so V3 files deserialize cleanly.

**Migration path (dual-read, write-V4):**
- V4 `jjdr_load` detects V3 (missing `schema_version`), deserializes as V3 types, transforms in memory to V4, skips round-trip byte-exact check
- V4 `jjdr_save` always writes V4 format with `schema_version: 4`
- First load of a V3 file silently upgrades it to V4 on next save
- Round-trip check remains strict for V4 files

**Frozen V3 reference:** V3 types are snapshot as `jjrt_v3_types.rs` (frozen, not edited). The V3 schema section of JJS0 remains as a reference until ₣An removes it.

### Pace Discipline

Every pace in ₣Ah that modifies the gallops schema MUST include in its docket:
- **V3 field**: What V3 field/structure is affected
- **V4 change**: What V4 does differently
- **Migration transform**: How `jjdr_load` converts V3→V4 in memory

### Validation Changes

V3 validation rules in `jjrv_validate.rs` that will break:
- Tack non-emptiness (tack eliminated in V4)
- `direction` present iff `bridled` state (both concepts eliminated)
- Pace state enum (V3: rough/bridled/complete/abandoned → V4: green/ready/reined/candidate/complete/abandoned)

These must be updated alongside the type changes, not as an afterthought.

## Heat Constellation

₣Ah is part of a V4 initiative:

| Heat | Silks | Role | Status |
|------|-------|------|--------|
| ₣Ah | jjk-v4-vision | Development — schema transition, breaking changes, gaits/beats | Racing |
| ₣An | jjk-v4-release-and-legacy-removal | Cleanup — upgrade installs, re-enable validation, remove V3 compat | Stabled (until ₣Ah nears completion) |
| ₣Am | jjk-v5-notional | Parking lot — post-V4 ideas relocated from ₣AG | Stabled |

## References

Superseded design seeds: `Memos/memo-20260222-jjk-v4-vision.md`, `Memos/memo-20260224-jjk-v4-gaits.md`

- `Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc` — V3 data model (what V4 replaces; renamed from JJS0-GallopsData.adoc by ₣Aw)
- cchat-20260224 — Schema decisions, slash command reduction
- cchat-20260301/b — Beats, voltes, corral, execution model, quirt identity, warrant structure, gait library
- cchat-20260302 — Gait working concept: school checklists, confidence gates, review phases
- cchat-20260304 — Backwards compatibility, three-heat constellation, migration discipline
- cchat-20260306 — Longe concept, school scope, assessment/planning split
- cchat-20260311/b — Corral refinement, tackle/blaze surfacing, beat merge, parallel subtrees
- cchat-20260317 — Martingale, kimberwick, chukker, warrants in gallops, beat table, branch-per-beat, lane isolation, well-formed gait beats
- cchat-20260317b — Provender, nosebag, piaffe: beat informational output and gait prose field naming
- ₢AhAAF/₢AhAAG — Verb restructure and slash command cleanup

## ₣Aw Migration Handoff (2026-03-24)

₣Aw (jjk-v4-0-jjs0-axla-normalization) completed V3 spec infrastructure work. Summary of what changed and what ₣Ah inherits:

### Annotation Migration

JJS0 annotations migrated from transport-coupled CLI-era to transport-agnostic:
- **20 operations**: `axi_cli_subcommand` → `axvo_procedure axd_transient`
- **13 arguments**: `axa_cli_option`/`axa_cli_flag` → `axa_keyword`
- **Deliberately retained**: `axl_voices` on upper API verbs (`axi_cc_claudemd_verb`), entities (`axo_entity`), enum values (`axt_enum_value`), records (`axr_record_json`), members (`axr_member`). These are not transport-coupled.

The `axhe*` entity voicing convention (entity/field/method/parameter/output hierarchy) is proven in JJSCGZ-gazette.adoc. V4 new entities should use `axhe*` from the start.

### Spec Gaps Closed

- `jjdo_close` (JJSCWP-wrap.adoc) and `jjdo_paddock` (JJSCCU-curry.adoc) now have operation specs
- Bridled state: added to current spec as V3-legacy deprecated (`axd_internal`), removal deferred to ₣An
- 4 vestigial arguments (`jjda_state`, `jjda_pace`, `jjda_created`, `jjda_direction`) marked `axd_internal`
- Unspecified MCP params: design decision documented — operation-specific params are inline in operation subdocs, not global `jjda_*` terms

### Data Model Additions

- Tack record (`jjdcr_tack`, 6 members) added to current spec
- Collection members added: `jjdgm_heats` (map), `jjdhm_paces` (map), `jjdpm_tacks` (array)
- `jjdgm_version` (schema_version) added to Gallops

### AXLA Extensions

- `axt_map` — 2-arity type (key type + value type)
- `mcm_sprue` — wire-level token identity tier (quoin → inlay → sprue)
- `axr_member` 1-arity for `mcm_sprue` (JSON key)
- No new `axhe*` markers needed — the existing 11-marker hierarchy was sufficient

### File Rename

`JJS0-GallopsData.adoc` → `JJS0_JobJockeySpec.adoc`. All active references updated.

### V4-Relevant Findings

- **Bridled removal** is ₣An scope, not ₣Ah. ~80 code references across Rust source and spec subdocs.
- **`jjdcm_direction`** member becomes vestigial when Bridled is removed.
- **JJF (gazette) file exchange** works for multiline MCP parameters. V4 operations with complex inputs (warrants, piaffes) should use gazette format.

## Paces

### select-martingale-kimberwick-symbols (₢AhAAV) [complete]

**[260318-0553] complete**

Select Unicode symbols for martingale (volte identity, 4 chars) and kimberwick (beat instance identity, 6 chars).

## Constraints

- Must be visually distinct from existing symbols: ₣ (firemark), Ꝗ (quirt), ₢ (coronet)
- Must render correctly in terminals (macOS Terminal, iTerm2, VS Code integrated terminal)
- Must not collide with first letters of other identity names: F (firemark), Q (quirt), C (coronet), M (martingale), K (kimberwick)
- Should be easy to type or paste
- ₡ (former caracole symbol) is available for reuse if appropriate for martingale

## Produces

- Martingale Unicode symbol selected and documented
- Kimberwick Unicode symbol selected and documented
- Paddock identity table updated with concrete symbols (no more TBD)

## Depends on

Nothing.

### spec-paces-conceptual-maturity-gate (₢AhAAS) [rough]

**[260317-2116] rough**

Manual longe: assess all spec pace dockets for conceptual maturity before executing any specification updates.

Character: Intricate but focused. This is a grooming exercise, not open-ended design. The paddock carries the authoritative design decisions; this pace verifies that each spec docket faithfully reflects those decisions with sufficient precision to execute against.

Gate Conditions -- a spec docket is conceptually mature when:

1. **Every new type/field is named** with minted prefix allocated (jjep_, jjfg_, etc.). No unnamed fields, no TBD type names, no TBD symbols. (Martingale and kimberwick symbols resolved by AhAAV before this pace begins.)
2. **Every V3 migration path** for affected fields is documented, where applicable.
3. **Every cross-pace dependency is surfaced**, not hedged with "depends on output of X."
4. **Precision about what, abstract about where.** Dockets specify what the spec section must define without prescribing JJS0 section headings.
5. **No stale vocabulary.** No references to caracole, DAG depends lists, monolithic warrant JSON, warrant-as-first-commit, or other superseded concepts.
6. **Confidence gates named.** Each spec docket identifies conditions that should cause the spec author to stop and flag.
7. **Dependency chain validated.** The spec paces form a DAG. Verify ordering is correct, no circular dependencies.
8. **File exchange discipline applied.** Any spec command that accepts or emits prose-length content (dockets, paddocks, warrants, provender) uses the MCP file exchange protocol defined in AhAAW, not inline JSON string params.

Scope -- review these spec paces against the gate conditions:
- AhAAA (design-v4-data-model)
- AhAAO (design-v4-tackle-blaze)
- AhAAP (design-v4-gait-quirt)
- AhAAQ (design-v4-martingale-beat-identity)
- AhAAT (design-v4-chukker-execution)
- AhAAU (design-v4-concurrent-dispatch)
- AhAAM (spec-v4-command-surface)

Produces:
- Reslated spec dockets where gates are not met
- Newly slated implementation paces with concrete dockets
- Complete when all spec dockets pass all gates AND implementation paces are slated

Depends on:
- AhAAV (select-martingale-kimberwick-symbols) -- symbols must be resolved before gate 1
- AhAAW (design-v4-mcp-file-exchange) -- file exchange protocol must be defined before gate 8

Not bridleable: Requires human judgment about precision/abstraction calibration and implementation pace scoping.

### cull-v3-spec-chalk-tack-bridle-arm (₢AhAAN) [complete]

**[260311-1454] complete**

Remove dead V3 concepts from JJS0 and Rust: (1) Delete tack record/members/bridled-state from spec and types, rewrite pace record as flat fields in spec, (2) Delete arm command from spec and Rust, (3) Remove A/B/F/T steeplechase markers from spec and marker registry, (4) Delete or gut jjdo_mark (chalk) command — remove approach/fly/bridle markers, keep only discussion marker if needed, (5) Update all command behaviors from 'prepend tack' to flat field mutation (revise_docket, relabel, drop), (6) Remove jjdkr_/jjdkm_/jjdpe_bridled linked terms from mapping section, (7) CLAUDE.md mount protocol: remove approach chalk step. Spec-first: update JJS0 and mapping section, then align Rust to match.

### fix-restring-eprintln-and-claudemd-coronets-doc (₢AhAAL) [complete]

**[260307-1244] complete**

Spook fix: jjx_transfer (restring) fails silently through MCP.

## Two bugs

1. **CLAUDE.md says coronets is newline-separated string** but jjrrs_restring.rs:45 deserializes via `serde_json::from_str::<Vec<String>>()` — needs a JSON array like `["AYAAA"]`.

2. **Errors go to stderr, not buf** — all error paths in jjrrs_run() use `eprintln!()` and return `(1, empty_buf)`. MCP transport only sees buf, so errors are silently lost.

## Fix

- Update CLAUDE.md (and vocjjmc_core.md) to document coronets as JSON array
- Change `eprintln!` error paths in jjrrs_restring.rs to `write!(buf, ...)` so errors flow through MCP
- Audit other commands for same eprintln pattern

### jjs0-upper-api-restructure (₢AhAAF) [complete]

**[260306-0905] complete**

Restructure JJS0 Upper API section: merge jjsud_/jjsum_ categories into jjsuv_ (JJ Spec Upper Verb).

## Changes to mapping section

- Replace category comments: remove `jjsud_` (direct verbs) and `jjsum_` (multistep verbs), add `jjsuv_` (verbs)
- Rename all surviving verb attributes from `jjsud_*`/`jjsum_*` to `jjsuv_*`
- Remove eliminated verb attributes entirely: bridle, quarter, braid, garland
- Remove `jjsus_*` entries for eliminated verbs: bridle, quarter, braid, garland
- Remove `jjsus_*` entries for demoted verbs: slate, reslate, notch, rail, furlough, restring, retire-dryrun, retire-FINAL, mount, groom

## Changes to definition sections

- Merge "Direct Verbs" and "Multistep Verbs" subsections into single "Verbs" subsection
- Rewrite all verb definitions using `jjsuv_*` linked terms
- Remove definitions for eliminated verbs: bridle, quarter, braid, garland
- Remove anchors for eliminated verbs: `[[jjsud_bridle]]`, `[[jjsum_quarter]]`, `[[jjsum_braid]]`, `[[jjsud_garland]]`
- Update `//axl_voices` annotations: all verbs voice `axi_cc_claudemd_verb`

## Changes to Upper API intro paragraph (lines 340-352)

- Rewrite inline references: `{jjsud_slate}` → `{jjsuv_slate}`, `{jjsum_mount}` → `{jjsuv_mount}`, etc.
- Remove `{jjsus_mount}` reference from "artifacts translate between layers" sentence — rewrite to reflect that most verbs are now verb-table-delivered, with slash commands reserved for protocol-heavy verbs (school, corral)

## Changes to Slash Commands section

- Remove all current `jjsus_*` entries (all demoted or eliminated)
- Add brief prose note: "V4 reserves slash commands for protocol-heavy orchestration (school, corral). These will be specified when implemented. All other verbs are delivered via the CLAUDE.md verb table."
- Do NOT add placeholder `jjsus_school` or `jjsus_corral` attributes — defer until those are built

## Verbs surviving as jjsuv_*

slate, reslate, wrap, rail, furlough, retire, restring, muster, parade, scout, nominate, mount, groom, notch

## Files

- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (sole file)

## Not in scope

- Tack/bridled state removal (that's schema work in ₢AhAAD)
- Actual slash command file deletion (that's ₢AhAAG)
- CLAUDE.md updates (that's ₢AhAAG)

### slash-command-cleanup-claudemd-sync (₢AhAAG) [complete]

**[260306-0915] complete**

Delete eliminated and demoted slash command files, update CLAUDE.md managed section to match. Anticipate V4 bridling elimination throughout.

## Slash command files to DELETE (eliminated concepts — verb removed entirely)

- .claude/commands/jjc-pace-bridle.md
- .claude/commands/jjc-heat-quarter.md
- .claude/commands/jjc-heat-braid.md
- .claude/commands/jjc-heat-garland.md

## Slash command files to DELETE (demoted — verb survives as verb table entry)

- .claude/commands/jjc-pace-slate.md
- .claude/commands/jjc-pace-reslate.md
- .claude/commands/jjc-pace-notch.md
- .claude/commands/jjc-heat-rail.md
- .claude/commands/jjc-heat-furlough.md
- .claude/commands/jjc-heat-restring.md
- .claude/commands/jjc-heat-retire-dryrun.md
- .claude/commands/jjc-heat-retire-FINAL.md
- .claude/commands/jjc-heat-mount.md
- .claude/commands/jjc-heat-groom.md

## Bridling elimination simplifies everything

V4 eliminates bridling. This removes the entire bridled-pace execution path from mount (agent spawning, warrant parsing, wrap discipline, landing commits — lines 120-167 of jjc-heat-mount.md). The surviving mount protocol is: orient → display context → name assessment → analyze docket → propose approach → work → record. This is verb-table-sized.

Similarly: groom drops bridled-pace suggestions, quarter references disappear, the Bridleability Assessment section is deleted.

## CLAUDE.md managed section updates

### Remove entirely
- Eliminated verbs from Quick Verbs table: bridle, quarter, braid, garland
- jjx_arm from CLI Command Reference
- Bridleability Assessment section
- All references to bridled state, warrants (V3 sense), and autonomous agent dispatch
- "JJ Slash Command Reference" table (replaced by verb table)

### Distill slash command logic into inline guidance

**Notch** → Commit Discipline section. Already mostly there. Add:
- Synthesize intent from conversation (--intent flag)
- jjx_record invocation pattern for pace-affiliated and heat-affiliated commits
- Size guard handling (attempt, report on failure, ask user)

**Mount** → New "Mount Protocol" subsection in verb table area. With bridling gone, this is concise:
- Run jjx_orient [firemark|coronet] to get context
- Parse output: Heat/Paddock/Next/Docket/Recent-work sections
- Display context to user
- Name assessment: if silks doesn't fit docket, offer rename via jjx_relabel
- Analyze docket, propose approach (2-4 bullets), ask to proceed
- All paces are rough (no bridled path) — always interactive

**Groom** → Verb table entry with brief guidance:
- Run jjx_show <firemark> --detail --remaining
- Display overview, summarize progress
- Enter planning mode: suggest structural operations (slate, rail, reslate, paddock review)

### Rewrite guidance pattern
- Remove "Read the corresponding slash command before attempting JJ operations" — that pattern is gone
- Replace all /jjc-* references with direct jjx_ commands or verb names
- The verb table + CLI reference + inline guidance sections are now the sole authority

### Skill system acknowledgment
- Deleting .claude/commands/jjc-*.md files means /jjc-* skills stop working
- This is intentional — V4 verb table replaces them
- Surviving slash commands: only /jjc-pace-notch-equivalent inline guidance and future school/corral

## Depends on

- ₢AhAAF (jjs0-upper-api-restructure) — spec authority established first

## Files

- .claude/commands/jjc-*.md (14 files deleted)
- CLAUDE.md (managed JJK section updated)

### retire-command-rework (₢AhAAI) [complete]

**[260307-1251] complete**

Rework jjx_archive (retire) command: (1) Remove dry-run mode — always execute. (2) Fail-fast guard: reject immediately if gallops has uncommitted changes. (3) Accept optional size_limit param for commit guard retry. (4) Ensure clean rollback on size-guard failure. (5) Route all error messages to buf, not eprintln, so MCP agent sees diagnostics.

### add-schema-version-detection (₢AhAAB) [complete]

**[260311-1537] complete**

Add schema_version field to jjrg_Gallops for V3/V4 detection.

## V3 field
No schema_version field exists in V3.

## V4 change
Add `schema_version: Option<u32>` with `#[serde(default, skip_serializing_if = "Option::is_none")]` to jjrg_Gallops. Wait — we want V4 to ALWAYS write it, so: `#[serde(default)]` only. V3 files deserialize as None. V4 save writes Some(4).

## Migration transform
In jjdr_load: extend migration detection. Currently `is_migration_mode = gallops.heat_order.is_empty()`. Add: `|| gallops.schema_version.is_none()`. When migrating, set `gallops.schema_version = Some(4)`. Next save writes the field.

## Files
- jjrt_types.rs — add field to jjrg_Gallops
- jjri_io.rs — extend migration detection in jjdr_load
- jjrv_validate.rs — add schema_version validation (must be Some(4) for V4)
- jjtg_gallops.rs — update tests

## Acceptance
- V3 gallops (no schema_version) load successfully with migration mode active
- Save writes schema_version: 4
- Round-trip check passes on V4 files
- Subsequent load of saved file is non-migration mode

### snapshot-v3-types-and-spec (₢AhAAC) [complete]

**[260311-1638] complete**

Freeze the current V3 schema as a reference artifact for migration work.

## Deliverables

1. **Rust snapshot**: Copy jjrt_types.rs to jjrt_v3_types.rs. Add a file header comment: "Frozen V3 types — DO NOT EDIT. Reference for V3→V4 migration in jjdr_load. Will be deleted by ₣An (jjk-v4-release-and-legacy-removal)."

2. **JJS0 spec section**: Add a "V3 Legacy Schema Reference" section to JJS0-GallopsData.adoc summarizing the V3 record/member/type definitions. Mark it clearly as frozen reference, not active spec.

3. **Wire V3 types into migration path**: Update jjri_io.rs to `use jjrt_v3_types` for deserializing V3 gallops in migration mode. The V3 types become the "read" side; V4 types become the "write" side.

## Files
- jjrt_types.rs — source for snapshot (do not modify)
- jjrt_v3_types.rs — NEW frozen copy
- JJS0-GallopsData.adoc — add V3 reference section
- jjri_io.rs — import V3 types for migration deserialize path
- Cargo.toml / mod.rs — register new module

## Acceptance
- jjrt_v3_types.rs exists and is byte-identical to current jjrt_types.rs (minus header comment)
- JJS0 has a clearly marked V3 reference section
- Builds clean

### design-v4-data-model (₢AhAAA) [rough]

**[260317-1829] rough**

Opus-tier design conversation: define the V4 pace and state model in JJS0.

## Scope (narrowed from original 13-item list)

This pace produces JJS0 spec sections for the pace/state core:

1. **jjrg_Pace V4 type definition** — flat fields replacing tack array (state, silks, docket, basis, ts, chain). Chain is inter-pace dependency set at slate/reslate time — structural metadata, not discovered by school. Longe depends on chains being pre-set to assess pace readiness.
2. **State enum** with minted prefixes — jjep_ (green/ready/reined/candidate/complete/abandoned)
3. **V3 migration mapping** — for every V3 pace/tack field, document what V4 does with it. Note: V3 `direction` has no V4 pace-level equivalent — warrants now live per-beat in the gallops beat table (designed in ₢AhAAQ).
4. **Tack elimination consequences** — what history is lost, is git history sufficient as audit trail

## What moved to other paces

- Tackle/blaze schema → design-v4-tackle-blaze (₢AhAAO)
- Gait/quirt schema → design-v4-gait-quirt (₢AhAAP)
- Martingale/volte model, kimberwick identity, beat table schema → design-v4-martingale-beat-identity (₢AhAAQ)

## Design constraints

- **AXLA coherence**: jjk minted enum/field names are voicings of AXLA motifs
- **Paddock is authoritative**: state enum, flat fields, chain semantics, and migration strategy are already decided there — this pace codifies them into spec

## Depends on
- ₢AhAAC (snapshot-v3-types-and-spec) — V3 reference must exist before specifying V4 replacements

## Produces
- JJS0 section: V4 Pace Data Model
- JJS0 section: V4 State Enums
- JJS0 section: V3→V4 Pace Migration Mapping

## Not bridleable

Design conversation — codifying paddock decisions into spec requires judgment about edge cases and field semantics.

### design-v4-tackle-blaze (₢AhAAO) [rough]

**[260317-1818] rough**

Opus-tier design conversation: define the V4 tackle and blaze data model in JJS0.

## Scope

This pace produces JJS0 spec sections for the tackle/blaze system:

1. **jjrg_Tackle type definition** — silks-identified, mutable, contains ₿-named file scopes, actions (command + exclusivity), gait affinities, read-also entries
2. **Blaze (₿) mechanics** — definition ownership (tackle defines, gaits/dockets reference), uniqueness scope (per-heat), validation rules (no duplicates, no orphan references)
3. **Tackle-gait interface contract** — how gaits declare required ₿ blazes, how jjx validates compatibility at school time
4. **Tackle integrity declarations** — build commands, test runs, linters, and quality checks that a tackle makes available. These inform school's chukker boundary decisions: whether mechanical git merge with tackle-declared checks suffices, or explicit alignment beats are needed.
5. **Exclusivity scopes** — jjee_ enum (worktree/workstation/project), how jjx enforces mutexes during concurrent chukker dispatch
6. **Top-level gallops key** — tackles registry alongside gaits registry

## Design constraints

- Tackles decouple dockets from concrete paths — school resolves blazes when writing per-beat warrant fields in the gallops beat table
- Blaze is NOT a global identity (not in ₣/₢/₡/Ꝗ table) — scoped to tackle, resolved at school time
- Paddock tackle/blaze vocabulary is authoritative starting point

## Depends on
- ₢AhAAA (design-v4-data-model) — pace/state model should be settled first

## Produces
- JJS0 section: V4 Tackle Data Model
- JJS0 section: Blaze Mechanics
- JJS0 section: Tackle Integrity Declarations
- JJS0 section: Exclusivity Enforcement

## Not bridleable

Design conversation — tackle is a new concept requiring careful edge-case exploration.

### design-v4-gait-quirt (₢AhAAP) [rough]

**[260317-2051] rough**

Opus-tier design conversation: define the V4 gait record, beat well-formedness model, and quirt identity in JJS0.

Scope: This pace produces JJS0 spec sections for the gait system:

1. **jjrg_Gait record fields** -- silks (shared across versions), default model preferences, confidence gates, required blazes. Beat templates carry a **piaffe** -- the named prose field declaring purpose, approach, nosebag production/expectations, and resources.
2. **Piaffe structure and nosebag declarations** -- piaffe contains: purpose, approach, nosebag production (sections this beat emits in provender), nosebag expectations (nosebag names needed from prior chukkers), resources (file scopes, blaze references). Also: auto-tool declarations and quality requirements.
3. **Nosebag routing contract** -- how piaffes declare what nosebags each beat produces/expects for jjx to match headers at chukker boundaries. Name-matched, not ID-matched.
4. **Chukker assignment** -- beat templates carry chukker numbers. Same-chukker beats concurrent; chukkers sequential. School inherits chukker structure from gait.
5. **Merge/alignment beats** -- gaits CAN name alignment beats at chukker boundaries. Not required. Tackle integrity declarations inform need.
6. **Quirt identity** -- allocation strategy, next_quirt seed, immutability.
7. **Gaits registry** -- top-level gallops key, keyed by quirt.

Design constraints:
- Gaits are school-time resources, not breeze-time
- Immutable snapshots; evolution mints new quirts with same silks
- Abstract beat (template+piaffe) vs concrete beat (warrant+provender) -- context disambiguates
- Docket to piaffe to warrant: intent to template to concrete (prose field escalation)
- Paddock authoritative (checklists for school, confidence gates)

Depends on: AhAAO (design-v4-tackle-blaze)

Produces:
- JJS0 section: V4 Gait Data Model
- JJS0 section: Piaffe Structure and Nosebag Declarations
- JJS0 section: Quirt Identity and Allocation

Not bridleable: Design conversation -- piaffe structure and nosebag routing contract are new territory (cchat-20260317b).

### design-v4-martingale-beat-identity (₢AhAAQ) [rough]

**[260317-2052] rough**

Opus-tier design conversation: define the V4 beat table schema, martingale/volte data model, and kimberwick identity system in JJS0.

Scope: This pace produces JJS0 spec sections for the beat table and identity infrastructure:

1. **Beat table schema** -- formalize the gallops beat table: kimberwick (key), coronet (immutable pace ref), warrant (resolved prompt), model, file scope, chukker (integer), status (pending/complete/failed), source quirt, provender (markdown document, optional, empty until completion, immutable thereafter). Beats are a flat collection; paces don't contain beats, beats point to their pace. Provender is the only field that changes after beat creation -- set once, indelibly, on completion.
2. **Beat table gallops location** -- resolve the open question: top-level collection keyed by kimberwick, or scoped under martingale? Evaluate serialization trade-offs, access patterns during breeze (including nosebag collection at chukker boundaries), and cleanup when a volte completes.
3. **Martingale/volte data model** -- next_martingale seed on heat, martingale identity (TBD symbol, 4 chars: 2 heat + 2 index). Volte lifecycle: gallops beat table carries execution state. One active volte per heat. What happens to beat table entries (including provender) when corral completes or rejects a volte.
4. **Kimberwick identity** -- TBD symbol, 6 chars (4 martingale + 2 index), 4,096 slots per martingale. Allocation strategy. If school exhausts slots, breeze and corral what exists.
5. **Identity system update** -- complete table with all five V4 symbols, document three naming regimes (minted/silks/blazes), select Unicode symbols for martingale and kimberwick.
6. **Beat status lifecycle** -- pending to complete or pending to failed. Completion transition records provender indelibly. Can a failed beat be retried (re-set to pending)? Or is retry a new volte?

What moved to other paces:
- Chukker boundary mechanics, branch-per-beat lifecycle, merge model, crash recovery: AhAAT
- Concurrent dispatch, instance lifecycle, throttle management: AhAAU
- Nosebag routing mechanics (jjx matching provender headers at chukker boundaries): AhAAT

Design constraints:
- Warrant is per-beat, fully resolved
- Provender is per-beat, indelible at completion -- declared informational output, not journaling
- Volte execution state lives in gallops beat table, not in git branch existence
- Beat status: three states only (pending/complete/failed)
- Provender scoped to martingale -- lives and dies with the volte

Depends on: AhAAP (design-v4-gait-quirt) -- beat rows cite source quirts; gait model should be settled

Produces:
- JJS0 section: V4 Beat Table Schema (including provender field)
- JJS0 section: Martingale and Volte Model
- JJS0 section: Kimberwick Identity
- JJS0 section: V4 Identity System (5 symbols)

Not bridleable: Design conversation -- beat table location, identity symbol selection, provender lifecycle at volte completion, all require interactive exploration. Provender field is new (cchat-20260317b).

### design-v4-chukker-execution (₢AhAAT) [rough]

**[260317-2052] rough**

Opus-tier design conversation: define the V4 chukker execution mechanics, branch-per-beat lifecycle, and merge model in JJS0.

Scope: This pace produces JJS0 spec sections for execution infrastructure:

1. **Chukker sequencing** -- jjx dispatches all beats in chukker N concurrently, waits for completion/failure, advances to N+1. At each boundary: mechanical merge, provender collection from completed beats, nosebag assembly per consuming beats' piaffe expectations. Sequencing logic, termination conditions, partial-chukker semantics.
2. **Provender collection and nosebag routing** -- at chukker boundaries, jjx scans completed prior-chukker beats for provender, parses markdown headers, matches against consuming beats' piaffe expects declarations, delivers matched nosebag sections alongside warrants. jjx reads structure (headers) but never interprets prose content. Name-matched, not ID-matched.
3. **Branch-per-beat lifecycle** -- branches created just-in-time when a beat's chukker becomes active. Branch naming: jj/MARTINGALE/KIMBERWICK. Worktree creation from staging point, worktree disposal after beat completion. Branches are execution artifacts, not planning artifacts.
4. **Chukker boundary merge** -- mechanical git merge of all prior-chukker beat branches into staging point before next-chukker branches fork. Fail-fast on conflict.
5. **Semantic merge beats** -- how gaits name explicit alignment beats at chukker boundaries. Merger beats receive assembled context: warrants (intent), beat diffs (outputs), docket (goal), and nosebags from prior beats. jjx constructs this context.
6. **Tackle-informed boundary checks** -- how tackle build/test/integrity-check declarations integrate with chukker boundaries.
7. **Crash recovery** -- idempotent re-dispatch of pending beats. No dispatched state. Orphan worktree cleanup. Provender from completed beats survives crash (already indelible in gallops).
8. **Beat failure semantics** -- when a beat fails: does the whole chukker fail? Does the volte halt? What information (including any partial provender) is available for diagnosis?

Design constraints:
- Chukker model replaces arbitrary DAG -- one integer field per beat
- Beat status: pending/complete/failed only
- Mechanical merge is jjx infrastructure; semantic merge is an explicit beat
- Branch-per-beat is just-in-time, not pre-planned
- Provender routing is jjx infrastructure alongside mechanical merge (cchat-20260317b)

Depends on:
- AhAAQ (design-v4-martingale-beat-identity) -- beat table schema including provender field
- AhAAP (design-v4-gait-quirt) -- gait merge/alignment beat and nosebag declarations

Produces:
- JJS0 section: Chukker Sequencing
- JJS0 section: Provender Collection and Nosebag Routing
- JJS0 section: Branch-Per-Beat Lifecycle
- JJS0 section: Chukker Boundary Merge
- JJS0 section: Crash Recovery and Beat Failure

Not bridleable: Execution mechanics have subtle edge cases. Nosebag routing at chukker boundaries is new (cchat-20260317b).

### design-v4-concurrent-dispatch (₢AhAAU) [rough]

**[260317-1845] rough**

Opus-tier design conversation: define the V4 concurrent dispatch model for OAuth-authenticated Claude Code instances in JJS0.

## Scope

This pace produces JJS0 spec sections for multi-instance dispatch:

1. **Instance lifecycle** — how jjx launches a Claude Code instance per beat within a chukker. OAuth authentication flow. Worktree binding (one instance per worktree per beat). Instance startup, prompt delivery, result collection, instance teardown.
2. **Throttle management** — staggered launches (1-2 seconds) for RPM mitigation. Different model tiers have independent rate limit pools. How jjx selects launch timing. What happens when rate limits are hit mid-chukker.
3. **Model-tier enforcement** — jjx requires model-identity parameter on every invocation. Self-reported identity. Hard gate: breeze-phase commands refuse non-haiku orchestrators. Model attribution in commit history.
4. **Result collection** — how jjx detects beat completion (worktree commits? exit code? status file?). Mapping instance output back to kimberwick identity. Updating beat status in gallops.
5. **Exclusivity enforcement** — jjee_ enum (worktree/workstation/project). How jjx holds mutexes for workstation and project-scoped actions during concurrent dispatch. What happens when two beats in the same chukker need an exclusive resource.
6. **Error handling** — instance crash vs beat failure vs throttle exhaustion. Which are retryable (pending re-dispatch) vs terminal (beat marked failed).

## Design constraints

- V4/₣Ah uses OAuth-authenticated Claude Code instances only; API-key REST dispatch deferred
- Bare prompt reproducibility: same prompt + same model + same worktree state = comparable output
- jjx is the conductor — dispatches warrants, collects commits, never interprets beat output
- Haiku-tier orchestration: breeze dispatch is mechanical, not judgmental

## Depends on
- ₢AhAAT (design-v4-chukker-execution) — chukker sequencing and branch lifecycle must be settled

## Produces
- JJS0 section: Concurrent Dispatch Model
- JJS0 section: Instance Lifecycle
- JJS0 section: Throttle and Rate Limit Management
- JJS0 section: Exclusivity Enforcement

## Not bridleable

Design conversation — concurrent dispatch has operational edge cases (throttle exhaustion, instance crashes, exclusive resource contention) that need interactive exploration. Also: first concrete design of the jjx-to-Claude-Code interface contract.

### spec-v4-command-surface (₢AhAAM) [rough]

**[260317-2053] rough**

Specify the complete V4 command surface: what verbs exist, what each does with the beat table and martingale lifecycle, and how existing commands change.

Why this pace exists: The data model paces define types. The execution paces define mechanics. The command surface is the user-facing contract.

What this pace produces -- JJS0 sections covering:

1. **New command specs**: school (reads docket + codebase + gait piaffes, produces beat table entries with nosebag flow), breeze (triggers chukker-sequenced dispatch with provender collection/nosebag routing), corral (per-pace review with provender visible for context), longe (heat-level readiness), tackle CRUD, gait management commands.
2. **Modified command specs**: orient, show, mount, notch, wrap -- how each changes for V4 beat-table/martingale awareness. Show displaying beat status, chukker progress, and provender summaries. Orient surfacing active martingale and next actionable context.
3. **Phase boundary commands** -- what jjx validates at each transition: pre-school blaze/chain checks, pre-breeze beat table integrity (including nosebag expectation satisfaction), post-corral volte completion.
4. **Kimberwick exhaustion handling** -- school runs out of 4,096 slots: how surfaced? What commands remain available?
5. **Beat table visibility** -- which commands expose beat-level detail (show with detail flag? provender content?) vs which stay at pace level. How beat status and provender roll up to pace-level display. Corral sees provender for context but evaluates at pace level.
6. **Provender inspection** -- how users view provender and nosebag content for debugging/understanding beat output. Read-only visibility into the informational flow.

What lives in other paces:
- Chukker sequencing, branch-per-beat lifecycle, nosebag routing mechanics: AhAAT
- Concurrent dispatch, OAuth instance management: AhAAU
- Beat table schema including provender field: AhAAQ

Depends on:
- AhAAA (design-v4-data-model) -- pace/state types
- AhAAQ (design-v4-martingale-beat-identity) -- beat table schema with provender
- AhAAT (design-v4-chukker-execution) -- execution mechanics including nosebag routing
- AhAAU (design-v4-concurrent-dispatch) -- dispatch model

Not bridleable: Command UX for provender visibility and phase transition contracts require judgment. Provender inspection is new (cchat-20260317b).

### define-v4-pace-struct (₢AhAAD) [rough]

**[260317-1830] rough**

Implement the V4 pace data model as designed in ₢AhAAA. Replace tack array with flat fields.

## V3 field
jjrg_Pace contains `tacks: Vec<jjrg_Tack>`. Each tack has: ts, state, text, silks, basis, direction.

## V4 change
jjrg_Pace gets flat fields: state, silks, docket (was text), basis, ts, chain. Exact field definitions come from ₢AhAAA design output. New state enum replaces jjrg_PaceState. Chain is inter-pace dependency set at slate/reslate time.

**Note**: V3 `direction` mapped to pace-level `warrant` in earlier designs. Per cchat-20260317, warrants now live per-beat in the gallops beat table (designed in ₢AhAAQ). Whether pace retains a `warrant` field (possibly repurposed or vestigial for migration) is for ₢AhAAA to determine. Beat table type implementation is a separate concern from this pace.

## Migration transform
In jjdr_load V3 migration path: deserialize as V3 types (jjrt_v3_types), extract tacks[0] values into flat V4 fields. Tack history (tacks[1..]) is discarded — git history preserves the audit trail. V3 `direction` field handling depends on ₢AhAAA's decision about pace-level warrant.

## Depends on
- ₢AhAAA (design-v4-data-model) — provides the type definitions
- ₢AhAAB (add-schema-version-detection) — provides migration detection
- ₢AhAAC (snapshot-v3-types-and-spec) — provides V3 types for migration read

## Files
- jjrt_types.rs — rewrite jjrg_Pace, jjrg_PaceState, remove jjrg_Tack
- jjri_io.rs — implement V3→V4 migration transform
- jjrv_validate.rs — rewrite pace validation for V4 fields
- jjtg_gallops.rs — rewrite tests for V4 types

## Acceptance
- V3 gallops load, migrate in memory, save as V4
- V4 gallops load with strict round-trip validation
- All existing jjx_* commands compile (but may need adaptation in next pace)

### adapt-commands-to-v4-types (₢AhAAE) [rough]

**[260317-1818] rough**

Update all jjx_* command implementations to use V4 flat pace fields instead of tack array access.

## V3 pattern
Commands access pace data via: pace.tacks[0].text, pace.tacks[0].state, pace.tacks[0].silks, pace.tacks[0].basis, pace.tacks.first().unwrap().direction

## V4 pattern
Direct field access: pace.docket, pace.state, pace.silks, pace.basis. Note: `pace.warrant` from earlier designs may not exist as a pace-level field — per cchat-20260317, warrants live per-beat in the gallops beat table. Exact V4 field set determined by ₢AhAAA and implemented in ₢AhAAD.

## Scope
Every Rust file in jjk/vov_veiled/src/ that reads or writes pace fields. This is the widest pace — touches most command implementations. But it is mechanical: find tack access patterns, replace with flat field access. Beat table access patterns are a separate concern from this pace.

## Depends on
- ₢AhAAD (define-v4-pace-struct) — V4 types must exist and compile

## Files
- All jjr*_.rs command files that access pace data
- jjru_util.rs — remove jjrg_make_tack if it exists
- Any helper functions that construct or destructure tacks

## Acceptance
- All commands work against V4 gallops
- cargo build clean, cargo test pass
- End-to-end: nominate heat, slate pace, show pace, close pace — all work

### csv-table-cue-trial (₢AhAAK) [abandoned]

**[260402-0735] abandoned**

Drafted from ₢AYAAB in ₣AY.

Iterate on minimum viable CLAUDE.md cue to get models to format CSV as a table.

## Approach

Build a trivial `jjx_table` command that echoes CSV stdin back as raw CSV output. Then run controlled trials across model tiers and cue levels to find the simplest cue that reliably produces formatted tables.

## Step 1: Build jjx_table

Minimal Rust command:
- Reads CSV lines from stdin
- Echoes them back unchanged to stdout
- No formatting, no parsing — just a passthrough

Files: new command file in Tools/jjk/vov_veiled/src/, register in jjrx_cli.rs

## Step 2: Define cue levels

| Level | CLAUDE.md Cue |
|-------|--------------|
| 0 | Nothing — no cue at all |
| 1 | "jjx_table outputs CSV. Present as a table." |
| 2 | "When jjx commands emit CSV, render as markdown table for the user." |
| 3 | Full slash command with step-by-step instructions (control/baseline) |

## Step 3: Run trials

For each cue level, test across model tiers using Task tool with model param:
- Opus: Task(model="opus")
- Sonnet: Task(model="sonnet")
- Haiku: Task(model="haiku")

Each trial: call jjx_table with 3-5 CSV rows, capture model's response, grade pass/fail (did it produce a markdown table with correct values?).

Test CSV payload (fixed across trials):
```
Name,Status,Count
alpha,racing,5
beta,stabled,12
gamma,retired,8
```

## Step 4: Record results

Build a 4x3 matrix (cue level x model tier) in a memo documenting:
- Pass/fail per cell
- Qualitative notes (added headers? extra commentary? reformatted values?)
- Minimum viable cue level per tier

## Step 5: Iterate

If Level 2 works universally, that's the target cue for CLAUDE.md.
If tiers diverge, consider whether --model flag lets jjx pre-format for weaker models.

## Success Criteria
- jjx_table command builds and runs
- At least 3 trials per cell in the matrix
- Clear recommendation for minimum cue wording
- Memo with findings: Memos/memo-YYYYMMDD-cue-calibration-csv-table.md

### rename-bcg-document-identity (₢AhAAX) [rough]

**[260331-1128] rough**

Drafted from ₢AUAAi in ₣AU.

Drafted from ₢ArAAA in ₣Ar.

HEAVY RESLATE NEEDED before mounting. The original docket (rename BCG document identity, BCS vs BMP decision) was drafted in the context of rbk-mvp-finalization. By the time jjk-v4-2 work begins, the BUK kit structure, naming conventions, and veiled-document patterns may have evolved significantly. Re-examine the renaming question from scratch in the v4.2 context — the right answer may be different, or the pace may no longer be needed at all.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 V select-martingale-kimberwick-symbols
  2 N cull-v3-spec-chalk-tack-bridle-arm
  3 L fix-restring-eprintln-and-claudemd-coronets-doc
  4 F jjs0-upper-api-restructure
  5 G slash-command-cleanup-claudemd-sync
  6 I retire-command-rework
  7 B add-schema-version-detection
  8 C snapshot-v3-types-and-spec
  9 A design-v4-data-model

VNLFGIBCA
·x·x···x· JJS0-GallopsData.adoc
·xx····x· lib.rs
·xx·x···· CLAUDE.md, vocjjmc_core.md
······xx· jjri_io.rs
··x···x·· jjrno_nominate.rs
··x··x··· jjrrt_retire.rs
·x···x··· jjrm_mcp.rs
·xx······ jjrtl_tally.rs
·······x· RBS0-SpecTop.adoc, jjrt_v3_types.rs, rbrr_reset_cli.sh, rbw-MR.MarshalReset.sh, rbz_zipper.sh
······x·· jjrt_types.rs, jjrv_validate.rs, jjtfu_furlough.rs, jjtg_gallops.rs, jjtgl_garland.rs, jjtpd_parade.rs, jjtq_query.rs, jjtrl_rail.rs, jjtrs_restring.rs
····x···· jjc-heat-braid.md, jjc-heat-furlough.md, jjc-heat-garland.md, jjc-heat-groom.md, jjc-heat-mount.md, jjc-heat-quarter.md, jjc-heat-rail.md, jjc-heat-restring.md, jjc-heat-retire-FINAL.md, jjc-heat-retire-dryrun.md, jjc-pace-bridle.md, jjc-pace-notch.md, jjc-pace-reslate.md, jjc-pace-slate.md
··x······ jjrc_core.rs, jjrch_chalk.rs, jjrcu_curry.rs, jjrdr_draft.rs, jjrfu_furlough.rs, jjrgc_get_coronets.rs, jjrgl_garland.rs, jjrgs_get_spec.rs, jjrld_landing.rs, jjrmu_muster.rs, jjrnc_notch.rs, jjrpd_parade.rs, jjrrl_rail.rs, jjrrs_restring.rs, jjrsc_scout.rs, jjrsd_saddle.rs, jjrsl_slate.rs, jjrvl_validate.rs, jjrwp_wrap.rs, rbdc_DerivedConstants.sh
·x······· JJSCTL-tally.adoc, jjrn_notch.rs, jjrnm_markers.rs, jjtn_notch.rs

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 121 commits)

  1 V select-martingale-kimberwick-symbols

123456789abcdefghijklmnopqrstuvwxyz
··························xx·······  V  2c
```

## Steeplechase

### 2026-07-05 20:59 - Heat - d

paddock curried: tackle cull: core definition re-pointed to the aspirant tackle sheaf (one-word-one-concept floor), only the ₣Ah extension layer stays homed here; gallops-registry row marked extension-layer schema; legacy bare-# title demoted to ## for gazette wire safety

### 2026-05-08 14:11 - Heat - f

stabled

### 2026-05-08 14:11 - Heat - f

silks=jjk-v4-4-school-breeze-founding

### 2026-04-02 07:35 - Heat - T

csv-table-cue-trial

### 2026-03-31 11:28 - Heat - D

restring 1 paces from ₣AU

### 2026-03-27 02:37 - Heat - n

Add vvcp_invitatory to VVC: gap-guarded officium commit (1-hour threshold via git log). Adds chrono dependency to vvc. Re-exports from lib.rs. Completes the missing piece that jjrm_mcp.rs calls.

### 2026-03-27 02:31 - Heat - n

Fix MCP server startup thrash: replace JJK-internal officium reimplementation with lazy call to VVC vvcp_invitatory (which has 1-hour gap guard). Remove exchange dir creation, compline, mutex/OnceLock machinery. Server startup is now zero-cost; invitatory deferred to first jjx command.

### 2026-03-18 05:53 - ₢AhAAV - W

Selected Unicode symbols for martingale (₼ U+20BC manat) and kimberwick (₭ U+20AD kip), continuing the accidental currency-symbol pattern. Updated paddock identity table, vocabulary definitions, V4 execution infrastructure section, and moved items from Still Open to Resolved.

### 2026-03-18 05:53 - ₢AhAAV - n

Select Unicode symbols for Martingale (₼) and Kimberwick (₭) identifiers

### 2026-03-17 21:16 - Heat - T

spec-paces-conceptual-maturity-gate

### 2026-03-17 21:16 - Heat - S

design-v4-mcp-file-exchange

### 2026-03-17 20:53 - Heat - T

spec-v4-command-surface

### 2026-03-17 20:52 - Heat - T

design-v4-chukker-execution

### 2026-03-17 20:52 - Heat - T

design-v4-martingale-beat-identity

### 2026-03-17 20:51 - Heat - T

design-v4-gait-quirt

### 2026-03-17 20:51 - Heat - T

design-v4-gait-quirt

### 2026-03-17 20:51 - Heat - T

design-v4-gait-quirt

### 2026-03-17 20:51 - Heat - T

design-v4-gait-quirt

### 2026-03-17 20:50 - Heat - T

design-v4-gait-quirt

### 2026-03-17 20:49 - Heat - T

design-v4-gait-quirt

### 2026-03-17 20:47 - Heat - n

Add provender, nosebag, and piaffe concepts to V4 paddock: beat informational output, addressable subsections, and gait beat template prose field naming

### 2026-03-17 19:34 - Heat - f

racing

### 2026-03-17 18:52 - Heat - T

spec-paces-conceptual-maturity-gate

### 2026-03-17 18:51 - Heat - S

select-martingale-kimberwick-symbols

### 2026-03-17 18:47 - Heat - T

spec-paces-conceptual-maturity-gate

### 2026-03-17 18:46 - Heat - T

spec-v4-command-surface

### 2026-03-17 18:46 - Heat - T

design-v4-martingale-beat-identity

### 2026-03-17 18:45 - Heat - S

design-v4-concurrent-dispatch

### 2026-03-17 18:45 - Heat - S

design-v4-chukker-execution

### 2026-03-17 18:43 - Heat - S

spec-paces-conceptual-maturity-gate

### 2026-03-17 18:32 - Heat - f

stabled

### 2026-03-17 18:30 - Heat - T

define-v4-pace-struct

### 2026-03-17 18:29 - Heat - T

design-v4-data-model

### 2026-03-17 18:26 - Heat - d

paddock curried

### 2026-03-17 18:21 - Heat - d

paddock curried

### 2026-03-17 18:18 - Heat - T

adapt-commands-to-v4-types

### 2026-03-17 18:18 - Heat - T

design-v4-tackle-blaze

### 2026-03-17 18:17 - Heat - T

define-v4-pace-struct

### 2026-03-17 18:16 - Heat - T

spec-v4-command-surface

### 2026-03-17 18:15 - Heat - T

design-v4-gait-quirt

### 2026-03-17 18:14 - Heat - T

design-v4-martingale-beat-identity

### 2026-03-17 18:14 - Heat - T

design-v4-warrant-volte-identity

### 2026-03-17 18:12 - Heat - d

paddock curried

### 2026-03-13 12:38 - Heat - S

size-guard-rename-awareness

### 2026-03-11 16:38 - ₢AhAAC - W

Froze V3 schema as migration reference: jjrt_v3_types.rs snapshot (byte-identical, module registered), V3 Legacy Schema Reference section in JJS0. Deliverable 3 (migration wiring) deferred — migration logic belongs in the pace that actually changes V4 types, not before.

### 2026-03-11 16:38 - ₢AhAAC - n

Freeze V3 migration spec as concise reference, add Marshal role and reset CLI (rbw-MR) for release-qualification regime blanking with zipper enrollment and tabtarget

### 2026-03-11 16:24 - ₢AhAAC - n

Freeze V3 schema as migration reference: snapshot types, add JJS0 V3 Legacy section, wire module with serde-compatibility migration discipline

### 2026-03-11 16:03 - Heat - S

design-v4-warrant-volte-identity

### 2026-03-11 16:03 - Heat - S

design-v4-gait-quirt

### 2026-03-11 16:03 - Heat - S

design-v4-tackle-blaze

### 2026-03-11 16:03 - Heat - T

design-v4-data-model

### 2026-03-11 16:00 - Heat - n

Surface tackle/blaze concepts and beat-merge-as-prompt-assembly resolution in paddock

### 2026-03-11 15:37 - ₢AhAAB - W

Added schema_version field to jjrg_Gallops for V3/V4 detection. V3 files (no field) deserialize as None triggering migration mode; V4 save writes Some(4). Added validation Rule 0, updated all Gallops constructors across 8 files. Build clean, 261 tests pass, live gallops migration verified.

### 2026-03-11 15:37 - ₢AhAAB - n

Add schema_version field to gallops with migration support and validation

### 2026-03-11 14:54 - ₢AhAAN - W

Culled dead V3 concepts from JJS0 spec and Rust: removed tack record/members/bridled-state from spec, rewrote pace as flat record, deleted arm command (spec+Rust), gutted chalk to W/d only (removed A/B/F markers), removed ~18 dead linked terms, rewrote command behaviors as flat field mutations, removed approach chalk step from mount protocol in CLAUDE.md. Build clean, 261 tests pass.

### 2026-03-11 14:54 - ₢AhAAN - n

Remove tack abstraction: flatten pace to direct mutable fields, drop bridled state, jjx_arm, jjx_mark, and A/F/B chalk markers

### 2026-03-11 14:29 - Heat - S

cull-v3-spec-chalk-tack-bridle-arm

### 2026-03-11 14:14 - ₢AhAAB - A

Read 4 files, add schema_version field to jjrg_Gallops, extend migration detection in jjdr_load, add validation, update tests. Sequential sonnet.

### 2026-03-11 11:43 - Heat - d

paddock curried

### 2026-03-10 20:29 - Heat - n

Promote equestrian vocabulary to top-level JJS0 section with vocabulary isolation rationale, racing/dressage glossary split, and presentation aliases

### 2026-03-08 10:55 - Heat - S

spec-v4-command-surface

### 2026-03-08 10:45 - Heat - T

design-v4-data-model

### 2026-03-08 10:40 - Heat - r

moved AhAAC after AhAAB

### 2026-03-08 10:40 - Heat - r

moved AhAAB to first

### 2026-03-08 10:33 - Heat - T

design-v4-data-model

### 2026-03-08 07:45 - ₢AhAAA - A

Resume interactive design session: reconcile V4 paddock schema into buildable data model spec for JJS0. Seven deliverables, opus-tier, sequential.

### 2026-03-07 12:51 - ₢AhAAA - A

Interactive design session: reconcile V4 paddock schema decisions into buildable data model spec for JJS0. Walk 7 deliverables (types, state enum, gait fields, warrant shape, tack elimination, quirt mechanics, V3 migration mapping).

### 2026-03-07 12:51 - ₢AhAAI - W

Completed retire command rework: removed dry-run mode (always execute), added fail-fast dirty-gallops guard via git status --porcelain, size_limit param already present, rollback on size-guard failure already present, all errors routed through jjbuf!. Removed execute param from MCP ArchiveParams and CLI args.

### 2026-03-07 12:51 - ₢AhAAI - n

Remove execute flag from retire command, always execute directly, add fail-fast guard rejecting uncommitted gallops changes

### 2026-03-07 12:45 - ₢AhAAI - A

Read retire.rs to confirm state, add size_limit param, implement rollback on size-guard failure, verify buf routing

### 2026-03-07 12:44 - ₢AhAAL - W

Added jjbuf! macro to jjrc_core.rs, converted 180 eprintln! calls to jjbuf! across 20 command handler files so MCP errors flow through transport instead of vanishing into stderr. Fixed CLAUDE.md and vocjjmc_core.md to document jjx_transfer coronets param as JSON array. Two library-level eprintln! calls in jjro_ops.rs and jjrs_steeplechase.rs remain as deferred scope (no buf in scope).

### 2026-03-07 12:44 - ₢AhAAL - n

Add rbdc_DerivedConstants.sh: extract credential path derivations from rbrr lock into dedicated module

### 2026-03-07 12:43 - ₢AhAAL - n

Add jjbuf! macro to jjrc_core, convert 180 eprintln! calls to jjbuf! across 20 command handlers so errors flow through MCP transport. Fix CLAUDE.md/vocjjmc_core.md: document jjx_transfer coronets param as JSON array.

### 2026-03-07 12:07 - ₢AhAAL - A

Fix eprintln→buf in restring, fix coronets doc in CLAUDE.md/vocjjmc, audit other commands for same pattern

### 2026-03-07 12:05 - Heat - S

fix-restring-eprintln-and-claudemd-coronets-doc

### 2026-03-07 11:55 - Heat - D

AYAAB → ₢AhAAK

### 2026-03-07 11:55 - Heat - D

AYAAA → ₢AhAAJ

### 2026-03-06 20:08 - ₢AhAAI - A

Remove dry-run, add dirty-gallops guard, fix error routing to buf, remove execute param from MCP

### 2026-03-06 20:07 - Heat - r

moved AhAAI to first

### 2026-03-06 20:05 - Heat - S

retire-command-rework

### 2026-03-06 11:53 - ₢AhAAA - n

Paddock: quirt immutability pattern (latest silks match, no lineage fields)

### 2026-03-06 11:44 - ₢AhAAA - n

Paddock: caracole identity for voltes, no concurrent voltes, branch-as-state, volte→caracole terminology split

### 2026-03-06 11:07 - ₢AhAAA - n

Paddock: add longe phase, clarify school scope, compress 339→237 lines

### 2026-03-06 09:25 - ₢AhAAH - A

GO recommendation: MCP stdio transport, dual-mode vvx, simplifies V4 breeze/warrant model

### 2026-03-06 09:23 - ₢AhAAH - A

Evaluate MCP stdio transport: examine vvx entry point, assess 7 docket questions, produce go/no-go with sketch

### 2026-03-06 09:22 - Heat - S

consider-mcp-transport-for-jjx

### 2026-03-06 09:15 - ₢AhAAG - W

Deleted 14 jjc-* slash commands, rewrote CLAUDE.md JJK section (verb table, mount/groom/notch protocols, removed bridling), synced vocjjmc_core.md

### 2026-03-06 09:15 - ₢AhAAG - n

Delete 14 slash commands, rewrite CLAUDE.md JJK section: remove bridling, distill mount/notch/groom to inline guidance

### 2026-03-06 09:06 - ₢AhAAG - A

Delete 14 slash commands, rewrite CLAUDE.md JJK section: remove bridling, distill mount/notch/groom to inline guidance

### 2026-03-06 09:05 - ₢AhAAF - W

Merged jjsud_/jjsum_ into unified jjsuv_ prefix: 14 surviving verbs, removed bridle/quarter/braid/garland, rewrote slash commands section as V4 reservation

### 2026-03-06 09:05 - ₢AhAAF - n

Merge jjsud_/jjsum_ to jjsuv_ in mapping+definitions, remove eliminated verbs, rewrite slash commands section

### 2026-03-06 08:56 - ₢AhAAF - A

Merge jjsud_/jjsum_ to jjsuv_ in mapping+definitions, remove eliminated verbs, rewrite slash commands section

### 2026-03-06 08:54 - Heat - f

racing

### 2026-03-06 08:46 - Heat - T

slash-command-cleanup-claudemd-sync

### 2026-03-06 08:41 - Heat - T

slash-command-cleanup-claudemd-sync

### 2026-03-06 08:41 - Heat - T

jjs0-upper-api-restructure

### 2026-03-06 08:37 - Heat - S

slash-command-cleanup-claudemd-sync

### 2026-03-06 08:34 - Heat - S

jjs0-upper-api-restructure

### 2026-03-06 07:56 - Heat - f

stabled, silks=jjk-v4-1-school-breeze-founding

### 2026-03-04 15:46 - Heat - S

adapt-commands-to-v4-types

### 2026-03-04 15:46 - Heat - S

define-v4-pace-struct

### 2026-03-04 15:44 - Heat - S

snapshot-v3-types-and-spec

### 2026-03-04 15:44 - Heat - S

add-schema-version-detection

### 2026-03-04 15:42 - Heat - S

design-v4-data-model

### 2026-03-04 15:08 - Heat - n

paddock updates: V3/V4 migration discipline for Ah, v5-notional context for Am, release phases for An

### 2026-03-02 08:29 - Heat - n

Paddock: gait working concept — gaits as school checklists with confidence gates, research steps, review phases, derived from ₣Ai ₢AiAAz vocabulary migration experience

### 2026-03-01 12:19 - Heat - n

paddock: add multi-gait decomposition principle

### 2026-03-01 11:52 - Heat - n

paddock: quirt identity, warrant JSON structure, school incrementalism, memory challenge, gait library, attention model

### 2026-03-01 09:41 - Heat - n

paddock: V4 dreaming session — beats, voltes, corral, execution model, jjx-as-orchestrator

### 2026-02-24 20:53 - Heat - n

paddock: jjk v4 gaits memo

### 2026-02-24 20:53 - Heat - d

paddock curried

### 2026-02-24 20:35 - Heat - d

paddock curried

### 2026-02-24 20:13 - Heat - d

paddock curried

### 2026-02-24 12:20 - Heat - f

racing

### 2026-02-24 11:50 - Heat - d

V4 design memo — gaits, branches, breeze pipeline

### 2026-02-24 11:44 - Heat - d

paddock curried

### 2026-02-24 11:43 - Heat - d

paddock curried

### 2026-02-22 08:35 - Heat - n

rename chat reference to cchat-20260222-gallops-at-dawn

### 2026-02-22 08:34 - Heat - n

gallops-at-dawn-260222: V4 vision memo

### 2026-02-22 08:34 - Heat - f

stabled

### 2026-02-22 08:33 - Heat - N

jjk-v4-vision

