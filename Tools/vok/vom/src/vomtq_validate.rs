// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrq_validate - planted fixtures run through the classify pipeline
//! (vomrb_seat_corpus), each surfacing as exactly one presentment (VOSMM
//! "Seating Validators" Done-when).

use super::vomrb_builder::vomrb_Builder;

#[test]
fn vomtq_planted_collision_surfaces_exactly_one_presentment() {
    // Same signet ("voftt_thing") declared as a Rust fn in two distinct
    // fixture files - an exact collision (VOr_k3p cipher `vo` makes it ours).
    let corpus = vec![
        (
            "a.rs".to_string(),
            "pub fn voftt_thing(x: i32) -> i32 { x }".to_string(),
        ),
        (
            "b.rs".to_string(),
            "pub fn voftt_thing(y: i32) -> i32 { y }".to_string(),
        ),
    ];
    let mut builder = vomrb_Builder::vomrb_raise();
    builder.vomrb_seat_corpus(&corpus);
    let census = builder.vomrb_seal();

    let collisions = census.vomrm_exact_collisions();
    assert_eq!(collisions.len(), 1, "expected exactly one collision presentment");
    assert_eq!(collisions[0].inscriptions, vec!["voftt_thing".to_string()]);
    assert_eq!(collisions[0].sites.len(), 2);

    assert!(census.vomrm_terminal_exclusivity().is_empty());
}

#[test]
fn vomtq_planted_terminal_exclusivity_surfaces_exactly_one_presentment() {
    // "voftt_thing" is seated as its own declaration AND is a strict prefix
    // of "voftt_thing_more" - the container-vs-terminal breach.
    let corpus = vec![(
        "a.rs".to_string(),
        "pub fn voftt_thing() {}\npub fn voftt_thing_more() {}\n".to_string(),
    )];
    let mut builder = vomrb_Builder::vomrb_raise();
    builder.vomrb_seat_corpus(&corpus);
    let census = builder.vomrb_seal();

    let breaches = census.vomrm_terminal_exclusivity();
    assert_eq!(breaches.len(), 1, "expected exactly one terminal-exclusivity presentment");
    assert_eq!(breaches[0].inscriptions, vec!["voftt_thing".to_string()]);
    assert!(breaches[0].detail.contains("voftt_thing_more"));

    assert!(census.vomrm_exact_collisions().is_empty());
}

#[test]
fn vomtq_foreign_declarations_never_seat() {
    // A foreign declaration (no project cipher prefix) is outside the mint
    // universe: it must neither seat nor present, even when it collides or
    // prefixes another foreign name (VOSMM "Classify by Subtraction": the
    // ours-or-foreign gate is the project cipher).
    let corpus = vec![
        ("a.sh".to_string(), "TOKEN=x\nTOKEN_JSON=y\n".to_string()),
        ("b.sh".to_string(), "TOKEN=z\n".to_string()),
    ];
    let mut builder = vomrb_Builder::vomrb_raise();
    builder.vomrb_seat_corpus(&corpus);
    let census = builder.vomrb_seal();

    assert!(census.vomrm_exact_collisions().is_empty());
    assert!(census.vomrm_terminal_exclusivity().is_empty());
}

#[test]
fn vomtq_clean_corpus_yields_no_presentments() {
    let corpus = vec![(
        "a.rs".to_string(),
        "pub fn voftt_alone() {}\n".to_string(),
    )];
    let mut builder = vomrb_Builder::vomrb_raise();
    builder.vomrb_seat_corpus(&corpus);
    let census = builder.vomrb_seal();

    assert!(census.vomrm_exact_collisions().is_empty());
    assert!(census.vomrm_terminal_exclusivity().is_empty());
}

#[test]
fn vomtq_rivet_finding_is_advisory() {
    use super::vomrs_signet::{vomrs_Site, vomrs_SignetTrie};
    let mut trie = vomrs_SignetTrie::vomrs_raise();
    trie.vomrs_seat("VOr_a1b", "VOr_a1b", vomrs_Site::vomrs_new("a.rs", 1));
    trie.vomrs_seat("VOr_a1b", "VOr_a1b", vomrs_Site::vomrs_new("b.rs", 3));
    let presentments = super::vomrq_validate::vomrq_exact_collisions(&trie);
    assert_eq!(presentments.len(), 1);
    assert!(presentments[0].advisory);
}

// eof
