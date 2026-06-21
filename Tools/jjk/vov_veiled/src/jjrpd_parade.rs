// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Parade command - Display comprehensive Heat status for project review
//!
//! Implements both heat view (Firemark) and pace view (Coronet).

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{
    jjrg_Gallops as Gallops,
    jjrg_Heat as Heat,
    jjrg_HeatStatus as HeatStatus,
    jjrg_lines_to_text,
    jjrg_PaceState as PaceState,
};
use crate::jjri_io::jjri_paddock_path;
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrq_query::{jjrq_files_for_pace, jjrq_file_touches_for_heat};
use crate::jjrs_steeplechase::{jjrs_ReinArgs, jjrs_get_entries, jjrs_SteeplechaseEntry};
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;

use vvc::{vvco_out, vvco_err, vvco_Output};

// Command name constant — RCG String Boundary Discipline
const JJRPD_CMD_NAME_SHOW: &str = "jjx_show";

/// Arguments for jjx_show command
#[derive(clap::Args, Debug)]
pub struct jjrpd_ParadeArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: std::path::PathBuf,

    /// Heterogeneous target list; each element self-types by length (Firemark
    /// -> heat expansion, Coronet -> single pace). Empty -> first racing heat.
    pub targets: Vec<String>,

    /// Show only remaining paces (exclude complete/abandoned). Affects firemark
    /// expansion only; a directly-named coronet returns regardless of state.
    #[arg(long)]
    pub remaining: bool,
}

/// Run the show command — terse tool-result table(s) plus an always-populated
/// gazette. `targets` is a heterogeneous list; each element self-types by
/// length (Firemark -> heat expansion, Coronet -> single pace). An empty list
/// auto-selects the first racing heat (the orient/mount/groom default). Pace
/// and paddock bodies reach the caller only through the gazette — never the
/// tool-result.
pub fn jjrpd_run_parade(args: jjrpd_ParadeArgs, gazette: &mut jjrz_Gazette) -> (i32, String) {
    let cn = JJRPD_CMD_NAME_SHOW;
    let mut output = vvco_Output::buffer();

    // Disk space guard — report survey or block if critical
    match crate::jjrdk_diskcheck::jjrdk_check_disk_space() {
        Ok(survey) => vvco_out!(output, "{}", survey),
        Err(msg) => {
            vvco_err!(output, "{}", msg);
            return (1, output.vvco_finish());
        }
    }

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Target selection now arrives solely through the gazette halter notice the
    // MCP arm parses — there is no empty-targets auto-select. An empty list is a
    // caller bug (the zero-write path the move was made to close), so it errors
    // rather than silently picking a heat.
    let targets: Vec<String> = args.targets.clone();
    if targets.is_empty() {
        vvco_err!(output, "{}: error: no target specified — name at least one firemark or coronet", cn);
        return (1, output.vvco_finish());
    }

    // The file-touch bitmap and commit swim lanes assume a single heat; render
    // them only when the request resolves to exactly one firemark target.
    let single_firemark = targets.len() == 1 && zjjrpd_strip_glyph(&targets[0]).len() == 2;

    let mut added_paddocks: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut added_paces: std::collections::HashSet<String> = std::collections::HashSet::new();

    for target in &targets {
        let target_str = zjjrpd_strip_glyph(target);
        let res = match target_str.len() {
            5 => zjjrpd_emit_coronet(&mut output, &gallops, target, gazette, &mut added_paddocks, &mut added_paces),
            2 => zjjrpd_emit_firemark(&mut output, &gallops, target, args.remaining, single_firemark, gazette, &mut added_paddocks, &mut added_paces),
            n => Err(format!(
                "{}: error: target '{}' must be Firemark (2 chars) or Coronet (5 chars), got {} chars",
                cn, target, n
            )),
        };
        if let Err(e) = res {
            vvco_err!(output, "{}", e);
            return (1, output.vvco_finish());
        }
    }

    (0, output.vvco_finish())
}

/// Strip an optional ₢/₣ identity glyph, returning the bare base64 tail.
fn zjjrpd_strip_glyph(target: &str) -> &str {
    target.strip_prefix('₢').or_else(|| target.strip_prefix('₣')).unwrap_or(target)
}

/// Add a heat's paddock to the gazette at most once per heat. Best-effort: a
/// missing paddock file is skipped (the pace dockets are the round-trip payload).
fn zjjrpd_gazette_paddock_once(
    gazette: &mut jjrz_Gazette,
    added: &mut std::collections::HashSet<String>,
    firemark: &Firemark,
) {
    let heat_key = firemark.jjrf_display();
    if added.insert(heat_key.clone()) {
        let paddock_path = jjri_paddock_path(firemark.jjrf_as_str());
        if let Ok(content) = fs::read_to_string(&paddock_path) {
            gazette.jjrz_add(jjrz_Slug::Paddock, &heat_key, &content).ok();
        }
    }
}

/// Emit one coronet target: a terse pace header to the tool-result, the pace
/// docket plus its parent paddock to the gazette. A directly-named coronet
/// returns regardless of pace state.
fn zjjrpd_emit_coronet(
    output: &mut vvco_Output,
    gallops: &Gallops,
    target: &str,
    gazette: &mut jjrz_Gazette,
    added_paddocks: &mut std::collections::HashSet<String>,
    added_paces: &mut std::collections::HashSet<String>,
) -> Result<(), String> {
    let cn = JJRPD_CMD_NAME_SHOW;
    let coronet = Coronet::jjrf_parse(target).map_err(|e| format!("{}: error: {}", cn, e))?;
    let firemark = coronet.jjrf_parent_firemark();
    let heat_key = firemark.jjrf_display();
    let heat = gallops.heats.get(&heat_key)
        .ok_or_else(|| format!("{}: error: Heat '{}' not found", cn, heat_key))?;
    let coronet_key = coronet.jjrf_display();
    let pace = heat.paces.get(&coronet_key)
        .ok_or_else(|| format!("{}: error: Pace '{}' not found in Heat '{}'", cn, coronet_key, heat_key))?;

    let current_tack = match pace.tacks.first() {
        Some(t) => t,
        None => return Ok(()),
    };

    // Terse header only — the docket body reaches context through the gazette.
    vvco_out!(output, "Pace: {} ({}) [{}]", current_tack.silks, coronet_key, zjjrpd_pace_state_str(&current_tack.state));
    vvco_out!(output, "Heat: {}", heat_key);
    if let Ok(files) = jjrq_files_for_pace(firemark.jjrf_as_str(), &coronet_key) {
        if !files.is_empty() {
            vvco_out!(output, "Work files: {}", files.join(", "));
        }
    }
    vvco_out!(output, "");

    zjjrpd_gazette_paddock_once(gazette, added_paddocks, &firemark);
    if added_paces.insert(coronet_key.clone()) {
        let pace_text = jjrg_lines_to_text(&current_tack.text);
        gazette.jjrz_add(jjrz_Slug::Pace, &coronet_key, &pace_text).ok();
    }
    Ok(())
}

/// Emit one firemark target: the terse numbered pace table to the tool-result
/// (remaining-filtered when requested), the paddock plus per-pace dockets to
/// the gazette. Bitmap and swim lanes are appended only when `with_bitmaps`.
fn zjjrpd_emit_firemark(
    output: &mut vvco_Output,
    gallops: &Gallops,
    target: &str,
    remaining: bool,
    with_bitmaps: bool,
    gazette: &mut jjrz_Gazette,
    added_paddocks: &mut std::collections::HashSet<String>,
    added_paces: &mut std::collections::HashSet<String>,
) -> Result<(), String> {
    let cn = JJRPD_CMD_NAME_SHOW;
    let firemark = Firemark::jjrf_parse(target).map_err(|e| format!("{}: error: {}", cn, e))?;
    let heat_key = firemark.jjrf_display();
    let heat = gallops.heats.get(&heat_key)
        .ok_or_else(|| format!("{}: error: Heat '{}' not found", cn, heat_key))?;

    {
        {
            // List view: numbered paces
            // If --remaining, output markdown format
            if remaining {
                let mut complete_count = 0;
                let mut abandoned_count = 0;
                let mut rough_count = 0;
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
                            }
                        }
                    }
                }

                let status_str = match heat.status {
                    HeatStatus::Racing => "racing",
                    HeatStatus::Stabled => "stabled",
                    HeatStatus::Retired => "retired",
                };

                let remaining_count = rough_count;

                // Header line with heat info
                vvco_out!(output, "Heat: {} ({}) [{}]", heat.silks, heat_key, status_str);
                vvco_out!(output, "Progress: {} complete | {} abandoned | {} remaining ({} rough)",
                    complete_count, abandoned_count, remaining_count, rough_count);
                vvco_out!(output, "");

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

                    // Write header and separator
                    table.jjrp_write_header(output);
                    table.jjrp_write_separator(output);

                    // Write data rows
                    for (idx, (coronet_key, pace)) in remaining_paces.iter().enumerate() {
                        if let Some(tack) = pace.tacks.first() {
                            let state_str = zjjrpd_pace_state_str(&tack.state);
                            table.jjrp_write_row(output, &[
                                &(idx + 1).to_string(),
                                state_str,
                                &tack.silks,
                                coronet_key,
                            ]);
                        }
                    }
                    vvco_out!(output, "");
                }

                // Next up callout
                if let Some((coronet_key, pace)) = first_remaining_pace {
                    if let Some(tack) = pace.tacks.first() {
                        let state_str = zjjrpd_pace_state_str(&tack.state);
                        vvco_out!(output, "Next: {} ({}) [{}]", tack.silks, coronet_key, state_str);
                        vvco_out!(output, "");
                        vvco_out!(output, "Recommended: mount {}", heat_key);
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

                // Write header and separator
                table.jjrp_write_header(output);
                table.jjrp_write_separator(output);

                // Write data rows
                let mut num = 0;
                for coronet_key in &heat.order {
                    if let Some(pace) = heat.paces.get(coronet_key) {
                        if let Some(tack) = pace.tacks.first() {
                            num += 1;
                            let state_str = zjjrpd_pace_state_str(&tack.state);
                            table.jjrp_write_row(output, &[
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

            // The bitmap and swim lanes assume a single heat — render only for
            // a lone firemark target.
            if with_bitmaps {
                jjrpd_write_file_bitmap(output, &firemark, heat);
                jjrpd_write_commit_swimlanes(output, &firemark, heat);
            }
        }

    // Always populate the gazette: paddock once, then each pace docket
    // (remaining-filtered to match the table).
    zjjrpd_gazette_paddock_once(gazette, added_paddocks, &firemark);
    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                if remaining && (tack.state == PaceState::Complete || tack.state == PaceState::Abandoned) {
                    continue;
                }
                if added_paces.insert(coronet_key.clone()) {
                    let pace_text = jjrg_lines_to_text(&tack.text);
                    gazette.jjrz_add(jjrz_Slug::Pace, coronet_key, &pace_text).ok();
                }
            }
        }
    }
    Ok(())
}

/// Format the file-touch bitmap for a heat as a String.
///
/// Uses shared query routines from jjrq_query to get file touches,
/// then formats as a bitmap with columns per pace and rows per file,
/// grouped by identical touch patterns.
pub fn jjrpd_format_file_bitmap(firemark: &Firemark, heat: &Heat) -> Result<String, String> {
    let touches = jjrq_file_touches_for_heat(firemark.jjrf_as_str())?;

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

    let total_cols = pace_columns.len();

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

    let mut output = String::new();

    if file_touches_map.is_empty() {
        writeln!(output, "File-touch bitmap: (no work file changes)").unwrap();
        return Ok(output);
    }

    // Group files by identical touch pattern
    let mut pattern_groups: BTreeMap<Vec<bool>, Vec<String>> = BTreeMap::new();
    for (filename, pattern) in &file_touches_map {
        pattern_groups.entry(pattern.clone()).or_default().push(filename.clone());
    }

    for files in pattern_groups.values_mut() {
        files.sort();
    }

    // Header
    writeln!(output, "File-touch bitmap (x = pace commit touched file):").unwrap();
    writeln!(output).unwrap();

    // Vertical legend
    for (i, (ch, _coronet, silks)) in pace_columns.iter().enumerate() {
        writeln!(output, "  {} {} {}", i + 1, ch, silks).unwrap();
    }
    writeln!(output).unwrap();

    // Column header line (terminal chars aligned with bitmap positions)
    let header_chars: Vec<char> = pace_columns.iter().map(|(ch, _, _)| *ch).collect();
    let header_line: String = header_chars.iter().collect();
    writeln!(output, "{}", header_line).unwrap();

    // Sort patterns: more touches first, then lexicographic
    let mut sorted_patterns: Vec<(Vec<bool>, Vec<String>)> = pattern_groups.into_iter().collect();
    sorted_patterns.sort_by(|(pat_a, _), (pat_b, _)| {
        let count_a: usize = pat_a.iter().filter(|&&b| b).count();
        let count_b: usize = pat_b.iter().filter(|&&b| b).count();
        count_b.cmp(&count_a).then_with(|| pat_a.cmp(pat_b))
    });

    for (pattern, files) in &sorted_patterns {
        let bitmap: String = pattern.iter().map(|&b| if b { 'x' } else { '·' }).collect();
        writeln!(output, "{} {}", bitmap, files.join(", ")).unwrap();
    }

    Ok(output)
}

/// Write the file-touch bitmap for a heat to output.
pub(crate) fn jjrpd_write_file_bitmap(output: &mut vvco_Output, firemark: &Firemark, heat: &Heat) {
    let cn = JJRPD_CMD_NAME_SHOW;
    match jjrpd_format_file_bitmap(firemark, heat) {
        Ok(text) => {
            for line in text.lines() {
                vvco_out!(output, "{}", line);
            }
        }
        Err(e) => vvco_err!(output, "{}: error getting file touches: {}", cn, e),
    }
}

/// Helper to convert PaceState to display string
fn zjjrpd_pace_state_str(state: &PaceState) -> &'static str {
    match state {
        PaceState::Rough => "rough",
        PaceState::Complete => "complete",
        PaceState::Abandoned => "abandoned",
    }
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

/// Format commit swim lanes for a heat as a String: paces × commits matrix.
///
/// Shows when work happened on each pace and how work interleaved.
pub fn jjrpd_format_commit_swimlanes(firemark: &Firemark, heat: &Heat) -> Result<String, String> {
    let rein_args = jjrs_ReinArgs {
        firemark: firemark.jjrf_as_str().to_string(),
        limit: 10000,
    };

    let entries = jjrs_get_entries(&rein_args)?;

    if entries.is_empty() {
        return Ok(String::new());
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
        }
        // heat-level entries (no coronet) are intentionally excluded from swim lanes
    }

    let total_rows = pace_order.len();

    if total_rows == 0 {
        return Ok(String::new());
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
        }
        // heat-level entries (no coronet) are intentionally excluded from swim lanes
    }

    let mut output = String::new();

    writeln!(output).unwrap();
    writeln!(output, "Commit swim lanes (x = commit affiliated with pace):").unwrap();
    writeln!(output).unwrap();

    if truncated {
        writeln!(output, "(showing last {} of {} commits)", max_commits, total_commits).unwrap();
        writeln!(output).unwrap();
    }

    // Vertical legend
    for (i, (_, ch, silks)) in pace_order.iter().enumerate() {
        writeln!(output, "  {} {} {}", i + 1, ch, silks).unwrap();
    }
    writeln!(output).unwrap();

    // Column header line
    let header: String = (0..num_cols).filter_map(zjjrpd_commit_index_char).collect();
    writeln!(output, "{}", header).unwrap();

    // Bitmap rows
    for row in 0..total_rows {
        let bits: String = bitmap[row].iter().map(|&b| if b { 'x' } else { '·' }).collect();
        let (term_char, count) = (pace_order[row].1, row_commit_counts[row]);
        writeln!(output, "{}  {}  {}c", bits, term_char, count).unwrap();
    }

    Ok(output)
}

/// Write commit swim lanes for a heat to output.
pub(crate) fn jjrpd_write_commit_swimlanes(output: &mut vvco_Output, firemark: &Firemark, heat: &Heat) {
    let cn = JJRPD_CMD_NAME_SHOW;
    match jjrpd_format_commit_swimlanes(firemark, heat) {
        Ok(text) => {
            for line in text.lines() {
                vvco_out!(output, "{}", line);
            }
        }
        Err(e) => vvco_err!(output, "{}: error getting steeplechase entries: {}", cn, e),
    }
}
