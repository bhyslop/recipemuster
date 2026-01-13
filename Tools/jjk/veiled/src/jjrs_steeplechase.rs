//! Steeplechase Operations - git history parsing for JJ session tracking
//!
//! Implements jjx_rein: parse git history for steeplechase entries belonging to a Heat.
//! Steeplechase entries are stored in git commit messages, not the Gallops JSON.

use serde::Serialize;
use std::process::Command;

use crate::jjrf_favor::{Firemark, FIREMARK_PREFIX};

/// Arguments for jjx_rein command
#[derive(Debug)]
pub struct ReinArgs {
    /// Target Heat identity (Firemark, with or without prefix)
    pub firemark: String,

    /// Repository brand identifier (from arcanum installation)
    pub brand: String,

    /// Maximum entries to return
    pub limit: usize,
}

/// Steeplechase entry - parsed from git commit message
#[derive(Debug, Clone, Serialize)]
pub struct SteeplechaseEntry {
    /// Timestamp in "YYYY-MM-DD HH:MM" format
    pub timestamp: String,

    /// Pace silks for standard commits, null for markers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pace_silks: Option<String>,

    /// Marker type for chalk entries, null for standard commits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Commit message or marker description
    pub subject: String,
}

/// Parse timestamp from git %ai format to "YYYY-MM-DD HH:MM"
/// Input format: "2024-01-15 14:30:00 -0800"
fn parse_timestamp(git_timestamp: &str) -> String {
    // Take first 16 chars: "YYYY-MM-DD HH:MM"
    let trimmed = git_timestamp.trim();
    if trimmed.len() >= 16 {
        trimmed[..16].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Parse a standard commit subject line
/// Format: [jj:BRAND][₣XX/pace-silks] message
fn parse_standard_commit(subject: &str, prefix_end: usize) -> Option<(String, String)> {
    // Subject after the prefix should be: /pace-silks] message
    let rest = &subject[prefix_end..];

    // Must start with /
    if !rest.starts_with('/') {
        return None;
    }

    // Find the closing bracket
    let close_bracket = rest.find(']')?;
    let pace_silks = rest[1..close_bracket].to_string();

    // Message is everything after "] "
    let message_start = close_bracket + 1;
    let message = rest[message_start..].trim_start().to_string();

    Some((pace_silks, message))
}

/// Parse a marker commit subject line
/// Format: [jj:BRAND][₣XX] MARKER: description
fn parse_marker_commit(subject: &str, prefix_end: usize) -> Option<(String, String)> {
    // Subject after the prefix should be: ] MARKER: description
    let rest = &subject[prefix_end..];

    // Must start with ]
    if !rest.starts_with(']') {
        return None;
    }

    // Skip "] " and look for marker type
    let after_bracket = rest[1..].trim_start();

    // Find the colon that separates MARKER from description
    let colon_pos = after_bracket.find(':')?;
    let marker = after_bracket[..colon_pos].to_string();
    let description = after_bracket[colon_pos + 1..].trim_start().to_string();

    // Validate marker type
    match marker.as_str() {
        "APPROACH" | "WRAP" | "FLY" | "DISCUSSION" => Some((marker, description)),
        _ => None,
    }
}

/// Parse a single git log line into a SteeplechaseEntry
/// Line format: "YYYY-MM-DD HH:MM:SS -ZZZZ<TAB>subject"
fn parse_log_line(line: &str, brand: &str, firemark_raw: &str) -> Option<SteeplechaseEntry> {
    // Split on tab
    let parts: Vec<&str> = line.splitn(2, '\t').collect();
    if parts.len() != 2 {
        return None;
    }

    let timestamp = parse_timestamp(parts[0]);
    let subject = parts[1];

    // Build the prefix pattern we're looking for: [jj:BRAND][₣XX
    let prefix_pattern = format!("[jj:{}][{}{}", brand, FIREMARK_PREFIX, firemark_raw);

    if !subject.starts_with(&prefix_pattern) {
        return None;
    }

    let prefix_end = prefix_pattern.len();

    // Try parsing as standard commit first (has pace-silks)
    if let Some((pace_silks, message)) = parse_standard_commit(subject, prefix_end) {
        return Some(SteeplechaseEntry {
            timestamp,
            pace_silks: Some(pace_silks),
            marker: None,
            subject: message,
        });
    }

    // Try parsing as marker commit
    if let Some((marker, description)) = parse_marker_commit(subject, prefix_end) {
        return Some(SteeplechaseEntry {
            timestamp,
            pace_silks: None,
            marker: Some(marker),
            subject: description,
        });
    }

    None
}

/// Run jjx_rein command
pub fn run(args: ReinArgs) -> i32 {
    // Parse and validate Firemark
    let firemark = match Firemark::parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("rein: error: {}", e);
            return 1;
        }
    };

    let firemark_raw = firemark.as_str();

    // Build the grep pattern for git log
    // Pattern: ^\[jj:BRAND\]\[₣XX
    let grep_pattern = format!(
        "^\\[jj:{}\\]\\[{}{}",
        args.brand, FIREMARK_PREFIX, firemark_raw
    );

    // Run git log with extended regexp
    let output = match Command::new("git")
        .args([
            "log",
            "--all",
            "--extended-regexp",
            &format!("--grep={}", grep_pattern),
            "--format=%ai\t%s",
        ])
        .output()
    {
        Ok(out) => out,
        Err(e) => {
            eprintln!("rein: error: failed to run git log: {}", e);
            return 1;
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("rein: error: git log failed: {}", stderr);
        return 1;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse each line
    let entries: Vec<SteeplechaseEntry> = stdout
        .lines()
        .filter_map(|line| parse_log_line(line, &args.brand, firemark_raw))
        .take(args.limit)
        .collect();

    // Output as JSON array
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
    fn test_parse_standard_commit() {
        let subject = "[jj:RBM][₣AB/my-pace] Fix the bug";
        let prefix_end = "[jj:RBM][₣AB".len();

        let result = parse_standard_commit(subject, prefix_end);
        assert!(result.is_some());
        let (pace_silks, message) = result.unwrap();
        assert_eq!(pace_silks, "my-pace");
        assert_eq!(message, "Fix the bug");
    }

    #[test]
    fn test_parse_marker_commit() {
        let subject = "[jj:RBM][₣AB] APPROACH: Proposed approach for pace";
        let prefix_end = "[jj:RBM][₣AB".len();

        let result = parse_marker_commit(subject, prefix_end);
        assert!(result.is_some());
        let (marker, description) = result.unwrap();
        assert_eq!(marker, "APPROACH");
        assert_eq!(description, "Proposed approach for pace");
    }

    #[test]
    fn test_parse_marker_commit_wrap() {
        let subject = "[jj:RBM][₣AB] WRAP: Completed implementation";
        let prefix_end = "[jj:RBM][₣AB".len();

        let result = parse_marker_commit(subject, prefix_end);
        assert!(result.is_some());
        let (marker, description) = result.unwrap();
        assert_eq!(marker, "WRAP");
        assert_eq!(description, "Completed implementation");
    }

    #[test]
    fn test_parse_marker_commit_invalid_marker() {
        let subject = "[jj:RBM][₣AB] INVALID: Some description";
        let prefix_end = "[jj:RBM][₣AB".len();

        let result = parse_marker_commit(subject, prefix_end);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_log_line_standard() {
        let line = "2024-01-15 14:30:00 -0800\t[jj:RBM][₣AB/my-pace] Fix bug";
        let result = parse_log_line(line, "RBM", "AB");

        assert!(result.is_some());
        let entry = result.unwrap();
        assert_eq!(entry.timestamp, "2024-01-15 14:30");
        assert_eq!(entry.pace_silks, Some("my-pace".to_string()));
        assert_eq!(entry.marker, None);
        assert_eq!(entry.subject, "Fix bug");
    }

    #[test]
    fn test_parse_log_line_marker() {
        let line = "2024-01-15 14:30:00 -0800\t[jj:RBM][₣AB] FLY: Starting autonomous execution";
        let result = parse_log_line(line, "RBM", "AB");

        assert!(result.is_some());
        let entry = result.unwrap();
        assert_eq!(entry.timestamp, "2024-01-15 14:30");
        assert_eq!(entry.pace_silks, None);
        assert_eq!(entry.marker, Some("FLY".to_string()));
        assert_eq!(entry.subject, "Starting autonomous execution");
    }

    #[test]
    fn test_parse_log_line_wrong_brand() {
        let line = "2024-01-15 14:30:00 -0800\t[jj:OTHER][₣AB/my-pace] Fix bug";
        let result = parse_log_line(line, "RBM", "AB");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_log_line_wrong_firemark() {
        let line = "2024-01-15 14:30:00 -0800\t[jj:RBM][₣CD/my-pace] Fix bug";
        let result = parse_log_line(line, "RBM", "AB");
        assert!(result.is_none());
    }
}
