# VVK - Voce Viva Kit

The living voice. User-facing tooling produced by VOK.

VVK is the **distributable artifact** that travels from source repo to target repos. It contains platform binaries and bash utilities that scripts and slash commands invoke.

## Directory Structure

```
Tools/vvk/
  vvg_git.sh              # Git utilities (locking, guard)
  bin/
    vvx                   # Platform-selecting wrapper
    vvr-darwin-arm64      # Platform binaries (populated by release)
    vvr-darwin-x86_64
    vvr-linux-x86_64
    vvr-linux-aarch64
    vvr-windows-x86_64.exe
  README.md               # This file
```

## Platform Wrapper (vvx)

The `vvx` script detects the current platform and executes the appropriate `vvr-{platform}` binary.

```bash
# Usage
Tools/vvk/bin/vvx --version
Tools/vvk/bin/vvx guard --limit 500000
```

**Supported platforms:**
- `darwin-arm64` — macOS Apple Silicon
- `darwin-x86_64` — macOS Intel
- `linux-x86_64` — Linux Intel/AMD, WSL, CI
- `linux-aarch64` — AWS Graviton, ARM servers
- `windows-x86_64` — Windows (MINGW/MSYS)

## Git Utilities (vvg_git.sh)

Source the script to access git-based locking functions.

```bash
source Tools/vvk/vvg_git.sh

# Acquire a lock (fails if already held)
vvg_lock_acquire "myresource" || echo "Lock held by another process"

# Do work...

# Release the lock
vvg_lock_release "myresource"
```

### Lock Functions

| Function | Purpose |
|----------|---------|
| `vvg_lock_acquire <resource>` | Create lock, fail if exists |
| `vvg_lock_release <resource>` | Release lock (normal use) |
| `vvg_lock_break <resource>` | Force release (human recovery) |
| `vvg_lock_check <resource>` | Check if lock held (0=held, 1=free) |
| `vvg_lock_list` | List all held locks |

**Refs namespace:** `refs/vvg/locks/*`

### Guard Function

```bash
source Tools/vvk/vvg_git.sh

# Run pre-commit size check
vvg_guard  # Uses VVG_SIZE_LIMIT (default 500KB)
```

Environment variables:
- `VVG_SIZE_LIMIT` — Maximum staged blob size in bytes (default: 500000)
- `VVG_WARN_LIMIT` — Warning threshold in bytes (default: 250000)

## vvr Subcommands

The `vvr` binary (invoked via `vvx`) provides:

### guard

Pre-commit size validation. Measures staged blob sizes to prevent accidental large commits.

```
vvx guard [--limit <bytes>] [--warn <bytes>]
```

**Exit codes:**
- 0 — Under limit
- 1 — Over limit (blocks commit)
- 2 — Over warn threshold (proceed with caution)

## Installation

VVK is installed by kit arcanums (e.g., JJK). The arcanum:
1. Copies `Tools/vvk/` from source repo
2. Copies release binaries from `Tools/vok/release/*/vvr`
3. Emits slash commands that invoke vvx

**Do not install VVK manually.** Use the appropriate kit arcanum.

## Relationship to VOK

```
VOK (Vox Obscura Kit)     - Source repo only, never distributed
       ↓ (release process)
VVK (Voce Viva Kit)       - Distributed to target repos
  └── vvx → vvr           - What users invoke
```

VOK compiles the `vvr` binary and prepares releases. VVK is the installable artifact containing the wrapper, utilities, and binaries.
