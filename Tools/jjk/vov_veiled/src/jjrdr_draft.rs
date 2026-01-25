// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Draft command - move a Pace from one Heat to another
//!
//! Implements the jjx_draft subcommand and related structures.

use clap::Args;
use std::path::PathBuf;

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_read_stdin as read_stdin};
use crate::jjrn_notch::{jjrn_HeatAction as HeatAction, jjrn_format_heat_message as format_heat_message};

/// Arguments for jjx_draft command
#[derive(Args, Debug)]
pub struct jjrdr_DraftArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Pace identity to move (Coronet)
    pub coronet: String,

    /// Destination Heat identity (Firemark)
    #[arg(long)]
    pub to: String,

    /// Insert before specified Coronet in destination
    #[arg(long, conflicts_with_all = ["after", "first"])]
    pub before: Option<String>,

    /// Insert after specified Coronet in destination
    #[arg(long, conflicts_with_all = ["before", "first"])]
    pub after: Option<String>,

    /// Insert at beginning of destination Heat
    #[arg(long, conflicts_with_all = ["before", "after"])]
    pub first: bool,
}

/// Handler function for draft command
pub fn jjrdr_run_draft(args: jjrdr_DraftArgs) -> i32 {
    use crate::jjrg_gallops::jjrg_DraftArgs as LibDraftArgs;

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_draft: error: {}", e);
            return 1;
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_draft: error loading Gallops: {}", e);
            return 1;
        }
    };

    let coronet = args.coronet.clone();
    let to = args.to.clone();
    let draft_args = LibDraftArgs {
        coronet: args.coronet,
        to: args.to,
        before: args.before,
        after: args.after,
        first: args.first,
    };

    match gallops.jjrg_draft(draft_args) {
        Ok(result) => {
            // Save gallops
            if let Err(e) = gallops.jjrg_save(&args.file) {
                eprintln!("jjx_draft: error saving Gallops: {}", e);
                return 1;
            }

            // Commit using machine_commit - draft affects source and dest paddocks
            // Parse both firemarks to get paddock paths
            let src_coronet = crate::jjrf_favor::jjrf_Coronet::jjrf_parse(&coronet)
                .expect("draft given invalid source coronet");
            let src_fm = src_coronet.jjrf_parent_firemark();
            let dest_fm = Firemark::jjrf_parse(&to).expect("draft given invalid destination firemark");

            let gallops_path = args.file.to_string_lossy().to_string();
            let src_paddock_path = format!(".claude/jjm/jjp_{}.md", src_fm.jjrf_as_str());
            let dest_paddock_path = format!(".claude/jjm/jjp_{}.md", dest_fm.jjrf_as_str());

            let commit_args = vvc::vvcm_CommitArgs {
                files: vec![
                    gallops_path,
                    src_paddock_path,
                    dest_paddock_path,
                ],
                message: format_heat_message(&dest_fm, HeatAction::Draft, &format!("{} → {}", coronet, result.new_coronet)),
                size_limit: 50000,
                warn_limit: 30000,
            };

            match vvc::machine_commit(&lock, &commit_args) {
                Ok(hash) => {
                    eprintln!("jjx_draft: committed {}", &hash[..8]);
                }
                Err(e) => {
                    eprintln!("jjx_draft: commit warning: {}", e);
                }
            }

            println!("{}", result.new_coronet);
            0
        }
        Err(e) => {
            eprintln!("jjx_draft: error: {}", e);
            1
        }
    }
    // lock released here
}
