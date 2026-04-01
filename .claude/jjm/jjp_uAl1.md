## Character
Architectural migration with careful sequencing. Early paces are infrastructure and minting; middle paces are mechanical porting; late paces require judgment about bash test equivalence. The "two binaries" design is settled but internal structure will emerge during implementation.

## Design Decision Record

**Problem**: Bash tests (BUK framework, ~75 cases) and Python sorties (ifrit adjutant, 11 attack modules) are two separate worlds that cannot coordinate. Neither can simultaneously observe from outside the bottle while attacking from inside. The testing language split adds maintenance burden without adding capability.

**Decision**: Replace both with Rust infrastructure:
- **Theurge** — orchestrator binary (`Tools/rbk/rbtd/`), runs on host macOS, owns charge/exec/monitor/report lifecycle. Code prefix: `rbtd`.
- **Ifrit** — attack binary (`Tools/rbk/rbid/`), built inside ifrit vessel image via Docker (no cross-compilation), copied into target bottle at test time, invoked with attack selector via bark tabtarget. Code prefix: `rbid`.
- Separate crates — completely different dependency profiles and build targets.
- **Not `rbtr`** — that AsciiDoc attribute category ("RB Term Role") is already populated in RBS0. Theurge uses `rbtd`, ifrit uses `rbid`.

**Key design choices**:
- Detached from `cargo test` model — theurge is an application that produces test verdicts, not a unit test suite. All tiers (fast, service, crucible) are selector arguments to the one theurge binary, selected via tabtarget imprints.
- Colophon manifest protocol — theurge always invokes bottle operations via tabtargets (never reimplements). Compiled-in colophon strings verified against live zipper registry at launch.
- New container exec tabtargets: writ (sentry), fiat (pentacle), bark (bottle) — non-interactive exec-and-return, distinct from interactive hail/rack/scry.
- Ifrit discovers its network environment from container state (env vars, resolv.conf, probing), no config arguments beyond attack selector.
- Reporting follows bash test precedent: colored pass/fail to terminal, verbose trace written to per-case temp dir files (never rerun for detail). No JSON.
- BUK test framework files (`butX`) retained with a minimal BUK self-test; only RBK consumers retired.

**Ifrit build model**:
- Single-stage Dockerfile: `rust:bookworm-slim` base, network tools installed, Cargo.toml+Cargo.lock copied first (layer cached), then source copied and built. Rust toolchain stays in image — it's a test vessel, size doesn't matter, iteration speed does.
- Custom `rbi-iB` kludge tabtarget writes to a constant consecration for daily iteration — no timestamped tags during development.
- No cross-compilation: Dockerfile builds Linux-for-Linux inside the container. Works identically on macOS, Linux, WSL.
- No Docker volumes, no persistent cargo cache. Layer caching handles dependency compilation; source-only changes recompile just the ifrit crate.
- Adding a new crate dependency requires a tethered kludge (same cost as adding an apt package today). Cargo.lock pins versions.

**Ifrit vessel**:
- Simplified but kept. No python, no nodejs, no Claude Code session. Contains compiled ifrit binary + network tools (socat, dnsutils, netcat, traceroute, strace).
- At test time, theurge charges the TARGET crucible (e.g., tadmor), then copies the ifrit binary from the ifrit image into the running bottle.

**Explicitly deferred**: new test design, tcpdump/continuous monitoring, cross-nameplate ifrit deployment, tabtarget reorganization.

## Provenance
Emerged from conversation about the fundamental flaw in split bash/python test architecture. The name "theurge" chosen for `rbt` prefix compatibility and the mythology (one who compels spirits through ceremony — commanding the ifrit).