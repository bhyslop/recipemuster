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
            jjrfr_RejectionKind::Conflict => kind,
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
        jjrfr_RejectionKind::Conflict,
    ];
    for k in kinds {
        jjtfr_exhaustive(k);
    }
    let strs: Vec<&str> = kinds.iter().map(|k| k.jjrfr_as_str()).collect();
    assert_eq!(
        strs,
        ["foreign-ground", "dirty-tree", "diverged", "lock-held", "lock-broken", "seat-vestige", "line-seated", "conflict"]
    );
    for s in &strs {
        assert!(!s.contains("git"), "rejection kind string must stay git-free: {}", s);
    }
}

#[test]
fn jjtfr_rejection_display_carries_op_repo_monitum() {
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
fn jjtfr_display_renders_the_monitum_never_the_diagnostic() {
    // The render contract's whole guarantee: the operator face is the monitum
    // alone, and the raw foreign evidence — ref path, remote URL, git's own
    // rejection prose — reaches only the diagnostic accessor, never Display. A
    // render surface cannot leak what the render has no path to, which is what
    // makes a leak a test (and construction) failure rather than a review catch.
    let leak = "! [rejected] refs/jjv/guidon (stale info)\n\
                error: failed to push some refs to 'git@host:repo.git'";
    let rejection = jjrfr_Rejection::jjrfr_new(
        jjrfr_RejectionKind::LockBroken,
        "proffer",
        PathBuf::from("/tmp/example-repo"),
        "the studbook lock was severed under this write — re-run the command.",
    )
    .jjrfr_with_diagnostic(leak);

    let rendered = format!("{}", rejection);
    assert!(rendered.contains("the studbook lock was severed"), "the monitum is the face: {}", rendered);
    for token in ["[rejected]", "refs/jjv/guidon", "stale info", "git@host", "failed to push"] {
        assert!(!rendered.contains(token), "the operator face must not carry '{}': {}", token, rendered);
    }
    assert_eq!(rejection.jjrfr_diagnostic(), Some(leak), "the raw evidence survives behind the accessor");
}

#[test]
fn jjtfr_a_rejection_without_diagnostic_reports_none() {
    // A composed refusal has no foreign evidence to sink — absence is real and
    // typed, not an empty-string stand-in the journal sink must filter.
    let rejection = jjrfr_Rejection::jjrfr_new(
        jjrfr_RejectionKind::DirtyTree,
        "billet_remove",
        PathBuf::from("/r"),
        "uncommitted changes block reaping the billet",
    );
    assert_eq!(rejection.jjrfr_diagnostic(), None);
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
