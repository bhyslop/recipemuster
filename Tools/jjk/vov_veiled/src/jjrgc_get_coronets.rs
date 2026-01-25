// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! GetCoronets command - List Coronets for a Heat
//!
//! Queries a Gallops JSON file and outputs coronets (pace identities) for a specified heat,
//! with optional filtering for remaining or rough paces only.

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState};
use std::path::PathBuf;

/// Arguments for jjx_get_coronets command
#[derive(clap::Args, Debug)]
pub struct jjrgc_GetCoronetsArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Exclude complete and abandoned paces
    #[arg(long)]
    pub remaining: bool,

    /// Include only rough paces
    #[arg(long)]
    pub rough: bool,
}

/// Run the get_coronets command - output coronets one per line
pub fn jjrgc_run_get_coronets(args: jjrgc_GetCoronetsArgs) -> i32 {
    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("jjx_get_coronets: error: {}", e);
            return 1;
        }
    };

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_get_coronets: error: {}", e);
            return 1;
        }
    };

    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_get_coronets: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                // Apply --remaining filter: skip complete/abandoned
                if args.remaining && (tack.state == PaceState::Complete || tack.state == PaceState::Abandoned) {
                    continue;
                }
                // Apply --rough filter: only include rough
                if args.rough && tack.state != PaceState::Rough {
                    continue;
                }
                println!("{}", coronet_key);
            }
        }
    }

    0
}
