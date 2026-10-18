// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDT — the veil assay's self-proofs, ported from the retired theurge loupe
// fixture when the content-grain leak detection was absorbed into the hierophant.
// They, not a green lap, are what prove the matcher.
//
//   SCAN     the content matcher catches a planted veiled-tree mention and a
//            planted withheld-document basename, and stays silent on benign
//            lines. A matcher that cannot catch a planted leak cannot be trusted
//            to catch a real one; one that reddens on a clean line gets waved
//            through in a week.
//
//   PROOF    the self-proof the live-tree verdict rests on runs green against
//            its own synthetic census — the guarantee the assay's finding is a
//            leak in the tree, not a broken matcher.
//
//   BOUNDARY the hostname word-boundary test: a machine name is a leak only when
//            it stands as its own token, never as a substring of a longer,
//            unrelated identifier.
//
//   CENSUS   the hostname needle set is non-empty after exemptions, and every
//            needle in it is one the matcher can actually fire on. This stands in
//            for the empty-census finding the scan carried while its needles were
//            harvested live: a literal cannot silently stop extracting, but it CAN
//            be emptied by edit, or exempted away to nothing, and either would
//            leave the scan passing every tree vacuously.
//
// The veil census is synthetic, so those tests hold in any tree — no filesystem,
// no git. The hostname census is the real literal: it is the thing under test.

use std::collections::BTreeSet;

use crate::rbvceu_loupe::{
    zrbvceu_names_token,
    zrbvceu_veil_scan_text,
    zrbvceu_veil_self_proof,
    zrbvceu_Finding,
    ZRBVCEU_HOST_CENSUS,
    ZRBVCEU_HOST_EXEMPT,
};

/// A synthetic one-document census: the withheld-basename half of the matcher is
/// proved against this, never against the live tree.
fn zrbvteu_census() -> BTreeSet<String> {
    ["ZZQ-Example.adoc".to_string()].into_iter().collect()
}

fn zrbvteu_scan(probe: &str, census: &BTreeSet<String>) -> Vec<zrbvceu_Finding> {
    let mut hits = Vec::new();
    zrbvceu_veil_scan_text("probe", probe, census, &mut hits);
    hits
}

// ── The content matcher ─────────────────────────────────────

/// Lines that name a withheld thing — the veiled-dir token, or the census
/// document's basename in a citation form. Each must produce at least one hit.
const ZRBVTEU_SCAN_PLANTED: &[&str] = &[
    "  - see vov_veiled/whatever.sh for the rule",
    "# Contract: ZZQ-Example.adoc.",
    "- **ZZQ**  → `zzk/vov_veiled/ZZQ-Example.adoc` (a maintainer-context row)",
];

/// Benign lines a coarser scanner might redden on: a shipping README mention, a
/// prose line with no withheld name, and a same-stem non-document extension.
const ZRBVTEU_SCAN_CLEAN: &[&str] = &[
    "start with the README.md at the project root",
    "the terrier records which citizens hold which mantles",
    "ZZQ-Example.txt is not a withheld document",
];

#[test]
fn rbvteu_veil_scan_catches_planted_leaks() {
    let census = zrbvteu_census();
    for planted in ZRBVTEU_SCAN_PLANTED {
        let hits = zrbvteu_scan(planted, &census);
        assert!(
            !hits.is_empty(),
            "the veil matcher missed a planted leak: {:?} — it would ride a regulus",
            planted
        );
    }
}

#[test]
fn rbvteu_veil_scan_silent_on_clean_lines() {
    let census = zrbvteu_census();
    for clean in ZRBVTEU_SCAN_CLEAN {
        let hits = zrbvteu_scan(clean, &census);
        assert!(
            hits.is_empty(),
            "the veil matcher reddened on a benign line {:?}: {:?} — a matcher that cries wolf gets disabled",
            clean,
            hits.iter().map(|f| f.detail.clone()).collect::<Vec<_>>()
        );
    }
}

// ── The self-proof ──────────────────────────────────────────

/// The self-proof the live-tree verdict rests on must run clean: an empty result
/// means the matcher catches every planted positive and reddens on no negative.
#[test]
fn rbvteu_veil_self_proof_holds() {
    let findings = zrbvceu_veil_self_proof();
    assert!(
        findings.is_empty(),
        "the veil matcher self-proof failed — the live-tree verdict cannot be trusted: {:?}",
        findings.iter().map(|f| f.detail.clone()).collect::<Vec<_>>()
    );
}

// ── The hostname word boundary ──────────────────────────────

/// A machine name is a leak only as its own token — never as a substring of a
/// longer, unrelated identifier.
#[test]
fn rbvteu_hostname_census_is_live() {
    let effective: Vec<&str> = ZRBVCEU_HOST_CENSUS
        .iter()
        .copied()
        .filter(|name| !ZRBVCEU_HOST_EXEMPT.contains(name))
        .collect();

    assert!(
        !effective.is_empty(),
        "the hostname census is empty after exemptions — the scan would pass every tree vacuously"
    );

    // A needle the matcher cannot fire on is a needle that protects nothing: an
    // empty or whitespace-bearing entry would sit in the list looking like cover.
    for name in &effective {
        assert!(
            zrbvceu_names_token(&format!("host is {} today", name), name),
            "census entry {:?} is not a token the matcher can catch",
            name
        );
    }
}

#[test]
fn rbvteu_hostname_names_token_word_boundary() {
    assert!(zrbvceu_names_token("host is falcon today", "falcon"), "a whole-word match is a leak");
    assert!(zrbvceu_names_token("falcon", "falcon"), "the bare token is a leak");
    assert!(zrbvceu_names_token("BURN_HOST=falcon", "falcon"), "a value after = is a leak");
    assert!(
        !zrbvceu_names_token("the falconry mews stands empty", "falcon"),
        "a substring of a longer word is not a leak"
    );
    assert!(
        !zrbvceu_names_token("gyrfalcon circles", "falcon"),
        "a trailing substring of a longer word is not a leak"
    );
}
