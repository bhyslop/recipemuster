// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validate command - Schema validation for Gallops JSON
//!
//! This module owns the validate command's Args struct and handler function.

use std::fmt::Write;
use std::path::PathBuf;
use crate::jjrg_gallops::jjrg_Gallops as Gallops;

/// Arguments for jjx_validate command
#[derive(clap::Args, Debug)]
pub struct jjrvl_ValidateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,
}

/// Run the validate command - validate Gallops JSON schema
pub fn jjrvl_run_validate(args: jjrvl_ValidateArgs) -> (i32, String) {
    let mut buf = String::new();

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_validate: error loading Gallops: {}", e);
            return (1, buf);
        }
    };

    match gallops.jjrg_validate() {
        Ok(()) => {
            let _ = writeln!(buf, "Gallops validation passed");
            (0, buf)
        }
        Err(errors) => {
            eprintln!("jjx_validate: validation failed with {} error(s):", errors.len());
            for error in errors {
                eprintln!("  - {}", error);
            }
            (1, buf)
        }
    }
}
