// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Refit door — the operator-invocable surface the staleness warning names as
//! its remedy (`jjdd_refit`, JJSVD-dispatch.adoc "Refit"). The engine
//! (`jjrrf_refit`) has been built and tested since its own pace; until this
//! module, nothing called it.
//!
//! Session-facing: resolves the tree at `cwd` — hippodrome or billet, per
//! `jjdf_identify` — to its own branch and its sire's pedigree trunk, exactly
//! as `zjjrm_open_staleness_notice` resolves the same tree to check it, then
//! hands both to `jjrrf_refit`. No caller-supplied parameters; there is
//! exactly one tree a session's `jjx_refit` call can mean, so `cwd` is the
//! whole input, captured once by the MCP door and passed in (the no-cwd rule
//! this crate's engines share, `jjrvb_blotter` `jjdb_studbook_config` doc).

use std::path::Path;

use crate::jjrds_stile::{
    jjrds_pedigree_lookup,
    JJRDS_KIND_PLAIN_GIT,
};
use crate::jjrfr_farrier::{
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_LineOfWork,
    jjrfr_Seat,
};
use crate::jjrrf_refit::{
    jjrrf_refit,
    jjrrf_RefitOutcome,
};
use crate::jjrvb_blotter::jjdb_studbook_config;

/// Run refit against the tree at `cwd`. Returns operator-facing text and an
/// exit code — 0 on any of refit's three outcomes (up-to-date, refitted,
/// offline-warned are all successful runs), 1 on a refusal (foreign ground,
/// a detached checkout, no upstream key, an unrecorded sire, or a farrier
/// rejection such as a dirty billet).
///
/// Unlike `zjjrm_open_staleness_notice`, which degrades silently on any of
/// these so it never blocks `jjx_open`, the door is an explicit operator
/// action: a resolution failure is the whole reason to invoke it, so it is
/// reported, not swallowed.
pub fn jjrrd_run_refit<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    cwd: &Path,
) -> (i32, String) {
    let identity = match farrier.jjrfr_identify(cwd) {
        Ok(id) => id,
        Err(e) => return (1, format!("jjx_refit: refused — {}", e)),
    };

    let branch = match &identity.line_of_work {
        jjrfr_LineOfWork::Branch(name) => name.clone(),
        jjrfr_LineOfWork::Detached(sha) => {
            return (
                1,
                format!(
                    "jjx_refit: refused — {} is a detached checkout at {}, not a branch; refit needs a branch to merge trunk into.",
                    identity.root.display(),
                    sha,
                ),
            );
        }
    };

    let hippodrome_root = match &identity.seat {
        jjrfr_Seat::Primary => identity.root.clone(),
        jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
    };
    let infield_root = match hippodrome_root.parent() {
        Some(p) => p.to_path_buf(),
        None => {
            return (
                1,
                format!(
                    "jjx_refit: refused — {} has no parent infield directory.",
                    hippodrome_root.display(),
                ),
            );
        }
    };
    let derived_key = match &identity.upstream_key {
        Some(k) => k.clone(),
        None => {
            return (
                1,
                format!(
                    "jjx_refit: refused — {} has no upstream key; refit needs a known sire to find the trunk.",
                    identity.root.display(),
                ),
            );
        }
    };

    let studbook = jjdb_studbook_config(&infield_root);
    let pedigree = match jjrds_pedigree_lookup(&studbook, &derived_key, JJRDS_KIND_PLAIN_GIT) {
        Ok(p) => p,
        Err(e) => return (1, format!("jjx_refit: refused — {}", e)),
    };

    match jjrrf_refit(farrier, &identity.root, &branch, &pedigree.trunk) {
        Ok(jjrrf_RefitOutcome::UpToDate) => (
            0,
            format!(
                "jjx_refit: up to date — {} already holds {}'s remote counterpart tip.",
                branch, pedigree.trunk,
            ),
        ),
        Ok(jjrrf_RefitOutcome::Refitted) => (
            0,
            format!(
                "jjx_refit: refitted — merged {}'s remote counterpart into {} and pushed.",
                pedigree.trunk, branch,
            ),
        ),
        Ok(jjrrf_RefitOutcome::OfflineWarned) => (
            0,
            format!(
                "jjx_refit: offline — merged the last-gleaned position of {} into {}; nothing pushed. Re-run refit once the remote is reachable.",
                pedigree.trunk, branch,
            ),
        ),
        Err(e) => (1, format!("jjx_refit: refused — {}", e)),
    }
}
