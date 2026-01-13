# Heat: JJR Gallops Core

Implement Job Jockey Rust backend: `vvx jjx_*` subcommands for Gallops JSON operations, steeplechase history, and JJ-aware commits. Complete the JJD specification, then build the Rust implementation informed by VOK/VVK patterns. Also adds `vvx commit` core infrastructure to VOK.

## Paddock

### Why "Gallops"

The gallops are where horses train - where real work happens before race day. The Gallops JSON tracks heats (initiatives), paces (tasks), and tacks (refinements). Renamed from "Studbook" to avoid 's' collision with "specification" in our mental model.

### Architecture Recap

**`vvx commit`** — Core commit infrastructure in VOK: lock, stage, guard, claude-for-message, commit. Shared by all commit operations.

**`vvx jjx_*`** — Flat namespace for all JJK operations. No nested subcommands; underscore-delimited single tokens.

```
vvx (Rust via VOK)
  ├─ commit              # Core: lock, stage, guard, claude call, commit
  │
  └─ jjx_* (JJK feature, flat namespace)
        ├─ jjx_notch     # JJ commit with heat/pace prefix (uses commit core)
        ├─ jjx_chalk     # Steeplechase marker, empty commit (uses commit core)
        ├─ jjx_validate  # Gallops JSON validation
        ├─ jjx_nominate  # Create heat
        ├─ jjx_slate     # Add pace
        ├─ jjx_rail      # Reorder paces
        ├─ jjx_tally     # Add tack (unified: state transition + plan refinement)
        ├─ jjx_muster    # List heats
        ├─ jjx_rein      # Steeplechase history from git log
        └─ ... (other ops)
```

**Single lock:** All `vvx` operations share one lock (`refs/vvg/locks/vvx`). Rust acquires at start, releases on completion or failure.

**Slash commands become thin wrappers:** Extract context from conversation, invoke background `vvx` process, return immediately.

### Pacing Strategy

Work proceeds in logical phases. Each pace notes its phase for context.

- **Phase 0 (Vocabulary)**: Rename studbook → gallops throughout
- **Phase 1 (Foundation)**: AXLA voicing, operation template style
- **Phase 2 (Specification)**: Complete all JJD operation specs
- **Phase 3 (Rust)**: Implement `vvx jjx_*` and `vvx commit` — read VOK/VVK READMEs first
- **Phase 4 (Integration)**: Wire bash to `vvx jjx_*`, end-to-end test

### Essential References

**JJD Specification** (our target):
- `Tools/jjk/JJD-StudbookData.adoc` → rename to `JJD-GallopsData.adoc`

**Bash Prototype** (behavior reference):
- `Tools/jjk/jju_utility.sh` — current jq-based implementation

**Rust Infrastructure** (read before Phase 3 paces):
- `Tools/vok/README.md` — VOK patterns and conventions
- `Tools/vvk/README.md` — VVK infrastructure for veiled Rust crates

**AXLA Lexicon** (for voicing decisions):
- `Tools/cmk/AXLA-Lexicon.adoc`

### Key Constraints

1. **JJD is the authority** — bash prototype shows behavior, JJD defines it precisely
2. **No jq in final system** — all JSON ops move to `vvx jjx_*`
3. **Atomic writes** — `vvx jjx_*` writes to temp file, then renames
4. **Git as journal** — every mutation is a diffable commit
5. **Uniform exit codes** — 0=success, non-zero=failure; answers to stdout, not exit codes

### Source File Names (minted)

Rust source lives in `Tools/jjk/veiled/src/`:
| File | Prefix | Purpose |
|------|--------|---------|
| `jjrf_favor.rs` | `jjrf_` | Favor encode/decode |
| `jjrg_gallops.rs` | `jjrg_` | Gallops JSON operations |
| `jjrc_core.rs` | `jjrc_` | Shared infrastructure |
| `jjrn_notch.rs` | `jjrn_` | Notch/chalk commit operations |
| `jjrs_steeplechase.rs` | `jjrs_` | Steeplechase (rein) operations |
| `lib.rs` | — | Crate root, module declarations |

### Prefix Changes (Studbook → Gallops)

| Old | New | Meaning |
|-----|-----|---------|
| `jjds*` | `jjdg*` | Gallops record/members in JJD |
| `jjdsr_studbook` | `jjdgr_gallops` | Root record |
| `jjdsm_seed` | `jjdgm_seed` | Next heat seed member |
| `jjs_studbook.json` | `jjg_gallops.json` | Runtime JSON file |

## Done

- **rename-studbook-to-gallops** — [Phase 0] Rename throughout JJD spec and update all prefixes. ✓

- **cli-structure-and-voicing** — [Phase 1] Established CLI voicing hierarchy in JJD (jjdx_vvx, jjdx_cli) and AXLA (axi_cli_command_group, axa_argument_list, axa_cli_option, axa_exit_*); added section header and argument linked terms; Compliance Rules section. ✓

- **operation-template-finalize** — [Phase 1] Added axi_cli_subcommand voicing to 8 operations; documented shared {jjda_file} pattern. ✓

- **pace-state-autonomy** — [Phase 1] Redesigned state enum: replaced pending/current with rough/primed; added direction field to Tack (required iff primed); tally handles all state transitions with --state and --direction args. ✓

- **spec-slate-reslate** — [Phase 2] Added next_pace_seed to Heat; documented Slate with full behavior specs. (Reslate later eliminated; see spec-rail-tally.) ✓

- **spec-rail-tally** — [Phase 2] Eliminated Reslate; documented Rail; unified Tally with optional --state and full behavior specs. ✓

- **spec-read-ops-query** — [Phase 2] Documented query operations: Validate (confirmed complete), Heat Exists (added Args/Behavior), Muster (full documentation with --status filter). ✓

- **spec-read-ops-extract** — [Phase 2] Simplified extraction: eliminated current_tack/current_pace, added jjx_saddle (bundled context), renamed retire_extract → retire with full JSON spec. ✓

- **jjd-steeplechase-ops** — [Phase 2] Added Steeplechase Operations section with commit patterns; documented jjx_rein (git log parsing), jjx_notch (JJ commit), jjx_chalk (markers). ✓

- **jjr-cargo-scaffold** — [Phase 3] Created JJK Rust crate structure. Cargo.toml, lib.rs, module stubs. VOK integration. ✓

- **jjr-favor-encoding** — [Phase 3] Implemented Firemark/Coronet encode/decode. 27 tests. ✓

- **jjr-gallops-validate** — [Phase 3] Implemented Gallops schema validation. load/save/validate. 30 tests. ✓

- **jjr-write-ops** — [Phase 3] Implemented nominate, slate, rail, tally. Atomic writes, seed increment. ✓

- **jjr-read-ops** — [Phase 3] Implemented muster, saddle, parade, retire. JSON output formats. ✓

- **vvr-commit-core** — [Phase 3] Built core commit infrastructure in VOK. Lock, stage, guard, claude-for-message. ✓

- **jjr-notch-chalk** — [Phase 3] Built JJ commit wrappers. jjx_notch, jjx_chalk via commit core. ✓

- **jjr-steeplechase-rein** — [Phase 3] Implemented git log parsing for steeplechase entries. ✓

## Remaining

- **jjb-orchestration-update** — [Phase 4] Update bash utility to call `vvx jjx_*` instead of jq.
  **Source file**: `Tools/jjk/jju_utility.sh`
  **Context**: The bash utility uses old studbook schema and jq pipelines. Most functions can be deleted since Rust handles everything. Keep minimal wrappers if needed for tabtargets.
  **Deliverables**:
  (1) Delete or stub out functions that duplicate `vvx jjx_*` operations
  (2) Update path constant: `jjs_studbook.json` → `jjg_gallops.json`
  (3) Update terminology: Favor → Firemark/Coronet
  (4) Verify remaining bash (if any) calls correct `vvx` commands
  **Success criteria**: `cargo test` still passes; bash doesn't break arcanum install.

- **test-full-lifecycle** — [Phase 4] Integration test of complete heat lifecycle via `vvx jjx_*`.
  **Deliverables**:
  (1) Create test Gallops file: `echo '{"next_heat_seed":"AA","heats":{}}' > test_gallops.json`
  (2) Create heat: `vvx jjx_nominate --file test_gallops.json --silks test-heat --created 260113`
  (3) Add pace: `echo "Test spec" | vvx jjx_slate --file test_gallops.json <FIREMARK> --silks test-pace`
  (4) Query: `vvx jjx_muster --file test_gallops.json`
  (5) Saddle: `vvx jjx_saddle --file test_gallops.json <FIREMARK>`
  (6) Tally complete: `echo "Done" | vvx jjx_tally --file test_gallops.json <CORONET> --state complete`
  (7) Retire: `vvx jjx_retire --file test_gallops.json <FIREMARK>`
  **Success criteria**: Full lifecycle works end-to-end; JSON valid at each step.

- **arcanum-state-workflow** — [Phase 4] Update arcanum emitters for rough/primed workflow.
  **Source file**: `Tools/jjk/jja_arcanum.sh`
  **Context**: Current arcanum uses old states (pending/current) and old commands (pace-arm, pace-fly). Need to reflect new rough/primed model.
  **Deliverables**:
  (1) Update `zjjw_emit_heat_saddle`:
      - Call `vvx jjx_saddle <FIREMARK>` and parse JSON output
      - Branch on `pace_state`: rough → propose approach; primed → execute per direction
      - Display paddock_content and tack_text
  (2) Add `zjjw_emit_pace_prime`:
      - LLM studies pace spec from saddle context
      - Recommend agent type (haiku/sonnet/opus) and strategy
      - Call `echo "<direction>" | vvx jjx_tally <CORONET> --state primed --direction "..."`
  (3) Update `zjjw_emit_pace_wrap`:
      - Call `echo "outcome" | vvx jjx_tally <CORONET> --state complete`
      - Call `vvx jjx_chalk <FIREMARK> --marker WRAP --description "..."`
  (4) Remove `zjjw_emit_pace_arm` and `zjjw_emit_pace_fly` (replaced by prime workflow)
  (5) Update `zjjw_emit_claudemd_section`:
      - New command list: `/jja-heat-saddle`, `/jja-pace-prime`, `/jja-pace-wrap`, `/jja-notch`, etc.
      - Explain rough/primed workflow
      - Update terminology: Firemark (Heat ID), Coronet (Pace ID)
  **Success criteria**: Fresh arcanum install produces working slash commands.

- **arcanum-commit-commands** — [Phase 4] Update arcanum to emit thin commit slash commands.
  **Source file**: `Tools/jjk/jja_arcanum.sh`
  **Context**: Slash commands extract context and invoke `vvx` directly. No haiku agent needed.
  **Actual CLI syntax** (must match exactly):
  - `vvx jjx_notch <FIREMARK> --pace <SILKS> [--message <MSG>]`
  - `vvx jjx_chalk <FIREMARK> --marker <TYPE> --description <TEXT>`
  - `vvx commit [--prefix <PREFIX>] [--message <MSG>]`
  **Deliverables**:
  (1) Update `zjjw_emit_notch_command`:
      - Extract FIREMARK and pace SILKS from conversation context
      - Run `vvx jjx_notch <FIREMARK> --pace <SILKS>` via Bash tool
      - If no staged changes, fail early
      - Return "Committed: <hash>" when done
  (2) Add `zjjw_emit_chalk_command`:
      - Extract FIREMARK from conversation context
      - Take --marker (APPROACH/WRAP/FLY/DISCUSSION) and --description from args
      - Run `vvx jjx_chalk <FIREMARK> --marker <TYPE> --description "<TEXT>"`
  (3) Add `/vvc-commit` for non-JJ repos:
      - Run `vvx commit --message "<ARGS>"` if args provided
      - Run `vvx commit` (claude generates message) if no args
  **Success criteria**: `/jja-notch` produces JJ-prefixed commit via Rust.

- **axla-relational-voicing** — [Phase 5 - Future] Evaluate AXLA voicings for relational table concepts.
  **Context**: As JJD defines structured data (Gallops JSON with heats, paces, tacks), consider whether AXLA should provide voicings to express database integrity concepts (foreign keys, referential integrity, cardinality, normalization).
  **Deliverables**:
  (1) Survey existing relational concepts in JJD and other specs
  (2) Evaluate whether explicit voicings would add clarity or are unnecessary
  (3) If worthwhile, propose specific voicings following MCM patterns
  **Success criteria**: Clear decision documented; if yes, AXLA updated accordingly.

## Steeplechase

---
### 2026-01-12 - Heat Created

**Context**: Spun off from jj-studbook-redesign heat to complete JJD specification and implement jjr Rust backend.

**Key decisions**:
- Renamed Studbook → Gallops (avoid 's' collision with specification)
- jjr owns all JSON file access (Option B from prior discussion)
- Phased approach: vocabulary migration → AXLA/style → operation specs → Rust implementation

**Prior work carried forward**:
- JJD spec has Types, Records, Serialization complete
- Nominate and Validate operations mostly documented
- Horse racing vocabulary established (Firemark, Coronet, Tack, Silks, Favor)

---
### 2026-01-13 - rust-commit-architecture - DISCUSSION
**Context**: Re-evaluated how JJ commit commands (notch, chalk) should integrate with VVK infrastructure.

**Original plan**: Slash commands spawn haiku agents; JJ uses `vvc-commit --prefix` parameter.

**Problems identified**:
1. Skill-calling-skill not supported (slash commands can't invoke other slash commands)
2. Behavioral divergence (notch auto-pushed, vvc-commit didn't; different staging models)
3. Agent configuration duplicated across multiple slash command prompts
4. No auto-push allowed (learned constraint)

**Key insight**: Rust can shell out to `claude` CLI for LLM-generated commit messages.

**New architecture**:
- `vvx commit` (VOK) — core infrastructure: lock, stage, guard, claude call, commit
- `vvx jjx_notch` / `vvx jjx_chalk` (JJK) — JJ-specific wrappers using core
- Flat namespace: all JJK ops are `vvx jjx_*` (underscore-delimited single tokens)
- Single lock for all vvx operations (`refs/vvg/locks/vvx`)
- Slash commands become thin: extract context → background `vvx` call → return immediately
- No haiku agent spawning needed; Rust orchestrates everything including LLM call
- If `claude` CLI unavailable: fail with error, require `--message` fallback

**Paces updated**:
- Removed: `vvc-prefix-support` (obsolete)
- Added: `vvr-commit-core` (VOK Rust commit infrastructure)
- Added: `jjr-notch-chalk` (JJK Rust commit wrappers)
- Renamed: `arcanum-vvc-integration` → `arcanum-commit-commands` (thin slash command emitters)
- Revised: `jjb-orchestration-update` (bash delegates commits to Rust)
- Revised: `jjd-steeplechase-ops` (notch/chalk spec only, impl in jjr-notch-chalk)
- Added: `jjrn_` and `jjrs_` to Source File Names table
---
