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

### Known issues driving this heat

1. butr_register packs too much into one call; glob discovery is implicit and fragile
2. buto_operations.sh mixes lightweight assertions with heavyweight engine (dispatch, evidence, step capture)
3. buto_dispatch forces BURD_NO_LOG=1 on inner tabtargets, altering their configured nature
4. No preflight/init mechanism — environment problems (unpushed git, TCC prompts) discovered late
5. No-arg invocation of rbtb-to/rbtb-ts gives error with no help listing available targets
6. BCG kindle constant used as inter-function communication in butd

## Downstream Work

- ₢ATAAS in ₣AT blocks on this heat
- split-buto-engine (₢AcAAE) informs where new init/assure functions land
- redesign-butr-registry (₢AcAAA) is the architectural keystone