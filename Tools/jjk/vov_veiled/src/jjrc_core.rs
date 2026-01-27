// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK Core - shared infrastructure
//!
//! Common utilities used across JJK modules.

#![allow(non_camel_case_types)]

use std::path::PathBuf;
use chrono::Local;

/// Default path to the Gallops JSON file
pub const JJRC_DEFAULT_GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";

/// Get the default Gallops file path relative to repo root
pub fn jjrc_default_gallops_path() -> PathBuf {
    PathBuf::from(JJRC_DEFAULT_GALLOPS_PATH)
}

/// Generate timestamp in YYMMDD format
pub fn jjrc_timestamp_date() -> String {
    Local::now().format("%y%m%d").to_string()
}

/// Generate timestamp in YYMMDD-HHMM format
pub fn jjrc_timestamp_full() -> String {
    Local::now().format("%y%m%d-%H%M").to_string()
}

/// Get creation date from BUD_NOW_STAMP env var, or fall back to system clock
///
/// If BUD_NOW_STAMP is set (format: YYYYMMDD-HHMMSS-PID-RANDOM),
/// extract YYYYMMDD and convert to YYMMDD (drop century).
/// Otherwise, use jjrc_timestamp_date() for current system clock.
pub fn jjrc_timestamp_from_env() -> String {
    match std::env::var("BUD_NOW_STAMP") {
        Ok(stamp) => {
            // Expected format: YYYYMMDD-HHMMSS-PID-RANDOM
            // Extract YYYYMMDD and convert to YYMMDD by dropping first 2 digits
            if stamp.len() >= 8 {
                let yyyymmdd = &stamp[..8];
                // Drop century (first 2 chars) to get YYMMDD
                if let Ok(year) = yyyymmdd[..4].parse::<u32>() {
                    let yy = year % 100;
                    let mmdd = &yyyymmdd[4..];
                    return format!("{:02}{}", yy, mmdd);
                }
            }
            // Fall back to system clock if parsing fails
            jjrc_timestamp_date()
        }
        Err(_) => {
            // BUD_NOW_STAMP not set, use system clock
            jjrc_timestamp_date()
        }
    }
}

/// Check if a session probe is needed based on time gap from last commit
///
/// Returns true if:
/// - No timestamp provided (no commits in steeplechase), OR
/// - Gap from last_timestamp to now is > 1 hour
///
/// Timestamp format expected: "YYYY-MM-DD HH:MM"
pub fn jjrc_needs_session_probe(last_timestamp: Option<&str>) -> bool {
    use chrono::NaiveDateTime;

    let Some(ts) = last_timestamp else {
        // No commits in steeplechase - new session
        return true;
    };

    // Parse timestamp: "YYYY-MM-DD HH:MM" -> NaiveDateTime
    let parsed = NaiveDateTime::parse_from_str(&format!("{}:00", ts), "%Y-%m-%d %H:%M:%S");
    let Ok(last_time) = parsed else {
        // If we can't parse the timestamp, assume new session
        return true;
    };

    // Get current local time as NaiveDateTime
    let now = Local::now().naive_local();

    // Calculate gap in seconds
    let duration = now.signed_duration_since(last_time);
    let gap_seconds = duration.num_seconds();

    // Session gap threshold: 1 hour
    gap_seconds > 60 * 60
}
