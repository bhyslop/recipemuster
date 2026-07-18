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

use std::path::Path;

/// Allowlisted file-name shapes, recursive over the whole candidate tree.
pub const VOMA_ALLOWLIST: &[&str] = &["*.md", "*.adoc", "*.rs", "*.sh"];

/// Check whether a path's shape is allowlisted.
pub fn voma_is_allowed(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    VOMA_ALLOWLIST
        .iter()
        .any(|pattern| name.ends_with(pattern.trim_start_matches('*')))
}

// eof
