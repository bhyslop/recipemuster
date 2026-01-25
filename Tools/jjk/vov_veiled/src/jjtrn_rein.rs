// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_rein command (steeplechase history)
//!
//! Tests edge cases for steeplechase parsing not covered by jjts_steeplechase.rs:
//! - Timestamp parsing with short/empty strings
//! - Malformed log lines with wrong field counts
//! - Log lines with non-JJ commits
//! - ReinArgs struct construction

use super::jjrs_steeplechase::{zjjrs_parse_timestamp, zjjrs_parse_log_line, zjjrs_parse_new_format, jjrs_ReinArgs};
use super::jjrrn_rein::jjrrn_ReinArgs;

// ===== Module import verification =====

#[test]
fn jjtrn_module_imports_work() {
    // Verify that the required types are accessible
    let _args = jjrs_ReinArgs {
        firemark: "AB".to_string(),
        limit: 50,
    };
    // If this compiles, imports work correctly
}

#[test]
fn jjtrn_cli_args_struct() {
    // Verify jjrrn_ReinArgs can be constructed with expected fields
    // Note: We can't easily test clap parsing without a full CLI context,
    // but we can verify the struct exists and has the right shape
    let args = jjrrn_ReinArgs {
        firemark: "AB".to_string(),
        limit: 100,
    };
    assert_eq!(args.firemark, "AB");
    assert_eq!(args.limit, 100);
}

// ===== Timestamp parsing edge cases =====

#[test]
fn jjtrn_parse_timestamp_empty() {
    // Empty string should return empty
    assert_eq!(zjjrs_parse_timestamp(""), "");
}

#[test]
fn jjtrn_parse_timestamp_short_string() {
    // String shorter than 16 chars should return as-is (trimmed)
    assert_eq!(zjjrs_parse_timestamp("2024-01-15"), "2024-01-15");
    assert_eq!(zjjrs_parse_timestamp("2024"), "2024");
    assert_eq!(zjjrs_parse_timestamp("x"), "x");
}

#[test]
fn jjtrn_parse_timestamp_whitespace_only() {
    // Whitespace-only string should return empty after trimming
    assert_eq!(zjjrs_parse_timestamp("   "), "");
    assert_eq!(zjjrs_parse_timestamp("\t\n"), "");
}

#[test]
fn jjtrn_parse_timestamp_with_leading_trailing_whitespace() {
    // Should trim and then truncate
    assert_eq!(
        zjjrs_parse_timestamp("  2024-01-15 14:30:00 -0800  "),
        "2024-01-15 14:30"
    );
}

#[test]
fn jjtrn_parse_timestamp_exactly_16_chars() {
    // Exactly 16 characters should return all of them
    assert_eq!(
        zjjrs_parse_timestamp("2024-01-15 14:30"),
        "2024-01-15 14:30"
    );
}

// ===== Malformed log line edge cases =====

#[test]
fn jjtrn_parse_log_line_no_tabs() {
    // Line with no tabs - only 1 part instead of 3
    let line = "2024-01-15 14:30:00 -0800 abc123ef jjb:1010:₢ABAAA:n: Fix bug";
    let result = zjjrs_parse_log_line(line, "AB");
    assert!(result.is_none(), "Line without tabs should not parse");
}

#[test]
fn jjtrn_parse_log_line_one_tab() {
    // Line with only one tab - 2 parts instead of 3
    let line = "2024-01-15 14:30:00 -0800\tabc123ef jjb:1010:₢ABAAA:n: Fix bug";
    let result = zjjrs_parse_log_line(line, "AB");
    assert!(result.is_none(), "Line with only one tab should not parse");
}

#[test]
fn jjtrn_parse_log_line_empty_parts() {
    // Line with tabs but empty parts
    let line = "\t\t";
    let result = zjjrs_parse_log_line(line, "AB");
    assert!(result.is_none(), "Line with empty parts should not parse");
}

#[test]
fn jjtrn_parse_log_line_empty_subject() {
    // Valid timestamp and commit but empty subject
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\t";
    let result = zjjrs_parse_log_line(line, "AB");
    // Empty subject doesn't start with "jjb:", so should be None
    assert!(result.is_none(), "Empty subject should not parse");
}

#[test]
fn jjtrn_parse_log_line_extra_tabs_in_subject() {
    // Tabs in subject should be preserved (splitn limits to 3)
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\tjjb:1010:₢ABAAA:n: Fix\tbug\twith\ttabs";
    let entry = zjjrs_parse_log_line(line, "AB").unwrap();
    assert_eq!(entry.subject, "Fix\tbug\twith\ttabs");
}

// ===== Non-JJ commit edge cases =====

#[test]
fn jjtrn_parse_log_line_regular_commit() {
    // Normal git commit without JJ prefix
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\tFix bug in login handler";
    let result = zjjrs_parse_log_line(line, "AB");
    assert!(result.is_none(), "Non-JJ commit should not parse");
}

#[test]
fn jjtrn_parse_log_line_merge_commit() {
    // Merge commit
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\tMerge branch 'feature' into main";
    let result = zjjrs_parse_log_line(line, "AB");
    assert!(result.is_none(), "Merge commit should not parse");
}

#[test]
fn jjtrn_parse_log_line_jjb_like_but_not() {
    // Subject starts with "jjb" but not "jjb:"
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\tjjbackup: Save files";
    let result = zjjrs_parse_log_line(line, "AB");
    assert!(result.is_none(), "jjb-like prefix without colon should not parse");
}

#[test]
fn jjtrn_parse_format_incomplete_coronet() {
    // Coronet with fewer than 5 chars after prefix
    let subject = "jjb:1010:₢AB:n: Short coronet";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none(), "Incomplete coronet should not parse");
}

#[test]
fn jjtrn_parse_format_incomplete_firemark() {
    // Firemark with fewer than 2 chars after prefix
    let subject = "jjb:1010:₣A:n: Short firemark";
    let result = zjjrs_parse_new_format(subject, "A");
    // This should fail because we're looking for "A" but format is broken
    assert!(result.is_none(), "Incomplete firemark should not parse");
}

#[test]
fn jjtrn_parse_format_missing_action_colon() {
    // Has identity but no trailing colon
    let subject = "jjb:1010:₢ABAAA Fix bug";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none(), "Missing colon after identity should not parse");
}

#[test]
fn jjtrn_parse_format_no_identity() {
    // Has hallmark but no identity prefix
    let subject = "jjb:1010:ABAAA:n: Fix bug";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none(), "Missing identity prefix should not parse");
}

#[test]
fn jjtrn_parse_format_action_without_message() {
    // Has action code but no message after it
    let subject = "jjb:1010:₢ABAAA:n:";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    // Empty message is valid
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "");
}

// ===== ReinArgs construction =====

#[test]
fn jjtrn_rein_args_default_limit() {
    // Verify we can construct with different limit values
    let args = jjrs_ReinArgs {
        firemark: "CD".to_string(),
        limit: 10,
    };
    assert_eq!(args.firemark, "CD");
    assert_eq!(args.limit, 10);
}

#[test]
fn jjtrn_rein_args_with_prefix() {
    // Firemark can be passed with or without ₣ prefix
    // (normalization happens in jjrs_get_entries, not in struct construction)
    let args_with_prefix = jjrs_ReinArgs {
        firemark: "₣AB".to_string(),
        limit: 50,
    };
    let args_without = jjrs_ReinArgs {
        firemark: "AB".to_string(),
        limit: 50,
    };
    assert_eq!(args_with_prefix.firemark, "₣AB");
    assert_eq!(args_without.firemark, "AB");
}
