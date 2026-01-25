// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Query operations for Gallops JSON
//!
//! Implements read operations: muster, saddle, parade, retire.
//! All operations read from Gallops JSON and optionally paddock files.

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrs_steeplechase::{jjrs_get_entries, jjrs_ReinArgs};
use serde::Serialize;
use std::fs;
use regex::Regex;

/// Resolve the default heat (first racing heat) when no target is specified
pub fn jjrq_resolve_default_heat(gallops: &Gallops) -> Result<String, String> {
    for (heat_key, heat) in &gallops.heats {
        if heat.status == HeatStatus::Racing {
            return Ok(heat_key.clone());
        }
    }
    Err("No racing heats found".to_string())
}

// ============================================================================
// Muster - List all Heats with summary information
// ============================================================================

/// Arguments for muster command
#[derive(Debug)]
pub struct jjrq_MusterArgs {
    pub file: std::path::PathBuf,
    pub status: Option<String>,
}

/// Run the muster command - list Heats as TSV
pub fn jjrq_run_muster(args: jjrq_MusterArgs) -> i32 {
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

    0
}

// ============================================================================
// Saddle - Return context needed to saddle up on a Heat
// ============================================================================

/// Arguments for saddle command
#[derive(Debug)]
pub struct jjrq_SaddleArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
}

/// Run the saddle command - return Heat context
pub fn jjrq_run_saddle(args: jjrq_SaddleArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_saddle: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_saddle: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Check if heat is stabled (cannot saddle stabled heat)
    if heat.status == HeatStatus::Stabled {
        eprintln!("jjx_saddle: error: Cannot saddle stabled heat '{}'", heat_key);
        return 1;
    }

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_saddle: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Get recent steeplechase entries
    let recent_work = jjrs_get_entries(&jjrs_ReinArgs {
        firemark: args.firemark.jjrf_as_str().to_string(),
        limit: 10,
    }).unwrap_or_default();

    // Find first actionable pace (rough or bridled)
    let mut pace_coronet: Option<String> = None;
    let mut pace_silks: Option<String> = None;
    let mut pace_state: Option<String> = None;
    let mut spec: Option<String> = None;
    let mut direction: Option<String> = None;

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
                    println!("Spec:");
                    for line in spec_text.lines() {
                        println!("  {}", line);
                    }
                    println!();
                }
                if let Some(dir_text) = direction {
                    println!("Direction:");
                    for line in dir_text.lines() {
                        println!("  {}", line);
                    }
                    println!();
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

    // Check if session probe is needed (gap > 1 hour or no commits)
    let last_timestamp = recent_work.first().map(|e| e.timestamp.as_str());
    if crate::jjrc_core::jjrc_needs_session_probe(last_timestamp) {
        println!();
        println!("Needs-session-probe: true");
    }

    0
}

// ============================================================================
// Parade - Display comprehensive Heat status for project review
// ============================================================================

/// Arguments for parade command
#[derive(Debug)]
pub struct jjrq_ParadeArgs {
    pub file: std::path::PathBuf,
    pub target: String,
    pub full: bool,
    pub remaining: bool,
}

/// Run the parade command - display comprehensive Heat status
pub fn jjrq_run_parade(args: jjrq_ParadeArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    // Determine target type by length
    let target_str = args.target.strip_prefix('₢').or_else(|| args.target.strip_prefix('₣')).unwrap_or(&args.target);

    if target_str.len() == 5 {
        // Coronet - pace view
        let coronet = match Coronet::jjrf_parse(&args.target) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_parade: error: {}", e);
                return 1;
            }
        };

        // Extract parent firemark
        let firemark = coronet.jjrf_parent_firemark();
        let heat_key = firemark.jjrf_display();
        let heat = match gallops.heats.get(&heat_key) {
            Some(h) => h,
            None => {
                eprintln!("jjx_parade: error: Heat '{}' not found", heat_key);
                return 1;
            }
        };

        // Find pace
        let coronet_key = coronet.jjrf_display();
        let pace = match heat.paces.get(&coronet_key) {
            Some(p) => p,
            None => {
                eprintln!("jjx_parade: error: Pace '{}' not found in Heat '{}'", coronet_key, heat_key);
                return 1;
            }
        };

        // Display tack history - iterate in reverse order (oldest first)
        if !pace.tacks.is_empty() {
            // Print header with current state from tacks[0]
            let current_tack = &pace.tacks[0];
            println!("Pace: {} ({})", current_tack.silks, coronet_key);
            println!("Heat: {}", heat_key);
            println!();

            // Iterate tacks in reverse order (oldest first)
            for (index, tack) in pace.tacks.iter().rev().enumerate() {
                let state_str = zjjrq_pace_state_str(&tack.state);
                let commit_str = if tack.commit == "0000000" {
                    "(no commit)".to_string()
                } else {
                    tack.commit.clone()
                };

                println!("[{}] {} ({})", index, state_str, commit_str);
                println!("    Silks: {}", tack.silks);
                if let Some(ref direction) = tack.direction {
                    println!("    Direction: {}", direction);
                }
                println!();
                // Indent spec text
                for line in tack.text.lines() {
                    println!("    {}", line);
                }
                println!();
            }
        }
    } else if target_str.len() == 2 {
        // Firemark - heat view
        let firemark = match Firemark::jjrf_parse(&args.target) {
            Ok(fm) => fm,
            Err(e) => {
                eprintln!("jjx_parade: error: {}", e);
                return 1;
            }
        };

        let heat_key = firemark.jjrf_display();
        let heat = match gallops.heats.get(&heat_key) {
            Some(h) => h,
            None => {
                eprintln!("jjx_parade: error: Heat '{}' not found", heat_key);
                return 1;
            }
        };

        if args.full {
            // Full view: paddock + all specs
            let paddock_content = match fs::read_to_string(&heat.paddock_file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("jjx_parade: error reading paddock file '{}': {}", heat.paddock_file, e);
                    return 1;
                }
            };

            let status_str = match heat.status {
                HeatStatus::Racing => "racing",
                HeatStatus::Stabled => "stabled",
                HeatStatus::Retired => "retired",
            };

            println!("Heat: {} ({})", heat.silks, heat_key);
            println!("Status: {}", status_str);
            println!("Created: {}", heat.creation_time);
            println!();
            println!("## Paddock");
            println!();
            println!("{}", paddock_content);
            println!();
            println!("## Paces");
            println!();

            for coronet_key in &heat.order {
                if let Some(pace) = heat.paces.get(coronet_key) {
                    if let Some(tack) = pace.tacks.first() {
                        // Skip complete/abandoned if --remaining
                        if args.remaining && (tack.state == PaceState::Complete || tack.state == PaceState::Abandoned) {
                            continue;
                        }
                        let state_str = zjjrq_pace_state_str(&tack.state);
                        println!("### {} ({}) [{}]", tack.silks, coronet_key, state_str);
                        println!();
                        println!("{}", tack.text);
                        if let Some(ref direction) = tack.direction {
                            println!();
                            println!("**Direction:** {}", direction);
                        }
                        println!();
                    }
                }
            }
        } else {
            // List view: numbered paces
            // If --remaining, output markdown format
            if args.remaining {
                let mut complete_count = 0;
                let mut abandoned_count = 0;
                let mut rough_count = 0;
                let mut bridled_count = 0;
                let mut remaining_paces: Vec<(&String, &crate::jjrg_gallops::jjrg_Pace)> = Vec::new();
                let mut first_remaining_pace: Option<(&String, &crate::jjrg_gallops::jjrg_Pace)> = None;

                for coronet_key in &heat.order {
                    if let Some(pace) = heat.paces.get(coronet_key) {
                        if let Some(tack) = pace.tacks.first() {
                            match tack.state {
                                PaceState::Complete => complete_count += 1,
                                PaceState::Abandoned => abandoned_count += 1,
                                PaceState::Rough => {
                                    rough_count += 1;
                                    remaining_paces.push((coronet_key, pace));
                                    if first_remaining_pace.is_none() {
                                        first_remaining_pace = Some((coronet_key, pace));
                                    }
                                }
                                PaceState::Bridled => {
                                    bridled_count += 1;
                                    remaining_paces.push((coronet_key, pace));
                                    if first_remaining_pace.is_none() {
                                        first_remaining_pace = Some((coronet_key, pace));
                                    }
                                }
                            }
                        }
                    }
                }

                let status_str = match heat.status {
                    HeatStatus::Racing => "racing",
                    HeatStatus::Stabled => "stabled",
                    HeatStatus::Retired => "retired",
                };

                let remaining_count = rough_count + bridled_count;

                // Header line with heat info
                println!("Heat: {} ({}) [{}]", heat.silks, heat_key, status_str);
                println!("Progress: {} complete | {} abandoned | {} remaining ({} rough, {} bridled)",
                    complete_count, abandoned_count, remaining_count, rough_count, bridled_count);
                println!();

                // Table with remaining paces
                if !remaining_paces.is_empty() {
                    // Set up table with column definitions
                    let mut table = jjrp_Table::jjrp_new(vec![
                        jjrp_Column::new("No", jjrp_Align::Right),
                        jjrp_Column::new("State", jjrp_Align::Left),
                        jjrp_Column::new("Pace", jjrp_Align::Left),
                        jjrp_Column::new("₢Coronet", jjrp_Align::Left),
                    ]);

                    // Measure all rows to compute column widths
                    for (idx, (coronet_key, pace)) in remaining_paces.iter().enumerate() {
                        if let Some(tack) = pace.tacks.first() {
                            let state_str = zjjrq_pace_state_str(&tack.state);
                            table.jjrp_measure(&[
                                &(idx + 1).to_string(),
                                state_str,
                                &tack.silks,
                                coronet_key,
                            ]);
                        }
                    }

                    // Print header and separator
                    table.jjrp_print_header();
                    table.jjrp_print_separator();

                    // Print data rows
                    for (idx, (coronet_key, pace)) in remaining_paces.iter().enumerate() {
                        if let Some(tack) = pace.tacks.first() {
                            let state_str = zjjrq_pace_state_str(&tack.state);
                            table.jjrp_print_row(&[
                                &(idx + 1).to_string(),
                                state_str,
                                &tack.silks,
                                coronet_key,
                            ]);
                        }
                    }
                    println!();
                }

                // Next up callout
                if let Some((coronet_key, pace)) = first_remaining_pace {
                    if let Some(tack) = pace.tacks.first() {
                        let state_str = zjjrq_pace_state_str(&tack.state);
                        println!("Next: {} ({}) [{}]", tack.silks, coronet_key, state_str);
                    }
                }
            } else {
                // Column-justified list view (all paces)
                // Set up table with column definitions
                let mut table = jjrp_Table::jjrp_new(vec![
                    jjrp_Column::new("No", jjrp_Align::Right),
                    jjrp_Column::new("State", jjrp_Align::Left),
                    jjrp_Column::new("Pace", jjrp_Align::Left),
                    jjrp_Column::new("₢Coronet", jjrp_Align::Left),
                ]);

                // Measure all rows to compute column widths
                let mut num = 0;
                for coronet_key in &heat.order {
                    if let Some(pace) = heat.paces.get(coronet_key) {
                        if let Some(tack) = pace.tacks.first() {
                            num += 1;
                            let state_str = zjjrq_pace_state_str(&tack.state);
                            table.jjrp_measure(&[
                                &num.to_string(),
                                state_str,
                                &tack.silks,
                                coronet_key,
                            ]);
                        }
                    }
                }

                // Print header and separator
                table.jjrp_print_header();
                table.jjrp_print_separator();

                // Print data rows
                let mut num = 0;
                for coronet_key in &heat.order {
                    if let Some(pace) = heat.paces.get(coronet_key) {
                        if let Some(tack) = pace.tacks.first() {
                            num += 1;
                            let state_str = zjjrq_pace_state_str(&tack.state);
                            table.jjrp_print_row(&[
                                &num.to_string(),
                                state_str,
                                &tack.silks,
                                coronet_key,
                            ]);
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("jjx_parade: error: target must be Firemark (2 chars) or Coronet (5 chars), got {} chars", target_str.len());
        return 1;
    }

    0
}

/// Helper to convert PaceState to display string
fn zjjrq_pace_state_str(state: &PaceState) -> &'static str {
    match state {
        PaceState::Rough => "rough",
        PaceState::Bridled => "bridled",
        PaceState::Complete => "complete",
        PaceState::Abandoned => "abandoned",
    }
}

// ============================================================================
// Scout - Search across heats and paces with regex
// ============================================================================

/// Arguments for scout command
#[derive(Debug)]
pub struct jjrq_ScoutArgs {
    pub file: std::path::PathBuf,
    pub pattern: String,
    pub actionable: bool,
}

/// Run the scout command - regex search across heats/paces
pub fn jjrq_run_scout(args: jjrq_ScoutArgs) -> i32 {
    use regex::RegexBuilder;

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_scout: error: {}", e);
            return 1;
        }
    };

    // Build case-insensitive regex
    let re = match RegexBuilder::new(&args.pattern)
        .case_insensitive(true)
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("jjx_scout: error: invalid regex pattern: {}", e);
            return 1;
        }
    };

    // Track which heats have matches (for group headers)
    let mut current_heat_key: Option<String> = None;

    // Iterate all heats (racing, stabled, retired)
    for (heat_key, heat) in &gallops.heats {
        // Search each pace in order
        for coronet_key in &heat.order {
            if let Some(pace) = heat.paces.get(coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    // Apply actionable filter if requested
                    if args.actionable {
                        match tack.state {
                            PaceState::Rough | PaceState::Bridled => {},
                            _ => continue,
                        }
                    }

                    // Search: silks, spec (tack text), direction, paddock content
                    // Use owned strings to avoid lifetime issues
                    let mut match_result: Option<(String, String)> = None;

                    if re.is_match(&tack.silks) {
                        match_result = Some(("silks".to_string(), tack.silks.clone()));
                    } else if re.is_match(&tack.text) {
                        match_result = Some(("spec".to_string(), tack.text.clone()));
                    } else if let Some(ref direction) = tack.direction {
                        if re.is_match(direction) {
                            match_result = Some(("direction".to_string(), direction.clone()));
                        }
                    } else {
                        // Search paddock content
                        let paddock_path = std::path::Path::new(&heat.paddock_file);
                        if let Ok(content) = fs::read_to_string(paddock_path) {
                            if re.is_match(&content) {
                                match_result = Some(("paddock".to_string(), content));
                            }
                        }
                    }

                    // If we found a match, output it
                    if let Some((field_name, field_content)) = match_result {
                        // Print heat header if this is a new heat
                        if current_heat_key.as_ref() != Some(heat_key) {
                            println!("{} {}", heat_key, heat.silks);
                            current_heat_key = Some(heat_key.clone());
                        }

                        // Print pace line
                        let state_str = zjjrq_pace_state_str(&tack.state);
                        println!("  {} [{}] {}", coronet_key, state_str, tack.silks);

                        // Print match line with context (extract ~60 chars around match)
                        let match_excerpt = zjjrq_extract_match_context(&field_content, &re);
                        println!("    {}: {}", field_name, match_excerpt);
                    }
                }
            }
        }
    }

    0
}

/// Extract context around the first regex match in content
fn zjjrq_extract_match_context(content: &str, re: &regex::Regex) -> String {
    if let Some(mat) = re.find(content) {
        let start = mat.start();
        let end = mat.end();

        // Find context window (30 chars before, 30 chars after)
        let context_before = 30;
        let context_after = 30;

        let window_start = start.saturating_sub(context_before);
        let window_end = (end + context_after).min(content.len());

        let mut excerpt = String::new();
        if window_start > 0 {
            excerpt.push_str("...");
        }
        excerpt.push_str(&content[window_start..window_end]);
        if window_end < content.len() {
            excerpt.push_str("...");
        }

        // Replace newlines with spaces for single-line display
        excerpt.replace('\n', " ").replace('\r', "")
    } else {
        // Shouldn't happen, but fallback to truncated content
        let truncated = content.chars().take(60).collect::<String>();
        if content.len() > 60 {
            format!("{}...", truncated)
        } else {
            truncated
        }
    }
}

// ============================================================================
// GetSpec - Get raw spec text for a Pace
// ============================================================================

/// Arguments for get_spec command
#[derive(Debug)]
pub struct jjrq_GetSpecArgs {
    pub file: std::path::PathBuf,
    pub coronet: Coronet,
}

/// Run the get_spec command - output raw spec text
pub fn jjrq_run_get_spec(args: jjrq_GetSpecArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_get_spec: error: {}", e);
            return 1;
        }
    };

    // Extract parent firemark and locate heat
    let firemark = args.coronet.jjrf_parent_firemark();
    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_get_spec: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Locate pace
    let coronet_key = args.coronet.jjrf_display();
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

// ============================================================================
// GetCoronets - List Coronets for a Heat
// ============================================================================

/// Arguments for get_coronets command
#[derive(Debug)]
pub struct jjrq_GetCoronetsArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
    pub remaining: bool,
    pub rough: bool,
}

/// Run the get_coronets command - output coronets one per line
pub fn jjrq_run_get_coronets(args: jjrq_GetCoronetsArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_get_coronets: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.jjrf_display();
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

// ============================================================================
// Retire - Extract complete Heat data for archival trophy
// ============================================================================

/// Arguments for retire command
#[derive(Debug)]
pub struct jjrq_RetireArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
}

/// Output structure for retire command
#[derive(Serialize)]
pub(crate) struct zjjrq_RetireOutput {
    pub(crate) firemark: String,
    pub(crate) silks: String,
    pub(crate) created: String,
    pub(crate) status: String,
    pub(crate) paddock_file: String,
    pub(crate) paddock_content: String,
    pub(crate) paces: Vec<zjjrq_RetirePace>,
    // TODO: steeplechase array from jjx_rein
    // steeplechase: Vec<SteeplechaseEntry>,
}

/// Pace structure for retire output (full tack history)
#[derive(Serialize)]
pub(crate) struct zjjrq_RetirePace {
    pub(crate) coronet: String,
    pub(crate) silks: String,
    pub(crate) tacks: Vec<zjjrq_RetireTack>,
}

/// Tack structure for retire output
#[derive(Serialize)]
pub(crate) struct zjjrq_RetireTack {
    pub(crate) ts: String,
    pub(crate) state: String,
    pub(crate) text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) direction: Option<String>,
}

/// Run the retire command - extract complete Heat data for archival
pub fn jjrq_run_retire(args: jjrq_RetireArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_retire: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_retire: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Build paces array with full tack history, ordered per Heat's order array
    let mut paces = Vec::new();
    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            let tacks: Vec<zjjrq_RetireTack> = pace.tacks.iter().map(|tack| {
                zjjrq_RetireTack {
                    ts: tack.ts.clone(),
                    state: match tack.state {
                        PaceState::Rough => "rough".to_string(),
                        PaceState::Bridled => "bridled".to_string(),
                        PaceState::Complete => "complete".to_string(),
                        PaceState::Abandoned => "abandoned".to_string(),
                    },
                    text: tack.text.clone(),
                    direction: tack.direction.clone(),
                }
            }).collect();

            paces.push(zjjrq_RetirePace {
                coronet: coronet_key.clone(),
                silks: pace.tacks.first().map(|t| t.silks.clone()).unwrap_or_default(),
                tacks,
            });
        }
    }

    let output = zjjrq_RetireOutput {
        firemark: heat_key.clone(),
        silks: heat.silks.clone(),
        created: heat.creation_time.clone(),
        status: match heat.status {
            HeatStatus::Racing => "racing".to_string(),
            HeatStatus::Stabled => "stabled".to_string(),
            HeatStatus::Retired => "retired".to_string(),
        },
        paddock_file: heat.paddock_file.clone(),
        paddock_content,
        paces,
        // TODO: Call jjx_rein internally to get steeplechase entries
    };

    // Output JSON
    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{}", json);
            0
        }
        Err(e) => {
            eprintln!("jjx_retire: error serializing output: {}", e);
            1
        }
    }
}

// ============================================================================
// Silks Sequence Parsing and Building
// ============================================================================

/// Parse silks into (base, sequence_number)
///
/// Silks ending with a 2-digit sequence number are parsed into base and number.
/// Format: "foo-bar" -> ("foo-bar", None)
///         "foo-bar-01" -> ("foo-bar", Some(1))
///         "foo-bar-42" -> ("foo-bar", Some(42))
pub fn jjrq_parse_silks_sequence(silks: &str) -> (String, Option<u32>) {
    // Use regex ^(.+)-(\d{2})$ for suffix detection
    let re = match Regex::new(r"^(.+)-(\d{2})$") {
        Ok(r) => r,
        Err(_) => {
            // Fallback: return full silks as base, no sequence
            return (silks.to_string(), None);
        }
    };

    if let Some(caps) = re.captures(silks) {
        if let (Some(base_match), Some(num_match)) = (caps.get(1), caps.get(2)) {
            let base = base_match.as_str().to_string();
            if let Ok(num_str) = num_match.as_str().parse::<u32>() {
                return (base, Some(num_str));
            }
        }
    }

    // No match: return full silks as base, no sequence
    (silks.to_string(), None)
}

/// Build garlanded silks: "garlanded-{base}-{seq:02}"
pub fn jjrq_build_garlanded_silks(base: &str, seq: u32) -> String {
    format!("garlanded-{}-{:02}", base, seq)
}

/// Build continuation silks: "{base}-{seq:02}"
pub fn jjrq_build_continuation_silks(base: &str, seq: u32) -> String {
    format!("{}-{:02}", base, seq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_silks_sequence_plain() {
        let (base, seq) = jjrq_parse_silks_sequence("foo-bar");
        assert_eq!(base, "foo-bar");
        assert_eq!(seq, None);
    }

    #[test]
    fn test_parse_silks_sequence_with_01_suffix() {
        let (base, seq) = jjrq_parse_silks_sequence("foo-bar-01");
        assert_eq!(base, "foo-bar");
        assert_eq!(seq, Some(1));
    }

    #[test]
    fn test_parse_silks_sequence_with_99_suffix() {
        let (base, seq) = jjrq_parse_silks_sequence("foo-bar-99");
        assert_eq!(base, "foo-bar");
        assert_eq!(seq, Some(99));
    }

    #[test]
    fn test_parse_silks_sequence_single_digit_not_matched() {
        let (base, seq) = jjrq_parse_silks_sequence("foo-bar-9");
        assert_eq!(base, "foo-bar-9");
        assert_eq!(seq, None);
    }

    #[test]
    fn test_build_garlanded_silks() {
        let result = jjrq_build_garlanded_silks("foo-bar", 1);
        assert_eq!(result, "garlanded-foo-bar-01");
    }

    #[test]
    fn test_build_garlanded_silks_high_number() {
        let result = jjrq_build_garlanded_silks("foo-bar", 42);
        assert_eq!(result, "garlanded-foo-bar-42");
    }

    #[test]
    fn test_build_continuation_silks() {
        let result = jjrq_build_continuation_silks("foo-bar", 1);
        assert_eq!(result, "foo-bar-01");
    }

    #[test]
    fn test_build_continuation_silks_high_number() {
        let result = jjrq_build_continuation_silks("foo-bar", 42);
        assert_eq!(result, "foo-bar-42");
    }
}

