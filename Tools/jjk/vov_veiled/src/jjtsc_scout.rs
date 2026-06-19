// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjx_scout command (search across heats/paces)

use regex::Regex;
use std::collections::BTreeMap;

use crate::jjrg_gallops::{
    jjrg_Gallops,
    jjrg_Heat,
    jjrg_HeatStatus,
    jjrg_Pace,
    jjrg_PaceState,
    jjrg_Tack,
};
use crate::jjrsc_scout::{
    zjjrsc_build_matchers,
    zjjrsc_extract_match_context,
    zjjrsc_is_regex_pattern,
    zjjrsc_search,
};

// ============================================================================
// Test fixture builders — in-memory Gallops, no disk, no git
// ============================================================================

fn jjtsc_make_tack(state: jjrg_PaceState, silks: &str, text: &str, direction: Option<&str>) -> jjrg_Tack {
    jjrg_Tack {
        ts: "20260612T120000Z".to_string(),
        state,
        text: text.to_string(),
        silks: silks.to_string(),
        basis: "0000000".to_string(),
        direction: direction.map(|d| d.to_string()),
    }
}

fn jjtsc_make_heat(silks: &str, status: jjrg_HeatStatus, paces: Vec<(&str, jjrg_Tack)>) -> jjrg_Heat {
    let order: Vec<String> = paces.iter().map(|(k, _)| k.to_string()).collect();
    let mut pace_map = BTreeMap::new();
    for (key, tack) in paces {
        pace_map.insert(key.to_string(), jjrg_Pace { tacks: vec![tack] });
    }
    jjrg_Heat {
        silks: silks.to_string(),
        creation_time: "260612".to_string(),
        status,
        order,
        next_pace_seed: "AAZ".to_string(),
        paces: pace_map,
    }
}

fn jjtsc_make_gallops(heats: Vec<(&str, jjrg_Heat)>) -> jjrg_Gallops {
    let heat_order: Vec<String> = heats.iter().map(|(k, _)| k.to_string()).collect();
    let mut heat_map = BTreeMap::new();
    for (key, heat) in heats {
        heat_map.insert(key.to_string(), heat);
    }
    jjrg_Gallops {
        next_heat_seed: "ZZ".to_string(),
        heat_order,
        heats: heat_map,
    }
}

fn jjtsc_no_paddock(_firemark: &str) -> Option<String> {
    None
}

// ============================================================================
// Mode detection and tier matching
// ============================================================================

#[test]
fn jjtsc_mode_plain_word_is_not_regex() {
    assert!(!zjjrsc_is_regex_pattern("ark"));
    assert!(!zjjrsc_is_regex_pattern("two words"));
    assert!(!zjjrsc_is_regex_pattern("kebab-case"));
}

#[test]
fn jjtsc_mode_metachars_select_regex() {
    for p in [r"\bark\b", "ark.*", "a+rk", "ar?k", "[ark]", "(ark)", "ark{2}", "a|b", "^ark", "ark$"] {
        assert!(zjjrsc_is_regex_pattern(p), "expected regex mode for: {}", p);
    }
}

#[test]
fn jjtsc_plain_mode_builds_three_tiers() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p-one", "the ark itself", None)),
            ("₢AAAAB", jjtsc_make_tack(jjrg_PaceState::Rough, "p-two", "several arks here", None)),
            ("₢AAAAC", jjtsc_make_tack(jjrg_PaceState::Rough, "p-three", "a hallmark word", None)),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    assert_eq!(blocks.len(), 1);
    let lines: Vec<&str> = blocks[0].rows.iter().map(|r| r.line.as_str()).collect();
    assert!(lines[0].contains("₢AAAAA") && lines[0].contains("[whole]"), "got: {:?}", lines);
    assert!(lines[1].contains("₢AAAAB") && lines[1].contains("[prefix]"), "got: {:?}", lines);
    assert!(lines[2].contains("₢AAAAC") && lines[2].contains("[substring]"), "got: {:?}", lines);
}

#[test]
fn jjtsc_regex_mode_single_pass_no_marker() {
    let m = zjjrsc_build_matchers(r"\bark\b").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p-one", "the ark itself", None)),
            ("₢AAAAB", jjtsc_make_tack(jjrg_PaceState::Rough, "p-two", "several arks here", None)),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].rows.len(), 1, "regex \\bark\\b should match only the whole word");
    assert!(blocks[0].rows[0].line.contains("₢AAAAA"));
    assert!(!blocks[0].rows[0].line.contains("[whole]"));
}

// ============================================================================
// Heat-level matching (behavior set A)
// ============================================================================

#[test]
fn jjtsc_zero_pace_heat_paddock_match_yields_heat_row() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-empty", jjrg_HeatStatus::Racing, vec![])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, |fm| {
        assert_eq!(fm, "AA", "paddock reader receives firemark without ₣");
        Some("paddock prose mentioning the ark".to_string())
    });
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].header, "₣AA h-empty");
    assert_eq!(blocks[0].rows.len(), 1);
    assert!(blocks[0].rows[0].line.contains("paddock:"), "got: {}", blocks[0].rows[0].line);
    assert!(!blocks[0].rows[0].line.contains('₢'), "heat-level row carries no coronet");
}

#[test]
fn jjtsc_zero_pace_heat_silks_match_yields_heat_row() {
    let m = zjjrsc_build_matchers("tackle").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣BF", jjtsc_make_heat("jjk-v4-2-add-tackle", jjrg_HeatStatus::Racing, vec![])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, |_| Some("placeholder".to_string()));
    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].rows[0].line.contains("heat-silks:"), "got: {}", blocks[0].rows[0].line);
}

#[test]
fn jjtsc_nonmatching_direction_no_longer_blocks_paddock() {
    // Old bug: a pace with present-but-nonmatching direction skipped the paddock fallback
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p-one", "no match here", Some("nor here"))),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, |_| {
        Some("paddock rich in ark terms".to_string())
    });
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].rows.len(), 1);
    assert!(blocks[0].rows[0].line.contains("paddock:"));
}

#[test]
fn jjtsc_self_matching_paces_do_not_mask_paddock() {
    // Old bug: a heat whose paces all self-match never had its paddock searched
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "ark-pace", "ark in docket", None)),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, |_| {
        Some("ark in the paddock too".to_string())
    });
    let lines: Vec<&str> = blocks[0].rows.iter().map(|r| r.line.as_str()).collect();
    assert!(lines.iter().any(|l| l.contains("paddock:")), "got: {:?}", lines);
    assert!(lines.iter().any(|l| l.contains("₢AAAAA")), "got: {:?}", lines);
}

#[test]
fn jjtsc_actionable_suppresses_heat_rows_and_filters_paces() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("ark-heat", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Complete, "p-done", "ark done", None)),
            ("₢AAAAB", jjtsc_make_tack(jjrg_PaceState::Rough, "p-open", "ark open", None)),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, true, |_| {
        Some("ark paddock".to_string())
    });
    assert_eq!(blocks.len(), 1);
    let lines: Vec<&str> = blocks[0].rows.iter().map(|r| r.line.as_str()).collect();
    assert_eq!(lines.len(), 1, "got: {:?}", lines);
    assert!(lines[0].contains("₢AAAAB"));
}

#[test]
fn jjtsc_all_statuses_searched() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("ark-racing", jjrg_HeatStatus::Racing, vec![])),
        ("₣AB", jjtsc_make_heat("ark-stabled", jjrg_HeatStatus::Stabled, vec![])),
        ("₣AC", jjtsc_make_heat("ark-retired", jjrg_HeatStatus::Retired, vec![])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    assert_eq!(blocks.len(), 3);
}

// ============================================================================
// Row format and ordering (behavior set B)
// ============================================================================

#[test]
fn jjtsc_pace_row_drops_silks_and_uses_spec_labels() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "my-distinctive-silks", "ark in docket", None)),
            ("₢AAAAB", jjtsc_make_tack(jjrg_PaceState::Bridled, "other-silks", "no match", Some("ark in warrant"))),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    let lines: Vec<&str> = blocks[0].rows.iter().map(|r| r.line.as_str()).collect();
    assert!(lines[0].contains("docket:"), "spec label is docket, not spec: {:?}", lines);
    assert!(!lines[0].contains("my-distinctive-silks"), "pace silks dropped from row: {:?}", lines);
    assert!(lines[0].contains("[rough]"));
    assert!(lines[1].contains("warrant:"), "spec label is warrant, not direction: {:?}", lines);
}

#[test]
fn jjtsc_blocks_sort_by_strongest_tier() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    // ₣AA matches only at substring tier; ₣AB at whole tier — ₣AB sorts first
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-weak", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p", "a hallmark word", None)),
        ])),
        ("₣AB", jjtsc_make_heat("h-strong", jjrg_HeatStatus::Racing, vec![
            ("₢ABAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p", "the ark itself", None)),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    assert_eq!(blocks[0].header, "₣AB h-strong");
    assert_eq!(blocks[1].header, "₣AA h-weak");
}

#[test]
fn jjtsc_tier_major_field_minor_within_pace() {
    // Silks matches only at substring tier, docket at whole tier: whole wins
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "hallmark-silks", "the ark itself", None)),
        ])),
    ]);
    let (blocks, _) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    let line = &blocks[0].rows[0].line;
    assert!(line.contains("[whole]") && line.contains("docket:"), "got: {}", line);
}

// ============================================================================
// Word-frequency summary
// ============================================================================

#[test]
fn jjtsc_frequency_counts_containing_words_plain_mode() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p", "ark and arks and hallmark and Ark", None)),
        ])),
    ]);
    let (_, freq) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    assert_eq!(freq.get("ark"), Some(&2), "case-insensitive grouping: {:?}", freq);
    assert_eq!(freq.get("arks"), Some(&1));
    assert_eq!(freq.get("hallmark"), Some(&1));
}

#[test]
fn jjtsc_frequency_includes_heat_level_text() {
    let m = zjjrsc_build_matchers("ark").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("ark-heat", jjrg_HeatStatus::Racing, vec![])),
    ]);
    let (_, freq) = zjjrsc_search(&gallops, &m, false, |_| {
        Some("ark twice: ark".to_string())
    });
    // 1 from heat silks (ark-heat → "ark-heat" splits at hyphen: word "ark") + 2 from paddock
    assert_eq!(freq.get("ark"), Some(&3), "got: {:?}", freq);
}

#[test]
fn jjtsc_frequency_regex_mode_counts_spans() {
    let m = zjjrsc_build_matchers(r"ark\w*").unwrap();
    let gallops = jjtsc_make_gallops(vec![
        ("₣AA", jjtsc_make_heat("h-one", jjrg_HeatStatus::Racing, vec![
            ("₢AAAAA", jjtsc_make_tack(jjrg_PaceState::Rough, "p", "ark arks arkive", None)),
        ])),
    ]);
    let (_, freq) = zjjrsc_search(&gallops, &m, false, jjtsc_no_paddock);
    assert_eq!(freq.get("ark"), Some(&1), "got: {:?}", freq);
    assert_eq!(freq.get("arks"), Some(&1));
    assert_eq!(freq.get("arkive"), Some(&1));
}

// ============================================================================
// zjjrsc_extract_match_context tests
// ============================================================================

#[test]
fn jjtsc_extract_match_context_middle() {
    // Match in the middle with full context available (need >30 chars before match for ellipsis)
    let content = "This is a longer prefix text that exceeds thirty characters MATCH_HERE and some suffix text after that also exceeds thirty chars";
    let re = Regex::new("MATCH_HERE").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    // Should have ... prefix (start > 30) and ... suffix (more than 30 chars after match)
    assert!(result.starts_with("..."), "Expected prefix ellipsis, got: {}", result);
    assert!(result.ends_with("..."), "Expected suffix ellipsis, got: {}", result);
    assert!(result.contains("MATCH_HERE"));
}

#[test]
fn jjtsc_extract_match_context_at_start() {
    // Match at the very beginning
    let content = "MATCH at the start of the content with more text following";
    let re = Regex::new("MATCH").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    // Should NOT have ... prefix (start == 0)
    assert!(!result.starts_with("..."));
    assert!(result.contains("MATCH"));
    // Should have ... suffix (content longer than context window)
    assert!(result.ends_with("..."));
}

#[test]
fn jjtsc_extract_match_context_at_end() {
    // Match at the very end
    let content = "Some text at the beginning and then MATCH";
    let re = Regex::new("MATCH").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    // Should have ... prefix
    assert!(result.starts_with("..."));
    // Should NOT have ... suffix (end == len)
    assert!(!result.ends_with("..."));
    assert!(result.contains("MATCH"));
}

#[test]
fn jjtsc_extract_match_context_short_content() {
    // Content shorter than context window
    let content = "short MATCH text";
    let re = Regex::new("MATCH").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    // Should have no ellipsis since content is small
    assert!(!result.starts_with("..."));
    assert!(!result.ends_with("..."));
    assert_eq!(result, "short MATCH text");
}

#[test]
fn jjtsc_extract_match_context_with_newlines() {
    // Content with newlines should have them replaced with spaces
    let content = "line one\nMATCH\nline three";
    let re = Regex::new("MATCH").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    // Newlines should be replaced with spaces
    assert!(!result.contains('\n'));
    assert!(result.contains("line one MATCH line three"));
}

#[test]
fn jjtsc_extract_match_context_carriage_return() {
    // Content with carriage returns should have them removed
    let content = "line one\r\nMATCH\r\nline three";
    let re = Regex::new("MATCH").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    // Carriage returns should be removed, newlines replaced with spaces
    assert!(!result.contains('\r'));
    assert!(!result.contains('\n'));
}

#[test]
fn jjtsc_extract_match_context_case_insensitive() {
    // Test with case-insensitive regex (as used in scout)
    use regex::RegexBuilder;
    let content = "searching for KEYWORD in text";
    let re = RegexBuilder::new("keyword")
        .case_insensitive(true)
        .build()
        .unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    assert!(result.contains("KEYWORD"));
}

#[test]
fn jjtsc_extract_match_context_exact_boundary() {
    // Match just past context boundary (31 chars from start means window_start = 1)
    let prefix = "a".repeat(31);
    let content = format!("{}MATCH suffix", prefix);
    let re = Regex::new("MATCH").unwrap();
    let result = zjjrsc_extract_match_context(&content, &re);

    // Should have ... prefix since window_start = 31 - 30 = 1 > 0
    assert!(result.starts_with("..."), "Expected prefix ellipsis, got: {}", result);
    assert!(result.contains("MATCH"));
}

#[test]
fn jjtsc_extract_match_context_long_match() {
    // Longer match pattern
    let content = "prefix LONG_MATCH_PATTERN_HERE suffix";
    let re = Regex::new("LONG_MATCH_PATTERN_HERE").unwrap();
    let result = zjjrsc_extract_match_context(content, &re);

    assert!(result.contains("LONG_MATCH_PATTERN_HERE"));
}
