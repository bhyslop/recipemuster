// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Muck — the operator-directed destroy door (`jjdd_muck`, JJSVD-dispatch.adoc
//! "Muck") for a named billet the {jjdd_stile_p} trailing step left standing:
//! dirty, stranded work aboard, or orphaned by a killed door. Operator-typed
//! like the {jjdd_cashier}, outside the stile's approach — no dispatch crosses
//! it, and nothing composes it. It is the constellation's one deliberate
//! data-loss surface.
//!
//! No retention window, no liveness join: the operator names the billet — a
//! `jjdw_yard` dirname, or the identity behind one — and the plan's report
//! (`jjrdm_plan`) is the occupancy evidence a human weighs before confirming
//! an arm and calling `jjrdm_reap`.
//!
//! Plan-then-confirm, never silent. The plan resolves the named billet and
//! reports: its kind and seat; the dirty paths by name (never a count); the
//! branch posture (commits not yet in remote custody named as loss, never as
//! advice); for a pace billet, the pace's own state read through the journal
//! sheaf's read bracket — an acting read, since the snapshot arms this
//! confirm — with the latest tack as evidence, never as gate. The removal is
//! `jjrfr_billet_remove` behind its explicit force — this door's confirmed
//! destroy is the force's only caller.
//!
//! The door clears the destroyed billet's own scratch sibling with it, and
//! nothing else — scratch dies with its billet, the {jjdd_stile} clearing a
//! passing billet's scratch on its own trailing step, this door clearing a
//! standing billet's on confirmed destroy. Scratch orphaned by any other
//! route (a killed door, a crash) is not this door's target and is left for
//! hand cleaning.

use crate::jjrds_stile::{
    jjrds_billet_identity,
    jjrds_type_target,
    jjrds_Target,
    JJRDS_SCRATCH_DIRNAME,
};
use crate::jjrfr_farrier::{
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_LockGuard,
    jjrfr_Rejection,
    jjrfr_RejectionKind,
    jjrfr_Seat,
    jjrfr_SyncState,
};
use crate::jjrvb_blotter::{
    jjdb_gallops_journal_load,
    jjdb_BlotterConfig,
};
use std::path::{Path, PathBuf};

// ---- Rejections ----

/// Muck's own rejection taxonomy: a composed farrier primitive's own kind, the
/// studbook's gallops copy being unreadable when pace evidence is fetched, or
/// the named target resolving to no billet or more than one.
#[derive(Debug)]
pub enum jjrdm_Rejection {
    Farrier(jjrfr_Rejection),
    GallopsUnreadable(String),
    NotFound { name: String, detail: String },
    Ambiguous { name: String, candidates: Vec<String> },
}

impl std::fmt::Display for jjrdm_Rejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // A stranded lock meets the operator HERE — so this is where the
            // cure must be named (JJSVD `jjdd_cashier`, Discoverability: a
            // cure the sufferer cannot find is not a cure).
            jjrdm_Rejection::Farrier(r) if r.kind == jjrfr_RejectionKind::LockHeld => write!(
                f,
                "{}\n  Another station holds the studbook lock. If it crashed, the lock is stranded.\n  \
                 Lock recovery is HUMAN-ONLY — an agent reading this reports it and stops. Operator,\n  \
                 by hand: sight with `tt/jjw-dc.SightLocks.sh`, cashier with `tt/jjw-dC.Cashier.sh`.",
                r
            ),
            jjrdm_Rejection::Farrier(r) => write!(f, "{}", r),
            jjrdm_Rejection::GallopsUnreadable(detail) => write!(f, "studbook gallops unreadable: {}", detail),
            jjrdm_Rejection::NotFound { name, detail } => write!(f, "muck: '{}' names no billet — {}", name, detail),
            jjrdm_Rejection::Ambiguous { name, candidates } => write!(
                f,
                "muck: '{}' names more than one billet — name the exact yard dirname:\n{}",
                name,
                candidates.iter().map(|c| format!("  {}", c)).collect::<Vec<_>>().join("\n")
            ),
        }
    }
}

impl From<jjrfr_Rejection> for jjrdm_Rejection {
    fn from(r: jjrfr_Rejection) -> Self {
        jjrdm_Rejection::Farrier(r)
    }
}

// ---- Kinds ----

/// Which kind of billet a `jjqb_*` dirname resolved to, from the identity
/// behind its serial: a pace billet seats a durable branch wearing the livery
/// badge; a groom billet is always detached and carries nothing durable
/// (`jjdd_billet`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrdm_Kind {
    Pace(String),
    Groom(String),
}

/// The pace evidence a pace billet's confirm carries — never a gate. `Unknown`
/// when the studbook carries no tack for this coronet; `Unavailable` when the
/// journal read itself could not be taken (the studbook unreachable, say) —
/// evidence that failed to arrive is reported, not allowed to block the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrdm_PaceEvidence {
    Tack { resolved: bool, state_label: String, silks: String, basis: String },
    Unknown,
    Unavailable(String),
}

/// The muck plan: the named billet resolved, and everything its confirm
/// needs — pure resolution, no mutation (JJSVD "Muck", Plan-then-confirm).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrdm_Plan {
    pub billet_root: PathBuf,
    pub billet_dirname: String,
    pub kind: jjrdm_Kind,
    pub primary_root: PathBuf,
    pub dirty_paths: Vec<PathBuf>,
    pub sync_state: jjrfr_SyncState,
    /// `Some` only for a pace billet — a groom billet has no pace to evidence.
    pub pace_evidence: Option<jjrdm_PaceEvidence>,
}

impl jjrdm_Plan {
    pub fn jjrdm_is_dirty(&self) -> bool {
        !self.dirty_paths.is_empty()
    }
}

/// The plan-then-confirm report (JJSVD `jjdd_muck`, Plan-then-confirm). This
/// function owns the format; a shell door prints this and asks.
pub fn jjrdm_report(plan: &jjrdm_Plan) -> String {
    let mut lines = Vec::new();

    let kind_str = match &plan.kind {
        jjrdm_Kind::Pace(c) => format!("pace billet {}{}", crate::jjrf_favor::JJRF_CORONET_PREFIX, c),
        jjrdm_Kind::Groom(fm) => format!("groom billet {}{}", crate::jjrf_favor::JJRF_FIREMARK_PREFIX, fm),
    };
    lines.push(format!("{}: {}", plan.billet_dirname, kind_str));

    // A groom ground is a git-free scratch dir with nothing durable aboard — no
    // seat, no branch, no pace to evidence. Its whole report is the one razed-whole
    // line; the git-derived lines below are the pace billet's alone.
    if let jjrdm_Kind::Groom(_) = &plan.kind {
        lines.push("  ground: git-free scratch — razed whole, nothing durable aboard".to_string());
        return lines.join("\n");
    }

    lines.push(format!("  seat:   partition of {}", plan.primary_root.display()));

    if plan.dirty_paths.is_empty() {
        lines.push("  tree:   clean".to_string());
    } else {
        lines.push("  tree:   DIRTY — destroy loses these paths:".to_string());
        for path in &plan.dirty_paths {
            lines.push(format!("            {}", path.display()));
        }
    }

    match plan.sync_state {
        jjrfr_SyncState::Tracking { ahead, behind } if ahead > 0 => {
            lines.push(format!("  branch: {} commit(s) not yet in remote custody — destroy loses them", ahead));
            let _ = behind;
        }
        jjrfr_SyncState::Tracking { behind, .. } => {
            lines.push(format!("  branch: even with remote custody ({} behind)", behind));
        }
        jjrfr_SyncState::Untracked => {
            lines.push("  branch: untracked — no remote counterpart to compare".to_string());
        }
    }

    match &plan.pace_evidence {
        Some(jjrdm_PaceEvidence::Tack { state_label, silks, basis, .. }) => {
            lines.push(format!(
                "  pace:   {} (silks {}, basis {}) — evidence only, never a gate",
                state_label, silks, basis
            ));
        }
        Some(jjrdm_PaceEvidence::Unknown) => lines.push("  pace:   unknown to the studbook — no tack recorded".to_string()),
        Some(jjrdm_PaceEvidence::Unavailable(detail)) => lines.push(format!("  pace:   state unavailable — {}", detail)),
        None => {}
    }

    lines.join("\n")
}

// ---- Resolution ----

/// Halter-type a billet dirname into its kind — a coronet is a pace billet, a
/// firemark a groom billet, typed by length exactly as identities are
/// everywhere. `None` for anything that is not a well-formed billet name.
fn zjjrdm_billet_kind(dirname: &str) -> Option<jjrdm_Kind> {
    let suffix = jjrds_billet_identity(dirname)?;
    match jjrds_type_target(suffix).ok()? {
        jjrds_Target::Coronet(c) => Some(jjrdm_Kind::Pace(c)),
        jjrds_Target::Firemark(f) => Some(jjrdm_Kind::Groom(f)),
    }
}

/// Resolve the operator's named target to exactly one billet under
/// `infield_root`: a literal yard dirname if one stands, else the identity
/// behind it (glyph-tolerant coronet or firemark) — scanned against every
/// standing billet's own identity, never a composed guess. Zero matches is
/// `NotFound`; more than one (a groom firemark shared by concurrent groom
/// billets, JJSVD "Catchword-serialed billets") is `Ambiguous`, naming every
/// candidate dirname so the operator can re-name the exact one.
fn zjjrdm_resolve_billet(infield_root: &Path, name: &str) -> Result<(PathBuf, String), jjrdm_Rejection> {
    let direct = infield_root.join(name);
    if direct.is_dir() && zjjrdm_billet_kind(name).is_some() {
        return Ok((direct, name.to_string()));
    }

    let target = jjrds_type_target(name).map_err(|e| jjrdm_Rejection::NotFound {
        name: name.to_string(),
        detail: e.to_string(),
    })?;

    let mut matches = Vec::new();
    if let Ok(entries) = std::fs::read_dir(infield_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let dirname = match path.file_name() {
                Some(n) => n.to_string_lossy().into_owned(),
                None => continue,
            };
            let kind = match zjjrdm_billet_kind(&dirname) {
                Some(k) => k,
                None => continue,
            };
            let hits = match (&kind, &target) {
                (jjrdm_Kind::Pace(c), jjrds_Target::Coronet(t)) => c == t,
                (jjrdm_Kind::Groom(fm), jjrds_Target::Firemark(t)) => fm == t,
                _ => false,
            };
            if hits {
                matches.push(dirname);
            }
        }
    }

    match matches.len() {
        0 => Err(jjrdm_Rejection::NotFound {
            name: name.to_string(),
            detail: format!("no billet under {} carries that identity", infield_root.display()),
        }),
        1 => {
            let dirname = matches.remove(0);
            let root = infield_root.join(&dirname);
            Ok((root, dirname))
        }
        _ => Err(jjrdm_Rejection::Ambiguous { name: name.to_string(), candidates: matches }),
    }
}

/// Read the named pace's latest tack through the journal sheaf's read bracket
/// (JJSVJ "The read bracket"): glean, stake, advance, load, release — the
/// same acting read `jjdb_journal` takes, so what this reads is the store's
/// truth at this moment. `Ok(None)` when the studbook carries no tack for
/// this coronet at all.
fn zjjrdm_snapshot_tack<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    coronet: &str,
    guidon: &str,
) -> Result<Option<crate::jjrt_types::jjrg_Tack>, jjrdm_Rejection> {
    let root = studbook.local_root.as_path();
    // Probe FIRST, like `jjrdc_sight_store`: every farrier op below presumes a
    // legitimate git-claimed tree, and panics on an unclassified failure
    // otherwise — a station that never founded its studbook has no clone to
    // stake, and that is an honest "unavailable", never a crash.
    if !root.exists() {
        return Err(jjrdm_Rejection::GallopsUnreadable(format!("no studbook clone at {}", root.display())));
    }
    let _ = farrier.jjrfr_glean(root);
    let guard = jjrfr_LockGuard::jjrfr_acquire(farrier, root, guidon)?;
    let sighted = farrier.jjrfr_sight(root)?;
    if sighted.as_deref() != Some(guard.jjrfr_guidon()) {
        panic!(
            "jjrdm_plan: sight after stake did not confirm our own guidon at {} (expected {:?}, saw {:?})",
            root.display(),
            guard.jjrfr_guidon(),
            sighted
        );
    }
    farrier.jjrfr_advance(root)?;
    let gallops = jjdb_gallops_journal_load(studbook)
        .map_err(jjrdm_Rejection::GallopsUnreadable)?
        .into_inner();
    drop(guard);

    for heat in gallops.heats.values() {
        for (pace_key, pace) in &heat.paces {
            let bare = pace_key.strip_prefix(crate::jjrf_favor::JJRF_CORONET_PREFIX).unwrap_or(pace_key);
            if bare == coronet {
                return Ok(pace.tacks.first().cloned());
            }
        }
    }
    Ok(None)
}

/// The pace evidence for the confirm display — never a gate, so a read
/// failure becomes `Unavailable` rather than aborting the plan.
fn zjjrdm_pace_evidence<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    coronet: &str,
    guidon: &str,
) -> jjrdm_PaceEvidence {
    match zjjrdm_snapshot_tack(farrier, studbook, coronet, guidon) {
        Ok(Some(tack)) => jjrdm_PaceEvidence::Tack {
            resolved: tack.state.jjrg_is_resolved(),
            state_label: tack.jjrg_state_label(),
            silks: tack.silks.clone(),
            basis: tack.basis.clone(),
        },
        Ok(None) => jjrdm_PaceEvidence::Unknown,
        Err(e) => jjrdm_PaceEvidence::Unavailable(e.to_string()),
    }
}

/// Plan the destroy: resolve the operator's named target to one billet, then —
/// for a pace billet — comb it and read its branch posture and evidence. Pure
/// resolution; nothing here mutates (JJSVD "Muck", Plan-then-confirm).
///
/// A groom ground is git-free (a scratch dir, no repo), so it takes none of the
/// git probes a pace billet does: its `primary_root` is the infield it stands in,
/// it carries no dirty paths and no branch sync, and it has no pace to evidence.
/// A surviving groom ground is a crashed session's residue — muck razes it whole.
pub fn jjrdm_plan<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    infield_root: &Path,
    name: &str,
    guidon: &str,
) -> Result<jjrdm_Plan, jjrdm_Rejection> {
    let (billet_root, billet_dirname) = zjjrdm_resolve_billet(infield_root, name)?;
    let kind = zjjrdm_billet_kind(&billet_dirname)
        .unwrap_or_else(|| panic!("resolved billet dirname '{}' does not type as a billet", billet_dirname));

    if let jjrdm_Kind::Groom(_) = &kind {
        return Ok(jjrdm_Plan {
            billet_root,
            billet_dirname,
            kind,
            primary_root: infield_root.to_path_buf(),
            dirty_paths: Vec::new(),
            sync_state: jjrfr_SyncState::Untracked,
            pace_evidence: None,
        });
    }

    let identity = farrier.jjrfr_identify(&billet_root)?;
    let primary_root = match &identity.seat {
        jjrfr_Seat::Primary => billet_root.clone(),
        jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
    };
    let comb = farrier.jjrfr_comb(&billet_root)?;
    let sync_state = farrier.jjrfr_sync_state(&billet_root)?;

    let pace_evidence = match &kind {
        jjrdm_Kind::Pace(coronet) => Some(zjjrdm_pace_evidence(farrier, studbook, coronet, guidon)),
        jjrdm_Kind::Groom(_) => None,
    };

    Ok(jjrdm_Plan {
        billet_root,
        billet_dirname,
        kind,
        primary_root,
        dirty_paths: comb.dirty_paths,
        sync_state,
        pace_evidence,
    })
}

// ---- Reap phase ----

/// Clear the reaped billet's own scratch sibling, and nothing else — muck is
/// a billet singleton, so its sweep never reaches past the billet it was
/// confirmed against. Best-effort: an unremovable or absent entry is skipped
/// rather than failing the reap that already succeeded.
fn zjjrdm_sweep_scratch(infield_root: &Path, billet_dirname: &str) -> Vec<PathBuf> {
    let scratch = infield_root.join(JJRDS_SCRATCH_DIRNAME).join(billet_dirname);
    if std::fs::remove_dir_all(&scratch).is_ok() {
        vec![scratch]
    } else {
        Vec::new()
    }
}

/// One reap's outcome, for the caller's report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrdm_Outcome {
    pub billet_root: PathBuf,
    pub scratch_swept: Vec<PathBuf>,
}

/// Execute the confirmed destroy, then clear this billet's own scratch sibling.
/// A pace billet is a worktree, taken by the forced destroy
/// (`jjrfr_billet_remove`'s only forced caller — the one deliberate data-loss
/// call in the taxonomy). A groom ground is a git-free scratch dir, razed whole
/// by a plain directory removal — no worktree to unregister.
pub fn jjrdm_reap<F: jjrfr_FarrierBillet>(
    farrier: &F,
    infield_root: &Path,
    plan: &jjrdm_Plan,
) -> Result<jjrdm_Outcome, jjrdm_Rejection> {
    match &plan.kind {
        jjrdm_Kind::Groom(_) => {
            std::fs::remove_dir_all(&plan.billet_root).map_err(|e| jjrdm_Rejection::NotFound {
                name: plan.billet_dirname.clone(),
                detail: format!("could not raze the groom ground at {}: {}", plan.billet_root.display(), e),
            })?;
        }
        jjrdm_Kind::Pace(_) => {
            farrier.jjrfr_billet_remove(&plan.billet_root, true)?;
        }
    }
    let scratch_swept = zjjrdm_sweep_scratch(infield_root, &plan.billet_dirname);
    Ok(jjrdm_Outcome { billet_root: plan.billet_root.clone(), scratch_swept })
}
