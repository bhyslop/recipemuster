// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Parade command - Display comprehensive Heat status for project review
//!
//! Implements both heat view (Firemark) and pace view (Coronet).

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_Heat as Heat, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrq_query::{jjrq_files_for_pace, jjrq_file_touches_for_heat};
use std::collections::BTreeMap;
use std::fs;

/// Arguments for jjx_parade command
#[derive(clap::Args, Debug)]
pub struct jjrpd_ParadeArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: std::path::PathBuf,

    /// Target: Firemark (heat view) or Coronet (pace view). If omitted, uses first racing heat.
    pub target: Option<String>,

    /// Show paddock and full specs (heat mode only)
    #[arg(long)]
    pub full: bool,

    /// Show only remaining paces (exclude complete/abandoned)
    #[arg(long)]
    pub remaining: bool,
}

/// Run the parade command - display comprehensive Heat status
pub fn jjrpd_run_parade(args: jjrpd_ParadeArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    // Resolve target: use provided value or auto-select first racing heat
    let target = match args.target {
        Some(t) => t,
        None => {
            // Load gallops to resolve default heat
            match zjjrpd_resolve_default_heat(&gallops) {
                Ok(fm) => fm,
                Err(e) => {
                    eprintln!("jjx_parade: error: {}", e);
                    return 1;
                }
            }
        }
    };

    // Determine target type by length
    let target_str = target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(&target);

    if target_str.len() == 5 {
        // Coronet - pace view
        let coronet = match Coronet::jjrf_parse(&target) {
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

            // Show work files touched by this pace's commits
            match jjrq_files_for_pace(firemark.jjrf_as_str(), &coronet_key) {
                Ok(files) if !files.is_empty() => {
                    println!("Work files: {}", files.join(", "));
                }
                _ => {}
            }
            println!();

            // Iterate tacks in reverse order (oldest first)
            for (index, tack) in pace.tacks.iter().rev().enumerate() {
                let state_str = zjjrpd_pace_state_str(&tack.state);
                let basis_str = if tack.basis == "0000000" {
                    "(no basis)".to_string()
                } else {
                    tack.basis.clone()
                };

                println!("[{}] {} (basis: {})", index, state_str, basis_str);
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
        let firemark = match Firemark::jjrf_parse(&target) {
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
                        let state_str = zjjrpd_pace_state_str(&tack.state);
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
                            let state_str = zjjrpd_pace_state_str(&tack.state);
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
                            let state_str = zjjrpd_pace_state_str(&tack.state);
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
                        let state_str = zjjrpd_pace_state_str(&tack.state);
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
                            let state_str = zjjrpd_pace_state_str(&tack.state);
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
                            let state_str = zjjrpd_pace_state_str(&tack.state);
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

        // Always show file-touch bitmap after pace listing
        zjjrpd_print_file_bitmap(&firemark, heat);
    } else {
        eprintln!("jjx_parade: error: target must be Firemark (2 chars) or Coronet (5 chars), got {} chars", target_str.len());
        return 1;
    }

    0
}

/// Build and print the file-touch bitmap for a heat.
///
/// Uses shared query routines from jjrq_query to get file touches,
/// then formats as a bitmap with columns per pace and rows per file,
/// grouped by identical touch patterns.
fn zjjrpd_print_file_bitmap(firemark: &Firemark, heat: &Heat) {
    let touches = match jjrq_file_touches_for_heat(firemark.jjrf_as_str()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_parade: error getting file touches: {}", e);
            return;
        }
    };

    let has_heat_level = !touches.heat_files.is_empty();

    // Build pace columns: only paces with commits, in heat order
    let mut pace_columns: Vec<(char, String, String)> = Vec::new();
    for coronet_key in &heat.order {
        if !touches.coronets_with_commits.contains(coronet_key) {
            continue;
        }
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                let raw = coronet_key.strip_prefix('₢').unwrap_or(coronet_key);
                if let Some(ch) = raw.chars().last() {
                    pace_columns.push((ch, coronet_key.clone(), tack.silks.clone()));
                }
            }
        }
    }

    // Map coronet_display -> column index
    let mut coronet_to_col: BTreeMap<String, usize> = BTreeMap::new();
    for (idx, (_, coronet_display, _)) in pace_columns.iter().enumerate() {
        coronet_to_col.insert(coronet_display.clone(), idx);
    }

    let heat_col_idx = pace_columns.len();
    let total_cols = if has_heat_level { heat_col_idx + 1 } else { heat_col_idx };

    // Build file -> touch vector from the shared query result
    let mut file_touches_map: BTreeMap<String, Vec<bool>> = BTreeMap::new();

    for (coronet, files) in &touches.pace_files {
        if let Some(&col_idx) = coronet_to_col.get(coronet) {
            for filename in files {
                let row = file_touches_map.entry(filename.clone()).or_insert_with(|| vec![false; total_cols]);
                if col_idx < row.len() {
                    row[col_idx] = true;
                }
            }
        }
    }

    if has_heat_level {
        for filename in &touches.heat_files {
            let row = file_touches_map.entry(filename.clone()).or_insert_with(|| vec![false; total_cols]);
            if heat_col_idx < row.len() {
                row[heat_col_idx] = true;
            }
        }
    }

    if file_touches_map.is_empty() {
        println!("File-touch bitmap: (no work file changes)");
        return;
    }

    // Group files by identical touch pattern
    let mut pattern_groups: BTreeMap<Vec<bool>, Vec<String>> = BTreeMap::new();
    for (filename, pattern) in &file_touches_map {
        pattern_groups.entry(pattern.clone()).or_default().push(filename.clone());
    }

    for files in pattern_groups.values_mut() {
        files.sort();
    }

    // Print header
    println!("File-touch bitmap (x = pace commit touched file):");
    println!();

    // Print vertical legend
    for (i, (ch, _coronet, silks)) in pace_columns.iter().enumerate() {
        println!("  {} {} {}", i + 1, ch, silks);
    }
    if has_heat_level {
        println!("  {} * heat-level", pace_columns.len() + 1);
    }
    println!();

    // Print column header line (terminal chars aligned with bitmap positions)
    let mut header_chars: Vec<char> = pace_columns.iter().map(|(ch, _, _)| *ch).collect();
    if has_heat_level {
        header_chars.push('*');
    }
    let header_line: String = header_chars.iter().collect();
    println!("{}", header_line);

    // Sort patterns: more touches first, then lexicographic
    let mut sorted_patterns: Vec<(Vec<bool>, Vec<String>)> = pattern_groups.into_iter().collect();
    sorted_patterns.sort_by(|(pat_a, _), (pat_b, _)| {
        let count_a: usize = pat_a.iter().filter(|&&b| b).count();
        let count_b: usize = pat_b.iter().filter(|&&b| b).count();
        count_b.cmp(&count_a).then_with(|| pat_a.cmp(pat_b))
    });

    for (pattern, files) in &sorted_patterns {
        let bitmap: String = pattern.iter().map(|&b| if b { 'x' } else { '·' }).collect();
        println!("{} {}", bitmap, files.join(", "));
    }
}

/// Helper to convert PaceState to display string
pub(crate) fn zjjrpd_pace_state_str(state: &PaceState) -> &'static str {
    match state {
        PaceState::Rough => "rough",
        PaceState::Bridled => "bridled",
        PaceState::Complete => "complete",
        PaceState::Abandoned => "abandoned",
    }
}

/// Resolve the default heat (first racing heat) when no target is specified
pub(crate) fn zjjrpd_resolve_default_heat(gallops: &Gallops) -> Result<String, String> {
    for (heat_key, heat) in &gallops.heats {
        if heat.status == HeatStatus::Racing {
            return Ok(heat_key.clone());
        }
    }
    Err("No racing heats found".to_string())
}
