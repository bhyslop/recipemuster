// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Slate command - add a new Pace to a Heat

use clap::Args;
use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

// Command name constant — RCG String Boundary Discipline
const JJRSL_CMD_NAME_ENROLL: &str = "jjx_enroll";

use crate::jjrf_favor::jjrf_Firemark;
use crate::jjrn_notch::{jjrn_HeatAction, jjrn_format_heat_message};

/// Arguments for jjx_slate command
#[derive(Args, Debug)]
pub struct jjrsl_SlateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Kebab-case display name for the Pace
    #[arg(long, short = 's')]
    pub silks: String,

    /// Insert before specified Coronet
    #[arg(long, conflicts_with_all = ["after", "first"])]
    pub before: Option<String>,

    /// Insert after specified Coronet
    #[arg(long, conflicts_with_all = ["before", "first"])]
    pub after: Option<String>,

    /// Insert at beginning of Heat
    #[arg(long, conflicts_with_all = ["before", "after"])]
    pub first: bool,

    /// Override commit size guard limit in bytes
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Handler for jjx_slate command
///
/// `dictation` and `precis` are the optional original-intent companions from
/// the gazette (operator's verbatim words; LLM distillation) — frozen onto the
/// new pace at this one moment, never writable again.
pub fn jjrsl_run_slate(
    args: jjrsl_SlateArgs,
    docket: String,
    dictation: Option<String>,
    precis: Option<String>,
    officium: &str,
) -> (i32, String) {
    use crate::jjrc_core::jjrc_timestamp_full;
    use crate::jjrg_gallops::jjrg_SlateArgs;
    let cn = JJRSL_CMD_NAME_ENROLL;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let text = docket;

    let mut gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let firemark = args.firemark.clone();
    let silks = args.silks.clone();
    let size_limit = args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
    let slate_args = jjrg_SlateArgs {
        firemark: args.firemark,
        silks: args.silks,
        text,
        dictation,
        precis,
        slated: jjrc_timestamp_full(),
        before: args.before,
        after: args.after,
        first: args.first,
    };

    // fm/message are needed by the write seam before the mutation. A malformed
    // firemark is surfaced by jjrg_slate itself (the pre-seam path reached the
    // same mutation), so on a parse miss run the mutation for its authoritative
    // error and never persist — the Ok arm is the old `.expect`'s panic verbatim.
    let fm = match jjrf_Firemark::jjrf_parse(&firemark) {
        Ok(fm) => fm,
        Err(_) => {
            return match gallops.jjrg_slate(slate_args) {
                Ok(_) => unreachable!("slate given invalid firemark"),
                Err(e) => {
                    vvco_err!(output, "{}: error: {}", cn, e);
                    (1, output.vvco_finish())
                }
            };
        }
    };
    let message = jjrn_format_heat_message(&fm, jjrn_HeatAction::Slate, &silks);

    match crate::jjrm_mcp::zjjrm_write_gallops(
        &lock,
        &args.file,
        &fm,
        message,
        size_limit,
        &mut output,
        officium,
        cn,
        gallops,
        |g| g.jjrg_slate(slate_args),
    ) {
        Ok((result, _hash)) => {
            vvco_out!(output, "{}", result.coronet);
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
