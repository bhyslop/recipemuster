// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM builder - the mutable Tier 1 accumulator (VOSMM-entity.adoc "Census
//! Lifecycle" raise/seat methods). Owns the walk -> tokenize ->
//! classify-by-subtraction -> seat pipeline; vomrb_seal consumes it into the
//! frozen vomrm_Matricula. No vesture claims a declaration yet, so every
//! ours-cipher token subtracts straight to the estray set (VOSMM "Classify by
//! Subtraction") - the signet trie stands ready for the first vesture.

use std::collections::BTreeSet;
use std::path::Path;

use crate::vomrm_matricula::vomrm_Matricula;
use crate::vomrs_signet::vomrs_SignetTrie;

/// The mutable Builder a scan owns; consumed by `vomrb_seal` into an immutable
/// census (VOSMM "raise").
pub struct vomrb_Builder {
    signet_trie: vomrs_SignetTrie,
    estrays: BTreeSet<String>,
}

impl vomrb_Builder {
    /// Construct an empty Builder over the fixed grammar (VOSMM "raise").
    pub fn vomrb_raise() -> Self {
        Self {
            signet_trie: vomrs_SignetTrie::vomrs_raise(),
            estrays: BTreeSet::new(),
        }
    }

    /// Walk the candidate corpus and classify-by-subtraction (VOSMM "seat").
    pub fn vomrb_seat(&mut self, repo_root: &Path) -> Result<(), String> {
        let tracked = vof::vofr_git_tracked_files(repo_root)?;

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

            for token in zvomrb_tokenize(&content) {
                if zvomrb_is_ours_token(token) {
                    self.estrays.insert(token.to_string());
                }
            }
        }

        Ok(())
    }

    /// Consume the Builder, returning the immutable census (VOSMM "seal").
    /// No method on the returned census takes `&mut self`, so this is the
    /// only path back into `vomrb_Builder`'s state - mutation after seal is
    /// unrepresentable in the type.
    pub fn vomrb_seal(self) -> vomrm_Matricula {
        vomrm_Matricula::zvomrm_from_parts(self.signet_trie, self.estrays)
    }
}

pub(crate) fn zvomrb_tokenize(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
        .filter(|tok| !tok.is_empty())
        .filter(|tok| tok.contains('_') || tok.contains('-'))
        .filter(|tok| tok.chars().next().is_some_and(|c| c.is_ascii_alphabetic()))
}

pub(crate) fn zvomrb_is_ours_token(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    vof::ALL_CIPHERS.iter().any(|c| lower.starts_with(c.prefix()))
}

// eof
