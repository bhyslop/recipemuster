// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM matricula - the frozen Tier 2 census (VOSMM-entity.adoc "Census
//! Lifecycle" seal/render methods, quoin vosmm_matricula). Immutable once
//! sealed: no method here takes `&mut self`, so mutation after seal is
//! unrepresentable in the type - `vomrb_Builder::vomrb_seal` consumes the
//! Builder to produce this struct, and nothing hands ownership back.

use std::collections::BTreeSet;

use crate::vomrs_signet::vomrs_SignetTrie;

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

/// The immutable census (VOSMM "seal") - the signet trie doubling as the
/// query index, plus the estray set surfaced as a product.
pub struct vomrm_Matricula {
    signet_trie: vomrs_SignetTrie,
    estrays: BTreeSet<String>,
}

impl vomrm_Matricula {
    /// Assembled only by `vomrb_Builder::vomrb_seal` - never constructed loose.
    pub(crate) fn zvomrm_from_parts(
        signet_trie: vomrs_SignetTrie,
        estrays: BTreeSet<String>,
    ) -> Self {
        Self {
            signet_trie,
            estrays,
        }
    }

    /// The estray set - ours-but-unclassified tokens, surfaced as a product.
    pub fn vomrm_estrays(&self) -> &BTreeSet<String> {
        &self.estrays
    }

    /// The signet trie - every in-use signet seated by a vesture claim.
    pub fn vomrm_signet_trie(&self) -> &vomrs_SignetTrie {
        &self.signet_trie
    }

    /// Render the operator-facing estray section, pure over the frozen
    /// census (VOSMM "render"). One estray per line, newline-terminated.
    pub fn vomrm_render(&self) -> String {
        let mut out = String::new();
        for token in &self.estrays {
            out.push_str(token);
            out.push('\n');
        }
        out
    }
}

// eof
