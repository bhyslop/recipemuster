// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops validation routines
//!
//! Validates Gallops JSON structure against schema rules.

use std::collections::HashSet;
use crate::jjrf_favor::{JJRF_CHARSET, JJRF_FIREMARK_LEN, JJRF_CORONET_LEN, JJRF_FIREMARK_PREFIX, JJRF_CORONET_PREFIX};
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

    // Rule 1b: next_pace_seed — the single global pace-mint seed (JJS0
    // jjdgm_pace_seed) — must be 5 URL-safe base64 characters.
    if gallops.next_pace_seed.len() != JJRF_CORONET_LEN {
        errors.push(format!(
            "next_pace_seed must be {} characters, got {}",
            JJRF_CORONET_LEN, gallops.next_pace_seed.len()
        ));
    } else if !zjjrg_is_base64(&gallops.next_pace_seed) {
        errors.push(format!(
            "next_pace_seed contains invalid base64 characters: '{}'",
            gallops.next_pace_seed
        ));
    }

    // Rule 2: heats object exists (implicitly satisfied by struct)

    // Validate each Heat
    for (heat_key, heat) in &gallops.heats {
        zjjrg_validate_heat(heat_key, heat, &mut errors);
    }

    // Cross-heat Coronet uniqueness (JJS0 jjdt_coronet): a Coronet is a flat
    // global id and appears in exactly one heat's paces — the immutable-id
    // invariant that replaces the retired heat-embedding rule.
    let mut seen: HashSet<&String> = HashSet::new();
    for heat in gallops.heats.values() {
        for pace_key in heat.paces.keys() {
            if !seen.insert(pace_key) {
                errors.push(format!(
                    "Coronet '{}' appears in more than one heat (cross-heat uniqueness)",
                    pace_key
                ));
            }
        }
    }

    // Top-level order/heats twin invariant — the heat-level mirror of Rule 5.
    // heat_order must be a duplicate-free permutation of the heats key-set: no
    // firemark twice (else jjx_list renders the heat twice — the observed
    // defect), no order slot without a heat, no heat without an order slot (else
    // the heat is invisible). jjrg_reconcile is the repair-half that restores
    // this on read; this check-half is what makes the strict save path refuse a
    // divergence a mutator would otherwise persist.
    let heat_order_set: HashSet<&String> = gallops.heat_order.iter().collect();
    if heat_order_set.len() != gallops.heat_order.len() {
        errors.push("heat_order contains duplicate entries".to_string());
    }
    let in_order_not_heats: Vec<_> = gallops
        .heat_order
        .iter()
        .filter(|fm| !gallops.heats.contains_key(*fm))
        .collect();
    let in_heats_not_order: Vec<_> = gallops
        .heats
        .keys()
        .filter(|fm| !heat_order_set.contains(fm))
        .collect();
    if !in_order_not_heats.is_empty() {
        errors.push(format!(
            "heat_order contains firemarks not in heats: {:?}",
            in_order_not_heats
        ));
    }
    if !in_heats_not_order.is_empty() {
        errors.push(format!(
            "heats contains firemarks not in heat_order: {:?}",
            in_heats_not_order
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Reconcile the top-level `heat_order` against `heats` — the repair-half of the
/// top-level order/heats twin invariant (the heat-level mirror of the per-heat
/// Rule 5). Ordering-only and idempotent: it restores `heat_order` to a
/// duplicate-free permutation of the `heats` key-set without inventing or losing
/// a heat record, so it is information-safe to run unconditionally on read.
///
/// Three axes, each a git merge concatenating `heat_order` can produce:
///   - a firemark appearing twice in `heat_order` → keep the first (earliest /
///     highest-priority) slot, drop the rest (the observed jjx_list double-render);
///   - a `heat_order` slot naming no heat → drop it (it renders nothing);
///   - a heat absent from `heat_order` → append it (in `heats` key order) so it is
///     never invisible.
///
/// Returns one description per axis that changed something; an empty vec is the
/// no-op (already-reconciled) case, and callers read `.is_empty()` as "the store
/// was clean". The strict assertion that this repair is complete lives in
/// `jjrg_validate`, which the save-side load-back runs so a mutator that writes a
/// divergence is refused loud rather than silently healed. Disjoint from the
/// reprieve write-forward (episode-gated schema migration): this is a standing
/// invariant repair, not a migration, and registers no reprieve episode.
///
/// After the top-level pass, this also drives the per-heat order/paces reconcile
/// (`zjjrg_reconcile_heat`) over every heat, so one call heals both the heat-level and
/// pace-level twins in memory and the shared report names every axis it touched.
pub fn jjrg_reconcile(gallops: &mut jjrg_Gallops) -> Vec<String> {
    let mut kept: Vec<String> = Vec::with_capacity(gallops.heat_order.len());
    let mut seen: HashSet<String> = HashSet::new();
    let mut dups: Vec<String> = Vec::new();
    let mut orphans: Vec<String> = Vec::new();

    for fm in &gallops.heat_order {
        if !gallops.heats.contains_key(fm) {
            orphans.push(fm.clone());
        } else if seen.insert(fm.clone()) {
            kept.push(fm.clone());
        } else {
            dups.push(fm.clone());
        }
    }

    let mut appended: Vec<String> = Vec::new();
    for fm in gallops.heats.keys() {
        if !seen.contains(fm) {
            seen.insert(fm.clone());
            kept.push(fm.clone());
            appended.push(fm.clone());
        }
    }

    let mut report = Vec::new();
    if !dups.is_empty() {
        report.push(format!(
            "deduped heat_order (kept first, dropped {}): {}",
            dups.len(),
            dups.join(", ")
        ));
    }
    if !orphans.is_empty() {
        report.push(format!(
            "dropped {} heat_order slot(s) naming no heat: {}",
            orphans.len(),
            orphans.join(", ")
        ));
    }
    if !appended.is_empty() {
        report.push(format!(
            "appended {} heat(s) missing from heat_order: {}",
            appended.len(),
            appended.join(", ")
        ));
    }

    if !report.is_empty() {
        gallops.heat_order = kept;
    }

    // Per-heat order/paces reconcile — the Rule-5 mirror one level down. Each heat's own
    // order/paces twin carries the same merge-concat exposure, repaired by the same
    // ordering-only rules (zjjrg_reconcile_heat), which append heat-qualified lines to the
    // shared report. One deliberate divergence lives in that helper: a per-heat orphan is
    // not dropped but left for Rule 5 to brick.
    for (heat_key, heat) in gallops.heats.iter_mut() {
        zjjrg_reconcile_heat(heat_key, heat, &mut report);
    }

    report
}

/// Reconcile one heat's `order` against its `paces` — the per-heat repair-half of the
/// order/paces twin (Rule 5 in `zjjrg_validate_heat` is its check-half). Ordering-only
/// and idempotent, the direct mirror of the top-level `jjrg_reconcile` one level down,
/// with ONE deliberate divergence on the orphan axis:
///   - a coronet appearing twice in `order` → keep the first slot, drop the rest;
///   - a pace in `paces` absent from `order` → append it (in paces key order) so it is
///     never invisible in listings;
///   - a coronet in `order` with NO `paces` record (an orphan slot) → LEFT IN PLACE.
///
/// At the top level an orphan `heat_order` slot renders nothing and is dropped; a per-heat
/// orphan slot is instead the last surviving evidence of a pace whose record a merge lost,
/// so dropping it would silently erase that evidence. It is kept and left for Rule 5 to
/// reject — a per-heat orphan stays Broken/exit-1, never repaired and never invented (the
/// cinched normalize-vs-brick line). Each repaired axis pushes one heat-qualified line onto
/// `report`; an orphan pushes nothing, so a store carrying only orphans reports no repair
/// and validate bricks it.
fn zjjrg_reconcile_heat(heat_key: &str, heat: &mut jjrg_Heat, report: &mut Vec<String>) {
    let mut kept: Vec<String> = Vec::with_capacity(heat.order.len());
    let mut seen: HashSet<String> = HashSet::new();
    let mut dups: Vec<String> = Vec::new();

    // First occurrence of each coronet is kept — orphans (not in paces) ride along
    // deliberately so Rule 5 still bricks them; only duplicates are dropped.
    for coronet in &heat.order {
        if seen.insert(coronet.clone()) {
            kept.push(coronet.clone());
        } else {
            dups.push(coronet.clone());
        }
    }

    let mut appended: Vec<String> = Vec::new();
    for coronet in heat.paces.keys() {
        if !seen.contains(coronet) {
            seen.insert(coronet.clone());
            kept.push(coronet.clone());
            appended.push(coronet.clone());
        }
    }

    let mut changed = false;
    if !dups.is_empty() {
        report.push(format!(
            "{}: deduped order (kept first, dropped {}): {}",
            heat_key,
            dups.len(),
            dups.join(", ")
        ));
        changed = true;
    }
    if !appended.is_empty() {
        report.push(format!(
            "{}: appended {} pace(s) missing from order: {}",
            heat_key,
            appended.len(),
            appended.join(", ")
        ));
        changed = true;
    }

    if changed {
        heat.order = kept;
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
        zjjrg_validate_pace(&heat_ctx, pace_key, pace, errors);
    }
}

fn zjjrg_validate_pace(
    heat_ctx: &str,
    pace_key: &str,
    pace: &jjrg_Pace,
    errors: &mut Vec<String>,
) {
    let pace_ctx = format!("{} Pace '{}'", heat_ctx, pace_key);

    // Rule 6: Pace key must match ₢[A-Za-z0-9_-]{5}. A Coronet is a flat global
    // index and carries no parent-heat identity (JJS0 jjdt_coronet) — the retired
    // heat-embedding check is gone; cross-heat uniqueness is enforced in
    // jjrg_validate.
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
        } else if !zjjrg_is_base64(suffix) {
            errors.push(format!(
                "{}: key contains invalid base64 characters",
                pace_ctx
            ));
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
