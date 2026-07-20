// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM signet trie - Tier 2 field: every in-use signet seated by the census
//! (VOSMM-entity.adoc "Census Lifecycle", quoin vosls_signet). A
//! vesture-claimed inscription is seated here by its signet with its
//! declaration site; the trie doubles as the query index the frozen
//! Matricula reads, and the seating validators (VOSMM "Seating Validators")
//! read the site lists and the signet key set directly - no separate pass.

use std::collections::BTreeMap;

/// One declaration site: the file a signet was claimed in, and the line
/// (1-based; 0 for a whole-file claim, e.g. a bare file-stem envelope).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct vomrs_Site {
    pub file: String,
    pub line: usize,
}

impl vomrs_Site {
    pub fn vomrs_new(file: impl Into<String>, line: usize) -> Self {
        Self {
            file: file.into(),
            line,
        }
    }
}

impl std::fmt::Display for vomrs_Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line == 0 {
            write!(f, "{}", self.file)
        } else {
            write!(f, "{}:{}", self.file, self.line)
        }
    }
}

/// Every signet seated so far, each mapping to the inscription-and-site pairs
/// declaring it (count-blind consumers dedupe by site; the exact-collision
/// validator reads the site count directly, VOSMM "Seating Validators").
pub struct vomrs_SignetTrie {
    seated: BTreeMap<String, Vec<(String, vomrs_Site)>>,
}

impl vomrs_SignetTrie {
    /// Construct an empty trie.
    pub fn vomrs_raise() -> Self {
        Self {
            seated: BTreeMap::new(),
        }
    }

    /// Seat a claimed inscription at its signet, recording the declaration site.
    pub fn vomrs_seat(&mut self, signet: &str, inscription: &str, site: vomrs_Site) {
        self.seated
            .entry(signet.to_string())
            .or_default()
            .push((inscription.to_string(), site));
    }

    /// Look up the first inscription seated at a signet, if any.
    pub fn vomrs_get(&self, signet: &str) -> Option<&str> {
        self.seated
            .get(signet)
            .and_then(|sites| sites.first())
            .map(|(inscription, _)| inscription.as_str())
    }

    /// Every declaration (inscription, site) seated at a signet.
    pub fn vomrs_sites(&self, signet: &str) -> &[(String, vomrs_Site)] {
        self.seated.get(signet).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Every seated signet, in trie (sorted) order - the key set the seating
    /// validators walk.
    pub fn vomrs_signets(&self) -> impl Iterator<Item = &str> {
        self.seated.keys().map(String::as_str)
    }

    /// Count of distinct signets seated.
    pub fn vomrs_len(&self) -> usize {
        self.seated.len()
    }
}

// eof
