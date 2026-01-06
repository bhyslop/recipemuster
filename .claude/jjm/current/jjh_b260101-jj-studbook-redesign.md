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

- **Vocabulary cleanup** — Phase transformation analysis, term releveling, scar naming reconsideration. Single pass on all vocabulary decisions.

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
