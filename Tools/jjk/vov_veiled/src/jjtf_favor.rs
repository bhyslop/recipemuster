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
    let fm = jjrf_Firemark("A".to_string());
    assert!(fm.jjrf_decode().is_err());

    let fm = jjrf_Firemark("ABC".to_string());
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
    let heat = jjrf_Firemark::jjrf_encode(0);
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 0);
    assert_eq!(coronet.jjrf_as_str(), "AAAAA");
    assert_eq!(coronet.jjrf_display(), "₢AAAAA");
}

#[test]
fn jjtf_coronet_encode_one() {
    let heat = jjrf_Firemark::jjrf_encode(0);
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 1);
    assert_eq!(coronet.jjrf_as_str(), "AAAAB");
}

#[test]
fn jjtf_coronet_encode_64() {
    // pace 64 = 0*4096 + 1*64 + 0
    let heat = jjrf_Firemark::jjrf_encode(0);
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 64);
    assert_eq!(coronet.jjrf_as_str(), "AAABA");
}

#[test]
fn jjtf_coronet_encode_4096() {
    // pace 4096 = 1*4096 + 0*64 + 0
    let heat = jjrf_Firemark::jjrf_encode(0);
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 4096);
    assert_eq!(coronet.jjrf_as_str(), "AABAA");
}

#[test]
fn jjtf_coronet_encode_max_pace() {
    // 262143 = 63*4096 + 63*64 + 63
    let heat = jjrf_Firemark::jjrf_encode(0);
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 262143);
    assert_eq!(coronet.jjrf_as_str(), "AA___");
}

#[test]
fn jjtf_coronet_encode_with_nonzero_heat() {
    let heat = jjrf_Firemark::jjrf_encode(1); // "AB"
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 0);
    assert_eq!(coronet.jjrf_as_str(), "ABAAA");
    assert_eq!(coronet.jjrf_display(), "₢ABAAA");
}

#[test]
fn jjtf_coronet_decode_roundtrip() {
    for heat_val in [0, 1, 100, 4095] {
        for pace_val in [0, 1, 63, 64, 4096, 100000, 262143] {
            let heat = jjrf_Firemark::jjrf_encode(heat_val);
            let coronet = jjrf_Coronet::jjrf_encode(&heat, pace_val);
            let (decoded_heat, decoded_pace) = coronet.jjrf_decode().unwrap();
            assert_eq!(
                decoded_heat.jjrf_decode().unwrap(),
                heat_val,
                "Heat roundtrip failed for heat={} pace={}",
                heat_val,
                pace_val
            );
            assert_eq!(
                decoded_pace, pace_val,
                "Pace roundtrip failed for heat={} pace={}",
                heat_val, pace_val
            );
        }
    }
}

#[test]
fn jjtf_coronet_parent_firemark() {
    let heat = jjrf_Firemark::jjrf_encode(42);
    let coronet = jjrf_Coronet::jjrf_encode(&heat, 123);
    let parent = coronet.jjrf_parent_firemark();
    assert_eq!(parent.jjrf_as_str(), heat.jjrf_as_str());
    assert_eq!(parent.jjrf_decode().unwrap(), 42);
}

#[test]
fn jjtf_coronet_parse_with_prefix() {
    let coronet = jjrf_Coronet::jjrf_parse("₢ABAAA").unwrap();
    assert_eq!(coronet.jjrf_as_str(), "ABAAA");
    let (heat, pace) = coronet.jjrf_decode().unwrap();
    assert_eq!(heat.jjrf_decode().unwrap(), 1);
    assert_eq!(pace, 0);
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
    let c = jjrf_Coronet("ABCD".to_string());
    assert!(c.jjrf_decode().is_err());

    let c = jjrf_Coronet("ABCDEF".to_string());
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
