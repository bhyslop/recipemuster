// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Muster command - List all Heats with summary information
//!
//! Implements the muster query operation which displays all heats
//! with their status and pace completion counts.

use std::path::PathBuf;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};

/// Arguments for muster command
#[derive(clap::Args, Debug)]
pub struct jjrmu_MusterArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Filter by heat status (racing, stabled)
    #[arg(long)]
    pub status: Option<String>,
}

/// Run the muster command - list Heats as TSV
pub async fn jjrmu_run_muster(args: jjrmu_MusterArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_muster: error: {}", e);
            return 1;
        }
    };

    // Collect heats with status info for sorting
    let mut heats_by_status: Vec<(&String, &crate::jjrg_gallops::jjrg_Heat)> = gallops.heats.iter().collect();

    // Apply status filter if specified
    if let Some(ref filter_status) = args.status {
        let filter_lowercase = filter_status.to_lowercase();
        heats_by_status.retain(|(_, heat)| {
            let heat_status_str = match heat.status {
                HeatStatus::Racing => "racing",
                HeatStatus::Stabled => "stabled",
                HeatStatus::Retired => "retired",
            };
            heat_status_str == filter_lowercase
        });
    }

    // Sort: racing first, then stabled, then retired
    heats_by_status.sort_by(|a, b| {
        let a_status_order = match a.1.status {
            HeatStatus::Racing => 0,
            HeatStatus::Stabled => 1,
            HeatStatus::Retired => 2,
        };
        let b_status_order = match b.1.status {
            HeatStatus::Racing => 0,
            HeatStatus::Stabled => 1,
            HeatStatus::Retired => 2,
        };
        a_status_order.cmp(&b_status_order)
    });

    // Set up table with column definitions
    let mut table = jjrp_Table::jjrp_new(vec![
        jjrp_Column::new("₣Fire", jjrp_Align::Left),
        jjrp_Column::new("Silks", jjrp_Align::Left),
        jjrp_Column::new("Status", jjrp_Align::Left),
        jjrp_Column::new("Done", jjrp_Align::Right),
        jjrp_Column::new("Total", jjrp_Align::Right),
    ]);

    // Measure all rows to compute column widths
    for (key, heat) in &heats_by_status {
        // Count paces where state != Abandoned
        let defined_count = heat.paces.values().filter(|pace| {
            if let Some(tack) = pace.tacks.first() {
                tack.state != PaceState::Abandoned
            } else {
                true
            }
        }).count();

        // Count paces where state == Complete
        let completed_count = heat.paces.values().filter(|pace| {
            if let Some(tack) = pace.tacks.first() {
                tack.state == PaceState::Complete
            } else {
                false
            }
        }).count();

        let status_str = match heat.status {
            HeatStatus::Racing => "racing",
            HeatStatus::Stabled => "stabled",
            HeatStatus::Retired => "retired",
        };

        table.jjrp_measure(&[
            key,
            &heat.silks,
            status_str,
            &completed_count.to_string(),
            &defined_count.to_string(),
        ]);
    }

    // Print header and separator
    table.jjrp_print_header();
    table.jjrp_print_separator();

    // Print data rows
    for (key, heat) in heats_by_status {
        // Count paces where state != Abandoned
        let defined_count = heat.paces.values().filter(|pace| {
            if let Some(tack) = pace.tacks.first() {
                tack.state != PaceState::Abandoned
            } else {
                true
            }
        }).count();

        // Count paces where state == Complete
        let completed_count = heat.paces.values().filter(|pace| {
            if let Some(tack) = pace.tacks.first() {
                tack.state == PaceState::Complete
            } else {
                false
            }
        }).count();

        let status_str = match heat.status {
            HeatStatus::Racing => "racing",
            HeatStatus::Stabled => "stabled",
            HeatStatus::Retired => "retired",
        };

        table.jjrp_print_row(&[
            key,
            &heat.silks,
            status_str,
            &completed_count.to_string(),
            &defined_count.to_string(),
        ]);
    }

    // Call invitatory to check/create officium marker
    if let Err(e) = vvc::vvcp_invitatory().await {
        eprintln!("jjx_muster: warning: invitatory failed: {}", e);
    }

    0
}
