// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrs_signet.

use super::vomrs_signet::*;

#[test]
fn vomts_raise_yields_empty_trie() {
    let trie = vomrs_SignetTrie::vomrs_raise();
    assert_eq!(trie.vomrs_len(), 0);
}

#[test]
fn vomts_seat_then_get_roundtrips() {
    let mut trie = vomrs_SignetTrie::vomrs_raise();
    trie.vomrs_seat(
        "rbga",
        "rbga_registry.sh",
        vomrs_Site::vomrs_new("rbga_registry.sh", 0),
    );
    assert_eq!(trie.vomrs_get("rbga"), Some("rbga_registry.sh"));
    assert_eq!(trie.vomrs_len(), 1);
}

#[test]
fn vomts_seat_twice_accumulates_sites_not_len() {
    let mut trie = vomrs_SignetTrie::vomrs_raise();
    trie.vomrs_seat("dup", "dup", vomrs_Site::vomrs_new("a.rs", 1));
    trie.vomrs_seat("dup", "dup", vomrs_Site::vomrs_new("b.rs", 1));
    assert_eq!(trie.vomrs_len(), 1);
    assert_eq!(trie.vomrs_sites("dup").len(), 2);
}

#[test]
fn vomts_get_missing_signet_is_none() {
    let trie = vomrs_SignetTrie::vomrs_raise();
    assert_eq!(trie.vomrs_get("absent"), None);
}

// eof
