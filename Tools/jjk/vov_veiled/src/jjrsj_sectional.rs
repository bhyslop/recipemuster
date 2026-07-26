// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! The sectional — the per-officium step journal (JJK CLAUDE.md "sectional":
//! the recorded per-segment split times of a run).
//!
//! Every jjx ceremony narrates its step-open and step-outcome to an
//! append-only local file living beside the officium's gazettes — MCP-side
//! only, no stile change, no env var, no BUK surface. Entry-without-exit is
//! the stall fingerprint: the open line writes before the dispatch, the
//! outcome line after, so a killed mid-ceremony command leaves a readable
//! torn tail by construction. The sectional is evidence, never a transaction
//! log — the studbook remains the record; no recovery logic may trust it.
//! (Provenance: Memos/memo-20260724-github-ssh-flap-jjx-stall-phenomena.md,
//! "The observability gap".)

use std::io::Write;
use std::path::PathBuf;

use rmcp::model::CallToolResult;
use rmcp::ErrorData;

use crate::jjrm_mcp::jjrm_exchange_dir;

/// The sectional's fixed basename within the officium exchange directory —
/// sibling to the gazettes, never the network path under diagnosis.
const JJRSJ_SECTIONAL_FILE: &str = "sectional.log";

/// Resolve an officium ID to its absolute sectional file path.
pub fn jjrsj_sectional_path(officium: &str) -> PathBuf {
    jjrm_exchange_dir(officium).join(JJRSJ_SECTIONAL_FILE)
}

/// Append one line to a sectional file, given its already-resolved path.
/// Best-effort: a write failure here must never fail or alter the ceremony
/// being narrated, so errors are silently swallowed — the sectional is
/// evidence, never authority.
fn zjjrsj_append(path: &std::path::Path, line: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{}", line);
    }
}

/// Step-open at an explicit path — the testable seam `jjrsj_step_open`
/// resolves onto (parallel to `zjjrm_exchange_dir_over`): written before the
/// command's dispatch, so a killed process leaves this line without a
/// matching outcome — the stall fingerprint.
pub(crate) fn zjjrsj_step_open_at(path: &std::path::Path, cmd: &str) {
    zjjrsj_append(path, &format!("OPEN {} cmd={}", chrono::Utc::now().to_rfc3339(), cmd));
}

/// Step-outcome at an explicit path — the testable seam `jjrsj_step_outcome`
/// resolves onto. Classifies the result as ok or error; an error can arrive
/// at either layer — the transport-level `ErrorData`, or an
/// application-level `CallToolResult` whose `is_error` flag is set
/// (refusals, INTERDICTUM, deser failures). For an error at either layer,
/// the raw result text lands on a RAW line ahead of the OUTCOME line —
/// raw foreign error text before any classification verdict —
/// JSON-escaped so a multi-line foreign message still holds the file's
/// line grain.
pub(crate) fn zjjrsj_step_outcome_at(path: &std::path::Path, cmd: &str, result: &Result<CallToolResult, ErrorData>) {
    let raw: Option<String> = match result {
        Ok(r) if r.is_error == Some(true) => Some(
            r.content
                .iter()
                .filter_map(|c| c.as_text().map(|t| t.text.as_str()))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        Ok(_) => None,
        Err(e) => Some(e.message.to_string()),
    };
    if let Some(raw) = raw {
        let escaped = serde_json::to_string(&raw).unwrap_or_else(|_| "\"<unencodable>\"".to_string());
        zjjrsj_append(path, &format!("RAW {} cmd={} {}", chrono::Utc::now().to_rfc3339(), cmd, escaped));
    }
    let status = match result {
        Ok(r) if r.is_error == Some(true) => "error",
        Ok(_) => "ok",
        Err(_) => "error",
    };
    zjjrsj_append(path, &format!("OUTCOME {} cmd={} status={}", chrono::Utc::now().to_rfc3339(), cmd, status));
}

/// Step-open: written before the command's dispatch, so a killed process
/// leaves this line without a matching outcome — the stall fingerprint.
pub fn jjrsj_step_open(officium: &str, cmd: &str) {
    zjjrsj_step_open_at(&jjrsj_sectional_path(officium), cmd);
}

/// Step-outcome: written after the command's dispatch completes.
pub fn jjrsj_step_outcome(officium: &str, cmd: &str, result: &Result<CallToolResult, ErrorData>) {
    zjjrsj_step_outcome_at(&jjrsj_sectional_path(officium), cmd, result);
}
