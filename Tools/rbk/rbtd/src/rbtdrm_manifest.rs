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
pub const RBTDRM_COLOPHON_ORDAIN: &str = "rbw-DO";
pub const RBTDRM_COLOPHON_ABJURE: &str = "rbw-DA";
pub const RBTDRM_COLOPHON_WREST: &str = "rbw-Rw";
pub const RBTDRM_COLOPHON_TALLY: &str = "rbw-Dt";
pub const RBTDRM_COLOPHON_KLUDGE: &str = "rbw-LK";

// Crucible active check (param1 channel — nameplate as argument)
pub const RBTDRM_COLOPHON_CRUCIBLE_ACTIVE: &str = "rbw-ca";

// Access probe colophon (imprint-scoped by role)
pub const RBTDRM_COLOPHON_ACCESS_PROBE: &str = "rbtd-ap";

// Fixture name consts — single definition per String Boundary Discipline.
// Crucible fixtures (charge/quench lifecycle)
pub const RBTDRM_FIXTURE_TADMOR: &str = "tadmor";
pub const RBTDRM_FIXTURE_SRJCL: &str = "srjcl";
pub const RBTDRM_FIXTURE_PLUML: &str = "pluml";
// Bare fixtures (GCP credentials, no container runtime)
pub const RBTDRM_FIXTURE_FOUR_MODE: &str = "four-mode";
pub const RBTDRM_FIXTURE_ACCESS_PROBE: &str = "access-probe";
// Fast fixtures (no external dependencies)
pub const RBTDRM_FIXTURE_ENROLLMENT_VALIDATION: &str = "enrollment-validation";
pub const RBTDRM_FIXTURE_REGIME_VALIDATION: &str = "regime-validation";
pub const RBTDRM_FIXTURE_REGIME_SMOKE: &str = "regime-smoke";

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
            RBTDRM_COLOPHON_WREST,
            RBTDRM_COLOPHON_TALLY,
            RBTDRM_COLOPHON_KLUDGE,
        ]),
        RBTDRM_FIXTURE_ACCESS_PROBE => Some(&[RBTDRM_COLOPHON_ACCESS_PROBE]),
        RBTDRM_FIXTURE_ENROLLMENT_VALIDATION
        | RBTDRM_FIXTURE_REGIME_VALIDATION
        | RBTDRM_FIXTURE_REGIME_SMOKE => Some(&[]),
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
