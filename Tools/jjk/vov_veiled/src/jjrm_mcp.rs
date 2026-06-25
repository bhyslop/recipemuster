// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK MCP Server - Single dispatcher tool for all jjx operations
//!
//! Exposes one MCP tool (`jjx`) that takes a command name and JSON params,
//! then dispatches to the appropriate handler. This replaces 25 individual
//! MCP tools with a single entry point, reducing ToolSearch friction.
//!
//! Handlers return (i32, String) — exit code and accumulated output.
//! The MCP layer converts this to CallToolResult (success/error).

use std::path::{Path, PathBuf};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo, CallToolResult, Content};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};
use vvc::vvco_out;

// Handler imports
use crate::jjrnc_notch::{jjrnc_NotchArgs, jjrnc_run_notch};
use crate::jjrrn_rein::{jjrrn_ReinArgs, jjrrn_run_rein};
use crate::jjrvl_validate::{jjrvl_ValidateArgs, jjrvl_run_validate};
use crate::jjrmu_muster::{jjrmu_MusterArgs, jjrmu_run_muster};
use crate::jjrsd_saddle::{jjrsd_SaddleArgs, jjrsd_run_saddle};
use crate::jjrpd_parade::{jjrpd_ParadeArgs, jjrpd_run_parade};
use crate::jjrrt_retire::{jjrrt_RetireArgs, jjrrt_run_retire};
use crate::jjrno_nominate::{jjrx_NominateArgs, jjrx_run_nominate};
use crate::jjrsl_slate::{jjrsl_SlateArgs, jjrsl_run_slate};
use crate::jjrrl_rail::{jjrrl_RailArgs, jjrrl_run_rail};
use crate::jjrtl_tally::{jjrtl_run_revise_docket, jjrtl_RelabelArgs, jjrtl_run_relabel, jjrtl_DropArgs, jjrtl_run_drop};
use crate::jjrdr_draft::{jjrdr_DraftArgs, jjrdr_run_draft};
use crate::jjrfu_furlough::{jjrfu_FurloughArgs, jjrfu_run_furlough};
use crate::jjrwp_wrap::{jjrx_WrapArgs, zjjrx_run_wrap};
use crate::jjrsc_scout::{jjrsc_ScoutArgs, jjrsc_run_scout};
use crate::jjrgs_get_spec::{jjrgs_GetSpecArgs, jjrgs_run_get_spec};
use crate::jjrgc_get_coronets::{jjrgc_GetCoronetsArgs, jjrgc_run_get_coronets};
use crate::jjrcu_curry::{jjrcu_CurryArgs, jjrcu_run_curry};
use crate::jjrrs_restring::{jjrrs_RestringArgs, jjrrs_run};
use crate::jjrld_landing::{jjrld_LandingArgs, jjrld_run_landing};
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug, JJRZ_SLUG_HALTER, jjrz_parse_slate_input, jjrz_parse_paddock_input, jjrz_parse_halter_input, jjrz_parse_batch_input};
use crate::jjrg_gallops::{jjrg_slate, jjrg_curry_apply};
use crate::jjrt_types::jjrg_SlateArgs;
use crate::jjrn_notch::jjrn_format_heat_discussion;

const GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";
const OFFICIA_DIR: &str = ".claude/jjm/officia";
const HEARTBEAT_FILE: &str = "heartbeat";
const GAZETTE_IN_FILE: &str = "gazette_in.md";
const GAZETTE_OUT_FILE: &str = "gazette_out.md";
const PROBE_DATE_FILE: &str = ".probe_date";
const EXSANGUINATION_THRESHOLD_SECS: u64 = 7 * 24 * 3600;
const OFFICIUM_SUN_PREFIX: char = '\u{2609}'; // ☉
const OFFICIUM_SUFFIX_LEN: usize = 4; // random discriminant chars appended to YYMMDD-NNNN
const OFFICIUM_FIRST_ORDINAL: u32 = 1000; // first-of-day daily ordinal NNNN (per-machine seed)

// Command name constants — RCG String Boundary Discipline.
// Lifecycle commands (bypass Gallops lock)
const JJRM_CMD_NAME_OPEN: &str = "jjx_open";
// Gallops commands
const JJRM_CMD_NAME_RECORD: &str = "jjx_record";
const JJRM_CMD_NAME_LOG: &str = "jjx_log";
const JJRM_CMD_NAME_VALIDATE: &str = "jjx_validate";
const JJRM_CMD_NAME_LIST: &str = "jjx_list";
const JJRM_CMD_NAME_ORIENT: &str = "jjx_orient";
const JJRM_CMD_NAME_SHOW: &str = "jjx_show";
const JJRM_CMD_NAME_ARCHIVE: &str = "jjx_archive";
const JJRM_CMD_NAME_CREATE: &str = "jjx_create";
const JJRM_CMD_NAME_ENROLL: &str = "jjx_enroll";
const JJRM_CMD_NAME_REORDER: &str = "jjx_reorder";
const JJRM_CMD_NAME_REDOCKET: &str = "jjx_redocket";
const JJRM_CMD_NAME_RELABEL: &str = "jjx_relabel";
const JJRM_CMD_NAME_DROP: &str = "jjx_drop";
const JJRM_CMD_NAME_RELOCATE: &str = "jjx_relocate";
const JJRM_CMD_NAME_ALTER: &str = "jjx_alter";
const JJRM_CMD_NAME_CLOSE: &str = "jjx_close";
const JJRM_CMD_NAME_SEARCH: &str = "jjx_search";
const JJRM_CMD_NAME_BRIEF: &str = "jjx_brief";
const JJRM_CMD_NAME_CORONETS: &str = "jjx_coronets";
const JJRM_CMD_NAME_PADDOCK: &str = "jjx_paddock";
const JJRM_CMD_NAME_TRANSFER: &str = "jjx_transfer";
const JJRM_CMD_NAME_LANDING: &str = "jjx_landing";
// Legatio commands (remote dispatch)
const JJRM_CMD_NAME_BIND: &str = "jjx_bind";
const JJRM_CMD_NAME_SEND: &str = "jjx_send";
const JJRM_CMD_NAME_PLANT: &str = "jjx_plant";
const JJRM_CMD_NAME_FETCH: &str = "jjx_fetch";
const JJRM_CMD_NAME_RELAY: &str = "jjx_relay";
const JJRM_CMD_NAME_CHECK: &str = "jjx_check";
// Complete registry of all commands
const JJRM_ALL_COMMANDS: &[&str] = &[
    JJRM_CMD_NAME_OPEN,
    JJRM_CMD_NAME_RECORD, JJRM_CMD_NAME_LOG, JJRM_CMD_NAME_VALIDATE,
    JJRM_CMD_NAME_LIST, JJRM_CMD_NAME_ORIENT, JJRM_CMD_NAME_SHOW,
    JJRM_CMD_NAME_ARCHIVE, JJRM_CMD_NAME_CREATE, JJRM_CMD_NAME_ENROLL,
    JJRM_CMD_NAME_REORDER, JJRM_CMD_NAME_REDOCKET, JJRM_CMD_NAME_RELABEL,
    JJRM_CMD_NAME_DROP, JJRM_CMD_NAME_RELOCATE, JJRM_CMD_NAME_ALTER,
    JJRM_CMD_NAME_CLOSE, JJRM_CMD_NAME_SEARCH, JJRM_CMD_NAME_BRIEF,
    JJRM_CMD_NAME_CORONETS, JJRM_CMD_NAME_PADDOCK,
    JJRM_CMD_NAME_TRANSFER, JJRM_CMD_NAME_LANDING,
    JJRM_CMD_NAME_BIND, JJRM_CMD_NAME_SEND, JJRM_CMD_NAME_PLANT,
    JJRM_CMD_NAME_FETCH, JJRM_CMD_NAME_RELAY, JJRM_CMD_NAME_CHECK,
];

fn gallops_pathbuf() -> PathBuf {
    PathBuf::from(GALLOPS_PATH)
}

// ============================================================================
// Result conversion
// ============================================================================

/// Convert handler (exit_code, output) to MCP CallToolResult.
fn jjrm_result(result: (i32, String)) -> Result<CallToolResult, McpError> {
    let (code, output) = result;
    if code == 0 {
        Ok(CallToolResult::success(vec![Content::text(output)]))
    } else {
        Ok(CallToolResult::error(vec![Content::text(output)]))
    }
}

/// Convert the validate handler's tri-state (exit_code, output) to an MCP CallToolResult.
///
/// validate's enumerated verdict (JJSCVL) is 0 clean / 2 normalized / 1 broken. Both 0 and 2 are
/// valid outcomes — a normalized store is a success, not a failure — so only the broken code maps
/// to an MCP error. The exact bucket is named in the self-describing stdout either way.
fn zjjrm_validate_result(result: (i32, String)) -> Result<CallToolResult, McpError> {
    let (code, output) = result;
    if code == 1 {
        Ok(CallToolResult::error(vec![Content::text(output)]))
    } else {
        Ok(CallToolResult::success(vec![Content::text(output)]))
    }
}


/// Return deserialization error as MCP error result.
fn jjrm_deser_error(cmd: &str, e: serde_json::Error) -> Result<CallToolResult, McpError> {
    Ok(CallToolResult::error(vec![Content::text(format!("jjx {}: invalid params: {}", cmd, e))]))
}

// ============================================================================
// Dispatch lifecycle — jjsodp_command_lifecycle (Operation Taxonomy in JJS0)
// ============================================================================

/// Shared dispatch body: lock → load → call handler → persist → return output.
///
/// Per jjdk_sole_operator, every operation locks and persists unconditionally.
/// The handler returns output text on success — it has no knowledge of locks,
/// persistence, or commit messages. The caller supplies the commit message:
/// heat-affiliated gazette ops (paddock revision, mixed batch) pass a branded
/// heat-discussion message so the single commit carries the ₣XX chalk and any
/// note.
fn zjjrm_dispatch_inner_msg(
    cmd: &str,
    firemark: &crate::jjrf_favor::jjrf_Firemark,
    size_limit: u64,
    message: String,
    handler: impl FnOnce(&mut crate::jjrg_gallops::jjrg_Gallops) -> Result<String, String>,
) -> Result<CallToolResult, McpError> {
    use vvc::vvco_Output;

    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: error: {}", cmd, e),
            )]));
        }
    };

    let gallops_path = gallops_pathbuf();
    let mut gallops = match crate::jjrg_gallops::jjrg_Gallops::jjrg_load(&gallops_path) {
        Ok(g) => g,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: error loading Gallops: {}", cmd, e),
            )]));
        }
    };

    let output = match handler(&mut gallops) {
        Ok(o) => o,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: error: {}", cmd, e),
            )]));
        }
    };

    let mut persist_output = vvco_Output::buffer();
    match crate::jjri_io::jjri_persist(
        &lock,
        &gallops,
        &gallops_path,
        firemark,
        message,
        size_limit,
        &mut persist_output,
    ) {
        Ok(_hash) => {}
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: error: {}", cmd, e),
            )]));
        }
    }

    Ok(CallToolResult::success(vec![Content::text(output)]))
}

/// Resolve the single heat firemark a mixed batch targets, enforcing the
/// same-firemark guard: every reslate coronet's parent and the paddock firemark
/// (if present) must name one heat. Slates carry silks, not a firemark — they
/// inherit the resolved heat. A slate-only batch has no anchor and is rejected.
/// This guard also closes the legacy mass-reslate cross-heat misattribution,
/// which keyed the whole batch off the first coronet's heat with no check.
///
/// Public so coverage lands on a real run-boundary rather than a `z` helper
/// (RCG: a `z` private never goes public for a test).
pub fn jjrm_resolve_batch_firemark(
    batch: &crate::jjrz_gazette::jjrz_BatchInput,
) -> Result<crate::jjrf_favor::jjrf_Firemark, String> {
    use crate::jjrf_favor::{jjrf_Firemark as Firemark, jjrf_Coronet as Coronet};
    let mut candidates: Vec<Firemark> = Vec::new();
    if let Some((fm_str, _)) = &batch.paddock {
        candidates.push(Firemark::jjrf_parse(fm_str).map_err(|e| format!("paddock firemark: {}", e))?);
    }
    for (coronet_str, _) in &batch.reslates {
        let coronet = Coronet::jjrf_parse(coronet_str)
            .map_err(|e| format!("reslate coronet '{}': {}", coronet_str, e))?;
        candidates.push(coronet.jjrf_parent_firemark());
    }
    let first = candidates.first()
        .ok_or_else(|| "slate-only batch has no heat anchor; include a paddock or reslate notice, or use jjx_enroll".to_string())?
        .clone();
    for fm in &candidates[1..] {
        if fm.jjrf_display() != first.jjrf_display() {
            return Err(format!(
                "cross-heat batch rejected: notices span heats {} and {} (a batch is single-heat)",
                first.jjrf_display(), fm.jjrf_display()
            ));
        }
    }
    Ok(first)
}

/// Apply a resolved single-heat batch to the in-memory gallops: paddock revision
/// (writes the paddock file), then reslates (order-free), then slates in file
/// order with only the first taking the cursor (before/after/first). Pure
/// transform over gallops + one paddock-file side effect — no lock, no commit;
/// the shared dispatch lifecycle persists the result in one commit. Returns the
/// human-readable per-notice summary. Public so tests exercise it directly.
pub fn jjrm_apply_batch(
    gallops: &mut crate::jjrg_gallops::jjrg_Gallops,
    batch: &crate::jjrz_gazette::jjrz_BatchInput,
    firemark: &crate::jjrf_favor::jjrf_Firemark,
    before: Option<String>,
    after: Option<String>,
    first: bool,
) -> Result<String, String> {
    let mut lines: Vec<String> = Vec::new();
    // Paddock first — write the paddock file; jjri_persist co-commits it.
    if let Some((_, content)) = &batch.paddock {
        jjrg_curry_apply(gallops, firemark, content)?;
        lines.push("paddock revised".to_string());
    }
    // Reslates — order irrelevant, each targets its own coronet.
    for (coronet, docket) in &batch.reslates {
        let diff = jjrtl_run_revise_docket(gallops, coronet, docket)?;
        if !diff.is_empty() {
            lines.push(format!("--- ₢{} reslate diff ---\n{}", coronet, diff));
        }
    }
    // Slates — file order is pace order; only the first takes the cursor.
    for (idx, (silks, docket)) in batch.slates.iter().enumerate() {
        let (b, a, f) = if idx == 0 {
            (before.clone(), after.clone(), first)
        } else {
            (None, None, false)
        };
        let res = jjrg_slate(gallops, jjrg_SlateArgs {
            firemark: firemark.jjrf_display(),
            silks: silks.clone(),
            text: docket.clone(),
            before: b,
            after: a,
            first: f,
        })?;
        lines.push(format!("slated ₢{}", res.coronet));
    }
    let mut output = format!(
        "Batch applied: {} paddock, {} reslate, {} slate",
        batch.paddock.is_some() as usize, batch.reslates.len(), batch.slates.len()
    );
    if !lines.is_empty() {
        output.push_str("\n\n");
        output.push_str(&lines.join("\n"));
    }
    Ok(output)
}

/// One-line description of a mixed batch for the branded heat-discussion commit.
fn zjjrm_batch_description(batch: &crate::jjrz_gazette::jjrz_BatchInput) -> String {
    let mut parts = Vec::new();
    if batch.paddock.is_some() { parts.push("paddock".to_string()); }
    if !batch.reslates.is_empty() { parts.push(format!("{} reslate", batch.reslates.len())); }
    if !batch.slates.is_empty() { parts.push(format!("{} slate", batch.slates.len())); }
    format!("batch: {}", parts.join(", "))
}

// ============================================================================
// MCP parameter structs (kept for serde deserialization)
// ============================================================================

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RecordParams {
    pub identity: String,
    pub files: Vec<String>,
    pub size_limit: Option<u64>,
    pub intent: Option<String>,
}


#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_LogParams {
    pub firemark: String,
    pub limit: Option<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ValidateParams {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ListParams {
    pub status: Option<String>,
}

// jjx_orient takes no params — its single target arrives solely through the
// gazette halter notice (one lede = one firemark or coronet). A `firemark`
// param is rejected, not honored, by zjjrm_rejected_target_param.

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ShowParams {
    /// REQUIRED. Affects firemark expansion only (exclude complete/abandoned);
    /// coronet-named targets return regardless of state. Target selection itself
    /// comes solely from the gazette halter notices, never a param.
    pub remaining: bool,
    /// Optional. Hark mode (JJS0 jjda_hark): a git revision. When present, show
    /// renders the Gallops and paddock as of that revision (read-only, via
    /// jjdr_hark) and sets no standing emblem; jjx_orient never accepts it.
    pub hark: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ArchiveParams {
    pub firemark: String,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CreateParams {
    pub silks: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_EnrollParams {
    pub firemark: String,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default)]
    pub first: bool,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ReorderParams {
    pub firemark: String,
    pub r#move: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default)]
    pub first: bool,
    #[serde(default)]
    pub last: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ReviseDocketParams {
    pub size_limit: Option<u64>,
    /// Position the FIRST slate notice before this coronet (mixed batch only;
    /// mutually exclusive with after/first). Reslate/paddock never move the cursor.
    pub before: Option<String>,
    /// Position the FIRST slate notice after this coronet (mixed batch only).
    pub after: Option<String>,
    /// Position the FIRST slate notice at the head of the heat (mixed batch only).
    #[serde(default)]
    pub first: bool,
}


#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RelabelParams {
    pub coronet: String,
    pub silks: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_DropParams {
    pub coronet: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RelocateParams {
    pub coronet: String,
    pub to: String,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default)]
    pub first: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_AlterParams {
    pub firemark: String,
    #[serde(default)]
    pub racing: bool,
    #[serde(default)]
    pub stabled: bool,
    pub silks: Option<String>,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CloseParams {
    pub coronet: String,
    pub summary: Option<String>,
    pub spook: Option<String>,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_SearchParams {
    pub pattern: String,
    #[serde(default)]
    pub actionable: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_GetBriefParams {
    pub coronet: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_GetCoronetsParams {
    pub firemark: String,
    #[serde(default)]
    pub remaining: bool,
    #[serde(default)]
    pub rough: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_PaddockParams {
    #[serde(default)]
    pub firemark: Option<String>,
    pub note: Option<String>,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_TransferParams {
    pub firemark: String,
    pub to: String,
    pub coronets: String,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_LandingParams {
    pub coronet: String,
    pub agent: String,
    pub content: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_BindParams {
    pub alias: String,
    pub reldir: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_SendParams {
    pub legatio: String,
    pub command: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_PlantParams {
    pub legatio: String,
    pub commit: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_FetchParams {
    pub legatio: String,
    pub path: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RelayParams {
    pub legatio: String,
    pub tabtarget: String,
    pub timeout: u64,
    pub firemark: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CheckParams {
    pub pensum: String,
    pub timeout: u64,
}

// ============================================================================
// Single dispatcher tool params
// ============================================================================

fn jjrm_empty_object() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_JjxParams {
    #[schemars(description = "Command name: jjx_list, jjx_show, jjx_orient, jjx_record, jjx_log, jjx_validate, jjx_create, jjx_enroll, jjx_close, jjx_archive, jjx_reorder, jjx_redocket, jjx_relabel, jjx_drop, jjx_relocate, jjx_alter, jjx_search, jjx_brief, jjx_coronets, jjx_paddock, jjx_continue, jjx_transfer, jjx_landing, jjx_bind, jjx_send, jjx_plant, jjx_fetch, jjx_relay, jjx_check, jjx_open")]
    pub command: String,
    #[schemars(description = "Command parameters as JSON object. See CLAUDE.md for per-command schemas.")]
    #[serde(default = "jjrm_empty_object")]
    pub params: serde_json::Value,
    #[schemars(description = "Officium identity (from jjx_open). Required for all commands except jjx_open.")]
    #[serde(default)]
    pub officium: Option<String>,
    #[schemars(description = "Agent model ID string (e.g. 'claude-opus-4-6[1m]'). Required on all commands.")]
    pub model: String,
}

// ============================================================================
// vvx_tt — tabtarget runner (vvx-surface sibling tool, NOT a jjx command)
// ============================================================================
//
// A second MCP tool on the vvx server: execs a tt/*.sh tabtarget so tabtarget
// runs stop drawing a per-invocation Bash permission prompt — one MCP approval
// absorbs the sediment. Bounded to tt/*.sh (no arbitrary-command path), and it
// enforces the tabtarget discipline the agent otherwise has to remember: run
// from the repo root, never pipe to tail/head/grep (output is captured, so the
// exit code is preserved — the pipe hazard that silently masks failures cannot
// arise), and point the caller at the self-logged ../logs-buk/ record.

/// Tabtarget directory, relative to the repo root (the server's cwd).
const VVX_TT_DIR: &str = "tt";
/// Conventional self-log of the most recent invocation (BURS_LOG_DIR default).
/// Stable across runs and — under the sequential-only test discipline — always
/// this run, so we point the caller here rather than re-derive the fragile,
/// station-configurable per-tabtarget log tag.
const VVX_TT_LOG_HINT: &str = "../logs-buk/last.txt";
/// Inline-output ceiling: return the last N bytes of captured output and defer
/// the full record to the log, so a long test suite cannot flood the response.
const VVX_TT_OUTPUT_TAIL_BYTES: usize = 60_000;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_VvxTtParams {
    #[schemars(description = "Tabtarget script to run, e.g. \"rbw-ts.TestSuite.fast.sh\". A leading \"tt/\" is accepted and stripped. Must be a bare filename ending in .sh (no path separators, no \"..\") — the tool is bounded to tt/*.sh and refuses anything else.")]
    pub tabtarget: String,
    #[schemars(description = "Positional arguments forwarded to the tabtarget unchanged (e.g. a fixture name for rbw-tf.FixtureRun.sh). Optional.")]
    #[serde(default)]
    pub args: Vec<String>,
}

/// Validate + normalize a tabtarget name to the bare `<name>.sh` form the tool
/// is bounded to. A leading `tt/` is stripped; the result must be a single path
/// component ending in `.sh`. This is the no-arbitrary-command guard — the tool
/// runs only `tt/*.sh`, never a free command or a path that escapes the dir.
fn zjjrm_normalize_tabtarget(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    let name = trimmed.strip_prefix("tt/").unwrap_or(trimmed);
    if name.is_empty() {
        return Err("tabtarget is empty".to_string());
    }
    if name.contains('/') || name.contains("..") {
        return Err(format!(
            "tabtarget '{}' must be a bare filename under tt/ (no path separators, no '..')",
            raw
        ));
    }
    if !name.ends_with(".sh") {
        return Err(format!("tabtarget '{}' must end in .sh", raw));
    }
    Ok(name.to_string())
}

/// Return the last `max_bytes` of `s` on a char boundary, with a truncation flag.
fn zjjrm_tail(s: &str, max_bytes: usize) -> (&str, bool) {
    if s.len() <= max_bytes {
        return (s, false);
    }
    let mut start = s.len() - max_bytes;
    while start < s.len() && !s.is_char_boundary(start) {
        start += 1;
    }
    (&s[start..], true)
}

/// Run a tt/ tabtarget as a subprocess from the repo root, capturing combined
/// stdout+stderr. Returns (exit_code, report). Blocking std::process — the MCP
/// server serves one client and tabtargets run sequentially by discipline, and
/// the sibling jjx handlers already block on git subprocesses, so no async
/// process machinery is warranted. The report leads with the exit status and
/// the self-logged output path, then a tail of the captured output.
fn zjjrm_run_tabtarget(params: jjrm_VvxTtParams) -> (i32, String) {
    let name = match zjjrm_normalize_tabtarget(&params.tabtarget) {
        Ok(n) => n,
        Err(e) => return (1, format!("vvx_tt: {}", e)),
    };
    let rel_path = format!("{}/{}", VVX_TT_DIR, name);
    if !Path::new(&rel_path).is_file() {
        return (1, format!(
            "vvx_tt: no tabtarget at {} — vvx_tt runs from the repo root and is bounded to tt/*.sh",
            rel_path
        ));
    }

    // bash <script> <args…>: byte-identical argv to a direct `./tt/<name>` run
    // ($0 -> tt/<name>, so the z-launcher basename/dir logic is unchanged), and
    // robust to a missing executable bit. We exec directly — never via a shell
    // string — so the args cannot be reinterpreted as a command.
    let output = match std::process::Command::new("bash")
        .arg(&rel_path)
        .args(&params.args)
        .output()
    {
        Ok(o) => o,
        Err(e) => return (1, format!("vvx_tt: failed to launch {}: {}", rel_path, e)),
    };

    let code = output.status.code().unwrap_or(-1);
    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        if !combined.is_empty() {
            combined.push('\n');
        }
        combined.push_str(&stderr);
    }

    let args_suffix = if params.args.is_empty() {
        String::new()
    } else {
        format!(" {}", params.args.join(" "))
    };
    let mut report = format!(
        "vvx_tt {}{}: exit {}\nlog: {} (self-logged full record; read it directly, never pipe to tail/head/grep)\n",
        name, args_suffix, code, VVX_TT_LOG_HINT,
    );
    let (shown, truncated) = zjjrm_tail(&combined, VVX_TT_OUTPUT_TAIL_BYTES);
    if truncated {
        report.push_str(&format!(
            "--- output (last {} bytes; full record in log) ---\n",
            VVX_TT_OUTPUT_TAIL_BYTES
        ));
    } else {
        report.push_str("--- output ---\n");
    }
    report.push_str(shown);
    (code, report)
}

// ============================================================================
// vvx_render — diagram-viewer push (vvx-surface sibling tool, NOT a jjx command)
// ============================================================================
//
// The lower tool behind the `unfurl` upper verb (JJS0 jjdo_render): read an
// image and push it to the standalone diagram viewer. Like vvx_tt it is a
// vvx-surface sibling tool — no officium, no gallops, transient — so it carries
// none of the lock→load→save invariant the gallops operations do. Best-effort /
// fail-soft: a missing port-file, an unreachable viewer, or an unreadable image
// returns a soft notice, never an McpError and never a panic. Bringing the
// viewer up is paneboard's job (it conducts the window), not this tool's.
//
// Wire framing is paneboard's FROZEN contract (the "Diagram Viewer — Wire
// Protocol" section of its PoC spec; reference impl viewer/src/pbgvt_transport.rs):
// one JSON control line '\n'-terminated, then exactly pbgvw_len payload bytes,
// then — only when pbgvw_dark_len is present and non-zero — exactly
// pbgvw_dark_len further bytes (the dark variant of the light/dark pair). rbm
// speaks it as a client and freezes nothing. Control keys and the verb enum
// carry the `pbgvw_` sprue, so `grep pbgvw_` is the format's whole census. The
// optional dark variant is now transported (the 2026-06-23 additive pair
// revision): when `dark` is supplied, both payloads ride one frame; when it is
// absent the frame is byte-identical to the prior single-payload push.

/// Discovery path tail (under `$HOME`): the viewer publishes its ephemeral
/// localhost port here (write-temp-then-rename). Sibling to the emblem root
/// under paneboard's per-user `~/.config/paneboard/` config home.
const JJRM_VIEWER_PORT_TAIL: &str = ".config/paneboard/viewer.port";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RenderParams {
    #[schemars(description = "Path to the light image to display (required). SVG or raster — the viewer sniffs the payload, so there is no type tag.")]
    pub light: String,
    #[schemars(description = "Optional path to the dark variant. When supplied, it is transported as the light/dark pair's second payload (the viewer holds both and toggles with d/l); the viewer never derives one from the other, so the producer resolves both paths. Omit it for a single-variant push.")]
    #[serde(default)]
    pub dark: Option<String>,
    #[schemars(description = "true = a fresh look (fit-to-window); false = iterate at the viewer's held zoom+pan. Set from conversational intent per the CLAUDE.md verb-table heuristic: a new or different image, or an explicit fresh look, is anew; tweaking the image already up is not. Defaults to a fresh look when omitted.")]
    #[serde(default)]
    pub anew: Option<bool>,
}

/// The one FROZEN control line: a `pbgvw_`-sprued JSON object, '\n'-terminated.
/// `anew` picks the wire verb (fresh vs update), `len` is the (light) payload
/// byte count, `dark_len` the optional dark-variant byte count. `pbgvw_dark_len`
/// rides only when `dark_len > 0`, so a single push stays byte-identical to the
/// pre-pair frame. Single home for the wire-format string — shared by the pusher
/// and the framing test, so a drift from paneboard's frozen contract fails it.
fn zjjrm_render_control(anew: bool, len: usize, dark_len: usize) -> String {
    let verb = if anew { "pbgvw_fresh" } else { "pbgvw_update" };
    if dark_len > 0 {
        format!(
            "{{\"pbgvw_verb\":\"{}\",\"pbgvw_id\":0,\"pbgvw_len\":{},\"pbgvw_dark_len\":{}}}\n",
            verb, len, dark_len
        )
    } else {
        format!("{{\"pbgvw_verb\":\"{}\",\"pbgvw_id\":0,\"pbgvw_len\":{}}}\n", verb, len)
    }
}

/// Push one image (plus the optional dark variant) to the running viewer over
/// the frozen `pbgvw_` wire. `Ok((bytes, dark_bytes, port))` on a landed push
/// (`dark_bytes` is 0 for a single-variant push); `Err(reason)` on any soft
/// failure (no port-file, unreachable viewer, unreadable image). The caller
/// renders either as a success-result notice — `vvx_render` never errors.
fn zjjrm_push_viewer(
    light: &Path,
    dark: Option<&Path>,
    anew: bool,
) -> Result<(usize, usize, u16), String> {
    use std::io::Write;

    let home = std::env::var_os("HOME").ok_or("HOME is unset")?;
    let port_path = PathBuf::from(home).join(JJRM_VIEWER_PORT_TAIL);
    let port_text = std::fs::read_to_string(&port_path)
        .map_err(|e| format!("read port-file {}: {e}", port_path.display()))?;
    let port: u16 = port_text
        .trim()
        .parse()
        .map_err(|e| format!("bad port in {}: {e}", port_path.display()))?;

    let bytes = std::fs::read(light).map_err(|e| format!("read {}: {e}", light.display()))?;
    let dark_bytes = match dark {
        Some(d) => Some(std::fs::read(d).map_err(|e| format!("read {}: {e}", d.display()))?),
        None => None,
    };
    let dark_len = dark_bytes.as_ref().map_or(0, Vec::len);

    let mut stream = std::net::TcpStream::connect(("127.0.0.1", port))
        .map_err(|e| format!("connect 127.0.0.1:{port}: {e}"))?;
    let control = zjjrm_render_control(anew, bytes.len(), dark_len);
    stream
        .write_all(control.as_bytes())
        .map_err(|e| format!("write control: {e}"))?;
    stream
        .write_all(&bytes)
        .map_err(|e| format!("write payload: {e}"))?;
    if let Some(d) = &dark_bytes {
        stream
            .write_all(d)
            .map_err(|e| format!("write dark payload: {e}"))?;
    }
    stream.flush().map_err(|e| format!("flush: {e}"))?;
    Ok((bytes.len(), dark_len, port))
}

/// Build the fail-soft report for one render call — always a success-result
/// string (the tool never errors). Names the landed push (light, plus the dark
/// variant when paired) or the soft cause.
fn zjjrm_render_report(p: &jjrm_RenderParams) -> String {
    let anew = p.anew.unwrap_or(true);
    let verb = if anew { "fresh" } else { "update" };
    let dark_path = p.dark.as_deref().map(Path::new);
    let dark_note = match &p.dark {
        Some(d) => format!(" (dark variant {d} supplied)"),
        None => String::new(),
    };
    match zjjrm_push_viewer(Path::new(&p.light), dark_path, anew) {
        Ok((n, dark_n, port)) => {
            let pair = if dark_n > 0 {
                format!(" + dark {dark_n} bytes")
            } else {
                String::new()
            };
            format!(
                "unfurled {} ({} bytes{}, {}) to the viewer on 127.0.0.1:{port}",
                p.light, n, pair, verb
            )
        }
        Err(e) => format!(
            "viewer push failed soft: {e}{} — bring the viewer up (paneboard conducts it), then retry",
            dark_note
        ),
    }
}

// ============================================================================
// Officium lifecycle — jjdxo_* (Officium Lifecycle in JJS0)
// ============================================================================

/// Reap stale officium directories by heartbeat mtime.
/// Returns (reaped_count, active_count) for summary reporting.
fn zjjrm_exsanguinate(officia: &Path) -> (usize, usize) {
    let entries = match std::fs::read_dir(officia) {
        Ok(e) => e,
        Err(_) => return (0, 0),
    };
    let now = std::time::SystemTime::now();
    let mut reaped: usize = 0;
    let mut active: usize = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let dir_name = match path.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => continue,
        };
        if dir_name.starts_with('.') { continue; }
        let heartbeat = path.join(HEARTBEAT_FILE);
        if !heartbeat.exists() {
            // No heartbeat file — legacy or corrupt. Log warning, skip.
            eprintln!("jjx exsanguinate: no heartbeat in {:?}, skipping", path.file_name());
            continue;
        }
        let mtime = match heartbeat.metadata().and_then(|m| m.modified()) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let age = match now.duration_since(mtime) {
            Ok(d) => d,
            Err(_) => continue,
        };
        if age.as_secs() > EXSANGUINATION_THRESHOLD_SECS {
            if let Err(e) = std::fs::remove_dir_all(&path) {
                eprintln!("jjx exsanguinate: failed to remove {:?}: {}", path.file_name(), e);
            } else {
                reaped += 1;
            }
        } else {
            active += 1;
        }
    }
    (reaped, active)
}

/// Short random discriminant for officium IDs: `len` lowercase-alphanumeric chars.
///
/// The daily ordinal NNNN is seeded per-machine (scan of the local officia dir),
/// so two machines both mint `…-1000` as their first-of-day officium — a
/// structural cross-machine collision independent of timing. This suffix breaks
/// it: each char is drawn from std's `RandomState`, which seeds from the OS
/// CSPRNG per process, so two machines draw independent discriminants. No `rand`
/// dependency for what is a four-character need.
fn zjjrm_random_suffix(len: usize) -> String {
    use std::hash::{BuildHasher, Hasher};
    const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let seed = std::collections::hash_map::RandomState::new();
    (0..len)
        .map(|i| {
            let mut hasher = seed.build_hasher();
            hasher.write_usize(i);
            ALPHABET[(hasher.finish() % ALPHABET.len() as u64) as usize] as char
        })
        .collect()
}

/// Generate officium ID: YYMMDD-NNNN-RAND (autonumber + random discriminant).
///
/// NNNN is a per-machine daily ordinal seeded by scanning the local officia dir;
/// RAND is a cross-machine collision-breaker (see `zjjrm_random_suffix`). The
/// scan must read only the NNNN segment — a dir is now `NNNN-RAND`, so parsing
/// the whole post-date field would fail on every new-format dir and re-mint 1000
/// forever; `split('-').next()` recovers the ordinal from both formats.
fn zjjrm_generate_officium_id(officia: &Path, today: &str) -> String {
    let prefix = format!("{}-", today);
    // Seed one below the first ordinal so the first mint of the day lands on it.
    let mut max_num: u32 = OFFICIUM_FIRST_ORDINAL - 1;
    if let Ok(entries) = std::fs::read_dir(officia) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(rest) = name.strip_prefix(&prefix) {
                let ordinal = rest.split('-').next().unwrap_or(rest);
                if let Ok(num) = ordinal.parse::<u32>() {
                    if num > max_num { max_num = num; }
                }
            }
        }
    }
    format!("{}{:04}-{}", prefix, max_num + 1, zjjrm_random_suffix(OFFICIUM_SUFFIX_LEN))
}

/// Check if daily probe is needed (probe_date file doesn't match today).
fn zjjrm_needs_probe(probe_date_path: &Path, today: &str) -> bool {
    match std::fs::read_to_string(probe_date_path) {
        Ok(content) => content.trim() != today,
        Err(_) => true,
    }
}

/// Encode a cwd path to its `~/.claude/projects/<encoded>/` directory name.
///
/// Replacement rules: both `/` and `_` map to `-`. All other characters pass through.
fn zjjrm_encode_cwd(cwd: &Path) -> String {
    cwd.to_string_lossy()
        .chars()
        .map(|c| match c {
            '/' | '_' => '-',
            c => c,
        })
        .collect()
}

/// Resolve current Claude Code session UUID from the transcript directory for `cwd`.
///
/// Lists `~/.claude/projects/<encoded-cwd>/*.jsonl`, picks newest by mtime,
/// returns the basename without `.jsonl`. Fails on missing dir, zero `.jsonl`
/// files, or mtime tie between the top two candidates — silent fallback would
/// bind the wrong session UUID into a durable git commit.
fn zjjrm_resolve_session_uuid(cwd: &Path) -> Result<String, String> {
    let home = std::env::var_os("HOME")
        .ok_or_else(|| "HOME env var not set".to_string())?;
    let projects_dir = PathBuf::from(home)
        .join(".claude")
        .join("projects")
        .join(zjjrm_encode_cwd(cwd));

    if !projects_dir.is_dir() {
        return Err(format!(
            "Claude transcripts dir not found: {}",
            projects_dir.display()
        ));
    }

    let entries = std::fs::read_dir(&projects_dir)
        .map_err(|e| format!("read_dir({}): {}", projects_dir.display(), e))?;

    let mut candidates: Vec<(std::time::SystemTime, String)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let Some(uuid) = name_str.strip_suffix(".jsonl") else { continue };
        let Ok(metadata) = entry.metadata() else { continue };
        let Ok(mtime) = metadata.modified() else { continue };
        candidates.push((mtime, uuid.to_string()));
    }

    if candidates.is_empty() {
        return Err(format!(
            "No transcripts under {}",
            projects_dir.display()
        ));
    }

    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    if candidates.len() >= 2 && candidates[0].0 == candidates[1].0 {
        return Err(format!(
            "mtime tie between transcripts under {} — cannot disambiguate session",
            projects_dir.display()
        ));
    }

    Ok(candidates.swap_remove(0).1)
}

/// iTerm window-reference scheme — the typed namespace under which an emblem
/// keys to its window. The reference is always scheme-qualified
/// (`iterm-session/<window-id>`, the value being the iTerm window's CGWindowID
/// — the same integer paneboard enumerates, so paneboard reads the emblem by
/// it). The namespace generalizes to other window types (Terminal.app, Windows
/// Terminal, ...) by adding a sibling scheme rather than overloading this one.
pub const JJRM_ITERM_SCHEME: &str = "iterm-session";

/// Emblem root tail, relative to `$HOME`: the paneboard-owned per-user
/// rendezvous directory emblems are written under. The full emblem file path is
/// `<home>/<this>/<scheme>/<value>.emblem`, and its body is the `pbge_` emblem
/// grammar (a gazette cousin). paneboard's PoC spec ("Emblem File Format") is
/// the authority for both the path literal and the grammar; we mirror them by
/// convention (no handshake) and cite that here so the duplication is a
/// deliberate mirror, not a second source.
pub const JJRM_EMBLEM_ROOT_TAIL: &str = ".config/paneboard/emblems";

/// Extract the bare session UUID from an `ITERM_SESSION_ID` value.
///
/// iTerm sets the variable as `<wNtNpN-position-prefix>:<UUID>`. We take the
/// UUID after the colon and discard the position prefix (it restamps on
/// tab-drag, and the index it carries is not the CGWindowID anyway). The UUID
/// is NOT the emblem key — it is the *question* vvx puts to iTerm to learn its
/// window id (`zjjrm_resolve_iterm_window_id`); the resolved window id is the
/// key, because that is the handle the sandboxed reader (paneboard) holds.
///
/// Pure (no env access) so the parse is unit-testable; `jjrm_iterm_window_ref`
/// wraps it with the environment read. Returns `None` on a value with no colon
/// or an empty UUID — fail-soft, treated as "not a usable iTerm session".
fn zjjrm_parse_iterm_uuid(raw: &str) -> Option<String> {
    let (_position_prefix, uuid) = raw.split_once(':')?;
    if uuid.is_empty() {
        return None;
    }
    Some(uuid.to_string())
}

/// A session UUID is hex digits and hyphens only. Guard before interpolating it
/// into the AppleScript text so a hostile `ITERM_SESSION_ID` cannot break out of
/// the quoted string; anything else folds to "not resolvable" (fail-soft).
fn zjjrm_is_plausible_uuid(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

/// Build the AppleScript that maps a session UUID to its containing window's id.
/// Pure (returns the script text) so the incantation is unit-testable without
/// spawning `osascript`. iTerm's AppleScript window id IS the CGWindowID
/// paneboard enumerates (confirmed by the grooming spike, 2026-06-22), so the
/// integer this resolves is exactly the key paneboard reads the emblem by.
fn zjjrm_iterm_resolve_script(session_uuid: &str) -> String {
    format!(
        "tell application \"iTerm2\"\n\
         repeat with w in windows\n\
         repeat with t in tabs of w\n\
         repeat with s in sessions of t\n\
         if (id of s) is \"{session_uuid}\" then return (id of w)\n\
         end repeat\n\
         end repeat\n\
         end repeat\n\
         end tell"
    )
}

/// Resolve, and cache, this process's iTerm window id by asking iTerm over
/// AppleScript. vvx is a non-sandboxed iTerm descendant, so this self-scripts
/// with no prompt — which is the whole reason the resolver lives here on the
/// writer and not in paneboard, whose sandbox forbids AppleEvents (paddock
/// "Resolver", JSS0 `jjdxw_marque`).
///
/// vvx is a long-running MCP server and a Claude Code session lives in one
/// window for its life, so the UUID->window-id map is stable: resolve once and
/// reuse, keeping the `osascript` round-trip off the per-command path. Only a
/// *success* is cached, so a transient failure retries on the next engagement.
/// Fail-soft: an implausible UUID or any `osascript` failure yields `None`.
fn zjjrm_resolve_iterm_window_id(session_uuid: &str) -> Option<u32> {
    static WINDOW_ID: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    if let Some(v) = WINDOW_ID.get() {
        return Some(*v);
    }
    if !zjjrm_is_plausible_uuid(session_uuid) {
        return None;
    }
    let out = std::process::Command::new("osascript")
        .arg("-e")
        .arg(zjjrm_iterm_resolve_script(session_uuid))
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let wid = String::from_utf8(out.stdout)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()?;
    let _ = WINDOW_ID.set(wid);
    Some(wid)
}

/// Derive this process's scheme-qualified iTerm window reference:
/// `iterm-session/<window-id>`, the value being the iTerm window's CGWindowID.
/// That is the handle paneboard enumerates, so it reads the emblem by the same
/// integer; the session UUID is used only transiently to ask iTerm and is never
/// persisted (see `zjjrm_resolve_iterm_window_id`).
///
/// Fail-soft: an unset `ITERM_SESSION_ID` (not under iTerm), a malformed value,
/// or an unresolvable window (osascript denied/failed) all fold to the same
/// `None`, and the caller writes no emblem and skips silently. Sibling to
/// `zjjrm_resolve_session_uuid` (the Claude Code session reader); the
/// env-read/parse/fallback idiom follows `jjrc_core`.
pub fn jjrm_iterm_window_ref() -> Option<String> {
    let raw = std::env::var("ITERM_SESSION_ID").ok()?;
    let uuid = zjjrm_parse_iterm_uuid(&raw)?;
    let window_id = zjjrm_resolve_iterm_window_id(&uuid)?;
    Some(format!("{}/{}", JJRM_ITERM_SCHEME, window_id))
}

/// Resolve the emblem root directory this process writes emblems into:
/// `$HOME/<JJRM_EMBLEM_ROOT_TAIL>`. The per-scheme subdirectory and
/// `<value>.emblem` basename are joined onto this by the writer from
/// `jjrm_iterm_window_ref`. Fail-soft: an unset `HOME` returns `None`, and the
/// caller skips — no emblem written.
pub fn jjrm_emblem_root() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(JJRM_EMBLEM_ROOT_TAIL))
}

/// Emblem file basename suffix. The full per-window path is
/// `<root>/<scheme>/<value>.emblem` (built by `jjrm_emblem_path`); paneboard's
/// PoC spec ("Emblem File Format") freezes the `.emblem` extension.
pub const JJRM_EMBLEM_SUFFIX: &str = "emblem";

/// rbm-side emblem STYLE config (JJS0 `jjdxw_emblem`: "an rbm-side style file
/// vvx reads at write time, never compiled in"), relative to the project root
/// (the server cwd). Edit it and the next jjx engagement repaints with the new
/// per-region style. An absent file or absent field folds to the built-in
/// paneboard default — the writer simply omits the attribute. Sibling to
/// `GALLOPS_PATH` under `.claude/jjm/`; the `jje_` file prefix is its home.
const EMBLEM_STYLE_PATH: &str = ".claude/jjm/jje_emblem.json";

/// Officium-resident marker recording what this session last *saddled* (mounted
/// or groomed): the work identity plus its resolved silks. Written by the mount
/// and groom arms — orient saddles the mounted pace's coronet (the resolved
/// next-actionable one, never the bare firemark lede) with its pace+heat silks,
/// show the heat firemark with heat silks only — and read by the emblem writer
/// so every later jjx engagement — `record`, `list`, `close`, … — paints that
/// identity, not just orient/show itself. Which identity is JJK's mount/groom
/// semantics, not the agent's lede choice (paddock "Emblem and window
/// reference"). The silks are resolved once at mount/groom and cached here, so
/// the per-engagement writer reads them without a gallops touch (they refresh
/// on the next mount/groom, the same staleness window as the identity itself).
/// Absent before the first mount: the emblem then degrades to the bare officium
/// identity (see `zjjrm_refresh_emblem`). Lives under the gitignored officia
/// tree, alongside `heartbeat`.
const SADDLE_MARKER_FILE: &str = "saddle";

/// The saddle marker's on-disk shape (JSON under `SADDLE_MARKER_FILE`). Carries
/// the glyph-prefixed work `identity` (coronet or firemark — the top/bottom
/// glance datum) plus the silks the emblem's middle band reads: `pace_silks` is
/// present only on a pace-mount (a coronet) and absent on a heat-mount/groom (a
/// firemark), so the middle is three lines on a pace and two on a heat;
/// `heat_silks` is always the owning heat's. Officium-resident scratch, not
/// gallops — so this format is free to change with no schema/reprieve concern,
/// and a stale or pre-silks marker that fails to parse simply degrades to the
/// bare officium handle until the next mount rewrites it.
#[derive(serde::Serialize, serde::Deserialize)]
struct jjrm_SaddleMarker {
    identity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pace_silks: Option<String>,
    #[serde(default)]
    heat_silks: String,
}

/// Compose the per-window emblem file path from the emblem root and a
/// scheme-qualified window reference. The reference carries its own `/`
/// (`iterm-session/<window-id>`), so the join lands the file under the scheme
/// subdirectory: `<root>/iterm-session/<window-id>.emblem`. One home for the
/// path literal, shared by the writer and the `vvx_emblem_probe` diagnostic.
pub fn jjrm_emblem_path(root: &Path, window_ref: &str) -> PathBuf {
    root.join(format!("{}.{}", window_ref, JJRM_EMBLEM_SUFFIX))
}

/// Per-region style (a font size and a color), each independently optional.
/// Sourced from `EMBLEM_STYLE_PATH`; an absent field stays `None` and the
/// writer omits the corresponding `pbge_` attribute so paneboard supplies its
/// built-in default. Unknown JSON fields are ignored (forward-compatible).
#[derive(Default, serde::Deserialize)]
struct jjrm_RegionStyle {
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    size: Option<u32>,
}

/// The three frozen emblem regions' styles. Each defaults to empty (all
/// paneboard defaults), so a partial config — or none — parses clean.
#[derive(Default, serde::Deserialize)]
struct jjrm_EmblemStyle {
    #[serde(default)]
    top: jjrm_RegionStyle,
    #[serde(default)]
    middle: jjrm_RegionStyle,
    #[serde(default)]
    bottom: jjrm_RegionStyle,
}

/// Load the emblem style config from `<project_root>/EMBLEM_STYLE_PATH`.
/// Fail-soft: a missing file or any parse error yields the all-default style
/// (every field `None` → paneboard defaults). Never errors — style is a
/// best-effort overlay, never a precondition for writing the emblem.
fn zjjrm_load_emblem_style(project_root: &Path) -> jjrm_EmblemStyle {
    std::fs::read_to_string(project_root.join(EMBLEM_STYLE_PATH))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Normalize a raw identity (firemark or coronet, with or without its glyph) to
/// its full glyph-prefixed display form — "never abbreviated" (JJS0
/// `jjdxw_emblem`). A 2-char body is a firemark (`₣`), a 5-char body a coronet
/// (`₢`); any other length passes through unchanged, so degraded input is shown
/// as-is rather than mislabeled.
fn zjjrm_normalize_identity(raw: &str) -> String {
    let body = raw
        .trim()
        .trim_start_matches(crate::jjrf_favor::JJRF_FIREMARK_PREFIX)
        .trim_start_matches(crate::jjrf_favor::JJRF_CORONET_PREFIX);
    match body.chars().count() {
        crate::jjrf_favor::JJRF_FIREMARK_LEN => format!("{}{}", crate::jjrf_favor::JJRF_FIREMARK_PREFIX, body),
        crate::jjrf_favor::JJRF_CORONET_LEN => format!("{}{}", crate::jjrf_favor::JJRF_CORONET_PREFIX, body),
        _ => body.to_string(),
    }
}

/// Derive the heat firemark (full glyph-prefixed display form) from a halter
/// lede — the groom emblem identity. A coronet lede folds to its parent
/// firemark; a firemark lede passes through. Fail-soft: an unparseable lede
/// yields `None`, so the emblem keeps its standing marker rather than saddle a
/// bad identity. The grooming counterpart of orient's resolved-coronet
/// derivation (paddock "Emblem and window reference": coronet when mounted on a
/// pace, else the heat firemark).
fn zjjrm_lede_firemark(lede: &str) -> Option<String> {
    let body = lede
        .trim()
        .trim_start_matches(crate::jjrf_favor::JJRF_FIREMARK_PREFIX)
        .trim_start_matches(crate::jjrf_favor::JJRF_CORONET_PREFIX);
    match body.chars().count() {
        crate::jjrf_favor::JJRF_FIREMARK_LEN => crate::jjrf_favor::jjrf_Firemark::jjrf_parse(lede).ok().map(|f| f.jjrf_display()),
        crate::jjrf_favor::JJRF_CORONET_LEN => crate::jjrf_favor::jjrf_Coronet::jjrf_parse(lede).ok().map(|c| c.jjrf_parent_firemark().jjrf_display()),
        _ => None,
    }
}

/// Emit one `pbge_region` block: the `### pbge_region {…}` lede plus its text
/// lines. Returns empty when `lines` is empty — a region with no content is
/// omitted entirely (placement is explicit in the lede, so order and absence
/// are both insignificant to the reader). `pbge_color` / `pbge_size` ride only
/// when the config supplied them; otherwise the attribute is absent and
/// paneboard defaults it.
fn zjjrm_region_block(location: &str, style: &jjrm_RegionStyle, lines: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut attrs = format!("pbge_location={}", location);
    if let Some(ref c) = style.color {
        attrs.push_str(&format!(", pbge_color={}", c));
    }
    if let Some(n) = style.size {
        attrs.push_str(&format!(", pbge_size={}", n));
    }
    let mut out = format!("### pbge_region {{{}}}\n", attrs);
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Compose the full `pbge_` emblem document (paneboard PoC "Emblem File
/// Format") for one window. Pure — no env, no I/O — so the grammar is
/// unit-testable. `identity` is the already-normalized work-identity line (the
/// primary glance datum); it fills both the top and bottom regions so it reads
/// from all four corners of the box. The middle band is human-readable context,
/// top-down: the cwd `basename` (always), then the `pace` `(coronet, silks)`
/// (only when mounted on a pace — absent on a heat-mount/groom, so the band is
/// two lines not three), then the `heat` `(firemark, silks)` (whenever known).
/// Each silks line is prefixed with its own glyph-identity (`₢…: silks` /
/// `₣…: silks`) so the line says both what it is and which it is. Region CONTENT
/// here is the soft, config-tunable set; region STRUCTURE is frozen.
fn zjjrm_compose_emblem(
    window_ref: &str,
    identity: &str,
    basename: &str,
    pace: Option<(&str, &str)>,
    heat: Option<(&str, &str)>,
    style: &jjrm_EmblemStyle,
) -> String {
    let mut out = String::from("# pbge_emblem\n");
    out.push_str(&format!("## pbge_pane {}\n", window_ref));
    out.push_str(&zjjrm_region_block(
        "pbge_top",
        &style.top,
        &[identity.to_string()],
    ));
    // Middle band: basename, then the pace silks (pace-mount only), then the
    // heat silks — each silks line prefixed with its glyph-identity. An absent
    // silks line is simply omitted; a heat-mount has no pace line, so the band
    // reads two lines instead of three.
    let mut middle = vec![basename.to_string()];
    if let Some((coronet, silks)) = pace {
        middle.push(format!("{}: {}", coronet, silks));
    }
    if let Some((firemark, silks)) = heat {
        middle.push(format!("{}: {}", firemark, silks));
    }
    out.push_str(&zjjrm_region_block("pbge_middle", &style.middle, &middle));
    // Bottom mirrors the top identity so the coronet/firemark reads from all four
    // corners of the box: paneboard paints pbge_top into both top corners and
    // pbge_bottom into both bottom corners. Same identity line, styled by
    // style.bottom.
    out.push_str(&zjjrm_region_block(
        "pbge_bottom",
        &style.bottom,
        &[identity.to_string()],
    ));
    out
}

/// Resolve the saddle marker — identity plus its silks — for a freshly mounted
/// or groomed work identity, by looking the silks up in the gallops once at the
/// mount/groom moment (paddock "Emblem and window reference"). A coronet yields
/// both its pace silks (the head tack) and its parent heat's silks; a firemark
/// yields heat silks only (no pace line). Fail-soft: a gallops load failure, an
/// unparseable identity, or one absent from the gallops yields the identity with
/// no silks, so the emblem still paints the glyph. The result is cached in the
/// marker so the per-engagement writer never re-touches the gallops.
fn zjjrm_resolve_saddle_marker(identity: &str) -> jjrm_SaddleMarker {
    use crate::jjrf_favor::{jjrf_Firemark as Firemark, jjrf_Coronet as Coronet, JJRF_FIREMARK_LEN, JJRF_CORONET_LEN};
    let mut marker = jjrm_SaddleMarker {
        identity: zjjrm_normalize_identity(identity),
        pace_silks: None,
        heat_silks: String::new(),
    };
    let gallops = match crate::jjrg_gallops::jjrg_Gallops::jjrg_load(&gallops_pathbuf()) {
        Ok(g) => g,
        Err(_) => return marker,
    };
    let body = identity
        .trim()
        .trim_start_matches(crate::jjrf_favor::JJRF_FIREMARK_PREFIX)
        .trim_start_matches(crate::jjrf_favor::JJRF_CORONET_PREFIX);
    let (heat_key, coronet_key): (String, Option<String>) = match body.chars().count() {
        JJRF_CORONET_LEN => match Coronet::jjrf_parse(identity) {
            Ok(c) => (c.jjrf_parent_firemark().jjrf_display(), Some(c.jjrf_display())),
            Err(_) => return marker,
        },
        JJRF_FIREMARK_LEN => match Firemark::jjrf_parse(identity) {
            Ok(f) => (f.jjrf_display(), None),
            Err(_) => return marker,
        },
        _ => return marker,
    };
    if let Some(heat) = gallops.heats.get(&heat_key) {
        marker.heat_silks = heat.silks.clone();
        if let Some(ref ck) = coronet_key {
            if let Some(pace) = heat.paces.get(ck) {
                if let Some(tack) = pace.tacks.first() {
                    marker.pace_silks = Some(tack.silks.clone());
                }
            }
        }
    }
    marker
}

/// True when this officium's standing saddle marker already holds a *coronet*
/// identity — the coronet-sticks guard (JJS0 `jjdxw_emblem`): once a mount has
/// saddled a coronet, a later groom must not demote it to the heat
/// firemark. A coronet is exactly what the `jjrf_Coronet` parser accepts (glyph
/// stripped, five base64 chars in the charset), so the parser is the
/// discriminator — a firemark identity fails it on the `₣` glyph. A missing,
/// empty, unparseable, or firemark-identity marker all yield false — the slot
/// is fillable, so the groom saddles normally. Officium-scoped, like the marker
/// itself.
fn zjjrm_standing_is_coronet(officium_dir: &Path) -> bool {
    std::fs::read_to_string(officium_dir.join(SADDLE_MARKER_FILE))
        .ok()
        .and_then(|s| serde_json::from_str::<jjrm_SaddleMarker>(&s).ok())
        .map(|m| crate::jjrf_favor::jjrf_Coronet::jjrf_parse(m.identity.trim()).is_ok())
        .unwrap_or(false)
}

/// Atomically write the emblem file: mkdir the scheme tree, write a sibling
/// temp file, then rename over the target (rename is atomic within a
/// filesystem, so paneboard never reads a half-written emblem). Every step is
/// `?`-propagated to one `io::Result`; the sole caller discards it — a refused
/// or absent emblem tree must never surface as a jjx error.
fn zjjrm_write_emblem_atomic(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, contents)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// Best-effort, fail-soft emblem write for this window (JJS0 `jjdxw_emblem`;
/// paddock "Transport" — the FILE channel, never a socket). Called once per jjx
/// engagement from the single dispatcher entry, after the model/officium gates.
/// Every step degrades to "write nothing, surface nothing": not under iTerm
/// (`ITERM_SESSION_ID` unset), `HOME` unset, an unwritable emblem tree — all
/// fold to a silent no-op, so a jjx command returns its normal result with no
/// added error and only a microsecond file touch.
///
/// `new_marker` is the freshly-resolved saddle marker to persist. The mounting
/// and grooming commands build it from their own resolution and pass it in AFTER
/// they run — orient the resolved next-actionable coronet (else the heat firemark
/// when the heat has no actionable pace) with pace+heat silks, show the heat
/// firemark with heat silks; the up-front per-engagement call and every other
/// command (and `jjx_open`) pass `None`. When present it overwrites the
/// officium's `SADDLE_MARKER_FILE`; the emblem is then composed from whatever
/// marker stands there, degrading to the bare officium handle (the `☉` session
/// id) with no silks when no mount has happened yet.
fn zjjrm_refresh_emblem(officium_dir: &Path, new_marker: Option<jjrm_SaddleMarker>) {
    // Persist a freshly-resolved marker (orient/show only); best-effort. Recorded
    // even when no emblem can be painted — the mount happened regardless.
    if let Some(ref m) = new_marker {
        if let Ok(json) = serde_json::to_string(m) {
            let _ = std::fs::write(officium_dir.join(SADDLE_MARKER_FILE), json.as_bytes());
        }
    }

    // Window reference + emblem root: either absent ⇒ not paintable, skip.
    let Some(window_ref) = jjrm_iterm_window_ref() else {
        return;
    };
    let Some(root) = jjrm_emblem_root() else {
        return;
    };
    let Ok(cwd) = std::env::current_dir() else {
        return;
    };

    // Standing marker (JSON). A missing, empty, or pre-silks-format file fails
    // to parse and folds to None → bare officium handle, no silks.
    let marker: Option<jjrm_SaddleMarker> =
        std::fs::read_to_string(officium_dir.join(SADDLE_MARKER_FILE))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok());

    let identity = match &marker {
        Some(m) if !m.identity.trim().is_empty() => zjjrm_normalize_identity(m.identity.trim()),
        _ => {
            let bare = officium_dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            format!("{}{}", OFFICIUM_SUN_PREFIX, bare)
        }
    };
    let basename = cwd
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    // The pace line's identity is the current identity itself (a coronet when
    // pace_silks is present, i.e. a pace-mount); the heat line's identity is the
    // owning heat firemark, derived from the identity (coronet → parent, firemark
    // → self) and fail-soft to None on the bare-officium no-mount case.
    let firemark = zjjrm_lede_firemark(&identity);
    let pace_silks = marker.as_ref().and_then(|m| m.pace_silks.clone());
    let heat_silks = marker
        .as_ref()
        .map(|m| m.heat_silks.clone())
        .filter(|s| !s.is_empty());
    let pace = pace_silks.as_deref().map(|s| (identity.as_str(), s));
    let heat = match (firemark.as_deref(), heat_silks.as_deref()) {
        (Some(fm), Some(s)) => Some((fm, s)),
        _ => None,
    };
    let style = zjjrm_load_emblem_style(&cwd);
    let body = zjjrm_compose_emblem(
        &window_ref,
        &identity,
        &basename,
        pace,
        heat,
        &style,
    );

    let path = jjrm_emblem_path(&root, &window_ref);
    let _ = zjjrm_write_emblem_atomic(&path, &body);
}

/// Validate officium directory exists and touch heartbeat.
fn zjjrm_validate_officium(officium: &str) -> Result<(), String> {
    let bare_id = officium.trim_start_matches(OFFICIUM_SUN_PREFIX);
    let exchange = PathBuf::from(OFFICIA_DIR).join(bare_id);
    if !exchange.is_dir() {
        return Err(format!(
            "Officium directory not found: {}. Call jjx_open to create a new officium.",
            bare_id
        ));
    }
    // Touch heartbeat
    std::fs::write(exchange.join(HEARTBEAT_FILE), b"").ok();
    Ok(())
}

/// Resolve an officium ID to its absolute exchange directory.
///
/// `OFFICIA_DIR` is relative to the server's working directory; canonicalize
/// turns it absolute so the gazette paths we hand back are unambiguous no
/// matter what the agent's own working directory is — the agent uses the
/// emitted path verbatim and never reconstructs it from the id. Reconstruction
/// is exactly the trap the ☉-glyph strip set: the id carries the glyph, the
/// on-disk dir does not, and a hand-built path lands in a sibling that does not
/// exist. jjx_open creates this directory and every gazette consumer validates
/// it first, so canonicalize normally succeeds; on failure we fall back to the
/// cwd-joined relative path rather than break gazette I/O.
fn zjjrm_exchange_dir(officium: &str) -> std::path::PathBuf {
    let bare_id = officium.trim_start_matches(OFFICIUM_SUN_PREFIX);
    let relative = PathBuf::from(OFFICIA_DIR).join(bare_id);
    std::fs::canonicalize(&relative).unwrap_or_else(|_| {
        std::env::current_dir()
            .map(|cwd| cwd.join(&relative))
            .unwrap_or(relative)
    })
}

/// Resolve officium ID to absolute gazette input file path (agent → server).
fn zjjrm_gazette_in_path(officium: &str) -> std::path::PathBuf {
    zjjrm_exchange_dir(officium).join(GAZETTE_IN_FILE)
}

/// Resolve officium ID to absolute gazette output file path (server → agent).
fn zjjrm_gazette_out_path(officium: &str) -> std::path::PathBuf {
    zjjrm_exchange_dir(officium).join(GAZETTE_OUT_FILE)
}

/// Format the gazette path pair for emission to the agent.
///
/// jjx_open and jjx_orient both surface this block so the agent reads
/// `gazette_out` and writes `gazette_in` at the exact paths the server uses,
/// rather than constructing them from the officium id.
fn zjjrm_gazette_paths_block(
    gazette_in: &std::path::Path,
    gazette_out: &std::path::Path,
) -> String {
    format!(
        "gazette_in:  {}\ngazette_out: {}",
        gazette_in.display(),
        gazette_out.display(),
    )
}

/// Reject a param-supplied target on the gazette-only read paths (orient, show).
///
/// Target selection on these two commands now comes solely from `gazette_in.md`
/// via halter notices — the forced gazette write is the whole point, dragging the
/// agent's first `Write` permission to the start of the mount/groom ceremony. A
/// `firemark` or `targets` param is therefore *rejected*, not a silent fallback;
/// a soft convention would let the write slip. Returns the offending key (presence
/// alone, value irrelevant) so the caller can name it in the refusal.
fn zjjrm_rejected_target_param(v: &serde_json::Value) -> Option<&'static str> {
    if v.get("firemark").is_some() {
        Some("firemark")
    } else if v.get("targets").is_some() {
        Some("targets")
    } else {
        None
    }
}

/// Best-effort, read-only reprieve nag for jjx_open.
///
/// Reads the on-disk Gallops without the commit lock and emits one status line per registered
/// reprieve episode — `<label> <episode>: <verdict> (<rivet>)`, e.g.
/// `reprieve V3→V4: dormant (JJr_a7c)` — carrying the opaque rivet token as a grep/census
/// surface: pending when the tolerance is still load-bearing on this install, dormant when the
/// store is already canonical for that episode (a removal candidate once every clone agrees).
/// Non-gating by contract — any read or parse failure is silently skipped so jjx_open always
/// succeeds. The lockless peek sees only whole files (jjdr_save renames atomically) and mutates
/// nothing.
fn zjjrm_reprieve_nag(output: &mut vvc::vvco_Output) {
    let path = gallops_pathbuf();
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => return,
    };
    let gallops: crate::jjrt_types::jjrg_Gallops = match serde_json::from_slice(&bytes) {
        Ok(g) => g,
        Err(_) => return,
    };
    for status in crate::jjri_io::jjdz_probe(&gallops, &bytes) {
        // Legible label + inline gloss + opaque rivet token (JJr_a7c) — the rivet rides the
        // emission as a grep surface; the label and gloss read at sight (the jjx_open echo of
        // JDG JDo_101), so the verdict is actionable without opening code or spec.
        vvco_out!(
            output,
            "{} {}: {} — {} ({})",
            crate::jjri_io::JJDZ_LABEL_REPRIEVE,
            status.label,
            status.jjdz_verdict(),
            status.jjdz_gloss(),
            crate::jjri_io::JJDZ_RIVET_REPRIEVE
        );
    }
}

/// Best-effort, read-only retention monitum for jjx_open — the second instance of the open-time
/// monitum (JJS0 `jjdz_monitum`), sibling to the reprieve nag above and deliberately independent of
/// it (no shared monitum abstraction until a third instance earns it).
///
/// Reads the on-disk Gallops without the commit lock and reports the chat-history retention policy
/// in three states (`jjri_retention_state`): Off emits nothing (the quiet shareable default), On
/// prints the since-date, Malformed prints loud and notes capture is disabled. Non-gating by
/// contract — any read or parse failure is silently skipped and bad config never blocks open, since
/// a malformed date is a read-time classification, not a parse error. The lockless peek sees only
/// whole files (jjdr_save renames atomically) and mutates nothing.
fn zjjrm_retention_monitum(output: &mut vvc::vvco_Output) {
    let path = gallops_pathbuf();
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => return,
    };
    let gallops: crate::jjrt_types::jjrg_Gallops = match serde_json::from_slice(&bytes) {
        Ok(g) => g,
        Err(_) => return,
    };
    match crate::jjri_io::jjri_retention_state(&gallops) {
        crate::jjri_io::jjri_RetentionState::Off => {}
        crate::jjri_io::jjri_RetentionState::On(date) => {
            vvco_out!(output, "retention: on since {}", date);
        }
        crate::jjri_io::jjri_RetentionState::Malformed(raw) => {
            vvco_out!(output, "retention: MALFORMED date \"{}\" — capture disabled", raw);
        }
    }
}

/// The jj-lifecycle files whose git state the open ceremony manages. For now the gallops
/// alone; the chat-history store joins this set when capture lands. Open requires every
/// managed file to be pristine before it proceeds — a staged or conflicted store is never
/// legitimate and must be resolved by hand, not ridden over.
fn zjjrm_managed_files() -> Vec<String> {
    vec![gallops_pathbuf().to_string_lossy().to_string()]
}

/// Always-gate: refuse the open ceremony if any managed file is dirty versus HEAD.
///
/// Determination is `git status --porcelain` per managed path: any output means a staged,
/// unstaged, or conflicted change, so open refuses with the porcelain shown. Fail-safe — if
/// the status command itself cannot run, the gate passes, so it only ever refuses on a
/// positively-determined dirty file (a plumbing failure must never brick officium open).
fn zjjrm_managed_clean() -> Result<(), String> {
    for path in zjjrm_managed_files() {
        let out = match vvc::vvce_git_command(&["status", "--porcelain", "--", path.as_str()]).output() {
            Ok(o) if o.status.success() => o,
            _ => continue,
        };
        let status = String::from_utf8_lossy(&out.stdout);
        if !status.trim().is_empty() {
            return Err(format!("managed store not pristine ({}): {}", path, status.trim()));
        }
    }
    Ok(())
}

/// Restore a managed file to HEAD and unstage it — the revert after an over-budget open
/// commit, so a blocked convergence never leaves the store staged-but-uncommitted.
fn zjjrm_revert_managed(path: &str) {
    let _ = vvc::vvce_git_command(&["checkout", "HEAD", "--", path]).output();
    let _ = vvc::vvce_git_command(&["reset", "--quiet", "--", path]).output();
}

/// Handle jjx_open: create a new officium.
///
/// `size_limit` is the convergence budget. 0 (the standing default) means no mutation — the
/// lockless empty-invitatory open we have by default. A value > 0 opts the ceremony into a
/// bulk-authorized convergence commit (reprieve conversions now; chat capture later),
/// gated by that budget; over budget hard-fails with the required size, reverts, and delivers
/// no officium.
async fn zjjrm_handle_open(size_limit: u64) -> Result<CallToolResult, McpError> {
    let cn = JJRM_CMD_NAME_OPEN;
    let mut output = vvc::vvco_Output::buffer();

    // Disk space guard — block before any state changes
    match crate::jjrdk_diskcheck::jjrdk_check_disk_space() {
        Ok(survey) => vvco_out!(output, "{}", survey),
        Err(msg) => return Ok(CallToolResult::error(vec![Content::text(msg)])),
    }

    // Always-gate: a staged or conflicted managed store is never legitimate — refuse before
    // creating any officium, so the failure is clean and no officium is delivered.
    if let Err(e) = zjjrm_managed_clean() {
        return Ok(CallToolResult::error(vec![Content::text(
            format!("{}: refusing open — {}", cn, e),
        )]));
    }

    let officia = PathBuf::from(OFFICIA_DIR);
    if let Err(e) = std::fs::create_dir_all(&officia) {
        return Ok(CallToolResult::error(vec![Content::text(
            format!("{}: error creating officia dir: {}", cn, e),
        )]));
    }

    // Exsanguinate stale officia before creating new one
    let (reaped, active) = zjjrm_exsanguinate(&officia);

    // Generate officium ID and atomically claim the exchange directory.
    // create_dir (not create_dir_all) fails with AlreadyExists if another
    // session raced to the same ID — retry with next autonumber.
    let today = chrono::Local::now().format("%y%m%d").to_string();
    let (id, exchange) = loop {
        let candidate = zjjrm_generate_officium_id(&officia, &today);
        let path = officia.join(&candidate);
        match std::fs::create_dir(&path) {
            Ok(()) => break (candidate, path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("{}: error creating exchange dir: {}", cn, e),
                )]));
            }
        }
    };
    // No gazette files created at open — they are ephemeral, single-MCP-call lifetime.
    std::fs::write(exchange.join(HEARTBEAT_FILE), b"").ok();

    // Degraded emblem paint: a freshly-opened officium has no saddled identity
    // yet, so the label shows the bare officium handle until the first mount.
    zjjrm_refresh_emblem(&exchange, None);

    // Resolve session UUID + cwd. Emitted on every invitatory body so the
    // chat that produced this commit can be located (`git log --grep='session: '`)
    // and resumed (`claude --resume <uuid>` from the recorded cwd).
    // Failure here aborts jjx_open — silently emitting a stale or wrong UUID
    // would poison the durable recall anchor.
    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("{}: current_dir error: {}", cn, e),
            )]));
        }
    };
    let session_uuid = match zjjrm_resolve_session_uuid(&cwd) {
        Ok(uuid) => uuid,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("{}: session UUID resolution failed: {}", cn, e),
            )]));
        }
    };
    let mut body = format!("session: {}\ncwd: {}", session_uuid, cwd.display());

    // Daily probe: append model inventory if .probe_date doesn't match today
    let probe_date_path = officia.join(PROBE_DATE_FILE);
    if zjjrm_needs_probe(&probe_date_path, &today) {
        match vvc::vvcp_probe().await {
            Ok(data) => {
                std::fs::write(&probe_date_path, &today).ok();
                body.push('\n');
                body.push_str(&data);
            }
            Err(e) => {
                eprintln!("{}: probe warning: {}", cn, e);
            }
        }
    }

    // Invitatory commit: jjb:BRAND::i: OFFICIUM <id>
    let brand = vvc::vvcc_get_brand();
    let subject = format!("OFFICIUM {}", id);
    let action = crate::jjrnm_markers::JJRNM_INVITATORY.to_string();
    let message = vvc::vvcc_format_branded(
        crate::jjrn_notch::JJRN_COMMIT_PREFIX,
        &brand,
        "",
        &action,
        &subject,
        Some(&body),
    );

    // size_limit > 0 opts this open into a bulk-authorized convergence commit: load (running any
    // pending reprieve conversion), save, and commit the gallops under the budget as the
    // invitatory. Over budget hard-fails — required size in the message, the store reverted, the
    // freshly-claimed officium rolled back — so a blocked convergence leaves no staged store.
    // size_limit == 0 keeps the default lockless empty-invitatory marker (open mutates nothing).
    let mut converged = false;
    if size_limit > 0 {
        let gallops_path = gallops_pathbuf();
        let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
            Ok(l) => l,
            Err(e) => {
                let _ = std::fs::remove_dir_all(&exchange);
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("{}: commit lock held: {} (break with `vvx vvx_unlock` in extremis)", cn, e),
                )]));
            }
        };
        let gallops = match crate::jjrg_gallops::jjrg_Gallops::jjrg_load(&gallops_path) {
            Ok(g) => g,
            Err(e) => {
                let _ = std::fs::remove_dir_all(&exchange);
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("{}: convergence load error: {}", cn, e),
                )]));
            }
        };
        if let Err(e) = crate::jjri_io::jjdr_save(&gallops, &gallops_path) {
            let _ = std::fs::remove_dir_all(&exchange);
            return Ok(CallToolResult::error(vec![Content::text(
                format!("{}: convergence save error: {}", cn, e),
            )]));
        }
        let path_str = gallops_path.to_string_lossy().to_string();
        let dirty = vvc::vvce_git_command(&["status", "--porcelain", "--", path_str.as_str()])
            .output()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(true);
        if dirty {
            let commit_args = vvc::vvcm_CommitArgs {
                files: vec![path_str.clone()],
                message: message.clone(),
                size_limit,
                warn_limit: vvc::VVCG_WARN_LIMIT,
            };
            let mut commit_out = vvc::vvco_Output::buffer();
            if let Err(e) = vvc::machine_commit(&lock, &commit_args, &mut commit_out) {
                zjjrm_revert_managed(&path_str);
                let _ = std::fs::remove_dir_all(&exchange);
                let detail = commit_out.vvco_finish();
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("{}: convergence over budget, reverted, no officium — {}\n{}", cn, e, detail),
                )]));
            }
            converged = true;
        }
        // lock drops here
    }

    if !converged {
        let marker_args = vvc::vvcc_MarkerArgs {
            prefix: None,
            message,
        };

        let mut marker_output = vvc::vvco_Output::buffer();
        if vvc::marker(&marker_args, &mut marker_output) != 0 {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("{}: invitatory commit error: {}", cn, marker_output.vvco_finish()),
            )]));
        }
    }

    if reaped > 0 || active > 0 {
        vvco_out!(output, "Exsanguination: {} active, {} reaped", active, reaped);
    }
    zjjrm_reprieve_nag(&mut output);
    zjjrm_retention_monitum(&mut output);
    vvco_out!(output, "{}{}", OFFICIUM_SUN_PREFIX, id);
    vvco_out!(output, "{}", zjjrm_gazette_paths_block(
        &zjjrm_gazette_in_path(&id),
        &zjjrm_gazette_out_path(&id),
    ));
    Ok(CallToolResult::success(vec![Content::text(output.vvco_finish())]))
}


// ============================================================================
// Model gate
// ============================================================================

/// Extract model tier from verbatim model ID string.
/// Returns "fable", "opus", "sonnet", "haiku", or "unknown".
fn zjjrm_extract_tier(model: &str) -> &'static str {
    let lower = model.to_ascii_lowercase();
    if lower.contains("fable") {
        "fable"
    } else if lower.contains("opus") {
        "opus"
    } else if lower.contains("sonnet") {
        "sonnet"
    } else if lower.contains("haiku") {
        "haiku"
    } else {
        "unknown"
    }
}

/// Gate check: require a frontier-tier model (opus or fable). Returns Err with diagnostic on failure.
fn zjjrm_check_model_gate(model: &str) -> Result<(), String> {
    let tier = zjjrm_extract_tier(model);
    if tier == "opus" || tier == "fable" {
        return Ok(());
    }
    Err(format!(
        "MODEL GATE — this command requires a frontier-tier model (opus or fable).\n\n  Received model: {}\n  Extracted tier: {}\n\nJob Jockey commands currently require a frontier-tier model.",
        model, tier
    ))
}

// ============================================================================
// MCP Server
// ============================================================================

#[derive(Debug, Clone)]
pub struct jjrm_McpServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl jjrm_McpServer {
    pub fn jjrm_new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(name = "jjx", description = "Job Jockey Kit - MCP tools for project initiative management")]
    async fn jjx(&self, Parameters(p): Parameters<jjrm_JjxParams>) -> Result<CallToolResult, McpError> {
        let cmd = p.command.as_str();

        // Model gate: required on all commands, checked first
        if let Err(msg) = zjjrm_check_model_gate(&p.model) {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: {}", cmd, msg),
            )]));
        }
        eprintln!("jjx {}: model={}", cmd, p.model);

        // jjx_open creates the officium — handle before officium validation.
        // size_limit (default 0) is the convergence budget; 0 means open mutates nothing.
        if cmd == JJRM_CMD_NAME_OPEN {
            // params may arrive stringified (the documented MCP quirk) — normalize before reading.
            let pv = match &p.params {
                serde_json::Value::String(s) => {
                    serde_json::from_str::<serde_json::Value>(s).unwrap_or_else(|_| p.params.clone())
                }
                _ => p.params.clone(),
            };
            let size_limit = pv.get("size_limit").and_then(|x| x.as_u64()).unwrap_or(0);
            return zjjrm_handle_open(size_limit).await;
        }

        // Officium envelope: required on all commands except jjx_open
        match p.officium {
            Some(ref officium) => {
                if let Err(e) = zjjrm_validate_officium(officium) {
                    return Ok(CallToolResult::error(vec![Content::text(
                        format!("jjx {}: {}", cmd, e),
                    )]));
                }
            }
            None => {
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("jjx {}: officium parameter required. Call jjx_open first to create an officium.", cmd),
                )]));
            }
        }
        // Universal entry rule: read+delete gazette_in, delete gazette_out.
        // Gazette content has single-MCP-call lifetime.
        let officium_id = p.officium.as_ref().unwrap();
        let gazette_in_path = zjjrm_gazette_in_path(officium_id);
        let gazette_out_path = zjjrm_gazette_out_path(officium_id);
        let gazette_in_content: Option<String> = {
            let content = std::fs::read_to_string(&gazette_in_path).ok();
            let _ = std::fs::remove_file(&gazette_in_path);
            content.filter(|s| !s.trim().is_empty())
        };
        let _ = std::fs::remove_file(&gazette_out_path);

        let v = match p.params {
            serde_json::Value::String(ref s) => {
                serde_json::from_str(s).unwrap_or(p.params)
            }
            other => other,
        };

        // Emblem overlay (best-effort, fail-soft): repaint this window's
        // work-identity label once per engagement, here at the single dispatcher
        // entry after the model/officium gates, reading whatever identity the
        // session last saddled. The mounting and grooming commands (orient,
        // show) re-derive and re-saddle the correct identity from their OWN
        // resolution after they run — orient the resolved next-actionable
        // coronet (never the bare firemark lede), show the heat firemark — so
        // which identity rides the emblem is JJK's mount/groom semantics, not
        // the agent's lede choice (paddock "Emblem and window reference"). The
        // call never affects the command result; see zjjrm_refresh_emblem.
        zjjrm_refresh_emblem(&zjjrm_exchange_dir(officium_id), None);

        macro_rules! deser {
            ($t:ty) => {
                match serde_json::from_value::<$t>(v) {
                    Ok(p) => p,
                    Err(e) => return jjrm_deser_error(cmd, e),
                }
            }
        }


        match cmd {
            JJRM_CMD_NAME_RECORD => {
                let p = deser!(jjrm_RecordParams);
                jjrm_result(jjrnc_run_notch(jjrnc_NotchArgs {
                    identity: p.identity,
                    files: p.files,
                    size_limit: p.size_limit,
                    intent: p.intent,
                }))
            }
            JJRM_CMD_NAME_LOG => {
                let p = deser!(jjrm_LogParams);
                jjrm_result(jjrrn_run_rein(jjrrn_ReinArgs {
                    firemark: p.firemark,
                    limit: p.limit.unwrap_or(50),
                }))
            }
            JJRM_CMD_NAME_VALIDATE => {
                let _p = deser!(jjrm_ValidateParams);
                zjjrm_validate_result(jjrvl_run_validate(jjrvl_ValidateArgs {
                    file: gallops_pathbuf(),
                    size_limit: vvc::VVCG_SIZE_LIMIT,
                }))
            }
            JJRM_CMD_NAME_LIST => {
                let p = deser!(jjrm_ListParams);
                jjrm_result(jjrmu_run_muster(jjrmu_MusterArgs {
                    file: gallops_pathbuf(),
                    status: p.status,
                }).await)
            }
            JJRM_CMD_NAME_ORIENT => {
                // Target selection is gazette-only — a param target is rejected, not a fallback.
                if let Some(bad) = zjjrm_rejected_target_param(&v) {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "{}: target selection moved to gazette_in.md — write a single `{}` notice (lede = firemark or coronet) and retry; the '{}' param is rejected, not a fallback.",
                        cmd, JJRZ_SLUG_HALTER, bad,
                    ))]));
                }
                // Require the gazette halter notice; orient saddles exactly one target.
                let targets = match gazette_in_content {
                    Some(ref content) => match jjrz_parse_halter_input(content) {
                        Ok(t) => t,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    },
                    None => return Ok(CallToolResult::error(vec![Content::text(
                        format!("{}: requires gazette_in.md with a {} notice (lede = firemark or coronet); checked {}", cmd, JJRZ_SLUG_HALTER, gazette_in_path.display()),
                    )])),
                };
                if targets.len() != 1 {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "{}: orient saddles one target — expected exactly one {} notice, got {}", cmd, JJRZ_SLUG_HALTER, targets.len(),
                    ))]));
                }
                let firemark = targets.into_iter().next().unwrap();
                let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                let (code, mut output) = jjrsd_run_saddle(jjrsd_SaddleArgs {
                    file: gallops_pathbuf(),
                    firemark: firemark.clone(),
                }, &mut gazette).await;
                let md = gazette.jjrz_emit();
                if !md.is_empty() { std::fs::write(&gazette_out_path, md.as_bytes()).ok(); }
                if code == 0 {
                    // Saddle the identity this mount actually landed on, derived
                    // from saddle's own resolution rather than the halter lede:
                    // the resolved next-actionable coronet — which saddle emitted
                    // as the Pace notice whether the lede was a firemark (next
                    // rough pace) or a coronet (looked up directly) — else the
                    // bare firemark lede when the heat has no actionable pace.
                    let mounted = gazette
                        .jjrz_query_by_slug(jjrz_Slug::Pace)
                        .into_iter()
                        .next()
                        .map(|(lede, _)| lede)
                        .unwrap_or(firemark);
                    zjjrm_refresh_emblem(
                        &zjjrm_exchange_dir(officium_id),
                        Some(zjjrm_resolve_saddle_marker(&mounted)),
                    );
                    output.push('\n');
                    output.push_str(&zjjrm_gazette_paths_block(&gazette_in_path, &gazette_out_path));
                    output.push('\n');
                }
                jjrm_result((code, output))
            }
            JJRM_CMD_NAME_SHOW => {
                // Target selection is gazette-only — a param target is rejected, not a fallback.
                if let Some(bad) = zjjrm_rejected_target_param(&v) {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "{}: target selection moved to gazette_in.md — write one `{}` notice per target (lede = firemark or coronet) and retry; the '{}' param is rejected, not a fallback.",
                        cmd, JJRZ_SLUG_HALTER, bad,
                    ))]));
                }
                let p = deser!(jjrm_ShowParams);
                // Require the gazette halter notice(s); show accepts the heterogeneous set.
                let targets = match gazette_in_content {
                    Some(ref content) => match jjrz_parse_halter_input(content) {
                        Ok(t) => t,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    },
                    None => return Ok(CallToolResult::error(vec![Content::text(
                        format!("{}: requires gazette_in.md with {} notice(s) (one lede = firemark or coronet per target); checked {}", cmd, JJRZ_SLUG_HALTER, gazette_in_path.display()),
                    )])),
                };
                // Groom saddles the heat firemark (paddock "Emblem and window
                // reference": coronet when mounted on a pace, else the heat
                // firemark). Derive it from the first target before `targets`
                // moves into the parade; a coronet target folds to its parent
                // heat, an unparseable lede leaves the standing marker untouched.
                let groom_firemark = targets.first().and_then(|lede| zjjrm_lede_firemark(lede));
                let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                let (code, mut output) = jjrpd_run_parade(jjrpd_ParadeArgs {
                    file: gallops_pathbuf(),
                    targets,
                    remaining: p.remaining,
                    hark: p.hark.clone(),
                }, &mut gazette);
                // The gazette is now show's load-bearing payload (the round-trip
                // surface), not an optional convenience. Crash-fast on a failed
                // write rather than inherit the historical .ok() silence — a
                // missing officium dir must surface, not no-op.
                if code == 0 {
                    let md = gazette.jjrz_emit();
                    if let Err(e) = std::fs::write(&gazette_out_path, md.as_bytes()) {
                        return jjrm_result((1, format!(
                            "{}: error: failed writing gazette_out {}: {}",
                            cmd, gazette_out_path.display(), e,
                        )));
                    }
                    // A hark (JJS0 `jjda_hark`) sets no standing emblem — a
                    // retrospective read mutates nothing live, officium scratch included.
                    if p.hark.is_none() {
                        if let Some(ref fm) = groom_firemark {
                            // Coronet-sticks (JJS0 `jjdxw_emblem`): a groom never
                            // demotes a coronet a prior mount saddled this
                            // officium. When a coronet already stands, saddle nothing
                            // (`None` — the emblem still repaints from the held
                            // coronet); otherwise fill the empty-or-firemark slot with
                            // the heat firemark as before.
                            let exchange = zjjrm_exchange_dir(officium_id);
                            let groom_marker = if zjjrm_standing_is_coronet(&exchange) {
                                None
                            } else {
                                Some(zjjrm_resolve_saddle_marker(fm))
                            };
                            zjjrm_refresh_emblem(&exchange, groom_marker);
                        }
                    }
                    output.push('\n');
                    output.push_str(&zjjrm_gazette_paths_block(&gazette_in_path, &gazette_out_path));
                    output.push('\n');
                }
                jjrm_result((code, output))
            }
            JJRM_CMD_NAME_ARCHIVE => {
                let p = deser!(jjrm_ArchiveParams);
                jjrm_result(jjrrt_run_retire(jjrrt_RetireArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    size_limit: p.size_limit,
                }))
            }
            JJRM_CMD_NAME_CREATE => {
                let p = deser!(jjrm_CreateParams);
                jjrm_result(jjrx_run_nominate(jjrx_NominateArgs {
                    file: gallops_pathbuf(),
                    silks: p.silks,
                }))
            }
            JJRM_CMD_NAME_ENROLL => {
                let p = deser!(jjrm_EnrollParams);
                let (silks, docket) = match gazette_in_content {
                    Some(ref content) => match jjrz_parse_slate_input(content) {
                        Ok(pair) => pair,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    },
                    None => return Ok(CallToolResult::error(vec![Content::text(
                        format!("{}: requires gazette_in.md with jjezs_slate notice; checked {}", cmd, gazette_in_path.display()),
                    )])),
                };
                jjrm_result(jjrsl_run_slate(jjrsl_SlateArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    silks,
                    before: p.before,
                    after: p.after,
                    first: p.first,
                    size_limit: p.size_limit,
                }, docket))
            }
            JJRM_CMD_NAME_REORDER => {
                let p = deser!(jjrm_ReorderParams);
                jjrm_result(jjrrl_run_rail(jjrrl_RailArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    order: vec![],
                    r#move: p.r#move,
                    before: p.before,
                    after: p.after,
                    first: p.first,
                    last: p.last,
                }))
            }
            JJRM_CMD_NAME_REDOCKET => {
                let p = deser!(jjrm_ReviseDocketParams);
                let batch = match gazette_in_content {
                    Some(ref content) => match jjrz_parse_batch_input(content) {
                        Ok(b) => b,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    },
                    None => return Ok(CallToolResult::error(vec![Content::text(
                        format!("{}: requires gazette_in.md with jjezs_reslate/jjezs_slate/jjezs_paddock notice(s); checked {}", cmd, gazette_in_path.display()),
                    )])),
                };
                // Resolve + guard the single heat firemark across all notices.
                let firemark = match jjrm_resolve_batch_firemark(&batch) {
                    Ok(fm) => fm,
                    Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                        format!("{}: {}", cmd, e),
                    )])),
                };
                let size_limit = p.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
                let message = jjrn_format_heat_discussion(&firemark, &zjjrm_batch_description(&batch));
                let firemark_for_handler = firemark.clone();
                let before = p.before.clone();
                let after = p.after.clone();
                let first = p.first;
                zjjrm_dispatch_inner_msg(cmd, &firemark, size_limit, message, move |gallops| {
                    jjrm_apply_batch(gallops, &batch, &firemark_for_handler, before, after, first)
                })
            }
            JJRM_CMD_NAME_RELABEL => {
                let p = deser!(jjrm_RelabelParams);
                jjrm_result(jjrtl_run_relabel(jjrtl_RelabelArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                    silks: p.silks,
                }))
            }
            JJRM_CMD_NAME_DROP => {
                let p = deser!(jjrm_DropParams);
                jjrm_result(jjrtl_run_drop(jjrtl_DropArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }))
            }
            JJRM_CMD_NAME_RELOCATE => {
                let p = deser!(jjrm_RelocateParams);
                jjrm_result(jjrdr_run_draft(jjrdr_DraftArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                    to: p.to,
                    before: p.before,
                    after: p.after,
                    first: p.first,
                }))
            }
            JJRM_CMD_NAME_ALTER => {
                let p = deser!(jjrm_AlterParams);
                jjrm_result(jjrfu_run_furlough(jjrfu_FurloughArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    racing: p.racing,
                    stabled: p.stabled,
                    silks: p.silks,
                    size_limit: p.size_limit,
                }))
            }
            JJRM_CMD_NAME_CLOSE => {
                let p = deser!(jjrm_CloseParams);
                jjrm_result(zjjrx_run_wrap(jjrx_WrapArgs {
                    coronet: p.coronet,
                    size_limit: p.size_limit,
                }, p.summary, p.spook))
            }
            JJRM_CMD_NAME_SEARCH => {
                let p = deser!(jjrm_SearchParams);
                jjrm_result(jjrsc_run_scout(jjrsc_ScoutArgs {
                    file: gallops_pathbuf(),
                    pattern: p.pattern,
                    actionable: p.actionable,
                }))
            }
            JJRM_CMD_NAME_BRIEF => {
                let p = deser!(jjrm_GetBriefParams);
                jjrm_result(jjrgs_run_get_spec(jjrgs_GetSpecArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }))
            }
            JJRM_CMD_NAME_CORONETS => {
                let p = deser!(jjrm_GetCoronetsParams);
                jjrm_result(jjrgc_run_get_coronets(jjrgc_GetCoronetsArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    remaining: p.remaining,
                    rough: p.rough,
                }))
            }
            JJRM_CMD_NAME_PADDOCK => {
                let p = deser!(jjrm_PaddockParams);
                if let Some(ref content) = gazette_in_content {
                    // Setter mode: gazette_in.md had paddock content. The paddock
                    // revision now joins the shared dispatch/persist lifecycle
                    // (jjri_persist co-commits gallops + paddock under the firemark),
                    // rather than self-committing on the old curry path.
                    let (firemark_str, paddock_content) = match jjrz_parse_paddock_input(content) {
                        Ok(pair) => pair,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    };
                    let firemark = match crate::jjrf_favor::jjrf_Firemark::jjrf_parse(&firemark_str) {
                        Ok(fm) => fm,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: error: {}", cmd, e),
                        )])),
                    };
                    let description = match &p.note {
                        Some(n) => format!("paddock curried: {}", n),
                        None => "paddock curried".to_string(),
                    };
                    let message = jjrn_format_heat_discussion(&firemark, &description);
                    let size_limit = p.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
                    let firemark_for_handler = firemark.clone();
                    zjjrm_dispatch_inner_msg(cmd, &firemark, size_limit, message, move |gallops| {
                        jjrg_curry_apply(gallops, &firemark_for_handler, &paddock_content)?;
                        Ok(format!("{}: paddock updated", cmd))
                    })
                } else {
                    // Getter mode: no gazette_in.md — read paddock to gazette_out.md
                    let firemark = match p.firemark {
                        Some(f) => f,
                        None => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: getter mode requires 'firemark' param", cmd),
                        )])),
                    };
                    let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                    let (code, mut output) = jjrcu_run_curry(jjrcu_CurryArgs {
                        file: gallops_pathbuf(),
                        firemark,
                        note: p.note,
                        size_limit: p.size_limit,
                    }, &mut gazette);
                    let md = gazette.jjrz_emit();
                    if !md.is_empty() { std::fs::write(&gazette_out_path, md.as_bytes()).ok(); }
                    if code == 0 {
                        // A setter-intent miss (gazette_in landed in the wrong dir)
                        // surfaces here as silent getter mode, never as an error —
                        // name both paths so the miss is self-diagnosing.
                        output.push_str(&format!(
                            "paddock returned via gazette_out: {}\n(getter mode — no gazette_in.md at {}; write content there to set)\n",
                            gazette_out_path.display(),
                            gazette_in_path.display(),
                        ));
                    }
                    jjrm_result((code, output))
                }
            }
            JJRM_CMD_NAME_TRANSFER => {
                let p = deser!(jjrm_TransferParams);
                jjrm_result(jjrrs_run(jjrrs_RestringArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    to: p.to,
                    size_limit: p.size_limit,
                }, p.coronets))
            }
            JJRM_CMD_NAME_LANDING => {
                let p = deser!(jjrm_LandingParams);
                jjrm_result(jjrld_run_landing(jjrld_LandingArgs {
                    coronet: p.coronet,
                    agent: p.agent,
                }, p.content.unwrap_or_default()))
            }
            JJRM_CMD_NAME_BIND => {
                let p = deser!(jjrm_BindParams);
                jjrm_result(crate::jjrlg_legatio::jjrlg_run_bind(
                    crate::jjrlg_legatio::jjrlg_BindArgs {
                        alias: p.alias,
                        reldir: p.reldir,
                    },
                    officium_id,
                ))
            }
            JJRM_CMD_NAME_SEND => {
                let p = deser!(jjrm_SendParams);
                jjrm_result(crate::jjrlg_legatio::jjrlg_run_send(
                    crate::jjrlg_legatio::jjrlg_SendArgs {
                        legatio: p.legatio,
                        command: p.command,
                    },
                    officium_id,
                ))
            }
            JJRM_CMD_NAME_PLANT => {
                let p = deser!(jjrm_PlantParams);
                jjrm_result(crate::jjrlg_legatio::jjrlg_run_plant(
                    crate::jjrlg_legatio::jjrlg_PlantArgs {
                        legatio: p.legatio,
                        commit: p.commit,
                    },
                    officium_id,
                ))
            }
            JJRM_CMD_NAME_FETCH => {
                let p = deser!(jjrm_FetchParams);
                jjrm_result(crate::jjrlg_legatio::jjrlg_run_fetch(
                    crate::jjrlg_legatio::jjrlg_FetchArgs {
                        legatio: p.legatio,
                        path: p.path,
                    },
                    officium_id,
                ))
            }
            JJRM_CMD_NAME_RELAY => {
                let p = deser!(jjrm_RelayParams);
                jjrm_result(crate::jjrlg_legatio::jjrlg_run_relay(
                    crate::jjrlg_legatio::jjrlg_RelayArgs {
                        legatio: p.legatio,
                        tabtarget: p.tabtarget,
                        timeout: p.timeout,
                        firemark: p.firemark,
                    },
                    officium_id,
                ))
            }
            JJRM_CMD_NAME_CHECK => {
                let p = deser!(jjrm_CheckParams);
                jjrm_result(crate::jjrlg_legatio::jjrlg_run_check(
                    crate::jjrlg_legatio::jjrlg_CheckArgs {
                        pensum: p.pensum,
                        timeout: p.timeout,
                    },
                    officium_id,
                ))
            }
            _ => {
                Ok(CallToolResult::error(vec![Content::text(format!(
                    "jjx: unknown command '{}'\nAvailable: {}",
                    cmd, JJRM_ALL_COMMANDS.join(", ")
                ))]))
            }
        }
    }

    /// vvx_tt — run a tt/*.sh tabtarget. Sibling to `jjx` on the vvx surface,
    /// not a jjx command: no officium, no model gate, no Gallops lock — it
    /// touches no heat/pace state, it just execs a curated launcher.
    #[tool(name = "vvx_tt", description = "Run a tt/*.sh tabtarget from the repo root and return its exit status, self-logged ../logs-buk/ output path, and a tail of its output. Bounded to tt/*.sh — no arbitrary commands. Absorbs the tabtarget discipline so it need not be remembered: runs from the repo root, captures output (the exit code is preserved, never eaten by a tail/head/grep pipe), and points at the self-logged record for the full text.")]
    async fn vvx_tt(&self, Parameters(p): Parameters<jjrm_VvxTtParams>) -> Result<CallToolResult, McpError> {
        let (code, report) = zjjrm_run_tabtarget(p);
        eprintln!("vvx_tt: exit={}", code);
        jjrm_result((code, report))
    }

    #[tool(name = "vvx_render", description = "Put an image on the standalone diagram viewer — the lower tool behind the `unfurl` verb. Pushes the light image over paneboard's localhost wire: a fresh look (fit-to-window) when `anew` is true, an update at the viewer's held zoom+pan when false. Best-effort / fail-soft: an absent or unreachable viewer is a soft notice, not an error — bringing the viewer up is paneboard's job. Takes no officium and touches no gallops. The optional `dark` path is accepted for a stable signature but not yet transported (today only the light image is pushed).")]
    async fn vvx_render(&self, Parameters(p): Parameters<jjrm_RenderParams>) -> Result<CallToolResult, McpError> {
        let report = zjjrm_render_report(&p);
        eprintln!("vvx_render: {}", report);
        Ok(CallToolResult::success(vec![Content::text(report)]))
    }
}

#[tool_handler]
impl ServerHandler for jjrm_McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Job Jockey Kit - MCP tools for project initiative management".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// ============================================================================
// Public entry point
// ============================================================================

/// Start MCP stdio server. Blocks until client disconnects.
///
/// Lifecycle: serve → waiting. Officium created by explicit jjx_open call.
pub async fn jjrm_serve_stdio() -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::ServiceExt;

    let server = jjrm_McpServer::jjrm_new();
    let service = server
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|e| format!("MCP serve error: {}", e))?;

    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fresh, empty scratch officia dir. Process-id-scoped so parallel test
    /// binaries don't share state; cleared at entry so a prior crash can't leak in.
    fn scratch_officia(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("jjrm_test_officia_{}_{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn random_suffix_is_lowercase_alphanumeric_of_requested_len() {
        let s = zjjrm_random_suffix(OFFICIUM_SUFFIX_LEN);
        assert_eq!(s.chars().count(), OFFICIUM_SUFFIX_LEN);
        assert!(
            s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
            "suffix {:?} contains a non-[a-z0-9] char",
            s
        );
    }

    #[test]
    fn first_of_day_id_is_ordinal_1000_with_random_suffix() {
        let officia = scratch_officia("first_of_day");
        let id = zjjrm_generate_officium_id(&officia, "260608");
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 3, "id {:?} is not three dash-segments", id);
        assert_eq!(parts[0], "260608");
        assert_eq!(parts[1], "1000");
        assert_eq!(parts[2].len(), OFFICIUM_SUFFIX_LEN);
        let _ = std::fs::remove_dir_all(&officia);
    }

    #[test]
    fn scan_reads_ordinal_from_new_format_dirs() {
        // The critical gotcha: a `NNNN-RAND` dir must not break the max-scan.
        // Parsing the whole post-date field would fail and re-mint 1000 forever.
        let officia = scratch_officia("scan_new_format");
        std::fs::create_dir(officia.join("260608-1000-h7k2")).unwrap();
        std::fs::create_dir(officia.join("260608-1003-q9z1")).unwrap();
        let id = zjjrm_generate_officium_id(&officia, "260608");
        assert!(
            id.starts_with("260608-1004-"),
            "expected ordinal 1004 after scanning new-format dirs, got {:?}",
            id
        );
        let _ = std::fs::remove_dir_all(&officia);
    }

    #[test]
    fn normalize_tabtarget_accepts_bare_and_tt_prefixed() {
        assert_eq!(
            zjjrm_normalize_tabtarget("rbw-ts.TestSuite.fast.sh").unwrap(),
            "rbw-ts.TestSuite.fast.sh"
        );
        assert_eq!(
            zjjrm_normalize_tabtarget("tt/rbw-ts.TestSuite.fast.sh").unwrap(),
            "rbw-ts.TestSuite.fast.sh"
        );
        assert_eq!(
            zjjrm_normalize_tabtarget("  tt/foo.sh  ").unwrap(),
            "foo.sh"
        );
    }

    #[test]
    fn normalize_tabtarget_rejects_arbitrary_paths_and_non_sh() {
        // Path escape, nested path, traversal, and non-.sh all refuse — the
        // no-arbitrary-command guard.
        assert!(zjjrm_normalize_tabtarget("/etc/passwd").is_err());
        assert!(zjjrm_normalize_tabtarget("tt/../secrets.sh").is_err());
        assert!(zjjrm_normalize_tabtarget("sub/dir/foo.sh").is_err());
        assert!(zjjrm_normalize_tabtarget("rbw-ts.TestSuite.fast").is_err());
        assert!(zjjrm_normalize_tabtarget("").is_err());
        assert!(zjjrm_normalize_tabtarget("tt/").is_err());
    }

    #[test]
    fn tail_returns_whole_under_limit_and_suffix_over() {
        let (s, trunc) = zjjrm_tail("short", 60_000);
        assert_eq!(s, "short");
        assert!(!trunc);

        let big = "x".repeat(100);
        let (s, trunc) = zjjrm_tail(&big, 10);
        assert_eq!(s.len(), 10);
        assert!(trunc);
    }

    #[test]
    fn render_control_matches_frozen_pbgvw_wire_format() {
        // Pins the control line vvx_render emits to paneboard's FROZEN wire
        // contract: one `pbgvw_`-sprued JSON object, '\n'-terminated, `anew`
        // selecting fresh vs update. A drift here is a wire-format break.
        // Single-variant push (dark_len 0): byte-identical to the pre-pair frame,
        // so `pbgvw_dark_len` is absent entirely.
        assert_eq!(
            zjjrm_render_control(true, 1234, 0),
            "{\"pbgvw_verb\":\"pbgvw_fresh\",\"pbgvw_id\":0,\"pbgvw_len\":1234}\n"
        );
        assert_eq!(
            zjjrm_render_control(false, 0, 0),
            "{\"pbgvw_verb\":\"pbgvw_update\",\"pbgvw_id\":0,\"pbgvw_len\":0}\n"
        );
        // Paired push (dark_len > 0): `pbgvw_dark_len` rides as the additive
        // trailing key, the dark payload following the light payload on the wire.
        assert_eq!(
            zjjrm_render_control(true, 1234, 5678),
            "{\"pbgvw_verb\":\"pbgvw_fresh\",\"pbgvw_id\":0,\"pbgvw_len\":1234,\"pbgvw_dark_len\":5678}\n"
        );
    }

    #[test]
    fn parse_iterm_uuid_extracts_uuid_and_discards_position_prefix() {
        // Real iTerm shape: <wNtNpN>:<UUID>. We take the UUID; w4t0p0 is
        // discarded (it restamps on tab-drag). The UUID is the question vvx puts
        // to iTerm, not the emblem key — the resolved window id is the key.
        assert_eq!(
            zjjrm_parse_iterm_uuid("w4t0p0:AA97D5ED-F633-4513-95B2-2A930EBB7365"),
            Some("AA97D5ED-F633-4513-95B2-2A930EBB7365".to_string())
        );
        // A different position prefix, same UUID, must yield the same UUID.
        assert_eq!(
            zjjrm_parse_iterm_uuid("w0t9p3:AA97D5ED-F633-4513-95B2-2A930EBB7365"),
            zjjrm_parse_iterm_uuid("w4t0p0:AA97D5ED-F633-4513-95B2-2A930EBB7365")
        );
    }

    #[test]
    fn parse_iterm_uuid_fails_soft_on_malformed() {
        assert_eq!(zjjrm_parse_iterm_uuid("no-colon-here"), None);
        assert_eq!(zjjrm_parse_iterm_uuid("w4t0p0:"), None);
        assert_eq!(zjjrm_parse_iterm_uuid(""), None);
    }

    #[test]
    fn plausible_uuid_guards_applescript_interpolation() {
        // Real iTerm session UUID: hex digits and hyphens — accepted.
        assert!(zjjrm_is_plausible_uuid("AA97D5ED-F633-4513-95B2-2A930EBB7365"));
        // Empty, or anything that could break out of the quoted AppleScript
        // string (quote, space, paren, backslash), is rejected.
        assert!(!zjjrm_is_plausible_uuid(""));
        assert!(!zjjrm_is_plausible_uuid("AA\" & (do shell script \"x\")"));
        assert!(!zjjrm_is_plausible_uuid("has space"));
    }

    #[test]
    fn iterm_resolve_script_embeds_uuid_and_returns_window_id() {
        let script = zjjrm_iterm_resolve_script("AA97D5ED-F633-4513-95B2-2A930EBB7365");
        // The UUID is the match target, and the script returns the window id.
        assert!(script.contains("is \"AA97D5ED-F633-4513-95B2-2A930EBB7365\""));
        assert!(script.contains("return (id of w)"));
        assert!(script.contains("tell application \"iTerm2\""));
    }

    #[test]
    fn normalize_identity_reconstructs_glyph_by_length() {
        // Bare body → full glyph-prefixed form (the marker may store either).
        assert_eq!(zjjrm_normalize_identity("Bh"), "₣Bh");
        assert_eq!(zjjrm_normalize_identity("BhAAF"), "₢BhAAF");
        // Already-glyphed input round-trips, not double-prefixed.
        assert_eq!(zjjrm_normalize_identity("₣Bh"), "₣Bh");
        assert_eq!(zjjrm_normalize_identity("₢BhAAF"), "₢BhAAF");
        // Surrounding whitespace (a trailing newline in the marker file) is trimmed.
        assert_eq!(zjjrm_normalize_identity("  ₣Bh\n"), "₣Bh");
        // A body that is neither 2 nor 5 chars passes through rather than being
        // mislabeled (the marker only ever stores real firemarks/coronets, so
        // this is purely the defensive degraded path).
        assert_eq!(zjjrm_normalize_identity("odd"), "odd");
        assert_eq!(zjjrm_normalize_identity("sixchar"), "sixchar");
    }

    #[test]
    fn region_block_omits_empty_and_rides_optional_style() {
        // No lines → no block at all (placement is explicit, absence is fine).
        let empty = zjjrm_region_block("pbge_bottom", &jjrm_RegionStyle::default(), &[]);
        assert_eq!(empty, "");

        // Absent style → bare location attr, no color/size.
        let plain = zjjrm_region_block(
            "pbge_middle",
            &jjrm_RegionStyle::default(),
            &["repo".to_string()],
        );
        assert_eq!(plain, "### pbge_region {pbge_location=pbge_middle}\nrepo\n");

        // Present style → attrs ride in order.
        let styled = zjjrm_region_block(
            "pbge_top",
            &jjrm_RegionStyle {
                color: Some("#ffffff".to_string()),
                size: Some(14),
            },
            &["₣Bh".to_string()],
        );
        assert_eq!(
            styled,
            "### pbge_region {pbge_location=pbge_top, pbge_color=#ffffff, pbge_size=14}\n₣Bh\n"
        );
    }

    #[test]
    fn compose_emblem_pace_middle_is_basename_pace_heat() {
        // Pace-mount: identity is a coronet, both silks present → middle is three
        // lines (basename, coronet-prefixed pace silks, firemark-prefixed heat
        // silks).
        let style = jjrm_EmblemStyle {
            top: jjrm_RegionStyle {
                color: Some("#ffffff".to_string()),
                size: Some(14),
            },
            middle: jjrm_RegionStyle::default(),
            bottom: jjrm_RegionStyle::default(),
        };
        let out = zjjrm_compose_emblem(
            "iterm-session/121",
            "₢BhAAP",
            "rbm_beta_recipemuster",
            Some(("₢BhAAP", "vvx-emblem-identity-derivation")),
            Some(("₣Bh", "jjk-v4-1-svg-viewer-and-pane-labels")),
            &style,
        );
        let expected = "\
# pbge_emblem
## pbge_pane iterm-session/121
### pbge_region {pbge_location=pbge_top, pbge_color=#ffffff, pbge_size=14}
₢BhAAP
### pbge_region {pbge_location=pbge_middle}
rbm_beta_recipemuster
₢BhAAP: vvx-emblem-identity-derivation
₣Bh: jjk-v4-1-svg-viewer-and-pane-labels
### pbge_region {pbge_location=pbge_bottom}
₢BhAAP
";
        assert_eq!(out, expected);
        // Identity mirrors top and bottom, for a four-corner read.
        assert!(out.contains("### pbge_region {pbge_location=pbge_bottom}\n₢BhAAP\n"));
    }

    #[test]
    fn compose_emblem_heat_middle_drops_pace_line() {
        // Heat-mount/groom: identity is a firemark, no pace → middle is two lines
        // (basename, firemark-prefixed heat silks). The pace line is absent, not
        // blank.
        let style = jjrm_EmblemStyle::default();
        let out = zjjrm_compose_emblem(
            "iterm-session/121",
            "₣Bh",
            "rbm_beta_recipemuster",
            None,
            Some(("₣Bh", "jjk-v4-1-svg-viewer-and-pane-labels")),
            &style,
        );
        let expected = "\
# pbge_emblem
## pbge_pane iterm-session/121
### pbge_region {pbge_location=pbge_top}
₣Bh
### pbge_region {pbge_location=pbge_middle}
rbm_beta_recipemuster
₣Bh: jjk-v4-1-svg-viewer-and-pane-labels
### pbge_region {pbge_location=pbge_bottom}
₣Bh
";
        assert_eq!(out, expected);
    }

    #[test]
    fn compose_emblem_no_silks_middle_is_basename_only() {
        // No mount yet (bare officium handle, no silks) → middle degrades to the
        // basename alone; never an empty band, never a stray blank line.
        let style = jjrm_EmblemStyle::default();
        let out = zjjrm_compose_emblem(
            "iterm-session/121",
            "☉260623-1006-6lq4",
            "rbm_beta_recipemuster",
            None,
            None,
            &style,
        );
        assert!(out.contains("### pbge_region {pbge_location=pbge_middle}\nrbm_beta_recipemuster\n### pbge_region"));
    }

    #[test]
    fn load_emblem_style_fails_soft_on_absent_and_garbage() {
        let dir = scratch_officia("emblem_style");
        // No config file present → all-default (every field None).
        let absent = zjjrm_load_emblem_style(&dir);
        assert!(absent.top.color.is_none() && absent.top.size.is_none());

        // Garbage at the config path → still all-default, never an error.
        let cfg = dir.join(".claude/jjm");
        std::fs::create_dir_all(&cfg).unwrap();
        std::fs::write(cfg.join("jje_emblem.json"), b"{ not json").unwrap();
        let garbage = zjjrm_load_emblem_style(&dir);
        assert!(garbage.middle.color.is_none());

        // A partial config parses: only the present field lands.
        std::fs::write(
            cfg.join("jje_emblem.json"),
            br#"{"top": {"size": 18}}"#,
        )
        .unwrap();
        let partial = zjjrm_load_emblem_style(&dir);
        assert_eq!(partial.top.size, Some(18));
        assert!(partial.top.color.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_emblem_atomic_succeeds_and_is_readable() {
        let dir = scratch_officia("emblem_write");
        let root = dir.join(".config/paneboard/emblems");
        let path = jjrm_emblem_path(&root, "iterm-session/UUID-1234");
        zjjrm_write_emblem_atomic(&path, "# pbge_emblem\n").unwrap();
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "# pbge_emblem\n"
        );
        // No temp residue left beside the target.
        let mut tmp = path.as_os_str().to_owned();
        tmp.push(".tmp");
        assert!(!std::path::Path::new(&tmp).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_emblem_atomic_fails_soft_on_refused_parent() {
        // The "refused/absent emblem directory" non-regression: when the parent
        // cannot be created (here it is an existing *file*, not a dir), the write
        // returns Err rather than panicking — and the sole caller discards it, so
        // a jjx engagement is never affected.
        let dir = scratch_officia("emblem_refused");
        let blocker = dir.join("not-a-dir");
        std::fs::write(&blocker, b"i am a file").unwrap();
        let path = blocker.join("iterm-session/UUID.emblem");
        assert!(zjjrm_write_emblem_atomic(&path, "x").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn standing_is_coronet_discriminates_marker_identity() {
        // The coronet-sticks discriminator (JJS0 jjdxw_emblem): only a standing
        // *coronet* marker blocks a groom's overwrite. A firemark, an absent
        // marker, and garbage are all "fillable" — the groom saddles normally.
        let officia = scratch_officia("standing_coronet");
        let marker_path = officia.join(SADDLE_MARKER_FILE);
        let write = |m: &jjrm_SaddleMarker| {
            std::fs::write(&marker_path, serde_json::to_string(m).unwrap()).unwrap();
        };

        // Absent marker (no mount yet) → fillable.
        assert!(!zjjrm_standing_is_coronet(&officia));

        // Firemark identity (heat-mount or groom) → fillable, a groom may replace it.
        write(&jjrm_SaddleMarker {
            identity: "₣Bh".to_string(),
            pace_silks: None,
            heat_silks: "heat".to_string(),
        });
        assert!(!zjjrm_standing_is_coronet(&officia));

        // Coronet identity (pace-mount) → sticks, a groom must not demote it.
        write(&jjrm_SaddleMarker {
            identity: "₢BhAAT".to_string(),
            pace_silks: Some("pace".to_string()),
            heat_silks: "heat".to_string(),
        });
        assert!(zjjrm_standing_is_coronet(&officia));

        // Unparseable marker fails soft → fillable.
        std::fs::write(&marker_path, b"not json").unwrap();
        assert!(!zjjrm_standing_is_coronet(&officia));

        let _ = std::fs::remove_dir_all(&officia);
    }

    #[test]
    fn scan_handles_mixed_legacy_and_foreign_date_dirs() {
        let officia = scratch_officia("scan_mixed");
        std::fs::create_dir(officia.join("260608-1000")).unwrap(); // legacy, no suffix
        std::fs::create_dir(officia.join("260608-1005-abcd")).unwrap(); // new format
        std::fs::create_dir(officia.join("260607-1099-zzzz")).unwrap(); // other day, ignored
        let id = zjjrm_generate_officium_id(&officia, "260608");
        assert!(
            id.starts_with("260608-1006-"),
            "expected ordinal 1006 from mixed dirs, got {:?}",
            id
        );
        let _ = std::fs::remove_dir_all(&officia);
    }
}
