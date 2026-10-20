// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTET — the cachet's self-proofs: grant (render round-trips through
// parse), accept (a matching tree hash reads as good), and the
// tree-hash-mismatch refusal (RBSHD/RBSHO "the cachet module proven by crate
// tests"). All three run on synthetic fields — no git, no filesystem.

use crate::rbvcet_cachet::{rbvcet_parse, rbvcet_render, rbvcet_Cachet, zrbvcet_check};

fn zrbvtet_cachet(tree: &str) -> rbvcet_Cachet {
    rbvcet_Cachet {
        tree: tree.to_string(),
        tip: "abc123abc123abc123abc123abc123abc123abcd".to_string(),
        maintainer_head: "def456def456def456def456def456def456de".to_string(),
        stamp: "20260718-101112".to_string(),
    }
}

/// Grant: a rendered cachet parses back to the same fields.
#[test]
fn rbvtet_render_round_trips_through_parse() {
    let cachet = zrbvtet_cachet("treehash0000000000000000000000000000000");
    let rendered = rbvcet_render(&cachet);
    let parsed = rbvcet_parse(&rendered).expect("a freshly rendered cachet must parse");
    assert_eq!(parsed.tree, cachet.tree);
    assert_eq!(parsed.tip, cachet.tip);
    assert_eq!(parsed.maintainer_head, cachet.maintainer_head);
    assert_eq!(parsed.stamp, cachet.stamp);
}

/// Accept: a well-formed cachet with every field present parses cleanly.
#[test]
fn rbvtet_parse_accepts_well_formed_content() {
    let content = "\
RBTHDR_CACHET_TREE=treehash0000000000000000000000000000000
RBTHDR_CACHET_TIP=tipsha0000000000000000000000000000000000
RBTHDR_CACHET_MAINTAINER_HEAD=headsha00000000000000000000000000000
RBTHDR_CACHET_STAMP=20260718-101112
";
    let cachet = rbvcet_parse(content).expect("a well-formed cachet must parse");
    assert_eq!(cachet.tree, "treehash0000000000000000000000000000000");
    assert_eq!(cachet.stamp, "20260718-101112");
}

/// Refusal: a missing field, an unknown key, and a malformed line all refuse
/// — a partially-written or corrupted cachet must never be read as a partial
/// verdict.
#[test]
fn rbvtet_parse_refuses_malformed_content() {
    assert!(rbvcet_parse("").is_err(), "an empty cachet must refuse — every field is required");
    assert!(
        rbvcet_parse("RBTHDR_CACHET_TREE=abc\n").is_err(),
        "a cachet missing tip/maintainer_head/stamp must refuse"
    );
    assert!(
        rbvcet_parse("RBTHDR_CACHET_TREE=abc\nnot-a-key-value-line\n").is_err(),
        "a line with no '=' must refuse"
    );
    assert!(
        rbvcet_parse("unknown=abc\n").is_err(),
        "an unrecognized key must refuse, not be silently ignored"
    );
    assert!(
        rbvcet_parse("RBTHDR_CACHET_TREE=\n").is_err(),
        "an empty value must refuse — a blank field is not a real verdict"
    );
}

/// The tree-hash-mismatch refusal: a cachet whose recorded tree equals the
/// standing regulus's is accepted; a re-cut regulus (drifted tree) is
/// refused, by name.
#[test]
fn rbvtet_tree_hash_mismatch_refuses() {
    let cachet = zrbvtet_cachet("granted-tree-hash");

    zrbvcet_check(&cachet, "granted-tree-hash").expect("a matching tree hash must be accepted");

    let err = zrbvcet_check(&cachet, "different-tree-hash-after-recut")
        .expect_err("a drifted tree hash (a re-cut regulus) must be refused");
    assert!(err.contains("granted-tree-hash"), "the refusal must name the granted tree: {}", err);
    assert!(
        err.contains("different-tree-hash-after-recut"),
        "the refusal must name the standing tree: {}",
        err
    );
}
