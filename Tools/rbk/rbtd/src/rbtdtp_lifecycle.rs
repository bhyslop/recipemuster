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
// Tests for rbtdrp_lifecycle — the depot-lifecycle fixture.
//
// The freehold-scheme machinery this fixture rides (prefix compose, family
// stem, install rejection, dual-station disjointness) is exercised once, in
// rbtdtk_freehold; these tests cover only what is lifecycle-specific.

use crate::rbtdre_engine::rbtdre_Disposition;
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_DEPOT_LIFECYCLE;
use crate::rbtdth_helpers::{
    rbtdth_assert_cases,
    rbtdth_assert_disposition,
};

/// depot-lifecycle is StateProgressing — the gauntlet's entry-gate fixture, so
/// the engine's keep-going refusal applies (case 1 failure short-circuits the arc).
#[test]
fn rbtdtp_disposition_is_state_progressing() {
    rbtdth_assert_disposition(
        RBTDRM_FIXTURE_DEPOT_LIFECYCLE,
        rbtdre_Disposition::StateProgressing,
    );
}

/// Case lookup binds the fixture name to the registry array and yields exactly
/// the four lifecycle cases (gate → stand-up → live-disqualify → tear-down).
#[test]
fn rbtdtp_cases_registered() {
    rbtdth_assert_cases(
        RBTDRM_FIXTURE_DEPOT_LIFECYCLE,
        4,
        &[
            "rbtdrp_marshal_zero_attestation",
            "rbtdrp_depot_stand_up",
            "rbtdrp_depot_live_disqualify",
            "rbtdrp_depot_tear_down",
        ],
    );
}
