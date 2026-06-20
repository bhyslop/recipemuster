// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Landing command - record agent landing after autonomous execution
//!
//! The landing command creates an empty commit recording when an autonomous agent
//! completes execution of a dispatched pace. The commit message uses the L (landing)
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
}

/// Execute the landing command
///
/// Creates an empty commit with L marker recording when an agent completes
/// execution of a dispatched pace. Content is the agent completion report.
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

    let marker_args = vvc::vvcc_MarkerArgs {
        prefix: None,
        message,
    };

    let rc = vvc::marker(&marker_args, &mut output);
    (rc, output.vvco_finish())
}
