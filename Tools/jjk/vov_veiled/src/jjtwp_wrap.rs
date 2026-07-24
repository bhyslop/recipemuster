// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the wrap-entry staleness gate (`jjrwp_staleness_gate`) and its
//! refusal text (`jjri_staleness_interdictum`).
//!
//! The gate is the whole behavioral carrier: `zjjrx_run_wrap` consults it once
//! at entry and returns on `Outstripped`, so both ruled behaviors — a stale
//! billet refuses, a refitted billet passes — are decided here. The command
//! itself is deliberately not driven end-to-end: it reads the ambient working
//! directory and spawns the claude CLI to author a commit message, neither of
//! which a unit test can honestly stand up.
//!
//! The infield fixture (bare upstream, tracking hippodrome, one-pedigree
//! studbook) is reproduced locally rather than shared, matching this crate's own
//! convention (`jjtrd_refit.rs`, `jjtm_mcp.rs`).

use super::jjrds_spine::JJRDS_PEDIGREES_REL_PATH;
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
};
use super::jjri_io::jjri_staleness_interdictum;
use super::jjrrd_refit::jjrrd_run_refit;
use super::jjrvb_blotter::JJDB_STUDBOOK_DIRNAME;
use super::jjrwp_wrap::{
    jjrwp_staleness_gate,
    jjrwp_GateVerdict,
};
use super::jjtu_testdir::JjkTestDir;
use std::path::Path;

const ZJJTWP_TRUNK: &str = "jjtwp-trunk";
const ZJJTWP_CORONET: &str = "AAAAA";

fn zjjtwp_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtwp_init_local(dir: &Path) {
    zjjtwp_git(dir, &["init", "-q", "-b", ZJJTWP_TRUNK]);
    zjjtwp_git(dir, &["config", "user.email", "jjtwp@example.invalid"]);
    zjjtwp_git(dir, &["config", "user.name", "jjtwp"]);
}

fn zjjtwp_commit_all(dir: &Path, name: &str, content: &str, message: &str) {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtwp_git(dir, &["add", "--", name]);
    zjjtwp_git(dir, &["commit", "-q", "-m", message]);
}

/// A full infield: bare upstream, a hippodrome clone tracking it, and a studbook
/// whose one pedigree records the upstream.
fn zjjtwp_infield(name: &str) -> (JjkTestDir, std::path::PathBuf) {
    let infield = JjkTestDir::new(name);
    let bare = infield.path().join("upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtwp_git(&bare, &["init", "-q", "--bare", "-b", ZJJTWP_TRUNK]);

    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtwp_init_local(&hippodrome);
    zjjtwp_commit_all(&hippodrome, "base.txt", "base", "init");
    let bare_url = bare.to_string_lossy().into_owned();
    zjjtwp_git(&hippodrome, &["remote", "add", "origin", &bare_url]);
    zjjtwp_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTWP_TRUNK]);

    let studbook_root = infield.path().join(JJDB_STUDBOOK_DIRNAME);
    std::fs::create_dir_all(&studbook_root).unwrap();
    let body = serde_json::json!({
        "jjop_sires": [{
            "jjop_kind": "plain-git",
            "jjop_addresses": [bare_url],
            "jjop_trunk": ZJJTWP_TRUNK,
        }]
    });
    std::fs::write(studbook_root.join(JJRDS_PEDIGREES_REL_PATH), serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    (infield, hippodrome)
}

/// Birth a billet on `ZJJTWP_CORONET` off the hippodrome's trunk.
fn zjjtwp_billet(infield: &JjkTestDir, hippodrome: &Path) -> std::path::PathBuf {
    let billet_root = infield.path().join(format!("jjqb_{}", ZJJTWP_CORONET));
    jjrfg_PlainGit
        .jjrfr_billet_create(
            hippodrome,
            &jjrfr_BilletBirth::Branch(ZJJTWP_CORONET.to_string()),
            &billet_root,
            ZJJTWP_TRUNK,
        )
        .unwrap();
    billet_root
}

/// Advance trunk from the hippodrome and push it, leaving the billet untouched.
fn zjjtwp_advance_trunk(hippodrome: &Path) {
    zjjtwp_commit_all(hippodrome, "b.txt", "moved", "trunk advances");
    zjjtwp_git(hippodrome, &["push", "-q", "origin", ZJJTWP_TRUNK]);
}

#[test]
fn jjtwp_gate_clears_a_billet_born_at_the_current_trunk() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_clear");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &billet_root),
        jjrwp_GateVerdict::Clear
    );
}

#[test]
fn jjtwp_gate_refuses_a_billet_trunk_has_outstripped() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_refuses");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_advance_trunk(&hippodrome);

    // The gate gleans for itself: no fetch is staged here, and it still sees the
    // advance — staleness is fetch-revealed, and the gate does the revealing.
    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &billet_root),
        jjrwp_GateVerdict::Outstripped {
            branch: ZJJTWP_CORONET.to_string(),
            trunk: ZJJTWP_TRUNK.to_string(),
        }
    );
}

#[test]
fn jjtwp_gate_clears_the_same_billet_once_refit_has_run() {
    // The ruled sequence end to end: a stale billet refuses, the operator-directed
    // refit runs, and the re-attempt passes the gate.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_post_refit");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_advance_trunk(&hippodrome);

    assert!(matches!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &billet_root),
        jjrwp_GateVerdict::Outstripped { .. }
    ));

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, &billet_root);
    assert_eq!(code, 0, "refit output: {}", output);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &billet_root),
        jjrwp_GateVerdict::Clear,
        "a refitted billet must pass the gate the pre-refit attempt met"
    );
}

#[test]
fn jjtwp_gate_clears_on_hippodrome_ground_however_far_trunk_has_moved() {
    // The gate judges billets. A hippodrome behind its own remote is the ground
    // guards' territory, not this gate's — it must not be swept up by it.
    let (_infield, hippodrome) = zjjtwp_infield("jjtwp_gate_hippodrome");
    let other = JjkTestDir::new("jjtwp_gate_hippodrome_other");
    let bare_url = zjjtwp_git(&hippodrome, &["remote", "get-url", "origin"]);
    zjjtwp_git(other.path(), &["clone", "-q", &bare_url, "."]);
    zjjtwp_git(other.path(), &["config", "user.email", "jjtwp@example.invalid"]);
    zjjtwp_git(other.path(), &["config", "user.name", "jjtwp"]);
    zjjtwp_commit_all(other.path(), "c.txt", "elsewhere", "trunk advances elsewhere");
    zjjtwp_git(other.path(), &["push", "-q", "origin", ZJJTWP_TRUNK]);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &hippodrome),
        jjrwp_GateVerdict::Clear
    );
}

#[test]
fn jjtwp_gate_clears_on_foreign_ground() {
    let td = JjkTestDir::new("jjtwp_gate_foreign");
    // No git init — foreign ground. The gate refuses only on what it observed.

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, td.path()),
        jjrwp_GateVerdict::Clear
    );
}

#[test]
fn jjtwp_staleness_interdictum_leads_with_the_token_and_names_refit() {
    let msg = jjri_staleness_interdictum("jjx_wrap", ZJJTWP_CORONET, ZJJTWP_TRUNK);

    assert!(msg.starts_with("INTERDICTUM"), "got: {}", msg);
    assert!(msg.contains("jjx_wrap"), "the refusing command is named: {}", msg);
    assert!(msg.contains(ZJJTWP_CORONET) && msg.contains(ZJJTWP_TRUNK), "got: {}", msg);
    assert!(msg.contains("refit"), "the remedy is named: {}", msg);
    assert!(msg.contains("never rebase"), "got: {}", msg);
}
