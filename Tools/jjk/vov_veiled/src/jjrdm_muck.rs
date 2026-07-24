// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Muck — the billet-clearing sweep (`jjdd_muck`, JJSVD-dispatch.adoc "Muck"),
//! the dispatch spine's leading step. Composed here as a standalone,
//! independently testable unit, generic over any `jjrfr_FarrierCore` +
//! `jjrfr_FarrierLock` (+ `jjrfr_FarrierBillet` for the reap phase) kind —
//! like the rest of this heat's studbook-authoritative surface, it is
//! complete and NOT wired into `jjrds_run`: the studbook's gallops copy this
//! module reads is a parallel path the conversion heat makes authoritative,
//! never the frozen local path `jjrds_plan`/`jjrds_board` still read.
//!
//! Two phases, matching the sheaf's behavior exactly: `jjrdm_plan` snapshots
//! pace states through the journal sheaf's read bracket — the snapshot arms
//! the reap, so it is an acting read (JJSVJ "The read bracket",
//! `jjdk_lockless_reads` scope rule): lock, advance, read, release, deciding
//! from the store's truth at that moment — and classifies every `jjqb_*`
//! billet under the infield into reap / dirty / kept, pure, no filesystem
//! mutation, the lock released before any billet is even looked at;
//! `jjrdm_reap` then executes lock-free.
//! The sweep globs `jjqb_*` (`jjdw_yard`), a positive match that
//! structurally excludes `jjqa_app`, `jjqs_studbook`, and `jjqd_scratch`.
//!
//! The liveness guard — a billet with a live officium standing under it is
//! never a candidate, regardless of pace state or age — is the pure JJ-data
//! join: live officia × billets, read straight off each billet's own
//! `.claude/jjm/officia` via the same heartbeat-freshness rule
//! `zjjrm_exsanguinate` applies. No platform process-probe.
//!
//! **This join's location is scoped to `JJRM_OFFICIUM_STUDBOOK_ENABLED ==
//! false` (the current, live seam state).** A dispatched session's officia
//! live at its own process cwd, which stirrup sets to the billet root — so
//! `billet_root/.claude/jjm/officia` is correct today. Flipping that seam
//! relocates every officium's exchange to the studbook
//! (`jjrm_studbook_exchange_dir`), and nothing then stands under any
//! billet: this guard would find "no live officium" for every billet,
//! always, silently — the exact failure it exists to prevent. Muck staying
//! unwired from `jjrds_run` (see below) means this cannot fire today; it
//! must be re-cut *before* muck is ever wired live, using whatever record
//! the conversion heat gives an officium of which billet it occupies
//! (today's officium-open composition captures only the seat's role, never
//! the billet — JJSVF "Toothing: officium open").

use crate::jjrds_spine::{
    jjrds_type_target,
    jjrds_Target,
    JJRDS_BILLET_DIR_PREFIX,
};
use crate::jjrf_favor::{
    jjrf_livery_parse,
    jjrf_LiveryKind,
};
use crate::jjrfr_farrier::{
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_LineOfWork,
    jjrfr_LockGuard,
    jjrfr_Rejection,
    jjrfr_RejectionKind,
};
use crate::jjrm_mcp::{
    jjrm_officium_dir_is_live,
    JJRM_EXSANGUINATION_THRESHOLD_SECS,
    OFFICIA_DIR,
};
use crate::jjrvb_blotter::{
    jjdb_gallops_journal_load,
    jjdb_BlotterConfig,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// The reap-eligibility retention window: how far past its own directory
/// mtime a billet must age before muck will touch it — the proxy this
/// module uses for "how long past pace-close" (pace billets) or "how long
/// since creation" (groom billets), since a pace's tack timestamps are
/// studbook-internal history, not a billet-floor signal, and a groom billet
/// has no pace to reference at all. One constant, one rhyme with officium
/// exsanguination (JJSVD "Muck").
pub const JJRDM_RETENTION_SECS: u64 = JJRM_EXSANGUINATION_THRESHOLD_SECS;

/// The advice a refused dirty candidate carries — refuse-with-advice is the
/// default outcome; auto-commit-push rides only behind an explicit confirm.
pub const JJRDM_DIRTY_ADVICE: &str =
    "billet carries uncommitted changes; muck refuses by default. Re-run with auto-commit-push confirmed, or clear the billet by hand.";

/// The commit message an auto-commit-push salvage lodges under, ahead of the
/// reap it clears the way for.
const JJRDM_AUTO_COMMIT_MESSAGE: &str = "muck: auto-commit-push before reap";

// ---- Rejections ----

/// Muck's own rejection taxonomy: a composed farrier primitive's own kind, or
/// the studbook's gallops copy being unreadable at snapshot time.
#[derive(Debug)]
pub enum jjrdm_Rejection {
    Farrier(jjrfr_Rejection),
    GallopsUnreadable(String),
}

impl std::fmt::Display for jjrdm_Rejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // A stranded lock meets the operator HERE — at every dispatch, since
            // muck leads the spine — so this is where the cure must be named
            // (JJSVD `jjdd_cashier`, Discoverability: a cure the sufferer cannot
            // find is not a cure).
            jjrdm_Rejection::Farrier(r) if r.kind == jjrfr_RejectionKind::LockHeld => write!(
                f,
                "{}\n  Another station holds the studbook lock. If it crashed, the lock is stranded:\n  \
                 sight it with `tt/jjw-dc.SightLocks.sh`, and cashier it with `tt/jjw-dC.Cashier.sh`.",
                r
            ),
            jjrdm_Rejection::Farrier(r) => write!(f, "{}", r),
            jjrdm_Rejection::GallopsUnreadable(detail) => write!(f, "studbook gallops unreadable: {}", detail),
        }
    }
}

impl From<jjrfr_Rejection> for jjrdm_Rejection {
    fn from(r: jjrfr_Rejection) -> Self {
        jjrdm_Rejection::Farrier(r)
    }
}

// ---- Kinds and candidates ----

/// Which kind of billet a `jjqb_*` dirname resolved to, per the yard
/// signet's typed-by-length convention: a pace billet seats a durable
/// branch (the bare coronet); a groom billet is always detached and carries
/// nothing durable (`jjdd_billet`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrdm_Kind {
    Pace(String),
    Groom(String),
}

/// One billet the plan phase classified as reap-eligible: past retention, no
/// live officium standing under it, and — for a pace billet only — its
/// pace resolved (complete or abandoned).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrdm_Candidate {
    pub billet_root: PathBuf,
    pub kind: jjrdm_Kind,
}

/// The muck plan: what `jjrdm_reap` will act on, resolved entirely ahead of
/// any mutation. Empty on both fields means nothing to muck — the caller
/// stays silent (JJSVD: "the entrance must be pleasant very early", the
/// consequence that forces muck's own silence when it has nothing to reap).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct jjrdm_Plan {
    /// Reap-eligible and clean — nothing stands in the way.
    pub reap: Vec<jjrdm_Candidate>,
    /// Reap-eligible but carrying uncommitted changes — the dirty anomaly
    /// `jjrdm_reap` refuses-with-advice by default.
    pub dirty: Vec<jjrdm_Candidate>,
}

impl jjrdm_Plan {
    pub fn jjrdm_is_empty(&self) -> bool {
        self.reap.is_empty() && self.dirty.is_empty()
    }
}

/// One candidate's post-reap fate, for the caller's report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrdm_Outcome {
    Reaped(PathBuf),
    /// Auto-committed and pushed before reaping (dirty, confirmed).
    Salvaged(PathBuf),
    Refused { billet_root: PathBuf, detail: String },
}

/// `jjrdm_reap`'s full report, in candidate order (reap set first, then
/// dirty set).
#[derive(Debug, Clone, Default)]
pub struct jjrdm_ReapReport {
    pub outcomes: Vec<jjrdm_Outcome>,
}

// ---- Plan phase ----

/// Snapshot every pace's resolved-or-not state from the studbook's gallops
/// copy through the journal sheaf's read bracket (JJSVJ "The read bracket"):
/// glean, stake, sight, advance, read, release — the `jjdb_journal` minus its
/// write steps. The snapshot arms the reap, so it is an acting read, never a
/// reporting one: advance fast-forwards the local clone to the remote tip
/// under the lock, so what this reads is the store's truth at this moment,
/// not a stale local copy. Keyed on the bare coronet: gallops pace keys carry the
/// self-typing `₢` sigil on disk, and billet dirnames are already bare.
fn zjjrdm_snapshot_resolved_paces<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    guidon: &str,
) -> Result<HashMap<String, bool>, jjrdm_Rejection> {
    let root = studbook.local_root.as_path();
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

    let mut resolved = HashMap::new();
    for heat in gallops.heats.values() {
        for (pace_key, pace) in &heat.paces {
            let bare = pace_key.strip_prefix(crate::jjrf_favor::JJRF_CORONET_PREFIX).unwrap_or(pace_key);
            if let Some(tack) = pace.tacks.first() {
                resolved.insert(bare.to_string(), tack.state.jjrg_is_resolved());
            }
        }
    }
    Ok(resolved)
}

/// Whether `billet_root`'s own mtime places it past `JJRDM_RETENTION_SECS`.
/// The filesystem is the age record for both billet kinds — a pace's tack
/// timestamps are studbook-internal history, not a billet-floor proxy, and
/// a groom billet has no pace to reference at all.
fn zjjrdm_past_retention(billet_root: &Path, now: SystemTime) -> bool {
    let mtime = match billet_root.metadata().and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return false,
    };
    match now.duration_since(mtime) {
        Ok(age) => age.as_secs() > JJRDM_RETENTION_SECS,
        Err(_) => false,
    }
}

/// The liveness guard: does a live officium stand under this billet's own
/// `.claude/jjm/officia` — the pure JJ-data join, no platform process-probe
/// (JJSVD "Muck"). Scoped to the current, pre-cutover officium exchange
/// location — see the module doc's `JJRM_OFFICIUM_STUDBOOK_ENABLED` caveat
/// before ever wiring this live.
fn zjjrdm_has_live_officium(billet_root: &Path) -> bool {
    let officia = billet_root.join(OFFICIA_DIR);
    let entries = match std::fs::read_dir(&officia) {
        Ok(e) => e,
        Err(_) => return false,
    };
    entries.flatten().filter(|e| e.path().is_dir()).any(|e| jjrm_officium_dir_is_live(&e.path()))
}

/// Halter-type a billet dirname's suffix (past the `jjqb_` signet) into its
/// kind — a coronet suffix is a pace billet, a firemark suffix a groom
/// billet, per the yard signet's typed-by-length convention. `None` for
/// anything that is not a well-formed `jjqb_*` billet name.
fn zjjrdm_billet_kind(dirname: &str) -> Option<jjrdm_Kind> {
    let suffix = dirname.strip_prefix(JJRDS_BILLET_DIR_PREFIX)?;
    match jjrds_type_target(suffix).ok()? {
        jjrds_Target::Coronet(c) => Some(jjrdm_Kind::Pace(c)),
        jjrds_Target::Firemark(f) => Some(jjrdm_Kind::Groom(f)),
    }
}

/// Classify every `jjqb_*` billet under `infield_root` against the
/// snapshot, lock-free — the studbook lock has already released by the
/// time this runs. A billet with a live officium, one not yet past
/// retention, or a pace billet whose pace is still open (or unknown to the
/// studbook) is simply not a candidate: it never enters the plan, kept
/// silently.
fn zjjrdm_classify<F: jjrfr_FarrierCore>(
    farrier: &F,
    infield_root: &Path,
    resolved: &HashMap<String, bool>,
    now: SystemTime,
) -> Result<jjrdm_Plan, jjrdm_Rejection> {
    let mut plan = jjrdm_Plan::default();
    let entries = match std::fs::read_dir(infield_root) {
        Ok(e) => e,
        Err(_) => return Ok(plan),
    };
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
            None => continue, // not a jjqb_* billet: jjqa_app/jjqs_studbook/jjqd_scratch never match
        };
        if zjjrdm_has_live_officium(&path) {
            continue;
        }
        if !zjjrdm_past_retention(&path, now) {
            continue;
        }
        if let jjrdm_Kind::Pace(coronet) = &kind {
            match resolved.get(coronet) {
                Some(true) => {}
                _ => continue, // still open, or unknown to the studbook — never touched
            }
        }
        let comb = farrier.jjrfr_comb(&path)?;
        let candidate = jjrdm_Candidate { billet_root: path, kind };
        if comb.jjrfr_is_clean() {
            plan.reap.push(candidate);
        } else {
            plan.dirty.push(candidate);
        }
    }
    Ok(plan)
}

/// Plan the sweep: snapshot pace states through the read bracket (glean,
/// stake, sight, advance, load, release), then classify every `jjqb_*`
/// billet under `infield_root` lock-free (JJSVD "Muck": the snapshot is an
/// acting read through the journal sheaf's read bracket; then plan, confirm,
/// and reap lock-free). `guidon` is the caller-composed lock-holder mark,
/// same convention as `jjdb_journal`.
pub fn jjrdm_plan<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    infield_root: &Path,
    guidon: &str,
) -> Result<jjrdm_Plan, jjrdm_Rejection> {
    let resolved = zjjrdm_snapshot_resolved_paces(farrier, studbook, guidon)?;
    zjjrdm_classify(farrier, infield_root, &resolved, SystemTime::now())
}

// ---- Reap phase ----

/// The path prefix muck must never lodge, even when dirty and confirmed:
/// JJ's own officium exchange (gazettes, heartbeats) is a knowledge
/// product, never a work-repo artifact (260709 footprint-posture cinch —
/// "knowledge products ... never land there"). A founded install gitignores
/// this, so `comb` ordinarily never surfaces it as dirty in the first
/// place; this filter is belt-and-braces against an incomplete or
/// hand-edited `.gitignore`.
const JJRDM_JJ_OWNED_PREFIX: &str = ".claude/jjm";

/// Whether `path` — a `comb` dirty-path entry — falls under the JJ-owned
/// tree. Checked in both directions: `git status --porcelain` collapses a
/// wholly-untracked directory to its own top-level entry rather than
/// descending into it (no `-uall`), so an as-yet-empty `.claude/` reports
/// as bare `.claude/` even though everything beneath it, once populated, is
/// JJ-owned — that entry must still be caught (`jj_owned.starts_with(path)`
/// alongside the ordinary `path.starts_with(jj_owned)`).
fn zjjrdm_is_jj_owned(path: &Path) -> bool {
    let jj_owned = Path::new(JJRDM_JJ_OWNED_PREFIX);
    path.starts_with(jj_owned) || jj_owned.starts_with(path)
}

/// Auto-commit-push a dirty pace billet's working changes before reaping —
/// lodges every non-JJ-owned dirty path under a standing message, then
/// consigns to the billet's own branch (plain fast-forward: a billet branch
/// is an ordinary hippodrome branch, never blotter content, so no lease
/// applies). A dirty groom billet has no durable home to consign to —
/// `jjdd_billet`'s "nothing must survive it" — so it refuses regardless of
/// confirm. Salvage requires the billet still seat the pace's own branch: a
/// manually switched or detached checkout would otherwise lodge onto the
/// wrong line while consigning the untouched coronet branch.
fn zjjrdm_auto_commit_push<F: jjrfr_FarrierCore>(farrier: &F, candidate: &jjrdm_Candidate) -> Result<(), jjrfr_Rejection> {
    let coronet = match &candidate.kind {
        jjrdm_Kind::Pace(c) => c,
        jjrdm_Kind::Groom(_) => {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::DirtyTree,
                "jjrdm_reap",
                candidate.billet_root.clone(),
                "a dirty groom billet carries nothing durable — auto-commit-push has no home to consign to",
            ));
        }
    };

    // The seated branch wears the livery badge, so the check reads the badge
    // rather than composing one: muck resolves no pedigree, and a sire with a
    // recorded livery prefix would defeat any composed comparison. Reading is
    // also the stricter form — what gets consigned below is exactly the branch
    // observed seated, never a name assembled beside it.
    let identity = farrier.jjrfr_identify(&candidate.billet_root)?;
    let seated_branch = match &identity.line_of_work {
        jjrfr_LineOfWork::Branch(name) => name.clone(),
        jjrfr_LineOfWork::Detached(_) => String::new(),
    };
    let seats_the_pace = jjrf_livery_parse(&seated_branch)
        .is_some_and(|(kind, body)| kind == jjrf_LiveryKind::Pace && body == coronet);
    if !seats_the_pace {
        return Err(jjrfr_Rejection::jjrfr_new(
            jjrfr_RejectionKind::DirtyTree,
            "jjrdm_reap",
            candidate.billet_root.clone(),
            format!(
                "billet no longer seats pace '{}' livery branch — resolve by hand before salvaging",
                coronet
            ),
        ));
    }

    let comb = farrier.jjrfr_comb(&candidate.billet_root)?;
    let work_paths: Vec<PathBuf> = comb.dirty_paths.into_iter().filter(|p| !zjjrdm_is_jj_owned(p)).collect();
    if work_paths.is_empty() {
        return Err(jjrfr_Rejection::jjrfr_new(
            jjrfr_RejectionKind::DirtyTree,
            "jjrdm_reap",
            candidate.billet_root.clone(),
            "only JJ-owned officium files were dirty — nothing legitimate to salvage",
        ));
    }
    farrier.jjrfr_lodge(&candidate.billet_root, &work_paths, JJRDM_AUTO_COMMIT_MESSAGE)?;
    farrier.jjrfr_consign(&candidate.billet_root, &seated_branch)
}

/// Execute the plan lock-free: clean candidates reap outright; dirty
/// candidates refuse-with-advice by default, or — behind an explicit
/// `confirm_auto_commit_push` — salvage (lodge, consign, then reap) before
/// falling back to refuse-with-advice on any failure. The only silence this
/// module owns is the empty-plan case; once a caller invokes `jjrdm_reap` at
/// all, every candidate lands an explicit outcome.
pub fn jjrdm_reap<F: jjrfr_FarrierBillet + jjrfr_FarrierCore>(
    farrier: &F,
    plan: &jjrdm_Plan,
    confirm_auto_commit_push: bool,
) -> jjrdm_ReapReport {
    let mut report = jjrdm_ReapReport::default();
    for candidate in &plan.reap {
        let outcome = match farrier.jjrfr_billet_remove(&candidate.billet_root) {
            Ok(()) => jjrdm_Outcome::Reaped(candidate.billet_root.clone()),
            Err(e) => jjrdm_Outcome::Refused { billet_root: candidate.billet_root.clone(), detail: e.to_string() },
        };
        report.outcomes.push(outcome);
    }
    for candidate in &plan.dirty {
        let outcome = if confirm_auto_commit_push {
            match zjjrdm_auto_commit_push(farrier, candidate).and_then(|()| farrier.jjrfr_billet_remove(&candidate.billet_root)) {
                Ok(()) => jjrdm_Outcome::Salvaged(candidate.billet_root.clone()),
                Err(e) => jjrdm_Outcome::Refused { billet_root: candidate.billet_root.clone(), detail: e.to_string() },
            }
        } else {
            jjrdm_Outcome::Refused { billet_root: candidate.billet_root.clone(), detail: JJRDM_DIRTY_ADVICE.to_string() }
        };
        report.outcomes.push(outcome);
    }
    report
}
