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

/// Commit message prefix
pub const JJRN_COMMIT_PREFIX: &str = "jjb";


/// Pace-level chalk markers (single-letter codes)
/// - A = APPROACH: proposed approach before work begins
/// - W = WRAP: pace completion summary
/// - F = FLY: autonomous execution began (bridled pace)
/// - B = BRIDLE: pace transitioned to bridled state
/// - d = discussion: significant decision (lowercase, can be heat-level too)
/// - s = session: new work session started (lowercase, heat-level only)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrn_ChalkMarker {
    Approach,
    Wrap,
    Fly,
    Bridle,
    Discussion,
    Session,
}

impl jjrn_ChalkMarker {
    /// Parse marker from string (accepts both single letter and full word)
    pub fn jjrn_parse(s: &str) -> Result<Self, String> {
        match s {
            "A" | "a" | "APPROACH" | "approach" | "Approach" => Ok(jjrn_ChalkMarker::Approach),
            "W" | "w" | "WRAP" | "wrap" | "Wrap" => Ok(jjrn_ChalkMarker::Wrap),
            "F" | "f" | "FLY" | "fly" | "Fly" => Ok(jjrn_ChalkMarker::Fly),
            "B" | "b" | "BRIDLE" | "bridle" | "Bridle" => Ok(jjrn_ChalkMarker::Bridle),
            "d" | "D" | "DISCUSSION" | "discussion" | "Discussion" => Ok(jjrn_ChalkMarker::Discussion),
            "s" | "S" | "SESSION" | "session" | "Session" => Ok(jjrn_ChalkMarker::Session),
            _ => Err(format!(
                "Invalid marker type '{}'. Valid: A(pproach), W(rap), F(ly), B(ridle), d(iscussion), s(ession)",
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
            jjrn_ChalkMarker::Bridle => 'B',
            jjrn_ChalkMarker::Discussion => 'd',
            jjrn_ChalkMarker::Session => 's',
        }
    }

    /// Get full display string for the marker (for user-facing output)
    pub fn jjrn_as_str(&self) -> &'static str {
        match self {
            jjrn_ChalkMarker::Approach => "APPROACH",
            jjrn_ChalkMarker::Wrap => "WRAP",
            jjrn_ChalkMarker::Fly => "FLY",
            jjrn_ChalkMarker::Bridle => "BRIDLE",
            jjrn_ChalkMarker::Discussion => "discussion",
            jjrn_ChalkMarker::Session => "session",
        }
    }

    /// Returns true if this marker type requires a pace to be specified
    pub fn jjrn_requires_pace(&self) -> bool {
        match self {
            jjrn_ChalkMarker::Approach | jjrn_ChalkMarker::Wrap | jjrn_ChalkMarker::Fly | jjrn_ChalkMarker::Bridle => true,
            jjrn_ChalkMarker::Discussion | jjrn_ChalkMarker::Session => false,
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
/// - G = Garland: celebrate completed heat, create continuation (uppercase)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrn_HeatAction {
    Nominate,
    Slate,
    Rail,
    Tally,
    Draft,
    Retire,
    Furlough,
    Garland,
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
            jjrn_HeatAction::Garland => 'G',
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
            jjrn_HeatAction::Garland => "garland",
        }
    }
}

/// Format the notch prefix: jjb:HALLMARK:₢CORONET:n:
///
/// Returns the prefix string to prepend to commit messages for JJ-aware commits.
/// The coronet provides full context (embeds parent firemark).
pub fn jjrn_format_notch_prefix(coronet: &Coronet) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    let identity = format!("{}{}", CORONET_PREFIX, coronet.jjrf_as_str());
    // Special case: subject="" produces "...:n: " and caller appends real message
    vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity, "n", "", None)
}

/// Format the chalk message: jjb:HALLMARK:₢CORONET:X: description
///
/// Returns the full commit message for a chalk (steeplechase marker) commit.
pub fn jjrn_format_chalk_message(coronet: &Coronet, marker: jjrn_ChalkMarker, description: &str) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    let identity = format!("{}{}", CORONET_PREFIX, coronet.jjrf_as_str());
    let action = marker.jjrn_code().to_string();
    vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity, &action, description, None)
}

/// Format a heat-level discussion message (no pace context): jjb:HALLMARK:₣XX:d: description
pub fn jjrn_format_heat_discussion(firemark: &Firemark, description: &str) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    let identity = format!("{}{}", FIREMARK_PREFIX, firemark.jjrf_as_str());
    vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity, "d", description, None)
}

/// Format a heat-level action message: jjb:HALLMARK:₣XX:X: description
///
/// Used for nominate, slate, rail, tally, draft, retire operations.
pub fn jjrn_format_heat_message(firemark: &Firemark, action: jjrn_HeatAction, description: &str) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    let identity = format!("{}{}", FIREMARK_PREFIX, firemark.jjrf_as_str());
    let action_code = action.jjrn_code().to_string();
    vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity, &action_code, description, None)
}

/// Format a bridle message: jjb:HALLMARK:₢CORONET:B: {agent} | {silks}
///
/// Creates the subject line for a B (bridle) commit.
/// Body should contain the full direction text.
pub fn jjrn_format_bridle_message(coronet: &Coronet, agent: &str, silks: &str) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    let identity = format!("{}{}", CORONET_PREFIX, coronet.jjrf_as_str());
    let subject = format!("{} | {}", agent, silks);
    vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity, "B", &subject, None)
}

/// Format a landing message: jjb:HALLMARK:₢CORONET:L: {agent} landed
///
/// Creates the subject line for an L (landing) commit.
/// Body should contain the agent completion report.
pub fn jjrn_format_landing_message(coronet: &Coronet, agent: &str) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    let identity = format!("{}{}", CORONET_PREFIX, coronet.jjrf_as_str());
    let subject = format!("{} landed", agent);
    vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity, "L", &subject, None)
}

/// Format a session marker message: jjb:HALLMARK:₣XX:s: YYMMDD-HHMM session
///
/// Creates the subject line for an s (session) commit.
/// Body should contain model IDs, host, and platform.
pub fn jjrn_format_session_message(firemark: &Firemark, timestamp: &str) -> String {
    let hallmark = vvc::vvcc_get_hallmark();
    format!(
        "{}:{}:{}{}:s: {} session",
        JJRN_COMMIT_PREFIX,
        hallmark,
        FIREMARK_PREFIX,
        firemark.jjrf_as_str(),
        timestamp
    )
}
