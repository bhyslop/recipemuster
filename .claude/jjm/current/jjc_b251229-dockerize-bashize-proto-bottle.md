# Steeplechase: Dockerize Bashize Proto Bottle

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
(execution log begins here)
