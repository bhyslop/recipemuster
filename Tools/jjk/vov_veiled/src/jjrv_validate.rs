// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops validation routines
//!
//! Validates Gallops JSON structure against schema rules.

use std::collections::HashSet;
use crate::jjrf_favor::{JJRF_CHARSET, JJRF_FIREMARK_LEN, JJRF_PACE_INDEX_LEN, JJRF_CORONET_LEN, JJRF_FIREMARK_PREFIX, JJRF_CORONET_PREFIX};
use crate::jjrt_types::*;

/// Check if string contains only URL-safe base64 characters
#[allow(dead_code)]
pub(crate) fn zjjrg_is_base64(s: &str) -> bool {
    s.bytes().all(|b| JJRF_CHARSET.contains(&b))
}

/// Check if string is valid kebab-case (non-empty, alphanumeric with hyphens, case-insensitive)
#[allow(dead_code)]
pub(crate) fn zjjrg_is_kebab_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Pattern: [a-zA-Z0-9]+(-[a-zA-Z0-9]+)*
    let parts: Vec<&str> = s.split('-').collect();
    for part in parts {
        if part.is_empty() {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
    }
    true
}

/// Check if string matches YYMMDD format
#[allow(dead_code)]
pub(crate) fn zjjrg_is_yymmdd(s: &str) -> bool {
    if s.len() != 6 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_digit())
}

/// Check if string matches YYMMDD-HHMM format
#[allow(dead_code)]
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

/// Check if string is valid commit SHA format (used for basis field)
///
/// Length is derived from JJRG_UNKNOWN_BASIS constant.
#[allow(dead_code)]
pub(crate) fn zjjrg_is_commit_sha(s: &str) -> bool {
    s.len() == JJRG_UNKNOWN_BASIS.len() && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate the Gallops structure
///
/// Returns Ok(()) if valid, Err(Vec<String>) with all validation errors otherwise.
pub fn jjrg_validate(gallops: &jjrg_Gallops) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Rule 1: next_heat_seed must be 2 URL-safe base64 characters
    if gallops.next_heat_seed.len() != JJRF_FIREMARK_LEN {
        errors.push(format!(
            "next_heat_seed must be {} characters, got {}",
            JJRF_FIREMARK_LEN, gallops.next_heat_seed.len()
        ));
    } else if !zjjrg_is_base64(&gallops.next_heat_seed) {
        errors.push(format!(
            "next_heat_seed contains invalid base64 characters: '{}'",
            gallops.next_heat_seed
        ));
    }

    // Rule 2: heats object exists (implicitly satisfied by struct)

    // Validate each Heat
    for (heat_key, heat) in &gallops.heats {
        zjjrg_validate_heat(heat_key, heat, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn zjjrg_validate_heat(heat_key: &str, heat: &jjrg_Heat, errors: &mut Vec<String>) {
    let heat_ctx = format!("Heat '{}'", heat_key);

    // Rule 3: Heat key must match ₣[A-Za-z0-9_-]{2}
    if !heat_key.starts_with('₣') {
        errors.push(format!("{}: key must start with '₣'", heat_ctx));
    } else {
        let suffix = &heat_key[JJRF_FIREMARK_PREFIX.len_utf8()..];
        if suffix.len() != JJRF_FIREMARK_LEN {
            errors.push(format!(
                "{}: key must have {} base64 chars after '₣', got {}",
                heat_ctx,
                JJRF_FIREMARK_LEN,
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
    // silks (non-empty alphanumeric-kebab)
    if !zjjrg_is_kebab_case(&heat.silks) {
        errors.push(format!(
            "{}: silks must be non-empty alphanumeric-kebab (letters, digits, hyphens), got '{}'",
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
    if heat.next_pace_seed.len() != JJRF_PACE_INDEX_LEN {
        errors.push(format!(
            "{}: next_pace_seed must be {} characters, got {}",
            heat_ctx,
            JJRF_PACE_INDEX_LEN,
            heat.next_pace_seed.len()
        ));
    } else if !zjjrg_is_base64(&heat.next_pace_seed) {
        errors.push(format!(
            "{}: next_pace_seed contains invalid base64 characters",
            heat_ctx
        ));
    }

    // Extract heat identity (base64 part without prefix) for pace validation
    let heat_identity = if heat_key.starts_with('₣') && heat_key.len() >= JJRF_FIREMARK_PREFIX.len_utf8() + JJRF_FIREMARK_LEN {
        Some(&heat_key[JJRF_FIREMARK_PREFIX.len_utf8()..])
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
        zjjrg_validate_pace(&heat_ctx, heat_identity, pace_key, pace, errors);
    }
}

fn zjjrg_validate_pace(
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
        let suffix = &pace_key[JJRF_CORONET_PREFIX.len_utf8()..];
        if suffix.len() != JJRF_CORONET_LEN {
            errors.push(format!(
                "{}: key must have {} base64 chars after '₢', got {}",
                pace_ctx,
                JJRF_CORONET_LEN,
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
                        &suffix[..JJRF_FIREMARK_LEN.min(suffix.len())]
                    ));
                }
            }
        }
    }

    // Rule 7: tacks must be non-empty array
    if pace.tacks.is_empty() {
        errors.push(format!("{}: tacks array must not be empty", pace_ctx));
    }

    // Validate each Tack
    for (i, tack) in pace.tacks.iter().enumerate() {
        zjjrg_validate_tack(&pace_ctx, i, tack, errors);
    }
}

fn zjjrg_validate_tack(pace_ctx: &str, index: usize, tack: &jjrg_Tack, errors: &mut Vec<String>) {
    let tack_ctx = format!("{} Tack[{}]", pace_ctx, index);

    // Rule 8: ts must be YYMMDD-HHMM
    if !zjjrg_is_yymmdd_hhmm(&tack.ts) {
        errors.push(format!(
            "{}: ts must be YYMMDD-HHMM format, got '{}'",
            tack_ctx, tack.ts
        ));
    }

    // Rule 8: text must be non-empty — the line array holds no content when it is
    // empty or every line is empty (an empty docket splits to [""], not []).
    if tack.text.iter().all(|line| line.is_empty()) {
        errors.push(format!("{}: text must not be empty", tack_ctx));
    }

    // Rule 8: silks must be non-empty alphanumeric-kebab
    if !zjjrg_is_kebab_case(&tack.silks) {
        errors.push(format!(
            "{}: silks must be non-empty alphanumeric-kebab (letters, digits, hyphens), got '{}'",
            tack_ctx, tack.silks
        ));
    }

    // Rule 8: basis must be valid format (7 hex chars)
    if !zjjrg_is_commit_sha(&tack.basis) {
        errors.push(format!(
            "{}: basis must be 7 hex characters, got '{}'",
            tack_ctx, tack.basis
        ));
    }

    // Rule 9: designation coherence. A bridled pace always carries its tier
    // (the designation IS the tier record); a rough pace never carries one
    // (bridle sets it, release and every revert trigger wipe it); resolved
    // states may carry either as close-time provenance. Effort never rides
    // without a tier.
    match tack.state {
        jjrg_PaceState::Bridled => {
            if tack.tier.is_none() {
                errors.push(format!("{}: bridled tack must carry a tier", tack_ctx));
            }
        }
        jjrg_PaceState::Rough => {
            if tack.tier.is_some() || tack.effort.is_some() {
                errors.push(format!("{}: rough tack must carry no tier or effort", tack_ctx));
            }
        }
        jjrg_PaceState::Complete | jjrg_PaceState::Abandoned => {}
    }
    if tack.effort.is_some() && tack.tier.is_none() {
        errors.push(format!("{}: effort must not ride without a tier", tack_ctx));
    }
}
