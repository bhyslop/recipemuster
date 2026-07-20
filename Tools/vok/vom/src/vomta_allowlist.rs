// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomra_allowlist.

use super::vomra_allowlist::*;
use std::path::Path;

#[test]
fn vomta_is_allowed_matches_shapes() {
    assert!(voma_is_allowed(Path::new(
        "Tools/vok/vom/src/vomrm_matricula.rs"
    )));
    assert!(voma_is_allowed(Path::new("README.md")));
    assert!(voma_is_allowed(Path::new(
        "Tools/vok/vov_veiled/VOSMM-entity.adoc"
    )));
    assert!(voma_is_allowed(Path::new("tt/vow-mb.MatriculaBuild.sh")));
}

#[test]
fn vomta_is_allowed_rejects_other_shapes() {
    assert!(!voma_is_allowed(Path::new("Cargo.lock")));
    assert!(!voma_is_allowed(Path::new("image.png")));
    assert!(!voma_is_allowed(Path::new("no_extension")));
}

#[test]
fn vomta_is_allowed_rejects_memos_as_reference_only() {
    assert!(!voma_is_allowed(Path::new(
        "Memos/memo-20260620-freeze-builder-pattern/README.md"
    )));
    assert!(!voma_is_allowed(Path::new("Memos/memo-example.adoc")));
}

#[test]
fn vomta_is_allowed_rejects_historical_prose_as_reference_only() {
    assert!(!voma_is_allowed(Path::new(
        ".claude/jjm/retired/jjh_b260114-r260124-example.md"
    )));
    assert!(!voma_is_allowed(Path::new(
        "Study/study-net-namespace-permutes/old/example.sh"
    )));
}

// eof
