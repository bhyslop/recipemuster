## Theurge & Ifrit: Crucible Security Testing

Read this file when working on theurge (test orchestrator), ifrit (attack binary), or crucible test cases. This is Recipe Bottle's internal test infrastructure only — Recipe Bottle does not require users to have Rust compilation tools.

### Architecture

Two Rust binaries with completely different roles and build targets:

- **Theurge** (`Tools/rbk/rbtd/`) — test orchestrator, runs on the **host** (macOS/Linux). Charges a crucible (sentry + pentacle + bottle containers), invokes attacks, observes results, produces verdicts. Built via `tt/rbtd-b.Build.sh`.
- **Ifrit** (`rbev-vessels/common-ifrit-context/`) — attack binary, runs **inside the bottle container**. Probes network security boundaries from the attacker's perspective. Source lives in the shared build context consumed by `rbev-bottle-ifrit-tether` (and the forthcoming airgap variant). Built inside the Docker image during `docker build` — there is no host-side compilation, no cross-compile, no `cargo check` on macOS. The Dockerfile IS the build system.

**Coordinated tests** are the distinctive capability: theurge simultaneously observes from outside (via sentry writ/fiat commands) while ifrit attacks from inside. Neither binary alone can do this.

### Crucible Iteration Loop

The typical development cycle when changing ifrit or theurge code:

#### Iteration strategy: single cases first, full suite last

The full tadmor fixture takes ~10 minutes (charge + 50+ cases + quench). **Do not run it as the first verification step** — whether debugging failures or verifying new code. Instead:

1. Charge the crucible once
2. Run single-case against each new or changed test to verify it passes
3. Run the full fixture only after all targeted cases pass — as a final regression check
4. Quench

This applies to **all** crucible verification: new tests, bug fixes, refactors. The full suite is always the last step, never the first.

#### Full run (charge + all cases + quench in one command)

1. Edit ifrit source (`rbev-vessels/common-ifrit-context/src/`) or theurge source (`Tools/rbk/rbtd/src/`)
2. If ifrit changed: kludge-rebuild the bottle image
   ```
   tt/rbw-cKB.KludgeBottle.sh tadmor
   ```
   This builds a new container image and drives the kludge hallmark into the nameplate's `rbrn.env`.
3. Git commit the hallmark change (kludge dirties `rbrn.env` — a clean working tree is required for charge, and the commit trail maintains audit integrity)
4. Run the full tadmor fixture:
   ```
   tt/rbtd-r.Run.tadmor.sh
   ```
   This charges the crucible, runs all cases, and quenches — one command.

#### Single-case debugging (manual charge/quench lifecycle)

When iterating on a specific failing test case:

1. Charge the crucible (starts containers, leaves them running):
   ```
   tt/rbw-cC.Charge.tadmor.sh
   ```
2. Run individual cases against the live crucible:
   ```
   tt/rbtd-s.SingleCase.tadmor.sh case-name
   ```
   Omit the case name to list all available cases.
3. Edit code, rebuild as needed (kludge for ifrit, `tt/rbtd-b.Build.sh` for theurge), re-run the single case. Repeat.
4. Quench when done:
   ```
   tt/rbw-cQ.Quench.tadmor.sh
   ```

#### Ordaining after green

Kludge builds are for rapid local iteration. Once all tests pass with the kludge hallmark, ordain for a production-grade Cloud Build image:

1. Ordain the vessel:
   ```
   tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-bottle-ifrit-tether
   ```
2. Summon the ordained hallmark locally:
   ```
   tt/rbw-fs.RetrieverSummonsHallmark.sh rbev-bottle-ifrit-tether <hallmark>
   ```
3. Drive the ordained hallmark into the nameplate (edit `RBRN_BOTTLE_HALLMARK` in `.rbk/tadmor/rbrn.env`), commit, and re-run the full fixture to verify.

**Hallmark prefixes** tell you what you have: `k` = kludge (local build), `c` = conjured (Cloud Build ordained).

### Test Sections

Tadmor crucible cases are organized by test pattern:

| Section | Pattern | Example |
|---------|---------|---------|
| `tadmor-basic-infra` | Smoke tests — containers up, DNS responding | pentacle ping, dnsmasq responds |
| `tadmor-ifrit-attacks` | Single ifrit attack, verdict from inside only | dns-allowed, dns-blocked, apt blocked |
| `tadmor-observation` | Sentry-side observation of bottle behavior | iptables loaded, blocked-with-observation |
| `tadmor-correlated` | Theurge resolves on sentry, ifrit attacks with result | tcp443 allow/block, ICMP hop tests |
| `tadmor-sortie-attacks` | Multi-step ifrit sorties (complex attack sequences) | DNS exfil, metadata probe, raw socket smuggle |
| `tadmor-unilateral-novel` | Ifrit sorties testing novel attack vectors | route manipulation, subnet escape, DNAT reflection |
| `tadmor-coordinated-attacks` | Simultaneous attack + observation | ARP gratuitous/poison, table stability |
| `tadmor-coordinated-integrity` | Attack then verify sentry state unchanged | sentry integrity, DNS cache integrity, MAC flood |

### Adding a New Test

**New ifrit attack** (simple probe, single command):
1. Add constant, enum variant, `from_selector`/`selector`/`all_selectors` entries in `rbida_attacks.rs`
2. Add dispatch arm in `rbida_run()`
3. Add theurge case function calling `rbtdrc_invoke_ifrit(ctx, "selector-name", dir)`
4. Register in appropriate section

**New ifrit sortie** (complex multi-step attack):
1. Add `pub fn sortie_name()` in `rbida_sorties.rs`
2. Add constant, enum variant, and dispatch in `rbida_attacks.rs` (same as above)
3. Add theurge case — sorties may need coordinated observation (writ/fiat before/after)
4. Register in appropriate section

**Crucible verification workflow** (applies to all new tests and changes):

1. Build theurge: `tt/rbtd-b.Build.sh` and run unit tests: `tt/rbtd-t.Test.sh`
2. If ifrit source changed: kludge-rebuild, commit hallmark, then charge:
   ```
   tt/rbw-cKB.KludgeBottle.sh tadmor    # builds image, drives hallmark into rbrn.env
   # commit the rbrn.env change (clean tree required for charge)
   tt/rbw-cC.Charge.tadmor.sh
   ```
3. Verify each new/changed case individually against the live crucible:
   ```
   tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_sortie_new_case_name
   ```
4. Only after all targeted cases pass, run the full fixture for regression:
   ```
   tt/rbtd-r.Run.tadmor.sh
   ```
   Note: this charges and quenches internally — quench the manual crucible first if one is active (`tt/rbw-cQ.Quench.tadmor.sh`).
