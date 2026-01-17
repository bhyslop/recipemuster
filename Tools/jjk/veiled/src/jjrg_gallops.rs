// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops JSON operations
//!
//! Implements read/write operations on the Gallops JSON store.
//! All operations are atomic (write to temp, then rename).

use serde::{Deserialize, Serialize};
use crate::jjrc_core::jjrc_timestamp_full as timestamp_full;
use crate::jjrf_favor::{JJRF_CHARSET, jjrf_Firemark as Firemark, jjrf_Coronet as Coronet, JJRF_FIREMARK_PREFIX as FIREMARK_PREFIX, JJRF_CORONET_PREFIX as CORONET_PREFIX};
use crate::jjrs_steeplechase::jjrs_SteeplechaseEntry as SteeplechaseEntry;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{Read as IoRead, Write};
use std::path::Path;

/// Pace state values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum jjrg_PaceState {
    Rough,
    #[serde(alias = "primed")]
    Bridled,
    Complete,
    Abandoned,
}

/// Heat status values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum jjrg_HeatStatus {
    Current,
    Retired,
}

/// Tack record - snapshot of Pace state and plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Tack {
    pub ts: String,
    pub state: jjrg_PaceState,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// Pace record - discrete action within a Heat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Pace {
    pub silks: String,
    pub tacks: Vec<jjrg_Tack>,
}

/// Heat record - bounded initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Heat {
    pub silks: String,
    pub creation_time: String,
    pub status: jjrg_HeatStatus,
    pub order: Vec<String>,
    pub next_pace_seed: String,
    pub paddock_file: String,
    pub paces: BTreeMap<String, jjrg_Pace>,
}

/// Root Gallops structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct jjrg_Gallops {
    pub next_heat_seed: String,
    pub heats: BTreeMap<String, jjrg_Heat>,
}

// Validation helper functions

/// Check if string contains only URL-safe base64 characters
pub(crate) fn zjjrg_is_base64(s: &str) -> bool {
    s.bytes().all(|b| JJRF_CHARSET.contains(&b))
}

/// Check if string is valid kebab-case (non-empty, lowercase alphanumeric with hyphens)
pub(crate) fn zjjrg_is_kebab_case(s: &str) -> bool {
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
pub(crate) fn zjjrg_is_yymmdd(s: &str) -> bool {
    if s.len() != 6 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_digit())
}

/// Check if string matches YYMMDD-HHMM format
pub(crate) fn zjjrg_is_yymmdd_hhmm(s: &str) -> bool {
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

impl jjrg_Gallops {
    /// Load Gallops from a file path
    pub fn jjrg_load(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

        let gallops: jjrg_Gallops = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(gallops)
    }

    /// Save Gallops to a file path (atomic write)
    pub fn jjrg_save(&self, path: &Path) -> Result<(), String> {
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
    pub fn jjrg_validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Rule 1: next_heat_seed must be 2 URL-safe base64 characters
        if self.next_heat_seed.len() != 2 {
            errors.push(format!(
                "next_heat_seed must be 2 characters, got {}",
                self.next_heat_seed.len()
            ));
        } else if !zjjrg_is_base64(&self.next_heat_seed) {
            errors.push(format!(
                "next_heat_seed contains invalid base64 characters: '{}'",
                self.next_heat_seed
            ));
        }

        // Rule 2: heats object exists (implicitly satisfied by struct)

        // Validate each Heat
        for (heat_key, heat) in &self.heats {
            self.zjjrg_validate_heat(heat_key, heat, &mut errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn zjjrg_validate_heat(&self, heat_key: &str, heat: &jjrg_Heat, errors: &mut Vec<String>) {
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
            } else if !zjjrg_is_base64(suffix) {
                errors.push(format!(
                    "{}: key contains invalid base64 characters",
                    heat_ctx
                ));
            }
        }

        // Rule 4: Required Heat fields
        // silks (non-empty kebab-case)
        if !zjjrg_is_kebab_case(&heat.silks) {
            errors.push(format!(
                "{}: silks must be non-empty kebab-case, got '{}'",
                heat_ctx, heat.silks
            ));
        }

        // creation_time (YYMMDD)
        if !zjjrg_is_yymmdd(&heat.creation_time) {
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
        } else if !zjjrg_is_base64(&heat.next_pace_seed) {
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
            self.zjjrg_validate_pace(&heat_ctx, heat_identity, pace_key, pace, errors);
        }
    }

    fn zjjrg_validate_pace(
        &self,
        heat_ctx: &str,
        heat_identity: Option<&str>,
        pace_key: &str,
        pace: &jjrg_Pace,
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
                if !zjjrg_is_base64(suffix) {
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
        if !zjjrg_is_kebab_case(&pace.silks) {
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
            self.zjjrg_validate_tack(&pace_ctx, i, tack, errors);
        }
    }

    fn zjjrg_validate_tack(&self, pace_ctx: &str, index: usize, tack: &jjrg_Tack, errors: &mut Vec<String>) {
        let tack_ctx = format!("{} Tack[{}]", pace_ctx, index);

        // Rule 8: ts must be YYMMDD-HHMM
        if !zjjrg_is_yymmdd_hhmm(&tack.ts) {
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
            jjrg_PaceState::Bridled => {
                match &tack.direction {
                    None => {
                        errors.push(format!(
                            "{}: direction is required when state is 'bridled'",
                            tack_ctx
                        ));
                    }
                    Some(d) if d.is_empty() => {
                        errors.push(format!(
                            "{}: direction must not be empty when state is 'bridled'",
                            tack_ctx
                        ));
                    }
                    _ => {}
                }
            }
            _ => {
                if tack.direction.is_some() {
                    errors.push(format!(
                        "{}: direction must be absent when state is not 'bridled'",
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
pub(crate) fn zjjrg_increment_seed(seed: &str) -> String {
    let mut chars: Vec<u8> = seed.bytes().collect();
    let mut carry = true;

    // Process from right to left
    for i in (0..chars.len()).rev() {
        if !carry {
            break;
        }

        // Find current position in charset
        let pos = JJRF_CHARSET.iter().position(|&c| c == chars[i]).unwrap_or(0);

        if pos == 63 {
            // Wrap around
            chars[i] = JJRF_CHARSET[0];
            // carry remains true
        } else {
            chars[i] = JJRF_CHARSET[pos + 1];
            carry = false;
        }
    }

    String::from_utf8(chars).unwrap_or_else(|_| seed.to_string())
}

// ===== Write Operations =====

/// Arguments for the nominate operation
pub struct jjrg_NominateArgs {
    pub silks: String,
    pub created: String,
}

/// Result of the nominate operation
#[derive(Debug)]
pub struct jjrg_NominateResult {
    pub firemark: String,
}

/// Arguments for the slate operation
pub struct jjrg_SlateArgs {
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
pub struct jjrg_SlateResult {
    pub coronet: String,
}

/// Arguments for the rail operation
///
/// Supports two modes:
/// - Order mode: provide `order` array to replace entire sequence
/// - Move mode: provide `move_coronet` + one positioning field to relocate a single pace
pub struct jjrg_RailArgs {
    pub firemark: String,
    /// Order mode: new sequence of all coronets
    pub order: Vec<String>,
    /// Move mode: coronet to relocate
    pub move_coronet: Option<String>,
    /// Move before this coronet
    pub before: Option<String>,
    /// Move after this coronet
    pub after: Option<String>,
    /// Move to beginning
    pub first: bool,
    /// Move to end
    pub last: bool,
}

/// Arguments for the tally operation
pub struct jjrg_TallyArgs {
    pub coronet: String,
    pub state: Option<jjrg_PaceState>,
    pub direction: Option<String>,
    pub text: Option<String>,
}

/// Arguments for the draft operation
pub struct jjrg_DraftArgs {
    /// Coronet of the pace to move
    pub coronet: String,
    /// Destination heat Firemark
    pub to: String,
    /// Coronet to insert before (mutually exclusive with after/first)
    pub before: Option<String>,
    /// Coronet to insert after (mutually exclusive with before/first)
    pub after: Option<String>,
    /// Insert at beginning (mutually exclusive with before/after)
    pub first: bool,
}

/// Result of the draft operation
#[derive(Debug)]
pub struct jjrg_DraftResult {
    /// New coronet in destination heat
    pub new_coronet: String,
}

/// Arguments for the retire operation
pub struct jjrg_RetireArgs {
    /// Firemark of heat to retire
    pub firemark: String,
    /// Today's date in YYMMDD format (for trophy filename)
    pub today: String,
}

/// Result of the retire operation
#[derive(Debug)]
pub struct jjrg_RetireResult {
    /// Path to created trophy file
    pub trophy_path: String,
    /// Path to deleted paddock file
    pub paddock_path: String,
    /// Heat silks (for commit message)
    pub silks: String,
    /// Firemark display string (for commit message)
    pub firemark: String,
}

impl jjrg_Gallops {
    /// Nominate a new Heat
    ///
    /// Creates a new Heat with empty Pace structure and creates the paddock file.
    pub fn jjrg_nominate(&mut self, args: jjrg_NominateArgs, base_path: &Path) -> Result<jjrg_NominateResult, String> {
        // Validate silks is kebab-case
        if !zjjrg_is_kebab_case(&args.silks) {
            return Err(format!("silks must be kebab-case, got '{}'", args.silks));
        }

        // Validate created is YYMMDD
        if !zjjrg_is_yymmdd(&args.created) {
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
        let heat = jjrg_Heat {
            silks: args.silks,
            creation_time: args.created,
            status: jjrg_HeatStatus::Current,
            order: Vec::new(),
            next_pace_seed: "AAA".to_string(),
            paddock_file,
            paces: BTreeMap::new(),
        };

        // Insert Heat
        self.heats.insert(firemark_str.clone(), heat);

        // Increment next_heat_seed
        self.next_heat_seed = zjjrg_increment_seed(&self.next_heat_seed);

        Ok(jjrg_NominateResult { firemark: firemark_str })
    }

    /// Slate a new Pace
    ///
    /// Adds a new Pace to a Heat with an initial Tack in rough state.
    /// Positioning: use before/after/first to insert at specific location.
    pub fn jjrg_slate(&mut self, args: jjrg_SlateArgs) -> Result<jjrg_SlateResult, String> {
        // Validate silks is kebab-case
        if !zjjrg_is_kebab_case(&args.silks) {
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
        let firemark = Firemark::jjrf_parse(&args.firemark)
            .map_err(|e| format!("Invalid firemark: {}", e))?;
        let firemark_key = firemark.jjrf_display();

        // Verify Heat exists
        let heat = self.heats.get_mut(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // If --before or --after specified, validate target coronet exists
        let insert_position = if let Some(ref before_str) = args.before {
            let target = Coronet::jjrf_parse(before_str)
                .map_err(|e| format!("Invalid --before coronet: {}", e))?;
            let target_key = target.jjrf_display();
            let pos = heat.order.iter().position(|c| c == &target_key)
                .ok_or_else(|| format!("Target coronet '{}' not found in heat", target_key))?;
            Some(pos) // Insert before this position
        } else if let Some(ref after_str) = args.after {
            let target = Coronet::jjrf_parse(after_str)
                .map_err(|e| format!("Invalid --after coronet: {}", e))?;
            let target_key = target.jjrf_display();
            let pos = heat.order.iter().position(|c| c == &target_key)
                .ok_or_else(|| format!("Target coronet '{}' not found in heat", target_key))?;
            Some(pos + 1) // Insert after this position
        } else if args.first {
            Some(0) // Insert at beginning
        } else {
            None // Append to end (default)
        };

        // Construct Coronet
        let coronet_str = format!("{}{}{}", CORONET_PREFIX, firemark.jjrf_as_str(), heat.next_pace_seed);

        // Create initial Tack
        let tack = jjrg_Tack {
            ts: timestamp_full(),
            state: jjrg_PaceState::Rough,
            text: args.text,
            direction: None,
        };

        // Create new Pace
        let pace = jjrg_Pace {
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
        heat.next_pace_seed = zjjrg_increment_seed(&heat.next_pace_seed);

        Ok(jjrg_SlateResult { coronet: coronet_str })
    }

    /// Rail - reorder Paces within a Heat
    ///
    /// Supports two modes:
    /// - Order mode: replace entire sequence with provided order array
    /// - Move mode: relocate a single pace using positioning flags
    pub fn jjrg_rail(&mut self, args: jjrg_RailArgs) -> Result<Vec<String>, String> {
        // Parse and normalize firemark
        let firemark = Firemark::jjrf_parse(&args.firemark)
            .map_err(|e| format!("Invalid firemark: {}", e))?;
        let firemark_key = firemark.jjrf_display();

        // Verify Heat exists
        let heat = self.heats.get_mut(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // Mode detection: if move_coronet present, use move mode
        if let Some(ref move_str) = args.move_coronet {
            // Move mode validation
            if !args.order.is_empty() {
                return Err("Cannot combine --move with positional coronets".to_string());
            }

            // Parse and normalize move coronet
            let move_coronet = Coronet::jjrf_parse(move_str)
                .map_err(|e| format!("Invalid --move coronet: {}", e))?;
            let move_key = move_coronet.jjrf_display();

            // Validate move coronet exists in heat
            if !heat.paces.contains_key(&move_key) {
                return Err(format!("Pace {} not found in heat {}", move_key, firemark_key));
            }

            // Count positioning flags
            let position_count = [
                args.before.is_some(),
                args.after.is_some(),
                args.first,
                args.last,
            ].iter().filter(|&&x| x).count();

            if position_count == 0 {
                return Err("Move mode requires exactly one positioning flag".to_string());
            }
            if position_count > 1 {
                let mut flags = Vec::new();
                if args.before.is_some() { flags.push("--before"); }
                if args.after.is_some() { flags.push("--after"); }
                if args.first { flags.push("--first"); }
                if args.last { flags.push("--last"); }
                return Err(format!("Conflicting positioning flags: {}", flags.join(", ")));
            }

            // Determine target position and validate
            let current_pos = heat.order.iter().position(|c| c == &move_key)
                .ok_or_else(|| format!("Pace {} not in order array", move_key))?;

            let new_pos = if args.first {
                0
            } else if args.last {
                heat.order.len() - 1
            } else if let Some(ref before_str) = args.before {
                let target = Coronet::jjrf_parse(before_str)
                    .map_err(|e| format!("Invalid --before coronet: {}", e))?;
                let target_key = target.jjrf_display();

                if target_key == move_key {
                    return Err("Cannot position pace relative to itself".to_string());
                }

                let target_pos = heat.order.iter().position(|c| c == &target_key)
                    .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, firemark_key))?;

                // If moving from before target, the target shifts down after removal
                if current_pos < target_pos {
                    target_pos - 1
                } else {
                    target_pos
                }
            } else if let Some(ref after_str) = args.after {
                let target = Coronet::jjrf_parse(after_str)
                    .map_err(|e| format!("Invalid --after coronet: {}", e))?;
                let target_key = target.jjrf_display();

                if target_key == move_key {
                    return Err("Cannot position pace relative to itself".to_string());
                }

                let target_pos = heat.order.iter().position(|c| c == &target_key)
                    .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, firemark_key))?;

                // If moving from before target, the target shifts down after removal
                if current_pos < target_pos {
                    target_pos // After removal, target is at target_pos-1, we want target_pos-1+1 = target_pos
                } else {
                    target_pos + 1
                }
            } else {
                unreachable!()
            };

            // Remove from current position and insert at new position
            heat.order.remove(current_pos);
            heat.order.insert(new_pos, move_key);

        } else {
            // Order mode: replace entire sequence

            // Normalize the input order (add prefix if missing)
            let normalized_order: Result<Vec<String>, String> = args.order.iter()
                .map(|c| {
                    let coronet = Coronet::jjrf_parse(c)
                        .map_err(|e| format!("Invalid coronet '{}': {}", c, e))?;
                    Ok(coronet.jjrf_display())
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
                let c = Coronet::jjrf_parse(coronet).unwrap();
                if c.jjrf_parent_firemark().jjrf_display() != firemark_key {
                    return Err(format!(
                        "Coronet '{}' does not embed parent Heat '{}'",
                        coronet, firemark_key
                    ));
                }
            }

            // Replace order
            heat.order = new_order;
        }

        // Return the new order
        Ok(heat.order.clone())
    }

    /// Tally - add a new Tack to a Pace
    ///
    /// Prepends a new Tack with state transition and/or plan refinement.
    pub fn jjrg_tally(&mut self, args: jjrg_TallyArgs) -> Result<(), String> {
        // Parse and normalize coronet
        let coronet = Coronet::jjrf_parse(&args.coronet)
            .map_err(|e| format!("Invalid coronet: {}", e))?;
        let coronet_key = coronet.jjrf_display();

        // Extract parent Firemark
        let firemark = coronet.jjrf_parent_firemark();
        let firemark_key = firemark.jjrf_display();

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
            // State explicitly set to bridled: direction required
            (Some(jjrg_PaceState::Bridled), _) => {
                match &args.direction {
                    Some(d) if !d.is_empty() => Some(d.clone()),
                    Some(_) => return Err("direction must not be empty when state is bridled".to_string()),
                    None => return Err("direction is required when state is bridled".to_string()),
                }
            }
            // State explicitly set to something other than bridled: direction forbidden
            (Some(_), _) => {
                if args.direction.is_some() {
                    return Err("direction must be absent when state is not bridled".to_string());
                }
                None
            }
            // State inherited and was bridled: inherit direction
            (None, jjrg_PaceState::Bridled) => {
                args.direction.or_else(|| current_tack.direction.clone())
            }
            // State inherited and was not bridled: no direction
            (None, _) => None,
        };

        // Determine new text
        let new_text = args.text.unwrap_or_else(|| current_tack.text.clone());
        if new_text.is_empty() {
            return Err("text must not be empty".to_string());
        }

        // Create new Tack
        let new_tack = jjrg_Tack {
            ts: timestamp_full(),
            state: new_state,
            text: new_text,
            direction: new_direction,
        };

        // Prepend to tacks array
        pace.tacks.insert(0, new_tack);

        Ok(())
    }

    /// Draft - move a Pace from one Heat to another
    ///
    /// Moves the pace to the destination heat with a new Coronet.
    /// All Tack history is preserved, with a new Tack recording the draft.
    /// State is NOT changed - draft is a move operation, not a state transition.
    pub fn jjrg_draft(&mut self, args: jjrg_DraftArgs) -> Result<jjrg_DraftResult, String> {
        // Validate positioning mutual exclusivity
        let position_count = [args.before.is_some(), args.after.is_some(), args.first]
            .iter()
            .filter(|&&x| x)
            .count();
        if position_count > 1 {
            return Err("Only one of --before, --after, or --first may be specified".to_string());
        }

        // Parse and normalize source coronet
        let source_coronet = Coronet::jjrf_parse(&args.coronet)
            .map_err(|e| format!("Invalid coronet: {}", e))?;
        let source_coronet_key = source_coronet.jjrf_display();

        // Extract source Firemark from coronet
        let source_firemark = source_coronet.jjrf_parent_firemark();
        let source_firemark_key = source_firemark.jjrf_display();

        // Parse and normalize destination firemark
        let dest_firemark = Firemark::jjrf_parse(&args.to)
            .map_err(|e| format!("Invalid destination firemark: {}", e))?;
        let dest_firemark_key = dest_firemark.jjrf_display();

        // Validate source and destination are different
        if source_firemark_key == dest_firemark_key {
            return Err("Cannot draft pace to same heat".to_string());
        }

        // Verify source heat exists
        if !self.heats.contains_key(&source_firemark_key) {
            return Err(format!("Source heat '{}' not found", source_firemark_key));
        }

        // Verify destination heat exists
        if !self.heats.contains_key(&dest_firemark_key) {
            return Err(format!("Heat '{}' not found", dest_firemark_key));
        }

        // Verify pace exists in source heat
        {
            let source_heat = self.heats.get(&source_firemark_key).unwrap();
            if !source_heat.paces.contains_key(&source_coronet_key) {
                return Err(format!("Pace {} not found in heat {}", source_coronet_key, source_firemark_key));
            }
        }

        // Validate positioning target if specified
        let insert_position = if let Some(ref before_str) = args.before {
            let target = Coronet::jjrf_parse(before_str)
                .map_err(|e| format!("Invalid --before coronet: {}", e))?;
            let target_key = target.jjrf_display();
            let dest_heat = self.heats.get(&dest_firemark_key).unwrap();
            let pos = dest_heat.order.iter().position(|c| c == &target_key)
                .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, dest_firemark_key))?;
            Some(pos)
        } else if let Some(ref after_str) = args.after {
            let target = Coronet::jjrf_parse(after_str)
                .map_err(|e| format!("Invalid --after coronet: {}", e))?;
            let target_key = target.jjrf_display();
            let dest_heat = self.heats.get(&dest_firemark_key).unwrap();
            let pos = dest_heat.order.iter().position(|c| c == &target_key)
                .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, dest_firemark_key))?;
            Some(pos + 1)
        } else if args.first {
            Some(0)
        } else {
            None // Append to end
        };

        // Remove pace from source heat
        let source_heat = self.heats.get_mut(&source_firemark_key).unwrap();
        let pace_data = source_heat.paces.remove(&source_coronet_key)
            .ok_or_else(|| format!("Pace {} not found", source_coronet_key))?;
        source_heat.order.retain(|c| c != &source_coronet_key);

        // Get destination heat and allocate new coronet
        let dest_heat = self.heats.get_mut(&dest_firemark_key).unwrap();
        let new_coronet_str = format!("{}{}{}", CORONET_PREFIX, dest_firemark.jjrf_as_str(), dest_heat.next_pace_seed);

        // Create new tack recording the draft
        let draft_note = format!("Drafted from {} in {}.\n\n{}",
            source_coronet_key, source_firemark_key,
            pace_data.tacks.first().map(|t| t.text.as_str()).unwrap_or(""));

        let draft_tack = jjrg_Tack {
            ts: timestamp_full(),
            state: pace_data.tacks.first().map(|t| t.state.clone()).unwrap_or(jjrg_PaceState::Rough),
            text: draft_note,
            direction: pace_data.tacks.first().and_then(|t| t.direction.clone()),
        };

        // Build new pace with draft tack prepended
        let mut new_tacks = vec![draft_tack];
        new_tacks.extend(pace_data.tacks);

        let new_pace = jjrg_Pace {
            silks: pace_data.silks,
            tacks: new_tacks,
        };

        // Insert into destination heat
        match insert_position {
            Some(pos) => dest_heat.order.insert(pos, new_coronet_str.clone()),
            None => dest_heat.order.push(new_coronet_str.clone()),
        }
        dest_heat.paces.insert(new_coronet_str.clone(), new_pace);

        // Increment destination seed
        dest_heat.next_pace_seed = zjjrg_increment_seed(&dest_heat.next_pace_seed);

        Ok(jjrg_DraftResult { new_coronet: new_coronet_str })
    }

    /// Retire a Heat
    ///
    /// Creates trophy file, removes heat from gallops, deletes paddock file.
    /// Does NOT save gallops or commit - caller is responsible for that.
    pub fn jjrg_retire(
        &mut self,
        args: jjrg_RetireArgs,
        base_path: &Path,
        steeplechase: &[SteeplechaseEntry],
    ) -> Result<jjrg_RetireResult, String> {
        // Parse and normalize firemark
        let firemark = Firemark::jjrf_parse(&args.firemark)
            .map_err(|e| format!("Invalid firemark: {}", e))?;
        let firemark_key = firemark.jjrf_display();

        // Validate today is YYMMDD
        if !zjjrg_is_yymmdd(&args.today) {
            return Err(format!("today must be YYMMDD format, got '{}'", args.today));
        }

        // Verify heat exists
        let heat = self.heats.get(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // Read paddock content before we remove anything
        let paddock_path = base_path.join(&heat.paddock_file);
        let paddock_content = fs::read_to_string(&paddock_path)
            .map_err(|e| format!("Failed to read paddock file '{}': {}", heat.paddock_file, e))?;

        // Build trophy content
        let trophy_content = self.zjjrg_build_trophy_content(&firemark_key, heat, &paddock_content, &args.today, steeplechase)?;

        // Compute trophy path: .claude/jjm/retired/jjh_<created>-r<today>-<silks>.md
        let trophy_filename = format!(
            "jjh_{}-r{}-{}.md",
            heat.creation_time,
            args.today,
            heat.silks
        );
        let trophy_rel_path = format!(".claude/jjm/retired/{}", trophy_filename);
        let trophy_full_path = base_path.join(&trophy_rel_path);

        // Create retired directory if needed
        if let Some(parent) = trophy_full_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create retired directory: {}", e))?;
        }

        // Write trophy file
        fs::write(&trophy_full_path, trophy_content)
            .map_err(|e| format!("Failed to write trophy file: {}", e))?;

        // Capture info for result before removing heat
        let silks = heat.silks.clone();
        let paddock_file = heat.paddock_file.clone();

        // Remove heat from gallops (do NOT change next_heat_seed)
        self.heats.remove(&firemark_key);

        // Delete paddock file
        if paddock_path.exists() {
            fs::remove_file(&paddock_path)
                .map_err(|e| format!("Failed to delete paddock file: {}", e))?;
        }

        Ok(jjrg_RetireResult {
            trophy_path: trophy_rel_path,
            paddock_path: paddock_file,
            silks,
            firemark: firemark_key,
        })
    }

    /// Build trophy markdown preview (dry-run, no file modifications)
    ///
    /// Returns the markdown content that would be written to the trophy file.
    pub fn jjrg_build_trophy_preview(
        &self,
        firemark: &str,
        paddock_content: &str,
        today: &str,
        steeplechase: &[SteeplechaseEntry],
    ) -> Result<String, String> {
        // Parse and normalize firemark
        let fm = Firemark::jjrf_parse(firemark)
            .map_err(|e| format!("Invalid firemark: {}", e))?;
        let firemark_key = fm.jjrf_display();

        // Verify heat exists
        let heat = self.heats.get(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        self.zjjrg_build_trophy_content(&firemark_key, heat, paddock_content, today, steeplechase)
    }

    /// Build trophy markdown content
    fn zjjrg_build_trophy_content(
        &self,
        firemark_key: &str,
        heat: &jjrg_Heat,
        paddock_content: &str,
        today: &str,
        steeplechase: &[SteeplechaseEntry],
    ) -> Result<String, String> {
        let mut content = String::new();

        // Header
        content.push_str(&format!("# Heat Trophy: {}\n\n", heat.silks));
        content.push_str(&format!("**Firemark:** {}\n", firemark_key));
        content.push_str(&format!("**Created:** {}\n", heat.creation_time));
        content.push_str(&format!("**Retired:** {}\n", today));
        content.push_str("**Status:** retired\n\n");

        // Paddock
        content.push_str("## Paddock\n\n");
        content.push_str(paddock_content);
        if !paddock_content.ends_with('\n') {
            content.push('\n');
        }
        content.push('\n');

        // Paces (in order)
        content.push_str("## Paces\n\n");
        for coronet_key in &heat.order {
            if let Some(pace) = heat.paces.get(coronet_key) {
                // Get final state from most recent tack
                let final_state = pace.tacks.first()
                    .map(|t| match t.state {
                        jjrg_PaceState::Rough => "rough",
                        jjrg_PaceState::Bridled => "bridled",
                        jjrg_PaceState::Complete => "complete",
                        jjrg_PaceState::Abandoned => "abandoned",
                    })
                    .unwrap_or("unknown");

                content.push_str(&format!(
                    "### {} ({}) [{}]\n\n",
                    pace.silks, coronet_key, final_state
                ));

                // Tack history (newest first, as stored)
                for tack in &pace.tacks {
                    let state_str = match tack.state {
                        jjrg_PaceState::Rough => "rough",
                        jjrg_PaceState::Bridled => "bridled",
                        jjrg_PaceState::Complete => "complete",
                        jjrg_PaceState::Abandoned => "abandoned",
                    };
                    content.push_str(&format!("**[{}] {}**\n\n", tack.ts, state_str));
                    content.push_str(&tack.text);
                    if !tack.text.ends_with('\n') {
                        content.push('\n');
                    }
                    if let Some(ref direction) = tack.direction {
                        content.push_str(&format!("\n*Direction:* {}\n", direction));
                    }
                    content.push('\n');
                }
            }
        }

        // Steeplechase (newest first, as provided)
        content.push_str("## Steeplechase\n\n");
        if steeplechase.is_empty() {
            content.push_str("(no entries)\n\n");
        } else {
            for entry in steeplechase {
                // Format: ### {date} - {coronet or "Heat"} - {action or "notch"}
                let identity = entry.coronet.as_deref().unwrap_or("Heat");
                let action = entry.action.as_deref().unwrap_or("notch");
                content.push_str(&format!(
                    "### {} - {} - {}\n\n",
                    entry.timestamp, identity, action
                ));
                content.push_str(&entry.subject);
                if !entry.subject.ends_with('\n') {
                    content.push('\n');
                }
                content.push('\n');
            }
        }

        Ok(content)
    }
}

/// Read text from stdin (for CLI commands)
pub fn jjrg_read_stdin() -> Result<String, String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)
        .map_err(|e| format!("Failed to read from stdin: {}", e))?;
    Ok(buffer.trim_end().to_string())
}

/// Read text from stdin, returning None if stdin is empty or a tty
pub fn jjrg_read_stdin_optional() -> Result<Option<String>, String> {
    use std::io::IsTerminal;

    // If stdin is a terminal, no input was piped
    if std::io::stdin().is_terminal() {
        return Ok(None);
    }

    let text = jjrg_read_stdin()?;
    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}
