// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_FarrierBillet,
    jjrfr_LineOfWork,
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

fn zjjtrf_init_bare(dir: &Path) {
    zjjtrf_git(dir, &["init", "-q", "--bare", "-b", ZJJTRF_TRUNK]);
}

fn zjjtrf_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    zjjtrf_git(dir, &["add", "--", name]);
    zjjtrf_git(dir, &["commit", "-q", "-m", message]);
    zjjtrf_git(dir, &["rev-parse", "HEAD"])
}

fn zjjtrf_billet_slot(name: &str) -> JjkTestDir {
    let slot = JjkTestDir::new(name);
    std::fs::remove_dir_all(slot.path()).unwrap();
    slot
}

/// Bare remote, a primary tracking it with a pushed baseline commit, and a
/// pace billet forked off trunk at that baseline — the shared precondition
/// every refit test builds on.
fn zjjtrf_billeted_primary(name: &str) -> (JjkTestDir, JjkTestDir, JjkTestDir) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtrf_init_bare(bare.path());
    let primary = JjkTestDir::new(&format!("{}_primary", name));
    zjjtrf_init_local(primary.path());
    zjjtrf_commit_all(primary.path(), "base.txt", "base", "init");
    zjjtrf_git(primary.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtrf_git(primary.path(), &["push", "-q", "-u", "origin", ZJJTRF_TRUNK]);
    let billet = zjjtrf_billet_slot(&format!("{}_billet", name));
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch(ZJJTRF_BRANCH.to_string()), billet.path())
        .unwrap();
    (bare, primary, billet)
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
    zjjtrf_commit_all(primary.path(), "b.txt", "trunk moved on", "trunk advances");

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::Refitted);
    assert!(billet.path().join("b.txt").exists());
    let billet_head = zjjtrf_git(billet.path(), &["rev-parse", "HEAD"]);
    let remote_tip = zjjtrf_git(bare.path(), &["rev-parse", ZJJTRF_BRANCH]);
    assert_eq!(remote_tip, billet_head, "the merge commit must be pushed immediately");
}

#[test]
fn jjtrf_refit_never_rebases_the_billets_own_commits_survive() {
    let (_bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_never_rebase");
    let billet_commit = zjjtrf_commit_all(billet.path(), "billet.txt", "billet work", "billet commit");
    zjjtrf_commit_all(primary.path(), "trunk.txt", "trunk work", "trunk commit");

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::Refitted);
    // The billet's own commit survives verbatim as a merge parent — a rebase
    // would have rewritten it under a new SHA instead.
    let parents = zjjtrf_git(billet.path(), &["log", "--pretty=%P", "-1"]);
    assert!(parents.contains(&billet_commit[..7]), "billet commit must survive as a merge parent, not be rewritten");
    assert!(billet.path().join("trunk.txt").exists());
    assert!(billet.path().join("billet.txt").exists());
}

#[test]
fn jjtrf_refit_offline_merges_locally_but_warns_instead_of_pushing() {
    let (bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_offline");
    let baseline_remote = zjjtrf_git(bare.path(), &["rev-parse", ZJJTRF_TRUNK]);
    zjjtrf_commit_all(primary.path(), "b.txt", "trunk moved on", "trunk advances");
    zjjtrf_git(billet.path(), &["remote", "set-url", "origin", "/nonexistent/jjtrf-nowhere"]);

    let outcome = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK).unwrap();

    assert_eq!(outcome, jjrrf_RefitOutcome::OfflineWarned);
    assert!(billet.path().join("b.txt").exists(), "the merge must land locally even when offline");
    let remote_tip = zjjtrf_git(bare.path(), &["rev-parse", ZJJTRF_TRUNK]);
    assert_eq!(remote_tip, baseline_remote, "an offline refit must push nothing");
}

#[test]
fn jjtrf_refit_propagates_a_dirty_billet_rejection_unchanged() {
    let (_bare, primary, billet) = zjjtrf_billeted_primary("jjtrf_dirty");
    zjjtrf_commit_all(primary.path(), "b.txt", "trunk moved on", "trunk advances");
    std::fs::write(billet.path().join("dirt.txt"), "uncommitted").unwrap();

    let result = jjrrf_refit(&jjrfg_PlainGit, billet.path(), ZJJTRF_BRANCH, ZJJTRF_TRUNK);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}
