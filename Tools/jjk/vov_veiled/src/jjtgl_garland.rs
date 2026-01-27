// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_garland command
//!
//! Tests the garland operation which celebrates heat completion and creates
//! a continuation heat.

use super::jjro_ops::jjrg_garland;
use super::jjrt_types::{jjrg_PaceState, jjrg_HeatStatus, jjrg_Tack, jjrg_Pace, jjrg_Heat, jjrg_Gallops, jjrg_GarlandArgs, JJRG_UNKNOWN_BASIS};
use super::jjrq_query::{jjrq_parse_silks_sequence, jjrq_build_garlanded_silks, jjrq_build_continuation_silks};
use std::collections::BTreeMap;
use indexmap::IndexMap;

// Helper to create a minimal valid Gallops structure
// Sets next_heat_seed to "AC" so that manually-created "AB" heat doesn't conflict
fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AC".to_string(),
        heats: IndexMap::new(),
    }
}

// Helper to create a valid Tack
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

// Helper to create a valid Heat with multiple paces
fn make_heat_with_paces(heat_id: &str, silks: &str, pace_states: Vec<jjrg_PaceState>) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();

    for (idx, state) in pace_states.iter().enumerate() {
        let pace_key = format!("₢{}AA{}", heat_id, char::from(b'A' + idx as u8));
        let pace_silks = format!("pace-{}", idx);
        let pace = jjrg_Pace {
            tacks: vec![make_valid_tack(state.clone(), &pace_silks, None)],
        };
        paces.insert(pace_key.clone(), pace);
        order.push(pace_key);
    }

    // Calculate next seed
    let next_char = char::from(b'A' + pace_states.len() as u8);
    let next_pace_seed = format!("AA{}", next_char);

    let heat = jjrg_Heat {
        silks: silks.to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order,
        next_pace_seed,
        paddock_file: format!(".claude/jjm/jjp_{}.md", heat_id),
        paces,
    };
    (heat_key, heat)
}

#[test]
fn jjtgl_garland_heat_not_found() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_not_found");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let args = jjrg_GarlandArgs {
        firemark: "CD".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_no_actionable_paces() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_no_actionable");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with only complete and abandoned paces
    let (heat_key, heat) = make_heat_with_paces(
        "AB",
        "test-heat",
        vec![jjrg_PaceState::Complete, jjrg_PaceState::Abandoned],
    );

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Test paddock content").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no actionable paces"));

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_successful_with_rough_paces() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_success_rough");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with mixed paces: 2 complete, 2 rough
    let (heat_key, heat) = make_heat_with_paces(
        "AB",
        "test-heat",
        vec![
            jjrg_PaceState::Complete,
            jjrg_PaceState::Complete,
            jjrg_PaceState::Rough,
            jjrg_PaceState::Rough,
        ],
    );

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Original paddock content\n\nMore details here.").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir).unwrap();

    // Verify result structure
    assert_eq!(result.old_firemark, "₣AB");
    assert_eq!(result.old_silks, "garlanded-test-heat-01");
    assert!(result.new_firemark.starts_with('₣'));
    assert_eq!(result.new_silks, "test-heat-02");
    assert_eq!(result.paces_transferred, 2);
    assert_eq!(result.paces_retained, 2);

    // Verify old heat was updated (same firemark, but silks/status changed)
    let old_heat = gallops.heats.get("₣AB").unwrap();
    assert_eq!(old_heat.silks, "garlanded-test-heat-01");
    assert_eq!(old_heat.status, jjrg_HeatStatus::Stabled);
    // After drafting to new heat, old heat retains only complete/abandoned paces
    assert_eq!(old_heat.paces.len(), 2); // Only retained paces

    // Verify new heat was created
    let new_heat = gallops.heats.get(&result.new_firemark).unwrap();
    assert_eq!(new_heat.silks, "test-heat-02");
    assert_eq!(new_heat.status, jjrg_HeatStatus::Racing);
    assert_eq!(new_heat.paces.len(), 2); // Only transferred paces

    // Verify new heat is at front of heats map
    let first_key = gallops.heats.keys().next().unwrap();
    assert_eq!(first_key, &result.new_firemark);

    // Verify paddock marker was added to old heat
    let old_paddock_content = std::fs::read_to_string(temp_dir.join(&old_heat.paddock_file)).unwrap();
    assert!(old_paddock_content.contains("Garlanded at pace 2"));

    // Verify paddock content was copied to new heat (without marker)
    let new_paddock_content = std::fs::read_to_string(temp_dir.join(&new_heat.paddock_file)).unwrap();
    assert!(new_paddock_content.contains("Original paddock content"));
    assert!(!new_paddock_content.contains("Garlanded at pace"));

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_successful_with_bridled_paces() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_success_bridled");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with complete and bridled paces
    let heat_key = "₣AB".to_string();
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();

    // Complete pace
    let pace1_key = "₢ABAAA".to_string();
    paces.insert(pace1_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "pace-0", None)],
    });
    order.push(pace1_key.clone());

    // Bridled pace (should be transferred)
    let pace2_key = "₢ABAAB".to_string();
    paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(
            jjrg_PaceState::Bridled,
            "pace-1",
            Some("Execute autonomously".to_string()),
        )],
    });
    order.push(pace2_key.clone());

    let heat = jjrg_Heat {
        silks: "test-heat".to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order,
        next_pace_seed: "AAC".to_string(),
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paces,
    };

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Paddock content").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir).unwrap();

    // Verify bridled pace was transferred
    assert_eq!(result.paces_transferred, 1);
    assert_eq!(result.paces_retained, 1);

    // Verify new heat has the bridled pace
    let new_heat = gallops.heats.get(&result.new_firemark).unwrap();
    assert_eq!(new_heat.paces.len(), 1);
    let new_pace = new_heat.paces.values().next().unwrap();
    assert_eq!(new_pace.tacks[0].state, jjrg_PaceState::Bridled);
    assert!(new_pace.tacks[0].direction.is_some());

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_silks_sequence_parsing_plain() {
    // Test that plain silks get sequence 1
    let (base, seq) = jjrq_parse_silks_sequence("foo-bar");
    assert_eq!(base, "foo-bar");
    assert_eq!(seq, None);

    let garlanded = jjrq_build_garlanded_silks(&base, seq.unwrap_or(1));
    assert_eq!(garlanded, "garlanded-foo-bar-01");

    let continuation = jjrq_build_continuation_silks(&base, seq.unwrap_or(1) + 1);
    assert_eq!(continuation, "foo-bar-02");
}

#[test]
fn jjtgl_garland_silks_sequence_parsing_numbered() {
    // Test that silks with sequence get incremented
    let (base, seq) = jjrq_parse_silks_sequence("foo-bar-03");
    assert_eq!(base, "foo-bar");
    assert_eq!(seq, Some(3));

    let garlanded = jjrq_build_garlanded_silks(&base, seq.unwrap());
    assert_eq!(garlanded, "garlanded-foo-bar-03");

    let continuation = jjrq_build_continuation_silks(&base, seq.unwrap() + 1);
    assert_eq!(continuation, "foo-bar-04");
}

#[test]
fn jjtgl_garland_preserves_pace_order() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_order");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with specific order: complete, rough, rough, complete
    let (heat_key, heat) = make_heat_with_paces(
        "AB",
        "test-heat",
        vec![
            jjrg_PaceState::Complete,
            jjrg_PaceState::Rough,
            jjrg_PaceState::Rough,
            jjrg_PaceState::Complete,
        ],
    );

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Paddock").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir).unwrap();

    // Verify new heat has rough paces in original order
    let new_heat = gallops.heats.get(&result.new_firemark).unwrap();
    assert_eq!(new_heat.order.len(), 2);

    // The two rough paces should maintain their relative order
    // (they were at positions 1 and 2 in original order)
    let new_pace_0 = new_heat.paces.get(&new_heat.order[0]).unwrap();
    let new_pace_1 = new_heat.paces.get(&new_heat.order[1]).unwrap();
    assert_eq!(new_pace_0.tacks[0].silks, "pace-1"); // From original position 1
    assert_eq!(new_pace_1.tacks[0].silks, "pace-2"); // From original position 2

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_all_actionable_paces() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_all_actionable");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with only rough paces
    let (heat_key, heat) = make_heat_with_paces(
        "AB",
        "test-heat",
        vec![jjrg_PaceState::Rough, jjrg_PaceState::Rough, jjrg_PaceState::Rough],
    );

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Paddock").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir).unwrap();

    // All paces transferred
    assert_eq!(result.paces_transferred, 3);
    assert_eq!(result.paces_retained, 0);

    // Old heat should be empty
    let old_heat = gallops.heats.get(&result.old_firemark).unwrap();
    assert_eq!(old_heat.paces.len(), 0);
    assert_eq!(old_heat.order.len(), 0);

    // New heat should have all paces
    let new_heat = gallops.heats.get(&result.new_firemark).unwrap();
    assert_eq!(new_heat.paces.len(), 3);

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_abandoned_paces_not_transferred() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_abandoned");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with abandoned and rough paces
    let (heat_key, heat) = make_heat_with_paces(
        "AB",
        "test-heat",
        vec![
            jjrg_PaceState::Abandoned,
            jjrg_PaceState::Rough,
            jjrg_PaceState::Abandoned,
        ],
    );

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Paddock").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir).unwrap();

    // Only rough pace transferred
    assert_eq!(result.paces_transferred, 1);
    assert_eq!(result.paces_retained, 2); // Two abandoned

    // Verify old heat retains abandoned paces
    let old_heat = gallops.heats.get(&result.old_firemark).unwrap();
    assert_eq!(old_heat.paces.len(), 2);
    for pace in old_heat.paces.values() {
        assert_eq!(pace.tacks[0].state, jjrg_PaceState::Abandoned);
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtgl_garland_complete_count_in_marker() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_garland_marker");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create heat with 3 complete paces and 1 rough
    let (heat_key, heat) = make_heat_with_paces(
        "AB",
        "test-heat",
        vec![
            jjrg_PaceState::Complete,
            jjrg_PaceState::Complete,
            jjrg_PaceState::Complete,
            jjrg_PaceState::Rough,
        ],
    );

    // Create paddock file
    let paddock_path = temp_dir.join(&heat.paddock_file);
    std::fs::create_dir_all(paddock_path.parent().unwrap()).unwrap();
    std::fs::write(&paddock_path, "Original content").unwrap();

    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_GarlandArgs {
        firemark: "AB".to_string(),
    };

    let result = jjrg_garland(&mut gallops, args, &temp_dir).unwrap();

    // Check paddock marker shows count of 3 complete paces
    let old_heat = gallops.heats.get(&result.old_firemark).unwrap();
    let paddock_content = std::fs::read_to_string(temp_dir.join(&old_heat.paddock_file)).unwrap();
    assert!(paddock_content.contains("Garlanded at pace 3 — magnificent service"));

    let _ = std::fs::remove_dir_all(&temp_dir);
}
