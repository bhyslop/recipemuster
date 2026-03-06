# Paddock: jjk-v4-vision

## Current Design

**Authoritative design seeds**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline. Partially superseded by cchat-20260301 sessions (corral replaces prance, warrant evolves from prose to structured beat map, volte naming, quirt identity).

Key decisions: no leg layer, worktrees for isolation (branches persist after worktree disposal), composable gaits, four-phase pipeline (longe → school → breeze → corral), warrant-as-beat-map, jjx as LLM orchestrator. Git-as-mutex proven under concurrent pressure; V4 extends git protection to work product (branches) and process artifacts (warrants in commits).

## Identity System

| Symbol | Name | Digits | Structure | Slots |
|--------|------|--------|-----------|-------|
| `₣` | Firemark | 2 | 2 heat | 4,096 heats |
| `₡` | Caracole | 4 | 2 heat + 2 index | 4,096/heat |
| `₢` | Coronet | 5 | 2 heat + 3 index | ~262K/heat |
| `Ꝗ` | Quirt | 3 | 3 global | 262,144 gaits |

Length alone disambiguates: 2=firemark, 3=quirt, 4=caracole, 5=coronet.

Quirt `ꝖABC` identifies an immutable gait snapshot. When a gait evolves, a new quirt is minted with the same silks. "Latest version" = highest quirt with matching silks. No lineage fields needed.

Beats do not have global identities. They are positional within a warrant (local IDs like `1a`, `1b`, `2`). Their output is commits on branches — the commits are the identity.

## Vocabulary

| Term | Definition |
|------|------------|
| **Beat** | Atomic unit: single dispatch, autonomous execution, no human interaction. The model may use tools freely (read files, search, etc.) but there is no multi-turn dialogue. The indivisible footfall within a gait. |
| **Gait** | Reusable recipe of beats — single, sequential, parallel, or a DAG. Gaits can compose other gaits (practical depth limit: ~2 levels). Stored as data in gallops with quirt identity. School-time resource: school reads gaits for inspiration and structure, but warrants are fully resolved with no gait references to look up at runtime. |
| **Warrant** | School's output: a fully resolved JSON beat map. Contains concrete prompts, model assignments, file scopes, and dependency DAG. No indirections — breeze/jjx need not look up gait definitions. Each beat cites its source quirt for audit. Stored as the first commit on the caracole branch (self-documenting). |
| **Volte** | An attempt at executing one or more paces. One active volte per heat at a time. School creates it; breeze executes it; corral reviews per-pace (accept/reject). State derived from git branch existence — no gallops bookkeeping beyond `next_caracole` seed. Branch namespace: `jj/₡AhAA/₢AhAAB`. Dressage term: a precise, controlled circle back to the same point with refined intent. |
| **Caracole** | Volte identity. `₡` + 4 base64 characters (2 heat + 2 index). Named for a cavalry half-turn to circle back for another pass. |
| **Quirt** | Gait identity. `Ꝗ` + 3 base64 characters. Named after a short riding whip — the thing that sets a gait in motion. |
| **Longe** | Heat-level readiness assessment. Evaluates all remaining paces in parallel: reads dockets, reads codebase to understand scope and file types, classifies each as breezable / needs-refinement / blocked. Read-only — produces a readiness report, not warrants. Guides where to focus groom/reslate effort before schooling. Named for working a horse on a long line to assess soundness before riding. |
| **School** | Per-pace planning phase. Reads pace docket AND codebase thoroughly (grep, read files, understand structure) to produce a fully resolved warrant. Does NOT web-search or produce work artifacts. Two internal phases: (1) assess docket quality — validate assumptions against codebase reality, check confidence gates; (2) plan — decompose into beats, write concrete prompts. May refuse to warrant a pace ("this docket has gaps"). Opus-tier, high human attention, one pace at a time. |
| **Breeze** | Execution phase. jjx reads the warrant, creates worktrees, dispatches beats per the DAG. Each beat is a bare prompt issued to the specified model in its worktree. jjx manages sequencing, parallelism, and merge. Zero human attention. |
| **Corral** | Review phase. Evaluates candidates, accepts/rejects/synthesizes. Medium human attention. Can see all voltes for parallax comparison. Replaces V3 "prance." |

### Type/Instance Note

"Beat" and "gait" serve as both type (in gait library templates) and instance (in warrants and execution). Context disambiguates: gait library → types, warrant → instantiation plan, volte commits → concrete history. If ambiguity ever bites, retrofit instance-specific words; don't premint vocabulary for it.

## Execution Model

### Four Phases

1. **Longe** (parallel, read-only) — assesses all remaining paces in a heat simultaneously. For each pace: reads docket, reads codebase to understand scope, identifies file types and potential gaits, classifies as breezable / needs-refinement / blocked. Output is a heat-level readiness report. Guides where to focus groom/reslate before committing to school. A school parameter controls how many paces to warrant.
2. **School** (opus, per-pace, conversational) — reads pace docket and codebase thoroughly (file reads, grep, structural understanding). Two internal phases: (a) assess docket quality against codebase reality, evaluate confidence gates; (b) produce fully resolved warrant JSON. May refuse to warrant ("this docket has gaps — here's what's wrong"). Does NOT web-search or produce work artifacts. Human Q&A refines the warrant. One pace at a time — most docket writing is interactive and focused.
3. **Breeze** (jjx-orchestrated) — jjx reads warrant from the volte branch, creates worktrees per beat, dispatches bare prompts to specified models per the dependency DAG. Parallel beats run concurrently. jjx collects results and merges beat branches into per-pace candidate branches. Haiku can dispatch — just following instructions, no judgment needed. Practical constraint: concurrent bare-prompt dispatches share a single account's RPM and token-per-minute throttle. Staggering beat launches by 1-2 seconds and right-sizing models per beat mitigates throughput contention. Different models have independent rate limit pools — a warrant mixing haiku/sonnet/opus beats gets more effective throughput than one using a single model.
4. **Corral** (human + LLM) — reviews per-pace candidates within the volte. Accept or reject each pace individually. When all paces reviewed, volte is done (branches are the archive). Rejected paces return to pool for future longe/groom/school cycle.

### Warrant Structure

The warrant is JSON, consumed by jjx for orchestration. Fully resolved — no gait references to dereference at runtime. Each beat cites its source quirt for process improvement audit.

```json
{
  "beats": [
    {
      "id": "1a",
      "quirt": "ꝖABC",
      "model": "sonnet",
      "depends": [],
      "files": ["lenses/RBSDC-depot_create.adoc"],
      "prompt": "You are writing AsciiDoc specifications..."
    },
    {
      "id": "1b",
      "quirt": "ꝖDEF",
      "model": "sonnet",
      "depends": [],
      "files": ["Tools/rbw/rbga_ArtifactRegistry.sh"],
      "prompt": "You are implementing bash functions..."
    },
    {
      "id": "2",
      "quirt": "ꝖGHI",
      "model": "haiku",
      "depends": ["1a", "1b"],
      "files": [],
      "prompt": "Review the following changes for correctness..."
    }
  ]
}
```

- `depends` defines the DAG. Empty = can run immediately. jjx dispatches in topological order, parallelizes where the DAG allows.
- `files` is write scope — files this beat is expected to modify. Empty = read-only beat (reviewer, produces commentary not code).
- `prompt` is the fully resolved text. The LLM reads additional files as needed via tool calls during execution.
- `quirt` cites which gait template this beat was expanded from. Audit/process-improvement metadata.

Bare prompts are reproducible. Same prompt + same model + same worktree state = comparable output. Multi-turn conversations are path-dependent and unreproducible; a bare prompt is a function call. This makes process improvement scientific: when a beat produces bad output, change one variable (the prompt, or the model) and rerun. The closed loop — prompt → output → evaluate → refine prompt — is what makes the gait library a learning system. Each quirt version encodes lessons from prior bare-prompt outcomes. Tool use during beat execution is expected (file reads, searches, etc.) but opaque to the orchestrator — jjx dispatches one prompt and collects commits. The internal tool-call chain is the model's problem, not the conductor's.

### Warrant Storage

The warrant is stored as the **first commit** on the volte branch (`jj/₡AhAA`) containing the full warrant JSON in the commit message. Per-pace branches (`jj/₡AhAA/₢AhAAB`) fork from there. The branch tells its own story: first commit is the plan, subsequent commits are the execution.

### Longe → School Handoff

Longe assesses the whole heat at once. School warrants one pace at a time. The cycle:

1. Longe reports: "A breezable, B breezable, C needs refinement, D blocked on C."
2. Human grooms C (reslate to clarify docket). Re-longes if desired.
3. School warrants A: reads codebase, validates docket, produces warrant. Human approves.
4. School warrants B: same process. Human approves.
5. Breeze executes A and B.
6. After corral, longe reassesses — C's fog may have lifted.

School's primary discipline is **refusing to plan on weak foundations**. Its most valuable output may be "I cannot warrant this docket — here's what's wrong." A well-groomed docket should be plannable; when it isn't, that signals docket gaps, not a problem for school to paper over.

### Voltes, Worktrees, and Parallax

A volte covers whatever paces school and the human agreed to execute in this pass. School decomposes multi-concern dockets into parallel gaits (e.g., spec-writer + bash-coder + reviewer as a DAG). One active volte per heat — no concurrent voltes in V4.

Worktrees provide isolation (cheap, disposable — branch is the permanent artifact). Future versions may enable parallax via multiple voltes: cross-model comparison, prompt engineering comparison, quality annealing. Deferred — V4 is sequential.

Every beat stores its prompt/warrant in the git commit — enabling replay, audit, semantic merge intelligence, and process improvement traced to specific quirt versions.

### jjx as LLM Orchestrator

V4 jjx doesn't just manage JSON — it emits prompts, directives, and context that shape what the LLM does next. Slash commands establish the interpretation contract (LLM treats jjx output as imperative, not advisory). jjx output design becomes a first-class concern: quality of directive text matters as much as correctness of JSON mutations.

Pattern: jjx handles state/sequencing, LLM handles git operations and judgment. Co-routine. jjx is the conductor — it never interprets the prompts, just routes them.

jjx orchestration is haiku-tier work. The JJS0 lower/upper API split already implies this: lower-layer commands are deliberately boring and mechanical. Reading warrants, creating worktrees, dispatching beats per the DAG, collecting results — this requires precision, not judgment. Reserve opus for school and corral; haiku conducts.

**Model-tier enforcement**: jjx requires a model-identity parameter on every invocation. The invoking model self-reports its identity (e.g., `claude-haiku-4-5`). jjx enforces tier constraints mechanically — breeze-phase orchestration commands refuse to execute unless invoked by haiku. No honor system; hard gate. Secondary benefit: commit history and execution logs carry a model-attribution trail, enabling process improvement queries ("which model tier produced better results on mechanical beats?").

### Attention Model

V4 is designed around human attention as the bottleneck:
- **Longe**: low attention — read report, decide where to focus groom effort
- **School**: high attention — shaping warrants, making stop/go judgments on ambiguous paces
- **Breeze**: zero attention — pure execution, go do something else
- **Corral**: medium attention — reviewing diffs per pace, not co-piloting

The system optimizes for the serial path being frictionless, not for raw parallelism.

## Gait Library

Gaits live in gallops as data, identified by quirt (`ꝖABC`). They are a **school-time resource** — school reads them for template patterns and prompt structures, but warrants are fully resolved. Breeze never looks up a gait.

The gait library is like a playbook a coach consults before writing the game plan. The game plan doesn't say "run play #47" — it says exactly what each player does.

Gaits evolve through practice: start with a few simple single-beat gaits, use them, see what works, crystallize recurring compositions as named gaits. The library grows from practice, not from design.

**Gait record fields (TBD)**: at minimum needs silks (shared across versions), prompt template content, default model preference, and whatever structure school needs to compose them. Exact schema deferred until first gaits are built. Gaits are immutable — evolution mints new quirts with the same silks.

## Schema Decisions (cchat-20260224 groom session, updated cchat-20260301)

- **Tack eliminated**: Flat mutable fields on pace (state, silks, gaits, docket, warrant, chain). No append-only history.
- **Branch names derived**: `jj/₡AhAA` (volte), `jj/₡AhAA/₢AhAAB` (per-pace). Volte active iff branch exists.
- **Dependencies via school**: `chain` field (optional coronet) set by school, not breeze. Breeze is mechanical.
- **New state enum**: green → ready/reined → candidate → complete/abandoned. Zero actionable overlap with V3.
- **Reined state**: Interactive-required. School decides ready (autonomous) vs reined (human-in-loop).
- **Gaits registry**: New top-level gallops key, keyed by quirt. Stored as data. Fields TBD.
- **Bridling eliminated**: School/breeze/corral replaces arm-and-fly. No bridled state in V4.
- **Markers eliminated**: Restore on need.
- **Field renames**: V3 `text` → `docket`, V3 `direction` → `warrant` (now structured JSON, not prose).

## Still Open

- **Gait data model fields**: What does a gait record contain? Beat shape (DAG pattern), confidence gates, default model preferences — exact schema deferred. Key insight: gaits are checklists for school (decomposition guidance + confidence gates), not plan templates for breeze.
- **Beat merge ceremony**: Merging beat branches into candidate branch after breeze — mechanical (automatic) or does it need a final beat in the warrant?
- **Memory curation**: How does jjx curate prior-volte context for school prompts? jjx is the memory — reads volte history, rejection notes, prior warrants. The curation logic (what to include, how much) is the hardest design problem. Deferred — likely ₣Am scope.
- **Longe output format**: What does the readiness report look like? Per-pace: breezable/needs-refinement/blocked + rationale + identified file types + candidate gaits.
- **Longe verb and CLI**: New upper API verb `longe`. Maps to what jjx operation? Needs spec.
- **Ready vs reined distinction**: Both in the state enum — is the distinction clear enough? School decides which.

## Resolved

- **School scope** (cchat-20260306): School reads codebase thoroughly (grep, file reads) to understand scope and validate dockets — this is planning intelligence, not "research." School does NOT web-search or produce work artifacts. School has two internal phases: assess (validate docket against codebase) and plan (produce warrant).
- **Assessment vs planning split** (cchat-20260306): New "longe" phase handles heat-level readiness assessment (parallel, all paces). School handles per-pace warrant production (sequential, interactive). This separates "which paces are ready?" from "plan this specific pace."
- **Groom's fate**: Groom remains a verb table entry. Longe → groom → school is the refinement cycle.
- **Volte identity** (cchat-20260306): Voltes are identified by caracole (`₡` + 4 base64, 2 heat + 2 index). Length-disambiguated from other identities.
- **No concurrent voltes** (cchat-20260306): One active volte per heat. Sequential: school → breeze → corral → done. Parallax via concurrent voltes deferred.
- **Volte state from git** (cchat-20260306): No volte state in gallops. Branch existence = active. Heat stores only `next_caracole` seed. Corral reviews per-pace (accept/reject individually); volte done when all paces reviewed.
- **Volte history synthesis deferred** (cchat-20260306): When a volte's pace is rejected, human grooms/reslates the docket with lessons learned. School starts fresh from the docket — the docket IS the memory. Machine-curated volte history is V4.1+ scope.
- **Quirt immutability** (cchat-20260306): Gaits are immutable snapshots. When a gait evolves, mint a new quirt with the same silks. Multiple quirts share silks; "latest version" = highest quirt with matching silks. No `supersedes` field or lineage chain — flat scan over the gaits registry (fast for hundreds). Beats cite specific quirts for audit; school picks the latest. Same pattern as immutable container images with mutable tags.

## Gait Design Principles (distilled from cchat-20260302, ₢AiAAz retrospective)

A gait contains three things: **beat shape** (DAG pattern), **codebase investigation steps** (school reads files to understand scope before decomposing beats), and **confidence gates** (conditions that cause school to stop and flag). Confidence gates are the most valuable part — they encode "where this kind of work goes wrong."

Key decisions:
- Gaits are checklists for school, not plan templates for breeze. School produces the plan; the gait guides decomposition.
- Gait selection belongs in school, not at slate time. Dockets evolve; stale gait selections add coupling. Humans may hint in docket prose.
- Review is multi-phase: self-review (parallel angles) → fix → human review (corral) → fix. Not a single terminal beat.
- School's primary discipline is refusing to plan on weak foundations. The school slash command is lightweight: jjx emits docket + gait library, LLM follows the gait's recipe or stops with a specific concern.

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

This discipline propagates to any continuation heats spawned from ₣Ah.

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

- `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline (partially superseded)
- `Memos/memo-20260222-jjk-v4-vision.md` — Superseded original design seeds
- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — V3 data model (what V4 replaces)
- cchat-20260224 — Groom session: schema decisions, slash command reduction
- cchat-20260301 — Dreaming session: beats, voltes, corral, execution model, attention model
- cchat-20260301b — Continuation: quirt identity, warrant structure (JSON), warrant evolution from V3, school incrementalism, memory challenge, gait library as school-time resource
- cchat-20260302 — Gait working concept: gaits as school checklists, confidence gates, research steps, review phases, MCM vocabulary migration example (derived from ₣Ai ₢AiAAz execution)
- cchat-20260304 — Groom session: backwards compatibility strategy, ₣AG triage, three-heat constellation, migration discipline
- cchat-20260306 — Longe concept, school scope clarification (reads codebase, not web search), assessment/planning split, paddock compression
- ₢AhAAF/₢AhAAG — Verb restructure and slash command cleanup (completed, outcomes in CLAUDE.md)
