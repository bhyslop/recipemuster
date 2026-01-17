// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

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
pub fn zjjrf_char_to_value(c: char) -> Result<u8, String> {
    let byte = c as u8;
    JJRF_CHARSET
        .iter()
        .position(|&b| b == byte)
        .map(|pos| pos as u8)
        .ok_or_else(|| format!("Invalid base64 character: '{}'", c))
}

/// Get the character at a given position in the charset
pub fn zjjrf_value_to_char(value: u8) -> char {
    JJRF_CHARSET[value as usize] as char
}

/// Heat identity - 2 base64 characters encoding 0-4095
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Firemark(pub String);

/// Pace identity - 5 base64 characters, globally unique
/// First 2 chars encode parent Heat, last 3 encode pace index (0-262143)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Coronet(pub String);

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
