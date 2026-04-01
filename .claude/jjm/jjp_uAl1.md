## Character
Architectural migration with careful sequencing. Early paces are infrastructure and minting; middle paces are mechanical porting; late paces require judgment about bash test equivalence. The "two binaries" design is settled but internal structure will emerge during implementation.

## Design Decision Record

**Problem**: Bash tests (BUK framework, ~75 cases) and Python sorties (ifrit adjutant, 11 attack modules) are two separate worlds that cannot coordinate. Neither can simultaneously observe from outside the bottle while attacking from inside. The testing language split adds maintenance burden without adding capability.

**Decision**: Replace both with Rust infrastructure:
- **Theurge** — orchestrator binary (`Tools/rbk/rbtd/`), runs on host macOS, owns charge/exec/monitor/report lifecycle
- **Ifrit** — attack binary (`Tools/rbk/rbid/`), compiled for Linux, copied into bottle at test time, invoked with attack selector via bark tabtarget
- Separate crates — completely different dependency profiles and build targets
- Prefix tree: `rbtr` (theurge rust), `rbid` (ifrit directory, after `rbi` prefix reclaimed)

**Key design choices**:
- Detached from `cargo test` model — theurge is an application that produces test verdicts, not a unit test suite. All tiers (fast, service, crucible) are selector arguments to the one theurge binary.
- Colophon manifest protocol — theurge always invokes bottle operations via tabtargets (never reimplements). Compiled-in colophon strings verified against live zipper registry at launch.
- New container exec tabtargets: writ (sentry), fiat (pentacle), bark (bottle) — non-interactive exec-and-return, distinct from interactive hail/rack/scry.
- Ifrit discovers its network environment from container state (env vars, resolv.conf, probing), no config arguments beyond attack selector
- Reporting follows bash test precedent: colored pass/fail to terminal, verbose trace written to per-case temp dir files (never rerun for detail). No JSON.
- Ifrit vessel simplified but kept — no python, no nodejs, no Claude Code session. Binary copied in at test time.
- BUK test framework files (`butX`) retained with a minimal BUK self-test; only RBK consumers retired.

**Open questions (require discussion before relevant paces):**
- Cross-OS build for ifrit: host is macOS/aarch64, ifrit must be Linux/aarch64 ELF. This is cross-compilation even though same architecture. Options: `cross-rs`, `brew install musl-cross`, build in lightweight container. Need to resolve before ₢A1AAD.
- Colophon assignments for writ/fiat/bark — need to mint before ₢A1AAC.

**Explicitly deferred**: new test design, tcpdump/continuous monitoring, cross-nameplate ifrit deployment, tabtarget reorganization.

## Provenance
Emerged from conversation about the fundamental flaw in split bash/python test architecture. The name "theurge" chosen for `rbt` prefix compatibility and the mythology (one who compels spirits through ceremony — commanding the ifrit).