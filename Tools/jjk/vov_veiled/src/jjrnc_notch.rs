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

use vvc::{vvco_err, vvco_Output};

use crate::jjrf_favor::{jjrf_Coronet, jjrf_Firemark, JJRF_FIREMARK_PREFIX, JJRF_CORONET_PREFIX, JJRF_FIREMARK_LEN, JJRF_CORONET_LEN};
use crate::jjrn_notch::{jjrn_format_notch_prefix, JJRN_COMMIT_PREFIX};

const JJRNC_CMD_NAME_RECORD: &str = "jjx_record";

/// Arguments for jjx_notch command
#[derive(Args, Debug)]
pub struct jjrnc_NotchArgs {
    /// Identity: Coronet (5-char, pace-affiliated) or Firemark (2-char, heat-only)
    pub identity: String,

    /// Files to commit; empty is legitimate — the empty notch (JJS0 `jjdz_monitum`)
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
pub fn jjrnc_run_notch(args: jjrnc_NotchArgs) -> (i32, String) {
    let cn = JJRNC_CMD_NAME_RECORD;
    let mut output = vvco_Output::buffer();

    // Check each file either exists on disk or is tracked by git (for deletions)
    let mut staged_deletions: HashSet<String> = HashSet::new();
    for file in &args.files {
        let path_exists = std::path::Path::new(file).exists();

        // If file doesn't exist on disk, check if it's tracked by git
        if !path_exists {
            let git_ls_output = match vvc::vvce_git_command(&["ls-files", "--error-unmatch", file])
                .output()
            {
                Ok(o) => o,
                Err(e) => {
                    vvco_err!(output, "{}: error: failed to check git tracking: {}", cn, e);
                    return (1, output.vvco_finish());
                }
            };

            // If git ls-files fails, check if it's a staged deletion
            if !git_ls_output.status.success() {
                let git_diff_output = match vvc::vvce_git_command(&["diff", "--cached", "--name-only", "--diff-filter=D", "--", file])
                    .output()
                {
                    Ok(o) => o,
                    Err(e) => {
                        vvco_err!(output, "{}: error: failed to check staged deletion: {}", cn, e);
                        return (1, output.vvco_finish());
                    }
                };

                // If diff output is non-empty, file is a staged deletion - accept it
                let is_staged_deletion = git_diff_output.status.success()
                    && !git_diff_output.stdout.is_empty();

                if !is_staged_deletion {
                    vvco_err!(output, "{}: error: file does not exist and is not tracked by git: {}", cn, file);
                    return (1, output.vvco_finish());
                }
                staged_deletions.insert(file.clone());
            }
        }
    }

    // Parse identity - support both Coronet (5 chars) and Firemark (2 chars)
    let identity = args.identity.strip_prefix(JJRF_CORONET_PREFIX).or_else(|| args.identity.strip_prefix(JJRF_FIREMARK_PREFIX)).unwrap_or(&args.identity);

    let message = if identity.len() == JJRF_CORONET_LEN {
        // Coronet - pace-affiliated commit
        let coronet = match jjrf_Coronet::jjrf_parse(&args.identity) {
            Ok(c) => c,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        jjrn_format_notch_prefix(&coronet)
    } else if identity.len() == JJRF_FIREMARK_LEN {
        // Firemark - heat-only commit
        let firemark = match jjrf_Firemark::jjrf_parse(&args.identity) {
            Ok(fm) => fm,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        let brand = vvc::vvcc_get_brand();
        let identity_str = format!("{}{}", JJRF_FIREMARK_PREFIX, firemark.jjrf_as_str());
        vvc::vvcc_format_branded(JJRN_COMMIT_PREFIX, &brand, &identity_str, "n", "", None)
    } else {
        vvco_err!(output, "{}: error: identity must be Coronet (5 chars) or Firemark (2 chars), got {} chars", cn, identity.len());
        return (1, output.vvco_finish());
    };

    // Warn about uncommitted changes outside the file list
    let status_result = match vvc::vvce_git_command(&["status", "--porcelain"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            vvco_err!(output, "{}: error: failed to run git status: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    if !status_result.status.success() {
        vvco_err!(output, "{}: error: git status failed", cn);
        return (1, output.vvco_finish());
    }

    let status_output = String::from_utf8_lossy(&status_result.stdout);
    let files_set: HashSet<_> = args.files.iter().map(|s| s.as_str()).collect();
    let warnings = jjrnc_outside_list_warnings(&status_output, &files_set);

    if !warnings.is_empty() {
        vvco_err!(output, "warning: uncommitted changes outside file list:");
        for warning in warnings {
            vvco_err!(output, "{}", warning);
        }
    }

    // Stage only the specified files (skip staged deletions — already staged by git rm)
    let files_to_add: Vec<&String> = args.files.iter().filter(|f| !staged_deletions.contains(f.as_str())).collect();
    if !files_to_add.is_empty() {
        let mut git_add = vvc::vvce_git_command(&["add"]);
        for file in &files_to_add {
            git_add.arg(file.as_str());
        }

        let add_output = match git_add.output() {
            Ok(o) => o,
            Err(e) => {
                vvco_err!(output, "{}: error: failed to run git add: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        if !add_output.status.success() {
            vvco_err!(output, "{}: error: git add failed", cn);
            return (1, output.vvco_finish());
        }
    }

    // Is the commit ahead of us empty? Asked of the index, not of the file list: listed
    // files that hold no changes stage nothing, and that is the case the monitum below
    // exists to name.
    let staged = match vvc::vvce_git_command(&["diff", "--cached", "--quiet"]).output() {
        Ok(o) => o,
        Err(e) => {
            vvco_err!(output, "{}: error: failed to check staged changes: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let is_empty = staged.status.success();

    // The haiku fallback describes a diff, and an empty notch has none — so an empty
    // commit's whole content is its intent, and the absent intent is a calling error the
    // caller corrects by naming what the commit records. The emptiness itself is never
    // refused (JJS0 `jjdz_monitum`).
    if is_empty && args.intent.is_none() {
        vvco_err!(output, "{}: error: an empty notch has no diff to describe, so --intent is required: name what this commit records", cn);
        return (1, output.vvco_finish());
    }

    // Size gate, ahead of the commit. The commit routine guards too, but it reports a
    // refusal in its own plain form; notch judges the staged cost itself so the refusal
    // it hands back is the interdictum, carrying the bytes the operator must review.
    let size_limit = args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
    match vvc::vvcg_cost(None) {
        Ok(cost) if cost.total > size_limit => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_size_interdictum(cn, &cost, size_limit));
            return (1, output.vvco_finish());
        }
        Ok(_) => {}
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    }

    // Commit using vvc with the generated message prefix
    // If --intent provided, use it as the message; otherwise let haiku generate it
    let commit_args = if let Some(intent) = args.intent {
        vvc::vvcc_CommitArgs {
            prefix: None,
            message: Some(format!("{}{}", message, intent)),
            allow_empty: true,
            no_stage: true,  // We already staged above
            size_limit,
            warn_limit: vvc::VVCG_WARN_LIMIT,
        }
    } else {
        vvc::vvcc_CommitArgs {
            prefix: Some(message),
            message: None,
            allow_empty: true,
            no_stage: true,  // We already staged above
            size_limit,
            warn_limit: vvc::VVCG_WARN_LIMIT,
        }
    };

    let rc = vvc::commit(&commit_args, &mut output);

    // After the fact, never ahead of it: the commit has landed, and the monitum names what
    // it recorded. The advisory is the whole safety mechanism of the allowed empty notch,
    // so it never gates — the exit code stays the commit's own.
    if rc == 0 && is_empty {
        vvco_err!(output, "{}", jjrnc_empty_notch_monitum(&args.files));
    }

    (rc, output.vvco_finish())
}

/// The empty-notch self-report — a monitum (JJS0 `jjdz_monitum`): non-gating, emitted after
/// the commit lands, naming what it actually recorded.
///
/// Two shapes, because the surprise differs. With no files listed, the empty commit is the
/// deliberate act — a verification run that changed nothing on disk — and the line says so.
/// With files listed, the caller expected content and got none, so the line names the gap and
/// its likeliest causes: a contentless commit under an expectant file list is the accident
/// this advisory exists to catch.
pub fn jjrnc_empty_notch_monitum(files: &[String]) -> String {
    let mut msg = String::from(
        "warning: empty notch — the commit landed with a tree identical to its parent's: it \
         records the affiliation and the intent, and no file content.\n",
    );
    if files.is_empty() {
        msg.push_str("  No files were listed — the empty notch, recording work that changed nothing on disk.");
    } else {
        msg.push_str(&format!(
            "  {} file(s) were listed and none held changes to stage. If content was expected, it may \
             already be committed (possibly by another officium), or the paths may be wrong.",
            files.len()
        ));
    }
    msg
}

/// Compute the warning lines for porcelain status entries not covered by the file list.
///
/// Parses `git status --porcelain` (v1) output. A staged rename or copy renders its
/// path as `old -> new`; such a line is covered only when BOTH sides appear in the
/// file list. Every other entry is covered when its single path appears in the list.
pub fn jjrnc_outside_list_warnings(status_output: &str, files_set: &HashSet<&str>) -> Vec<String> {
    let mut warnings = Vec::new();

    for line in status_output.lines() {
        if line.len() < 4 {
            continue;
        }
        // Parse git status --porcelain format: "XY filename"
        let status_code = &line[0..2];
        let filepath = &line[3..];
        // Renames/copies carry both endpoints in one "old -> new" path; cover only
        // when both endpoints are listed, otherwise the pair is genuinely outside.
        if status_code.starts_with('R') || status_code.starts_with('C') {
            if let Some((old_path, new_path)) = filepath.split_once(" -> ") {
                if !(files_set.contains(old_path) && files_set.contains(new_path)) {
                    warnings.push(format!("  {}", line));
                }
                continue;
            }
        }
        if !files_set.contains(filepath) {
            warnings.push(format!("  {}", line));
        }
    }

    warnings
}
