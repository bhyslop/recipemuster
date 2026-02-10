// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrg_gallops::*;
use super::jjrv_validate::{zjjrg_is_base64, zjjrg_is_kebab_case, zjjrg_is_yymmdd, zjjrg_is_yymmdd_hhmm};
use super::jjru_util::zjjrg_increment_seed;
use std::collections::BTreeMap;
use indexmap::IndexMap;

#[test]
fn jjtg_pace_state_serialization() {
    let state = jjrg_PaceState::Rough;
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(json, "\"rough\"");
}

// Helper to create a minimal valid Gallops structure
fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heats: IndexMap::new(),
    }
}

// Helper to create a valid Tack (uses JJRG_UNKNOWN_BASIS for consistent basis format)
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

// Helper to create a valid Pace
fn make_valid_pace(heat_id: &str, silks: &str) -> (String, jjrg_Pace) {
    let pace_key = format!("₢{}AAA", heat_id);
    let pace = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, silks, None)],
    };
    (pace_key, pace)
}

// Helper to create a valid Heat
fn make_valid_heat(heat_id: &str, silks: &str) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let (pace_key, pace) = make_valid_pace(heat_id, "test-pace");
    let mut paces = BTreeMap::new();
    paces.insert(pace_key.clone(), pace);

    let heat = jjrg_Heat {
        silks: silks.to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec![pace_key],
        next_pace_seed: "AAB".to_string(),
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paces,
    };
    (heat_key, heat)
}

// ===== Validation helper tests =====

#[test]
fn jjtg_zjjrg_is_base64_valid() {
    assert!(zjjrg_is_base64("AB"));
    assert!(zjjrg_is_base64("ABCDE"));
    assert!(zjjrg_is_base64("Az09-_"));
}

#[test]
fn jjtg_zjjrg_is_base64_invalid() {
    assert!(!zjjrg_is_base64("A!"));
    assert!(!zjjrg_is_base64("A B"));
    assert!(!zjjrg_is_base64("+/"));
}

#[test]
fn jjtg_zjjrg_is_kebab_case_valid() {
    assert!(zjjrg_is_kebab_case("test"));
    assert!(zjjrg_is_kebab_case("test-pace"));
    assert!(zjjrg_is_kebab_case("my-cool-heat123"));
    assert!(zjjrg_is_kebab_case("a1-b2-c3"));
    assert!(zjjrg_is_kebab_case("Test"));
    assert!(zjjrg_is_kebab_case("MyHeat-Name"));
    assert!(zjjrg_is_kebab_case("ALLCAPS"));
    assert!(zjjrg_is_kebab_case("mixedCase-123"));
}

#[test]
fn jjtg_zjjrg_is_kebab_case_invalid() {
    assert!(!zjjrg_is_kebab_case(""));
    assert!(!zjjrg_is_kebab_case("test_pace"));
    assert!(!zjjrg_is_kebab_case("-test"));
    assert!(!zjjrg_is_kebab_case("test-"));
    assert!(!zjjrg_is_kebab_case("test--pace"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_valid() {
    assert!(zjjrg_is_yymmdd("260101"));
    assert!(zjjrg_is_yymmdd("991231"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_invalid() {
    assert!(!zjjrg_is_yymmdd("2601"));
    assert!(!zjjrg_is_yymmdd("26010101"));
    assert!(!zjjrg_is_yymmdd("26-01-01"));
    assert!(!zjjrg_is_yymmdd("26ab01"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_hhmm_valid() {
    assert!(zjjrg_is_yymmdd_hhmm("260101-1234"));
    assert!(zjjrg_is_yymmdd_hhmm("991231-2359"));
}

#[test]
fn jjtg_zjjrg_is_yymmdd_hhmm_invalid() {
    assert!(!zjjrg_is_yymmdd_hhmm("260101"));
    assert!(!zjjrg_is_yymmdd_hhmm("260101-123"));
    assert!(!zjjrg_is_yymmdd_hhmm("260101-12345"));
    assert!(!zjjrg_is_yymmdd_hhmm("26010112:34"));
}

// ===== Gallops validation tests =====

#[test]
fn jjtg_validate_minimal_valid_gallops() {
    let gallops = make_valid_gallops();
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_gallops_with_heat() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_invalid_next_heat_seed_length() {
    let mut gallops = make_valid_gallops();
    gallops.next_heat_seed = "ABC".to_string();
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("next_heat_seed must be 2 characters")));
}

#[test]
fn jjtg_validate_invalid_next_heat_seed_chars() {
    let mut gallops = make_valid_gallops();
    gallops.next_heat_seed = "A!".to_string();
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("invalid base64 characters")));
}

#[test]
fn jjtg_validate_heat_key_missing_prefix() {
    let mut gallops = make_valid_gallops();
    let (_, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert("AB".to_string(), heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("key must start with '₣'")));
}

#[test]
fn jjtg_validate_heat_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    heat.silks = "Invalid_Silks".to_string();
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("silks must be non-empty alphanumeric-kebab")));
}

#[test]
fn jjtg_validate_heat_invalid_creation_time() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    heat.creation_time = "2026-01-01".to_string();
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("creation_time must be YYMMDD format")));
}

#[test]
fn jjtg_validate_heat_invalid_next_pace_seed() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    heat.next_pace_seed = "AB".to_string(); // Should be 3 chars
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("next_pace_seed must be 3 characters")));
}

#[test]
fn jjtg_validate_order_paces_mismatch() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    // Add extra entry to order that doesn't exist in paces
    heat.order.push("₢ABXXX".to_string());
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("order contains keys not in paces")));
}

#[test]
fn jjtg_validate_pace_key_wrong_heat_identity() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    // Replace pace with one that embeds wrong heat identity
    heat.paces.clear();
    heat.order.clear();
    let bad_pace_key = "₢CDAAA".to_string(); // CD instead of AB
    let pace = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "bad-pace", None)],
    };
    heat.paces.insert(bad_pace_key.clone(), pace);
    heat.order.push(bad_pace_key);
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("must embed parent heat identity")));
}

#[test]
fn jjtg_validate_tack_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    // Get the first pace's first tack and modify its silks
    if let Some(pace) = heat.paces.values_mut().next() {
        if let Some(tack) = pace.tacks.first_mut() {
            tack.silks = "".to_string();
        }
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("silks must be non-empty alphanumeric-kebab")));
}

#[test]
fn jjtg_validate_pace_empty_tacks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks.clear();
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("tacks array must not be empty")));
}

#[test]
fn jjtg_validate_tack_invalid_ts() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].ts = "invalid".to_string();
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("ts must be YYMMDD-HHMM format")));
}

#[test]
fn jjtg_validate_tack_empty_text() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].text = "".to_string();
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("text must not be empty")));
}

#[test]
fn jjtg_validate_bridled_without_direction() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Bridled;
        pace.tacks[0].direction = None;
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("direction is required when state is 'bridled'")));
}

#[test]
fn jjtg_validate_bridled_with_empty_direction() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Bridled;
        pace.tacks[0].direction = Some("".to_string());
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("direction must not be empty when state is 'bridled'")));
}

#[test]
fn jjtg_validate_bridled_with_direction_valid() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Bridled;
        pace.tacks[0].direction = Some("Execute autonomously".to_string());
    }
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_non_bridled_with_direction() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Rough;
        pace.tacks[0].direction = Some("Should not be here".to_string());
    }
    gallops.heats.insert(heat_key, heat);
    let errors = gallops.jjrg_validate().unwrap_err();
    assert!(errors.iter().any(|e| e.contains("direction must be absent when state is not 'bridled'")));
}

#[test]
fn jjtg_validate_complete_state_valid() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Complete;
        pace.tacks[0].direction = None;
    }
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

#[test]
fn jjtg_validate_abandoned_state_valid() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
    if let Some(pace) = heat.paces.values_mut().next() {
        pace.tacks[0].state = jjrg_PaceState::Abandoned;
        pace.tacks[0].direction = None;
    }
    gallops.heats.insert(heat_key, heat);
    assert!(gallops.jjrg_validate().is_ok());
}

// ===== Load/Save round-trip tests =====

#[test]
fn jjtg_serialize_deserialize_roundtrip() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let json = serde_json::to_string_pretty(&gallops).unwrap();
    let restored: jjrg_Gallops = serde_json::from_str(&json).unwrap();

    assert_eq!(gallops.next_heat_seed, restored.next_heat_seed);
    assert_eq!(gallops.heats.len(), restored.heats.len());
}

#[test]
fn jjtg_multiple_errors_collected() {
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "!!!".to_string(), // Wrong length and chars
        heats: IndexMap::new(),
    };
    let (_, mut heat) = make_valid_heat("AB", "my-heat");
    heat.silks = "Invalid_Silks".to_string(); // Has underscore
    heat.creation_time = "invalid".to_string(); // Not YYMMDD
    gallops.heats.insert("₣AB".to_string(), heat);

    let errors = gallops.jjrg_validate().unwrap_err();
    // Should have multiple errors
    assert!(errors.len() >= 3);
}

// ===== Seed increment tests =====

#[test]
fn jjtg_zjjrg_increment_seed_simple() {
    assert_eq!(zjjrg_increment_seed("AA"), "AB");
    assert_eq!(zjjrg_increment_seed("AB"), "AC");
    assert_eq!(zjjrg_increment_seed("Az"), "A0");
}

#[test]
fn jjtg_zjjrg_increment_seed_carry() {
    // '_' is position 63, should wrap to 'A' (position 0) and carry
    assert_eq!(zjjrg_increment_seed("A_"), "BA");
    assert_eq!(zjjrg_increment_seed("__"), "AA"); // Full wrap around
}

#[test]
fn jjtg_zjjrg_increment_seed_three_chars() {
    assert_eq!(zjjrg_increment_seed("AAA"), "AAB");
    assert_eq!(zjjrg_increment_seed("AA_"), "ABA");
    assert_eq!(zjjrg_increment_seed("A__"), "BAA");
    assert_eq!(zjjrg_increment_seed("___"), "AAA");
}

// ===== Write operation tests =====

#[test]
fn jjtg_nominate_creates_heat() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir().join("jjk_test_nominate");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let args = jjrg_NominateArgs {
        silks: "test-heat".to_string(),
        created: "260113".to_string(),
    };

    let result = gallops.jjrg_nominate(args, &temp_dir).unwrap();

    // Check result
    assert!(result.firemark.starts_with('₣'));

    // Check heat was created
    assert!(gallops.heats.contains_key(&result.firemark));
    let heat = gallops.heats.get(&result.firemark).unwrap();
    assert_eq!(heat.silks, "test-heat");
    assert_eq!(heat.creation_time, "260113");
    assert_eq!(heat.status, jjrg_HeatStatus::Stabled);
    assert!(heat.order.is_empty());
    assert_eq!(heat.next_pace_seed, "AAA");

    // Check seed was incremented
    assert_eq!(gallops.next_heat_seed, "AC");

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn jjtg_nominate_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir();

    let args = jjrg_NominateArgs {
        silks: "invalid_silks".to_string(),
        created: "260113".to_string(),
    };

    let result = gallops.jjrg_nominate(args, &temp_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("silks must be non-empty alphanumeric-kebab"));
}

#[test]
fn jjtg_nominate_invalid_created() {
    let mut gallops = make_valid_gallops();
    let temp_dir = std::env::temp_dir();

    let args = jjrg_NominateArgs {
        silks: "test-heat".to_string(),
        created: "2026-01-13".to_string(),
    };

    let result = gallops.jjrg_nominate(args, &temp_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("created must be YYMMDD format"));
}

#[test]
fn jjtg_slate_creates_pace() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "test-pace".to_string(),
        text: "Do something useful".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    // Check result
    assert!(result.coronet.starts_with('₢'));
    assert!(result.coronet.contains("AB")); // Embeds heat identity

    // Check pace was created
    let heat = gallops.heats.get(&heat_key).unwrap();
    assert!(heat.paces.contains_key(&result.coronet));
    let pace = heat.paces.get(&result.coronet).unwrap();
    assert_eq!(pace.tacks.len(), 1);
    assert_eq!(pace.tacks[0].silks, "test-pace");
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Rough);
    assert_eq!(pace.tacks[0].text, "Do something useful");

    // Check order was updated
    assert!(heat.order.contains(&result.coronet));

    // Check seed was incremented (was AAB, now AAC due to existing pace)
    assert_eq!(heat.next_pace_seed, "AAC");
}

#[test]
fn jjtg_slate_heat_not_found() {
    let mut gallops = make_valid_gallops();

    let args = jjrg_SlateArgs {
        firemark: "CD".to_string(),
        silks: "test-pace".to_string(),
        text: "Do something".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn jjtg_slate_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "invalid_silks".to_string(),
        text: "Do something".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("silks must be non-empty alphanumeric-kebab"));
}

#[test]
fn jjtg_slate_empty_text() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "test-pace".to_string(),
        text: "".to_string(),
        before: None,
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("text must not be empty"));
}

#[test]
fn jjtg_slate_with_first_inserts_at_beginning() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let existing_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "new-first-pace".to_string(),
        text: "This should be first".to_string(),
        before: None,
        after: None,
        first: true,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], result.coronet); // New pace is first
    assert_eq!(heat.order[1], existing_pace); // Existing pace moved to second
}

#[test]
fn jjtg_slate_with_before_inserts_at_position() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Add a second pace
    let pace2_key = "₢ABAAB".to_string();
    let pace2 = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    };
    heat.paces.insert(pace2_key.clone(), pace2);
    heat.order.push(pace2_key.clone());
    heat.next_pace_seed = "AAC".to_string();

    let first_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Insert before the second pace
    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "inserted-pace".to_string(),
        text: "Insert before second".to_string(),
        before: Some(pace2_key.clone()),
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], first_pace); // Original first unchanged
    assert_eq!(heat.order[1], result.coronet); // New pace inserted
    assert_eq!(heat.order[2], pace2_key); // Second pace moved
}

#[test]
fn jjtg_slate_with_after_inserts_at_position() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Add a second pace
    let pace2_key = "₢ABAAB".to_string();
    let pace2 = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    };
    heat.paces.insert(pace2_key.clone(), pace2);
    heat.order.push(pace2_key.clone());
    heat.next_pace_seed = "AAC".to_string();

    let first_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Insert after the first pace
    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "inserted-pace".to_string(),
        text: "Insert after first".to_string(),
        before: None,
        after: Some(first_pace.clone()),
        first: false,
    };

    let result = gallops.jjrg_slate(args).unwrap();

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], first_pace); // Original first unchanged
    assert_eq!(heat.order[1], result.coronet); // New pace inserted after first
    assert_eq!(heat.order[2], pace2_key); // Second pace at end
}

#[test]
fn jjtg_slate_mutual_exclusivity() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let existing_pace = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    // Try with both before and first
    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "bad-pace".to_string(),
        text: "Should fail".to_string(),
        before: Some(existing_pace),
        after: None,
        first: true,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Only one of"));
}

#[test]
fn jjtg_slate_before_invalid_coronet() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_SlateArgs {
        firemark: "AB".to_string(),
        silks: "new-pace".to_string(),
        text: "Test".to_string(),
        before: Some("₢ABXXX".to_string()), // Non-existent
        after: None,
        first: false,
    };

    let result = gallops.jjrg_slate(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in heat"));
}

#[test]
fn jjtg_rail_reorders_paces() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Add another pace
    let pace2_key = "₢ABAAB".to_string();
    let pace2 = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    };
    heat.paces.insert(pace2_key.clone(), pace2);
    heat.order.push(pace2_key.clone());
    heat.next_pace_seed = "AAC".to_string();

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Reorder: swap the two paces (order mode)
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![pace2_key.clone(), original_first.clone()],
        move_coronet: None,
        before: None,
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], original_first);
}

#[test]
fn jjtg_rail_count_mismatch() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    gallops.heats.insert(heat_key, heat);

    // Try to reorder with wrong count
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec!["₢ABAAA".to_string(), "₢ABAAB".to_string()], // 2 items but only 1 pace
        move_coronet: None,
        before: None,
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("count mismatch"));
}

#[test]
fn jjtg_rail_duplicate_coronets() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Add another pace
    let pace2_key = "₢ABAAB".to_string();
    let pace2 = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    };
    heat.paces.insert(pace2_key.clone(), pace2);
    heat.order.push(pace2_key.clone());
    gallops.heats.insert(heat_key, heat);

    // Try with duplicate
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec!["₢ABAAA".to_string(), "₢ABAAA".to_string()],
        move_coronet: None,
        before: None,
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("duplicate"));
}

// Move mode tests

#[test]
fn jjtg_rail_move_first() {
    // Test that --first moves to first ACTIONABLE position, not absolute position 0
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Make the first pace (from make_valid_heat) complete
    let pace1_key = heat.order[0].clone();
    heat.paces.get_mut(&pace1_key).unwrap().tacks[0].state = jjrg_PaceState::Complete;

    // Add pace2 (complete) and pace3 (rough) and pace4 (rough)
    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    let pace4_key = "₢ABAAD".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "second-pace", None)],
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "third-pace", None)],
    });
    heat.paces.insert(pace4_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "fourth-pace", None)],
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());
    heat.order.push(pace4_key.clone());
    heat.next_pace_seed = "AAE".to_string();

    // Order: [complete1, complete2, rough3, rough4]
    gallops.heats.insert(heat_key.clone(), heat);

    // Move fourth pace (rough) to first actionable position
    // Should go BEFORE pace3 (first rough), not before pace1 (absolute first)
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace4_key.clone()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    // Expected order: [complete1, complete2, rough4, rough3]
    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace1_key, "complete1 should stay at index 0");
    assert_eq!(heat.order[1], pace2_key, "complete2 should stay at index 1");
    assert_eq!(heat.order[2], pace4_key, "rough4 should move to first actionable (index 2)");
    assert_eq!(heat.order[3], pace3_key, "rough3 should shift to index 3");
}

#[test]
fn jjtg_rail_move_first_all_complete() {
    // Test that --first moves to end when all paces are complete
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    // Make all paces complete
    let pace1_key = heat.order[0].clone();
    heat.paces.get_mut(&pace1_key).unwrap().tacks[0].state = jjrg_PaceState::Complete;

    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "second-pace", None)],
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Complete, "third-pace", None)],
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());
    heat.next_pace_seed = "AAD".to_string();

    // Order: [complete1, complete2, complete3]
    gallops.heats.insert(heat_key.clone(), heat);

    // Move first pace to "first actionable" - but all are complete, so goes to end
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace1_key.clone()),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    // Expected order: [complete2, complete3, complete1]
    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], pace3_key);
    assert_eq!(heat.order[2], pace1_key, "pace1 should move to end when no actionable paces");
}

#[test]
fn jjtg_rail_move_last() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    let pace2_key = "₢ABAAB".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    });
    heat.order.push(pace2_key.clone());
    heat.next_pace_seed = "AAC".to_string();

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Move first pace to last
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(original_first.clone()),
        before: None,
        after: None,
        first: false,
        last: true,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], original_first);
}

#[test]
fn jjtg_rail_move_before() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "third-pace", None)],
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());
    heat.next_pace_seed = "AAD".to_string();

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Move third pace before second (from [1,2,3] to [1,3,2])
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace3_key.clone()),
        before: Some(pace2_key.clone()),
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], original_first);
    assert_eq!(heat.order[1], pace3_key);
    assert_eq!(heat.order[2], pace2_key);
}

#[test]
fn jjtg_rail_move_after() {
    let mut gallops = make_valid_gallops();
    let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

    let pace2_key = "₢ABAAB".to_string();
    let pace3_key = "₢ABAAC".to_string();
    heat.paces.insert(pace2_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "second-pace", None)],
    });
    heat.paces.insert(pace3_key.clone(), jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "third-pace", None)],
    });
    heat.order.push(pace2_key.clone());
    heat.order.push(pace3_key.clone());
    heat.next_pace_seed = "AAD".to_string();

    let original_first = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Move first pace after second (from [1,2,3] to [2,1,3])
    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(original_first.clone()),
        before: None,
        after: Some(pace2_key.clone()),
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.order[0], pace2_key);
    assert_eq!(heat.order[1], original_first);
    assert_eq!(heat.order[2], pace3_key);
}

#[test]
fn jjtg_rail_move_requires_positioning_flag() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace_key),
        before: None,
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires exactly one positioning flag"));
}

#[test]
fn jjtg_rail_move_cannot_position_relative_to_self() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![],
        move_coronet: Some(pace_key.clone()),
        before: Some(pace_key),
        after: None,
        first: false,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("relative to itself"));
}

#[test]
fn jjtg_rail_move_cannot_combine_with_order() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_RailArgs {
        firemark: "AB".to_string(),
        order: vec![pace_key.clone()],
        move_coronet: Some(pace_key),
        before: None,
        after: None,
        first: true,
        last: false,
    };

    let result = gallops.jjrg_rail(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot combine --move with positional coronets"));
}

#[test]
fn jjtg_tally_state_transition() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Transition to complete
    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: Some(jjrg_PaceState::Complete),
        direction: None,
        text: Some("Work completed successfully".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks.len(), 2); // Original + new
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Complete);
    assert_eq!(pace.tacks[0].text, "Work completed successfully");
}

#[test]
fn jjtg_tally_bridled_requires_direction() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_TallyArgs {
        coronet: pace_key,
        state: Some(jjrg_PaceState::Bridled),
        direction: None, // Missing!
        text: Some("Ready for execution".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("direction is required"));
}

#[test]
fn jjtg_tally_bridled_with_direction() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: Some(jjrg_PaceState::Bridled),
        direction: Some("Execute autonomously".to_string()),
        text: Some("Ready for execution".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Bridled);
    assert_eq!(pace.tacks[0].direction.as_ref().unwrap(), "Execute autonomously");
}

#[test]
fn jjtg_tally_inherit_state() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // First tack is rough, add new tack without specifying state
    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: None, // Inherit
        direction: None,
        text: Some("Updated plan text".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Rough); // Inherited
    assert_eq!(pace.tacks[0].text, "Updated plan text");
}

#[test]
fn jjtg_tally_inherit_text() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    let original_text = heat.paces.get(&pace_key).unwrap().tacks[0].text.clone();
    gallops.heats.insert(heat_key.clone(), heat);

    // Just change state, inherit text
    let args = jjrg_TallyArgs {
        coronet: pace_key.clone(),
        state: Some(jjrg_PaceState::Complete),
        direction: None,
        text: None, // Inherit
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    let pace = heat.paces.get(&pace_key).unwrap();
    assert_eq!(pace.tacks[0].state, jjrg_PaceState::Complete);
    assert_eq!(pace.tacks[0].text, original_text); // Inherited
}

#[test]
fn jjtg_tally_non_bridled_forbids_direction() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat");
    let pace_key = heat.order[0].clone();
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_TallyArgs {
        coronet: pace_key,
        state: Some(jjrg_PaceState::Complete),
        direction: Some("Should not be here".to_string()), // Not allowed!
        text: Some("Done".to_string()),
        silks: None,
    };

    let result = gallops.jjrg_tally(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("direction must be absent"));
}
