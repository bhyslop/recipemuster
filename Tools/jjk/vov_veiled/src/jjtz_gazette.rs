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

#[test]
fn jjtz_parse_fused_notice_header_rejected() {
    // Body's last line lacks a trailing newline before the next header, so the
    // next notice's header text fuses onto the same physical line — the
    // boundary scan never sees a line starting with `#`, and the swallowed
    // header must be caught by content-scan instead.
    let md = format!(
        "# {} pace-one\n\nFirst docket# {} pace-two\n\nSecond docket\n",
        JJRZ_SLUG_RESLATE, JJRZ_SLUG_RESLATE
    );
    let err = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Reslate], &md).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("Line 3"));
    assert!(err[0].contains("fused notice header"));
}

#[test]
fn jjtz_parse_clean_batch_still_applies() {
    // A well-formed multi-notice batch (proper trailing newlines throughout)
    // is unaffected by the fused-header guard.
    let md = format!(
        "# {} pace-one\n\nFirst docket\n\n# {} pace-two\n\nSecond docket\n",
        JJRZ_SLUG_RESLATE, JJRZ_SLUG_RESLATE
    );
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Reslate], &md).unwrap();
    let entries = g.jjrz_query_by_slug(jjrz_Slug::Reslate);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].1, "First docket");
    assert_eq!(entries[1].1, "Second docket");
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
    assert_eq!(jjrz_Slug::Halter.jjrz_direction(), jjrz_Direction::Input);
    assert_eq!(jjrz_Slug::Dictation.jjrz_direction(), jjrz_Direction::Input);
    assert_eq!(jjrz_Slug::Precis.jjrz_direction(), jjrz_Direction::Input);
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
    let input = jjrz_parse_slate_input(&md).unwrap();
    assert_eq!(input.silks, "my-pace");
    assert_eq!(input.docket, "Docket text here");
    // A bare slate carries no original-intent companions.
    assert!(input.dictation.is_none());
    assert!(input.precis.is_none());
}

#[test]
fn jjtz_parse_slate_input_multiline_docket() {
    let md = format!("# {} my-pace\n\nLine 1\n\n## Section\n\nLine 2\n", JJRZ_SLUG_SLATE);
    let input = jjrz_parse_slate_input(&md).unwrap();
    assert_eq!(input.silks, "my-pace");
    assert!(input.docket.contains("Line 1"));
    assert!(input.docket.contains("## Section"));
    assert!(input.docket.contains("Line 2"));
}

#[test]
fn jjtz_parse_slate_input_with_intent_companions() {
    // The ceremony shape: slate + dictation + precis, bound by a shared lede.
    let md = format!(
        "# {} my-pace\n\nDocket text\n\n# {} my-pace\n\nOperator said do the thing\n\n# {} my-pace\n\nDistilled: do the thing\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_DICTATION, JJRZ_SLUG_PRECIS
    );
    let input = jjrz_parse_slate_input(&md).unwrap();
    assert_eq!(input.silks, "my-pace");
    assert_eq!(input.docket, "Docket text");
    assert_eq!(input.dictation.as_deref(), Some("Operator said do the thing"));
    assert_eq!(input.precis.as_deref(), Some("Distilled: do the thing"));
}

#[test]
fn jjtz_parse_slate_input_companion_alone() {
    // Each companion is independently optional at the tool layer.
    let md = format!(
        "# {} my-pace\n\nDocket text\n\n# {} my-pace\n\nJust the raw words\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_DICTATION
    );
    let input = jjrz_parse_slate_input(&md).unwrap();
    assert_eq!(input.dictation.as_deref(), Some("Just the raw words"));
    assert!(input.precis.is_none());
}

#[test]
fn jjtz_parse_slate_input_companion_lede_mismatch() {
    // The shared-lede binding: a companion aimed at different silks rejects.
    let md = format!(
        "# {} my-pace\n\nDocket text\n\n# {} other-pace\n\nStray words\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_PRECIS
    );
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("does not match slate silks"), "got: {}", err);
}

#[test]
fn jjtz_parse_slate_input_companion_empty_body() {
    let md = format!(
        "# {} my-pace\n\nDocket text\n\n# {} my-pace\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_DICTATION
    );
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
}

#[test]
fn jjtz_parse_slate_input_duplicate_companion() {
    let md = format!(
        "# {} p1\n\nD\n\n# {} p1\n\nW1\n\n# {} p2\n\nW2\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_DICTATION, JJRZ_SLUG_DICTATION
    );
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("at most one"), "got: {}", err);
}

#[test]
fn jjtz_parse_batch_input_rejects_intent_companions() {
    // Batch-born paces carry no intent capture (documented follow-on): a
    // staged companion must fail loud as not-in-vocabulary, never drop silent.
    let md = format!(
        "# {} new-pace\n\nDocket\n\n# {} new-pace\n\nWords\n",
        JJRZ_SLUG_SLATE, JJRZ_SLUG_DICTATION
    );
    let err = jjrz_parse_batch_input(&md).unwrap_err();
    assert!(err.contains("not in vocabulary"), "got: {}", err);
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

// --- Non-empty-body law: empty input notices reject, never execute ---

#[test]
fn jjtz_parse_paddock_input_empty_body() {
    // The wipe shape: a bare pre-staged notice with no body must reject
    // loud, never execute as "replace the paddock with nothing".
    let md = format!("# {} ₣Bg\n", JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_paddock_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
    assert!(err.contains("never blanks"), "names the refusal: {}", err);
}

#[test]
fn jjtz_parse_paddock_input_whitespace_only_body() {
    let md = format!("# {} ₣Bg\n\n   \n\t\n", JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_paddock_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
}

#[test]
fn jjtz_parse_slate_input_empty_body() {
    let md = format!("# {} hollow-pace\n", JJRZ_SLUG_SLATE);
    let err = jjrz_parse_slate_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
    assert!(err.contains("hollow-pace"), "names the lede: {}", err);
}

#[test]
fn jjtz_parse_reslate_input_empty_body() {
    let md = format!("# {} AFAAa\n", JJRZ_SLUG_RESLATE);
    let err = jjrz_parse_reslate_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
}

#[test]
fn jjtz_parse_batch_input_empty_paddock_body() {
    // A well-formed sibling notice does not launder the empty one.
    let md = format!("# {} ₣Bg\n\n# {} real-pace\n\ndocket text\n",
        JJRZ_SLUG_PADDOCK, JJRZ_SLUG_SLATE);
    let err = jjrz_parse_batch_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
}

#[test]
fn jjtz_parse_batch_input_empty_reslate_body() {
    let md = format!("# {} AFAAa\n", JJRZ_SLUG_RESLATE);
    let err = jjrz_parse_batch_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
}

#[test]
fn jjtz_parse_batch_input_empty_slate_body() {
    let md = format!("# {} AFAAa\n\nreal docket\n\n# {} hollow-pace\n",
        JJRZ_SLUG_RESLATE, JJRZ_SLUG_SLATE);
    let err = jjrz_parse_batch_input(&md).unwrap_err();
    assert!(err.contains("empty body"), "got: {}", err);
}

// --- Halter (read-path target selection) input parsing ---

#[test]
fn jjtz_parse_halter_input_single_firemark() {
    let md = format!("# {} ₣BD\n", JJRZ_SLUG_HALTER);
    let targets = jjrz_parse_halter_input(&md).unwrap();
    assert_eq!(targets, vec!["₣BD".to_string()]);
}

#[test]
fn jjtz_parse_halter_input_single_coronet() {
    let md = format!("# {} ₢BDAAT\n", JJRZ_SLUG_HALTER);
    let targets = jjrz_parse_halter_input(&md).unwrap();
    assert_eq!(targets, vec!["₢BDAAT".to_string()]);
}

#[test]
fn jjtz_parse_halter_input_heterogeneous_set() {
    // Show's many-target case: a firemark and a coronet in one gazette. Order is
    // by-lede (BTreeMap), not insertion — each target is independent, so the set
    // is what matters, not the sequence.
    let md = format!("# {} ₣AB\n\n# {} ₢CDAAA\n", JJRZ_SLUG_HALTER, JJRZ_SLUG_HALTER);
    let mut targets = jjrz_parse_halter_input(&md).unwrap();
    targets.sort();
    assert_eq!(targets, vec!["₢CDAAA".to_string(), "₣AB".to_string()]);
}

#[test]
fn jjtz_parse_halter_input_ignores_body() {
    // A stray body is tolerated — the lede carries the whole signal.
    let md = format!("# {} ₣BD\n\nstray text the agent should not have written\n", JJRZ_SLUG_HALTER);
    let targets = jjrz_parse_halter_input(&md).unwrap();
    assert_eq!(targets, vec!["₣BD".to_string()]);
}

#[test]
fn jjtz_parse_halter_input_no_notices() {
    let err = jjrz_parse_halter_input("").unwrap_err();
    assert!(err.contains("No halter notice"));
}

#[test]
fn jjtz_parse_halter_input_missing_lede() {
    let md = format!("# {}\n\nbody only\n", JJRZ_SLUG_HALTER);
    let err = jjrz_parse_halter_input(&md).unwrap_err();
    assert!(err.contains("missing lede"));
}

#[test]
fn jjtz_parse_halter_input_wrong_slug() {
    let md = format!("# {} ₣BD\n", JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_halter_input(&md).unwrap_err();
    assert!(err.contains("not in vocabulary"));
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
    let preamble = "Heat: my-heat (Aw) [racing]\nPaddock: paddocks/jjp_uAlw.md\n\nNext: do-stuff (AwAAJ) [rough]\n\n";
    let gazette = jjrz_build_read_output("Aw", "Paddock text", &[("AwAAJ", "Docket text")]);
    let full_output = format!("{}\n{}", preamble, gazette);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Paddock, jjrz_Slug::Pace], &full_output).unwrap();
    let paddock = g.jjrz_query_by_slug(jjrz_Slug::Paddock);
    assert_eq!(paddock[0].0, "Aw");
    assert_eq!(paddock[0].1, "Paddock text");
}

// --- File-order access + mixed batch parsing ---

#[test]
fn jjtz_query_by_slug_ordered_preserves_file_order() {
    // Ledes deliberately authored in NON-lexical order. The default by-slug query
    // sorts by lede; the ordered query must return file order instead.
    let md = format!(
        "# {0} zebra\n\nfirst authored\n\n# {0} alpha\n\nsecond authored\n\n# {0} mango\n\nthird authored\n",
        JJRZ_SLUG_SLATE);
    let g = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap();

    let lexical: Vec<String> = g.jjrz_query_by_slug(jjrz_Slug::Slate)
        .into_iter().map(|(l, _)| l).collect();
    assert_eq!(lexical, vec!["alpha", "mango", "zebra"], "by-slug query is lede-sorted");

    let g2 = jjrz_Gazette::jjrz_parse(&[jjrz_Slug::Slate], &md).unwrap();
    let filed: Vec<String> = g2.jjrz_query_by_slug_ordered(jjrz_Slug::Slate)
        .into_iter().map(|(l, _)| l).collect();
    assert_eq!(filed, vec!["zebra", "alpha", "mango"], "ordered query is file order");
}

#[test]
fn jjtz_query_by_slug_ordered_builder_uses_insertion_order() {
    let mut g = jjrz_Gazette::jjrz_build(&[jjrz_Slug::Slate]);
    g.jjrz_add(jjrz_Slug::Slate, "zulu", "c1").unwrap();
    g.jjrz_add(jjrz_Slug::Slate, "delta", "c2").unwrap();
    let filed: Vec<String> = g.jjrz_query_by_slug_ordered(jjrz_Slug::Slate)
        .into_iter().map(|(l, _)| l).collect();
    assert_eq!(filed, vec!["zulu", "delta"]);
}

#[test]
fn jjtz_parse_batch_input_mixed() {
    let md = format!(
        "# {} BD\n\npaddock body\n\n# {} ₢BDAAb\n\nreslate body\n\n# {} new-pace\n\nslate body\n",
        JJRZ_SLUG_PADDOCK, JJRZ_SLUG_RESLATE, JJRZ_SLUG_SLATE);
    let batch = jjrz_parse_batch_input(&md).unwrap();
    assert_eq!(batch.paddock.as_ref().unwrap().0, "BD");
    assert_eq!(batch.paddock.as_ref().unwrap().1, "paddock body");
    assert_eq!(batch.reslates.len(), 1);
    assert_eq!(batch.reslates[0].0, "₢BDAAb");
    assert_eq!(batch.slates.len(), 1);
    assert_eq!(batch.slates[0].0, "new-pace");
}

#[test]
fn jjtz_parse_batch_input_slates_in_file_order_not_silks_order() {
    // The cinch: notice order is pace order. Silks authored non-alphabetically
    // must come back in file order, NOT sorted by silks.
    let md = format!(
        "# {0} yankee\n\nd1\n\n# {0} bravo\n\nd2\n\n# {0} mike\n\nd3\n",
        JJRZ_SLUG_SLATE);
    let batch = jjrz_parse_batch_input(&md).unwrap();
    let silks: Vec<String> = batch.slates.iter().map(|(s, _)| s.clone()).collect();
    assert_eq!(silks, vec!["yankee", "bravo", "mike"]);
}

#[test]
fn jjtz_parse_batch_input_rejects_two_paddocks() {
    let md = format!(
        "# {0} BD\n\nbody one\n\n# {0} BE\n\nbody two\n",
        JJRZ_SLUG_PADDOCK);
    let err = jjrz_parse_batch_input(&md).unwrap_err();
    assert!(err.contains("at most one paddock"), "got: {}", err);
}

#[test]
fn jjtz_parse_batch_input_rejects_empty() {
    let err = jjrz_parse_batch_input("no notices here\n").unwrap_err();
    assert!(err.contains("no notices"), "got: {}", err);
}
