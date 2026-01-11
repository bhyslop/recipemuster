# Heat: Vox First Whisper

Nucleate VOK (Vox Obscura Kit) as the foundational kit for Claude-powered infrastructure. Migrate JJK to depend on it.

## Paddock

### Vox Obscura - The Hidden Voice

VOK provides the arcane infrastructure that other kits depend on:
- Version tracking (Codex)
- Version identity (Sigil)
- Upstream filtering (Veil)
- Kit operations pattern (install, uninstall, check)

### Voce Viva - The Living Voice

**Voce Viva** (the living voice) is the poetic name for the user-facing tooling that VOK produces. Users invoke `vvx`, the platform-detecting wrapper, which dispatches to the appropriate `vvr` binary.

```
VOK (Vox Obscura Kit)     - The veiled source, hidden infrastructure
       ↓ (release process - whispering into existence)
VVK (Voce Viva Kit)       - The installable artifact (bash + binaries)
  └── vvx                 - What users invoke (wrapper → vvr binary)
```

**The relationship:**
- **VOK** is the *source* - veiled, arcane, never seen by users
- **VVK** is the *artifact* - the kit that travels to target repos
- **vvx** is the *entry point* - what users and scripts invoke
- **vvr** is the *binary* - Rust code, platform-specific variants

Users invoke `vvx guard`. The wrapper detects platform and execs `vvr-{platform} guard`. They never need to know about VOK or the hidden voice that whispered it into existence.

**Commands:** `vvx guard`, etc.

### VVK - The Living Kit

VVK (Voce Viva Kit) is the installable artifact that VOK produces. While VOK is veiled and never leaves the source repo, VVK is visible and travels to target repos.

```
Tools/vok/                    # Veiled - source repo only
  voa_arcanum.sh              # THE arcanum (does the installing)
  Cargo.toml, src/, build.rs  # Rust compilation
  release/                    # Multi-platform binaries built here
    darwin-arm64/vvr
    linux-x86_64/vvr
    ...

Tools/vvk/                    # Visible - copied wholesale to targets
  vvg_git.sh                  # Git utilities (locking, guard)
  bin/
    vvx                       # Platform-selecting wrapper (checked in)
    vvr-darwin-arm64          # Binaries (populated by VOK release)
    vvr-darwin-x86_64
    vvr-linux-x86_64
    vvr-linux-aarch64
    vvr-windows-x86_64.exe
  README.md
```

**VOK arcanum (`voa-i <target>`) does:**
1. Delete `<target>/Tools/vvk/` if exists
2. Copy `Tools/vvk/` from source → target
3. Emit slash commands to `<target>/.claude/commands/`
4. Patch `<target>/CLAUDE.md`

**The relationship:**
- VOK compiles vvr for all platforms → `Tools/vok/release/*/vvr`
- VOK release populates `Tools/vvk/bin/vvr-*`
- VVK is a complete, self-contained artifact (bash + binaries)
- VOK arcanum installs VVK + Claude config into target repos

### VV Prefix Map

| Prefix | Name | Purpose |
|--------|------|---------|
| `vvc-` | Command | Slash commands (per `*c_` convention) |
| `vvg_` | Git | Git utilities (locking, guard, refs) |
| `vvk` | Kit | Directory name (terminal) |
| `vvr` | Rust | Binary base name (terminal) |
| `vvx` | eXecutor | Platform wrapper (terminal) |

### Platform Binary Selection

VVK includes binaries for all supported platforms. Selection happens at runtime via a wrapper script:

```bash
# Tools/vvk/bin/vvx (checked into git)
#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)   exec "$SCRIPT_DIR/vvr-darwin-arm64" "$@" ;;
  Darwin-x86_64)  exec "$SCRIPT_DIR/vvr-darwin-x86_64" "$@" ;;
  Linux-x86_64)   exec "$SCRIPT_DIR/vvr-linux-x86_64" "$@" ;;
  Linux-aarch64)  exec "$SCRIPT_DIR/vvr-linux-aarch64" "$@" ;;
  MINGW*|MSYS*)   exec "$SCRIPT_DIR/vvr-windows-x86_64.exe" "$@" ;;
  *)              echo "vvx: unsupported platform: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac
```

**Why this approach:**
- No priming step after clone
- Works identically in source and target repos
- Arcanum scribes absolute path to wrapper
- Tiny overhead (single exec)

### Git Locking Infrastructure

VVK provides git-based locking for multi-agent coordination via `vvg_git.sh`.

**Why git update-ref:**
- Git already solves distributed coordination
- `git update-ref` provides atomic operations
- Platform-portable (Git handles macOS/Linux/Windows)
- Temporally stable (Git CLI more stable than git2 crate)

**Refs namespace:** `refs/vvg/locks/*`

**Functions:**
```bash
vvg_lock_acquire "resource"   # Create lock, fail if exists
vvg_lock_release "resource"   # Release lock (normal use)
vvg_lock_break "resource"     # Unconditional release (human recovery)
```

**Usage pattern:**
```bash
source Tools/vvk/vvg_git.sh

vvg_lock_acquire "studbook" || buc_die "Lock held, try later"
# ... do work ...
vvg_lock_release "studbook"
```

**Recovery:** If a script fails mid-operation, human runs `vvg_lock_break "studbook"` to clear the stale lock.

Kits using VVK locking (JJK, etc.) source `vvg_git.sh` and call these functions.

### VO Prefix Map

| Prefix | Name | Purpose |
|--------|------|---------|
| `voa_` | Arcanum | Main kit entry (install/uninstall) |
| `vol_` | Ledger | Release record (`vol_ledger.json`) |
| `vop_` | Prepare | Release preparation (compile/test/package) |
| `vos_` | Sigil | Version ID |
| `vov_` | Veil | Upstream filter |
| `vox_` | Codex | Version tracking |

### Reserved Suffixes (Claude Code Native Types)

All kits use these consistently:

| Suffix | Type | Notes |
|--------|------|-------|
| `*a_` | Arcanum | Kit main entry |
| `*b_` | suBagent | |
| `*c-` | slash Command | Hyphen for command names (`/vvc-commit`) |
| `*g_` | Git utilities | Locking, refs, guard |
| `*h_` | Hook | |
| `*k` | Kit directory | Terminal (no underscore) |
| `*l_` | Ledger | Release records |
| `*r` | Rust binary | Terminal (no underscore) |
| `*t_` | Testbench | |
| `*w_` | Workbench | |
| `*x` | eXecutor | Platform wrapper, terminal |

### Kit Facility Pattern

Kits may implement one or more facilities:

- **Arcanum** (`*a_`) - Claude environment installation (slash commands, subagents, hooks, skills). Hidden voice setup layer.
- **Workbench** (`*w_`) - Classic dispatch of tabtargets to bash utilities. User-facing operational tooling.
- **Testbench** (`*t_`) - Testing facility for bash utilities.

### Kit Operations Pattern

| Pattern | Operation |
|---------|-----------|
| `*k-i` | Install (runs FROM source repo, targets another repo) |
| `*k-u` | Uninstall |
| `*k-c` | Check |

### The Two-Repo Install Model

Kits are developed in a **source repo** (kit forge) and installed into **target repos** (consumers).

```
Source Repo (Kit Forge):
  Tools/jjk/              # kit source lives here permanently
    jja_arcanum.sh        # install script runs FROM here
    veiled/               # never distributed
      ...

Target Repo (Consumer):
  Tools/vvk/              # VVK copied wholesale from source
    bin/vvx               # Platform wrapper
    bin/vvr-*             # Platform binaries
    vvg_git.sh            # Git utilities
  .claude/
    commands/             # emitted slash commands
    kit_manifest.json     # what's installed
  CLAUDE.md               # patched with kit sections
```

**Install command:**
```bash
cd ~/kit-forge                              # source repo
./Tools/jjk/jja_arcanum.sh jja-i ~/my-app   # install into target
```

The arcanum:
1. Knows where IT lives (source repo)
2. Takes target repo path as argument
3. Copies `Tools/vvk/` wholesale (includes vvx wrapper + vvr binaries)
4. Emits slash commands into target's .claude/commands/
5. Patches target's CLAUDE.md
6. Records in target's .claude/kit_manifest.json

**Why this model:**
- Kit source never travels to target
- Ledger stays in source repo (never distributed)
- Veiled content automatically excluded (it's not copied)
- Clean separation: source repo = development, target repo = consumption

### The veiled/ Convention

Each kit has a `veiled/` subdirectory that **never leaves the source repo**.

```
Tools/«kit»/
  «kit»a_arcanum.sh       # public: the install script
  README.md               # public: basic docs
  veiled/                 # NEVER distributed
    Cargo.toml            # Rust crate root (flat!)
    src/
      lib.rs              # Rust source
    «kit»l_ledger.json    # release record
    internal-notes.md     # dev docs
```

**Structural veiling:** No config needed. Directory name IS the rule.

**Flat inside veiled/:** Only `src/` nesting (required by Cargo). Everything else (ledger, docs) sits alongside Cargo.toml.

**What lives in veiled/:**
- Rust source for this kit
- Release ledger (history of releases)
- Internal documentation
- Test fixtures
- Anything source-repo-only

**What lives outside veiled/:**
- Arcanum script (the "API" for installation)
- Public README
- Anything that might travel to target or upstream

**prep-pr filter:** Simply `--exclude='*/veiled/'`

### VOK Guard - Pre-commit Size Validation

Core VOK utility that measures staged blob sizes before commit. Prevents catastrophic auto-adds (node_modules, build artifacts, binaries) by enforcing size limits.

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
vvx guard [--limit <bytes>] [--warn <bytes>]
  Exit 0: under limit
  Exit 1: over limit (with breakdown by file)
  Exit 2: over warn threshold (proceed with caution)
```

**Subcommand:** `vvx guard` (invokes `vvr guard`, core VOK functionality in `Tools/vok/src/guard.rs`)

### CRCG - Claude Rust Coding Guide

CRCG (`Tools/vok/CRCG.md`) documents constraints for small Rust utilities that complement BCG-constrained bash. These are text-wrangling tools too sophisticated for reliable bash/jq but simple enough to stay minimal.

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

### VOK Rust Architecture

Single `vvr` (Voce Viva Rust) multicall binary. Core functionality (guard) lives in VOK. Kit-specific Rust code lives WITH each kit (in veiled/), VOK references via path dependencies and feature flags.

**VOK is inherently veiled** - the entire `Tools/vok/` directory never leaves the source repo. No veiled/ subdirectory needed.

```
Tools/vok/                    # ALL of this is veiled (never distributed)
  voa_arcanum.sh
  Cargo.toml                  # [package] name = "vvr"
  Cargo.lock                  # checked in
  build.rs                    # auto-detects kit veiled/ dirs
  src/
    main.rs                   # multicall dispatch
    guard.rs                  # core: pre-commit size validation
    core.rs                   # shared infrastructure
  release/                    # built binaries per platform
    darwin-arm64/vvr
    linux-x86_64/vvr
  vol_ledger.json             # VOK release record
  README.md                   # full internal docs

Tools/jjk/                    # public part
  jja_arcanum.sh
  veiled/                     # private part
    Cargo.toml                # [lib] name = "jjk"
    src/lib.rs                # JJK Rust code
    jjl_ledger.json
```

**VOK Cargo.toml:**
```toml
[package]
name = "vvr"   # Voce Viva Rust

[features]
default = []
jjk = ["dep:jjk"]

[dependencies]
jjk = { path = "../jjk/veiled", optional = true }
clap = "4"
serde_json = "1"
```

**build.rs auto-detection:**
```rust
fn main() {
    if Path::new("../jjk/veiled/Cargo.toml").exists() {
        println!("cargo:rustc-cfg=feature=\"jjk\"");
    }
}
```

**Why this structure:**
- Core functionality (guard) always available in VOK
- Kit-specific Rust lives with kit (co-maintenance)
- VOK orchestrates via path deps (no copying)
- Single binary output (std/serde link once)
- Feature flags compile out absent kits
- Each kit's veiled/ is self-contained

### Prepare Release vs Arcanum Install

Two distinct operations in the two-repo model:

| Operation | Script | Where | What |
|-----------|--------|-------|------|
| **Prepare Release** | `vop_prepare_release.sh` | Source repo | Build, test, package, record in ledger |
| **Install** | `«kit»a_arcanum.sh` | Source repo → Target repo | Copy artifacts, emit commands, patch CLAUDE.md |

**Prepare Release flow:**
```bash
cd ~/kit-forge
tt/vok-R.GenerateRelease.sh
  → cargo build --release (in Tools/vok/)
  → cargo test
  → copy vvr binary to Tools/vok/release/«platform»/
  → compute release hash
  → record in Tools/vok/vol_ledger.json
```

**Install flow:**
```bash
cd ~/kit-forge
./Tools/jjk/jja_arcanum.sh jja-i ~/my-app
  → copy Tools/vvk/ to ~/my-app/Tools/vvk/ (wrapper + utilities)
  → copy Tools/vok/release/*/vvr to ~/my-app/Tools/vvk/bin/vvr-* (binaries)
  → emit slash commands to ~/my-app/.claude/commands/
  → patch ~/my-app/CLAUDE.md
  → record in ~/my-app/.claude/kit_manifest.json
```

**Target repo structure:**
```
Tools/vvk/
  bin/
    vvx                   # Platform wrapper
    vvr-darwin-arm64      # Platform binaries
    vvr-linux-x86_64
    ...
  vvg_git.sh              # Git utilities
.claude/
  commands/
    jjc-notch.md
    jjc-heat-saddle.md
  kit_manifest.json       # what's installed, which versions
```

### Ledger as Release Record

Each kit's ledger lives in veiled/ and records release events:

```json
{
  "releases": [
    {
      "sigil": "v1.0.0",
      "date": "2025-01-10",
      "source_hash": "abc123def456",
      "features": ["jjk"],
      "platforms": ["darwin-arm64", "linux-x86_64"],
      "commit": "2597deb"
    }
  ]
}
```

**What's recorded:**
- **sigil**: Version identifier
- **date**: When released
- **source_hash**: Hash of source artifacts
- **features**: Which kit features were enabled
- **platforms**: Which platforms were built
- **commit**: Git commit at release time

### The prep-pr Flow

For contributing to upstream from a private fork:

```
1. Read VOK config (which kits present, which upstream-safe)
2. Build vvr binary (features for upstream kits only)
3. Run tests (gate on green)
4. Compute release hash
5. Assign sigil, record in ledger
6. Apply veil (exclude */veiled/ directories)
7. Prepare clean PR branch
```

**Private fork workflow:**
- Your fork has internal kits (e.g., Tools/internal-kit/)
- Upstream has shared kits (e.g., Tools/jjk/)
- prep-pr builds minimal vvr for upstream (only shared kit features)
- prep-pr excludes all veiled/ content
- PR is clean, upstream-ready

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
- `vvx --help` exists (wrapper shows usage), `vvr --help` (clap-generated), but no external docs
- Bash scripts bridge slash commands to vvx calls
- Only source repo readers see the full picture

**The veiled README:**
`Tools/vok/README.md` documents the compilation model and full API - for kit authors only. Since VOK is inherently veiled, it never leaves the source repo.

## Done

- **Bootstrap VOK+VVK** — Created VOK crate, VVK wrapper, release dirs, gitignore for dev/release separation
- **Write VOK README** — 301-line kit-author reference: compilation model, two-repo install, veiled convention, Voce Viva, Rust architecture, API, glossary

## Remaining

### Foundation

- **Clarify kit facility patterns**
  Document when kits use arcanum vs workbench vs testbench. Can kits have multiple? Where does model differential tool belong (JJ workbench)? Output: Section in VOK README.

- **Create VOK skeleton**
  Create `Tools/vok/` (inherently veiled - no veiled/ subdirectory needed):
  ```
  Tools/vok/
    voa_arcanum.sh              # two-repo install logic
    Cargo.toml                  # vvr binary crate
    build.rs                    # kit auto-detection
    src/
      main.rs                   # multicall dispatch
      guard.rs                  # core: pre-commit size validation
      core.rs                   # shared infrastructure
    release/                    # tracked release binaries (arcanum source)
      darwin-arm64/vvr
      darwin-x86_64/vvr
      linux-x86_64/vvr
      linux-aarch64/vvr
      windows-x86_64/vvr.exe
    vol_ledger.json             # release record (empty)
    README.md                   # full internal docs
  ```

### VVK Infrastructure

- **Create VVK skeleton**
  Create `Tools/vvk/` directory structure:
  ```
  Tools/vvk/
    vvg_git.sh                  # Git utilities (locking, guard) - stub
    bin/
      vvx                       # Platform-selecting wrapper (checked in)
    README.md                   # VVK documentation
  ```
  The `bin/vvr-*` binaries will be populated by VOK release.

- **Create platform wrapper script**
  Create `Tools/vvk/bin/vvx` - the platform-detecting wrapper that execs correct `vvr-{platform}` binary. Handle Darwin-arm64, Darwin-x86_64, Linux-x86_64, Linux-aarch64, Windows (MINGW/MSYS). Checked into git. Exit with clear error on unsupported platform.

- **Implement git locking utilities**
  Create `Tools/vvk/vvg_git.sh` with BCG-compliant implementation:
  - `vvg_lock_acquire(resource)` - create `refs/vvg/locks/<resource>`, fail if exists
  - `vvg_lock_release(resource)` - delete ref (normal use)
  - `vvg_lock_break(resource)` - unconditional delete (human recovery)
  - Include guard pattern (ZVVG_INCLUDED)
  - Use `git update-ref` for atomic operations

- **Create VVK README**
  Document: (1) VVK purpose (living kit, installed artifact), (2) Directory structure, (3) Platform wrapper usage, (4) Git locking API (`vvg_lock_*`), (5) Relationship to VOK.

- **Implement guarded commit facility**
  Create guarded commit workflow with size validation and background processing.

  **`vvg_guard()` function in `Tools/vvk/vvg_git.sh`** - Pre-commit guard:
  - Acquire lock via `vvg_lock_acquire "commit"`
  - Run `vvx guard --limit ${VVG_SIZE_LIMIT:-500000}`
  - On guard failure: release lock, return non-zero with message
  - On success: return 0 (lock remains held for agent)

  **Slash command `/vvc-commit <description>`:**
  1. Source `vvg_git.sh`, run `vvg_guard` — non-zero stops, 0 proceeds
  2. Spawn background sonnet agent (Task tool, run_in_background=true) with prompt:
     - User's description
     - Commit format template (from CLAUDE.md config)
     - Git safety rules: no force push, no skip hooks, no secrets, `git add -u` only
     - Steps: `git add -u`, `git diff --cached --stat`, construct message, `git commit`
     - Final step: `source Tools/vvk/vvg_git.sh && vvg_lock_release "commit"`
     - Report: hash, files changed, remind user to push
  3. Report "Commit dispatched" and return immediately

  **Commit message format** (default, configured in CLAUDE.md):
  ```
  <description>

  - <change 1>
  - <change 2>

  Co-Authored-By: Claude <noreply@anthropic.com>
  ```

  **Git safety rules** (embedded in agent prompt):
  - Never update git config
  - Never force push or skip hooks
  - Never commit files matching secret patterns (.env, credentials.*, etc.)
  - Only modified/deleted files (`git add -u`), new files allowed (guard protects)
  - No auto-push (sandbox considerations), remind user to push

  **VOK arcanum emits:**
  - Slash command file: `.claude/commands/vvc-commit.md`
  - CLAUDE.md section: commit format template, size limit (default 500KB)

### Rust Infrastructure

- **Create VOK Rust crate (vvr)**
  In `Tools/vok/`: Cargo.toml with `name = "vvr"` and path deps to kit veiled/ dirs. build.rs that detects `../«kit»/veiled/Cargo.toml` and enables features. src/main.rs with clap multicall dispatch. src/core.rs with shared platform/env utilities. No kit logic yet - just skeleton that compiles and produces `vvr` binary.

- **Implement vvr guard subcommand**
  Add guard logic to `Tools/vok/src/guard.rs`. Core VOK functionality, always available. Pure filter: bash pipes git output, vvr returns verdict. Exit codes: 0 (ok), 1 (over), 2 (warn). This enables the guarded commit facility in VVK.

- **Create JJK veiled/ Rust crate**
  In `Tools/jjk/veiled/`: Cargo.toml as `[lib]` crate. src/lib.rs with placeholder exports. Move jjl_ledger.json here (rename from brand-based). This establishes the kit Rust co-location pattern.

- **Design CRCG**
  Write Claude Rust Coding Guide as `Tools/vok/CRCG.md`. Document: pure filter pattern, minimal deps, exit codes, Cargo.lock pinning, 5 target platforms, BCG integration. This guides all kit Rust development.

### Release & Install

- **Create Prepare Release script**
  `Tools/vok/vop_prepare_release.sh`: Build vvr binary for current platform (`cargo build --release`), run tests, copy to `Tools/vok/release/«platform»/vvr`. Compute hash, record in `vol_ledger.json`. Tabtarget: `tt/vok-R.GenerateRelease.sh`. Note: dev workflow copies to `vvk/bin/` for local testing (gitignored); release copies to `vok/release/` (tracked).

- **Implement two-repo arcanum install**
  Rewrite `voa_arcanum.sh` for VVK model. Takes target path as argument. Steps:
  1. Delete `<target>/Tools/vvk/` if exists
  2. Copy `Tools/vvk/` from source → `<target>/Tools/vvk/` (wrapper + utilities, no binaries)
  3. Copy `Tools/vok/release/*/vvr` → `<target>/Tools/vvk/bin/vvr-*` (release binaries)
  4. Emit slash commands to `<target>/.claude/commands/`
  5. Patch `<target>/CLAUDE.md` with VVK configuration
  6. Record in `<target>/.claude/kit_manifest.json`
  Fails if release binaries missing in `Tools/vok/release/` (run release first).

- **Implement prep-pr flow**
  Script that: reads which kits are upstream-safe, builds minimal vvr (features for upstream kits only), records release, applies veil (`--exclude='*/veiled/'`), prepares PR branch. Move veil config from CMK.

### Kit Migration

- **Migrate JJK to veiled/ structure**
  Move `Tools/jjk/jjl_ledger.json` to `Tools/jjk/veiled/jjl_ledger.json`. Update arcanum to work in two-repo model. JJK arcanum calls vvx (which dispatches to vvr) for Rust operations.
