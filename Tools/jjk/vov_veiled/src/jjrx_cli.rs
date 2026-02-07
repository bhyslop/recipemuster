// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK CLI - Command line interface for Job Jockey Kit
//!
//! This module owns command definitions and dispatch logic for jjx_* commands.
//! VOK delegates unknown subcommands here via external_subcommand pattern.
//!
//! Command implementations are in separate per-command modules (jjrxx_*.rs).
//! This module imports Args from those modules and dispatches to their handlers.

use clap::Parser;
use std::ffi::OsString;

// Import Args and handlers from per-command modules
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

/// JJK subcommands - all jjx_* commands
#[derive(Parser)]
#[command(name = "jjx")]
#[command(about = "Job Jockey Kit commands")]
pub enum jjrx_JjxCommands {
    /// JJ-aware commit with heat/pace context prefix
    #[command(name = "jjx_record")]
    Record(jjrnc_NotchArgs),

    /// Empty commit marking a steeplechase event
    #[command(name = "jjx_mark")]
    Mark(jjrx_ChalkArgs),

    /// Parse git history for steeplechase entries
    #[command(name = "jjx_log")]
    Log(jjrrn_ReinArgs),

    /// Validate Gallops JSON schema
    #[command(name = "jjx_validate")]
    Validate(jjrvl_ValidateArgs),

    /// List all Heats with summary information
    #[command(name = "jjx_list")]
    List(jjrmu_MusterArgs),

    /// Return context needed to saddle up on a Heat
    #[command(name = "jjx_orient")]
    Orient(jjrsd_SaddleArgs),

    /// Display comprehensive Heat status for project review
    #[command(name = "jjx_show")]
    Show(jjrpd_ParadeArgs),

    /// Extract complete Heat data for archival trophy
    #[command(name = "jjx_archive")]
    Archive(jjrrt_RetireArgs),

    /// Create a new Heat with empty Pace structure
    #[command(name = "jjx_create")]
    Create(jjrx_NominateArgs),

    /// Add a new Pace to a Heat
    #[command(name = "jjx_enroll")]
    Enroll(jjrsl_SlateArgs),

    /// Reorder Paces within a Heat
    #[command(name = "jjx_reorder")]
    Reorder(jjrrl_RailArgs),

    /// Update pace docket text (stdin)
    #[command(name = "jjx_revise_docket")]
    ReviseDocket(jjrtl_ReviseDocketArgs),

    /// Set pace state to bridled with warrant (stdin)
    #[command(name = "jjx_arm")]
    Arm(jjrtl_ArmArgs),

    /// Rename pace silks
    #[command(name = "jjx_relabel")]
    Relabel(jjrtl_RelabelArgs),

    /// Set pace state to abandoned
    #[command(name = "jjx_drop")]
    Drop(jjrtl_DropArgs),

    /// Move a Pace from one Heat to another
    #[command(name = "jjx_relocate")]
    Relocate(jjrdr_DraftArgs),

    /// Change Heat status (racing/stabled) or rename
    #[command(name = "jjx_alter")]
    Alter(jjrfu_FurloughArgs),

    /// Mark a pace complete and commit in one operation
    #[command(name = "jjx_close")]
    Close(jjrx_WrapArgs),

    /// Search across heats and paces with regex
    #[command(name = "jjx_search")]
    Search(jjrsc_ScoutArgs),

    /// Get raw docket text for a Pace
    #[command(name = "jjx_get_brief")]
    GetBrief(jjrgs_GetSpecArgs),

    /// List Coronets for a Heat
    #[command(name = "jjx_get_coronets")]
    GetCoronets(jjrgc_GetCoronetsArgs),

    /// Get or update Heat paddock (getter/setter)
    #[command(name = "jjx_paddock")]
    Paddock(jjrcu_CurryArgs),

    /// Garland a heat - celebrate completion and create continuation
    #[command(name = "jjx_continue")]
    Continue(jjrgl_GarlandArgs),

    /// Bulk draft multiple paces between heats atomically
    #[command(name = "jjx_transfer")]
    Transfer(jjrrs_RestringArgs),

    /// Record agent landing after autonomous execution
    #[command(name = "jjx_landing")]
    Landing(jjrld_LandingArgs),
}

// ============================================================================
// Public dispatch API
// ============================================================================

/// Dispatch JJK commands from raw arguments.
///
/// Called by VOK when it receives an external subcommand starting with "jjx_".
/// The first element of `args` should be the subcommand name (e.g., "jjx_muster").
///
/// Returns exit code (0 for success, non-zero for failure).
pub async fn jjrx_dispatch(args: &[OsString]) -> i32 {
    // Prepend synthetic binary name for clap parsing.
    // Clap expects args[0] to be the binary name (used in help text).
    // VOK passes ["jjx_muster", ...] so we prepend "jjx" to get ["jjx", "jjx_muster", ...].
    let mut full_args = vec![OsString::from("jjx")];
    full_args.extend(args.iter().cloned());

    // Parse the subcommand and arguments
    let parsed = match jjrx_JjxCommands::try_parse_from(&full_args) {
        Ok(cmd) => cmd,
        Err(e) => {
            // Let clap handle help/version/error display
            e.print().ok();
            return if e.kind() == clap::error::ErrorKind::DisplayHelp
                   || e.kind() == clap::error::ErrorKind::DisplayVersion {
                0
            } else {
                1
            };
        }
    };

    match parsed {
        jjrx_JjxCommands::Record(args) => jjrnc_run_notch(args),
        jjrx_JjxCommands::Mark(args) => jjrx_run_chalk(args),
        jjrx_JjxCommands::Log(args) => jjrrn_run_rein(args),
        jjrx_JjxCommands::Validate(args) => jjrvl_run_validate(args),
        jjrx_JjxCommands::List(args) => jjrmu_run_muster(args).await,
        jjrx_JjxCommands::Orient(args) => jjrsd_run_saddle(args).await,
        jjrx_JjxCommands::Show(args) => jjrpd_run_parade(args),
        jjrx_JjxCommands::Archive(args) => jjrrt_run_retire(args),
        jjrx_JjxCommands::Create(args) => jjrx_run_nominate(args),
        jjrx_JjxCommands::Enroll(args) => jjrsl_run_slate(args),
        jjrx_JjxCommands::Reorder(args) => jjrrl_run_rail(args),
        jjrx_JjxCommands::ReviseDocket(args) => jjrtl_run_revise_docket(args),
        jjrx_JjxCommands::Arm(args) => jjrtl_run_arm(args),
        jjrx_JjxCommands::Relabel(args) => jjrtl_run_relabel(args),
        jjrx_JjxCommands::Drop(args) => jjrtl_run_drop(args),
        jjrx_JjxCommands::Relocate(args) => jjrdr_run_draft(args),
        jjrx_JjxCommands::Alter(args) => jjrfu_run_furlough(args),
        jjrx_JjxCommands::Close(args) => zjjrx_run_wrap(args),
        jjrx_JjxCommands::Search(args) => jjrsc_run_scout(args),
        jjrx_JjxCommands::GetBrief(args) => jjrgs_run_get_spec(args),
        jjrx_JjxCommands::GetCoronets(args) => jjrgc_run_get_coronets(args),
        jjrx_JjxCommands::Paddock(args) => jjrcu_run_curry(args),
        jjrx_JjxCommands::Continue(args) => jjrgl_run_garland(args),
        jjrx_JjxCommands::Transfer(args) => jjrrs_run(args),
        jjrx_JjxCommands::Landing(args) => jjrld_run_landing(args),
    }
}

/// Check if a command name is a JJK command
pub fn jjrx_is_jjk_command(name: &str) -> bool {
    name.starts_with("jjx_")
}
