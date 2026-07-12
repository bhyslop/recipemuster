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
    jjrm_resolve_batch_firemark,
    jjrm_resolve_officium_billet,
    jjrm_studbook_exchange_dir,
    zjjrm_ProcEntry,
    zjjrm_procmap_select,
    ZJJRM_SESSION_ABSENT,
    JJRM_OFFICIUM_STUDBOOK_ENABLED,
};
use super::jjrz_gazette::{jjrz_BatchInput, jjrz_parse_batch_input};
use super::jjrg_gallops::{jjrg_Gallops, jjrg_Heat, jjrg_Pace, jjrg_Tack, jjrg_HeatStatus, jjrg_PaceState, JJRG_UNKNOWN_BASIS};
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{jjrfr_RejectionKind, jjrfr_Seat};
use super::jjtu_testdir::JjkTestDir;
use std::collections::BTreeMap;
use std::path::Path;

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

#[test]
fn jjtm_apply_batch_positioned_slates_fold_in_contiguously() {
    let mut gallops = jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    };
    let (k, h) = make_heat_with_docket("BD", "## Goal\nstanding pace");
    gallops.heats.insert(k, h);

    let md = "# jjezs_slate yankee-pace\n\nfirst slate\n\n# jjezs_slate bravo-pace\n\nsecond slate\n\n# jjezs_reslate ₢BDAAA\n\n## Goal\nstanding pace\n";
    let b = jjrz_parse_batch_input(md).unwrap();
    let fm = jjrm_resolve_batch_firemark(&b).unwrap();

    // first=true aims the run at the head of the heat.
    jjrm_apply_batch(&mut gallops, &b, &fm, None, None, true).unwrap();

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
    // the apostille (bridle/unbridle) command itself, and the remote family.
    for cmd in [
        "jjx_create", "jjx_enroll", "jjx_redocket", "jjx_relabel", "jjx_drop",
        "jjx_relocate", "jjx_reorder", "jjx_alter", "jjx_close", "jjx_validate",
        "jjx_archive", "jjx_transfer", "jjx_paddock", "jjx_curry", "jjx_apostille",
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
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), sonnet).is_ok());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Fable), fable).is_ok());

    // Both directions hold: a frontier caller is refused on a sub-frontier-bridled pace.
    let err = zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), fable).unwrap_err();
    assert!(err.contains("bridled for tier 'sonnet'") && err.contains("'fable'"), "got: {}", err);

    // No frontier carve-out: fable is refused on an opus-bridled pace and vice versa,
    // keeping the persisted tier honest provenance of the executing session.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Opus), fable).is_err());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Fable), opus).is_err());

    // Non-designable families never match a designation.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Gpt).is_err());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Gemini).is_err());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), zjjrm_CallerTier::Unknown).is_err());
}

/// The interdictum recognition law (JJS0 `jjdz_interdictum`): recognition is by
/// wire token alone, so every gating refusal LEADS with the literal token — a
/// prefix ahead of it (the old `jjx <cmd>: ` wrapper) breaks the one thing the
/// agent keys on. Message self-sufficiency rides along: the body names the
/// command that refused and a remedy, because standing context says nothing
/// about the generators.
#[test]
fn jjtm_designation_refusal_leads_with_interdictum_token() {
    use jjrg_PaceState as S;
    let haiku = zjjrm_CallerTier::Designable(jjrg_Tier::Haiku);

    let bridled_mismatch = zjjrm_judge_designation(
        "jjx_orient", "₢AAAAA", &S::Bridled, Some(jjrg_Tier::Sonnet), haiku).unwrap_err();
    let rough_refusal = zjjrm_judge_designation(
        "jjx_record", "₢AAAAA", &S::Rough, None, haiku).unwrap_err();
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
    use jjrg_PaceState as S;
    // A frontier caller proceeds on an undesignated pace.
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Fable)).is_ok());
    assert!(zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Rough,
        None, zjjrm_CallerTier::Designable(jjrg_Tier::Opus)).is_ok());
    // A sub-frontier caller is refused on rough — undesignated work is judgment work.
    let err = zjjrm_judge_designation("jjx_orient", "₢AAAAA", &S::Rough,
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
fn jjtm_resolve_officium_billet_reads_identify_and_station() {
    let td = JjkTestDir::new("jjtm_resolve_officium_billet_reads_identify_and_station");
    zjjtm_init_local(td.path());
    zjjtm_commit_all(td.path(), "a.txt", "hello", "init");

    let billet = jjrm_resolve_officium_billet(&jjrfg_PlainGit, td.path(), "260712-1000-abcd")
        .expect("a git tree must resolve a billet");

    assert_eq!(billet.seat, jjrfr_Seat::Primary);
    assert_eq!(billet.session, "260712-1000-abcd");
    assert!(!billet.station.is_empty(), "station must not be empty");
    assert_eq!(billet.station, sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string()));
}

#[test]
fn jjtm_resolve_officium_billet_propagates_foreign_ground_rejection() {
    let td = JjkTestDir::new("jjtm_resolve_officium_billet_propagates_foreign_ground_rejection");
    // No git init — foreign ground.

    let rejection = jjrm_resolve_officium_billet(&jjrfg_PlainGit, td.path(), "260712-1000-abcd")
        .expect_err("a non-git tree must decline, not claim");

    assert_eq!(rejection.kind, jjrfr_RejectionKind::ForeignGround);
}

#[test]
fn jjtm_studbook_exchange_dir_nests_under_scratch_dirname() {
    let studbook_root = Path::new("/infield/jjqs_studbook");
    let dir = jjrm_studbook_exchange_dir(studbook_root, "260712-1000-abcd");
    assert_eq!(dir, studbook_root.join("officia_scratch").join("260712-1000-abcd"));
}

#[test]
fn jjtm_officium_studbook_enablement_seam_defaults_off() {
    assert!(!JJRM_OFFICIUM_STUDBOOK_ENABLED, "the studbook-resident officium must stay inert until the conversion heat flips it");
}
