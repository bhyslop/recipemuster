// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjri_io — the size gate's emission.

use crate::jjri_io::{jjri_commit_refusal, jjri_size_interdictum};
use vvc::{vvcg_Cost, vvcg_StagedFile, vvcm_CommitError};

fn jjti_cost(entries: &[(&str, u64)]) -> vvcg_Cost {
    let files: Vec<vvcg_StagedFile> = entries
        .iter()
        .map(|(p, s)| vvcg_StagedFile { path: p.to_string(), size: *s })
        .collect();
    vvcg_Cost {
        total: files.iter().map(|f| f.size).sum(),
        files,
    }
}

/// The interdictum recognition law (JJS0 `jjdz_interdictum`): the token leads, with
/// nothing ahead of it. Message self-sufficiency (also normative): standing agent
/// context carries nothing about this guard, so the body alone must name what refused,
/// why — the bytes against the ceiling, and which files carry them — and the remedies.
#[test]
fn jjti_size_refusal_leads_with_interdictum_token_and_stands_alone() {
    let cost = jjti_cost(&[("Tools/big.bin", 61_000), ("Tools/small.rs", 400)]);
    let msg = jjri_size_interdictum("jjx_record", &cost, 50_000);

    assert!(msg.starts_with("INTERDICTUM — "), "token must lead: {}", msg);
    assert!(msg.contains("jjx_record"), "names the act that refused: {}", msg);
    assert!(msg.contains("61400"), "names the staged cost: {}", msg);
    assert!(msg.contains("50000"), "names the ceiling: {}", msg);
    assert!(msg.contains("Tools/big.bin"), "breaks the cost down by file: {}", msg);
    assert!(msg.contains("Remed"), "names the remedies: {}", msg);
}

/// The breakdown reads largest-first and truncates, so a wide staging still explains
/// its total in the lines that matter.
#[test]
fn jjti_size_refusal_truncates_a_long_breakdown() {
    let entries: Vec<(String, u64)> = (0..14).map(|i| (format!("f{}", i), 10_000 - i * 100)).collect();
    let cost = jjti_cost(&entries.iter().map(|(p, s)| (p.as_str(), *s)).collect::<Vec<_>>());
    let msg = jjri_size_interdictum("jjx_close", &cost, 50_000);

    assert!(msg.contains("f0"), "keeps the largest: {}", msg);
    assert!(!msg.contains("f13"), "drops the tail: {}", msg);
    assert!(msg.contains("... and 4 more files"), "counts what it dropped: {}", msg);
}

/// Only the size gate speaks the token. An ordinary commit fault is a plain error and
/// keeps the command name in front — a fault the caller can correct is not a gating
/// verdict, and must not read as one.
#[test]
fn jjti_commit_fault_is_not_an_interdictum() {
    let over = vvcm_CommitError::OverLimit {
        cost: jjti_cost(&[("Tools/big.bin", 61_000)]),
        limit: 50_000,
    };
    let fault = vvcm_CommitError::Fault("git commit failed".to_string());

    assert!(jjri_commit_refusal("jjx_enroll", &over).starts_with("INTERDICTUM — "));

    let plain = jjri_commit_refusal("jjx_enroll", &fault);
    assert!(!plain.contains("INTERDICTUM"), "a fault never speaks the token: {}", plain);
    assert_eq!(plain, "jjx_enroll: error: git commit failed");
}
