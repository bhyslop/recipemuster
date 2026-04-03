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
// RBTDTM — tests for colophon manifest verification

use super::rbtdrm_manifest::*;

/// Build a manifest string from the required colophons for a fixture.
fn rbtdtm_manifest_for(fixture: &str) -> String {
    rbtdrm_required_colophons(fixture)
        .expect("unknown fixture")
        .join(" ")
}

#[test]
fn rbtdtm_accepts_valid_crucible_manifest() {
    let manifest = rbtdtm_manifest_for(RBTDRM_FIXTURE_TADMOR);
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_TADMOR).is_ok());
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_SRJCL).is_ok());
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_PLUML).is_ok());
}

#[test]
fn rbtdtm_accepts_valid_fourmode_manifest() {
    let manifest = rbtdtm_manifest_for(RBTDRM_FIXTURE_FOUR_MODE);
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_FOUR_MODE).is_ok());
}

#[test]
fn rbtdtm_accepts_valid_accessprobe_manifest() {
    let manifest = rbtdtm_manifest_for(RBTDRM_FIXTURE_ACCESS_PROBE);
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_ACCESS_PROBE).is_ok());
}

#[test]
fn rbtdtm_rejects_missing_colophon() {
    // Manifest with only charge — missing quench and others
    let manifest = RBTDRM_COLOPHON_CHARGE;
    let err = rbtdrm_verify(manifest, RBTDRM_FIXTURE_TADMOR).unwrap_err();
    assert!(err.contains(RBTDRM_COLOPHON_QUENCH));
}

#[test]
fn rbtdtm_rejects_empty_manifest() {
    let err = rbtdrm_verify("", RBTDRM_FIXTURE_TADMOR).unwrap_err();
    assert!(err.contains(RBTDRM_COLOPHON_CHARGE));
}

#[test]
fn rbtdtm_no_partial_match() {
    let manifest = format!("{}X {}Y", RBTDRM_COLOPHON_CHARGE, RBTDRM_COLOPHON_QUENCH);
    let err = rbtdrm_verify(&manifest, RBTDRM_FIXTURE_TADMOR).unwrap_err();
    assert!(err.contains(RBTDRM_COLOPHON_CHARGE));
}

#[test]
fn rbtdtm_rejects_unknown_fixture() {
    let err = rbtdrm_verify(RBTDRM_COLOPHON_CHARGE, "nonexistent").unwrap_err();
    assert!(err.contains("unknown fixture"));
}

#[test]
fn rbtdtm_accepts_fast_fixtures() {
    // Fast-tier fixtures have no colophon requirements — empty manifest passes
    let manifest = rbtdtm_manifest_for(RBTDRM_FIXTURE_ENROLLMENT_VALIDATION);
    assert!(manifest.is_empty());
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_ENROLLMENT_VALIDATION).is_ok());
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_REGIME_VALIDATION).is_ok());
    assert!(rbtdrm_verify(&manifest, RBTDRM_FIXTURE_REGIME_SMOKE).is_ok());
}
