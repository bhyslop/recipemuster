// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — ostend: the reveal ceremony's irreversible showing (RBSHO). The
// once-per-cycle disclosure of the standing dry candidate under a granted
// cachet: the re-asserted ground, the operator's own file-list eyes, the
// disclosure, and promotion — every assert machine-performed, every push
// typed by the operator. Never re-cuts: the dry candidate is bit-for-bit
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

/// The quarantine, shared with docimasy's own gate (RBS0 rbth_quarantine).
const RBTHDR_QUARANTINE_URL: &str = "git@github.com:scaleinv/recipebottle-staging.git";
const RBTHDR_QUARANTINE_HTTPS: &str = "https://github.com/scaleinv/recipebottle-staging";
const RBTHDR_QUARANTINE_PRIVATE_STATUS: &str = "404";

/// The disclosure and promotion target is the real public repository — the
/// same endpoint the cut clones read-only (rbthdr_expede::RBTHDR_BASE_URL).
const RBTHDR_MAIN_REF: &str = "refs/heads/main";

/// Conduct the ostend. `rehearse` proves the reversible stages (cachet
/// tolerant, re-assert the ground, the file-list review) and stops before the
/// disclosure line — no push shown, nothing irreversible touched. Fatal on
/// any deficit; ExitCode::SUCCESS only when, outside rehearse, the
/// disclosure and promotion both verified by remote read.
pub fn conduct(rehearse: bool) -> ExitCode {
    rbthdr_log::section("Hierophant Ostend — the reveal's irreversible showing (RBSHO)");
    if rehearse {
        rbthdr_log::line("REHEARSAL — reversible stages only: stops before the disclosure line.");
    }

    let top = rbthdr_repo::toplevel();
    let parent = rbthdr_repo::parent(&top);
    rbthdr_log::line(&format!("Maintainer tree: {}", top.display()));

    let candidate_parent = parent.join(rbthdr_repo::RBTHDR_CANDIDATE_DIRNAME);
    let candidate_clone = candidate_parent.join(rbthdr_repo::RBTHDR_CANDIDATE_SUBDIR);
    if !candidate_clone.is_dir() {
        crate::rbthdr_fatal!(
            "no standing candidate at {} — run essai first (RBSHE)",
            candidate_clone.display()
        );
    }
    let candidate_tip = rbthdr_repo::commit_sha(&candidate_clone, &top);
    rbthdr_log::line(&format!("Standing candidate: {} (tip {})", candidate_clone.display(), candidate_tip));

    zrbthdr_require_cachet(&candidate_parent, &candidate_clone, &top, rehearse);
    zrbthdr_reassert_ground(&top, &parent, &candidate_clone, &candidate_tip);
    zrbthdr_file_list_review(&top, &candidate_clone);

    if rehearse {
        rbthdr_log::blank();
        rbthdr_log::success("Ostend rehearsal complete — cachet checked, ground re-asserted, file list reviewed. Stopped before the disclosure line.");
        return ExitCode::SUCCESS;
    }

    zrbthdr_disclosure(&top, &candidate_clone, &candidate_tip);
    zrbthdr_promotion(&top, &candidate_tip);
    zrbthdr_close();

    rbthdr_log::success("Ostend complete — disclosed and promoted, every assert machine-performed, every push human-typed (RBSHO completion).");
    ExitCode::SUCCESS
}

// ── Step 1: require the cachet ──────────────────────────────

fn zrbthdr_require_cachet(candidate_parent: &Path, candidate_clone: &Path, top: &Path, rehearse: bool) {
    rbthdr_log::section("Require the cachet (RBSHO step 1)");
    if rehearse {
        rbthdr_cachet::require_rehearse(candidate_parent, candidate_clone, top);
    } else {
        rbthdr_cachet::require(candidate_parent, candidate_clone, top);
    }
}

// ── Step 2: re-assert the ground ────────────────────────────

fn zrbthdr_reassert_ground(top: &Path, parent: &Path, candidate_clone: &Path, candidate_tip: &str) {
    rbthdr_log::section("Re-assert the ground (RBSHO step 2)");

    let status = rbthdr_run::capture(
        "curl",
        &["-s", "-o", "/dev/null", "-w", "%{http_code}", RBTHDR_QUARANTINE_HTTPS],
        top,
    );
    if status.code != 0 {
        crate::rbthdr_fatal!("anonymous read of the quarantine failed to execute (curl exited {})", status.code);
    }
    if status.stdout.trim() != RBTHDR_QUARANTINE_PRIVATE_STATUS {
        crate::rbthdr_fatal!(
            "anonymous read of the quarantine ({}) returned HTTP {}, not {} — the quarantine is public or misnamed",
            RBTHDR_QUARANTINE_HTTPS, status.stdout.trim(), RBTHDR_QUARANTINE_PRIVATE_STATUS
        );
    }
    rbthdr_log::line("quarantine reads anonymous-404: private");

    let branch = rbthdr_expede::RBTHDR_CANDIDATE_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);
    let refs = rbthdr_repo::ls_remote(RBTHDR_QUARANTINE_URL, top);
    let preview_stands = refs.iter().any(|(sha, name)| name == &branch_ref && sha == candidate_tip);
    if !preview_stands {
        crate::rbthdr_fatal!(
            "the quarantine's {} tip does not equal the candidate tip {} — the preview does not stand; the cycle returns to essai, never forward",
            branch, candidate_tip
        );
    }
    rbthdr_log::line("preview stands: quarantine tip equals the candidate tip");

    rbthdr_expede::assert_fresh(top, parent, candidate_clone);
}

// ── Step 3: file-list review — the operator's own eyes ──────

fn zrbthdr_file_list_review(top: &Path, candidate_clone: &Path) {
    rbthdr_log::section("File-list review — the operator's own eyes (RBSHO step 3)");
    let clone = rbthdr_repo::as_str(candidate_clone);
    let files = rbthdr_run::capture("git", &["-C", &clone, "ls-files"], top);
    if files.code != 0 {
        crate::rbthdr_fatal!("git ls-files failed in the candidate:\n{}", files.stderr.trim());
    }
    rbthdr_log::raw(files.stdout.trim_end());
    rbthdr_log::line("no machine judgment substitutes for the maintainer reading what they are about to publish");
    rbthdr_log::confirm("reviewed the candidate's file list above?");
}

// ── Step 4: the disclosure (irreversible) ───────────────────

fn zrbthdr_disclosure(top: &Path, candidate_clone: &Path, candidate_tip: &str) {
    rbthdr_log::section("The disclosure (RBSHO step 4) — IRREVERSIBLE");
    let public_url = rbthdr_expede::RBTHDR_BASE_URL;
    let branch = rbthdr_expede::RBTHDR_CANDIDATE_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);

    let before = rbthdr_repo::ls_remote(public_url, top);
    let main_before = before.iter().find(|(_, name)| name == RBTHDR_MAIN_REF).map(|(sha, _)| sha.clone());

    let clone = rbthdr_repo::as_str(candidate_clone);
    rbthdr_log::blank();
    rbthdr_log::warn("POINT OF NO RETURN — a public object store cannot be un-disclosed.");
    rbthdr_log::line("Staging push line — type this yourself:");
    rbthdr_log::blank();
    rbthdr_log::raw(&format!("        git -C {} push {} {}:{}", clone, public_url, branch, branch));
    rbthdr_log::blank();
    rbthdr_log::confirm("pushed the staging push line above?");

    let after = rbthdr_repo::ls_remote(public_url, top);
    let main_after = after.iter().find(|(_, name)| name == RBTHDR_MAIN_REF).map(|(sha, _)| sha.clone());
    if main_before != main_after {
        crate::rbthdr_fatal!(
            "public main moved during the disclosure push ({:?} -> {:?}) — the staging push must never touch main",
            main_before, main_after
        );
    }
    let staged = after.iter().any(|(sha, name)| name == &branch_ref && sha == candidate_tip);
    if !staged {
        crate::rbthdr_fatal!(
            "the public repository does not carry {} at the candidate tip {} after the reported push — resolve and re-run ostend",
            branch, candidate_tip
        );
    }
    rbthdr_log::line("disclosed: main untouched, POSTULANT_LOCAL stands at the candidate tip");
}

// ── Step 5: promotion (discoverability) ─────────────────────

fn zrbthdr_promotion(top: &Path, candidate_tip: &str) {
    rbthdr_log::section("Promotion (RBSHO step 5)");
    let public_url = rbthdr_expede::RBTHDR_BASE_URL;
    let branch = rbthdr_expede::RBTHDR_CANDIDATE_BRANCH;

    rbthdr_log::line("Promotion line — from a fresh clone or fetch of the public repository,");
    rbthdr_log::line("never the candidate directory. Type this yourself:");
    rbthdr_log::blank();
    rbthdr_log::raw(&format!("        git push {} {}:main", public_url, branch));
    rbthdr_log::blank();
    rbthdr_log::confirm("promoted (fast-forwarded main to the walked staging branch) above?");

    let after = rbthdr_repo::ls_remote(public_url, top);
    let main_sha = after.iter().find(|(_, name)| name == RBTHDR_MAIN_REF).map(|(sha, _)| sha.clone());
    match main_sha {
        Some(sha) if sha == candidate_tip => {
            rbthdr_log::line("promoted: public main equals the candidate tip — the byte claim is checked, not assumed");
        }
        Some(sha) => crate::rbthdr_fatal!(
            "public main is {} after the reported promotion, not the candidate tip {} — a refused fast-forward means main moved since the cut: STOP, never --force, re-cut atop the moved base",
            sha, candidate_tip
        ),
        None => crate::rbthdr_fatal!("public repository carries no main ref after the reported promotion"),
    }
}

// ── Close ────────────────────────────────────────────────────

fn zrbthdr_close() {
    rbthdr_log::section("Close the reveal (RBSHO close)");
    rbthdr_log::line("Hand-off: run the harbinger command for the confirmation coldwalk against promoted main.");
    rbthdr_log::blank();
    rbthdr_log::line("Ceremony-hygiene reminders — your own hands, once dispositioned:");
    rbthdr_log::line("  - delete the public staging branch (POSTULANT_LOCAL) on the public repository");
    rbthdr_log::line("  - delete the private quarantine repository");
    rbthdr_log::line("  - discard the candidate directory");
}
