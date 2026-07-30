// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Affiliate command — an identity-grained fill-blanks verb over pace→sire
//! affiliation.
//!
//! Pace-hard affiliation (`jjrg_Pace.sire`, the sire's operator-facing handle) is
//! minted at slate going forward — the launch inversion's registry-reading
//! election. This verb fills the gaps: the pre-schema paces that carry no sire,
//! and the sire-less stragglers a wipe or a migration leaves behind.
//!
//! Three routes, all typed off the arguments:
//!
//! - **Coronet grain** `{identity: coronet, sire}` — one pace. Blank fills; a pace
//!   already on this sire is a no-op; a pace on a *different* sire refuses loud,
//!   naming the incumbent. This is the conflict guard: one explicit target, so a
//!   mismatch is a surprise worth halting on. A deliberate re-set is not yet a verb.
//! - **Firemark grain** `{identity: firemark, sire}` — one heat's paces. Only the
//!   blanks fill; every already-affiliated pace (this sire OR another) is left
//!   untouched and reported. A heat-wide fill is a blank-drain, never a flatten —
//!   so a mixed-sire heat's foreign paces survive it intact.
//! - **Audit** `{}` — no identity, read-only. Every unaffiliated pace, grouped by
//!   heat in canonical order. The census route the interface otherwise lacks
//!   (nothing else surfaces sire) and the drain-verification check.
//!
//! The write routes journal through the standard single-heat funnel
//! (`zjjrm_write_gallops`) under an `:a:` affiliate marker, so the commit carries
//! the affected heat's identity and the journal stays self-describing.

use std::path::{Path, PathBuf};
use vvc::{vvco_err, vvco_out, vvco_Output};
use crate::jjrf_favor::{
    jjrf_bare, jjrf_Coronet, jjrf_Firemark, JJRF_CORONET_LEN, JJRF_FIREMARK_LEN,
    JJRF_CORONET_PREFIX, JJRF_CORONET_QUALIFIER, JJRF_FIREMARK_PREFIX,
};
use crate::jjrt_types::jjrg_Gallops;

const JJAF_CMD_NAME: &str = "jjx_affiliate";

/// Arguments for jjx_affiliate. Both `identity` and `sire` are optional: present
/// together they fill; absent together they audit; exactly one is a usage error.
pub struct jjaf_AffiliateArgs {
    pub file: PathBuf,
    /// Coronet (one pace) or Firemark (one heat's paces); absent for the audit.
    pub identity: Option<String>,
    /// The sire handle to stamp onto the targeted blanks; absent for the audit.
    pub sire: Option<String>,
    pub size_limit: u64,
}

// ---- Pure transforms (no I/O) ------------------------------------------------

/// The disposition of a single-pace (coronet-grain) fill.
pub enum jjaf_PaceFill {
    /// Was unaffiliated; now stamped with the sire.
    Filled,
    /// Already carried this exact sire — nothing changed.
    AlreadySame,
    /// Already carried a different sire (the incumbent handle); left untouched.
    Conflict(String),
    /// No pace carries this Coronet.
    Absent,
}

/// The blank-drain report for one heat (firemark grain).
pub struct jjaf_HeatFill {
    /// Display coronets newly stamped with the sire.
    pub filled: Vec<String>,
    /// Display coronets already on this sire — left as-is.
    pub already: Vec<String>,
    /// (Display coronet, incumbent sire) for paces on a *different* sire — left
    /// untouched, never overwritten.
    pub skipped: Vec<(String, String)>,
}

/// One heat's slice of the blank audit.
pub struct jjaf_AuditGroup {
    /// Firemark in display form (`₣XX`).
    pub firemark: String,
    pub silks: String,
    /// Unaffiliated paces, display-qualified (`₢XX·CAAAG`), in heat order.
    pub coronets: Vec<String>,
}

/// Fill one pace's sire iff it is blank — the authoritative coronet-grain
/// transform. Inspects the pace's current affiliation and mutates only on the
/// blank path, so `AlreadySame`, `Conflict`, and `Absent` all leave the gallops
/// untouched. Accepts stored, bare, or heat-qualified input — normalized to the
/// stored `₢`+body key first (as `jjrg_qualify_coronet` does).
pub fn jjrg_fill_pace(gallops: &mut jjrg_Gallops, coronet: &str, sire: &str) -> jjaf_PaceFill {
    let key = format!("{}{}", JJRF_CORONET_PREFIX, jjrf_bare(coronet));
    let heat_key = match gallops.jjrg_heat_key_of_coronet(&key) {
        Some(k) => k,
        None => return jjaf_PaceFill::Absent,
    };
    let pace = match gallops.heats.get_mut(&heat_key).and_then(|h| h.paces.get_mut(&key)) {
        Some(p) => p,
        None => return jjaf_PaceFill::Absent,
    };
    match pace.sire.as_deref() {
        None => {
            pace.sire = Some(sire.to_string());
            jjaf_PaceFill::Filled
        }
        Some(s) if s == sire => jjaf_PaceFill::AlreadySame,
        Some(s) => jjaf_PaceFill::Conflict(s.to_string()),
    }
}

/// Drain every blank pace in one heat, stamping `sire`. Already-affiliated paces —
/// this sire or another — are left untouched and reported, so a heat-wide fill
/// never overwrites a foreign affiliation. Accepts a bare or sigiled firemark —
/// normalized to the stored `₣`+body key first; `None` when no heat carries it.
pub fn jjrg_fill_heat(gallops: &mut jjrg_Gallops, firemark: &str, sire: &str) -> Option<jjaf_HeatFill> {
    let fm_bare = jjrf_bare(firemark).to_string();
    let fm_key = format!("{}{}", JJRF_FIREMARK_PREFIX, fm_bare);
    let heat = gallops.heats.get_mut(&fm_key)?;
    let order = heat.order.clone();
    let mut report = jjaf_HeatFill { filled: Vec::new(), already: Vec::new(), skipped: Vec::new() };
    for pace_key in &order {
        if let Some(pace) = heat.paces.get_mut(pace_key) {
            let disp = zjjaf_qualify(&fm_bare, pace_key);
            match pace.sire.as_deref() {
                None => {
                    pace.sire = Some(sire.to_string());
                    report.filled.push(disp);
                }
                Some(s) if s == sire => report.already.push(disp),
                Some(s) => report.skipped.push((disp, s.to_string())),
            }
        }
    }
    Some(report)
}

/// Every unaffiliated pace, grouped by heat in canonical order — heats in
/// `heat_order` (falling back to the map's own order when unset), paces in each
/// heat's `order`. Heats with no blanks are omitted.
pub fn jjrg_audit_blanks(gallops: &jjrg_Gallops) -> Vec<jjaf_AuditGroup> {
    let heat_keys: Vec<String> = if gallops.heat_order.is_empty() {
        gallops.heats.keys().cloned().collect()
    } else {
        gallops.heat_order.clone()
    };
    let mut groups = Vec::new();
    for fm_key in &heat_keys {
        if let Some(heat) = gallops.heats.get(fm_key) {
            let fm_bare = jjrf_bare(fm_key);
            let coronets: Vec<String> = heat.order.iter()
                .filter(|pk| heat.paces.get(*pk).is_some_and(|p| p.sire.is_none()))
                .map(|pk| zjjaf_qualify(fm_bare, pk))
                .collect();
            if !coronets.is_empty() {
                groups.push(jjaf_AuditGroup {
                    firemark: format!("{}{}", JJRF_FIREMARK_PREFIX, fm_bare),
                    silks: heat.silks.clone(),
                    coronets,
                });
            }
        }
    }
    groups
}

/// Render a pace's heat-qualified display coronet (`₢XX·CAAAG`) from a bare heat
/// body and a stored pace key — the one place the fill/audit outputs compose it,
/// so an affiliation display never fabricates a heat it cannot prove.
fn zjjaf_qualify(fm_bare: &str, pace_key: &str) -> String {
    format!("{}{}{}{}", JJRF_CORONET_PREFIX, fm_bare, JJRF_CORONET_QUALIFIER, jjrf_bare(pace_key))
}

/// Compose an affiliate commit message under the `:a:` marker, carrying the
/// affected heat's identity so the journal is self-describing.
fn zjjaf_message(firemark: &jjrf_Firemark, subject: &str) -> String {
    let brand = vvc::vvcc_get_brand();
    let identity = format!("{}{}", JJRF_FIREMARK_PREFIX, firemark.jjrf_as_str());
    vvc::vvcc_format_branded(
        crate::jjrn_notch::JJRN_COMMIT_PREFIX,
        &brand,
        &identity,
        &crate::jjrnm_markers::JJRNM_AFFILIATE.to_string(),
        subject,
        None,
    )
}

// ---- Command run (I/O) -------------------------------------------------------

/// Run the affiliate command — audit, or a coronet/firemark fill, routed off the
/// arguments. Present identity + sire fill; absent identity + sire audit; exactly
/// one is a usage error naming the two modes.
pub fn jjaf_run_affiliate(args: jjaf_AffiliateArgs, officium: &str) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    let identity = args.identity.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let sire = args.sire.as_deref().map(str::trim).filter(|s| !s.is_empty());
    match (identity, sire) {
        (None, None) => zjjaf_run_audit(&args.file),
        (Some(id), Some(sire)) => zjjaf_run_fill(&args.file, id, sire, args.size_limit, officium),
        (Some(_), None) => (
            1,
            format!("{}: error: an identity needs a sire to fill — pass {{identity, sire}}, or neither for the read-only audit\n", cn),
        ),
        (None, Some(_)) => (
            1,
            format!("{}: error: a sire needs an identity to target — pass a Coronet or Firemark with it, or neither for the read-only audit\n", cn),
        ),
    }
}

/// The read-only audit route — every unaffiliated pace, grouped by heat.
fn zjjaf_run_audit(file: &Path) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    let mut output = vvco_Output::buffer();
    let gallops = match crate::jjrm_mcp::zjjrm_load_gallops(file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let groups = jjrg_audit_blanks(&gallops);
    if groups.is_empty() {
        vvco_out!(output, "{}: no unaffiliated paces — every pace carries a sire", cn);
        return (0, output.vvco_finish());
    }
    let total: usize = groups.iter().map(|g| g.coronets.len()).sum();
    vvco_out!(output, "{}: {} unaffiliated pace(s) across {} heat(s):", cn, total, groups.len());
    for g in &groups {
        vvco_out!(output, "  {} ({}) — {} blank", g.firemark, g.silks, g.coronets.len());
        for c in &g.coronets {
            vvco_out!(output, "      {}", c);
        }
    }
    (0, output.vvco_finish())
}

/// The fill route — dispatch by identity length to the coronet or firemark grain.
fn zjjaf_run_fill(file: &Path, identity: &str, sire: &str, size_limit: u64, officium: &str) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    let bare_len = jjrf_bare(identity).len();
    if bare_len == JJRF_CORONET_LEN {
        zjjaf_fill_pace(file, identity, sire, size_limit, officium)
    } else if bare_len == JJRF_FIREMARK_LEN {
        zjjaf_fill_heat(file, identity, sire, size_limit, officium)
    } else {
        (
            1,
            format!("{}: error: identity must be a Coronet (5 chars) or Firemark (2 chars), got {} chars\n", cn, bare_len),
        )
    }
}

/// Coronet grain — fill one pace, refusing loud on a differing incumbent sire.
fn zjjaf_fill_pace(file: &Path, identity: &str, sire: &str, size_limit: u64, officium: &str) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    let mut output = vvco_Output::buffer();

    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    crate::jjrsj_sectional::jjrsj_phase(cn, "lock");

    let gallops = match crate::jjrm_mcp::zjjrm_load_gallops(file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    crate::jjrsj_sectional::jjrsj_phase(cn, "load");

    let coronet = match jjrf_Coronet::jjrf_parse(identity) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let disp = coronet.jjrf_display();
    let heat_key = match gallops.jjrg_heat_key_of_coronet(&disp) {
        Some(k) => k,
        None => {
            vvco_err!(output, "{}: error: Pace '{}' not found", cn, disp);
            return (1, output.vvco_finish());
        }
    };
    let fm = match jjrf_Firemark::jjrf_parse(&heat_key) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Read the current affiliation before writing — so a no-op and a conflict
    // both resolve without acquiring a commit for an unchanged gallops.
    let current = gallops.heats.get(&heat_key)
        .and_then(|h| h.paces.get(&disp))
        .and_then(|p| p.sire.clone());
    match current {
        Some(s) if s == sire => {
            vvco_out!(output, "{}: {} already affiliated to '{}' — nothing to fill", cn, disp, s);
            return (0, output.vvco_finish());
        }
        Some(s) => {
            vvco_err!(
                output,
                "{}: error: {} is affiliated to '{}', not '{}' — refusing to overwrite (fill-blanks only; a deliberate re-set is not yet a verb)",
                cn, disp, s, sire
            );
            return (1, output.vvco_finish());
        }
        None => {}
    }

    let subject = format!("affiliate {} → {}", disp, sire);
    let message = zjjaf_message(&fm, &subject);
    let disp_m = disp.clone();
    let sire_m = sire.to_string();
    match crate::jjrm_mcp::zjjrm_write_gallops(
        &lock,
        file,
        &fm,
        message,
        size_limit,
        &mut output,
        officium,
        cn,
        Vec::new(),
        gallops,
        |g| match jjrg_fill_pace(g, &disp_m, &sire_m) {
            jjaf_PaceFill::Filled | jjaf_PaceFill::AlreadySame => Ok(()),
            jjaf_PaceFill::Conflict(s) => Err(format!(
                "{} changed sire to '{}' under the write — refusing to overwrite", disp_m, s
            )),
            jjaf_PaceFill::Absent => Err(format!("Pace '{}' vanished under the write", disp_m)),
        },
    ) {
        Ok(((), hash)) => {
            vvco_out!(output, "{}: affiliated {} → '{}'; committed {}", cn, disp, sire, &hash[..hash.len().min(9)]);
            (0, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            (1, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            (1, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
            vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}

/// Firemark grain — drain a heat's blanks, leaving every affiliated pace intact.
fn zjjaf_fill_heat(file: &Path, identity: &str, sire: &str, size_limit: u64, officium: &str) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    let mut output = vvco_Output::buffer();

    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    crate::jjrsj_sectional::jjrsj_phase(cn, "lock");

    let gallops = match crate::jjrm_mcp::zjjrm_load_gallops(file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    crate::jjrsj_sectional::jjrsj_phase(cn, "load");

    let fm = match jjrf_Firemark::jjrf_parse(identity) {
        Ok(fm) => fm,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let fm_disp = fm.jjrf_display();

    // Count the blanks before writing — no blanks means nothing to commit.
    let (blanks, total) = match gallops.heats.get(&fm_disp) {
        Some(h) => (h.paces.values().filter(|p| p.sire.is_none()).count(), h.paces.len()),
        None => {
            vvco_err!(output, "{}: error: Heat '{}' not found", cn, fm_disp);
            return (1, output.vvco_finish());
        }
    };
    if blanks == 0 {
        vvco_out!(output, "{}: {} has no unaffiliated paces ({} already affiliated) — nothing to fill", cn, fm_disp, total);
        return (0, output.vvco_finish());
    }

    let subject = format!("affiliate {} → {}: {} filled", fm_disp, sire, blanks);
    let message = zjjaf_message(&fm, &subject);
    let fm_disp_m = fm_disp.clone();
    let sire_m = sire.to_string();
    match crate::jjrm_mcp::zjjrm_write_gallops(
        &lock,
        file,
        &fm,
        message,
        size_limit,
        &mut output,
        officium,
        cn,
        Vec::new(),
        gallops,
        |g| jjrg_fill_heat(g, &fm_disp_m, &sire_m)
            .ok_or_else(|| format!("Heat '{}' vanished under the write", fm_disp_m)),
    ) {
        Ok((report, hash)) => {
            vvco_out!(
                output,
                "{}: {} → '{}'; {} filled, {} already, {} skipped; committed {}",
                cn, fm_disp, sire, report.filled.len(), report.already.len(), report.skipped.len(),
                &hash[..hash.len().min(9)]
            );
            for (c, s) in &report.skipped {
                vvco_out!(output, "    skipped {} (affiliated to '{}')", c, s);
            }
            (0, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            (1, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            (1, output.vvco_finish())
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
            vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}
