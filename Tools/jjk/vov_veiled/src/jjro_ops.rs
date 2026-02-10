// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops write operations
//!
//! All mutation operations on Gallops: nominate, slate, rail, tally, draft, retire, furlough.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;
use crate::jjrf_favor::{jjrf_Firemark as Firemark, jjrf_Coronet as Coronet, JJRF_FIREMARK_PREFIX as FIREMARK_PREFIX, JJRF_CORONET_PREFIX as CORONET_PREFIX};
use crate::jjrs_steeplechase::jjrs_SteeplechaseEntry as SteeplechaseEntry;
use crate::jjrt_types::*;
use crate::jjru_util::{zjjrg_increment_seed, jjrg_make_tack};
use crate::jjrv_validate::{zjjrg_is_kebab_case, zjjrg_is_yymmdd};

/// Nominate a new Heat
///
/// Creates a new Heat with empty Pace structure and creates the paddock file.
pub fn jjrg_nominate(gallops: &mut jjrg_Gallops, args: jjrg_NominateArgs, base_path: &Path) -> Result<jjrg_NominateResult, String> {
    // Validate silks is alphanumeric-kebab
    if !zjjrg_is_kebab_case(&args.silks) {
        return Err(format!("silks must be non-empty alphanumeric-kebab (letters, digits, hyphens), got '{}'", args.silks));
    }

    // Validate created is YYMMDD
    if !zjjrg_is_yymmdd(&args.created) {
        return Err(format!("created must be YYMMDD format, got '{}'", args.created));
    }

    // Allocate Firemark from next_heat_seed
    let firemark_str = format!("{}{}", FIREMARK_PREFIX, gallops.next_heat_seed);
    let heat_id = gallops.next_heat_seed.clone();

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
        status: jjrg_HeatStatus::Stabled,
        order: Vec::new(),
        next_pace_seed: "AAA".to_string(),
        paddock_file,
        paces: BTreeMap::new(),
    };

    // Insert Heat
    gallops.heats.insert(firemark_str.clone(), heat);

    // Increment next_heat_seed
    gallops.next_heat_seed = zjjrg_increment_seed(&gallops.next_heat_seed);

    Ok(jjrg_NominateResult { firemark: firemark_str })
}

/// Slate a new Pace
///
/// Adds a new Pace to a Heat with an initial Tack in rough state.
/// Positioning: use before/after/first to insert at specific location.
pub fn jjrg_slate(gallops: &mut jjrg_Gallops, args: jjrg_SlateArgs) -> Result<jjrg_SlateResult, String> {
    // Validate silks is alphanumeric-kebab
    if !zjjrg_is_kebab_case(&args.silks) {
        return Err(format!("silks must be non-empty alphanumeric-kebab (letters, digits, hyphens), got '{}'", args.silks));
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
    let heat = gallops.heats.get_mut(&firemark_key)
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

    // Create initial Tack and Pace
    let tack = jjrg_make_tack(
        jjrg_PaceState::Rough,
        args.text,
        args.silks,
        None,
    );
    let pace = jjrg_Pace {
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
pub fn jjrg_rail(gallops: &mut jjrg_Gallops, args: jjrg_RailArgs) -> Result<Vec<String>, String> {
    // Parse and normalize firemark
    let firemark = Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let firemark_key = firemark.jjrf_display();

    // Verify Heat exists
    let heat = gallops.heats.get_mut(&firemark_key)
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
            // Find first actionable pace (rough or bridled)
            // If none found, use end of array (nothing actionable to precede)
            let first_actionable_idx = heat.order.iter().position(|coronet| {
                if let Some(pace) = heat.paces.get(coronet) {
                    if let Some(tack) = pace.tacks.first() {
                        matches!(tack.state, jjrg_PaceState::Rough | jjrg_PaceState::Bridled)
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            // Use len() as fallback so adjustment logic works correctly for "end" position
            let target_idx = first_actionable_idx.unwrap_or(heat.order.len());
            // If moving from before target, the target shifts down after removal
            if current_pos < target_idx {
                target_idx - 1
            } else {
                target_idx
            }
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
pub fn jjrg_tally(gallops: &mut jjrg_Gallops, args: jjrg_TallyArgs) -> Result<(), String> {
    // Parse and normalize coronet
    let coronet = Coronet::jjrf_parse(&args.coronet)
        .map_err(|e| format!("Invalid coronet: {}", e))?;
    let coronet_key = coronet.jjrf_display();

    // Extract parent Firemark
    let firemark = coronet.jjrf_parent_firemark();
    let firemark_key = firemark.jjrf_display();

    // Verify Heat exists
    let heat = gallops.heats.get_mut(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    // Verify Pace exists
    let pace = heat.paces.get_mut(&coronet_key)
        .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;

    // Read current Tack
    let current_tack = pace.tacks.first()
        .ok_or_else(|| "Pace has no tacks (should never happen)".to_string())?;

    // Check for the new-spec-invalidates-bridled condition:
    // If pace is currently bridled AND new spec text is provided AND no explicit --state,
    // then auto-reset to rough (new spec invalidates old direction)
    let auto_reset_to_rough = args.state.is_none()
        && matches!(current_tack.state, jjrg_PaceState::Bridled)
        && args.text.is_some();

    // Determine new state
    let new_state = if auto_reset_to_rough {
        jjrg_PaceState::Rough
    } else {
        args.state.clone().unwrap_or_else(|| current_tack.state.clone())
    };

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
        // Auto-reset to rough: no direction
        _ if auto_reset_to_rough => None,
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

    // Determine new silks
    let new_silks = args.silks.unwrap_or_else(|| current_tack.silks.clone());

    // Create new Tack
    let new_tack = jjrg_make_tack(
        new_state,
        new_text,
        new_silks,
        new_direction,
    );

    // Prepend to tacks array
    pace.tacks.insert(0, new_tack);

    Ok(())
}

/// Draft - move a Pace from one Heat to another
///
/// Moves the pace to the destination heat with a new Coronet.
/// All Tack history is preserved, with a new Tack recording the draft.
/// State is NOT changed - draft is a move operation, not a state transition.
pub fn jjrg_draft(gallops: &mut jjrg_Gallops, args: jjrg_DraftArgs) -> Result<jjrg_DraftResult, String> {
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
    if !gallops.heats.contains_key(&source_firemark_key) {
        return Err(format!("Source heat '{}' not found", source_firemark_key));
    }

    // Verify destination heat exists
    if !gallops.heats.contains_key(&dest_firemark_key) {
        return Err(format!("Heat '{}' not found", dest_firemark_key));
    }

    // Verify pace exists in source heat
    {
        let source_heat = gallops.heats.get(&source_firemark_key).unwrap();
        if !source_heat.paces.contains_key(&source_coronet_key) {
            return Err(format!("Pace {} not found in heat {}", source_coronet_key, source_firemark_key));
        }
    }

    // Validate positioning target if specified
    let insert_position = if let Some(ref before_str) = args.before {
        let target = Coronet::jjrf_parse(before_str)
            .map_err(|e| format!("Invalid --before coronet: {}", e))?;
        let target_key = target.jjrf_display();
        let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
        let pos = dest_heat.order.iter().position(|c| c == &target_key)
            .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, dest_firemark_key))?;
        Some(pos)
    } else if let Some(ref after_str) = args.after {
        let target = Coronet::jjrf_parse(after_str)
            .map_err(|e| format!("Invalid --after coronet: {}", e))?;
        let target_key = target.jjrf_display();
        let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
        let pos = dest_heat.order.iter().position(|c| c == &target_key)
            .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, dest_firemark_key))?;
        Some(pos + 1)
    } else if args.first {
        Some(0)
    } else {
        None // Append to end
    };

    // Remove pace from source heat
    let source_heat = gallops.heats.get_mut(&source_firemark_key).unwrap();
    let pace_data = source_heat.paces.remove(&source_coronet_key)
        .ok_or_else(|| format!("Pace {} not found", source_coronet_key))?;
    source_heat.order.retain(|c| c != &source_coronet_key);

    // Get destination heat and allocate new coronet
    let dest_heat = gallops.heats.get_mut(&dest_firemark_key).unwrap();
    let new_coronet_str = format!("{}{}{}", CORONET_PREFIX, dest_firemark.jjrf_as_str(), dest_heat.next_pace_seed);

    // Create new tack recording the draft
    let first_tack = pace_data.tacks.first();
    let draft_note = format!("Drafted from {} in {}.\n\n{}",
        source_coronet_key, source_firemark_key,
        first_tack.map(|t| t.text.as_str()).unwrap_or(""));

    let draft_tack = jjrg_make_tack(
        first_tack.map(|t| t.state.clone()).unwrap_or(jjrg_PaceState::Rough),
        draft_note,
        first_tack.map(|t| t.silks.clone()).unwrap_or_default(),
        first_tack.and_then(|t| t.direction.clone()),
    );

    // Build new pace with draft tack prepended
    let mut new_tacks = vec![draft_tack];
    new_tacks.extend(pace_data.tacks);

    let new_pace = jjrg_Pace {
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
    gallops: &mut jjrg_Gallops,
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
    let heat = gallops.heats.get(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    // Read paddock content before we remove anything
    let paddock_path = base_path.join(&heat.paddock_file);
    let paddock_content = fs::read_to_string(&paddock_path)
        .map_err(|e| format!("Failed to read paddock file '{}': {}", heat.paddock_file, e))?;

    // Build trophy content
    let trophy_content = zjjrg_build_trophy_content(&firemark_key, heat, &paddock_content, &args.today, steeplechase)?;

    // Compute trophy path: .claude/jjm/retired/jjh_b<created>-r<today>-<silks>.md
    let trophy_filename = format!(
        "jjh_b{}-r{}-{}.md",
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
    // Use shift_remove to preserve order of remaining heats
    gallops.heats.shift_remove(&firemark_key);

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
    gallops: &jjrg_Gallops,
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
    let heat = gallops.heats.get(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    zjjrg_build_trophy_content(&firemark_key, heat, paddock_content, today, steeplechase)
}

/// Build trophy markdown content
fn zjjrg_build_trophy_content(
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

            let pace_silks = pace.tacks.first()
                .map(|t| t.silks.as_str())
                .unwrap_or("unknown");

            content.push_str(&format!(
                "### {} ({}) [{}]\n\n",
                pace_silks, coronet_key, final_state
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

/// Curry - update Heat paddock with chalk entry
///
/// Writes new paddock content and creates a chalk commit recording the change.
pub fn jjrg_curry(
    gallops_path: &std::path::Path,
    firemark: &Firemark,
    new_content: &str,
    note: Option<&str>,
) -> Result<(), String> {
    use std::fs;

    // Load gallops
    let gallops = jjrg_Gallops::jjrg_load(gallops_path)
        .map_err(|e| format!("Failed to load Gallops: {}", e))?;

    // Verify heat exists
    let firemark_key = firemark.jjrf_display();
    let heat = gallops.heats.get(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    // Write new paddock content
    let paddock_path = std::path::Path::new(&heat.paddock_file);
    fs::write(paddock_path, new_content)
        .map_err(|e| format!("Failed to write paddock file: {}", e))?;

    // Build chalk message
    let description = if let Some(n) = note {
        format!("paddock curried: {}", n)
    } else {
        "paddock curried".to_string()
    };

    // Create chalk commit using vvc
    use crate::jjrn_notch::jjrn_format_heat_discussion;
    let message = jjrn_format_heat_discussion(firemark, &description);

    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![paddock_path.to_string_lossy().to_string()],
        message,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    // Acquire lock for commit
    let lock = vvc::vvcc_CommitLock::vvcc_acquire()
        .map_err(|e| format!("Failed to acquire commit lock: {}", e))?;

    match vvc::machine_commit(&lock, &commit_args) {
        Ok(hash) => {
            eprintln!("jjx_curry: committed {}", &hash[..8]);
            Ok(())
        }
        Err(e) => Err(format!("Commit failed: {}", e)),
    }
    // lock released here
}

/// Furlough a Heat - change status or rename
///
/// Updates heat status (racing/stabled) and/or silks.
/// At least one change must be requested.
pub fn jjrg_furlough(gallops: &mut jjrg_Gallops, args: jjrg_FurloughArgs) -> Result<(), String> {
    // Validate at least one option provided
    if !args.racing && !args.stabled && args.silks.is_none() {
        return Err("At least one option required: --racing, --stabled, or --silks".to_string());
    }

    // Validate racing/stabled mutual exclusivity
    if args.racing && args.stabled {
        return Err("Cannot specify both --racing and --stabled".to_string());
    }

    // Validate silks if provided
    if let Some(ref silks) = args.silks {
        if !zjjrg_is_kebab_case(silks) {
            return Err(format!("silks must be non-empty alphanumeric-kebab (letters, digits, hyphens), got '{}'", silks));
        }
    }

    // Parse and normalize firemark
    let firemark = Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let firemark_key = firemark.jjrf_display();

    // Verify Heat exists and check status
    {
        let heat = gallops.heats.get(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // Check not retired (terminal state)
        if heat.status == jjrg_HeatStatus::Retired {
            return Err(format!("Heat '{}' is retired (terminal state)", firemark_key));
        }

        // Note: requesting the same status is allowed — it promotes the heat
        // to the top of its list (racing or stabled) for reordering purposes.
    }

    // Apply status change if requested — also promotes heat to top of list
    if args.racing {
        gallops.heats.get_mut(&firemark_key).unwrap().status = jjrg_HeatStatus::Racing;
        if let Some(heat_entry) = gallops.heats.shift_remove(&firemark_key) {
            gallops.heats.shift_insert(0, firemark_key.clone(), heat_entry);
        }
    } else if args.stabled {
        gallops.heats.get_mut(&firemark_key).unwrap().status = jjrg_HeatStatus::Stabled;
        if let Some(heat_entry) = gallops.heats.shift_remove(&firemark_key) {
            gallops.heats.shift_insert(0, firemark_key.clone(), heat_entry);
        }
    }

    // Apply silks change if requested
    if let Some(silks) = args.silks {
        gallops.heats.get_mut(&firemark_key).unwrap().silks = silks;
    }

    Ok(())
}

/// Garland a Heat - celebrate completion and create continuation
///
/// Updates source heat silks to garlanded form and status to stabled.
/// Creates new heat with continuation silks, status racing.
/// Transfers actionable paces (rough/bridled) to new heat.
/// Retains complete/abandoned paces in garlanded heat.
pub fn jjrg_garland(gallops: &mut jjrg_Gallops, args: jjrg_GarlandArgs, base_path: &Path) -> Result<jjrg_GarlandResult, String> {
    use crate::jjrc_core::jjrc_timestamp_from_env;
    use crate::jjrq_query::{jjrq_parse_silks_sequence, jjrq_build_garlanded_silks, jjrq_build_continuation_silks};

    // Parse and normalize firemark
    let firemark = Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let firemark_key = firemark.jjrf_display();

    // Verify source heat exists and extract needed data (avoid holding the borrow)
    let (source_silks, source_paddock_file, source_order, actionable_count, retained_count) = {
        let source_heat = gallops.heats.get(&firemark_key)
            .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

        // Parse source silks to determine sequence
        let (base, sequence) = jjrq_parse_silks_sequence(&source_heat.silks);
        let seq = sequence.unwrap_or(1);

        // Count actionable paces
        let actionable_count = source_heat.order.iter().filter(|coronet_key| {
            if let Some(pace) = source_heat.paces.get(*coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    matches!(tack.state, jjrg_PaceState::Rough | jjrg_PaceState::Bridled)
                } else {
                    false
                }
            } else {
                false
            }
        }).count();

        // Validate there are actionable paces to transfer
        if actionable_count == 0 {
            return Err(format!("Heat '{}' has no actionable paces to transfer", firemark_key));
        }

        // Count retained paces
        let retained_count = source_heat.paces.len() - actionable_count;

        // Count complete paces for steeplechase marker
        let complete_count = source_heat.order.iter().filter(|coronet_key| {
            if let Some(pace) = source_heat.paces.get(*coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    tack.state == jjrg_PaceState::Complete
                } else {
                    false
                }
            } else {
                false
            }
        }).count();

        // Create steeplechase marker entry
        let marker_text = format!("Garlanded at pace {} — magnificent service", complete_count);

        // Update source heat: add steeplechase marker (prepend to paddock file)
        let source_paddock_path = base_path.join(&source_heat.paddock_file);
        let existing_content = std::fs::read_to_string(&source_paddock_path)
            .map_err(|e| format!("Failed to read paddock file: {}", e))?;

        let updated_content = format!("{}\n\n{}", marker_text, existing_content);
        std::fs::write(&source_paddock_path, updated_content)
            .map_err(|e| format!("Failed to write paddock file: {}", e))?;

        ((base, seq), source_heat.paddock_file.clone(), source_heat.order.clone(), actionable_count, retained_count)
    };

    // Build garlanded and continuation silks
    let (base, seq) = source_silks;
    let garlanded_silks = jjrq_build_garlanded_silks(&base, seq);
    let continuation_silks = jjrq_build_continuation_silks(&base, seq + 1);

    // Nominate new heat with continuation silks
    let created_ts = jjrc_timestamp_from_env();
    let nominate_args = jjrg_NominateArgs {
        silks: continuation_silks.clone(),
        created: created_ts,
    };
    let nominate_result = jjrg_nominate(gallops, nominate_args, base_path)?;
    let new_firemark_key = nominate_result.firemark.clone();

    // Get destination heat paddock file for copy
    let dest_paddock_file = {
        let dest_heat = gallops.heats.get(&new_firemark_key)
            .ok_or_else(|| "Failed to retrieve newly nominated heat".to_string())?;
        dest_heat.paddock_file.clone()
    };

    // Read original paddock content before modifications (for new heat)
    let source_paddock_path = base_path.join(&source_paddock_file);
    let original_paddock_content = std::fs::read_to_string(&source_paddock_path)
        .map_err(|e| format!("Failed to read paddock file: {}", e))?;

    // Extract original content (removing marker we just added)
    let existing_content = original_paddock_content.split_once("\n\n")
        .map(|(_, rest)| rest)
        .unwrap_or(&original_paddock_content);

    // Copy paddock content from source to new heat
    let dest_paddock_path = base_path.join(&dest_paddock_file);
    std::fs::write(&dest_paddock_path, existing_content)
        .map_err(|e| format!("Failed to copy paddock to new heat: {}", e))?;

    // Draft all actionable paces to new heat (in order)
    for coronet_key in source_order {
        if let Some(pace) = gallops.heats.get(&firemark_key).unwrap().paces.get(&coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                if matches!(tack.state, jjrg_PaceState::Rough | jjrg_PaceState::Bridled) {
                    // Draft this pace to new heat (append to end)
                    let draft_args = jjrg_DraftArgs {
                        coronet: coronet_key.clone(),
                        to: new_firemark_key.clone(),
                        before: None,
                        after: None,
                        first: false, // Append to end to maintain order
                    };
                    jjrg_draft(gallops, draft_args)?;
                }
            }
        }
    }

    // Update source heat: change silks to garlanded, set status to stabled
    let source_heat_mut = gallops.heats.get_mut(&firemark_key).unwrap();
    source_heat_mut.silks = garlanded_silks.clone();
    source_heat_mut.status = jjrg_HeatStatus::Stabled;

    // Set new heat status to racing and move to front
    let new_heat_mut = gallops.heats.get_mut(&new_firemark_key).unwrap();
    new_heat_mut.status = jjrg_HeatStatus::Racing;
    if let Some(heat_entry) = gallops.heats.shift_remove(&new_firemark_key) {
        gallops.heats.shift_insert(0, new_firemark_key.clone(), heat_entry);
    }

    Ok(jjrg_GarlandResult {
        old_firemark: firemark_key,
        old_silks: garlanded_silks,
        new_firemark: new_firemark_key,
        new_silks: continuation_silks,
        paces_transferred: actionable_count,
        paces_retained: retained_count,
    })
}

/// Restring - bulk draft multiple paces atomically
///
/// Moves multiple paces from source heat to destination heat in a single operation.
/// All paces are validated before any mutations occur.
/// Order is preserved in the destination heat.
pub fn jjrg_restring(gallops: &mut jjrg_Gallops, args: jjrg_RestringArgs) -> Result<jjrg_RestringResult, String> {
    // Parse and normalize firemarks
    let source_firemark = Firemark::jjrf_parse(&args.source_firemark)
        .map_err(|e| format!("Invalid source firemark: {}", e))?;
    let source_firemark_key = source_firemark.jjrf_display();

    let dest_firemark = Firemark::jjrf_parse(&args.dest_firemark)
        .map_err(|e| format!("Invalid destination firemark: {}", e))?;
    let dest_firemark_key = dest_firemark.jjrf_display();

    // Validate firemarks are different
    if source_firemark_key == dest_firemark_key {
        return Err("Cannot draft paces to same heat".to_string());
    }

    // Validate both heats exist
    if !gallops.heats.contains_key(&source_firemark_key) {
        return Err(format!("Heat '{}' not found", source_firemark_key));
    }
    if !gallops.heats.contains_key(&dest_firemark_key) {
        return Err(format!("Heat '{}' not found", dest_firemark_key));
    }

    // Validate coronet array is non-empty
    if args.coronets.is_empty() {
        return Err("No paces specified for draft".to_string());
    }

    // Validate all coronets before any mutations
    let mut normalized_coronets = Vec::new();
    for coronet_str in &args.coronets {
        // Parse and normalize coronet
        let coronet = Coronet::jjrf_parse(coronet_str)
            .map_err(|e| format!("Invalid coronet '{}': {}", coronet_str, e))?;
        let coronet_key = coronet.jjrf_display();

        // Verify coronet belongs to source heat
        let coronet_firemark = coronet.jjrf_parent_firemark();
        if coronet_firemark.jjrf_display() != source_firemark_key {
            return Err(format!(
                "Pace {} does not belong to source heat {}",
                coronet_key, source_firemark_key
            ));
        }

        // Verify pace exists in source heat
        let source_heat = gallops.heats.get(&source_firemark_key).unwrap();
        if !source_heat.paces.contains_key(&coronet_key) {
            return Err(format!(
                "Pace {} not found in heat {}",
                coronet_key, source_firemark_key
            ));
        }

        normalized_coronets.push(coronet_key);
    }

    // All validations passed - now perform the operations

    // Capture source heat info before mutations
    let source_heat = gallops.heats.get(&source_firemark_key).unwrap();
    let source_silks = source_heat.silks.clone();
    let source_paddock = source_heat.paddock_file.clone();

    // Capture destination heat info
    let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
    let dest_silks = dest_heat.silks.clone();
    let dest_paddock = dest_heat.paddock_file.clone();

    // Draft each pace to destination (preserving order)
    let mut mappings = Vec::new();
    for coronet_key in normalized_coronets {
        let draft_args = jjrg_DraftArgs {
            coronet: coronet_key.clone(),
            to: dest_firemark_key.clone(),
            before: None,
            after: None,
            first: false, // Append to end to preserve order
        };

        // Get pace info before drafting (for output)
        let source_heat = gallops.heats.get(&source_firemark_key).unwrap();
        let pace = source_heat.paces.get(&coronet_key).unwrap();
        let first_tack = pace.tacks.first();
        let silks = first_tack.map(|t| t.silks.clone()).unwrap_or_default();
        let state = first_tack.map(|t| t.state.clone()).unwrap_or(jjrg_PaceState::Rough);
        let spec_full = first_tack.map(|t| t.text.clone()).unwrap_or_default();
        let spec_preview = if spec_full.len() > 80 {
            format!("{}...", &spec_full[..77])
        } else {
            spec_full.clone()
        };

        // Perform the draft
        let result = jjrg_draft(gallops, draft_args)?;

        mappings.push(jjrg_RestringMapping {
            old_coronet: coronet_key,
            new_coronet: result.new_coronet,
            silks,
            state,
            spec: spec_preview,
        });
    }

    // Check if source heat is now empty
    let source_heat_after = gallops.heats.get(&source_firemark_key).unwrap();
    let source_empty_after = source_heat_after.paces.is_empty();

    Ok(jjrg_RestringResult {
        source_firemark: source_firemark_key,
        source_silks,
        source_paddock,
        source_empty_after,
        dest_firemark: dest_firemark_key,
        dest_silks,
        dest_paddock,
        drafted: mappings,
    })
}
