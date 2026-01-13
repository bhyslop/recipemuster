//! Gallops JSON operations
//!
//! Implements read/write operations on the Gallops JSON store.
//! All operations are atomic (write to temp, then rename).

use serde::{Deserialize, Serialize};
use crate::jjrf_favor::CHARSET;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Pace state values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaceState {
    Rough,
    Primed,
    Complete,
    Abandoned,
}

/// Heat status values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HeatStatus {
    Current,
    Retired,
}

/// Tack record - snapshot of Pace state and plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tack {
    pub ts: String,
    pub state: PaceState,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// Pace record - discrete action within a Heat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pace {
    pub silks: String,
    pub tacks: Vec<Tack>,
}

/// Heat record - bounded initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heat {
    pub silks: String,
    pub creation_time: String,
    pub status: HeatStatus,
    pub order: Vec<String>,
    pub next_pace_seed: String,
    pub paddock_file: String,
    pub paces: std::collections::HashMap<String, Pace>,
}

/// Root Gallops structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gallops {
    pub next_heat_seed: String,
    pub heats: std::collections::HashMap<String, Heat>,
}

// Validation helper functions

/// Check if string contains only URL-safe base64 characters
fn is_base64(s: &str) -> bool {
    s.bytes().all(|b| CHARSET.contains(&b))
}

/// Check if string is valid kebab-case (non-empty, lowercase alphanumeric with hyphens)
fn is_kebab_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Pattern: [a-z0-9]+(-[a-z0-9]+)*
    let parts: Vec<&str> = s.split('-').collect();
    for part in parts {
        if part.is_empty() {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return false;
        }
    }
    true
}

/// Check if string matches YYMMDD format
fn is_yymmdd(s: &str) -> bool {
    if s.len() != 6 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_digit())
}

/// Check if string matches YYMMDD-HHMM format
fn is_yymmdd_hhmm(s: &str) -> bool {
    if s.len() != 11 {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    parts[0].len() == 6
        && parts[0].chars().all(|c| c.is_ascii_digit())
        && parts[1].len() == 4
        && parts[1].chars().all(|c| c.is_ascii_digit())
}

impl Gallops {
    /// Load Gallops from a file path
    pub fn load(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

        let gallops: Gallops = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(gallops)
    }

    /// Save Gallops to a file path (atomic write)
    pub fn save(&self, path: &Path) -> Result<(), String> {
        // Serialize to pretty JSON
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        // Create temp file in same directory for atomic rename
        let parent = path.parent().ok_or_else(|| "Invalid path: no parent directory".to_string())?;
        let temp_path = parent.join(format!(".tmp.{}.json", std::process::id()));

        // Write to temp file
        let mut file = fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;
        file.sync_all()
            .map_err(|e| format!("Failed to sync temp file: {}", e))?;
        drop(file);

        // Atomic rename
        fs::rename(&temp_path, path)
            .map_err(|e| format!("Failed to rename temp file to target: {}", e))?;

        Ok(())
    }

    /// Validate the Gallops structure
    ///
    /// Returns Ok(()) if valid, Err(Vec<String>) with all validation errors otherwise.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Rule 1: next_heat_seed must be 2 URL-safe base64 characters
        if self.next_heat_seed.len() != 2 {
            errors.push(format!(
                "next_heat_seed must be 2 characters, got {}",
                self.next_heat_seed.len()
            ));
        } else if !is_base64(&self.next_heat_seed) {
            errors.push(format!(
                "next_heat_seed contains invalid base64 characters: '{}'",
                self.next_heat_seed
            ));
        }

        // Rule 2: heats object exists (implicitly satisfied by struct)

        // Validate each Heat
        for (heat_key, heat) in &self.heats {
            self.validate_heat(heat_key, heat, &mut errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_heat(&self, heat_key: &str, heat: &Heat, errors: &mut Vec<String>) {
        let heat_ctx = format!("Heat '{}'", heat_key);

        // Rule 3: Heat key must match ₣[A-Za-z0-9_-]{2}
        if !heat_key.starts_with('₣') {
            errors.push(format!("{}: key must start with '₣'", heat_ctx));
        } else {
            let suffix = &heat_key[3..]; // ₣ is 3 bytes in UTF-8
            if suffix.len() != 2 {
                errors.push(format!(
                    "{}: key must have 2 base64 chars after '₣', got {}",
                    heat_ctx,
                    suffix.len()
                ));
            } else if !is_base64(suffix) {
                errors.push(format!(
                    "{}: key contains invalid base64 characters",
                    heat_ctx
                ));
            }
        }

        // Rule 4: Required Heat fields
        // silks (non-empty kebab-case)
        if !is_kebab_case(&heat.silks) {
            errors.push(format!(
                "{}: silks must be non-empty kebab-case, got '{}'",
                heat_ctx, heat.silks
            ));
        }

        // creation_time (YYMMDD)
        if !is_yymmdd(&heat.creation_time) {
            errors.push(format!(
                "{}: creation_time must be YYMMDD format, got '{}'",
                heat_ctx, heat.creation_time
            ));
        }

        // status (validated by serde enum)

        // next_pace_seed (3 URL-safe base64 characters)
        if heat.next_pace_seed.len() != 3 {
            errors.push(format!(
                "{}: next_pace_seed must be 3 characters, got {}",
                heat_ctx,
                heat.next_pace_seed.len()
            ));
        } else if !is_base64(&heat.next_pace_seed) {
            errors.push(format!(
                "{}: next_pace_seed contains invalid base64 characters",
                heat_ctx
            ));
        }

        // Extract heat identity (base64 part without prefix) for pace validation
        let heat_identity = if heat_key.starts_with('₣') && heat_key.len() >= 5 {
            Some(&heat_key[3..]) // ₣ is 3 bytes
        } else {
            None
        };

        // Rule 5: order array and paces object must have identical key sets
        let order_set: HashSet<&String> = heat.order.iter().collect();
        let _paces_set: HashSet<&String> = heat.paces.keys().collect();

        if order_set.len() != heat.order.len() {
            errors.push(format!("{}: order array contains duplicate entries", heat_ctx));
        }

        let in_order_not_paces: Vec<_> = heat
            .order
            .iter()
            .filter(|k| !heat.paces.contains_key(*k))
            .collect();
        let in_paces_not_order: Vec<_> = heat
            .paces
            .keys()
            .filter(|k| !order_set.contains(k))
            .collect();

        if !in_order_not_paces.is_empty() {
            errors.push(format!(
                "{}: order contains keys not in paces: {:?}",
                heat_ctx, in_order_not_paces
            ));
        }
        if !in_paces_not_order.is_empty() {
            errors.push(format!(
                "{}: paces contains keys not in order: {:?}",
                heat_ctx, in_paces_not_order
            ));
        }

        // Validate each Pace
        for (pace_key, pace) in &heat.paces {
            self.validate_pace(&heat_ctx, heat_identity, pace_key, pace, errors);
        }
    }

    fn validate_pace(
        &self,
        heat_ctx: &str,
        heat_identity: Option<&str>,
        pace_key: &str,
        pace: &Pace,
        errors: &mut Vec<String>,
    ) {
        let pace_ctx = format!("{} Pace '{}'", heat_ctx, pace_key);

        // Rule 6: Pace key must match ₢[A-Za-z0-9_-]{5}
        if !pace_key.starts_with('₢') {
            errors.push(format!("{}: key must start with '₢'", pace_ctx));
        } else {
            let suffix = &pace_key[3..]; // ₢ is 3 bytes in UTF-8
            if suffix.len() != 5 {
                errors.push(format!(
                    "{}: key must have 5 base64 chars after '₢', got {}",
                    pace_ctx,
                    suffix.len()
                ));
            } else {
                if !is_base64(suffix) {
                    errors.push(format!(
                        "{}: key contains invalid base64 characters",
                        pace_ctx
                    ));
                }
                // Pace must embed parent Heat identity (first 2 chars)
                if let Some(heat_id) = heat_identity {
                    if !suffix.starts_with(heat_id) {
                        errors.push(format!(
                            "{}: key must embed parent heat identity '{}', got '{}'",
                            pace_ctx,
                            heat_id,
                            &suffix[..2.min(suffix.len())]
                        ));
                    }
                }
            }
        }

        // Rule 7: Pace must have silks (non-empty kebab-case)
        if !is_kebab_case(&pace.silks) {
            errors.push(format!(
                "{}: silks must be non-empty kebab-case, got '{}'",
                pace_ctx, pace.silks
            ));
        }

        // Rule 7: tacks must be non-empty array
        if pace.tacks.is_empty() {
            errors.push(format!("{}: tacks array must not be empty", pace_ctx));
        }

        // Validate each Tack
        for (i, tack) in pace.tacks.iter().enumerate() {
            self.validate_tack(&pace_ctx, i, tack, errors);
        }
    }

    fn validate_tack(&self, pace_ctx: &str, index: usize, tack: &Tack, errors: &mut Vec<String>) {
        let tack_ctx = format!("{} Tack[{}]", pace_ctx, index);

        // Rule 8: ts must be YYMMDD-HHMM
        if !is_yymmdd_hhmm(&tack.ts) {
            errors.push(format!(
                "{}: ts must be YYMMDD-HHMM format, got '{}'",
                tack_ctx, tack.ts
            ));
        }

        // Rule 8: text must be non-empty
        if tack.text.is_empty() {
            errors.push(format!("{}: text must not be empty", tack_ctx));
        }

        // Rule 9: direction presence depends on state
        match tack.state {
            PaceState::Primed => {
                match &tack.direction {
                    None => {
                        errors.push(format!(
                            "{}: direction is required when state is 'primed'",
                            tack_ctx
                        ));
                    }
                    Some(d) if d.is_empty() => {
                        errors.push(format!(
                            "{}: direction must not be empty when state is 'primed'",
                            tack_ctx
                        ));
                    }
                    _ => {}
                }
            }
            _ => {
                if tack.direction.is_some() {
                    errors.push(format!(
                        "{}: direction must be absent when state is not 'primed'",
                        tack_ctx
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pace_state_serialization() {
        let state = PaceState::Rough;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"rough\"");
    }

    // Helper to create a minimal valid Gallops structure
    fn make_valid_gallops() -> Gallops {
        Gallops {
            next_heat_seed: "AB".to_string(),
            heats: HashMap::new(),
        }
    }

    // Helper to create a valid Tack
    fn make_valid_tack(state: PaceState, direction: Option<String>) -> Tack {
        Tack {
            ts: "260101-1200".to_string(),
            state,
            text: "Test tack text".to_string(),
            direction,
        }
    }

    // Helper to create a valid Pace
    fn make_valid_pace(heat_id: &str, silks: &str) -> (String, Pace) {
        let pace_key = format!("₢{}AAA", heat_id);
        let pace = Pace {
            silks: silks.to_string(),
            tacks: vec![make_valid_tack(PaceState::Rough, None)],
        };
        (pace_key, pace)
    }

    // Helper to create a valid Heat
    fn make_valid_heat(heat_id: &str, silks: &str) -> (String, Heat) {
        let heat_key = format!("₣{}", heat_id);
        let (pace_key, pace) = make_valid_pace(heat_id, "test-pace");
        let mut paces = HashMap::new();
        paces.insert(pace_key.clone(), pace);

        let heat = Heat {
            silks: silks.to_string(),
            creation_time: "260101".to_string(),
            status: HeatStatus::Current,
            order: vec![pace_key],
            next_pace_seed: "AAB".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paces,
        };
        (heat_key, heat)
    }

    // ===== Validation helper tests =====

    #[test]
    fn test_is_base64_valid() {
        assert!(is_base64("AB"));
        assert!(is_base64("ABCDE"));
        assert!(is_base64("Az09-_"));
    }

    #[test]
    fn test_is_base64_invalid() {
        assert!(!is_base64("A!"));
        assert!(!is_base64("A B"));
        assert!(!is_base64("+/"));
    }

    #[test]
    fn test_is_kebab_case_valid() {
        assert!(is_kebab_case("test"));
        assert!(is_kebab_case("test-pace"));
        assert!(is_kebab_case("my-cool-heat123"));
        assert!(is_kebab_case("a1-b2-c3"));
    }

    #[test]
    fn test_is_kebab_case_invalid() {
        assert!(!is_kebab_case(""));
        assert!(!is_kebab_case("Test"));
        assert!(!is_kebab_case("test_pace"));
        assert!(!is_kebab_case("-test"));
        assert!(!is_kebab_case("test-"));
        assert!(!is_kebab_case("test--pace"));
    }

    #[test]
    fn test_is_yymmdd_valid() {
        assert!(is_yymmdd("260101"));
        assert!(is_yymmdd("991231"));
    }

    #[test]
    fn test_is_yymmdd_invalid() {
        assert!(!is_yymmdd("2601"));
        assert!(!is_yymmdd("26010101"));
        assert!(!is_yymmdd("26-01-01"));
        assert!(!is_yymmdd("26ab01"));
    }

    #[test]
    fn test_is_yymmdd_hhmm_valid() {
        assert!(is_yymmdd_hhmm("260101-1234"));
        assert!(is_yymmdd_hhmm("991231-2359"));
    }

    #[test]
    fn test_is_yymmdd_hhmm_invalid() {
        assert!(!is_yymmdd_hhmm("260101"));
        assert!(!is_yymmdd_hhmm("260101-123"));
        assert!(!is_yymmdd_hhmm("260101-12345"));
        assert!(!is_yymmdd_hhmm("26010112:34"));
    }

    // ===== Gallops validation tests =====

    #[test]
    fn test_validate_minimal_valid_gallops() {
        let gallops = make_valid_gallops();
        assert!(gallops.validate().is_ok());
    }

    #[test]
    fn test_validate_gallops_with_heat() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key, heat);
        assert!(gallops.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_next_heat_seed_length() {
        let mut gallops = make_valid_gallops();
        gallops.next_heat_seed = "ABC".to_string();
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("next_heat_seed must be 2 characters")));
    }

    #[test]
    fn test_validate_invalid_next_heat_seed_chars() {
        let mut gallops = make_valid_gallops();
        gallops.next_heat_seed = "A!".to_string();
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("invalid base64 characters")));
    }

    #[test]
    fn test_validate_heat_key_missing_prefix() {
        let mut gallops = make_valid_gallops();
        let (_, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert("AB".to_string(), heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("key must start with '₣'")));
    }

    #[test]
    fn test_validate_heat_invalid_silks() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        heat.silks = "Invalid_Silks".to_string();
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("silks must be non-empty kebab-case")));
    }

    #[test]
    fn test_validate_heat_invalid_creation_time() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        heat.creation_time = "2026-01-01".to_string();
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("creation_time must be YYMMDD format")));
    }

    #[test]
    fn test_validate_heat_invalid_next_pace_seed() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        heat.next_pace_seed = "AB".to_string(); // Should be 3 chars
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("next_pace_seed must be 3 characters")));
    }

    #[test]
    fn test_validate_order_paces_mismatch() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        // Add extra entry to order that doesn't exist in paces
        heat.order.push("₢ABXXX".to_string());
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("order contains keys not in paces")));
    }

    #[test]
    fn test_validate_pace_key_wrong_heat_identity() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        // Replace pace with one that embeds wrong heat identity
        heat.paces.clear();
        heat.order.clear();
        let bad_pace_key = "₢CDAAA".to_string(); // CD instead of AB
        let pace = Pace {
            silks: "bad-pace".to_string(),
            tacks: vec![make_valid_tack(PaceState::Rough, None)],
        };
        heat.paces.insert(bad_pace_key.clone(), pace);
        heat.order.push(bad_pace_key);
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("must embed parent heat identity")));
    }

    #[test]
    fn test_validate_pace_invalid_silks() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        // Get the first pace and modify its silks
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.silks = "".to_string();
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("silks must be non-empty kebab-case")));
    }

    #[test]
    fn test_validate_pace_empty_tacks() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks.clear();
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("tacks array must not be empty")));
    }

    #[test]
    fn test_validate_tack_invalid_ts() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].ts = "invalid".to_string();
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("ts must be YYMMDD-HHMM format")));
    }

    #[test]
    fn test_validate_tack_empty_text() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].text = "".to_string();
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("text must not be empty")));
    }

    #[test]
    fn test_validate_primed_without_direction() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].state = PaceState::Primed;
            pace.tacks[0].direction = None;
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("direction is required when state is 'primed'")));
    }

    #[test]
    fn test_validate_primed_with_empty_direction() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].state = PaceState::Primed;
            pace.tacks[0].direction = Some("".to_string());
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("direction must not be empty when state is 'primed'")));
    }

    #[test]
    fn test_validate_primed_with_direction_valid() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].state = PaceState::Primed;
            pace.tacks[0].direction = Some("Execute autonomously".to_string());
        }
        gallops.heats.insert(heat_key, heat);
        assert!(gallops.validate().is_ok());
    }

    #[test]
    fn test_validate_non_primed_with_direction() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].state = PaceState::Rough;
            pace.tacks[0].direction = Some("Should not be here".to_string());
        }
        gallops.heats.insert(heat_key, heat);
        let errors = gallops.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("direction must be absent when state is not 'primed'")));
    }

    #[test]
    fn test_validate_complete_state_valid() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].state = PaceState::Complete;
            pace.tacks[0].direction = None;
        }
        gallops.heats.insert(heat_key, heat);
        assert!(gallops.validate().is_ok());
    }

    #[test]
    fn test_validate_abandoned_state_valid() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");
        if let Some(pace) = heat.paces.values_mut().next() {
            pace.tacks[0].state = PaceState::Abandoned;
            pace.tacks[0].direction = None;
        }
        gallops.heats.insert(heat_key, heat);
        assert!(gallops.validate().is_ok());
    }

    // ===== Load/Save round-trip tests =====

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key, heat);

        let json = serde_json::to_string_pretty(&gallops).unwrap();
        let restored: Gallops = serde_json::from_str(&json).unwrap();

        assert_eq!(gallops.next_heat_seed, restored.next_heat_seed);
        assert_eq!(gallops.heats.len(), restored.heats.len());
    }

    #[test]
    fn test_multiple_errors_collected() {
        let mut gallops = Gallops {
            next_heat_seed: "!!!".to_string(), // Wrong length and chars
            heats: HashMap::new(),
        };
        let (_, mut heat) = make_valid_heat("AB", "my-heat");
        heat.silks = "InvalidSilks".to_string(); // Not kebab-case
        heat.creation_time = "invalid".to_string(); // Not YYMMDD
        gallops.heats.insert("₣AB".to_string(), heat);

        let errors = gallops.validate().unwrap_err();
        // Should have multiple errors
        assert!(errors.len() >= 3);
    }
}
