// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM signet trie - Tier 2 field: every in-use signet seated by the census
//! (VOSMM-entity.adoc "Census Lifecycle", quoin vosls_signet). A
//! vesture-claimed inscription is seated here by its signet; the trie doubles
//! as the query index the frozen Matricula reads.

use std::collections::BTreeMap;

/// Every signet seated so far, mapping the signet to its inscription.
pub struct vomrs_SignetTrie {
    seated: BTreeMap<String, String>,
}

impl vomrs_SignetTrie {
    /// Construct an empty trie.
    pub fn vomrs_raise() -> Self {
        Self {
            seated: BTreeMap::new(),
        }
    }

    /// Seat a claimed inscription at its signet.
    pub fn vomrs_seat(&mut self, signet: &str, inscription: &str) {
        self.seated.insert(signet.to_string(), inscription.to_string());
    }

    /// Look up the inscription seated at a signet, if any.
    pub fn vomrs_get(&self, signet: &str) -> Option<&str> {
        self.seated.get(signet).map(String::as_str)
    }

    /// Count of signets seated.
    pub fn vomrs_len(&self) -> usize {
        self.seated.len()
    }
}

// eof
