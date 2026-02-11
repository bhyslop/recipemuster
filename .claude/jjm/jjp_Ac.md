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