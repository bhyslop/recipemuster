// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::jjrf_favor::{jjrf_livery_compose, jjrf_LiveryKind};
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{jjrfr_BilletBirth, jjrfr_FarrierBillet};
use super::jjrnc_notch::{jjrnc_empty_notch_monitum, jjrnc_outside_list_warnings, jjrnc_run_notch, jjrnc_NotchArgs};
use super::jjtu_testdir::JjkTestDir;

fn jjtnc_files_set<'a>(files: &'a [&'a str]) -> HashSet<&'a str> {
    files.iter().copied().collect()
}

// A staged rename whose BOTH endpoints are in the file list is covered — no warning.
#[test]
fn jjtnc_rename_both_sides_listed_no_warning() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["old/path.rs", "new/path.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

// A rename also modified in the worktree ("RM") still resolves by its index status.
#[test]
fn jjtnc_rename_modified_both_sides_listed_no_warning() {
    let status = "RM old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["old/path.rs", "new/path.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

// A copy line uses the same "old -> new" shape and the same both-sides rule.
#[test]
fn jjtnc_copy_both_sides_listed_no_warning() {
    let status = "C  src/orig.rs -> src/dup.rs\n";
    let files = jjtnc_files_set(&["src/orig.rs", "src/dup.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

// A rename genuinely absent from the file list still warns.
#[test]
fn jjtnc_rename_neither_side_listed_warns() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["unrelated.rs"]);
    let warnings = jjrnc_outside_list_warnings(status, &files);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("old/path.rs -> new/path.rs"));
}

// The stricter rule: a rename with only one endpoint listed still warns.
#[test]
fn jjtnc_rename_only_new_side_listed_warns() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["new/path.rs"]);
    assert_eq!(jjrnc_outside_list_warnings(status, &files).len(), 1);
}

#[test]
fn jjtnc_rename_only_old_side_listed_warns() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["old/path.rs"]);
    assert_eq!(jjrnc_outside_list_warnings(status, &files).len(), 1);
}

// Non-rename entries keep their plain single-path coverage.
#[test]
fn jjtnc_modified_listed_no_warning() {
    let status = "M  tracked.rs\n";
    let files = jjtnc_files_set(&["tracked.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

#[test]
fn jjtnc_modified_outside_list_warns() {
    let status = "M  stray.rs\n";
    let files = jjtnc_files_set(&["tracked.rs"]);
    assert_eq!(jjrnc_outside_list_warnings(status, &files).len(), 1);
}

// A clean rename pair must not be masked by an unrelated stray file in the same status.
#[test]
fn jjtnc_rename_covered_stray_still_warns() {
    let status = "R  old/path.rs -> new/path.rs\nM  stray.rs\n";
    let files = jjtnc_files_set(&["old/path.rs", "new/path.rs"]);
    let warnings = jjrnc_outside_list_warnings(status, &files);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("stray.rs"));
}

// The empty-notch monitum is an advisory, never a refusal: it reports the landed commit.
#[test]
fn jjtnc_empty_notch_monitum_reports_the_landed_commit() {
    let monitum = jjrnc_empty_notch_monitum(&[]);
    assert!(monitum.starts_with("warning: "), "a monitum never gates — it warns: {}", monitum);
    assert!(monitum.contains("empty notch"));
    assert!(!monitum.contains("INTERDICTUM"), "an interdictum bars the act; this one landed");
}

// With no files listed, the monitum names the deliberate act — work that changed nothing on disk.
#[test]
fn jjtnc_empty_notch_monitum_no_files_names_the_deliberate_act() {
    let monitum = jjrnc_empty_notch_monitum(&[]);
    assert!(monitum.contains("No files were listed"));
    assert!(monitum.contains("changed nothing on disk"));
}

// With files listed and nothing staged, the monitum names the gap the caller did not expect.
#[test]
fn jjtnc_empty_notch_monitum_listed_files_name_the_gap() {
    let files = vec!["a.rs".to_string(), "b.rs".to_string()];
    let monitum = jjrnc_empty_notch_monitum(&files);
    assert!(monitum.contains("2 file(s) were listed"));
    assert!(monitum.contains("none held changes"));
    assert!(!monitum.contains("No files were listed"));
}

// ---- jjrnc_run_notch consign wiring (Ruling 3) ----
//
// jjrnc_run_notch reads and writes only through the ambient process cwd (no
// -C, no root parameter — vvce_git_command's own contract), so exercising it
// end-to-end means pointing the process cwd at a real repo. That is
// process-global state, so every test below is serialized through the
// crate-wide jjtu_testdir::JJTU_CWD_SERIAL.

const ZJJTNC_TRUNK: &str = "jjtnc-trunk";
const ZJJTNC_CORONET: &str = "CAAAA";

fn zjjtnc_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtnc_init_local(dir: &Path) {
    zjjtnc_git(dir, &["init", "-q", "-b", ZJJTNC_TRUNK]);
    zjjtnc_git(dir, &["config", "user.email", "jjtnc@example.invalid"]);
    zjjtnc_git(dir, &["config", "user.name", "jjtnc"]);
}

fn zjjtnc_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtnc_git(dir, &["add", "--", name]);
    zjjtnc_git(dir, &["commit", "-q", "-m", message]);
    zjjtnc_git(dir, &["rev-parse", "HEAD"])
}

fn zjjtnc_billet_slot(name: &str) -> JjkTestDir {
    let slot = JjkTestDir::new(name);
    std::fs::remove_dir_all(slot.path()).unwrap();
    slot
}

/// Points the process cwd at `dir` for the guard's lifetime, serialized
/// against every other cwd-hopping test in this module.
struct ZjjtncCwdGround {
    prior_cwd: PathBuf,
    _serial: std::sync::MutexGuard<'static, ()>,
}

impl ZjjtncCwdGround {
    fn new(dir: &Path) -> Self {
        let serial = super::jjtu_testdir::JJTU_CWD_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let prior_cwd = std::env::current_dir().expect("a cwd to restore");
        std::env::set_current_dir(dir).expect("point the process cwd at the test repo");
        ZjjtncCwdGround { prior_cwd, _serial: serial }
    }
}

impl Drop for ZjjtncCwdGround {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.prior_cwd);
    }
}

/// Bare remote, a primary tracking it with a pushed baseline commit, and a
/// pace billet forked off trunk wearing the `jjls_pace/{coronet}` badge —
/// the shared precondition every consign-wiring test builds on.
fn zjjtnc_billeted_primary(name: &str) -> (JjkTestDir, JjkTestDir, JjkTestDir, String) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtnc_git(bare.path(), &["init", "-q", "--bare", "-b", ZJJTNC_TRUNK]);
    let primary = JjkTestDir::new(&format!("{}_primary", name));
    zjjtnc_init_local(primary.path());
    zjjtnc_commit_all(primary.path(), "base.txt", "base", "init");
    zjjtnc_git(primary.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtnc_git(primary.path(), &["push", "-q", "-u", "origin", ZJJTNC_TRUNK]);
    let branch = jjrf_livery_compose(None, jjrf_LiveryKind::Pace, ZJJTNC_CORONET);
    let billet = zjjtnc_billet_slot(&format!("{}_billet", name));
    jjrfg_PlainGit
        .jjrfr_billet_create(primary.path(), &jjrfr_BilletBirth::Branch(branch.clone()), billet.path(), ZJJTNC_TRUNK)
        .unwrap();
    (bare, primary, billet, branch)
}

fn zjjtnc_remote_branch_tip(bare: &Path, branch: &str) -> Option<String> {
    let refs = zjjtnc_git(bare, &["for-each-ref", "--format=%(refname) %(objectname)"]);
    refs.lines()
        .find(|line| line.starts_with(&format!("refs/heads/{} ", branch)))
        .map(|line| line.rsplit(' ').next().unwrap().to_string())
}

// A notch on a badged pace-billet branch (`jjls_pace/{coronet}`) pushes it to
// the remote immediately — Ruling 3, "notch consigns the billet branch every
// time".
#[test]
fn jjtnc_notch_consigns_the_badged_pace_branch() {
    let (bare, _primary, billet, branch) = zjjtnc_billeted_primary("jjtnc_consign");
    let _ground = ZjjtncCwdGround::new(billet.path());
    std::fs::write(billet.path().join("work.txt"), "billet work").unwrap();

    let (rc, output) = jjrnc_run_notch(jjrnc_NotchArgs {
        identity: ZJJTNC_CORONET.to_string(),
        files: vec!["work.txt".to_string()],
        size_limit: None,
        intent: Some("do the work".to_string()),
    });

    assert_eq!(rc, 0, "notch output: {}", output);
    let local_head = zjjtnc_git(billet.path(), &["rev-parse", "HEAD"]);
    let remote_tip = zjjtnc_remote_branch_tip(bare.path(), &branch);
    assert_eq!(remote_tip, Some(local_head), "the notch commit must be pushed immediately");
}

// A notch on a branch outside the pace livery badge (here, the primary's own
// trunk checkout) commits locally but leaves the remote untouched — notch's
// ground is not yet gated, so the badge is the only signal telling a pace
// billet apart from everything else notch can currently run from.
#[test]
fn jjtnc_notch_on_an_unbadged_branch_never_consigns() {
    let (bare, primary, _billet, _branch) = zjjtnc_billeted_primary("jjtnc_unbadged");
    let remote_trunk_before = zjjtnc_git(bare.path(), &["rev-parse", ZJJTNC_TRUNK]);
    let _ground = ZjjtncCwdGround::new(primary.path());
    std::fs::write(primary.path().join("trunk-work.txt"), "trunk work").unwrap();

    let (rc, output) = jjrnc_run_notch(jjrnc_NotchArgs {
        identity: ZJJTNC_CORONET.to_string(),
        files: vec!["trunk-work.txt".to_string()],
        size_limit: None,
        intent: Some("trunk-side work".to_string()),
    });

    assert_eq!(rc, 0, "notch output: {}", output);
    let local_head = zjjtnc_git(primary.path(), &["rev-parse", "HEAD"]);
    assert_ne!(local_head, remote_trunk_before, "the local commit must still land");
    let remote_trunk_after = zjjtnc_git(bare.path(), &["rev-parse", ZJJTNC_TRUNK]);
    assert_eq!(remote_trunk_after, remote_trunk_before, "an unbadged branch must never be pushed by notch");
}

// The failure surface: a push a plain fast-forward cannot make (another
// station already advanced the same pace branch on the remote) reports loud
// through the same channel as every other notch error, and the exit code
// stops meaning success — while the local commit, already landed, stands.
#[test]
fn jjtnc_notch_reports_a_diverged_consign_as_failure() {
    let (bare, _primary, billet, branch) = zjjtnc_billeted_primary("jjtnc_diverged");
    // Another clone of the same pace branch publishes a commit the billet has
    // never seen, so the billet's own push can only fast-forward past it if
    // git rewrites history — which consign refuses to do (JJr_d81).
    let other = JjkTestDir::new("jjtnc_diverged_other");
    zjjtnc_git(
        bare.path(),
        &["clone", "-q", "-b", ZJJTNC_TRUNK, &bare.path().to_string_lossy(), &other.path().to_string_lossy()],
    );
    zjjtnc_git(other.path(), &["config", "user.email", "jjtnc@example.invalid"]);
    zjjtnc_git(other.path(), &["config", "user.name", "jjtnc"]);
    // The pace branch has never been pushed by anyone yet — the billet only
    // holds it locally (worktree-add -b). So the other station mints its own
    // local branch of the same name off trunk and publishes it first; that
    // publication is what the billet's later push cannot fast-forward past.
    zjjtnc_git(other.path(), &["checkout", "-q", "-b", &branch]);
    zjjtnc_commit_all(other.path(), "other.txt", "from another station", "other station advances the pace branch");
    zjjtnc_git(other.path(), &["push", "-q", "origin", &branch]);

    let _ground = ZjjtncCwdGround::new(billet.path());
    std::fs::write(billet.path().join("work.txt"), "billet work").unwrap();

    let (rc, output) = jjrnc_run_notch(jjrnc_NotchArgs {
        identity: ZJJTNC_CORONET.to_string(),
        files: vec!["work.txt".to_string()],
        size_limit: None,
        intent: Some("do the work".to_string()),
    });

    assert_eq!(rc, 1, "a diverged push must fail the notch: {}", output);
    assert!(output.contains("consign"), "the failure must name what failed: {}", output);
    // The local commit already landed — additive discipline never rolls it back.
    let local_subject = zjjtnc_git(billet.path(), &["log", "-1", "--pretty=%s"]);
    assert!(local_subject.contains("do the work"), "the local commit must stand despite the failed push");
    let remote_tip = zjjtnc_remote_branch_tip(bare.path(), &branch);
    let other_head = zjjtnc_git(other.path(), &["rev-parse", "HEAD"]);
    assert_eq!(remote_tip, Some(other_head), "the remote must still hold only the other station's commit");
}
