// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
    jjrfr_RejectionKind,
};
use super::jjrrf_refit::{
    jjrrf_refit,
    jjrrf_RefitOutcome,
};
use super::jjtu_testdir::JjkTestDir;
use std::path::Path;

const ZJJTRF_TRUNK: &str = "jjtrf-trunk";
const ZJJTRF_BRANCH: &str = "jjtrf-billet";

fn zjjtrf_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtrf_init_local(dir: &Path) {
    zjjtrf_git(dir, &["init", "-q", "-b", ZJJTRF_TRUNK]);
    zjjtrf_git(dir, &["config", "user.email", "jjtrf@example.invalid"]);
    zjjtrf_git(dir, &["config", "user.name", "jjtrf"]);
}

fn zjjtrf_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtrf_git(dir, &["add", "--", name]);
    zjjtrf_git(dir, &["commit", "-q", "-m", message]);
    zjjtrf_git(dir, &["rev-parse", "HEAD"])
}

/// Advance trunk the way a *published* trunk moves: commit on the primary and
/// push. Refit reads the trunk's counterpart, so only a published advance is one
/// it can see — an unpushed commit moves the operator's local ref alone.
fn zjjtrf_trunk_advances(primary: &Path, name: &str, content: &str, message: &str) -> String {
    let sha = zjjtrf_commit_all(primary, name, content, message);
    zjjtrf_git(primary, &["push", "-q", "origin", ZJJTRF_TRUNK]);
    sha
}

fn zjjtrf_billet_slot(name: &str) -> JjkTestDir {
    let slot = JjkTestDir::new(name);
    std::fs::remove_dir_all(slot.path()).unwrap();
    slot
}

/// Bare remote, a primary tracking it with a pushed baseline commit, and a pace
/// billet forked off trunk at that baseline — the shared precondition every
/// refit test builds on.
fn zjjtrf_billeted_primary(name: &str) -> (JjkTestDir, JjkTestDir, JjkTestDir) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtrf_git(bare.path(), &["init", "-q", "--bare", "-b", ZJJTRF_TRUNK]);
    let primary = JjkTestDir::new(&format!("{}_primary", name));
    zjjtrf_init_local(primary.path());
    zjjtrf_commit_all(primary.path(), "base.txt", "base", "init");
    zjjtrf_git(primary.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtrf_git(primary.path(), &["push", "-q", "-u", "origin", ZJJTRF_TRUNK]);
    let billet = zjjtrf_billet_slot(&format!("{}_billet", name));
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch(ZJJTRF_BRANCH.to_string()), billet.path(), ZJJTRF_TRUNK)
        .unwrap();
    (bare, primary, billet)
}

/// Whether the bare remote carries the billet branch at all — the check that a
/// refit pushed nothing.
fn zjjtrf_remote_has_billet_branch(bare: &Path) -> bool {
    zjjtrf_git(bare, &["for-each-ref", "--format=%(refname)"]).contains(ZJJTRF_BRANCH)
}

#[test]
fn jjtrf_refit_reports_up_to_date_when_trunk_unmoved() {
    let (_bare, _primary, billet) = zjjtrf_billeted_primary("jjtrf_up_to_date");

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::UpToDate);
}

#[test]
fn jjtrf_refit_merges_trunk_in_and_consigns_the_billet_branch() {
    let (bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_refitted");
    zjjtrf_trunk_advances(primary.path(), "b.txt", "trunk moved on", "trunk advances");

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::Refitted);
    assert!(billet.path().join("b.txt").exists());
    let billet_head = zjjtrf_git(billet.path(), &["rev-parse", "HEAD"]);
    let remote_tip = zjjtrf_git(bare.path(), &["rev-parse", ZJJTRF_BRANCH]);
    assert_eq!(remote_tip, billet_head, "the merge must be pushed immediately");
}

#[test]
fn jjtrf_refit_fetches_a_trunk_advanced_by_another_station() {
    let (bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_two_station");

    // Another station publishes trunk. This station's local trunk ref never
    // moves — only a fetch reveals the advance, and refit is what fetches.
    let other = JjkTestDir::new("jjtrf_two_station_other");
    zjjtrf_git(
        bare.path(),
        &["clone", "-q", "-b", ZJJTRF_TRUNK, &bare.path().to_string_lossy(), &other.path().to_string_lossy()],
    );
    zjjtrf_git(other.path(), &["config", "user.email", "jjtrf@example.invalid"]);
    zjjtrf_git(other.path(), &["config", "user.name", "jjtrf"]);
    zjjtrf_commit_all(other.path(), "other.txt", "from another station", "other station advances trunk");
    zjjtrf_git(other.path(), &["push", "-q", "origin", ZJJTRF_TRUNK]);

    let local_trunk_before = zjjtrf_git(primary.path(), &["rev-parse", ZJJTRF_TRUNK]);

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::Refitted);
    assert!(billet.path().join("other.txt").exists(), "refit must fetch the published trunk and merge it");
    assert_eq!(
        zjjtrf_git(primary.path(), &["rev-parse", ZJJTRF_TRUNK]),
        local_trunk_before,
        "refit must never advance the operator's own trunk",
    );
}

#[test]
fn jjtrf_refit_never_consigns_the_operators_unpushed_trunk_work() {
    let (bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_no_exfil");
    zjjtrf_commit_all(billet.path(), "billet.txt", "billet work", "billet commit");
    // The operator's own trunk work: committed, not published, still mutable.
    zjjtrf_commit_all(primary.path(), "unpushed.txt", "operator's own", "unpushed trunk work");

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    // The counterpart never moved, so there is nothing to enfold and nothing to
    // push — and the unpublished commit does not ride out as billet ancestry.
    assert_eq!(outcome, jjrrf_RefitOutcome::UpToDate);
    assert!(!billet.path().join("unpushed.txt").exists());
    assert!(
        !zjjtrf_remote_has_billet_branch(bare.path()),
        "an unpushed trunk must never reach the remote as billet ancestry",
    );
}

#[test]
fn jjtrf_refit_never_rebases_the_billets_own_commits_survive() {
    let (_bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_never_rebase");
    let billet_commit = zjjtrf_commit_all(billet.path(), "billet.txt", "billet work", "billet commit");
    zjjtrf_trunk_advances(primary.path(), "trunk.txt", "trunk work", "trunk commit");

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::Refitted);
    // The billet's own commit survives verbatim as a merge parent — a rebase
    // would have rewritten it under a new SHA instead.
    let parents = zjjtrf_git(billet.path(), &["log", "--pretty=%P", "-1"]);
    assert!(parents.contains(&billet_commit), "the billet commit must survive as a merge parent, not be rewritten");
    assert!(billet.path().join("trunk.txt").exists());
    assert!(billet.path().join("billet.txt").exists());
}

#[test]
fn jjtrf_refit_offline_merges_what_was_gleaned_but_pushes_nothing() {
    let (bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_offline");
    // Trunk advances and is published while the station can still reach the
    // remote, so the counterpart is current on disk...
    zjjtrf_trunk_advances(primary.path(), "b.txt", "trunk moved on", "trunk advances");
    // ...and only then does the remote go away.
    zjjtrf_git(billet.path(), &["remote", "set-url", "origin", "/nonexistent/jjtrf-nowhere"]);

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::OfflineWarned);
    assert!(billet.path().join("b.txt").exists(), "the last-gleaned trunk must still merge locally");
    assert!(!zjjtrf_remote_has_billet_branch(bare.path()), "an offline refit must push nothing");
}

#[test]
fn jjtrf_refit_offline_reports_warned_even_when_the_merge_was_a_no_op() {
    let (_bare, _primary, billet) = zjjtrf_billeted_primary("jjtrf_offline_noop");
    zjjtrf_git(billet.path(), &["remote", "set-url", "origin", "/nonexistent/jjtrf-nowhere"]);

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    // Never UpToDate: unverified sameness is ignorance, not freshness, and a
    // remedy verb cannot relabel one as the other.
    assert_eq!(outcome, jjrrf_RefitOutcome::OfflineWarned);
}

#[test]
fn jjtrf_refit_propagates_a_dirty_billet_rejection_unchanged() {
    let (_bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_dirty");
    zjjtrf_trunk_advances(primary.path(), "b.txt", "trunk moved on", "trunk advances");
    std::fs::write(billet.path().join("dirt.txt"), "uncommitted").unwrap();

    let result = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}
