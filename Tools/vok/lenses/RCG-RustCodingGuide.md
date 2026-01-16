<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Rust Coding Guide (RCG) - Pattern Reference

## Core Philosophy

RCG is deliberately minimal. Trust Claude's inherent Rust idioms for error handling, struct design, documentation, and general style. This guide covers only project-specific conventions that override or extend standard Rust practice.

The primary concern is **minting discipline** — ensuring all identifiers follow the project's prefix naming rules for clarity, grep-ability, and ease of discussion in non-Rust-aware contexts.

## Crate Boilerplate

Every crate's `lib.rs` must include this attribute to allow prefixed type names:

```rust
#![allow(non_camel_case_types)]
```

This is mandatory. RCG naming conventions override Rust's default CamelCase warnings.

## File Naming

### Source Files

Pattern: `{cipher}r{classifier}_{name}.rs`

| Component | Description |
|-----------|-------------|
| `{cipher}` | Project cipher (2-5 chars): `jj`, `vv`, `vo`, `cm`, `bu` |
| `r` | Rust source marker |
| `{classifier}` | Single letter, name typically starts with this letter |
| `{name}` | Descriptive snake_case name |

Examples:
- `jjrg_gallops.rs` — JJ Rust, classifier `g`, "gallops"
- `jjrc_core.rs` — JJ Rust, classifier `c`, "core"
- `vvcg_guard.rs` — VV Core, classifier `g`, "guard"

### Test Files

Pattern: `{cipher}t{classifier}_{name}.rs`

Tests use `t` instead of `r` after the cipher, with matching classifier:

| Source | Test |
|--------|------|
| `jjrg_gallops.rs` | `jjtg_gallops.rs` |
| `jjrc_core.rs` | `jjtc_core.rs` |
| `vvcg_guard.rs` | `vvtg_guard.rs` |

Module wiring in `lib.rs`:
```rust
pub mod jjrg_gallops;

#[cfg(test)]
mod jjtg_gallops;
```

## Declaration Naming

**All public declarations from a source file must start with that file's full prefix.**

This is non-negotiable. It enables grep-based discovery and clear provenance in discussions.

### Naming Patterns by Type

For a file with prefix `jjrg`:

| Declaration | Pattern | Example |
|-------------|---------|---------|
| Struct | `{prefix}_PascalName` | `jjrg_Gallops` |
| Enum | `{prefix}_PascalName` | `jjrg_PaceState` |
| Function | `{prefix}_snake_name` | `jjrg_load()` |
| Constant | `{PREFIX}_SCREAMING` | `JJRG_DEFAULT_PATH` |
| Trait | `{prefix}_TraitName` | `jjrg_Storable` |
| Type alias | `{prefix}_AliasName` | `jjrg_Result` |

### Impl Methods

Methods on prefixed types also carry the prefix:

```rust
impl jjrg_Gallops {
    pub fn jjrg_load(path: &Path) -> Result<Self, String> { ... }
    pub fn jjrg_save(&self, path: &Path) -> Result<(), String> { ... }
}
```

Rationale: When reading `gallops.jjrg_save(path)`, the provenance is immediately clear. In discussions, "call jjrg_save" is unambiguous.

## Internal Declarations

Internal (crate-private) declarations use the `z` prefix pattern:

| Visibility | Pattern | Example |
|------------|---------|---------|
| Public | `{prefix}_name` | `jjrg_load()` |
| Internal | `z{prefix}_name` | `zjjrg_validate()` |

Internal items use `pub(crate)` visibility to enable testing from separate test files:

```rust
pub(crate) fn zjjrg_validate_silks(s: &str) -> bool {
    // Internal helper, testable from jjtg_gallops.rs
}
```

The `z` prefix signals "do not call from outside this module's conceptual boundary" even though Rust visibility permits crate-internal access.

## Test Functions

Test functions in `jjt{classifier}_{name}.rs` files use the test file's prefix:

```rust
// In jjtg_gallops.rs

use super::jjrg_gallops::*;

#[test]
fn jjtg_load_empty_file() {
    // ...
}

#[test]
fn jjtg_save_creates_parent_dirs() {
    // ...
}
```

## Visibility Layering

| Level | Visibility | Prefix | Testable |
|-------|------------|--------|----------|
| Public API | `pub` | `{prefix}_` | Yes |
| Internal | `pub(crate)` | `z{prefix}_` | Yes |
| Implementation detail | (none) | (rare) | No |

Prefer `pub(crate)` with `z` prefix over true private for anything that benefits from testing. Reserve true private for trivial implementation details.

## Minting Discipline

All naming follows the project's minting rules. Key constraints:

1. **Terminal exclusivity**: A prefix either IS a name or HAS children, never both
2. **Cipher ownership**: All identifiers derive from their project's cipher
3. **Grep-ability**: `grep jjrg_` finds all declarations from that module

Reference: CLAUDE.md "Prefix Naming Discipline" section.

## What RCG Does Not Cover

Trust Claude's Rust idioms for:

- Error handling (`Result<T, E>` patterns)
- Documentation (`///` doc comments)
- Module organization beyond naming
- Struct field design
- Lifetime annotations
- Generic patterns
- Async patterns
- Macro design

## Quick Reference

```rust
#![allow(non_camel_case_types)]

// Constants
pub const JJRG_DEFAULT_PATH: &str = ".claude/jjm/jjg_gallops.json";

// Types
pub struct jjrg_Gallops { ... }
pub enum jjrg_PaceState { ... }

// Public functions
pub fn jjrg_load(path: &Path) -> Result<jjrg_Gallops, String> { ... }

// Internal functions (pub(crate) for testability)
pub(crate) fn zjjrg_validate(s: &str) -> bool { ... }

// Impl methods - also prefixed
impl jjrg_Gallops {
    pub fn jjrg_new() -> Self { ... }
    pub fn jjrg_save(&self, path: &Path) -> Result<(), String> { ... }
    pub(crate) fn zjjrg_validate_internal(&self) -> bool { ... }
}
```

## File Templates

### Proprietary License Header

```rust
// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
```

### Open Source License Header (Apache 2.0)

```rust
// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
```

### Placement

The license header goes at the very top of the file, before `//!` doc comments or `#![...]` attributes.
