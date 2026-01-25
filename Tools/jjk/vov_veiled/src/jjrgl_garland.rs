// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Garland command - celebrate heat completion and create continuation
//!
//! Garlanding a heat:
//! - Updates source heat silks to garlanded form and status to stabled
//! - Creates new heat with continuation silks, status racing
//! - Transfers actionable paces (rough/bridled) to new heat
//! - Retains complete/abandoned paces in garlanded heat

use std::path::PathBuf;

use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_GarlandArgs as LibGarlandArgs};
use crate::jjrf_favor::{jjrf_Firemark as Firemark};
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

/// Arguments for jjx_garland command
#[derive(clap::Args, Debug)]
pub struct jjrgl_GarlandArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Heat to garland (Firemark)
    pub firemark: String,
}

/// Run the garland command
///
/// Executes garland operation, saves gallops, and commits the result.
/// Returns exit code (0 for success, non-zero for failure).
pub fn jjrgl_run_garland(args: jjrgl_GarlandArgs) -> i32 {
    use std::path::Path;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_garland: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_garland: error loading Gallops: {}", e);
            return 1;
        }
    };

    let firemark_str = args.firemark.clone();

    // Compute base path from gallops file
    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let garland_args = LibGarlandArgs {
        firemark: firemark_str.clone(),
    };

    // Execute garland operation
    let result = match gallops.jjrg_garland(garland_args, base_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("jjx_garland: error: {}", e);
            return 1;
        }
    };

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&args.file) {
        eprintln!("jjx_garland: error saving Gallops: {}", e);
        return 1;
    }

    // Parse both firemarks for commit file list
    let old_fm = Firemark::jjrf_parse(&result.old_firemark).expect("garland returned invalid old firemark");
    let new_fm = Firemark::jjrf_parse(&result.new_firemark).expect("garland returned invalid new firemark");

    let gallops_path = args.file.to_string_lossy().to_string();
    let old_paddock_path = format!(".claude/jjm/jjp_{}.md", old_fm.jjrf_as_str());
    let new_paddock_path = format!(".claude/jjm/jjp_{}.md", new_fm.jjrf_as_str());

    // Build commit message using heat-level Garland action
    let commit_message = format_heat_message(
        &old_fm,
        HeatAction::Garland,
        &format!("{} → {}", result.old_silks, result.new_silks)
    );

    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![
            gallops_path,
            old_paddock_path,
            new_paddock_path,
        ],
        message: commit_message,
        size_limit: 100000,  // 100KB - paddock files can be large
        warn_limit: 50000,
    };

    match vvc::machine_commit(&lock, &commit_args) {
        Ok(hash) => {
            eprintln!("jjx_garland: committed {}", &hash[..8]);
        }
        Err(e) => {
            eprintln!("jjx_garland: commit warning: {}", e);
        }
    }

    // Output JSON result
    let output = serde_json::json!({
        "old_firemark": result.old_firemark,
        "old_silks": result.old_silks,
        "new_firemark": result.new_firemark,
        "new_silks": result.new_silks,
        "paces_transferred": result.paces_transferred,
        "paces_retained": result.paces_retained,
    });

    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{}", json);
            0
        }
        Err(e) => {
            eprintln!("jjx_garland: error serializing output: {}", e);
            1
        }
    }
    // lock released here
}
