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
