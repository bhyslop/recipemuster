// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrdm_muck::{
    jjrdm_plan,
    jjrdm_reap,
    jjrdm_report,
    jjrdm_Arm,
    jjrdm_Kind,
    jjrdm_PaceEvidence,
    jjrdm_Rejection,
};
use super::jjrds_stile::{jjrds_billet_dirname, JJRDS_SCRATCH_DIRNAME};
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
const ZJJTDM_SERIAL: u64 = 200500;
const ZJJTDM_SERIAL_2: u64 = 200501;

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

fn zjjtdm_tack(state: jjrg_PaceState) -> jjrg_Tack {
    jjrg_Tack {
        ts: "260712-1200".to_string(),
        state,
        tier: None,
        effort: None,
        text: vec!["a docket line".to_string()],
        silks: "muck-test-pace".to_string(),
        basis: "abc1234".to_string(),
    }
}

/// A studbook gallops with one heat carrying a complete pace (₢AAAAC) and a
/// rough pace (₢AAAAA) — enough shape to prove pace evidence reads the real
/// tack when one stands, and reports honestly when it does not.
fn zjjtdm_gallops() -> jjrg_Gallops {
    let mut paces = std::collections::BTreeMap::new();
    paces.insert("₢AAAAC".to_string(), jjrg_Pace { tacks: vec![zjjtdm_tack(jjrg_PaceState::Complete)], ..Default::default() });
    paces.insert("₢AAAAA".to_string(), jjrg_Pace { tacks: vec![zjjtdm_tack(jjrg_PaceState::Rough)], ..Default::default() });
    let mut heats = std::collections::BTreeMap::new();
    heats.insert("₣AA".to_string(), jjrg_Heat {
        silks: "muck-test-heat".to_string(),
        creation_time: "260712".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec!["₢AAAAC".to_string(), "₢AAAAA".to_string()],
        paces,
    });
    jjrg_Gallops { next_heat_seed: "AB".to_string(), next_pace_seed: "CAAAA".to_string(), heat_order: vec!["₣AA".to_string()], heats, retention_since: None }
}

/// The combined fixture: an infield holding a hippodrome (its own bare
/// upstream, for billet creation) and a studbook (its own bare upstream, for
/// the lock guidon and the gallops journal), the studbook pre-seeded with
/// `zjjtdm_gallops`.
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

fn zjjtdm_pace_billet(infield: &Path, hippodrome: &Path, coronet: &str, serial: u64) -> std::path::PathBuf {
    // Yard signet on the dirname, livery badge on the branch — the same split
    // the stile's approach makes, so salvage meets here what it meets in the field.
    let billet_root = infield.join(jjrds_billet_dirname(serial, coronet));
    let branch = crate::jjrf_favor::jjrf_livery_compose(None, crate::jjrf_favor::jjrf_LiveryKind::Pace, coronet);
    jjrfg_PlainGit
        .jjrfr_billet_create(hippodrome, &jjrfr_BilletBirth::Branch(branch), &billet_root, ZJJTDM_HIP_TRUNK)
        .unwrap();
    billet_root
}

fn zjjtdm_groom_billet(infield: &Path, hippodrome: &Path, firemark: &str, serial: u64) -> std::path::PathBuf {
    let billet_root = infield.join(jjrds_billet_dirname(serial, firemark));
    jjrfg_PlainGit.jjrfr_billet_create(hippodrome, &jjrfr_BilletBirth::Detached, &billet_root, ZJJTDM_HIP_TRUNK).unwrap();
    billet_root
}

fn zjjtdm_scratch_dir(infield: &Path, billet_dirname: &str) -> std::path::PathBuf {
    let dir = infield.join(JJRDS_SCRATCH_DIRNAME).join(billet_dirname);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

// ---- Resolution ----

#[test]
fn jjtdm_plan_resolves_a_named_dirname_directly() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_resolve_dirname");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    let dirname = jjrds_billet_dirname(ZJJTDM_SERIAL, "AAAAC");

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), &dirname, ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.billet_root, billet);
    assert_eq!(plan.kind, jjrdm_Kind::Pace("AAAAC".to_string()));
}

#[test]
fn jjtdm_plan_resolves_by_bare_identity() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_resolve_identity");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.billet_root, billet);
}

#[test]
fn jjtdm_plan_refuses_a_name_with_no_billet() {
    let (infield, _hippodrome, studbook) = zjjtdm_fixture("jjtdm_resolve_not_found");

    let result = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "ZZZZZ", ZJJTDM_GUIDON);

    assert!(matches!(result, Err(jjrdm_Rejection::NotFound { .. })));
}

#[test]
fn jjtdm_plan_refuses_an_ambiguous_groom_identity() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_resolve_ambiguous");
    // Two concurrent groom billets of one heat share a firemark, distinguished
    // only by their serials (JJSVD "Catchword-serialed billets") — naming the
    // bare firemark cannot pick one.
    zjjtdm_groom_billet(infield.path(), &hippodrome, "AA", ZJJTDM_SERIAL);
    zjjtdm_groom_billet(infield.path(), &hippodrome, "AA", ZJJTDM_SERIAL_2);

    let result = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AA", ZJJTDM_GUIDON);

    match result {
        Err(jjrdm_Rejection::Ambiguous { candidates, .. }) => assert_eq!(candidates.len(), 2),
        other => panic!("expected Ambiguous, got {:?}", other),
    }
}

// ---- Plan content: dirt, sync posture, arms ----

#[test]
fn jjtdm_plan_reports_a_clean_pace_billet_with_destroy_only() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_clean");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    assert!(!plan.jjrdm_is_dirty());
    assert!(plan.dirty_paths.is_empty());
    assert_eq!(plan.jjrdm_available_arms(), vec![jjrdm_Arm::Destroy]);
}

#[test]
fn jjtdm_plan_reports_dirty_paths_by_name_and_opens_both_arms_on_a_pace_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_dirty_pace");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_dirty());
    assert!(plan.dirty_paths.iter().any(|p| p.to_string_lossy().contains("wip.txt")));
    assert_eq!(plan.jjrdm_available_arms(), vec![jjrdm_Arm::Destroy, jjrdm_Arm::SalvageThenDestroy]);

    let report = jjrdm_report(&plan);
    assert!(report.contains("wip.txt"));
    assert!(report.contains("DIRTY"));
    assert!(report.contains("salvage-then-destroy"));
}

#[test]
fn jjtdm_plan_dirty_groom_billet_opens_destroy_only() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_dirty_groom");
    let billet = zjjtdm_groom_billet(infield.path(), &hippodrome, "AA", ZJJTDM_SERIAL);
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AA", ZJJTDM_GUIDON).unwrap();

    assert!(plan.jjrdm_is_dirty());
    assert_eq!(plan.jjrdm_available_arms(), vec![jjrdm_Arm::Destroy]);

    let report = jjrdm_report(&plan);
    assert!(!report.contains("salvage-then-destroy"));
}

#[test]
fn jjtdm_plan_reports_unavailable_pace_evidence_when_no_studbook_is_founded_rather_than_crashing() {
    // A station that never founded its studbook has no clone to stake at all
    // — the journal read bracket must report this as unavailable evidence,
    // never let an unclassified git failure escape as a panic. Point the
    // studbook config at a path that was never created.
    let (infield, hippodrome, mut studbook) = zjjtdm_fixture("jjtdm_plan_no_studbook");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    studbook.local_root = infield.path().join("no-such-studbook");

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    assert!(matches!(plan.pace_evidence, Some(jjrdm_PaceEvidence::Unavailable(_))));
}

// ---- Pace evidence: reported, never a gate ----

#[test]
fn jjtdm_plan_reports_the_known_tack_as_pace_evidence() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_evidence_known");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    match &plan.pace_evidence {
        Some(jjrdm_PaceEvidence::Tack { resolved, state_label, .. }) => {
            assert!(*resolved, "complete is a resolved state");
            assert_eq!(state_label, "complete");
        }
        other => panic!("expected known Tack evidence, got {:?}", other),
    }

    let report = jjrdm_report(&plan);
    assert!(report.contains("complete"));
    assert!(report.contains("never a gate"));
}

#[test]
fn jjtdm_plan_reports_a_still_open_pace_honestly_without_gating() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_evidence_open");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAA", ZJJTDM_SERIAL);

    // Rough is an open pace state — the plan must still resolve; evidence is
    // never a gate, so an open pace does not block the plan from forming.
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAA", ZJJTDM_GUIDON).unwrap();

    match &plan.pace_evidence {
        Some(jjrdm_PaceEvidence::Tack { resolved, .. }) => assert!(!resolved),
        other => panic!("expected known Tack evidence, got {:?}", other),
    }
}

#[test]
fn jjtdm_plan_reports_unknown_pace_evidence_for_a_coronet_absent_from_the_studbook() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_evidence_unknown");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "ZZZZZ", ZJJTDM_SERIAL);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "ZZZZZ", ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.pace_evidence, Some(jjrdm_PaceEvidence::Unknown));
}

#[test]
fn jjtdm_plan_carries_no_pace_evidence_for_a_groom_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_plan_evidence_groom");
    zjjtdm_groom_billet(infield.path(), &hippodrome, "AA", ZJJTDM_SERIAL);

    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AA", ZJJTDM_GUIDON).unwrap();

    assert_eq!(plan.pace_evidence, None);
}

// ---- The confirm gate: reap only executes an arm the plan opened ----

#[test]
fn jjtdm_reap_refuses_an_arm_the_plan_never_opened() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_invalid_arm");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();
    assert_eq!(plan.jjrdm_available_arms(), vec![jjrdm_Arm::Destroy], "clean billet opens destroy alone");

    let result = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::SalvageThenDestroy);

    assert!(matches!(result, Err(jjrdm_Rejection::InvalidArm(_))));
    assert!(plan.billet_root.exists(), "a refused reap must leave the billet untouched");
}

#[test]
fn jjtdm_reap_refuses_salvage_on_a_dirty_groom_billet_even_requested() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_groom_salvage_refused");
    let billet = zjjtdm_groom_billet(infield.path(), &hippodrome, "AA", ZJJTDM_SERIAL);
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AA", ZJJTDM_GUIDON).unwrap();

    let result = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::SalvageThenDestroy);

    assert!(matches!(result, Err(jjrdm_Rejection::InvalidArm(_))));
    assert!(billet.exists());
}

// ---- Reap execution ----

#[test]
fn jjtdm_reap_destroys_a_clean_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_clean");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    let outcome = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::Destroy).unwrap();

    assert!(!billet.exists());
    assert!(!outcome.salvaged);
}

#[test]
fn jjtdm_reap_destroys_a_dirty_billet_via_the_forced_destroy_arm() {
    // Destroy is open even when dirty — muck is the constellation's one
    // deliberate data-loss surface, and `billet_remove`'s force is the
    // confirmed destroy arm's alone.
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_dirty_destroy");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::Destroy).unwrap();

    assert!(!billet.exists());
}

#[test]
fn jjtdm_reap_salvages_a_dirty_pace_billet_before_destroying_it() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_salvage");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    let outcome = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::SalvageThenDestroy).unwrap();

    assert!(outcome.salvaged);
    assert!(!billet.exists());
    // Salvage consigns the badged branch, so the badge is what lands in the
    // sire's ref store — the mint's whole point, observed at the far end.
    let remote_subject = zjjtdm_git(&hippodrome, &["log", "-1", "--pretty=%s", "refs/remotes/origin/jjls_pace/AAAAC"]);
    assert_eq!(remote_subject, "muck: salvage before destroy");
}

#[test]
fn jjtdm_reap_never_salvages_jj_owned_officium_content() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_jj_owned");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    let officium_dir = billet.join(".claude/jjm/officia/260712-1000-abcd");
    std::fs::create_dir_all(&officium_dir).unwrap();
    std::fs::write(officium_dir.join("heartbeat"), "").unwrap();
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();
    assert!(plan.jjrdm_is_dirty(), "the officium exchange file makes the billet dirty");

    let result = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::SalvageThenDestroy);

    match result {
        Err(jjrdm_Rejection::Farrier(r)) => assert!(r.detail.contains("nothing legitimate to salvage")),
        other => panic!("expected a Farrier DirtyTree rejection, got {:?}", other),
    }
    assert!(billet.exists(), "must never lodge JJ-owned officium content into the work repo");
}

#[test]
fn jjtdm_reap_refuses_to_salvage_a_billet_whose_checkout_drifted() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_drifted_checkout");
    let billet = zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    std::fs::write(billet.join("wip.txt"), "uncommitted").unwrap();
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    // Between plan and reap, the billet's checkout drifts off its pace
    // branch by hand — salvage must refuse rather than lodge onto the wrong
    // line and consign the untouched livery branch.
    zjjtdm_git(&billet, &["checkout", "-q", "-b", "some-other-line"]);

    let result = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::SalvageThenDestroy);

    match result {
        Err(jjrdm_Rejection::Farrier(r)) => assert!(r.detail.contains("no longer seats pace")),
        other => panic!("expected a Farrier DirtyTree rejection, got {:?}", other),
    }
}

// ---- Scratch sweep ----

#[test]
fn jjtdm_reap_clears_the_destroyed_billets_own_scratch_sibling() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_own_scratch");
    let dirname = jjrds_billet_dirname(ZJJTDM_SERIAL, "AAAAC");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    let scratch = zjjtdm_scratch_dir(infield.path(), &dirname);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    let outcome = jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::Destroy).unwrap();

    assert!(!scratch.exists());
    assert!(outcome.scratch_swept.contains(&scratch));
}

#[test]
fn jjtdm_reap_clears_killed_door_orphan_scratch_left_by_an_unrelated_billet() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_orphan_scratch");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    // A scratch directory whose billet no longer stands at all — the residue
    // a killed dispatch door leaves behind.
    let orphan_dirname = jjrds_billet_dirname(ZJJTDM_SERIAL_2, "ZZZZZ");
    let orphan = zjjtdm_scratch_dir(infield.path(), &orphan_dirname);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::Destroy).unwrap();

    assert!(!orphan.exists(), "killed-door orphan scratch is this door's to clear too");
}

#[test]
fn jjtdm_reap_leaves_a_standing_billets_own_scratch_untouched() {
    let (infield, hippodrome, studbook) = zjjtdm_fixture("jjtdm_reap_other_scratch_survives");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAC", ZJJTDM_SERIAL);
    let other_dirname = jjrds_billet_dirname(ZJJTDM_SERIAL_2, "AAAAA");
    zjjtdm_pace_billet(infield.path(), &hippodrome, "AAAAA", ZJJTDM_SERIAL_2);
    let other_scratch = zjjtdm_scratch_dir(infield.path(), &other_dirname);
    let plan = jjrdm_plan(&jjrfg_PlainGit, &studbook, infield.path(), "AAAAC", ZJJTDM_GUIDON).unwrap();

    jjrdm_reap(&jjrfg_PlainGit, infield.path(), &plan, jjrdm_Arm::Destroy).unwrap();

    assert!(other_scratch.exists(), "a still-standing billet's own scratch must survive");
}
