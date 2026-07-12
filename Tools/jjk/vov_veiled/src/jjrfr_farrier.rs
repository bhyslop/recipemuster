// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Farrier — JJ's polymorphic revision-control driver contract (JJSVF-farrier.adoc,
//! `jjdf_farrier`).
//!
//! Capability is structural, not declarative: the trait splits into three facets —
//! `jjrfr_FarrierCore` (every kind serves this), `jjrfr_FarrierLock` (the guidon
//! compare-and-swap trio, sequence-internal to the journal and the break), and
//! `jjrfr_FarrierBillet` (partition lifecycle, consumed by the dispatch doors) — so a
//! kind lacking a facet is a compile-time fact, not a runtime capability check.
//!
//! The vocabulary Palisade (`jjdf_palisade`): every name below the trait boundary is
//! git's own language; every name here is git-free by design. A fresh word fails
//! loud — the caller looks it up; a familiar git word fires the wrong reflex (amend,
//! force, CRUD).

use std::collections::HashSet;
use std::path::{
    Path,
    PathBuf,
};
use std::sync::{
    Mutex,
    OnceLock,
};

// ---- Rejection taxonomy ----

/// One farrier-wide rejection-kind taxonomy. Closed by the farrier sheaf: a new
/// kind must be allocated there and named git-free per the vocabulary Palisade —
/// never invented ad hoc by a kind implementation, and never a catch-all bucket.
/// A kind is a shared semantic fact — `Diverged` means the same thing whether it
/// surfaces from `jjrfr_advance` or `jjrfr_consign` — so consumers branch on kind,
/// never on message text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrfr_RejectionKind {
    ForeignGround,
    DirtyTree,
    Diverged,
    LockHeld,
    LockBroken,
}

impl jjrfr_RejectionKind {
    pub fn jjrfr_as_str(&self) -> &'static str {
        match self {
            jjrfr_RejectionKind::ForeignGround => "foreign-ground",
            jjrfr_RejectionKind::DirtyTree => "dirty-tree",
            jjrfr_RejectionKind::Diverged => "diverged",
            jjrfr_RejectionKind::LockHeld => "lock-held",
            jjrfr_RejectionKind::LockBroken => "lock-broken",
        }
    }
}

impl std::fmt::Display for jjrfr_RejectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.jjrfr_as_str())
    }
}

/// A rejection from a core-facet op: the kind plus the op/repo/detail context every
/// consumer needs to act on it — never a bare failure (op census, farrier sheaf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_Rejection {
    pub kind: jjrfr_RejectionKind,
    pub op: &'static str,
    pub repo: PathBuf,
    pub detail: String,
}

impl jjrfr_Rejection {
    pub fn jjrfr_new(
        kind: jjrfr_RejectionKind,
        op: &'static str,
        repo: impl Into<PathBuf>,
        detail: impl Into<String>,
    ) -> Self {
        jjrfr_Rejection { kind, op, repo: repo.into(), detail: detail.into() }
    }
}

impl std::fmt::Display for jjrfr_Rejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rejected ({}) at {}: {}", self.op, self.kind, self.repo.display(), self.detail)
    }
}

impl std::error::Error for jjrfr_Rejection {}

// ---- Core facet value types ----

/// Where a claimed tree sits relative to its constellation (`jjdf_identify` seat
/// resolution). A partition carries its primary's root; the linkage mechanics are
/// the kind's own (worktree gitdir for plain git, a planted station-local marker
/// for full-clone fallbacks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrfr_Seat {
    Primary,
    Partition { primary_root: PathBuf },
}

/// One seat-grain designation of a claimed tree's line of work, even for
/// constellations (`jjdf_identify`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrfr_LineOfWork {
    Branch(String),
    Detached(String),
}

/// The four resolutions `jjdf_identify` returns for a claimed tree. `root` is
/// station-local and transient — never journaled (the no-worktree-paths rivet,
/// studbook sheaf); the identity dirname derives as its basename. `upstream_key` is
/// the kind-canonicalized upstream address — one key per tree, `None` for a tree
/// with no upstream to key on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_Identity {
    pub root: PathBuf,
    pub upstream_key: Option<String>,
    pub seat: jjrfr_Seat,
    pub line_of_work: jjrfr_LineOfWork,
}

/// `comb` result: clean/dirty, per-path. Feeds clean-tree gates and explicit-list
/// deposits above the trait; `comb` itself never rejects on what it finds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_CombReport {
    pub dirty_paths: Vec<PathBuf>,
}

impl jjrfr_CombReport {
    pub fn jjrfr_is_clean(&self) -> bool {
        self.dirty_paths.is_empty()
    }
}

/// `sync_state` result: ahead/behind vs. the remote from the last `glean` — never
/// touches the network itself. `Untracked` is a first-class outcome, not a
/// rejection: a branch with no configured upstream has no ahead/behind to report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrfr_SyncState {
    Tracking { ahead: u32, behind: u32 },
    Untracked,
}

/// `counterfoil` result: the member->SHA manifest — one-element for a single repo,
/// the constellation axis's generalization point — annotated with the line of work
/// and dirty flag. The manifest shape's one home is `jjdb_counterfoil`, studbook
/// sheaf; this is the farrier op that produces it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_Counterfoil {
    pub members: std::collections::BTreeMap<String, String>,
    pub line_of_work: jjrfr_LineOfWork,
    pub dirty: bool,
}

/// `glean`'s total outcome. Glean is opportunistic and never blocks on the
/// network, so its result is not one of the five rejection kinds — a fetch that
/// cannot reach the remote is a normal outcome, not a failure the caller must
/// unwrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrfr_GleanOutcome {
    Updated,
    Unreachable,
}

/// The atomic-under-lease flavor of `consign`: the expected remote SHA the push is
/// conditioned on, binding the push to what the caller last observed. `None` at
/// `jjrfr_consign` selects the plain fast-forward flavor instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_ConsignLease(pub String);

// ---- Facet traits ----

/// The core facet (`jjdf_core`): the ops every farrier kind serves — orientation,
/// inspection, deposit, and remote sync. Every op takes its repo root explicitly
/// and reads no ambient working directory (the no-cwd rule); `self` selects the
/// kind, never the tree. Primitives are capabilities, not commands — the
/// ceremonies that compose them (the journal, the break) are homed above the
/// trait, and every mutating primitive here fails loud and specifically.
pub trait jjrfr_FarrierCore {
    /// Resolve the tree at an explicit probe path: studbook-blind, network-silent,
    /// lock-free. Claim-or-decline — `jjrfr_RejectionKind::ForeignGround` is this
    /// op's sole failure, and doubles as ground detection for a kind-roster probe.
    fn jjrfr_identify(&self, probe_path: &Path) -> Result<jjrfr_Identity, jjrfr_Rejection>;

    /// Comb the tree for dirt: clean/dirty, per-path. Sequence-internal.
    fn jjrfr_comb(&self, root: &Path) -> Result<jjrfr_CombReport, jjrfr_Rejection>;

    /// Ahead/behind vs. the remote, from the last `jjrfr_glean` — never blocks on
    /// the network. Ambient at open and orient: the staleness warning's source.
    fn jjrfr_sync_state(&self, root: &Path) -> Result<jjrfr_SyncState, jjrfr_Rejection>;

    /// Take the counterfoil at the tree's current position. Ambient on record.
    fn jjrfr_counterfoil(&self, root: &Path) -> Result<jjrfr_Counterfoil, jjrfr_Rejection>;

    /// Formally deposit an explicit file list with a message — no stage-all, no
    /// amend (additive discipline). Sequence-internal.
    fn jjrfr_lodge(&self, root: &Path, files: &[PathBuf], message: &str) -> Result<(), jjrfr_Rejection>;

    /// Gather what the remote holds: update remote-tracking state, mutate nothing
    /// local. Sequence-internal, opportunistic; fetches and never merges.
    fn jjrfr_glean(&self, root: &Path) -> jjrfr_GleanOutcome;

    /// Fast-forward-only move of the tree's line of work to its remote
    /// counterpart's tip, as of the last `jjrfr_glean`. The kind resolves the
    /// counterpart itself — callers never speak a kind-native ref dialect.
    /// Rejects `Diverged` when fast-forward is impossible, `DirtyTree` when
    /// uncommitted changes block it. Never merges toward a remote, never
    /// rebases. Composed by the journal.
    fn jjrfr_advance(&self, root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Hand `branch` into the remote's custody. `lease` absent selects plain
    /// fast-forward (hippodrome branches); present selects atomic-under-lease
    /// (blotter content — the lease binds the push to what the caller last
    /// observed). Never force in either flavor (`JJr_d81`).
    fn jjrfr_consign(&self, root: &Path, branch: &str, lease: Option<&jjrfr_ConsignLease>) -> Result<(), jjrfr_Rejection>;
}

/// The lock facet (`jjdf_lock`): the guidon verbs over a blotter's lock ref — the
/// compare-and-swap primitives composed only by the journal and the break.
/// Sequence-internal, never operator-typed.
pub trait jjrfr_FarrierLock {
    /// Stake the guidon in: atomically create the lock ref bearing it, or reject
    /// `LockHeld`.
    fn jjrfr_stake(&self, root: &Path, guidon: &str) -> Result<(), jjrfr_Rejection>;

    /// Pluck the lock out: lease-guarded delete against an observed guidon, never
    /// blind. Rejects `LockBroken` when the observed guidon no longer matches.
    fn jjrfr_pluck(&self, root: &Path, observed_guidon: &str) -> Result<(), jjrfr_Rejection>;

    /// Sight whose guidon flies: read the lock ref, read-only. `None` when no
    /// guidon is staked.
    fn jjrfr_sight(&self, root: &Path) -> Result<Option<String>, jjrfr_Rejection>;
}

/// The billet facet (`jjdf_billet`): the partition lifecycle ops, consumed by the
/// dispatch doors.
pub trait jjrfr_FarrierBillet {
    /// Birth a billet: seat an isolated partition on a named branch, or detached
    /// at a named position.
    fn jjrfr_billet_create(&self, root: &Path, at: &jjrfr_LineOfWork, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Reap a billet; refuses `DirtyTree` on dirty.
    fn jjrfr_billet_remove(&self, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Merge trunk *into* a billet branch — never rebase; fail-loud on conflict,
    /// resolution belonging to the attended session. The bare primitive beneath
    /// the dispatch sheaf's refit. The caller names the trunk branch: trunk-ness
    /// is pedigree-relative and classified above the trait (the identify
    /// contract), never inferred by a kind from ambient checkout state.
    fn jjrfr_enfold(&self, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection>;
}

// ---- Lock guard and break sequence ----

/// Process-local registry of roots with a currently live `jjrfr_LockGuard` — the
/// enforcement mechanism behind nested-acquire panicking. Distributed contention
/// is `jjrfr_stake`'s own `LockHeld` rejection, against another station or
/// officium; a same-process double-acquire over the same root is a distinct,
/// programming-error case a bare compare-and-swap cannot itself distinguish from
/// ordinary contention, since staking again would either legitimately race the
/// first guard's own guidon or wait on a lock this same process already holds.
fn zjjrfr_held_roots() -> &'static Mutex<HashSet<PathBuf>> {
    static HELD: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(HashSet::new()))
}

/// A held guidon-ref lock: object lifetime is lock lifetime (the entity design's
/// RAII posture, blotter sheaf). Release is best-effort on drop — a die mid-hold
/// leaves a stale lock still flying its guidon, recoverable only by the break
/// sequence (`jjrfr_break`), per the journal sheaf's durable-first design; `Drop`
/// cannot itself be fallible.
pub struct jjrfr_LockGuard<'a, F: jjrfr_FarrierLock> {
    farrier: &'a F,
    root: PathBuf,
    guidon: String,
}

impl<'a, F: jjrfr_FarrierLock> jjrfr_LockGuard<'a, F> {
    /// Stake the guidon and hold it for the guard's lifetime. Panics if this
    /// process already holds a guard over `root` — a nested acquire is a
    /// programming error, never legitimate contention (legitimate contention is
    /// `jjrfr_stake`'s own `LockHeld` rejection, against a different holder).
    pub fn jjrfr_acquire(farrier: &'a F, root: &Path, guidon: impl Into<String>) -> Result<Self, jjrfr_Rejection> {
        let root = root.to_path_buf();
        {
            let mut held = zjjrfr_held_roots().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if !held.insert(root.clone()) {
                panic!("nested lock acquire over {}: this process already holds a guard on it", root.display());
            }
        }
        let guidon = guidon.into();
        match farrier.jjrfr_stake(&root, &guidon) {
            Ok(()) => Ok(jjrfr_LockGuard { farrier, root, guidon }),
            Err(rejection) => {
                zjjrfr_held_roots().lock().unwrap_or_else(|poisoned| poisoned.into_inner()).remove(&root);
                Err(rejection)
            }
        }
    }

    /// The guidon this guard staked — what a consumer journals as "ours".
    pub fn jjrfr_guidon(&self) -> &str {
        &self.guidon
    }
}

impl<'a, F: jjrfr_FarrierLock> Drop for jjrfr_LockGuard<'a, F> {
    fn drop(&mut self) {
        // Release is best-effort against the driver's fail-loud posture too: an
        // unclassifiable release failure — an unreachable remote, typically —
        // panics inside the pluck, and a panic escaping a destructor during an
        // unwind aborts the process, turning a recoverable stale lock (the
        // break's whole purpose) into a crash. The default panic hook still
        // reports the caught panic loudly before the guard absorbs it.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = self.farrier.jjrfr_pluck(&self.root, &self.guidon);
        }));
        zjjrfr_held_roots().lock().unwrap_or_else(|poisoned| poisoned.into_inner()).remove(&self.root);
    }
}

/// The break sequence (`jjdb_break`, journal sheaf): clears a stale lock. Never a
/// method on the holder's guard — it acts on someone else's lock. Sights the lock
/// ref and, if a guidon flies, lease-guarded plucks against exactly that observed
/// value — never blind, never forced. `Ok(None)` when there was nothing to clear;
/// `Ok(Some(guidon))` names what was cleared. A `LockBroken` rejection means the
/// guidon changed between the sight and the pluck — someone else's break or a
/// fresh stake raced this one.
pub fn jjrfr_break<F: jjrfr_FarrierLock>(farrier: &F, root: &Path) -> Result<Option<String>, jjrfr_Rejection> {
    let observed = match farrier.jjrfr_sight(root)? {
        Some(guidon) => guidon,
        None => return Ok(None),
    };
    farrier.jjrfr_pluck(root, &observed)?;
    Ok(Some(observed))
}
