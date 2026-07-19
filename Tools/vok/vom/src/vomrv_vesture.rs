// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM vesture recognizers - Tier 1 per-vesture declaration-site line
//! patterns (VOSMM-entity.adoc "Scan Mechanics": `fn prefix_…`,
//! `prefix_…() {`, `[[anchor]]`). Per-vesture line patterns only, leaning on
//! RCG/BCG declaration-site discipline; no tree-sitter, no syn (VOr_m7w).
//!
//! Rust, Bash, and AsciiDoc attribute/anchor declarations are recognized
//! here. A rivet-shape recognizer claims opaque `RBr_`/`VOr_`/`JJr_` IDs as
//! their own kind regardless of declaration site: a rivet has one true
//! declaration (its `[[anchor]]`) but many bare citation sites, so requiring
//! the declaration site would leave every citation reading as estray.

use std::sync::LazyLock;

use regex::Regex;

// Rust: `fn`/`const`/`static`/`struct`/`enum`/`trait`/`type`/`mod` item
// declarations, optionally `pub`/`pub(crate)`-qualified (RCG "Declaration
// Naming"; `static` rides alongside `const` though the guide's table
// predates it).
static ZVOMRV_RUST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:fn|const|static|struct|enum|trait|type|mod)\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap()
});

// Bash: `name() {` function declarations (BCG "Function Patterns").
static ZVOMRV_BASH_FN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)\s*\(\)\s*\{").unwrap());

// Bash: top-level `NAME=value` constants, optionally `readonly`/`export`-qualified.
// The epithet half rides lowercase in this codebase's constants
// (`BUBC_band_admission`) as often as full SCREAMING_CASE (`BUC_VERBOSE`), so
// only the signet-leading capital is required.
static ZVOMRV_BASH_CONST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:readonly\s+|export\s+)?([A-Z][A-Za-z0-9_]*)=").unwrap());

// AsciiDoc: `:name:` attribute declarations (CMK "Mapping Section").
static ZVOMRV_ADOC_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^:([a-z][a-z0-9_]*):").unwrap());

// AsciiDoc: `[[name]]` anchor declarations (CMK "Anchors").
static ZVOMRV_ADOC_ANCHOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([A-Za-z_][A-Za-z0-9_]*)\]\]").unwrap());

// Rivet: 2-4 uppercase cipher letters, a lowercase `r`, and a short opaque
// tail (MCM `mcm_rivet`: "{proj}r_<opaque-tail>"). Shape-only, honoring
// VOr_m7w (mechanical, never linguistic) - no declaration-site requirement.
static ZVOMRV_RIVET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z]{2,4}r_[a-z0-9]{2,8}$").unwrap());

/// Claim every declaration a known vesture recognizes on one line. Returns
/// (signet, inscription) pairs to seat; signet and inscription are the same
/// token for the MVP (exact signet/epithet decomposition is seating-validator
/// work, VOSMM "Seating Validators", not this pace's concern).
pub fn vomrv_claim_line(line: &str) -> Vec<(String, String)> {
    let mut claims = Vec::new();

    if let Some(cap) = ZVOMRV_RUST.captures(line) {
        zvomrv_push(&mut claims, &cap[1]);
    }
    if let Some(cap) = ZVOMRV_BASH_FN.captures(line) {
        zvomrv_push(&mut claims, &cap[1]);
    }
    if let Some(cap) = ZVOMRV_BASH_CONST.captures(line) {
        zvomrv_push(&mut claims, &cap[1]);
    }
    if let Some(cap) = ZVOMRV_ADOC_ATTR.captures(line) {
        zvomrv_push(&mut claims, &cap[1]);
    }
    for cap in ZVOMRV_ADOC_ANCHOR.captures_iter(line) {
        zvomrv_push(&mut claims, &cap[1]);
    }

    claims
}

/// Claim a whole token by rivet shape alone.
pub fn vomrv_claim_rivet(token: &str) -> bool {
    ZVOMRV_RIVET.is_match(token)
}

pub(crate) fn zvomrv_push(claims: &mut Vec<(String, String)>, name: &str) {
    claims.push((name.to_string(), name.to_string()));
}

// eof
