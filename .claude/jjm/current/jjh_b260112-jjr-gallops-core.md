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

## Remaining

- **jjr-cargo-scaffold** — [Phase 3] Create JJK Rust crate structure following VOK/VVK patterns.
  **Prerequisite**: Read `Tools/vok/README.md` and `Tools/vvk/README.md` before starting.
  **Source files**: `Tools/jjk/veiled/Cargo.toml`, `lib.rs`, `jjrf_favor.rs`, `jjrg_gallops.rs`, `jjrc_core.rs`
  **Deliverables**:
  (1) `Tools/jjk/veiled/Cargo.toml` with `[lib] name = "jjk"`, minimal deps (serde, serde_json)
  (2) `lib.rs` with module declarations
  (3) Stub files: `jjrf_favor.rs`, `jjrg_gallops.rs`, `jjrc_core.rs`
  (4) Update VOK `Cargo.toml` to add jjk optional dependency
  **Success criteria**: `cargo build --features jjk` succeeds in VOK.

- **jjr-favor-encoding** — [Phase 3] Implement Favor encode/decode in Rust.
  **Reference**: JJD Serialization section, `zjju_favor_encode/decode()` in bash
  **Source file**: `jjrf_favor.rs`
  **Deliverables**:
  (1) Encode/decode functions in `jjrf_favor.rs`
  (2) Firemark (0-4095) ↔ 2 base64 chars
  (3) Coronet (0-262143) ↔ 3 base64 chars
  (4) Unit tests for encode/decode roundtrip
  **Success criteria**: All favor encoding tests pass.

- **jjr-gallops-validate** — [Phase 3] Implement Gallops schema validation in Rust.
  **Reference**: JJD Validate operation, `zjju_studbook_validate()` in bash
  **Source file**: `jjrg_gallops.rs`
  **Deliverables**:
  (1) Serde structs in `jjrg_gallops.rs` matching JJD schema
  (2) Validation function checking all rules from JJD
  (3) `vvx jjx_validate --file PATH` subcommand
  **Success criteria**: Validates correct JSON, rejects malformed with clear errors.

- **jjr-write-ops** — [Phase 3] Implement write operations: nominate, slate, rail, tally.
  **Reference**: JJD operation specs (completed in Phase 2)
  **Source file**: `jjrg_gallops.rs`
  **Deliverables**:
  (1) Each operation as subcommand: `vvx jjx_nominate --file PATH --silks X --created YYMMDD`
  (2) Atomic write (temp file + rename)
  (3) Stdout output per JJD spec
  **Success criteria**: All write operations match JJD behavior spec.

- **jjr-read-ops** — [Phase 3] Implement read operations: heat-exists, muster, retire-extract, current-tack, current-pace.
  **Reference**: JJD operation specs (completed in Phase 2)
  **Source file**: `jjrg_gallops.rs`
  **Deliverables**:
  (1) Each operation as subcommand
  (2) Uniform exit codes (0=success, non-zero=error), answers to stdout
  (3) Stdout output per JJD spec
  **Success criteria**: All read operations match JJD behavior spec.

- **vvr-commit-core** — [Phase 3] Build core commit infrastructure in VOK Rust.
  **Context**: Unified commit implementation that both vvc-commit and JJ commands use. Rust orchestrates: lock, stage, guard, claude call, commit.
  **Location**: `Tools/vok/src/vorc_commit.rs` (new file)
  **Deliverables**:
  (1) `vvx commit` subcommand with args: `--prefix`, `--allow-empty`, `--message` (optional, triggers claude call if absent)
  (2) Lock acquire via git update-ref before any operation
  (3) Stage files (`git add -u` by default, or respect pre-staged with `--no-stage`)
  (4) Size guard check (reuse existing vorg_guard.rs)
  (5) Shell to `claude --print "..."` to generate commit message from diff (if no --message)
  (6) `git commit` with formatted message
  (7) Lock release on success or failure
  (8) If `claude` CLI unavailable or fails: exit with error, do not commit (require --message as fallback)
  **Success criteria**: `vvx commit` produces guarded commit with LLM-generated message.

- **jjr-notch-chalk** — [Phase 3] Build JJ commit commands in JJK Rust.
  **Context**: JJ-specific commit wrappers that use vvr-commit-core.
  **Depends on**: vvr-commit-core, jjr-cargo-scaffold
  **Location**: `Tools/jjk/veiled/src/jjrn_notch.rs`
  **Deliverables**:
  (1) `vvx jjx_notch --heat FAVOR --pace SILKS` — formats `[jj:BRAND][heat/pace]` prefix, calls commit core
  (2) `vvx jjx_chalk --heat FAVOR --marker TYPE` — steeplechase marker, uses `--allow-empty`
  (3) Marker types: APPROACH, WRAP, FLY, DISCUSSION
  (4) Brand value baked in at arcanum emit time (see jja_arcanum.sh)
  **Success criteria**: `vvx jjx_notch` produces JJ-prefixed commit via core infrastructure.

- **jjr-steeplechase-rein** — [Phase 3] Implement steeplechase rein in Rust.
  **Reference**: JJD steeplechase-ops spec
  **Depends on**: jjr-cargo-scaffold
  **Location**: `Tools/jjk/veiled/src/jjrs_steeplechase.rs`
  **Deliverables**:
  (1) Parse git log for commits matching JJ steeplechase patterns
  (2) Extract and format entries (APPROACH, WRAP, FLY, DISCUSSION)
  (3) Add `vvx jjx_rein --heat ₣HH` subcommand
  (4) Output formatted history for saddle context display
  **Success criteria**: Returns formatted steeplechase history for specified heat.

- **jjb-orchestration-update** — [Phase 4] Update bash orchestration to call `vvx` instead of jq.
  **Source file**: `jju_utility.sh`
  **Deliverables**:
  (1) Replace jq pipelines in `jju_utility.sh` with `vvx jjx_*` calls
  (2) Update file path constant to `jjg_gallops.json`
  **Clarification - bash retains**:
  - Slash command dispatch (tabtargets)
  **Clarification - bash delegates**:
  - All JSON operations → `vvx jjx_*` (validate, nominate, slate, etc.)
  - Steeplechase reading → `vvx jjx_rein`
  - Commits → `vvx jjx_notch` / `vvx jjx_chalk` (Rust owns locking)
  **Success criteria**: All existing JJ commands work with `vvx` backend.

- **test-full-lifecycle** — [Phase 4] Integration test of complete heat lifecycle via `vvx jjx_*`.
  **Deliverables**:
  (1) Create test heat via `vvx jjx_nominate`
  (2) Add paces via `vvx jjx_slate`
  (3) Run saddle/wrap/retire cycle
  (4) Verify trophy creation
  **Success criteria**: Full lifecycle works end-to-end with Rust backend.

- **arcanum-state-workflow** — [Phase 4] Update arcanum emitters for rough/primed workflow.
  **Context**: New states require different saddle behavior and new prime command.
  **Source file**: `Tools/jjk/jja_arcanum.sh`
  **Deliverables**:
  (1) Update `zjjw_emit_heat_saddle`: branch on pace state
      - rough: guide LLM to refine spec, recommend approach, ask before proceeding
      - primed: read direction field, execute per direction without asking
  (2) Add `zjjw_emit_pace_prime`: study pace spec, recommend agent type + cardinality, call `vvx jjx_tally --state primed --direction "..."`
  (3) Update `zjjw_emit_pace_wrap`: works with rough/primed states
  (4) Remove `zjjw_emit_pace_arm` and `zjjw_emit_pace_fly` (replaced by prime + primed execution)
  (5) Update `zjjw_emit_claudemd_section`: new command list, rough/primed concepts
  **Success criteria**: Fresh arcanum install produces commands reflecting rough/primed workflow.

- **arcanum-commit-commands** — [Phase 4] Emit thin slash commands for Rust commit infrastructure.
  **Context**: Slash commands extract context and invoke background `vvx` processes. No agent spawning needed.
  **Depends on**: vvr-commit-core, jjr-notch-chalk
  **Source file**: `Tools/jjk/jja_arcanum.sh`
  **Deliverables**:
  (1) Update `zjjw_emit_notch_command`: extract heat/pace from conversation context, invoke `vvx jjx_notch --heat X --pace Y` via Bash tool with run_in_background=true
  (2) Update `zjjw_emit_chalk_command`: extract heat from context, invoke `vvx jjx_chalk --heat X --marker TYPE` (background)
  (3) Emit `/vvc-commit` that invokes `vvx commit --message "$ARGS"` (background) for non-JJ repos
  **Success criteria**: `/jja-notch` produces JJ-prefixed commits via background Rust process.

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
