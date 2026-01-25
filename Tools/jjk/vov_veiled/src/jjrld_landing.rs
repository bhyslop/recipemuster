// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Landing command - record agent landing after autonomous execution
//!
//! The landing command creates an empty commit recording when an autonomous agent
//! completes execution of a bridled pace. The commit message uses the L (landing)
//! marker in the steeplechase format.

use clap::Args;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrn_notch::jjrn_format_landing_message;

/// Arguments for jjx_landing command
#[derive(Args, Debug)]
pub struct jjrld_LandingArgs {
    /// Pace identity (Coronet)
    pub coronet: String,

    /// Agent tier that executed the pace
    pub agent: String,
}

/// Execute the landing command
///
/// Creates an empty commit with L marker recording when an agent completes
/// execution of a bridled pace.
///
/// Returns exit code (0 for success, non-zero for failure).
pub fn jjrld_run_landing(args: jjrld_LandingArgs) -> i32 {
    // Parse coronet
    let coronet = match Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("jjx_landing: error: {}", e);
            return 1;
        }
    };

    // Use jjrn_format_landing_message to create the commit message
    let message = jjrn_format_landing_message(&coronet, &args.agent);

    // Create empty landing commit
    let commit_args = vvc::vvcc_CommitArgs {
        prefix: None,
        message: Some(message),
        allow_empty: true,
        no_stage: true,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    vvc::commit(&commit_args)
}
