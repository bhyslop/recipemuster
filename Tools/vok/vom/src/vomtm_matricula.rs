// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrm_matricula.

use super::vomrb_builder::vomrb_Builder;
use super::vomrm_matricula::*;
use super::vomrs_signet::vomrs_SignetTrie;
use std::collections::BTreeSet;

#[test]
fn vomtm_identity_names_crate_and_links_vof() {
    let identity = vomrm_identity();
    assert!(
        identity.starts_with("vom "),
        "identity line must name the crate: {identity}"
    );
    // The vof path-dependency is live: the vo cipher's project resolves.
    assert!(
        identity.contains("Vox Obscura"),
        "vof link must resolve the vo cipher project: {identity}"
    );
}

#[test]
fn vomtm_seal_freezes_an_empty_builder_into_an_empty_census() {
    let census = vomrb_Builder::vomrb_raise().vomrb_seal();
    assert!(census.vomrm_estrays().is_empty());
    assert_eq!(census.vomrm_signet_trie().vomrs_len(), 0);
    assert_eq!(census.vomrm_render(), "");
}

#[test]
fn vomtm_render_lists_each_estray_one_per_line_sorted() {
    let mut estrays = BTreeSet::new();
    estrays.insert("rbw-tb".to_string());
    estrays.insert("vofc_registry".to_string());
    let census = vomrm_Matricula::zvomrm_from_parts(vomrs_SignetTrie::vomrs_raise(), estrays);
    assert_eq!(census.vomrm_render(), "rbw-tb\nvofc_registry\n");
}

// eof
