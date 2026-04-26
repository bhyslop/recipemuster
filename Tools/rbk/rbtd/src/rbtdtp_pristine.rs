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
    rbtdrp_install_throwaway_prefixes, RBTDRP_THROWAWAY_CLOUD_PREFIX,
    RBTDRP_THROWAWAY_RUNTIME_PREFIX,
};

/// Throwaway-prefix marker shape: lowercase letters ending in hyphen, distinct
/// from any canonical operator prefix. Asserted as the contract Cases 2-5
/// rely on for state-detection.
#[test]
fn rbtdtp_throwaway_prefix_shape() {
    assert!(RBTDRP_THROWAWAY_CLOUD_PREFIX.ends_with('-'));
    assert!(RBTDRP_THROWAWAY_RUNTIME_PREFIX.ends_with('-'));
    assert_ne!(
        RBTDRP_THROWAWAY_CLOUD_PREFIX,
        RBTDRP_THROWAWAY_RUNTIME_PREFIX
    );
    assert!(
        RBTDRP_THROWAWAY_CLOUD_PREFIX
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-')
    );
    assert!(
        RBTDRP_THROWAWAY_RUNTIME_PREFIX
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-')
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
