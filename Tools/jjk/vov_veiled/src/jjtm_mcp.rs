// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the mixed single-heat batch surface (jjx_redocket extended).
//!
//! ## Coverage
//!
//! - jjrm_resolve_batch_firemark: same-firemark guard — agreement, cross-heat
//!   rejection (paddock-vs-reslate, reslate-vs-reslate / the closed mass-reslate
//!   misattribution), and slate-only rejection.
//! - jjrm_apply_batch: a mixed batch applies to one in-memory gallops — reslate
//!   revises, slates append in file order (notice order = pace order).
//!
//! The single-commit-per-batch property is structural: jjrm_apply_batch is a
//! pure transform run inside the one shared dispatch/persist lifecycle (one
//! machine_commit), already covered by the dispatch tests.

use super::jjrm_mcp::{
    jjrm_apply_batch,
    jjrm_resolve_batch_firemark,
    zjjrm_ProcEntry,
    zjjrm_procmap_select,
    ZJJRM_SESSION_ABSENT,
};
use super::jjrz_gazette::{jjrz_BatchInput, jjrz_parse_batch_input};
use super::jjrg_gallops::{jjrg_Gallops, jjrg_Heat, jjrg_Pace, jjrg_Tack, jjrg_HeatStatus, jjrg_PaceState, JJRG_UNKNOWN_BASIS};
use std::collections::BTreeMap;

// ===== Helpers =====

fn make_heat_with_docket(heat_id: &str, docket: &str) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let pace_key = format!("₢{}AAA", heat_id);
    let tack = jjrg_Tack {
        ts: "260101-1200".to_string(),
        state: jjrg_PaceState::Rough,
        text: docket.lines().map(|l| l.to_string()).collect(),
        silks: format!("pace-{}", heat_id),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
    };
    let mut paces = BTreeMap::new();
    paces.insert(pace_key.clone(), jjrg_Pace { tacks: vec![tack] });
    let heat = jjrg_Heat {
        silks: format!("heat-{}", heat_id),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec![pace_key],
        next_pace_seed: "AAB".to_string(),
        paces,
    };
    (heat_key, heat)
}

fn batch(paddock: Option<(&str, &str)>, reslates: &[(&str, &str)], slates: &[(&str, &str)]) -> jjrz_BatchInput {
    jjrz_BatchInput {
        paddock: paddock.map(|(f, c)| (f.to_string(), c.to_string())),
        reslates: reslates.iter().map(|(c, d)| (c.to_string(), d.to_string())).collect(),
        slates: slates.iter().map(|(s, d)| (s.to_string(), d.to_string())).collect(),
    }
}

// ===== Same-firemark guard =====

#[test]
fn jjtm_resolve_agrees_paddock_and_reslate_same_heat() {
    let b = batch(Some(("BD", "body")), &[("₢BDAAb", "d")], &[("new-pace", "d")]);
    let fm = jjrm_resolve_batch_firemark(&b).unwrap();
    assert_eq!(fm.jjrf_display(), "₣BD");
}

#[test]
fn jjtm_resolve_rejects_cross_heat_paddock_vs_reslate() {
    let b = batch(Some(("BD", "body")), &[("₢BEAAb", "d")], &[]);
    let err = jjrm_resolve_batch_firemark(&b).unwrap_err();
    assert!(err.contains("cross-heat batch rejected"), "got: {}", err);
}

#[test]
fn jjtm_resolve_rejects_cross_heat_mass_reslate() {
    // The closed legacy bug: two reslates in different heats. The old path keyed
    // off the first coronet with no check; the guard now rejects.
    let b = batch(None, &[("₢BDAAb", "d"), ("₢BEAAc", "d")], &[]);
    let err = jjrm_resolve_batch_firemark(&b).unwrap_err();
    assert!(err.contains("cross-heat batch rejected"), "got: {}", err);
}

#[test]
fn jjtm_resolve_rejects_slate_only_batch() {
    let b = batch(None, &[], &[("new-pace", "d")]);
    let err = jjrm_resolve_batch_firemark(&b).unwrap_err();
    assert!(err.contains("no heat anchor"), "got: {}", err);
}

// ===== Batch application =====

#[test]
fn jjtm_apply_batch_reslates_and_slates_in_file_order() {
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    };
    let (k, h) = make_heat_with_docket("BD", "## Goal\nold goal");
    gallops.heats.insert(k, h);

    // Mixed batch authored as a gazette: one reslate of the existing pace, then
    // two slates whose silks are deliberately NOT alphabetical.
    let md = "# jjezs_reslate ₢BDAAA\n\n## Goal\nnew goal\n\n# jjezs_slate yankee-pace\n\nfirst slate\n\n# jjezs_slate bravo-pace\n\nsecond slate\n";
    let b = jjrz_parse_batch_input(md).unwrap();
    let fm = jjrm_resolve_batch_firemark(&b).unwrap();

    let out = jjrm_apply_batch(&mut gallops, &b, &fm, None, None, false).unwrap();
    assert!(out.contains("1 reslate"));
    assert!(out.contains("2 slate"));

    let heat = gallops.heats.get("₣BD").unwrap();
    // Original pace + two new ones, the new ones appended in file order.
    assert_eq!(heat.order.len(), 3);
    assert_eq!(heat.order[0], "₢BDAAA");
    // Reslate revised the original docket.
    let revised = &heat.paces.get("₢BDAAA").unwrap().tacks.first().unwrap().text;
    assert!(revised.iter().any(|l| l == "new goal"), "reslate applied");
    // Slates carry their silks, file order preserved (yankee before bravo).
    let s1 = &heat.paces.get(&heat.order[1]).unwrap().tacks.first().unwrap().silks;
    let s2 = &heat.paces.get(&heat.order[2]).unwrap().tacks.first().unwrap().silks;
    assert_eq!(s1, "yankee-pace");
    assert_eq!(s2, "bravo-pace");
}

// ===== session-procmap selection =====

#[test]
fn jjtm_procmap_select_prefers_busy_over_newer_idle() {
    let picked = zjjrm_procmap_select(vec![
        zjjrm_ProcEntry { session_id: "idle-new".to_string(), busy: false, updated_at: 200 },
        zjjrm_ProcEntry { session_id: "busy-old".to_string(), busy: true, updated_at: 100 },
    ]);
    assert_eq!(picked, "busy-old");
}

#[test]
fn jjtm_procmap_select_newest_among_busy() {
    let picked = zjjrm_procmap_select(vec![
        zjjrm_ProcEntry { session_id: "busy-old".to_string(), busy: true, updated_at: 100 },
        zjjrm_ProcEntry { session_id: "busy-newer".to_string(), busy: true, updated_at: 300 },
    ]);
    assert_eq!(picked, "busy-newer");
}

#[test]
fn jjtm_procmap_select_empty_is_absent() {
    assert_eq!(zjjrm_procmap_select(Vec::new()), ZJJRM_SESSION_ABSENT);
}
