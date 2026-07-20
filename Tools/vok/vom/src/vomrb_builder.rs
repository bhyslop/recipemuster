// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM builder - the mutable Tier 1 accumulator (VOSMM-entity.adoc "Census
//! Lifecycle" raise/seat methods). Owns the walk -> tokenize ->
//! classify-by-subtraction -> seat pipeline; vomrb_seal consumes it into the
//! frozen vomrm_Matricula. Each vesture in vomrv_vesture claims its
//! declaration sites first (line-by-line, seated into the signet trie); an
//! ours-cipher token from the wide tokenize net becomes an estray only if no
//! vesture claimed it (VOSMM "Classify by Subtraction").

use std::collections::BTreeSet;
use std::path::Path;

use crate::vomrm_matricula::vomrm_Matricula;
use crate::vomrs_signet::{vomrs_Site, vomrs_SignetTrie};

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
        let mut corpus: Vec<(String, String)> = Vec::new();

        for rel_path in &tracked {
            if vof::vofr_is_veiled_path(rel_path) {
                continue;
            }
            if !crate::vomra_allowlist::voma_is_allowed(rel_path) {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(repo_root.join(rel_path)) else {
                continue;
            };
            let Some(path_str) = rel_path.to_str() else {
                continue;
            };
            corpus.push((path_str.to_string(), content));
        }

        self.vomrb_seat_corpus(&corpus);
        Ok(())
    }

    /// Classify-by-subtraction over an already-gathered (path, content)
    /// corpus, decoupled from the git/filesystem walk so seating validators
    /// can be exercised against planted fixture corpora (VOSMM "Seating
    /// Validators").
    pub fn vomrb_seat_corpus(&mut self, corpus: &[(String, String)]) {
        let mut claimed: BTreeSet<String> = BTreeSet::new();

        for (path, content) in corpus {
            // A tracked file's own basename is itself a declaration site: the
            // vesture's `envelope` field (VOS0 Liturgy) makes the file the
            // inscription, so the stem is claimed the same as any in-content
            // declaration. Multi-dot basenames (tabtarget-style
            // `colophon.Frontispiece.imprint.sh`) are that vesture's concern,
            // not this bare-stem claim - skipped rather than mis-claimed.
            if let Some(stem) = Path::new(path).file_stem().and_then(|s| s.to_str()) {
                if !stem.contains('.') && zvomrb_is_ours_token(stem) {
                    self.signet_trie
                        .vomrs_seat(stem, stem, vomrs_Site::vomrs_new(path.clone(), 0));
                    claimed.insert(stem.to_string());
                }
            }

            let dress = crate::vomrv_vesture::vomrv_dress(path);
            for (line_no, line) in content.lines().enumerate() {
                for (signet, inscription) in crate::vomrv_vesture::vomrv_claim_line(&dress, line) {
                    // The ours-or-foreign gate is the project cipher (VOSMM
                    // "Classify by Subtraction") - it bounds the census
                    // universe at seating exactly as it bounds the estray
                    // net below and the file-stem claim above; a foreign
                    // declaration (`TOKEN=`, a vendor doc's `to=`) is
                    // outside the mint universe entirely.
                    if !zvomrb_is_ours_token(&signet) {
                        continue;
                    }
                    self.signet_trie.vomrs_seat(
                        &signet,
                        &inscription,
                        vomrs_Site::vomrs_new(path.clone(), line_no + 1),
                    );
                    claimed.insert(inscription);
                }
            }
        }

        for (path, content) in corpus {
            for (line_no, line) in content.lines().enumerate() {
                for token in zvomrb_tokenize(line) {
                    if !zvomrb_is_ours_token(token) || claimed.contains(token) {
                        continue;
                    }
                    if crate::vomrv_vesture::vomrv_claim_rivet(token) {
                        self.signet_trie.vomrs_seat(
                            token,
                            token,
                            vomrs_Site::vomrs_new(path.clone(), line_no + 1),
                        );
                        claimed.insert(token.to_string());
                        continue;
                    }
                    self.estrays.insert(token.to_string());
                }
            }
        }
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
