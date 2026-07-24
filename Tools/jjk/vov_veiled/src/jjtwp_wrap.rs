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
use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_BequeathOutcome,
    jjrfr_BilletBirth,
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_RejectionKind,
};
use super::jjri_io::{
    jjri_converge_refusal,
    jjri_staleness_interdictum,
};
use super::jjrrd_refit::jjrrd_run_refit;
use super::jjrvb_blotter::JJDB_STUDBOOK_DIRNAME;
use super::jjrwp_wrap::{
    jjrwp_billet_ground,
    jjrwp_chalk_message,
    jjrwp_staleness_gate,
    jjrwp_BilletGround,
    jjrwp_GateVerdict,
    JJRWP_TRAILER_NONE,
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
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root)),
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
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root)),
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
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root)),
        jjrwp_GateVerdict::Outstripped
    );

    let (code, output) = jjrrd_run_refit(&jjrfg_PlainGit, &billet_root);
    assert_eq!(code, 0, "refit output: {}", output);

    assert_eq!(
        jjrwp_staleness_gate(&jjrfg_PlainGit, &zjjtwp_ground(&billet_root)),
        jjrwp_GateVerdict::Clear,
        "a refitted billet must pass the gate the pre-refit attempt met"
    );
}

// ---- The converge ----

#[test]
fn jjtwp_converge_lands_the_billet_tree_on_trunk_over_the_trunk_tip() {
    let (infield, hippodrome) = zjjtwp_infield("jjtwp_converge_lands");
    let billet_root = zjjtwp_billet(&infield, &hippodrome);
    let ground = zjjtwp_ground(&billet_root);
    assert_eq!(jjrwp_staleness_gate(&jjrfg_PlainGit, &ground), jjrwp_GateVerdict::Clear);

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
