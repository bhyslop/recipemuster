// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Curry command - paddock update operation for Heat context
//!
//! Supports getter mode (display paddock) and setter mode (update with chalk entry).
//! Setter mode requires a verb flag (--refine, --level, --muck) to indicate update type.

use std::path::PathBuf;
use clap::Parser;
use crate::jjrf_favor::{jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops};

/// Curry verb for paddock update mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrcu_CurryVerb {
    Refine,
    Level,
    Muck,
}

impl jjrcu_CurryVerb {
    pub fn as_str(&self) -> &'static str {
        match self {
            jjrcu_CurryVerb::Refine => "refine",
            jjrcu_CurryVerb::Level => "level",
            jjrcu_CurryVerb::Muck => "muck",
        }
    }
}

/// Arguments for jjx_curry command
#[derive(clap::Args, Debug)]
pub struct jjrcu_CurryArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Verb for setter mode (required if stdin present)
    #[arg(long)]
    pub refine: bool,

    #[arg(long)]
    pub level: bool,

    #[arg(long)]
    pub muck: bool,

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
            // Setter mode: require verb
            let verb_count = [args.refine, args.level, args.muck]
                .iter()
                .filter(|&&x| x)
                .count();

            if verb_count == 0 {
                eprintln!("jjx_curry: error: setter mode requires exactly one verb flag (--refine, --level, or --muck)");
                return 1;
            }
            if verb_count > 1 {
                eprintln!("jjx_curry: error: only one verb flag allowed");
                return 1;
            }

            let verb = if args.refine {
                jjrcu_CurryVerb::Refine
            } else if args.level {
                jjrcu_CurryVerb::Level
            } else {
                jjrcu_CurryVerb::Muck
            };

            // Call operation (curry acquires its own lock)
            match jjrg_curry(&args.file, &firemark, &new_content, verb.as_str(), args.note.as_deref()) {
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
