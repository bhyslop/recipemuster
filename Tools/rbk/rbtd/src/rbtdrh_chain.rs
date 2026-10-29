// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDRH — chaining-fact-band fixture: the band matrix for the durable-config
// chain LINKS (feoff, yoke, anoint, mitre) and the read-side consumers (summon,
// plumb, augur, rekon), driven through the real tabtarget exec path.
//
// The chaining-fact discipline splits the value-forwarding verbs by role; only
// feoff/anoint/yoke/mitre write durable config from a resolved express-or-chain value,
// and a bad resolution (a wrong-kind touchmark, a broken chain) must be REJECTED
// with the named precision band — never a bare nonzero, and never after a
// destructive write. The band fires only at the RBK consumer (buc_reject in
// feoff/yoke); the BUK footing resolver returns a bare 1, so the band can only
// be asserted here, against the real verbs. Each negative case asserts the
// SPECIFIC band (RBTGEB_CHAIN_DENTIL) — like the regime-poison precedent, a
// harness breakage exits with some other code and fails the case loud, where a
// bare-nonzero assertion would pass on it.
//
// feoff resolves its vessel by PATH (zrbfcv_resolve_vessel, never the strict
// load), so every feoff case runs against a vessel staged in the case temp dir —
// no tracked rbrv.env is ever touched. The chained fact is seeded by writing it
// into the BURV root's current/, which bud_dispatch promotes to previous/ at
// dispatch start (the same path chain_next_invoke replicates for cloud pairs).
// yoke fans out across the tracked vessel tree, but its kind gate rejects BEFORE
// auth and BEFORE the write loop, so the yoke negatives are creds-free and
// write-free. anoint takes the strict load (it must, to read RBRV_VESSEL_MODE), so
// unlike feoff it cannot use a temp vessel — it drives the one real graft vessel
// (rbev_graft_demo) as its folio; its broken-chain reject precedes the rewrite, so
// that tracked rbrv.env stays write-free too. Nothing here mints a token — the
// fixture is credless.
//
// The read-side consumers (summon, plumb, augur, rekon) resolve the same
// express-or-chain fact but write no durable config. They now reject a broken resolve with the same
// chaining band, so a folio-less drive against an empty BURV root asserts the band
// before any auth — credless, no cloud. These cases prove the resolve-logic
// rejection ONLY: a furnish gap (a CLI that forgot to source buf_fact) exits the
// same band 105 — command-not-found also trips the `|| buc_reject` — so it is
// indistinguishable here and is caught instead by the static furnish case below.
//
// Every consumer relays-then-reads (RBr_3e7): buf_relay at the top of the verb
// forwards the baton before any read or failure point, so a fact survives any run
// of consecutive chain verbs and a failed consumer has already passed the baton to
// its own retry. The two positive cases at the bottom prove that law across REAL
// successive dispatches over one shared BURV root — multi-consumer reuse and
// retry-after-failure.

use std::path::{Path, PathBuf};

use crate::rbtcee_case;
use crate::rbtgeb_breviary::{
    RBTGEB_CHAIN_DENTIL,
    RBTGEB_NAMEPLATE_MITRE_BASE,
};
use crate::rbtgec_consts::{
    RBTGEC_ANOINT_GRAFT,
    RBTGEC_AUGUR_LODE,
    RBTGEC_CONTAINER_BOTTLE,
    RBTGEC_FEOFF_BOLE,
    RBTGEC_MITRE_HALLMARK,
    RBTGEC_PLUMB_FULL,
    RBTGEC_RBRV_FILE,
    RBTGEC_REKON_HALLMARK,
    RBTGEC_SUMMON_HALLMARK,
    RBTGEC_THEURGE_NIHIL,
    RBTGEC_YOKE_RELIQUARY,
};
use crate::rbtcee_engine::{
    rbtcee_Tariff,
    rbtcee_Case,
    rbtcee_Disposition,
    rbtcee_Fixture,
    rbtcee_Verdict,
};
use crate::rbtcei_invocation::{
    rbtcei_find_tabtarget_global,
    rbtcei_tabtarget_command,
    RBTCEI_BURE_CONFIRM_KEY,
    RBTCEI_BURE_CONFIRM_SKIP,
};
use crate::rbtcem_manifest::RBTCEM_FIXTURE_CHAINING_FACT_BAND;
use crate::rbtcep_platform::rbtcep_native_to_posix;

// Lode touchmark literals — mirror rbgc_constants.sh (RBGC_LODE_KIND_*,
// RBF_FACT_LODE_TOUCHMARK, RBGC_LODE_TAG_BOLE). A touchmark is
// <kind-letter(s)><YYMMDDHHMMSS>; the kind letters are the stable on-disk Lode
// format. The values are deliberately fixed (no clock) so the cases are
// deterministic — feoff/yoke decode the kind from the prefix and never resolve
// the touchmark against GAR on the paths these cases exercise.
const RBTCGH_FACT_TOUCHMARK: &str = "rbgc_fact_lode_touchmark"; // RBF_FACT_LODE_TOUCHMARK
const RBTCGH_BOLE_TOUCHMARK: &str = "b260327172456"; // RBGC_LODE_KIND_BOLE "b"
const RBTCGH_RELIQUARY_TOUCHMARK: &str = "r260327172456"; // RBGC_LODE_KIND_RELIQUARY "r"
const RBTCGH_UNKNOWN_TOUCHMARK: &str = "zz260327172456"; // no RBGC_LODE_KIND_* prefix
const RBTCGH_TAG_BOLE: &str = "rbi_bole"; // RBGC_LODE_TAG_BOLE

// The staged vessel's rbrv.env — one populated RBRV_IMAGE_1_ORIGIN slot, which
// is all feoff needs to locate the slot whose ANCHOR it elects. Shared between
// the stager and the fact-intact assertion so byte-identity is checked against
// the exact bytes written.
const RBTCGH_VESSEL_RBRV: &str = "RBRV_IMAGE_1_ORIGIN=docker.io/library/debian:bookworm\n";

// anoint's folio is the real tracked graft vessel — the sole graft-mode vessel in
// the tree. Unlike feoff (which reads a temp vessel by path), anoint LOADS the
// vessel through the strict zrbfcv_load_vessel, which demands it sit at its
// canonical RBRR_VESSEL_DIR location, so a temp-staged vessel cannot reach the
// chain read. The reject precedes the rewrite, so driving the real vessel leaves
// its rbrv.env untouched (the case asserts that byte-identity).
const RBTCGH_GRAFT_VESSEL: &str = "rbev_graft_demo";

// The mitre's folio is a real tracked nameplate — the mitre resolves the target
// rbrn.env by MONIKER (never a path, unlike feoff's temp vessel), so it must address
// a real nameplate under rbmm_moorings. The reject precedes the rewrite, so the
// mitre leaves its rbrn.env byte-identical (the case asserts that identity).
const RBTCGH_MITRE_NAMEPLATE: &str = "tadmor";

// ── Harness ─────────────────────────────────────────────────

/// Shared feoff driver over a case-shared BURV root (`dir`/burv — successive
/// calls with one `dir` exercise the REAL cross-dispatch promotion + relay hop).
/// Stages a vessel named `vessel_name` under `dir` (or deliberately omits it,
/// passing the nonexistent path anyway — the post-relay failure), optionally
/// seeds a chained touchmark fact into current/ (which bud promotes to
/// previous/), then drives the feoff tabtarget. Returns (exit_code, rbrv.env
/// path). The vessel and BURV roots live under the case temp dir, so feoff's
/// rbrv.env rewrite never reaches tracked config.
fn rbtcgh_drive_feoff_shared(
    dir: &Path,
    vessel_name: &str,
    stage_vessel: bool,
    seed_chain: Option<&str>,
    express: Option<&str>,
    label: &str,
) -> Result<(i32, PathBuf), String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtcei_find_tabtarget_global(&root, RBTGEC_FEOFF_BOLE)?;

    let vessel_dir = dir.join(vessel_name);
    let rbrv = vessel_dir.join(RBTGEC_RBRV_FILE);
    if stage_vessel {
        std::fs::create_dir_all(&vessel_dir).map_err(|e| format!("stage vessel dir: {}", e))?;
        std::fs::write(&rbrv, RBTCGH_VESSEL_RBRV).map_err(|e| format!("stage rbrv.env: {}", e))?;
    }

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;
    match seed_chain {
        Some(touchmark) => {
            // Seed current/; bud's start-of-dispatch promotion moves it to previous/.
            let current = burv.join("current");
            std::fs::create_dir_all(&current).map_err(|e| format!("mkdir burv current: {}", e))?;
            std::fs::write(current.join(RBTCGH_FACT_TOUCHMARK), format!("{}\n", touchmark))
                .map_err(|e| format!("seed chain fact: {}", e))?;
        }
        None => {
            std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
        }
    }

    let mut cmd = rbtcei_tabtarget_command(&tt);
    cmd.arg(rbtcep_native_to_posix(&vessel_dir));
    if let Some(e) = express {
        cmd.arg(e);
    }
    cmd.env("BURV_OUTPUT_ROOT_DIR", rbtcep_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtcep_native_to_posix(&burv_temp))
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run feoff {}: {}", tt.display(), e))?;
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &output.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &output.stderr);
    Ok((output.status.code().unwrap_or(-1), rbrv))
}

/// Single-dispatch feoff drive — the original harness shape the negative and
/// precedence cases use. Returns (exit_code, staged rbrv.env path, promoted
/// previous/ fact path).
fn rbtcgh_drive_feoff(
    dir: &Path,
    seed_chain: Option<&str>,
    express: Option<&str>,
) -> Result<(i32, PathBuf, PathBuf), String> {
    let (code, rbrv) = rbtcgh_drive_feoff_shared(dir, "vessel", true, seed_chain, express, "feoff")?;
    let prev_fact = dir.join("burv").join("previous").join(RBTCGH_FACT_TOUCHMARK);
    Ok((code, rbrv, prev_fact))
}

/// Drive the yoke tabtarget with an express touchmark (yoke's folio). Returns
/// the exit code. yoke's kind gate rejects before auth and the fan-out write,
/// so a negative express never reaches credentials or the tracked vessel tree.
fn rbtcgh_drive_yoke(dir: &Path, express: &str) -> Result<i32, String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtcei_find_tabtarget_global(&root, RBTGEC_YOKE_RELIQUARY)?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;

    let mut cmd = rbtcei_tabtarget_command(&tt);
    cmd.arg(express)
        .env("BURV_OUTPUT_ROOT_DIR", rbtcep_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtcep_native_to_posix(&burv_temp))
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run yoke {}: {}", tt.display(), e))?;
    let _ = std::fs::write(dir.join("yoke-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("yoke-stderr.txt"), &output.stderr);
    Ok(output.status.code().unwrap_or(-1))
}

/// Drive the anoint tabtarget against the REAL tracked graft vessel (its mandatory
/// folio) with an EMPTY BURV root. anoint is a durable-config LINK like feoff/yoke,
/// but chain-only (no express), so a broken chain is its sole resolve failure. It
/// LOADS the vessel (the strict zrbfcv_load_vessel demands the canonical location),
/// so the folio must be the real tracked vessel, not a temp stage. With no chained
/// fact the resolve finds nothing and anoint rejects with the chaining band BEFORE
/// the rbrv.env rewrite — the same reject-before-write shape that keeps the yoke
/// negatives write-free, so the tracked rbrv.env is never mutated (asserted by the
/// case). Credless, no cloud — the reject fires before any token mint. Returns the
/// exit code.
fn rbtcgh_drive_anoint(dir: &Path) -> Result<i32, String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtcei_find_tabtarget_global(&root, RBTGEC_ANOINT_GRAFT)?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;

    let mut cmd = rbtcei_tabtarget_command(&tt);
    cmd.arg(RBTCGH_GRAFT_VESSEL)
        .env("BURV_OUTPUT_ROOT_DIR", rbtcep_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtcep_native_to_posix(&burv_temp))
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run anoint {}: {}", tt.display(), e))?;
    let _ = std::fs::write(dir.join("anoint-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("anoint-stderr.txt"), &output.stderr);
    Ok(output.status.code().unwrap_or(-1))
}

/// Drive the rbw-nm nameplate-mitre tabtarget against a REAL tracked nameplate (its
/// folio) with an EMPTY BURV root and no express hallmark. The mitre is the fourth
/// durable-config LINK (it rewrites RBRN_{BOTTLE,SENTRY}_HALLMARK); with no express
/// and no chained fact the express-or-chain resolve finds nothing and the mitre
/// rejects with the chaining band BEFORE the rbrn.env rewrite — the same reject-
/// before-write shape that keeps the anoint negative write-free, so the tracked
/// rbrn.env is never mutated (asserted by the case). The nameplate is addressed by
/// moniker and NOT loaded, so a still-blank hallmark field never blocks the run.
/// Credless, no cloud — the reject fires before any token mint. Returns the exit code.
fn rbtcgh_drive_mitre(dir: &Path, nameplate: &str, field: &str) -> Result<i32, String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtcei_find_tabtarget_global(&root, RBTGEC_MITRE_HALLMARK)?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;

    let mut cmd = rbtcei_tabtarget_command(&tt);
    cmd.arg(nameplate)
        .arg(field)
        .env("BURV_OUTPUT_ROOT_DIR", rbtcep_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtcep_native_to_posix(&burv_temp))
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| {
            format!(
                "failed to run {} {}: {}",
                RBTGEB_NAMEPLATE_MITRE_BASE,
                tt.display(),
                e
            )
        })?;
    let _ = std::fs::write(dir.join("mitre-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("mitre-stderr.txt"), &output.stderr);
    Ok(output.status.code().unwrap_or(-1))
}

/// Drive a read-side consumer (summon, plumb, augur, rekon) through its tabtarget with an
/// optional express folio, against an EMPTY BURV root (no chained fact). With no
/// express and no chain the express-or-chain resolve finds nothing and the verb
/// rejects with the chaining band BEFORE any auth — so the drive is credless and
/// needs no cloud. This asserts the resolve-logic rejection, NOT the furnish wiring:
/// a furnish gap exits the same band, so the static furnish case covers that.
/// `label` names the artifact files. Returns the exit code.
fn rbtcgh_drive_readside(
    dir: &Path,
    colophon: &str,
    label: &str,
    express: Option<&str>,
) -> Result<i32, String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtcei_find_tabtarget_global(&root, colophon)?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;

    let mut cmd = rbtcei_tabtarget_command(&tt);
    if let Some(e) = express {
        cmd.arg(e);
    }
    cmd.env("BURV_OUTPUT_ROOT_DIR", rbtcep_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtcep_native_to_posix(&burv_temp))
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run {} {}: {}", label, tt.display(), e))?;
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &output.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &output.stderr);
    Ok(output.status.code().unwrap_or(-1))
}

/// Assert an exit code equals the chain-rejection band, with the artifact dir
/// named for triage on mismatch.
fn rbtcgh_expect_band(code: i32, label: &str, dir: &Path) -> rbtcee_Verdict {
    if code == RBTGEB_CHAIN_DENTIL {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "{}: exited {} — expected chain-rejection band {} (artifacts in {})",
            label, code, RBTGEB_CHAIN_DENTIL, dir.display()
        ))
    }
}

// ── feoff cases ─────────────────────────────────────────────

fn rbtcgh_feoff_wrong_kind(dir: &Path) -> rbtcee_Verdict {
    // An express reliquary touchmark decodes fine but is the wrong kind — feoff
    // elects a base anchor, which only a bole carries.
    match rbtcgh_drive_feoff(dir, None, Some(RBTCGH_RELIQUARY_TOUCHMARK)) {
        Ok((code, _, _)) => rbtcgh_expect_band(code, "feoff wrong-kind (reliquary express)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_feoff_unknown_prefix(dir: &Path) -> rbtcee_Verdict {
    // A touchmark with no recognizable Lode kind prefix — the decoder rejects.
    match rbtcgh_drive_feoff(dir, None, Some(RBTCGH_UNKNOWN_TOUCHMARK)) {
        Ok((code, _, _)) => rbtcgh_expect_band(code, "feoff unknown-prefix express", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_feoff_broken_chain(dir: &Path) -> rbtcee_Verdict {
    // No express and an empty previous/ — the express-or-chain resolve finds
    // nothing: a broken chain.
    match rbtcgh_drive_feoff(dir, None, None) {
        Ok((code, _, _)) => rbtcgh_expect_band(code, "feoff broken chain (no express, empty previous/)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_feoff_good(dir: &Path) -> rbtcee_Verdict {
    // An express bole touchmark elects the base anchor: exit 0, and the staged
    // rbrv.env carries the elected ANCHOR line bearing that touchmark.
    let (code, rbrv, _) = match rbtcgh_drive_feoff(dir, None, Some(RBTCGH_BOLE_TOUCHMARK)) {
        Ok(t) => t,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    if code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "feoff good (bole express) exited {} — expected 0 (artifacts in {})",
            code, dir.display()
        ));
    }
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    let expected_locator = format!("{}:{}", RBTCGH_BOLE_TOUCHMARK, RBTCGH_TAG_BOLE);
    if content.contains("RBRV_IMAGE_1_ANCHOR=") && content.contains(&expected_locator) {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "feoff good wrote no ANCHOR bearing '{}'; rbrv.env:\n{}",
            expected_locator, content
        ))
    }
}

fn rbtcgh_feoff_precedence(dir: &Path) -> rbtcee_Verdict {
    // Express bole AND a seeded reliquary chain fact present: express must win
    // (the chain is relayed but never read), so the election succeeds with the
    // bole and the reliquary touchmark never appears in the rewritten rbrv.env.
    let (code, rbrv, _) =
        match rbtcgh_drive_feoff(dir, Some(RBTCGH_RELIQUARY_TOUCHMARK), Some(RBTCGH_BOLE_TOUCHMARK)) {
            Ok(t) => t,
            Err(e) => return rbtcee_Verdict::Fail(e),
        };
    if code != 0 {
        return rbtcee_Verdict::Fail(format!(
            "feoff precedence (bole express over reliquary chain) exited {} — expected 0 \
             (a chain-read would have rejected wrong-kind; artifacts in {})",
            code, dir.display()
        ));
    }
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    let bole_locator = format!("{}:{}", RBTCGH_BOLE_TOUCHMARK, RBTCGH_TAG_BOLE);
    if content.contains(&bole_locator) && !content.contains(RBTCGH_RELIQUARY_TOUCHMARK) {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "feoff precedence did not elect the express bole over the chained reliquary; rbrv.env:\n{}",
            content
        ))
    }
}

fn rbtcgh_feoff_fact_intact(dir: &Path) -> rbtcee_Verdict {
    // The operator's worry, defended: a GOOD (valid) reliquary touchmark sits in
    // previous/ as the chained fact — good, but the wrong kind for feoff. With no
    // express, feoff reads it, the bole gate rejects it with the band, and must
    // have written NOTHING durable: the staged rbrv.env and the seeded fact both
    // survive byte-identical. Rejection precedes any destructive write; the relay
    // copies the baton forward but never mutates its previous/ source (RBr_3e7).
    let (code, rbrv, prev_fact) =
        match rbtcgh_drive_feoff(dir, Some(RBTCGH_RELIQUARY_TOUCHMARK), None) {
            Ok(t) => t,
            Err(e) => return rbtcee_Verdict::Fail(e),
        };
    if code != RBTGEB_CHAIN_DENTIL {
        return rbtcee_Verdict::Fail(format!(
            "feoff wrong-kind chain exited {} — expected band {} (artifacts in {})",
            code, RBTGEB_CHAIN_DENTIL, dir.display()
        ));
    }
    let rbrv_after = std::fs::read_to_string(&rbrv).unwrap_or_default();
    if rbrv_after != RBTCGH_VESSEL_RBRV {
        return rbtcee_Verdict::Fail(format!(
            "rbrv.env was mutated under a band reject (rejection must precede any write):\n{}",
            rbrv_after
        ));
    }
    let seeded = format!("{}\n", RBTCGH_RELIQUARY_TOUCHMARK);
    let fact_after = std::fs::read_to_string(&prev_fact).unwrap_or_default();
    if fact_after != seeded {
        return rbtcee_Verdict::Fail(format!(
            "the seeded previous/ fact was mutated under a band reject (the relay \
             copies, never mutates its previous/ source): {:?}",
            fact_after
        ));
    }
    rbtcee_Verdict::Pass
}

// ── yoke cases ──────────────────────────────────────────────

fn rbtcgh_yoke_wrong_kind(dir: &Path) -> rbtcee_Verdict {
    // An express bole touchmark decodes fine but is the wrong kind — yoke
    // requires a reliquary. Rejects before auth and the fan-out write.
    match rbtcgh_drive_yoke(dir, RBTCGH_BOLE_TOUCHMARK) {
        Ok(code) => rbtcgh_expect_band(code, "yoke wrong-kind (bole express)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_yoke_unknown_prefix(dir: &Path) -> rbtcee_Verdict {
    match rbtcgh_drive_yoke(dir, RBTCGH_UNKNOWN_TOUCHMARK) {
        Ok(code) => rbtcgh_expect_band(code, "yoke unknown-prefix express", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

// ── anoint case ─────────────────────────────────────────────
// anoint is the third durable-config LINK (it rewrites RBRV_GRAFT_IMAGE), write-side
// sibling of feoff/yoke. It is chain-only — no express — so a broken chain is its
// only resolve failure. Driven against the real graft vessel with an empty BURV
// root, anoint loads the vessel, then the first fact read fails and rejects with
// the band BEFORE the rewrite, so the tracked rbrv.env must survive byte-identical.

fn rbtcgh_anoint_broken_chain(dir: &Path) -> rbtcee_Verdict {
    let rbrv = match std::env::current_dir() {
        Ok(r) => r
            .join("rbmm_moorings")
            .join("rbmv_vessels")
            .join(RBTCGH_GRAFT_VESSEL)
            .join(RBTGEC_RBRV_FILE),
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let before = std::fs::read_to_string(&rbrv).unwrap_or_default();

    let code = match rbtcgh_drive_anoint(dir) {
        Ok(c) => c,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    if code != RBTGEB_CHAIN_DENTIL {
        return rbtcee_Verdict::Fail(format!(
            "anoint broken chain (empty BURV root) exited {} — expected band {} (artifacts in {})",
            code, RBTGEB_CHAIN_DENTIL, dir.display()
        ));
    }

    // The reject must precede the rewrite — that invariant is what makes driving the
    // real tracked vessel write-free. Prove the rbrv.env is byte-identical after.
    let after = std::fs::read_to_string(&rbrv).unwrap_or_default();
    if after != before {
        return rbtcee_Verdict::Fail(format!(
            "the tracked graft vessel rbrv.env was mutated under a band reject \
             (rejection must precede any write); after:\n{}",
            after
        ));
    }
    rbtcee_Verdict::Pass
}

// ── mitre case ──────────────────────────────────────────────
// mitre is the fourth durable-config LINK (it rewrites RBRN_{BOTTLE,SENTRY}_HALLMARK,
// the rbrn_regime sibling of feoff/yoke/anoint). It is express-or-chain like feoff,
// but resolves the nameplate by MONIKER (never a path), so — like anoint — it cannot
// use a temp target and takes the real tracked nameplate as its folio. With no
// express and an empty BURV root the chain read fails and mitre rejects with the band
// BEFORE the rbrn.env rewrite, so the tracked nameplate stays byte-identical.

fn rbtcgh_mitre_broken_chain(dir: &Path) -> rbtcee_Verdict {
    let rbrn = match std::env::current_dir() {
        Ok(r) => r
            .join("rbmm_moorings")
            .join(format!(
                "{}{}",
                crate::rbtgec_consts::RBTGEC_NAMEPLATE_SPRUE,
                RBTCGH_MITRE_NAMEPLATE
            ))
            .join("rbrn.env"),
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let before = std::fs::read_to_string(&rbrn).unwrap_or_default();

    let code = match rbtcgh_drive_mitre(dir, RBTCGH_MITRE_NAMEPLATE, RBTGEC_CONTAINER_BOTTLE) {
        Ok(c) => c,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    if code != RBTGEB_CHAIN_DENTIL {
        return rbtcee_Verdict::Fail(format!(
            "{} broken chain (empty BURV root, no express) exited {} — expected band {} (artifacts in {})",
            RBTGEB_NAMEPLATE_MITRE_BASE,
            code, RBTGEB_CHAIN_DENTIL, dir.display()
        ));
    }

    // The reject must precede the rewrite — that invariant is what makes driving the
    // real tracked nameplate write-free. Prove the rbrn.env is byte-identical after.
    let after = std::fs::read_to_string(&rbrn).unwrap_or_default();
    if after != before {
        return rbtcee_Verdict::Fail(format!(
            "the tracked nameplate rbrn.env was mutated under a band reject \
             (rejection must precede any write); after:\n{}",
            after
        ));
    }
    rbtcee_Verdict::Pass
}

// ── read-side consumer cases ────────────────────────────────
// summon/plumb/augur/rekon write no durable config, but they resolve the same
// express-or-chain fact and now reject a broken resolve with the same band as the
// LINKS. Each is driven folio-less against an empty BURV root, so the resolve fails
// before any auth — credless, no cloud. These prove the resolve-logic rejection; a
// furnish gap exits the same band 105, so the static furnish case (not these) is the
// net for that.

fn rbtcgh_summon_no_folio(dir: &Path) -> rbtcee_Verdict {
    match rbtcgh_drive_readside(dir, RBTGEC_SUMMON_HALLMARK, "summon", None) {
        Ok(code) => rbtcgh_expect_band(code, "summon no-folio (broken chain)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_plumb_no_folio(dir: &Path) -> rbtcee_Verdict {
    match rbtcgh_drive_readside(dir, RBTGEC_PLUMB_FULL, "plumb", None) {
        Ok(code) => rbtcgh_expect_band(code, "plumb no-folio (broken chain)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_augur_no_folio(dir: &Path) -> rbtcee_Verdict {
    match rbtcgh_drive_readside(dir, RBTGEC_AUGUR_LODE, "augur", None) {
        Ok(code) => rbtcgh_expect_band(code, "augur no-folio (broken chain)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_augur_unknown_prefix(dir: &Path) -> rbtcee_Verdict {
    // An express touchmark with no recognizable kind prefix — augur accepts any KNOWN
    // kind, so its decode gate rejects the unknown one with the chaining band, before auth.
    match rbtcgh_drive_readside(dir, RBTGEC_AUGUR_LODE, "augur-badkind", Some(RBTCGH_UNKNOWN_TOUCHMARK)) {
        Ok(code) => rbtcgh_expect_band(code, "augur unknown-prefix express", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn rbtcgh_rekon_no_folio(dir: &Path) -> rbtcee_Verdict {
    match rbtcgh_drive_readside(dir, RBTGEC_REKON_HALLMARK, "rekon", None) {
        Ok(code) => rbtcgh_expect_band(code, "rekon no-folio (broken chain)", dir),
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

// ── relay-then-read lifetime cases ──────────────────────────
// The chain-lifetime law (RBr_3e7) proven at the bash-verb grain with feoff:
// REAL successive dispatches over one shared BURV root, the promotion between
// them the dispatcher's own. The two positive cases (multi-consumer, retry) are
// the ones the old terminal-consumption law would have failed; the staleness
// case proves the kill side — a non-chain dispatch (the nihil calibrant, which
// never relays) drops the chain, so the next consumer rejects with the band.

/// Dispatch the nihil calibrant tabtarget (rbw-tn) against the case-shared BURV
/// root under `dir` — a real non-chain dispatch: it promotes like any other but
/// relays nothing, so a live fact dies at its promotion horizon.
fn rbtcgh_drive_nihil(dir: &Path) -> Result<i32, String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtcei_find_tabtarget_global(&root, RBTGEC_THEURGE_NIHIL)?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;

    let mut cmd = rbtcei_tabtarget_command(&tt);
    cmd.env("BURV_OUTPUT_ROOT_DIR", rbtcep_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtcep_native_to_posix(&burv_temp))
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run nihil {}: {}", tt.display(), e))?;
    let _ = std::fs::write(dir.join("nihil-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("nihil-stderr.txt"), &output.stderr);
    Ok(output.status.code().unwrap_or(-1))
}

fn rbtcgh_multi_consumer(dir: &Path) -> rbtcee_Verdict {
    // One seeded bole fact, two consumers in successive dispatches: feoff A
    // relays then elects; feoff B's dispatch-start promotion finds A's relayed
    // baton, so B's chain read still elects the same touchmark.
    let (code_a, rbrv_a) = match rbtcgh_drive_feoff_shared(
        dir, "vessel-a", true, Some(RBTCGH_BOLE_TOUCHMARK), None, "feoff-a",
    ) {
        Ok(t) => t,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    if code_a != 0 {
        return rbtcee_Verdict::Fail(format!(
            "first consumer (feoff A, chained bole) exited {} — expected 0 (artifacts in {})",
            code_a, dir.display()
        ));
    }
    let (code_b, rbrv_b) =
        match rbtcgh_drive_feoff_shared(dir, "vessel-b", true, None, None, "feoff-b") {
            Ok(t) => t,
            Err(e) => return rbtcee_Verdict::Fail(e),
        };
    if code_b != 0 {
        return rbtcee_Verdict::Fail(format!(
            "second consumer (feoff B, relay-carried bole) exited {} — expected 0: \
             the first consumer's relay must keep the fact alive (artifacts in {})",
            code_b, dir.display()
        ));
    }
    let expected_locator = format!("{}:{}", RBTCGH_BOLE_TOUCHMARK, RBTCGH_TAG_BOLE);
    for (name, rbrv) in [("A", &rbrv_a), ("B", &rbrv_b)] {
        let content = std::fs::read_to_string(rbrv).unwrap_or_default();
        if !content.contains(&expected_locator) {
            return rbtcee_Verdict::Fail(format!(
                "vessel {} rbrv.env lacks the elected anchor '{}':\n{}",
                name, expected_locator, content
            ));
        }
    }
    rbtcee_Verdict::Pass
}

fn rbtcgh_retry_after_failure(dir: &Path) -> rbtcee_Verdict {
    // Fail-after-forward: consumer 1 relays at the top of the verb, then dies on
    // a missing vessel — a failure unrelated to the chain. Its retry (dispatch 2)
    // still finds the baton, because the relay preceded the failure point.
    let (code_fail, _) = match rbtcgh_drive_feoff_shared(
        dir, "vessel-missing", false, Some(RBTCGH_BOLE_TOUCHMARK), None, "feoff-fail",
    ) {
        Ok(t) => t,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    if code_fail == 0 {
        return rbtcee_Verdict::Fail(format!(
            "feoff against a missing vessel exited 0 — expected a failure (artifacts in {})",
            dir.display()
        ));
    }
    let (code_retry, rbrv) =
        match rbtcgh_drive_feoff_shared(dir, "vessel-retry", true, None, None, "feoff-retry") {
            Ok(t) => t,
            Err(e) => return rbtcee_Verdict::Fail(e),
        };
    if code_retry != 0 {
        return rbtcee_Verdict::Fail(format!(
            "retry after a failed consumer exited {} — expected 0: the failed consumer's \
             relay must have passed the baton to its own retry (artifacts in {})",
            code_retry, dir.display()
        ));
    }
    let expected_locator = format!("{}:{}", RBTCGH_BOLE_TOUCHMARK, RBTCGH_TAG_BOLE);
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    if content.contains(&expected_locator) {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "retry elected no anchor bearing '{}'; rbrv.env:\n{}",
            expected_locator, content
        ))
    }
}

fn rbtcgh_dies_at_non_chain_dispatch(dir: &Path) -> rbtcee_Verdict {
    // The staleness bound's kill side: a live bole fact, then a NON-chain dispatch
    // (nihil — promotes but never relays), then a chain consumer. The consumer's
    // dispatch-start promotion finds only nihil's relay-less current/, so the
    // fact is gone and feoff rejects with the band — the chain died at the first
    // non-chain dispatch, exactly the behavioral staleness bound (RBr_3e7).
    let seed_dir = dir.join("burv").join("current");
    if let Err(e) = std::fs::create_dir_all(&seed_dir) {
        return rbtcee_Verdict::Fail(format!("mkdir burv current: {}", e));
    }
    if let Err(e) = std::fs::write(
        seed_dir.join(RBTCGH_FACT_TOUCHMARK),
        format!("{}\n", RBTCGH_BOLE_TOUCHMARK),
    ) {
        return rbtcee_Verdict::Fail(format!("seed chain fact: {}", e));
    }

    let code_nihil = match rbtcgh_drive_nihil(dir) {
        Ok(c) => c,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    if code_nihil != 0 {
        return rbtcee_Verdict::Fail(format!(
            "nihil dispatch exited {} — expected 0 (artifacts in {})",
            code_nihil, dir.display()
        ));
    }

    let (code, rbrv) =
        match rbtcgh_drive_feoff_shared(dir, "vessel-stale", true, None, None, "feoff-stale") {
            Ok(t) => t,
            Err(e) => return rbtcee_Verdict::Fail(e),
        };
    if code != RBTGEB_CHAIN_DENTIL {
        return rbtcee_Verdict::Fail(format!(
            "feoff after an intervening non-chain dispatch exited {} — expected chain-rejection \
             band {}: the fact must die at the first non-chain dispatch (artifacts in {})",
            code, RBTGEB_CHAIN_DENTIL, dir.display()
        ));
    }
    // The reject left the staged vessel untouched — no stale value was elected.
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    if content != RBTCGH_VESSEL_RBRV {
        return rbtcee_Verdict::Fail(format!(
            "rbrv.env was mutated under the staleness reject:\n{}",
            content
        ));
    }
    rbtcee_Verdict::Pass
}

// ── furnish invariant (static) ──────────────────────────────
// The chaining consumers reach the buf_* fact helpers through their dispatching
// CLI; the CLI must source buf_fact or the helper is undefined and the verb dies
// command-not-found at the resolve — the furnish gap that started this work. The
// runtime cases above cannot see it (a furnish gap exits the same band), so it is
// asserted here statically: for every rb*_cli.sh whose transitive source closure
// reaches a buf_* fact caller, buf_fact.sh must be in that closure. Reads source,
// runs nothing — the development-time net for a coding error, never a tolerated
// runtime path. The fix is CLI-level (the lode/ledger precedent), so the closure is
// followed transitively: the 0-trick CLIs reach their caller modules through a
// gestalt entry, not a direct source.

const RBTCGH_FACT_CALLERS: &[&str] = &["buf_elect_fact_capture", "buf_read_fact_capture", "buf_relay"];
const RBTCGH_FACT_MODULE: &str = "buf_fact.sh";

/// Resolve a `source "..."` line to a repo path under Tools/rbk or Tools/buk, or
/// None when the prefix is a runtime variable not statically resolvable (e.g.
/// ${RBCC_rbrr_file} — a regime env file, never a fact caller). The three resolved
/// prefixes are the only ones a kit module uses to source sibling code.
fn rbtcgh_resolve_source(line: &str, rbk: &Path, buk: &Path) -> Option<PathBuf> {
    let q = line.find('"')?;
    let rest = &line[q + 1..];
    let end = rest.find('"')?;
    let spec = &rest[..end];
    let base = spec.rsplit('/').next()?;
    if !base.ends_with(".sh") {
        return None;
    }
    if spec.starts_with("${BURD_BUK_DIR}/") {
        Some(buk.join(base))
    } else if spec.starts_with("${BASH_SOURCE[0]%/*}/") || spec.starts_with("${z_rbk_kit_dir}/") {
        Some(rbk.join(base))
    } else {
        None
    }
}

/// Transitive source closure of `start`, following only statically-resolvable
/// source lines. Returns every file reached, including `start`.
fn rbtcgh_source_closure(start: &Path, rbk: &Path, buk: &Path) -> Vec<PathBuf> {
    let mut seen: Vec<PathBuf> = Vec::new();
    let mut queue: Vec<PathBuf> = vec![start.to_path_buf()];
    while let Some(f) = queue.pop() {
        if seen.iter().any(|s| s == &f) {
            continue;
        }
        seen.push(f.clone());
        let content = match std::fs::read_to_string(&f) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for line in content.lines() {
            if !line.trim_start().starts_with("source ") {
                continue;
            }
            if let Some(target) = rbtcgh_resolve_source(line, rbk, buk) {
                queue.push(target);
            }
        }
    }
    seen
}

/// Static furnish invariant: every dispatching CLI whose source closure reaches a
/// buf_* fact caller must also source buf_fact. A miss is the furnish gap — the
/// helper undefined at runtime, the verb dying command-not-found at the resolve.
fn rbtcgh_furnish_invariant(dir: &Path) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let rbk = root.join("Tools").join("rbk");
    let buk = root.join("Tools").join("buk");

    let entries = match std::fs::read_dir(&rbk) {
        Ok(e) => e,
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot read {}: {}", rbk.display(), e)),
    };
    let mut clis: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("_cli.sh"))
                .unwrap_or(false)
        })
        .collect();
    clis.sort();

    let mut violations: Vec<String> = Vec::new();
    let mut covered = 0usize;
    for cli in &clis {
        let closure = rbtcgh_source_closure(cli, &rbk, &buk);
        let mut calls_fact = false;
        let mut has_furnish = false;
        for f in &closure {
            let base = f.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if base == RBTCGH_FACT_MODULE {
                has_furnish = true; // buf_fact's own definitions are not "calls"
                continue;
            }
            let content = std::fs::read_to_string(f).unwrap_or_default();
            if RBTCGH_FACT_CALLERS.iter().any(|c| content.contains(c)) {
                calls_fact = true;
            }
        }
        if calls_fact {
            let name = cli.file_name().and_then(|n| n.to_str()).unwrap_or("?");
            if has_furnish {
                covered += 1;
            } else {
                violations.push(format!(
                    "{} reaches a buf_* fact caller but its source closure omits {}",
                    name, RBTCGH_FACT_MODULE
                ));
            }
        }
    }

    let _ = std::fs::write(
        dir.join("furnish-invariant.txt"),
        format!(
            "CLIs scanned: {}; fact-consuming + furnished: {}\nviolations:\n{}\n",
            clis.len(), covered, violations.join("\n")
        ),
    );

    if violations.is_empty() {
        rbtcee_Verdict::Pass
    } else {
        rbtcee_Verdict::Fail(format!(
            "{} furnish gap(s) — a CLI reaches a buf_* fact caller without sourcing {}:\n{}",
            violations.len(), RBTCGH_FACT_MODULE, violations.join("\n")
        ))
    }
}

// ── Fixture ─────────────────────────────────────────────────

pub static RBTCGH_CASES_CHAINING_FACT_BAND: &[rbtcee_Case] = &[
    rbtcee_case!(rbtcgh_feoff_wrong_kind),
    rbtcee_case!(rbtcgh_feoff_unknown_prefix),
    rbtcee_case!(rbtcgh_feoff_broken_chain),
    rbtcee_case!(rbtcgh_feoff_good),
    rbtcee_case!(rbtcgh_feoff_precedence),
    rbtcee_case!(rbtcgh_feoff_fact_intact),
    rbtcee_case!(rbtcgh_yoke_wrong_kind),
    rbtcee_case!(rbtcgh_yoke_unknown_prefix),
    rbtcee_case!(rbtcgh_anoint_broken_chain),
    rbtcee_case!(rbtcgh_mitre_broken_chain),
    rbtcee_case!(rbtcgh_summon_no_folio),
    rbtcee_case!(rbtcgh_plumb_no_folio),
    rbtcee_case!(rbtcgh_augur_no_folio),
    rbtcee_case!(rbtcgh_augur_unknown_prefix),
    rbtcee_case!(rbtcgh_rekon_no_folio),
    rbtcee_case!(rbtcgh_multi_consumer),
    rbtcee_case!(rbtcgh_retry_after_failure),
    rbtcee_case!(rbtcgh_dies_at_non_chain_dispatch),
    rbtcee_case!(rbtcgh_furnish_invariant),
];

pub static RBTCGH_FIXTURE_CHAINING_FACT_BAND: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_CHAINING_FACT_BAND,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCGH_CASES_CHAINING_FACT_BAND,
    credless: true,
    tariff: rbtcee_Tariff { min_secs: None, max_secs: Some(25), invocations: Some(21) },
};
