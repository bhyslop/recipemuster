// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTET — the cachet's self-proofs: grant (render round-trips through
// parse, its world included), accept (a matching tree hash in the elected
// world reads as good), the tree-hash-mismatch refusal, and the world
// refusal in both directions (RBSHD/RBSHO "the cachet module proven by crate
// tests"; RBSHC "The election"). All run on synthetic fields — no git, no
// filesystem. Content carrying a world is built from the render, never spelled.

use crate::rbvcet_cachet::{rbvcet_parse, rbvcet_render, rbvcet_Cachet, zrbvcet_check};
use crate::rbvcew_world::rbvcew_World;

fn zrbvtet_cachet(tree: &str, world: rbvcew_World) -> rbvcet_Cachet {
    rbvcet_Cachet {
        tree: tree.to_string(),
        tip: "abc123abc123abc123abc123abc123abc123abcd".to_string(),
        maintainer_head: "def456def456def456def456def456def456de".to_string(),
        stamp: "20260718-101112".to_string(),
        world,
    }
}

/// Grant: a rendered cachet parses back to the same fields, in either world.
#[test]
fn rbvtet_render_round_trips_through_parse() {
    for world in [rbvcew_World::Agora, rbvcew_World::Palaestra] {
        let cachet = zrbvtet_cachet("treehash0000000000000000000000000000000", world);
        let rendered = rbvcet_render(&cachet);
        let parsed = rbvcet_parse(&rendered).expect("the round trip must succeed");
        assert_eq!(parsed.tree, cachet.tree);
        assert_eq!(parsed.tip, cachet.tip);
        assert_eq!(parsed.maintainer_head, cachet.maintainer_head);
        assert_eq!(parsed.stamp, cachet.stamp);
        assert_eq!(parsed.world, world, "the world must survive the round trip");
    }
}

/// Accept: a well-formed cachet with every field present parses cleanly.
#[test]
fn rbvtet_parse_accepts_well_formed_content() {
    let content = rbvcet_render(&zrbvtet_cachet("treehash0000000000000000000000000000000", rbvcew_World::Agora));
    let cachet = rbvcet_parse(&content).expect("well-formed text must be accepted");
    assert_eq!(cachet.tree, "treehash0000000000000000000000000000000");
    assert_eq!(cachet.stamp, "20260718-101112");
    assert_eq!(cachet.world, rbvcew_World::Agora);
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

/// Refusal: a cachet without its world refuses — the four older keys alone
/// are not a verdict any world may honor — and so does a world no election
/// names.
#[test]
fn rbvtet_parse_refuses_a_missing_or_unknown_world() {
    let rendered = rbvcet_render(&zrbvtet_cachet("treehash0000000000000000000000000000000", rbvcew_World::Palaestra));
    let worldless: String = rendered
        .lines()
        .filter(|line| !line.starts_with("RBTHDR_CACHET_WORLD="))
        .map(|line| format!("{}\n", line))
        .collect();
    assert_eq!(worldless.lines().count(), 4, "the strip must leave exactly the four older keys");
    assert!(rbvcet_parse(&worldless).is_err(), "text missing its world must refuse");

    let foreign = format!("{}RBTHDR_CACHET_WORLD=elsewhere\n", worldless);
    assert!(rbvcet_parse(&foreign).is_err(), "text naming no known world must refuse");

    let blank = format!("{}RBTHDR_CACHET_WORLD=\n", worldless);
    assert!(rbvcet_parse(&blank).is_err(), "text with a blank world must refuse");
}

/// The tree-hash-mismatch refusal: a cachet whose recorded tree equals the
/// standing regulus's is accepted; a re-cut regulus (drifted tree) is
/// refused, by name.
#[test]
fn rbvtet_tree_hash_mismatch_refuses() {
    let cachet = zrbvtet_cachet("granted-tree-hash", rbvcew_World::Agora);

    zrbvcet_check(&cachet, "granted-tree-hash", rbvcew_World::Agora).expect("an equal tree hash must be accepted");

    let err = zrbvcet_check(&cachet, "different-tree-hash-after-recut", rbvcew_World::Agora)
        .expect_err("a drifted tree hash (a re-cut regulus) must be refused");
    assert!(err.contains("granted-tree-hash"), "the refusal must name the granted tree: {}", err);
    assert!(
        err.contains("different-tree-hash-after-recut"),
        "the refusal must name the standing tree: {}",
        err
    );
}

/// The world refusal, at ONE tree hash for both worlds — the hazard this
/// proof exists for: a practice regulus cut from the same HEAD atop a
/// stand-in seeded at public main carries the real regulus's tree, so the
/// world is the only byte that can refuse. Each world's cachet is accepted in
/// its own world and refused in the other, naming both.
#[test]
fn rbvtet_cross_world_refuses_in_both_directions() {
    let one_tree = "the-same-tree-hash-in-both-worlds";
    let worlds = [rbvcew_World::Agora, rbvcew_World::Palaestra];
    for granted in worlds {
        let cachet = zrbvtet_cachet(one_tree, granted);
        for elected in worlds {
            let verdict = zrbvcet_check(&cachet, one_tree, elected);
            if granted == elected {
                verdict.expect("the verdict must be accepted in the world that issued it");
            } else {
                let err = verdict.expect_err("the verdict must be refused in the other world");
                let granted_word = crate::rbvcew_world::rbvcew_word(granted);
                let elected_word = crate::rbvcew_world::rbvcew_word(elected);
                assert!(err.contains(granted_word), "the refusal must name the issuing world: {}", err);
                assert!(err.contains(elected_word), "the refusal must name the elected world: {}", err);
            }
        }
    }
}
