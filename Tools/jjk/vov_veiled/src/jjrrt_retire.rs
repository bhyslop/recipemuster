// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! jjx_retire command - Extract complete Heat data for archival trophy
//!
//! Handles retire operation: write trophy, remove from gallops, delete paddock, commit.
//! Fails fast if gallops file has uncommitted changes.

use std::path::{Path, PathBuf};

use vvc::{vvco_out, vvco_err, vvco_Output};

// Command name constant — RCG String Boundary Discipline
const JJRRT_CMD_NAME_ARCHIVE: &str = "jjx_archive";

use crate::jjrf_favor::jjrf_Firemark;
use crate::jjrg_gallops::jjrg_RetireArgs;
use crate::jjrs_steeplechase::{jjrs_ReinArgs, jjrs_get_entries};
use crate::jjrc_core::jjrc_timestamp_date;
use crate::jjrn_notch::{jjrn_format_heat_message, jjrn_HeatAction};

/// Arguments for jjx_retire command
#[derive(clap::Args, Debug)]
pub struct jjrrt_RetireArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark)
    pub firemark: String,

    /// Override size limit for commit guard (bytes)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// Run the retire command
pub fn jjrrt_run_retire(args: jjrrt_RetireArgs, officium: &str) -> (i32, String) {
    let cn = JJRRT_CMD_NAME_ARCHIVE;
    let mut output = vvco_Output::buffer();

    let firemark = match jjrf_Firemark::jjrf_parse(&args.firemark) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Load gallops
    let gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Get steeplechase entries (filtered by firemark identity)
    let rein_args = jjrs_ReinArgs {
        firemark: args.firemark.clone(),
        limit: 1000,
    };
    let steeplechase = match jjrs_get_entries(&rein_args) {
        Ok(entries) => entries,
        Err(e) => {
            vvco_err!(output, "{}: warning: could not get steeplechase: {}", cn, e);
            Vec::new()
        }
    };

    // Compute base path from gallops file
    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        // ===== Seam-off: the pre-seam path, verbatim =====
        // Fail-fast guard: reject if gallops file has uncommitted changes
        let gallops_rel = args.file.to_string_lossy().to_string();
        let status_output = match vvc::vvce_git_command(&["status", "--porcelain", "--", &gallops_rel])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                vvco_err!(output, "{}: error: failed to run git status: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        if !status_output.status.success() {
            vvco_err!(output, "{}: error: git status failed", cn);
            return (1, output.vvco_finish());
        }
        let status_text = String::from_utf8_lossy(&status_output.stdout);
        if !status_text.trim().is_empty() {
            vvco_err!(output, "{}: error: gallops file has uncommitted changes — commit or discard first", cn);
            return (1, output.vvco_finish());
        }

        let mut gallops = gallops;

        // Acquire lock FIRST
        let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
            Ok(l) => l,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        // Execute retire
        let retire_args = jjrg_RetireArgs {
            firemark: args.firemark.clone(),
            today: jjrc_timestamp_date(),
        };

        let result = match gallops.jjrg_retire(retire_args, base_path, &steeplechase) {
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

        // Commit using vvcm_commit with explicit file list
        // Files: gallops.json, trophy file (created), paddock file (deleted - git add handles this)
        let gallops_path = args.file.to_string_lossy().to_string();
        let commit_args = vvc::vvcm_CommitArgs {
            files: vec![
                gallops_path,
                result.trophy_path.clone(),
                result.paddock_path.clone(),
            ],
            message: jjrn_format_heat_message(&firemark, jjrn_HeatAction::Retire, &result.silks),
            size_limit: args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT),
            warn_limit: vvc::VVCG_WARN_LIMIT,
        };

        match vvc::machine_commit(&lock, &commit_args, &mut output) {
            Ok(hash) => {
                vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
            }
            Err(e) => {
                vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
                // Rollback: restore gallops and paddock from HEAD, remove trophy
                let gp = args.file.to_string_lossy().to_string();
                let _ = vvc::vvce_git_command(&["checkout", "HEAD", "--", &gp, &result.paddock_path])
                    .status();
                let _ = vvc::vvce_git_command(&["rm", "-f", "--cached", &result.trophy_path])
                    .status();
                let _ = std::fs::remove_file(&result.trophy_path);
                vvco_err!(output, "{}: rolled back file changes", cn);
                vvco_err!(output, "{}: retry with --size-limit <bytes> to override guard", cn);
                return (1, output.vvco_finish());
            }
        }

        // Output result
        vvco_out!(output, "trophy: {}", result.trophy_path);
        vvco_out!(output, "Heat {} retired successfully", result.firemark);

        (0, output.vvco_finish())
        // lock released here
    } else {
        // ===== Seam-on: journal the excision to the studbook, then apply the fs tail =====
        // The fail-fast guard and the gallops-in-commit are seam-off's — seam-on
        // the studbook is authority and the consumer gallops is never written, so
        // a guard on the consumer gallops.json checks the wrong store; the journal
        // ceremony (lock → advance → re-run against tip) IS the currency the guard
        // once stood in for. Paddock content is consumer-side (never a studbook
        // tenant), read before the ceremony. Steeplechase still reads the consumer
        // git log — its repoint is a banked flip-time concern for the conversion
        // heat, inert while the seam is closed.
        let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
            Ok(l) => l,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        let paddock_file = crate::jjri_io::jjri_paddock_path(firemark.jjrf_as_str());
        let paddock_content = match std::fs::read_to_string(base_path.join(&paddock_file)) {
            Ok(c) => c,
            Err(e) => {
                vvco_err!(output, "{}: error: Failed to read paddock file '{}': {}", cn, paddock_file, e);
                return (1, output.vvco_finish());
            }
        };

        let retire_args = jjrg_RetireArgs {
            firemark: args.firemark.clone(),
            today: jjrc_timestamp_date(),
        };
        let size_limit = args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);

        // Derive+excise against the LOCKED TIP, size-check the tip-derived trophy
        // there (a pre-ceremony check against a session read would be a stale
        // estimate — the tip can carry a concurrent station's paces), and journal
        // the excision. Nothing on disk yet; a decline (vanished heat = another
        // station already retired it) or a reject lands nothing anywhere.
        let plan = match crate::jjrm_mcp::zjjrm_journal_gallops(officium, cn, |g| {
            let plan = crate::jjrg_gallops::jjrg_retire_excise(g, &retire_args, &paddock_content, &steeplechase)?;
            let trophy_bytes = plan.trophy_content.len() as u64;
            if trophy_bytes > size_limit {
                return Err(format!(
                    "trophy is {} bytes, over the {}-byte ceiling — retry with a raised size_limit if the bulk is legitimate",
                    trophy_bytes, size_limit
                ));
            }
            let message = jjrn_format_heat_message(&firemark, jjrn_HeatAction::Retire, &plan.silks);
            Ok((plan, message))
        }) {
            Ok((plan, _sha)) => plan,
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
        };

        // Post-journal: the excision is durable in the studbook, so apply the fs
        // tail (write trophy, delete paddock) and commit [trophy, paddock] to the
        // consumer repo. Any failure past here is a loud split-state — the studbook
        // says retired and the repair is additive (the files ARE the repair), never
        // a backward revert that would manufacture a record contradicting the store
        // of truth.
        if let Err(e) = crate::jjrg_gallops::jjrg_retire_apply(base_path, &plan) {
            vvco_err!(output, "{}: studbook retired the heat but the fs apply failed — the repair is to write and commit the trophy: {}", cn, e);
            return (1, output.vvco_finish());
        }

        let commit_args = vvc::vvcm_CommitArgs {
            files: vec![
                plan.trophy_rel_path.clone(),
                plan.paddock_path.clone(),
            ],
            message: jjrn_format_heat_message(&firemark, jjrn_HeatAction::Retire, &plan.silks),
            size_limit,
            warn_limit: vvc::VVCG_WARN_LIMIT,
        };
        if let Err(e) = vvc::machine_commit(&lock, &commit_args, &mut output) {
            vvco_err!(output, "{}: studbook retired the heat but the consumer commit failed (trophy+paddock are staged on disk — commit them): {}", cn, crate::jjri_io::jjri_commit_refusal(cn, &e));
            return (1, output.vvco_finish());
        }

        // Output result
        vvco_out!(output, "trophy: {}", plan.trophy_rel_path);
        vvco_out!(output, "Heat {} retired successfully", plan.firemark_key);

        (0, output.vvco_finish())
        // lock released here
    }
}
