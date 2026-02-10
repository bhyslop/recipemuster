// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Slate command - add a new Pace to a Heat

use clap::Args;
use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_read_stdin as read_stdin};
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

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
}

/// Handler for jjx_slate command
pub fn jjrsl_run_slate(args: jjrsl_SlateArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_SlateArgs as LibSlateArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            return 1;
        }
    };

    let text = match read_stdin() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_slate: error loading Gallops: {}", e);
            return 1;
        }
    };

    let firemark = args.firemark.clone();
    let silks = args.silks.clone();
    let slate_args = LibSlateArgs {
        firemark: args.firemark,
        silks: args.silks,
        text,
        before: args.before,
        after: args.after,
        first: args.first,
    };

    match gallops.jjrg_slate(slate_args) {
        Ok(result) => {
            let fm = Firemark::jjrf_parse(&firemark).expect("slate given invalid firemark");
            let message = format_heat_message(&fm, HeatAction::Slate, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(_hash) => {}
                Err(e) => {
                    eprintln!("jjx_slate: error: {}", e);
                    return 1;
                }
            }

            println!("{}", result.coronet);
            0
        }
        Err(e) => {
            eprintln!("jjx_slate: error: {}", e);
            1
        }
    }
    // lock released here
}
