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
// RBTDRS — regime-poison fixture: in-universe negative validation.
//
// Each case drives the REAL validate verb against a real regime (in-tree
// tracked baseline, or a staged synthetic one for operator-local regimes) with
// exactly one field corrupted via the regime-poison tweak — zbuv_poison_apply,
// the single BUK regime-load membrane crossed at every regime kindle — and
// asserts the SPECIFIC band code of the gate that rejects:
//
//   RBTDGC_BAND_REGIME (100)  a regime module's own enforce rule fired
//                             (cross-field, format regex, existence)
//   RBTDGC_BAND_ENROLL (101)  the buv enrollment pipeline rejected
//                             (buv_vet type/format/enum/range/presence, or
//                              buv_scope_sentinel on an unexpected variable)
//
// Asserting the band — not bare nonzero — closes the wrong-layer hole: a
// harness breakage (unbound variable, missing file, refactor typo) exits with
// some other code and fails the case loud, where a bare-nonzero assertion would
// pass on it.
//
// NOT credless, so it cannot ride fast (whose single tweak slot belongs to the
// credless guard). The per-case poison occupies that slot, so the fixture
// enrolls in service/crucible/complete — see RBTDRC_SUITES.

use std::path::Path;

use crate::case;
use crate::rbtdgc_consts::{
    RBTDGC_BAND_ENROLL,
    RBTDGC_BAND_REGIME,
    RBTDGC_TWEAK_REGIME_POISON,
    RBTDGC_VALIDATE_DEPOT,
    RBTDGC_VALIDATE_PAYOR,
    RBTDGC_VALIDATE_REPO,
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
    RBTDRI_BURE_TWEAK_NAME_KEY,
    RBTDRI_BURE_TWEAK_VALUE_KEY,
};
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_REGIME_POISON;

// Regime variable names referenced by 2+ poison specs — single definition so a
// typo cannot silently poison a different field than intended.
const RBTDRS_VAR_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";
const RBTDRS_VAR_RBRD_CLOUD_PREFIX: &str = "RBRD_CLOUD_PREFIX";
const RBTDRS_VAR_RBRD_DEPOT_MONIKER: &str = "RBRD_DEPOT_MONIKER";

// BUK validate colophons — not projected into RBTDGC_* (those carry the rbw-*
// RB colophons only), so the buw-* BUK colophons are named here.
const RBTDRS_VALIDATE_BURC: &str = "buw-rcv";

// ── Poison harness ──────────────────────────────────────────

/// Drive a validate tabtarget under the regime-poison tweak, asserting the exit
/// equals `expected_band`. `folio` is the verb's positional args (empty for
/// repo/depot/payor; the nameplate/vessel moniker otherwise). `poison` is the
/// BURE_TWEAK_VALUE: "VAR=value" to corrupt a field, bare "VAR" to unset a
/// required one. The corrupted VAR must carry the regime's enroll scope prefix,
/// or the seam rides inert (zbuv_poison_apply's scope guard). The poison rides
/// BURE_TWEAK_NAME + BURE_TWEAK_VALUE as extra env on the one tabtarget-launch
/// constructor; this fixture is not credless, so the slot is free (the
/// rbtdri_invoke conflict gate fires only under the credless guard).
fn rbtdrs_poison(
    dir: &Path,
    validate_colophon: &str,
    folio: &[&str],
    poison: &str,
    expected_band: i32,
    label: &str,
) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let tt = match rbtdri_find_tabtarget_global(&root, validate_colophon) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(e),
    };
    let output = match rbtdri_tabtarget_command(&tt)
        .args(folio)
        .env(RBTDRI_BURE_TWEAK_NAME_KEY, RBTDGC_TWEAK_REGIME_POISON)
        .env(RBTDRI_BURE_TWEAK_VALUE_KEY, poison)
        .current_dir(&root)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!("{}: failed to run {}: {}", label, tt.display(), e));
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let code = output.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &stderr);
    if code != expected_band {
        return rbtdre_Verdict::Fail(format!(
            "{}: {} under poison '{}' exited {} — expected band {}\nstdout:\n{}\n\nstderr:\n{}",
            label, validate_colophon, poison, code, expected_band, stdout, stderr
        ));
    }
    rbtdre_Verdict::Pass
}

// ── RBRR (repo) — verb rbw-rrv against the tracked rbrr.env ──

fn rbtdrs_rbrr_bad_timeout(dir: &Path) -> rbtdre_Verdict {
    // RBRR_GCB_TIMEOUT enrolls as a plain string; the NNNs format is a
    // zrbrr_enforce regex, so a non-NNNs value rejects in the module → regime.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[], "RBRR_GCB_TIMEOUT=1200",
        RBTDGC_BAND_REGIME, "rbrr-bad-timeout")
}

fn rbtdrs_rbrr_unexpected_var(dir: &Path) -> rbtdre_Verdict {
    // An unenrolled RBRR_* variable trips buv_scope_sentinel → enroll.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[], "RBRR_BOGUS=foo",
        RBTDGC_BAND_ENROLL, "rbrr-unexpected-var")
}

fn rbtdrs_rbrr_bad_vessel_dir(dir: &Path) -> rbtdre_Verdict {
    // A nonexistent directory fails the zrbrr_enforce existence check → regime.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[],
        "RBRR_VESSEL_DIR=/tmp/nonexistent-rbtdrs-vessel-dir",
        RBTDGC_BAND_REGIME, "rbrr-bad-vessel-dir")
}

fn rbtdrs_rbrr_bad_secrets_dir(dir: &Path) -> rbtdre_Verdict {
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[],
        "RBRR_SECRETS_DIR=/tmp/nonexistent-rbtdrs-secrets-dir",
        RBTDGC_BAND_REGIME, "rbrr-bad-secrets-dir")
}

fn rbtdrs_rbrr_bad_runtime_prefix_uppercase(dir: &Path) -> rbtdre_Verdict {
    // Valid length, so it clears the buv enroll; uppercase fails the
    // zrbrr_enforce format regex → regime.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[],
        &format!("{}=BAD-", RBTDRS_VAR_RBRR_RUNTIME_PREFIX),
        RBTDGC_BAND_REGIME, "rbrr-bad-runtime-prefix-uppercase")
}

fn rbtdrs_rbrr_bad_runtime_prefix_no_trailing_hyphen(dir: &Path) -> rbtdre_Verdict {
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[],
        &format!("{}=acme", RBTDRS_VAR_RBRR_RUNTIME_PREFIX),
        RBTDGC_BAND_REGIME, "rbrr-bad-runtime-prefix-no-trailing-hyphen")
}

fn rbtdrs_rbrr_bad_runtime_prefix_too_long(dir: &Path) -> rbtdre_Verdict {
    // 12 chars exceeds the buv_string_enroll max (11) → enroll.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_REPO, &[],
        &format!("{}=twelvechars-", RBTDRS_VAR_RBRR_RUNTIME_PREFIX),
        RBTDGC_BAND_ENROLL, "rbrr-bad-runtime-prefix-too-long")
}

// ── RBRD (depot) — verb rbw-rdv against the tracked rbrd.env ─

fn rbtdrs_rbrd_missing_moniker(dir: &Path) -> rbtdre_Verdict {
    // Unset a required field → buv presence check → enroll.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_DEPOT, &[], RBTDRS_VAR_RBRD_DEPOT_MONIKER,
        RBTDGC_BAND_ENROLL, "rbrd-missing-moniker")
}

fn rbtdrs_rbrd_bad_moniker(dir: &Path) -> rbtdre_Verdict {
    // Uppercase + hyphen fails the zrbrd_enforce moniker regex → regime.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_DEPOT, &[],
        &format!("{}=BAD-MONIKER", RBTDRS_VAR_RBRD_DEPOT_MONIKER),
        RBTDGC_BAND_REGIME, "rbrd-bad-moniker")
}

fn rbtdrs_rbrd_bad_cloud_prefix_uppercase(dir: &Path) -> rbtdre_Verdict {
    rbtdrs_poison(dir, RBTDGC_VALIDATE_DEPOT, &[],
        &format!("{}=BAD-", RBTDRS_VAR_RBRD_CLOUD_PREFIX),
        RBTDGC_BAND_REGIME, "rbrd-bad-cloud-prefix-uppercase")
}

fn rbtdrs_rbrd_bad_cloud_prefix_no_trailing_hyphen(dir: &Path) -> rbtdre_Verdict {
    rbtdrs_poison(dir, RBTDGC_VALIDATE_DEPOT, &[],
        &format!("{}=acme", RBTDRS_VAR_RBRD_CLOUD_PREFIX),
        RBTDGC_BAND_REGIME, "rbrd-bad-cloud-prefix-no-trailing-hyphen")
}

fn rbtdrs_rbrd_bad_cloud_prefix_too_long(dir: &Path) -> rbtdre_Verdict {
    // 12 chars exceeds the buv_string_enroll max (11) → enroll, before the
    // joint-length enforce ever runs.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_DEPOT, &[],
        &format!("{}=twelvechars-", RBTDRS_VAR_RBRD_CLOUD_PREFIX),
        RBTDGC_BAND_ENROLL, "rbrd-bad-cloud-prefix-too-long")
}

// ── RBRP (payor) — verb rbw-rpv against the tracked rbrp.env ─

fn rbtdrs_rbrp_bad_payor_project(dir: &Path) -> rbtdre_Verdict {
    // Valid length, so it clears the buv enroll; fails the zrbrp_enforce
    // payor-project regex → regime.
    rbtdrs_poison(dir, RBTDGC_VALIDATE_PAYOR, &[],
        "RBRP_PAYOR_PROJECT_ID=not-a-payor-project",
        RBTDGC_BAND_REGIME, "rbrp-bad-payor-project")
}

// ── BURC (config) — verb buw-rcv against the tracked burc.env ─

fn rbtdrs_burc_missing_station_file(dir: &Path) -> rbtdre_Verdict {
    // Unset a required field → buv presence check → enroll.
    rbtdrs_poison(dir, RBTDRS_VALIDATE_BURC, &[], "BURC_STATION_FILE",
        RBTDGC_BAND_ENROLL, "burc-missing-station-file")
}

// ── Fixture ─────────────────────────────────────────────────

pub static RBTDRS_CASES_REGIME_POISON: &[rbtdre_Case] = &[
    case!(rbtdrs_rbrr_bad_timeout),
    case!(rbtdrs_rbrr_unexpected_var),
    case!(rbtdrs_rbrr_bad_vessel_dir),
    case!(rbtdrs_rbrr_bad_secrets_dir),
    case!(rbtdrs_rbrr_bad_runtime_prefix_uppercase),
    case!(rbtdrs_rbrr_bad_runtime_prefix_no_trailing_hyphen),
    case!(rbtdrs_rbrr_bad_runtime_prefix_too_long),
    case!(rbtdrs_rbrd_missing_moniker),
    case!(rbtdrs_rbrd_bad_moniker),
    case!(rbtdrs_rbrd_bad_cloud_prefix_uppercase),
    case!(rbtdrs_rbrd_bad_cloud_prefix_no_trailing_hyphen),
    case!(rbtdrs_rbrd_bad_cloud_prefix_too_long),
    case!(rbtdrs_rbrp_bad_payor_project),
    case!(rbtdrs_burc_missing_station_file),
];

pub static RBTDRS_FIXTURE_REGIME_POISON: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_REGIME_POISON,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRS_CASES_REGIME_POISON,
    credless: false,
};
