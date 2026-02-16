// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Saddle command - Return context needed to saddle up on a Heat

use std::fs;
use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrs_steeplechase::{jjrs_get_entries, jjrs_ReinArgs};
use crate::jjrq_query::jjrq_resolve_default_heat;
use crate::jjrpd_parade::{jjrpd_print_file_bitmap, jjrpd_print_commit_swimlanes};

/// Arguments for saddle command
#[derive(clap::Args, Debug)]
pub struct jjrsd_SaddleArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark) or Pace identity (Coronet). If omitted, uses first racing heat.
    pub firemark: Option<String>,
}

/// Run the saddle command - return Heat context
pub async fn jjrsd_run_saddle(args: jjrsd_SaddleArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_orient: error: {}", e);
            return 1;
        }
    };

    // Collect racing heats and display summary table
    {
        let mut racing_heats: Vec<(&String, &crate::jjrg_gallops::jjrg_Heat)> = gallops.heats.iter()
            .filter(|(_, heat)| heat.status == HeatStatus::Racing)
            .collect();

        // Sort by key for stability
        racing_heats.sort_by(|a, b| a.0.cmp(b.0));

        if !racing_heats.is_empty() {
            let mut table = jjrp_Table::jjrp_new(vec![
                jjrp_Column::new("₣Fire", jjrp_Align::Left),
                jjrp_Column::new("Silks", jjrp_Align::Left),
                jjrp_Column::new("Status", jjrp_Align::Left),
                jjrp_Column::new("Done", jjrp_Align::Right),
                jjrp_Column::new("Total", jjrp_Align::Right),
            ]);

            // Measure all rows to compute column widths
            for (key, heat) in &racing_heats {
                let defined_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state != PaceState::Abandoned
                    } else {
                        true
                    }
                }).count();

                let completed_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state == PaceState::Complete
                    } else {
                        false
                    }
                }).count();

                table.jjrp_measure(&[
                    key,
                    &heat.silks,
                    "racing",
                    &completed_count.to_string(),
                    &defined_count.to_string(),
                ]);
            }

            // Print header and separator
            println!("Racing-heats:");
            table.jjrp_print_header();
            table.jjrp_print_separator();

            // Print data rows
            for (key, heat) in &racing_heats {
                let defined_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state != PaceState::Abandoned
                    } else {
                        true
                    }
                }).count();

                let completed_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state == PaceState::Complete
                    } else {
                        false
                    }
                }).count();

                table.jjrp_print_row(&[
                    key,
                    &heat.silks,
                    "racing",
                    &completed_count.to_string(),
                    &defined_count.to_string(),
                ]);
            }

            println!();
        }
    }

    // Resolve firemark: use provided value or auto-select first racing heat
    let firemark_str = match args.firemark {
        Some(fm) => fm,
        None => {
            match jjrq_resolve_default_heat(&gallops) {
                Ok(fm) => fm,
                Err(e) => {
                    eprintln!("jjx_orient: error: {}", e);
                    return 1;
                }
            }
        }
    };

    // Detect if input is a coronet (5 chars) or firemark (2 chars)
    // Strip any prefix first: ₣ (U+20A3) or ₢ (U+20A2)
    let stripped_input = firemark_str
        .trim_start_matches('₣')
        .trim_start_matches('₢');

    let target_coronet = if stripped_input.len() == 5 {
        // It's a coronet - parse to get parent firemark
        let coronet = match Coronet::jjrf_parse(&firemark_str) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_orient: error: {}", e);
                return 1;
            }
        };
        Some(coronet)
    } else if stripped_input.len() == 2 {
        // It's a firemark - existing behavior
        None
    } else {
        eprintln!("jjx_orient: error: Invalid argument '{}' (must be 2-char firemark or 5-char coronet)", firemark_str);
        return 1;
    };

    // Extract firemark (either directly provided or from coronet parent)
    let firemark = if let Some(ref coronet) = target_coronet {
        coronet.jjrf_parent_firemark()
    } else {
        match Firemark::jjrf_parse(&firemark_str) {
            Ok(fm) => fm,
            Err(e) => {
                eprintln!("jjx_orient: error: {}", e);
                return 1;
            }
        }
    };

    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_orient: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Check if heat is stabled (cannot saddle stabled heat)
    if heat.status == HeatStatus::Stabled {
        eprintln!("jjx_orient: error: Cannot saddle stabled heat '{}'", heat_key);
        return 1;
    }

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_orient: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Get recent steeplechase entries
    let recent_work = jjrs_get_entries(&jjrs_ReinArgs {
        firemark: firemark.jjrf_as_str().to_string(),
        limit: 10,
    }).unwrap_or_default();

    // Find pace to display
    let mut pace_coronet: Option<String> = None;
    let mut pace_silks: Option<String> = None;
    let mut pace_state: Option<String> = None;
    let mut spec: Option<String> = None;
    let mut direction: Option<String> = None;

    if let Some(ref coronet) = target_coronet {
        // Specific coronet requested - look it up directly
        let coronet_key = coronet.jjrf_display();

        match heat.paces.get(&coronet_key) {
            Some(pace) => {
                if let Some(tack) = pace.tacks.first() {
                    match tack.state {
                        PaceState::Rough | PaceState::Bridled => {
                            pace_coronet = Some(coronet_key.clone());
                            pace_silks = Some(tack.silks.clone());
                            pace_state = Some(match tack.state {
                                PaceState::Rough => "rough".to_string(),
                                PaceState::Bridled => "bridled".to_string(),
                                _ => unreachable!(),
                            });
                            spec = Some(tack.text.clone());
                            if tack.state == PaceState::Bridled {
                                direction = tack.direction.clone();
                            }
                        }
                        PaceState::Complete => {
                            eprintln!("jjx_orient: error: Pace '{}' is already complete", coronet_key);
                            return 1;
                        }
                        PaceState::Abandoned => {
                            eprintln!("jjx_orient: error: Pace '{}' is abandoned", coronet_key);
                            return 1;
                        }
                        _ => {
                            eprintln!("jjx_orient: error: Pace '{}' has invalid state", coronet_key);
                            return 1;
                        }
                    }
                } else {
                    eprintln!("jjx_orient: error: Pace '{}' has no tacks", coronet_key);
                    return 1;
                }
            }
            None => {
                eprintln!("jjx_orient: error: Pace '{}' not found in heat '{}'", coronet_key, heat_key);
                return 1;
            }
        }
    } else {
        // Find first actionable pace (rough or bridled)
        for coronet_key in &heat.order {
            if let Some(pace) = heat.paces.get(coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    match tack.state {
                        PaceState::Rough | PaceState::Bridled => {
                            pace_coronet = Some(coronet_key.clone());
                            pace_silks = Some(tack.silks.clone());
                            pace_state = Some(match tack.state {
                                PaceState::Rough => "rough".to_string(),
                                PaceState::Bridled => "bridled".to_string(),
                                _ => unreachable!(),
                            });
                            spec = Some(tack.text.clone());
                            if tack.state == PaceState::Bridled {
                                direction = tack.direction.clone();
                            }
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    // Determine heat status string
    let status_str = match heat.status {
        HeatStatus::Racing => "racing",
        HeatStatus::Stabled => "stabled",
        HeatStatus::Retired => "retired",
    };

    // Output plain text format
    println!("Heat: {} ({}) [{}]", heat.silks, heat_key, status_str);
    println!("Paddock: {}", heat.paddock_file);
    println!();
    println!("Paddock-content:");
    for line in paddock_content.lines() {
        println!("  {}", line);
    }
    println!();

    if let Some(coronet) = pace_coronet {
        if let Some(silks) = pace_silks {
            if let Some(state) = pace_state {
                println!("Next: {} ({}) [{}]", silks, coronet, state);
                println!();
                if let Some(spec_text) = spec {
                    println!("Docket:");
                    for line in spec_text.lines() {
                        println!("  {}", line);
                    }
                    println!();
                }
                if let Some(dir_text) = direction {
                    println!("Warrant:");
                    for line in dir_text.lines() {
                        println!("  {}", line);
                    }
                    println!();
                }
                eprintln!();
                if state == "bridled" {
                    eprintln!("Recommended: /jjc-heat-mount {} to execute", firemark.jjrf_as_str());
                } else {
                    eprintln!("Recommended: /jjc-pace-bridle {} or /jjc-heat-mount {}", coronet, firemark.jjrf_as_str());
                }
            }
        }
    }

    // Output recent work table
    // Filter entries to only show action codes 'n' (notch), 'A' (approach), 'd' (discussion)
    let filtered_work: Vec<_> = recent_work.iter().filter(|entry| {
        if let Some(ref action) = entry.action {
            action == "n" || action == "A" || action == "d"
        } else {
            false
        }
    }).collect();

    if !filtered_work.is_empty() {
        println!("Recent-work:");

        let mut table = jjrp_Table::jjrp_new(vec![
            jjrp_Column::new("Commit", jjrp_Align::Left),
            jjrp_Column::new("Identity", jjrp_Align::Left),
            jjrp_Column::new("Subject", jjrp_Align::Left),
        ]);

        // Measure all rows to compute column widths
        for entry in &filtered_work {
            let identity_str = if let Some(ref coronet) = entry.coronet {
                coronet.clone()
            } else {
                heat_key.clone()
            };
            table.jjrp_measure(&[
                &entry.commit,
                &identity_str,
                &entry.subject,
            ]);
        }

        // Print header and separator
        table.jjrp_print_header();
        table.jjrp_print_separator();

        // Print data rows
        for entry in &filtered_work {
            let identity_str = if let Some(ref coronet) = entry.coronet {
                coronet.clone()
            } else {
                heat_key.clone()
            };
            table.jjrp_print_row(&[
                &entry.commit,
                &identity_str,
                &entry.subject,
            ]);
        }
    }

    // Always show file-touch bitmap and commit swim lanes after recent work
    jjrpd_print_file_bitmap(&firemark, heat);
    jjrpd_print_commit_swimlanes(&firemark, heat);

    // Call invitatory to check/create officium marker
    if let Err(e) = vvc::vvcp_invitatory().await {
        eprintln!("jjx_orient: warning: invitatory failed: {}", e);
    }

    0
}
