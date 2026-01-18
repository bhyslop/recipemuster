// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrs_steeplechase::*;

#[test]
fn jjts_parse_timestamp() {
    assert_eq!(
        zjjrs_parse_timestamp("2024-01-15 14:30:00 -0800"),
        "2024-01-15 14:30"
    );
    assert_eq!(
        zjjrs_parse_timestamp("2024-12-31 23:59:59 +0000"),
        "2024-12-31 23:59"
    );
}

#[test]
fn jjts_parse_new_format_standard_notch() {
    let subject = "jjb:RBM:₢ABAAA:n: Fix the bug";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "Fix the bug");
}

#[test]
fn jjts_parse_new_format_chalk_wrap() {
    let subject = "jjb:RBM:₢ABAAA:W: Completed the task";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("W".to_string()));
    assert_eq!(entry.subject, "Completed the task");
}

#[test]
fn jjts_parse_new_format_chalk_approach() {
    let subject = "jjb:RBM:₢ABCDE:A: Starting work";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, Some("₢ABCDE".to_string()));
    assert_eq!(entry.action, Some("A".to_string()));
    assert_eq!(entry.subject, "Starting work");
}

#[test]
fn jjts_parse_new_format_heat_level_nominate() {
    let subject = "jjb:RBM:₣AB:N: my-new-heat";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("N".to_string()));
    assert_eq!(entry.subject, "my-new-heat");
}

#[test]
fn jjts_parse_new_format_heat_level_slate() {
    let subject = "jjb:RBM:₣AB:S: new-pace-silks";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("S".to_string()));
    assert_eq!(entry.subject, "new-pace-silks");
}

#[test]
fn jjts_parse_new_format_heat_level_rail() {
    let subject = "jjb:RBM:₣AB:r: reordered";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("r".to_string()));
    assert_eq!(entry.subject, "reordered");
}

#[test]
fn jjts_parse_new_format_heat_level_retire() {
    let subject = "jjb:RBM:₣AB:R: my-heat-silks";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("R".to_string()));
    assert_eq!(entry.subject, "my-heat-silks");
}

#[test]
fn jjts_parse_new_format_heat_discussion() {
    let subject = "jjb:RBM:₣AB:d: Design discussion";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("d".to_string()));
    assert_eq!(entry.subject, "Design discussion");
}

#[test]
fn jjts_parse_new_format_any_brand() {
    // Should parse successfully with any brand - filtering is by identity
    let subject = "jjb:OTHER:₢ABAAA:n: Fix bug";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "Fix bug");
}

#[test]
fn jjts_parse_new_format_wrong_firemark() {
    // Different firemark in coronet - should NOT match
    let subject = "jjb:RBM:₢CDAAA:n: Fix bug";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none());
}

#[test]
fn jjts_parse_new_format_wrong_heat_firemark() {
    let subject = "jjb:RBM:₣CD:N: some-heat";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none());
}

#[test]
fn jjts_parse_log_line_new_format() {
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\tjjb:RBM:₢ABAAA:n: Fix bug";
    let entry = zjjrs_parse_log_line(line, "AB").unwrap();
    assert_eq!(entry.timestamp, "2024-01-15 14:30");
    assert_eq!(entry.commit, "abc123ef");
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "Fix bug");
}

#[test]
fn jjts_parse_log_line_new_format_with_action() {
    let line = "2024-01-15 14:30:00 -0800\tdef456ab\tjjb:RBM:₢ABAAA:F: Autonomous execution";
    let entry = zjjrs_parse_log_line(line, "AB").unwrap();
    assert_eq!(entry.timestamp, "2024-01-15 14:30");
    assert_eq!(entry.commit, "def456ab");
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("F".to_string()));
    assert_eq!(entry.subject, "Autonomous execution");
}
