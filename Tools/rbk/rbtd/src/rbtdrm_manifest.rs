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

/// Colophon consts — single definition per String Boundary Discipline.
/// Each names the bash tabtarget colophon theurge invokes for that operation.

// Crucible lifecycle colophons (nameplate-scoped)
pub const RBTDRM_COLOPHON_CHARGE: &str = "rbw-cC";
pub const RBTDRM_COLOPHON_QUENCH: &str = "rbw-cQ";
pub const RBTDRM_COLOPHON_WRIT: &str = "rbw-cw";
pub const RBTDRM_COLOPHON_FIAT: &str = "rbw-cf";
pub const RBTDRM_COLOPHON_BARK: &str = "rbw-cb";

// Foundry colophons (global — no nameplate imprint)
pub const RBTDRM_COLOPHON_ORDAIN: &str = "rbw-fO";
pub const RBTDRM_COLOPHON_ABJURE: &str = "rbw-fA";
pub const RBTDRM_COLOPHON_TALLY: &str = "rbw-ft";
pub const RBTDRM_COLOPHON_KLUDGE: &str = "rbw-fk";
pub const RBTDRM_COLOPHON_VOUCH: &str = "rbw-fV";
pub const RBTDRM_COLOPHON_SUMMON: &str = "rbw-fs";
pub const RBTDRM_COLOPHON_PLUMB_FULL: &str = "rbw-fpf";
pub const RBTDRM_COLOPHON_PLUMB_COMPACT: &str = "rbw-fpc";

// Image colophons (global — no nameplate imprint)
// Three-domain symmetric: hallmarks (h), reliquaries (r), enshrinements (e).
// Verbs: rekon (member-list), muster (catalog-list), Jettison (delete), wrest (pull).
pub const RBTDRM_COLOPHON_REKON_HALLMARK: &str = "rbw-irh";
pub const RBTDRM_COLOPHON_REKON_RELIQUARY: &str = "rbw-irr";
pub const RBTDRM_COLOPHON_MUSTER_HALLMARKS: &str = "rbw-imh";
pub const RBTDRM_COLOPHON_MUSTER_RELIQUARIES: &str = "rbw-imr";
pub const RBTDRM_COLOPHON_MUSTER_ENSHRINEMENTS: &str = "rbw-ime";
pub const RBTDRM_COLOPHON_WREST_HALLMARK_IMAGE: &str = "rbw-iwh";
pub const RBTDRM_COLOPHON_WREST_RELIQUARY_IMAGE: &str = "rbw-iwr";
pub const RBTDRM_COLOPHON_WREST_ENSHRINED_IMAGE: &str = "rbw-iwe";
pub const RBTDRM_COLOPHON_JETTISON_HALLMARK_IMAGE: &str = "rbw-iJh";
pub const RBTDRM_COLOPHON_JETTISON_RELIQUARY_IMAGE: &str = "rbw-iJr";
pub const RBTDRM_COLOPHON_JETTISON_ENSHRINEMENT: &str = "rbw-iJe";

// Crucible active check (param1 channel — nameplate as argument)
pub const RBTDRM_COLOPHON_CRUCIBLE_ACTIVE: &str = "rbw-cic";

// Access probe colophon (imprint-scoped by role)
pub const RBTDRM_COLOPHON_ACCESS_PROBE: &str = "rbtd-ap";

// Handbook display colophons — one symbolic ref per RBZ_* zipper constant.
// Onboarding group (8)
pub const RBTDRM_COLOPHON_ONBOARD_START_HERE: &str = "rbw-o";
pub const RBTDRM_COLOPHON_ONBOARD_CRASH_COURSE: &str = "rbw-Occ";
pub const RBTDRM_COLOPHON_ONBOARD_CRED_RETRIEVER: &str = "rbw-Ocr";
pub const RBTDRM_COLOPHON_ONBOARD_CRED_DIRECTOR: &str = "rbw-Ocd";
pub const RBTDRM_COLOPHON_ONBOARD_FIRST_CRUCIBLE: &str = "rbw-Ofc";
pub const RBTDRM_COLOPHON_ONBOARD_DIR_FIRST_BUILD: &str = "rbw-Odf";
pub const RBTDRM_COLOPHON_ONBOARD_PAYOR_HB: &str = "rbw-Op";
pub const RBTDRM_COLOPHON_ONBOARD_GOVERNOR_HB: &str = "rbw-Og";
// Windows group (4 of 5 — rbw-HWdw uses param1 channel, deferred to ₢A-AAS)
pub const RBTDRM_COLOPHON_HANDBOOK_TOP: &str = "rbw-h0";
pub const RBTDRM_COLOPHON_HANDBOOK_WINDOWS: &str = "rbw-hw";
pub const RBTDRM_COLOPHON_HW_DOCKER_DESKTOP: &str = "rbw-HWdd";
pub const RBTDRM_COLOPHON_HW_DOCKER_CONTEXT: &str = "rbw-HWdc";
// Payor group (3)
pub const RBTDRM_COLOPHON_PAYOR_ESTABLISH: &str = "rbw-gPE";
pub const RBTDRM_COLOPHON_PAYOR_REFRESH: &str = "rbw-gPR";
pub const RBTDRM_COLOPHON_QUOTA_BUILD: &str = "rbw-gq";

// Fixture name consts — single definition per String Boundary Discipline.
// Crucible fixtures (charge/quench lifecycle)
pub const RBTDRM_FIXTURE_TADMOR: &str = "tadmor";
pub const RBTDRM_FIXTURE_SRJCL: &str = "srjcl";
pub const RBTDRM_FIXTURE_PLUML: &str = "pluml";
// Bare fixtures (GCP credentials, no container runtime)
pub const RBTDRM_FIXTURE_FOUR_MODE: &str = "four-mode";
pub const RBTDRM_FIXTURE_BATCH_VOUCH: &str = "batch-vouch";
pub const RBTDRM_FIXTURE_ACCESS_PROBE: &str = "access-probe";
// Fast fixtures (no external dependencies)
pub const RBTDRM_FIXTURE_ENROLLMENT_VALIDATION: &str = "enrollment-validation";
pub const RBTDRM_FIXTURE_REGIME_VALIDATION: &str = "regime-validation";
pub const RBTDRM_FIXTURE_REGIME_SMOKE: &str = "regime-smoke";
pub const RBTDRM_FIXTURE_HANDBOOK_RENDER: &str = "handbook-render";
// Pristine-lifecycle fixture (gate + SA/depot lifecycle cases)
pub const RBTDRM_FIXTURE_PRISTINE_LIFECYCLE: &str = "pristine-lifecycle";

/// Per-fixture required colophons. Returns None for unknown fixtures.
pub fn rbtdrm_required_colophons(fixture: &str) -> Option<&'static [&'static str]> {
    match fixture {
        RBTDRM_FIXTURE_TADMOR | RBTDRM_FIXTURE_SRJCL | RBTDRM_FIXTURE_PLUML => Some(&[
            RBTDRM_COLOPHON_CHARGE,
            RBTDRM_COLOPHON_QUENCH,
            RBTDRM_COLOPHON_WRIT,
            RBTDRM_COLOPHON_FIAT,
            RBTDRM_COLOPHON_BARK,
            RBTDRM_COLOPHON_CRUCIBLE_ACTIVE,
        ]),
        RBTDRM_FIXTURE_FOUR_MODE => Some(&[
            RBTDRM_COLOPHON_ORDAIN,
            RBTDRM_COLOPHON_ABJURE,
            RBTDRM_COLOPHON_WREST_HALLMARK_IMAGE,
            RBTDRM_COLOPHON_KLUDGE,
            RBTDRM_COLOPHON_SUMMON,
            RBTDRM_COLOPHON_PLUMB_FULL,
            RBTDRM_COLOPHON_PLUMB_COMPACT,
            RBTDRM_COLOPHON_REKON_HALLMARK,
            RBTDRM_COLOPHON_JETTISON_HALLMARK_IMAGE,
        ]),
        RBTDRM_FIXTURE_BATCH_VOUCH => Some(&[
            RBTDRM_COLOPHON_ORDAIN,
            RBTDRM_COLOPHON_ABJURE,
            RBTDRM_COLOPHON_JETTISON_HALLMARK_IMAGE,
            RBTDRM_COLOPHON_VOUCH,
            RBTDRM_COLOPHON_TALLY,
        ]),
        RBTDRM_FIXTURE_ACCESS_PROBE => Some(&[RBTDRM_COLOPHON_ACCESS_PROBE]),
        RBTDRM_FIXTURE_ENROLLMENT_VALIDATION
        | RBTDRM_FIXTURE_REGIME_VALIDATION
        | RBTDRM_FIXTURE_REGIME_SMOKE
        | RBTDRM_FIXTURE_PRISTINE_LIFECYCLE => Some(&[]),
        RBTDRM_FIXTURE_HANDBOOK_RENDER => Some(&[
            RBTDRM_COLOPHON_ONBOARD_START_HERE,
            RBTDRM_COLOPHON_ONBOARD_CRASH_COURSE,
            RBTDRM_COLOPHON_ONBOARD_CRED_RETRIEVER,
            RBTDRM_COLOPHON_ONBOARD_CRED_DIRECTOR,
            RBTDRM_COLOPHON_ONBOARD_FIRST_CRUCIBLE,
            RBTDRM_COLOPHON_ONBOARD_DIR_FIRST_BUILD,
            RBTDRM_COLOPHON_ONBOARD_PAYOR_HB,
            RBTDRM_COLOPHON_ONBOARD_GOVERNOR_HB,
            RBTDRM_COLOPHON_HANDBOOK_TOP,
            RBTDRM_COLOPHON_HANDBOOK_WINDOWS,
            RBTDRM_COLOPHON_HW_DOCKER_DESKTOP,
            RBTDRM_COLOPHON_HW_DOCKER_CONTEXT,
            RBTDRM_COLOPHON_PAYOR_ESTABLISH,
            RBTDRM_COLOPHON_PAYOR_REFRESH,
            RBTDRM_COLOPHON_QUOTA_BUILD,
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
