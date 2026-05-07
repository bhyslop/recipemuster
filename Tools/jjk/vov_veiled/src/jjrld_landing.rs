// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Landing command - record agent landing after autonomous execution
//!
//! The landing command creates an empty commit recording when an autonomous agent
//! completes execution of a bridled pace. The commit message uses the L (landing)
//! marker in the steeplechase format.

use clap::Args;
use vvc::{vvco_err, vvco_Output};
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrn_notch::jjrn_format_landing_message;

const JJRLD_CMD_NAME_LANDING: &str = "jjx_landing";

/// Arguments for jjx_landing command
#[derive(Args, Debug)]
pub struct jjrld_LandingArgs {
    /// Pace identity (Coronet)
    pub coronet: String,

    /// Agent tier that executed the pace
    pub agent: String,

    /// Override commit size guard limit in bytes
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Execute the landing command
///
/// Creates an empty commit with L marker recording when an agent completes
/// execution of a bridled pace. Content is the agent completion report.
///
/// Returns exit code (0 for success, non-zero for failure).
pub fn jjrld_run_landing(args: jjrld_LandingArgs, content: String) -> (i32, String) {
    let cn = JJRLD_CMD_NAME_LANDING;
    let mut output = vvco_Output::buffer();

    // Parse coronet
    let coronet = match Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Use jjrn_format_landing_message to create the commit message, with content as body
    let base_message = jjrn_format_landing_message(&coronet, &args.agent);
    let message = if content.trim().is_empty() {
        base_message
    } else {
        format!("{}\n\n{}", base_message, content.trim())
    };

    // Create empty landing commit
    let commit_args = vvc::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT),
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    let rc = vvc::commit(&commit_args, &mut output);
    (rc, output.vvco_finish())
}
