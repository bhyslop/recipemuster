# VOK - Vox Obscura Kit

The hidden voice. Foundational infrastructure for Claude Code kits.

VOK is **inherently veiled** - this entire directory never leaves the source repo. This README is for kit authors only.

## The Compilation Model

Arcanum emitters are compiled documentation. We (Claude + human) are the compiler.

```
Source:      Full API knowledge (Rust source, specs, session context)
Compiler:    Us writing arcanum emitter functions
Object code: The arcanum emitters themselves (zjjw_emit_*, etc.)
Install:     Mechanical deployment - runs emitters, copies binaries
Runtime:     Target Claude executes emitted instructions
```

**Knowledge lifecycle:**
- API understanding exists during our working session (compilation time)
- We encode it into `z*_emit_*` functions
- Knowledge is embedded in code, not documented separately
- Install deploys compiled artifacts to target repo
- Target Claude follows instructions without understanding the API
- Original knowledge doesn't persist - the emitters ARE the documentation

**Why this enables obscurity:**
- No API spec to publish - knowledge compiles away
- Target repo has instructions only, not understanding
- `vvx --help` and `vvr --help` exist (clap-generated), but no external docs
- Bash scripts bridge slash commands to vvx calls
- Only source repo readers see the full picture

## Two-Repo Install Model

Kits are developed in a **source repo** (kit forge) and installed into **target repos** (consumers).

```
Source Repo (Kit Forge):
  Tools/vok/              # VOK - veiled, never distributed
    release/              # Built binaries (tracked, source of truth)
      darwin-arm64/vvr
      linux-x86_64/vvr
      ...
  Tools/vvk/              # VVK template (wrapper + utilities)
    bin/vvx               # Platform wrapper (tracked)
    bin/vvr-*             # Dev binaries (gitignored)
  Tools/jjk/              # Other kits
    jja_arcanum.sh        # Install script
    veiled/               # Never distributed

Target Repo (Consumer):
  Tools/vvk/              # Assembled by arcanum
    bin/vvx               # Platform wrapper
    bin/vvr-*             # Release binaries (from vok/release/)
  .claude/
    commands/             # Emitted slash commands
    kit_manifest.json     # What's installed
  CLAUDE.md               # Patched with kit sections
```

**Install command:**
```bash
cd ~/kit-forge                              # source repo
./Tools/jjk/jja_arcanum.sh jja-i ~/my-app   # install into target
```

**The arcanum:**
1. Knows where IT lives (source repo)
2. Takes target repo path as argument
3. Copies `Tools/vvk/` (wrapper + utilities, not dev binaries)
4. Copies `Tools/vok/release/*/vvr` → target's `vvk/bin/vvr-*`
5. Emits slash commands into target's `.claude/commands/`
6. Patches target's `CLAUDE.md`
7. Records in target's `.claude/kit_manifest.json`

**Why this model:**
- Kit source never travels to target
- Ledger stays in source repo (never distributed)
- Veiled content automatically excluded
- Clean separation: source repo = development, target repo = consumption

## The veiled/ Convention

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

**Flat inside veiled/:** Only `src/` nesting (required by Cargo). Everything else sits alongside Cargo.toml.

**What lives in veiled/:**
- Rust source for this kit
- Release ledger (history of releases)
- Internal documentation
- Test fixtures

**What lives outside veiled/:**
- Arcanum script (the "API" for installation)
- Public README
- Anything that might travel to target or upstream

**prep-pr filter:** Simply `--exclude='*/veiled/'`

**VOK exception:** VOK is inherently veiled (entire `Tools/vok/` stays in source repo), so it has no veiled/ subdirectory.

## Voce Viva Concept

**Vox Obscura** (the hidden voice) is the source - this kit.
**Voce Viva** (the living voice) is what users experience.

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

Users invoke `vvx guard`. The wrapper detects platform and execs `vvr-{platform} guard`. They never need to know about VOK.

## VOK Rust Architecture

Single `vvr` (Voce Viva Rust) multicall binary. Core functionality lives in VOK. Kit-specific Rust code lives WITH each kit (in veiled/), VOK references via path dependencies and feature flags.

```
Tools/vok/                    # ALL veiled (never distributed)
  Cargo.toml                  # [package] name = "vvr"
  Cargo.lock                  # checked in (reproducible builds)
  build.rs                    # auto-detects kit veiled/ dirs
  src/
    vorm_main.rs              # multicall dispatch
    vorg_guard.rs             # core: pre-commit size validation
    vorc_core.rs              # shared infrastructure
  release/                    # built binaries (tracked)
    darwin-arm64/vvr
    linux-x86_64/vvr
    ...

Tools/jjk/                    # other kit
  jja_arcanum.sh              # public
  veiled/                     # private
    Cargo.toml                # [lib] name = "jjk"
    src/lib.rs                # JJK Rust code
```

**VOK Cargo.toml pattern:**
```toml
[package]
name = "vvr"

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
- Core functionality (guard) always available
- Kit-specific Rust lives with kit (co-maintenance)
- VOK orchestrates via path deps (no copying)
- Single binary output (std/serde link once)
- Feature flags compile out absent kits

**Target platforms (5):**

| Platform | Rust triple | Notes |
|----------|-------------|-------|
| darwin-arm64 | `aarch64-apple-darwin` | macOS Apple Silicon |
| darwin-x86_64 | `x86_64-apple-darwin` | macOS Intel |
| linux-x86_64 | `x86_64-unknown-linux-gnu` | Linux Intel/AMD, WSL, CI |
| linux-aarch64 | `aarch64-unknown-linux-gnu` | AWS Graviton, ARM servers |
| windows-x86_64 | `x86_64-pc-windows-gnu` | Windows (gnu for cross-compile) |

## vvx/vvr Subcommand API

### guard

Pre-commit size validation. Measures staged blob sizes, prevents catastrophic auto-adds.

```
vvx guard [--limit <bytes>] [--warn <bytes>]
```

**Exit codes:**
- 0: Under limit
- 1: Over limit (with breakdown by file)
- 2: Over warn threshold (proceed with caution)

**Why blob size:** Git commits blobs, not files. Pack compression is non-deterministic. Blob size is the correct pre-commit proxy.

### Future subcommands

Kit-specific subcommands are added via feature flags. When JJK Rust code exists:
```
vvx jj <subcommand>    # JJK functionality (when jjk feature enabled)
```

## Glossary

| Term | Definition |
|------|------------|
| **Arcanum** | Kit's install script (`*a_`). The hidden knowledge that configures Claude. |
| **Cipher** | Project prefix (2-5 chars). Const declarations in `voci_`. Globally unique namespace root. |
| **Codex** | Version tracking system (`vox_`) |
| **Conclave** | Kit registration API (`vocv_`). Collects whispers, validates ciphers. |
| **Kit** | A distributable Claude Code extension (directory in Tools/) |
| **Kit Forge** | Source repo where kits are developed |
| **Ledger** | Release record (`*l_`). JSON history of releases. |
| **Parcel** | Distribution archive (`vvk-parcel-{sigil}.tar.gz`). Contains binaries and kit assets. |
| **Sigil** | Version identifier (YYMMDD-HHMM format). Generated at release time. |
| **Target Repo** | Consumer repo where kits are installed |
| **Veil** | Upstream filter (`vov_`). Excludes private content from PRs. |
| **Veiled** | Content that never leaves the source repo (`vov_veiled/` directories) |
| **Voce Viva** | The living voice - user-facing tooling (VVK, vvx) |
| **Vox Obscura** | The hidden voice - source infrastructure (VOK) |
| **vvr** | Voce Viva Rust - the compiled binary |
| **vvx** | Platform wrapper that dispatches to correct vvr binary |
| **Whisper** | Kit's declaration to conclave (`*w_`). Builder API for registration. |

## Prefix Maps

### VO Prefixes (VOK internal)

| Prefix | Name | Purpose |
|--------|------|---------|
| `voa_` | Arcanum | Main kit entry (install/uninstall) |
| `voci_` | Cipher | Project prefix registry (const declarations) |
| `vocv_` | Conclave | Kit registration API (collects whispers) |
| `vol_` | Ledger | Release record |
| `vop_` | Prepare | Release preparation |
| `vor`  | Rust source | Has children (not terminal) |
| `vorg_` | Rust Guard | Pre-commit size validation |
| `vorm_` | Rust Main | Binary entry point |
| `vov_` | Veil | Upstream filter |
| `vox_` | Codex | Version tracking |

### VV Prefixes (VVK distributed)

| Prefix | Name | Purpose |
|--------|------|---------|
| `vvc-` | Command | Slash commands |
| `vvk` | Kit | Directory name (terminal) |
| `vvr` | Rust | Binary base name (terminal) |
| `vvx` | eXecutor | Platform wrapper (terminal) |

### Reserved Kit Suffixes

| Suffix | Type | Notes |
|--------|------|-------|
| `*a_` | Arcanum | Kit main entry |
| `*b_` | suBagent | |
| `*c-` | slash Command | Hyphen for names (`/vvc-commit`) |
| `*g_` | Git utilities | |
| `*h_` | Hook | |
| `*k` | Kit directory | Terminal |
| `*l_` | Ledger | |
| `*r` | Rust binary | Terminal |
| `*t_` | Testbench | |
| `*w_` | Workbench | |
| `*x` | eXecutor | Terminal |

## Kit Operations

| Pattern | Operation |
|---------|-----------|
| `*k-i` | Install (from source → target) |
| `*k-u` | Uninstall |
| `*k-c` | Check installation |

## Kit Facilities

Kits may implement one or more facilities. Most kits have multiple.

### Facility Types

| Facility | Suffix | Purpose | Audience |
|----------|--------|---------|----------|
| **Arcanum** | `*a_` | Claude environment setup | Claude Code |
| **Workbench** | `*w_` | Tabtarget dispatch to bash | Humans via terminal |
| **Testbench** | `*t_` | Test harness for bash utilities | Developers |

### When to Use Each

**Arcanum** — Use when the kit needs to configure Claude Code:
- Emit slash commands to `.claude/commands/`
- Register subagents, hooks, or skills
- Patch `CLAUDE.md` with kit configuration
- Any "install into target repo" logic

**Workbench** — Use for human-invoked operations via tabtargets:
- Commands that humans run from terminal (`tt/jjw-H.HeatSaddle.sh`)
- Operations that don't need Claude (pure bash workflows)
- Tools that predate or complement Claude integration

**Testbench** — Use for automated testing:
- Unit tests for bash functions
- Integration tests for kit workflows
- CI/CD validation

### Facility Combinations

| Kit Pattern | Facilities | Example |
|-------------|------------|---------|
| Claude-only | Arcanum | VOK (installs VVK + config) |
| Human-only | Workbench | Legacy bash tooling |
| Claude + Human | Arcanum + Workbench | JJK (slash commands + tabtargets) |
| Any + Testing | + Testbench | Add to any of above |

**Common pattern:** Arcanum + Workbench. The arcanum installs Claude integration (slash commands that call vvx). The workbench provides tabtargets for humans who prefer terminal over Claude.

### Tool Placement Decision

**Question:** Where does a new tool belong?

```
Is it Claude-environment setup (slash commands, hooks, CLAUDE.md)?
  → Arcanum emitter function (z*a_emit_*)

Is it a user-facing operation (humans invoke from terminal)?
  → Workbench dispatch function (*w_*)

Is it a Rust utility (text processing too complex for bash)?
  → vvr subcommand (in kit's veiled/src/)

Is it test automation?
  → Testbench function (*t_*)
```

**Example: Model differential tool**
A tool that compares model versions is user-facing (humans want to see diffs). It belongs in JJ workbench (`jjw_`), not arcanum. If it needs Rust for complex diffing, the workbench function calls `vvx jj diff` which dispatches to Rust code in `jjk/veiled/src/`.
