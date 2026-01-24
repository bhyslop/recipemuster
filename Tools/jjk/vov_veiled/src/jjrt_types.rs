// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops type definitions
//!
//! Core data structures for Job Jockey heat and pace tracking.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use indexmap::IndexMap;

/// Unknown/default commit SHA (7 zeros)
///
/// Used when commit cannot be determined (git errors).
/// All commit-related code derives the expected length from this constant.
pub const JJRG_UNKNOWN_COMMIT: &str = "0000000";

/// Pace state values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum jjrg_PaceState {
    Rough,
    #[serde(alias = "primed")]
    Bridled,
    Complete,
    Abandoned,
}

/// Heat status values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum jjrg_HeatStatus {
    /// Heat is actively being worked
    Racing,
    /// Heat is paused, not actively worked
    Stabled,
    /// Heat is complete and archived (terminal state)
    Retired,
}

/// Tack record - snapshot of Pace state and plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Tack {
    pub ts: String,
    pub state: jjrg_PaceState,
    pub text: String,
    pub silks: String,
    pub commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// Pace record - discrete action within a Heat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Pace {
    pub tacks: Vec<jjrg_Tack>,
}

/// Heat record - bounded initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Heat {
    pub silks: String,
    pub creation_time: String,
    pub status: jjrg_HeatStatus,
    pub order: Vec<String>,
    pub next_pace_seed: String,
    pub paddock_file: String,
    pub paces: BTreeMap<String, jjrg_Pace>,
}

/// Root Gallops structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Gallops {
    pub next_heat_seed: String,
    pub heats: IndexMap<String, jjrg_Heat>,
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
    pub direction: Option<String>,
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

/// Arguments for the garland operation
pub struct jjrg_GarlandArgs {
    /// Firemark of heat to garland
    pub firemark: String,
}

/// Result of the garland operation
#[derive(Debug)]
pub struct jjrg_GarlandResult {
    /// Old firemark
    pub old_firemark: String,
    /// Old silks (garlanded)
    pub old_silks: String,
    /// New firemark
    pub new_firemark: String,
    /// New silks (continuation)
    pub new_silks: String,
    /// Number of paces transferred to new heat
    pub paces_transferred: usize,
    /// Number of paces retained in garlanded heat
    pub paces_retained: usize,
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
