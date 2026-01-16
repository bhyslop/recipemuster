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

use crate::jjrf_favor::{Coronet, Firemark, CORONET_PREFIX, FIREMARK_PREFIX};

/// Default brand identifier (placeholder until configured by installation-identifier pace)
const DEFAULT_BRAND: &str = "RBM";

/// Commit message prefix
const COMMIT_PREFIX: &str = "jjb";

/// Pace-level chalk markers (single-letter codes)
/// - A = APPROACH: proposed approach before work begins
/// - W = WRAP: pace completion summary
/// - F = FLY: autonomous execution began (primed pace)
/// - d = discussion: significant decision (lowercase, can be heat-level too)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChalkMarker {
    Approach,
    Wrap,
    Fly,
    Discussion,
}

impl ChalkMarker {
    /// Parse marker from string (accepts both single letter and full word)
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "A" | "a" | "APPROACH" | "approach" | "Approach" => Ok(ChalkMarker::Approach),
            "W" | "w" | "WRAP" | "wrap" | "Wrap" => Ok(ChalkMarker::Wrap),
            "F" | "f" | "FLY" | "fly" | "Fly" => Ok(ChalkMarker::Fly),
            "d" | "D" | "DISCUSSION" | "discussion" | "Discussion" => Ok(ChalkMarker::Discussion),
            _ => Err(format!(
                "Invalid marker type '{}'. Valid: A(pproach), W(rap), F(ly), d(iscussion)",
                s
            )),
        }
    }

    /// Get single-letter code for commit message
    pub fn code(&self) -> char {
        match self {
            ChalkMarker::Approach => 'A',
            ChalkMarker::Wrap => 'W',
            ChalkMarker::Fly => 'F',
            ChalkMarker::Discussion => 'd',
        }
    }

    /// Get full display string for the marker (for user-facing output)
    pub fn as_str(&self) -> &'static str {
        match self {
            ChalkMarker::Approach => "APPROACH",
            ChalkMarker::Wrap => "WRAP",
            ChalkMarker::Fly => "FLY",
            ChalkMarker::Discussion => "discussion",
        }
    }

    /// Returns true if this marker type requires a pace to be specified
    pub fn requires_pace(&self) -> bool {
        match self {
            ChalkMarker::Approach | ChalkMarker::Wrap | ChalkMarker::Fly => true,
            ChalkMarker::Discussion => false,
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
pub enum HeatAction {
    Nominate,
    Slate,
    Rail,
    Tally,
    Draft,
    Retire,
}

impl HeatAction {
    /// Get single-letter code for commit message
    pub fn code(&self) -> char {
        match self {
            HeatAction::Nominate => 'N',
            HeatAction::Slate => 'S',
            HeatAction::Rail => 'r',
            HeatAction::Tally => 'T',
            HeatAction::Draft => 'D',
            HeatAction::Retire => 'R',
        }
    }

    /// Get full display string (for user-facing output)
    pub fn as_str(&self) -> &'static str {
        match self {
            HeatAction::Nominate => "nominate",
            HeatAction::Slate => "slate",
            HeatAction::Rail => "rail",
            HeatAction::Tally => "tally",
            HeatAction::Draft => "draft",
            HeatAction::Retire => "retire",
        }
    }
}

/// Format the notch prefix: jjb:BRAND:₢CORONET:n:
///
/// Returns the prefix string to prepend to commit messages for JJ-aware commits.
/// The coronet provides full context (embeds parent firemark).
pub fn format_notch_prefix(coronet: &Coronet) -> String {
    format!(
        "{}:{}:{}{}:n: ",
        COMMIT_PREFIX,
        DEFAULT_BRAND,
        CORONET_PREFIX,
        coronet.as_str(),
    )
}

/// Format the chalk message: jjb:BRAND:₢CORONET:X: description
/// When coronet is None (heat-level discussion), format as: jjb:BRAND:₣XX:d: description
///
/// Returns the full commit message for a chalk (steeplechase marker) commit.
pub fn format_chalk_message(coronet: &Coronet, marker: ChalkMarker, description: &str) -> String {
    format!(
        "{}:{}:{}{}:{}: {}",
        COMMIT_PREFIX,
        DEFAULT_BRAND,
        CORONET_PREFIX,
        coronet.as_str(),
        marker.code(),
        description
    )
}

/// Format a heat-level discussion message (no pace context): jjb:BRAND:₣XX:d: description
pub fn format_heat_discussion(firemark: &Firemark, description: &str) -> String {
    format!(
        "{}:{}:{}{}:d: {}",
        COMMIT_PREFIX,
        DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.as_str(),
        description
    )
}

/// Format a heat-level action message: jjb:BRAND:₣XX:X: description
///
/// Used for nominate, slate, rail, tally, draft, retire operations.
pub fn format_heat_message(firemark: &Firemark, action: HeatAction, description: &str) -> String {
    format!(
        "{}:{}:{}{}:{}: {}",
        COMMIT_PREFIX,
        DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.as_str(),
        action.code(),
        description
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ChalkMarker tests

    #[test]
    fn test_chalk_marker_parse_single_letter() {
        assert_eq!(ChalkMarker::parse("A").unwrap(), ChalkMarker::Approach);
        assert_eq!(ChalkMarker::parse("W").unwrap(), ChalkMarker::Wrap);
        assert_eq!(ChalkMarker::parse("F").unwrap(), ChalkMarker::Fly);
        assert_eq!(ChalkMarker::parse("d").unwrap(), ChalkMarker::Discussion);
    }

    #[test]
    fn test_chalk_marker_parse_full_word() {
        assert_eq!(ChalkMarker::parse("APPROACH").unwrap(), ChalkMarker::Approach);
        assert_eq!(ChalkMarker::parse("approach").unwrap(), ChalkMarker::Approach);
        assert_eq!(ChalkMarker::parse("Wrap").unwrap(), ChalkMarker::Wrap);
        assert_eq!(ChalkMarker::parse("FLY").unwrap(), ChalkMarker::Fly);
        assert_eq!(ChalkMarker::parse("discussion").unwrap(), ChalkMarker::Discussion);
        assert!(ChalkMarker::parse("invalid").is_err());
    }

    #[test]
    fn test_chalk_marker_code() {
        assert_eq!(ChalkMarker::Approach.code(), 'A');
        assert_eq!(ChalkMarker::Wrap.code(), 'W');
        assert_eq!(ChalkMarker::Fly.code(), 'F');
        assert_eq!(ChalkMarker::Discussion.code(), 'd');
    }

    #[test]
    fn test_chalk_marker_as_str() {
        assert_eq!(ChalkMarker::Approach.as_str(), "APPROACH");
        assert_eq!(ChalkMarker::Wrap.as_str(), "WRAP");
        assert_eq!(ChalkMarker::Fly.as_str(), "FLY");
        assert_eq!(ChalkMarker::Discussion.as_str(), "discussion");
    }

    #[test]
    fn test_chalk_marker_requires_pace() {
        assert!(ChalkMarker::Approach.requires_pace());
        assert!(ChalkMarker::Wrap.requires_pace());
        assert!(ChalkMarker::Fly.requires_pace());
        assert!(!ChalkMarker::Discussion.requires_pace());
    }

    // HeatAction tests

    #[test]
    fn test_heat_action_code() {
        assert_eq!(HeatAction::Nominate.code(), 'N');
        assert_eq!(HeatAction::Slate.code(), 'S');
        assert_eq!(HeatAction::Rail.code(), 'r');
        assert_eq!(HeatAction::Tally.code(), 'T');
        assert_eq!(HeatAction::Draft.code(), 'D');
        assert_eq!(HeatAction::Retire.code(), 'R');
    }

    #[test]
    fn test_heat_action_as_str() {
        assert_eq!(HeatAction::Nominate.as_str(), "nominate");
        assert_eq!(HeatAction::Slate.as_str(), "slate");
        assert_eq!(HeatAction::Rail.as_str(), "rail");
        assert_eq!(HeatAction::Tally.as_str(), "tally");
        assert_eq!(HeatAction::Draft.as_str(), "draft");
        assert_eq!(HeatAction::Retire.as_str(), "retire");
    }

    // Format function tests

    #[test]
    fn test_format_notch_prefix() {
        let coronet = Coronet::parse("ABAAA").unwrap();
        let prefix = format_notch_prefix(&coronet);
        assert_eq!(prefix, "jjb:RBM:₢ABAAA:n: ");
    }

    #[test]
    fn test_format_chalk_message_approach() {
        let coronet = Coronet::parse("ABAAA").unwrap();
        let msg = format_chalk_message(&coronet, ChalkMarker::Approach, "Starting work on feature");
        assert_eq!(msg, "jjb:RBM:₢ABAAA:A: Starting work on feature");
    }

    #[test]
    fn test_format_chalk_message_wrap() {
        let coronet = Coronet::parse("__AAA").unwrap();
        let msg = format_chalk_message(&coronet, ChalkMarker::Wrap, "Completed the task");
        assert_eq!(msg, "jjb:RBM:₢__AAA:W: Completed the task");
    }

    #[test]
    fn test_format_chalk_message_fly() {
        let coronet = Coronet::parse("ABCDE").unwrap();
        let msg = format_chalk_message(&coronet, ChalkMarker::Fly, "Autonomous execution");
        assert_eq!(msg, "jjb:RBM:₢ABCDE:F: Autonomous execution");
    }

    #[test]
    fn test_format_chalk_message_discussion() {
        let coronet = Coronet::parse("ABAAA").unwrap();
        let msg = format_chalk_message(&coronet, ChalkMarker::Discussion, "Design discussion");
        assert_eq!(msg, "jjb:RBM:₢ABAAA:d: Design discussion");
    }

    #[test]
    fn test_format_heat_discussion() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_heat_discussion(&fm, "Design discussion without pace");
        assert_eq!(msg, "jjb:RBM:₣AB:d: Design discussion without pace");
    }

    #[test]
    fn test_format_heat_message_nominate() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_heat_message(&fm, HeatAction::Nominate, "my-new-heat");
        assert_eq!(msg, "jjb:RBM:₣AB:N: my-new-heat");
    }

    #[test]
    fn test_format_heat_message_slate() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_heat_message(&fm, HeatAction::Slate, "new-pace-silks");
        assert_eq!(msg, "jjb:RBM:₣AB:S: new-pace-silks");
    }

    #[test]
    fn test_format_heat_message_rail() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_heat_message(&fm, HeatAction::Rail, "reordered");
        assert_eq!(msg, "jjb:RBM:₣AB:r: reordered");
    }

    #[test]
    fn test_format_heat_message_tally() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_heat_message(&fm, HeatAction::Tally, "pace-name");
        assert_eq!(msg, "jjb:RBM:₣AB:T: pace-name");
    }

    #[test]
    fn test_format_heat_message_draft() {
        let fm = Firemark::parse("CD").unwrap();
        let msg = format_heat_message(&fm, HeatAction::Draft, "₢ABAAA → ₣CD");
        assert_eq!(msg, "jjb:RBM:₣CD:D: ₢ABAAA → ₣CD");
    }

    #[test]
    fn test_format_heat_message_retire() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_heat_message(&fm, HeatAction::Retire, "my-heat-silks");
        assert_eq!(msg, "jjb:RBM:₣AB:R: my-heat-silks");
    }
}
