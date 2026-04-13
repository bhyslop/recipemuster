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
use crate::jjrgl_garland::{jjrgl_GarlandArgs, jjrgl_run_garland};
use crate::jjrrs_restring::{jjrrs_RestringArgs, jjrrs_run};
use crate::jjrld_landing::{jjrld_LandingArgs, jjrld_run_landing};
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug, jjrz_parse_slate_input, jjrz_parse_reslate_input, jjrz_parse_paddock_input};

const GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";
const OFFICIA_DIR: &str = ".claude/jjm/officia";
const HEARTBEAT_FILE: &str = "heartbeat";
const GAZETTE_IN_FILE: &str = "gazette_in.md";
const GAZETTE_OUT_FILE: &str = "gazette_out.md";
const PROBE_DATE_FILE: &str = ".probe_date";
const EXSANGUINATION_THRESHOLD_SECS: u64 = 7 * 24 * 3600;
const OFFICIUM_SUN_PREFIX: char = '\u{2609}'; // ☉

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
const JJRM_CMD_NAME_CONTINUE: &str = "jjx_continue";
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
    JJRM_CMD_NAME_CORONETS, JJRM_CMD_NAME_PADDOCK, JJRM_CMD_NAME_CONTINUE,
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
/// persistence, or commit messages.
fn zjjrm_dispatch_inner(
    cmd: &str,
    firemark: &crate::jjrf_favor::jjrf_Firemark,
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
        format!("jjx: {}", cmd),
        50000,
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


/// Dispatch a pace-affiliated command. Coronet supplied from params; parent firemark derived.
fn jjrm_dispatch_pace(
    cmd: &str,
    coronet_str: &str,
    handler: impl FnOnce(&mut crate::jjrg_gallops::jjrg_Gallops) -> Result<String, String>,
) -> Result<CallToolResult, McpError> {
    let coronet = match crate::jjrf_favor::jjrf_Coronet::jjrf_parse(coronet_str) {
        Ok(c) => c,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: error: {}", cmd, e),
            )]));
        }
    };
    let firemark = coronet.jjrf_parent_firemark();
    zjjrm_dispatch_inner(cmd, &firemark, handler)
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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_OrientParams {
    pub firemark: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ShowParams {
    pub target: Option<String>,
    #[serde(default)]
    pub detail: bool,
    #[serde(default)]
    pub remaining: bool,
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
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CloseParams {
    pub coronet: String,
    pub summary: Option<String>,
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
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ContinueParams {
    pub firemark: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_TransferParams {
    pub firemark: String,
    pub to: String,
    pub coronets: String,
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

/// Generate officium ID: YYMMDD-NNNN (autonumber from directory listing).
fn zjjrm_generate_officium_id(officia: &Path, today: &str) -> String {
    let prefix = format!("{}-", today);
    let mut max_num: u32 = 999;
    if let Ok(entries) = std::fs::read_dir(officia) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(suffix) = name.strip_prefix(&prefix) {
                if let Ok(num) = suffix.parse::<u32>() {
                    if num > max_num { max_num = num; }
                }
            }
        }
    }
    format!("{}{:04}", prefix, max_num + 1)
}

/// Check if daily probe is needed (probe_date file doesn't match today).
fn zjjrm_needs_probe(probe_date_path: &Path, today: &str) -> bool {
    match std::fs::read_to_string(probe_date_path) {
        Ok(content) => content.trim() != today,
        Err(_) => true,
    }
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

/// Resolve officium ID to gazette input file path (agent → server).
fn zjjrm_gazette_in_path(officium: &str) -> std::path::PathBuf {
    let bare_id = officium.trim_start_matches(OFFICIUM_SUN_PREFIX);
    PathBuf::from(OFFICIA_DIR).join(bare_id).join(GAZETTE_IN_FILE)
}

/// Resolve officium ID to gazette output file path (server → agent).
fn zjjrm_gazette_out_path(officium: &str) -> std::path::PathBuf {
    let bare_id = officium.trim_start_matches(OFFICIUM_SUN_PREFIX);
    PathBuf::from(OFFICIA_DIR).join(bare_id).join(GAZETTE_OUT_FILE)
}

/// Handle jjx_open: create a new officium.
async fn zjjrm_handle_open() -> Result<CallToolResult, McpError> {
    let cn = JJRM_CMD_NAME_OPEN;
    let mut output = vvc::vvco_Output::buffer();

    // Disk space guard — block before any state changes
    match crate::jjrdk_diskcheck::jjrdk_check_disk_space() {
        Ok(survey) => vvco_out!(output, "{}", survey),
        Err(msg) => return Ok(CallToolResult::error(vec![Content::text(msg)])),
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

    // Daily probe: run vvcp_probe if .probe_date doesn't match today
    let probe_date_path = officia.join(PROBE_DATE_FILE);
    let probe_data = if zjjrm_needs_probe(&probe_date_path, &today) {
        match vvc::vvcp_probe().await {
            Ok(data) => {
                std::fs::write(&probe_date_path, &today).ok();
                Some(data)
            }
            Err(e) => {
                eprintln!("{}: probe warning: {}", cn, e);
                None
            }
        }
    } else {
        None
    };

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
        probe_data.as_deref(),
    );

    let commit_args = vvc::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(lock) => {
            let mut commit_output = vvc::vvco_Output::buffer();
            if let Err(e) = lock.vvcc_commit(&commit_args, &mut commit_output) {
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("{}: invitatory commit error: {}", cn, e),
                )]));
            }
        }
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("{}: lock error: {}", cn, e),
            )]));
        }
    };

    if reaped > 0 || active > 0 {
        vvco_out!(output, "Exsanguination: {} active, {} reaped", active, reaped);
    }
    vvco_out!(output, "{}{}", OFFICIUM_SUN_PREFIX, id);
    Ok(CallToolResult::success(vec![Content::text(output.vvco_finish())]))
}


// ============================================================================
// Model gate
// ============================================================================

/// Extract model tier from verbatim model ID string.
/// Returns "opus", "sonnet", "haiku", or "unknown".
fn zjjrm_extract_tier(model: &str) -> &'static str {
    let lower = model.to_ascii_lowercase();
    if lower.contains("opus") {
        "opus"
    } else if lower.contains("sonnet") {
        "sonnet"
    } else if lower.contains("haiku") {
        "haiku"
    } else {
        "unknown"
    }
}

/// Gate check: require opus-tier model. Returns Err with diagnostic on failure.
fn zjjrm_check_model_gate(model: &str) -> Result<(), String> {
    let tier = zjjrm_extract_tier(model);
    if tier == "opus" {
        return Ok(());
    }
    Err(format!(
        "MODEL GATE — this command requires opus.\n\n  Received model: {}\n  Extracted tier: {}\n\nJob Jockey commands currently require an opus-tier model.",
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

        // jjx_open creates the officium — handle before officium validation
        if cmd == JJRM_CMD_NAME_OPEN {
            return zjjrm_handle_open().await;
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
                jjrm_result(jjrvl_run_validate(jjrvl_ValidateArgs {
                    file: gallops_pathbuf(),
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
                let p = deser!(jjrm_OrientParams);
                let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                let result = jjrsd_run_saddle(jjrsd_SaddleArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                }, &mut gazette).await;
                let md = gazette.jjrz_emit();
                if !md.is_empty() { std::fs::write(&gazette_out_path, md.as_bytes()).ok(); }
                jjrm_result(result)
            }
            JJRM_CMD_NAME_SHOW => {
                let p = deser!(jjrm_ShowParams);
                let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                let result = jjrpd_run_parade(jjrpd_ParadeArgs {
                    file: gallops_pathbuf(),
                    target: p.target,
                    detail: p.detail,
                    remaining: p.remaining,
                }, &mut gazette);
                let md = gazette.jjrz_emit();
                if !md.is_empty() { std::fs::write(&gazette_out_path, md.as_bytes()).ok(); }
                jjrm_result(result)
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
                        format!("{}: requires gazette_in.md with jjezs_slate notice", cmd),
                    )])),
                };
                jjrm_result(jjrsl_run_slate(jjrsl_SlateArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    silks,
                    before: p.before,
                    after: p.after,
                    first: p.first,
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
                let _p = deser!(jjrm_ReviseDocketParams);
                let pairs = match gazette_in_content {
                    Some(ref content) => match jjrz_parse_reslate_input(content) {
                        Ok(p) => p,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    },
                    None => return Ok(CallToolResult::error(vec![Content::text(
                        format!("{}: requires gazette_in.md with jjezs_reslate notice(s)", cmd),
                    )])),
                };
                let first_coronet = pairs[0].0.clone();
                jjrm_dispatch_pace(cmd, &first_coronet, |gallops| {
                    let mut diffs = Vec::new();
                    for (coronet, docket) in &pairs {
                        let diff = jjrtl_run_revise_docket(gallops, coronet, docket)?;
                        if !diff.is_empty() {
                            diffs.push(format!("--- ₢{} reslate diff ---\n{}", coronet, diff));
                        }
                    }
                    let mut output = format!("Revised {} pace(s)", pairs.len());
                    if !diffs.is_empty() {
                        output.push_str("\n\n");
                        output.push_str(&diffs.join("\n"));
                    }
                    Ok(output)
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
                }))
            }
            JJRM_CMD_NAME_CLOSE => {
                let p = deser!(jjrm_CloseParams);
                jjrm_result(zjjrx_run_wrap(jjrx_WrapArgs {
                    coronet: p.coronet,
                    size_limit: p.size_limit,
                }, p.summary))
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
                    // Setter mode: gazette_in.md had paddock content
                    let (firemark, paddock_content) = match jjrz_parse_paddock_input(content) {
                        Ok(pair) => pair,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: gazette input error: {}", cmd, e),
                        )])),
                    };
                    let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                    let result = jjrcu_run_curry(jjrcu_CurryArgs {
                        file: gallops_pathbuf(),
                        firemark,
                        note: p.note,
                    }, Some(paddock_content), &mut gazette);
                    let md = gazette.jjrz_emit();
                    if !md.is_empty() { std::fs::write(&gazette_out_path, md.as_bytes()).ok(); }
                    jjrm_result(result)
                } else {
                    // Getter mode: no gazette_in.md — read paddock to gazette_out.md
                    let firemark = match p.firemark {
                        Some(f) => f,
                        None => return Ok(CallToolResult::error(vec![Content::text(
                            format!("{}: getter mode requires 'firemark' param", cmd),
                        )])),
                    };
                    let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
                    let result = jjrcu_run_curry(jjrcu_CurryArgs {
                        file: gallops_pathbuf(),
                        firemark,
                        note: p.note,
                    }, None, &mut gazette);
                    let md = gazette.jjrz_emit();
                    if !md.is_empty() { std::fs::write(&gazette_out_path, md.as_bytes()).ok(); }
                    jjrm_result(result)
                }
            }
            JJRM_CMD_NAME_CONTINUE => {
                let p = deser!(jjrm_ContinueParams);
                jjrm_result(jjrgl_run_garland(jjrgl_GarlandArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                }))
            }
            JJRM_CMD_NAME_TRANSFER => {
                let p = deser!(jjrm_TransferParams);
                jjrm_result(jjrrs_run(jjrrs_RestringArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    to: p.to,
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
