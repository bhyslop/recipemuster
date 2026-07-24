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
    SeatVestige,
    LineSeated,
}

impl jjrfr_RejectionKind {
    pub fn jjrfr_as_str(&self) -> &'static str {
        match self {
            jjrfr_RejectionKind::ForeignGround => "foreign-ground",
            jjrfr_RejectionKind::DirtyTree => "dirty-tree",
            jjrfr_RejectionKind::Diverged => "diverged",
            jjrfr_RejectionKind::LockHeld => "lock-held",
            jjrfr_RejectionKind::LockBroken => "lock-broken",
            jjrfr_RejectionKind::SeatVestige => "seat-vestige",
            jjrfr_RejectionKind::LineSeated => "line-seated",
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

/// The atomic-under-lease binding of `proffer`: the holder's own guidon, binding
/// the content push to the holder's own lock ref (JJSVF proffer contract, JJSVJ
/// step 5) — if the lock was broken under the holder, the whole push fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrfr_ConsignLease(pub String);

/// `bequeath`'s total outcome. `Unchanged` is a first-class result, not a
/// rejection: a billet whose tree already stands at trunk's tip has an estate to
/// pass that trunk already holds — the verification-only pace's ordinary shape —
/// and composing a commit for it would put an empty position on the trunk every
/// such wrap. Nothing is composed and nothing is pushed, so there is no position
/// to name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrfr_BequeathOutcome {
    Landed(String),
    Unchanged,
}

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

    /// Advance the tree's line of work to its remote counterpart's tip, as of
    /// the last `jjrfr_glean` — strictly forward, never destructive (`JJr_b52`,
    /// the no-residue construction): a line at or behind the tip moves up to it;
    /// a line holding commits the counterpart does not have rejects `Diverged` —
    /// halt-and-surface for the attended session, since under compose-then-push
    /// (`jjrfr_proffer`) the local branch only ever moves to positions the
    /// remote has accepted, and a diverged blotter clone is an impossible state
    /// nothing here may auto-destroy. The kind resolves the counterpart itself —
    /// callers never speak a kind-native ref dialect. Never merges toward a
    /// remote, never rebases. Composed by the journal.
    fn jjrfr_advance(&self, root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Hand `branch` into the remote's custody: plain fast-forward push
    /// (hippodrome/billet branches; a content race rejects `Diverged`). Never
    /// force (`JJr_d81`). Blotter content never passes here — it goes through
    /// `jjrfr_proffer`, which is what binds a write to the held lock.
    fn jjrfr_consign(&self, root: &Path, branch: &str) -> Result<(), jjrfr_Rejection>;

    /// Proffer a composed write for the remote's acceptance — the blotter write
    /// primitive (`JJr_b52`, compose-then-push). Composes one commit whose sole
    /// parent is `branch`'s remote counterpart's tip (as of the last
    /// `jjrfr_glean`), carrying exactly `files` as they now stand in the working
    /// tree, WITHOUT moving the local branch; pushes it atomic-under-lease (the
    /// lease binds the push to the holder's own lock ref — `LockBroken` when it
    /// no longer flies the leased guidon, `Diverged` on a plain content race);
    /// and only on acceptance advances the local branch to the accepted
    /// position. A refusal leaves the local branch and its record untouched —
    /// no residue exists for any later ceremony to scrub. Never force
    /// (`JJr_d81`). Returns the accepted position's SHA.
    ///
    /// An EMPTY `files` list is admitted and means the record-only commit: the
    /// composed tree is the counterpart tip's own, so the whole content of the
    /// write is its message. The dispatch record is the standing consumer —
    /// a billet's birth is an event, and an event has no file.
    fn jjrfr_proffer(
        &self,
        root: &Path,
        branch: &str,
        files: &[PathBuf],
        message: &str,
        lease: &jjrfr_ConsignLease,
    ) -> Result<String, jjrfr_Rejection>;
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

    /// Seat a branch this station does not yet carry, taking its position from
    /// the remote counterpart — the second station's adopt-never-fork arm behind
    /// the spine's billet-ensure. A fresh local branch is minted AT the
    /// counterpart's tip (as of the last `jjrfr_glean`) and seated in the fresh
    /// partition, so a station meeting work another station pushed rejoins that
    /// line of work instead of forking a rival one from trunk.
    ///
    /// Caller contract, both halves observed first: the branch must NOT exist
    /// locally (`jjrfr_line_exists`) and its counterpart MUST be known
    /// (`jjrfr_line_abroad`). Violating either fails loud.
    fn jjrfr_billet_adopt(&self, root: &Path, branch: &str, billet_root: &Path) -> Result<(), jjrfr_Rejection>;

    /// Re-detach an existing billet at the trunk branch's remote counterpart —
    /// groom-billet reuse (dispatch sheaf entrance spine: "a groom billet in
    /// reuse re-detaches to trunk tip"). Refuses `DirtyTree` on dirt.
    fn jjrfr_billet_detach(&self, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection>;

    /// Whether `branch` exists in the constellation — the observation behind the
    /// spine's create-or-seat choice at billet-ensure. Read-only, network-silent.
    fn jjrfr_line_exists(&self, root: &Path, branch: &str) -> Result<bool, jjrfr_Rejection>;

    /// Whether `branch` stands ABROAD — whether its remote counterpart is known
    /// to this constellation, as of the last `jjrfr_glean`. The observation
    /// behind the spine's adopt-or-fork choice once `jjrfr_line_exists` has
    /// answered no: a line absent at home but standing abroad is another
    /// station's pushed work, and the one that a fork would rival.
    /// Read-only, network-silent — the glean is the caller's beat, so an
    /// unreachable remote leaves this answering from what was last seen rather
    /// than blocking the dispatch.
    fn jjrfr_line_abroad(&self, root: &Path, branch: &str) -> Result<bool, jjrfr_Rejection>;

    /// WHERE `branch` is seated, if the constellation seats it in a partition at
    /// all — the observation behind the spine's rediscovery of a standing billet
    /// (`jjdd_billet` reuse). The kind's own partition registry is the authority,
    /// never a dirname: a billet's dirname is a denormalized label, so a search
    /// by name would answer for the yard while this answers for the seat.
    /// A record whose root no longer stands is NOT a seat and reads `None` — the
    /// caller proceeds to seat, meeting the `SeatVestige` refusal with its own
    /// remedy rather than a silent skip invented here. Read-only, network-silent.
    fn jjrfr_line_seated(&self, root: &Path, branch: &str) -> Result<Option<PathBuf>, jjrfr_Rejection>;

    /// The staleness probe: is this billet outstripped by trunk — does the trunk
    /// branch's remote counterpart (as of the last `jjrfr_glean`) hold work the
    /// billet's position lacks? A local ancestry check, network-silent, run after
    /// any glean; `jjrfr_sync_state` cannot serve here since it compares the
    /// billet's line against *its own* counterpart, never against trunk's.
    /// `false` when no counterpart is known locally: nothing observed can be
    /// ahead, and the warning this probe feeds must not cry on ignorance.
    fn jjrfr_outstripped(&self, billet_root: &Path, trunk: &str) -> Result<bool, jjrfr_Rejection>;

    /// The stranding probe: is the tree's current position an ancestor of — or
    /// equal to — the named branch's remote counterpart, as of the last
    /// `jjrfr_glean`? A local ancestry check, network-silent, the
    /// {jjdd_stile} groom-litmus conjunct (dispatch sheaf). `false` when no
    /// counterpart is known locally: nothing can be proven held on ignorance,
    /// and the exit litmus this probe feeds must never destroy on an unproven
    /// claim — the opposite polarity from `jjrfr_outstripped` above, whose
    /// `false`-on-ignorance suppresses a warning rather than blocking a
    /// destruction; the two neighbors are deliberately never harmonized.
    fn jjrfr_reachable(&self, billet_root: &Path, trunk: &str) -> Result<bool, jjrfr_Rejection>;

    /// Pass the billet's whole estate up to the trunk: compose ONE commit whose
    /// tree is the billet's line-of-work tip tree exactly as it stands, with the
    /// trunk branch's remote counterpart tip — its position as of the last
    /// `jjrfr_glean` — as SOLE parent, and hand that commit into the remote's
    /// custody on trunk. The estate passes whole; the billet's interior history
    /// does not travel with it, which is what makes this a bequest rather than a
    /// merge. The billet branch itself keeps that history and is untouched here.
    ///
    /// Composed, never checked out: no working tree anywhere is written, no local
    /// line of work moves, and the primary's tree is never read (the same
    /// posture `jjrfr_enfold` states — the operator's local trunk ref is not
    /// consulted, only its counterpart). A caller that has just passed the
    /// staleness gate therefore hands this a trivial input by construction: the
    /// counterpart tip is already the billet's ancestor.
    ///
    /// `Unchanged` when the composed tree already equals the counterpart tip's
    /// tree — nothing is composed and nothing is pushed. Never force
    /// (`JJr_d81`); a content race on trunk rejects `Diverged`, and because the
    /// composition happens before any push and moves nothing local, a refusal
    /// leaves no residue on either side.
    fn jjrfr_bequeath(&self, billet_root: &Path, trunk: &str, message: &str) -> Result<jjrfr_BequeathOutcome, jjrfr_Rejection>;

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
