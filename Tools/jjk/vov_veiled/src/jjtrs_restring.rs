// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_restring command (bulk pace transfer between heats)

use super::jjrg_gallops::*;
use super::jjro_ops::jjrg_restring;
use indexmap::IndexMap;

// ===== Helper functions (following jjtg_gallops.rs patterns) =====

fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heats: IndexMap::new(),
    }
}

fn make_valid_tack(state: jjrg_PaceState, silks: &str, direction: Option<String>) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260101-1200".to_string(),
        state,
        text: "Test tack text".to_string(),
        silks: silks.to_string(),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
        direction,
    }
}

fn make_heat_with_paces(heat_id: &str, silks: &str, pace_count: usize) -> (String, jjrg_Heat) {
    use std::collections::BTreeMap;

    let heat_key = format!("₣{}", heat_id);
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();

    for i in 0..pace_count {
        // Generate coronet: AAA, AAB, AAC, etc.
        let pace_suffix = format!("AA{}", (b'A' + i as u8) as char);
        let pace_key = format!("₢{}{}", heat_id, pace_suffix);
        let pace_silks = format!("pace-{}", i + 1);

        let pace = jjrg_Pace {
            tacks: vec![make_valid_tack(jjrg_PaceState::Rough, &pace_silks, None)],
        };

        order.push(pace_key.clone());
        paces.insert(pace_key, pace);
    }

    let next_pace_suffix = format!("AA{}", (b'A' + pace_count as u8) as char);

    let heat = jjrg_Heat {
        silks: silks.to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order,
        next_pace_seed: next_pace_suffix,
        paddock_file: format!(".claude/jjm/jjp_{}.md", heat_id),
        paces,
    };

    (heat_key, heat)
}

// ===== Validation tests (error cases) =====

#[test]
fn jjtrs_restring_same_heat_error() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_heat_with_paces("AB", "source-heat", 2);
    let pace1 = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AB".to_string(),  // Same heat!
        coronets: vec![pace1],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot draft paces to same heat"));
}

#[test]
fn jjtrs_restring_source_heat_not_found() {
    let mut gallops = make_valid_gallops();
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),  // Doesn't exist
        dest_firemark: "AC".to_string(),
        coronets: vec!["₢ABAAA".to_string()],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Heat '₣AB' not found"));
}

#[test]
fn jjtrs_restring_dest_heat_not_found() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let pace1 = source_heat.order[0].clone();
    gallops.heats.insert(source_key, source_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "CD".to_string(),  // Doesn't exist
        coronets: vec![pace1],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Heat '₣CD' not found"));
}

#[test]
fn jjtrs_restring_empty_coronets_error() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);
    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![],  // Empty!
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No paces specified for draft"));
}

#[test]
fn jjtrs_restring_coronet_not_in_source_heat() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);
    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec!["₢ABXXX".to_string()],  // Doesn't exist
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in heat"));
}

#[test]
fn jjtrs_restring_coronet_wrong_heat_identity() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);
    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec!["₢ACAAA".to_string()],  // Embeds AC (dest), not AB (source)
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("does not belong to source heat"), "Expected error about heat identity, got: {}", err_msg);
}

#[test]
fn jjtrs_restring_invalid_coronet_format() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);
    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec!["invalid!coronet".to_string()],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid coronet"));
}

// ===== Success tests (happy path) =====

#[test]
fn jjtrs_restring_single_pace_success() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 3);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 1);

    let pace_to_move = source_heat.order[1].clone();  // Move second pace

    gallops.heats.insert(source_key.clone(), source_heat);
    gallops.heats.insert(dest_key.clone(), dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace_to_move.clone()],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();

    // Check result structure
    assert_eq!(result.source_firemark, "₣AB");
    assert_eq!(result.dest_firemark, "₣AC");
    assert_eq!(result.drafted.len(), 1);
    assert_eq!(result.drafted[0].old_coronet, pace_to_move);

    // Check source heat now has 2 paces (started with 3, moved 1)
    let source_heat = gallops.heats.get(&source_key).unwrap();
    assert_eq!(source_heat.paces.len(), 2);
    assert_eq!(source_heat.order.len(), 2);
    assert!(!source_heat.paces.contains_key(&pace_to_move));
    assert_eq!(result.source_empty_after, false);

    // Check dest heat now has 2 paces (started with 1, added 1)
    let dest_heat = gallops.heats.get(&dest_key).unwrap();
    assert_eq!(dest_heat.paces.len(), 2);
    assert_eq!(dest_heat.order.len(), 2);

    // Verify new coronet embeds dest heat identity
    let new_coronet = &result.drafted[0].new_coronet;
    assert!(new_coronet.starts_with("₢AC"));
}

#[test]
fn jjtrs_restring_multiple_paces_preserves_order() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 5);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);

    // Move paces 1, 2, and 4 (indices 0, 1, 3)
    let pace1 = source_heat.order[0].clone();
    let pace2 = source_heat.order[1].clone();
    let pace4 = source_heat.order[3].clone();

    gallops.heats.insert(source_key.clone(), source_heat);
    gallops.heats.insert(dest_key.clone(), dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace1.clone(), pace2.clone(), pace4.clone()],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.drafted.len(), 3);

    // Verify order is preserved in dest heat
    let dest_heat = gallops.heats.get(&dest_key).unwrap();
    assert_eq!(dest_heat.order.len(), 3);

    // The new coronets should be in the same order as the input
    assert_eq!(result.drafted[0].old_coronet, pace1);
    assert_eq!(result.drafted[1].old_coronet, pace2);
    assert_eq!(result.drafted[2].old_coronet, pace4);

    // Source should have 2 remaining paces
    let source_heat = gallops.heats.get(&source_key).unwrap();
    assert_eq!(source_heat.paces.len(), 2);
    assert_eq!(result.source_empty_after, false);
}

#[test]
fn jjtrs_restring_all_paces_empties_source() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 1);

    let pace1 = source_heat.order[0].clone();
    let pace2 = source_heat.order[1].clone();

    gallops.heats.insert(source_key.clone(), source_heat);
    gallops.heats.insert(dest_key.clone(), dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace1, pace2],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();

    // Source should be empty
    assert_eq!(result.source_empty_after, true);
    let source_heat = gallops.heats.get(&source_key).unwrap();
    assert_eq!(source_heat.paces.len(), 0);
    assert_eq!(source_heat.order.len(), 0);

    // Dest should have 3 paces
    let dest_heat = gallops.heats.get(&dest_key).unwrap();
    assert_eq!(dest_heat.paces.len(), 3);
}

#[test]
fn jjtrs_restring_accepts_coronets_without_prefix() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);

    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    // Use coronet without prefix (should be normalized by parser)
    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec!["ABAAA".to_string()],  // No ₢ prefix, just the 5-char coronet
    };

    let result = jjrg_restring(&mut gallops, args);
    if result.is_err() {
        panic!("Expected Ok, got Err: {}", result.unwrap_err());
    }

    let result = result.unwrap();
    assert_eq!(result.drafted.len(), 1);
    assert_eq!(result.drafted[0].old_coronet, "₢ABAAA");
}

#[test]
fn jjtrs_restring_preserves_pace_state_and_direction() {
    let mut gallops = make_valid_gallops();
    let (source_key, mut source_heat) = make_heat_with_paces("AB", "source-heat", 1);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);

    // Set the pace to bridled with direction
    let pace_key = source_heat.order[0].clone();
    source_heat.paces.get_mut(&pace_key).unwrap().tacks[0].state = jjrg_PaceState::Bridled;
    source_heat.paces.get_mut(&pace_key).unwrap().tacks[0].direction = Some("Execute autonomously".to_string());

    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key.clone(), dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace_key],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result.drafted[0].state, jjrg_PaceState::Bridled);

    // Verify the transferred pace retains bridled state and direction
    let dest_heat = gallops.heats.get(&dest_key).unwrap();
    let new_coronet = &result.drafted[0].new_coronet;
    let transferred_pace = dest_heat.paces.get(new_coronet).unwrap();

    // First tack is the draft note, second is the original
    assert!(transferred_pace.tacks.len() >= 2);
    let original_tack = &transferred_pace.tacks[1];
    assert_eq!(original_tack.state, jjrg_PaceState::Bridled);
    assert_eq!(original_tack.direction.as_ref().unwrap(), "Execute autonomously");
}

#[test]
fn jjtrs_restring_creates_draft_note_tack() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 1);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);

    let pace_key = source_heat.order[0].clone();

    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key.clone(), dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace_key.clone()],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();

    // Check that draft note was created
    let dest_heat = gallops.heats.get(&dest_key).unwrap();
    let new_coronet = &result.drafted[0].new_coronet;
    let transferred_pace = dest_heat.paces.get(new_coronet).unwrap();

    // Should have 2 tacks: draft note + original
    assert_eq!(transferred_pace.tacks.len(), 2);

    // First tack should contain draft note
    let draft_tack = &transferred_pace.tacks[0];
    assert!(draft_tack.text.contains("Drafted from"));
    assert!(draft_tack.text.contains(&pace_key));
    assert!(draft_tack.text.contains("₣AB"));
}

#[test]
fn jjtrs_restring_increments_dest_pace_seed() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 1);

    let original_dest_seed = dest_heat.next_pace_seed.clone();

    let pace1 = source_heat.order[0].clone();
    let pace2 = source_heat.order[1].clone();

    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key.clone(), dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace1, pace2],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    // Dest heat started with seed AAB (1 existing pace), should now be AAD (added 2)
    let dest_heat = gallops.heats.get(&dest_key).unwrap();
    assert_eq!(original_dest_seed, "AAB");
    assert_eq!(dest_heat.next_pace_seed, "AAD");
}

#[test]
fn jjtrs_restring_result_contains_correct_metadata() {
    let mut gallops = make_valid_gallops();
    let (source_key, source_heat) = make_heat_with_paces("AB", "source-heat", 2);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);

    let pace1 = source_heat.order[0].clone();

    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace1],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();

    // Verify result contains expected metadata
    assert_eq!(result.source_firemark, "₣AB");
    assert_eq!(result.source_silks, "source-heat");
    assert_eq!(result.source_paddock, ".claude/jjm/jjp_AB.md");
    assert_eq!(result.dest_firemark, "₣AC");
    assert_eq!(result.dest_silks, "dest-heat");
    assert_eq!(result.dest_paddock, ".claude/jjm/jjp_AC.md");
    assert_eq!(result.drafted.len(), 1);
}

#[test]
fn jjtrs_restring_spec_preview_truncates_long_text() {
    let mut gallops = make_valid_gallops();
    let (source_key, mut source_heat) = make_heat_with_paces("AB", "source-heat", 1);
    let (dest_key, dest_heat) = make_heat_with_paces("AC", "dest-heat", 0);

    // Create a pace with very long text
    let pace_key = source_heat.order[0].clone();
    let long_text = "a".repeat(100);  // 100 chars, should be truncated to 80
    source_heat.paces.get_mut(&pace_key).unwrap().tacks[0].text = long_text;

    gallops.heats.insert(source_key, source_heat);
    gallops.heats.insert(dest_key, dest_heat);

    let args = jjrg_RestringArgs {
        source_firemark: "AB".to_string(),
        dest_firemark: "AC".to_string(),
        coronets: vec![pace_key],
    };

    let result = jjrg_restring(&mut gallops, args);
    assert!(result.is_ok());

    let result = result.unwrap();

    // Spec preview should be truncated with ellipsis
    assert!(result.drafted[0].spec.len() == 80);  // 77 chars + "..."
    assert!(result.drafted[0].spec.ends_with("..."));
}
