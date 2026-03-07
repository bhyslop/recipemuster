// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Saddle command - Return context needed to saddle up on a Heat

use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrs_steeplechase::{jjrs_get_entries, jjrs_ReinArgs};
use crate::jjrq_query::jjrq_resolve_default_heat;
use crate::jjrpd_parade::{jjrpd_write_file_bitmap, jjrpd_write_commit_swimlanes};

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
pub async fn jjrsd_run_saddle(args: jjrsd_SaddleArgs) -> (i32, String) {
    let mut buf = String::new();
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            jjbuf!(buf, "jjx_orient: error: {}", e);
            return (1, buf);
        }
    };

    // Collect racing heats in heat_order priority sequence for display table
    {
        let racing_heats: Vec<(&String, &crate::jjrg_gallops::jjrg_Heat)> = gallops.heat_order.iter()
            .filter_map(|fm| gallops.heats.get(fm).map(|h| (fm, h)))
            .filter(|(_, heat)| heat.status == HeatStatus::Racing)
            .collect();

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
            let _ = writeln!(buf, "Racing-heats:");
            table.jjrp_write_header(&mut buf);
            table.jjrp_write_separator(&mut buf);

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

                table.jjrp_write_row(&mut buf, &[
                    key,
                    &heat.silks,
                    "racing",
                    &completed_count.to_string(),
                    &defined_count.to_string(),
                ]);
            }

            let _ = writeln!(buf);
        }
    }

    // Resolve firemark: use provided value or auto-select first racing heat
    let firemark_str = match args.firemark {
        Some(fm) => fm,
        None => {
            match jjrq_resolve_default_heat(&gallops) {
                Ok(fm) => fm,
                Err(e) => {
                    jjbuf!(buf, "jjx_orient: error: {}", e);
                    return (1, buf);
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
                jjbuf!(buf, "jjx_orient: error: {}", e);
                return (1, buf);
            }
        };
        Some(coronet)
    } else if stripped_input.len() == 2 {
        // It's a firemark - existing behavior
        None
    } else {
        jjbuf!(buf, "jjx_orient: error: Invalid argument '{}' (must be 2-char firemark or 5-char coronet)", firemark_str);
        return (1, buf);
    };

    // Extract firemark (either directly provided or from coronet parent)
    let firemark = if let Some(ref coronet) = target_coronet {
        coronet.jjrf_parent_firemark()
    } else {
        match Firemark::jjrf_parse(&firemark_str) {
            Ok(fm) => fm,
            Err(e) => {
                jjbuf!(buf, "jjx_orient: error: {}", e);
                return (1, buf);
            }
        }
    };

    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            jjbuf!(buf, "jjx_orient: error: Heat '{}' not found", heat_key);
            return (1, buf);
        }
    };

    // Check if heat is stabled (cannot saddle stabled heat)
    if heat.status == HeatStatus::Stabled {
        jjbuf!(buf, "jjx_orient: error: Cannot saddle stabled heat '{}'", heat_key);
        return (1, buf);
    }

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            jjbuf!(buf, "jjx_orient: error reading paddock file '{}': {}", heat.paddock_file, e);
            return (1, buf);
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
                            jjbuf!(buf, "jjx_orient: error: Pace '{}' is already complete", coronet_key);
                            return (1, buf);
                        }
                        PaceState::Abandoned => {
                            jjbuf!(buf, "jjx_orient: error: Pace '{}' is abandoned", coronet_key);
                            return (1, buf);
                        }
                        _ => {
                            jjbuf!(buf, "jjx_orient: error: Pace '{}' has invalid state", coronet_key);
                            return (1, buf);
                        }
                    }
                } else {
                    jjbuf!(buf, "jjx_orient: error: Pace '{}' has no tacks", coronet_key);
                    return (1, buf);
                }
            }
            None => {
                jjbuf!(buf, "jjx_orient: error: Pace '{}' not found in heat '{}'", coronet_key, heat_key);
                return (1, buf);
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
    let _ = writeln!(buf, "Heat: {} ({}) [{}]", heat.silks, heat_key, status_str);
    let _ = writeln!(buf, "Paddock: {}", heat.paddock_file);
    let _ = writeln!(buf);
    let _ = writeln!(buf, "Paddock-content:");
    for line in paddock_content.lines() {
        let _ = writeln!(buf, "  {}", line);
    }
    let _ = writeln!(buf);

    if let Some(coronet) = pace_coronet {
        if let Some(silks) = pace_silks {
            if let Some(state) = pace_state {
                let _ = writeln!(buf, "Next: {} ({}) [{}]", silks, coronet, state);
                let _ = writeln!(buf);
                if let Some(spec_text) = spec {
                    let _ = writeln!(buf, "Docket:");
                    for line in spec_text.lines() {
                        let _ = writeln!(buf, "  {}", line);
                    }
                    let _ = writeln!(buf);
                }
                if let Some(dir_text) = direction {
                    let _ = writeln!(buf, "Warrant:");
                    for line in dir_text.lines() {
                        let _ = writeln!(buf, "  {}", line);
                    }
                    let _ = writeln!(buf);
                }
                jjbuf!(buf, "");
                if state == "bridled" {
                    jjbuf!(buf, "Recommended: /jjc-heat-mount {} to execute", firemark.jjrf_as_str());
                } else {
                    jjbuf!(buf, "Recommended: /jjc-pace-bridle {} or /jjc-heat-mount {}", coronet, firemark.jjrf_as_str());
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
        let _ = writeln!(buf, "Recent-work:");

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
        table.jjrp_write_header(&mut buf);
        table.jjrp_write_separator(&mut buf);

        // Print data rows
        for entry in &filtered_work {
            let identity_str = if let Some(ref coronet) = entry.coronet {
                coronet.clone()
            } else {
                heat_key.clone()
            };
            table.jjrp_write_row(&mut buf, &[
                &entry.commit,
                &identity_str,
                &entry.subject,
            ]);
        }
    }

    // Always show file-touch bitmap and commit swim lanes after recent work
    jjrpd_write_file_bitmap(&mut buf, &firemark, heat);
    jjrpd_write_commit_swimlanes(&mut buf, &firemark, heat);

    if let Err(e) = vvc::vvcp_invitatory().await {
        jjbuf!(buf, "jjx_orient: warning: invitatory failed: {}", e);
    }

    (0, buf)
}
