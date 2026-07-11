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

use std::path::{Path, PathBuf};

// ---- Rejection taxonomy ----

/// One farrier-wide rejection-kind taxonomy. Closed at MVP by the farrier sheaf: a
/// new kind is allocated there, never invented ad hoc by a kind implementation
/// (`jjdk_no_catch_all` — totality by humble first-class rows, never a bucket).
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
/// the kind-canonicalized upstream address, one key per tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_Identity {
    pub root: PathBuf,
    pub upstream_key: String,
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

    /// Fast-forward-only move of `root` to `remote_ref`; rejects `Diverged` when
    /// fast-forward is impossible, `DirtyTree` when uncommitted changes block it.
    /// Never merges toward a remote, never rebases. Composed by the journal.
    fn jjrfr_advance(&self, root: &Path, remote_ref: &str) -> Result<(), jjrfr_Rejection>;

    /// Hand `branch` into the remote's custody. `lease` absent selects plain
    /// fast-forward (hippodrome branches); present selects atomic-under-lease
    /// (blotter content — the lease binds the push to what the caller last
    /// observed). Never force in either flavor.
    fn jjrfr_consign(&self, root: &Path, branch: &str, lease: Option<&jjrfr_ConsignLease>) -> Result<(), jjrfr_Rejection>;
}

/// The lock facet (`jjdf_lock`): the guidon verbs over a blotter's lock ref — the
/// compare-and-swap primitives composed only by the journal and the break.
/// Sequence-internal, never operator-typed. Not implemented this pace.
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
/// dispatch doors. Not implemented this pace.
pub trait jjrfr_FarrierBillet {
    /// Birth a billet: seat an isolated partition on a named branch, or detached
    /// at a named position.
    fn jjrfr_billet_create(&self, root: &Path, at: &jjrfr_LineOfWork, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Reap a billet; refuses `DirtyTree` on dirty.
    fn jjrfr_billet_remove(&self, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Merge trunk *into* a billet branch — never rebase; fail-loud on conflict,
    /// resolution belonging to the attended session. The bare primitive beneath
    /// the dispatch sheaf's refit.
    fn jjrfr_enfold(&self, billet_root: &Path) -> Result<(), jjrfr_Rejection>;
}
