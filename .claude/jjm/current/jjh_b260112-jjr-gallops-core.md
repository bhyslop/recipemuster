# Heat: JJR Gallops Core

Implement jjr, the Rust CLI that owns all Gallops (formerly Studbook) JSON operations. Complete the JJD specification, then build the Rust implementation informed by VOK/VVK patterns.

## Paddock

### Why "Gallops"

The gallops are where horses train - where real work happens before race day. The Gallops JSON tracks heats (initiatives), paces (tasks), and tacks (refinements). Renamed from "Studbook" to avoid 's' collision with "specification" in our mental model.

### Architecture Recap

**jjr owns all Gallops file access.** No jq, no direct JSON manipulation. Bash handles locking via `git update-ref`; jjr handles read, transform, atomic write.

```
Bash (locking, git ops, orchestration)
  │
  └─► jjr (Rust) - all JSON operations
        ├─ favor encode/decode
        ├─ gallops validate
        ├─ gallops transform (nominate, slate, tally, etc.)
        └─ gallops query (muster, heat_exists, current_pace, etc.)
```

### Essential References

**JJD Specification** (our target):
- `Tools/jjk/JJD-StudbookData.adoc` → rename to `JJD-GallopsData.adoc`

**Bash Prototype** (behavior reference):
- `Tools/jjk/jju_utility.sh` — current jq-based implementation

**Rust Infrastructure** (read before Phase 3):
- `Tools/vok/README.md` — VOK patterns and conventions
- `Tools/vvk/README.md` — VVK infrastructure for veiled Rust crates

**AXLA Lexicon** (for voicing decisions):
- `Tools/cmk/AXLA-Lexicon.adoc`

### Key Constraints

1. **JJD is the authority** — bash prototype shows behavior, JJD defines it precisely
2. **No jq in final system** — all JSON ops move to jjr
3. **Atomic writes** — jjr writes to temp file, then renames
4. **Git as journal** — every mutation is a diffable commit
5. **Exit codes matter** — queries return 0/1 as boolean result, mutations return 0 on success

### Prefix Changes (Studbook → Gallops)

| Old | New | Meaning |
|-----|-----|---------|
| `jjds*` | `jjdg*` | Gallops record/members in JJD |
| `jjdsr_studbook` | `jjdgr_gallops` | Root record |
| `jjdsm_seed` | `jjdgm_seed` | Next heat seed member |
| `jjs_studbook.json` | `jjg_gallops.json` | Runtime JSON file |

## Done

(none yet)

## Remaining

### Phase 0: Vocabulary Migration

- **rename-studbook-to-gallops** — Rename throughout JJD spec and update all prefixes.
  **Deliverables**:
  (1) Rename `JJD-StudbookData.adoc` → `JJD-GallopsData.adoc`
  (2) Update category declarations: `jjdsr_` → `jjdgr_`, `jjdsm_` → `jjdgm_`
  (3) Update all attribute references and anchors
  (4) Update prose references (Studbook → Gallops)
  (5) Update file path reference (`.claude/jjm/jjd_studbook.json` → `.claude/jjm/jjg_gallops.json`)
  **Success criteria**: Document renders correctly, all internal links resolve, grep finds no "studbook" references.

### Phase 1: AXLA/Style Foundation

- **exit-status-treatment** — Decide and document exit code semantics for operations.
  **Context**: AXLA has `axc_fatal` and `axc_warn` but no explicit exit code motifs. Queries (heat_exists, current_pace) use exit code as boolean result (0=true, 1=false), not error indicator. Mutations use 0=success, non-zero=fatal.
  **Deliverables**:
  (1) Decide: add AXLA terms, JJD-local terms, or prose-only documentation
  (2) Add Exit Status subsection to each operation in JJD
  (3) Document the query-vs-mutation distinction
  **Success criteria**: Every operation has clear exit status documentation.

- **operation-template-finalize** — Establish DRY template for operation documentation.
  **Context**: Nominate is fully documented with Arguments, Stdout, Behavior. Other ops are stubs.
  **Deliverables**:
  (1) Confirm template sections: Brief, Arguments, Stdout, Exit Status, Behavior (numbered steps)
  (2) Document shared argument `{jjda_file}` usage pattern
  (3) Add any missing AXLA voicing annotations to operations
  **Success criteria**: Template is clear enough to apply mechanically to remaining operations.

### Phase 2: Operation Specifications

- **spec-slate-reslate** — Document Slate and Reslate operations in JJD.
  **Reference**: `jju_slate()` lines 624-715, `jju_reslate()` lines 717-791
  **Deliverables**:
  (1) Slate: Arguments, Stdout, Exit Status, Behavior (auto-assign Coronet, first pace gets `current` state)
  (2) Reslate: Arguments, Stdout, Exit Status, Behavior (prepend Tack to position 0)
  **Success criteria**: Operations fully specified with all sections.

- **spec-rail-tally** — Document Rail and Tally operations in JJD.
  **Reference**: `jju_rail()` lines 793-889, `jju_tally()` lines 891-972
  **Deliverables**:
  (1) Rail: Arguments (heat favor, order string), validation (same key set), Behavior
  (2) Tally: Arguments (pace favor, state), valid states, Behavior
  **Success criteria**: Operations fully specified with all sections.

- **spec-read-ops-query** — Document query operations: Validate, Heat Exists, Muster.
  **Reference**: `zjju_studbook_validate()` lines 235-279, `jju_muster()` lines 349-386
  **Note**: Heat Exists not explicit in bash - derive from validation pattern
  **Deliverables**:
  (1) Validate: validation rules already listed, add Exit Status
  (2) Heat Exists: Arguments, Exit Status (0=exists, 1=not found), no stdout
  (3) Muster: output format, Exit Status
  **Success criteria**: Query operations fully specified.

- **spec-read-ops-extract** — Document extraction operations: Retire Extract, Current Tack, Current Pace.
  **Reference**: `jju_retire_extract()` lines 974-1107
  **Note**: Current Tack/Current Pace not explicit - derive from saddle/wrap usage
  **Deliverables**:
  (1) Retire Extract: output JSON structure for trophy creation
  (2) Current Tack: return latest Tack text for a Pace
  (3) Current Pace: return first pending/current Pace Favor, exit 1 if none
  **Success criteria**: Extraction operations fully specified.

### Phase 3: Rust Implementation

**IMPORTANT**: Before starting any Phase 3 pace, read:
- `Tools/vok/README.md` — VOK patterns and conventions
- `Tools/vvk/README.md` — VVK infrastructure for veiled Rust crates

- **jjr-cargo-scaffold** — Create jjr crate structure following VOK/VVK patterns.
  **Deliverables**:
  (1) `Tools/jjk/jjr/Cargo.toml` with minimal deps (serde, serde_json, clap)
  (2) `src/main.rs` with clap command structure
  (3) Module structure: `favor.rs`, `gallops.rs`, `ops/` directory
  **Success criteria**: `cargo build` succeeds, `jjr --help` shows command structure.

- **jjr-favor-encoding** — Implement Favor encode/decode in Rust.
  **Reference**: JJD Serialization section, `zjju_favor_encode/decode()` in bash
  **Deliverables**:
  (1) `favor.rs` module with encode/decode functions
  (2) Firemark (0-4095) ↔ 2 base64 chars
  (3) Coronet (0-262143) ↔ 3 base64 chars
  (4) Unit tests for encode/decode roundtrip
  **Success criteria**: All favor encoding tests pass.

- **jjr-gallops-validate** — Implement Gallops schema validation in Rust.
  **Reference**: JJD Validate operation, `zjju_studbook_validate()` in bash
  **Deliverables**:
  (1) `gallops.rs` with serde structs matching JJD schema
  (2) Validation function checking all rules from JJD
  (3) `jjr gallops validate --file PATH` command
  **Success criteria**: Validates correct JSON, rejects malformed with clear errors.

- **jjr-write-ops** — Implement write operations: nominate, slate, reslate, rail, tally.
  **Reference**: JJD operation specs (completed in Phase 2)
  **Deliverables**:
  (1) Each operation as subcommand: `jjr gallops nominate --file PATH --silks X --created YYMMDD`
  (2) Atomic write (temp file + rename)
  (3) Stdout output per JJD spec
  **Success criteria**: All write operations match JJD behavior spec.

- **jjr-read-ops** — Implement read operations: heat-exists, muster, retire-extract, current-tack, current-pace.
  **Reference**: JJD operation specs (completed in Phase 2)
  **Deliverables**:
  (1) Each operation as subcommand
  (2) Exit codes per JJD spec (queries return 0/1)
  (3) Stdout output per JJD spec
  **Success criteria**: All read operations match JJD behavior spec.

### Phase 4: Integration

- **jjb-orchestration-update** — Update bash orchestration to call jjr instead of jq.
  **Deliverables**:
  (1) Replace jq pipelines in jju_utility.sh with jjr calls
  (2) Maintain locking pattern (lock → jjr → unlock)
  (3) Update file path constant to `jjg_gallops.json`
  **Success criteria**: All existing JJ commands work with jjr backend.

- **test-full-lifecycle** — Integration test of complete heat lifecycle via jjr.
  **Deliverables**:
  (1) Create test heat via jjr nominate
  (2) Add paces via jjr slate
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
