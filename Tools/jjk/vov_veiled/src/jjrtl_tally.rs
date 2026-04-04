// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! JJK Tally commands - pace modification operations
//!
//! Split into three single-purpose commands:
//! - revise_docket: Update pace docket text
//! - relabel: Rename pace silks
//! - drop: Set state to abandoned

use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

const JJRTL_CMD_NAME_RELABEL: &str = "jjx_relabel";
const JJRTL_CMD_NAME_DROP: &str = "jjx_drop";

use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState};
use crate::jjrn_notch::{jjrn_format_heat_message as format_heat_message, jjrn_HeatAction as HeatAction};

/// Arguments for jjx_redocket command
#[derive(clap::Args, Debug)]
pub struct jjrtl_ReviseDocketArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Pace identity (Coronet)
    pub coronet: String,
}

/// Run the revise_docket procedure within the dispatch lifecycle.
///
/// Receives &mut Gallops from dispatcher. Returns diff of old vs new docket.
/// Lock, load, and persist are owned by the dispatcher (jjsodp_command_lifecycle).
pub fn jjrtl_run_revise_docket(
    gallops: &mut Gallops,
    coronet: &str,
    docket: &str,
) -> Result<String, String> {
    // Capture I/O at procedure boundary — method is pure
    let basis = crate::jjru_util::jjrg_capture_commit_sha();
    let ts = crate::jjrc_core::jjrc_timestamp_full();

    let ctx = gallops.jjrg_revise_docket(coronet, docket, &basis, &ts)?;

    Ok(jjrtl_diff_docket(&ctx.text, docket))
}

/// Generate a line-level diff between old and new docket text.
///
/// Uses longest-common-subsequence to produce unified-style output:
/// unchanged lines shown as-is, removed lines prefixed with "- ",
/// added lines prefixed with "+ ".
fn jjrtl_diff_docket(old: &str, new: &str) -> String {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    let m = old_lines.len();
    let n = new_lines.len();

    // Build LCS table
    let mut dp = vec![vec![0u32; n + 1]; m + 1];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if old_lines[i] == new_lines[j] {
                dp[i + 1][j + 1] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }

    // Walk the table to emit diff lines
    let mut result = String::new();
    let mut i = 0;
    let mut j = 0;
    while i < m || j < n {
        if i < m && j < n && old_lines[i] == new_lines[j] {
            // Context line — skip to keep output compact
            i += 1;
            j += 1;
        } else if i < m && (j >= n || dp[i + 1][j] >= dp[i][j + 1]) {
            result.push_str("- ");
            result.push_str(old_lines[i]);
            result.push('\n');
            i += 1;
        } else {
            result.push_str("+ ");
            result.push_str(new_lines[j]);
            result.push('\n');
            j += 1;
        }
    }

    result
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
pub fn jjrtl_run_relabel(args: jjrtl_RelabelArgs) -> (i32, String) {
    let cn = JJRTL_CMD_NAME_RELABEL;
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Get firemark for commit message before we move args
    let coronet_str = args.coronet.clone();
    let new_silks = args.silks.clone();
    let fm = match Coronet::jjrf_parse(&coronet_str) {
        Ok(c) => c.jjrf_parent_firemark(),
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
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

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000, &mut output) {
                Ok(hash) => {
                    vvco_out!(output, "committed {}", hash);
                    (0, output.vvco_finish())
                }
                Err(e) => {
                    vvco_err!(output, "{}: error: {}", cn, e);
                    (1, output.vvco_finish())
                }
            }
        }
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            (1, output.vvco_finish())
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
pub fn jjrtl_run_drop(args: jjrtl_DropArgs) -> (i32, String) {
    let cn = JJRTL_CMD_NAME_DROP;
    use crate::jjrg_gallops::jjrg_TallyArgs as LibTallyArgs;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
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
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
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

            match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, 50000, &mut output) {
                Ok(hash) => {
                    vvco_out!(output, "committed {}", hash);
                    (0, output.vvco_finish())
                }
                Err(e) => {
                    vvco_err!(output, "{}: error: {}", cn, e);
                    (1, output.vvco_finish())
                }
            }
        }
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jjrtl_diff_identical_texts_empty() {
        let text = "line one\nline two\nline three";
        assert_eq!(jjrtl_diff_docket(text, text), "");
    }

    #[test]
    fn jjrtl_diff_empty_to_content() {
        let result = jjrtl_diff_docket("", "added line");
        assert_eq!(result, "+ added line\n");
    }

    #[test]
    fn jjrtl_diff_content_to_empty() {
        let result = jjrtl_diff_docket("removed line", "");
        assert_eq!(result, "- removed line\n");
    }

    #[test]
    fn jjrtl_diff_single_line_changed() {
        let result = jjrtl_diff_docket("old line", "new line");
        assert_eq!(result, "- old line\n+ new line\n");
    }

    #[test]
    fn jjrtl_diff_line_added_in_middle() {
        let old = "first\nthird";
        let new = "first\nsecond\nthird";
        let result = jjrtl_diff_docket(old, new);
        assert_eq!(result, "+ second\n");
    }

    #[test]
    fn jjrtl_diff_line_removed_from_middle() {
        let old = "first\nsecond\nthird";
        let new = "first\nthird";
        let result = jjrtl_diff_docket(old, new);
        assert_eq!(result, "- second\n");
    }

    #[test]
    fn jjrtl_diff_line_replaced_in_context() {
        let old = "## Docket\nold requirement\n### Verification\nrun tests";
        let new = "## Docket\nnew requirement\n### Verification\nrun tests";
        let result = jjrtl_diff_docket(old, new);
        assert_eq!(result, "- old requirement\n+ new requirement\n");
    }

    #[test]
    fn jjrtl_diff_multiple_changes() {
        let old = "alpha\nbeta\ngamma\ndelta";
        let new = "alpha\nBETA\ngamma\nDELTA";
        let result = jjrtl_diff_docket(old, new);
        assert_eq!(result, "- beta\n+ BETA\n- delta\n+ DELTA\n");
    }
}
