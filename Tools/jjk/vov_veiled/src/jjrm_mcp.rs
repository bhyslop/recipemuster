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
use crate::jjrz_gazette::{jjrz_parse_slate_input, jjrz_parse_reslate_input, jjrz_parse_paddock_input};

const GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";
const OFFICIA_DIR: &str = ".claude/jjm/officia";
const HEARTBEAT_FILE: &str = "heartbeat";
const GAZETTE_FILE: &str = "gazette.md";
const PROBE_DATE_FILE: &str = ".probe_date";
const EXSANGUINATION_THRESHOLD_SECS: u64 = 4 * 3600;
const OFFICIUM_SUN_PREFIX: char = '\u{2609}'; // ☉

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

/// Dispatch a heat-affiliated command. Firemark supplied directly from params.
fn jjrm_dispatch_heat(
    cmd: &str,
    firemark_str: &str,
    handler: impl FnOnce(&mut crate::jjrg_gallops::jjrg_Gallops) -> Result<String, String>,
) -> Result<CallToolResult, McpError> {
    let firemark = match crate::jjrf_favor::jjrf_Firemark::jjrf_parse(firemark_str) {
        Ok(f) => f,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx {}: error: {}", cmd, e),
            )]));
        }
    };
    zjjrm_dispatch_inner(cmd, &firemark, handler)
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
    pub firemark: Option<String>,
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
    #[serde(default)]
    pub silks: Option<String>,
    #[serde(default)]
    pub docket: Option<String>,
    #[schemars(description = "Gazette markdown input (alternative to silks+docket). Format: # slate <silks>\\n\\n<docket text>")]
    pub input: Option<String>,
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
    #[serde(default)]
    pub coronet: Option<String>,
    #[serde(default)]
    pub docket: Option<String>,
    #[schemars(description = "Gazette markdown input (alternative to coronet+docket). Supports mass reslate with multiple notices. Format: # reslate <coronet>\\n\\n<docket text>")]
    pub input: Option<String>,
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
    pub content: Option<String>,
    pub note: Option<String>,
    #[schemars(description = "Gazette markdown input for setter mode (alternative to firemark+content). Format: # paddock <firemark>\\n\\n<paddock content>")]
    pub input: Option<String>,
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

// ============================================================================
// Single dispatcher tool params
// ============================================================================

fn jjrm_empty_object() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_JjxParams {
    #[schemars(description = "Command name: jjx_list, jjx_show, jjx_orient, jjx_record, jjx_log, jjx_validate, jjx_create, jjx_enroll, jjx_close, jjx_archive, jjx_reorder, jjx_redocket, jjx_relabel, jjx_drop, jjx_relocate, jjx_alter, jjx_search, jjx_brief, jjx_coronets, jjx_paddock, jjx_continue, jjx_transfer, jjx_landing, jjx_open")]
    pub command: String,
    #[schemars(description = "Command parameters as JSON object. See CLAUDE.md for per-command schemas.")]
    #[serde(default = "jjrm_empty_object")]
    pub params: serde_json::Value,
    #[schemars(description = "Officium identity (from jjx_open). Required for all commands except jjx_open.")]
    #[serde(default)]
    pub officium: Option<String>,
}

// ============================================================================
// Officium lifecycle — jjdxo_* (Officium Lifecycle in JJS0)
// ============================================================================

/// Reap stale officium directories by heartbeat mtime.
fn zjjrm_exsanguinate(officia: &Path) {
    let entries = match std::fs::read_dir(officia) {
        Ok(e) => e,
        Err(_) => return,
    };
    let now = std::time::SystemTime::now();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
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
            }
        }
    }
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

/// Handle jjx_open: create a new officium.
async fn zjjrm_handle_open() -> Result<CallToolResult, McpError> {
    let officia = PathBuf::from(OFFICIA_DIR);
    if let Err(e) = std::fs::create_dir_all(&officia) {
        return Ok(CallToolResult::error(vec![Content::text(
            format!("jjx_open: error creating officia dir: {}", e),
        )]));
    }

    // Exsanguinate stale officia before creating new one
    zjjrm_exsanguinate(&officia);

    // Generate officium ID
    let today = chrono::Local::now().format("%y%m%d").to_string();
    let id = zjjrm_generate_officium_id(&officia, &today);

    // Create exchange directory with gazette and heartbeat
    let exchange = officia.join(&id);
    if let Err(e) = std::fs::create_dir_all(&exchange) {
        return Ok(CallToolResult::error(vec![Content::text(
            format!("jjx_open: error creating exchange dir: {}", e),
        )]));
    }
    std::fs::write(exchange.join(GAZETTE_FILE), b"").ok();
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
                eprintln!("jjx_open: probe warning: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Invitatory commit: jjb:HALLMARK::i: OFFICIUM <id>
    let hallmark = vvc::vvcc_get_hallmark();
    let subject = format!("OFFICIUM {}", id);
    let action = crate::jjrnm_markers::JJRNM_INVITATORY.to_string();
    let message = vvc::vvcc_format_branded(
        crate::jjrn_notch::JJRN_COMMIT_PREFIX,
        &hallmark,
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
            let mut output = vvc::vvco_Output::buffer();
            if let Err(e) = lock.vvcc_commit(&commit_args, &mut output) {
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("jjx_open: invitatory commit error: {}", e),
                )]));
            }
        }
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(
                format!("jjx_open: lock error: {}", e),
            )]));
        }
    };

    Ok(CallToolResult::success(vec![Content::text(
        format!("{}{}", OFFICIUM_SUN_PREFIX, id),
    )]))
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

        // jjx_open creates the officium — handle before officium validation
        if cmd == "jjx_open" {
            return zjjrm_handle_open().await;
        }

        // Officium envelope: validate directory exists and touch heartbeat
        if let Some(ref officium) = p.officium {
            if let Err(e) = zjjrm_validate_officium(officium) {
                return Ok(CallToolResult::error(vec![Content::text(
                    format!("jjx {}: {}", cmd, e),
                )]));
            }
        }
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
            "jjx_record" => {
                let p = deser!(jjrm_RecordParams);
                jjrm_result(jjrnc_run_notch(jjrnc_NotchArgs {
                    identity: p.identity,
                    files: p.files,
                    size_limit: p.size_limit,
                    intent: p.intent,
                }))
            }
            "jjx_log" => {
                let p = deser!(jjrm_LogParams);
                jjrm_result(jjrrn_run_rein(jjrrn_ReinArgs {
                    firemark: p.firemark,
                    limit: p.limit.unwrap_or(50),
                }))
            }
            "jjx_validate" => {
                let _p = deser!(jjrm_ValidateParams);
                jjrm_result(jjrvl_run_validate(jjrvl_ValidateArgs {
                    file: gallops_pathbuf(),
                }))
            }
            "jjx_list" => {
                let p = deser!(jjrm_ListParams);
                jjrm_result(jjrmu_run_muster(jjrmu_MusterArgs {
                    file: gallops_pathbuf(),
                    status: p.status,
                }).await)
            }
            "jjx_orient" => {
                let p = deser!(jjrm_OrientParams);
                jjrm_result(jjrsd_run_saddle(jjrsd_SaddleArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                }).await)
            }
            "jjx_show" => {
                let p = deser!(jjrm_ShowParams);
                jjrm_result(jjrpd_run_parade(jjrpd_ParadeArgs {
                    file: gallops_pathbuf(),
                    target: p.target,
                    detail: p.detail,
                    remaining: p.remaining,
                }))
            }
            "jjx_archive" => {
                let p = deser!(jjrm_ArchiveParams);
                jjrm_result(jjrrt_run_retire(jjrrt_RetireArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    size_limit: p.size_limit,
                }))
            }
            "jjx_create" => {
                let p = deser!(jjrm_CreateParams);
                jjrm_result(jjrx_run_nominate(jjrx_NominateArgs {
                    file: gallops_pathbuf(),
                    silks: p.silks,
                }))
            }
            "jjx_enroll" => {
                let p = deser!(jjrm_EnrollParams);
                let (silks, docket) = if let Some(ref input) = p.input {
                    match jjrz_parse_slate_input(input) {
                        Ok(pair) => pair,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("jjx_enroll: gazette input error: {}", e),
                        )])),
                    }
                } else {
                    match (p.silks, p.docket) {
                        (Some(s), Some(d)) => (s, d),
                        _ => return Ok(CallToolResult::error(vec![Content::text(
                            "jjx_enroll: requires either 'input' (gazette) or both 'silks' and 'docket'".to_string(),
                        )])),
                    }
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
            "jjx_reorder" => {
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
            "jjx_redocket" => {
                let p = deser!(jjrm_ReviseDocketParams);
                if let Some(ref input) = p.input {
                    let pairs = match jjrz_parse_reslate_input(input) {
                        Ok(p) => p,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("jjx_redocket: gazette input error: {}", e),
                        )])),
                    };
                    let first_coronet = pairs[0].0.clone();
                    jjrm_dispatch_pace(cmd, &first_coronet, |gallops| {
                        for (coronet, docket) in &pairs {
                            jjrtl_run_revise_docket(gallops, coronet, docket)?;
                        }
                        Ok(format!("Revised {} pace(s)", pairs.len()))
                    })
                } else {
                    match (p.coronet, p.docket) {
                        (Some(coronet), Some(docket)) => {
                            jjrm_dispatch_pace(cmd, &coronet, |gallops| {
                                jjrtl_run_revise_docket(gallops, &coronet, &docket)
                            })
                        }
                        _ => Ok(CallToolResult::error(vec![Content::text(
                            "jjx_redocket: requires either 'input' (gazette) or both 'coronet' and 'docket'".to_string(),
                        )])),
                    }
                }
            }
            "jjx_relabel" => {
                let p = deser!(jjrm_RelabelParams);
                jjrm_result(jjrtl_run_relabel(jjrtl_RelabelArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                    silks: p.silks,
                }))
            }
            "jjx_drop" => {
                let p = deser!(jjrm_DropParams);
                jjrm_result(jjrtl_run_drop(jjrtl_DropArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }))
            }
            "jjx_relocate" => {
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
            "jjx_alter" => {
                let p = deser!(jjrm_AlterParams);
                jjrm_result(jjrfu_run_furlough(jjrfu_FurloughArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    racing: p.racing,
                    stabled: p.stabled,
                    silks: p.silks,
                }))
            }
            "jjx_close" => {
                let p = deser!(jjrm_CloseParams);
                jjrm_result(zjjrx_run_wrap(jjrx_WrapArgs {
                    coronet: p.coronet,
                    size_limit: p.size_limit,
                }, p.summary))
            }
            "jjx_search" => {
                let p = deser!(jjrm_SearchParams);
                jjrm_result(jjrsc_run_scout(jjrsc_ScoutArgs {
                    file: gallops_pathbuf(),
                    pattern: p.pattern,
                    actionable: p.actionable,
                }))
            }
            "jjx_brief" => {
                let p = deser!(jjrm_GetBriefParams);
                jjrm_result(jjrgs_run_get_spec(jjrgs_GetSpecArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }))
            }
            "jjx_coronets" => {
                let p = deser!(jjrm_GetCoronetsParams);
                jjrm_result(jjrgc_run_get_coronets(jjrgc_GetCoronetsArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    remaining: p.remaining,
                    rough: p.rough,
                }))
            }
            "jjx_paddock" => {
                let p = deser!(jjrm_PaddockParams);
                if let Some(ref input) = p.input {
                    let (firemark, content) = match jjrz_parse_paddock_input(input) {
                        Ok(pair) => pair,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(
                            format!("jjx_paddock: gazette input error: {}", e),
                        )])),
                    };
                    jjrm_result(jjrcu_run_curry(jjrcu_CurryArgs {
                        file: gallops_pathbuf(),
                        firemark,
                        note: p.note,
                    }, Some(content)))
                } else {
                    let firemark = match p.firemark {
                        Some(f) => f,
                        None => return Ok(CallToolResult::error(vec![Content::text(
                            "jjx_paddock: requires either 'input' (gazette) or 'firemark'".to_string(),
                        )])),
                    };
                    jjrm_result(jjrcu_run_curry(jjrcu_CurryArgs {
                        file: gallops_pathbuf(),
                        firemark,
                        note: p.note,
                    }, p.content))
                }
            }
            "jjx_continue" => {
                let p = deser!(jjrm_ContinueParams);
                jjrm_result(jjrgl_run_garland(jjrgl_GarlandArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                }))
            }
            "jjx_transfer" => {
                let p = deser!(jjrm_TransferParams);
                jjrm_result(jjrrs_run(jjrrs_RestringArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    to: p.to,
                }, p.coronets))
            }
            "jjx_landing" => {
                let p = deser!(jjrm_LandingParams);
                jjrm_result(jjrld_run_landing(jjrld_LandingArgs {
                    coronet: p.coronet,
                    agent: p.agent,
                }, p.content.unwrap_or_default()))
            }
            _ => {
                Ok(CallToolResult::error(vec![Content::text(format!("jjx: unknown command '{}'\nAvailable: jjx_open, jjx_list, jjx_show, jjx_orient, jjx_record, jjx_log, jjx_validate, jjx_create, jjx_enroll, jjx_close, jjx_archive, jjx_reorder, jjx_redocket, jjx_relabel, jjx_drop, jjx_relocate, jjx_alter, jjx_search, jjx_brief, jjx_coronets, jjx_paddock, jjx_continue, jjx_transfer, jjx_landing", cmd))]))
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
