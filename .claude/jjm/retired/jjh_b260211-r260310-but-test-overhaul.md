# Heat Trophy: but-test-overhaul

**Firemark:** ₣Ac
**Created:** 260211
**Retired:** 260310
**Status:** retired

## Paddock

## Provenance

This heat continues BUT test infrastructure work begun in ₣AT (rbw-regime-consolidation). The completed paces in ₣AT established the current framework; this heat redesigns it.

₢ATAAS (exercise-but-suites) in ₣AT is reslated to block on this heat's completion. Its original intent — manually run all suites and amend anything wrong — is captured in ₢AcAAF (exercise-revised-test-suites).

## File Lineage from ₣AT

Renaming history so that stale pace references can be resolved against the current repo.

### Framework files (created by ₢ATAAP but-framework-infrastructure)

| Current file | Origin |
|---|---|
| Tools/buk/buto_operations.sh | Extracted from but_test.sh + buz_zipper.sh dispatch sections |
| Tools/buk/butr_registry.sh | New (suite registration, parallel arrays) |
| Tools/buk/butd_dispatch.sh | New (runner: butd_run_all, butd_run_suite, butd_run_one) |
| Tools/rbw/rbtb_testbench.sh | New (domain testbench, sources framework, routes commands) |
| Tools/rbw/rbtckk_KickTires.sh | New (kick-tires smoke test) |

### Case file conversions

| Current file | Old file | Pace |
|---|---|---|
| Tools/rbw/rbtcal_ArkLifecycle.sh | rbtg_case_ark_lifecycle (in rbtg_testbench.sh) | ₢ATAAO |
| Tools/buk/butcde_DispatchExercise.sh | rbtg_case_dispatch_exercise (in rbtg_testbench.sh) | ₢ATAAN |
| Tools/buk/butctt.TestTarget.sh (tabtarget) | New (trivial test colophon for butcde) | ₢ATAAN |
| Tools/rbw/rbtcns_NsproSecurity.sh | test_nsproto_* (in rbt_testbench.sh) | ₢ATAAM |
| Tools/rbw/rbtcsj_SrjclJupyter.sh | test_srjcl_* (in rbt_testbench.sh) | ₢ATAAM |
| Tools/rbw/rbtcpl_PlumlDiagram.sh | test_pluml_* (in rbt_testbench.sh) | ₢ATAAM |
| Tools/buk/butcvu_XnameValidation.sh | tbvu_suite_xname.sh | ₢ATAAL |
| Tools/rbw/rbtcim_ImageManagement.sh | trbim_suite.sh | ₢ATAAL |

### Function renaming (₢ATAAI refactor-but-test-families)

| Current name | Old name |
|---|---|
| buto_unit_expect_ok | but_expect_ok → but_unit_expect_ok → buto_unit_expect_ok |
| buto_unit_expect_fatal | but_expect_fatal → but_unit_expect_fatal → buto_unit_expect_fatal |
| buto_unit_expect_ok_stdout | but_expect_ok_stdout → but_unit_expect_ok_stdout → buto_unit_expect_ok_stdout |
| buto_tt_expect_ok | New (₢ATAAI), resolves colophon to tt/ file |
| buto_tt_expect_fatal | New (₢ATAAI) |
| buto_launch_expect_ok | New (₢ATAAI), routes through workbench |
| buto_launch_expect_fatal | New (₢ATAAI) |

### Retired files

| Retired file | Replaced by | Pace |
|---|---|---|
| Tools/buk/but_test.sh | buto_operations.sh (shim kept briefly) | ₢ATAAP |
| Tools/rbw/rbtg_testbench.sh | rbtb_testbench.sh + butcde/rbtcal | ₢ATAAN |
| Tools/rbw/rbt_testbench.sh | rbtb_testbench.sh + rbtcns/rbtcsj/rbtcpl | ₢ATAAM |
| Tools/buk/tbvu_suite_xname.sh | butcvu_XnameValidation.sh | ₢ATAAL |
| Tools/rbw/trbim_suite.sh | rbtcim_ImageManagement.sh | ₢ATAAL |

## Current Architecture (as of ₣AT completion)

### Prefix tree

```
but (BUK Test)
├── buto_ — Operations (assertions, invocations, output, dispatch, evidence)
├── butr_ — Registry (suite registration via parallel arrays, glob discovery)
├── butd_ — Dispatch (suite runner: butd_run_all, butd_run_suite, butd_run_one)
├── butc_ — Cases (butcXY_* BUK-level test case files)
└── bute_ — Engine (proposed split target from buto_)

rbt (Recipe Bottle Test)
├── rbtb_ — Bench (testbench, domain helpers, registration, routing)
└── rbtc_ — Cases (rbtcXY_* test case files)
```

### Registration model (current — will be redesigned by ₢AcAAA)

```bash
butr_register "ark-lifecycle"      "rbtcal_" "zrbtb_setup_ark"      "slow"
butr_register "dispatch-exercise"  "butcde_" "zrbtb_setup_dispatch" "fast"
```

Uses glob-based function discovery, single setup function, fast/slow tier.

### Registration model (target — after ₢AcAAA)

```bash
butr_suite_enroll "ark-lifecycle"  "rbtb_assure_cloud_and_git"  "zrbtb_setup_ark"
butr_case_enroll  "ark-lifecycle"  rbtcal_lifecycle
```

BCG enroll/recite/roll pattern. Explicit case registration, two-phase init+setup, no globs, no tiers.

### Known issues driving this heat

1. butr_register packs too much into one call; glob discovery is implicit and fragile
2. buto_operations.sh mixes lightweight assertions with heavyweight engine (dispatch, evidence, step capture)
3. buto_dispatch forces BURD_NO_LOG=1 on inner tabtargets, altering their configured nature
4. No preflight/init mechanism — environment problems (unpushed git, TCC prompts) discovered late
5. No-arg invocation of rbtb-to/rbtb-ts gives error with no help listing available targets
6. BCG kindle constant used as inter-function communication in butd

## Rename Lineage from ₣Ac Paces

### API renames (₢AcAAA redesign-butr-registry)

| Old | New | Notes |
|---|---|---|
| `butr_register()` | `butr_suite_enroll()` + `butr_case_enroll()` | BCG FM-001 migration |
| `butr_get_suites()` | `butr_suites_recite()` | BCG recite pattern |
| `butr_get_glob()` | removed | Glob discovery eliminated |
| `butr_get_setup()` | `butr_setup_recite()` | BCG recite pattern |
| `butr_get_tier()` | removed | Tier concept eliminated |
| `buto_execute()` | removed | Replaced by direct zbuto_case iteration in butd |
| `zbutr_suite_names[]` | `z_butr_name_roll[]` | BCG roll naming |
| `zbutr_suite_globs[]` | removed | |
| `zbutr_suite_setups[]` | `z_butr_setup_roll[]` | BCG roll naming |
| `zbutr_suite_tiers[]` | removed | |

New functions (₢AcAAA): `butr_suite_enroll`, `butr_case_enroll`, `butr_suites_recite`, `butr_init_recite`, `butr_setup_recite`, `butr_cases_recite`, `butr_suite_for_case_recite`

New arrays (₢AcAAA): `z_butr_init_roll[]`, `z_butr_case_fn_roll[]`, `z_butr_case_suite_roll[]`

### File and function renames (₢AcAAJ revert-bud-file-function-renames)

| Old | New | Notes |
|---|---|---|
| `burd_dispatch.sh` | `bud_dispatch.sh` | Revert to correct module prefix |
| `zburd_*()` functions | `zbud_*()` functions | Revert: zburd_show→zbud_show, zburd_die→zbud_die, zburd_setup→zbud_setup, zburd_process_args→zbud_process_args, etc. |

Note: BURD_* variables are NOT renamed — the BUD→BURD variable rename was correct. Only the file and function prefixes revert.

### Variable renames (₢AcAAI fix-buk-bcg-noncompliance)

All buv_validation.sh local variables gain `z_` prefix (e.g., `varname`→`z_varname`, `val`→`z_val`). Internal only — no cross-pace impact.

## Cross-Pace Dependencies and Staleness Warnings

### ₢AcAAE (split-buto-engine) — STALE after ₢AcAAA

AcAAA removes `buto_execute()` which was the main "heavyweight" piece the split was designed around. After AcAAA, remaining heavyweight in buto_operations.sh is: `zbuto_case`, dispatch/evidence. The split boundary changes significantly. **Docket must be revised after AcAAA executes.**

### ₢AcAAB (bcg-kindle-comm-cleanup) — POSSIBLY MOOT after ₢AcAAA

AcAAA rewrites butd_dispatch.sh entirely. The "kindle constant as inter-function communication" in butd_run_one/butd_run_suite may not survive the rewrite. **Re-evaluate after AcAAA executes.**

### ₢AcAAC + ₢AcAAA — shared file (buto_operations.sh)

AcAAA removes buto_execute (lines 312-354). AcAAC modifies buto_dispatch (lines 401-437). Different sections, low conflict risk. AcAAC runs after AcAAA.

### ₢AcAAJ inherits ₢AcAAI patches

AcAAI patches zburd_setup/zburd_process_args in burd_dispatch.sh (subshell→brace-group fixes). AcAAJ later renames file to bud_dispatch.sh and functions zburd_*→zbud_*. Fixes survive; AcAAJ should be aware it inherits patched code.

### ₢AcAAD depends on ₢AcAAA API

AcAAD uses butr_suites_recite/butr_cases_recite (new API from AcAAA) to list available targets. Warrant already accounts for this.

## Downstream Work

- ₢ATAAS in ₣AT blocks on this heat
- split-buto-engine (₢AcAAE) informs where new init/assure functions land — **but scope changes after ₢AcAAA**
- redesign-butr-registry (₢AcAAA) is the architectural keystone

## Paces

### fix-buk-bcg-noncompliance (₢AcAAI) [complete]

**[260211-1548] complete**

Fix BCG noncompliance discovered during document review. All issues are in Tools/buk/ files.

## 1. buv_validation.sh — `test $(command)` silent pass-through

Multiple validators use `test $(echo "$val" | grep -E '^pattern$')` which silently passes when grep matches nothing (zero-arg `test` returns true). Replace with `echo "${z_val}" | grep -qE '^pattern$' || buc_die "..."`.

Affected functions: buv_val_xname (line 136), buv_val_ipv4 (line 277), buv_val_cidr (line 297), buv_val_domain (line 317), buv_val_list_ipv4, buv_val_list_cidr, buv_val_list_domain.

## 2. buv_validation.sh — BCG variable style noncompliance

- Uses `[[ ]]` for inclusion guard (should be `test`)
- Uses `$(dirname ...)` instead of `${BASH_SOURCE[0]%/*}`
- Uses unbraced `$var` in multiple places (should be `"${var}"`)
- Local variables missing `z_` prefix in validator functions

Design consideration: buv predates BCG. Full migration may touch many callers. Scope to the file itself — don't chase callers.

## 3. burd_dispatch.sh — subshell exit bug

Lines 259, 263: `zburd_setup || (echo ... && exit 1)` — subshell exit kills only the subshell, script continues. Fix: `|| { echo ... >&2; exit 1; }`.

## 4. buc_command.sh — `local -i` usage

Line 67: `local -i z_d="${1:-1}"` — silently coerces non-integers to 0. Replace with plain `local` and explicit validation per BCG.

## Scope guard

Modify only files in Tools/buk/. Do not modify BCG-BashConsoleGuide.md. Do not modify files in Tools/rbw/ (legacy, separate migration).

## Design considerations

- buv_validation.sh variable renaming (z_ prefix) may be extensive; use judgment on how deep to go in one pass
- burd_dispatch.sh has many other BCG divergences (its own zburd_die, [[ ]] usage, unbraced vars) — scope this pace to the two subshell exit bugs only, flag remaining as future work

**[260211-1526] bridled**

Fix BCG noncompliance discovered during document review. All issues are in Tools/buk/ files.

## 1. buv_validation.sh — `test $(command)` silent pass-through

Multiple validators use `test $(echo "$val" | grep -E '^pattern$')` which silently passes when grep matches nothing (zero-arg `test` returns true). Replace with `echo "${z_val}" | grep -qE '^pattern$' || buc_die "..."`.

Affected functions: buv_val_xname (line 136), buv_val_ipv4 (line 277), buv_val_cidr (line 297), buv_val_domain (line 317), buv_val_list_ipv4, buv_val_list_cidr, buv_val_list_domain.

## 2. buv_validation.sh — BCG variable style noncompliance

- Uses `[[ ]]` for inclusion guard (should be `test`)
- Uses `$(dirname ...)` instead of `${BASH_SOURCE[0]%/*}`
- Uses unbraced `$var` in multiple places (should be `"${var}"`)
- Local variables missing `z_` prefix in validator functions

Design consideration: buv predates BCG. Full migration may touch many callers. Scope to the file itself — don't chase callers.

## 3. burd_dispatch.sh — subshell exit bug

Lines 259, 263: `zburd_setup || (echo ... && exit 1)` — subshell exit kills only the subshell, script continues. Fix: `|| { echo ... >&2; exit 1; }`.

## 4. buc_command.sh — `local -i` usage

Line 67: `local -i z_d="${1:-1}"` — silently coerces non-integers to 0. Replace with plain `local` and explicit validation per BCG.

## Scope guard

Modify only files in Tools/buk/. Do not modify BCG-BashConsoleGuide.md. Do not modify files in Tools/rbw/ (legacy, separate migration).

## Design considerations

- buv_validation.sh variable renaming (z_ prefix) may be extensive; use judgment on how deep to go in one pass
- burd_dispatch.sh has many other BCG divergences (its own zburd_die, [[ ]] usage, unbraced vars) — scope this pace to the two subshell exit bugs only, flag remaining as future work

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/buv_validation.sh, Tools/buk/burd_dispatch.sh, Tools/buk/buc_command.sh (3 files) | Steps: 1. Read BCG-BashConsoleGuide.md for correct patterns especially test-vs-bracket, variable naming, grep -q pattern, brace-group error blocks, and local -i prohibition 2. In buv_validation.sh fix all test-dollar-command-grep patterns to use grep -qE with || buc_die preserving the existing error messages -- affects buv_val_xname buv_val_gname buv_val_fqin buv_val_ipv4 buv_val_cidr buv_val_domain buv_val_list_ipv4 buv_val_list_cidr buv_val_list_domain 3. In buv_validation.sh fix inclusion guard from double-bracket to test, fix dirname command substitution to parameter expansion, add z_ prefix to all local variables in all functions, quote and brace all variable expansions per BCG 4. In burd_dispatch.sh fix exactly two subshell exit bugs in zburd_main: zburd_setup and zburd_process_args lines using parentheses-echo-exit to brace-group pattern -- do not modify any other lines in this file 5. In buc_command.sh replace local -i with plain local in zbuc_make_tag -- the value comes from BASH_SOURCE indexing so arithmetic validation is not needed, just remove the -i flag 6. Run bash -n on all 3 files | Verify: bash -n Tools/buk/buv_validation.sh && bash -n Tools/buk/burd_dispatch.sh && bash -n Tools/buk/buc_command.sh

**[260211-1516] rough**

Fix BCG noncompliance discovered during document review. All issues are in Tools/buk/ files.

## 1. buv_validation.sh — `test $(command)` silent pass-through

Multiple validators use `test $(echo "$val" | grep -E '^pattern$')` which silently passes when grep matches nothing (zero-arg `test` returns true). Replace with `echo "${z_val}" | grep -qE '^pattern$' || buc_die "..."`.

Affected functions: buv_val_xname (line 136), buv_val_ipv4 (line 277), buv_val_cidr (line 297), buv_val_domain (line 317), buv_val_list_ipv4, buv_val_list_cidr, buv_val_list_domain.

## 2. buv_validation.sh — BCG variable style noncompliance

- Uses `[[ ]]` for inclusion guard (should be `test`)
- Uses `$(dirname ...)` instead of `${BASH_SOURCE[0]%/*}`
- Uses unbraced `$var` in multiple places (should be `"${var}"`)
- Local variables missing `z_` prefix in validator functions

Design consideration: buv predates BCG. Full migration may touch many callers. Scope to the file itself — don't chase callers.

## 3. burd_dispatch.sh — subshell exit bug

Lines 259, 263: `zburd_setup || (echo ... && exit 1)` — subshell exit kills only the subshell, script continues. Fix: `|| { echo ... >&2; exit 1; }`.

## 4. buc_command.sh — `local -i` usage

Line 67: `local -i z_d="${1:-1}"` — silently coerces non-integers to 0. Replace with plain `local` and explicit validation per BCG.

## Scope guard

Modify only files in Tools/buk/. Do not modify BCG-BashConsoleGuide.md. Do not modify files in Tools/rbw/ (legacy, separate migration).

## Design considerations

- buv_validation.sh variable renaming (z_ prefix) may be extensive; use judgment on how deep to go in one pass
- burd_dispatch.sh has many other BCG divergences (its own zburd_die, [[ ]] usage, unbraced vars) — scope this pace to the two subshell exit bugs only, flag remaining as future work

### bcg-shell-trap-guidance (₢AcAAH) [complete]

**[260211-1511] complete**

Apply 9 targeted improvements to BCG-BashConsoleGuide.md addressing silent failure risks, compatibility corrections, and safety rules identified during document review.

## Changes

### 1. Anti-pattern: `test $(command)` empty expansion (new entry near line 630)

`test` with zero arguments returns true (0). When `$(command)` expands to empty string, `test $(command)` silently succeeds. Add explicit warning and recommend exit-status predicates (`grep -q`, `case`). Show the "0-args `test` is true" landmine.

```bash
# ❌ Silent pass when grep matches nothing
test $(echo "${z_val}" | grep -E '^pattern$')

# ✅ Exit-status predicate
echo "${z_val}" | grep -qE '^pattern$' || buc_die "Invalid format"
```

### 2. Anti-pattern: subshell exit vs brace-group (new entry)

`(exit 1)` kills the subshell, not the calling script. Add warning: error blocks must use `{ ...; }` not `( ... )` when intending to exit or return.

```bash
# ❌ exit kills subshell only — script continues
some_cmd || (echo "ERROR" >&2 && exit 1)

# ✅ Brace-group stays in same process
some_cmd || { echo "ERROR" >&2; exit 1; }
```

### 3. Expand "Loops and set -e" → "set -e is not sufficient" (rename/expand lines 665-678)

State the general POSIX rule: `set -e` is suppressed inside `if`, `while`, `||`, `&&` test expressions, and this propagates through the entire call tree of the tested command.

**New rule: Only `_predicate` functions may appear in `if`/`while` conditions.** All other functions must be invoked as simple commands with explicit `|| buc_die` / `|| return`. This rule completely prevents the suppression hazard by leveraging BCG's existing special function types.

```bash
# ✅ Predicate in conditional — designed for this, never dies, status only
if z«prefix»_ready_predicate; then ...

# ✅ Regular function — explicit error handling, not inside conditional
some_function || buc_die "..."

# ❌ Regular function in conditional — set -e suppressed for entire call tree
if some_function; then ...
```

Keep the existing loop-specific guidance and function-type error suffix table (that table is excellent). Add note: BCG's `|| buc_die` discipline is the mitigation — it works precisely because it doesn't rely on set -e. The `|| z_status=$?` capture pattern intentionally relies on this suppression.

### 4. Array iteration under `set -u` (near Bash Compatibility, line 840 area)

Under `set -u`, `"${array[@]}"` on an empty array triggers "unbound variable" in bash 3.2.

Primary pattern: index iteration `${!arr[@]}` — works safely on empty arrays, no guard needed.

Acceptable alternative: guard value iteration with `(( ${#arr[@]} ))` before expanding.

```bash
# ✅ Safe: index iteration works on empty arrays under set -u
for z_i in "${!z_«prefix»_name_roll[@]}"; do
  echo "${z_«prefix»_name_roll[$z_i]}" || buc_die "..."
done

# ❌ Unsafe: value iteration fails on empty arrays under set -u in bash 3.2
for z_val in "${z_«prefix»_name_roll[@]}"; do ...
```

### 5. Remove here-string misclassification (line 663)

Remove `z_var=<<<${z_input}` from "Bash 4+ Features" section. This is invalid syntax and here-strings are bash 3.0+, not 4+. Line 350 already correctly allows single-line here-strings. No contradiction remains after removal.

### 6. Forbid `local -i` (Variable Handling, near line 255)

`local -i` silently coerces non-integer values to 0 — violates no-silent-failures principle. Forbid it. Use plain `local` with explicit validation.

```bash
# ❌ local -i silently coerces "abc" to 0
local -i z_count="${z_input}"

# ✅ Plain local with explicit validation
local z_count="${z_input}"
test "${z_count}" -ge 0 2>/dev/null || buc_die "z_count must be integer, got: ${z_count}"
```

### 7. Legacy code note (near Core Philosophy or end of document)

One sentence: some older code (particularly rbw modules) predates current BCG form; treat as migration targets, not exemplars.

### 8. eval policy (new subsection near Sourcing Rules)

`eval` forbidden except for validated variable-name dereference. Require `^[A-Za-z_][A-Za-z0-9_]*$` regex validation before any eval. Prefer `${!name}` indirect expansion where sufficient (works in bash 3.2 for reading). `eval` only when assigning to an indirect variable.

```bash
# ✅ Prefer indirect expansion for reading
local z_val="${!z_varname}"

# ✅ eval with validated name for assignment
echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' \
  || buc_die "Invalid variable name: ${z_varname}"
eval "${z_varname}=\${z_new_value}"

# ❌ Unvalidated eval — injection risk
eval "local z_val=\${${z_varname}:-}"
```

### 9. Temp file lifecycle (near Temp Files section, line 333 area)

Temp files under BURD_TEMP_DIR are preserved after execution for forensic debugging. Never delete temp files in module code — their persistence is intentional. Cleanup is handled by infrastructure outside BCG's scope.

## Scope guard

Modify ONLY Tools/buk/vov_veiled/BCG-BashConsoleGuide.md. Do NOT modify any .sh files. Code fixes to buv_validation.sh, burd_dispatch.sh, buc_command.sh are separate work.

## Examples

All examples must use «prefix» synthetic notation, consistent with existing BCG style. Do not use real project names.

**[260211-1436] rough**

Apply 9 targeted improvements to BCG-BashConsoleGuide.md addressing silent failure risks, compatibility corrections, and safety rules identified during document review.

## Changes

### 1. Anti-pattern: `test $(command)` empty expansion (new entry near line 630)

`test` with zero arguments returns true (0). When `$(command)` expands to empty string, `test $(command)` silently succeeds. Add explicit warning and recommend exit-status predicates (`grep -q`, `case`). Show the "0-args `test` is true" landmine.

```bash
# ❌ Silent pass when grep matches nothing
test $(echo "${z_val}" | grep -E '^pattern$')

# ✅ Exit-status predicate
echo "${z_val}" | grep -qE '^pattern$' || buc_die "Invalid format"
```

### 2. Anti-pattern: subshell exit vs brace-group (new entry)

`(exit 1)` kills the subshell, not the calling script. Add warning: error blocks must use `{ ...; }` not `( ... )` when intending to exit or return.

```bash
# ❌ exit kills subshell only — script continues
some_cmd || (echo "ERROR" >&2 && exit 1)

# ✅ Brace-group stays in same process
some_cmd || { echo "ERROR" >&2; exit 1; }
```

### 3. Expand "Loops and set -e" → "set -e is not sufficient" (rename/expand lines 665-678)

State the general POSIX rule: `set -e` is suppressed inside `if`, `while`, `||`, `&&` test expressions, and this propagates through the entire call tree of the tested command.

**New rule: Only `_predicate` functions may appear in `if`/`while` conditions.** All other functions must be invoked as simple commands with explicit `|| buc_die` / `|| return`. This rule completely prevents the suppression hazard by leveraging BCG's existing special function types.

```bash
# ✅ Predicate in conditional — designed for this, never dies, status only
if z«prefix»_ready_predicate; then ...

# ✅ Regular function — explicit error handling, not inside conditional
some_function || buc_die "..."

# ❌ Regular function in conditional — set -e suppressed for entire call tree
if some_function; then ...
```

Keep the existing loop-specific guidance and function-type error suffix table (that table is excellent). Add note: BCG's `|| buc_die` discipline is the mitigation — it works precisely because it doesn't rely on set -e. The `|| z_status=$?` capture pattern intentionally relies on this suppression.

### 4. Array iteration under `set -u` (near Bash Compatibility, line 840 area)

Under `set -u`, `"${array[@]}"` on an empty array triggers "unbound variable" in bash 3.2.

Primary pattern: index iteration `${!arr[@]}` — works safely on empty arrays, no guard needed.

Acceptable alternative: guard value iteration with `(( ${#arr[@]} ))` before expanding.

```bash
# ✅ Safe: index iteration works on empty arrays under set -u
for z_i in "${!z_«prefix»_name_roll[@]}"; do
  echo "${z_«prefix»_name_roll[$z_i]}" || buc_die "..."
done

# ❌ Unsafe: value iteration fails on empty arrays under set -u in bash 3.2
for z_val in "${z_«prefix»_name_roll[@]}"; do ...
```

### 5. Remove here-string misclassification (line 663)

Remove `z_var=<<<${z_input}` from "Bash 4+ Features" section. This is invalid syntax and here-strings are bash 3.0+, not 4+. Line 350 already correctly allows single-line here-strings. No contradiction remains after removal.

### 6. Forbid `local -i` (Variable Handling, near line 255)

`local -i` silently coerces non-integer values to 0 — violates no-silent-failures principle. Forbid it. Use plain `local` with explicit validation.

```bash
# ❌ local -i silently coerces "abc" to 0
local -i z_count="${z_input}"

# ✅ Plain local with explicit validation
local z_count="${z_input}"
test "${z_count}" -ge 0 2>/dev/null || buc_die "z_count must be integer, got: ${z_count}"
```

### 7. Legacy code note (near Core Philosophy or end of document)

One sentence: some older code (particularly rbw modules) predates current BCG form; treat as migration targets, not exemplars.

### 8. eval policy (new subsection near Sourcing Rules)

`eval` forbidden except for validated variable-name dereference. Require `^[A-Za-z_][A-Za-z0-9_]*$` regex validation before any eval. Prefer `${!name}` indirect expansion where sufficient (works in bash 3.2 for reading). `eval` only when assigning to an indirect variable.

```bash
# ✅ Prefer indirect expansion for reading
local z_val="${!z_varname}"

# ✅ eval with validated name for assignment
echo "${z_varname}" | grep -qE '^[A-Za-z_][A-Za-z0-9_]*$' \
  || buc_die "Invalid variable name: ${z_varname}"
eval "${z_varname}=\${z_new_value}"

# ❌ Unvalidated eval — injection risk
eval "local z_val=\${${z_varname}:-}"
```

### 9. Temp file lifecycle (near Temp Files section, line 333 area)

Temp files under BURD_TEMP_DIR are preserved after execution for forensic debugging. Never delete temp files in module code — their persistence is intentional. Cleanup is handled by infrastructure outside BCG's scope.

## Scope guard

Modify ONLY Tools/buk/vov_veiled/BCG-BashConsoleGuide.md. Do NOT modify any .sh files. Code fixes to buv_validation.sh, burd_dispatch.sh, buc_command.sh are separate work.

## Examples

All examples must use «prefix» synthetic notation, consistent with existing BCG style. Do not use real project names.

### clarify-bcg-register-pattern (₢AcAAG) [complete]

**[260211-1205] complete**

Expand BCG's _register section and related constraints, informed by two concrete usages: buz_register (zipper colophon routing) and butr_register (test suite registry). Rename the pattern family to roll/enroll/recite.

## Summary of changes to BCG-BashConsoleGuide.md

### 1. Rename _register pattern family

Replace `_register` special function type with `_enroll`. Update the special function table (line 348 area), the example/contract section (lines 417-451), and all references throughout.

New naming family:

| Concept | BCG term | Variable/naming pattern |
|---------|----------|------------------------|
| Parallel array storage | **roll** | `z«PREFIX»_«name»_roll` |
| Populate function (kindle-only) | **enroll** | `[z]«prefix»_[«scope»_]enroll` |
| Read-only query function | **recite** | `[z]«prefix»_«what»_recite` |
| Return channel from enroll | — | `z_«funcname»_«retval»` |

### 2. Retire z1z_ return convention

Replace all BCG references to `z1z_«prefix»_«term»` with the new convention: `z_«funcname»_«retval»`. The function name is embedded verbatim in the variable name for traceability. Example: `buz_enroll` sets `z_buz_enroll_colophon`.

Remove `z1z_` from:
- Special function table (line 348)
- Register contract section (lines 417-451)
- Naming convention table (line 480)
- Checklist items (line 718)
- Quick reference decision matrix (line 675)

### 3. Expand enroll section with parallel-array registry pattern

Add structural guidance covering:

a) **Parallel-array registry pattern**: kindle initializes N empty `_roll` arrays of same length; enroll validates and appends atomically to all rolls; recite functions scan by key roll.

b) **Registration-time validation**: invariants checked at enroll time, not at recite or execution time. Use synthetic examples (NOT project code like buz_register).

c) **Two-level registries**: when entities have parent-child relationships, use two flat registries with a foreign-key column rather than per-parent dynamic arrays. Avoids eval, stays bash 3.2 safe.

d) **Kindle/sentinel/enroll triple**: enroll functions may ONLY be called within kindle. Rolls are initialized in kindle, populated by enroll in kindle, then immutable. Recite functions are the only access path after kindle completes.

e) **Arrays-only constraint**: enroll functions work exclusively with rolls (parallel arrays). No other shared state mutation.

### 4. Add module constant exclusivity rule

Add explicit prohibition: All `Z«PREFIX»_*` internal constants and `«PREFIX»_*` public exports must be defined exclusively within the kindle function. No other function may assign to these variables.

Specific additions:
- Explicit rule statement near the kindle boilerplate table (line 78 area)
- Anti-pattern example showing violation and correction
- Strengthen checklist line 706 to say "exclusively" (matching line 707)
- Add new checklist item: "No Z«PREFIX»_* or «PREFIX»_* assignments outside kindle function"

### 5. Add recite function contract

Add a new special function type alongside predicate and capture:

- Suffix: `_recite`
- Purpose: Read-only query of roll arrays populated by enroll
- Contract: never mutates rolls, returns via echo (like capture) or sets local variables
- May use buc_die if key not found
- Pattern: `«prefix»_«what»_recite` (public) or `z«prefix»_«what»_recite` (internal)

### 6. Add "Fading Memory" section at end of BCG

New section documenting superseded conventions for migration guidance. Structured as individual entries, each containing:

- **Identifier**: Short reference name (e.g., FM-001)
- **Superseded pattern**: What the old convention looked like
- **Recognition**: How to identify legacy usage in code
- **Replacement**: What it should become
- **Known legacy sites**: Where old pattern still lives in the codebase
- **Migration notes**: Any ordering constraints or gotchas

First entry — FM-001: _register to _enroll migration:
- Old: `_register` suffix, `z1z_«prefix»_«term»` return vars, unstructured array names, no `_recite` accessor convention
- Recognition: function named `*_register`; variables starting `z1z_`; parallel arrays without `_roll` suffix; accessor functions named `*_get_*` without `_recite` suffix
- Replacement: `_enroll` suffix; `z_«funcname»_«retval»` returns; `_roll` array suffix; `_recite` accessor suffix
- Known legacy: `buz_register` in Tools/buk/buz_zipper.sh (will persist beyond this heat); `butr_register` in Tools/buk/butr_registry.sh (rewritten in ₢AcAAA this heat)
- Migration: When touching a file that uses old pattern, transform to new. Do not bulk-rename across codebase — migrate opportunistically per-pace.

### 7. Use synthetic examples only

All examples in the enroll/roll/recite sections must use «prefix» template notation, NOT real project names like buz or butr. This prevents context confusion between BCG guidance and project-specific code.

## Scope guard

Modify ONLY Tools/buk/vov_veiled/BCG-BashConsoleGuide.md. Do NOT modify any .sh files. The butr_registry.sh redesign and buz_zipper.sh migration are separate paces.

**[260211-1009] rough**

Expand BCG's _register section and related constraints, informed by two concrete usages: buz_register (zipper colophon routing) and butr_register (test suite registry). Rename the pattern family to roll/enroll/recite.

## Summary of changes to BCG-BashConsoleGuide.md

### 1. Rename _register pattern family

Replace `_register` special function type with `_enroll`. Update the special function table (line 348 area), the example/contract section (lines 417-451), and all references throughout.

New naming family:

| Concept | BCG term | Variable/naming pattern |
|---------|----------|------------------------|
| Parallel array storage | **roll** | `z«PREFIX»_«name»_roll` |
| Populate function (kindle-only) | **enroll** | `[z]«prefix»_[«scope»_]enroll` |
| Read-only query function | **recite** | `[z]«prefix»_«what»_recite` |
| Return channel from enroll | — | `z_«funcname»_«retval»` |

### 2. Retire z1z_ return convention

Replace all BCG references to `z1z_«prefix»_«term»` with the new convention: `z_«funcname»_«retval»`. The function name is embedded verbatim in the variable name for traceability. Example: `buz_enroll` sets `z_buz_enroll_colophon`.

Remove `z1z_` from:
- Special function table (line 348)
- Register contract section (lines 417-451)
- Naming convention table (line 480)
- Checklist items (line 718)
- Quick reference decision matrix (line 675)

### 3. Expand enroll section with parallel-array registry pattern

Add structural guidance covering:

a) **Parallel-array registry pattern**: kindle initializes N empty `_roll` arrays of same length; enroll validates and appends atomically to all rolls; recite functions scan by key roll.

b) **Registration-time validation**: invariants checked at enroll time, not at recite or execution time. Use synthetic examples (NOT project code like buz_register).

c) **Two-level registries**: when entities have parent-child relationships, use two flat registries with a foreign-key column rather than per-parent dynamic arrays. Avoids eval, stays bash 3.2 safe.

d) **Kindle/sentinel/enroll triple**: enroll functions may ONLY be called within kindle. Rolls are initialized in kindle, populated by enroll in kindle, then immutable. Recite functions are the only access path after kindle completes.

e) **Arrays-only constraint**: enroll functions work exclusively with rolls (parallel arrays). No other shared state mutation.

### 4. Add module constant exclusivity rule

Add explicit prohibition: All `Z«PREFIX»_*` internal constants and `«PREFIX»_*` public exports must be defined exclusively within the kindle function. No other function may assign to these variables.

Specific additions:
- Explicit rule statement near the kindle boilerplate table (line 78 area)
- Anti-pattern example showing violation and correction
- Strengthen checklist line 706 to say "exclusively" (matching line 707)
- Add new checklist item: "No Z«PREFIX»_* or «PREFIX»_* assignments outside kindle function"

### 5. Add recite function contract

Add a new special function type alongside predicate and capture:

- Suffix: `_recite`
- Purpose: Read-only query of roll arrays populated by enroll
- Contract: never mutates rolls, returns via echo (like capture) or sets local variables
- May use buc_die if key not found
- Pattern: `«prefix»_«what»_recite` (public) or `z«prefix»_«what»_recite` (internal)

### 6. Add "Fading Memory" section at end of BCG

New section documenting superseded conventions for migration guidance. Structured as individual entries, each containing:

- **Identifier**: Short reference name (e.g., FM-001)
- **Superseded pattern**: What the old convention looked like
- **Recognition**: How to identify legacy usage in code
- **Replacement**: What it should become
- **Known legacy sites**: Where old pattern still lives in the codebase
- **Migration notes**: Any ordering constraints or gotchas

First entry — FM-001: _register to _enroll migration:
- Old: `_register` suffix, `z1z_«prefix»_«term»` return vars, unstructured array names, no `_recite` accessor convention
- Recognition: function named `*_register`; variables starting `z1z_`; parallel arrays without `_roll` suffix; accessor functions named `*_get_*` without `_recite` suffix
- Replacement: `_enroll` suffix; `z_«funcname»_«retval»` returns; `_roll` array suffix; `_recite` accessor suffix
- Known legacy: `buz_register` in Tools/buk/buz_zipper.sh (will persist beyond this heat); `butr_register` in Tools/buk/butr_registry.sh (rewritten in ₢AcAAA this heat)
- Migration: When touching a file that uses old pattern, transform to new. Do not bulk-rename across codebase — migrate opportunistically per-pace.

### 7. Use synthetic examples only

All examples in the enroll/roll/recite sections must use «prefix» template notation, NOT real project names like buz or butr. This prevents context confusion between BCG guidance and project-specific code.

## Scope guard

Modify ONLY Tools/buk/vov_veiled/BCG-BashConsoleGuide.md. Do NOT modify any .sh files. The butr_registry.sh redesign and buz_zipper.sh migration are separate paces.

**[260211-0945] rough**

Expand BCG section on _register functions with structural guidance drawn from the two concrete usages: buz_register (zipper colophon routing) and butr_register (test suite registry).

## Current BCG coverage

BCG defines _register at line 348 (special function table) and lines 417-451 (example + contract). The coverage establishes the subshell-avoidance rationale and z1z_ return convention but does not describe the parallel-array registry pattern that both usages share.

## What to add

Expand the _register section in BCG to cover:

1. **Parallel-array registry pattern**: kindle initializes N empty arrays of same length; register validates + appends atomically to all arrays; query scans by key array.

2. **Registration-time validation**: invariants checked at register time, not query/execution time. Examples: buz_register validates tabtarget exists; butr_register (new) should validate declare -F on case functions.

3. **Two-level registries**: when entities have parent-child relationships (suite→case), use two flat registries with a foreign-key column rather than per-parent dynamic arrays. Avoids eval and stays bash 3.2 safe.

4. **Kindle/sentinel/register triple**: how _register functions relate to the kindle/sentinel boilerplate — kindle creates arrays, sentinel guards them, register populates them.

## Scope guard

Do NOT redesign butr_registry.sh in this pace. Only modify BCG-BashConsoleGuide.md. The registry redesign uses this guidance but is a separate pace.

### redesign-butr-registry (₢AcAAA) [complete]

**[260211-1601] complete**

Redesign butr_registry.sh with explicit case registration and two-phase suite initialization, following BCG enroll/recite/roll patterns per FM-001.

## New registration API — BCG enroll pattern

butr_suite_enroll registers a suite with name, init function, and setup function. Called within consuming module kindle.
butr_case_enroll registers a single case function for a suite. Fatal at registration time if declare -F fails. No glob discovery.

Example usage in rbtb_kindle:
  butr_suite_enroll "ark-lifecycle"  "rbtb_assure_cloud_and_git"  "zrbtb_setup_ark"
  butr_case_enroll  "ark-lifecycle"  rbtcal_lifecycle
  butr_suite_enroll "kick-tires"    ""  "zrbtb_setup_kick"
  butr_case_enroll  "kick-tires"    rbtckk_smoke_test

## Storage — BCG two-level registry with foreign key

Suite rolls: z_butr_name_roll, z_butr_init_roll, z_butr_setup_roll
Case rolls: z_butr_case_fn_roll, z_butr_case_suite_roll (index into suite rolls)

## Recite functions

butr_suites_recite — list all suite names on stdout
butr_init_recite SUITE — get init function for named suite
butr_setup_recite SUITE — get setup function for named suite
butr_cases_recite SUITE — list case function names for suite
butr_suite_for_case_recite FN — find owning suite name for a case function

## Init semantics — resolved

Init is a regular function. Can buc_die for real infrastructure errors. Returns non-zero for "not ready." Dispatch captures init status with "|| z_status=$?" pattern. Non-zero skips the suite. Single init function per suite. Empty string means no init check — always ready.

No special exit codes. Just exit 1. Error printouts explain why.

## Dispatch changes

butd_run_suite: runs init via status capture, skips suite on non-zero with buc_warn. Then runs setup. Then iterates cases via butr_cases_recite, calling zbuto_case per function.
butd_run_all: continues past failed/inconclusive suites, reports summary at end. Exit 1 if any failed. No tier filtering.
butd_run_one: finds owning suite via butr_suite_for_case_recite, runs init+setup, then the single case.

## What dies

- Glob-based function discovery — replaced by explicit butr_case_enroll
- Tier concept — fast/slow filtering removed entirely
- buto_execute — replaced by direct case roll iteration in butd
- butr_register — replaced by butr_suite_enroll + butr_case_enroll
- All butr_get_* query functions — replaced by butr_*_recite

## What is preserved

- zbuto_case — runs single test case in subshell, called directly by butd
- buto_dispatch/evidence infrastructure — untouched, separate concern
- rbtb_route and rbtb_main — routing unchanged, just registration changes

**[260211-1521] bridled**

Redesign butr_registry.sh with explicit case registration and two-phase suite initialization, following BCG enroll/recite/roll patterns per FM-001.

## New registration API — BCG enroll pattern

butr_suite_enroll registers a suite with name, init function, and setup function. Called within consuming module kindle.
butr_case_enroll registers a single case function for a suite. Fatal at registration time if declare -F fails. No glob discovery.

Example usage in rbtb_kindle:
  butr_suite_enroll "ark-lifecycle"  "rbtb_assure_cloud_and_git"  "zrbtb_setup_ark"
  butr_case_enroll  "ark-lifecycle"  rbtcal_lifecycle
  butr_suite_enroll "kick-tires"    ""  "zrbtb_setup_kick"
  butr_case_enroll  "kick-tires"    rbtckk_smoke_test

## Storage — BCG two-level registry with foreign key

Suite rolls: z_butr_name_roll, z_butr_init_roll, z_butr_setup_roll
Case rolls: z_butr_case_fn_roll, z_butr_case_suite_roll (index into suite rolls)

## Recite functions

butr_suites_recite — list all suite names on stdout
butr_init_recite SUITE — get init function for named suite
butr_setup_recite SUITE — get setup function for named suite
butr_cases_recite SUITE — list case function names for suite
butr_suite_for_case_recite FN — find owning suite name for a case function

## Init semantics — resolved

Init is a regular function. Can buc_die for real infrastructure errors. Returns non-zero for "not ready." Dispatch captures init status with "|| z_status=$?" pattern. Non-zero skips the suite. Single init function per suite. Empty string means no init check — always ready.

No special exit codes. Just exit 1. Error printouts explain why.

## Dispatch changes

butd_run_suite: runs init via status capture, skips suite on non-zero with buc_warn. Then runs setup. Then iterates cases via butr_cases_recite, calling zbuto_case per function.
butd_run_all: continues past failed/inconclusive suites, reports summary at end. Exit 1 if any failed. No tier filtering.
butd_run_one: finds owning suite via butr_suite_for_case_recite, runs init+setup, then the single case.

## What dies

- Glob-based function discovery — replaced by explicit butr_case_enroll
- Tier concept — fast/slow filtering removed entirely
- buto_execute — replaced by direct case roll iteration in butd
- butr_register — replaced by butr_suite_enroll + butr_case_enroll
- All butr_get_* query functions — replaced by butr_*_recite

## What is preserved

- zbuto_case — runs single test case in subshell, called directly by butd
- buto_dispatch/evidence infrastructure — untouched, separate concern
- rbtb_route and rbtb_main — routing unchanged, just registration changes

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/butr_registry.sh, Tools/buk/butd_dispatch.sh, Tools/buk/buto_operations.sh, Tools/rbw/rbtb_testbench.sh (4 files) | Steps: 1. Read BCG-BashConsoleGuide.md thoroughly for enroll/recite/roll patterns and FM-001 migration guidance 2. Read all 4 target files plus rbtb test case files to understand current function names for explicit registration 3. Rewrite butr_registry.sh: replace butr_register with butr_suite_enroll and butr_case_enroll using BCG roll naming z_butr_name_roll etc, add butr_suites_recite butr_init_recite butr_setup_recite butr_cases_recite butr_suite_for_case_recite, remove glob and tier arrays, use index iteration for set-u safety 4. Adapt butd_dispatch.sh: replace butr_get_glob and butr_get_setup calls with butr recite functions, replace buto_execute call with direct zbuto_case iteration over butr_cases_recite output, add init phase using z_status capture pattern before setup, remove tier filtering from butd_run_all, add skip count and summary reporting, replace glob-based suite matching in butd_run_one with butr_suite_for_case_recite 5. Remove buto_execute function and its empty-array-check helper from buto_operations.sh, preserve zbuto_case and all other functions 6. Convert rbtb_testbench.sh rbtb_kindle: replace 8 butr_register calls with butr_suite_enroll plus butr_case_enroll per case function, add init functions where needed, enumerate every test case function explicitly by reading each test case source file for function names 7. Verify syntax: bash -n on all 4 modified files | Verify: bash -n Tools/buk/butr_registry.sh && bash -n Tools/buk/butd_dispatch.sh && bash -n Tools/buk/buto_operations.sh && bash -n Tools/rbw/rbtb_testbench.sh

**[260211-1521] rough**

Redesign butr_registry.sh with explicit case registration and two-phase suite initialization, following BCG enroll/recite/roll patterns per FM-001.

## New registration API — BCG enroll pattern

butr_suite_enroll registers a suite with name, init function, and setup function. Called within consuming module kindle.
butr_case_enroll registers a single case function for a suite. Fatal at registration time if declare -F fails. No glob discovery.

Example usage in rbtb_kindle:
  butr_suite_enroll "ark-lifecycle"  "rbtb_assure_cloud_and_git"  "zrbtb_setup_ark"
  butr_case_enroll  "ark-lifecycle"  rbtcal_lifecycle
  butr_suite_enroll "kick-tires"    ""  "zrbtb_setup_kick"
  butr_case_enroll  "kick-tires"    rbtckk_smoke_test

## Storage — BCG two-level registry with foreign key

Suite rolls: z_butr_name_roll, z_butr_init_roll, z_butr_setup_roll
Case rolls: z_butr_case_fn_roll, z_butr_case_suite_roll (index into suite rolls)

## Recite functions

butr_suites_recite — list all suite names on stdout
butr_init_recite SUITE — get init function for named suite
butr_setup_recite SUITE — get setup function for named suite
butr_cases_recite SUITE — list case function names for suite
butr_suite_for_case_recite FN — find owning suite name for a case function

## Init semantics — resolved

Init is a regular function. Can buc_die for real infrastructure errors. Returns non-zero for "not ready." Dispatch captures init status with "|| z_status=$?" pattern. Non-zero skips the suite. Single init function per suite. Empty string means no init check — always ready.

No special exit codes. Just exit 1. Error printouts explain why.

## Dispatch changes

butd_run_suite: runs init via status capture, skips suite on non-zero with buc_warn. Then runs setup. Then iterates cases via butr_cases_recite, calling zbuto_case per function.
butd_run_all: continues past failed/inconclusive suites, reports summary at end. Exit 1 if any failed. No tier filtering.
butd_run_one: finds owning suite via butr_suite_for_case_recite, runs init+setup, then the single case.

## What dies

- Glob-based function discovery — replaced by explicit butr_case_enroll
- Tier concept — fast/slow filtering removed entirely
- buto_execute — replaced by direct case roll iteration in butd
- butr_register — replaced by butr_suite_enroll + butr_case_enroll
- All butr_get_* query functions — replaced by butr_*_recite

## What is preserved

- zbuto_case — runs single test case in subshell, called directly by butd
- buto_dispatch/evidence infrastructure — untouched, separate concern
- rbtb_route and rbtb_main — routing unchanged, just registration changes

**[260211-0859] rough**

Drafted from ₢ATAAW in ₣AT.

Redesign butr_registry.sh with explicit case registration and two-phase suite initialization.

## New registration API

```bash
z_init_cloudbuild="rbtb_assure_cloud_and_git"
z_setup_ark="zrbtb_setup_ark"

z_suite="ark-lifecycle"
butr_suite  ${z_suite}  ${z_init_cloudbuild}  ${z_setup_ark}
butr_case   ${z_suite}  rbtcal_lifecycle
```

## Semantics

- butr_suite: declares suite with init function and setup function
- butr_case: registers a single explicitly-named function. Fatal error at registration time if declare -F does not find it (no glob discovery)
- Init: runs first. Failure = inconclusive (non-zero exit, semantically "environment not ready," not "test broken"). Example: unpushed git commits blocking cloud build, container registry login requiring interactive TCC prompt
- Setup: runs after init passes. Failure = fatal
- Cases run after both succeed

## Execution modes

- Run all cases in a suite: rbtb-ts ark-lifecycle
- Run one case: rbtb-to rbtcal_lifecycle (engine finds owning suite, runs init+setup, then one case)

## What dies

- Glob-based function discovery (replaced by explicit butr_case registration)
- Tier concept (fast/slow filtering removed for now)

## Design work needed

- Exact inconclusive reporting format and exit code convention
- How butd_dispatch.sh adapts to new registry API
- Whether init functions compose (suite declares multiple init traits) or remain single-function
- Integration with buto_execute flow

Requires collaborative design — multiple valid approaches for inconclusive semantics and init composition.

**[260211-0849] rough**

Redesign butr_registry.sh with explicit case registration and two-phase suite initialization.

## New registration API

```bash
z_init_cloudbuild="rbtb_assure_cloud_and_git"
z_setup_ark="zrbtb_setup_ark"

z_suite="ark-lifecycle"
butr_suite  ${z_suite}  ${z_init_cloudbuild}  ${z_setup_ark}
butr_case   ${z_suite}  rbtcal_lifecycle
```

## Semantics

- butr_suite: declares suite with init function and setup function
- butr_case: registers a single explicitly-named function. Fatal error at registration time if declare -F does not find it (no glob discovery)
- Init: runs first. Failure = inconclusive (non-zero exit, semantically "environment not ready," not "test broken"). Example: unpushed git commits blocking cloud build, container registry login requiring interactive TCC prompt
- Setup: runs after init passes. Failure = fatal
- Cases run after both succeed

## Execution modes

- Run all cases in a suite: rbtb-ts ark-lifecycle
- Run one case: rbtb-to rbtcal_lifecycle (engine finds owning suite, runs init+setup, then one case)

## What dies

- Glob-based function discovery (replaced by explicit butr_case registration)
- Tier concept (fast/slow filtering removed for now)

## Design work needed

- Exact inconclusive reporting format and exit code convention
- How butd_dispatch.sh adapts to new registry API
- Whether init functions compose (suite declares multiple init traits) or remain single-function
- Integration with buto_execute flow

Requires collaborative design — multiple valid approaches for inconclusive semantics and init composition.

**[260211-0848] rough**

Design a pre-flight check mechanism for the BUT test framework. Before running any test cases, the engine would run an 'init' (name TBD — perhaps 'preflight', 'ready', 'gate', 'muster') function for each test module/suite that checks whether the environment is fit for execution. If the check fails, the outcome is 'inconclusive' (not 'failed') — still unix non-zero, but semantically distinct from a test failure. This addresses cases like the ark-lifecycle suite which spends 15s discovering that the local repo has unpushed commits — a precondition that could be caught before any test runs. Design questions: Is this per-module or per-suite? Does it compose with the existing buto_execute flow? What's the right name? Is 'inconclusive' a new result category alongside pass/fail? Requires collaborative design — the idea may not be congruous with the current model.

**[260211-0755] rough**

Design a pre-flight check mechanism for the BUT test framework. Before running any test cases, the engine would run an 'init' (name TBD — perhaps 'preflight', 'ready', 'gate', 'muster') function for each test module/suite that checks whether the environment is fit for execution. If the check fails, the outcome is 'inconclusive' (not 'failed') — still unix non-zero, but semantically distinct from a test failure. This addresses cases like the ark-lifecycle suite which spends 15s discovering that the local repo has unpushed commits — a precondition that could be caught before any test runs. Design questions: Is this per-module or per-suite? Does it compose with the existing buto_execute flow? What's the right name? Is 'inconclusive' a new result category alongside pass/fail? Requires collaborative design — the idea may not be congruous with the current model.

### fix-subdispatch-logging (₢AcAAC) [complete]

**[260211-1641] complete**

Drafted from ₢ATAAX in ₣AT.

Two changes to buto_dispatch subdispatch behavior:

1. Remove BURD_NO_LOG=1 from buto_dispatch's inner tabtarget invocation (buto_operations.sh ~line 425). Inner tabtargets must run with their natural logging behavior — the test framework should not suppress logging. BURV overrides (BURV_OUTPUT_ROOT_DIR, BURV_TEMP_ROOT_DIR) remain as-is for evidence isolation.

2. Surface inner artifact paths in the outer transcript. After each subdispatch step completes, record the inner process's transcript, log files, and output dir paths into the outer rbtb-c* transcript. On step failure, unconditionally display these paths to console regardless of BUT_VERBOSE, so the operator can immediately inspect what happened in the inner process.

**[260211-1529] bridled**

Drafted from ₢ATAAX in ₣AT.

Two changes to buto_dispatch subdispatch behavior:

1. Remove BURD_NO_LOG=1 from buto_dispatch's inner tabtarget invocation (buto_operations.sh ~line 425). Inner tabtargets must run with their natural logging behavior — the test framework should not suppress logging. BURV overrides (BURV_OUTPUT_ROOT_DIR, BURV_TEMP_ROOT_DIR) remain as-is for evidence isolation.

2. Surface inner artifact paths in the outer transcript. After each subdispatch step completes, record the inner process's transcript, log files, and output dir paths into the outer rbtb-c* transcript. On step failure, unconditionally display these paths to console regardless of BUT_VERBOSE, so the operator can immediately inspect what happened in the inner process.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/buto_operations.sh (1 file) | Steps: 1. Read BCG-BashConsoleGuide.md for buc_log_args and output function patterns 2. In buto_dispatch function, delete the BURD_NO_LOG=1 line from the tabtarget invocation block, keeping BURV_OUTPUT_ROOT_DIR and BURV_TEMP_ROOT_DIR env overrides intact 3. After the existing "Step N exit status" buc_log_args line, add buc_log_args calls to record inner process artifact paths: z_burv_output for inner BURD output, z_burv_temp for inner BURD temp, z_evidence_dir for harvested evidence 4. Add conditional block after evidence harvest: if z_exit_status is non-zero, use buto_section to unconditionally display the inner artifact paths to console so operator can immediately inspect what happened in the failing inner process 5. Run bash -n on the file | Verify: bash -n Tools/buk/buto_operations.sh

**[260211-0859] rough**

Drafted from ₢ATAAX in ₣AT.

Two changes to buto_dispatch subdispatch behavior:

1. Remove BURD_NO_LOG=1 from buto_dispatch's inner tabtarget invocation (buto_operations.sh ~line 425). Inner tabtargets must run with their natural logging behavior — the test framework should not suppress logging. BURV overrides (BURV_OUTPUT_ROOT_DIR, BURV_TEMP_ROOT_DIR) remain as-is for evidence isolation.

2. Surface inner artifact paths in the outer transcript. After each subdispatch step completes, record the inner process's transcript, log files, and output dir paths into the outer rbtb-c* transcript. On step failure, unconditionally display these paths to console regardless of BUT_VERBOSE, so the operator can immediately inspect what happened in the inner process.

**[260211-0813] rough**

Two changes to buto_dispatch subdispatch behavior:

1. Remove BURD_NO_LOG=1 from buto_dispatch's inner tabtarget invocation (buto_operations.sh ~line 425). Inner tabtargets must run with their natural logging behavior — the test framework should not suppress logging. BURV overrides (BURV_OUTPUT_ROOT_DIR, BURV_TEMP_ROOT_DIR) remain as-is for evidence isolation.

2. Surface inner artifact paths in the outer transcript. After each subdispatch step completes, record the inner process's transcript, log files, and output dir paths into the outer rbtb-c* transcript. On step failure, unconditionally display these paths to console regardless of BUT_VERBOSE, so the operator can immediately inspect what happened in the inner process.

### split-buto-engine (₢AcAAE) [complete]

**[260211-1648] complete**

Split buto_operations.sh into buto_operations.sh (test-case API) and bute_engine.sh (execution machinery). All moved functions rename from buto_/zbuto_ to bute_/zbute_. Drop inclusion guards. bute_engine.sh sources buto_operations.sh.

## Boundary

**Stay in buto_operations.sh** (lines 1-280):
- Color codes, generic renderer (zbuto_render_lines)
- Output functions: buto_section, buto_info, buto_trace, buto_fatal, buto_fatal_on_error, buto_fatal_on_success, buto_success
- zbuto_invoke (captures stdout/stderr/status)
- buto_unit_* assertions (expect_ok, expect_ok_stdout, expect_fatal)
- buto_tt_* assertions (tabtarget file invocation)
- buto_launch_* assertions (workbench dispatch)

**Move to bute_engine.sh** (lines 282-418), renaming buto_→bute_, zbuto_→zbute_:
- zbute_case (test case runner in subshell)
- bute_init_dispatch / zbute_dispatch_sentinel
- zbute_resolve_tabtarget_capture (non-fatal resolution)
- bute_init_evidence
- bute_dispatch (BURV-isolated tabtarget invocation with evidence harvest)
- bute_last_step_capture, bute_get_step_exit_capture, bute_get_step_output_capture

## BURV bridge: BUTE_BURV_ROOT

Add per-invocation BURV isolation to zbuto_invoke so test cases get BURV automatically without calling engine functions.

Engine side (zbute_case): set `BUTE_BURV_ROOT="${z_case_temp_dir}/burv"` before running the test function.

Operations side (zbuto_invoke): if BUTE_BURV_ROOT is set, call zbuto_next_invoke_capture to find next unused invoke number, create per-invocation BURV dirs, set BURV_OUTPUT_ROOT_DIR and BURV_TEMP_ROOT_DIR for the child process.

New stateless capture function in buto_operations.sh:
```
zbuto_next_invoke_capture() — starts at 10000, scans ${BUTE_BURV_ROOT}/invoke-NNNNN dirs, returns next unused number. No variable memory.
```

Directory structure per invocation:
```
${BUTE_BURV_ROOT}/invoke-10000/output/
${BUTE_BURV_ROOT}/invoke-10000/temp/
${BUTE_BURV_ROOT}/invoke-10001/output/
...
```

## Rewrite rbtcal_ArkLifecycle.sh

Replace all buto_dispatch + step_capture patterns with buto_tt_expect_ok. Each 4-line dispatch-check block becomes:
```
buto_tt_expect_ok "${RBZ_LIST_IMAGES}"
echo "${ZBUTO_STDOUT}" > "${z_baseline_file}"
```
Steps that don't need stdout become single-line: `buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"`. Remove buto_init_dispatch and buto_init_evidence calls from rbtcal — BURV handled automatically via BUTE_BURV_ROOT.

## Caller updates

| File | Change |
|------|--------|
| butd_dispatch.sh | source bute_engine.sh, zbuto_case→zbute_case, ZBUTO_ROOT_TEMP_DIR→ZBUTE_ROOT_TEMP_DIR |
| but_test.sh | source bute_engine.sh (gets operations transitively), zbut_case wrapper→zbute_case |
| rbtb_testbench.sh | source bute_engine.sh, buto_init_dispatch→bute_init_dispatch, buto_init_evidence→bute_init_evidence, ZBUTO_ROOT_TEMP_DIR→ZBUTE_ROOT_TEMP_DIR |
| buz_zipper.sh | rename wrappers: buz_dispatch→bute_dispatch, buz_last_step_capture→bute_last_step_capture, etc. Source bute_engine.sh |
| rbtcal_ArkLifecycle.sh | Full rewrite per above — no engine function calls |

## Variable renames

- ZBUTO_ROOT_TEMP_DIR → ZBUTE_ROOT_TEMP_DIR (rbtb_testbench.sh, butd_dispatch.sh, zbute_case)
- ZBUTO_DISPATCH_READY → ZBUTE_DISPATCH_READY
- ZBUTO_EVIDENCE_ROOT → ZBUTE_EVIDENCE_ROOT
- zbuto_step_colophons/exit_status/output_dir arrays → zbute_*

## Inclusion guards

Remove ZBUTO_INCLUDED guard from buto_operations.sh. Do not add guard to bute_engine.sh. Sourcing tree is acyclic: bute_engine.sh → buto_operations.sh, testbenches → bute_engine.sh.

**[260211-1617] bridled**

Split buto_operations.sh into buto_operations.sh (test-case API) and bute_engine.sh (execution machinery). All moved functions rename from buto_/zbuto_ to bute_/zbute_. Drop inclusion guards. bute_engine.sh sources buto_operations.sh.

## Boundary

**Stay in buto_operations.sh** (lines 1-280):
- Color codes, generic renderer (zbuto_render_lines)
- Output functions: buto_section, buto_info, buto_trace, buto_fatal, buto_fatal_on_error, buto_fatal_on_success, buto_success
- zbuto_invoke (captures stdout/stderr/status)
- buto_unit_* assertions (expect_ok, expect_ok_stdout, expect_fatal)
- buto_tt_* assertions (tabtarget file invocation)
- buto_launch_* assertions (workbench dispatch)

**Move to bute_engine.sh** (lines 282-418), renaming buto_→bute_, zbuto_→zbute_:
- zbute_case (test case runner in subshell)
- bute_init_dispatch / zbute_dispatch_sentinel
- zbute_resolve_tabtarget_capture (non-fatal resolution)
- bute_init_evidence
- bute_dispatch (BURV-isolated tabtarget invocation with evidence harvest)
- bute_last_step_capture, bute_get_step_exit_capture, bute_get_step_output_capture

## BURV bridge: BUTE_BURV_ROOT

Add per-invocation BURV isolation to zbuto_invoke so test cases get BURV automatically without calling engine functions.

Engine side (zbute_case): set `BUTE_BURV_ROOT="${z_case_temp_dir}/burv"` before running the test function.

Operations side (zbuto_invoke): if BUTE_BURV_ROOT is set, call zbuto_next_invoke_capture to find next unused invoke number, create per-invocation BURV dirs, set BURV_OUTPUT_ROOT_DIR and BURV_TEMP_ROOT_DIR for the child process.

New stateless capture function in buto_operations.sh:
```
zbuto_next_invoke_capture() — starts at 10000, scans ${BUTE_BURV_ROOT}/invoke-NNNNN dirs, returns next unused number. No variable memory.
```

Directory structure per invocation:
```
${BUTE_BURV_ROOT}/invoke-10000/output/
${BUTE_BURV_ROOT}/invoke-10000/temp/
${BUTE_BURV_ROOT}/invoke-10001/output/
...
```

## Rewrite rbtcal_ArkLifecycle.sh

Replace all buto_dispatch + step_capture patterns with buto_tt_expect_ok. Each 4-line dispatch-check block becomes:
```
buto_tt_expect_ok "${RBZ_LIST_IMAGES}"
echo "${ZBUTO_STDOUT}" > "${z_baseline_file}"
```
Steps that don't need stdout become single-line: `buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"`. Remove buto_init_dispatch and buto_init_evidence calls from rbtcal — BURV handled automatically via BUTE_BURV_ROOT.

## Caller updates

| File | Change |
|------|--------|
| butd_dispatch.sh | source bute_engine.sh, zbuto_case→zbute_case, ZBUTO_ROOT_TEMP_DIR→ZBUTE_ROOT_TEMP_DIR |
| but_test.sh | source bute_engine.sh (gets operations transitively), zbut_case wrapper→zbute_case |
| rbtb_testbench.sh | source bute_engine.sh, buto_init_dispatch→bute_init_dispatch, buto_init_evidence→bute_init_evidence, ZBUTO_ROOT_TEMP_DIR→ZBUTE_ROOT_TEMP_DIR |
| buz_zipper.sh | rename wrappers: buz_dispatch→bute_dispatch, buz_last_step_capture→bute_last_step_capture, etc. Source bute_engine.sh |
| rbtcal_ArkLifecycle.sh | Full rewrite per above — no engine function calls |

## Variable renames

- ZBUTO_ROOT_TEMP_DIR → ZBUTE_ROOT_TEMP_DIR (rbtb_testbench.sh, butd_dispatch.sh, zbute_case)
- ZBUTO_DISPATCH_READY → ZBUTE_DISPATCH_READY
- ZBUTO_EVIDENCE_ROOT → ZBUTE_EVIDENCE_ROOT
- zbuto_step_colophons/exit_status/output_dir arrays → zbute_*

## Inclusion guards

Remove ZBUTO_INCLUDED guard from buto_operations.sh. Do not add guard to bute_engine.sh. Sourcing tree is acyclic: bute_engine.sh → buto_operations.sh, testbenches → bute_engine.sh.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/bute_engine.sh NEW, Tools/buk/buto_operations.sh, Tools/buk/butd_dispatch.sh, Tools/buk/but_test.sh, Tools/rbw/rbtb_testbench.sh, Tools/buk/buz_zipper.sh, Tools/rbw/rbtcal_ArkLifecycle.sh (7 files) | Steps: 1. Read buto_operations.sh fully, identify split boundary at line 280 between buto_launch_expect_fatal and zbuto_case 2. Create bute_engine.sh with moved functions renamed buto_/zbuto_ to bute_/zbute_: zbute_case, bute_init_dispatch, zbute_dispatch_sentinel, zbute_resolve_tabtarget_capture, bute_init_evidence, bute_dispatch, bute_last_step_capture, bute_get_step_exit_capture, bute_get_step_output_capture plus arrays zbute_step_colophons/exit_status/output_dir and variables ZBUTE_DISPATCH_READY, ZBUTE_EVIDENCE_ROOT, ZBUTE_ROOT_TEMP_DIR; bute_engine.sh sources buto_operations.sh at top 3. Trim buto_operations.sh: remove moved functions after line 280, remove ZBUTO_INCLUDED inclusion guard and zbuto_guard_die 4. Add zbuto_next_invoke_capture to buto_operations.sh: stateless function starting at 10000, scans BUTE_BURV_ROOT/invoke-NNNNN dirs for next unused number 5. Enhance zbuto_invoke in buto_operations.sh: if BUTE_BURV_ROOT is set, call zbuto_next_invoke_capture, mkdir invoke-N/output and invoke-N/temp, set BURV_OUTPUT_ROOT_DIR and BURV_TEMP_ROOT_DIR as env vars on the child command 6. In zbute_case in bute_engine.sh, export BUTE_BURV_ROOT pointing to z_case_temp_dir/burv before running test function 7. Update butd_dispatch.sh: source bute_engine.sh instead of buto_operations.sh, rename zbuto_case to zbute_case, ZBUTO_ROOT_TEMP_DIR to ZBUTE_ROOT_TEMP_DIR 8. Update but_test.sh: source bute_engine.sh, rename zbut_case wrapper to call zbute_case 9. Update rbtb_testbench.sh: source bute_engine.sh, rename buto_init_dispatch to bute_init_dispatch, buto_init_evidence to bute_init_evidence, ZBUTO_ROOT_TEMP_DIR to ZBUTE_ROOT_TEMP_DIR 10. Update buz_zipper.sh: source bute_engine.sh, rename all buto_ wrappers to bute_ names 11. Rewrite rbtcal_ArkLifecycle.sh: replace all buto_dispatch plus step_capture patterns with buto_tt_expect_ok, use ZBUTO_STDOUT for stdout capture, remove buto_init_dispatch and buto_init_evidence calls 12. Run bash -n on all 7 files | Verify: bash -n Tools/buk/bute_engine.sh Tools/buk/buto_operations.sh Tools/buk/butd_dispatch.sh Tools/buk/but_test.sh Tools/rbw/rbtb_testbench.sh Tools/buk/buz_zipper.sh Tools/rbw/rbtcal_ArkLifecycle.sh

**[260211-1615] rough**

Split buto_operations.sh into buto_operations.sh (test-case API) and bute_engine.sh (execution machinery). All moved functions rename from buto_/zbuto_ to bute_/zbute_. Drop inclusion guards. bute_engine.sh sources buto_operations.sh.

## Boundary

**Stay in buto_operations.sh** (lines 1-280):
- Color codes, generic renderer (zbuto_render_lines)
- Output functions: buto_section, buto_info, buto_trace, buto_fatal, buto_fatal_on_error, buto_fatal_on_success, buto_success
- zbuto_invoke (captures stdout/stderr/status)
- buto_unit_* assertions (expect_ok, expect_ok_stdout, expect_fatal)
- buto_tt_* assertions (tabtarget file invocation)
- buto_launch_* assertions (workbench dispatch)

**Move to bute_engine.sh** (lines 282-418), renaming buto_→bute_, zbuto_→zbute_:
- zbute_case (test case runner in subshell)
- bute_init_dispatch / zbute_dispatch_sentinel
- zbute_resolve_tabtarget_capture (non-fatal resolution)
- bute_init_evidence
- bute_dispatch (BURV-isolated tabtarget invocation with evidence harvest)
- bute_last_step_capture, bute_get_step_exit_capture, bute_get_step_output_capture

## BURV bridge: BUTE_BURV_ROOT

Add per-invocation BURV isolation to zbuto_invoke so test cases get BURV automatically without calling engine functions.

Engine side (zbute_case): set `BUTE_BURV_ROOT="${z_case_temp_dir}/burv"` before running the test function.

Operations side (zbuto_invoke): if BUTE_BURV_ROOT is set, call zbuto_next_invoke_capture to find next unused invoke number, create per-invocation BURV dirs, set BURV_OUTPUT_ROOT_DIR and BURV_TEMP_ROOT_DIR for the child process.

New stateless capture function in buto_operations.sh:
```
zbuto_next_invoke_capture() — starts at 10000, scans ${BUTE_BURV_ROOT}/invoke-NNNNN dirs, returns next unused number. No variable memory.
```

Directory structure per invocation:
```
${BUTE_BURV_ROOT}/invoke-10000/output/
${BUTE_BURV_ROOT}/invoke-10000/temp/
${BUTE_BURV_ROOT}/invoke-10001/output/
...
```

## Rewrite rbtcal_ArkLifecycle.sh

Replace all buto_dispatch + step_capture patterns with buto_tt_expect_ok. Each 4-line dispatch-check block becomes:
```
buto_tt_expect_ok "${RBZ_LIST_IMAGES}"
echo "${ZBUTO_STDOUT}" > "${z_baseline_file}"
```
Steps that don't need stdout become single-line: `buto_tt_expect_ok "${RBZ_CONJURE_ARK}" "${z_vessel_dir}"`. Remove buto_init_dispatch and buto_init_evidence calls from rbtcal — BURV handled automatically via BUTE_BURV_ROOT.

## Caller updates

| File | Change |
|------|--------|
| butd_dispatch.sh | source bute_engine.sh, zbuto_case→zbute_case, ZBUTO_ROOT_TEMP_DIR→ZBUTE_ROOT_TEMP_DIR |
| but_test.sh | source bute_engine.sh (gets operations transitively), zbut_case wrapper→zbute_case |
| rbtb_testbench.sh | source bute_engine.sh, buto_init_dispatch→bute_init_dispatch, buto_init_evidence→bute_init_evidence, ZBUTO_ROOT_TEMP_DIR→ZBUTE_ROOT_TEMP_DIR |
| buz_zipper.sh | rename wrappers: buz_dispatch→bute_dispatch, buz_last_step_capture→bute_last_step_capture, etc. Source bute_engine.sh |
| rbtcal_ArkLifecycle.sh | Full rewrite per above — no engine function calls |

## Variable renames

- ZBUTO_ROOT_TEMP_DIR → ZBUTE_ROOT_TEMP_DIR (rbtb_testbench.sh, butd_dispatch.sh, zbute_case)
- ZBUTO_DISPATCH_READY → ZBUTE_DISPATCH_READY
- ZBUTO_EVIDENCE_ROOT → ZBUTE_EVIDENCE_ROOT
- zbuto_step_colophons/exit_status/output_dir arrays → zbute_*

## Inclusion guards

Remove ZBUTO_INCLUDED guard from buto_operations.sh. Do not add guard to bute_engine.sh. Sourcing tree is acyclic: bute_engine.sh → buto_operations.sh, testbenches → bute_engine.sh.

**[260211-0859] rough**

Drafted from ₢ATAAT in ₣AT.

Explore splitting buto_operations.sh into two files: buto_operations.sh retains lightweight test-case API (output functions, unit/tt/launch assertions), while bute_engine.sh extracts the heavyweight runner infrastructure (case execution, dispatch, evidence harvest, step capture). bute_engine.sh sources buto_operations.sh — dependency flows engine→operations. Assess the boundary, identify callers on each side, and propose the split.

**[260211-0732] rough**

Explore splitting buto_operations.sh into two files: buto_operations.sh retains lightweight test-case API (output functions, unit/tt/launch assertions), while bute_engine.sh extracts the heavyweight runner infrastructure (case execution, dispatch, evidence harvest, step capture). bute_engine.sh sources buto_operations.sh — dependency flows engine→operations. Assess the boundary, identify callers on each side, and propose the split.

### test-cmd-list-targets (₢AcAAD) [complete]

**[260211-1649] complete**

Drafted from ₢ATAAU in ₣AT.

When tt/rbtb-to.TestOne.sh or tt/rbtb-ts.TestSuite.sh is invoked with no arguments, enhance the error output to list all available targets after the existing error message. Still exit with non-zero status and keep the current ERROR line. For TestOne, list available test function names. For TestSuite, list available suite names. The discovery mechanism should match what these commands already use to resolve their argument.

**[260211-1531] bridled**

Drafted from ₢ATAAU in ₣AT.

When tt/rbtb-to.TestOne.sh or tt/rbtb-ts.TestSuite.sh is invoked with no arguments, enhance the error output to list all available targets after the existing error message. Still exit with non-zero status and keep the current ERROR line. For TestOne, list available test function names. For TestSuite, list available suite names. The discovery mechanism should match what these commands already use to resolve their argument.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/butd_dispatch.sh (1 file) | Steps: 1. Read BCG-BashConsoleGuide.md for output patterns especially buto_fatal buto_info buto_section 2. Read Tools/buk/butr_registry.sh to understand the recite functions for querying suite names and case function names -- this file will have been rewritten by prior pace AcAAA with butr_suites_recite and butr_cases_recite 3. In butd_run_suite, when z_suite is empty, before the existing fatal, output "Available suites:" header via buto_info then list each suite name on its own line by calling butr_suites_recite and formatting via buto_info, then call buto_fatal with the existing error message 4. In butd_run_one, when z_func is empty, before the existing fatal, output "Available test functions:" header via buto_info then iterate all suites via butr_suites_recite, for each suite call butr_cases_recite to get case functions, format each as "  suite-name: func_name" via buto_info, then call buto_fatal with the existing error message 5. Run bash -n | Verify: bash -n Tools/buk/butd_dispatch.sh

**[260211-0859] rough**

Drafted from ₢ATAAU in ₣AT.

When tt/rbtb-to.TestOne.sh or tt/rbtb-ts.TestSuite.sh is invoked with no arguments, enhance the error output to list all available targets after the existing error message. Still exit with non-zero status and keep the current ERROR line. For TestOne, list available test function names. For TestSuite, list available suite names. The discovery mechanism should match what these commands already use to resolve their argument.

**[260211-0741] rough**

When tt/rbtb-to.TestOne.sh or tt/rbtb-ts.TestSuite.sh is invoked with no arguments, enhance the error output to list all available targets after the existing error message. Still exit with non-zero status and keep the current ERROR line. For TestOne, list available test function names. For TestSuite, list available suite names. The discovery mechanism should match what these commands already use to resolve their argument.

### bcg-kindle-comm-cleanup (₢AcAAB) [complete]

**[260211-1651] complete**

Drafted from ₢ATAAV in ₣AT.

BCG cleanup: butd_run_one and butd_run_suite set what appears to be a kindle constant as inter-function communication. This violates BCG conventions where kindle constants should be set once at module initialization, not mutated during execution. Requires collaborative design — not a mechanical fix. Identify the offending variables, trace their usage, and design a proper BCG-compliant communication pattern.

**[260211-0859] rough**

Drafted from ₢ATAAV in ₣AT.

BCG cleanup: butd_run_one and butd_run_suite set what appears to be a kindle constant as inter-function communication. This violates BCG conventions where kindle constants should be set once at module initialization, not mutated during execution. Requires collaborative design — not a mechanical fix. Identify the offending variables, trace their usage, and design a proper BCG-compliant communication pattern.

**[260211-0742] rough**

BCG cleanup: butd_run_one and butd_run_suite set what appears to be a kindle constant as inter-function communication. This violates BCG conventions where kindle constants should be set once at module initialization, not mutated during execution. Requires collaborative design — not a mechanical fix. Identify the offending variables, trace their usage, and design a proper BCG-compliant communication pattern.

### revert-bud-file-function-renames (₢AcAAJ) [complete]

**[260211-1701] complete**

Revert incorrect file and function renames from the BUD_→BURD_ variable rename.

The BUD_→BURD_ variable rename was correct, but the model also incorrectly renamed:
1. File: bud_dispatch.sh → burd_dispatch.sh (WRONG - bud is the file/module prefix)
2. Functions: zbud_* → zburd_* (WRONG - zbud_ is the function prefix for the bud module)

The bud prefix (file/function namespace) and BURD prefix (variable namespace) are distinct per terminal exclusivity.

Fix:
- Rename file back: burd_dispatch.sh → bud_dispatch.sh
- Rename all functions back: zburd_* → zbud_* inside the file (zburd_show, zburd_die, zburd_check_string, zburd_setup, zburd_process_args, zburd_curate_same, zburd_curate_hist, zburd_generate_checksum, zburd_resolve_color, zburd_main, plus local zburd_invocation)
- Update all references in ~14 files: bul_launcher.sh, README.md, CLAUDE.md, 6 tt/ tabtargets, 2 memos, rbga_cli.sh, rbgb_cli.sh, rgbs_cli.sh
- Leave all BURD_* variables untouched (those renames were correct)

**[260211-1654] bridled**

Revert incorrect file and function renames from the BUD_→BURD_ variable rename.

The BUD_→BURD_ variable rename was correct, but the model also incorrectly renamed:
1. File: bud_dispatch.sh → burd_dispatch.sh (WRONG - bud is the file/module prefix)
2. Functions: zbud_* → zburd_* (WRONG - zbud_ is the function prefix for the bud module)

The bud prefix (file/function namespace) and BURD prefix (variable namespace) are distinct per terminal exclusivity.

Fix:
- Rename file back: burd_dispatch.sh → bud_dispatch.sh
- Rename all functions back: zburd_* → zbud_* inside the file (zburd_show, zburd_die, zburd_check_string, zburd_setup, zburd_process_args, zburd_curate_same, zburd_curate_hist, zburd_generate_checksum, zburd_resolve_color, zburd_main, plus local zburd_invocation)
- Update all references in ~14 files: bul_launcher.sh, README.md, CLAUDE.md, 6 tt/ tabtargets, 2 memos, rbga_cli.sh, rbgb_cli.sh, rgbs_cli.sh
- Leave all BURD_* variables untouched (those renames were correct)

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/burd_dispatch.sh RENAME to bud_dispatch.sh, Tools/buk/bul_launcher.sh, Tools/buk/README.md, CLAUDE.md, Memos/memo-20260209-regime-inventory.md, Memos/memo-20260110-acronym-selection-study.md, Tools/buk/vov_veiled/vocbumc_core.md, tt/rbw-l.ListCurrentRegistryImages.sh, tt/rbw-him.HelpImageManagement.sh, tt/rbw-hga.HelpGoogleAdmin.sh, tt/gadcf.LaunchFactoryInContainer.sh, tt/gadi-i.Inspect.sh, tt/rbw-iB.BuildImageRemotely.sh (13 files) | Steps: 1. Run git mv Tools/buk/burd_dispatch.sh Tools/buk/bud_dispatch.sh 2. In bud_dispatch.sh rename all zburd_ function definitions and calls to zbud_ using replace-all: zburd_show zburd_die zburd_check_string zburd_setup zburd_process_args zburd_curate_same zburd_curate_hist zburd_generate_checksum zburd_resolve_color zburd_main zburd_invocation — leave all BURD_ variables untouched 3. In bul_launcher.sh change burd_dispatch.sh to bud_dispatch.sh in the exec line 4. In all 6 tt/ tabtargets change burd_dispatch.sh to bud_dispatch.sh in line 2 5. In CLAUDE.md update acronym mapping: BURD entry becomes BUD pointing to buk/bud_dispatch.sh with bud_* functions, and update the managed BUK section path reference 6. In README.md update all burd_dispatch.sh references to bud_dispatch.sh 7. In vocbumc_core.md update path reference 8. In both Memos update burd_dispatch.sh references to bud_dispatch.sh 9. Run bash -n Tools/buk/bud_dispatch.sh 10. Grep for any remaining zburd_ or burd_dispatch.sh references across the repo excluding .claude/jjm/ to confirm clean sweep | Verify: bash -n Tools/buk/bud_dispatch.sh

**[260211-1530] rough**

Revert incorrect file and function renames from the BUD_→BURD_ variable rename.

The BUD_→BURD_ variable rename was correct, but the model also incorrectly renamed:
1. File: bud_dispatch.sh → burd_dispatch.sh (WRONG - bud is the file/module prefix)
2. Functions: zbud_* → zburd_* (WRONG - zbud_ is the function prefix for the bud module)

The bud prefix (file/function namespace) and BURD prefix (variable namespace) are distinct per terminal exclusivity.

Fix:
- Rename file back: burd_dispatch.sh → bud_dispatch.sh
- Rename all functions back: zburd_* → zbud_* inside the file (zburd_show, zburd_die, zburd_check_string, zburd_setup, zburd_process_args, zburd_curate_same, zburd_curate_hist, zburd_generate_checksum, zburd_resolve_color, zburd_main, plus local zburd_invocation)
- Update all references in ~14 files: bul_launcher.sh, README.md, CLAUDE.md, 6 tt/ tabtargets, 2 memos, rbga_cli.sh, rbgb_cli.sh, rgbs_cli.sh
- Leave all BURD_* variables untouched (those renames were correct)

### address-bcg-fix-review-concerns (₢AcAAK) [complete]

**[260211-1657] complete**

Reconsider issues identified during review of ₢AcAAI (fix-buk-bcg-noncompliance) agent work.

## Issue 1: buc_command.sh alignment whitespace removed (lines 68-70)

Agent removed intentional vertical alignment spaces from three `local` declarations adjacent to the `-i` removal. Only the `-i` flag should have been removed. Restore alignment.

## Issue 2: buc_command.sh no arithmetic validation after `-i` removal (line 67)

BCG pattern for replacing `local -i` calls for explicit validation. Warrant said skip it since callers pass numeric literals. Reconsider whether a guard is warranted given that without `-i`, a non-integer value would be silently interpreted as a variable name in arithmetic context.

## Issue 3: buv_validation.sh eval without variable-name validation (lines 49, 59)

`buv_env_wrapper` and `buv_opt_wrapper` contain `eval "local z_val=\${${z_varname}:-}"` without BCG-required `^[A-Za-z_][A-Za-z0-9_]*$` regex gate. BCG also prefers `${!name}` indirect expansion. Not in original docket but same file, same noncompliance category.

## Issue 4: buv_validation.sh $(ls -A) command substitution in buv_dir_empty (line 42)

`test -z "$(ls -A "${z_dirpath}" 2>/dev/null)"` uses prohibited command substitution pattern. Pre-existing, not introduced by agent. Decide whether to fix in this pass or defer.

## Scope

Modify only Tools/buk/buc_command.sh and Tools/buk/buv_validation.sh. Issues 1-2 are fixups to ₢AcAAI work. Issues 3-4 are pre-existing noncompliance discovered during review — decide fix-or-defer for each.

**[260211-1547] rough**

Reconsider issues identified during review of ₢AcAAI (fix-buk-bcg-noncompliance) agent work.

## Issue 1: buc_command.sh alignment whitespace removed (lines 68-70)

Agent removed intentional vertical alignment spaces from three `local` declarations adjacent to the `-i` removal. Only the `-i` flag should have been removed. Restore alignment.

## Issue 2: buc_command.sh no arithmetic validation after `-i` removal (line 67)

BCG pattern for replacing `local -i` calls for explicit validation. Warrant said skip it since callers pass numeric literals. Reconsider whether a guard is warranted given that without `-i`, a non-integer value would be silently interpreted as a variable name in arithmetic context.

## Issue 3: buv_validation.sh eval without variable-name validation (lines 49, 59)

`buv_env_wrapper` and `buv_opt_wrapper` contain `eval "local z_val=\${${z_varname}:-}"` without BCG-required `^[A-Za-z_][A-Za-z0-9_]*$` regex gate. BCG also prefers `${!name}` indirect expansion. Not in original docket but same file, same noncompliance category.

## Issue 4: buv_validation.sh $(ls -A) command substitution in buv_dir_empty (line 42)

`test -z "$(ls -A "${z_dirpath}" 2>/dev/null)"` uses prohibited command substitution pattern. Pre-existing, not introduced by agent. Decide whether to fix in this pass or defer.

## Scope

Modify only Tools/buk/buc_command.sh and Tools/buk/buv_validation.sh. Issues 1-2 are fixups to ₢AcAAI work. Issues 3-4 are pre-existing noncompliance discovered during review — decide fix-or-defer for each.

### exercise-revised-test-suites (₢AcAAF) [complete]

**[260212-0658] complete**

Human pace: manually run all BUT framework testbench suites against the redesigned butr_/buto_/butd_ infrastructure. Exercise the new explicit registration API, verify init/setup/case two-phase flow, confirm inconclusive reporting, and validate subdispatch logging improvements. Amend anything that feels wrong. This is the interactive validation gate before the heat can close.

**[260211-0859] rough**

Human pace: manually run all BUT framework testbench suites against the redesigned butr_/buto_/butd_ infrastructure. Exercise the new explicit registration API, verify init/setup/case two-phase flow, confirm inconclusive reporting, and validate subdispatch logging improvements. Amend anything that feels wrong. This is the interactive validation gate before the heat can close.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 I fix-buk-bcg-noncompliance
  2 H bcg-shell-trap-guidance
  3 G clarify-bcg-register-pattern
  4 A redesign-butr-registry
  5 C fix-subdispatch-logging
  6 E split-buto-engine
  7 D test-cmd-list-targets
  8 B bcg-kindle-comm-cleanup
  9 J revert-bud-file-function-renames
  10 K address-bcg-fix-review-concerns
  11 F exercise-revised-test-suites

IHGACEDBJKF
·········xx bud_dispatch.sh, rbw-iB.BuildImageRemotely.sh
·····x····x rbtb_testbench.sh
····xx····· buto_operations.sh
···x·x····· butd_dispatch.sh
·xx········ BCG-BashConsoleGuide.md
··········x .gitignore, Dockerfile, bhyslop-nopasswd, rbk_Coordinator.sh, rbob_bottle.sh, rbob_cli.sh, rbrn_pluml.env, rbrn_srjcl.env, rbrr_RecipeBottleRegimeRepo.sh, rbrr_regime.sh, rbrv.env, rbtcim_ImageManagement.sh
·········x· bul_launcher.sh, burd_dispatch.sh, gadcf.LaunchFactoryInContainer.sh, gadi-i.Inspect.sh, rbw-hga.HelpGoogleAdmin.sh, rbw-l.ListCurrentRegistryImages.sh
········x·· CLAUDE.md, README.md, memo-20260110-acronym-selection-study.md, memo-20260209-regime-inventory.md, rbw-him.HelpImageManagement.sh, vocbumc_core.md
·····x····· but_test.sh, butcde_DispatchExercise.sh, bute_engine.sh, buz_zipper.sh, rbtcal_ArkLifecycle.sh
···x······· butr_registry.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 70 commits)

  1 A redesign-butr-registry
  2 I fix-buk-bcg-noncompliance
  3 C fix-subdispatch-logging
  4 E split-buto-engine
  5 D test-cmd-list-targets
  6 B bcg-kindle-comm-cleanup
  7 J revert-bud-file-function-renames
  8 K address-bcg-fix-review-concerns
  9 F exercise-revised-test-suites

123456789abcdefghijklmnopqrstuvwxyz
x··xx······························  A  3c
··x································  I  1c
······x··xxx·······················  C  4c
········x···x··xx··················  E  4c
·············xx··x·················  D  3c
··················x················  B  1c
····················xx··xx·········  J  4c
······················xx···········  K  2c
··························xxxxxxxx·  F  8c
```

## Steeplechase

### 2026-02-12 06:59 - Heat - f

stabled

### 2026-02-12 06:58 - ₢AcAAF - W

All 7 suites pass individually. Fixed: rbw-iB tabtarget, vessel dirs, RBRR dead code, RBGC/RBGD kindling, DRY image URLs, preflight check, stale rbtcim removal. TestAll has pre-existing kindle isolation issue between ark-lifecycle and dispatch-exercise.

### 2026-02-12 06:53 - ₢AcAAF - n

DRY image URLs in rbob_bottle (ZRBOB_SENTRY_IMAGE/ZRBOB_BOTTLE_IMAGE), add preflight image check, add RBGC/RBGD kindling to testbench, remove stale rbtcim image-management suite

### 2026-02-12 06:44 - ₢AcAAF - n

Populate consecrations for srjcl and pluml nameplates from successful GAR builds

### 2026-02-12 06:32 - ₢AcAAF - n

Add RBGC and RBGD kindling to rbob_cli.sh so bottle operations can resolve GAR image URLs

### 2026-02-12 06:24 - ₢AcAAF - n

Remove dead RBRR_HISTORY_DIR and RBRR_NAMEPLATE_PATH from repo config and validation, replace with RBRR_VESSEL_DIR check

### 2026-02-12 06:21 - ₢AcAAF - n

Fix rbw-iB tabtarget (launcher pattern + coordinator routing), create vessel dirs for rbev-bottle-anthropic-jupyter and rbev-bottle-plantuml with Dockerfiles from legacy recipes, delete stale build-context and nopasswd files

### 2026-02-11 17:10 - ₢AcAAF - d

Suites 1-3 pass (kick-tires, ark-lifecycle, dispatch-exercise). Suite 4 nsproto-security fails on missing containers (env precondition). Suites 5-8 pending.

### 2026-02-11 17:03 - ₢AcAAF - A

Interactive: run all suites, review output, fix issues, re-validate

### 2026-02-11 17:01 - ₢AcAAJ - W

Reverted burd_dispatch.sh to bud_dispatch.sh and all zburd_ functions to zbud_, updated 13 files, BURD_ variables untouched

### 2026-02-11 17:01 - ₢AcAAJ - n

Update CLAUDE.md, memos, README, and tabtarget references from burd_dispatch.sh to bud_dispatch.sh after dispatch module rename

### 2026-02-11 16:57 - ₢AcAAK - W

Fixed: numeric depth guard in zbuc_make_tag, regex-gated eval in buv wrappers, find-based dir empty check replacing ls -A

### 2026-02-11 16:57 - ₢AcAAK - n

Rename burd_dispatch.sh to bud_dispatch.sh and rename all zburd_ functions to zbud_, updating all tabtarget references

### 2026-02-11 16:56 - ₢AcAAJ - F

Executing bridled pace via sonnet agent

### 2026-02-11 16:54 - ₢AcAAJ - B

arm | revert-bud-file-function-renames

### 2026-02-11 16:54 - Heat - T

revert-bud-file-function-renames

### 2026-02-11 16:51 - ₢AcAAB - W

Moot: kindle-as-communication antipattern eliminated by AcAAA/AcAAE rewrites of butd_dispatch.sh

### 2026-02-11 16:49 - ₢AcAAD - W

Added available target listing to butd_run_suite and butd_run_one for no-arg invocation

### 2026-02-11 16:48 - ₢AcAAE - W

Split buto_operations.sh into operations (test-case API) and bute_engine.sh (execution machinery), added BURV bridge via BUTE_BURV_ROOT, rewrote rbtcal to use operations-only functions, updated all callers

### 2026-02-11 16:48 - ₢AcAAE - n

Extract test engine from buto_operations into bute_engine.sh, migrate dispatch/evidence/case machinery to bute_ namespace, add BURV bridge isolation to zbuto_invoke, simplify ark-lifecycle to use buto_tt_expect_ok, and add suite/function listing to butd_run_suite and butd_run_one

### 2026-02-11 16:47 - ₢AcAAD - L

sonnet landed

### 2026-02-11 16:45 - ₢AcAAD - F

Executing bridled pace via sonnet agent

### 2026-02-11 16:43 - ₢AcAAE - F

Executing bridled pace via sonnet agent

### 2026-02-11 16:41 - ₢AcAAC - W

Removed BURD_NO_LOG=1 suppression (replaced with BURD_NO_LOG= to clear inherited env), added inner artifact path logging via buc_log_args, added unconditional failure display via zbuto_render_lines

### 2026-02-11 16:41 - ₢AcAAC - n

Review fix: clear inherited BURD_NO_LOG via empty-string env prefix so inner tabtargets run with natural logging behavior

### 2026-02-11 16:17 - ₢AcAAC - L

sonnet landed

### 2026-02-11 16:17 - ₢AcAAE - B

arm | split-buto-engine

### 2026-02-11 16:17 - Heat - T

split-buto-engine

### 2026-02-11 16:16 - ₢AcAAC - F

Executing bridled pace via sonnet agent

### 2026-02-11 16:15 - Heat - T

split-buto-engine

### 2026-02-11 16:01 - ₢AcAAA - W

Redesigned butr_registry.sh with explicit BCG enroll/recite/roll patterns, adapted butd_dispatch.sh with init phase and skip tracking, removed buto_execute, converted rbtb_testbench.sh to explicit 46-case enrollment across 8 suites

### 2026-02-11 15:59 - ₢AcAAA - n

Fix review findings: wire up skip tracking in butd_run_all, quote arithmetic test vars, remove dead return-via-variable code

### 2026-02-11 15:48 - ₢AcAAI - W

Fixed BCG noncompliance: 9 test-dollar-grep silent pass-throughs to grep -qE, inclusion guard and dirname to BCG patterns, z_ prefix on all locals, braced/quoted all expansions, 2 subshell exit bugs to brace-group, local -i removal. Review identified 4 follow-up concerns tracked in ₢AcAAK.

### 2026-02-11 15:47 - Heat - S

address-bcg-fix-review-concerns

### 2026-02-11 15:37 - ₢AcAAA - F

Executing bridled pace via sonnet agent

### 2026-02-11 15:37 - ₢AcAAI - L

sonnet landed

### 2026-02-11 15:32 - ₢AcAAI - F

Executing bridled pace via sonnet agent

### 2026-02-11 15:31 - ₢AcAAD - B

arm | test-cmd-list-targets

### 2026-02-11 15:31 - Heat - T

test-cmd-list-targets

### 2026-02-11 15:30 - Heat - S

revert-bud-file-function-renames

### 2026-02-11 15:29 - ₢AcAAC - B

arm | fix-subdispatch-logging

### 2026-02-11 15:29 - Heat - T

fix-subdispatch-logging

### 2026-02-11 15:26 - ₢AcAAI - B

arm | fix-buk-bcg-noncompliance

### 2026-02-11 15:26 - Heat - T

fix-buk-bcg-noncompliance

### 2026-02-11 15:21 - ₢AcAAA - B

arm | redesign-butr-registry

### 2026-02-11 15:21 - Heat - T

redesign-butr-registry

### 2026-02-11 15:21 - Heat - T

redesign-butr-registry

### 2026-02-11 15:16 - Heat - S

fix-buk-bcg-noncompliance

### 2026-02-11 15:13 - ₢AcAAA - A

Explicit butr_suite/butr_case API, single-init, exit-75 inconclusive, no tiers, no globs

### 2026-02-11 15:11 - ₢AcAAH - W

BCG shell trap guidance: 9 original changes (silent failure traps, compatibility fixes, safety rules) plus review-driven refinements (printf-v over eval, checklist hardening with mechanically verifiable items, array teaching content relocated to body, pipeline specificity)

### 2026-02-11 15:10 - ₢AcAAH - n

Replace vague pipeline checklist item with three mechanically verifiable conditions

### 2026-02-11 15:10 - ₢AcAAH - n

Checklist hardening: strengthen conditional/error-block/eval items, relocate array teaching content to body, add missing body-rule encodings, qualify enroll/recite items

### 2026-02-11 15:02 - ₢AcAAH - n

Review-driven refinements: printf-v over eval, local-i comment fix, set-e section promotion, checklist hardening, arithmetic command context

### 2026-02-11 14:51 - ₢AcAAH - n

Apply 9 targeted BCG improvements: anti-patterns (test empty expansion, subshell exit), set-e predicate rule, array set-u safety, remove here-string misclassification, forbid local -i, legacy code note, eval policy, temp file lifecycle

### 2026-02-11 14:38 - ₢AcAAH - A

Sequential sonnet: 9 targeted BCG edits (anti-patterns, compatibility, safety rules) in single file

### 2026-02-11 14:36 - Heat - S

bcg-shell-trap-guidance

### 2026-02-11 12:05 - ₢AcAAG - W

Expanded BCG with roll/enroll/recite pattern family, module constant exclusivity rule, KINDLED-last rule, loop error handling guidance per function type, Fading Memory section with FM-001, and synthetic-only examples

### 2026-02-11 12:05 - ₢AcAAG - n

Expand BCG with roll/enroll/recite pattern, module constant exclusivity, KINDLED-last rule, loop error handling guidance, and Fading Memory section

### 2026-02-11 10:09 - Heat - T

clarify-bcg-register-pattern

### 2026-02-11 09:45 - Heat - S

clarify-bcg-register-pattern

### 2026-02-11 09:15 - Heat - f

racing

### 2026-02-11 09:05 - Heat - r

order: ₢AcAAA, ₢AcAAC, ₢AcAAE, ₢AcAAD, ₢AcAAB, ₢AcAAF

### 2026-02-11 09:02 - Heat - d

paddock curried: Initial paddock with lineage from ₣AT

### 2026-02-11 08:59 - Heat - S

exercise-revised-test-suites

### 2026-02-11 08:59 - Heat - D

ATAAT → ₢AcAAE

### 2026-02-11 08:59 - Heat - D

ATAAU → ₢AcAAD

### 2026-02-11 08:59 - Heat - D

ATAAX → ₢AcAAC

### 2026-02-11 08:59 - Heat - D

ATAAV → ₢AcAAB

### 2026-02-11 08:59 - Heat - D

ATAAW → ₢AcAAA

### 2026-02-11 08:59 - Heat - N

but-test-overhaul

