// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! The stile's approach — the one shared sequence behind both doors (`jjdd_stile`,
//! JJSVD-dispatch.adoc "The approach"): saddle and lunge contribute only their
//! target typing and their tier source; everything else runs here. This module
//! composes `jjdf_farrier` primitives and the blotter's engine-known config — it
//! owns no git of its own.
//!
//! Approach order (JJSVD "The approach"): identify at the captured
//! invocation path, pedigree lookup (one indirection: derived key → sire →
//! pedigree), billet ensure, glean, BURV export, provision, launch. The
//! launch primitive is stirrup: pace-blind, parameterized (billet, tier,
//! opening prompt); pace-coupling lives in the callers here. Muck
//! (`jjrdm_muck`) is not a step of this approach — it is the operator-directed
//! destroy door, outside the stile entirely: no dispatch crosses it, and
//! nothing here composes it (JJSVD "Muck").
//!
//! Inertness: nothing on the frozen path reaches this module's doors — they
//! are new opt-in surfaces (a station without a founded studbook meets the
//! fair-faced studbook rejection at pedigree lookup). The staleness surfacing
//! composed here is no longer inert, though: `jjrds_staleness_notice` is
//! wired into the live `jjx_open` path unconditionally
//! (`zjjrm_open_staleness_notice`, jjrm_mcp.rs) — that wiring does not wait
//! on `JJRM_OFFICIUM_STUDBOOK_ENABLED`, which gates only where the officium's
//! own exchange directory lives, not this probe. Notch/wrap wiring remains
//! unwired.

use crate::jjrfg_plaingit::jjrfg_PlainGit;
use crate::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_GleanOutcome,
    jjrfr_LineOfWork,
    jjrfr_Rejection,
    jjrfr_Seat,
    jjrfr_SyncState,
};
use crate::jjrt_types::{
    jjrg_Effort,
    jjrg_PaceState,
    jjrg_Tier,
};
use crate::jjrvb_blotter::{
    jjdb_journal_marks,
    jjdb_pin,
    jjdb_read,
    jjdb_read_pinned,
    jjdb_studbook_config,
    jjdb_BlotterConfig,
    JJDB_GALLOPS_OVER_STUDBOOK_ENABLED,
    JJDB_GALLOPS_REL_PATH,
};
use serde::{Deserialize, Serialize};
use std::path::{
    Path,
    PathBuf,
};

// ---- Kind roster ----

/// The recorded-kind word for the plain-git farrier kind — what a pedigree's
/// kind member says when this driver serves the sire. The MVP kind roster is
/// this one kind; a second kind widens the probe loop in `jjrds_plan`.
pub const JJRDS_KIND_PLAIN_GIT: &str = "plain-git";

// ---- Pedigree read (studbook tenant, read side) ----

/// The pedigrees file's fixed relative path within the studbook. The founding
/// ceremony writes it; this module only reads. Wire keys ride the `jjop_`
/// sprue (the `jjo` JSON-sprue container, JJSVT allocation — `jjop` is the
/// pedigree wire's child).
pub const JJRDS_PEDIGREES_REL_PATH: &str = "pedigrees.json";

/// One pedigree: the per-sire record (`jjdb_pedigree`, JJSVS-studbook.adoc).
/// Keys on the addresses a derived upstream key matches against directly.
///
/// The registered-identity indirection (derived key → minted sire id →
/// pedigree, so an address stays a mutable attribute a repo can change by
/// moving hosts) is deliberately NOT carried here: no consumer resolves it, so
/// the field would seed durable committed records with a value nothing sets
/// meaningfully. It lands as an optional field the day address mobility is
/// real — a non-breaking add, unlike the removal it would otherwise cost
/// (operator ruling 260713, superseding the 260709 cinch's key clause).
///
/// Three record-driven-registry fields ride below the standing three — the
/// launch inversion's record layer, where target election stops climbing from
/// cwd and reads a recorded decision instead. All optional in the wire (default
/// + skip-when-empty), so the pre-inversion single-sire pedigrees.json reads and
/// re-serializes byte-identical and no reprieve is owed; a real sire carries them
/// once the operator elects its values:
///   - `clone_name` — the declared infield clone dirname, the recorded decision
///     `jjrds_elect_clone` resolves against (never a discovery scan). Verified at
///     use: the named dir's derived upstream key must match one of `addresses`.
///   - `handle` — the sire's operator-facing handle (its major project prefix),
///     the legible name a refusal speaks and a future pace→sire affiliation aims.
///   - `kits` — the canonical-kit claims: the kit modules this sire is the
///     canonical home for. `jjrds_validate_claims` enforces one canonical home
///     per kit across the whole set. JJ declares and validates uniqueness; it
///     never release-gates on them (the parcel machinery may someday read them).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct jjrds_Pedigree {
    #[serde(rename = "jjop_kind")]
    pub kind: String,
    #[serde(rename = "jjop_addresses")]
    pub addresses: Vec<String>,
    #[serde(rename = "jjop_trunk")]
    pub trunk: String,
    /// The livery path prefix this sire's owner demands JJ's refs sit under
    /// (`jjdd_livery`). Absent is the ordinary case — the livery sprue is
    /// itself the namespace root, so a prefix is owed only where a house
    /// convention demands one. Optional in the wire so every pedigree written
    /// before the livery mint reads unchanged.
    #[serde(rename = "jjop_livery_prefix", default, skip_serializing_if = "Option::is_none")]
    pub livery_prefix: Option<String>,
    /// The declared infield clone dirname (`jjdw_infield`): the recorded decision
    /// `jjrds_elect_clone` resolves to `infield_root/<clone_name>` — a join, not a
    /// scan for which clone to use. `None` until the operator elects it; the field
    /// becomes load-bearing when the launch inversion switches election onto it.
    #[serde(rename = "jjop_clone", default, skip_serializing_if = "Option::is_none")]
    pub clone_name: Option<String>,
    /// The sire's operator-facing handle — its major project prefix (`rb`, `jj`).
    /// The legible name a refusal speaks; the intended target of a later pace→sire
    /// affiliation. `None` until elected.
    #[serde(rename = "jjop_handle", default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// The canonical-kit claims: kit modules this sire is the canonical home for.
    /// `jjrds_validate_claims` enforces one canonical home per kit across every
    /// sire. Empty until elected; a sire that homes no shared kit stays empty.
    #[serde(rename = "jjop_kits", default, skip_serializing_if = "Vec::is_empty")]
    pub kits: Vec<String>,
}

impl jjrds_Pedigree {
    /// How this sire names itself to the operator: its handle where declared,
    /// else its first address (a pre-handle sire still names something legible in
    /// a refusal). Degrades to a fixed marker for the address-less shape rather
    /// than panicking — that shape is caught elsewhere.
    fn jjrds_moniker(&self) -> String {
        self.handle
            .clone()
            .or_else(|| self.addresses.first().cloned())
            .unwrap_or_else(|| "(unnamed sire)".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct zjjrds_PedigreeFile {
    #[serde(rename = "jjop_sires")]
    sires: Vec<jjrds_Pedigree>,
}

/// Compose the pedigrees-file seed the founding ceremony writes (JJSAS
/// Founding-and-cutover): the write side of the pedigree wire, serializing the
/// SAME structs the read side deserializes — one home for the `jjop_` key
/// names, so a seeded pedigree can never drift from what the lookup expects.
/// Pretty-printed (serde declaration order — `jjop_kind`, `jjop_addresses`,
/// `jjop_trunk`); the reader resolves by key name, so field order is free and
/// this becomes the on-disk form the found writes and every later read
/// round-trips. The founding is the only production writer; everything else in
/// this module reads.
pub fn jjrds_seed_pedigrees_json(sires: Vec<jjrds_Pedigree>) -> Result<String, String> {
    // Unique-claimant gate at the write side: a registry that lets two sires
    // claim one kit never reaches disk. The read side re-runs the same gate so a
    // hand-edited registry is caught too (zjjrds_pedigree_from_bytes).
    jjrds_validate_claims(&sires).map_err(|errs| format!("pedigrees seed: {}", errs.join("; ")))?;
    let file = zjjrds_PedigreeFile { sires };
    serde_json::to_string_pretty(&file).map_err(|e| format!("pedigrees seed: could not serialize: {}", e))
}

// ---- Approach rejections ----

/// The approach's fair-faced refusals (JJSVD "Rejections"): named per the farrier
/// taxonomy where a primitive supplies them, plus the two lookup rejections the
/// sheaf names for the pedigree step. Everything else fails loud through the
/// composed primitive's own rejection or panic.
#[derive(Debug)]
pub enum jjrds_Rejection {
    /// `jjrfr_identify` declined — no kind claims the tree at the invocation path.
    ForeignGround(jjrfr_Rejection),
    /// The studbook clone could not be read at all — most often a station whose
    /// studbook is not yet founded (JJSVS Founding-and-cutover).
    StudbookUnreadable { path: PathBuf, detail: String },
    /// The dispatch door's glean of the studbook could not reach the remote —
    /// currency at the door is strict, so an Unreachable glean refuses the whole
    /// dispatch loud (operator ruling 260719: a failed git operation is for the
    /// attended session, never to silently ride past).
    StudbookUnreachable { path: PathBuf },
    /// A write ceremony is mid-flight on the studbook this second: the courtesy
    /// sight found a guidon still flying after the wait-and-re-glean. Refused so a
    /// read never rides a half-written store; names the holder the guidon carries.
    WriteInFlight { holder: String },
    /// The derived upstream key resolves no sire in the studbook's pedigrees.
    UnrecordedSire { key: String },
    /// The claiming kind contradicts the pedigree's recorded kind.
    RecordGroundDrift { claimed: String, recorded: String },
    /// A billet already stands for this pace — the yard gate's fail-fast refusal
    /// (JJSVD "Yard step"): at most one live billet per coronet, so a saddle
    /// whose pace already has a standing billet refuses before the birth record's
    /// journal write and before any session spawn. Names the standing partition
    /// and the remedies. Keyed on the livery-branch seat and the coronet-labelled
    /// yard entry both, so a partition the seat-read misses (a detached tip, a
    /// lost registration) is still caught; `detail` says which key answered.
    StandingBillet { root: PathBuf, detail: String },
    /// The dispatch target token failed halter typing, or resolution against
    /// the gallops (unknown identity, no actionable pace, terminal pace state).
    BadTarget { detail: String },
    /// An invalid (family, effort) launch pair at stirrup.
    BadLaunchPair { family: String, effort: String },
    /// A farrier primitive rejected mid-approach (e.g. a dirty groom billet at
    /// re-detach).
    Farrier(jjrfr_Rejection),
    /// The pedigree registry lets two sires claim one kit — the unique-claimant
    /// gate's refusal, fired at the dispatch read so a hand-edited registry with a
    /// contested canonical home cannot serve dispatch. `detail` carries one line
    /// per contested kit.
    ClaimConflict { detail: String },
    /// A sire's pedigree records no declared infield clone name, so record-driven
    /// election cannot resolve it — the pre-inversion transitional state, named
    /// rather than papered over.
    CloneUndeclared { sire: String },
    /// Record-driven clone election refused (`jjrds_elect_clone`): the declared
    /// clone is not standing, or rival clones break the one-clone-per-sire invariant.
    CloneRefusal(jjrds_CloneRefusal),
}

impl std::fmt::Display for jjrds_Rejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            jjrds_Rejection::ForeignGround(r) => {
                write!(f, "foreign ground: no revision-control kind claims this directory ({})", r)
            }
            jjrds_Rejection::StudbookUnreadable { path, detail } => {
                write!(
                    f,
                    "studbook unreadable at {}: {} — a station serves dispatch only after its studbook is founded",
                    path.display(),
                    detail
                )
            }
            jjrds_Rejection::StudbookUnreachable { path } => {
                write!(
                    f,
                    "studbook unreachable at {}: the dispatch door's glean could not reach the remote — currency at the door is strict, so the dispatch refuses rather than read a stale snapshot",
                    path.display()
                )
            }
            jjrds_Rejection::WriteInFlight { holder } => {
                write!(
                    f,
                    "a write ceremony is in flight on the studbook (held by '{}') — wait a beat and re-dispatch so its result is read",
                    holder
                )
            }
            jjrds_Rejection::UnrecordedSire { key } => {
                write!(
                    f,
                    "unrecorded sire: no pedigree lists the upstream '{}' — a new sire needs a founding pedigree entry before JJ serves it",
                    key
                )
            }
            jjrds_Rejection::RecordGroundDrift { claimed, recorded } => {
                write!(
                    f,
                    "record/ground drift: the ground claims kind '{}' but the pedigree records '{}'",
                    claimed, recorded
                )
            }
            jjrds_Rejection::StandingBillet { root, detail } => {
                write!(
                    f,
                    "a billet already stands for this pace at {} ({}) — at most one live billet per pace: \
                     work in that session, or `muck` it before saddling again",
                    root.display(),
                    detail
                )
            }
            jjrds_Rejection::BadTarget { detail } => write!(f, "bad dispatch target: {}", detail),
            jjrds_Rejection::BadLaunchPair { family, effort } => {
                write!(f, "invalid launch pair: family '{}' does not admit effort '{}'", family, effort)
            }
            jjrds_Rejection::Farrier(r) => write!(f, "{}", r),
            jjrds_Rejection::ClaimConflict { detail } => {
                write!(f, "canonical-kit claim conflict: {}", detail)
            }
            jjrds_Rejection::CloneUndeclared { sire } => write!(
                f,
                "sire '{}' declares no infield clone name — record one in its pedigree before record-driven election can resolve it",
                sire
            ),
            jjrds_Rejection::CloneRefusal(r) => write!(f, "{}", r),
        }
    }
}

/// Pedigree lookup — the approach's studbook read: lock-free (`jjdk_lockless_reads`),
/// one indirection from the kind-derived upstream key through the sire to its
/// pedigree, then the record/ground cross-check against the claiming kind
/// (`jjdf_identify` contract, farrier sheaf). Reads the studbook's working tree
/// directly — the frozen-path form, used while the gallops-over-studbook seam is
/// closed. The enabled path reads the same file from the pinned snapshot instead
/// (`jjrds_pedigree_lookup_pinned`), so gallops and pedigree share one commit.
pub fn jjrds_pedigree_lookup(
    studbook: &jjdb_BlotterConfig,
    derived_key: &str,
    claiming_kind: &str,
) -> Result<jjrds_Pedigree, jjrds_Rejection> {
    let rel = Path::new(JJRDS_PEDIGREES_REL_PATH);
    let bytes = jjdb_read(studbook, rel).map_err(|e| jjrds_Rejection::StudbookUnreadable {
        path: studbook.local_root.join(rel),
        detail: e.to_string(),
    })?;
    zjjrds_pedigree_from_bytes(&bytes, &studbook.local_root.join(rel), derived_key, claiming_kind)
}

/// Pinned pedigree lookup — the enabled path's studbook read: the same
/// resolution as `jjrds_pedigree_lookup`, but from the pinned snapshot's object
/// database (`git show <pin>:pedigrees.json`) rather than the working tree, so a
/// dispatch reads pedigree and gallops from one coherent commit and touches no
/// studbook working-tree state.
pub fn jjrds_pedigree_lookup_pinned(
    studbook: &jjdb_BlotterConfig,
    pin: &str,
    derived_key: &str,
    claiming_kind: &str,
) -> Result<jjrds_Pedigree, jjrds_Rejection> {
    let bytes = jjdb_read_pinned(studbook, pin, JJRDS_PEDIGREES_REL_PATH).map_err(|detail| {
        jjrds_Rejection::StudbookUnreadable { path: studbook.local_root.join(JJRDS_PEDIGREES_REL_PATH), detail }
    })?;
    zjjrds_pedigree_from_bytes(&bytes, &studbook.local_root.join(JJRDS_PEDIGREES_REL_PATH), derived_key, claiming_kind)
}

/// The pedigree resolution proper, over already-read bytes — shared by the
/// working-tree and pinned readers so parse, indirection, and the record/ground
/// cross-check have one home. `path_for_err` names the source only for the
/// malformed-file rejection.
fn zjjrds_pedigree_from_bytes(
    bytes: &[u8],
    path_for_err: &Path,
    derived_key: &str,
    claiming_kind: &str,
) -> Result<jjrds_Pedigree, jjrds_Rejection> {
    let file: zjjrds_PedigreeFile =
        serde_json::from_slice(bytes).map_err(|e| jjrds_Rejection::StudbookUnreadable {
            path: path_for_err.to_path_buf(),
            detail: format!("malformed pedigrees file: {}", e),
        })?;
    // Unique-claimant gate at the read side: a registry where two sires claim one
    // kit refuses the whole dispatch, not only the seed write — so a hand-edited
    // pedigrees.json cannot serve dispatch with a contested canonical home.
    jjrds_validate_claims(&file.sires)
        .map_err(|errs| jjrds_Rejection::ClaimConflict { detail: errs.join("; ") })?;
    let pedigree = file
        .sires
        .into_iter()
        .find(|p| p.addresses.iter().any(|a| a == derived_key))
        .ok_or_else(|| jjrds_Rejection::UnrecordedSire { key: derived_key.to_string() })?;
    if pedigree.kind != claiming_kind {
        return Err(jjrds_Rejection::RecordGroundDrift {
            claimed: claiming_kind.to_string(),
            recorded: pedigree.kind,
        });
    }
    Ok(pedigree)
}

// ---- The unique-claimant gate over canonical-kit claims ----

/// The unique-claimant law over the pedigree set (`jjop_kits`): a kit names
/// exactly one canonical-home sire, so no two sires may both claim it. Pure over
/// the sire list — no I/O — so it is the one gate both the write side
/// (`jjrds_seed_pedigrees_json`, refusing a violating registry before it reaches
/// disk) and the read side (`zjjrds_pedigree_from_bytes`, refusing a hand-edited
/// registry at every dispatch) call. JJ declares these claims and validates their
/// uniqueness; it never release-gates on them. Returns one message per contested
/// kit, naming the kit and its rival claimants (by handle, or by first address
/// where a sire declares no handle yet).
pub fn jjrds_validate_claims(sires: &[jjrds_Pedigree]) -> Result<(), Vec<String>> {
    use std::collections::BTreeMap;
    // kit -> the claimant monikers that named it, in declaration order.
    let mut claimants: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for sire in sires {
        let who = sire.jjrds_moniker();
        for kit in &sire.kits {
            claimants.entry(kit.as_str()).or_default().push(who.clone());
        }
    }
    let errors: Vec<String> = claimants
        .iter()
        .filter(|(_, who)| who.len() > 1)
        .map(|(kit, who)| {
            format!(
                "kit '{}' is claimed by more than one sire ({}) — a kit has exactly one canonical home",
                kit,
                who.join(", ")
            )
        })
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Appraise a pedigrees.json byte image for the unique-claimant law — the
/// registry-integrity check `jjx_validate` runs over the studbook's pedigree
/// tenant, beside the gallops canonicalization. A parse failure or a contested
/// kit is `Err(message)`; a clean (or empty) registry is `Ok`. Keeps
/// `zjjrds_PedigreeFile` private while giving validate one entry point.
pub fn jjrds_validate_claims_bytes(bytes: &[u8]) -> Result<(), String> {
    let file: zjjrds_PedigreeFile = serde_json::from_slice(bytes)
        .map_err(|e| format!("malformed pedigrees file: {}", e))?;
    jjrds_validate_claims(&file.sires).map_err(|errs| errs.join("; "))
}

// ---- Record-driven clone election ----

/// The two refusals declared-clone election can return (`jjdw_infield`, one clone
/// per sire). Neither is a discovery failure — the declared name is the recorded
/// decision, so `Uncloned` says the recorded clone is not standing and `Rival`
/// says the one-clone invariant is broken.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrds_CloneRefusal {
    /// Zero-match: no clone keyed to this sire stands at its declared name. The
    /// remedy names the clone to make and the address to make it from.
    Uncloned { declared: String, address: String },
    /// Two-match: rival clones of one sire stand in the infield — the transitional
    /// multi-clone hazard the one-time infield sweep drains. Names both dirs.
    Rival { dirs: Vec<String> },
}

impl std::fmt::Display for jjrds_CloneRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            jjrds_CloneRefusal::Uncloned { declared, address } => write!(
                f,
                "no infield clone stands at the declared name '{}' — clone {} there before record-driven election can resolve this sire",
                declared, address
            ),
            jjrds_CloneRefusal::Rival { dirs } => write!(
                f,
                "rival clones of one sire stand in the infield ({}) — one clone per sire: sweep to a single clone before record-driven election can resolve it",
                dirs.join(", ")
            ),
        }
    }
}

/// Elect a sire's infield clone from the recorded declared name — the pure core,
/// over `(dirname, upstream_key)` entries so it is exhaustively testable with no
/// filesystem and no farrier. NOT a discovery scan: the declared name is the
/// recorded decision (`registry over discovery`), and the entries are walked only
/// to enforce one-clone-per-sire and to name the two refusals. An entry counts as
/// this sire's clone iff its upstream key matches one of the sire's addresses — so
/// the elected dir is upstream-verified by construction (the record/ground
/// cross-check re-homed as registry verification). Two-match precedes zero-match:
/// the one-clone invariant is absolute, so rival clones refuse even when one bears
/// the declared name.
pub fn jjrds_elect_clone(
    entries: &[(String, Option<String>)],
    declared: &str,
    addresses: &[String],
) -> Result<String, jjrds_CloneRefusal> {
    let clones: Vec<&String> = entries
        .iter()
        .filter(|(_, key)| key.as_ref().is_some_and(|k| addresses.iter().any(|a| a == k)))
        .map(|(name, _)| name)
        .collect();
    if clones.len() >= 2 {
        return Err(jjrds_CloneRefusal::Rival {
            dirs: clones.into_iter().cloned().collect(),
        });
    }
    if clones.iter().any(|name| name.as_str() == declared) {
        Ok(declared.to_string())
    } else {
        Err(jjrds_CloneRefusal::Uncloned {
            declared: declared.to_string(),
            address: addresses.first().cloned().unwrap_or_default(),
        })
    }
}

/// Resolve a sire's infield clone DIRECTORY from its pedigree — the record-driven
/// election the launch inversion will make dispatch's clone-election path. NOT
/// yet wired live: today's live path still climbs from the captured cwd
/// (`zjjrds_infield`); this is the recorded-decision inverse that supersedes it.
/// Reads the infield's immediate entries, derives each one's upstream key through
/// the farrier's identify (a non-repo entry keys to `None` and simply never
/// matches), and elects via the pure `jjrds_elect_clone`. The declared name
/// resolves to `infield_root/<name>` — a join, never a scan for which clone to use.
pub fn jjrds_resolve_clone<F: jjrfr_FarrierCore>(
    farrier: &F,
    infield_root: &Path,
    pedigree: &jjrds_Pedigree,
) -> Result<PathBuf, jjrds_Rejection> {
    let declared = pedigree
        .clone_name
        .as_deref()
        .ok_or_else(|| jjrds_Rejection::CloneUndeclared { sire: pedigree.jjrds_moniker() })?;
    let read = std::fs::read_dir(infield_root).map_err(|e| jjrds_Rejection::StudbookUnreadable {
        path: infield_root.to_path_buf(),
        detail: format!("cannot read infield: {}", e),
    })?;
    let mut entries: Vec<(String, Option<String>)> = Vec::new();
    for ent in read.flatten() {
        let path = ent.path();
        if !path.is_dir() {
            continue;
        }
        let name = ent.file_name().to_string_lossy().into_owned();
        let key = farrier.jjrfr_identify(&path).ok().and_then(|id| id.upstream_key);
        entries.push((name, key));
    }
    jjrds_elect_clone(&entries, declared, &pedigree.addresses)
        .map(|name| infield_root.join(name))
        .map_err(jjrds_Rejection::CloneRefusal)
}

// ---- Tier roster and the two-source launch choice ----

/// One row of the engine-known tier roster: family name → launch model ID +
/// valid effort set, no default columns (JJSVD "Session launch"). Every kind of
/// vendor drift lands as an edit to this one table; a surprise is a spook.
pub struct jjrds_TierRow {
    pub family: jjrg_Tier,
    pub model_id: &'static str,
    pub efforts: &'static [jjrg_Effort],
}

/// Every effort word the vendor's product surface admits today. Per-family
/// restrictions, when the vendor grows them, land as narrower slices here.
const ZJJRDS_ALL_EFFORTS: &[jjrg_Effort] =
    &[jjrg_Effort::Low, jjrg_Effort::Medium, jjrg_Effort::High, jjrg_Effort::Xhigh, jjrg_Effort::Max];

/// The one roster table, baked into the engine (the blotter's engine-known
/// posture). Stirrup is its one launch consumer — callers speak tier words,
/// never model IDs. The fable row exists but no launch policy names it until
/// its pricing settles (JJSVD): it is reachable only through a pace explicitly
/// bridled at fable.
pub const JJRDS_TIER_ROSTER: &[jjrds_TierRow] = &[
    jjrds_TierRow { family: jjrg_Tier::Haiku, model_id: "claude-haiku-4-5-20251001", efforts: ZJJRDS_ALL_EFFORTS },
    jjrds_TierRow { family: jjrg_Tier::Sonnet, model_id: "claude-sonnet-5", efforts: ZJJRDS_ALL_EFFORTS },
    jjrds_TierRow { family: jjrg_Tier::Opus, model_id: "claude-opus-4-8", efforts: ZJJRDS_ALL_EFFORTS },
    jjrds_TierRow { family: jjrg_Tier::Fable, model_id: "claude-fable-5", efforts: ZJJRDS_ALL_EFFORTS },
];

/// The judgment constant — the one named cell a pace-less or designation-less
/// launch takes (JJSVD: "opus/xhigh"). Undesignated work is judgment work.
pub const JJRDS_JUDGMENT_TIER: jjrg_Tier = jjrg_Tier::Opus;
pub const JJRDS_JUDGMENT_EFFORT: jjrg_Effort = jjrg_Effort::Xhigh;

/// The (tier, effort) two-source choice (JJSVD "Session launch"): a designation
/// launches exactly as recorded — effort absent means the knob is omitted and
/// the vendor default governs, JJ invents nothing; no designation (lunge, or
/// saddle on an unbridled pace) takes the judgment constant.
pub fn jjrds_resolve_launch(
    designation: Option<(jjrg_Tier, Option<jjrg_Effort>)>,
) -> (jjrg_Tier, Option<jjrg_Effort>) {
    match designation {
        Some((tier, effort)) => (tier, effort),
        None => (JJRDS_JUDGMENT_TIER, Some(JJRDS_JUDGMENT_EFFORT)),
    }
}

/// Roster row for a family. The roster is total over `jjrg_Tier` by
/// construction; a family missing from it is an engine defect, not a runtime
/// case, so this panics rather than posing as a classified outcome.
pub fn jjrds_roster_row(family: jjrg_Tier) -> &'static jjrds_TierRow {
    JJRDS_TIER_ROSTER
        .iter()
        .find(|row| row.family == family)
        .unwrap_or_else(|| panic!("tier roster carries no row for family '{}'", family.jjrg_as_str()))
}

/// Whether a roster row admits an effort — stirrup's fair-faced gate, separable
/// so a restricted row stays testable ahead of the vendor ever shipping one.
pub fn jjrds_pair_admitted(row: &jjrds_TierRow, effort: jjrg_Effort) -> bool {
    row.efforts.contains(&effort)
}

// ---- Provisioning: the conduct core and the pull door ----

/// The invariant conduct core, provisioned unhoned with every dispatch (JJSVD
/// "Launch-time provisioning"): the standing repair against silent context
/// starvation, paired with the pull door — a context-lookup verb named here —
/// so a missing piece of context costs one extra round-trip instead of silent
/// ignorance.
pub const JJRDS_CONDUCT_CORE: &str = "\
JJ conduct core (dispatched session):\n\
- Open an officium first: call jjx_open, then pass its ☉-id on every jjx call.\n\
- Never reach past the JJ interface to raw storage: no parsing gallops JSON or officium files directly.\n\
- Additive only: commit through jjx_record with an explicit file list; never git reset/restore/clean/stash, never checkout-to-discard.\n\
- Locks are not yours to break: on any lock-held refusal, stop and surface it verbatim. Cashiering a blotter lock is a human-only ceremony — never sight locks toward recovery, never cashier, never route a confirm gate.\n\
- Pull door — context on demand, before improvising: jjx_get_spec for operation specs, jjx_brief {coronet} for a pace docket, jjx_paddock {firemark} for heat shape. If context seems missing, pull it.\n\
- If the mounted pace is bridled at a sub-frontier tier (haiku, sonnet): designee protocol — orient, work the docket, jjx_record, finish with jjx_landing; never wrap; stop and surface on any hole.\n\
- Otherwise (unbridled, or bridled at your own frontier tier): full ceremony; never auto-wrap — ask the operator.\n";

/// The staleness recommendation body — one text (JJSVD "Refit"). `jjx_open`
/// leads with it today (`zjjrm_open_staleness_notice`, `jjrm_mcp.rs`); notch is
/// to append the same text once its own wiring lands. Wrap deliberately does
/// NOT share this text: there the same probe gates rather than advises, and a
/// refusal speaks the interdictum genre (`jjri_staleness_interdictum`), which
/// this advisory body would dilute. Names refit as the remedy; refit is ashlar,
/// so the words here are operator-facing.
pub const JJRDS_REFIT_RECOMMENDATION: &str =
    "trunk has moved: this billet is behind trunk's remote counterpart. Remedy: refit — merge trunk into the billet and push (never rebase).";

/// The staleness surfacing: the cheap probe the enfold counterpart ruling
/// leaves behind — billet behind trunk's remote counterpart, a local ancestry
/// check after any glean, needing only the trunk name refit already takes.
/// `None` means current (or nothing known to be ahead — the probe never cries
/// on ignorance). Wired into the live jjx_open path unconditionally
/// (`zjjrm_open_staleness_notice`, `jjrm_mcp.rs`) — that wiring does not wait
/// on `JJRM_OFFICIUM_STUDBOOK_ENABLED`, which gates only where the officium's
/// own exchange directory lives, not this probe. Notch/wrap wiring remains
/// unwired.
pub fn jjrds_staleness_notice<F: jjrfr_FarrierBillet>(
    farrier: &F,
    billet_root: &Path,
    trunk: &str,
) -> Result<Option<String>, jjrfr_Rejection> {
    Ok(if farrier.jjrfr_outstripped(billet_root, trunk)? {
        Some(JJRDS_REFIT_RECOMMENDATION.to_string())
    } else {
        None
    })
}

// ---- Doors and targets ----

/// The two doors (`jjdd_saddle`, `jjdd_lunge`). A door contributes its target
/// typing and its tier source; the approach below is shared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrds_Door {
    Saddle,
    Lunge,
}

/// A typed dispatch target, per the halter-typing cinch: glyph stripped if
/// present, then typed by length exactly as today (2 firemark, 5 coronet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrds_Target {
    Coronet(String),
    Firemark(String),
}

/// Halter-type a raw dispatch token. Tolerates the emitted forms: a leading ₢
/// or ₣ sigil strips, and a qualified form's interpunct-separated tail resolves
/// (the heat qualifier is emission-only and ignored on ingest).
pub fn jjrds_type_target(raw: &str) -> Result<jjrds_Target, jjrds_Rejection> {
    // jjrf_bare is the single ingest-normalization home (JJS0 jjdz_encoding): it
    // strips the ₢/₣ glyph and any `·` heat-qualifier down to the bare body.
    let body = crate::jjrf_favor::jjrf_bare(raw.trim());
    match body.chars().count() {
        n if n == crate::jjrf_favor::JJRF_FIREMARK_LEN => Ok(jjrds_Target::Firemark(body.to_string())),
        n if n == crate::jjrf_favor::JJRF_CORONET_LEN => Ok(jjrds_Target::Coronet(body.to_string())),
        n => Err(jjrds_Rejection::BadTarget {
            detail: format!("'{}' types neither firemark (2 chars) nor coronet (5 chars) — {} chars", raw, n),
        }),
    }
}

// ---- Saddle resolution against the gallops ----

/// What saddle resolved for its pace: the coronet (the billet branch, bare) and
/// the designation the launch consumes. Read from the frozen, still-
/// authoritative gallops store at the hippodrome — the operator bridles there
/// until the cutover ceremony.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrds_Saddled {
    pub coronet: String,
    pub designation: Option<(jjrg_Tier, Option<jjrg_Effort>)>,
}

/// Resolve a saddle target against the gallops: a coronet saddles that pace; a
/// firemark saddles the heat's next actionable pace (first Rough or Bridled in
/// heat order — resolution never skips, matching orient's own posture).
///
/// Gallops keys carry their type sigils on disk (₢/₣ — the minted-mark
/// carriage law: self-typing keys in project-authored stores); this function
/// looks up sigiled keys and returns the bare identity body, since the billet
/// branch the coronet becomes is machine context.
pub fn jjrds_resolve_saddle(
    gallops: &crate::jjrt_types::jjrg_Gallops,
    target: &jjrds_Target,
) -> Result<jjrds_Saddled, jjrds_Rejection> {
    match target {
        jjrds_Target::Coronet(coronet) => {
            let pace_key = format!("{}{}", crate::jjrf_favor::JJRF_CORONET_PREFIX, coronet);
            let pace = gallops
                .heats
                .values()
                .find_map(|heat| heat.paces.get(&pace_key))
                .ok_or_else(|| jjrds_Rejection::BadTarget {
                    detail: format!("no pace '{}' in the gallops", pace_key),
                })?;
            let tack = pace.tacks.first().ok_or_else(|| jjrds_Rejection::BadTarget {
                detail: format!("pace '{}' has no tacks", pace_key),
            })?;
            match tack.state {
                jjrg_PaceState::Rough | jjrg_PaceState::Bridled => Ok(jjrds_Saddled {
                    coronet: coronet.clone(),
                    designation: tack.tier.map(|t| (t, tack.effort)),
                }),
                jjrg_PaceState::Complete => Err(jjrds_Rejection::BadTarget {
                    detail: format!("pace '{}' is already complete", pace_key),
                }),
                jjrg_PaceState::Abandoned => Err(jjrds_Rejection::BadTarget {
                    detail: format!("pace '{}' is abandoned", pace_key),
                }),
            }
        }
        jjrds_Target::Firemark(firemark) => {
            let heat_key = format!("{}{}", crate::jjrf_favor::JJRF_FIREMARK_PREFIX, firemark);
            let heat = gallops.heats.get(&heat_key).ok_or_else(|| jjrds_Rejection::BadTarget {
                detail: format!("no heat '{}' in the gallops", heat_key),
            })?;
            for pace_key in &heat.order {
                if let Some(pace) = heat.paces.get(pace_key) {
                    if let Some(tack) = pace.tacks.first() {
                        match tack.state {
                            jjrg_PaceState::Rough | jjrg_PaceState::Bridled => {
                                let body = pace_key
                                    .strip_prefix(crate::jjrf_favor::JJRF_CORONET_PREFIX)
                                    .unwrap_or(pace_key)
                                    .to_string();
                                return Ok(jjrds_Saddled {
                                    coronet: body,
                                    designation: tack.tier.map(|t| (t, tack.effort)),
                                });
                            }
                            _ => continue,
                        }
                    }
                }
            }
            Err(jjrds_Rejection::BadTarget {
                detail: format!("heat '{}' has no actionable pace to saddle", heat_key),
            })
        }
    }
}

/// The heat a pace-aimed lunge labels its billet with: the groomed pace's own
/// heat, bare. The lunge's counterpart to `jjrds_resolve_saddle` — a groom
/// billet wears a firemark whatever the aim was (JJSVD "The billet", the yard's
/// kind channel), so a coronet aim must find its heat before the yard can name
/// the billet at all.
///
/// Pace STATE is deliberately not read, and this is the one place the two
/// resolutions part: saddle refuses a complete or abandoned pace because there
/// is no work to mount, while a groom assesses a docket, and a settled pace's
/// docket is as legible as a live one's. Existence is the whole gate.
pub fn jjrds_groomed_heat(
    gallops: &crate::jjrt_types::jjrg_Gallops,
    coronet: &str,
) -> Result<String, jjrds_Rejection> {
    let pace_key = format!("{}{}", crate::jjrf_favor::JJRF_CORONET_PREFIX, coronet);
    gallops
        .heats
        .iter()
        .find(|(_, heat)| heat.paces.contains_key(&pace_key))
        .map(|(heat_key, _)| {
            heat_key
                .strip_prefix(crate::jjrf_favor::JJRF_FIREMARK_PREFIX)
                .unwrap_or(heat_key)
                .to_string()
        })
        .ok_or_else(|| jjrds_Rejection::BadTarget {
            detail: format!("no pace '{}' in the gallops", pace_key),
        })
}

// ---- The yard: billet and scratch naming ----

/// The billet dirname signet (`jjdw_yard`): `jjqb_{catchword}_{identity}` — the
/// serial the dispatch record minted, then the identity it dispatched to (a
/// coronet for a pace billet, a firemark for a groom billet). The serial sorts
/// the yard by creation and keeps concurrent groom billets of one heat distinct;
/// muck's own billet resolution (`jjrdm_muck`) keys on this prefix too.
pub const JJRDS_BILLET_DIR_PREFIX: &str = "jjqb_";

/// The dispatch-scratch container dirname — the infield-resident home of
/// per-billet BUK state (BURV output/temp/log roots) and the session-scoped
/// MCP config. Deliberately NOT under the `jjqb_` signet: a billet
/// resolution's positive glob must never match it, and it must never shadow
/// a billet.
pub const JJRDS_SCRATCH_DIRNAME: &str = "jjqd_scratch";

/// Mint a billet's dirname: the yard signet, the dispatch record's catchword,
/// and the identity body (bare, no glyph — a dirname is a foreign-traversed
/// surface, and the minted-mark carriage law bars the sigil there).
///
/// The serial is a LABEL, never an identity. It is written here and read
/// nowhere: `jjrds_billet_identity` steps over it without parsing it, so a
/// dirname carries a human-facing sort key that no ingestion path depends on.
pub fn jjrds_billet_dirname(catchword: u64, identity_body: &str) -> String {
    format!("{}{}_{}", JJRDS_BILLET_DIR_PREFIX, catchword, identity_body)
}

/// The identity a billet dirname labels — the yard's one tail-token read, and
/// the single home every consumer of the dirname shape resolves through.
/// `None` for anything that is not a billet dirname at all.
///
/// The read steps over a leading serial rather than parsing it: a run of
/// decimal digits followed by `_` is the catchword when the run is LONGER than
/// any identity body can be, and the tail behind it is the token. That length
/// test is what makes the read unambiguous while the pre-catchword shape
/// (`jjqb_{identity}`, no serial) still stands in the yard — `_` is in the
/// insignia charset, so `jjqb_12_AB` would otherwise read two ways, and the
/// discriminator has to be a fact about identities rather than a fact about the
/// catchword's founding value, which grows.
///
/// Typing the token is the caller's — this answers for the yard's shape alone.
pub fn jjrds_billet_identity(dirname: &str) -> Option<&str> {
    let suffix = dirname.strip_prefix(JJRDS_BILLET_DIR_PREFIX)?;
    match suffix.split_once('_') {
        Some((serial, tail))
            // The coronet is the longer of the two identity bodies, so its
            // length is the ceiling any identity can reach.
            if serial.len() > crate::jjrf_favor::JJRF_CORONET_LEN
                && serial.bytes().all(|b| b.is_ascii_digit()) =>
        {
            Some(tail)
        }
        _ => Some(suffix),
    }
}

// ---- Ground: which tree a caller stands in ----

/// The groom-billet posture, said wherever a groom session meets its own
/// ground: at the door in the opening prompt, and again at every orientation
/// the engine answers. One home, so the door's first impression and the
/// engine's authoritative line cannot drift apart.
///
/// The line is the soft layer under the ground guard: it says what the ground
/// affords BEFORE anything is attempted, so the guard's refusal is met as a
/// reminder rather than a surprise.
pub const JJRDS_GROOM_POSTURE: &str = "Ground: groom billet — detached and ephemeral. \
Work-repo edits have no durable home here and notch refuses them; \
discovery that warrants work becomes a slated pace.";

/// Where a caller stands, read from `jjdf_identify` alone: the seat separates a
/// hippodrome from a partition of one, and the partition's line of work says
/// which billet kind it is (JJSVD "The billet" — a pace billet seats the pace's
/// livery branch, a groom billet a detached tip).
///
/// Deliberately NOT read from the billet dirname: the `jjqb_` signet is a
/// denormalized label whose shape belongs to the yard, while seat and line of
/// work are identify's own answers and stay true however the yard renames.
///
/// One honest imprecision, recorded rather than papered over: a detached
/// partition an operator made by hand is indistinguishable from a groom billet
/// and reads as one. Both afford exactly what the groom posture says, so the
/// conflation costs nothing the ground guard depends on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrds_Ground {
    /// The operator's own clone — the primary seat.
    Hippodrome,
    /// A partition seating a pace's livery branch, carrying that pace's coronet
    /// body (bare, no glyph).
    PaceBillet { coronet: String },
    /// A partition at a detached tip.
    GroomBillet,
    /// A partition JJ never boarded: it seats a branch outside the livery pace
    /// roster. An operator's own worktree lands here, and so does the reserved
    /// `jjls_groom` word — a contract violation nameable exactly because the
    /// badge parses.
    Unboarded { line: String },
}

impl jjrds_Ground {
    /// How a refusal names this ground to the operator.
    pub fn jjrds_as_str(&self) -> String {
        match self {
            jjrds_Ground::Hippodrome => "hippodrome ground".to_string(),
            jjrds_Ground::PaceBillet { coronet } => format!("the pace billet of {}{}", crate::jjrf_favor::JJRF_CORONET_PREFIX, coronet),
            jjrds_Ground::GroomBillet => "a groom billet".to_string(),
            jjrds_Ground::Unboarded { line } => format!("an unboarded partition seating '{}'", line),
        }
    }
}

/// Read the ground from a resolved identity — the pure half, so a caller that
/// already holds an identity never identifies twice.
pub fn jjrds_ground_of(identity: &crate::jjrfr_farrier::jjrfr_Identity) -> jjrds_Ground {
    use crate::jjrf_favor::{jjrf_livery_parse, jjrf_LiveryKind};

    match (&identity.seat, &identity.line_of_work) {
        (jjrfr_Seat::Primary, _) => jjrds_Ground::Hippodrome,
        (jjrfr_Seat::Partition { .. }, jjrfr_LineOfWork::Detached(_)) => jjrds_Ground::GroomBillet,
        (jjrfr_Seat::Partition { .. }, jjrfr_LineOfWork::Branch(name)) => {
            match jjrf_livery_parse(name) {
                Some((jjrf_LiveryKind::Pace, body)) => jjrds_Ground::PaceBillet { coronet: body },
                _ => jjrds_Ground::Unboarded { line: name.clone() },
            }
        }
    }
}

/// Read the ground at an explicit probe path. `None` when no kind claims the
/// tree: ground is a fact to observe, and a caller that could not observe one
/// judges nothing — the verb it guards fails on its own terms instead.
pub fn jjrds_ground<F: jjrfr_FarrierCore>(farrier: &F, cwd: &Path) -> Option<jjrds_Ground> {
    farrier.jjrfr_identify(cwd).ok().map(|id| jjrds_ground_of(&id))
}

// ---- Infield resolution and the door's currency step ----

/// Resolve the infield coordinates from the captured invocation path: identify
/// (a decline is the fair-faced foreign-ground rejection), climb from the
/// claimed tree to its hippodrome (a billet's primary), then to the infield that
/// holds the studbook. Shared by the door's currency step and by `jjrds_plan`,
/// so both name the same clone. Never reads the environment — `cwd` is the one
/// captured path (the no-cwd rule `jjrfr_identify` honors).
fn zjjrds_infield(cwd: &Path) -> Result<(crate::jjrfr_farrier::jjrfr_Identity, PathBuf, PathBuf), jjrds_Rejection> {
    let farrier = jjrfg_PlainGit;
    let identity = farrier.jjrfr_identify(cwd).map_err(jjrds_Rejection::ForeignGround)?;
    let hippodrome_root = match &identity.seat {
        jjrfr_Seat::Primary => identity.root.clone(),
        jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
    };
    let infield_root = hippodrome_root
        .parent()
        .unwrap_or_else(|| panic!("hippodrome at {} has no parent to serve as the infield", hippodrome_root.display()))
        .to_path_buf();
    Ok((identity, hippodrome_root, infield_root))
}

/// The beat the courtesy sight waits before re-gleaning, giving a genuinely
/// in-flight write ceremony a moment to complete so its result is the one read.
/// `jjrds_currency` takes the pause as a parameter so a test drives it to zero;
/// the live door passes this.
pub const JJRDS_CURRENCY_BEAT: std::time::Duration = std::time::Duration::from_millis(750);

/// The dispatch door's currency step (operator ruling 260719, JJSVD): glean the
/// studbook clone so the pinned snapshot every read takes is current, then a
/// courtesy sight for an in-flight write.
///
/// Strict currency: an Unreachable glean REFUSES the whole dispatch — a failed
/// git operation is for the attended session, never to silently ride a stale
/// store. Courtesy sight: a flying guidon means a write ceremony is mid-flight
/// this second, so wait a beat, re-glean to pick up its result, and sight again;
/// a guidon still flying refuses, naming the holder its mark carries. This is a
/// freshness courtesy, not a lock — the read takes no lock, ever. Meaningful
/// only over the studbook; the door skips it when the seam is closed.
pub fn jjrds_currency<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    beat: std::time::Duration,
) -> Result<(), jjrds_Rejection> {
    if farrier.jjrfr_glean(&studbook.local_root) == jjrfr_GleanOutcome::Unreachable {
        return Err(jjrds_Rejection::StudbookUnreachable { path: studbook.local_root.clone() });
    }
    if farrier.jjrfr_sight(&studbook.local_root).map_err(jjrds_Rejection::Farrier)?.is_some() {
        std::thread::sleep(beat);
        let _ = farrier.jjrfr_glean(&studbook.local_root);
        if let Some(holder) = farrier.jjrfr_sight(&studbook.local_root).map_err(jjrds_Rejection::Farrier)? {
            return Err(jjrds_Rejection::WriteInFlight { holder });
        }
    }
    Ok(())
}

// ---- The launch plan ----

/// Everything the approach resolved ahead of boarding: what the billet seats, who
/// it dispatches to, and how the session launches. Planning is pure resolution;
/// where the billet stands is NOT resolved here — that waits on the yard step
/// (`jjrds_yard_gate`, then a dirname minted from the dispatch record's
/// catchword), because a mint costs a journal write and planning takes no lock
/// and touches no remote.
#[derive(Debug)]
pub struct jjrds_LaunchPlan {
    pub door: jjrds_Door,
    /// The pedigree's recorded livery path prefix (`None` by default,
    /// org-demand-only), carried from planning to the mint so the branch name can
    /// be dressed there — beside the dirname, from the one catchword — rather than
    /// here, where the serial is not yet known. Meaningful for a saddle only; a
    /// lunge births detached and carries no branch. Which of the two a dispatch is
    /// (pace branch vs groom detached) reads from `door`, never from this field's
    /// presence — a `None` prefix is an unprefixed pace, not a groom.
    pub livery_prefix: Option<String>,
    /// The YARD LABEL, bare: the pace's coronet for a saddle, the heat's
    /// firemark for a lunge — including a lunge aimed at a pace, which wears
    /// that pace's heat here. This is what the dirname carries, and it is
    /// deliberately NOT always the dispatch's target: the dirname's identity is
    /// the yard's kind channel (JJSVD "The billet"), read by length alone by both
    /// the yard gate's yard key and muck's kind resolution, so a groom must never
    /// wear a coronet whatever it was aimed at. What the dispatch is FOR is
    /// `aim`.
    pub identity_body: String,
    /// What the dispatch is FOR, typed: the pace for a saddle or a pace-aimed
    /// lunge, the heat for a heat-aimed lunge. Equals `identity_body` for every
    /// aim but the pace-aimed lunge, where the label drops to the heat and only
    /// this field still names the pace. The meaning-bearing surfaces read it —
    /// the dispatch record's sigil and the opening prompt — never the dirname.
    pub aim: jjrds_Target,
    pub hippodrome_root: PathBuf,
    pub infield_root: PathBuf,
    pub trunk: String,
    pub tier: jjrg_Tier,
    pub effort: Option<jjrg_Effort>,
    pub opening_prompt: String,
}

/// Where a dispatch's billet stands, once the yard step has answered: the
/// freshly minted dirname, the billet root under the infield, and the per-billet
/// scratch keyed by that same dirname — so two concurrent groom billets of one
/// heat, distinguished by their serials, carry distinct BUK state rather than
/// sharing one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrds_Yard {
    pub billet_dirname: String,
    pub billet_root: PathBuf,
    pub scratch_root: PathBuf,
}

/// Compose the yard coordinates from a billet root — the one place the dirname
/// and the scratch root are derived, so every minted billet is keyed
/// identically. The scratch always sits under THIS station's infield, keyed by
/// the billet's own dirname.
pub fn jjrds_yard(infield_root: &Path, billet_root: PathBuf) -> jjrds_Yard {
    let billet_dirname = billet_root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| panic!("billet root {} names no directory", billet_root.display()));
    jjrds_Yard {
        scratch_root: infield_root.join(JJRDS_SCRATCH_DIRNAME).join(&billet_dirname),
        billet_dirname,
        billet_root,
    }
}

/// The yard gate's verdict for a saddle. `Clear` mints a fresh billet; `Resume`
/// re-enters a cleanly-standing one in place under an attended confirm; the
/// anomalous seat-evading partition is neither — it is the `StandingBillet`
/// refusal (the fork's reasoning lives on `jjrds_yard_gate`).
#[derive(Debug)]
pub enum jjrds_YardVerdict {
    /// Nothing stands: mint a fresh billet along the ordinary dispatch path.
    Clear,
    /// A billet stands cleanly for this pace — its livery branch seated in a
    /// registered partition (K1). Carries the partition root and the seated
    /// livery branch for the resume report and launch.
    Resume { root: PathBuf, branch: String },
}

/// The yard gate — the dispatch spine's fork between a fresh birth, an attended
/// resume, and a refusal (`jjdd_billet`, at most one live billet per coronet):
/// a saddle whose pace already has a billet does not silently rejoin a live
/// worktree or mint a rival past a missed seat — it resumes a cleanly-standing
/// one under confirm, and refuses an anomalous one before the birth record's
/// journal write and any session spawn.
///
/// Two keys, because a standing billet can evade either read alone, and the two
/// answers part here:
/// - K1, the livery-branch SEAT the constellation's partition registry records
///   (`jjrfr_seated_lines`), the authority for a billet on its own branch — a
///   CLEAN standing billet, so its answer is `Resume`, not a refusal;
/// - K2, the coronet-labelled YARD entry (`zjjrds_yard_label`), which catches a
///   partition the seat-read misses — a detached tip, a lost or unregistered
///   worktree, a pre-livery bare-branch billet — its livery branch seated
///   nowhere, so re-entering it would need branch-resolution this arm does not
///   own: its answer is the `StandingBillet` refusal, muck the remedy.
/// Only when BOTH are silent is the pace `Clear`. The registry alone is not
/// enough: the constellation's checkout exclusivity guards only the narrow
/// worktree-add seat arm, never the dispatch, so without the gate a re-saddle
/// rejoins a live worktree or, where the seat-read misses, mints a rival past it
/// — the hole this gate closes.
///
/// A groom billet seats no branch and grooms of one heat are deliberately
/// concurrent, so a detached (groom) birth is always `Clear` — the guard is the
/// pace half of the at-most-one ruling alone.
pub fn jjrds_yard_gate<F: jjrfr_FarrierBillet>(
    farrier: &F,
    plan: &jjrds_LaunchPlan,
) -> Result<jjrds_YardVerdict, jjrds_Rejection> {
    // Only a saddle seats a livery branch; a lunge births detached and has no
    // pace billet to collide with.
    if !matches!(plan.door, jjrds_Door::Saddle) {
        return Ok(jjrds_YardVerdict::Clear);
    }
    let coronet = &plan.identity_body;
    // K1 — the livery-branch seat: the registry is the authority for a billet
    // standing on its own branch. A per-birth serial makes the branch name
    // unguessable ahead of the mint (`jjrf_livery_compose`), so the gate can no
    // longer ask "is THIS name seated" — it enumerates every seat and matches by
    // the coronet behind the badge, never by a composed name. A hit is the clean
    // standing billet — `Resume`.
    for (branch, root) in farrier.jjrfr_seated_lines(&plan.hippodrome_root).map_err(jjrds_Rejection::Farrier)? {
        if crate::jjrf_favor::jjrf_livery_parse(&branch)
            .is_some_and(|(kind, body)| kind == crate::jjrf_favor::jjrf_LiveryKind::Pace && &body == coronet)
        {
            return Ok(jjrds_YardVerdict::Resume { root, branch });
        }
    }
    // K2 — the coronet-labelled yard entry: catches a standing partition the
    // seat-read misses (detached tip, lost registration, pre-livery bare branch).
    // Its livery branch is seated nowhere, so this is the anomalous case the
    // resume arm does not own — refuse, muck the remedy.
    if let Some(root) = zjjrds_yard_label(&plan.infield_root, coronet) {
        return Err(jjrds_Rejection::StandingBillet {
            root,
            detail: format!(
                "a partition labelled for {}{} stands there, its livery branch seated nowhere — a detached or unregistered partition",
                crate::jjrf_favor::JJRF_CORONET_PREFIX, coronet
            ),
        });
    }
    Ok(jjrds_YardVerdict::Clear)
}

/// The resume confirm's report (JJSVD "Yard step", resume arm): the standing
/// billet named by its partition root, its seated livery branch, and the dirty
/// paths BY NAME — never a count, muck's own report discipline, since a count
/// hides which work is aboard. The live-session possibility is spoken last so the
/// operator answers occupied-vs-vacated.
fn zjjrds_resume_report<F: jjrfr_FarrierCore>(farrier: &F, root: &Path, branch: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!("a billet already stands for this pace at {}\n", root.display()));
    s.push_str(&format!("  branch:  {}\n", branch));
    match farrier.jjrfr_comb(root) {
        Ok(comb) if comb.dirty_paths.is_empty() => s.push_str("  tree:    clean\n"),
        Ok(comb) => {
            s.push_str("  tree:    DIRTY — uncommitted paths aboard:\n");
            for path in &comb.dirty_paths {
                s.push_str(&format!("             {}\n", path.display()));
            }
        }
        Err(e) => s.push_str(&format!("  tree:    (could not comb the billet: {})\n", e)),
    }
    s.push_str(
        "This billet may be occupied by a live session on another terminal — \
         resume only if you know that session has ended.\n",
    );
    s
}

/// The yard's own answer to "is a billet here labelled for this identity" — the
/// glob half, read through the one tail-token home so a serialed label and a
/// pre-catchword one both resolve. The yard gate's second key (K2): a filesystem
/// read independent of the registry, so a standing partition the seat-read misses
/// (a detached tip, a lost registration) is still caught by its coronet-labelled
/// dirname.
fn zjjrds_yard_label(infield_root: &Path, identity_body: &str) -> Option<PathBuf> {
    let entries = std::fs::read_dir(infield_root).ok()?;
    entries.flatten().map(|e| e.path()).find(|path| {
        path.is_dir()
            && path
                .file_name()
                .map(|n| jjrds_billet_identity(&n.to_string_lossy()) == Some(identity_body))
                .unwrap_or(false)
    })
}

// ---- The dispatch record ----

/// The word for a billet's kind, as the dispatch record names it. Keyed on the
/// door, not on a birth: the record is composed inside the journal mutate, before
/// the catchword that dresses the branch even exists.
fn zjjrds_billet_kind_word(door: jjrds_Door) -> &'static str {
    match door {
        jjrds_Door::Saddle => "pace billet",
        jjrds_Door::Lunge => "groom billet",
    }
}

/// Compose the dispatch record's subject — the event a billet's birth is: which
/// door, which billet kind, which target, which station. The worktree path is
/// deliberately absent: a path is station-local and volatile, while the event is
/// the durable fact, and JJ records no worktree paths (`JJr_f30`'s neighborhood
/// — the derivability posture the dispatch sheaf keeps).
///
/// The target carries its sigil: a commit message is operator-facing output, so
/// the minted-mark carriage law makes the glyph mandatory here exactly as the
/// dirname's foreign-traversed surface bars it.
///
/// The record names the AIM, not the yard label, and its sigil follows the aim's
/// own type rather than the door — a lunge aimed at a pace records that pace,
/// though its billet is labelled by the heat. The record is the durable event
/// and the dirname is a label, so this is the surface that must stay true to
/// what the dispatch was for.
pub fn jjrds_dispatch_record(door: jjrds_Door, aim: &jjrds_Target, station: &str) -> String {
    let (sigil, body) = match aim {
        jjrds_Target::Coronet(c) => (crate::jjrf_favor::JJRF_CORONET_PREFIX, c),
        jjrds_Target::Firemark(f) => (crate::jjrf_favor::JJRF_FIREMARK_PREFIX, f),
    };
    format!(
        "dispatch {} — {} for {}{} at station {}",
        match door {
            jjrds_Door::Saddle => "saddle",
            jjrds_Door::Lunge => "lunge",
        },
        zjjrds_billet_kind_word(door),
        sigil,
        body,
        station,
    )
}

/// Record a billet's birth in the studbook journal, and return the catchword the
/// ceremony allocated — the serial the new billet's dirname wears.
///
/// This is the one place dispatch WRITES. It runs only when a billet is about to
/// be minted: the yard gate has already cleared the way (no standing billet for
/// this pace), so every dispatch that reaches here is a genuine birth. The
/// accepted cost is named in the ruling this builds: a mint is a locked journal
/// write, online and `LockHeld`-refusable — the same bracket muck already rides
/// at every dispatch.
///
/// The record is content-less by construction: an event has no file, so the
/// commit's whole content is its message and its tree is the tip's own.
///
/// The guidon carries no officium: a dispatch precedes the session it launches,
/// so there is no officium to name yet, and the field says so rather than
/// inventing one. The deferred officium/dispatch-record convergence is what
/// would fill it.
pub fn jjrds_record_dispatch<F: jjrfr_FarrierCore + jjrfr_FarrierLock>(
    farrier: &F,
    studbook: &jjdb_BlotterConfig,
    plan: &jjrds_LaunchPlan,
    station: &str,
) -> Result<u64, jjrds_Rejection> {
    let guidon = crate::jjrvg_guidon::jjdb_guidon_compose(
        "",
        station,
        chrono::Utc::now(),
        match plan.door {
            jjrds_Door::Saddle => "saddle",
            jjrds_Door::Lunge => "lunge",
        },
    );
    let subject = jjrds_dispatch_record(plan.door, &plan.aim, station);
    crate::jjrvb_blotter::jjdb_journal_mark(farrier, studbook, &guidon, |_root| (Vec::new(), subject))
        .map(|landing| landing.catchword)
        .map_err(jjrds_Rejection::Farrier)
}

// ---- The live-line resolution: a coronet's current branch, from the journal ----

/// The lede a SADDLE's dispatch record opens with, through the coronet sigil —
/// the reader's needle for a pace birth, kept beside `jjrds_dispatch_record`
/// (which composes it) so a drift test catches any skew. Only a saddle seats a
/// pace billet's livery branch: a groom lunge records "groom billet" and opens
/// no line, so it is deliberately not matched here.
pub const JJRDS_SADDLE_BIRTH_LEDE: &str = "dispatch saddle — pace billet for ₢";

/// A journal mark's meaning to the live-line resolver. Births OPEN a coronet's
/// branch-bearing line (a saddle's dispatch record, carrying the catchword the
/// branch was dressed with); wraps CLOSE it (the W chalk the wrap ceremony
/// journals). Every other journal subject — a groom lunge, a slate, a curry,
/// each non-saddle write — is neither, and classifies to `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrds_LineEvent {
    Birth { coronet: String, catchword: u64 },
    Wrap { coronet: String },
}

/// Classify one journal mark — its catchword and unbaked subject — as a birth,
/// a wrap, or neither. The reader half of `jjrds_dispatch_record` (births) and
/// `jjrn_format_chalk_message` (the wrap chalk), co-located here so the writers
/// and this reader stay honest — the studbook journal is composed by known JJ
/// code, and this is where its subjects are read back.
pub fn jjrds_classify_line_event(catchword: u64, subject: &str) -> Option<jjrds_LineEvent> {
    // Birth: a saddle's dispatch record. The coronet is the whitespace-free
    // token right after the sigil, before " at station".
    if let Some(rest) = subject.strip_prefix(JJRDS_SADDLE_BIRTH_LEDE) {
        let coronet = rest.split_whitespace().next()?.to_string();
        return Some(jjrds_LineEvent::Birth { coronet, catchword });
    }
    // Wrap: the W chalk header `jjb:BRAND:₢CORONET:W: subject`. splitn(5, ':')
    // keeps the subject (which may itself carry colons) whole in the final field;
    // BRAND holds no colon, so the four leading fields are stable.
    let fields: Vec<&str> = subject.splitn(5, ':').collect();
    if let [prefix, _brand, identity, action, ..] = fields.as_slice() {
        let is_wrap = action.parse::<char>().ok() == Some(crate::jjrn_notch::jjrn_ChalkMarker::Wrap.jjrn_code());
        if *prefix == crate::jjrn_notch::JJRN_COMMIT_PREFIX && is_wrap {
            if let Some(coronet) = identity.strip_prefix(crate::jjrf_favor::JJRF_CORONET_PREFIX) {
                return Some(jjrds_LineEvent::Wrap { coronet: coronet.to_string() });
            }
        }
    }
    None
}

/// Resolve a coronet's CURRENT live branch from the studbook journal marks
/// (newest-first) — the one line every re-saddle of this coronet re-seats, and
/// this pace's whole reason (`jjdd_billet`, the studbook-routed resumption): the
/// selection routes through the journal, the lock-coherent record of births and
/// wraps, NEVER git-ref enumeration, which is race-partial (an unpushed or
/// offline rival is invisible) and would lull a caller into "I am the only line."
///
/// The rule reads both sources. Walking newest→oldest, the first WRAP for this
/// coronet closes the epoch — every birth below it belongs to a line the wrap
/// converged, so the walk stops there. Among the births ABOVE that wrap (or all
/// births, if none wrapped), the EARLIEST is the founding one: a per-birth serial
/// makes each occupancy's dirname unique, but the branch is born once at the
/// founding saddle and re-seated by every saddle after it, so the founding
/// catchword is the one the live branch's name carries. That name is composed
/// here — `jjrf_livery_compose` with the pedigree's prefix — and handed to
/// board, whose seat/adopt arms then reawaken on a name that actually stands.
///
/// `None` when no live line stands: the coronet was never born, or the newest
/// event for it is a wrap. The caller births fresh from trunk — the founding
/// saddle of a new line.
pub fn jjrds_resolve_live_branch(marks: &[(u64, String)], coronet: &str, prefix: Option<&str>) -> Option<String> {
    let mut founding: Option<u64> = None;
    for (catchword, subject) in marks {
        match jjrds_classify_line_event(*catchword, subject) {
            // The most recent wrap for this coronet: the line below is closed.
            Some(jjrds_LineEvent::Wrap { coronet: c }) if c == coronet => break,
            // A birth in the current epoch: keep the earliest (min), so a re-saddle
            // that journals a fresh event never repoints the branch off its founding.
            Some(jjrds_LineEvent::Birth { coronet: c, catchword: cw }) if c == coronet => {
                founding = Some(founding.map_or(cw, |f| f.min(cw)));
            }
            _ => {}
        }
    }
    founding.map(|cw| {
        crate::jjrf_favor::jjrf_livery_compose(prefix, crate::jjrf_favor::jjrf_LiveryKind::Pace, cw, coronet)
    })
}

/// The live-line resolution's read side: pin the studbook's gleaned snapshot,
/// read its journal marks, and resolve the coronet's current branch
/// (`jjrds_resolve_live_branch`). `Ok(None)` is a coronet with no live line —
/// the founding-birth case the caller dresses fresh. A pin or read failure
/// surfaces loud (`StudbookUnreadable`): the door has already gleaned and
/// planned against this studbook, so a failure here is a genuine fault, never a
/// quiet fall-through to a fork.
pub fn jjrds_live_branch(
    studbook: &jjdb_BlotterConfig,
    coronet: &str,
    prefix: Option<&str>,
) -> Result<Option<String>, jjrds_Rejection> {
    let pin = jjdb_pin(studbook).map_err(|detail| jjrds_Rejection::StudbookUnreadable {
        path: studbook.local_root.clone(),
        detail,
    })?;
    let marks = jjdb_journal_marks(studbook, &pin).map_err(|detail| jjrds_Rejection::StudbookUnreadable {
        path: studbook.local_root.clone(),
        detail,
    })?;
    Ok(jjrds_resolve_live_branch(&marks, coronet, prefix))
}

/// Plan a dispatch: the approach's resolution half — identify at the captured
/// invocation path (the door captures cwd exactly once; this function never
/// reads the environment), pedigree lookup, target resolution, and the
/// two-source (tier, effort) choice. No mutation, no network: `over_studbook`
/// selects only WHERE the gallops and pedigree are read from, and both the
/// working-tree and ref-read forms are pure-local.
///
/// `over_studbook` is the enablement seam the door pins to
/// `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED` (`jjrds_run`); a test drives it `true`
/// while the const stays `false`. Off (the frozen path): the pedigree reads the
/// studbook working tree and the gallops the hippodrome's in-repo
/// `.claude/jjm/jjg_gallops.json`. On (the enabled path): one pin over the
/// fetched `origin/<trunk>` snapshot backs BOTH reads — gallops and pedigree
/// from one commit — and neither the in-repo gallops nor the studbook working
/// tree is touched. The pin is a pure-local ref-read; the currency glean that
/// advances it belongs to the door (`jjrds_currency`), never here.
pub fn jjrds_plan(
    door: jjrds_Door,
    raw_target: &str,
    cwd: &Path,
    over_studbook: bool,
) -> Result<jjrds_LaunchPlan, jjrds_Rejection> {
    let (identity, hippodrome_root, infield_root) = zjjrds_infield(cwd)?;

    // Pedigree lookup: derived key → sire → pedigree, then the record/ground
    // cross-check. A tree with no upstream cannot key a sire.
    let derived_key = identity.upstream_key.clone().ok_or_else(|| jjrds_Rejection::UnrecordedSire {
        key: "(no upstream configured on this clone)".to_string(),
    })?;
    let studbook = jjdb_studbook_config(&infield_root);

    // One pin backs every enabled-path read, so gallops and pedigree resolve
    // from one coherent commit. Pure-local (`jjdb_pin` reads the ref store); a
    // studbook with no fetched snapshot is unreadable here.
    let pin = if over_studbook {
        Some(jjdb_pin(&studbook).map_err(|detail| jjrds_Rejection::StudbookUnreadable {
            path: studbook.local_root.clone(),
            detail,
        })?)
    } else {
        None
    };

    let pedigree = match &pin {
        Some(pin) => jjrds_pedigree_lookup_pinned(&studbook, pin, &derived_key, JJRDS_KIND_PLAIN_GIT)?,
        None => jjrds_pedigree_lookup(&studbook, &derived_key, JJRDS_KIND_PLAIN_GIT)?,
    };

    // Target typing and door-specific resolution.
    let target = jjrds_type_target(raw_target)?;
    // Both doors may need the gallops now — saddle always, lunge only for a
    // coronet aim (whose heat the yard label is read from) — so the read is
    // composed once here and taken on demand, leaving a heat-aimed lunge as
    // gallops-free as it has always been.
    let gallops = || -> Result<crate::jjri_io::jjdr_ValidatedGallops, jjrds_Rejection> {
        match &pin {
            Some(pin) => {
                let bytes = jjdb_read_pinned(&studbook, pin, JJDB_GALLOPS_REL_PATH).map_err(|detail| {
                    jjrds_Rejection::StudbookUnreadable {
                        path: studbook.local_root.clone(),
                        detail,
                    }
                })?;
                crate::jjri_io::jjdr_hark(&bytes).map_err(|e| jjrds_Rejection::BadTarget { detail: e })
            }
            None => {
                let gallops_path = hippodrome_root.join(".claude/jjm/jjg_gallops.json");
                crate::jjri_io::jjdr_load(&gallops_path).map_err(|e| jjrds_Rejection::BadTarget { detail: e })
            }
        }
    };

    let (livery_prefix, identity_body, aim, designation, opening_prompt) = match door {
        jjrds_Door::Saddle => {
            let gallops = gallops()?;
            let saddled = jjrds_resolve_saddle(gallops.inner(), &target)?;
            let prompt = format!(
                "mount {}{}",
                crate::jjrf_favor::JJRF_CORONET_PREFIX,
                saddled.coronet
            );
            // The branch wears the livery badge; the dirname stays the bare
            // body under the yard signet. Two different surfaces: the yard is
            // JJ's own infield, where a bare body is unambiguous, while the
            // branch lands in the sire's ref store, which JJ does not own. Both
            // labels are composed at the mint from the one birth catchword — the
            // plan carries only the prefix, since the serial is not known until
            // the dispatch record lands.
            (
                pedigree.livery_prefix.clone(),
                saddled.coronet.clone(),
                jjrds_Target::Coronet(saddled.coronet.clone()),
                saddled.designation,
                prompt,
            )
        }
        jjrds_Door::Lunge => {
            // Both aims share this ground, and both label their billet with a
            // firemark: a heat aim wears its own, a pace aim wears its pace's
            // heat. The label is never the coronet, because the dirname's
            // identity is the yard's kind channel (JJSVD "The billet") — the
            // yard gate's yard key and muck's kind resolution both read a
            // coronet dirname as a pace billet, and a groom is not one.
            let firemark = match &target {
                jjrds_Target::Firemark(fm) => fm.clone(),
                jjrds_Target::Coronet(coronet) => jjrds_groomed_heat(gallops()?.inner(), coronet)?,
            };
            // The verb, then the aim, then the posture — the door's first
            // impression, so the session reads what its ground affords before it
            // has done anything the ground would refuse. A pace aim is named in
            // the qualified emission form, which carries the heat beside the
            // coronet, so the groom opens already in its heat's context.
            let mark = match &target {
                jjrds_Target::Firemark(fm) => format!("{}{}", crate::jjrf_favor::JJRF_FIREMARK_PREFIX, fm),
                jjrds_Target::Coronet(coronet) => format!(
                    "{}{}{}{}",
                    crate::jjrf_favor::JJRF_CORONET_PREFIX,
                    firemark,
                    crate::jjrf_favor::JJRF_CORONET_QUALIFIER,
                    coronet
                ),
            };
            let prompt = format!("groom {}\n\n{}", mark, JJRDS_GROOM_POSTURE);
            (None, firemark, target.clone(), None, prompt)
        }
    };

    let (tier, effort) = jjrds_resolve_launch(designation);

    Ok(jjrds_LaunchPlan {
        door,
        livery_prefix,
        identity_body,
        aim,
        hippodrome_root,
        infield_root,
        trunk: pedigree.trunk,
        tier,
        effort,
        opening_prompt,
    })
}

/// Does the pace's branch stand abroad — is another station's pushed work
/// waiting for this one to adopt? The glean rides here rather than at the top of
/// the ensure because this is the sole question whose answer it changes: the
/// counterpart read is network-silent, so without a fresh fetch it would answer
/// from whatever this station last happened to see, and a stale no forks.
///
/// An unreachable remote is not a refusal: the read simply answers from what was
/// last seen, and an offline station falls through to birth. Adopting late is
/// the cost of working offline; refusing the dispatch would be a larger one.
fn zjjrds_stands_abroad<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    hippodrome_root: &Path,
    branch: &str,
) -> Result<bool, jjrds_Rejection> {
    let _ = farrier.jjrfr_glean(hippodrome_root);
    farrier
        .jjrfr_line_abroad(hippodrome_root, branch)
        .map_err(jjrds_Rejection::Farrier)
}

/// Board the billet: the approach's mutation half — billet ensure
/// (seat-or-adopt-or-create; a groom billet in reuse re-detaches to trunk tip), then
/// glean (the approach fetches and never merges), then the staleness probe whose
/// answer the launch surfaces. Returns the staleness notice, if any.
///
/// `birth` arrives from the caller rather than the plan: a pace's branch name
/// carries the birth catchword, minted only when the dispatch record lands, so
/// the serialed `Branch` is composed at the mint (`jjrf_livery_compose`) and
/// handed in here — the plan carried only the prefix.
pub fn jjrds_board<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    plan: &jjrds_LaunchPlan,
    birth: &jjrfr_BilletBirth,
    yard: &jjrds_Yard,
) -> Result<Option<String>, jjrds_Rejection> {
    if yard.billet_root.exists() {
        // Board is total over billet-root existence, so a partition already
        // standing at this path is honoured rather than clobbered — a pace
        // billet already seats its own branch and needs no ensure. The yard
        // gate refuses a standing pace billet upstream, so the dispatch path
        // never reaches here with one; a partition present at a freshly minted
        // path is an operator's own hand-built directory, and a groom's is
        // re-detached to the freshest trunk tip — the honest reading of that.
        if *birth == jjrfr_BilletBirth::Detached {
            farrier
                .jjrfr_billet_detach(&yard.billet_root, &plan.trunk)
                .map_err(jjrds_Rejection::Farrier)?;
        }
    } else {
        // Under per-birth serials a fresh birth's branch never pre-exists, so the
        // seat and adopt arms are dormant on the ordinary dispatch path: they fire
        // only for a birth whose branch a caller resolved from an existing
        // occupancy rather than minting fresh. Held, not derivation-based, so the
        // resolution that reawakens them can be supplied without reshaping board.
        match birth {
            jjrfr_BilletBirth::Branch(branch)
                if farrier
                    .jjrfr_line_exists(&plan.hippodrome_root, branch)
                    .map_err(jjrds_Rejection::Farrier)? =>
            {
                // The durable branch survives its reaped billet: re-seat it.
                // A registry that records the branch seated elsewhere refuses
                // here by name — seat-vestige or line-seated, each carrying its
                // own remedy (`jjrfr_billet_seat`).
                farrier
                    .jjrfr_billet_seat(&plan.hippodrome_root, branch, &yard.billet_root)
                    .map_err(jjrds_Rejection::Farrier)?;
            }
            jjrfr_BilletBirth::Branch(branch)
                if zjjrds_stands_abroad(farrier, &plan.hippodrome_root, branch)? =>
            {
                // Absent at home, standing abroad: another station has worked
                // this pace and pushed. Adopting its line is what makes one
                // pace one line of work across stations — a birth here would
                // fork a rival from trunk that no ceremony ever reconciles.
                farrier
                    .jjrfr_billet_adopt(&plan.hippodrome_root, branch, &yard.billet_root)
                    .map_err(jjrds_Rejection::Farrier)?;
            }
            birth => {
                farrier
                    .jjrfr_billet_create(&plan.hippodrome_root, birth, &yard.billet_root, &plan.trunk)
                    .map_err(jjrds_Rejection::Farrier)?;
            }
        }
    }

    // Glean: staleness becomes known here so the open can report it; refit is
    // the remedy. The probe is meaningful for a pace billet's branch; a groom
    // billet just re-detached to the freshest counterpart this station knew.
    let _ = farrier.jjrfr_glean(&yard.billet_root);
    match birth {
        jjrfr_BilletBirth::Branch(_) => {
            jjrds_staleness_notice(farrier, &yard.billet_root, &plan.trunk).map_err(jjrds_Rejection::Farrier)
        }
        jjrfr_BilletBirth::Detached => Ok(None),
    }
}

// ---- Stirrup: the launch primitive ----

/// The session-scoped MCP config content, generated per dispatch (JJSVD
/// "Launch-time provisioning"): hippodromes carry no JJ inserts, so MCP
/// registration arrives from the approach, pointing at the kit repo's vvx.
pub fn jjrds_mcp_config_json(kit_root: &Path) -> String {
    serde_json::json!({
        "mcpServers": {
            "vvx": {
                "command": kit_root.join("Tools/vvk/bin/vvx").to_string_lossy(),
                "args": ["mcp"]
            }
        }
    })
    .to_string()
}

/// Chain-export predicate (BUr_q2m): true for every env key the dispatch
/// chain computes fresh on each invocation — the whole `BURD_`/`BURC_`
/// families (`bud_dispatch.sh`'s allowlist, `burc_regime.sh`'s enrollment)
/// plus the one ungoverned `JJSL_INVOKE_DIR` name — never a name the
/// operator's own shell set. `BURE_*` (operator ambient) and `BURV_*` (the
/// per-billet payload composed at the stirrup) are deliberately not matched.
/// Census: `Memos/memo-20260728-stirrup-env-composition-census.md`.
pub(crate) fn zjjrds_is_chain_export(key: &str) -> bool {
    key.starts_with("BURD_") || key.starts_with("BURC_") || key == "JJSL_INVOKE_DIR"
}

/// Removes every chain-export key found in `source_env` from `cmd`. Takes
/// the env pairs as a parameter rather than reading `std::env::vars()`
/// itself so the strip is unit-testable against a synthetic env without
/// mutating the test process's own (shared, thread-racy) environment.
pub(crate) fn zjjrds_strip_chain_exports(
    cmd: &mut std::process::Command,
    source_env: impl Iterator<Item = (String, String)>,
) {
    for (key, _) in source_env {
        if zjjrds_is_chain_export(&key) {
            cmd.env_remove(key);
        }
    }
}

/// Stirrup — the launch primitive at the approach's end: pace-blind,
/// parameterized (billet, tier, opening prompt); pace-coupling lives in the
/// caller. The one consumer of the tier roster: callers speak tier words,
/// never model IDs, and an invalid (family, effort) pair refuses fair-facedly.
/// Returns the composed command, cwd set inside the billet, env carrying the
/// per-billet BURV exports (the BUK meld: output, temp, and the log-dir
/// override) and stripped of every chain export the door's own dispatch ran
/// (BUr_q2m) — parity with a hand-launched hippodrome session, operator
/// ambient (`BURE_*` and vendor env) passed through untouched — ready to
/// spawn with inherited stdio.
pub fn jjrds_stirrup_command(
    billet_root: &Path,
    tier: jjrg_Tier,
    effort: Option<jjrg_Effort>,
    opening_prompt: &str,
    mcp_config_path: &Path,
    scratch_root: &Path,
) -> Result<std::process::Command, jjrds_Rejection> {
    let row = jjrds_roster_row(tier);
    if let Some(e) = effort {
        if !jjrds_pair_admitted(row, e) {
            return Err(jjrds_Rejection::BadLaunchPair {
                family: tier.jjrg_as_str().to_string(),
                effort: e.jjrg_as_str().to_string(),
            });
        }
    }
    let mut cmd = std::process::Command::new("claude");
    cmd.current_dir(billet_root);
    cmd.arg("--model").arg(row.model_id);
    if let Some(e) = effort {
        cmd.arg("--effort").arg(e.jjrg_as_str());
    }
    cmd.arg("--permission-mode").arg("auto");
    cmd.arg("--mcp-config").arg(mcp_config_path);
    cmd.arg("--append-system-prompt").arg(JJRDS_CONDUCT_CORE);
    cmd.arg(opening_prompt);
    cmd.env("BURV_OUTPUT_ROOT_DIR", scratch_root.join("output-buk"));
    cmd.env("BURV_TEMP_ROOT_DIR", scratch_root.join("temp-buk"));
    cmd.env("BURV_LOG_DIR", scratch_root.join("logs-buk"));
    // BUr_q2m: the launched session is a new dispatch context — it takes its
    // dispatch modes and regime config from its own inlets, never from this
    // door's. Every BURD_/BURC_ name and JJSL_INVOKE_DIR are this door's own
    // dispatch-chain computation and stop here, so every tabtarget the
    // session runs re-derives its own fresh dispatch context under the BURV
    // roots composed above.
    zjjrds_strip_chain_exports(&mut cmd, std::env::vars());
    Ok(cmd)
}

// ---- The door driver (CLI entry) ----

/// The approach's terminal shape. Either the dispatch finishes here — a refusal, a
/// dry run, or a provisioning failure, with the report string carrying the whole
/// of what to say and the code the exit code — or a session stands composed and
/// ready to launch. The launch is the one console-handoff I/O effect, and it is
/// the caller's, never this module's: the caller prints the report first and
/// then hands the terminal over, so the door's whole report reaches the operator
/// BEFORE the session it introduces (JJSVD "Report precedes launch").
pub enum jjrds_Outcome {
    /// Nothing to launch: the `i32` is the exit code; the report string is all
    /// there is to print.
    Done(i32),
    /// A composed session ready to launch. The caller prints the report, then
    /// hands it the terminal; the session's own exit code becomes the dispatch's.
    /// `billet_root`/`trunk` ride along so the caller can run the stile's
    /// trailing step (`jjrds_trailing_step`) against the same billet once the
    /// session returns — the approach resolves both already; re-deriving them from
    /// the launched `Command` would mean parsing its own `current_dir` back out.
    Launch { cmd: std::process::Command, billet_root: PathBuf, trunk: String },
    /// A billet stands cleanly for this pace: the report (already in this
    /// outcome's text) names it, and the caller must obtain an attended confirm
    /// before the resume launches. On confirm the caller runs `jjrds_resume`
    /// (which returns a `Launch`); on decline it stops, changing nothing. The
    /// confirm is held at the door driver (JJSVD "The stile").
    Standing { resume: jjrds_ResumePlan },
}

/// The coordinates a confirmed resume launches from: an existing billet reused
/// in place. No catchword, no serial, no birth record — the standing worktree
/// and its seated livery branch ARE the state (JJSVD "Yard step", resume arm),
/// so a resume carries only what the launch needs, not a fresh birth's ceremony.
#[derive(Debug)]
pub struct jjrds_ResumePlan {
    pub billet_root: PathBuf,
    pub branch: String,
    pub infield_root: PathBuf,
    pub trunk: String,
    pub tier: jjrg_Tier,
    pub effort: Option<jjrg_Effort>,
    pub opening_prompt: String,
    pub kit_root: PathBuf,
}

/// Resolve one dispatch to the point of launch — plan, board, provision, and
/// compose the session command — but do NOT launch it. `dry_run` stops after
/// planning and reports the resolved plan (the rehearsal and debugging surface).
/// The returned report string is always what to print; the outcome says whether
/// a session remains for the caller to launch. Keeping the console-handoff out
/// of this function is what lets the caller emit the report before the session
/// takes the terminal (JJSVD "Report precedes launch").
pub fn jjrds_run(door: jjrds_Door, raw_target: &str, cwd: &Path, kit_root: &Path, dry_run: bool) -> (jjrds_Outcome, String) {
    let mut out = String::new();
    let farrier = jjrfg_PlainGit;

    // Currency at the door: over the studbook, glean it fresh (an Unreachable
    // glean refuses) and courtesy-sight for an in-flight write BEFORE the
    // pure-local pinned read plan takes. Skipped while the seam is closed —
    // the frozen path reads the in-repo gallops and needs no studbook glean.
    let over_studbook = JJDB_GALLOPS_OVER_STUDBOOK_ENABLED;
    if over_studbook {
        match zjjrds_infield(cwd) {
            Ok((_, _, infield_root)) => {
                let studbook = jjdb_studbook_config(&infield_root);
                if let Err(e) = jjrds_currency(&farrier, &studbook, JJRDS_CURRENCY_BEAT) {
                    return (jjrds_Outcome::Done(1), format!("dispatch refused: {}\n", e));
                }
            }
            Err(e) => return (jjrds_Outcome::Done(1), format!("dispatch refused: {}\n", e)),
        }
    }

    let plan = match jjrds_plan(door, raw_target, cwd, over_studbook) {
        Ok(p) => p,
        Err(e) => return (jjrds_Outcome::Done(1), format!("dispatch refused: {}\n", e)),
    };

    // The yard gate: fork on a pace whose billet already stands, before the birth
    // record's journal write and before any session spawn, so a re-saddle can
    // never rejoin a live worktree or mint a rival past a missed seat. A clean
    // standing billet resumes; an anomalous one refuses (muck the remedy).
    // Pure-local, so it runs even on a dry run; the birth record it clears the way
    // for is a studbook write and waits past the dry-run stop.
    let verdict = match jjrds_yard_gate(&farrier, &plan) {
        Ok(v) => v,
        Err(e) => return (jjrds_Outcome::Done(1), format!("dispatch refused: {}\n", e)),
    };

    out.push_str(&format!(
        "launch:  {} / {}\nprompt:  {}\n",
        plan.tier.jjrg_as_str(),
        plan.effort.map(|e| e.jjrg_as_str()).unwrap_or("(vendor default)"),
        plan.opening_prompt,
    ));

    // A clean standing billet resumes in place: report it, and hand the caller a
    // resume plan to launch once the attended confirm answers. No birth record,
    // no serial. The dry run stops at the report, exactly as the fresh path stops
    // before its mint.
    if let jjrds_YardVerdict::Resume { root, branch } = verdict {
        out.push_str(&zjjrds_resume_report(&farrier, &root, &branch));
        if dry_run {
            out.push_str("dry run: stopping before the resume confirm and launch (would re-enter this billet)\n");
            return (jjrds_Outcome::Done(0), out);
        }
        let resume = jjrds_ResumePlan {
            billet_root: root,
            branch,
            infield_root: plan.infield_root.clone(),
            trunk: plan.trunk.clone(),
            tier: plan.tier,
            effort: plan.effort,
            opening_prompt: plan.opening_prompt.clone(),
            kit_root: kit_root.to_path_buf(),
        };
        return (jjrds_Outcome::Standing { resume }, out);
    }

    if dry_run {
        out.push_str("dry run: stopping before the dispatch record, board, and launch (would mint a billet)\n");
        return (jjrds_Outcome::Done(0), out);
    }

    // The gate passed, so nothing stands. Before minting this dispatch's own
    // birth record, resolve whether a prior line already stands for this pace —
    // the studbook-routed resumption (`jjdd_billet`): a saddle whose coronet was
    // born before (its billet since reaped, or worked and pushed on another
    // station) re-seats or adopts that line rather than forking a rival from
    // trunk. The read takes the journal as it stood BEFORE this event, so the
    // question stays "was there a prior line," and it is meaningful only over the
    // studbook — the frozen path has no journal and always births fresh.
    let studbook = jjdb_studbook_config(&plan.infield_root);
    let resolved_line: Option<String> = if over_studbook && matches!(plan.door, jjrds_Door::Saddle) {
        match jjrds_live_branch(&studbook, &plan.identity_body, plan.livery_prefix.as_deref()) {
            Ok(line) => line,
            Err(e) => return (jjrds_Outcome::Done(1), format!("{}dispatch refused resolving the live line: {}\n", out, e)),
        }
    } else {
        None
    };

    // Mint the billet: the birth record allocates the catchword the dirname wears.
    // A founding saddle with no prior line dresses the branch from that same
    // catchword; a re-seat or adopt keeps the dirname's fresh serial but dresses
    // the branch in the RESOLVED name, so the dirname labels this occupancy while
    // the branch stays the founding line's — the two denormalized labels part
    // exactly where the resumption reuses a durable branch.
    let (billet_root, birth) = {
        let station = crate::jjrvg_guidon::jjdb_station_name();
        match jjrds_record_dispatch(&farrier, &studbook, &plan, &station) {
            Ok(catchword) => {
                let root = plan.infield_root.join(jjrds_billet_dirname(catchword, &plan.identity_body));
                let birth = match plan.door {
                    jjrds_Door::Saddle => jjrfr_BilletBirth::Branch(resolved_line.unwrap_or_else(|| {
                        crate::jjrf_favor::jjrf_livery_compose(
                            plan.livery_prefix.as_deref(),
                            crate::jjrf_favor::jjrf_LiveryKind::Pace,
                            catchword,
                            &plan.identity_body,
                        )
                    })),
                    jjrds_Door::Lunge => jjrfr_BilletBirth::Detached,
                };
                (root, birth)
            }
            Err(e) => return (jjrds_Outcome::Done(1), format!("{}dispatch refused at the record: {}\n", out, e)),
        }
    };
    let yard = jjrds_yard(&plan.infield_root, billet_root);

    out.push_str(&format!(
        "billet:  {}  ({})\n",
        yard.billet_root.display(),
        match &birth {
            jjrfr_BilletBirth::Branch(b) => format!("branch {}", b),
            jjrfr_BilletBirth::Detached => "detached at trunk tip".to_string(),
        },
    ));

    let staleness = match jjrds_board(&farrier, &plan, &birth, &yard) {
        Ok(s) => s,
        Err(e) => return (jjrds_Outcome::Done(1), format!("{}dispatch refused at boarding: {}\n", out, e)),
    };
    if let Some(notice) = &staleness {
        out.push_str(&format!("{}\n", notice));
    }

    // Provision: the session-scoped MCP config and the per-billet BUK scratch.
    for sub in ["output-buk", "temp-buk", "logs-buk"] {
        if let Err(e) = std::fs::create_dir_all(yard.scratch_root.join(sub)) {
            return (jjrds_Outcome::Done(1), format!("{}dispatch failed provisioning scratch at {}: {}\n", out, yard.scratch_root.display(), e));
        }
    }
    let mcp_path = yard.scratch_root.join("mcp.json");
    if let Err(e) = std::fs::write(&mcp_path, jjrds_mcp_config_json(kit_root)) {
        return (jjrds_Outcome::Done(1), format!("{}dispatch failed writing MCP config at {}: {}\n", out, mcp_path.display(), e));
    }

    let cmd = match jjrds_stirrup_command(
        &yard.billet_root,
        plan.tier,
        plan.effort,
        &plan.opening_prompt,
        &mcp_path,
        &yard.scratch_root,
    ) {
        Ok(c) => c,
        Err(e) => return (jjrds_Outcome::Done(1), format!("{}dispatch refused at stirrup: {}\n", out, e)),
    };

    (jjrds_Outcome::Launch { cmd, billet_root: yard.billet_root, trunk: plan.trunk.clone() }, out)
}

/// Compose a confirmed resume's launch: reuse the standing billet in place. No
/// birth record (no catchword, no serial) and no billet-ensure — the worktree
/// stands and its livery branch is seated, so `jjrds_yard` re-derives the
/// EXISTING dirname and its existing scratch, and the crossing rejoins the same
/// billet the earlier session left. The tail mirrors `jjrds_run` from the yard
/// onward, minus everything a fresh birth owes: glean so staleness can be
/// reported (`jjdd_refit` the remedy), provision the per-billet scratch, stirrup.
/// Returns a `Launch` on the same contract as a fresh dispatch, so the door
/// driver's launch-and-trail path handles a resume identically.
pub fn jjrds_resume<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    resume: &jjrds_ResumePlan,
) -> (jjrds_Outcome, String) {
    let mut out = String::new();
    let yard = jjrds_yard(&resume.infield_root, resume.billet_root.clone());
    out.push_str(&format!("resuming: {}  (branch {})\n", yard.billet_root.display(), resume.branch));

    // Glean so staleness becomes known and the resume can report it; the billet
    // stands and its branch is seated, so nothing is ensured or born.
    let _ = farrier.jjrfr_glean(&yard.billet_root);
    match jjrds_staleness_notice(farrier, &yard.billet_root, &resume.trunk) {
        Ok(Some(notice)) => out.push_str(&format!("{}\n", notice)),
        Ok(None) => {}
        Err(e) => return (jjrds_Outcome::Done(1), format!("{}resume refused at staleness probe: {}\n", out, jjrds_Rejection::Farrier(e))),
    }

    // Provision: the session-scoped MCP config and the per-billet BUK scratch,
    // keyed by the existing dirname — the resumed billet shares the scratch the
    // birth first minted for it.
    for sub in ["output-buk", "temp-buk", "logs-buk"] {
        if let Err(e) = std::fs::create_dir_all(yard.scratch_root.join(sub)) {
            return (jjrds_Outcome::Done(1), format!("{}resume failed provisioning scratch at {}: {}\n", out, yard.scratch_root.display(), e));
        }
    }
    let mcp_path = yard.scratch_root.join("mcp.json");
    if let Err(e) = std::fs::write(&mcp_path, jjrds_mcp_config_json(&resume.kit_root)) {
        return (jjrds_Outcome::Done(1), format!("{}resume failed writing MCP config at {}: {}\n", out, mcp_path.display(), e));
    }

    let cmd = match jjrds_stirrup_command(
        &yard.billet_root,
        resume.tier,
        resume.effort,
        &resume.opening_prompt,
        &mcp_path,
        &yard.scratch_root,
    ) {
        Ok(c) => c,
        Err(e) => return (jjrds_Outcome::Done(1), format!("{}resume refused at stirrup: {}\n", out, e)),
    };

    (jjrds_Outcome::Launch { cmd, billet_root: yard.billet_root, trunk: resume.trunk.clone() }, out)
}

// ---- The stile's trailing step ----

/// The stile's closing act (`jjdd_stile`, JJSVD "The stile"): once the launched
/// session has returned, run the kind-aware exit litmus against the billet it
/// ran in and destroy a passing one (`jjrfr_billet_remove`, un-forced —
/// `billet_remove`'s own internal `comb` check is the same verdict this litmus
/// renders, so a cold build's ignored residue never blocks either). Automatic,
/// no confirm: the litmus is the proof that destruction loses nothing.
///
/// The caller is the door driver, sitting outside the billet as the launched
/// session's own parent — the geometry the approach never reaches on its own,
/// since `jjrds_run` only composes the command and returns before it is
/// spawned. One line reports the outcome either way (JJSVD "The stile"): a
/// cleared billet names where the work now stands — the destroyed worktree
/// being precisely where it no longer does — and a standing billet names the
/// failed conjunct and `muck` as the remedy. A cleared billet's scratch
/// sibling dies with it — scratch is forensics only for a billet that stands;
/// a standing billet's scratch is left for `muck` to clear with the rest of
/// its residue.
pub fn jjrds_trailing_step<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(farrier: &F, billet_root: &Path, trunk: &str) -> String {
    let identity = match farrier.jjrfr_identify(billet_root) {
        Ok(id) => id,
        Err(e) => return format!("stile: billet stands at {} — could not identify it: {}\n", billet_root.display(), e),
    };
    let ground = jjrds_ground_of(&identity);
    let verdict = match &ground {
        jjrds_Ground::PaceBillet { .. } => zjjrds_stile_pace_verdict(farrier, billet_root, trunk),
        jjrds_Ground::GroomBillet => zjjrds_stile_groom_verdict(farrier, billet_root, trunk),
        // Neither ground the door ever seats a session in — the litmus is total
        // rather than partial, and entitles destruction to neither.
        jjrds_Ground::Hippodrome | jjrds_Ground::Unboarded { .. } => Ok(zjjrds_StileVerdict::NotABillet),
    };
    match verdict {
        Ok(zjjrds_StileVerdict::Passes) => match farrier.jjrfr_billet_remove(billet_root, false) {
            Ok(()) => {
                if let Some(infield_root) = billet_root.parent() {
                    let _ = std::fs::remove_dir_all(jjrds_yard(infield_root, billet_root.to_path_buf()).scratch_root);
                }
                format!("stile: billet cleared ({}) — {}\n", billet_root.display(), zjjrds_where_it_stands(&identity, trunk))
            }
            Err(e) => format!("stile: billet stands at {} — {}\n", billet_root.display(), e),
        },
        Ok(conjunct) => format!(
            "stile: billet stands at {} ({}) — {} — `muck` to clear it, or exit clean later\n",
            billet_root.display(),
            ground.jjrds_as_str(),
            conjunct.zjjrds_as_str()
        ),
        Err(e) => format!("stile: billet stands at {} — {}\n", billet_root.display(), e),
    }
}

/// Where the work stands once a passing billet is cleared, for the cleared line
/// (JJSVD "The stile": "a cleared billet names where the work stands"). The
/// reaped worktree is not the answer — it is the one place the work no longer
/// is. A pace billet's work stands on its durable branch: `billet_remove` takes
/// only the worktree, and the branch survives in the primary's ref store to
/// re-seat from — pushed, where the custody pass proved it not ahead of its own
/// counterpart; a local-only marker, where the content-proof pass cleared a
/// dropped no-work billet the branch never carried anything of value onto. A
/// groom billet seats no branch and carried nothing of its own; content-empty
/// against trunk's custody base, its position is already in trunk.
fn zjjrds_where_it_stands(identity: &crate::jjrfr_farrier::jjrfr_Identity, trunk: &str) -> String {
    match &identity.line_of_work {
        jjrfr_LineOfWork::Branch(name) => format!("work stands on branch {}", name),
        jjrfr_LineOfWork::Detached(_) => format!("work stands in trunk {}", trunk),
    }
}

/// The exit litmus's verdict — named per JJSVD "The stile": a standing billet
/// names the failed conjunct, not just the fact of standing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum zjjrds_StileVerdict {
    Passes,
    Dirty,
    /// Pace arm: local commits exist beyond the billet's own counterpart, and
    /// the content-proof pass did not clear them either — real work trunk lacks.
    AheadOfCounterpart,
    /// Pace arm: never consigned (no counterpart of its own), and the
    /// content-proof pass did not clear it — it carries content beyond trunk's
    /// custody base, or nothing can be proven held at all. The re-scoped
    /// ignorance-stands arm: a derivable custody base is proof, so only genuine
    /// content or total ignorance stands here (JJSVD).
    Untracked,
    /// Groom arm: the detached tip carries content beyond trunk's counterpart.
    Unreachable,
    /// A ground the door never seats a session in.
    NotABillet,
}

impl zjjrds_StileVerdict {
    fn zjjrds_as_str(&self) -> &'static str {
        match self {
            zjjrds_StileVerdict::Passes => "passes",
            zjjrds_StileVerdict::Dirty => "uncommitted changes",
            zjjrds_StileVerdict::AheadOfCounterpart => "commits not yet in remote custody",
            zjjrds_StileVerdict::Untracked => "never consigned — no counterpart known",
            zjjrds_StileVerdict::Unreachable => "detached tip carries content beyond trunk's counterpart",
            zjjrds_StileVerdict::NotABillet => "not a billet the stile boards",
        }
    }
}

/// The pace-billet arm: clean, AND cleared by either of two independent passes.
/// The *custody pass* — its tip not ahead of its own counterpart, every commit
/// already in remote custody — clears a consigned-not-wrapped billet whose real
/// work is safe on its own pushed branch. The *content-proof pass* — no content
/// added beyond trunk's custody base — clears a dropped no-work billet whose
/// only commit is the officium marker, though it was never consigned: a
/// derivable custody base is proof, not ignorance. The passes are additive, not
/// a replacement; only when neither holds — genuine unconsigned content, or
/// total ignorance of any custody base — does the billet stand, named from the
/// sync posture.
fn zjjrds_stile_pace_verdict<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    billet_root: &Path,
    trunk: &str,
) -> Result<zjjrds_StileVerdict, jjrfr_Rejection> {
    let comb = farrier.jjrfr_comb(billet_root)?;
    if !comb.jjrfr_is_clean() {
        return Ok(zjjrds_StileVerdict::Dirty);
    }
    let sync = farrier.jjrfr_sync_state(billet_root)?;
    // Custody pass: every commit already in the billet's own remote custody.
    if matches!(sync, jjrfr_SyncState::Tracking { ahead: 0, .. }) {
        return Ok(zjjrds_StileVerdict::Passes);
    }
    // Content-proof pass: nothing added beyond trunk's custody base, so the
    // billet holds nothing trunk does not — the dropped no-work billet clears
    // even though it was never consigned.
    if farrier.jjrfr_reachable(billet_root, trunk)? {
        return Ok(zjjrds_StileVerdict::Passes);
    }
    // Neither pass: the billet carries unconsigned content, or nothing can be
    // proven held at all.
    Ok(match sync {
        jjrfr_SyncState::Tracking { .. } => zjjrds_StileVerdict::AheadOfCounterpart,
        jjrfr_SyncState::Untracked => zjjrds_StileVerdict::Untracked,
    })
}

/// The groom-billet arm: clean AND cleared by the content-proof pass — every
/// commit beyond trunk's custody base is tree-identical to its parent, so a
/// marker-only commit (every dispatch's `jjdo_open` echo) passes while a raw
/// detached commit with real content stands. A groom is detached and never
/// consigned, so the content-proof pass is its whole litmus — the pace arm's
/// custody pass has no counterpart here.
fn zjjrds_stile_groom_verdict<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    billet_root: &Path,
    trunk: &str,
) -> Result<zjjrds_StileVerdict, jjrfr_Rejection> {
    let comb = farrier.jjrfr_comb(billet_root)?;
    if !comb.jjrfr_is_clean() {
        return Ok(zjjrds_StileVerdict::Dirty);
    }
    if farrier.jjrfr_reachable(billet_root, trunk)? {
        Ok(zjjrds_StileVerdict::Passes)
    } else {
        Ok(zjjrds_StileVerdict::Unreachable)
    }
}
