// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_furlough command (change heat status/rename)

use super::jjrg_gallops::*;
use std::collections::BTreeMap;
use indexmap::IndexMap;

// Helper to create a minimal valid Gallops structure
fn make_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heats: IndexMap::new(),
    }
}

// Helper to create a valid Tack
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

// Helper to create a valid Pace
fn make_valid_pace(heat_id: &str) -> (String, jjrg_Pace) {
    let pace_key = format!("₢{}AAA", heat_id);
    let pace = jjrg_Pace {
        tacks: vec![make_valid_tack(jjrg_PaceState::Rough, "test-pace")],
    };
    (pace_key, pace)
}

// Helper to create a valid Heat
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
        paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
        paces,
    };
    (heat_key, heat)
}

// ===== Error cases =====

#[test]
fn jjtfu_no_options_error() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: false,
        stabled: false,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("At least one option required"));
}

#[test]
fn jjtfu_both_racing_and_stabled_error() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: true,
        stabled: true,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cannot specify both --racing and --stabled"));
}

#[test]
fn jjtfu_invalid_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: false,
        stabled: false,
        silks: Some("invalid_silks".to_string()),
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("silks must be non-empty alphanumeric-kebab"));
}

#[test]
fn jjtfu_heat_not_found() {
    let gallops = make_valid_gallops();

    let args = jjrg_FurloughArgs {
        firemark: "CD".to_string(),
        racing: true,
        stabled: false,
        silks: None,
    };

    let mut gallops = gallops;
    let result = gallops.jjrg_furlough(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn jjtfu_retired_heat_error() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Retired);
    gallops.heats.insert(heat_key, heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: true,
        stabled: false,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("retired (terminal state)"));
}

#[test]
fn jjtfu_already_racing_promotes_to_top() {
    let mut gallops = make_valid_gallops();
    let (heat_key_a, heat_a) = make_valid_heat("AA", "first-heat", jjrg_HeatStatus::Racing);
    let (heat_key_b, heat_b) = make_valid_heat("AB", "second-heat", jjrg_HeatStatus::Racing);
    gallops.heats.insert(heat_key_a, heat_a);
    gallops.heats.insert(heat_key_b, heat_b);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: true,
        stabled: false,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());
    // AB should now be first
    let keys: Vec<&String> = gallops.heats.keys().collect();
    assert_eq!(keys[0], "₣AB");
}

#[test]
fn jjtfu_already_stabled_promotes_to_top() {
    let mut gallops = make_valid_gallops();
    let (heat_key_a, heat_a) = make_valid_heat("AA", "first-heat", jjrg_HeatStatus::Stabled);
    let (heat_key_b, heat_b) = make_valid_heat("AB", "second-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key_a, heat_a);
    gallops.heats.insert(heat_key_b, heat_b);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: false,
        stabled: true,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());
    // AB should now be first
    let keys: Vec<&String> = gallops.heats.keys().collect();
    assert_eq!(keys[0], "₣AB");
}

// ===== Happy path cases =====

#[test]
fn jjtfu_change_status_stabled_to_racing() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: true,
        stabled: false,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.status, jjrg_HeatStatus::Racing);
    // Verify silks unchanged
    assert_eq!(heat.silks, "my-heat");
}

#[test]
fn jjtfu_change_status_racing_to_stabled() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Racing);
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: false,
        stabled: true,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.status, jjrg_HeatStatus::Stabled);
    // Verify silks unchanged
    assert_eq!(heat.silks, "my-heat");
}

#[test]
fn jjtfu_rename_heat() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: false,
        stabled: false,
        silks: Some("new-heat-name".to_string()),
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.silks, "new-heat-name");
    // Verify status unchanged
    assert_eq!(heat.status, jjrg_HeatStatus::Stabled);
}

#[test]
fn jjtfu_change_status_and_rename() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key.clone(), heat);

    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: true,
        stabled: false,
        silks: Some("new-heat-name".to_string()),
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.status, jjrg_HeatStatus::Racing);
    assert_eq!(heat.silks, "new-heat-name");
}

#[test]
fn jjtfu_racing_moves_heat_to_front() {
    let mut gallops = make_valid_gallops();

    // Create three heats with valid base64 firemarks (A-Z, a-z, 0-9, -, _)
    let (key_ab, heat_ab) = make_valid_heat("AB", "first-heat", jjrg_HeatStatus::Stabled);
    let (key_cd, heat_cd) = make_valid_heat("CD", "second-heat", jjrg_HeatStatus::Stabled); // Note: start as Stabled
    let (key_ef, heat_ef) = make_valid_heat("EF", "third-heat", jjrg_HeatStatus::Stabled);

    gallops.heats.insert(key_ab.clone(), heat_ab);
    gallops.heats.insert(key_cd.clone(), heat_cd);
    gallops.heats.insert(key_ef.clone(), heat_ef);

    // Move second heat (CD) to racing - should move to front
    let args = jjrg_FurloughArgs {
        firemark: "CD".to_string(),
        racing: true,
        stabled: false,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok(), "Furlough failed: {:?}", result);

    // Verify ordering: CD should be first now
    let keys: Vec<_> = gallops.heats.keys().collect();
    assert_eq!(*keys[0], key_cd, "Heat CD should be moved to front");
}

#[test]
fn jjtfu_valid_kebab_case_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key.clone(), heat);

    // Test various valid kebab-case names
    let valid_names = vec![
        "simple",
        "test-heat",
        "my-cool-heat",
        "heat-with-123-numbers",
        "a-b-c-d-e",
        "MixedCase",
        "Upper-lower-Mix",
    ];

    for new_silks in valid_names {
        let mut gallops_copy = make_valid_gallops();
        let (heat_key_copy, heat_copy) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
        gallops_copy.heats.insert(heat_key_copy.clone(), heat_copy);

        let args = jjrg_FurloughArgs {
            firemark: "AB".to_string(),
            racing: false,
            stabled: false,
            silks: Some(new_silks.to_string()),
        };

        let result = gallops_copy.jjrg_furlough(args);
        assert!(result.is_ok(), "Valid kebab-case '{}' should be accepted", new_silks);
    }
}

#[test]
fn jjtfu_invalid_kebab_case_silks() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key, heat);

    // Test various invalid kebab-case names
    let invalid_names = vec![
        "snake_case",
        "-leading-dash",
        "trailing-dash-",
        "double--dash",
        "",
    ];

    for invalid_silks in invalid_names {
        let mut gallops_copy = make_valid_gallops();
        let (heat_key_copy, heat_copy) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
        gallops_copy.heats.insert(heat_key_copy, heat_copy);

        let args = jjrg_FurloughArgs {
            firemark: "AB".to_string(),
            racing: false,
            stabled: false,
            silks: Some(invalid_silks.to_string()),
        };

        let result = gallops_copy.jjrg_furlough(args);
        assert!(result.is_err(), "Invalid kebab-case '{}' should be rejected", invalid_silks);
    }
}

#[test]
fn jjtfu_normalized_firemark_short_form() {
    let mut gallops = make_valid_gallops();
    let (heat_key, heat) = make_valid_heat("AB", "my-heat", jjrg_HeatStatus::Stabled);
    gallops.heats.insert(heat_key.clone(), heat);

    // Test with short form "AB" instead of "₣AB"
    let args = jjrg_FurloughArgs {
        firemark: "AB".to_string(),
        racing: true,
        stabled: false,
        silks: None,
    };

    let result = gallops.jjrg_furlough(args);
    assert!(result.is_ok());

    let heat = gallops.heats.get(&heat_key).unwrap();
    assert_eq!(heat.status, jjrg_HeatStatus::Racing);
}
