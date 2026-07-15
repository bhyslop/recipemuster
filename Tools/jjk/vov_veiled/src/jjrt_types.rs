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
pub const JJRG_STATE_BRIDLED: &str = "bridled";
pub const JJRG_STATE_COMPLETE: &str = "complete";
pub const JJRG_STATE_ABANDONED: &str = "abandoned";

/// Pace state values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum jjrg_PaceState {
    // Reprieve JJr_a7c — `primed` is the V3-era on-disk token for the retired
    // V3 bridled state; it demotes to Rough at the deserialize boundary under
    // the V3→V4 episode (frozen reference: jjrt_v3_types.rs). It collides with
    // nothing current — no live write path emits it.
    #[serde(rename = "jjgte_rough", alias = "primed")]
    Rough,
    /// Designated for execution: a frontier agent judged this pace mechanically
    /// defined and recorded its execution tier (and optionally effort) on the
    /// tack. A distinct open state — excluded by the rough filter, included by
    /// remaining, resolved by next-actionable. The wire token is a sanctioned
    /// re-mint of the retired V3-era state's token (eviction sweep proven by
    /// repo-wide grep at the re-mint; the old sense demoted through the
    /// now-stripped bridle-retirement reprieve episode).
    #[serde(rename = "jjgte_bridled")]
    Bridled,
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
            jjrg_PaceState::Bridled => JJRG_STATE_BRIDLED,
            jjrg_PaceState::Complete => JJRG_STATE_COMPLETE,
            jjrg_PaceState::Abandoned => JJRG_STATE_ABANDONED,
        }
    }

    /// The JJS0 `jjdpe_resolved` predicate: this pace requires no further
    /// action. Its negation is *actionable* — the open set an orient mounts
    /// and a `--first` positioning aims at.
    ///
    /// Query logic asks this predicate rather than matching state names, so a
    /// future state declares its value here once and every caller inherits it;
    /// the `--first` sites drifted precisely because they matched `Rough` by
    /// name and never learned about `Bridled`.
    pub fn jjrg_is_resolved(&self) -> bool {
        match self {
            jjrg_PaceState::Rough | jjrg_PaceState::Bridled => false,
            jjrg_PaceState::Complete | jjrg_PaceState::Abandoned => true,
        }
    }
}

/// Canonical human display labels for designation tiers — same pattern as the
/// pace-state labels above: one display home, distinct from the wire tokens.
pub const JJRG_TIER_HAIKU: &str = "haiku";
pub const JJRG_TIER_SONNET: &str = "sonnet";
pub const JJRG_TIER_OPUS: &str = "opus";
pub const JJRG_TIER_FABLE: &str = "fable";

/// Designation tier — the vendor model-family word a bridle designation records
/// and the orient/record/landing guard matches against the caller's model wire
/// param. Tiers, never model IDs: IDs drift, tiers hold. The designable set is
/// exactly these four; the gpt/codex and gemini families are recognized by the
/// caller-tier extractor for diagnostics but are never designable.
///
/// Wire tokens ride the `jjgde_` family — gallops designation enum values,
/// shared by tier and effort (the two designation vocabularies are disjoint
/// word sets, so each token stays unique; `grep jjgde_` is the census).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum jjrg_Tier {
    #[serde(rename = "jjgde_haiku")]
    Haiku,
    #[serde(rename = "jjgde_sonnet")]
    Sonnet,
    #[serde(rename = "jjgde_opus")]
    Opus,
    #[serde(rename = "jjgde_fable")]
    Fable,
}

impl jjrg_Tier {
    /// Human display label (the vendor family word) — single display home.
    pub fn jjrg_as_str(&self) -> &'static str {
        match self {
            jjrg_Tier::Haiku => JJRG_TIER_HAIKU,
            jjrg_Tier::Sonnet => JJRG_TIER_SONNET,
            jjrg_Tier::Opus => JJRG_TIER_OPUS,
            jjrg_Tier::Fable => JJRG_TIER_FABLE,
        }
    }

    /// Parse a designation word (the display label) into the tier — the
    /// recognized-word validation for the bridle command's `tier` param.
    pub fn jjrg_from_word(word: &str) -> Result<Self, String> {
        match word {
            JJRG_TIER_HAIKU => Ok(jjrg_Tier::Haiku),
            JJRG_TIER_SONNET => Ok(jjrg_Tier::Sonnet),
            JJRG_TIER_OPUS => Ok(jjrg_Tier::Opus),
            JJRG_TIER_FABLE => Ok(jjrg_Tier::Fable),
            other => Err(format!(
                "unknown tier '{}' — designable tiers: {}, {}, {}, {}",
                other, JJRG_TIER_HAIKU, JJRG_TIER_SONNET, JJRG_TIER_OPUS, JJRG_TIER_FABLE
            )),
        }
    }
}

/// Canonical human display labels for designation efforts — one display home.
pub const JJRG_EFFORT_LOW: &str = "low";
pub const JJRG_EFFORT_MEDIUM: &str = "medium";
pub const JJRG_EFFORT_HIGH: &str = "high";
pub const JJRG_EFFORT_XHIGH: &str = "xhigh";
pub const JJRG_EFFORT_MAX: &str = "max";

/// Designation effort — an optional reasoning-effort word recorded beside the
/// tier at bridle time. Anchored transparently on Anthropic's effort
/// classification as the product surfaces it (verified against the live
/// product surface 2026-07-07: low, medium, high, xhigh, max); JJ mints no
/// effort vocabulary of its own. Effort never rides the MCP wire and is never
/// guarded — it is designation-and-dispatch data, consumed by the dispatch
/// layer when it lands. Wire tokens share the `jjgde_` designation family
/// with the tier (see jjrg_Tier).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum jjrg_Effort {
    #[serde(rename = "jjgde_low")]
    Low,
    #[serde(rename = "jjgde_medium")]
    Medium,
    #[serde(rename = "jjgde_high")]
    High,
    #[serde(rename = "jjgde_xhigh")]
    Xhigh,
    #[serde(rename = "jjgde_max")]
    Max,
}

impl jjrg_Effort {
    /// Human display label (Anthropic's effort word) — single display home.
    pub fn jjrg_as_str(&self) -> &'static str {
        match self {
            jjrg_Effort::Low => JJRG_EFFORT_LOW,
            jjrg_Effort::Medium => JJRG_EFFORT_MEDIUM,
            jjrg_Effort::High => JJRG_EFFORT_HIGH,
            jjrg_Effort::Xhigh => JJRG_EFFORT_XHIGH,
            jjrg_Effort::Max => JJRG_EFFORT_MAX,
        }
    }

    /// Parse an effort word (the display label) — the recognized-word
    /// validation for the bridle command's optional `effort` param.
    pub fn jjrg_from_word(word: &str) -> Result<Self, String> {
        match word {
            JJRG_EFFORT_LOW => Ok(jjrg_Effort::Low),
            JJRG_EFFORT_MEDIUM => Ok(jjrg_Effort::Medium),
            JJRG_EFFORT_HIGH => Ok(jjrg_Effort::High),
            JJRG_EFFORT_XHIGH => Ok(jjrg_Effort::Xhigh),
            JJRG_EFFORT_MAX => Ok(jjrg_Effort::Max),
            other => Err(format!(
                "unknown effort '{}' — recognized efforts: {}, {}, {}, {}, {}",
                other, JJRG_EFFORT_LOW, JJRG_EFFORT_MEDIUM, JJRG_EFFORT_HIGH,
                JJRG_EFFORT_XHIGH, JJRG_EFFORT_MAX
            )),
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
    /// Designation tier, present while the pace is bridled and persisting
    /// through close as provenance of the executing session. Serialized only
    /// when present so an untouched store stays byte-canonical — the additive
    /// carve-out that lets this field ride without a reprieve episode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jjgtn_tier")]
    pub tier: Option<jjrg_Tier>,
    /// Optional designation effort beside the tier — same serialized-only-when-
    /// present posture, so a designation without effort is byte-identical to
    /// the tier-only form. Never present without a tier.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jjgtn_effort")]
    pub effort: Option<jjrg_Effort>,
    /// Docket text as a line array — one element per physical line, so pretty-JSON
    /// decomposes the docket line-by-line and git merges it at line granularity.
    /// Custom deserialize tolerates the legacy string shape (the tack-text→lines
    /// reprieve episode — rivet JJr_a7c, JJS0 jjdz_reprieve): an on-disk
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

/// Stand-in for the tier of a Bridled tack whose tier is absent — a shape the
/// designation gate forbids, so it can only arise from a hand-edited store.
/// Displayed rather than swallowed: a visibly odd cell is the loud report.
pub const JJRG_TIER_ABSENT: &str = "?";

impl jjrg_Tack {
    /// The state label a reader sees, tier folded in while bridled (`bridled
    /// opus`). One home for every listing surface — the parade table, the
    /// orient `Next:` line, the coronets bracket tag — so the shape cannot
    /// drift between them. Effort is designation-and-dispatch data and is
    /// deliberately not displayed.
    pub fn jjrg_state_label(&self) -> String {
        match self.state {
            jjrg_PaceState::Bridled => format!(
                "{} {}",
                self.state.jjrg_as_str(),
                self.tier.map(|t| t.jjrg_as_str()).unwrap_or(JJRG_TIER_ABSENT)
            ),
            _ => self.state.jjrg_as_str().to_string(),
        }
    }
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
    #[serde(rename = "jjghn_paces")]
    pub paces: BTreeMap<String, jjrg_Pace>,
}

/// Root Gallops structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Gallops {
    #[serde(rename = "jjgrn_next_heat_seed")]
    pub next_heat_seed: String,
    /// Global pace-mint seed — the single next-Coronet to allocate for the whole
    /// gallops (JJS0 jjdgm_pace_seed). Coronets are flat global ids minted from
    /// here under the commit lock, never per-heat. `#[serde(default)]` so an
    /// old-format store (which lacks it and carries per-heat jjghn_next_pace_seed
    /// instead) still deserializes; the reprieve write-forward then founds it
    /// (rivet JJr_a7c). Always serialized — canonical form always carries it.
    #[serde(default, rename = "jjgrn_next_pace_seed")]
    pub next_pace_seed: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "jjgrn_heat_order")]
    pub heat_order: Vec<String>,
    #[serde(rename = "jjgrn_heats")]
    pub heats: BTreeMap<String, jjrg_Heat>,
    /// Chat-history retention policy — the ISO date (YYYY-MM-DD) since which this install is
    /// permitted to capture its own chat transcripts into the project. Absent or empty means off
    /// (the shareable default: the binary can be handed to a friend without absorbing their chat
    /// history). Held as a RAW string and never serde-typed: a malformed date must not make the
    /// gallops fail to parse — bad config never makes the store illegitimate. Classification and
    /// validation happen at read via `jjri_retention_state`; the value is consumed by the capture
    /// mechanism and set by its operator-facing setter, both of which land separately. Optional +
    /// skip-when-None keeps an off store byte-identical, so adding this field needs no reprieve
    /// episode (the easy mirror of a field removal).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jjgrn_retention_since")]
    pub retention_since: Option<String>,
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
    /// Current designation tier (present while bridled; provenance after close)
    pub tier: Option<jjrg_Tier>,
    /// Current designation effort (rides only beside a tier)
    pub effort: Option<jjrg_Effort>,
    /// Current docket text
    pub text: String,
    /// Current silks
    pub silks: String,
}
