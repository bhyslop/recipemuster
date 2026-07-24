// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validate command — normalize-and-report pass over the Gallops store.
//!
//! `jjx_validate` is a deliberate normalize-and-report pass, not a read-only check. It replaces the
//! former behavior, which fataled on any valid-but-non-canonical store via the load round-trip
//! gate. The verdict rides the exit code (JJSCVL), so a caller branches without scraping stdout:
//!
//!   0 / clean       — valid and already canonical; no write.
//!   2 / normalized  — valid but non-canonical; rewritten to canonical form and committed.
//!   1 / broken      — could not parse/validate (structural or invariant failure); file untouched.
//!
//! Normalization is the same canonicalization the reprieve mechanism drives in migration mode
//! (the shared jjdz_write_forward transform), plus the serializer's key-order/whitespace
//! normalization. It never invents or repairs missing/contradictory data — structural breakage is
//! exit 1, never a silent fix.
//!
//! Store of record (const-gated `_over` idiom, `jjrrt_run_retire` the template): seam-on (the
//! compiled default post-cutover) the store of record is the studbook, so `jjrvl_run_validate_over`
//! appraises the studbook tip's bytes (via jjdb_read_pinned) and journals any normalization back
//! through the standard studbook writer. Seam-off (`jjrvl_run_validate_raw`) it reads the in-repo
//! bytes directly and self-commits there — the pre-cutover behavior the two byte-fixture tests
//! drive. Either way the read is raw bytes, never jjdr_load: its round-trip gate is exactly the
//! fatal-on-non-canonical behavior this pass replaces.
//!
//! The pure verdict (zjjrvl_appraise) is split from the effectful commit so the canonicalizer is
//! unit-tested against in-memory byte fixtures, never a live gallops.

use std::path::PathBuf;
use vvc::{vvco_out, vvco_err, vvco_Output};
use crate::jjrt_types::jjrg_Gallops;
use crate::jjrv_validate::{jjrg_validate, jjrg_reconcile};
use crate::jjri_io::{jjdz_probe, jjdz_Status, jjdz_write_forward, jjri_consign};

const JJRVL_CMD_NAME_VALIDATE: &str = "jjx_validate";

/// Exit codes — the enumerated verdict (JJSCVL "Exit Status"). A caller branches on these.
const JJRVL_EXIT_CLEAN: i32 = 0;
const JJRVL_EXIT_BROKEN: i32 = 1;
const JJRVL_EXIT_NORMALIZED: i32 = 2;

/// Arguments for jjx_validate command
#[derive(clap::Args, Debug)]
pub struct jjrvl_ValidateArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,
    /// Byte budget for the normalize commit (exit-2 path). Over budget hard-fails exit 1, store
    /// reverted — the same byte-sanity guard the other self-committing verbs carry.
    #[arg(long, default_value_t = vvc::VVCG_SIZE_LIMIT)]
    pub size_limit: u64,
}

/// Verdict of appraising the on-disk gallops bytes — the pure outcome, no I/O beyond the bytes in.
pub(crate) enum zjjrvl_Appraisal {
    /// Valid and already canonical — exit 0, no write. Carries the reprieve census.
    Canonical(String),
    /// Valid but non-canonical — exit 2 once the canonical form is saved and committed. Carries the
    /// canonical struct to persist and the census. Boxed: the struct dwarfs the other variants.
    Normalize(Box<jjrg_Gallops>, String),
    /// Structural or invariant failure — exit 1, file untouched. Carries the operator message.
    Broken(String),
}

/// Appraise raw gallops bytes against the current schema — the pure canonicalizer verdict.
///
/// Parse, run the shared reprieve probe (rivet JJr_a7c), apply the migration write-forward when
/// any episode is live, semantic-validate, then compare the canonical reserialization against the
/// bytes as found. No disk, no git, no lock — so it is exhaustively unit-testable on fixtures and
/// never touches a live store.
pub(crate) fn zjjrvl_appraise(original_bytes: &[u8]) -> zjjrvl_Appraisal {
    let mut gallops: jjrg_Gallops = match serde_json::from_slice(original_bytes) {
        Ok(g) => g,
        Err(e) => return zjjrvl_Appraisal::Broken(format!("parse failed: {}", e)),
    };

    // Reprieve probe — the shared single source of old-format detection, consumed here and by
    // the jjx_open nag. It feeds the census and gates the write-forward.
    let reprieve = jjdz_probe(&gallops, original_bytes);
    if reprieve.iter().any(|s| s.live) {
        // Migration mode: run the same canonicalizer jjdr_load drives, so a legacy on-disk shape
        // normalizes to the current schema rather than tripping the comparison below.
        jjdz_write_forward(&mut gallops);
    }

    // Reconcile the top-level heat_order/heats twin — the same standing repair jjdr_load applies
    // on read (jjrg_reconcile). A merge-diverged heat_order normalizes here (exit 2) rather than
    // tripping the byte comparison below as an opaque diff; the per-axis report names what was
    // fixed, so the normalize stdout is self-describing rather than a bare byte offset.
    let repairs = jjrg_reconcile(&mut gallops);

    // Semantic validation. An invariant failure is broken — never a silent fix.
    if let Err(errors) = jjrg_validate(&gallops) {
        return zjjrvl_Appraisal::Broken(format!(
            "validation failed with {} error(s): {}",
            errors.len(),
            errors.join("; ")
        ));
    }

    let canonical = match serde_json::to_string_pretty(&gallops) {
        Ok(s) => s,
        Err(e) => return zjjrvl_Appraisal::Broken(format!("reserialize failed: {}", e)),
    };
    let census = zjjrvl_census(&reprieve, &repairs);

    if canonical.as_bytes() == original_bytes {
        zjjrvl_Appraisal::Canonical(census)
    } else {
        zjjrvl_Appraisal::Normalize(Box::new(gallops), census)
    }
}

/// Run the validate command — normalize-and-report over the Gallops store of
/// record. Const-gated `_over` idiom (`jjrrt_run_retire` is the template): off,
/// it appraises the in-repo `args.file` and self-commits there; on (the compiled
/// default post-cutover), the store of record is the studbook, so it appraises
/// the studbook tip and journals any normalization back through the standard
/// studbook writer. The banked flip-time question — whether validate names the
/// studbook or notices-and-no-ops against the fossil consumer gallops
/// (`zjjrm_write_gallops`'s doc) — is answered here: name the studbook, so
/// `jjx_validate` checks the store of record rather than a tombstone.
pub fn jjrvl_run_validate(args: jjrvl_ValidateArgs, officium: &str) -> (i32, String) {
    if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        return jjrvl_run_validate_raw(args);
    }
    let cn = JJRVL_CMD_NAME_VALIDATE;
    let (studbook, guidon) = match crate::jjrm_mcp::zjjrm_studbook_and_guidon(officium, cn) {
        Ok(sg) => sg,
        Err(e) => {
            let mut output = vvco_Output::buffer();
            vvco_err!(output, "{}: broken — {}", cn, e);
            return (JJRVL_EXIT_BROKEN, output.vvco_finish());
        }
    };
    let mut output = vvco_Output::buffer();
    let code = jjrvl_run_validate_over(
        &crate::jjrfg_plaingit::jjrfg_PlainGit,
        &studbook,
        &guidon,
        args.size_limit,
        &mut output,
        cn,
    );
    (code, output.vvco_finish())
}

/// The seam-ON validate path, extracted from the const gate so a test drives it
/// against a fixture studbook while `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED` stays
/// false (the `_over` idiom: `jjrrt_retire_over`). Read the studbook tip bytes
/// for the byte-canonical appraisal (the same `jjdb_read_pinned` the studbook
/// read seam is built on), reuse the unchanged pure appraiser, and — on a
/// non-canonical tip — re-canonicalize the LOCKED, advanced tip inside the
/// standard studbook journal ceremony (`zjjrm_journal_run`), so the normalize
/// targets whatever the locked tip actually holds rather than a stale pre-lock
/// read (the same "derive against the locked tip" discipline retire keeps). The
/// studbook has no commit-machinery size guard, so the byte ceiling is an
/// explicit in-mutate check (retire's precedent). Normalize is a near-dead
/// safety net in practice — every studbook write lands canonical through
/// `jjdr_save` and `jjdr_hark` write-forwards on read — reachable only by a live
/// reprieve migration, a serializer drift across binary versions, or a
/// hand-edited tip, which is precisely validate's remaining purpose. `studbook`/
/// `guidon` arrive resolved so the fixture aims the ceremony at its scratch store.
#[allow(clippy::too_many_arguments)]
pub(crate) fn jjrvl_run_validate_over<F>(
    farrier: &F,
    studbook: &crate::jjrvb_blotter::jjdb_BlotterConfig,
    guidon: &str,
    size_limit: u64,
    output: &mut vvco_Output,
    cn: &str,
) -> i32
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore + crate::jjrfr_farrier::jjrfr_FarrierLock,
{
    let pin = match crate::jjrvb_blotter::jjdb_pin(studbook) {
        Ok(p) => p,
        Err(e) => {
            vvco_err!(output, "{}: broken — cannot pin studbook: {}", cn, e);
            return JJRVL_EXIT_BROKEN;
        }
    };
    let original_bytes = match crate::jjrvb_blotter::jjdb_read_pinned(
        studbook, &pin, crate::jjrvb_blotter::JJDB_GALLOPS_REL_PATH,
    ) {
        Ok(b) => b,
        Err(e) => {
            vvco_err!(output, "{}: broken — cannot read studbook gallops: {}", cn, e);
            return JJRVL_EXIT_BROKEN;
        }
    };

    match zjjrvl_appraise(&original_bytes) {
        zjjrvl_Appraisal::Canonical(census) => {
            vvco_out!(output, "{}: clean — valid and already canonical. {}", cn, census);
            JJRVL_EXIT_CLEAN
        }
        zjjrvl_Appraisal::Broken(msg) => {
            vvco_err!(output, "{}: broken — {}", cn, msg);
            JJRVL_EXIT_BROKEN
        }
        zjjrvl_Appraisal::Normalize(_gallops, census) => {
            let brand = vvc::vvcc_get_brand();
            let subject = format!("VALIDATE normalized gallops — {}", census);
            let action = crate::jjrnm_markers::JJRNM_VALIDATE.to_string();
            let message = vvc::vvcc_format_branded(
                crate::jjrn_notch::JJRN_COMMIT_PREFIX,
                &brand,
                "", // gallops-wide: no heat/pace identity
                &action,
                &subject,
                None,
            );
            let result = crate::jjrm_mcp::zjjrm_journal_run(farrier, studbook, guidon, |g| {
                // Re-canonicalize the locked tip: `g` was already write-forwarded by
                // jjdr_hark on the ceremony's tip read, so reconcile (idempotent) then
                // let the journal's jjdr_save re-serialize canonically. The explicit
                // byte-ceiling check stands in for the commit-machinery size guard the
                // studbook journal path lacks.
                let _ = jjrg_reconcile(g);
                let canonical = serde_json::to_string_pretty(g)
                    .map_err(|e| format!("reserialize failed: {}", e))?;
                if canonical.len() as u64 > size_limit {
                    return Err(format!(
                        "normalized gallops is {} bytes, over the {}-byte ceiling — retry with a raised size_limit if the bulk is legitimate",
                        canonical.len(), size_limit
                    ));
                }
                Ok(((), message))
            });
            match result {
                Ok(((), sha)) => {
                    let short = &sha[..sha.len().min(9)];
                    vvco_out!(output, "{}: normalized — journaled canonical gallops to the studbook {}. {}", cn, short, census);
                    JJRVL_EXIT_NORMALIZED
                }
                Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
                    vvco_err!(output, "{}: broken — {}", cn, e);
                    JJRVL_EXIT_BROKEN
                }
                Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
                    vvco_err!(output, "{}: broken — {}", cn, crate::jjri_io::jjri_commit_refusal(cn, &e));
                    JJRVL_EXIT_BROKEN
                }
                Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
                    vvco_err!(output, "{}: broken — studbook journal refused: {}", cn, r);
                    JJRVL_EXIT_BROKEN
                }
            }
        }
    }
}

/// The seam-OFF validate path — the pre-cutover behavior, verbatim: appraise the
/// in-repo `args.file` bytes and, on a non-canonical store, self-commit the
/// canonical form there under the byte budget. Reached only when the studbook
/// seam is off (the compiled default is on); the two byte-fixture tests
/// (`jjtg_validate_run_*`) drive it directly against a raw on-disk file.
pub(crate) fn jjrvl_run_validate_raw(args: jjrvl_ValidateArgs) -> (i32, String) {
    let cn = JJRVL_CMD_NAME_VALIDATE;
    let mut output = vvco_Output::buffer();

    // Read the on-disk bytes directly — the canonical comparison and the reprieve probe both
    // need the stored form, and we deliberately do not go through jjdr_load: its round-trip gate is
    // exactly the fatal-on-non-canonical behavior this pass replaces.
    let original_bytes = match std::fs::read(&args.file) {
        Ok(b) => b,
        Err(e) => {
            vvco_err!(output, "{}: broken — cannot read '{}': {}", cn, args.file.display(), e);
            return (JJRVL_EXIT_BROKEN, output.vvco_finish());
        }
    };

    match zjjrvl_appraise(&original_bytes) {
        zjjrvl_Appraisal::Canonical(census) => {
            vvco_out!(output, "{}: clean — valid and already canonical. {}", cn, census);
            (JJRVL_EXIT_CLEAN, output.vvco_finish())
        }
        zjjrvl_Appraisal::Broken(msg) => {
            vvco_err!(output, "{}: broken — {}", cn, msg);
            (JJRVL_EXIT_BROKEN, output.vvco_finish())
        }
        zjjrvl_Appraisal::Normalize(gallops, census) => {
            zjjrvl_commit_normalization(cn, &args, &gallops, &census)
        }
    }
}

/// Save the canonical gallops and commit it — the effectful exit-2 tail.
///
/// Acquires the commit lock, then consigns the gallops alone (gallops-wide, no heat/pace identity,
/// under the byte budget). The commit finalizes any in-progress merge (two parents); a blocked
/// commit reverts the store to HEAD so the file is left untouched and the verdict falls to broken.
fn zjjrvl_commit_normalization(
    cn: &str,
    args: &jjrvl_ValidateArgs,
    gallops: &jjrg_Gallops,
    census: &str,
) -> (i32, String) {
    let mut output = vvco_Output::buffer();
    let lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(
                output,
                "{}: broken — could not normalize, commit lock held: {} (break with `vvx vvx_unlock` in extremis)",
                cn, e
            );
            return (JJRVL_EXIT_BROKEN, output.vvco_finish());
        }
    };

    let brand = vvc::vvcc_get_brand();
    let subject = format!("VALIDATE normalized gallops — {}", census);
    let action = crate::jjrnm_markers::JJRNM_VALIDATE.to_string();
    let message = vvc::vvcc_format_branded(
        crate::jjrn_notch::JJRN_COMMIT_PREFIX,
        &brand,
        "", // gallops-wide: no heat/pace identity
        &action,
        &subject,
        None,
    );

    let mut commit_out = vvco_Output::buffer();
    let result = jjri_consign(&lock, gallops, &args.file, message, args.size_limit, &mut commit_out);
    // lock drops at end of scope
    match result {
        Ok(Some(hash)) => {
            let short = &hash[..hash.len().min(9)];
            vvco_out!(output, "{}: normalized — rewrote to canonical and committed {}. {}", cn, short, census);
            (JJRVL_EXIT_NORMALIZED, output.vvco_finish())
        }
        Ok(None) => {
            vvco_out!(
                output,
                "{}: normalized — rewrote working tree to canonical; HEAD already canonical, no new commit. {}",
                cn, census
            );
            (JJRVL_EXIT_NORMALIZED, output.vvco_finish())
        }
        Err(e) => {
            vvco_err!(
                output,
                "{}: broken — normalization commit failed, store reverted to HEAD: {}\n{}",
                cn, e, commit_out.vvco_finish()
            );
            (JJRVL_EXIT_BROKEN, output.vvco_finish())
        }
    }
}

/// One-line reprieve census for the outcome stdout — names each registered episode's verdict
/// against the on-disk store, so a normalized result says *what* shape it migrated and a clean
/// result is positive evidence rather than bare silence (the self-describing-stdout cinch). Any
/// heat_order/heats reconcile that fired is appended per-axis, so a normalize that deduped a
/// merge-diverged heat_order names the firemarks it touched rather than a bare byte offset.
fn zjjrvl_census(reprieve: &[jjdz_Status], repairs: &[String]) -> String {
    let reprieve_part = if reprieve.is_empty() {
        "no reprieve episodes registered.".to_string()
    } else {
        let parts: Vec<String> = reprieve
            .iter()
            .map(|s| format!("{} {}", s.label, s.jjdz_verdict()))
            .collect();
        format!("reprieve: {}.", parts.join(", "))
    };
    if repairs.is_empty() {
        reprieve_part
    } else {
        format!("{} reconcile: {}.", reprieve_part, repairs.join("; "))
    }
}
