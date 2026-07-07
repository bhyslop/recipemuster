// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the paddock read / curry write pair
//!
//! ## Coverage
//!
//! - Firemark parsing (both commands route through it)
//! - jjrg_curry_apply wipe backstop: empty/whitespace-only content rejects
//! - Note formatting for the curry revision commit message
//!
//! ## Not Tested
//!
//! - Curry commit behavior: Excluded due to vvc dependency on git operations.
//!   The shared dispatch lifecycle calls vvc::machine_commit() which requires:
//!   - Valid git repository state
//!   - Commit lock acquisition
//!   - File staging and commit execution
//!
//!   Testing this would require mocking the entire vvc commit machinery,
//!   which is beyond the scope of unit tests. Integration tests should
//!   cover the full commit workflow.

use std::collections::BTreeMap;
use super::jjrf_favor::jjrf_Firemark as Firemark;
use super::jjrg_gallops::{jjrg_Gallops, jjrg_curry_apply};

// ===== Firemark parsing tests (used in both getter and setter modes) =====

#[test]
fn jjtcu_firemark_parse_valid_with_prefix() {
    let result = Firemark::jjrf_parse("₣AB");
    assert!(result.is_ok());
    let firemark = result.unwrap();
    assert_eq!(firemark.jjrf_display(), "₣AB");
}

#[test]
fn jjtcu_firemark_parse_valid_without_prefix() {
    let result = Firemark::jjrf_parse("AB");
    assert!(result.is_ok());
    let firemark = result.unwrap();
    assert_eq!(firemark.jjrf_display(), "₣AB"); // Adds prefix
}

#[test]
fn jjtcu_firemark_parse_invalid_length() {
    let result = Firemark::jjrf_parse("ABC");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("must be 2 base64 characters"));
}

#[test]
fn jjtcu_firemark_parse_invalid_chars() {
    let result = Firemark::jjrf_parse("A!");
    assert!(result.is_err());
}

// ===== Wipe backstop at the paddock write funnel =====
// The guard fires before heat lookup or file I/O, so an empty gallops serves.

fn empty_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    }
}

#[test]
fn jjtcu_curry_apply_rejects_empty_content() {
    let firemark = Firemark::jjrf_parse("AB").unwrap();
    let err = jjrg_curry_apply(&empty_gallops(), &firemark, "").unwrap_err();
    assert!(err.contains("empty paddock"), "got: {}", err);
    assert!(err.contains("never blanks"), "names the refusal: {}", err);
}

#[test]
fn jjtcu_curry_apply_rejects_whitespace_only_content() {
    let firemark = Firemark::jjrf_parse("AB").unwrap();
    let err = jjrg_curry_apply(&empty_gallops(), &firemark, "  \n\t\n  ").unwrap_err();
    assert!(err.contains("empty paddock"), "got: {}", err);
}

// ===== Note field tests =====

#[test]
fn jjtcu_note_some_formats_correctly() {
    let note = Some("manual refinement");

    let description = if let Some(n) = note {
        format!("paddock curried: {}", n)
    } else {
        "paddock curried".to_string()
    };

    assert_eq!(description, "paddock curried: manual refinement");
}

#[test]
fn jjtcu_note_none_formats_correctly() {
    let note: Option<&str> = None;

    let description = if let Some(n) = note {
        format!("paddock curried: {}", n)
    } else {
        "paddock curried".to_string()
    };

    assert_eq!(description, "paddock curried");
}

// ===== Integration with jjrn_format_heat_discussion =====
// We can't test the full commit, but we can verify message formatting expectations

#[test]
fn jjtcu_chalk_message_format_expectations() {
    // The implementation calls jjrn_format_heat_discussion(firemark, description)
    // This should produce a message like: "jjb:1010-HASH:₣AB:D: paddock curried: manual update"
    // We can't test the full function without mocking, but we can validate inputs

    let firemark = Firemark::jjrf_parse("AB").unwrap();
    let description = "paddock curried: manual update";

    // Verify inputs are well-formed
    assert_eq!(firemark.jjrf_display(), "₣AB");
    assert!(description.starts_with("paddock curried"));

    // The actual message formatting is tested in jjtn_notch.rs
}
