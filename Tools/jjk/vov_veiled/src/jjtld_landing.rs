// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the landing command's consign weld: `jjrld_run_landing` pushes
//! the billet branch to its remote counterpart as part of landing, so remote
//! custody of the L commit never waits on a session exit.
//!
//! `jjrld_run_landing` reads and writes only through the ambient process cwd
//! (`vvc::marker` and the consign helper both resolve it), so exercising it
//! end-to-end means pointing the process cwd at a real repo. That is
//! process-global state, so every test below is serialized through the
//! crate-wide jjtu_testdir::JJTU_CWD_SERIAL.

use std::path::{Path, PathBuf};

use super::jjrf_favor::{jjrf_livery_compose, jjrf_LiveryKind};
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{jjrfr_BilletBirth, jjrfr_FarrierBillet};
use super::jjrld_landing::{jjrld_run_landing, jjrld_LandingArgs};
use super::jjtu_testdir::JjkTestDir;

const ZJJTLD_TRUNK: &str = "jjtld-trunk";
const ZJJTLD_CORONET: &str = "CAAAA";

fn zjjtld_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtld_init_local(dir: &Path) {
    zjjtld_git(dir, &["init", "-q", "-b", ZJJTLD_TRUNK]);
    zjjtld_git(dir, &["config", "user.email", "jjtld@example.invalid"]);
    zjjtld_git(dir, &["config", "user.name", "jjtld"]);
}

fn zjjtld_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtld_git(dir, &["add", "--", name]);
    zjjtld_git(dir, &["commit", "-q", "-m", message]);
    zjjtld_git(dir, &["rev-parse", "HEAD"])
}

fn zjjtld_billet_slot(name: &str) -> JjkTestDir {
    let slot = JjkTestDir::new(name);
    std::fs::remove_dir_all(slot.path()).unwrap();
    slot
}

/// Points the process cwd at `dir` for the guard's lifetime, serialized
/// against every other cwd-hopping test in this module.
struct ZjjtldCwdGround {
    prior_cwd: PathBuf,
    _serial: std::sync::MutexGuard<'static, ()>,
}

impl ZjjtldCwdGround {
    fn new(dir: &Path) -> Self {
        let serial = super::jjtu_testdir::JJTU_CWD_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let prior_cwd = std::env::current_dir().expect("a cwd to restore");
        std::env::set_current_dir(dir).expect("point the process cwd at the test repo");
        ZjjtldCwdGround { prior_cwd, _serial: serial }
    }
}

impl Drop for ZjjtldCwdGround {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.prior_cwd);
    }
}

/// Bare remote, a primary tracking it with a pushed baseline commit, and a
/// pace billet forked off trunk wearing the `jjls_pace/{coronet}` badge —
/// the shared precondition every consign-weld test builds on.
fn zjjtld_billeted_primary(name: &str) -> (JjkTestDir, JjkTestDir, JjkTestDir, String) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtld_git(bare.path(), &["init", "-q", "--bare", "-b", ZJJTLD_TRUNK]);
    let primary = JjkTestDir::new(&format!("{}_primary", name));
    zjjtld_init_local(primary.path());
    zjjtld_commit_all(primary.path(), "base.txt", "base", "init");
    zjjtld_git(primary.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtld_git(primary.path(), &["push", "-q", "-u", "origin", ZJJTLD_TRUNK]);
    let branch = jjrf_livery_compose(None, jjrf_LiveryKind::Pace, ZJJTLD_CORONET);
    let billet = zjjtld_billet_slot(&format!("{}_billet", name));
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch(branch.clone()), billet.path(), ZJJTLD_TRUNK)
        .unwrap();
    (bare, primary, billet, branch)
}

fn zjjtld_remote_branch_tip(bare: &Path, branch: &str) -> Option<String> {
    let refs = zjjtld_git(bare, &["for-each-ref", "--format=%(refname) %(objectname)"]);
    refs.lines()
        .find(|line| line.starts_with(&format!("refs/heads/{} ", branch)))
        .map(|line| line.rsplit(' ').next().unwrap().to_string())
}

fn zjjtld_landing_args() -> jjrld_LandingArgs {
    jjrld_LandingArgs {
        coronet: ZJJTLD_CORONET.to_string(),
        agent: "sonnet".to_string(),
    }
}

// A landing on a badged pace-billet branch (`jjls_pace/{coronet}`) pushes it
// to the remote as part of landing — the weld: remote custody of the L commit
// never waits on a session exit, which a same-session land→wrap never fires.
#[test]
fn jjtld_landing_consigns_the_badged_pace_branch() {
    let (bare, _primary, billet, branch) = zjjtld_billeted_primary("jjtld_consign");
    let _ground = ZjjtldCwdGround::new(billet.path());

    let (rc, output) = jjrld_run_landing(zjjtld_landing_args(), "Work: commit abc123. Verified: suite green.".to_string());

    assert_eq!(rc, 0, "landing output: {}", output);
    let local_head = zjjtld_git(billet.path(), &["rev-parse", "HEAD"]);
    let remote_tip = zjjtld_remote_branch_tip(bare.path(), &branch);
    assert_eq!(remote_tip, Some(local_head), "the landing commit must be pushed as part of landing");
}

// A landing on a branch outside the pace livery badge (here, the primary's
// own trunk checkout) commits locally but leaves the remote untouched — the
// badge is the one signal telling a pace billet apart from everything else
// landing can currently run from.
#[test]
fn jjtld_landing_on_an_unbadged_branch_never_consigns() {
    let (bare, primary, _billet, _branch) = zjjtld_billeted_primary("jjtld_unbadged");
    let remote_trunk_before = zjjtld_git(bare.path(), &["rev-parse", ZJJTLD_TRUNK]);
    let _ground = ZjjtldCwdGround::new(primary.path());

    let (rc, output) = jjrld_run_landing(zjjtld_landing_args(), String::new());

    assert_eq!(rc, 0, "landing output: {}", output);
    let local_head = zjjtld_git(primary.path(), &["rev-parse", "HEAD"]);
    assert_ne!(local_head, remote_trunk_before, "the local L commit must still land");
    let remote_trunk_after = zjjtld_git(bare.path(), &["rev-parse", ZJJTLD_TRUNK]);
    assert_eq!(remote_trunk_after, remote_trunk_before, "an unbadged branch must never be pushed by landing");
}

// The failure surface: a push a plain fast-forward cannot make (another
// station already advanced the same pace branch on the remote) reports loud
// and the exit code stops meaning success — while the local L commit, already
// landed, stands (additive discipline never rolls it back).
#[test]
fn jjtld_landing_reports_a_diverged_consign_as_failure() {
    let (bare, _primary, billet, branch) = zjjtld_billeted_primary("jjtld_diverged");
    // Another clone publishes a commit on the same pace branch that the billet
    // has never seen, so the billet's own push cannot fast-forward past it.
    let other = JjkTestDir::new("jjtld_diverged_other");
    zjjtld_git(
        bare.path(),
        &["clone", "-q", "-b", ZJJTLD_TRUNK, &bare.path().to_string_lossy(), &other.path().to_string_lossy()],
    );
    zjjtld_git(other.path(), &["config", "user.email", "jjtld@example.invalid"]);
    zjjtld_git(other.path(), &["config", "user.name", "jjtld"]);
    zjjtld_git(other.path(), &["checkout", "-q", "-b", &branch]);
    zjjtld_commit_all(other.path(), "other.txt", "from another station", "other station advances the pace branch");
    zjjtld_git(other.path(), &["push", "-q", "origin", &branch]);

    let _ground = ZjjtldCwdGround::new(billet.path());

    let (rc, output) = jjrld_run_landing(zjjtld_landing_args(), String::new());

    assert_eq!(rc, 1, "a diverged push must fail the landing: {}", output);
    assert!(output.contains("consign"), "the failure must name what failed: {}", output);
    // The local L commit already landed — additive discipline never rolls it back.
    let local_subject = zjjtld_git(billet.path(), &["log", "-1", "--pretty=%s"]);
    assert!(local_subject.contains(":L:"), "the local L commit must stand despite the failed push: {}", local_subject);
    let remote_tip = zjjtld_remote_branch_tip(bare.path(), &branch);
    let other_head = zjjtld_git(other.path(), &["rev-parse", "HEAD"]);
    assert_eq!(remote_tip, Some(other_head), "the remote must still hold only the other station's commit");
}
