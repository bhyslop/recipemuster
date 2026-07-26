// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;

use super::jjrsj_sectional::{zjjrsj_step_open_at, zjjrsj_step_outcome_at};
use super::jjtu_testdir::JjkTestDir;

fn jjtsj_read(dir: &JjkTestDir) -> String {
    std::fs::read_to_string(dir.path().join("sectional.log")).unwrap()
}

#[test]
fn jjtsj_open_outcome_pairing_ok() {
    let dir = JjkTestDir::new("jjtsj-pairing-ok");
    let path = dir.path().join("sectional.log");
    zjjrsj_step_open_at(&path, "jjx_record");
    let result: Result<CallToolResult, ErrorData> =
        Ok(CallToolResult::success(vec![Content::text("fine")]));
    zjjrsj_step_outcome_at(&path, "jjx_record", &result);

    let body = jjtsj_read(&dir);
    let lines: Vec<&str> = body.lines().collect();
    assert_eq!(lines.len(), 2, "expected exactly one open/outcome pair: {:?}", lines);
    assert!(lines[0].starts_with("OPEN "), "line 0: {}", lines[0]);
    assert!(lines[0].contains("cmd=jjx_record"), "line 0: {}", lines[0]);
    assert!(lines[1].starts_with("OUTCOME "), "line 1: {}", lines[1]);
    assert!(lines[1].contains("cmd=jjx_record"), "line 1: {}", lines[1]);
    assert!(lines[1].contains("status=ok"), "line 1: {}", lines[1]);
}

#[test]
fn jjtsj_outcome_classifies_application_error() {
    let dir = JjkTestDir::new("jjtsj-app-error");
    let path = dir.path().join("sectional.log");
    zjjrsj_step_open_at(&path, "jjx_close");
    let result: Result<CallToolResult, ErrorData> =
        Ok(CallToolResult::error(vec![Content::text("INTERDICTUM")]));
    zjjrsj_step_outcome_at(&path, "jjx_close", &result);

    let body = jjtsj_read(&dir);
    let lines: Vec<&str> = body.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[1].contains("status=error"), "line 1: {}", lines[1]);
}

#[test]
fn jjtsj_outcome_classifies_transport_error() {
    let dir = JjkTestDir::new("jjtsj-transport-error");
    let path = dir.path().join("sectional.log");
    zjjrsj_step_open_at(&path, "jjx_show");
    let result: Result<CallToolResult, ErrorData> =
        Err(ErrorData::internal_error("boom", None));
    zjjrsj_step_outcome_at(&path, "jjx_show", &result);

    let body = jjtsj_read(&dir);
    let lines: Vec<&str> = body.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[1].contains("status=error"), "line 1: {}", lines[1]);
}

/// The stall fingerprint: a killed mid-ceremony command writes the open line
/// and never returns to write the outcome. The torn tail is readable and
/// distinguishable from a paired (complete) entry by construction — no
/// special marker, just an odd number of lines / a trailing OPEN with no
/// following OUTCOME for that cmd.
#[test]
fn jjtsj_torn_tail_reads_as_entry_without_exit() {
    let dir = JjkTestDir::new("jjtsj-torn-tail");
    let path = dir.path().join("sectional.log");
    zjjrsj_step_open_at(&path, "jjx_record");
    let result: Result<CallToolResult, ErrorData> =
        Ok(CallToolResult::success(vec![Content::text("fine")]));
    zjjrsj_step_outcome_at(&path, "jjx_record", &result);
    // The killed ceremony: an open with no matching outcome.
    zjjrsj_step_open_at(&path, "jjx_close");

    let body = jjtsj_read(&dir);
    let lines: Vec<&str> = body.lines().collect();
    assert_eq!(lines.len(), 3, "torn tail: {:?}", lines);
    assert!(lines[2].starts_with("OPEN "), "torn tail line: {}", lines[2]);
    assert!(lines[2].contains("cmd=jjx_close"), "torn tail line: {}", lines[2]);
    assert!(
        !lines.iter().any(|l| l.starts_with("OUTCOME") && l.contains("cmd=jjx_close")),
        "no outcome should exist for the killed step: {:?}",
        lines
    );
}
