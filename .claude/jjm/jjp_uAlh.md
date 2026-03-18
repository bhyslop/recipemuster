# Paddock: jjk-v4-vision

## Current Design

**Authoritative design seeds**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline. Partially superseded by cchat-20260301 sessions and cchat-20260317 execution model refinements.

Key decisions: no leg layer, worktrees for isolation (branches persist after worktree disposal), composable gaits, four-phase pipeline (longe → school → breeze → corral), warrants as per-beat fields in gallops, jjx as LLM orchestrator. Git-as-mutex proven under concurrent pressure; V4 extends git protection to work product (beat branches).

## Identity System

| Symbol | Name | Digits | Structure | Slots |
|--------|------|--------|-----------|-------|
| `₣` | Firemark | 2 | 2 heat | 4,096 heats |
| `Ꝗ` | Quirt | 3 | 3 global | 262,144 gaits |
| TBD | Martingale | 4 | 2 heat + 2 index | 4,096/heat |
| `₢` | Coronet | 5 | 2 heat + 3 index | ~262K/heat |
| TBD | Kimberwick | 6 | 4 martingale + 2 index | 4,096/martingale |

Length alone disambiguates: 2=firemark, 3=quirt, 4=martingale, 5=coronet, 6=kimberwick.

Quirt `ꝖABC` identifies an immutable gait snapshot. When a gait evolves, a new quirt is minted with the same silks. "Latest version" = highest quirt with matching silks. No lineage fields needed.

## Vocabulary

| Term | Definition |
|------|------------|
| **Beat** | Abstract step within a gait: defines inputs, outputs, resources, and goals for one unit of work. In gallops execution tables, beat rows are concrete instances carrying resolved warrants. Context disambiguates: gait definition → abstract template, gallops table → concrete dispatch unit. |
| **Gait** | Reusable recipe of beats organized by chukker. Gaits can compose other gaits (practical depth limit: ~2 levels). Stored as data in gallops with quirt identity. School-time resource: school reads gaits for decomposition guidance and confidence gates, then produces concrete beat table entries. Breeze never looks up a gait. |
| **Warrant** | The resolved prompt school writes into a single beat table entry. Contains concrete instructions for one model dispatch. Fully resolved — no ₿ symbols, no gait references, only concrete values. Each warrant cites its source quirt for audit. |
| **Volte** | An attempt at executing one or more paces. One active volte per heat at a time. School creates it (populating beat table entries in gallops); breeze executes it (dispatching beats per chukker); corral reviews per-pace (accept/refine/reject). Identified by martingale. Dressage term: a precise, controlled circle back to the same point with refined intent. |
| **Martingale** | Volte identity. TBD symbol + 4 base64 characters (2 heat + 2 index). Named for the control strap that keeps the horse from going off course — what holds execution on track. Replaces "caracole" (cchat-20260317). |
| **Kimberwick** | Beat instance identifier. TBD symbol + 6 base64 characters (4 martingale + 2 index). 4,096 slots per martingale. Named for a type of bit providing precise, engineered control. Scoped to its martingale. If school exhausts slots, breeze and corral what exists — not an error condition. |
| **Chukker** | Numbered concurrency layer within a volte. All beats in chukker N execute concurrently; chukkers execute in sequence. School assigns each beat a chukker number. Named for a numbered period in polo — distinctive, won't bleed into generic usage. |
| **Quirt** | Gait identity. `Ꝗ` + 3 base64 characters. Named after a short riding whip — the thing that sets a gait in motion. |
| **Longe** | Heat-level readiness assessment. Classifies remaining paces as breezable / needs-refinement / blocked. Read-only. See Four Phases for detail. |
| **School** | Per-pace planning phase. Produces concrete beat table entries from docket + codebase investigation. Opus-tier, high human attention. See Four Phases for detail. |
| **Breeze** | Execution phase. jjx dispatches beats per chukker via OAuth-authenticated Claude Code instances. Zero human attention. See Four Phases for detail. |
| **Corral** | Per-pace review verb (parallels mount). Reviews composed pace outcome (not individual beats). Three outcomes: accept, refine + accept, or reject. No rebreeze — rejection flows upstream. See Four Phases for detail. |
| **Tackle** | Named resource bundle within a heat. Silks-identified, mutable. Contains ₿-named file scopes (blazes mapping symbolic names to file paths/globs), ₿-named actions (command + exclusivity), gait affinities, and read-also entries. Tackles let dockets and gaits reference files indirectly — school resolves ₿ references to concrete paths when building beat warrants. Tackle's build/test/integrity-check declarations inform school's chukker boundary decisions. |
| **Blaze** | Symbolic file/resource reference defined by a tackle. Notation: `₿guide-doc`. Tackles own blaze definitions; dockets and gaits reference them. School resolves blazes to concrete paths in warrants — breeze never sees ₿ symbols, only resolved paths. Global uniqueness within a heat. |

### Three Naming Regimes

| Regime | Scope | Example | Where used |
|--------|-------|---------|------------|
| Minted prefixes | Code, specs, cross-chat precision | `jjep_`, `jjfg_` | Rust types, AsciiDoc anchors, AXLA |
| Silks | Gallops entities (heats, paces, gaits, tackles) | `design-v4-data-model` | JSON data, human display |
| Blazes (₿) | Tackle slot references | `₿guide-doc` | Dockets, gaits — resolved by school |

### Beat: Abstract and Concrete

"Beat" serves as both abstract template (in gait definitions) and concrete instance (in gallops execution tables). The structural context disambiguates: a beat in a gait's recipe is a template with inputs/outputs/resources/goals; a beat row in gallops is a concrete dispatch unit with a resolved warrant, model, file scope, chukker number, and status. This dual usage is intentional — the word maps naturally to both contexts, and the home (gait library vs gallops table) eliminates ambiguity.

## Execution Model

### Four Phases

1. **Longe** (parallel, read-only) — assesses all remaining paces in a heat simultaneously. For each pace: reads docket, reads codebase to understand scope, identifies file types and potential gaits, classifies as breezable / needs-refinement / blocked. Output is a heat-level readiness report. Guides where to focus groom/reslate before committing to school.
2. **School** (opus, per-pace, conversational) — reads pace docket and codebase thoroughly (file reads, grep, structural understanding). Two internal phases: (a) assess docket quality against codebase reality, evaluate confidence gates; (b) produce concrete beat table entries in gallops, each carrying a resolved warrant with chukker assignment and model selection. May refuse to warrant ("this docket has gaps — here's what's wrong"). Does NOT web-search or produce work artifacts. Human Q&A refines the beat plan. One pace at a time.
3. **Breeze** (jjx-orchestrated, concurrent) — jjx reads beat table from gallops. For each chukker in sequence: merges prior-chukker branches, creates per-beat branches and worktrees, dispatches warrants as bare prompts to specified models via OAuth-authenticated Claude Code instances. Multiple beats within a chukker execute concurrently. jjx collects results and updates beat status in gallops. Haiku can orchestrate — precision, not judgment. Staggering beat launches by 1-2 seconds mitigates RPM throttle. Different model tiers have independent rate limit pools.
4. **Corral** (human + LLM, per-pace) — invoked per pace like mount, targeting next candidate (no param) or a specific pace. Reviews composed pace outcome against docket, diffs against base. Individual beats are internal detail — corral evaluates the whole. Three outcomes: (a) accept as-is → merge to main, pace complete; (b) refine interactively → agent and human adjust near-miss output, then merge; (c) reject → pace returns to pool for groom/reslate, eventual re-school produces new volte. No rebreeze — the fix is always upstream.

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

Three status states only. No "dispatched" — idempotent re-dispatch on crash recovery. Failed = critical issue encountered during execution, needs human attention.

No rationale, discovery notes, or "might need it later" fields. Lean records.

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

Merger beats receive assembled context from the post-mechanical-merge staging point: warrants (intent per beat), beat diffs (actual outputs), and docket (human's original goal). jjx constructs this context.

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

jjx orchestration is haiku-tier work. Reading beat tables, creating worktrees, dispatching per chukker, collecting results — precision, not judgment. Reserve opus for school and corral; haiku conducts.

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

Abstract beats in gait definitions must specify:
- **Inputs**: What the beat receives (prior outputs, docket content, codebase state)
- **Outputs**: What the beat produces (file modifications, review commentary)
- **Resources**: File scopes, ₿ blaze references
- **Goals**: What school should discern from the docket for this beat

Additionally, well-formed beats may include:
- **Auto-tool gaits**: Builds, test runs, linters — automatic tool invocations as beat steps
- **Quality requirements**: Retry semantics ("try and fix up to N times") or fail-fast ("stop on any failure")

**Gait record fields (TBD)**: At minimum needs silks (shared across versions), beat templates with the above structure, default model preferences, confidence gates, and required ₿ blazes. Exact schema deferred until first gaits are built. Gaits are immutable — evolution mints new quirts with the same silks.

## Schema Decisions

Accumulated across groom sessions.

### Core V4 Type Changes
- **Tack eliminated**: Flat mutable fields on pace (state, silks, docket, basis, ts, chain). Chain is inter-pace dependency set at slate/reslate time — structural metadata, not discovered by school. Longe depends on chains being pre-set to assess pace readiness.
- **Field renames**: V3 `text` → `docket`. V3 `direction` → no longer a pace-level field; warrants live on beat rows.
- **New state enum**: green → ready/reined → candidate → complete/abandoned. Reined = interactive-required (school decides ready vs reined).
- **Bridling and markers eliminated**: School/breeze/corral replaces arm-and-fly. Restore markers on need.

### V4 Execution Infrastructure (cchat-20260317)
- **Caracole → Martingale**: Volte identity renamed to avoid C-collision with Coronet. Symbol TBD.
- **Kimberwick**: Beat instance identity. 6 chars (4 martingale + 2 index). 4,096 slots per martingale. Symbol TBD.
- **Beat table in gallops**: Flat collection of beat rows with immutable coronet refs, replacing monolithic warrant JSON. Warrants in gallops (not git) for lane isolation, natural state store, granular status tracking.
- **Chukker model**: Integer concurrency layers replace arbitrary DAG `depends` lists.
- **Branch-per-beat JIT**: Branches created at dispatch time. Mechanical merge at chukker boundaries. Naming: `jj/MARTINGALE_ID/KIMBERWICK_ID`.
- **Beat status**: pending/complete/failed. Three states, idempotent re-dispatch.
- **next_martingale seed**: Heat-level field for martingale allocation.

### Gallops Registries
- **Gaits registry**: Top-level gallops key, keyed by quirt. Fields TBD.
- **Tackles as resource bundles**: Top-level gallops key. Silks-identified, mutable. Contains blaze definitions, actions, gait affinities.
- **Blaze identity (₿)**: Not a global identity — scoped to tackle, resolved by school.

## Still Open

- **Martingale Unicode symbol**: Needs selection. Must be visually distinct.
- **Kimberwick Unicode symbol**: Needs selection. Must be visually distinct.
- **Gait data model fields**: Beat templates, confidence gates, model preferences, auto-tool gaits, quality requirements — exact schema deferred.
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

## Gait Design Principles

Distilled from cchat-20260302 (₢AiAAz retrospective), updated cchat-20260317.

A gait contains: **beat templates** (with inputs/outputs/resources/goals, organized by chukker), **codebase investigation steps** (school reads files to understand scope before creating beat entries), and **confidence gates** (conditions that cause school to stop and flag). Confidence gates are the most valuable part — they encode "where this kind of work goes wrong."

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

- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — V3 data model (what V4 replaces)
- cchat-20260224 — Schema decisions, slash command reduction
- cchat-20260301/b — Beats, voltes, corral, execution model, quirt identity, warrant structure, gait library
- cchat-20260302 — Gait working concept: school checklists, confidence gates, review phases
- cchat-20260304 — Backwards compatibility, three-heat constellation, migration discipline
- cchat-20260306 — Longe concept, school scope, assessment/planning split
- cchat-20260311/b — Corral refinement, tackle/blaze surfacing, beat merge, parallel subtrees
- cchat-20260317 — Martingale, kimberwick, chukker, warrants in gallops, beat table, branch-per-beat, lane isolation, well-formed gait beats
- ₢AhAAF/₢AhAAG — Verb restructure and slash command cleanup