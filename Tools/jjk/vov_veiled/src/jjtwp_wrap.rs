// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the wrap's two billet beats — the entry staleness gate
//! (`jjrwp_staleness_gate`) with its refusal text (`jjri_staleness_interdictum`),
//! and the converge that bequeaths the billet's tree to trunk
//! (`jjrfr_bequeath`) with its own (`jjri_converge_refusal`) — over the one
//! ground resolution both stand on (`jjrwp_billet_ground`).
//!
//! Those carry the whole ruled behavior: `zjjrx_run_wrap` consults the gate once
//! at entry and returns on `Outstripped`, and delivers by one `jjrfr_bequeath`
//! call behind the work commit. The command itself is deliberately not driven
//! end-to-end: it reads the ambient working directory and spawns the claude CLI
//! to author a commit message, neither of which a unit test can honestly stand
//! up.
//!
//! The infield fixture (bare upstream, tracking hippodrome, one-pedigree
//! studbook) is reproduced locally rather than shared, matching this crate's own
//! convention (`jjtrd_refit.rs`, `jjtm_mcp.rs`).

use super::jjrds_stile::JJRDS_PEDIGREES_REL_PATH;
use super::jjrf_favor::{jjrf_livery_compose, jjrf_LiveryKind};
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BequeathOutcome,
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_RejectionKind,
};
use super::jjrg_gallops::{jjrg_Gallops, jjrg_Heat, jjrg_HeatStatus, jjrg_Pace, jjrg_PaceState, jjrg_Tack, JJRG_UNKNOWN_BASIS};
use super::jjri_io::{
    jjri_converge_refusal,
    jjri_staleness_interdictum,
};
use super::jjrrd_refit::jjrrd_run_refit;
use super::jjrvb_blotter::{jjdb_studbook_config, JJDB_GALLOPS_REL_PATH, JJDB_STUDBOOK_DIRNAME};
use super::jjrwp_wrap::{
    jjrwp_billet_ground,
    jjrwp_chalk_message,
    jjrwp_staleness_gate,
    jjrx_WrapArgs,
    zjjrx_run_wrap,
    jjrwp_BilletGround,
    jjrwp_GateVerdict,
    JJRWP_TRAILER_NONE,
};
use super::jjtu_testdir::JjkTestDir;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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

/// Advance trunk at the SIRE from a station of its own, so this infield learns
/// nothing of it until it gleans.
///
/// A billet is a partition of the hippodrome and shares its ref store, so a push
/// from the hippodrome updates the counterpart the billet reads in the same
/// stroke — which is the ordinary case, and precisely why it cannot stage a race.
/// Only a second station can leave the counterpart stale, and that is the
/// concurrency case the converge must answer for.
fn zjjtwp_stranger_advances_trunk(hippodrome: &Path, name: &str) {
    let stranger = JjkTestDir::new(name);
    let bare_url = zjjtwp_git(hippodrome, &["remote", "get-url", "origin"]);
    zjjtwp_git(stranger.path(), &["clone", "-q", &bare_url, "."]);
    zjjtwp_git(stranger.path(), &["config", "user.email", "jjtwp@example.invalid"]);
    zjjtwp_git(stranger.path(), &["config", "user.name", "jjtwp"]);
    zjjtwp_commit_all(stranger.path(), "b.txt", "moved", "trunk advances at another station");
    zjjtwp_git(stranger.path(), &["push", "-q", "origin", ZJJTWP_TRUNK]);
}

/// The ground a wrap in `billet_root` would resolve, or the test fails: every
/// gate and converge case below is about a billet the resolution DID claim.
fn zjjtwp_ground(billet_root: &Path) -> jjrwp_BilletGround {
    jjrwp_billet_ground(&jjrfg_PlainGit, billet_root).expect("the fixture billet must resolve as wrap ground")
}

/// The bare upstream's own view of trunk — what the sire actually holds, read
/// past every local tracking ref.
fn zjjtwp_sire_trunk(infield: &JjkTestDir) -> String {
    zjjtwp_git(&infield.path().join("upstream"), &["rev-parse", ZJJTWP_TRUNK])
}

/// The coronet every fixture billet wraps under.
fn zjjtwp_coronet() -> super::jjrf_favor::jjrf_Coronet {
    super::jjrf_favor::jjrf_Coronet::jjrf_parse(ZJJTWP_CORONET).unwrap()
}

// ---- Ground resolution ----

#[test]
fn jjtwp_ground_resolves_a_billet_to_its_branch_and_its_sire_trunk() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_ground_billet");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);

    let ground = zjjtwp_ground(&billet_root);
    assert_eq!(ground.branch, ZJJTWP_CORONET);
    assert_eq!(ground.trunk, ZJJTWP_TRUNK);
}

#[test]
fn jjtwp_ground_declines_hippodrome_ground_however_far_trunk_has_moved() {
    // The gate judges billets and the converge delivers them. A hippodrome behind
    // its own remote is the ground guards' territory, not either beat's — it must
    // not be swept up by them.
    let (_infield, hippodrome) = zjjtwp_infield("jjtwp_ground_hippodrome");
    let other = JjkTestDir::new("jjtwp_ground_hippodrome_other");
    let bare_url = zjjtwp_git(&hippodrome, &["remote", "get-url", "origin"]);
    zjjtwp_git(other.path(), &["clone", "-q", &bare_url, "."]);
    zjjtwp_git(other.path(), &["config", "user.email", "jjtwp@example.invalid"]);
    zjjtwp_git(other.path(), &["config", "user.name", "jjtwp"]);
    zjjtwp_commit_all(other.path(), "c.txt", "elsewhere", "trunk advances elsewhere");
    zjjtwp_git(other.path(), &["push", "-q", "origin", ZJJTWP_TRUNK]);

    assert_eq!(jjrwp_billet_ground(&jjrfg_PlainGit, &hippodrome), None);
}

#[test]
fn jjtwp_ground_declines_foreign_ground() {
    let td = JjkTestDir::new("jjtwp_ground_foreign");
    // No git init — foreign ground. Nothing is observed, so nothing is claimed.

    assert_eq!(jjrwp_billet_ground(&jjrfg_PlainGit, td.path()), None);
}

// ---- The staleness gate ----

#[test]
fn jjtwp_gate_clears_a_billet_born_at_the_current_trunk() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_clear");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root), &zjjtwp_coronet()),
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
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root), &zjjtwp_coronet()),
        jjrwp_GateVerdict::Outstripped
    );
}

#[test]
fn jjtwp_gate_clears_the_same_billet_once_refit_has_run() {
    // The ruled sequence end to end: a stale billet refuses, the operator-directed
    // refit runs, and the re-attempt passes the gate.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_post_refit");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_advance_trunk(&hippodrome);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root), &zjjtwp_coronet()),
        jjrwp_GateVerdict::Outstripped
    );

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, &billet_root);
    assert_eq!(code, 0, "refit output: {}", output);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root), &zjjtwp_coronet()),
        jjrwp_GateVerdict::Clear,
        "a refitted billet must pass the gate the pre-refit attempt met"
    );
}

#[test]
fn jjtwp_gate_resumes_when_trunk_holds_this_paces_own_landed_chalk() {
    // The crash-mid-wrap specimen: the converge already pushed this pace's own
    // W chalk to trunk (composed, not descended from the billet's history — a
    // squash leaves no ancestry), but the crash struck before the journal wrote.
    // A resumed wrap meets a trunk it never enfolded, yet the advance is its own
    // already-delivered work, not drift — the gate must resume, never refuse.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_resume");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_commit_all(&billet_root, "work.txt", "mine", "pace work");

    let coronet = zjjtwp_coronet();
    let chalk_message = format!(
        "{}\n\nBillet: {}",
        super::jjrn_notch::jjrn_format_chalk_message(&coronet, super::jjrn_notch::jjrn_ChalkMarker::Wrap, "pace complete"),
        ZJJTWP_CORONET
    );
    let landed = match jjrfg_PlainGit.jjrfr_bequeath(&billet_root, ZJJTWP_TRUNK, &chalk_message).unwrap() {
        jjrfr_BequeathOutcome::Landed(sha) => sha,
        other => panic!("the crash specimen's own converge must land, got {:?}", other),
    };
    assert_eq!(zjjtwp_sire_trunk(&infield), landed, "the specimen's converge must actually be trunk's tip");

    match jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root), &coronet) {
        jjrwp_GateVerdict::Resume(sha) => assert_eq!(sha, landed, "the resume must name the exact position the crashed converge landed"),
        other => panic!("a trunk carrying this pace's own chalk must resume, not {:?}", other),
    }
}

#[test]
fn jjtwp_gate_refuses_a_trunk_carrying_another_paces_chalk() {
    // Recognition is scoped to THIS coronet: a foreign pace's own W chalk landed
    // on trunk still reads as drift from this billet's perspective, not a resume
    // — the header names a different identity, so the gate must not confuse it
    // for its own crash-mid-wrap signature.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_gate_foreign_chalk");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);

    let foreign = super::jjrf_favor::jjrf_Coronet::jjrf_parse("BBBBB").unwrap();
    let chalk_message = format!(
        "{}\n\nBillet: {}",
        super::jjrn_notch::jjrn_format_chalk_message(&foreign, super::jjrn_notch::jjrn_ChalkMarker::Wrap, "a different pace complete"),
        "BBBBB"
    );
    zjjtwp_commit_all(&hippodrome, "c.txt", "elsewhere", &chalk_message);
    zjjtwp_git(&hippodrome, &["push", "-q", "origin", ZJJTWP_TRUNK]);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root), &zjjtwp_coronet()),
        jjrwp_GateVerdict::Outstripped,
        "another pace's own chalk must not be mistaken for this pace's resume"
    );
}

// ---- The converge ----

#[test]
fn jjtwp_converge_lands_the_billet_tree_on_trunk_over_the_trunk_tip() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_converge_lands");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    let ground = zjjtwp_ground(&billet_root);
    assert_eq!(jjrwp_staleness_gate(&jjrfg_PlainGit, &ground, &zjjtwp_coronet()), jjrwp_GateVerdict::Clear);

    // Two commits on the billet: what the squash must collapse into one.
    let trunk_before = zjjtwp_sire_trunk(&infield);
    zjjtwp_commit_all(&billet_root, "work.txt", "first", "pace work, part one");
    zjjtwp_commit_all(&billet_root, "work.txt", "second", "pace work, part two");
    let billet_tip = zjjtwp_git(&billet_root, &["rev-parse", "HEAD"]);
    let billet_tree = zjjtwp_git(&billet_root, &["rev-parse", "HEAD^{tree}"]);

    let landed = match jjrfg_PlainGit.jjrfr_bequeath(&billet_root, ZJJTWP_TRUNK, "the wrap's bequest").unwrap() {
        jjrfr_BequeathOutcome::Landed(sha) => sha,
        other => panic!("a billet holding work must land a position, got {:?}", other),
    };

    assert_eq!(zjjtwp_sire_trunk(&infield), landed, "the sire's trunk must stand at the position the bequest named");
    assert_eq!(
        zjjtwp_git(&billet_root, &["rev-parse", &format!("{}^{{tree}}", landed)]),
        billet_tree,
        "the estate must pass whole: trunk's new tree is the billet's tree exactly"
    );
    assert_eq!(
        zjjtwp_git(&billet_root, &["rev-list", "--parents", "-n", "1", &landed]),
        format!("{} {}", landed, trunk_before),
        "the bequest carries the trunk tip as its SOLE parent — one squash, no merge"
    );
    assert_eq!(
        zjjtwp_git(&billet_root, &["rev-parse", "HEAD"]),
        billet_tip,
        "the billet's own line of work must not move: no checkout, no adoption"
    );
    assert!(
        zjjtwp_git(&billet_root, &["log", "--oneline", &format!("{}..{}", trunk_before, landed)])
            .lines()
            .count()
            == 1,
        "the billet's interior history must not travel with the estate"
    );
}

#[test]
fn jjtwp_converge_reports_unchanged_when_trunk_already_holds_the_billet_tree() {
    // The verification-only pace: nothing was committed on the billet, so the
    // estate trunk already holds. Composing here would put an empty position on
    // trunk at every such wrap.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_converge_unchanged");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    let trunk_before = zjjtwp_sire_trunk(&infield);

    assert_eq!(
        jjrfg_PlainGit.jjrfr_bequeath(&billet_root, ZJJTWP_TRUNK, "nothing to pass").unwrap(),
        jjrfr_BequeathOutcome::Unchanged
    );
    assert_eq!(zjjtwp_sire_trunk(&infield), trunk_before, "an unchanged estate must move trunk not at all");
}

#[test]
fn jjtwp_converge_refused_by_a_raced_trunk_leaves_no_residue() {
    // Trunk moves after this billet's last glean, so the composed commit no longer
    // fast-forwards. The push is rejected, and the whole point of composing before
    // pushing is that there is nothing to scrub on either side.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_converge_raced");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_commit_all(&billet_root, "work.txt", "mine", "pace work");
    let billet_tip = zjjtwp_git(&billet_root, &["rev-parse", "HEAD"]);

    // The race: trunk advances at the sire from another station, and this billet
    // never gleans it — so the counterpart it composes on is already behind.
    zjjtwp_stranger_advances_trunk(&hippodrome, "jjtwp_converge_raced_stranger");
    let raced_trunk = zjjtwp_sire_trunk(&infield);

    let rejection = jjrfg_PlainGit
        .jjrfr_bequeath(&billet_root, ZJJTWP_TRUNK, "the bequest that loses the race")
        .expect_err("a raced trunk must refuse the bequest");
    assert_eq!(rejection.kind, jjrfr_RejectionKind::Diverged);

    assert_eq!(zjjtwp_sire_trunk(&infield), raced_trunk, "a refused bequest must leave the sire's trunk exactly where it stood");
    assert_eq!(
        zjjtwp_git(&billet_root, &["rev-parse", "HEAD"]),
        billet_tip,
        "a refused bequest must leave the billet's own line of work untouched"
    );
    assert!(
        jjrfg_PlainGit.jjrfr_comb(&billet_root).unwrap().jjrfr_is_clean(),
        "a refused bequest must leave no working-tree residue — it never wrote one"
    );
}

#[test]
fn jjtwp_converge_after_refit_lands_the_work_the_race_refused() {
    // The ruled recovery, end to end: the raced converge refuses, the
    // operator-directed refit runs, and the re-attempt delivers — carrying both
    // trunk's advance and the pace's own work.
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_converge_post_refit");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_commit_all(&billet_root, "work.txt", "mine", "pace work");
    zjjtwp_stranger_advances_trunk(&hippodrome, "jjtwp_converge_post_refit_stranger");

    assert!(jjrfg_PlainGit.jjrfr_bequeath(&billet_root, ZJJTWP_TRUNK, "refused").is_err());

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, &billet_root);
    assert_eq!(code, 0, "refit output: {}", output);

    let landed = match jjrfg_PlainGit.jjrfr_bequeath(&billet_root, ZJJTWP_TRUNK, "the re-attempted bequest").unwrap() {
        jjrfr_BequeathOutcome::Landed(sha) => sha,
        other => panic!("the re-attempt must land, got {:?}", other),
    };
    assert_eq!(zjjtwp_sire_trunk(&infield), landed);
    assert_eq!(
        zjjtwp_git(&billet_root, &["show", &format!("{}:work.txt", landed)]),
        "mine",
        "the pace's own work must reach trunk through the recovery"
    );
    assert_eq!(
        zjjtwp_git(&billet_root, &["show", &format!("{}:b.txt", landed)]),
        "moved",
        "trunk's own advance must survive the bequest that follows the refit"
    );
}

// ---- The journal's second counterfoil ----

#[test]
fn jjtwp_journal_message_records_the_position_the_converge_produced() {
    // The squash leaves trunk no ancestry back to the billet, so this trailer is
    // the only link between the pace's record and the position trunk accepted.
    let coronet = super::jjrf_favor::jjrf_Coronet::jjrf_parse(ZJJTWP_CORONET).unwrap();
    let msg = jjrwp_chalk_message(&coronet, "pace complete", "1234abcd", Some("a snag"));

    assert!(msg.contains("\nConverge: 1234abcd\n"), "the produced position rides its own line: {}", msg);
    assert!(msg.contains("\nSpook: a snag"), "the friction report keeps its own line beside it: {}", msg);
}

#[test]
fn jjtwp_journal_message_carries_both_trailers_even_with_nothing_to_say() {
    // Always-present trailers are what make a corpus greppable: an absent value is
    // spelled, never omitted, so no census has to reason about missing lines.
    let coronet = super::jjrf_favor::jjrf_Coronet::jjrf_parse(ZJJTWP_CORONET).unwrap();
    let msg = jjrwp_chalk_message(&coronet, "pace complete", JJRWP_TRAILER_NONE, None);

    assert!(msg.contains("\nConverge: none\n"), "got: {}", msg);
    assert!(msg.contains("\nSpook: none"), "got: {}", msg);
}

#[test]
fn jjtwp_journal_message_keeps_a_multiline_spook_on_one_line() {
    let coronet = super::jjrf_favor::jjrf_Coronet::jjrf_parse(ZJJTWP_CORONET).unwrap();
    let msg = jjrwp_chalk_message(&coronet, "pace complete", "1234abcd", Some("first\nsecond"));

    assert!(msg.ends_with("\nSpook: first second"), "got: {}", msg);
}

// ---- Refusal texts ----

#[test]
fn jjtwp_staleness_interdictum_leads_with_the_token_and_names_refit() {
    let msg = jjri_staleness_interdictum("jjx_wrap", ZJJTWP_CORONET, ZJJTWP_TRUNK);

    assert!(msg.starts_with("INTERDICTUM"), "got: {}", msg);
    assert!(msg.contains("jjx_wrap"), "the refusing command is named: {}", msg);
    assert!(msg.contains(ZJJTWP_CORONET) && msg.contains(ZJJTWP_TRUNK), "got: {}", msg);
    assert!(msg.contains("refit"), "the remedy is named: {}", msg);
    assert!(msg.contains("never rebase"), "got: {}", msg);
}

#[test]
fn jjtwp_converge_refusal_names_the_trunk_the_no_residue_fact_and_refit() {
    let rejection = super::jjrfr_farrier::jjrfr_Rejection::jjrfr_new(
        jjrfr_RejectionKind::Diverged,
        "bequeath",
        Path::new("/jjtwp/billet"),
        "[rejected] non-fast-forward",
    );
    let msg = jjri_converge_refusal("jjx_wrap", ZJJTWP_TRUNK, &rejection);

    assert!(!msg.starts_with("INTERDICTUM"), "a lost race bars nothing — it must not wear the gating token: {}", msg);
    assert!(msg.contains("jjx_wrap"), "the refusing command is named: {}", msg);
    assert!(msg.contains(ZJJTWP_TRUNK), "the trunk that refused is named: {}", msg);
    assert!(msg.contains("untouched"), "the no-residue fact is stated: {}", msg);
    assert!(msg.to_lowercase().contains("refit"), "the remedy is named: {}", msg);
    assert!(msg.contains("never rebase"), "got: {}", msg);
}

// ---- Consign wiring (Ruling 3 mirror, ₣B9 paddock ₢CAABf) ----
//
// `zjjrx_run_wrap` reads and writes only through the ambient process cwd, and
// (on a staged diff) shells out to the claude CLI to author a commit message —
// neither of which the rest of this file's tests drive end-to-end. Under the
// live studbook seam (`JJDB_GALLOPS_OVER_STUDBOOK_ENABLED`) the gallops tally
// journals to the studbook, not the billet, so a billet whose own work was
// already committed before wrap runs (no fresh staging) reaches the consign
// weld without the claude CLI at all: these tests seed the studbook's gallops
// journal and the billet's own work commit BEFORE calling wrap, matching the
// notch/landing consign trio's cwd-hopping pattern (`jjtnc_notch.rs`,
// `jjtld_landing.rs`) plus the seam-on studbook ground (`jjtu_seam_on_ground`).

/// A gallops holding exactly the one pace `ZJJTWP_CORONET` names, rough and
/// ready to tally — the minimal store `zjjrx_run_wrap` needs to find its own
/// coronet, transition it to complete, and run the lookahead.
fn zjjtwp_consign_gallops() -> jjrg_Gallops {
    let heat_key = "₣ZZ".to_string();
    let pace_key = format!("₢{}", ZJJTWP_CORONET);
    let tack = jjrg_Tack {
        ts: "260101-1200".to_string(),
        state: jjrg_PaceState::Rough,
        tier: None,
        effort: None,
        text: vec!["consign wiring test pace".to_string()],
        silks: "consign-wiring".to_string(),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
    };
    let mut paces = BTreeMap::new();
    paces.insert(pace_key.clone(), jjrg_Pace { tacks: vec![tack], ..Default::default() });
    let mut heats = BTreeMap::new();
    heats.insert(heat_key.clone(), jjrg_Heat {
        silks: "heat-zz".to_string(),
        creation_time: "260101".to_string(),
        status: jjrg_HeatStatus::Racing,
        order: vec![pace_key],
        paces,
    });
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![heat_key],
        heats,
        retention_since: None,
    }
}

/// Turn the infield's studbook directory — already holding the pedigree file
/// `zjjtwp_infield` wrote — into a real studbook clone: git-initialized,
/// seeded with the consign-gallops fixture, and pushed to a local bare origin
/// so `refs/remotes/origin/{trunk}` resolves exactly as
/// `jjtu_seam_on_ground` lays it at the real derived path
/// (`jjdb_studbook_config`'s own output). Wrap's studbook-config derivation
/// (`zjjrm_infield_root` → `jjdb_studbook_config`) finds this directory from
/// the billet's cwd, so no separate config needs threading to the command.
fn zjjtwp_seed_studbook_gallops(infield: &JjkTestDir) {
    let config = jjdb_studbook_config(infield.path());
    let trunk = config.trunk.clone();
    let local = &config.local_root;

    let bare = infield.path().join("studbook_origin.git");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtwp_git(&bare, &["init", "-q", "--bare", "-b", &trunk]);

    zjjtwp_git(local, &["init", "-q", "-b", &trunk]);
    zjjtwp_git(local, &["config", "user.email", "jjtwp@example.invalid"]);
    zjjtwp_git(local, &["config", "user.name", "jjtwp"]);
    zjjtwp_consign_gallops()
        .jjrg_save(&local.join(JJDB_GALLOPS_REL_PATH))
        .expect("seed gallops saves to the studbook clone");
    zjjtwp_git(local, &["add", "-A"]);
    zjjtwp_git(local, &["commit", "-q", "-m", "seed studbook"]);
    zjjtwp_git(local, &["remote", "add", "origin", &bare.to_string_lossy()]);
    zjjtwp_git(local, &["push", "-q", "-u", "origin", &trunk]);
}

/// Points the process cwd at `dir` for the guard's lifetime, serialized
/// against every other cwd-hopping test in this module.
struct ZjjtwpCwdGround {
    prior_cwd: PathBuf,
    _serial: std::sync::MutexGuard<'static, ()>,
}

impl ZjjtwpCwdGround {
    fn new(dir: &Path) -> Self {
        let serial = super::jjtu_testdir::JJTU_CWD_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let prior_cwd = std::env::current_dir().expect("a cwd to restore");
        std::env::set_current_dir(dir).expect("point the process cwd at the test repo");
        ZjjtwpCwdGround { prior_cwd, _serial: serial }
    }
}

impl Drop for ZjjtwpCwdGround {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.prior_cwd);
    }
}

/// An infield plus a pace billet wearing the `jjls_pace/{coronet}` badge,
/// carrying its own work commit, with the infield's studbook journal seeded —
/// the shared precondition every consign-wiring test builds on, mirroring
/// `zjjtnc_billeted_primary` / `zjjtld_billeted_primary`. The work commit
/// lands BEFORE wrap runs (never staged at wrap time), so the tree is clean
/// when `zjjrx_run_wrap` stages — no claude-CLI commit-message path fires,
/// and this is exactly the commit the consign weld must still push.
fn zjjtwp_billeted_infield(name: &str) -> (JjkTestDir, PathBuf, String) {
    let (infield, hippodrome) = zjjtwp_infield(name);
    let branch = jjrf_livery_compose(None, jjrf_LiveryKind::Pace, ZJJTWP_CORONET);
    let billet_root = infield.path().join(format!("jjqb_{}_badged", ZJJTWP_CORONET));
    jjrfg_PlainGit
        .jjrfr_billet_create(&hippodrome, &jjrfr_BilletBirth::Branch(branch.clone()), &billet_root, ZJJTWP_TRUNK)
        .unwrap();
    zjjtwp_seed_studbook_gallops(&infield);
    zjjtwp_commit_all(&billet_root, "work.txt", "mine", "pace work");
    (infield, billet_root, branch)
}

fn zjjtwp_remote_branch_tip(bare: &Path, branch: &str) -> Option<String> {
    let refs = zjjtwp_git(bare, &["for-each-ref", "--format=%(refname) %(objectname)"]);
    refs.lines()
        .find(|line| line.starts_with(&format!("refs/heads/{} ", branch)))
        .map(|line| line.rsplit(' ').next().unwrap().to_string())
}

fn zjjtwp_wrap_args() -> jjrx_WrapArgs {
    jjrx_WrapArgs { coronet: ZJJTWP_CORONET.to_string(), size_limit: None }
}

// A wrap on a badged pace-billet branch (`jjls_pace/{coronet}`) pushes it to
// the remote after its final commit — the Ruling 3 mirror: the converge above
// already delivers the tree to trunk, but the branch itself must still reach
// remote custody, so a same-session land→wrap exits with the stile clearing
// the billet instead of raising a false-alarm custody warning.
#[test]
fn jjtwp_wrap_consigns_the_badged_pace_branch() {
    let (infield, billet_root, branch) = zjjtwp_billeted_infield("jjtwp_consign");
    let _ground = ZjjtwpCwdGround::new(&billet_root);

    let (rc, output) = zjjrx_run_wrap(zjjtwp_wrap_args(), Some("consign wiring test".to_string()), None, "");

    assert_eq!(rc, 0, "wrap output: {}", output);
    let local_head = zjjtwp_git(&billet_root, &["rev-parse", "HEAD"]);
    let remote_tip = zjjtwp_remote_branch_tip(&infield.path().join("upstream"), &branch);
    assert_eq!(remote_tip, Some(local_head), "the wrap's chalk commit must be pushed as part of wrap");
}

// A wrap on a branch outside the pace livery badge (a bare-coronet checkout,
// the shape every other wrap fixture in this file uses) commits locally but
// leaves the remote untouched — the badge is the one signal telling a pace
// billet apart from everything else wrap can currently run from.
#[test]
fn jjtwp_wrap_on_an_unbadged_branch_never_consigns() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_wrap_unbadged");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    zjjtwp_seed_studbook_gallops(&infield);
    zjjtwp_commit_all(&billet_root, "work.txt", "mine", "pace work");
    let _ground = ZjjtwpCwdGround::new(&billet_root);

    let (rc, output) = zjjrx_run_wrap(zjjtwp_wrap_args(), Some("consign wiring test".to_string()), None, "");

    assert_eq!(rc, 0, "wrap output: {}", output);
    let remote_tip = zjjtwp_remote_branch_tip(&infield.path().join("upstream"), ZJJTWP_CORONET);
    assert_eq!(remote_tip, None, "an unbadged branch must never be pushed by wrap");
}

// The failure surface: a push a plain fast-forward cannot make (another
// station already advanced the same pace branch on the remote) reports loud
// through the same channel as every other wrap error, and the exit code stops
// meaning success — while the converge that already delivered the tree to
// trunk, and the billet's own local work commit, both stand (additive
// discipline never rolls either back).
#[test]
fn jjtwp_wrap_reports_a_diverged_consign_as_failure() {
    let (infield, billet_root, branch) = zjjtwp_billeted_infield("jjtwp_wrap_diverged");
    let bare = infield.path().join("upstream");
    // Another station mints its own local branch of the same badged name off
    // trunk and publishes it first — the billet only ever holds the badge
    // locally (worktree-add -b), so this publication is what the billet's
    // later push cannot fast-forward past.
    let other = JjkTestDir::new("jjtwp_wrap_diverged_other");
    let bare_url = bare.to_string_lossy().into_owned();
    zjjtwp_git(other.path(), &["clone", "-q", "-b", ZJJTWP_TRUNK, &bare_url, "."]);
    zjjtwp_git(other.path(), &["config", "user.email", "jjtwp@example.invalid"]);
    zjjtwp_git(other.path(), &["config", "user.name", "jjtwp"]);
    zjjtwp_git(other.path(), &["checkout", "-q", "-b", &branch]);
    zjjtwp_commit_all(other.path(), "other.txt", "from another station", "other station advances the pace branch");
    zjjtwp_git(other.path(), &["push", "-q", "origin", &branch]);

    let _ground = ZjjtwpCwdGround::new(&billet_root);

    let (rc, output) = zjjrx_run_wrap(zjjtwp_wrap_args(), Some("consign wiring test".to_string()), None, "");

    assert_eq!(rc, 1, "a diverged push must fail the wrap: {}", output);
    assert!(output.contains("consign"), "the failure must name what failed: {}", output);
    // The billet's own work commit already stood before wrap ran — additive
    // discipline never rolls it back, and the seam-on chalk step journals to
    // the studbook rather than adding a further commit here.
    let local_subject = zjjtwp_git(&billet_root, &["log", "-1", "--pretty=%s"]);
    assert!(local_subject.contains("pace work"), "the billet's own work commit must stand despite the failed push: {}", local_subject);
    let remote_tip = zjjtwp_remote_branch_tip(&bare, &branch);
    let other_head = zjjtwp_git(other.path(), &["rev-parse", "HEAD"]);
    assert_eq!(remote_tip, Some(other_head), "the remote must still hold only the other station's commit");
}
