//! Notch and Chalk - Steeplechase commit formatting
//!
//! Provides message formatting for JJ-aware git commits that record session history:
//! - jjx_notch: Standard commit with heat/pace context prefix
//! - jjx_chalk: Empty commit marking a steeplechase event
//!
//! The actual commit execution is handled by vorc_commit in the vok crate.
//! This module exports formatting functions that the CLI handlers use.

use crate::jjrf_favor::{Firemark, FIREMARK_PREFIX};

/// Default brand identifier (placeholder until configured)
const DEFAULT_BRAND: &str = "RBM";

/// Valid marker types for chalk commits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChalkMarker {
    Approach,
    Wrap,
    Fly,
    Discussion,
}

impl ChalkMarker {
    /// Parse marker from string (case-insensitive)
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "APPROACH" => Ok(ChalkMarker::Approach),
            "WRAP" => Ok(ChalkMarker::Wrap),
            "FLY" => Ok(ChalkMarker::Fly),
            "DISCUSSION" => Ok(ChalkMarker::Discussion),
            _ => Err(format!(
                "Invalid marker type '{}'. Valid types: APPROACH, WRAP, FLY, DISCUSSION",
                s
            )),
        }
    }

    /// Get display string for the marker
    pub fn as_str(&self) -> &'static str {
        match self {
            ChalkMarker::Approach => "APPROACH",
            ChalkMarker::Wrap => "WRAP",
            ChalkMarker::Fly => "FLY",
            ChalkMarker::Discussion => "DISCUSSION",
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

/// Format the notch prefix: [jj:BRAND][₣XX/pace-silks]
///
/// Returns the prefix string to prepend to commit messages for JJ-aware commits.
pub fn format_notch_prefix(firemark: &Firemark, pace_silks: &str) -> String {
    format!(
        "[jj:{}][{}{}/{}] ",
        DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.as_str(),
        pace_silks
    )
}

/// Format the chalk message: [jj:BRAND][₣XX/pace-silks] MARKER: description
/// When pace is None, format as: [jj:BRAND][₣XX] MARKER: description
///
/// Returns the full commit message for a chalk (steeplechase marker) commit.
pub fn format_chalk_message(firemark: &Firemark, marker: ChalkMarker, pace: Option<&str>, description: &str) -> String {
    let heat_pace = match pace {
        Some(p) => format!("{}{}/{}", FIREMARK_PREFIX, firemark.as_str(), p),
        None => format!("{}{}", FIREMARK_PREFIX, firemark.as_str()),
    };
    format!(
        "[jj:{}][{}] {}: {}",
        DEFAULT_BRAND,
        heat_pace,
        marker.as_str(),
        description
    )
}

/// Validate chalk arguments
///
/// Returns Ok(()) if valid, Err with message if invalid.
pub fn validate_chalk_args(marker: ChalkMarker, pace: Option<&str>) -> Result<(), String> {
    if marker.requires_pace() && pace.is_none() {
        return Err(format!(
            "{} marker requires --pace to be specified",
            marker.as_str()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chalk_marker_parse() {
        assert_eq!(ChalkMarker::parse("APPROACH").unwrap(), ChalkMarker::Approach);
        assert_eq!(ChalkMarker::parse("approach").unwrap(), ChalkMarker::Approach);
        assert_eq!(ChalkMarker::parse("Wrap").unwrap(), ChalkMarker::Wrap);
        assert_eq!(ChalkMarker::parse("FLY").unwrap(), ChalkMarker::Fly);
        assert_eq!(ChalkMarker::parse("discussion").unwrap(), ChalkMarker::Discussion);
        assert!(ChalkMarker::parse("invalid").is_err());
    }

    #[test]
    fn test_chalk_marker_as_str() {
        assert_eq!(ChalkMarker::Approach.as_str(), "APPROACH");
        assert_eq!(ChalkMarker::Wrap.as_str(), "WRAP");
        assert_eq!(ChalkMarker::Fly.as_str(), "FLY");
        assert_eq!(ChalkMarker::Discussion.as_str(), "DISCUSSION");
    }

    #[test]
    fn test_format_notch_prefix() {
        let fm = Firemark::parse("AB").unwrap();
        let prefix = format_notch_prefix(&fm, "my-pace");
        assert_eq!(prefix, "[jj:RBM][₣AB/my-pace] ");
    }

    #[test]
    fn test_chalk_marker_requires_pace() {
        assert!(ChalkMarker::Approach.requires_pace());
        assert!(ChalkMarker::Wrap.requires_pace());
        assert!(ChalkMarker::Fly.requires_pace());
        assert!(!ChalkMarker::Discussion.requires_pace());
    }

    #[test]
    fn test_format_chalk_message_with_pace() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_chalk_message(&fm, ChalkMarker::Approach, Some("my-pace"), "Starting work on feature");
        assert_eq!(msg, "[jj:RBM][₣AB/my-pace] APPROACH: Starting work on feature");
    }

    #[test]
    fn test_format_chalk_message_wrap_with_pace() {
        let fm = Firemark::parse("__").unwrap();
        let msg = format_chalk_message(&fm, ChalkMarker::Wrap, Some("test-pace"), "Completed the task");
        assert_eq!(msg, "[jj:RBM][₣__/test-pace] WRAP: Completed the task");
    }

    #[test]
    fn test_format_chalk_message_without_pace() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_chalk_message(&fm, ChalkMarker::Discussion, None, "Design discussion");
        assert_eq!(msg, "[jj:RBM][₣AB] DISCUSSION: Design discussion");
    }

    #[test]
    fn test_format_chalk_message_discussion_with_optional_pace() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_chalk_message(&fm, ChalkMarker::Discussion, Some("context-pace"), "Design discussion");
        assert_eq!(msg, "[jj:RBM][₣AB/context-pace] DISCUSSION: Design discussion");
    }
}
