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
// RBTHDT — the cachet's self-proofs: grant (render round-trips through
// parse), accept (a matching tree hash reads as good), and the
// tree-hash-mismatch refusal (RBSHD/RBSHO "the cachet module proven by crate
// tests"). All three run on synthetic fields — no git, no filesystem.

use crate::rbthdr_cachet::{parse, render, rbthdr_Cachet, zrbthdr_check};

fn zrbthdt_cachet(tree: &str) -> rbthdr_Cachet {
    rbthdr_Cachet {
        tree: tree.to_string(),
        tip: "abc123abc123abc123abc123abc123abc123abcd".to_string(),
        maintainer_head: "def456def456def456def456def456def456de".to_string(),
        stamp: "20260718-101112".to_string(),
    }
}

/// Grant: a rendered cachet parses back to the same fields.
#[test]
fn rbthdt_render_round_trips_through_parse() {
    let cachet = zrbthdt_cachet("treehash0000000000000000000000000000000");
    let rendered = render(&cachet);
    let parsed = parse(&rendered).expect("a freshly rendered cachet must parse");
    assert_eq!(parsed.tree, cachet.tree);
    assert_eq!(parsed.tip, cachet.tip);
    assert_eq!(parsed.maintainer_head, cachet.maintainer_head);
    assert_eq!(parsed.stamp, cachet.stamp);
}

/// Accept: a well-formed cachet with every field present parses cleanly.
#[test]
fn rbthdt_parse_accepts_well_formed_content() {
    let content = "\
RBTHDR_CACHET_TREE=treehash0000000000000000000000000000000
RBTHDR_CACHET_TIP=tipsha0000000000000000000000000000000000
RBTHDR_CACHET_MAINTAINER_HEAD=headsha00000000000000000000000000000
RBTHDR_CACHET_STAMP=20260718-101112
";
    let cachet = parse(content).expect("a well-formed cachet must parse");
    assert_eq!(cachet.tree, "treehash0000000000000000000000000000000");
    assert_eq!(cachet.stamp, "20260718-101112");
}

/// Refusal: a missing field, an unknown key, and a malformed line all refuse
/// — a partially-written or corrupted cachet must never be read as a partial
/// verdict.
#[test]
fn rbthdt_parse_refuses_malformed_content() {
    assert!(parse("").is_err(), "an empty cachet must refuse — every field is required");
    assert!(
        parse("RBTHDR_CACHET_TREE=abc\n").is_err(),
        "a cachet missing tip/maintainer_head/stamp must refuse"
    );
    assert!(
        parse("RBTHDR_CACHET_TREE=abc\nnot-a-key-value-line\n").is_err(),
        "a line with no '=' must refuse"
    );
    assert!(
        parse("RBTHDR_CACHET_UNKNOWN=abc\n").is_err(),
        "an unrecognized key must refuse, not be silently ignored"
    );
    assert!(
        parse("RBTHDR_CACHET_TREE=\n").is_err(),
        "an empty value must refuse — a blank field is not a real verdict"
    );
}

/// The tree-hash-mismatch refusal: a cachet whose recorded tree equals the
/// standing candidate's is accepted; a re-cut candidate (drifted tree) is
/// refused, by name.
#[test]
fn rbthdt_tree_hash_mismatch_refuses() {
    let cachet = zrbthdt_cachet("granted-tree-hash");

    zrbthdr_check(&cachet, "granted-tree-hash").expect("a matching tree hash must be accepted");

    let err = zrbthdr_check(&cachet, "different-tree-hash-after-recut")
        .expect_err("a drifted tree hash (a re-cut candidate) must be refused");
    assert!(err.contains("granted-tree-hash"), "the refusal must name the granted tree: {}", err);
    assert!(
        err.contains("different-tree-hash-after-recut"),
        "the refusal must name the standing tree: {}",
        err
    );
}
