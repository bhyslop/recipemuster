// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfg_plaingit::{
    jjrfg_PlainGit,
    zjjrfg_canonicalize_upstream,
    zjjrfg_push_rejected,
    zjjrfg_resolve_relative,
};
use super::jjrfr_farrier::{
    jjrfr_break,
    jjrfr_ConsignLease,
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_GleanOutcome,
    jjrfr_LineOfWork,
    jjrfr_LockGuard,
    jjrfr_RejectionKind,
    jjrfr_Seat,
    jjrfr_SyncState,
};
use super::jjtu_testdir::JjkTestDir;
use std::path::{
    Path,
    PathBuf,
};

const ZJJTFG_TRUNK: &str = "jjtfg-trunk";

/// The remote-tracking name of the test trunk, derived from the trunk const.
fn zjjtfg_remote_trunk() -> String {
    format!("origin/{}", ZJJTFG_TRUNK)
}

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
    assert_eq!(identity.upstream_key, None);
}

#[test]
fn jjtfg_identify_canonicalizes_the_upstream_key() {
    let td = JjkTestDir::new("jjtfg_identify_canonicalizes_the_upstream_key");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");
    zjjtfg_git(td.path(), &["remote", "add", "origin", "/somewhere/upstream.git"]);

    let identity = jjrfg_PlainGit.jjrfr_identify(td.path()).unwrap();

    assert_eq!(identity.upstream_key, Some("/somewhere/upstream".to_string()));
}

#[test]
fn jjtfg_identify_partition_seat_carries_primary_root() {
    let primary = JjkTestDir::new("jjtfg_identify_partition_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let partition = JjkTestDir::new("jjtfg_identify_partition_worktree");
    // The guard pre-creates its directory, but a worktree seat wants to create its own
    std::fs::remove_dir_all(partition.path()).unwrap();
    zjjtfg_git(
        primary.path(),
        &["worktree", "add", "-q", "-b", "jjtfg-partition", &partition.path().to_string_lossy()],
    );

    let identity = jjrfg_PlainGit.jjrfr_identify(partition.path()).unwrap();

    match &identity.seat {
        jjrfr_Seat::Partition { primary_root } => {
            assert_eq!(
                primary_root.canonicalize().unwrap(),
                primary.path().canonicalize().unwrap(),
            );
        }
        other => panic!("expected a partition seat, got {:?}", other),
    }
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Branch("jjtfg-partition".to_string()));
}

#[test]
fn jjtfg_identify_detached_reports_position_faithfully() {
    let td = JjkTestDir::new("jjtfg_identify_detached_reports_position_faithfully");
    zjjtfg_init_local(td.path());
    let sha = zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");
    zjjtfg_git(td.path(), &["checkout", "-q", "--detach"]);

    let identity = jjrfg_PlainGit.jjrfr_identify(td.path()).unwrap();

    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Detached(sha));
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
    jjrfg_PlainGit.jjrfr_advance(local1.path()).unwrap();

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

    let result = jjrfg_PlainGit.jjrfr_advance(local1.path());

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

    let result = jjrfg_PlainGit.jjrfr_advance(local1.path());

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
    let expected = zjjtfg_git(local1.path(), &["rev-parse", &zjjtfg_remote_trunk()]);
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
    let stale_expected = zjjtfg_git(local1.path(), &["rev-parse", &zjjtfg_remote_trunk()]);

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

#[test]
fn jjtfg_canonicalize_upstream_strips_suffix_and_whitespace() {
    assert_eq!(zjjrfg_canonicalize_upstream("/somewhere/upstream.git\n"), "/somewhere/upstream");
    assert_eq!(zjjrfg_canonicalize_upstream("  /plain/path  "), "/plain/path");
    assert_eq!(zjjrfg_canonicalize_upstream("git@host:org/repo.git"), "git@host:org/repo");
}

#[test]
fn jjtfg_push_rejected_matches_transport_vocabulary_only() {
    assert!(zjjrfg_push_rejected("! [rejected] trunk -> trunk (fetch first)"));
    assert!(zjjrfg_push_rejected("! [remote rejected] trunk -> trunk (stale info)"));
    assert!(zjjrfg_push_rejected("Updates were rejected because a pushed branch tip is non-fast-forward"));
    assert!(!zjjrfg_push_rejected("fatal: Could not read from remote repository."));
}

#[test]
fn jjtfg_resolve_relative_joins_only_relative_paths() {
    let base = Path::new("/base/repo");
    assert_eq!(zjjrfg_resolve_relative(base, "/already/absolute"), PathBuf::from("/already/absolute"));
    assert_eq!(zjjrfg_resolve_relative(base, ".git"), PathBuf::from("/base/repo/.git"));
}

/// Sets up a bare remote plus one local clone tracking it, with a baseline commit
/// pushed — the common precondition for lock-facet tests, which need only one
/// station talking to the shared remote (no divergence).
fn zjjtfg_local_with_remote(name: &str) -> (JjkTestDir, JjkTestDir) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtfg_init_bare(bare.path());
    let local = JjkTestDir::new(&format!("{}_local", name));
    zjjtfg_init_local(local.path());
    zjjtfg_commit_all(local.path(), "base.txt", "base", "init");
    zjjtfg_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtfg_git(local.path(), &["push", "-q", "-u", "origin", ZJJTFG_TRUNK]);
    (bare, local)
}

/// Creates a fresh, not-yet-existing directory under the OS temp root — the shape
/// `jjrfr_billet_create` requires (it seats the worktree itself). Reuses
/// `JjkTestDir`'s naming and eventual RAII cleanup; the guard pre-creates its
/// directory, so it is removed immediately, mirroring the partition-seat identify
/// test's own setup.
fn zjjtfg_billet_slot(name: &str) -> JjkTestDir {
    let slot = JjkTestDir::new(name);
    std::fs::remove_dir_all(slot.path()).unwrap();
    slot
}

#[test]
fn jjtfg_stake_creates_the_guidon_ref_and_sight_reads_it_back() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_stake_create");

    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-alpha").unwrap();

    let sighted = jjrfg_PlainGit.jjrfr_sight(local.path()).unwrap();
    assert_eq!(sighted, Some("guidon-alpha".to_string()));
}

#[test]
fn jjtfg_sight_is_none_when_unlocked() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_sight_unlocked");

    let sighted = jjrfg_PlainGit.jjrfr_sight(local.path()).unwrap();

    assert_eq!(sighted, None);
}

#[test]
fn jjtfg_stake_rejects_lock_held_when_already_staked() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_stake_held");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-first").unwrap();

    let result = jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-second");

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockHeld);
}

#[test]
fn jjtfg_pluck_releases_on_matching_observed_guidon() {
    let (bare, local) = zjjtfg_local_with_remote("jjtfg_pluck_ok");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-release-me").unwrap();

    jjrfg_PlainGit.jjrfr_pluck(local.path(), "guidon-release-me").unwrap();

    let remaining = zjjtfg_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.is_empty());
}

#[test]
fn jjtfg_pluck_rejects_lock_broken_on_stale_observed_guidon() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_pluck_broken");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-actual").unwrap();

    let result = jjrfg_PlainGit.jjrfr_pluck(local.path(), "guidon-not-what-is-there");

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockBroken);
}

#[test]
fn jjtfg_lock_guard_acquires_and_releases_on_drop() {
    let (bare, local) = zjjtfg_local_with_remote("jjtfg_guard_release");

    {
        let guard = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local.path(), "guidon-guarded").unwrap();
        assert_eq!(guard.jjrfr_guidon(), "guidon-guarded");
        let sighted = jjrfg_PlainGit.jjrfr_sight(local.path()).unwrap();
        assert_eq!(sighted, Some("guidon-guarded".to_string()));
    }

    let remaining = zjjtfg_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.is_empty());
}

#[test]
#[should_panic(expected = "nested lock acquire")]
fn jjtfg_lock_guard_nested_acquire_panics() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_guard_nested");

    let _first = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local.path(), "guidon-first").unwrap();
    let _second = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local.path(), "guidon-second");
}

#[test]
fn jjtfg_lock_guard_reacquire_after_drop_succeeds() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_guard_reacquire");

    {
        let _first = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local.path(), "guidon-first").unwrap();
    }
    let second = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local.path(), "guidon-second");

    assert!(second.is_ok());
}

#[test]
fn jjtfg_break_clears_a_staked_lock_and_reports_the_observed_guidon() {
    let (bare, local) = zjjtfg_local_with_remote("jjtfg_break_clears");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-stale").unwrap();

    let cleared = jjrfr_break(&jjrfg_PlainGit, local.path()).unwrap();

    assert_eq!(cleared, Some("guidon-stale".to_string()));
    let remaining = zjjtfg_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.is_empty());
}

#[test]
fn jjtfg_break_is_none_when_nothing_staked() {
    let (_bare, local) = zjjtfg_local_with_remote("jjtfg_break_nothing");

    let cleared = jjrfr_break(&jjrfg_PlainGit, local.path()).unwrap();

    assert_eq!(cleared, None);
}

#[test]
fn jjtfg_sight_reads_a_guidon_staked_by_another_station() {
    let bare = JjkTestDir::new("jjtfg_sight_cross_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_sight_cross");

    jjrfg_PlainGit.jjrfr_stake(local1.path(), "guidon-station-one").unwrap();

    // The sighting station has never held the guidon blob locally — sight's
    // fetch must actually transfer it.
    let sighted = jjrfg_PlainGit.jjrfr_sight(local2.path()).unwrap();
    assert_eq!(sighted, Some("guidon-station-one".to_string()));
}

#[test]
fn jjtfg_break_clears_another_stations_lock() {
    let bare = JjkTestDir::new("jjtfg_break_cross_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_break_cross");
    jjrfg_PlainGit.jjrfr_stake(local1.path(), "guidon-abandoned").unwrap();

    let cleared = jjrfr_break(&jjrfg_PlainGit, local2.path()).unwrap();

    assert_eq!(cleared, Some("guidon-abandoned".to_string()));
    let remaining = zjjtfg_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.is_empty());
}

#[test]
fn jjtfg_lock_guard_drop_survives_an_unreachable_remote() {
    let (bare, local) = zjjtfg_local_with_remote("jjtfg_guard_offline");
    let guard = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local.path(), "guidon-marooned").unwrap();
    zjjtfg_git(local.path(), &["remote", "set-url", "origin", "/nonexistent/jjtfg-nowhere"]);

    // Best-effort release: the pluck's unclassified failure must not escape
    // the destructor.
    drop(guard);

    // The lock still flies on the remote — a stale lock for the break, not a
    // crash.
    let remaining = zjjtfg_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.contains("refs/jjv/guidon"));
}

#[test]
fn jjtfg_lock_guard_rejected_acquire_leaves_no_registry_residue() {
    let bare = JjkTestDir::new("jjtfg_guard_residue_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_guard_residue");
    let first = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local1.path(), "guidon-holder").unwrap();

    // Contention from another clone of the same blotter is a rejection, never
    // the nested-acquire panic — the registry keys on the local root.
    let kind = match jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local2.path(), "guidon-contender") {
        Err(rejection) => rejection.kind,
        Ok(_) => panic!("a stake over a held lock must reject"),
    };
    assert_eq!(kind, jjrfr_RejectionKind::LockHeld);

    drop(first);

    let second = jjrfr_LockGuard::jjrfr_acquire(&jjrfg_PlainGit, local2.path(), "guidon-second").unwrap();
    assert_eq!(second.jjrfr_guidon(), "guidon-second");
}

#[test]
fn jjtfg_billet_create_seats_a_new_branch_worktree() {
    let primary = JjkTestDir::new("jjtfg_billet_create_new_branch_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_new_branch_billet");

    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("billet-branch".to_string()), billet.path())
        .unwrap();

    let identity = jjrfg_PlainGit.jjrfr_identify(billet.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Branch("billet-branch".to_string()));
    match identity.seat {
        jjrfr_Seat::Partition { .. } => {}
        other => panic!("expected a partition seat, got {:?}", other),
    }
}

#[test]
#[should_panic(expected = "unclassified git failure")]
fn jjtfg_billet_create_has_one_canonical_form_and_fails_loud_on_a_name_collision() {
    let primary = JjkTestDir::new("jjtfg_billet_create_collision_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    zjjtfg_git(primary.path(), &["branch", "preexisting"]);
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_collision_billet");

    let _ = jjrfg_PlainGit.jjrfr_billet_create(
        primary.path(),
        &jjrfr_LineOfWork::Branch("preexisting".to_string()),
        billet.path(),
    );
}

#[test]
fn jjtfg_billet_create_seats_detached_at_a_position() {
    let primary = JjkTestDir::new("jjtfg_billet_create_detached_primary");
    zjjtfg_init_local(primary.path());
    let sha = zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_detached_billet");

    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Detached(sha.clone()), billet.path())
        .unwrap();

    let identity = jjrfg_PlainGit.jjrfr_identify(billet.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Detached(sha));
}

#[test]
fn jjtfg_billet_remove_reaps_a_clean_billet() {
    let primary = JjkTestDir::new("jjtfg_billet_remove_clean_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_billet_remove_clean_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("removable".to_string()), billet.path())
        .unwrap();

    jjrfg_PlainGit.jjrfr_billet_remove(billet.path()).unwrap();

    assert!(!billet.path().exists());
    let worktrees = zjjtfg_git(primary.path(), &["worktree", "list"]);
    assert!(!worktrees.contains("removable"));
}

#[test]
fn jjtfg_billet_remove_rejects_dirty_tree() {
    let primary = JjkTestDir::new("jjtfg_billet_remove_dirty_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_billet_remove_dirty_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("dirty-billet".to_string()), billet.path())
        .unwrap();
    zjjtfg_write(billet.path(), "dirt.txt", "uncommitted");

    let result = jjrfg_PlainGit.jjrfr_billet_remove(billet.path());

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}

#[test]
fn jjtfg_enfold_fast_forwards_when_billet_has_no_local_commits() {
    let primary = JjkTestDir::new("jjtfg_enfold_ff_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_enfold_ff_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("ff-billet".to_string()), billet.path())
        .unwrap();
    let tip = zjjtfg_commit_all(primary.path(), "b.txt", "trunk moved on", "trunk advances");

    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();

    let head = zjjtfg_git(billet.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, tip);
}

#[test]
fn jjtfg_enfold_merges_the_named_trunk_not_the_primarys_checkout() {
    let primary = JjkTestDir::new("jjtfg_enfold_named_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_enfold_named_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("named-trunk-billet".to_string()), billet.path())
        .unwrap();
    zjjtfg_commit_all(primary.path(), "b.txt", "trunk work", "trunk advances");
    // Park the primary on a different line — trunk-ness must come from the
    // caller, never from the primary's ambient checkout.
    zjjtfg_git(primary.path(), &["checkout", "-q", "-b", "sidetrack"]);
    zjjtfg_commit_all(primary.path(), "c.txt", "side work", "side commit");

    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();

    assert!(billet.path().join("b.txt").exists());
    assert!(!billet.path().join("c.txt").exists());
}

#[test]
fn jjtfg_enfold_merges_divergent_trunk_and_billet_history() {
    let primary = JjkTestDir::new("jjtfg_enfold_merge_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_enfold_merge_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("merge-billet".to_string()), billet.path())
        .unwrap();
    zjjtfg_commit_all(billet.path(), "billet.txt", "billet work", "billet commit");
    zjjtfg_commit_all(primary.path(), "trunk.txt", "trunk work", "trunk commit");

    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();

    assert!(billet.path().join("trunk.txt").exists());
    assert!(billet.path().join("billet.txt").exists());
    let log = zjjtfg_git(billet.path(), &["log", "--oneline", "-1"]);
    assert!(log.contains("enfold trunk"));
}

#[test]
fn jjtfg_enfold_rejects_dirty_tree() {
    let primary = JjkTestDir::new("jjtfg_enfold_dirty_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_enfold_dirty_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("dirty-enfold-billet".to_string()), billet.path())
        .unwrap();
    zjjtfg_commit_all(primary.path(), "b.txt", "trunk moved on", "trunk advances");
    zjjtfg_write(billet.path(), "dirt.txt", "uncommitted");

    let result = jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}

#[test]
#[should_panic(expected = "unclassified git failure")]
fn jjtfg_enfold_fails_loud_on_conflict() {
    let primary = JjkTestDir::new("jjtfg_enfold_conflict_primary");
    zjjtfg_init_local(primary.path());
    zjjtfg_commit_all(primary.path(), "a.txt", "hello", "init");
    let billet = zjjtfg_billet_slot("jjtfg_enfold_conflict_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_LineOfWork::Branch("conflict-billet".to_string()), billet.path())
        .unwrap();
    zjjtfg_commit_all(billet.path(), "a.txt", "billet changed this line", "billet edits a.txt");
    zjjtfg_commit_all(primary.path(), "a.txt", "trunk changed this line too", "trunk edits a.txt");

    let _ = jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK);
}
