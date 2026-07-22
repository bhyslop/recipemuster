// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Furlough command - change Heat status or rename
//!
//! Supports toggling between racing/stabled status and/or renaming a Heat.

use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

const JJRFU_CMD_NAME_FURLOUGH: &str = "jjx_furlough";

use crate::jjrf_favor::jjrf_Firemark;
use crate::jjrg_gallops::jjrg_FurloughArgs;
use crate::jjrn_notch::{jjrn_HeatAction, jjrn_format_heat_message};

/// Arguments for jjx_furlough command
#[derive(clap::Args, Debug)]
pub struct jjrfu_FurloughArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Set status to racing
    #[arg(long, conflicts_with = "stabled")]
    pub racing: bool,

    /// Set status to stabled
    #[arg(long, conflicts_with = "racing")]
    pub stabled: bool,

    /// New silks (rename heat)
    #[arg(long, short = 's')]
    pub silks: Option<String>,

    /// Override size limit for staged content guard (bytes)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Handler for jjx_furlough command
pub fn jjrfu_run_furlough(args: jjrfu_FurloughArgs, officium: &str) -> (i32, String) {
    let cn = JJRFU_CMD_NAME_FURLOUGH;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Build description for commit message
    let mut changes = Vec::new();
    if args.racing {
        changes.push("racing".to_string());
    } else if args.stabled {
        changes.push("stabled".to_string());
    }
    if let Some(ref silks) = args.silks {
        changes.push(format!("silks={}", silks));
    }
    let description = changes.join(", ");

    let firemark_str = args.firemark.clone();
    let size_limit = args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
    let furlough_args = jjrg_FurloughArgs {
        firemark: args.firemark,
        racing: args.racing,
        stabled: args.stabled,
        silks: args.silks,
    };

    // fm/message precede the write seam; a malformed firemark is surfaced by
    // jjrg_furlough itself, so on a parse miss run the mutation for its
    // authoritative error (the Ok arm is the old `.expect`'s panic verbatim).
    let fm = match jjrf_Firemark::jjrf_parse(&firemark_str) {
        Ok(fm) => fm,
        Err(_) => {
            return match gallops.jjrg_furlough(furlough_args) {
                Ok(()) => unreachable!("furlough given invalid firemark"),
                Err(e) => {
                    vvco_err!(output, "{}: error: {}", cn, e);
                    (1, output.vvco_finish())
                }
            };
        }
    };
    let message = jjrn_format_heat_message(&fm, jjrn_HeatAction::Furlough, &description);

    match crate::jjrm_mcp::zjjrm_write_gallops(
        &lock,
        &args.file,
        &fm,
        message,
        size_limit,
        &mut output,
        officium,
        cn,
        Vec::new(),
        gallops,
        |g| g.jjrg_furlough(furlough_args),
    ) {
        Ok(((), hash)) => {
            vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
            (0, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            (1, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            (1, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
            vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}
