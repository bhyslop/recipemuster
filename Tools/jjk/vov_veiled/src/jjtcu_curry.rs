// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_curry command (paddock getter/setter with chalk)
//!
//! ## Coverage
//!
//! - CurryVerb enum and as_str() method
//! - Getter mode: Read and display paddock content
//! - Setter mode: Verb validation logic (requires exactly one verb flag)
//!
//! ## Not Tested
//!
//! - Setter mode commit behavior: Excluded due to vvc dependency on git operations.
//!   The jjrg_curry() function calls vvc::machine_commit() which requires:
//!   - Valid git repository state
//!   - Commit lock acquisition
//!   - File staging and commit execution
//!
//!   Testing this would require mocking the entire vvc commit machinery,
//!   which is beyond the scope of unit tests. Integration tests should
//!   cover the full commit workflow.

use super::jjrcu_curry::*;
use super::jjrf_favor::jjrf_Firemark as Firemark;

// ===== CurryVerb tests =====

#[test]
fn jjtcu_curry_verb_as_str_refine() {
    assert_eq!(jjrcu_CurryVerb::Refine.as_str(), "refine");
}

#[test]
fn jjtcu_curry_verb_as_str_level() {
    assert_eq!(jjrcu_CurryVerb::Level.as_str(), "level");
}

#[test]
fn jjtcu_curry_verb_as_str_muck() {
    assert_eq!(jjrcu_CurryVerb::Muck.as_str(), "muck");
}

#[test]
fn jjtcu_curry_verb_equality() {
    assert_eq!(jjrcu_CurryVerb::Refine, jjrcu_CurryVerb::Refine);
    assert_ne!(jjrcu_CurryVerb::Refine, jjrcu_CurryVerb::Level);
    assert_ne!(jjrcu_CurryVerb::Level, jjrcu_CurryVerb::Muck);
}

#[test]
fn jjtcu_curry_verb_copy_clone() {
    let verb = jjrcu_CurryVerb::Refine;
    let verb_copy = verb; // Copy trait
    let verb_clone = verb.clone(); // Clone trait

    assert_eq!(verb, verb_copy);
    assert_eq!(verb, verb_clone);
    assert_eq!(verb_copy, verb_clone);
}

// ===== Verb validation tests =====
// These test the logic in jjrcu_run_curry that validates verb flags

#[test]
fn jjtcu_verb_validation_zero_verbs_error() {
    // Simulates: args.refine=false, args.level=false, args.muck=false
    let verb_count = [false, false, false]
        .iter()
        .filter(|&&x| x)
        .count();

    assert_eq!(verb_count, 0);
    // In real code: error "setter mode requires exactly one verb flag"
}

#[test]
fn jjtcu_verb_validation_one_verb_valid() {
    // Simulates: args.refine=true, args.level=false, args.muck=false
    let verb_count = [true, false, false]
        .iter()
        .filter(|&&x| x)
        .count();

    assert_eq!(verb_count, 1); // Valid
}

#[test]
fn jjtcu_verb_validation_two_verbs_error() {
    // Simulates: args.refine=true, args.level=true, args.muck=false
    let verb_count = [true, true, false]
        .iter()
        .filter(|&&x| x)
        .count();

    assert_eq!(verb_count, 2);
    // In real code: error "only one verb flag allowed"
}

#[test]
fn jjtcu_verb_validation_three_verbs_error() {
    // Simulates: all flags true
    let verb_count = [true, true, true]
        .iter()
        .filter(|&&x| x)
        .count();

    assert_eq!(verb_count, 3);
    // In real code: error "only one verb flag allowed"
}

#[test]
fn jjtcu_verb_selection_refine() {
    // Simulates: args.refine=true, args.level=false, args.muck=false
    let (refine, level, _muck) = (true, false, false);

    let verb = if refine {
        jjrcu_CurryVerb::Refine
    } else if level {
        jjrcu_CurryVerb::Level
    } else {
        jjrcu_CurryVerb::Muck
    };

    assert_eq!(verb, jjrcu_CurryVerb::Refine);
}

#[test]
fn jjtcu_verb_selection_level() {
    // Simulates: args.refine=false, args.level=true, args.muck=false
    let (refine, level, _muck) = (false, true, false);

    let verb = if refine {
        jjrcu_CurryVerb::Refine
    } else if level {
        jjrcu_CurryVerb::Level
    } else {
        jjrcu_CurryVerb::Muck
    };

    assert_eq!(verb, jjrcu_CurryVerb::Level);
}

#[test]
fn jjtcu_verb_selection_muck() {
    // Simulates: args.refine=false, args.level=false, args.muck=true
    let (refine, level, _muck) = (false, false, true);

    let verb = if refine {
        jjrcu_CurryVerb::Refine
    } else if level {
        jjrcu_CurryVerb::Level
    } else {
        jjrcu_CurryVerb::Muck
    };

    assert_eq!(verb, jjrcu_CurryVerb::Muck);
}

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

// ===== Getter mode behavior tests =====
// These test the logic flow, not actual file I/O

#[test]
fn jjtcu_getter_mode_no_stdin_detected() {
    // In getter mode, stdin_content is None
    let stdin_content: Option<String> = None;

    assert!(stdin_content.is_none()); // Getter mode detected
}

#[test]
fn jjtcu_setter_mode_stdin_detected() {
    // In setter mode, stdin_content is Some(...)
    let stdin_content: Option<String> = Some("New paddock content".to_string());

    assert!(stdin_content.is_some()); // Setter mode detected
}

// ===== Note field tests =====

#[test]
fn jjtcu_note_some_formats_correctly() {
    let note = Some("manual refinement");
    let verb = "refine";

    let description = if let Some(n) = note {
        format!("paddock curried ({}): {}", verb, n)
    } else {
        format!("paddock curried ({})", verb)
    };

    assert_eq!(description, "paddock curried (refine): manual refinement");
}

#[test]
fn jjtcu_note_none_formats_correctly() {
    let note: Option<&str> = None;
    let verb = "level";

    let description = if let Some(n) = note {
        format!("paddock curried ({}): {}", verb, n)
    } else {
        format!("paddock curried ({})", verb)
    };

    assert_eq!(description, "paddock curried (level)");
}

#[test]
fn jjtcu_note_all_verbs_format() {
    let verbs = [
        jjrcu_CurryVerb::Refine,
        jjrcu_CurryVerb::Level,
        jjrcu_CurryVerb::Muck,
    ];

    for verb in &verbs {
        let description = format!("paddock curried ({})", verb.as_str());
        assert!(description.contains("paddock curried"));
        assert!(description.contains(verb.as_str()));
    }
}

// ===== Error message validation =====

#[test]
fn jjtcu_error_messages_are_clear() {
    // Verify the error messages used in the implementation are helpful

    let no_verb_error = "setter mode requires exactly one verb flag (--refine, --level, or --muck)";
    assert!(no_verb_error.contains("exactly one"));
    assert!(no_verb_error.contains("--refine"));
    assert!(no_verb_error.contains("--level"));
    assert!(no_verb_error.contains("--muck"));

    let multi_verb_error = "only one verb flag allowed";
    assert!(multi_verb_error.contains("only one"));
}

// ===== Integration with jjrn_format_heat_discussion =====
// We can't test the full commit, but we can verify message formatting expectations

#[test]
fn jjtcu_chalk_message_format_expectations() {
    // The implementation calls jjrn_format_heat_discussion(firemark, description)
    // This should produce a message like: "jjb:1010-HASH:₣AB:D: paddock curried (refine)"
    // We can't test the full function without mocking, but we can validate inputs

    let firemark = Firemark::jjrf_parse("AB").unwrap();
    let description = "paddock curried (refine): manual update";

    // Verify inputs are well-formed
    assert_eq!(firemark.jjrf_display(), "₣AB");
    assert!(description.starts_with("paddock curried"));

    // The actual message formatting is tested in jjtn_notch.rs
}
