// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Get spec command - Extract spec text for a pace
//!
//! Implements the `jjx_get_spec` command which retrieves and outputs the raw
//! specification text for a single pace.

use std::path::PathBuf;
use clap::Args;

use vvc::{vvco_out, vvco_err, vvco_Output};

use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::jjrg_Gallops as Gallops;

const JJRGS_CMD_NAME_GET_SPEC: &str = "jjx_get_spec";

/// Arguments for jjx_get_spec command
#[derive(Args, Debug)]
pub struct jjrgs_GetSpecArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the get_spec command - output raw spec text
pub fn jjrgs_run_get_spec(args: jjrgs_GetSpecArgs) -> (i32, String) {
    let cn = JJRGS_CMD_NAME_GET_SPEC;
    let mut output = vvco_Output::buffer();

    let coronet = match Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Extract parent firemark and locate heat
    let firemark = coronet.jjrf_parent_firemark();
    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            vvco_err!(output, "{}: error: Heat '{}' not found", cn, heat_key);
            return (1, output.vvco_finish());
        }
    };

    // Locate pace
    let coronet_key = coronet.jjrf_display();
    let pace = match heat.paces.get(&coronet_key) {
        Some(p) => p,
        None => {
            vvco_err!(output, "{}: error: Pace '{}' not found in Heat '{}'", cn, coronet_key, heat_key);
            return (1, output.vvco_finish());
        }
    };

    // Output tacks[0].text (current spec)
    if let Some(tack) = pace.tacks.first() {
        vvco_out!(output, "{}", tack.text);
        (0, output.vvco_finish())
    } else {
        vvco_err!(output, "{}: error: Pace '{}' has no tacks", cn, coronet_key);
        (1, output.vvco_finish())
    }
}
