// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrm_matricula.

use super::vomrm_matricula::*;

#[test]
fn vomtm_identity_names_crate_and_links_vof() {
    let identity = vomrm_identity();
    assert!(
        identity.starts_with("vom "),
        "identity line must name the crate: {identity}"
    );
    // The vof path-dependency is live: the vo cipher's project resolves.
    assert!(
        identity.contains("Vox Obscura"),
        "vof link must resolve the vo cipher project: {identity}"
    );
}

// eof
