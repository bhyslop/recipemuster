// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — ostend: the reveal ceremony's irreversible showing (RBSHO). The
// once-per-cycle kerygma of the standing dry regulus under a granted
// cachet: the re-asserted ground, the operator's own file-list eyes, the
// kerygma, and promotion — every assert machine-performed, every push
// typed by the operator. Never re-cuts: the dry regulus is bit-for-bit
// what ships, and a terminal re-cut would break exactly that identity.
//
// Machine-asserts-around-human-pushes (RBSHC "The command seam"): every push
// is shown, never held — this module holds no cloud credential and spends
// nothing; its acts are remote reads and the operator's own pushes. No `git
// push` to any real remote lives in this file.
//
// Rehearse tolerates an absent cachet with a loud warning and stops before
// the kerygma line — the reversible stages proven, the irreversible ones
// never touched.

use std::path::Path;
use std::process::ExitCode;

use crate::rbvcet_cachet;
use crate::rbvcex_expede;
use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvcer_run;

/// The kerygma and promotion target is the real public repository — the
/// same endpoint the cut clones read-only (rbvcex_expede::RBVCEX_BASE_URL).
const RBVCCO_MAIN_REF: &str = "refs/heads/main";

/// Conduct the ostend. `rehearse` proves the reversible stages (cachet
/// tolerant, re-assert the ground, the file-list review) and stops before the
/// kerygma line — no push shown, nothing irreversible touched. Fatal on
/// any deficit; ExitCode::SUCCESS only when, outside rehearse, the
/// kerygma and promotion both verified by remote read.
pub fn rbvcco_ostend_conduct(rehearse: bool) -> ExitCode {
    rbvcel_log::rbvcel_section("Hierophant Ostend — the reveal's irreversible showing (RBSHO)");
    if rehearse {
        rbvcel_log::rbvcel_line("REHEARSAL — reversible stages only: stops before the kerygma line.");
    }

    let top = rbvcep_repo::rbvcep_toplevel();
    let parent = rbvcep_repo::rbvcep_parent(&top);
    rbvcel_log::rbvcel_line(&format!("Maintainer tree: {}", top.display()));

    let regulus_parent = parent.join(rbvcep_repo::RBVCEP_REGULUS_DIRNAME);
    let regulus_clone = regulus_parent.join(rbvcep_repo::RBVCEP_REGULUS_SUBDIR);
    if !regulus_clone.is_dir() {
        crate::rbvcel_fatal!(
            "no standing regulus at {} — run essai first (RBSHE)",
            regulus_clone.display()
        );
    }
    let regulus_tip = rbvcep_repo::rbvcep_commit_sha(&regulus_clone, &top);
    rbvcel_log::rbvcel_line(&format!("Standing regulus: {} (tip {})", regulus_clone.display(), regulus_tip));

    zrbvcco_require_cachet(&regulus_parent, &regulus_clone, &top, rehearse);
    zrbvcco_reassert_ground(&top, &parent, &regulus_clone, &regulus_tip);
    zrbvcco_file_list_review(&top, &regulus_clone);

    if rehearse {
        rbvcel_log::rbvcel_blank();
        rbvcel_log::rbvcel_success("Ostend rehearsal complete — cachet checked, ground re-asserted, file list reviewed. Stopped before the kerygma line.");
        return ExitCode::SUCCESS;
    }

    zrbvcco_kerygma(&top, &regulus_clone, &regulus_tip);
    zrbvcco_promotion(&top, &regulus_tip);
    zrbvcco_close();

    rbvcel_log::rbvcel_success("Ostend complete — disclosed and promoted, every assert machine-performed, every push human-typed (RBSHO completion).");
    ExitCode::SUCCESS
}

// ── Step 1: require the cachet ──────────────────────────────

fn zrbvcco_require_cachet(regulus_parent: &Path, regulus_clone: &Path, top: &Path, rehearse: bool) {
    rbvcel_log::rbvcel_section("Require the cachet (RBSHO step 1)");
    if rehearse {
        rbvcet_cachet::rbvcet_require_rehearse(regulus_parent, regulus_clone, top);
    } else {
        rbvcet_cachet::rbvcet_require(regulus_parent, regulus_clone, top);
    }
}

// ── Step 2: re-assert the ground ────────────────────────────

fn zrbvcco_reassert_ground(top: &Path, parent: &Path, regulus_clone: &Path, regulus_tip: &str) {
    rbvcel_log::rbvcel_section("Re-assert the ground (RBSHO step 2)");

    rbvcex_expede::rbvcex_assert_narthex_private(top);

    let branch = rbvcex_expede::RBVCEX_REGULUS_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);
    let refs = rbvcep_repo::rbvcep_ls_remote(rbvcex_expede::RBVCEX_NARTHEX_URL, top);
    let preview_stands = refs.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if !preview_stands {
        crate::rbvcel_fatal!(
            "the narthex's {} tip does not equal the regulus tip {} — the preview does not stand; the cycle returns to essai, never forward",
            branch, regulus_tip
        );
    }
    rbvcel_log::rbvcel_line("preview stands: narthex tip equals the regulus tip");

    rbvcex_expede::rbvcex_assert_fresh(top, parent, regulus_clone);
}

// ── Step 3: file-list review — the operator's own eyes ──────

fn zrbvcco_file_list_review(top: &Path, regulus_clone: &Path) {
    rbvcel_log::rbvcel_section("File-list review — the operator's own eyes (RBSHO step 3)");
    let clone = rbvcep_repo::rbvcep_as_str(regulus_clone);
    let files = rbvcer_run::rbvcer_capture("git", &["-C", &clone, "ls-files"], top);
    if files.code != 0 {
        crate::rbvcel_fatal!("git ls-files failed in the regulus:\n{}", files.stderr.trim());
    }
    rbvcel_log::rbvcel_raw(files.stdout.trim_end());
    rbvcel_log::rbvcel_line("no machine judgment substitutes for the maintainer reading what they are about to publish");
    rbvcel_log::rbvcel_confirm("reviewed the regulus's file list above?");
}

// ── Step 4: the kerygma (irreversible) ───────────────────

fn zrbvcco_kerygma(top: &Path, regulus_clone: &Path, regulus_tip: &str) {
    rbvcel_log::rbvcel_section("The kerygma (RBSHO step 4) — IRREVERSIBLE");
    let public_url = rbvcex_expede::RBVCEX_BASE_URL;
    let branch = rbvcex_expede::RBVCEX_REGULUS_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);

    let before = rbvcep_repo::rbvcep_ls_remote(public_url, top);
    let main_before = before.iter().find(|(_, name)| name == RBVCCO_MAIN_REF).map(|(sha, _)| sha.clone());

    let clone = rbvcep_repo::rbvcep_as_str(regulus_clone);
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_warn("POINT OF NO RETURN — a public object store cannot be un-disclosed.");
    rbvcel_log::rbvcel_line("Staging push line — type this yourself:");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_raw(&format!("        git -C {} push {} {}:{}", clone, public_url, branch, branch));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_confirm("pushed the staging push line above?");

    let after = rbvcep_repo::rbvcep_ls_remote(public_url, top);
    let main_after = after.iter().find(|(_, name)| name == RBVCCO_MAIN_REF).map(|(sha, _)| sha.clone());
    if main_before != main_after {
        crate::rbvcel_fatal!(
            "public main moved during the kerygma push ({:?} -> {:?}) — the staging push must never touch main",
            main_before, main_after
        );
    }
    let staged = after.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if !staged {
        crate::rbvcel_fatal!(
            "the public repository does not carry {} at the regulus tip {} after the reported push — resolve and re-run ostend",
            branch, regulus_tip
        );
    }
    rbvcel_log::rbvcel_line("disclosed: main untouched, POSTULANT_LOCAL stands at the regulus tip");
}

// ── Step 5: promotion (discoverability) ─────────────────────

fn zrbvcco_promotion(top: &Path, regulus_tip: &str) {
    rbvcel_log::rbvcel_section("Promotion (RBSHO step 5)");
    let public_url = rbvcex_expede::RBVCEX_BASE_URL;
    let branch = rbvcex_expede::RBVCEX_REGULUS_BRANCH;

    rbvcel_log::rbvcel_line("Promotion line — from a fresh clone or fetch of the public repository,");
    rbvcel_log::rbvcel_line("never the regulus directory. Type this yourself:");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_raw(&format!("        git push {} {}:main", public_url, branch));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_confirm("promoted (fast-forwarded main to the walked staging branch) above?");

    let after = rbvcep_repo::rbvcep_ls_remote(public_url, top);
    let main_sha = after.iter().find(|(_, name)| name == RBVCCO_MAIN_REF).map(|(sha, _)| sha.clone());
    match main_sha {
        Some(sha) if sha == regulus_tip => {
            rbvcel_log::rbvcel_line("promoted: public main equals the regulus tip — the byte claim is checked, not assumed");
        }
        Some(sha) => crate::rbvcel_fatal!(
            "public main is {} after the reported promotion, not the regulus tip {} — a refused fast-forward means main moved since the cut: STOP, never --force, re-cut atop the moved base",
            sha, regulus_tip
        ),
        None => crate::rbvcel_fatal!("public repository carries no main ref after the reported promotion"),
    }
}

// ── Close ────────────────────────────────────────────────────

fn zrbvcco_close() {
    rbvcel_log::rbvcel_section("Close the reveal (RBSHO close)");
    rbvcel_log::rbvcel_line("Hand-off: run the harbinger command for the confirmation coldwalk against promoted main.");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("Ceremony-hygiene reminders — your own hands, once dispositioned:");
    rbvcel_log::rbvcel_line("  - delete the public staging branch (POSTULANT_LOCAL) on the public repository");
    rbvcel_log::rbvcel_line("  - delete the private narthex repository");
    rbvcel_log::rbvcel_line("  - discard the regulus directory");
}
