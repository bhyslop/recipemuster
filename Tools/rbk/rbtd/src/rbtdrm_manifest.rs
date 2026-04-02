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
pub const RBTDRM_COLOPHON_KLUDGE: &str = "rbw-ak";

// Access probe colophon (imprint-scoped by role)
pub const RBTDRM_COLOPHON_ACCESS_PROBE: &str = "rbtd-ap";

/// Per-fixture required colophons. Returns None for unknown fixtures.
pub fn rbtdrm_required_colophons(fixture: &str) -> Option<&'static [&'static str]> {
    match fixture {
        "tadmor" | "srjcl" | "pluml" => Some(&[
            RBTDRM_COLOPHON_CHARGE,
            RBTDRM_COLOPHON_QUENCH,
            RBTDRM_COLOPHON_WRIT,
            RBTDRM_COLOPHON_FIAT,
            RBTDRM_COLOPHON_BARK,
        ]),
        "four-mode" => Some(&[
            RBTDRM_COLOPHON_ORDAIN,
            RBTDRM_COLOPHON_ABJURE,
            RBTDRM_COLOPHON_WREST,
            RBTDRM_COLOPHON_TALLY,
            RBTDRM_COLOPHON_KLUDGE,
        ]),
        "access-probe" => Some(&[RBTDRM_COLOPHON_ACCESS_PROBE]),
        // Fast-tier fixtures — no colophon requirements (pure bash/local)
        "enrollment-validation" | "regime-validation" | "regime-smoke" => Some(&[]),
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
