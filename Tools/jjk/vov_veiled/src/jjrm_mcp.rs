// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK MCP Server - MCP tool definitions for all jjx_* operations
//!
//! Defines typed MCP tools that delegate to the same handler functions
//! used by the (now-removed) CLI dispatch path. Each tool call is stateless:
//! lock → load → transform → save → unlock per invocation.
//!
//! stdout capture: Handlers print results to stdout via println!. Since
//! MCP owns stdout for JSON-RPC transport, we redirect the fd during
//! handler execution and capture output as the tool result text.

use std::path::PathBuf;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo, CallToolResult, Content};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};

// Handler imports
use crate::jjrnc_notch::{jjrnc_NotchArgs, jjrnc_run_notch};
use crate::jjrch_chalk::{jjrx_ChalkArgs, jjrx_run_chalk};
use crate::jjrrn_rein::{jjrrn_ReinArgs, jjrrn_run_rein};
use crate::jjrvl_validate::{jjrvl_ValidateArgs, jjrvl_run_validate};
use crate::jjrmu_muster::{jjrmu_MusterArgs, jjrmu_run_muster};
use crate::jjrsd_saddle::{jjrsd_SaddleArgs, jjrsd_run_saddle};
use crate::jjrpd_parade::{jjrpd_ParadeArgs, jjrpd_run_parade};
use crate::jjrrt_retire::{jjrrt_RetireArgs, jjrrt_run_retire};
use crate::jjrno_nominate::{jjrx_NominateArgs, jjrx_run_nominate};
use crate::jjrsl_slate::{jjrsl_SlateArgs, jjrsl_run_slate};
use crate::jjrrl_rail::{jjrrl_RailArgs, jjrrl_run_rail};
use crate::jjrtl_tally::{jjrtl_ReviseDocketArgs, jjrtl_run_revise_docket, jjrtl_ArmArgs, jjrtl_run_arm, jjrtl_RelabelArgs, jjrtl_run_relabel, jjrtl_DropArgs, jjrtl_run_drop};
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

const GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";

fn gallops_pathbuf() -> PathBuf {
    PathBuf::from(GALLOPS_PATH)
}

// ============================================================================
// stdout capture
// ============================================================================

/// Capture stdout from a synchronous handler, returning output as MCP tool result.
/// Uses fd-level redirection with a reader thread to avoid pipe deadlock.
fn jjrm_capture<F: FnOnce() -> i32>(f: F) -> Result<CallToolResult, McpError> {
    use std::io::{Read, Write};
    use std::os::unix::io::FromRawFd;

    unsafe {
        let mut fds = [0i32; 2];
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(McpError::internal_error("pipe creation failed", None));
        }
        let (read_fd, write_fd) = (fds[0], fds[1]);

        let saved = libc::dup(libc::STDOUT_FILENO);
        if saved < 0 {
            libc::close(read_fd);
            libc::close(write_fd);
            return Err(McpError::internal_error("dup stdout failed", None));
        }
        libc::dup2(write_fd, libc::STDOUT_FILENO);
        libc::close(write_fd);

        // Reader thread prevents pipe buffer deadlock
        let reader_handle = std::thread::spawn(move || {
            let mut reader = std::fs::File::from_raw_fd(read_fd);
            let mut output = String::new();
            reader.read_to_string(&mut output).ok();
            output
        });

        let code = f();
        std::io::stdout().flush().ok();

        libc::dup2(saved, libc::STDOUT_FILENO);
        libc::close(saved);

        let output = reader_handle.join().unwrap_or_default();

        if code == 0 {
            Ok(CallToolResult::success(vec![Content::text(output)]))
        } else {
            Ok(CallToolResult::error(vec![Content::text(output)]))
        }
    }
}

/// Async variant of jjrm_capture for handlers that need .await
async fn jjrm_capture_async<F, Fut>(f: F) -> Result<CallToolResult, McpError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = i32>,
{
    use std::io::{Read, Write};
    use std::os::unix::io::FromRawFd;

    unsafe {
        let mut fds = [0i32; 2];
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(McpError::internal_error("pipe creation failed", None));
        }
        let (read_fd, write_fd) = (fds[0], fds[1]);

        let saved = libc::dup(libc::STDOUT_FILENO);
        if saved < 0 {
            libc::close(read_fd);
            libc::close(write_fd);
            return Err(McpError::internal_error("dup stdout failed", None));
        }
        libc::dup2(write_fd, libc::STDOUT_FILENO);
        libc::close(write_fd);

        let code = f().await;
        std::io::stdout().flush().ok();

        libc::dup2(saved, libc::STDOUT_FILENO);
        libc::close(saved);

        // Read captured output (pipe write end closed by dup2 restore)
        let mut reader = std::fs::File::from_raw_fd(read_fd);
        let mut output = String::new();
        reader.read_to_string(&mut output).ok();

        if code == 0 {
            Ok(CallToolResult::success(vec![Content::text(output)]))
        } else {
            Ok(CallToolResult::error(vec![Content::text(output)]))
        }
    }
}

// ============================================================================
// MCP parameter structs
// ============================================================================

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RecordParams {
    #[schemars(description = "Coronet (5-char, pace-affiliated) or Firemark (2-char, heat-only)")]
    pub identity: String,
    #[schemars(description = "Files to commit (at least one required)")]
    pub files: Vec<String>,
    #[schemars(description = "Size limit in bytes (overrides default 50KB guard)")]
    pub size_limit: Option<u64>,
    #[schemars(description = "Commit message intent (overrides haiku-generated message)")]
    pub intent: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_MarkParams {
    #[schemars(description = "Coronet identity for the pace")]
    pub identity: String,
    #[schemars(description = "Marker type (A=approach, R=review, etc.)")]
    pub marker: String,
    #[schemars(description = "Description text for the steeplechase entry")]
    pub description: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_LogParams {
    #[schemars(description = "Heat firemark to show log for")]
    pub firemark: String,
    #[schemars(description = "Maximum number of entries (default 50)")]
    pub limit: Option<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ValidateParams {
    #[schemars(description = "Path to gallops JSON file (default: .claude/jjm/jjg_gallops.json)")]
    pub file: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ListParams {
    #[schemars(description = "Filter by status: racing, stabled, or retired")]
    pub status: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_OrientParams {
    #[schemars(description = "Heat firemark (optional, defaults to auto-select)")]
    pub firemark: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ShowParams {
    #[schemars(description = "Firemark or Coronet to show")]
    pub target: Option<String>,
    #[schemars(description = "Show detailed pace information")]
    #[serde(default)]
    pub detail: bool,
    #[schemars(description = "Show only remaining (incomplete) paces")]
    #[serde(default)]
    pub remaining: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ArchiveParams {
    #[schemars(description = "Heat firemark to archive")]
    pub firemark: String,
    #[schemars(description = "Actually execute the archive (default: dry run)")]
    #[serde(default)]
    pub execute: bool,
    #[schemars(description = "Size limit in bytes for archive commit")]
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CreateParams {
    #[schemars(description = "Kebab-case display name for the new Heat")]
    pub silks: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_EnrollParams {
    #[schemars(description = "Heat firemark to add the pace to")]
    pub firemark: String,
    #[schemars(description = "Kebab-case display name for the new pace")]
    pub silks: String,
    #[schemars(description = "Docket text (description/spec for the pace)")]
    pub docket: String,
    #[schemars(description = "Insert before this coronet")]
    pub before: Option<String>,
    #[schemars(description = "Insert after this coronet")]
    pub after: Option<String>,
    #[schemars(description = "Insert at the beginning of the pace list")]
    #[serde(default)]
    pub first: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ReorderParams {
    #[schemars(description = "Heat firemark")]
    pub firemark: String,
    #[schemars(description = "Coronet to move within order")]
    pub r#move: Option<String>,
    #[schemars(description = "Move before this coronet")]
    pub before: Option<String>,
    #[schemars(description = "Move after this coronet")]
    pub after: Option<String>,
    #[schemars(description = "Move to beginning")]
    #[serde(default)]
    pub first: bool,
    #[schemars(description = "Move to end")]
    #[serde(default)]
    pub last: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ReviseDocketParams {
    #[schemars(description = "Pace coronet")]
    pub coronet: String,
    #[schemars(description = "New docket text")]
    pub docket: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ArmParams {
    #[schemars(description = "Pace coronet")]
    pub coronet: String,
    #[schemars(description = "Warrant text for bridling the pace")]
    pub warrant: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RelabelParams {
    #[schemars(description = "Pace coronet")]
    pub coronet: String,
    #[schemars(description = "New silks (kebab-case display name)")]
    pub silks: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_DropParams {
    #[schemars(description = "Pace coronet to abandon")]
    pub coronet: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_RelocateParams {
    #[schemars(description = "Pace coronet to move")]
    pub coronet: String,
    #[schemars(description = "Destination heat firemark")]
    pub to: String,
    #[schemars(description = "Insert before this coronet in destination")]
    pub before: Option<String>,
    #[schemars(description = "Insert after this coronet in destination")]
    pub after: Option<String>,
    #[schemars(description = "Insert at beginning of destination pace list")]
    #[serde(default)]
    pub first: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_AlterParams {
    #[schemars(description = "Heat firemark")]
    pub firemark: String,
    #[schemars(description = "Set heat to racing status")]
    #[serde(default)]
    pub racing: bool,
    #[schemars(description = "Set heat to stabled status")]
    #[serde(default)]
    pub stabled: bool,
    #[schemars(description = "New silks (kebab-case display name)")]
    pub silks: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CloseParams {
    #[schemars(description = "Pace coronet to close")]
    pub coronet: String,
    #[schemars(description = "Summary of work accomplished")]
    pub summary: Option<String>,
    #[schemars(description = "Size limit in bytes for close commit")]
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_SearchParams {
    #[schemars(description = "Regex pattern to search across heats and paces")]
    pub pattern: String,
    #[schemars(description = "Only show actionable (incomplete) results")]
    #[serde(default)]
    pub actionable: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_GetBriefParams {
    #[schemars(description = "Pace coronet to get docket for")]
    pub coronet: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_GetCoronetsParams {
    #[schemars(description = "Heat firemark")]
    pub firemark: String,
    #[schemars(description = "Only show remaining (incomplete) paces")]
    #[serde(default)]
    pub remaining: bool,
    #[schemars(description = "Only show rough (unstarted) paces")]
    #[serde(default)]
    pub rough: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_PaddockParams {
    #[schemars(description = "Heat firemark")]
    pub firemark: String,
    #[schemars(description = "Paddock content to set (omit to get current paddock)")]
    pub content: Option<String>,
    #[schemars(description = "Append note to existing paddock")]
    pub note: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ContinueParams {
    #[schemars(description = "Heat firemark to garland and continue")]
    pub firemark: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_TransferParams {
    #[schemars(description = "Source heat firemark")]
    pub firemark: String,
    #[schemars(description = "Destination heat firemark")]
    pub to: String,
    #[schemars(description = "JSON array of coronets to transfer, e.g. [\"ABCDe\",\"ABCDf\"]")]
    pub coronets: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_LandingParams {
    #[schemars(description = "Pace coronet")]
    pub coronet: String,
    #[schemars(description = "Agent identifier")]
    pub agent: String,
    #[schemars(description = "Landing content/report")]
    pub content: Option<String>,
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

    #[tool(name = "jjx_record", description = "JJ-aware commit with heat/pace context prefix. Stages specified files and commits with identity-derived message prefix.")]
    fn record(&self, Parameters(p): Parameters<jjrm_RecordParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrnc_run_notch(jjrnc_NotchArgs {
            identity: p.identity,
            files: p.files,
            size_limit: p.size_limit,
            intent: p.intent,
        }))
    }

    #[tool(name = "jjx_mark", description = "Create empty commit marking a steeplechase event (A=approach, R=review, etc.)")]
    fn mark(&self, Parameters(p): Parameters<jjrm_MarkParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrx_run_chalk(jjrx_ChalkArgs {
            identity: p.identity,
            marker: p.marker,
            description: p.description,
        }))
    }

    #[tool(name = "jjx_log", description = "Parse git history for steeplechase entries affiliated with a heat")]
    fn log(&self, Parameters(p): Parameters<jjrm_LogParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrrn_run_rein(jjrrn_ReinArgs {
            firemark: p.firemark,
            limit: p.limit.unwrap_or(50),
        }))
    }

    #[tool(name = "jjx_validate", description = "Validate Gallops JSON schema integrity")]
    fn validate(&self, Parameters(p): Parameters<jjrm_ValidateParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrvl_run_validate(jjrvl_ValidateArgs {
            file: p.file.map(PathBuf::from).unwrap_or_else(gallops_pathbuf),
        }))
    }

    #[tool(name = "jjx_list", description = "List all Heats with status and pace completion counts")]
    async fn list(&self, Parameters(p): Parameters<jjrm_ListParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture_async(|| jjrmu_run_muster(jjrmu_MusterArgs {
            file: gallops_pathbuf(),
            status: p.status,
        })).await
    }

    #[tool(name = "jjx_orient", description = "Get saddling context for a Heat: racing heats, paddock, next pace, docket, recent work, and file-touch bitmap")]
    async fn orient(&self, Parameters(p): Parameters<jjrm_OrientParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture_async(|| jjrsd_run_saddle(jjrsd_SaddleArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
        })).await
    }

    #[tool(name = "jjx_show", description = "Display comprehensive Heat or Pace status for project review")]
    fn show(&self, Parameters(p): Parameters<jjrm_ShowParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrpd_run_parade(jjrpd_ParadeArgs {
            file: gallops_pathbuf(),
            target: p.target,
            detail: p.detail,
            remaining: p.remaining,
        }))
    }

    #[tool(name = "jjx_archive", description = "Extract complete Heat data for archival trophy and retire the heat")]
    fn archive(&self, Parameters(p): Parameters<jjrm_ArchiveParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrrt_run_retire(jjrrt_RetireArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
            execute: p.execute,
            size_limit: p.size_limit,
        }))
    }

    #[tool(name = "jjx_create", description = "Create a new Heat with empty Pace structure")]
    fn create(&self, Parameters(p): Parameters<jjrm_CreateParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrx_run_nominate(jjrx_NominateArgs {
            file: gallops_pathbuf(),
            silks: p.silks,
        }))
    }

    #[tool(name = "jjx_enroll", description = "Add a new Pace to a Heat with docket text")]
    fn enroll(&self, Parameters(p): Parameters<jjrm_EnrollParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrsl_run_slate(jjrsl_SlateArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
            silks: p.silks,
            before: p.before,
            after: p.after,
            first: p.first,
        }, p.docket))
    }

    #[tool(name = "jjx_reorder", description = "Reorder Paces within a Heat using move semantics")]
    fn reorder(&self, Parameters(p): Parameters<jjrm_ReorderParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrrl_run_rail(jjrrl_RailArgs {
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

    #[tool(name = "jjx_revise_docket", description = "Update pace docket text")]
    fn revise_docket(&self, Parameters(p): Parameters<jjrm_ReviseDocketParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrtl_run_revise_docket(jjrtl_ReviseDocketArgs {
            file: gallops_pathbuf(),
            coronet: p.coronet,
        }, p.docket))
    }

    #[tool(name = "jjx_arm", description = "Set pace state to bridled with warrant text")]
    fn arm(&self, Parameters(p): Parameters<jjrm_ArmParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrtl_run_arm(jjrtl_ArmArgs {
            file: gallops_pathbuf(),
            coronet: p.coronet,
        }, p.warrant))
    }

    #[tool(name = "jjx_relabel", description = "Rename pace silks (display name)")]
    fn relabel(&self, Parameters(p): Parameters<jjrm_RelabelParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrtl_run_relabel(jjrtl_RelabelArgs {
            file: gallops_pathbuf(),
            coronet: p.coronet,
            silks: p.silks,
        }))
    }

    #[tool(name = "jjx_drop", description = "Set pace state to abandoned")]
    fn drop_pace(&self, Parameters(p): Parameters<jjrm_DropParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrtl_run_drop(jjrtl_DropArgs {
            file: gallops_pathbuf(),
            coronet: p.coronet,
        }))
    }

    #[tool(name = "jjx_relocate", description = "Move a Pace from one Heat to another")]
    fn relocate(&self, Parameters(p): Parameters<jjrm_RelocateParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrdr_run_draft(jjrdr_DraftArgs {
            file: gallops_pathbuf(),
            coronet: p.coronet,
            to: p.to,
            before: p.before,
            after: p.after,
            first: p.first,
        }))
    }

    #[tool(name = "jjx_alter", description = "Change Heat status (racing/stabled) or rename silks")]
    fn alter(&self, Parameters(p): Parameters<jjrm_AlterParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrfu_run_furlough(jjrfu_FurloughArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
            racing: p.racing,
            stabled: p.stabled,
            silks: p.silks,
        }))
    }

    #[tool(name = "jjx_close", description = "Mark a pace complete, commit all uncommitted changes in one operation")]
    fn close(&self, Parameters(p): Parameters<jjrm_CloseParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| zjjrx_run_wrap(jjrx_WrapArgs {
            coronet: p.coronet,
            size_limit: p.size_limit,
        }, p.summary))
    }

    #[tool(name = "jjx_search", description = "Search across heats and paces with regex pattern")]
    fn search(&self, Parameters(p): Parameters<jjrm_SearchParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrsc_run_scout(jjrsc_ScoutArgs {
            file: gallops_pathbuf(),
            pattern: p.pattern,
            actionable: p.actionable,
        }))
    }

    #[tool(name = "jjx_get_brief", description = "Get raw docket text for a Pace")]
    fn get_brief(&self, Parameters(p): Parameters<jjrm_GetBriefParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrgs_run_get_spec(jjrgs_GetSpecArgs {
            file: gallops_pathbuf(),
            coronet: p.coronet,
        }))
    }

    #[tool(name = "jjx_get_coronets", description = "List Coronets for a Heat with optional filtering")]
    fn get_coronets(&self, Parameters(p): Parameters<jjrm_GetCoronetsParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrgc_run_get_coronets(jjrgc_GetCoronetsArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
            remaining: p.remaining,
            rough: p.rough,
        }))
    }

    #[tool(name = "jjx_paddock", description = "Get or set Heat paddock content. Omit content to read; provide content to write.")]
    fn paddock(&self, Parameters(p): Parameters<jjrm_PaddockParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrcu_run_curry(jjrcu_CurryArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
            note: p.note,
        }, p.content))
    }

    #[tool(name = "jjx_continue", description = "Garland a heat - celebrate completion and create continuation heat")]
    fn r#continue(&self, Parameters(p): Parameters<jjrm_ContinueParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrgl_run_garland(jjrgl_GarlandArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
        }))
    }

    #[tool(name = "jjx_transfer", description = "Bulk transfer multiple paces between heats atomically")]
    fn transfer(&self, Parameters(p): Parameters<jjrm_TransferParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrrs_run(jjrrs_RestringArgs {
            file: gallops_pathbuf(),
            firemark: p.firemark,
            to: p.to,
        }, p.coronets))
    }

    #[tool(name = "jjx_landing", description = "Record agent landing after autonomous execution")]
    fn landing(&self, Parameters(p): Parameters<jjrm_LandingParams>) -> Result<CallToolResult, McpError> {
        jjrm_capture(|| jjrld_run_landing(jjrld_LandingArgs {
            coronet: p.coronet,
            agent: p.agent,
        }, p.content.unwrap_or_default()))
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
