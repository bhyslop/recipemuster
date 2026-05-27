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
// RBTDRM — colophon manifest verification for theurge

// Colophon names are projected from the zipper registry into the generated
// RBTDGC_* consts (rbtdgc_consts.rs). This module consumes them for the
// per-fixture required-colophon manifest and the role→probe mapping; the
// runtime drift check (rbtdrm_verify) is retained until the build-time diff
// gate lands.
use crate::rbtdgc_consts::*;

// Credential role consts — the credential vocabulary that crosses the
// access-probe CLI boundary (role → colophon mapping, fact-file labels,
// diagnostics). Single definition per String Boundary Discipline.
pub const RBTDRM_ROLE_GOVERNOR: &str = "governor";
pub const RBTDRM_ROLE_RETRIEVER: &str = "retriever";
pub const RBTDRM_ROLE_DIRECTOR: &str = "director";
pub const RBTDRM_ROLE_PAYOR: &str = "payor";

/// Map a credential role to its access-probe colophon. Returns None for
/// unknown roles. Replaces the former role-as-imprint scheme: each role now
/// names its own global tabtarget under the rbw-ac* family.
pub fn rbtdrm_credential_check_colophon(role: &str) -> Option<&'static str> {
    match role {
        RBTDRM_ROLE_GOVERNOR => Some(RBTDGC_CHECK_GOVERNOR),
        RBTDRM_ROLE_RETRIEVER => Some(RBTDGC_CHECK_RETRIEVER),
        RBTDRM_ROLE_DIRECTOR => Some(RBTDGC_CHECK_DIRECTOR),
        RBTDRM_ROLE_PAYOR => Some(RBTDGC_CHECK_PAYOR),
        _ => None,
    }
}

// Fixture name consts — single definition per String Boundary Discipline.
// Crucible fixtures (charge/quench lifecycle)
pub const RBTDRM_FIXTURE_TADMOR: &str = "tadmor";
pub const RBTDRM_FIXTURE_MORIAH: &str = "moriah";
pub const RBTDRM_FIXTURE_SRJCL: &str = "srjcl";
pub const RBTDRM_FIXTURE_PLUML: &str = "pluml";
// Bare fixtures (GCP credentials, no container runtime)
pub const RBTDRM_FIXTURE_HALLMARK_LIFECYCLE: &str = "hallmark-lifecycle";
pub const RBTDRM_FIXTURE_BATCH_VOUCH: &str = "batch-vouch";
pub const RBTDRM_FIXTURE_ACCESS_PROBE: &str = "access-probe";
// Fast fixtures (no external dependencies)
pub const RBTDRM_FIXTURE_ENROLLMENT_VALIDATION: &str = "enrollment-validation";
pub const RBTDRM_FIXTURE_REGIME_VALIDATION: &str = "regime-validation";
pub const RBTDRM_FIXTURE_REGIME_SMOKE: &str = "regime-smoke";
pub const RBTDRM_FIXTURE_HANDBOOK_RENDER: &str = "handbook-render";
pub const RBTDRM_FIXTURE_DOCKERFILE_HYGIENE: &str = "dockerfile-hygiene";
// Pristine-lifecycle fixture (gate + SA/depot lifecycle cases)
pub const RBTDRM_FIXTURE_PRISTINE_LIFECYCLE: &str = "pristine-lifecycle";
// Gauntlet canonical-establish fixture (§2: canonical depot levy + governor
// mantle + retriever/director invest with per-case precondition probes)
pub const RBTDRM_FIXTURE_CANONICAL_ESTABLISH: &str = "canonical-establish";
// Skirmish canonical-invest fixture — the no-levy variant of
// canonical-establish. Reuses the three investiture cases (governor mantle +
// retriever/director invest) against a depot the operator has already levied
// by hand; omits depot-levy so the skirmish suite creates no GCP project per
// run. Distinction from canonical-establish is precondition, not behavior.
pub const RBTDRM_FIXTURE_CANONICAL_INVEST: &str = "canonical-invest";
// Gauntlet onboarding-sequence fixture (§3: handbook-walked vessel
// construction — inscribe reliquary, enshrine bases, kludge tadmor/ccyolo,
// plus one ordain-* case per director-mode handbook track, build-only)
pub const RBTDRM_FIXTURE_ONBOARDING_SEQUENCE: &str = "onboarding-sequence";
// Self-contained tadmor build fixture (rbw-ts.TestSuite.tadmor): kludges tadmor sentry+bottle
// locally and commits each hallmark (so the subsequent tadmor crucible fixture
// charges against a clean nameplate). Reuses onboarding's kludge helper minus
// its reliquary-stamp witness probe — local kludge has no GCP/reliquary dep.
pub const RBTDRM_FIXTURE_KLUDGE_TADMOR: &str = "kludge-tadmor";
// Dogfight cloud-build viability fixture — standing-depot sibling to
// canonical-invest, proving the cloud-build → summon → run path yields a
// runnable artifact with NO crucible charged (the orthogonal axis skirmish
// covers). Ordains conjure-mode busybox, summons it, runs a degenerate
// container-runtime command proving executability, then abjures.
pub const RBTDRM_FIXTURE_DOGFIGHT: &str = "dogfight";
// Calibrant fixtures — synthetic deterministic-verdict fixtures driving the
// bash blackbox testbench. Internal framework-test plumbing.
pub const RBTDRM_FIXTURE_CALIBRANT_VERDICTS: &str = "calibrant-verdicts";
pub const RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST: &str = "calibrant-fail-fast";
pub const RBTDRM_FIXTURE_CALIBRANT_PROGRESSING: &str = "calibrant-progressing";
pub const RBTDRM_FIXTURE_CALIBRANT_SENTINEL: &str = "calibrant-sentinel";

// Operation verb consts — single definition per String Boundary Discipline.
// Used for error-message prefixes, log labels, and commit messages where the
// operation name is a vocabulary unit. Atomic — compound operations like
// "kludge sentry" are composed at the call site from RBTDRM_OPERATION_KLUDGE
// and the relevant container-role constant.
pub const RBTDRM_OPERATION_INSCRIBE: &str = "inscribe";
pub const RBTDRM_OPERATION_YOKE: &str = "yoke";
pub const RBTDRM_OPERATION_ORDAIN: &str = "ordain";
pub const RBTDRM_OPERATION_ENSHRINE: &str = "enshrine";
pub const RBTDRM_OPERATION_KLUDGE: &str = "kludge";

// Container role consts — sentry (security gateway) and bottle (workload) are
// the two container roles within a crucible. Used as suffixes for kludge
// operations and as commit-message vocabulary distinguishing the two builds.
pub const RBTDRM_CONTAINER_SENTRY: &str = "sentry";
pub const RBTDRM_CONTAINER_BOTTLE: &str = "bottle";

// Regime-validation contract surfaces — the regime module theurge sources and
// the public *_probate entry it calls to drive a staged regime file through
// kindle+enforce. Single definition per String Boundary Discipline.
pub const RBTDRM_MODULE_RBRR: &str = "rbrr_regime.sh";
pub const RBTDRM_PROBATE_RBRR: &str = "rbrr_probate";
pub const RBTDRM_MODULE_RBRD: &str = "rbrd_regime.sh";
pub const RBTDRM_PROBATE_RBRD: &str = "rbrd_probate";
pub const RBTDRM_MODULE_RBRV: &str = "rbrv_regime.sh";
pub const RBTDRM_PROBATE_RBRV: &str = "rbrv_probate";
pub const RBTDRM_MODULE_RBRN: &str = "rbrn_regime.sh";
pub const RBTDRM_PROBATE_RBRN: &str = "rbrn_probate";

/// Per-fixture required colophons. Returns None for unknown fixtures.
pub fn rbtdrm_required_colophons(fixture: &str) -> Option<&'static [&'static str]> {
    match fixture {
        RBTDRM_FIXTURE_TADMOR
        | RBTDRM_FIXTURE_MORIAH
        | RBTDRM_FIXTURE_SRJCL
        | RBTDRM_FIXTURE_PLUML => Some(&[
            RBTDGC_CRUCIBLE_CHARGE,
            RBTDGC_CRUCIBLE_QUENCH,
            RBTDGC_CRUCIBLE_WRIT,
            RBTDGC_CRUCIBLE_FIAT,
            RBTDGC_CRUCIBLE_BARK,
            RBTDGC_CRUCIBLE_ACTIVE,
        ]),
        RBTDRM_FIXTURE_HALLMARK_LIFECYCLE => Some(&[
            RBTDGC_ORDAIN_HALLMARK,
            RBTDGC_ABJURE_HALLMARK,
            RBTDGC_REKON_HALLMARK,
            RBTDGC_AUDIT_HALLMARKS,
        ]),
        RBTDRM_FIXTURE_BATCH_VOUCH => Some(&[
            RBTDGC_ORDAIN_HALLMARK,
            RBTDGC_ABJURE_HALLMARK,
            RBTDGC_JETTISON_HALLMARK_IMAGE,
            RBTDGC_VOUCH_HALLMARKS,
            RBTDGC_TALLY_HALLMARKS,
        ]),
        RBTDRM_FIXTURE_ACCESS_PROBE => Some(&[
            RBTDGC_CHECK_GOVERNOR,
            RBTDGC_CHECK_RETRIEVER,
            RBTDGC_CHECK_DIRECTOR,
            RBTDGC_CHECK_PAYOR,
        ]),
        RBTDRM_FIXTURE_ENROLLMENT_VALIDATION
        | RBTDRM_FIXTURE_REGIME_VALIDATION
        | RBTDRM_FIXTURE_REGIME_SMOKE
        | RBTDRM_FIXTURE_CALIBRANT_VERDICTS
        | RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST
        | RBTDRM_FIXTURE_CALIBRANT_PROGRESSING
        | RBTDRM_FIXTURE_CALIBRANT_SENTINEL => Some(&[]),
        RBTDRM_FIXTURE_DOCKERFILE_HYGIENE => Some(&[
            RBTDGC_HYGIENE_CHECK_DOCKERFILE,
            RBTDGC_HYGIENE_CHECK_VESSEL,
        ]),
        RBTDRM_FIXTURE_PRISTINE_LIFECYCLE => Some(&[
            RBTDGC_LEVY_DEPOT,
            RBTDGC_LIST_DEPOT,
            RBTDGC_UNMAKE_DEPOT,
            RBTDGC_MANTLE_GOVERNOR,
            RBTDGC_INVEST_RETRIEVER,
            RBTDGC_INVEST_DIRECTOR,
            RBTDGC_DIVEST_RETRIEVER,
            RBTDGC_DIVEST_DIRECTOR,
            RBTDGC_CHECK_RETRIEVER,
            RBTDGC_CHECK_DIRECTOR,
        ]),
        RBTDRM_FIXTURE_CANONICAL_ESTABLISH => Some(&[
            RBTDGC_LEVY_DEPOT,
            RBTDGC_LIST_DEPOT,
            RBTDGC_MANTLE_GOVERNOR,
            RBTDGC_INVEST_RETRIEVER,
            RBTDGC_INVEST_DIRECTOR,
            RBTDGC_CHECK_RETRIEVER,
            RBTDGC_CHECK_DIRECTOR,
        ]),
        // canonical-invest reuses canonical-establish's investiture cases sans
        // levy — same colophons minus LEVY_DEPOT (depot is operator-provided).
        RBTDRM_FIXTURE_CANONICAL_INVEST => Some(&[
            RBTDGC_LIST_DEPOT,
            RBTDGC_MANTLE_GOVERNOR,
            RBTDGC_INVEST_RETRIEVER,
            RBTDGC_INVEST_DIRECTOR,
            RBTDGC_CHECK_RETRIEVER,
            RBTDGC_CHECK_DIRECTOR,
        ]),
        // kludge-tadmor builds both vessels locally; only the two kludge
        // colophons are exercised (no charge/credential colophons here).
        RBTDRM_FIXTURE_KLUDGE_TADMOR => Some(&[
            RBTDGC_CRUCIBLE_KLUDGE_SENTRY,
            RBTDGC_CRUCIBLE_KLUDGE_BOTTLE,
        ]),
        // dogfight ordains/summons/abjures a single conjure-mode hallmark; the
        // bare container-runtime run is shelled directly, not via a colophon.
        RBTDRM_FIXTURE_DOGFIGHT => Some(&[
            RBTDGC_ORDAIN_HALLMARK,
            RBTDGC_SUMMON_HALLMARK,
            RBTDGC_ABJURE_HALLMARK,
        ]),
        RBTDRM_FIXTURE_ONBOARDING_SEQUENCE => Some(&[
            RBTDGC_INSCRIBE_RELIQUARY,
            RBTDGC_YOKE_RELIQUARY,
            RBTDGC_ENSHRINE_VESSEL,
            RBTDGC_ORDAIN_HALLMARK,
            RBTDGC_CRUCIBLE_KLUDGE_SENTRY,
            RBTDGC_CRUCIBLE_KLUDGE_BOTTLE,
            RBTDGC_WREST_HALLMARK_IMAGE,
            RBTDGC_SUMMON_HALLMARK,
            RBTDGC_PLUMB_FULL,
            RBTDGC_PLUMB_COMPACT,
            RBTDGC_REKON_HALLMARK,
            RBTDGC_JETTISON_HALLMARK_IMAGE,
            RBTDGC_ABJURE_HALLMARK,
        ]),
        RBTDRM_FIXTURE_HANDBOOK_RENDER => Some(&[
            RBTDGC_ONBOARD_START_HERE,
            RBTDGC_ONBOARD_CRASH_COURSE,
            RBTDGC_ONBOARD_CRED_RETRIEVER,
            RBTDGC_ONBOARD_CRED_DIRECTOR,
            RBTDGC_ONBOARD_FIRST_CRUCIBLE,
            RBTDGC_ONBOARD_DIR_FIRST_BUILD,
            RBTDGC_ONBOARD_PAYOR_HB,
            RBTDGC_ONBOARD_GOVERNOR_HB,
            RBTDGC_HANDBOOK_TOP,
            RBTDGC_HANDBOOK_WINDOWS,
            RBTDGC_HW_DOCKER_DESKTOP,
            RBTDGC_HW_DOCKER_CONTEXT,
            RBTDGC_PAYOR_ESTABLISH,
            RBTDGC_PAYOR_REFRESH,
            RBTDGC_QUOTA_BUILD,
        ]),
        _ => None,
    }
}

/// Verify that all required colophons for a fixture appear in the zipper manifest string.
pub fn rbtdrm_verify(manifest: &str, fixture: &str) -> Result<(), String> {
    let required = match rbtdrm_required_colophons(fixture) {
        Some(r) => r,
        None => {
            return Err(format!(
                "rbtd: unknown fixture '{}' — not registered in manifest",
                fixture
            ));
        }
    };
    for colophon in required {
        let found = manifest.split_whitespace().any(|token| token == *colophon);
        if !found {
            return Err(format!(
                "rbtd: colophon '{}' not found in zipper manifest (fixture '{}')",
                colophon, fixture
            ));
        }
    }
    Ok(())
}
