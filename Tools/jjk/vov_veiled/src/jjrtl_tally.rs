// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK Tally commands - pace modification operations
//!
//! Split into four single-purpose commands:
//! - revise_docket: Update pace docket text
//! - arm: Set state to bridled with warrant
//! - relabel: Rename pace silks
//! - drop: Set state to abandoned

use std::path::PathBuf;

use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState, jjrg_read_stdin_optional as read_stdin_optional};
use crate::jjrn_notch::{jjrn_format_heat_message as format_heat_message, jjrn_HeatAction as HeatAction};

/// Arguments for jjx_revise_docket command
#[derive(clap::Args, Debug)]
pub struct jjrtl_ReviseDocketArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the revise_docket command
///
/// Updates the docket text for a pace from stdin.
pub fn jjrtl_run_revise_docket(args: jjrtl_ReviseDocketArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_revise_docket: error: {}", e);
            return 1;
        }
    };

    let text = match read_stdin_optional() {
        Ok(Some(t)) => t,
        Ok(None) => {
            eprintln!("jjx_revise_docket: error: docket text required via stdin");
            return 1;
        }
        Err(e) => {
            eprintln!("jjx_revise_docket: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_revise_docket: error loading Gallops: {}", e);
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
            eprintln!("jjx_revise_docket: error: {}", e);
            return 1;
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state: None,
        direction: None,
        text: Some(text),
        silks: None,
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    println!("committed {}", hash);
                    0
                }
                Err(e) => {
                    eprintln!("jjx_revise_docket: error: {}", e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("jjx_revise_docket: error: {}", e);
            1
        }
    }
    // lock released here
}

/// Arguments for jjx_arm command
#[derive(clap::Args, Debug)]
pub struct jjrtl_ArmArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the arm command
///
/// Sets pace state to bridled with warrant from stdin.
/// Creates both a tally commit and a B commit.
pub fn jjrtl_run_arm(args: jjrtl_ArmArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_arm: error: {}", e);
            return 1;
        }
    };

    let warrant = match read_stdin_optional() {
        Ok(Some(w)) => w,
        Ok(None) => {
            eprintln!("jjx_arm: error: warrant text required via stdin");
            return 1;
        }
        Err(e) => {
            eprintln!("jjx_arm: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_arm: error loading Gallops: {}", e);
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
            eprintln!("jjx_arm: error: {}", e);
            return 1;
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state: Some(PaceState::Bridled),
        direction: Some(warrant.clone()),
        text: None,
        silks: None,
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    println!("committed {}", hash);
                }
                Err(e) => {
                    eprintln!("jjx_arm: error: {}", e);
                    return 1;
                }
            }

            // Create B commit for bridling
            let coronet = match Coronet::jjrf_parse(&coronet_str) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("jjx_arm: error parsing coronet for B commit: {}", e);
                    return 1;
                }
            };

            let agent = "arm"; // Agent that bridled the pace

            // Build B commit message
            use crate::jjrn_notch::jjrn_format_bridle_message;
            let b_subject = jjrn_format_bridle_message(&coronet, agent, &silks);
            let b_message = format!("{}\n\n{}", b_subject, warrant);

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
                    eprintln!("jjx_arm: error creating B commit: {}", stderr);
                    return 1;
                }
                Err(e) => {
                    eprintln!("jjx_arm: error creating B commit: {}", e);
                    return 1;
                }
            }

            eprintln!();
            eprintln!("Recommended: /jjc-heat-mount {} to execute", fm.jjrf_as_str());
            0
        }
        Err(e) => {
            eprintln!("jjx_arm: error: {}", e);
            1
        }
    }
    // lock released here
}

/// Arguments for jjx_relabel command
#[derive(clap::Args, Debug)]
pub struct jjrtl_RelabelArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,

    /// New silks (kebab-case display name)
    #[arg(long)]
    pub silks: String,
}

/// Run the relabel command
///
/// Renames the pace silks.
pub fn jjrtl_run_relabel(args: jjrtl_RelabelArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_relabel: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_relabel: error loading Gallops: {}", e);
            return 1;
        }
    };

    // Get firemark for commit message before we move args
    let coronet_str = args.coronet.clone();
    let new_silks = args.silks.clone();
    let fm = match Coronet::jjrf_parse(&coronet_str) {
        Ok(c) => c.jjrf_parent_firemark(),
        Err(e) => {
            eprintln!("jjx_relabel: error: {}", e);
            return 1;
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state: None,
        direction: None,
        text: None,
        silks: Some(args.silks),
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &new_silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    println!("committed {}", hash);
                    0
                }
                Err(e) => {
                    eprintln!("jjx_relabel: error: {}", e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("jjx_relabel: error: {}", e);
            1
        }
    }
    // lock released here
}

/// Arguments for jjx_drop command
#[derive(clap::Args, Debug)]
pub struct jjrtl_DropArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the drop command
///
/// Sets pace state to abandoned.
pub fn jjrtl_run_drop(args: jjrtl_DropArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_drop: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_drop: error loading Gallops: {}", e);
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
            eprintln!("jjx_drop: error: {}", e);
            return 1;
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet,
        state: Some(PaceState::Abandoned),
        direction: None,
        text: None,
        silks: None,
    };

    match gallops.jjrg_tally(tally_args) {
        Ok(()) => {
            let message = format_heat_message(&fm, HeatAction::Tally, &silks);

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000) {
                Ok(hash) => {
                    println!("committed {}", hash);
                    0
                }
                Err(e) => {
                    eprintln!("jjx_drop: error: {}", e);
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("jjx_drop: error: {}", e);
            1
        }
    }
    // lock released here
}
