// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — ostend: the reveal ceremony's irreversible showing (RBSHO). The
// once-per-cycle disclosure of the standing dry regulus under a granted
// cachet: the re-asserted ground, the operator's own file-list eyes, the
// disclosure, and promotion — every assert machine-performed, every push
// typed by the operator. Never re-cuts: the dry regulus is bit-for-bit
// what ships, and a terminal re-cut would break exactly that identity.
//
// Machine-asserts-around-human-pushes (RBSHC "The command seam"): every push
// is shown, never held — this module holds no cloud credential and spends
// nothing; its acts are remote reads and the operator's own pushes. No `git
// push` to any real remote lives in this file.
//
// Rehearse tolerates an absent cachet with a loud warning and stops before
// the disclosure line — the reversible stages proven, the irreversible ones
// never touched.

use std::path::Path;
use std::process::ExitCode;

use crate::rbthdr_cachet;
use crate::rbthdr_expede;
use crate::rbthdr_log;
use crate::rbthdr_repo;
use crate::rbthdr_run;

/// The disclosure and promotion target is the real public repository — the
/// same endpoint the cut clones read-only (rbthdr_expede::RBTHDR_BASE_URL).
const RBTHDR_MAIN_REF: &str = "refs/heads/main";

/// Conduct the ostend. `rehearse` proves the reversible stages (cachet
/// tolerant, re-assert the ground, the file-list review) and stops before the
/// disclosure line — no push shown, nothing irreversible touched. Fatal on
/// any deficit; ExitCode::SUCCESS only when, outside rehearse, the
/// disclosure and promotion both verified by remote read.
pub fn rbthdr_ostend_conduct(rehearse: bool) -> ExitCode {
    rbthdr_log::rbthdr_section("Hierophant Ostend — the reveal's irreversible showing (RBSHO)");
    if rehearse {
        rbthdr_log::rbthdr_line("REHEARSAL — reversible stages only: stops before the disclosure line.");
    }

    let top = rbthdr_repo::rbthdr_toplevel();
    let parent = rbthdr_repo::rbthdr_parent(&top);
    rbthdr_log::rbthdr_line(&format!("Maintainer tree: {}", top.display()));

    let regulus_parent = parent.join(rbthdr_repo::RBTHDR_REGULUS_DIRNAME);
    let regulus_clone = regulus_parent.join(rbthdr_repo::RBTHDR_REGULUS_SUBDIR);
    if !regulus_clone.is_dir() {
        crate::rbthdr_fatal!(
            "no standing regulus at {} — run essai first (RBSHE)",
            regulus_clone.display()
        );
    }
    let regulus_tip = rbthdr_repo::rbthdr_commit_sha(&regulus_clone, &top);
    rbthdr_log::rbthdr_line(&format!("Standing regulus: {} (tip {})", regulus_clone.display(), regulus_tip));

    zrbthdr_require_cachet(&regulus_parent, &regulus_clone, &top, rehearse);
    zrbthdr_reassert_ground(&top, &parent, &regulus_clone, &regulus_tip);
    zrbthdr_file_list_review(&top, &regulus_clone);

    if rehearse {
        rbthdr_log::rbthdr_blank();
        rbthdr_log::rbthdr_success("Ostend rehearsal complete — cachet checked, ground re-asserted, file list reviewed. Stopped before the disclosure line.");
        return ExitCode::SUCCESS;
    }

    zrbthdr_disclosure(&top, &regulus_clone, &regulus_tip);
    zrbthdr_promotion(&top, &regulus_tip);
    zrbthdr_close();

    rbthdr_log::rbthdr_success("Ostend complete — disclosed and promoted, every assert machine-performed, every push human-typed (RBSHO completion).");
    ExitCode::SUCCESS
}

// ── Step 1: require the cachet ──────────────────────────────

fn zrbthdr_require_cachet(regulus_parent: &Path, regulus_clone: &Path, top: &Path, rehearse: bool) {
    rbthdr_log::rbthdr_section("Require the cachet (RBSHO step 1)");
    if rehearse {
        rbthdr_cachet::rbthdr_require_rehearse(regulus_parent, regulus_clone, top);
    } else {
        rbthdr_cachet::rbthdr_require(regulus_parent, regulus_clone, top);
    }
}

// ── Step 2: re-assert the ground ────────────────────────────

fn zrbthdr_reassert_ground(top: &Path, parent: &Path, regulus_clone: &Path, regulus_tip: &str) {
    rbthdr_log::rbthdr_section("Re-assert the ground (RBSHO step 2)");

    rbthdr_expede::rbthdr_assert_narthex_private(top);

    let branch = rbthdr_expede::RBTHDR_REGULUS_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);
    let refs = rbthdr_repo::rbthdr_ls_remote(rbthdr_expede::RBTHDR_NARTHEX_URL, top);
    let preview_stands = refs.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if !preview_stands {
        crate::rbthdr_fatal!(
            "the narthex's {} tip does not equal the regulus tip {} — the preview does not stand; the cycle returns to essai, never forward",
            branch, regulus_tip
        );
    }
    rbthdr_log::rbthdr_line("preview stands: narthex tip equals the regulus tip");

    rbthdr_expede::rbthdr_assert_fresh(top, parent, regulus_clone);
}

// ── Step 3: file-list review — the operator's own eyes ──────

fn zrbthdr_file_list_review(top: &Path, regulus_clone: &Path) {
    rbthdr_log::rbthdr_section("File-list review — the operator's own eyes (RBSHO step 3)");
    let clone = rbthdr_repo::rbthdr_as_str(regulus_clone);
    let files = rbthdr_run::rbthdr_capture("git", &["-C", &clone, "ls-files"], top);
    if files.code != 0 {
        crate::rbthdr_fatal!("git ls-files failed in the regulus:\n{}", files.stderr.trim());
    }
    rbthdr_log::rbthdr_raw(files.stdout.trim_end());
    rbthdr_log::rbthdr_line("no machine judgment substitutes for the maintainer reading what they are about to publish");
    rbthdr_log::rbthdr_confirm("reviewed the regulus's file list above?");
}

// ── Step 4: the disclosure (irreversible) ───────────────────

fn zrbthdr_disclosure(top: &Path, regulus_clone: &Path, regulus_tip: &str) {
    rbthdr_log::rbthdr_section("The disclosure (RBSHO step 4) — IRREVERSIBLE");
    let public_url = rbthdr_expede::RBTHDR_BASE_URL;
    let branch = rbthdr_expede::RBTHDR_REGULUS_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);

    let before = rbthdr_repo::rbthdr_ls_remote(public_url, top);
    let main_before = before.iter().find(|(_, name)| name == RBTHDR_MAIN_REF).map(|(sha, _)| sha.clone());

    let clone = rbthdr_repo::rbthdr_as_str(regulus_clone);
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_warn("POINT OF NO RETURN — a public object store cannot be un-disclosed.");
    rbthdr_log::rbthdr_line("Staging push line — type this yourself:");
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_raw(&format!("        git -C {} push {} {}:{}", clone, public_url, branch, branch));
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_confirm("pushed the staging push line above?");

    let after = rbthdr_repo::rbthdr_ls_remote(public_url, top);
    let main_after = after.iter().find(|(_, name)| name == RBTHDR_MAIN_REF).map(|(sha, _)| sha.clone());
    if main_before != main_after {
        crate::rbthdr_fatal!(
            "public main moved during the disclosure push ({:?} -> {:?}) — the staging push must never touch main",
            main_before, main_after
        );
    }
    let staged = after.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if !staged {
        crate::rbthdr_fatal!(
            "the public repository does not carry {} at the regulus tip {} after the reported push — resolve and re-run ostend",
            branch, regulus_tip
        );
    }
    rbthdr_log::rbthdr_line("disclosed: main untouched, POSTULANT_LOCAL stands at the regulus tip");
}

// ── Step 5: promotion (discoverability) ─────────────────────

fn zrbthdr_promotion(top: &Path, regulus_tip: &str) {
    rbthdr_log::rbthdr_section("Promotion (RBSHO step 5)");
    let public_url = rbthdr_expede::RBTHDR_BASE_URL;
    let branch = rbthdr_expede::RBTHDR_REGULUS_BRANCH;

    rbthdr_log::rbthdr_line("Promotion line — from a fresh clone or fetch of the public repository,");
    rbthdr_log::rbthdr_line("never the regulus directory. Type this yourself:");
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_raw(&format!("        git push {} {}:main", public_url, branch));
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_confirm("promoted (fast-forwarded main to the walked staging branch) above?");

    let after = rbthdr_repo::rbthdr_ls_remote(public_url, top);
    let main_sha = after.iter().find(|(_, name)| name == RBTHDR_MAIN_REF).map(|(sha, _)| sha.clone());
    match main_sha {
        Some(sha) if sha == regulus_tip => {
            rbthdr_log::rbthdr_line("promoted: public main equals the regulus tip — the byte claim is checked, not assumed");
        }
        Some(sha) => crate::rbthdr_fatal!(
            "public main is {} after the reported promotion, not the regulus tip {} — a refused fast-forward means main moved since the cut: STOP, never --force, re-cut atop the moved base",
            sha, regulus_tip
        ),
        None => crate::rbthdr_fatal!("public repository carries no main ref after the reported promotion"),
    }
}

// ── Close ────────────────────────────────────────────────────

fn zrbthdr_close() {
    rbthdr_log::rbthdr_section("Close the reveal (RBSHO close)");
    rbthdr_log::rbthdr_line("Hand-off: run the harbinger command for the confirmation coldwalk against promoted main.");
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_line("Ceremony-hygiene reminders — your own hands, once dispositioned:");
    rbthdr_log::rbthdr_line("  - delete the public staging branch (POSTULANT_LOCAL) on the public repository");
    rbthdr_log::rbthdr_line("  - delete the private narthex repository");
    rbthdr_log::rbthdr_line("  - discard the regulus directory");
}
