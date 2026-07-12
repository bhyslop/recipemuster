// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Blotter engine — the journal ceremony bracket every blotter write passes
//! through (JJSVJ-journal.adoc, `jjdb_journal`), engine-known bootstrap config
//! for a blotter instance (JJSVB-blotter.adoc, bootstrap config), and the
//! lock-free staleness-tolerant read path (`jjdk_lockless_reads`).
//!
//! Generic over any `jjrfr_FarrierCore + jjrfr_FarrierLock` kind — the same
//! machinery serves the studbook today and the mews fleet store later (blotter
//! sheaf: "two repos, two locks: instances share machinery, never state").
//!
//! Scope note: the journal ceremony's "work half" (JJSVJ step 1 — lodging and
//! consigning issued work on the *work repo*, present only for a notch or a
//! wrap) runs on a different root than the blotter and is therefore a caller
//! concern, not this module's — a caller that has issued work produces its own
//! counterfoil first and folds it into what it writes via the `mutate` closure
//! below. `jjdb_journal` covers the blotter-side bracket: lock, advance, mutate
//! and lodge, consign, release.

use crate::jjrfr_farrier::{
    jjrfr_ConsignLease,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_LockGuard,
    jjrfr_Rejection,
};
use std::path::{
    Path,
    PathBuf,
};

// ---- Bootstrap config ----

/// The studbook's fixed dirname within the infield peer ring (`jjqs_studbook`,
/// cosmology sheaf `jjdw_yard`).
pub const JJDB_STUDBOOK_DIRNAME: &str = "jjqs_studbook";

/// Placeholder remote — the studbook's founding ceremony (JJSVS
/// Founding-and-cutover) has not run on any station yet, so no real hosting
/// exists. Left loud and obviously non-functional rather than a plausible-looking
/// fake, so an accidental live call fails immediately instead of silently
/// addressing nothing.
const ZJJDB_STUDBOOK_REMOTE_UNPROVISIONED: &str = "UNPROVISIONED";

/// Placeholder trunk name for the (not-yet-founded) studbook. A blotter is
/// linear and never branches, so this names its one line of work.
const ZJJDB_STUDBOOK_TRUNK_UNPROVISIONED: &str = "trunk";

/// Engine-known bootstrap coordinates for one blotter instance (`jjdb_blotter`
/// bootstrap config, blotter sheaf): where it lives locally, its remote, and its
/// one line of work. Never pedigree-resolved — a store cannot look itself up in
/// itself; the pedigree resolves consumer repos, the engine carries its own
/// stores' coordinates. Plain data — tests construct their own pointed at a
/// scratch repo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjdb_BlotterConfig {
    pub local_root: PathBuf,
    pub remote_url: String,
    pub trunk: String,
}

/// The studbook's engine-known bootstrap config, given the station's infield
/// root. `infield_root` is captured once by the caller (the no-cwd rule this
/// engine shares with the farrier trait: the door captures cwd, this function
/// never reads the environment or the working directory itself). Coordinates
/// are placeholders pending the founding ceremony — real values land there, not
/// here (JJSVS Founding-and-cutover).
pub fn jjdb_studbook_config(infield_root: &Path) -> jjdb_BlotterConfig {
    jjdb_BlotterConfig {
        local_root: infield_root.join(JJDB_STUDBOOK_DIRNAME),
        remote_url: ZJJDB_STUDBOOK_REMOTE_UNPROVISIONED.to_string(),
        trunk: ZJJDB_STUDBOOK_TRUNK_UNPROVISIONED.to_string(),
    }
}

// ---- Read path ----

/// The lock-free, staleness-tolerant read path (`jjdk_lockless_reads`, blotter
/// sheaf): reads a blotter's local clone directly — no lock, no fetch, no
/// blocking on the network. Consumers reach a blotter's content through this,
/// never raw JSON off an unmanaged path.
pub fn jjdb_read(config: &jjdb_BlotterConfig, rel_path: &Path) -> std::io::Result<Vec<u8>> {
    std::fs::read(config.local_root.join(rel_path))
}

// ---- Journal ceremony ----

/// The journal ceremony (`jjdb_journal`, journal sheaf): the blotter-side
/// bracket every write passes through, ordered durable-first. `guidon` is the
/// caller-composed lock-holder mark (officium, station, acquire time,
/// operation); `mutate` receives the blotter's local root, performs whatever
/// file writes it needs, and returns the explicit file list plus commit message
/// `jjrfr_lodge` will use (additive discipline — no stage-all, no amend).
///
/// Sequence: glean (opportunistic) -> stake (via the RAII guard) -> sight
/// (confirm the held guidon is ours) -> advance (fast-forward to remote tip) ->
/// mutate and lodge -> consign (atomic-under-lease against our own lock ref) ->
/// release (best-effort pluck via the guard's drop).
///
/// Returns the new local HEAD SHA on success — the position now also live on
/// the remote, since consign succeeded. A rejection at any lock-held step
/// leaves `mutate` never called; a rejection at advance, lodge, or consign
/// still releases the lock on the way out (the guard drops on every exit path),
/// since holding it serves no purpose once this ceremony cannot complete.
pub fn jjdb_journal<F, M>(
    farrier: &F,
    config: &jjdb_BlotterConfig,
    guidon: &str,
    mutate: M,
) -> Result<String, jjrfr_Rejection>
where
    F: jjrfr_FarrierCore + jjrfr_FarrierLock,
    M: FnOnce(&Path) -> (Vec<PathBuf>, String),
{
    let root = config.local_root.as_path();

    // Take the lock: glean is opportunistic (never blocks on the network, and
    // its outcome does not gate the ceremony); stake is the guard's own
    // compare-and-swap; sight confirms the held guidon is ours before we trust
    // it enough to advance and write under it.
    let _ = farrier.jjrfr_glean(root);
    let guard = jjrfr_LockGuard::jjrfr_acquire(farrier, root, guidon)?;
    let sighted = farrier.jjrfr_sight(root)?;
    if sighted.as_deref() != Some(guard.jjrfr_guidon()) {
        panic!(
            "jjdb_journal: sight after stake did not confirm our own guidon at {} (expected {:?}, saw {:?})",
            root.display(),
            guard.jjrfr_guidon(),
            sighted
        );
    }

    // Advance: fast-forward the local blotter to remote tip, always clean
    // under the lock.
    farrier.jjrfr_advance(root)?;

    // Mutate and lodge: the caller writes its content; we commit exactly the
    // files it names.
    let (files, message) = mutate(root);
    farrier.jjrfr_lodge(root, &files, &message)?;

    // Consign content: atomic-under-lease against our own lock ref — if the
    // lock was broken under us, the whole push fails (journal sheaf, step 5).
    // No corruption, no service but git: a rejection leaves the local commit
    // stranded but unpushed, never partially applied on the remote.
    farrier.jjrfr_consign(
        root,
        &config.trunk,
        Some(&jjrfr_ConsignLease(guard.jjrfr_guidon().to_string())),
    )?;

    let new_head = zjjrvb_head_sha(farrier, root);

    // Release: best-effort pluck via the guard's drop, right here at the
    // ceremony's natural end.
    drop(guard);
    Ok(new_head)
}

/// The tree's current position, single-repo grain — the counterfoil op's
/// member->SHA manifest has exactly one entry for a non-constellation kind
/// (`jjdf_counterfoil`, farrier sheaf).
fn zjjrvb_head_sha<F: jjrfr_FarrierCore>(farrier: &F, root: &Path) -> String {
    let counterfoil = farrier
        .jjrfr_counterfoil(root)
        .unwrap_or_else(|e| panic!("jjdb_journal: counterfoil failed at {}: {}", root.display(), e));
    counterfoil
        .members
        .get(".")
        .cloned()
        .unwrap_or_else(|| panic!("jjdb_journal: counterfoil carried no single-repo member at {}", root.display()))
}

// ---- Gallops-over-studbook surface (enablement seam) ----

/// Enablement seam: the gallops-over-studbook surface below is complete and
/// tested, but not yet live. Every `jjx_*` command still calls jjri_io's
/// jjdr_load/jjdr_save/jjri_persist/jjri_consign directly and unconditionally —
/// nothing outside this module and its tests reads this constant. Flipping it
/// is the conversion heat's act (JJSVS Founding-and-cutover), not this pace's.
pub const JJDB_GALLOPS_OVER_STUDBOOK_ENABLED: bool = false;

/// The gallops file's fixed relative path within the studbook — its first
/// tenant (`jjdb_studbook` Scope at birth).
const ZJJDB_GALLOPS_REL_PATH: &str = "gallops.json";

/// Persist a Gallops through the studbook's journal ceremony. Reuses
/// `jjdr_save`'s atomic write plus load-back validation unchanged (the old
/// path's own machinery, frozen) for the file itself, then commits it through
/// `jjdb_journal` instead of `vvc::machine_commit` — the studbook is a
/// JJ-owned blotter, not a consumer repo, so the vvc commit-lock apparatus
/// does not apply here.
pub fn jjdb_gallops_journal_save<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    config: &jjdb_BlotterConfig,
    guidon: &str,
    gallops: &crate::jjrt_types::jjrg_Gallops,
    message: String,
) -> Result<String, jjrfr_Rejection> {
    jjdb_journal(farrier, config, guidon, |root| {
        let path = root.join(ZJJDB_GALLOPS_REL_PATH);
        crate::jjri_io::jjdr_save(gallops, &path)
            .unwrap_or_else(|e| panic!("jjdb_gallops_journal_save: jjdr_save failed at {}: {}", path.display(), e));
        (vec![PathBuf::from(ZJJDB_GALLOPS_REL_PATH)], message)
    })
}

/// Load a Gallops from the studbook via the lock-free read path. Reuses
/// `jjdr_load` unchanged (round-trip, reprieve, and semantic validation all
/// carry over) pointed at the studbook's local clone instead of a consumer
/// repo's `.claude/jjm/`.
pub fn jjdb_gallops_journal_load(
    config: &jjdb_BlotterConfig,
) -> Result<crate::jjri_io::jjdr_ValidatedGallops, String> {
    crate::jjri_io::jjdr_load(&config.local_root.join(ZJJDB_GALLOPS_REL_PATH))
}
