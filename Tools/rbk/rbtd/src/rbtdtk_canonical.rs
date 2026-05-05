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
    rbtdrk_canonical_cloud_prefix, rbtdrk_canonical_runtime_prefix, rbtdrk_family_stem,
    rbtdrk_install_canonical_prefixes, RBTDRK_CANONICAL_CLOUD_BASE,
    RBTDRK_CANONICAL_RUNTIME_BASE, RBTDRK_FAMILY_STEM_BASE,
};
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_CANONICAL_ESTABLISH;

/// Canonical-prefix base shape: lowercase letters, distinct cloud/runtime
/// pair, no trailing hyphen (the composer adds it). Cases 2-4 rely on the
/// composed form for state detection in rbrr.env.
#[test]
fn rbtdtk_canonical_base_shape() {
    assert_ne!(RBTDRK_CANONICAL_CLOUD_BASE, RBTDRK_CANONICAL_RUNTIME_BASE);
    assert!(!RBTDRK_CANONICAL_CLOUD_BASE.ends_with('-'));
    assert!(!RBTDRK_CANONICAL_RUNTIME_BASE.ends_with('-'));
    assert!(RBTDRK_CANONICAL_CLOUD_BASE
        .chars()
        .all(|c| c.is_ascii_lowercase()));
    assert!(RBTDRK_CANONICAL_RUNTIME_BASE
        .chars()
        .all(|c| c.is_ascii_lowercase()));
}

/// Composed canonical prefix shape: tinctured base ends with a hyphen and
/// stays disjoint between cloud and runtime.
#[test]
fn rbtdtk_canonical_prefix_compose() {
    let cloud = rbtdrk_canonical_cloud_prefix("xyz");
    let runtime = rbtdrk_canonical_runtime_prefix("xyz");
    assert!(cloud.ends_with('-'));
    assert!(runtime.ends_with('-'));
    assert_ne!(cloud, runtime);
    assert!(cloud.starts_with(RBTDRK_CANONICAL_CLOUD_BASE));
    assert!(runtime.starts_with(RBTDRK_CANONICAL_RUNTIME_BASE));
    assert!(cloud.contains("xyz"));
    assert!(runtime.contains("xyz"));
}

/// Family-stem base 'canest2' is a contract: composed family stems start
/// with this exact string.
#[test]
fn rbtdtk_family_stem_value() {
    assert_eq!(RBTDRK_FAMILY_STEM_BASE, "canest2");
    assert_eq!(rbtdrk_family_stem("xyz"), "canest2xyz");
}

/// Distinct tinctures yield disjoint composed names (prefixes and family
/// stems) — the load-bearing disjointness property for parallel-station
/// runs on a shared payor manor.
#[test]
fn rbtdtk_canonical_disjoint_per_tincture() {
    assert_ne!(
        rbtdrk_canonical_cloud_prefix("aa"),
        rbtdrk_canonical_cloud_prefix("bb")
    );
    assert_ne!(
        rbtdrk_canonical_runtime_prefix("aa"),
        rbtdrk_canonical_runtime_prefix("bb")
    );
    assert_ne!(rbtdrk_family_stem("aa"), rbtdrk_family_stem("bb"));
}

/// Dual-station dry-run for the canonical fixture: two distinct tinctures
/// produce disjoint depot project IDs, GAR repos, GCS buckets, and SA
/// emails — the wrap criterion for ₢BBABB. Mirrors RBDC composition rules
/// (rbdc_DerivedConstants.sh) without invoking GCP.
#[test]
fn rbtdtk_canonical_dual_station_disjoint() {
    let (a, b) = ("aaa", "bbb");
    let cloud_a = rbtdrk_canonical_cloud_prefix(a);
    let cloud_b = rbtdrk_canonical_cloud_prefix(b);
    let moniker_a = format!("{}100000", rbtdrk_family_stem(a));
    let moniker_b = format!("{}100000", rbtdrk_family_stem(b));

    let project_a = format!("{}d-{}", cloud_a, moniker_a);
    let project_b = format!("{}d-{}", cloud_b, moniker_b);
    assert_ne!(project_a, project_b);
    assert!(project_a.len() <= 30, "project_a {} > 30", project_a);
    assert!(project_b.len() <= 30, "project_b {} > 30", project_b);

    assert_ne!(
        format!("{}{}-gar", cloud_a, moniker_a),
        format!("{}{}-gar", cloud_b, moniker_b)
    );

    assert_ne!(
        format!("{}b-{}", cloud_a, moniker_a),
        format!("{}b-{}", cloud_b, moniker_b)
    );

    assert_ne!(
        format!("canest-ret@{}.iam.gserviceaccount.com", project_a),
        format!("canest-ret@{}.iam.gserviceaccount.com", project_b)
    );
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
