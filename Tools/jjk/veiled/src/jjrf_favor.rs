//! Favor encoding - Firemark and Coronet identity types
//!
//! Implements base64 encoding/decoding for Job Jockey identity types:
//! - Firemark: Heat identity (2 base64 chars, 0-4095)
//! - Coronet: Pace identity (5 base64 chars, globally unique)

use serde::{Deserialize, Serialize};

/// URL-safe base64 charset (RFC 4648 section 5)
pub const JJRF_CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Firemark prefix character
pub const JJRF_FIREMARK_PREFIX: char = '₣';

/// Coronet prefix character
pub const JJRF_CORONET_PREFIX: char = '₢';

/// Maximum value for Firemark (2^12 - 1)
pub const JJRF_FIREMARK_MAX: u16 = 4095;

/// Maximum value for Coronet pace index (2^18 - 1)
pub const JJRF_CORONET_PACE_MAX: u32 = 262143;

/// Look up the position of a character in the charset
fn zjjrf_char_to_value(c: char) -> Result<u8, String> {
    let byte = c as u8;
    JJRF_CHARSET
        .iter()
        .position(|&b| b == byte)
        .map(|pos| pos as u8)
        .ok_or_else(|| format!("Invalid base64 character: '{}'", c))
}

/// Get the character at a given position in the charset
fn zjjrf_value_to_char(value: u8) -> char {
    JJRF_CHARSET[value as usize] as char
}

/// Heat identity - 2 base64 characters encoding 0-4095
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Firemark(String);

/// Pace identity - 5 base64 characters, globally unique
/// First 2 chars encode parent Heat, last 3 encode pace index (0-262143)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Coronet(String);

impl jjrf_Firemark {
    /// Encode an integer (0-4095) as a Firemark
    pub fn jjrf_encode(value: u16) -> Self {
        debug_assert!(value <= JJRF_FIREMARK_MAX, "Firemark value {} exceeds max {}", value, JJRF_FIREMARK_MAX);
        let high = zjjrf_value_to_char((value / 64) as u8);
        let low = zjjrf_value_to_char((value % 64) as u8);
        jjrf_Firemark(format!("{}{}", high, low))
    }

    /// Decode a Firemark to its integer value
    pub fn jjrf_decode(&self) -> Result<u16, String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != 2 {
            return Err(format!("Firemark must be 2 characters, got {}", chars.len()));
        }
        let high = zjjrf_char_to_value(chars[0])? as u16;
        let low = zjjrf_char_to_value(chars[1])? as u16;
        Ok(high * 64 + low)
    }

    /// Parse a Firemark from string input (with or without prefix)
    pub fn jjrf_parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(JJRF_FIREMARK_PREFIX).unwrap_or(input);
        if stripped.len() != 2 {
            return Err(format!(
                "Firemark must be 2 base64 characters (with or without {} prefix), got '{}'",
                JJRF_FIREMARK_PREFIX, input
            ));
        }
        // Validate all characters are in charset
        for c in stripped.chars() {
            zjjrf_char_to_value(c)?;
        }
        Ok(jjrf_Firemark(stripped.to_string()))
    }

    /// Get the raw base64 string (without prefix)
    pub fn jjrf_as_str(&self) -> &str {
        &self.0
    }

    /// Format with prefix for display
    pub fn jjrf_display(&self) -> String {
        format!("{}{}", JJRF_FIREMARK_PREFIX, self.0)
    }
}

impl jjrf_Coronet {
    /// Encode a Heat identity and pace index as a Coronet
    pub fn jjrf_encode(heat: &jjrf_Firemark, pace_index: u32) -> Self {
        debug_assert!(
            pace_index <= JJRF_CORONET_PACE_MAX,
            "Coronet pace index {} exceeds max {}",
            pace_index,
            JJRF_CORONET_PACE_MAX
        );
        let p2 = zjjrf_value_to_char((pace_index / 4096) as u8);
        let p1 = zjjrf_value_to_char(((pace_index / 64) % 64) as u8);
        let p0 = zjjrf_value_to_char((pace_index % 64) as u8);
        jjrf_Coronet(format!("{}{}{}{}", heat.jjrf_as_str(), p2, p1, p0))
    }

    /// Decode a Coronet to its parent Heat and pace index
    pub fn jjrf_decode(&self) -> Result<(jjrf_Firemark, u32), String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != 5 {
            return Err(format!("Coronet must be 5 characters, got {}", chars.len()));
        }
        let heat = jjrf_Firemark(chars[0..2].iter().collect());
        let p2 = zjjrf_char_to_value(chars[2])? as u32;
        let p1 = zjjrf_char_to_value(chars[3])? as u32;
        let p0 = zjjrf_char_to_value(chars[4])? as u32;
        let pace_index = p2 * 4096 + p1 * 64 + p0;
        Ok((heat, pace_index))
    }

    /// Parse a Coronet from string input (with or without prefix)
    pub fn jjrf_parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(JJRF_CORONET_PREFIX).unwrap_or(input);
        if stripped.len() != 5 {
            return Err(format!(
                "Coronet must be 5 base64 characters (with or without {} prefix), got '{}'",
                JJRF_CORONET_PREFIX, input
            ));
        }
        // Validate all characters are in charset
        for c in stripped.chars() {
            zjjrf_char_to_value(c)?;
        }
        Ok(jjrf_Coronet(stripped.to_string()))
    }

    /// Get the raw base64 string (without prefix)
    pub fn jjrf_as_str(&self) -> &str {
        &self.0
    }

    /// Format with prefix for display
    pub fn jjrf_display(&self) -> String {
        format!("{}{}", JJRF_CORONET_PREFIX, self.0)
    }

    /// Extract the parent Firemark (first 2 base64 chars)
    pub fn jjrf_parent_firemark(&self) -> jjrf_Firemark {
        jjrf_Firemark(self.0[..2].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset_length() {
        assert_eq!(JJRF_CHARSET.len(), 64);
    }

    #[test]
    fn test_charset_values() {
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
    fn test_char_to_value_invalid() {
        assert!(zjjrf_char_to_value('!').is_err());
        assert!(zjjrf_char_to_value(' ').is_err());
        assert!(zjjrf_char_to_value('+').is_err());
        assert!(zjjrf_char_to_value('/').is_err());
    }

    // Firemark tests

    #[test]
    fn test_firemark_encode_zero() {
        let fm = jjrf_Firemark::jjrf_encode(0);
        assert_eq!(fm.as_str(), "AA");
        assert_eq!(fm.jjrf_display(), "₣AA");
    }

    #[test]
    fn test_firemark_encode_one() {
        let fm = jjrf_Firemark::jjrf_encode(1);
        assert_eq!(fm.as_str(), "AB");
        assert_eq!(fm.jjrf_display(), "₣AB");
    }

    #[test]
    fn test_firemark_encode_64() {
        // 64 = 1*64 + 0
        let fm = jjrf_Firemark::jjrf_encode(64);
        assert_eq!(fm.as_str(), "BA");
    }

    #[test]
    fn test_firemark_encode_max() {
        // 4095 = 63*64 + 63
        let fm = jjrf_Firemark::jjrf_encode(4095);
        assert_eq!(fm.as_str(), "__");
        assert_eq!(fm.jjrf_display(), "₣__");
    }

    #[test]
    fn test_firemark_encode_decode_roundtrip() {
        for value in [0, 1, 63, 64, 65, 100, 1000, 4000, 4095] {
            let fm = jjrf_Firemark::jjrf_encode(value);
            let decoded = fm.jjrf_decode().unwrap();
            assert_eq!(decoded, value, "Roundtrip failed for {}", value);
        }
    }

    #[test]
    fn test_firemark_decode_invalid_length() {
        let fm = jjrf_Firemark("A".to_string());
        assert!(fm.jjrf_decode().is_err());

        let fm = jjrf_Firemark("ABC".to_string());
        assert!(fm.jjrf_decode().is_err());
    }

    #[test]
    fn test_firemark_parse_with_prefix() {
        let fm = jjrf_Firemark::jjrf_parse("₣AB").unwrap();
        assert_eq!(fm.as_str(), "AB");
        assert_eq!(fm.jjrf_decode().unwrap(), 1);
    }

    #[test]
    fn test_firemark_parse_without_prefix() {
        let fm = jjrf_Firemark::jjrf_parse("AB").unwrap();
        assert_eq!(fm.as_str(), "AB");
        assert_eq!(fm.jjrf_decode().unwrap(), 1);
    }

    #[test]
    fn test_firemark_parse_invalid() {
        assert!(jjrf_Firemark::jjrf_parse("A").is_err());
        assert!(jjrf_Firemark::jjrf_parse("ABC").is_err());
        assert!(jjrf_Firemark::jjrf_parse("₣A").is_err());
        assert!(jjrf_Firemark::jjrf_parse("₣ABC").is_err());
        assert!(jjrf_Firemark::jjrf_parse("A!").is_err());
    }

    // Coronet tests

    #[test]
    fn test_coronet_encode_zero() {
        let heat = jjrf_Firemark::jjrf_encode(0);
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 0);
        assert_eq!(coronet.as_str(), "AAAAA");
        assert_eq!(coronet.jjrf_display(), "₢AAAAA");
    }

    #[test]
    fn test_coronet_encode_one() {
        let heat = jjrf_Firemark::jjrf_encode(0);
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 1);
        assert_eq!(coronet.as_str(), "AAAAB");
    }

    #[test]
    fn test_coronet_encode_64() {
        // pace 64 = 0*4096 + 1*64 + 0
        let heat = jjrf_Firemark::jjrf_encode(0);
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 64);
        assert_eq!(coronet.as_str(), "AAABA");
    }

    #[test]
    fn test_coronet_encode_4096() {
        // pace 4096 = 1*4096 + 0*64 + 0
        let heat = jjrf_Firemark::jjrf_encode(0);
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 4096);
        assert_eq!(coronet.as_str(), "AABAA");
    }

    #[test]
    fn test_coronet_encode_max_pace() {
        // 262143 = 63*4096 + 63*64 + 63
        let heat = jjrf_Firemark::jjrf_encode(0);
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 262143);
        assert_eq!(coronet.as_str(), "AA___");
    }

    #[test]
    fn test_coronet_encode_with_nonzero_heat() {
        let heat = jjrf_Firemark::jjrf_encode(1); // "AB"
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 0);
        assert_eq!(coronet.as_str(), "ABAAA");
        assert_eq!(coronet.jjrf_display(), "₢ABAAA");
    }

    #[test]
    fn test_coronet_decode_roundtrip() {
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
    fn test_coronet_parent_firemark() {
        let heat = jjrf_Firemark::jjrf_encode(42);
        let coronet = jjrf_Coronet::jjrf_encode(&heat, 123);
        let parent = coronet.jjrf_parent_firemark();
        assert_eq!(parent.as_str(), heat.as_str());
        assert_eq!(parent.jjrf_decode().unwrap(), 42);
    }

    #[test]
    fn test_coronet_parse_with_prefix() {
        let coronet = jjrf_Coronet::jjrf_parse("₢ABAAA").unwrap();
        assert_eq!(coronet.as_str(), "ABAAA");
        let (heat, pace) = coronet.jjrf_decode().unwrap();
        assert_eq!(heat.jjrf_decode().unwrap(), 1);
        assert_eq!(pace, 0);
    }

    #[test]
    fn test_coronet_parse_without_prefix() {
        let coronet = jjrf_Coronet::jjrf_parse("ABAAA").unwrap();
        assert_eq!(coronet.as_str(), "ABAAA");
    }

    #[test]
    fn test_coronet_parse_invalid() {
        assert!(jjrf_Coronet::jjrf_parse("ABAA").is_err());
        assert!(jjrf_Coronet::jjrf_parse("ABAAAA").is_err());
        assert!(jjrf_Coronet::jjrf_parse("₢ABAA").is_err());
        assert!(jjrf_Coronet::jjrf_parse("ABA!A").is_err());
    }

    #[test]
    fn test_coronet_decode_invalid_length() {
        let c = jjrf_Coronet("ABCD".to_string());
        assert!(c.jjrf_decode().is_err());

        let c = jjrf_Coronet("ABCDEF".to_string());
        assert!(c.jjrf_decode().is_err());
    }

    // Edge case tests

    #[test]
    fn test_specific_encoding_example() {
        // Example from spec: charset[H/64] + charset[H%64]
        // H = 65 -> 65/64 = 1, 65%64 = 1 -> "BB"
        let fm = jjrf_Firemark::jjrf_encode(65);
        assert_eq!(fm.as_str(), "BB");
    }

    #[test]
    fn test_all_charset_positions_encode_decode() {
        // Ensure every charset position works
        for i in 0..64 {
            let fm = jjrf_Firemark::jjrf_encode(i as u16);
            let decoded = fm.jjrf_decode().unwrap();
            assert_eq!(decoded, i as u16);
        }
    }
}
