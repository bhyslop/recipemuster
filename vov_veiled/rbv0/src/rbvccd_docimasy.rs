// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — docimasy: the reveal ceremony's reversible proving act (RBSHD).
// Narthex gates, freshness assert, the preview conduction, credential
// preflight, and the once-per-cycle gauntlet stage — granting the cachet on
// green. Named for the Athenian civic examination of a candidate's
// qualification before taking office.
//
// Workers are sequenced as subprocesses, never re-implemented (RBSHC), mirroring
// rbvcce_essai's worker-sequencing and tabtarget-location shape. Every push is
// shown, never held (RBSHC "Structural incapacity"): the operator types it, this
// module re-verifies by remote read after — no `git push` to any real remote
// lives in this file.
//
// Rehearse proves the reversible stages against the real private narthex:
// it skips credential preflight and the gauntlet, and grants no cachet.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::rbvcet_cachet;
use crate::rbvcex_expede;
use crate::rbvcel_log;
use crate::rbvcep_repo;
use crate::rbvcer_run;

/// Tabtarget colophon prefixes this command sequences, on the MAINTAINER
/// tree — credential preflight and the gauntlet ladder are host/session-level
/// and source-tree-only concerns (rbw-MZ's own gate demands an upstream and a
/// pushed HEAD, and its own completeness gate is explicitly source-only: "a
/// stripped consumer never has this tabtarget and never runs it"), never the
/// severed, remote-less regulus clone.
const RBVCCD_COL_PAYOR_CHECK: &str = "rbw-ap.";
const RBVCCD_COL_NOVATE_SEDERUNT: &str = "rbw-aN.";
const RBVCCD_COL_MARSHAL_ZERO: &str = "rbw-MZ.";
pub(crate) const RBVCCD_COL_SUITE: &str = "rbw-ts.";
const RBVCCD_SUITE_GAUNTLET: &str = "gauntlet";

/// The `tt/` subdirectory holding tabtargets, relative to a repo root.
pub(crate) const RBVCCD_TT_SUBDIR: &str = "tt";

/// Conduct the docimasy. `rehearse` proves the reversible stages (narthex
/// gate, freshness, preview) against the real private narthex, skipping
/// credential preflight, the gauntlet, and the cachet grant. Fatal on any
/// deficit; ExitCode::SUCCESS only when the standing regulus is previewed,
/// and — outside rehearse — the gauntlet ran green and a cachet stands.
pub fn rbvccd_docimasy_conduct(rehearse: bool) -> ExitCode {
    rbvcel_log::rbvcel_section("Hierophant Docimasy — the reveal's reversible proving act (RBSHD)");
    if rehearse {
        rbvcel_log::rbvcel_line("REHEARSAL — reversible stages only: no credential spend, no gauntlet, no cachet granted.");
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

    zrbvccd_gate_narthex(&top, &regulus_tip);
    rbvcex_expede::rbvcex_assert_fresh(&top, &parent, &regulus_clone);
    zrbvccd_preview(&top, &regulus_clone, &regulus_tip);

    if rehearse {
        rbvcel_log::rbvcel_blank();
        rbvcel_log::rbvcel_success("Docimasy rehearsal complete — narthex gated, freshness proven, preview stands. No cachet granted.");
        return ExitCode::SUCCESS;
    }

    zrbvccd_credential_preflight(&top);
    zrbvccd_gauntlet_stage(&top);

    rbvcel_log::rbvcel_section("Grant the cachet (RBSHD grant step)");
    rbvcet_cachet::rbvcet_grant(&regulus_parent, &regulus_clone, &top);

    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("Hand-off: the reveal's irreversible act is now admissible — run ostend.");
    rbvcel_log::rbvcel_success("Docimasy complete — regulus previewed, gauntlet green, cachet granted (RBSHD completion).");

    ExitCode::SUCCESS
}

// ── Step 1: gate the narthex ─────────────────────────────

fn zrbvccd_gate_narthex(top: &Path, regulus_tip: &str) {
    rbvcel_log::rbvcel_section("Gate the narthex (RBSHD step 1)");

    rbvcex_expede::rbvcex_assert_narthex_private(top);

    let refs = rbvcep_repo::rbvcep_ls_remote(rbvcex_expede::RBVCEX_NARTHEX_URL, top);
    let branch_ref = format!("refs/heads/{}", rbvcex_expede::RBVCEX_REGULUS_BRANCH);
    let fresh = refs.is_empty()
        || (refs.len() == 1 && refs[0].1 == branch_ref && refs[0].0 == regulus_tip);
    if !fresh {
        crate::rbvcel_fatal!(
            "the narthex is not fresh — it carries {} ref(s) other than this cut's own {}, an undispositioned prior cut the operator must dispose of:\n{}",
            refs.len(),
            rbvcex_expede::RBVCEX_REGULUS_BRANCH,
            refs.iter().map(|(sha, name)| format!("  {} {}", sha, name)).collect::<Vec<_>>().join("\n")
        );
    }
    rbvcel_log::rbvcel_line("narthex fresh: no refs, or exactly this cut's own preview");
}

// ── Step 2 is rbvcex_expede::rbvcex_assert_fresh, called directly by conduct ──

// ── Step 3: preview into the narthex (reversible) ────────

fn zrbvccd_preview(top: &Path, regulus_clone: &Path, regulus_tip: &str) {
    rbvcel_log::rbvcel_section("Preview into the narthex (RBSHD step 3)");
    let branch = rbvcex_expede::RBVCEX_REGULUS_BRANCH;
    let branch_ref = format!("refs/heads/{}", branch);

    let refs = rbvcep_repo::rbvcep_ls_remote(rbvcex_expede::RBVCEX_NARTHEX_URL, top);
    let already = refs.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if already {
        rbvcel_log::rbvcel_line("narthex already previews this regulus tip — the preview line is not re-typed");
        return;
    }

    let clone = rbvcep_repo::rbvcep_as_str(regulus_clone);
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_line("Preview push line — type this yourself:");
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_raw(&format!("        git -C {} push {} {}:{}", clone, rbvcex_expede::RBVCEX_NARTHEX_URL, branch, branch));
    rbvcel_log::rbvcel_blank();
    rbvcel_log::rbvcel_confirm("pushed the preview line above?");

    let refs = rbvcep_repo::rbvcep_ls_remote(rbvcex_expede::RBVCEX_NARTHEX_URL, top);
    let landed = refs.iter().any(|(sha, name)| name == &branch_ref && sha == regulus_tip);
    if !landed {
        crate::rbvcel_fatal!(
            "the narthex's {} tip does not equal the regulus tip {} after the reported push — resolve and re-run docimasy",
            branch, regulus_tip
        );
    }
    rbvcel_log::rbvcel_line("narthex previews the regulus: tip verified by remote read");
}

// ── Step 4: credential preflight (skipped under rehearse) ───

fn zrbvccd_credential_preflight(top: &Path) {
    rbvcel_log::rbvcel_section("Credential preflight (RBSHD step 4)");
    let tt = top.join(RBVCCD_TT_SUBDIR);

    let check = zrbvccd_find_tt(&tt, RBVCCD_COL_PAYOR_CHECK, None);
    zrbvccd_require(rbvcer_run::rbvcer_stream(&check, &[], top, &[]), "payor credential check");

    let novate = zrbvccd_find_tt(&tt, RBVCCD_COL_NOVATE_SEDERUNT, None);
    zrbvccd_require(rbvcer_run::rbvcer_stream(&novate, &[], top, &[]), "sederunt novation");

    rbvcel_log::rbvcel_line("payor credential live, sederunt fresh — the gauntlet's build verbs are runway-gated");
}

// ── Step 5: run the gauntlet stage (skipped under rehearse) ─

fn zrbvccd_gauntlet_stage(top: &Path) {
    rbvcel_log::rbvcel_section("Run the gauntlet stage (RBSHD step 5)");
    rbvcel_log::rbvcel_warn("Marshal zero blanks the maintainer tree's regime and auto-commits; the ladder costs about an hour and two GCP projects.");
    rbvcel_log::rbvcel_confirm("proceed with marshal zero and the gauntlet ladder?");

    let tt = top.join(RBVCCD_TT_SUBDIR);
    let basename = top
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| crate::rbvcel_fatal!("cannot read the maintainer tree's basename: {}", top.display()));

    let zero = zrbvccd_find_tt(&tt, RBVCCD_COL_MARSHAL_ZERO, None);
    zrbvccd_require(rbvcer_run::rbvcer_stream(&zero, &[basename], top, &[]), "marshal zero");

    let suite = zrbvccd_find_tt(&tt, RBVCCD_COL_SUITE, Some(RBVCCD_SUITE_GAUNTLET));
    zrbvccd_require(rbvcer_run::rbvcer_stream(&suite, &[], top, &[]), "gauntlet suite");

    rbvcel_log::rbvcel_line("gauntlet green — the marshal-zero baseline qualified end to end");
}

// ── Small shared helpers (mirrors rbvcce_essai's shape) ─────

/// Locate the single tabtarget under `tt` whose name starts with `colophon`
/// and, if given, embeds `.{imprint}.`. Fatal on zero or multiple matches.
pub(crate) fn zrbvccd_find_tt(tt: &Path, colophon: &str, imprint: Option<&str>) -> PathBuf {
    let entries = std::fs::read_dir(tt)
        .unwrap_or_else(|e| crate::rbvcel_fatal!("cannot read tabtarget dir {}: {}", tt.display(), e));
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
        0 => crate::rbvcel_fatal!(
            "no tabtarget '{}*{}' under {}",
            colophon,
            imprint.map(|i| format!("{}.", i)).unwrap_or_default(),
            tt.display()
        ),
        n => crate::rbvcel_fatal!(
            "ambiguous: {} tabtargets match '{}*{}' under {}",
            n,
            colophon,
            imprint.map(|i| format!("{}.", i)).unwrap_or_default(),
            tt.display()
        ),
    }
}

/// A red worker: the fault is on the maintainer tree or its credentials.
fn zrbvccd_require(code: i32, what: &str) {
    if code != 0 {
        crate::rbvcel_fatal!("{} failed (exit {}) — resolve on the maintainer tree, then run docimasy again (RBSHD)", what, code);
    }
}
