// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Favor encoding — the Job Jockey insignia types (the `axd_insignia` family).
//!
//! Base64url encoding/decoding for the four minted identity marks:
//! - Firemark: Heat identity (2 base64url chars, 0-4095) — seeded
//! - Coronet:  Pace identity (5 base64url chars, globally unique) — seeded
//! - Pensum:   Remote dispatch identity (5 chars with `%` sentinel) — seeded
//! - Incipit:  Officium identity (`YYMMDD-NNNN` + discriminant) — temporal
//!
//! Value vs. carriage (AXLA Minted-Mark Dimensions, cited by JJS0
//! `jjdt_insignia`): the identity IS the bare encoded body (`jjrf_as_str`); the
//! unicode sigil is the type sentinel, carried surface-keyed — mandatory in
//! operator-facing output and in project-authored structured wires
//! (`jjrf_display` serves both), forbidden on foreign-traversed surfaces (git
//! refs, on-disk paths), tolerant on input. `zjjrf_emblazon` is the single
//! render home that applies a sigil to a bare body. The body is set at
//! construction and never after (encapsulated field). Derivation is generative:
//! `jjrf_successor` / `jjrf_parent_firemark` return a fresh immutable value,
//! never mutating the source — a seeded-nature affordance (arithmetic on the
//! base-64 numeral), so the temporal Incipit carries no successor.

use serde::{Deserialize, Serialize};

/// URL-safe base64 charset (RFC 4648 section 5)
pub const JJRF_CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Encoding radix — derived from charset length
pub const JJRF_RADIX: u32 = JJRF_CHARSET.len() as u32;

/// Firemark prefix character
pub const JJRF_FIREMARK_PREFIX: char = '₣';

/// Coronet prefix character
pub const JJRF_CORONET_PREFIX: char = '₢';

/// Pensum prefix character
pub const JJRF_PENSUM_PREFIX: char = '₱';

/// Incipit (officium) prefix character — the ☉ temporal sigil (U+2609 SUN)
pub const JJRF_INCIPIT_PREFIX: char = '☉';

/// Pensum sentinel character — literal `%` at position 3 disambiguates from Coronet
pub const JJRF_PENSUM_SENTINEL: char = '%';

/// Firemark body length — base64 chars after the optional ₣ prefix (heat identity)
pub const JJRF_FIREMARK_LEN: usize = 2;

/// Coronet body length — a flat 5-char global pace index (JJS0 jjdt_coronet:
/// one flat index, no embedded heat).
pub const JJRF_CORONET_LEN: usize = 5;

/// Pensum body length — firemark + `%` sentinel + 2-char index (2 + 1 + 2)
pub const JJRF_PENSUM_LEN: usize = 5;

/// Maximum value for Firemark (RADIX^2 - 1)
pub const JJRF_FIREMARK_MAX: u16 = (JJRF_RADIX * JJRF_RADIX - 1) as u16;

/// Maximum Coronet index (RADIX^5 - 1) — the flat global pace-id space (~1.07B).
pub const JJRF_CORONET_MAX: u32 = JJRF_RADIX.pow(5) - 1;

/// Maximum value for Pensum index (RADIX^2 - 1)
pub const JJRF_PENSUM_INDEX_MAX: u32 = JJRF_RADIX * JJRF_RADIX - 1;

/// The heat-qualifier separator in a Coronet's display form (JJS0 jjdt_coronet):
/// `₢` + current-heat firemark + this interpunct + the 5-char body. Outside the
/// charset (like the pensum `%` sentinel), so ingest splits it mechanically.
pub const JJRF_CORONET_QUALIFIER: char = '·';

/// Founding floor for the global pace seed (JJS0 jjdgm_pace_seed): a fresh
/// gallops starts here, and the reprieve write-forward founds at
/// max(highest existing index + 1, this). Every grandfathered id leads with A or
/// B, so this floor makes every seed-minted id lead with C or later.
pub const JJRF_CORONET_SEED_FLOOR: &str = "CAAAA";

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

/// The single render home for insignia: apply a type-sentinel `sigil` to a bare
/// encoded `body`, yielding the sigiled form for sentinel-carrying surfaces —
/// operator-facing output and project-authored structured wires. The one place a
/// sigil is prepended (AXLA Minted-Mark carriage law, via JJS0 `jjdt_insignia`).
/// The bare body alone rides foreign-traversed surfaces (git refs, on-disk paths).
fn zjjrf_emblazon(sigil: char, body: &str) -> String {
    format!("{}{}", sigil, body)
}

/// Emblazon a revision ordinal (`axd_ordinal`, AXLA Minted-Mark Dimensions) in
/// its operator-facing sigiled form. Unlike the four insignia types below, an
/// ordinal is annotative, not identity — a denormalized order label aliasing a
/// pre-existing SHA, never truth — so it carries no dedicated type, only this
/// pass through the shared `zjjrf_emblazon` render home. Bare decimal is
/// charset-valid, so an ordinal must never circulate glyphless: callers apply
/// this at every point the ordinal reaches operator-facing output.
pub fn jjrf_emblazon_ordinal(sigil: char, ordinal: u64) -> String {
    zjjrf_emblazon(sigil, &ordinal.to_string())
}

/// Heat identity - 2 base64 characters encoding 0-4095
/// The body is private — set at construction, immutable thereafter (`axd_immutable`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Firemark(String);

/// Pace identity - 5 base64 characters, one flat global index (0..=RADIX^5-1).
/// Immutable for life (JJS0 jjdt_coronet): minted once from the global seed,
/// carries no parent Heat — resolution scans heats' paces, never infers.
/// The body is private — set at construction, immutable thereafter (`axd_immutable`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Coronet(String);

/// Pensum identity - 5 characters, globally unique
/// Format: 2 base64url (heat firemark) + literal `%` + 2 base64url (index within heat)
/// Example: `Ah%BE` belongs to heat `₣Ah`
/// The body is private — set at construction, immutable thereafter (`axd_immutable`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Pensum(String);

impl jjrf_Firemark {
    /// Encode an integer (0-4095) as a Firemark
    pub fn jjrf_encode(value: u16) -> Self {
        debug_assert!(value <= JJRF_FIREMARK_MAX, "Firemark value {} exceeds max {}", value, JJRF_FIREMARK_MAX);
        let radix = JJRF_RADIX as u16;
        let high = zjjrf_value_to_char((value / radix) as u8);
        let low = zjjrf_value_to_char((value % radix) as u8);
        jjrf_Firemark(format!("{}{}", high, low))
    }

    /// Decode a Firemark to its integer value
    pub fn jjrf_decode(&self) -> Result<u16, String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != JJRF_FIREMARK_LEN {
            return Err(format!("Firemark must be {} characters, got {}", JJRF_FIREMARK_LEN, chars.len()));
        }
        let high = zjjrf_char_to_value(chars[0])? as u16;
        let low = zjjrf_char_to_value(chars[1])? as u16;
        Ok(high * JJRF_RADIX as u16 + low)
    }

    /// Parse a Firemark from string input (with or without prefix)
    pub fn jjrf_parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(JJRF_FIREMARK_PREFIX).unwrap_or(input);
        if stripped.len() != JJRF_FIREMARK_LEN {
            return Err(format!(
                "Firemark must be {} base64 characters (with or without {} prefix), got '{}'",
                JJRF_FIREMARK_LEN, JJRF_FIREMARK_PREFIX, input
            ));
        }
        // Validate all characters are in charset
        for c in stripped.chars() {
            zjjrf_char_to_value(c)?;
        }
        Ok(jjrf_Firemark(stripped.to_string()))
    }

    /// Get the raw base64 string (the bare body — the identity, sigil stripped)
    pub fn jjrf_as_str(&self) -> &str {
        &self.0
    }

    /// The render-layer sigil (`₣`). Render-only — never part of the identity.
    pub fn jjrf_sigil(&self) -> char {
        JJRF_FIREMARK_PREFIX
    }

    /// Operator-facing form: sigil + bare body (sigil mandatory)
    pub fn jjrf_display(&self) -> String {
        zjjrf_emblazon(self.jjrf_sigil(), self.jjrf_as_str())
    }

    /// Seeded successor — the next Firemark in seed order (this value + 1) as a
    /// fresh immutable value; the source is untouched. `Err` when the base-64
    /// numeral is saturated (no successor within capacity).
    pub fn jjrf_successor(&self) -> Result<jjrf_Firemark, String> {
        let value = self.jjrf_decode()?;
        if value >= JJRF_FIREMARK_MAX {
            return Err(format!(
                "Firemark '{}' is at capacity ({}), no successor",
                self.jjrf_display(), JJRF_FIREMARK_MAX
            ));
        }
        Ok(jjrf_Firemark::jjrf_encode(value + 1))
    }

    /// Test-only raw wrap — build from an unvalidated body to exercise decode /
    /// error paths that validated construction (`jjrf_encode` / `jjrf_parse`)
    /// can never produce. Unavailable outside tests: production identities are
    /// always validated at construction and immutable thereafter.
    #[cfg(test)]
    pub fn jjrf_from_raw(body: &str) -> Self {
        jjrf_Firemark(body.to_string())
    }
}

impl jjrf_Coronet {
    /// Encode a flat global pace index as a Coronet (JJS0 jjdt_coronet: one flat
    /// 5-char index, no embedded heat).
    pub fn jjrf_encode(index: u32) -> Self {
        debug_assert!(index <= JJRF_CORONET_MAX, "Coronet index {} exceeds max {}", index, JJRF_CORONET_MAX);
        let mut n = index;
        let mut buf = [0u8; JJRF_CORONET_LEN];
        for slot in buf.iter_mut().rev() {
            *slot = JJRF_CHARSET[(n % JJRF_RADIX) as usize];
            n /= JJRF_RADIX;
        }
        jjrf_Coronet(String::from_utf8(buf.to_vec()).expect("charset bytes are ASCII"))
    }

    /// Decode a Coronet to its flat global index.
    pub fn jjrf_decode(&self) -> Result<u32, String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != JJRF_CORONET_LEN {
            return Err(format!("Coronet must be {} characters, got {}", JJRF_CORONET_LEN, chars.len()));
        }
        let mut value: u32 = 0;
        for c in chars {
            value = value * JJRF_RADIX + zjjrf_char_to_value(c)? as u32;
        }
        Ok(value)
    }

    /// Parse a Coronet from string input (with or without prefix)
    pub fn jjrf_parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(JJRF_CORONET_PREFIX).unwrap_or(input);
        if stripped.len() != JJRF_CORONET_LEN {
            return Err(format!(
                "Coronet must be {} base64 characters (with or without {} prefix), got '{}'",
                JJRF_CORONET_LEN, JJRF_CORONET_PREFIX, input
            ));
        }
        // Validate all characters are in charset
        for c in stripped.chars() {
            zjjrf_char_to_value(c)?;
        }
        Ok(jjrf_Coronet(stripped.to_string()))
    }

    /// Get the raw base64 string (the bare body — the identity, sigil stripped)
    pub fn jjrf_as_str(&self) -> &str {
        &self.0
    }

    /// The render-layer sigil (`₢`). Render-only — never part of the identity.
    pub fn jjrf_sigil(&self) -> char {
        JJRF_CORONET_PREFIX
    }

    /// Operator-facing form: sigil + bare body (sigil mandatory)
    pub fn jjrf_display(&self) -> String {
        zjjrf_emblazon(self.jjrf_sigil(), self.jjrf_as_str())
    }

    /// Seeded successor — the next Coronet in the global index (index + 1) as a
    /// fresh immutable value; the source is untouched. `Err` when the index is
    /// saturated (no successor within capacity).
    pub fn jjrf_successor(&self) -> Result<jjrf_Coronet, String> {
        let index = self.jjrf_decode()?;
        if index >= JJRF_CORONET_MAX {
            return Err(format!(
                "Coronet '{}' index is at capacity ({}), no successor",
                self.jjrf_display(), JJRF_CORONET_MAX
            ));
        }
        Ok(jjrf_Coronet::jjrf_encode(index + 1))
    }

    /// Test-only raw wrap — see `jjrf_Firemark::jjrf_from_raw`.
    #[cfg(test)]
    pub fn jjrf_from_raw(body: &str) -> Self {
        jjrf_Coronet(body.to_string())
    }
}

impl jjrf_Pensum {
    /// Encode a Heat firemark and index as a Pensum
    pub fn jjrf_encode(heat: &jjrf_Firemark, index: u32) -> Self {
        debug_assert!(
            index <= JJRF_PENSUM_INDEX_MAX,
            "Pensum index {} exceeds max {}",
            index,
            JJRF_PENSUM_INDEX_MAX
        );
        let high = zjjrf_value_to_char((index / JJRF_RADIX) as u8);
        let low = zjjrf_value_to_char((index % JJRF_RADIX) as u8);
        jjrf_Pensum(format!("{}{}{}{}",
            heat.jjrf_as_str(),
            JJRF_PENSUM_SENTINEL,
            high,
            low,
        ))
    }

    /// Decode a Pensum to its parent Heat firemark and index
    pub fn jjrf_decode(&self) -> Result<(jjrf_Firemark, u32), String> {
        let chars: Vec<char> = self.0.chars().collect();
        if chars.len() != JJRF_PENSUM_LEN {
            return Err(format!("Pensum must be {} characters, got {}", JJRF_PENSUM_LEN, chars.len()));
        }
        if chars[2] != JJRF_PENSUM_SENTINEL {
            return Err(format!(
                "Pensum must have '{}' sentinel at position 3, got '{}'",
                JJRF_PENSUM_SENTINEL, chars[2]
            ));
        }
        let heat = jjrf_Firemark(chars[0..JJRF_FIREMARK_LEN].iter().collect());
        let high = zjjrf_char_to_value(chars[3])? as u32;
        let low = zjjrf_char_to_value(chars[4])? as u32;
        Ok((heat, high * JJRF_RADIX + low))
    }

    /// Parse a Pensum from string input (with or without prefix)
    pub fn jjrf_parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(JJRF_PENSUM_PREFIX).unwrap_or(input);
        if stripped.len() != JJRF_PENSUM_LEN {
            return Err(format!(
                "Pensum must be {} characters (with or without {} prefix), got '{}'",
                JJRF_PENSUM_LEN, JJRF_PENSUM_PREFIX, input
            ));
        }
        let chars: Vec<char> = stripped.chars().collect();
        if chars[2] != JJRF_PENSUM_SENTINEL {
            return Err(format!(
                "Pensum must have '{}' sentinel at position 3, got '{}'",
                JJRF_PENSUM_SENTINEL, chars[2]
            ));
        }
        // Validate firemark chars (positions 0-1)
        zjjrf_char_to_value(chars[0])?;
        zjjrf_char_to_value(chars[1])?;
        // Validate index chars (positions 3-4)
        zjjrf_char_to_value(chars[3])?;
        zjjrf_char_to_value(chars[4])?;
        Ok(jjrf_Pensum(stripped.to_string()))
    }

    /// Get the raw string (the bare body — the identity, sigil stripped)
    pub fn jjrf_as_str(&self) -> &str {
        &self.0
    }

    /// The render-layer sigil (`₱`). Render-only — never part of the identity.
    pub fn jjrf_sigil(&self) -> char {
        JJRF_PENSUM_PREFIX
    }

    /// Operator-facing form: sigil + bare body (sigil mandatory)
    pub fn jjrf_display(&self) -> String {
        zjjrf_emblazon(self.jjrf_sigil(), self.jjrf_as_str())
    }

    /// Extract the parent Firemark (first 2 base64 chars) — a generative
    /// derivation: a fresh immutable value, the source untouched.
    pub fn jjrf_parent_firemark(&self) -> jjrf_Firemark {
        jjrf_Firemark(self.0[..JJRF_FIREMARK_LEN].to_string())
    }

    /// Seeded successor — the next Pensum in the same heat (index + 1) as a
    /// fresh immutable value; the source is untouched. `Err` when the index is
    /// saturated (no successor within capacity).
    pub fn jjrf_successor(&self) -> Result<jjrf_Pensum, String> {
        let (heat, index) = self.jjrf_decode()?;
        if index >= JJRF_PENSUM_INDEX_MAX {
            return Err(format!(
                "Pensum '{}' index is at capacity ({}), no successor",
                self.jjrf_display(), JJRF_PENSUM_INDEX_MAX
            ));
        }
        Ok(jjrf_Pensum::jjrf_encode(&heat, index + 1))
    }

    /// Test-only raw wrap — see `jjrf_Firemark::jjrf_from_raw`.
    #[cfg(test)]
    pub fn jjrf_from_raw(body: &str) -> Self {
        jjrf_Pensum(body.to_string())
    }
}

/// Officium identity - temporal insignia (JJS0 `jjdt_incipit`)
/// Format: `YYMMDD-NNNN` datestamp + autonumber, plus a cross-machine random
/// discriminant. Minted by filesystem enumeration, not arithmetic — the
/// `axd_temporal` nature, not seeded — so it carries no successor.
/// The body is private — set at construction, immutable thereafter (`axd_immutable`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct jjrf_Incipit(String);

impl jjrf_Incipit {
    /// Wrap an already-minted bare body (the officium directory name, sigil
    /// stripped) as an Incipit. The mint — filesystem enumeration — is the
    /// caller's; this is the typed home the minted value lands in.
    pub fn jjrf_new(body: impl Into<String>) -> Self {
        jjrf_Incipit(body.into())
    }

    /// Parse an Incipit from input with or without the `☉` sigil (sigil ignored
    /// on input). Temporal bodies are filesystem-derived, so the only structural
    /// check is non-emptiness.
    pub fn jjrf_parse(input: &str) -> Result<Self, String> {
        let stripped = input.strip_prefix(JJRF_INCIPIT_PREFIX).unwrap_or(input);
        if stripped.is_empty() {
            return Err(format!(
                "Incipit must be non-empty (with or without {} prefix), got '{}'",
                JJRF_INCIPIT_PREFIX, input
            ));
        }
        Ok(jjrf_Incipit(stripped.to_string()))
    }

    /// The bare body — the identity itself, and the on-disk directory name
    /// (sigil stripped; the machine form).
    pub fn jjrf_as_str(&self) -> &str {
        &self.0
    }

    /// The render-layer sigil (`☉`). Render-only — never part of the identity.
    pub fn jjrf_sigil(&self) -> char {
        JJRF_INCIPIT_PREFIX
    }

    /// Operator-facing form: sigil + bare body (sigil mandatory)
    pub fn jjrf_display(&self) -> String {
        zjjrf_emblazon(self.jjrf_sigil(), self.jjrf_as_str())
    }
}
