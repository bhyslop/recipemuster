// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Notch command - JJ-aware commit with heat/pace context prefix
//!
//! This module handles the jjx_notch command which performs a staged commit
//! with heat/pace context prefix. It accepts explicit file list and generates
//! a JJ context-aware commit message.

use clap::Args;
use std::collections::HashSet;
use std::process::Command;

use crate::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark, JJRF_FIREMARK_PREFIX, JJRF_CORONET_PREFIX};
use crate::jjrn_notch::{jjrn_format_notch_prefix, JJRN_COMMIT_PREFIX};

/// Arguments for jjx_notch command
#[derive(Args, Debug)]
pub struct jjrnc_NotchArgs {
    /// Identity: Coronet (5-char, pace-affiliated) or Firemark (2-char, heat-only)
    pub identity: String,

    /// Files to commit (required, at least one)
    #[arg(required = true)]
    pub files: Vec<String>,

    /// Size limit in bytes (overrides default 50KB guard)
    #[arg(long)]
    pub size_limit: Option<u64>,

    /// User-provided commit message intent (overrides haiku-generated message)
    #[arg(long)]
    pub intent: Option<String>,
}

/// Run the notch command
///
/// Stages specified files and commits with JJ-aware prefix.
/// Supports both pace-affiliated (Coronet) and heat-only (Firemark) commits.
pub fn jjrnc_run_notch(args: jjrnc_NotchArgs) -> i32 {
    // Require non-empty files list
    if args.files.is_empty() {
        eprintln!("jjx_notch: error: at least one file required");
        return 1;
    }

    // Check each file either exists on disk or is tracked by git (for deletions)
    for file in &args.files {
        let path_exists = std::path::Path::new(file).exists();

        // If file doesn't exist on disk, check if it's tracked by git
        if !path_exists {
            let git_ls_output = match Command::new("git")
                .args(["ls-files", "--error-unmatch", file])
                .output()
            {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("jjx_notch: error: failed to check git tracking: {}", e);
                    return 1;
                }
            };

            // If git ls-files fails, file is neither on disk nor tracked
            if !git_ls_output.status.success() {
                eprintln!("jjx_notch: error: file does not exist and is not tracked by git: {}", file);
                return 1;
            }
        }
    }

    // Parse identity - support both Coronet (5 chars) and Firemark (2 chars)
    let identity = args.identity.strip_prefix(JJRF_CORONET_PREFIX).or_else(|| args.identity.strip_prefix(JJRF_FIREMARK_PREFIX)).unwrap_or(&args.identity);

    let message = if identity.len() == 5 {
        // Coronet - pace-affiliated commit
        let coronet = match Coronet::jjrf_parse(&args.identity) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_notch: error: {}", e);
                return 1;
            }
        };
        jjrn_format_notch_prefix(&coronet)
    } else if identity.len() == 2 {
        // Firemark - heat-only commit
        let firemark = match Firemark::jjrf_parse(&args.identity) {
            Ok(fm) => fm,
            Err(e) => {
                eprintln!("jjx_notch: error: {}", e);
                return 1;
            }
        };
        let hallmark = vvc::vvcc_get_hallmark();
        let identity_str = format!("{}{}", JJRF_FIREMARK_PREFIX, firemark.jjrf_as_str());
        vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &hallmark, &identity_str, "n", "", None)
    } else {
        eprintln!("jjx_notch: error: identity must be Coronet (5 chars) or Firemark (2 chars), got {} chars", identity.len());
        return 1;
    };

    // Warn about uncommitted changes outside the file list
    let output = match Command::new("git")
        .args(["status", "--porcelain"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("jjx_notch: error: failed to run git status: {}", e);
            return 1;
        }
    };

    if !output.status.success() {
        eprintln!("jjx_notch: error: git status failed");
        return 1;
    }

    let status_output = String::from_utf8_lossy(&output.stdout);
    let files_set: HashSet<_> = args.files.iter().map(|s| s.as_str()).collect();
    let mut warnings = Vec::new();

    for line in status_output.lines() {
        if line.len() < 4 {
            continue;
        }
        // Parse git status --porcelain format: "XY filename"
        let filepath = &line[3..];
        if !files_set.contains(filepath) {
            warnings.push(format!("  {}", line));
        }
    }

    if !warnings.is_empty() {
        eprintln!("warning: uncommitted changes outside file list:");
        for warning in warnings {
            eprintln!("{}", warning);
        }
    }

    // Stage only the specified files
    let mut git_add = Command::new("git");
    git_add.arg("add");
    for file in &args.files {
        git_add.arg(file);
    }

    let add_output = match git_add.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("jjx_notch: error: failed to run git add: {}", e);
            return 1;
        }
    };

    if !add_output.status.success() {
        eprintln!("jjx_notch: error: git add failed");
        return 1;
    }

    // Commit using vvc with the generated message prefix
    // If --intent provided, use it as the message; otherwise let haiku generate it
    let commit_args = if let Some(intent) = args.intent {
        vvc::vvcc_CommitArgs {
            prefix: None,
            message: Some(format!("{}{}", message, intent)),
            allow_empty: false,
            no_stage: true,  // We already staged above
            size_limit: args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT),
            warn_limit: vvc::VVCG_WARN_LIMIT,
        }
    } else {
        vvc::vvcc_CommitArgs {
            prefix: Some(message),
            message: None,
            allow_empty: false,
            no_stage: true,  // We already staged above
            size_limit: args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT),
            warn_limit: vvc::VVCG_WARN_LIMIT,
        }
    };

    vvc::commit(&commit_args)
}
