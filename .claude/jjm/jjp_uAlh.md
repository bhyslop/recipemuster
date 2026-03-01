# Paddock: jjk-v4-vision

## Current Design

**Authoritative**: `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline.

Key decisions: no leg layer, worktrees for isolation (branches persist after worktree disposal), composable gaits, three-phase pipeline (school → breeze → corral), warrant-as-beat-map.

## Vocabulary (cchat-20260301 dreaming session)

| Term | Definition |
|------|------------|
| **Beat** | Atomic unit: one prompt, one response. Pure. The indivisible footfall within a gait. |
| **Gait** | Reusable recipe of beats — single, sequential, parallel, or a DAG. Gaits can compose other gaits (practical depth limit: ~2 levels). Stored as data in gallops, evolve over time. |
| **Warrant** | School's output: the beat map for a pace. Selects and composes gaits, assigns models and file scopes per beat. The warrant *is* the structured plan, not prose. |
| **Volte** | An attempt at executing a pace. Branch namespace: `jj/{firemark}/volte-N/{coronet}`. Multiple voltes enable parallax — different approaches to the same problem, compared at corral. Dressage term: a precise, controlled circle back to the same point with refined intent. |
| **School** | Planning phase. Has memory of prior voltes. Reads pace docket, selects gaits from library, produces warrant. High human attention. |
| **Breeze** | Execution phase. Pure — each beat gets a worktree, executes its prompt, produces commits on its branch. Zero human attention. |
| **Corral** | Review phase. Evaluates candidates, accepts/rejects/synthesizes. Medium human attention. Can see all voltes for parallax comparison. Replaces V3 "prance." |

### Type/Instance Note

"Beat" and "gait" serve as both type (in gait library templates) and instance (in warrants and execution). Context disambiguates: gait library → types, warrant → instantiation plan, volte commits → concrete history. If ambiguity ever bites, retrofit instance-specific words; don't premint vocabulary for it.

## Execution Model (cchat-20260301)

### Three Phases

1. **School** — reads pace docket, selects gaits, produces warrant (beat map). Slash command primes LLM; jjx emits directive output that LLM interprets as instructions. Human Q&A refines the warrant.
2. **Breeze** — executes warrant. Each beat gets its own worktree and branch. Parallel beats run concurrently. At completion, beat branches merge together into a single candidate branch for this pace in this volte.
3. **Corral** — reviews candidate. Accept, reject (school produces new volte with revised warrant), or synthesize across voltes.

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
- **Merge intelligence**: semantic merge conflict resolution (know *why* each side changed, not just *what* changed)
- **Process improvement**: compare prompts that led to good vs bad outcomes

### jjx as LLM Orchestrator

V4 jjx doesn't just manage JSON — it emits prompts, directives, and context that shape what the LLM does next. Slash commands establish the interpretation contract (LLM treats jjx output as imperative, not advisory). jjx output design becomes a first-class concern: quality of directive text matters as much as correctness of JSON mutations.

Pattern: jjx handles state/sequencing, LLM handles git operations and judgment. Co-routine.

## Schema Decisions (cchat-20260224 groom session)

- **Tack eliminated**: Flat mutable fields on pace (state, silks, gaits, docket, warrant, chain). No append-only history.
- **Branch names derived**: `jj/{firemark}/volte-N/{coronet}`, not stored. State determines validity.
- **Dependencies via school**: `chain` field (optional coronet) set by school, not breeze. Breeze is mechanical.
- **New state enum**: green → ready/reined → candidate → complete/abandoned. Zero actionable overlap with V3.
- **Reined state**: Interactive-required. School decides ready (autonomous) vs reined (human-in-loop).
- **Gaits registry**: New top-level gallops key, stored as data. Fields TBD — table named but columns not yet defined.
- **Markers eliminated**: Restore on need.
- **Field renames**: V3 `text` → `docket`, V3 `direction` → `warrant`.

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

## Still Open

- **Gait data model fields**: The gaits registry table is named but its columns/fields are undefined. What does a gait record contain?
- **Beat merge ceremony**: Merging beat branches into candidate branch after breeze — is this mechanical (automatic) or does it need a final beat in the warrant?
- **Groom's fate**: Slash command or verb table entry in V4?
- **Ready vs reined distinction**: Both in the state enum — is the distinction clear enough? School decides which.

## References

- `Memos/memo-20260224-jjk-v4-gaits.md` — Gaits, branches, and the breeze pipeline
- `Memos/memo-20260222-jjk-v4-vision.md` — Superseded original design seeds
- `Tools/jjk/vov_veiled/JJS0-GallopsData.adoc` — V3 data model (what V4 replaces)
- cchat-20260224 — Groom session: schema decisions, slash command reduction
- cchat-20260301 — Dreaming session: beats, voltes, corral, execution model
