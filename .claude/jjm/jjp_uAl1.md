## Character
Architectural migration with careful sequencing. Early paces are infrastructure and minting; middle paces are mechanical porting; late paces require judgment about bash test equivalence. The "two binaries" design is settled but internal structure will emerge during implementation.

## Design Decision Record

**Problem**: Bash tests (BUK framework, ~75 cases) and Python sorties (ifrit adjutant, 11 attack modules) are two separate worlds that cannot coordinate. Neither can simultaneously observe from outside the bottle while attacking from inside. The testing language split adds maintenance burden without adding capability.

**Decision**: Replace both with Rust infrastructure:
- **Theurge** — orchestrator binary (`Tools/rbk/rbtd/`), runs on host, owns charge/exec/monitor/report lifecycle. Code prefix: `rbtd`.
- **Ifrit** — attack binary, source at `Tools/rbk/rbid/`, built inside the ifrit vessel Dockerfile (no cross-compilation). The vessel image IS both the build artifact and the runtime bottle. Code prefix: `rbid`.
- Separate crates — completely different dependency profiles and build targets.
- **Not `rbtr`** — that AsciiDoc attribute category ("RB Term Role") is already populated in RBS0. Theurge uses `rbtd`, ifrit uses `rbid`.

**Key design choices**:
- Detached from `cargo test` model — theurge is an application that produces test verdicts, not a unit test suite. All tiers (fast, service, crucible) are selector arguments to the one theurge binary, selected via tabtarget imprints.
- Colophon manifest protocol — theurge always invokes bottle operations via tabtargets (never reimplements). Compiled-in colophon strings verified against live zipper registry at launch. The colophon set grows as paces add dependencies (charge/quench initially, then writ/fiat/bark, then foundry colophons for four-mode).
- New container exec tabtargets: writ (sentry), fiat (pentacle), bark (bottle) — non-interactive exec-and-return, distinct from interactive hail/rack/scry.
- Ifrit discovers its network environment from container state (env vars, resolv.conf, probing), no config arguments beyond attack selector.
- Reporting follows bash test precedent: colored pass/fail to terminal, verbose trace written to per-case temp dir files (never rerun for detail). No JSON.
- BUK test framework files (`butX`) retained with a minimal BUK self-test; only RBK consumers retired.

**Ifrit build model — single Dockerfile, single image:**
- One Dockerfile at `rbev-vessels/rbev-bottle-ifrit/Dockerfile`. Single stage: `rust:bookworm-slim` base, network tools installed, `Cargo.toml`+`Cargo.lock` copied first (layer cached), then source copied and built. Rust toolchain stays in image — it's a test vessel, size doesn't matter, iteration speed does.
- The resulting image IS the runtime bottle. When the tadmor crucible charges, the ifrit binary is already inside. No extraction, no docker cp, no two-image dance.
- Custom `rbi-iK` kludge tabtarget writes to a constant hallmark for daily iteration.
- Steady state: cloud conjure builds the same Dockerfile.
- No Docker volumes, no persistent cargo cache. Layer caching handles dependency compilation; source-only changes recompile just the ifrit crate.
- Adding a new crate dependency requires a tethered kludge (same cost as adding an apt package today). `Cargo.lock` pins versions.

**RBTW workbench — orthogonal Rust pipeline:**
- Theurge build/test is fully independent of the VOW pipeline (vvk/jjk/cmk kits destined for parcel delivery). VOW will be peeled away; theurge stays with rbk permanently.
- `rbtw` minted as new sibling under `rbt` (alongside `rbtc` colophons, `rbtd` theurge, `rbtr` roles). Workbench at `Tools/rbk/rbtd/rbtw_workbench.sh`.
- Tabtargets: `rbtd-b` (build), `rbtd-t` (unit test). Future: `rbtd-r` (run orchestration suite with tier imprints).
- Theurge unit tests (`cargo test` on rbtd) verify engine internals — distinct from theurge-the-application which produces crucible verdicts. The paddock's "detached from cargo test" describes the application's purpose, not a prohibition on testing engine code.
- `rbrt` was considered but rejected: `rbr` branch is semantically "Regime" (rbra, rbrm, rbrn, rbro, rbrp, rbrr, rbrs, rbrv) — a Rust test prefix would be an interloper.
- Ifrit has no macOS unit tests — it's a Linux container binary. Its testing comes from theurge orchestrating the charged bottle.

**Explicitly deferred**: new test design, tcpdump/continuous monitoring, cross-nameplate ifrit deployment.

## Provenance
Emerged from conversation about the fundamental flaw in split bash/python test architecture. The name "theurge" chosen for `rbt` prefix compatibility and the mythology (one who compels spirits through ceremony — commanding the ifrit).