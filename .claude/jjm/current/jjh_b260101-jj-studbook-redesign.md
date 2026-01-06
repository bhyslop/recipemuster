# Heat: JJ Studbook Redesign

Redesign Job Jockey around a JSON-based studbook registry, git-based steeplechase, and streamlined bash script APIs that keep implementation details hidden from Claude.

## Paddock

### How to Work This Heat

**IMPORTANT**: This heat executes under the CURRENT JJK installation while building its replacement.

- Use existing `/jja-*` commands (heat-saddle, pace-wrap, notch, etc.)
- The new system becomes active only after this heat retires and fresh arcanum install

### No Migration - Retire First

**CRITICAL CONSTRAINT**: There will be NO migration from v1 to v2 schema.

This heat will be **retired** before v2 is installed. The sequence is:
1. Complete all v2 implementation paces (under v1)
2. Retire this heat (creates trophy, clears studbook)
3. Run fresh `jja_arcanum.sh install` (creates empty v2 studbook)
4. Create new heats under v2

**Implications for implementation**:
- V2 code assumes it ONLY ever sees v2 schema (no version detection)
- No dual-path logic or compatibility shims
- No `schema_version` field checking at runtime (validation only)
- Test suites create fresh v2 studbooks, never convert v1

### Schema Simplifications in V2

**`display` field removed from heats**: V1 had both `display` (human-readable) and `silks` (kebab-case). These were redundant — always the same concept in different formats. V2 uses only `silks`. Format for display if needed, or show kebab-case directly.

**Pace description via specs**: Pace `silks` is the stable identifier. The human-readable description is `specs[last].text`. No separate `display` field on paces.

**Steeplechase unchanged**: `chalk`, `rein`, `notch` work with git history, not studbook JSON. They need no v2 variants and are not in scope for these paces.

### File Prefix Conventions

| Prefix | Purpose | Example |
|--------|---------|---------|
| `jja_` | Arcanum (install/uninstall) | `jja_arcanum.sh` |
| `jjb_` | suBagent | `jjb_*.md` |
| `jjc_` | slash Command | `jjc_saddle.md` |
| `jjh_` | Hook | `jjh_*.sh` |
| `jji_` | Itch | `jji_itch.md` |
| `jjl_` | skiLl (future) | `jjl_*.md` |
| `jjp_` | Paddock | `jjp_Kb.md` |
| `jjs_` | Studbook | `jjs_studbook.json` |
| `jjt_` | Testbench | `jjt_testbench.sh` |
| `jju_` | Utility (bash impl) | `jju_utility.sh` |
| `jjw_` | Workbench | `jjw_workbench.sh` |
| `jjy_` | Trophy | `jjy_Kb_260101-260115_silks.md` |
| `jjz_` | Scar (declined itch) | `jjz_scar.md` |

Note: `jjk` refers to the Job Jockey Kit directory (`Tools/jjk/`), not a file prefix. Tabtargets like `tt/jjk-h.Help.sh` indicate kit CLI entry points.

### Testing Patterns

**BCG-Compliant Validation:**
- Write JSON to temp file once: `printf '%s' "${z_json}" > "${z_file}"`
- Multiple `jq -e` checks on same file (exit status only)
- Direct status check: `jq -e 'expression' "${z_file}" >/dev/null 2>&1 || buc_die "message"`
- No command substitution `$(...)` - check exit codes directly

**Test Structure:**
- Production logic in `zjju_*` functions (jju_utility.sh)
- Test wrappers `jjt_*` expose internals (jjt_testbench.sh)
- Test suites `jjt_test_*` with but_expect_ok/fatal (jjt_testbench.sh)
- One tabtarget per suite: `tt/jjt-X.TestY.sh`

**Test Coverage Strategy:**
- Valid cases with `but_expect_ok` / `but_expect_ok_stdout`
- Boundary cases with `but_expect_fatal` (ranges, empty values)
- Invalid inputs with `but_expect_fatal` (format violations, missing fields)

### File Locations

| File Type | Location | Example |
|-----------|----------|---------|
| Studbook | `.claude/jjm/jjs_studbook.json` | Single file |
| Paddock | `.claude/jjm/jjp_Kb.md` | Per heat |
| Trophy | `.claude/jjm/retired/jjy_Kb_*.md` | Retired heats |
| Itch | `.claude/jjm/jji_itch.md` | Single file |
| Scar | `.claude/jjm/jjz_scar.md` | Single file |
| Utility scripts | `Tools/jjk/jju_*.sh` | Bash implementations |
| Commands | `.claude/commands/jjc_*.md` | Slash commands |
| Skills | `.claude/skills/jjl_*.md` | Skills (future) |
| Workbench | `Tools/jjk/jjw_workbench.sh` | Dispatch |
| Arcanum | `Tools/jjk/jja_arcanum.sh` | Install/uninstall |

### Constraints

- URL-safe base64 for Favors (simple bash math + printf, no base64 utility)
- No Unicode in filenames (₣ is display notation only)
- `--sort-keys` for all JSON writes (stable diffs)
- Paddock stays markdown (human-editable prose)
- Scripts abstract all git complexity from Claude
- Backward compatible migration path from current heat files

**Claude Code Integration**
- **Commands** (`jjc_*.md` in `.claude/commands/`): Manually invoked via `/jjc-*`, explicit trigger
- **Skills** (`jjl_*.md` in `.claude/skills/`): Auto-invoked by Claude based on context matching (future)
- Current JJ uses commands exclusively; skills (`jjl_`) reserved for future auto-apply workflows
- Arcanum generates jjc_* files via per-command emitter functions (each emitter knows its tabtarget)

### Foundation Design Decisions

**Dirty-Worktree Guards Policy**

| Command | Policy | Push | Rationale |
|---------|--------|------|-----------|
| muster | any | — | read-only |
| saddle | any | — | read-only |
| rein | any | — | read-only |
| slate | any | — | JSON-only |
| reslate | any | — | JSON-only |
| rail | any | — | JSON-only |
| tally | any | — | JSON-only |
| nominate | any | — | JSON + markdown only |
| chalk | any | — | empty commit (background OK) |
| notch | dirty expected | required (sync) | commits + pushes; won't lose work |
| wrap | clean required | required (sync) | checkpoint; must verify success |
| retire | clean required | required (sync) | archival must be safe |

**Trophy Extraction Spec**

File `jjy_HH_YYMMDD-YYMMDD_silks.md` with sections: header (favor, silks, duration, pace counts), paddock (full text), paces table, steeplechase (from git log). Sources: studbook, paddock file, git history.

**Pace Emplacement API**

Decision: **Append-only**
- `/jjc-pace-slate "Display"` appends to pace array, auto-assigns next ID
- Use `/jjc-pace-rail 001 003 002` to reorder if needed
- Simpler than positional insertion; covers all use cases

## Done

- **Foundation design decisions** — Added worktree guards, wrap flow, trophy spec, emplacement policy to paddock
- **BUK infrastructure for JJK** — Renamed workbench to arcanum, created BUK-style workbench/launcher, renamed tabtargets to jja-*
- **jju-skeleton-construction** — Created BCG-compliant jju_utility.sh with 14 stubs, jju_cli.sh, routing in workbench, help tabtarget
- **paddock-dedup** — Condensed paddock 444→363 lines, removed redundant API docs, kept design rationale
- **jjc-interface-design** — Resolved skill/command distinction (jjl_ future), fixed jjk prefix collision, confirmed emitter-per-command pattern, synced arcanum jjz_scar
- **Implement favor encoding in jju_utility.sh** — Added zjju_favor_encode/decode with charset helpers, jjt_testbench.sh with test suite, launcher and tabtarget
- **studbook-schema-design** — Added BCG-compliant schema validation gate, read/write functions, empty studbook, 9 test cases
- **implement-studbook-operations** — Added 7 studbook ops (nominate, slate, tally, muster, reslate, rail, retire_extract), test suite with 17 cases
- **implement-steeplechase-operations** — Added jju_chalk/rein/notch functions, test suite with 7 cases, tabtarget
- **Remove saddled from studbook schema** — Removed saddled field from schema, validation, tests; context lives in chat
- **Implement /jjc-heat-saddle** — jju_saddle(), tabtarget, workbench routing with auto-select logic, arcanum emitter update
- **Implement /jjc-pace-wrap** — jju_wrap() with tally+chalk+advance ceremony, BCG compliance, workbench routing, tabtarget
- **Unify favor arguments** — Implemented zjju_favor_normalize() helper, updated jju_saddle/slate/rail/retire_extract to accept ₣HHAAA format, added PPP==AAA semantic check to jju_rein, updated workbench auto-saddle logic, updated test cases and added normalizer tests, updated arcanum docs. Favor/ops test suites pass.
- **Fix steeplechase test failure** — Fixed substring extraction (2:3), extended-regexp bracket escaping, test 5 semantics (AAA=heat-only)
- **Code review by Opus** — Reviewed favor/rein changes for BCG compliance; approved; applied dirname fix to tt/jjt-a.TestAll.sh
- **Implement /jjc-heat-retire** — Added jju_retire() with clean-worktree validation, trophy creation, studbook removal, paddock archival, commit+push; updated arcanum emitter; added tt/jjw-hr.HeatRetire.sh tabtarget
- **Unify route stems to jjw- prefix** — Two-tier policy: jja- (arcanum), jjw- (workflow). Renamed jjk-m→jjw-m, jjk-w→jjw-pw, jjk-h→jjw-i. Updated routing. Documented in README.
- **Update CLAUDE.md vocabulary** — Added 5 concepts to emitter (Favor, Silks, Paddock, Steeplechase, Trophy). Reordered for comprehensibility. Fixed notch description. Removed file locations for cleaner definitions.
- **Remove pace-wrap approval prompt** — Removed approval step from jja-pace-wrap.md, renumbered steps 5-9 → 4-8

## Remaining

- **studbook-v2-schema-validation** — Implement v2 schema validation function. No ops yet, just validation.
  **Schema specification** (canonical reference):
  ```json
  {
    "heats": {
      "₣Kb": {
        "silks": "jj-studbook-redesign",
        "datestamp": "260101",
        "status": "current",
        "order": ["₣KbAAA", "₣KbAAB"],
        "states": {"₣KbAAA": "pending", "₣KbAAB": "complete"},
        "paces": {
          "₣KbAAA": {
            "silks": "vocabulary-cleanup",
            "specs": [
              {"ts": "260105-0900", "text": "Initial spec text..."},
              {"ts": "260105-1400", "text": "Refined spec text..."}
            ]
          }
        }
      }
    },
    "next_heat_seed": "Kc"
  }
  ```
  **Deliverables**: (1) Add `zjju_studbook_validate_v2()` to `Tools/jjk/jju_utility.sh`. (2) Add test suite `jjt_test_studbook_v2_validation` to `Tools/jjk/jjt_testbench.sh`. (3) Add tabtarget `tt/jjt-sv2.TestStudbookV2Validation.sh`.
  **Validation rules**:
  - `heats` object required at root, `next_heat_seed` required (2-char base64)
  - Heat Favor key matches `₣[A-Za-z0-9]{2}`
  - Heat requires: `silks` (non-empty string), `datestamp` (YYMMDD), `status` (`current`|`retired`), `order` (array), `states` (object), `paces` (object)
  - `order`, `states`, `paces` must have identical key sets (may be empty for fresh heat)
  - Pace Favor key matches `₣[A-Za-z0-9]{5}` and starts with parent heat Favor
  - Pace requires: `silks` (non-empty kebab-case), `specs` (non-empty array)
  - Each spec requires: `ts` (YYMMDD-HHMM format), `text` (non-empty string)
  - `states` values must be: `pending`|`current`|`complete`|`abandoned`
  **Test cases**: (1) Valid minimal: one heat, one pace, one spec. (2) Valid multi-heat, multi-pace. (3) Valid heat with zero paces (fresh nominate). (4) Invalid heat Favor format → fatal. (5) Pace Favor doesn't match heat prefix → fatal. (6) Order/states/paces key mismatch → fatal. (7) Empty specs array → fatal. (8) Invalid state value → fatal. (9) Missing required field → fatal. (10) Invalid datestamp format → fatal.
  **Success criteria**: `tt/jjt-sv2.TestStudbookV2Validation.sh` passes all 10 cases.

- **studbook-v2-write-ops** — Migrate write operations to v2 schema. Depends on: studbook-v2-schema-validation.
  **Deliverables**: Add these functions to `Tools/jjk/jju_utility.sh`:
  (1) `zjju_studbook_nominate_v2(silks)` — Create heat with empty paces structure
  (2) `zjju_studbook_slate_v2(heat_favor, silks, spec_text)` — Append pace, auto-generate Favor
  (3) `zjju_studbook_reslate_v2(pace_favor, spec_text)` — Append spec entry (never overwrite)
  (4) `zjju_studbook_rail_v2(heat_favor, order_string)` — Reorder paces
  (5) `zjju_studbook_tally_v2(pace_favor, state)` — Update pace state
  **Function behaviors**:
  - `nominate_v2(silks)`: Generates next heat Favor from `next_heat_seed`, creates heat with `datestamp` from `BUD_NOW_STAMP` (YYMMDD, first 6 chars), `status: "current"`, empty `order`/`states`/`paces`. Increments `next_heat_seed`.
  - `slate_v2(heat_favor, silks, spec_text)`: Generates pace Favor by finding max PPP suffix in heat's `order`, incrementing (charset `A-Za-z0-9`, starting `AAA`, increment rightmost with carry). Appends Favor to `order`, adds entry to `states` (value: `pending`), adds pace object to `paces` with single spec entry timestamped from `BUD_NOW_STAMP` (YYMMDD-HHMM, first 11 chars).
  - `reslate_v2(pace_favor, spec_text)`: Appends new spec entry to pace's `specs` array with auto-timestamp. Never modifies existing specs.
  - `rail_v2(heat_favor, order_string)`: Replaces `order` array with space-separated Favors from `order_string`. Validates all Favors exist in `states`/`paces`.
  - `tally_v2(pace_favor, state)`: Updates `states[pace_favor]` to new state value.
  **Implementation notes**: All writes call `zjju_studbook_validate_v2` before writing. Use `jq --sort-keys` for stable diffs. Steeplechase ops (`chalk`, `rein`, `notch`) are unchanged — they work with git, not studbook JSON.
  **Test cases**: (1) nominate creates valid empty heat with correct datestamp. (2) slate appends first pace with Favor `₣HHAAA`. (3) slate appends second pace with Favor `₣HHAAB`. (4) reslate appends spec, array length increases. (5) reslate twice: spec array has 3 entries. (6) rail reorders without data loss. (7) tally changes state correctly. (8) slate on nonexistent heat → fatal. (9) reslate on nonexistent pace → fatal. (10) rail with invalid Favor → fatal.
  **Success criteria**: All 10 test cases pass.

- **studbook-v2-read-ops** — Migrate read operations and add query helpers. Depends on: studbook-v2-write-ops.
  **Deliverables**: Add these functions to `Tools/jjk/jju_utility.sh`:
  (1) `zjju_studbook_muster_v2()` — List heats with summary info
  (2) `zjju_studbook_retire_extract_v2(heat_favor)` — Extract heat data for trophy
  (3) `zjju_pace_current_spec(pace_favor)` — Return last spec text for pace
  (4) `zjju_heat_current_pace(heat_favor)` — Return first pending/current pace Favor
  **Function behaviors**:
  - `muster_v2()`: Outputs one line per heat: `₣HH  silks  (N paces)`. Sorted by Favor.
  - `retire_extract_v2(heat_favor)`: Returns JSON with `silks`, `datestamp`, and for each pace: Favor, silks, final spec text (last entry only — git has full history).
  - `current_spec(pace_favor)`: Returns `specs[last].text` — the human-readable description shown in listings.
  - `current_pace(heat_favor)`: Iterates `order` array, returns first Favor where `states[favor]` is `pending` or `current`. Returns empty string if all complete/abandoned.
  **Test cases**: (1) muster with zero heats → empty output. (2) muster with multiple heats shows correct counts. (3) retire_extract produces valid JSON with final specs. (4) current_spec returns last text entry. (5) current_spec after reslate returns new text. (6) current_pace returns first pending. (7) current_pace skips complete, returns next pending. (8) current_pace with all complete → empty string. (9) current_spec on nonexistent pace → fatal.
  **Success criteria**: All 9 test cases pass.

- **studbook-v2-workflow-ops** — Migrate saddle/wrap/retire to v2 schema. Depends on: studbook-v2-read-ops.
  **Deliverables**: Add these functions to `Tools/jjk/jju_utility.sh`:
  (1) `jju_saddle_v2(heat_favor)` — Mount heat, show paddock and paces
  (2) `jju_wrap_v2(pace_favor)` — Complete pace ceremony
  (3) `jju_retire_v2(heat_favor)` — Archive heat to trophy
  **Saddle output format**:
  ```
  === Heat: {silks} ===

  {paddock file content from .claude/jjm/jjp_{HH}.md}

  === Paces ===
  [{state}] ₣HHPPP {silks}: {last spec text, truncated to 60 chars}...
  [{state}] ₣HHPPP {silks}: {last spec text}

  === Recent Steeplechase ===
  {last 3 chalk entries from git log}
  ```
  **Function behaviors**:
  - `saddle_v2(heat_favor)`: Reads studbook, reads paddock file, calls `jju_rein` for recent entries. If `heat_favor` is empty/omitted and exactly one heat exists, auto-selects it.
  - `wrap_v2(pace_favor)`: (1) Validate clean worktree via `git status --porcelain`. (2) Call `tally_v2(pace_favor, "complete")`. (3) Call `jju_chalk` with WRAP emblem. (4) Call `current_pace` and display next pace if any.
  - `retire_v2(heat_favor)`: (1) Validate clean worktree. (2) Call `retire_extract_v2` for trophy data. (3) Write trophy to `.claude/jjm/retired/jjy_{HH}_{datestamp}_{silks}.md`. (4) Remove heat from studbook. (5) Move paddock to retired dir. (6) Commit and push.
  **Test approach**: Manual integration test — create test heat via nominate, add paces via slate, run saddle/wrap cycle, verify state transitions.
  **Success criteria**: Saddle displays correct format; wrap advances state and chalks; retire creates trophy and cleans up.

- **studbook-v2-code-cleanup** — Remove v2 suffixes and delete v1 functions. Depends on: studbook-v2-workflow-ops.
  **Deliverables**:
  (1) Remove `_v2` suffixes — these become the only implementation
  (2) Delete v1 functions that are fully replaced
  (3) Implement paddock location: `.claude/jjm/jjp_{HH}.md` (no `current/` subdir)
  **Suffix removal**: Rename all v2 functions to canonical names:
  - `zjju_studbook_validate_v2` → `zjju_studbook_validate`
  - `zjju_studbook_nominate_v2` → `zjju_studbook_nominate`
  - `jju_saddle_v2` → `jju_saddle`
  - (and all other `_v2` functions)
  Delete the original v1 implementations.
  **File location changes**:
  | Old (v1) | New (v2) |
  |----------|----------|
  | `.claude/jjm/current/jjh_*.md` | `.claude/jjm/jjp_{HH}.md` |
  | `.claude/jjm/current/jjc_*.md` | (removed — command context in paddock) |
  **Note**: Arcanum emitter updates are DEFERRED to after modularization, so they reference final names (jjbo_*, jjbg_*).
  **Success criteria**: All v2 functions work without `_v2` suffixes; v1 code deleted; paddock at correct path.

- **jju-modularization-review** — Review and finalize jju→jjb* modularization plan after v2 implementation stabilizes.
  **Context**: `jju_utility.sh` splits into three modules preparing for future Rust (jjr) replacement:
  | Module | Prefix | Purpose | Side Effects |
  |--------|--------|---------|--------------|
  | `jjbd_database.sh` | `zjjbd_`/`jjbd_` | Pure data transforms (stdin→stdout) | **None** — future jjr replacement target |
  | `jjbg_git.sh` | `zjjbg_`/`jjbg_` | Git operations (chalk, rein, notch) + future locking | Git repo state |
  | `jjbo_orchestration.sh` | `zjjbo_`/`jjbo_` | File I/O, composition, user output | Files, stdout |
  **Key design principles**:
  - `jjbd` functions have NO file I/O, NO git, NO buc_say — pure data in, data out
  - `jjbg` consolidates all git operations including future `git update-ref` locking
  - `jjbo` is the composition layer — calls jjbd for transforms, jjbg for git, handles file paths and user feedback
  - `jjw` (workbench CLI) remains thin dispatch, unchanged
  **Deliverables**: (1) Read current `jju_utility.sh` to inventory all functions. (2) Produce confirmed allocation table: function → destination module. (3) Flag any functions with ambiguous placement. (4) Update File Prefix Conventions table in paddock with jjbd/jjbg/jjbo.
  **Success criteria**: Clear allocation of every jju function to exactly one destination module.

- **jju-modularization-execute** — Split `jju_utility.sh` into `jjbd_database.sh`, `jjbg_git.sh`, `jjbo_orchestration.sh`. Depends on: jju-modularization-review.
  **Deliverables**:
  (1) Create `Tools/jjk/jjbd_database.sh` — move favor functions, studbook validation, heat_seed_next; rename prefixes zjju_→zjjbd_
  (2) Create `Tools/jjk/jjbg_git.sh` — move chalk/rein/notch; rename prefixes jju_→jjbg_
  (3) Create `Tools/jjk/jjbo_orchestration.sh` — move everything else; rename prefixes zjju_→zjjbo_ and jju_→jjbo_
  (4) Update internal function calls in each file to use new prefixes
  (5) Update `jju_cli.sh` to source all three modules instead of jju_utility.sh
  (6) Run test suite to verify behavior unchanged
  (7) Delete `jju_utility.sh`
  **Constants allocation**: ZJJU_FAVOR_CHARSET → jjbd, ZJJU_STUDBOOK_FILE → jjbo.
  **Implementation notes**: This is primarily mechanical move-and-rename. The jjbd module should match future jjr command surface (favor encode/decode, studbook validate/transforms). Locking stubs (zjjbg_lock_acquire/release) can be no-ops initially.
  **Success criteria**: All existing test suites pass; `jju_utility.sh` deleted; three new modules sourced correctly.

- **jj-arcanum-finalize** — Update arcanum emitters with final modularized names. Depends on: jju-modularization-execute.
  **Deliverables**:
  (1) Update `Tools/jjk/jja_arcanum.sh` emitters to reference modularized function names
  (2) Update CLAUDE.md Job Jockey section with v2 file locations and new module structure
  **Arcanum emitter changes**:
  - `/jja-heat-saddle`: References `jjbo_saddle`
  - `/jja-pace-new`: Documents that `jjbo_slate` takes `(heat_favor, silks, spec_text)`, mention `jjbo_reslate` for refinement
  - `/jja-pace-wrap`: References `jjbo_wrap`
  - `/jja-heat-retire`: References `jjbo_retire`
  - Vocabulary: Update to mention append-only specs, single `silks` identifier (no `display`), modular structure (jjbd/jjbg/jjbo)
  **CLAUDE.md updates**:
  - File prefix table: Add jjbd_, jjbg_, jjbo_ entries
  - Module descriptions: jjbd (data), jjbg (git), jjbo (orchestration)
  **Success criteria**: Fresh `jja_arcanum.sh install` produces correct CLAUDE.md with modularized names; all commands work correctly.
  **Post-completion**: Retire this heat, then run fresh install to activate v2 with modular structure.

- **Vocabulary cleanup** — Phase transformation analysis, term releveling, scar naming reconsideration. Single pass on all vocabulary decisions.

- **Documentation** — Update JJK README: VOK prefix conventions, future directions reflecting what was built.

- **Test full workflow** — After retiring this heat, run fresh `jja_arcanum.sh install` to activate v2, then create a new test heat and run through full lifecycle: nominate → saddle → slate → chalk → wrap → retire. Verify all v2 behaviors work correctly with the new schema.

## Steeplechase

---
### 2026-01-01 - Paddock Refined

**Clarifications added after deep review**:
- Chalk emblems are freeform (APPROACH, WRAP, BLOCKED, NOTE as patterns)
- Saddle output format specified (full paddock, current/remaining paces, 2-3 steeple entries)
- `/jjc-pace-rein` takes heat Favor (full) or pace Favor (filtered)
- `/jjc-heat-nominate` added for heat creation
- ₣ included in JSON for distinctive grep; stripped only for filenames
- Studbook stays lean: `next_heat_seed` tracks allocation, heats removed on retirement
- Trophy is self-contained with full Favor→silks mapping
- jjz_ prefix for scar (declined itch, pending reconsideration)

**New paces added**: `/jjc-heat-nominate`, jju_notch function, trophy extraction doc, wrap advancement design, scar reconsideration

---
### 2026-01-01 - Heat Created

**Context**: Spun off from `b251231-jj-retro-cloud-bashize` after design discussion clarified a larger architectural change than originally scoped.

**Key decisions made**:
- Studbook (JSON) + Paddock (markdown) + Git steeple architecture
- Favor as 5-digit base64 identifier
- Bash scripts as Claude's only interface
- Full vocabulary: muster, saddle, chalk, rein, slate, rail, tally, wrap, retire, notch

**This heat executes under current JJK** while building its replacement.

---
### 2026-01-04 10:35 - implement-jjc-heat-saddle - WRAP
**Outcome**: jju_saddle(), tabtarget, workbench routing with auto-select logic, arcanum emitter update

---
