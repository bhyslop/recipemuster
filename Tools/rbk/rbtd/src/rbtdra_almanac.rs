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
// RBTDRA — the almanac: theurge's fixture roster and suite composition.
//
// Single source of truth for which fixtures exist (RBTDRA_FIXTURES) and how
// named suites compose them (RBTDRA_SUITES), plus the name->definition lookups.
// Extracted from rbtdrc_crucible; consulted at runtime by main.rs dispatch and
// in unit tests. A compile-time guard rejects duplicate fixture/suite names.

use crate::rbtdre_engine::{rbtdre_Fixture, rbtdre_Suite};

/// Registry of all fixtures known to theurge. Single source of truth: drives
/// rbtdra_lookup_fixture and the helpful "list valid fixtures" diagnostic the
/// single-case tabtarget emits on missing/unknown fixture arg. Declaration
/// order is also the listing order operators see.
pub static RBTDRA_FIXTURES: &[&'static rbtdre_Fixture] = &[
    &crate::rbtdrc_crucible::RBTDRC_FIXTURE_TADMOR,
    &crate::rbtdrc_crucible::RBTDRC_FIXTURE_MORIAH,
    &crate::rbtdrc_crucible::RBTDRC_FIXTURE_SRJCL,
    &crate::rbtdrc_crucible::RBTDRC_FIXTURE_PLUML,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_HALLMARK_LIFECYCLE,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_LODE_LIFECYCLE,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_RELIQUARY_LIFECYCLE,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_WSL_LIFECYCLE,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_PODVM_LIFECYCLE,
    // foedus-lifecycle: discovery-registered, operator-invoked only — quota-touching,
    // so a member of no suite (see RBTDRA_SUITES). Runnable via FixtureRun.
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_FOEDUS_LIFECYCLE,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_BATCH_VOUCH,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_ACCESS_PROBE,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_TERRIER_SCAFFOLD,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_TERRIER_ATOMICITY,
    &crate::rbtdrv_patrol::RBTDRV_FIXTURE_CHAINING_LIVERY,
    &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
    &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
    &crate::rbtdrs_poison::RBTDRS_FIXTURE_REGIME_POISON,
    &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
    &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
    &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
    &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
    &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
    &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
    &crate::rbtdrh_chain::RBTDRH_FIXTURE_CHAINING_FACT_BAND,
    &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
    &crate::rbtdrp_lifecycle::RBTDRP_FIXTURE_DEPOT_LIFECYCLE,
    &crate::rbtdrk_depot::RBTDRK_FIXTURE_FREEHOLD_ESTABLISH,
    &crate::rbtdrk_depot::RBTDRK_FIXTURE_FREEHOLD_CHURN,
    &crate::rbtdro_onboarding::RBTDRO_FIXTURE_ONBOARDING_SEQUENCE,
    &crate::rbtdro_onboarding::RBTDRO_FIXTURE_KLUDGE_TADMOR,
    &crate::rbtdrd_dogfight::RBTDRD_FIXTURE_DOGFIGHT,
    &crate::rbtdrl_calibrant::RBTDRL_FIXTURE_VERDICTS,
    &crate::rbtdrl_calibrant::RBTDRL_FIXTURE_FAIL_FAST,
    &crate::rbtdrl_calibrant::RBTDRL_FIXTURE_PROGRESSING,
    &crate::rbtdrl_calibrant::RBTDRL_FIXTURE_SENTINEL,
];

/// Resolve a fixture name to its registered Fixture definition. Returns None
/// for unregistered names; callers decide whether that is fatal.
pub fn rbtdra_lookup_fixture(fixture: &str) -> Option<&'static rbtdre_Fixture> {
    RBTDRA_FIXTURES.iter().find(|f| f.name == fixture).copied()
}

/// Suite registry — the sole owner of suite→fixture composition. The
/// `rbw-ts.TestSuite.{imprint}` tabtargets carry only the suite name; theurge
/// resolves membership here. Each member is a compile-checked reference to a
/// fixture static, so a mistyped or deleted member fails the build.
///
/// The dependency-tiered suites (picket, bivouac, echelon) list the reveille
/// fixtures explicitly rather than splicing a shared `reveille` slice: const slice
/// concatenation would be non-load-bearing cleverness, and the compile-time
/// member check already guards correctness. Reveille remains the conceptual base —
/// the explicit duplication is the cost of that being a compile-checked list.
pub static RBTDRA_SUITES: &[rbtdre_Suite] = &[
    // Reveille — no external dependencies.
    rbtdre_Suite {
        name: "reveille",
        fixtures: &[
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
            &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
            &crate::rbtdrh_chain::RBTDRH_FIXTURE_CHAINING_FACT_BAND,
        ],
    },
    // Picket — reveille + GCP-credentialed bare fixtures.
    rbtdre_Suite {
        name: "picket",
        fixtures: &[
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtdrs_poison::RBTDRS_FIXTURE_REGIME_POISON,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
            &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_ACCESS_PROBE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_HALLMARK_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_LODE_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_RELIQUARY_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_WSL_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_PODVM_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_BATCH_VOUCH,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_TERRIER_SCAFFOLD,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_TERRIER_ATOMICITY,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_CHAINING_LIVERY,
            &crate::rbtdrh_chain::RBTDRH_FIXTURE_CHAINING_FACT_BAND,
        ],
    },
    // Bivouac — reveille + container-runtime crucible fixtures.
    rbtdre_Suite {
        name: "bivouac",
        fixtures: &[
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtdrs_poison::RBTDRS_FIXTURE_REGIME_POISON,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
            &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_TADMOR,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_SRJCL,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_PLUML,
            &crate::rbtdrh_chain::RBTDRH_FIXTURE_CHAINING_FACT_BAND,
        ],
    },
    // Echelon — reveille + every dependency-tiered fixture (picket ∪ bivouac).
    rbtdre_Suite {
        name: "echelon",
        fixtures: &[
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtdrs_poison::RBTDRS_FIXTURE_REGIME_POISON,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
            &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_ACCESS_PROBE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_HALLMARK_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_LODE_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_RELIQUARY_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_WSL_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_PODVM_LIFECYCLE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_BATCH_VOUCH,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_TERRIER_SCAFFOLD,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_TERRIER_ATOMICITY,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_CHAINING_LIVERY,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_TADMOR,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_SRJCL,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_PLUML,
            &crate::rbtdrh_chain::RBTDRH_FIXTURE_CHAINING_FACT_BAND,
        ],
    },
    // Gauntlet — release-qualification ladder. Walks marshal-zero state through
    // freehold-credentialed state to crucible verification. Depot-lifecycle
    // case 1 is the entry-contract gate; the preceding enrollment-validation
    // runs state-indifferent and is harmless on broken state. The two depot
    // fixtures stand up two depots from the one freehold scheme: depot-lifecycle
    // mints + tears down an ephemeral leasehold (the full create→destroy proof),
    // then freehold-establish ensures the durable freehold the downstream
    // fixtures inherit. Fail-fast across fixtures is provided by the suite
    // runner's break-on-failure.
    rbtdre_Suite {
        name: "gauntlet",
        fixtures: &[
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtdrp_lifecycle::RBTDRP_FIXTURE_DEPOT_LIFECYCLE,
            &crate::rbtdrk_depot::RBTDRK_FIXTURE_FREEHOLD_ESTABLISH,
            &crate::rbtdro_onboarding::RBTDRO_FIXTURE_ONBOARDING_SEQUENCE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
            &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
            &crate::rbtdrv_patrol::RBTDRV_FIXTURE_HALLMARK_LIFECYCLE,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_TADMOR,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_MORIAH,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_SRJCL,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_PLUML,
        ],
    },
    // Skirmish — the "mini gauntlet": the depot→build→crucible chain WITHOUT
    // project-ID churn, against a standing operator-levied depot (no levy, no
    // unmake) where the gauntlet's depot-lifecycle/freehold-establish each levy a
    // fresh project; the lifecycle fixture is dropped entirely. onboarding-sequence
    // builds the crucible images (local kludge + cloud ordain into the standing
    // depot) and the four crucibles charge+run. OPERATOR PRECONDITION: a freehold
    // depot already levied (install freehold prefixes and run rbw-dL by hand) AND
    // federation credentials ready — a live sitting with the depot's mantles
    // donnable (the standing-freehold credential step is federation test-rig work,
    // no longer a keyfile re-enrobe preamble). Spends cloud build/GAR but creates
    // no GCP project per run.
    rbtdre_Suite {
        name: "skirmish",
        fixtures: &[
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtdro_onboarding::RBTDRO_FIXTURE_ONBOARDING_SEQUENCE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_REGIME_SMOKE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtdrf_handbook::RBTDRF_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtdrf_fast::RBTDRF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtdru_cupel::RBTDRU_FIXTURE_CUPEL,
            &crate::rbtdrn_conformance::RBTDRN_FIXTURE_CONFORMANCE,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_TADMOR,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_MORIAH,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_SRJCL,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_PLUML,
        ],
    },
    // Dogfight — standing-depot cloud-build viability probe. Sibling to skirmish
    // in the operator-precondition family (reuses a hand-levied depot, no levy,
    // no unmake) but charges NO crucible: it proves only the cloud-build →
    // summon → run path yields a runnable artifact; the fixture stays
    // crucible-free. OPERATOR PRECONDITION: a freehold depot already levied AND
    // federation credentials ready (a live sitting, the depot's mantles donnable),
    // exactly as skirmish assumes.
    rbtdre_Suite {
        name: "dogfight",
        fixtures: &[
            &crate::rbtdrd_dogfight::RBTDRD_FIXTURE_DOGFIGHT,
        ],
    },
    // Tadmor self-contained — fully local, no GCP/depot/project. Two fixtures in
    // sequence: kludge-tadmor builds BOTH vessels (sentry + bottle) locally and
    // commits each hallmark (the fixture owns the notch — same precedent as
    // onboarding's rbtdro_kludge_nameplate); then the tadmor crucible fixture
    // charges against the now-clean nameplate, runs the security cases, quenches.
    // The build is a separate fixture (nameplate passed explicitly) rather than
    // a self-charging tadmor fixture, because the crucible security cases resolve
    // their nameplate from the fixture name and would collide on "tadmor".
    rbtdre_Suite {
        name: "siege",
        fixtures: &[
            &crate::rbtdro_onboarding::RBTDRO_FIXTURE_KLUDGE_TADMOR,
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_TADMOR,
        ],
    },
    // Blockade - moriah airgap crucible. Sibling to siege on the network-posture
    // axis (siege = tether bottle, blockade = airgap bottle), but unlike siege it
    // is NOT fully local: moriah is conjure-mode and auto-summons its hallmarks
    // from the depot's GAR, so the charge needs a live Retriever mantle. The
    // moriah crucible charges (auto-summoning its already-ordained conjure
    // hallmarks), runs the security cases, quenches. No kludge predecessor —
    // conjure hallmarks come from GAR, not a local build. OPERATOR PRECONDITION:
    // freehold depot levied, federation credentials ready (a live sitting, the
    // retriever mantle donnable), AND the moriah conjure hallmark already ordained
    // into its GAR.
    rbtdre_Suite {
        name: "blockade",
        fixtures: &[
            &crate::rbtdrc_crucible::RBTDRC_FIXTURE_MORIAH,
        ],
    },
];

/// Resolve a suite name to its registered Suite definition. Returns None for
/// unregistered names; callers decide whether that is fatal.
pub fn rbtdra_lookup_suite(suite: &str) -> Option<&'static rbtdre_Suite> {
    RBTDRA_SUITES.iter().find(|s| s.name == suite)
}

// ── Compile-time uniqueness guard ────────────────────────────
//
// Fixture and suite name strings are author-maintained and lookups are
// first-match, so a duplicate name would silently shadow rather than error.
// These const assertions reject any duplicate at const-eval time — the
// strongest form of "fail as the registry is built up", with zero runtime cost.

const fn zrbtdra_str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn zrbtdra_assert_unique_fixtures(fixtures: &[&rbtdre_Fixture]) {
    let mut i = 0;
    while i < fixtures.len() {
        let mut j = i + 1;
        while j < fixtures.len() {
            if zrbtdra_str_eq(fixtures[i].name, fixtures[j].name) {
                panic!("duplicate fixture name in RBTDRA_FIXTURES");
            }
            j += 1;
        }
        i += 1;
    }
}

const fn zrbtdra_assert_unique_suites(suites: &[rbtdre_Suite]) {
    let mut i = 0;
    while i < suites.len() {
        let mut j = i + 1;
        while j < suites.len() {
            if zrbtdra_str_eq(suites[i].name, suites[j].name) {
                panic!("duplicate suite name in RBTDRA_SUITES");
            }
            j += 1;
        }
        i += 1;
    }
}

const _: () = zrbtdra_assert_unique_fixtures(RBTDRA_FIXTURES);
const _: () = zrbtdra_assert_unique_suites(RBTDRA_SUITES);
