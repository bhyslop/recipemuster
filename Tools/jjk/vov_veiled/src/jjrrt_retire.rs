// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! jjx_retire command - Extract complete Heat data for archival trophy
//!
//! Handles retire operation: write trophy, remove from gallops, delete paddock, commit.
//! Fails fast if gallops file has uncommitted changes.

use std::path::{Path, PathBuf};

use vvc::{vvco_out, vvco_err, vvco_Output};

// Command name constant — RCG String Boundary Discipline
const JJRRT_CMD_NAME_ARCHIVE: &str = "jjx_archive";

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_RetireArgs as LibRetireArgs};
use crate::jjrs_steeplechase::{jjrs_ReinArgs as ReinArgs, jjrs_get_entries as get_entries};
use crate::jjrc_core::jjrc_timestamp_date;
use crate::jjrn_notch::{jjrn_format_heat_message, jjrn_HeatAction};

/// Arguments for jjx_retire command
#[derive(clap::Args, Debug)]
pub struct jjrrt_RetireArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Override size limit for commit guard (bytes)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Run the retire command
pub fn jjrrt_run_retire(args: jjrrt_RetireArgs) -> (i32, String) {
    let cn = JJRRT_CMD_NAME_ARCHIVE;
    let mut output = vvco_Output::buffer();

    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Load gallops
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Get steeplechase entries (filtered by firemark identity)
    let rein_args = ReinArgs {
        firemark: args.firemark.clone(),
        limit: 1000,
    };
    let steeplechase = match get_entries(&rein_args) {
        Ok(entries) => entries,
        Err(e) => {
            vvco_err!(output, "{}: warning: could not get steeplechase: {}", cn, e);
            Vec::new()
        }
    };

    // Compute base path from gallops file
    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    // Fail-fast guard: reject if gallops file has uncommitted changes
    let gallops_rel = args.file.to_string_lossy().to_string();
    let status_output = match vvc::vvce_git_command(&["status", "--porcelain", "--", &gallops_rel])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            vvco_err!(output, "{}: error: failed to run git status: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    if !status_output.status.success() {
        vvco_err!(output, "{}: error: git status failed", cn);
        return (1, output.vvco_finish());
    }
    let status_text = String::from_utf8_lossy(&status_output.stdout);
    if !status_text.trim().is_empty() {
        vvco_err!(output, "{}: error: gallops file has uncommitted changes — commit or discard first", cn);
        return (1, output.vvco_finish());
    }

    let mut gallops = gallops;

    // Acquire lock FIRST
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Execute retire
    let retire_args = LibRetireArgs {
        firemark: args.firemark.clone(),
        today: jjrc_timestamp_date(),
    };

    let result = match gallops.jjrg_retire(retire_args, base_path, &steeplechase) {
        Ok(r) => r,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&args.file) {
        vvco_err!(output, "{}: error saving Gallops: {}", cn, e);
        return (1, output.vvco_finish());
    }

    // Commit using vvcm_commit with explicit file list
    // Files: gallops.json, trophy file (created), paddock file (deleted - git add handles this)
    let gallops_path = args.file.to_string_lossy().to_string();
    let effective_size_limit = args.size_limit.unwrap_or(200000);
    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            result.trophy_path.clone(),
            result.paddock_path.clone(),
        ],
        message: jjrn_format_heat_message(&firemark, jjrn_HeatAction::Retire, &result.silks),
        size_limit: effective_size_limit,
        warn_limit: effective_size_limit / 2,
    };

    match vvc::machine_commit(&lock, &commit_args, &mut output) {
        Ok(hash) => {
            vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
        }
        Err(e) => {
            vvco_err!(output, "{}: error: commit failed: {}", cn, e);
            // Rollback: restore gallops and paddock from HEAD, remove trophy
            let gp = args.file.to_string_lossy().to_string();
            let _ = vvc::vvce_git_command(&["checkout", "HEAD", "--", &gp, &result.paddock_path])
                .status();
            let _ = vvc::vvce_git_command(&["rm", "-f", "--cached", &result.trophy_path])
                .status();
            let _ = std::fs::remove_file(&result.trophy_path);
            vvco_err!(output, "{}: rolled back file changes", cn);
            vvco_err!(output, "{}: retry with --size-limit <bytes> to override guard", cn);
            return (1, output.vvco_finish());
        }
    }

    // Output result
    vvco_out!(output, "trophy: {}", result.trophy_path);
    vvco_out!(output, "Heat {} retired successfully", result.firemark);

    (0, output.vvco_finish())
    // lock released here
}
