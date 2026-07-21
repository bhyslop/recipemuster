// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Nominate command - create a new Heat
//!
//! This module provides the Args struct and handler for the jjx_nominate command.

use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

use vvc::{vvco_out, vvco_err, vvco_Output};

// Command name constant — RCG String Boundary Discipline
const JJRNO_CMD_NAME_CREATE: &str = "jjx_create";

use crate::jjrf_favor::{jjrf_Firemark};
use crate::jjrg_gallops::{jjrg_Gallops, jjrg_NominateArgs};
use crate::jjrc_core::jjrc_timestamp_from_env;
use crate::jjrn_notch::{jjrn_HeatAction, jjrn_format_heat_message};

/// Arguments for jjx_nominate command
#[derive(clap::Args, Debug)]
pub struct jjrx_NominateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Kebab-case display name for the Heat
    #[arg(long, short = 's')]
    pub silks: String,
}

/// Handler for jjx_nominate command
pub fn jjrx_run_nominate(args: jjrx_NominateArgs, officium: &str) -> (i32, String) {
    let cn = JJRNO_CMD_NAME_CREATE;
    let mut output = vvco_Output::buffer();

    let base_path = args.file.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(Path::new("."));

    let silks = args.silks.clone();
    let nominate_args = jjrg_NominateArgs {
        silks: args.silks,
        created: jjrc_timestamp_from_env(),
    };

    if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        // ===== Seam-off: the pre-seam path, verbatim =====
        // Acquire lock FIRST - fail fast if another operation is in progress
        let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
            Ok(l) => l,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        let mut gallops = if args.file.exists() {
            match jjrg_Gallops::jjrg_load(&args.file) {
                Ok(g) => g,
                Err(e) => {
                    vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
                    return (1, output.vvco_finish());
                }
            }
        } else {
            if let Some(parent) = args.file.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    vvco_err!(output, "{}: error creating directory: {}", cn, e);
                    return (1, output.vvco_finish());
                }
            }
            jjrg_Gallops {
                next_heat_seed: "AA".to_string(),
                next_pace_seed: crate::jjrf_favor::JJRF_CORONET_SEED_FLOOR.to_string(),
                heat_order: vec![],
                heats: BTreeMap::new(),
                retention_since: None,
            }
        };

        match gallops.jjrg_nominate(nominate_args, base_path) {
            Ok(result) => {
                let fm = jjrf_Firemark::jjrf_parse(&result.firemark).expect("nominate returned invalid firemark");
                let message = jjrn_format_heat_message(&fm, jjrn_HeatAction::Nominate, &silks);

                match crate::jjri_io::jjri_persist(&lock, &gallops, &args.file, &fm, message, vvc::VVCG_SIZE_LIMIT, &mut output) {
                    Ok(hash) => {
                        vvco_out!(output, "{}: committed {}", cn, &hash[..8]);
                    }
                    Err(e) => {
                        vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
                        return (1, output.vvco_finish());
                    }
                }

                vvco_out!(output, "{}", result.firemark);
                (0, output.vvco_finish())
            }
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                (1, output.vvco_finish())
            }
        }
        // lock released here
    } else {
        // ===== Seam-on: derive the studbook + guidon, then journal the
        // nomination through the extracted ON path (jjrno_nominate_over) — the
        // explicit-config form a test drives against a fixture studbook + temp
        // consumer repo while the const stays false, so this two-store path
        // executes before flip-time rather than first in production. =====
        let (studbook, guidon) = match crate::jjrm_mcp::zjjrm_studbook_and_guidon(officium, cn) {
            Ok(sg) => sg,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        let code = jjrno_nominate_over(
            &crate::jjrfg_plaingit::jjrfg_PlainGit,
            &studbook,
            &guidon,
            nominate_args,
            base_path,
            &mut output,
            cn,
        );
        (code, output.vvco_finish())
    }
}

/// The seam-ON nominate path, extracted from the const gate so a test drives it
/// against a fixture studbook + temp consumer repo while
/// `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED` stays false (the `_over` idiom:
/// `zjjrm_write_gallops_over`, `jjrrt_retire_over`). nominate is the
/// machine_commit family's other two-store member: journal the heat insertion to
/// the studbook against the locked tip — which is also where `next_heat_seed` is
/// allocated from, so the printed firemark derives from the TIP (Shape B, the
/// message-from-transform shape draft/restring use, applied here to the printed
/// firemark instead of a message) — then apply the fs tail (write the paddock
/// template) and commit it to the consumer repo. `studbook`/`guidon` arrive
/// resolved. Writes to `output`, returns the exit code; the caller finishes the
/// buffer.
pub(crate) fn jjrno_nominate_over<F>(
    farrier: &F,
    studbook: &crate::jjrvb_blotter::jjdb_BlotterConfig,
    guidon: &str,
    nominate_args: jjrg_NominateArgs,
    base_path: &Path,
    output: &mut vvco_Output,
    cn: &str,
) -> i32
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore + crate::jjrfr_farrier::jjrfr_FarrierLock,
{
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return 1;
        }
    };

    // Derive+insert against the LOCKED TIP (next_heat_seed is the tip's, so a
    // concurrent station's genesis is never clobbered), then journal it. Nothing
    // on disk yet; a reject lands nothing anywhere.
    let plan = match crate::jjrm_mcp::zjjrm_journal_run(farrier, studbook, guidon, |g| {
        let plan = crate::jjrg_gallops::jjrg_nominate_excise(g, nominate_args)?;
        let fm = jjrf_Firemark::jjrf_parse(&plan.firemark_str)
            .map_err(|e| format!("nominate minted invalid firemark: {}", e))?;
        let message = jjrn_format_heat_message(&fm, jjrn_HeatAction::Nominate, &plan.silks);
        Ok((plan, message))
    }) {
        Ok((plan, _sha)) => plan,
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return 1;
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            return 1;
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
            vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
            return 1;
        }
    };

    // Post-journal: the nomination is durable in the studbook, so apply the fs
    // tail (write the paddock template) and commit it to the consumer repo. Any
    // failure past here is a loud split-state — the studbook says nominated and
    // the repair is additive (writing and committing the paddock IS the repair),
    // never a backward revert that would manufacture a record contradicting the
    // store of truth.
    if let Err(e) = crate::jjrg_gallops::jjrg_nominate_apply(base_path, &plan) {
        vvco_err!(output, "{}: studbook nominated the heat but the fs apply failed — the repair is to write and commit the paddock: {}", cn, e);
        return 1;
    }

    let fm = jjrf_Firemark::jjrf_parse(&plan.firemark_str).expect("nominate journaled a valid firemark");
    let commit_args = vvc::vvcm_CommitArgs {
        files: vec![plan.paddock_rel_path.clone()],
        message: jjrn_format_heat_message(&fm, jjrn_HeatAction::Nominate, &plan.silks),
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };
    if let Err(e) = vvc::machine_commit(&lock, &commit_args, output) {
        vvco_err!(output, "{}: studbook nominated the heat but the consumer commit failed (paddock is staged on disk — commit it): {}", cn, crate::jjri_io::jjri_commit_refusal(cn, &e));
        return 1;
    }

    vvco_out!(output, "{}", plan.firemark_str);
    0
    // lock released here
}

#[cfg(test)]
mod tests {
    use crate::jjrg_gallops::*;
    use crate::jjtu_testdir::JjkTestDir;
    use std::collections::BTreeMap;

    #[test]
    fn test_nominate_appends_to_heat_order() {
        let td = JjkTestDir::new("jjk_test_nominate_appends_to_heat_order");

        let mut gallops = jjrg_Gallops {
            next_heat_seed: "AA".to_string(),
            next_pace_seed: "CAAAA".to_string(),
            heat_order: vec![],
            heats: BTreeMap::new(),
            retention_since: None,
        };
        let args = jjrg_NominateArgs {
            silks: "test-heat".to_string(),
            created: "260101".to_string(),
        };
        let result = gallops.jjrg_nominate(args, td.path()).unwrap();
        assert!(gallops.heat_order.contains(&result.firemark),
            "heat_order should contain the new firemark after nominate");
    }
}
