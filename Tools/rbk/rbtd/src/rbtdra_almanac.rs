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
// RBTDRA — the almanac: theurge's fixture roster and suite composition.
//
// Single source of truth for which fixtures exist (RBTCEA_FIXTURES) and how
// named suites compose them (RBTCEA_SUITES), plus the name->definition lookups.
// Extracted from rbtccc_crucible; consulted at runtime by main.rs dispatch and
// in unit tests. A compile-time guard rejects duplicate fixture/suite names.

use crate::rbtcee_engine::{rbtcee_Fixture, rbtcee_Suite};

/// Registry of all fixtures known to theurge. Single source of truth: drives
/// rbtcea_lookup_fixture and the helpful "list valid fixtures" diagnostic the
/// single-case tabtarget emits on missing/unknown fixture arg. Declaration
/// order is also the listing order operators see.
pub static RBTCEA_FIXTURES: &[&'static rbtcee_Fixture] = &[
    &crate::rbtccc_crucible::RBTCCC_FIXTURE_TADMOR,
    &crate::rbtccc_crucible::RBTCCC_FIXTURE_MORIAH,
    &crate::rbtccc_crucible::RBTCCC_FIXTURE_SRJCL,
    &crate::rbtccc_crucible::RBTCCC_FIXTURE_PLUML,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_HALLMARK_LIFECYCLE,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_LODE_LIFECYCLE,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_RELIQUARY_LIFECYCLE,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_WSL_LIFECYCLE,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PODVM_LIFECYCLE,
    // foedus-lifecycle: discovery-registered, operator-invoked only — quota-touching,
    // so a member of no suite (see RBTCEA_SUITES). Runnable via FixtureRun.
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_FOEDUS_LIFECYCLE,
    // foedus-reuse: the standing-freehold REUSE credential leg (descry -> reuse-or-
    // affiance -> instate -> avow + don). Operator-invoked (avow reuses the
    // sederunt the operator novated beforehand; live mantle dons); quota-neutral on the reuse path but a member of no suite —
    // it heals creds against a STANDING freehold rather than provisioning one.
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_FOEDUS_REUSE,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_BATCH_VOUCH,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_ACCESS_PROBE,
    // credential-readiness: the standing-freehold readiness leader of the
    // release ladders (skirmish/dogfight/blockade lead with it; gauntlet
    // re-verifies right after freehold-establish) — espy + gated avow +
    // director/retriever dons, seconds not minutes on a credential deficit.
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CREDENTIAL_READINESS,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_POLITY_DENIAL,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PARLEY,
    &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CHAINING_LIVERY,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
    &crate::rbtchs_poison::RBTCHS_FIXTURE_REGIME_POISON,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
    // clipboard: discovery-registered, operator-invoked only — the round-trip
    // case reads and mutates the live desktop clipboard (arboard save/restore),
    // so a member of no suite. Runnable via FixtureRun.
    &crate::rbtchf_fast::RBTCHF_FIXTURE_CLIPBOARD,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
    &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
    &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
    // damnatio: the delivered-tree identity assay — a member of NO SUITE, by
    // construction. Its cases hold only of a stripped, lustrated candidate, so
    // against the maintainer's working tree it is red on purpose; a suite that ran
    // it would be red forever. The release ceremony invokes it by name post-strip.
    // Runnable via FixtureRun.
    &crate::rbtchd_damnatio::RBTCHD_FIXTURE_DAMNATIO,
    &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
    &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
    &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
    &crate::rbtcgl_lifecycle::RBTCGL_FIXTURE_DEPOT_LIFECYCLE,
    &crate::rbtcgk_depot::RBTCGK_FIXTURE_FREEHOLD_ESTABLISH,
    &crate::rbtcgk_depot::RBTCGK_FIXTURE_FREEHOLD_CHURN,
    &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_ONBOARDING_SEQUENCE,
    &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_KLUDGE_TADMOR,
    &crate::rbtcgd_dogfight::RBTCGD_FIXTURE_DOGFIGHT,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_VERDICTS,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_FAIL_FAST,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_PROGRESSING,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_SENTINEL,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_COVERAGE_ALIGNED,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_COVERAGE_UNDECLARED,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_COVERAGE_UNUSED,
    &crate::rbtchl_calibrant::RBTCHL_FIXTURE_TARIFF_DRIFT,
    &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
    &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
    // quiet-hush: the levels of environment-quiet's chain, reached
    // only through it one case at a time — a member of no suite.
    &crate::rbtdry_quiet::RBTDRY_FIXTURE_QUIET_HUSH,
];

/// Resolve a fixture name to its registered Fixture definition. Returns None
/// for unregistered names; callers decide whether that is fatal.
pub fn rbtcea_lookup_fixture(fixture: &str) -> Option<&'static rbtcee_Fixture> {
    RBTCEA_FIXTURES.iter().find(|f| f.name == fixture).copied()
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
/// the explicit duplication is the cost of that being a compile-checked list, and
/// the independent `RBTCEA_REVEILLE_BASE` set-equality/superset guard below pins
/// that base against silent per-ladder drift (audit 260623, finding X-d: the
/// chaining-fact-band member had been quietly dropped from gauntlet and skirmish).
pub static RBTCEA_SUITES: &[rbtcee_Suite] = &[
    // Reveille — no external dependencies.
    rbtcee_Suite {
        name: "reveille",
        fixtures: &[
            &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
            &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
            &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
            &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
            &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
            &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
        ],
    },
    // Picket — reveille + GCP-credentialed bare fixtures.
    rbtcee_Suite {
        name: "picket",
        fixtures: &[
            &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtchs_poison::RBTCHS_FIXTURE_REGIME_POISON,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
            &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
            &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_ACCESS_PROBE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_HALLMARK_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_LODE_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_RELIQUARY_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_WSL_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PODVM_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_BATCH_VOUCH,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_POLITY_DENIAL,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PARLEY,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CHAINING_LIVERY,
            &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
            &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
            &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
        ],
    },
    // Bivouac — reveille + container-runtime crucible fixtures. kludge-tadmor
    // precedes the tadmor crucible for the reason siege pairs them: tadmor's
    // hallmarks are k-prefixed (local kludge), so they resolve in the local
    // kludge namespace and NOTHING can summon them — charge's preflight can only
    // die. A committed k-pin is the residue of whichever station last kludged,
    // so a suite that charges tadmor without a kludge producer ahead of it is
    // runnable on that one station alone. The producer is local docker with no
    // GCP dependency, so it sits inside this suite's charter.
    rbtcee_Suite {
        name: "bivouac",
        fixtures: &[
            &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtchs_poison::RBTCHS_FIXTURE_REGIME_POISON,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
            &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
            &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_KLUDGE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_SRJCL,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_PLUML,
            &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
            &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
            &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
        ],
    },
    // Echelon — reveille + every dependency-tiered fixture (picket ∪ bivouac).
    // Carries bivouac's kludge-tadmor producer for the same reason, and must:
    // rbttcc_crucible asserts echelon == picket ∪ bivouac by name.
    rbtcee_Suite {
        name: "echelon",
        fixtures: &[
            &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtchs_poison::RBTCHS_FIXTURE_REGIME_POISON,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
            &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
            &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_ACCESS_PROBE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_HALLMARK_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_LODE_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_RELIQUARY_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_WSL_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PODVM_LIFECYCLE,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_BATCH_VOUCH,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_POLITY_DENIAL,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PARLEY,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CHAINING_LIVERY,
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_KLUDGE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_SRJCL,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_PLUML,
            &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
            &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
            &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
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
    rbtcee_Suite {
        name: "gauntlet",
        fixtures: &[
            &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
            &crate::rbtcgl_lifecycle::RBTCGL_FIXTURE_DEPOT_LIFECYCLE,
            &crate::rbtcgk_depot::RBTCGK_FIXTURE_FREEHOLD_ESTABLISH,
            // credential-readiness re-verifies the just-seated freehold
            // credentials (sederunt + director/retriever dons) before the build
            // bodies spend — and keeps the ladder containment law
            // (blockade ⊆ skirmish ⊆ gauntlet) whole now that the
            // standing-reuse ladders lead with it. It cannot lead HERE:
            // gauntlet starts from marshal-zero, where no depot exists to don
            // against; freehold-establish seats what this fixture verifies.
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CREDENTIAL_READINESS,
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_ONBOARDING_SEQUENCE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
            &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
            &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
            &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
            &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
            &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_HALLMARK_LIFECYCLE,
            // kludge-tadmor re-pins tadmor right before its charge, on
            // bivouac's pattern: onboarding-sequence's own tadmor kludge is
            // followed by its vessel commits, which the charge guard reads as
            // staling that pin.
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_KLUDGE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_MORIAH,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_SRJCL,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_PLUML,
        ],
    },
    // Skirmish — the "mini gauntlet": the depot→build→crucible chain WITHOUT
    // project-ID churn, against a standing operator-levied depot (no levy, no
    // unmake) where the gauntlet's depot-lifecycle/freehold-establish each levy a
    // fresh project; the lifecycle fixture is dropped entirely. onboarding-sequence
    // builds the crucible images (local kludge + cloud ordain into the standing
    // depot) and the four crucibles charge+run. OPERATOR PRECONDITION: a freehold
    // depot already levied (install freehold prefixes and run rbw-dL by hand) AND
    // federation credentials ready — a live sederunt with the depot's mantles
    // donnable; the credential-readiness leader proves that precondition in
    // seconds (remedy advisory on deficit) before any cloud spend. Spends cloud
    // build/GAR but creates no GCP project per run.
    rbtcee_Suite {
        name: "skirmish",
        fixtures: &[
            &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
            // The credential leader precedes the first cloud act: a credential
            // deficit fails in seconds with the renew advisory, never minutes
            // into onboarding-sequence (enrollment-validation stays first —
            // state-indifferent, catches a broken tree before any credential
            // probe is spent).
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CREDENTIAL_READINESS,
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_ONBOARDING_SEQUENCE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
            &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
            &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
            &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
            &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
            &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
            &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
            &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
            &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
            // kludge-tadmor stands after onboarding-sequence's vessel commits,
            // for the reason gauntlet carries it.
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_KLUDGE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_MORIAH,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_SRJCL,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_PLUML,
        ],
    },
    // Dogfight — standing-depot cloud-build viability probe. Sibling to skirmish
    // in the operator-precondition family (reuses a hand-levied depot, no levy,
    // no unmake) but charges NO crucible: it proves only the cloud-build →
    // summon → run path yields a runnable artifact; the fixture stays
    // crucible-free. OPERATOR PRECONDITION: a freehold depot already levied AND
    // federation credentials ready (a live sederunt, the depot's mantles donnable),
    // exactly as skirmish assumes; the credential-readiness leader proves it up
    // front.
    rbtcee_Suite {
        name: "dogfight",
        fixtures: &[
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CREDENTIAL_READINESS,
            &crate::rbtcgd_dogfight::RBTCGD_FIXTURE_DOGFIGHT,
        ],
    },
    // Tadmor self-contained — fully local, no GCP/depot/project. Two fixtures in
    // sequence: kludge-tadmor builds BOTH vessels (sentry + bottle) locally and
    // commits each hallmark (the fixture owns the notch — same precedent as
    // onboarding's rbtcgo_kludge_nameplate); then the tadmor crucible fixture
    // charges against the now-clean nameplate, runs the security cases, quenches.
    // The build is a separate fixture (nameplate passed explicitly) rather than
    // a self-charging tadmor fixture, because the crucible security cases resolve
    // their nameplate from the fixture name and would collide on "tadmor".
    rbtcee_Suite {
        name: "siege",
        fixtures: &[
            &crate::rbtcgo_onboarding::RBTCGO_FIXTURE_KLUDGE_TADMOR,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_TADMOR,
        ],
    },
    // Blockade - moriah airgap crucible. Sibling to siege on the network-posture
    // axis (siege = tether bottle, blockade = airgap bottle), but unlike siege it
    // is NOT fully local: moriah is conjure-mode and auto-summons its hallmarks
    // from the depot's GAR, so the charge needs a live Retriever mantle. The
    // moriah crucible charges (auto-summoning its already-ordained conjure
    // hallmarks), runs the security cases, quenches. No kludge predecessor —
    // conjure hallmarks come from GAR, not a local build. OPERATOR PRECONDITION:
    // freehold depot levied, federation credentials ready (a live sederunt, the
    // retriever mantle donnable), AND the moriah conjure hallmark already ordained
    // into its GAR; the credential-readiness leader proves the credential
    // precondition up front.
    rbtcee_Suite {
        name: "blockade",
        fixtures: &[
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_CREDENTIAL_READINESS,
            &crate::rbtccc_crucible::RBTCCC_FIXTURE_MORIAH,
        ],
    },
    // Parley — positive federation-admission probe. Base-free (like dogfight/siege/
    // blockade), a single-fixture REUSE probe: it drives the real polity verbs
    // against the standing freehold subject, proving the manor roll reflects a
    // genuine unseat→restore-brevet churn on the retriever mantle, then leaves the
    // freehold exactly as found. OPERATOR PRECONDITION: a freehold depot levied with
    // its standing terrier, the subject brevetted onto retriever, and federation
    // credentials ready (a live sederunt, the governor mantle donnable) — the parley
    // fixture self-skips only on an unreachable payor credential.
    rbtcee_Suite {
        name: "parley",
        fixtures: &[
            &crate::rbtcgp_patrol::RBTCGP_FIXTURE_PARLEY,
        ],
    },
    // Calibrant — the deliberately-failing two-member suite the touchstone
    // surface fixture drives as its suite-abort subject: calibrant-fail-fast
    // fails, so the runner's break-on-failure must keep calibrant-sentinel
    // from ever running (its sentinel file asserted absent). Never green by
    // design; belongs to no release ladder and bears no reveille base.
    rbtcee_Suite {
        name: RBTCEA_SUITE_NAME_CALIBRANT,
        fixtures: &[
            &crate::rbtchl_calibrant::RBTCHL_FIXTURE_FAIL_FAST,
            &crate::rbtchl_calibrant::RBTCHL_FIXTURE_SENTINEL,
        ],
    },
];

/// The calibrant suite's name — also the imprint of its
/// `rbw-ts.TestSuite.calibrant.sh` tabtarget, which the touchstone fixture
/// resolves for its suite-abort child run. Single definition serving both the
/// registration above and that resolution.
pub const RBTCEA_SUITE_NAME_CALIBRANT: &str = "calibrant";

/// Resolve a suite name to its registered Suite definition. Returns None for
/// unregistered names; callers decide whether that is fatal.
pub fn rbtcea_lookup_suite(suite: &str) -> Option<&'static rbtcee_Suite> {
    RBTCEA_SUITES.iter().find(|s| s.name == suite)
}

/// Canonical reveille base — the substrate-independent fixture set every
/// dependency-tiered suite and release ladder is contractually required to carry.
/// Declared INDEPENDENTLY of any suite (never spliced into one — see the
/// RBTCEA_SUITES doc) so that editing a suite's literal list cannot silently move
/// the bar it is checked against: this const is the regression oracle, the suites
/// are checked against it by the guard below.
///
/// Drift history (260623 consolidation audit, finding X-d): chaining-fact-band —
/// the credless feoff/yoke band-matrix conformance fixture — had been silently
/// dropped from the gauntlet and skirmish ladders while reveille/picket/bivouac/
/// echelon kept it, a conformance hole in exactly the suites that gate a release.
/// The set-equality + superset assertions below make any such drift a build error.
pub static RBTCEA_REVEILLE_BASE: &[&'static rbtcee_Fixture] = &[
    &crate::rbtchf_fast::RBTCHF_FIXTURE_ENROLLMENT_VALIDATION,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_VALIDATION,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_REGIME_SMOKE,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_PODVM_RESOLVE,
    &crate::rbtchh_handbook::RBTCHH_FIXTURE_HANDBOOK_RENDER,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_DOCKERFILE_HYGIENE,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_FOUNDRY_PATH,
    &crate::rbtchf_fast::RBTCHF_FIXTURE_RECIPE_VALIDATION,
    &crate::rbtchc_cupel::RBTCHC_FIXTURE_CUPEL,
    &crate::rbtchx_pyx::RBTCHX_FIXTURE_PYX,
    &crate::rbtchn_conformance::RBTCHN_FIXTURE_CONFORMANCE,
    &crate::rbtcgh_chain::RBTCGH_FIXTURE_CHAINING_FACT_BAND,
    &crate::rbtcct_touchstone::RBTCCT_FIXTURE_TOUCHSTONE,
    &crate::rbtdry_quiet::RBTDRY_FIXTURE_ENVIRONMENT_QUIET,
];

// ── Compile-time uniqueness guard ────────────────────────────
//
// Fixture and suite name strings are author-maintained and lookups are
// first-match, so a duplicate name would silently shadow rather than error.
// These const assertions reject any duplicate at const-eval time — the
// strongest form of "fail as the registry is built up", with zero runtime cost.

const fn zrbtcea_str_eq(a: &str, b: &str) -> bool {
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

const fn zrbtcea_assert_unique_fixtures(fixtures: &[&rbtcee_Fixture]) {
    let mut i = 0;
    while i < fixtures.len() {
        let mut j = i + 1;
        while j < fixtures.len() {
            if zrbtcea_str_eq(fixtures[i].name, fixtures[j].name) {
                panic!("duplicate fixture name in RBTCEA_FIXTURES");
            }
            j += 1;
        }
        i += 1;
    }
}

const fn zrbtcea_assert_unique_suites(suites: &[rbtcee_Suite]) {
    let mut i = 0;
    while i < suites.len() {
        let mut j = i + 1;
        while j < suites.len() {
            if zrbtcea_str_eq(suites[i].name, suites[j].name) {
                panic!("duplicate suite name in RBTCEA_SUITES");
            }
            j += 1;
        }
        i += 1;
    }
}

const _: () = zrbtcea_assert_unique_fixtures(RBTCEA_FIXTURES);
const _: () = zrbtcea_assert_unique_suites(RBTCEA_SUITES);

// ── Compile-time reveille-base guard ─────────────────────────
//
// Pin the canonical reveille set (RBTCEA_REVEILLE_BASE) against silent
// per-suite drift. Every reveille-bearing suite must carry the base as a subset;
// the "reveille" suite itself must equal it (mutual subset). The probe suites
// dogfight/siege/blockade are deliberately base-free and are named as bearers
// nowhere below. This is a pure ⊆/= check over the literal lists — the lists are
// never concatenated, so the duplication the RBTCEA_SUITES doc defends stays.

const fn zrbtcea_fixtures_contain(fixtures: &[&rbtcee_Fixture], name: &str) -> bool {
    let mut i = 0;
    while i < fixtures.len() {
        if zrbtcea_str_eq(fixtures[i].name, name) {
            return true;
        }
        i += 1;
    }
    false
}

/// True when every fixture in `needles` is present (by name) in `haystack`.
const fn zrbtcea_contains_all(haystack: &[&rbtcee_Fixture], needles: &[&rbtcee_Fixture]) -> bool {
    let mut i = 0;
    while i < needles.len() {
        if !zrbtcea_fixtures_contain(haystack, needles[i].name) {
            return false;
        }
        i += 1;
    }
    true
}

/// Suites contractually required to carry the full reveille base as a subset.
/// "reveille" is additionally held to equality (see below). dogfight/siege/
/// blockade are base-free probe suites and return false on purpose.
const fn zrbtcea_bears_base(name: &str) -> bool {
    zrbtcea_str_eq(name, "reveille")
        || zrbtcea_str_eq(name, "picket")
        || zrbtcea_str_eq(name, "bivouac")
        || zrbtcea_str_eq(name, "echelon")
        || zrbtcea_str_eq(name, "gauntlet")
        || zrbtcea_str_eq(name, "skirmish")
}

const fn zrbtcea_assert_reveille_base(suites: &[rbtcee_Suite], base: &[&rbtcee_Fixture]) {
    let mut i = 0;
    while i < suites.len() {
        let name = suites[i].name;
        let fixtures = suites[i].fixtures;
        if zrbtcea_bears_base(name) {
            // base ⊆ suite — every reveille-base member must run in this suite.
            if !zrbtcea_contains_all(fixtures, base) {
                panic!("a reveille-bearing suite is missing a reveille-base fixture");
            }
        }
        if zrbtcea_str_eq(name, "reveille") {
            // suite ⊆ base — the reveille suite carries nothing beyond the base
            // (this half plus the superset above pins reveille to set-equality).
            if !zrbtcea_contains_all(base, fixtures) {
                panic!("the reveille suite carries a fixture outside RBTCEA_REVEILLE_BASE");
            }
        }
        i += 1;
    }
}

const _: () = zrbtcea_assert_reveille_base(RBTCEA_SUITES, RBTCEA_REVEILLE_BASE);

// ── Compile-time suite ⊆ roster guard ───────────────────────
//
// Every fixture referenced in any suite must be registered in RBTCEA_FIXTURES.
// The reverse is NOT asserted: intentional roster-only fixtures (foedus-lifecycle,
// freehold-churn, calibrant-verdicts/-progressing, the calibrant-coverage-* trio
// etc.) belong to no suite by design — of the calibrants only calibrant-fail-fast
// and calibrant-sentinel sit in a suite, composing the calibrant suite. This is
// a pure suite ⊆ roster check — a suite gaining a roster-less fixture fails the
// build.

const fn zrbtcea_assert_suites_subset_fixtures(
    suites: &[rbtcee_Suite],
    roster: &[&rbtcee_Fixture],
) {
    let mut i = 0;
    while i < suites.len() {
        let suite_fixtures = suites[i].fixtures;
        let mut j = 0;
        while j < suite_fixtures.len() {
            if !zrbtcea_fixtures_contain(roster, suite_fixtures[j].name) {
                panic!("suite references a fixture not registered in RBTCEA_FIXTURES");
            }
            j += 1;
        }
        i += 1;
    }
}

const _: () = zrbtcea_assert_suites_subset_fixtures(RBTCEA_SUITES, RBTCEA_FIXTURES);
