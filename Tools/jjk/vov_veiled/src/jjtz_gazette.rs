// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrz_gazette::*;

// --- Parse: valid cases ---

#[test]
fn jjtz_parse_single_notice() {
    let md = format!("# {} my-pace\n\nDocket text here\n", JJRZ_SLUG_SLATE);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap();
    assert!(g.jjrz_is_frozen());
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "my-pace");
    assert_eq!(entries[0].1, "Docket text here");
}

#[test]
fn jjtz_parse_multiple_notices_same_slug() {
    let md = format!("# {} pace-one\n\nFirst docket\n\n# {} pace-two\n\nSecond docket\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_SLATE);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(entries.len(), 2);
}

#[test]
fn jjtz_parse_multiple_slugs() {
    let md = format!("# {} my-pace\n\nDocket\n\n# {} AF\n\n## Purpose\n\nContent\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_PADDOCK);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate, jjrz_Slug::Paddock], &md).unwrap();
    let all = g.jjrz_query_all();
    assert_eq!(all.len(), 2);
}

#[test]
fn jjtz_parse_empty_content() {
    let md = format!("# {} AF\n", JJRZ_SLUG_PADDOCK);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "AF");
    assert_eq!(entries[0].1, "");
}

#[test]
fn jjtz_parse_empty_input() {
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], "").unwrap();
    assert!(g.jjrz_query_all().is_empty());
}

#[test]
fn jjtz_parse_absent_lede() {
    let md = format!("# {}\n\nContent without lede\n", JJRZ_SLUG_PADDOCK);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "");
    assert_eq!(entries[0].1, "Content without lede");
}

#[test]
fn jjtz_parse_content_with_markdown_headers() {
    let md = format!("# {} AF\n\n## Purpose\n\nSome text\n\n### Details\n\nMore text\n", JJRZ_SLUG_PADDOCK);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert!(entries[0].1.contains("## Purpose"));
    assert!(entries[0].1.contains("### Details"));
}

#[test]
fn jjtz_parse_content_before_first_header_ignored() {
    let md = format!("Some preamble text\n\n# {} my-pace\n\nDocket\n", JJRZ_SLUG_SLATE);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].1, "Docket");
}

// --- Parse: error cases ---

#[test]
fn jjtz_parse_unknown_slug() {
    let md = "# bogus data\n";
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("unknown slug 'bogus'"));
}

#[test]
fn jjtz_parse_slug_not_in_vocab() {
    let md = format!("# {} AF\n", JJRZ_SLUG_PADDOCK);
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("not in vocabulary"));
}

#[test]
fn jjtz_parse_duplicate_key() {
    let md = format!("# {} my-pace\n\nFirst\n\n# {} my-pace\n\nSecond\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_SLATE);
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("duplicate key"));
}

#[test]
fn jjtz_parse_multiple_errors_collected() {
    let md = "# bogus one\n\n# unknown two\n";
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap_err();
    assert_eq!(err.len(), 2);
}

#[test]
fn jjtz_parse_malformed_header_no_slug() {
    let md = "#  \n";
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("malformed header"));
}

// --- Build/Add/Freeze ---

#[test]
fn jjtz_build_unfrozen() {
    let g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate, jjrz_Slug::Paddock]);
    assert!(!g.jjrz_is_frozen());
}

#[test]
fn jjtz_add_success() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "my-pace", "Docket text").unwrap();
    assert!(!g.jjrz_is_frozen());
}

#[test]
fn jjtz_query_by_slug_freezes() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "p1", "D1").unwrap();
    assert!(!g.jjrz_is_frozen());
    let _ = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert!(g.jjrz_is_frozen());
}

#[test]
fn jjtz_query_all_freezes() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "p1", "D1").unwrap();
    assert!(!g.jjrz_is_frozen());
    let _ = g.jjrz_query_all();
    assert!(g.jjrz_is_frozen());
}

#[test]
fn jjtz_emit_freezes() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "p1", "D1").unwrap();
    assert!(!g.jjrz_is_frozen());
    let _ = g.jjrz_emit();
    assert!(g.jjrz_is_frozen());
}

#[test]
fn jjtz_add_to_frozen_fails() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "one", "Content").unwrap();
    let _ = g.jjrz_emit();
    let err = g.jjrz_add(jjrz_Slug::Slate, "two", "More").unwrap_err();
    assert!(err.contains("frozen"));
}

#[test]
fn jjtz_add_bad_slug_fails() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    let err = g.jjrz_add(jjrz_Slug::Paddock, "AF", "Content").unwrap_err();
    assert!(err.contains("not in vocabulary"));
}

#[test]
fn jjtz_add_duplicate_key_fails() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "my-pace", "First").unwrap();
    let err = g.jjrz_add(jjrz_Slug::Slate, "my-pace", "Second").unwrap_err();
    assert!(err.contains("Duplicate key"));
}

// --- Round-trip ---

#[test]
fn jjtz_round_trip_single_notice() {
    let vocab = &[jjrz_Slug::Slate];
    let mut g = jjrz_Gazette::jjrz_build(vocab);
    g.jjrz_add(jjrz_Slug::Slate, "my-pace", "Line 1\nLine 2").unwrap();
    let md = g.jjrz_emit();
    let g2 = jjrz_Gazette::jjrz_parse(vocab, &md).unwrap();
    let entries = g2.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "my-pace");
    assert_eq!(entries[0].1, "Line 1\nLine 2");
}

#[test]
fn jjtz_round_trip_multi_slug() {
    let vocab = &[jjrz_Slug::Slate, jjrz_Slug::Paddock, jjrz_Slug::Pace];
    let mut g = jjrz_Gazette::jjrz_build(vocab);
    g.jjrz_add(jjrz_Slug::Slate, "pace-a", "Docket A").unwrap();
    g.jjrz_add(jjrz_Slug::Paddock, "AF", "## Purpose\n\nBuild things").unwrap();
    g.jjrz_add(jjrz_Slug::Pace, "AFAAa", "Per-pace detail").unwrap();
    let md = g.jjrz_emit();
    let g2 = jjrz_Gazette::jjrz_parse(vocab, &md).unwrap();
    let all = g2.jjrz_query_all();
    assert_eq!(all.len(), 3);
    assert!(all.iter().any(|(s, l, c)| *s == jjrz_Slug::Slate && l == "pace-a" && c == "Docket A"));
    assert!(all.iter().any(|(s, l, c)| *s == jjrz_Slug::Paddock && l == "AF" && c == "## Purpose\n\nBuild things"));
    assert!(all.iter().any(|(s, l, c)| *s == jjrz_Slug::Pace && l == "AFAAa" && c == "Per-pace detail"));
}

#[test]
fn jjtz_round_trip_empty_content() {
    let vocab = &[jjrz_Slug::Paddock];
    let mut g = jjrz_Gazette::jjrz_build(vocab);
    g.jjrz_add(jjrz_Slug::Paddock, "AF", "").unwrap();
    let md = g.jjrz_emit();
    let g2 = jjrz_Gazette::jjrz_parse(vocab, &md).unwrap();
    let entries = g2.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "AF");
    assert_eq!(entries[0].1, "");
}

#[test]
fn jjtz_round_trip_absent_lede() {
    let vocab = &[jjrz_Slug::Paddock];
    let mut g = jjrz_Gazette::jjrz_build(vocab);
    g.jjrz_add(jjrz_Slug::Paddock, "", "Content here").unwrap();
    let md = g.jjrz_emit();
    let g2 = jjrz_Gazette::jjrz_parse(vocab, &md).unwrap();
    let entries = g2.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "");
    assert_eq!(entries[0].1, "Content here");
}

// --- Directionality ---

#[test]
fn jjtz_slug_directions() {
    assert_eq!(jjrz_Slug::Slate.jjrz_direction(), jjrz_Direction::Input);
    assert_eq!(jjrz_Slug::Reslate.jjrz_direction(), jjrz_Direction::Input);
    assert_eq!(jjrz_Slug::Paddock.jjrz_direction(), jjrz_Direction::Bidirectional);
    assert_eq!(jjrz_Slug::Pace.jjrz_direction(), jjrz_Direction::Output);
}

#[test]
fn jjtz_slug_round_trip_strings() {
    for slug in JJRZ_ALL_SLUGS {
        let s = slug.jjrz_as_str();
        let parsed = jjrz_Slug::jjrz_from_str(s).unwrap();
        assert_eq!(*slug, parsed);
    }
}

// --- Internal helper tests ---

#[test]
fn jjtz_notice_boundary_detection() {
    assert!(zjjrz_is_notice_boundary(&format!("# {} my-pace", JJRZ_SLUG_SLATE)));
    assert!(zjjrz_is_notice_boundary(&format!("# {}", JJRZ_SLUG_PADDOCK)));
    assert!(zjjrz_is_notice_boundary("#\tslug"));
    assert!(!zjjrz_is_notice_boundary("## heading"));
    assert!(!zjjrz_is_notice_boundary("### heading"));
    assert!(!zjjrz_is_notice_boundary("#word"));
    assert!(!zjjrz_is_notice_boundary("#"));
    assert!(!zjjrz_is_notice_boundary(""));
    assert!(!zjjrz_is_notice_boundary("regular text"));
}

// --- Fenced code block awareness ---

#[test]
fn jjtz_parse_fenced_code_block_hash_not_boundary() {
    let md = format!(
        "# {} AF\n\nSome text\n\n```bash\n# This is a bash comment\n# Another comment\necho hello\n```\n\nMore text\n",
        JJRZ_SLUG_PADDOCK
    );
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "AF");
    assert!(entries[0].1.contains("# This is a bash comment"));
    assert!(entries[0].1.contains("echo hello"));
    assert!(entries[0].1.contains("More text"));
}

#[test]
fn jjtz_parse_fenced_block_with_slug_like_content() {
    let md = format!(
        "# {} AF\n\nBefore\n\n```\n# jjezs_slate fake-header\n# jjezs_paddock also-fake\n```\n\nAfter\n",
        JJRZ_SLUG_PADDOCK
    );
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert!(entries[0].1.contains("# jjezs_slate fake-header"));
    assert!(entries[0].1.contains("After"));
}

#[test]
fn jjtz_parse_notice_after_fenced_block() {
    let md = format!(
        "# {} AF\n\n```\n# comment inside\n```\n\n# {} pace-one\n\nDocket\n",
        JJRZ_SLUG_PADDOCK, JJRZ_SLUG_SLATE
    );
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Slate], &md).unwrap();
    let paddock = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(paddock.len(), 1);
    assert!(paddock[0].1.contains("# comment inside"));
    let slate = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(slate.len(), 1);
    assert_eq!(slate[0].0, "pace-one");
    assert_eq!(slate[0].1, "Docket");
}

#[test]
fn jjtz_round_trip_fenced_code_block() {
    let vocab = &[jjrz_Slug::Paddock];
    let content = "## Example\n\n```bash\n# safety check\ncd \"$HOME/$RELDIR\" || exit 99\n```\n\nDone";
    let mut g = jjrz_Gazette::jjrz_build(vocab);
    g.jjrz_add(jjrz_Slug::Paddock, "AF", content).unwrap();
    let md = g.jjrz_emit();
    let g2 = jjrz_Gazette::jjrz_parse(vocab, &md).unwrap();
    let entries = g2.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].1, content);
}

// --- Slate input parsing ---

#[test]
fn jjtz_parse_slate_input_valid() {
    let md = format!("# {} my-pace\n\nDocket text here\n", JJRZ_SLUG_SLATE);
    let (silks, docket) = jjrz_parse_slate_input(&md).unwrap();
    assert_eq!(silks, "my-pace");
    assert_eq!(docket, "Docket text here");
}

#[test]
fn jjtz_parse_slate_input_multiline_docket() {
    let md = format!("# {} my-pace\n\nLine 1\n\n## Section\n\nLine 2\n", JJRZ_SLUG_SLATE);
    let (silks, docket) = jjrz_parse_slate_input(&md).unwrap();
    assert_eq!(silks, "my-pace");
    assert!(docket.contains("Line 1"));
    assert!(docket.contains("## Section"));
    assert!(docket.contains("Line 2"));
}

#[test]
fn jjtz_parse_slate_input_no_notices() {
    let err = jjrz_parse_slate_input("").unwrap_err();
    assert!(err.contains("No slate notice"));
}

#[test]
fn jjtz_parse_slate_input_multiple_notices() {
    let md = format!("# {} p1\n\nD1\n\n# {} p2\n\nD2\n", JJRZ_SLUG_SLATE, JJRZ_SLUG_SLATE);
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("Expected one"));
}

#[test]
fn jjtz_parse_slate_input_missing_lede() {
    let md = format!("# {}\n\nDocket\n", JJRZ_SLUG_SLATE);
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("missing lede"));
}

#[test]
fn jjtz_parse_slate_input_wrong_slug() {
    let md = format!("# {} AF\n\nContent\n", JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("not in vocabulary"));
}

// --- Reslate input parsing ---

#[test]
fn jjtz_parse_reslate_input_single() {
    let md = format!("# {} AFAAa\n\nNew docket\n", JJRZ_SLUG_RESLATE);
    let pairs = jjrz_parse_reslate_input(&md).unwrap();
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].0, "AFAAa");
    assert_eq!(pairs[0].1, "New docket");
}

#[test]
fn jjtz_parse_reslate_input_mass() {
    let md = format!("# {} AFAAa\n\nDocket A\n\n# {} AFAAb\n\nDocket B\n",
        JJRZ_SLUG_RESLATE, JJRZ_SLUG_RESLATE);
    let pairs = jjrz_parse_reslate_input(&md).unwrap();
    assert_eq!(pairs.len(), 2);
}

#[test]
fn jjtz_parse_reslate_input_no_notices() {
    let err = jjrz_parse_reslate_input("").unwrap_err();
    assert!(err.contains("No reslate notice"));
}

#[test]
fn jjtz_parse_reslate_input_missing_lede() {
    let md = format!("# {}\n\nDocket\n", JJRZ_SLUG_RESLATE);
    let err = jjrz_parse_reslate_input(&md).unwrap_err();
    assert!(err.contains("missing lede"));
}

// --- Paddock input parsing ---

#[test]
fn jjtz_parse_paddock_input_valid() {
    let md = format!("# {} AF\n\n## Purpose\n\nBuild things\n", JJRZ_SLUG_PADDOCK);
    let (firemark, content) = jjrz_parse_paddock_input(&md).unwrap();
    assert_eq!(firemark, "AF");
    assert!(content.contains("## Purpose"));
    assert!(content.contains("Build things"));
}

#[test]
fn jjtz_parse_paddock_input_no_notices() {
    let err = jjrz_parse_paddock_input("").unwrap_err();
    assert!(err.contains("No paddock notice"));
}

#[test]
fn jjtz_parse_paddock_input_missing_lede() {
    let md = format!("# {}\n\nContent\n", JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_paddock_input(&md).unwrap_err();
    assert!(err.contains("missing lede"));
}

#[test]
fn jjtz_parse_paddock_input_multiple_notices() {
    let md = format!("# {} AF\n\nC1\n\n# {} AG\n\nC2\n", JJRZ_SLUG_PADDOCK, JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_paddock_input(&md).unwrap_err();
    assert!(err.contains("Expected one"));
}

// --- Output round-trip tests ---

#[test]
fn jjtz_output_orient_round_trip() {
    let md = jjrz_build_read_output("Aw", "## Purpose\n\nBuild things", &[("AwAAJ", "Implement gazette output")]);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &md).unwrap();
    let paddock = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(paddock.len(), 1);
    assert_eq!(paddock[0].0, "Aw");
    assert!(paddock[0].1.contains("## Purpose"));
    assert!(paddock[0].1.contains("Build things"));
}

#[test]
fn jjtz_output_orient_pace_round_trip() {
    let md = jjrz_build_read_output("Aw", "Paddock text", &[("AwAAJ", "Docket text")]);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &md).unwrap();
    let paces = g.jjrz_query_by_slug(jjrz_Slug::Pace);
    assert_eq!(paces.len(), 1);
    assert_eq!(paces[0].0, "AwAAJ");
    assert_eq!(paces[0].1, "Docket text");
}

#[test]
fn jjtz_output_parade_detail_round_trip() {
    let paces = &[
        ("AwAAA", "First docket"),
        ("AwAAB", "Second docket"),
        ("AwAAC", "Third docket"),
    ];
    let md = jjrz_build_read_output("Aw", "Paddock content", paces);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &md).unwrap();
    let paddock = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(paddock.len(), 1);
    assert_eq!(paddock[0].0, "Aw");
}

#[test]
fn jjtz_output_parade_detail_paces_round_trip() {
    let paces = &[
        ("AwAAA", "First docket"),
        ("AwAAB", "Second docket"),
        ("AwAAC", "Third docket"),
    ];
    let md = jjrz_build_read_output("Aw", "Paddock content", paces);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &md).unwrap();
    let pace_entries = g.jjrz_query_by_slug(jjrz_Slug::Pace);
    assert_eq!(pace_entries.len(), 3);
    assert_eq!(pace_entries[0].0, "AwAAA");
    assert_eq!(pace_entries[0].1, "First docket");
    assert_eq!(pace_entries[1].0, "AwAAB");
    assert_eq!(pace_entries[2].0, "AwAAC");
}

#[test]
fn jjtz_output_paddock_getter_round_trip() {
    let md = jjrz_build_read_output("AF", "## Paddock heading\n\nPaddock body", &[]);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &md).unwrap();
    let paddock = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(paddock.len(), 1);
    assert_eq!(paddock[0].0, "AF");
    assert!(paddock[0].1.contains("Paddock body"));
    let paces = g.jjrz_query_by_slug(jjrz_Slug::Pace);
    assert!(paces.is_empty());
}

#[test]
fn jjtz_output_with_preamble_parses() {
    let preamble = "Heat: my-heat (Aw) [racing]\nPaddock: .claude/jjm/jjp_uAlw.md\n\nNext: do-stuff (AwAAJ) [rough]\n\n";
    let gazette = jjrz_build_read_output("Aw", "Paddock text", &[("AwAAJ", "Docket text")]);
    let full_output = format!("{}\n{}", preamble, gazette);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &full_output).unwrap();
    let paddock = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(paddock[0].0, "Aw");
    assert_eq!(paddock[0].1, "Paddock text");
}
