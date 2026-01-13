//! Favor encoding - Firemark and Coronet identity types
//!
//! Implements base64 encoding/decoding for Job Jockey identity types:
//! - Firemark: Heat identity (2 base64 chars, 0-4095)
//! - Coronet: Pace identity (5 base64 chars, globally unique)

use serde::{Deserialize, Serialize};

/// URL-safe base64 charset (RFC 4648 section 5)
pub const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Firemark prefix character
pub const FIREMARK_PREFIX: char = '₣';

/// Coronet prefix character
pub const CORONET_PREFIX: char = '₢';

/// Maximum value for Firemark (2^12 - 1)
pub const FIREMARK_MAX: u16 = 4095;

/// Maximum value for Coronet pace index (2^18 - 1)
pub const CORONET_PACE_MAX: u32 = 262143;

/// Look up the position of a character in the charset
fn char_to_value(c: char) -> Result<u8, String> {
    let byte = c as u8;
    CHARSET
        .iter()
        .position(|&b| b == byte)
        .map(|pos| pos as u8)
        .ok_or_else(|| format!("Invalid base64 character: '{}'", c))
}

/// Get the character at a given position in the charset
fn value_to_char(value: u8) -> char {
    CHARSET[value as usize] as char
}

/// Heat identity - 2 base64 characters encoding 0-4095
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Firemark(String);

/// Pace identity - 5 base64 characters, globally unique
/// First 2 chars encode parent Heat, last 3 encode pace index (0-262143)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coronet(String);

impl Firemark {
    /// Encode an integer (0-4095) as a Firemark
    pub fn encode(value: u16) -> Self {
        debug_assert!(value <= FIREMARK_MAX, "Firemark value {} exceeds max {}", value, FIREMARK_MAX);
        let high = value_to_char((value / 64) as u8);
        let low = value_to_char((value % 64) as u8);
        Firemark(format!("{}{}", high, low))
    }

    /// Decode a Firemark to its integer value
    pub fn decode(&self) -> Result<u16, String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != 2 {
            return Err(format!("Firemark must be 2 characters, got {}", chars.len()));
        }
        let high = char_to_value(chars[0])? as u16;
        let low = char_to_value(chars[1])? as u16;
        Ok(high * 64 + low)
    }

    /// Parse a Firemark from string input (with or without prefix)
    pub fn parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(FIREMARK_PREFIX).unwrap_or(input);
        if stripped.len() != 2 {
            return Err(format!(
                "Firemark must be 2 base64 characters (with or without {} prefix), got '{}'",
                FIREMARK_PREFIX, input
            ));
        }
        // Validate all characters are in charset
        for c in stripped.chars() {
            char_to_value(c)?;
        }
        Ok(Firemark(stripped.to_string()))
    }

    /// Get the raw base64 string (without prefix)
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Format with prefix for display
    pub fn display(&self) -> String {
        format!("{}{}", FIREMARK_PREFIX, self.0)
    }
}

impl Coronet {
    /// Encode a Heat identity and pace index as a Coronet
    pub fn encode(heat: &Firemark, pace_index: u32) -> Self {
        debug_assert!(
            pace_index <= CORONET_PACE_MAX,
            "Coronet pace index {} exceeds max {}",
            pace_index,
            CORONET_PACE_MAX
        );
        let p2 = value_to_char((pace_index / 4096) as u8);
        let p1 = value_to_char(((pace_index / 64) % 64) as u8);
        let p0 = value_to_char((pace_index % 64) as u8);
        Coronet(format!("{}{}{}{}", heat.as_str(), p2, p1, p0))
    }

    /// Decode a Coronet to its parent Heat and pace index
    pub fn decode(&self) -> Result<(Firemark, u32), String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != 5 {
            return Err(format!("Coronet must be 5 characters, got {}", chars.len()));
        }
        let heat = Firemark(chars[0..2].iter().collect());
        let p2 = char_to_value(chars[2])? as u32;
        let p1 = char_to_value(chars[3])? as u32;
        let p0 = char_to_value(chars[4])? as u32;
        let pace_index = p2 * 4096 + p1 * 64 + p0;
        Ok((heat, pace_index))
    }

    /// Parse a Coronet from string input (with or without prefix)
    pub fn parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(CORONET_PREFIX).unwrap_or(input);
        if stripped.len() != 5 {
            return Err(format!(
                "Coronet must be 5 base64 characters (with or without {} prefix), got '{}'",
                CORONET_PREFIX, input
            ));
        }
        // Validate all characters are in charset
        for c in stripped.chars() {
            char_to_value(c)?;
        }
        Ok(Coronet(stripped.to_string()))
    }

    /// Get the raw base64 string (without prefix)
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Format with prefix for display
    pub fn display(&self) -> String {
        format!("{}{}", CORONET_PREFIX, self.0)
    }

    /// Extract the parent Firemark (first 2 base64 chars)
    pub fn parent_firemark(&self) -> Firemark {
        Firemark(self.0[..2].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset_length() {
        assert_eq!(CHARSET.len(), 64);
    }

    #[test]
    fn test_charset_values() {
        // Verify specific positions in charset
        assert_eq!(char_to_value('A').unwrap(), 0);
        assert_eq!(char_to_value('B').unwrap(), 1);
        assert_eq!(char_to_value('Z').unwrap(), 25);
        assert_eq!(char_to_value('a').unwrap(), 26);
        assert_eq!(char_to_value('z').unwrap(), 51);
        assert_eq!(char_to_value('0').unwrap(), 52);
        assert_eq!(char_to_value('9').unwrap(), 61);
        assert_eq!(char_to_value('-').unwrap(), 62);
        assert_eq!(char_to_value('_').unwrap(), 63);
    }

    #[test]
    fn test_char_to_value_invalid() {
        assert!(char_to_value('!').is_err());
        assert!(char_to_value(' ').is_err());
        assert!(char_to_value('+').is_err());
        assert!(char_to_value('/').is_err());
    }

    // Firemark tests

    #[test]
    fn test_firemark_encode_zero() {
        let fm = Firemark::encode(0);
        assert_eq!(fm.as_str(), "AA");
        assert_eq!(fm.display(), "₣AA");
    }

    #[test]
    fn test_firemark_encode_one() {
        let fm = Firemark::encode(1);
        assert_eq!(fm.as_str(), "AB");
        assert_eq!(fm.display(), "₣AB");
    }

    #[test]
    fn test_firemark_encode_64() {
        // 64 = 1*64 + 0
        let fm = Firemark::encode(64);
        assert_eq!(fm.as_str(), "BA");
    }

    #[test]
    fn test_firemark_encode_max() {
        // 4095 = 63*64 + 63
        let fm = Firemark::encode(4095);
        assert_eq!(fm.as_str(), "__");
        assert_eq!(fm.display(), "₣__");
    }

    #[test]
    fn test_firemark_encode_decode_roundtrip() {
        for value in [0, 1, 63, 64, 65, 100, 1000, 4000, 4095] {
            let fm = Firemark::encode(value);
            let decoded = fm.decode().unwrap();
            assert_eq!(decoded, value, "Roundtrip failed for {}", value);
        }
    }

    #[test]
    fn test_firemark_decode_invalid_length() {
        let fm = Firemark("A".to_string());
        assert!(fm.decode().is_err());

        let fm = Firemark("ABC".to_string());
        assert!(fm.decode().is_err());
    }

    #[test]
    fn test_firemark_parse_with_prefix() {
        let fm = Firemark::parse("₣AB").unwrap();
        assert_eq!(fm.as_str(), "AB");
        assert_eq!(fm.decode().unwrap(), 1);
    }

    #[test]
    fn test_firemark_parse_without_prefix() {
        let fm = Firemark::parse("AB").unwrap();
        assert_eq!(fm.as_str(), "AB");
        assert_eq!(fm.decode().unwrap(), 1);
    }

    #[test]
    fn test_firemark_parse_invalid() {
        assert!(Firemark::parse("A").is_err());
        assert!(Firemark::parse("ABC").is_err());
        assert!(Firemark::parse("₣A").is_err());
        assert!(Firemark::parse("₣ABC").is_err());
        assert!(Firemark::parse("A!").is_err());
    }

    // Coronet tests

    #[test]
    fn test_coronet_encode_zero() {
        let heat = Firemark::encode(0);
        let coronet = Coronet::encode(&heat, 0);
        assert_eq!(coronet.as_str(), "AAAAA");
        assert_eq!(coronet.display(), "₢AAAAA");
    }

    #[test]
    fn test_coronet_encode_one() {
        let heat = Firemark::encode(0);
        let coronet = Coronet::encode(&heat, 1);
        assert_eq!(coronet.as_str(), "AAAAB");
    }

    #[test]
    fn test_coronet_encode_64() {
        // pace 64 = 0*4096 + 1*64 + 0
        let heat = Firemark::encode(0);
        let coronet = Coronet::encode(&heat, 64);
        assert_eq!(coronet.as_str(), "AAABA");
    }

    #[test]
    fn test_coronet_encode_4096() {
        // pace 4096 = 1*4096 + 0*64 + 0
        let heat = Firemark::encode(0);
        let coronet = Coronet::encode(&heat, 4096);
        assert_eq!(coronet.as_str(), "AABAA");
    }

    #[test]
    fn test_coronet_encode_max_pace() {
        // 262143 = 63*4096 + 63*64 + 63
        let heat = Firemark::encode(0);
        let coronet = Coronet::encode(&heat, 262143);
        assert_eq!(coronet.as_str(), "AA___");
    }

    #[test]
    fn test_coronet_encode_with_nonzero_heat() {
        let heat = Firemark::encode(1); // "AB"
        let coronet = Coronet::encode(&heat, 0);
        assert_eq!(coronet.as_str(), "ABAAA");
        assert_eq!(coronet.display(), "₢ABAAA");
    }

    #[test]
    fn test_coronet_decode_roundtrip() {
        for heat_val in [0, 1, 100, 4095] {
            for pace_val in [0, 1, 63, 64, 4096, 100000, 262143] {
                let heat = Firemark::encode(heat_val);
                let coronet = Coronet::encode(&heat, pace_val);
                let (decoded_heat, decoded_pace) = coronet.decode().unwrap();
                assert_eq!(
                    decoded_heat.decode().unwrap(),
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
        let heat = Firemark::encode(42);
        let coronet = Coronet::encode(&heat, 123);
        let parent = coronet.parent_firemark();
        assert_eq!(parent.as_str(), heat.as_str());
        assert_eq!(parent.decode().unwrap(), 42);
    }

    #[test]
    fn test_coronet_parse_with_prefix() {
        let coronet = Coronet::parse("₢ABAAA").unwrap();
        assert_eq!(coronet.as_str(), "ABAAA");
        let (heat, pace) = coronet.decode().unwrap();
        assert_eq!(heat.decode().unwrap(), 1);
        assert_eq!(pace, 0);
    }

    #[test]
    fn test_coronet_parse_without_prefix() {
        let coronet = Coronet::parse("ABAAA").unwrap();
        assert_eq!(coronet.as_str(), "ABAAA");
    }

    #[test]
    fn test_coronet_parse_invalid() {
        assert!(Coronet::parse("ABAA").is_err());
        assert!(Coronet::parse("ABAAAA").is_err());
        assert!(Coronet::parse("₢ABAA").is_err());
        assert!(Coronet::parse("ABA!A").is_err());
    }

    #[test]
    fn test_coronet_decode_invalid_length() {
        let c = Coronet("ABCD".to_string());
        assert!(c.decode().is_err());

        let c = Coronet("ABCDEF".to_string());
        assert!(c.decode().is_err());
    }

    // Edge case tests

    #[test]
    fn test_specific_encoding_example() {
        // Example from spec: charset[H/64] + charset[H%64]
        // H = 65 -> 65/64 = 1, 65%64 = 1 -> "BB"
        let fm = Firemark::encode(65);
        assert_eq!(fm.as_str(), "BB");
    }

    #[test]
    fn test_all_charset_positions_encode_decode() {
        // Ensure every charset position works
        for i in 0..64 {
            let fm = Firemark::encode(i as u16);
            let decoded = fm.decode().unwrap();
            assert_eq!(decoded, i as u16);
        }
    }
}
