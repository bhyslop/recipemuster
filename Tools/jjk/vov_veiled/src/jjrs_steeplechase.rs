// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Steeplechase Operations - git history parsing for JJ session tracking
//!
//! Implements jjx_rein: parse git history for steeplechase entries belonging to a Heat.
//! Steeplechase entries are stored in git commit messages, not the Gallops JSON.
//!
//! New commit format: `jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message`
//! - HALLMARK: Version identifier (NNNN or NNNN-xxxxxxx)
//! - IDENTITY: ₢CORONET for pace-level, ₣FIREMARK for heat-level
//! - ACTION: Single letter code (optional for standard notch)

use serde::Serialize;
use std::process::Command;

use crate::jjrf_favor::{jjrf_Firemark as Firemark, JJRF_FIREMARK_PREFIX as FIREMARK_PREFIX, JJRF_CORONET_PREFIX as CORONET_PREFIX};

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

    /// Hallmark version identifier (NNNN or NNNN-xxxxxxx)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hallmark: Option<String>,

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

/// Parse a new-format commit: jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message
/// Filters by firemark identity, not by brand. Brand is parsed but not matched.
pub fn zjjrs_parse_new_format(subject: &str, firemark_raw: &str) -> Option<jjrs_SteeplechaseEntry> {
    // Expected format: jjb:BRAND:HALLMARK:IDENTITY[:ACTION]: message
    // We match any brand - filtering is by identity only
    if !subject.starts_with("jjb:") {
        return None;
    }

    // Skip past "jjb:" and find the brand (next colon-delimited segment)
    let after_jjb = &subject[4..]; // skip "jjb:"
    let brand_end = after_jjb.find(':')?;
    let after_brand = &after_jjb[brand_end + 1..];

    // Parse hallmark or identity
    // If next segment starts with ₢ or ₣, it's old format (no hallmark)
    // Otherwise, parse hallmark then identity
    let (hallmark, after_hallmark) = if after_brand.starts_with(CORONET_PREFIX) || after_brand.starts_with(FIREMARK_PREFIX) {
        // Old format: no hallmark
        (None, after_brand)
    } else {
        // New format: parse hallmark
        let hallmark_end = after_brand.find(':')?;
        let hallmark_str = &after_brand[..hallmark_end];
        (Some(hallmark_str.to_string()), &after_brand[hallmark_end + 1..])
    };

    // Parse identity (₢CORONET or ₣FIREMARK)
    let (identity, rest) = if after_hallmark.starts_with(CORONET_PREFIX) {
        // Coronet: ₢XXXXX (1 prefix char + 5 base64 chars = 6 chars total in UTF-8)
        let coronet_end = CORONET_PREFIX.len_utf8() + 5;
        if after_hallmark.len() < coronet_end {
            return None;
        }
        let coronet = &after_hallmark[..coronet_end];
        // Verify this coronet belongs to our heat (first 2 chars of base64 match firemark)
        let coronet_heat = &after_hallmark[CORONET_PREFIX.len_utf8()..CORONET_PREFIX.len_utf8() + 2];
        if coronet_heat != firemark_raw {
            return None;
        }
        (Some(coronet.to_string()), &after_hallmark[coronet_end..])
    } else if after_hallmark.starts_with(FIREMARK_PREFIX) {
        // Firemark: ₣XX (1 prefix char + 2 base64 chars)
        let firemark_end = FIREMARK_PREFIX.len_utf8() + 2;
        if after_hallmark.len() < firemark_end {
            return None;
        }
        // Verify this is our firemark
        let fm = &after_hallmark[FIREMARK_PREFIX.len_utf8()..firemark_end];
        if fm != firemark_raw {
            return None;
        }
        (None, &after_hallmark[firemark_end..])
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
        hallmark,
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
    // Pattern matches both heat-level (₣XX) and pace-level (₢XX...) entries
    // Filter by identity, not brand: jjb:*:₣XX or jjb:*:₢XX
    let grep_pattern = format!(
        "^jjb:[^:]+:({}{}|{}{})",
        FIREMARK_PREFIX, firemark_raw,
        CORONET_PREFIX, firemark_raw
    );

    let output = Command::new("git")
        .args([
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

/// Run jjx_rein command (CLI wrapper)
pub fn jjrs_run(args: jjrs_ReinArgs) -> i32 {
    match jjrs_get_entries(&args) {
        Ok(entries) => {
            match serde_json::to_string_pretty(&entries) {
                Ok(json) => {
                    println!("{}", json);
                    0
                }
                Err(e) => {
                    eprintln!("rein: error: failed to serialize JSON: {}", e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("rein: error: {}", e);
            1
        }
    }
}
