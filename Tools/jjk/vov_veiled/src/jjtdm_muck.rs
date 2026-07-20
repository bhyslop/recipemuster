// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrdm_muck::{
    jjrdm_plan,
    jjrdm_reap,
    jjrdm_Kind,
    jjrdm_Outcome,
    JJRDM_RETENTION_SECS,
};
use super::jjrds_spine::jjrds_billet_dirname;
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
};
use super::jjrt_types::{
    jjrg_Gallops,
    jjrg_Heat,
    jjrg_HeatStatus,
    jjrg_Pace,
    jjrg_PaceState,
    jjrg_Tack,
    jjrg_Tier,
};
use super::jjrvb_blotter::{
    jjdb_gallops_journal_save,
    jjdb_BlotterConfig,
    JJDB_CATCHWORD_FOUNDING,
    JJDB_CATCHWORD_SIGIL,
    JJDB_STUDBOOK_DIRNAME,
};
use super::jjtu_testdir::JjkTestDir;
use std::path::Path;

const ZJJTDM_HIP_TRUNK: &str = "jjtdm-hip-trunk";
const ZJJTDM_SB_TRUNK: &str = "jjtdm-sb-trunk";
const ZJJTDM_GUIDON: &str = "jjtdm-guidon";

// ---- Scaffolding ----

fn zjjtdm_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtdm_init_local(dir: &Path, trunk: &str) {
    zjjtdm_git(dir, &["init", "-q", "-b", trunk]);
    zjjtdm_git(dir, &["config", "user.email", "jjtdm@example.invalid"]);
    zjjtdm_git(dir, &["config", "user.name", "jjtdm"]);
}

fn zjjtdm_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtdm_git(dir, &["add", "--", name]);
    zjjtdm_git(dir, &["commit", "-q", "-m", message]);
    zjjtdm_git(dir, &["rev-parse", "HEAD"])
}

/// Push a directory's mtime into the past — the age record `zjjrdm_past_retention`
/// reads. Opening a directory as a `File` and calling `set_modified` works on
/// Unix (the only platform this test harness runs on).
fn zjjtdm_backdate(path: &Path, secs_ago: u64) {
    let target = std::time::SystemTime::now() - std::time::Duration::from_secs(secs_ago);
    let file = std::fs::File::open(path).expect("directory must open for mtime backdating");
    file.set_modified(target).expect("set_modified must succeed on this platform");
}

fn zjjtdm_tack(state: jjrg_PaceState) -> jjrg_Tack {
    zjjtdm_tack_with_tier(state, None)
}

fn zjjtdm_tack_with_tier(state: jjrg_PaceState, tier: Option<jjrg_Tier>) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260712-1200".to_string(),
        state,
        tier,
        effort: None,
        text: vec!["a docket line".to_string()],
        silks: "muck-test-pace".to_string(),
        basis: "0000000".to_string(),
    }
}

/// A studbook gallops with one heat carrying a complete pace (₢AAAAC), a
/// rough pace (₢AAAAA), an abandoned pace (₢AAAAD), and a bridled pace
/// (₢AAAAE) — enough shape for the closed/open classification gate across
/// all four states.
fn zjjtdm_gallops() -> jjrg_Gallops {
    let mut paces = std::collections::BTreeMap::new();
    paces.insert("₢AAAAC".to_string(), jjrg_Pace { tacks: vec![zjjtdm_tack(jjrg_PaceState::Complete)], ..Default::default() });
    paces.insert("₢AAAAA".to_string(), jjrg_Pace { tacks: vec![zjjtdm_tack(jjrg_PaceState::Rough)], ..Default::default() });
    paces.insert("₢AAAAD".to_string(), jjrg_Pace { tacks: vec![zjjtdm_tack(jjrg_PaceState::Abandoned)], ..Default::default() });
    paces.insert("₢AAAAE".to_string(), jjrg_Pace { tacks: vec![zjjtdm_tack_with_tier(jjrg_PaceState::Bridled, Some(jjrg_Tier::Sonnet))], ..Default::default() });
    let mut heats = std::collections::BTreeMap::new();
    heats.insert("₣AA".to_string(), jjrg_Heat {
        silks: "muck-test-heat".to_string(),
        creation_time: "260712".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec!["₢AAAAC".to_string(), "₢AAAAA".to_string(), "₢AAAAD".to_string(), "₢AAAAE".to_string()],
        paces,
    });
    jjrg_Gallops { next_heat_seed: "AB".to_string(), next_pace_seed: "CAAAA".to_string(), heat_order: vec!["₣AA".to_string()], heats, retention_since: None }
}

/// The combined fixture: an infield holding a hippodrome (its own bare
/// upstream, for billet creation) and a studbook (its own bare upstream, for
/// the lock guidon and the gallops journal), the studbook pre-seeded with
/// `zjjtdm_gallops`. Bare remotes and the hippodrome sit alongside
/// `jjqs_studbook` directly under the infield root — none carry the `jjqb_`
/// prefix, so the sweep's positive glob never touches them.
fn zjjtdm_fixture(name: &str) -> (JjkTestDir, std::path::PathBuf, jjdb_BlotterConfig) {
    let infield = JjkTestDir::new(name);

    let hip_bare = infield.path().join("hip-upstream");
    std::fs::create_dir_all(&hip_bare).unwrap();
    zjjtdm_git(&hip_bare, &["init", "-q", "--bare", "-b", ZJJTDM_HIP_TRUNK]);
    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtdm_init_local(&hippodrome, ZJJTDM_HIP_TRUNK);
    zjjtdm_commit_all(&hippodrome, "base.txt", "base", "init");
    zjjtdm_git(&hippodrome, &["remote", "add", "origin", &hip_bare.to_string_lossy()]);
    zjjtdm_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTDM_HIP_TRUNK]);

    let sb_bare = infield.path().join("sb-upstream");
    std::fs::create_dir_all(&sb_bare).unwrap();
    zjjtdm_git(&sb_bare, &["init", "-q", "--bare", "-b", ZJJTDM_SB_TRUNK]);
    let studbook_local = infield.path().join(JJDB_STUDBOOK_DIRNAME);
    std::fs::create_dir_all(&studbook_local).unwrap();
    zjjtdm_init_local(&studbook_local, ZJJTDM_SB_TRUNK);
    zjjtdm_commit_all(&studbook_local, "base.txt", "base", "init");
    zjjtdm_git(&studbook_local, &["remote", "add", "origin", &sb_bare.to_string_lossy()]);
    zjjtdm_git(&studbook_local, &["push", "-q", "-u", "origin", ZJJTDM_SB_TRUNK]);

    let studbook_config = jjdb_BlotterConfig {
        local_root: studbook_local,
        remote_url: sb_bare.to_string_lossy().into_owned(),
        trunk: ZJJTDM_SB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_gallops_journal_save(&jjrfg_PlainGit, &studbook_config, "jjtdm-seed", |_| zjjtdm_gallops(), "seed gallops".to_string())
        .unwrap();

    (infield, hippodrome, studbook_config)
}

fn zjjtdm_pace_billet(infield: &Path, hippodrome: &Path, coronet: &str) -> std::path::PathBuf {
    let billet_root = infield.join(jjrds_billet_dirname(coronet));
    jjrfg_PlainGit
        .jjrfr_billet_create(hippodrome, &jjrfr_BilletBirth::Branch(coronet.to_string()), &billet_root, ZJJTDM_HIP_TRUNK)
        .unwrap();
    billet_root
}

fn zjjtdm_groom_billet(infield: &Path, hippodrome: &Path, firemark: &str) -> std::path::PathBuf {
    let billet_root = infield.join(jjrds_billet_dirname(firemark));
    jjrfg_PlainGit.jjrfr_billet_create(hippodrome, &jjrfr_BilletBirth::Detached, &billet_root, ZJJTDM_HIP_TRUNK).unwrap();
    billet_root
}

fn zjjtdm_mark_live_officium(billet_root: &Path) {
    let officium_dir = billet_root.join(".claude/jjm/officia/260712-1000-abcd");
    std::fs::create_dir_all(&officium_dir).unwrap();
    std::fs::write(officium_dir.join("heartbeat"), "").unwrap();
}

// ---- Plan: emptiness and silence ----

#[test]
fn jjtdm_plan_is_empty_over_a_bare_infield() {
    let (infield, _hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_empty");
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();
    assert!(plan.jjrdm_is_empty());
}

// ---- Pace billets: closed/open × retention gates ----

#[test]
fn jjtdm_plan_reaps_a_closed_past_retention_clean_pace_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_closed_old_clean");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.reap.len(), 1);
    assert!(plan.dirty.is_empty());
    assert_eq!(plan.reap[0].billet_root, billet);
    assert_eq!(plan.reap[0].kind, jjrdm_Kind::Pace("AAAAC".to_string()));
}

#[test]
fn jjtdm_plan_keeps_a_closed_billet_still_within_retention() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_closed_fresh");
    let _billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    // No backdating: the billet is fresh.

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_empty(), "a fresh billet must never be a candidate regardless of pace state");
}

#[test]
fn jjtdm_plan_keeps_an_open_pace_billet_even_past_retention() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_open_old");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAA");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_empty(), "an open (rough) pace's billet must never be reaped");
}

#[test]
fn jjtdm_plan_keeps_a_billet_unknown_to_the_studbook_even_past_retention() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_unknown_old");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "ZZZZZ");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_empty(), "a pace absent from the studbook snapshot is never touched, never assumed closed");
}

// ---- The liveness join ----

#[test]
fn jjtdm_plan_keeps_a_reap_eligible_billet_carrying_a_live_officium() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_live_officium");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);
    zjjtdm_mark_live_officium(&billet);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_empty(), "a live officium overrides pace-closed and age — belt-and-braces");
}

// ---- Groom billets: no pace gate, retention alone ----

#[test]
fn jjtdm_plan_reaps_a_past_retention_clean_groom_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_groom_old_clean");
    let billet = zjjtdm_groom_billet(infield.path(), &hippodrome, "AA");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.reap.len(), 1);
    assert_eq!(plan.reap[0].kind, jjrdm_Kind::Groom("AA".to_string()));
}

#[test]
fn jjtdm_plan_keeps_a_fresh_groom_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_groom_fresh");
    let _billet = zjjtdm_groom_billet(infield.path(), &hippodrome, "AA");

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_empty());
}

// ---- Dirty anomalies ----

#[test]
fn jjtdm_plan_sorts_a_dirty_reap_eligible_pace_billet_into_dirty() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_dirty_pace");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.reap.is_empty());
    assert_eq!(plan.dirty.len(), 1);
    assert_eq!(plan.dirty[0].billet_root, billet);
}

#[test]
fn jjtdm_reap_refuses_a_dirty_candidate_by_default() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_dirty_refuse");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    let report = jjrdm_reap(&jjrfg_PlainGit, &plan, false);

    assert_eq!(report.outcomes.len(), 1);
    match &report.outcomes[0] {
        jjrdm_Outcome::Refused { billet_root, detail } => {
            assert_eq!(billet_root, &billet);
            assert!(detail.contains("auto-commit-push"));
        }
        other => panic!("expected Refused, got {:?}", other),
    }
    assert!(billet.exists(), "a refused billet must survive untouched");
}

#[test]
fn jjtdm_reap_salvages_a_dirty_pace_billet_when_confirmed() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_dirty_salvage");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    let report = jjrdm_reap(&jjrfg_PlainGit, &plan, true);

    assert_eq!(report.outcomes.len(), 1);
    assert!(matches!(&report.outcomes[0], jjrdm_Outcome::Salvaged(p) if p == &billet));
    assert!(!billet.exists(), "a salvaged billet must still be reaped");
    let remote_subject = zjjtdm_git(&hippodrome, &["log", "-1", "--pretty=%s", "refs/remotes/origin/AAAAC"]);
    assert_eq!(remote_subject, "muck: auto-commit-push before reap");
}

#[test]
fn jjtdm_reap_never_salvages_a_dirty_groom_billet_even_when_confirmed() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_dirty_groom");
    let billet = zjjtdm_groom_billet(infield.path(), &hippodrome, "AA");
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    let report = jjrdm_reap(&jjrfg_PlainGit, &plan, true);

    assert_eq!(report.outcomes.len(), 1);
    match &report.outcomes[0] {
        jjrdm_Outcome::Refused { billet_root, detail } => {
            assert_eq!(billet_root, &billet);
            assert!(detail.contains("nothing durable"));
        }
        other => panic!("expected Refused, got {:?}", other),
    }
    assert!(billet.exists(), "a dirty groom billet must never be swept, confirmed or not");
}

// ---- Reap execution on the clean set ----

#[test]
fn jjtdm_reap_removes_every_clean_candidate() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_clean");
    let pace_billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    let groom_billet = zjjtdm_groom_billet(infield.path(), &hippodrome, "AA");
    zjjtdm_backdate(&pace_billet, JJRDM_RETENTION_SECS + 3600);
    zjjtdm_backdate(&groom_billet, JJRDM_RETENTION_SECS + 3600);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();
    assert_eq!(plan.reap.len(), 2);

    let report = jjrdm_reap(&jjrfg_PlainGit, &plan, false);

    assert!(report.outcomes.iter().all(|o| matches!(o, jjrdm_Outcome::Reaped(_))));
    assert!(!pace_billet.exists());
    assert!(!groom_billet.exists());
}

// ---- The remaining resolved/open states ----

#[test]
fn jjtdm_plan_reaps_an_abandoned_pace_billet_past_retention() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_abandoned");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAD");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.reap.len(), 1, "abandoned is resolved, same as complete");
    assert_eq!(plan.reap[0].kind, jjrdm_Kind::Pace("AAAAD".to_string()));
}

#[test]
fn jjtdm_plan_keeps_a_bridled_pace_billet_even_past_retention() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_bridled_old");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAE");
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_empty(), "bridled is still open work, same as rough");
}

// ---- Salvage guards: JJ-owned paths and line-of-work drift ----

#[test]
fn jjtdm_reap_never_salvages_jj_owned_officium_content() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_jj_owned");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    // The only dirty content is a JJ-owned officium exchange file — nothing
    // legitimate to salvage, so a confirmed reap must still refuse.
    zjjtdm_mark_live_officium(&billet);
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);
    // Backdate the officium's own heartbeat so it reads as stale, not live —
    // the liveness guard and the JJ-owned-path filter are independent
    // concerns and this test isolates the latter.
    let heartbeat = billet.join(".claude/jjm/officia/260712-1000-abcd/heartbeat");
    zjjtdm_backdate(&heartbeat, JJRDM_RETENTION_SECS + 3600);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();
    assert_eq!(plan.dirty.len(), 1, "the officium exchange file makes the billet dirty");

    let report = jjrdm_reap(&jjrfg_PlainGit, &plan, true);

    assert_eq!(report.outcomes.len(), 1);
    match &report.outcomes[0] {
        jjrdm_Outcome::Refused { billet_root, detail } => {
            assert_eq!(billet_root, &billet);
            assert!(detail.contains("nothing legitimate to salvage"));
        }
        other => panic!("expected Refused, got {:?}", other),
    }
    assert!(billet.exists(), "must never lodge JJ-owned officium content into the work repo");
}

#[test]
fn jjtdm_reap_refuses_to_salvage_a_billet_whose_checkout_drifted() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_drifted_checkout");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC");
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    zjjtdm_backdate(&billet, JJRDM_RETENTION_SECS + 3600);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), ZJJTDM_GUIDON).unwrap();
    assert_eq!(plan.dirty.len(), 1);

    // Between plan and reap, the billet's checkout drifts off its pace
    // branch by hand — salvage must refuse rather than lodge onto the
    // wrong line and consign the untouched coronet branch.
    zjjtdm_git(&billet, &["checkout", "-q", "-b", "some-other-line"]);

    let report = jjrdm_reap(&jjrfg_PlainGit, &plan, true);

    assert_eq!(report.outcomes.len(), 1);
    match &report.outcomes[0] {
        jjrdm_Outcome::Refused { billet_root, detail } => {
            assert_eq!(billet_root, &billet);
            assert!(detail.contains("no longer seats branch"));
        }
        other => panic!("expected Refused, got {:?}", other),
    }
}
