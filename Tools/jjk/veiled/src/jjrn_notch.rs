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
//! Commit format: `jjb:BRAND:IDENTITY[:ACTION]: message`
//! - BRAND: Repository identifier (e.g., "RBM")
//! - IDENTITY: ₢CORONET for pace-level, ₣FIREMARK for heat-level
//! - ACTION: Single letter code (optional for standard notch)
//!
//! The actual commit execution is handled by vorc_commit in the vok crate.
//! This module exports formatting functions that the CLI handlers use.

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark, JJRF_CORONET_PREFIX as CORONET_PREFIX, JJRF_FIREMARK_PREFIX as FIREMARK_PREFIX};

/// Default brand identifier (placeholder until configured by installation-identifier pace)
pub const JJRN_DEFAULT_BRAND: &str = "RBM";

/// Commit message prefix
pub const JJRN_COMMIT_PREFIX: &str = "jjb";

/// Pace-level chalk markers (single-letter codes)
/// - A = APPROACH: proposed approach before work begins
/// - W = WRAP: pace completion summary
/// - F = FLY: autonomous execution began (primed pace)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrn_HeatAction {
    Nominate,
    Slate,
    Rail,
    Tally,
    Draft,
    Retire,
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
        }
    }
}

/// Format the notch prefix: jjb:BRAND:₢CORONET:n:
///
/// Returns the prefix string to prepend to commit messages for JJ-aware commits.
/// The coronet provides full context (embeds parent firemark).
pub fn jjrn_format_notch_prefix(coronet: &Coronet) -> String {
    format!(
        "{}:{}:{}{}:n: ",
        JJRN_COMMIT_PREFIX,
        JJRN_DEFAULT_BRAND,
        CORONET_PREFIX,
        coronet.jjrf_as_str(),
    )
}

/// Format the chalk message: jjb:BRAND:₢CORONET:X: description
/// When coronet is None (heat-level discussion), format as: jjb:BRAND:₣XX:d: description
///
/// Returns the full commit message for a chalk (steeplechase marker) commit.
pub fn jjrn_format_chalk_message(coronet: &Coronet, marker: jjrn_ChalkMarker, description: &str) -> String {
    format!(
        "{}:{}:{}{}:{}: {}",
        JJRN_COMMIT_PREFIX,
        JJRN_DEFAULT_BRAND,
        CORONET_PREFIX,
        coronet.jjrf_as_str(),
        marker.jjrn_code(),
        description
    )
}

/// Format a heat-level discussion message (no pace context): jjb:BRAND:₣XX:d: description
pub fn jjrn_format_heat_discussion(firemark: &Firemark, description: &str) -> String {
    format!(
        "{}:{}:{}{}:d: {}",
        JJRN_COMMIT_PREFIX,
        JJRN_DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.jjrf_as_str(),
        description
    )
}

/// Format a heat-level action message: jjb:BRAND:₣XX:X: description
///
/// Used for nominate, slate, rail, tally, draft, retire operations.
pub fn jjrn_format_heat_message(firemark: &Firemark, action: jjrn_HeatAction, description: &str) -> String {
    format!(
        "{}:{}:{}{}:{}: {}",
        JJRN_COMMIT_PREFIX,
        JJRN_DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.jjrf_as_str(),
        action.jjrn_code(),
        description
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ChalkMarker tests

    #[test]
    fn test_chalk_marker_parse_single_letter() {
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("A").unwrap(), jjrn_ChalkMarker::Approach);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("W").unwrap(), jjrn_ChalkMarker::Wrap);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("F").unwrap(), jjrn_ChalkMarker::Fly);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("d").unwrap(), jjrn_ChalkMarker::Discussion);
    }

    #[test]
    fn test_chalk_marker_parse_full_word() {
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("APPROACH").unwrap(), jjrn_ChalkMarker::Approach);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("approach").unwrap(), jjrn_ChalkMarker::Approach);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("Wrap").unwrap(), jjrn_ChalkMarker::Wrap);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("FLY").unwrap(), jjrn_ChalkMarker::Fly);
        assert_eq!(jjrn_ChalkMarker::jjrn_parse("discussion").unwrap(), jjrn_ChalkMarker::Discussion);
        assert!(jjrn_ChalkMarker::jjrn_parse("invalid").is_err());
    }

    #[test]
    fn test_chalk_marker_code() {
        assert_eq!(jjrn_ChalkMarker::Approach.jjrn_code(), 'A');
        assert_eq!(jjrn_ChalkMarker::Wrap.jjrn_code(), 'W');
        assert_eq!(jjrn_ChalkMarker::Fly.jjrn_code(), 'F');
        assert_eq!(jjrn_ChalkMarker::Discussion.jjrn_code(), 'd');
    }

    #[test]
    fn test_chalk_marker_as_str() {
        assert_eq!(jjrn_ChalkMarker::Approach.jjrn_as_str(), "APPROACH");
        assert_eq!(jjrn_ChalkMarker::Wrap.jjrn_as_str(), "WRAP");
        assert_eq!(jjrn_ChalkMarker::Fly.jjrn_as_str(), "FLY");
        assert_eq!(jjrn_ChalkMarker::Discussion.jjrn_as_str(), "discussion");
    }

    #[test]
    fn test_chalk_marker_requires_pace() {
        assert!(jjrn_ChalkMarker::Approach.jjrn_requires_pace());
        assert!(jjrn_ChalkMarker::Wrap.jjrn_requires_pace());
        assert!(jjrn_ChalkMarker::Fly.jjrn_requires_pace());
        assert!(!jjrn_ChalkMarker::Discussion.jjrn_requires_pace());
    }

    // HeatAction tests

    #[test]
    fn test_heat_action_code() {
        assert_eq!(jjrn_HeatAction::Nominate.jjrn_code(), 'N');
        assert_eq!(jjrn_HeatAction::Slate.jjrn_code(), 'S');
        assert_eq!(jjrn_HeatAction::Rail.jjrn_code(), 'r');
        assert_eq!(jjrn_HeatAction::Tally.jjrn_code(), 'T');
        assert_eq!(jjrn_HeatAction::Draft.jjrn_code(), 'D');
        assert_eq!(jjrn_HeatAction::Retire.jjrn_code(), 'R');
    }

    #[test]
    fn test_heat_action_as_str() {
        assert_eq!(jjrn_HeatAction::Nominate.jjrn_as_str(), "nominate");
        assert_eq!(jjrn_HeatAction::Slate.jjrn_as_str(), "slate");
        assert_eq!(jjrn_HeatAction::Rail.jjrn_as_str(), "rail");
        assert_eq!(jjrn_HeatAction::Tally.jjrn_as_str(), "tally");
        assert_eq!(jjrn_HeatAction::Draft.jjrn_as_str(), "draft");
        assert_eq!(jjrn_HeatAction::Retire.jjrn_as_str(), "retire");
    }

    // Format function tests

    #[test]
    fn test_format_notch_prefix() {
        let coronet = Coronet::jjrf_parse("ABAAA").unwrap();
        let prefix = jjrn_format_notch_prefix(&coronet);
        assert_eq!(prefix, "jjb:RBM:₢ABAAA:n: ");
    }

    #[test]
    fn test_format_chalk_message_approach() {
        let coronet = Coronet::jjrf_parse("ABAAA").unwrap();
        let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Approach, "Starting work on feature");
        assert_eq!(msg, "jjb:RBM:₢ABAAA:A: Starting work on feature");
    }

    #[test]
    fn test_format_chalk_message_wrap() {
        let coronet = Coronet::jjrf_parse("__AAA").unwrap();
        let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Wrap, "Completed the task");
        assert_eq!(msg, "jjb:RBM:₢__AAA:W: Completed the task");
    }

    #[test]
    fn test_format_chalk_message_fly() {
        let coronet = Coronet::jjrf_parse("ABCDE").unwrap();
        let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Fly, "Autonomous execution");
        assert_eq!(msg, "jjb:RBM:₢ABCDE:F: Autonomous execution");
    }

    #[test]
    fn test_format_chalk_message_discussion() {
        let coronet = Coronet::jjrf_parse("ABAAA").unwrap();
        let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Discussion, "Design discussion");
        assert_eq!(msg, "jjb:RBM:₢ABAAA:d: Design discussion");
    }

    #[test]
    fn test_format_heat_discussion() {
        let fm = Firemark::jjrf_parse("AB").unwrap();
        let msg = jjrn_format_heat_discussion(&fm, "Design discussion without pace");
        assert_eq!(msg, "jjb:RBM:₣AB:d: Design discussion without pace");
    }

    #[test]
    fn test_format_heat_message_nominate() {
        let fm = Firemark::jjrf_parse("AB").unwrap();
        let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Nominate, "my-new-heat");
        assert_eq!(msg, "jjb:RBM:₣AB:N: my-new-heat");
    }

    #[test]
    fn test_format_heat_message_slate() {
        let fm = Firemark::jjrf_parse("AB").unwrap();
        let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Slate, "new-pace-silks");
        assert_eq!(msg, "jjb:RBM:₣AB:S: new-pace-silks");
    }

    #[test]
    fn test_format_heat_message_rail() {
        let fm = Firemark::jjrf_parse("AB").unwrap();
        let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Rail, "reordered");
        assert_eq!(msg, "jjb:RBM:₣AB:r: reordered");
    }

    #[test]
    fn test_format_heat_message_tally() {
        let fm = Firemark::jjrf_parse("AB").unwrap();
        let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Tally, "pace-name");
        assert_eq!(msg, "jjb:RBM:₣AB:T: pace-name");
    }

    #[test]
    fn test_format_heat_message_draft() {
        let fm = Firemark::jjrf_parse("CD").unwrap();
        let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Draft, "₢ABAAA → ₣CD");
        assert_eq!(msg, "jjb:RBM:₣CD:D: ₢ABAAA → ₣CD");
    }

    #[test]
    fn test_format_heat_message_retire() {
        let fm = Firemark::jjrf_parse("AB").unwrap();
        let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Retire, "my-heat-silks");
        assert_eq!(msg, "jjb:RBM:₣AB:R: my-heat-silks");
    }
}
