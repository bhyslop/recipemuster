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
pub const RBTDRM_COLOPHON_CHARGE: &str = "rbw-cC";
pub const RBTDRM_COLOPHON_QUENCH: &str = "rbw-cQ";
pub const RBTDRM_COLOPHON_WRIT: &str = "rbw-cw";
pub const RBTDRM_COLOPHON_FIAT: &str = "rbw-cf";
pub const RBTDRM_COLOPHON_BARK: &str = "rbw-cb";

/// Colophon manifest gate — all must appear in the zipper roll at launch.
pub const RBTDRM_REQUIRED_COLOPHONS: &[&str] = &[
    RBTDRM_COLOPHON_CHARGE,
    RBTDRM_COLOPHON_QUENCH,
    RBTDRM_COLOPHON_WRIT,
    RBTDRM_COLOPHON_FIAT,
    RBTDRM_COLOPHON_BARK,
];

/// Verify that all required colophons appear in the zipper manifest string.
pub fn rbtdrm_verify(manifest: &str) -> Result<(), String> {
    for colophon in RBTDRM_REQUIRED_COLOPHONS {
        let found = manifest.split_whitespace().any(|token| token == *colophon);
        if !found {
            return Err(format!(
                "rbtd: colophon '{}' not found in zipper manifest",
                colophon
            ));
        }
    }
    Ok(())
}
