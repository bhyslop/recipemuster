// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrz_gazette::*;

// --- Parse: valid cases ---

#[test]
fn jjtz_parse_single_notice() {
    let md = "# slate my-pace\n\nDocket text here\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap();
    assert!(g.jjrz_is_frozen());
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "my-pace");
    assert_eq!(entries[0].1, "Docket text here");
}

#[test]
fn jjtz_parse_multiple_notices_same_slug() {
    let md = "# slate pace-one\n\nFirst docket\n\n# slate pace-two\n\nSecond docket\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Slate);
    assert_eq!(entries.len(), 2);
}

#[test]
fn jjtz_parse_multiple_slugs() {
    let md = "# slate my-pace\n\nDocket\n\n# paddock AF\n\n## Purpose\n\nContent\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate, jjrz_Slug::Paddock], md).unwrap();
    let all = g.jjrz_query_all();
    assert_eq!(all.len(), 2);
}

#[test]
fn jjtz_parse_empty_content() {
    let md = "# paddock AF\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], md).unwrap();
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
    let md = "# paddock\n\nContent without lede\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "");
    assert_eq!(entries[0].1, "Content without lede");
}

#[test]
fn jjtz_parse_content_with_markdown_headers() {
    let md = "# paddock AF\n\n## Purpose\n\nSome text\n\n### Details\n\nMore text\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock], md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(entries.len(), 1);
    assert!(entries[0].1.contains("## Purpose"));
    assert!(entries[0].1.contains("### Details"));
}

#[test]
fn jjtz_parse_content_before_first_header_ignored() {
    let md = "Some preamble text\n\n# slate my-pace\n\nDocket\n";
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap();
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
    let md = "# paddock AF\n";
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("not in vocabulary"));
}

#[test]
fn jjtz_parse_duplicate_key() {
    let md = "# slate my-pace\n\nFirst\n\n# slate my-pace\n\nSecond\n";
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("duplicate key"));
}

#[test]
fn jjtz_parse_near_match_suggestion() {
    let md = "# slatee my-pace\n";
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("did you mean 'slate'"));
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
    assert!(zjjrz_is_notice_boundary("# slate my-pace"));
    assert!(zjjrz_is_notice_boundary("# paddock"));
    assert!(zjjrz_is_notice_boundary("#\tslug"));
    assert!(!zjjrz_is_notice_boundary("## heading"));
    assert!(!zjjrz_is_notice_boundary("### heading"));
    assert!(!zjjrz_is_notice_boundary("#word"));
    assert!(!zjjrz_is_notice_boundary("#"));
    assert!(!zjjrz_is_notice_boundary(""));
    assert!(!zjjrz_is_notice_boundary("regular text"));
}

#[test]
fn jjtz_edit_distance_values() {
    assert_eq!(zjjrz_edit_distance("slate", "slate"), 0);
    assert_eq!(zjjrz_edit_distance("slatee", "slate"), 1);
    assert_eq!(zjjrz_edit_distance("slat", "slate"), 1);
    assert_eq!(zjjrz_edit_distance("slate", "reslate"), 2);
    assert_eq!(zjjrz_edit_distance("xyz", "slate"), 5);
}

#[test]
fn jjtz_near_match_finds_close() {
    let vocab = &[jjrz_Slug::Slate, jjrz_Slug::Paddock];
    assert_eq!(zjjrz_near_match("slatee", vocab), Some("slate"));
    assert_eq!(zjjrz_near_match("slte", vocab), Some("slate"));
    assert_eq!(zjjrz_near_match("paddok", vocab), Some("paddock"));
}

#[test]
fn jjtz_near_match_none_when_distant() {
    let vocab = &[jjrz_Slug::Slate];
    assert_eq!(zjjrz_near_match("completely_different", vocab), None);
}

// --- Slate input parsing ---

#[test]
fn jjtz_parse_slate_input_valid() {
    let md = "# slate my-pace\n\nDocket text here\n";
    let (silks, docket) = jjrz_parse_slate_input(md).unwrap();
    assert_eq!(silks, "my-pace");
    assert_eq!(docket, "Docket text here");
}

#[test]
fn jjtz_parse_slate_input_multiline_docket() {
    let md = "# slate my-pace\n\nLine 1\n\n## Section\n\nLine 2\n";
    let (silks, docket) = jjrz_parse_slate_input(md).unwrap();
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
    let md = "# slate p1\n\nD1\n\n# slate p2\n\nD2\n";
    let err = jjrz_parse_slate_input(md).unwrap_err();
    assert!(err.contains("Expected one"));
}

#[test]
fn jjtz_parse_slate_input_missing_lede() {
    let md = "# slate\n\nDocket\n";
    let err = jjrz_parse_slate_input(md).unwrap_err();
    assert!(err.contains("missing lede"));
}

#[test]
fn jjtz_parse_slate_input_wrong_slug() {
    let md = "# paddock AF\n\nContent\n";
    let err = jjrz_parse_slate_input(md).unwrap_err();
    assert!(err.contains("not in vocabulary"));
}

// --- Reslate input parsing ---

#[test]
fn jjtz_parse_reslate_input_single() {
    let md = "# reslate AFAAa\n\nNew docket\n";
    let pairs = jjrz_parse_reslate_input(md).unwrap();
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].0, "AFAAa");
    assert_eq!(pairs[0].1, "New docket");
}

#[test]
fn jjtz_parse_reslate_input_mass() {
    let md = "# reslate AFAAa\n\nDocket A\n\n# reslate AFAAb\n\nDocket B\n";
    let pairs = jjrz_parse_reslate_input(md).unwrap();
    assert_eq!(pairs.len(), 2);
}

#[test]
fn jjtz_parse_reslate_input_no_notices() {
    let err = jjrz_parse_reslate_input("").unwrap_err();
    assert!(err.contains("No reslate notice"));
}

#[test]
fn jjtz_parse_reslate_input_missing_lede() {
    let md = "# reslate\n\nDocket\n";
    let err = jjrz_parse_reslate_input(md).unwrap_err();
    assert!(err.contains("missing lede"));
}

// --- Paddock input parsing ---

#[test]
fn jjtz_parse_paddock_input_valid() {
    let md = "# paddock AF\n\n## Purpose\n\nBuild things\n";
    let (firemark, content) = jjrz_parse_paddock_input(md).unwrap();
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
    let md = "# paddock\n\nContent\n";
    let err = jjrz_parse_paddock_input(md).unwrap_err();
    assert!(err.contains("missing lede"));
}

#[test]
fn jjtz_parse_paddock_input_multiple_notices() {
    let md = "# paddock AF\n\nC1\n\n# paddock AG\n\nC2\n";
    let err = jjrz_parse_paddock_input(md).unwrap_err();
    assert!(err.contains("Expected one"));
}
