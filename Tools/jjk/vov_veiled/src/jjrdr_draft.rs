// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Draft command - move a Pace from one Heat to another
//!
//! Implements the jjx_draft subcommand and related structures.

use clap::Args;
use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

const JJRDR_CMD_NAME_DRAFT: &str = "jjx_draft";

use crate::jjrf_favor::jjrf_Firemark;
use crate::jjri_io::jjri_paddock_path;
use crate::jjrn_notch::{jjrn_HeatAction, jjrn_format_heat_message};

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
pub fn jjrdr_run_draft(args: jjrdr_DraftArgs, officium: &str) -> (i32, String) {
    let cn = JJRDR_CMD_NAME_DRAFT;
    use crate::jjrg_gallops::jjrg_DraftArgs;
    let mut output = vvco_Output::buffer();

    // Acquire lock FIRST - fail fast if another operation is in progress
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
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

    let coronet = args.coronet.clone();
    let to = args.to.clone();
    // Source firemark for the source paddock path — captured BEFORE the draft,
    // since post-move the pace lives in the destination (JJS0 jjdt_coronet: the
    // source is found by paces-scan, not inferred from the flat id).
    let src_fm = crate::jjrf_favor::jjrf_Coronet::jjrf_parse(&coronet).ok()
        .and_then(|c| gallops.jjrg_heat_key_of_coronet(&c.jjrf_display()))
        .and_then(|k| jjrf_Firemark::jjrf_parse(&k).ok());
    let draft_args = jjrg_DraftArgs {
        coronet: args.coronet,
        to: args.to,
        before: args.before,
        after: args.after,
        first: args.first,
    };

    if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        // Seam-off: the pre-seam path — mutate the session gallops, save it, and
        // machine_commit the gallops with the two paddocks (unchanged, staged
        // defensively) to the consumer repo, keeping draft's own commit-error arms.
        match gallops.jjrg_draft(draft_args) {
            Ok(result) => {
                // Save gallops
                if let Err(e) = gallops.jjrg_save(&args.file) {
                    vvco_err!(output, "{}: error saving Gallops: {}", cn, e);
                    return (1, output.vvco_finish());
                }

                // Commit using machine_commit - draft affects source and dest paddocks
                let src_fm = src_fm.expect("draft succeeded, so its source heat was found");
                let dest_fm = jjrf_Firemark::jjrf_parse(&to).expect("draft given invalid destination firemark");

                let gallops_path = args.file.to_string_lossy().to_string();
                let src_paddock_path = jjri_paddock_path(src_fm.jjrf_as_str());
                let dest_paddock_path = jjri_paddock_path(dest_fm.jjrf_as_str());

                let commit_args = vvc::vvcm_CommitArgs {
                    files: vec![
                        gallops_path,
                        src_paddock_path,
                        dest_paddock_path,
                    ],
                    message: jjrn_format_heat_message(&dest_fm, jjrn_HeatAction::Draft, &format!("{} → {}", coronet, result.new_coronet)),
                    size_limit: vvc::VVCG_SIZE_LIMIT,
                    warn_limit: vvc::VVCG_WARN_LIMIT,
                };

                match vvc::machine_commit(&lock, &commit_args, &mut output) {
                    Ok(hash) => {
                        vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
                    }
                    Err(e @ vvc::vvcm_CommitError::OverLimit { .. }) => {
                        // A barred act leaves nothing behind: restore the files this
                        // relocate wrote, so the refusal is a refusal and not a
                        // half-applied move waiting to ride the next command's commit.
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

                vvco_out!(output, "{}", result.new_coronet);
                (0, output.vvco_finish())
            }
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                (1, output.vvco_finish())
            }
        }
    } else {
        // Seam-on: derive the studbook + guidon, then journal the relocate through
        // the extracted ON path (jjrdr_draft_over) — the explicit-config form a test
        // drives against a fixture studbook while the const stays false.
        let (studbook, guidon) = match crate::jjrm_mcp::zjjrm_studbook_and_guidon(officium, cn) {
            Ok(sg) => sg,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        let code = jjrdr_draft_over(
            &crate::jjrfg_plaingit::jjrfg_PlainGit,
            &studbook,
            &guidon,
            draft_args,
            &coronet,
            &to,
            &mut output,
            cn,
        );
        (code, output.vvco_finish())
    }
    // lock released here
}

/// The seam-ON draft (relocate) path, extracted from the const gate so a test
/// drives it against a fixture studbook while `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED`
/// stays false (the `_over` idiom). draft is a pure in-memory member of the
/// machine_commit family: `jjrg_draft` mutates only the gallops, so the on path
/// journals to the studbook against the locked tip and has no consumer remainder to
/// commit; the message names the TIP's own minted coronet (message-from-transform).
/// `studbook`/`guidon` arrive resolved. Writes to `output`, returns the exit code.
#[allow(clippy::too_many_arguments)]
pub(crate) fn jjrdr_draft_over<F>(
    farrier: &F,
    studbook: &crate::jjrvb_blotter::jjdb_BlotterConfig,
    guidon: &str,
    draft_args: crate::jjrg_gallops::jjrg_DraftArgs,
    coronet: &str,
    to: &str,
    output: &mut vvco_Output,
    cn: &str,
) -> i32
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore + crate::jjrfr_farrier::jjrfr_FarrierLock,
{
    match crate::jjrm_mcp::zjjrm_journal_run(farrier, studbook, guidon, |g| {
        let result = g.jjrg_draft(draft_args)?;
        let dest_fm = jjrf_Firemark::jjrf_parse(to)
            .map_err(|e| format!("draft given invalid destination firemark: {}", e))?;
        let message = jjrn_format_heat_message(&dest_fm, jjrn_HeatAction::Draft, &format!("{} → {}", coronet, result.new_coronet));
        Ok((result, message))
    }) {
        Ok((result, _sha)) => {
            vvco_out!(output, "{}", result.new_coronet);
            0
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            1
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            1
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
            vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
            1
        }
    }
}
