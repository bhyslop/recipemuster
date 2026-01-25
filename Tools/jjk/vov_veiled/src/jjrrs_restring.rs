// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Restring command - bulk draft multiple paces between heats
//!
//! Handles the jjx_restring command which moves multiple paces from one heat
//! to another in a single atomic operation.

use clap::Parser;
use std::path::PathBuf;

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_RestringArgs as LibRestringArgs, jjrg_PaceState};
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

/// Arguments for jjx_restring command
#[derive(clap::Args, Debug)]
pub struct jjrrs_RestringArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Source Heat identity (Firemark)
    pub firemark: String,

    /// Destination Heat identity (Firemark)
    #[arg(long)]
    pub to: String,
}

/// Execute restring command - bulk draft multiple paces atomically
pub fn jjrrs_run(args: jjrrs_RestringArgs) -> i32 {
    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_restring: error: {}", e);
            return 1;
        }
    };

    // Read coronets from stdin as JSON array
    let coronets_json = match crate::jjrg_gallops::jjrg_read_stdin() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("jjx_restring: error reading stdin: {}", e);
            return 1;
        }
    };

    let coronets: Vec<String> = match serde_json::from_str(&coronets_json) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("jjx_restring: error: Expected JSON array of coronets: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_restring: error loading Gallops: {}", e);
            return 1;
        }
    };

    let restring_args = LibRestringArgs {
        source_firemark: args.firemark.clone(),
        dest_firemark: args.to.clone(),
        coronets,
    };

    // Execute restring operation
    let result = match gallops.jjrg_restring(restring_args) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("jjx_restring: error: {}", e);
            return 1;
        }
    };

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&args.file) {
        eprintln!("jjx_restring: error saving Gallops: {}", e);
        return 1;
    }

    // Parse both firemarks for commit file list
    let dest_fm = Firemark::jjrf_parse(&result.dest_firemark).expect("restring returned invalid dest firemark");

    let gallops_path = args.file.to_string_lossy().to_string();
    let source_paddock_path = result.source_paddock.clone();
    let dest_paddock_path = result.dest_paddock.clone();

    // Build commit message using heat-level action
    let commit_message = format_heat_message(
        &dest_fm,
        HeatAction::Draft,  // Reusing Draft action since this is a bulk draft operation
        &format!("restring {} paces from {}", result.drafted.len(), result.source_firemark)
    );

    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            source_paddock_path,
            dest_paddock_path,
        ],
        message: commit_message,
        size_limit: 100000,  // 100KB - bulk operations can be large
        warn_limit: 50000,
    };

    match vvc::machine_commit(&lock, &commit_args) {
        Ok(hash) => {
            eprintln!("jjx_restring: committed {}", &hash[..8]);
        }
        Err(e) => {
            eprintln!("jjx_restring: commit warning: {}", e);
        }
    }

    // Output JSON result
    let drafted_json: Vec<_> = result.drafted.iter().map(|m| {
        serde_json::json!({
            "old_coronet": m.old_coronet,
            "new_coronet": m.new_coronet,
            "silks": m.silks,
            "state": match m.state {
                jjrg_PaceState::Rough => "rough",
                jjrg_PaceState::Bridled => "bridled",
                jjrg_PaceState::Complete => "complete",
                jjrg_PaceState::Abandoned => "abandoned",
            },
            "spec": m.spec,
        })
    }).collect();

    let output = serde_json::json!({
        "source": {
            "firemark": result.source_firemark,
            "silks": result.source_silks,
            "paddock": result.source_paddock,
            "empty_after": result.source_empty_after,
        },
        "destination": {
            "firemark": result.dest_firemark,
            "silks": result.dest_silks,
            "paddock": result.dest_paddock,
        },
        "drafted": drafted_json,
    });

    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{}", json);
            0
        }
        Err(e) => {
            eprintln!("jjx_restring: error serializing output: {}", e);
            1
        }
    }
    // lock released here
}
