// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Landing command - record agent landing after autonomous execution
//!
//! The landing command creates an empty commit recording when an autonomous agent
//! completes execution of a dispatched pace. The commit message uses the L (landing)
//! marker in the steeplechase format, and the billet branch is consigned to its
//! remote counterpart as part of landing — remote custody of the L commit never
//! waits on a session exit (a same-session land→wrap never fires one).

use clap::Args;
use vvc::{vvco_err, vvco_Output};
use crate::jjrf_favor::jjrf_Coronet;
use crate::jjrfg_plaingit::jjrfg_PlainGit;
use crate::jjrn_notch::jjrn_format_landing_message;
use crate::jjrnc_notch::jjrnc_consign_current_branch;

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
/// execution of a dispatched pace, then consigns the billet branch so the
/// landing stands in remote custody. Content is the agent completion report.
///
/// Returns exit code (0 for success, non-zero for failure).
pub fn jjrld_run_landing(args: jjrld_LandingArgs, content: String) -> (i32, String) {
    let cn = JJRLD_CMD_NAME_LANDING;
    let mut output = vvco_Output::buffer();

    // Parse coronet
    let coronet = match jjrf_Coronet::jjrf_parse(&args.coronet) {
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

    // The weld: landing consigns the billet branch, so the L commit reaches
    // remote custody when landing completes — never deferred to a session
    // exit, which a same-session land→wrap never fires. The commit already
    // landed locally, so a push failure here cannot be undone (additive
    // discipline) — it surfaces loud instead, turning the local-only landing
    // into a reportable gap rather than a silent one. The exit-stile's litmus
    // remains the backstop: it refuses to clear a billet this push never
    // reached.
    let rc = if rc == 0 {
        match jjrnc_consign_current_branch(&jjrfg_PlainGit) {
            Ok(()) => rc,
            Err(e) => {
                vvco_err!(output, "{}: error: landing committed locally, but consigning the billet branch failed: {}", cn, e);
                1
            }
        }
    } else {
        rc
    };

    (rc, output.vvco_finish())
}
