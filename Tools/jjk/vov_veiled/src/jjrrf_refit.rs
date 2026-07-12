// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Refit — the session-facing staleness remedy (`jjdd_refit`, JJSVD-dispatch.adoc
//! "Refit"). Composes only `jjdf_farrier` primitives (`jjrfr_enfold`,
//! `jjrfr_consign`, `jjrfr_glean`, `jjrfr_counterfoil`) — this module owns no
//! primitive of its own, matching the dispatch sheaf's own posture.
//!
//! Behavior per the paddock's trunk-resync cinch: merge trunk into the billet
//! (never rebase — `jjrfr_enfold` already refuses to rebase), then push the
//! merge immediately. The operation never refuses on staleness itself — only
//! an orthogonal precondition (a dirty billet, at `jjrfr_enfold`) can reject;
//! staleness is drift, never corruption. An unreachable remote at push time
//! degrades to a warning rather than a failure: the merge already landed
//! locally (the billet branch is additive and the operation exists to clear
//! staleness "for every station," which is only ever true when online).
//!
//! Deliberately not built here: a read-only "has trunk moved" probe usable
//! before committing to a merge. The paddock flags that composition as still
//! aspirant (it wants pedigree/sire infrastructure this heat does not build),
//! so `jjx_open`/notch/wrap surfacing the staleness warning ahead of running
//! refit is left to the dispatch spine (`jjdd_spine`) or the conversion heat.

use crate::jjrfr_farrier::{
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_GleanOutcome,
    jjrfr_Rejection,
};
use std::path::Path;

/// Refit's outcome. Never a rejection kind of its own (JJSVD: "no verb
/// refuses on a stale trunk") — a genuine precondition failure (e.g. a dirty
/// billet) still surfaces through `jjrfr_enfold`'s own `jjrfr_Rejection`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrrf_RefitOutcome {
    /// The billet already held trunk's tip — enfold was a no-op.
    UpToDate,
    /// Enfolded trunk in and consigned (pushed) the merge immediately.
    Refitted,
    /// Enfolded trunk in; the push could not reach the remote. The merge
    /// stands locally — offline degrades to warn-and-proceed, never a
    /// failure.
    OfflineWarned,
}

/// Refit a billet: merge `trunk` into the billet branch (never rebase), then
/// push `branch` immediately unless the remote is unreachable.
///
/// `branch` is the billet's own line-of-work name, supplied by the caller
/// (the dispatch door that seated the billet already knows it) rather than
/// re-derived via `jjrfr_identify` — refit is a small composition over
/// explicit parameters, matching the farrier primitives' own no-ambient-state
/// discipline.
///
/// Connectivity is decided by `jjrfr_glean`, farrier's own opportunistic,
/// never-blocks-on-network primitive: an unreachable remote there is a
/// normal outcome, not a failure to unwrap, so it is the sanctioned way to
/// preflight `jjrfr_consign` (which has no offline classification of its
/// own). Glean only runs when there is something to push — an up-to-date
/// billet never touches the network.
pub fn jjrrf_refit<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    billet_root: &Path,
    branch: &str,
    trunk: &str,
) -> Result<jjrrf_RefitOutcome, jjrfr_Rejection> {
    let before = farrier.jjrfr_counterfoil(billet_root)?;
    farrier.jjrfr_enfold(billet_root, trunk)?;
    let after = farrier.jjrfr_counterfoil(billet_root)?;
    if before.members == after.members {
        return Ok(jjrrf_RefitOutcome::UpToDate);
    }
    match farrier.jjrfr_glean(billet_root) {
        jjrfr_GleanOutcome::Unreachable => Ok(jjrrf_RefitOutcome::OfflineWarned),
        jjrfr_GleanOutcome::Updated => {
            farrier.jjrfr_consign(billet_root, branch, None)?;
            Ok(jjrrf_RefitOutcome::Refitted)
        }
    }
}
