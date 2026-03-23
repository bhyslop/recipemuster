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

const JJRZ_SLUG_SLATE: &str = "slate";
const JJRZ_SLUG_RESLATE: &str = "reslate";
const JJRZ_SLUG_PADDOCK: &str = "paddock";
const JJRZ_SLUG_PACE: &str = "pace";

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
}

/// All defined slug values
pub const JJRZ_ALL_SLUGS: &[jjrz_Slug] = &[
    jjrz_Slug::Slate,
    jjrz_Slug::Reslate,
    jjrz_Slug::Paddock,
    jjrz_Slug::Pace,
];

impl jjrz_Slug {
    /// Wire-format string for this slug
    pub fn jjrz_as_str(&self) -> &'static str {
        match self {
            Self::Slate => JJRZ_SLUG_SLATE,
            Self::Reslate => JJRZ_SLUG_RESLATE,
            Self::Paddock => JJRZ_SLUG_PADDOCK,
            Self::Pace => JJRZ_SLUG_PACE,
        }
    }

    /// Parse slug from wire-format string (exact match only)
    pub fn jjrz_from_str(s: &str) -> Option<jjrz_Slug> {
        match s {
            JJRZ_SLUG_SLATE => Some(Self::Slate),
            JJRZ_SLUG_RESLATE => Some(Self::Reslate),
            JJRZ_SLUG_PADDOCK => Some(Self::Paddock),
            JJRZ_SLUG_PACE => Some(Self::Pace),
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
        }
    }
}

impl fmt::Display for jjrz_Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.jjrz_as_str())
    }
}

/// Two-level map gazette with freeze-on-disclosure semantics
#[derive(Debug)]
pub struct jjrz_Gazette {
    vocabulary: Vec<jjrz_Slug>,
    notices: BTreeMap<jjrz_Slug, BTreeMap<String, String>>,
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
        let vocab_set: std::collections::HashSet<jjrz_Slug> = vocabulary.iter().copied().collect();

        // (slug, lede, content_lines, header_line_number)
        let mut current: Option<(jjrz_Slug, String, Vec<String>, usize)> = None;

        for (line_idx, line) in markdown.lines().enumerate() {
            let line_num = line_idx + 1;

            if zjjrz_is_notice_boundary(line) {
                if let Some((slug, lede, content_lines, hdr_line)) = current.take() {
                    zjjrz_finalize_notice(slug, &lede, &content_lines, hdr_line, &mut notices, &mut diagnostics);
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
                        let suggestion = zjjrz_near_match(slug_str, vocabulary);
                        let msg = match suggestion {
                            Some(near) => format!(
                                "Line {}: unknown slug '{}' (did you mean '{}'?)",
                                line_num, slug_str, near
                            ),
                            None => format!(
                                "Line {}: unknown slug '{}' (valid: {})",
                                line_num, slug_str, zjjrz_format_vocab(vocabulary)
                            ),
                        };
                        diagnostics.push(msg);
                    }
                }
            } else if let Some((_, _, ref mut content_lines, _)) = current {
                content_lines.push(line.to_string());
            }
        }

        if let Some((slug, lede, content_lines, hdr_line)) = current.take() {
            zjjrz_finalize_notice(slug, &lede, &content_lines, hdr_line, &mut notices, &mut diagnostics);
        }

        if diagnostics.is_empty() {
            Ok(jjrz_Gazette {
                vocabulary: vocabulary.to_vec(),
                notices,
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
        Ok(())
    }

    /// Retrieve all notices with the given slug.
    /// Discloses notice map contents (freezes gazette permanently).
    pub fn jjrz_query_by_slug(&self, slug: jjrz_Slug) -> Vec<(String, String)> {
        self.frozen.set(true);
        self.notices.get(&slug)
            .map(|inner| inner.iter().map(|(l, c)| (l.clone(), c.clone())).collect())
            .unwrap_or_default()
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
/// Validates: exactly one slate notice, non-empty lede (silks).
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
    Ok((silks, docket))
}

/// Parse gazette input for revise_docket (reslate) operation.
/// Returns Vec of (coronet, docket) pairs for mass reslate support.
/// Validates: at least one reslate notice, all ledes (coronets) non-empty.
pub fn jjrz_parse_reslate_input(markdown: &str) -> Result<Vec<(String, String)>, String> {
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Reslate], markdown)
        .map_err(|diags| diags.join("\n"))?;
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Reslate);
    if entries.is_empty() {
        return Err("No reslate notice found in gazette input".to_string());
    }
    for (coronet, _) in &entries {
        if coronet.is_empty() {
            return Err("Reslate notice missing lede (coronet)".to_string());
        }
    }
    Ok(entries)
}

/// Parse gazette input for paddock setter operation.
/// Returns (firemark, content) from a single paddock notice.
/// Validates: exactly one paddock notice, non-empty lede (firemark).
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
    Ok((firemark, content))
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
    }
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

/// Find nearest vocabulary slug within edit distance 2
pub(crate) fn zjjrz_near_match(unknown: &str, vocab: &[jjrz_Slug]) -> Option<&'static str> {
    let mut best: Option<(&'static str, usize)> = None;
    for slug in vocab {
        let dist = zjjrz_edit_distance(unknown, slug.jjrz_as_str());
        if dist <= 2 {
            match best {
                Some((_, d)) if dist < d => best = Some((slug.jjrz_as_str(), dist)),
                None => best = Some((slug.jjrz_as_str(), dist)),
                _ => {}
            }
        }
    }
    best.map(|(s, _)| s)
}

/// Levenshtein edit distance
pub(crate) fn zjjrz_edit_distance(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let (m, n) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}
