# Steeplechase: JJ Studbook Redesign

---
### 2026-01-01 16:00 - foundation-design-decisions - APPROACH
**Proposed approach**:
- Dirty-worktree guards: Define policy table (clean vs dirty allowed per command)
- Wrap advancement flow: Specify tally→chalk→advance→display sequence
- Trophy extraction spec: Define file structure and content sources
- Pace emplacement API: Decide append-only vs positional insertion
---
### 2026-01-03 10:30 - foundation-design-decisions - APPROACH
**Proposed approach**:
- Complete dirty-worktree guards policy table for each command (which require clean vs allow dirty)
- Specify wrap advancement sequence: tally→chalk→advance→display with exact studbook mutations
- Define trophy file structure with content sources (studbook extract, paddock, git history)
- Decide pace emplacement: append-only (simpler) vs positional insertion (more flexible)
- Document all decisions in paddock before moving to BUK infrastructure pace
---
### 2026-01-03 10:45 - foundation-design-decisions - DONE
**Decisions documented**:
- Dirty-worktree guards: most commands allow any; notch expects dirty; retire requires clean
- Wrap flow: tally→chalk→advance→display with studbook mutations specified
- Trophy: `jjy_HH_YYMMDD-YYMMDD_silks.md` with paddock, paces table, steeplechase
- Emplacement: append-only (use rail for reordering)
---
### 2026-01-03 11:00 - foundation-design-decisions - REFINEMENT
**Push requirements added** (don't lose work):
- notch: commits + pushes (synchronous)
- wrap: requires clean worktree + push before marking complete (synchronous)
- retire: requires clean + pushed (synchronous)
- chalk: empty commit only, background OK (no push needed)
---
### 2026-01-03 11:10 - foundation-design-decisions - WRAP
**Outcome**: Added worktree guards, wrap flow, trophy spec, emplacement policy to paddock
---
### 2026-01-03 11:20 - buk-infrastructure-for-jjk - APPROACH
**Proposed approach**:
- Rename `Tools/jjk/jjw_workbench.sh` → `Tools/jjk/jja_arcanum.sh` (preserve install/uninstall/check)
- Create new BUK-style `Tools/jjk/jjw_workbench.sh` with `jjw_route()` case routing
- Create `.buk/launcher.jjw_workbench.sh` following buw launcher pattern
- Create `tt/jja-i.Install.sh` and `tt/jja-c.Check.sh` tabtargets (arcanum prefix)
---
### 2026-01-03 11:30 - buk-infrastructure-for-jjk - WRAP
**Outcome**: Renamed workbench to arcanum, created BUK-style workbench/launcher, renamed tabtargets to jja-*
---
### 2026-01-03 11:35 - jju-skeleton-construction - APPROACH
**Proposed approach**:
- Create `Tools/jjk/jju_utility.sh` with 14 function stubs using buc_doc_* pattern
- Add `jj-*` command routing in `jjw_workbench.sh` that sources jju_utility.sh
- Create `tt/jjk-h.Help.sh` tabtarget for doc mode
- Verify skeleton compiles and doc mode works
---
### 2026-01-03 11:50 - jju-skeleton-construction - WRAP
**Outcome**: Created BCG-compliant jju_utility.sh with 14 stubs, jju_cli.sh, routing in workbench, help tabtarget
---
### 2026-01-04 07:54 - paddock-dedup - WRAP
**Outcome**: Condensed paddock 444→363 lines, removed redundant API docs, kept design rationale
---
### 2026-01-04 08:15 - jjc-interface-design - APPROACH
**Proposed approach**:
- Pattern: Slash commands invoke tabtargets → launcher → workbench → jju_utility functions
- Hybrid model: Commands trigger bash scripts for data, Claude interprets output for prose
- Output convention: jju_* functions emit structured data (JSON or tab-delimited), Claude formats for user
- Blocking by default (saddle, wrap, retire need results); background option for notch
- Document mapping table: jjc_* → tt/jjk-*.sh → jju_* function
---
### 2026-01-04 08:45 - jjc-interface-design - WRAP
**Outcome**: Resolved skill/command distinction (jjl_ future), fixed jjk prefix collision, confirmed emitter-per-command pattern, synced arcanum jjz_scar
---
### 2026-01-04 08:50 - implement-favor-encoding - APPROACH
**Proposed approach**:
- Define URL-safe base64 character set (A-Za-z0-9-_) as lookup table
- Implement jju_favor_encode: heat (0-4095) + pace (0-262143) → 5-char string
- Implement jju_favor_decode: 5-char string → heat + pace numbers
- Add validation (range checks, character validation)
- Test with known values to verify round-trip
- BCG compliant: bash 3.2, no command substitution, temp files for pipelines, buc_doc_* annotations
---
### 2026-01-04 09:15 - implement-favor-encoding - WRAP
**Outcome**: Added zjju_favor_encode/decode with charset helpers, jjt_testbench.sh with test suite, launcher and tabtarget
---
### 2026-01-04 09:20 - studbook-schema-design - WRAP
**Outcome**: Added BCG-compliant schema validation gate, read/write functions, empty studbook, 9 test cases
---
### 2026-01-04 09:26 - implement-studbook-operations - APPROACH
**Proposed approach**:
- Start with `jju_nominate` - creates a heat (foundation for testing others)
- Then `jju_muster` - lists heats (simple read-only, verifies studbook reads work)
- Then `jju_slate`/`jju_tally` - add paces and set state (core mutations)
- Finally `jju_reslate`, `jju_rail`, `jju_retire_extract` (less critical paths)
- Each function: read studbook → jq transform → validate → write studbook
- BCG compliant: temp files, exit status checks, no command substitution
- Add test cases to jjt_testbench.sh as we go
---
### 2026-01-04 09:49 - implement-studbook-operations - WRAP
**Outcome**: Added 7 studbook ops (nominate, slate, tally, muster, reslate, rail, retire_extract), test suite with 17 cases
---
### 2026-01-04 10:00 - remove-saddled-from-studbook - APPROACH
**Proposed approach**:
- Remove `saddled` field from studbook schema (context lives in chat, not disk)
- Update `zjju_studbook_validate` to remove saddled validation checks
- Update `jjs_studbook.json` to remove saddled field
- Update test cases: remove saddled from valid JSON fixture, delete bad-saddled test case
- Update paddock: schema example and Fresh Session Handling section (saddle outputs context for Claude)
---
### 2026-01-04 10:15 - implement-steeplechase-operations - WRAP
**Outcome**: Added jju_chalk/rein/notch functions, test suite with 7 cases, tabtarget
---
