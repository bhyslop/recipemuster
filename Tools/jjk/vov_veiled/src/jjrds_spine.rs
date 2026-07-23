// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Dispatch spine — the one shared sequence behind both doors (`jjdd_spine`,
//! JJSVD-dispatch.adoc): saddle and lunge contribute only their target typing
//! and their tier source; everything else runs here. This module composes
//! `jjdf_farrier` primitives and the blotter's engine-known config — it owns no
//! git of its own.
//!
//! Spine order (JJSVD "The entrance spine"): muck — built in `jjrdm_muck`,
//! its slot is the leading step, but NOT YET CALLED from `jjrds_run` below:
//! muck reads pace-closed state from the studbook's gallops copy, which
//! stays a parallel, non-authoritative path until the cutover ceremony,
//! while `jjrds_plan`/`jjrds_board` still read the frozen local gallops —
//! wiring muck here before that cutover would let it judge every pace
//! "open" (or, for a groom billet, judge on no pace at all) against a copy
//! nothing keeps current — then identify at the captured invocation path,
//! pedigree lookup (one indirection: derived key → sire → pedigree), billet
//! ensure, glean, BURV export, provision, launch. The launch primitive is
//! stirrup: pace-blind, parameterized (billet, tier, opening prompt);
//! pace-coupling lives in the callers here.
//!
//! Inertness: nothing on the frozen path reaches this module's doors — they
//! are new opt-in surfaces (a station without a founded studbook meets the
//! fair-faced studbook rejection at pedigree lookup), and `jjrds_run`'s spine
//! itself still runs without muck (above). The staleness surfacing composed
//! here is no longer inert, though: `jjrds_staleness_notice` is wired into the
//! live `jjx_open` path unconditionally (`zjjrm_open_staleness_notice`,
//! jjrm_mcp.rs) — that wiring does not wait on
//! `JJRM_OFFICIUM_STUDBOOK_ENABLED`, which gates only where the officium's own
//! exchange directory lives, not this probe. Notch/wrap wiring remains
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
};
use crate::jjrt_types::{
    jjrg_Effort,
    jjrg_PaceState,
    jjrg_Tier,
};
use crate::jjrvb_blotter::{
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct jjrds_Pedigree {
    #[serde(rename = "jjop_kind")]
    pub kind: String,
    #[serde(rename = "jjop_addresses")]
    pub addresses: Vec<String>,
    #[serde(rename = "jjop_trunk")]
    pub trunk: String,
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
    let file = zjjrds_PedigreeFile { sires };
    serde_json::to_string_pretty(&file).map_err(|e| format!("pedigrees seed: could not serialize: {}", e))
}

// ---- Spine rejections ----

/// The spine's fair-faced refusals (JJSVD "Rejections"): named per the farrier
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
    /// The dispatch target token failed halter typing, or resolution against
    /// the gallops (unknown identity, no actionable pace, terminal pace state).
    BadTarget { detail: String },
    /// An invalid (family, effort) launch pair at stirrup.
    BadLaunchPair { family: String, effort: String },
    /// A farrier primitive rejected mid-spine (e.g. a dirty groom billet at
    /// re-detach).
    Farrier(jjrfr_Rejection),
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
            jjrds_Rejection::BadTarget { detail } => write!(f, "bad dispatch target: {}", detail),
            jjrds_Rejection::BadLaunchPair { family, effort } => {
                write!(f, "invalid launch pair: family '{}' does not admit effort '{}'", family, effort)
            }
            jjrds_Rejection::Farrier(r) => write!(f, "{}", r),
        }
    }
}

/// Pedigree lookup — the spine's studbook read: lock-free (`jjdk_lockless_reads`),
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
- Pull door — context on demand, before improvising: jjx_get_spec for operation specs, jjx_brief {coronet} for a pace docket, jjx_paddock {firemark} for heat shape. If context seems missing, pull it.\n\
- If the mounted pace is bridled at a sub-frontier tier (haiku, sonnet): designee protocol — orient, work the docket, jjx_record, finish with jjx_landing; never wrap; stop and surface on any hole.\n\
- Otherwise (unbridled, or bridled at your own frontier tier): full ceremony; never auto-wrap — ask the operator.\n";

/// The staleness recommendation body — one text (JJSVD "Refit"). `jjx_open`
/// leads with it today (`zjjrm_open_staleness_notice`, `jjrm_mcp.rs`); notch
/// and wrap are to append the same text once their own wiring lands. Names
/// refit as the remedy; refit is ashlar, so the words here are operator-facing.
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
/// typing and its tier source; the spine below is shared.
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

// ---- The yard: billet and scratch naming ----

/// The billet dirname signet (`jjdw_yard`): `jjqb_{coronet}` for a pace billet,
/// `jjqb_{firemark}` for a groom billet — one signet, typed by length exactly
/// as identities are everywhere. The muck sweep's positive glob (`jjqb_*`)
/// keys on this prefix.
pub const JJRDS_BILLET_DIR_PREFIX: &str = "jjqb_";

/// The dispatch-scratch container dirname — the infield-resident home of
/// per-billet BUK state (BURV output/temp/log roots) and the session-scoped
/// MCP config. Deliberately NOT under the `jjqb_` signet: the muck sweep's
/// positive glob must never match it, and it must never shadow a billet.
pub const JJRDS_SCRATCH_DIRNAME: &str = "jjqd_scratch";

/// A billet's dirname from its identity body (bare, no glyph — dirnames are
/// machine context).
pub fn jjrds_billet_dirname(identity_body: &str) -> String {
    format!("{}{}", JJRDS_BILLET_DIR_PREFIX, identity_body)
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

/// Everything the spine resolved ahead of boarding: where the billet sits, what
/// seats it, and how the session launches. Planning is pure resolution;
/// `jjrds_board` performs the ensure/glean/probe and `jjrds_stirrup_command`
/// composes the launch.
#[derive(Debug)]
pub struct jjrds_LaunchPlan {
    pub door: jjrds_Door,
    /// The billet's line: `Branch(coronet)` for a pace billet, `Detached` for a
    /// groom billet (at trunk's counterpart).
    pub birth: jjrfr_BilletBirth,
    pub billet_dirname: String,
    pub billet_root: PathBuf,
    pub hippodrome_root: PathBuf,
    pub infield_root: PathBuf,
    pub scratch_root: PathBuf,
    pub trunk: String,
    pub tier: jjrg_Tier,
    pub effort: Option<jjrg_Effort>,
    pub opening_prompt: String,
}

/// Plan a dispatch: the spine's resolution half — identify at the captured
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
    let (birth, identity_body, designation, opening_prompt) = match door {
        jjrds_Door::Saddle => {
            let gallops = match &pin {
                Some(pin) => {
                    let bytes = jjdb_read_pinned(&studbook, pin, JJDB_GALLOPS_REL_PATH)
                        .map_err(|detail| jjrds_Rejection::StudbookUnreadable {
                            path: studbook.local_root.clone(),
                            detail,
                        })?;
                    crate::jjri_io::jjdr_hark(&bytes).map_err(|e| jjrds_Rejection::BadTarget { detail: e })?
                }
                None => {
                    let gallops_path = hippodrome_root.join(".claude/jjm/jjg_gallops.json");
                    crate::jjri_io::jjdr_load(&gallops_path).map_err(|e| jjrds_Rejection::BadTarget { detail: e })?
                }
            };
            let saddled = jjrds_resolve_saddle(gallops.inner(), &target)?;
            let prompt = format!(
                "mount {}{}",
                crate::jjrf_favor::JJRF_CORONET_PREFIX,
                saddled.coronet
            );
            (
                jjrfr_BilletBirth::Branch(saddled.coronet.clone()),
                saddled.coronet.clone(),
                saddled.designation,
                prompt,
            )
        }
        jjrds_Door::Lunge => {
            let firemark = match &target {
                jjrds_Target::Firemark(fm) => fm.clone(),
                jjrds_Target::Coronet(c) => {
                    return Err(jjrds_Rejection::BadTarget {
                        detail: format!("lunge takes a firemark; '{}' is a coronet — groom the heat, not a pace", c),
                    })
                }
            };
            let prompt = format!("groom {}{}", crate::jjrf_favor::JJRF_FIREMARK_PREFIX, firemark);
            (jjrfr_BilletBirth::Detached, firemark, None, prompt)
        }
    };

    let (tier, effort) = jjrds_resolve_launch(designation);
    let billet_dirname = jjrds_billet_dirname(&identity_body);
    let billet_root = infield_root.join(&billet_dirname);
    let scratch_root = infield_root.join(JJRDS_SCRATCH_DIRNAME).join(&billet_dirname);

    Ok(jjrds_LaunchPlan {
        door,
        birth,
        billet_dirname,
        billet_root,
        hippodrome_root,
        infield_root,
        scratch_root,
        trunk: pedigree.trunk,
        tier,
        effort,
        opening_prompt,
    })
}

/// Board the billet: the spine's mutation half — billet ensure
/// (create-or-reuse; a groom billet in reuse re-detaches to trunk tip), then
/// glean (the spine fetches and never merges), then the staleness probe whose
/// answer the launch surfaces. Returns the staleness notice, if any.
pub fn jjrds_board<F: jjrfr_FarrierCore + jjrfr_FarrierBillet>(
    farrier: &F,
    plan: &jjrds_LaunchPlan,
) -> Result<Option<String>, jjrds_Rejection> {
    if plan.billet_root.exists() {
        match &plan.birth {
            jjrfr_BilletBirth::Branch(coronet) => {
                // A standing pace billet must already seat its own branch;
                // anything else in that slot is an anomaly to surface, not ride.
                let seated = farrier
                    .jjrfr_identify(&plan.billet_root)
                    .map_err(jjrds_Rejection::ForeignGround)?;
                if seated.line_of_work != jjrfr_LineOfWork::Branch(coronet.clone()) {
                    return Err(jjrds_Rejection::BadTarget {
                        detail: format!(
                            "billet {} stands but does not seat branch '{}' — resolve by hand before dispatching",
                            plan.billet_root.display(),
                            coronet
                        ),
                    });
                }
            }
            jjrfr_BilletBirth::Detached => {
                farrier
                    .jjrfr_billet_detach(&plan.billet_root, &plan.trunk)
                    .map_err(jjrds_Rejection::Farrier)?;
            }
        }
    } else {
        match &plan.birth {
            jjrfr_BilletBirth::Branch(coronet)
                if farrier
                    .jjrfr_line_exists(&plan.hippodrome_root, coronet)
                    .map_err(jjrds_Rejection::Farrier)? =>
            {
                // The durable branch survives its reaped billet: re-seat it.
                farrier
                    .jjrfr_billet_seat(&plan.hippodrome_root, coronet, &plan.billet_root)
                    .map_err(jjrds_Rejection::Farrier)?;
            }
            birth => {
                farrier
                    .jjrfr_billet_create(&plan.hippodrome_root, birth, &plan.billet_root, &plan.trunk)
                    .map_err(jjrds_Rejection::Farrier)?;
            }
        }
    }

    // Glean: staleness becomes known here so the open can report it; refit is
    // the remedy. The probe is meaningful for a pace billet's branch; a groom
    // billet just re-detached to the freshest counterpart this station knew.
    let _ = farrier.jjrfr_glean(&plan.billet_root);
    match plan.birth {
        jjrfr_BilletBirth::Branch(_) => {
            jjrds_staleness_notice(farrier, &plan.billet_root, &plan.trunk).map_err(jjrds_Rejection::Farrier)
        }
        jjrfr_BilletBirth::Detached => Ok(None),
    }
}

// ---- Stirrup: the launch primitive ----

/// The session-scoped MCP config content, generated per dispatch (JJSVD
/// "Launch-time provisioning"): hippodromes carry no JJ inserts, so MCP
/// registration arrives from the spine, pointing at the kit repo's vvx.
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

/// Stirrup — the launch primitive at the spine's end: pace-blind,
/// parameterized (billet, tier, opening prompt); pace-coupling lives in the
/// caller. The one consumer of the tier roster: callers speak tier words,
/// never model IDs, and an invalid (family, effort) pair refuses fair-facedly.
/// Returns the composed command, cwd set inside the billet, env carrying the
/// per-billet BURV exports (the BUK meld: output, temp, and the log-dir
/// override), ready to spawn with inherited stdio.
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
    Ok(cmd)
}

// ---- The door driver (CLI entry) ----

/// The spine's terminal shape. Either the dispatch finishes here — a refusal, a
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
    Launch(std::process::Command),
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

    out.push_str(&format!(
        "billet:  {}  ({})\nlaunch:  {} / {}\nprompt:  {}\n",
        plan.billet_root.display(),
        match &plan.birth {
            jjrfr_BilletBirth::Branch(b) => format!("branch {}", b),
            jjrfr_BilletBirth::Detached => "detached at trunk tip".to_string(),
        },
        plan.tier.jjrg_as_str(),
        plan.effort.map(|e| e.jjrg_as_str()).unwrap_or("(vendor default)"),
        plan.opening_prompt,
    ));

    if dry_run {
        out.push_str("dry run: stopping before board and launch\n");
        return (jjrds_Outcome::Done(0), out);
    }

    let staleness = match jjrds_board(&farrier, &plan) {
        Ok(s) => s,
        Err(e) => return (jjrds_Outcome::Done(1), format!("{}dispatch refused at boarding: {}\n", out, e)),
    };
    if let Some(notice) = &staleness {
        out.push_str(&format!("{}\n", notice));
    }

    // Provision: the session-scoped MCP config and the per-billet BUK scratch.
    for sub in ["output-buk", "temp-buk", "logs-buk"] {
        if let Err(e) = std::fs::create_dir_all(plan.scratch_root.join(sub)) {
            return (jjrds_Outcome::Done(1), format!("{}dispatch failed provisioning scratch at {}: {}\n", out, plan.scratch_root.display(), e));
        }
    }
    let mcp_path = plan.scratch_root.join("mcp.json");
    if let Err(e) = std::fs::write(&mcp_path, jjrds_mcp_config_json(kit_root)) {
        return (jjrds_Outcome::Done(1), format!("{}dispatch failed writing MCP config at {}: {}\n", out, mcp_path.display(), e));
    }

    let cmd = match jjrds_stirrup_command(
        &plan.billet_root,
        plan.tier,
        plan.effort,
        &plan.opening_prompt,
        &mcp_path,
        &plan.scratch_root,
    ) {
        Ok(c) => c,
        Err(e) => return (jjrds_Outcome::Done(1), format!("{}dispatch refused at stirrup: {}\n", out, e)),
    };

    (jjrds_Outcome::Launch(cmd), out)
}
