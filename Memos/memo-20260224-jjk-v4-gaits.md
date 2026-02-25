# JJK V4 Design: Gaits, Branches, and the Breeze Pipeline

Successor to `memo-20260222-jjk-v4-vision.md`. Refines the V4 vision based on design
conversation of 2026-02-24. Opus drove the session.

## Origin Chat

**cchat-20260224-gaits-and-breezes** — Single-session design conversation (2026-02-24).
Opus throughout. Workshopped gaits, the three-phase pipeline, and verb vocabulary.
Built on the original V4 vision seeds, making concrete decisions where the prior
conversation left open questions.

## Relationship to Prior Memo

The original memo (`memo-20260222-jjk-v4-vision.md`) captured design seeds from
cchat-20260222-gallops-at-dawn. This memo records decisions that supersede, refine,
or retire elements from that conversation.

## Decisions That Supersede the Original Memo

### Leg Layer: Retired Before Implementation

The leg (heat > leg > pace) added a full layer of identity, lifecycle, and state
management to solve "paces are too small for narrative continuity." But that's a
presentation problem, not a data model problem. Compound chain analysis (see Breeze
below) provides the same benefit — knowing which paces form a sequence — without
a new layer. The heat > pace model stays flat.

### Worktrees: Replaced by Branches

The original memo proposed per-leg and per-pace worktrees. Problems:
- Filesystem churn from worktree creation/destruction
- Gallops coordination across worktrees was an unsolved synchronization headache
- "Merge as a pace within the leg" was a sub-problem unto itself

**Replacement**: Git branches for candidate work. Branches are cheap, local, reviewable
(`git diff main..branch`), and merge is a single command. Main stays clean. Gallops
stays on main — no split-brain problem.

Branch naming convention: `jj/{firemark}/{coronet}` — e.g., `jj/Ak/AkAAB`.
One branch per pace. Compound chains use successive branches rooted on the
previous pace's branch (see Compound Chains below).

### Chalk: Confirmed Retired

Per the original memo. Chalk (APPROACH markers) didn't pay off. Dropped in V4.

### Bridled State: Eliminated

The `bridled` pace state conflated specification clarity, warrant authorship, autonomous
authorization, and proactive eligibility. V4 replaces it with the `ready` state plus
gaits plus warrants-as-LLM-conversation (see Pace State Machine and Warrant below).

### Quarter Command: Eliminated

Quarter assessed bridleability. With the bridled state gone, quarter is superseded
by the school command which handles readiness promotion.

### Proactive Agents / No Daemon: Deferred

The original memo's proactive agent scanning is conceptually valid but depends on
the branch/breeze infrastructure being solid first. Not in initial V4 scope.

## Gaits: Composable Pace Templates

**Core concept**: A gait is a reusable template that defines how a type of work is
executed and verified. Gaits live in the gallops registry (top-level, shared across
heats) and are assigned to paces at creation time.

### Gait Fields

```json
{
  "gaits": {
    "rust-vok": {
      "description": "Rust code changes in VOK",
      "guides": [
        "Tools/vok/vov_veiled/RCG-RustCodingGuide.md"
      ],
      "build": ["tt/vow-b.Build.sh"],
      "check": ["tt/vow-t.Test.sh"],
      "model": null,
      "cardinality": "single"
    },
    "bash-buk": {
      "description": "Bash utility kit changes",
      "guides": [
        "Tools/buk/vov_veiled/BCG-BashConsoleGuide.md",
        "Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc"
      ],
      "build": [],
      "check": [],
      "model": null,
      "cardinality": "single"
    },
    "spec-adoc": {
      "description": "AsciiDoc concept model documents",
      "guides": [
        "Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc"
      ],
      "build": [],
      "check": [],
      "model": null,
      "cardinality": "single"
    },
    "design": {
      "description": "Architectural decisions requiring deep thinking",
      "guides": [],
      "build": [],
      "check": [],
      "model": "opus",
      "cardinality": "single"
    },
    "scout": {
      "description": "Exploratory investigation, file discovery",
      "guides": [],
      "build": [],
      "check": [],
      "model": "haiku",
      "cardinality": "single"
    }
  }
}
```

### Gait Fields Explained

- **description**: Human-readable purpose
- **guides**: File paths the agent should read before executing. This is JJ-as-context-assembler — the gait carries the coding standards, not the human's memory. Referenced (agent reads them), not loaded in full.
- **build**: Commands to verify structural correctness (compiles?). Run by breeze after execution and by prance before checks.
- **check**: Commands to verify functional correctness (tests pass?). Run by prance after merge.
- **model**: Preferred LLM model (`null` = driver decides, `"haiku"`, `"sonnet"`, `"opus"`). When gaits compose, take the max.
- **cardinality**: `"single"` (one agent) or `"parallel"` (fan out). When gaits compose, single unless all agree on parallel.

### Composition

A pace carries a list of gait keys. The system unions fields:
- guides: concatenate (deduped)
- build: concatenate (deduped)
- check: concatenate (deduped)
- model: take the maximum tier
- cardinality: single unless all agree on parallel

Example: a pace gaited `["rust-vok", "spec-adoc"]` gets RCG + MCM as guides,
cargo build, and cargo test + cma-validate. An `["rust-vok", "design"]` pace
gets opus model with Rust build/test.

### Management

Gaits are managed via `jjx_` commands (CRUD), never hand-edited. Silks-keyed
(human-readable keys like `"rust-vok"`). Expected to be stable — a project
defines its gaits early and rarely changes them.

### Rendering

Available gaits are rendered during mount and groom context, so that when paces
are slated, the LLM sees the menu and can assign appropriately. Gait assignment
at pace creation; cheap to switch via a dedicated command.

## Pace State Machine

### States

| State | Meaning |
|-------|---------|
| rough | Needs clarification before work can proceed |
| ready | Docket clear, gait assigned, warrant written, eligible for breeze |
| candidate | Branch exists with attempted work, pending human review |
| complete | Approved and merged |
| abandoned | Stopped without completion |

### Transitions

```
rough ──(school)──> ready ──(breeze)──> candidate ──(prance)──> complete
  ^                   |                    |
  |                   |                    └──(prance reject)──> ready
  |                   └──(manual)──> complete                     (re-breezable)
  └──(groom)── [new pace arrives rough]
```

### State Predicates (updated)

| Predicate | rough | ready | candidate | complete | abandoned |
|-----------|-------|-------|-----------|----------|-----------|
| defined   | true  | true  | true      | true     | false     |
| resolved  | false | false | false     | true     | true      |

## Warrant: Model-to-Model Communication

Warrant survives from V3 but changes nature entirely.

| Aspect | V3 | V4 |
|--------|----|----|
| Written by | Human (via bridle ceremony) | LLM (via school) |
| Consumed by | LLM (via mount) | LLM (via breeze) |
| Audience | Model executing the pace | Model executing the pace |
| Voice | Structured execution guidance | One model briefing another |
| Required when | state = bridled | state = ready |

### Warrant as LLM Conversation

Warrants are written by models for other models. They should read as a colleague
briefing another colleague:

> "The key files are `rbgi_IAM.sh` and `rbgm_ManualProcedures.sh`. Split the SA
> functions into a new file following the pattern in `rbga_ArtifactRegistry.sh`.
> Watch out for the shared `RBGI_PROJECT` variable — both files will need it.
> Don't touch the OAuth flow; it's mid-refactor in another heat."

The human will not read warrants routinely. Their value is:
1. **Model tier bridge**: opus writes the plan during school, sonnet follows it during breeze
2. **Forensic trail**: when a candidate is wrong, trace whether the warrant was bad (school's fault) or execution was bad (breeze's fault)

Don't over-invest in warrant structure. Useful, not precious.

## Verb Set

### Updated Upper API

| Verb | What it does | State transitions |
|------|-------------|-------------------|
| **groom** | Structural editing: add/reorder/drop paces, assign gaits | Indirect (new paces arrive rough) |
| **mount** | Focus on heat, present status and options intelligently | None — orientation only |
| **school** | Refine rough paces via smart-model Q&A, generate warrants, promote to ready. Also: rebase stale candidate branches on stale heats. | rough → ready |
| **breeze** | Autonomous execution of ready paces into candidate branches | ready → candidate |
| **prance** | Merge candidates to main, run build+check from gait, present for human approval/rejection | candidate → complete (or back to ready) |
| **notch** | JJ-aware git commit (unchanged) | None (records work) |
| **wrap** | Mark pace complete manually (escape hatch for non-pipeline work) | → complete |

### Verb Comparison with V3

| V3 verb | V4 fate |
|---------|---------|
| mount | Redefined: orientation only, not execution |
| groom | Narrowed: structural editing only |
| quarter | Eliminated (superseded by school) |
| bridle | Eliminated (superseded by school + ready state) |
| slate | Unchanged (creates rough paces) |
| reslate | Unchanged (revises dockets) |
| notch | Unchanged |
| wrap | Unchanged (escape hatch) |
| — | **New**: school, breeze, prance |

### Three-Phase Pipeline

The design optimizes for human attention as the scarce resource:

| Phase | Verb | Human attention | What happens |
|-------|------|-----------------|-------------|
| Prepare | school | Medium (answering Q&A) | Rough paces refined, warrants generated, promoted to ready |
| Execute | breeze | Low (watching or away) | Ready paces executed autonomously on branches |
| Review | prance | Focused judgment | Candidates merged, tested, approved or rejected |

These phases are orthogonal — different officia can run them concurrently on
different paces without conflict:
- Officium A: grooming/slating new rough paces
- Officium B: schooling rough paces into ready
- Officium C: breezing ready paces into candidates
- Officium D: prancing candidates through review

Each touches different pace states and different git objects.

## Compound Chains

### Detection

At breeze start, the command analyzes remaining ready-pace dockets and uses LLM
judgment to classify dependency: which paces are standalone (independent, branch
from main) vs compound (must build on a prior pace's output).

### Branch Structure

Each pace gets its own branch. Compound chains express dependency through git
parentage:

```
main
 └─ jj/Ak/AkAAB  (pace A's work)
      └─ jj/Ak/AkAAC  (pace B's work, includes A)
           └─ jj/Ak/AkAAD  (pace C's work, includes A+B)
```

No multi-coronet branch names. Dependencies live in git's commit graph.

### Prance Ordering

Prance enforces order for compound chains — merge A before B before C. Once A is
merged to main, B's merge is a clean fast-forward of just B's additional commits.
If A is rejected, prance flags B and C as dependent on a rejected pace.

## Retained from Original Memo

### Gallops Stays Global

Unchanged. Gallops is coordination state, never forked into branches. JJX always
reads/writes `.claude/jjm/jjg_gallops.json` on main. Branch work only touches
code/spec files.

### Tiered Model Dispatch

Realized through gaits rather than leg-level metadata. The gait's `model` field
carries the tier hint. Composition takes the max.

### JJ as Context Assembler

Realized through gait `guides` field. Each gait specifies exactly which coding
standards, specs, and references the executing agent needs. Tight precise contexts
let lighter models punch above their weight.

### Meta-Observations

All observations from the original memo remain valid:
- Model handoff evidence (Sonnet→Opus undetected)
- Convergence is real (long conversations accumulate shared understanding)
- Collaboration structure as first-class engineering problem
- Kindness as discipline

## Open Items for Implementation Discussion

### Decided — Captured from cchat-20260224 (groom session)

**#1 Tack elimination / Pace schema flattening:**
The tack append-only history pattern is retired. After-action reports from tack
history never proved useful, and the duplication bloated gallops. V4 paces carry
mutable fields directly:
- `state` — mutable in place
- `silks` — mutable in place
- `gaits` — list of gait keys (composable)
- `docket` — plan text (renamed from V3's `text`)
- `warrant` — execution guidance (renamed from V3's `direction`). Present when ready+.
- `chain` — optional coronet of upstream dependency. Written by school, cleared on prance accept/reject.

History lives in git commits of gallops.json. No tack array.

**Branch names: derived, not stored.** Convention `jj/{firemark}/{coronet}` is
computed from pace identity. Pace state determines whether the branch is valid.
No `branch` field in the pace record.

**#4 Dependency analysis moved to school, not breeze.** School (opus-tier,
human-in-loop) sets `chain` links when promoting paces. Breeze is mechanical:
skip non-ready, hold if chain points to non-complete, run otherwise. School is
the dependency planner, not just a docket refiner.

**State enum — zero actionable overlap with V3:**
- V3 actionable: `rough`, `bridled`
- V4 actionable: `green` (unrefined, replaces rough), `ready` (autonomous-eligible),
  `reined` (interactive-required, human must be in the loop), `candidate` (branch exists)
- V4 terminal: `complete`, `abandoned` (overlap with V3 fine — inert states)

New state: `reined` — school promotes green paces to either `ready` (breeze can
handle it) or `reined` (human judgment required, mount only). Reined paces
naturally block downstream chain dependents without special logic.

**Gaits registry:** New top-level key in gallops. Silks-keyed (human-readable
keys like `"rust-vok"`). Slow-moving, simple IDs sufficient.

**Markers/steeplechase:** Aggressively eliminated for V4. Can restore on need.
Gallops/paddock updates still produce main-branch commits.

### Remaining — Decide Now

These still need resolution before slating paces on ₣Ah:

2. **School mechanics**: How does the Q&A refinement work? Batch all questions
   then promote? One-pace-at-a-time? How does it decide a docket is "clear enough"?
   Now also: how does school determine chain dependencies and ready-vs-reined
   promotion? The centerpiece command.

3. **Candidate rejection flow**: When prance rejects a candidate, does it go back
   to ready (same docket, re-breezable) or to green (needs rethinking)? Probably
   the human decides per-rejection. What's the UI?

5. **Prance merge conflict handling**: When a candidate branch conflicts with
   current main, what does prance do? Present the conflict for human resolution?
   Attempt automatic rebase? Flag for re-breeze?

### Work Out Later — Mechanical (fall out of the above decisions)

6. **Gait CRUD commands**: Minting exercise once the schema is decided.

7. **Mount rendering**: Presentation; follows from state machine decisions.

8. **Gait switching command**: Tiny UI call.

9. **Orient changes**: Mechanical once states are locked.

10. **Migration path from V3**: Blocked by schema decisions (now mostly resolved).
    Key note: V3 `text` → V4 `docket`, V3 `direction` → V4 `warrant`,
    tack array → flat fields, `rough` → `green`, `bridled` → eliminated.
