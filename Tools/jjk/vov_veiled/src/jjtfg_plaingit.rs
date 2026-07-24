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
    jjrfr_BilletBirth,
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

/// The inverse of `zjjtfg_git`: run a command expected to fail and return what git
/// said about it. The seat signatures' indistinguishability is asserted against this.
fn zjjtfg_git_failure(dir: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("test harness git invocation must spawn");
    assert!(!out.status.success(), "test harness git -C {} {:?} was expected to fail", dir.display(), args);
    format!("exit {:?} | {}", out.status.code(), String::from_utf8_lossy(&out.stderr).trim())
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
fn jjtfg_advance_fast_forwards_a_behind_line_to_the_remote_tip() {
    let bare = JjkTestDir::new("jjtfg_advance_behind_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_behind");

    let tip = zjjtfg_commit_all(local2.path(), "ahead.txt", "ahead", "ahead");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);

    jjrfg_PlainGit.jjrfr_glean(local1.path());
    jjrfg_PlainGit.jjrfr_advance(local1.path()).unwrap();

    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, tip);
}

/// JJr_b52: a line ahead of the tip is an impossible state under
/// compose-then-push — the local branch only ever moves to positions the
/// remote accepted — so advance halts and surfaces (`Diverged`) rather than
/// auto-destroying what it cannot explain. The position is left untouched.
/// This is the LOCAL-AHEAD verdict: the refusal names the stranded commit
/// and the push remedy, distinct from a true fork.
#[test]
fn jjtfg_advance_rejects_diverged_on_a_line_ahead_of_the_remote_tip() {
    let bare = JjkTestDir::new("jjtfg_advance_ahead_bare");
    let (local1, _local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_ahead");

    let ahead = zjjtfg_commit_all(local1.path(), "unexplained.txt", "not the remote's", "unexplained");
    jjrfg_PlainGit.jjrfr_glean(local1.path());
    let result = jjrfg_PlainGit.jjrfr_advance(local1.path());

    let rejection = result.unwrap_err();
    assert_eq!(rejection.kind, jjrfr_RejectionKind::Diverged);
    assert!(rejection.detail.contains("LOCAL-AHEAD"), "detail: {}", rejection.detail);
    assert!(rejection.detail.contains("unexplained"), "stranded commit must be named: {}", rejection.detail);
    assert!(rejection.detail.contains("consign"), "the push remedy must be named: {}", rejection.detail);
    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, ahead, "advance must leave the unexplained position untouched — never auto-destroy");
    assert!(local1.path().join("unexplained.txt").exists(), "its content must stand for the operator to inspect");
}

/// JJr_b52: unrelated dirt does not block the fast-forward and is not
/// destroyed by it — the working tree is writer-only scratch, and the move
/// touches only the paths the tip changed.
#[test]
fn jjtfg_advance_fast_forwards_past_unrelated_dirt() {
    let bare = JjkTestDir::new("jjtfg_advance_dirty_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_dirty");
    let tip = zjjtfg_commit_all(local2.path(), "ahead.txt", "ahead", "ahead");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);
    jjrfg_PlainGit.jjrfr_glean(local1.path());
    zjjtfg_write(local1.path(), "scratch.txt", "half-written by a ceremony that died");

    jjrfg_PlainGit.jjrfr_advance(local1.path()).unwrap();

    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, tip);
    let scratch = std::fs::read_to_string(local1.path().join("scratch.txt")).unwrap();
    assert_eq!(scratch, "half-written by a ceremony that died", "unrelated scratch survives the move untouched");
}

/// JJr_b52: a genuinely diverged line — ahead AND behind — is the same
/// impossible state as merely-ahead: halt and surface, position untouched.
/// This is the FORKED verdict, distinguished in the refusal from LOCAL-AHEAD.
#[test]
fn jjtfg_advance_rejects_diverged_on_a_forked_line() {
    let bare = JjkTestDir::new("jjtfg_advance_diverged_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_advance_diverged");

    zjjtfg_commit_all(local2.path(), "from-local2.txt", "from local2", "from local2");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);

    let forked = zjjtfg_commit_all(local1.path(), "from-local1.txt", "from local1", "from local1");
    jjrfg_PlainGit.jjrfr_glean(local1.path());

    let result = jjrfg_PlainGit.jjrfr_advance(local1.path());

    let rejection = result.unwrap_err();
    assert_eq!(rejection.kind, jjrfr_RejectionKind::Diverged);
    assert!(rejection.detail.contains("FORKED"), "detail: {}", rejection.detail);
    assert!(!rejection.detail.contains("LOCAL-AHEAD"), "a fork must not read as local-ahead: {}", rejection.detail);
    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, forked, "the forked position must stand for the operator to inspect");
    assert!(local1.path().join("from-local1.txt").exists());
}

#[test]
fn jjtfg_consign_plain_pushes_a_fast_forward_commit() {
    let bare = JjkTestDir::new("jjtfg_consign_plain_bare");
    let (local1, _local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_consign_plain");
    let tip = zjjtfg_commit_all(local1.path(), "new.txt", "new", "new");

    jjrfg_PlainGit.jjrfr_consign(local1.path(), ZJJTFG_TRUNK).unwrap();

    let remote_tip = zjjtfg_git(bare.path(), &["rev-parse", ZJJTFG_TRUNK]);
    assert_eq!(remote_tip, tip);
}

#[test]
fn jjtfg_consign_rejects_diverged_on_a_content_race() {
    let bare = JjkTestDir::new("jjtfg_consign_diverged_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_consign_diverged");
    zjjtfg_commit_all(local2.path(), "from-local2.txt", "from local2", "from local2");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);
    zjjtfg_commit_all(local1.path(), "from-local1.txt", "from local1", "from local1");

    let result = jjrfg_PlainGit.jjrfr_consign(local1.path(), ZJJTFG_TRUNK);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::Diverged);
}

/// Proffer's happy path: the composed write lands on the remote AND the local
/// branch adopts it — while the held lock stands exactly as it stood.
#[test]
fn jjtfg_proffer_lands_the_composed_write_while_lock_held() {
    let bare = JjkTestDir::new("jjtfg_proffer_ok_bare");
    let (local1, _local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_proffer_ok");
    jjrfg_PlainGit.jjrfr_stake(local1.path(), "guidon-holder").unwrap();
    zjjtfg_write(local1.path(), "new.txt", "new");

    let sha = jjrfg_PlainGit
        .jjrfr_proffer(
            local1.path(),
            ZJJTFG_TRUNK,
            &[PathBuf::from("new.txt")],
            "proffered write",
            &jjrfr_ConsignLease("guidon-holder".to_string()),
        )
        .unwrap();

    let remote_tip = zjjtfg_git(bare.path(), &["rev-parse", ZJJTFG_TRUNK]);
    assert_eq!(remote_tip, sha, "the accepted position must be live on the remote");
    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, sha, "the local branch must adopt the accepted position");
    let flying = jjrfg_PlainGit.jjrfr_sight(local1.path()).unwrap();
    assert_eq!(flying.as_deref(), Some("guidon-holder"), "proffer must leave the held lock exactly as it stood");
}

/// The refused-consign proof (JJr_b52, compose-then-push): a lock broken under
/// the holder fails the whole push AND leaves the local branch and its record
/// untouched — no refused commit rides the branch, nothing exists to scrub.
#[test]
fn jjtfg_proffer_rejects_lock_broken_leaving_branch_and_remote_untouched() {
    let bare = JjkTestDir::new("jjtfg_proffer_broken_bare");
    let (local1, _local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_proffer_broken");
    let baseline = zjjtfg_git(bare.path(), &["rev-parse", ZJJTFG_TRUNK]);
    let local_baseline = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    jjrfg_PlainGit.jjrfr_stake(local1.path(), "guidon-holder").unwrap();
    zjjtfg_write(local1.path(), "new.txt", "new");

    // The lock breaks under the holder (plucked, not re-staked) between its
    // sight and its proffer — the atomic lease must fail the content push too.
    jjrfg_PlainGit.jjrfr_pluck(local1.path(), "guidon-holder").unwrap();

    let result = jjrfg_PlainGit.jjrfr_proffer(
        local1.path(),
        ZJJTFG_TRUNK,
        &[PathBuf::from("new.txt")],
        "must never land",
        &jjrfr_ConsignLease("guidon-holder".to_string()),
    );

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockBroken);
    let remote_tip = zjjtfg_git(bare.path(), &["rev-parse", ZJJTFG_TRUNK]);
    assert_eq!(remote_tip, baseline, "a broken-lock proffer must land nothing on the remote");
    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, local_baseline, "a refused proffer must leave the local branch untouched");
    let on_branch = zjjtfg_git(local1.path(), &["log", "--pretty=%s", "HEAD"]);
    assert!(!on_branch.contains("must never land"), "the refused commit must not ride the branch");
}

#[test]
fn jjtfg_proffer_rejects_content_race_as_diverged_leaving_branch_untouched() {
    let bare = JjkTestDir::new("jjtfg_proffer_race_bare");
    let (local1, local2) = zjjtfg_two_clones_from_baseline(bare.path(), "jjtfg_proffer_race");
    jjrfg_PlainGit.jjrfr_stake(local1.path(), "guidon-holder").unwrap();
    let local_baseline = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);

    // The lock stands untouched, but content lands from elsewhere after our
    // glean — the rejection classifies on the branch, never the guidon.
    zjjtfg_commit_all(local2.path(), "from-local2.txt", "from local2", "from local2");
    zjjtfg_git(local2.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);
    zjjtfg_write(local1.path(), "new.txt", "new");

    let result = jjrfg_PlainGit.jjrfr_proffer(
        local1.path(),
        ZJJTFG_TRUNK,
        &[PathBuf::from("new.txt")],
        "raced write",
        &jjrfr_ConsignLease("guidon-holder".to_string()),
    );

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::Diverged);
    let flying = jjrfg_PlainGit.jjrfr_sight(local1.path()).unwrap();
    assert_eq!(flying.as_deref(), Some("guidon-holder"), "a content-race rejection must leave the held lock standing");
    let head = zjjtfg_git(local1.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, local_baseline, "a refused proffer must leave the local branch untouched");
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
fn jjtfg_billet_create_seats_a_new_branch_worktree_at_the_counterpart() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_create_new_branch");
    let baseline = zjjtfg_git(primary.path(), &["rev-parse", "HEAD"]);
    // The operator's local trunk moves on without a push: birth must anchor at
    // the counterpart, never at this unpublished tip (no-exfiltration at birth).
    zjjtfg_commit_all(primary.path(), "unpushed.txt", "local only", "unpushed trunk work");
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_new_branch_billet");

    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("billet-branch".to_string()), billet.path(), ZJJTFG_TRUNK)
        .unwrap();

    let identity = jjrfg_PlainGit.jjrfr_identify(billet.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Branch("billet-branch".to_string()));
    match identity.seat {
        jjrfr_Seat::Partition { .. } => {}
        other => panic!("expected a partition seat, got {:?}", other),
    }
    let billet_head = zjjtfg_git(billet.path(), &["rev-parse", "HEAD"]);
    assert_eq!(billet_head, baseline, "birth must anchor at trunk's counterpart, not the primary's unpushed tip");
}

#[test]
fn jjtfg_billet_create_self_tracks_before_and_after_the_first_consign() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_create_self_tracks");
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_self_tracks_billet");

    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("billet-branch".to_string()), billet.path(), ZJJTFG_TRUNK)
        .unwrap();

    // Before the first consign, the branch's own counterpart does not exist
    // yet: `@{upstream}` fails to resolve and `sync_state` answers Untracked —
    // the litmus's ignorance-stands arm, never a false Tracking-against-trunk.
    assert_eq!(jjrfg_PlainGit.jjrfr_sync_state(billet.path()).unwrap(), jjrfr_SyncState::Untracked);

    jjrfg_PlainGit.jjrfr_consign(billet.path(), "billet-branch").unwrap();

    // After the first consign the own counterpart exists and tracking is live:
    // this is the SAME line, not trunk's — a plain further commit reads ahead
    // of the billet's OWN counterpart, never trunk's.
    assert_eq!(
        zjjtfg_git(billet.path(), &["rev-parse", "--abbrev-ref", "@{upstream}"]),
        "origin/billet-branch"
    );
    assert_eq!(
        jjrfg_PlainGit.jjrfr_sync_state(billet.path()).unwrap(),
        jjrfr_SyncState::Tracking { ahead: 0, behind: 0 }
    );
}

#[test]
#[should_panic(expected = "unclassified git failure")]
fn jjtfg_billet_create_has_one_canonical_form_and_fails_loud_on_a_name_collision() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_create_collision");
    zjjtfg_git(primary.path(), &["branch", "preexisting"]);
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_collision_billet");

    let _ = jjrfg_PlainGit.jjrfr_billet_create(
        primary.path(),
        &jjrfr_BilletBirth::Branch("preexisting".to_string()),
        billet.path(),
        ZJJTFG_TRUNK,
    );
}

#[test]
fn jjtfg_billet_create_seats_detached_at_the_counterpart() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_create_detached");
    let baseline = zjjtfg_git(primary.path(), &["rev-parse", "HEAD"]);
    zjjtfg_commit_all(primary.path(), "unpushed.txt", "local only", "unpushed trunk work");
    let billet = zjjtfg_billet_slot("jjtfg_billet_create_detached_billet");

    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Detached, billet.path(), ZJJTFG_TRUNK)
        .unwrap();

    let identity = jjrfg_PlainGit.jjrfr_identify(billet.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Detached(baseline));
}

#[test]
fn jjtfg_billet_seat_reseats_a_durable_branch_with_its_history() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_seat");
    let first = zjjtfg_billet_slot("jjtfg_billet_seat_first");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("durable".to_string()), first.path(), ZJJTFG_TRUNK)
        .unwrap();
    let wip = zjjtfg_commit_all(first.path(), "wip.txt", "carried work", "wip on the durable branch");
    jjrfg_PlainGit.jjrfr_billet_remove(first.path()).unwrap();

    let second = zjjtfg_billet_slot("jjtfg_billet_seat_second");
    jjrfg_PlainGit.jjrfr_billet_seat(primary.path(), "durable", second.path()).unwrap();

    let head = zjjtfg_git(second.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, wip, "re-seating must carry the branch's WIP history, not re-anchor it");
    let identity = jjrfg_PlainGit.jjrfr_identify(second.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Branch("durable".to_string()));
}

/// The incident this survey was cut from: a billet removed out of band leaves the
/// registration standing, and the next seat of its branch dies on the residue.
#[test]
fn jjtfg_billet_seat_rejects_seat_vestige_when_the_recorded_root_is_gone() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_seat_vestige");
    let first = zjjtfg_billet_slot("jjtfg_seat_vestige_first");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("durable".to_string()), first.path(), ZJJTFG_TRUNK)
        .unwrap();
    // The out-of-band removal: the root goes, the registration stays.
    std::fs::remove_dir_all(first.path()).unwrap();

    let second = zjjtfg_billet_slot("jjtfg_seat_vestige_second");
    let rejection = jjrfg_PlainGit
        .jjrfr_billet_seat(primary.path(), "durable", second.path())
        .expect_err("a vestige registration must reject, never panic");

    assert_eq!(rejection.kind, jjrfr_RejectionKind::SeatVestige);
    assert!(rejection.detail.contains("worktree prune"), "the refusal must name the remedy: {}", rejection.detail);

    // Advice, never automatic: the driver leaves the registry exactly as it found it.
    let registry = zjjtfg_git(primary.path(), &["worktree", "list", "--porcelain"]);
    assert!(registry.contains("prunable"), "the driver must never prune on the operator's behalf");
}

/// The exclusivity a live billet holds over its branch — the mechanism the
/// at-most-one-live-billet-per-coronet invariant leans on, surfaced as a kind.
#[test]
fn jjtfg_billet_seat_rejects_line_seated_when_a_live_billet_holds_the_branch() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_line_seated");
    let live = zjjtfg_billet_slot("jjtfg_line_seated_live");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("durable".to_string()), live.path(), ZJJTFG_TRUNK)
        .unwrap();

    let second = zjjtfg_billet_slot("jjtfg_line_seated_second");
    let rejection = jjrfg_PlainGit
        .jjrfr_billet_seat(primary.path(), "durable", second.path())
        .expect_err("a branch seated in a live billet must reject, never panic");

    assert_eq!(rejection.kind, jjrfr_RejectionKind::LineSeated);
    assert!(
        rejection.detail.contains(&live.path().to_string_lossy().into_owned()),
        "the refusal must name the standing billet: {}",
        rejection.detail
    );
}

/// The two kinds are told apart by the registry alone: git renders both refusals
/// identically, so a classification reading message text would collapse them.
#[test]
fn jjtfg_billet_seat_signatures_are_indistinguishable_in_gits_own_words() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_seat_same_words");
    let live = zjjtfg_billet_slot("jjtfg_seat_same_words_live");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("durable".to_string()), live.path(), ZJJTFG_TRUNK)
        .unwrap();
    let seated_refusal = zjjtfg_git_failure(primary.path(), &["worktree", "add", "-q", "/nonexistent-slot-a", "durable"]);

    std::fs::remove_dir_all(live.path()).unwrap();
    let vestige_refusal = zjjtfg_git_failure(primary.path(), &["worktree", "add", "-q", "/nonexistent-slot-b", "durable"]);

    assert_eq!(
        seated_refusal, vestige_refusal,
        "if git ever distinguishes these, the registry probe may be revisited — until then it is the sole discriminator"
    );
}

/// A refusal the registry cannot explain keeps the panic: classifying it would mean
/// reading the message text the farrier sheaf bars.
#[test]
#[should_panic(expected = "unclassified git failure")]
fn jjtfg_billet_seat_panics_when_the_registry_records_no_seat() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_seat_unrecorded");
    let occupied = JjkTestDir::new("jjtfg_seat_unrecorded_occupied");
    std::fs::write(occupied.path().join("squatter.txt"), "not a billet").unwrap();
    zjjtfg_git(primary.path(), &["branch", "durable"]);

    // The branch is seated nowhere; the add fails on the occupied destination.
    let _ = jjrfg_PlainGit.jjrfr_billet_seat(primary.path(), "durable", occupied.path());
}

#[test]
fn jjtfg_billet_detach_moves_a_groom_billet_to_the_counterpart() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_detach");
    let billet = zjjtfg_billet_slot("jjtfg_billet_detach_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Detached, billet.path(), ZJJTFG_TRUNK)
        .unwrap();
    let advanced = zjjtfg_trunk_advances(primary.path(), "b.txt", "moved", "trunk advances");
    let _ = jjrfg_PlainGit.jjrfr_glean(billet.path());

    jjrfg_PlainGit.jjrfr_billet_detach(billet.path(), ZJJTFG_TRUNK).unwrap();

    let identity = jjrfg_PlainGit.jjrfr_identify(billet.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Detached(advanced));
}

#[test]
fn jjtfg_billet_detach_rejects_dirty_tree() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_detach_dirty");
    let billet = zjjtfg_billet_slot("jjtfg_billet_detach_dirty_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Detached, billet.path(), ZJJTFG_TRUNK)
        .unwrap();
    zjjtfg_write(billet.path(), "dirt.txt", "uncommitted");

    let result = jjrfg_PlainGit.jjrfr_billet_detach(billet.path(), ZJJTFG_TRUNK);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}

#[test]
fn jjtfg_line_exists_answers_both_ways() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_line_exists");
    zjjtfg_git(primary.path(), &["branch", "a-real-line"]);

    assert!(jjrfg_PlainGit.jjrfr_line_exists(primary.path(), "a-real-line").unwrap());
    assert!(!jjrfg_PlainGit.jjrfr_line_exists(primary.path(), "no-such-line").unwrap());
}

/// Another station works a pace and pushes its branch: the line stands abroad
/// but not at home, and only a glean can reveal it — the precondition the adopt
/// arm turns on.
fn zjjtfg_line_pushed_by_another_station(bare: &Path, name: &str, branch: &str) -> String {
    let other = JjkTestDir::new(name);
    zjjtfg_git(other.path(), &["clone", "-q", &bare.to_string_lossy(), "."]);
    zjjtfg_git(other.path(), &["config", "user.email", "jjtfg@example.invalid"]);
    zjjtfg_git(other.path(), &["config", "user.name", "jjtfg"]);
    zjjtfg_git(other.path(), &["checkout", "-q", "-b", branch]);
    let tip = zjjtfg_commit_all(other.path(), "abroad.txt", "worked elsewhere", "wip on another station");
    zjjtfg_git(other.path(), &["push", "-q", "origin", branch]);
    tip
}

#[test]
fn jjtfg_line_abroad_answers_both_ways_and_only_after_a_glean() {
    let (bare, primary) = zjjtfg_local_with_remote("jjtfg_line_abroad");
    zjjtfg_line_pushed_by_another_station(bare.path(), "jjtfg_line_abroad_other", "jjls_pace/AAAAA");

    // Network-silent by contract: before the glean this station has never seen
    // the counterpart, so the honest answer is no.
    assert!(!jjrfg_PlainGit.jjrfr_line_abroad(primary.path(), "jjls_pace/AAAAA").unwrap());
    let _ = jjrfg_PlainGit.jjrfr_glean(primary.path());

    assert!(jjrfg_PlainGit.jjrfr_line_abroad(primary.path(), "jjls_pace/AAAAA").unwrap());
    assert!(!jjrfg_PlainGit.jjrfr_line_abroad(primary.path(), "jjls_pace/NOSUCH").unwrap());
    // Abroad is not home: the adopt arm's two observations must disagree here,
    // or the spine could never tell a re-seat from an adoption.
    assert!(!jjrfg_PlainGit.jjrfr_line_exists(primary.path(), "jjls_pace/AAAAA").unwrap());
}

#[test]
fn jjtfg_billet_adopt_seats_the_remote_line_and_consigns_back_to_it() {
    let (bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_adopt");
    let abroad = zjjtfg_line_pushed_by_another_station(bare.path(), "jjtfg_billet_adopt_other", "jjls_pace/AAAAA");
    // The operator's local trunk moves on unpushed: adopting must land on the
    // pushed line, nowhere near this tip.
    zjjtfg_commit_all(primary.path(), "unpushed.txt", "local only", "unpushed trunk work");
    let _ = jjrfg_PlainGit.jjrfr_glean(primary.path());
    let billet = zjjtfg_billet_slot("jjtfg_billet_adopt_billet");

    jjrfg_PlainGit.jjrfr_billet_adopt(primary.path(), "jjls_pace/AAAAA", billet.path()).unwrap();

    let identity = jjrfg_PlainGit.jjrfr_identify(billet.path()).unwrap();
    assert_eq!(identity.line_of_work, jjrfr_LineOfWork::Branch("jjls_pace/AAAAA".to_string()));
    assert_eq!(
        zjjtfg_git(billet.path(), &["rev-parse", "HEAD"]),
        abroad,
        "adopting must take the other station's position, never fork from trunk"
    );
    // The upstream is what makes the adopted line the SAME line: a consign from
    // here lands where the other station is looking.
    assert_eq!(
        zjjtfg_git(billet.path(), &["rev-parse", "--abbrev-ref", "@{upstream}"]),
        "origin/jjls_pace/AAAAA"
    );
}

/// A branch already standing at home is a caller-contract violation here — the
/// spine consults `line_exists` first, and re-seating is `billet_seat`'s work.
#[test]
#[should_panic(expected = "unclassified git failure")]
fn jjtfg_billet_adopt_fails_loud_when_the_line_already_stands_at_home() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_adopt_collision");
    zjjtfg_git(primary.path(), &["branch", "jjls_pace/AAAAA"]);
    let billet = zjjtfg_billet_slot("jjtfg_billet_adopt_collision_billet");

    let _ = jjrfg_PlainGit.jjrfr_billet_adopt(primary.path(), "jjls_pace/AAAAA", billet.path());
}

#[test]
fn jjtfg_outstripped_is_false_while_the_billet_holds_the_counterpart_tip() {
    let (_bare, _primary, billet) = zjjtfg_billeted_with_remote("jjtfg_outstripped_current");

    assert!(!jjrfg_PlainGit.jjrfr_outstripped(billet.path(), ZJJTFG_TRUNK).unwrap());
}

#[test]
fn jjtfg_outstripped_is_true_after_a_glean_reveals_trunk_moved() {
    let (bare, _primary, billet) = zjjtfg_billeted_with_remote("jjtfg_outstripped_moved");

    // Another station publishes trunk: this station's counterpart ref cannot
    // move on its own (a same-station push would update it immediately —
    // worktrees share the primary's ref store), so staleness here is genuinely
    // fetch-revealed: before the glean the billet cannot know; after it, it must.
    let other = JjkTestDir::new("jjtfg_outstripped_moved_other");
    zjjtfg_git(other.path(), &["clone", "-q", &bare.path().to_string_lossy(), "."]);
    zjjtfg_git(other.path(), &["config", "user.email", "jjtfg@example.invalid"]);
    zjjtfg_git(other.path(), &["config", "user.name", "jjtfg"]);
    zjjtfg_commit_all(other.path(), "b.txt", "moved elsewhere", "trunk advances on another station");
    zjjtfg_git(other.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);

    assert!(!jjrfg_PlainGit.jjrfr_outstripped(billet.path(), ZJJTFG_TRUNK).unwrap());
    let _ = jjrfg_PlainGit.jjrfr_glean(billet.path());

    assert!(jjrfg_PlainGit.jjrfr_outstripped(billet.path(), ZJJTFG_TRUNK).unwrap());
}

#[test]
fn jjtfg_outstripped_is_false_when_no_counterpart_is_known() {
    // A remote-less repo has no counterpart to be behind — the probe must not
    // cry on ignorance.
    let td = JjkTestDir::new("jjtfg_outstripped_no_counterpart");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");

    assert!(!jjrfg_PlainGit.jjrfr_outstripped(td.path(), ZJJTFG_TRUNK).unwrap());
}

/// Bare remote, a primary tracking it with a pushed baseline, and a billet
/// detached at trunk's counterpart — the groom-billet birth form `reachable`'s
/// tests exercise (mirrors `zjjtfg_billeted_with_remote`, Detached instead of
/// Branch).
fn zjjtfg_detached_billeted_with_remote(name: &str) -> (JjkTestDir, JjkTestDir, JjkTestDir) {
    let (bare, primary) = zjjtfg_local_with_remote(name);
    let billet = zjjtfg_billet_slot(&format!("{}_billet", name));
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Detached, billet.path(), ZJJTFG_TRUNK)
        .unwrap();
    (bare, primary, billet)
}

#[test]
fn jjtfg_reachable_is_true_at_the_trunk_tip() {
    let (_bare, _primary, billet) = zjjtfg_detached_billeted_with_remote("jjtfg_reachable_at_tip");

    // Freshly detached exactly at trunk's counterpart: HEAD equals it, the
    // ancestor-or-equal case.
    assert!(jjrfg_PlainGit.jjrfr_reachable(billet.path(), ZJJTFG_TRUNK).unwrap());
}

#[test]
fn jjtfg_reachable_is_false_for_a_raw_local_commit() {
    let (_bare, _primary, billet) = zjjtfg_detached_billeted_with_remote("jjtfg_reachable_raw_commit");

    // A groom billet must retain nothing: a commit made directly on the
    // detached checkout is a raw commit reachable from nowhere, and the
    // stranding probe must say so.
    zjjtfg_commit_all(billet.path(), "stray.txt", "stray work", "raw detached commit");

    assert!(!jjrfg_PlainGit.jjrfr_reachable(billet.path(), ZJJTFG_TRUNK).unwrap());
}

#[test]
fn jjtfg_reachable_is_false_when_no_counterpart_is_known() {
    // A remote-less repo has no counterpart to be an ancestor of — the probe
    // must not prove destruction-safety on ignorance.
    let td = JjkTestDir::new("jjtfg_reachable_no_counterpart");
    zjjtfg_init_local(td.path());
    zjjtfg_commit_all(td.path(), "a.txt", "hello", "init");

    assert!(!jjrfg_PlainGit.jjrfr_reachable(td.path(), ZJJTFG_TRUNK).unwrap());
}

#[test]
fn jjtfg_billet_remove_reaps_a_clean_billet() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_remove_clean");
    let billet = zjjtfg_billet_slot("jjtfg_billet_remove_clean_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("removable".to_string()), billet.path(), ZJJTFG_TRUNK)
        .unwrap();

    jjrfg_PlainGit.jjrfr_billet_remove(billet.path()).unwrap();

    assert!(!billet.path().exists());
    let worktrees = zjjtfg_git(primary.path(), &["worktree", "list"]);
    assert!(!worktrees.contains("removable"));
}

#[test]
fn jjtfg_billet_remove_rejects_dirty_tree() {
    let (_bare, primary) = zjjtfg_local_with_remote("jjtfg_billet_remove_dirty");
    let billet = zjjtfg_billet_slot("jjtfg_billet_remove_dirty_billet");
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch("dirty-billet".to_string()), billet.path(), ZJJTFG_TRUNK)
        .unwrap();
    zjjtfg_write(billet.path(), "dirt.txt", "uncommitted");

    let result = jjrfg_PlainGit.jjrfr_billet_remove(billet.path());

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}

/// Bare remote, a primary tracking it with a pushed baseline, and a billet
/// forked off trunk. Enfold merges trunk's *counterpart* — `refs/remotes/origin/
/// <trunk>` — so there must be a remote for that ref to exist at all; a
/// remote-less repo has no counterpart to resolve.
fn zjjtfg_billeted_with_remote(name: &str) -> (JjkTestDir, JjkTestDir, JjkTestDir) {
    let (bare, primary) = zjjtfg_local_with_remote(name);
    let billet = zjjtfg_billet_slot(&format!("{}_billet", name));
    jjrfg_PlainGit
        .jjrfr_billet_create(
            primary.path(),
            &jjrfr_BilletBirth::Branch(format!("{}-billet", name)),
            billet.path(),
            ZJJTFG_TRUNK,
        )
        .unwrap();
    (bare, primary, billet)
}

/// Advance trunk the way a *published* trunk moves: commit on the primary and
/// push, so the counterpart ref actually moves. A commit without the push moves
/// the local trunk ref alone — which is precisely what enfold must ignore.
fn zjjtfg_trunk_advances(primary: &Path, name: &str, content: &str, message: &str) -> String {
    let sha = zjjtfg_commit_all(primary, name, content, message);
    zjjtfg_git(primary, &["push", "-q", "origin", ZJJTFG_TRUNK]);
    sha
}

#[test]
fn jjtfg_enfold_fast_forwards_when_billet_has_no_local_commits() {
    let (_bare, primary, billet) = zjjtfg_billeted_with_remote("jjtfg_enfold_ff");
    let tip = zjjtfg_trunk_advances(primary.path(), "b.txt", "trunk moved on", "trunk advances");

    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();

    let head = zjjtfg_git(billet.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, tip);
}

#[test]
fn jjtfg_enfold_merges_the_named_trunk_not_the_primarys_checkout() {
    let (_bare, primary, billet) = zjjtfg_billeted_with_remote("jjtfg_enfold_named");
    zjjtfg_trunk_advances(primary.path(), "b.txt", "trunk work", "trunk advances");
    // Park the primary on a different published line — trunk-ness must come from
    // the caller's name, never from the primary's ambient checkout.
    zjjtfg_git(primary.path(), &["checkout", "-q", "-b", "sidetrack"]);
    zjjtfg_commit_all(primary.path(), "c.txt", "side work", "side commit");
    zjjtfg_git(primary.path(), &["push", "-q", "origin", "sidetrack"]);

    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();

    assert!(billet.path().join("b.txt").exists());
    assert!(!billet.path().join("c.txt").exists());
}

#[test]
fn jjtfg_enfold_merges_the_counterpart_never_the_local_trunk_ref() {
    let (_bare, primary, billet) = zjjtfg_billeted_with_remote("jjtfg_enfold_counterpart");

    // Trunk moves locally but is NOT published — the operator's own unpushed
    // work, still mutable. Enfold must not see it: merging it would ride it out
    // to the remote as billet ancestry at the next consign.
    zjjtfg_commit_all(primary.path(), "unpushed.txt", "operator's own", "unpushed trunk work");
    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();
    assert!(
        !billet.path().join("unpushed.txt").exists(),
        "enfold must never read the local trunk ref"
    );

    // Published now — the counterpart moves, and the very same call brings it.
    zjjtfg_git(primary.path(), &["push", "-q", "origin", ZJJTFG_TRUNK]);
    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();
    assert!(
        billet.path().join("unpushed.txt").exists(),
        "a published trunk commit must enfold"
    );
}

#[test]
fn jjtfg_enfold_merges_divergent_trunk_and_billet_history() {
    let (_bare, primary, billet) = zjjtfg_billeted_with_remote("jjtfg_enfold_merge");
    zjjtfg_commit_all(billet.path(), "billet.txt", "billet work", "billet commit");
    zjjtfg_trunk_advances(primary.path(), "trunk.txt", "trunk work", "trunk commit");

    jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK).unwrap();

    assert!(billet.path().join("trunk.txt").exists());
    assert!(billet.path().join("billet.txt").exists());
    let log = zjjtfg_git(billet.path(), &["log", "--oneline", "-1"]);
    assert!(log.contains("enfold trunk"));
}

#[test]
fn jjtfg_enfold_rejects_dirty_tree() {
    let (_bare, primary, billet) = zjjtfg_billeted_with_remote("jjtfg_enfold_dirty");
    zjjtfg_trunk_advances(primary.path(), "b.txt", "trunk moved on", "trunk advances");
    zjjtfg_write(billet.path(), "dirt.txt", "uncommitted");

    let result = jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK);

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::DirtyTree);
}

#[test]
#[should_panic(expected = "unclassified git failure")]
fn jjtfg_enfold_fails_loud_on_conflict() {
    let (_bare, primary, billet) = zjjtfg_billeted_with_remote("jjtfg_enfold_conflict");
    zjjtfg_commit_all(billet.path(), "base.txt", "billet changed this line", "billet edits base.txt");
    zjjtfg_trunk_advances(primary.path(), "base.txt", "trunk changed this line too", "trunk edits base.txt");

    let _ = jjrfg_PlainGit.jjrfr_enfold(billet.path(), ZJJTFG_TRUNK);
}
