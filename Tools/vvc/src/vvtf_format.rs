// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for branded commit formatting

use super::vvcc_format::vvcc_format_branded;

#[test]
fn test_empty_identity() {
    let result = vvcc_format_branded("jjb", "1011", "", "n", "Fix bug", None);
    assert_eq!(result, "jjb:1011::n: Fix bug");
}

#[test]
fn test_with_identity() {
    let result = vvcc_format_branded("jjb", "1011", "₢AWAAb", "n", "Fix bug", None);
    assert_eq!(result, "jjb:1011:₢AWAAb:n: Fix bug");
}

#[test]
fn test_with_body() {
    let result = vvcc_format_branded("jjb", "1011", "₢AWAAb", "W", "Complete pace", Some("All tests passing\nReady for review"));
    assert_eq!(result, "jjb:1011:₢AWAAb:W: Complete pace\n\nAll tests passing\nReady for review");
}

#[test]
fn test_without_body_no_trailing_newline() {
    let result = vvcc_format_branded("jjb", "1011", "₢AWAAb", "n", "Fix bug", None);
    assert!(!result.ends_with('\n'));
}

#[test]
fn test_colon_enforcement() {
    // Verify 4 colon-delimited fields always present
    let result = vvcc_format_branded("vvb", "1011-a8c3738", "₣AW", "B", "Build complete", None);
    let parts: Vec<&str> = result.split(':').collect();
    assert!(parts.len() >= 4);
    assert_eq!(parts[0], "vvb");
    assert_eq!(parts[1], "1011-a8c3738");
    assert_eq!(parts[2], "₣AW");
    assert_eq!(parts[3], "B");
}

#[test]
fn test_empty_identity_colon_enforcement() {
    // Empty identity should produce consecutive colons
    let result = vvcc_format_branded("jjb", "1011", "", "s", "Session start", None);
    let prefix = "jjb:1011::s:";
    assert!(result.starts_with(prefix));
}
