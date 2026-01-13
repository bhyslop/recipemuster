//! Notch and Chalk - Steeplechase commit operations
//!
//! Implements JJ-aware git commits that record session history:
//! - jjx_notch: Standard commit with heat/pace context prefix
//! - jjx_chalk: Empty commit marking a steeplechase event
//!
//! Both commands delegate to vvx commit for the actual git work.

use std::process::Command;
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
}

/// Arguments for jjx_notch command
#[derive(Debug)]
pub struct NotchArgs {
    /// Active Heat identity (positional)
    pub firemark: Firemark,
    /// Silks of the current pace
    pub pace_silks: String,
    /// Optional commit message (if absent, claude generates from diff)
    pub message: Option<String>,
}

/// Arguments for jjx_chalk command
#[derive(Debug)]
pub struct ChalkArgs {
    /// Active Heat identity (positional)
    pub firemark: Firemark,
    /// Marker type
    pub marker: ChalkMarker,
    /// Marker description text
    pub description: String,
}

/// Format the notch prefix: [jj:BRAND][₣XX/pace-silks]
fn format_notch_prefix(firemark: &Firemark, pace_silks: &str) -> String {
    format!(
        "[jj:{}][{}{}/{}] ",
        DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.as_str(),
        pace_silks
    )
}

/// Format the chalk message: [jj:BRAND][₣XX] MARKER: description
fn format_chalk_message(firemark: &Firemark, marker: ChalkMarker, description: &str) -> String {
    format!(
        "[jj:{}][{}{}] {}: {}",
        DEFAULT_BRAND,
        FIREMARK_PREFIX,
        firemark.as_str(),
        marker.as_str(),
        description
    )
}

/// Execute jjx_notch: JJ-aware commit with heat/pace context
///
/// Shells out to `vvx commit` with appropriate prefix.
/// Returns exit code (0 = success).
pub fn run_notch(args: NotchArgs) -> i32 {
    let prefix = format_notch_prefix(&args.firemark, &args.pace_silks);

    // Build vvx commit arguments
    let mut cmd_args = vec!["commit".to_string(), "--prefix".to_string(), prefix];

    if let Some(msg) = &args.message {
        cmd_args.push("--message".to_string());
        cmd_args.push(msg.clone());
    }

    eprintln!("notch: invoking vvx commit");

    let result = Command::new("vvx")
        .args(&cmd_args)
        .output();

    match result {
        Ok(output) => {
            // Forward stdout (commit hash)
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            // Forward stderr
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            if output.status.success() {
                0
            } else {
                output.status.code().unwrap_or(1)
            }
        }
        Err(e) => {
            eprintln!("notch: error: failed to invoke vvx: {}", e);
            1
        }
    }
}

/// Execute jjx_chalk: Empty commit marking a steeplechase event
///
/// Shells out to `vvx commit` with --allow-empty and formatted message.
/// Returns exit code (0 = success).
pub fn run_chalk(args: ChalkArgs) -> i32 {
    let message = format_chalk_message(&args.firemark, args.marker, &args.description);

    eprintln!("chalk: invoking vvx commit --allow-empty");

    let result = Command::new("vvx")
        .args(["commit", "--allow-empty", "--no-stage", "--message", &message])
        .output();

    match result {
        Ok(output) => {
            // Forward stdout (commit hash)
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            // Forward stderr
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            if output.status.success() {
                0
            } else {
                output.status.code().unwrap_or(1)
            }
        }
        Err(e) => {
            eprintln!("chalk: error: failed to invoke vvx: {}", e);
            1
        }
    }
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
    fn test_format_chalk_message() {
        let fm = Firemark::parse("AB").unwrap();
        let msg = format_chalk_message(&fm, ChalkMarker::Approach, "Starting work on feature");
        assert_eq!(msg, "[jj:RBM][₣AB] APPROACH: Starting work on feature");
    }

    #[test]
    fn test_format_chalk_message_wrap() {
        let fm = Firemark::parse("__").unwrap();
        let msg = format_chalk_message(&fm, ChalkMarker::Wrap, "Completed the task");
        assert_eq!(msg, "[jj:RBM][₣__] WRAP: Completed the task");
    }
}
