## Theurge & Ifrit: Crucible Security Testing

Read this file when working on theurge (test orchestrator), ifrit (attack binary), or crucible test cases.
This is Recipe Bottle's internal test infrastructure only — Recipe Bottle does not require users to have Rust compilation tools.

### Architecture

Two Rust binaries with completely different roles and build targets:

- **Theurge** (`Tools/rbk/rbtd/`) — test orchestrator, runs on the **host** (macOS/Linux). Charges a crucible (sentry + pentacle + bottle containers), invokes attacks, observes results, produces verdicts. Built via `tt/rbw-tb.Build.sh`.
- **Ifrit** (`rbmv_vessels/common-ifrit-context/`) — attack binary, runs **inside the bottle container**. Probes network security boundaries from the attacker's perspective. Source lives in the shared build context consumed by `rbev_bottle_ifrit_tether` and `rbev_bottle_ifrit_airgap`. Built inside the Docker image during `docker build` — there is no host-side compilation, no cross-compile, no `cargo check` on macOS. The Dockerfile IS the build system.

**Coordinated tests** are the distinctive capability: theurge simultaneously observes from outside (via sentry writ/fiat commands) while ifrit attacks from inside.
Neither binary alone can do this.

### Suites

A suite runs with `tt/rbw-ts.TestSuite.<suite>.sh`.
Which fixtures each suite holds is the `RBTCEA_SUITES` registry in `Tools/rbk/rbtd/src/rbtdra_almanac.rs`, the one authority, so the tables below say what each suite reaches and what it needs, never its member list.

The dependency-tiered suites:

| Suite | Tabtarget | Needs | Covers |
|-------|-----------|-------|--------|
| `reveille` | `tt/rbw-ts.TestSuite.reveille.sh` | Nothing outside the tree | The credless base: regime and enrollment validation, handbook render, hygiene and conformance checks, the fact-chain band |
| `picket` | `tt/rbw-ts.TestSuite.picket.sh` | GCP credentials: a live sederunt | reveille, regime-poison, and the cloud fixtures: registry access, hallmark, Lode, reliquary, WSL and podman-machine lifecycles, batch vouch, admission denial, parley and the chained build facts |
| `bivouac` | `tt/rbw-ts.TestSuite.bivouac.sh` | A container runtime | reveille, regime-poison, and the local crucibles: a fresh tadmor kludge, then tadmor, srjcl and pluml |
| `echelon` | `tt/rbw-ts.TestSuite.echelon.sh` | Both | picket and bivouac together |

`regime-poison` is the negative half of regime validation: real validate verbs against real regimes, one field corrupted through the regime-poison tweak, each case asserting the refusal code it expects.
It rides above reveille, which keeps the tweak slot for the credless guard, so a regime or validation change runs reveille for the positives and `tt/rbw-tf.FixtureRun.sh regime-poison` for the negatives.
Its station-local cases skip themselves where the regime is not configured on the machine.

The release and probe suites, told apart by project churn, crucible and network posture rather than by dependency tier:

| Suite | Tabtarget | Needs | Covers |
|-------|-----------|-------|--------|
| `gauntlet` | `tt/rbw-ts.TestSuite.gauntlet.sh` | A payor install; it levies its own projects | Release qualification from an unconfigured regime: depot lifecycle, freehold establish, credential readiness, the onboarding sequence, the reveille fixtures, hallmark lifecycle, then the tadmor, moriah, srjcl and pluml crucibles |
| `skirmish` | `tt/rbw-ts.TestSuite.skirmish.sh` | A levied freehold depot and a live sederunt | The gauntlet's build-and-crucible chain on a standing depot, with no project levied or unmade |
| `dogfight` | `tt/rbw-ts.TestSuite.dogfight.sh` | A levied freehold depot and a live sederunt | Cloud build viability: ordain, summon and run, with no crucible |
| `siege` | `tt/rbw-ts.TestSuite.siege.sh` | Nothing beyond a container runtime | Fully local: kludge both tadmor vessels, then the tadmor security cases |
| `blockade` | `tt/rbw-ts.TestSuite.blockade.sh` | A levied depot, a live sederunt and the moriah hallmark ordained | The moriah airgap crucible, its conjure hallmarks summoned from the depot |
| `parley` | `tt/rbw-ts.TestSuite.parley.sh` | A levied freehold depot whose subject holds the retriever mantle by `brevet`, and a live sederunt | A positive admission round trip: `unseat` the retriever mantle and `brevet` it back, asserting through rehearse's manor roll that the muniment stands, vanishes and stands again |
| `calibrant` | `tt/rbw-ts.TestSuite.calibrant.sh` | Nothing | Fails by design: the touchstone fixture runs it to prove a suite stops at its first failing fixture. Never run it as a check |

The credentialed suites lead with the credential-readiness fixture, which fails in seconds, naming the remedy, when the sederunt or a mantle is not ready.

After a code change, run the tier it touches:

- Regime or validation changes: `reveille` plus `tt/rbw-tf.FixtureRun.sh regime-poison`
- Foundry or credential changes: `picket`
- Bottle, sentry or network changes: `bivouac`
- Before a release, or after a change across many modules: `echelon`

### Crucible Iteration Loop

The typical development cycle when changing ifrit or theurge code:

#### Iteration strategy: single cases first, full suite last

The full tadmor fixture takes ~10 minutes (charge + 50+ cases + quench).
**Do not run it as the first verification step** — whether debugging failures or verifying new code.
Instead:

1. Charge the crucible once
2. Run single-case against each new or changed test to verify it passes
3. Run the full fixture only after all targeted cases pass — as a final regression check
4. Quench

This applies to **all** crucible verification: new tests, bug fixes, refactors.
The full suite is always the last step, never the first.

#### Full run (charge + all cases + quench in one command)

1. Edit ifrit source (`rbmm_moorings/rbmv_vessels/common-ifrit-context/src/`) or theurge source (`Tools/rbk/rbtd/src/`)
2. Run the full tadmor fixture:
   ```
   tt/rbw-tf.FixtureRun.sh tadmor
   ```
   This charges the crucible, runs all cases, and quenches — one command.
3. If charge refuses with a stale kludge pin, rebuild the vessel it names and commit the pin, then re-run step 2:
   ```
   tt/rbw-cKB.KludgeBottle.sh tadmor
   tt/rbw-cKS.KludgeSentry.sh tadmor
   ```
   Charge answers the did-anything-change question for you: it compares each pinned kludge hallmark against the vessel source and refuses when they have parted, naming the vessel and printing the producer tabtarget to run.
   Run the one it names — the refusal is per vessel — then commit the hallmark it `mitres` into the nameplate's `rbrn.env`.
   The commit is required rather than tidy: kludge dirties `rbrn.env`, and charge demands a clean working tree.

#### Single-case debugging (manual charge/quench lifecycle)

When iterating on a specific failing test case:

1. Charge the crucible (starts containers, leaves them running):
   ```
   tt/rbw-cC.Charge.tadmor.sh
   ```
2. Run individual cases against the live crucible:
   ```
   tt/rbw-tc.FixtureCase.sh tadmor case-name
   ```
   Omit the case name to list all available cases.
   Omit the fixture name to list all available fixtures.
3. Edit code, rebuild as needed (kludge for ifrit, `tt/rbw-tb.Build.sh` for theurge), re-run the single case. Repeat.
4. Quench when done:
   ```
   tt/rbw-cQ.Quench.tadmor.sh
   ```

#### Ordaining after green

Kludge builds are for rapid local iteration.
Once all tests pass with the kludge hallmark, ordain for a production-grade Cloud Build image:

1. Ordain the vessel:
   ```
   tt/rbw-fO.DirectorOrdainsHallmark.sh rbev_bottle_ifrit_tether
   ```
2. Summon the ordained hallmark locally:
   ```
   tt/rbw-fs.RetrieverSummonsHallmark.sh rbev_bottle_ifrit_tether <hallmark>
   ```
3. Write the ordained hallmark into the nameplate with the `mitre` door, `tt/rbw-nm.MitreNameplateHallmark.sh tadmor bottle <hallmark>` (or omit `<hallmark>` to chain-read it from the ordain you just ran), commit, and re-run the full fixture to verify.

**Hallmark prefixes** tell you what you have: `k` = kludge (local build), `c` = conjured (Cloud Build ordained).

### Test Cases

Bottle/sentry security cases — shared verbatim by the `tadmor` and `moriah` fixtures (`RBTCCC_CASES_SECURITY` in `rbtdrc_crucible.rs`).
The two fixtures differ only in provenance: tadmor runs against a locally-kludged bottle, moriah against the cloud-built airgap bottle.
Runtime semantics are identical.

Cases group by purpose in source order (no formal section structure):

- **basic-infra** — smoke tests: pentacle ping, dnsmasq responds
- **ifrit-attacks** — single ifrit attack, verdict from inside only: dns-allowed/blocked, apt blocked
- **observation** — sentry-side observation of bottle behavior: iptables loaded, blocked-with-observation
- **correlated** — theurge resolves on sentry, ifrit attacks with result: tcp443 allow/block, ICMP hop tests
- **sortie-attacks** — multi-step ifrit sorties: DNS exfil, metadata probe, raw socket smuggle
- **unilateral-novel** — ifrit sorties testing novel attack vectors: route manipulation, subnet escape, DNAT reflection
- **coordinated-attacks** — simultaneous attack + observation: ARP gratuitous/poison, table stability
- **coordinated-integrity** — attack then verify sentry state unchanged: sentry integrity, DNS cache integrity, MAC flood

### Reveille-Tier Cases: Credless by Construction

The reveille suite's fixtures carry `credless: true` on their `rbtcee_Fixture` static.
While such a fixture runs, every tabtarget Command theurge builds (via `rbtcei_tabtarget_command` — including the direct-Command helpers) carries the credless-guard tweak (`RBCC_tweak_credless_guard` / `RBTGEC_TWEAK_CREDLESS_GUARD`), and both token mints refuse under it before touching any credential, leaving the shell with the credless band code (`RBCG_CREDLESS_DENTIL` / `RBTGEB_CREDLESS_DENTIL`): `rbau_avow` rejects itself, and the payor's `rbgp_authenticate_capture` emits its credless refusal word, which `rbgp_authenticate_die` at every caller rejects in that band.
The guard rides the fixture, not the suite — a reveille fixture hosted in picket/echelon is still guarded.

Rules for authoring a reveille-tier case:

- **A reveille case may invoke any tabtarget freely** — if its chain reaches a token mint, the run dies with the credless band code instead of spending money or mutating the depot. The proof case is `rbtchf_rs_credless_guard_mint_refusal` (regime-smoke).
- **Fast cases carry no tweaks of their own.** The BURE tweak slot belongs to the guard in the reveille tier (one tweak at a time, and the suite reserves the slot); `rbtcei_invoke*` fails loud if a guarded case supplies `BURE_TWEAK_NAME`. A case that needs a test seam has self-identified as not belonging in reveille — home it in a higher tier.
- **A new reveille fixture must set `credless: true`** and join the reveille base `RBTCEA_REVEILLE_BASE` and every suite that carries that base in `RBTCEA_SUITES`, both in `rbtdra_almanac.rs`, where a compile-time guard holds each reveille-bearing suite to the base; fixtures in higher tiers set `credless: false`.

### Fixture Config-Evolution Console

A fixture that walks an onboarding or freehold track *evolves* the tracked config it tests — `mitring` hallmarks into nameplates, electing vessel anchors, setting regime fields — and commits each step so the next sees a clean tree.
Those write-side actions are a named cohort homed in `rbtdre_engine.rs` (the "Fixture config-evolution console"), beside the read-side `rbtcee_tree_clean`.
**When a fixture commits config, it goes through the console — never an ad-hoc `git add`/`git commit`.**
`rbtcee_tree_clean` itself refuses every drive on a dirty tree, a single fixture and a suite alike, and each refusal names the commit as the remedy.

The console commits config in exactly three classes, each through its own scoped verb that derives its own paths from a class identifier and stages only those:

| Class | Identifier | Verb | Stages |
|-------|-----------|------|--------|
| nameplate | nameplate moniker(s) | `rbtcee_commit_nameplates` | `<moorings>/<np>/rbrn.env` |
| vessel | vessel dir(s) | `rbtcee_commit_vessels` / `rbtcee_commit_vessels_all` | `<vessel_dir>/rbrv.env` |
| regime | `rbtcee_RegimeFile` tag(s) | `rbtcee_commit_regime` | `<moorings>/rbrd.env` / `<moorings>/rbrr.env` |

The load-bearing property is **no free-form file list**.
The only entry that takes an arbitrary path set, `rbtcee_commit_paths`, is private to the engine module; every public verb is class-typed.
So a fixture is *structurally incapable* of sweeping a surprise edit, another session's work or a half-finished change, into its commit.
A scoped `git status --porcelain -- <derived-paths>` runs first; if nothing in the class changed, the commit is a clean no-op, not an error.

**Config-zero** (`rbtcee_config_zero`) is the console's reset seam, for tracked config written in place.
Zeroing a field before an in-place write defeats the stale-value false-green — a silently-skipped write then leaves an obviously-empty value, not a passing stale one — and its fail-on-absent doubles as a schema-drift catch: a renamed field stops the run instead of writing nothing.
It shares one validated core (`rbtcee_config_set_field`) with the vessel-env write; it is **one seam, never a per-field function farm**.
Temp-vessel cases that construct known bytes need no zero call.

**A second home outside the console.**
The same field rewrite also stands in `rbtcgf_replace_env_fields`, the regime multi-field rewrite in `rbtdrk_freehold.rs`; fold it onto `rbtcee_config_set_field` when next in that code.
A nameplate's hallmark is written by the real `rbw-nm` `mitre` tabtarget, whose one home is `rbrn_mitre`.
New fixture state-mutation helpers land in the console; they do not re-grow per module.

### Adding a New Test

**New ifrit attack** (simple probe, single command):
1. Add constant, enum variant, `from_selector`/`selector`/`all_selectors` entries in `rbida_attacks.rs`
2. Add dispatch arm in `rbtica_run()`
3. Add theurge case function calling `rbtccc_invoke_ifrit(ctx, "selector-name", dir)`
4. Register in `RBTCCC_CASES_SECURITY` (or the relevant fixture's case array)

**New ifrit sortie** (complex multi-step attack):
1. Add `pub fn sortie_name()` in `rbida_sorties.rs`
2. Add constant, enum variant, and dispatch in `rbida_attacks.rs` (same as above)
3. Add theurge case — sorties may need coordinated observation (writ/fiat before/after)
4. Register in `RBTCCC_CASES_SECURITY` (or the relevant fixture's case array)

**Crucible verification workflow** (applies to all new tests and changes):

1. Build theurge: `tt/rbw-tb.Build.sh` and run unit tests: `tt/rbw-tt.Test.sh`
2. If ifrit source changed: kludge-rebuild, commit hallmark, then charge:
   ```
   tt/rbw-cKB.KludgeBottle.sh tadmor    # builds image, rbrn_mitre writes hallmark into rbrn.env
   # commit the rbrn.env change (clean tree required for charge)
   tt/rbw-cC.Charge.tadmor.sh
   ```
3. Verify each new/changed case individually against the live crucible:
   ```
   tt/rbw-tc.FixtureCase.sh tadmor rbtccc_sortie_new_case_name
   ```
4. Only after all targeted cases pass, run the full fixture for regression:
   ```
   tt/rbw-tf.FixtureRun.sh tadmor
   ```
   Note: this charges and quenches internally.
   A crucible you charged manually from this same tree is recycled automatically; one charged from a *different* working tree refuses the charge and names its owner — quench it from its own tree, or eyes-open from here (`BURE_CONFIRM=skip tt/rbw-cQ.Quench.tadmor.sh`, which prints the owner as it proceeds).
