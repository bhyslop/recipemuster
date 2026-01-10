# Heat: Vox First Whisper

Nucleate VOK (Vox Obscura Kit) as the foundational kit for Claude-powered infrastructure. Migrate JJK to depend on it.

## Paddock

### Vox Obscura - The Hidden Voice

VOK provides the arcane infrastructure that other kits depend on:
- Version tracking (Codex)
- Version identity (Sigil)
- Upstream filtering (Veil)
- Kit operations pattern (install, uninstall, check)

### VO Prefix Map

| Prefix | Name | Purpose |
|--------|------|---------|
| `voa_` | Arcanum | Main kit entry (install/uninstall) |
| `vop_` | Prepare | Release preparation (compile/test/package) |
| `vox_` | Codex | Version ledger |
| `vos_` | Sigil | Version ID |
| `vov_` | Veil | Upstream filter |

### Reserved Suffixes (Claude Code Native Types)

All kits use these consistently:

| Suffix | Type |
|--------|------|
| `*a_` | Arcanum (kit main) |
| `*b_` | suBagent |
| `*c_` | slash Command |
| `*h_` | Hook |
| `*k_` | sKill |
| `*w_` | Workbench |
| `*t_` | Testbench |

### Kit Facility Pattern

Kits may implement one or more facilities:

- **Arcanum** (`*a_`) - Claude environment installation (slash commands, subagents, hooks, skills). Hidden voice setup layer.
- **Workbench** (`*w_`) - Classic dispatch of tabtargets to bash utilities. User-facing operational tooling.
- **Testbench** (`*t_`) - Testing facility for bash utilities.

### Kit Operations Pattern

| Pattern | Operation |
|---------|-----------|
| `*k-i` | Install |
| `*k-u` | Uninstall |
| `*k-c` | Check |

### Key Files

- `Tools/vok/voa_arcanum.sh` - main kit entry (install/uninstall)
- `Tools/vok/vop_prepare_release.sh` - compile, test, package Rust binaries
- `Tools/vok/vox_codex.json` - version ledger
- `Tools/vok/rust/` - Cargo workspace for all kit Rust utilities
- `Tools/vok/release/` - prepared binaries by platform
- `Tools/vok/lenses/vop-CRCG-ClaudeRustCodingGuide.md` - Rust coding constraints
- `Tools/vok/README.md` - documentation

### CGK Guard - Pre-commit Size Validation

Rust utility that measures staged blob sizes before commit. Prevents catastrophic auto-adds (node_modules, build artifacts, binaries) by enforcing size limits.

**Why blob size, not file size:**
- Git commits blobs, not files
- Pack compression is non-deterministic (happens later)
- Blob size is the correct pre-commit proxy
- Intentionally overestimates final pack size (safe for gating)

**Measurement approach:**
```bash
git diff --cached --name-only -z \
| xargs -0 git ls-files -s \
| awk '{print $4}' \
| xargs git cat-file -s \
| awk '{sum+=$1} END {print sum}'
```

**Interface:**
```
cgr guard [--limit <bytes>] [--warn <bytes>]
  Exit 0: under limit
  Exit 1: over limit (with breakdown by file)
  Exit 2: over warn threshold (proceed with caution)
```

**Subcommand:** `vok guard` (cgk feature-gated)

### CRCG - Claude Rust Coding Guide

CRCG documents constraints for small Rust utilities that complement BCG-constrained bash. These are text-wrangling tools too sophisticated for reliable bash/jq but simple enough to stay minimal.

**Core constraints:**
- Pure filter pattern (stdin → stdout, no side effects)
- Minimal deps (serde_json, clap - that's typically it)
- No git2/libgit2 (shell out to git for temporal stability)
- Exit codes as contract (0/1/2 semantics documented per utility)
- Cargo.lock checked in (reproducible builds)
- Edition pinned (2021 for years of stability)

**Target platforms (5):**

| Platform | Rust triple | Notes |
|----------|-------------|-------|
| darwin-arm64 | `aarch64-apple-darwin` | macOS Apple Silicon |
| darwin-x86_64 | `x86_64-apple-darwin` | macOS Intel |
| linux-x86_64 | `x86_64-unknown-linux-gnu` | Linux Intel/AMD, WSL, CI |
| linux-aarch64 | `aarch64-unknown-linux-gnu` | AWS Graviton, ARM servers |
| windows-x86_64 | `x86_64-pc-windows-gnu` | Windows (gnu target for Docker cross-compile from macOS) |

### VOK Rust Workspace (Option A: All Rust in VOK)

Single Cargo workspace in `Tools/vok/rust/` hosts one multicall binary with feature-gated subcommands. Each kit's Rust logic lives here, enabled only when that kit is present.

```
Tools/vok/rust/
├── Cargo.toml      # [package] name = "vok", [features] jjk = [], cgk = []
├── Cargo.lock      # Shared dependency versions (checked in)
├── build.rs        # Auto-detects ../../jjk, ../../cgk → enables features
└── src/
    ├── main.rs     # Multicall dispatch: vok jj, vok guard, vok sigil
    ├── core/       # Shared infrastructure (platform, env discovery)
    ├── jj/         # #[cfg(feature = "jjk")] - JJK text processing
    └── guard/      # #[cfg(feature = "cgk")] - CGK pre-commit validation
```

**Why single binary:**
- Rust std library links once (~1-2MB saved per additional "binary")
- serde_json links once
- Kit-specific logic is tiny text manipulation
- Feature flags compile out unused code paths

**build.rs auto-detection:**
```rust
fn main() {
    if Path::new("../../jjk").exists() {
        println!("cargo:rustc-cfg=feature=\"jjk\"");
    }
    if Path::new("../../cgk").exists() {
        println!("cargo:rustc-cfg=feature=\"cgk\"");
    }
}
```

Project builds only include subcommands for kits actually present. No manifest to maintain.

### Prepare Release vs Arcanum

Two distinct operations, cleanly separated:

| Operation | Script | Where it runs | What it does |
|-----------|--------|---------------|--------------|
| **Prepare Release** | `vop_prepare_release.sh` | Source repo only | Compile, test, package binaries |
| **Install** | `voa_arcanum.sh` | Any repo | Copy pre-built artifacts, update CLAUDE.md |

**Flow:**
```
Source repo:
  tt/vok-pr.PrepareRelease.sh
    → cargo build --release --workspace
    → cargo test --workspace
    → copy binaries to Tools/vok/release/«platform»/

Then:
  tt/vok-i.Install.sh
    → copy from release/ to .claude/«kit»/«binary»/«platform»/
    → update CLAUDE.md
```

**Binary ownership:** Binaries live under their consuming kit:
```
.claude/
├── cgk/
│   └── cgr/
│       └── darwin-arm64/
│           └── cgr
└── jjk/
    └── jjr/
        └── darwin-arm64/
            └── jjr
```

Uninstall = `rm -rf .claude/«kit»/` removes that kit's binary.

**Distribution flexibility:**
- Full kit (source + prepare + arcanum) → maintainers
- Artifacts + arcanum only → consumers
- Binaries may be gitignored or checked in (decision deferred)

### The Compilation Model

Arcanum emitters are compiled documentation. We (Claude + human) are the compiler.

```
Source:      Full API knowledge (Rust source, specs, our session context)
Compiler:    Us writing the arcanum emitter functions
Object code: The arcanum emitters themselves (zjjw_emit_*, etc.)
Install:     Mechanical deployment - runs emitters, copies binaries
Runtime:     Target Claude executes emitted instructions
```

**Knowledge lifecycle:**
- API understanding exists during our working session (compilation time)
- We encode it into `zjjw_emit_*` functions
- Knowledge is now embedded in code, not documented separately
- Install deploys compiled artifacts to target repo
- Target Claude follows instructions without understanding the API
- Original knowledge doesn't persist - the emitters ARE the documentation

**Why this enables obscurity:**
- No API spec to publish - knowledge compiles away
- Target repo has instructions only, not understanding
- `vok --help` exists (clap-generated), but no external docs
- Bash scripts bridge slash commands to vok calls
- Only source repo readers see the full picture

**The veiled README:**
`Tools/vok/README.md` documents the compilation model and full API - for kit authors only. It is veiled (filtered from upstream/distribution). Target repos never see it.

## Done

## Remaining

- **Write veiled VOK README**
  Create `Tools/vok/README.md` (veiled - never distributed). Document: (1) The Compilation Model - we are the compiler, arcanum emitters are object code, (2) Option A architecture - all Rust in VOK, feature-gated multicall binary, (3) build.rs auto-detection mechanism, (4) Full vok subcommand API reference (for kit authors writing arcanums), (5) Why this doc is veiled - compiler docs, not user docs. This is the foundational conceptual document.

- **Clarify kit facility patterns**
  Document when kits use arcanum vs workbench vs testbench. Define rules: Which kits need which facilities? Can kits have multiple? Examples: Does JJK need `jja_arcanum.sh` AND `jjw_workbench.sh`? Does CGK need `cga_arcanum.sh` AND `cgw_workbench.sh`? Where does model differential tool belong (JJ workbench, not CGK)? Output: Section in VOK README.

- **Create VOK skeleton**
  Create `Tools/vok/` directory with `voa_arcanum.sh` stub. README already written in prior pace.

- **Implement Codex**
  Extract ledger machinery from JJK. Functions: `zvoa_compute_source_hash()`, `zvoa_lookup_sigil_by_hash()`. File: `vox_codex.json`.

- **Implement Sigil**
  Version ID computation and registration. Equivalent to JJK's "brand" concept.

- **Add kit operations**
  Implement `vok-i` (install), `vok-u` (uninstall), `vok-c` (check). Create tabtargets (`tt/vok-i.Install.sh`, `tt/vok-u.Uninstall.sh`, `tt/vok-c.Check.sh`).

- **Implement Veil**
  prep-pr upstream filtering. Config defines what's internal vs upstream-safe. Move from CMK.

- **Migrate JJK to VOK**
  Refactor JJK to call VOK's Codex instead of its own ledger machinery. JJK depends on VOK.

- **Design CGK config schema**
  Define per-repo configuration for Claude Git Kit. Key decisions: module name mappings (terse `JJK` vs expanded `Job Jockey Kit`), config file location (`.claude/cgk_config.json` or section in existing file), default behavior, inheritance/override patterns. Output: documented schema ready for implementation.

- **Design CRCG**
  Write the Claude Rust Coding Guide as a VOK lens document. Document: pure filter pattern, minimal deps policy, exit code conventions, Cargo.lock/edition pinning, the 5 target platforms, and how Rust utilities integrate with BCG bash orchestration. Reference BCG as the companion guide for bash. Output: `Tools/vok/lenses/vop-CRCG-ClaudeRustCodingGuide.md`.

- **Create VOK Rust workspace**
  Create `Tools/vok/rust/` with Option A structure: single `vok` binary with feature-gated subcommands. Cargo.toml with `[features]` for jjk, cgk, etc. build.rs with kit directory auto-detection. src/ with main.rs (clap multicall dispatch), core/ (shared platform/env), and placeholder modules for jj/ and guard/. No business logic yet - just the skeleton that compiles and proves the feature-gating works.

- **Create Prepare Release script**
  Implement `vop_prepare_release.sh` with functions: detect current platform, build all workspace members (`cargo build --release`), run all tests (`cargo test`), copy binaries to `Tools/vok/release/«platform»/`. Create tabtarget `tt/vok-pr.PrepareRelease.sh`. Exit non-zero if tests fail (gate release on tests).

- **Update arcanum for Rust artifacts**
  Extend `voa_arcanum.sh` to copy prepared binaries from `Tools/vok/release/` to `.claude/«kit»/«binary»/«platform»/`. Detect current platform and copy appropriate binary. Fail gracefully if release artifacts missing (prompt to run prepare-release first).

- **Implement vok guard subcommand**
  Add `vok guard` subcommand (gated by `cgk` feature) for pre-commit size validation. Uses git plumbing to sum staged blob sizes. Pure filter pattern - bash pipes git output to vok, vok returns verdict. No git2 crate. Exit codes: 0 (ok), 1 (over limit), 2 (warning). Tabtarget: `tt/cgk-guard.sh`. Hook integration: `.claude/hooks/pre-commit`. Simpler than JJ operations (no LLM), good first subcommand to prove the pattern.

- **Create CGK (Claude Git Kit)**
  Foundational kit for Claude-aware git operations, starting with commit message generation. Create `Tools/cgk/` directory with `cga_arcanum.sh` and tabtarget `tt/cgk-commit.sh` that invokes Claude to write commit messages. Document pattern for extensibility to other git operations (branch prep, PR formatting, etc.).

- **Document arcane vocabulary**
  Add to veiled VOK README: full prefix conventions, reserved suffixes, arcane term glossary (Arcanum, Codex, Sigil, Veil, etc.). This is reference material for kit authors.
