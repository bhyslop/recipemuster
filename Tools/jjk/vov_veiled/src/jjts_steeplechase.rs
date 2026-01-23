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
fn jjts_parse_format_standard_notch() {
    let subject = "jjb:1010-abc1234:₢ABAAA:n: Fix the bug";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010-abc1234".to_string()));
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "Fix the bug");
}

#[test]
fn jjts_parse_format_chalk_wrap() {
    let subject = "jjb:1010:₢ABAAA:W: Completed the task";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("W".to_string()));
    assert_eq!(entry.subject, "Completed the task");
}

#[test]
fn jjts_parse_format_chalk_approach() {
    let subject = "jjb:1010-def5678:₢ABCDE:A: Starting work";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010-def5678".to_string()));
    assert_eq!(entry.coronet, Some("₢ABCDE".to_string()));
    assert_eq!(entry.action, Some("A".to_string()));
    assert_eq!(entry.subject, "Starting work");
}

#[test]
fn jjts_parse_format_heat_level_nominate() {
    let subject = "jjb:1010:₣AB:N: my-new-heat";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("N".to_string()));
    assert_eq!(entry.subject, "my-new-heat");
}

#[test]
fn jjts_parse_format_heat_level_slate() {
    let subject = "jjb:1010:₣AB:S: new-pace-silks";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("S".to_string()));
    assert_eq!(entry.subject, "new-pace-silks");
}

#[test]
fn jjts_parse_format_heat_level_rail() {
    let subject = "jjb:1010:₣AB:r: reordered";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("r".to_string()));
    assert_eq!(entry.subject, "reordered");
}

#[test]
fn jjts_parse_format_heat_level_retire() {
    let subject = "jjb:1010:₣AB:R: my-heat-silks";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("R".to_string()));
    assert_eq!(entry.subject, "my-heat-silks");
}

#[test]
fn jjts_parse_format_heat_discussion() {
    let subject = "jjb:1010:₣AB:d: Design discussion";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, None);
    assert_eq!(entry.action, Some("d".to_string()));
    assert_eq!(entry.subject, "Design discussion");
}

#[test]
fn jjts_parse_format_any_hallmark() {
    // Should parse successfully with any hallmark format
    let subject = "jjb:9999-xyz9999:₢ABAAA:n: Fix bug";
    let entry = zjjrs_parse_new_format(subject, "AB").unwrap();
    assert_eq!(entry.hallmark, Some("9999-xyz9999".to_string()));
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "Fix bug");
}

#[test]
fn jjts_parse_format_wrong_firemark() {
    // Different firemark in coronet - should NOT match
    let subject = "jjb:1010:₢CDAAA:n: Fix bug";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none());
}

#[test]
fn jjts_parse_format_wrong_heat_firemark() {
    let subject = "jjb:1010:₣CD:N: some-heat";
    let result = zjjrs_parse_new_format(subject, "AB");
    assert!(result.is_none());
}

#[test]
fn jjts_parse_log_line() {
    let line = "2024-01-15 14:30:00 -0800\tabc123ef\tjjb:1010:₢ABAAA:n: Fix bug";
    let entry = zjjrs_parse_log_line(line, "AB").unwrap();
    assert_eq!(entry.timestamp, "2024-01-15 14:30");
    assert_eq!(entry.commit, "abc123ef");
    assert_eq!(entry.hallmark, Some("1010".to_string()));
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("n".to_string()));
    assert_eq!(entry.subject, "Fix bug");
}

#[test]
fn jjts_parse_log_line_with_action() {
    let line = "2024-01-15 14:30:00 -0800\tdef456ab\tjjb:1010-abc1234:₢ABAAA:F: Autonomous execution";
    let entry = zjjrs_parse_log_line(line, "AB").unwrap();
    assert_eq!(entry.timestamp, "2024-01-15 14:30");
    assert_eq!(entry.commit, "def456ab");
    assert_eq!(entry.hallmark, Some("1010-abc1234".to_string()));
    assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
    assert_eq!(entry.action, Some("F".to_string()));
    assert_eq!(entry.subject, "Autonomous execution");
}
