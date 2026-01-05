# Heat: JJ Studbook Redesign

Redesign Job Jockey around a JSON-based studbook registry, git-based steeplechase, and streamlined bash script APIs that keep implementation details hidden from Claude.

## Paddock

### How to Work This Heat

**IMPORTANT**: This heat executes under the CURRENT JJK installation while building its replacement.

- Use existing `/jja-*` commands (heat-saddle, pace-wrap, notch, etc.)
- The new system becomes active only after migration and arcanum update

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

### Concept Surgery Log

Tracking new/modified terms for continuity during this heat:

**New Terms:**
| Term | Type | Meaning |
|------|------|---------|
| Favor | noun | 5-digit heat+pace identifier (₣HHPPP) |
| Studbook | noun | JSON registry of heats/paces |
| Paddock | noun | Per-heat prose context file (was: section in heat file) |
| Trophy | noun | Retired heat archive (was: retired heat file) |
| Chalk | verb | Write steeplechase entry |
| Rein | verb | Read steeplechase entries |
| Muster | verb | List current heats |
| Slate | verb | Add new pace |
| Reslate | verb | Revise pace description |
| Rail | verb | Reorder paces |
| Tally | verb | Set pace state |
| Nominate | verb | Create new heat |

**Modified Terms:**
| Term | Was | Now |
|------|-----|-----|
| Saddle | Read heat file, pick pace | Read studbook + paddock + steeple, unified context |
| Wrap | Mark pace done in heat file | Mark complete in studbook, auto-chalk, advance |
| Retire | Move heat file to retired/ | Extract from studbook, create trophy |
| Notch | Git commit (unchanged) | Git commit (adding jju script) |

**Preserved Terms:**
- Heat, Pace, Itch, Scar (concepts unchanged, storage changed)
- Silks, Steeplechase (concepts unchanged)

**Prefix Assignments (VOK-aligned):**
| Prefix | Purpose | Status |
|--------|---------|--------|
| `jja_` | Arcanum | existing |
| `jjc_` | Command | new usage |
| `jji_` | Itch | existing |
| `jjl_` | skiLl | future reservation |
| `jjp_` | Paddock | new |
| `jjs_` | Studbook | new |
| `jju_` | Utility | new |
| `jjy_` | Trophy | new |
| `jjz_` | Scar | renamed from jjs_ |

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

## Remaining

- **Unify favor arguments to full format** — All jju_* functions accept ₣HHPPP (6-char). Heat-only operations use ₣HHAAA (pace=0 per existing spec at jju_utility.sh:62-63).

  **Normalization helper:**
  ```
  zjju_favor_normalize()
  Input: ₣HH (3 chars) or ₣HHPPP (6 chars)
  Output: ₣HHPPP (always 6 chars)
  - 3-char input → append AAA (pace=0, heat-only reference)
  - 6-char input → pass through unchanged
  - Validates format, dies on invalid
  ```

  **Functions taking heat favor (₣HH) → convert to full:**
  | Function | Line | Param Change | Doc Update |
  |----------|------|--------------|------------|
  | `jju_saddle` | 360 | keep `favor` | ₣HH → ₣HHPPP |
  | `jju_slate` | 590 | `heat` → `favor` | ₣AA → ₣AAAAA |
  | `jju_rail` | 749 | `heat` → `favor` | ₣AA → ₣AAAAA |
  | `jju_retire_extract` | 928 | keep `favor` | ₣HH → ₣HHPPP |

  **jju_rein semantic preservation (line 1087):**
  Currently accepts both ₣HH and ₣HHPPP for different behaviors. With normalization:
  - Check if PPP == AAA → heat-only query (match all paces in heat)
  - Otherwise → pace-specific query (match exact pace)
  Uses existing spec convention; makes caller intent explicit.

  **jjw_workbench.sh (lines 62-96):**
  Auto-saddle extracts ₣XX from muster output. Must expand to ₣XXAAA before calling jju_saddle.

  **jja_arcanum.sh updates:**
  - Line 80: `₣HH` → `₣HHAAA` in usage examples
  - Line 86, 95: Update favor format documentation

  **jjt_testbench.sh test case updates:**
  - Lines 308, 320, 330: `jjt_slate "₣AA"` → `"₣AAAAA"`
  - Lines 354, 366, 370: `jjt_rail "₣AA"` → `"₣AAAAA"`
  - Line 374: `jjt_retire_extract "₣AA"` → `"₣AAAAA"`
  - Add normalizer test cases (₣AA→₣AAAAA, ₣KbAAB→₣KbAAB, invalid inputs)

  **Studbook JSON keys:** No change (remain ₣HH). Normalization is at API boundary, not storage.

  **Implementation notes:**
  - Line numbers above are stale after BCG refactoring. Use grep to find functions.
  - The heat extraction pattern already exists in `jju_wrap`, `jju_reslate`, `jju_tally`:
    ```bash
    local z_heat_digits="${z_favor:1:2}"
    local z_heat_favor="₣${z_heat_digits}"
    ```
    Apply this same pattern to saddle, slate, rail, retire_extract.
  - Normalizer output via stdout; caller uses temp file + read (BCG pattern).
  - For `jju_rein` semantic check after normalize:
    ```bash
    local z_pace_digits="${z_favor:3:3}"
    if test "${z_pace_digits}" = "AAA"; then
      # heat-only: match all paces
    else
      # pace-specific: exact match
    fi
    ```
  - Workbench auto-saddle fix is simple: `z_favor="${z_favor}AAA"` before calling saddle.

  **Success criteria:**
  - All existing tests pass (after updating test inputs)
  - New tests verify normalizer edge cases
  - Heat-only ops work with ₣XXAAA input
  - jju_rein with ₣XXAAA queries all paces; ₣XXPPP (PPP≠AAA) queries specific pace
  - Files touched: jju_utility.sh, jjw_workbench.sh, jja_arcanum.sh, jjt_testbench.sh

- **Implement /jjc-heat-retire** — Create trophy file from studbook extract + paddock + steeple history. Remove heat from studbook. Move paddock to retired/.

- **Migration & arcanum update** — Migrate existing jjh_* heat files to studbook + paddock format. Update `jja_arcanum.sh` for new structure. Revise CLAUDE.md term definitions (all new terms from Concept Surgery Log).

- **Vocabulary cleanup** — Phase transformation analysis, term releveling, scar naming reconsideration. Single pass on all vocabulary decisions.

- **Remove pace-wrap approval prompt** — Edit `.claude/commands/jja-pace-wrap.md` step 4 to proceed directly from summary generation to heat file update, eliminating the "Show proposed summary and ask for approval" confirmation.

- **Documentation** — Update JJK README: VOK prefix conventions, future directions reflecting what was built.

- **Test full workflow** — Create test heat, run through full lifecycle: nominate → saddle → slate → chalk → wrap → retire.

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
