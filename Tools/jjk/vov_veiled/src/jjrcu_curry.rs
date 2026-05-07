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
use crate::jjri_io::jjri_paddock_path;
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug};

const JJRCU_CMD_NAME_CURRY: &str = "jjx_curry";

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

    /// Override commit size guard limit in bytes (setter mode only)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Handler for jjx_curry command
pub fn jjrcu_run_curry(args: jjrcu_CurryArgs, content: Option<String>, gazette: &mut jjrz_Gazette) -> (i32, String) {
    let cn = JJRCU_CMD_NAME_CURRY;
    use crate::jjro_ops::jjrg_curry;

    let mut output = vvco_Output::buffer();

    // Parse firemark
    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    match content {
        None => {
            // Getter mode: display paddock content
            let gallops = match Gallops::jjrg_load(&args.file) {
                Ok(g) => g,
                Err(e) => {
                    vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
                    return (1, output.vvco_finish());
                }
            };

            let firemark_key = firemark.jjrf_display();
            if !gallops.heats.contains_key(&firemark_key) {
                vvco_err!(output, "{}: error: Heat '{}' not found", cn, firemark_key);
                return (1, output.vvco_finish());
            }

            let paddock_path_string = jjri_paddock_path(firemark.jjrf_as_str());
            let paddock_path = std::path::Path::new(&paddock_path_string);
            let paddock_content = match std::fs::read_to_string(paddock_path) {
                Ok(c) => c,
                Err(e) => {
                    vvco_err!(output, "{}: error reading paddock: {}", cn, e);
                    return (1, output.vvco_finish());
                }
            };

            // Add gazette notice for downstream consumption
            gazette.jjrz_add(jjrz_Slug::Paddock, &firemark_key, &paddock_content).ok();

            (0, output.vvco_finish())
        }
        Some(new_content) => {
            // Setter mode: update paddock
            // Call operation (curry acquires its own lock)
            match jjrg_curry(&args.file, &firemark, &new_content, args.note.as_deref(), args.size_limit, &mut output) {
                Ok(()) => {
                    vvco_out!(output, "{}: paddock updated", cn);
                    (0, output.vvco_finish())
                }
                Err(e) => {
                    vvco_err!(output, "{}: error: {}", cn, e);
                    (1, output.vvco_finish())
                }
            }
        }
    }
}
