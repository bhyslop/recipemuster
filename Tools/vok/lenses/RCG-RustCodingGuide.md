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

### Struct Field Visibility for Tests

When tests construct internal structs directly, both the struct AND its fields need `pub(crate)` visibility:

```rust
// ❌ Tests can't construct this — fields are private
pub(crate) struct zjjrg_Output {
    heat_silks: String,
    pace_count: Option<u32>,
}

// ✅ Tests can construct this — fields are accessible
pub(crate) struct zjjrg_Output {
    pub(crate) heat_silks: String,
    pub(crate) pace_count: Option<u32>,
}
```

If tests only READ internal structs (returned by functions), field visibility doesn't matter. But if tests CONSTRUCT them for unit testing serialization, validation, or helper logic, fields must be `pub(crate)`.

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

## Test File Extraction Checklist

When extracting inline tests from `{cipher}r{x}_{name}.rs` to `{cipher}t{x}_{name}.rs`:

### Source File Changes
- [ ] Remove entire `#[cfg(test)] mod tests { ... }` block
- [ ] Change internal functions from `fn` to `pub(crate) fn` if tests call them
- [ ] Change internal structs from `struct` to `pub(crate) struct` if tests construct them
- [ ] Add `pub(crate)` to struct fields if tests construct the struct

### New Test File
- [ ] File named `{cipher}t{x}_{name}.rs` (matching source classifier)
- [ ] Copyright header copied from source
- [ ] Import: `use super::{cipher}r{x}_{name}::*;`
- [ ] Additional imports for types from sibling modules as needed
- [ ] Test functions renamed from `test_*` to `{cipher}t{x}_*`
- [ ] Each test function has `#[test]` attribute
- [ ] NO `#[cfg(test)]` wrapper — lib.rs handles that
- [ ] NO `mod tests` wrapper — functions go directly in file
- [ ] Helper functions (e.g., `create_test_data()`) can remain unprefixed

### lib.rs Wiring
- [ ] Add `#[cfg(test)] mod {cipher}t{x}_{name};` declaration
- [ ] Place after corresponding `pub mod {cipher}r{x}_{name};`

### Verification
- [ ] `cargo build` succeeds
- [ ] `cargo test` runs all extracted tests
- [ ] Test count matches pre-extraction count

## Module Maturity Checklist

### File Structure
- [ ] Source file: `{cipher}r{x}_{name}.rs`
- [ ] Test file: `{cipher}t{x}_{name}.rs` (if tests exist)
- [ ] `lib.rs` has `#![allow(non_camel_case_types)]`
- [ ] `lib.rs` declares source module: `pub mod {cipher}r{x}_{name};`
- [ ] `lib.rs` declares test module: `#[cfg(test)] mod {cipher}t{x}_{name};`

### Naming Conventions
- [ ] All public structs/enums: `{prefix}_PascalName`
- [ ] All public functions: `{prefix}_snake_name`
- [ ] All public constants: `{PREFIX}_SCREAMING`
- [ ] All impl methods: `{prefix}_method_name`
- [ ] All internal items: `z{prefix}_name` with `pub(crate)`
- [ ] All test functions: `{cipher}t{x}_test_name`

### Visibility Discipline
- [ ] Public API uses `pub`
- [ ] Internal helpers use `pub(crate)` with `z` prefix
- [ ] Structs constructed by tests have `pub(crate)` fields
- [ ] True private reserved for trivial implementation details

### Test Organization
- [ ] Tests in separate `{cipher}t{x}_*.rs` files, not inline
- [ ] Test file imports source module with `use super::*`
- [ ] Test functions prefixed with test file's prefix
- [ ] Helper functions local to test file (no prefix needed)
