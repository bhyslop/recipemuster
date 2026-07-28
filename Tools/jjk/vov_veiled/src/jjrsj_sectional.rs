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
//!
//! Beneath the command grain, the same file carries the git grain: while a
//! command is in flight the dispatcher arms the trace sink here, and the
//! farrier's git-execution boundary narrates every git child it spawns
//! (GIT-OPEN/GIT-OUTCOME lines, composed in `jjrfg_plaingit`) between that
//! command's OPEN and OUTCOME. A torn tail thereby names the exact child in
//! flight — "hung inside this git op" versus "hung between steps" —
//! which the 2026-07-26 refit wedge lacked
//! (Provenance: Memos/memo-20260726-refit-wedge-incident.md).

use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

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

/// The armed trace sink: the sectional path of the officium whose command is
/// in flight. The dispatcher arms it at step-open and disarms it after the
/// outcome, so anything narrated from the officium-blind depths (the farrier's
/// git-child lines) lands between that command's OPEN and OUTCOME in the same
/// file — one timeline, one torn tail. The server handles one command at a
/// time (stdio MCP), so this is a plain slot; the Mutex is only Rust's
/// spelling of a mutable static, not a concurrency design.
static ZJJRSJ_TRACE_SINK: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Arm the trace sink at `path`. Overwrites any prior arming — a command that
/// died before disarming (a panic mid-ceremony) is healed by the next arm.
pub fn jjrsj_trace_arm(path: PathBuf) {
    *ZJJRSJ_TRACE_SINK.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(path);
}

/// Disarm the trace sink. Narration while disarmed is silently dropped —
/// the sink is evidence, never authority, so absence of a sink is not an error.
pub fn jjrsj_trace_disarm() {
    *ZJJRSJ_TRACE_SINK.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
}

/// Append one narration line to the armed sink, if any. The caller composes
/// the whole line (the git-grain GIT-OPEN/GIT-OUTCOME grammar lives with its
/// narrator in `jjrfg_plaingit`); this module owns only the sink and the
/// best-effort append posture it shares with the step lines above.
pub fn jjrsj_trace(line: &str) {
    let sink = ZJJRSJ_TRACE_SINK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(path) = sink.as_ref() {
        zjjrsj_append(path, line);
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

/// Render one phase-grain beat line. Pure — no armed-sink dependency — so
/// deterministic tests can assert the grammar without arming the process-
/// global trace sink (`jjtfg_narration_and_local_deadline` is this crate's
/// sole arming test; a second arming test would race its slot).
pub(crate) fn zjjrsj_phase_line(now: chrono::DateTime<chrono::Utc>, cmd: &str, step: &str) -> String {
    format!("PHASE {} cmd={} step={}", now.to_rfc3339(), cmd, step)
}

/// Append a phase-grain beat to the armed sink: marks entry into one named
/// step of the crash-safe dispatch spine (lock, load, transform, save,
/// unlock) or a sibling I/O step (gazette, consign). One line per phase
/// entry — not an open/close bracket like the git grain, since the spine's
/// steps run in a fixed order with no child process to time — so a
/// mid-command kill's torn tail names the last phase the command reached.
pub fn jjrsj_phase(cmd: &str, step: &str) {
    jjrsj_trace(&zjjrsj_phase_line(chrono::Utc::now(), cmd, step));
}
