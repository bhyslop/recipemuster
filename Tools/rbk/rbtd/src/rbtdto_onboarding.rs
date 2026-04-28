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
// Tests for rbtdro_onboarding — gauntlet canonical-onboarding-sequence fixture.

use crate::rbtdrc_crucible::{rbtdrc_fixture_disposition, rbtdrc_sections_for_fixture};
use crate::rbtdre_engine::rbtdre_Disposition;
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE;

/// Canonical-onboarding-sequence is StateProgressing — case N's hallmark/scratch
/// state establishes preconditions for case N+1, so engine keep-going is refused.
#[test]
fn rbtdto_disposition_is_state_progressing() {
    assert_eq!(
        rbtdrc_fixture_disposition(RBTDRM_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE),
        rbtdre_Disposition::StateProgressing
    );
}

/// Sections lookup binds the fixture name to the registry array and yields
/// exactly five cases (1 inscribe + 4 ordain) under one section
/// ("canonical-onboarding-arc").
#[test]
fn rbtdto_sections_registered() {
    let sections = rbtdrc_sections_for_fixture(RBTDRM_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE);
    assert_eq!(sections.len(), 1, "expected one section");
    assert_eq!(sections[0].name, "canonical-onboarding-arc");
    assert_eq!(sections[0].cases.len(), 5, "expected five cases");
    let names: Vec<&str> = sections[0].cases.iter().map(|c| c.name).collect();
    assert!(names.iter().any(|n| n.contains("rbtdro_inscribe_reliquary")));
    assert!(names.iter().any(|n| n.contains("rbtdro_ordain_conjure")));
    assert!(names.iter().any(|n| n.contains("rbtdro_ordain_airgap")));
    assert!(names.iter().any(|n| n.contains("rbtdro_ordain_bind")));
    assert!(names.iter().any(|n| n.contains("rbtdro_ordain_graft")));
}

/// Case order is load-bearing for StateProgressing — inscribe must precede
/// the conjure/airgap cases that read the reliquary scratch.
#[test]
fn rbtdto_inscribe_precedes_reliquary_consumers() {
    let sections = rbtdrc_sections_for_fixture(RBTDRM_FIXTURE_CANONICAL_ONBOARDING_SEQUENCE);
    let names: Vec<&str> = sections[0].cases.iter().map(|c| c.name).collect();
    let inscribe_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_inscribe_reliquary"))
        .expect("inscribe case present");
    let conjure_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_ordain_conjure"))
        .expect("conjure case present");
    let airgap_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_ordain_airgap"))
        .expect("airgap case present");
    assert!(inscribe_idx < conjure_idx);
    assert!(inscribe_idx < airgap_idx);
}
