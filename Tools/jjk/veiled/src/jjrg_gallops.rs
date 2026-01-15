//! Gallops JSON operations
//!
//! Implements read/write operations on the Gallops JSON store.
//! All operations are atomic (write to temp, then rename).

use serde::{Deserialize, Serialize};
use crate::jjrc_core::timestamp_full;
use crate::jjrf_favor::{CHARSET, Firemark, Coronet, FIREMARK_PREFIX, CORONET_PREFIX};
use std::collections::HashSet;
use std::fs;
use std::io::{Read as IoRead, Write};
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

// ===== Seed Increment Helpers =====

/// Increment a base64 seed string (with carry)
/// Works for both 2-char (heat) and 3-char (pace) seeds
fn increment_seed(seed: &str) -> String {
    let mut chars: Vec<u8> = seed.bytes().collect();
    let mut carry = true;

    // Process from right to left
    for i in (0..chars.len()).rev() {
        if !carry {
            break;
        }

        // Find current position in charset
        let pos = CHARSET.iter().position(|&c| c == chars[i]).unwrap_or(0);

        if pos == 63 {
            // Wrap around
            chars[i] = CHARSET[0];
            // carry remains true
        } else {
            chars[i] = CHARSET[pos + 1];
            carry = false;
        }
    }

    String::from_utf8(chars).unwrap_or_else(|_| seed.to_string())
}

// ===== Write Operations =====

/// Arguments for the nominate operation
pub struct NominateArgs {
    pub silks: String,
    pub created: String,
}

/// Result of the nominate operation
#[derive(Debug)]
pub struct NominateResult {
    pub firemark: String,
}

/// Arguments for the slate operation
pub struct SlateArgs {
    pub firemark: String,
    pub silks: String,
    pub text: String,
    /// Coronet to insert before (mutually exclusive with after/first)
    pub before: Option<String>,
    /// Coronet to insert after (mutually exclusive with before/first)
    pub after: Option<String>,
    /// Insert at beginning (mutually exclusive with before/after)
    pub first: bool,
}

/// Result of the slate operation
#[derive(Debug)]
pub struct SlateResult {
    pub coronet: String,
}

/// Arguments for the rail operation
pub struct RailArgs {
    pub firemark: String,
    pub order: Vec<String>,
}

/// Arguments for the tally operation
pub struct TallyArgs {
    pub coronet: String,
    pub state: Option<PaceState>,
    pub direction: Option<String>,
    pub text: Option<String>,
}

impl Gallops {
    /// Nominate a new Heat
    ///
    /// Creates a new Heat with empty Pace structure and creates the paddock file.
    pub fn nominate(&mut self, args: NominateArgs, base_path: &Path) -> Result<NominateResult, String> {
        // Validate silks is kebab-case
        if !is_kebab_case(&args.silks) {
            return Err(format!("silks must be kebab-case, got '{}'", args.silks));
        }

        // Validate created is YYMMDD
        if !is_yymmdd(&args.created) {
            return Err(format!("created must be YYMMDD format, got '{}'", args.created));
        }

        // Allocate Firemark from next_heat_seed
        let firemark_str = format!("{}{}", FIREMARK_PREFIX, self.next_heat_seed);
        let heat_id = self.next_heat_seed.clone();

        // Compute paddock path
        let paddock_file = format!(".claude/jjm/jjp_{}.md", heat_id);

        // Create paddock file with template
        let paddock_path = base_path.join(&paddock_file);
        if let Some(parent) = paddock_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create paddock directory: {}", e))?;
        }

        let paddock_content = format!(
            "# Paddock: {}\n\n## Context\n\n(Describe the initiative's background and goals)\n\n## References\n\n(List relevant files, docs, or prior work)\n",
            args.silks
        );

        fs::write(&paddock_path, paddock_content)
            .map_err(|e| format!("Failed to write paddock file: {}", e))?;

        // Create new Heat
        let heat = Heat {
            silks: args.silks,
            creation_time: args.created,
            status: HeatStatus::Current,
            order: Vec::new(),
            next_pace_seed: "AAA".to_string(),
            paddock_file,
            paces: std::collections::HashMap::new(),
        };

        // Insert Heat
        self.heats.insert(firemark_str.clone(), heat);

        // Increment next_heat_seed
        self.next_heat_seed = increment_seed(&self.next_heat_seed);

        Ok(NominateResult { firemark: firemark_str })
    }

    /// Slate a new Pace
    ///
    /// Adds a new Pace to a Heat with an initial Tack in rough state.
    /// Positioning: use before/after/first to insert at specific location.
    pub fn slate(&mut self, args: SlateArgs) -> Result<SlateResult, String> {
        // Validate silks is kebab-case
        if !is_kebab_case(&args.silks) {
            return Err(format!("silks must be kebab-case, got '{}'", args.silks));
        }

        // Validate text is non-empty
        if args.text.is_empty() {
            return Err("text must not be empty".to_string());
        }

        // Validate positioning mutual exclusivity
        let position_count = [args.before.is_some(), args.after.is_some(), args.first]
            .iter()
            .filter(|&&x| x)
            .count();
        if position_count > 1 {
            return Err("Only one of --before, --after, or --first may be specified".to_string());
        }

        // Parse and normalize firemark
        let firemark = Firemark::parse(&args.firemark)
            .map_err(|e| format!("Invalid firemark: {}", e))?;
        let firemark_key = firemark.display();

        // Verify Heat exists
        let heat = self.heats.get_mut(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // If --before or --after specified, validate target coronet exists
        let insert_position = if let Some(ref before_str) = args.before {
            let target = Coronet::parse(before_str)
                .map_err(|e| format!("Invalid --before coronet: {}", e))?;
            let target_key = target.display();
            let pos = heat.order.iter().position(|c| c == &target_key)
                .ok_or_else(|| format!("Target coronet '{}' not found in heat", target_key))?;
            Some(pos) // Insert before this position
        } else if let Some(ref after_str) = args.after {
            let target = Coronet::parse(after_str)
                .map_err(|e| format!("Invalid --after coronet: {}", e))?;
            let target_key = target.display();
            let pos = heat.order.iter().position(|c| c == &target_key)
                .ok_or_else(|| format!("Target coronet '{}' not found in heat", target_key))?;
            Some(pos + 1) // Insert after this position
        } else if args.first {
            Some(0) // Insert at beginning
        } else {
            None // Append to end (default)
        };

        // Construct Coronet
        let coronet_str = format!("{}{}{}", CORONET_PREFIX, firemark.as_str(), heat.next_pace_seed);

        // Create initial Tack
        let tack = Tack {
            ts: timestamp_full(),
            state: PaceState::Rough,
            text: args.text,
            direction: None,
        };

        // Create new Pace
        let pace = Pace {
            silks: args.silks,
            tacks: vec![tack],
        };

        // Insert into order at determined position
        match insert_position {
            Some(pos) => heat.order.insert(pos, coronet_str.clone()),
            None => heat.order.push(coronet_str.clone()),
        }
        heat.paces.insert(coronet_str.clone(), pace);

        // Increment next_pace_seed
        heat.next_pace_seed = increment_seed(&heat.next_pace_seed);

        Ok(SlateResult { coronet: coronet_str })
    }

    /// Rail - reorder Paces within a Heat
    ///
    /// Replaces the order array with a new sequence. Validates that the new order
    /// contains exactly the same Coronets as the current order.
    pub fn rail(&mut self, args: RailArgs) -> Result<(), String> {
        // Parse and normalize firemark
        let firemark = Firemark::parse(&args.firemark)
            .map_err(|e| format!("Invalid firemark: {}", e))?;
        let firemark_key = firemark.display();

        // Verify Heat exists
        let heat = self.heats.get_mut(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // Normalize the input order (add prefix if missing)
        let normalized_order: Result<Vec<String>, String> = args.order.iter()
            .map(|c| {
                let coronet = Coronet::parse(c)
                    .map_err(|e| format!("Invalid coronet '{}': {}", c, e))?;
                Ok(coronet.display())
            })
            .collect();
        let new_order = normalized_order?;

        // Validate count matches
        if new_order.len() != heat.order.len() {
            return Err(format!(
                "Order count mismatch: got {}, expected {}",
                new_order.len(),
                heat.order.len()
            ));
        }

        // Validate no duplicates
        let new_set: HashSet<&String> = new_order.iter().collect();
        if new_set.len() != new_order.len() {
            return Err("Order contains duplicate Coronets".to_string());
        }

        // Validate all Coronets exist in paces
        for coronet in &new_order {
            if !heat.paces.contains_key(coronet) {
                return Err(format!("Coronet '{}' not found in Heat's paces", coronet));
            }
        }

        // Validate all Coronets embed correct parent Firemark
        for coronet in &new_order {
            let c = Coronet::parse(coronet).unwrap();
            if c.parent_firemark().display() != firemark_key {
                return Err(format!(
                    "Coronet '{}' does not embed parent Heat '{}'",
                    coronet, firemark_key
                ));
            }
        }

        // Replace order
        heat.order = new_order;

        Ok(())
    }

    /// Tally - add a new Tack to a Pace
    ///
    /// Prepends a new Tack with state transition and/or plan refinement.
    pub fn tally(&mut self, args: TallyArgs) -> Result<(), String> {
        // Parse and normalize coronet
        let coronet = Coronet::parse(&args.coronet)
            .map_err(|e| format!("Invalid coronet: {}", e))?;
        let coronet_key = coronet.display();

        // Extract parent Firemark
        let firemark = coronet.parent_firemark();
        let firemark_key = firemark.display();

        // Verify Heat exists
        let heat = self.heats.get_mut(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // Verify Pace exists
        let pace = heat.paces.get_mut(&coronet_key)
            .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;

        // Read current Tack
        let current_tack = pace.tacks.first()
            .ok_or_else(|| "Pace has no tacks (should never happen)".to_string())?;

        // Determine new state
        let new_state = args.state.clone().unwrap_or_else(|| current_tack.state.clone());

        // Determine new direction
        let new_direction = match (&args.state, &new_state) {
            // State explicitly set to primed: direction required
            (Some(PaceState::Primed), _) => {
                match &args.direction {
                    Some(d) if !d.is_empty() => Some(d.clone()),
                    Some(_) => return Err("direction must not be empty when state is primed".to_string()),
                    None => return Err("direction is required when state is primed".to_string()),
                }
            }
            // State explicitly set to something other than primed: direction forbidden
            (Some(_), _) => {
                if args.direction.is_some() {
                    return Err("direction must be absent when state is not primed".to_string());
                }
                None
            }
            // State inherited and was primed: inherit direction
            (None, PaceState::Primed) => {
                args.direction.or_else(|| current_tack.direction.clone())
            }
            // State inherited and was not primed: no direction
            (None, _) => None,
        };

        // Determine new text
        let new_text = args.text.unwrap_or_else(|| current_tack.text.clone());
        if new_text.is_empty() {
            return Err("text must not be empty".to_string());
        }

        // Create new Tack
        let new_tack = Tack {
            ts: timestamp_full(),
            state: new_state,
            text: new_text,
            direction: new_direction,
        };

        // Prepend to tacks array
        pace.tacks.insert(0, new_tack);

        Ok(())
    }
}

/// Read text from stdin (for CLI commands)
pub fn read_stdin() -> Result<String, String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)
        .map_err(|e| format!("Failed to read from stdin: {}", e))?;
    Ok(buffer.trim_end().to_string())
}

/// Read text from stdin, returning None if stdin is empty or a tty
pub fn read_stdin_optional() -> Result<Option<String>, String> {
    use std::io::IsTerminal;

    // If stdin is a terminal, no input was piped
    if std::io::stdin().is_terminal() {
        return Ok(None);
    }

    let text = read_stdin()?;
    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
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

    // ===== Seed increment tests =====

    #[test]
    fn test_increment_seed_simple() {
        assert_eq!(increment_seed("AA"), "AB");
        assert_eq!(increment_seed("AB"), "AC");
        assert_eq!(increment_seed("Az"), "A0");
    }

    #[test]
    fn test_increment_seed_carry() {
        // '_' is position 63, should wrap to 'A' (position 0) and carry
        assert_eq!(increment_seed("A_"), "BA");
        assert_eq!(increment_seed("__"), "AA"); // Full wrap around
    }

    #[test]
    fn test_increment_seed_three_chars() {
        assert_eq!(increment_seed("AAA"), "AAB");
        assert_eq!(increment_seed("AA_"), "ABA");
        assert_eq!(increment_seed("A__"), "BAA");
        assert_eq!(increment_seed("___"), "AAA");
    }

    // ===== Write operation tests =====

    #[test]
    fn test_nominate_creates_heat() {
        let mut gallops = make_valid_gallops();
        let temp_dir = std::env::temp_dir().join("jjk_test_nominate");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let args = NominateArgs {
            silks: "test-heat".to_string(),
            created: "260113".to_string(),
        };

        let result = gallops.nominate(args, &temp_dir).unwrap();

        // Check result
        assert!(result.firemark.starts_with('₣'));

        // Check heat was created
        assert!(gallops.heats.contains_key(&result.firemark));
        let heat = gallops.heats.get(&result.firemark).unwrap();
        assert_eq!(heat.silks, "test-heat");
        assert_eq!(heat.creation_time, "260113");
        assert_eq!(heat.status, HeatStatus::Current);
        assert!(heat.order.is_empty());
        assert_eq!(heat.next_pace_seed, "AAA");

        // Check seed was incremented
        assert_eq!(gallops.next_heat_seed, "AC");

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_nominate_invalid_silks() {
        let mut gallops = make_valid_gallops();
        let temp_dir = std::env::temp_dir();

        let args = NominateArgs {
            silks: "InvalidSilks".to_string(),
            created: "260113".to_string(),
        };

        let result = gallops.nominate(args, &temp_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("silks must be kebab-case"));
    }

    #[test]
    fn test_nominate_invalid_created() {
        let mut gallops = make_valid_gallops();
        let temp_dir = std::env::temp_dir();

        let args = NominateArgs {
            silks: "test-heat".to_string(),
            created: "2026-01-13".to_string(),
        };

        let result = gallops.nominate(args, &temp_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("created must be YYMMDD format"));
    }

    #[test]
    fn test_slate_creates_pace() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key.clone(), heat);

        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "test-pace".to_string(),
            text: "Do something useful".to_string(),
            before: None,
            after: None,
            first: false,
        };

        let result = gallops.slate(args).unwrap();

        // Check result
        assert!(result.coronet.starts_with('₢'));
        assert!(result.coronet.contains("AB")); // Embeds heat identity

        // Check pace was created
        let heat = gallops.heats.get(&heat_key).unwrap();
        assert!(heat.paces.contains_key(&result.coronet));
        let pace = heat.paces.get(&result.coronet).unwrap();
        assert_eq!(pace.silks, "test-pace");
        assert_eq!(pace.tacks.len(), 1);
        assert_eq!(pace.tacks[0].state, PaceState::Rough);
        assert_eq!(pace.tacks[0].text, "Do something useful");

        // Check order was updated
        assert!(heat.order.contains(&result.coronet));

        // Check seed was incremented (was AAB, now AAC due to existing pace)
        assert_eq!(heat.next_pace_seed, "AAC");
    }

    #[test]
    fn test_slate_heat_not_found() {
        let mut gallops = make_valid_gallops();

        let args = SlateArgs {
            firemark: "CD".to_string(),
            silks: "test-pace".to_string(),
            text: "Do something".to_string(),
            before: None,
            after: None,
            first: false,
        };

        let result = gallops.slate(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_slate_invalid_silks() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key, heat);

        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "InvalidSilks".to_string(),
            text: "Do something".to_string(),
            before: None,
            after: None,
            first: false,
        };

        let result = gallops.slate(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("silks must be kebab-case"));
    }

    #[test]
    fn test_slate_empty_text() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key, heat);

        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "test-pace".to_string(),
            text: "".to_string(),
            before: None,
            after: None,
            first: false,
        };

        let result = gallops.slate(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("text must not be empty"));
    }

    #[test]
    fn test_slate_with_first_inserts_at_beginning() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let existing_pace = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "new-first-pace".to_string(),
            text: "This should be first".to_string(),
            before: None,
            after: None,
            first: true,
        };

        let result = gallops.slate(args).unwrap();

        let heat = gallops.heats.get(&heat_key).unwrap();
        assert_eq!(heat.order[0], result.coronet); // New pace is first
        assert_eq!(heat.order[1], existing_pace); // Existing pace moved to second
    }

    #[test]
    fn test_slate_with_before_inserts_at_position() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

        // Add a second pace
        let pace2_key = "₢ABAAB".to_string();
        let pace2 = Pace {
            silks: "second-pace".to_string(),
            tacks: vec![make_valid_tack(PaceState::Rough, None)],
        };
        heat.paces.insert(pace2_key.clone(), pace2);
        heat.order.push(pace2_key.clone());
        heat.next_pace_seed = "AAC".to_string();

        let first_pace = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        // Insert before the second pace
        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "inserted-pace".to_string(),
            text: "Insert before second".to_string(),
            before: Some(pace2_key.clone()),
            after: None,
            first: false,
        };

        let result = gallops.slate(args).unwrap();

        let heat = gallops.heats.get(&heat_key).unwrap();
        assert_eq!(heat.order[0], first_pace); // Original first unchanged
        assert_eq!(heat.order[1], result.coronet); // New pace inserted
        assert_eq!(heat.order[2], pace2_key); // Second pace moved
    }

    #[test]
    fn test_slate_with_after_inserts_at_position() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

        // Add a second pace
        let pace2_key = "₢ABAAB".to_string();
        let pace2 = Pace {
            silks: "second-pace".to_string(),
            tacks: vec![make_valid_tack(PaceState::Rough, None)],
        };
        heat.paces.insert(pace2_key.clone(), pace2);
        heat.order.push(pace2_key.clone());
        heat.next_pace_seed = "AAC".to_string();

        let first_pace = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        // Insert after the first pace
        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "inserted-pace".to_string(),
            text: "Insert after first".to_string(),
            before: None,
            after: Some(first_pace.clone()),
            first: false,
        };

        let result = gallops.slate(args).unwrap();

        let heat = gallops.heats.get(&heat_key).unwrap();
        assert_eq!(heat.order[0], first_pace); // Original first unchanged
        assert_eq!(heat.order[1], result.coronet); // New pace inserted after first
        assert_eq!(heat.order[2], pace2_key); // Second pace at end
    }

    #[test]
    fn test_slate_mutual_exclusivity() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let existing_pace = heat.order[0].clone();
        gallops.heats.insert(heat_key, heat);

        // Try with both before and first
        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "bad-pace".to_string(),
            text: "Should fail".to_string(),
            before: Some(existing_pace),
            after: None,
            first: true,
        };

        let result = gallops.slate(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Only one of"));
    }

    #[test]
    fn test_slate_before_invalid_coronet() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key, heat);

        let args = SlateArgs {
            firemark: "AB".to_string(),
            silks: "new-pace".to_string(),
            text: "Test".to_string(),
            before: Some("₢ABXXX".to_string()), // Non-existent
            after: None,
            first: false,
        };

        let result = gallops.slate(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found in heat"));
    }

    #[test]
    fn test_rail_reorders_paces() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

        // Add another pace
        let pace2_key = "₢ABAAB".to_string();
        let pace2 = Pace {
            silks: "second-pace".to_string(),
            tacks: vec![make_valid_tack(PaceState::Rough, None)],
        };
        heat.paces.insert(pace2_key.clone(), pace2);
        heat.order.push(pace2_key.clone());
        heat.next_pace_seed = "AAC".to_string();

        let original_first = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        // Reorder: swap the two paces
        let args = RailArgs {
            firemark: "AB".to_string(),
            order: vec![pace2_key.clone(), original_first.clone()],
        };

        let result = gallops.rail(args);
        assert!(result.is_ok());

        let heat = gallops.heats.get(&heat_key).unwrap();
        assert_eq!(heat.order[0], pace2_key);
        assert_eq!(heat.order[1], original_first);
    }

    #[test]
    fn test_rail_count_mismatch() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        gallops.heats.insert(heat_key, heat);

        // Try to reorder with wrong count
        let args = RailArgs {
            firemark: "AB".to_string(),
            order: vec!["₢ABAAA".to_string(), "₢ABAAB".to_string()], // 2 items but only 1 pace
        };

        let result = gallops.rail(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("count mismatch"));
    }

    #[test]
    fn test_rail_duplicate_coronets() {
        let mut gallops = make_valid_gallops();
        let (heat_key, mut heat) = make_valid_heat("AB", "my-heat");

        // Add another pace
        let pace2_key = "₢ABAAB".to_string();
        let pace2 = Pace {
            silks: "second-pace".to_string(),
            tacks: vec![make_valid_tack(PaceState::Rough, None)],
        };
        heat.paces.insert(pace2_key.clone(), pace2);
        heat.order.push(pace2_key.clone());
        gallops.heats.insert(heat_key, heat);

        // Try with duplicate
        let args = RailArgs {
            firemark: "AB".to_string(),
            order: vec!["₢ABAAA".to_string(), "₢ABAAA".to_string()],
        };

        let result = gallops.rail(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("duplicate"));
    }

    #[test]
    fn test_tally_state_transition() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let pace_key = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        // Transition to complete
        let args = TallyArgs {
            coronet: pace_key.clone(),
            state: Some(PaceState::Complete),
            direction: None,
            text: Some("Work completed successfully".to_string()),
        };

        let result = gallops.tally(args);
        assert!(result.is_ok());

        let heat = gallops.heats.get(&heat_key).unwrap();
        let pace = heat.paces.get(&pace_key).unwrap();
        assert_eq!(pace.tacks.len(), 2); // Original + new
        assert_eq!(pace.tacks[0].state, PaceState::Complete);
        assert_eq!(pace.tacks[0].text, "Work completed successfully");
    }

    #[test]
    fn test_tally_primed_requires_direction() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let pace_key = heat.order[0].clone();
        gallops.heats.insert(heat_key, heat);

        let args = TallyArgs {
            coronet: pace_key,
            state: Some(PaceState::Primed),
            direction: None, // Missing!
            text: Some("Ready for execution".to_string()),
        };

        let result = gallops.tally(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("direction is required"));
    }

    #[test]
    fn test_tally_primed_with_direction() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let pace_key = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        let args = TallyArgs {
            coronet: pace_key.clone(),
            state: Some(PaceState::Primed),
            direction: Some("Execute autonomously".to_string()),
            text: Some("Ready for execution".to_string()),
        };

        let result = gallops.tally(args);
        assert!(result.is_ok());

        let heat = gallops.heats.get(&heat_key).unwrap();
        let pace = heat.paces.get(&pace_key).unwrap();
        assert_eq!(pace.tacks[0].state, PaceState::Primed);
        assert_eq!(pace.tacks[0].direction.as_ref().unwrap(), "Execute autonomously");
    }

    #[test]
    fn test_tally_inherit_state() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let pace_key = heat.order[0].clone();
        gallops.heats.insert(heat_key.clone(), heat);

        // First tack is rough, add new tack without specifying state
        let args = TallyArgs {
            coronet: pace_key.clone(),
            state: None, // Inherit
            direction: None,
            text: Some("Updated plan text".to_string()),
        };

        let result = gallops.tally(args);
        assert!(result.is_ok());

        let heat = gallops.heats.get(&heat_key).unwrap();
        let pace = heat.paces.get(&pace_key).unwrap();
        assert_eq!(pace.tacks[0].state, PaceState::Rough); // Inherited
        assert_eq!(pace.tacks[0].text, "Updated plan text");
    }

    #[test]
    fn test_tally_inherit_text() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let pace_key = heat.order[0].clone();
        let original_text = heat.paces.get(&pace_key).unwrap().tacks[0].text.clone();
        gallops.heats.insert(heat_key.clone(), heat);

        // Just change state, inherit text
        let args = TallyArgs {
            coronet: pace_key.clone(),
            state: Some(PaceState::Complete),
            direction: None,
            text: None, // Inherit
        };

        let result = gallops.tally(args);
        assert!(result.is_ok());

        let heat = gallops.heats.get(&heat_key).unwrap();
        let pace = heat.paces.get(&pace_key).unwrap();
        assert_eq!(pace.tacks[0].state, PaceState::Complete);
        assert_eq!(pace.tacks[0].text, original_text); // Inherited
    }

    #[test]
    fn test_tally_non_primed_forbids_direction() {
        let mut gallops = make_valid_gallops();
        let (heat_key, heat) = make_valid_heat("AB", "my-heat");
        let pace_key = heat.order[0].clone();
        gallops.heats.insert(heat_key, heat);

        let args = TallyArgs {
            coronet: pace_key,
            state: Some(PaceState::Complete),
            direction: Some("Should not be here".to_string()), // Not allowed!
            text: Some("Done".to_string()),
        };

        let result = gallops.tally(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("direction must be absent"));
    }
}
