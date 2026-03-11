// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_reorder command (rail - reorder paces within a heat)

use super::jjrg_gallops::*;
use std::collections::BTreeMap;

fn make_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        schema_version: Some(4),
        next_heat_seed: "AB".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
    }
}

fn make_tack(state: jjrg_PaceState, silks: &str) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260101-1200".to_string(),
        state,
        text: "Test tack".to_string(),
        silks: silks.to_string(),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
        direction: None,
    }
}

fn make_heat_with_paces(heat_id: &str, pace_ids: &[&str]) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();

    for &pid in pace_ids {
        let coronet = format!("₢{}{}", heat_id, pid);
        paces.insert(coronet.clone(), jjrg_Pace {
            tacks: vec![make_tack(jjrg_PaceState::Rough, "test-pace")],
        });
        order.push(coronet);
    }

    let heat = jjrg_Heat {
        silks: "test-heat".to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order,
        next_pace_seed: "AAZ".to_string(),
        paddock_file: ".claude/jjm/jjp_test.md".to_string(),
        paces,
    };
    (heat_key, heat)
}

// ===== No-op detection tests =====

#[test]
fn jjtrl_move_last_already_last_is_noop() {
    let mut gallops = make_gallops();
    let (hk, heat) = make_heat_with_paces("AB", &["AAA", "AAB", "AAC"]);
    let old_order = heat.order.clone();
    gallops.heats.insert(hk, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAC".to_string()),
        before: None,
        after: None,
        first: false,
        last: true,
    };

    let result = gallops.jjrg_rail(args).unwrap();
    assert_eq!(result, old_order, "Moving last pace to --last should return identical order");
}

#[test]
fn jjtrl_move_first_already_first_is_noop() {
    let mut gallops = make_gallops();
    let (hk, heat) = make_heat_with_paces("AB", &["AAA", "AAB", "AAC"]);
    let old_order = heat.order.clone();
    gallops.heats.insert(hk, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAA".to_string()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args).unwrap();
    assert_eq!(result, old_order, "Moving first pace to --first should return identical order");
}

#[test]
fn jjtrl_move_before_adjacent_is_noop() {
    let mut gallops = make_gallops();
    // Order: AAA, AAB, AAC
    let (hk, heat) = make_heat_with_paces("AB", &["AAA", "AAB", "AAC"]);
    let old_order = heat.order.clone();
    gallops.heats.insert(hk, heat);

    // Move AAA before AAB — AAA is already right before AAB
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAA".to_string()),
        before: Some("₢ABAAB".to_string()),
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args).unwrap();
    assert_eq!(result, old_order, "Moving pace before its immediate successor should be noop");
}

#[test]
fn jjtrl_move_after_adjacent_is_noop() {
    let mut gallops = make_gallops();
    // Order: AAA, AAB, AAC
    let (hk, heat) = make_heat_with_paces("AB", &["AAA", "AAB", "AAC"]);
    let old_order = heat.order.clone();
    gallops.heats.insert(hk, heat);

    // Move AAB after AAA — AAB is already right after AAA
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAB".to_string()),
        before: None,
        after: Some("₢ABAAA".to_string()),
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args).unwrap();
    assert_eq!(result, old_order, "Moving pace after its immediate predecessor should be noop");
}

// ===== Actual move tests =====

#[test]
fn jjtrl_move_last_to_first() {
    let mut gallops = make_gallops();
    let (hk, _heat) = make_heat_with_paces("AB", &["AAA", "AAB", "AAC"]);
    gallops.heats.insert(hk, _heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAC".to_string()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args).unwrap();
    assert_eq!(result[0], "₢ABAAC", "Last pace should now be first");
    assert_eq!(result.len(), 3);
}

#[test]
fn jjtrl_move_first_to_last() {
    let mut gallops = make_gallops();
    let (hk, _heat) = make_heat_with_paces("AB", &["AAA", "AAB", "AAC"]);
    gallops.heats.insert(hk, _heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAA".to_string()),
        before: None,
        after: None,
        first: false,
        last: true,
    };

    let result = gallops.jjrg_rail(args).unwrap();
    assert_eq!(result[2], "₢ABAAA", "First pace should now be last");
    assert_eq!(result.len(), 3);
}

// ===== Error cases =====

#[test]
fn jjtrl_heat_not_found() {
    let mut gallops = make_gallops();

    let args = jjrg_RailArgs {
        firemark: "ZZ".to_string(),
        order: vec![],
        move_coronet: Some("₢ZZAAA".to_string()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn jjtrl_pace_not_found() {
    let mut gallops = make_gallops();
    let (hk, heat) = make_heat_with_paces("AB", &["AAA"]);
    gallops.heats.insert(hk, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABZZZ".to_string()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn jjtrl_no_positioning_flag() {
    let mut gallops = make_gallops();
    let (hk, heat) = make_heat_with_paces("AB", &["AAA", "AAB"]);
    gallops.heats.insert(hk, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAA".to_string()),
        before: None,
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("positioning flag"));
}

#[test]
fn jjtrl_self_relative_error() {
    let mut gallops = make_gallops();
    let (hk, heat) = make_heat_with_paces("AB", &["AAA", "AAB"]);
    gallops.heats.insert(hk, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some("₢ABAAA".to_string()),
        before: Some("₢ABAAA".to_string()),
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("relative to itself"));
}
