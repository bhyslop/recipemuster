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
// Tests for rbtdrk_canonical — gauntlet canonical-establish fixture.

use std::path::PathBuf;

use crate::rbtdrc_crucible::rbtdrc_lookup_fixture;
use crate::rbtdre_engine::rbtdre_Disposition;
use crate::rbtdrk_canonical::{
    rbtdrk_install_canonical_prefixes, RBTDRK_CANONICAL_CLOUD_PREFIX,
    RBTDRK_CANONICAL_RUNTIME_PREFIX, RBTDRK_FAMILY_STEM,
};
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_CANONICAL_ESTABLISH;

/// Canonical-prefix marker shape: lowercase letters ending in hyphen, distinct
/// pair, distinct from pristine throwaway markers. Cases 2-4 rely on these
/// shapes for state detection in rbrr.env.
#[test]
fn rbtdtk_canonical_prefix_shape() {
    assert!(RBTDRK_CANONICAL_CLOUD_PREFIX.ends_with('-'));
    assert!(RBTDRK_CANONICAL_RUNTIME_PREFIX.ends_with('-'));
    assert_ne!(
        RBTDRK_CANONICAL_CLOUD_PREFIX,
        RBTDRK_CANONICAL_RUNTIME_PREFIX
    );
    assert!(RBTDRK_CANONICAL_CLOUD_PREFIX
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-'));
    assert!(RBTDRK_CANONICAL_RUNTIME_PREFIX
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-'));
}

/// Family stem 'canest2' is a contract: case 2's probe matches monikers
/// starting with this exact string.
#[test]
fn rbtdtk_family_stem_value() {
    assert_eq!(RBTDRK_FAMILY_STEM, "canest2");
}

/// Canonical-establish is StateProgressing — the engine's keep-going
/// refusal applies to this fixture too, by design (per BBAAd policy gate).
#[test]
fn rbtdtk_disposition_is_state_progressing() {
    let fixture = rbtdrc_lookup_fixture(RBTDRM_FIXTURE_CANONICAL_ESTABLISH)
        .expect("canonical-establish is registered");
    assert_eq!(fixture.disposition, rbtdre_Disposition::StateProgressing);
}

/// Sections lookup binds the fixture name to the registry array and yields
/// exactly four cases under one section ("canonical-establish-arc").
#[test]
fn rbtdtk_sections_registered() {
    let fixture = rbtdrc_lookup_fixture(RBTDRM_FIXTURE_CANONICAL_ESTABLISH)
        .expect("canonical-establish is registered");
    let sections = fixture.sections;
    assert_eq!(sections.len(), 1, "expected one section");
    assert_eq!(sections[0].name, "canonical-establish-arc");
    assert_eq!(sections[0].cases.len(), 4, "expected four cases");
    let names: Vec<&str> = sections[0].cases.iter().map(|c| c.name).collect();
    assert!(names.iter().any(|n| n.contains("rbtdrk_depot_levy")));
    assert!(names.iter().any(|n| n.contains("rbtdrk_governor_mantle")));
    assert!(names.iter().any(|n| n.contains("rbtdrk_retriever_invest")));
    assert!(names.iter().any(|n| n.contains("rbtdrk_director_invest")));
}

/// install_canonical_prefixes refuses cleanly when rbrr.env is absent — the
/// helper does not silently succeed against a missing regime file.
#[test]
fn rbtdtk_install_canonical_prefixes_rejects_missing_rbrr() {
    let tmp: PathBuf = std::env::temp_dir().join("rbtdtk-nonexistent-root-xyz");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("create tempdir");
    let result = rbtdrk_install_canonical_prefixes(&tmp);
    let _ = std::fs::remove_dir_all(&tmp);
    assert!(result.is_err(), "expected Err when .rbk/rbrr.env is absent");
}
