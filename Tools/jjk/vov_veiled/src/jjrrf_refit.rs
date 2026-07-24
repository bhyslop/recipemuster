// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Refit — the session-facing staleness remedy (`jjdd_refit`, JJSVD-dispatch.adoc
//! "Refit"). Composes only `jjdf_farrier` primitives (`jjrfr_glean`,
//! `jjrfr_enfold`, `jjrfr_counterfoil`, `jjrfr_consign`) — this module owns no
//! primitive of its own, matching the dispatch sheaf's own posture.
//!
//! Refit fetches first, because staleness is fetch-revealed: the entrance spine
//! learns trunk has moved at its `glean` step, and refit is the remedy the open
//! then names. The merge input is therefore the trunk's remote counterpart, as
//! of that fetch (the `enfold` contract, farrier sheaf) — never the operator's
//! local trunk ref, which JJ neither reads nor handles.
//!
//! Then `consign` the merge immediately: the operation exists to clear staleness
//! for every station and runs online by construction. It never refuses on
//! staleness itself — only an orthogonal precondition (a dirty billet, at
//! `jjrfr_enfold`) can reject; staleness is drift, never corruption.
//!
//! An unreachable remote does not fail: refit merges what this station already
//! knows and pushes nothing (`jjrrf_RefitOutcome::OfflineWarned`). Offline it
//! cannot report `UpToDate`, even when the merge was a no-op — an unverified
//! no-op is ignorance, not freshness, and a remedy verb cannot afford to relabel
//! one as the other.
//!
//! The read-only "has trunk moved" probe that warns ahead of running refit
//! lives in the farrier itself (`jjrfr_outstripped`), not here — billet behind
//! `origin/<trunk>`, a local ancestry check after any glean. The dispatch
//! board surfaces it via `jjrds_staleness_notice` (dispatch spine), and
//! `jjx_open` leads its own report with the same notice
//! (`zjjrm_open_staleness_notice`, `jjrm_mcp.rs`). Wrap consults the same probe
//! but does not advise on it: at wrap entry the probe is a gate
//! (`jjrwp_staleness_gate`), and a stale billet meets an interdictum naming
//! refit rather than a notice. Notch remains unwired.

use crate::jjrfr_farrier::{
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_GleanOutcome,
    jjrfr_Rejection,
};
use std::path::Path;

/// Refit's outcome. Never a rejection kind of its own (JJSVD: "no verb refuses
/// on a stale trunk") — a genuine precondition failure (e.g. a dirty billet)
/// still surfaces through `jjrfr_enfold`'s own `jjrfr_Rejection`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrrf_RefitOutcome {
    /// The billet already held trunk's counterpart tip, verified against a
    /// successful fetch. An online-only verdict by construction.
    UpToDate,
    /// Enfolded the trunk counterpart in and consigned (pushed) the merge
    /// immediately.
    Refitted,
    /// The remote was unreachable. Refit merged the last-gleaned trunk position
    /// — possibly a no-op — and pushed nothing. The merge stands locally;
    /// freshness is unverified. Run again online.
    OfflineWarned,
}

/// Refit a billet: merge the counterpart of `trunk` into the billet branch
/// (never rebase), then push `branch` immediately.
///
/// `branch` is the billet's own line-of-work name and `trunk` the pedigree's
/// trunk branch name, both supplied by the caller (the dispatch door that
/// seated the billet knows them) rather than re-derived via `jjrfr_identify` —
/// refit is a small composition over explicit parameters, matching the farrier
/// primitives' own no-ambient-state discipline.
///
/// `jjrfr_glean` leads and carries two loads at once: it is the fetch that makes
/// the counterpart current, and — being farrier's one primitive that classifies
/// an unreachable remote as a normal outcome rather than a failure to unwrap —
/// it is also the reachability verdict that decides whether the push may follow.
pub fn jjrrf_refit<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    billet_root: &Path,
    branch: &str,
    trunk: &str,
) -> Result<jjrrf_RefitOutcome, jjrfr_Rejection> {
    let reachable = farrier.jjrfr_glean(billet_root) == jjrfr_GleanOutcome::Updated;

    let before = farrier.jjrfr_counterfoil(billet_root)?;
    farrier.jjrfr_enfold(billet_root, trunk)?;
    let after = farrier.jjrfr_counterfoil(billet_root)?;

    if !reachable {
        return Ok(jjrrf_RefitOutcome::OfflineWarned);
    }
    if before.members == after.members {
        return Ok(jjrrf_RefitOutcome::UpToDate);
    }
    farrier.jjrfr_consign(billet_root, branch)?;
    Ok(jjrrf_RefitOutcome::Refitted)
}
