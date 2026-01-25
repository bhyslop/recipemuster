// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_scout command (regex search across heats/paces)

use regex::Regex;
use crate::jjrg_gallops::jjrg_PaceState as PaceState;
use crate::jjrsc_scout::{zjrsc_pace_state_str, zjrsc_extract_match_context};

// ============================================================================
// zjrsc_pace_state_str tests
// ============================================================================

#[test]
fn jjtsc_pace_state_str_rough() {
    assert_eq!(zjrsc_pace_state_str(&PaceState::Rough), "rough");
}

#[test]
fn jjtsc_pace_state_str_bridled() {
    assert_eq!(zjrsc_pace_state_str(&PaceState::Bridled), "bridled");
}

#[test]
fn jjtsc_pace_state_str_complete() {
    assert_eq!(zjrsc_pace_state_str(&PaceState::Complete), "complete");
}

#[test]
fn jjtsc_pace_state_str_abandoned() {
    assert_eq!(zjrsc_pace_state_str(&PaceState::Abandoned), "abandoned");
}

// ============================================================================
// zjrsc_extract_match_context tests
// ============================================================================

#[test]
fn jjtsc_extract_match_context_middle() {
    // Match in the middle with full context available (need >30 chars before match for ellipsis)
    let content = "This is a longer prefix text that exceeds thirty characters MATCH_HERE and some suffix text after that also exceeds thirty chars";
    let re = Regex::new("MATCH_HERE").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    // Should have ... prefix (start > 30) and ... suffix (more than 30 chars after match)
    assert!(result.starts_with("..."), "Expected prefix ellipsis, got: {}", result);
    assert!(result.ends_with("..."), "Expected suffix ellipsis, got: {}", result);
    assert!(result.contains("MATCH_HERE"));
}

#[test]
fn jjtsc_extract_match_context_at_start() {
    // Match at the very beginning
    let content = "MATCH at the start of the content with more text following";
    let re = Regex::new("MATCH").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    // Should NOT have ... prefix (start == 0)
    assert!(!result.starts_with("..."));
    assert!(result.contains("MATCH"));
    // Should have ... suffix (content longer than context window)
    assert!(result.ends_with("..."));
}

#[test]
fn jjtsc_extract_match_context_at_end() {
    // Match at the very end
    let content = "Some text at the beginning and then MATCH";
    let re = Regex::new("MATCH").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    // Should have ... prefix
    assert!(result.starts_with("..."));
    // Should NOT have ... suffix (end == len)
    assert!(!result.ends_with("..."));
    assert!(result.contains("MATCH"));
}

#[test]
fn jjtsc_extract_match_context_short_content() {
    // Content shorter than context window
    let content = "short MATCH text";
    let re = Regex::new("MATCH").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    // Should have no ellipsis since content is small
    assert!(!result.starts_with("..."));
    assert!(!result.ends_with("..."));
    assert_eq!(result, "short MATCH text");
}

#[test]
fn jjtsc_extract_match_context_with_newlines() {
    // Content with newlines should have them replaced with spaces
    let content = "line one\nMATCH\nline three";
    let re = Regex::new("MATCH").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    // Newlines should be replaced with spaces
    assert!(!result.contains('\n'));
    assert!(result.contains("line one MATCH line three"));
}

#[test]
fn jjtsc_extract_match_context_carriage_return() {
    // Content with carriage returns should have them removed
    let content = "line one\r\nMATCH\r\nline three";
    let re = Regex::new("MATCH").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    // Carriage returns should be removed, newlines replaced with spaces
    assert!(!result.contains('\r'));
    assert!(!result.contains('\n'));
}

#[test]
fn jjtsc_extract_match_context_case_insensitive() {
    // Test with case-insensitive regex (as used in scout)
    use regex::RegexBuilder;
    let content = "searching for KEYWORD in text";
    let re = RegexBuilder::new("keyword")
        .case_insensitive(true)
        .build()
        .unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    assert!(result.contains("KEYWORD"));
}

#[test]
fn jjtsc_extract_match_context_exact_boundary() {
    // Match just past context boundary (31 chars from start means window_start = 1)
    let prefix = "a".repeat(31);
    let content = format!("{}MATCH suffix", prefix);
    let re = Regex::new("MATCH").unwrap();
    let result = zjrsc_extract_match_context(&content, &re);

    // Should have ... prefix since window_start = 31 - 30 = 1 > 0
    assert!(result.starts_with("..."), "Expected prefix ellipsis, got: {}", result);
    assert!(result.contains("MATCH"));
}

#[test]
fn jjtsc_extract_match_context_long_match() {
    // Longer match pattern
    let content = "prefix LONG_MATCH_PATTERN_HERE suffix";
    let re = Regex::new("LONG_MATCH_PATTERN_HERE").unwrap();
    let result = zjrsc_extract_match_context(content, &re);

    assert!(result.contains("LONG_MATCH_PATTERN_HERE"));
}
