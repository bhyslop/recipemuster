// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Curry command - paddock update operation for Heat context
//!
//! Supports getter mode (display paddock) and setter mode (update with chalk entry).

use std::path::PathBuf;
use crate::jjrf_favor::{jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops};

/// Arguments for jjx_curry command
#[derive(clap::Args, Debug)]
pub struct jjrcu_CurryArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Optional note for chalk entry
    #[arg(long)]
    pub note: Option<String>,
}

/// Handler for jjx_curry command
pub fn jjrcu_run_curry(args: jjrcu_CurryArgs) -> i32 {
    use crate::jjro_ops::jjrg_curry;
    use crate::jjrg_gallops::jjrg_read_stdin_optional;

    // Parse firemark
    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_curry: error: {}", e);
            return 1;
        }
    };

    // Check stdin
    let stdin_content = match jjrg_read_stdin_optional() {
        Ok(opt) => opt,
        Err(e) => {
            eprintln!("jjx_curry: error: {}", e);
            return 1;
        }
    };

    match stdin_content {
        None => {
            // Getter mode: display paddock content
            let gallops = match Gallops::jjrg_load(&args.file) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("jjx_curry: error loading Gallops: {}", e);
                    return 1;
                }
            };

            let firemark_key = firemark.jjrf_display();
            let heat = match gallops.heats.get(&firemark_key) {
                Some(h) => h,
                None => {
                    eprintln!("jjx_curry: error: Heat '{}' not found", firemark_key);
                    return 1;
                }
            };

            let paddock_path = std::path::Path::new(&heat.paddock_file);
            let content = match std::fs::read_to_string(paddock_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("jjx_curry: error reading paddock: {}", e);
                    return 1;
                }
            };

            print!("{}", content);
            0
        }
        Some(new_content) => {
            // Setter mode: update paddock
            // Call operation (curry acquires its own lock)
            match jjrg_curry(&args.file, &firemark, &new_content, args.note.as_deref()) {
                Ok(()) => {
                    eprintln!("jjx_curry: paddock updated");
                    0
                }
                Err(e) => {
                    eprintln!("jjx_curry: error: {}", e);
                    1
                }
            }
        }
    }
}
