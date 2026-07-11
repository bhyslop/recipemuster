// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Steeplechase Operations - git history parsing for JJ session tracking
//!
//! Implements jjx_rein: parse git history for steeplechase entries belonging to a Heat.
//! Steeplechase entries are stored in git commit messages, not the Gallops JSON.
//!
//! Commit format: `jjb:BRAND:IDENTITY:ACTION: message`
//! - BRAND: Version identifier (NNNN or NNNN-xxxxxxx)
//! - IDENTITY: ₢CORONET for pace-level, ₣FIREMARK for heat-level
//! - ACTION: Single letter code (required for all commits)

use serde::Serialize;

use crate::jjrf_favor::{jjrf_Firemark as Firemark, JJRF_FIREMARK_PREFIX as FIREMARK_PREFIX, JJRF_CORONET_PREFIX as CORONET_PREFIX, JJRF_FIREMARK_LEN, JJRF_CORONET_LEN};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug};

const JJRS_CMD_NAME_REIN: &str = "jjx_rein";

/// Characters of a commit subject the inline table shows before clipping —
/// git's own one-line subject bound, enough to recognize a commit and pick the
/// gazette stanza to read whole.
/// A jjb subject is a paragraph carried on one line, so an unclipped table
/// grows with subject length as well as row count — a several-hundred-commit
/// heat renders hundreds of kilobytes and overruns the tool-result channel.
/// The untruncated subjects ride the gazette instead.
const JJRS_SUBJECT_CAP: usize = 72;

/// Rows the inline table shows, newest first, however many entries were asked
/// for. Clipping alone leaves the table linear in commit count — it merely
/// postpones the overrun as a heat grows — so the row ceiling is what makes the
/// inline channel bounded outright. The gazette carries every requested entry,
/// so the ceiling costs no history: `limit` sizes the gazette, and the table is
/// the index into it.
const JJRS_TABLE_ROWS: usize = 50;

/// Arguments for jjx_rein command
#[derive(Debug)]
pub struct jjrs_ReinArgs {
    /// Target Heat identity (Firemark, with or without prefix)
    pub firemark: String,

    /// Maximum entries to return
    pub limit: usize,
}

/// Steeplechase entry - parsed from git commit message
#[derive(Debug, Clone, Serialize, Default)]
pub struct jjrs_SteeplechaseEntry {
    /// Timestamp in "YYYY-MM-DD HH:MM" format
    pub timestamp: String,

    /// Abbreviated git commit SHA
    pub commit: String,

    /// Brand version identifier (NNNN or NNNN-xxxxxxx)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Coronet for pace-level entries, None for heat-level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coronet: Option<String>,

    /// Action code (single letter), 'n' for standard notch commits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Commit message or marker description
    pub subject: String,
}

/// Parse timestamp from git %ai format to "YYYY-MM-DD HH:MM"
/// Input format: "2024-01-15 14:30:00 -0800"
pub(crate) fn zjjrs_parse_timestamp(git_timestamp: &str) -> String {
    let trimmed = git_timestamp.trim();
    if trimmed.len() >= 16 {
        trimmed[..16].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Parse commit format: jjb:BRAND:IDENTITY:ACTION: message
/// Filters by firemark identity.
pub fn zjjrs_parse_new_format(subject: &str, firemark_raw: &str) -> Option<jjrs_SteeplechaseEntry> {
    // Expected format: jjb:BRAND:IDENTITY:ACTION: message
    if !subject.starts_with("jjb:") {
        return None;
    }

    // Skip past "jjb:" to get brand
    let after_jjb = &subject[4..]; // skip "jjb:"

    // Parse brand (required) - find next colon
    let brand_end = after_jjb.find(':')?;
    let brand_str = &after_jjb[..brand_end];
    let brand = Some(brand_str.to_string());
    let after_brand = &after_jjb[brand_end + 1..];

    // Parse identity (₢CORONET or ₣FIREMARK)
    let (identity, rest) = if after_brand.starts_with(CORONET_PREFIX) {
        // Coronet: ₢XXXXX (1 prefix char + 5 base64 chars = 6 chars total in UTF-8)
        let coronet_end = CORONET_PREFIX.len_utf8() + JJRF_CORONET_LEN;
        if after_brand.len() < coronet_end {
            return None;
        }
        let coronet = &after_brand[..coronet_end];
        // Verify this coronet belongs to our heat (first 2 chars of base64 match firemark)
        let coronet_heat = &after_brand[CORONET_PREFIX.len_utf8()..CORONET_PREFIX.len_utf8() + JJRF_FIREMARK_LEN];
        if coronet_heat != firemark_raw {
            return None;
        }
        (Some(coronet.to_string()), &after_brand[coronet_end..])
    } else if after_brand.starts_with(FIREMARK_PREFIX) {
        // Firemark: ₣XX (1 prefix char + 2 base64 chars)
        let firemark_end = FIREMARK_PREFIX.len_utf8() + JJRF_FIREMARK_LEN;
        if after_brand.len() < firemark_end {
            return None;
        }
        // Verify this is our firemark
        let fm = &after_brand[FIREMARK_PREFIX.len_utf8()..firemark_end];
        if fm != firemark_raw {
            return None;
        }
        (None, &after_brand[firemark_end..])
    } else {
        return None;
    };

    // Now parse: [:ACTION]: message
    // rest should start with either ": " (no action) or ":X: " (with action)
    if !rest.starts_with(':') {
        return None;
    }

    let after_colon = &rest[1..];

    let (action, message) = if after_colon.starts_with(' ') {
        // No action code, just ": message"
        (None, after_colon[1..].to_string())
    } else {
        // Has action code: "X: message"
        // Find the next colon
        if let Some(colon_pos) = after_colon.find(':') {
            let action_code = &after_colon[..colon_pos];
            let msg = after_colon[colon_pos + 1..].trim_start().to_string();
            (Some(action_code.to_string()), msg)
        } else {
            return None;
        }
    };

    Some(jjrs_SteeplechaseEntry {
        timestamp: String::new(), // filled by caller
        commit: String::new(),    // filled by caller
        brand,
        coronet: identity,
        action,
        subject: message,
    })
}

/// Parse a single git log line into a SteeplechaseEntry
/// Line format: "YYYY-MM-DD HH:MM:SS -ZZZZ<TAB>abbrev_commit<TAB>subject"
pub fn zjjrs_parse_log_line(line: &str, firemark_raw: &str) -> Option<jjrs_SteeplechaseEntry> {
    let parts: Vec<&str> = line.splitn(3, '\t').collect();
    if parts.len() != 3 {
        return None;
    }

    let timestamp = zjjrs_parse_timestamp(parts[0]);
    let commit = parts[1].to_string();
    let subject = parts[2];

    // Try parsing new format
    if let Some(mut entry) = zjjrs_parse_new_format(subject, firemark_raw) {
        entry.timestamp = timestamp;
        entry.commit = commit;
        return Some(entry);
    }

    None
}

/// Get steeplechase entries for a heat (library function)
pub fn jjrs_get_entries(args: &jjrs_ReinArgs) -> Result<Vec<jjrs_SteeplechaseEntry>, String> {
    let firemark = Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;

    let firemark_raw = firemark.jjrf_as_str();

    // Build the grep pattern for git log
    // Format: jjb:BRAND:IDENTITY:ACTION: message
    // Pattern matches both heat-level (₣XX) and pace-level (₢XX...) entries
    let grep_pattern = format!(
        "^jjb:[^:]+:({}{}|{}{})",
        FIREMARK_PREFIX, firemark_raw,
        CORONET_PREFIX, firemark_raw
    );

    let output = vvc::vvce_git_command(&[
            "log",
            "--all",
            "--extended-regexp",
            &format!("--grep={}", grep_pattern),
            "--format=%ai\t%h\t%s",
        ])
        .output()
        .map_err(|e| format!("Failed to run git log: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git log failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let entries: Vec<jjrs_SteeplechaseEntry> = stdout
        .lines()
        .filter_map(|line| zjjrs_parse_log_line(line, firemark_raw))
        .take(args.limit)
        .collect();

    Ok(entries)
}

/// Action cell for an entry: the bracketed code, empty for an entry carrying none
fn zjjrs_act_cell(entry: &jjrs_SteeplechaseEntry) -> String {
    entry.action.as_ref().map(|a| format!("[{}]", a)).unwrap_or_default()
}

/// Affiliation cell for an entry: the coronet for a pace-level entry,
/// the heat firemark for a heat-level one.
fn zjjrs_affil_cell(entry: &jjrs_SteeplechaseEntry, heat_key: &str) -> String {
    entry.coronet.clone().unwrap_or_else(|| heat_key.to_string())
}

/// Compose the gazette body: every entry, subject untruncated, newest first.
/// Each entry is a stanza — a header line carrying the same fields as the
/// inline row, then the whole subject beneath it.
pub(crate) fn zjjrs_gazette_body(entries: &[jjrs_SteeplechaseEntry], heat_key: &str) -> String {
    entries.iter()
        .map(|entry| format!(
            "{}  {}  {}  {}\n{}",
            entry.timestamp,
            entry.commit,
            zjjrs_act_cell(entry),
            zjjrs_affil_cell(entry, heat_key),
            entry.subject,
        ))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Run jjx_rein command (CLI wrapper)
///
/// Two channels, the shape jjx_show already carries: the inline result is the
/// terse table, its Subject column clipped to a bounded line; the gazette
/// carries every subject whole.
pub fn jjrs_run(
    args: jjrs_ReinArgs,
    output: &mut vvc::vvco_Output,
    gazette: &mut jjrz_Gazette,
) -> i32 {
    let cn = JJRS_CMD_NAME_REIN;

    let heat_key = match Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm.jjrf_display(),
        Err(e) => {
            vvc::vvco_err!(output, "{}: error: Invalid firemark: {}", cn, e);
            return 1;
        }
    };

    let entries = match jjrs_get_entries(&args) {
        Ok(entries) => entries,
        Err(e) => {
            vvc::vvco_err!(output, "{}: error: {}", cn, e);
            return 1;
        }
    };

    // The table shows the newest rows only; the gazette below carries them all.
    let tabled = entries.iter().take(JJRS_TABLE_ROWS).collect::<Vec<_>>();

    // Set up table with column definitions
    let mut table = jjrp_Table::jjrp_new(vec![
        jjrp_Column::new("Timestamp", jjrp_Align::Left),
        jjrp_Column::new("Commit", jjrp_Align::Left),
        jjrp_Column::new("Act", jjrp_Align::Left),
        jjrp_Column::new("Affil", jjrp_Align::Left),
        jjrp_Column::with_cap("Subject", JJRS_SUBJECT_CAP, jjrp_Align::Left),
    ]);

    // Measure all rows to compute column widths
    for entry in &tabled {
        table.jjrp_measure(&[
            &entry.timestamp,
            &entry.commit,
            &zjjrs_act_cell(entry),
            &zjjrs_affil_cell(entry, &heat_key),
            &entry.subject,
        ]);
    }

    // Write header and separator
    table.jjrp_write_header(output);
    table.jjrp_write_separator(output);

    // Write data rows
    for entry in &tabled {
        table.jjrp_write_row(output, &[
            &entry.timestamp,
            &entry.commit,
            &zjjrs_act_cell(entry),
            &zjjrs_affil_cell(entry, &heat_key),
            &entry.subject,
        ]);
    }

    // Say what was withheld, and where it went. A silent ceiling reads as
    // "that is the whole history".
    if entries.len() > tabled.len() {
        vvc::vvco_out!(
            output,
            "(showing newest {} of {} entries — every entry, subjects whole, in gazette_out.md)",
            tabled.len(),
            entries.len(),
        );
    }

    // The gazette is the whole-subject channel, not a convenience — a clipped
    // inline row has no other reading. It is always populated, even when the
    // heat has no entries yet.
    let body = zjjrs_gazette_body(&entries, &heat_key);
    if let Err(e) = gazette.jjrz_add(jjrz_Slug::Steeplechase, &heat_key, &body) {
        vvc::vvco_err!(output, "{}: error: gazette add failed: {}", cn, e);
        return 1;
    }

    0
}
