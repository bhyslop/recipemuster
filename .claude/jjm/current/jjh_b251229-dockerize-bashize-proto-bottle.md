# Heat: Dockerize Bashize Proto Bottle

## Paddock

### Goal

Migrate the bottle/sentry/censer container lifecycle from Makefile (`rbp.podman.mk`) to bash (`rbw_workbench.sh`), validated with Docker as the initial runtime. Preserve the hard-won censer network namespace model. Migrate all three nameplate test suites (nsproto, srjcl, pluml) to bash (`rbt_testbench.sh`), with nsproto first as the prototype.

### Approach

**Runtime abstraction first**: Add `RBRN_RUNTIME=docker|podman` to configuration. Bash code uses this to select runtime command. Docker-first validation avoids podman VM complexity; if Docker fails, we're positioned for podman fallback.

**Vertical slice on nsproto**: Complete end-to-end migration for nsproto nameplate before generalizing. Nsproto is simplest vessel and exercises security architecture most thoroughly (20+ tests).

**Preserve, don't delete**: Makefile code that isn't rehosted stays in place. Only remove rules that are successfully migrated and validated. Podman VM machinery (`rbp_podman_machine_*`, stash/pin logic) remains untouched for future heat.

**Test migration is critical**: The security tests validate the censer model works. Migrating them to bash proves Docker runtime is viable.

### Key Architectural Decisions

1. **Configuration format**: Nameplates move from `nameplate.*.mk` (Makefile syntax) to `rbrn_<moniker>.env` (bash-sourceable)

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

### File Inventory

| File | Status | Notes |
|------|--------|-------|
| **Workbench & Config (new)** |||
| `Tools/rbw/rbw_workbench.sh` | create | Container lifecycle workbench |
| `Tools/rbw/rbt_testbench.sh` | create | Test runner using but_test.sh |
| `Tools/rbw/rbrn_nsproto.env` | create | Bash-sourceable nameplate config |
| `Tools/rbw/rbrn_srjcl.env` | create | Bash-sourceable nameplate config |
| `Tools/rbw/rbrn_pluml.env` | create | Bash-sourceable nameplate config |
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

(none yet)

## Remaining

- **Modernize rbrn_regime.sh** — Convert from old self-sourcing format to kindle/sentinel pattern. Remove direct `source buv_validation.sh`, add multiple-inclusion guard, create `zrbrn_kindle()` and `zrbrn_sentinel()` functions. Follow `rbrr_regime.sh` as template.
  mode: manual

- **Create rbrn_nsproto.env** — Convert `nameplate.nsproto.mk` to bash-sourceable format. Add `RBRN_RUNTIME=docker` parameter. Validate with modernized `rbrn_regime.sh`.
  mode: manual

- **Create rbw_workbench.sh skeleton** — Establish workbench structure with runtime abstraction. Include command routing that parses moniker from tabtarget filename tokens. Source `buc_command.sh` for output. No lifecycle implementation yet.
  mode: manual

- **Create BUD launchers** — Create `.buk/launcher.rbw_workbench.sh` and `.buk/launcher.rbt_testbench.sh` following existing launcher pattern.
  mode: manual

- **Implement local recipe build** — Add `rbw-lB` route for local Docker builds. Build from `RBM-recipes/${TOKEN_3}.recipe` files (these are Dockerfiles). Tag as `${recipe_name}:local-${timestamp}`. Create per-recipe tabtargets (e.g., `tt/rbw-lB.LocalBuild.sentry_ubuntu_large.sh`). Long-term capability replacing blocked GCB.
  mode: manual

- **Design RBRR env injection for sentry** — The sentry setup script (`rbss.sentry.sh`) requires `RBRR_DNS_SERVER` from RBRR regime. Decide how launcher/workbench loads and passes RBRR variables to container. Options: launcher loads both regimes, workbench loads RBRR on demand, or nameplate includes DNS server. Reference `rbw.workbench.mk` for current Makefile approach.
  mode: manual

- **Implement rbw-start** — Port `rbp_start_service_rule` to bash. Orchestrate: cleanup prior containers, create enclave network, launch sentry (bridge + enclave), configure sentry security, launch censer, configure censer routing, create and start bottle.
  mode: manual

- **Implement rbw-stop** — Create service shutdown. Stop and remove bottle, censer, sentry containers. Remove enclave network. Create new tabtarget `tt/rbw-z.Stop.nsproto.sh`.
  mode: manual

- **Implement rbw-connect commands** — Port `rbp_connect_sentry/censer/bottle_rule` to bash functions. Interactive exec into each container type.
  mode: manual

- **Implement rbw-observe (partial)** — Port what's possible without podman machine ssh. The full `rbo.observe.sh` requires `podman machine ssh` for bridge capture which has no Docker equivalent. Implement sentry/censer tcpdump capture; defer bridge capture to podman heat.
  mode: manual

- **Migrate lifecycle tabtargets** — Modify existing tabtargets (`rbw-s.Start`, `rbw-S.ConnectSentry`, `rbw-C.ConnectCenser`, `rbw-B.ConnectBottle`, `rbw-o.ObserveNetworks`) from mbd.dispatch to BUD launcher pattern.
  mode: manual

- **Determine exec -i requirements** — Review existing tests to understand when `-i` (stdin) flag is needed for container exec. Current Makefile uses both `MBT_PODMAN_EXEC_BOTTLE` and `MBT_PODMAN_EXEC_BOTTLE_I`. Document pattern for testbench helper functions.
  mode: manual

- **Create rbt_testbench.sh skeleton** — Establish test workbench using `but_test.sh`. Include test helper functions (`rbt_exec_sentry`, `rbt_exec_bottle`, etc.) with runtime abstraction. Apply -i requirements from previous pace.
  mode: manual

- **Migrate nsproto security tests** — Port `rbt.test.nsproto.mk` test cases to bash functions. 20+ tests covering DNS filtering, TCP blocking, ICMP isolation, package blocking.
  mode: manual

- **Migrate test tabtarget** — Modify `tt/rbw-to.TestBottleService.nsproto.sh` from mbd.dispatch to rbt_testbench launcher.
  mode: manual

- **Validate nsproto with Docker** — Full end-to-end validation: start nsproto service with Docker runtime, run security test suite, verify all tests pass.
  mode: manual

- **Migrate srjcl nameplate and tests** — Create `rbrn_srjcl.env`, migrate `rbt.test.srjcl.mk` tests to bash (invoke existing `rbt.test.srjcl.py` from bash wrapper), migrate srjcl tabtargets (`rbw-s.Start.srjcl.sh`, `rbw-S.ConnectSentry.srjcl.sh`, `rbw-B.ConnectBottle.srjcl.sh`, `rbw-o.ObserveNetworks.srjcl.sh`, `rbw-to.TestBottleService.srjcl.sh`). Validate with Docker.
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
