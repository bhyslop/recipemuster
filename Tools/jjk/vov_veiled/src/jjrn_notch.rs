// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Notch and Chalk - Steeplechase commit formatting
//!
//! Provides message formatting for JJ-aware git commits that record session history:
//! - jjx_notch: Standard commit with heat/pace context prefix
//! - jjx_chalk: Empty commit marking a steeplechase event (pace-level)
//! - Heat-level commits: nominate, slate, rail, tally, draft, retire
//!
//! Commit format: `jjb:HALLMARK:IDENTITY:ACTION: message`
//! - HALLMARK: Version identifier (NNNN or NNNN-xxxxxxx)
//! - IDENTITY: ₢CORONET for pace-level, ₣FIREMARK for heat-level
//! - ACTION: Single letter code (required for all commits)
//!
//! The actual commit execution is handled by vorc_commit in the vok crate.
//! This module exports formatting functions that the CLI handlers use.

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark, JJRF_CORONET_PREFIX as CORONET_PREFIX, JJRF_FIREMARK_PREFIX as FIREMARK_PREFIX};
use std::fs;
use std::process::Command;

/// Commit message prefix
pub const JJRN_COMMIT_PREFIX: &str = "jjb";

/// Get hallmark for commit message versioning
///
/// Source logic:
/// 1. Try `.vvk/vvbf_brand.json` → if exists, use `vvbh_hallmark` (4 digits)
/// 2. If missing (Kit Forge) → read `Tools/vok/vov_veiled/vovr_registry.json`,
///    find max hallmark, get `git rev-parse --short HEAD`, format as `{hallmark}-{commit}`
fn zjjrn_get_hallmark() -> String {
    // Try reading brand file
    if let Ok(brand_content) = fs::read_to_string(".vvk/vvbf_brand.json") {
        if let Ok(brand_json) = serde_json::from_str::<serde_json::Value>(&brand_content) {
            if let Some(hallmark) = brand_json.get("vvbh_hallmark") {
                if let Some(hallmark_str) = hallmark.as_str() {
                    return hallmark_str.to_string();
                }
            }
        }
    }

    // Fallback: Kit Forge mode (read registry + git HEAD)
    let mut max_hallmark = 0u32;
    if let Ok(registry_content) = fs::read_to_string("Tools/vok/vov_veiled/vovr_registry.json") {
        if let Ok(registry_json) = serde_json::from_str::<serde_json::Value>(&registry_content) {
            if let Some(hallmarks) = registry_json.get("hallmarks").and_then(|v| v.as_object()) {
                for key in hallmarks.keys() {
                    if let Ok(num) = key.parse::<u32>() {
                        max_hallmark = max_hallmark.max(num);
                    }
                }
            }
        }
    }

    // Get short commit hash
    let git_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    let commit_hash = if let Ok(output) = git_output {
        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "0000000".to_string()
        }
    } else {
        "0000000".to_string()
    };

    format!("{:04}-{}", max_hallmark, commit_hash)
}

/// Pace-level chalk markers (single-letter codes)
/// - A = APPROACH: proposed approach before work begins
/// - W = WRAP: pace completion summary
/// - F = FLY: autonomous execution began (bridled pace)
/// - d = discussion: significant decision (lowercase, can be heat-level too)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrn_ChalkMarker {
    Approach,
    Wrap,
    Fly,
    Discussion,
}

impl jjrn_ChalkMarker {
    /// Parse marker from string (accepts both single letter and full word)
    pub fn jjrn_parse(s: &str) -> Result<Self, String> {
        match s {
            "A" | "a" | "APPROACH" | "approach" | "Approach" => Ok(jjrn_ChalkMarker::Approach),
            "W" | "w" | "WRAP" | "wrap" | "Wrap" => Ok(jjrn_ChalkMarker::Wrap),
            "F" | "f" | "FLY" | "fly" | "Fly" => Ok(jjrn_ChalkMarker::Fly),
            "d" | "D" | "DISCUSSION" | "discussion" | "Discussion" => Ok(jjrn_ChalkMarker::Discussion),
            _ => Err(format!(
                "Invalid marker type '{}'. Valid: A(pproach), W(rap), F(ly), d(iscussion)",
                s
            )),
        }
    }

    /// Get single-letter code for commit message
    pub fn jjrn_code(&self) -> char {
        match self {
            jjrn_ChalkMarker::Approach => 'A',
            jjrn_ChalkMarker::Wrap => 'W',
            jjrn_ChalkMarker::Fly => 'F',
            jjrn_ChalkMarker::Discussion => 'd',
        }
    }

    /// Get full display string for the marker (for user-facing output)
    pub fn jjrn_as_str(&self) -> &'static str {
        match self {
            jjrn_ChalkMarker::Approach => "APPROACH",
            jjrn_ChalkMarker::Wrap => "WRAP",
            jjrn_ChalkMarker::Fly => "FLY",
            jjrn_ChalkMarker::Discussion => "discussion",
        }
    }

    /// Returns true if this marker type requires a pace to be specified
    pub fn jjrn_requires_pace(&self) -> bool {
        match self {
            jjrn_ChalkMarker::Approach | jjrn_ChalkMarker::Wrap | jjrn_ChalkMarker::Fly => true,
            jjrn_ChalkMarker::Discussion => false,
        }
    }
}

/// Heat-level action codes (single-letter codes)
/// - N = Nominate: create new heat
/// - S = Slate: add new pace
/// - r = rail: reorder paces (lowercase)
/// - T = Tally: add tack to pace
/// - D = Draft: move pace between heats (uppercase, rare)
/// - R = Retire: archive heat (uppercase)
/// - f = furlough: change heat status or rename (lowercase)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrn_HeatAction {
    Nominate,
    Slate,
    Rail,
    Tally,
    Draft,
    Retire,
    Furlough,
}

impl jjrn_HeatAction {
    /// Get single-letter code for commit message
    pub fn jjrn_code(&self) -> char {
        match self {
            jjrn_HeatAction::Nominate => 'N',
            jjrn_HeatAction::Slate => 'S',
            jjrn_HeatAction::Rail => 'r',
            jjrn_HeatAction::Tally => 'T',
            jjrn_HeatAction::Draft => 'D',
            jjrn_HeatAction::Retire => 'R',
            jjrn_HeatAction::Furlough => 'f',
        }
    }

    /// Get full display string (for user-facing output)
    pub fn jjrn_as_str(&self) -> &'static str {
        match self {
            jjrn_HeatAction::Nominate => "nominate",
            jjrn_HeatAction::Slate => "slate",
            jjrn_HeatAction::Rail => "rail",
            jjrn_HeatAction::Tally => "tally",
            jjrn_HeatAction::Draft => "draft",
            jjrn_HeatAction::Retire => "retire",
            jjrn_HeatAction::Furlough => "furlough",
        }
    }
}

/// Format the notch prefix: jjb:HALLMARK:₢CORONET:n:
///
/// Returns the prefix string to prepend to commit messages for JJ-aware commits.
/// The coronet provides full context (embeds parent firemark).
pub fn jjrn_format_notch_prefix(coronet: &Coronet) -> String {
    let hallmark = zjjrn_get_hallmark();
    format!(
        "{}:{}:{}{}:n: ",
        JJRN_COMMIT_PREFIX,
        hallmark,
        CORONET_PREFIX,
        coronet.jjrf_as_str(),
    )
}

/// Format the chalk message: jjb:HALLMARK:₢CORONET:X: description
///
/// Returns the full commit message for a chalk (steeplechase marker) commit.
pub fn jjrn_format_chalk_message(coronet: &Coronet, marker: jjrn_ChalkMarker, description: &str) -> String {
    let hallmark = zjjrn_get_hallmark();
    format!(
        "{}:{}:{}{}:{}: {}",
        JJRN_COMMIT_PREFIX,
        hallmark,
        CORONET_PREFIX,
        coronet.jjrf_as_str(),
        marker.jjrn_code(),
        description
    )
}

/// Format a heat-level discussion message (no pace context): jjb:HALLMARK:₣XX:d: description
pub fn jjrn_format_heat_discussion(firemark: &Firemark, description: &str) -> String {
    let hallmark = zjjrn_get_hallmark();
    format!(
        "{}:{}:{}{}:d: {}",
        JJRN_COMMIT_PREFIX,
        hallmark,
        FIREMARK_PREFIX,
        firemark.jjrf_as_str(),
        description
    )
}

/// Format a heat-level action message: jjb:HALLMARK:₣XX:X: description
///
/// Used for nominate, slate, rail, tally, draft, retire operations.
pub fn jjrn_format_heat_message(firemark: &Firemark, action: jjrn_HeatAction, description: &str) -> String {
    let hallmark = zjjrn_get_hallmark();
    format!(
        "{}:{}:{}{}:{}: {}",
        JJRN_COMMIT_PREFIX,
        hallmark,
        FIREMARK_PREFIX,
        firemark.jjrf_as_str(),
        action.jjrn_code(),
        description
    )
}
