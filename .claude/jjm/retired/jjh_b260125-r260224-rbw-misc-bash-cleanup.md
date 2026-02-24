# Heat Trophy: rbw-misc-bash-cleanup

**Firemark:** ₣AO
**Created:** 260125
**Retired:** 260224
**Status:** retired

## Paddock

# Paddock: rbm-misc-bash-cleanup

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### bcg-test-execution-vocabulary (₢AOAAP) [complete]

**[260217-1811] complete**

Add test execution vocabulary to BCG with the same rigor as kindle/sentinel and enroll/recite.

## Problem

BCG formalizes production bash patterns into named vocabulary with explicit contracts — kindle/sentinel for module initialization, enroll/recite for registries, capture for return values. Test execution has no equivalent formalization. The test infrastructure (bute, butd, butr, buto) works but its contracts are implicit, discoverable only by reading code.

In bash 3.2, isolation is discipline, not syntax. Discipline needs vocabulary to be teachable and auditable.

## Vocabulary to Define

**_tsuite** — the suite isolation boundary:
- Runs in a subshell (state dies at boundary)
- Setup runs inside (visible to cases, dies with suite)
- Init/precondition runs outside (parent decides whether to enter)
- Communicates only exit status to the runner
- Kindle guards catch double-kindle within a suite; subshell boundary prevents cross-suite contamination

**_tcase** — the verification unit:
- Runs in a subshell within the suite subshell
- Inherits setup state, cannot mutate sibling state
- Communicates only exit status and stdio
- Each case gets isolated temp dir and BURV root

**Three-layer model with two subshell boundaries:**
- Parent shell: init, precondition, suite iteration, reporting
- Suite subshell: setup, kindle, module sourcing, case iteration
- Case subshell: verification, assertions, tabtarget invocation

## Scope

- Add a "Test Execution Patterns" section to BCG
- Define _tsuite and _tcase as BCG vocabulary alongside kindle/sentinel
- Document what's allowed at each layer
- Reference the subshell prohibition section — this is the principled counterpoint
- Keep it tight: contracts and rules, not tutorial

## Acceptance

- BCG has a test execution section with _tsuite and _tcase vocabulary
- The three-layer isolation model is documented with allowed operations per layer
- Subshell prohibition section references the test isolation section (and vice versa)
- A developer reading BCG understands why suites and cases use subshells without reading implementation code

**[260217-1752] rough**

Add test execution vocabulary to BCG with the same rigor as kindle/sentinel and enroll/recite.

## Problem

BCG formalizes production bash patterns into named vocabulary with explicit contracts — kindle/sentinel for module initialization, enroll/recite for registries, capture for return values. Test execution has no equivalent formalization. The test infrastructure (bute, butd, butr, buto) works but its contracts are implicit, discoverable only by reading code.

In bash 3.2, isolation is discipline, not syntax. Discipline needs vocabulary to be teachable and auditable.

## Vocabulary to Define

**_tsuite** — the suite isolation boundary:
- Runs in a subshell (state dies at boundary)
- Setup runs inside (visible to cases, dies with suite)
- Init/precondition runs outside (parent decides whether to enter)
- Communicates only exit status to the runner
- Kindle guards catch double-kindle within a suite; subshell boundary prevents cross-suite contamination

**_tcase** — the verification unit:
- Runs in a subshell within the suite subshell
- Inherits setup state, cannot mutate sibling state
- Communicates only exit status and stdio
- Each case gets isolated temp dir and BURV root

**Three-layer model with two subshell boundaries:**
- Parent shell: init, precondition, suite iteration, reporting
- Suite subshell: setup, kindle, module sourcing, case iteration
- Case subshell: verification, assertions, tabtarget invocation

## Scope

- Add a "Test Execution Patterns" section to BCG
- Define _tsuite and _tcase as BCG vocabulary alongside kindle/sentinel
- Document what's allowed at each layer
- Reference the subshell prohibition section — this is the principled counterpoint
- Keep it tight: contracts and rules, not tutorial

## Acceptance

- BCG has a test execution section with _tsuite and _tcase vocabulary
- The three-layer isolation model is documented with allowed operations per layer
- Subshell prohibition section references the test isolation section (and vice versa)
- A developer reading BCG understands why suites and cases use subshells without reading implementation code

### rename-test-functions-bcg-vocabulary (₢AOAAS) [complete]

**[260217-1824] complete**

Rename existing test infrastructure functions to follow BCG test execution vocabulary naming conventions.

## Problem

BCG now defines `_tsuite_init`, `_tsuite_setup`, and `_tcase` as named vocabulary with grep-able naming conventions. Existing test functions predate these conventions and use ad-hoc names that don't support compliance checking.

## Scope

**Setup functions** (in `Tools/rbw/rbtb_testbench.sh`):
Rename `zrbtb_setup_«name»` → `zrbtb_«name»_tsuite_setup` (10 functions):
- `zrbtb_setup_kick` → `zrbtb_kick_tsuite_setup`
- `zrbtb_setup_qualify` → `zrbtb_qualify_tsuite_setup`
- `zrbtb_setup_ark` → `zrbtb_ark_tsuite_setup`
- `zrbtb_setup_dispatch` → `zrbtb_dispatch_tsuite_setup`
- `zrbtb_setup_nsproto` → `zrbtb_nsproto_tsuite_setup`
- `zrbtb_setup_srjcl` → `zrbtb_srjcl_tsuite_setup`
- `zrbtb_setup_pluml` → `zrbtb_pluml_tsuite_setup`
- `zrbtb_setup_xname` → `zrbtb_xname_tsuite_setup`
- `zrbtb_setup_regime` → `zrbtb_regime_tsuite_setup`
- `zrbtb_setup_credentials` → `zrbtb_credentials_tsuite_setup`

**Case functions** (~50 functions across ~10 files):
Append `_tcase` suffix to all case functions. Files:
- `Tools/rbw/rbtckk_KickTires.sh` — `rbtckk_false`, `rbtckk_true`
- `Tools/rbw/rbtcqa_QualifyAll.sh` — `rbtcqa_qualify_all`
- `Tools/rbw/rbtcal_ArkLifecycle.sh` — `rbtcal_lifecycle`
- `Tools/buk/butcde_DispatchExercise.sh` — `butcde_burv_isolation`, `butcde_evidence_created`, `butcde_exit_capture`
- `Tools/rbw/rbtcns_NsproSecurity.sh` — all `rbtcns_*` functions
- `Tools/rbw/rbtcsj_SrjclJupyter.sh` — all `rbtcsj_*` functions
- `Tools/rbw/rbtcpl_PlumlDiagram.sh` — all `rbtcpl_*` functions
- `Tools/buk/butcrg_RegimeSmoke.sh` — all `butcrg_*` case functions
- `Tools/buk/butcrg_RegimeCredentials.sh` — all `butcrg_*` case functions
- `Tools/buk/butcvu_XnameValidation.sh` — all `butcvu_*` case functions

**Infrastructure** (`Tools/buk/bute_engine.sh`):
- `zbute_case` → `zbute_tcase`

**Enrollment sites** (`Tools/rbw/rbtb_testbench.sh`):
- All `butr_suite_enroll` calls: update setup function references
- All `butr_case_enroll` calls: update case function references

## Acceptance

- All setup functions use `_tsuite_setup` suffix
- All case functions use `_tcase` suffix
- Infrastructure boundary function uses `zbute_tcase`
- `grep -rn '_tsuite_setup' Tools/` returns all setup functions
- `grep -rn '_tcase' Tools/` returns all case functions
- Test suite runs clean: `./tt/rbw-ta.TestAll.sh` passes (kick-tires + regime-smoke at minimum)

**[260217-1810] rough**

Rename existing test infrastructure functions to follow BCG test execution vocabulary naming conventions.

## Problem

BCG now defines `_tsuite_init`, `_tsuite_setup`, and `_tcase` as named vocabulary with grep-able naming conventions. Existing test functions predate these conventions and use ad-hoc names that don't support compliance checking.

## Scope

**Setup functions** (in `Tools/rbw/rbtb_testbench.sh`):
Rename `zrbtb_setup_«name»` → `zrbtb_«name»_tsuite_setup` (10 functions):
- `zrbtb_setup_kick` → `zrbtb_kick_tsuite_setup`
- `zrbtb_setup_qualify` → `zrbtb_qualify_tsuite_setup`
- `zrbtb_setup_ark` → `zrbtb_ark_tsuite_setup`
- `zrbtb_setup_dispatch` → `zrbtb_dispatch_tsuite_setup`
- `zrbtb_setup_nsproto` → `zrbtb_nsproto_tsuite_setup`
- `zrbtb_setup_srjcl` → `zrbtb_srjcl_tsuite_setup`
- `zrbtb_setup_pluml` → `zrbtb_pluml_tsuite_setup`
- `zrbtb_setup_xname` → `zrbtb_xname_tsuite_setup`
- `zrbtb_setup_regime` → `zrbtb_regime_tsuite_setup`
- `zrbtb_setup_credentials` → `zrbtb_credentials_tsuite_setup`

**Case functions** (~50 functions across ~10 files):
Append `_tcase` suffix to all case functions. Files:
- `Tools/rbw/rbtckk_KickTires.sh` — `rbtckk_false`, `rbtckk_true`
- `Tools/rbw/rbtcqa_QualifyAll.sh` — `rbtcqa_qualify_all`
- `Tools/rbw/rbtcal_ArkLifecycle.sh` — `rbtcal_lifecycle`
- `Tools/buk/butcde_DispatchExercise.sh` — `butcde_burv_isolation`, `butcde_evidence_created`, `butcde_exit_capture`
- `Tools/rbw/rbtcns_NsproSecurity.sh` — all `rbtcns_*` functions
- `Tools/rbw/rbtcsj_SrjclJupyter.sh` — all `rbtcsj_*` functions
- `Tools/rbw/rbtcpl_PlumlDiagram.sh` — all `rbtcpl_*` functions
- `Tools/buk/butcrg_RegimeSmoke.sh` — all `butcrg_*` case functions
- `Tools/buk/butcrg_RegimeCredentials.sh` — all `butcrg_*` case functions
- `Tools/buk/butcvu_XnameValidation.sh` — all `butcvu_*` case functions

**Infrastructure** (`Tools/buk/bute_engine.sh`):
- `zbute_case` → `zbute_tcase`

**Enrollment sites** (`Tools/rbw/rbtb_testbench.sh`):
- All `butr_suite_enroll` calls: update setup function references
- All `butr_case_enroll` calls: update case function references

## Acceptance

- All setup functions use `_tsuite_setup` suffix
- All case functions use `_tcase` suffix
- Infrastructure boundary function uses `zbute_tcase`
- `grep -rn '_tsuite_setup' Tools/` returns all setup functions
- `grep -rn '_tcase' Tools/` returns all case functions
- Test suite runs clean: `./tt/rbw-ta.TestAll.sh` passes (kick-tires + regime-smoke at minimum)

### audit-but-against-test-vocab (₢AOAAQ) [complete]

**[260217-1833] complete**

Audit the BUT test infrastructure (bute/butd/butr/buto) against the new BCG test execution vocabulary.

## Problem

The BCG test vocabulary (₢AOAAP) defines _tsuite and _tcase contracts. The existing test code predates this formalization. Need to verify conformance and fix any violations.

## Scope

Review these files against the new vocabulary:
- `Tools/buk/bute_engine.sh` — case runner, dispatch engine
- `Tools/buk/butd_dispatch.sh` — suite runner (now with subshell isolation)
- `Tools/buk/butr_registry.sh` — suite/case enrollment
- `Tools/buk/buto_operations.sh` — test operations (invoke, assertions)
- `Tools/buk/but_test.sh` — test utilities

Check for:
1. **Boundary violations** — does anything leak across suite or case subshell boundaries?
2. **Contract violations** — does anything communicate outside exit-status/stdio from a case?
3. **Naming conformance** — do function names follow the vocabulary patterns?
4. **Documentation** — do file headers and function comments use the new vocabulary?

## Acceptance

- Each BUT file reviewed against BCG test vocabulary
- Any violations documented and fixed
- Function comments updated to use _tsuite/_tcase vocabulary where appropriate
- No state leaks between suites or between cases confirmed by TestAll

**[260217-1752] rough**

Audit the BUT test infrastructure (bute/butd/butr/buto) against the new BCG test execution vocabulary.

## Problem

The BCG test vocabulary (₢AOAAP) defines _tsuite and _tcase contracts. The existing test code predates this formalization. Need to verify conformance and fix any violations.

## Scope

Review these files against the new vocabulary:
- `Tools/buk/bute_engine.sh` — case runner, dispatch engine
- `Tools/buk/butd_dispatch.sh` — suite runner (now with subshell isolation)
- `Tools/buk/butr_registry.sh` — suite/case enrollment
- `Tools/buk/buto_operations.sh` — test operations (invoke, assertions)
- `Tools/buk/but_test.sh` — test utilities

Check for:
1. **Boundary violations** — does anything leak across suite or case subshell boundaries?
2. **Contract violations** — does anything communicate outside exit-status/stdio from a case?
3. **Naming conformance** — do function names follow the vocabulary patterns?
4. **Documentation** — do file headers and function comments use the new vocabulary?

## Acceptance

- Each BUT file reviewed against BCG test vocabulary
- Any violations documented and fixed
- Function comments updated to use _tsuite/_tcase vocabulary where appropriate
- No state leaks between suites or between cases confirmed by TestAll

### repair-tabtarget-prescribed-form (₢AOAAI) [complete]

**[260217-0740] complete**

Resolve remaining 23 non-standard tabtargets that fail qualification.

## Done

- Generator fixed (buut_tabtarget.sh line 78: dirname → parameter expansion)
- 79 standard tabtargets regenerated via creator tabtargets
- Qualification down from 103/103 failures to 23/103

## Remaining (23 failures)

Decide disposition for each category:

### Dead legacy (2) — reference missing Tools/tabtarget-dispatch.sh
- `oga.OpenGithubAction.sh`
- `machine_setup_PROTOTYPE_rule.sh`
→ Delete?

### Unregistered rbw-* colophons (3) — not in rbz_zipper.sh
- `rbw-hga.HelpGoogleAdmin.sh`
- `rbw-him.HelpImageManagement.sh`
- `rbw-l.ListCurrentRegistryImages.sh`
→ Delete or register in zipper?

### Kit tabtargets with no zipper (15) — kits lack zipper infrastructure
- ccck-* (6): StartContainer, BuildContainer, ConnectCode, ConnectGitStatus, ResetContainer, StopContainer
- cmk-* (2): Install, Uninstall
- gad* (3): LaunchFactory, Factory, Inspect
- jja-* (3): Check, Install, Uninstall
- vslk-i (1): InstallSlickEditProject
→ Convert to BURD_LAUNCHER form? Leave as-is? Create kit zippers?

### Infrastructure/special (3)
- `vow-r.RunVVX.sh` — pre-BURD form, launcher exists
- `vvk-T.RunTests.sh` — full script, not a dispatcher
- `butctt.TestTarget.sh` — BUK test target
→ Convert to BURD_LAUNCHER form?

## Acceptance

- `tt/rbw-qa.QualifyAll.sh` passes (zero failures)
- All decisions documented

**[260217-0720] rough**

Resolve remaining 23 non-standard tabtargets that fail qualification.

## Done

- Generator fixed (buut_tabtarget.sh line 78: dirname → parameter expansion)
- 79 standard tabtargets regenerated via creator tabtargets
- Qualification down from 103/103 failures to 23/103

## Remaining (23 failures)

Decide disposition for each category:

### Dead legacy (2) — reference missing Tools/tabtarget-dispatch.sh
- `oga.OpenGithubAction.sh`
- `machine_setup_PROTOTYPE_rule.sh`
→ Delete?

### Unregistered rbw-* colophons (3) — not in rbz_zipper.sh
- `rbw-hga.HelpGoogleAdmin.sh`
- `rbw-him.HelpImageManagement.sh`
- `rbw-l.ListCurrentRegistryImages.sh`
→ Delete or register in zipper?

### Kit tabtargets with no zipper (15) — kits lack zipper infrastructure
- ccck-* (6): StartContainer, BuildContainer, ConnectCode, ConnectGitStatus, ResetContainer, StopContainer
- cmk-* (2): Install, Uninstall
- gad* (3): LaunchFactory, Factory, Inspect
- jja-* (3): Check, Install, Uninstall
- vslk-i (1): InstallSlickEditProject
→ Convert to BURD_LAUNCHER form? Leave as-is? Create kit zippers?

### Infrastructure/special (3)
- `vow-r.RunVVX.sh` — pre-BURD form, launcher exists
- `vvk-T.RunTests.sh` — full script, not a dispatcher
- `butctt.TestTarget.sh` — BUK test target
→ Convert to BURD_LAUNCHER form?

## Acceptance

- `tt/rbw-qa.QualifyAll.sh` passes (zero failures)
- All decisions documented

**[260217-0702] rough**

Regenerate all tabtargets to match the prescribed form enforced by buv_qualify_tabtargets.

## Context

The qualifier now enforces strict tabtarget structure:
- Line 1: `#!/bin/bash`
- Line 2: `export BURD_LAUNCHER="<path>"`
- Middle lines: optional `export BURD_*=*` flags
- Last line: `exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"`

All ~80 existing tabtargets use `$(dirname ...)` instead of parameter expansion in the exec line. Several use hardcoded launcher paths instead of BURD_LAUNCHER. Two legacy tabtargets (oga, machine_setup_PROTOTYPE_rule) use entirely different patterns.

## Work

1. Fix the generator: `Tools/buk/buut_tabtarget.sh` line 78 — change `$(dirname "${BASH_SOURCE[0]}")` to `${BASH_SOURCE[0]%/*}` in the emitted exec line.

2. Classify all tabtargets by launcher path and mode (batch-logging, batch-nolog, interactive-logging, interactive-nolog) by inspecting their current BURD_* exports.

3. Regenerate each group using the appropriate creator tabtarget:
   - `tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh <launcher> <name1> <name2> ...`
   - `tt/buw-tt-cbn.CreateTabTargetBatchNolog.sh <launcher> <name1> <name2> ...`
   - `tt/buw-tt-cil.CreateTabTargetInteractiveLogging.sh <launcher> <name1> <name2> ...`
   - `tt/buw-tt-cin.CreateTabTargetInteractiveNolog.sh <launcher> <name1> <name2> ...`

4. The four creator tabtargets themselves must also be regenerated (they currently use dirname). Bootstrap: fix the generator first, then regenerate the creators, then use them for everything else.

5. Handle legacy tabtargets (oga.OpenGithubAction.sh, machine_setup_PROTOTYPE_rule.sh) — convert to BURD_LAUNCHER form or remove if obsolete.

6. Run `tt/rbw-qa.QualifyAll.sh` to verify all tabtargets pass.

## Acceptance

- `tt/rbw-qa.QualifyAll.sh` passes (zero tabtarget failures)
- No `dirname` or `basename` in any tabtarget
- All tabtargets use BURD_LAUNCHER pattern

### extract-orphaned-heat-paces (₢AOAAA) [complete]

**[260216-1034] complete**

Extract paces from legacy orphaned heat file `.claude/jjm/current/jjh_b251226-bash-tooling-cleanup.md` into current Job Jockey structure. This file predates the firemark system and contains unfinished work from the Dec 26, 2025 Recipe Bottle session. Review content, migrate any relevant incomplete paces to appropriate heats, then archive or delete the orphaned file.

**[260125-0828] rough**

Extract paces from legacy orphaned heat file `.claude/jjm/current/jjh_b251226-bash-tooling-cleanup.md` into current Job Jockey structure. This file predates the firemark system and contains unfinished work from the Dec 26, 2025 Recipe Bottle session. Review content, migrate any relevant incomplete paces to appropriate heats, then archive or delete the orphaned file.

### refresh-register-colophons (₢AOAAC) [complete]

**[260216-1046] complete**

Refresh all workbench _register function implementations against the updated BCG (BashConsoleGuide) document, which has
 been significantly improved regarding colophon registration patterns. Read the current BCG spec, understand the
updated conventions for _register functions, then audit and update each workbench's _register function to conform.

**[260211-1406] rough**

Refresh all workbench _register function implementations against the updated BCG (BashConsoleGuide) document, which has
 been significantly improved regarding colophon registration patterns. Read the current BCG spec, understand the
updated conventions for _register functions, then audit and update each workbench's _register function to conform.

### detect-macos-app-permissions (₢AOAAD) [abandoned]

**[260216-1039] abandoned**

Detect missing macOS app-data-access permissions at runtime and fail gracefully instead of hanging.

## Context

During ₣AP pace ₢APAAo testing, `docker manifest inspect` hung indefinitely because macOS presented an invisible modal dialog: "iTerm would like to access data from other apps." The dialog blocks the Docker CLI socket connection with no timeout. This was encountered when running `./tt/rbw-rrg.RefreshGcbPins.sh` — the transcript showed it stuck on "Inspecting gcr.io/go-containerregistry/gcrane:latest" after the oras discovery phase completed normally (oras uses curl, not Docker socket).

The popup is macOS's TCC (Transparency, Consent, and Control) system. It appears when iTerm (or any terminal app) first tries to access another app's data — in this case, the Docker/Podman socket or container runtime data. Once "Allow" is clicked, the permission is cached in TCC.db and won't appear again. But if it's never been granted, any Docker CLI command will hang waiting for the modal.

## Problem

- `docker manifest inspect` hangs forever waiting for TCC consent dialog
- No timeout, no error message — just silence
- Tests and tabtargets that use Docker commands become "stuck" with no diagnostic
- The dialog may not be visible if the terminal is not in focus

## Work

1. Research how to detect TCC permission state programmatically on macOS (tccutil, sqlite3 on TCC.db, or behavioral probe)
2. Design a lightweight pre-flight check: attempt a trivial Docker command with a short timeout (e.g., `timeout 5 docker version`)
3. If the probe times out, print a clear diagnostic: "Docker CLI is not responding. If macOS prompted 'iTerm would like to access data from other apps', click Allow and retry."
4. Integrate the check into buc_countdown or a new buc_preflight utility so tabtargets that need Docker fail fast with an actionable message
5. Mark test results as INCONCLUSIVE rather than hanging when this condition is detected

## Acceptance

- Docker-dependent tabtargets fail within 10 seconds with a clear message when TCC permission is missing
- No hang, no silent failure
- Works on macOS (skip check gracefully on Linux)

**[260216-1009] rough**

Detect missing macOS app-data-access permissions at runtime and fail gracefully instead of hanging.

## Context

During ₣AP pace ₢APAAo testing, `docker manifest inspect` hung indefinitely because macOS presented an invisible modal dialog: "iTerm would like to access data from other apps." The dialog blocks the Docker CLI socket connection with no timeout. This was encountered when running `./tt/rbw-rrg.RefreshGcbPins.sh` — the transcript showed it stuck on "Inspecting gcr.io/go-containerregistry/gcrane:latest" after the oras discovery phase completed normally (oras uses curl, not Docker socket).

The popup is macOS's TCC (Transparency, Consent, and Control) system. It appears when iTerm (or any terminal app) first tries to access another app's data — in this case, the Docker/Podman socket or container runtime data. Once "Allow" is clicked, the permission is cached in TCC.db and won't appear again. But if it's never been granted, any Docker CLI command will hang waiting for the modal.

## Problem

- `docker manifest inspect` hangs forever waiting for TCC consent dialog
- No timeout, no error message — just silence
- Tests and tabtargets that use Docker commands become "stuck" with no diagnostic
- The dialog may not be visible if the terminal is not in focus

## Work

1. Research how to detect TCC permission state programmatically on macOS (tccutil, sqlite3 on TCC.db, or behavioral probe)
2. Design a lightweight pre-flight check: attempt a trivial Docker command with a short timeout (e.g., `timeout 5 docker version`)
3. If the probe times out, print a clear diagnostic: "Docker CLI is not responding. If macOS prompted 'iTerm would like to access data from other apps', click Allow and retry."
4. Integrate the check into buc_countdown or a new buc_preflight utility so tabtargets that need Docker fail fast with an actionable message
5. Mark test results as INCONCLUSIVE rather than hanging when this condition is detected

## Acceptance

- Docker-dependent tabtargets fail within 10 seconds with a clear message when TCC permission is missing
- No hang, no silent failure
- Works on macOS (skip check gracefully on Linux)

### buc-countdown-bypass-flag (₢AOAAE) [complete]

**[260217-0645] complete**

Add a parameter to buc_countdown that allows callers to bypass the interactive countdown, enabling Claude to run these tabtargets directly.

## Context

During ₣AP pace ₢APAAo, Claude could not run `./tt/rbw-rrg.RefreshGcbPins.sh` because buc_countdown reads from /dev/tty for the cancel window, which fails in non-interactive contexts (Claude's Bash tool, CI pipelines, etc.). The user had to run it manually in their terminal.

buc_countdown is in Tools/buk/buc_command.sh around line 290. It currently does a timed read from /dev/tty with a "press Ctrl-C to cancel" prompt.

## Work

1. Add an environment variable check (e.g., BUC_NONINTERACTIVE=1 or BUK_BATCH=1) that skips the countdown entirely
2. Alternative: detect non-interactive context automatically (test if /dev/tty is available, or check if stdin is a terminal)
3. The skip should still print the warning message (so logs show it was acknowledged) but not wait for input
4. Ensure the pattern works for all tabtargets that use buc_countdown — this is a general solution, not specific to rbw-rrg
5. Document the mechanism in BCG or BUK README

## Acceptance

- Claude can run `./tt/rbw-rrg.RefreshGcbPins.sh` and similar countdown-protected tabtargets via Bash tool
- CI pipelines can run these without hanging on /dev/tty
- Interactive users still get the countdown cancel window by default
- Warning text still appears in logs even when countdown is bypassed

**[260216-1009] rough**

Add a parameter to buc_countdown that allows callers to bypass the interactive countdown, enabling Claude to run these tabtargets directly.

## Context

During ₣AP pace ₢APAAo, Claude could not run `./tt/rbw-rrg.RefreshGcbPins.sh` because buc_countdown reads from /dev/tty for the cancel window, which fails in non-interactive contexts (Claude's Bash tool, CI pipelines, etc.). The user had to run it manually in their terminal.

buc_countdown is in Tools/buk/buc_command.sh around line 290. It currently does a timed read from /dev/tty with a "press Ctrl-C to cancel" prompt.

## Work

1. Add an environment variable check (e.g., BUC_NONINTERACTIVE=1 or BUK_BATCH=1) that skips the countdown entirely
2. Alternative: detect non-interactive context automatically (test if /dev/tty is available, or check if stdin is a terminal)
3. The skip should still print the warning message (so logs show it was acknowledged) but not wait for input
4. Ensure the pattern works for all tabtargets that use buc_countdown — this is a general solution, not specific to rbw-rrg
5. Document the mechanism in BCG or BUK README

## Acceptance

- Claude can run `./tt/rbw-rrg.RefreshGcbPins.sh` and similar countdown-protected tabtargets via Bash tool
- CI pipelines can run these without hanging on /dev/tty
- Interactive users still get the countdown cancel window by default
- Warning text still appears in logs even when countdown is bypassed

### strip-down-quota-build-guide (₢AOAAF) [complete]

**[260217-1542] complete**

Strip down the RBSQB quota build manual procedure. The original guide was written to address concurrent build serialization, but the root cause turned out to be machine type selection (E2_HIGHCPU_8 consuming 8 of 10 CPU quota), not an actual quota shortage. The machine type was already fixed to UNSPECIFIED. The programmatic preflight check (rbgd_check_gcb_quota) stays as-is.

## Changes

Reframe purpose from "fix serialization problem" to "review Cloud Build capacity" — a health-check procedure, not a repair procedure.

### Keep
- Machine type / vCPU / concurrency table (genuinely useful reference)
- Console navigation steps to check quota
- Regime config display (RBRR_DEPOT_PROJECT_ID, RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE)

### Demote
- "Request quota increase" steps become a brief footnote, not a primary option
- Lesson learned: if builds serialize, check machine type first — quota increase is rarely the answer

### Remove
- "Option A vs Option B" framing that treats machine type change and quota increase as equal alternatives
- Machine type is the primary lever; quota increase is last resort

## Files
- `lenses/RBSQB-quota_build.adoc` — spec (primary edit)
- `Tools/rbw/rbgm_ManualProcedures.sh` — rbgm_quota_build() function (lines 324-376)
- `lenses/RBS0-SpecTop.adoc` — cross-reference text may need minor adjustment (lines ~669, 714-720)

## Wiring (no changes needed)
- `tt/rbw-QB.QuotaBuild.sh` — tabtarget stays
- `Tools/rbw/rbz_zipper.sh` — blazon stays
- `Tools/rbw/rbw_workbench.sh` — help text stays
- `Tools/rbw/rbgd_DepotConstants.sh` — programmatic check stays
- `Tools/rbw/rbf_Foundry.sh` — preflight call stays
- `Tools/rbw/rbrn_cli.sh` — audit call stays

**[260216-1010] rough**

Strip down the RBSQB quota build manual procedure. The original guide was written to address concurrent build serialization, but the root cause turned out to be machine type selection (E2_HIGHCPU_8 consuming 8 of 10 CPU quota), not an actual quota shortage. The machine type was already fixed to UNSPECIFIED. The programmatic preflight check (rbgd_check_gcb_quota) stays as-is.

## Changes

Reframe purpose from "fix serialization problem" to "review Cloud Build capacity" — a health-check procedure, not a repair procedure.

### Keep
- Machine type / vCPU / concurrency table (genuinely useful reference)
- Console navigation steps to check quota
- Regime config display (RBRR_DEPOT_PROJECT_ID, RBRR_GCP_REGION, RBRR_GCB_MACHINE_TYPE)

### Demote
- "Request quota increase" steps become a brief footnote, not a primary option
- Lesson learned: if builds serialize, check machine type first — quota increase is rarely the answer

### Remove
- "Option A vs Option B" framing that treats machine type change and quota increase as equal alternatives
- Machine type is the primary lever; quota increase is last resort

## Files
- `lenses/RBSQB-quota_build.adoc` — spec (primary edit)
- `Tools/rbw/rbgm_ManualProcedures.sh` — rbgm_quota_build() function (lines 324-376)
- `lenses/RBS0-SpecTop.adoc` — cross-reference text may need minor adjustment (lines ~669, 714-720)

## Wiring (no changes needed)
- `tt/rbw-QB.QuotaBuild.sh` — tabtarget stays
- `Tools/rbw/rbz_zipper.sh` — blazon stays
- `Tools/rbw/rbw_workbench.sh` — help text stays
- `Tools/rbw/rbgd_DepotConstants.sh` — programmatic check stays
- `Tools/rbw/rbf_Foundry.sh` — preflight call stays
- `Tools/rbw/rbrn_cli.sh` — audit call stays

### fix-rbgm-os-specific-link-instructions (₢AOAAG) [complete]

**[260217-1545] complete**

Fix OS-specific link instructions and redundant text in rbgm_ManualProcedures.sh.

## Issues (from orphaned heat triage)

1. Line 143: Remove redundant "Default text is this color." line — adds no value
2. Line 144: "(often, Ctrl + mouse click)" is wrong on macOS (should be Cmd+click). Replace with OS-detected instruction.
3. The Key section (lines 140-144) appears in both rbgm_payor_establish and rbgm_quota_build — fix in both places.

## Approach

Detect OS via `uname` and set the click modifier accordingly (Ctrl on Linux, Cmd on macOS). Apply to all bug_link calls that mention the modifier.

## Files
- `Tools/rbw/rbgm_ManualProcedures.sh` — primary edit target (lines 140-144 in payor_establish, similar block in quota_build around line 340-343)

**[260216-1031] rough**

Fix OS-specific link instructions and redundant text in rbgm_ManualProcedures.sh.

## Issues (from orphaned heat triage)

1. Line 143: Remove redundant "Default text is this color." line — adds no value
2. Line 144: "(often, Ctrl + mouse click)" is wrong on macOS (should be Cmd+click). Replace with OS-detected instruction.
3. The Key section (lines 140-144) appears in both rbgm_payor_establish and rbgm_quota_build — fix in both places.

## Approach

Detect OS via `uname` and set the click modifier accordingly (Ctrl on Linux, Cmd on macOS). Apply to all bug_link calls that mention the modifier.

## Files
- `Tools/rbw/rbgm_ManualProcedures.sh` — primary edit target (lines 140-144 in payor_establish, similar block in quota_build around line 340-343)

### untangle-rbgm-path-indirections (₢AOAAH) [complete]

**[260217-1550] complete**

Replace rbgm_ManualProcedures.sh's hand-rolled path resolution with the standard RBCC constants that every other module uses.

## Problem

`zrbgm_kindle()` (lines 66-69) builds absolute paths to `rbrp.env` and `rbrr.env` using `$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)` — a subshell cd pattern that bypasses the project's convention of resolving from project root. Every other module uses `RBCC_RBRP_FILE` and `RBCC_RBRR_FILE` (bare relative paths set in `rbcc_Constants.sh`), which are already available via the `rbgm_cli.sh` sourcing chain (line 31 sources `rbcc_Constants.sh`).

## Changes

1. In `zrbgm_kindle()`:
   - Delete the `# ITCH_LINK_TO_RBL` comment (line 66)
   - Replace `ZRBGM_RBRP_FILE="$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)/rbrp.env"` with `ZRBGM_RBRP_FILE="${RBCC_RBRP_FILE}"`
   - Replace `ZRBGM_RBRR_FILE="$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)/rbrr.env"` with `ZRBGM_RBRR_FILE="${RBCC_RBRR_FILE}"`
   - Keep `ZRBGM_RBRP_FILE_BASENAME` derivation (line 68) — it still works

2. Verify downstream references still work — ~6 uses of `ZRBGM_RBRP_FILE` / `ZRBGM_RBRR_FILE` in bug_tc/bug_tu display calls. These display the file path to the user, so bare relative paths (`rbrp.env`) are actually better for display than absolute paths.

## Files
- `Tools/rbw/rbgm_ManualProcedures.sh` — lines 66-69 in `zrbgm_kindle()`

## Verification
- Confirm `RBCC_RBRP_FILE` and `RBCC_RBRR_FILE` are set before `zrbgm_kindle()` runs (trace sourcing order in `rbgm_cli.sh`)

**[260217-1549] rough**

Replace rbgm_ManualProcedures.sh's hand-rolled path resolution with the standard RBCC constants that every other module uses.

## Problem

`zrbgm_kindle()` (lines 66-69) builds absolute paths to `rbrp.env` and `rbrr.env` using `$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)` — a subshell cd pattern that bypasses the project's convention of resolving from project root. Every other module uses `RBCC_RBRP_FILE` and `RBCC_RBRR_FILE` (bare relative paths set in `rbcc_Constants.sh`), which are already available via the `rbgm_cli.sh` sourcing chain (line 31 sources `rbcc_Constants.sh`).

## Changes

1. In `zrbgm_kindle()`:
   - Delete the `# ITCH_LINK_TO_RBL` comment (line 66)
   - Replace `ZRBGM_RBRP_FILE="$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)/rbrp.env"` with `ZRBGM_RBRP_FILE="${RBCC_RBRP_FILE}"`
   - Replace `ZRBGM_RBRR_FILE="$(cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd)/rbrr.env"` with `ZRBGM_RBRR_FILE="${RBCC_RBRR_FILE}"`
   - Keep `ZRBGM_RBRP_FILE_BASENAME` derivation (line 68) — it still works

2. Verify downstream references still work — ~6 uses of `ZRBGM_RBRP_FILE` / `ZRBGM_RBRR_FILE` in bug_tc/bug_tu display calls. These display the file path to the user, so bare relative paths (`rbrp.env`) are actually better for display than absolute paths.

## Files
- `Tools/rbw/rbgm_ManualProcedures.sh` — lines 66-69 in `zrbgm_kindle()`

## Verification
- Confirm `RBCC_RBRP_FILE` and `RBCC_RBRR_FILE` are set before `zrbgm_kindle()` runs (trace sourcing order in `rbgm_cli.sh`)

**[260216-1031] rough**

Simplify path resolution in rbgm_ManualProcedures.sh lines 60-63.

## Issues (from orphaned heat triage)

1. Line 60: `ITCH_LINK_TO_RBL` comment still present — resolve or remove
2. Lines 61-63: Use `cd "${ZRBGM_SCRIPT_DIR}/../.." && pwd` to compute paths — fragile indirection
3. `ZRBGM_SCRIPT_DIR` is never defined in this file (comes from sourcing chain) — verify provenance

## Approach

Replace `cd .. && pwd` pattern with direct path resolution using BURC_PROJECT_ROOT or equivalent project-root variable that should already be available in the kindling context. Remove the ITCH comment once resolved.

## Files
- `Tools/rbw/rbgm_ManualProcedures.sh` — lines 60-63 in zrbgm_kindle()

### create-burd-regime-module (₢AOAAM) [complete]

**[260223-1840] complete**

Create and wire burd_regime.sh — BURD dispatch runtime regime module.

## COMPLETED

1. Created `Tools/buk/burd_regime.sh` with zburd_kindle/sentinel/validate_fields/render
   - Variable taxonomy: 16 required, 6 optional, 3 conditional (log paths)
   - Log path validation gated on both BURD_NO_LOG empty AND BURD_LOG_LAST present
   - Unexpected variable detection covers all 25 BURD_ variables

2. Added `BURC_BUK_DIR="${BURC_TOOLS_DIR}/buk"` to burc_regime.sh kindle

3. Swept 19 files replacing ad-hoc BURD checks with zburd_kindle/sentinel:
   - 5 workbenches: vvw, vow, jjw, cccw, vslw (source + kindle at top, sentinel in route)
   - 4 BUK files: burc_cli, burs_cli, buut_cli, buut_tabtarget
   - 3 RBW CLIs: rbf_cli, rbrr_cli, rbgg_cli (source + kindle)
   - 4 RBW modules: rbf_Foundry, rbgo_OAuth, rbgg_Governor, rbi_Image (sentinel only)
   - 1 VOK CLI: vob_cli (source + kindle)
   - 1 VOK module: vob_build (sentinel only)
   - 1 BUK module: bute_engine (sentinel only)

4. Fixed double-kindle bugs in rbtb_testbench.sh exposed by test-all:
   - Removed redundant zrbcc_kindle from zrbtb_setup_qualify
   - Hoisted zbuz_kindle + zrbz_kindle to top level (was repeated in 3 setup functions)

5. Tests passing: rbw-trg (7 green), buw-rcv (green), vow-b (green), rbw-qa (green)

## REMAINING — ark-lifecycle regression in rbw-ta.TestAll.sh

`rbw-ta.TestAll.sh` fails at ark-lifecycle suite: `rbtcal_lifecycle` dispatches `rbw-aC.ConjureArk.sh`
which transits rbw_workbench → rbf_cli → rbf_Foundry chain. Error is "Tabtarget failed with status 1".

This test was passing before the BURD sweep. Likely cause: our changes to rbf_cli.sh
(added burd_regime.sh source + zburd_kindle in furnish) or rbf_Foundry.sh (replaced ad-hoc
checks with zburd_sentinel).

Investigate:
- Run `rbw-ts ark-lifecycle` with BURD_VERBOSE=1 to see where it fails
- Check if zburd_kindle in rbf_cli.sh furnish function interacts badly with buc_execute
- Check if the BURV isolation in bute_dispatch affects BURD variable availability

## Acceptance

- burd_regime.sh exists with kindle/sentinel/validate/render
- BURC_BUK_DIR constant eliminates magic "buk" strings
- All ad-hoc "is BURD set?" guards replaced with zburd_kindle or zburd_sentinel
- `rbw-ta.TestAll.sh` passes (ALL suites green)

**[260217-1710] rough**

Create and wire burd_regime.sh — BURD dispatch runtime regime module.

## COMPLETED

1. Created `Tools/buk/burd_regime.sh` with zburd_kindle/sentinel/validate_fields/render
   - Variable taxonomy: 16 required, 6 optional, 3 conditional (log paths)
   - Log path validation gated on both BURD_NO_LOG empty AND BURD_LOG_LAST present
   - Unexpected variable detection covers all 25 BURD_ variables

2. Added `BURC_BUK_DIR="${BURC_TOOLS_DIR}/buk"` to burc_regime.sh kindle

3. Swept 19 files replacing ad-hoc BURD checks with zburd_kindle/sentinel:
   - 5 workbenches: vvw, vow, jjw, cccw, vslw (source + kindle at top, sentinel in route)
   - 4 BUK files: burc_cli, burs_cli, buut_cli, buut_tabtarget
   - 3 RBW CLIs: rbf_cli, rbrr_cli, rbgg_cli (source + kindle)
   - 4 RBW modules: rbf_Foundry, rbgo_OAuth, rbgg_Governor, rbi_Image (sentinel only)
   - 1 VOK CLI: vob_cli (source + kindle)
   - 1 VOK module: vob_build (sentinel only)
   - 1 BUK module: bute_engine (sentinel only)

4. Fixed double-kindle bugs in rbtb_testbench.sh exposed by test-all:
   - Removed redundant zrbcc_kindle from zrbtb_setup_qualify
   - Hoisted zbuz_kindle + zrbz_kindle to top level (was repeated in 3 setup functions)

5. Tests passing: rbw-trg (7 green), buw-rcv (green), vow-b (green), rbw-qa (green)

## REMAINING — ark-lifecycle regression in rbw-ta.TestAll.sh

`rbw-ta.TestAll.sh` fails at ark-lifecycle suite: `rbtcal_lifecycle` dispatches `rbw-aC.ConjureArk.sh`
which transits rbw_workbench → rbf_cli → rbf_Foundry chain. Error is "Tabtarget failed with status 1".

This test was passing before the BURD sweep. Likely cause: our changes to rbf_cli.sh
(added burd_regime.sh source + zburd_kindle in furnish) or rbf_Foundry.sh (replaced ad-hoc
checks with zburd_sentinel).

Investigate:
- Run `rbw-ts ark-lifecycle` with BURD_VERBOSE=1 to see where it fails
- Check if zburd_kindle in rbf_cli.sh furnish function interacts badly with buc_execute
- Check if the BURV isolation in bute_dispatch affects BURD variable availability

## Acceptance

- burd_regime.sh exists with kindle/sentinel/validate/render
- BURC_BUK_DIR constant eliminates magic "buk" strings
- All ad-hoc "is BURD set?" guards replaced with zburd_kindle or zburd_sentinel
- `rbw-ta.TestAll.sh` passes (ALL suites green)

**[260217-1648] rough**

Create and wire burd_regime.sh — BURD dispatch runtime regime module.

## COMPLETED

1. Created `Tools/buk/burd_regime.sh` with zburd_kindle/sentinel/validate_fields/render
   - Variable taxonomy: 16 required, 6 optional, 3 conditional (log paths)
   - Log path validation gated on both BURD_NO_LOG empty AND BURD_LOG_LAST present
     (log vars are NOT exported by dispatch, so child processes won't have them)
   - Unexpected variable detection covers all 25 BURD_ variables

2. Added `BURC_BUK_DIR="${BURC_TOOLS_DIR}/buk"` to burc_regime.sh kindle
   - Exported, added to z_known list
   - All consumers source via `source "${BURC_BUK_DIR}/burd_regime.sh"`

3. Converted and tested:
   - buw_workbench.sh — kindle at top, sentinel in route. Tested via buw-rcv, buw-rev, buw-rsv.
   - rbw_workbench.sh — kindle at top, sentinel in route
   - rbtb_testbench.sh — kindle at top (after sources, before zrbcc_kindle), sentinel in route
   - butcrg_RegimeSmoke.sh — sentinel + zburd_validate_fields in butcrg_burd test case
   - ALL tested via rbw-trg.TestRegimeSmoke.sh (7 cases, all green)

4. Fixed bure_cli.sh execute bit (lost by haiku agent Write; chmod +x committed)

## REMAINING — replace ad-hoc BURD checks with zburd_kindle/sentinel

Pattern for each file:
- Add `source "${BURC_BUK_DIR}/burd_regime.sh"` near other source lines
- Replace first batch of ad-hoc BURD_ checks with `zburd_kindle`
- Replace subsequent checks in same file with `zburd_sentinel`
- For files exec'd from a workbench (new process), use zburd_kindle
- For sourced files where kindle already happened upstream, use zburd_sentinel

### Workbenches (kindle at top, sentinel in route):
- vvw_workbench.sh (Tools/vvk/) — lines 33-34 + 44-45
- vow_workbench.sh (Tools/vok/) — lines 33-34 + 49-50
- jjw_workbench.sh (Tools/jjk/) — lines 45-46
- cccw_workbench.sh (Tools/ccck/) — lines 54-55
- vslw_workbench.sh (Tools/vslk/) — lines 60-61

### Test infrastructure:
- bute_engine.sh (Tools/buk/) — line 94
- buut_tabtarget.sh (Tools/buk/) — line 37

### CLIs (exec'd from workbenches — new process, need kindle):
- burc_cli.sh (Tools/buk/) — line 41 (BURD_REGIME_FILE check in zburc_cli_kindle)
- burs_cli.sh (Tools/buk/) — line 41 (BURD_STATION_FILE check in zburs_cli_kindle)
- rbf_Foundry.sh (Tools/rbw/) — line 35
- rbf_cli.sh (Tools/rbw/) — line 41
- rbi_Image.sh (Tools/rbw/) — line 38
- rbgo_OAuth.sh (Tools/rbw/) — line 43
- rbrr_cli.sh (Tools/rbw/) — lines 141-142
- rbgg_Governor.sh (Tools/rbw/) — lines 490, 535
- vob_build.sh (Tools/vok/) — line 38

Note: rbw_workbench.sh line 75 (BURD_TOKEN_3 check) is a semantic check — leave it.

### After sweep, run these tests:
- `tt/rbw-trg.TestRegimeSmoke.sh` (regime smoke — already green)
- `tt/buw-rcv.ValidateConfigRegime.sh` (buw path)
- `tt/vow-b.Build.sh` (vok path)
- `tt/rbw-qa.QualifyAll.sh` (full qualification if available)

## Acceptance

- burd_regime.sh exists with kindle/sentinel/validate/render
- BURC_BUK_DIR constant eliminates magic "buk" strings
- All ad-hoc "is BURD set?" guards replaced with zburd_kindle or zburd_sentinel
- rbw-trg.TestRegimeSmoke.sh passes
- Qualification passes

**[260217-1628] rough**

Create Tools/buk/burd_regime.sh following the pattern of burc_regime.sh, burs_regime.sh, and bure_regime.sh.

## Context

BURD (Dispatch Runtime) is fully specified in BUSA and BUSD-DispatchRuntime.adoc, but lacks a regime module file. Currently zburd_sentinel() lives orphaned inside bud_dispatch.sh. Ad-hoc BURD variable checks are scattered across ~25 sites in workbenches, testbenches, CLIs, and utility modules — each testing one or two BURD_ variables to confirm dispatch happened.

## Variable Taxonomy

### Required (always set by launcher + dispatch, validate as non-empty):
- BURD_REGIME_FILE (launcher)
- BURD_STATION_FILE (launcher)
- BURD_COORDINATOR_SCRIPT (launcher)
- BURD_LAUNCHER (launcher)
- BURD_TERM_COLS (launcher, defaults 80)
- BURD_NOW_STAMP (dispatch computed)
- BURD_TEMP_DIR (dispatch computed)
- BURD_OUTPUT_DIR (dispatch computed)
- BURD_TRANSCRIPT (dispatch computed)
- BURD_GIT_CONTEXT (dispatch computed)
- BURD_COLOR (dispatch resolved to 0 or 1)
- BURD_TARGET (dispatch parsed)
- BURD_COMMAND (dispatch parsed)
- BURD_TOKEN_1 (dispatch parsed)
- BURD_TOKEN_2 (dispatch parsed)
- BURD_CLI_ARGS (dispatch parsed, may be empty array)

### Optional (caller may or may not set):
- BURD_VERBOSE (defaults to 0 if unset)
- BURD_NO_LOG (unset = logging enabled)
- BURD_INTERACTIVE (unset = non-interactive)
- BURD_TOKEN_3 (empty if no imprint)
- BURD_TOKEN_4 (rare)
- BURD_TOKEN_5 (rare)

### Conditional on logging (set only when BURD_NO_LOG is empty):
- BURD_LOG_LAST
- BURD_LOG_SAME
- BURD_LOG_HIST

## Scope

1. Create `Tools/buk/burd_regime.sh` with:
   - `zburd_kindle()` — set defaults for optional vars, detect unexpected BURD_ variables
   - `zburd_sentinel()` — assert dispatch initialized (moved from bud_dispatch.sh)
   - `zburd_validate()` — verify all required vars non-empty; if logging active, verify log vars
   - `zburd_render()` — diagnostic display of current BURD_ state grouped by taxonomy

2. Source burd_regime.sh from bud_dispatch.sh; remove inline zburd_sentinel()

3. Replace ALL ad-hoc BURD_ variable checks with zburd_sentinel() calls:

   Workbenches (check REGIME_FILE + STATION_FILE and/or TEMP_DIR + NOW_STAMP):
   - buw_workbench.sh (lines 33-34, 49-50)
   - rbw_workbench.sh (lines 64-65)
   - vvw_workbench.sh (lines 33-34, 44-45)
   - vow_workbench.sh (lines 33-34, 49-50)
   - jjw_workbench.sh (lines 45-46)
   - cccw_workbench.sh (lines 54-55)
   - vslw_workbench.sh (lines 60-61)

   Testbenches and test infrastructure:
   - rbtb_testbench.sh (lines 264-265)
   - butcrg_RegimeSmoke.sh (lines 134-137)
   - bute_engine.sh (line 94)
   - buut_tabtarget.sh (line 37)

   CLIs and modules:
   - burc_cli.sh (line 41 — BURD_REGIME_FILE check in cli_kindle)
   - burs_cli.sh (line 41 — BURD_STATION_FILE check in cli_kindle)
   - rbf_Foundry.sh (line 35)
   - rbf_cli.sh (line 41)
   - rbi_Image.sh (line 38)
   - rbgo_OAuth.sh (line 43)
   - rbrr_cli.sh (lines 141-142)
   - rbgg_Governor.sh (lines 490, 535)
   - vob_build.sh (line 38)

   Note: rbw_workbench.sh line 75 (BURD_TOKEN_3 check) is a semantic check — leave it.

## Acceptance

- burd_regime.sh exists with kindle/sentinel/validate/render following sibling regime patterns
- zburd_sentinel() no longer defined inline in bud_dispatch.sh
- All ad-hoc "is BURD set?" guards replaced with zburd_sentinel()
- Qualification passes

**[260217-1626] rough**

Create Tools/buk/burd_regime.sh following the pattern of burc_regime.sh, burs_regime.sh, and bure_regime.sh.

## Context

BURD (Dispatch Runtime) is fully specified in BUSA and BUSD-DispatchRuntime.adoc, but lacks a regime module file. Currently zburd_sentinel() lives orphaned inside bud_dispatch.sh. Ad-hoc BURD variable checks are scattered across ~25 sites in workbenches, testbenches, CLIs, and utility modules — each testing one or two BURD_ variables to confirm dispatch happened.

## Scope

1. Create `Tools/buk/burd_regime.sh` with:
   - `zburd_kindle()` — initialize/validate BURD regime state
   - `zburd_validate()` — verify all expected BURD_ variables are set and well-formed
   - `zburd_render()` — diagnostic display of current BURD_ variable state
   - Move `zburd_sentinel()` from bud_dispatch.sh into this file

2. Source burd_regime.sh from bud_dispatch.sh so sentinel remains available

3. Replace ALL ad-hoc BURD_ variable checks with zburd_sentinel() calls:

   Workbenches (check REGIME_FILE + STATION_FILE and/or TEMP_DIR + NOW_STAMP):
   - buw_workbench.sh (lines 33-34, 49-50)
   - rbw_workbench.sh (lines 64-65)
   - vvw_workbench.sh (lines 33-34, 44-45)
   - vow_workbench.sh (lines 33-34, 49-50)
   - jjw_workbench.sh (lines 45-46)
   - cccw_workbench.sh (lines 54-55)
   - vslw_workbench.sh (lines 60-61)

   Testbenches and test infrastructure:
   - rbtb_testbench.sh (lines 264-265)
   - butcrg_RegimeSmoke.sh (lines 134-137)
   - bute_engine.sh (line 94)
   - buut_tabtarget.sh (line 37)

   CLIs and modules:
   - burc_cli.sh (line 41 — BURD_REGIME_FILE check)
   - burs_cli.sh (line 41 — BURD_STATION_FILE check)
   - rbf_Foundry.sh (line 35)
   - rbf_cli.sh (line 41)
   - rbi_Image.sh (line 38)
   - rbgo_OAuth.sh (line 43)
   - rbrr_cli.sh (lines 141-142)
   - rbgg_Governor.sh (lines 490, 535)
   - vob_build.sh (line 38)

   Note: rbw_workbench.sh line 75 (BURD_TOKEN_3 check) is a semantic check, not a dispatch-happened guard — leave it.

## Acceptance

- burd_regime.sh exists with kindle/validate/render following sibling regime patterns
- zburd_sentinel() no longer defined inline in bud_dispatch.sh
- All ad-hoc "is BURD set?" guards replaced with zburd_sentinel()
- Qualification passes

**[260217-1621] rough**

Create Tools/buk/burd_regime.sh following the pattern of burc_regime.sh, burs_regime.sh, and bure_regime.sh.

## Context

BURD (Dispatch Runtime) is fully specified in BUSA and BUSD-DispatchRuntime.adoc, but lacks a regime module file. Currently zburd_sentinel() lives orphaned inside bud_dispatch.sh. Ad-hoc BURD variable checks are scattered across CLI files (burc_cli.sh checks BURD_REGIME_FILE, burs_cli.sh checks BURD_STATION_FILE, buw_workbench.sh checks both).

## Scope

1. Create `Tools/buk/burd_regime.sh` with:
   - `zburd_kindle()` — initialize/validate BURD regime state (or delegate to sentinel)
   - `zburd_validate()` — verify all expected BURD_ variables are set and well-formed
   - `zburd_render()` — diagnostic display of current BURD_ variable state
   - Move `zburd_sentinel()` from bud_dispatch.sh into this file (source it from dispatch)

2. Replace ad-hoc BURD variable checks with calls to burd_regime.sh functions:
   - burc_cli.sh: replace BURD_REGIME_FILE check with zburd_validate or sentinel
   - burs_cli.sh: replace BURD_STATION_FILE check with zburd_validate or sentinel
   - buw_workbench.sh lines 33-34: replace both checks

3. Source burd_regime.sh from bud_dispatch.sh so sentinel remains available

## Acceptance

- burd_regime.sh exists with kindle/validate/render following sibling regime patterns
- zburd_sentinel() no longer defined inline in bud_dispatch.sh
- All ad-hoc BURD_ variable checks consolidated into burd_regime.sh
- Qualification passes

### replace-dotdot-buk-with-burd-buk-dir (₢AOAAO) [complete]

**[260223-1853] complete**

Replace all ../buk/ path gymnastics with BURD_BUK_DIR constant.

## Context

BURD_BUK_DIR is set by bud_dispatch.sh during tabtarget dispatch, before any workbench/cli code executes. Seven files were already converted during ₢AOAAM. The remaining files still use relative ../buk/ paths.

## Current Scope (assessed 2026-02-23)

14 files, 36 occurrences of ../buk/:

Tools/rbw/: rbtb_testbench.sh(7) rbrr_cli.sh(4) rbq_cli.sh(3) rbob_cli.sh(2) rbcr_render.sh(1) rbq_Qualify.sh(1)
Tools/jjk/: jjw_workbench.sh(3) jja_arcanum.sh(1)
Tools/vok/: vob_cli.sh(3) vow_workbench.sh(3)
Tools/cmk/: cmw_workbench.sh(1)
Tools/vvk/: vvw_workbench.sh(3) vvb_cli.sh(1)
Tools/vslk/: vslw_workbench.sh(3)

Additionally, 2 stale BURC_BUK_DIR references to fix:
- rbtb_testbench.sh:29 (BURC_BUK_DIR → BURD_BUK_DIR)
- buw_workbench.sh:29 (BURC_BUK_DIR → BURD_BUK_DIR)

Total: 38 edits across 15 files.

## Transformation

Replace: `source "${SCRIPT_DIR}/../buk/filename.sh"`
With:    `source "${BURD_BUK_DIR}/filename.sh"`

Also replace: `${BURC_BUK_DIR}/` with `${BURD_BUK_DIR}/` in rbtb_testbench.sh and buw_workbench.sh.

Note: rbtb_testbench.sh line 25 has `RBTB_BUTS_DIR="${RBTB_SCRIPT_DIR}/../buk/buts"` (directory path, not source) — also replace with `${BURD_BUK_DIR}/buts`.

BURD_BUK_DIR is set by bud_dispatch.sh and exported before any workbench/cli runs.

## Acceptance

- Zero occurrences of `../buk/` remain in .sh files
- Zero occurrences of BURC_BUK_DIR in source lines (only burc_regime.sh definition)
- rbw-trg.TestRegimeSmoke.sh passes
- Qualification passes

**[260223-1849] rough**

Replace all ../buk/ path gymnastics with BURD_BUK_DIR constant.

## Context

BURD_BUK_DIR is set by bud_dispatch.sh during tabtarget dispatch, before any workbench/cli code executes. Seven files were already converted during ₢AOAAM. The remaining files still use relative ../buk/ paths.

## Current Scope (assessed 2026-02-23)

14 files, 36 occurrences of ../buk/:

Tools/rbw/: rbtb_testbench.sh(7) rbrr_cli.sh(4) rbq_cli.sh(3) rbob_cli.sh(2) rbcr_render.sh(1) rbq_Qualify.sh(1)
Tools/jjk/: jjw_workbench.sh(3) jja_arcanum.sh(1)
Tools/vok/: vob_cli.sh(3) vow_workbench.sh(3)
Tools/cmk/: cmw_workbench.sh(1)
Tools/vvk/: vvw_workbench.sh(3) vvb_cli.sh(1)
Tools/vslk/: vslw_workbench.sh(3)

Additionally, 2 stale BURC_BUK_DIR references to fix:
- rbtb_testbench.sh:29 (BURC_BUK_DIR → BURD_BUK_DIR)
- buw_workbench.sh:29 (BURC_BUK_DIR → BURD_BUK_DIR)

Total: 38 edits across 15 files.

## Transformation

Replace: `source "${SCRIPT_DIR}/../buk/filename.sh"`
With:    `source "${BURD_BUK_DIR}/filename.sh"`

Also replace: `${BURC_BUK_DIR}/` with `${BURD_BUK_DIR}/` in rbtb_testbench.sh and buw_workbench.sh.

Note: rbtb_testbench.sh line 25 has `RBTB_BUTS_DIR="${RBTB_SCRIPT_DIR}/../buk/buts"` (directory path, not source) — also replace with `${BURD_BUK_DIR}/buts`.

BURD_BUK_DIR is set by bud_dispatch.sh and exported before any workbench/cli runs.

## Acceptance

- Zero occurrences of `../buk/` remain in .sh files
- Zero occurrences of BURC_BUK_DIR in source lines (only burc_regime.sh definition)
- rbw-trg.TestRegimeSmoke.sh passes
- Qualification passes

**[260223-1849] rough**

Replace all ../buk/ path gymnastics with BURD_BUK_DIR constant.

## Context

BURD_BUK_DIR is set by bud_dispatch.sh during tabtarget dispatch, before any workbench/cli code executes. Seven files were already converted during ₢AOAAM. The remaining files still use relative ../buk/ paths.

## Current Scope (assessed 2026-02-23)

14 files, 36 occurrences of ../buk/:

Tools/rbw/: rbtb_testbench.sh(7) rbrr_cli.sh(4) rbq_cli.sh(3) rbob_cli.sh(2) rbcr_render.sh(1) rbq_Qualify.sh(1)
Tools/jjk/: jjw_workbench.sh(3) jja_arcanum.sh(1)
Tools/vok/: vob_cli.sh(3) vow_workbench.sh(3)
Tools/cmk/: cmw_workbench.sh(1)
Tools/vvk/: vvw_workbench.sh(3) vvb_cli.sh(1)
Tools/vslk/: vslw_workbench.sh(3)

Additionally, 2 stale BURC_BUK_DIR references to fix:
- rbtb_testbench.sh:29 (BURC_BUK_DIR → BURD_BUK_DIR)
- buw_workbench.sh:29 (BURC_BUK_DIR → BURD_BUK_DIR)

Total: 38 edits across 15 files.

## Transformation

Replace: `source "${SCRIPT_DIR}/../buk/filename.sh"`
With:    `source "${BURD_BUK_DIR}/filename.sh"`

Also replace: `${BURC_BUK_DIR}/` with `${BURD_BUK_DIR}/` in rbtb_testbench.sh and buw_workbench.sh.

Note: rbtb_testbench.sh line 25 has `RBTB_BUTS_DIR="${RBTB_SCRIPT_DIR}/../buk/buts"` (directory path, not source) — also replace with `${BURD_BUK_DIR}/buts`.

BURD_BUK_DIR is set by bud_dispatch.sh and exported before any workbench/cli runs.

## Acceptance

- Zero occurrences of `../buk/` remain in .sh files
- Zero occurrences of BURC_BUK_DIR in source lines (only burc_regime.sh definition)
- rbw-trg.TestRegimeSmoke.sh passes
- Qualification passes

**[260217-1649] rough**

Replace all ../buk/ path gymnastics with BURC_BUK_DIR constant.

## Context

BURC_BUK_DIR was introduced in ₢AOAAM to eliminate the magic "buk" directory name. Currently 41 occurrences of `../buk/` exist across 20 files. These should all use `${BURC_BUK_DIR}/` instead.

## Scope

20 files, 41 occurrences:

Tools/rbw/: rbrs_cli.sh(2) rbw_workbench.sh(2) rbq_cli.sh(3) rbob_cli.sh(2) rbcr_render.sh(1) rbrr_cli.sh(2) rbrp_cli.sh(2) rbro_cli.sh(2) rbra_cli.sh(2) rbtb_testbench.sh(10) rbrn_cli.sh(2) rbrv_cli.sh(2)
Tools/jjk/: jjw_workbench.sh(1) jja_arcanum.sh(1)
Tools/cmk/: cmw_workbench.sh(1)
Tools/vok/: vob_cli.sh(2) vow_workbench.sh(1)
Tools/vvk/: vvw_workbench.sh(1) vvb_cli.sh(1)
Tools/vslk/: vslw_workbench.sh(1)

## Transformation

Replace: `source "${SCRIPT_DIR}/../buk/filename.sh"`
With:    `source "${BURC_BUK_DIR}/filename.sh"`

BURC_BUK_DIR is exported by burc_regime.sh and available in all dispatch contexts.

## Acceptance

- Zero occurrences of `../buk/` remain in .sh files
- rbw-trg.TestRegimeSmoke.sh passes
- Qualification passes

### audit-cli-kindle-functions (₢AOAAJ) [complete]

**[260223-1903] complete**

Investigate _cli_kindle functions across the codebase. Determine if they serve a purpose or are dead code. If dead, remove them.

## Scope

- Find all *_cli_kindle function definitions
- Trace callers — are they invoked anywhere?
- If unused, delete the functions and any related scaffolding
- If used, document what they do and close as no-op

## Acceptance

- Every _cli_kindle function either has a documented caller or is deleted
- Qualification passes

**[260217-0748] rough**

Investigate _cli_kindle functions across the codebase. Determine if they serve a purpose or are dead code. If dead, remove them.

## Scope

- Find all *_cli_kindle function definitions
- Trace callers — are they invoked anywhere?
- If unused, delete the functions and any related scaffolding
- If used, document what they do and close as no-op

## Acceptance

- Every _cli_kindle function either has a documented caller or is deleted
- Qualification passes

### bcg-modernize-rbrs-cli (₢AOAAK) [complete]

**[260224-0555] complete**

Interactively modernize rbrs_cli.sh to full BCG compliance.

## Approach

- Work through rbrs_cli.sh with human, applying BCG patterns
- Keep a running list of every cleanup applied (pattern name + before/after)
- This list becomes the spec for a follow-on pace that applies the same cleanups to all config regime CLI files

## Acceptance

- rbrs_cli.sh fully BCG-compliant
- Cleanup list captured (for follow-on pace)
- Follow-on pace slated with the cleanup list as its docket

**[260217-0750] rough**

Interactively modernize rbrs_cli.sh to full BCG compliance.

## Approach

- Work through rbrs_cli.sh with human, applying BCG patterns
- Keep a running list of every cleanup applied (pattern name + before/after)
- This list becomes the spec for a follow-on pace that applies the same cleanups to all config regime CLI files

## Acceptance

- rbrs_cli.sh fully BCG-compliant
- Cleanup list captured (for follow-on pace)
- Follow-on pace slated with the cleanup list as its docket

### bcg-reconcile-sourcing-in-furnish (₢AOAAT) [complete]

**[260224-0614] complete**

Reconcile BCG sourcing rules with the furnish-sources-everything pattern.

## Context

Two living patterns exist for CLI dependency sourcing:

**Pattern A (header-sources):** CLI header sources all modules; furnish only kindles.
Examples: rbob_cli.sh, rbf_cli.sh, rbrr_cli.sh

**Pattern B (furnish-sources):** CLI header sources only buc_command.sh; furnish sources all modules + kindles.
Examples: rbrs_cli.sh, rbrn_cli.sh (differential furnish)

BCG currently prescribes Pattern A ("CLI file header → All dependencies → Module loading"). Pattern B emerged naturally from differential furnish (rbrn) and has been adopted by newer CLIs.

## Analysis Needed

1. Inventory all CLIs: which use Pattern A vs Pattern B?
2. Does Pattern B enable doc-mode help display? (furnish can gate after buc_doc_env before sourcing fails)
3. Does Pattern A have advantages Pattern B lacks?
4. Should BCG bless both patterns, or converge on one?
5. If converging: what's the migration scope?

## Key BCG locations to update if converging

- Boilerplate Functions table (furnish row, CLI header row)
- Template 2: CLI Entry Point
- Sourcing Rules table
- Module Maturity Checklist (Module Structure, Required Functions, Sourcing Restrictions)
- Literal constant note (references "CLI header block")

## Acceptance

- BCG sourcing rules reflect the chosen pattern(s)
- All affected checklist items updated
- No code changes in this pace — BCG spec only

**[260223-1924] rough**

Reconcile BCG sourcing rules with the furnish-sources-everything pattern.

## Context

Two living patterns exist for CLI dependency sourcing:

**Pattern A (header-sources):** CLI header sources all modules; furnish only kindles.
Examples: rbob_cli.sh, rbf_cli.sh, rbrr_cli.sh

**Pattern B (furnish-sources):** CLI header sources only buc_command.sh; furnish sources all modules + kindles.
Examples: rbrs_cli.sh, rbrn_cli.sh (differential furnish)

BCG currently prescribes Pattern A ("CLI file header → All dependencies → Module loading"). Pattern B emerged naturally from differential furnish (rbrn) and has been adopted by newer CLIs.

## Analysis Needed

1. Inventory all CLIs: which use Pattern A vs Pattern B?
2. Does Pattern B enable doc-mode help display? (furnish can gate after buc_doc_env before sourcing fails)
3. Does Pattern A have advantages Pattern B lacks?
4. Should BCG bless both patterns, or converge on one?
5. If converging: what's the migration scope?

## Key BCG locations to update if converging

- Boilerplate Functions table (furnish row, CLI header row)
- Template 2: CLI Entry Point
- Sourcing Rules table
- Module Maturity Checklist (Module Structure, Required Functions, Sourcing Restrictions)
- Literal constant note (references "CLI header block")

## Acceptance

- BCG sourcing rules reflect the chosen pattern(s)
- All affected checklist items updated
- No code changes in this pace — BCG spec only

### migrate-cli-sourcing-to-furnish (₢AOAAU) [complete]

**[260224-0654] complete**

Migrate all Pattern A CLIs to Pattern B (furnish-sources) and add buc_doc_env + buc_doc_env_done gate to all CLIs.

## Context

BCG now prescribes Pattern B: CLI header sources only buc_command.sh; furnish sources all other dependencies. 11 CLIs still use Pattern A (header-sources). Additionally, most CLIs lack buc_doc_env calls and the buc_doc_env_done gate.

## Pattern A CLIs to migrate (move top-level sources into furnish)

1. rbw/rbgg_cli.sh
2. rbw/rbgm_cli.sh
3. rbw/rbv_cli.sh
4. rbw/rbq_cli.sh
5. buk/buut_cli.sh
6. vvk/vvb_cli.sh
7. rbw/rbrr_cli.sh
8. rbw/rbf_cli.sh
9. rbw/rbgp_cli.sh
10. rbw/rbob_cli.sh
11. vok/vob_cli.sh

## Pattern B CLIs needing buc_doc_env + gate (already furnish-sources)

1. rbw/rbra_cli.sh
2. rbw/rbrv_cli.sh (has doc_env, no gate)
3. buk/burc_cli.sh
4. rbw/rbro_cli.sh
5. rbw/rbrp_cli.sh
6. buk/burs_cli.sh
7. rbw/rbrn_cli.sh
8. buk/bure_cli.sh

(rbrs_cli.sh already done)

## Transformation pattern

For each CLI:
1. Keep only `source "${BURD_BUK_DIR}/buc_command.sh"` at top level
2. Move all other source statements into furnish, after buc_doc_env_done gate
3. Add buc_doc_env calls for BURD_BUK_DIR, BURD_TOOLS_DIR (and any other external env vars)
4. Add `buc_doc_env_done || return 0` gate after the env doc block
5. Replace any BASH_SOURCE-relative paths with BURD_BUK_DIR/BURD_TOOLS_DIR

## Acceptance

- All 20 CLIs follow Pattern B with buc_doc_env + gate
- All tests pass (regime-smoke, regime-credentials, nsproto-security, pluml-diagram, srjcl-jupyter) — skip ark-lifecycle
- Bottles must be started before running nameplate test suites

**[260224-0609] rough**

Migrate all Pattern A CLIs to Pattern B (furnish-sources) and add buc_doc_env + buc_doc_env_done gate to all CLIs.

## Context

BCG now prescribes Pattern B: CLI header sources only buc_command.sh; furnish sources all other dependencies. 11 CLIs still use Pattern A (header-sources). Additionally, most CLIs lack buc_doc_env calls and the buc_doc_env_done gate.

## Pattern A CLIs to migrate (move top-level sources into furnish)

1. rbw/rbgg_cli.sh
2. rbw/rbgm_cli.sh
3. rbw/rbv_cli.sh
4. rbw/rbq_cli.sh
5. buk/buut_cli.sh
6. vvk/vvb_cli.sh
7. rbw/rbrr_cli.sh
8. rbw/rbf_cli.sh
9. rbw/rbgp_cli.sh
10. rbw/rbob_cli.sh
11. vok/vob_cli.sh

## Pattern B CLIs needing buc_doc_env + gate (already furnish-sources)

1. rbw/rbra_cli.sh
2. rbw/rbrv_cli.sh (has doc_env, no gate)
3. buk/burc_cli.sh
4. rbw/rbro_cli.sh
5. rbw/rbrp_cli.sh
6. buk/burs_cli.sh
7. rbw/rbrn_cli.sh
8. buk/bure_cli.sh

(rbrs_cli.sh already done)

## Transformation pattern

For each CLI:
1. Keep only `source "${BURD_BUK_DIR}/buc_command.sh"` at top level
2. Move all other source statements into furnish, after buc_doc_env_done gate
3. Add buc_doc_env calls for BURD_BUK_DIR, BURD_TOOLS_DIR (and any other external env vars)
4. Add `buc_doc_env_done || return 0` gate after the env doc block
5. Replace any BASH_SOURCE-relative paths with BURD_BUK_DIR/BURD_TOOLS_DIR

## Acceptance

- All 20 CLIs follow Pattern B with buc_doc_env + gate
- All tests pass (regime-smoke, regime-credentials, nsproto-security, pluml-diagram, srjcl-jupyter) — skip ark-lifecycle
- Bottles must be started before running nameplate test suites

### drop-heat-level-swim-lane (₢AOAAR) [complete]

**[260224-0701] complete**

Remove the heat-level (`*`) commit swim lane from jjx_orient output.

## Reasoning

The orient display shows commit swim lanes per pace, plus a `*` lane for heat-level (non-pace-affiliated) commits. The `*` lane is noise for these reasons:

1. **Most heat-level commits are gallops bookkeeping** — jjx_enroll, jjx_mark, jjx_alter, jjx_close. These are infrastructure commits that record intentions, not work product. They don't help a developer understand what happened recently.

2. **Genuinely useful heat-level commits are rare.** When they exist, they'd show up in `jjx_log` or `git log` anyway.

3. **The lane is confusing.** It prompts "what happened there?" when the answer is usually "nothing interesting." It dilutes attention from the pace-affiliated lanes which actually tell the story of work done.

4. **The pace-affiliated lanes are sufficient.** They show which paces had work, how they interleaved, and the file-touch bitmap connects commits to files. The `*` lane adds no information that changes decisions.

## Scope

- Remove the `*` lane from the commit swim lane display in orient output
- Keep pace-affiliated lanes unchanged
- Keep the file-touch bitmap unchanged (it already has `*` for heat-level; evaluate whether to drop there too or leave it for completeness)

## Acceptance

- `jjx_orient` output shows only pace-affiliated swim lanes
- No `*` row in swim lane display
- File-touch bitmap decision documented (drop or keep `*` column)

**[260217-1758] rough**

Remove the heat-level (`*`) commit swim lane from jjx_orient output.

## Reasoning

The orient display shows commit swim lanes per pace, plus a `*` lane for heat-level (non-pace-affiliated) commits. The `*` lane is noise for these reasons:

1. **Most heat-level commits are gallops bookkeeping** — jjx_enroll, jjx_mark, jjx_alter, jjx_close. These are infrastructure commits that record intentions, not work product. They don't help a developer understand what happened recently.

2. **Genuinely useful heat-level commits are rare.** When they exist, they'd show up in `jjx_log` or `git log` anyway.

3. **The lane is confusing.** It prompts "what happened there?" when the answer is usually "nothing interesting." It dilutes attention from the pace-affiliated lanes which actually tell the story of work done.

4. **The pace-affiliated lanes are sufficient.** They show which paces had work, how they interleaved, and the file-touch bitmap connects commits to files. The `*` lane adds no information that changes decisions.

## Scope

- Remove the `*` lane from the commit swim lane display in orient output
- Keep pace-affiliated lanes unchanged
- Keep the file-touch bitmap unchanged (it already has `*` for heat-level; evaluate whether to drop there too or leave it for completeness)

## Acceptance

- `jjx_orient` output shows only pace-affiliated swim lanes
- No `*` row in swim lane display
- File-touch bitmap decision documented (drop or keep `*` column)

### fix-bcg-docmode-furnish (₢AOAAL) [abandoned]

**[260224-0609] abandoned**

Fix BCG doc-mode so furnish functions bail cleanly in doc mode.

## Problem

`buc_execute` calls `buc_set_doc_mode` then `zbuc_show_help`, which calls the furnish function. The `buc_doc_env` lines print correctly, but furnish continues into kindle/validation/sourcing and dies because env vars aren't set. Doc mode can never display a complete help screen unless all runtime env vars are present.

## Fix

Add a doc-mode early-exit gate after the `buc_doc_env` block in furnish functions. Either:
- Reuse existing `buc_doc_shown || return 0` after the env doc lines
- Or add a dedicated `buc_doc_env_done || return 0` if semantics differ

Then verify by invoking `rbf_cli.sh` with no command and no env vars — it should print the full help (env vars + all commands with briefs/params) and exit cleanly.

## Scope

1. Determine correct BCG gate pattern for furnish
2. Apply to `rbf_cli.sh` as the reference specimen
3. Verify doc-mode works end-to-end: `bash Tools/rbw/rbf_cli.sh` prints full help
4. Update BCG template if the pattern is new
5. Apply to all other furnish functions that use `buc_execute`

## Acceptance

- `bash Tools/rbw/rbf_cli.sh` (no args, no env) prints complete help and exits 1
- All CLIs using `buc_execute` have the gate in their furnish
- BCG template updated if pattern changed

**[260217-0803] rough**

Fix BCG doc-mode so furnish functions bail cleanly in doc mode.

## Problem

`buc_execute` calls `buc_set_doc_mode` then `zbuc_show_help`, which calls the furnish function. The `buc_doc_env` lines print correctly, but furnish continues into kindle/validation/sourcing and dies because env vars aren't set. Doc mode can never display a complete help screen unless all runtime env vars are present.

## Fix

Add a doc-mode early-exit gate after the `buc_doc_env` block in furnish functions. Either:
- Reuse existing `buc_doc_shown || return 0` after the env doc lines
- Or add a dedicated `buc_doc_env_done || return 0` if semantics differ

Then verify by invoking `rbf_cli.sh` with no command and no env vars — it should print the full help (env vars + all commands with briefs/params) and exit cleanly.

## Scope

1. Determine correct BCG gate pattern for furnish
2. Apply to `rbf_cli.sh` as the reference specimen
3. Verify doc-mode works end-to-end: `bash Tools/rbw/rbf_cli.sh` prints full help
4. Update BCG template if the pattern is new
5. Apply to all other furnish functions that use `buc_execute`

## Acceptance

- `bash Tools/rbw/rbf_cli.sh` (no args, no env) prints complete help and exits 1
- All CLIs using `buc_execute` have the gate in their furnish
- BCG template updated if pattern changed

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 P bcg-test-execution-vocabulary
  2 S rename-test-functions-bcg-vocabulary
  3 Q audit-but-against-test-vocab
  4 I repair-tabtarget-prescribed-form
  5 A extract-orphaned-heat-paces
  6 C refresh-register-colophons
  7 E buc-countdown-bypass-flag
  8 F strip-down-quota-build-guide
  9 G fix-rbgm-os-specific-link-instructions
  10 H untangle-rbgm-path-indirections
  11 M create-burd-regime-module
  12 O replace-dotdot-buk-with-burd-buk-dir
  13 J audit-cli-kindle-functions
  14 K bcg-modernize-rbrs-cli
  15 T bcg-reconcile-sourcing-in-furnish
  16 U migrate-cli-sourcing-to-furnish
  17 R drop-heat-level-swim-lane
  18 L fix-bcg-docmode-furnish

PSQIACEFGHMOJKTURL
···········xx··x·· rbq_cli.sh
··········xx···x·· rbrr_cli.sh, vob_cli.sh
·······xxx········ rbgm_ManualProcedures.sh
·x········xx······ rbtb_testbench.sh
·xx·······x······· butd_dispatch.sh, bute_engine.sh
·············x·x·· buc_command.sh
············x··x·· rbra_cli.sh, rbrn_cli.sh, rbro_cli.sh, rbrp_cli.sh, rbrv_cli.sh
············xx···· rbrs_cli.sh
···········x···x·· rbob_cli.sh, vvb_cli.sh
··········x····x·· burc_cli.sh, bure_cli.sh, burs_cli.sh, buut_cli.sh, rbf_cli.sh, rbgg_cli.sh
··········x···x··· rbw_workbench.sh
··········xx······ buw_workbench.sh, jjw_workbench.sh, vow_workbench.sh, vslw_workbench.sh, vvw_workbench.sh
···x·······x······ rbq_Qualify.sh
···x······x······· buut_tabtarget.sh
·x········x······· butcrg_RegimeSmoke.sh
·xx··············· but_test.sh
x·············x··· BCG-BashConsoleGuide.md
················x· jjrpd_parade.rs
···············x·· rbgm_cli.sh, rbgp_cli.sh, rbv_cli.sh
·············x···· AXLA-Lexicon.adoc
············x····· rbrr_regime.sh
···········x······ cmw_workbench.sh, jja_arcanum.sh, rbcr_render.sh
··········x······· RBSCB-CloudBuildRoadmap.adoc, burc_regime.sh, burd_regime.sh, cccw_workbench.sh, rbf_Foundry.sh, rbgg_Governor.sh, rbgo_OAuth.sh, rbi_Image.sh, vob_build.sh
·······x·········· RBSA-SpecTop.adoc, RBSQB-quota_build.adoc
···x·············· buv_validation.sh, buw-rcr.RenderConfigRegime.sh, buw-rcv.ValidateConfigRegime.sh, buw-rer.RenderEnvironmentRegime.sh, buw-rev.ValidateEnvironmentRegime.sh, buw-rsr.RenderStationRegime.sh, buw-rsv.ValidateStationRegime.sh, buw-tt-cbl.CreateTabTargetBatchLogging.sh, buw-tt-cbn.CreateTabTargetBatchNolog.sh, buw-tt-cil.CreateTabTargetInteractiveLogging.sh, buw-tt-cin.CreateTabTargetInteractiveNolog.sh, buw-tt-cl.CreateLauncher.sh, buw-tt-ll.ListLaunchers.sh, ccck-B.BuildContainer.sh, ccck-R.ResetContainer.sh, ccck-a.StartContainer.sh, ccck-c.ConnectCode.sh, ccck-g.ConnectGitStatus.sh, ccck-s.ConnectShell.sh, ccck-z.StopContainer.sh, cmk-i.InstallConceptModelKit.sh, cmk-u.UninstallConceptModelKit.sh, gadcf.LaunchFactoryInContainer.sh, gadf-f.Factory.sh, gadi-i.Inspect.sh, jja-c.Check.sh, jja-i.Install.sh, jja-u.Uninstall.sh, machine_setup_PROTOTYPE_rule.sh, oga.OpenGithubAction.sh, rbw-B.ConnectBottle.nsproto.sh, rbw-B.ConnectBottle.srjcl.sh, rbw-C.ConnectCenser.nsproto.sh, rbw-GD.GovernorDirectorCreate.sh, rbw-GR.GovernorRetrieverCreate.sh, rbw-GS.DeleteServiceAccount.sh, rbw-Gl.ListServiceAccounts.sh, rbw-PC.PayorDepotCreate.sh, rbw-PD.PayorDepotDestroy.sh, rbw-PE.PayorEstablishment.sh, rbw-PG.PayorGovernorReset.sh, rbw-PI.PayorInstall.sh, rbw-PR.PayorRefresh.sh, rbw-QB.QuotaBuild.sh, rbw-S.ConnectSentry.nsproto.sh, rbw-S.ConnectSentry.pluml.sh, rbw-S.ConnectSentry.srjcl.sh, rbw-aA.AbjureArk.sh, rbw-aC.ConjureArk.sh, rbw-ab.BeseechArk.sh, rbw-as.SummonArk.sh, rbw-hga.HelpGoogleAdmin.sh, rbw-him.HelpImageManagement.sh, rbw-iB.BuildImageRemotely.sh, rbw-iD.DeleteImage.sh, rbw-il.ImageList.sh, rbw-ir.RetrieveImage.sh, rbw-l.ListCurrentRegistryImages.sh, rbw-ld.ListDepots.sh, rbw-ni.NameplateInfo.sh, rbw-nv.ValidateNameplates.sh, rbw-o.ObserveNetworks.nsproto.sh, rbw-o.ObserveNetworks.pluml.sh, rbw-o.ObserveNetworks.srjcl.sh, rbw-ps.ShowPayorEstablishment.sh, rbw-qa.QualifyAll.sh, rbw-ral.ListAuthRegimes.sh, rbw-rar.RenderAuthRegime.sh, rbw-rav.ValidateAuthRegime.sh, rbw-rnr.RenderNameplateRegime.sh, rbw-rnv.ValidateNameplateRegime.sh, rbw-ror.RenderOauthRegime.sh, rbw-rov.ValidateOauthRegime.sh, rbw-rpr.RenderPayorRegime.sh, rbw-rpv.ValidatePayorRegime.sh, rbw-rrg.RefreshGcbPins.sh, rbw-rrr.RenderRepoRegime.sh, rbw-rrv.ValidateRepoRegime.sh, rbw-rsr.RenderStationRegime.sh, rbw-rsv.ValidateStationRegime.sh, rbw-rvr.RenderVesselRegime.sh, rbw-rvv.ValidateVesselRegime.sh, rbw-s.Start.nsproto.sh, rbw-s.Start.pluml.sh, rbw-s.Start.srjcl.sh, rbw-ta.TestAll.sh, rbw-tn.TestNameplate.nsproto.sh, rbw-tn.TestNameplate.pluml.sh, rbw-tn.TestNameplate.srjcl.sh, rbw-to.TestOne.sh, rbw-trc.TestRegimeCredentials.sh, rbw-trg.TestRegimeSmoke.sh, rbw-ts.TestSuite.sh, vow-F.Freshen.sh, vow-P.Parcel.sh, vow-R.Release.sh, vow-b.Build.sh, vow-c.Clean.sh, vow-r.RunVVX.sh, vow-t.Test.sh, vslk-i.InstallSlickEditProject.sh, vvk-T.RunTests.sh, vvw-r.RunVVX.sh
··x··············· butr_registry.sh
·x················ butcde_DispatchExercise.sh, butcrg_RegimeCredentials.sh, butcvu_XnameValidation.sh, rbtcal_ArkLifecycle.sh, rbtckk_KickTires.sh, rbtcns_NsproSecurity.sh, rbtcpl_PlumlDiagram.sh, rbtcqa_QualifyAll.sh, rbtcsj_SrjclJupyter.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 106 commits)

  1 S rename-test-functions-bcg-vocabulary
  2 Q audit-but-against-test-vocab
  3 M create-burd-regime-module
  4 O replace-dotdot-buk-with-burd-buk-dir
  5 J audit-cli-kindle-functions
  6 L fix-bcg-docmode-furnish
  7 K bcg-modernize-rbrs-cli
  8 T bcg-reconcile-sourcing-in-furnish
  9 U migrate-cli-sourcing-to-furnish
  10 R drop-heat-level-swim-lane

123456789abcdefghijklmnopqrstuvwxyz
xx·································  S  2c
··xxx······························  Q  3c
·····xxx···························  M  3c
········x··xxx·····················  O  4c
··············xx···················  J  2c
················x··················  L  1c
··················x·xxx············  K  4c
·······················xx··xx······  T  4c
·····························xxx···  U  3c
································xxx  R  3c
```

## Steeplechase

### 2026-02-24 07:01 - ₢AOAAR - W

Removed heat-level swim lane and bitmap column from orient display

### 2026-02-24 07:01 - ₢AOAAR - n

Remove heat-level swim lane from orient display and file-touch bitmap

### 2026-02-24 06:55 - ₢AOAAR - A

Remove heat-level swim lane from orient; drop bitmap column for consistency

### 2026-02-24 06:54 - ₢AOAAU - W

Fix buc_execute furnish-before-dispatch; migrate all 20 CLIs to Pattern B with buc_doc_env + gate

### 2026-02-24 06:54 - ₢AOAAU - n

Fix buc_execute to furnish before declare-F check; migrate 11 Pattern A CLIs + add gate to 8 Pattern B CLIs

### 2026-02-24 06:25 - ₢AOAAU - A

Direct sonnet agent: migrate 11 Pattern A CLIs + add gate to 8 Pattern B CLIs

### 2026-02-24 06:14 - ₢AOAAT - W

Converge BCG on Pattern B: furnish sources all deps; update 6 spec sections; fix qualification gate regression; slate migration pace; drop subsumed AOAAL

### 2026-02-24 06:09 - ₢AOAAT - n

Converge BCG on Pattern B: furnish sources all deps; update template, sourcing rules, maturity checklist; slate migration pace; drop subsumed AOAAL

### 2026-02-24 06:09 - Heat - T

fix-bcg-docmode-furnish

### 2026-02-24 06:09 - Heat - S

migrate-cli-sourcing-to-furnish

### 2026-02-24 06:06 - ₢AOAAT - n

Fix qualification gate: qualify_all -> rbq_qualify_all (regression from cli-kindle audit)

### 2026-02-24 05:56 - ₢AOAAT - A

Inventory Pattern A vs B CLIs; analyze doc-mode implications; recommend convergence

### 2026-02-24 05:55 - ₢AOAAK - W

Fix buc_doc_env doc-mode validation skip; add buc_doc_env_done furnish gate; modernize rbrs_cli.sh with env docs and gate

### 2026-02-24 05:55 - ₢AOAAK - n

Add GCP build trigger, developer connect, and SLSA provenance motifs to AXLA lexicon

### 2026-02-24 05:55 - ₢AOAAK - n

Fix buc_doc_env to skip validation in doc mode; add buc_doc_env_done gate; add env docs to rbrs_cli.sh furnish

### 2026-02-23 19:24 - Heat - S

bcg-reconcile-sourcing-in-furnish

### 2026-02-23 19:13 - ₢AOAAK - A

Interactive BCG compliance review of rbrs_cli.sh with user; capture cleanup list for follow-on pace

### 2026-02-23 19:13 - Heat - r

moved AOAAL to last

### 2026-02-23 19:05 - ₢AOAAL - A

Fix buc_doc_env to skip validation in doc mode; add buc_doc_env_done gate; apply to all 21 furnish functions; update BCG template

### 2026-02-23 19:03 - ₢AOAAJ - W

Replaced zrbq_cli_kindle with zrbq_furnish + buc_execute; zero _cli_kindle remaining; smoke tests pass

### 2026-02-23 19:03 - ₢AOAAJ - n

Replace zrbq_cli_kindle with zrbq_furnish + buc_execute pattern; eliminate last _cli_kindle conflation

### 2026-02-23 18:53 - ₢AOAAO - W

Replaced 38 ../buk/ and BURC_BUK_DIR references with BURD_BUK_DIR across 15 files; smoke tests pass

### 2026-02-23 18:53 - ₢AOAAO - n

Replace all ../buk/ relative paths with BURD_BUK_DIR across 15 files (36 ../buk/ + 2 stale BURC_BUK_DIR)

### 2026-02-23 18:50 - ₢AOAAO - F

Executing rough pace via haiku agent — mechanical ../buk/ to BURD_BUK_DIR replacement

### 2026-02-23 18:49 - Heat - T

replace-dotdot-buk-with-burd-buk-dir

### 2026-02-23 18:49 - Heat - T

replace-dotdot-buk-with-burc-buk-dir

### 2026-02-23 18:41 - ₢AOAAO - A

Mechanical replace ../buk/ with BURC_BUK_DIR across 20 files, 41 occurrences

### 2026-02-23 18:40 - ₢AOAAM - W

BURD regime module complete: kindle/sentinel/enforce wired across 19 files; ark-lifecycle regression traced to OAuth curl issue (not BURD sweep), tracked in APAAm

### 2026-02-23 18:40 - ₢AOAAM - n

Revise Cloud Build roadmap to per-vessel mini-repo approach with fully-baked yaml and zero custom substitutions

### 2026-02-18 08:14 - ₢AOAAM - A

Investigate ark-lifecycle regression: run with BURD_VERBOSE, trace dispatch chain

### 2026-02-17 18:33 - ₢AOAAQ - W

Audited 5 BUT infrastructure files against BCG test vocabulary: no boundary/contract violations found; updated comments to use _tsuite/_tcase vocabulary; removed dead but_execute shim

### 2026-02-17 18:33 - ₢AOAAQ - n

Remove dead `but_execute` shim; update BUK test comments to use _tsuite/_tcase boundary vocabulary

### 2026-02-17 18:26 - ₢AOAAQ - A

Comment vocabulary update + dead shim removal; no structural changes

### 2026-02-17 18:24 - ₢AOAAS - W

Renamed 51 case functions (_tcase suffix), 10 setup functions (_tsuite_setup suffix), 1 infrastructure function (zbute_tcase) across 12 files; fixed parallel-agent overlap in enrollment sites; all acceptance greps clean, 8/8 rename-related test suites green

### 2026-02-17 18:24 - ₢AOAAS - n

Rename test functions to _tcase/_tsuite vocabulary per BCG naming conventions

### 2026-02-17 18:13 - ₢AOAAS - A

Parallel sonnet x2: rbw renames + buk renames, then verify

### 2026-02-17 18:11 - ₢AOAAP - W

Added _tsuite/_tcase vocabulary, three-layer isolation model, naming conventions, and compliance checking section to BCG

### 2026-02-17 18:11 - ₢AOAAP - n

Add test execution vocabulary section to BCG: _tsuite, _tcase, three-layer isolation model, naming conventions, compliance checking

### 2026-02-17 18:10 - Heat - S

rename-test-functions-bcg-vocabulary

### 2026-02-17 18:00 - ₢AOAAP - A

Sequential opus: add _tsuite/_tcase vocabulary section to BCG between Maturity Checklist and Fading Memory, cross-ref subshell prohibition

### 2026-02-17 17:58 - Heat - S

drop-heat-level-swim-lane

### 2026-02-17 17:54 - ₢AOAAP - A

Interactive opus: read BCG + BUT code, draft test execution vocabulary section

### 2026-02-17 17:52 - Heat - S

audit-but-against-test-vocab

### 2026-02-17 17:52 - Heat - S

bcg-test-execution-vocabulary

### 2026-02-17 17:44 - ₢AOAAM - n

Isolate suites via subshell in butd_dispatch; prevent kindle/dispatch state bleeding between suites

### 2026-02-17 17:35 - ₢AOAAM - n

Remove spurious bute_init_dispatch/bute_init_evidence from zrbtb_setup_ark; ark-lifecycle uses buto_tt not bute_dispatch

### 2026-02-17 17:11 - ₢AOAAM - A

Debug ark-lifecycle regression: trace BURD sweep impact on rbf_cli/rbf_Foundry chain via verbose test run

### 2026-02-17 17:10 - Heat - T

create-burd-regime-module

### 2026-02-17 17:08 - ₢AOAAM - n

Hoist zbuz/zrbz kindle to testbench top level; fix double-kindle across sequential suites in test-all

### 2026-02-17 17:07 - ₢AOAAM - n

Fix double-kindle of rbcc in rbtb_testbench qualify-all setup

### 2026-02-17 17:06 - ₢AOAAJ - n

Audit and update CLI kindle functions across rbw regime CLIs

### 2026-02-17 17:05 - ₢AOAAM - n

Sweep 19 files replacing ad-hoc BURD checks with zburd_kindle/sentinel; add burd_regime.sh sourcing to CLI wrappers

### 2026-02-17 16:50 - ₢AOAAM - A

Sweep 16 files replacing ad-hoc BURD checks with zburd_kindle/sentinel; sequential haiku; batch commit + test

### 2026-02-17 16:49 - Heat - S

replace-dotdot-buk-with-burc-buk-dir

### 2026-02-17 16:48 - Heat - T

create-burd-regime-module

### 2026-02-17 16:47 - ₢AOAAM - n

Wire burd_regime into rbw_workbench, rbtb_testbench, butcrg_RegimeSmoke; fix log path validation for non-exported vars

### 2026-02-17 16:43 - ₢AOAAM - n

Add BURC_BUK_DIR derived constant; use it in buw_workbench.sh for burd_regime.sh sourcing

### 2026-02-17 16:37 - ₢AOAAM - n

Wire burd_regime.sh into buw_workbench.sh; replace ad-hoc BURD checks with kindle+sentinel; fix bure_cli.sh execute bit

### 2026-02-17 16:34 - ₢AOAAM - n

Create burd_regime.sh with kindle/sentinel/validate/render for BURD dispatch runtime regime

### 2026-02-17 16:29 - ₢AOAAM - A

Create burd_regime.sh interactively, then parallel-sweep ad-hoc checks

### 2026-02-17 16:28 - Heat - T

create-burd-regime-module

### 2026-02-17 16:26 - Heat - T

create-burd-regime-module

### 2026-02-17 16:25 - Heat - S

relocate-ambient-burd-vars-to-bure

### 2026-02-17 16:21 - Heat - S

create-burd-regime-module

### 2026-02-17 15:52 - ₢AOAAJ - A

Delete 7 degenerate cli_kindle fns; inline 4 non-trivial ones; remove all CLI_KINDLED flags

### 2026-02-17 15:50 - ₢AOAAH - W

Replaced cd/../.. path gymnastics with RBCC_RBRP_FILE and RBCC_RBRR_FILE constants in zrbgm_kindle(); removed ITCH_LINK_TO_RBL comment

### 2026-02-17 15:50 - ₢AOAAH - n

Replace cd/../.. path gymnastics with RBCC_RBRP_FILE and RBCC_RBRR_FILE constants in zrbgm_kindle()

### 2026-02-17 15:49 - ₢AOAAH - F

Executing via haiku agent - replace path indirections with RBCC constants

### 2026-02-17 15:49 - ₢AOAAH - A

Replace cd/../.. path gymnastics with RBCC_RBRP_FILE and RBCC_RBRR_FILE constants in zrbgm_kindle()

### 2026-02-17 15:49 - Heat - T

untangle-rbgm-path-indirections

### 2026-02-17 15:45 - ₢AOAAG - W

Added OS-detected click modifier (Cmd/Ctrl) in zrbgm_kindle; removed redundant color line; fixed both Key sections in payor_establish and quota_build

### 2026-02-17 15:45 - ₢AOAAG - n

OS-detect click modifier in kindle; replace hardcoded Ctrl with Cmd/Ctrl per platform; remove redundant color key line

### 2026-02-17 15:43 - ₢AOAAG - A

OS-detect click modifier in kindle; remove redundant color line; fix both Key sections

### 2026-02-17 15:42 - ₢AOAAF - W

Reframed RBSQB quota build guide as capacity health-check: machine type is primary lever, quota increase demoted to last-resort footnote, Option A/B framing removed; mirrored in rbgm CLI function; updated RBSA cross-reference

### 2026-02-17 15:42 - ₢AOAAF - n

Reframe quota build guide as capacity health-check: keep tables+nav, demote quota-increase to footnote, remove Option A/B framing; mirror in rbgm CLI function; update RBSA cross-ref

### 2026-02-17 15:40 - ₢AOAAF - F

Executing via haiku agent - reframe quota build guide as capacity health-check

### 2026-02-17 15:40 - ₢AOAAF - A

Reframe RBSQB as capacity health-check: keep tables+nav, demote quota-increase to footnote, remove Option A/B framing; mirror in rbgm CLI function; update RBSA cross-ref

### 2026-02-17 08:03 - Heat - S

fix-bcg-docmode-furnish

### 2026-02-17 07:50 - Heat - S

bcg-modernize-rbrs-cli

### 2026-02-17 07:48 - Heat - S

audit-cli-kindle-functions

### 2026-02-17 07:40 - ₢AOAAI - W

Deleted 20 non-standard tabtargets, converted 2 to BURD form, added glob-based exemption support to buv_qualify_tabtargets; qualification passes clean (82 checked, 1 exempt)

### 2026-02-17 07:40 - ₢AOAAI - n

Add tabtarget exemption pattern support to buv_qualify_tabtargets; delete 22 non-standard legacy tabtargets; update 2 surviving tabtargets to prescribed form

### 2026-02-17 07:20 - Heat - T

repair-tabtarget-prescribed-form

### 2026-02-17 07:09 - ₢AOAAI - n

Fix generator and regenerate 79 standard tabtargets: dirname to parameter expansion in exec line

### 2026-02-17 07:06 - ₢AOAAI - A

Fix generator, bootstrap creators, regenerate 79 standard TTs; delete 2 dead legacy; investigate 22 non-standard

### 2026-02-17 07:02 - Heat - S

repair-tabtarget-prescribed-form

### 2026-02-17 07:02 - Heat - n

Strict tabtarget qualification: enforce prescribed form (BURD_LAUNCHER, parameter expansion exec, no dirname)

### 2026-02-17 06:45 - ₢AOAAE - W

Added BURE ambient environment regime with BURE_COUNTDOWN=skip override, enum validator (buv_val_enum), bure_regime.sh, bure_cli.sh, BUSA spec terms, buw workbench routes, and tabtargets

### 2026-02-17 06:13 - ₢AOAAE - A

Auto-detect /dev/tty + BUC_NONINTERACTIVE env var; print warning to stderr when skipping; doc in README

### 2026-02-16 10:47 - ₢AOAAE - A

Auto-detect non-interactive context via /dev/tty check + BUC_NONINTERACTIVE env var override in buc_countdown

### 2026-02-16 10:46 - ₢AOAAC - W

Already complete — _register→_enroll migration finished in prior work

### 2026-02-16 10:39 - Heat - T

detect-macos-app-permissions

### 2026-02-16 10:34 - ₢AOAAA - W

Triaged 8 orphaned items: slated 2 as paces (AOAAG, AOAAH), added 2 to itch, dropped 4 as stale/fixed, deleted orphaned heat file

### 2026-02-16 10:33 - ₢AOAAA - n

Triage orphaned heat: slate #5 #7 as paces, add #2 #8 to itch, drop stale items, delete orphaned file

### 2026-02-16 10:31 - Heat - S

untangle-rbgm-path-indirections

### 2026-02-16 10:31 - Heat - S

fix-rbgm-os-specific-link-instructions

### 2026-02-16 10:10 - Heat - S

strip-down-quota-build-guide

### 2026-02-16 10:09 - Heat - S

buc-countdown-bypass-flag

### 2026-02-16 10:09 - Heat - S

detect-macos-app-permissions

### 2026-02-16 09:52 - ₢AOAAA - A

Triage 8 orphaned items: slate relevant as paces or itch, delete orphaned file

### 2026-02-11 14:06 - Heat - S

refresh-register-colophons

### 2026-02-10 13:12 - Heat - D

restring 1 paces from ₣AH

### 2026-01-25 13:42 - Heat - f

silks=rbw-misc-bash-cleanup

### 2026-01-25 08:28 - Heat - S

extract-orphaned-heat-paces

### 2026-01-25 08:14 - Heat - f

racing

### 2026-01-25 08:10 - Heat - N

rbm-misc-bash-cleanup

