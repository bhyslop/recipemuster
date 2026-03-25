// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK Tally commands - pace modification operations
//!
//! Split into three single-purpose commands:
//! - revise_docket: Update pace docket text
//! - relabel: Rename pace silks
//! - drop: Set state to abandoned

use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState};
use crate::jjrn_notch::{jjrn_format_heat_message as format_heat_message, jjrn_HeatAction as HeatAction};

/// Arguments for jjx_redocket command
#[derive(clap::Args, Debug)]
pub struct jjrtl_ReviseDocketArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the revise_docket procedure within the dispatch lifecycle.
///
/// Receives &mut Gallops from dispatcher. Returns output text.
/// Lock, load, and persist are owned by the dispatcher (jjsodp_command_lifecycle).
pub fn jjrtl_run_revise_docket(
    gallops: &mut Gallops,
    coronet: &str,
    docket: &str,
) -> Result<String, String> {
    // Capture I/O at procedure boundary — method is pure
    let basis = crate::jjru_util::jjrg_capture_commit_sha();
    let ts = crate::jjrc_core::jjrc_timestamp_full();

    gallops.jjrg_revise_docket(coronet, docket, &basis, &ts)?;

    Ok(String::new())
}

/// Arguments for jjx_relabel command
#[derive(clap::Args, Debug)]
pub struct jjrtl_RelabelArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,

    /// New silks (kebab-case display name)
    #[arg(long)]
    pub silks: String,
}

/// Run the relabel command
///
/// Renames the pace silks.
pub fn jjrtl_run_relabel(args: jjrtl_RelabelArgs) -> (i32, String) {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "jjx_relabel: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "jjx_relabel: error loading Gallops: {}", e);
            return (1, output.vvco_finish());
        }
    };

    // Get firemark for commit message before we move args
    let coronet_str = args.coronet.clone();
    let new_silks = args.silks.clone();
    let fm = match Coronet::jjrf_parse(&coronet_str) {
        Ok(c) => c.jjrf_parent_firemark(),
        Err(e) => {
            vvco_err!(output, "jjx_relabel: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state: None,
        direction: None,
        text: None,
        silks: Some(args.silks),
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &new_silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000, &mut output) {
                Ok(hash) => {
                    vvco_out!(output, "committed {}", hash);
                    (0, output.vvco_finish())
                }
                Err(e) => {
                    vvco_err!(output, "jjx_relabel: error: {}", e);
                    (1, output.vvco_finish())
                }
            }
        }
        Err(e) => {
            vvco_err!(output, "jjx_relabel: error: {}", e);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}

/// Arguments for jjx_drop command
#[derive(clap::Args, Debug)]
pub struct jjrtl_DropArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the drop command
///
/// Sets pace state to abandoned.
pub fn jjrtl_run_drop(args: jjrtl_DropArgs) -> (i32, String) {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "jjx_drop: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "jjx_drop: error loading Gallops: {}", e);
            return (1, output.vvco_finish());
        }
    };

    // Get firemark and silks for commit message before we move args
    let coronet_str = args.coronet.clone();
    let (fm, silks) = match Coronet::jjrf_parse(&coronet_str) {
        Ok(c) => {
            let parent_fm = c.jjrf_parent_firemark();
            let silks = gallops.heats.get(&parent_fm.jjrf_display())
                .and_then(|h| h.paces.get(&c.jjrf_display()))
                .and_then(|p| p.tacks.first().map(|t| t.silks.clone()))
                .unwrap_or_else(|| coronet_str.clone());
            (parent_fm, silks)
        }
        Err(e) => {
            vvco_err!(output, "jjx_drop: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state: Some(PaceState::Abandoned),
        direction: None,
        text: None,
        silks: None,
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000, &mut output) {
                Ok(hash) => {
                    vvco_out!(output, "committed {}", hash);
                    (0, output.vvco_finish())
                }
                Err(e) => {
                    vvco_err!(output, "jjx_drop: error: {}", e);
                    (1, output.vvco_finish())
                }
            }
        }
        Err(e) => {
            vvco_err!(output, "jjx_drop: error: {}", e);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}
