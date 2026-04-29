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
// Tests for rbtdro_onboarding — gauntlet onboarding-sequence fixture.

use crate::rbtdrc_crucible::rbtdrc_lookup_fixture;
use crate::rbtdre_engine::rbtdre_Disposition;
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_ONBOARDING_SEQUENCE;

/// Onboarding-sequence is StateProgressing — case N's hallmark/scratch
/// state establishes preconditions for case N+1, so engine keep-going is refused.
#[test]
fn rbtdto_disposition_is_state_progressing() {
    let fix = rbtdrc_lookup_fixture(RBTDRM_FIXTURE_ONBOARDING_SEQUENCE)
        .expect("onboarding-sequence is registered");
    assert_eq!(fix.disposition, rbtdre_Disposition::StateProgressing);
}

/// Sections lookup binds the fixture name to the registry array and yields
/// exactly nine cases under one section ("onboarding-arc").
#[test]
fn rbtdto_sections_registered() {
    let fix = rbtdrc_lookup_fixture(RBTDRM_FIXTURE_ONBOARDING_SEQUENCE)
        .expect("onboarding-sequence is registered");
    let sections = fix.sections;
    assert_eq!(sections.len(), 1, "expected one section");
    assert_eq!(sections[0].name, "onboarding-arc");
    assert_eq!(sections[0].cases.len(), 9, "expected nine cases");
    let names: Vec<&str> = sections[0].cases.iter().map(|c| c.name).collect();
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_inscribe_reliquary")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_enshrine_bases")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_kludge_tadmor")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_kludge_ccyolo")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_ordain_conjure")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_conjure_srjcl")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_ordain_airgap")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_ordain_bind")));
    assert!(names.iter().any(|n| n.contains("rbtdro_onboarding_ordain_graft")));
}

/// Case order is load-bearing for StateProgressing — inscribe must precede
/// all reliquary-consuming cases (kludge, conjure, srjcl, airgap, bind).
#[test]
fn rbtdto_inscribe_precedes_reliquary_consumers() {
    let fix = rbtdrc_lookup_fixture(RBTDRM_FIXTURE_ONBOARDING_SEQUENCE)
        .expect("onboarding-sequence is registered");
    let sections = fix.sections;
    let names: Vec<&str> = sections[0].cases.iter().map(|c| c.name).collect();
    let inscribe_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_inscribe_reliquary"))
        .expect("inscribe case present");
    let enshrine_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_enshrine_bases"))
        .expect("enshrine-bases case present");
    let kludge_tadmor_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_kludge_tadmor"))
        .expect("kludge-tadmor case present");
    let kludge_ccyolo_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_kludge_ccyolo"))
        .expect("kludge-ccyolo case present");
    let conjure_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_ordain_conjure"))
        .expect("ordain-conjure case present");
    let srjcl_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_conjure_srjcl"))
        .expect("conjure-srjcl case present");
    let airgap_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_ordain_airgap"))
        .expect("airgap case present");
    let bind_idx = names
        .iter()
        .position(|n| n.contains("rbtdro_onboarding_ordain_bind"))
        .expect("ordain-bind case present");
    assert!(inscribe_idx < enshrine_idx);
    assert!(inscribe_idx < kludge_tadmor_idx);
    assert!(inscribe_idx < kludge_ccyolo_idx);
    assert!(inscribe_idx < conjure_idx);
    assert!(inscribe_idx < srjcl_idx);
    assert!(inscribe_idx < airgap_idx);
    assert!(inscribe_idx < bind_idx);
}
