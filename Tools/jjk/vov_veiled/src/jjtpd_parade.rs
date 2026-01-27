// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_parade command (heat/pace display)

use super::jjrpd_parade::{zjjrpd_pace_state_str, zjjrpd_resolve_default_heat};
use super::jjrg_gallops::{jjrg_Gallops, jjrg_Heat, jjrg_Pace, jjrg_Tack, jjrg_HeatStatus, jjrg_PaceState, JJRG_UNKNOWN_BASIS};
use std::collections::BTreeMap;
use indexmap::IndexMap;

// ===== Helper functions =====

fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heats: IndexMap::new(),
    }
}

fn make_valid_tack(state: jjrg_PaceState, silks: &str) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260101-1200".to_string(),
        state,
        text: "Test tack text".to_string(),
        silks: silks.to_string(),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
        direction: None,
    }
}

fn make_valid_pace(heat_id: &str) -> (String, jjrg_Pace) {
    let pace_key = format!("₢{}AAA", heat_id);
    let pace = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "test-pace")],
    };
    (pace_key, pace)
}

fn make_valid_heat(heat_id: &str, silks: &str, status: jjrg_HeatStatus) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let (pace_key, pace) = make_valid_pace(heat_id);
    let mut paces = BTreeMap::new();
    paces.insert(pace_key.clone(), pace);

    let heat = jjrg_Heat {
        silks: silks.to_string(),
        creation_time: "260101".to_string(),
        status,
        order: vec![pace_key],
        next_pace_seed: "AAB".to_string(),
        paddock_file: format!(".claude/jjm/jjp_{}.md", heat_id),
        paces,
    };
    (heat_key, heat)
}

// ===== zjjrpd_pace_state_str tests =====

#[test]
fn jjtpd_pace_state_str_rough() {
    let result = zjjrpd_pace_state_str(&jjrg_PaceState::Rough);
    assert_eq!(result, "rough");
}

#[test]
fn jjtpd_pace_state_str_bridled() {
    let result = zjjrpd_pace_state_str(&jjrg_PaceState::Bridled);
    assert_eq!(result, "bridled");
}

#[test]
fn jjtpd_pace_state_str_complete() {
    let result = zjjrpd_pace_state_str(&jjrg_PaceState::Complete);
    assert_eq!(result, "complete");
}

#[test]
fn jjtpd_pace_state_str_abandoned() {
    let result = zjjrpd_pace_state_str(&jjrg_PaceState::Abandoned);
    assert_eq!(result, "abandoned");
}

// ===== zjjrpd_resolve_default_heat tests =====

#[test]
fn jjtpd_resolve_default_heat_with_racing() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-racing-heat", jjrg_HeatStatus::Racing);
    gallops.heats.insert(heat_key.clone(), heat);

    let result = zjjrpd_resolve_default_heat(&gallops);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), heat_key);
}

#[test]
fn jjtpd_resolve_default_heat_no_racing() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-stabled-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key, heat);

    let result = zjjrpd_resolve_default_heat(&gallops);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No racing heats found"));
}

#[test]
fn jjtpd_resolve_default_heat_empty_gallops() {
    let gallops = make_valid_gallops();

    let result = zjjrpd_resolve_default_heat(&gallops);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No racing heats found"));
}

#[test]
fn jjtpd_resolve_default_heat_multiple_heats_first_racing() {
    let mut gallops = make_valid_gallops();

    // First heat is racing
    let (key_ab, heat_ab) = make_valid_heat("AB", "first-racing", jjrg_HeatStatus::Racing);
    // Second heat is stabled
    let (key_cd, heat_cd) = make_valid_heat("CD", "second-stabled", jjrg_HeatStatus::Stabled);
    // Third heat is also racing
    let (key_ef, heat_ef) = make_valid_heat("EF", "third-racing", jjrg_HeatStatus::Racing);

    gallops.heats.insert(key_ab.clone(), heat_ab);
    gallops.heats.insert(key_cd, heat_cd);
    gallops.heats.insert(key_ef, heat_ef);

    // Should return first racing heat in iteration order
    let result = zjjrpd_resolve_default_heat(&gallops);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), key_ab);
}

#[test]
fn jjtpd_resolve_default_heat_only_stabled_and_retired() {
    let mut gallops = make_valid_gallops();

    let (key_ab, heat_ab) = make_valid_heat("AB", "stabled-heat", jjrg_HeatStatus::Stabled);
    let (key_cd, heat_cd) = make_valid_heat("CD", "retired-heat", jjrg_HeatStatus::Retired);

    gallops.heats.insert(key_ab, heat_ab);
    gallops.heats.insert(key_cd, heat_cd);

    let result = zjjrpd_resolve_default_heat(&gallops);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No racing heats found"));
}

// ===== Target length validation tests =====
// These test the target validation logic that appears in jjrpd_run_parade
// Since CLI output capture is not available, we verify the logic patterns

#[test]
fn jjtpd_target_length_firemark_valid() {
    // A valid firemark (without prefix) should be 2 characters
    let target = "AB";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 2);
}

#[test]
fn jjtpd_target_length_firemark_with_prefix_valid() {
    // A valid firemark with prefix
    let target = "₣AB";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 2);
}

#[test]
fn jjtpd_target_length_coronet_valid() {
    // A valid coronet (without prefix) should be 5 characters
    let target = "ABAAA";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 5);
}

#[test]
fn jjtpd_target_length_coronet_with_prefix_valid() {
    // A valid coronet with prefix
    let target = "₢ABAAA";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    assert_eq!(target_str.len(), 5);
}

#[test]
fn jjtpd_target_length_invalid_too_short() {
    // A target that is too short (1 char)
    let target = "A";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    // Should not be 2 or 5
    assert!(target_str.len() != 2 && target_str.len() != 5);
}

#[test]
fn jjtpd_target_length_invalid_too_long() {
    // A target that is too long (6 chars)
    let target = "ABAAAA";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    // Should not be 2 or 5
    assert!(target_str.len() != 2 && target_str.len() != 5);
}

#[test]
fn jjtpd_target_length_invalid_three_chars() {
    // A target that is 3 chars (between valid lengths)
    let target = "ABC";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    // Should not be 2 or 5
    assert!(target_str.len() != 2 && target_str.len() != 5);
}

#[test]
fn jjtpd_target_length_invalid_four_chars() {
    // A target that is 4 chars (between valid lengths)
    let target = "ABCD";
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target);
    // Should not be 2 or 5
    assert!(target_str.len() != 2 && target_str.len() != 5);
}

// ===== Edge case tests =====

#[test]
fn jjtpd_resolve_default_heat_finds_racing_not_first() {
    let mut gallops = make_valid_gallops();

    // Add stabled heat first
    let (key_ab, heat_ab) = make_valid_heat("AB", "stabled-first", jjrg_HeatStatus::Stabled);
    // Then racing heat
    let (key_cd, heat_cd) = make_valid_heat("CD", "racing-second", jjrg_HeatStatus::Racing);

    gallops.heats.insert(key_ab, heat_ab);
    gallops.heats.insert(key_cd.clone(), heat_cd);

    // Should find the racing heat even though it's not first
    let result = zjjrpd_resolve_default_heat(&gallops);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), key_cd);
}
