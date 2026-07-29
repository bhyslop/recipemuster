// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Affiliate command — stamp pace→sire affiliation across the gallops.
//!
//! The backfill mechanism the sire registry needs. Pace-hard affiliation
//! (`jjrg_Pace.sire`, the sire's operator-facing handle) is minted at slate going
//! forward — the launch inversion's registry-reading election — but pre-schema
//! paces carry none, so this verb stamps them once, and it stands as the
//! deliberate re-affiliation primitive thereafter. One gallops-wide write,
//! journaled through the studbook, mirroring validate's seam-aware structure (the
//! store of record is the studbook post-cutover).
//!
//! The resolved/unresolved backfill rule collapses to a default-plus-overrides
//! shape: resolved paces and unresolved paces landing in the founding sire all
//! take the default handle, and only the unresolved paces landing in a second
//! sire ride the override map — so the operator-confirmed affiliation census IS
//! the override set.

use std::collections::BTreeMap;
use std::path::PathBuf;
use vvc::{vvco_err, vvco_out, vvco_Output};
use crate::jjrt_types::jjrg_Gallops;

const JJAF_CMD_NAME: &str = "jjx_affiliate";

/// Arguments for jjx_affiliate.
pub struct jjaf_AffiliateArgs {
    pub file: PathBuf,
    /// The sire handle every pace not named in `overrides` receives.
    pub default: String,
    /// Per-coronet sire overrides (bare or display coronet → sire handle).
    pub overrides: BTreeMap<String, String>,
    pub size_limit: u64,
}

/// Stamp every pace's sire affiliation — the pure transform, no I/O. A pace whose
/// bare coronet body is named in `overrides` takes that sire; every other pace
/// takes `default`. Returns the count stamped. Override keys are normalized to
/// their bare body once, so a display-form (`₢B_·CAAAG`) or sigiled key still
/// matches the stored coronet.
pub fn jjrg_backfill_affiliation(
    gallops: &mut jjrg_Gallops,
    default: &str,
    overrides: &BTreeMap<String, String>,
) -> usize {
    use crate::jjrf_favor::jjrf_bare;
    let bare_overrides: BTreeMap<String, &String> =
        overrides.iter().map(|(k, v)| (jjrf_bare(k).to_string(), v)).collect();
    let mut n = 0;
    for heat in gallops.heats.values_mut() {
        for (coronet_key, pace) in heat.paces.iter_mut() {
            let bare = jjrf_bare(coronet_key);
            let sire = bare_overrides.get(bare).map(|s| s.as_str()).unwrap_or(default);
            pace.sire = Some(sire.to_string());
            n += 1;
        }
    }
    n
}

/// Compose the gallops-wide affiliate commit message — no heat/pace identity (the
/// stamp spans the whole gallops), carrying the affiliate marker so the journal is
/// self-describing.
fn zjjaf_message(n_paces: usize, n_overrides: usize) -> String {
    let brand = vvc::vvcc_get_brand();
    let subject = format!(
        "AFFILIATE backfill — {} pace(s) sire-stamped ({} override(s))",
        n_paces, n_overrides
    );
    vvc::vvcc_format_branded(
        crate::jjrn_notch::JJRN_COMMIT_PREFIX,
        &brand,
        "", // gallops-wide: no heat/pace identity
        &crate::jjrnm_markers::JJRNM_AFFILIATE.to_string(),
        &subject,
        None,
    )
}

/// Run the affiliate command — a gallops-wide sire-stamp, journaled as one commit.
/// Seam-aware (`jjrvl_run_validate`'s template): the compiled default journals to
/// the studbook store of record; the pre-cutover seam-off path self-commits the
/// in-repo store.
pub fn jjaf_run_affiliate(args: jjaf_AffiliateArgs, officium: &str) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    if args.default.trim().is_empty() {
        return (1, format!("{}: error: default sire handle must not be empty\n", cn));
    }
    if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        return jjaf_run_affiliate_raw(args);
    }
    let mut output = vvco_Output::buffer();
    let (studbook, guidon) = match crate::jjrm_mcp::zjjrm_studbook_and_guidon(officium, cn) {
        Ok(sg) => sg,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let n_overrides = args.overrides.len();
    let result = crate::jjrm_mcp::zjjrm_journal_run(
        &crate::jjrfg_plaingit::jjrfg_PlainGit,
        &studbook,
        &guidon,
        |g| {
            let n = jjrg_backfill_affiliation(g, &args.default, &args.overrides);
            let canonical = serde_json::to_string_pretty(g)
                .map_err(|e| format!("reserialize failed: {}", e))?;
            if canonical.len() as u64 > args.size_limit {
                return Err(format!(
                    "affiliated gallops is {} bytes, over the {}-byte ceiling — retry with a raised size_limit if the bulk is legitimate",
                    canonical.len(), args.size_limit
                ));
            }
            Ok((n, zjjaf_message(n, n_overrides)))
        },
    );
    match result {
        Ok((n, sha)) => {
            let short = &sha[..sha.len().min(9)];
            vvco_out!(
                output,
                "{}: affiliated {} pace(s) → default '{}' ({} override(s)); journaled {}",
                cn, n, args.default, n_overrides, short
            );
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
}

/// The seam-OFF path — pre-cutover, the in-repo store: load, stamp, self-commit
/// under the byte budget (validate's `zjjrvl_commit_normalization` template).
fn jjaf_run_affiliate_raw(args: jjaf_AffiliateArgs) -> (i32, String) {
    let cn = JJAF_CMD_NAME;
    let mut output = vvco_Output::buffer();
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let mut gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };
    let n = jjrg_backfill_affiliation(&mut gallops, &args.default, &args.overrides);
    let message = zjjaf_message(n, args.overrides.len());
    let mut commit_out = vvco_Output::buffer();
    match crate::jjri_io::jjri_consign(cn, &lock, &gallops, &args.file, message, args.size_limit, &mut commit_out) {
        Ok(Some(hash)) => {
            vvco_out!(output, "{}: affiliated {} pace(s); committed {}", cn, n, &hash[..hash.len().min(9)]);
            (0, output.vvco_finish())
        }
        Ok(None) => {
            vvco_out!(output, "{}: affiliated {} pace(s); working tree rewritten, HEAD already matched", cn, n);
            (0, output.vvco_finish())
        }
        Err(e) => {
            vvco_err!(output, "{}: error: {}\n{}", cn, e, commit_out.vvco_finish());
            (1, output.vvco_finish())
        }
    }
}
