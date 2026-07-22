// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Gallops write operations
//!
//! All mutation operations on Gallops: nominate, slate, rail, tally, draft, retire, furlough.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use crate::jjrf_favor::{jjrf_Firemark, jjrf_Coronet, JJRF_FIREMARK_PREFIX, JJRF_CORONET_PREFIX};
use crate::jjri_io::jjri_paddock_path;
use crate::jjrpd_parade::{jjrpd_format_file_census, jjrpd_format_commit_swimlanes, JJRPD_CENSUS_EVERY_FILE};
use crate::jjrs_steeplechase::jjrs_SteeplechaseEntry;
use crate::jjrt_types::*;
use crate::jjru_util::{zjjrg_increment_seed, jjrg_make_tack};
use crate::jjrv_validate::{zjjrg_is_kebab_case, zjjrg_is_yymmdd};

/// Nominate a new Heat
///
/// Creates a new Heat with empty Pace structure and creates the paddock file.
pub fn jjrg_nominate(gallops: &mut jjrg_Gallops, args: jjrg_NominateArgs, base_path: &Path) -> Result<jjrg_NominateResult, String> {
    // Compose the two halves into today's behavior: derive-and-insert (no fs),
    // then apply the fs tail. The paddock-template builder lives once in
    // jjrg_nominate_excise, so the seam-on path (which drives excise against the
    // studbook tip and applies the tail only after the journal lands) can never
    // drift from this one.
    let plan = jjrg_nominate_excise(gallops, args)?;
    jjrg_nominate_apply(base_path, &plan)?;

    Ok(jjrg_NominateResult { firemark: plan.firemark_str })
}

/// The pure plan a nominate produces before any filesystem write: the paddock
/// template content already derived and the heat already inserted into the
/// gallops, but nothing on disk yet. Split out so the studbook write seam
/// derives against the LOCKED TIP and mutates under the lock (which is also
/// where next_heat_seed is allocated from — Shape B), then applies the fs tail
/// (`jjrg_nominate_apply`) only AFTER the journal lands — no orphan-paddock
/// window, because a journal reject leaves the disk untouched.
pub struct jjrg_NominatePlan {
    pub firemark_str: String,
    pub paddock_rel_path: String,
    pub paddock_content: String,
    pub silks: String,
}

/// Validate and insert the new heat — pure of the filesystem. Allocates the
/// Firemark from `next_heat_seed` (the studbook tip's, seam-on), builds the
/// paddock template content, inserts the `jjrg_Heat`, and advances
/// `next_heat_seed`. The paddock is only planned here; writing it is
/// `jjrg_nominate_apply`'s job.
pub fn jjrg_nominate_excise(gallops: &mut jjrg_Gallops, args: jjrg_NominateArgs) -> Result<jjrg_NominatePlan, String> {
    // Validate silks is alphanumeric-kebab
    if !zjjrg_is_kebab_case(&args.silks) {
        return Err(format!("silks must be non-empty alphanumeric-kebab (letters, digits, hyphens), got '{}'", args.silks));
    }

    // Validate created is YYMMDD
    if !zjjrg_is_yymmdd(&args.created) {
        return Err(format!("created must be YYMMDD format, got '{}'", args.created));
    }

    // Allocate Firemark from next_heat_seed
    let firemark_str = format!("{}{}", JJRF_FIREMARK_PREFIX, gallops.next_heat_seed);
    let heat_id = gallops.next_heat_seed.clone();

    let paddock_rel_path = jjri_paddock_path(&heat_id);
    let paddock_content = format!(
        "# Paddock: {}\n\n## Context\n\n(Describe the initiative's background and goals)\n\n## References\n\n(List relevant files, docs, or prior work)\n",
        args.silks
    );

    // Create new Heat
    let heat = jjrg_Heat {
        silks: args.silks.clone(),
        creation_time: args.created,
        status: jjrg_HeatStatus::Stabled,
        order: Vec::new(),
        paces: BTreeMap::new(),
    };

    // Insert Heat
    gallops.heats.insert(firemark_str.clone(), heat);
    gallops.heat_order.push(firemark_str.clone());

    // Increment next_heat_seed
    gallops.next_heat_seed = zjjrg_increment_seed(&gallops.next_heat_seed);

    Ok(jjrg_NominatePlan {
        firemark_str,
        paddock_rel_path,
        paddock_content,
        silks: args.silks,
    })
}

/// The filesystem tail of a nominate: write the paddock template under
/// `base_path`. Runs inline seam-off; seam-on only AFTER the journal has landed
/// the heat insertion to the studbook, so a journal reject leaves nothing on
/// disk to reverse.
pub fn jjrg_nominate_apply(base_path: &Path, plan: &jjrg_NominatePlan) -> Result<(), String> {
    let paddock_path = base_path.join(&plan.paddock_rel_path);
    if let Some(parent) = paddock_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create paddock directory: {}", e))?;
    }

    fs::write(&paddock_path, &plan.paddock_content)
        .map_err(|e| format!("Failed to write paddock file: {}", e))?;

    Ok(())
}

/// Index of the first actionable pace in a heat's order — the slot `--first`
/// aims at, per the JJS0 `jjda_first` contract. `None` when the heat holds no
/// actionable pace; callers then position at the end.
///
/// One home for the rule, shared by slate and rail: the head of `order` is
/// history (wrapped and abandoned paces stay in place), so aiming at index 0
/// buries a new pace above the record rather than at the head of the work
/// remaining.
fn zjjrg_first_actionable_idx(heat: &jjrg_Heat) -> Option<usize> {
    heat.order.iter().position(|coronet| {
        heat.paces.get(coronet)
            .and_then(|pace| pace.tacks.first())
            .is_some_and(|tack| !tack.state.jjrg_is_resolved())
    })
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
    let firemark = jjrf_Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let firemark_key = firemark.jjrf_display();

    // Capture the global pace seed before borrowing the heat: the Coronet is
    // minted from the single gallops-wide seed (JJS0 jjdgm_pace_seed), not
    // per-heat, and advanced once below after the heat borrow ends.
    let global_pace_seed = gallops.next_pace_seed.clone();

    // Verify Heat exists
    let heat = gallops.heats.get_mut(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    // If --before or --after specified, validate target coronet exists
    let insert_position = if let Some(ref before_str) = args.before {
        let target = jjrf_Coronet::jjrf_parse(before_str)
            .map_err(|e| format!("Invalid --before coronet: {}", e))?;
        let target_key = target.jjrf_display();
        let pos = heat.order.iter().position(|c| c == &target_key)
            .ok_or_else(|| format!("Target coronet '{}' not found in heat", target_key))?;
        Some(pos) // Insert before this position
    } else if let Some(ref after_str) = args.after {
        let target = jjrf_Coronet::jjrf_parse(after_str)
            .map_err(|e| format!("Invalid --after coronet: {}", e))?;
        let target_key = target.jjrf_display();
        let pos = heat.order.iter().position(|c| c == &target_key)
            .ok_or_else(|| format!("Target coronet '{}' not found in heat", target_key))?;
        Some(pos + 1) // Insert after this position
    } else if args.first {
        Some(zjjrg_first_actionable_idx(heat).unwrap_or(heat.order.len()))
    } else {
        None // Append to end (default)
    };

    // Construct Coronet from the single global pace seed — a flat 5-char id, no
    // embedded heat (JJS0 jjdt_coronet).
    let coronet_str = format!("{}{}", JJRF_CORONET_PREFIX, global_pace_seed);

    // Create initial Tack and Pace. The original-intent capture (dictation,
    // precis, slated) is frozen here, once — no other path writes these fields.
    let tack = jjrg_make_tack(
        jjrg_PaceState::Rough,
        args.text,
        args.silks,
    );
    let pace = jjrg_Pace {
        tacks: vec![tack],
        dictation: args.dictation,
        precis: args.precis,
        slated: Some(args.slated),
        redocket_count: 0,
    };

    // Insert into order at determined position
    match insert_position {
        Some(pos) => heat.order.insert(pos, coronet_str.clone()),
        None => heat.order.push(coronet_str.clone()),
    }
    heat.paces.insert(coronet_str.clone(), pace);

    // Advance the global pace seed (the heat borrow has ended above).
    gallops.next_pace_seed = zjjrg_increment_seed(&gallops.next_pace_seed);

    Ok(jjrg_SlateResult { coronet: coronet_str })
}

/// Rail - reorder Paces within a Heat
///
/// Supports two modes:
/// - Order mode: replace entire sequence with provided order array
/// - Move mode: relocate a single pace using positioning flags
pub fn jjrg_rail(gallops: &mut jjrg_Gallops, args: jjrg_RailArgs) -> Result<Vec<String>, String> {
    // Parse and normalize firemark
    let firemark = jjrf_Firemark::jjrf_parse(&args.firemark)
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
        let move_coronet = jjrf_Coronet::jjrf_parse(move_str)
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
            // Use len() as fallback so adjustment logic works correctly for "end" position
            let target_idx = zjjrg_first_actionable_idx(heat).unwrap_or(heat.order.len());
            // If moving from before target, the target shifts down after removal
            if current_pos < target_idx {
                target_idx - 1
            } else {
                target_idx
            }
        } else if args.last {
            heat.order.len() - 1
        } else if let Some(ref before_str) = args.before {
            let target = jjrf_Coronet::jjrf_parse(before_str)
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
            let target = jjrf_Coronet::jjrf_parse(after_str)
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
        // Order mode has been removed
        if !args.order.is_empty() {
            return Err("Order mode removed. Use --move to reposition individual paces.".to_string());
        }
        return Err("No --move specified. Use --move CORONET with a positioning flag (--first, --last, --before, --after).".to_string());
    }

    // Return the new order
    Ok(heat.order.clone())
}

/// Tally - add a new Tack to a Pace
///
/// Prepends a new Tack with state transition and/or plan refinement.
pub fn jjrg_tally(gallops: &mut jjrg_Gallops, args: jjrg_TallyArgs) -> Result<(), String> {
    // Parse and normalize coronet
    let coronet = jjrf_Coronet::jjrf_parse(&args.coronet)
        .map_err(|e| format!("Invalid coronet: {}", e))?;
    let coronet_key = coronet.jjrf_display();

    // Resolve the harbouring heat by paces-scan — a Coronet embeds no
    // affiliation (JJS0 jjdt_coronet Resolution).
    let firemark_key = gallops.jjrg_heat_key_of_coronet(&coronet_key)
        .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;

    // Verify Heat exists
    let heat = gallops.heats.get_mut(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    // Verify Pace exists
    let pace = heat.paces.get_mut(&coronet_key)
        .ok_or_else(|| format!("Pace '{}' not found", coronet_key))?;

    // Read current Tack
    let current_tack = pace.tacks.first()
        .ok_or_else(|| "Pace has no tacks (should never happen)".to_string())?;

    // Determine new state — explicit override, else inherit the current state
    let new_state = args.state.clone().unwrap_or_else(|| current_tack.state.clone());

    // Determine new text
    let new_text = args.text.unwrap_or_else(|| jjrg_lines_to_text(&current_tack.text));
    if new_text.is_empty() {
        return Err("text must not be empty".to_string());
    }

    // Determine new silks
    let new_silks = args.silks.unwrap_or_else(|| current_tack.silks.clone());

    // Designation carries through tally: relabel reverts nothing, and close
    // (wrap → complete) and drop (→ abandoned) persist tier/effort as
    // provenance. The revert triggers (redocket, transfer, relocate) run on
    // their own paths (jjrg_revise_docket, jjrg_draft), not through tally.
    let (carry_tier, carry_effort) = (current_tack.tier, current_tack.effort);

    // Create new Tack
    let mut new_tack = jjrg_make_tack(
        new_state,
        new_text,
        new_silks,
    );
    new_tack.tier = carry_tier;
    new_tack.effort = carry_effort;

    // Replace the pace's single current tack (tack evolution lives in git)
    pace.tacks = vec![new_tack];

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
    let source_coronet = jjrf_Coronet::jjrf_parse(&args.coronet)
        .map_err(|e| format!("Invalid coronet: {}", e))?;
    let source_coronet_key = source_coronet.jjrf_display();

    // Resolve the source heat by paces-scan — a Coronet embeds no affiliation
    // (JJS0 jjdt_coronet Resolution); this both locates the source and verifies
    // the pace exists.
    let source_firemark_key = gallops.jjrg_heat_key_of_coronet(&source_coronet_key)
        .ok_or_else(|| format!("Pace {} not found in any heat", source_coronet_key))?;

    // Parse and normalize destination firemark
    let dest_firemark = jjrf_Firemark::jjrf_parse(&args.to)
        .map_err(|e| format!("Invalid destination firemark: {}", e))?;
    let dest_firemark_key = dest_firemark.jjrf_display();

    // Validate source and destination are different
    if source_firemark_key == dest_firemark_key {
        return Err("Cannot draft pace to same heat".to_string());
    }

    // Verify destination heat exists
    if !gallops.heats.contains_key(&dest_firemark_key) {
        return Err(format!("Heat '{}' not found", dest_firemark_key));
    }

    // Validate positioning target if specified
    let insert_position = if let Some(ref before_str) = args.before {
        let target = jjrf_Coronet::jjrf_parse(before_str)
            .map_err(|e| format!("Invalid --before coronet: {}", e))?;
        let target_key = target.jjrf_display();
        let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
        let pos = dest_heat.order.iter().position(|c| c == &target_key)
            .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, dest_firemark_key))?;
        Some(pos)
    } else if let Some(ref after_str) = args.after {
        let target = jjrf_Coronet::jjrf_parse(after_str)
            .map_err(|e| format!("Invalid --after coronet: {}", e))?;
        let target_key = target.jjrf_display();
        let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
        let pos = dest_heat.order.iter().position(|c| c == &target_key)
            .ok_or_else(|| format!("Target pace {} not found in heat {}", target_key, dest_firemark_key))?;
        Some(pos + 1)
    } else if args.first {
        let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
        Some(zjjrg_first_actionable_idx(dest_heat).unwrap_or(dest_heat.order.len()))
    } else {
        None // Append to end
    };

    // Remove pace from source heat
    let source_heat = gallops.heats.get_mut(&source_firemark_key).unwrap();
    let mut pace_data = source_heat.paces.remove(&source_coronet_key)
        .ok_or_else(|| format!("Pace {} not found", source_coronet_key))?;
    source_heat.order.retain(|c| c != &source_coronet_key);

    // Re-affiliation without re-keying (JJS0 jjdt_coronet): the pace moves into
    // the destination heat under its SAME immutable Coronet. Its tack data is
    // carried unchanged — except the bridle revert below.
    //
    // Revert trigger: a designation is void when its judgment inputs change — the
    // paddock context this pace was judged against is gone, so a bridled pace
    // demotes to rough with tier and effort wiped. A non-bridled pace's tack is
    // preserved verbatim (resolved states keep their designation as provenance).
    let was_bridled = pace_data.tacks.first()
        .is_some_and(|t| t.state == jjrg_PaceState::Bridled);
    if was_bridled {
        let prior = pace_data.tacks.first().expect("bridled implies a tack");
        let reverted = jjrg_make_tack(
            jjrg_PaceState::Rough,
            jjrg_lines_to_text(&prior.text),
            prior.silks.clone(),
        );
        pace_data.tacks = vec![reverted];
    }

    // Insert into destination heat under the unchanged Coronet. Moving pace_data
    // whole carries the original-intent capture (dictation/precis/slated) and the
    // redocket_count forward untouched — a relocation never re-freezes or
    // increments (BcAAK's ₢BcAAK contract, satisfied by the move-under-same-key of
    // BcAAO's re-gestalt without the retired re-key/draft-note reconstruction).
    let dest_heat = gallops.heats.get_mut(&dest_firemark_key).unwrap();
    match insert_position {
        Some(pos) => dest_heat.order.insert(pos, source_coronet_key.clone()),
        None => dest_heat.order.push(source_coronet_key.clone()),
    }
    dest_heat.paces.insert(source_coronet_key.clone(), pace_data);

    Ok(jjrg_DraftResult { new_coronet: source_coronet_key })
}

/// Retire a Heat
///
/// Creates trophy file, removes heat from gallops, deletes paddock file.
/// Does NOT save gallops or commit - caller is responsible for that.
pub fn jjrg_retire(
    gallops: &mut jjrg_Gallops,
    args: jjrg_RetireArgs,
    base_path: &Path,
    steeplechase: &[jjrs_SteeplechaseEntry],
) -> Result<jjrg_RetireResult, String> {
    // Compose the two halves into today's behavior: read the paddock (consumer
    // fs), derive-and-excise (no fs), then apply the fs tail. The content builder
    // and filename format live once in jjrg_retire_excise, so the seam-on path
    // (which drives excise against the studbook tip and applies the tail only
    // after the journal lands) can never drift from this one.
    let firemark = jjrf_Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let paddock_file = jjri_paddock_path(firemark.jjrf_as_str());
    let paddock_content = fs::read_to_string(base_path.join(&paddock_file))
        .map_err(|e| format!("Failed to read paddock file '{}': {}", paddock_file, e))?;

    let plan = jjrg_retire_excise(gallops, &args, &paddock_content, steeplechase)?;
    jjrg_retire_apply(base_path, &plan)?;

    Ok(jjrg_RetireResult {
        trophy_path: plan.trophy_rel_path,
        paddock_path: plan.paddock_path,
        silks: plan.silks,
        firemark: plan.firemark_key,
    })
}

/// The pure plan a retire produces before any filesystem write: the trophy
/// content already derived and the heat already excised from the gallops, but
/// nothing on disk yet. Split out so the studbook write seam derives against the
/// LOCKED TIP and mutates under the lock, then applies the fs tail
/// (`jjrg_retire_apply`) only AFTER the journal lands — no orphan-trophy rollback
/// window, because a journal reject leaves the disk untouched.
pub struct jjrg_RetirePlan {
    pub firemark_key: String,
    pub trophy_rel_path: String,
    pub trophy_content: String,
    pub paddock_path: String,
    pub silks: String,
}

/// Derive the trophy and excise the heat — pure of the filesystem. Verifies the
/// heat exists (its absence is the caller's decline signal: seam-on, a vanished
/// heat means another station retired it under the lock — Shape B declining
/// exactly as intended), builds the trophy content from THIS gallops (the studbook
/// tip, seam-on), computes the trophy filename from this heat's own
/// creation_time/silks (silks drift via jjx_alter, so a session-derived name could
/// be stale — the tip's is authoritative), then removes the heat (next_heat_seed
/// untouched). `paddock_content` is read by the caller from the consumer fs — the
/// paddock is never a studbook tenant — so this function stays fs-free.
pub fn jjrg_retire_excise(
    gallops: &mut jjrg_Gallops,
    args: &jjrg_RetireArgs,
    paddock_content: &str,
    steeplechase: &[jjrs_SteeplechaseEntry],
) -> Result<jjrg_RetirePlan, String> {
    let firemark = jjrf_Firemark::jjrf_parse(&args.firemark)
        .map_err(|e| format!("Invalid firemark: {}", e))?;
    let firemark_key = firemark.jjrf_display();

    if !zjjrg_is_yymmdd(&args.today) {
        return Err(format!("today must be YYMMDD format, got '{}'", args.today));
    }

    let heat = gallops.heats.get(&firemark_key)
        .ok_or_else(|| format!("Heat '{}' not found", firemark_key))?;

    let trophy_content = zjjrg_build_trophy_content(&firemark_key, heat, paddock_content, &args.today, steeplechase)?;
    let trophy_filename = format!(
        "jjh_b{}-r{}-{}.md",
        heat.creation_time,
        args.today,
        heat.silks
    );
    let trophy_rel_path = format!(".claude/jjm/retired/{}", trophy_filename);
    let silks = heat.silks.clone();
    let paddock_path = jjri_paddock_path(firemark.jjrf_as_str());

    gallops.heats.remove(&firemark_key);
    gallops.heat_order.retain(|f| f != &firemark_key);

    Ok(jjrg_RetirePlan {
        firemark_key,
        trophy_rel_path,
        trophy_content,
        paddock_path,
        silks,
    })
}

/// The filesystem tail of a retire: write the trophy and delete the paddock,
/// under `base_path`. Runs inline seam-off; seam-on only AFTER the journal has
/// landed the excision to the studbook, so a journal reject leaves nothing on
/// disk to reverse.
pub fn jjrg_retire_apply(base_path: &Path, plan: &jjrg_RetirePlan) -> Result<(), String> {
    let trophy_full_path = base_path.join(&plan.trophy_rel_path);
    if let Some(parent) = trophy_full_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create retired directory: {}", e))?;
    }
    fs::write(&trophy_full_path, &plan.trophy_content)
        .map_err(|e| format!("Failed to write trophy file: {}", e))?;

    let paddock_full = base_path.join(&plan.paddock_path);
    if paddock_full.exists() {
        fs::remove_file(&paddock_full)
            .map_err(|e| format!("Failed to delete paddock file: {}", e))?;
    }
    Ok(())
}

/// Build trophy markdown preview (dry-run, no file modifications)
///
/// Returns the markdown content that would be written to the trophy file.
pub fn jjrg_build_trophy_preview(
    gallops: &jjrg_Gallops,
    firemark: &str,
    paddock_content: &str,
    today: &str,
    steeplechase: &[jjrs_SteeplechaseEntry],
) -> Result<String, String> {
    // Parse and normalize firemark
    let fm = jjrf_Firemark::jjrf_parse(firemark)
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
    steeplechase: &[jjrs_SteeplechaseEntry],
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
                .map(|t| t.state.jjrg_as_str())
                .unwrap_or("unknown");

            let pace_silks = pace.tacks.first()
                .map(|t| t.silks.as_str())
                .unwrap_or("unknown");

            content.push_str(&format!(
                "### {} ({}) [{}]\n\n",
                pace_silks, coronet_key, final_state
            ));

            // The pace's single current tack (tack history lives in git)
            for tack in &pace.tacks {
                let state_str = tack.state.jjrg_as_str();
                content.push_str(&format!("**[{}] {}**\n\n", tack.ts, state_str));
                let body = jjrg_lines_to_text(&tack.text);
                content.push_str(&body);
                if !body.ends_with('\n') {
                    content.push('\n');
                }
                content.push('\n');
            }
        }
    }

    // Commit Activity. The trophy is an archive, so its census keeps every
    // touched file — the planning floor that groom and parade apply would drop
    // the lone-pace files from the heat's final account of itself.
    let firemark = jjrf_Firemark::jjrf_parse(firemark_key)
        .map_err(|e| format!("Invalid firemark in trophy builder: {}", e))?;
    content.push_str("## Commit Activity\n\n");
    content.push_str("```\n");
    if let Ok(census) = jjrpd_format_file_census(&firemark, heat, JJRPD_CENSUS_EVERY_FILE) {
        content.push_str(&census);
    }
    if let Ok(swimlanes) = jjrpd_format_commit_swimlanes(&firemark, heat) {
        content.push_str(&swimlanes);
    }
    content.push_str("```\n\n");

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

/// Curry-apply - write Heat paddock content as an in-memory-phase side effect.
///
/// The pure-transform half of the old self-committing `jjrg_curry`: verifies the
/// heat exists in the already-loaded gallops and writes the paddock file. It does
/// NOT load, lock, or commit — the shared dispatch lifecycle (jjri_persist, which
/// co-commits `[gallops, paddock]` under the heat firemark) owns persistence, so a
/// paddock revision folds into the same single commit as any batched reslate/slate.
pub fn jjrg_curry_apply(
    gallops: &jjrg_Gallops,
    firemark: &jjrf_Firemark,
    new_content: &str,
) -> Result<(), String> {
    // Wipe backstop: a paddock is born non-empty (jjrg_nominate seeds the
    // template) and a revision always carries full replacement content, so an
    // empty write is a mis-staged gazette, never intent. The gazette parsers
    // reject empty-bodied notices upstream; this funnel guard holds for every
    // caller.
    if new_content.trim().is_empty() {
        return Err(format!(
            "refusing to write an empty paddock for '{}' — a revision replaces the whole paddock, never blanks it",
            firemark.jjrf_display()
        ));
    }
    let firemark_key = firemark.jjrf_display();
    if !gallops.heats.contains_key(&firemark_key) {
        return Err(format!("Heat '{}' not found", firemark_key));
    }

    let paddock_path_string = jjri_paddock_path(firemark.jjrf_as_str());
    fs::write(&paddock_path_string, new_content)
        .map_err(|e| format!("Failed to write paddock file: {}", e))?;
    Ok(())
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
    let firemark = jjrf_Firemark::jjrf_parse(&args.firemark)
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

        // Note: requesting the same status is allowed (idempotent).
    }

    // Apply status change if requested, then reorder to front of target status group
    let target_status = if args.racing {
        gallops.heats.get_mut(&firemark_key).unwrap().status = jjrg_HeatStatus::Racing;
        Some(jjrg_HeatStatus::Racing)
    } else if args.stabled {
        gallops.heats.get_mut(&firemark_key).unwrap().status = jjrg_HeatStatus::Stabled;
        Some(jjrg_HeatStatus::Stabled)
    } else {
        None
    };

    // Reorder: move heat to just before the first heat with the target status
    if let Some(target) = target_status {
        gallops.heat_order.retain(|f| f != &firemark_key);
        let insert_idx = gallops.heat_order.iter().position(|f| {
            gallops.heats.get(f).map_or(false, |h| h.status == target)
        });
        match insert_idx {
            Some(idx) => gallops.heat_order.insert(idx, firemark_key.clone()),
            None => gallops.heat_order.push(firemark_key.clone()),
        }
    }

    // Apply silks change if requested
    if let Some(silks) = args.silks {
        gallops.heats.get_mut(&firemark_key).unwrap().silks = silks;
    }

    Ok(())
}

/// Restring - bulk draft multiple paces atomically
///
/// Moves multiple paces from source heat to destination heat in a single operation.
/// All paces are validated before any mutations occur.
/// Order is preserved in the destination heat.
pub fn jjrg_restring(gallops: &mut jjrg_Gallops, args: jjrg_RestringArgs) -> Result<jjrg_RestringResult, String> {
    // Parse and normalize firemarks
    let source_firemark = jjrf_Firemark::jjrf_parse(&args.source_firemark)
        .map_err(|e| format!("Invalid source firemark: {}", e))?;
    let source_firemark_key = source_firemark.jjrf_display();

    let dest_firemark = jjrf_Firemark::jjrf_parse(&args.dest_firemark)
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
        let coronet = jjrf_Coronet::jjrf_parse(coronet_str)
            .map_err(|e| format!("Invalid coronet '{}': {}", coronet_str, e))?;
        let coronet_key = coronet.jjrf_display();

        // Verify the pace is in the source heat's paces (JJS0 jjdt_coronet: a
        // Coronet embeds no affiliation, so source membership is the check — the
        // retired heat-embedding validation is gone).
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
    let source_paddock = jjri_paddock_path(source_firemark.jjrf_as_str());

    // Capture destination heat info
    let dest_heat = gallops.heats.get(&dest_firemark_key).unwrap();
    let dest_silks = dest_heat.silks.clone();
    let dest_paddock = jjri_paddock_path(dest_firemark.jjrf_as_str());

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
        let spec_full = first_tack.map(|t| jjrg_lines_to_text(&t.text)).unwrap_or_default();
        const SPEC_PREVIEW_WIDTH: usize = 80;
        const ELLIPSIS: &str = "...";
        let spec_preview = if spec_full.len() > SPEC_PREVIEW_WIDTH {
            format!("{}{}", &spec_full[..SPEC_PREVIEW_WIDTH - ELLIPSIS.len()], ELLIPSIS)
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
