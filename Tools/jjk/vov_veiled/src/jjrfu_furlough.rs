// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Furlough command - change Heat status or rename
//!
//! Supports toggling between racing/stabled status and/or renaming a Heat.

use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_FurloughArgs as LibFurloughArgs};
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

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
}

/// Handler for jjx_furlough command
pub fn jjrfu_run_furlough(args: jjrfu_FurloughArgs) -> i32 {
    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_furlough: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_furlough: error loading Gallops: {}", e);
            return 1;
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
    let furlough_args = LibFurloughArgs {
        firemark: args.firemark,
        racing: args.racing,
        stabled: args.stabled,
        silks: args.silks,
    };

    match gallops.jjrg_furlough(furlough_args) {
        Ok(()) => {
            let fm = Firemark::jjrf_parse(&firemark_str).expect("furlough given invalid firemark");
            let message = format_heat_message(&fm, HeatAction::Furlough, &description);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 100000) {
                Ok(hash) => {
                    eprintln!("jjx_furlough: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_furlough: error: {}", e);
                    return 1;
                }
            }

            0
        }
        Err(e) => {
            eprintln!("jjx_furlough: error: {}", e);
            1
        }
    }
    // lock released here
}
