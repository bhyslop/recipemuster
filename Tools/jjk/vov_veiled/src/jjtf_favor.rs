// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrf_favor::*;

#[test]
fn jjtf_charset_length() {
    assert_eq!(JJRF_CHARSET.len(), 64);
}

#[test]
fn jjtf_charset_values() {
    // Verify specific positions in charset
    assert_eq!(zjjrf_char_to_value('A').unwrap(), 0);
    assert_eq!(zjjrf_char_to_value('B').unwrap(), 1);
    assert_eq!(zjjrf_char_to_value('Z').unwrap(), 25);
    assert_eq!(zjjrf_char_to_value('a').unwrap(), 26);
    assert_eq!(zjjrf_char_to_value('z').unwrap(), 51);
    assert_eq!(zjjrf_char_to_value('0').unwrap(), 52);
    assert_eq!(zjjrf_char_to_value('9').unwrap(), 61);
    assert_eq!(zjjrf_char_to_value('-').unwrap(), 62);
    assert_eq!(zjjrf_char_to_value('_').unwrap(), 63);
}

#[test]
fn jjtf_char_to_value_invalid() {
    assert!(zjjrf_char_to_value('!').is_err());
    assert!(zjjrf_char_to_value(' ').is_err());
    assert!(zjjrf_char_to_value('+').is_err());
    assert!(zjjrf_char_to_value('/').is_err());
}

// Firemark tests

#[test]
fn jjtf_firemark_encode_zero() {
    let fm = jjrf_Firemark::jjrf_encode(0);
    assert_eq!(fm.jjrf_as_str(), "AA");
    assert_eq!(fm.jjrf_display(), "₣AA");
}

#[test]
fn jjtf_firemark_encode_one() {
    let fm = jjrf_Firemark::jjrf_encode(1);
    assert_eq!(fm.jjrf_as_str(), "AB");
    assert_eq!(fm.jjrf_display(), "₣AB");
}

#[test]
fn jjtf_firemark_encode_64() {
    // 64 = 1*64 + 0
    let fm = jjrf_Firemark::jjrf_encode(64);
    assert_eq!(fm.jjrf_as_str(), "BA");
}

#[test]
fn jjtf_firemark_encode_max() {
    // 4095 = 63*64 + 63
    let fm = jjrf_Firemark::jjrf_encode(4095);
    assert_eq!(fm.jjrf_as_str(), "__");
    assert_eq!(fm.jjrf_display(), "₣__");
}

#[test]
fn jjtf_firemark_encode_decode_roundtrip() {
    for value in [0, 1, 63, 64, 65, 100, 1000, 4000, 4095] {
        let fm = jjrf_Firemark::jjrf_encode(value);
        let decoded = fm.jjrf_decode().unwrap();
        assert_eq!(decoded, value, "Roundtrip failed for {}", value);
    }
}

#[test]
fn jjtf_firemark_decode_invalid_length() {
    let fm = jjrf_Firemark::jjrf_from_raw("A");
    assert!(fm.jjrf_decode().is_err());

    let fm = jjrf_Firemark::jjrf_from_raw("ABC");
    assert!(fm.jjrf_decode().is_err());
}

#[test]
fn jjtf_firemark_parse_with_prefix() {
    let fm = jjrf_Firemark::jjrf_parse("₣AB").unwrap();
    assert_eq!(fm.jjrf_as_str(), "AB");
    assert_eq!(fm.jjrf_decode().unwrap(), 1);
}

#[test]
fn jjtf_firemark_parse_without_prefix() {
    let fm = jjrf_Firemark::jjrf_parse("AB").unwrap();
    assert_eq!(fm.jjrf_as_str(), "AB");
    assert_eq!(fm.jjrf_decode().unwrap(), 1);
}

#[test]
fn jjtf_firemark_parse_invalid() {
    assert!(jjrf_Firemark::jjrf_parse("A").is_err());
    assert!(jjrf_Firemark::jjrf_parse("ABC").is_err());
    assert!(jjrf_Firemark::jjrf_parse("₣A").is_err());
    assert!(jjrf_Firemark::jjrf_parse("₣ABC").is_err());
    assert!(jjrf_Firemark::jjrf_parse("A!").is_err());
}

// Coronet tests

#[test]
fn jjtf_coronet_encode_zero() {
    let coronet = jjrf_Coronet::jjrf_encode(0);
    assert_eq!(coronet.jjrf_as_str(), "AAAAA");
    assert_eq!(coronet.jjrf_display(), "₢AAAAA");
}

#[test]
fn jjtf_coronet_encode_each_digit_position() {
    // RADIX^position places a single "1" in that digit position (0 = the low,
    // rightmost digit), exercising every digit of the flat 5-char encoding. No
    // embedded heat — the leading digits are ordinary index digits, never a firemark.
    let expected = ["AAAAB", "AAABA", "AABAA", "ABAAA", "BAAAA"];
    for (position, want) in expected.iter().enumerate() {
        let coronet = jjrf_Coronet::jjrf_encode(JJRF_RADIX.pow(position as u32));
        assert_eq!(coronet.jjrf_as_str(), *want, "digit position {}", position);
    }
}

#[test]
fn jjtf_coronet_encode_max() {
    // Every digit saturated (JJRF_CORONET_MAX = RADIX^CORONET_LEN - 1) → the last
    // charset character in all five positions.
    let coronet = jjrf_Coronet::jjrf_encode(JJRF_CORONET_MAX);
    assert_eq!(coronet.jjrf_as_str(), "_____");
}

#[test]
fn jjtf_coronet_decode_roundtrip() {
    let samples = [0, 1, JJRF_RADIX, JJRF_RADIX.pow(2), JJRF_RADIX.pow(4), JJRF_CORONET_MAX];
    for idx in samples {
        let coronet = jjrf_Coronet::jjrf_encode(idx);
        assert_eq!(coronet.jjrf_decode().unwrap(), idx, "roundtrip failed for index {}", idx);
    }
}

#[test]
fn jjtf_coronet_parse_with_prefix() {
    let coronet = jjrf_Coronet::jjrf_parse("₢ABAAA").unwrap();
    assert_eq!(coronet.jjrf_as_str(), "ABAAA");
    // "ABAAA" carries a single "1" in digit position 3.
    assert_eq!(coronet.jjrf_decode().unwrap(), JJRF_RADIX.pow(3));
}

#[test]
fn jjtf_coronet_parse_without_prefix() {
    let coronet = jjrf_Coronet::jjrf_parse("ABAAA").unwrap();
    assert_eq!(coronet.jjrf_as_str(), "ABAAA");
}

#[test]
fn jjtf_coronet_parse_invalid() {
    assert!(jjrf_Coronet::jjrf_parse("ABAA").is_err());
    assert!(jjrf_Coronet::jjrf_parse("ABAAAA").is_err());
    assert!(jjrf_Coronet::jjrf_parse("₢ABAA").is_err());
    assert!(jjrf_Coronet::jjrf_parse("ABA!A").is_err());
}

#[test]
fn jjtf_coronet_decode_invalid_length() {
    let c = jjrf_Coronet::jjrf_from_raw("ABCD");
    assert!(c.jjrf_decode().is_err());

    let c = jjrf_Coronet::jjrf_from_raw("ABCDEF");
    assert!(c.jjrf_decode().is_err());
}

// Edge case tests

#[test]
fn jjtf_specific_encoding_example() {
    // Example from spec: charset[H/64] + charset[H%64]
    // H = 65 -> 65/64 = 1, 65%64 = 1 -> "BB"
    let fm = jjrf_Firemark::jjrf_encode(65);
    assert_eq!(fm.jjrf_as_str(), "BB");
}

#[test]
fn jjtf_all_charset_positions_encode_decode() {
    // Ensure every charset position works
    for i in 0..64 {
        let fm = jjrf_Firemark::jjrf_encode(i as u16);
        let decoded = fm.jjrf_decode().unwrap();
        assert_eq!(decoded, i as u16);
    }
}

// Pensum tests

#[test]
fn jjtf_pensum_encode_zero() {
    let heat = jjrf_Firemark::jjrf_encode(0);
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 0);
    assert_eq!(pensum.jjrf_as_str(), "AA%AA");
    assert_eq!(pensum.jjrf_display(), "₱AA%AA");
}

#[test]
fn jjtf_pensum_encode_one() {
    let heat = jjrf_Firemark::jjrf_encode(0);
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 1);
    assert_eq!(pensum.jjrf_as_str(), "AA%AB");
}

#[test]
fn jjtf_pensum_encode_64() {
    let heat = jjrf_Firemark::jjrf_encode(0);
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 64);
    assert_eq!(pensum.jjrf_as_str(), "AA%BA");
}

#[test]
fn jjtf_pensum_encode_max() {
    // 4095 = 63*64 + 63
    let heat = jjrf_Firemark::jjrf_encode(0);
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 4095);
    assert_eq!(pensum.jjrf_as_str(), "AA%__");
}

#[test]
fn jjtf_pensum_encode_with_nonzero_heat() {
    // Example from spec: heat ₣Ah, index for "BE"
    let heat = jjrf_Firemark::jjrf_parse("Ah").unwrap();
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 68); // B=1, E=4 -> 1*64+4=68
    assert_eq!(pensum.jjrf_as_str(), "Ah%BE");
    assert_eq!(pensum.jjrf_display(), "₱Ah%BE");
}

#[test]
fn jjtf_pensum_encode_decode_roundtrip() {
    for heat_val in [0, 1, 100, 4095] {
        for index_val in [0, 1, 63, 64, 1000, 4095] {
            let heat = jjrf_Firemark::jjrf_encode(heat_val);
            let pensum = jjrf_Pensum::jjrf_encode(&heat, index_val);
            let (decoded_heat, decoded_index) = pensum.jjrf_decode().unwrap();
            assert_eq!(
                decoded_heat.jjrf_decode().unwrap(),
                heat_val,
                "Heat roundtrip failed for heat={} index={}",
                heat_val,
                index_val
            );
            assert_eq!(
                decoded_index, index_val,
                "Index roundtrip failed for heat={} index={}",
                heat_val, index_val
            );
        }
    }
}

#[test]
fn jjtf_pensum_parse_with_prefix() {
    let pensum = jjrf_Pensum::jjrf_parse("₱Ah%BE").unwrap();
    assert_eq!(pensum.jjrf_as_str(), "Ah%BE");
    let (heat, index) = pensum.jjrf_decode().unwrap();
    assert_eq!(heat.jjrf_as_str(), "Ah");
    assert_eq!(index, 68);
}

#[test]
fn jjtf_pensum_parse_without_prefix() {
    let pensum = jjrf_Pensum::jjrf_parse("Ah%BE").unwrap();
    assert_eq!(pensum.jjrf_as_str(), "Ah%BE");
}

#[test]
fn jjtf_pensum_parse_invalid_length() {
    assert!(jjrf_Pensum::jjrf_parse("Ah%B").is_err());
    assert!(jjrf_Pensum::jjrf_parse("Ah%BEE").is_err());
    assert!(jjrf_Pensum::jjrf_parse("₱Ah%B").is_err());
}

#[test]
fn jjtf_pensum_parse_missing_sentinel() {
    // 5 base64 chars but no % at position 3 — looks like a Coronet, not Pensum
    assert!(jjrf_Pensum::jjrf_parse("AhABE").is_err());
}

#[test]
fn jjtf_pensum_parse_invalid_chars() {
    assert!(jjrf_Pensum::jjrf_parse("A!%BE").is_err());
    assert!(jjrf_Pensum::jjrf_parse("Ah%B!").is_err());
}

#[test]
fn jjtf_pensum_sentinel_disambiguation_vs_coronet() {
    // Coronet: 5 base64url chars, no % at position 3
    // Pensum: 5 chars with % at position 3
    let coronet_str = "AhABE";
    let pensum_str = "Ah%BE";

    // Coronet parses as Coronet, fails as Pensum
    assert!(jjrf_Coronet::jjrf_parse(coronet_str).is_ok());
    assert!(jjrf_Pensum::jjrf_parse(coronet_str).is_err());

    // Pensum parses as Pensum, fails as Coronet (% is not in base64url charset)
    assert!(jjrf_Pensum::jjrf_parse(pensum_str).is_ok());
    assert!(jjrf_Coronet::jjrf_parse(pensum_str).is_err());
}

#[test]
fn jjtf_pensum_parent_firemark() {
    let heat = jjrf_Firemark::jjrf_encode(42);
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 123);
    let parent = pensum.jjrf_parent_firemark();
    assert_eq!(parent.jjrf_as_str(), heat.jjrf_as_str());
    assert_eq!(parent.jjrf_decode().unwrap(), 42);
}

#[test]
fn jjtf_pensum_decode_invalid_length() {
    let p = jjrf_Pensum::jjrf_from_raw("Ah%B");
    assert!(p.jjrf_decode().is_err());
}

#[test]
fn jjtf_pensum_decode_missing_sentinel() {
    let p = jjrf_Pensum::jjrf_from_raw("AhABE");
    assert!(p.jjrf_decode().is_err());
}

// Sigil accessor — render-layer mark, distinct from the bare body

#[test]
fn jjtf_sigils_are_render_only() {
    assert_eq!(jjrf_Firemark::jjrf_encode(0).jjrf_sigil(), JJRF_FIREMARK_PREFIX);
    let heat = jjrf_Firemark::jjrf_encode(0);
    assert_eq!(jjrf_Coronet::jjrf_encode(0).jjrf_sigil(), JJRF_CORONET_PREFIX);
    assert_eq!(jjrf_Pensum::jjrf_encode(&heat, 0).jjrf_sigil(), JJRF_PENSUM_PREFIX);
    assert_eq!(jjrf_Incipit::jjrf_new("260705-1000-abcd").jjrf_sigil(), JJRF_INCIPIT_PREFIX);
}

// Algebraic successor — generative derivation, fresh immutable value

#[test]
fn jjtf_firemark_successor() {
    let fm = jjrf_Firemark::jjrf_encode(0);
    let next = fm.jjrf_successor().unwrap();
    assert_eq!(next.jjrf_as_str(), "AB");
    // Source untouched — derivation is generative, not mutative.
    assert_eq!(fm.jjrf_as_str(), "AA");
}

#[test]
fn jjtf_firemark_successor_saturates() {
    let fm = jjrf_Firemark::jjrf_encode(JJRF_FIREMARK_MAX);
    assert!(fm.jjrf_successor().is_err());
}

#[test]
fn jjtf_coronet_successor() {
    // Flat successor: index + 1. "ABAAA" (a "1" in digit position 3) → "ABAAB".
    let coronet = jjrf_Coronet::jjrf_encode(JJRF_RADIX.pow(3));
    let next = coronet.jjrf_successor().unwrap();
    assert_eq!(next.jjrf_as_str(), "ABAAB");
    // Source untouched — derivation is generative, not mutative.
    assert_eq!(coronet.jjrf_as_str(), "ABAAA");
}

#[test]
fn jjtf_coronet_successor_saturates() {
    let coronet = jjrf_Coronet::jjrf_encode(JJRF_CORONET_MAX);
    assert!(coronet.jjrf_successor().is_err());
}

#[test]
fn jjtf_pensum_successor_stays_in_heat() {
    let heat = jjrf_Firemark::jjrf_parse("Ah").unwrap();
    let pensum = jjrf_Pensum::jjrf_encode(&heat, 0);
    let next = pensum.jjrf_successor().unwrap();
    assert_eq!(next.jjrf_as_str(), "Ah%AB");
}

#[test]
fn jjtf_pensum_successor_saturates() {
    let heat = jjrf_Firemark::jjrf_encode(0);
    let pensum = jjrf_Pensum::jjrf_encode(&heat, JJRF_PENSUM_INDEX_MAX);
    assert!(pensum.jjrf_successor().is_err());
}

// Incipit — temporal insignia (officium)

#[test]
fn jjtf_incipit_display_and_bare() {
    let inc = jjrf_Incipit::jjrf_new("260705-1000-abcd");
    assert_eq!(inc.jjrf_as_str(), "260705-1000-abcd");        // bare — the on-disk dir name
    assert_eq!(inc.jjrf_display(), "☉260705-1000-abcd");      // operator form — sigil mandatory
}

#[test]
fn jjtf_incipit_parse_ignores_sigil() {
    let with = jjrf_Incipit::jjrf_parse("☉260705-1000-abcd").unwrap();
    let without = jjrf_Incipit::jjrf_parse("260705-1000-abcd").unwrap();
    assert_eq!(with.jjrf_as_str(), "260705-1000-abcd");
    assert_eq!(without.jjrf_as_str(), "260705-1000-abcd");
    assert_eq!(with, without);
}

#[test]
fn jjtf_incipit_parse_rejects_empty() {
    assert!(jjrf_Incipit::jjrf_parse("").is_err());
    assert!(jjrf_Incipit::jjrf_parse("☉").is_err());
}

// jjrf_bare — the single ingest-normalization home (JJS0 jjdz_encoding)

#[test]
fn jjtf_bare_strips_glyph_and_qualifier() {
    // Every emitted coronet form reduces to the bare 5-char body.
    assert_eq!(jjrf_bare("₢Bc·CAAAB"), "CAAAB"); // heat-qualified
    assert_eq!(jjrf_bare("₢CAAAB"), "CAAAB");    // glyph, bare
    assert_eq!(jjrf_bare("CAAAB"), "CAAAB");     // naked body
    // Firemarks carry no qualifier; the glyph strips, the body passes.
    assert_eq!(jjrf_bare("₣Bc"), "Bc");
    assert_eq!(jjrf_bare("Bc"), "Bc");
    // A grandfathered coronet renders its heat twice; the qualifier still splits
    // on the interpunct, leaving the full body.
    assert_eq!(jjrf_bare("₢Bc·BcAAO"), "BcAAO");
}

#[test]
fn jjtf_coronet_parse_tolerates_qualified_form() {
    // Ingest tolerates the emitted heat-qualified form (JJS0 jjdz_encoding
    // "Input flexibility"): the `·` qualifier and `₢` glyph strip to the bare body.
    let bare = jjrf_Coronet::jjrf_parse("₢CAAAB").unwrap();
    let qualified = jjrf_Coronet::jjrf_parse("₢Bc·CAAAB").unwrap();
    let naked = jjrf_Coronet::jjrf_parse("CAAAB").unwrap();
    assert_eq!(qualified, bare);
    assert_eq!(naked, bare);
    assert_eq!(qualified.jjrf_as_str(), "CAAAB");
    // Display alone (no live heat) is the bare form; qualification needs the gallops.
    assert_eq!(qualified.jjrf_display(), "₢CAAAB");
}

// ---- The livery badge ----

#[test]
fn jjtf_livery_composes_the_pace_badge() {
    // The ordinary form: sprue as namespace root, no pedigree prefix.
    assert_eq!(
        jjrf_livery_compose(None, jjrf_LiveryKind::Pace, "CAAA9"),
        "jjls_pace/CAAA9"
    );
    // The body rides bare — a git ref is a foreign-traversed surface, so the
    // sigil stays behind and the badge is what the ref carries instead.
    assert!(!jjrf_livery_compose(None, jjrf_LiveryKind::Pace, "CAAA9").contains(JJRF_CORONET_PREFIX));
}

#[test]
fn jjtf_livery_prefix_is_org_demand_only() {
    // A pedigree-recorded path prefix nests the whole badge; absent (and empty,
    // the shape a hand-edited pedigree yields) it never appears at all.
    assert_eq!(
        jjrf_livery_compose(Some("teams/jj"), jjrf_LiveryKind::Pace, "CAAA9"),
        "teams/jj/jjls_pace/CAAA9"
    );
    assert_eq!(
        jjrf_livery_compose(Some("teams/jj/"), jjrf_LiveryKind::Pace, "CAAA9"),
        "teams/jj/jjls_pace/CAAA9"
    );
    assert_eq!(
        jjrf_livery_compose(Some("  "), jjrf_LiveryKind::Pace, "CAAA9"),
        "jjls_pace/CAAA9"
    );
}

#[test]
fn jjtf_livery_round_trips_through_any_prefix() {
    // Parse strips the prefix by finding the sprue-bearing segment, so a prefix
    // JJ never recorded still reads — the prefix is presentation, not a parse input.
    for prefix in [None, Some("teams/jj"), Some("a/b/c")] {
        let branch = jjrf_livery_compose(prefix, jjrf_LiveryKind::Pace, "CAAA9");
        assert_eq!(jjrf_livery_parse(&branch), Some((jjrf_LiveryKind::Pace, "CAAA9")));
    }
}

#[test]
fn jjtf_livery_parses_the_reserved_groom_word() {
    // Groom is never populated, but it parses: the reservation is enforceable
    // only if a violating branch is nameable when met.
    assert_eq!(
        jjrf_livery_parse("jjls_groom/B9"),
        Some((jjrf_LiveryKind::Groom, "B9"))
    );
}

#[test]
fn jjtf_livery_declines_what_it_does_not_claim() {
    // The bare-coronet form this mint retired — the whole point is that it no
    // longer reads as a JJ branch.
    assert_eq!(jjrf_livery_parse("CAAA9"), None);
    // Trunk and ordinary consumer branches.
    assert_eq!(jjrf_livery_parse("main"), None);
    assert_eq!(jjrf_livery_parse("feature/CAAA9"), None);
    // An unrostered sprue word is refused, never guessed at: a future kind
    // costs a roster word rather than a silent reinterpretation.
    assert_eq!(jjrf_livery_parse("jjls_scout/CAAA9"), None);
    // The length-type backstop: a roster word alone does not make a body.
    assert_eq!(jjrf_livery_parse("jjls_pace/CAAA"), None);
    assert_eq!(jjrf_livery_parse("jjls_pace/CAAA99"), None);
    assert_eq!(jjrf_livery_parse("jjls_groom/B9X"), None);
    // A bare roster word with no identity at all.
    assert_eq!(jjrf_livery_parse("jjls_pace"), None);
}

#[test]
fn jjtf_livery_roster_words_all_carry_the_sprue() {
    // The grep tie is the mint's whole point: one grep over the sprue must
    // reach every roster word.
    for kind in [jjrf_LiveryKind::Pace, jjrf_LiveryKind::Groom] {
        assert!(kind.jjrf_as_str().starts_with(JJRF_LIVERY_SPRUE));
    }
}
