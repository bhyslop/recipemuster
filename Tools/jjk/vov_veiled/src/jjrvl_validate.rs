// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validate command - Schema validation for Gallops JSON
//!
//! This module owns the validate command's Args struct and handler function.

use std::path::PathBuf;
use vvc::{vvco_out, vvco_err, vvco_Output};
use crate::jjrg_gallops::jjrg_Gallops as Gallops;

const JJRVL_CMD_NAME_VALIDATE: &str = "jjx_validate";

/// Arguments for jjx_validate command
#[derive(clap::Args, Debug)]
pub struct jjrvl_ValidateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,
}

/// Run the validate command - validate Gallops JSON schema
pub fn jjrvl_run_validate(args: jjrvl_ValidateArgs) -> (i32, String) {
    let cn = JJRVL_CMD_NAME_VALIDATE;
    let mut output = vvco_Output::buffer();

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    match gallops.jjrg_validate() {
        Ok(()) => {
            vvco_out!(output, "Gallops validation passed");
            (0, output.vvco_finish())
        }
        Err(errors) => {
            vvco_err!(output, "{}: validation failed with {} error(s):", cn, errors.len());
            for error in errors {
                vvco_err!(output, "  - {}", error);
            }
            (1, output.vvco_finish())
        }
    }
}
