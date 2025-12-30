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
- `rbp_podman_machine_start_rule` - VM startup
- `rbp_podman_machine_stop_rule` - VM shutdown
- `rbp_podman_machine_nuke_rule` - VM removal
- `rbp_stash_*` rules - VM image pinning/caching (marked buggy/deferred in source)
- `RBM_MACHINE`, `RBM_CONNECTION` variables - podman machine connection
- `zRBM_PODMAN_SSH_CMD` - SSH into podman VM
- `rbo.observe.sh` - uses `podman machine ssh` to capture bridge traffic; no Docker equivalent

**Ported (update as paces complete):**
- (none yet)

**Notes for future heats:**
- Parallel test execution: Makefile runs tests per-nameplate sequentially but could parallelize across nameplates (nsproto, srjcl, pluml are independent). Current `-j` flag applied within test suite. Not needed for this heat but preserve capability awareness.

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

## Remaining

- **Investigate Docker Desktop host-to-container HTTP issue** — HTTP requests from macOS host through sentry's socat proxy timeout, blocking srjcl and pluml test validation. This pace must resolve before continuing srjcl/pluml migration.
  mode: manual

  **Symptom**: `curl http://localhost:7999/lab` from macOS host hangs indefinitely (sends request, never receives response). Returns HTTP code "000".

  **What works**:
  - TCP connection from host succeeds: `nc -zv localhost 7999` → "succeeded!"
  - Docker port mapping exists: `docker port srjcl-sentry` → "7999/tcp -> 0.0.0.0:7999"
  - Sentry is dual-homed (bridge + enclave networks)
  - Socat running in sentry: `socat TCP-LISTEN:7999,fork,reuseaddr TCP:10.242.2.3:8000`
  - Bottle's Jupyter listening: `netstat -tuln` shows 0.0.0.0:8000
  - Sentry-to-bottle HTTP works: `docker exec srjcl-sentry nc -zv 10.242.2.3 8000` → "succeeded!"
  - HTTP from inside sentry works: `docker exec srjcl-sentry curl localhost:7999/api` → HTTP 200
  - Same issue affects nsproto (port 8890) - not srjcl-specific

  **Architecture flow**:
  ```
  Host:7999 → Docker port mapping → Sentry:7999 (socat) → Bottle:8000 (Jupyter)
  ```

  **Hypothesis**: Docker Desktop for Mac networking quirk. TCP handshake completes but HTTP response data doesn't flow back through the socat proxy when accessed from the host. May be related to Docker Desktop's VM-based networking layer.

  **Investigation paths**:
  1. Check Docker Desktop networking settings (VirtioFS, gRPC FUSE, etc.)
  2. Try binding socat to specific interface vs 0.0.0.0
  3. Test with simpler proxy (netcat instead of socat)
  4. Check if `--network host` on test container works differently
  5. Compare behavior on Linux Docker (if available)
  6. Check if issue is IPv4 vs IPv6 related (curl tries ::1 first)

  **Workaround options** (if root cause not fixable):
  - Run HTTP tests from inside container network (via docker exec)
  - Run Python test container with `--network container:srjcl-sentry` to share sentry's namespace
  - Accept limitation and document as Docker Desktop caveat

  **Files created during srjcl partial migration** (preserved, awaiting this fix):
  - `Tools/rbw/rbrn_srjcl.env` - nameplate config (uses local images)
  - `tt/rbw-*.srjcl.sh` - all 6 tabtargets migrated to BUD launcher
  - `rbt_testbench.sh` - srjcl test functions added (3 tests)
  - `RBM-tests/rbt.test.srjcl.py` - fixed env var typo (RBN→RBRN)
  - `bottle_anthropic_jupyter:local-*` - locally built image for arm64

- **Complete srjcl test validation** — After HTTP issue resolved, run full srjcl test suite. May need to adjust test approach based on resolution.
  mode: manual

- **Migrate pluml nameplate and tests** — Create `rbrn_pluml.env`, migrate `rbt.test.pluml.mk` tests to bash, migrate pluml tabtargets (`rbw-s.Start.pluml.sh`, `rbw-S.ConnectSentry.pluml.sh`, `rbw-o.ObserveNetworks.pluml.sh`, `rbw-to.TestBottleService.pluml.sh`). Validate with Docker.
  mode: manual

- **Absorb Coordinator lifecycle routes** — Move only lifecycle-related routes from `rbk_Coordinator.sh` to `rbw_workbench.sh`. Non-lifecycle commands (payor, governor, foundry, image management) remain in Coordinator. Consolidation of all routes deferred to future heat.
  mode: manual

- **Retire rehosted Makefile rules** — Remove successfully ported rules from `rbp.podman.mk`. Keep podman VM machinery intact. Update `rbw.workbench.mk` to remove migrated test helper macros.
  mode: manual

- **Document deferred strands** — Finalize Paddock "Deferred" section. Create itch for podman VM migration heat with specific items to address.
  mode: manual

## Steeplechase

(execution log begins here)
