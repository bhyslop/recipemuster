// Copyright 2026 Scale Invariant, Inc.
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
// RBTDRH — chaining-fact-band fixture: the band matrix for the durable-leak
// chain LINKS (feoff, yoke), driven through the real tabtarget exec path.
//
// The chaining-fact discipline splits the value-forwarding verbs by role; only
// feoff/anoint/yoke write durable config from a resolved express-or-chain value,
// and a bad resolution (a wrong-kind touchmark, a broken chain) must be REJECTED
// with the named precision band — never a bare nonzero, and never after a
// destructive write. The band fires only at the RBK consumer (buc_reject in
// feoff/yoke); the BUK footing resolver returns a bare 1, so the band can only
// be asserted here, against the real verbs. Each negative case asserts the
// SPECIFIC band (RBTDGC_BAND_CHAIN) — like the regime-poison precedent, a
// harness breakage exits with some other code and fails the case loud, where a
// bare-nonzero assertion would pass on it.
//
// feoff resolves its vessel by PATH (zrbfc_resolve_vessel, never the strict
// load), so every feoff case runs against a vessel staged in the case temp dir —
// no tracked rbrv.env is ever touched. The chained fact is seeded by writing it
// into the BURV root's current/, which bud_dispatch promotes to previous/ at
// dispatch start (the same path chain_next_invoke replicates for cloud pairs).
// yoke fans out across the tracked vessel tree, but its kind gate rejects BEFORE
// auth and BEFORE the write loop, so the yoke negatives are creds-free and
// write-free. Nothing here mints a token — the fixture is credless.

use std::path::{Path, PathBuf};

use crate::case;
use crate::rbtdgc_consts::{
    RBTDGC_BAND_CHAIN,
    RBTDGC_FEOFF_BOLE,
    RBTDGC_YOKE_RELIQUARY,
};
use crate::rbtdre_engine::{
    rbtdre_Case,
    rbtdre_Disposition,
    rbtdre_Fixture,
    rbtdre_Verdict,
};
use crate::rbtdri_invocation::{
    rbtdri_find_tabtarget_global,
    rbtdri_tabtarget_command,
    RBTDRI_BURE_CONFIRM_KEY,
    RBTDRI_BURE_CONFIRM_SKIP,
};
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_CHAINING_FACT_BAND;
use crate::rbtdrx_platform::rbtdrx_native_to_posix;

// Lode touchmark literals — mirror rbgc_constants.sh (RBGC_LODE_KIND_*,
// RBF_FACT_LODE_TOUCHMARK, RBGC_LODE_TAG_BOLE). A touchmark is
// <kind-letter(s)><YYMMDDHHMMSS>; the kind letters are the stable on-disk Lode
// format. The values are deliberately fixed (no clock) so the cases are
// deterministic — feoff/yoke decode the kind from the prefix and never resolve
// the touchmark against GAR on the paths these cases exercise.
const RBTDRH_FACT_TOUCHMARK: &str = "rbf_fact_lode_touchmark"; // RBF_FACT_LODE_TOUCHMARK
const RBTDRH_BOLE_TOUCHMARK: &str = "b260327172456"; // RBGC_LODE_KIND_BOLE "b"
const RBTDRH_RELIQUARY_TOUCHMARK: &str = "r260327172456"; // RBGC_LODE_KIND_RELIQUARY "r"
const RBTDRH_UNKNOWN_TOUCHMARK: &str = "zz260327172456"; // no RBGC_LODE_KIND_* prefix
const RBTDRH_TAG_BOLE: &str = "rbi_bole"; // RBGC_LODE_TAG_BOLE

// The staged vessel's rbrv.env — one populated RBRV_IMAGE_1_ORIGIN slot, which
// is all feoff needs to locate the slot whose ANCHOR it elects. Shared between
// the stager and the fact-intact assertion so byte-identity is checked against
// the exact bytes written.
const RBTDRH_VESSEL_RBRV: &str = "RBRV_IMAGE_1_ORIGIN=docker.io/library/debian:bookworm\n";

// ── Harness ─────────────────────────────────────────────────

/// Stage a temp vessel under `dir`, prepare a BURV root (optionally seeding a
/// chained touchmark fact into current/, which bud promotes to previous/), then
/// drive the feoff tabtarget. Returns (exit_code, staged rbrv.env path,
/// promoted previous/ fact path). The vessel and BURV roots live under the case
/// temp dir, so feoff's rbrv.env rewrite never reaches tracked config.
fn rbtdrh_drive_feoff(
    dir: &Path,
    seed_chain: Option<&str>,
    express: Option<&str>,
) -> Result<(i32, PathBuf, PathBuf), String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtdri_find_tabtarget_global(&root, RBTDGC_FEOFF_BOLE)?;

    let vessel_dir = dir.join("vessel");
    std::fs::create_dir_all(&vessel_dir).map_err(|e| format!("stage vessel dir: {}", e))?;
    let rbrv = vessel_dir.join("rbrv.env");
    std::fs::write(&rbrv, RBTDRH_VESSEL_RBRV).map_err(|e| format!("stage rbrv.env: {}", e))?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;
    let prev_fact = burv.join("previous").join(RBTDRH_FACT_TOUCHMARK);
    match seed_chain {
        Some(touchmark) => {
            // Seed current/; bud's start-of-dispatch promotion moves it to previous/.
            let current = burv.join("current");
            std::fs::create_dir_all(&current).map_err(|e| format!("mkdir burv current: {}", e))?;
            std::fs::write(current.join(RBTDRH_FACT_TOUCHMARK), format!("{}\n", touchmark))
                .map_err(|e| format!("seed chain fact: {}", e))?;
        }
        None => {
            std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
        }
    }

    let mut cmd = rbtdri_tabtarget_command(&tt);
    cmd.arg(rbtdrx_native_to_posix(&vessel_dir));
    if let Some(e) = express {
        cmd.arg(e);
    }
    cmd.env("BURV_OUTPUT_ROOT_DIR", rbtdrx_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtdrx_native_to_posix(&burv_temp))
        .env(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run feoff {}: {}", tt.display(), e))?;
    let _ = std::fs::write(dir.join("feoff-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("feoff-stderr.txt"), &output.stderr);
    Ok((output.status.code().unwrap_or(-1), rbrv, prev_fact))
}

/// Drive the yoke tabtarget with an express touchmark (yoke's folio). Returns
/// the exit code. yoke's kind gate rejects before auth and the fan-out write,
/// so a negative express never reaches credentials or the tracked vessel tree.
fn rbtdrh_drive_yoke(dir: &Path, express: &str) -> Result<i32, String> {
    let root = std::env::current_dir().map_err(|e| format!("cannot get cwd: {}", e))?;
    let tt = rbtdri_find_tabtarget_global(&root, RBTDGC_YOKE_RELIQUARY)?;

    let burv = dir.join("burv");
    let burv_temp = dir.join("burvtmp");
    std::fs::create_dir_all(&burv).map_err(|e| format!("mkdir burv root: {}", e))?;
    std::fs::create_dir_all(&burv_temp).map_err(|e| format!("mkdir burv temp: {}", e))?;

    let mut cmd = rbtdri_tabtarget_command(&tt);
    cmd.arg(express)
        .env("BURV_OUTPUT_ROOT_DIR", rbtdrx_native_to_posix(&burv))
        .env("BURV_TEMP_ROOT_DIR", rbtdrx_native_to_posix(&burv_temp))
        .env(RBTDRI_BURE_CONFIRM_KEY, RBTDRI_BURE_CONFIRM_SKIP)
        .current_dir(&root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to run yoke {}: {}", tt.display(), e))?;
    let _ = std::fs::write(dir.join("yoke-stdout.txt"), &output.stdout);
    let _ = std::fs::write(dir.join("yoke-stderr.txt"), &output.stderr);
    Ok(output.status.code().unwrap_or(-1))
}

/// Assert an exit code equals the chain-rejection band, with the artifact dir
/// named for triage on mismatch.
fn rbtdrh_expect_band(code: i32, label: &str, dir: &Path) -> rbtdre_Verdict {
    if code == RBTDGC_BAND_CHAIN {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "{}: exited {} — expected chain-rejection band {} (artifacts in {})",
            label, code, RBTDGC_BAND_CHAIN, dir.display()
        ))
    }
}

// ── feoff cases ─────────────────────────────────────────────

fn rbtdrh_feoff_wrong_kind(dir: &Path) -> rbtdre_Verdict {
    // An express reliquary touchmark decodes fine but is the wrong kind — feoff
    // elects a base anchor, which only a bole carries.
    match rbtdrh_drive_feoff(dir, None, Some(RBTDRH_RELIQUARY_TOUCHMARK)) {
        Ok((code, _, _)) => rbtdrh_expect_band(code, "feoff wrong-kind (reliquary express)", dir),
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrh_feoff_unknown_prefix(dir: &Path) -> rbtdre_Verdict {
    // A touchmark with no recognizable Lode kind prefix — the decoder rejects.
    match rbtdrh_drive_feoff(dir, None, Some(RBTDRH_UNKNOWN_TOUCHMARK)) {
        Ok((code, _, _)) => rbtdrh_expect_band(code, "feoff unknown-prefix express", dir),
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrh_feoff_broken_chain(dir: &Path) -> rbtdre_Verdict {
    // No express and an empty previous/ — the express-or-chain resolve finds
    // nothing: a broken chain.
    match rbtdrh_drive_feoff(dir, None, None) {
        Ok((code, _, _)) => rbtdrh_expect_band(code, "feoff broken chain (no express, empty previous/)", dir),
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrh_feoff_good(dir: &Path) -> rbtdre_Verdict {
    // An express bole touchmark elects the base anchor: exit 0, and the staged
    // rbrv.env carries the elected ANCHOR line bearing that touchmark.
    let (code, rbrv, _) = match rbtdrh_drive_feoff(dir, None, Some(RBTDRH_BOLE_TOUCHMARK)) {
        Ok(t) => t,
        Err(e) => return rbtdre_Verdict::Fail(e),
    };
    if code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "feoff good (bole express) exited {} — expected 0 (artifacts in {})",
            code, dir.display()
        ));
    }
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    let expected_locator = format!("{}:{}", RBTDRH_BOLE_TOUCHMARK, RBTDRH_TAG_BOLE);
    if content.contains("RBRV_IMAGE_1_ANCHOR=") && content.contains(&expected_locator) {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "feoff good wrote no ANCHOR bearing '{}'; rbrv.env:\n{}",
            expected_locator, content
        ))
    }
}

fn rbtdrh_feoff_precedence(dir: &Path) -> rbtdre_Verdict {
    // Express bole AND a seeded reliquary chain fact present: express must win
    // (the chain is never read), so the election succeeds with the bole and the
    // reliquary touchmark never appears in the rewritten rbrv.env.
    let (code, rbrv, _) =
        match rbtdrh_drive_feoff(dir, Some(RBTDRH_RELIQUARY_TOUCHMARK), Some(RBTDRH_BOLE_TOUCHMARK)) {
            Ok(t) => t,
            Err(e) => return rbtdre_Verdict::Fail(e),
        };
    if code != 0 {
        return rbtdre_Verdict::Fail(format!(
            "feoff precedence (bole express over reliquary chain) exited {} — expected 0 \
             (a chain-read would have rejected wrong-kind; artifacts in {})",
            code, dir.display()
        ));
    }
    let content = std::fs::read_to_string(&rbrv).unwrap_or_default();
    let bole_locator = format!("{}:{}", RBTDRH_BOLE_TOUCHMARK, RBTDRH_TAG_BOLE);
    if content.contains(&bole_locator) && !content.contains(RBTDRH_RELIQUARY_TOUCHMARK) {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "feoff precedence did not elect the express bole over the chained reliquary; rbrv.env:\n{}",
            content
        ))
    }
}

fn rbtdrh_feoff_fact_intact(dir: &Path) -> rbtdre_Verdict {
    // The operator's worry, defended: a GOOD (valid) reliquary touchmark sits in
    // previous/ as the chained fact — good, but the wrong kind for feoff. With no
    // express, feoff reads it, the bole gate rejects it with the band, and must
    // have written NOTHING: the staged rbrv.env and the seeded fact both survive
    // byte-identical. Rejection precedes any destructive write, and the chain is
    // terminally consumed (never relayed or mutated).
    let (code, rbrv, prev_fact) =
        match rbtdrh_drive_feoff(dir, Some(RBTDRH_RELIQUARY_TOUCHMARK), None) {
            Ok(t) => t,
            Err(e) => return rbtdre_Verdict::Fail(e),
        };
    if code != RBTDGC_BAND_CHAIN {
        return rbtdre_Verdict::Fail(format!(
            "feoff wrong-kind chain exited {} — expected band {} (artifacts in {})",
            code, RBTDGC_BAND_CHAIN, dir.display()
        ));
    }
    let rbrv_after = std::fs::read_to_string(&rbrv).unwrap_or_default();
    if rbrv_after != RBTDRH_VESSEL_RBRV {
        return rbtdre_Verdict::Fail(format!(
            "rbrv.env was mutated under a band reject (rejection must precede any write):\n{}",
            rbrv_after
        ));
    }
    let seeded = format!("{}\n", RBTDRH_RELIQUARY_TOUCHMARK);
    let fact_after = std::fs::read_to_string(&prev_fact).unwrap_or_default();
    if fact_after != seeded {
        return rbtdre_Verdict::Fail(format!(
            "the seeded previous/ fact was mutated under a band reject (chain must be \
             terminally consumed, never altered): {:?}",
            fact_after
        ));
    }
    rbtdre_Verdict::Pass
}

// ── yoke cases ──────────────────────────────────────────────

fn rbtdrh_yoke_wrong_kind(dir: &Path) -> rbtdre_Verdict {
    // An express bole touchmark decodes fine but is the wrong kind — yoke
    // requires a reliquary. Rejects before auth and the fan-out write.
    match rbtdrh_drive_yoke(dir, RBTDRH_BOLE_TOUCHMARK) {
        Ok(code) => rbtdrh_expect_band(code, "yoke wrong-kind (bole express)", dir),
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrh_yoke_unknown_prefix(dir: &Path) -> rbtdre_Verdict {
    match rbtdrh_drive_yoke(dir, RBTDRH_UNKNOWN_TOUCHMARK) {
        Ok(code) => rbtdrh_expect_band(code, "yoke unknown-prefix express", dir),
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

// ── Fixture ─────────────────────────────────────────────────

pub static RBTDRH_CASES_CHAINING_FACT_BAND: &[rbtdre_Case] = &[
    case!(rbtdrh_feoff_wrong_kind),
    case!(rbtdrh_feoff_unknown_prefix),
    case!(rbtdrh_feoff_broken_chain),
    case!(rbtdrh_feoff_good),
    case!(rbtdrh_feoff_precedence),
    case!(rbtdrh_feoff_fact_intact),
    case!(rbtdrh_yoke_wrong_kind),
    case!(rbtdrh_yoke_unknown_prefix),
];

pub static RBTDRH_FIXTURE_CHAINING_FACT_BAND: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CHAINING_FACT_BAND,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRH_CASES_CHAINING_FACT_BAND,
    credless: true,
};
