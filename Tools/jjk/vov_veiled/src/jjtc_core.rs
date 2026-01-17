// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrc_core::*;

#[test]
fn jjtc_default_path() {
    let path = jjrc_default_gallops_path();
    assert!(path.to_str().unwrap().contains("jjg_gallops.json"));
}
