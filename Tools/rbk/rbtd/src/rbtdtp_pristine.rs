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
// Tests for rbtdrp_pristine — pristine-lifecycle fixture helpers.

use std::path::PathBuf;

use crate::rbtdrp_pristine::{
    rbtdrp_family_stem_arc, rbtdrp_install_throwaway_prefixes, rbtdrp_throwaway_cloud_prefix,
    rbtdrp_throwaway_runtime_prefix, RBTDRP_THROWAWAY_CLOUD_BASE,
    RBTDRP_THROWAWAY_RUNTIME_BASE,
};

/// Throwaway-prefix base shape: lowercase letters, distinct cloud/runtime
/// pair, no trailing hyphen (the composer adds it). Cases 2-5 rely on the
/// composed form for state-detection.
#[test]
fn rbtdtp_throwaway_base_shape() {
    assert_ne!(RBTDRP_THROWAWAY_CLOUD_BASE, RBTDRP_THROWAWAY_RUNTIME_BASE);
    assert!(!RBTDRP_THROWAWAY_CLOUD_BASE.ends_with('-'));
    assert!(!RBTDRP_THROWAWAY_RUNTIME_BASE.ends_with('-'));
    assert!(RBTDRP_THROWAWAY_CLOUD_BASE
        .chars()
        .all(|c| c.is_ascii_lowercase()));
    assert!(RBTDRP_THROWAWAY_RUNTIME_BASE
        .chars()
        .all(|c| c.is_ascii_lowercase()));
}

/// Composed throwaway prefix shape: tinctured base ends with a hyphen and
/// contains only lowercase alphanumeric (plus the trailing hyphen).
#[test]
fn rbtdtp_throwaway_prefix_compose() {
    let cloud = rbtdrp_throwaway_cloud_prefix("xyz");
    let runtime = rbtdrp_throwaway_runtime_prefix("xyz");
    assert!(cloud.ends_with('-'));
    assert!(runtime.ends_with('-'));
    assert_ne!(cloud, runtime);
    assert!(cloud.starts_with(RBTDRP_THROWAWAY_CLOUD_BASE));
    assert!(runtime.starts_with(RBTDRP_THROWAWAY_RUNTIME_BASE));
    assert!(cloud.contains("xyz"));
    assert!(runtime.contains("xyz"));
}

/// Distinct tinctures yield disjoint composed prefixes — the load-bearing
/// disjointness property for parallel-station runs on a shared payor manor.
#[test]
fn rbtdtp_throwaway_prefix_disjoint_per_tincture() {
    assert_ne!(
        rbtdrp_throwaway_cloud_prefix("aa"),
        rbtdrp_throwaway_cloud_prefix("bb")
    );
    assert_ne!(
        rbtdrp_throwaway_runtime_prefix("aa"),
        rbtdrp_throwaway_runtime_prefix("bb")
    );
}

/// Dual-station dry-run for the pristine fixture: two distinct tinctures
/// produce disjoint depot project IDs, GAR repos, GCS buckets, and SA
/// emails — the wrap criterion for ₢BBABB. Mirrors RBDC composition rules
/// (rbdc_DerivedConstants.sh) without invoking GCP.
#[test]
fn rbtdtp_pristine_dual_station_disjoint() {
    let (a, b) = ("aaa", "bbb");
    let cloud_a = rbtdrp_throwaway_cloud_prefix(a);
    let cloud_b = rbtdrp_throwaway_cloud_prefix(b);
    let moniker_a = format!("{}100000", rbtdrp_family_stem_arc(a));
    let moniker_b = format!("{}100000", rbtdrp_family_stem_arc(b));

    // Mirror RBDC: project_id = cloud + "d-" + moniker
    let project_a = format!("{}d-{}", cloud_a, moniker_a);
    let project_b = format!("{}d-{}", cloud_b, moniker_b);
    assert_ne!(project_a, project_b);
    assert!(project_a.len() <= 30, "project_a {} > 30", project_a);
    assert!(project_b.len() <= 30, "project_b {} > 30", project_b);

    // Mirror RBDC: gar_repo = cloud + moniker + "-gar"
    assert_ne!(
        format!("{}{}-gar", cloud_a, moniker_a),
        format!("{}{}-gar", cloud_b, moniker_b)
    );

    // Mirror RBDC: bucket = cloud + "b-" + moniker
    assert_ne!(
        format!("{}b-{}", cloud_a, moniker_a),
        format!("{}b-{}", cloud_b, moniker_b)
    );

    // SA email = sa_name + "@" + project_id (sa_name unchanged per docket
    // out-of-scope; disjointness comes from project_id)
    assert_ne!(
        format!("pristl-ret@{}.iam.gserviceaccount.com", project_a),
        format!("pristl-ret@{}.iam.gserviceaccount.com", project_b)
    );
}

/// Reading rbrr.env from a non-existent path returns Err — the helper does
/// not silently succeed when the regime file is missing.
#[test]
fn rbtdtp_install_throwaway_prefixes_rejects_missing_rbrr() {
    let tmp: PathBuf = std::env::temp_dir().join("rbtdtp-nonexistent-root-xyz");
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("create tempdir");
    let result = rbtdrp_install_throwaway_prefixes(&tmp);
    let _ = std::fs::remove_dir_all(&tmp);
    assert!(
        result.is_err(),
        "expected Err when .rbk/rbrr.env is absent"
    );
}
