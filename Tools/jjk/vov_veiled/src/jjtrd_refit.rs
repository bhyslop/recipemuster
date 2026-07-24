// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the refit door (`jjrrd_run_refit`) — the operator-invocable
//! surface over the already-tested `jjrrf_refit` engine.
//!
//! The infield fixture (bare upstream, tracking hippodrome, one-pedigree
//! studbook) is reproduced locally rather than shared, matching this crate's
//! own convention (`jjtm_mcp.rs`'s `zjjtm_staleness_infield` doc: "reproduced
//! here since it is test-module-private there").

use super::jjrds_stile::JJRDS_PEDIGREES_REL_PATH;
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
};
use super::jjrrd_refit::jjrrd_run_refit;
use super::jjrvb_blotter::JJDB_STUDBOOK_DIRNAME;
use super::jjtu_testdir::JjkTestDir;
use std::path::Path;

const ZJJTRD_TRUNK: &str = "jjtrd-trunk";

fn zjjtrd_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtrd_init_local(dir: &Path) {
    zjjtrd_git(dir, &["init", "-q", "-b", ZJJTRD_TRUNK]);
    zjjtrd_git(dir, &["config", "user.email", "jjtrd@example.invalid"]);
    zjjtrd_git(dir, &["config", "user.name", "jjtrd"]);
}

fn zjjtrd_commit_all(dir: &Path, name: &str, content: &str, message: &str) {
    std::fs::write(dir.join(name), content).unwrap();
    zjjtrd_git(dir, &["add", "--", name]);
    zjjtrd_git(dir, &["commit", "-q", "-m", message]);
}

/// A full infield: bare upstream, a hippodrome clone tracking it, and a
/// studbook whose one pedigree records the upstream — same shape as
/// `jjtm_mcp.rs`'s `zjjtm_staleness_infield`.
fn zjjtrd_staleness_infield(name: &str) -> (JjkTestDir, std::path::PathBuf) {
    let infield = JjkTestDir::new(name);
    let bare = infield.path().join("upstream");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtrd_git(&bare, &["init", "-q", "--bare", "-b", ZJJTRD_TRUNK]);

    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtrd_init_local(&hippodrome);
    zjjtrd_commit_all(&hippodrome, "base.txt", "base", "init");
    let bare_url = bare.to_string_lossy().into_owned();
    zjjtrd_git(&hippodrome, &["remote", "add", "origin", &bare_url]);
    zjjtrd_git(&hippodrome, &["push", "-q", "-u", "origin", ZJJTRD_TRUNK]);

    let studbook_root = infield.path().join(JJDB_STUDBOOK_DIRNAME);
    std::fs::create_dir_all(&studbook_root).unwrap();
    let body = serde_json::json!({
        "jjop_sires": [{
            "jjop_kind": "plain-git",
            "jjop_addresses": [bare_url],
            "jjop_trunk": ZJJTRD_TRUNK,
        }]
    });
    std::fs::write(studbook_root.join(JJRDS_PEDIGREES_REL_PATH), serde_json::to_vec_pretty(&body).unwrap()).unwrap();

    (infield, hippodrome)
}

#[test]
fn jjtrd_refit_reports_up_to_date_when_trunk_unmoved() {
    let (_infield, hippodrome) = zjjtrd_staleness_infield("jjtrd_up_to_date");

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, &hippodrome);

    assert_eq!(code, 0, "output: {}", output);
    assert!(output.contains("up to date"), "got: {}", output);
}

#[test]
fn jjtrd_refit_merges_and_reports_refitted_when_billet_reentered_stale() {
    // Finding B's exact motivating scenario: a billet born before trunk
    // advanced, never re-dispatched, and now the operator runs jjx_refit
    // directly against it.
    let (infield, hippodrome) = zjjtrd_staleness_infield("jjtrd_refitted");
    let billet_root = infield.path().join("jjqb_AAAAA");
    jjrfg_PlainGit
        .jjrfr_billet_create(&hippodrome, &jjrfr_BilletBirth::Branch("AAAAA".to_string()), &billet_root, ZJJTRD_TRUNK)
        .unwrap();

    // Trunk advances from the hippodrome and is pushed; the billet is never touched.
    zjjtrd_commit_all(&hippodrome, "b.txt", "moved", "trunk advances");
    zjjtrd_git(&hippodrome, &["push", "-q", "origin", ZJJTRD_TRUNK]);

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, &billet_root);

    assert_eq!(code, 0, "output: {}", output);
    assert!(output.contains("refitted"), "got: {}", output);

    // The merge landed and was pushed: the billet branch now carries b.txt,
    // and the remote counterpart matches the local tip.
    assert!(billet_root.join("b.txt").exists());
    let local_tip = zjjtrd_git(&billet_root, &["rev-parse", "AAAAA"]);
    let remote_tip = zjjtrd_git(&billet_root, &["rev-parse", "origin/AAAAA"]);
    assert_eq!(local_tip, remote_tip, "refit must push the merge, not just land it locally");
}

#[test]
fn jjtrd_refit_refuses_on_foreign_ground() {
    let td = JjkTestDir::new("jjtrd_foreign_ground");
    // No git init — foreign ground.

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, td.path());

    assert_eq!(code, 1, "output: {}", output);
    assert!(output.starts_with("jjx_refit: refused"), "got: {}", output);
}

#[test]
fn jjtrd_refit_refuses_when_no_pedigree_recorded() {
    // A plain local repo with no studbook at all: identify succeeds, but
    // there is no sire to resolve a trunk against.
    let td = JjkTestDir::new("jjtrd_no_pedigree");
    zjjtrd_init_local(td.path());
    zjjtrd_commit_all(td.path(), "a.txt", "hello", "init");

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, td.path());

    assert_eq!(code, 1, "output: {}", output);
    assert!(output.starts_with("jjx_refit: refused"), "got: {}", output);
}
