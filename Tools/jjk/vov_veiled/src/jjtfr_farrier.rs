// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfr_farrier::{
    jjrfr_CombReport,
    jjrfr_Rejection,
    jjrfr_RejectionKind,
};
use std::path::PathBuf;

#[test]
fn jjtfr_rejection_kind_as_str_is_git_free() {
    // Exhaustive match, no wildcard arm: a new jjrfr_RejectionKind variant fails
    // this to compile until it is added both here and to the `kinds` list below.
    fn jjtfr_exhaustive(kind: jjrfr_RejectionKind) -> jjrfr_RejectionKind {
        match kind {
            jjrfr_RejectionKind::ForeignGround => kind,
            jjrfr_RejectionKind::DirtyTree => kind,
            jjrfr_RejectionKind::Diverged => kind,
            jjrfr_RejectionKind::LockHeld => kind,
            jjrfr_RejectionKind::LockBroken => kind,
            jjrfr_RejectionKind::SeatVestige => kind,
            jjrfr_RejectionKind::LineSeated => kind,
        }
    }
    let kinds = [
        jjrfr_RejectionKind::ForeignGround,
        jjrfr_RejectionKind::DirtyTree,
        jjrfr_RejectionKind::Diverged,
        jjrfr_RejectionKind::LockHeld,
        jjrfr_RejectionKind::LockBroken,
        jjrfr_RejectionKind::SeatVestige,
        jjrfr_RejectionKind::LineSeated,
    ];
    for k in kinds {
        jjtfr_exhaustive(k);
    }
    let strs: Vec<&str> = kinds.iter().map(|k| k.jjrfr_as_str()).collect();
    assert_eq!(
        strs,
        ["foreign-ground", "dirty-tree", "diverged", "lock-held", "lock-broken", "seat-vestige", "line-seated"]
    );
    for s in &strs {
        assert!(!s.contains("git"), "rejection kind string must stay git-free: {}", s);
    }
}

#[test]
fn jjtfr_rejection_display_carries_op_repo_detail() {
    let rejection = jjrfr_Rejection::jjrfr_new(
        jjrfr_RejectionKind::Diverged,
        "consign",
        PathBuf::from("/tmp/example-repo"),
        "the remote moved under us",
    );
    let rendered = format!("{}", rejection);
    assert!(rendered.contains("consign"));
    assert!(rendered.contains("diverged"));
    assert!(rendered.contains("/tmp/example-repo"));
    assert!(rendered.contains("the remote moved under us"));
}

#[test]
fn jjtfr_rejection_is_a_std_error() {
    let rejection = jjrfr_Rejection::jjrfr_new(jjrfr_RejectionKind::LockHeld, "stake", PathBuf::from("/r"), "held");
    let as_error: &dyn std::error::Error = &rejection;
    assert_eq!(as_error.to_string(), format!("{}", rejection));
}

#[test]
fn jjtfr_comb_report_clean_when_no_dirty_paths() {
    let clean = jjrfr_CombReport { dirty_paths: vec![] };
    assert!(clean.jjrfr_is_clean());

    let dirty = jjrfr_CombReport { dirty_paths: vec![PathBuf::from("a.txt")] };
    assert!(!dirty.jjrfr_is_clean());
}
