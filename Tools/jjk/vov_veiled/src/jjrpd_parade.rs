// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Parade command - Display comprehensive Heat status for project review
//!
//! Implements both heat view (Firemark) and pace view (Coronet).

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrf_favor::{JJRF_FIREMARK_LEN, JJRF_CORONET_LEN};
use crate::jjrg_gallops::{
    jjrg_Gallops as Gallops,
    jjrg_Heat as Heat,
    jjrg_HeatStatus as HeatStatus,
    jjrg_lines_to_text,
    jjrg_PaceState as PaceState,
};
use crate::jjri_io::{
    jjri_paddock_path,
    jjri_show_blob,
};
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

    /// Hark mode (JJS0 jjda_hark): when Some(rev), source the Gallops and each
    /// heat paddock from that git revision (read-only retrospective via jjdr_hark)
    /// instead of the working tree, suppress the live-git bitmaps, and stamp AS OF.
    #[arg(long)]
    pub hark: Option<String>,
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

    let gallops = match zjjrpd_load_gallops(&args) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Hark mode (JJS0 jjda_hark): stamp the retrospective banner once the historical
    // store has loaded, and note that the live-git bitmaps are omitted (they reflect
    // current HEAD history, not the rev's, so they have no place in a snapshot).
    if let Some(rev) = &args.hark {
        vvco_out!(output, "=== HARK — state AS OF {} (read-only retrospective; live-git bitmaps omitted) ===", rev);
        vvco_out!(output, "");
    }

    // Target selection now arrives solely through the gazette halter notice the
    // MCP arm parses — there is no empty-targets auto-select. An empty list is a
    // caller bug (the zero-write path the move was made to close), so it errors
    // rather than silently picking a heat.
    let targets: Vec<String> = args.targets.clone();
    if targets.is_empty() {
        vvco_err!(output, "{}: error: no target specified — name at least one firemark or coronet", cn);
        return (1, output.vvco_finish());
    }

    // The file-touch census and commit swim lanes assume a single heat; render
    // them only when the request resolves to exactly one firemark target.
    // Both derive from live git history (current HEAD), so they are incoherent
    // in hark mode; suppress them there.
    let single_firemark = args.hark.is_none() && targets.len() == 1 && zjjrpd_strip_glyph(&targets[0]).len() == JJRF_FIREMARK_LEN;

    let mut added_paddocks: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut added_paces: std::collections::HashSet<String> = std::collections::HashSet::new();

    let hark = args.hark.as_deref();
    for target in &targets {
        let target_str = zjjrpd_strip_glyph(target);
        let res = match target_str.len() {
            JJRF_CORONET_LEN => zjjrpd_emit_coronet(&mut output, &gallops, target, gazette, &mut added_paddocks, &mut added_paces, hark),
            JJRF_FIREMARK_LEN => zjjrpd_emit_firemark(&mut output, &gallops, target, args.remaining, single_firemark, gazette, &mut added_paddocks, &mut added_paces, hark),
            n => Err(format!(
                "{}: error: target '{}' must be Firemark ({} chars) or Coronet ({} chars), got {} chars",
                cn, target, JJRF_FIREMARK_LEN, JJRF_CORONET_LEN, n
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

/// Load the Gallops for a parade run: the working-tree file by default, or — in
/// hark mode (JJS0 jjda_hark) — the blob at `args.hark`'s git revision via the
/// read-only jjdr_hark path. Both flavors return a validated Gallops.
fn zjjrpd_load_gallops(args: &jjrpd_ParadeArgs) -> Result<Gallops, String> {
    match &args.hark {
        Some(rev) => {
            let path = args.file.to_string_lossy();
            let bytes = jjri_show_blob(rev, &path)?;
            Gallops::jjrg_hark(&bytes)
        }
        None => Gallops::jjrg_load(&args.file),
    }
}

/// Add a heat's paddock to the gazette at most once per heat. Best-effort: a
/// missing paddock file is skipped (the pace dockets are the round-trip payload).
fn zjjrpd_gazette_paddock_once(
    gazette: &mut jjrz_Gazette,
    added: &mut std::collections::HashSet<String>,
    firemark: &Firemark,
    hark: Option<&str>,
) {
    let heat_key = firemark.jjrf_display();
    if added.insert(heat_key.clone()) {
        let paddock_path = jjri_paddock_path(firemark.jjrf_as_str());
        // Hark mode reads the paddock blob at the same revision as the Gallops, so
        // the rendered paddock is coherent with the historical dockets.
        let content = match hark {
            Some(rev) => jjri_show_blob(rev, &paddock_path)
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok()),
            None => fs::read_to_string(&paddock_path).ok(),
        };
        if let Some(content) = content {
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
    hark: Option<&str>,
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
    vvco_out!(output, "Pace: {} ({}) [{}]", current_tack.silks, coronet_key, current_tack.jjrg_state_label());
    vvco_out!(output, "Heat: {}", heat_key);
    if let Ok(files) = jjrq_files_for_pace(firemark.jjrf_as_str(), &coronet_key) {
        if !files.is_empty() {
            vvco_out!(output, "Work files: {}", files.join(", "));
        }
    }
    vvco_out!(output, "");

    zjjrpd_gazette_paddock_once(gazette, added_paddocks, &firemark, hark);
    if added_paces.insert(coronet_key.clone()) {
        let pace_text = jjrg_lines_to_text(&current_tack.text);
        gazette.jjrz_add(jjrz_Slug::Pace, &coronet_key, &pace_text).ok();
    }
    Ok(())
}

/// Emit one firemark target: the terse numbered pace table to the tool-result
/// (remaining-filtered when requested), the paddock plus per-pace dockets to
/// the gazette. Census and swim lanes are appended only when `with_census`.
fn zjjrpd_emit_firemark(
    output: &mut vvco_Output,
    gallops: &Gallops,
    target: &str,
    remaining: bool,
    with_census: bool,
    gazette: &mut jjrz_Gazette,
    added_paddocks: &mut std::collections::HashSet<String>,
    added_paces: &mut std::collections::HashSet<String>,
    hark: Option<&str>,
) -> Result<(), String> {
    let cn = JJRPD_CMD_NAME_SHOW;
    let firemark = Firemark::jjrf_parse(target).map_err(|e| format!("{}: error: {}", cn, e))?;
    let heat_key = firemark.jjrf_display();
    let heat = gallops.heats.get(&heat_key)
        .ok_or_else(|| format!("{}: error: Heat '{}' not found", cn, heat_key))?;

    // The listed set: every pace in heat order, or only the open ones under
    // --remaining. One row shape either way, so the table is built once.
    let mut listed: Vec<(&String, &crate::jjrg_gallops::jjrg_Tack)> = Vec::new();
    let mut complete_count = 0;
    let mut abandoned_count = 0;
    let mut rough_count = 0;
    let mut bridled_count = 0;

    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                match tack.state {
                    PaceState::Complete => complete_count += 1,
                    PaceState::Abandoned => abandoned_count += 1,
                    // Both open states are remaining; bridled is a distinct
                    // open state, never folded into rough.
                    PaceState::Bridled => bridled_count += 1,
                    PaceState::Rough => rough_count += 1,
                }
                if remaining && tack.state.jjrg_is_resolved() {
                    continue;
                }
                listed.push((coronet_key, tack));
            }
        }
    }

    if remaining {
        let status_str = match heat.status {
            HeatStatus::Racing => "racing",
            HeatStatus::Stabled => "stabled",
            HeatStatus::Retired => "retired",
        };
        let remaining_count = rough_count + bridled_count;
        vvco_out!(output, "Heat: {} ({}) [{}]", heat.silks, heat_key, status_str);
        vvco_out!(output, "Progress: {} complete | {} abandoned | {} remaining ({} rough, {} bridled)",
            complete_count, abandoned_count, remaining_count, rough_count, bridled_count);
        vvco_out!(output, "");
    }

    if !listed.is_empty() {
        let mut table = jjrp_Table::jjrp_new(vec![
            jjrp_Column::new("No", jjrp_Align::Right),
            jjrp_Column::new("State", jjrp_Align::Left),
            jjrp_Column::new("Pace", jjrp_Align::Left),
            jjrp_Column::new("₢Coronet", jjrp_Align::Left),
        ]);

        // Rows built once: the State cell carries the tier of a bridled pace.
        let rows: Vec<[String; 4]> = listed.iter().enumerate().map(|(idx, (coronet_key, tack))| {
            [
                (idx + 1).to_string(),
                tack.jjrg_state_label(),
                tack.silks.clone(),
                (*coronet_key).clone(),
            ]
        }).collect();

        for row in &rows {
            table.jjrp_measure(&[&row[0], &row[1], &row[2], &row[3]]);
        }
        table.jjrp_write_header(output);
        table.jjrp_write_separator(output);
        for row in &rows {
            table.jjrp_write_row(output, &[&row[0], &row[1], &row[2], &row[3]]);
        }
        if remaining {
            vvco_out!(output, "");
        }
    }

    // Next-up callout, remaining view only — the first open pace in heat order.
    if remaining {
        if let Some((coronet_key, tack)) = listed.first() {
            vvco_out!(output, "Next: {} ({}) [{}]", tack.silks, coronet_key, tack.jjrg_state_label());
            vvco_out!(output, "");
            vvco_out!(output, "Recommended: mount {}", heat_key);
        }
    }

    // The census and swim lanes assume a single heat — render only for
    // a lone firemark target.
    if with_census {
        jjrpd_write_file_census(output, &firemark, heat, JJRPD_CENSUS_CROSS_PACE);
        jjrpd_write_commit_swimlanes(output, &firemark, heat);
    }

    // Always populate the gazette: paddock once, then each pace docket
    // (remaining-filtered to match the table).
    zjjrpd_gazette_paddock_once(gazette, added_paddocks, &firemark, hark);
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

/// Census floor for a planning reader (groom, parade): a file only earns a row
/// once two paces have touched it, because a lone-pace file poses no collision
/// question. Roughly half the rows on a wide heat, and none of the signal.
pub const JJRPD_CENSUS_CROSS_PACE: usize = 2;

/// Census floor for an archive reader (the retire trophy): every touched file,
/// because a retired heat's record is the last account of what it moved.
pub const JJRPD_CENSUS_EVERY_FILE: usize = 1;

/// Format the heat-wide file-touch census as a String: one row per file, naming
/// the paces whose commits touched it.
///
/// Sparse by construction — a wide heat's touch matrix runs a few percent dense,
/// so a per-pace column grid spends most of its width on absence. Naming the
/// paces present costs less than drawing the ones that are not, and spares the
/// reader a positional decode it performs unreliably.
///
/// `min_paces` is the floor a file must meet to earn a row (see the two
/// constants above).
pub fn jjrpd_format_file_census(firemark: &Firemark, heat: &Heat, min_paces: usize) -> Result<String, String> {
    let touches = jjrq_file_touches_for_heat(firemark.jjrf_as_str())?;

    // Paces with commits, in heat order, keyed by their coronet's terminal char.
    let mut pace_marks: Vec<(char, String, String)> = Vec::new();
    for coronet_key in &heat.order {
        if !touches.coronets_with_commits.contains(coronet_key) {
            continue;
        }
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                let raw = coronet_key.strip_prefix('₢').unwrap_or(coronet_key);
                if let Some(ch) = raw.chars().last() {
                    pace_marks.push((ch, coronet_key.clone(), tack.silks.clone()));
                }
            }
        }
    }

    let mut mark_of: BTreeMap<String, char> = BTreeMap::new();
    for (ch, coronet_display, _) in &pace_marks {
        mark_of.insert(coronet_display.clone(), *ch);
    }

    // Invert the query result: file -> the paces that touched it, in heat order.
    let mut file_paces: BTreeMap<String, Vec<char>> = BTreeMap::new();
    for (ch, coronet_display, _) in &pace_marks {
        if let Some(files) = touches.pace_files.get(coronet_display) {
            for filename in files {
                file_paces.entry(filename.clone()).or_default().push(*ch);
            }
        }
    }

    // Leading blank, as the swim lanes below do — the census must not butt
    // against whatever section precedes it.
    let mut output = String::new();
    writeln!(output).unwrap();

    if file_paces.is_empty() {
        writeln!(output, "File touches: (no work file changes)").unwrap();
        return Ok(output);
    }

    let total_files = file_paces.len();
    let mut rows: Vec<(String, Vec<char>)> = file_paces
        .into_iter()
        .filter(|(_, marks)| marks.len() >= min_paces)
        .collect();

    // Busiest files first — the ones most likely to collide with the next pace.
    rows.sort_by(|(file_a, marks_a), (file_b, marks_b)| {
        marks_b.len().cmp(&marks_a.len()).then_with(|| file_a.cmp(file_b))
    });

    if rows.is_empty() {
        writeln!(output, "File touches: (no file touched by {} or more paces; {} touched by one)",
            min_paces, total_files).unwrap();
        return Ok(output);
    }

    writeln!(output, "File touches (file: the paces whose commits touched it):").unwrap();
    if rows.len() < total_files {
        writeln!(output, "  ({} of {} files; those touched by a single pace are omitted)",
            rows.len(), total_files).unwrap();
    }
    writeln!(output).unwrap();

    // Legend: the mark, the coronet it stands for, and what that pace is.
    for (ch, coronet_display, silks) in &pace_marks {
        writeln!(output, "  {}  {}  {}", ch, coronet_display, silks).unwrap();
    }
    writeln!(output).unwrap();

    let width = rows.iter().map(|(file, _)| file.len()).max().unwrap_or(0);
    for (file, marks) in &rows {
        let marks_str: String = marks.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ");
        writeln!(output, "  {:width$}  {}", file, marks_str, width = width).unwrap();
    }

    Ok(output)
}

/// Write the heat-wide file-touch census to output.
pub(crate) fn jjrpd_write_file_census(output: &mut vvco_Output, firemark: &Firemark, heat: &Heat, min_paces: usize) {
    let cn = JJRPD_CMD_NAME_SHOW;
    match jjrpd_format_file_census(firemark, heat, min_paces) {
        Ok(text) => {
            for line in text.lines() {
                vvco_out!(output, "{}", line);
            }
        }
        Err(e) => vvco_err!(output, "{}: error getting file touches: {}", cn, e),
    }
}

/// Format the mounted pace's file digest: the files this pace's commits touched,
/// each naming the other paces that also touched it.
///
/// Scoped to the one pace a mount is about. The heat-wide census answers a
/// planning question a mount is not asking, and a pace with no commits yet — the
/// ordinary case at mount — has no digest at all, so nothing is rendered.
/// Self-contained by design: the collision names are spelled out inline, so the
/// reader decodes no legend.
pub fn jjrpd_format_pace_digest(firemark: &Firemark, heat: &Heat, coronet_display: &str) -> Result<String, String> {
    let touches = jjrq_file_touches_for_heat(firemark.jjrf_as_str())?;

    let mounted_files = match touches.pace_files.get(coronet_display) {
        Some(files) if !files.is_empty() => files,
        _ => return Ok(String::new()),
    };

    // For each file the mounted pace touched, the other paces that touched it too.
    let mut collisions: BTreeMap<&String, Vec<String>> = BTreeMap::new();
    for filename in mounted_files {
        collisions.insert(filename, Vec::new());
    }
    for coronet_key in &heat.order {
        if coronet_key == coronet_display {
            continue;
        }
        let Some(files) = touches.pace_files.get(coronet_key) else { continue };
        let silks = heat.paces.get(coronet_key)
            .and_then(|pace| pace.tacks.first())
            .map(|tack| tack.silks.as_str())
            .unwrap_or("unknown");
        for filename in files {
            if let Some(others) = collisions.get_mut(filename) {
                others.push(format!("{} {}", coronet_key, silks));
            }
        }
    }

    let mut output = String::new();
    writeln!(output, "Work-files ({} — files this pace's commits touched):", coronet_display).unwrap();
    writeln!(output).unwrap();

    let width = mounted_files.iter().map(|file| file.len()).max().unwrap_or(0);
    for (file, others) in &collisions {
        if others.is_empty() {
            writeln!(output, "  {}", file).unwrap();
        } else {
            writeln!(output, "  {:width$}  also: {}", file, others.join(", "), width = width).unwrap();
        }
    }
    writeln!(output).unwrap();

    Ok(output)
}

/// Write the mounted pace's file digest to output. Silent when the pace has no
/// commits — there is nothing to digest.
pub(crate) fn jjrpd_write_pace_digest(output: &mut vvco_Output, firemark: &Firemark, heat: &Heat, coronet_display: &str) {
    let cn = JJRPD_CMD_NAME_SHOW;
    match jjrpd_format_pace_digest(firemark, heat, coronet_display) {
        Ok(text) if text.is_empty() => {}
        Ok(text) => {
            for line in text.lines() {
                vvco_out!(output, "{}", line);
            }
        }
        Err(e) => vvco_err!(output, "{}: error getting file touches: {}", cn, e),
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
