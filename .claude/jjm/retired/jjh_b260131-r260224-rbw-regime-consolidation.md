# Heat Trophy: rbw-regime-consolidation

**Firemark:** ₣AT
**Created:** 260131
**Retired:** 260224
**Status:** retired

## Paddock

Memos/memo-20260209-regime-inventory.md

## Paces

### but-framework-infrastructure (₢ATAAP) [complete]

**[260210-1257] complete**

Create the new BUT test framework: buto_ operations, butr_ registry,
butd_ dispatch, plus rbtb_ testbench skeleton with one kick-tires case.

Absorbs the work from abandoned ATAAJ (extract dispatch from buz).

## New prefix tree

  but (parent -- BUK Test)
  +-- buto_ -- Operations (assertions, invocations, output)
  +-- butr_ -- Registry (suite registration, pure data)
  +-- butd_ -- Dispatch (runner, execution, reporting)
  +-- butc_ -- Cases (butcXY_* BUK-level test case files, future)
  +-- butb_ -- Bench (BUK-level testbench, future)

  rbt (parent -- Recipe Bottle Test)
  +-- rbtc_ -- Cases (rbtcXY_* test case files)
  +-- rbtb_ -- Bench (testbench, domain helpers, kindle)

## Files to create

1. Tools/buk/buto_operations.sh
   - All content from current but_test.sh, renamed but_* to buto_*:
     buto_section, buto_info, buto_trace, buto_fatal,
     buto_fatal_on_error, buto_fatal_on_success,
     buto_unit_expect_ok, buto_unit_expect_ok_stdout,
     buto_unit_expect_fatal,
     buto_tt_expect_ok, buto_tt_expect_fatal,
     buto_launch_expect_ok, buto_launch_expect_fatal
   - Extract from buz_zipper.sh and rename:
     buto_init_evidence, buto_dispatch, buto_last_step_capture,
     buto_get_step_exit_capture, buto_get_step_output_capture
   - Internal: buc_log_args to buto_info, buc_die to buto_fatal,
     zbuz_sentinel to buto-local validation,
     zbut_resolve_tabtarget used for tabtarget resolution
   - Include guard: ZBUTO_INCLUDED
   - Add buto_success (green output to stderr, matching zbut_case
     pattern at current but_test.sh:288)

2. Tools/buk/butr_registry.sh
   - butr_kindle: initialize suite parallel arrays
   - butr_register(suite_name, glob_pattern, setup_fn, tier):
     populate arrays
   - butr_get_suites, butr_get_pattern, butr_get_setup, butr_get_tier:
     query functions
   - Include guard: ZBUTR_INCLUDED

3. Tools/buk/butd_dispatch.sh
   - butd_run_all [--fast|--slow]: run all registered suites
   - butd_run_suite suite_name: run one suite
   - butd_run_one function_name: run single test
   - Creates per-suite temp dirs under BURD_TEMP_DIR
   - Calls setup function before each suite (from registry)
   - Discovers functions via declare -F + registered glob
   - Runs each in subshell (zbuto_case pattern, moved from buto_)
   - Reports pass/fail summary
   - Include guard: ZBUTD_INCLUDED

4. Tools/rbw/rbtb_testbench.sh (skeleton)
   - Sources buto_operations.sh, butr_registry.sh, butd_dispatch.sh
   - Sources buz_zipper.sh, rbz_zipper.sh (for colophon constants)
   - rbtb_kindle: register at least one kick-tires suite
   - Provides setup functions (zrbtb_setup_*)
   - Route command to butd_ runner

5. Tools/rbw/rbtckk_KickTires.sh (kick-tires case file)
   - One or two trivial rbtckk_* test functions
   - Proves end-to-end: tabtarget -> BURD -> rbtb -> butd -> buto

6. Tabtarget: tt/rbtb-t.TestAll.sh (or similar colophon)

## Also modify

- Tools/buk/buz_zipper.sh: remove dispatch/evidence/step sections
  (lines 107-191). Remove step arrays from zbuz_kindle. Keep only
  registry: zbuz_kindle, zbuz_sentinel, zbuz_resolve_tabtarget_capture,
  buz_register.
- Delete Tools/buk/but_test.sh (after all callers migrated -- may need
  to keep as shim initially if other paces have not yet converted callers)

## Callers NOT migrated in this pace
- rbtg_testbench.sh (handled by convert-dispatch-exercise and
  convert-ark-lifecycle paces)
- rbt_testbench.sh (handled by convert-rbt-suites pace)
- tbvu_suite_xname.sh, trbim_suite.sh (handled by
  convert-remaining-testbenches pace)
These callers continue using but_test.sh until their conversion paces.
Keeping but_test.sh as a compatibility shim that sources buto_operations.sh
and aliases buto_* back to but_* enables incremental migration.

## Verification
- rbtckk kick-tires case runs end-to-end through the new framework
- buz_zipper.sh contains only registry functions
- buto_operations.sh has all assertion/invocation/dispatch functions
- butr_registry.sh handles suite registration
- butd_dispatch.sh runs registered suites

**[260210-1236] rough**

Create the new BUT test framework: buto_ operations, butr_ registry,
butd_ dispatch, plus rbtb_ testbench skeleton with one kick-tires case.

Absorbs the work from abandoned ATAAJ (extract dispatch from buz).

## New prefix tree

  but (parent -- BUK Test)
  +-- buto_ -- Operations (assertions, invocations, output)
  +-- butr_ -- Registry (suite registration, pure data)
  +-- butd_ -- Dispatch (runner, execution, reporting)
  +-- butc_ -- Cases (butcXY_* BUK-level test case files, future)
  +-- butb_ -- Bench (BUK-level testbench, future)

  rbt (parent -- Recipe Bottle Test)
  +-- rbtc_ -- Cases (rbtcXY_* test case files)
  +-- rbtb_ -- Bench (testbench, domain helpers, kindle)

## Files to create

1. Tools/buk/buto_operations.sh
   - All content from current but_test.sh, renamed but_* to buto_*:
     buto_section, buto_info, buto_trace, buto_fatal,
     buto_fatal_on_error, buto_fatal_on_success,
     buto_unit_expect_ok, buto_unit_expect_ok_stdout,
     buto_unit_expect_fatal,
     buto_tt_expect_ok, buto_tt_expect_fatal,
     buto_launch_expect_ok, buto_launch_expect_fatal
   - Extract from buz_zipper.sh and rename:
     buto_init_evidence, buto_dispatch, buto_last_step_capture,
     buto_get_step_exit_capture, buto_get_step_output_capture
   - Internal: buc_log_args to buto_info, buc_die to buto_fatal,
     zbuz_sentinel to buto-local validation,
     zbut_resolve_tabtarget used for tabtarget resolution
   - Include guard: ZBUTO_INCLUDED
   - Add buto_success (green output to stderr, matching zbut_case
     pattern at current but_test.sh:288)

2. Tools/buk/butr_registry.sh
   - butr_kindle: initialize suite parallel arrays
   - butr_register(suite_name, glob_pattern, setup_fn, tier):
     populate arrays
   - butr_get_suites, butr_get_pattern, butr_get_setup, butr_get_tier:
     query functions
   - Include guard: ZBUTR_INCLUDED

3. Tools/buk/butd_dispatch.sh
   - butd_run_all [--fast|--slow]: run all registered suites
   - butd_run_suite suite_name: run one suite
   - butd_run_one function_name: run single test
   - Creates per-suite temp dirs under BURD_TEMP_DIR
   - Calls setup function before each suite (from registry)
   - Discovers functions via declare -F + registered glob
   - Runs each in subshell (zbuto_case pattern, moved from buto_)
   - Reports pass/fail summary
   - Include guard: ZBUTD_INCLUDED

4. Tools/rbw/rbtb_testbench.sh (skeleton)
   - Sources buto_operations.sh, butr_registry.sh, butd_dispatch.sh
   - Sources buz_zipper.sh, rbz_zipper.sh (for colophon constants)
   - rbtb_kindle: register at least one kick-tires suite
   - Provides setup functions (zrbtb_setup_*)
   - Route command to butd_ runner

5. Tools/rbw/rbtckk_KickTires.sh (kick-tires case file)
   - One or two trivial rbtckk_* test functions
   - Proves end-to-end: tabtarget -> BURD -> rbtb -> butd -> buto

6. Tabtarget: tt/rbtb-t.TestAll.sh (or similar colophon)

## Also modify

- Tools/buk/buz_zipper.sh: remove dispatch/evidence/step sections
  (lines 107-191). Remove step arrays from zbuz_kindle. Keep only
  registry: zbuz_kindle, zbuz_sentinel, zbuz_resolve_tabtarget_capture,
  buz_register.
- Delete Tools/buk/but_test.sh (after all callers migrated -- may need
  to keep as shim initially if other paces have not yet converted callers)

## Callers NOT migrated in this pace
- rbtg_testbench.sh (handled by convert-dispatch-exercise and
  convert-ark-lifecycle paces)
- rbt_testbench.sh (handled by convert-rbt-suites pace)
- tbvu_suite_xname.sh, trbim_suite.sh (handled by
  convert-remaining-testbenches pace)
These callers continue using but_test.sh until their conversion paces.
Keeping but_test.sh as a compatibility shim that sources buto_operations.sh
and aliases buto_* back to but_* enables incremental migration.

## Verification
- rbtckk kick-tires case runs end-to-end through the new framework
- buz_zipper.sh contains only registry functions
- buto_operations.sh has all assertion/invocation/dispatch functions
- butr_registry.sh handles suite registration
- butd_dispatch.sh runs registered suites

### convert-ark-lifecycle (₢ATAAO) [complete]

**[260210-1308] complete**

Convert rbtg_case_ark_lifecycle to a conventional testbench using the new
BUT framework (buto_/butr_/butd_/rbtcXY_ pattern).

## Work

1. Create rbtcal_ArkLifecycle.sh containing rbtcal_lifecycle() function.
   - Zero-arg function, self-contained
   - Uses buto_dispatch for BURV isolation and evidence collection
   - Uses buto_fatal_on_error for assertion
   - Uses buto_info/buto_section for progress output (not buc_*)
   - Uses BUT_TEMP_DIR (from butd_ framework) for temp files
   - Accesses RBZ_* colophon constants (rbz_zipper.sh must be sourced)

2. Register in rbtb_testbench.sh:
   butr_register "rbtcal" "rbtcal_*" zrbtb_setup_ark "slow"
   Setup function kindles buz, rbz, and initializes buto evidence.

3. Verify it runs through butd_ runner end-to-end.

## Note
This is a real integration test against live GCP. Tier is "slow".
The 6-step sequence (list/conjure/verify/retrieve/delete/verify) stays
as one function -- it is inherently sequential with dependencies between
steps.

## Depends on
- but-framework-infrastructure pace (buto_/butr_/butd_ must exist)

**[260210-1235] rough**

Convert rbtg_case_ark_lifecycle to a conventional testbench using the new
BUT framework (buto_/butr_/butd_/rbtcXY_ pattern).

## Work

1. Create rbtcal_ArkLifecycle.sh containing rbtcal_lifecycle() function.
   - Zero-arg function, self-contained
   - Uses buto_dispatch for BURV isolation and evidence collection
   - Uses buto_fatal_on_error for assertion
   - Uses buto_info/buto_section for progress output (not buc_*)
   - Uses BUT_TEMP_DIR (from butd_ framework) for temp files
   - Accesses RBZ_* colophon constants (rbz_zipper.sh must be sourced)

2. Register in rbtb_testbench.sh:
   butr_register "rbtcal" "rbtcal_*" zrbtb_setup_ark "slow"
   Setup function kindles buz, rbz, and initializes buto evidence.

3. Verify it runs through butd_ runner end-to-end.

## Note
This is a real integration test against live GCP. Tier is "slow".
The 6-step sequence (list/conjure/verify/retrieve/delete/verify) stays
as one function -- it is inherently sequential with dependencies between
steps.

## Depends on
- but-framework-infrastructure pace (buto_/butr_/butd_ must exist)

### convert-dispatch-exercise (₢ATAAN) [complete]

**[260210-1319] complete**

Convert rbtg_case_dispatch_exercise to BUK-level test case and retire
rbtg_testbench.sh.

## Context

dispatch_exercise tests BUK infrastructure (buto_dispatch step arrays,
evidence directories, BURV isolation). It belongs in Tools/buk/ as a
BUK-level test case, not in Tools/rbw/.

Currently it uses RBZ_LIST_IMAGES (RB colophon) as something to dispatch.
To make it self-contained within BUK, create a trivial test colophon
butctt_ ("test target") that dispatch_exercise can use instead.

## Work

1. Create Tools/buk/butctt_TestTarget.sh (or tabtarget equivalent):
   A trivial "hello world" command registered as a BUK test colophon.
   Needs a tabtarget in tt/ and a BUK-level zipper registration so
   buto_dispatch can resolve it. Produces known stdout so dispatch
   tests can verify output capture. Exits 0 on success.

2. Create Tools/buk/butcde_DispatchExercise.sh:
   - butcde_evidence_created(): dispatch butctt_ colophon via
     buto_dispatch, verify step arrays populated, evidence dir exists
   - butcde_burv_isolation(): verify BURV temp dirs created by inner
     dispatch
   - butcde_exit_capture(): verify buto_get_step_exit_capture returns
     correct status
   - Uses buto_* output throughout (no buc_* mixing -- the motivation
     for the original extraction)

3. Register in butb_testbench.sh (or rbtb_ if no BUK-level testbench
   exists yet -- decide during but-framework-infrastructure):
   butr_register "butcde" "butcde_*" zbutb_setup_dispatch "fast"
   Setup function kindles buz (for colophon registry), registers
   butctt_ colophon, initializes buto evidence.

4. Retire rbtg_testbench.sh after BOTH this pace and convert-ark-lifecycle
   complete. If ark_lifecycle is not yet converted, leave rbtg in place
   but remove only dispatch_exercise from it.

## Depends on
- but-framework-infrastructure (buto_/butr_/butd_ must exist)
- convert-ark-lifecycle should complete before full rbtg retirement

**[260210-1242] rough**

Convert rbtg_case_dispatch_exercise to BUK-level test case and retire
rbtg_testbench.sh.

## Context

dispatch_exercise tests BUK infrastructure (buto_dispatch step arrays,
evidence directories, BURV isolation). It belongs in Tools/buk/ as a
BUK-level test case, not in Tools/rbw/.

Currently it uses RBZ_LIST_IMAGES (RB colophon) as something to dispatch.
To make it self-contained within BUK, create a trivial test colophon
butctt_ ("test target") that dispatch_exercise can use instead.

## Work

1. Create Tools/buk/butctt_TestTarget.sh (or tabtarget equivalent):
   A trivial "hello world" command registered as a BUK test colophon.
   Needs a tabtarget in tt/ and a BUK-level zipper registration so
   buto_dispatch can resolve it. Produces known stdout so dispatch
   tests can verify output capture. Exits 0 on success.

2. Create Tools/buk/butcde_DispatchExercise.sh:
   - butcde_evidence_created(): dispatch butctt_ colophon via
     buto_dispatch, verify step arrays populated, evidence dir exists
   - butcde_burv_isolation(): verify BURV temp dirs created by inner
     dispatch
   - butcde_exit_capture(): verify buto_get_step_exit_capture returns
     correct status
   - Uses buto_* output throughout (no buc_* mixing -- the motivation
     for the original extraction)

3. Register in butb_testbench.sh (or rbtb_ if no BUK-level testbench
   exists yet -- decide during but-framework-infrastructure):
   butr_register "butcde" "butcde_*" zbutb_setup_dispatch "fast"
   Setup function kindles buz (for colophon registry), registers
   butctt_ colophon, initializes buto evidence.

4. Retire rbtg_testbench.sh after BOTH this pace and convert-ark-lifecycle
   complete. If ark_lifecycle is not yet converted, leave rbtg in place
   but remove only dispatch_exercise from it.

## Depends on
- but-framework-infrastructure (buto_/butr_/butd_ must exist)
- convert-ark-lifecycle should complete before full rbtg retirement

**[260210-1234] rough**

Convert rbtg_case_dispatch_exercise to new BUT framework and retire
rbtg_testbench.sh.

## Context

dispatch_exercise is a test-of-test-infrastructure: it verifies that
buto_dispatch populates step arrays, creates evidence directories, and
provides BURV isolation. Its ongoing value drops once buto_dispatch is
proven, but it serves as a smoke test for the test framework itself.

## Work

1. Create rbtcde_DispatchExercise.sh (or butcde_ if this is BUK-level
   infrastructure testing -- decide based on whether dispatch testing
   belongs to BUK or RB).

2. The test currently uses buc_* output because buz internals use buc.
   After but-framework-infrastructure, dispatch lives in buto_ with
   buto_* output. So the mixing problem is SOLVED -- the test case can
   use buto_* throughout.

3. Convert buc_step/buc_log_args/buc_die/buc_success calls to
   buto_section/buto_info/buto_fatal/buto_success.

4. Register in rbtb_testbench.sh (or butb_ if BUK-level):
   butr_register "rbtcde" "rbtcde_*" zrbtb_setup_dispatch "fast"

5. Retire rbtg_testbench.sh after both dispatch_exercise and
   ark_lifecycle have been migrated.

## Decision needed
- Is dispatch exercise RB-level (rbtcde_) or BUK-level (butcde_)?
  It tests buto_dispatch which is BUK infrastructure, but uses
  rbz_zipper constants (RBZ_LIST_IMAGES) which are RB-specific.
  Leaning rbtcde_ since it depends on RB colophon registration.

## Depends on
- but-framework-infrastructure
- convert-ark-lifecycle (both must complete before rbtg can be retired)

### convert-rbt-suites (₢ATAAM) [complete]

**[260210-1331] complete**

Convert rbt_testbench.sh suites (nsproto, srjcl, pluml) to the new BUT
framework using rbtcXY_ case files.

## Work

1. Create test case files:
   - rbtcns_NsproSecurity.sh: rename test_nsproto_* to rbtcns_*
   - rbtcsj_SrjclJupyter.sh: rename test_srjcl_* to rbtcsj_*
   - rbtcpl_PlumlDiagram.sh: rename test_pluml_* to rbtcpl_*

2. Move shared helpers to rbtb_testbench.sh:
   - rbt_exec_sentry/censer/bottle and _i variants -> rbtb_exec_*
   - rbt_load_nameplate -> rbtb_load_nameplate
   - rbt_show -> rbtb_show

3. Register suites in rbtb_testbench.sh kindle:
   - butr_register "rbtcns" "rbtcns_*" zrbtb_setup_nsproto "slow"
   - butr_register "rbtcsj" "rbtcsj_*" zrbtb_setup_srjcl "slow"
   - butr_register "rbtcpl" "rbtcpl_*" zrbtb_setup_pluml "slow"
   (All slow because they require live containers)

4. Retire rbt_testbench.sh

## Depends on
- but-framework-infrastructure pace (buto_/butr_/butd_ must exist)

**[260210-1234] rough**

Convert rbt_testbench.sh suites (nsproto, srjcl, pluml) to the new BUT
framework using rbtcXY_ case files.

## Work

1. Create test case files:
   - rbtcns_NsproSecurity.sh: rename test_nsproto_* to rbtcns_*
   - rbtcsj_SrjclJupyter.sh: rename test_srjcl_* to rbtcsj_*
   - rbtcpl_PlumlDiagram.sh: rename test_pluml_* to rbtcpl_*

2. Move shared helpers to rbtb_testbench.sh:
   - rbt_exec_sentry/censer/bottle and _i variants -> rbtb_exec_*
   - rbt_load_nameplate -> rbtb_load_nameplate
   - rbt_show -> rbtb_show

3. Register suites in rbtb_testbench.sh kindle:
   - butr_register "rbtcns" "rbtcns_*" zrbtb_setup_nsproto "slow"
   - butr_register "rbtcsj" "rbtcsj_*" zrbtb_setup_srjcl "slow"
   - butr_register "rbtcpl" "rbtcpl_*" zrbtb_setup_pluml "slow"
   (All slow because they require live containers)

4. Retire rbt_testbench.sh

## Depends on
- but-framework-infrastructure pace (buto_/butr_/butd_ must exist)

### convert-remaining-testbenches (₢ATAAL) [complete]

**[260210-1338] complete**

Convert tbvu_suite_xname.sh, trbim_suite.sh, and any other remaining testbenches
to the new BUT framework (buto_/butr_/butd_/rbtcXY_ pattern).

For each testbench:
- Create rbtcXY_*.sh case file(s) with renamed test functions
- Register suites in rbtb_testbench.sh
- Retire the old testbench file

## Files to survey
- Tools/buk/tbvu_suite_xname.sh
- Tools/rbw/trbim_suite.sh (verify this exists and check its structure)
- Any other *_suite*.sh or *_testbench*.sh files

## Depends on
- but-framework-infrastructure pace (buto_/butr_/butd_ must exist)

**[260210-1233] rough**

Convert tbvu_suite_xname.sh, trbim_suite.sh, and any other remaining testbenches
to the new BUT framework (buto_/butr_/butd_/rbtcXY_ pattern).

For each testbench:
- Create rbtcXY_*.sh case file(s) with renamed test functions
- Register suites in rbtb_testbench.sh
- Retire the old testbench file

## Files to survey
- Tools/buk/tbvu_suite_xname.sh
- Tools/rbw/trbim_suite.sh (verify this exists and check its structure)
- Any other *_suite*.sh or *_testbench*.sh files

## Depends on
- but-framework-infrastructure pace (buto_/butr_/butd_ must exist)

### refactor-but-test-families (₢ATAAI) [complete]

**[260210-0953] complete**

Fix critical issues from initial refactoring of but_test.sh three assertion families.

## Issues to fix

### CRITICAL 1: rbtg_testbench.sh but_tt_expect_ok bypasses buz_dispatch

rbtg_testbench.sh was mechanically renamed from buz_dispatch_expect_ok to but_tt_expect_ok, but buz_dispatch_expect_ok called buz_dispatch() which provides BURV isolation, evidence harvesting, and step array tracking. but_tt_expect_ok uses zbut_invoke instead -- no BURV, no evidence, no steps.

rbtg_case_dispatch_exercise (lines 46-60) calls buz_last_step_capture and verifies evidence/BURV -- these will fail at runtime since but_tt_expect_ok never populates step arrays.

**Fix**: rbtg_testbench.sh must NOT use but_tt_expect_ok. It should keep calling buz_dispatch directly for the invocation (BURV isolation + evidence), then assert on the step exit status using buz_last_step_capture and buz_get_step_exit_capture. The assertion pattern becomes:

```bash
buz_dispatch "colophon" args...
z_step=$(buz_last_step_capture)
z_status=$(buz_get_step_exit_capture "$z_step")
but_fatal_on_error "$z_status" "dispatch failed" "Colophon: colophon"
```

This is verbose but correct -- rbtg is testing the dispatch/evidence infrastructure itself, not just the command outcome.

### CRITICAL 2: stdout swallowed in ark lifecycle

zbut_invoke captures stdout into ZBUT_STDOUT variable. The ark lifecycle test redirects but_tt_expect_ok output to files:

```bash
but_tt_expect_ok "RBZ_LIST_IMAGES" > "baseline.txt"
```

This produces an empty file. The old buz_dispatch invoked tabtargets directly so stdout flowed through.

**Fix**: Same as Critical 1 -- rbtg_testbench.sh reverts to buz_dispatch for all invocations. buz_dispatch runs tabtargets directly (stdout flows through). The redirect pattern works with buz_dispatch.

### MODERATE: BURC_TABTARGET_DIR fallback to tt

zbut_resolve_tabtarget line 172 uses fallback:
```bash
local z_tt_dir="${BURC_TABTARGET_DIR:-tt}"
```

**Fix**: Remove fallback. Die if BURC_TABTARGET_DIR is not set:
```bash
local z_tt_dir="${BURC_TABTARGET_DIR:-}"
test -n "${z_tt_dir}" || but_fatal "BURC_TABTARGET_DIR not set -- but_tt requires BUK environment"
```

### MINOR: Extra blank line in buz_zipper.sh

Line 163 has stray blank line where removed functions were.

**Fix**: Remove the extra blank line.

## Files to modify

- Tools/rbw/rbtg_testbench.sh -- revert but_tt_expect_ok calls back to buz_dispatch + manual assertion pattern
- Tools/buk/but_test.sh -- remove BURC_TABTARGET_DIR fallback, die instead
- Tools/buk/buz_zipper.sh -- remove stray blank line at line 163

## Verification

- rbtg_case_dispatch_exercise evidence/BURV verification logic is intact (uses buz_dispatch, buz_last_step_capture, buz_get_step_exit_capture)
- rbtg_case_ark_lifecycle stdout redirects produce non-empty files (uses buz_dispatch directly)
- zbut_resolve_tabtarget dies when BURC_TABTARGET_DIR is unset (no fallback)
- grep confirms no but_tt_expect calls remain in rbtg_testbench.sh
- grep confirms no "tt" fallback in zbut_resolve_tabtarget

**[260210-0944] rough**

Fix critical issues from initial refactoring of but_test.sh three assertion families.

## Issues to fix

### CRITICAL 1: rbtg_testbench.sh but_tt_expect_ok bypasses buz_dispatch

rbtg_testbench.sh was mechanically renamed from buz_dispatch_expect_ok to but_tt_expect_ok, but buz_dispatch_expect_ok called buz_dispatch() which provides BURV isolation, evidence harvesting, and step array tracking. but_tt_expect_ok uses zbut_invoke instead -- no BURV, no evidence, no steps.

rbtg_case_dispatch_exercise (lines 46-60) calls buz_last_step_capture and verifies evidence/BURV -- these will fail at runtime since but_tt_expect_ok never populates step arrays.

**Fix**: rbtg_testbench.sh must NOT use but_tt_expect_ok. It should keep calling buz_dispatch directly for the invocation (BURV isolation + evidence), then assert on the step exit status using buz_last_step_capture and buz_get_step_exit_capture. The assertion pattern becomes:

```bash
buz_dispatch "colophon" args...
z_step=$(buz_last_step_capture)
z_status=$(buz_get_step_exit_capture "$z_step")
but_fatal_on_error "$z_status" "dispatch failed" "Colophon: colophon"
```

This is verbose but correct -- rbtg is testing the dispatch/evidence infrastructure itself, not just the command outcome.

### CRITICAL 2: stdout swallowed in ark lifecycle

zbut_invoke captures stdout into ZBUT_STDOUT variable. The ark lifecycle test redirects but_tt_expect_ok output to files:

```bash
but_tt_expect_ok "RBZ_LIST_IMAGES" > "baseline.txt"
```

This produces an empty file. The old buz_dispatch invoked tabtargets directly so stdout flowed through.

**Fix**: Same as Critical 1 -- rbtg_testbench.sh reverts to buz_dispatch for all invocations. buz_dispatch runs tabtargets directly (stdout flows through). The redirect pattern works with buz_dispatch.

### MODERATE: BURC_TABTARGET_DIR fallback to tt

zbut_resolve_tabtarget line 172 uses fallback:
```bash
local z_tt_dir="${BURC_TABTARGET_DIR:-tt}"
```

**Fix**: Remove fallback. Die if BURC_TABTARGET_DIR is not set:
```bash
local z_tt_dir="${BURC_TABTARGET_DIR:-}"
test -n "${z_tt_dir}" || but_fatal "BURC_TABTARGET_DIR not set -- but_tt requires BUK environment"
```

### MINOR: Extra blank line in buz_zipper.sh

Line 163 has stray blank line where removed functions were.

**Fix**: Remove the extra blank line.

## Files to modify

- Tools/rbw/rbtg_testbench.sh -- revert but_tt_expect_ok calls back to buz_dispatch + manual assertion pattern
- Tools/buk/but_test.sh -- remove BURC_TABTARGET_DIR fallback, die instead
- Tools/buk/buz_zipper.sh -- remove stray blank line at line 163

## Verification

- rbtg_case_dispatch_exercise evidence/BURV verification logic is intact (uses buz_dispatch, buz_last_step_capture, buz_get_step_exit_capture)
- rbtg_case_ark_lifecycle stdout redirects produce non-empty files (uses buz_dispatch directly)
- zbut_resolve_tabtarget dies when BURC_TABTARGET_DIR is unset (no fallback)
- grep confirms no but_tt_expect calls remain in rbtg_testbench.sh
- grep confirms no "tt" fallback in zbut_resolve_tabtarget

**[260210-0926] bridled**

Refactor but_test.sh to introduce three assertion families: but_unit_*, but_tt_*, but_launch_*.

## Motivation

Test assertions are currently split between BUT (raw commands) and BUZ zipper (tabtarget dispatch). The zipper reinvented expect_ok/expect_fail using buc_die instead of BUT primitives. Consolidate all test assertions into but_test.sh with clear family prefixes that distinguish invocation mechanism.

## Three assertion families

| Family | Invokes via | Entry point | Requires |
|--------|------------|-------------|----------|
| `but_unit_*` | `zbut_invoke` | Raw command | Nothing (self-contained) |
| `but_tt_*` | Tabtarget file | `tt/{colophon}.*.sh` (must exist) | BUZ sourced |
| `but_launch_*` | BUK dispatch | Workbench routing (no tt/ file) | BURD sourced |

- `but_unit_*` — renamed from current `but_expect_*`. Tests raw commands.
- `but_tt_*` — moved from `buz_dispatch_expect_ok/fail`. Tests tabtarget wiring. File must exist in tt/.
- `but_launch_*` — new. Routes colophon through BURD workbench dispatch without requiring tt/ file. For testing workbench routing of commands that don't need operator-facing tabtargets (e.g., test-only ark operations).

## Function signatures

### but_unit_* (raw commands)

```bash
but_unit_expect_ok         <command> [args...]
but_unit_expect_fatal      <command> [args...]
but_unit_expect_ok_stdout  <expected_output> <command> [args...]
```

Renamed from current `but_expect_ok`, `but_expect_fatal`, `but_expect_ok_stdout`. Internally uses `zbut_invoke` to capture stdout/stderr/status.

### but_tt_* (tabtarget file invocation)

```bash
but_tt_expect_ok    <colophon> [args...]
but_tt_expect_fatal <colophon> [args...]
```

Resolves colophon to `tt/{colophon}.*.sh` file (must exist, dies if missing). Extra args pass through to the tabtarget. Examples:

```bash
but_tt_expect_ok    "rbw-rnr" nsproto     # tt/rbw-rnr.RenderNameplateRegime.sh nsproto
but_tt_expect_fatal "rbw-rnv" bogus       # expect validate to fail on bad nameplate
```

### but_launch_* (workbench dispatch, no tt/ file required)

```bash
but_launch_expect_ok    <launcher> <colophon> [args...]
but_launch_expect_fatal <launcher> <colophon> [args...]
```

First arg is the workbench (launcher), second is the colophon it routes, rest are args. Explicit launcher because workbench can't always be derived from colophon. Examples:

```bash
but_launch_expect_ok    "rbw_workbench" "rbw-rnr" nsproto
but_launch_expect_fatal "rbw_workbench" "rbw-rnr" bogus
```

## Naming conventions

- Use `_fatal` not `_fail` (matches BUT convention — means "die on assertion failure")
- `but_*` shared infrastructure (colors, rendering, but_fatal, but_execute) stays unchanged
- Double-prefix (`but_unit_`, `but_tt_`, `but_launch_`) is allowed within test rigging

## Files in scope

Modify:
- Tools/buk/but_test.sh — add but_tt_* and but_launch_* families, rename but_expect_* to but_unit_expect_*
- Tools/buk/buz_zipper.sh — remove buz_dispatch_expect_ok and buz_dispatch_expect_fail
- All testbenches calling but_expect_* or buz_dispatch_expect_* — update to new names

## Design notes

- but_test.sh does NOT source BUZ or BUC. The but_tt_* and but_launch_* functions assume the caller's environment has sourced the necessary dispatch infrastructure.
- but_tt_* resolves colophon to tt/ file (like current zbuz_resolve_tabtarget_capture), dies if file missing. Resolution logic moves from buz_zipper.sh into but_test.sh.
- but_launch_* invokes the launcher (workbench) directly, passing colophon and args. Check how burd_dispatch.sh resolves colophons to understand the invocation pattern.
- All three families capture exit status for assertion. but_tt_* and but_launch_* use BURV-isolated invocation when available (caller's responsibility to set up BURV environment).

## Verification

- All existing testbench tests pass with renamed functions
- but_tt_expect_ok dies when tt/ file is missing for given colophon
- but_tt_expect_ok passes args through to tabtarget (e.g., nameplate moniker)
- but_launch_expect_ok routes through workbench without tt/ file
- but_launch_expect_ok passes colophon + args through to launcher
- No but_expect_ok (old name) calls remain in codebase
- No buz_dispatch_expect_ok/fail calls remain in codebase
- but_test.sh has no BUC sourcing

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/buk/but_test.sh, Tools/buk/buz_zipper.sh, Tools/test/tbvu_suite_xname.sh, Tools/test/trbim_suite.sh, Tools/rbw/rbt_testbench.sh, Tools/rbw/rbtg_testbench.sh (6 files) | Steps: 1. Read but_test.sh and buz_zipper.sh to understand current implementations 2. Add but_tt_expect_ok and but_tt_expect_fatal to but_test.sh -- resolve colophon to tt/ file via glob, die if missing, invoke with extra args, assert exit status 3. Add but_launch_expect_ok and but_launch_expect_fatal to but_test.sh -- first arg is launcher workbench, second is colophon, rest are args, invoke launcher with colophon+args, assert exit status 4. Rename but_expect_ok to but_unit_expect_ok, but_expect_fatal to but_unit_expect_fatal, but_expect_ok_stdout to but_unit_expect_ok_stdout in but_test.sh 5. Remove buz_dispatch_expect_ok and buz_dispatch_expect_fail from buz_zipper.sh 6. Update tbvu_suite_xname.sh -- all but_expect_ok_stdout to but_unit_expect_ok_stdout, all but_expect_fatal to but_unit_expect_fatal, all but_expect_ok to but_unit_expect_ok 7. Update trbim_suite.sh -- all but_expect_ok to but_unit_expect_ok 8. Update rbt_testbench.sh -- all but_expect_ok to but_unit_expect_ok, all but_expect_fatal to but_unit_expect_fatal, update comment on line 113 9. Update rbtg_testbench.sh -- all buz_dispatch_expect_ok to but_tt_expect_ok | Verify: grep -r but_expect_ok Tools and grep -r buz_dispatch_expect Tools should return zero matches for old names. but_test.sh must not source buc or buz.

**[260210-0923] rough**

Refactor but_test.sh to introduce three assertion families: but_unit_*, but_tt_*, but_launch_*.

## Motivation

Test assertions are currently split between BUT (raw commands) and BUZ zipper (tabtarget dispatch). The zipper reinvented expect_ok/expect_fail using buc_die instead of BUT primitives. Consolidate all test assertions into but_test.sh with clear family prefixes that distinguish invocation mechanism.

## Three assertion families

| Family | Invokes via | Entry point | Requires |
|--------|------------|-------------|----------|
| `but_unit_*` | `zbut_invoke` | Raw command | Nothing (self-contained) |
| `but_tt_*` | Tabtarget file | `tt/{colophon}.*.sh` (must exist) | BUZ sourced |
| `but_launch_*` | BUK dispatch | Workbench routing (no tt/ file) | BURD sourced |

- `but_unit_*` — renamed from current `but_expect_*`. Tests raw commands.
- `but_tt_*` — moved from `buz_dispatch_expect_ok/fail`. Tests tabtarget wiring. File must exist in tt/.
- `but_launch_*` — new. Routes colophon through BURD workbench dispatch without requiring tt/ file. For testing workbench routing of commands that don't need operator-facing tabtargets (e.g., test-only ark operations).

## Function signatures

### but_unit_* (raw commands)

```bash
but_unit_expect_ok         <command> [args...]
but_unit_expect_fatal      <command> [args...]
but_unit_expect_ok_stdout  <expected_output> <command> [args...]
```

Renamed from current `but_expect_ok`, `but_expect_fatal`, `but_expect_ok_stdout`. Internally uses `zbut_invoke` to capture stdout/stderr/status.

### but_tt_* (tabtarget file invocation)

```bash
but_tt_expect_ok    <colophon> [args...]
but_tt_expect_fatal <colophon> [args...]
```

Resolves colophon to `tt/{colophon}.*.sh` file (must exist, dies if missing). Extra args pass through to the tabtarget. Examples:

```bash
but_tt_expect_ok    "rbw-rnr" nsproto     # tt/rbw-rnr.RenderNameplateRegime.sh nsproto
but_tt_expect_fatal "rbw-rnv" bogus       # expect validate to fail on bad nameplate
```

### but_launch_* (workbench dispatch, no tt/ file required)

```bash
but_launch_expect_ok    <launcher> <colophon> [args...]
but_launch_expect_fatal <launcher> <colophon> [args...]
```

First arg is the workbench (launcher), second is the colophon it routes, rest are args. Explicit launcher because workbench can't always be derived from colophon. Examples:

```bash
but_launch_expect_ok    "rbw_workbench" "rbw-rnr" nsproto
but_launch_expect_fatal "rbw_workbench" "rbw-rnr" bogus
```

## Naming conventions

- Use `_fatal` not `_fail` (matches BUT convention — means "die on assertion failure")
- `but_*` shared infrastructure (colors, rendering, but_fatal, but_execute) stays unchanged
- Double-prefix (`but_unit_`, `but_tt_`, `but_launch_`) is allowed within test rigging

## Files in scope

Modify:
- Tools/buk/but_test.sh — add but_tt_* and but_launch_* families, rename but_expect_* to but_unit_expect_*
- Tools/buk/buz_zipper.sh — remove buz_dispatch_expect_ok and buz_dispatch_expect_fail
- All testbenches calling but_expect_* or buz_dispatch_expect_* — update to new names

## Design notes

- but_test.sh does NOT source BUZ or BUC. The but_tt_* and but_launch_* functions assume the caller's environment has sourced the necessary dispatch infrastructure.
- but_tt_* resolves colophon to tt/ file (like current zbuz_resolve_tabtarget_capture), dies if file missing. Resolution logic moves from buz_zipper.sh into but_test.sh.
- but_launch_* invokes the launcher (workbench) directly, passing colophon and args. Check how burd_dispatch.sh resolves colophons to understand the invocation pattern.
- All three families capture exit status for assertion. but_tt_* and but_launch_* use BURV-isolated invocation when available (caller's responsibility to set up BURV environment).

## Verification

- All existing testbench tests pass with renamed functions
- but_tt_expect_ok dies when tt/ file is missing for given colophon
- but_tt_expect_ok passes args through to tabtarget (e.g., nameplate moniker)
- but_launch_expect_ok routes through workbench without tt/ file
- but_launch_expect_ok passes colophon + args through to launcher
- No but_expect_ok (old name) calls remain in codebase
- No buz_dispatch_expect_ok/fail calls remain in codebase
- but_test.sh has no BUC sourcing

**[260210-0920] rough**

Refactor but_test.sh to introduce three assertion families: but_unit_*, but_tt_*, but_launch_*.

## Motivation

Test assertions are currently split between BUT (raw commands) and BUZ zipper (tabtarget dispatch). The zipper reinvented expect_ok/expect_fail using buc_die instead of BUT primitives. Consolidate all test assertions into but_test.sh with clear family prefixes that distinguish invocation mechanism.

## Three assertion families

| Family | Invokes via | Entry point | Requires |
|--------|------------|-------------|----------|
| `but_unit_*` | `zbut_invoke` | Raw command | Nothing (self-contained) |
| `but_tt_*` | Tabtarget file | `tt/{colophon}.*.sh` (must exist) | BUZ sourced |
| `but_launch_*` | BUK dispatch | Workbench routing (no tt/ file) | BURD sourced |

- `but_unit_*` — renamed from current `but_expect_*`. Tests raw commands.
- `but_tt_*` — moved from `buz_dispatch_expect_ok/fail`. Tests tabtarget wiring. File must exist in tt/.
- `but_launch_*` — new. Routes colophon through BURD workbench dispatch without requiring tt/ file. For testing workbench routing of commands that don't need operator-facing tabtargets (e.g., test-only ark operations).

## Functions per family

Each family gets:
- `but_{family}_expect_ok` — invoke, assert exit 0
- `but_{family}_expect_fatal` — invoke, assert exit != 0

Plus unit keeps:
- `but_unit_expect_ok_stdout` — invoke, assert exit 0 + stdout match

## Naming conventions

- Use `_fatal` not `_fail` (matches BUT convention — means "die on assertion failure")
- `but_*` shared infrastructure (colors, rendering, but_fatal, but_execute) stays unchanged
- Double-prefix (`but_unit_`, `but_tt_`, `but_launch_`) is allowed within test rigging

## Files in scope

Modify:
- Tools/buk/but_test.sh — add but_tt_* and but_launch_* families, rename but_expect_* to but_unit_expect_*
- Tools/buk/buz_zipper.sh — remove buz_dispatch_expect_ok and buz_dispatch_expect_fail
- All testbenches calling but_expect_* or buz_dispatch_expect_* — update to new names

## Design notes

- but_test.sh does NOT source BUZ or BUC. The but_tt_* and but_launch_* functions assume the caller's environment has sourced the necessary dispatch infrastructure.
- but_tt_* resolves colophon to tt/ file (like current zbuz_resolve_tabtarget_capture), dies if file missing
- but_launch_* routes through BURD dispatch — check how burd_dispatch.sh resolves colophons to workbenches

## Verification

- All existing testbench tests pass with renamed functions
- but_tt_expect_ok dies when tt/ file is missing for given colophon
- but_launch_expect_ok routes through workbench without tt/ file
- No but_expect_ok (old name) calls remain in codebase
- No buz_dispatch_expect_ok/fail calls remain in codebase
- but_test.sh has no BUC sourcing

### refactor-rbrv-rbrn-render-validate (₢ATAAG) [complete]

**[260210-0206] complete**

Refactor RBRV and RBRN regime infrastructure: separate kindle (state prep) from validate (strict checking), make render the complete diagnostic tool, add regime tabtargets.

## Design decisions (from discussion)

### Three-layer architecture

| Layer | File | Role | Dies? |
|-------|------|------|-------|
| **kindle** | *_regime.sh | Set defaults, detect unexpected vars, build rollup | no |
| **validate** | *_cli.sh | Call buv_env_* for strict type checking + die on unexpected | yes |
| **render** | *_cli.sh | Display all fields annotated with values/descriptions/warnings, then call validate | yes, at end |

- kindle does NOT call buv_env_* — those move exclusively to validate
- kindle detects unexpected PREFIX_* variables, records them (does not die)
- validate reads kindle's unexpected-var finding and dies if any
- render calls kindle, displays everything, then calls validate at end
- info is removed — render subsumes it (shows descriptions alongside live values)

### CLI operations per regime (was 3, now 2)

| Command | Shows values? | Shows descriptions? | Validates? | Dies on error? |
|---------|--------------|-------------------|------------|----------------|
| **render** | yes | yes | yes, at end | yes, after display |
| **validate** | no | no | yes | yes, immediately |

### Tabtarget naming

Pattern: `rbw-r{x}{op}.{Frontispiece}.sh` where x = regime letter, op = r(ender) or v(alidate)

| Regime | Render | Validate | Cardinality |
|--------|--------|----------|-------------|
| Nameplate (n) | rbw-rnr.RenderNameplateRegime.sh | rbw-rnv.ValidateNameplateRegime.sh | N (arg = moniker) |
| Vessel (v) | rbw-rvr.RenderVesselRegime.sh | rbw-rvv.ValidateVesselRegime.sh | N (arg = sigil) |

Cardinality-N behavior: first arg selects instance, no arg lists available instances.

## Files in scope

Modify:
- Tools/rbw/rbrv_regime.sh — refactor zrbrv_kindle(): remove buv_env_*, add unexpected-var detection
- Tools/rbw/rbrv_cli.sh — refactor rbrv_validate() to own buv_env_* calls, refactor rbrv_render() as diagnostic display + validate-at-end, remove rbrv_info()
- Tools/rbw/rbrn_regime.sh — refactor zrbrn_kindle(): remove buv_env_*, add unexpected-var detection
- Tools/rbw/rbrn_cli.sh — refactor rbrn_validate() to own buv_env_* calls, refactor rbrn_render() as diagnostic display + validate-at-end, remove rbrn_info()
- Tools/rbw/rbw_workbench.sh — add routing for rbw-rn* and rbw-rv* colophons

Create:
- tt/rbw-rnr.RenderNameplateRegime.sh
- tt/rbw-rnv.ValidateNameplateRegime.sh
- tt/rbw-rvr.RenderVesselRegime.sh
- tt/rbw-rvv.ValidateVesselRegime.sh

## Implementation layers

### Layer 1: Refactor kindle (regime.sh files)
- Strip buv_env_* calls from zrbrv_kindle() and zrbrn_kindle()
- Add unexpected-variable detection: enumerate PREFIX_* vars, compare against known set, record in ZRBRV_UNEXPECTED / ZRBRN_UNEXPECTED
- Keep: default-setting, rollup building, sentinel pattern
- RBRV and RBRN are independent files — parallelizable

### Layer 2: Refactor CLI (cli.sh files)
- validate: call kindle, check unexpected vars (die), run buv_env_* sequence (die on first error)
- render: call kindle, display each field (name + description + value + per-field warnings), display unexpected vars, call validate at end
- Remove info command and its dispatch case
- Add no-arg discovery: for cardinality-N regimes, list available instances when no arg provided
- RBRV and RBRN CLI are independent — parallelizable

### Layer 3: Tabtargets and routing
- Create 4 tabtarget launcher scripts
- Add rbw-rn* and rbw-rv* routing to rbw_workbench.sh
- Ensure workbench extracts instance arg correctly for cardinality-N regimes

## Scope control
- Do NOT modify .adoc or .env files
- Do NOT modify rbob_bottle.sh, rbt_testbench.sh, or other consumers — they call kindle+validate as before
- Do NOT add regime tabtargets for RBRR/RBRP/etc. in this pace (future work, same pattern)
- Programmatic consumers (rbob, rbf) that currently call kindle must be updated to call kindle+validate

## Verification
- Existing programmatic consumers still work (kindle+validate replaces kindle-alone)
- rbrv_render and rbrn_render show full annotated display for valid regimes
- rbrv_render and rbrn_render show full display THEN die for invalid regimes
- Unexpected variables are detected and cause validate to die
- All 4 tabtargets route correctly
- No-arg on cardinality-N tabtargets lists available instances

**[260209-1627] rough**

Refactor RBRV and RBRN regime infrastructure: separate kindle (state prep) from validate (strict checking), make render the complete diagnostic tool, add regime tabtargets.

## Design decisions (from discussion)

### Three-layer architecture

| Layer | File | Role | Dies? |
|-------|------|------|-------|
| **kindle** | *_regime.sh | Set defaults, detect unexpected vars, build rollup | no |
| **validate** | *_cli.sh | Call buv_env_* for strict type checking + die on unexpected | yes |
| **render** | *_cli.sh | Display all fields annotated with values/descriptions/warnings, then call validate | yes, at end |

- kindle does NOT call buv_env_* — those move exclusively to validate
- kindle detects unexpected PREFIX_* variables, records them (does not die)
- validate reads kindle's unexpected-var finding and dies if any
- render calls kindle, displays everything, then calls validate at end
- info is removed — render subsumes it (shows descriptions alongside live values)

### CLI operations per regime (was 3, now 2)

| Command | Shows values? | Shows descriptions? | Validates? | Dies on error? |
|---------|--------------|-------------------|------------|----------------|
| **render** | yes | yes | yes, at end | yes, after display |
| **validate** | no | no | yes | yes, immediately |

### Tabtarget naming

Pattern: `rbw-r{x}{op}.{Frontispiece}.sh` where x = regime letter, op = r(ender) or v(alidate)

| Regime | Render | Validate | Cardinality |
|--------|--------|----------|-------------|
| Nameplate (n) | rbw-rnr.RenderNameplateRegime.sh | rbw-rnv.ValidateNameplateRegime.sh | N (arg = moniker) |
| Vessel (v) | rbw-rvr.RenderVesselRegime.sh | rbw-rvv.ValidateVesselRegime.sh | N (arg = sigil) |

Cardinality-N behavior: first arg selects instance, no arg lists available instances.

## Files in scope

Modify:
- Tools/rbw/rbrv_regime.sh — refactor zrbrv_kindle(): remove buv_env_*, add unexpected-var detection
- Tools/rbw/rbrv_cli.sh — refactor rbrv_validate() to own buv_env_* calls, refactor rbrv_render() as diagnostic display + validate-at-end, remove rbrv_info()
- Tools/rbw/rbrn_regime.sh — refactor zrbrn_kindle(): remove buv_env_*, add unexpected-var detection
- Tools/rbw/rbrn_cli.sh — refactor rbrn_validate() to own buv_env_* calls, refactor rbrn_render() as diagnostic display + validate-at-end, remove rbrn_info()
- Tools/rbw/rbw_workbench.sh — add routing for rbw-rn* and rbw-rv* colophons

Create:
- tt/rbw-rnr.RenderNameplateRegime.sh
- tt/rbw-rnv.ValidateNameplateRegime.sh
- tt/rbw-rvr.RenderVesselRegime.sh
- tt/rbw-rvv.ValidateVesselRegime.sh

## Implementation layers

### Layer 1: Refactor kindle (regime.sh files)
- Strip buv_env_* calls from zrbrv_kindle() and zrbrn_kindle()
- Add unexpected-variable detection: enumerate PREFIX_* vars, compare against known set, record in ZRBRV_UNEXPECTED / ZRBRN_UNEXPECTED
- Keep: default-setting, rollup building, sentinel pattern
- RBRV and RBRN are independent files — parallelizable

### Layer 2: Refactor CLI (cli.sh files)
- validate: call kindle, check unexpected vars (die), run buv_env_* sequence (die on first error)
- render: call kindle, display each field (name + description + value + per-field warnings), display unexpected vars, call validate at end
- Remove info command and its dispatch case
- Add no-arg discovery: for cardinality-N regimes, list available instances when no arg provided
- RBRV and RBRN CLI are independent — parallelizable

### Layer 3: Tabtargets and routing
- Create 4 tabtarget launcher scripts
- Add rbw-rn* and rbw-rv* routing to rbw_workbench.sh
- Ensure workbench extracts instance arg correctly for cardinality-N regimes

## Scope control
- Do NOT modify .adoc or .env files
- Do NOT modify rbob_bottle.sh, rbt_testbench.sh, or other consumers — they call kindle+validate as before
- Do NOT add regime tabtargets for RBRR/RBRP/etc. in this pace (future work, same pattern)
- Programmatic consumers (rbob, rbf) that currently call kindle must be updated to call kindle+validate

## Verification
- Existing programmatic consumers still work (kindle+validate replaces kindle-alone)
- rbrv_render and rbrn_render show full annotated display for valid regimes
- rbrv_render and rbrn_render show full display THEN die for invalid regimes
- Unexpected variables are detected and cause validate to die
- All 4 tabtargets route correctly
- No-arg on cardinality-N tabtargets lists available instances

**[260209-1627] rough**

Refactor RBRV and RBRN regime infrastructure: separate kindle (state prep) from validate (strict checking), make render the complete diagnostic tool, add regime tabtargets.

## Design decisions (from discussion)

### Three-layer architecture

| Layer | File | Role | Dies? |
|-------|------|------|-------|
| **kindle** | *_regime.sh | Set defaults, detect unexpected vars, build rollup | no |
| **validate** | *_cli.sh | Call buv_env_* for strict type checking + die on unexpected | yes |
| **render** | *_cli.sh | Display all fields annotated with values/descriptions/warnings, then call validate | yes, at end |

- kindle does NOT call buv_env_* — those move exclusively to validate
- kindle detects unexpected PREFIX_* variables, records them (does not die)
- validate reads kindle's unexpected-var finding and dies if any
- render calls kindle, displays everything, then calls validate at end
- info is removed — render subsumes it (shows descriptions alongside live values)

### CLI operations per regime (was 3, now 2)

| Command | Shows values? | Shows descriptions? | Validates? | Dies on error? |
|---------|--------------|-------------------|------------|----------------|
| **render** | yes | yes | yes, at end | yes, after display |
| **validate** | no | no | yes | yes, immediately |

### Tabtarget naming

Pattern: `rbw-r{x}{op}.{Frontispiece}.sh` where x = regime letter, op = r(ender) or v(alidate)

| Regime | Render | Validate | Cardinality |
|--------|--------|----------|-------------|
| Nameplate (n) | rbw-rnr.RenderNameplateRegime.sh | rbw-rnv.ValidateNameplateRegime.sh | N (arg = moniker) |
| Vessel (v) | rbw-rvr.RenderVesselRegime.sh | rbw-rvv.ValidateVesselRegime.sh | N (arg = sigil) |

Cardinality-N behavior: first arg selects instance, no arg lists available instances.

## Files in scope

Modify:
- Tools/rbw/rbrv_regime.sh — refactor zrbrv_kindle(): remove buv_env_*, add unexpected-var detection
- Tools/rbw/rbrv_cli.sh — refactor rbrv_validate() to own buv_env_* calls, refactor rbrv_render() as diagnostic display + validate-at-end, remove rbrv_info()
- Tools/rbw/rbrn_regime.sh — refactor zrbrn_kindle(): remove buv_env_*, add unexpected-var detection
- Tools/rbw/rbrn_cli.sh — refactor rbrn_validate() to own buv_env_* calls, refactor rbrn_render() as diagnostic display + validate-at-end, remove rbrn_info()
- Tools/rbw/rbw_workbench.sh — add routing for rbw-rn* and rbw-rv* colophons

Create:
- tt/rbw-rnr.RenderNameplateRegime.sh
- tt/rbw-rnv.ValidateNameplateRegime.sh
- tt/rbw-rvr.RenderVesselRegime.sh
- tt/rbw-rvv.ValidateVesselRegime.sh

## Implementation layers

### Layer 1: Refactor kindle (regime.sh files)
- Strip buv_env_* calls from zrbrv_kindle() and zrbrn_kindle()
- Add unexpected-variable detection: enumerate PREFIX_* vars, compare against known set, record in ZRBRV_UNEXPECTED / ZRBRN_UNEXPECTED
- Keep: default-setting, rollup building, sentinel pattern
- RBRV and RBRN are independent files — parallelizable

### Layer 2: Refactor CLI (cli.sh files)
- validate: call kindle, check unexpected vars (die), run buv_env_* sequence (die on first error)
- render: call kindle, display each field (name + description + value + per-field warnings), display unexpected vars, call validate at end
- Remove info command and its dispatch case
- Add no-arg discovery: for cardinality-N regimes, list available instances when no arg provided
- RBRV and RBRN CLI are independent — parallelizable

### Layer 3: Tabtargets and routing
- Create 4 tabtarget launcher scripts
- Add rbw-rn* and rbw-rv* routing to rbw_workbench.sh
- Ensure workbench extracts instance arg correctly for cardinality-N regimes

## Scope control
- Do NOT modify .adoc or .env files
- Do NOT modify rbob_bottle.sh, rbt_testbench.sh, or other consumers — they call kindle+validate as before
- Do NOT add regime tabtargets for RBRR/RBRP/etc. in this pace (future work, same pattern)
- Programmatic consumers (rbob, rbf) that currently call kindle must be updated to call kindle+validate

## Verification
- Existing programmatic consumers still work (kindle+validate replaces kindle-alone)
- rbrv_render and rbrn_render show full annotated display for valid regimes
- rbrv_render and rbrn_render show full display THEN die for invalid regimes
- Unexpected variables are detected and cause validate to die
- All 4 tabtargets route correctly
- No-arg on cardinality-N tabtargets lists available instances

**[260209-1521] rough**

Synchronize RBRV and RBRN verification implementation and tabtarget coverage across all three services.

## Motivation

The rest of ₣AT is about treating all regimes the same way. This pace establishes uniform bash infrastructure for RBRV (vessel) and RBRN (nameplate) so subsequent spec/doc paces build on consistent implementation.

## Current asymmetries

### Tabtarget coverage gaps (service imprints: nsproto, srjcl, pluml)

| Operation         | nsproto | srjcl | pluml |
|-------------------|---------|-------|-------|
| Start             | yes     | yes   | yes   |
| Stop              | yes     | yes   | yes   |
| ObserveNetworks   | yes     | yes   | yes   |
| TestBottleService | yes     | yes   | yes   |
| ConnectBottle     | yes     | yes   | NO    |
| ConnectSentry     | yes     | yes   | yes   |
| ConnectCenser     | yes     | NO    | NO    |

### Test suite asymmetry
- test_nsproto_* — 21 tests (security/DNS/ICMP)
- test_srjcl_* — 3 tests (Jupyter)
- test_pluml_* — 5 tests (PlantUML)

## Bash files in scope

- Tools/rbw/rbrv_regime.sh — zrbrv_kindle(), zrbrv_sentinel()
- Tools/rbw/rbrv_cli.sh — rbrv_validate(), rbrv_render(), rbrv_info()
- Tools/rbw/rbrn_regime.sh — zrbrn_kindle(), zrbrn_sentinel()
- Tools/rbw/rbrn_cli.sh — rbrn_validate(), rbrn_render(), rbrn_info()
- Tools/rbw/rbob_bottle.sh — rbob_start/stop/connect_{sentry,censer,bottle}()
- Tools/rbw/rbob_cli.sh — rbob_validate(), rbob_info(), rbob_observe(), zrbob_furnish()
- Tools/rbw/rboo_observe.sh — rboo_observe(), zrboo_kindle()
- Tools/rbw/rbt_testbench.sh — rbt_suite_{nsproto,srjcl,pluml}(), rbt_load_nameplate(), rbt_route()
- Tools/rbw/rbss.sentry.sh — sentry iptables/DNS config
- Tools/rbw/rbf_Foundry.sh — build orchestration, consumes RBRV

## Tabtargets in scope

Service-imprinted (per-service):
- tt/rbw-s.Start.{nsproto,srjcl,pluml}.sh
- tt/rbw-z.Stop.{nsproto,srjcl,pluml}.sh
- tt/rbw-o.ObserveNetworks.{nsproto,srjcl,pluml}.sh
- tt/rbw-to.TestBottleService.{nsproto,srjcl,pluml}.sh
- tt/rbw-B.ConnectBottle.{nsproto,srjcl}.sh (missing pluml)
- tt/rbw-S.ConnectSentry.{nsproto,srjcl,pluml}.sh
- tt/rbw-C.ConnectCenser.nsproto.sh (missing srjcl, pluml)

## RBRN env files (3 services)
- Tools/rbw/rbrn_nsproto.env, rbrn_srjcl.env, rbrn_pluml.env

## RBRV env files (6 vessels)
- rbev-vessels/{busybox,nginx-ward,pg-keeper,ubu-safety,sentry-ubuntu-large,bottle-ubuntu-test}/rbrv.env

## Related RBS subdocuments (context, not primary focus)
Service lifecycle: RBSSS, RBSBS, RBSBK, RBSBC, RBSBL, RBSBR
Network: RBSNC, RBSNX
Security: RBSSC, RBSAX, RBSIP, RBSDS, RBSPT
Vessel: RBSRV

## Tasks
1. Audit tabtarget gaps: determine which missing Connect tabtargets are intentional vs oversight
2. Fill intentional tabtarget gaps (add missing ConnectBottle.pluml, ConnectCenser.{srjcl,pluml} if appropriate)
3. Synchronize validation patterns between rbrv_cli.sh and rbrn_cli.sh
4. Review rbt_testbench.sh test coverage — determine if srjcl/pluml suites need additional verification tests
5. Ensure kindle/sentinel patterns are consistent across rbrv_regime.sh and rbrn_regime.sh

## Scope control
- Focus is bash implementation, not spec documents
- Do NOT modify .adoc or .env files (those are separate paces)
- Tabtarget additions should follow existing patterns exactly

## Verification
- All three services have identical tabtarget coverage (or documented reasons for differences)
- rbrv and rbrn validation patterns are consistent
- No regressions in existing test suites

### extract-but-dispatch (₢ATAAJ) [abandoned]

**[260210-1232] abandoned**

Extract dispatch/evidence/BURV infrastructure from buz_zipper.sh into but_test.sh.

## Background

buz_zipper.sh currently contains two concerns:
1. Colophon registry (buz_register, zbuz_resolve_tabtarget_capture) — production infrastructure
2. Dispatch/evidence/step-tracking (buz_dispatch, buz_init_evidence, buz_last_step_capture, etc.) — test infrastructure

The dispatch machinery uses buc_* internally (buc_log_args, buc_die), which causes mixed output when called from testbenches that should use but_* output discipline.

## Work

Move the following from buz_zipper.sh to but_test.sh:

| buz_zipper.sh (remove) | but_test.sh (add) |
|---|---|
| buz_init_evidence | but_init_evidence |
| buz_dispatch | but_dispatch |
| buz_last_step_capture | but_last_step_capture |
| buz_get_step_exit_capture | but_get_step_exit_capture |
| buz_get_step_output_capture | but_get_step_output_capture |

Internal changes in the moved code:
- buc_log_args → but_info
- buc_die → but_fatal
- buc_warn → but_info (or remove)
- zbuz_sentinel calls → but-local sentinel or remove (but_dispatch validates its own state)
- Tabtarget resolution: use zbut_resolve_tabtarget (already in but_test.sh) instead of zbuz_resolve_tabtarget_capture
- Step arrays (zbuz_step_colophons etc.) → zbut_step_colophons etc.
- ZBUZ_EVIDENCE_ROOT → ZBUT_EVIDENCE_ROOT

buz_zipper.sh retains ONLY:
- zbuz_kindle (registry arrays: zbuz_colophons, zbuz_modules, zbuz_commands, zbuz_tabtargets)
- zbuz_sentinel
- zbuz_resolve_tabtarget_capture
- buz_register

Remove the extra blank line left by extraction if any.

## Callers to update

- rbtg_testbench.sh: buz_init_evidence → but_init_evidence, buz_dispatch → but_dispatch, buz_last_step_capture → but_last_step_capture, buz_get_step_exit_capture → but_get_step_exit_capture, buz_get_step_output_capture → but_get_step_output_capture, ZBUZ_EVIDENCE_ROOT → ZBUT_EVIDENCE_ROOT

## Verification

- grep confirms no buz_dispatch/buz_init_evidence/buz_last_step_capture/buz_get_step_exit_capture/buz_get_step_output_capture remain in buz_zipper.sh
- grep confirms no ZBUZ_EVIDENCE_ROOT in rbtg_testbench.sh
- buz_zipper.sh contains only registry functions
- but_test.sh contains dispatch/evidence functions using but_* output discipline
- rbtg_testbench.sh calls but_* dispatch functions

**[260210-1048] bridled**

Extract dispatch/evidence/BURV infrastructure from buz_zipper.sh into but_test.sh.

## Background

buz_zipper.sh currently contains two concerns:
1. Colophon registry (buz_register, zbuz_resolve_tabtarget_capture) — production infrastructure
2. Dispatch/evidence/step-tracking (buz_dispatch, buz_init_evidence, buz_last_step_capture, etc.) — test infrastructure

The dispatch machinery uses buc_* internally (buc_log_args, buc_die), which causes mixed output when called from testbenches that should use but_* output discipline.

## Work

Move the following from buz_zipper.sh to but_test.sh:

| buz_zipper.sh (remove) | but_test.sh (add) |
|---|---|
| buz_init_evidence | but_init_evidence |
| buz_dispatch | but_dispatch |
| buz_last_step_capture | but_last_step_capture |
| buz_get_step_exit_capture | but_get_step_exit_capture |
| buz_get_step_output_capture | but_get_step_output_capture |

Internal changes in the moved code:
- buc_log_args → but_info
- buc_die → but_fatal
- buc_warn → but_info (or remove)
- zbuz_sentinel calls → but-local sentinel or remove (but_dispatch validates its own state)
- Tabtarget resolution: use zbut_resolve_tabtarget (already in but_test.sh) instead of zbuz_resolve_tabtarget_capture
- Step arrays (zbuz_step_colophons etc.) → zbut_step_colophons etc.
- ZBUZ_EVIDENCE_ROOT → ZBUT_EVIDENCE_ROOT

buz_zipper.sh retains ONLY:
- zbuz_kindle (registry arrays: zbuz_colophons, zbuz_modules, zbuz_commands, zbuz_tabtargets)
- zbuz_sentinel
- zbuz_resolve_tabtarget_capture
- buz_register

Remove the extra blank line left by extraction if any.

## Callers to update

- rbtg_testbench.sh: buz_init_evidence → but_init_evidence, buz_dispatch → but_dispatch, buz_last_step_capture → but_last_step_capture, buz_get_step_exit_capture → but_get_step_exit_capture, buz_get_step_output_capture → but_get_step_output_capture, ZBUZ_EVIDENCE_ROOT → ZBUT_EVIDENCE_ROOT

## Verification

- grep confirms no buz_dispatch/buz_init_evidence/buz_last_step_capture/buz_get_step_exit_capture/buz_get_step_output_capture remain in buz_zipper.sh
- grep confirms no ZBUZ_EVIDENCE_ROOT in rbtg_testbench.sh
- buz_zipper.sh contains only registry functions
- but_test.sh contains dispatch/evidence functions using but_* output discipline
- rbtg_testbench.sh calls but_* dispatch functions

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: buz_zipper.sh, but_test.sh, rbtg_testbench.sh (3 files) | Steps: 1. Extract dispatch/evidence functions from buz_zipper.sh lines 107-191 into but_test.sh, renaming buz_ to but_ and zbuz_ to zbut_ for step arrays and evidence root, replacing buc_log_args with but_info, buc_die with but_fatal, buc_warn with but_info, using zbut_resolve_tabtarget instead of zbuz_resolve_tabtarget_capture, adding zbut_step_colophons/zbut_step_exit_status/zbut_step_output_dir arrays and ZBUT_EVIDENCE_ROOT to but_init_evidence 2. Remove step result arrays from zbuz_kindle, remove dispatch/evidence/step sections from buz_zipper.sh keeping only registry functions zbuz_kindle zbuz_sentinel zbuz_resolve_tabtarget_capture buz_register 3. Update rbtg_testbench.sh callers: buz_init_evidence to but_init_evidence, buz_dispatch to but_dispatch, buz_last_step_capture to but_last_step_capture, buz_get_step_exit_capture to but_get_step_exit_capture, buz_get_step_output_capture to but_get_step_output_capture, ZBUZ_EVIDENCE_ROOT to ZBUT_EVIDENCE_ROOT | Verify: grep confirms no buz_dispatch or buz_init_evidence or buz_last_step_capture or buz_get_step_exit_capture or buz_get_step_output_capture in buz_zipper.sh, grep confirms no ZBUZ_EVIDENCE_ROOT in rbtg_testbench.sh

**[260210-1045] rough**

Extract dispatch/evidence/BURV infrastructure from buz_zipper.sh into but_test.sh.

## Background

buz_zipper.sh currently contains two concerns:
1. Colophon registry (buz_register, zbuz_resolve_tabtarget_capture) — production infrastructure
2. Dispatch/evidence/step-tracking (buz_dispatch, buz_init_evidence, buz_last_step_capture, etc.) — test infrastructure

The dispatch machinery uses buc_* internally (buc_log_args, buc_die), which causes mixed output when called from testbenches that should use but_* output discipline.

## Work

Move the following from buz_zipper.sh to but_test.sh:

| buz_zipper.sh (remove) | but_test.sh (add) |
|---|---|
| buz_init_evidence | but_init_evidence |
| buz_dispatch | but_dispatch |
| buz_last_step_capture | but_last_step_capture |
| buz_get_step_exit_capture | but_get_step_exit_capture |
| buz_get_step_output_capture | but_get_step_output_capture |

Internal changes in the moved code:
- buc_log_args → but_info
- buc_die → but_fatal
- buc_warn → but_info (or remove)
- zbuz_sentinel calls → but-local sentinel or remove (but_dispatch validates its own state)
- Tabtarget resolution: use zbut_resolve_tabtarget (already in but_test.sh) instead of zbuz_resolve_tabtarget_capture
- Step arrays (zbuz_step_colophons etc.) → zbut_step_colophons etc.
- ZBUZ_EVIDENCE_ROOT → ZBUT_EVIDENCE_ROOT

buz_zipper.sh retains ONLY:
- zbuz_kindle (registry arrays: zbuz_colophons, zbuz_modules, zbuz_commands, zbuz_tabtargets)
- zbuz_sentinel
- zbuz_resolve_tabtarget_capture
- buz_register

Remove the extra blank line left by extraction if any.

## Callers to update

- rbtg_testbench.sh: buz_init_evidence → but_init_evidence, buz_dispatch → but_dispatch, buz_last_step_capture → but_last_step_capture, buz_get_step_exit_capture → but_get_step_exit_capture, buz_get_step_output_capture → but_get_step_output_capture, ZBUZ_EVIDENCE_ROOT → ZBUT_EVIDENCE_ROOT

## Verification

- grep confirms no buz_dispatch/buz_init_evidence/buz_last_step_capture/buz_get_step_exit_capture/buz_get_step_output_capture remain in buz_zipper.sh
- grep confirms no ZBUZ_EVIDENCE_ROOT in rbtg_testbench.sh
- buz_zipper.sh contains only registry functions
- but_test.sh contains dispatch/evidence functions using but_* output discipline
- rbtg_testbench.sh calls but_* dispatch functions

### exercise-but-suites (₢ATAAS) [complete]

**[260212-0700] complete**

Blocked on ₣Ac (but-test-overhaul). That heat redesigns butr_/buto_/butd_ infrastructure including explicit case registration, init/setup two-phase model, and subdispatch logging. Once ₣Ac completes, revisit this pace to exercise the suites under the new framework. Original intent absorbed into ₣Ac paddock and final pace ₢AcAAF (exercise-revised-test-suites).

**[260211-0859] rough**

Blocked on ₣Ac (but-test-overhaul). That heat redesigns butr_/buto_/butd_ infrastructure including explicit case registration, init/setup two-phase model, and subdispatch logging. Once ₣Ac completes, revisit this pace to exercise the suites under the new framework. Original intent absorbed into ₣Ac paddock and final pace ₢AcAAF (exercise-revised-test-suites).

**[260210-1338] rough**

Human pace: manually run all BUT framework testbench suites, review output, and get comfortable with the butc*/butr/buto patterns. Amend anything that feels wrong before moving on to the audit paces.

### audit-regime-validation-consumers (₢ATAAH) [complete]

**[260212-0827] complete**

Audit and fix how programmatic consumers call regime validation functions.
Depends on ₢ATAAJ (extract-but-dispatch) completing first.

Known items from ₢ATAAG work, refined by ₢ATAAI/₢ATAAJ analysis:

1. buv echo suppression: rbrn_load_moniker (rbrn_regime.sh:208) and
   rbrn_load_file (rbrn_regime.sh:221) call zrbrn_validate_fields without
   > /dev/null. Every buv_val_* echoes the validated value to stdout by
   design. All downstream consumers (rbob_cli.sh, rbt_testbench.sh) inherit
   the leak. Fix: suppress in rbrn_load_moniker/rbrn_load_file (one-line
   fix at the source, not in each consumer). Same pattern likely needed for
   rbrv equivalent.

2. return 0 vs exit 0 in rbw_workbench.sh discovery listing (lines 127, 151):
   Normal path uses exec (never returns). Discovery path uses return 0.
   Verify this is correct given BUD's exec chain expectations.

3. buc_step vs echo in render valid message: rbrn_cli.sh:49 uses buc_step
   (stderr) for standalone validate. rbrn_cli.sh:140 uses echo (stdout) for
   render+validate to avoid stderr/stdout ordering through BUD pipe buffering.
   Same in rbrv_cli.sh:60 vs :131. Decide if split is intentional and
   document, or unify.

4. Testbench output discipline policy (post ₢ATAAJ):
   a. Audit rbt_testbench.sh for buc_* calls that should become but_*.
      With but_dispatch now in but_test.sh (from ₢ATAAJ), normal testbenches
      have clean separation through the exec gate.
   b. Evaluate rbtg_testbench.sh future: rbtg tests the dispatch/evidence
      infrastructure directly (now but_dispatch). dispatch_exercise is a
      test-of-test-infrastructure — its ongoing value drops once but_dispatch
      is proven. ark_lifecycle is a real integration test that should become a
      conventional testbench using but_dispatch or but_tt_*. Consider retiring
      rbtg and migrating ark_lifecycle to a conventional testbench.
   c. Define but_success (no equivalent exists — zbut_case uses inline green
      echo at but_test.sh:288).
   d. Establish policy: "normal testbenches use but_* exclusively for own
      output; BCG-internal test-of-test cases are the only exception."

CRITICAL: Items 1-3 and 4b-4d are HUMAN DECISIONS. Item 4a is mechanical
once decided. Present options and tradeoffs, then ask the user to decide.

**[260210-1046] rough**

Audit and fix how programmatic consumers call regime validation functions.
Depends on ₢ATAAJ (extract-but-dispatch) completing first.

Known items from ₢ATAAG work, refined by ₢ATAAI/₢ATAAJ analysis:

1. buv echo suppression: rbrn_load_moniker (rbrn_regime.sh:208) and
   rbrn_load_file (rbrn_regime.sh:221) call zrbrn_validate_fields without
   > /dev/null. Every buv_val_* echoes the validated value to stdout by
   design. All downstream consumers (rbob_cli.sh, rbt_testbench.sh) inherit
   the leak. Fix: suppress in rbrn_load_moniker/rbrn_load_file (one-line
   fix at the source, not in each consumer). Same pattern likely needed for
   rbrv equivalent.

2. return 0 vs exit 0 in rbw_workbench.sh discovery listing (lines 127, 151):
   Normal path uses exec (never returns). Discovery path uses return 0.
   Verify this is correct given BUD's exec chain expectations.

3. buc_step vs echo in render valid message: rbrn_cli.sh:49 uses buc_step
   (stderr) for standalone validate. rbrn_cli.sh:140 uses echo (stdout) for
   render+validate to avoid stderr/stdout ordering through BUD pipe buffering.
   Same in rbrv_cli.sh:60 vs :131. Decide if split is intentional and
   document, or unify.

4. Testbench output discipline policy (post ₢ATAAJ):
   a. Audit rbt_testbench.sh for buc_* calls that should become but_*.
      With but_dispatch now in but_test.sh (from ₢ATAAJ), normal testbenches
      have clean separation through the exec gate.
   b. Evaluate rbtg_testbench.sh future: rbtg tests the dispatch/evidence
      infrastructure directly (now but_dispatch). dispatch_exercise is a
      test-of-test-infrastructure — its ongoing value drops once but_dispatch
      is proven. ark_lifecycle is a real integration test that should become a
      conventional testbench using but_dispatch or but_tt_*. Consider retiring
      rbtg and migrating ark_lifecycle to a conventional testbench.
   c. Define but_success (no equivalent exists — zbut_case uses inline green
      echo at but_test.sh:288).
   d. Establish policy: "normal testbenches use but_* exclusively for own
      output; BCG-internal test-of-test cases are the only exception."

CRITICAL: Items 1-3 and 4b-4d are HUMAN DECISIONS. Item 4a is mechanical
once decided. Present options and tradeoffs, then ask the user to decide.

**[260210-1019] rough**

Audit and fix how programmatic consumers call regime validation functions.

Known items from ₢ATAAG work:

1. buv echo suppression: rbob_cli.sh:112 and rbt_testbench.sh:62 call
   zrbrn_validate_fields without > /dev/null. Their buv echo output leaks
   into BUD logs. Decide if this matters and what the right fix is.

2. return 0 vs exit 0 in rbw_workbench.sh discovery listing (lines 130, 154):
   Uses return 0 after listing available nameplates/vessels. Verify this is
   correct given BUD's exec chain vs function call invocation.

3. buc_step vs echo in render valid message: rbrn_cli.sh:140 and
   rbrv_cli.sh:131 use echo to stdout for "valid" messages, while
   rbrn_cli.sh:49 and rbrv_cli.sh:60 use buc_step to stderr for the same
   message. The echo variant avoids stderr/stdout ordering issues through
   BUD pipe buffering. Decide if this is the right long-term pattern or if
   a buc-level fix is better. This is a production code question, not a
   testbench question.

4. Testbench output discipline policy. The ₢ATAAI BUT refactoring
   established three test assertion families (but_unit_*, but_tt_*,
   but_launch_*) plus test output functions (but_section, but_info,
   but_trace, but_fatal). All three families run through zbut_invoke
   which captures stdout/stderr in a subshell — this is the exec gate
   that separates test output (but_*) from production output (buc_*).

   Key architectural finding: testbenches that use BUT families have clean
   separation and SHOULD use but_* exclusively for their own output.
   However, rbtg_testbench.sh calls buz_dispatch and buz_* functions
   directly in-process (no exec gate) because it tests the dispatch/evidence
   infrastructure itself. buz internals call buc_die/buc_log_args in the
   same process, so rbtg's stderr mixes but_* and buc_* with no boundary.

   Decision: rbtg stays on buc_* (it operates inside the BCG module world).
   Normal testbenches (like rbt_testbench.sh) that use BUT families should
   migrate to but_* output exclusively.

   Sub-items:
   a. Audit rbt_testbench.sh for buc_* calls that should become but_*
   b. Define but_success (no equivalent exists yet — zbut_case uses inline
      green echo at but_test.sh:288, need a public function or pattern)
   c. Establish policy: "normal testbenches use but_*, BCG-internal
      testbenches (rbtg) use buc_*"

CRITICAL: Items 1-3 and 4c are HUMAN DECISIONS about internal factoring and
output discipline. Item 4a is mechanical once 4b and 4c are decided. Do not
presume answers — present options and tradeoffs, then ask the user to decide.

**[260209-1654] rough**

Audit and fix how programmatic consumers call regime validation functions.

Known items from ₢ATAAG work:

1. buv echo suppression: rbob_cli.sh:112 and rbt_testbench.sh:62 call
   zrbrn_validate_fields without > /dev/null. Their buv echo output leaks
   into BUD logs. Decide if this matters and what the right fix is.

2. return 0 vs exit 0 in rbw_workbench.sh discovery listing (lines 130, 154):
   Uses return 0 after listing available nameplates/vessels. Verify this is
   correct given BUD's exec chain vs function call invocation.

3. buc_step vs echo in render valid message: The current fix uses echo to
   stdout to avoid stderr/stdout ordering issues through BUD pipe buffering.
   Decide if this is the right long-term pattern or if a buc-level fix is
   better.

4. Testbench factoring: rbt_testbench.sh may have excellent reasons NOT to
   use buc at all. Evaluate whether testbenches should have their own output
   discipline separate from buc.

CRITICAL: All of these are HUMAN DECISIONS about internal factoring and output
discipline. Do not presume answers — present options and tradeoffs, then ask
the user to decide.

### study-buv-env-stdout-pattern (₢ATAAQ) [complete]

**[260212-0722] complete**

Study whether buv_env_* validators should echo validated values to stdout.

## Context

The buv_env_* functions (buv_env_xname, buv_env_string, etc.) in buv_validation.sh echo the validated value to stdout on success. This was designed for capture patterns like `result=$(buv_env_xname ...)`, but most callers (notably zrbrr_kindle) call them without capturing, spilling raw values to stdout.

Currently rbrv_cli.sh and rbrn_cli.sh work around this by redirecting `zrbrv_validate_fields > /dev/null`. This `> /dev/null` pattern is a code smell — it suppresses output because the API contract is wrong, not because the output is intentionally discardable.

## Work

1. Audit all callers of buv_env_* across the codebase — how many capture the return value vs discard it?
2. Determine if the echo-to-stdout contract is actually used by anyone (capture pattern)
3. If no callers capture: remove the echo from buv_val_* validators, making them silent-on-success / die-on-failure
4. If some callers capture: consider splitting into buv_env_* (silent, for kindle) and buv_val_* (returns value, for capture)
5. Remove all `> /dev/null` suppressions that become unnecessary
6. Verify no regressions in render/validate paths

## Acceptance

- buv_env_* calls in kindle functions produce no stdout noise
- No `> /dev/null` workarounds remain for validation calls
- All existing render and validate tabtargets still work correctly

**[260210-1307] rough**

Study whether buv_env_* validators should echo validated values to stdout.

## Context

The buv_env_* functions (buv_env_xname, buv_env_string, etc.) in buv_validation.sh echo the validated value to stdout on success. This was designed for capture patterns like `result=$(buv_env_xname ...)`, but most callers (notably zrbrr_kindle) call them without capturing, spilling raw values to stdout.

Currently rbrv_cli.sh and rbrn_cli.sh work around this by redirecting `zrbrv_validate_fields > /dev/null`. This `> /dev/null` pattern is a code smell — it suppresses output because the API contract is wrong, not because the output is intentionally discardable.

## Work

1. Audit all callers of buv_env_* across the codebase — how many capture the return value vs discard it?
2. Determine if the echo-to-stdout contract is actually used by anyone (capture pattern)
3. If no callers capture: remove the echo from buv_val_* validators, making them silent-on-success / die-on-failure
4. If some callers capture: consider splitting into buv_env_* (silent, for kindle) and buv_val_* (returns value, for capture)
5. Remove all `> /dev/null` suppressions that become unnecessary
6. Verify no regressions in render/validate paths

## Acceptance

- buv_env_* calls in kindle functions produce no stdout noise
- No `> /dev/null` workarounds remain for validation calls
- All existing render and validate tabtargets still work correctly

### create-rbvr-regime-spec (₢ATAAD) [complete]

**[260212-0924] complete**

Create RBVR (Recipe Bottle Vessel Regime) specification.

RBVR documents the vessel configuration regime—how containers are configured, built, and bound in Recipe Bottle. Currently implemented via:
- rbrv_regime.sh validator (Tools/rbw/)
- rbrv.env instance files in rbev-vessels/
- Support for both binding (pre-built images) and conjuring (build from source)

This pace consolidates the pattern into authoritative spec:
- Vessel identity and sigil requirements
- Binding vs conjuring modes
- Dockerfile and build context configuration
- Multi-platform support (linux/amd64, linux/arm64, etc.)
- binfmt policy discipline
- Variable naming and validation patterns

Depends on: ₣AT ₢ATAAA (regime inventory includes RBRR vessel regime)
Complements: BURS (universal patterns), BURC (bash specifics)
Result: RBVR becomes definitive spec for vessel configuration across Recipe Bottle.

**[260131-1202] rough**

Create RBVR (Recipe Bottle Vessel Regime) specification.

RBVR documents the vessel configuration regime—how containers are configured, built, and bound in Recipe Bottle. Currently implemented via:
- rbrv_regime.sh validator (Tools/rbw/)
- rbrv.env instance files in rbev-vessels/
- Support for both binding (pre-built images) and conjuring (build from source)

This pace consolidates the pattern into authoritative spec:
- Vessel identity and sigil requirements
- Binding vs conjuring modes
- Dockerfile and build context configuration
- Multi-platform support (linux/amd64, linux/arm64, etc.)
- binfmt policy discipline
- Variable naming and validation patterns

Depends on: ₣AT ₢ATAAA (regime inventory includes RBRR vessel regime)
Complements: BURS (universal patterns), BURC (bash specifics)
Result: RBVR becomes definitive spec for vessel configuration across Recipe Bottle.

### process-rbrv-regime-vessel-spec (₢ATAAE) [complete]

**[260212-0934] complete**

Process fresh RBRV-RegimeVessel.adoc document: normalize formatting, validate concept model links, integrate into lens library and update supporting specs (RBAGS, RBRN, RBRR) with new vessel regime terminology.

**[260131-1228] rough**

Process fresh RBRV-RegimeVessel.adoc document: normalize formatting, validate concept model links, integrate into lens library and update supporting specs (RBAGS, RBRN, RBRR) with new vessel regime terminology.

### normalize-regime-operation-specs (₢ATAAZ) [complete]

**[260212-1134] complete**

Normalize and validate the regime operation model additions across AXLA, RBSA, and regime subdocuments.

## Context

Heat-level work in this officium added regime operation concepts to AXLA (axvr_broach/validate/render, axhro_broach/validate/render, axrd_* provenance dimensions) and RBSA (rbkro_*/bukro_* kit operation terms). Regime subdocuments RBSRV and RBRN were updated with //axhro_* markers.

## Work

1. Run /cma-normalize on all changed .adoc files:
   - Tools/cmk/vov_veiled/AXLA-Lexicon.adoc
   - lenses/RBS0-SpecTop.adoc
   - lenses/RBSRV-RegimeVessel.adoc
   - lenses/RBRN-RegimeNameplate.adoc

2. Run /cma-validate on each to check link integrity

3. Verify AXLA internal consistency:
   - All new mapping entries have corresponding anchors
   - All new anchors are referenced
   - axhro_* markers appear in nesting rules table
   - Completeness rules updated for axvr_regime provenance dimension

4. Verify RBSA consistency:
   - rbkro_* and bukro_* mapping entries have anchors with voicings
   - axrd_file_sourced dimension on RBRV and RBRN regime voicings

5. Verify subdocument consistency:
   - RBSRV axhro_* markers reference rbkro_* terms
   - RBRN axhro_* markers reference rbkro_* terms
   - Operation descriptions are regime-specific

## Acceptance

- All four files pass cma-normalize (no changes needed or changes applied)
- All four files pass cma-validate (no errors)
- New terms are properly cross-referenced between AXLA, RBSA, and subdocuments

**[260212-1102] rough**

Normalize and validate the regime operation model additions across AXLA, RBSA, and regime subdocuments.

## Context

Heat-level work in this officium added regime operation concepts to AXLA (axvr_broach/validate/render, axhro_broach/validate/render, axrd_* provenance dimensions) and RBSA (rbkro_*/bukro_* kit operation terms). Regime subdocuments RBSRV and RBRN were updated with //axhro_* markers.

## Work

1. Run /cma-normalize on all changed .adoc files:
   - Tools/cmk/vov_veiled/AXLA-Lexicon.adoc
   - lenses/RBS0-SpecTop.adoc
   - lenses/RBSRV-RegimeVessel.adoc
   - lenses/RBRN-RegimeNameplate.adoc

2. Run /cma-validate on each to check link integrity

3. Verify AXLA internal consistency:
   - All new mapping entries have corresponding anchors
   - All new anchors are referenced
   - axhro_* markers appear in nesting rules table
   - Completeness rules updated for axvr_regime provenance dimension

4. Verify RBSA consistency:
   - rbkro_* and bukro_* mapping entries have anchors with voicings
   - axrd_file_sourced dimension on RBRV and RBRN regime voicings

5. Verify subdocument consistency:
   - RBSRV axhro_* markers reference rbkro_* terms
   - RBRN axhro_* markers reference rbkro_* terms
   - Operation descriptions are regime-specific

## Acceptance

- All four files pass cma-normalize (no changes needed or changes applied)
- All four files pass cma-validate (no errors)
- New terms are properly cross-referenced between AXLA, RBSA, and subdocuments

### implement-rbrr-regime-operation-model (₢ATAAR) [complete]

**[260212-1201] complete**

Implement regime operation model for RBRR: broach/validate split and render CLI.

## Context

AXLA now defines regime operation terms (axvr_broach, axvr_validate, axvr_render) and subdocument hierarchy markers (axhro_broach, axhro_validate, axhro_render). RBSA defines kit-level operation terms (rbkro_broach, rbkro_validate, rbkro_render). RBRN and RBRV both implement the full pattern. RBRR is the laggard.

Pace ATAAZ (normalize-regime-operation-specs) must be complete first — it ensures the AXLA/RBSA/subdocument additions are normalized and validated.

## Work

### 1. Code: Split zrbrr_kindle into broach + validate

Refactor rbrr_regime.sh following RBRN/RBRV pattern:
- `zrbrr_broach()` — set defaults with VAR="${VAR:-}", detect unexpected vars via compgen, build ZRBRR_DOCKER_ENV. Silent.
- `zrbrr_validate_fields()` — buv_env_* calls + format checks (podman version, timeout, platform). Dies on error.
- Update rbrr_load() to call both broach and validate
- Add unexpected variable detection (missing from RBRR today)

### 2. Code: Create rbrr_cli.sh with render/validate commands

Following rbrn_cli.sh and rbrv_cli.sh patterns:
- render command using rbcr_* shared module
- validate command using rbrr_load
- Tabtargets: rbw-rrr.RenderRepoRegime.sh, rbw-rrv.ValidateRepoRegime.sh

### 3. Spec: Add RBRR voicings to RBSA

- Add axvr_* voicing annotations to existing RBRR variable anchors
- Add axrd_file_sourced to rbrr_regime voicing
- Verify all 23 RBRR variables have proper type voicings

### 4. Spec: Create RBSRR subdocument

Create lenses/RBSRR-RegimeRepo.adoc following RBSRV pattern:
- axhrb_regime + axhrv_variable hierarchy for all RBRR variables
- axhro_broach/validate/render markers referencing rbkro_* terms
- Regime-specific operation descriptions

### 5. Verify

- tt/rbw-rrr.RenderRepoRegime.sh produces clean output
- tt/rbw-rrv.ValidateRepoRegime.sh passes
- Existing callers of rbrr_load still work (rbrv_cli.sh, rbrn_cli.sh)
- Ark lifecycle test passes

## Acceptance

- rbrr_regime.sh follows broach/validate_fields pattern matching RBRN and RBRV
- rbrr_cli.sh exists with render and validate commands
- RBSA has full axvr_* voicings for all RBRR variables
- RBSRR subdocument exists with axhr* hierarchy and axhro_* operations
- All render/validate tabtargets pass
- Ark lifecycle test passes

**[260212-1103] rough**

Implement regime operation model for RBRR: broach/validate split and render CLI.

## Context

AXLA now defines regime operation terms (axvr_broach, axvr_validate, axvr_render) and subdocument hierarchy markers (axhro_broach, axhro_validate, axhro_render). RBSA defines kit-level operation terms (rbkro_broach, rbkro_validate, rbkro_render). RBRN and RBRV both implement the full pattern. RBRR is the laggard.

Pace ATAAZ (normalize-regime-operation-specs) must be complete first — it ensures the AXLA/RBSA/subdocument additions are normalized and validated.

## Work

### 1. Code: Split zrbrr_kindle into broach + validate

Refactor rbrr_regime.sh following RBRN/RBRV pattern:
- `zrbrr_broach()` — set defaults with VAR="${VAR:-}", detect unexpected vars via compgen, build ZRBRR_DOCKER_ENV. Silent.
- `zrbrr_validate_fields()` — buv_env_* calls + format checks (podman version, timeout, platform). Dies on error.
- Update rbrr_load() to call both broach and validate
- Add unexpected variable detection (missing from RBRR today)

### 2. Code: Create rbrr_cli.sh with render/validate commands

Following rbrn_cli.sh and rbrv_cli.sh patterns:
- render command using rbcr_* shared module
- validate command using rbrr_load
- Tabtargets: rbw-rrr.RenderRepoRegime.sh, rbw-rrv.ValidateRepoRegime.sh

### 3. Spec: Add RBRR voicings to RBSA

- Add axvr_* voicing annotations to existing RBRR variable anchors
- Add axrd_file_sourced to rbrr_regime voicing
- Verify all 23 RBRR variables have proper type voicings

### 4. Spec: Create RBSRR subdocument

Create lenses/RBSRR-RegimeRepo.adoc following RBSRV pattern:
- axhrb_regime + axhrv_variable hierarchy for all RBRR variables
- axhro_broach/validate/render markers referencing rbkro_* terms
- Regime-specific operation descriptions

### 5. Verify

- tt/rbw-rrr.RenderRepoRegime.sh produces clean output
- tt/rbw-rrv.ValidateRepoRegime.sh passes
- Existing callers of rbrr_load still work (rbrv_cli.sh, rbrn_cli.sh)
- Ark lifecycle test passes

## Acceptance

- rbrr_regime.sh follows broach/validate_fields pattern matching RBRN and RBRV
- rbrr_cli.sh exists with render and validate commands
- RBSA has full axvr_* voicings for all RBRR variables
- RBSRR subdocument exists with axhr* hierarchy and axhro_* operations
- All render/validate tabtargets pass
- Ark lifecycle test passes

**[260212-1103] rough**

Implement regime operation model for RBRR: broach/validate split and render CLI.

## Context

AXLA now defines regime operation terms (axvr_broach, axvr_validate, axvr_render) and subdocument hierarchy markers (axhro_broach, axhro_validate, axhro_render). RBSA defines kit-level operation terms (rbkro_broach, rbkro_validate, rbkro_render). RBRN and RBRV both implement the full pattern. RBRR is the laggard.

Pace ATAAZ (normalize-regime-operation-specs) must be complete first — it ensures the AXLA/RBSA/subdocument additions are normalized and validated.

## Work

### 1. Code: Split zrbrr_kindle into broach + validate

Refactor rbrr_regime.sh following RBRN/RBRV pattern:
- `zrbrr_broach()` — set defaults with VAR="${VAR:-}", detect unexpected vars via compgen, build ZRBRR_DOCKER_ENV. Silent.
- `zrbrr_validate_fields()` — buv_env_* calls + format checks (podman version, timeout, platform). Dies on error.
- Update rbrr_load() to call both broach and validate
- Add unexpected variable detection (missing from RBRR today)

### 2. Code: Create rbrr_cli.sh with render/validate commands

Following rbrn_cli.sh and rbrv_cli.sh patterns:
- render command using rbcr_* shared module
- validate command using rbrr_load
- Tabtargets: rbw-rrr.RenderRepoRegime.sh, rbw-rrv.ValidateRepoRegime.sh

### 3. Spec: Add RBRR voicings to RBSA

- Add axvr_* voicing annotations to existing RBRR variable anchors
- Add axrd_file_sourced to rbrr_regime voicing
- Verify all 23 RBRR variables have proper type voicings

### 4. Spec: Create RBSRR subdocument

Create lenses/RBSRR-RegimeRepo.adoc following RBSRV pattern:
- axhrb_regime + axhrv_variable hierarchy for all RBRR variables
- axhro_broach/validate/render markers referencing rbkro_* terms
- Regime-specific operation descriptions

### 5. Verify

- tt/rbw-rrr.RenderRepoRegime.sh produces clean output
- tt/rbw-rrv.ValidateRepoRegime.sh passes
- Existing callers of rbrr_load still work (rbrv_cli.sh, rbrn_cli.sh)
- Ark lifecycle test passes

## Acceptance

- rbrr_regime.sh follows broach/validate_fields pattern matching RBRN and RBRV
- rbrr_cli.sh exists with render and validate commands
- RBSA has full axvr_* voicings for all RBRR variables
- RBSRR subdocument exists with axhr* hierarchy and axhro_* operations
- All render/validate tabtargets pass
- Ark lifecycle test passes

**[260210-1308] rough**

Refactor zrbrr_kindle to follow the kindle/validate_fields separation pattern used by RBRN and RBRV.

## Context

RBRN and RBRV regimes use a clean two-phase pattern:
- `kindle()` — silent: sets defaults with `VAR="${VAR:-}"`, detects unexpected vars, builds rollup/docker-env arrays
- `validate_fields()` — strict: calls buv_env_* for validation, dies on first error

RBRR's `zrbrr_kindle()` mixes both phases: it calls buv_env_* directly during kindle, making kindle noisy and inseparable from validation. This causes stdout noise in rbrv_cli.sh render (which calls rbrr_load as a dependency).

## Prerequisites

- ₢ATAAQ (study-buv-env-stdout-pattern) must be complete first — the buv_env_* contract may change, which affects how validate_fields is written.

## Work

1. Split zrbrr_kindle() into:
   - `zrbrr_kindle()` — set defaults, detect unexpected vars, build ZRBRR_DOCKER_ENV, set ZRBRR_KINDLED. Silent.
   - `zrbrr_validate_fields()` — buv_env_* calls + format checks (podman version, timeout, platform). Dies on error.
2. Update rbrr_load() to call both kindle and validate
3. Update rbrv_cli.sh render/validate to benefit (no more stdout noise from rbrr_load)
4. Verify all callers of rbrr_load / zrbrr_kindle still work

## Acceptance

- rbrr_regime.sh follows same kindle/validate_fields pattern as rbrn_regime.sh and rbrv_regime.sh
- `tt/rbw-rvr.RenderVesselRegime.sh` produces no preamble noise
- All render and validate tabtargets pass

### rename-bus-to-busa (₢ATAAB) [complete]

**[260208-1548] complete**

Rename BUS-BashUtilitiesSpec.adoc → BUS0-BashUtilitiesSpec.adoc

Mechanical file rename to establish subdocument architecture pattern. This parallels the planned RBS/RBAGS → RBSA merge, enabling future subdocs (BUSxx) within BUSA.

## Tasks
1. Rename file: `Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc` → `BUS0-BashUtilitiesSpec.adoc`
2. Update CLAUDE.md acronym mapping (BUS → BUSA)
3. Grep for any other BUS references and update

## Scope control
- Pure mechanical rename — no content changes
- Do NOT add regime vocabulary yet (that's ₢ATAAC)

## Verification
- File exists at new path
- No dangling BUS references
- CLAUDE.md updated

Result: BUSA ready for expansion with regime vocabulary in subsequent pace.

**[260201-2058] rough**

Rename BUS-BashUtilitiesSpec.adoc → BUS0-BashUtilitiesSpec.adoc

Mechanical file rename to establish subdocument architecture pattern. This parallels the planned RBS/RBAGS → RBSA merge, enabling future subdocs (BUSxx) within BUSA.

## Tasks
1. Rename file: `Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc` → `BUS0-BashUtilitiesSpec.adoc`
2. Update CLAUDE.md acronym mapping (BUS → BUSA)
3. Grep for any other BUS references and update

## Scope control
- Pure mechanical rename — no content changes
- Do NOT add regime vocabulary yet (that's ₢ATAAC)

## Verification
- File exists at new path
- No dangling BUS references
- CLAUDE.md updated

Result: BUSA ready for expansion with regime vocabulary in subsequent pace.

**[260131-1158] rough**

Create BURS (Bash Utilities Regime Spec) as universal voicing of AXLA patterns.

BURS defines the authoritative regime pattern for bash utilities, serving as reference for all bash-based configuration regimes. While BURC will provide bash-specific details, BURS itself documents:
- General regime architecture following AXLA axrg_* motifs
- Regime specification structure (sections, variable tables, format requirements)
- Assignment file organization and patterns
- Validation and rendering architecture
- Glossary and term mapping discipline

Structure following AXLA regime patterns established in expanded axrg_* section (₣AS ₢ASAAA):
- Core regime components and their relationships
- Variable naming and organization conventions
- Applicable to all regime implementations (bash, makefile, JSON, etc.)

Reference: AXLA expanded regime section
Complements: BURC (bash-native specifics)
Depends on: ₣AT ₢ATAAA (regime inventory study)

Result: BURS becomes definitive universal regime spec, enabling consistent patterns across all Recipe Bottle configuration regimes.

**[260131-1154] rough**

Create BURS (Bash Utilities Regime Spec) as voicing of AXLA patterns.

BURS replaces the old role of CRR for bash utility configuration. Define the regime pattern for BUK tools (buc, bud, but, buv, buw, burc, burs) using AXLA's axrg_* motifs.

Structure following AXLA regime patterns established in expanded axrg_* section:
- Variables for bash utility configuration (prefixes, paths, settings)
- Assignment file formats (makefile, bash, JSON)
- Validation and rendering patterns
- Glossary with AXLA voicings

Reference: AXLA expanded regime section (from ₣AS ₢ASAAA)
Related: Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc (naming vocabulary)
Depends on: ₣AT ₢ATAAA (regime inventory study)

Result: BURS becomes definitive regime spec for bash utilities, enabling other projects to adopt similar patterns.

### study-all-recipe-bottle-regimes (₢ATAAA) [complete]

**[260209-1008] complete**

Comprehensive study of ALL configuration regimes in Recipe Bottle.

## Discovery findings (from mount research)

**9 regime prefixes confirmed across two domains:**

### BUK domain (infrastructure)
- **BURC_** (Regime Configuration) — project-level config checked into git. Validator: Tools/buk/burc_regime.sh. Assignment: .buk/burc.env. Variables: STATION_FILE, TABTARGET_DIR, TABTARGET_DELIMITER, TOOLS_DIR, TEMP_ROOT_DIR, OUTPUT_ROOT_DIR, LOG_LAST, LOG_EXT, PROJECT_ROOT, MANAGED_KITS
- **BURS_** (Regime Station) — developer/machine-level config, NOT in git. Validator: Tools/buk/burs_regime.sh. Assignment: external (path in BURC_STATION_FILE). Variables: LOG_DIR
- **BUD_** (Dispatch runtime) — ephemeral variables set during bud_dispatch.sh execution. NOT a declared regime — runtime-only. Variables: VERBOSE, REGIME_FILE, NOW_STAMP, TEMP_DIR, OUTPUT_DIR, TRANSCRIPT, GIT_CONTEXT, COMMAND, TARGET, CLI_ARGS, TOKEN_1-5, LOG_LAST, LOG_SAME, LOG_HIST, COLOR, NO_LOG, INTERACTIVE

### RBW domain (Recipe Bottle)
- **RBRR_** (RegimeRepo) — repository-level config. Validator: Tools/rbw/rbrr_regime.sh. Spec: lenses/RBRR-RegimeRepo.adoc. Assignment: rbrr_RecipeBottleRegimeRepo.sh. ~30 variables covering registry, GCP, Cloud Build, machine config, service account file paths
- **RBRN_** (RegimeNameplate) — per-service deployment config. Validator: Tools/rbw/rbrn_regime.sh. Spec: lenses/RBRN-RegimeNameplate.adoc. Assignments: Tools/rbw/rbrn_nsproto.env, rbrn_srjcl.env, rbrn_pluml.env. ~20 variables covering identity, containers, networking, entry, uplink, volumes
- **RBRP_** (RegimePayor) — GCP payor project config. Validator: Tools/rbw/rbrp_regime.sh. Assignment: rbrp.env. Variables: PAYOR_PROJECT_ID, PARENT_TYPE, PARENT_ID, BILLING_ACCOUNT_ID, OAUTH_CLIENT_ID
- **RBRE_** (RegimeECR) — AWS ECR credentials. Validator: Tools/rbw/rbre_regime.sh. Variables: AWS_CREDENTIALS_ENV, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_ACCOUNT_ID, AWS_REGION, REPOSITORY_NAME
- **RBRG_** (RegimeGitHub) — GitHub credentials. Validator: Tools/rbw/rbrg_regime.sh. Variables: PAT, USERNAME
- **RBRS_** (RegimeStation) — developer workstation config. Validator: Tools/rbw/rbrs_regime.sh. Variables: PODMAN_ROOT_DIR, VMIMAGE_CACHE_DIR, VM_PLATFORM
- **RBRV_** (RegimeVessel) — per-vessel build/bind config. Validator: Tools/rbw/rbrv_regime.sh. Assignments: rbev-vessels/*/rbrv.env (6 vessels). Variables: SIGIL, DESCRIPTION, BIND_IMAGE, CONJURE_DOCKERFILE, CONJURE_BLDCONTEXT, CONJURE_PLATFORMS, CONJURE_BINFMT_POLICY

### Credential format (not a regime per se)
- **RBRA_** — Service account credential files sourced at runtime. Variables: CLIENT_EMAIL, PRIVATE_KEY, PROJECT_ID, TOKEN_LIFETIME_SEC. Written by rbgu_Utility.sh, consumed by rbgo_OAuth.sh. Referenced via RBRR_ paths (RBRR_GOVERNOR_RBRA_FILE, etc.)

### Corrections from original docket
- **RBB_*** does NOT exist as a regime — was hypothetical
- **RBS_*** maps to RBRS_ (RegimeStation) — the actual prefix
- BUD_ is runtime-only, not a declared regime with validator/assignment pattern

## Remaining work
Synthesize into a formal memo document (Memos/memo-YYYYMMDD-regime-inventory.md) with:
- Regime inventory table (prefix, scope, lifecycle, assignment file, validator, spec doc)
- Per-regime variable listings with types
- Cross-regime dependency map (BURC sources BURS, RBRR references RBRA files, etc.)
- Naming convention analysis per CRR framework
- Gap analysis (regimes without spec docs, missing validators, etc.)

Prior art: Memos/memo-20260206-rbrn-regime-fit-assessment.md (RBRN axhr analysis)

**[260208-1617] rough**

Comprehensive study of ALL configuration regimes in Recipe Bottle.

## Discovery findings (from mount research)

**9 regime prefixes confirmed across two domains:**

### BUK domain (infrastructure)
- **BURC_** (Regime Configuration) — project-level config checked into git. Validator: Tools/buk/burc_regime.sh. Assignment: .buk/burc.env. Variables: STATION_FILE, TABTARGET_DIR, TABTARGET_DELIMITER, TOOLS_DIR, TEMP_ROOT_DIR, OUTPUT_ROOT_DIR, LOG_LAST, LOG_EXT, PROJECT_ROOT, MANAGED_KITS
- **BURS_** (Regime Station) — developer/machine-level config, NOT in git. Validator: Tools/buk/burs_regime.sh. Assignment: external (path in BURC_STATION_FILE). Variables: LOG_DIR
- **BUD_** (Dispatch runtime) — ephemeral variables set during bud_dispatch.sh execution. NOT a declared regime — runtime-only. Variables: VERBOSE, REGIME_FILE, NOW_STAMP, TEMP_DIR, OUTPUT_DIR, TRANSCRIPT, GIT_CONTEXT, COMMAND, TARGET, CLI_ARGS, TOKEN_1-5, LOG_LAST, LOG_SAME, LOG_HIST, COLOR, NO_LOG, INTERACTIVE

### RBW domain (Recipe Bottle)
- **RBRR_** (RegimeRepo) — repository-level config. Validator: Tools/rbw/rbrr_regime.sh. Spec: lenses/RBRR-RegimeRepo.adoc. Assignment: rbrr_RecipeBottleRegimeRepo.sh. ~30 variables covering registry, GCP, Cloud Build, machine config, service account file paths
- **RBRN_** (RegimeNameplate) — per-service deployment config. Validator: Tools/rbw/rbrn_regime.sh. Spec: lenses/RBRN-RegimeNameplate.adoc. Assignments: Tools/rbw/rbrn_nsproto.env, rbrn_srjcl.env, rbrn_pluml.env. ~20 variables covering identity, containers, networking, entry, uplink, volumes
- **RBRP_** (RegimePayor) — GCP payor project config. Validator: Tools/rbw/rbrp_regime.sh. Assignment: rbrp.env. Variables: PAYOR_PROJECT_ID, PARENT_TYPE, PARENT_ID, BILLING_ACCOUNT_ID, OAUTH_CLIENT_ID
- **RBRE_** (RegimeECR) — AWS ECR credentials. Validator: Tools/rbw/rbre_regime.sh. Variables: AWS_CREDENTIALS_ENV, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_ACCOUNT_ID, AWS_REGION, REPOSITORY_NAME
- **RBRG_** (RegimeGitHub) — GitHub credentials. Validator: Tools/rbw/rbrg_regime.sh. Variables: PAT, USERNAME
- **RBRS_** (RegimeStation) — developer workstation config. Validator: Tools/rbw/rbrs_regime.sh. Variables: PODMAN_ROOT_DIR, VMIMAGE_CACHE_DIR, VM_PLATFORM
- **RBRV_** (RegimeVessel) — per-vessel build/bind config. Validator: Tools/rbw/rbrv_regime.sh. Assignments: rbev-vessels/*/rbrv.env (6 vessels). Variables: SIGIL, DESCRIPTION, BIND_IMAGE, CONJURE_DOCKERFILE, CONJURE_BLDCONTEXT, CONJURE_PLATFORMS, CONJURE_BINFMT_POLICY

### Credential format (not a regime per se)
- **RBRA_** — Service account credential files sourced at runtime. Variables: CLIENT_EMAIL, PRIVATE_KEY, PROJECT_ID, TOKEN_LIFETIME_SEC. Written by rbgu_Utility.sh, consumed by rbgo_OAuth.sh. Referenced via RBRR_ paths (RBRR_GOVERNOR_RBRA_FILE, etc.)

### Corrections from original docket
- **RBB_*** does NOT exist as a regime — was hypothetical
- **RBS_*** maps to RBRS_ (RegimeStation) — the actual prefix
- BUD_ is runtime-only, not a declared regime with validator/assignment pattern

## Remaining work
Synthesize into a formal memo document (Memos/memo-YYYYMMDD-regime-inventory.md) with:
- Regime inventory table (prefix, scope, lifecycle, assignment file, validator, spec doc)
- Per-regime variable listings with types
- Cross-regime dependency map (BURC sources BURS, RBRR references RBRA files, etc.)
- Naming convention analysis per CRR framework
- Gap analysis (regimes without spec docs, missing validators, etc.)

Prior art: Memos/memo-20260206-rbrn-regime-fit-assessment.md (RBRN axhr analysis)

**[260131-1154] rough**

Comprehensive study of ALL configuration regimes in Recipe Bottle.

Recipe Bottle uses multiple configuration regimes:
- **RBRR** (RegimeRepo) — repository-level configuration
- **RBRN** (RegimeNameplate) — service-level configuration  
- **RBB_*** (Base regime, if exists) — core system configuration
- **RBS_*** (Station regime, if exists) — deployment-specific configuration
- **BUD_*** (HIDDEN — dispatch-related regime) — variables set during buk dispatch operations

Discovery tasks:
1. Enumerate all prefixes in use (RB*, BUD_*, others?)
2. Map each to its spec document or implementation
3. Document what each regime controls and its lifecycle
4. Identify overlaps, conflicts, or inconsistencies
5. Understand BUD_ regime — what variables exist, where set, how consumed?

Deliverable: Complete regime inventory with:
- Prefix mapping
- Variable scope and lifecycle  
- Cross-regime dependencies
- Naming conventions per regime
- Gap analysis (unmapped variables, undocumented regimes)

This study enables consistent treatment of all regimes in subsequent paces.

### expand-busa-regime-vocabulary (₢ATAAC) [complete]

**[260212-1250] complete**

Expand BUS0-BashUtilitiesSpec.adoc with BURC and BURS regime vocabulary.

BUSA becomes the complete BUK concept model — dispatch vocabulary (existing) plus configuration regimes (new). This follows the pattern where each kit's XXS spec is the authoritative MCM/AXLA-compliant concept model.

## Pattern reference
Study how RBEV (vessel regime) is documented within RBAGS/RBS:
- Mappings in parent spec's mapping section
- Detailed regime definitions with AXLA voicings (axrg_regime, axrg_variable, axrg_assignment, axrg_prefix)
- The RBEV/RBSRV subdocument pattern is our most nuanced treatment

See: lenses/RBAGS-AdminGoogleSpec.adoc, lenses/RBS-Specification.adoc, lenses/RBSRV-*.adoc

## Tasks
1. Add `burc_*` attribute mappings to BUSA mapping section
2. Add `burs_*` attribute mappings to BUSA mapping section  
3. Add BURC regime section with:
   - Regime definition voicing axrg_regime
   - Variable definitions voicing axrg_variable (BURC_STATION_FILE, BURC_TABTARGET_DIR, etc.)
   - AXLA annotations throughout
4. Add BURS regime section with:
   - Station-level variables (BURS_LOG_DIR, etc.)
   - Personal/developer configuration patterns
5. Absorb content from orphaned `Tools/buk/burc_specification.md`
6. Delete or deprecate the old markdown spec

## Scope control
- Focus on regime vocabulary, not implementation details
- Do NOT create subdocuments yet (future work)
- Do NOT modify burc_regime.sh or burs_regime.sh (implementation unchanged)

## Verification
- BUSA has complete burc_* and burs_* mappings
- All regime variables have AXLA voicings
- No orphaned spec files

Result: BUSA is authoritative BUK concept model covering dispatch + regimes.

**[260201-2059] rough**

Expand BUS0-BashUtilitiesSpec.adoc with BURC and BURS regime vocabulary.

BUSA becomes the complete BUK concept model — dispatch vocabulary (existing) plus configuration regimes (new). This follows the pattern where each kit's XXS spec is the authoritative MCM/AXLA-compliant concept model.

## Pattern reference
Study how RBEV (vessel regime) is documented within RBAGS/RBS:
- Mappings in parent spec's mapping section
- Detailed regime definitions with AXLA voicings (axrg_regime, axrg_variable, axrg_assignment, axrg_prefix)
- The RBEV/RBSRV subdocument pattern is our most nuanced treatment

See: lenses/RBAGS-AdminGoogleSpec.adoc, lenses/RBS-Specification.adoc, lenses/RBSRV-*.adoc

## Tasks
1. Add `burc_*` attribute mappings to BUSA mapping section
2. Add `burs_*` attribute mappings to BUSA mapping section  
3. Add BURC regime section with:
   - Regime definition voicing axrg_regime
   - Variable definitions voicing axrg_variable (BURC_STATION_FILE, BURC_TABTARGET_DIR, etc.)
   - AXLA annotations throughout
4. Add BURS regime section with:
   - Station-level variables (BURS_LOG_DIR, etc.)
   - Personal/developer configuration patterns
5. Absorb content from orphaned `Tools/buk/burc_specification.md`
6. Delete or deprecate the old markdown spec

## Scope control
- Focus on regime vocabulary, not implementation details
- Do NOT create subdocuments yet (future work)
- Do NOT modify burc_regime.sh or burs_regime.sh (implementation unchanged)

## Verification
- BUSA has complete burc_* and burs_* mappings
- All regime variables have AXLA voicings
- No orphaned spec files

Result: BUSA is authoritative BUK concept model covering dispatch + regimes.

**[260131-1159] rough**

Create BURC (Bash Utilities Regime Configuration) specification.

BURC documents bash-native specifics for configuration regimes. While BURS provides the universal pattern, BURC grounds it in bash contexts:
- Bash variable naming conventions and scoping
- Source/export discipline for shell contexts
- Makefile variable declaration patterns (`:=` vs `=`)
- Integration with BUK dispatch system (BUD_* variables discovered in ₢ATAAA study)
- Bash validator and renderer architecture
- Assignment file format for bash (sourceable scripts)

This pace consolidates bash-specific regime knowledge discovered across RBRR, RBRN, and BUD_ regimes.

Reference: BURS spec (₢ATAAB)
Depends on: ₣AT ₢ATAAA (regime inventory study)
Complements: ₣AT ₢ATAAB (universal BURS patterns)

Result: Complete bash utilities regime definition, enabling consistent configuration and validation across BUK tools and Recipe Bottle.

**[260131-1156] rough**

Drafted from ₢ASAAE in ₣AS.

Create BURS (Bash Utilities Regime Spec) as voicing of AXLA patterns.

BURS replaces the old role of CRR for bash utility configuration. Define the regime pattern for BUK tools (buc, bud, but, buv, buw, burc, burs) using AXLA's axrg_* motifs.

Structure following AXLA regime patterns established in expanded axrg_* section:
- Variables for bash utility configuration (prefixes, paths, settings)
- Assignment file formats (makefile, bash, JSON)
- Validation and rendering patterns
- Glossary with AXLA voicings

Reference: AXLA expanded regime section (from ₢ASAAA)
Related: Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc (naming vocabulary)

Result: BURS becomes definitive regime spec for bash utilities, enabling other projects to adopt similar patterns.

**[260131-1152] rough**

Create BURS (Bash Utilities Regime Spec) as voicing of AXLA patterns.

BURS replaces the old role of CRR for bash utility configuration. Define the regime pattern for BUK tools (buc, bud, but, buv, buw, burc, burs) using AXLA's axrg_* motifs.

Structure following AXLA regime patterns established in expanded axrg_* section:
- Variables for bash utility configuration (prefixes, paths, settings)
- Assignment file formats (makefile, bash, JSON)
- Validation and rendering patterns
- Glossary with AXLA voicings

Reference: AXLA expanded regime section (from ₢ASAAA)
Related: Tools/buk/vov_veiled/BUS-BashUtilitiesSpec.adoc (naming vocabulary)

Result: BURS becomes definitive regime spec for bash utilities, enabling other projects to adopt similar patterns.

### consolidate-testbench-into-workbench (₢ATAAY) [complete]

**[260212-1418] complete**

Route testbench through workbench and clean up zipper compatibility shims.

## Context

The testbench (rbtb_testbench.sh) is already a swiss-army executable: it sources
~20 modules, kindles, enrolls all suites via butr registry, and routes to
butd_run_all/butd_run_suite/butd_run_one. BUD already execs it via its own
launcher. The internal model (sourced functions in subshells) is fine.

The problem: the testbench has its own launcher (launcher.rbtb_testbench.sh)
parallel to the workbench launcher. And buz_zipper.sh carries compatibility
shims (buz_dispatch, buz_init_evidence, etc.) from the pre-bute migration.

## Work

1. **Add test routes to rbw_workbench.sh**: Workbench gains test colophons
   (rbw-ta, rbw-ts, rbw-to, rbw-tns, rbw-tsj, rbw-tpl, rbw-trg) that exec
   rbtb_testbench.sh with the appropriate command. BUD env vars (BURD_TEMP_DIR,
   BURD_NOW_STAMP) are already exported and inherited.

2. **Rename tabtargets**: Migrate tt/rbtb-*.sh to tt/rbw-t*.sh colophons,
   pointing to launcher.rbw_workbench.sh instead of launcher.rbtb_testbench.sh.

3. **Delete launcher.rbtb_testbench.sh**: No longer needed — all test routing
   goes through the workbench.

4. **Remove buz_ compatibility shims**: Delete buz_dispatch, buz_init_evidence,
   buz_last_step_capture, buz_get_step_exit_capture, buz_get_step_output_capture
   from buz_zipper.sh (lines 103-121). Migrate any remaining callers to bute_*.

5. **Verify all tests pass**: Run full test suite through new routing.

## Acceptance

- All test tabtargets route through rbw_workbench.sh (no testbench launcher)
- launcher.rbtb_testbench.sh deleted
- buz_zipper.sh has no dispatch/evidence compatibility shims
- All existing tests still pass via new rbw-t* tabtargets

**[260212-1337] bridled**

Route testbench through workbench and clean up zipper compatibility shims.

## Context

The testbench (rbtb_testbench.sh) is already a swiss-army executable: it sources
~20 modules, kindles, enrolls all suites via butr registry, and routes to
butd_run_all/butd_run_suite/butd_run_one. BUD already execs it via its own
launcher. The internal model (sourced functions in subshells) is fine.

The problem: the testbench has its own launcher (launcher.rbtb_testbench.sh)
parallel to the workbench launcher. And buz_zipper.sh carries compatibility
shims (buz_dispatch, buz_init_evidence, etc.) from the pre-bute migration.

## Work

1. **Add test routes to rbw_workbench.sh**: Workbench gains test colophons
   (rbw-ta, rbw-ts, rbw-to, rbw-tns, rbw-tsj, rbw-tpl, rbw-trg) that exec
   rbtb_testbench.sh with the appropriate command. BUD env vars (BURD_TEMP_DIR,
   BURD_NOW_STAMP) are already exported and inherited.

2. **Rename tabtargets**: Migrate tt/rbtb-*.sh to tt/rbw-t*.sh colophons,
   pointing to launcher.rbw_workbench.sh instead of launcher.rbtb_testbench.sh.

3. **Delete launcher.rbtb_testbench.sh**: No longer needed — all test routing
   goes through the workbench.

4. **Remove buz_ compatibility shims**: Delete buz_dispatch, buz_init_evidence,
   buz_last_step_capture, buz_get_step_exit_capture, buz_get_step_output_capture
   from buz_zipper.sh (lines 103-121). Migrate any remaining callers to bute_*.

5. **Verify all tests pass**: Run full test suite through new routing.

## Acceptance

- All test tabtargets route through rbw_workbench.sh (no testbench launcher)
- launcher.rbtb_testbench.sh deleted
- buz_zipper.sh has no dispatch/evidence compatibility shims
- All existing tests still pass via new rbw-t* tabtargets

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/rbw/rbw_workbench.sh, Tools/rbw/rbtb_testbench.sh, Tools/buk/buz_zipper.sh, .buk/launcher.rbtb_testbench.sh, tt/rbtb-ta.TestAll.sh, tt/rbtb-ts.TestSuite.sh, tt/rbtb-to.TestOne.sh, tt/rbtb-ns.TestNsproSecurity.nsproto.sh, tt/rbtb-sj.TestSrjclJupyter.srjcl.sh, tt/rbtb-pl.TestPlumlDiagram.pluml.sh, tt/rbtb-rg.TestRegimeSmoke.sh (11 files) | Steps: 1. Add test routing case arms to rbw_workbench.sh that exec rbtb_testbench.sh -- colophon mapping: rbw-ta=TestAll, rbw-ts=TestSuite, rbw-to=TestOne, rbw-tns=nsproto, rbw-tj=srjcl, rbw-tpl=pluml, rbw-trg=regime-smoke 2. Update rbtb_testbench.sh rbtb_route case arms from rbtb-ta/rbtb-ts/rbtb-to/rbtb-ns/rbtb-sj/rbtb-pl/rbtb-rg to rbw-ta/rbw-ts/rbw-to/rbw-tns/rbw-tj/rbw-tpl/rbw-trg 3. Create new tabtarget files tt/rbw-ta.TestAll.sh, tt/rbw-ts.TestSuite.sh, tt/rbw-to.TestOne.sh, tt/rbw-tns.TestNsproSecurity.nsproto.sh, tt/rbw-tj.TestSrjclJupyter.srjcl.sh, tt/rbw-tpl.TestPlumlDiagram.pluml.sh, tt/rbw-trg.TestRegimeSmoke.sh -- each uses BURD_LAUNCHER=.buk/launcher.rbw_workbench.sh 4. Delete old tt/rbtb-*.sh tabtarget files 5. Delete .buk/launcher.rbtb_testbench.sh 6. Remove buz_ compatibility shims from buz_zipper.sh -- delete the "Dispatch/evidence compatibility shims" section including buz_init_evidence, buz_dispatch, buz_last_step_capture, buz_get_step_exit_capture, buz_get_step_output_capture and the source of bute_engine.sh that supports them 7. Make new tabtarget files executable with chmod +x | Verify: tt/rbw-ta.TestAll.sh

**[260212-1333] rough**

Route testbench through workbench and clean up zipper compatibility shims.

## Context

The testbench (rbtb_testbench.sh) is already a swiss-army executable: it sources
~20 modules, kindles, enrolls all suites via butr registry, and routes to
butd_run_all/butd_run_suite/butd_run_one. BUD already execs it via its own
launcher. The internal model (sourced functions in subshells) is fine.

The problem: the testbench has its own launcher (launcher.rbtb_testbench.sh)
parallel to the workbench launcher. And buz_zipper.sh carries compatibility
shims (buz_dispatch, buz_init_evidence, etc.) from the pre-bute migration.

## Work

1. **Add test routes to rbw_workbench.sh**: Workbench gains test colophons
   (rbw-ta, rbw-ts, rbw-to, rbw-tns, rbw-tsj, rbw-tpl, rbw-trg) that exec
   rbtb_testbench.sh with the appropriate command. BUD env vars (BURD_TEMP_DIR,
   BURD_NOW_STAMP) are already exported and inherited.

2. **Rename tabtargets**: Migrate tt/rbtb-*.sh to tt/rbw-t*.sh colophons,
   pointing to launcher.rbw_workbench.sh instead of launcher.rbtb_testbench.sh.

3. **Delete launcher.rbtb_testbench.sh**: No longer needed — all test routing
   goes through the workbench.

4. **Remove buz_ compatibility shims**: Delete buz_dispatch, buz_init_evidence,
   buz_last_step_capture, buz_get_step_exit_capture, buz_get_step_output_capture
   from buz_zipper.sh (lines 103-121). Migrate any remaining callers to bute_*.

5. **Verify all tests pass**: Run full test suite through new routing.

## Acceptance

- All test tabtargets route through rbw_workbench.sh (no testbench launcher)
- launcher.rbtb_testbench.sh deleted
- buz_zipper.sh has no dispatch/evidence compatibility shims
- All existing tests still pass via new rbw-t* tabtargets

**[260212-1333] rough**

Route testbench through workbench and clean up zipper compatibility shims.

## Context

The testbench (rbtb_testbench.sh) is already a swiss-army executable: it sources
~20 modules, kindles, enrolls all suites via butr registry, and routes to
butd_run_all/butd_run_suite/butd_run_one. BUD already execs it via its own
launcher. The internal model (sourced functions in subshells) is fine.

The problem: the testbench has its own launcher (launcher.rbtb_testbench.sh)
parallel to the workbench launcher. And buz_zipper.sh carries compatibility
shims (buz_dispatch, buz_init_evidence, etc.) from the pre-bute migration.

## Work

1. **Add test routes to rbw_workbench.sh**: Workbench gains test colophons
   (rbw-ta, rbw-ts, rbw-to, rbw-tns, rbw-tsj, rbw-tpl, rbw-trg) that exec
   rbtb_testbench.sh with the appropriate command. BUD env vars (BURD_TEMP_DIR,
   BURD_NOW_STAMP) are already exported and inherited.

2. **Rename tabtargets**: Migrate tt/rbtb-*.sh to tt/rbw-t*.sh colophons,
   pointing to launcher.rbw_workbench.sh instead of launcher.rbtb_testbench.sh.

3. **Delete launcher.rbtb_testbench.sh**: No longer needed — all test routing
   goes through the workbench.

4. **Remove buz_ compatibility shims**: Delete buz_dispatch, buz_init_evidence,
   buz_last_step_capture, buz_get_step_exit_capture, buz_get_step_output_capture
   from buz_zipper.sh (lines 103-121). Migrate any remaining callers to bute_*.

5. **Verify all tests pass**: Run full test suite through new routing.

## Acceptance

- All test tabtargets route through rbw_workbench.sh (no testbench launcher)
- launcher.rbtb_testbench.sh deleted
- buz_zipper.sh has no dispatch/evidence compatibility shims
- All existing tests still pass via new rbw-t* tabtargets

**[260212-1313] rough**

Convert testbench from in-process sourced-function execution to exec-based dispatch through BUD.

## Finding (from ₢ATAAY exploration)

The zipper (buz_zipper.sh) currently serves two dispatch models:
- **Runtime**: colophon → exec CLI script (via RBK/workbench, clean out-of-process)
- **Tests**: colophon → sourced function in kindled process (in-process, inherits full state)

This dual model makes the zipper unnecessarily complex. The fix: make tests run as
their own processes, so the zipper has one universal dispatch model (exec).

## Current state

rbtb_testbench.sh sources ~20 modules (buc, bute, butr, buz, rbz, buv, rbrn, rbrr,
rbcc, rbgc, rbgd, rbob, plus 8 test case files). Test case functions run in subshells
via `zbute_case()` — isolated from each other but sharing kindled state from the
testbench process.

The dispatch-exercise suite already proves the exec model works — it uses
bute_dispatch to invoke tabtargets through the full BUD pipeline with evidence
collection.

## Work

1. **Design test script pattern**: Each test suite becomes a self-contained
   executable script that sources+kindles its own dependencies. Suite setup
   (e.g., rbtb_load_nameplate) moves into the script.

2. **Convert one suite as prototype**: Pick a simple suite (e.g., kick-tires or
   xname-validation) and convert it to an exec'd script invoked through BUD.
   Verify evidence collection works.

3. **Convert remaining suites**: nsproto-security, srjcl-jupyter, pluml-diagram,
   regime-smoke, ark-lifecycle. Each becomes self-contained.

4. **Simplify testbench**: rbtb_testbench.sh stops sourcing the world. It becomes
   a thin routing workbench that execs test scripts, same as rbw_workbench execs
   CLI scripts.

5. **Remove buz compatibility shims**: The buz_dispatch/buz_init_evidence shims
   in buz_zipper.sh (lines 103-121) that delegate to bute_engine.sh can be removed
   once all test callers use bute_ directly or go through exec.

## Acceptance

- All test suites execute via exec (not sourced functions)
- Testbench no longer sources module dependency tree
- buz_zipper.sh has no in-process dispatch functions
- All existing tests still pass

**[260212-1313] rough**

Convert testbench from in-process sourced-function execution to exec-based dispatch through BUD.

## Finding (from ₢ATAAY exploration)

The zipper (buz_zipper.sh) currently serves two dispatch models:
- **Runtime**: colophon → exec CLI script (via RBK/workbench, clean out-of-process)
- **Tests**: colophon → sourced function in kindled process (in-process, inherits full state)

This dual model makes the zipper unnecessarily complex. The fix: make tests run as
their own processes, so the zipper has one universal dispatch model (exec).

## Current state

rbtb_testbench.sh sources ~20 modules (buc, bute, butr, buz, rbz, buv, rbrn, rbrr,
rbcc, rbgc, rbgd, rbob, plus 8 test case files). Test case functions run in subshells
via `zbute_case()` — isolated from each other but sharing kindled state from the
testbench process.

The dispatch-exercise suite already proves the exec model works — it uses
bute_dispatch to invoke tabtargets through the full BUD pipeline with evidence
collection.

## Work

1. **Design test script pattern**: Each test suite becomes a self-contained
   executable script that sources+kindles its own dependencies. Suite setup
   (e.g., rbtb_load_nameplate) moves into the script.

2. **Convert one suite as prototype**: Pick a simple suite (e.g., kick-tires or
   xname-validation) and convert it to an exec'd script invoked through BUD.
   Verify evidence collection works.

3. **Convert remaining suites**: nsproto-security, srjcl-jupyter, pluml-diagram,
   regime-smoke, ark-lifecycle. Each becomes self-contained.

4. **Simplify testbench**: rbtb_testbench.sh stops sourcing the world. It becomes
   a thin routing workbench that execs test scripts, same as rbw_workbench execs
   CLI scripts.

5. **Remove buz compatibility shims**: The buz_dispatch/buz_init_evidence shims
   in buz_zipper.sh (lines 103-121) that delegate to bute_engine.sh can be removed
   once all test callers use bute_ directly or go through exec.

## Acceptance

- All test suites execute via exec (not sourced functions)
- Testbench no longer sources module dependency tree
- buz_zipper.sh has no in-process dispatch functions
- All existing tests still pass

**[260212-0747] rough**

Explore converting buz_zipper from sourced-function dispatch to exec-based dispatch.

## Context

The zipper (buz_zipper.sh) currently maps colophon → module → command where
commands are sourced functions called in the same process. But several command
families already use exec to standalone CLI scripts:
- rbob_cli.sh (bottle operations) via workbench case routing
- rbrn_cli.sh, rbrv_cli.sh (regime operations) — refactored in ₢ATAAH to be
  self-contained with their own listing and path resolution

The workbench (rbw_workbench.sh) now has two dispatch patterns side by side:
- Bottle ops: case-based exec to rbob_cli.sh
- Regime ops: pure one-liner exec to rbrn_cli.sh / rbrv_cli.sh
- Zipper-registered ops: go through rbk_Coordinator.sh (legacy) which also execs

## Question

Should all zipper-registered commands use exec for simplicity? This would:
1. Unify the dispatch model (no more sourced-function vs exec split)
2. Let regime CLIs participate in the zipper registry
3. Potentially retire RBK (legacy coordinator) by merging its routing into
   the unified zipper dispatch

## Regime-specific notes

rbrn_cli.sh and rbrv_cli.sh are now fully self-contained:
- Accept `render [moniker/sigil]` and `validate [moniker/sigil]`
- No arg = list available items
- Path resolution uses RBCC constants (nameplate) or RBRR_VESSEL_DIR (vessel)
- Ready to be zipper targets once zipper supports exec pattern

## Work

1. Study buz_register / buz_dispatch flow — identify what sourced-context
   assumptions exist (shared variables, kindle state, etc.)
2. Prototype exec-based dispatch variant (buz_register_exec or flag)
3. Evaluate: can ALL current zipper commands convert, or do some need
   same-process state? (e.g., commands that read kindled variables)
4. If viable: convert rbz_zipper.sh registrations, register regime colophons
5. Assess RBK retirement path

## Acceptance

- Clear recommendation: exec-only, dual-mode, or keep sourced
- If exec-only viable: prototype working with at least regime colophons registered

### refresh-docker-hub-pins (₢ATAAc) [complete]

**[260213-0811] complete**

Re-run `tt/rbw-rrg.RefreshGcbPins.sh` until all three Docker Hub images resolve successfully.

## Context

Commit 0937134a fully qualified three Docker Hub image refs (alpine, syft, binfmt) with `docker.io/` prefix to pass `buv_val_odref` validation. The refresh script now correctly targets `docker.io/library/alpine`, `docker.io/anchore/syft`, and `docker.io/tonistiigi/binfmt`, but Docker Hub rate limiting caused all three to fail on first attempt.

## Steps

1. Run `tt/rbw-rrg.RefreshGcbPins.sh`
2. If Docker Hub fetches fail with rate limit warnings:
   - Try `docker login docker.io` (free account raises limit from 100→200 pulls/6hr)
   - Or wait a few minutes and retry
3. Verify all 8 images show "unchanged" or "updated" (0 failed)
4. If any pins changed, notch the updated `rbrr_RecipeBottleRegimeRepo.sh`

## Key Files
- `rbrr_RecipeBottleRegimeRepo.sh` — live pin values
- `Tools/rbw/rbrr_regime.sh` — refresh spec (lines 273-282)

**[260213-0642] complete**

Re-run `tt/rbw-rrg.RefreshGcbPins.sh` until all three Docker Hub images resolve successfully.

## Context

Commit 0937134a fully qualified three Docker Hub image refs (alpine, syft, binfmt) with `docker.io/` prefix to pass `buv_val_odref` validation. The refresh script now correctly targets `docker.io/library/alpine`, `docker.io/anchore/syft`, and `docker.io/tonistiigi/binfmt`, but Docker Hub rate limiting caused all three to fail on first attempt.

## Steps

1. Run `tt/rbw-rrg.RefreshGcbPins.sh`
2. If Docker Hub fetches fail with rate limit warnings:
   - Try `docker login docker.io` (free account raises limit from 100→200 pulls/6hr)
   - Or wait a few minutes and retry
3. Verify all 8 images show "unchanged" or "updated" (0 failed)
4. If any pins changed, notch the updated `rbrr_RecipeBottleRegimeRepo.sh`

## Key Files
- `rbrr_RecipeBottleRegimeRepo.sh` — live pin values
- `Tools/rbw/rbrr_regime.sh` — refresh spec (lines 273-282)

**[260212-1418] rough**

Re-run `tt/rbw-rrg.RefreshGcbPins.sh` until all three Docker Hub images resolve successfully.

## Context

Commit 0937134a fully qualified three Docker Hub image refs (alpine, syft, binfmt) with `docker.io/` prefix to pass `buv_val_odref` validation. The refresh script now correctly targets `docker.io/library/alpine`, `docker.io/anchore/syft`, and `docker.io/tonistiigi/binfmt`, but Docker Hub rate limiting caused all three to fail on first attempt.

## Steps

1. Run `tt/rbw-rrg.RefreshGcbPins.sh`
2. If Docker Hub fetches fail with rate limit warnings:
   - Try `docker login docker.io` (free account raises limit from 100→200 pulls/6hr)
   - Or wait a few minutes and retry
3. Verify all 8 images show "unchanged" or "updated" (0 failed)
4. If any pins changed, notch the updated `rbrr_RecipeBottleRegimeRepo.sh`

## Key Files
- `rbrr_RecipeBottleRegimeRepo.sh` — live pin values
- `Tools/rbw/rbrr_regime.sh` — refresh spec (lines 273-282)

### improve-test-discoverability (₢ATAAb) [complete]

**[260213-0855] complete**

Improve test tabtarget UX: make suites/cases discoverable, audit and remove dead tabtargets.

## Problem

Running test tabtargets without arguments produces hostile output — a raw FATAL error
with no guidance. For example, `tt/rbw-to.TestOne.sh` prints:

  ERROR: butd_run_one: function_name required

And there are pre-existing test tabtargets of unknown status:
- rbw-tb.TestBottles.parallel.sh
- rbw-tb.TestBottles.single.sh
- rbw-tf.FastTest.sh
- rbw-tg.TestGithubWorkflow.sh

These may be dead or stale. Each must be researched — trace what they route to,
whether the target coordinator/function still exists, and whether they serve any
current purpose. Do not delete without confirming dead.

## Work

1. **Audit pre-existing test tabtargets**: For each of rbw-tb, rbw-tf, rbw-tg,
   trace the routing chain. Check if the coordinator they reference exists, if
   the commands they invoke are still valid. Report findings and delete confirmed
   dead ones.

2. **butd_run_suite with no arg**: Currently lists suites then fatals. Change to
   print a clean table of available suites with case counts and exit 0.

3. **butd_run_one with no arg**: Currently lists all functions then fatals. Change
   to print a clean table grouped by suite and exit 0.

4. **Testbench unknown command**: When rbtb_testbench.sh receives an unknown
   command, list available test colophons with descriptions instead of dying
   with "Unknown command".

## Acceptance

- Dead test tabtargets identified and removed (with rationale documented)
- tt/rbw-ts.TestSuite.sh with no args prints available suites cleanly, exits 0
- tt/rbw-to.TestOne.sh with no args prints available functions, exits 0
- Output formatted for human scanning (aligned columns, suite grouping)
- No changes to test execution behavior when args are provided

**[260212-1415] rough**

Improve test tabtarget UX: make suites/cases discoverable, audit and remove dead tabtargets.

## Problem

Running test tabtargets without arguments produces hostile output — a raw FATAL error
with no guidance. For example, `tt/rbw-to.TestOne.sh` prints:

  ERROR: butd_run_one: function_name required

And there are pre-existing test tabtargets of unknown status:
- rbw-tb.TestBottles.parallel.sh
- rbw-tb.TestBottles.single.sh
- rbw-tf.FastTest.sh
- rbw-tg.TestGithubWorkflow.sh

These may be dead or stale. Each must be researched — trace what they route to,
whether the target coordinator/function still exists, and whether they serve any
current purpose. Do not delete without confirming dead.

## Work

1. **Audit pre-existing test tabtargets**: For each of rbw-tb, rbw-tf, rbw-tg,
   trace the routing chain. Check if the coordinator they reference exists, if
   the commands they invoke are still valid. Report findings and delete confirmed
   dead ones.

2. **butd_run_suite with no arg**: Currently lists suites then fatals. Change to
   print a clean table of available suites with case counts and exit 0.

3. **butd_run_one with no arg**: Currently lists all functions then fatals. Change
   to print a clean table grouped by suite and exit 0.

4. **Testbench unknown command**: When rbtb_testbench.sh receives an unknown
   command, list available test colophons with descriptions instead of dying
   with "Unknown command".

## Acceptance

- Dead test tabtargets identified and removed (with rationale documented)
- tt/rbw-ts.TestSuite.sh with no args prints available suites cleanly, exits 0
- tt/rbw-to.TestOne.sh with no args prints available functions, exits 0
- Output formatted for human scanning (aligned columns, suite grouping)
- No changes to test execution behavior when args are provided

### unify-workbench-zipper-dispatch (₢ATAAa) [complete]

**[260213-0942] complete**

Unify runtime dispatch: merge RBK into workbench, use zipper kindle constants, add lookup dispatch.

## Finding (from ₢ATAAY exploration)

Three runtime dispatch patterns coexist:
1. **Zipper+RBK** (20 tabtargets): tabtarget → launcher.rbk_Coordinator → BUD → rbk_Coordinator.sh case → exec CLI
2. **Workbench case** (bottle ops, 11 tabtargets): tabtarget → launcher.rbw_workbench → BUD → rbw_workbench.sh case → exec CLI
3. **Workbench pure exec** (regime ops, 8 tabtargets): same as #2 but one-liner exec

RBK is redundant — it does the same case→exec routing as workbench. Neither uses
the zipper kindle constants (RBZ_CREATE_DEPOT etc.) — both have hardcoded magic strings.

## Work

1. **Merge RBK case arms into rbw_workbench.sh**: Move all 20 routing entries.
   Switch their tabtargets from launcher.rbk_Coordinator to launcher.rbw_workbench.

2. **Replace magic strings with zipper constants**: Workbench case arms use
   ${RBZ_CREATE_DEPOT} instead of "rbw-PC", etc. Requires kindling buz/rbz
   in the workbench (or its launcher).

3. **Add zipper lookup dispatch for degenerate cases**: For arms with no
   imprint logic, use a zbuz_exec_lookup that resolves colophon→CLI+command
   from the registry. The workbench case statement shrinks to just non-degenerate
   entries (bottle ops with RBOB_MONIKER, etc.).

4. **Delete rbk_Coordinator.sh and launcher.rbk_Coordinator.sh**

5. **Register regime colophons in zipper**: Add buz_register calls for
   rbw-rnr, rbw-rnv, rbw-rvr, rbw-rvv, rbw-rrr, rbw-rrv so they participate
   in the registry.

## Acceptance

- rbk_Coordinator.sh deleted
- All 39 rbw-* tabtargets route through rbw_workbench.sh
- Degenerate case arms use zipper lookup, not hardcoded case entries
- All tabtargets still function correctly

**[260212-1314] rough**

Unify runtime dispatch: merge RBK into workbench, use zipper kindle constants, add lookup dispatch.

## Finding (from ₢ATAAY exploration)

Three runtime dispatch patterns coexist:
1. **Zipper+RBK** (20 tabtargets): tabtarget → launcher.rbk_Coordinator → BUD → rbk_Coordinator.sh case → exec CLI
2. **Workbench case** (bottle ops, 11 tabtargets): tabtarget → launcher.rbw_workbench → BUD → rbw_workbench.sh case → exec CLI
3. **Workbench pure exec** (regime ops, 8 tabtargets): same as #2 but one-liner exec

RBK is redundant — it does the same case→exec routing as workbench. Neither uses
the zipper kindle constants (RBZ_CREATE_DEPOT etc.) — both have hardcoded magic strings.

## Work

1. **Merge RBK case arms into rbw_workbench.sh**: Move all 20 routing entries.
   Switch their tabtargets from launcher.rbk_Coordinator to launcher.rbw_workbench.

2. **Replace magic strings with zipper constants**: Workbench case arms use
   ${RBZ_CREATE_DEPOT} instead of "rbw-PC", etc. Requires kindling buz/rbz
   in the workbench (or its launcher).

3. **Add zipper lookup dispatch for degenerate cases**: For arms with no
   imprint logic, use a zbuz_exec_lookup that resolves colophon→CLI+command
   from the registry. The workbench case statement shrinks to just non-degenerate
   entries (bottle ops with RBOB_MONIKER, etc.).

4. **Delete rbk_Coordinator.sh and launcher.rbk_Coordinator.sh**

5. **Register regime colophons in zipper**: Add buz_register calls for
   rbw-rnr, rbw-rnv, rbw-rvr, rbw-rvv, rbw-rrr, rbw-rrv so they participate
   in the registry.

## Acceptance

- rbk_Coordinator.sh deleted
- All 39 rbw-* tabtargets route through rbw_workbench.sh
- Degenerate case arms use zipper lookup, not hardcoded case entries
- All tabtargets still function correctly

### cleanup-rbrr-legacy-variables (₢ATAAF) [complete]

**[260214-0648] complete**

Clean up dead RBRR variables identified by full-codebase audit and fix discovered bug.

Audit results (completed)

8 dead variables to remove:
- RBRR_REGISTRY_SERVER — spec-only, never implemented
- RBRR_BUILD_ARCHITECTURES — only in ABANDONED-github/
- RBRR_HISTORY_DIR — only in ABANDONED-github/
- RBRR_MACHINE_NAME — superseded by IGNITE/DEPLOY split
- RBRR_ENCLAVE_SUBNET — mentioned in spec term def, never implemented
- RBRR_VMDIST_TAG — Makefile-only, not in regime validation
- RBRR_VMDIST_BLOB_SHA — Makefile-only, not in regime validation
- RBRR_VMDIST_CRANE — Makefile-only, not in regime validation

1 bug to fix:
- rbw.workbench.mk:49 uses RBV_GITHUB_PAT_ENV (typo, should be RBRR_GITHUB_PAT_ENV)

Note: RBS-Specification.adoc fate (including its RBV_USERNAME/RBV_PAT doc mismatch) belongs to ₣AS pace ₢ASAAI (retire-rbags-rbs-files). Not in scope here.

RBGD overlap is intentional kindling — no action needed.
RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active — leave as-is.

Tasks

1. Remove 8 dead variables from RBRR-RegimeRepo.adoc documentation
2. Remove dead variables from rbrr_regime.sh validation if present
3. Fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk:49

Acceptance

- Dead variables removed from spec and validation
- Bug fixed
- RBS fate explicitly deferred to ₣AS ₢ASAAI

**[260213-1006] rough**

Clean up dead RBRR variables identified by full-codebase audit and fix discovered bug.

Audit results (completed)

8 dead variables to remove:
- RBRR_REGISTRY_SERVER — spec-only, never implemented
- RBRR_BUILD_ARCHITECTURES — only in ABANDONED-github/
- RBRR_HISTORY_DIR — only in ABANDONED-github/
- RBRR_MACHINE_NAME — superseded by IGNITE/DEPLOY split
- RBRR_ENCLAVE_SUBNET — mentioned in spec term def, never implemented
- RBRR_VMDIST_TAG — Makefile-only, not in regime validation
- RBRR_VMDIST_BLOB_SHA — Makefile-only, not in regime validation
- RBRR_VMDIST_CRANE — Makefile-only, not in regime validation

1 bug to fix:
- rbw.workbench.mk:49 uses RBV_GITHUB_PAT_ENV (typo, should be RBRR_GITHUB_PAT_ENV)

Note: RBS-Specification.adoc fate (including its RBV_USERNAME/RBV_PAT doc mismatch) belongs to ₣AS pace ₢ASAAI (retire-rbags-rbs-files). Not in scope here.

RBGD overlap is intentional kindling — no action needed.
RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active — leave as-is.

Tasks

1. Remove 8 dead variables from RBRR-RegimeRepo.adoc documentation
2. Remove dead variables from rbrr_regime.sh validation if present
3. Fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk:49

Acceptance

- Dead variables removed from spec and validation
- Bug fixed
- RBS fate explicitly deferred to ₣AS ₢ASAAI

**[260213-1006] rough**

Clean up dead RBRR variables identified by full-codebase audit and fix discovered bug.

Audit results (completed)

8 dead variables to remove:
- RBRR_REGISTRY_SERVER — spec-only, never implemented
- RBRR_BUILD_ARCHITECTURES — only in ABANDONED-github/
- RBRR_HISTORY_DIR — only in ABANDONED-github/
- RBRR_MACHINE_NAME — superseded by IGNITE/DEPLOY split
- RBRR_ENCLAVE_SUBNET — mentioned in spec term def, never implemented
- RBRR_VMDIST_TAG — Makefile-only, not in regime validation
- RBRR_VMDIST_BLOB_SHA — Makefile-only, not in regime validation
- RBRR_VMDIST_CRANE — Makefile-only, not in regime validation

1 bug to fix:
- rbw.workbench.mk:49 uses RBV_GITHUB_PAT_ENV (typo, should be RBRR_GITHUB_PAT_ENV)

Note: RBS-Specification.adoc fate (including its RBV_USERNAME/RBV_PAT doc mismatch) belongs to ₣AS pace ₢ASAAI (retire-rbags-rbs-files). Not in scope here.

RBGD overlap is intentional kindling — no action needed.
RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active — leave as-is.

Tasks

1. Remove 8 dead variables from RBRR-RegimeRepo.adoc documentation
2. Remove dead variables from rbrr_regime.sh validation if present
3. Fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk:49

Acceptance

- Dead variables removed from spec and validation
- Bug fixed
- RBS fate explicitly deferred to ₣AS ₢ASAAI

**[260213-1000] rough**

Clean up dead RBRR variables identified by full-codebase audit, fix discovered bugs, and determine if RBS-Specification.adoc is redundant with RBS0-SpecTop.adoc.

Audit results (completed)

8 dead variables to remove:
- RBRR_REGISTRY_SERVER — spec-only, never implemented
- RBRR_BUILD_ARCHITECTURES — only in ABANDONED-github/
- RBRR_HISTORY_DIR — only in ABANDONED-github/
- RBRR_MACHINE_NAME — superseded by IGNITE/DEPLOY split
- RBRR_ENCLAVE_SUBNET — mentioned in spec term def, never implemented
- RBRR_VMDIST_TAG — Makefile-only, not in regime validation
- RBRR_VMDIST_BLOB_SHA — Makefile-only, not in regime validation
- RBRR_VMDIST_CRANE — Makefile-only, not in regime validation

2 bugs to fix:
- rbw.workbench.mk:49 uses RBV_GITHUB_PAT_ENV (typo, should be RBRR_GITHUB_PAT_ENV)
- RBS-Specification.adoc:1010 says RBV_USERNAME/RBV_PAT but code uses RBRG_USERNAME/RBRG_PAT

RBGD overlap is intentional kindling — no action needed.
RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active — leave as-is.

Tasks

1. Remove 8 dead variables from RBRR-RegimeRepo.adoc documentation
2. Remove dead variables from rbrr_regime.sh validation if present
3. Fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk:49
4. Fix RBV_USERNAME/RBV_PAT doc mismatch in RBS-Specification.adoc
5. Audit RBS-Specification.adoc against RBS0-SpecTop.adoc:
   - Compare content coverage (is everything in RBS also in RBSA?)
   - Identify any unique content in RBS not present in RBSA
   - Determine if RBS can be deleted or needs content migrated first
6. If RBS is deletable, propose deletion (separate commit or same pace)

Acceptance

- Dead variables removed from spec and validation
- Bugs fixed
- Clear determination on RBS fate with evidence

**[260213-1000] rough**

Clean up dead RBRR variables identified by full-codebase audit, fix discovered bugs, and determine if RBS-Specification.adoc is redundant with RBS0-SpecTop.adoc.

Audit results (completed)

8 dead variables to remove:
- RBRR_REGISTRY_SERVER — spec-only, never implemented
- RBRR_BUILD_ARCHITECTURES — only in ABANDONED-github/
- RBRR_HISTORY_DIR — only in ABANDONED-github/
- RBRR_MACHINE_NAME — superseded by IGNITE/DEPLOY split
- RBRR_ENCLAVE_SUBNET — mentioned in spec term def, never implemented
- RBRR_VMDIST_TAG — Makefile-only, not in regime validation
- RBRR_VMDIST_BLOB_SHA — Makefile-only, not in regime validation
- RBRR_VMDIST_CRANE — Makefile-only, not in regime validation

2 bugs to fix:
- rbw.workbench.mk:49 uses RBV_GITHUB_PAT_ENV (typo, should be RBRR_GITHUB_PAT_ENV)
- RBS-Specification.adoc:1010 says RBV_USERNAME/RBV_PAT but code uses RBRG_USERNAME/RBRG_PAT

RBGD overlap is intentional kindling — no action needed.
RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active — leave as-is.

Tasks

1. Remove 8 dead variables from RBRR-RegimeRepo.adoc documentation
2. Remove dead variables from rbrr_regime.sh validation if present
3. Fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk:49
4. Fix RBV_USERNAME/RBV_PAT doc mismatch in RBS-Specification.adoc
5. Audit RBS-Specification.adoc against RBS0-SpecTop.adoc:
   - Compare content coverage (is everything in RBS also in RBSA?)
   - Identify any unique content in RBS not present in RBSA
   - Determine if RBS can be deleted or needs content migrated first
6. If RBS is deletable, propose deletion (separate commit or same pace)

Acceptance

- Dead variables removed from spec and validation
- Bugs fixed
- Clear determination on RBS fate with evidence

**[260201-1953] rough**

Audit RBRR regime for unused/legacy variables from GitHub Container Registry era.

Background

RBRR originally targeted GitHub Container Registry (GHCR). The project has since moved to Google Artifact Registry (GAR). Some RBRR variables may be vestigial.

Suspect variables (from RBRR-RegimeRepo.adoc and rbrr_regime.sh)

- RBRR_REGISTRY_SERVER — generic registry server (GHCR era?)
- RBRR_REGISTRY_OWNER — GitHub username/org
- RBRR_REGISTRY_NAME — GitHub repo name
- RBRR_GITHUB_PAT_ENV — GitHub credentials file

Variables that appear GAR-native

- RBRR_GAR_REPOSITORY — GAR repository name
- RBRR_DEPOT_PROJECT_ID — GCP project (but also see RBGD_GAR_PROJECT_ID overlap)
- RBRR_GCP_REGION — GCP region (but also see RBGD_GAR_LOCATION overlap)

Tasks

1. Grep for each RBRR_REGISTRY_* and RBRR_GITHUB_* variable usage
2. Determine if any are still referenced in active code paths
3. Check for RBGD/RBRR overlap (DEPOT_PROJECT_ID vs GAR_PROJECT_ID, GCP_REGION vs GAR_LOCATION)
4. Propose which to remove vs consolidate
5. Update RBRR-RegimeRepo.adoc to remove stale documentation
6. Update rbrr_regime.sh validation if variables are removed

Acceptance

- Clear inventory of which RBRR variables are live vs dead
- Spec changes proposed (separate pace for actual removal)

### cleanup-rbrr-dead-vm-vars (₢ATAAe) [abandoned]

**[260214-0636] abandoned**

Remove 8 dead RBRR variables from RBRR-RegimeRepo.adoc and fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk.

Note: This pace replaces ₢ATAAF (cleanup-rbrr-legacy-variables) which covered the same scope. ₢ATAAF should be abandoned.

Dead variables to remove from spec (table entries):
- RBRR_REGISTRY_SERVER (lines 72-82)
- RBRR_BUILD_ARCHITECTURES (lines 123-134)
- RBRR_HISTORY_DIR (lines 136-148)
- RBRR_MACHINE_NAME (lines 150-162)
- RBRR_VMDIST_TAG (lines 164-174)
- RBRR_VMDIST_BLOB_SHA (lines 176-186)
- RBRR_VMDIST_CRANE (lines 188-198)

Inline reference to remove:
- RBRR_ENCLAVE_SUBNET in term_enclave_network definition (line 209)

Bug fix:
- rbw.workbench.mk:49 — change RBV_GITHUB_PAT_ENV to RBRR_GITHUB_PAT_ENV

Notes:
- rbrr_regime.sh is already clean (none of the 8 appear there)
- RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active, leave as-is
- RBS-Specification.adoc fate deferred to heat AS pace ASAAI
- Removing VMDIST/MACHINE entries is safe: superseded by RBRR_CHOSEN_* and RBRR_IGNITE/DEPLOY_MACHINE_NAME in modern bash (rbv_PodmanVM.sh)

Acceptance:
- Dead variables removed from spec
- Bug fixed
- No orphaned section headers remain (Build Configuration and Virtual Machine Configuration sections fully removed)

**[260214-0615] rough**

Remove 8 dead RBRR variables from RBRR-RegimeRepo.adoc and fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk.

Note: This pace replaces ₢ATAAF (cleanup-rbrr-legacy-variables) which covered the same scope. ₢ATAAF should be abandoned.

Dead variables to remove from spec (table entries):
- RBRR_REGISTRY_SERVER (lines 72-82)
- RBRR_BUILD_ARCHITECTURES (lines 123-134)
- RBRR_HISTORY_DIR (lines 136-148)
- RBRR_MACHINE_NAME (lines 150-162)
- RBRR_VMDIST_TAG (lines 164-174)
- RBRR_VMDIST_BLOB_SHA (lines 176-186)
- RBRR_VMDIST_CRANE (lines 188-198)

Inline reference to remove:
- RBRR_ENCLAVE_SUBNET in term_enclave_network definition (line 209)

Bug fix:
- rbw.workbench.mk:49 — change RBV_GITHUB_PAT_ENV to RBRR_GITHUB_PAT_ENV

Notes:
- rbrr_regime.sh is already clean (none of the 8 appear there)
- RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active, leave as-is
- RBS-Specification.adoc fate deferred to heat AS pace ASAAI
- Removing VMDIST/MACHINE entries is safe: superseded by RBRR_CHOSEN_* and RBRR_IGNITE/DEPLOY_MACHINE_NAME in modern bash (rbv_PodmanVM.sh)

Acceptance:
- Dead variables removed from spec
- Bug fixed
- No orphaned section headers remain (Build Configuration and Virtual Machine Configuration sections fully removed)

**[260214-0614] rough**

Remove 8 dead RBRR variables from RBRR-RegimeRepo.adoc and fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk.

Note: This pace replaces ₢ATAAF (cleanup-rbrr-legacy-variables) which covered the same scope. ₢ATAAF should be abandoned.

Dead variables to remove from spec (table entries):
- RBRR_REGISTRY_SERVER (lines 72-82)
- RBRR_BUILD_ARCHITECTURES (lines 123-134)
- RBRR_HISTORY_DIR (lines 136-148)
- RBRR_MACHINE_NAME (lines 150-162)
- RBRR_VMDIST_TAG (lines 164-174)
- RBRR_VMDIST_BLOB_SHA (lines 176-186)
- RBRR_VMDIST_CRANE (lines 188-198)

Inline reference to remove:
- RBRR_ENCLAVE_SUBNET in term_enclave_network definition (line 209)

Bug fix:
- rbw.workbench.mk:49 — change RBV_GITHUB_PAT_ENV to RBRR_GITHUB_PAT_ENV

Notes:
- rbrr_regime.sh is already clean (none of the 8 appear there)
- RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active, leave as-is
- RBS-Specification.adoc fate deferred to heat AS pace ASAAI
- Removing VMDIST/MACHINE entries is safe: superseded by RBRR_CHOSEN_* and RBRR_IGNITE/DEPLOY_MACHINE_NAME in modern bash (rbv_PodmanVM.sh)

Acceptance:
- Dead variables removed from spec
- Bug fixed
- No orphaned section headers remain (Build Configuration and Virtual Machine Configuration sections fully removed)

**[260214-0610] rough**

Remove 8 dead RBRR variables from RBRR-RegimeRepo.adoc and fix RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk.

Dead variables to remove from spec (table entries):
- RBRR_REGISTRY_SERVER (lines 72-82)
- RBRR_BUILD_ARCHITECTURES (lines 123-134)
- RBRR_HISTORY_DIR (lines 136-148)
- RBRR_MACHINE_NAME (lines 150-162)
- RBRR_VMDIST_TAG (lines 164-174)
- RBRR_VMDIST_BLOB_SHA (lines 176-186)
- RBRR_VMDIST_CRANE (lines 188-198)

Inline reference to remove:
- RBRR_ENCLAVE_SUBNET in term_enclave_network definition (line 209)

Bug fix:
- rbw.workbench.mk:49 — change RBV_GITHUB_PAT_ENV to RBRR_GITHUB_PAT_ENV

Notes:
- rbrr_regime.sh is already clean (none of the 8 appear there)
- RBRR_GITHUB_PAT_ENV and RBRR_NAMEPLATE_PATH are partially active, leave as-is
- RBS-Specification.adoc fate deferred to heat AS pace ASAAI
- Removing VMDIST/MACHINE entries is safe: superseded by RBRR_CHOSEN_* and RBRR_IGNITE/DEPLOY_MACHINE_NAME in modern bash (rbv_PodmanVM.sh)

Acceptance:
- Dead variables removed from spec
- Bug fixed
- No orphaned section headers remain (Build Configuration and Virtual Machine Configuration sections fully removed)

### consolidate-podman-vm-spec (₢ATAAf) [complete]

**[260214-0659] complete**

Create consolidated RBSPV-PodmanVmSupplyChain.adoc. Add warning to rbv_PodmanVM.sh.

New document sections (prose, NOT linked terms):

1. Motivation
   - Quay.io regression: deleted and redownloaded a VM using the same Podman SemVer, silently got a different image
   - The regression manifested as broken virtual networking; systematic investigation (Study/study-net-namespace-permutes/) revealed Podman 5.3+ hardened setns() permissions on external network namespaces — the root cause was not immediately apparent
   - The definitive post-mortem: Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md
   - Upstream (quay.io) rebuilds every 3 hours and deletes old versions; tags are not immutable within a Podman SemVer
   - This motivated bringing VM images into user-controlled Depot (artifact registry)

2. Constraints
   - VM images are OCI artifacts, not standard container images (compressed rootfs tarballs for WSL, CoreOS disk images for standard; non-standard media types; empty artifactType; custom annotations like disktype=wsl; platform-indexed but not platform-runnable)
   - Bootstrap chicken-and-egg: need crane/oras/skopeo but don't want to obligate user to install them; use ignite VM (initialized with uncontrolled latest image) as disposable tooling environment
   - Provenance difficulty: podman machine init prints SHA during initialization that can be captured and compared, but only at init time — no reliable way to verify after the fact
   - VM locked to Podman SemVer but upstream images not immutable within a version
   - Two distinct image families: machine-os-wsl (WSL) and machine-os (standard/CoreOS) with different build processes and artifact structures
   - Storage cost is low: users don't frequently update the Podman VM, so maintaining a few full images in the Depot is practical

3. Architecture
   - Two-machine pattern: ignite (temporary, disposable tooling) vs deploy (persistent, controlled)
   - Lifecycle overview: check -> mirror -> fetch -> init -> start/stop -> nuke

4. Operations (brief prose descriptions of each)
   - Check: discover upstream updates by comparing digests (unimplemented in bash)
   - Mirror: query upstream manifests for WSL + standard families, download individual disk blobs, repackage as container images, push to registry with platform-specific tags
   - Fetch: pull platform-specific container from registry, extract disk image to local cache
   - Init: initialize deploy VM from cached disk image, write brand file with provenance
   - Start/Stop: start or stop the deploy VM
   - Nuke: destroy all VMs (ignite + deploy) and local cache

5. Configuration
   - List RBRR variables used by VM management (prose table, not linked terms):
     RBRR_IGNITE_MACHINE_NAME, RBRR_DEPLOY_MACHINE_NAME, RBRR_CRANE_TAR_GZ,
     RBRR_CHOSEN_PODMAN_VERSION, RBRR_CHOSEN_VMIMAGE_ORIGIN, RBRR_CHOSEN_IDENTITY,
     RBRR_MANIFEST_PLATFORMS
   - Station variables (per-workstation, not per-repo):
     RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
   - Note: these will eventually be pulled out of RBRR into a dedicated VM regime
   - Note: GHCR-era variables (RBRR_REGISTRY_OWNER, RBRR_REGISTRY_NAME, RBRR_GITHUB_PAT_ENV) are being deleted in ATAAg; when GAR support is implemented, new proper variables will be minted

6. Implementation Status
   - rbv_PodmanVM.sh exists targeting GHCR, never ported to GAR
   - Check operation (former RBSVC) has no bash implementation
   - Former specs (RBSVC/RBSVM) described an earlier, simpler design: whole-image crane copy with variables rbrr_chosen_vmimage_fqin and rbrr_chosen_vmimage_sha that were never implemented in code (spec-only ghosts)
   - Bash evolved well past the specs: blob-level extraction, repackaging as container images, multi-family (WSL + standard), multi-platform iteration, identity stamping
   - vme.extractor.sh was a standalone manifest introspection tool superseded by zrbv_process_image_type() in the modern bash
   - rbp.podman.mk contains historical Makefile prototypes from the GHCR era, fully superseded by rbv_PodmanVM.sh

Warning for rbv_PodmanVM.sh:
- Add header comment block: "WARNING: This implementation targets GHCR (GitHub Container Registry). It has NOT been ported to Google Artifact Registry (GAR/Depot). The operations are functionally correct but the registry target is obsolete for this project. See lenses/RBSPV-PodmanVmSupplyChain.adoc for context."

Reference material (in this repo):
- Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md (definitive post-mortem)
- Study/study-net-namespace-permutes/README.md (systematic test protocol)
- podman-gateway-proposal.md (networking journey leading to regression)
- Study/study-shared-pod-userid/ and study-shared-pod-ip-forward/ (iptables architecture failures)

Acceptance:
- RBSPV-PodmanVmSupplyChain.adoc created in lenses/
- All six sections present with substantive prose
- rbv_PodmanVM.sh has warning banner
- No linked terms introduced; prose only
- RBSA integration deferred to ATAAg (which handles all RBSA edits)

**[260214-0637] rough**

Create consolidated RBSPV-PodmanVmSupplyChain.adoc. Add warning to rbv_PodmanVM.sh.

New document sections (prose, NOT linked terms):

1. Motivation
   - Quay.io regression: deleted and redownloaded a VM using the same Podman SemVer, silently got a different image
   - The regression manifested as broken virtual networking; systematic investigation (Study/study-net-namespace-permutes/) revealed Podman 5.3+ hardened setns() permissions on external network namespaces — the root cause was not immediately apparent
   - The definitive post-mortem: Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md
   - Upstream (quay.io) rebuilds every 3 hours and deletes old versions; tags are not immutable within a Podman SemVer
   - This motivated bringing VM images into user-controlled Depot (artifact registry)

2. Constraints
   - VM images are OCI artifacts, not standard container images (compressed rootfs tarballs for WSL, CoreOS disk images for standard; non-standard media types; empty artifactType; custom annotations like disktype=wsl; platform-indexed but not platform-runnable)
   - Bootstrap chicken-and-egg: need crane/oras/skopeo but don't want to obligate user to install them; use ignite VM (initialized with uncontrolled latest image) as disposable tooling environment
   - Provenance difficulty: podman machine init prints SHA during initialization that can be captured and compared, but only at init time — no reliable way to verify after the fact
   - VM locked to Podman SemVer but upstream images not immutable within a version
   - Two distinct image families: machine-os-wsl (WSL) and machine-os (standard/CoreOS) with different build processes and artifact structures
   - Storage cost is low: users don't frequently update the Podman VM, so maintaining a few full images in the Depot is practical

3. Architecture
   - Two-machine pattern: ignite (temporary, disposable tooling) vs deploy (persistent, controlled)
   - Lifecycle overview: check -> mirror -> fetch -> init -> start/stop -> nuke

4. Operations (brief prose descriptions of each)
   - Check: discover upstream updates by comparing digests (unimplemented in bash)
   - Mirror: query upstream manifests for WSL + standard families, download individual disk blobs, repackage as container images, push to registry with platform-specific tags
   - Fetch: pull platform-specific container from registry, extract disk image to local cache
   - Init: initialize deploy VM from cached disk image, write brand file with provenance
   - Start/Stop: start or stop the deploy VM
   - Nuke: destroy all VMs (ignite + deploy) and local cache

5. Configuration
   - List RBRR variables used by VM management (prose table, not linked terms):
     RBRR_IGNITE_MACHINE_NAME, RBRR_DEPLOY_MACHINE_NAME, RBRR_CRANE_TAR_GZ,
     RBRR_CHOSEN_PODMAN_VERSION, RBRR_CHOSEN_VMIMAGE_ORIGIN, RBRR_CHOSEN_IDENTITY,
     RBRR_MANIFEST_PLATFORMS
   - Station variables (per-workstation, not per-repo):
     RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
   - Note: these will eventually be pulled out of RBRR into a dedicated VM regime
   - Note: GHCR-era variables (RBRR_REGISTRY_OWNER, RBRR_REGISTRY_NAME, RBRR_GITHUB_PAT_ENV) are being deleted in ATAAg; when GAR support is implemented, new proper variables will be minted

6. Implementation Status
   - rbv_PodmanVM.sh exists targeting GHCR, never ported to GAR
   - Check operation (former RBSVC) has no bash implementation
   - Former specs (RBSVC/RBSVM) described an earlier, simpler design: whole-image crane copy with variables rbrr_chosen_vmimage_fqin and rbrr_chosen_vmimage_sha that were never implemented in code (spec-only ghosts)
   - Bash evolved well past the specs: blob-level extraction, repackaging as container images, multi-family (WSL + standard), multi-platform iteration, identity stamping
   - vme.extractor.sh was a standalone manifest introspection tool superseded by zrbv_process_image_type() in the modern bash
   - rbp.podman.mk contains historical Makefile prototypes from the GHCR era, fully superseded by rbv_PodmanVM.sh

Warning for rbv_PodmanVM.sh:
- Add header comment block: "WARNING: This implementation targets GHCR (GitHub Container Registry). It has NOT been ported to Google Artifact Registry (GAR/Depot). The operations are functionally correct but the registry target is obsolete for this project. See lenses/RBSPV-PodmanVmSupplyChain.adoc for context."

Reference material (in this repo):
- Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md (definitive post-mortem)
- Study/study-net-namespace-permutes/README.md (systematic test protocol)
- podman-gateway-proposal.md (networking journey leading to regression)
- Study/study-shared-pod-userid/ and study-shared-pod-ip-forward/ (iptables architecture failures)

Acceptance:
- RBSPV-PodmanVmSupplyChain.adoc created in lenses/
- All six sections present with substantive prose
- rbv_PodmanVM.sh has warning banner
- No linked terms introduced; prose only
- RBSA integration deferred to ATAAg (which handles all RBSA edits)

**[260214-0637] rough**

Create consolidated RBSPV-PodmanVmSupplyChain.adoc. Add warning to rbv_PodmanVM.sh.

New document sections (prose, NOT linked terms):

1. Motivation
   - Quay.io regression: deleted and redownloaded a VM using the same Podman SemVer, silently got a different image
   - The regression manifested as broken virtual networking; systematic investigation (Study/study-net-namespace-permutes/) revealed Podman 5.3+ hardened setns() permissions on external network namespaces — the root cause was not immediately apparent
   - The definitive post-mortem: Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md
   - Upstream (quay.io) rebuilds every 3 hours and deletes old versions; tags are not immutable within a Podman SemVer
   - This motivated bringing VM images into user-controlled Depot (artifact registry)

2. Constraints
   - VM images are OCI artifacts, not standard container images (compressed rootfs tarballs for WSL, CoreOS disk images for standard; non-standard media types; empty artifactType; custom annotations like disktype=wsl; platform-indexed but not platform-runnable)
   - Bootstrap chicken-and-egg: need crane/oras/skopeo but don't want to obligate user to install them; use ignite VM (initialized with uncontrolled latest image) as disposable tooling environment
   - Provenance difficulty: podman machine init prints SHA during initialization that can be captured and compared, but only at init time — no reliable way to verify after the fact
   - VM locked to Podman SemVer but upstream images not immutable within a version
   - Two distinct image families: machine-os-wsl (WSL) and machine-os (standard/CoreOS) with different build processes and artifact structures
   - Storage cost is low: users don't frequently update the Podman VM, so maintaining a few full images in the Depot is practical

3. Architecture
   - Two-machine pattern: ignite (temporary, disposable tooling) vs deploy (persistent, controlled)
   - Lifecycle overview: check -> mirror -> fetch -> init -> start/stop -> nuke

4. Operations (brief prose descriptions of each)
   - Check: discover upstream updates by comparing digests (unimplemented in bash)
   - Mirror: query upstream manifests for WSL + standard families, download individual disk blobs, repackage as container images, push to registry with platform-specific tags
   - Fetch: pull platform-specific container from registry, extract disk image to local cache
   - Init: initialize deploy VM from cached disk image, write brand file with provenance
   - Start/Stop: start or stop the deploy VM
   - Nuke: destroy all VMs (ignite + deploy) and local cache

5. Configuration
   - List RBRR variables used by VM management (prose table, not linked terms):
     RBRR_IGNITE_MACHINE_NAME, RBRR_DEPLOY_MACHINE_NAME, RBRR_CRANE_TAR_GZ,
     RBRR_CHOSEN_PODMAN_VERSION, RBRR_CHOSEN_VMIMAGE_ORIGIN, RBRR_CHOSEN_IDENTITY,
     RBRR_MANIFEST_PLATFORMS
   - Station variables (per-workstation, not per-repo):
     RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
   - Note: these will eventually be pulled out of RBRR into a dedicated VM regime
   - Note: GHCR-era variables (RBRR_REGISTRY_OWNER, RBRR_REGISTRY_NAME, RBRR_GITHUB_PAT_ENV) are being deleted in ATAAg; when GAR support is implemented, new proper variables will be minted

6. Implementation Status
   - rbv_PodmanVM.sh exists targeting GHCR, never ported to GAR
   - Check operation (former RBSVC) has no bash implementation
   - Former specs (RBSVC/RBSVM) described an earlier, simpler design: whole-image crane copy with variables rbrr_chosen_vmimage_fqin and rbrr_chosen_vmimage_sha that were never implemented in code (spec-only ghosts)
   - Bash evolved well past the specs: blob-level extraction, repackaging as container images, multi-family (WSL + standard), multi-platform iteration, identity stamping
   - vme.extractor.sh was a standalone manifest introspection tool superseded by zrbv_process_image_type() in the modern bash
   - rbp.podman.mk contains historical Makefile prototypes from the GHCR era, fully superseded by rbv_PodmanVM.sh

Warning for rbv_PodmanVM.sh:
- Add header comment block: "WARNING: This implementation targets GHCR (GitHub Container Registry). It has NOT been ported to Google Artifact Registry (GAR/Depot). The operations are functionally correct but the registry target is obsolete for this project. See lenses/RBSPV-PodmanVmSupplyChain.adoc for context."

Reference material (in this repo):
- Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md (definitive post-mortem)
- Study/study-net-namespace-permutes/README.md (systematic test protocol)
- podman-gateway-proposal.md (networking journey leading to regression)
- Study/study-shared-pod-userid/ and study-shared-pod-ip-forward/ (iptables architecture failures)

Acceptance:
- RBSPV-PodmanVmSupplyChain.adoc created in lenses/
- All six sections present with substantive prose
- rbv_PodmanVM.sh has warning banner
- No linked terms introduced; prose only
- RBSA integration deferred to ATAAg (which handles all RBSA edits)

**[260214-0615] rough**

Create consolidated RBSV-PodmanVmSupplyChain.adoc replacing RBSVC + RBSVM. Add warning to rbv_PodmanVM.sh.

New document sections (prose, NOT linked terms):

1. Motivation
   - Quay.io regression: deleted and redownloaded a VM using the same Podman SemVer, silently got a different image
   - The regression manifested as broken virtual networking; systematic investigation (Study/study-net-namespace-permutes/) revealed Podman 5.3+ hardened setns() permissions on external network namespaces — the root cause was not immediately apparent
   - The definitive post-mortem: Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md
   - Upstream (quay.io) rebuilds every 3 hours and deletes old versions; tags are not immutable within a Podman SemVer
   - This motivated bringing VM images into user-controlled Depot (artifact registry)

2. Constraints
   - VM images are OCI artifacts, not standard container images (compressed rootfs tarballs for WSL, CoreOS disk images for standard; non-standard media types; empty artifactType; custom annotations like disktype=wsl; platform-indexed but not platform-runnable)
   - Bootstrap chicken-and-egg: need crane/oras/skopeo but don't want to obligate user to install them; use ignite VM (initialized with uncontrolled latest image) as disposable tooling environment
   - Provenance difficulty: podman machine init prints SHA during initialization that can be captured and compared, but only at init time — no reliable way to verify after the fact
   - VM locked to Podman SemVer but upstream images not immutable within a version
   - Two distinct image families exist: machine-os-wsl (WSL) and machine-os (standard/CoreOS) with different build processes and artifact structures

3. Architecture
   - Two-machine pattern: ignite (temporary, disposable tooling) vs deploy (persistent, controlled)
   - Lifecycle overview: check -> mirror -> fetch -> init -> start/stop -> nuke

4. Operations (brief prose descriptions of each)
   - Check: discover upstream updates by comparing digests (unimplemented in bash)
   - Mirror: query upstream manifests for WSL + standard families, download individual disk blobs, repackage as container images, push to registry with platform-specific tags
   - Fetch: pull platform-specific container from registry, extract disk image to local cache
   - Init: initialize deploy VM from cached disk image, write brand file with provenance
   - Start/Stop: start or stop the deploy VM
   - Nuke: destroy all VMs (ignite + deploy) and local cache

5. Configuration
   - List RBRR variables used by VM management (prose table, not linked terms):
     RBRR_IGNITE_MACHINE_NAME, RBRR_DEPLOY_MACHINE_NAME, RBRR_CRANE_TAR_GZ,
     RBRR_CHOSEN_PODMAN_VERSION, RBRR_CHOSEN_VMIMAGE_ORIGIN, RBRR_CHOSEN_IDENTITY,
     RBRR_MANIFEST_PLATFORMS, RBRR_REGISTRY_OWNER, RBRR_REGISTRY_NAME,
     RBRR_GITHUB_PAT_ENV (credential path for registry authentication, used by mirror/fetch)
   - Station variables (per-workstation, not per-repo):
     RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
   - Note: these will eventually be pulled out of RBRR into a dedicated VM regime

6. Implementation Status
   - rbv_PodmanVM.sh exists targeting GHCR, never ported to GAR
   - Check operation (former RBSVC) has no bash implementation
   - Former specs (RBSVC/RBSVM) described an earlier, simpler design: whole-image crane copy with variables rbrr_chosen_vmimage_fqin and rbrr_chosen_vmimage_sha that were never implemented in code (spec-only ghosts)
   - Bash evolved well past the specs: blob-level extraction, repackaging as container images, multi-family (WSL + standard), multi-platform iteration, identity stamping
   - vme.extractor.sh was a standalone manifest introspection tool (piped scripts into VM, structured JSON output) superseded by zrbv_process_image_type() in the modern bash
   - rbp.podman.mk contains historical Makefile prototypes from the GHCR era, fully superseded by rbv_PodmanVM.sh

Warning for rbv_PodmanVM.sh:
- Add header comment block: "WARNING: This implementation targets GHCR (GitHub Container Registry). It has NOT been ported to Google Artifact Registry (GAR/Depot). The operations are functionally correct but the registry target is obsolete for this project. See RBSV-PodmanVmSupplyChain.adoc for context."

Reference material (in this repo):
- Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md (definitive post-mortem)
- Study/study-net-namespace-permutes/README.md (systematic test protocol)
- podman-gateway-proposal.md (networking journey leading to regression)
- Study/study-shared-pod-userid/ and study-shared-pod-ip-forward/ (iptables architecture failures)

Acceptance:
- RBSV-PodmanVmSupplyChain.adoc created in lenses/
- All six sections present with substantive prose
- rbv_PodmanVM.sh has warning banner
- No linked terms used; prose only

**[260214-0615] rough**

Create consolidated RBSV-PodmanVmSupplyChain.adoc replacing RBSVC + RBSVM. Add warning to rbv_PodmanVM.sh.

New document sections (prose, NOT linked terms):

1. Motivation
   - Quay.io regression: deleted and redownloaded a VM using the same Podman SemVer, silently got a different image
   - The regression manifested as broken virtual networking; systematic investigation (Study/study-net-namespace-permutes/) revealed Podman 5.3+ hardened setns() permissions on external network namespaces — the root cause was not immediately apparent
   - The definitive post-mortem: Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md
   - Upstream (quay.io) rebuilds every 3 hours and deletes old versions; tags are not immutable within a Podman SemVer
   - This motivated bringing VM images into user-controlled Depot (artifact registry)

2. Constraints
   - VM images are OCI artifacts, not standard container images (compressed rootfs tarballs for WSL, CoreOS disk images for standard; non-standard media types; empty artifactType; custom annotations like disktype=wsl; platform-indexed but not platform-runnable)
   - Bootstrap chicken-and-egg: need crane/oras/skopeo but don't want to obligate user to install them; use ignite VM (initialized with uncontrolled latest image) as disposable tooling environment
   - Provenance difficulty: podman machine init prints SHA during initialization that can be captured and compared, but only at init time — no reliable way to verify after the fact
   - VM locked to Podman SemVer but upstream images not immutable within a version
   - Two distinct image families exist: machine-os-wsl (WSL) and machine-os (standard/CoreOS) with different build processes and artifact structures

3. Architecture
   - Two-machine pattern: ignite (temporary, disposable tooling) vs deploy (persistent, controlled)
   - Lifecycle overview: check -> mirror -> fetch -> init -> start/stop -> nuke

4. Operations (brief prose descriptions of each)
   - Check: discover upstream updates by comparing digests (unimplemented in bash)
   - Mirror: query upstream manifests for WSL + standard families, download individual disk blobs, repackage as container images, push to registry with platform-specific tags
   - Fetch: pull platform-specific container from registry, extract disk image to local cache
   - Init: initialize deploy VM from cached disk image, write brand file with provenance
   - Start/Stop: start or stop the deploy VM
   - Nuke: destroy all VMs (ignite + deploy) and local cache

5. Configuration
   - List RBRR variables used by VM management (prose table, not linked terms):
     RBRR_IGNITE_MACHINE_NAME, RBRR_DEPLOY_MACHINE_NAME, RBRR_CRANE_TAR_GZ,
     RBRR_CHOSEN_PODMAN_VERSION, RBRR_CHOSEN_VMIMAGE_ORIGIN, RBRR_CHOSEN_IDENTITY,
     RBRR_MANIFEST_PLATFORMS, RBRR_REGISTRY_OWNER, RBRR_REGISTRY_NAME,
     RBRR_GITHUB_PAT_ENV (credential path for registry authentication, used by mirror/fetch)
   - Station variables (per-workstation, not per-repo):
     RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
   - Note: these will eventually be pulled out of RBRR into a dedicated VM regime

6. Implementation Status
   - rbv_PodmanVM.sh exists targeting GHCR, never ported to GAR
   - Check operation (former RBSVC) has no bash implementation
   - Former specs (RBSVC/RBSVM) described an earlier, simpler design: whole-image crane copy with variables rbrr_chosen_vmimage_fqin and rbrr_chosen_vmimage_sha that were never implemented in code (spec-only ghosts)
   - Bash evolved well past the specs: blob-level extraction, repackaging as container images, multi-family (WSL + standard), multi-platform iteration, identity stamping
   - vme.extractor.sh was a standalone manifest introspection tool (piped scripts into VM, structured JSON output) superseded by zrbv_process_image_type() in the modern bash
   - rbp.podman.mk contains historical Makefile prototypes from the GHCR era, fully superseded by rbv_PodmanVM.sh

Warning for rbv_PodmanVM.sh:
- Add header comment block: "WARNING: This implementation targets GHCR (GitHub Container Registry). It has NOT been ported to Google Artifact Registry (GAR/Depot). The operations are functionally correct but the registry target is obsolete for this project. See RBSV-PodmanVmSupplyChain.adoc for context."

Reference material (in this repo):
- Study/study-net-namespace-permutes/claudeopus4-netns-fail-memo.md (definitive post-mortem)
- Study/study-net-namespace-permutes/README.md (systematic test protocol)
- podman-gateway-proposal.md (networking journey leading to regression)
- Study/study-shared-pod-userid/ and study-shared-pod-ip-forward/ (iptables architecture failures)

Acceptance:
- RBSV-PodmanVmSupplyChain.adoc created in lenses/
- All six sections present with substantive prose
- rbv_PodmanVM.sh has warning banner
- No linked terms used; prose only

**[260214-0610] rough**

Create consolidated RBSV-PodmanVmSupplyChain.adoc replacing RBSVC + RBSVM. Add warning to rbv_PodmanVM.sh.

New document sections (prose, NOT linked terms):

1. Motivation
   - Quay.io regression: deleting and redownloading a VM with same Podman SemVer silently got a different image
   - The new image had security hardening that prohibited setns() on external network namespaces — broke working SENTRY/BOTTLE architecture
   - Upstream rebuilds every 3 hours and deletes old versions; tags are not immutable
   - This motivated bringing VM images into user-controlled Depot

2. Constraints
   - VM images are OCI artifacts, not container images (compressed rootfs tarballs or CoreOS disk images, non-standard media types, empty artifactType, custom annotations like disktype=wsl)
   - Bootstrap chicken-and-egg: need crane/oras/skopeo but don't want to obligate user to install them; use ignite VM as tooling environment
   - Provenance difficulty: podman machine init prints SHA that can be captured and compared, but only during init
   - VM locked to Podman SemVer but upstream images not immutable within a version

3. Architecture
   - Two-machine pattern: ignite (temporary, disposable tooling) vs deploy (persistent, controlled)
   - Lifecycle overview: check -> mirror -> fetch -> init -> start/stop -> nuke

4. Operations (brief prose descriptions of each)
   - Check: discover upstream updates (unimplemented)
   - Mirror: query upstream manifests for WSL + standard families, download blobs, repackage as container images, push to registry
   - Fetch: pull platform-specific container from registry, extract disk image to local cache
   - Init: initialize deploy VM from cache, write brand file
   - Nuke: destroy all VMs and cache

5. Configuration
   - List RBRR variables used by VM management (prose table, not linked terms):
     RBRR_IGNITE_MACHINE_NAME, RBRR_DEPLOY_MACHINE_NAME, RBRR_CRANE_TAR_GZ,
     RBRR_CHOSEN_PODMAN_VERSION, RBRR_CHOSEN_VMIMAGE_ORIGIN, RBRR_CHOSEN_IDENTITY,
     RBRR_MANIFEST_PLATFORMS, RBRR_REGISTRY_OWNER, RBRR_REGISTRY_NAME
   - Station variables: RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
   - Note: these will eventually be pulled out of RBRR into a dedicated VM regime

6. Implementation Status
   - rbv_PodmanVM.sh exists targeting GHCR, never ported to GAR
   - Check operation has no implementation
   - Specs (former RBSVC/RBSVM) described earlier crane-copy model; bash evolved to blob-level repackaging
   - rbp.podman.mk contains historical prototypes, superseded by rbv_PodmanVM.sh

Warning for rbv_PodmanVM.sh:
- Add header comment block: "WARNING: This implementation targets GHCR (GitHub Container Registry). It has NOT been ported to Google Artifact Registry (GAR/Depot). The operations are functionally correct but the registry target is obsolete for this project. See RBSV-PodmanVmSupplyChain.adoc for context."

Reference material (in this repo):
- Study/study-net-namespace-permutes/ (regression characterization)
- podman-gateway-proposal.md (networking journey)
- Study/study-shared-pod-userid/ and study-shared-pod-ip-forward/ (iptables failures)

Acceptance:
- RBSV-PodmanVmSupplyChain.adoc created in lenses/
- All six sections present with substantive prose
- rbv_PodmanVM.sh has warning banner
- No linked terms used; prose only

### delete-ghcr-legacy-vm-files (₢ATAAg) [complete]

**[260214-0728] complete**

Delete superseded VM management files, clean RBSA, remove GHCR-era plumbing. Depends on ATAAf completing first.

Files to delete:
- lenses/RBSVC-rbv_check.adoc
- lenses/RBSVM-rbv_mirror.adoc
- Tools/rbw/rbp.podman.mk (fully deletable — all living rules are being removed from rbw.workbench.mk)
- Tools/vme.extractor.sh
- tt/rbw-z.PodmanStop.sh
- tt/rbw-Z.PodmanNuke.sh

rbw.workbench.mk changes:
- Remove line 52: include $(MBV_TOOLS_DIR)/rbp.podman.mk
- Remove line 79: rbw-a.% rule (rbp_podman_machine_start_rule rbp_check_connection)
- Remove line 82: rbw-z.% rule (rbp_podman_machine_stop_rule)
- Remove line 85: rbw-Z.% rule (rbp_podman_machine_nuke_rule)

GHCR-era variable removal (no longer needed; GAR equivalents will be minted when podman VM is reimplemented):
- RBRR_REGISTRY_OWNER — remove from rbrr_regime.sh (broach default, known list, rollup, validation), RBRR-RegimeRepo.adoc, RBSA linked terms, rbrr_RecipeBottleRegimeRepo.sh
- RBRR_REGISTRY_NAME — same locations
- RBRR_GITHUB_PAT_ENV — same locations

RBS0-SpecTop.adoc cleanup:
1. Remove attribute references (mapping section):
   - :ops_rbv_check:, :ops_rbv_mirror:
   - :at_stash_machine:, :at_operational_machine:
   - :rbrr_ignite_machine_name:, :rbrr_deploy_machine_name:
   - :rbrr_crane_tar_gz:, :rbrr_chosen_podman_version:
   - :rbrr_chosen_vmimage_origin:, :rbrr_chosen_identity:
   - :rbrr_manifest_platforms:
   - :rbrr_registry_owner:, :rbrr_registry_name:, :rbrr_github_pat_env: (if they exist as linked terms)

2. Remove definition anchors and definitions:
   - [[at_stash_machine]], [[at_operational_machine]]
   - [[ops_rbv_check]] section, [[ops_rbv_mirror]] section
   - All [[rbrr_*]] anchors for the variables listed above
   - The === VM Management Operations header

3. Replace old VM Management Operations section with:
   === Podman VM Supply Chain
   include::RBSPV-PodmanVmSupplyChain.adoc[]

4. Before removing each attribute reference, verify no remaining cross-references in other adoc files outside the VM section.

Cross-reference check before each deletion:
- Grep for each term/file across all .adoc, .sh, .mk files
- Only delete if no remaining references
- Stale itch entries (jji_itch.md lines 1310-1364) noted but left as-is (historical)

Acceptance:
- All six files deleted (2 spec, 1 makefile, 1 extractor, 2 tabtargets)
- Make rules and include removed from rbw.workbench.mk
- GHCR variables removed from regime validator, config file, spec, and RBSA
- RBSA cleaned: old terms removed, new RBSPV included
- No dangling cross-references remain
- rbrr_RecipeBottleRegimeRepo.sh updated (GHCR vars removed or commented)

**[260214-0637] rough**

Delete superseded VM management files, clean RBSA, remove GHCR-era plumbing. Depends on ATAAf completing first.

Files to delete:
- lenses/RBSVC-rbv_check.adoc
- lenses/RBSVM-rbv_mirror.adoc
- Tools/rbw/rbp.podman.mk (fully deletable — all living rules are being removed from rbw.workbench.mk)
- Tools/vme.extractor.sh
- tt/rbw-z.PodmanStop.sh
- tt/rbw-Z.PodmanNuke.sh

rbw.workbench.mk changes:
- Remove line 52: include $(MBV_TOOLS_DIR)/rbp.podman.mk
- Remove line 79: rbw-a.% rule (rbp_podman_machine_start_rule rbp_check_connection)
- Remove line 82: rbw-z.% rule (rbp_podman_machine_stop_rule)
- Remove line 85: rbw-Z.% rule (rbp_podman_machine_nuke_rule)

GHCR-era variable removal (no longer needed; GAR equivalents will be minted when podman VM is reimplemented):
- RBRR_REGISTRY_OWNER — remove from rbrr_regime.sh (broach default, known list, rollup, validation), RBRR-RegimeRepo.adoc, RBSA linked terms, rbrr_RecipeBottleRegimeRepo.sh
- RBRR_REGISTRY_NAME — same locations
- RBRR_GITHUB_PAT_ENV — same locations

RBS0-SpecTop.adoc cleanup:
1. Remove attribute references (mapping section):
   - :ops_rbv_check:, :ops_rbv_mirror:
   - :at_stash_machine:, :at_operational_machine:
   - :rbrr_ignite_machine_name:, :rbrr_deploy_machine_name:
   - :rbrr_crane_tar_gz:, :rbrr_chosen_podman_version:
   - :rbrr_chosen_vmimage_origin:, :rbrr_chosen_identity:
   - :rbrr_manifest_platforms:
   - :rbrr_registry_owner:, :rbrr_registry_name:, :rbrr_github_pat_env: (if they exist as linked terms)

2. Remove definition anchors and definitions:
   - [[at_stash_machine]], [[at_operational_machine]]
   - [[ops_rbv_check]] section, [[ops_rbv_mirror]] section
   - All [[rbrr_*]] anchors for the variables listed above
   - The === VM Management Operations header

3. Replace old VM Management Operations section with:
   === Podman VM Supply Chain
   include::RBSPV-PodmanVmSupplyChain.adoc[]

4. Before removing each attribute reference, verify no remaining cross-references in other adoc files outside the VM section.

Cross-reference check before each deletion:
- Grep for each term/file across all .adoc, .sh, .mk files
- Only delete if no remaining references
- Stale itch entries (jji_itch.md lines 1310-1364) noted but left as-is (historical)

Acceptance:
- All six files deleted (2 spec, 1 makefile, 1 extractor, 2 tabtargets)
- Make rules and include removed from rbw.workbench.mk
- GHCR variables removed from regime validator, config file, spec, and RBSA
- RBSA cleaned: old terms removed, new RBSPV included
- No dangling cross-references remain
- rbrr_RecipeBottleRegimeRepo.sh updated (GHCR vars removed or commented)

**[260214-0637] rough**

Delete superseded VM management files, clean RBSA, remove GHCR-era plumbing. Depends on ATAAf completing first.

Files to delete:
- lenses/RBSVC-rbv_check.adoc
- lenses/RBSVM-rbv_mirror.adoc
- Tools/rbw/rbp.podman.mk (fully deletable — all living rules are being removed from rbw.workbench.mk)
- Tools/vme.extractor.sh
- tt/rbw-z.PodmanStop.sh
- tt/rbw-Z.PodmanNuke.sh

rbw.workbench.mk changes:
- Remove line 52: include $(MBV_TOOLS_DIR)/rbp.podman.mk
- Remove line 79: rbw-a.% rule (rbp_podman_machine_start_rule rbp_check_connection)
- Remove line 82: rbw-z.% rule (rbp_podman_machine_stop_rule)
- Remove line 85: rbw-Z.% rule (rbp_podman_machine_nuke_rule)

GHCR-era variable removal (no longer needed; GAR equivalents will be minted when podman VM is reimplemented):
- RBRR_REGISTRY_OWNER — remove from rbrr_regime.sh (broach default, known list, rollup, validation), RBRR-RegimeRepo.adoc, RBSA linked terms, rbrr_RecipeBottleRegimeRepo.sh
- RBRR_REGISTRY_NAME — same locations
- RBRR_GITHUB_PAT_ENV — same locations

RBS0-SpecTop.adoc cleanup:
1. Remove attribute references (mapping section):
   - :ops_rbv_check:, :ops_rbv_mirror:
   - :at_stash_machine:, :at_operational_machine:
   - :rbrr_ignite_machine_name:, :rbrr_deploy_machine_name:
   - :rbrr_crane_tar_gz:, :rbrr_chosen_podman_version:
   - :rbrr_chosen_vmimage_origin:, :rbrr_chosen_identity:
   - :rbrr_manifest_platforms:
   - :rbrr_registry_owner:, :rbrr_registry_name:, :rbrr_github_pat_env: (if they exist as linked terms)

2. Remove definition anchors and definitions:
   - [[at_stash_machine]], [[at_operational_machine]]
   - [[ops_rbv_check]] section, [[ops_rbv_mirror]] section
   - All [[rbrr_*]] anchors for the variables listed above
   - The === VM Management Operations header

3. Replace old VM Management Operations section with:
   === Podman VM Supply Chain
   include::RBSPV-PodmanVmSupplyChain.adoc[]

4. Before removing each attribute reference, verify no remaining cross-references in other adoc files outside the VM section.

Cross-reference check before each deletion:
- Grep for each term/file across all .adoc, .sh, .mk files
- Only delete if no remaining references
- Stale itch entries (jji_itch.md lines 1310-1364) noted but left as-is (historical)

Acceptance:
- All six files deleted (2 spec, 1 makefile, 1 extractor, 2 tabtargets)
- Make rules and include removed from rbw.workbench.mk
- GHCR variables removed from regime validator, config file, spec, and RBSA
- RBSA cleaned: old terms removed, new RBSPV included
- No dangling cross-references remain
- rbrr_RecipeBottleRegimeRepo.sh updated (GHCR vars removed or commented)

**[260214-0616] rough**

Delete superseded VM management files and clean RBSA linked terms. Depends on ₢ATAAf (consolidate-podman-vm-spec) completing first.

Files to delete:
- lenses/RBSVC-rbv_check.adoc
- lenses/RBSVM-rbv_mirror.adoc
- Tools/vme.extractor.sh

Conditional deletion (requires verification):
- Tools/rbw/rbp.podman.mk — this file has TWO distinct sections:
  (a) Stash/VM-image machinery (dead, superseded by rbv_PodmanVM.sh)
  (b) Living Makefile infrastructure that MAY still be in use:
      - Lines 21-26: RBM_SENTRY_CONTAINER, RBM_CENSER_CONTAINER, RBM_BOTTLE_CONTAINER, RBM_ENCLAVE_NETWORK, RBM_MACHINE, RBM_CONNECTION
      - Lines 29-38: zRBM_EXPORT_ENV, zRBM_PODMAN_RAW_CMD, zRBM_PODMAN_SSH_CMD
      - Line 40: zrbp_validate_regimes_rule
      - Lines 181-218: rbp_podman_machine_start/stop/nuke_rule, rbp_check_connection
  Before deleting: grep for all rule names and variables across .mk files and tabtargets.
  If living rules are still referenced: either migrate them elsewhere or split the file, deleting only the stash machinery.

RBS0-SpecTop.adoc cleanup:
1. Remove attribute references (mapping section):
   - :ops_rbv_check:
   - :ops_rbv_mirror:
   - :rbrr_ignite_machine_name:
   - :rbrr_deploy_machine_name:
   - :rbrr_crane_tar_gz:
   - :rbrr_chosen_podman_version:
   - :rbrr_chosen_vmimage_origin:
   - :rbrr_chosen_identity:
   - :at_stash_machine:
   - :at_operational_machine:

2. Remove definition anchors and definitions (check BOTH RBS0-SpecTop.adoc and RBSRR-RegimeRepo.adoc):
   - [[at_stash_machine]] and its definition
   - [[at_operational_machine]] and its definition
   - [[ops_rbv_check]] section including include::RBSVC-rbv_check.adoc[]
   - [[ops_rbv_mirror]] section including include::RBSVM-rbv_mirror.adoc[]
   - [[rbrr_ignite_machine_name]] and its definition
   - [[rbrr_deploy_machine_name]] and its definition
   - [[rbrr_crane_tar_gz]] and its definition
   - [[rbrr_chosen_podman_version]] and its definition
   - [[rbrr_chosen_vmimage_origin]] and its definition
   - [[rbrr_chosen_identity]] and its definition
   - The === VM Management Operations header

3. Before removing each attribute reference, verify it is not used elsewhere in RBSA prose outside the VM section (e.g., rbrr_chosen_podman_version might appear in other operation descriptions).

4. Replace the VM Management Operations section with a brief cross-reference note:
   "VM management operations (check, mirror, fetch, init, nuke) are documented in RBSV-PodmanVmSupplyChain.adoc. Implementation is in Tools/rbw/rbv_PodmanVM.sh (GHCR-era, not yet ported to GAR)."

5. rbw.workbench.mk: remove the include line for rbp.podman.mk (line 51: include $(MBV_TOOLS_DIR)/rbp.podman.mk) — but ONLY if rbp.podman.mk is fully deleted. If only partially cleaned, update the include path.

Cross-reference check before each deletion:
- Grep for each term/file across all .adoc, .sh, .mk files
- Only delete if no remaining references (or update references to point to new doc)
- Pay special attention to other RBS*.adoc files that may reference VM terms

Acceptance:
- RBSVC and RBSVM files deleted
- vme.extractor.sh deleted
- rbp.podman.mk fully deleted OR partially cleaned (with justification if living rules remain)
- RBSA attribute references and definitions removed
- Replacement cross-reference note in place
- No dangling cross-references remain
- Build/include paths updated

**[260214-0616] rough**

Delete superseded VM management files and clean RBSA linked terms. Depends on ₢ATAAf (consolidate-podman-vm-spec) completing first.

Files to delete:
- lenses/RBSVC-rbv_check.adoc
- lenses/RBSVM-rbv_mirror.adoc
- Tools/vme.extractor.sh

Conditional deletion (requires verification):
- Tools/rbw/rbp.podman.mk — this file has TWO distinct sections:
  (a) Stash/VM-image machinery (dead, superseded by rbv_PodmanVM.sh)
  (b) Living Makefile infrastructure that MAY still be in use:
      - Lines 21-26: RBM_SENTRY_CONTAINER, RBM_CENSER_CONTAINER, RBM_BOTTLE_CONTAINER, RBM_ENCLAVE_NETWORK, RBM_MACHINE, RBM_CONNECTION
      - Lines 29-38: zRBM_EXPORT_ENV, zRBM_PODMAN_RAW_CMD, zRBM_PODMAN_SSH_CMD
      - Line 40: zrbp_validate_regimes_rule
      - Lines 181-218: rbp_podman_machine_start/stop/nuke_rule, rbp_check_connection
  Before deleting: grep for all rule names and variables across .mk files and tabtargets.
  If living rules are still referenced: either migrate them elsewhere or split the file, deleting only the stash machinery.

RBS0-SpecTop.adoc cleanup:
1. Remove attribute references (mapping section):
   - :ops_rbv_check:
   - :ops_rbv_mirror:
   - :rbrr_ignite_machine_name:
   - :rbrr_deploy_machine_name:
   - :rbrr_crane_tar_gz:
   - :rbrr_chosen_podman_version:
   - :rbrr_chosen_vmimage_origin:
   - :rbrr_chosen_identity:
   - :at_stash_machine:
   - :at_operational_machine:

2. Remove definition anchors and definitions (check BOTH RBS0-SpecTop.adoc and RBSRR-RegimeRepo.adoc):
   - [[at_stash_machine]] and its definition
   - [[at_operational_machine]] and its definition
   - [[ops_rbv_check]] section including include::RBSVC-rbv_check.adoc[]
   - [[ops_rbv_mirror]] section including include::RBSVM-rbv_mirror.adoc[]
   - [[rbrr_ignite_machine_name]] and its definition
   - [[rbrr_deploy_machine_name]] and its definition
   - [[rbrr_crane_tar_gz]] and its definition
   - [[rbrr_chosen_podman_version]] and its definition
   - [[rbrr_chosen_vmimage_origin]] and its definition
   - [[rbrr_chosen_identity]] and its definition
   - The === VM Management Operations header

3. Before removing each attribute reference, verify it is not used elsewhere in RBSA prose outside the VM section (e.g., rbrr_chosen_podman_version might appear in other operation descriptions).

4. Replace the VM Management Operations section with a brief cross-reference note:
   "VM management operations (check, mirror, fetch, init, nuke) are documented in RBSV-PodmanVmSupplyChain.adoc. Implementation is in Tools/rbw/rbv_PodmanVM.sh (GHCR-era, not yet ported to GAR)."

5. rbw.workbench.mk: remove the include line for rbp.podman.mk (line 51: include $(MBV_TOOLS_DIR)/rbp.podman.mk) — but ONLY if rbp.podman.mk is fully deleted. If only partially cleaned, update the include path.

Cross-reference check before each deletion:
- Grep for each term/file across all .adoc, .sh, .mk files
- Only delete if no remaining references (or update references to point to new doc)
- Pay special attention to other RBS*.adoc files that may reference VM terms

Acceptance:
- RBSVC and RBSVM files deleted
- vme.extractor.sh deleted
- rbp.podman.mk fully deleted OR partially cleaned (with justification if living rules remain)
- RBSA attribute references and definitions removed
- Replacement cross-reference note in place
- No dangling cross-references remain
- Build/include paths updated

**[260214-0611] rough**

Delete superseded VM management files and clean RBSA linked terms. Depends on ₢ATAAf (consolidate-spec) completing first.

Files to delete:
- lenses/RBSVC-rbv_check.adoc
- lenses/RBSVM-rbv_mirror.adoc
- Tools/rbw/rbp.podman.mk
- Tools/vme.extractor.sh

RBS0-SpecTop.adoc cleanup:
1. Remove attribute references (mapping section):
   - :ops_rbv_check:
   - :ops_rbv_mirror:
   - :rbrr_ignite_machine_name:
   - :rbrr_deploy_machine_name:
   - :rbrr_crane_tar_gz:
   - :rbrr_chosen_podman_version:
   - :rbrr_chosen_vmimage_origin:
   - :rbrr_chosen_identity:
   - :at_stash_machine:
   - :at_operational_machine:

2. Remove definition anchors and definitions:
   - [[at_stash_machine]] and its definition
   - [[at_operational_machine]] and its definition
   - [[ops_rbv_check]] section including include::RBSVC-rbv_check.adoc[]
   - [[ops_rbv_mirror]] section including include::RBSVM-rbv_mirror.adoc[]
   - The === VM Management Operations header if now empty

3. Remove definition anchors for deleted rbrr_ terms (find each [[rbrr_*]] block)

4. Add replacement reference: brief note in RBSA where VM Management Operations was, pointing to RBSV-PodmanVmSupplyChain.adoc

5. rbw.workbench.mk: remove the include line for rbp.podman.mk if it exists

Cross-reference check before each deletion:
- Grep for each term/file across all .adoc, .sh, .mk files
- Only delete if no remaining references (or update references to point to new doc)

Acceptance:
- All four files deleted
- RBSA attribute references and definitions removed
- No dangling cross-references remain
- Build/include paths updated

### axla-kindle-and-cardinality (₢ATAAi) [complete]

**[260215-0744] complete**

Rename broach→kindle in AXLA and add cardinality dimensions and collection-level operation markers.

## File: Tools/cmk/vov_veiled/AXLA-Lexicon.adoc

## Rename broach→kindle

Rename all broach terms to kindle throughout AXLA:
- `axvr_broach` → `axvr_kindle` (mapping section attribute reference + anchor + definition)
- `axhro_broach` → `axhro_kindle` (mapping section attribute reference + anchor + definition)
- Update all prose references to "broach" that refer to this operation

Rationale: kindle is the established codebase convention (4 of 5 regime validators use it). validate and render already match their code-level names. Aligning kindle completes the set. The concept (parameterless, call-once initialization from assignments) is language-neutral, not bash-specific.

## Add cardinality dimensions

Add two new regime dimensions under "Regime Dimensions" section (alongside existing `axd_conditional`):

- `axrd_singleton` — Regime has exactly one assignment source. No enumeration or cross-instance operations apply.
- `axrd_manifold` — Regime has multiple assignment sources (one per instance). Enumeration and cross-instance operations are meaningful.

These qualify `axvr_regime` declarations, alongside existing provenance dimensions (`axrd_file_sourced`, `axrd_env_inherited`, `axrd_constructed`).

Add attribute references to mapping section:
- `:axrd_singleton: <<axrd_singleton,Singleton>>`
- `:axrd_manifold: <<axrd_manifold,Manifold>>`

## Add collection-level operation markers

Add three new hierarchy operation markers (alongside existing kindle/validate/render):

- `axhro_list` — Marks a regime list operation in a subdocument. Enumerates available instances. Only valid on `axrd_manifold` regimes.
- `axhro_survey` — Marks a regime survey operation. Read-only cross-instance summary. Only valid on `axrd_manifold` regimes.
- `axhro_audit` — Marks a regime audit operation. Cross-instance validation plus external verification. Only valid on `axrd_manifold` regimes.

Add corresponding definition-site voicing terms:
- `axvr_list` — Voices a regime list operation definition
- `axvr_survey` — Voices a regime survey operation definition
- `axvr_audit` — Voices a regime audit operation definition

Add all attribute references to mapping section.

## Do NOT change
- Any file other than AXLA-Lexicon.adoc
- Provenance dimensions (axrd_file_sourced etc.) — leave as-is
- Any non-regime uses of dimensions

**[260215-0713] rough**

Rename broach→kindle in AXLA and add cardinality dimensions and collection-level operation markers.

## File: Tools/cmk/vov_veiled/AXLA-Lexicon.adoc

## Rename broach→kindle

Rename all broach terms to kindle throughout AXLA:
- `axvr_broach` → `axvr_kindle` (mapping section attribute reference + anchor + definition)
- `axhro_broach` → `axhro_kindle` (mapping section attribute reference + anchor + definition)
- Update all prose references to "broach" that refer to this operation

Rationale: kindle is the established codebase convention (4 of 5 regime validators use it). validate and render already match their code-level names. Aligning kindle completes the set. The concept (parameterless, call-once initialization from assignments) is language-neutral, not bash-specific.

## Add cardinality dimensions

Add two new regime dimensions under "Regime Dimensions" section (alongside existing `axd_conditional`):

- `axrd_singleton` — Regime has exactly one assignment source. No enumeration or cross-instance operations apply.
- `axrd_manifold` — Regime has multiple assignment sources (one per instance). Enumeration and cross-instance operations are meaningful.

These qualify `axvr_regime` declarations, alongside existing provenance dimensions (`axrd_file_sourced`, `axrd_env_inherited`, `axrd_constructed`).

Add attribute references to mapping section:
- `:axrd_singleton: <<axrd_singleton,Singleton>>`
- `:axrd_manifold: <<axrd_manifold,Manifold>>`

## Add collection-level operation markers

Add three new hierarchy operation markers (alongside existing kindle/validate/render):

- `axhro_list` — Marks a regime list operation in a subdocument. Enumerates available instances. Only valid on `axrd_manifold` regimes.
- `axhro_survey` — Marks a regime survey operation. Read-only cross-instance summary. Only valid on `axrd_manifold` regimes.
- `axhro_audit` — Marks a regime audit operation. Cross-instance validation plus external verification. Only valid on `axrd_manifold` regimes.

Add corresponding definition-site voicing terms:
- `axvr_list` — Voices a regime list operation definition
- `axvr_survey` — Voices a regime survey operation definition
- `axvr_audit` — Voices a regime audit operation definition

Add all attribute references to mapping section.

## Do NOT change
- Any file other than AXLA-Lexicon.adoc
- Provenance dimensions (axrd_file_sourced etc.) — leave as-is
- Any non-regime uses of dimensions

### rbsa-busa-vocabulary-alignment (₢ATAAj) [complete]

**[260215-0752] complete**

Update both parent spec documents to use renamed kindle terms, add collection-level operation terms, and annotate all regime declarations with cardinality dimensions.

## Files
- lenses/RBS0-SpecTop.adoc
- Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc

## RBSA changes

### Rename rbkro_broach → rbkro_kindle
- Mapping section: `:rbkro_broach:` → `:rbkro_kindle:`
- Anchor: `[[rbkro_broach]]` → `[[rbkro_kindle]]`
- Definition and all prose references

### Add collection-level operation terms
Add to mapping section and define with anchors:
- `rbkro_list` — Kit-level list operation for RBW regimes (voices `axvr_list`)
- `rbkro_survey` — Kit-level survey operation (voices `axvr_survey`)
- `rbkro_audit` — Kit-level audit operation (voices `axvr_audit`)

### Add cardinality to all regime declarations
Update every `// ⟦axvr_regime ...⟧` annotation to include cardinality:
- RBRR: `axrd_singleton` (one repo config)
- RBRN: `axrd_manifold` (3 nameplates)
- RBRV: `axrd_manifold` (6 vessels)
- RBRP: `axrd_singleton` (one payor project)
- RBRE: `axrd_manifold` (per-ECR environment)
- RBRG: `axrd_manifold` (per-GitHub config)
- RBRS: `axrd_singleton` (one workstation)
- RBRO: assess and annotate

## BUSA changes

### Rename bukro_broach → bukro_kindle
- Mapping section: `:bukro_broach:` → `:bukro_kindle:`
- Anchor: `[[bukro_broach]]` → `[[bukro_kindle]]`
- Definition and all prose references

### Add cardinality to regime declarations
- BURC: `axrd_singleton`
- BURS: `axrd_singleton`

## Do NOT change
- AXLA-Lexicon.adoc (done in prior pace)
- Subdocuments (done in subsequent paces)
- Shell scripts

**[260215-0713] rough**

Update both parent spec documents to use renamed kindle terms, add collection-level operation terms, and annotate all regime declarations with cardinality dimensions.

## Files
- lenses/RBS0-SpecTop.adoc
- Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc

## RBSA changes

### Rename rbkro_broach → rbkro_kindle
- Mapping section: `:rbkro_broach:` → `:rbkro_kindle:`
- Anchor: `[[rbkro_broach]]` → `[[rbkro_kindle]]`
- Definition and all prose references

### Add collection-level operation terms
Add to mapping section and define with anchors:
- `rbkro_list` — Kit-level list operation for RBW regimes (voices `axvr_list`)
- `rbkro_survey` — Kit-level survey operation (voices `axvr_survey`)
- `rbkro_audit` — Kit-level audit operation (voices `axvr_audit`)

### Add cardinality to all regime declarations
Update every `// ⟦axvr_regime ...⟧` annotation to include cardinality:
- RBRR: `axrd_singleton` (one repo config)
- RBRN: `axrd_manifold` (3 nameplates)
- RBRV: `axrd_manifold` (6 vessels)
- RBRP: `axrd_singleton` (one payor project)
- RBRE: `axrd_manifold` (per-ECR environment)
- RBRG: `axrd_manifold` (per-GitHub config)
- RBRS: `axrd_singleton` (one workstation)
- RBRO: assess and annotate

## BUSA changes

### Rename bukro_broach → bukro_kindle
- Mapping section: `:bukro_broach:` → `:bukro_kindle:`
- Anchor: `[[bukro_broach]]` → `[[bukro_kindle]]`
- Definition and all prose references

### Add cardinality to regime declarations
- BURC: `axrd_singleton`
- BURS: `axrd_singleton`

## Do NOT change
- AXLA-Lexicon.adoc (done in prior pace)
- Subdocuments (done in subsequent paces)
- Shell scripts

### rbrn-collection-voicings (₢ATAAk) [complete]

**[260215-0755] complete**

Add collection-level operation voicings to the RBRN subdocument, establishing the exemplar pattern for manifold regimes.

## File: lenses/RBRN-RegimeNameplate.adoc

## Rename existing operation marker
- `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧`

## Add collection-level operation voicings

After the existing kindle/validate/render operation voicings, add three new sections:

### axhro_list
```
// ⟦axhro_list⟧
{rbkro_list}

Enumerates available nameplate instances by scanning assignment files matching the RBRN naming pattern.
Returns moniker identifiers suitable for instance selection.
```

### axhro_survey
```
// ⟦axhro_survey⟧
{rbkro_survey}

Read-only cross-instance summary of all nameplates.
Iterates rbrn_list, kindles each instance in a subshell to avoid sentinel conflicts.
Displays network topology, entry configuration, and local image availability for all instances.
```

### axhro_audit
```
// ⟦axhro_audit⟧
{rbkro_audit}

Cross-instance validation of all nameplates plus external verification.
Performs survey display, then validates cross-instance constraints: port uniqueness, subnet non-overlap, IP uniqueness.
Includes GCB quota headroom check via Service Usage API.
```

## Pattern note
This establishes the reference exemplar. RBRV and future manifold regimes will follow this six-operation structure: kindle, validate, render (instance-level) + list, survey, audit (collection-level).

**[260215-0713] rough**

Add collection-level operation voicings to the RBRN subdocument, establishing the exemplar pattern for manifold regimes.

## File: lenses/RBRN-RegimeNameplate.adoc

## Rename existing operation marker
- `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧`

## Add collection-level operation voicings

After the existing kindle/validate/render operation voicings, add three new sections:

### axhro_list
```
// ⟦axhro_list⟧
{rbkro_list}

Enumerates available nameplate instances by scanning assignment files matching the RBRN naming pattern.
Returns moniker identifiers suitable for instance selection.
```

### axhro_survey
```
// ⟦axhro_survey⟧
{rbkro_survey}

Read-only cross-instance summary of all nameplates.
Iterates rbrn_list, kindles each instance in a subshell to avoid sentinel conflicts.
Displays network topology, entry configuration, and local image availability for all instances.
```

### axhro_audit
```
// ⟦axhro_audit⟧
{rbkro_audit}

Cross-instance validation of all nameplates plus external verification.
Performs survey display, then validates cross-instance constraints: port uniqueness, subnet non-overlap, IP uniqueness.
Includes GCB quota headroom check via Service Usage API.
```

## Pattern note
This establishes the reference exemplar. RBRV and future manifold regimes will follow this six-operation structure: kindle, validate, render (instance-level) + list, survey, audit (collection-level).

### rbrv-list-extraction-and-voicings (₢ATAAl) [complete]

**[260215-0802] complete**

Extract vessel listing into proper rbrv_list() function and add collection-level voicings to RBRV subdocument.

## Code: Tools/rbw/rbrv_regime.sh

### Extract rbrv_list()
Move the inline vessel enumeration from rbrv_cli.sh (lines 128-134) into a proper function in rbrv_regime.sh:

```bash
rbrv_list() {
  zrbrr_sentinel  # needs RBRR_VESSEL_DIR
  for z_d in "${RBRR_VESSEL_DIR}"/*/; do
    test -d "${z_d}" || continue
    test -f "${z_d}/rbrv.env" || continue
    local z_s="${z_d%/}"
    echo "${z_s##*/}"
  done
}
```

### Update rbrv_cli.sh
Replace inline listing with call to `rbrv_list`. The CLI still handles the "no sigil given → show list" dispatch, but delegates enumeration to the regime module.

## Spec: lenses/RBSRV-RegimeVessel.adoc

### Rename operation marker
- `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧`

### Add collection-level voicing
Add after existing kindle/validate/render:

```
// ⟦axhro_list⟧
{rbkro_list}

Enumerates available vessel instances by scanning subdirectories of RBRR_VESSEL_DIR for rbrv.env files.
Returns sigil identifiers suitable for instance selection.
```

Note: RBRV does not currently have survey or audit operations. Add only list for now. Survey/audit can be added when cross-vessel validation needs arise.

## Do NOT change
- AXLA or RBSA (done in prior paces)
- rbrv_validate or rbrv_render logic

**[260215-0713] rough**

Extract vessel listing into proper rbrv_list() function and add collection-level voicings to RBRV subdocument.

## Code: Tools/rbw/rbrv_regime.sh

### Extract rbrv_list()
Move the inline vessel enumeration from rbrv_cli.sh (lines 128-134) into a proper function in rbrv_regime.sh:

```bash
rbrv_list() {
  zrbrr_sentinel  # needs RBRR_VESSEL_DIR
  for z_d in "${RBRR_VESSEL_DIR}"/*/; do
    test -d "${z_d}" || continue
    test -f "${z_d}/rbrv.env" || continue
    local z_s="${z_d%/}"
    echo "${z_s##*/}"
  done
}
```

### Update rbrv_cli.sh
Replace inline listing with call to `rbrv_list`. The CLI still handles the "no sigil given → show list" dispatch, but delegates enumeration to the regime module.

## Spec: lenses/RBSRV-RegimeVessel.adoc

### Rename operation marker
- `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧`

### Add collection-level voicing
Add after existing kindle/validate/render:

```
// ⟦axhro_list⟧
{rbkro_list}

Enumerates available vessel instances by scanning subdirectories of RBRR_VESSEL_DIR for rbrv.env files.
Returns sigil identifiers suitable for instance selection.
```

Note: RBRV does not currently have survey or audit operations. Add only list for now. Survey/audit can be added when cross-vessel validation needs arise.

## Do NOT change
- AXLA or RBSA (done in prior paces)
- rbrv_validate or rbrv_render logic

### rename-rbrr-regime-to-env (₢ATAAd) [complete]

**[260215-0804] complete**

Rename rbrr_RecipeBottleRegimeRepo.sh to rbrr.env for consistency with regime variable file conventions.

## Context

All other regime variable files use the .env pattern:
- rbrv.env (vessel), rbrp.env (payor), rbrn_*.env (nameplate), burc.env (BUK config), cccr.env (CCCK)

The repo regime is the sole outlier: rbrr_RecipeBottleRegimeRepo.sh

## Work required

1. Rename rbrr_RecipeBottleRegimeRepo.sh to rbrr.env
2. Update all references across the codebase (source/include statements, CLAUDE.md mappings, etc.)
3. Verify regime validation still works after rename

## Acceptance criteria

- File renamed to rbrr.env
- All references updated — no broken sources
- Regime validation passes

**[260213-1008] rough**

Rename rbrr_RecipeBottleRegimeRepo.sh to rbrr.env for consistency with regime variable file conventions.

## Context

All other regime variable files use the .env pattern:
- rbrv.env (vessel), rbrp.env (payor), rbrn_*.env (nameplate), burc.env (BUK config), cccr.env (CCCK)

The repo regime is the sole outlier: rbrr_RecipeBottleRegimeRepo.sh

## Work required

1. Rename rbrr_RecipeBottleRegimeRepo.sh to rbrr.env
2. Update all references across the codebase (source/include statements, CLAUDE.md mappings, etc.)
3. Verify regime validation still works after rename

## Acceptance criteria

- File renamed to rbrr.env
- All references updated — no broken sources
- Regime validation passes

### align-rbrr-with-rbrn-rbrv-pattern (₢ATAAh) [complete]

**[260215-0811] complete**

Bring RBRR spec documentation and code into structural parity with RBRN and RBRV.

## Delete legacy document
- Delete `lenses/RBRR-RegimeRepo.adoc` — old CRR-style format, only 2 of 26 variables, references non-existent RBRR_NAMEPLATE_PATH, no new information not already in RBSRR or RBSA

## Update RBSRR-RegimeRepo.adoc
- Remove dead variables `rbrr_registry_owner` and `rbrr_registry_name` (deleted from validator and assignment file in earlier paces)
- Verify all current validator variables are documented
- Rename `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧` (aligning with AXLA rename done in prior pace)
- Confirm `axhro_kindle`, `axhro_validate`, `axhro_render` operation voicings are present and accurate

## Wire RBSRR into RBSA via include::
- Add `include::RBSRR-RegimeRepo.adoc[]` in RBSA's RBRR section (matching RBRN line 2229 and RBRV line 2049 pattern)
- Remove inline RBRR variable definitions from RBSA body (they move into the subdocument)
- Keep RBRR regime header and introductory text in RBSA

## Normalize RBRR regime annotation in RBSA
- Change `// ⟦axl_voices axrg_regime axf_bash axrd_file_sourced axrd_singleton⟧` to `// ⟦axvr_regime axf_bash axrd_file_sourced axrd_singleton⟧` (modern axvr_ style, matching RBRN/RBRV)

## Expand RBSA mapping section
- Add attribute references for all RBRR variables (currently only 7 have RBSA anchors)
- Add missing variable anchors: container runtime vars, build configuration vars, GCB image ref vars
- Follow the `:rbrr_variable_name: <<rbrr_variable_name,Display Text>>` pattern used by RBRN and RBRV

## Rename zrbrr_broach → zrbrr_kindle
- In Tools/rbw/rbrr_regime.sh: rename function `zrbrr_broach()` → `zrbrr_kindle()`
- Update the internal ZRBRR_BROACHED guard variable → ZRBRR_KINDLED
- Update all callers of zrbrr_broach (grep for references)
- This aligns RBRR with every other regime's naming convention

**[260215-0755] rough**

Bring RBRR spec documentation and code into structural parity with RBRN and RBRV.

## Delete legacy document
- Delete `lenses/RBRR-RegimeRepo.adoc` — old CRR-style format, only 2 of 26 variables, references non-existent RBRR_NAMEPLATE_PATH, no new information not already in RBSRR or RBSA

## Update RBSRR-RegimeRepo.adoc
- Remove dead variables `rbrr_registry_owner` and `rbrr_registry_name` (deleted from validator and assignment file in earlier paces)
- Verify all current validator variables are documented
- Rename `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧` (aligning with AXLA rename done in prior pace)
- Confirm `axhro_kindle`, `axhro_validate`, `axhro_render` operation voicings are present and accurate

## Wire RBSRR into RBSA via include::
- Add `include::RBSRR-RegimeRepo.adoc[]` in RBSA's RBRR section (matching RBRN line 2229 and RBRV line 2049 pattern)
- Remove inline RBRR variable definitions from RBSA body (they move into the subdocument)
- Keep RBRR regime header and introductory text in RBSA

## Normalize RBRR regime annotation in RBSA
- Change `// ⟦axl_voices axrg_regime axf_bash axrd_file_sourced axrd_singleton⟧` to `// ⟦axvr_regime axf_bash axrd_file_sourced axrd_singleton⟧` (modern axvr_ style, matching RBRN/RBRV)

## Expand RBSA mapping section
- Add attribute references for all RBRR variables (currently only 7 have RBSA anchors)
- Add missing variable anchors: container runtime vars, build configuration vars, GCB image ref vars
- Follow the `:rbrr_variable_name: <<rbrr_variable_name,Display Text>>` pattern used by RBRN and RBRV

## Rename zrbrr_broach → zrbrr_kindle
- In Tools/rbw/rbrr_regime.sh: rename function `zrbrr_broach()` → `zrbrr_kindle()`
- Update the internal ZRBRR_BROACHED guard variable → ZRBRR_KINDLED
- Update all callers of zrbrr_broach (grep for references)
- This aligns RBRR with every other regime's naming convention

**[260215-0716] rough**

Bring RBRR spec documentation and code into structural parity with RBRN and RBRV.

## Delete legacy document
- Delete `lenses/RBRR-RegimeRepo.adoc` — old CRR-style format, only 2 of 26 variables, references non-existent RBRR_NAMEPLATE_PATH, no new information not already in RBSRR or RBSA

## Update RBSRR-RegimeRepo.adoc
- Remove dead variables `rbrr_registry_owner` and `rbrr_registry_name` (deleted from validator and assignment file in earlier paces)
- Verify all current validator variables are documented
- Rename `// ⟦axhro_broach⟧` → `// ⟦axhro_kindle⟧` (aligning with AXLA rename done in prior pace)
- Confirm `axhro_kindle`, `axhro_validate`, `axhro_render` operation voicings are present and accurate

## Wire RBSRR into RBSA via include::
- Add `include::RBSRR-RegimeRepo.adoc[]` in RBSA's RBRR section (matching RBRN line 2229 and RBRV line 2049 pattern)
- Remove inline RBRR variable definitions from RBSA body (they move into the subdocument)
- Keep RBRR regime header and introductory text in RBSA

## Expand RBSA mapping section
- Add attribute references for all RBRR variables (currently only 7 have RBSA anchors)
- Add missing variable anchors: container runtime vars, build configuration vars, GCB image ref vars
- Follow the `:rbrr_variable_name: <<rbrr_variable_name,Display Text>>` pattern used by RBRN and RBRV

## Rename zrbrr_broach → zrbrr_kindle
- In Tools/rbw/rbrr_regime.sh: rename function `zrbrr_broach()` → `zrbrr_kindle()`
- Update the internal ZRBRR_BROACHED guard variable → ZRBRR_KINDLED
- Update all callers of zrbrr_broach (grep for references)
- This aligns RBRR with every other regime's naming convention

**[260214-0724] rough**

Bring RBRR spec documentation into structural parity with RBRN and RBRV.

## Delete legacy document
- Delete `lenses/RBRR-RegimeRepo.adoc` — old CRR-style format, only 2 of 26 variables, references non-existent `RBRR_NAMEPLATE_PATH`, no new information not already in RBSRR or RBSA

## Update RBSRR-RegimeRepo.adoc
- Remove dead variables `rbrr_registry_owner` and `rbrr_registry_name` (deleted from validator and assignment file in earlier paces)
- Verify all 26 current validator variables are documented
- Confirm `axhro_broach`, `axhro_validate`, `axhro_render` operation voicings are present and accurate

## Wire RBSRR into RBSA via include::
- Add `include::RBSRR-RegimeRepo.adoc[]` in RBSA's RBRR section (matching RBRN line 2229 and RBRV line 2049 pattern)
- Remove inline RBRR variable definitions from RBSA body (they move into the subdocument)
- Keep RBRR regime header and introductory text in RBSA

## Expand RBSA mapping section
- Add attribute references for all 26 RBRR variables (currently only 7 of 26 have RBSA anchors)
- Add missing variable anchors: container runtime vars, build configuration vars, GCB image ref vars
- Follow the `:rbrr_variable_name: <<rbrr_variable_name,Display Text>>` pattern used by RBRN and RBRV

## Do NOT change
- `rbrr_regime.sh` validator (broach vs kindle naming divergence is a separate concern)
- `rbrr_cli.sh` 
- Any shell scripts

### consolidate-rbrp-retire-rbl (₢ATAAK) [complete]

**[260215-0821] complete**

Consolidate RBRP regime loading to match RBRR/RBRN/RBRV pattern, then retire rbl_Locator.sh.

## Context

rbl_Locator.sh is the last legacy path-resolution module. RBL_RBRR_FILE consumers were migrated to rbrr_load in ₢APAAg/₢APAAh. RBL_RBRP_FILE has two remaining consumers (rbgm_cli.sh, rbgp_cli.sh). Five other files source rbl_Locator.sh as dead imports.

Plan of record: RBRP should function like RBRV and RBRN — proper load function, RBCC constant, kindle/validate pattern.

## 1. Add RBCC_RBRP_FILE constant

In rbcc_Constants.sh zrbcc_kindle(), add:
```
RBCC_RBRP_FILE="rbrp.env"
```

RBRP is singleton (one file at project root), same as RBRR.

## 2. Create rbrp_load in rbrp_regime.sh

Following rbrr_load pattern:
```
rbrp_load() {
  local z_rbrp_file="${RBCC_RBRP_FILE}"
  test -f "${z_rbrp_file}" || buc_die "RBRP config not found: ${z_rbrp_file}"
  source "${z_rbrp_file}" || buc_die "Failed to source RBRP config: ${z_rbrp_file}"
  zrbrp_kindle
}
```

Note: zrbrp_kindle depends on RBGC being kindled (uses RBGC_GLOBAL_PAYOR_REGEX). Callers must kindle RBGC before calling rbrp_load.

## 3. Switch rbgm_cli.sh and rbgp_cli.sh to rbrp_load

Replace the RBL-based RBRP loading block in each:
```
zrbl_kindle
buv_file_exists "${RBL_RBRP_FILE}"
source          "${RBL_RBRP_FILE}" || buc_die "Failed to source RBRP regime file"
```
with:
```
rbrp_load
```

Ensure zrbgc_kindle runs before rbrp_load (already the case — zrbcc_kindle and rbrr_load precede this block, and zrbgc_kindle follows). Check kindle ordering: zrbcc_kindle → rbrr_load → zrbgc_kindle → rbrp_load → zrbrp_kindle.

Wait: current ordering in rbgm has zrbgc_kindle AFTER the RBRP block. Need to move zrbgc_kindle before rbrp_load. Verify and fix kindle ordering.

## 4. Remove all rbl_Locator.sh sourcing

Remove `source ...rbl_Locator.sh` from these 7 files:
- rbgm_cli.sh (was active consumer, now migrated)
- rbgp_cli.sh (was active consumer, now migrated)
- rbgg_cli.sh (dead import)
- rbf_cli.sh (dead import)
- rbga_cli.sh (dead import)
- rbgb_cli.sh (dead import)
- rgbs_cli.sh (dead import)

## 5. Rehome tool checks

zrbl_kindle validates: openssl, curl, base64, jq. These are needed by OAuth/REST callers. Move these checks to a sentinel in the modules that actually need them (rbgo_OAuth.sh or rbgu_Utility.sh), or into the CLI furnish functions that use those tools. Assess which modules actually require each tool.

## 6. Delete rbl_Locator.sh

After all consumers removed and tool checks rehomed.

## Verification

- grep confirms zero references to RBL_, zrbl_, rbl_Locator in non-ABANDONED code
- rbgm_cli.sh and rbgp_cli.sh furnish kindle ordering is correct
- rbrp_load works (sources rbrp.env, kindles, validates)

**[260210-1052] rough**

Consolidate RBRP regime loading to match RBRR/RBRN/RBRV pattern, then retire rbl_Locator.sh.

## Context

rbl_Locator.sh is the last legacy path-resolution module. RBL_RBRR_FILE consumers were migrated to rbrr_load in ₢APAAg/₢APAAh. RBL_RBRP_FILE has two remaining consumers (rbgm_cli.sh, rbgp_cli.sh). Five other files source rbl_Locator.sh as dead imports.

Plan of record: RBRP should function like RBRV and RBRN — proper load function, RBCC constant, kindle/validate pattern.

## 1. Add RBCC_RBRP_FILE constant

In rbcc_Constants.sh zrbcc_kindle(), add:
```
RBCC_RBRP_FILE="rbrp.env"
```

RBRP is singleton (one file at project root), same as RBRR.

## 2. Create rbrp_load in rbrp_regime.sh

Following rbrr_load pattern:
```
rbrp_load() {
  local z_rbrp_file="${RBCC_RBRP_FILE}"
  test -f "${z_rbrp_file}" || buc_die "RBRP config not found: ${z_rbrp_file}"
  source "${z_rbrp_file}" || buc_die "Failed to source RBRP config: ${z_rbrp_file}"
  zrbrp_kindle
}
```

Note: zrbrp_kindle depends on RBGC being kindled (uses RBGC_GLOBAL_PAYOR_REGEX). Callers must kindle RBGC before calling rbrp_load.

## 3. Switch rbgm_cli.sh and rbgp_cli.sh to rbrp_load

Replace the RBL-based RBRP loading block in each:
```
zrbl_kindle
buv_file_exists "${RBL_RBRP_FILE}"
source          "${RBL_RBRP_FILE}" || buc_die "Failed to source RBRP regime file"
```
with:
```
rbrp_load
```

Ensure zrbgc_kindle runs before rbrp_load (already the case — zrbcc_kindle and rbrr_load precede this block, and zrbgc_kindle follows). Check kindle ordering: zrbcc_kindle → rbrr_load → zrbgc_kindle → rbrp_load → zrbrp_kindle.

Wait: current ordering in rbgm has zrbgc_kindle AFTER the RBRP block. Need to move zrbgc_kindle before rbrp_load. Verify and fix kindle ordering.

## 4. Remove all rbl_Locator.sh sourcing

Remove `source ...rbl_Locator.sh` from these 7 files:
- rbgm_cli.sh (was active consumer, now migrated)
- rbgp_cli.sh (was active consumer, now migrated)
- rbgg_cli.sh (dead import)
- rbf_cli.sh (dead import)
- rbga_cli.sh (dead import)
- rbgb_cli.sh (dead import)
- rgbs_cli.sh (dead import)

## 5. Rehome tool checks

zrbl_kindle validates: openssl, curl, base64, jq. These are needed by OAuth/REST callers. Move these checks to a sentinel in the modules that actually need them (rbgo_OAuth.sh or rbgu_Utility.sh), or into the CLI furnish functions that use those tools. Assess which modules actually require each tool.

## 6. Delete rbl_Locator.sh

After all consumers removed and tool checks rehomed.

## Verification

- grep confirms zero references to RBL_, zrbl_, rbl_Locator in non-ABANDONED code
- rbgm_cli.sh and rbgp_cli.sh furnish kindle ordering is correct
- rbrp_load works (sources rbrp.env, kindles, validates)

### rbrp-regime-spec (₢ATAAm) [complete]

**[260215-0825] complete**

Create RBRP (Regime Payor) CLI and subdocument, wiring into RBSA following the established RBRN/RBRV/RBRR pattern.

## Create: Tools/rbw/rbrp_cli.sh

Following rbrn_cli.sh / rbrv_cli.sh pattern:
- Source dependencies: buc_command.sh, buv_validation.sh, rbrp_regime.sh, rbcc_Constants.sh, rbcr_render.sh
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, then render using rbcr_section_begin/item/end pattern
  - Section: Payor Project Identity (RBRP_PAYOR_PROJECT_ID)
  - Section: Billing Configuration (RBRP_BILLING_ACCOUNT_ID, gated optional)
  - Section: OAuth Configuration (RBRP_OAUTH_CLIENT_ID, gated optional)
  - Unexpected variable display in red
  - Validate after full render
- RBRP is singleton — no list/survey/audit commands needed

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern. These are the exemplars.

## Create: lenses/RBSRP-RegimePayor.adoc

Follow the subdocument pattern established by RBRN-RegimeNameplate.adoc:
- `// ⟦axhrb_regime⟧` header with {rbrp_regime} reference
- Variable voicings for all 3 RBRP variables:
  - RBRP_PAYOR_PROJECT_ID (required, regex-validated)
  - RBRP_BILLING_ACCOUNT_ID (optional during initial setup)
  - RBRP_OAUTH_CLIENT_ID (optional during initial setup)
- Operation voicings: `axhro_kindle`, `axhro_validate`, `axhro_render`
- Document RBGC dependency (zrbgc_sentinel required before kindle)

## Update: lenses/RBS0-SpecTop.adoc

### Add mapping section entries
Add attribute references for RBRP variables:
- `:rbrp_regime:`, `:rbrp_prefix:`
- `:rbrp_payor_project_id:`, `:rbrp_billing_account_id:`, `:rbrp_oauth_client_id:`

### Wire subdocument
Add `include::RBSRP-RegimePayor.adoc[]` in the RBRP section of RBSA.
Remove any inline RBRP definitions if present.

### Normalize RBRP regime annotation
- Change `// ⟦axl_voices axrg_regime axf_bash axrd_singleton⟧` to `// ⟦axvr_regime axf_bash axrd_file_sourced axrd_singleton⟧` (modern axvr_ style + add missing axrd_file_sourced provenance)

### Cardinality
Already annotated `axrd_singleton` in prior vocabulary pace.

## Reference
- Validator: Tools/rbw/rbrp_regime.sh (zrbrp_kindle)
- Assignment: rbrp.env at project root
- Dependencies: RBGC (Constants) must kindle first
- Render pattern: rbrn_cli.sh, rbrv_cli.sh (rbcr_section_begin/item/end)

**[260215-0755] rough**

Create RBRP (Regime Payor) CLI and subdocument, wiring into RBSA following the established RBRN/RBRV/RBRR pattern.

## Create: Tools/rbw/rbrp_cli.sh

Following rbrn_cli.sh / rbrv_cli.sh pattern:
- Source dependencies: buc_command.sh, buv_validation.sh, rbrp_regime.sh, rbcc_Constants.sh, rbcr_render.sh
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, then render using rbcr_section_begin/item/end pattern
  - Section: Payor Project Identity (RBRP_PAYOR_PROJECT_ID)
  - Section: Billing Configuration (RBRP_BILLING_ACCOUNT_ID, gated optional)
  - Section: OAuth Configuration (RBRP_OAUTH_CLIENT_ID, gated optional)
  - Unexpected variable display in red
  - Validate after full render
- RBRP is singleton — no list/survey/audit commands needed

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern. These are the exemplars.

## Create: lenses/RBSRP-RegimePayor.adoc

Follow the subdocument pattern established by RBRN-RegimeNameplate.adoc:
- `// ⟦axhrb_regime⟧` header with {rbrp_regime} reference
- Variable voicings for all 3 RBRP variables:
  - RBRP_PAYOR_PROJECT_ID (required, regex-validated)
  - RBRP_BILLING_ACCOUNT_ID (optional during initial setup)
  - RBRP_OAUTH_CLIENT_ID (optional during initial setup)
- Operation voicings: `axhro_kindle`, `axhro_validate`, `axhro_render`
- Document RBGC dependency (zrbgc_sentinel required before kindle)

## Update: lenses/RBS0-SpecTop.adoc

### Add mapping section entries
Add attribute references for RBRP variables:
- `:rbrp_regime:`, `:rbrp_prefix:`
- `:rbrp_payor_project_id:`, `:rbrp_billing_account_id:`, `:rbrp_oauth_client_id:`

### Wire subdocument
Add `include::RBSRP-RegimePayor.adoc[]` in the RBRP section of RBSA.
Remove any inline RBRP definitions if present.

### Normalize RBRP regime annotation
- Change `// ⟦axl_voices axrg_regime axf_bash axrd_singleton⟧` to `// ⟦axvr_regime axf_bash axrd_file_sourced axrd_singleton⟧` (modern axvr_ style + add missing axrd_file_sourced provenance)

### Cardinality
Already annotated `axrd_singleton` in prior vocabulary pace.

## Reference
- Validator: Tools/rbw/rbrp_regime.sh (zrbrp_kindle)
- Assignment: rbrp.env at project root
- Dependencies: RBGC (Constants) must kindle first
- Render pattern: rbrn_cli.sh, rbrv_cli.sh (rbcr_section_begin/item/end)

**[260215-0720] rough**

Create RBRP (Regime Payor) CLI and subdocument, wiring into RBSA following the established RBRN/RBRV/RBRR pattern.

## Create: Tools/rbw/rbrp_cli.sh

Following rbrn_cli.sh / rbrv_cli.sh pattern:
- Source dependencies: buc_command.sh, buv_validation.sh, rbrp_regime.sh, rbcc_Constants.sh, rbcr_render.sh
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, then render using rbcr_section_begin/item/end pattern
  - Section: Payor Project Identity (RBRP_PAYOR_PROJECT_ID)
  - Section: Billing Configuration (RBRP_BILLING_ACCOUNT_ID, gated optional)
  - Section: OAuth Configuration (RBRP_OAUTH_CLIENT_ID, gated optional)
  - Unexpected variable display in red
  - Validate after full render
- RBRP is singleton — no list/survey/audit commands needed

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern. These are the exemplars.

## Create: lenses/RBSRP-RegimePayor.adoc

Follow the subdocument pattern established by RBRN-RegimeNameplate.adoc:
- `// ⟦axhrb_regime⟧` header with {rbrp_regime} reference
- Variable voicings for all 3 RBRP variables:
  - RBRP_PAYOR_PROJECT_ID (required, regex-validated)
  - RBRP_BILLING_ACCOUNT_ID (optional during initial setup)
  - RBRP_OAUTH_CLIENT_ID (optional during initial setup)
- Operation voicings: `axhro_kindle`, `axhro_validate`, `axhro_render`
- Document RBGC dependency (zrbgc_sentinel required before kindle)

## Update: lenses/RBS0-SpecTop.adoc

### Add mapping section entries
Add attribute references for RBRP variables:
- `:rbrp_regime:`, `:rbrp_prefix:`
- `:rbrp_payor_project_id:`, `:rbrp_billing_account_id:`, `:rbrp_oauth_client_id:`

### Wire subdocument
Add `include::RBSRP-RegimePayor.adoc[]` in the RBRP section of RBSA.
Remove any inline RBRP definitions if present.

### Cardinality
Already annotated `axrd_singleton` in prior vocabulary pace.

## Reference
- Validator: Tools/rbw/rbrp_regime.sh (zrbrp_kindle)
- Assignment: rbrp.env at project root
- Dependencies: RBGC (Constants) must kindle first
- Render pattern: rbrn_cli.sh, rbrv_cli.sh (rbcr_section_begin/item/end)

**[260215-0714] rough**

Create RBRP (Regime Payor) subdocument and wire into RBSA, following the established RBRN/RBRV/RBRR pattern.

## Create: lenses/RBSRP-RegimePayor.adoc

Follow the subdocument pattern established by RBRN-RegimeNameplate.adoc:
- `// ⟦axhrb_regime⟧` header with {rbrp_regime} reference
- Variable voicings for all 3 RBRP variables:
  - RBRP_PAYOR_PROJECT_ID (required, regex-validated)
  - RBRP_BILLING_ACCOUNT_ID (optional during initial setup)
  - RBRP_OAUTH_CLIENT_ID (optional during initial setup)
- Operation voicings: `axhro_kindle`, `axhro_validate`, `axhro_render`
- Document RBGC dependency (zrbgc_sentinel required before kindle)

## Update: lenses/RBS0-SpecTop.adoc

### Add mapping section entries
Add attribute references for RBRP variables:
- `:rbrp_regime:`, `:rbrp_prefix:`
- `:rbrp_payor_project_id:`, `:rbrp_billing_account_id:`, `:rbrp_oauth_client_id:`

### Wire subdocument
Add `include::RBSRP-RegimePayor.adoc[]` in the RBRP section of RBSA.
Remove any inline RBRP definitions if present.

### Cardinality
Already annotated `axrd_singleton` in prior vocabulary pace.

## Reference
- Validator: Tools/rbw/rbrp_regime.sh (zrbrp_kindle)
- Assignment: rbrp.env at project root
- Dependencies: RBGC (Constants) must kindle first

### rbra-regime-formalization (₢ATAAn) [complete]

**[260215-0832] complete**

Formalize RBRA (Credential Format) with a validator and subdocument. RBRA is multi-instance (3 credential files) and currently has no validator or spec despite being security-sensitive.

## Create: Tools/rbw/rbra_regime.sh

Following the regime.sh pattern:
- Multiple inclusion guard (ZRBRA_SOURCED)
- `zrbra_kindle()` with ZRBRA_KINDLED guard
- Validate 4 variables:
  - RBRA_CLIENT_EMAIL (string, service account email format)
  - RBRA_PRIVATE_KEY (string, PEM format presence check)
  - RBRA_PROJECT_ID (gname, GCP project ID)
  - RBRA_TOKEN_LIFETIME_SEC (integer, typically 1800)
- Unexpected variable detection via compgen
- ZRBRA_ROLLUP construction
- `zrbra_sentinel()` function

## Create: Tools/rbw/rbra_cli.sh

Following rbrn_cli.sh pattern:
- validate command (per-instance)
- render command (per-instance, using rbcr_* shared module)
- list command (enumerate RBRA files referenced by RBRR)

Instance discovery: RBRA files are referenced by RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE. The list function enumerates these three paths.

## Create: lenses/RBSRA-CredentialFormat.adoc

Subdocument following RBRN pattern:
- `// ⟦axhrb_regime⟧` header
- Variable voicings for all 4 variables
- Operation voicings: kindle, validate, render, list
- Note: RBRA files are NOT in git — external credential files

## Update: lenses/RBS0-SpecTop.adoc

- Add mapping section entries for RBRA variables
- Wire `include::RBSRA-CredentialFormat.adoc[]`
- Cardinality: `axrd_manifold` (3 instances: governor, retriever, director)

## Security note
RBRA credential files contain RSA private keys. Validator should verify format, NOT log values. Render should mask RBRA_PRIVATE_KEY display.

**[260215-0714] rough**

Formalize RBRA (Credential Format) with a validator and subdocument. RBRA is multi-instance (3 credential files) and currently has no validator or spec despite being security-sensitive.

## Create: Tools/rbw/rbra_regime.sh

Following the regime.sh pattern:
- Multiple inclusion guard (ZRBRA_SOURCED)
- `zrbra_kindle()` with ZRBRA_KINDLED guard
- Validate 4 variables:
  - RBRA_CLIENT_EMAIL (string, service account email format)
  - RBRA_PRIVATE_KEY (string, PEM format presence check)
  - RBRA_PROJECT_ID (gname, GCP project ID)
  - RBRA_TOKEN_LIFETIME_SEC (integer, typically 1800)
- Unexpected variable detection via compgen
- ZRBRA_ROLLUP construction
- `zrbra_sentinel()` function

## Create: Tools/rbw/rbra_cli.sh

Following rbrn_cli.sh pattern:
- validate command (per-instance)
- render command (per-instance, using rbcr_* shared module)
- list command (enumerate RBRA files referenced by RBRR)

Instance discovery: RBRA files are referenced by RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE. The list function enumerates these three paths.

## Create: lenses/RBSRA-CredentialFormat.adoc

Subdocument following RBRN pattern:
- `// ⟦axhrb_regime⟧` header
- Variable voicings for all 4 variables
- Operation voicings: kindle, validate, render, list
- Note: RBRA files are NOT in git — external credential files

## Update: lenses/RBS0-SpecTop.adoc

- Add mapping section entries for RBRA variables
- Wire `include::RBSRA-CredentialFormat.adoc[]`
- Cardinality: `axrd_manifold` (3 instances: governor, retriever, director)

## Security note
RBRA credential files contain RSA private keys. Validator should verify format, NOT log values. Render should mask RBRA_PRIVATE_KEY display.

### external-regime-specs (₢ATAAo) [complete]

**[260215-0840] complete**

Create CLIs and subdocuments for the three external (not-in-git) regimes: RBRE, RBRG, RBRS. These are structurally similar — small, external assignment files, already have validators but NO CLI scripts.

Also normalize RBRO annotation in RBSA (found during review checkpoint 1).

## For each regime, create TWO new files:

### CLI script (Tools/rbw/rbr{x}_cli.sh)

Following rbrn_cli.sh / rbrv_cli.sh pattern:
- Source dependencies: buc_command.sh, buv_validation.sh, rbr{x}_regime.sh, rbcc_Constants.sh, rbcr_render.sh
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, render using rbcr_section_begin/item/end
- Unexpected variable display in red, validate after full render

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern. These are the exemplars.

### Subdocument (lenses/RBS*.adoc)

Following RBRN-RegimeNameplate.adoc subdocument pattern with axhr* markers.

---

## RBRE (Regime ECR) — 6 variables

### Create: Tools/rbw/rbre_cli.sh
- Sections: AWS Identity (credentials env, access key, secret key, account ID), Region & Repository (region, repository name)
- Security: mask RBRE_AWS_SECRET_ACCESS_KEY in render display
- Cardinality: assess — currently one ECR config, potentially manifold per-environment. If manifold, add list command.

### Create: lenses/RBSRE-RegimeEcr.adoc
- Variables: RBRE_AWS_CREDENTIALS_ENV, RBRE_AWS_ACCESS_KEY_ID, RBRE_AWS_SECRET_ACCESS_KEY, RBRE_AWS_ACCOUNT_ID, RBRE_AWS_REGION, RBRE_REPOSITORY_NAME
- Operations: kindle, validate, render (+ list if manifold)

### Update RBSA
- Add mapping section entries for all RBRE variables
- Wire include::RBSRE-RegimeEcr.adoc[]

## RBRG (Regime GitHub) — 2 variables

### Create: Tools/rbw/rbrg_cli.sh
- Sections: GitHub Identity (username, PAT)
- Security: mask RBRG_PAT in render display
- Cardinality: assess — currently one GitHub config, potentially manifold per-account. If manifold, add list command.

### Create: lenses/RBSRG-RegimeGithub.adoc
- Variables: RBRG_PAT, RBRG_USERNAME
- Operations: kindle, validate, render (+ list if manifold)

### Update RBSA
- Add mapping section entries for RBRG variables
- Wire include::RBSRG-RegimeGithub.adoc[]

## RBRS (Regime Station) — 3 variables

### Create: Tools/rbw/rbrs_cli.sh
- Sections: Podman Configuration (root dir, vmimage cache dir, platform)
- Cardinality: axrd_singleton (one workstation) — no list/survey/audit

### Create: lenses/RBSRS-RegimeStation.adoc
- Variables: RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
- Operations: kindle, validate, render

### Update RBSA
- Add mapping section entries for RBRS variables
- Wire include::RBSRS-RegimeStation.adoc[]

## Normalize RBRO regime annotation in RBSA
- Change `// ⟦axl_voices axrg_regime axf_bash axrd_singleton⟧` to `// ⟦axvr_regime axf_bash axrd_file_sourced axrd_singleton⟧` (modern axvr_ style + add missing axrd_file_sourced provenance)
- This is a one-line fix found during review checkpoint 1

**[260215-0755] rough**

Create CLIs and subdocuments for the three external (not-in-git) regimes: RBRE, RBRG, RBRS. These are structurally similar — small, external assignment files, already have validators but NO CLI scripts.

Also normalize RBRO annotation in RBSA (found during review checkpoint 1).

## For each regime, create TWO new files:

### CLI script (Tools/rbw/rbr{x}_cli.sh)

Following rbrn_cli.sh / rbrv_cli.sh pattern:
- Source dependencies: buc_command.sh, buv_validation.sh, rbr{x}_regime.sh, rbcc_Constants.sh, rbcr_render.sh
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, render using rbcr_section_begin/item/end
- Unexpected variable display in red, validate after full render

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern. These are the exemplars.

### Subdocument (lenses/RBS*.adoc)

Following RBRN-RegimeNameplate.adoc subdocument pattern with axhr* markers.

---

## RBRE (Regime ECR) — 6 variables

### Create: Tools/rbw/rbre_cli.sh
- Sections: AWS Identity (credentials env, access key, secret key, account ID), Region & Repository (region, repository name)
- Security: mask RBRE_AWS_SECRET_ACCESS_KEY in render display
- Cardinality: assess — currently one ECR config, potentially manifold per-environment. If manifold, add list command.

### Create: lenses/RBSRE-RegimeEcr.adoc
- Variables: RBRE_AWS_CREDENTIALS_ENV, RBRE_AWS_ACCESS_KEY_ID, RBRE_AWS_SECRET_ACCESS_KEY, RBRE_AWS_ACCOUNT_ID, RBRE_AWS_REGION, RBRE_REPOSITORY_NAME
- Operations: kindle, validate, render (+ list if manifold)

### Update RBSA
- Add mapping section entries for all RBRE variables
- Wire include::RBSRE-RegimeEcr.adoc[]

## RBRG (Regime GitHub) — 2 variables

### Create: Tools/rbw/rbrg_cli.sh
- Sections: GitHub Identity (username, PAT)
- Security: mask RBRG_PAT in render display
- Cardinality: assess — currently one GitHub config, potentially manifold per-account. If manifold, add list command.

### Create: lenses/RBSRG-RegimeGithub.adoc
- Variables: RBRG_PAT, RBRG_USERNAME
- Operations: kindle, validate, render (+ list if manifold)

### Update RBSA
- Add mapping section entries for RBRG variables
- Wire include::RBSRG-RegimeGithub.adoc[]

## RBRS (Regime Station) — 3 variables

### Create: Tools/rbw/rbrs_cli.sh
- Sections: Podman Configuration (root dir, vmimage cache dir, platform)
- Cardinality: axrd_singleton (one workstation) — no list/survey/audit

### Create: lenses/RBSRS-RegimeStation.adoc
- Variables: RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
- Operations: kindle, validate, render

### Update RBSA
- Add mapping section entries for RBRS variables
- Wire include::RBSRS-RegimeStation.adoc[]

## Normalize RBRO regime annotation in RBSA
- Change `// ⟦axl_voices axrg_regime axf_bash axrd_singleton⟧` to `// ⟦axvr_regime axf_bash axrd_file_sourced axrd_singleton⟧` (modern axvr_ style + add missing axrd_file_sourced provenance)
- This is a one-line fix found during review checkpoint 1

**[260215-0721] rough**

Create CLIs and subdocuments for the three external (not-in-git) regimes: RBRE, RBRG, RBRS. These are structurally similar — small, external assignment files, already have validators but NO CLI scripts.

## For each regime, create TWO new files:

### CLI script (Tools/rbw/rbr{x}_cli.sh)

Following rbrn_cli.sh / rbrv_cli.sh pattern:
- Source dependencies: buc_command.sh, buv_validation.sh, rbr{x}_regime.sh, rbcc_Constants.sh, rbcr_render.sh
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, render using rbcr_section_begin/item/end
- Unexpected variable display in red, validate after full render

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern. These are the exemplars.

### Subdocument (lenses/RBS*.adoc)

Following RBRN-RegimeNameplate.adoc subdocument pattern with axhr* markers.

---

## RBRE (Regime ECR) — 6 variables

### Create: Tools/rbw/rbre_cli.sh
- Sections: AWS Identity (credentials env, access key, secret key, account ID), Region & Repository (region, repository name)
- Security: mask RBRE_AWS_SECRET_ACCESS_KEY in render display
- Cardinality: assess — currently one ECR config, potentially manifold per-environment. If manifold, add list command.

### Create: lenses/RBSRE-RegimeEcr.adoc
- Variables: RBRE_AWS_CREDENTIALS_ENV, RBRE_AWS_ACCESS_KEY_ID, RBRE_AWS_SECRET_ACCESS_KEY, RBRE_AWS_ACCOUNT_ID, RBRE_AWS_REGION, RBRE_REPOSITORY_NAME
- Operations: kindle, validate, render (+ list if manifold)

### Update RBSA
- Add mapping section entries for all RBRE variables
- Wire include::RBSRE-RegimeEcr.adoc[]

## RBRG (Regime GitHub) — 2 variables

### Create: Tools/rbw/rbrg_cli.sh
- Sections: GitHub Identity (username, PAT)
- Security: mask RBRG_PAT in render display
- Cardinality: assess — currently one GitHub config, potentially manifold per-account. If manifold, add list command.

### Create: lenses/RBSRG-RegimeGithub.adoc
- Variables: RBRG_PAT, RBRG_USERNAME
- Operations: kindle, validate, render (+ list if manifold)

### Update RBSA
- Add mapping section entries for RBRG variables
- Wire include::RBSRG-RegimeGithub.adoc[]

## RBRS (Regime Station) — 3 variables

### Create: Tools/rbw/rbrs_cli.sh
- Sections: Podman Configuration (root dir, vmimage cache dir, platform)
- Cardinality: axrd_singleton (one workstation) — no list/survey/audit

### Create: lenses/RBSRS-RegimeStation.adoc
- Variables: RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
- Operations: kindle, validate, render

### Update RBSA
- Add mapping section entries for RBRS variables
- Wire include::RBSRS-RegimeStation.adoc[]

**[260215-0715] rough**

Create subdocuments for the three external (not-in-git) regimes: RBRE, RBRG, RBRS. These are structurally similar — singleton or small manifold, external assignment files, already have validators.

## RBRE (Regime ECR) — 6 variables

### Create: lenses/RBSRE-RegimeEcr.adoc
- Variables: RBRE_AWS_CREDENTIALS_ENV, RBRE_AWS_ACCESS_KEY_ID, RBRE_AWS_SECRET_ACCESS_KEY, RBRE_AWS_ACCOUNT_ID, RBRE_AWS_REGION, RBRE_REPOSITORY_NAME
- Operations: kindle, validate, render
- Cardinality: assess — currently one ECR config, but potentially manifold per-environment
- Security: mask secret key in render

### Update RBSA
- Add mapping section entries for all RBRE variables
- Wire include::RBSRE-RegimeEcr.adoc[]

## RBRG (Regime GitHub) — 2 variables

### Create: lenses/RBSRG-RegimeGithub.adoc
- Variables: RBRG_PAT, RBRG_USERNAME
- Operations: kindle, validate, render
- Cardinality: assess — currently one GitHub config, but potentially manifold per-account
- Security: mask PAT in render

### Update RBSA
- Add mapping section entries for RBRG variables
- Wire include::RBSRG-RegimeGithub.adoc[]

## RBRS (Regime Station) — 3 variables

### Create: lenses/RBSRS-RegimeStation.adoc
- Variables: RBRS_PODMAN_ROOT_DIR, RBRS_VMIMAGE_CACHE_DIR, RBRS_VM_PLATFORM
- Operations: kindle, validate, render
- Cardinality: axrd_singleton (one workstation)

### Update RBSA
- Add mapping section entries for RBRS variables
- Wire include::RBSRS-RegimeStation.adoc[]

## Pattern
All three follow the same subdocument template. No new CLI scripts needed — existing validators and CLI commands already work. This pace is spec-only.

### burd-dispatch-regime-formalization (₢ATAAr) [complete]

**[260215-0850] complete**

Formalize BURD (Dispatch Runtime) as a proper regime in BUSA with sentinel, unexpected variable detection, and spec subdocument. BURD is unique: ephemeral, constructed (not file-sourced), and assembled during dispatch rather than kindled from an assignment file.

## Context

BURD_ variables are set by bud_dispatch.sh during execution. No assignment file exists. Variables arrive via three phases:
- **Input**: set by caller before dispatch (BURD_VERBOSE, BURD_REGIME_FILE, BURD_NO_LOG, BURD_INTERACTIVE)
- **Computed**: derived during dispatch setup (BURD_NOW_STAMP, BURD_TEMP_DIR, BURD_OUTPUT_DIR, BURD_TRANSCRIPT, BURD_GIT_CONTEXT, BURD_LOG_LAST, BURD_LOG_SAME, BURD_LOG_HIST, BURD_COLOR)
- **Parsed**: extracted from tabtarget filename (BURD_COMMAND, BURD_TARGET, BURD_CLI_ARGS, BURD_TOKEN_1..5)

Currently has no validator, no sentinel, no unexpected variable detection, no spec doc.

## Code: Tools/buk/bud_dispatch.sh

### Add zburd_sentinel()
Downstream code can assert dispatch is properly initialized. Guard on a ZBURD_INITIALIZED variable set at the end of the dispatch setup block.

### Add unexpected variable detection
After all BURD_ variables are set, scan with `compgen -v BURD_` and flag any variables not in the known set. Follow the pattern used by zrbrn_kindle (ZRBRN_UNEXPECTED array).

### Do NOT refactor into a separate burd_regime.sh
BURD's initialization is inherently inline in bud_dispatch.sh — there is no file to source, no kindle in the traditional sense. The initialization block IS the kindle. Keep it in place; just add the sentinel and unexpected variable detection.

## Spec: create BUSD-DispatchRuntime.adoc (or similar)

Following subdocument pattern:
- `// ⟦axhrb_regime⟧` header with {burd_regime} reference
- Three groups for variable phases:
  - `// ⟦axhrgb_group⟧` {burd_group_input} — set by caller before dispatch
  - `// ⟦axhrgb_group⟧` {burd_group_computed} — derived during dispatch setup
  - `// ⟦axhrgb_group⟧` {burd_group_parsed} — extracted from tabtarget invocation
- Variable voicings for all ~17 BURD_ variables with appropriate dimensions
- Operation voicings: `axhro_kindle` (the inline initialization block), `axhro_validate` (unexpected variable detection)
- No render operation (ephemeral — nothing to render after the fact)

## Update: Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc

### Add mapping section entries
- `:burd_regime:`, `:burd_prefix:`
- Attribute references for all BURD_ variables
- Group references: `:burd_group_input:`, `:burd_group_computed:`, `:burd_group_parsed:`

### Wire subdocument
Add `include::` for the BURD subdocument.

### Cardinality and provenance
Annotate: `// ⟦axvr_regime axf_bash axrd_constructed axrd_singleton⟧`
- `axrd_constructed` — assembled programmatically, not sourced from file
- `axrd_singleton` — one per dispatch invocation

## No CLI, no tabtargets
BURD is transient (dies with process). No rbrX_cli.sh, no render, no list/survey/audit. The sentinel is the primary consumer-facing guarantee.

**[260215-0725] rough**

Formalize BURD (Dispatch Runtime) as a proper regime in BUSA with sentinel, unexpected variable detection, and spec subdocument. BURD is unique: ephemeral, constructed (not file-sourced), and assembled during dispatch rather than kindled from an assignment file.

## Context

BURD_ variables are set by bud_dispatch.sh during execution. No assignment file exists. Variables arrive via three phases:
- **Input**: set by caller before dispatch (BURD_VERBOSE, BURD_REGIME_FILE, BURD_NO_LOG, BURD_INTERACTIVE)
- **Computed**: derived during dispatch setup (BURD_NOW_STAMP, BURD_TEMP_DIR, BURD_OUTPUT_DIR, BURD_TRANSCRIPT, BURD_GIT_CONTEXT, BURD_LOG_LAST, BURD_LOG_SAME, BURD_LOG_HIST, BURD_COLOR)
- **Parsed**: extracted from tabtarget filename (BURD_COMMAND, BURD_TARGET, BURD_CLI_ARGS, BURD_TOKEN_1..5)

Currently has no validator, no sentinel, no unexpected variable detection, no spec doc.

## Code: Tools/buk/bud_dispatch.sh

### Add zburd_sentinel()
Downstream code can assert dispatch is properly initialized. Guard on a ZBURD_INITIALIZED variable set at the end of the dispatch setup block.

### Add unexpected variable detection
After all BURD_ variables are set, scan with `compgen -v BURD_` and flag any variables not in the known set. Follow the pattern used by zrbrn_kindle (ZRBRN_UNEXPECTED array).

### Do NOT refactor into a separate burd_regime.sh
BURD's initialization is inherently inline in bud_dispatch.sh — there is no file to source, no kindle in the traditional sense. The initialization block IS the kindle. Keep it in place; just add the sentinel and unexpected variable detection.

## Spec: create BUSD-DispatchRuntime.adoc (or similar)

Following subdocument pattern:
- `// ⟦axhrb_regime⟧` header with {burd_regime} reference
- Three groups for variable phases:
  - `// ⟦axhrgb_group⟧` {burd_group_input} — set by caller before dispatch
  - `// ⟦axhrgb_group⟧` {burd_group_computed} — derived during dispatch setup
  - `// ⟦axhrgb_group⟧` {burd_group_parsed} — extracted from tabtarget invocation
- Variable voicings for all ~17 BURD_ variables with appropriate dimensions
- Operation voicings: `axhro_kindle` (the inline initialization block), `axhro_validate` (unexpected variable detection)
- No render operation (ephemeral — nothing to render after the fact)

## Update: Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc

### Add mapping section entries
- `:burd_regime:`, `:burd_prefix:`
- Attribute references for all BURD_ variables
- Group references: `:burd_group_input:`, `:burd_group_computed:`, `:burd_group_parsed:`

### Wire subdocument
Add `include::` for the BURD subdocument.

### Cardinality and provenance
Annotate: `// ⟦axvr_regime axf_bash axrd_constructed axrd_singleton⟧`
- `axrd_constructed` — assembled programmatically, not sourced from file
- `axrd_singleton` — one per dispatch invocation

## No CLI, no tabtargets
BURD is transient (dies with process). No rbrX_cli.sh, no render, no list/survey/audit. The sentinel is the primary consumer-facing guarantee.

### regime-tabtarget-completeness (₢ATAAq) [complete]

**[260216-0540] complete**

Create all missing tabtargets, zipper registrations, and workbench help text for regime operations across all regimes.

## Inventory

For each regime, determine required tabtarget set based on cardinality:

### Singleton regimes (RBRR, RBRP, RBRO, BURC, BURS, RBRS)
- validate (no imprint needed)
- render (no imprint needed)

### Manifold regimes (RBRN, RBRV, RBRA, RBRE, RBRG)
- validate (imprint = instance identifier)
- render (imprint = instance identifier)
- list (no imprint)
- survey (no imprint, where implemented)
- audit (no imprint, where implemented)

### No tabtargets (BURD)
- Ephemeral/constructed regime — no CLI, no tabtargets

## Already exists (verify, do not recreate)
- RBRN: rbw-rnr, rbw-rnv, rbw-ni (survey), rbw-nv (audit)
- RBRV: rbw-rvr, rbw-rvv
- RBRR: rbw-rrr, rbw-rrv, rbw-rrg (refresh pins)
- BURC: buw-rgr-burc, buw-rgv-burc (in BUK workbench)
- BURS: buw-rgr-burs, buw-rgv-burs (in BUK workbench)

## Create missing

### Tabtarget launcher scripts (tt/)
Standard 3-line launcher format. One per missing operation.

### Zipper registrations (Tools/rbw/rbz_zipper.sh)
Add `buz_register` calls for each new RBW tabtarget.
For BUK tabtargets, update the BUK zipper if applicable.

### Workbench help text
Update `rbw_workbench.sh` help display to include new regime operations.
Group by regime, show singleton vs manifold operations appropriately.

## Naming convention
Follow established colophon patterns:
- `rbw-rn*` for RBRN operations
- `rbw-rv*` for RBRV operations
- `rbw-rr*` for RBRR operations
- Mint new colophons for RBRP, RBRO, RBRA, RBRE, RBRG, RBRS following prefix naming discipline

## Acceptance criteria
- Every regime with a CLI has tabtargets for all its operations
- All tabtargets registered in appropriate zipper
- Workbench help text is complete and organized
- Tab-completion works for all new launchers

**[260215-0853] rough**

Create all missing tabtargets, zipper registrations, and workbench help text for regime operations across all regimes.

## Inventory

For each regime, determine required tabtarget set based on cardinality:

### Singleton regimes (RBRR, RBRP, RBRO, BURC, BURS, RBRS)
- validate (no imprint needed)
- render (no imprint needed)

### Manifold regimes (RBRN, RBRV, RBRA, RBRE, RBRG)
- validate (imprint = instance identifier)
- render (imprint = instance identifier)
- list (no imprint)
- survey (no imprint, where implemented)
- audit (no imprint, where implemented)

### No tabtargets (BURD)
- Ephemeral/constructed regime — no CLI, no tabtargets

## Already exists (verify, do not recreate)
- RBRN: rbw-rnr, rbw-rnv, rbw-ni (survey), rbw-nv (audit)
- RBRV: rbw-rvr, rbw-rvv
- RBRR: rbw-rrr, rbw-rrv, rbw-rrg (refresh pins)
- BURC: buw-rgr-burc, buw-rgv-burc (in BUK workbench)
- BURS: buw-rgr-burs, buw-rgv-burs (in BUK workbench)

## Create missing

### Tabtarget launcher scripts (tt/)
Standard 3-line launcher format. One per missing operation.

### Zipper registrations (Tools/rbw/rbz_zipper.sh)
Add `buz_register` calls for each new RBW tabtarget.
For BUK tabtargets, update the BUK zipper if applicable.

### Workbench help text
Update `rbw_workbench.sh` help display to include new regime operations.
Group by regime, show singleton vs manifold operations appropriately.

## Naming convention
Follow established colophon patterns:
- `rbw-rn*` for RBRN operations
- `rbw-rv*` for RBRV operations
- `rbw-rr*` for RBRR operations
- Mint new colophons for RBRP, RBRO, RBRA, RBRE, RBRG, RBRS following prefix naming discipline

## Acceptance criteria
- Every regime with a CLI has tabtargets for all its operations
- All tabtargets registered in appropriate zipper
- Workbench help text is complete and organized
- Tab-completion works for all new launchers

**[260215-0716] rough**

Create all missing tabtargets, zipper registrations, and workbench help text for regime operations across all regimes.

## Inventory

For each regime, determine required tabtarget set based on cardinality:

### Singleton regimes (RBRR, RBRP, BURC, BURS, RBRS)
- validate (no imprint needed)
- render (no imprint needed)

### Manifold regimes (RBRN, RBRV, RBRA, RBRE, RBRG)
- validate (imprint = instance identifier)
- render (imprint = instance identifier)
- list (no imprint)
- survey (no imprint, where implemented)
- audit (no imprint, where implemented)

## Already exists (verify, do not recreate)
- RBRN: rbw-rnr, rbw-rnv, rbw-ni (survey), rbw-nv (audit)
- RBRV: rbw-rvr, rbw-rvv
- RBRR: rbw-rrr, rbw-rrv, rbw-rrg (refresh pins)
- BURC: buw-rgr-burc, buw-rgv-burc (in BUK workbench)
- BURS: buw-rgr-burs, buw-rgv-burs (in BUK workbench)

## Create missing

### Tabtarget launcher scripts (tt/)
Standard 3-line launcher format. One per missing operation.

### Zipper registrations (Tools/rbw/rbz_zipper.sh)
Add `buz_register` calls for each new RBW tabtarget.
For BUK tabtargets, update the BUK zipper if applicable.

### Workbench help text
Update `rbw_workbench.sh` help display to include new regime operations.
Group by regime, show singleton vs manifold operations appropriately.

## Naming convention
Follow established colophon patterns:
- `rbw-rn*` for RBRN operations
- `rbw-rv*` for RBRV operations
- `rbw-rr*` for RBRR operations
- Mint new colophons for RBRP, RBRA, RBRE, RBRG, RBRS following prefix naming discipline

## Acceptance criteria
- Every regime with a CLI has tabtargets for all its operations
- All tabtargets registered in appropriate zipper
- Workbench help text is complete and organized
- Tab-completion works for all new launchers

### regime-smoke-test-expansion (₢ATAAs) [complete]

**[260216-0613] complete**

Expand regime smoke tests to cover all regimes, including a new credentials-required suite for external regimes.

## Context

Current `butcrg_RegimeSmoke.sh` covers 5 regimes: BURC, BURS, RBRN, RBRR, RBRV. After this heat's work, there are 12 regimes total (including BURD). Several are missing coverage.

Two test suites needed:
- **regime-smoke** (existing, expanded): git-tracked regimes, CI-safe, always runnable
- **regime-credentials** (new): external regimes requiring credential files on developer workstation

## Expand regime-smoke suite

### Add to butcrg_RegimeSmoke.sh:

`butcrg_rbrp` — validate + render RBRP via new tabtargets (git-tracked rbrp.env, CI-safe)

`butcrg_burd` — verify BURD sentinel is set after dispatch setup. This tests that zburd_sentinel() passes in the test dispatch context (BURD is already constructed by the test infrastructure's bute_init_dispatch).

### Expand existing cases for manifold list operations:

`butcrg_rbrn` — add list operation test (invoke rbrn_list, verify non-empty output, verify each listed moniker validates)

`butcrg_rbrv` — add list operation test (invoke rbrv_list, verify non-empty output, verify each listed sigil validates)

### Enroll new cases in rbtb_testbench.sh:
Add `butr_case_enroll "regime-smoke" butcrg_rbrp` and `butcrg_burd` to existing enrollment block.

## Create regime-credentials suite

### New file: butcrg_RegimeCredentials.sh (or extend butcrg_RegimeSmoke.sh with gated section)

Test cases for external regimes that require credential files present:

`butcrg_rbra` — For each of RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE: verify file exists, validate, render (with masked private key). Requires RBRR kindled first for file paths.

`butcrg_rbre` — validate + render RBRE regime. Requires external RBRE credential file.

`butcrg_rbrg` — validate + render RBRG regime. Requires external RBRG credential file.

`butcrg_rbro` — validate + render RBRO regime. Requires external RBRO OAuth credential file. Verify both RBRO_CLIENT_SECRET and RBRO_REFRESH_TOKEN are masked in render output.

`butcrg_rbrs` — validate + render RBRS regime. Requires external RBRS station file (path from BURC_STATION_FILE → BURS).

### Prerequisite checking:
Each credentials test case should verify its required file exists BEFORE attempting validate/render. If the file is missing, fail with a clear message: "RBRO credential file not found: <path>. This suite requires a fully configured workstation."

Do NOT silently skip — if you run the credentials suite, you expect credentials present.

### Enroll in rbtb_testbench.sh:
Add new suite "regime-credentials" with all five cases.
Add tabtarget for the new suite (e.g., `rbw-trc` or similar — mint colophon following naming discipline).

## Tabtarget for new suite
Create launcher in tt/ and register in rbz_zipper.sh. Follow existing regime-smoke pattern (`rbw-trg`).

## Reference
- Existing smoke test: Tools/buk/butcrg_RegimeSmoke.sh
- Testbench enrollment: Tools/rbw/rbtb_testbench.sh lines 211-215
- rbcr_* render pattern for verifying render output
- Exemplar: butcrg_rbrn, butcrg_rbrv for dispatch pattern

**[260215-0853] rough**

Expand regime smoke tests to cover all regimes, including a new credentials-required suite for external regimes.

## Context

Current `butcrg_RegimeSmoke.sh` covers 5 regimes: BURC, BURS, RBRN, RBRR, RBRV. After this heat's work, there are 12 regimes total (including BURD). Several are missing coverage.

Two test suites needed:
- **regime-smoke** (existing, expanded): git-tracked regimes, CI-safe, always runnable
- **regime-credentials** (new): external regimes requiring credential files on developer workstation

## Expand regime-smoke suite

### Add to butcrg_RegimeSmoke.sh:

`butcrg_rbrp` — validate + render RBRP via new tabtargets (git-tracked rbrp.env, CI-safe)

`butcrg_burd` — verify BURD sentinel is set after dispatch setup. This tests that zburd_sentinel() passes in the test dispatch context (BURD is already constructed by the test infrastructure's bute_init_dispatch).

### Expand existing cases for manifold list operations:

`butcrg_rbrn` — add list operation test (invoke rbrn_list, verify non-empty output, verify each listed moniker validates)

`butcrg_rbrv` — add list operation test (invoke rbrv_list, verify non-empty output, verify each listed sigil validates)

### Enroll new cases in rbtb_testbench.sh:
Add `butr_case_enroll "regime-smoke" butcrg_rbrp` and `butcrg_burd` to existing enrollment block.

## Create regime-credentials suite

### New file: butcrg_RegimeCredentials.sh (or extend butcrg_RegimeSmoke.sh with gated section)

Test cases for external regimes that require credential files present:

`butcrg_rbra` — For each of RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE: verify file exists, validate, render (with masked private key). Requires RBRR kindled first for file paths.

`butcrg_rbre` — validate + render RBRE regime. Requires external RBRE credential file.

`butcrg_rbrg` — validate + render RBRG regime. Requires external RBRG credential file.

`butcrg_rbro` — validate + render RBRO regime. Requires external RBRO OAuth credential file. Verify both RBRO_CLIENT_SECRET and RBRO_REFRESH_TOKEN are masked in render output.

`butcrg_rbrs` — validate + render RBRS regime. Requires external RBRS station file (path from BURC_STATION_FILE → BURS).

### Prerequisite checking:
Each credentials test case should verify its required file exists BEFORE attempting validate/render. If the file is missing, fail with a clear message: "RBRO credential file not found: <path>. This suite requires a fully configured workstation."

Do NOT silently skip — if you run the credentials suite, you expect credentials present.

### Enroll in rbtb_testbench.sh:
Add new suite "regime-credentials" with all five cases.
Add tabtarget for the new suite (e.g., `rbw-trc` or similar — mint colophon following naming discipline).

## Tabtarget for new suite
Create launcher in tt/ and register in rbz_zipper.sh. Follow existing regime-smoke pattern (`rbw-trg`).

## Reference
- Existing smoke test: Tools/buk/butcrg_RegimeSmoke.sh
- Testbench enrollment: Tools/rbw/rbtb_testbench.sh lines 211-215
- rbcr_* render pattern for verifying render output
- Exemplar: butcrg_rbrn, butcrg_rbrv for dispatch pattern

**[260215-0828] rough**

Expand regime smoke tests to cover all regimes, including a new credentials-required suite for external regimes.

## Context

Current `butcrg_RegimeSmoke.sh` covers 5 regimes: BURC, BURS, RBRN, RBRR, RBRV. After this heat's work, there are 11 regimes total. Six are missing coverage.

Two test suites needed:
- **regime-smoke** (existing, expanded): git-tracked regimes, CI-safe, always runnable
- **regime-credentials** (new): external regimes requiring credential files on developer workstation

## Expand regime-smoke suite

### Add to butcrg_RegimeSmoke.sh:

`butcrg_rbrp` — validate + render RBRP via new tabtargets (git-tracked rbrp.env, CI-safe)

`butcrg_burd` — verify BURD sentinel is set after dispatch setup. This tests that zburd_sentinel() passes in the test dispatch context (BURD is already constructed by the test infrastructure's bute_init_dispatch).

### Expand existing cases for manifold list operations:

`butcrg_rbrn` — add list operation test (invoke rbrn_list, verify non-empty output, verify each listed moniker validates)

`butcrg_rbrv` — add list operation test (invoke rbrv_list, verify non-empty output, verify each listed sigil validates)

### Enroll new cases in rbtb_testbench.sh:
Add `butr_case_enroll "regime-smoke" butcrg_rbrp` and `butcrg_burd` to existing enrollment block.

## Create regime-credentials suite

### New file: butcrg_RegimeCredentials.sh (or extend butcrg_RegimeSmoke.sh with gated section)

Test cases for external regimes that require credential files present:

`butcrg_rbra` — For each of RBRR_GOVERNOR_RBRA_FILE, RBRR_RETRIEVER_RBRA_FILE, RBRR_DIRECTOR_RBRA_FILE: verify file exists, validate, render (with masked private key). Requires RBRR kindled first for file paths.

`butcrg_rbre` — validate + render RBRE regime. Requires external RBRE credential file.

`butcrg_rbrg` — validate + render RBRG regime. Requires external RBRG credential file.

`butcrg_rbrs` — validate + render RBRS regime. Requires external RBRS station file (path from BURC_STATION_FILE → BURS).

### Prerequisite checking:
Each credentials test case should verify its required file exists BEFORE attempting validate/render. If the file is missing, fail with a clear message: "RBRE credential file not found: <path>. This suite requires a fully configured workstation."

Do NOT silently skip — if you run the credentials suite, you expect credentials present.

### Enroll in rbtb_testbench.sh:
Add new suite "regime-credentials" with all four cases.
Add tabtarget for the new suite (e.g., `rbw-trc` or similar — mint colophon following naming discipline).

## Tabtarget for new suite
Create launcher in tt/ and register in rbz_zipper.sh. Follow existing regime-smoke pattern (`rbw-trg`).

## Reference
- Existing smoke test: Tools/buk/butcrg_RegimeSmoke.sh
- Testbench enrollment: Tools/rbw/rbtb_testbench.sh lines 211-215
- rbcr_* render pattern for verifying render output
- Exemplar: butcrg_rbrn, butcrg_rbrv for dispatch pattern

### audit-regime-variable-voicings (₢ATAAp) [complete]

**[260216-0550] complete**

Full cross-regime audit of all variable voicings across RBSA and BUSA. Quality gate before wiring tabtargets.

## Scope

For EVERY regime (BURC, BURS, BURD, RBRR, RBRN, RBRV, RBRP, RBRE, RBRG, RBRS, RBRA, RBRO):

### 1. Variable count match
Compare the number of variables in:
- The validator (regime.sh kindle function)
- The spec (RBSA/BUSA mapping section attribute references)
- The subdocument (axhrgv_variable / axhrv_variable markers)

All three counts must agree. Flag any discrepancies.

### 2. Dimension consistency
For each variable, verify:
- `axd_required` / `axd_optional` / `axd_conditional` matches validator logic
- Type annotation (axtu_path, axtu_xname, axtu_string, etc.) matches buv_env_* call
- Conditional variables are in groups with proper axhrgc_gate markers

### 3. Operation completeness
For each regime, verify:
- Singleton regimes have: kindle, validate, render
- Manifold regimes have: kindle, validate, render, list (minimum); survey/audit where implemented
- BURD: kindle and validate only (ephemeral, no render/list)
- All operation voicings use axhro_kindle (not axhro_broach — catch any missed renames)

### 4. Cardinality annotation check
Every `// ⟦axvr_regime ...⟧` annotation includes axrd_singleton or axrd_manifold.

### 5. Cross-document consistency
- Every RBSA/BUSA attribute reference has a corresponding anchor
- Every subdocument operation voices the correct kit-level term (rbkro_* or bukro_*)
- No orphaned attribute references (mapping entry with no anchor)

### 6. Remove RBEV placeholder
RBEV is a dead concept. The RBSA section at ~line 2501 ("Environment Variables Regime") is a TBD placeholder with no variables, no validator, no CLI. The `RBEV_` prefix only appears as Dockerfile-local ARGs in `rbev-bottle-anthropic-jupyter/Dockerfile`, not as a regime. The `rbev-` prefix on vessel directory names is the vessel sigil convention, unrelated to a regime.

Actions:
- Remove the RBEV section from RBSA (the `=== Environment Variables Regime (RBEV)` block)
- Remove `rbev_regime` and `rbev_prefix` attribute references from the RBSA mapping section
- Remove corresponding anchors (`[[rbev_regime]]`, `[[rbev_prefix]]`)
- Verify no other documents reference the removed terms
- Note in audit memo: "RBEV was a placeholder concept; per-vessel build parameters are Dockerfile-local, not a regime"

## Output
Create a memo documenting findings: Memos/memo-YYYYMMDD-regime-voicing-audit.md
List all fixes applied during this pace.

## Acceptance criteria
- Zero variable count discrepancies
- All dimensions match validator logic
- All operation markers use post-rename terms
- All cardinality annotations present
- RBEV placeholder removed from RBSA

**[260216-0510] bridled**

Full cross-regime audit of all variable voicings across RBSA and BUSA. Quality gate before wiring tabtargets.

## Scope

For EVERY regime (BURC, BURS, BURD, RBRR, RBRN, RBRV, RBRP, RBRE, RBRG, RBRS, RBRA, RBRO):

### 1. Variable count match
Compare the number of variables in:
- The validator (regime.sh kindle function)
- The spec (RBSA/BUSA mapping section attribute references)
- The subdocument (axhrgv_variable / axhrv_variable markers)

All three counts must agree. Flag any discrepancies.

### 2. Dimension consistency
For each variable, verify:
- `axd_required` / `axd_optional` / `axd_conditional` matches validator logic
- Type annotation (axtu_path, axtu_xname, axtu_string, etc.) matches buv_env_* call
- Conditional variables are in groups with proper axhrgc_gate markers

### 3. Operation completeness
For each regime, verify:
- Singleton regimes have: kindle, validate, render
- Manifold regimes have: kindle, validate, render, list (minimum); survey/audit where implemented
- BURD: kindle and validate only (ephemeral, no render/list)
- All operation voicings use axhro_kindle (not axhro_broach — catch any missed renames)

### 4. Cardinality annotation check
Every `// ⟦axvr_regime ...⟧` annotation includes axrd_singleton or axrd_manifold.

### 5. Cross-document consistency
- Every RBSA/BUSA attribute reference has a corresponding anchor
- Every subdocument operation voices the correct kit-level term (rbkro_* or bukro_*)
- No orphaned attribute references (mapping entry with no anchor)

### 6. Remove RBEV placeholder
RBEV is a dead concept. The RBSA section at ~line 2501 ("Environment Variables Regime") is a TBD placeholder with no variables, no validator, no CLI. The `RBEV_` prefix only appears as Dockerfile-local ARGs in `rbev-bottle-anthropic-jupyter/Dockerfile`, not as a regime. The `rbev-` prefix on vessel directory names is the vessel sigil convention, unrelated to a regime.

Actions:
- Remove the RBEV section from RBSA (the `=== Environment Variables Regime (RBEV)` block)
- Remove `rbev_regime` and `rbev_prefix` attribute references from the RBSA mapping section
- Remove corresponding anchors (`[[rbev_regime]]`, `[[rbev_prefix]]`)
- Verify no other documents reference the removed terms
- Note in audit memo: "RBEV was a placeholder concept; per-vessel build parameters are Dockerfile-local, not a regime"

## Output
Create a memo documenting findings: Memos/memo-YYYYMMDD-regime-voicing-audit.md
List all fixes applied during this pace.

## Acceptance criteria
- Zero variable count discrepancies
- All dimensions match validator logic
- All operation markers use post-rename terms
- All cardinality annotations present
- RBEV placeholder removed from RBSA

*Direction:* Agent: opus | Cardinality: 1 sequential | Files: burc_regime.sh, burs_regime.sh, rbra_regime.sh, rbrn_regime.sh, rbro_regime.sh, rbrp_regime.sh, rbrr_regime.sh, rbrs_regime.sh, rbrv_regime.sh, RBS0-SpecTop.adoc, BUS0-BashUtilitiesSpec.adoc, BUSD-DispatchRuntime.adoc, RBSRR-RegimeRepo.adoc, RBSRV-RegimeVessel.adoc, RBRN-RegimeNameplate.adoc, RBSRP-RegimePayor.adoc, RBSRS-RegimeStation.adoc, RBSRA-CredentialFormat.adoc, RBSRO-RegimeOauth.adoc, memo-20260216-regime-voicing-audit.md (20 files) | Steps: 1. Read 9 regime.sh kindle functions, count variables per regime 2. Read RBSA and BUSA mapping sections, count attribute refs per regime 3. Read 7 subdocuments, count axhrgv_variable markers 4. Compare counts across all three sources, fix discrepancies 5. Verify dimension annotations match validator logic 6. Verify operation completeness -- singleton vs manifold sets 7. Check all cardinality annotations present 8. Cross-check attribute refs have anchors, no orphans 9. Remove RBEV placeholder section from RBSA 10. Write audit memo to Memos/memo-20260216-regime-voicing-audit.md | Verify: no axhro_broach in any .adoc | SKIP: Do NOT touch RBRE or RBRG -- these regimes are being deleted elsewhere

**[260215-0850] rough**

Full cross-regime audit of all variable voicings across RBSA and BUSA. Quality gate before wiring tabtargets.

## Scope

For EVERY regime (BURC, BURS, BURD, RBRR, RBRN, RBRV, RBRP, RBRE, RBRG, RBRS, RBRA, RBRO):

### 1. Variable count match
Compare the number of variables in:
- The validator (regime.sh kindle function)
- The spec (RBSA/BUSA mapping section attribute references)
- The subdocument (axhrgv_variable / axhrv_variable markers)

All three counts must agree. Flag any discrepancies.

### 2. Dimension consistency
For each variable, verify:
- `axd_required` / `axd_optional` / `axd_conditional` matches validator logic
- Type annotation (axtu_path, axtu_xname, axtu_string, etc.) matches buv_env_* call
- Conditional variables are in groups with proper axhrgc_gate markers

### 3. Operation completeness
For each regime, verify:
- Singleton regimes have: kindle, validate, render
- Manifold regimes have: kindle, validate, render, list (minimum); survey/audit where implemented
- BURD: kindle and validate only (ephemeral, no render/list)
- All operation voicings use axhro_kindle (not axhro_broach — catch any missed renames)

### 4. Cardinality annotation check
Every `// ⟦axvr_regime ...⟧` annotation includes axrd_singleton or axrd_manifold.

### 5. Cross-document consistency
- Every RBSA/BUSA attribute reference has a corresponding anchor
- Every subdocument operation voices the correct kit-level term (rbkro_* or bukro_*)
- No orphaned attribute references (mapping entry with no anchor)

### 6. Remove RBEV placeholder
RBEV is a dead concept. The RBSA section at ~line 2501 ("Environment Variables Regime") is a TBD placeholder with no variables, no validator, no CLI. The `RBEV_` prefix only appears as Dockerfile-local ARGs in `rbev-bottle-anthropic-jupyter/Dockerfile`, not as a regime. The `rbev-` prefix on vessel directory names is the vessel sigil convention, unrelated to a regime.

Actions:
- Remove the RBEV section from RBSA (the `=== Environment Variables Regime (RBEV)` block)
- Remove `rbev_regime` and `rbev_prefix` attribute references from the RBSA mapping section
- Remove corresponding anchors (`[[rbev_regime]]`, `[[rbev_prefix]]`)
- Verify no other documents reference the removed terms
- Note in audit memo: "RBEV was a placeholder concept; per-vessel build parameters are Dockerfile-local, not a regime"

## Output
Create a memo documenting findings: Memos/memo-YYYYMMDD-regime-voicing-audit.md
List all fixes applied during this pace.

## Acceptance criteria
- Zero variable count discrepancies
- All dimensions match validator logic
- All operation markers use post-rename terms
- All cardinality annotations present
- RBEV placeholder removed from RBSA

**[260215-0715] rough**

Full cross-regime audit of all variable voicings across RBSA and BUSA. Quality gate before wiring tabtargets.

## Scope

For EVERY regime (BURC, BURS, RBRR, RBRN, RBRV, RBRP, RBRE, RBRG, RBRS, RBRA):

### 1. Variable count match
Compare the number of variables in:
- The validator (regime.sh kindle function)
- The spec (RBSA/BUSA mapping section attribute references)
- The subdocument (axhrgv_variable / axhrv_variable markers)

All three counts must agree. Flag any discrepancies.

### 2. Dimension consistency
For each variable, verify:
- `axd_required` / `axd_optional` / `axd_conditional` matches validator logic
- Type annotation (axtu_path, axtu_xname, axtu_string, etc.) matches buv_env_* call
- Conditional variables are in groups with proper axhrgc_gate markers

### 3. Operation completeness
For each regime, verify:
- Singleton regimes have: kindle, validate, render
- Manifold regimes have: kindle, validate, render, list (minimum); survey/audit where implemented
- All operation voicings use axhro_kindle (not axhro_broach — catch any missed renames)

### 4. Cardinality annotation check
Every `// ⟦axvr_regime ...⟧` annotation includes axrd_singleton or axrd_manifold.

### 5. Cross-document consistency
- Every RBSA/BUSA attribute reference has a corresponding anchor
- Every subdocument operation voices the correct kit-level term (rbkro_* or bukro_*)
- No orphaned attribute references (mapping entry with no anchor)

## Output
Create a memo documenting findings: Memos/memo-YYYYMMDD-regime-voicing-audit.md
List all fixes applied during this pace.

## Acceptance criteria
- Zero variable count discrepancies
- All dimensions match validator logic
- All operation markers use post-rename terms
- All cardinality annotations present

### review-checkpoint-findings (₢ATAAu) [complete]

**[260216-0520] complete**

Findings from regime-reviewer session (2026-02-15). Study each item to determine resolution before cleanup.

## Finding 1: ATAAK — rbl_kindle_all calls survive in 3 CLIs [HIGH confidence to fix]
- Tools/rbw/rbga_cli.sh:35 calls rbl_kindle_all (deleted function)
- Tools/rbw/rbgb_cli.sh:35 calls rbl_kindle_all (deleted function)
- Tools/rgbs_cli.sh:34 calls rbl_kindle_all (deleted function)
These are runtime failures. Likely just need the calls removed (dead code), but study what rbl_kindle_all actually did before deleting — it may have kindled dependencies these CLIs still need.

## Finding 2: ATAAm — old-style variable annotations in RBSA [LOW confidence — needs study]
RBSA lines ~1989-2021 use `// ⟦axl_voices axrg_variable ...⟧` instead of modern `// ⟦axvr_variable ...⟧` style. Question: is this a RBRP-specific issue from this pace, or a broader pattern across all RBSA variable annotations? If broader, this is a separate normalization sweep, not a pace-specific fix. Need to assess scope before acting.

## Finding 3: ATAAm — orphan rbrp_depot_project_ids in RBSA [LOW confidence — needs study]
RBSA line ~2020-2023 defines rbrp_depot_project_ids with axd_repeated dimension. This variable does NOT appear in RBSRP-RegimePayor.adoc subdocument or in rbrp_regime.sh validator. Is this a real variable that was missed, or a stale artifact? Need to check rbrp_regime.sh and rbrp.env to determine if this variable exists.

## Finding 4: ATAAo — RBRE/RBRG marked singleton, architecture says manifold [LOW confidence — needs decision]
RBSA line ~2061: RBRE annotated axrd_singleton
RBSA line ~2105: RBRG annotated axrd_singleton
Regime Cardinality Map says both are `manifold(?)` with question marks. Need to decide:
- Option A: Commit to manifold now (add list operations to CLIs and subdocuments)
- Option B: Keep singleton for now, change when multi-instance support is actually needed
- Option C: The question marks meant "uncertain" — resolve the actual cardinality based on current usage

## Action
Study each finding. For items marked HIGH confidence, the reviewer agent can likely fix mechanically. For LOW confidence items, human judgment needed before proceeding.

**[260215-0858] rough**

Findings from regime-reviewer session (2026-02-15). Study each item to determine resolution before cleanup.

## Finding 1: ATAAK — rbl_kindle_all calls survive in 3 CLIs [HIGH confidence to fix]
- Tools/rbw/rbga_cli.sh:35 calls rbl_kindle_all (deleted function)
- Tools/rbw/rbgb_cli.sh:35 calls rbl_kindle_all (deleted function)
- Tools/rgbs_cli.sh:34 calls rbl_kindle_all (deleted function)
These are runtime failures. Likely just need the calls removed (dead code), but study what rbl_kindle_all actually did before deleting — it may have kindled dependencies these CLIs still need.

## Finding 2: ATAAm — old-style variable annotations in RBSA [LOW confidence — needs study]
RBSA lines ~1989-2021 use `// ⟦axl_voices axrg_variable ...⟧` instead of modern `// ⟦axvr_variable ...⟧` style. Question: is this a RBRP-specific issue from this pace, or a broader pattern across all RBSA variable annotations? If broader, this is a separate normalization sweep, not a pace-specific fix. Need to assess scope before acting.

## Finding 3: ATAAm — orphan rbrp_depot_project_ids in RBSA [LOW confidence — needs study]
RBSA line ~2020-2023 defines rbrp_depot_project_ids with axd_repeated dimension. This variable does NOT appear in RBSRP-RegimePayor.adoc subdocument or in rbrp_regime.sh validator. Is this a real variable that was missed, or a stale artifact? Need to check rbrp_regime.sh and rbrp.env to determine if this variable exists.

## Finding 4: ATAAo — RBRE/RBRG marked singleton, architecture says manifold [LOW confidence — needs decision]
RBSA line ~2061: RBRE annotated axrd_singleton
RBSA line ~2105: RBRG annotated axrd_singleton
Regime Cardinality Map says both are `manifold(?)` with question marks. Need to decide:
- Option A: Commit to manifold now (add list operations to CLIs and subdocuments)
- Option B: Keep singleton for now, change when multi-instance support is actually needed
- Option C: The question marks meant "uncertain" — resolve the actual cardinality based on current usage

## Action
Study each finding. For items marked HIGH confidence, the reviewer agent can likely fix mechanically. For LOW confidence items, human judgment needed before proceeding.

### rbro-oauth-regime-formalization (₢ATAAt) [complete]

**[260215-0856] complete**

Formalize RBRO (OAuth Regime) with a validator, CLI, and subdocument. RBRO is a small external credential regime (2 variables) similar in nature to RBRG. Currently lives inline in RBSA with anchors and definitions but has no validator, no CLI, and no subdocument.

## Context

RBRO holds OAuth 2.0 credentials used by rbgo_OAuth.sh for token generation.
- RBRO_CLIENT_SECRET — OAuth client secret from Google Cloud Console
- RBRO_REFRESH_TOKEN — long-lived refresh token from authorization flow
Both are sensitive credentials, NOT in git. External assignment file.

Currently referenced in RBSA at ~line 2028 with proper anchors and voicings.
Has an existing load pattern (rbtoe_rbro_load) but no formal regime infrastructure.

## Create: Tools/rbw/rbro_regime.sh

Following regime.sh pattern:
- Multiple inclusion guard (ZRBRO_SOURCED)
- zrbro_kindle() with ZRBRO_KINDLED guard
- Validate 2 variables:
  - RBRO_CLIENT_SECRET (string, required)
  - RBRO_REFRESH_TOKEN (string, required)
- Unexpected variable detection via compgen
- ZRBRO_ROLLUP construction
- zrbro_sentinel() function

## Create: Tools/rbw/rbro_cli.sh

Following rbrn_cli.sh pattern:
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, render using rbcr_section_begin/item/end
  - Section: OAuth Credentials (client secret, refresh token)
  - Security: MASK both values in render display — these are secrets
- Singleton regime — no list/survey/audit

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern.

## Create: lenses/RBSRO-RegimeOauth.adoc

Subdocument following RBRN pattern:
- // ⟦axhrb_regime⟧ header with {rbro_regime} reference
- Variable voicings for both variables
- Operation voicings: axhro_kindle, axhro_validate, axhro_render

## Update: lenses/RBS0-SpecTop.adoc

- Move inline RBRO definitions to subdocument
- Keep 1-sentence gestalt at parent anchor
- Wire include::RBSRO-RegimeOauth.adoc[]
- Cardinality: axrd_singleton
- Provenance: axrd_file_sourced (external credential file)

## Migrate load pattern
- Assess rbtoe_rbro_load and migrate to rbro_load() in rbro_regime.sh following rbrr_load/rbrn_load_file patterns
- Update consumers of rbtoe_rbro_load to use new function

**[260215-0845] rough**

Formalize RBRO (OAuth Regime) with a validator, CLI, and subdocument. RBRO is a small external credential regime (2 variables) similar in nature to RBRG. Currently lives inline in RBSA with anchors and definitions but has no validator, no CLI, and no subdocument.

## Context

RBRO holds OAuth 2.0 credentials used by rbgo_OAuth.sh for token generation.
- RBRO_CLIENT_SECRET — OAuth client secret from Google Cloud Console
- RBRO_REFRESH_TOKEN — long-lived refresh token from authorization flow
Both are sensitive credentials, NOT in git. External assignment file.

Currently referenced in RBSA at ~line 2028 with proper anchors and voicings.
Has an existing load pattern (rbtoe_rbro_load) but no formal regime infrastructure.

## Create: Tools/rbw/rbro_regime.sh

Following regime.sh pattern:
- Multiple inclusion guard (ZRBRO_SOURCED)
- zrbro_kindle() with ZRBRO_KINDLED guard
- Validate 2 variables:
  - RBRO_CLIENT_SECRET (string, required)
  - RBRO_REFRESH_TOKEN (string, required)
- Unexpected variable detection via compgen
- ZRBRO_ROLLUP construction
- zrbro_sentinel() function

## Create: Tools/rbw/rbro_cli.sh

Following rbrn_cli.sh pattern:
- validate command: source file, kindle, validate fields
- render command: source file, kindle, rbcr_kindle, render using rbcr_section_begin/item/end
  - Section: OAuth Credentials (client secret, refresh token)
  - Security: MASK both values in render display — these are secrets
- Singleton regime — no list/survey/audit

Reference rbrn_cli.sh and rbrv_cli.sh for the rbcr_* shared rendering pattern.

## Create: lenses/RBSRO-RegimeOauth.adoc

Subdocument following RBRN pattern:
- // ⟦axhrb_regime⟧ header with {rbro_regime} reference
- Variable voicings for both variables
- Operation voicings: axhro_kindle, axhro_validate, axhro_render

## Update: lenses/RBS0-SpecTop.adoc

- Move inline RBRO definitions to subdocument
- Keep 1-sentence gestalt at parent anchor
- Wire include::RBSRO-RegimeOauth.adoc[]
- Cardinality: axrd_singleton
- Provenance: axrd_file_sourced (external credential file)

## Migrate load pattern
- Assess rbtoe_rbro_load and migrate to rbro_load() in rbro_regime.sh following rbrr_load/rbrn_load_file patterns
- Update consumers of rbtoe_rbro_load to use new function

### audit-regime-validation-gates (₢ATAAv) [complete]

**[260216-0605] complete**

Audit that every regime consumption point is guarded by validation.

Three audit vectors:

1. **Direct .env sourcing** — grep for scripts that `source` a regime .env file
   without going through the canonical `*_load()` function. Any direct source
   bypasses the atomic source→kindle→validate chain.

2. **Regime variable usage outside load→furnish chain** — find consumption of
   RBRR_*, RBRN_*, RBRP_*, RBRO_*, RBRS_*, RBRA_*, RBRV_* variables in scripts
   that do not call the corresponding `*_load()` in their furnish function.

3. **CLI furnish pattern completeness** — verify every `*_cli.sh` uses the
   buc_execute furnish pattern. Known concern: `rbga_cli.sh` uses an older
   pattern without furnish. Newer CLIs (rbre_cli.sh, rbrg_cli.sh) need
   verification.

Deliverable: a list of gaps (if any) and fixes applied, or confirmation that
all consumption paths are gated.

**[260216-0557] bridled**

Audit that every regime consumption point is guarded by validation.

Three audit vectors:

1. **Direct .env sourcing** — grep for scripts that `source` a regime .env file
   without going through the canonical `*_load()` function. Any direct source
   bypasses the atomic source→kindle→validate chain.

2. **Regime variable usage outside load→furnish chain** — find consumption of
   RBRR_*, RBRN_*, RBRP_*, RBRO_*, RBRS_*, RBRA_*, RBRV_* variables in scripts
   that do not call the corresponding `*_load()` in their furnish function.

3. **CLI furnish pattern completeness** — verify every `*_cli.sh` uses the
   buc_execute furnish pattern. Known concern: `rbga_cli.sh` uses an older
   pattern without furnish. Newer CLIs (rbre_cli.sh, rbrg_cli.sh) need
   verification.

Deliverable: a list of gaps (if any) and fixes applied, or confirmation that
all consumption paths are gated.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/rbw/*_cli.sh, Tools/rbw/*_regime.sh, Tools/rbw/rbrr.validator.sh, Tools/rbw/rbgd_DepotConstants.sh, Tools/buk/bud_dispatch.sh, Tools/buk/buv_validation.sh (all regime CLIs and consumers) | Steps: 1. Grep for direct .env sourcing that bypasses *_load functions 2. Grep for regime variable consumption RBRR_/RBRN_/RBRP_/RBRO_/RBRS_/RBRA_/RBRV_ in scripts lacking corresponding *_load in furnish 3. Verify every *_cli.sh uses buc_execute furnish pattern, flag rbga_cli.sh and rbre_cli.sh and rbrg_cli.sh 4. Fix any gaps by adding furnish functions or replacing direct sourcing with *_load calls 5. Report findings and fixes | Verify: tt/rbw-trg.TestRegimeSmoke.sh

**[260216-0554] rough**

Audit that every regime consumption point is guarded by validation.

Three audit vectors:

1. **Direct .env sourcing** — grep for scripts that `source` a regime .env file
   without going through the canonical `*_load()` function. Any direct source
   bypasses the atomic source→kindle→validate chain.

2. **Regime variable usage outside load→furnish chain** — find consumption of
   RBRR_*, RBRN_*, RBRP_*, RBRO_*, RBRS_*, RBRA_*, RBRV_* variables in scripts
   that do not call the corresponding `*_load()` in their furnish function.

3. **CLI furnish pattern completeness** — verify every `*_cli.sh` uses the
   buc_execute furnish pattern. Known concern: `rbga_cli.sh` uses an older
   pattern without furnish. Newer CLIs (rbre_cli.sh, rbrg_cli.sh) need
   verification.

Deliverable: a list of gaps (if any) and fixes applied, or confirmation that
all consumption paths are gated.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 P but-framework-infrastructure
  2 O convert-ark-lifecycle
  3 N convert-dispatch-exercise
  4 M convert-rbt-suites
  5 L convert-remaining-testbenches
  6 I refactor-but-test-families
  7 G refactor-rbrv-rbrn-render-validate
  8 J extract-but-dispatch
  9 S exercise-but-suites
  10 H audit-regime-validation-consumers
  11 Q study-buv-env-stdout-pattern
  12 D create-rbvr-regime-spec
  13 E process-rbrv-regime-vessel-spec
  14 Z normalize-regime-operation-specs
  15 R implement-rbrr-regime-operation-model
  16 B rename-bus-to-busa
  17 A study-all-recipe-bottle-regimes
  18 C expand-busa-regime-vocabulary
  19 Y consolidate-testbench-into-workbench
  20 c refresh-docker-hub-pins
  21 b improve-test-discoverability
  22 a unify-workbench-zipper-dispatch
  23 F cleanup-rbrr-legacy-variables
  24 f consolidate-podman-vm-spec
  25 g delete-ghcr-legacy-vm-files
  26 i axla-kindle-and-cardinality
  27 j rbsa-busa-vocabulary-alignment
  28 k rbrn-collection-voicings
  29 l rbrv-list-extraction-and-voicings
  30 d rename-rbrr-regime-to-env
  31 h align-rbrr-with-rbrn-rbrv-pattern
  32 K consolidate-rbrp-retire-rbl
  33 m rbrp-regime-spec
  34 n rbra-regime-formalization
  35 o external-regime-specs
  36 r burd-dispatch-regime-formalization
  37 q regime-tabtarget-completeness
  38 s regime-smoke-test-expansion
  39 p audit-regime-variable-voicings
  40 u review-checkpoint-findings
  41 t rbro-oauth-regime-formalization
  42 v audit-regime-validation-gates

PONMLIGJSHQDEZRBACYcbaFfgijkldhKmnorqsputv
············x···········x·x···x·xxx·x·x·x· RBSA-SpecTop.adoc
xxxxx············xx·x···············x····x rbtb_testbench.sh
······x··x····x···x·xx··············x····x rbw_workbench.sh
x····x············x··x··············x····· buz_zipper.sh
························x····xx·····x····· rbrr_regime.sh
···············x·x········x········x······ BUSA-BashUtilitiesSpec.adoc
···············x·x···x·········x·········· CLAUDE.md
··············x·········x·····x·····x····· rbrr_cli.sh
······x·····················x·······x·x··· rbrv_regime.sh
······x··x··················x·········x··· rbrv_cli.sh
···x·xx··x································ rbt_testbench.sh
······························x·····x·x··· RBSRR-RegimeRepo.adoc
······················x·x·····x··········· RBRR-RegimeRepo.adoc
······················x·x····x············ rbw.workbench.mk
·····················x··············x····x rbz_zipper.sh
······································x··x rbrv.env
····································x···x· rbro_cli.sh
····································x·x··· memo-20260216-regime-voicing-audit.md
··································x·x····· rbrs_cli.sh
·································x··x····· rbra_cli.sh
································x···x····· rbrp_cli.sh
·····························x·x·········· rbcc_Constants.sh, rbl_Locator.sh
························x····x············ rbrr_RecipeBottleRegimeRepo.sh
···················x···············x······ bud_dispatch.sh
··················x·x····················· rbw-tj.TestSrjclJupyter.srjcl.sh, rbw-tns.TestNsproSecurity.nsproto.sh, rbw-tpl.TestPlumlDiagram.pluml.sh
·················x·······················x butcrg_RegimeSmoke.sh
·················xx······················· bupr_PresentationRegime.sh, rbtb-rg.TestRegimeSmoke.sh
················x············x············ memo-20260209-regime-inventory.md
············x···············x············· RBSRV-RegimeVessel.adoc
············x··············x·············· RBRN-RegimeNameplate.adoc
············x············x················ AXLA-Lexicon.adoc
···········xx····························· RBRV-RegimeVessel.adoc
······x··x································ rbob_cli.sh, rbrn_cli.sh, rbrn_regime.sh
····x····x································ butcvu_XnameValidation.sh
····xx···································· tbvu_suite_xname.sh, trbim_suite.sh
···x··············x······················· rbtb-ns.TestNsproSecurity.nsproto.sh, rbtb-pl.TestPlumlDiagram.pluml.sh, rbtb-sj.TestSrjclJupyter.srjcl.sh
··x··x···································· rbtg_testbench.sh
x···················x····················· butd_dispatch.sh, buto_operations.sh
x·················x······················· launcher.rbtb_testbench.sh, rbtb-ta.TestAll.sh
x····x···································· but_test.sh
·········································x butcrg_RegimeCredentials.sh, rbw-trc.TestRegimeCredentials.sh
········································x· RBSRO-RegimeOauth.adoc, rbgu_Utility.sh, rbro_regime.sh
·······································x·· jjrwp_wrap.rs, lib.rs, vvcc_commit.rs, vvce_env.rs, vvcp_probe.rs
····································x····· RBSDC-depot_create.adoc, rbgp_Payor.sh, rbw-ral.ListAuthRegimes.sh, rbw-rar.RenderAuthRegime.sh, rbw-rav.ValidateAuthRegime.sh, rbw-ror.RenderOauthRegime.sh, rbw-rov.ValidateOauthRegime.sh, rbw-rpr.RenderPayorRegime.sh, rbw-rpv.ValidatePayorRegime.sh, rbw-rsr.RenderStationRegime.sh, rbw-rsv.ValidateStationRegime.sh
···································x······ BUSD-DispatchRuntime.adoc
··································x······· RBSRE-RegimeEcr.adoc, RBSRG-RegimeGithub.adoc, RBSRS-RegimeStation.adoc, rbre_cli.sh, rbrg_cli.sh
·································x········ RBSRA-CredentialFormat.adoc, rbra_regime.sh
································x········· RBSRP-RegimePayor.adoc
·······························x·········· rbf_cli.sh, rbga_cli.sh, rbgb_cli.sh, rbgg_cli.sh, rbgm_cli.sh, rbgo_OAuth.sh, rbgp_cli.sh, rbrp_regime.sh, rgbs_cli.sh
·····························x············ rbgm_ManualProcedures.sh, rbrr.env
························x················· RBSVC-rbv_check.adoc, RBSVM-rbv_mirror.adoc, rbp.podman.mk, rbw-Z.PodmanNuke.sh, rbw-z.PodmanStop.sh, rbw-z.Stop.nsproto.sh, rbw-z.Stop.pluml.sh, rbw-z.Stop.srjcl.sh, vme.extractor.sh
·······················x·················· RBSPV-PodmanVmSupplyChain.adoc, rbv_PodmanVM.sh
·····················x···················· buut_tabtarget.sh, jjc-pace-notch.md, launcher.rbk_Coordinator.sh, rbk_Coordinator.sh, rbw-GD.GovernorDirectorCreate.sh, rbw-GR.GovernorRetrieverCreate.sh, rbw-GS.DeleteServiceAccount.sh, rbw-Gl.ListServiceAccounts.sh, rbw-PC.PayorDepotCreate.sh, rbw-PD.PayorDepotDestroy.sh, rbw-PE.PayorEstablishment.sh, rbw-PG.PayorGovernorReset.sh, rbw-PI.PayorInstall.sh, rbw-PR.PayorRefresh.sh, rbw-QB.QuotaBuild.sh, rbw-aA.AbjureArk.sh, rbw-aC.ConjureArk.sh, rbw-ab.BeseechArk.sh, rbw-as.SummonArk.sh, rbw-iB.BuildImageRemotely.sh, rbw-iD.DeleteImage.sh, rbw-il.ImageList.sh, rbw-ir.RetrieveImage.sh, rbw-ld.ListDepots.sh, rbw-ps.ShowPayorEstablishment.sh
····················x····················· rbw-tb.TestBottles.parallel.sh, rbw-tb.TestBottles.single.sh, rbw-tf.FastTest.sh, rbw-tg.TestGithubWorkflow.sh, rbw-tn.TestNameplate.nsproto.sh, rbw-tn.TestNameplate.pluml.sh, rbw-tn.TestNameplate.srjcl.sh
···················x······················ buc_command.sh
··················x······················· rbtb-to.TestOne.sh, rbtb-ts.TestSuite.sh, rbw-ta.TestAll.sh, rbw-to.TestOne.sh, rbw-trg.TestRegimeSmoke.sh, rbw-ts.TestSuite.sh
·················x························ burc_cli.sh, burc_regime.sh, burc_specification.md, burs_cli.sh, burs_regime.sh, burs_specification.md, buw-rcr.RenderConfigRegime.sh, buw-rcv.ValidateConfigRegime.sh, buw-rgi-burc.InfoBurcRegime.sh, buw-rgi-burs.InfoBursRegime.sh, buw-rgr-burc.RenderBurcRegime.sh, buw-rgr-burs.RenderBursRegime.sh, buw-rgv-burc.ValidateBurcRegime.sh, buw-rgv-burs.ValidateBursRegime.sh, buw-rsr.RenderStationRegime.sh, buw-rsv.ValidateStationRegime.sh, buw_workbench.sh, rbcr_render.sh
···············x·························· BUS-BashUtilitiesSpec.adoc, VLS-VoxLiturgicalSpec.adoc, VOS-VoxObscuraSpec.adoc
·········x································ BCG-BashConsoleGuide.md, buv_validation.sh
······x··································· JJSCSD-saddle.adoc, jjrpd_parade.rs, jjrsd_saddle.rs, rbw-rnr.RenderNameplateRegime.sh, rbw-rnv.ValidateNameplateRegime.sh, rbw-rvr.RenderVesselRegime.sh, rbw-rvv.ValidateVesselRegime.sh
····x····································· rbtcim_ImageManagement.sh
···x······································ launcher.rbt_testbench.sh, rbtcns_NsproSecurity.sh, rbtcpl_PlumlDiagram.sh, rbtcsj_SrjclJupyter.sh, rbw-to.TestBottleService.nsproto.sh, rbw-to.TestBottleService.pluml.sh, rbw-to.TestBottleService.srjcl.sh
··x······································· butcde_DispatchExercise.sh, butctt.TestTarget.sh, launcher.rbtg_testbench.sh, rbtg-al.TEST_ONLY.trbim-macos.sh, rbtg-de.DispatchExercise.sh
·x········································ rbtcal_ArkLifecycle.sh
x········································· butr_registry.sh, rbtckk_KickTires.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 281 commits)

  1 r burd-dispatch-regime-formalization
  2 t rbro-oauth-regime-formalization
  3 u review-checkpoint-findings
  4 p audit-regime-variable-voicings
  5 q regime-tabtarget-completeness
  6 s regime-smoke-test-expansion
  7 v audit-regime-validation-gates

123456789abcdefghijklmnopqrstuvwxyz
xx·································  r  2c
···x··xx···························  t  3c
·········xx····xx··················  u  4c
············x·x····xx···x··x·······  p  6c
··················x··xxx···········  q  4c
··························x·······x  s  2c
······························x·xx·  v  3c
```

## Steeplechase

### 2026-02-16 06:13 - ₢ATAAs - W

Expanded regime-smoke suite (7 cases: BURC/BURS/RBRN+list/RBRR/RBRV+list/RBRP/BURD), created regime-credentials suite (RBRA/RBRO/RBRS with file-exists prechecks), added rbw-trc tabtarget+zipper, fixed incomplete vessel configs for rbev-pg-keeper and rbev-ubu-safety

### 2026-02-16 06:05 - ₢ATAAv - W

Audit complete: 7 gaps found (5 direct RBRA/RBRO sourcing bypassing validate, 1 direct RBRV source in rbf_Foundry, 3 abandoned CLIs with undefined burd_dispatch). All active CLIs properly gated via furnish pattern. Sentinel guards universal.

### 2026-02-16 06:05 - ₢ATAAv - n

Add regime-credentials test suite and expand regime-smoke coverage with list validation, RBRP, and BURD cases; add multi-platform conjure config to vessel envs

### 2026-02-16 05:57 - Heat - n

Clarify gcloud not a workstation dependency: remove from native tool list, note server-side Cloud Build usage only

### 2026-02-16 05:57 - ₢ATAAv - B

arm | audit-regime-validation-gates

### 2026-02-16 05:57 - Heat - T

audit-regime-validation-gates

### 2026-02-16 05:54 - Heat - S

audit-regime-validation-gates

### 2026-02-16 05:50 - ₢ATAAp - W

Full cross-regime variable voicing audit completed: 9 regimes audited, RBEV placeholder removed, 4 open items resolved (stale vars, decimal validator, RBRV vessel mode alignment)

### 2026-02-16 05:46 - ₢ATAAs - A

Expand smoke suite (RBRP+BURD+list ops), create credentials suite (RBRA+RBRO+RBRS), new trc tabtarget; skip RBRE/RBRG (no code exists)

### 2026-02-16 05:42 - Heat - r

moved ATAAs to first

### 2026-02-16 05:42 - ₢ATAAp - n

Resolve 4 open items: remove stale RBRR_PAYOR_RBRA_FILE and RBRP_DEPOT_PROJECT_IDS, add buv_env_decimal for RBRR_GCB_MIN_CONCURRENT_BUILDS, align RBRV_VESSEL_MODE with spec gated-group pattern (explicit enum replacing content-sniffing)

### 2026-02-16 05:40 - ₢ATAAq - W

Created 9 tabtargets (RBRP render/validate, RBRO render/validate, RBRS render/validate, RBRA render/validate/list), added zipper registrations, updated workbench help text, updated RBRO/RBRS CLIs to singleton defaults, added RBRA role-based dispatch

### 2026-02-16 05:40 - ₢ATAAq - n

Resolve voicing audit open items: remove dead RBRR_PAYOR_RBRA_FILE and RBRP_DEPOT_PROJECT_IDS from spec/code, add GCB_MIN_CONCURRENT_BUILDS validation, add role-based RBRA dispatch, add regime CLI tabtargets

### 2026-02-16 05:39 - ₢ATAAq - n

Migrate z1z_buz_colophon to z_buz_register_colophon per BCG FM-001

### 2026-02-16 05:27 - ₢ATAAp - L

opus landed

### 2026-02-16 05:27 - ₢ATAAp - n

Full cross-regime variable voicing audit with fixes

### 2026-02-16 05:26 - ₢ATAAq - A

Singletons RBRP/RBRO/RBRS (constants+CLI+tt+zipper), manifold RBRA (role-based dispatch+tt+zipper), help text update

### 2026-02-16 05:21 - Heat - r

moved ATAAq to first

### 2026-02-16 05:20 - ₢ATAAu - W

Investigated 4 checkpoint findings: fixed rbl_kindle_all dead calls in 3 CLIs, deleted vestigial RBRE/RBRG files, added vvce_claude_command() to prevent nested-session guard, confirmed rbrp_depot_project_ids not orphaned, deferred annotation normalization to ATAAp

### 2026-02-16 05:20 - ₢ATAAu - n

Add vvce_claude_command() to remove CLAUDECODE env var from claude subprocess invocations, preventing nested-session guard from blocking legitimate tool usage

### 2026-02-16 05:17 - ₢ATAAp - F

Executing bridled pace via opus agent

### 2026-02-16 05:16 - Heat - r

moved ATAAp to first

### 2026-02-16 05:10 - ₢ATAAp - B

arm | audit-regime-variable-voicings

### 2026-02-16 05:10 - Heat - T

audit-regime-variable-voicings

### 2026-02-16 05:01 - ₢ATAAu - A

Fix rbl_kindle_all (3 CLIs), document dispositions for findings 2-4 (defer annotation sweep, computed var OK, keep singleton)

### 2026-02-16 04:57 - ₢ATAAu - A

Sequential opus: study 4 findings - rbl_kindle_all removal, annotation style scope, orphan variable, singleton-vs-manifold decision

### 2026-02-15 08:58 - Heat - S

review-checkpoint-findings

### 2026-02-15 08:56 - ₢ATAAt - W

Created rbro_regime.sh with rbro_load, rbro_cli.sh with masked render, RBSRO subdoc, RBSA include, migrated rbgu_rbro_load to delegate

### 2026-02-15 08:56 - ₢ATAAt - n

Create rbro_regime.sh validator with rbro_load, rbro_cli.sh with masked render, RBSRO-RegimeOauth.adoc subdocument, wire RBSA include, migrate rbgu_rbro_load to delegate

### 2026-02-15 08:53 - Heat - T

regime-smoke-test-expansion

### 2026-02-15 08:53 - Heat - T

regime-tabtarget-completeness

### 2026-02-15 08:52 - ₢ATAAt - A

Create rbro_regime.sh+cli.sh, RBSRO subdoc, wire RBSA include, migrate rbgu_rbro_load

### 2026-02-15 08:50 - Heat - T

audit-regime-variable-voicings

### 2026-02-15 08:50 - ₢ATAAr - W

Added zburd_sentinel, unexpected BURD_ variable detection, BUSD-DispatchRuntime.adoc subdocument, and BUSA wiring with all 25 BURD_ variables

### 2026-02-15 08:50 - ₢ATAAr - n

Add zburd_sentinel and unexpected variable detection to bud_dispatch.sh, create BUSD-DispatchRuntime.adoc subdocument, wire BURD regime into BUSA with mappings and include

### 2026-02-15 08:45 - Heat - S

rbro-oauth-regime-formalization

### 2026-02-15 08:43 - ₢ATAAr - A

Sentinel+unexpected in bud_dispatch.sh, create BUSD subdoc, wire BUSA mappings+include

### 2026-02-15 08:40 - ₢ATAAo - W

Created 3 CLIs (rbre/rbrg/rbrs) with masked secrets, 3 subdocuments (RBSRE/RBSRG/RBSRS), wired into RBSA with mappings/prefixes/sections/includes, normalized RBRO annotation

### 2026-02-15 08:40 - ₢ATAAo - n

Created CLIs (rbre/rbrg/rbrs), subdocuments (RBSRE/RBSRG/RBSRS), wired into RBSA with mappings+prefixes+sections+includes, normalized RBRO annotation

### 2026-02-15 08:34 - ₢ATAAo - A

Parallel: 3 CLIs (rbre/rbrg/rbrs), 3 subdocs (RBSRE/RBSRG/RBSRS), RBSA wiring+RBRO normalize

### 2026-02-15 08:32 - ₢ATAAn - W

Created rbra_regime.sh validator, rbra_cli.sh with masked private key render, RBSRA-CredentialFormat.adoc subdocument, wired into RBSA with include and normalized annotation

### 2026-02-15 08:32 - ₢ATAAn - n

Created rbra_regime.sh validator, rbra_cli.sh with masked render, RBSRA-CredentialFormat.adoc subdocument, wired into RBSA with include and normalized annotation

### 2026-02-15 08:28 - Heat - S

regime-smoke-test-expansion

### 2026-02-15 08:27 - ₢ATAAn - A

Follow RBRP/RBRN pattern: regime.sh validator, cli.sh with masked render, RBSRA subdoc, RBSA wiring

### 2026-02-15 08:25 - ₢ATAAm - W

Created rbrp_cli.sh with validate/render commands, RBSRP-RegimePayor.adoc subdocument, wired into RBSA with include and axvr_regime annotation

### 2026-02-15 08:25 - ₢ATAAm - n

Create RBRP CLI and RBSRP subdocument, wire into RBSA with include and normalized annotation

### 2026-02-15 08:23 - ₢ATAAm - A

Parallel: create rbrp_cli.sh, create RBSRP subdoc, update RBSA annotation+include

### 2026-02-15 08:21 - ₢ATAAK - W

Consolidated RBRP loading via rbrp_load(), retired rbl_Locator.sh, rehomed tool checks to rbgo_OAuth.sh

### 2026-02-15 08:21 - ₢ATAAK - n

Consolidate RBRP loading via rbrp_load(), remove all rbl_Locator.sh sourcing, rehome tool checks to rbgo_OAuth.sh, delete rbl_Locator.sh

### 2026-02-15 08:18 - ₢ATAAK - A

Parallel agents: rbcc+rbrp_load, remove 5 dead imports, explore tool usage; then migrate rbgm/rbgp, rehome tool checks, delete rbl

### 2026-02-15 08:17 - Heat - n

Reviewer fixes: stale broach→kindle comments in rbrr_regime.sh, reorder list voicing after validate/render in RBSRV

### 2026-02-15 08:11 - ₢ATAAh - W

Deleted legacy RBRR-RegimeRepo.adoc, renamed zrbrr_broach→kindle, removed dead vars from RBSRR, wired RBSRR into RBSA via include, expanded RBSA mappings and definitions, updated annotation to axvr_regime style

### 2026-02-15 08:10 - ₢ATAAh - n

Align RBRR with RBRN/RBRV pattern: delete legacy RBRR-RegimeRepo.adoc, rename zrbrr_broach→kindle, remove dead vars from RBSRR, wire RBSRR into RBSA via include, expand RBSA mappings and variable definitions

### 2026-02-15 08:08 - ₢ATAAh - A

Delete legacy RBRR-RegimeRepo.adoc, update RBSRR (dead vars, broach→kindle), rename zrbrr_broach→kindle in code, expand RBSA mappings+definitions+include+annotation

### 2026-02-15 08:04 - ₢ATAAd - W

Renamed rbrr_RecipeBottleRegimeRepo.sh to rbrr.env, updated all 6 referencing files

### 2026-02-15 08:04 - ₢ATAAd - n

Rename rbrr_RecipeBottleRegimeRepo.sh to rbrr.env, update all references across codebase

### 2026-02-15 08:03 - ₢ATAAd - A

git mv rename + update 6 files with string replacement

### 2026-02-15 08:02 - ₢ATAAl - W

Extracted rbrv_list() to rbrv_regime.sh, updated CLI dispatch, renamed broach→kindle and added list voicing in RBSRV spec

### 2026-02-15 08:01 - ₢ATAAl - n

Extract rbrv_list() from inline CLI enumeration, rename broach→kindle and add list voicing in RBSRV spec

### 2026-02-15 07:59 - ₢ATAAl - A

Extract rbrv_list() following rbrn_list pattern, update CLI dispatch, rename broach→kindle and add list voicing in RBSRV spec

### 2026-02-15 07:55 - ₢ATAAk - W

Renamed axhro_broach→kindle, added list/survey/audit collection-level operation voicings to RBRN subdocument as manifold regime exemplar

### 2026-02-15 07:55 - Heat - T

external-regime-specs

### 2026-02-15 07:55 - Heat - T

rbrp-regime-spec

### 2026-02-15 07:55 - Heat - T

align-rbrr-with-rbrn-rbrv-pattern

### 2026-02-15 07:53 - ₢ATAAk - n

Rename axhro_broach→kindle, add list/survey/audit collection-level operation voicings to RBRN subdocument

### 2026-02-15 07:52 - ₢ATAAk - A

Rename axhro_broach→kindle, add list/survey/audit operation voicings to RBRN subdoc

### 2026-02-15 07:52 - ₢ATAAj - W

Renamed broach→kindle, added rbkro_list/survey/audit collection ops, annotated all regime declarations with singleton/manifold cardinality in RBSA and BUSA

### 2026-02-15 07:51 - ₢ATAAj - n

Rename broach→kindle, add collection-level operation terms, annotate all regime declarations with cardinality dimensions in RBSA and BUSA

### 2026-02-15 07:47 - ₢ATAAj - A

Parallel agents: RBSA kindle rename+cardinality+collection-ops, BUSA kindle rename+cardinality

### 2026-02-15 07:44 - ₢ATAAi - W

Renamed broach→kindle, added singleton/manifold cardinality dimensions, added list/survey/audit collection-level operation markers in AXLA

### 2026-02-15 07:44 - ₢ATAAi - n

Rename broach→kindle in AXLA, add axrd_singleton/manifold cardinality dimensions, add list/survey/audit collection-level operation markers

### 2026-02-15 07:39 - ₢ATAAi - A

Rename broach→kindle, add axrd_singleton/manifold dims, add list/survey/audit operation markers

### 2026-02-15 07:25 - Heat - S

burd-dispatch-regime-formalization

### 2026-02-15 07:21 - Heat - T

external-regime-specs

### 2026-02-15 07:20 - Heat - T

rbrp-regime-spec

### 2026-02-15 07:17 - Heat - T

align-rbrr-with-rbrn-rbrv-pattern

### 2026-02-15 07:16 - Heat - r

moved ATAAh before ATAAK

### 2026-02-15 07:16 - Heat - r

moved ATAAl after ATAAk

### 2026-02-15 07:16 - Heat - r

moved ATAAk after ATAAj

### 2026-02-15 07:16 - Heat - r

moved ATAAj after ATAAi

### 2026-02-15 07:16 - Heat - r

moved ATAAi to first

### 2026-02-15 07:16 - Heat - S

regime-tabtarget-completeness

### 2026-02-15 07:15 - Heat - S

audit-regime-variable-voicings

### 2026-02-15 07:15 - Heat - S

external-regime-specs

### 2026-02-15 07:14 - Heat - S

rbra-regime-formalization

### 2026-02-15 07:14 - Heat - S

rbrp-regime-spec

### 2026-02-15 07:13 - Heat - S

rbrv-list-extraction-and-voicings

### 2026-02-15 07:13 - Heat - S

rbrn-collection-voicings

### 2026-02-15 07:13 - Heat - S

rbsa-busa-vocabulary-alignment

### 2026-02-15 07:13 - Heat - S

axla-kindle-and-cardinality

### 2026-02-14 07:28 - ₢ATAAg - W

Deleted 9 superseded files, removed GHCR vars from regime/cli/config, cleaned RBSA with RBSPV include

### 2026-02-14 07:28 - ₢ATAAg - n

delete superseded VM files, remove GHCR vars, clean RBSA with RBSPV include, delete orphaned tabtargets

### 2026-02-14 07:24 - Heat - S

align-rbrr-with-rbrn-rbrv-pattern

### 2026-02-14 06:59 - ₢ATAAf - W

Created RBSPV-PodmanVmSupplyChain.adoc with 6 prose sections, added GHCR warning banner to rbv_PodmanVM.sh

### 2026-02-14 06:59 - ₢ATAAf - n

Refine RBSPV prose: restructure Operations into subsections, clarify Motivation narrative, expand Constraints on OCI tooling, add RBRR_GITHUB_PAT_ENV to Configuration

### 2026-02-14 06:56 - ₢ATAAf - n

create RBSPV-PodmanVmSupplyChain.adoc with 6 prose sections, add GHCR warning to rbv_PodmanVM.sh

### 2026-02-14 06:49 - ₢ATAAf - A

Read Study/ refs, create RBSPV adoc with 6 prose sections, add warning to rbv_PodmanVM.sh

### 2026-02-14 06:48 - ₢ATAAF - W

Removed 8 dead RBRR variables from RBRR-RegimeRepo.adoc, fixed RBV_GITHUB_PAT_ENV typo in rbw.workbench.mk

### 2026-02-14 06:46 - ₢ATAAF - n

remove 8 dead RBRR variables from spec, fix RBV_GITHUB_PAT_ENV typo

### 2026-02-14 06:41 - ₢ATAAF - A

Remove 7 dead var sections + ENCLAVE_SUBNET ref from RBRR adoc, fix RBV_GITHUB_PAT_ENV typo in workbench.mk

### 2026-02-14 06:39 - Heat - r

moved ATAAK after ATAAd

### 2026-02-14 06:39 - Heat - r

moved ATAAd after ATAAg

### 2026-02-14 06:37 - Heat - T

delete-ghcr-legacy-vm-files

### 2026-02-14 06:37 - Heat - T

delete-legacy-vm-files

### 2026-02-14 06:37 - Heat - T

consolidate-podman-vm-spec

### 2026-02-14 06:37 - Heat - T

consolidate-podman-vm-spec

### 2026-02-14 06:36 - Heat - T

cleanup-rbrr-dead-vm-vars

### 2026-02-14 06:16 - Heat - T

delete-legacy-vm-files

### 2026-02-14 06:16 - Heat - T

rbk-podman-vm-delete-legacy

### 2026-02-14 06:15 - Heat - T

consolidate-podman-vm-spec

### 2026-02-14 06:15 - Heat - T

rbk-podman-vm-consolidate-spec

### 2026-02-14 06:15 - Heat - T

cleanup-rbrr-dead-vm-vars

### 2026-02-14 06:14 - Heat - T

rbk-podman-vm-cleanup-rbrr

### 2026-02-14 06:11 - Heat - S

rbk-podman-vm-delete-legacy

### 2026-02-14 06:10 - Heat - S

rbk-podman-vm-consolidate-spec

### 2026-02-14 06:10 - Heat - S

rbk-podman-vm-cleanup-rbrr

### 2026-02-13 11:37 - ₢ATAAF - A

Remove 7 table entries + 1 inline ref from RBRR adoc, fix RBV_GITHUB_PAT_ENV typo in mk

### 2026-02-13 10:08 - Heat - S

rename-rbrr-regime-to-env

### 2026-02-13 10:06 - Heat - T

cleanup-rbrr-legacy-variables

### 2026-02-13 10:06 - Heat - T

cleanup-rbrr-legacy-and-rbs-fate

### 2026-02-13 10:00 - Heat - T

cleanup-rbrr-legacy-and-rbs-fate

### 2026-02-13 10:00 - Heat - T

audit-rbrr-legacy-variables

### 2026-02-13 09:43 - ₢ATAAF - A

Grep all RBRR_REGISTRY/GITHUB vars and RBGD overlaps, read RBRR spec and regime, produce live/dead inventory with consolidation proposals

### 2026-02-13 09:42 - ₢ATAAa - W

Unified dispatch: merged RBK into workbench via zbuz_exec_lookup, registered all colophons in zipper, deleted rbk_Coordinator

### 2026-02-13 09:37 - Heat - n

Refresh skopeo GCB pin after quay.io digest garbage-collected

### 2026-02-13 09:36 - ₢ATAAa - n

Revise notch slash command: don't pre-check size, ask user before overriding size guard

### 2026-02-13 09:24 - ₢ATAAa - n

Unify dispatch: merge RBK into workbench via zbuz_exec_lookup, register all colophons in zipper, delete rbk_Coordinator

### 2026-02-13 08:58 - ₢ATAAa - A

Sequential sonnet: create zbuz_exec_lookup, register missing colophons, merge RBK into workbench, convert degenerate arms, delete RBK

### 2026-02-13 08:55 - ₢ATAAb - W

Improved test discoverability: clean no-arg suite/case listings, helpful unknown-command output, deleted 7 dead tabtargets, ungated buto_info, consolidated nameplate tests into rbw-tn imprint colophon

### 2026-02-13 08:55 - ₢ATAAb - n

Consolidate per-nameplate test colophons (rbw-tns/tj/tpl) into imprinted rbw-tn colophon

### 2026-02-13 08:43 - ₢ATAAb - n

Ungate buto_info verbosity, revert butd_dispatch back to buto_info from buc_info

### 2026-02-13 08:22 - ₢ATAAb - n

Improve test discoverability: clean no-arg output for suite/case dispatch, helpful unknown-command listings, delete 4 dead legacy tabtargets, remove stale rbw-lB comment

### 2026-02-13 08:12 - ₢ATAAb - A

Delete 4 legacy tabtargets, improve no-arg UX for butd_run_suite/butd_run_one, fix unknown command listing

### 2026-02-13 08:11 - ₢ATAAc - W

Fixed dispatch stderr logging, added buc_countdown, Docker Hub rate-limit warning — cloud builds working again

### 2026-02-13 08:03 - Heat - n

Refactor: inline rbrr_refresh_gcb_pins into rbrr_cli.sh, delete rbru_update.sh, platform constants to CLI kindle

### 2026-02-13 08:01 - Heat - n

Update skopeo pin to linux/amd64 digest via platform-aware refresh

### 2026-02-13 07:50 - Heat - n

Fix GCB pin architecture: select linux/amd64 digest explicitly instead of taking first manifest entry, add kindle constants for target platform

### 2026-02-13 07:46 - Heat - n

Tighten BCG warn policy: require human comment for non-fatal error paths; remove legacy code note

### 2026-02-13 07:26 - ₢ATAAc - n

Add sleep between buc_step and /dev/tty countdown to let pipe drain

### 2026-02-13 07:26 - Heat - n

Refresh GCB image pins via rbru_refresh_gcb_pins (succeed-or-die, no fallback)

### 2026-02-13 07:21 - Heat - n

Factor rbrr_refresh_gcb_pins into rbru_update.sh: new BCG module with succeed-or-die semantics, no fallback, no partial updates

### 2026-02-13 07:08 - ₢ATAAc - n

Fix countdown buffering: write to /dev/tty, add buc_step for log visibility

### 2026-02-13 07:00 - ₢ATAAc - n

Add buc_countdown function for human-cancellable delay with yellow prompt

### 2026-02-13 06:42 - ₢ATAAc - W

Fixed stderr capture in normal dispatch path by adding 2>&1, verified logs now capture all coordinator output

### 2026-02-13 06:41 - ₢ATAAc - n

Merge stderr into stdout in normal dispatch path so all coordinator output reaches log files

### 2026-02-13 06:30 - ₢ATAAc - A

Run refresh script, retry if rate-limited, notch if pins changed

### 2026-02-12 14:18 - Heat - S

refresh-docker-hub-pins

### 2026-02-12 14:18 - ₢ATAAY - W

Routed all test tabtargets through workbench, removed buz_ compatibility shims, deleted testbench launcher

### 2026-02-12 14:18 - ₢ATAAY - n

Improve regime field rendering: colorize value (white) and description (gray with "meaning =>" label)

### 2026-02-12 14:15 - Heat - S

improve-test-discoverability

### 2026-02-12 13:50 - ₢ATAAY - n

Route test tabtargets through workbench, remove buz_ compatibility shims, delete testbench launcher

### 2026-02-12 13:46 - ₢ATAAY - L

sonnet landed

### 2026-02-12 13:41 - ₢ATAAY - F

Executing bridled pace via sonnet agent

### 2026-02-12 13:37 - ₢ATAAY - B

arm | consolidate-testbench-into-workbench

### 2026-02-12 13:37 - Heat - T

consolidate-testbench-into-workbench

### 2026-02-12 13:33 - Heat - T

consolidate-testbench-into-workbench

### 2026-02-12 13:33 - Heat - T

convert-testbench-exec-dispatch

### 2026-02-12 13:16 - ₢ATAAY - A

Sequential sonnet: design test-script pattern, prototype with kick-tires, convert remaining suites, simplify testbench, remove buz shims

### 2026-02-12 13:14 - Heat - S

unify-workbench-zipper-dispatch

### 2026-02-12 13:13 - Heat - T

convert-testbench-exec-dispatch

### 2026-02-12 13:13 - Heat - T

explore-zipper-exec-dispatch

### 2026-02-12 13:10 - Heat - n

Introduce 'kindle constant' as formal BCG vocabulary for variables defined exclusively in kindle

### 2026-02-12 12:55 - ₢ATAAY - A

Study 3 dispatch patterns: zipper+RBK, workbench case, workbench exec; assess exec-only viability

### 2026-02-12 12:54 - Heat - r

moved ATAAY to first

### 2026-02-12 12:50 - ₢ATAAC - W

Expanded BUSA regime vocabulary, upgraded BUK regime infrastructure (tabtarget renames, bupr presentation, broach/validate split, sectioned render), added regime-smoke test suite

### 2026-02-12 12:44 - ₢ATAAC - n

Add regime-smoke test suite: 5 cases covering render+validate for all BUK and RBW regimes

### 2026-02-12 12:40 - ₢ATAAC - n

Upgrade BUK regime infrastructure: rename tabtargets to RBRN-style colophons, move rbcr_render to bupr_PresentationRegime, broach/validate split, sectioned bupr_* render display

### 2026-02-12 12:31 - ₢ATAAC - n

Expand BUSA with burc_*/burs_*/bukro_* regime vocabulary, delete orphaned markdown specs + info CLI commands/tabtargets, fix CLAUDE.md BURS description

### 2026-02-12 12:04 - ₢ATAAC - A

Pattern-follow RBRV in RBSA: add burc_*/burs_* mappings+definitions to BUSA, delete orphaned markdown specs

### 2026-02-12 12:01 - ₢ATAAR - W

Implemented RBRR regime operation model: broach/validate split, rbrr_cli.sh render/validate, workbench routing, RBSA voicings, RBSRR subdoc; ark lifecycle test passes

### 2026-02-12 11:56 - ₢ATAAR - n

Add rbw-rrr/rbw-rrv workbench routing for repo regime render/validate, make rbrr_cli.sh executable

### 2026-02-12 11:37 - ₢ATAAR - A

Sequential sonnet: broach/validate split in rbrr_regime.sh, create rbrr_cli.sh with render/validate, add RBRR voicings to RBSA, create RBSRR subdoc

### 2026-02-12 11:34 - ₢ATAAZ - W

Normalized 4 .adoc files: mapping alphabetization, term isolation, Strachey bracket annotation conversion; verified cross-references AXLA→RBSA→RBSRV/RBRN

### 2026-02-12 11:12 - ₢ATAAZ - A

Sequential haiku: cma-normalize + cma-validate on 4 .adoc files, fix cross-ref issues

### 2026-02-12 11:11 - Heat - n

Add missing CLAUDE.md lens mappings: RBSA, RBSRV, RBSAB, RBSAC, RBSAS, RBSAX

### 2026-02-12 11:03 - ₢ATAAE - n

Add regime operation model: axvr_broach/validate/render voicings, axhro_* hierarchy markers, axrd_* provenance dimensions in AXLA; rbkro_/bukro_ kit operation terms in RBSA; operation markers in RBSRV and RBRN subdocs; remove invalid parent-regime back-reference constraint from axvr_broach/validate/render

### 2026-02-12 11:03 - Heat - T

implement-rbrr-regime-operation-model

### 2026-02-12 11:03 - Heat - T

align-rbrr-kindle-with-rbrn-rbrv

### 2026-02-12 11:02 - Heat - S

normalize-regime-operation-specs

### 2026-02-12 09:34 - ₢ATAAE - W

Added rbrv_binfmt_allow/forbid to RBSA; updated RBSRV subdoc with normalized content, vessel_mode enum gates; verified with ark-lifecycle test

### 2026-02-12 09:30 - ₢ATAAE - n

Add rbrv_binfmt_allow/forbid to RBSA mapping+definitions; update RBSRV subdoc with normalized content, vessel_mode enum gates, validation constraints

### 2026-02-12 09:24 - ₢ATAAE - A

Normalize RBRV, validate links, integrate into RBAGS parent, update cross-refs

### 2026-02-12 09:24 - ₢ATAAD - W

Created RBRV-RegimeVessel.adoc with explicit vessel_mode enum (bind/conjure), MCM-compliant gates on both groups

### 2026-02-12 09:24 - ₢ATAAD - n

Create RBRV-RegimeVessel.adoc following RBRN annotation pattern; sources: rbrv_regime.sh validator, rbrv_cli.sh render, inventory memo

### 2026-02-12 09:19 - ₢ATAAD - F

Executing via sonnet agent: create RBRV-RegimeVessel.adoc

### 2026-02-12 09:18 - ₢ATAAD - A

Create RBRV-RegimeVessel.adoc following RBRN annotation pattern; sources: rbrv_regime.sh validator, rbrv_cli.sh render, inventory memo

### 2026-02-12 09:16 - Heat - r

moved ATAAE after ATAAD

### 2026-02-12 09:16 - Heat - r

moved ATAAD to first

### 2026-02-12 08:27 - ₢ATAAH - W

Audited 4 items: (1) removed buv_val_* stdout echo, dropped > /dev/null suppressions; (2) refactored regime routing from workbench into self-contained CLIs; (3) blessed buc_step/echo split as BCG-correct; (4) confirmed test output discipline already resolved by ₣Ac

### 2026-02-12 08:26 - Heat - n

Interactive render review: run rbrn/rbrv renders, collect user cosmetic nudges, apply refinements

### 2026-02-12 07:47 - Heat - S

explore-zipper-exec-dispatch

### 2026-02-12 07:40 - ₢ATAAH - n

Move regime listing and path resolution from rbw_workbench into rbrn_cli/rbrv_cli; workbench becomes pure routing

### 2026-02-12 07:22 - ₢ATAAQ - W

Absorbed into ₢ATAAH — buv_val_* echo removal, suppression cleanup, test/doc updates all done there

### 2026-02-12 07:22 - ₢ATAAH - n

Remove stdout echo from buv_val_* validators (silent-on-success/die-on-failure), drop > /dev/null suppressions, update tests and BCG doc

### 2026-02-12 07:03 - ₢ATAAH - A

Audit 4 items: buv echo suppression, return-vs-exit, buc_step-vs-echo, testbench output discipline - present human decisions

### 2026-02-12 07:00 - ₢ATAAS - W

Blocker ₣Ac completed (furloughed with trophies debug deferred); original intent absorbed into ₣Ac paddock and ₢AcAAF

### 2026-02-11 08:59 - Heat - T

exercise-but-suites

### 2026-02-11 08:49 - Heat - T

redesign-butr-registry

### 2026-02-11 08:48 - Heat - T

redesign-butr-registry

### 2026-02-11 08:13 - Heat - S

fix-subdispatch-logging

### 2026-02-11 07:55 - Heat - S

design-test-preflight-gate

### 2026-02-11 07:42 - Heat - S

bcg-kindle-comm-cleanup

### 2026-02-11 07:41 - Heat - S

test-cmd-list-targets

### 2026-02-11 07:32 - Heat - S

split-buto-engine

### 2026-02-10 13:38 - ₢ATAAL - W

Converted tbvu_suite_xname.sh and trbim_suite.sh to BUT framework (butcvu_XnameValidation.sh + rbtcim_ImageManagement.sh), registered in rbtb, retired old files

### 2026-02-10 13:38 - ₢ATAAL - n

Convert xname-validation and image-management test suites to BUT framework case files, register in rbtb testbench, retire old standalone runners

### 2026-02-10 13:38 - Heat - S

exercise-but-suites

### 2026-02-10 13:33 - ₢ATAAL - A

Convert 2 remaining test suites (tbvu_xname, trbim) to BUT framework, register in rbtb, retire old files

### 2026-02-10 13:31 - ₢ATAAM - W

Converted rbt_testbench.sh suites (nsproto/srjcl/pluml) to BUT framework case files, migrated shared helpers to rbtb, created proper tabtargets via buw-tt-cbl, retired rbt_testbench

### 2026-02-10 13:31 - ₢ATAAM - n

Mechanical conversion: 3 case files (rbtcns/rbtcsj/rbtcpl), shared helpers to rbtb, register suites, retire rbt_testbench

### 2026-02-10 13:21 - ₢ATAAM - A

Mechanical conversion: 3 case files (rbtcns/rbtcsj/rbtcpl), shared helpers to rbtb, register suites, retire rbt_testbench

### 2026-02-10 13:19 - ₢ATAAN - W

Converted dispatch_exercise to BUT framework (butcde_ + butctt tabtarget), registered in rbtb, retired rbtg entirely

### 2026-02-10 13:19 - ₢ATAAN - n

Create butctt tabtarget and butcde dispatch exercise test cases, register in rbtb testbench, retire rbtg testbench and its tabtargets entirely

### 2026-02-10 13:13 - ₢ATAAN - A

Mechanical conversion: create butctt tabtarget + butcde test cases, register in rbtb, retire rbtg entirely

### 2026-02-10 13:08 - Heat - S

align-rbrr-kindle-with-rbrn-rbrv

### 2026-02-10 13:08 - ₢ATAAO - W

Converted rbtg_case_ark_lifecycle to BUT framework: rbtcal_ArkLifecycle.sh + registration in rbtb_testbench.sh

### 2026-02-10 13:08 - ₢ATAAO - n

Mechanical translation of rbtg_case_ark_lifecycle to BUT framework: rbtcal_ArkLifecycle.sh + registration in rbtb_testbench.sh

### 2026-02-10 13:07 - Heat - S

study-buv-env-stdout-pattern

### 2026-02-10 13:01 - ₢ATAAO - A

Mechanical translation of rbtg_case_ark_lifecycle to BUT framework: rbtcal_ArkLifecycle.sh + registration in rbtb_testbench.sh

### 2026-02-10 12:57 - ₢ATAAP - W

Created buto/butr/butd framework, rbtb testbench with kick-tires, compatibility shims for but_test.sh and buz_zipper.sh, trimmed buz dispatch

### 2026-02-10 12:57 - ₢ATAAP - n

Extract BUK test ops into buto_operations, add butr_registry and butd_dispatch modules, shim but_test.sh and buz_zipper.sh for backward compat, wire RBTB testbench with kick-tires suite and tabtarget

### 2026-02-10 12:42 - Heat - T

convert-dispatch-exercise

### 2026-02-10 12:38 - ₢ATAAP - A

7-step sequential: buto_ops→butr_registry→butd_dispatch→rbtb_skeleton→rbtckk_kick→trim_buz→shim_but

### 2026-02-10 12:36 - Heat - S

but-framework-infrastructure

### 2026-02-10 12:35 - Heat - S

convert-ark-lifecycle

### 2026-02-10 12:34 - Heat - S

convert-dispatch-exercise

### 2026-02-10 12:34 - Heat - S

convert-rbt-suites

### 2026-02-10 12:33 - Heat - S

convert-remaining-testbenches

### 2026-02-10 12:32 - Heat - T

extract-but-dispatch

### 2026-02-10 10:52 - Heat - S

consolidate-rbrp-retire-rbl

### 2026-02-10 10:48 - ₢ATAAJ - B

arm | extract-but-dispatch

### 2026-02-10 10:48 - Heat - T

extract-but-dispatch

### 2026-02-10 10:46 - Heat - T

audit-regime-validation-consumers

### 2026-02-10 10:45 - Heat - S

extract-but-dispatch

### 2026-02-10 10:25 - ₢ATAAH - n

Rename rbrn_load to rbrn_load_moniker for clarity alongside rbrn_load_file

### 2026-02-10 10:19 - Heat - T

audit-regime-validation-consumers

### 2026-02-10 09:53 - ₢ATAAI - W

Reverted rbtg_testbench.sh from but_tt_expect_ok to buz_dispatch+assertion pattern, removed BURC_TABTARGET_DIR fallback in but_test.sh, removed stray blank line in buz_zipper.sh

### 2026-02-10 09:53 - ₢ATAAI - n

Fix but_test refactoring: revert rbtg to buz_dispatch+assertion, remove BURC_TABTARGET_DIR fallback, remove stray blank line

### 2026-02-10 09:45 - ₢ATAAI - A

Revert rbtg to buz_dispatch+assertion, remove BURC_TABTARGET_DIR fallback, remove stray blank line

### 2026-02-10 09:44 - Heat - T

refactor-but-test-families

### 2026-02-10 09:32 - ₢ATAAI - n

Refactor BUT test families: rename but_expect to but_unit_expect, add but_tt and but_launch families, remove buz_dispatch_expect from zipper, update all testbench callers

### 2026-02-10 09:28 - ₢ATAAI - F

Executing bridled pace via sonnet agent

### 2026-02-10 09:26 - ₢ATAAI - B

arm | refactor-but-test-families

### 2026-02-10 09:26 - Heat - T

refactor-but-test-families

### 2026-02-10 09:23 - Heat - T

refactor-but-test-families

### 2026-02-10 09:20 - Heat - S

refactor-but-test-families

### 2026-02-10 02:32 - Heat - n

Fix RBRN type voicings in RBSA: runtime rbst_xname→axt_enumeration with enum values, consecrations rbst_xname→new rbst_consecration type, update subdoc to reference enum terms

### 2026-02-10 02:06 - ₢ATAAG - W

pace complete

### 2026-02-10 02:06 - ₢ATAAG - n

Add bitmap/swimlane displays to saddle output: promote parade rendering functions to pub(crate), call from saddle after recent-work, update spec with new section documentation

### 2026-02-09 16:54 - ₢ATAAG - n

Fix render valid message ordering: use stdout echo instead of stderr buc_step to avoid BUD pipe buffering reorder

### 2026-02-09 16:54 - Heat - S

audit-regime-validation-consumers

### 2026-02-09 16:46 - ₢ATAAG - n

Fix rbrv_cli: source RBRR before vessel files (conjure-mode support), suppress buv_env echo output in validate_fields calls for both CLI files

### 2026-02-09 16:37 - ₢ATAAG - n

Refactor RBRV/RBRN: separate kindle (state prep) from validate (strict buv_env checking), render as diagnostic display with validate-at-end, remove info, add regime tabtargets rbw-r{n,v}{r,v}, update programmatic consumers

### 2026-02-09 16:28 - ₢ATAAG - A

Three-layer implementation: refactor kindle (parallel), refactor CLI (parallel), tabtargets+routing. Consumer update needed for rbob_cli zrbob_furnish.

### 2026-02-09 16:27 - Heat - T

refactor-rbrv-rbrn-render-validate

### 2026-02-09 16:27 - Heat - T

synchronize-rbrv-rbrn-bash-and-tabtargets

### 2026-02-09 15:32 - ₢ATAAG - A

Audit found 3 missing tabtargets (pure oversight - infra already supports all services). CLI patterns already consistent. Test asymmetry is intentional (different services test different things).

### 2026-02-09 15:21 - Heat - S

synchronize-rbrv-rbrn-bash-and-tabtargets

### 2026-02-09 10:08 - ₢ATAAA - W

pace complete

### 2026-02-09 10:08 - Heat - d

paddock curried (refine)

### 2026-02-09 10:05 - ₢ATAAA - n

Synthesize regime inventory memo from validator source code and docket research

### 2026-02-09 10:02 - ₢ATAAA - A

Synthesize regime inventory memo from validator source code and docket research

### 2026-02-09 06:33 - ₢ATAAA - A

Synthesize regime inventory memo from docket research data, following prior art memo format

### 2026-02-08 16:17 - Heat - T

study-all-recipe-bottle-regimes

### 2026-02-08 16:13 - ₢ATAAA - A

Sequential sonnet research: enumerate all regime prefixes, read specs, trace lifecycles, map dependencies, produce inventory document

### 2026-02-08 15:48 - ₢ATAAB - W

pace complete

### 2026-02-08 15:47 - ₢ATAAB - n

Remove old BUS-BashUtilitiesSpec.adoc (renamed to BUSA)

### 2026-02-08 15:47 - ₢ATAAB - n

Rename BUS-BashUtilitiesSpec.adoc to BUSA-BashUtilitiesSpec.adoc, update CLAUDE.md mapping and references in VOS and VLS specs

### 2026-02-08 15:44 - Heat - f

racing

### 2026-02-01 21:00 - Heat - r

moved ATAAB before ATAAA

### 2026-02-01 20:59 - Heat - T

create-burc-regime-spec

### 2026-02-01 20:58 - Heat - T

create-burs-regime-spec

### 2026-02-01 19:53 - Heat - S

audit-rbrr-legacy-variables

### 2026-01-31 12:28 - Heat - S

process-rbrv-regime-vessel-spec

### 2026-01-31 12:02 - Heat - S

create-rbvr-regime-spec

### 2026-01-31 11:59 - Heat - T

create-burs-regime-spec

### 2026-01-31 11:58 - Heat - T

create-burs-regime-spec

### 2026-01-31 11:56 - Heat - D

restring 1 paces from ₣AS

### 2026-01-31 11:54 - Heat - S

create-burs-regime-spec

### 2026-01-31 11:54 - Heat - S

study-all-recipe-bottle-regimes

### 2026-01-31 11:54 - Heat - N

rbw-regime-consolidation

