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
// RBTDRS — regime-poison fixture: in-universe negative validation.
//
// Each case drives the REAL validate verb against a real regime (in-tree
// tracked baseline, or a staged synthetic one for operator-local regimes) with
// exactly one field corrupted via the regime-poison tweak — zbuv_poison_apply,
// the single BUK regime-load membrane crossed at every regime kindle — and
// asserts the SPECIFIC band code of the gate that rejects:
//
//   RBTGEB_BUSX_REGIME_DENTIL (100)  a regime module's own enforce rule fired
//                             (cross-field, format regex, existence)
//   RBTGEB_BUSX_ENROLL_DENTIL (101)  the buv enrollment pipeline rejected
//                             (buv_vet type/format/enum/range/presence, or
//                              buv_scope_sentinel on an unexpected variable)
//
// Asserting the band — not bare nonzero — closes the wrong-layer hole: a
// harness breakage (unbound variable, missing file, refactor typo) exits with
// some other code and fails the case loud, where a bare-nonzero assertion would
// pass on it.
//
// NOT credless, so it cannot ride reveille (whose single tweak slot belongs to
// the credless guard). The per-case poison occupies that slot, so the fixture
// enrolls in picket/bivouac/echelon — see RBTCEA_SUITES.

use std::path::Path;

use crate::rbtcee_case;
use crate::rbtgec_consts::{
    BUWGC_RC_VALIDATE,
    BUWGC_RS_VALIDATE,
    RBTGEC_TWEAK_REGIME_POISON,
    RBTGEC_VALIDATE_DEPOT,
    RBTGEC_VALIDATE_NAMEPLATE,
    RBTGEC_VALIDATE_OAUTH,
    RBTGEC_VALIDATE_PAYOR,
    RBTGEC_VALIDATE_REPO,
    RBTGEC_VALIDATE_VESSEL,
};
use crate::rbtgeb_breviary::{
    RBTGEB_BUSX_ENROLL_DENTIL,
    RBTGEB_BUSX_REGIME_DENTIL,
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
    rbtdri_rooted_tabtarget_command,
    RBTCEI_BURE_TWEAK_NAME_KEY,
    RBTCEI_BURE_TWEAK_VALUE_KEY,
};
use crate::rbtcem_manifest::RBTCEM_FIXTURE_REGIME_POISON;

// Regime variable names referenced by 2+ poison specs — single definition so a
// typo cannot silently poison a different field than intended.
const RBTCHS_VAR_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";
const RBTCHS_VAR_RBRD_CLOUD_PREFIX: &str = "RBRD_CLOUD_PREFIX";
const RBTCHS_VAR_RBRD_DEPOT_MONIKER: &str = "RBRD_DEPOT_MONIKER";

// Folio monikers referenced by 2+ cases — a known-good entry-enabled nameplate
// and known-good conjure vessels, all in-tree. Removing any fails the cases loud
// (the verb cannot locate the regime), not silently.
const RBTCHS_NAMEPLATE_TADMOR: &str = "tadmor";
const RBTCHS_VESSEL_BUSYBOX: &str = "rbev_busybox";
const RBTCHS_VESSEL_PLANTUML: &str = "rbev_bottle_plantuml";

// ── Poison harness ──────────────────────────────────────────

/// Drive a validate tabtarget under the regime-poison tweak, asserting the exit
/// equals `expected_band`. `folio` is the verb's positional args (empty for
/// repo/depot/payor; the nameplate/vessel moniker otherwise). `poison` is the
/// BURE_TWEAK_VALUE: "VAR=value" to corrupt a field, bare "VAR" to unset a
/// required one. The corrupted VAR must carry the regime's enroll scope prefix,
/// or the seam rides inert (zbuv_poison_apply's scope guard). The poison rides
/// BURE_TWEAK_NAME + BURE_TWEAK_VALUE as extra env on the one tabtarget-launch
/// constructor; this fixture is not credless, so the slot is free (the
/// rbtcei_invoke conflict gate fires only under the credless guard).
fn rbtchs_poison(
    dir: &Path,
    validate_colophon: &str,
    folio: &[&str],
    poison: &str,
    expected_band: i32,
    label: &str,
) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let tt = match rbtcei_find_tabtarget_global(&root, validate_colophon) {
        Ok(p) => p,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    let mut cmd = match rbtdri_rooted_tabtarget_command(&tt, &dir.join(label)) {
        Ok(c) => c,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    let output = match cmd
        .args(folio)
        .env(RBTCEI_BURE_TWEAK_NAME_KEY, RBTGEC_TWEAK_REGIME_POISON)
        .env(RBTCEI_BURE_TWEAK_VALUE_KEY, poison)
        .current_dir(&root)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!("{}: failed to run {}: {}", label, tt.display(), e));
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let code = output.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &stderr);
    if code != expected_band {
        return rbtcee_Verdict::Fail(format!(
            "{}: {} under poison '{}' exited {} — expected band {}\nstdout:\n{}\n\nstderr:\n{}",
            label, validate_colophon, poison, code, expected_band, stdout, stderr
        ));
    }
    rbtcee_Verdict::Pass
}

// ── RBRR (repo) — verb rbw-rrv against the tracked rbrr.env ──

fn rbtchs_rbrr_bad_timeout(dir: &Path) -> rbtcee_Verdict {
    // RBRR_GCB_TIMEOUT enrolls as a plain string; the NNNs format is a
    // zrbrr_enforce regex, so a non-NNNs value rejects in the module → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[], "RBRR_GCB_TIMEOUT=1200",
        RBTGEB_BUSX_REGIME_DENTIL, "rbrr-bad-timeout")
}

fn rbtchs_rbrr_unexpected_var(dir: &Path) -> rbtcee_Verdict {
    // An unenrolled RBRR_* variable trips buv_scope_sentinel → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[], "RBRR_BOGUS=foo",
        RBTGEB_BUSX_ENROLL_DENTIL, "rbrr-unexpected-var")
}

fn rbtchs_rbrr_bad_vessel_dir(dir: &Path) -> rbtcee_Verdict {
    // A nonexistent directory fails the zrbrr_enforce existence check → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[],
        "RBRR_VESSEL_DIR=/tmp/nonexistent-rbtdrs-vessel-dir",
        RBTGEB_BUSX_REGIME_DENTIL, "rbrr-bad-vessel-dir")
}

fn rbtchs_rbrr_bad_secrets_dir(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[],
        "RBRR_SECRETS_DIR=/tmp/nonexistent-rbtdrs-secrets-dir",
        RBTGEB_BUSX_REGIME_DENTIL, "rbrr-bad-secrets-dir")
}

fn rbtchs_rbrr_bad_runtime_prefix_uppercase(dir: &Path) -> rbtcee_Verdict {
    // Valid length, so it clears the buv enroll; uppercase fails the
    // zrbrr_enforce format regex → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[],
        &format!("{}=BAD-", RBTCHS_VAR_RBRR_RUNTIME_PREFIX),
        RBTGEB_BUSX_REGIME_DENTIL, "rbrr-bad-runtime-prefix-uppercase")
}

fn rbtchs_rbrr_bad_runtime_prefix_no_trailing_hyphen(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[],
        &format!("{}=acme", RBTCHS_VAR_RBRR_RUNTIME_PREFIX),
        RBTGEB_BUSX_REGIME_DENTIL, "rbrr-bad-runtime-prefix-no-trailing-hyphen")
}

fn rbtchs_rbrr_bad_runtime_prefix_too_long(dir: &Path) -> rbtcee_Verdict {
    // 12 chars exceeds the buv_string_enroll max (11) → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[],
        &format!("{}=twelvechars-", RBTCHS_VAR_RBRR_RUNTIME_PREFIX),
        RBTGEB_BUSX_ENROLL_DENTIL, "rbrr-bad-runtime-prefix-too-long")
}

fn rbtchs_rbrr_bad_substrate_reliquary(dir: &Path) -> rbtcee_Verdict {
    // Legal-empty, so any non-empty value runs the zrbrr_enforce format check;
    // a non-touchmark shape (not r + 12 digits) clears the buv enroll (≤14 chars)
    // but fails the format regex → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_REPO, &[],
        "RBRR_SUBSTRATE_RELIQUARY=not-a-relic",
        RBTGEB_BUSX_REGIME_DENTIL, "rbrr-bad-substrate-reliquary")
}

// ── RBRD (depot) — verb rbw-rdv against the tracked rbrd.env ─

fn rbtchs_rbrd_missing_moniker(dir: &Path) -> rbtcee_Verdict {
    // Unset a required field → buv presence check → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_DEPOT, &[], RBTCHS_VAR_RBRD_DEPOT_MONIKER,
        RBTGEB_BUSX_ENROLL_DENTIL, "rbrd-missing-moniker")
}

fn rbtchs_rbrd_bad_moniker(dir: &Path) -> rbtcee_Verdict {
    // Uppercase + hyphen fails the zrbrd_enforce moniker regex → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_DEPOT, &[],
        &format!("{}=BAD-MONIKER", RBTCHS_VAR_RBRD_DEPOT_MONIKER),
        RBTGEB_BUSX_REGIME_DENTIL, "rbrd-bad-moniker")
}

fn rbtchs_rbrd_bad_cloud_prefix_uppercase(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_DEPOT, &[],
        &format!("{}=BAD-", RBTCHS_VAR_RBRD_CLOUD_PREFIX),
        RBTGEB_BUSX_REGIME_DENTIL, "rbrd-bad-cloud-prefix-uppercase")
}

fn rbtchs_rbrd_bad_cloud_prefix_no_trailing_hyphen(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_DEPOT, &[],
        &format!("{}=acme", RBTCHS_VAR_RBRD_CLOUD_PREFIX),
        RBTGEB_BUSX_REGIME_DENTIL, "rbrd-bad-cloud-prefix-no-trailing-hyphen")
}

fn rbtchs_rbrd_bad_cloud_prefix_too_long(dir: &Path) -> rbtcee_Verdict {
    // 12 chars exceeds the buv_string_enroll max (11) → enroll, before the
    // joint-length enforce ever runs.
    rbtchs_poison(dir, RBTGEC_VALIDATE_DEPOT, &[],
        &format!("{}=twelvechars-", RBTCHS_VAR_RBRD_CLOUD_PREFIX),
        RBTGEB_BUSX_ENROLL_DENTIL, "rbrd-bad-cloud-prefix-too-long")
}

// ── RBRP (payor) — verb rbw-rpv against the tracked rbrp.env ─

fn rbtchs_rbrp_bad_payor_project(dir: &Path) -> rbtcee_Verdict {
    // Valid length, so it clears the buv enroll; fails the zrbrp_enforce
    // payor-project regex → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_PAYOR, &[],
        "RBRP_PAYOR_PROJECT_ID=not-a-payor-project",
        RBTGEB_BUSX_REGIME_DENTIL, "rbrp-bad-payor-project")
}

// ── BURC (config) — verb buw-rcv against the tracked burc.env ─

fn rbtchs_burc_missing_station_file(dir: &Path) -> rbtcee_Verdict {
    // Unset a required field → buv presence check → enroll.
    rbtchs_poison(dir, BUWGC_RC_VALIDATE, &[], "BURC_STATION_FILE",
        RBTGEB_BUSX_ENROLL_DENTIL, "burc-missing-station-file")
}

// ── RBRN (nameplate) — verb rbw-rnv against a real nameplate ─
//
// Folio is an entry-enabled nameplate; all eight cases poison one field of its
// real regime. Seven reject in the buv pipeline (enum, ipv4, presence,
// sentinel); port-conflict is the lone module-enforce case.

fn rbtchs_rbrn_missing_moniker(dir: &Path) -> rbtcee_Verdict {
    // Unset a required field → buv presence check → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_MONIKER", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-missing-moniker")
}

fn rbtchs_rbrn_invalid_runtime(dir: &Path) -> rbtcee_Verdict {
    // Off-enum value fails the buv_enum_enroll check → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_RUNTIME=invalid", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-invalid-runtime")
}

fn rbtchs_rbrn_invalid_entry_mode(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_ENTRY_MODE=bogus", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-invalid-entry-mode")
}

fn rbtchs_rbrn_invalid_dns_mode(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_UPLINK_DNS_MODE=bogus", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-invalid-dns-mode")
}

fn rbtchs_rbrn_invalid_access_mode(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_UPLINK_ACCESS_MODE=bogus", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-invalid-access-mode")
}

fn rbtchs_rbrn_bad_ip(dir: &Path) -> rbtcee_Verdict {
    // Malformed address fails the buv_ipv4_enroll check → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_ENCLAVE_BASE_IP=not-an-ip", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-bad-ip")
}

fn rbtchs_rbrn_unexpected_var(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_BOGUS=foo", RBTGEB_BUSX_ENROLL_DENTIL, "rbrn-unexpected-var")
}

fn rbtchs_rbrn_port_conflict(dir: &Path) -> rbtcee_Verdict {
    // A workstation port at/above the uplink minimum (a valid port number, so it
    // clears the buv enroll) trips the zrbrn_enforce cross-port check → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_NAMEPLATE, &[RBTCHS_NAMEPLATE_TADMOR],
        "RBRN_ENTRY_PORT_WORKSTATION=10001", RBTGEB_BUSX_REGIME_DENTIL, "rbrn-port-conflict")
}

// ── RBRV (vessel) — verb rbw-rvv against a real vessel ───────
//
// Mode-specific: the conjure cases use a conjure vessel, the bind case a bind
// vessel, so the gated field being poisoned is active. Folio is the vessel
// moniker (= RBRV_SIGIL).

fn rbtchs_rbrv_missing_sigil(dir: &Path) -> rbtcee_Verdict {
    // Unset a required field → buv presence check → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_VESSEL, &[RBTCHS_VESSEL_BUSYBOX],
        "RBRV_SIGIL", RBTGEB_BUSX_ENROLL_DENTIL, "rbrv-missing-sigil")
}

fn rbtchs_rbrv_unexpected_var(dir: &Path) -> rbtcee_Verdict {
    rbtchs_poison(dir, RBTGEC_VALIDATE_VESSEL, &[RBTCHS_VESSEL_BUSYBOX],
        "RBRV_BOGUS=foo", RBTGEB_BUSX_ENROLL_DENTIL, "rbrv-unexpected-var")
}

fn rbtchs_rbrv_partial_conjure(dir: &Path) -> rbtcee_Verdict {
    // Unset a conjure-gated required field on a conjure vessel → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_VESSEL, &[RBTCHS_VESSEL_BUSYBOX],
        "RBRV_CONJURE_PLATFORMS", RBTGEB_BUSX_ENROLL_DENTIL, "rbrv-partial-conjure")
}

fn rbtchs_rbrv_no_bind_image(dir: &Path) -> rbtcee_Verdict {
    // Unset the bind-gated required field on a bind vessel → enroll.
    rbtchs_poison(dir, RBTGEC_VALIDATE_VESSEL, &[RBTCHS_VESSEL_PLANTUML],
        "RBRV_BIND_IMAGE", RBTGEB_BUSX_ENROLL_DENTIL, "rbrv-no-bind-image")
}

fn rbtchs_rbrv_bind_image_tag_only(dir: &Path) -> rbtcee_Verdict {
    // A tag-pinned FQIN clears the buv_fqin enroll but lacks the @sha256:<64-hex>
    // digest, tripping the zrbrv_enforce bind-digest check → regime.
    rbtchs_poison(dir, RBTGEC_VALIDATE_VESSEL, &[RBTCHS_VESSEL_PLANTUML],
        "RBRV_BIND_IMAGE=docker.io/plantuml/plantuml-server:1.2024.7",
        RBTGEB_BUSX_REGIME_DENTIL, "rbrv-bind-image-tag-only")
}

// ── Operator-local regimes — station, oauth, auth ──
//
// These regimes have no in-tree baseline: their files live in the operator's
// station tree, present only on a configured workstation. Each case probes the
// baseline verb un-poisoned first — if it is not green the regime is not
// configured here and the case self-skips (the regime-smoke station precedent);
// when green, the baseline run also proves the poison is the only variable, then
// the poisoned run asserts the band. The poison corrupts an in-memory variable
// in the validate subshell after the file is sourced — it never modifies the
// operator's regime file, so even secret-bearing oauth/auth config is untouched.

/// Operator-local variant of rbtchs_poison: self-skip when the baseline verb is
/// not green (regime absent on this machine), else assert the poisoned band.
fn rbtchs_optional(
    dir: &Path,
    validate_colophon: &str,
    folio: &[&str],
    poison: &str,
    expected_band: i32,
    label: &str,
) -> rbtcee_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtcee_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let tt = match rbtcei_find_tabtarget_global(&root, validate_colophon) {
        Ok(p) => p,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    let mut baseline_cmd = match rbtdri_rooted_tabtarget_command(&tt, &dir.join(format!("{}-baseline", label))) {
        Ok(c) => c,
        Err(e) => return rbtcee_Verdict::Fail(e),
    };
    let baseline = match baseline_cmd
        .args(folio)
        .current_dir(&root)
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return rbtcee_Verdict::Fail(format!("{}: failed to run {}: {}", label, tt.display(), e));
        }
    };
    let base_code = baseline.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join(format!("{}-baseline-stdout.txt", label)), &baseline.stdout);
    let _ = std::fs::write(dir.join(format!("{}-baseline-stderr.txt", label)), &baseline.stderr);
    if base_code != 0 {
        return rbtcee_Verdict::Skip(format!(
            "{}: {} baseline not green (exit {}) — operator-local regime not configured here",
            label, validate_colophon, base_code
        ));
    }
    rbtchs_poison(dir, validate_colophon, folio, poison, expected_band, label)
}

fn rbtchs_rbro_missing_refresh_token(dir: &Path) -> rbtcee_Verdict {
    rbtchs_optional(dir, RBTGEC_VALIDATE_OAUTH, &[], "RBRO_REFRESH_TOKEN",
        RBTGEB_BUSX_ENROLL_DENTIL, "rbro-missing-refresh-token")
}

fn rbtchs_burs_bad_tincture(dir: &Path) -> rbtcee_Verdict {
    // Uppercase clears the length enroll but fails the zburs_enforce regex → regime.
    rbtchs_optional(dir, BUWGC_RS_VALIDATE, &[], "BURS_TINCTURE=A1",
        RBTGEB_BUSX_REGIME_DENTIL, "burs-bad-tincture")
}

// ── Fixture ─────────────────────────────────────────────────

pub static RBTCHS_CASES_REGIME_POISON: &[rbtcee_Case] = &[
    rbtcee_case!(rbtchs_rbrr_bad_timeout),
    rbtcee_case!(rbtchs_rbrr_unexpected_var),
    rbtcee_case!(rbtchs_rbrr_bad_vessel_dir),
    rbtcee_case!(rbtchs_rbrr_bad_secrets_dir),
    rbtcee_case!(rbtchs_rbrr_bad_runtime_prefix_uppercase),
    rbtcee_case!(rbtchs_rbrr_bad_runtime_prefix_no_trailing_hyphen),
    rbtcee_case!(rbtchs_rbrr_bad_runtime_prefix_too_long),
    rbtcee_case!(rbtchs_rbrr_bad_substrate_reliquary),
    rbtcee_case!(rbtchs_rbrd_missing_moniker),
    rbtcee_case!(rbtchs_rbrd_bad_moniker),
    rbtcee_case!(rbtchs_rbrd_bad_cloud_prefix_uppercase),
    rbtcee_case!(rbtchs_rbrd_bad_cloud_prefix_no_trailing_hyphen),
    rbtcee_case!(rbtchs_rbrd_bad_cloud_prefix_too_long),
    rbtcee_case!(rbtchs_rbrp_bad_payor_project),
    rbtcee_case!(rbtchs_burc_missing_station_file),
    rbtcee_case!(rbtchs_rbrn_missing_moniker),
    rbtcee_case!(rbtchs_rbrn_invalid_runtime),
    rbtcee_case!(rbtchs_rbrn_invalid_entry_mode),
    rbtcee_case!(rbtchs_rbrn_invalid_dns_mode),
    rbtcee_case!(rbtchs_rbrn_invalid_access_mode),
    rbtcee_case!(rbtchs_rbrn_bad_ip),
    rbtcee_case!(rbtchs_rbrn_unexpected_var),
    rbtcee_case!(rbtchs_rbrn_port_conflict),
    rbtcee_case!(rbtchs_rbrv_missing_sigil),
    rbtcee_case!(rbtchs_rbrv_unexpected_var),
    rbtcee_case!(rbtchs_rbrv_partial_conjure),
    rbtcee_case!(rbtchs_rbrv_no_bind_image),
    rbtcee_case!(rbtchs_rbrv_bind_image_tag_only),
    rbtcee_case!(rbtchs_rbro_missing_refresh_token),
    rbtcee_case!(rbtchs_burs_bad_tincture),
];

pub static RBTCHS_FIXTURE_REGIME_POISON: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTCEM_FIXTURE_REGIME_POISON,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTCHS_CASES_REGIME_POISON,
    credless: false,
    tariff: rbtcee_Tariff::RBTCEE_UNCHECKED,
};
