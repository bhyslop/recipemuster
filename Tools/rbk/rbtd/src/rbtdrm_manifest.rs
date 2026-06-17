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
// per-fixture required-colophon manifest and the role→probe mapping. Colophon
// existence is now enforced by compilation (this map references the generated
// consts directly) plus the build-time diff gate (rbq regenerates and diffs
// the consts against the zipper); the former runtime drift check is retired.
use crate::rbtdgc_consts::*;

// Credential roles are projected from rbcc_constants.sh into the generated
// RBTDGC_ACCOUNT_* consts (rbtdgc_consts.rs) — consumed here and across the access
// probe surface. The former hand-written RBTDRM_ROLE_* mirror is retired.

/// Map a credential role to its access-probe colophon. Returns None for
/// unknown roles. Replaces the former role-as-imprint scheme: each role now
/// names its own global tabtarget under the rbw-ac* family.
pub fn rbtdrm_credential_check_colophon(role: &str) -> Option<&'static str> {
    match role {
        RBTDGC_ACCOUNT_GOVERNOR => Some(RBTDGC_CHECK_GOVERNOR),
        RBTDGC_ACCOUNT_RETRIEVER => Some(RBTDGC_CHECK_RETRIEVER),
        RBTDGC_ACCOUNT_DIRECTOR => Some(RBTDGC_CHECK_DIRECTOR),
        RBTDGC_ACCOUNT_PAYOR => Some(RBTDGC_CHECK_PAYOR),
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
// Lode-lifecycle fixture — fetched-side base capture against live GAR:
// ensconce -> divine (enumerate + inspect) -> banish, registry restored.
pub const RBTDRM_FIXTURE_LODE_LIFECYCLE: &str = "lode-lifecycle";
// Reliquary-lifecycle fixture — fetched-side cohort capture against live GAR:
// conclave -> divine (enumerate + inspect members) -> banish, registry restored.
pub const RBTDRM_FIXTURE_RELIQUARY_LIFECYCLE: &str = "reliquary-lifecycle";
// Wsl-lifecycle fixture — fetched-side rootfs capture against live GAR:
// underpin -> divine (enumerate + inspect rootfs member) -> banish, restored.
pub const RBTDRM_FIXTURE_WSL_LIFECYCLE: &str = "wsl-lifecycle";
// Podvm-lifecycle fixture — fetched-side podvm disk-leaf capture against live GAR:
// immure -> divine (cohort) -> augur (members + envelope) -> per-member jettison -> banish, restored.
pub const RBTDRM_FIXTURE_PODVM_LIFECYCLE: &str = "podvm-lifecycle";
// Foedus-lifecycle fixture — federation IdP-trust round-trip against the live org:
// probe payor -> affiance a throwaway pool -> jilt (DELETED) -> re-jilt (no-op).
// Quota-touching (a genuine create cannot reuse a soft-deleted id; soft-deleted
// pools hold the 100-per-org cap ~30 days), so operator-invoked only — registered
// for discovery, a member of no suite. The payor-credential gate fails loud, never
// skips: this fixture is never a suite passenger (see the pace docket).
pub const RBTDRM_FIXTURE_FOEDUS_LIFECYCLE: &str = "foedus-lifecycle";
// Fast fixtures (no external dependencies)
pub const RBTDRM_FIXTURE_ENROLLMENT_VALIDATION: &str = "enrollment-validation";
pub const RBTDRM_FIXTURE_RECIPE_VALIDATION: &str = "recipe-validation";
pub const RBTDRM_FIXTURE_REGIME_VALIDATION: &str = "regime-validation";
pub const RBTDRM_FIXTURE_REGIME_SMOKE: &str = "regime-smoke";
// Regime-poison — in-universe negatives. Drives the real validate verbs against
// real (in-tree or staged) regimes with one field corrupted via the
// regime-poison tweak, asserting the specific band code of the gate that fires.
// NOT credless: the tweak slot carries the per-case poison, so this fixture
// cannot ride fast (whose slot belongs to the credless guard) — it enrolls in
// service/crucible/complete instead.
pub const RBTDRM_FIXTURE_REGIME_POISON: &str = "regime-poison";
pub const RBTDRM_FIXTURE_HANDBOOK_RENDER: &str = "handbook-render";
pub const RBTDRM_FIXTURE_DOCKERFILE_HYGIENE: &str = "dockerfile-hygiene";
// Conformance — vocabulary-eviction static analysis over Tools/ and tt/. No
// external dependency; the standing home for evicted-term assertions (ACG).
pub const RBTDRM_FIXTURE_CONFORMANCE: &str = "conformance";
// Foundry-path — buc_native_path_capture Cygwin /cygdrive normalizer. No
// external dependency; pure bash-function unit test sourced direct (no kindle).
pub const RBTDRM_FIXTURE_FOUNDRY_PATH: &str = "foundry-path";
// Podvm-resolve — host-side zrbld_immure_resolve_family brand mapping. No GCP
// creds or container runtime required; invokes immure colophon, asserts the
// diagnostic line emitted before credential load, expects non-zero exit.
pub const RBTDRM_FIXTURE_PODVM_RESOLVE: &str = "podvm-resolve";
// Cupel — BCG command-dependency static analysis over all Tools/ bash. No
// external dependency; partitions kit-bash (strict) from GCB-bash (looser).
pub const RBTDRM_FIXTURE_CUPEL: &str = "cupel";
// Pristine-lifecycle fixture (gate + SA/depot lifecycle cases)
pub const RBTDRM_FIXTURE_PRISTINE_LIFECYCLE: &str = "pristine-lifecycle";
// Gauntlet canonical-establish fixture (§2: canonical depot levy + governor
// enrobe + retriever/director enrobe with per-case precondition probes)
pub const RBTDRM_FIXTURE_CANONICAL_ESTABLISH: &str = "canonical-establish";
// Skirmish canonical-enrobe fixture — the no-levy variant of
// canonical-establish. Reuses the three enrobe cases (governor enrobe +
// retriever/director enrobe) against a depot the operator has already levied
// by hand; omits depot-levy so the skirmish suite creates no GCP project per
// run. Distinction from canonical-establish is precondition, not behavior.
pub const RBTDRM_FIXTURE_CANONICAL_ENROBE: &str = "canonical-enrobe";
// Gauntlet onboarding-sequence fixture (§3: handbook-walked vessel
// construction — conclave reliquary, ensconce bases, kludge tadmor/ccyolo,
// plus one ordain-* case per director-mode handbook track, build-only)
pub const RBTDRM_FIXTURE_ONBOARDING_SEQUENCE: &str = "onboarding-sequence";
// Self-contained tadmor build fixture (rbw-ts.TestSuite.tadmor): kludges tadmor sentry+bottle
// locally and commits each hallmark (so the subsequent tadmor crucible fixture
// charges against a clean nameplate). Reuses onboarding's kludge helper minus
// its reliquary-touchmark witness probe — local kludge has no GCP/reliquary dep.
pub const RBTDRM_FIXTURE_KLUDGE_TADMOR: &str = "kludge-tadmor";
// Dogfight cloud-build viability fixture — standing-depot sibling to
// canonical-enrobe, proving the cloud-build → summon → run path yields a
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

// Operation verbs and container roles are generated as RBTDGC_VERB_* and
// RBTDGC_CONTAINER_* (rbtdgc_consts.rs) from their canonical bash home in
// rbcc_constants.sh; consumers source those directly. Compound operations like
// "kludge sentry" are composed at the call site from RBTDGC_VERB_KLUDGE and the
// relevant RBTDGC_CONTAINER_* constant.

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
        RBTDRM_FIXTURE_LODE_LIFECYCLE => Some(&[
            RBTDGC_ENSCONCE_BOLE,
            RBTDGC_DIVINE_LODES,
            RBTDGC_BANISH_LODE,
        ]),
        RBTDRM_FIXTURE_RELIQUARY_LIFECYCLE => Some(&[
            RBTDGC_CONCLAVE_RELIQUARY,
            RBTDGC_DIVINE_LODES,
            RBTDGC_BANISH_LODE,
        ]),
        RBTDRM_FIXTURE_WSL_LIFECYCLE => Some(&[
            RBTDGC_UNDERPIN_WSL,
            RBTDGC_DIVINE_LODES,
            RBTDGC_BANISH_LODE,
        ]),
        RBTDRM_FIXTURE_PODVM_LIFECYCLE => Some(&[
            RBTDGC_IMMURE_PODVM,
            RBTDGC_DIVINE_LODES,
            RBTDGC_AUGUR_LODE,
            RBTDGC_LIST_IMAGES,
            RBTDGC_JETTISON_IMAGE,
            RBTDGC_BANISH_LODE,
        ]),
        RBTDRM_FIXTURE_FOEDUS_LIFECYCLE => Some(&[
            RBTDGC_CHECK_PAYOR,
            RBTDGC_AFFIANCE_MANOR,
            RBTDGC_JILT_MANOR,
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
        | RBTDRM_FIXTURE_RECIPE_VALIDATION
        | RBTDRM_FIXTURE_REGIME_VALIDATION
        | RBTDRM_FIXTURE_REGIME_SMOKE
        | RBTDRM_FIXTURE_FOUNDRY_PATH
        | RBTDRM_FIXTURE_CUPEL
        | RBTDRM_FIXTURE_CONFORMANCE
        | RBTDRM_FIXTURE_CALIBRANT_VERDICTS
        | RBTDRM_FIXTURE_CALIBRANT_FAIL_FAST
        | RBTDRM_FIXTURE_CALIBRANT_PROGRESSING
        | RBTDRM_FIXTURE_CALIBRANT_SENTINEL => Some(&[]),
        RBTDRM_FIXTURE_PODVM_RESOLVE => Some(&[
            RBTDGC_IMMURE_PODVM,
        ]),
        RBTDRM_FIXTURE_DOCKERFILE_HYGIENE => Some(&[
            RBTDGC_HYGIENE_CHECK_DOCKERFILE,
            RBTDGC_HYGIENE_CHECK_VESSEL,
        ]),
        RBTDRM_FIXTURE_PRISTINE_LIFECYCLE => Some(&[
            RBTDGC_LEVY_DEPOT,
            RBTDGC_LIST_DEPOT,
            RBTDGC_UNMAKE_DEPOT,
            RBTDGC_ENROBE_GOVERNOR,
            RBTDGC_ENROBE_RETRIEVER,
            RBTDGC_ENROBE_DIRECTOR,
            RBTDGC_DEFROCK_RETRIEVER,
            RBTDGC_DEFROCK_DIRECTOR,
            RBTDGC_CHECK_RETRIEVER,
            RBTDGC_CHECK_DIRECTOR,
        ]),
        RBTDRM_FIXTURE_CANONICAL_ESTABLISH => Some(&[
            RBTDGC_LEVY_DEPOT,
            RBTDGC_LIST_DEPOT,
            RBTDGC_ENROBE_GOVERNOR,
            RBTDGC_ENROBE_RETRIEVER,
            RBTDGC_ENROBE_DIRECTOR,
            RBTDGC_CHECK_RETRIEVER,
            RBTDGC_CHECK_DIRECTOR,
        ]),
        // canonical-enrobe reuses canonical-establish's enrobe cases sans
        // levy — same colophons minus LEVY_DEPOT (depot is operator-provided).
        RBTDRM_FIXTURE_CANONICAL_ENROBE => Some(&[
            RBTDGC_LIST_DEPOT,
            RBTDGC_ENROBE_GOVERNOR,
            RBTDGC_ENROBE_RETRIEVER,
            RBTDGC_ENROBE_DIRECTOR,
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
            RBTDGC_CONCLAVE_RELIQUARY,
            RBTDGC_YOKE_RELIQUARY,
            RBTDGC_ENSCONCE_BOLE,
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

