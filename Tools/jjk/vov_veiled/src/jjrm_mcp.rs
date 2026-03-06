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
pub struct jjrm_MarkParams {
    pub identity: String,
    pub marker: String,
    pub description: String,
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
    #[serde(default)]
    pub execute: bool,
    pub size_limit: Option<u64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_CreateParams {
    pub silks: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_EnrollParams {
    pub firemark: String,
    pub silks: String,
    pub docket: String,
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
    pub coronet: String,
    pub docket: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_ArmParams {
    pub coronet: String,
    pub warrant: String,
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
    pub firemark: String,
    pub content: Option<String>,
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

// ============================================================================
// Single dispatcher tool params
// ============================================================================

fn jjrm_empty_object() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct jjrm_JjxParams {
    #[schemars(description = "Command name: list, show, orient, record, mark, log, validate, create, enroll, close, archive, reorder, revise_docket, arm, relabel, drop, relocate, alter, search, get_brief, get_coronets, paddock, continue, transfer, landing")]
    pub command: String,
    #[schemars(description = "Command parameters as JSON object. See CLAUDE.md for per-command schemas.")]
    #[serde(default = "jjrm_empty_object")]
    pub params: serde_json::Value,
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
        let cmd = p.command.strip_prefix("jjx_").unwrap_or(&p.command);
        let v = p.params;

        macro_rules! deser {
            ($t:ty) => {
                match serde_json::from_value::<$t>(v) {
                    Ok(p) => p,
                    Err(e) => return jjrm_deser_error(cmd, e),
                }
            }
        }

        match cmd {
            "record" => {
                let p = deser!(jjrm_RecordParams);
                jjrm_result(jjrnc_run_notch(jjrnc_NotchArgs {
                    identity: p.identity,
                    files: p.files,
                    size_limit: p.size_limit,
                    intent: p.intent,
                }))
            }
            "mark" => {
                let p = deser!(jjrm_MarkParams);
                jjrm_result(jjrx_run_chalk(jjrx_ChalkArgs {
                    identity: p.identity,
                    marker: p.marker,
                    description: p.description,
                }))
            }
            "log" => {
                let p = deser!(jjrm_LogParams);
                jjrm_result(jjrrn_run_rein(jjrrn_ReinArgs {
                    firemark: p.firemark,
                    limit: p.limit.unwrap_or(50),
                }))
            }
            "validate" => {
                let _p = deser!(jjrm_ValidateParams);
                jjrm_result(jjrvl_run_validate(jjrvl_ValidateArgs {
                    file: gallops_pathbuf(),
                }))
            }
            "list" => {
                let p = deser!(jjrm_ListParams);
                jjrm_result(jjrmu_run_muster(jjrmu_MusterArgs {
                    file: gallops_pathbuf(),
                    status: p.status,
                }).await)
            }
            "orient" => {
                let p = deser!(jjrm_OrientParams);
                jjrm_result(jjrsd_run_saddle(jjrsd_SaddleArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                }).await)
            }
            "show" => {
                let p = deser!(jjrm_ShowParams);
                jjrm_result(jjrpd_run_parade(jjrpd_ParadeArgs {
                    file: gallops_pathbuf(),
                    target: p.target,
                    detail: p.detail,
                    remaining: p.remaining,
                }))
            }
            "archive" => {
                let p = deser!(jjrm_ArchiveParams);
                jjrm_result(jjrrt_run_retire(jjrrt_RetireArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    execute: p.execute,
                    size_limit: p.size_limit,
                }))
            }
            "create" => {
                let p = deser!(jjrm_CreateParams);
                jjrm_result(jjrx_run_nominate(jjrx_NominateArgs {
                    file: gallops_pathbuf(),
                    silks: p.silks,
                }))
            }
            "enroll" => {
                let p = deser!(jjrm_EnrollParams);
                jjrm_result(jjrsl_run_slate(jjrsl_SlateArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    silks: p.silks,
                    before: p.before,
                    after: p.after,
                    first: p.first,
                }, p.docket))
            }
            "reorder" => {
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
            "revise_docket" => {
                let p = deser!(jjrm_ReviseDocketParams);
                jjrm_result(jjrtl_run_revise_docket(jjrtl_ReviseDocketArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }, p.docket))
            }
            "arm" => {
                let p = deser!(jjrm_ArmParams);
                jjrm_result(jjrtl_run_arm(jjrtl_ArmArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }, p.warrant))
            }
            "relabel" => {
                let p = deser!(jjrm_RelabelParams);
                jjrm_result(jjrtl_run_relabel(jjrtl_RelabelArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                    silks: p.silks,
                }))
            }
            "drop" => {
                let p = deser!(jjrm_DropParams);
                jjrm_result(jjrtl_run_drop(jjrtl_DropArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }))
            }
            "relocate" => {
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
            "alter" => {
                let p = deser!(jjrm_AlterParams);
                jjrm_result(jjrfu_run_furlough(jjrfu_FurloughArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    racing: p.racing,
                    stabled: p.stabled,
                    silks: p.silks,
                }))
            }
            "close" => {
                let p = deser!(jjrm_CloseParams);
                jjrm_result(zjjrx_run_wrap(jjrx_WrapArgs {
                    coronet: p.coronet,
                    size_limit: p.size_limit,
                }, p.summary))
            }
            "search" => {
                let p = deser!(jjrm_SearchParams);
                jjrm_result(jjrsc_run_scout(jjrsc_ScoutArgs {
                    file: gallops_pathbuf(),
                    pattern: p.pattern,
                    actionable: p.actionable,
                }))
            }
            "get_brief" => {
                let p = deser!(jjrm_GetBriefParams);
                jjrm_result(jjrgs_run_get_spec(jjrgs_GetSpecArgs {
                    file: gallops_pathbuf(),
                    coronet: p.coronet,
                }))
            }
            "get_coronets" => {
                let p = deser!(jjrm_GetCoronetsParams);
                jjrm_result(jjrgc_run_get_coronets(jjrgc_GetCoronetsArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    remaining: p.remaining,
                    rough: p.rough,
                }))
            }
            "paddock" => {
                let p = deser!(jjrm_PaddockParams);
                jjrm_result(jjrcu_run_curry(jjrcu_CurryArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    note: p.note,
                }, p.content))
            }
            "continue" => {
                let p = deser!(jjrm_ContinueParams);
                jjrm_result(jjrgl_run_garland(jjrgl_GarlandArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                }))
            }
            "transfer" => {
                let p = deser!(jjrm_TransferParams);
                jjrm_result(jjrrs_run(jjrrs_RestringArgs {
                    file: gallops_pathbuf(),
                    firemark: p.firemark,
                    to: p.to,
                }, p.coronets))
            }
            "landing" => {
                let p = deser!(jjrm_LandingParams);
                jjrm_result(jjrld_run_landing(jjrld_LandingArgs {
                    coronet: p.coronet,
                    agent: p.agent,
                }, p.content.unwrap_or_default()))
            }
            _ => {
                Ok(CallToolResult::error(vec![Content::text(format!("jjx: unknown command '{}'\nAvailable: list, show, orient, record, mark, log, validate, create, enroll, close, archive, reorder, revise_docket, arm, relabel, drop, relocate, alter, search, get_brief, get_coronets, paddock, continue, transfer, landing", cmd))]))
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
