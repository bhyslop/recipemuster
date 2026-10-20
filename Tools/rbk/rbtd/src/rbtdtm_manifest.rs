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
// RBTDTM — manifest-module tests: pin the generated-const path literal

use super::rbtgec_consts::*;
use crate::rbtcea_almanac::RBTCEA_FIXTURES;
use crate::rbtcem_manifest::{rbtcem_permitted_colophons, rbtcem_required_colophons};

/// Pin the one surviving compile-time path literal (the `rbtcee_vessels_dir!`
/// macro in the engine module) to the generated source of truth. The macro must equal
/// `<RBTGEC_MOORINGS_DIR>/<RBTGEC_VESSELS_SUBDIR>`; if rbcc_constants.sh changes
/// either, codegen updates the consts and this test fails until the macro
/// literal is corrected — drift caught at test time, no new dependency.
#[test]
fn rbttem_vessels_dir_matches_generated() {
    assert_eq!(
        crate::rbtcee_vessels_dir!(),
        format!("{}/{}", RBTGEC_MOORINGS_DIR, RBTGEC_VESSELS_SUBDIR)
    );
}

/// A permitted declaration is inert unless the fixture also carries a required
/// entry: `rbtcem_required_colophons` returning `None` disables the census
/// outright, and the positive check reads the permitted set only from inside
/// that `Some` arm. So a fixture declaring permitted colophons with no required
/// entry would silently get no census at all — the permitted list reading as
/// coverage while enforcing nothing. Pin the invariant across the whole roster
/// so that pairing cannot land unnoticed.
#[test]
fn rbttem_permitted_declaration_requires_a_required_entry() {
    for fixture in RBTCEA_FIXTURES {
        let permitted = rbtcem_permitted_colophons(fixture.name);
        assert!(
            permitted.is_empty() || rbtcem_required_colophons(fixture.name).is_some(),
            "fixture '{}' declares permitted colophons {:?} but no required-colophons entry — \
             None disables the census entirely, so the permitted tier would be inert",
            fixture.name,
            permitted
        );
    }
}
