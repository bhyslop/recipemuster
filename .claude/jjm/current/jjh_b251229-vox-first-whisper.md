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

**Naming parallel:** `jjr` (Job Jockey Rust) : `cgr` (Claude Git Rust)

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

### VOK Rust Workspace

Single Cargo workspace hosts all kit Rust utilities. No shared library - each binary is fully self-contained.

```
Tools/vok/rust/
├── Cargo.toml      # [workspace] members = ["cgr", "jjr"]
├── Cargo.lock      # Shared dependency versions (checked in)
├── cgr/
│   ├── Cargo.toml
│   └── src/main.rs
└── jjr/
    ├── Cargo.toml
    └── src/main.rs
```

**What the workspace provides:**
- Single Cargo.lock = consistent dependency versions across utilities
- Single `cargo build --workspace` = build all at once
- Single `cargo test --workspace` = test all at once

**What it does NOT provide:**
- No shared code between utilities (copy-paste is fine for small utils)
- No library crate dependencies between utilities

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

## Done

## Remaining

- **Clarify kit facility patterns**
  Document when kits use arcanum vs workbench vs testbench. Define rules: Which kits need which facilities? Can kits have multiple? Examples: Does JJK need `jja_arcanum.sh` AND `jjw_workbench.sh`? Does CGK need `cga_arcanum.sh` AND `cgw_workbench.sh`? Where does model differential tool belong (JJ workbench, not CGK)? Output: Clear architectural decision documented in VOK README.

- **Create VOK skeleton**
  Create `Tools/vok/` directory with `voa_arcanum.sh` stub and `README.md` documenting the Vox Obscura concept and prefix conventions.

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
  Create `Tools/vok/rust/` directory structure. Root `Cargo.toml` with `[workspace]` definition. Empty member directories for future utilities (`cgr/`, `jjr/`). Document workspace conventions in README. No actual Rust code yet - just the skeleton that prepare-release will build.

- **Create Prepare Release script**
  Implement `vop_prepare_release.sh` with functions: detect current platform, build all workspace members (`cargo build --release`), run all tests (`cargo test`), copy binaries to `Tools/vok/release/«platform»/`. Create tabtarget `tt/vok-pr.PrepareRelease.sh`. Exit non-zero if tests fail (gate release on tests).

- **Update arcanum for Rust artifacts**
  Extend `voa_arcanum.sh` to copy prepared binaries from `Tools/vok/release/` to `.claude/«kit»/«binary»/«platform»/`. Detect current platform and copy appropriate binary. Fail gracefully if release artifacts missing (prompt to run prepare-release first).

- **Create CGK Guard (Rust)**
  Implement `cgr` (Claude Git Rust) binary for pre-commit size validation. Uses git plumbing to sum staged blob sizes. Pure stdin/stdout filter pattern (like jjr). No git2 crate - shells out to git for blob queries. Tabtarget: `tt/cgk-guard.sh`. Hook integration: `.claude/hooks/pre-commit`. Simpler than commit (no LLM), establishes Rust pattern for CGK.

- **Create CGK (Claude Git Kit)**
  Foundational kit for Claude-aware git operations, starting with commit message generation. Create `Tools/cgk/` directory with `cga_arcanum.sh` and tabtarget `tt/cgk-commit.sh` that invokes Claude to write commit messages. Document pattern for extensibility to other git operations (branch prep, PR formatting, etc.).

- **Document arcane vocabulary**
  Finalize README with full prefix conventions, reserved suffixes, and arcane term glossary.
