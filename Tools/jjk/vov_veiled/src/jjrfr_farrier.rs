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
/// A kind is a shared semantic fact — `DirtyTree` means the same thing whether it
/// surfaces from `jjrfr_billet_detach` or `jjrfr_billet_remove` — so consumers
/// branch on kind, never on message text.
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

/// The atomic-under-lease flavor of `consign`: the holder's own guidon, binding
/// the content push to the holder's own lock ref (JJSVF consign contract, JJSVJ
/// step 5) — if the lock was broken under the holder, the whole push fails.
/// `None` at `jjrfr_consign` selects the plain fast-forward flavor instead.
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
    /// the network. Aspirant source for a future orient-time staleness line
    /// (JJSAF-farrier.adoc, "mount / orient"); open's own staleness warning is
    /// `jjrfr_outstripped`, not this (JJSVF-farrier.adoc, "Toothing: officium
    /// open" — sync_state compares a tree only against its own counterpart and
    /// cannot tell a billet its *trunk* has moved).
    fn jjrfr_sync_state(&self, root: &Path) -> Result<jjrfr_SyncState, jjrfr_Rejection>;

    /// Take the counterfoil at the tree's current position. Ambient on record.
    fn jjrfr_counterfoil(&self, root: &Path) -> Result<jjrfr_Counterfoil, jjrfr_Rejection>;

    /// Formally deposit an explicit file list with a message — no stage-all, no
    /// amend (additive discipline). Sequence-internal.
    fn jjrfr_lodge(&self, root: &Path, files: &[PathBuf], message: &str) -> Result<(), jjrfr_Rejection>;

    /// Gather what the remote holds: update remote-tracking state, mutate nothing
    /// local. Sequence-internal, opportunistic; fetches and never merges.
    fn jjrfr_glean(&self, root: &Path) -> jjrfr_GleanOutcome;

    /// Equalize the tree's line of work with its remote counterpart's tip, as of
    /// the last `jjrfr_glean` — made *equal to* the tip, never merely moved
    /// toward it: a line holding commits the counterpart does not have is
    /// retrenched back to it (`JJr_b52`). The kind resolves the counterpart
    /// itself — callers never speak a kind-native ref dialect. Never merges
    /// toward a remote, never rebases. Composed by the journal.
    ///
    /// A kind may be total here and reject nothing (the fallible signature is the
    /// trait's, not any one kind's); the rejection set each kind speaks is the op
    /// census's to state, farrier sheaf.
    fn jjrfr_advance(&self, root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Hand `branch` into the remote's custody. `lease` absent selects plain
    /// fast-forward (hippodrome branches); present selects atomic-under-lease
    /// (blotter content — the lease binds the push to the holder's own lock
    /// ref, rejecting `LockBroken` when it no longer flies the leased guidon;
    /// a plain content race stays `Diverged`). Never force in either flavor
    /// (`JJr_d81`).
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

/// How a new billet is born (`jjrfr_billet_create`): on a fresh branch, or
/// detached. Both forms anchor at the trunk branch's remote counterpart — its
/// position as of the last `jjrfr_glean` — never at the primary's own checkout.
/// Birthing at the primary's HEAD would carry the operator's unpushed trunk work
/// into the billet branch, publishing it at the first `jjrfr_consign` — the
/// `jjrfr_enfold` no-exfiltration posture, applied at birth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrfr_BilletBirth {
    Branch(String),
    Detached,
}

/// The billet facet (`jjdf_billet`): the partition lifecycle ops, consumed by the
/// dispatch doors.
pub trait jjrfr_FarrierBillet {
    /// Birth a billet: seat an isolated partition on a fresh branch, or detached,
    /// anchored at the trunk branch's remote counterpart (`jjrfr_BilletBirth`).
    /// The caller names the trunk; the kind resolves the counterpart itself —
    /// the `jjrfr_advance`/`jjrfr_enfold` posture. A branch-name collision is a
    /// caller-contract violation: a durable branch re-seats via
    /// `jjrfr_billet_seat`, never re-births.
    fn jjrfr_billet_create(&self, root: &Path, birth: &jjrfr_BilletBirth, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection>;

    /// Seat an existing branch into a fresh partition — the reuse form behind
    /// the spine's billet-ensure: billets are chat-ephemeral, branches durable
    /// (dispatch sheaf), so a reaped billet's branch re-seats here on the next
    /// dispatch rather than re-birthing.
    fn jjrfr_billet_seat(&self, root: &Path, branch: &str, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Re-detach an existing billet at the trunk branch's remote counterpart —
    /// groom-billet reuse (dispatch sheaf entrance spine: "a groom billet in
    /// reuse re-detaches to trunk tip"). Refuses `DirtyTree` on dirt.
    fn jjrfr_billet_detach(&self, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection>;

    /// Whether `branch` exists in the constellation — the observation behind the
    /// spine's create-or-seat choice at billet-ensure. Read-only, network-silent.
    fn jjrfr_line_exists(&self, root: &Path, branch: &str) -> Result<bool, jjrfr_Rejection>;

    /// The staleness probe: is this billet outstripped by trunk — does the trunk
    /// branch's remote counterpart (as of the last `jjrfr_glean`) hold work the
    /// billet's position lacks? A local ancestry check, network-silent, run after
    /// any glean; `jjrfr_sync_state` cannot serve here since it compares the
    /// billet's line against *its own* counterpart, never against trunk's.
    /// `false` when no counterpart is known locally: nothing observed can be
    /// ahead, and the warning this probe feeds must not cry on ignorance.
    fn jjrfr_outstripped(&self, billet_root: &Path, trunk: &str) -> Result<bool, jjrfr_Rejection>;

    /// Reap a billet; refuses `DirtyTree` on dirty.
    fn jjrfr_billet_remove(&self, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Merge the trunk branch's remote counterpart — its position as of the last
    /// `jjrfr_glean` — *into* a billet branch. Never rebase; fail-loud on
    /// conflict, resolution belonging to the attended session. The bare primitive
    /// beneath the dispatch sheaf's refit.
    ///
    /// The caller names the trunk branch: trunk-ness is pedigree-relative and
    /// classified above the trait (the identify contract), never inferred by a
    /// kind from ambient checkout state. The kind resolves the *counterpart* of
    /// that name itself, the `jjrfr_advance` posture — callers never speak a
    /// kind-native ref dialect.
    ///
    /// The operator's local trunk ref is never read. Reading it would make a
    /// consigned billet carry unpushed trunk work into remote custody — the ref
    /// stays unpushed while its content rides out as billet ancestry — and a
    /// later rewrite of those still-mutable commits would strand the billet on
    /// abandoned history.
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
