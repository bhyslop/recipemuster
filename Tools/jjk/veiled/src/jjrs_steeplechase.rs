//! Steeplechase Operations - git history parsing for JJ session tracking
//!
//! Implements jjx_rein: parse git history for steeplechase entries belonging to a Heat.
//! Steeplechase entries are stored in git commit messages, not the Gallops JSON.
//!
//! New commit format: `jjb:BRAND:IDENTITY[:ACTION]: message`
//! - IDENTITY: ₢CORONET for pace-level, ₣FIREMARK for heat-level
//! - ACTION: Single letter code (optional for standard notch)

use serde::Serialize;
use std::process::Command;

use crate::jjrf_favor::{Firemark, FIREMARK_PREFIX, CORONET_PREFIX};

/// Arguments for jjx_rein command
#[derive(Debug)]
pub struct ReinArgs {
    /// Target Heat identity (Firemark, with or without prefix)
    pub firemark: String,

    /// Maximum entries to return
    pub limit: usize,
}

/// Steeplechase entry - parsed from git commit message
#[derive(Debug, Clone, Serialize)]
pub struct SteeplechaseEntry {
    /// Timestamp in "YYYY-MM-DD HH:MM" format
    pub timestamp: String,

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
fn parse_timestamp(git_timestamp: &str) -> String {
    let trimmed = git_timestamp.trim();
    if trimmed.len() >= 16 {
        trimmed[..16].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Parse a new-format commit: jjb:BRAND:IDENTITY[:ACTION]: message
/// Filters by firemark identity, not by brand. Brand is parsed but not matched.
fn parse_new_format(subject: &str, firemark_raw: &str) -> Option<SteeplechaseEntry> {
    // Expected format: jjb:BRAND:IDENTITY[:ACTION]: message
    // We match any brand - filtering is by identity only
    if !subject.starts_with("jjb:") {
        return None;
    }

    // Skip past "jjb:" and find the brand (next colon-delimited segment)
    let after_jjb = &subject[4..]; // skip "jjb:"
    let brand_end = after_jjb.find(':')?;
    let after_brand = &after_jjb[brand_end + 1..];

    // Parse identity (₢CORONET or ₣FIREMARK)
    let (identity, rest) = if after_brand.starts_with(CORONET_PREFIX) {
        // Coronet: ₢XXXXX (1 prefix char + 5 base64 chars = 6 chars total in UTF-8)
        let coronet_end = CORONET_PREFIX.len_utf8() + 5;
        if after_brand.len() < coronet_end {
            return None;
        }
        let coronet = &after_brand[..coronet_end];
        // Verify this coronet belongs to our heat (first 2 chars of base64 match firemark)
        let coronet_heat = &after_brand[CORONET_PREFIX.len_utf8()..CORONET_PREFIX.len_utf8() + 2];
        if coronet_heat != firemark_raw {
            return None;
        }
        (Some(coronet.to_string()), &after_brand[coronet_end..])
    } else if after_brand.starts_with(FIREMARK_PREFIX) {
        // Firemark: ₣XX (1 prefix char + 2 base64 chars)
        let firemark_end = FIREMARK_PREFIX.len_utf8() + 2;
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

    Some(SteeplechaseEntry {
        timestamp: String::new(), // filled by caller
        coronet: identity,
        action,
        subject: message,
    })
}

/// Parse a single git log line into a SteeplechaseEntry
/// Line format: "YYYY-MM-DD HH:MM:SS -ZZZZ<TAB>subject"
fn parse_log_line(line: &str, firemark_raw: &str) -> Option<SteeplechaseEntry> {
    let parts: Vec<&str> = line.splitn(2, '\t').collect();
    if parts.len() != 2 {
        return None;
    }

    let timestamp = parse_timestamp(parts[0]);
    let subject = parts[1];

    // Try parsing new format
    if let Some(mut entry) = parse_new_format(subject, firemark_raw) {
        entry.timestamp = timestamp;
        return Some(entry);
    }

    None
}

/// Get steeplechase entries for a heat (library function)
pub fn get_entries(args: &ReinArgs) -> Result<Vec<SteeplechaseEntry>, String> {
    let firemark = Firemark::parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;

    let firemark_raw = firemark.as_str();

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
            "--format=%ai\t%s",
        ])
        .output()
        .map_err(|e| format!("Failed to run git log: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git log failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let entries: Vec<SteeplechaseEntry> = stdout
        .lines()
        .filter_map(|line| parse_log_line(line, firemark_raw))
        .take(args.limit)
        .collect();

    Ok(entries)
}

/// Run jjx_rein command (CLI wrapper)
pub fn run(args: ReinArgs) -> i32 {
    match get_entries(&args) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(
            parse_timestamp("2024-01-15 14:30:00 -0800"),
            "2024-01-15 14:30"
        );
        assert_eq!(
            parse_timestamp("2024-12-31 23:59:59 +0000"),
            "2024-12-31 23:59"
        );
    }

    #[test]
    fn test_parse_new_format_standard_notch() {
        let subject = "jjb:RBM:₢ABAAA:n: Fix the bug";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
        assert_eq!(entry.action, Some("n".to_string()));
        assert_eq!(entry.subject, "Fix the bug");
    }

    #[test]
    fn test_parse_new_format_chalk_wrap() {
        let subject = "jjb:RBM:₢ABAAA:W: Completed the task";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
        assert_eq!(entry.action, Some("W".to_string()));
        assert_eq!(entry.subject, "Completed the task");
    }

    #[test]
    fn test_parse_new_format_chalk_approach() {
        let subject = "jjb:RBM:₢ABCDE:A: Starting work";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, Some("₢ABCDE".to_string()));
        assert_eq!(entry.action, Some("A".to_string()));
        assert_eq!(entry.subject, "Starting work");
    }

    #[test]
    fn test_parse_new_format_heat_level_nominate() {
        let subject = "jjb:RBM:₣AB:N: my-new-heat";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, None);
        assert_eq!(entry.action, Some("N".to_string()));
        assert_eq!(entry.subject, "my-new-heat");
    }

    #[test]
    fn test_parse_new_format_heat_level_slate() {
        let subject = "jjb:RBM:₣AB:S: new-pace-silks";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, None);
        assert_eq!(entry.action, Some("S".to_string()));
        assert_eq!(entry.subject, "new-pace-silks");
    }

    #[test]
    fn test_parse_new_format_heat_level_rail() {
        let subject = "jjb:RBM:₣AB:r: reordered";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, None);
        assert_eq!(entry.action, Some("r".to_string()));
        assert_eq!(entry.subject, "reordered");
    }

    #[test]
    fn test_parse_new_format_heat_level_retire() {
        let subject = "jjb:RBM:₣AB:R: my-heat-silks";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, None);
        assert_eq!(entry.action, Some("R".to_string()));
        assert_eq!(entry.subject, "my-heat-silks");
    }

    #[test]
    fn test_parse_new_format_heat_discussion() {
        let subject = "jjb:RBM:₣AB:d: Design discussion";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, None);
        assert_eq!(entry.action, Some("d".to_string()));
        assert_eq!(entry.subject, "Design discussion");
    }

    #[test]
    fn test_parse_new_format_any_brand() {
        // Should parse successfully with any brand - filtering is by identity
        let subject = "jjb:OTHER:₢ABAAA:n: Fix bug";
        let entry = parse_new_format(subject, "AB").unwrap();
        assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
        assert_eq!(entry.action, Some("n".to_string()));
        assert_eq!(entry.subject, "Fix bug");
    }

    #[test]
    fn test_parse_new_format_wrong_firemark() {
        // Different firemark in coronet - should NOT match
        let subject = "jjb:RBM:₢CDAAA:n: Fix bug";
        let result = parse_new_format(subject, "AB");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_new_format_wrong_heat_firemark() {
        let subject = "jjb:RBM:₣CD:N: some-heat";
        let result = parse_new_format(subject, "AB");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_log_line_new_format() {
        let line = "2024-01-15 14:30:00 -0800\tjjb:RBM:₢ABAAA:n: Fix bug";
        let entry = parse_log_line(line, "AB").unwrap();
        assert_eq!(entry.timestamp, "2024-01-15 14:30");
        assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
        assert_eq!(entry.action, Some("n".to_string()));
        assert_eq!(entry.subject, "Fix bug");
    }

    #[test]
    fn test_parse_log_line_new_format_with_action() {
        let line = "2024-01-15 14:30:00 -0800\tjjb:RBM:₢ABAAA:F: Autonomous execution";
        let entry = parse_log_line(line, "AB").unwrap();
        assert_eq!(entry.timestamp, "2024-01-15 14:30");
        assert_eq!(entry.coronet, Some("₢ABAAA".to_string()));
        assert_eq!(entry.action, Some("F".to_string()));
        assert_eq!(entry.subject, "Autonomous execution");
    }
}
