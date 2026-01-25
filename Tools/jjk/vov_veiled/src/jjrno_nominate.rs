// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Nominate command - create a new Heat
//!
//! This module provides the Args struct and handler for the jjx_nominate command.

use std::path::PathBuf;
use indexmap::IndexMap;

use crate::jjrf_favor::{jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_NominateArgs as LibNominateArgs};
use crate::jjrc_core::jjrc_timestamp_from_env;
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

/// Arguments for jjx_nominate command
#[derive(clap::Args, Debug)]
pub struct jjrx_NominateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Kebab-case display name for the Heat
    #[arg(long, short = 's')]
    pub silks: String,
}

/// Handler for jjx_nominate command
pub fn jjrx_run_nominate(args: jjrx_NominateArgs) -> i32 {
    use std::path::Path;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_nominate: error: {}", e);
            return 1;
        }
    };

    let mut gallops = if args.file.exists() {
        match Gallops::jjrg_load(&args.file) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("jjx_nominate: error loading Gallops: {}", e);
                return 1;
            }
        }
    } else {
        if let Some(parent) = args.file.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("jjx_nominate: error creating directory: {}", e);
                return 1;
            }
        }
        Gallops {
            next_heat_seed: "AA".to_string(),
            heats: IndexMap::new(),
        }
    };

    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let silks = args.silks.clone();
    let nominate_args = LibNominateArgs {
        silks: args.silks,
        created: jjrc_timestamp_from_env(),
    };

    match gallops.jjrg_nominate(nominate_args, base_path) {
        Ok(result) => {
            let fm = Firemark::jjrf_parse(&result.firemark).expect("nominate returned invalid firemark");
            let message = format_heat_message(&fm, HeatAction::Nominate, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    eprintln!("jjx_nominate: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_nominate: error: {}", e);
                    return 1;
                }
            }

            println!("{}", result.firemark);
            0
        }
        Err(e) => {
            eprintln!("jjx_nominate: error: {}", e);
            1
        }
    }
    // lock released here
}
