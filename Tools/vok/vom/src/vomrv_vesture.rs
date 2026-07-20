// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VOM vesture recognizers - Tier 1 per-vesture declaration-site line
//! patterns (VOSMM-entity.adoc "Scan Mechanics": `fn prefix_…`,
//! `prefix_…() {`, `[[anchor]]`). Per-vesture line patterns only, leaning on
//! RCG/BCG declaration-site discipline; no tree-sitter, no syn (VOr_m7w).
//!
//! Envelope dispatch: a file is dressed by the vesture whose envelope claims
//! it (VOS0 "Liturgy Domains" - each vesture defines an envelope), and only
//! the dressed vesture's patterns claim declarations there. A tabtarget's
//! body is formulary (BUS0 dispatch vocabulary): its config-line assignments
//! fill protocol slots, minting nothing - tabtargets are colophons, not
//! prefix space. Markdown bodies are reference-only prose: example lines are
//! records about names, never declarations (basename stem claims still run,
//! so slash-command files keep their minted names).
//!
//! Bash constants carry a home-file rule: `NAME=` claims a declaration only
//! when the signet's head (leading segment before `_`) agrees with the
//! declaring file's own stem head by string-prefix (either a prefix of the
//! other, case-folded). A cross-family assignment is an inscription site,
//! not a declaration; an ours name left with no declaration home anywhere
//! surfaces as estray - the honest verdict for unhomed wire names.
//!
//! Rust, Bash, and AsciiDoc attribute/anchor declarations are recognized
//! here. A rivet-shape recognizer claims opaque `RBr_`/`VOr_`/`JJr_` IDs as
//! their own kind regardless of declaration site: a rivet has one true
//! declaration (its `[[anchor]]`) but many bare citation sites, so requiring
//! the declaration site would leave every citation reading as estray.

use std::path::Path;
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

/// The vesture a file wears, selected by envelope. Only the dressed
/// vesture's line patterns claim declarations in that file.
pub enum vomrv_Dress {
    /// Rust source: item declarations claim.
    Rust,
    /// Bash source: function declarations claim; constants claim under the
    /// home-file rule against the carried file-stem head.
    Bash { file_head: String },
    /// AsciiDoc source: attribute and anchor declarations claim.
    Asciidoc,
    /// Reference-only prose body - no line claims.
    Markdown,
    /// Tabtarget body is formulary - no line claims.
    TabtargetFormulary,
    /// Unrecognized envelope - no line claims.
    Foreign,
}

/// Dress a file by its envelope: `tt/` is the tabtarget vesture regardless
/// of extension; otherwise the extension selects.
pub fn vomrv_dress(path: &str) -> vomrv_Dress {
    if path.starts_with("tt/") {
        return vomrv_Dress::TabtargetFormulary;
    }
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("rs") => vomrv_Dress::Rust,
        Some("sh") => {
            let file_head = Path::new(path)
                .file_stem()
                .and_then(|s| s.to_str())
                .map(zvomrv_head)
                .unwrap_or_default()
                .to_ascii_lowercase();
            vomrv_Dress::Bash { file_head }
        }
        Some("adoc") => vomrv_Dress::Asciidoc,
        Some("md") => vomrv_Dress::Markdown,
        _ => vomrv_Dress::Foreign,
    }
}

/// Claim every declaration the file's dressed vesture recognizes on one
/// line. Returns (signet, inscription) pairs to seat; signet and inscription
/// are the same token for the MVP (exact signet/epithet decomposition is
/// seating-validator work, VOSMM "Seating Validators").
pub fn vomrv_claim_line(dress: &vomrv_Dress, line: &str) -> Vec<(String, String)> {
    let mut claims = Vec::new();

    match dress {
        vomrv_Dress::Rust => {
            if let Some(cap) = ZVOMRV_RUST.captures(line) {
                zvomrv_push(&mut claims, &cap[1]);
            }
        }
        vomrv_Dress::Bash { file_head } => {
            if let Some(cap) = ZVOMRV_BASH_FN.captures(line) {
                zvomrv_push(&mut claims, &cap[1]);
            }
            if let Some(cap) = ZVOMRV_BASH_CONST.captures(line) {
                if zvomrv_family_agrees(&cap[1], file_head) {
                    zvomrv_push(&mut claims, &cap[1]);
                }
            }
        }
        vomrv_Dress::Asciidoc => {
            if let Some(cap) = ZVOMRV_ADOC_ATTR.captures(line) {
                zvomrv_push(&mut claims, &cap[1]);
            }
            for cap in ZVOMRV_ADOC_ANCHOR.captures_iter(line) {
                zvomrv_push(&mut claims, &cap[1]);
            }
        }
        vomrv_Dress::Markdown | vomrv_Dress::TabtargetFormulary | vomrv_Dress::Foreign => {}
    }

    claims
}

/// Claim a whole token by rivet shape alone.
pub fn vomrv_claim_rivet(token: &str) -> bool {
    ZVOMRV_RIVET.is_match(token)
}

/// Leading segment before the first `_`.
pub(crate) fn zvomrv_head(token: &str) -> &str {
    token.split('_').next().unwrap_or(token)
}

// Home-file agreement: the signet's head and the file-stem head, case-folded,
// where either is a prefix of the other. Bidirectional so a numbered or
// 0-trick module (`rbfc0_cli.sh`) still homes its family's constants.
pub(crate) fn zvomrv_family_agrees(signet: &str, file_head: &str) -> bool {
    if file_head.is_empty() {
        return false;
    }
    let signet_head = zvomrv_head(signet).to_ascii_lowercase();
    signet_head.starts_with(file_head) || file_head.starts_with(&signet_head)
}

pub(crate) fn zvomrv_push(claims: &mut Vec<(String, String)>, name: &str) {
    claims.push((name.to_string(), name.to_string()));
}

// eof
