// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK Tally command - add a new Tack to a Pace
//!
//! Handles pace state transitions (rough → bridled → complete/abandoned)
//! and plan refinement through tack prepending.

use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState, jjrg_read_stdin_optional as read_stdin_optional};
use crate::jjrn_notch::{jjrn_format_heat_message as format_heat_message, jjrn_HeatAction as HeatAction};

/// Arguments for jjx_tally command
#[derive(clap::Args, Debug)]
pub struct jjrtl_TallyArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,

    /// Target state (rough, bridled, complete, abandoned)
    #[arg(long)]
    pub state: Option<String>,

    /// Execution guidance (required if state is bridled)
    #[arg(long, short = 'd')]
    pub direction: Option<String>,

    /// Kebab-case display name (if provided, new Tack uses this value; otherwise inherits)
    #[arg(long, short = 's')]
    pub silks: Option<String>,
}

/// Run the tally command
///
/// Adds a new Tack to a Pace with optional state transition and refinement.
/// If state transitions to bridled, creates an additional B commit.
pub fn jjrtl_run_tally(args: jjrtl_TallyArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    let text = match read_stdin_optional() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    let state = match &args.state {
        Some(s) => match s.to_lowercase().as_str() {
            "rough" => Some(PaceState::Rough),
            "bridled" => Some(PaceState::Bridled),
            "complete" => Some(PaceState::Complete),
            "abandoned" => Some(PaceState::Abandoned),
            _ => {
                eprintln!("jjx_tally: error: invalid state '{}', must be rough, bridled, complete, or abandoned", s);
                return 1;
            }
        },
        None => None,
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_tally: error loading Gallops: {}", e);
            return 1;
        }
    };

    // Get firemark and silks for commit message before we move args
    let coronet_str = args.coronet.clone();
    let (fm, silks) = match Coronet::jjrf_parse(&coronet_str) {
        Ok(c) => {
            let parent_fm = c.jjrf_parent_firemark();
            let silks = gallops.heats.get(&parent_fm.jjrf_display())
                .and_then(|h| h.paces.get(&c.jjrf_display()))
                .and_then(|p| p.tacks.first().map(|t| t.silks.clone()))
                .unwrap_or_else(|| coronet_str.clone());
            (parent_fm, silks)
        }
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            return 1;
        }
    };

    // Capture whether we're bridling before state is moved
    let is_bridling = matches!(state, Some(PaceState::Bridled));

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state,
        direction: args.direction,
        text,
        silks: args.silks,
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    println!("committed {}", hash);
                }
                Err(e) => {
                    eprintln!("jjx_tally: error: {}", e);
                    return 1;
                }
            }

            // If state transitioned to bridled, create B commit
            if is_bridling {
                // Parse coronet to get the full identity
                let coronet = match Coronet::jjrf_parse(&coronet_str) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("jjx_tally: error parsing coronet for B commit: {}", e);
                        return 1;
                    }
                };

                // Get direction from the new tack
                let direction_text = match gallops.heats.get(&fm.jjrf_display())
                    .and_then(|h| h.paces.get(&coronet.jjrf_display()))
                    .and_then(|p| p.tacks.first())
                    .and_then(|t| t.direction.as_ref()) {
                    Some(d) => d.clone(),
                    None => {
                        String::new()
                    }
                };

                let agent = "tally"; // Agent that bridled the pace

                // Build B commit message
                use crate::jjrn_notch::jjrn_format_bridle_message;
                let b_subject = jjrn_format_bridle_message(&coronet, agent, &silks);
                let b_message = format!("{}\n\n{}", b_subject, direction_text);

                // Create empty B commit using git directly (like chalk markers)
                use std::process::Command;
                let output = Command::new("git")
                    .args(["commit", "--allow-empty", "-m", &b_message])
                    .output();

                match output {
                    Ok(result) if result.status.success() => {
                        // Get commit hash
                        let hash_output = Command::new("git")
                            .args(["rev-parse", "HEAD"])
                            .output();

                        if let Ok(hash_result) = hash_output {
                            if hash_result.status.success() {
                                let hash = String::from_utf8_lossy(&hash_result.stdout).trim().to_string();
                                println!("B commit {}", &hash[..8.min(hash.len())]);
                            }
                        }
                    }
                    Ok(result) => {
                        let stderr = String::from_utf8_lossy(&result.stderr);
                        eprintln!("jjx_tally: error creating B commit: {}", stderr);
                        return 1;
                    }
                    Err(e) => {
                        eprintln!("jjx_tally: error creating B commit: {}", e);
                        return 1;
                    }
                }
            }

            0
        }
        Err(e) => {
            eprintln!("jjx_tally: error: {}", e);
            1
        }
    }
    // lock released here
}
