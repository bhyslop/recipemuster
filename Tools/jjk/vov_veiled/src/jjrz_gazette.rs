// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gazette — two-level map with freeze-on-disclosure semantics
//!
//! Implements the JJSCGZ-gazette.adoc specification.
//! A gazette maps (slug, lede) pairs to content strings,
//! with a fixed vocabulary of accepted slugs and permanent
//! freeze-on-disclosure semantics.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt;

// --- String boundary consts for slug wire format ---

pub(crate) const JJRZ_SLUG_SLATE: &str = "jjezs_slate";
pub(crate) const JJRZ_SLUG_RESLATE: &str = "jjezs_reslate";
pub(crate) const JJRZ_SLUG_PADDOCK: &str = "jjezs_paddock";
pub(crate) const JJRZ_SLUG_PACE: &str = "jjezs_pace";
pub(crate) const JJRZ_SLUG_HALTER: &str = "jjezs_halter";
pub(crate) const JJRZ_SLUG_STEEPLECHASE: &str = "jjezs_steeplechase";

/// Slug direction — metadata for how each slug is used in operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrz_Direction {
    Input,
    Output,
    Bidirectional,
}

/// Slug enumeration — category identifiers for gazette notices
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum jjrz_Slug {
    Slate,
    Reslate,
    Paddock,
    Pace,
    Halter,
    Steeplechase,
}

/// All defined slug values
pub const JJRZ_ALL_SLUGS: &[jjrz_Slug] = &[
    jjrz_Slug::Slate,
    jjrz_Slug::Reslate,
    jjrz_Slug::Paddock,
    jjrz_Slug::Pace,
    jjrz_Slug::Halter,
    jjrz_Slug::Steeplechase,
];

impl jjrz_Slug {
    /// Wire-format string for this slug
    pub fn jjrz_as_str(&self) -> &'static str {
        match self {
            Self::Slate => JJRZ_SLUG_SLATE,
            Self::Reslate => JJRZ_SLUG_RESLATE,
            Self::Paddock => JJRZ_SLUG_PADDOCK,
            Self::Pace => JJRZ_SLUG_PACE,
            Self::Halter => JJRZ_SLUG_HALTER,
            Self::Steeplechase => JJRZ_SLUG_STEEPLECHASE,
        }
    }

    /// Parse slug from wire-format string (exact match only)
    pub fn jjrz_from_str(s: &str) -> Option<jjrz_Slug> {
        match s {
            JJRZ_SLUG_SLATE => Some(Self::Slate),
            JJRZ_SLUG_RESLATE => Some(Self::Reslate),
            JJRZ_SLUG_PADDOCK => Some(Self::Paddock),
            JJRZ_SLUG_PACE => Some(Self::Pace),
            JJRZ_SLUG_HALTER => Some(Self::Halter),
            JJRZ_SLUG_STEEPLECHASE => Some(Self::Steeplechase),
            _ => None,
        }
    }

    /// Direction metadata per spec
    pub fn jjrz_direction(&self) -> jjrz_Direction {
        match self {
            Self::Slate => jjrz_Direction::Input,
            Self::Reslate => jjrz_Direction::Input,
            Self::Paddock => jjrz_Direction::Bidirectional,
            Self::Pace => jjrz_Direction::Output,
            Self::Halter => jjrz_Direction::Input,
            Self::Steeplechase => jjrz_Direction::Output,
        }
    }
}

impl fmt::Display for jjrz_Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.jjrz_as_str())
    }
}

/// Two-level map gazette with freeze-on-disclosure semantics
///
/// `notices` keys notices by (slug, lede) for dedup and content lookup; that
/// keying sorts ledes lexically and so DROPS the order in which notices arrived.
/// `order` is the companion record of (slug, lede) in file/insertion order, so
/// callers that need the authored sequence — multi-slate batches, where notice
/// order is pace order — can recover it via jjrz_query_by_slug_ordered.
#[derive(Debug)]
pub struct jjrz_Gazette {
    vocabulary: Vec<jjrz_Slug>,
    notices: BTreeMap<jjrz_Slug, BTreeMap<String, String>>,
    order: Vec<(jjrz_Slug, String)>,
    frozen: Cell<bool>,
}

impl jjrz_Gazette {
    /// Construct a frozen gazette from raw markdown per the wire format.
    ///
    /// All validation errors are collected and returned as a diagnostic list
    /// rather than failing on the first.
    pub fn jjrz_parse(vocabulary: &[jjrz_Slug], markdown: &str) -> Result<jjrz_Gazette, Vec<String>> {
        let mut diagnostics: Vec<String> = Vec::new();
        let mut notices: BTreeMap<jjrz_Slug, BTreeMap<String, String>> = BTreeMap::new();
        let mut order: Vec<(jjrz_Slug, String)> = Vec::new();
        let vocab_set: std::collections::HashSet<jjrz_Slug> = vocabulary.iter().copied().collect();

        // (slug, lede, content_lines, header_line_number)
        let mut current: Option<(jjrz_Slug, String, Vec<String>, usize)> = None;
        let mut in_fence = false;

        for (line_idx, line) in markdown.lines().enumerate() {
            let line_num = line_idx + 1;

            if line.starts_with("```") {
                in_fence = !in_fence;
            }

            if !in_fence && zjjrz_is_notice_boundary(line) {
                if let Some((slug, lede, content_lines, hdr_line)) = current.take() {
                    zjjrz_finalize_notice(slug, &lede, &content_lines, hdr_line, &mut notices, &mut order, &mut diagnostics);
                }

                let after_hash = line[1..].trim();
                if after_hash.is_empty() {
                    diagnostics.push(format!("Line {}: malformed header (no slug after #)", line_num));
                    continue;
                }

                let (slug_str, lede) = match after_hash.split_once(char::is_whitespace) {
                    Some((s, l)) => (s, l.trim().to_string()),
                    None => (after_hash, String::new()),
                };

                match jjrz_Slug::jjrz_from_str(slug_str) {
                    Some(slug) if vocab_set.contains(&slug) => {
                        current = Some((slug, lede, Vec::new(), line_num));
                    }
                    Some(slug) => {
                        diagnostics.push(format!(
                            "Line {}: slug '{}' not in vocabulary (accepted: {})",
                            line_num, slug, zjjrz_format_vocab(vocabulary)
                        ));
                    }
                    None => {
                        diagnostics.push(format!(
                            "Line {}: unknown slug '{}' (valid: {})",
                            line_num, slug_str, zjjrz_format_vocab(vocabulary)
                        ));
                    }
                }
            } else if let Some((_, _, ref mut content_lines, _)) = current {
                content_lines.push(line.to_string());
            }
        }

        if let Some((slug, lede, content_lines, hdr_line)) = current.take() {
            zjjrz_finalize_notice(slug, &lede, &content_lines, hdr_line, &mut notices, &mut order, &mut diagnostics);
        }

        if diagnostics.is_empty() {
            Ok(jjrz_Gazette {
                vocabulary: vocabulary.to_vec(),
                notices,
                order,
                frozen: Cell::new(true),
            })
        } else {
            Err(diagnostics)
        }
    }

    /// Construct an unfrozen, empty gazette
    pub fn jjrz_build(vocabulary: &[jjrz_Slug]) -> jjrz_Gazette {
        jjrz_Gazette {
            vocabulary: vocabulary.to_vec(),
            notices: BTreeMap::new(),
            order: Vec::new(),
            frozen: Cell::new(false),
        }
    }

    /// Add a notice to the gazette.
    ///
    /// Fatal if frozen, if slug is not in vocabulary, or if (slug, lede) already exists.
    pub fn jjrz_add(&mut self, slug: jjrz_Slug, lede: &str, content: &str) -> Result<(), String> {
        if self.frozen.get() {
            return Err("Cannot add to frozen gazette".to_string());
        }
        if !self.vocabulary.contains(&slug) {
            return Err(format!(
                "Slug '{}' not in vocabulary (accepted: {})",
                slug, zjjrz_format_vocab(&self.vocabulary)
            ));
        }
        let inner = self.notices.entry(slug).or_default();
        if inner.contains_key(lede) {
            let lede_display = if lede.is_empty() { "<absent>" } else { lede };
            return Err(format!("Duplicate key ({}, {:?})", slug, lede_display));
        }
        inner.insert(lede.to_string(), content.to_string());
        self.order.push((slug, lede.to_string()));
        Ok(())
    }

    /// Retrieve all notices with the given slug, in lede (lexical) order.
    /// Discloses notice map contents (freezes gazette permanently).
    pub fn jjrz_query_by_slug(&self, slug: jjrz_Slug) -> Vec<(String, String)> {
        self.frozen.set(true);
        self.notices.get(&slug)
            .map(|inner| inner.iter().map(|(l, c)| (l.clone(), c.clone())).collect())
            .unwrap_or_default()
    }

    /// Retrieve all notices with the given slug, in file/insertion order.
    /// Use when authored sequence is load-bearing — multi-slate batches, where
    /// notice order is pace order. Discloses notice map contents (freezes).
    pub fn jjrz_query_by_slug_ordered(&self, slug: jjrz_Slug) -> Vec<(String, String)> {
        self.frozen.set(true);
        let inner = match self.notices.get(&slug) {
            Some(m) => m,
            None => return Vec::new(),
        };
        self.order.iter()
            .filter(|(s, _)| *s == slug)
            .filter_map(|(_, lede)| inner.get(lede).map(|c| (lede.clone(), c.clone())))
            .collect()
    }

    /// Retrieve all notices.
    /// Discloses notice map contents (freezes gazette permanently).
    pub fn jjrz_query_all(&self) -> Vec<(jjrz_Slug, String, String)> {
        self.frozen.set(true);
        let mut result = Vec::new();
        for (&slug, inner) in &self.notices {
            for (lede, content) in inner {
                result.push((slug, lede.clone(), content.clone()));
            }
        }
        result
    }

    /// Produce formatted markdown per the wire format.
    /// Discloses notice map contents (freezes gazette permanently).
    pub fn jjrz_emit(&self) -> String {
        self.frozen.set(true);
        let mut output = String::new();
        let mut first = true;
        for (&slug, inner) in &self.notices {
            for (lede, content) in inner {
                if !first {
                    output.push('\n');
                }
                first = false;
                if lede.is_empty() {
                    output.push_str(&format!("# {}\n", slug));
                } else {
                    output.push_str(&format!("# {} {}\n", slug, lede));
                }
                if !content.is_empty() {
                    output.push('\n');
                    output.push_str(content);
                    output.push('\n');
                }
            }
        }
        output
    }

    /// Check if gazette is frozen
    pub fn jjrz_is_frozen(&self) -> bool {
        self.frozen.get()
    }

    /// Get the vocabulary (does not trigger freeze)
    pub fn jjrz_vocabulary(&self) -> &[jjrz_Slug] {
        &self.vocabulary
    }
}

// --- Operation input parsing ---

/// Parse gazette input for enroll (slate) operation.
/// Returns (silks, docket) from a single slate notice.
/// Validates: exactly one slate notice, non-empty lede (silks), non-empty body.
pub fn jjrz_parse_slate_input(markdown: &str) -> Result<(String, String), String> {
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], markdown)
        .map_err(|diags| diags.join("\n"))?;
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    if entries.is_empty() {
        return Err("No slate notice found in gazette input".to_string());
    }
    if entries.len() > 1 {
        return Err(format!("Expected one slate notice, got {}", entries.len()));
    }
    let (silks, docket) = entries.into_iter().next().unwrap();
    if silks.is_empty() {
        return Err("Slate notice missing lede (silks)".to_string());
    }
    if docket.is_empty() {
        return Err(zjjrz_empty_body_error(jjrz_Slug::Slate, &silks));
    }
    Ok((silks, docket))
}

/// Parse gazette input for revise_docket (reslate) operation.
/// Returns Vec of (coronet, docket) pairs for mass reslate support.
/// Validates: at least one reslate notice, all ledes (coronets) non-empty,
/// all bodies non-empty.
pub fn jjrz_parse_reslate_input(markdown: &str) -> Result<Vec<(String, String)>, String> {
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Reslate], markdown)
        .map_err(|diags| diags.join("\n"))?;
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Reslate);
    if entries.is_empty() {
        return Err("No reslate notice found in gazette input".to_string());
    }
    for (coronet, docket) in &entries {
        if coronet.is_empty() {
            return Err("Reslate notice missing lede (coronet)".to_string());
        }
        if docket.is_empty() {
            return Err(zjjrz_empty_body_error(jjrz_Slug::Reslate, coronet));
        }
    }
    Ok(entries)
}

/// Parse gazette input for the read-path target selection (orient, show).
/// Returns the Vec of target ledes — each a firemark (heat) or coronet (pace),
/// self-typed downstream by length. Bodies are ignored: a halter notice is
/// body-less, its lede carrying the whole signal.
/// Validates: at least one halter notice, every lede non-empty.
/// Cardinality beyond "at least one" is the caller's to enforce — orient
/// mounts exactly one target, show accepts the heterogeneous set.
pub fn jjrz_parse_halter_input(markdown: &str) -> Result<Vec<String>, String> {
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Halter], markdown)
        .map_err(|diags| diags.join("\n"))?;
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Halter);
    if entries.is_empty() {
        return Err("No halter notice found in gazette input".to_string());
    }
    let mut targets = Vec::with_capacity(entries.len());
    for (lede, _body) in entries {
        if lede.is_empty() {
            return Err("Halter notice missing lede (firemark or coronet)".to_string());
        }
        targets.push(lede);
    }
    Ok(targets)
}

/// Parse gazette input for the curry (paddock revision) operation.
/// Returns (firemark, content) from a single paddock notice.
/// Validates: exactly one paddock notice, non-empty lede (firemark),
/// non-empty body.
pub fn jjrz_parse_paddock_input(markdown: &str) -> Result<(String, String), String> {
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], markdown)
        .map_err(|diags| diags.join("\n"))?;
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    if entries.is_empty() {
        return Err("No paddock notice found in gazette input".to_string());
    }
    if entries.len() > 1 {
        return Err(format!("Expected one paddock notice, got {}", entries.len()));
    }
    let (firemark, content) = entries.into_iter().next().unwrap();
    if firemark.is_empty() {
        return Err("Paddock notice missing lede (firemark)".to_string());
    }
    if content.is_empty() {
        return Err(zjjrz_empty_body_error(jjrz_Slug::Paddock, &firemark));
    }
    Ok((firemark, content))
}

/// A heterogeneous single-heat gazette batch: at most one paddock revision,
/// zero-or-more reslates (order irrelevant — each targets a coronet), and
/// zero-or-more slates in file order (notice order = pace order). The
/// same-heat guard is the caller's (firemarks are resolved in jjrm_mcp); this
/// parser only shapes and shallow-validates the notices.
#[derive(Debug)]
pub struct jjrz_BatchInput {
    /// (firemark, content) of the lone paddock notice, if present.
    pub paddock: Option<(String, String)>,
    /// (coronet, docket) reslate pairs.
    pub reslates: Vec<(String, String)>,
    /// (silks, docket) slate pairs, in file order.
    pub slates: Vec<(String, String)>,
}

/// Parse gazette input for a mixed single-heat batch (jjx_redocket extended).
/// Accepts paddock + reslate + slate notices in one gazette_in.md.
/// Validates: at most one paddock notice; all ledes non-empty; all bodies
/// non-empty; at least one notice overall. Slate order is file order so
/// notice order is pace order.
pub fn jjrz_parse_batch_input(markdown: &str) -> Result<jjrz_BatchInput, String> {
    let g = jjrz_Gazette::jjrz_parse(
        &[jjrz_Slug::Slate, jjrz_Slug::Reslate, jjrz_Slug::Paddock],
        markdown,
    ).map_err(|diags| diags.join("\n"))?;

    let paddock_entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    if paddock_entries.len() > 1 {
        return Err(format!("Expected at most one paddock notice, got {}", paddock_entries.len()));
    }
    let paddock = match paddock_entries.into_iter().next() {
        Some((firemark, content)) => {
            if firemark.is_empty() {
                return Err("Paddock notice missing lede (firemark)".to_string());
            }
            if content.is_empty() {
                return Err(zjjrz_empty_body_error(jjrz_Slug::Paddock, &firemark));
            }
            Some((firemark, content))
        }
        None => None,
    };

    let reslates = g.jjrz_query_by_slug(jjrz_Slug::Reslate);
    for (coronet, docket) in &reslates {
        if coronet.is_empty() {
            return Err("Reslate notice missing lede (coronet)".to_string());
        }
        if docket.is_empty() {
            return Err(zjjrz_empty_body_error(jjrz_Slug::Reslate, coronet));
        }
    }

    let slates = g.jjrz_query_by_slug_ordered(jjrz_Slug::Slate);
    for (silks, docket) in &slates {
        if silks.is_empty() {
            return Err("Slate notice missing lede (silks)".to_string());
        }
        if docket.is_empty() {
            return Err(zjjrz_empty_body_error(jjrz_Slug::Slate, silks));
        }
    }

    if paddock.is_none() && reslates.is_empty() && slates.is_empty() {
        return Err("Batch gazette has no notices (expected paddock, reslate, or slate)".to_string());
    }

    Ok(jjrz_BatchInput { paddock, reslates, slates })
}

// --- Operation output building ---

/// Build gazette output for read operations (orient, parade detail, paddock read).
/// Returns emitted gazette markdown with paddock and optional pace notices.
/// Paddock lede is the firemark; pace ledes are coronets.
pub fn jjrz_build_read_output(firemark: &str, paddock_content: &str, paces: &[(&str, &str)]) -> String {
    let mut gazette = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Paddock, jjrz_Slug::Pace]);
    gazette.jjrz_add(jjrz_Slug::Paddock, firemark, paddock_content).unwrap();
    for &(coronet, docket) in paces {
        gazette.jjrz_add(jjrz_Slug::Pace, coronet, docket).unwrap();
    }
    gazette.jjrz_emit()
}

// --- Internal helpers ---

/// Check if a line is a notice boundary (# followed by whitespace).
/// Lines starting with ## or #word are NOT boundaries, preserving
/// markdown headers in gazette content.
pub(crate) fn zjjrz_is_notice_boundary(line: &str) -> bool {
    let bytes = line.as_bytes();
    bytes.len() > 1 && bytes[0] == b'#' && bytes[1].is_ascii_whitespace()
}

/// Finalize a parsed notice: trim blank lines and insert into map
fn zjjrz_finalize_notice(
    slug: jjrz_Slug,
    lede: &str,
    content_lines: &[String],
    header_line: usize,
    notices: &mut BTreeMap<jjrz_Slug, BTreeMap<String, String>>,
    order: &mut Vec<(jjrz_Slug, String)>,
    diagnostics: &mut Vec<String>,
) {
    let content = zjjrz_trim_blank_lines(content_lines);
    let inner = notices.entry(slug).or_default();
    if inner.contains_key(lede) {
        let lede_display = if lede.is_empty() { "<absent>" } else { lede };
        diagnostics.push(format!(
            "Line {}: duplicate key ({}, {:?})",
            header_line, slug, lede_display
        ));
    } else {
        inner.insert(lede.to_string(), content);
        order.push((slug, lede.to_string()));
    }
}

/// Compose the rejection for a content-bearing input notice whose body
/// arrived empty (bodies are blank-trimmed at finalize, so empty here covers
/// whitespace-only). The uniform law for the input slugs slate/reslate/
/// paddock: an empty body is a staged-by-mistake or half-authored notice,
/// never intent — executing it would mint a docket-less pace or blank a
/// living docket or paddock and auto-commit the loss. Halter is exempt
/// (body-less by design; the lede is the whole signal).
fn zjjrz_empty_body_error(slug: jjrz_Slug, lede: &str) -> String {
    let refusal = match slug {
        jjrz_Slug::Slate => "a new pace is never docket-less",
        jjrz_Slug::Reslate => "a reslate replaces the whole docket, never blanks it",
        jjrz_Slug::Paddock => "a paddock revision replaces the whole paddock, never blanks it",
        jjrz_Slug::Pace | jjrz_Slug::Halter | jjrz_Slug::Steeplechase =>
            "an input notice carries its payload in the body",
    };
    format!(
        "{} notice '{}' has an empty body — {}; author the full content beneath the notice line",
        slug, lede, refusal
    )
}

/// Trim leading and trailing blank lines, join remaining with newline
pub(crate) fn zjjrz_trim_blank_lines(lines: &[String]) -> String {
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(lines.len());
    let end = lines.iter().rposition(|l| !l.trim().is_empty()).map(|i| i + 1).unwrap_or(0);
    if start >= end {
        String::new()
    } else {
        lines[start..end].join("\n")
    }
}

/// Format vocabulary slugs for diagnostic messages
fn zjjrz_format_vocab(vocab: &[jjrz_Slug]) -> String {
    vocab.iter().map(|s| s.jjrz_as_str()).collect::<Vec<_>>().join(", ")
}

