// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrb_builder.

use super::vomrb_builder::*;

#[test]
fn vomtb_tokenize_casts_widest_underscore_hyphen_net() {
    let text = "See vofc_registry.rs and the rbw-tb tabtarget, plus plain prose.";
    let tokens: Vec<&str> = zvomrb_tokenize(text).collect();
    assert_eq!(tokens, vec!["vofc_registry", "rbw-tb"]);
}

#[test]
fn vomtb_tokenize_excludes_plain_words_and_punct_runs() {
    let tokens: Vec<&str> = zvomrb_tokenize("plain prose --- 123-456").collect();
    assert!(!tokens.contains(&"plain"));
    assert!(!tokens.contains(&"---"));
    assert!(!tokens.contains(&"123-456"));
}

#[test]
fn vomtb_is_ours_token_matches_known_ciphers_case_insensitively() {
    assert!(zvomrb_is_ours_token("vofc_registry"));
    assert!(zvomrb_is_ours_token("rbw-tb"));
    assert!(zvomrb_is_ours_token("VOr_k3p"));
    assert!(zvomrb_is_ours_token("RBr_a3f"));
    assert!(!zvomrb_is_ours_token("well-known"));
}

#[test]
fn vomtb_raise_yields_empty_builder() {
    let builder = vomrb_Builder::vomrb_raise();
    let census = builder.vomrb_seal();
    assert!(census.vomrm_estrays().is_empty());
}

// eof
