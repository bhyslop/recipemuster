// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrm_matricula.

use super::vomrm_matricula::*;

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
fn vomtm_tokenize_casts_widest_underscore_hyphen_net() {
    let text = "See vofc_registry.rs and the rbw-tb tabtarget, plus plain prose.";
    let tokens: Vec<&str> = zvomrm_tokenize(text).collect();
    assert_eq!(tokens, vec!["vofc_registry", "rbw-tb"]);
}

#[test]
fn vomtm_tokenize_excludes_plain_words_and_punct_runs() {
    let tokens: Vec<&str> = zvomrm_tokenize("plain prose --- 123-456").collect();
    assert!(!tokens.contains(&"plain"));
    assert!(!tokens.contains(&"---"));
    assert!(!tokens.contains(&"123-456"));
}

#[test]
fn vomtm_is_ours_token_matches_known_ciphers_case_insensitively() {
    assert!(zvomrm_is_ours_token("vofc_registry"));
    assert!(zvomrm_is_ours_token("rbw-tb"));
    assert!(zvomrm_is_ours_token("VOr_k3p"));
    assert!(zvomrm_is_ours_token("RBr_a3f"));
    assert!(!zvomrm_is_ours_token("well-known"));
}

// eof
