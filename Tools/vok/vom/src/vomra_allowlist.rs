// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM allowlist - Tier 0 grammar member: the file-selection allowlist.
//!
//! Schema-compatible with the tackle table's claims shape (JJSVT-tackle.adoc
//! jjottn_claims: anchored subtree x shape glob conjunction) — the MVP bounds
//! are the whole git-tracked tree (no subtree anchor), so only the shape half
//! is carried here. Homed VOK-side per VOSMM-entity.adoc Tier 0, pending
//! migration to the pedigree home at studbook founding.
//!
//! Carries the one file-role exclusion VOSMM names outright (Scan Mechanics:
//! "Memos are the first reference-only case — excluded from the MVP
//! declaration scan"); the fuller declaration-bearing/reference-only/
//! index-of-record/generated layer stays deferred to the tackle-table
//! projection.

use std::path::Path;

/// Allowlisted file-name shapes, recursive over the whole candidate tree.
pub const VOMA_ALLOWLIST: &[&str] = &["*.md", "*.adoc", "*.rs", "*.sh"];

/// Reference-only path prefixes excluded from the MVP declaration scan.
/// `.claude/jjm/` (heat blotters, chat archives, gallops record) and
/// `Study/` (scratch investigations) are historical prose in the same
/// reference-only sense as Memos: records ABOUT names, never declarations
/// of them. Deliberately NOT all of `.claude/` - `.claude/commands/` is a
/// live minted namespace (CLAUDE.md Extended Namespace Checklist: command
/// files `.claude/commands/{cmd}.md`), so its basenames stay in the census.
pub const VOMA_REFERENCE_ONLY: &[&str] = &["Memos/", ".claude/jjm/", "Study/"];

/// Check whether a path's shape is allowlisted.
pub fn voma_is_allowed(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    let Some(path_str) = path.to_str() else {
        return false;
    };
    if VOMA_REFERENCE_ONLY.iter().any(|prefix| path_str.starts_with(prefix)) {
        return false;
    }
    VOMA_ALLOWLIST
        .iter()
        .any(|pattern| name.ends_with(pattern.trim_start_matches('*')))
}

// eof
