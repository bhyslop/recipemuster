// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_ConsignLease, jjrfr_FarrierCore, jjrfr_GleanOutcome, jjrfr_LineOfWork, jjrfr_RejectionKind, jjrfr_Seat,
    jjrfr_SyncState,
};
use super::jjtu_testdir::JjkTestDir;
use std::path::{Path, PathBuf};

const ZJJTFG_TRUNK: &str = "jjtfg-trunk";

fn zjjtfg_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtfg_init_local(dir: &Path) {
    zjjtfg_git(dir, &["init", "-q", "-b", ZJJTFG_TRUNK]);
    zjjtfg_git(dir, &["config", "user.email", "jjtfg@example.invalid"]);
    zjjtfg_git(dir, &["config", "user.name", "jjtfg"]);
}

fn zjjtfg_init_bare(dir: &Path) {
    zjjtfg_git(dir, &["init", "-q", "--bare", "-b", ZJJTFG_TRUNK]);
}

fn zjjtfg_write(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    PathBuf::from(name)
}

fn zjjtfg_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    zjjtfg_write(dir, name, content);
    zjjtfg_git(dir, &["add", "--", name]);
    zjjtfg_git(dir, &["commit", "-q", "-m", message]);
    zjjtfg_git(dir, &["rev-parse", "HEAD"])
}

#[test]
fn jjtfg_identify_claims_a_git_tree() {
    let td = JjkTestDir::new("jjtfg_identify_claims_a_git_tree");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");

    let identity = jjrfg_PlainGit.jjrfr_identify(td.path()).expect("a git tree must be claimed");

    assert_eq!(identity.seat, jjrfr_Seat::Primary);
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Branch(ZJJTFG_TRUNK.to_string()));
    assert_eq!(identity.upstream_key, "");
}

#[test]
fn jjtfg_identify_declines_non_git_dir() {
    let td = JjkTestDir::new("jjtfg_identify_declines_non_git_dir");

    let result = jjrfg_PlainGit.jjrfr_identify(td.path());

    let rejection = result.expect_err("a plain directory must not be claimed");
    assert_eq!(rejection.kind, jjrfr_RejectionKind::ForeignGround);
}

#[test]
fn jjtfg_comb_reports_dirty_paths() {
    let td = JjkTestDir::new("jjtfg_comb_reports_dirty_paths");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");
    zjjtfg_write(td.path(), "b.txt", "untracked");

    let comb = jjrfg_PlainGit.jjrfr_comb(td.path()).unwrap();

    assert!(!comb.jjrfr_is_clean());
    assert!(comb.dirty_paths.contains(&PathBuf::from("b.txt")));
}

#[test]
fn jjtfg_sync_state_is_untracked_without_upstream() {
    let td = JjkTestDir::new("jjtfg_sync_state_is_untracked_without_upstream");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");

    let state = jjrfg_PlainGit.jjrfr_sync_state(td.path()).unwrap();

    assert_eq!(state, jjrfr_SyncState::Untracked);
}

#[test]
fn jjtfg_sync_state_tracks_ahead_after_local_commit() {
    let bare = JjkTestDir::new("jjtfg_sync_state_tracks_ahead_bare");
    zjjtfg_init_bare(bare.path());
    let local = JjkTestDir::new("jjtfg_sync_state_tracks_ahead_local");
    zjjtfg_init_local(local.path());
    zjjtfg_commit_all(local.path(), "a.txt", "hello", "init");
    zjjtfg_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtfg_git(local.path(), &["push", "-q", "-u", "origin", ZJJTFG_TRUNK]);

    zjjtfg_commit_all(local.path(), "b.txt", "second", "second");

    let state = jjrfg_PlainGit.jjrfr_sync_state(local.path()).unwrap();

    assert_eq!(state, jjrfr_SyncState::Tracking { ahead: 1, behind: 0 });
}

#[test]
fn jjtfg_counterfoil_reports_sha_and_dirty_flag() {
    let td = JjkTestDir::new("jjtfg_counterfoil_reports_sha_and_dirty_flag");
    zjjtfg_init_local(td.path());
    let sha = zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");

    let clean = jjrfg_PlainGit.jjrfr_counterfoil(td.path()).unwrap();
    assert_eq!(clean.members.get("."), Some(&sha));
    assert_eq!(clean.line_of_work, jjrfr_LineOfWork::Branch(ZJJTFG_TRUNK.to_string()));
    assert!(!clean.dirty);

    zjjtfg_write(td.path(), "b.txt", "untracked");
    let dirty = jjrfg_PlainGit.jjrfr_counterfoil(td.path()).unwrap();
    assert!(dirty.dirty);
}

#[test]
fn jjtfg_lodge_commits_only_the_explicit_file_list() {
    let td = JjkTestDir::new("jjtfg_lodge_commits_only_the_explicit_file_list");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");
    zjjtfg_write(td.path(), "wanted.txt", "wanted");
    zjjtfg_write(td.path(), "unwanted.txt", "unwanted");

    jjrfg_PlainGit
        .jjrfr_lodge(td.path(), &[PathBuf::from("wanted.txt")], "lodge wanted only")
        .unwrap();

    let subject = zjjtfg_git(td.path(), &["log", "-1", "--pretty=%s"]);
    assert_eq!(subject, "lodge wanted only");

    let comb = jjrfg_PlainGit.jjrfr_comb(td.path()).unwrap();
    assert!(comb.dirty_paths.contains(&PathBuf::from("unwanted.txt")));
    assert!(!comb.dirty_paths.iter().any(|p| p == Path::new("wanted.txt")));
}

#[test]
fn jjtfg_glean_updated_when_remote_reachable() {
    let bare = JjkTestDir::new("jjtfg_glean_updated_bare");
    zjjtfg_init_bare(bare.path());
    let local = JjkTestDir::new("jjtfg_glean_updated_local");
    zjjtfg_init_local(local.path());
    zjjtfg_commit_all(local.path(), "a.txt", "hello", "init");
    zjjtfg_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtfg_git(local.path(), &["push", "-q", "-u", "origin", ZJJTFG_TRUNK]);

    let outcome = jjrfg_PlainGit.jjrfr_glean(local.path());

    assert_eq!(outcome, jjrfr_GleanOutcome::Updated);
}

#[test]
fn jjtfg_glean_unreachable_when_remote_absent() {
    let td = JjkTestDir::new("jjtfg_glean_unreachable_when_remote_absent");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");
    zjjtfg_git(td.path(), &["remote", "add", "origin", "/nonexistent/jjtfg-nowhere"]);

    let outcome = jjrfg_PlainGit.jjrfr_glean(td.path());

    assert_eq!(outcome, jjrfr_GleanOutcome::Unreachable);
}

/// Sets up a bare remote plus two independent local clones both tracking it, each
/// having pushed a baseline commit — the shared ancestor divergence tests fork from.
/// `name` must be unique per caller: tests run concurrently, and a shared fixed
/// directory name races two tests' git processes against the same path.
fn zjjtfg_two_clones_from_baseline(bare: &Path, name: &str) -> (JjkTestDir, JjkTestDir) {
    zjjtfg_init_bare(bare);

    let local1 = JjkTestDir::new(&format!("{}_local1", name));
    zjjtfg_init_local(local1.path());
    zjjtfg_commit_all(local1.path(), "base.txt", "base", "init");
    zjjtfg_git(local1.path(), &["remote", "add", "origin", &bare.to_string_lossy()]);
    zjjtfg_git(local1.path(), &["push", "-q", "-u", "origin", ZJJTFG_TRUNK]);

    let local2 = JjkTestDir::new(&format!("{}_local2", name));
    zjjtfg_git(bare, &["clone", "-q", "-b", ZJJTFG_TRUNK, &bare.to_string_lossy(), &local2.path().to_string_lossy()]);
    zjjtfg_git(local2.path(), &["config", "user.email", "jjtfg@example.invalid"]);
    zjjtfg_git(local2.path(), &["config", "user.name", "jjtfg"]);

    (local1, local2)
}

#[test]
fn jjtfg_advance_fast_forwards_to_remote_tip() {
    let bare = JjkTestDir::new("jjtfg_advance_ff_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_ff");

    let tip = zjjtfg_commit_all(local2.path(), "ahead.txt", "ahead", "ahead");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);

    jjrfg_PlainGit.jjrfr_glean(local1.path());
    jjrfg_PlainGit
        .jjrfr_advance(local1.path(), &format!("{}/{}", "origin", ZJJTFG_TRUNK))
        .unwrap();

    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, tip);
}

#[test]
fn jjtfg_advance_rejects_dirty_tree() {
    let bare = JjkTestDir::new("jjtfg_advance_dirty_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_dirty");
    zjjtfg_commit_all(local2.path(), "ahead.txt", "ahead", "ahead");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);
    jjrfg_PlainGit.jjrfr_glean(local1.path());
    zjjtfg_write(local1.path(), "uncommitted.txt", "dirty");

    let result = jjrfg_PlainGit.jjrfr_advance(local1.path(), &format!("{}/{}", "origin", ZJJTFG_TRUNK));

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}

#[test]
fn jjtfg_advance_rejects_diverged_when_ff_impossible() {
    let bare = JjkTestDir::new("jjtfg_advance_diverged_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_diverged");

    zjjtfg_commit_all(local2.path(), "from-local2.txt", "from local2", "from local2");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);

    zjjtfg_commit_all(local1.path(), "from-local1.txt", "from local1", "from local1");
    jjrfg_PlainGit.jjrfr_glean(local1.path());

    let result = jjrfg_PlainGit.jjrfr_advance(local1.path(), &format!("{}/{}", "origin", ZJJTFG_TRUNK));

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::Diverged);
}

#[test]
fn jjtfg_consign_plain_pushes_a_fast_forward_commit() {
    let bare = JjkTestDir::new("jjtfg_consign_plain_bare");
    let (local1, _local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_consign_plain");
    let tip = zjjtfg_commit_all(local1.path(), "new.txt", "new", "new");

    jjrfg_PlainGit.jjrfr_consign(local1.path(), ZJJTFG_TRUNK, None).unwrap();

    let remote_tip = zjjtfg_git(bare.path(), &["rev-parse", ZJJTFG_TRUNK]);
    assert_eq!(remote_tip, tip);
}

#[test]
fn jjtfg_consign_rejects_diverged_without_lease() {
    let bare = JjkTestDir::new("jjtfg_consign_diverged_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_consign_diverged");
    zjjtfg_commit_all(local2.path(), "from-local2.txt", "from local2", "from local2");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);
    zjjtfg_commit_all(local1.path(), "from-local1.txt", "from local1", "from local1");

    let result = jjrfg_PlainGit.jjrfr_consign(local1.path(), ZJJTFG_TRUNK, None);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::Diverged);
}

#[test]
fn jjtfg_consign_atomic_lease_succeeds_when_expected_matches() {
    let bare = JjkTestDir::new("jjtfg_consign_lease_ok_bare");
    let (local1, _local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_consign_lease_ok");
    let expected = zjjtfg_git(local1.path(), &["rev-parse", &format!("{}/{}", "origin", ZJJTFG_TRUNK)]);
    let tip = zjjtfg_commit_all(local1.path(), "new.txt", "new", "new");

    jjrfg_PlainGit
        .jjrfr_consign(local1.path(), ZJJTFG_TRUNK, Some(&jjrfr_ConsignLease(expected)))
        .unwrap();

    let remote_tip = zjjtfg_git(bare.path(), &["rev-parse", ZJJTFG_TRUNK]);
    assert_eq!(remote_tip, tip);
}

#[test]
fn jjtfg_consign_atomic_lease_rejects_when_stale() {
    let bare = JjkTestDir::new("jjtfg_consign_lease_stale_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_consign_lease_stale");
    let stale_expected = zjjtfg_git(local1.path(), &["rev-parse", &format!("{}/{}", "origin", ZJJTFG_TRUNK)]);

    zjjtfg_commit_all(local2.path(), "from-local2.txt", "from local2", "from local2");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);

    zjjtfg_commit_all(local1.path(), "new.txt", "new", "new");

    let result = jjrfg_PlainGit.jjrfr_consign(
        local1.path(),
        ZJJTFG_TRUNK,
        Some(&jjrfr_ConsignLease(stale_expected)),
    );

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::Diverged);
}
