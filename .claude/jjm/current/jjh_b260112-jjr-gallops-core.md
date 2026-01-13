# Heat: JJR Gallops Core

Implement `vvx jjx`, the Job Jockey subcommand that owns all Gallops JSON operations. Complete the JJD specification, then build the Rust implementation informed by VOK/VVK patterns.

## Paddock

### Why "Gallops"

The gallops are where horses train - where real work happens before race day. The Gallops JSON tracks heats (initiatives), paces (tasks), and tacks (refinements). Renamed from "Studbook" to avoid 's' collision with "specification" in our mental model.

### Architecture Recap

**`vvx jjx` owns all Gallops file access.** No jq, no direct JSON manipulation. Bash handles locking via `git update-ref`; `vvx jjx` handles read, transform, atomic write.

```
Bash (locking, git ops, orchestration)
  │
  └─► vvx jjx (Rust) - all JSON operations
        ├─ favor encode/decode
        ├─ gallops validate
        ├─ gallops transform (nominate, slate, tally, etc.)
        └─ gallops query (muster, heat_exists, current_pace, etc.)
```

### Pacing Strategy

Work proceeds in logical phases. Each pace notes its phase for context.

- **Phase 0 (Vocabulary)**: Rename studbook → gallops throughout
- **Phase 1 (Foundation)**: AXLA voicing, operation template style
- **Phase 2 (Specification)**: Complete all JJD operation specs
- **Phase 3 (Rust)**: Implement `vvx jjx` — read VOK/VVK READMEs first
- **Phase 4 (Integration)**: Wire bash to `vvx jjx`, end-to-end test

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
2. **No jq in final system** — all JSON ops move to `vvx jjx`
3. **Atomic writes** — `vvx jjx` writes to temp file, then renames
4. **Git as journal** — every mutation is a diffable commit
5. **Uniform exit codes** — 0=success, non-zero=failure; answers to stdout, not exit codes

### Source File Names (minted)

Rust source lives in `Tools/jjk/veiled/src/`:
| File | Prefix | Purpose |
|------|--------|---------|
| `jjrf_favor.rs` | `jjrf_` | Favor encode/decode |
| `jjrg_gallops.rs` | `jjrg_` | Gallops JSON operations |
| `jjrc_core.rs` | `jjrc_` | Shared infrastructure |
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

## Remaining

- **cli-structure-and-voicing** — [Phase 1] Establish `vvx jjx` CLI structure in JJD using new AXLA terms.
  **Context**: Minted `axi_cli_program` and `axi_cli_subcommand` in AXLA. All operations are subcommands with uniform exit semantics: 0=success, non-zero=failure. No predicate/boolean exit codes — fact-finding ops output answers to stdout. Updated JJD to use `vvx jjx` naming.
  **Deliverables**:
  (1) Add "vvx jjx CLI" section to JJD with `axi_cli_program` voicing, documenting global behavior (exit semantics, stdout/stderr, non-interactive)
  (2) Add `axi_cli_subcommand` voicing annotation to each operation
  (3) Add *Exit Status:* line to each operation (uniform: 0=success, non-zero=error)
  (4) For query ops (heat_exists, validate, etc.), clarify that answers go to stdout, not exit code
  **Success criteria**: JJD has clear CLI structure; all operations voiced as `axi_cli_subcommand` with uniform exit semantics.

- **operation-template-finalize** — [Phase 1] Establish DRY template for operation documentation.
  **Context**: Nominate is fully documented with Arguments, Stdout, Behavior. Other ops are stubs.
  **Deliverables**:
  (1) Confirm template sections: Brief, Arguments, Stdout, Exit Status, Behavior (numbered steps)
  (2) Document shared argument `{jjda_file}` usage pattern
  (3) Add any missing AXLA voicing annotations to operations
  **Success criteria**: Template is clear enough to apply mechanically to remaining operations.

- **pace-state-autonomy** — [Phase 1] Explore whether Pace state enum should capture readiness for autonomous execution.
  **Context**: Current states (pending/current/complete/abandoned) track progress but not whether a pace spec is detailed enough for unattended model execution vs. needing human collaboration.
  **Deliverables**:
  (1) Evaluate: expand state enum (e.g., add `armed`) vs. status quo
  (2) If expanding, update JJD Pace state values and document semantics
  (3) If expanding, define `vvx jjx` operation to transition pace to armed state
  **Success criteria**: Clear decision documented; if yes, JJD updated accordingly.

- **spec-slate-reslate** — [Phase 2] Document Slate and Reslate operations in JJD.
  **Reference**: `jju_slate()` lines 624-715, `jju_reslate()` lines 717-791
  **Deliverables**:
  (1) Slate: Arguments, Stdout, Exit Status, Behavior (auto-assign Coronet, first pace gets `current` state)
  (2) Reslate: Arguments, Stdout, Exit Status, Behavior (prepend Tack to position 0)
  **Success criteria**: Operations fully specified with all sections.

- **spec-rail-tally** — [Phase 2] Document Rail and Tally operations in JJD.
  **Reference**: `jju_rail()` lines 793-889, `jju_tally()` lines 891-972
  **Deliverables**:
  (1) Rail: Arguments (heat favor, order string), validation (same key set), Behavior
  (2) Tally: Arguments (pace favor, state), valid states, Behavior
  **Success criteria**: Operations fully specified with all sections.

- **spec-read-ops-query** — [Phase 2] Document query operations: Validate, Heat Exists, Muster.
  **Reference**: `zjju_studbook_validate()` lines 235-279, `jju_muster()` lines 349-386
  **Note**: Heat Exists not explicit in bash - derive from validation pattern
  **Deliverables**:
  (1) Validate: validation rules already listed, add Exit Status (0=valid, non-zero=error)
  (2) Heat Exists: Arguments, Stdout (true/false), Exit Status (uniform 0=success)
  (3) Muster: output format, Exit Status
  **Success criteria**: Query operations fully specified with uniform exit semantics.

- **spec-read-ops-extract** — [Phase 2] Document extraction operations: Retire Extract, Current Tack, Current Pace.
  **Reference**: `jju_retire_extract()` lines 974-1107
  **Note**: Current Tack/Current Pace not explicit - derive from saddle/wrap usage
  **Deliverables**:
  (1) Retire Extract: output JSON structure for trophy creation
  (2) Current Tack: return latest Tack text for a Pace
  (3) Current Pace: return first pending/current Pace Favor, exit 1 if none
  **Success criteria**: Extraction operations fully specified.

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
  (3) `vvx jjx gallops validate --file PATH` subcommand
  **Success criteria**: Validates correct JSON, rejects malformed with clear errors.

- **jjr-write-ops** — [Phase 3] Implement write operations: nominate, slate, reslate, rail, tally.
  **Reference**: JJD operation specs (completed in Phase 2)
  **Source file**: `jjrg_gallops.rs`
  **Deliverables**:
  (1) Each operation as subcommand: `vvx jjx gallops nominate --file PATH --silks X --created YYMMDD`
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

- **jjb-orchestration-update** — [Phase 4] Update bash orchestration to call `vvx jjx` instead of jq.
  **Source file**: `jju_utility.sh`
  **Deliverables**:
  (1) Replace jq pipelines in `jju_utility.sh` with `vvx jjx` calls
  (2) Maintain locking pattern (lock → `vvx jjx` → unlock)
  (3) Update file path constant to `jjg_gallops.json`
  **Success criteria**: All existing JJ commands work with `vvx jjx` backend.

- **test-full-lifecycle** — [Phase 4] Integration test of complete heat lifecycle via `vvx jjx`.
  **Deliverables**:
  (1) Create test heat via `vvx jjx gallops nominate`
  (2) Add paces via `vvx jjx gallops slate`
  (3) Run saddle/wrap/retire cycle
  (4) Verify trophy creation
  **Success criteria**: Full lifecycle works end-to-end with Rust backend.

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
