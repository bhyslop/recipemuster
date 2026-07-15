// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! GetCoronets command - List Coronets for a Heat
//!
//! Queries a Gallops JSON file and outputs coronets (pace identities) for a specified heat,
//! with optional filtering for remaining or rough paces only.

use vvc::{vvco_out, vvco_err, vvco_Output};
use crate::jjrf_favor::jjrf_Firemark;
use crate::jjrg_gallops::{jjrg_Gallops, jjrg_PaceState};
use std::path::PathBuf;

const JJRGC_CMD_NAME_CORONETS: &str = "jjx_coronets";

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
    let cn = JJRGC_CMD_NAME_CORONETS;
    let mut output = vvco_Output::buffer();
    let firemark = match jjrf_Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let gallops = match jjrg_Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            vvco_err!(output, "{}: error: Heat '{}' not found", cn, heat_key);
            return (1, output.vvco_finish());
        }
    };

    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                // Apply --remaining filter: skip complete/abandoned
                if args.remaining && (tack.state == jjrg_PaceState::Complete || tack.state == jjrg_PaceState::Abandoned) {
                    continue;
                }
                // Apply --rough filter: only include rough
                if args.rough && tack.state != jjrg_PaceState::Rough {
                    continue;
                }
                // Tag abandoned paces so a dropped pace can't be mistaken for a
                // live one in the default (unfiltered) listing, and bridled paces
                // with state and tier so a designated pace reads as claimed work.
                // The coronet stays the first whitespace token either way, so a
                // token-wise reader still parses it.
                if tack.state == jjrg_PaceState::Abandoned || tack.state == jjrg_PaceState::Bridled {
                    vvco_out!(output, "{}  [{}]", coronet_key, tack.jjrg_state_label());
                } else {
                    vvco_out!(output, "{}", coronet_key);
                }
            }
        }
    }

    (0, output.vvco_finish())
}
