// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — docimasy: the reveal ceremony's reversible proving act (RBSHD).
// Quarantine gates, freshness assert, the preview conduction, credential
// preflight, and the once-per-cycle gauntlet stage — granting the cachet on
// green. Named for the Athenian civic examination of a candidate's
// qualification before taking office.
//
// Workers are sequenced as subprocesses, never re-implemented (RBSHC), mirroring
// rbthdr_essai's worker-sequencing and tabtarget-location shape. Every push is
// shown, never held (RBSHC "Structural incapacity"): the operator types it, this
// module re-verifies by remote read after — no `git push` to any real remote
// lives in this file.
//
// Rehearse proves the reversible stages against the real private quarantine:
// it skips credential preflight and the gauntlet, and grants no cachet.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::rbthdr_cachet;
use crate::rbthdr_expede;
use crate::rbthdr_log;
use crate::rbthdr_repo;
use crate::rbthdr_run;

/// Tabtarget colophon prefixes this command sequences, on the MAINTAINER
/// tree — credential preflight and the gauntlet ladder are host/session-level
/// and source-tree-only concerns (rbw-MZ's own gate demands an upstream and a
/// pushed HEAD, and its own completeness gate is explicitly source-only: "a
/// stripped consumer never has this tabtarget and never runs it"), never the
/// severed, remote-less regulus clone.
const RBTHDR_COL_PAYOR_CHECK: &str = "rbw-ap.";
const RBTHDR_COL_NOVATE_SEDERUNT: &str = "rbw-aN.";
const RBTHDR_COL_MARSHAL_ZERO: &str = "rbw-MZ.";
pub(crate) const RBTHDR_COL_SUITE: &str = "rbw-ts.";
const RBTHDR_SUITE_GAUNTLET: &str = "gauntlet";

/// The `tt/` subdirectory holding tabtargets, relative to a repo root.
pub(crate) const RBTHDR_TT_SUBDIR: &str = "tt";

/// Conduct the docimasy. `rehearse` proves the reversible stages (quarantine
/// gate, freshness, preview) against the real private quarantine, skipping
/// credential preflight, the gauntlet, and the cachet grant. Fatal on any
/// deficit; ExitCode::SUCCESS only when the standing regulus is previewed,
/// and — outside rehearse — the gauntlet ran green and a cachet stands.
pub fn rbthdr_docimasy_conduct(rehearse: bool) -> ExitCode {
    rbthdr_log::rbthdr_section("Hierophant Docimasy — the reveal's reversible proving act (RBSHD)");
    if rehearse {
        rbthdr_log::rbthdr_line("REHEARSAL — reversible stages only: no credential spend, no gauntlet, no cachet granted.");
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

    zrbthdr_gate_quarantine(&top, &regulus_tip);
    rbthdr_expede::rbthdr_assert_fresh(&top, &parent, &regulus_clone);
    zrbthdr_preview(&top, &regulus_clone, &regulus_tip);

    if rehearse {
        rbthdr_log::rbthdr_blank();
        rbthdr_log::rbthdr_success("Docimasy rehearsal complete — quarantine gated, freshness proven, preview stands. No cachet granted.");
        return ExitCode::SUCCESS;
    }

    zrbthdr_credential_preflight(&top);
    zrbthdr_gauntlet_stage(&top);

    rbthdr_log::rbthdr_section("Grant the cachet (RBSHD grant step)");
    rbthdr_cachet::rbthdr_grant(&regulus_parent, &regulus_clone, &top);

    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_line("Hand-off: the reveal's irreversible act is now admissible — run ostend.");
    rbthdr_log::rbthdr_success("Docimasy complete — regulus previewed, gauntlet green, cachet granted (RBSHD completion).");

    ExitCode::SUCCESS
}

// ── Step 1: gate the quarantine ─────────────────────────────

fn zrbthdr_gate_quarantine(top: &Path, regulus_tip: &str) {
    rbthdr_log::rbthdr_section("Gate the quarantine (RBSHD step 1)");

    rbthdr_expede::rbthdr_assert_quarantine_private(top);

    let refs = rbthdr_repo::rbthdr_ls_remote(rbthdr_expede::RBTHDR_QUARANTINE_URL, top);
    let branch_ref = format!("refs/heads/{}", rbthdr_expede::RBTHDR_REGULUS_BRANCH);
    let fresh = refs.is_empty()
        || (refs.len() == 1 && refs[0].1 == branch_ref && refs[0].0 == regulus_tip);
    if !fresh {
        crate::rbthdr_fatal!(
            "the quarantine is not fresh — it carries {} ref(s) other than this cut's own {}, an undispositioned prior cut the operator must dispose of:\n{}",
            refs.len(),
            rbthdr_expede::RBTHDR_REGULUS_BRANCH,
            refs.iter().map(|(sha, name)| format!("  {} {}", sha, name)).collect::<Vec<_>>().join("\n")
        );
    }
    rbthdr_log::rbthdr_line("quarantine fresh: no refs, or exactly this cut's own preview");
}

// ── Step 2 is rbthdr_expede::rbthdr_assert_fresh, called directly by conduct ──

// ── Step 3: preview into the quarantine (reversible) ────────

fn zrbthdr_preview(top: &Path, regulus_clone: &Path, regulus_tip: &str) {
    rbthdr_log::rbthdr_section("Preview into the quarantine (RBSHD step 3)");
    let branch = rbthdr_expede::RBTHDR_REGULUS_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);

    let refs = rbthdr_repo::rbthdr_ls_remote(rbthdr_expede::RBTHDR_QUARANTINE_URL, top);
    let already = refs.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if already {
        rbthdr_log::rbthdr_line("quarantine already previews this regulus tip — the preview line is not re-typed");
        return;
    }

    let clone = rbthdr_repo::rbthdr_as_str(regulus_clone);
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_line("Preview push line — type this yourself:");
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_raw(&format!("        git -C {} push {} {}:{}", clone, rbthdr_expede::RBTHDR_QUARANTINE_URL, branch, branch));
    rbthdr_log::rbthdr_blank();
    rbthdr_log::rbthdr_confirm("pushed the preview line above?");

    let refs = rbthdr_repo::rbthdr_ls_remote(rbthdr_expede::RBTHDR_QUARANTINE_URL, top);
    let landed = refs.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if !landed {
        crate::rbthdr_fatal!(
            "the quarantine's {} tip does not equal the regulus tip {} after the reported push — resolve and re-run docimasy",
            branch, regulus_tip
        );
    }
    rbthdr_log::rbthdr_line("quarantine previews the regulus: tip verified by remote read");
}

// ── Step 4: credential preflight (skipped under rehearse) ───

fn zrbthdr_credential_preflight(top: &Path) {
    rbthdr_log::rbthdr_section("Credential preflight (RBSHD step 4)");
    let tt = top.join(RBTHDR_TT_SUBDIR);

    let check = zrbthdr_find_tt(&tt, RBTHDR_COL_PAYOR_CHECK, None);
    zrbthdr_require(rbthdr_run::rbthdr_stream(&check, &[], top, &[]), "payor credential check");

    let novate = zrbthdr_find_tt(&tt, RBTHDR_COL_NOVATE_SEDERUNT, None);
    zrbthdr_require(rbthdr_run::rbthdr_stream(&novate, &[], top, &[]), "sederunt novation");

    rbthdr_log::rbthdr_line("payor credential live, sederunt fresh — the gauntlet's build verbs are runway-gated");
}

// ── Step 5: run the gauntlet stage (skipped under rehearse) ─

fn zrbthdr_gauntlet_stage(top: &Path) {
    rbthdr_log::rbthdr_section("Run the gauntlet stage (RBSHD step 5)");
    rbthdr_log::rbthdr_warn("Marshal zero blanks the maintainer tree's regime and auto-commits; the ladder costs about an hour and two GCP projects.");
    rbthdr_log::rbthdr_confirm("proceed with marshal zero and the gauntlet ladder?");

    let tt = top.join(RBTHDR_TT_SUBDIR);
    let basename = top
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| crate::rbthdr_fatal!("cannot read the maintainer tree's basename: {}", top.display()));

    let zero = zrbthdr_find_tt(&tt, RBTHDR_COL_MARSHAL_ZERO, None);
    zrbthdr_require(rbthdr_run::rbthdr_stream(&zero, &[basename], top, &[]), "marshal zero");

    let suite = zrbthdr_find_tt(&tt, RBTHDR_COL_SUITE, Some(RBTHDR_SUITE_GAUNTLET));
    zrbthdr_require(rbthdr_run::rbthdr_stream(&suite, &[], top, &[]), "gauntlet suite");

    rbthdr_log::rbthdr_line("gauntlet green — the marshal-zero baseline qualified end to end");
}

// ── Small shared helpers (mirrors rbthdr_essai's shape) ─────

/// Locate the single tabtarget under `tt` whose name starts with `colophon`
/// and, if given, embeds `.{imprint}.`. Fatal on zero or multiple matches.
pub(crate) fn zrbthdr_find_tt(tt: &Path, colophon: &str, imprint: Option<&str>) -> PathBuf {
    let entries = std::fs::read_dir(tt)
        .unwrap_or_else(|e| crate::rbthdr_fatal!("cannot read tabtarget dir {}: {}", tt.display(), e));
    let needle = imprint.map(|i| format!(".{}.", i));
    let mut hits: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with(colophon) || !name.ends_with(".sh") {
            continue;
        }
        if let Some(needle) = &needle {
            if !name.contains(needle.as_str()) {
                continue;
            }
        }
        hits.push(entry.path());
    }
    match hits.len() {
        1 => hits.remove(0),
        0 => crate::rbthdr_fatal!(
            "no tabtarget '{}*{}' under {}",
            colophon,
            imprint.map(|i| format!("{}.", i)).unwrap_or_default(),
            tt.display()
        ),
        n => crate::rbthdr_fatal!(
            "ambiguous: {} tabtargets match '{}*{}' under {}",
            n,
            colophon,
            imprint.map(|i| format!("{}.", i)).unwrap_or_default(),
            tt.display()
        ),
    }
}

/// A red worker: the fault is on the maintainer tree or its credentials.
fn zrbthdr_require(code: i32, what: &str) {
    if code != 0 {
        crate::rbthdr_fatal!("{} failed (exit {}) — resolve on the maintainer tree, then run docimasy again (RBSHD)", what, code);
    }
}
