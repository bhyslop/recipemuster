// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM seating validators - Tier 3 pure reads over the frozen census
//! (VOSMM-entity.adoc "Seating Validators"). Both validators fall out of the
//! seal invariants rather than running as separate passes: exact collision
//! reads the signet trie's site lists directly, terminal-exclusivity reads
//! the signet key set as a prefix trie. Mechanical only (VOr_m7w) - no word
//! generation, no fuzzy match.

use crate::vomrp_presentment::vomrp_Presentment;
use crate::vomrs_signet::vomrs_SignetTrie;

/// A finding is advisory (never an assertion) when it turns on a fallible
/// gate - today, rivet shape (VOr_m7w); sprue coverage is not yet MVP.
fn zvomrq_is_advisory(signet: &str) -> bool {
    crate::vomrv_vesture::vomrv_claim_rivet(signet)
}

/// Exact collision: a signet declared at two or more distinct sites
/// (count-blind - many sites still surface as one presentment).
pub fn vomrq_exact_collisions(trie: &vomrs_SignetTrie) -> Vec<vomrp_Presentment> {
    let mut out = Vec::new();
    for signet in trie.vomrs_signets() {
        let sites = trie.vomrs_sites(signet);
        let mut distinct: Vec<&(String, crate::vomrs_signet::vomrs_Site)> = Vec::new();
        for entry in sites {
            if !distinct.iter().any(|(_, s)| *s == entry.1) {
                distinct.push(entry);
            }
        }
        if distinct.len() < 2 {
            continue;
        }
        out.push(vomrp_Presentment {
            inscriptions: vec![signet.to_string()],
            sites: distinct.iter().map(|(_, s)| s.clone()).collect(),
            detail: format!(
                "signet `{signet}` declared at {} distinct sites",
                distinct.len()
            ),
            rule: "CLAUDE.md 'Word Selection Discipline': semantic uniqueness repo-wide, one meaning per word, ever",
            advisory: zvomrq_is_advisory(signet),
        });
    }
    out
}

/// Terminal-exclusivity: a signet that is both seated (terminal) and a
/// strict prefix of at least one other seated signet (has children) - a
/// prefix either IS a name or HAS children, never both.
pub fn vomrq_terminal_exclusivity(trie: &vomrs_SignetTrie) -> Vec<vomrp_Presentment> {
    let mut out = Vec::new();
    let signets: Vec<&str> = trie.vomrs_signets().collect();
    for signet in &signets {
        let children: Vec<&str> = signets
            .iter()
            .filter(|other| other.len() > signet.len() && other.starts_with(signet))
            .copied()
            .collect();
        if children.is_empty() {
            continue;
        }
        let sites: Vec<crate::vomrs_signet::vomrs_Site> = trie
            .vomrs_sites(signet)
            .iter()
            .map(|(_, s)| s.clone())
            .collect();
        out.push(vomrp_Presentment {
            inscriptions: vec![(*signet).to_string()],
            sites,
            detail: format!(
                "signet `{signet}` is seated and also has {} child signet(s): {}",
                children.len(),
                children.join(", ")
            ),
            rule: "CLAUDE.md 'Prefix Naming Discipline' Rule 2 - Terminal Exclusivity: a prefix either IS a name or HAS children, never both",
            advisory: zvomrq_is_advisory(signet),
        });
    }
    out
}

// eof
