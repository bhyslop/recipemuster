# Steeplechase: JJR Gallops Core

---
### 2026-01-12 10:30 - rename-studbook-to-gallops - APPROACH
**Proposed approach**:
- Rename file `JJD-StudbookData.adoc` → `JJD-GallopsData.adoc`
- Update category declarations in mapping section: `jjdsr_` → `jjdgr_`, `jjdsm_` → `jjdgm_`
- Update all attribute references and anchors using those prefixes
- Search/replace prose references: "Studbook" → "Gallops", "studbook" → "gallops"
- Update file path reference: `jjd_studbook.json` → `jjg_gallops.json`
- Verify with grep that no "studbook" references remain
---

---
### 2026-01-12 15:40 - exit-status-treatment - DISCUSSION
**Key decisions from collaborative session**:

1. **No predicate/boolean exit codes**: Rejected Unix `test`/`grep -q` pattern where exit 0=true, 1=false. All jjr operations use uniform exit semantics: 0=success, non-zero=failure.

2. **Answers to stdout, not exit code**: Fact-finding operations (heat_exists, validate) output their answers to stdout. Exit code only indicates whether the operation completed successfully.

3. **New AXLA terms minted**:
   - `axi_cli_program` — Unix-style CLI program (stdout, stderr, exit status)
   - `axi_cli_subcommand` — operation within a CLI program, inherits parent semantics

4. **Pace renamed**: `exit-status-treatment` → `cli-structure-and-voicing` to reflect broader scope (CLI structure + voicing + exit semantics).

5. **New pace added**: `pace-state-autonomy` — explore whether Pace state enum should capture readiness for autonomous execution (raw vs armed).

**Rationale**: Uniform exit semantics are simpler and align with modern CLI conventions. The predicate pattern adds complexity (three outcomes: true/false/error) without clear benefit.
---

---
### 2026-01-13 16:45 - cli-structure-and-voicing - WRAP
**Outcome**: Established CLI voicing hierarchy (jjdx_vvx/jjdx_cli), AXLA motifs (axi_cli_command_group, axa_argument_list, axa_cli_option, axa_exit_*), section header terms (jjds_*), argument terms (jjda_silks/created), Compliance Rules section. Articulated ops voiced as axi_cli_subcommand.
---

---
### 2026-01-13 - operation-template-finalize - WRAP
**Outcome**: Added axi_cli_subcommand voicing to 8 stub operations; documented shared {jjda_file} pattern in Arguments section.
---

---
### 2026-01-13 - pace-state-autonomy - APPROACH
**Proposed approach**:
- Review current JJD Pace/Tack state definitions and /jja-pace-arm skill usage
- Evaluate: separate state (`armed`) vs. orthogonal flag vs. status quo
- If expanding, add `armed` state to JJD with clear semantics
- If expanding, define `vvx jjx gallops arm` operation
---

---
### 2026-01-13 - pace-state-autonomy - WRAP
**Outcome**: Major state redesign via collaborative discussion.

**States revised**:
- Removed: `pending`, `current` (redundant; both meant "needs work")
- Added: `rough` (needs clarification), `primed` (ready for autonomous execution)
- Kept: `complete`, `abandoned`

**New Tack field**: `direction` — execution guidance for autonomous operation. Required iff state=primed; forbidden otherwise. Contains agent type, cardinality, strategy.

**Tally operation revised**: Now handles all state transitions. Args: --state (required), --direction (required iff primed), stdin text (optional, inherits from previous tack).

**JJD updated**: mapping section, state enum definitions, Tack members, validation rules, current_pace operation, tally arguments, slate initial state.

**Key insight**: "priming" workflow where LLM studies pace spec and recommends agent strategy before transitioning to primed state. Direction captures that recommendation.

**Related decision**: jj-studbook-redesign heat to be abandoned (bash v2 superseded by Rust backend). Added jj-system-integration pace for arcanum updates.
---

---
### 2026-01-13 - pace-planning - DISCUSSION
**Context**: Analyzed path to working JJ with rough/primed states.

**Architecture decisions**:
- jjx owns JSON ops (gallops subcommand) AND steeplechase reading (rein)
- vvc-commit owns all git commits; JJ uses --prefix for context
- Bash retains locking (git update-ref) and slash command dispatch
- notch/chalk become thin wrappers around vvc-commit

**New paces added**:
- jjd-steeplechase-ops [Phase 2] - spec rein operation
- vvc-prefix-support [Phase 3] - add --prefix to vvc-commit (VVK)
- jjr-steeplechase-rein [Phase 3] - implement rein in Rust
- arcanum-state-workflow [Phase 4] - rough/primed emitters
- arcanum-vvc-integration [Phase 4] - notch/chalk via vvc-commit

**Removed**: jj-system-integration (absorbed into specific paces)

**Updated**: jjb-orchestration-update (clarified what bash retains vs delegates)

**jj-studbook-redesign**: To be archived as abandoned (bash v2 superseded).
---
