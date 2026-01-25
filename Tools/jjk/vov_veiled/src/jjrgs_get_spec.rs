// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Get spec command - Extract spec text for a pace
//!
//! Implements the `jjx_get_spec` command which retrieves and outputs the raw
//! specification text for a single pace.

use std::path::PathBuf;
use clap::Args;

use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::jjrg_Gallops as Gallops;

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
pub fn jjrgs_run_get_spec(args: jjrgs_GetSpecArgs) -> i32 {
    let coronet = match Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("jjx_get_spec: error: {}", e);
            return 1;
        }
    };

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_get_spec: error: {}", e);
            return 1;
        }
    };

    // Extract parent firemark and locate heat
    let firemark = coronet.jjrf_parent_firemark();
    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_get_spec: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Locate pace
    let coronet_key = coronet.jjrf_display();
    let pace = match heat.paces.get(&coronet_key) {
        Some(p) => p,
        None => {
            eprintln!("jjx_get_spec: error: Pace '{}' not found in Heat '{}'", coronet_key, heat_key);
            return 1;
        }
    };

    // Output tacks[0].text (current spec)
    if let Some(tack) = pace.tacks.first() {
        print!("{}", tack.text);
        0
    } else {
        eprintln!("jjx_get_spec: error: Pace '{}' has no tacks", coronet_key);
        1
    }
}
