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
//!   revises, slates append in file order (notice order = pace order), and a
//!   positioned run folds in contiguously at the cursor rather than leaving its
//!   tail at the end of the heat.
//!
//! The single-commit-per-batch property is structural: jjrm_apply_batch is a
//! pure transform run inside the one shared dispatch/persist lifecycle (one
//! machine_commit), already covered by the dispatch tests.
//!
//! - jjrm_resolve_officium_billet / jjrm_studbook_exchange_dir: the officium
//!   re-gestalt's inert seam (JJRM_OFFICIUM_STUDBOOK_ENABLED) — identify-based
//!   billet resolution (station, seat, session) and the studbook-relative
//!   exchange path shape. Nothing here is wired into the live open path.

use super::jjrm_mcp::{
    jjrm_apply_batch,
    jjrm_exchange_dir,
    jjrm_resolve_batch_firemark,
    jjrm_resolve_officium_billet,
    jjrm_station_name,
    jjrm_studbook_exchange_dir,
    zjjrm_exchange_dir_over,
    zjjrm_glean_studbook,
    zjjrm_infield_root,
    zjjrm_load_gallops_over,
    zjjrm_open_staleness_notice,
    zjjrm_open_station_refusal,
    zjjrm_ProcEntry,
    zjjrm_procmap_select,
    ZJJRM_SESSION_ABSENT,
    JJRM_OFFICIUM_STUDBOOK_ENABLED,
};
use super::jjrz_gazette::{jjrz_BatchInput, jjrz_parse_batch_input};
use super::jjrg_gallops::{jjrg_Gallops, jjrg_Heat, jjrg_Pace, jjrg_Tack, jjrg_HeatStatus, jjrg_PaceState, JJRG_UNKNOWN_BASIS};
use super::jjrds_stile::{jjrds_Ground, JJRDS_PEDIGREES_REL_PATH};
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{jjrfr_BilletBirth, jjrfr_FarrierBillet, jjrfr_RejectionKind, jjrfr_Seat};
use super::jjrvb_blotter::{
    jjdb_BlotterConfig,
    jjdb_studbook_config,
    JJDB_CATCHWORD_FOUNDING,
    JJDB_CATCHWORD_SIGIL,
    JJDB_GALLOPS_REL_PATH,
    JJDB_STUDBOOK_DIRNAME,
};
use super::jjtu_testdir::JjkTestDir;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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
    paces.insert(pace_key.clone(), jjrg_Pace { tacks: vec![tack], ..Default::default() });
    let heat = jjrg_Heat {
        silks: format!("heat-{}", heat_id),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec![pace_key],
        paces,
    };
    (heat_key, heat)
}

// A gallops holding the coronets the same-firemark guard tests reference, each in
// its (grandfathered) embedded heat — the paces-scan confirms live affiliation.
fn guard_gallops() -> jjrg_Gallops {
    let mut heats = BTreeMap::new();
    for (heat_id, coronet_ids) in [("BD", ["BDAAb", "BDAAA"]), ("BE", ["BEAAb", "BEAAc"])] {
        let mut paces = BTreeMap::new();
        let mut order = Vec::new();
        for cid in coronet_ids {
            let ck = format!("₢{}", cid);
            let tack = jjrg_Tack {
                ts: "260101-1200".to_string(),
                state: jjrg_PaceState::Rough,
                tier: None,
                effort: None,
                text: vec!["d".to_string()],
                silks: "guard-pace".to_string(),
                basis: JJRG_UNKNOWN_BASIS.to_string(),
            };
            paces.insert(ck.clone(), jjrg_Pace { tacks: vec![tack], ..Default::default() });
            order.push(ck);
        }
        heats.insert(format!("₣{}", heat_id), jjrg_Heat {
            silks: format!("heat-{}", heat_id.to_lowercase()),
            creation_time: "260101".to_string(),
            status: jjrg_HeatStatus::Racing,
            order,
            paces,
        });
    }
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats,
        retention_since: None,
    }
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
    let fm = jjrm_resolve_batch_firemark(&b, &guard_gallops()).unwrap();
    assert_eq!(fm.jjrf_display(), "₣BD");
}

#[test]
fn jjtm_resolve_rejects_cross_heat_paddock_vs_reslate() {
    let b = batch(Some(("BD", "body")), &[("₢BEAAb", "d")], &[]);
    let err = jjrm_resolve_batch_firemark(&b, &guard_gallops()).unwrap_err();
    assert!(err.contains("cross-heat batch rejected"), "got: {}", err);
}

#[test]
fn jjtm_resolve_rejects_cross_heat_mass_reslate() {
    // The closed legacy bug: two reslates in different heats. The old path keyed
    // off the first coronet with no check; the guard now rejects.
    let b = batch(None, &[("₢BDAAb", "d"), ("₢BEAAc", "d")], &[]);
    let err = jjrm_resolve_batch_firemark(&b, &guard_gallops()).unwrap_err();
    assert!(err.contains("cross-heat batch rejected"), "got: {}", err);
}

#[test]
fn jjtm_resolve_rejects_slate_only_batch() {
    let b = batch(None, &[], &[("new-pace", "d")]);
    let err = jjrm_resolve_batch_firemark(&b, &guard_gallops()).unwrap_err();
    assert!(err.contains("no heat anchor"), "got: {}", err);
}

// ===== Batch application =====

#[test]
fn jjtm_apply_batch_reslates_and_slates_in_file_order() {
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
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
    let fm = jjrm_resolve_batch_firemark(&b, &gallops).unwrap();

    let out = jjrm_apply_batch(&mut gallops, &b, &fm, std::path::Path::new("jjtm-unused-root"), None, None, false).unwrap();
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

#[test]
fn jjtm_apply_batch_intent_posture() {
    // Batch-born paces carry no original-intent capture (the batch vocabulary
    // excludes the companions — documented follow-on) but still get a slate
    // stamp; a batch reslate bumps the target's redocket counter.
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    };
    let (k, h) = make_heat_with_docket("BD", "## Goal\nold goal");
    gallops.heats.insert(k, h);

    let md = "# jjezs_reslate ₢BDAAA\n\n## Goal\nnew goal\n\n# jjezs_slate batch-pace\n\nbatch docket\n";
    let b = jjrz_parse_batch_input(md).unwrap();
    let fm = jjrm_resolve_batch_firemark(&b, &gallops).unwrap();
    jjrm_apply_batch(&mut gallops, &b, &fm, std::path::Path::new("jjtm-unused-root"), None, None, false).unwrap();

    let heat = gallops.heats.get("₣BD").unwrap();
    let reslated = heat.paces.get("₢BDAAA").unwrap();
    assert_eq!(reslated.redocket_count, 1, "batch reslate bumps the counter");

    let born = heat.paces.get(&heat.order[1]).unwrap();
    assert!(born.dictation.is_none());
    assert!(born.precis.is_none());
    assert!(born.slated.as_deref().is_some_and(|s| !s.is_empty()), "slate stamp still recorded");
    assert_eq!(born.redocket_count, 0);
}

#[test]
fn jjtm_apply_batch_positioned_slates_fold_in_contiguously() {
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    };
    let (k, h) = make_heat_with_docket("BD", "## Goal\nstanding pace");
    gallops.heats.insert(k, h);

    let md = "# jjezs_slate yankee-pace\n\nfirst slate\n\n# jjezs_slate bravo-pace\n\nsecond slate\n\n# jjezs_reslate ₢BDAAA\n\n## Goal\nstanding pace\n";
    let b = jjrz_parse_batch_input(md).unwrap();
    let fm = jjrm_resolve_batch_firemark(&b, &gallops).unwrap();

    // first=true aims the run at the head of the heat.
    jjrm_apply_batch(&mut gallops, &b, &fm, std::path::Path::new("jjtm-unused-root"), None, None, true).unwrap();

    let heat = gallops.heats.get("₣BD").unwrap();
    assert_eq!(heat.order.len(), 3);
    // Both slates land at the head, in file order, ahead of the standing pace —
    // the second does not scatter to the tail behind it.
    let s0 = &heat.paces.get(&heat.order[0]).unwrap().tacks.first().unwrap().silks;
    let s1 = &heat.paces.get(&heat.order[1]).unwrap().tacks.first().unwrap().silks;
    assert_eq!(s0, "yankee-pace");
    assert_eq!(s1, "bravo-pace");
    assert_eq!(heat.order[2], "₢BDAAA");
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
    zjjrm_frontier_refusal,
    zjjrm_guard_bucket,
    zjjrm_GuardBucket,
    zjjrm_judge_designation,
    zjjrm_protocol_verdict,
    zjjrm_slate_refusal,
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
    // SLATE: jjx_enroll alone — the docket-authoring floor admitting sonnet.
    assert_eq!(zjjrm_guard_bucket("jjx_enroll"), zjjrm_GuardBucket::Slate);
    // FRONTIER-ONLY: the remaining state-mutating verbs, close, validate,
    // the apostille (bridle/unbridle) command itself, and the remote family.
    for cmd in [
        "jjx_create", "jjx_redocket", "jjx_relabel", "jjx_drop",
        "jjx_relocate", "jjx_reorder", "jjx_alter", "jjx_close", "jjx_validate",
        "jjx_archive", "jjx_transfer", "jjx_paddock", "jjx_curry", "jjx_apostille",
        "jjx_bind", "jjx_send", "jjx_plant", "jjx_fetch", "jjx_relay", "jjx_check",
    ] {
        assert_eq!(zjjrm_guard_bucket(cmd), zjjrm_GuardBucket::Frontier, "{}", cmd);
    }
}

#[test]
fn jjtm_slate_gate_admits_sonnet_and_up_only() {
    // The slate floor lifts jjx_enroll to admit sonnet without touching the rest
    // of the frontier surface. Sonnet is slate-qualified but NOT frontier, so it
    // slates paces yet stays refused on every other Frontier-bucket verb.
    let sonnet = zjjrm_extract_tier("claude-sonnet-5");
    assert!(sonnet.zjjrm_is_slate_qualified());
    assert!(!sonnet.zjjrm_is_frontier());

    // Frontier tiers remain slate-qualified (the floor only widens downward).
    assert!(zjjrm_extract_tier("claude-opus-4-8").zjjrm_is_slate_qualified());
    assert!(zjjrm_extract_tier("claude-fable-5").zjjrm_is_slate_qualified());

    // Haiku and the non-designable families stay below the slate floor.
    assert!(!zjjrm_extract_tier("claude-haiku-4-5-20251001").zjjrm_is_slate_qualified());
    assert!(!zjjrm_extract_tier("gpt-5.5").zjjrm_is_slate_qualified());
    assert!(!zjjrm_extract_tier("gemini-3-pro").zjjrm_is_slate_qualified());
    assert!(!zjjrm_extract_tier("mystery-model").zjjrm_is_slate_qualified());

    // The slate refusal is an interdictum that names the floor and a remedy.
    let refusal = zjjrm_slate_refusal("jjx_enroll", "claude-haiku-4-5", zjjrm_extract_tier("claude-haiku-4-5"));
    assert!(refusal.starts_with("INTERDICTUM — "), "token must lead: {}", refusal);
    assert!(refusal.contains("jjx_enroll") && refusal.contains("sonnet"), "got: {}", refusal);
    assert!(refusal.contains("Remedy"), "names a remedy: {}", refusal);
}

#[test]
fn jjtm_protocol_verdict_splits_on_session_standing() {
    // The verdict speaks only to a session the designation guard already
    // cleared, so it turns on the caller's own standing alone: a frontier
    // session wraps its own work, a designee lands it for review. The words the
    // mounting agent obeys are asserted literally — the agent no longer derives
    // this, so a silent wording drift would leave it with no instruction.
    let frontier = zjjrm_protocol_verdict(zjjrm_extract_tier("claude-opus-4-8"));
    assert!(frontier.contains("full ceremony"), "{}", frontier);
    assert!(frontier.contains("wrap this pace yourself"), "{}", frontier);
    assert!(frontier.contains("Standing: opus"), "{}", frontier);

    let designee = zjjrm_protocol_verdict(zjjrm_extract_tier("claude-sonnet-5"));
    assert!(designee.contains("designee"), "{}", designee);
    assert!(designee.contains("jjx_landing"), "{}", designee);
    assert!(designee.contains("NEVER wrap"), "{}", designee);
    assert!(designee.contains("Standing: sonnet"), "{}", designee);

    // Fable is the other frontier family; haiku the other designee one.
    assert!(zjjrm_protocol_verdict(zjjrm_extract_tier("claude-fable-5")).contains("full ceremony"));
    assert!(zjjrm_protocol_verdict(zjjrm_extract_tier("claude-haiku-4-5")).contains("designee"));
}

#[test]
fn jjtm_judge_designation_matches_strict_tier_equality() {
    use jjrg_PaceState;
    let sonnet = zjjrm_CallerTier::Designable(jjrg_Tier::Sonnet);
    let fable = zjjrm_CallerTier::Designable(jjrg_Tier::Fable);
    let opus = zjjrm_CallerTier::Designable(jjrg_Tier::Opus);

    // A designated pace admits exactly its tier.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), sonnet).is_ok());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Fable), fable).is_ok());

    // Both directions hold: a frontier caller is refused on a sub-frontier-bridled pace.
    let err = zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), fable).unwrap_err();
    assert!(err.contains("bridled for tier 'sonnet'") && err.contains("'fable'"), "got: {}", err);

    // No frontier carve-out: fable is refused on an opus-bridled pace and vice versa,
    // keeping the persisted tier honest provenance of the executing session.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Opus), fable).is_err());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Fable), opus).is_err());

    // Non-designable families never match a designation.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Gpt).is_err());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Gemini).is_err());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Unknown).is_err());
}

/// The interdictum recognition law (JJS0 `jjdz_interdictum`): recognition is by
/// wire token alone, so every gating refusal LEADS with the literal token — a
/// prefix ahead of it (the old `jjx <cmd>: ` wrapper) breaks the one thing the
/// agent keys on. Message self-sufficiency rides along: the body names the
/// command that refused and a remedy, because standing context says nothing
/// about the generators.
#[test]
fn jjtm_designation_refusal_leads_with_interdictum_token() {
    use jjrg_PaceState;
    let haiku = zjjrm_CallerTier::Designable(jjrg_Tier::Haiku);

    let bridled_mismatch = zjjrm_judge_designation(
        "jjx_orient", "₢AAAAA", &jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet), haiku).unwrap_err();
    let rough_refusal = zjjrm_judge_designation(
        "jjx_record", "₢AAAAA", &jjrg_PaceState::Rough, None, haiku).unwrap_err();
    let frontier_refusal = zjjrm_frontier_refusal(
        "jjx_close", "claude-haiku-4-5", haiku);

    for msg in [&bridled_mismatch, &rough_refusal, &frontier_refusal] {
        assert!(msg.starts_with("INTERDICTUM — "), "token must lead: {}", msg);
        assert!(msg.contains("Remed"), "message names a remedy: {}", msg);
    }
    // Self-sufficient: the body says which command refused.
    assert!(bridled_mismatch.contains("jjx_orient"), "got: {}", bridled_mismatch);
    assert!(rough_refusal.contains("jjx_record"), "got: {}", rough_refusal);
    assert!(frontier_refusal.contains("jjx_close"), "got: {}", frontier_refusal);
}

#[test]
fn jjtm_judge_designation_rough_is_frontier_judgment_work() {
    use jjrg_PaceState;
    // A frontier caller proceeds on an undesignated pace.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Fable)).is_ok());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Opus)).is_ok());
    // A sub-frontier caller is refused on rough — undesignated work is judgment work.
    let err = zjjrm_judge_designation("jjx_orient", "₢AAAAA", &jjrg_PaceState::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Haiku)).unwrap_err();
    assert!(err.contains("judgment work"), "got: {}", err);
    assert!(err.contains("jjx_apostille"), "refusal names the remedy: {}", err);
}

// ===== Officium re-gestalt (inert seam) =====

const ZJJTM_TRUNK: &str = "jjtm-trunk";

fn zjjtm_git(dir: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("test harness git invocation must spawn");
    assert!(
        out.status.success(),
        "test harness git -C {} {:?} failed: {}",
        dir.display(),
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("git stdout must be UTF-8").trim().to_string()
}

fn zjjtm_init_local(dir: &Path) {
    zjjtm_git(dir, &["init", "-q", "-b", ZJJTM_TRUNK]);
    zjjtm_git(dir, &["config", "user.email", "jjtm@example.invalid"]);
    zjjtm_git(dir, &["config", "user.name", "jjtm"]);
}

fn zjjtm_commit_all(dir: &Path, name: &str, content: &str, message: &str) {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtm_git(dir, &["add", "--", name]);
    zjjtm_git(dir, &["commit", "-q", "-m", message]);
}

#[test]
fn jjtm_resolve_officium_billet_carries_all_three_members() {
    let td = JjkTestDir::new("jjtm_resolve_officium_billet_carries_all_three_members");
    zjjtm_init_local(td.path());
    zjjtm_commit_all(td.path(), "a.txt", "hello", "init");

    let billet =
        jjrm_resolve_officium_billet(&jjrfg_PlainGit, td.path(), "jjtm-station", "260712-1000-abcd")
            .expect("a git tree must resolve a billet");

    assert_eq!(billet.seat, jjrfr_Seat::Primary);
    assert_eq!(billet.station, "jjtm-station");
    assert_eq!(billet.session, "260712-1000-abcd");
}

#[test]
fn jjtm_resolve_officium_billet_propagates_foreign_ground_rejection() {
    let td = JjkTestDir::new("jjtm_resolve_officium_billet_propagates_foreign_ground_rejection");
    // No git init — foreign ground.

    let rejection =
        jjrm_resolve_officium_billet(&jjrfg_PlainGit, td.path(), "jjtm-station", "260712-1000-abcd")
            .expect_err("a non-git tree must decline, not claim");

    assert_eq!(rejection.kind, jjrfr_RejectionKind::ForeignGround);
}

#[test]
fn jjtm_station_name_offers_no_stand_in() {
    // The station either names itself or yields None — never a shared stand-in
    // two unnamed stations would both record (JJSVF officium-open; the refusal
    // is jjx_open's own, wired at the conversion heat).
    if let Some(station) = jjrm_station_name() {
        assert!(!station.is_empty(), "a named station must not name itself the empty string");
        assert_ne!(station, "unknown", "'unknown' is a stand-in, not a station name");
    }
}

#[test]
fn jjtm_studbook_exchange_dir_nests_under_scratch_dirname() {
    let studbook_root = Path::new("/infield/jjqs_studbook");
    let dir = jjrm_studbook_exchange_dir(studbook_root, "260712-1000-abcd");
    assert_eq!(dir, studbook_root.join("officia_scratch").join("260712-1000-abcd"));
}

#[test]
fn jjtm_officium_studbook_enablement_seam_is_live() {
    assert!(JJRM_OFFICIUM_STUDBOOK_ENABLED, "the studbook-resident officium is live post-cutover (₣B3 founding-and-cutover); a revert to false would silently relocate the officium exchange back into the consumer repo");
}

#[test]
fn jjtm_exchange_dir_over_resolves_under_studbook_when_seam_on() {
    let studbook = jjdb_studbook_config(Path::new("/infield"));
    let dir = zjjrm_exchange_dir_over("260712-1000-abcd", true, &studbook);
    assert_eq!(dir, jjrm_studbook_exchange_dir(&studbook.local_root, "260712-1000-abcd"));
}

#[test]
fn jjtm_exchange_dir_over_strips_incipit_prefix_when_seam_on() {
    let studbook = jjdb_studbook_config(Path::new("/infield"));
    let dir = zjjrm_exchange_dir_over("\u{2609}260712-1000-abcd", true, &studbook);
    assert_eq!(dir, jjrm_studbook_exchange_dir(&studbook.local_root, "260712-1000-abcd"));
}

#[test]
fn jjtm_exchange_dir_live_wrapper_matches_the_seam_on_branch() {
    // Post-cutover the seam is on, so the live wrapper resolves under the
    // studbook — byte-identical to the seam-on testable branch driven with the
    // studbook config the wrapper itself derives from cwd (the test runs inside
    // a hippodrome, so that derivation resolves, exactly as the live wrapper
    // assumes).
    let cwd = std::env::current_dir().expect("test cwd");
    let infield_root = zjjrm_infield_root(&jjrfg_PlainGit, &cwd)
        .expect("the test runs inside a hippodrome, so the infield root resolves");
    let studbook = jjdb_studbook_config(&infield_root);
    let via_over = zjjrm_exchange_dir_over("260712-1000-abcd", true, &studbook);
    let via_wrapper = jjrm_exchange_dir("260712-1000-abcd");
    assert_eq!(via_over, via_wrapper, "seam-on: the live wrapper must match the seam-on testable branch");
}

#[test]
fn jjtm_open_station_refusal_fires_only_when_seam_on_and_unnamed() {
    assert!(zjjrm_open_station_refusal("jjx_open", None, true).is_some());
    assert!(zjjrm_open_station_refusal("jjx_open", Some("mac.lan"), true).is_none());
    assert!(zjjrm_open_station_refusal("jjx_open", None, false).is_none(), "seam-off must never refuse — jjx_open's seam-off behavior is unchanged");
    assert!(zjjrm_open_station_refusal("jjx_open", Some("mac.lan"), false).is_none());
}

// ===== Open's staleness lead (warn-at-open) =====

/// A full infield: bare upstream, a hippodrome clone tracking it, and a
/// studbook whose one pedigree records the upstream — the same shape
/// `jjtds_stile.rs`'s `zjjtds_infield` builds for the dispatch board's own
/// staleness tests, reproduced here since it is test-module-private there.
fn zjjtm_staleness_infield(name: &str) -> (JjkTestDir, std::path::PathBuf) {
    let infield = JjkTestDir::new(name);
    let bare = infield.path().join("upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtm_git(&bare, &["init", "-q", "--bare", "-b", ZJJTM_TRUNK]);

    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtm_init_local(&hippodrome);
    zjjtm_commit_all(&hippodrome, "base.txt", "base", "init");
    let bare_url = bare.to_string_lossy().into_owned();
    zjjtm_git(&hippodrome, &["remote", "add", "origin", &bare_url]);
    zjjtm_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTM_TRUNK]);

    let studbook_root = infield.path().join(JJDB_STUDBOOK_DIRNAME);
    std::fs::create_dir_all(&studbook_root).unwrap();
    let body = serde_json::json!({
        "jjop_sires": [{
            "jjop_kind": "plain-git",
            "jjop_addresses": [bare_url],
            "jjop_trunk": ZJJTM_TRUNK,
        }]
    });
    std::fs::write(studbook_root.join(JJRDS_PEDIGREES_REL_PATH), serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    (infield, hippodrome)
}

#[test]
fn jjtm_open_staleness_notice_is_none_when_current() {
    let (_infield, hippodrome) = zjjtm_staleness_infield("jjtm_open_staleness_current");
    assert_eq!(zjjrm_open_staleness_notice(&jjrfg_PlainGit, &hippodrome), None);
}

#[test]
fn jjtm_open_staleness_notice_names_refit_when_trunk_moved() {
    let (infield, hippodrome) = zjjtm_staleness_infield("jjtm_open_staleness_stale");

    // A second clone (another station) advances trunk and pushes past this
    // hippodrome's back — the scenario Finding B describes: a tree re-entered
    // for `jjx_open` without ever re-dispatching through the board.
    let other = infield.path().join("other-station");
    zjjtm_git(infield.path(), &["clone", "-q", &infield.path().join("upstream").to_string_lossy(), &other.to_string_lossy()]);
    zjjtm_git(&other, &["config", "user.email", "jjtm@example.invalid"]);
    zjjtm_git(&other, &["config", "user.name", "jjtm"]);
    zjjtm_commit_all(&other, "b.txt", "moved", "trunk advances from elsewhere");
    zjjtm_git(&other, &["push", "-q", "origin", ZJJTM_TRUNK]);

    let notice = zjjrm_open_staleness_notice(&jjrfg_PlainGit, &hippodrome);
    assert!(
        notice.as_deref().unwrap_or("").contains("refit"),
        "a stale hippodrome must lead with a notice naming refit, got: {:?}",
        notice
    );
}

#[test]
fn jjtm_open_staleness_notice_warns_on_billet_reentry_after_trunk_moves() {
    // The Partition-seat path, and Finding B's exact motivating scenario: a
    // billet born before trunk advanced, never re-dispatched, re-entered later
    // by a plain `jjx_open`.
    let (infield, hippodrome) = zjjtm_staleness_infield("jjtm_open_staleness_billet");
    let billet_root = infield.path().join("jjqb_AAAAA");
    jjrfg_PlainGit
        .jjrfr_billet_create(&hippodrome, &jjrfr_BilletBirth::Branch("AAAAA".to_string()), &billet_root, ZJJTM_TRUNK)
        .unwrap();

    // Fresh: the billet just anchored at trunk's tip.
    assert_eq!(zjjrm_open_staleness_notice(&jjrfg_PlainGit, &billet_root), None);

    // Trunk advances from the hippodrome and is pushed; the billet itself is
    // never touched again.
    zjjtm_commit_all(&hippodrome, "b.txt", "moved", "trunk advances");
    zjjtm_git(&hippodrome, &["push", "-q", "origin", ZJJTM_TRUNK]);

    let notice = zjjrm_open_staleness_notice(&jjrfg_PlainGit, &billet_root);
    assert!(
        notice.as_deref().unwrap_or("").contains("refit"),
        "a session re-entering a stale billet must be warned, got: {:?}",
        notice
    );
}

#[test]
fn jjtm_open_staleness_notice_is_none_when_studbook_unreadable() {
    // No pedigree at all: a plain local repo, no studbook — the probe must
    // degrade silently rather than block open.
    let td = JjkTestDir::new("jjtm_open_staleness_unreadable");
    zjjtm_init_local(td.path());
    zjjtm_commit_all(td.path(), "a.txt", "hello", "init");
    assert_eq!(zjjrm_open_staleness_notice(&jjrfg_PlainGit, td.path()), None);
}

#[test]
fn jjtm_open_staleness_notice_is_none_on_foreign_ground() {
    let td = JjkTestDir::new("jjtm_open_staleness_foreign");
    // No git init — foreign ground.
    assert_eq!(zjjrm_open_staleness_notice(&jjrfg_PlainGit, td.path()), None);
}

// ===== Command-surface gallops read (studbook seam, build-gap rescope) =====
//
// `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED` stays false at land (mainline-inert —
// see jjtvb_blotter.rs's own seam-defaults-off test); these exercise the
// on-branch directly via `zjjrm_load_gallops_over`'s explicit parameter,
// mirroring how `jjrds_plan`'s `over_studbook` parameter is tested in
// jjtds_stile.rs. The seam-off branch needs no dedicated fixture here: it is
// byte-identical to the pre-seam `jjrg_Gallops::jjrg_load(path)` call every
// existing jjx_* command test already exercises.

fn zjjtm_gallops_valid(seed: &str) -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: seed.to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    }
}

/// A studbook scratch triple (bare remote, local clone, config) with one
/// `gallops.json` committed and pushed — the same shape `jjtvb_blotter.rs`'s
/// `zjjtvb_scratch` builds, reproduced here since it is test-module-private
/// there. `seed` rides `next_heat_seed` so a test can tell "read the
/// studbook" apart from "read anything else".
fn zjjtm_gallops_scratch(name: &str, seed: &str) -> (JjkTestDir, JjkTestDir, jjdb_BlotterConfig) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtm_git(bare.path(), &["init", "-q", "--bare", "-b", ZJJTM_TRUNK]);
    let local = JjkTestDir::new(&format!("{}_local", name));
    zjjtm_init_local(local.path());
    let json = serde_json::to_string_pretty(&zjjtm_gallops_valid(seed)).unwrap();
    zjjtm_commit_all(local.path(), JJDB_GALLOPS_REL_PATH, &json, "seed gallops");
    zjjtm_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtm_git(local.path(), &["push", "-q", "-u", "origin", ZJJTM_TRUNK]);
    let config = jjdb_BlotterConfig {
        local_root: local.path().to_path_buf(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTM_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    (bare, local, config)
}

#[test]
fn jjtm_load_gallops_over_enabled_reads_the_studbook_pin() {
    // next_heat_seed is a fixed-width 2-char field — the marker must fit it.
    let (_bare, _local, config) = zjjtm_gallops_scratch("jjtm_load_seam_on", "SB");
    // A path that does not exist at all — proves the on-branch never touches it.
    let untouched_path = Path::new("/nonexistent/jjtm-never-read.json");

    let gallops = zjjrm_load_gallops_over(true, untouched_path, &config).unwrap();

    assert_eq!(gallops.next_heat_seed, "SB");
}

#[test]
fn jjtm_load_gallops_over_enabled_refuses_loud_when_clone_absent() {
    let td = JjkTestDir::new("jjtm_load_seam_on_absent");
    let config = jjdb_BlotterConfig {
        local_root: td.path().join("never-founded"),
        remote_url: String::new(),
        trunk: ZJJTM_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let result = zjjrm_load_gallops_over(true, Path::new("irrelevant.json"), &config);

    assert!(result.is_err(), "a missing studbook clone must refuse loud, never silently fall back");
}

#[test]
fn jjtm_load_gallops_over_enabled_refuses_loud_when_clone_never_gleaned() {
    // A real local git repo, but never pushed/fetched against any remote —
    // refs/remotes/origin/<trunk> does not exist, so the pin has nothing to
    // resolve.
    let local = JjkTestDir::new("jjtm_load_seam_on_never_gleaned");
    zjjtm_init_local(local.path());
    zjjtm_commit_all(local.path(), "base.txt", "base", "init");
    let config = jjdb_BlotterConfig {
        local_root: local.path().to_path_buf(),
        remote_url: String::new(),
        trunk: ZJJTM_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let result = zjjrm_load_gallops_over(true, Path::new("irrelevant.json"), &config);

    assert!(result.is_err(), "an ungleaned clone (no fetched pin yet) must refuse loud");
}

#[test]
fn jjtm_load_gallops_over_disabled_reads_path_and_never_touches_studbook() {
    let td = JjkTestDir::new("jjtm_load_seam_off");
    let path = td.path().join(JJDB_GALLOPS_REL_PATH);
    std::fs::write(&path, serde_json::to_string_pretty(&zjjtm_gallops_valid("PA")).unwrap()).unwrap();
    // A studbook config pointed at nothing — proves the off branch never
    // reaches for it (a touch would error, not just misread).
    let poison_config = jjdb_BlotterConfig {
        local_root: PathBuf::from("/nonexistent/jjtm-poison"),
        remote_url: String::new(),
        trunk: String::new(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let loaded = zjjrm_load_gallops_over(false, &path, &poison_config).unwrap();

    assert_eq!(loaded.next_heat_seed, "PA");
}

/// A marker file planted directly under the infield root, checked back
/// through whatever root `zjjrm_infield_root` derives — a functional
/// same-directory proof that never compares path strings. `jjrfr_identify`
/// resolves through `git rev-parse --show-toplevel`, which unwinds
/// platform-specific symlinks (e.g. macOS's /var -> /private/var) that the
/// test's own unresolved path construction does not — a raw
/// `assert_eq!(root, Some(expected_path))` is platform-fragile for exactly
/// that reason, and `std::fs::canonicalize` is its own Windows wart, so
/// this sidesteps path comparison entirely rather than papering over it.
fn zjjtm_assert_same_dir(root: Option<PathBuf>, expected: &Path) {
    let root = root.expect("a git tree must resolve an infield root");
    let via_root = std::fs::read_to_string(root.join("jjtm-marker.txt")).expect("marker must read back through the derived root");
    let via_expected = std::fs::read_to_string(expected.join("jjtm-marker.txt")).expect("marker must read back through the planted path");
    assert_eq!(
        via_root, via_expected,
        "the derived root must be the same directory as the marker's, however each path string reads"
    );
}

#[test]
fn jjtm_infield_root_is_hippodrome_parent_for_primary_seat() {
    let infield = JjkTestDir::new("jjtm_infield_root_primary");
    std::fs::write(infield.path().join("jjtm-marker.txt"), "primary").unwrap();
    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtm_init_local(&hippodrome);
    zjjtm_commit_all(&hippodrome, "a.txt", "hello", "init");

    let root = zjjrm_infield_root(&jjrfg_PlainGit, &hippodrome);

    zjjtm_assert_same_dir(root, infield.path());
}

#[test]
fn jjtm_infield_root_is_primary_root_parent_for_partition_seat() {
    let infield = JjkTestDir::new("jjtm_infield_root_partition");
    std::fs::write(infield.path().join("jjtm-marker.txt"), "partition").unwrap();
    // billet_create anchors at trunk's remote counterpart — the hippodrome
    // needs a real pushed origin, the same shape zjjtm_staleness_infield
    // builds for the same reason.
    let bare = infield.path().join("upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtm_git(&bare, &["init", "-q", "--bare", "-b", ZJJTM_TRUNK]);
    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtm_init_local(&hippodrome);
    zjjtm_commit_all(&hippodrome, "a.txt", "hello", "init");
    zjjtm_git(&hippodrome, &["remote", "add", "origin", &bare.to_string_lossy()]);
    zjjtm_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTM_TRUNK]);
    let billet_root = infield.path().join("jjqb_AAAAA");
    jjrfg_PlainGit
        .jjrfr_billet_create(&hippodrome, &jjrfr_BilletBirth::Branch("AAAAA".to_string()), &billet_root, ZJJTM_TRUNK)
        .unwrap();

    let root = zjjrm_infield_root(&jjrfg_PlainGit, &billet_root);

    zjjtm_assert_same_dir(root, infield.path());
}

#[test]
fn jjtm_infield_root_is_none_on_foreign_ground() {
    let td = JjkTestDir::new("jjtm_infield_root_foreign");
    // No git init — foreign ground.
    assert_eq!(zjjrm_infield_root(&jjrfg_PlainGit, td.path()), None);
}

#[test]
fn jjtm_glean_studbook_succeeds_against_a_reachable_remote() {
    let (_bare, _local, config) = zjjtm_gallops_scratch("jjtm_glean_ok", "GK");

    let result = zjjrm_glean_studbook(&jjrfg_PlainGit, &config);

    assert!(result.is_ok(), "a clone with a live origin must glean cleanly: {:?}", result);
}

#[test]
fn jjtm_glean_studbook_refuses_loud_when_remote_is_unreachable() {
    let local = JjkTestDir::new("jjtm_glean_unreachable");
    zjjtm_init_local(local.path());
    zjjtm_commit_all(local.path(), "base.txt", "base", "init");
    zjjtm_git(local.path(), &["remote", "add", "origin", "/nonexistent/jjtm-nowhere"]);
    let config = jjdb_BlotterConfig {
        local_root: local.path().to_path_buf(),
        remote_url: String::new(),
        trunk: ZJJTM_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let result = zjjrm_glean_studbook(&jjrfg_PlainGit, &config);

    assert!(result.is_err(), "an unreachable remote must refuse loud, never silently succeed");
}

// ===== The ground guard =====
//
// The judgment is pure over (command, ground, aim), so these drive it directly:
// the ground taxonomy's own resolution from real worktrees is jjtds_stile's.

/// A parent-heat resolver for the one case that consults one — the notch's
/// heat-affiliated form. Every coronet in these tests harbours in ₣AA.
fn zjjtm_heat_of(_coronet: &str) -> Option<String> {
    Some("₣AA".to_string())
}

/// No heat resolves — the shape a gallops the guard cannot read presents.
fn zjjtm_heat_of_none(_coronet: &str) -> Option<String> {
    None
}

fn zjjtm_pace_billet(coronet: &str) -> jjrds_Ground {
    jjrds_Ground::PaceBillet { coronet: coronet.to_string() }
}

#[test]
fn jjtm_ground_need_binds_exactly_the_three_work_repo_verbs() {
    use super::jjrm_mcp::{zjjrm_ground_need, zjjrm_GroundNeed};

    for bound in ["jjx_orient", "jjx_record", "jjx_close"] {
        assert_eq!(zjjrm_ground_need(bound), zjjrm_GroundNeed::OwnPaceBillet, "{} reaches the work repo", bound);
    }
    // Every studbook verb is free — grooming any heat from any billet is legal.
    for free in [
        "jjx_show", "jjx_search", "jjx_brief", "jjx_coronets", "jjx_paddock", "jjx_curry",
        "jjx_enroll", "jjx_redocket", "jjx_log", "jjx_list", "jjx_apostille", "jjx_landing",
        "jjx_create", "jjx_archive", "jjx_reorder", "jjx_relabel", "jjx_drop", "jjx_relocate",
        "jjx_alter", "jjx_transfer", "jjx_validate",
    ] {
        assert_eq!(zjjrm_ground_need(free), zjjrm_GroundNeed::Free, "{} touches only the studbook", free);
    }
}

#[test]
fn jjtm_ground_lets_every_studbook_verb_pass_from_all_three_grounds() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let grounds = [
        jjrds_Ground::Hippodrome,
        zjjtm_pace_billet("CAAAA"),
        jjrds_Ground::GroomBillet,
    ];
    for ground in &grounds {
        for cmd in ["jjx_show", "jjx_curry", "jjx_enroll", "jjx_redocket", "jjx_apostille", "jjx_landing"] {
            assert!(
                zjjrm_judge_ground(cmd, ground, Some("CAAAB"), &zjjtm_heat_of).is_ok(),
                "{} must pass on {}", cmd, ground.jjrds_as_str()
            );
        }
    }
}

#[test]
fn jjtm_ground_refuses_the_bound_verbs_off_a_pace_billet() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    for ground in [jjrds_Ground::Hippodrome, jjrds_Ground::GroomBillet, jjrds_Ground::Unboarded { line: "operator-branch".to_string() }] {
        for (cmd, aim) in [("jjx_orient", "CAAAA"), ("jjx_record", "CAAAA"), ("jjx_close", "CAAAA")] {
            let refusal = zjjrm_judge_ground(cmd, &ground, Some(aim), &zjjtm_heat_of)
                .expect_err("a work-repo verb has no ground to stand on here");
            assert!(refusal.starts_with("INTERDICTUM — ground gate:"), "token leads: {}", refusal);
            assert!(refusal.contains(cmd));
            assert!(refusal.contains("Remedy:"), "every refusal names its remedy: {}", refusal);
        }
    }
}

#[test]
fn jjtm_ground_names_slating_as_the_notch_remedy_and_saddling_as_the_mount_remedy() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let notch = zjjrm_judge_ground("jjx_record", &jjrds_Ground::GroomBillet, Some("CAAAA"), &zjjtm_heat_of).unwrap_err();
    assert!(notch.contains("slate a pace for this work"), "the misplaced-work remedy is a slated pace: {}", notch);

    let mount = zjjrm_judge_ground("jjx_orient", &jjrds_Ground::Hippodrome, Some("CAAAA"), &zjjtm_heat_of).unwrap_err();
    assert!(mount.contains("jjy_saddle"), "the mount remedy is the shell door: {}", mount);
}

#[test]
fn jjtm_ground_admits_re_orienting_the_billets_own_pace() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let ground = zjjtm_pace_billet("CAAAA");
    // Re-orientation is not a remount: the same pace, mounted again, stays legal.
    assert!(zjjrm_judge_ground("jjx_orient", &ground, Some("CAAAA"), &zjjtm_heat_of).is_ok());
    assert!(zjjrm_judge_ground("jjx_close", &ground, Some("CAAAA"), &zjjtm_heat_of).is_ok());
    assert!(zjjrm_judge_ground("jjx_record", &ground, Some("CAAAA"), &zjjtm_heat_of).is_ok());
}

#[test]
fn jjtm_ground_aim_reads_each_bound_verb_where_it_carries_its_target() {
    use super::jjrm_mcp::zjjrm_ground_aim;

    // Orient's target lives in the gazette halter notice; the other two in
    // params. Every form arrives bare, so a qualified emission ingests unchanged.
    let halter = "# jjezs_halter ₢AA·CAAAA\n";
    assert_eq!(zjjrm_ground_aim("jjx_orient", &serde_json::json!({}), Some(halter)).as_deref(), Some("CAAAA"));
    assert_eq!(
        zjjrm_ground_aim("jjx_record", &serde_json::json!({"identity": "₢CAAAA"}), None).as_deref(),
        Some("CAAAA")
    );
    assert_eq!(
        zjjrm_ground_aim("jjx_close", &serde_json::json!({"coronet": "CAAAA"}), None).as_deref(),
        Some("CAAAA")
    );

    // Nothing to read: an unwritten gazette, a missing param, a free verb.
    assert_eq!(zjjrm_ground_aim("jjx_orient", &serde_json::json!({}), None), None);
    assert_eq!(zjjrm_ground_aim("jjx_record", &serde_json::json!({}), None), None);
    assert_eq!(zjjrm_ground_aim("jjx_show", &serde_json::json!({}), Some(halter)), None);

    // Orient mounts one target: a heterogeneous set is the show verb's shape,
    // and the orient handler owns that refusal in its own words.
    let two = "# jjezs_halter ₢CAAAA\n# jjezs_halter ₢CAAAB\n";
    assert_eq!(zjjrm_ground_aim("jjx_orient", &serde_json::json!({}), Some(two)), None);
}

#[test]
fn jjtm_ground_refuses_a_foreign_pace_inside_a_pace_billet() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let ground = zjjtm_pace_billet("CAAAA");
    for cmd in ["jjx_orient", "jjx_record", "jjx_close"] {
        let refusal = zjjrm_judge_ground(cmd, &ground, Some("CAAAB"), &zjjtm_heat_of)
            .expect_err("the remount singularity: another pace's work in this billet");
        assert!(refusal.starts_with("INTERDICTUM — ground gate:"));
        assert!(refusal.contains("₢CAAAA"), "the seated pace is named: {}", refusal);
        assert!(refusal.contains("₢CAAAB"), "the aimed pace is named: {}", refusal);
        assert!(refusal.contains("jjy_saddle CAAAB"), "the remedy saddles the other pace: {}", refusal);
    }
}

#[test]
fn jjtm_ground_takes_a_heat_aim_for_notch_alone() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let ground = zjjtm_pace_billet("CAAAA");

    // The notch's admitted second form: the billet's pace or its parent heat.
    assert!(zjjrm_judge_ground("jjx_record", &ground, Some("AA"), &zjjtm_heat_of).is_ok());

    // Another heat is neither.
    let stray = zjjrm_judge_ground("jjx_record", &ground, Some("AB"), &zjjtm_heat_of).unwrap_err();
    assert!(stray.contains("₣AA"), "the billet's own heat is named: {}", stray);
    assert!(stray.contains("₣AB"), "the aimed heat is named: {}", stray);

    // Mount and wrap bind to a pace, so a heat aim — which resolves to whichever
    // pace is next actionable — is exactly the severance the guard exists to stop.
    for cmd in ["jjx_orient", "jjx_close"] {
        let refusal = zjjrm_judge_ground(cmd, &ground, Some("AA"), &zjjtm_heat_of)
            .expect_err("a heat aim cannot bind a billet-bound verb");
        assert!(refusal.contains("names a heat"), "{}", refusal);
        assert!(refusal.contains("₢CAAAA"), "the seated pace is offered instead: {}", refusal);
    }
}

#[test]
fn jjtm_ground_refuses_a_heat_notch_it_cannot_resolve() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let refusal = zjjrm_judge_ground("jjx_record", &zjjtm_pace_billet("CAAAA"), Some("AA"), &zjjtm_heat_of_none)
        .expect_err("an unresolvable parent heat admits nothing");
    assert!(refusal.contains("(unresolved)"), "the gap is stated, never guessed past: {}", refusal);
}

#[test]
fn jjtm_ground_leaves_an_unreadable_aim_to_the_handlers_own_parse() {
    use super::jjrm_mcp::zjjrm_judge_ground;

    let ground = zjjtm_pace_billet("CAAAA");
    // No aim to read, and a token that types as no identity: the guard has
    // nothing to judge and the command answers in its own words.
    assert!(zjjrm_judge_ground("jjx_orient", &ground, None, &zjjtm_heat_of).is_ok());
    assert!(zjjrm_judge_ground("jjx_record", &ground, Some("not-an-identity"), &zjjtm_heat_of).is_ok());
}
