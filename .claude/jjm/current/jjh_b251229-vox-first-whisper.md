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

**vv** (Voce Viva) is the creature that springs from the release process - the refined yet secretive executable that animates Claude context management tooling in target repos.

```
VOK (Vox Obscura Kit)     - The veiled source, hidden infrastructure
       ↓ (release process - whispering into existence)
vv (Voce Viva)            - The living voice, what users experience
vvr                       - The Rust binary that embodies vv
```

**The relationship:**
- **VOK** is the *source* - veiled, arcane, never seen by users
- **vv** is the *concept* - the living voice, creation, symphony, beauty
- **vvr** is the *creature* - the actual binary that helps users

Users know vvr. They experience it as helpful and alive. They never need to know about VOK or the hidden voice that whispered it into existence.

**Commands:** `vvr guard`, `vvr jj`, etc.

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
  .claude/                # only installed artifacts
    commands/             # emitted slash commands
    bin/vvr               # Voce Viva Rust binary
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
3. Copies pre-built binaries from its own veiled/release/
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

**Subcommand:** `vvr guard` (cgk feature-gated, code in `Tools/cgk/veiled/src/lib.rs`)

### CRCG - Claude Rust Coding Guide

CRCG (`Tools/vok/veiled/CRCG.md`) documents constraints for small Rust utilities that complement BCG-constrained bash. These are text-wrangling tools too sophisticated for reliable bash/jq but simple enough to stay minimal.

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

Single `vvr` (Voce Viva Rust) multicall binary with feature-gated subcommands. Kit Rust source lives WITH each kit (in veiled/), VOK references via path dependencies.

```
Tools/vok/
  voa_arcanum.sh
  veiled/
    Cargo.toml          # [package] name = "vvr"
    Cargo.lock          # checked in
    build.rs            # auto-detects kit veiled/ dirs
    src/
      main.rs           # multicall dispatch
      core.rs           # shared infrastructure
    release/            # built binaries per platform
      darwin-arm64/vvr
      linux-x86_64/vvr
    vol_ledger.json     # VOK release record

Tools/jjk/
  jja_arcanum.sh
  veiled/
    Cargo.toml          # [lib] name = "jjk"
    src/lib.rs          # JJK Rust code
    jjl_ledger.json

Tools/cgk/
  cga_arcanum.sh
  veiled/
    Cargo.toml          # [lib] name = "cgk"
    src/lib.rs          # CGK Rust code
    cgl_ledger.json
```

**VOK Cargo.toml:**
```toml
[package]
name = "vvr"   # Voce Viva Rust

[features]
default = []
jjk = ["dep:jjk"]
cgk = ["dep:cgk"]

[dependencies]
jjk = { path = "../../jjk/veiled", optional = true }
cgk = { path = "../../cgk/veiled", optional = true }
clap = "4"
serde_json = "1"
```

**build.rs auto-detection:**
```rust
fn main() {
    if Path::new("../../jjk/veiled/Cargo.toml").exists() {
        println!("cargo:rustc-cfg=feature=\"jjk\"");
    }
    if Path::new("../../cgk/veiled/Cargo.toml").exists() {
        println!("cargo:rustc-cfg=feature=\"cgk\"");
    }
}
```

**Why this structure:**
- Kit Rust lives with kit (co-maintenance)
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
tt/vok-pr.PrepareRelease.sh
  → cargo build --release (in Tools/vok/veiled/)
  → cargo test
  → copy vvr binary to Tools/vok/veiled/release/«platform»/
  → compute release hash
  → record in Tools/vok/veiled/vol_ledger.json
```

**Install flow:**
```bash
cd ~/kit-forge
./Tools/jjk/jja_arcanum.sh jja-i ~/my-app
  → copy vvr binary to ~/my-app/.claude/bin/
  → emit slash commands to ~/my-app/.claude/commands/
  → patch ~/my-app/CLAUDE.md
  → record in ~/my-app/.claude/kit_manifest.json
```

**Target repo structure:**
```
.claude/
  bin/
    vvr                   # Voce Viva Rust, feature-gated
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
      "features": ["jjk", "cgk"],
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
- Upstream only has public kits (e.g., Tools/cgk/)
- prep-pr builds minimal vvr for upstream
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
- `vvr --help` exists (clap-generated), but no external docs
- Bash scripts bridge slash commands to vvr calls
- Only source repo readers see the full picture

**The veiled README:**
`Tools/vok/veiled/README.md` documents the compilation model and full API - for kit authors only. It lives in veiled/ so it never leaves the source repo.

## Done

## Remaining

### Foundation

- **Establish subcommand naming convention**
  References to "jj subcommand", "guard subcommand" need acronym-consistent naming. BLOCKED: Requires acronym convention document (user to provide). Once available, revise all paddock/pace references to use consistent pattern (e.g., if convention is `vvr-jj` vs `vvr jj` vs other). This affects: VOK Rust Architecture section, CGK Guard section, Deferred paces.

- **Write veiled VOK README**
  Create `Tools/vok/veiled/README.md` (lives in veiled/, never distributed). Document: (1) The Compilation Model, (2) Two-repo install model, (3) veiled/ convention, (4) Voce Viva concept (VOK → vv → vvr), (5) VOK Rust architecture with path deps, (6) Full vvr subcommand API reference (for kit authors writing arcanums), (7) Arcane vocabulary glossary. This is the foundational conceptual document for kit authors.

- **Clarify kit facility patterns**
  Document when kits use arcanum vs workbench vs testbench. Can kits have multiple? Where does model differential tool belong (JJ workbench, not CGK)? Output: Section in veiled README.

- **Create VOK skeleton**
  Create `Tools/vok/` with veiled/ structure:
  ```
  Tools/vok/
    voa_arcanum.sh              # two-repo install logic
    README.md                   # public minimal docs
    veiled/
      Cargo.toml                # vvr binary crate
      build.rs                  # kit auto-detection
      src/main.rs               # multicall stub
      vol_ledger.json           # release record (empty)
      README.md                 # full internal docs
  ```

### Rust Infrastructure

- **Create VOK veiled/ Rust crate**
  In `Tools/vok/veiled/`: Cargo.toml with `name = "vvr"` and path deps to kit veiled/ dirs. build.rs that detects `../../«kit»/veiled/Cargo.toml` and enables features. src/main.rs with clap multicall dispatch. src/core.rs with shared platform/env utilities. No kit logic yet - just skeleton that compiles and produces `vvr` binary.

- **Create JJK veiled/ Rust crate**
  In `Tools/jjk/veiled/`: Cargo.toml as `[lib]` crate. src/lib.rs with placeholder exports. Move jjl_ledger.json here (rename from brand-based). This establishes the kit Rust co-location pattern.

- **Create CGK veiled/ Rust crate**
  In `Tools/cgk/veiled/`: Cargo.toml as `[lib]` crate. src/lib.rs with guard logic (blob size validation). cgl_ledger.json for releases. First real Rust functionality - proves the path dep pattern works.

- **Design CRCG**
  Write Claude Rust Coding Guide as `Tools/vok/veiled/CRCG.md`. Document: pure filter pattern, minimal deps, exit codes, Cargo.lock pinning, 5 target platforms, BCG integration. This guides all kit Rust development.

### Release & Install

- **Create Prepare Release script**
  `Tools/vok/vop_prepare_release.sh`: Build vvr binary in veiled/ (`cargo build --release`), run tests, copy to `veiled/release/«platform»/`, compute hash, record in `veiled/vol_ledger.json`. Tabtarget: `tt/vok-pr.PrepareRelease.sh`.

- **Implement two-repo arcanum install**
  Rewrite `voa_arcanum.sh` for two-repo model. Takes target path as argument. Copies binary from `veiled/release/` to target's `.claude/bin/`. Emits commands to target's `.claude/commands/`. Patches target's CLAUDE.md. Records in target's `.claude/kit_manifest.json`. Fails if release artifacts missing.

- **Implement prep-pr flow**
  Script that: reads which kits are upstream-safe, builds minimal vvr (features for upstream kits only), records release, applies veil (`--exclude='*/veiled/'`), prepares PR branch. Move veil config from CMK.

### Kit Migration

- **Migrate JJK to veiled/ structure**
  Move `Tools/jjk/jjl_ledger.json` to `Tools/jjk/veiled/jjl_ledger.json`. Update arcanum to work in two-repo model. JJK arcanum calls vvr binary for Rust operations.

- **Create CGK skeleton**
  Create `Tools/cgk/` with cga_arcanum.sh and veiled/ structure. Arcanum emits guard-related slash commands. Calls `vvr guard` for size validation.

### Deferred

- **Implement vvr guard subcommand**
  Add guard logic to `Tools/cgk/veiled/src/lib.rs`. VOK imports and exposes as `vvr guard`. Pure filter: bash pipes git output, vvr returns verdict. Exit codes: 0 (ok), 1 (over), 2 (warn).

- **Implement vvr jj subcommand**
  Add JJ text processing to `Tools/jjk/veiled/src/lib.rs`. VOK imports and exposes as `vvr jj`. Operations TBD based on what JJK needs.
