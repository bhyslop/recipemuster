// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! GetCoronets command - List Coronets for a Heat
//!
//! Queries a Gallops JSON file and outputs coronets (pace identities) for a specified heat,
//! with optional filtering for remaining or rough paces only.

use vvc::{vvco_out, vvco_err, vvco_Output};
use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState};
use std::path::PathBuf;

/// Arguments for jjx_coronets command
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
pub fn jjrgc_run_get_coronets(args: jjrgc_GetCoronetsArgs) -> (i32, String) {
    let mut output = vvco_Output::buffer();
    let firemark = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "jjx_coronets: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "jjx_coronets: error: {}", e);
            return (1, output.vvco_finish());
        }
    };

    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            vvco_err!(output, "jjx_coronets: error: Heat '{}' not found", heat_key);
            return (1, output.vvco_finish());
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
                vvco_out!(output, "{}", coronet_key);
            }
        }
    }

    (0, output.vvco_finish())
}
