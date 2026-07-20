// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Restring command - bulk draft multiple paces between heats
//!
//! Handles the jjx_restring command which moves multiple paces from one heat
//! to another in a single atomic operation.

use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

use crate::jjrf_favor::jjrf_Firemark;
use crate::jjrg_gallops::jjrg_RestringArgs;
use crate::jjrn_notch::{jjrn_HeatAction, jjrn_format_heat_message};

const JJRRS_CMD_NAME_RESTRING: &str = "jjx_restring";

/// Arguments for jjx_restring command
#[derive(clap::Args, Debug)]
pub struct jjrrs_RestringArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Source Heat identity (Firemark)
    pub firemark: String,

    /// Destination Heat identity (Firemark)
    #[arg(long)]
    pub to: String,

    /// Override commit size guard limit in bytes
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Execute restring command - bulk draft multiple paces atomically
pub fn jjrrs_run(args: jjrrs_RestringArgs, coronets: String, officium: &str) -> (i32, String) {
    let cn = JJRRS_CMD_NAME_RESTRING;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let coronets: Vec<String> = match serde_json::from_str(&coronets) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error: Expected JSON array of coronets: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let mut gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let restring_args = jjrg_RestringArgs {
        source_firemark: args.firemark.clone(),
        dest_firemark: args.to.clone(),
        coronets,
    };

    // Execute restring, then persist. Off: mutate the session gallops, save it, and
    // machine_commit the gallops with the two paddocks (unchanged, staged
    // defensively) to the consumer repo, keeping restring's own commit-error arms.
    // On: journal the bulk relocate to the studbook against the locked tip — a pure
    // in-memory mutation, so no consumer remainder to commit, and the message counts
    // the tip's own drafted set. Either way `result` drives the shared JSON output.
    let result = if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        // Execute restring operation
        let result = match gallops.jjrg_restring(restring_args) {
            Ok(r) => r,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        // Save gallops
        if let Err(e) = gallops.jjrg_save(&args.file) {
            vvco_err!(output, "{}: error saving Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }

        // Parse both firemarks for commit file list
        let dest_fm = jjrf_Firemark::jjrf_parse(&result.dest_firemark).expect("restring returned invalid dest firemark");

        let gallops_path = args.file.to_string_lossy().to_string();
        let source_paddock_path = result.source_paddock.clone();
        let dest_paddock_path = result.dest_paddock.clone();

        // Build commit message using heat-level action
        let commit_message = jjrn_format_heat_message(
            &dest_fm,
            jjrn_HeatAction::Draft,  // Reusing Draft action since this is a bulk draft operation
            &format!("restring {} paces from {}", result.drafted.len(), result.source_firemark)
        );

        let size_limit = args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
        let commit_args = vvc::vvcm_CommitArgs {
            files: vec![
                gallops_path,
                source_paddock_path,
                dest_paddock_path,
            ],
            message: commit_message,
            size_limit,
            warn_limit: vvc::VVCG_WARN_LIMIT,
        };

        match vvc::machine_commit(&lock, &commit_args, &mut output) {
            Ok(hash) => {
                vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
            }
            Err(e @ vvc::vvcm_CommitError::OverLimit { .. }) => {
                // A barred act leaves nothing behind: restore the files this restring
                // wrote, so the refusal is a refusal and not a half-applied transfer
                // waiting to ride the next command's commit.
                for f in &commit_args.files {
                    let _ = vvc::vvce_git_command(&["checkout", "HEAD", "--", f.as_str()]).output();
                    let _ = vvc::vvce_git_command(&["reset", "--quiet", "--", f.as_str()]).output();
                }
                vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
                return (1, output.vvco_finish());
            }
            Err(e) => {
                vvco_err!(output, "{}: commit warning: {}", cn, e);
            }
        }
        result
    } else {
        match crate::jjrm_mcp::zjjrm_journal_gallops(officium, cn, |g| {
            let result = g.jjrg_restring(restring_args)?;
            let dest_fm = jjrf_Firemark::jjrf_parse(&result.dest_firemark)
                .map_err(|e| format!("restring returned invalid dest firemark: {}", e))?;
            let message = jjrn_format_heat_message(
                &dest_fm,
                jjrn_HeatAction::Draft,
                &format!("restring {} paces from {}", result.drafted.len(), result.source_firemark),
            );
            Ok((result, message))
        }) {
            Ok((result, _sha)) => result,
            Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
            Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
                vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
                return (1, output.vvco_finish());
            }
            Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
                vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
                return (1, output.vvco_finish());
            }
        }
    };

    // Output JSON result
    let drafted_json: Vec<_> = result.drafted.iter().map(|m| {
        serde_json::json!({
            "old_coronet": m.old_coronet,
            "new_coronet": m.new_coronet,
            "silks": m.silks,
            "state": m.state.jjrg_as_str(),
            "spec": m.spec,
        })
    }).collect();

    let json_result = serde_json::json!({
        "source": {
            "firemark": result.source_firemark,
            "silks": result.source_silks,
            "paddock": result.source_paddock,
            "empty_after": result.source_empty_after,
        },
        "destination": {
            "firemark": result.dest_firemark,
            "silks": result.dest_silks,
            "paddock": result.dest_paddock,
        },
        "drafted": drafted_json,
    });

    match serde_json::to_string_pretty(&json_result) {
        Ok(json) => {
            vvco_out!(output, "{}", json);
            (0, output.vvco_finish())
        }
        Err(e) => {
            vvco_err!(output, "{}: error serializing output: {}", cn, e);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}
