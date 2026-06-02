## Shape

Prove that Recipe Bottle's test harness (theurge) can run on an **uncontrolled**
Cygwin — a stock Windows+Cygwin box we do not provision. This is a consumer-
credibility claim: a consumer can qualify their own install on their own machine
and trust the verdict. It is deliberately distinct from the `rbk-NN-win-*` heats,
which are all controlled-infrastructure work (formal BURN garrison, remote
control, depot regime) on machines we own.

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