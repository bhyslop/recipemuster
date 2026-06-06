# Heat Trophy: rbk-10-mvp-uncontrolled-cygwin-test

**Firemark:** ₣BV
**Created:** 260601
**Retired:** 260605
**Status:** retired

## Paddock

## Shape

Prove that Recipe Bottle's test harness (theurge) can run on an **uncontrolled**
Cygwin — a stock Windows+Cygwin box we do not provision. This is a consumer-
credibility claim: a consumer can qualify their own install on their own machine
and trust the verdict. It is deliberately distinct from the `rbk-NN-win-*` heats,
which are all controlled-infrastructure work (formal BURN garrison, remote
control, depot regime) on machines we own.

## Progress & scope (updated 2026-06-03)

The heat outgrew its original fast/validation-tier scope: the **crucible tier now
runs on uncontrolled Cygwin Docker Desktop**. siege (the renamed tadmor suite)
charges and runs the adversarial sortie set 45/46; blockade (moriah, airgap)
follows. The two membranes that made it possible — the compose-CLI path
normalizer and the nested bind-mountpoint pre-creation — are documented in
`Memos/memo-20260603-windows-docker-desktop-bind-mount.md`. This supersedes the
"container tiers run on WSL only / fast-tier only" locked decision below. One
open finding: the `conntrack_spoofed_ack` sortie BREACHED on siege; adjudication
pending (`Memos/memo-20260603-cygwin-ifrit-first-run-conntrack.md`).

## Remote build discipline (git is the filesystem)

All work runs on a remote fundus (`cygwin@rocket`); code reaches it through git,
never scp. The cycle, learned the hard way this heat:

1. Edit on the curia (this station).
2. Notch (`jjx_record`) — a real commit; the affiliation is the transport label.
3. `git pull --rebase` then `git push` on the curia.
4. On the fundus: `git pull` to fast-forward — or, when the fundus tree is dirty
   or detached (a prior `jjx_plant` or scratch state), `git fetch && git checkout
   -f main && git reset --hard origin/main`. Destructive git is sanctioned here
   only because it replaces with byte-identical committed content.
5. Run the tabtarget on the fundus over ssh. No trailing `echo` — it masks the
   exit code; let ssh propagate the tabtarget's status.

Two traps that cost time this heat:

- **scp is wrong.** It dirties the fundus tree and diverges from git. Git is the
  only sanctioned transport for code.
- **The fundus needs a git identity.** Without `git config user.name/email` on
  the fundus, any fixture that owns its own notch (kludge-tadmor and friends)
  dies with `Author identity unknown` (exit 128). Set it once per host.

Exceptions and adjuncts:

- **Kludge-mode crucibles cannot ride git-as-filesystem.** Kludge images are
  host-local — the image must be built on the fundus. So kludge on the fundus and
  let the fixture commit the stamp there; accept the temporary cygwin-resident
  stamp pollution. (Conjure-mode / blockade summons from GAR, so it rides git
  normally.)
- **Shared-depot credentials go stale.** Another host's governor mantle obsoletes
  this fundus's Director/Retriever SAs — they fail with `Invalid JWT Signature`
  (persisting past the retry budget, not a real propagation race). Re-establish
  before any GAR-touching run: reinstall the Payor OAuth, then run `dogfight`
  (its `canonical-invest` re-mantles the Governor and re-invests
  Retriever+Director).

## Central thesis (the real problem)

The build system is not the hard part — **the bash-launch boundary is.** RBTDRX
(`rbtdrx_platform.rs`) already solves the *path-string* boundary at theurge's
Rust↔bash crossing, and it is sound: it keys on `OSTYPE=cygwin`, which Cygwin
bash exports to spawned children (verified on `cygwin@rocket`). But RBTDRX does
**not** solve the *process-launch* boundary, and that is where the wall is:

- A Windows-native binary (which `x86_64-pc-windows-gnu` produces) **cannot**
  `CreateProcess` a `.sh` directly — Rust's `std::process::Command` on Windows
  supports only `.exe` (undocumented `.bat`/`.cmd`); a `.sh` yields "not a valid
  Win32 application." This is a documented Rust limitation, not a bug to wait out.
- theurge's invocation layer is inconsistent: some paths route through
  `bash -c` (correct), but the tabtarget-invocation primitives
  (`rbtdri_invoke_impl`, `rbtdrf_run_tt`) exec the `.sh` directly via
  `Command::new(tabtarget)`. Those paths cannot have run green on Cygwin —
  RBTDRX is forward-looking scaffolding and the Cygwin bring-up is unfinished.

## Locked decisions

- **Commit to Route A first — not route-neutral.** Build with the
  already-installed `stable-x86_64-pc-windows-gnu` toolchain (rustc 1.95.0 at
  `/cygdrive/c/Users/cygwin/.cargo/bin`; theurge's deps are pure-Rust, so no C
  toolchain / no system libs needed), and **finish the bring-up by routing every
  tabtarget invocation through `bash` instead of exec'ing the `.sh` directly.**
  RBTDRX stays and handles the path strings.
- **Route C is the documented fallback, not the opening move.** A first-class
  `x86_64-pc-cygwin` target exists (Rust 1.86+, Tier 3) and is semantically the
  "correct" answer — but it ships no precompiled std (needs nightly `-Z
  build-std`), and requires Cygwin `gcc` + LLVM ≥20.1. Fall back to it only if
  the native-binary path translation proves too leaky to finish Route A.
- **Toolchain installs are deferred to the experiment, not the dep pace.** The
  dep-install pace carries only route-independent knowns; `gcc`/`rust`/LLVM are
  Route-C-specific and must not be installed until/unless Route A is abandoned.
- **Scope is the runtime-free tier.** Container tiers (service/crucible) need a
  real container runtime; on Cygwin the only `docker` is Windows Docker Desktop,
  with a separate bind-mount path-translation wall RBTDRX does not cover.
  Per existing discipline, container tests run on the WSL side. This heat
  targets the **fast/validation** tier only.
  *(Superseded 2026-06-03 — see "Progress & scope" above: the crucible tier now
  runs on Cygwin Docker Desktop via the two Windows-docker membranes.)*

## What done looks like

`tt/rbw-ts.TestSuite.fast.sh` runs green over `cygwin@rocket`. A legitimate
alternative outcome is honest: the launch-boundary fix is proven and the
remaining work scoped, with heavier surgery deferred — the experiment is
exploratory and may reveal more than the release timeline wants to absorb.

## Known gaps to close

- **PATH visibility.** `cargo` exists on the host but is not on the
  non-interactive ssh command channel's PATH (rustup wired the Windows user
  PATH; the sshd-spawned shell that runs tabtargets does not see it). Make
  `.cargo/bin` resolvable to that shell — most robustly via the Windows *system*
  PATH or `~/.ssh/environment`.
- **The launch-boundary code change** in `rbtdri`/`rbtdrf` (above).
- **shellcheck path-argument boundary (deferred — off the fast-tier critical
  path).** `shellcheck` has no native Cygwin package (Haskell/GHC is
  incompatible with Cygwin's path layout); the install is the official Windows
  `shellcheck.exe` in `/usr/local/bin` (admin-placed — the unprivileged account
  cannot write there). It resolves and runs, but as a Windows-native tool it
  cannot open Cygwin `/`-style path arguments (`openBinaryFile: does not
  exist`) — it needs Windows-form paths (`cygpath -w`) or stdin. This is
  RBTDRX-family path translation but a **distinct site** from the `.sh`-launch
  boundary above: a Windows *tool* receiving Cygwin path *arguments*, not a
  `.sh` being `CreateProcess`'d. It surfaces only when the harness drives
  shellcheck in `rbw-tr` QualifyRelease — never in the fast/validation tier this
  heat targets. Close it if/when QualifyRelease is brought to Cygwin.

## Paces

### windows-docker-finish-and-requal (₢BVAAD) [complete]

**[260603-0604] complete**

## Character
Finish-out + cross-platform requal. Mostly bounded mechanical steps; the docker-driver consolidation (step 3) needs design judgment.

## Landed (orientation — mechanisms live in the commits, not restated)
Heat goal MET: `rbw-ts dogfight` green on BOTH cygwin@rocket and macOS. Three Windows-native-docker walls under Cygwin, fixed + proven green:
- docker build path args -> zrbfc_native_path_capture (rbfc_FoundryCore.sh) [landed pre-resume]
- docker login store -> rbgo_docker_login self-writes the base64 auths fileStore on the wincred "logon session" signature [7fd74e97]. Mechanism: login STORE always routes to the Windows helper (no config diverts it); push RETRIEVES from the auths map directly. Corroborated docker/cli#4353/#1263.
- docker cp dest -> zrbndb_native_path_capture (rbndb_base.sh), a DRAFT duplicate of zrbfc's [0f5ade76]
All three bends are off-Cygwin identity (macOS proved it). A pre-existing bash-4 `mapfile` spook in rbfl_FoundryLedger.sh (broke macOS abjure) fixed to a bash-3.2 while-read count [9bab941e, local — unpushed].

## Remaining
1. Cygwin crucible (tadmor, moriah): sized at ~1 wall — the docker compose CLI path args (-f / --env-file / --project-directory) mangle absolute /cygdrive paths like docker cp did; normalize them in zrbob_compose (rbob_bottle.sh). Compose already resolves the relative volume mounts (the reason the operator chose compose). Then a confirmation charge — open question is whether the sentry enclave networking comes up through Docker Desktop's WSL2 backend. Do this WHILE Cygwin holds the governor.
2. Requalify cerebro (Linux) + WSL with current changes.
3. Docker-driver consolidation: fold the 3 identical /cygdrive normalizer sites (rbfc, rbndb, rbob) + the login auths bend into one Windows-docker adapter. Design conversation, not mechanical.
4. Wind-down: push 9bab941e; BCG-clean the rbgo bend (the `local` declared mid-loop + the piped `$(... | openssl)` capture); clean cygwin@rocket of probe scripts (rbprobe_*.sh) + the live-token ~/.docker/config.json, and the GAR test artifacts (cygprobe/probe:*, graft g260323* hallmarks).

## IAM gotcha (durable, operator-relevant)
Shared depot; per-host governor mantling obsoletes the other hosts' invests. canonical-invest's divest->invest can race IAM propagation ("SA already exists"); pre-divest + roster-confirm-0 before a dogfight to de-risk. The `canonical-invest` fixture (skirmish suite) re-establishes a host's creds without levying a depot.

## Done
All targets green (Cygwin foundry + crucible, macOS, Linux/cerebro, WSL); the Windows-docker membranes consolidated into one adapter (or that consolidation explicitly carved into its own pace); draft duplications + BCG debt cleared; probe/test artifacts cleaned.

**[260603-0320] rough**

## Character
Finish-out + cross-platform requal. Mostly bounded mechanical steps; the docker-driver consolidation (step 3) needs design judgment.

## Landed (orientation — mechanisms live in the commits, not restated)
Heat goal MET: `rbw-ts dogfight` green on BOTH cygwin@rocket and macOS. Three Windows-native-docker walls under Cygwin, fixed + proven green:
- docker build path args -> zrbfc_native_path_capture (rbfc_FoundryCore.sh) [landed pre-resume]
- docker login store -> rbgo_docker_login self-writes the base64 auths fileStore on the wincred "logon session" signature [7fd74e97]. Mechanism: login STORE always routes to the Windows helper (no config diverts it); push RETRIEVES from the auths map directly. Corroborated docker/cli#4353/#1263.
- docker cp dest -> zrbndb_native_path_capture (rbndb_base.sh), a DRAFT duplicate of zrbfc's [0f5ade76]
All three bends are off-Cygwin identity (macOS proved it). A pre-existing bash-4 `mapfile` spook in rbfl_FoundryLedger.sh (broke macOS abjure) fixed to a bash-3.2 while-read count [9bab941e, local — unpushed].

## Remaining
1. Cygwin crucible (tadmor, moriah): sized at ~1 wall — the docker compose CLI path args (-f / --env-file / --project-directory) mangle absolute /cygdrive paths like docker cp did; normalize them in zrbob_compose (rbob_bottle.sh). Compose already resolves the relative volume mounts (the reason the operator chose compose). Then a confirmation charge — open question is whether the sentry enclave networking comes up through Docker Desktop's WSL2 backend. Do this WHILE Cygwin holds the governor.
2. Requalify cerebro (Linux) + WSL with current changes.
3. Docker-driver consolidation: fold the 3 identical /cygdrive normalizer sites (rbfc, rbndb, rbob) + the login auths bend into one Windows-docker adapter. Design conversation, not mechanical.
4. Wind-down: push 9bab941e; BCG-clean the rbgo bend (the `local` declared mid-loop + the piped `$(... | openssl)` capture); clean cygwin@rocket of probe scripts (rbprobe_*.sh) + the live-token ~/.docker/config.json, and the GAR test artifacts (cygprobe/probe:*, graft g260323* hallmarks).

## IAM gotcha (durable, operator-relevant)
Shared depot; per-host governor mantling obsoletes the other hosts' invests. canonical-invest's divest->invest can race IAM propagation ("SA already exists"); pre-divest + roster-confirm-0 before a dogfight to de-risk. The `canonical-invest` fixture (skirmish suite) re-establishes a host's creds without levying a depot.

## Done
All targets green (Cygwin foundry + crucible, macOS, Linux/cerebro, WSL); the Windows-docker membranes consolidated into one adapter (or that consolidation explicitly carved into its own pace); draft duplications + BCG debt cleared; probe/test artifacts cleaned.

**[260603-0318] rough**

## Character
Finish-out + cross-platform requal. Mostly bounded mechanical steps; the docker-driver consolidation (step 3) needs design judgment.

## Landed (orientation — mechanisms live in the commits, not restated)
Heat goal MET: `rbw-ts dogfight` green on BOTH cygwin@rocket and macOS. Three Windows-native-docker walls under Cygwin, fixed + proven green:
- docker build path args -> zrbfc_native_path_capture (rbfc_FoundryCore.sh) [landed pre-resume]
- docker login store -> rbgo_docker_login self-writes the base64 auths fileStore on the wincred "logon session" signature [7fd74e97]. Mechanism: login STORE always routes to the Windows helper (no config diverts it); push RETRIEVES from the auths map directly. Corroborated docker/cli#4353/#1263.
- docker cp dest -> zrbndb_native_path_capture (rbndb_base.sh), a DRAFT duplicate of zrbfc's [0f5ade76]
All three bends are off-Cygwin identity (macOS proved it). A pre-existing bash-4 `mapfile` spook in rbfl_FoundryLedger.sh (broke macOS abjure) fixed to a bash-3.2 while-read count [9bab941e, local — unpushed].

## Remaining
1. Cygwin crucible (tadmor, moriah): sized at ~1 wall — the docker compose CLI path args (-f / --env-file / --project-directory) mangle absolute /cygdrive paths like docker cp did; normalize them in zrbob_compose (rbob_bottle.sh). Compose already resolves the relative volume mounts (the reason the operator chose compose). Then a confirmation charge — open question is whether the sentry enclave networking comes up through Docker Desktop's WSL2 backend. Do this WHILE Cygwin holds the governor.
2. Requalify cerebro (Linux) + WSL with current changes.
3. Docker-driver consolidation: fold the 3 identical /cygdrive normalizer sites (rbfc, rbndb, rbob) + the login auths bend into one Windows-docker adapter. Design conversation, not mechanical.
4. Wind-down: push 9bab941e; BCG-clean the rbgo bend (the `local` declared mid-loop + the piped `$(... | openssl)` capture); clean cygwin@rocket of probe scripts (rbprobe_*.sh) + the live-token ~/.docker/config.json, and the GAR test artifacts (cygprobe/probe:*, graft g260323* hallmarks).

## IAM gotcha (durable, operator-relevant)
Shared depot; per-host governor mantling obsoletes the other hosts' invests. canonical-invest's divest->invest can race IAM propagation ("SA already exists"); pre-divest + roster-confirm-0 before a dogfight to de-risk. The `canonical-invest` fixture (skirmish suite) re-establishes a host's creds without levying a depot.

## Done
All targets green (Cygwin foundry + crucible, macOS, Linux/cerebro, WSL); the Windows-docker membranes consolidated into one adapter (or that consolidation explicitly carved into its own pace); draft duplications + BCG debt cleared; probe/test artifacts cleaned.

**[260602-1426] rough**

## Character
Pale-boundary workaround — characterize, contain at one membrane.

## Goal
rbgo_docker_login (rbgo_OAuth.sh) succeeds on an uncontrolled Cygwin host so the conjure pouch's GAR push completes — the remaining block to a green dogfight on cygwin@rocket.

## The wall
docker login authenticates but fails to SAVE the credential: Docker Desktop's docker-credential-wincred returns "A specified logon session does not exist" — a headless sshd session has no interactive Windows logon for the credential vault. The token mint and the docker-build path are sound; only credential persistence fails.

## Done
The pouch push completes on cygwin@rocket. At the Pale (docker's own credential store): contain at the rbgo_docker_login membrane with a logged bend and a removal condition; do not suppress real login errors.

### foundry-docker-cygwin-paths (₢BVAAC) [complete]

**[260603-0608] complete**

## Character
Path-translation fix — landed; remaining work is the integration gate.

## Goal
Both foundry docker builds — the conjure "pouch" (rbfd_FoundryDirectorBuild.sh) and the local kludge (rbfk_kludge.sh) — succeed on an uncontrolled Cygwin host.

## Landed (ce106ec0a)
zrbfc_native_path_capture normalizes docker path args (/cygdrive/X -> X:/, BURD_OSTYPE-gated, sentinel-free) at both build sites, plus a foundry-path fast fixture. Verified on cygwin@rocket: kludge build green (rbw-fk); the pouch's docker build — the proven-failure absolute -f — accepted in the real conjure (dogfight), context image built and tagged.

## Done — final gate
tt/rbw-ts.TestSuite.dogfight.sh runs green on cygwin@rocket. The dogfight currently stops at the conjure GAR push (docker-login wall), so this gate depends on the preceding docker-login pace; run it after that lands.

**[260602-1426] rough**

## Character
Path-translation fix — landed; remaining work is the integration gate.

## Goal
Both foundry docker builds — the conjure "pouch" (rbfd_FoundryDirectorBuild.sh) and the local kludge (rbfk_kludge.sh) — succeed on an uncontrolled Cygwin host.

## Landed (ce106ec0a)
zrbfc_native_path_capture normalizes docker path args (/cygdrive/X -> X:/, BURD_OSTYPE-gated, sentinel-free) at both build sites, plus a foundry-path fast fixture. Verified on cygwin@rocket: kludge build green (rbw-fk); the pouch's docker build — the proven-failure absolute -f — accepted in the real conjure (dogfight), context image built and tagged.

## Done — final gate
tt/rbw-ts.TestSuite.dogfight.sh runs green on cygwin@rocket. The dogfight currently stops at the conjure GAR push (docker-login wall), so this gate depends on the preceding docker-login pace; run it after that lands.

**[260602-1345] rough**

## Character
Contained path-translation fix; mostly mechanical.

## Goal
Both foundry docker builds — the conjure "pouch" (rbfd_FoundryDirectorBuild.sh) and the local kludge (rbfk_kludge.sh) — succeed on an uncontrolled Cygwin host.

## Locked approach (260602)
- One BCG `_capture` path normalizer, contract: `/cygdrive/X/...` -> `X:/...`; a relative OR already-native path passes through UNCHANGED; a bare-absolute-POSIX path (leading `/` but not `/cygdrive`) returns 1 (unsurveyed). Identity off-Cygwin (BURD_OSTYPE gate). Pure `/cygdrive` parameter expansion — mirrors RBTDRX's fast path and zrbte_kindle. NO cygpath (prior pace evicted it from kit bash).
- Route EVERY docker path arg through it uniformly — the `-f` dockerfile AND the build-context positional, at BOTH sites. Do not special-case which "needs" it: the normalizer is a no-op where a path is already relative/native, so wrapping is free, and uniform routing kills two fragile assumptions — dockerfile-inside-context is NOT schema-guaranteed, and RBRR_VESSEL_DIR-renders-relative is a contract, not enforced.
- Consume each normalized path via the BCG two-line guarded capture into a `local` (`local z; z=$(...) || buc_die`) BEFORE the docker line; the docker invocation uses the locals. Do NOT inline `$(normalize ...)` in the command args (the per-capture `|| buc_die` guard cannot attach inside a command's arg list), and NOT `local -r z=$(...)` (masks the substitution exit status / locks an empty value).
- The pouch's generated absolute `-f` (`${BURD_TEMP_DIR}/rbfd_context_*`) is the proven failure; the rest are belt-and-suspenders (passthrough today, correct if a vessel ever renders absolute).

## Done
Normalizer has a basic test for all four cases (`/cygdrive` transform, relative passthrough, native passthrough, off-Cygwin identity). Pouch and kludge builds succeed on cygwin@rocket.

**[260602-1341] rough**

## Character
Contained path-translation fix; narrow scope, mostly mechanical.

## Goal
The conjure "pouch" context build (rbfd_FoundryDirectorBuild.sh) succeeds on an uncontrolled Cygwin host. Today it fails because its generated `-f` context Dockerfile is an absolute `${BURD_TEMP_DIR}/...` path, and Windows docker.exe cannot resolve a `/cygdrive` path.

## Scope pinned (260602)
- The kludge build (rbfk_kludge.sh) does NOT need touching: vessel conjure paths derive from RBRR_VESSEL_DIR, which the regime contract renders repo-relative ("relative to repo root"), with the Dockerfile inside its context — so docker.exe resolves them against its native cwd. Cygwin-safe as-is. (A one-line vessel-regime render confirms, but the pouch is the established failure regardless.)
- The pouch is the lone failing site, and its lone bad arg is the generated absolute `-f` (`${BURD_TEMP_DIR}/rbfd_context_*`); the context positional there is already relative.

## Locked approach
- One BCG `_capture` path normalizer: pure `/cygdrive/X/ -> X:/` parameter expansion, gated on BURD_OSTYPE=cygwin, identity + fail-fast off-pattern. Mirrors RBTDRX's `/cygdrive` fast path and zrbte_kindle's manifest-path transform. NO cygpath — the prior pace evicted it from kit bash (cupel flags it).
- Apply to the pouch's absolute `-f`. House it reusable — a full skirmish run may surface further absolute docker path args.

## Done
Normalizer has a basic test (identity off-Cygwin; `/cygdrive`->drive on). The pouch build succeeds on cygwin@rocket.

**[260602-1336] rough**

## Character
Contained path-translation fix; designed shape, mostly mechanical.

## Goal
Foundry docker builds resolve their path arguments on an uncontrolled Cygwin host, so the conjure "pouch" context build (rbfd_FoundryDirectorBuild.sh) and the local kludge build (rbfk_kludge.sh) succeed. Today a Windows-native docker.exe receives a `/cygdrive/...` context and `CreateFile` fails.

## Locked approach (decided 260602)
- One BCG `_capture` path normalizer: pure `/cygdrive/X/... -> X:/...` parameter-expansion transform, gated on `BURD_OSTYPE=cygwin`, identity off-Cygwin, fail-fast on any unsurveyed shape. Mirrors RBTDRX's existing `/cygdrive` fast path (rbtdrx_platform.rs) and zrbte_kindle's manifest-path transform — same technique, in kit bash.
- NO cygpath. The prior pace deliberately evicted cygpath from kit bash (the cupel command-discipline scanner flags it); do not reintroduce it. Pure bash only.
- Wrap the `-f` dockerfile and the build-context positional at the two foundry build sites above. The pouch is the proven failure; the kludge is the same `docker build` pattern (confirm whether its rendered context is already relative — it may not need wrapping).

## Done
Normalizer has a basic test (identity off-Cygwin; `/cygdrive`->drive on-Cygwin). A wrapped docker build succeeds on cygwin@rocket. Full skirmish-green is the next pace's gate, not this one's.

### install-known-cygwin-deps (₢BVAAA) [complete]

**[260602-0641] complete**

## Character

Mechanical, operator-driven. Install the route-independent Cygwin packages on
the stock `cygwin@rocket` box.

## Scope

Install via Cygwin `setup-x86_64.exe`: `tmux`, `shellcheck`, `curl`. The `curl`
package makes `/usr/bin/curl` shadow the Windows `system32` curl — a platform-
variant hazard per BCG's Platform-Variant Command Guidance.

## Locked

No `gcc` / `rust` / LLVM here — those are Route-C-specific toolchain, deferred to
the experiment pace by ₣BV paddock decision.

## Out of scope

Remote automation — the operator runs `setup` interactively at the machine.

## Done

`tmux` and `shellcheck` present; `command -v curl` resolves to `/usr/bin/curl`
(Cygwin), not the Windows `system32` curl.

**[260601-0733] rough**

## Character

Mechanical, operator-driven. Install the route-independent Cygwin packages on
the stock `cygwin@rocket` box.

## Scope

Install via Cygwin `setup-x86_64.exe`: `tmux`, `shellcheck`, `curl`. The `curl`
package makes `/usr/bin/curl` shadow the Windows `system32` curl — a platform-
variant hazard per BCG's Platform-Variant Command Guidance.

## Locked

No `gcc` / `rust` / LLVM here — those are Route-C-specific toolchain, deferred to
the experiment pace by ₣BV paddock decision.

## Out of scope

Remote automation — the operator runs `setup` interactively at the machine.

## Done

`tmux` and `shellcheck` present; `command -v curl` resolves to `/usr/bin/curl`
(Cygwin), not the Windows `system32` curl.

### requal-cerebro-and-wsl (₢BVAAE) [abandoned]

**[260604-2011] abandoned**

## Character
Mechanical cross-platform validation; the one judgment is reading the conntrack verdict.

Validate this heat's Windows-docker changes off-Windows: run the suites on cerebro (native Linux) and rocket's WSL side. The path normalizers are BURD_OSTYPE-gated, so expect identity behavior there — confirm it, no regression. Capture the conntrack_spoofed_ack verdict on each platform; that cross-platform read is the decisive input for the conntrack-adjudication pace — siege and blockade both breach it on Cygwin Docker Desktop, so the open question is whether native Linux/WSL breaches it too.

## Done
siege + blockade green (or differences characterized) on cerebro and WSL, with the conntrack_spoofed_ack verdict recorded per platform.

**[260603-0600] rough**

## Character
Mechanical cross-platform validation; the one judgment is reading the conntrack verdict.

Validate this heat's Windows-docker changes off-Windows: run the suites on cerebro (native Linux) and rocket's WSL side. The path normalizers are BURD_OSTYPE-gated, so expect identity behavior there — confirm it, no regression. Capture the conntrack_spoofed_ack verdict on each platform; that cross-platform read is the decisive input for the conntrack-adjudication pace — siege and blockade both breach it on Cygwin Docker Desktop, so the open question is whether native Linux/WSL breaches it too.

## Done
siege + blockade green (or differences characterized) on cerebro and WSL, with the conntrack_spoofed_ack verdict recorded per platform.

### conntrack-spoofed-ack-adjudication (₢BVAAF) [complete]

**[260604-2331] complete**

## Character
Investigation → conditional fix; security-relevant. Reconfirm the platform split first, then adjudicate.

Substrate: all execution rides branch bv-conntrack-20260604-BVAAF (off the requal wrap 80b681e62); git is the only transport to the rocket funduses (wsl@rocket, native docker; cygwin@rocket, Docker Desktop). Before each run, force the fundus to the branch's pushed tip via the paddock's sanctioned hard-sync to origin/bv-conntrack-20260604-BVAAF.

Integrity under kludge: siege kludges host-local on the fundus and self-notches a stamp commit there, so every run mutates the branch. Keep it single-lined — pull --rebase the fundus commit into the curia and push before the next fundus force-syncs, so WSL and Cygwin always land on the same integral tip. (Fundus needs git user.name/email or the kludge notch dies exit 128 — paddock.)

Reconfirm: run siege (kludge-tadmor + tadmor crucible) on WSL first, then Cygwin DD, and compare the conntrack_spoofed_ack verdict. A clean split (WSL green; Cygwin 45/46, the recorded baseline) is platform divergence; a breach on both is a latent egress gap.

Mechanism for the adjudication — sortie_conntrack_spoofed_ack (rbida_sorties.rs) + Memos/memo-20260603-cygwin-ifrit-first-run-conntrack.md: the "response" is a RST from the allowed host returning through a conntrack entry the lone ACK created (nf_conntrack_tcp_loose=1 on the shared WSL2 kernel, host value confirmed on wsl@rocket), past a dest-based (not state-gated) allowlist in the sentry FORWARD chain (common-sentry-context/rbjs_sentry.sh).

## Done
Verdict explained (divergence vs real gap), grounded in the cross-platform verdicts. If a real gap: sentry egress hardened (e.g. tcp_loose=0, drop INVALID, or state-gate the allowlist) and re-tested green on both platforms. If divergence: documented with rationale and the suite verdict reconciled across backends.

**[260604-2104] rough**

## Character
Investigation → conditional fix; security-relevant. Reconfirm the platform split first, then adjudicate.

Substrate: all execution rides branch bv-conntrack-20260604-BVAAF (off the requal wrap 80b681e62); git is the only transport to the rocket funduses (wsl@rocket, native docker; cygwin@rocket, Docker Desktop). Before each run, force the fundus to the branch's pushed tip via the paddock's sanctioned hard-sync to origin/bv-conntrack-20260604-BVAAF.

Integrity under kludge: siege kludges host-local on the fundus and self-notches a stamp commit there, so every run mutates the branch. Keep it single-lined — pull --rebase the fundus commit into the curia and push before the next fundus force-syncs, so WSL and Cygwin always land on the same integral tip. (Fundus needs git user.name/email or the kludge notch dies exit 128 — paddock.)

Reconfirm: run siege (kludge-tadmor + tadmor crucible) on WSL first, then Cygwin DD, and compare the conntrack_spoofed_ack verdict. A clean split (WSL green; Cygwin 45/46, the recorded baseline) is platform divergence; a breach on both is a latent egress gap.

Mechanism for the adjudication — sortie_conntrack_spoofed_ack (rbida_sorties.rs) + Memos/memo-20260603-cygwin-ifrit-first-run-conntrack.md: the "response" is a RST from the allowed host returning through a conntrack entry the lone ACK created (nf_conntrack_tcp_loose=1 on the shared WSL2 kernel, host value confirmed on wsl@rocket), past a dest-based (not state-gated) allowlist in the sentry FORWARD chain (common-sentry-context/rbjs_sentry.sh).

## Done
Verdict explained (divergence vs real gap), grounded in the cross-platform verdicts. If a real gap: sentry egress hardened (e.g. tcp_loose=0, drop INVALID, or state-gate the allowlist) and re-tested green on both platforms. If divergence: documented with rationale and the suite verdict reconciled across backends.

**[260603-0601] rough**

## Character
Investigation, then a conditional fix; security-relevant — may warrant jumping ahead of the cleanup paces.

siege and blockade both BREACHED conntrack_spoofed_ack on Cygwin Docker Desktop (a stateless spoofed ACK to an allowlisted dest drew a response). Adjudicate platform-divergence vs latent egress gap using the cross-platform verdicts and the sortie source (rbtid, "conntrack-spoofed-ack"). Likely mechanism: the sentry's RELATED,ESTABLISHED accept combined with nf_conntrack_tcp_loose adopting a lone mid-stream ACK as established. See Memos/memo-20260603-cygwin-ifrit-first-run-conntrack.md.

## Done
Verdict explained (platform-divergence vs real gap). If a real gap: sentry egress hardened (e.g. tcp_loose=0, drop INVALID, or state-gate the allowlist) and re-tested green. If divergence: documented with rationale, and the suite's verdict reconciled across platforms.

### conntrack-provenance-negative-control (₢BVAAI) [complete]

**[260605-1019] complete**

## Character
Judgment plus a real fixture build; security-relevant. Closes epistemic debt the conntrack hardening incurred.

## Goal
conntrack_spoofed_ack now suppresses a would-be breach by L2 provenance, so its SECURE verdict is unfalsifiable — green could mean containment held, or that the probe/capture/classify pipeline silently broke (egress never left, AF_PACKET capture missed the reply, or the gateway-MAC compare is wrong). Build a deliberately-weakened-sentry nameplate whose loosened return path lets the lone-ACK reply transit the sentry (arriving with gateway-MAC provenance), and assert the sortie BREACHes there — proving the whole pipeline still goes red on a real return-path failure.

## Cinched
- Validates the SentryMediated -> BREACH direction only. OffPath -> SECURE suppression is certified solely by running on real Windows DD; the control does not prove quirk suppression.
- Pilots a discipline: any sortie that suppresses a would-be breach on an observation carries a negative control proving the suppression is conditional. This is currently the only such gate.
- Live upstream plus container runtime -> service/crucible tier, not fast. The new weakened-sentry nameplate is the real cost.

## Done
A weakened-sentry nameplate forwards the lone-ACK reply to the bottle; conntrack_spoofed_ack asserts BREACH against it and SECURE against the hardened nameplates, and the contrast runs green in the suite.

Sources: Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md; RBSIP "Substrate Quirks" (rbsq_wdd_offpath_reply); sortie_conntrack_spoofed_ack in rbida_sorties.rs; RBS0 Security Properties ("architectural reasoning alone has been empirically insufficient").

**[260604-2325] rough**

## Character
Judgment plus a real fixture build; security-relevant. Closes epistemic debt the conntrack hardening incurred.

## Goal
conntrack_spoofed_ack now suppresses a would-be breach by L2 provenance, so its SECURE verdict is unfalsifiable — green could mean containment held, or that the probe/capture/classify pipeline silently broke (egress never left, AF_PACKET capture missed the reply, or the gateway-MAC compare is wrong). Build a deliberately-weakened-sentry nameplate whose loosened return path lets the lone-ACK reply transit the sentry (arriving with gateway-MAC provenance), and assert the sortie BREACHes there — proving the whole pipeline still goes red on a real return-path failure.

## Cinched
- Validates the SentryMediated -> BREACH direction only. OffPath -> SECURE suppression is certified solely by running on real Windows DD; the control does not prove quirk suppression.
- Pilots a discipline: any sortie that suppresses a would-be breach on an observation carries a negative control proving the suppression is conditional. This is currently the only such gate.
- Live upstream plus container runtime -> service/crucible tier, not fast. The new weakened-sentry nameplate is the real cost.

## Done
A weakened-sentry nameplate forwards the lone-ACK reply to the bottle; conntrack_spoofed_ack asserts BREACH against it and SECURE against the hardened nameplates, and the contrast runs green in the suite.

Sources: Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md; RBSIP "Substrate Quirks" (rbsq_wdd_offpath_reply); sortie_conntrack_spoofed_ack in rbida_sorties.rs; RBS0 Security Properties ("architectural reasoning alone has been empirically insufficient").

### offpath-blocked-dest-probe (₢BVAAJ) [complete]

**[260605-1046] complete**

## Character
Intricate but mostly mechanical — a near-variant of the existing provenance-classified sortie. Security-relevant.

## Goal
The rbsq_wdd_offpath_reply quirk is judged benign on one untested premise: the substrate answers only egress the sentry *allowed*, because blocked destinations are dropped at the sentry before the substrate ever sees them. Convert that to a tested fact — send the provenance-classified lone-ACK to a non-allowlisted destination; any reply, off-path or sentry-mediated, is a BREACH, because the substrate reached the bottle on behalf of a destination the sentry never approved (a real path-in, not a quirk).

## Cinched
- This is the empirical backstop for the quirk's keystone clause. A BREACH on Windows DD voids the quirk's benign classification and becomes a real finding.
- Unilateral; runs on the existing hardened nameplates, no new nameplate. The blocked-dest choice is a mount-time decision.
- Selector `offpath-blocked-dest` (deliberately NOT the conntrack-* family — the keystone is the egress allowlist + off-path premise, not conntrack state); blocked dest is 8.8.8.8 (suite-wide forbidden-dest convention).

## Landed
- Sortie written and wired (commit e730fee5): ifrit selector/enum/from_str/as_str/dispatch + sortie_offpath_blocked_dest reusing send_lone_ack_classify_provenance; theurge wrapper + RBTDRC_CASES_SECURITY case (runs in siege/tadmor and blockade/moriah).
- **Passes in current form on macOS** — siege green 61/61, NoResponse -> SECURE (sentry dropped egress to 8.8.8.8). This proves the control compiles on Linux, wires end-to-end, and goes green on a healthy substrate.

## Remaining (the load-bearing legs)
The control's teeth are on Windows Docker Desktop, where the off-path quirk actually lives. Must run green (or surface a real finding) on **both the WSL and the Cygwin users of rocket** — not just one. macOS green alone does not close this pace.

## Done
offpath-blocked-dest runs SECURE (or surfaces a real finding) on the WSL and Cygwin rocket substrates, in addition to the confirmed macOS pass.

Sources: Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md (Scope of exposure); RBSIP "Substrate Quirks" (rbsq_wdd_offpath_reply); sortie_conntrack_spoofed_ack in rbida_sorties.rs.

**[260605-0908] rough**

## Character
Intricate but mostly mechanical — a near-variant of the existing provenance-classified sortie. Security-relevant.

## Goal
The rbsq_wdd_offpath_reply quirk is judged benign on one untested premise: the substrate answers only egress the sentry *allowed*, because blocked destinations are dropped at the sentry before the substrate ever sees them. Convert that to a tested fact — send the provenance-classified lone-ACK to a non-allowlisted destination; any reply, off-path or sentry-mediated, is a BREACH, because the substrate reached the bottle on behalf of a destination the sentry never approved (a real path-in, not a quirk).

## Cinched
- This is the empirical backstop for the quirk's keystone clause. A BREACH on Windows DD voids the quirk's benign classification and becomes a real finding.
- Unilateral; runs on the existing hardened nameplates, no new nameplate. The blocked-dest choice is a mount-time decision.
- Selector `offpath-blocked-dest` (deliberately NOT the conntrack-* family — the keystone is the egress allowlist + off-path premise, not conntrack state); blocked dest is 8.8.8.8 (suite-wide forbidden-dest convention).

## Landed
- Sortie written and wired (commit e730fee5): ifrit selector/enum/from_str/as_str/dispatch + sortie_offpath_blocked_dest reusing send_lone_ack_classify_provenance; theurge wrapper + RBTDRC_CASES_SECURITY case (runs in siege/tadmor and blockade/moriah).
- **Passes in current form on macOS** — siege green 61/61, NoResponse -> SECURE (sentry dropped egress to 8.8.8.8). This proves the control compiles on Linux, wires end-to-end, and goes green on a healthy substrate.

## Remaining (the load-bearing legs)
The control's teeth are on Windows Docker Desktop, where the off-path quirk actually lives. Must run green (or surface a real finding) on **both the WSL and the Cygwin users of rocket** — not just one. macOS green alone does not close this pace.

## Done
offpath-blocked-dest runs SECURE (or surfaces a real finding) on the WSL and Cygwin rocket substrates, in addition to the confirmed macOS pass.

Sources: Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md (Scope of exposure); RBSIP "Substrate Quirks" (rbsq_wdd_offpath_reply); sortie_conntrack_spoofed_ack in rbida_sorties.rs.

**[260604-2326] rough**

## Character
Intricate but mostly mechanical — a near-variant of the existing provenance-classified sortie. Security-relevant.

## Goal
The rbsq_wdd_offpath_reply quirk is judged benign on one untested premise: the substrate answers only egress the sentry *allowed*, because blocked destinations are dropped at the sentry before the substrate ever sees them. Convert that to a tested fact — send the provenance-classified lone-ACK to a non-allowlisted destination; any reply, off-path or sentry-mediated, is a BREACH, because the substrate reached the bottle on behalf of a destination the sentry never approved (a real path-in, not a quirk).

## Cinched
- This is the empirical backstop for the quirk's keystone clause. A BREACH on Windows DD voids the quirk's benign classification and becomes a real finding.
- Unilateral; runs on the existing hardened nameplates, no new nameplate. The blocked-dest choice is a mount-time decision.

## Done
A provenance-aware lone-ACK to a blocked destination asserts no reply of any provenance; it runs SECURE across the platform matrix, or surfaces a real finding if it does not.

Sources: Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md (Scope of exposure); RBSIP "Substrate Quirks" (rbsq_wdd_offpath_reply); sortie_conntrack_spoofed_ack in rbida_sorties.rs.

### crucible-tier-cygwin-bringup (₢BVAAB) [complete]

**[260605-1131] complete**

## Character
Integration gate — verify the container-runtime crucible tier on Cygwin; surface and fix-or-scope any wall the conjure crucibles hit.

## Goal
siege (tether crucible) and the two conjure crucibles srjcl + pluml run green over cygwin@rocket. The standing-depot build chain is deliberately out of scope (see Scoped out).

## Preconditions
- Payor OAuth on rocket refreshed; Retriever SA fresh via canonical-invest — srjcl/pluml are conjure-mode, they auto-summon from the standing depot's GAR (same credential shape blockade already proved green on Cygwin).
- srjcl/pluml conjure hallmarks already ordained in the standing depot's GAR.
- siege carries its own kludge-tadmor predecessor, so it needs no pre-build.

## Scoped out (known gap, not abandoned)
onboarding-sequence on Cygwin — the local-kludge + cloud-ordain build orchestration. Justified: dogfight already proves cloud-build to summon to run on Cygwin, and blockade proves the airgap crucible plus credential chain; only the full ordain sequence stays unproven, and it costs cloud-build spend plus project setup that the heat's consumer-qualification claim does not require. Reconsider if a consumer build-on-Cygwin story becomes load-bearing.

## Done
siege, srjcl, and pluml green over cygwin@rocket; onboarding-sequence-on-Cygwin left as the recorded scoped-out gap.

**[260605-1102] rough**

## Character
Integration gate — verify the container-runtime crucible tier on Cygwin; surface and fix-or-scope any wall the conjure crucibles hit.

## Goal
siege (tether crucible) and the two conjure crucibles srjcl + pluml run green over cygwin@rocket. The standing-depot build chain is deliberately out of scope (see Scoped out).

## Preconditions
- Payor OAuth on rocket refreshed; Retriever SA fresh via canonical-invest — srjcl/pluml are conjure-mode, they auto-summon from the standing depot's GAR (same credential shape blockade already proved green on Cygwin).
- srjcl/pluml conjure hallmarks already ordained in the standing depot's GAR.
- siege carries its own kludge-tadmor predecessor, so it needs no pre-build.

## Scoped out (known gap, not abandoned)
onboarding-sequence on Cygwin — the local-kludge + cloud-ordain build orchestration. Justified: dogfight already proves cloud-build to summon to run on Cygwin, and blockade proves the airgap crucible plus credential chain; only the full ordain sequence stays unproven, and it costs cloud-build spend plus project setup that the heat's consumer-qualification claim does not require. Reconsider if a consumer build-on-Cygwin story becomes load-bearing.

## Done
siege, srjcl, and pluml green over cygwin@rocket; onboarding-sequence-on-Cygwin left as the recorded scoped-out gap.

**[260605-1101] rough**

## Character
Integration gate — verification, plus whatever walls skirmish still surfaces.

## Goal
skirmish runs green over cygwin@rocket — the full standing-depot chain on an uncontrolled Cygwin host, not just the fast tier. The bar before the uncontrolled-Cygwin bringup is considered done.

## Landed (this heat — jjx_log BV)
- Fast tier green on cygwin@rocket (cat repair + the prior pace's launch/platform boundaries).
- theurge native config reads nativized on Cygwin (rbtdrx_path_from_env); canonical-invest proven green end-to-end on cygwin@rocket — governor mantle + retriever/director invest, real GCP IAM.
- rocket station BURS_TINCTURE aligned to the live canonical depot.

## Remaining
Re-run skirmish on cygwin@rocket; characterize and fix-or-scope each wall it surfaces (crucible-charge bind-mounts the next likely one). Payor OAuth on rocket expires periodically — refresh before a run.

## Done
skirmish green over cygwin@rocket.

**[260602-1336] rough**

## Character
Integration gate — verification, plus whatever walls skirmish still surfaces.

## Goal
skirmish runs green over cygwin@rocket — the full standing-depot chain on an uncontrolled Cygwin host, not just the fast tier. The bar before the uncontrolled-Cygwin bringup is considered done.

## Landed (this heat — jjx_log BV)
- Fast tier green on cygwin@rocket (cat repair + the prior pace's launch/platform boundaries).
- theurge native config reads nativized on Cygwin (rbtdrx_path_from_env); canonical-invest proven green end-to-end on cygwin@rocket — governor mantle + retriever/director invest, real GCP IAM.
- rocket station BURS_TINCTURE aligned to the live canonical depot.

## Remaining
Re-run skirmish on cygwin@rocket; characterize and fix-or-scope each wall it surfaces (crucible-charge bind-mounts the next likely one). Payor OAuth on rocket expires periodically — refresh before a run.

## Done
skirmish green over cygwin@rocket.

**[260602-0826] rough**

## Character

Experiment, now largely proven — continuation requiring judgment. Read the ₣BV
paddock first, but note its central thesis is partly disproven (below) and the
paddock itself owes a curry.

## Landed (see this pace's commits — jjx_log BV)

theurge runs the runtime-free fast tier green on an uncontrolled Cygwin
(cygwin@rocket), save only the pre-existing violation under Remaining. The
launch and platform boundaries are solved, correcting the paddock thesis:

- bash does NOT export `$OSTYPE` to native children, so RBTDRX sat inert.
  Platform now arrives as a synthesized BURD regime member, `BURD_OSTYPE`
  (bud_dispatch + burd_regime); theurge's `rbtdrx_is_cygwin` reads it.
- A bare `bash` from a Windows-native binary resolves to System32's WSL
  launcher, never Cygwin's. theurge now derives the Cygwin bash via cygpath
  (`rbtdri_bash_program`), reusing RBTDRX's existing cygpath dependency.
- Also: cargo on the ssh PATH via rocket's `~/.bash_profile`; cargo
  `--manifest-path` translation in `rbte_engine.sh`.

## Remaining (in scope)

- Repair the pre-existing `cat` command-discipline violation in
  `Tools/rbk/rbgo_OAuth.sh` (BCG eviction table → bash builtin). It is what the
  kit-bash fixture (`rbtdru_kit_bash`) flags, and the last thing between here and
  a fully green fast suite. Not introduced by this heat, but fixed under it.

## Known gap (likely a follow-on, given the heat's locked runtime-free scope)

A whole cloud process has not been exercised from Cygwin — only the
runtime-free fast tier is proven. Recorded so it is not forgotten; whether it
belongs here or in a new heat is an operator call.

## Done

Fast suite green over cygwin@rocket, kit-bash included, after the `cat` repair.

**[260601-0733] rough**

## Character

Experiment requiring judgment. Finish theurge's Cygwin bring-up so the
runtime-free fast suite runs on an uncontrolled Cygwin. Read the ₣BV paddock
first — the launch-boundary thesis and the Route A / Route C decision live
there; this docket does not restate them.

## Scope (Route A — committed)

- Build theurge with the installed `stable-x86_64-pc-windows-gnu` toolchain
  (deps are pure-Rust; no C toolchain needed).
- Wire `.cargo/bin` onto the non-interactive ssh-channel PATH so
  `rbte_engine.sh`'s bare `cargo` resolves.
- Route tabtarget invocation through `bash` in `rbtdri`/`rbtdrf` — no direct
  `Command::new(.sh)`. RBTDRX stays for path strings.

## Done

`tt/rbw-ts.TestSuite.fast.sh` green over `cygwin@rocket`. Legitimate alternative:
the launch-boundary fix proven and remaining scope documented, with heavier
surgery deferred — the experiment may reveal more than the release timeline
wants to absorb.

### windows-docker-adapter-consolidation (₢BVAAG) [complete]

**[260605-1018] complete**

## Character
Design conversation, then a mechanical fold; needs judgment on the adapter's shape.

Three byte-identical /cygdrive path normalizers (zrbfc_native_path_capture, zrbndb_native_path_capture, zrbob_native_path_capture), the nested-mountpoint discipline (rbtid/project), and the rbgo headless-login auths-file bend are scattered Windows-docker membranes accreted across this heat. Fold them into one adapter. Off-Windows identity must be preserved (BURD_OSTYPE-gated); suites stay green. See Memos/memo-20260603-windows-docker-desktop-bind-mount.md.

## Done
One canonical Windows-docker adapter; the duplicate normalizers removed; suites green on Cygwin and at least one non-Windows platform.

**[260603-0601] rough**

## Character
Design conversation, then a mechanical fold; needs judgment on the adapter's shape.

Three byte-identical /cygdrive path normalizers (zrbfc_native_path_capture, zrbndb_native_path_capture, zrbob_native_path_capture), the nested-mountpoint discipline (rbtid/project), and the rbgo headless-login auths-file bend are scattered Windows-docker membranes accreted across this heat. Fold them into one adapter. Off-Windows identity must be preserved (BURD_OSTYPE-gated); suites stay green. See Memos/memo-20260603-windows-docker-desktop-bind-mount.md.

## Done
One canonical Windows-docker adapter; the duplicate normalizers removed; suites green on Cygwin and at least one non-Windows platform.

### cygwin-host-and-repo-winddown (₢BVAAH) [complete]

**[260605-1843] complete**

## Character
Cleanup; mostly mechanical, one decision (the kludge-stamp commits).

Wind down the spike's residue. On cygwin@rocket: remove probe scripts, the live-token ~/.docker/config.json, and GAR test artifacts (cygprobe/probe images, graft test hallmarks). Reconcile the cygwin-resident kludge-stamp commits (push as accepted temporary pollution, or discard). BCG-clean the rbgo login bend (the local declared mid-loop, the piped openssl capture). Regenerate the rbw-ts help string (still lists the pre-rename suite names) with its generated context doc.

## Done
cygwin@rocket and the repo clean; rbgo BCG debt cleared; rbw-ts help string current.

**[260603-0601] rough**

## Character
Cleanup; mostly mechanical, one decision (the kludge-stamp commits).

Wind down the spike's residue. On cygwin@rocket: remove probe scripts, the live-token ~/.docker/config.json, and GAR test artifacts (cygprobe/probe images, graft test hallmarks). Reconcile the cygwin-resident kludge-stamp commits (push as accepted temporary pollution, or discard). BCG-clean the rbgo login bend (the local declared mid-loop, the piped openssl capture). Regenerate the rbw-ts help string (still lists the pre-rename suite names) with its generated context doc.

## Done
cygwin@rocket and the repo clean; rbgo BCG debt cleared; rbw-ts help string current.

### writ-capture-color-decoupling (₢BVAAK) [complete]

**[260605-1925] complete**

## Character
Small fix; the diagnosis is done, the judgment is which seam to cut.

## Goal
theurge's sentry-config reads must parse a clean value regardless of terminal
color. rbtdrc_read_sentry_env reads an env var via writ; rbtdrc_filter_writ_output
(rbtdrc_crucible.rs) then strips buc status lines by their ANSI gray prefix — a
presentation attribute. On a cold render (NO_COLOR / unset TERM / pipe / dumb
terminal — e.g. plain ssh to cerebro) the prefix is absent, so rbob_writ's
"Writ to sentry: …" status line survives the filter and is parsed as the value,
failing rbtdrc_sentry_config_rp_filter ("unexpected RBRN_ENTRY_MODE 'rbob_writ
Writ to sentry: …'"). A correctness path keying on style as type — Interface
Contamination inside the harness.

## Cinched
- buym's NO_COLOR/TERM color gating is correct and stays; fix the consumer side.
- Must hold under NO_COLOR. rbob_writ status goes to stderr — find where the
  launcher folds it into the captured stdout; that merge is the deeper, cleaner
  seam to cut.

## Done
A cold-render siege on cerebro runs green; the writ value-read no longer depends
on ANSI color.

Refs: rbtdrc_filter_writ_output / rbtdrc_read_sentry_env (rbtdrc_crucible.rs); buym color gate (buym_yelp.sh).

**[260605-1014] rough**

## Character
Small fix; the diagnosis is done, the judgment is which seam to cut.

## Goal
theurge's sentry-config reads must parse a clean value regardless of terminal
color. rbtdrc_read_sentry_env reads an env var via writ; rbtdrc_filter_writ_output
(rbtdrc_crucible.rs) then strips buc status lines by their ANSI gray prefix — a
presentation attribute. On a cold render (NO_COLOR / unset TERM / pipe / dumb
terminal — e.g. plain ssh to cerebro) the prefix is absent, so rbob_writ's
"Writ to sentry: …" status line survives the filter and is parsed as the value,
failing rbtdrc_sentry_config_rp_filter ("unexpected RBRN_ENTRY_MODE 'rbob_writ
Writ to sentry: …'"). A correctness path keying on style as type — Interface
Contamination inside the harness.

## Cinched
- buym's NO_COLOR/TERM color gating is correct and stays; fix the consumer side.
- Must hold under NO_COLOR. rbob_writ status goes to stderr — find where the
  launcher folds it into the captured stdout; that merge is the deeper, cleaner
  seam to cut.

## Done
A cold-render siege on cerebro runs green; the writ value-read no longer depends
on ANSI color.

Refs: rbtdrc_filter_writ_output / rbtdrc_read_sentry_env (rbtdrc_crucible.rs); buym color gate (buym_yelp.sh).

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 D windows-docker-finish-and-requal
  2 C foundry-docker-cygwin-paths
  3 A install-known-cygwin-deps
  4 E requal-cerebro-and-wsl
  5 F conntrack-spoofed-ack-adjudication
  6 I conntrack-provenance-negative-control
  7 J offpath-blocked-dest-probe
  8 B crucible-tier-cygwin-bringup
  9 G windows-docker-adapter-consolidation
  10 H cygwin-host-and-repo-winddown
  11 K writ-capture-color-decoupling

DCAEFIJBGHK
xx·x·xx···x rbtdrc_crucible.rs
····xxx···· rbida_sorties.rs
·x·····xx·· rbtdrf_fast.rs
x······x·x· rbgo_OAuth.sh
·······x··x rbtdri_invocation.rs
·····xx···· rbida_attacks.rs
·x······x·· rbfd_FoundryDirectorBuild.sh, rbfk_kludge.sh, rbtdrm_manifest.rs
x·······x·· rbndb_base.sh, rbob_bottle.sh
x···x······ memo-20260603-cygwin-ifrit-first-run-conntrack.md
·········x· claude-rbk-tabtarget-context.md, rbq_Qualify.sh, rbz_zipper.sh
········x·· buc_command.sh, memo-20260605-bvaag-crossplatform-validation.md, memo-20260605-cerebro-noninteractive-cargo-path.md, rbfcb_BuildHost.sh
·······x··· bud_dispatch.sh, burd_regime.sh, rbtdrf_handbook.rs, rbtdrx_platform.rs, rbte_engine.sh
····x······ RBS0-SpecTop.adoc, RBSIP-ifrit_pentester.adoc, main.rs, memo-20260604-conntrack-spoofed-ack-adjudication.md
·x········· rbfc_FoundryCore.sh
x·········· .gitkeep, memo-20260603-windows-docker-desktop-bind-mount.md, rbgc_Constants.sh, rbw-ts.TestSuite.blockade.sh, rbw-ts.TestSuite.siege.sh, rbw-ts.TestSuite.tadmor.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 73 commits)

  1 D windows-docker-finish-and-requal
  2 C foundry-docker-cygwin-paths
  3 E requal-cerebro-and-wsl
  4 F conntrack-spoofed-ack-adjudication
  5 G windows-docker-adapter-consolidation
  6 J offpath-blocked-dest-probe
  7 I conntrack-provenance-negative-control
  8 B crucible-tier-cygwin-bringup
  9 H cygwin-host-and-repo-winddown
  10 K writ-capture-color-decoupling

123456789abcdefghijklmnopqrstuvwxyz
··x································  D  1c
···x·······························  C  1c
·····x·····························  E  1c
········xxxxx··xxxxxx··············  F  11c
·····················x···xx········  G  3c
······················x·····x······  J  2c
·······················x···x·······  I  2c
······························x····  B  1c
·······························xx··  H  2c
·································xx  K  2c
```

## Steeplechase

### 2026-06-05 19:25 - ₢BVAAK - W

Decoupled theurge's writ value-read from terminal color (commit b04b84ebd). Root cause was Interface Contamination inside the harness: bud_dispatch's default path folds the coordinator under 2>&1 so BUK's unified logs-buk transcript captures both streams, smearing rbob_writ's buc_step 'Writ to sentry' status line (stderr) onto the captured stdout; rbtdrc_filter_writ_output then stripped it by its ANSI gray prefix -- a presentation attribute absent on a cold render (NO_COLOR / dumb TERM / plain ssh), so the status line survived and was parsed as the value. Fix cuts the consumer out of the merge rather than filtering the symptom: added rbtdri_invoke_env (fixture-imprinted invoke threading child env, mirroring rbtdri_invoke_global) and ran rbtdrc_writ under BURD_NO_LOG=1, so dispatch neither folds stderr into stdout nor emits log-path headers and captured stdout is exactly the sentry command output. Deleted rbtdrc_filter_writ_output entirely and simplified all 5 call sites (read_sentry_env, rp_filter value, dnsmasq-log audit, pre/post ip-link snapshots) to read raw stdout. BUK's universal 2>&1 (load-bearing for the transcript across ~200 tabtargets) is untouched. Tests passed: build green; theurge unit suite 137/137; cold-render siege on cerebro (native docker, no color) 62/62 with the once-failing rbtdrc_sentry_config_rp_filter now PASSED; siege on cygwin@rocket (Windows Docker Desktop, the heat's actual uncontrolled-Cygwin target, color rendered) 62/62, rp_filter and its read_sentry_env siblings (prerouting_dnat, postrouting_masquerade) PASSED, and rbtdrc_sortie_conntrack_spoofed_ack PASSED on both fundi. rp_filter is thus proven green under both warm and cold render. Note: the cygwin fundus likely carries a host-local kludge-stamp commit from the kludge-mode tadmor fixture (accepted temporary pollution per paddock, left to elapse).

### 2026-06-05 19:01 - ₢BVAAK - n

Decouple theurge's writ value-read from terminal color by cutting the consumer out of BUK's stdout/stderr fold rather than filtering its symptom. Root cause: bud_dispatch's default path runs the coordinator under 2>&1 so the unified logs-buk transcript captures both streams; that fold smears rbob_writ's buc_step status line (stderr) onto the captured stdout, and rbtdrc_filter_writ_output then stripped it by its ANSI gray prefix -- a presentation attribute absent on a cold render (NO_COLOR / dumb TERM / plain ssh), so the status line survived the filter and was parsed as the value (Interface Contamination inside the harness). Fix: add rbtdri_invoke_env (fixture-imprinted invoke threading extra child env, mirroring rbtdri_invoke_global) and run rbtdrc_writ under BURD_NO_LOG=1. Under NO_LOG the dispatch neither folds stderr into stdout nor emits log-path headers, so captured stdout is exactly the sentry command's output; rbob_writ's buc_step stays on stderr and the value-read no longer depends on color. Deleted rbtdrc_filter_writ_output entirely and simplified all 5 call sites (read_sentry_env, rp_filter value, dnsmasq-log audit, pre/post ip-link snapshots) to consume raw stdout -- eliminating the contamination-shaped helper rather than leaving it dormant. The BUK universal 2>&1 (load-bearing for the unified transcript across ~200 tabtargets) is untouched; only this consumer is cut out of it. NO_LOG skips only the BURS station regime (logging metadata), not BURC/nameplate regime, so writ still resolves its sentry container (proven live by rbw-rnl running under NO_LOG). theurge keeps its own per-invocation stdout/stderr capture, so dropping the redundant logs-buk transcript for these probe writs costs no diagnostics. Build green; theurge unit tests 137/137. Cold-render siege on cerebro is the empirical proof, run next.

### 2026-06-05 18:43 - ₢BVAAH - W

Wound down the rbk-10 uncontrolled-cygwin spike residue across curia and fundus. Curia code (commit 93e4b2ec2): BCG-cleaned the rbgo headless-Cygwin docker-login bend — hoisted the mid-loop `local z_auth_b64` to function top and routed the credential base64 through the canonical rbgo_base64_encode_string_capture primitive, eliminating both the pipeline-inside-$() and the non-canonical `openssl base64`; refreshed the rbw-ts suite roster to the post-rename set (tadmor->siege, +blockade) in the rbz_zipper source string, the theurge-regenerated context doc, and the stale rationale comments in rbz_zipper/rbq_Qualify. Shellcheck 200 files clean. Host (cygwin@rocket): removed the 6 rbprobe_*.sh probe scripts, rbprobe_token.txt, and the live-token ~/.docker/config.json; force-resynced the fundus (git fetch + reset --hard origin/main, sanctioned destructive form) from a diverged state (2 stray host-local kludge-stamp commits ahead, ~30 behind) to clean+current. Operator decisions: kludge-stamp commit reconcile resolved to let-elapse (the bogus-identity stamps already on origin/main are permanent/moot); rustup-init.sh kept (Route-A toolchain bootstrap, not spike residue); cygprobe docker tags + remote GAR graft artifacts left to elapse; GAR abjure skipped. Note: the curia notch is local, not pushed — a push + fundus re-pull is the separate step if these fixes are wanted on the fundus.

### 2026-06-05 18:42 - ₢BVAAH - n

Wind down rbk-10 cygwin spike residue: BCG-clean the rbgo headless-Cygwin docker-login bend (hoist the mid-loop local to function top; route the credential base64 through the canonical rbgo_base64_encode_string_capture primitive, eliminating the inline pipeline-in-$() and the non-canonical 'openssl base64' in favor of 'openssl enc -base64 -A'). Refresh the rbw-ts suite roster to the post-rename set (tadmor->siege, +blockade): source help string in rbz_zipper.sh, regenerated claude-rbk-tabtarget-context.md via the theurge build, and the stale rationale comments in rbz_zipper.sh and rbq_Qualify.sh. Shellcheck 200 files clean. Host-side residue (probe scripts, live-token config.json) scrubbed on cygwin@rocket and the fundus resynced clean+current; those are operator-host actions, not tracked here.

### 2026-06-05 11:31 - ₢BVAAB - W

Crucible tier verified green on uncontrolled Cygwin (cygwin@rocket): siege 62/0/0 (kludge-tadmor predecessor + 61-case tadmor sortie set, incl. conntrack_spoofed_ack now PASSING — the BVAAI/BVAAJ negative-control adjudication holds on Cygwin), srjcl 3/0/0, pluml 5/0/0. Every crucible charged AND quenched clean (inactive confirmed). No walls to fix-or-scope — gate passed on first integrated run. Credential chain already live (Retriever + Payor JWT/OAuth probes green), so the Payor-OAuth reinstall + dogfight canonical-invest refresh was skipped. Conjure images (jupyter/plantuml) were cached on the fundus, so charges came up instantly: this run exercised the conjure resolution/credential path (Retriever reaches GAR, verified) but NOT a cold GAR layer pull — that cold-summon chain is what blockade already proved on Cygwin. onboarding-sequence-on-Cygwin remains the cinched scoped-out gap.

### 2026-06-05 11:01 - Heat - T

crucible-tier-cygwin-bringup

### 2026-06-05 10:46 - ₢BVAAJ - W

Verified the offpath-blocked-dest negative control green on both rocket Docker substrates, closing the pace's load-bearing legs. Ran siege (kludge-mode tadmor, no depot/GAR/OAuth) at origin/main 018320551 over the git-as-filesystem transport (no scp; ssh-propagated exit, clean trees both fundi): WSL (wsl@rocket, Docker 29.1.3) 62/0/0 exit 0, and Cygwin/Windows Docker Desktop (cygwin@rocket, Docker Desktop 28.3.2) 62/0/0 exit 0 — alongside the prior macOS 62/62. All three watched cases PASS on both fundi: rbtdrc_sortie_offpath_blocked_dest (NoResponse -> SECURE, no breach), rbtdrc_sortie_conntrack_pipeline_selfcheck (pure-logic detector intact), rbtdrc_sortie_conntrack_spoofed_ack (stayed SECURE). The decisive result is on Windows Docker Desktop, where the rbsq_wdd_offpath_reply off-path quirk lives: no reply of any provenance returned from the blocked destination 8.8.8.8, converting the quirk's untested keystone premise (blocked dests dropped at the sentry before the substrate sees them; substrate answers only sentry-allowed egress) into a tested fact on the actual quirk substrate. The quirk's benign classification holds; no finding surfaced. Caveat: proven on these substrate versions on rocket (WSL Docker 29.1.3, Win DD 28.3.2), not Windows Docker Desktop universally.

### 2026-06-05 10:19 - ₢BVAAI - W

Reframed the conntrack-provenance negative control after empirical study on a live tadmor crucible. Proved no real gateway-sourced reply can be staged on Linux: the spoofed ACK is conntrack-INVALID, so NAT/REJECT firewall levers never fire (DNAT rule sat at 0 matches; REJECT emitted nothing on capture), and the substrate never injects one — a true end-to-end breach would need a packet-crafting tool added to the sentry. Delivered instead a load-bearing pipeline self-check: extracted the parse+classify core into pure fn inspect_capture_frame (live capture loop refactored onto it) and added sortie conntrack-pipeline-selfcheck, which feeds synthetic frames and asserts SentryMediated/OffPath/Indeterminate/Outbound/Ignore all classify correctly — goes red if the detector is silently broken, which is the epistemic debt the pace targeted. Also fixed conntrack_spoofed_ack's NoResponse message, which overclaimed 'return-path enforcement intact'; it now states honestly that it reflects observed silence (a non-answering remote produces the same) and points at the self-check for liveness. Self-check mutates no sentry state, so it lives as a plain case in RBTDRC_CASES_SECURITY rather than a dedicated fixture. siege green 62/62 on macOS. Caveat: macOS only; WSL + Cygwin rocket legs not exercised here.

### 2026-06-05 10:18 - ₢BVAAG - W

Consolidated the three byte-identical /cygdrive->Windows path normalizers into one BUK buc_native_path_capture (buc_command.sh); 13 call sites repointed across rbfk/rbfd/rbob/rbndb, theurge foundry-path parity test re-homed onto the BUK function, rbte_engine.sh inline variant left by design (raw-OSTYPE/die contract). Validated cross-platform: darwin (shellcheck 200, 137 unit, fast 146/146), Cygwin (fast 145/0/1, dogfight 6/0/0, blockade 64/0/0), WSL (siege 60/0/0) all green; cerebro siege green on every touched path (19/1/0, sole failure the unrelated writ-color contamination). Matrix in memo-20260605-bvaag-crossplatform-validation.md. Scope narrowed in conversation from the docket's 'Windows-docker adapter' to the normalizer consolidation; rbgo headless-login bend + rbtid/project mountpoint left to BVAAH, no adapter module built. Cerebro writ-color Interface Contamination carved to BVAAK; cerebro non-interactive cargo PATH gap fixed + memo'd.

### 2026-06-05 10:17 - ₢BVAAG - n

Record ₢BVAAG cross-platform validation of the buc_native_path_capture consolidation (cb798526): green on darwin (shellcheck 200, 137 unit, fast 146/146), Cygwin (fast 145/0/1, dogfight 6/0/0, blockade 64/0/0), WSL (siege 60/0/0), and cerebro (siege 19/1/0 — sole failure the unrelated writ-color Interface Contamination, slated ₢BVAAK). Also lands the cerebro non-interactive-ssh cargo-PATH provisioning memo referenced by the ₣A- paddock standing note.

### 2026-06-05 10:14 - Heat - S

writ-capture-color-decoupling

### 2026-06-05 09:57 - ₢BVAAI - n

Negative control for the conntrack provenance pipeline, reframed after empirical study. Established that on Linux no real gateway-sourced reply can be staged: the spoofed ACK is conntrack-INVALID so NAT/REJECT firewall levers never fire (DNAT rule sat at 0 matches; REJECT emitted nothing), and the substrate never injects one — a true end-to-end breach would require a packet-crafting tool in the sentry. Instead: (1) extracted the load-bearing parse+classify core into pure fn inspect_capture_frame (shared by the live capture loop, refactored to use it) and added sortie_conntrack_pipeline_selfcheck, which feeds it synthetic frames and asserts SentryMediated/OffPath/Indeterminate/Outbound/Ignore classify correctly — a control that goes red if the detector is silently broken. (2) Fixed conntrack_spoofed_ack's NoResponse message which overclaimed 'return-path enforcement intact'; it now states honestly that it reflects observed silence (a non-answering remote also produces it) and points at the self-check for capture/classify liveness. Self-check mutates no sentry state so it lives as a plain case in RBTDRC_CASES_SECURITY, not a dedicated fixture. Registered selector conntrack-pipeline-selfcheck across the 5 ifrit sites + theurge wrapper/case.

### 2026-06-05 08:56 - ₢BVAAJ - n

Add offpath-blocked-dest sortie: negative control for the rbsq_wdd_offpath_reply quirk's keystone premise. Fires the conntrack_spoofed_ack provenance-classified lone ACK at a blocked destination (8.8.8.8); any reply of any provenance (off-path or sentry-mediated) is a BREACH, since the substrate carried traffic for a destination the sentry never approved. Only silence is SECURE. Reuses send_lone_ack_classify_provenance; registers selector offpath-blocked-dest across the 5 attack sites plus the theurge fixture wrapper and RBTDRC_CASES_SECURITY case.

### 2026-06-05 08:27 - ₢BVAAG - n

Consolidate the three byte-identical /cygdrive->Windows path normalizers into one BUK home. New buc_native_path_capture in buc_command.sh replaces the duplicated zrbfc_/zrbndb_/zrbob_native_path_capture copies (rbfcb_BuildHost.sh, rbndb_base.sh, rbob_bottle.sh); all 13 call sites in rbfk/rbfd/rbob/rbndb repointed. Theurge foundry-path parity test (rbtdrf_fast.rs) and its manifest comment now source buc_command.sh and drive buc_native_path_capture; the formerly-dead RBTDRF_RBFCB_BUILDHOST const repurposed to RBTDRF_BUC_COMMAND. The rbte_engine.sh inline variant left untouched (different raw-OSTYPE/die contract). Verified non-Windows: theurge build, shellcheck 200 clean, 137 unit tests green.

### 2026-06-04 23:31 - ₢BVAAF - W

Adjudicated the conntrack_spoofed_ack cross-platform divergence as a Windows Docker Desktop substrate quirk, not a sentry egress gap — proven by packet capture (off-path RST from a non-sentry MAC, bypassing the sentry; identical kernel/conntrack/iptables on both backends). Provenance-hardened the sortie (AF_PACKET L2 capture): a reply is a breach only if its Ethernet source is the sentry gateway MAC; off-path replies read SECURE-as-quirk, unresolvable provenance fails conservatively. Surfaced verdict detail on PASS so the deviation is captured in traces. Minted the rbsq_ (RB Security Quirk) category with rbsq_quirk concept anchor + rbsq_wdd_offpath_reply instance in RBS0/RBSIP. Validated 60/60 on both WSL (native docker, no-reply path) and Cygwin (DD, off-path quirk path), provenance logic provably exercised on each. Deferred follow-up: negative-control fixture to prove the gate still fails on a real breach (slated consideration handed off).

### 2026-06-04 23:08 - ₢BVAAF - n

Surface ifrit verdict detail on PASS: print result.detail after IFRIT_VERDICT: PASS so the off-path/quirk classification (and every SECURE rationale) is captured in the trace/debrief instead of discarded. theurge's parser already tolerates trailing detail on PASS (starts_with check). Completes the Windows-DD deviation capture — the rbsq_wdd_offpath_reply quirk is now observable in artifacts, not just classified.

### 2026-06-04 22:58 - ₢BVAAF - n

Mint rbsq_ (RB Security Quirk) concept category: rbsq_quirk concept anchor (a tracked, non-threatening, characterized substrate deviation — distinct from a breach and from an owned defect) plus first instance rbsq_wdd_offpath_reply documenting the Windows Docker Desktop (WSL2 backend) off-path RST injection that bypasses the sentry. Mapping refs in RBS0; definitions + Substrate Quirks subsection in RBSIP, naming the conntrack_spoofed_ack L2-provenance check as the containment membrane and a retirement condition.

### 2026-06-04 22:51 - ₢BVAAF - n

Provenance-harden conntrack_spoofed_ack: classify any reply by Ethernet source (AF_PACKET L2 capture) — a breach only if it arrived via the sentry gateway MAC; an off-path reply (Windows Docker Desktop substrate injection) reads SECURE with the foreign MAC recorded; unresolvable provenance fails conservatively. Restores the ifrit breach-maps-to-sentry-rule invariant across backends.

### 2026-06-04 22:29 - ₢BVAAF - n

Consolidate the two conntrack memos into one authoritative record, conclusion-first: lead with the durable platform-divergence finding (Docker Desktop delivers off-path to the bottle, violating the sole-boundary invariant), explicitly close the disproven nf_conntrack_tcp_loose hypothesis, fold the 2026-06-03 discovery in as dated history, and add scope/repair-direction. Reduce the original discovery memo to a redirect stub so the stale hypothesis cannot mislead future readers.

### 2026-06-04 22:07 - ₢BVAAF - n

Adjudicate conntrack_spoofed_ack cross-platform divergence: detailed memo proving via packet-level capture (off-path RST from non-sentry MAC) that the Cygwin Docker Desktop breach is a userspace-network-emulation artifact bypassing the sentry, not an egress gap; tcp_loose hypothesis disproven (=1 on the secure platform too). Add RESOLVED pointer on the prior discovery memo.

### 2026-06-04 23:26 - Heat - S

offpath-blocked-dest-probe

### 2026-06-04 23:25 - Heat - S

conntrack-provenance-negative-control

### 2026-06-04 23:08 - ₢BVAAF - n

Surface ifrit verdict detail on PASS: print result.detail after IFRIT_VERDICT: PASS so the off-path/quirk classification (and every SECURE rationale) is captured in the trace/debrief instead of discarded. theurge's parser already tolerates trailing detail on PASS (starts_with check). Completes the Windows-DD deviation capture — the rbsq_wdd_offpath_reply quirk is now observable in artifacts, not just classified.

### 2026-06-04 22:58 - ₢BVAAF - n

Mint rbsq_ (RB Security Quirk) concept category: rbsq_quirk concept anchor (a tracked, non-threatening, characterized substrate deviation — distinct from a breach and from an owned defect) plus first instance rbsq_wdd_offpath_reply documenting the Windows Docker Desktop (WSL2 backend) off-path RST injection that bypasses the sentry. Mapping refs in RBS0; definitions + Substrate Quirks subsection in RBSIP, naming the conntrack_spoofed_ack L2-provenance check as the containment membrane and a retirement condition.

### 2026-06-04 22:51 - ₢BVAAF - n

Provenance-harden conntrack_spoofed_ack: classify any reply by Ethernet source (AF_PACKET L2 capture) — a breach only if it arrived via the sentry gateway MAC; an off-path reply (Windows Docker Desktop substrate injection) reads SECURE with the foreign MAC recorded; unresolvable provenance fails conservatively. Restores the ifrit breach-maps-to-sentry-rule invariant across backends.

### 2026-06-04 22:29 - ₢BVAAF - n

Consolidate the two conntrack memos into one authoritative record, conclusion-first: lead with the durable platform-divergence finding (Docker Desktop delivers off-path to the bottle, violating the sole-boundary invariant), explicitly close the disproven nf_conntrack_tcp_loose hypothesis, fold the 2026-06-03 discovery in as dated history, and add scope/repair-direction. Reduce the original discovery memo to a redirect stub so the stale hypothesis cannot mislead future readers.

### 2026-06-04 22:07 - ₢BVAAF - n

Adjudicate conntrack_spoofed_ack cross-platform divergence: detailed memo proving via packet-level capture (off-path RST from non-sentry MAC) that the Cygwin Docker Desktop breach is a userspace-network-emulation artifact bypassing the sentry, not an egress gap; tcp_loose hypothesis disproven (=1 on the secure platform too). Add RESOLVED pointer on the prior discovery memo.

### 2026-06-04 20:11 - Heat - T

requal-cerebro-and-wsl

### 2026-06-04 20:11 - Heat - T

requal-cerebro-and-wsl

### 2026-06-03 10:37 - ₢BVAAE - n

Make blockade suite self-credentialing on macOS: prepend the canonical-invest fixture (re-mantle Governor + re-invest Retriever/Director) ahead of MORIAH, so the airgap conjure-summon no longer walls on a stale Retriever SA left by another shared-depot host's governor mantle. Mirrors how skirmish/dogfight lead with canonical-invest. Comment documents the cross-host see-saw and the remaining precondition (moriah hallmark must already be ordained in GAR).

### 2026-06-03 06:15 - Heat - r

moved BVAAB after BVAAF

### 2026-06-03 06:08 - ₢BVAAC - W

Foundry docker builds (conjure pouch + local kludge) succeed on uncontrolled Cygwin via zrbfc_native_path_capture (landed ce106ec0a). Final gate met this session: rbw-ts.TestSuite.dogfight.sh ran green on cygwin@rocket (4/4 — canonical-invest + build_run_lifecycle), past the docker-login wall that previously blocked the conjure GAR push.

### 2026-06-03 06:04 - ₢BVAAD - W

Brought the crucible tier up on uncontrolled Cygwin Docker Desktop: compose-CLI path normalization (zrbob_native_path_capture) + the nested bind-mountpoint fix (rbtid/project/.gitkeep). siege 45/46 and blockade 45/46 (both contain 45 sorties; both fail only the shared conntrack_spoofed_ack, carved to BVAAF). Renamed the tadmor suite -> siege, added the blockade suite. Re-credentialed cygwin (Payor + dogfight). Two memos captured (DD-WSL2 bind-mount wall; first-cygwin-ifrit conntrack finding). Remaining heat work carved into BVAAE-BVAAH.

### 2026-06-03 06:01 - Heat - S

cygwin-host-and-repo-winddown

### 2026-06-03 06:01 - Heat - S

windows-docker-adapter-consolidation

### 2026-06-03 06:01 - Heat - S

conntrack-spoofed-ack-adjudication

### 2026-06-03 06:00 - Heat - S

requal-cerebro-and-wsl

### 2026-06-03 06:00 - ₢BVAAD - n

Addendum to the conntrack memo: blockade (moriah/airgap) reproduced the identical 45/46 with the same conntrack_spoofed_ack breach, confirming the breach is cross-nameplate on Cygwin DD (moriah carries a runtime allowlist too) and not tether/airgap-asymmetric. Sharpens native-Linux/WSL reproduction as the decisive adjudication test.

### 2026-06-03 05:57 - Heat - d

paddock curried: invest remote-build discipline (git-as-filesystem) + 2026-06-03 scope/crucible-tier update

### 2026-06-03 05:41 - ₢BVAAD - n

Two memos capturing the first crucible charge + ifrit run on an uncontrolled Cygwin Docker Desktop host. (1) windows-docker-desktop-bind-mount: the two Windows-native-docker walls for crucible charge (compose CLI path args -> zrbob_native_path_capture; nested bind-mountpoint EACCES -> tracked rbtid/project/.gitkeep), the overlay-vs-nested-in-Windows-bind mechanism, why sentry/pentacle were unaffected, and cross-project implications (APCK, the locked crucibles-on-WSL decision now revisitable, 4x normalizer duplication). (2) cygwin-ifrit-first-run-conntrack: siege 45/46 on uncontrolled Cygwin, and the one breach (conntrack_spoofed_ack) with the crucial nuance that the target 192.0.46.9 is inside the allowlisted 192.0.32.0/20 CIDR -- so the conntrack attribution is incomplete -- plus the open platform-divergence-vs-latent-gap question and the adjudication steps (read the sortie script, reproduce on cerebro/WSL, compare nf_conntrack_tcp_loose). Both flagged for the operator to process later.

### 2026-06-03 05:35 - ₢BVAAD - n

Rename the tadmor test suite to 'siege' and add a 'blockade' suite, disambiguating the suite layer from the tadmor/moriah nameplate+fixture names (which stay welded). siege = [kludge-tadmor, tadmor] (tether bottle); blockade = [moriah] (airgap bottle; conjure hallmarks summoned from GAR, so no kludge predecessor). Renamed tt/rbw-ts.TestSuite.tadmor.sh -> .siege.sh and added .blockade.sh (generic z-launcher tabtargets; the imprint in the filename carries the suite name). Suite-name rename verified collision-free: every 'tadmor' literal in the theurge source is the fixture/nameplate name, not the suite. Also formalizes the Docker-Desktop-WSL2 bottle bind-mount fix: tracked mountpoint Tools/rbk/rbtid/project/.gitkeep so runc binds onto an existing dir instead of mkdir-ing the nested /workspace/project mountpoint inside the Windows-hosted rbtid bind (DD's WSL2 gateway denies that mkdir with EACCES). Cross-platform no-op (the ro repo bind overlays the empty dir on Linux/macOS); covers both siege and blockade since both mount the same rbtid pair. Theurge builds green on macOS.

### 2026-06-03 03:46 - ₢BVAAD - n

Fix the fourth headless-Cygwin Windows-docker wall: the docker compose CLI path args. Under Cygwin the runtime is Windows-native docker, so the compose CLI opens -f / --env-file / --project-directory as Windows paths and misreads a /cygdrive/X/... absolute ("does not exist") exactly as docker build and docker cp did. Added zrbob_native_path_capture to rbob_bottle.sh -- a third DRAFT copy of zrbfc_native_path_capture (rbfc_FoundryCore.sh), byte-identical body, kindle-independent, gated on BURD_OSTYPE, identity off Cygwin -- and routed every compose path arg (project-directory, the five env-files, the base compose, and the optional nameplate fragment) through it via BCG two-line guarded captures before building z_args. The fragment existence check stays on the POSIX path; only the form compose receives is normalized. Pinning --project-directory to the native drive-letter form additionally makes compose resolve the fragment's relative volume mounts to Windows-native paths, which is what Docker Desktop's WSL2 backend needs to bind-mount them. The scattered per-module normalizer copies (rbfc, rbndb, now rbob) plus the rbgo login bend are slated to consolidate into one Windows-docker adapter (heat BV step 3). Pending the on-host confirmation charge of tadmor/moriah over cygwin@rocket; committing first to sync via git-as-filesystem.

### 2026-06-03 03:20 - Heat - T

windows-docker-finish-and-requal

### 2026-06-03 03:00 - Heat - n

Spook fix (BCG bash-3.2 compliance), surfaced during the macOS dogfight requalification of this heat's changes. rbfl_abjure used mapfile (a bash 4+ builtin) at line 438 purely to count packages for a confirm message; macOS ships bash 3.2 at /bin/bash, so abjure died with 'mapfile: command not found' (exit 127), failing rbtdrd_build_run_lifecycle two steps past anything this heat touched. BCG explicitly forbids bash 4+ features for older-system compatibility, macOS deployments included. Replaced the mapfile-into-array-for-count with the same bash-3.2-safe while-read counting loop already used twice immediately below it for the confirm-message and delete iterations; the array it populated was otherwise unused. Only mapfile site in the rbk kit. Unrelated to the Cygwin docker work — caught incidentally because the requal exercised the full ordain->summon->run->abjure lifecycle on macOS.

### 2026-06-03 02:15 - ₢BVAAD - n

Fix the third headless-Cygwin Windows-docker wall: docker cp's host-side destination path. A Windows-native docker reads a Cygwin /cygdrive/c/... dest as drive-relative and prepends the current drive (C:\cygdrive\c\... 'invalid output path: directory does not exist'), so rbrd_check's tripwire extraction (docker cp cid:/rbrd.env <dest>) failed during the graft/conjure about+vouch phase. Added zrbndb_native_path_capture (a DRAFT duplicate of zrbfc_native_path_capture in rbfc_FoundryCore.sh; the scattered per-module copies are slated to consolidate into one Windows-docker path adapter) and routed only the cp destination argument through it; z_inscribed_file itself stays POSIX for the subsequent openssl digest read. Confirmed end-to-end: the real rbw-fO graft path now runs fully green on cygwin@rocket (docker login fileStore bend + this docker cp fix), including the about+vouch Cloud Build (SUCCESS, 1m1s). Sizing complete: the dogfight's only host-side docker walls are docker build paths (already landed), docker login cred store, and this docker cp dest; summon/bare-run/abjure carry no host paths and the fixture charges no crucible. Draft accumulating toward a full dogfight green before driver consolidation and Linux requalification.

### 2026-06-03 01:38 - ₢BVAAD - n

Correct the headless-Cygwin docker-login bend after direct on-host experimentation disproved the first approach. The committed fix (write {credsStore:''} and retry login) does NOT work: docker 28.x on Windows ignores an empty credsStore and still detects wincred, so login fails identically. The actual mechanism is a store-vs-retrieve asymmetry — docker login always routes the credential STORE through the (headless-broken) wincred helper, but docker push RETRIEVES by reading the config.json auths map directly when no credsStore key is present. Proven on cygwin@rocket: hand-writing {auths:{host:{auth:base64(oauth2accesstoken:token)}}} with credsStore absent makes docker push succeed (rc=0); login with credsStore:'' still fails. Revised bend: on the exact wincred signature (auth already succeeded), write the authenticated credential straight into the base64 file store ourselves and return success — the on-disk form WSL uses natively — so the caller's docker push reads it without touching the Windows vault. credsStore is intentionally omitted so retrieval reads the file. Corroborated by docker/cli#4353 and #1263 (wincred needs the Windows Credential Manager, unreachable from a headless SSH service session; documented workaround is credentials直 in config.json).

### 2026-06-03 01:24 - ₢BVAAD - n

Add a second Pale bend to rbgo_docker_login for headless Cygwin: Docker Desktop's wincred credential helper cannot persist the credential auth just succeeded for, from an sshd session with no interactive Windows logon. It dies with 'Error saving credentials ... A specified logon session does not exist' AFTER auth succeeds, which buc_die'd the conjure pouch push on cygwin@rocket (confirmed against yesterday's captured rbgo_docker_login_1_stderr.txt). On that exact surveyed signature (new RBGC_DOCKER_WINCRED_HEADLESS_SIGNATURE) the function forces docker's base64 file store by writing ${HOME}/.docker/config.json={credsStore:''} and retries login once, guarded against looping. The caller's docker push shares this shell and the default config location, so the bent-in credential carries through without a DOCKER_CONFIG path the Windows-native docker would need translated. Reactive-on-signature mirrors the existing moby#44350 bend at this site: real 'unauthorized' falls through to fail-fast, off-Cygwin behavior is byte-identical, and the bend self-retires once credsStore is empty.

### 2026-06-02 14:26 - Heat - S

cygwin-docker-login-headless

### 2026-06-02 21:04 - Heat - n

README: clear two clarity items. (1) Drop the hard-coded tabtarget count from the Reference Project file tree — it had drifted (claimed 136, actual ~199) and re-pinning only re-arms the decay; the line's value is the tab-completion discoverability (tt/rbw-<TAB>), not a census. (2) Repair the Reference Nameplates prose, which described only ccyolo/tadmor/moriah while the file tree and test suites also ship srjcl and pluml. Add matching prose entries for both — srjcl (Conjure-mode Jupyter, academic-domain allowlist) and pluml (Bind-mode PlantUML, no-egress) — which also showcases two non-adversarial real workloads across two vessel modes and two network postures, broadening the section beyond the three security entries.

### 2026-06-02 21:01 - Heat - n

README: correct the keyfile-tier revocation description, which was factually wrong. Per RBSGS Recovery section, the on-disk service-account key is permanent (no auto-expiry; only the minted OAuth token is short-lived), and revocation is a manual operation — Divest deletes a Director/Retriever SA, Mantle replaces the Governor SA — not 'rotation.' Replace the Foundry-Lifecycle clause 'revoked by rotation rather than centrally' with the accurate statement that the key is long-lived, does not expire on its own, and stays valid until manually revoked by deleting the account (no central or automatic revocation). Also fix the twin error in the Operator Federation roadmap bullet ('per-keyfile rotation' -> 'a manual, per-key Divest', linked).

### 2026-06-02 20:57 - Heat - n

README: make the no-organization consequence explicit and honest. Expand the keyfile-model line in Foundry Lifecycle to state the present-tier consequence directly — no-org is ideal for evaluating RB and for small-team-scale use, but its tradeoff is a long-lived service-account key at rest on disk (revoked by rotation, not centrally), a posture unlikely to clear a corporate security bar long-term; that 'assess-now, corporate-later' line is what Operator Federation lifts. Reframe the Operator Federation roadmap bullet to lead with 'the path to corporate-acceptable identity' rather than a neutral alt-auth mechanism, and align its wording to 'long-lived service-account keys on disk / no secret at rest' for consistency with the lifecycle line.

### 2026-06-02 14:01 - ₢BVAAC - n

Route foundry docker path args through a Cygwin /cygdrive normalizer so both builds succeed on an uncontrolled Cygwin host. Added zrbfc_native_path_capture to rbfc_FoundryCore.sh: pure /cygdrive/X/... -> X:/... parameter expansion (no cygpath), gated on BURD_OSTYPE, relative/native passthrough unchanged, bare-absolute-POSIX returns 1 (unsurveyed), identity off-Cygwin. Kindle-independent and sentinel-free (mirrors zrbfc_write_script_body) so it unit-tests without the foundry kindle chain. Routed every docker path arg uniformly via the BCG two-line guarded capture into locals before the docker line, at both sites: the pouch in rbfd_FoundryDirectorBuild.sh (the proven absolute -f failure plus build context) and the local kludge in rbfk_kludge.sh (conjure Dockerfile plus build context). Added a foundry-path fast fixture (5 cases: cygdrive transform, relative passthrough, native passthrough, off-Cygwin identity, bare-absolute unsurveyed) in rbtdrf_fast.rs driving the normalizer directly via a sentinel-free sourced-bash harness; declared RBTDRM_FIXTURE_FOUNDRY_PATH in rbtdrm_manifest.rs (pure no-colophon fixture) and registered the fixture in the fast/service/crucible/complete suites. Theurge build green on macOS.

### 2026-06-02 13:36 - Heat - S

foundry-docker-cygwin-paths

### 2026-06-02 20:00 - Heat - n

README: retire the undefined 'release-1' milestone label for a version-agnostic 'release-qualified' across all three platform-scope uses — the '-1' was defined nowhere in repo (RELEASE.md never uses it) and would go stale once release-1 ships, and the label had already caused one accuracy slip (Windows over-claimed as qualified in 7124cec70, corrected in eef0cc49f). Add an explicit ReleaseProcedure anchor to the Release Procedure heading and link the first platform-scope use to it so 'qualified' gains its meaning on first contact (225 lines before the ceremony is otherwise explained); fix the stale #Release-Procedure anchor casing in the Reference Project file-tree table to point at the new anchor.

### 2026-06-02 12:53 - Heat - n

Nativize theurge's env-sourced config-file paths so the Windows-native binary (x86_64-pc-windows-gnu under Cygwin) can open them. theurge read BURD_STATION_FILE — exported by bul_launcher.sh in Cygwin /cygdrive POSIX form — straight into std::fs::read_to_string with no path translation, so on Cygwin the read silently failed and canonical-invest's governor_mantle precondition reported 'BURS_TINCTURE not in burs.env' on a file that contained the key. This blocked the entire canonical/pristine/crucible/skirmish fixture family; only the fast tier ran green because its validation fixtures read files via bash subprocesses, not native Rust. Added rbtdrx_path_from_env() as theurge's single env-path intake membrane (env::var + existing rbtdrx_posix_to_native; identity off-Cygwin). Routed the two station-file readers (rbtdrk_burs_tincture, rbtdrp_burs_tincture), main.rs's dispatch-dir reader (deleting its now-redundant rbtdb_read_dispatch_dir copy), and the rbtdth test scratch-root helper through it. root stays native (current_dir() is native on Cygwin; left untouched, documented). Invariant locked: native is theurge's internal form, POSIX produced only at the bash door via existing rbtdrx_native_to_posix. Local theurge build + 137 unit tests green on macOS (identity path).

### 2026-06-02 11:04 - ₢BVAAB - n

Clear the last fast-tier command-discipline violation: replace the cat in rbgo_OAuth.sh's token-mint failure path with the BCG builtin $(<file) redirect via printf (format recycles '%s' across both files). The enclosing { ... } 2>/dev/null || true block already absorbs missing-file errors, so behavior is preserved. cat was pre-existing, not introduced by this heat, and lives inside the TEMP-FORENSIC BBABL block owned by heat BB — left that block intact (its removal is BB's call); only the cat line changed. This is what rbtdru_kit_bash flagged, the last thing between BV and a fully green fast suite.

### 2026-06-02 18:43 - Heat - n

README frontmatter: sharpen paragraph 2 to emphasize the deliberately tiny dependency surface — scope the claim to running RB ('running it needs only…') so it stays honest against the in-repo Rust, and fold in the portability ('nothing language-specific to install') and shrunken-exposure ('almost nothing of its own to audit, patch, or trust') framing. Add an evaluator-facing note at the end of Supported Platforms: the regression and adversarial test suites (the Theurge orchestrator) are written in Rust, needed only to validate the Crucible's containment yourself or to contribute — running Recipe Bottle does not require it.

### 2026-06-02 18:26 - Heat - n

README frontmatter de-sterilization: replace the taxonomy-first opening with a cost-of-control lead grounded in the lean bash/curl/openssl stack, and state the two capabilities once as the two edges of container control (origin via Foundry, reach via Crucible) with discrete in-sentence links; dissolve the redundant Environment re-intro (its 'compose, neither requires the other' line graduates into the opener); add 'Appendix: How This Is Normally Done' surveying the conventional platform-team stack under links (SLSA/Sigstore/SBOM/admission-control for builds; secure web gateway/SASE/NetworkPolicy/Cilium for egress) with an honest-scope note that RB reaches small teams, not fleet scale.

### 2026-06-02 08:18 - ₢BVAAB - n

Move native-bash-path derivation out of dispatch and into theurge, to keep cygpath out of kit bash (the BCG command-discipline scanner flagged cygpath in bud_dispatch as an undeclared dependency). Dropped BURD_BASH_BIN entirely — its regime enrollment, dispatch synthesis block, export, and z_known entry. theurge's rbtdri_bash_program now nativizes the canonical /bin/bash via RBTDRX's existing cygpath (cached in a OnceLock), gated on rbtdrx_is_cygwin; cygpath resolves fine from a native binary since it has no System32 twin, unlike bash. BURD_OSTYPE (platform) stays as the sole synthesized regime member. Local build green.

### 2026-06-02 08:09 - ₢BVAAB - n

Deliver Cygwin platform facts to theurge as synthesized BURD regime members instead of leaking ambient bash state. Corrects the disproven thesis that bash exports OSTYPE to native children: it does not, so RBTDRX was inert and a bare `bash` reached System32's WSL launcher, not Cygwin's. New BURD_OSTYPE (live $OSTYPE) and BURD_BASH_BIN (cygpath -w $BASH on Cygwin, $BASH elsewhere) are enrolled in burd_regime, computed+exported in bud_dispatch's zbud_setup, and added to the z_known allowlist. theurge's rbtdrx_is_cygwin now reads BURD_OSTYPE; the bash-launch sites (rbtdri_tabtarget_command, rbtdrf_run_bash) resolve the interpreter via new rbtdri_bash_program() reading BURD_BASH_BIN. Local build green through dispatch on macOS (else-branch, identity).

### 2026-06-02 14:58 - Heat - n

README sync with RBSHR roadmap: flip Windows host status to supported-experimental (not yet release-1-qualified) in both platform-scope spots and drop the Windows roadmap bullet; add anchored Operator Federation roadmap entry leading with the GCP-organization / DNS-domain prerequisite, plus a linked no-org keyfile sentence in the credential narrative; rename 'Credential confinement' -> 'Bottle Credential Custody' with Bottle-resident/charge-time framing cross-linked to the conduit; anchor all eight surviving roadmap bullets with self-links

### 2026-06-02 14:47 - Heat - n

RBSHR roadmap: drop the 'Windows host workstation support — shell transport hardening' horizon entry; Windows host support is now effectively in hand, no longer a deferred item (the wsl-substrate-supply story survives in the revised-enshrine entry)

### 2026-06-02 14:45 - Heat - n

RBSHR roadmap: rename 'Credential confinement' -> 'Bottle credential custody' (secret-at-rest moves off workstation into the bottle, charge-time injection not image-bake, blocks workstation credential theft, distinguished from the sentry-held tunnel key of the conduit item); health-fix Operator federation entry to match the committed two-tier memo (shipping = keyfile/RBRA no-org, federation = org-required successor; drop stale 'near-term user: grants' framing) and flag the domain-name prerequisite (free org via Cloud Identity domain verification)

### 2026-06-02 07:02 - ₢BVAAB - n

Clear a second Cygwin path boundary at the build step: Windows-native cargo cannot open the Cygwin /cygdrive manifest-path arg (reads it literally, reports does-not-exist) — same path-argument family as the deferred shellcheck site, distinct from the .sh-launch boundary. zrbte_kindle now derives RBTE_MANIFEST_ARG: under Cygwin, translate /cygdrive/X/... to X:/... via pure parameter expansion (no subshell, no external cygpath — BCG-compliant, mirrors RBTDRX's /cygdrive fast path), fail fast on any unsurveyed shape; identity off Cygwin. The three cargo build/test invocations now pass RBTE_MANIFEST_ARG.

### 2026-06-02 06:52 - ₢BVAAB - n

Route every theurge .sh-launch site through bash to clear the Cygwin process-launch boundary. New helper rbtdri_tabtarget_command(&Path) builds `bash <script-as-posix>` (RBTDRX renders the path string only); a Windows-native theurge cannot CreateProcess a .sh directly. Threaded through all five sites in rbtdri/rbtdrf: rbtdri_invoke_impl, rbtdrf_run_tt, rbtdrf_run_tt_neg, the unmake empty-arg refusal case, and rbtdrf_hb_render. Behavior-neutral on Linux/macOS where native_to_posix is identity and bash script.sh equals shebang exec; local theurge build green.

### 2026-06-02 06:43 - Heat - d

paddock curried: retain shellcheck path-arg boundary carry-forward from BVAAA

### 2026-06-02 06:41 - ₢BVAAA - W

Route-independent Cygwin deps verified/installed on cygwin@rocket. tmux (3.2) and curl already present; curl correctly shadows system32 (command -v curl -> /usr/bin/curl, Cygwin 8.20.0). shellcheck has NO Cygwin package (Haskell/GHC incompatible with Cygwin paths), so installed the official Windows shellcheck.exe v0.11.0 (sha256 8a4e35ab... verified against GitHub release) into /usr/local/bin via the admin channel (tt/buw-jpS bujn-winpc as bhyslop) — the unprivileged cygwin account cannot write there. FINDING (heat thesis confirmed): the Windows-native shellcheck.exe resolves and runs (command -v ok, --version ok) but cannot open Cygwin /-style path args (openBinaryFile: does not exist); it works perfectly with Windows-form paths (cygpath -w) or stdin redirect. This is an RBTDRX-family path-translation boundary, but a DISTINCT site from the Route-A .sh-launch boundary: it is a Windows tool receiving Cygwin path arguments, surfacing when the harness drives shellcheck in rbw-tr QualifyRelease (not the fast/validation tier that is this heat's done-condition).

### 2026-06-02 06:35 - Heat - f

silks=rbk-10-mvp-uncontrolled-cygwin-test

### 2026-06-01 07:33 - Heat - S

theurge-fast-suite-cygwin-bringup

### 2026-06-01 07:33 - Heat - S

install-known-cygwin-deps

### 2026-06-01 07:29 - Heat - d

paddock curried: initial paddock — uncontrolled-cygwin thesis

### 2026-06-01 07:28 - Heat - f

racing

### 2026-06-01 07:28 - Heat - N

rbk-mvp-uncontrolled-cygwin-test

