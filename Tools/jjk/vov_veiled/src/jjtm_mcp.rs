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
        tier: None,
        effort: None,
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

// ===== Model-tier extraction and the three-bucket guard policy =====

use super::jjrm_mcp::{
    zjjrm_CallerTier,
    zjjrm_extract_tier,
    zjjrm_guard_bucket,
    zjjrm_GuardBucket,
    zjjrm_judge_designation,
};
use super::jjrt_types::jjrg_Tier;

#[test]
fn jjtm_extract_tier_maps_model_families() {
    // Designable vendor families, extracted from real-shaped model IDs.
    assert_eq!(zjjrm_extract_tier("claude-fable-5"), zjjrm_CallerTier::Designable(jjrg_Tier::Fable));
    assert_eq!(zjjrm_extract_tier("claude-opus-4-8"), zjjrm_CallerTier::Designable(jjrg_Tier::Opus));
    assert_eq!(zjjrm_extract_tier("claude-opus-4-6[1m]"), zjjrm_CallerTier::Designable(jjrg_Tier::Opus));
    assert_eq!(zjjrm_extract_tier("claude-sonnet-5"), zjjrm_CallerTier::Designable(jjrg_Tier::Sonnet));
    assert_eq!(zjjrm_extract_tier("claude-haiku-4-5-20251001"), zjjrm_CallerTier::Designable(jjrg_Tier::Haiku));
    // Recognized-but-refused families — named for fair-faced diagnostics,
    // outside both the frontier and designable sets.
    assert_eq!(zjjrm_extract_tier("gpt-5.5"), zjjrm_CallerTier::Gpt);
    assert_eq!(zjjrm_extract_tier("gpt-5.5-codex"), zjjrm_CallerTier::Gpt);
    assert_eq!(zjjrm_extract_tier("codex-mini"), zjjrm_CallerTier::Gpt);
    assert_eq!(zjjrm_extract_tier("gemini-3-pro"), zjjrm_CallerTier::Gemini);
    assert_eq!(zjjrm_extract_tier("mystery-model"), zjjrm_CallerTier::Unknown);
}

#[test]
fn jjtm_frontier_is_fable_and_opus_only() {
    // gpt-5.5 is demoted OUT of the frontier gate (operator-sanctioned).
    assert!(zjjrm_extract_tier("claude-fable-5").zjjrm_is_frontier());
    assert!(zjjrm_extract_tier("claude-opus-4-8").zjjrm_is_frontier());
    assert!(!zjjrm_extract_tier("claude-sonnet-5").zjjrm_is_frontier());
    assert!(!zjjrm_extract_tier("claude-haiku-4-5-20251001").zjjrm_is_frontier());
    assert!(!zjjrm_extract_tier("gpt-5.5").zjjrm_is_frontier());
    assert!(!zjjrm_extract_tier("gemini-3-pro").zjjrm_is_frontier());
    assert!(!zjjrm_extract_tier("mystery-model").zjjrm_is_frontier());
}

#[test]
fn jjtm_guard_buckets_partition_the_command_surface() {
    // OPEN: officium lifecycle + the read-only commands.
    for cmd in ["jjx_open", "jjx_list", "jjx_show", "jjx_brief", "jjx_coronets", "jjx_log", "jjx_search"] {
        assert_eq!(zjjrm_guard_bucket(cmd), zjjrm_GuardBucket::Open, "{}", cmd);
    }
    // DESIGNATION-GUARDED: per-command logic at the dispatch arm.
    for cmd in ["jjx_orient", "jjx_record", "jjx_landing"] {
        assert_eq!(zjjrm_guard_bucket(cmd), zjjrm_GuardBucket::Designation, "{}", cmd);
    }
    // FRONTIER-ONLY: docket-authoring and state-mutating verbs, close, validate,
    // the bridle command itself, and the remote family.
    for cmd in [
        "jjx_create", "jjx_enroll", "jjx_redocket", "jjx_relabel", "jjx_drop",
        "jjx_relocate", "jjx_reorder", "jjx_alter", "jjx_close", "jjx_validate",
        "jjx_archive", "jjx_transfer", "jjx_paddock", "jjx_bridle",
        "jjx_bind", "jjx_send", "jjx_plant", "jjx_fetch", "jjx_relay", "jjx_check",
    ] {
        assert_eq!(zjjrm_guard_bucket(cmd), zjjrm_GuardBucket::Frontier, "{}", cmd);
    }
}

#[test]
fn jjtm_judge_designation_matches_strict_tier_equality() {
    use jjrg_PaceState as S;
    let sonnet = zjjrm_CallerTier::Designable(jjrg_Tier::Sonnet);
    let fable = zjjrm_CallerTier::Designable(jjrg_Tier::Fable);
    let opus = zjjrm_CallerTier::Designable(jjrg_Tier::Opus);

    // A designated pace admits exactly its tier.
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), sonnet).is_ok());
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Fable), fable).is_ok());

    // Both directions hold: a frontier caller is refused on a sub-frontier-bridled pace.
    let err = zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), fable).unwrap_err();
    assert!(err.contains("bridled for tier 'sonnet'") && err.contains("'fable'"), "got: {}", err);

    // No frontier carve-out: fable is refused on an opus-bridled pace and vice versa,
    // keeping the persisted tier honest provenance of the executing session.
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Opus), fable).is_err());
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Fable), opus).is_err());

    // Non-designable families never match a designation.
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Gpt).is_err());
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Gemini).is_err());
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Unknown).is_err());
}

#[test]
fn jjtm_judge_designation_rough_is_frontier_judgment_work() {
    use jjrg_PaceState as S;
    // A frontier caller proceeds on an undesignated pace.
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Fable)).is_ok());
    assert!(zjjrm_judge_designation("₢AAAAA", &S::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Opus)).is_ok());
    // A sub-frontier caller is refused on rough — undesignated work is judgment work.
    let err = zjjrm_judge_designation("₢AAAAA", &S::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Haiku)).unwrap_err();
    assert!(err.contains("judgment work"), "got: {}", err);
    assert!(err.contains("jjx_bridle"), "refusal names the remedy: {}", err);
}
