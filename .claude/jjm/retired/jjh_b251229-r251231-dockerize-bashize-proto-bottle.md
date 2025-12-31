# Heat: Dockerize Bashize Proto Bottle

## Paddock

### Goal

Migrate the bottle/sentry/censer container lifecycle from Makefile (`rbp.podman.mk`) to bash (`rbw_workbench.sh`), validated with Docker as the initial runtime. Preserve the hard-won censer network namespace model. Migrate all three nameplate test suites (nsproto, srjcl, pluml) to bash (`rbt_testbench.sh`), with nsproto first as the prototype.

### Approach

**Runtime abstraction first**: Add `RBRN_RUNTIME=docker` to configuration (podman support deferred to future heat). Bash code uses this to select runtime command. Docker validation avoids podman VM complexity.

**Vertical slice on nsproto**: Complete end-to-end migration for nsproto nameplate before generalizing. Nsproto is simplest vessel and exercises security architecture most thoroughly (20+ tests).

**Preserve, don't delete**: Makefile code that isn't rehosted stays in place. Only remove rules that are successfully migrated and validated. Podman VM machinery (`rbp_podman_machine_*`, stash/pin logic) remains untouched for future heat.

**Test migration is critical**: The security tests validate the censer model works. Migrating them to bash proves Docker runtime is viable.

### Key Architectural Decisions

1. **Configuration format**: Nameplates move from `nameplate.*.mk` (Makefile syntax) to `rbrn_<moniker>.env` (bash-sourceable)
   - Specification: `lenses/rbw-RBRN-RegimeNameplate.adoc`
   - CLI: `Tools/rbw/rbrn_cli.sh` (validate, render, info commands)

2. **Runtime parameter**: `RBRN_RUNTIME=docker|podman` in nameplate config; workbench abstracts the difference

3. **Workbench pattern**: `rbw_workbench.sh` follows `buw_workbench.sh` pattern - case-based routing, `buc_*` output functions

4. **Test architecture**: Single `rbt_testbench.sh` uses only `but_test.sh` utilities; invoked via tabtargets with moniker token; routes to nameplate-specific test functions

5. **Coordinator absorption**: `rbk_Coordinator.sh` routing logic merges into `rbw_workbench.sh`

### The Censer Model (Reference)

Three-tier container architecture for network isolation:

```
bridge network <-- SENTRY (privileged, dual-homed) --> enclave network
                        |                                    |
                        |                              CENSER (sleep infinity)
                        |                                    |
                        |                              BOTTLE (--net=container:censer)
                        v
                   host/internet (filtered by sentry iptables)
```

- **Sentry**: Privileged container on bridge + enclave networks; runs iptables/dnsmasq for filtering
- **Censer**: Staging container on enclave; provides network namespace for bottle
- **Bottle**: Application container sharing censer's network namespace; all traffic routes through sentry

### Deferred Strands Guidance

**IMPORTANT**: This heat deliberately defers podman VM machinery. As paces complete, maintain awareness of what remains unported. Mid-heat decisions may affect the eventual podman VM heat.

Track in this section as the heat progresses:

**Deferred (do not port in this heat):**
- `rbp_podman_machine_start_rule` - VM startup (rbp.podman.mk:181-201)
- `rbp_podman_machine_stop_rule` - VM shutdown (rbp.podman.mk:203-206)
- `rbp_podman_machine_nuke_rule` - VM removal (rbp.podman.mk:208-213)
- `rbp_stash_check_rule` - VM image acquisition/validation (rbp.podman.mk:62-107, marked DEFERRED)
- `rbp_stash_update_rule` - VM image pinning to controlled version (rbp.podman.mk:138-179, marked DEFERRED)
- `RBM_MACHINE`, `RBM_CONNECTION` variables - podman machine connection params (rbp.podman.mk:25-26)
- `zRBM_PODMAN_SSH_CMD`, `zRBM_PODMAN_SHELL_CMD` - SSH into podman VM (rbp.podman.mk:37-38)
- `zRBM_EXPORT_ENV` - environment variable rollup for VM exec (rbp.podman.mk:29-34)
- `rbo.observe.sh` - uses `podman machine ssh` to capture bridge traffic; no Docker equivalent
- `rbp_check_connection` - validates podman VM connection (rbp.podman.mk:215-218) - kept for VM machinery
- Makefile wrapper rules: `rbw-a.%` (VM start), `rbw-z.%` (VM stop), `rbw-Z.%` (VM nuke) - still functional
- Makefile VM management rules: `rbw-c.%`, `rbw-m.%`, `rbw-f.%`, `rbw-i.%`, `rbw-N.%`, `rbw-e.%` (rbw.workbench.mk:98-116)

**Ported to bash (completed in this heat):**
- Nameplate config: `nameplate.*.mk` → `rbrn_*.env` (bash-sourceable format)
- Lifecycle operations: `rbp_start_service_rule` → `rbob_start()` in rbob_bottle.sh
- Stop operation: (no Makefile equivalent) → `rbob_stop()` in rbob_bottle.sh
- Connect operations: `rbp_connect_*_rule` → `rbob_connect_*()` in rbob_bottle.sh
- Observe operation: `rbp_observe_networks_rule` → `rboo_observe()` in rboo_observe.sh
- Test execution: `rbt_test_bottle_service_rule` + `MBT_PODMAN_*` macros → `rbt_suite_*()` + `rbt_exec_*()` in rbt_testbench.sh
- All lifecycle tabtargets: migrated from `mbd.dispatch.sh` to BUD launcher pattern (17 tabtargets across 3 nameplates)
- Runtime abstraction: `RBRN_RUNTIME=docker|podman` with `rbw_runtime_cmd()` in rbob_bottle.sh
- Full test suites: nsproto (22 tests), srjcl (3 tests), pluml (5 tests)

**Notes for future heats:**
- Parallel test execution: Original Makefile `rbw-tb.%` runs tests sequentially per nameplate but could parallelize across nameplates. Batch test automation deferred (individual tests work perfectly via tabtargets).
- Podman runtime support: Architecture is runtime-agnostic. Future heat should add `RBRN_RUNTIME=podman` support by implementing VM lifecycle in bash, then testing with all three nameplates.
- Bridge observation: Docker has no equivalent to `podman machine ssh` for bridge captures. Podman heat needs different approach (possibly docker exec into special diagnostic container, or accept bridge observation unavailable on Docker).

### Testing Insights (Learned from nsproto migration)

**Docker vs Podman differences:**
- Docker `--internal` network flag blocks ALL egress, even through dual-homed containers. Podman allows forwarding. Solution: omit `--internal` for Docker, rely on sentry's iptables for isolation.
- ICMP traceroute behaves differently: podman shows sentry at hop 1, Docker shows `* * *` (blocked). Tests should accept either behavior.

**Test patterns:**
- `but_expect_ok` for commands expected to succeed
- `but_expect_fatal` for commands expected to fail (security blocks)
- Use `rbt_exec_*_i` variants (with `-i` flag) for stdin-consuming commands: `dig`, `traceroute`, `apt-get`
- Use `rbt_exec_*` (no `-i`) for simple output commands: `nslookup`, `nc`, `ping`, `ps`

**Debugging workflow:**
- Single-test parameter: `rbt-to nsproto test_nsproto_dns_allow_anthropic` runs one test
- Run `rboo observe` in background during tests: start with `run_in_background`, run test, kill, check TaskOutput for tcpdump traces
- Network captures show exact packet flow through censer model

**Test structure in testbench:**
- Test functions named `test_<moniker>_<description>()` - prefix determines which suite runs them
- `but_execute "${BUD_TEMP_DIR}/tests" "test_nsproto_" "${single_test}"` - needs empty subdir (BUD_TEMP_DIR has transcript.txt)
- Tests can capture output: `z_output=$(rbt_exec_bottle_i cmd 2>&1)` then grep/check

**Ported items (update from "none yet"):**
- nsproto lifecycle tabtargets (Start, ConnectSentry, ConnectCenser, ConnectBottle, Stop, Observe)
- nsproto test tabtarget (TestBottleService)
- Full nsproto security test suite (22 tests)

### File Inventory

| File | Status | Notes |
|------|--------|-------|
| **Workbench & Config (new)** |||
| `Tools/rbw/rbw_workbench.sh` | create | Container lifecycle workbench |
| `Tools/rbw/rbt_testbench.sh` | create | Test runner using but_test.sh |
| `Tools/rbw/rbrn_nsproto.env` | create | Bash-sourceable nameplate config |
| `Tools/rbw/rbrn_srjcl.env` | create | Bash-sourceable nameplate config |
| `Tools/rbw/rbrn_pluml.env` | create | Bash-sourceable nameplate config |
| `Tools/rbw/rbrn_cli.sh` | create | CLI for nameplate operations |
| **Tabtargets - nsproto (modify to BUD launcher)** |||
| `tt/rbw-s.Start.nsproto.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-S.ConnectSentry.nsproto.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-C.ConnectCenser.nsproto.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-B.ConnectBottle.nsproto.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-o.ObserveNetworks.nsproto.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-to.TestBottleService.nsproto.sh` | modify | Change mbd.dispatch → rbt_testbench launcher |
| `tt/rbw-z.Stop.nsproto.sh` | create | Stop service (no existing equivalent) |
| **Tabtargets - srjcl (modify to BUD launcher)** |||
| `tt/rbw-s.Start.srjcl.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-S.ConnectSentry.srjcl.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-B.ConnectBottle.srjcl.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-o.ObserveNetworks.srjcl.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-to.TestBottleService.srjcl.sh` | modify | Change mbd.dispatch → rbt_testbench launcher |
| `tt/rbw-z.Stop.srjcl.sh` | create | Stop service (no existing equivalent) |
| **Tabtargets - pluml (modify to BUD launcher)** |||
| `tt/rbw-s.Start.pluml.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-S.ConnectSentry.pluml.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-o.ObserveNetworks.pluml.sh` | modify | Change mbd.dispatch → rbw_workbench launcher |
| `tt/rbw-to.TestBottleService.pluml.sh` | modify | Change mbd.dispatch → rbt_testbench launcher |
| `tt/rbw-z.Stop.pluml.sh` | create | Stop service (no existing equivalent) |
| **Launcher (new)** |||
| `.buk/launcher.rbw_workbench.sh` | create | BUD launcher for rbw_workbench |
| `.buk/launcher.rbt_testbench.sh` | create | BUD launcher for rbt_testbench |
| **Makefile (modify)** |||
| `Tools/rbw/rbp.podman.mk` | modify | Remove ported lifecycle rules; keep VM machinery |
| `Tools/rbw/rbw.workbench.mk` | modify | Remove MBT_* test helper macros |
| `Tools/rbw/rbk_Coordinator.sh` | modify | Absorb lifecycle routes into rbw_workbench.sh |
| **Makefile (reference only)** |||
| `RBM-nameplates/nameplate.nsproto.mk` | reference | Source for rbrn_nsproto.env conversion |
| `RBM-nameplates/nameplate.srjcl.mk` | reference | Source for rbrn_srjcl.env conversion |
| `RBM-nameplates/nameplate.pluml.mk` | reference | Source for rbrn_pluml.env conversion |
| `RBM-tests/rbt.test.nsproto.mk` | reference | Source for test migration (20+ security tests) |
| `RBM-tests/rbt.test.srjcl.mk` | reference | Source for test migration (Jupyter service tests) |
| `RBM-tests/rbt.test.srjcl.py` | reference | Python test script for Jupyter WebSocket |
| `RBM-tests/rbt.test.pluml.mk` | reference | Source for test migration (PlantUML tests) |
| **BUK utilities (unchanged)** |||
| `Tools/buk/but_test.sh` | unchanged | Test framework (sourced by testbench) |
| `Tools/buk/buc_command.sh` | unchanged | Output utilities (sourced by workbench) |
| `Tools/buk/buv_validation.sh` | unchanged | Validation (sourced by regime) |
| **RBW utilities** |||
| `Tools/rbw/rbrn_regime.sh` | modify | Modernize to kindle/sentinel pattern |
| `Tools/rbw/rbss.sentry.sh` | unchanged | Sentry iptables config (exec'd into container) |
| **Recipes (reference only)** |||
| `RBM-recipes/*.recipe` | reference | Dockerfiles for sentry/bottle/test images |

### Runtime Command Abstraction Pattern

```bash
rbw_runtime_cmd() {
  case "${RBRN_RUNTIME}" in
    docker) echo "docker" ;;
    podman) echo "podman -c ${RBRN_MACHINE}" ;;
    *) buc_die "Unknown runtime: ${RBRN_RUNTIME}" ;;
  esac
}
```

## Done

- **Modernize rbrn_regime.sh** — Modernized with kindle/sentinel, defaults for optional/conditional fields, ZRBRN_ROLLUP; created rbrn_cli.sh
- **Modernize rbrv_regime.sh** — Modernized with kindle/sentinel, defaults, ZRBRV_ROLLUP; created rbrv_cli.sh
- **Create rbrn_nsproto.env** — Added RBRN_RUNTIME to spec/regime/cli; fixed buv validators; created nsproto.env
- **Create rbw_workbench.sh skeleton** — Created workbench with load_nameplate, runtime_cmd, stub commands
- **Create BUD launchers** — Created launcher.rbw_workbench.sh and launcher.rbt_testbench.sh for BUD dispatch
- **Implement local recipe build** — Implemented rbw_cmd_local_build; created test_busybox tabtarget
- **Validate Docker with busybox** — Full flow validated: tabtarget→launcher→workbench→docker build; image created successfully
- **Design RBRR env injection for sentry** — Added ZRBRN_DOCKER_ENV and ZRBRR_DOCKER_ENV arrays to regime kindle functions
- **Implement rbw-start** — Created rbob_bottle.sh with full container lifecycle: cleanup, network creation, sentry/censer/bottle launch
- **Modernize RBOB to BCG pattern** — Moved RBRR loading from RBOB to callers (workbench/furnish); added rbob_validate command
- **Implement rbw-stop** — Added rbob_stop(); refactored workbench to two-phase routing; created tt/rbw-z.Stop.nsproto.sh
- **Implement rbw-connect commands** — Added connect functions; refactored RBOB to kindle pattern (compute all derived values once, no subshells)
- **Migrate lifecycle tabtargets** — Merged with "Validate bottle lifecycle"; migrated 4 tabtargets (Start, ConnectSentry, ConnectCenser, ConnectBottle) to BUD launcher; full lifecycle validated
- **Implement rbw-observe (partial)** — Created rboo_observe.sh with kindle pattern; sentry/censer captures for Docker; runtime-conditional bridge for podman; tabtarget migrated
- **Determine exec -i requirements** — Pattern: `-i` for stdin-reading commands (dig, traceroute, apt-get); no `-i` for simple output (nslookup, nc, ping, ps). Testbench provides both variants.
- **Create rbt_testbench.sh skeleton** — Created with exec helpers (6 functions), nameplate loading via zrbob_kindle, routing to suite placeholders.
- **Single test end-to-end** 🎉 — FIRST LIGHT! Full stack validated: launcher→testbench→kindle→but_execute→docker exec→DNS. Fixed but_execute temp dir issue. Network captures prove censer model works on Docker.
- **Migrate nsproto security tests** — All 22 tests passing. Fixed Docker --internal network flag (blocks forwarding unlike podman). Added single-test parameter for targeted runs. Adjusted ICMP test for Docker/podman differences.
- **Migrate test tabtarget** — Updated `tt/rbw-to.TestBottleService.nsproto.sh` to BUD launcher pattern.
- **Validate nsproto with Docker** — Done (22 tests passing via `tt/rbw-to.TestBottleService.nsproto.sh`).
- **Fix and validate iptables entry rule for host access** — Added missing RBM-INGRESS rule in `Tools/rbw/rbss.sentry.sh:85` to allow eth0 ingress traffic on entry port. HTTP connectivity from macOS host now working. Network captures confirmed traffic flow. All srjcl tests passed (3/3).
- **Complete srjcl test validation** — Full srjcl test suite passed (3/3): jupyter_running, jupyter_connectivity, websocket_kernel. HTTP from macOS host to Jupyter via Docker port mapping now working perfectly.
- **Update RBS port forwarding specification** — Replaced DNAT specification with socat proxy + iptables implementation in Phase 2. Added eth0 RBM-INGRESS rule documentation. Updated architecture to document three-container censer model throughout (System Overview, Bottle Pattern, term definitions). Added censer container attributes and definitions. Aligned with recipebottle-admin/index.adoc.
- **Migrate pluml nameplate and tests** — Created rbrn_pluml.env; migrated 5 tabtargets to BUD launcher; added 5 PlantUML tests to rbt_testbench.sh; validated full lifecycle (5/5 tests passed)
- **Absorb Coordinator lifecycle routes** — Removed 14 unused routes from rbk_Coordinator.sh (image mgmt, legacy admin, foundry delete/study, help); 13 routes remain, all with active tabtargets
- **Retire rehosted Makefile rules** — Removed all ported lifecycle rules (rbp_start_service_rule, rbp_connect_*_rule, rbp_observe_networks_rule) from rbp.podman.mk; removed test macros (MBT_PODMAN_*) from rbw.workbench.mk; preserved podman VM machinery intact; validated full lifecycle via bash (5/5 pluml tests passed)
- **Document deferred strands** — Finalized Paddock "Deferred" section with line-numbered references to preserved Makefile machinery; updated "Ported to bash" section documenting all migrations (17 tabtargets, 30 tests, 3 nameplates); created comprehensive `rbw-podman-vm-migration` itch for future heat

## Remaining

(all paces complete)

## Steeplechase

---
### 2025-12-30 08:00 - determine-exec-i-requirements - APPROACH
**Proposed approach**:
- Analyzed `rbt.test.nsproto.mk` for -i flag usage patterns
- Pattern: `-i` needed for stdin-reading commands (dig, traceroute, apt-get); not needed for simple output commands (nslookup, nc -z, ping, ps)
- Define two helper variants in testbench: `rbt_exec_*` and `rbt_exec_*_i` for each container type
---
### 2025-12-30 08:05 - determine-exec-i-requirements - WRAP
**Outcome**: Pattern documented: use `-i` for dig/traceroute/apt-get (stdin readers); no `-i` for nslookup/nc/ping/ps (simple output). Testbench will provide `rbt_exec_*` and `rbt_exec_*_i` variants.
---
### 2025-12-30 08:10 - create-rbt-testbench-skeleton - APPROACH
**Proposed approach**:
- Source but_test.sh, rbrn_regime.sh, rbrr_regime.sh, rbob_bottle.sh
- Use zrbob_kindle to set up container names (ZRBOB_SENTRY/CENSER/BOTTLE) and runtime
- Define exec helpers: `rbt_exec_{sentry,censer,bottle}` and `rbt_exec_{sentry,censer,bottle}_i`
- Route `rbt-to` command to nameplate-specific test functions (placeholder for next pace)
---
### 2025-12-30 08:20 - create-rbt-testbench-skeleton - WRAP
**Outcome**: Created rbt_testbench.sh with exec helpers (6 functions: 3 containers x 2 variants), nameplate loading via zrbob_kindle, routing to suite placeholders. Verified working via launcher.
---
### 2025-12-30 08:30 - single-test-end-to-end - APPROACH
**Proposed approach**:
- Add `test_nsproto_dns_allow_anthropic` function using `rbt_exec_bottle nslookup anthropic.com`
- Wire into `rbt_suite_nsproto` using `but_expect_ok`
- Start nsproto service manually, then run testbench
- Debug any integration issues (container names, exec paths, etc.)
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
### 2025-12-30 14:00 - single-test-end-to-end - RESUME
**Proposed approach**:
- Add `test_nsproto_dns_allow_anthropic()` using `but_expect_ok rbt_exec_bottle nslookup anthropic.com`
- Wire `rbt_suite_nsproto()` to call `but_execute` with prefix `test_nsproto_`
- Start nsproto manually, then run testbench via tabtarget
- Debug integration issues (container names, exec paths, BUT_TEMP_DIR flow)
---
### 2025-12-30 14:05 - single-test-end-to-end - WRAP 🎉
**Outcome**: FIRST LIGHT! End-to-end test infrastructure validated.

**Fixed**: `but_execute` needs empty dir; created `${BUD_TEMP_DIR}/tests` subdirectory.

**Validated stack**: BUD launcher → rbt_testbench.sh → nameplate kindle → RBRR kindle → RBOB kindle → but_execute → docker exec → DNS resolution through sentry dnsmasq.

**Network capture proof**: Ran rbw-o (observe) in background during test. tcpdump traces show:
- Bottle (10.242.0.3) → Sentry (10.242.0.2:53): `A? anthropic.com`
- Sentry → Bottle: `anthropic.com A 160.79.104.10` (allowed CIDR)
- Censer network namespace model working correctly on Docker runtime.

**Unlocked**: Confidence to migrate all 20+ nsproto security tests. Pattern proven for remaining nameplates.
---
### 2025-12-30 14:15 - migrate-nsproto-security-tests - APPROACH
**Proposed approach**:
- Port tests in functional groups: basic network, DNS allow/block, TCP 443, DNS protocol, DNS security, package blocking, ICMP
- Pattern: `! cmd` → `but_expect_fatal`, `cmd` → `but_expect_ok`
- Use `_i` variants for dig/traceroute/apt-get
- Run incrementally after each group
---
### 2025-12-30 14:45 - migrate-nsproto-security-tests - WRAP
**Outcome**: All 22 tests passing on Docker.

**Tests added**: 3 basic network, 2 DNS allow/block, 2 TCP 443, 3 DNS protocol, 9 DNS security bypass, 1 package blocking, 2 ICMP.

**Key fixes**:
- Docker `--internal` network flag blocks ALL forwarding (unlike podman). Removed for Docker, rely on sentry iptables.
- ICMP test adjusted to accept both sentry-visible (podman) and blocked (Docker) behaviors.
- Added single-test parameter to testbench (`rbt-to nsproto test_name`) for targeted debugging.

**Network validation**: Used rboo observe in background during tests - tcpdump confirms traffic flowing through sentry.
---
### 2025-12-30 13:55 - migrate-srjcl-nameplate-and-tests - BLOCKED
**Work completed**:
- Created `rbrn_srjcl.env` with local image refs (arm64 builds required)
- Built `bottle_anthropic_jupyter:local-20251230-134226-85803-332` for arm64
- Migrated all 6 srjcl tabtargets to BUD launcher pattern
- Added 3 test functions to testbench (`test_srjcl_jupyter_running`, `test_srjcl_jupyter_connectivity`, `test_srjcl_websocket_kernel`)
- Fixed typo in `rbt.test.srjcl.py` (RBN→RBRN env var)
- Validated srjcl lifecycle starts correctly (all 3 containers running)

**Blocking issue discovered**: HTTP from macOS host to container via socat times out.
- TCP connects but HTTP response never arrives
- Same issue affects nsproto (not srjcl-specific)
- Works perfectly from inside container network
- Created detailed investigation pace: "Investigate Docker Desktop host-to-container HTTP issue"

**Files preserved for when issue resolved**:
- `Tools/rbw/rbrn_srjcl.env`
- `tt/rbw-*.srjcl.sh` (6 files)
- Test functions in `rbt_testbench.sh`
---
### 2025-12-31 05:53 - fix-validate-iptables-entry-rule - APPROACH
**Proposed approach**:
- Update `Tools/rbw/rbss.sentry.sh` Phase 2 (Port Setup) to add missing RBM-INGRESS rule
- Add: `iptables -A RBM-INGRESS -i eth0 -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} -j ACCEPT`
- Place after existing socat/egress rules, before Phase 3
- Test with srjcl: start service, run rboo in background, curl from macOS host, verify traffic flows
---
### 2025-12-31 06:10 - fix-validate-iptables-entry-rule - WRAP
**Outcome**: HTTP connectivity from macOS host to containers via Docker Desktop now working perfectly. All srjcl tests passed (3/3).

**Root cause**: Sentry iptables RBM-INGRESS chain only had rules for eth1 (enclave network). Missing rule for eth0 (bridge network) caused default DROP policy to block incoming connections from Docker host.

**Fix applied** (Tools/rbw/rbss.sentry.sh:85):
```bash
iptables -A RBM-INGRESS -i eth0 -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} -j ACCEPT
```

**Validation**:
1. HTTP test: `curl http://localhost:7999/api` returned `200 OK` with Jupyter version
2. Network captures: tcpdump confirmed traffic flowing: macOS → Docker bridge → sentry eth0 → socat → sentry eth1 → bottle port 8000
3. Full test suite: All 3 srjcl tests passed (jupyter_running, jupyter_connectivity, websocket_kernel)

**Next pace ready**: "Complete srjcl test validation" can be removed (already done). Move to "Update RBS port forwarding specification" or "Migrate pluml nameplate and tests".
---
### 2025-12-31 06:22 - update-rbs-port-forwarding-specification - APPROACH
**Proposed approach**:
- Read recipebottle-admin/index.adoc to understand how censer model is documented there
- Update RBS Phase 2 (Port Setup) to replace DNAT specification with socat proxy implementation
- Update Filter Configuration section to document the eth0 RBM-INGRESS rule we just added
- Add or enhance architecture section documenting three-container censer model (sentry/censer/bottle with shared namespace)
---
### 2025-12-31 06:35 - update-rbs-port-forwarding-specification - WRAP
**Outcome**: RBS specification now accurately documents the actual implementation and three-container censer model.

**Changes made**:
1. **Phase 2 Port Setup**: Replaced DNAT specification with socat proxy configuration; documented all three iptables rules (eth0 ingress, eth1 egress, eth1 ingress)
2. **System Overview - Bottle Pattern**: Updated from two-container to three-container architecture; added censer namespace establishment and sharing explanation
3. **Architecture Terms**: Added censer container attribute references (`:at_censer_container:`, `:at_censer_container_s:`)
4. **Term Definitions**: Added `[[term_censer_container]]` definition; updated `[[term_bottle_service]]` and `[[term_bottle_container]]` to reference censer
5. **Alignment**: Specification now matches recipebottle-admin/index.adoc and actual rbss.sentry.sh implementation

**Next pace ready**: "Migrate pluml nameplate and tests" - final nameplate migration to complete the nsproto/srjcl/pluml trilogy.
---
### 2025-12-31 06:36 - migrate-pluml-nameplate-and-tests - APPROACH
**Proposed approach**:
- Create rbrn_pluml.env from nameplate.pluml.mk (simplest config: no DNS, no access, no volumes)
- Migrate 4 tabtargets to BUD launcher pattern (Start, ConnectSentry, ObserveNetworks, TestBottleService)
- Add pluml test functions to rbt_testbench.sh based on rbt.test.pluml.mk (6 tests: text rendering, local diagram, HTTP headers, invalid hash, malformed diagram)
- Validate full lifecycle with Docker (expect this to work immediately since HTTP fix already validated)
---
### 2025-12-31 06:56 - migrate-pluml-nameplate-and-tests - WRAP
**Outcome**: Created rbrn_pluml.env; migrated 5 tabtargets to BUD launcher; added 5 PlantUML tests to rbt_testbench.sh; validated full lifecycle (5/5 tests passed)
---
### 2025-12-31 06:57 - absorb-coordinator-lifecycle-routes - APPROACH
**Proposed approach**:
- Verify lifecycle routes (rbw-s, rbw-z, rbw-S, rbw-C, rbw-B, rbw-o) are in workbench (CONFIRMED - already present)
- Verify coordinator has NO lifecycle routes (CONFIRMED - only has payor/governor/foundry/image routes)
- Assessment: This pace appears ALREADY COMPLETE - lifecycle routes were migrated during workbench creation
- Recommend marking this pace complete immediately, or clarifying if additional work is needed
---
### 2025-12-31 07:02 - absorb-coordinator-lifecycle-routes - WRAP
**Outcome**: Removed 14 unused routes from rbk_Coordinator.sh (image mgmt, legacy admin, foundry delete/study, help); 13 routes remain, all with active tabtargets
---
### 2025-12-31 07:03 - retire-rehosted-makefile-rules - APPROACH
**Proposed approach**:
- Review rbp.podman.mk to identify ported lifecycle rules (start/stop/connect/observe for nsproto/srjcl/pluml)
- Review rbw.workbench.mk to identify ported test helper macros (MBT_* macros)
- Remove ported rules while preserving podman VM machinery (rbp_podman_machine_*, stash/pin logic)
- Verify Makefiles still work for remaining (non-ported) functionality
---
### 2025-12-31 07:25 - retire-rehosted-makefile-rules - WRAP
**Outcome**: Successfully retired all ported lifecycle and test rules from Makefiles. Podman VM machinery preserved intact.

**Changes to Tools/rbw/rbp.podman.mk**:
- Removed: `rbp_start_service_rule` (94 lines) - replaced by rbob_start()
- Removed: `rbp_connect_sentry_rule`, `rbp_connect_censer_rule`, `rbp_connect_bottle_rule` - replaced by rbob_connect_*()
- Removed: `rbp_observe_networks_rule` - replaced by rboo_observe()
- Preserved: `rbp_podman_machine_*` rules (start/stop/nuke) for future podman heat
- Preserved: `rbp_stash_*` rules (marked DEFERRED) for future podman heat
- Preserved: `rbp_check_connection` (still used by VM machinery)
- Added: Documentation comment mapping removed rules to bash equivalents

**Changes to Tools/rbw/rbw.workbench.mk**:
- Removed: `MBT_PODMAN_*` test macros (6 macros) - replaced by rbt_exec_* functions
- Removed: `rbw-S.%`, `rbw-C.%`, `rbw-B.%`, `rbw-o.%`, `rbw-s.%` wrapper rules - now route via tabtargets to bash
- Updated: `rbw-to.%` to show migration message and fail (use tabtargets instead)
- Updated: `rbw-tb.%` (batch test) to show not-yet-migrated message (parallel execution deferred)
- Updated: `rbw-tf.%` (fast test) to call bash testbench tabtarget directly
- Updated: `rbw-ta.%` (all tests) to show manual instructions (batch execution deferred)
- Added: Documentation comments throughout explaining migration

**Validation**:
- Verified no active references to removed rules remain (only documentation comments)
- Tested full lifecycle: start → test → stop via bash tabtargets (5/5 pluml tests passed)
- Confirmed podman VM machinery untouched and available for future heat

**Deferred to future work**:
- Parallel batch test execution (rbw-tb.%, rbw-ta.%) - not critical for this heat
- Individual tests work perfectly via tabtargets; batch automation is enhancement
---
### 2025-12-31 07:30 - document-deferred-strands - WRAP
**Outcome**: Finalized deferred strands documentation and created comprehensive itch for podman VM migration heat.

**Heat file updates** (jjh_b251229-dockerize-bashize-proto-bottle.md):
- Updated "Deferred" section with complete line-numbered references to preserved Makefile machinery
- Updated "Ported to bash" section documenting all completed migrations (17 tabtargets, 30 tests, 3 nameplates)
- Added "Notes for future heats" with guidance on parallel testing, podman runtime, and bridge observation challenges

**Itch created** (jji_itch.md):
- Added `rbw-podman-vm-migration` itch with comprehensive migration plan
- Documented scope: 4 VM lifecycle rules, 9 Makefile wrappers, VM image management (stash/pin)
- Specified architecture: `rbpv_PodmanVM.sh` module following BCG pattern
- Outlined key challenges: bridge observation, VM image pinning, connection validation
- Defined testing strategy: vertical slice on nsproto, then full validation across all nameplates
- Set success criteria: all 30 tests passing with `RBRN_RUNTIME=podman`

**Heat completion status**:
- All paces complete ✓
- Docker runtime fully validated (30/30 tests passing) ✓
- Makefile rules retired with clear migration path ✓
- Podman machinery preserved and documented for future heat ✓

---
