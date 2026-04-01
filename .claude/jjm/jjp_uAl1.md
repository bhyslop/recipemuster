## Character
Architectural migration with careful sequencing. Early paces are infrastructure and minting; middle paces are mechanical porting; late paces require judgment about bash test equivalence. The "two binaries" design is settled but internal structure will emerge during implementation.

## Design Decision Record

**Problem**: Bash tests (BUK framework, ~75 cases) and Python sorties (ifrit adjutant, 11 attack modules) are two separate worlds that cannot coordinate. Neither can simultaneously observe from outside the bottle while attacking from inside. The testing language split adds maintenance burden without adding capability.

**Decision**: Replace both with Rust infrastructure:
- **Theurge** — orchestrator binary, runs outside the bottle, owns charge/exec/monitor/report lifecycle
- **Ifrit** — attack binary, compiled statically, copied into bottle at test time, invoked with attack selector via `docker exec`
- Prefix tree: `rbtr` (theurge rust) to coexist during transition, then retire bash `rbt` tree

**Key design choices**:
- Detached from `cargo test` model — theurge is an application that produces test verdicts, not a unit test suite
- Ifrit discovers its network environment from container state (env vars, resolv.conf, probing), no config arguments beyond attack selector
- Reporting follows bash test precedent: colored pass/fail to terminal, verbose trace written to TEMP_DIR files (never rerun for detail)
- Ifrit vessel becomes trivially slim — static binary, no python, no nodejs, no Claude Code session inside
- Fast-tier tests (regime/enrollment validation) migrate last

**Explicitly deferred**: new test design, tcpdump/continuous monitoring, cross-nameplate ifrit deployment, tabtarget reorganization.

## Provenance
Emerged from conversation about the fundamental flaw in split bash/python test architecture. The name "theurge" chosen for `rbt` prefix compatibility and the mythology (one who compels spirits through ceremony — commanding the ifrit).