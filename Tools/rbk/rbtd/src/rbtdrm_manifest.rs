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
// RBTDRM — colophon manifest verification for theurge

// Colophon names are projected from the zipper registry into the generated
// RBTGEC_* consts (rbtdgc_consts.rs). This module consumes them for the
// per-fixture required-colophon manifest and the role→probe mapping. Colophon
// existence is enforced by compilation (this map references the generated
// consts directly) plus the build-time diff gate (rbq regenerates and diffs
// the consts against the zipper).
use crate::rbtgec_consts::*;

// Credential roles are projected from rbcc_constants.sh into the generated
// RBTGEC_ACCOUNT_* consts (rbtdgc_consts.rs) — consumed here and across the access
// probe surface.

/// Map a credential role to its access-probe colophon. Returns None for
/// unknown roles. Only the payor OAuth probe survives the keyfile JWT-probe
/// demolition; the governor/director/retriever JWT probes retired with the
/// RBRA estate (their federation persona readiness rides avowal + the
/// mantle don under freehold-establish, not this map).
pub fn rbtcem_credential_check_colophon(role: &str) -> Option<&'static str> {
    match role {
        RBTGEC_ACCOUNT_PAYOR => Some(RBTGEC_CHECK_PAYOR),
        _ => None,
    }
}

// Fixture name consts — single definition per String Boundary Discipline.
// Crucible fixtures (charge/quench lifecycle)
pub const RBTCEM_FIXTURE_TADMOR: &str = "tadmor";
pub const RBTCEM_FIXTURE_MORIAH: &str = "moriah";
pub const RBTCEM_FIXTURE_SRJCL: &str = "srjcl";
pub const RBTCEM_FIXTURE_PLUML: &str = "pluml";
// Bare fixtures (GCP credentials, no container runtime)
pub const RBTCEM_FIXTURE_HALLMARK_LIFECYCLE: &str = "hallmark-lifecycle";
pub const RBTCEM_FIXTURE_BATCH_VOUCH: &str = "batch-vouch";
pub const RBTCEM_FIXTURE_ACCESS_PROBE: &str = "access-probe";
// Credential-readiness — the standing-freehold readiness leader of the release
// ladders: skirmish/dogfight/blockade run it ahead of any cloud spend, and
// gauntlet re-verifies right after freehold-establish. Espy the sederunt
// (fail-fast advisory when absent), a promptless baseline avow (the runway
// gate band-rejects a short sederunt with the novate advisory), then don
// director + retriever — the mantles the ladders' inner bodies wield. Restores
// the up-front credential step the keyfile re-enrobe preamble provided before
// the estate demolition; a deficit surfaces in seconds, never minutes into a
// build. Pure consumer of the espy/gate/novate surfaces — no new mechanism.
pub const RBTCEM_FIXTURE_CREDENTIAL_READINESS: &str = "credential-readiness";
// Lode-lifecycle fixture — fetched-side base capture against live GAR:
// ensconce -> divine (enumerate + inspect) -> banish, registry restored.
pub const RBTCEM_FIXTURE_LODE_LIFECYCLE: &str = "lode-lifecycle";
// Reliquary-lifecycle fixture — fetched-side cohort capture against live GAR:
// conclave -> divine (enumerate + inspect members) -> banish, registry restored.
pub const RBTCEM_FIXTURE_RELIQUARY_LIFECYCLE: &str = "reliquary-lifecycle";
// Wsl-lifecycle fixture — fetched-side rootfs capture against live GAR:
// underpin -> divine (enumerate + inspect rootfs member) -> banish, restored.
pub const RBTCEM_FIXTURE_WSL_LIFECYCLE: &str = "wsl-lifecycle";
// Podvm-lifecycle fixture — fetched-side podvm disk-leaf capture against live GAR:
// immure -> divine (cohort) -> augur (members + envelope) -> per-member jettison -> banish, restored.
pub const RBTCEM_FIXTURE_PODVM_LIFECYCLE: &str = "podvm-lifecycle";
// Foedus-lifecycle fixture — federation IdP-trust round-trip against the live org:
// probe payor -> affiance a throwaway pool -> jilt (DELETED) -> re-jilt (no-op).
// Quota-touching (a genuine create cannot reuse a soft-deleted id; soft-deleted
// pools hold the 100-per-org cap ~30 days), so operator-invoked only — registered
// for discovery, a member of no suite. The payor-credential gate fails loud, never
// skips: this fixture is never a suite passenger (see the pace docket).
pub const RBTCEM_FIXTURE_FOEDUS_LIFECYCLE: &str = "foedus-lifecycle";
// Foedus-reuse fixture — the standing-freehold REUSE credential leg the release
// ladders (skirmish/dogfight/blockade) assume but no fixture established. Probes
// the standing foedus (descry), reuses it cap-flat when healthy (affiance only on
// check-failure), re-points the selector (instate), then heals the credentials
// (avow + don governor/director/retriever). No pool churn on the reuse path, so —
// unlike foedus-lifecycle — it is quota-neutral; still operator-invoked (human-
// present avow, live mantle dons), a member of no suite, payor-gate fails loud.
pub const RBTCEM_FIXTURE_FOEDUS_REUSE: &str = "foedus-reuse";
// Polity-denial fixture — proves the polity verbs reject with the exact precision
// band across their failure surface. Admission arc: a governor-wielded verb's don is
// refused when the citizen is not brevetted onto the mantle (RBTGEC_BAND_ADMISSION) —
// don retriever (baseline) -> unseat retriever -> poll the don to the admission band
// -> isolation (held mantles still reach AR) -> brevet back -> poll positive (restore).
// Terrier-band arc (the regime-poison analogue for HTTP, folded in to supersede the
// retired interim terrier-atomicity rbw-dT proof): the same real verbs under the rbuh
// http-fault seam reject in the engross/expunge/peruse bands (brevet/unseat/rehearse),
// on a synthetic subject with pre-clean + final sweep. Picket-suite member; self-skips
// when the payor credential is unreachable (suite-passenger protection).
pub const RBTCEM_FIXTURE_POLITY_DENIAL: &str = "polity-denial";
// Parley fixture — the POSITIVE federation admission round-trip (the positive mirror
// of polity-denial, which owns every rejection-band assertion). Drives the real
// polity verbs against the REAL freehold subject: baseline rehearse (muniment stands)
// -> unseat retriever -> rehearse (muniment vanishes) -> restore-brevet -> rehearse
// (muniment stands again) -> poll the retriever don green, leaving the freehold as
// found. The novel content is rehearse's POSITIVE manor-roll assertions (asserted
// nowhere else). Picket-suite member (and its own base-free probe suite); self-skips
// when the payor credential is unreachable (suite-passenger protection).
pub const RBTCEM_FIXTURE_PARLEY: &str = "parley";
// Reveille fixtures (no external dependencies)
pub const RBTCEM_FIXTURE_ENROLLMENT_VALIDATION: &str = "enrollment-validation";
pub const RBTCEM_FIXTURE_RECIPE_VALIDATION: &str = "recipe-validation";
pub const RBTCEM_FIXTURE_REGIME_VALIDATION: &str = "regime-validation";
pub const RBTCEM_FIXTURE_REGIME_SMOKE: &str = "regime-smoke";
// Regime-poison — in-universe negatives. Drives the real validate verbs against
// real (in-tree or staged) regimes with one field corrupted via the
// regime-poison tweak, asserting the specific band code of the gate that fires.
// NOT credless: the tweak slot carries the per-case poison, so this fixture
// cannot ride reveille (whose slot belongs to the credless guard) — it enrolls in
// picket/bivouac/echelon instead.
pub const RBTCEM_FIXTURE_REGIME_POISON: &str = "regime-poison";
pub const RBTCEM_FIXTURE_HANDBOOK_RENDER: &str = "handbook-render";
pub const RBTCEM_FIXTURE_DOCKERFILE_HYGIENE: &str = "dockerfile-hygiene";
// Conformance — vocabulary-eviction static analysis over Tools/ and tt/. No
// external dependency; the standing home for evicted-term assertions.
pub const RBTCEM_FIXTURE_CONFORMANCE: &str = "conformance";
// Foundry-path — buc_native_path_capture Cygwin /cygdrive normalizer. No
// external dependency; pure bash-function unit test sourced direct (no kindle).
pub const RBTCEM_FIXTURE_FOUNDRY_PATH: &str = "foundry-path";
// Clipboard — buc_clipboard_copy_predicate platform normalizer (BUK footing,
// foundry-path sibling). Credless, but roster-only (member of no suite): the
// round-trip case reads and mutates the live desktop clipboard via arboard
// (read capability deliberately confined to this test binary — never on the
// shipped bash surface), with save/restore best-effort. Operator-invoked via
// FixtureRun; the no-tool decline case is deterministic everywhere.
pub const RBTCEM_FIXTURE_CLIPBOARD: &str = "clipboard";
// Podvm-resolve — host-side zrbldv_immure_resolve_family brand mapping. No GCP
// creds or container runtime required; invokes the presage colophon (rbw-lp),
// the read-only dry-run verb that resolves a family and reports what immure
// would capture, expecting exit 0 with a brand-mapping-line assertion.
pub const RBTCEM_FIXTURE_PODVM_RESOLVE: &str = "podvm-resolve";
// Chaining-fact-band — the band matrix for the durable-config chain LINKS (feoff,
// yoke). Drives the real verbs through the exec path against a staged temp
// vessel + a seeded previous/ fact, asserting the specific chain-rejection band
// on wrong-kind / unknown-prefix / broken chain, the good election, express-
// beats-chain precedence, and fact-intact (rejection precedes any write). The
// band fires only at the RBK consumer, so this is the home the footing (BUK)
// self-test cannot reach. Credless — nothing here mints a token.
pub const RBTCEM_FIXTURE_CHAINING_FACT_BAND: &str = "chaining-fact-band";
// Chaining-fact-livery — the cloud sibling of the local chaining-fact-band
// matrix. Proves the GENUINE producer->consumer succession: a real bole ensconce
// captures into live GAR and hands its touchmark forward, the chain wires it to a
// real feoff, and feoff elects the base anchor into a STAGED TEMP vessel (no
// tracked config, no commit). Catches drift between what a live ensconce writes
// and what a live feoff reads — which the synthetic band matrix cannot. Service
// tier; the touchmark is pinned via the ensconce-stamp tweak for a stable reset
// handle, and the case self-contains a banish-if-present baseline + best-effort
// cleanup (no setup hook — that channel is the crucible-charge signal).
pub const RBTCEM_FIXTURE_CHAINING_LIVERY: &str = "chaining-fact-livery";
// Cupel — command-dependency static analysis over all Tools/ bash. No
// external dependency; partitions kit-bash (strict) from GCB-bash (looser).
pub const RBTCEM_FIXTURE_CUPEL: &str = "cupel";
// Pyx — release-hygiene tree-invariants: crate-license allowlist over the
// committed Cargo.lock, root LICENSE presence, a curated secret-shape scan over
// the shipping roots, and the handbook-anchor check against README. No external
// dependency; every root is existence-tolerant so the checks hold on the
// stripped candidate tree as well as on main.
pub const RBTCEM_FIXTURE_PYX: &str = "pyx";

/// damnatio — the delivered-tree identity assay. A member of no suite: every case
/// is true only of a stripped, lustrated candidate, so it is red by construction
/// against the maintainer's working tree. The release ceremony invokes it by name.
pub const RBTCEM_FIXTURE_DAMNATIO: &str = "damnatio";

// Depot-lifecycle fixture (marshal-zero gate + ephemeral create→destroy arc).
// Shares the freehold scheme with the durable fixtures; tears down only the
// fresh leasehold it mints, never the standing freehold.
pub const RBTCEM_FIXTURE_DEPOT_LIFECYCLE: &str = "depot-lifecycle";
// Gauntlet freehold-establish fixture (§2: freehold ensure (reuse-or-levy) +
// federation-persona admission — avow, gird, brevet+don director/retriever,
// recognosce — with per-case precondition probes)
pub const RBTCEM_FIXTURE_FREEHOLD_ESTABLISH: &str = "freehold-establish";
// Freehold-churn fixture — the deliberate teardown of the standing freehold
// (rotate moniker off the live project, then unmake) that makes room for a fresh
// levy. Member of no suite; operator-invoked, like the lifecycle round-trips.
pub const RBTCEM_FIXTURE_FREEHOLD_CHURN: &str = "freehold-churn";
// Gauntlet onboarding-sequence fixture (§3: handbook-walked vessel
// construction — conclave reliquary, ensconce bases, kludge tadmor/ccyolo,
// plus one ordain-* case per director-mode handbook track, build-only)
pub const RBTCEM_FIXTURE_ONBOARDING_SEQUENCE: &str = "onboarding-sequence";
// Self-contained tadmor build fixture (rbw-ts.TestSuite.tadmor): kludges tadmor sentry+bottle
// locally and commits each hallmark (so the subsequent tadmor crucible fixture
// charges against a clean nameplate). Reuses onboarding's kludge helper minus
// its reliquary-touchmark witness probe — local kludge has no GCP/reliquary dep.
pub const RBTCEM_FIXTURE_KLUDGE_TADMOR: &str = "kludge-tadmor";
// Dogfight cloud-build viability fixture — runs against a standing
// operator-levied depot, proving the cloud-build → summon → run path yields a
// runnable artifact with NO crucible charged (the orthogonal axis skirmish
// covers). Ordains conjure-mode busybox, summons it, runs a degenerate
// container-runtime command proving executability, then abjures.
pub const RBTCEM_FIXTURE_DOGFIGHT: &str = "dogfight";
// Calibrant fixtures — synthetic deterministic-verdict fixtures, the test
// subjects the touchstone surface fixture spawns as child rbt runs. Internal
// framework-test plumbing.
pub const RBTCEM_FIXTURE_CALIBRANT_VERDICTS: &str = "calibrant-verdicts";
pub const RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST: &str = "calibrant-fail-fast";
pub const RBTCEM_FIXTURE_CALIBRANT_PROGRESSING: &str = "calibrant-progressing";
pub const RBTCEM_FIXTURE_CALIBRANT_SENTINEL: &str = "calibrant-sentinel";
// Calibrant coverage fixtures — prove the colophon census enforcement lands
// its diagnostics against the synthetic RBTGEC_THEURGE_NIHIL colophon:
// aligned declares+invokes (Pass), undeclared invokes without declaring
// (fixture FAILs via the positive check), unused declares without invoking
// (fixture FAILs via the negative check). Roster-only, driven by the
// touchstone surface fixture's coverage cases, never by any suite.
pub const RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED: &str = "calibrant-coverage-aligned";
pub const RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED: &str = "calibrant-coverage-undeclared";
pub const RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED: &str = "calibrant-coverage-unused";
// Calibrant tariff needle — proves the tariff shortfall gate (rbtdre_engine.rs)
// reddens a fixture that ran short of its declared invocation count. Its single
// case SKIPS and invokes nothing against a declared count of one, so the fixture
// exits zero without the gate and FAILs with it. Roster-only, driven by the
// touchstone surface fixture, never by any suite.
pub const RBTCEM_FIXTURE_CALIBRANT_TARIFF_DRIFT: &str = "calibrant-tariff-drift";
// Touchstone — the surface fixture: theurge certifying its own operator
// surface. A reveille member whose cases spawn child rbt runs through the
// real tabtarget chain against the deliberately-failing calibrant fixtures,
// asserting the child's exit code, diagnostic shape on the operator-visible
// stream, and sentinel/trace files. The watcher passes; the watched calibrant
// fixtures stay out of every dependency-tier and release suite.
pub const RBTCEM_FIXTURE_TOUCHSTONE: &str = "touchstone";

// Operation verbs and container roles are generated as RBTGEC_VERB_* and
// RBTGEC_CONTAINER_* (rbtdgc_consts.rs) from their canonical bash home in
// rbcc_constants.sh; consumers source those directly. Compound operations like
// "kludge sentry" are composed at the call site from RBTGEC_VERB_KLUDGE and the
// relevant RBTGEC_CONTAINER_* constant.

/// Per-fixture required colophons. Returns None for unknown fixtures.
pub fn rbtcem_required_colophons(fixture: &str) -> Option<&'static [&'static str]> {
    match fixture {
        // The security-case crucibles: both fixtures run RBTCCC_CASES_SECURITY,
        // which execs into sentry (writ) and pentacle (fiat) and drives the
        // ifrit from the bottle (bark).
        RBTCEM_FIXTURE_TADMOR | RBTCEM_FIXTURE_MORIAH => Some(&[
            RBTGEC_CRUCIBLE_CHARGE,
            RBTGEC_CRUCIBLE_QUENCH,
            RBTGEC_CRUCIBLE_WRIT,
            RBTGEC_CRUCIBLE_FIAT,
            RBTGEC_CRUCIBLE_BARK,
            RBTGEC_CRUCIBLE_ACTIVE,
        ]),
        // srjcl reaches the bottle to probe jupyter, but never execs into
        // sentry or pentacle.
        RBTCEM_FIXTURE_SRJCL => Some(&[
            RBTGEC_CRUCIBLE_CHARGE,
            RBTGEC_CRUCIBLE_QUENCH,
            RBTGEC_CRUCIBLE_BARK,
            RBTGEC_CRUCIBLE_ACTIVE,
        ]),
        // pluml drives the rendering server over host-side HTTP alone; charge
        // and quench (with their active assertions) are its whole colophon
        // surface.
        RBTCEM_FIXTURE_PLUML => Some(&[
            RBTGEC_CRUCIBLE_CHARGE,
            RBTGEC_CRUCIBLE_QUENCH,
            RBTGEC_CRUCIBLE_ACTIVE,
        ]),
        RBTCEM_FIXTURE_HALLMARK_LIFECYCLE => Some(&[
            RBTGEC_ORDAIN_HALLMARK,
            RBTGEC_ABJURE_HALLMARK,
            RBTGEC_REKON_HALLMARK,
            RBTGEC_AUDIT_HALLMARKS,
            RBTGEC_SUMMON_HALLMARK,
            RBTGEC_PLUMB_FULL,
        ]),
        RBTCEM_FIXTURE_LODE_LIFECYCLE => Some(&[
            RBTGEC_ENSCONCE_BOLE,
            RBTGEC_DIVINE_LODES,
            RBTGEC_AUGUR_LODE,
            RBTGEC_BANISH_LODE,
        ]),
        RBTCEM_FIXTURE_RELIQUARY_LIFECYCLE => Some(&[
            RBTGEC_CONCLAVE_RELIQUARY,
            RBTGEC_DIVINE_LODES,
            RBTGEC_AUGUR_LODE,
            RBTGEC_LIST_IMAGES,
            RBTGEC_JETTISON_IMAGE,
            RBTGEC_BANISH_LODE,
        ]),
        RBTCEM_FIXTURE_WSL_LIFECYCLE => Some(&[
            RBTGEC_UNDERPIN_WSL,
            RBTGEC_DIVINE_LODES,
            RBTGEC_AUGUR_LODE,
            RBTGEC_BANISH_LODE,
        ]),
        RBTCEM_FIXTURE_PODVM_LIFECYCLE => Some(&[
            RBTGEC_IMMURE_PODVM,
            RBTGEC_DIVINE_LODES,
            RBTGEC_AUGUR_LODE,
            RBTGEC_LIST_IMAGES,
            RBTGEC_JETTISON_IMAGE,
            RBTGEC_BANISH_LODE,
        ]),
        RBTCEM_FIXTURE_FOEDUS_LIFECYCLE => Some(&[
            RBTGEC_CHECK_PAYOR,
            RBTGEC_AFFIANCE_MANOR,
            RBTGEC_CANVASS_FOEDUS,
            RBTGEC_JILT_MANOR,
        ]),
        RBTCEM_FIXTURE_FOEDUS_REUSE => Some(&[
            RBTGEC_CHECK_PAYOR,
            RBTGEC_DESCRY_FOEDUS,
            RBTGEC_INSTATE_FOEDUS,
            RBTGEC_CHECK_AVOWAL,
            RBTGEC_CHECK_MANTLE,
        ]),
        RBTCEM_FIXTURE_POLITY_DENIAL => Some(&[
            RBTGEC_CHECK_PAYOR,
            RBTGEC_CHECK_MANTLE,
            RBTGEC_UNSEAT_POLITY,
            RBTGEC_BREVET_POLITY,
            RBTGEC_REHEARSE_POLITY,
        ]),
        RBTCEM_FIXTURE_PARLEY => Some(&[
            RBTGEC_CHECK_PAYOR,
            RBTGEC_CHECK_MANTLE,
            RBTGEC_UNSEAT_POLITY,
            RBTGEC_BREVET_POLITY,
            RBTGEC_REHEARSE_POLITY,
        ]),
        RBTCEM_FIXTURE_BATCH_VOUCH => Some(&[
            RBTGEC_ORDAIN_HALLMARK,
            RBTGEC_ABJURE_HALLMARK,
            RBTGEC_JETTISON_HALLMARK_IMAGE,
            RBTGEC_VOUCH_HALLMARKS,
            RBTGEC_TALLY_HALLMARKS,
        ]),
        RBTCEM_FIXTURE_ACCESS_PROBE => Some(&[
            RBTGEC_CHECK_PAYOR,
            RBTGEC_CHECK_AVOWAL,
            RBTGEC_ESPY_SEDERUNT,
        ]),
        RBTCEM_FIXTURE_CREDENTIAL_READINESS => Some(&[
            RBTGEC_ESPY_SEDERUNT,
            RBTGEC_CHECK_AVOWAL,
            RBTGEC_CHECK_MANTLE,
        ]),
        RBTCEM_FIXTURE_ENROLLMENT_VALIDATION
        | RBTCEM_FIXTURE_RECIPE_VALIDATION
        | RBTCEM_FIXTURE_REGIME_VALIDATION
        | RBTCEM_FIXTURE_REGIME_SMOKE
        | RBTCEM_FIXTURE_FOUNDRY_PATH
        | RBTCEM_FIXTURE_CLIPBOARD
        | RBTCEM_FIXTURE_CUPEL
        | RBTCEM_FIXTURE_PYX
        | RBTCEM_FIXTURE_DAMNATIO
        | RBTCEM_FIXTURE_CONFORMANCE
        | RBTCEM_FIXTURE_CALIBRANT_VERDICTS
        | RBTCEM_FIXTURE_CALIBRANT_FAIL_FAST
        | RBTCEM_FIXTURE_CALIBRANT_PROGRESSING
        | RBTCEM_FIXTURE_CALIBRANT_SENTINEL
        | RBTCEM_FIXTURE_CALIBRANT_TARIFF_DRIFT => Some(&[]),
        // DELIBERATELY EMPTY — do not "repair" by adding RBTGEC_THEURGE_NIHIL.
        // The arm above is empty because those fixtures invoke nothing; this one
        // is empty though its case DOES invoke nihil, which is the whole point:
        // the mismatch is what trips the positive census check, and the
        // fixture's expected verdict is FAIL. Declaring the colophon here would
        // turn the fixture green and silently delete the test.
        RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNDECLARED => Some(&[]),
        RBTCEM_FIXTURE_CALIBRANT_COVERAGE_ALIGNED | RBTCEM_FIXTURE_CALIBRANT_COVERAGE_UNUSED => {
            Some(&[RBTGEC_THEURGE_NIHIL])
        }
        RBTCEM_FIXTURE_PODVM_RESOLVE => Some(&[
            RBTGEC_PRESAGE_IMMURE,
        ]),
        // touchstone drives the theurge's own runner tabtargets — its children
        // are rbt invocations of the fixture, suite, and single-case runners.
        RBTCEM_FIXTURE_TOUCHSTONE => Some(&[
            RBTGEC_THEURGE_FIXTURE,
            RBTGEC_THEURGE_SUITE,
            RBTGEC_THEURGE_CASE,
        ]),
        RBTCEM_FIXTURE_DOCKERFILE_HYGIENE => Some(&[
            RBTGEC_HYGIENE_CHECK_DOCKERFILE,
            RBTGEC_HYGIENE_CHECK_VESSEL,
        ]),
        RBTCEM_FIXTURE_DEPOT_LIFECYCLE => Some(&[
            RBTGEC_LEVY_DEPOT,
            RBTGEC_LIST_DEPOT,
            RBTGEC_INCUSE_DEPOT,
            RBTGEC_COLLATE_DEPOT,
            RBTGEC_UNMAKE_DEPOT,
        ]),
        RBTCEM_FIXTURE_FREEHOLD_ESTABLISH => Some(&[
            RBTGEC_LIST_DEPOT,
            RBTGEC_RECOGNOSCE_DEPOT,
            RBTGEC_CHECK_AVOWAL,
            RBTGEC_GIRD_POLITY,
            RBTGEC_BREVET_POLITY,
            RBTGEC_CHECK_MANTLE,
        ]),
        RBTCEM_FIXTURE_FREEHOLD_CHURN => Some(&[
            RBTGEC_UNMAKE_DEPOT,
            RBTGEC_LIST_DEPOT,
        ]),
        // kludge-tadmor builds both vessels locally; only the two kludge
        // colophons are exercised (no charge/credential colophons here).
        RBTCEM_FIXTURE_KLUDGE_TADMOR => Some(&[
            RBTGEC_CRUCIBLE_KLUDGE_SENTRY,
            RBTGEC_CRUCIBLE_KLUDGE_BOTTLE,
        ]),
        // dogfight ordains/summons/abjures a single conjure-mode hallmark; the
        // bare container-runtime run is shelled directly, not via a colophon.
        RBTCEM_FIXTURE_DOGFIGHT => Some(&[
            RBTGEC_ORDAIN_HALLMARK,
            RBTGEC_SUMMON_HALLMARK,
            RBTGEC_ABJURE_HALLMARK,
        ]),
        RBTCEM_FIXTURE_ONBOARDING_SEQUENCE => Some(&[
            RBTGEC_CONCLAVE_RELIQUARY,
            RBTGEC_YOKE_RELIQUARY,
            RBTGEC_ENSCONCE_BOLE,
            RBTGEC_FEOFF_BOLE,
            RBTGEC_ORDAIN_HALLMARK,
            RBTGEC_ANOINT_GRAFT,
            RBTGEC_DRIVE_HALLMARK,
            RBTGEC_CRUCIBLE_KLUDGE_SENTRY,
            RBTGEC_CRUCIBLE_KLUDGE_BOTTLE,
            RBTGEC_WREST_HALLMARK_IMAGE,
            RBTGEC_SUMMON_HALLMARK,
            RBTGEC_PLUMB_FULL,
            RBTGEC_PLUMB_COMPACT,
            RBTGEC_REKON_HALLMARK,
            RBTGEC_JETTISON_HALLMARK_IMAGE,
            RBTGEC_ABJURE_HALLMARK,
        ]),
        RBTCEM_FIXTURE_HANDBOOK_RENDER => Some(&[
            RBTGEC_ONBOARD_START_HERE,
            RBTGEC_ONBOARD_CRASH_COURSE,
            RBTGEC_ONBOARD_FIRST_CRUCIBLE,
            RBTGEC_ONBOARD_DIR_FIRST_BUILD,
            RBTGEC_ONBOARD_PAYOR_HB,
            RBTGEC_PAYOR_ESTABLISH,
            RBTGEC_QUOTA_BUILD,
        ]),
        // Chaining-fact-band drives every band case through a per-verb funnel
        // (feoff/yoke/anoint/nameplate) or the shared readside funnel
        // (summon/plumb/augur/rekon); each of the 8 colophons below fires
        // unconditionally in at least one case, so none are permitted-only.
        RBTCEM_FIXTURE_CHAINING_FACT_BAND => Some(&[
            RBTGEC_FEOFF_BOLE,
            RBTGEC_YOKE_RELIQUARY,
            RBTGEC_ANOINT_GRAFT,
            RBTGEC_DRIVE_HALLMARK,
            RBTGEC_SUMMON_HALLMARK,
            RBTGEC_PLUMB_FULL,
            RBTGEC_AUGUR_LODE,
            RBTGEC_REKON_HALLMARK,
        ]),
        // Chaining-fact-livery's producer->consumer arc (ensconce -> feoff)
        // and the reset's divine both fire every run; banish is the
        // banish-if-present cleanup, declared permitted below.
        RBTCEM_FIXTURE_CHAINING_LIVERY => Some(&[
            RBTGEC_DIVINE_LODES,
            RBTGEC_ENSCONCE_BOLE,
            RBTGEC_FEOFF_BOLE,
        ]),
        // Every regime-poison case launches its validate colophon, so all eight
        // are required — the operator-local pair (oauth, station tincture)
        // included. Their self-skip elides only the POISONED invocation: the
        // un-poisoned baseline probe launches first and unconditionally, and
        // the census records at the launch (rbtcei_tabtarget_command), so
        // neither can ever go unused. On a station where those regimes are
        // absent the case skips, and a skipped case suppresses the negative
        // check outright — so requiring them cannot false-fail there either.
        RBTCEM_FIXTURE_REGIME_POISON => Some(&[
            RBTGEC_VALIDATE_REPO,
            RBTGEC_VALIDATE_DEPOT,
            RBTGEC_VALIDATE_PAYOR,
            BUWGC_RC_VALIDATE,
            RBTGEC_VALIDATE_NAMEPLATE,
            RBTGEC_VALIDATE_VESSEL,
            RBTGEC_VALIDATE_OAUTH,
            BUWGC_RS_VALIDATE,
        ]),
        _ => None,
    }
}

/// Per-fixture permitted colophons — the positive-only census tier. Admitted
/// at the invoke chokepoint exactly like a required colophon, but never
/// demanded by the negative (post-fixture) check: a permitted colophon may go
/// unused on a healthy run without failing the fixture. Defaults to empty for
/// every fixture not listed here. Reserved for invocations that are
/// conditional by design — a fixture whose real-world path only sometimes
/// reaches a colophon (e.g. reuse-vs-mint branches).
pub fn rbtcem_permitted_colophons(fixture: &str) -> &'static [&'static str] {
    match fixture {
        // Affiance fires only on a descry deficit — the healthy reuse path
        // never reaches it, so it cannot be required.
        RBTCEM_FIXTURE_FOEDUS_REUSE => &[RBTGEC_AFFIANCE_MANOR],
        // Levy fires only when the standing freehold is absent — the healthy
        // reuse path never reaches it, so it cannot be required.
        RBTCEM_FIXTURE_FREEHOLD_ESTABLISH => &[RBTGEC_LEVY_DEPOT],
        // Banish fires only when the reset's divine finds the pinned
        // touchmark already present — the steady-state pass never reaches it.
        RBTCEM_FIXTURE_CHAINING_LIVERY => &[RBTGEC_BANISH_LODE],
        _ => &[],
    }
}

