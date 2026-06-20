// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Scout command - Search across heats and paces
//!
//! A pattern containing regex metacharacters runs as a single regex pass;
//! a plain pattern runs a three-tier word search (whole, prefix, substring).
//! Heats are searched at the heat level (heat silks + paddock) independent
//! of pace count, then per-pace (silks, docket text).

use clap::Args;
use regex::{
    Regex,
    RegexBuilder,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use vvc::{
    vvco_Output,
    vvco_err,
    vvco_out,
};

const JJRSC_CMD_NAME_SCOUT: &str = "jjx_scout";

use crate::jjrg_gallops::{
    jjrg_Gallops as Gallops,
    jjrg_lines_to_text,
    jjrg_PaceState as PaceState,
};
use crate::jjri_io::jjri_paddock_path;

/// Characters whose presence in a pattern selects regex mode
pub(crate) const ZJJRSC_REGEX_METACHARS: &[char] = &['.', '*', '+', '?', '[', ']', '(', ')', '{', '}', '|', '^', '$', '\\'];

/// Tier provenance markers, indexed by tier position (plain mode)
pub(crate) const ZJJRSC_TIER_MARKERS: [&str; 3] = ["whole", "prefix", "substring"];

/// Arguments for jjx_scout command
#[derive(Args, Debug)]
pub struct jjrsc_ScoutArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Search pattern (regex mode if metacharacters present, else plain word)
    pub pattern: String,

    /// Limit to actionable paces only (rough); suppresses heat-level rows
    #[arg(long)]
    pub actionable: bool,
}

/// Compiled search mode: tiered matchers plus a frequency counter
pub(crate) struct zjjrsc_Matchers {
    /// (provenance marker, regex) per tier; single None-marker entry in regex mode
    tiers: Vec<(Option<&'static str>, Regex)>,
    /// Counts whole containing words (plain mode) or raw match spans (regex mode)
    freq: Regex,
}

/// One rendered result row with its tier for ordering
pub(crate) struct zjjrsc_Row {
    pub(crate) tier: usize,
    pub(crate) line: String,
}

/// One heat group: header plus its rows, sorted by tier
pub(crate) struct zjjrsc_Block {
    pub(crate) header: String,
    pub(crate) rows: Vec<zjjrsc_Row>,
}

/// True when the pattern carries any regex metacharacter
pub(crate) fn zjjrsc_is_regex_pattern(pattern: &str) -> bool {
    pattern.chars().any(|c| ZJJRSC_REGEX_METACHARS.contains(&c))
}

/// Compile the matchers for a pattern: single-pass regex mode, or three plain tiers
pub(crate) fn zjjrsc_build_matchers(pattern: &str) -> Result<zjjrsc_Matchers, regex::Error> {
    let ci = |p: &str| RegexBuilder::new(p).case_insensitive(true).build();
    if zjjrsc_is_regex_pattern(pattern) {
        Ok(zjjrsc_Matchers {
            tiers: vec![(None, ci(pattern)?)],
            freq: ci(pattern)?,
        })
    } else {
        let escaped = regex::escape(pattern);
        Ok(zjjrsc_Matchers {
            tiers: vec![
                (Some(ZJJRSC_TIER_MARKERS[0]), ci(&format!(r"\b{}\b", escaped))?),
                (Some(ZJJRSC_TIER_MARKERS[1]), ci(&format!(r"\b{}", escaped))?),
                (Some(ZJJRSC_TIER_MARKERS[2]), ci(&escaped)?),
            ],
            freq: ci(&format!(r"\b\w*{}\w*\b", escaped))?,
        })
    }
}

/// Lowest (strongest) tier whose regex matches the content
pub(crate) fn zjjrsc_best_tier<'a>(matchers: &'a zjjrsc_Matchers, content: &str) -> Option<(usize, &'a Regex)> {
    matchers.tiers.iter().enumerate()
        .find_map(|(i, (_, re))| re.is_match(content).then_some((i, re)))
}

/// Tally every frequency-regex occurrence in content, grouped case-insensitively
pub(crate) fn zjjrsc_tally(freq_re: &Regex, content: &str, freq: &mut BTreeMap<String, usize>) {
    for mat in freq_re.find_iter(content) {
        *freq.entry(mat.as_str().to_lowercase()).or_insert(0) += 1;
    }
}

/// Tier marker prefix for a row line: "[whole] " in plain mode, "" in regex mode
pub(crate) fn zjjrsc_marker_part(matchers: &zjjrsc_Matchers, tier: usize) -> String {
    matchers.tiers[tier].0.map(|m| format!("[{}] ", m)).unwrap_or_default()
}

/// Search the Gallops: heat-level surfaces (heat silks + paddock) once per heat,
/// then pace surfaces in heat order. Returns heat blocks sorted by strongest tier
/// and the word-frequency tally over every searched surface.
pub(crate) fn zjjrsc_search(
    gallops: &Gallops,
    matchers: &zjjrsc_Matchers,
    actionable: bool,
    read_paddock: impl Fn(&str) -> Option<String>,
) -> (Vec<zjjrsc_Block>, BTreeMap<String, usize>) {
    let mut freq: BTreeMap<String, usize> = BTreeMap::new();
    let mut blocks: Vec<zjjrsc_Block> = Vec::new();

    for (heat_key, heat) in &gallops.heats {
        let mut rows: Vec<zjjrsc_Row> = Vec::new();

        // Heat-level search, once per heat, independent of pace count.
        // The actionable filter is pace-scoped: it suppresses heat-level rows.
        if !actionable {
            zjjrsc_tally(&matchers.freq, &heat.silks, &mut freq);
            if let Some((tier, re)) = zjjrsc_best_tier(matchers, &heat.silks) {
                let excerpt = zjjrsc_extract_match_context(&heat.silks, re);
                rows.push(zjjrsc_Row {
                    tier,
                    line: format!("  {}heat-silks: {}", zjjrsc_marker_part(matchers, tier), excerpt),
                });
            }
            if let Some(content) = read_paddock(heat_key.trim_start_matches('₣')) {
                zjjrsc_tally(&matchers.freq, &content, &mut freq);
                if let Some((tier, re)) = zjjrsc_best_tier(matchers, &content) {
                    let excerpt = zjjrsc_extract_match_context(&content, re);
                    rows.push(zjjrsc_Row {
                        tier,
                        line: format!("  {}paddock: {}", zjjrsc_marker_part(matchers, tier), excerpt),
                    });
                }
            }
        }

        for coronet_key in &heat.order {
            let Some(pace) = heat.paces.get(coronet_key) else { continue };
            let Some(tack) = pace.tacks.first() else { continue };

            if actionable && !matches!(tack.state, PaceState::Rough) {
                continue;
            }

            let docket = jjrg_lines_to_text(&tack.text);
            let fields: [(&str, Option<&str>); 2] = [
                ("silks", Some(tack.silks.as_str())),
                ("docket", Some(docket.as_str())),
            ];

            for (_, content) in &fields {
                if let Some(c) = content {
                    zjjrsc_tally(&matchers.freq, c, &mut freq);
                }
            }

            // Tier-major, field-minor: the strongest tier wins across fields;
            // within a tier, the first field in declaration order wins.
            'pace: for (tier, (_, re)) in matchers.tiers.iter().enumerate() {
                for (label, content) in &fields {
                    let Some(c) = content else { continue };
                    if re.is_match(c) {
                        let excerpt = zjjrsc_extract_match_context(c, re);
                        rows.push(zjjrsc_Row {
                            tier,
                            line: format!("  {} [{}] {}{}: {}",
                                coronet_key, tack.state.jjrg_as_str(),
                                zjjrsc_marker_part(matchers, tier), label, excerpt),
                        });
                        break 'pace;
                    }
                }
            }
        }

        if !rows.is_empty() {
            // Stable sort: heat-level rows precede pace rows within a tier,
            // pace rows keep heat order within a tier
            rows.sort_by_key(|r| r.tier);
            blocks.push(zjjrsc_Block {
                header: format!("{} {}", heat_key, heat.silks),
                rows,
            });
        }
    }

    // Rows are tier-sorted, so rows[0] holds each block's strongest tier
    blocks.sort_by_key(|b| b.rows[0].tier);
    (blocks, freq)
}

/// Run the scout command - search across heats/paces
pub fn jjrsc_run_scout(args: jjrsc_ScoutArgs) -> (i32, String) {
    let cn = JJRSC_CMD_NAME_SCOUT;

    let mut output = vvco_Output::buffer();

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let matchers = match zjjrsc_build_matchers(&args.pattern) {
        Ok(m) => m,
        Err(e) => {
            vvco_err!(output, "{}: error: invalid regex pattern: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let (blocks, freq) = zjjrsc_search(&gallops, &matchers, args.actionable, |firemark| {
        fs::read_to_string(jjri_paddock_path(firemark)).ok()
    });

    if !freq.is_empty() {
        vvco_out!(output, "Matches:");
        let mut entries: Vec<(&String, &usize)> = freq.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (word, count) in entries {
            vvco_out!(output, "  {:>4} {}", count, word);
        }
        vvco_out!(output, "");
    }

    for block in &blocks {
        vvco_out!(output, "{}", block.header);
        for row in &block.rows {
            vvco_out!(output, "{}", row.line);
        }
    }

    (0, output.vvco_finish())
}

/// Extract context around the first regex match in content
pub(crate) fn zjjrsc_extract_match_context(content: &str, re: &Regex) -> String {
    if let Some(mat) = re.find(content) {
        let start = mat.start();
        let end = mat.end();

        // Find context window (30 chars before, 30 chars after)
        let context_before = 30;
        let context_after = 30;

        // Snap window boundaries to valid UTF-8 char boundaries
        let mut window_start = start.saturating_sub(context_before);
        while !content.is_char_boundary(window_start) {
            window_start -= 1;
        }

        let mut window_end = (end + context_after).min(content.len());
        while !content.is_char_boundary(window_end) {
            window_end += 1;
        }
        let window_end = window_end.min(content.len());

        let mut excerpt = String::new();
        if window_start > 0 {
            excerpt.push_str("...");
        }
        excerpt.push_str(&content[window_start..window_end]);
        if window_end < content.len() {
            excerpt.push_str("...");
        }

        // Replace newlines with spaces for single-line display
        excerpt.replace('\n', " ").replace('\r', "")
    } else {
        // Shouldn't happen, but fallback to truncated content
        let truncated = content.chars().take(60).collect::<String>();
        if content.len() > 60 {
            format!("{}...", truncated)
        } else {
            truncated
        }
    }
}
