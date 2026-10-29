// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTEW — the world election's self-proofs (RBSHC "The election"): the two
// bundles share no address and no basename, so no practice run can reach a
// real target through a shared value; and the argument parser selects each
// world by its word and refuses a bare invocation, an unknown word, the
// struck mode token and a trailing extra argument. The world words are taken
// from the breviary, never spelled.

use crate::rbvcew_world::{
    rbvcew_bundle, rbvcew_elect, rbvcew_word, rbvcew_Bundle, rbvcew_World, RBVCEW_AGORA, RBVCEW_PALAESTRA,
};

/// The struck mode token docimasy and ostend once took, assembled so the
/// crate spells it nowhere: it must now refuse like any unknown word.
const ZRBVTEW_STRUCK_TOKEN: &str = concat!("re", "hearse");

/// The verb the parser is told it serves; any of the four would do.
const ZRBVTEW_VERB: &str = crate::rbvgeb_breviary::RBVGEB_ESSAI_BASE;

/// Every value a bundle carries, address and basename alike.
fn zrbvtew_values(bundle: &rbvcew_Bundle) -> Vec<&'static str> {
    vec![
        bundle.base_remote,
        bundle.base_push_disabled,
        bundle.base_url,
        bundle.narthex_url,
        bundle.narthex_https,
        bundle.harbinger_url,
        bundle.regulus_dirname,
        bundle.freshness_dirname,
        bundle.temenos_dirname,
    ]
}

fn zrbvtew_args(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| w.to_string()).collect()
}

/// The bundles share no address and no basename: no value of one world equals
/// any value of the other, and none of the practice world's addresses names
/// the real repository by containing one of the real world's values.
#[test]
fn rbvtew_bundles_share_no_address_and_no_basename() {
    let real = zrbvtew_values(&RBVCEW_AGORA);
    let practice = zrbvtew_values(&RBVCEW_PALAESTRA);
    for r in &real {
        assert!(!r.is_empty(), "a real-world entry is empty");
        for p in &practice {
            assert!(!p.is_empty(), "a practice-world entry is empty");
            assert_ne!(r, p, "the two worlds share the entry '{}'", r);
        }
    }
    for p in &practice {
        for r in [RBVCEW_AGORA.base_url, RBVCEW_AGORA.narthex_url, RBVCEW_AGORA.narthex_https, RBVCEW_AGORA.harbinger_url] {
            assert!(!p.contains(r), "the practice entry '{}' carries the real address '{}'", p, r);
        }
    }
}

/// Each world's push sentinel spells its own remote, so a remote neutered with
/// the other world's sentinel is refused.
#[test]
fn rbvtew_each_sentinel_spells_its_own_remote() {
    for bundle in [&RBVCEW_AGORA, &RBVCEW_PALAESTRA] {
        assert!(
            bundle.base_push_disabled.contains(bundle.base_remote),
            "sentinel '{}' does not name '{}'",
            bundle.base_push_disabled,
            bundle.base_remote
        );
        assert_ne!(bundle.base_push_disabled, bundle.base_url, "a sentinel must never be a pushable URL");
    }
    assert!(!RBVCEW_PALAESTRA.base_push_disabled.contains(RBVCEW_AGORA.base_remote));
    assert!(!RBVCEW_AGORA.base_push_disabled.contains(RBVCEW_PALAESTRA.base_remote));
}

/// Each word selects its own world, and the selection reaches its own bundle.
#[test]
fn rbvtew_each_word_selects_its_world() {
    for world in [rbvcew_World::Agora, rbvcew_World::Palaestra] {
        let elected = rbvcew_elect(ZRBVTEW_VERB, &zrbvtew_args(&[rbvcew_word(world)]))
            .expect("a world's own word must elect it");
        assert_eq!(elected, world);
    }
    let agora = rbvcew_elect(ZRBVTEW_VERB, &zrbvtew_args(&[rbvcew_word(rbvcew_World::Agora)])).unwrap();
    assert_eq!(rbvcew_bundle(agora).base_url, RBVCEW_AGORA.base_url);
    let palaestra = rbvcew_elect(ZRBVTEW_VERB, &zrbvtew_args(&[rbvcew_word(rbvcew_World::Palaestra)])).unwrap();
    assert_eq!(rbvcew_bundle(palaestra).base_url, RBVCEW_PALAESTRA.base_url);
}

/// A bare invocation refuses: there is no default world.
#[test]
fn rbvtew_bare_invocation_refuses() {
    assert!(rbvcew_elect(ZRBVTEW_VERB, &[]).is_err(), "a bare invocation must refuse");
}

/// An unknown word refuses — a near miss, a case variant, an empty word, and
/// the struck mode token alike.
#[test]
fn rbvtew_unknown_word_refuses() {
    let agora = rbvcew_word(rbvcew_World::Agora);
    let near = format!("{}s", agora);
    let shouted = agora.to_uppercase();
    for word in [near.as_str(), shouted.as_str(), "", "main", ZRBVTEW_STRUCK_TOKEN] {
        assert!(
            rbvcew_elect(ZRBVTEW_VERB, &zrbvtew_args(&[word])).is_err(),
            "the word '{}' must refuse",
            word
        );
    }
}

/// A trailing extra argument refuses, even after a good world, and the struck
/// mode token is no exception.
#[test]
fn rbvtew_extra_argument_refuses() {
    for world in [rbvcew_World::Agora, rbvcew_World::Palaestra] {
        let word = rbvcew_word(world);
        assert!(rbvcew_elect(ZRBVTEW_VERB, &zrbvtew_args(&[word, ZRBVTEW_STRUCK_TOKEN])).is_err());
        assert!(rbvcew_elect(ZRBVTEW_VERB, &zrbvtew_args(&[word, word])).is_err());
    }
    assert!(rbvcew_elect(
        ZRBVTEW_VERB,
        &zrbvtew_args(&[rbvcew_word(rbvcew_World::Agora), rbvcew_word(rbvcew_World::Palaestra)])
    )
    .is_err());
}
