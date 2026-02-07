// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Query utilities for Gallops JSON
//!
//! Contains shared utilities used by per-command modules.
//! The original jjrq_run_* command functions have been moved to dedicated modules:
//! - jjrmu_muster.rs (muster)
//! - jjrpd_parade.rs (parade)
//! - jjrsc_scout.rs (scout)
//! - jjrgs_get_spec.rs (get_spec)
//! - jjrgc_get_coronets.rs (get_coronets)
//! - jjrrt_retire.rs (retire)
//! - jjrsd_saddle.rs (saddle)

use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus};
use crate::jjrs_steeplechase::{jjrs_ReinArgs, jjrs_get_entries};
use regex::Regex;
use serde::Serialize;
use std::process::Command;

/// Path prefix for JJ infrastructure files (excluded from work file queries)
const JJRQ_INFRA_PREFIX: &str = ".claude/jjm/";

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
// File-touch queries (work files touched by pace commits)
// ============================================================================

/// Get files changed by a single commit via git diff-tree
pub(crate) fn zjjrq_files_for_commit(sha: &str) -> Vec<String> {
    let output = Command::new("git")
        .args(["diff-tree", "--no-commit-id", "--name-only", "-r", sha])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.to_string())
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Extract bare filename from a path (last component)
pub(crate) fn zjjrq_bare_filename(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

/// Test whether a file path is JJ infrastructure (excluded from work file results)
pub(crate) fn zjjrq_is_infra_file(path: &str) -> bool {
    path.starts_with(JJRQ_INFRA_PREFIX)
}

/// Get sorted list of work files touched by a pace's commits.
///
/// Queries steeplechase entries for the heat, filters by coronet,
/// runs `git diff-tree` per commit, and returns unique bare filenames
/// with JJ infrastructure files excluded.
pub fn jjrq_files_for_pace(firemark_raw: &str, coronet_display: &str) -> Result<Vec<String>, String> {
    let rein_args = jjrs_ReinArgs {
        firemark: firemark_raw.to_string(),
        limit: 10000,
    };

    let entries = jjrs_get_entries(&rein_args)?;

    let mut files = std::collections::BTreeSet::new();
    for entry in &entries {
        let dominated = match &entry.coronet {
            Some(c) => c == coronet_display,
            None => false,
        };
        if !dominated {
            continue;
        }
        for file_path in zjjrq_files_for_commit(&entry.commit) {
            if !zjjrq_is_infra_file(&file_path) {
                files.insert(zjjrq_bare_filename(&file_path));
            }
        }
    }

    Ok(files.into_iter().collect())
}

/// Get all steeplechase entries for a heat with file touch data per commit.
///
/// Returns entries plus a mapping of coronet -> set of bare work filenames.
/// Also returns heat-level (no coronet) work files separately.
/// Used by the file-touch bitmap visualization.
pub fn jjrq_file_touches_for_heat(firemark_raw: &str) -> Result<jjrq_HeatFileTouches, String> {
    let rein_args = jjrs_ReinArgs {
        firemark: firemark_raw.to_string(),
        limit: 10000,
    };

    let entries = jjrs_get_entries(&rein_args)?;

    let mut pace_files: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> = std::collections::BTreeMap::new();
    let mut heat_files: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut coronets_seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    for entry in &entries {
        let commit_files: Vec<String> = zjjrq_files_for_commit(&entry.commit)
            .into_iter()
            .filter(|p| !zjjrq_is_infra_file(p))
            .map(|p| zjjrq_bare_filename(&p))
            .collect();

        if let Some(ref coronet) = entry.coronet {
            coronets_seen.insert(coronet.clone());
            let set = pace_files.entry(coronet.clone()).or_default();
            for f in commit_files {
                set.insert(f);
            }
        } else {
            for f in commit_files {
                heat_files.insert(f);
            }
        }
    }

    Ok(jjrq_HeatFileTouches {
        pace_files,
        heat_files,
        coronets_with_commits: coronets_seen,
    })
}

/// Result of querying file touches for an entire heat
pub struct jjrq_HeatFileTouches {
    /// Coronet display string -> set of bare work filenames
    pub pace_files: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    /// Work files from heat-level (no coronet) commits
    pub heat_files: std::collections::BTreeSet<String>,
    /// All coronets that have at least one commit
    pub coronets_with_commits: std::collections::BTreeSet<String>,
}

// ============================================================================
// Retire Output Types (used by jjrrt_retire.rs and tests)
// ============================================================================

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
