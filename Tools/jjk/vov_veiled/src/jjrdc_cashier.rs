// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Cashier — the door that dismisses a derelict lock-holder from service
//! (`jjdd_cashier`, JJSVD-dispatch.adoc): sight every JJ blotter lock, report
//! the observed guidons, and — behind the door's confirm gate — lease-guarded
//! pluck per lock.
//!
//! This module adds NOTHING to the break sequence (`jjdb_break`, JJSVJ-journal.adoc):
//! it composes `jjrfr_break` once per lock and contributes only what a door
//! contributes — a roster to sweep, a report the operator can judge from, and
//! the deliberateness the gate imposes. The sequence's safety is the sequence's:
//! sight-arms-pluck is never blind, so the door takes no bracket of its own.
//!
//! Outside the entrance spine. Nothing composes this module; the operator does.

use crate::jjrfg_plaingit::jjrfg_PlainGit;
use crate::jjrfr_farrier::{
    jjrfr_break,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_Rejection,
    jjrfr_Seat,
};
use crate::jjrvb_blotter::{
    jjdb_studbook_config,
    jjdb_BlotterConfig,
};
use crate::jjrvg_guidon::{
    jjdb_guidon_read,
    jjdb_is_probably_live,
    jjdb_render_age,
    jjdb_GuidonRead,
};
use chrono::Utc;
use std::path::{
    Path,
    PathBuf,
};

/// The studbook's roster name — what the report calls the store a lock guards.
/// A roster of one makes the column look redundant; it is not. A wrong-store
/// break is exactly the mistake a storeless one-line report invites, and the
/// roster grows (JJSVD `jjdd_cashier`, The roster).
pub const JJRDC_STORE_STUDBOOK: &str = "studbook";

/// One entry in the door's roster: a blotter to sight, under the name the
/// report gives it.
pub struct jjrdc_Store {
    pub name: &'static str,
    pub config: jjdb_BlotterConfig,
}

/// The roster the door sweeps: every JJ blotter this engine knows, its
/// coordinates engine-known like every blotter's own (never pedigree-resolved —
/// a store cannot look itself up in itself).
///
/// One entry today. The mews fleet store joins when its contract lands, and not
/// before: its coordinates invented now would be a forward reference with
/// nothing behind it. The `vvx` commit lock (`refs/vvg/locks/vvx`) is a
/// different apparatus with its own door — it guards a consumer repo's commits,
/// not a blotter, and speaks no guidon — and is deliberately NOT here.
pub fn jjrdc_roster(infield_root: &Path) -> Vec<jjrdc_Store> {
    vec![jjrdc_Store {
        name: JJRDC_STORE_STUDBOOK,
        config: jjdb_studbook_config(infield_root),
    }]
}

/// What one store's lock ref was found to be flying.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrdc_State {
    /// No clone of this store on this station — nothing to sight, and NOT an
    /// error. A station that never founded its studbook has no lock to break;
    /// the door says so and moves on, rather than failing on the git error a
    /// blind sight would raise.
    Unfounded,
    /// A clone, and no guidon flying.
    Free,
    /// A clone flying a guidon — the mark read for display; `verbatim` inside
    /// it is what a break plucks against.
    Held(jjdb_GuidonRead),
}

/// One store's sighting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrdc_Sighting {
    pub store: String,
    pub root: PathBuf,
    pub state: jjrdc_State,
}

/// Sight one store: probe the root FIRST, then sight. The probe is the whole
/// reason this is not a bare `jjrfr_sight` — an unfounded station's clone is
/// simply absent, and sighting into nothing raises an unclassifiable git failure
/// where the honest answer is "nothing to sight".
pub fn jjrdc_sight_store<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(farrier: &F, store: &jjrdc_Store) -> jjrdc_Sighting {
    let root = store.config.local_root.clone();
    let sighting = |state| jjrdc_Sighting {
        store: store.name.to_string(),
        root: root.clone(),
        state,
    };

    // Unfounded covers both a missing directory and a directory no farrier kind
    // claims: neither is a blotter, and neither holds a lock this door can act on.
    if !root.exists() || farrier.jjrfr_identify(&root).is_err() {
        return sighting(jjrdc_State::Unfounded);
    }

    match farrier.jjrfr_sight(&root) {
        Ok(Some(verbatim)) => sighting(jjrdc_State::Held(jjdb_guidon_read(&verbatim))),
        Ok(None) => sighting(jjrdc_State::Free),
        // A sight that fails against a real clone is a transport fact (an
        // unreachable remote, typically), not a lock state. Report it as the
        // store being unsightable rather than invent a verdict: the operator
        // must not be told a lock is free because the network was down.
        Err(_) => sighting(jjrdc_State::Unfounded),
    }
}

/// Sight the whole roster. Read-only, always safe — the door's first mode, and
/// the one an agent or a script may run freely.
pub fn jjrdc_sight(infield_root: &Path) -> Vec<jjrdc_Sighting> {
    let farrier = jjrfg_PlainGit;
    jjrdc_roster(infield_root)
        .iter()
        .map(|store| jjrdc_sight_store(&farrier, store))
        .collect()
}

/// Break one held lock: the break sequence, once, against this store's clone
/// (`jjdb_break`). Sights afresh and plucks against exactly what it sighted, so
/// a guidon that changed since the report refuses `LockBroken` rather than
/// clearing a lock the operator never saw.
pub fn jjrdc_cashier_store(infield_root: &Path, store_name: &str) -> Result<Option<String>, jjrfr_Rejection> {
    let farrier = jjrfg_PlainGit;
    let roster = jjrdc_roster(infield_root);
    let store = roster
        .iter()
        .find(|store| store.name == store_name)
        .unwrap_or_else(|| panic!("jjrdc_cashier_store: '{}' names no store in the roster", store_name));
    jjrfr_break(&farrier, &store.config.local_root)
}

/// Resolve the station's infield root from the operator's invocation directory:
/// cwd elects the clone, a partition's primary is the hippodrome, and the
/// infield is the hippodrome's parent — the spine's own resolution, run here
/// without the spine (this door needs no pedigree, and must work on a station
/// whose studbook was never founded).
pub fn jjrdc_infield_root(cwd: &Path) -> Result<PathBuf, jjrfr_Rejection> {
    let identity = jjrfg_PlainGit.jjrfr_identify(cwd)?;
    let hippodrome_root = match &identity.seat {
        jjrfr_Seat::Primary => identity.root.clone(),
        jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
    };
    Ok(hippodrome_root
        .parent()
        .unwrap_or_else(|| panic!("hippodrome at {} has no parent to serve as the infield", hippodrome_root.display()))
        .to_path_buf())
}

/// The report the gate shows, per lock (JJSVD `jjdd_cashier`, The confirm gate):
/// the guidon's four fields with acquire time rendered as an AGE, the liveness
/// warning that warns and never blocks, the differentiated consequence, and
/// which store the lock guards.
///
/// This function owns the format. The bash door renders nothing of its own — it
/// prints this and asks.
pub fn jjrdc_report(sightings: &[jjrdc_Sighting]) -> String {
    let now = Utc::now();
    let mut lines = Vec::new();

    for sighting in sightings {
        match &sighting.state {
            jjrdc_State::Unfounded => {
                lines.push(format!("{}: no clone at {} — nothing to sight", sighting.store, sighting.root.display()));
            }
            jjrdc_State::Free => {
                lines.push(format!("{}: no lock held", sighting.store));
            }
            jjrdc_State::Held(read) => {
                lines.push(format!("{}: LOCK HELD — this lock guards the {} at {}", sighting.store, sighting.store, sighting.root.display()));
                lines.push(format!("  officium:  {}", read.officium.as_deref().unwrap_or("(unreadable)")));
                lines.push(format!("  station:   {}", read.station.as_deref().unwrap_or("(unreadable)")));
                lines.push(format!(
                    "  acquired:  {}",
                    match read.acquired {
                        Some(acquired) => jjdb_render_age(acquired, now),
                        None => "(unreadable)".to_string(),
                    }
                ));
                lines.push(format!("  operation: {}", read.operation.as_deref().unwrap_or("(unreadable)")));

                if !read.jjdb_is_well_formed() {
                    lines.push(format!("  mark:      {}", read.verbatim));
                    lines.push("  NOTE: this mark did not read as a composed guidon — it is still a lock,".to_string());
                    lines.push("        and it breaks exactly like any other (the break plucks the mark verbatim).".to_string());
                }

                if jjdb_is_probably_live(read, now) {
                    lines.push("  WARNING: this lock is YOUNG — a ceremony runs in seconds, so a holder this".to_string());
                    lines.push("           fresh is probably a LIVE writer, not a crashed one. Break it only if".to_string());
                    lines.push("           you know something the clock does not.".to_string());
                }

                lines.push("  If the holder is a live WRITER: it loses its ceremony and nothing lands.".to_string());
                lines.push("  If the holder is a live READER: it may finish and ACT on a stale image, with".to_string());
                lines.push("  nothing to catch it. The two are not the same risk.".to_string());
            }
        }
    }

    lines.join("\n")
}

/// Whether anything on the roster is actually held — what the door branches on
/// to decide there is a gate to run at all.
pub fn jjrdc_any_held(sightings: &[jjrdc_Sighting]) -> bool {
    sightings.iter().any(|sighting| matches!(sighting.state, jjrdc_State::Held(_)))
}

/// The stores currently held, by roster name — what the break mode iterates.
pub fn jjrdc_held_stores(sightings: &[jjrdc_Sighting]) -> Vec<String> {
    sightings
        .iter()
        .filter(|sighting| matches!(sighting.state, jjrdc_State::Held(_)))
        .map(|sighting| sighting.store.clone())
        .collect()
}
