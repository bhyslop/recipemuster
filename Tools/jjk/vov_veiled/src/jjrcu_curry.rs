// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Curry command - paddock update operation for Heat context
//!
//! Supports getter mode (display paddock) and setter mode (update with chalk entry).

use std::path::PathBuf;
use vvc::{vvco_out, vvco_err, vvco_Output};
use crate::jjrf_favor::{jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops};
use crate::jjrz_gazette::jjrz_build_read_output;

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
pub fn jjrcu_run_curry(args: jjrcu_CurryArgs, content: Option<String>) -> (i32, String) {
    use crate::jjro_ops::jjrg_curry;

    let mut output = vvco_Output::buffer();

    // Parse firemark
    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "jjx_curry: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    match content {
        None => {
            // Getter mode: display paddock content
            let gallops = match Gallops::jjrg_load(&args.file) {
                Ok(g) => g,
                Err(e) => {
                    vvco_err!(output, "jjx_curry: error loading Gallops: {}", e);
                    return (1, output.vvco_finish());
                }
            };

            let firemark_key = firemark.jjrf_display();
            let heat = match gallops.heats.get(&firemark_key) {
                Some(h) => h,
                None => {
                    vvco_err!(output, "jjx_curry: error: Heat '{}' not found", firemark_key);
                    return (1, output.vvco_finish());
                }
            };

            let paddock_path = std::path::Path::new(&heat.paddock_file);
            let paddock_content = match std::fs::read_to_string(paddock_path) {
                Ok(c) => c,
                Err(e) => {
                    vvco_err!(output, "jjx_curry: error reading paddock: {}", e);
                    return (1, output.vvco_finish());
                }
            };

            vvco_out!(output, "{}", paddock_content);

            // Gazette output for structured downstream consumption
            let gazette_md = jjrz_build_read_output(&firemark_key, &paddock_content, &[]);
            vvco_out!(output, "");
            for line in gazette_md.lines() {
                vvco_out!(output, "{}", line);
            }

            (0, output.vvco_finish())
        }
        Some(new_content) => {
            // Setter mode: update paddock
            // Call operation (curry acquires its own lock)
            match jjrg_curry(&args.file, &firemark, &new_content, args.note.as_deref(), &mut output) {
                Ok(()) => {
                    vvco_out!(output, "jjx_curry: paddock updated");
                    (0, output.vvco_finish())
                }
                Err(e) => {
                    vvco_err!(output, "jjx_curry: error: {}", e);
                    (1, output.vvco_finish())
                }
            }
        }
    }
}
