# Steeplechase: Dockerize Bashize Proto Bottle

---
### 2025-12-29 09:45 - modernize-rbrn-regime - APPROACH
**Proposed approach**:
- Add multiple-inclusion guard at top of file using `ZRBRN_SOURCED` pattern
- Create `zrbrn_kindle()` function wrapping validation, with defaults for optional fields
- Create `zrbrn_sentinel()` function for kindle verification
- Remove direct sourcing of buv_validation.sh (caller provides BUV)
---
### 2025-12-29 10:15 - modernize-rbrn-regime - WRAP
**Outcome**: Modernized with kindle/sentinel, defaults for optional/conditional fields, ZRBRN_ROLLUP; created rbrn_cli.sh
---
### 2025-12-29 10:20 - modernize-rbrv-regime - APPROACH
**Proposed approach**:
- Add multiple-inclusion guard using `ZRBRV_SOURCED` pattern
- Set defaults for all optional/conditional fields before validation
- Wrap validation in `zrbrv_kindle()` preserving conditional logic
- Add `zrbrv_sentinel()` and ZRBRV_ROLLUP
- Create rbrv_cli.sh with validate, render, info commands (like rbrn_cli.sh)
---
### 2025-12-29 10:30 - modernize-rbrv-regime - WRAP
**Outcome**: Modernized with kindle/sentinel, defaults, ZRBRV_ROLLUP; created rbrv_cli.sh
---
### 2025-12-29 11:00 - create-rbrn-nsproto-env - WRAP
**Outcome**: Added RBRN_RUNTIME to spec/regime/cli; fixed buv validators (${N-} pattern); created nsproto.env
---
### 2025-12-29 11:30 - create-rbw-workbench-skeleton - WRAP
**Outcome**: Created workbench with load_nameplate, runtime_cmd, stub commands following vslw/buw pattern
---
### 2025-12-29 11:03 - create-bud-launchers - APPROACH
**Proposed approach**:
- Create `.buk/launcher.rbw_workbench.sh` following buw_workbench pattern
- Create `.buk/launcher.rbt_testbench.sh` with same pattern (coordinator ready for future pace)
- Verify rbw launcher works (reaches stub), rbt launcher loads correctly
---
### 2025-12-29 11:03 - create-bud-launchers - WRAP
**Outcome**: Created both launchers; rbw verified working (reaches stub); rbt loads correctly (fails at missing coordinator as expected)
---
### 2025-12-29 11:10 - implement-local-recipe-build - APPROACH
**Proposed approach**:
- Implement `rbw_cmd_local_build` in rbw_workbench.sh (validate recipe, docker build, tag with local-timestamp)
- Create sample tabtarget `tt/rbw-lB.LocalBuild.test_busybox.sh` using launcher pattern
- Defer busybox validation to next pace, per-recipe tabtargets to later
---
### 2025-12-29 11:15 - implement-local-recipe-build - WRAP
**Outcome**: Implemented rbw_cmd_local_build; created test_busybox tabtarget
---
### 2025-12-29 11:18 - validate-docker-with-busybox - APPROACH
**Proposed approach**:
- Run tt/rbw-lB.LocalBuild.test_busybox.sh tabtarget
- Verify build succeeds and image appears in docker images
- Debug/fix any issues
---
### 2025-12-29 11:20 - validate-docker-with-busybox - WRAP
**Outcome**: Full flow validated; test_busybox:local-* image built successfully (6.27MB)
---
### 2025-12-29 11:25 - design-rbrr-env-injection - APPROACH
**Proposed approach**:
- Add ZRBRN_DOCKER_ENV array to rbrn_regime.sh (parallel to existing ZRBRN_ROLLUP string)
- Add ZRBRR_DOCKER_ENV array to rbrr_regime.sh (currently just RBRR_DNS_SERVER)
- Arrays built at kindle time, used via "${ARRAY[@]}" expansion in docker run
- Keep both patterns: string rollup for shell eval, array for container injection
---
### 2025-12-29 11:30 - design-rbrr-env-injection - WRAP
**Outcome**: Added ZRBRN_DOCKER_ENV (22 vars) and ZRBRR_DOCKER_ENV (1 var) arrays to regime kindle functions
---
### 2025-12-29 11:45 - implement-rbw-start - APPROACH
**Proposed approach**:
- Add workbench deps: source rbl_Locator.sh, rbrr_regime.sh; add rbw_load_rbrr() helper
- Implement container naming from moniker (e.g., ${RBRN_MONIKER}-sentry)
- Implement cleanup helper rbw_cleanup_containers() for stop/rm
- Implement network setup: create enclave network with subnet
- Implement sentry launch with "${ZRBRN_DOCKER_ENV[@]}" "${ZRBRR_DOCKER_ENV[@]}"
- Implement sentry config: exec rbss.sentry.sh via stdin pipe
- Implement censer launch and networking config
- Implement bottle create/start with --net=container:censer
---
### 2025-12-30 09:15 - implement-rbw-start - WRAP
**Outcome**: Created rbob_bottle.sh with full container lifecycle: cleanup, network creation, sentry/censer/bottle launch
---
### 2025-12-30 09:30 - modernize-rbob-to-bcg-pattern - APPROACH
**Proposed approach**:
- Add zrbob_kindle/sentinel to rbob_bottle.sh
- Move rbw_load_rbrr() into RBOB as zrbob_load_rbrr()
- Remove duplicate rbw_runtime_cmd() from workbench
- Create rbob_cli.sh with info command
---
### 2025-12-30 09:40 - modernize-rbob-to-bcg-pattern - BLOCKED
**Issue**: RBRR config loading design conflict with parallel heat work.
- Created zrbob_load_rbrr() in RBOB, but per BCG pattern config sourcing belongs in furnish/workbench
- RBOB kindle should just validate regimes are already kindled (zrbrr_sentinel), not load config
- Parallel heat is modifying RBRR regime; need to embrace those repairs before continuing
**Resume**: Remove zrbob_load_rbrr() from RBOB, move config loading to workbench's rbw_load_nameplate
---
### 2025-12-30 10:00 - modernize-rbob-to-bcg-pattern - APPROACH
**Proposed approach**:
- Remove `zrbob_load_rbrr()` from `rbob_bottle.sh` - per BCG, config loading belongs in caller
- Inline RBRR loading directly in `rbw_workbench.sh`'s `rbw_load_nameplate()`
- Update `rbob_cli.sh` to inline RBRR loading in `zrbob_furnish()`
- Add `validate` command to `rbob_cli.sh` (validates moniker produces runnable config)
- RBOB kindle calls `zrbrr_sentinel` to verify RBRR is ready (not load it)
---
### 2025-12-30 10:50 - modernize-rbob-to-bcg-pattern - WRAP
**Outcome**: Moved RBRR loading from RBOB to callers (workbench/furnish); added rbob_validate command
---
### 2025-12-30 11:05 - implement-rbw-stop - WRAP
**Outcome**: Added rbob_stop(); refactored workbench to two-phase routing; created tt/rbw-z.Stop.nsproto.sh
---
### 2025-12-30 11:30 - implement-rbw-connect-commands - WRAP
**Outcome**: Added connect functions; refactored RBOB to kindle pattern (compute all derived values once, no subshells)
---
### 2025-12-30 12:00 - migrate-lifecycle-tabtargets - APPROACH
**Proposed approach**:
- Merge "Validate bottle lifecycle" pace into "Migrate lifecycle tabtargets" (validation becomes part of testing)
- Migrate 4 tabtargets from mbd.dispatch to BUD launcher: Start, ConnectSentry, ConnectCenser, ConnectBottle
- Test full lifecycle: start → connect to each → stop
---
### 2025-12-30 12:30 - migrate-lifecycle-tabtargets - WRAP
**Outcome**: Merged paces; migrated 4 tabtargets (Start, ConnectSentry, ConnectCenser, ConnectBottle) to BUD launcher; full lifecycle validated (start→connect→stop)
---
### 2025-12-30 13:00 - implement-rbw-observe-partial - APPROACH
**Proposed approach**:
- Create `rboo_observe.sh` as dedicated observe module with kindle/sentinel pattern
- Implement sentry (eth1) and censer (eth0) tcpdump captures for Docker
- Runtime-conditional bridge capture (podman only, via `podman machine ssh`)
- Add `rbw-o` routing to workbench
- Update `tt/rbw-o.ObserveNetworks.nsproto.sh` to BUD launcher
---
### 2025-12-30 13:15 - implement-rbw-observe-partial - WRAP
**Outcome**: Created rboo_observe.sh with kindle pattern; sentry/censer captures for Docker; runtime-conditional bridge for podman; tabtarget migrated; testing deferred (interactive)
---
(execution log begins here)
