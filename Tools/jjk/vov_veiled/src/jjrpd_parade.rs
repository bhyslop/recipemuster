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
use crate::jjrq_query::{jjrq_files_for_pace, jjrq_file_touches_for_heat, zjjrq_files_for_commit, zjjrq_bare_filename, zjjrq_is_infra_file};
use crate::jjrs_steeplechase::{jjrs_ReinArgs, jjrs_get_entries, jjrs_SteeplechaseEntry};
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

            // Append files-per-commit bitmap
            zjjrpd_print_pace_commits(&firemark, &coronet_key);
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

        // Always show file-touch bitmap and commit swim lanes after pace listing
        zjjrpd_print_file_bitmap(&firemark, heat);
        zjjrpd_print_commit_swimlanes(&firemark, heat);
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

/// Encode a column index as a dense character: 1-9 then a-z then A-Z (61 max)
fn zjjrpd_commit_index_char(index: usize) -> Option<char> {
    match index {
        0..=8 => Some((b'1' + index as u8) as char),
        9..=34 => Some((b'a' + (index - 9) as u8) as char),
        35..=60 => Some((b'A' + (index - 35) as u8) as char),
        _ => None,
    }
}

/// Print commit swim lanes for a heat: paces × commits matrix.
///
/// Shows when work happened on each pace and how work interleaved.
/// Called after file-touch bitmap in heat parade.
fn zjjrpd_print_commit_swimlanes(firemark: &Firemark, heat: &Heat) {
    let rein_args = jjrs_ReinArgs {
        firemark: firemark.jjrf_as_str().to_string(),
        limit: 10000,
    };

    let entries = match jjrs_get_entries(&rein_args) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("jjx_parade: error getting steeplechase entries: {}", e);
            return;
        }
    };

    if entries.is_empty() {
        return;
    }

    // Entries are newest-first; truncate to 35 most recent, then reverse for chronological
    let total_commits = entries.len();
    let max_commits: usize = 35;
    let truncated = total_commits > max_commits;
    let display_entries: Vec<&jjrs_SteeplechaseEntry> = entries
        .iter()
        .take(max_commits)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // Build pace rows: ordered by first chronological appearance
    let mut pace_order: Vec<(String, char, String)> = Vec::new(); // (coronet_display, term_char, silks)
    let mut pace_index: BTreeMap<String, usize> = BTreeMap::new();
    let mut has_heat_level = false;

    for entry in &display_entries {
        if let Some(ref coronet) = entry.coronet {
            if !pace_index.contains_key(coronet) {
                let idx = pace_order.len();
                let raw = coronet.strip_prefix('₢').unwrap_or(coronet);
                let ch = raw.chars().last().unwrap_or('?');
                let silks = heat.paces.get(coronet)
                    .and_then(|p| p.tacks.first())
                    .map(|t| t.silks.clone())
                    .unwrap_or_else(|| "unknown".to_string());
                pace_index.insert(coronet.clone(), idx);
                pace_order.push((coronet.clone(), ch, silks));
            }
        } else {
            has_heat_level = true;
        }
    }

    let total_rows = pace_order.len() + if has_heat_level { 1 } else { 0 };
    let heat_row_idx = pace_order.len();

    if total_rows == 0 {
        return;
    }

    // Build bitmap: rows × columns
    let num_cols = display_entries.len();
    let mut bitmap: Vec<Vec<bool>> = vec![vec![false; num_cols]; total_rows];
    let mut row_commit_counts: Vec<usize> = vec![0; total_rows];

    for (col, entry) in display_entries.iter().enumerate() {
        if let Some(ref coronet) = entry.coronet {
            if let Some(&row) = pace_index.get(coronet) {
                bitmap[row][col] = true;
                row_commit_counts[row] += 1;
            }
        } else if has_heat_level {
            bitmap[heat_row_idx][col] = true;
            row_commit_counts[heat_row_idx] += 1;
        }
    }

    // Print
    println!();
    println!("Commit swim lanes (x = commit affiliated with pace):");
    println!();

    if truncated {
        println!("(showing last {} of {} commits)", max_commits, total_commits);
        println!();
    }

    // Vertical legend
    for (i, (_, ch, silks)) in pace_order.iter().enumerate() {
        println!("  {} {} {}", i + 1, ch, silks);
    }
    if has_heat_level {
        println!("  {} * heat-level", pace_order.len() + 1);
    }
    println!();

    // Column header line
    let header: String = (0..num_cols).filter_map(zjjrpd_commit_index_char).collect();
    println!("{}", header);

    // Bitmap rows
    for row in 0..total_rows {
        let bits: String = bitmap[row].iter().map(|&b| if b { 'x' } else { '·' }).collect();
        let (term_char, count) = if row < pace_order.len() {
            (pace_order[row].1, row_commit_counts[row])
        } else {
            ('*', row_commit_counts[row])
        };
        println!("{}  {}  {}c", bits, term_char, count);
    }
}

/// Print files-per-commit bitmap for a single pace.
///
/// Shows which files each commit in this pace touched.
/// Called after tack history in pace parade.
fn zjjrpd_print_pace_commits(firemark: &Firemark, coronet_key: &str) {
    let rein_args = jjrs_ReinArgs {
        firemark: firemark.jjrf_as_str().to_string(),
        limit: 10000,
    };

    let entries = match jjrs_get_entries(&rein_args) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("jjx_parade: error getting steeplechase entries: {}", e);
            return;
        }
    };

    // Filter by coronet, then reverse for chronological order
    let pace_entries: Vec<&jjrs_SteeplechaseEntry> = entries
        .iter()
        .filter(|e| e.coronet.as_deref() == Some(coronet_key))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    if pace_entries.is_empty() {
        return;
    }

    // Get files for each commit
    let num_commits = pace_entries.len();
    let mut file_touches: BTreeMap<String, Vec<bool>> = BTreeMap::new();

    for (col, entry) in pace_entries.iter().enumerate() {
        let files = zjjrq_files_for_commit(&entry.commit);
        for path in files {
            if zjjrq_is_infra_file(&path) {
                continue;
            }
            let bare = zjjrq_bare_filename(&path);
            let row = file_touches.entry(bare).or_insert_with(|| vec![false; num_commits]);
            row[col] = true;
        }
    }

    if file_touches.is_empty() {
        println!();
        println!("Pace commits: (no project file commits)");
        return;
    }

    println!();
    println!("Pace commits (x = commit touched file):");
    println!();

    // Legend: commit index char, SHA, subject
    for (i, entry) in pace_entries.iter().enumerate() {
        let ch = zjjrpd_commit_index_char(i).unwrap_or('?');
        println!("  {} {}  {}", ch, entry.commit, entry.subject);
    }
    println!();

    // Column header line
    let header: String = (0..num_commits).filter_map(zjjrpd_commit_index_char).collect();
    println!("{}", header);

    // Group by identical touch pattern
    let mut pattern_groups: BTreeMap<Vec<bool>, Vec<String>> = BTreeMap::new();
    for (filename, pattern) in &file_touches {
        pattern_groups.entry(pattern.clone()).or_default().push(filename.clone());
    }
    for files in pattern_groups.values_mut() {
        files.sort();
    }

    // Sort: more touches first, then lexicographic
    let mut sorted: Vec<(Vec<bool>, Vec<String>)> = pattern_groups.into_iter().collect();
    sorted.sort_by(|(a, _), (b, _)| {
        let ca: usize = a.iter().filter(|&&v| v).count();
        let cb: usize = b.iter().filter(|&&v| v).count();
        cb.cmp(&ca).then_with(|| a.cmp(b))
    });

    for (pattern, files) in &sorted {
        let bits: String = pattern.iter().map(|&b| if b { 'x' } else { '·' }).collect();
        println!("{} {}", bits, files.join(", "));
    }
}
