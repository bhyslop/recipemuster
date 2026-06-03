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