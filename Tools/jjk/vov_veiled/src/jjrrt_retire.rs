// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! jjx_retire command - Extract complete Heat data for archival trophy
//!
//! Handles retire operation: write trophy, remove from gallops, delete paddock, commit.
//! Fails fast if gallops file has uncommitted changes.

use std::fmt::Write;
use std::path::{Path, PathBuf};

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
    let mut buf = String::new();

    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            jjbuf!(buf, "jjx_retire: error: {}", e);
            return (1, buf);
        }
    };

    // Load gallops
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            jjbuf!(buf, "jjx_retire: error loading Gallops: {}", e);
            return (1, buf);
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
            jjbuf!(buf, "jjx_retire: warning: could not get steeplechase: {}", e);
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
            jjbuf!(buf, "jjx_retire: error: failed to run git status: {}", e);
            return (1, buf);
        }
    };
    if !status_output.status.success() {
        jjbuf!(buf, "jjx_retire: error: git status failed");
        return (1, buf);
    }
    let status_text = String::from_utf8_lossy(&status_output.stdout);
    if !status_text.trim().is_empty() {
        jjbuf!(buf, "jjx_retire: error: gallops file has uncommitted changes — commit or discard first");
        return (1, buf);
    }

    let mut gallops = gallops;

    // Acquire lock FIRST
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            jjbuf!(buf, "jjx_retire: error: {}", e);
            return (1, buf);
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
            jjbuf!(buf, "jjx_retire: error: {}", e);
            return (1, buf);
        }
    };

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&args.file) {
        jjbuf!(buf, "jjx_retire: error saving Gallops: {}", e);
        return (1, buf);
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

    match vvc::machine_commit(&lock, &commit_args) {
        Ok(hash) => {
            jjbuf!(buf, "jjx_retire: committed {}", &hash[..8]);
        }
        Err(e) => {
            jjbuf!(buf, "jjx_retire: error: commit failed: {}", e);
            // Rollback: restore gallops and paddock from HEAD, remove trophy
            let gp = args.file.to_string_lossy().to_string();
            let _ = vvc::vvce_git_command(&["checkout", "HEAD", "--", &gp, &result.paddock_path])
                .status();
            let _ = vvc::vvce_git_command(&["rm", "-f", "--cached", &result.trophy_path])
                .status();
            let _ = std::fs::remove_file(&result.trophy_path);
            jjbuf!(buf, "jjx_retire: rolled back file changes");
            jjbuf!(buf, "jjx_retire: retry with --size-limit <bytes> to override guard");
            return (1, buf);
        }
    }

    // Output result
    let _ = writeln!(buf, "trophy: {}", result.trophy_path);
    let _ = writeln!(buf, "Heat {} retired successfully", result.firemark);

    (0, buf)
    // lock released here
}
