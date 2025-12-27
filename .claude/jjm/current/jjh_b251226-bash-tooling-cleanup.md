# Heat: Bash Tooling Cleanup Batch

## Context

Non-blocking bash script and tooling improvements discovered during cloud build revival and manual debugging. These are simple fixes and modernizations that don't block the critical path but improve code quality, maintainability, and usability when addressed.

This heat operates as an open-ended cleanup queue: paces are added as issues surface during development, and the heat completes when the queue is empty.

### Known Issues

1. **Unbound variable in rbgm_ManualProcedures.sh:102** - Error: `2: unbound variable` when running `tt/rbw-PE.PayorEstablishment.sh`
2. **Missing documentation for payor establish process** - Locate or integrate README explaining the payor establishment workflow
3. **Replace makefile rbw with rbk_coordinator script style** - Modernize tool invocation to use rbk_coordinator pattern (rbw_workbench.sh preferred in future)

## Done

(none yet)

## Current

(waiting to start)

## Remaining

1. **Fix unbound variable error in rbgm_ManualProcedures.sh:102** — Running `tt/rbw-PE.PayorEstablishment.sh` produces error: `/Users/bhyslop/projects/brm_recipebottle/Tools/rbw/rbgm_ManualProcedures.sh: line 102: 2: unbound variable`. Investigate and fix.

2. **Locate/integrate README for payor establishment process** — Documentation exists in `lenses/rbw-RBSPE-payor_establish.adoc` and `lenses/rbw-RBSPR-payor_refresh.adoc` (AsciiDoc format), but no user-friendly README. Consider creating helper guide or linking to existing docs from `Tools/rbw/`.

3. **Replace makefile rbw with rbk_coordinator script style** — Modernize invocation pattern from makefile-based to rbk_coordinator (rbw_workbench.sh preferred in future).

4. **Clarify payor regime file path and documentation** — In payor establishment docs, the payor regime file path is shown as relative (`./rbrp.env`). Should be full path and clearly marked as payor identification (not secrets). File is checked into repo; clarify the distinction between identification and OAuth credentials (which are secrets).

5. **Make link instructions OS-specific in payor establishment docs** — Line 143 in `rbgm_ManualProcedures.sh` shows clickable links with `(often, Ctrl + mouse click)` but macOS requires Command + mouse click. Replace with OS-specific instruction. Also move key explanation to top of payor establishment section and eliminate redundant "Default text is this color" line.

6. **Fix payor regime file path color in payor establishment docs** — The full path `/Users/bhyslop/projects/brm_recipebottle/rbrp.env` should be displayed in cyan (something you might copy) not magenta (website/UI text). Update function call in rbgm_ManualProcedures.sh line 146 from `zrbgm_dm()` to `zrbgm_dc()`.

7. **Untangle path indirections in rbgm_ManualProcedures.sh:61-63** — Lines 61-63 use indirect path resolution (`cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd`) to compute file paths. Related itch: ITCH_LINK_TO_RBL (line 60). Simplify by resolving paths more directly; clarify relationship between ZRBGM_RBRP_FILE and ZRBGM_RBRP_FILE_BASENAME (line 62, newly added).

8. **Define and scope image registry listing operation** — Implement image listing operation for Director role to enumerate available images in the repository. Establish operation name (candidate: rbgx_image_list or similar), define scope, parameters, and integration points in RBAGS specification.
   mode: manual

## Steeplechase

(execution log begins here)
