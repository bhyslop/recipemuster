# Paddock: jjk-v4-vision

## Current Design

**Authoritative design seeds**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline. Partially superseded by cchat-20260301 sessions (corral replaces prance, warrant evolves from prose to structured beat map, volte naming, quirt identity).

Key decisions: no leg layer, worktrees for isolation (branches persist after worktree disposal), composable gaits, three-phase pipeline (school → breeze → corral), warrant-as-beat-map, jjx as LLM orchestrator.

## Identity System

| Symbol | Name | Digits | Namespace | Slots |
|--------|------|--------|-----------|-------|
| `₣` | Firemark | 2 | Heats | 4,096 |
| `₢` | Coronet | 5 | Paces | ~268M |
| `Ꝗ` | Quirt | 3 | Gaits | 262,144 |

Quirt `ꝖABC` identifies a gait. If the gait evolves, it gets a new quirt — version history lives in gallops and git, not in the identifier.

Beats do not have global identities. They are positional within a warrant (local IDs like `1a`, `1b`, `2`). Their output is commits on branches — the commits are the identity.

## Vocabulary

| Term | Definition |
|------|------------|
| **Beat** | Atomic unit: single dispatch, autonomous execution, no human interaction. The model may use tools freely (read files, search, etc.) but there is no multi-turn dialogue. The indivisible footfall within a gait. |
| **Gait** | Reusable recipe of beats — single, sequential, parallel, or a DAG. Gaits can compose other gaits (practical depth limit: ~2 levels). Stored as data in gallops with quirt identity. School-time resource: school reads gaits for inspiration and structure, but warrants are fully resolved with no gait references to look up at runtime. |
| **Warrant** | School's output: a fully resolved JSON beat map. Contains concrete prompts, model assignments, file scopes, and dependency DAG. No indirections — breeze/jjx need not look up gait definitions. Each beat cites its source quirt for audit. Stored as an empty first commit on the volte branch (self-documenting). |
| **Volte** | An attempt at executing a pace (or batch of paces). Branch namespace: `jj/{firemark}/volte-N/{coronet}`. Multiple voltes enable parallax — different approaches to the same problem, compared at corral. Dressage term: a precise, controlled circle back to the same point with refined intent. |
| **Quirt** | Gait identity. `Ꝗ` + 3 base64 characters. Named after a short riding whip — the thing that sets a gait in motion. |
| **School** | Planning phase. Reads pace docket, selects gaits from library, produces warrant. Incremental — schools forward until ambiguity is too thick, then stops. Opus-tier work. High human attention. |
| **Breeze** | Execution phase. jjx reads the warrant, creates worktrees, dispatches beats per the DAG. Each beat is a bare prompt issued to the specified model in its worktree. jjx manages sequencing, parallelism, and merge. Zero human attention. |
| **Corral** | Review phase. Evaluates candidates, accepts/rejects/synthesizes. Medium human attention. Can see all voltes for parallax comparison. Replaces V3 "prance." |

### Type/Instance Note

"Beat" and "gait" serve as both type (in gait library templates) and instance (in warrants and execution). Context disambiguates: gait library → types, warrant → instantiation plan, volte commits → concrete history. If ambiguity ever bites, retrofit instance-specific words; don't premint vocabulary for it.

## Execution Model

### Three Phases

1. **School** (opus, conversational) — reads pace docket, selects gaits, produces warrant JSON. Incremental: proposes per pace, human approves, includes ambiguity/malformation assessment to decide when to stop schooling further paces. Slash command primes LLM; jjx emits directive output that LLM interprets as instructions. Human Q&A refines the warrant.
2. **Breeze** (jjx-orchestrated) — jjx reads warrant, creates worktrees per beat, dispatches bare prompts to specified models per the dependency DAG. Parallel beats run concurrently. jjx collects results and merges beat branches into a single candidate branch for this pace in this volte. Haiku can dispatch — just following instructions, no judgment needed. Practical constraint: concurrent bare-prompt dispatches share a single account's RPM and token-per-minute throttle. Staggering beat launches by 1-2 seconds and right-sizing models per beat mitigates throughput contention. Different models have independent rate limit pools — a warrant mixing haiku/sonnet/opus beats gets more effective throughput than one using a single model.
3. **Corral** (human + LLM) — reviews candidate. Accept, reject (school produces new volte with revised warrant), or synthesize across voltes.

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

The warrant is stored as an **empty commit** (first commit on the volte branch) containing the full warrant JSON in the commit message. The branch tells its own story: first commit is the plan, subsequent commits are the execution. Anyone reading branch history sees intent followed by action.

### School is Incremental

School doesn't plan the whole heat at once. It schools forward until the fog gets too thick:

1. School presents pace A: "clear docket, here's the warrant." Human approves.
2. School presents pace B: "clear, two parallel gaits." Human approves.
3. School presents pace C: "ambiguous docket, depends on B's outcome. I'd stop here."
4. Human agrees. Breeze executes A and B.
5. After corral, school can see results and C's fog may have lifted.

School's primary value-add is the **ambiguity/malformation assessment** — not just "here's a warrant" but "here's my confidence level, and here's why we should stop."

### Multi-Gait Decomposition

A single pace docket often commingles concerns (e.g., "update the spec and implement the feature"). School's intelligence lies in decomposing this into parallel gaits — a spec-writer gait on the adoc files AND a bash-coder gait on the shell files, running concurrently, followed by a reviewer beat that reads both outputs. The warrant captures this as a DAG of beats drawn from different gaits. For well-understood changes, parallel is safe; for exploratory work, school serializes and adds a reconciliation step.

### Volte Scope

A volte is **whatever school and the human agreed to execute in this pass.** Could be one pace, could be five. The volte boundary is set by the schooling conversation, not by a fixed rule.

Volte-1 might cover paces A and B. Volte-2 covers C and D after seeing volte-1's results. Volte-3 redoes B and ripples through C-D because corral found a flaw.

### Parallax

Multiple voltes are a feature, not failure. Different prompts, different models, different gaits applied to the same pace docket. Enables:
- Cross-language triangulation (implement in Rust, compare back to bash — use Rust as a lens to find bugs/gaps)
- Model comparison (haiku vs sonnet on the same beat)
- Prompt engineering comparison (same gait, different warrant framing)
- Quality annealing through school learning from prior volte results

### Worktrees

Worktrees provide isolation. Cheap (shared .git database, milliseconds to create). Disposable — the branch is the permanent artifact, the worktree is scaffolding.

Lifecycle: create worktree → create branch → do work → commit → remove worktree → branch persists for corral.

### Prompts in Git Commits

Every beat stores its prompt/warrant context in the git commit. This enables:
- **Replay**: re-run a beat with a better prompt or different model
- **Audit**: trace flaws back to the prompt that produced them
- **Merge intelligence**: semantic merge conflict resolution — corral knows *why* each side changed, not just *what* changed. A reconciler beat can read both warrants and produce an intelligent merge.
- **Process improvement**: compare prompts that led to good vs bad outcomes, traced back to specific quirt versions

### jjx as LLM Orchestrator

V4 jjx doesn't just manage JSON — it emits prompts, directives, and context that shape what the LLM does next. Slash commands establish the interpretation contract (LLM treats jjx output as imperative, not advisory). jjx output design becomes a first-class concern: quality of directive text matters as much as correctness of JSON mutations.

Pattern: jjx handles state/sequencing, LLM handles git operations and judgment. Co-routine. jjx is the conductor — it never interprets the prompts, just routes them.

jjx orchestration is haiku-tier work. The JJS0 lower/upper API split already implies this: lower-layer commands are deliberately boring and mechanical. Reading warrants, creating worktrees, dispatching beats per the DAG, collecting results — this requires precision, not judgment. Reserve opus for school and corral; haiku conducts.

**Model-tier enforcement**: jjx requires a model-identity parameter on every invocation. The invoking model self-reports its identity (e.g., `claude-haiku-4-5`). jjx enforces tier constraints mechanically — breeze-phase orchestration commands refuse to execute unless invoked by haiku. No honor system; hard gate. Secondary benefit: commit history and execution logs carry a model-attribution trail, enabling process improvement queries ("which model tier produced better results on mechanical beats?").

### Attention Model

V4 is designed around human attention as the bottleneck:
- **School**: high attention — shaping warrants, making stop/go judgments on ambiguous paces
- **Breeze**: zero attention — pure execution, go do something else
- **Corral**: medium attention — reviewing diffs, not co-piloting. Parallax (multiple voltes) gives intuition something to triangulate against

The system optimizes for the serial path being frictionless, not for raw parallelism.

## Warrant Evolution (from V3)

V3 "warrant" (the `direction` field) was prose execution guidance for a bridled pace — manually written, hoping the LLM interprets it correctly. V4 warrant is a structured JSON beat map produced by school. The V3 meaning is a degenerate special case (single beat, prose prompt).

**Bridling is eliminated.** School/breeze/corral replaces the manual arm-and-fly pattern entirely. The `jjdo_arm` operation and `direction` field on tack are V3-only concepts.

## Gait Library

Gaits live in gallops as data, identified by quirt (`ꝖABC`). They are a **school-time resource** — school reads them for template patterns and prompt structures, but warrants are fully resolved. Breeze never looks up a gait.

The gait library is like a playbook a coach consults before writing the game plan. The game plan doesn't say "run play #47" — it says exactly what each player does.

Gaits evolve through practice: start with a few simple single-beat gaits, use them, see what works, crystallize recurring compositions as named gaits. The library grows from practice, not from design.

**Gait record fields (TBD)**: at minimum needs prompt template content, default model preference, and whatever structure school needs to compose them. Exact schema deferred until first gaits are built.

## Schema Decisions (cchat-20260224 groom session, updated cchat-20260301)

- **Tack eliminated**: Flat mutable fields on pace (state, silks, gaits, docket, warrant, chain). No append-only history.
- **Branch names derived**: `jj/{firemark}/volte-N/{coronet}`, not stored. State determines validity.
- **Dependencies via school**: `chain` field (optional coronet) set by school, not breeze. Breeze is mechanical.
- **New state enum**: green → ready/reined → candidate → complete/abandoned. Zero actionable overlap with V3.
- **Reined state**: Interactive-required. School decides ready (autonomous) vs reined (human-in-loop).
- **Gaits registry**: New top-level gallops key, keyed by quirt. Stored as data. Fields TBD.
- **Bridling eliminated**: School/breeze/corral replaces arm-and-fly. No bridled state in V4.
- **Markers eliminated**: Restore on need.
- **Field renames**: V3 `text` → `docket`, V3 `direction` → `warrant` (now structured JSON, not prose).

## Slash Command Reduction (cchat-20260224 groom session)

V4 aggressively reduces slash commands. Most become CLAUDE.md verb table entries. Only protocol-heavy verbs justify slash commands.

**Delete entirely (concept eliminated):**
- bridle, quarter, braid, garland

**Demote to CLAUDE.md verb table:**
- slate, reslate, notch, rail, furlough, restring, retire-dryrun, retire-FINAL

**Notch simplification**: Branch model eliminates file-selection complexity. All changes on a pace branch belong to that pace. Notch becomes a simple verb table entry.

**Keep as slash command:**
- mount — protocol-heavy, may keep
- school — new, protocol-heavy (jjx-directive + LLM-conversational co-routine)
- corral — new, protocol-heavy (replaces prance)

**Undecided:**
- groom — changing nature in V4, TBD

**Design principle**: Slash commands set behavioral protocols for complex multi-step interactions. Simple verb→jjx mappings go in the CLAUDE.md verb table (cheaper, no token tax).

## V3 Wins to Preserve

- **Git-as-mutex for gallops.json**: Proven under genuine concurrent pressure. V4 extends this — git protects not just metadata but work product (branches) and process artifacts (warrants in commits).

## Memory Design (serious open challenge)

Memory is context. Context is tokens. "School has memory of prior voltes" requires externalized memory — stored in git, in gallops, in structured artifacts — not assumed to live in an LLM context window. Different school sessions may be different LLM invocations.

jjx is the memory. It reads volte history, rejection notes, prior warrants, and **curates what to include** in the school prompt. The curation logic — what to include, how much, in what form — is the critical design challenge:
- Too little context: school repeats mistakes
- Too much: token bloat, confusion, cost
- Must evolve through practice

Volte branches and warrant commits are the raw material. jjx's job is to summarize and present the relevant subset.

## Still Open

- **Gait data model fields**: What does a gait record contain? Prompt template, default model, beat structure — exact schema deferred.
- **Beat merge ceremony**: Merging beat branches into candidate branch after breeze — mechanical (automatic) or does it need a final beat in the warrant?
- **Memory curation**: How does jjx decide what prior-volte context to feed into school? This is the hardest design problem.
- **Groom's fate**: Slash command or verb table entry in V4?
- **Ready vs reined distinction**: Both in the state enum — is the distinction clear enough? School decides which. (Note: bridled is eliminated, but reined may still be meaningful as "needs human interaction during execution.")

## Gait Working Concept (cchat-20260302)

Derived from executing ₢AiAAz (update-specs-cbv2-migration) on ₣Ai and reflecting on what a gait would have needed to guide that work. The pace was a 9-file AsciiDoc vocabulary migration — term renames, spec rewrites, retirements, cascading reference fixes, multi-angle review.

### Gaits are checklists for school, not plan templates for breeze

A gait does not contain a plan. It contains the *structure school follows to produce a plan*:

- **Beat shape**: the DAG pattern ("parallel file edits with a shared vocabulary table as input")
- **Research steps**: bounded investigations school runs before committing to beat breakdown ("scan for all attribute references across includes," "classify files by change type: rename, rewrite, retire, cascade")
- **Confidence gates**: conditions that should cause school to stop and flag rather than produce a warrant ("if blast radius exceeds docket's file list by >50%, flag," "if any file requires rewrite but paddock lacks explicit guidance for that rewrite, flag")

Confidence gates are the most valuable part. They encode "where this kind of work goes wrong" — distilled experience from prior executions of this gait pattern.

### Research is steps within a gait, not a separate category

Early in the conversation we considered "research gaits" as a distinct type. Discarded. Research is steps that school follows within a gait's recipe before breaking beats. The outputs are structured artifacts (file manifests, translation tables, classification lists) that downstream beats consume. These artifacts are auditable — they're commits on the volte branch.

### Gait selection belongs in school, not at slate time

Considered attaching candidate gaits at pace slating. Rejected — dockets evolve through reslate and groom. Stale gait selections add coupling without meaningful savings. The gait library is small enough that school scans it alongside the docket. Gait selection is cheap relative to research and beat-breaking.

A human *may* hint at a gait in docket prose ("this looks like a vocabulary migration"). School considers hints but isn't bound by them.

### Review is multi-phase, not one beat

The ₢AiAAz execution revealed review is richer than a single "consistency check" beat:

1. **Self-review**: parallel agents examining different angles (mapping completeness, paddock fidelity, cross-file consistency)
2. **Fix**: beats addressing self-review findings
3. **Human review**: corral checkpoint — may happen in a separate session
4. **Fix**: beats addressing human review findings

A gait's beat shape should model review as a phase with its own internal structure, not a single terminal beat.

### School's primary discipline is refusing to plan

School's most valuable output may be "I cannot produce a warrant for this docket." A well-defined docket (refined through groom) should be plannable. When it isn't, that's a signal the docket has gaps — not a problem for school to paper over.

The school slash command should be lightweight: jjx emits the docket + gait library, the LLM (sonnet or opus) follows the matched gait's recipe, commissions research as the gait prescribes, and either produces a warrant or stops with a specific concern. The human never reads the warrant — it's machine-consumed by breeze.

### Concrete example: MCM vocabulary migration

Beat shape observed in ₢AiAAz that could become a named gait:

```
Research R1 (sonnet): blast-radius-scan — grep term usage, produce affected-file manifest
Research R2 (sonnet): vocabulary-table — extract old→new term mappings from paddock decisions
Research R3 (sonnet): file-classify — classify each file as rename/rewrite/retire/cascade
Edit 2a (sonnet): backbone spec (RBS0) — renames + definition rewrites (serial, one file)
Edit 2b-2n (sonnet/haiku): operation specs — parallel per file, consuming R2+R3
Review 3a-3c (sonnet): parallel review beats with different angles
Fix 4 (sonnet/haiku): address review findings
Corral: human review checkpoint
```

Confidence gate: "if R1 discovers files not in the docket, and R3 classifies any as 'rewrite' rather than 'cascade', stop — the docket underspecified the scope."

## References

- `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline (partially superseded)
- `Memos/memo-20260222-jjk-v4-vision.md` — Superseded original design seeds
- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — V3 data model (what V4 replaces)
- cchat-20260224 — Groom session: schema decisions, slash command reduction
- cchat-20260301 — Dreaming session: beats, voltes, corral, execution model, attention model
- cchat-20260301b — Continuation: quirt identity, warrant structure (JSON), warrant evolution from V3, school incrementalism, memory challenge, gait library as school-time resource
- cchat-20260302 — Gait working concept: gaits as school checklists, confidence gates, research steps, review phases, MCM vocabulary migration example (derived from ₣Ai ₢AiAAz execution)
