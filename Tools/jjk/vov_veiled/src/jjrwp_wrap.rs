// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Wrap command - mark a pace complete with automated commit
//!
//! Stages all changes, generates a commit message with Claude,
//! and transitions the pace state to complete with chalk marker.

use std::path::PathBuf;
use std::process::Command;
use std::io::Write;

use crate::jjrf_favor::{jjrf_Coronet as Coronet};
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_TallyArgs as LibTallyArgs, jjrg_PaceState};
use crate::jjrn_notch::{jjrn_ChalkMarker, jjrn_format_notch_prefix, jjrn_format_chalk_message};

/// Arguments for jjx_wrap command
#[derive(clap::Args, Debug)]
pub struct jjrx_WrapArgs {
    /// Pace identity (Coronet)
    pub coronet: String,

    /// Size limit in bytes (overrides default 50KB guard)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Execute wrap command - mark pace complete with commit
///
/// Stages all changes, checks size, generates commit message using Claude,
/// creates the work commit, then transitions pace to complete state with
/// chalk marker commit.
///
/// Returns exit code (0 for success, non-zero for failure).
pub fn zjjrx_run_wrap(args: jjrx_WrapArgs) -> i32 {
    // Parse coronet
    let coronet = match Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("jjx_wrap: error: {}", e);
            return 1;
        }
    };

    // Acquire commit lock
    let _lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("jjx_wrap: error: {}", e);
            return 1;
        }
    };

    // Stage all changes
    let add_output = match Command::new("git")
        .args(["add", "-A"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("jjx_wrap: error: failed to run git add: {}", e);
            return 1;
        }
    };

    if !add_output.status.success() {
        eprintln!("jjx_wrap: error: git add failed");
        return 1;
    }

    // Size guard check
    let size_limit = args.size_limit.unwrap_or(50000); // 50KB default
    let diff_output = match Command::new("git")
        .args(["diff", "--cached", "--stat"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("jjx_wrap: error: failed to run git diff: {}", e);
            return 1;
        }
    };

    if !diff_output.status.success() {
        eprintln!("jjx_wrap: error: git diff failed");
        return 1;
    }

    // Parse size from last line of --stat output (format: "N files changed, M insertions(+), K deletions(-)")
    // Better size check: get actual staged content size
    let numstat_output = match Command::new("git")
        .args(["diff", "--cached", "--numstat"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            eprintln!("jjx_wrap: error: failed to run git diff --numstat: {}", e);
            return 1;
        }
    };

    if !numstat_output.status.success() {
        eprintln!("jjx_wrap: error: git diff --numstat failed");
        return 1;
    }

    let numstat_str = String::from_utf8_lossy(&numstat_output.stdout);
    let mut total_size: u64 = 0;
    for line in numstat_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            // Format: added\tdeleted\tfilename
            if let Ok(added) = parts[0].parse::<u64>() {
                total_size += added;
            }
            if let Ok(deleted) = parts[1].parse::<u64>() {
                total_size += deleted;
            }
        }
    }

    if total_size > size_limit {
        eprintln!("jjx_wrap: error: staged changes exceed size limit ({} > {} bytes)", total_size, size_limit);
        // Lock released automatically by Drop
        return 2;
    }

    // Check if there are staged changes to commit
    let has_staged_changes = !numstat_str.trim().is_empty();

    // Only generate commit message and commit if there are staged changes
    let commit_hash = if has_staged_changes {
        // Generate commit message using Claude CLI
        let diff_content = match Command::new("git")
            .args(["diff", "--cached"])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                eprintln!("jjx_wrap: error: failed to run git diff --cached: {}", e);
                return 1;
            }
        };

        if !diff_content.status.success() {
            eprintln!("jjx_wrap: error: git diff --cached failed");
            return 1;
        }

        let mut claude_cmd = match Command::new("claude")
            .args([
                "--print",
                &format!("Generate a concise commit message for this diff. The commit wraps pace {}. Output only the message, no quotes.", args.coronet)
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("jjx_wrap: error: failed to spawn claude command: {}", e);
                return 1;
            }
        };

        // Write diff to stdin
        if let Some(mut stdin) = claude_cmd.stdin.take() {
            if let Err(e) = stdin.write_all(&diff_content.stdout) {
                eprintln!("jjx_wrap: error: failed to write to claude stdin: {}", e);
                return 1;
            }
        }

        let claude_output = match claude_cmd.wait_with_output() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("jjx_wrap: error: failed to wait for claude: {}", e);
                return 1;
            }
        };

        if !claude_output.status.success() {
            eprintln!("jjx_wrap: error: claude command failed");
            eprintln!("{}", String::from_utf8_lossy(&claude_output.stderr));
            return 1;
        }

        let generated_message = String::from_utf8_lossy(&claude_output.stdout).trim().to_string();

        // Commit with git directly (already staged via git add -A)
        let prefix = jjrn_format_notch_prefix(&coronet);
        let full_message = format!("{}{}\n\nCo-Authored-By: Claude <noreply@anthropic.com>", prefix, generated_message);

        let commit_output = match Command::new("git")
            .args(["commit", "-m", &full_message])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                eprintln!("jjx_wrap: error: failed to run git commit: {}", e);
                return 1;
            }
        };

        if !commit_output.status.success() {
            eprintln!("jjx_wrap: error: git commit failed");
            eprintln!("{}", String::from_utf8_lossy(&commit_output.stderr));
            return 1;
        }

        // Get commit hash
        let hash_output = match Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                eprintln!("jjx_wrap: error: failed to get commit hash: {}", e);
                return 1;
            }
        };

        String::from_utf8_lossy(&hash_output.stdout).trim().to_string()
    } else {
        // No staged changes - this is valid for verification-only paces
        eprintln!("jjx_wrap: no staged changes, proceeding with state transition only");

        // Get current HEAD as reference (no new work commit created)
        let hash_output = match Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                eprintln!("jjx_wrap: error: failed to get commit hash: {}", e);
                return 1;
            }
        };

        String::from_utf8_lossy(&hash_output.stdout).trim().to_string()
    };

    // Transition pace state to complete
    let gallops_path = PathBuf::from(".claude/jjm/jjg_gallops.json");
    let mut gallops = match Gallops::jjrg_load(&gallops_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_wrap: error loading Gallops: {}", e);
            return 1;
        }
    };

    let tally_args = LibTallyArgs {
        coronet: args.coronet.clone(),
        state: Some(jjrg_PaceState::Complete),
        direction: None,
        text: None,
        silks: None,
    };

    if let Err(e) = gallops.jjrg_tally(tally_args) {
        eprintln!("jjx_wrap: error: {}", e);
        return 1;
    }

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&gallops_path) {
        eprintln!("jjx_wrap: error saving Gallops: {}", e);
        return 1;
    }

    // Create W chalk marker commit with gallops state change
    let chalk_message = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Wrap, "pace complete");

    let chalk_commit_args = vvc::vvcm_CommitArgs {
        files: vec![".claude/jjm/jjg_gallops.json".to_string()],
        message: chalk_message,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    match vvc::machine_commit(&_lock, &chalk_commit_args) {
        Ok(_) => {
            println!("{}", commit_hash);
            let fm = coronet.jjrf_parent_firemark();
            let fm_str = fm.jjrf_as_str();

            // Lookahead: find next actionable pace in this heat
            let next_pace_info = gallops.heats.get(fm_str).and_then(|heat| {
                heat.order.iter().find_map(|c| {
                    heat.paces.get(c.as_str()).and_then(|pace| {
                        pace.tacks.first().and_then(|tack| {
                            match tack.state {
                                jjrg_PaceState::Rough | jjrg_PaceState::Bridled => {
                                    Some((c.clone(), tack.silks.clone()))
                                }
                                _ => None,
                            }
                        })
                    })
                })
            });

            eprintln!();
            match next_pace_info {
                Some((next_coronet, next_silks)) => {
                    eprintln!("AGENT_RESPONSE: \u{20a2}{} wrapped. Next: {} (\u{20a2}{}) \u{2014} `/clear` then `/jjc-heat-mount {}`",
                        args.coronet, next_silks, next_coronet, fm_str);
                }
                None => {
                    eprintln!("AGENT_RESPONSE: \u{20a2}{} wrapped. All paces complete \u{2014} `/clear` then `/jjc-heat-retire {}`",
                        args.coronet, fm_str);
                }
            }
            0
        }
        Err(e) => {
            eprintln!("jjx_wrap: error: chalk commit failed: {}", e);
            eprintln!("jjx_wrap: warning: gallops state updated but not committed");
            1
        }
    }
    // lock released here
}
