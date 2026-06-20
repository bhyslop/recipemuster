// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops type definitions
//!
//! Core data structures for Job Jockey heat and pace tracking.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Unknown/default basis SHA (7 zeros)
///
/// Used when basis commit cannot be determined (git errors).
/// All basis-related code derives the expected length from this constant.
pub const JJRG_UNKNOWN_BASIS: &str = "0000000";

/// Canonical human display labels for pace states — the single home for the
/// short forms shown in CLI output (scout listings, paddock rendering, drop
/// confirmation). Distinct from the persisted serde wire tokens (`jjgte_*`
/// per-variant renames below); these are the operator-facing labels.
pub const JJRG_STATE_ROUGH: &str = "rough";
pub const JJRG_STATE_COMPLETE: &str = "complete";
pub const JJRG_STATE_ABANDONED: &str = "abandoned";

/// Pace state values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum jjrg_PaceState {
    // Forgiveness JJr_a7c — `jjgte_bridled` (and its V3 alias `primed`) are the
    // retired bridled state's legacy on-disk tokens; the bridle-retirement episode
    // demotes them to Rough at the deserialize boundary so a pre-retirement gallops
    // still parses, then the round-trip gate stand-down lets the next save rewrite
    // `jjgte_rough`. See jjri_io ZJJDZ_REGISTRY and JJS0 jjdz_forgiveness.
    #[serde(rename = "jjgte_rough", alias = "jjgte_bridled", alias = "primed")]
    Rough,
    #[serde(rename = "jjgte_complete")]
    Complete,
    #[serde(rename = "jjgte_abandoned")]
    Abandoned,
}

impl jjrg_PaceState {
    /// Human display label for this state (short form, e.g. `rough`) — the
    /// single source of truth for CLI/display output. NOT the persisted serde
    /// token (`jjgte_*`); see the variant renames above for the wire form.
    pub fn jjrg_as_str(&self) -> &'static str {
        match self {
            jjrg_PaceState::Rough => JJRG_STATE_ROUGH,
            jjrg_PaceState::Complete => JJRG_STATE_COMPLETE,
            jjrg_PaceState::Abandoned => JJRG_STATE_ABANDONED,
        }
    }
}

/// Heat status values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum jjrg_HeatStatus {
    /// Heat is actively being worked
    #[serde(rename = "jjghe_racing")]
    Racing,
    /// Heat is paused, not actively worked
    #[serde(rename = "jjghe_stabled")]
    Stabled,
    /// Heat is complete and archived (terminal state)
    #[serde(rename = "jjghe_retired")]
    Retired,
}

/// Tack record - snapshot of Pace state and plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Tack {
    #[serde(rename = "jjgtn_ts")]
    pub ts: String,
    #[serde(rename = "jjgtn_state")]
    pub state: jjrg_PaceState,
    /// Docket text as a line array — one element per physical line, so pretty-JSON
    /// decomposes the docket line-by-line and git merges it at line granularity.
    /// Custom deserialize tolerates the legacy string shape (the tack-text→lines
    /// forgiveness episode — rivet JJr_a7c, JJS0 jjdz_forgiveness): an on-disk
    /// string is split on '\n' into the array, an array is taken verbatim. Serialize
    /// is the default array form. Round-trip is lossless under the
    /// jjrg_text_to_lines / jjrg_lines_to_text pair (split('\n') ⇔ join('\n')).
    #[serde(rename = "jjgtn_text", deserialize_with = "zjjrg_deserialize_text")]
    pub text: Vec<String>,
    #[serde(rename = "jjgtn_silks")]
    pub silks: String,
    #[serde(rename = "jjgtn_basis")]
    pub basis: String,
}

/// Split docket text into the stored line array — the write-side boundary.
/// `split('\n')` (not `lines()`) so the array round-trips losslessly under
/// `jjrg_lines_to_text`: an empty string yields `[""]`, a trailing newline a
/// trailing empty element, consecutive newlines empty interior elements.
pub fn jjrg_text_to_lines(text: &str) -> Vec<String> {
    text.split('\n').map(String::from).collect()
}

/// Join the stored line array back into flat text — the read-side boundary,
/// the exact inverse of `jjrg_text_to_lines`.
pub fn jjrg_lines_to_text(lines: &[String]) -> String {
    lines.join("\n")
}

/// Deserialize tack docket text, tolerating both on-disk shapes (rivet JJr_a7c).
/// A JSON string is the legacy shape (split into lines); a JSON array is the
/// current shape (taken verbatim). Anything else fails fast at the parse boundary.
fn zjjrg_deserialize_text<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum zjjrg_TextShape {
        Legacy(String),
        Lines(Vec<String>),
    }
    Ok(match zjjrg_TextShape::deserialize(deserializer)? {
        zjjrg_TextShape::Legacy(s) => jjrg_text_to_lines(&s),
        zjjrg_TextShape::Lines(v) => v,
    })
}

/// Pace record - discrete action within a Heat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Pace {
    #[serde(rename = "jjgpn_tacks")]
    pub tacks: Vec<jjrg_Tack>,
}

/// Initial pensum seed value (2-char base64url, starts at "AA")
pub const JJRT_PENSUM_SEED_INIT: &str = "AA";

/// Heat record - bounded initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Heat {
    #[serde(rename = "jjghn_silks")]
    pub silks: String,
    #[serde(rename = "jjghn_creation_time")]
    pub creation_time: String,
    #[serde(rename = "jjghn_status")]
    pub status: jjrg_HeatStatus,
    #[serde(rename = "jjghn_order")]
    pub order: Vec<String>,
    #[serde(rename = "jjghn_next_pace_seed")]
    pub next_pace_seed: String,
    #[serde(rename = "jjghn_paces")]
    pub paces: BTreeMap<String, jjrg_Pace>,
}

/// Root Gallops structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Gallops {
    #[serde(rename = "jjgrn_next_heat_seed")]
    pub next_heat_seed: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "jjgrn_heat_order")]
    pub heat_order: Vec<String>,
    #[serde(rename = "jjgrn_heats")]
    pub heats: BTreeMap<String, jjrg_Heat>,
}

/// Arguments for the nominate operation
pub struct jjrg_NominateArgs {
    pub silks: String,
    pub created: String,
}

/// Result of the nominate operation
#[derive(Debug)]
pub struct jjrg_NominateResult {
    pub firemark: String,
}

/// Arguments for the slate operation
pub struct jjrg_SlateArgs {
    pub firemark: String,
    pub silks: String,
    pub text: String,
    /// Coronet to insert before (mutually exclusive with after/first)
    pub before: Option<String>,
    /// Coronet to insert after (mutually exclusive with before/first)
    pub after: Option<String>,
    /// Insert at beginning (mutually exclusive with before/after)
    pub first: bool,
}

/// Result of the slate operation
#[derive(Debug)]
pub struct jjrg_SlateResult {
    pub coronet: String,
}

/// Arguments for the rail operation
///
/// Supports two modes:
/// - Order mode: provide `order` array to replace entire sequence
/// - Move mode: provide `move_coronet` + one positioning field to relocate a single pace
pub struct jjrg_RailArgs {
    pub firemark: String,
    /// Order mode: new sequence of all coronets
    pub order: Vec<String>,
    /// Move mode: coronet to relocate
    pub move_coronet: Option<String>,
    /// Move before this coronet
    pub before: Option<String>,
    /// Move after this coronet
    pub after: Option<String>,
    /// Move to beginning
    pub first: bool,
    /// Move to end
    pub last: bool,
}

/// Arguments for the tally operation
pub struct jjrg_TallyArgs {
    pub coronet: String,
    pub state: Option<jjrg_PaceState>,
    pub text: Option<String>,
    pub silks: Option<String>,
}

/// Arguments for the draft operation
pub struct jjrg_DraftArgs {
    /// Coronet of the pace to move
    pub coronet: String,
    /// Destination heat Firemark
    pub to: String,
    /// Coronet to insert before (mutually exclusive with after/first)
    pub before: Option<String>,
    /// Coronet to insert after (mutually exclusive with before/first)
    pub after: Option<String>,
    /// Insert at beginning (mutually exclusive with before/after)
    pub first: bool,
}

/// Result of the draft operation
#[derive(Debug)]
pub struct jjrg_DraftResult {
    /// New coronet in destination heat
    pub new_coronet: String,
}

/// Arguments for the retire operation
pub struct jjrg_RetireArgs {
    /// Firemark of heat to retire
    pub firemark: String,
    /// Today's date in YYMMDD format (for trophy filename)
    pub today: String,
}

/// Result of the retire operation
#[derive(Debug)]
pub struct jjrg_RetireResult {
    /// Path to created trophy file
    pub trophy_path: String,
    /// Path to deleted paddock file
    pub paddock_path: String,
    /// Heat silks (for commit message)
    pub silks: String,
    /// Firemark display string (for commit message)
    pub firemark: String,
}

/// Arguments for the furlough operation
pub struct jjrg_FurloughArgs {
    /// Firemark of heat to furlough
    pub firemark: String,
    /// Set status to racing (mutually exclusive with stabled)
    pub racing: bool,
    /// Set status to stabled (mutually exclusive with racing)
    pub stabled: bool,
    /// New silks (rename heat)
    pub silks: Option<String>,
}

/// Arguments for the restring operation
pub struct jjrg_RestringArgs {
    /// Source heat Firemark
    pub source_firemark: String,
    /// Destination heat Firemark
    pub dest_firemark: String,
    /// Coronets to transfer (in order)
    pub coronets: Vec<String>,
}

/// Mapping of old coronet to new coronet with pace metadata
#[derive(Debug, Clone)]
pub struct jjrg_RestringMapping {
    pub old_coronet: String,
    pub new_coronet: String,
    pub silks: String,
    pub state: jjrg_PaceState,
    pub spec: String,
}

/// Result of the restring operation
#[derive(Debug)]
pub struct jjrg_RestringResult {
    /// Source heat info
    pub source_firemark: String,
    pub source_silks: String,
    pub source_paddock: String,
    pub source_empty_after: bool,
    /// Destination heat info
    pub dest_firemark: String,
    pub dest_silks: String,
    pub dest_paddock: String,
    /// Drafted pace mappings (in transfer order)
    pub drafted: Vec<jjrg_RestringMapping>,
}

/// Context returned by resolve_pace — snapshot of current pace state
#[derive(Debug, Clone)]
pub struct jjrg_PaceContext {
    /// Coronet display key
    pub coronet_key: String,
    /// Parent firemark display key
    pub firemark_key: String,
    /// Current pace state
    pub state: jjrg_PaceState,
    /// Current docket text
    pub text: String,
    /// Current silks
    pub silks: String,
}
