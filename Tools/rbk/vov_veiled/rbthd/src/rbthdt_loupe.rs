// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
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
// The census is synthetic, so these hold in any tree — no filesystem, no git.

use std::collections::BTreeSet;

use crate::rbthdr_loupe::{
    zrbthdr_names_token,
    zrbthdr_veil_scan_text,
    zrbthdr_veil_self_proof,
    zrbthdr_Finding,
};

/// A synthetic one-document census: the withheld-basename half of the matcher is
/// proved against this, never against the live tree.
fn zrbthdt_census() -> BTreeSet<String> {
    ["ZZQ-Example.adoc".to_string()].into_iter().collect()
}

fn zrbthdt_scan(probe: &str, census: &BTreeSet<String>) -> Vec<zrbthdr_Finding> {
    let mut hits = Vec::new();
    zrbthdr_veil_scan_text("probe", probe, census, &mut hits);
    hits
}

// ── The content matcher ─────────────────────────────────────

/// Lines that name a withheld thing — the veiled-dir token, or the census
/// document's basename in a citation form. Each must produce at least one hit.
const ZRBTHDT_PLANTED: &[&str] = &[
    "  - see Tools/rbk/vov_veiled/whatever.sh for the rule",
    "# Contract: ZZQ-Example.adoc.",
    "- **ZZQ**  → `zzk/vov_veiled/ZZQ-Example.adoc` (a maintainer-context row)",
];

/// Benign lines a coarser scanner might redden on: a shipping README mention, a
/// prose line with no withheld name, and a same-stem non-document extension.
const ZRBTHDT_CLEAN: &[&str] = &[
    "start with the README.md at the project root",
    "the terrier records which citizens hold which mantles",
    "ZZQ-Example.txt is not a withheld document",
];

#[test]
fn rbthdt_veil_scan_catches_planted_leaks() {
    let census = zrbthdt_census();
    for planted in ZRBTHDT_PLANTED {
        let hits = zrbthdt_scan(planted, &census);
        assert!(
            !hits.is_empty(),
            "the veil matcher missed a planted leak: {:?} — it would ride a candidate",
            planted
        );
    }
}

#[test]
fn rbthdt_veil_scan_silent_on_clean_lines() {
    let census = zrbthdt_census();
    for clean in ZRBTHDT_CLEAN {
        let hits = zrbthdt_scan(clean, &census);
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
fn rbthdt_veil_self_proof_holds() {
    let findings = zrbthdr_veil_self_proof();
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
fn rbthdt_hostname_names_token_word_boundary() {
    assert!(zrbthdr_names_token("host is falcon today", "falcon"), "a whole-word match is a leak");
    assert!(zrbthdr_names_token("falcon", "falcon"), "the bare token is a leak");
    assert!(zrbthdr_names_token("BURN_HOST=falcon", "falcon"), "a value after = is a leak");
    assert!(
        !zrbthdr_names_token("the falconry mews stands empty", "falcon"),
        "a substring of a longer word is not a leak"
    );
    assert!(
        !zrbthdr_names_token("gyrfalcon circles", "falcon"),
        "a trailing substring of a longer word is not a leak"
    );
}
