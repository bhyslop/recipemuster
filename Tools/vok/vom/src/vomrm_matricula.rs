// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM runtime - the matricula machinery.
//!
//! Degenerate census: Tier 0 grammar (the vof cipher registry plus the vomra
//! allowlist) and the raising walk, with zero vestures claiming anything —
//! every ours-shaped token lands as an estray. The signet trie, seal, and the
//! seating validators (VOSMM-entity.adoc Tier 1/2) populate this module in
//! later paces.

use std::collections::BTreeSet;
use std::path::Path;

/// The crate's identity line, incorporating the foundation cipher's project
/// resolved through the vof path-dependency. Returned for callers and tests;
/// this function emits nothing (emission is the caller's concern, via vomrl_*!).
pub fn vomrm_identity() -> String {
    format!(
        "vom {} - Vox Matricula (foundation cipher: {})",
        env!("CARGO_PKG_VERSION"),
        vof::VO.project(),
    )
}

/// Raise the degenerate estray census: walk the candidate corpus (git-tracked
/// files intersected with the allowlist, veiled paths excluded), tokenize on
/// the widest `_`/`-` net, and classify every ours-shaped token as an estray
/// (no vestures exist yet to claim anything).
pub fn vomrm_raise_estrays(repo_root: &Path) -> Result<BTreeSet<String>, String> {
    let tracked = vof::vofr_git_tracked_files(repo_root)?;
    let mut estrays = BTreeSet::new();

    for rel_path in tracked {
        if vof::vofr_is_veiled_path(&rel_path) {
            continue;
        }
        if !crate::vomra_allowlist::voma_is_allowed(&rel_path) {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(repo_root.join(&rel_path)) else {
            continue;
        };

        for token in zvomrm_tokenize(&content) {
            if zvomrm_is_ours_token(token) {
                estrays.insert(token.to_string());
            }
        }
    }

    Ok(estrays)
}

/// Tokenize on the widest `_`- and `-`-shaped net: maximal runs of
/// alphanumeric/underscore/hyphen characters that contain at least one `_`
/// or `-` and start with a letter (excludes bare punctuation runs and
/// numeric-leading fragments split out of prose).
pub(crate) fn zvomrm_tokenize(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
        .filter(|tok| !tok.is_empty())
        .filter(|tok| tok.contains('_') || tok.contains('-'))
        .filter(|tok| tok.chars().next().is_some_and(|c| c.is_ascii_alphabetic()))
}

/// Ours-or-foreign gate: does this token carry a registered cipher, per
/// VOr_m7w (mechanical, never linguistic — case-normalized so both
/// `vofc_registry` and `VOr_k3p`-shaped rivets match the same lowercase
/// prefix).
pub(crate) fn zvomrm_is_ours_token(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    vof::ALL_CIPHERS.iter().any(|c| lower.starts_with(c.prefix()))
}

// eof
