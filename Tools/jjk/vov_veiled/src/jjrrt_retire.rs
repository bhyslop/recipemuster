// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! jjx_retire command - Extract complete Heat data for archival trophy
//!
//! Handles retire operation: optionally dry-run (preview trophy) or execute
//! (write trophy, remove from gallops, delete paddock, commit).

use clap::Parser;
use std::path::PathBuf;

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

    /// Execute the retire (write trophy, remove from gallops, delete paddock, commit)
    #[arg(long)]
    pub execute: bool,
}

/// Run the retire command
pub fn jjrrt_run_retire(args: jjrrt_RetireArgs) -> i32 {
    use std::path::Path;

    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    // Load gallops
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_retire: error loading Gallops: {}", e);
            return 1;
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
            eprintln!("jjx_retire: warning: could not get steeplechase: {}", e);
            Vec::new()
        }
    };

    // Compute base path from gallops file
    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    // If --execute not specified, output trophy markdown preview
    if !args.execute {
        // Read paddock content
        let firemark_key = firemark.jjrf_display();
        let heat = match gallops.heats.get(&firemark_key) {
            Some(h) => h,
            None => {
                eprintln!("jjx_retire: error: Heat '{}' not found", firemark_key);
                return 1;
            }
        };
        let paddock_path = base_path.join(&heat.paddock_file);
        let paddock_content = match std::fs::read_to_string(&paddock_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_retire: error reading paddock: {}", e);
                return 1;
            }
        };

        // Build and output trophy preview
        let today = jjrc_timestamp_date();
        match gallops.jjrg_build_trophy_preview(&args.firemark, &paddock_content, &today, &steeplechase) {
            Ok(markdown) => {
                println!("{}", markdown);
                return 0;
            }
            Err(e) => {
                eprintln!("jjx_retire: error: {}", e);
                return 1;
            }
        }
    }

    // --execute: perform the actual retire operation

    // Need mutable gallops for execute path
    let mut gallops = gallops;

    // Acquire lock FIRST
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
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
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&args.file) {
        eprintln!("jjx_retire: error saving Gallops: {}", e);
        return 1;
    }

    // Commit using vvcm_commit with explicit file list
    // Files: gallops.json, trophy file (created), paddock file (deleted - git add handles this)
    let gallops_path = args.file.to_string_lossy().to_string();
    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            result.trophy_path.clone(),
            result.paddock_path.clone(),
        ],
        message: jjrn_format_heat_message(&firemark, jjrn_HeatAction::Retire, &result.silks),
        size_limit: 200000,  // 200KB - trophy files can be large
        warn_limit: 100000,
    };

    match vvc::machine_commit(&lock, &commit_args) {
        Ok(hash) => {
            eprintln!("jjx_retire: committed {}", &hash[..8]);
        }
        Err(e) => {
            eprintln!("jjx_retire: commit warning: {}", e);
        }
    }

    // Output result
    println!("trophy: {}", result.trophy_path);
    println!("Heat {} retired successfully", result.firemark);

    0
    // lock released here
}
