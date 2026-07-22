// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrfg_plaingit::jjrfg_PlainGit;
use super::jjrfr_farrier::{
    jjrfr_break,
    jjrfr_FarrierLock,
    jjrfr_RejectionKind,
};
use super::jjrds_spine::{
    JJRDS_KIND_PLAIN_GIT,
    JJRDS_PEDIGREES_REL_PATH,
};
use super::jjrt_types::{
    jjrg_Gallops,
    jjrg_Heat,
    jjrg_HeatStatus,
    jjrg_Pace,
    jjrg_PaceState,
    jjrg_Tack,
    jjrg_DraftArgs,
    jjrg_FurloughArgs,
    jjrg_NominateArgs,
    jjrg_RailArgs,
    jjrg_RestringArgs,
    jjrg_SlateArgs,
    jjrg_TallyArgs,
    JJRG_UNKNOWN_BASIS,
};
use super::jjrf_favor::jjrf_Firemark;
use super::jjrvb_blotter::{
    jjdb_found,
    jjdb_founding_import,
    jjdb_gallops_journal_load,
    jjdb_gallops_journal_save,
    jjdb_gallops_journal_try_save,
    jjdb_journal,
    jjdb_read,
    jjdb_studbook_config,
    jjdb_BlotterConfig,
    jjdb_JournalReject,
    JJDB_CATCHWORD_FOUNDING,
    JJDB_CATCHWORD_SIGIL,
    JJDB_GALLOPS_OVER_STUDBOOK_ENABLED,
};
use super::jjtu_testdir::JjkTestDir;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{
    Path,
    PathBuf,
};

const ZJJTVB_TRUNK: &str = "jjtvb-trunk";

fn zjjtvb_git(dir: &Path, args: &[&str]) -> String {
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

fn zjjtvb_init_local(dir: &Path) {
    zjjtvb_git(dir, &["init", "-q", "-b", ZJJTVB_TRUNK]);
    zjjtvb_git(dir, &["config", "user.email", "jjtvb@example.invalid"]);
    zjjtvb_git(dir, &["config", "user.name", "jjtvb"]);
}

fn zjjtvb_init_bare(dir: &Path) {
    zjjtvb_git(dir, &["init", "-q", "--bare", "-b", ZJJTVB_TRUNK]);
}

fn zjjtvb_write(dir: &Path, name: &str, content: &str) {
    std::fs::write(dir.join(name), content).unwrap();
}

fn zjjtvb_commit_all(dir: &Path, name: &str, content: &str, message: &str) -> String {
    zjjtvb_write(dir, name, content);
    zjjtvb_git(dir, &["add", "--", name]);
    zjjtvb_git(dir, &["commit", "-q", "-m", message]);
    zjjtvb_git(dir, &["rev-parse", "HEAD"])
}

/// The expected baked-subject form for an ordinal/message pair, matching
/// `zjjrvb_bake_ordinal`'s composition — the one place tests need to know the
/// bake's exact shape.
fn zjjtvb_baked_subject(ordinal: u64, message: &str) -> String {
    format!("{}{}: {}", JJDB_CATCHWORD_SIGIL, ordinal, message)
}

/// A bare remote plus one local clone tracking it, with a baseline commit
/// pushed — the blotter engine's standard scratch fixture. `name` must be
/// unique per caller: tests run concurrently against real on-disk git repos.
fn zjjtvb_scratch(name: &str) -> (JjkTestDir, JjkTestDir, jjdb_BlotterConfig) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtvb_init_bare(bare.path());
    let local = JjkTestDir::new(&format!("{}_local", name));
    zjjtvb_init_local(local.path());
    zjjtvb_commit_all(local.path(), "base.txt", "base", "init");
    zjjtvb_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtvb_git(local.path(), &["push", "-q", "-u", "origin", ZJJTVB_TRUNK]);
    let config = jjdb_BlotterConfig {
        local_root: local.path().to_path_buf(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    (bare, local, config)
}

#[test]
fn jjtvb_journal_lodges_content_and_pushes_it() {
    let (bare, _local, config) = zjjtvb_scratch("jjtvb_journal_happy");

    let sha = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-happy", |root| {
        zjjtvb_write(root, "entry.txt", "journaled content");
        (vec![PathBuf::from("entry.txt")], "journal entry".to_string())
    })
    .unwrap();

    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha);
    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--pretty=%s", ZJJTVB_TRUNK]);
    assert_eq!(subject, zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 1, "journal entry"));
}

#[test]
fn jjtvb_journal_releases_the_lock_after_completion() {
    let (bare, _local, config) = zjjtvb_scratch("jjtvb_journal_release");

    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-release", |root| {
        zjjtvb_write(root, "entry.txt", "content");
        (vec![PathBuf::from("entry.txt")], "entry".to_string())
    })
    .unwrap();

    let remaining = zjjtvb_git(bare.path(), &["for-each-ref", "refs/jjv"]);
    assert!(remaining.is_empty(), "guidon must be released after a completed ceremony");
}

#[test]
fn jjtvb_journal_rejects_lock_held_and_never_calls_mutate() {
    let (_bare, local, config) = zjjtvb_scratch("jjtvb_journal_lock_held");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-another-station").unwrap();

    let called = Cell::new(false);
    let result = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-contender", |root| {
        called.set(true);
        zjjtvb_write(root, "should-not-land.txt", "x");
        (vec![PathBuf::from("should-not-land.txt")], "should not land".to_string())
    });

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockHeld);
    assert!(!called.get(), "mutate must not run when the lock cannot be acquired");
}

#[test]
fn jjtvb_journal_rejects_lock_broken_mid_ceremony_and_pushes_nothing() {
    let (bare, local, config) = zjjtvb_scratch("jjtvb_journal_lock_broken");
    let baseline = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);

    // Another station breaks our lock and stakes its own while we sit between
    // sight and consign — the race the lock-ref lease exists to close. The
    // mutate closure is where the ceremony dwells at that point, so the break
    // is staged from inside it.
    let result = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-victim", |root| {
        jjrfg_PlainGit.jjrfr_pluck(root, "guidon-victim").unwrap();
        jjrfg_PlainGit.jjrfr_stake(root, "guidon-usurper").unwrap();
        zjjtvb_write(root, "stranded.txt", "must never reach the remote");
        (vec![PathBuf::from("stranded.txt")], "stranded entry".to_string())
    });

    assert_eq!(result.unwrap_err().kind, jjrfr_RejectionKind::LockBroken);
    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, baseline, "the broken-lock ceremony must land nothing on the remote");
    let flying = jjrfg_PlainGit.jjrfr_sight(local.path()).unwrap();
    assert_eq!(
        flying.as_deref(),
        Some("guidon-usurper"),
        "the usurper's lock must survive both the failed consign and our guard's release"
    );
}

/// JJr_b52, the no-residue proof: a ceremony whose proffer the lease refused
/// leaves the local branch and its record UNTOUCHED — the refused commit never
/// rides the branch, so there is nothing for any later ceremony to scrub, and
/// the station's next authorized write proceeds clean. (Under the retired
/// equalize-and-retrench construction the refused commit landed on the branch
/// and had to be destroyed under the next lock; compose-then-push never
/// creates it there at all.)
#[test]
fn jjtvb_journal_refused_proffer_leaves_branch_and_record_untouched() {
    let (bare, local, config) = zjjtvb_scratch("jjtvb_journal_refused");
    let local_baseline = zjjtvb_git(local.path(), &["rev-parse", "HEAD"]);

    // Drive the station into the refused state: another station breaks our lock
    // mid-ceremony, so our proffer fails its lease with the commit composed.
    let err = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-victim", |root| {
        jjrfg_PlainGit.jjrfr_pluck(root, "guidon-victim").unwrap();
        jjrfg_PlainGit.jjrfr_stake(root, "guidon-usurper").unwrap();
        zjjtvb_write(root, "refused.txt", "refused by the lock, never authorized");
        (vec![PathBuf::from("refused.txt")], "REFUSED: the lock said no".to_string())
    })
    .expect_err("the broken lease must refuse the proffer");
    assert_eq!(err.kind, jjrfr_RejectionKind::LockBroken);

    // The refused-consign invariant: local branch and record untouched.
    let head = zjjtvb_git(local.path(), &["rev-parse", "HEAD"]);
    assert_eq!(head, local_baseline, "a refused proffer must leave the local branch exactly where it stood");
    let on_branch = zjjtvb_git(local.path(), &["log", "--pretty=%s", "HEAD"]);
    assert!(!on_branch.contains("REFUSED"), "the refused commit must never ride the local branch");

    jjrfg_PlainGit.jjrfr_pluck(local.path(), "guidon-usurper").unwrap();

    // With the lock free again, the station writes something else entirely.
    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-retry", |root| {
        zjjtvb_write(root, "intended.txt", "the write the station actually intended");
        (vec![PathBuf::from("intended.txt")], "the intended write".to_string())
    })
    .expect("with the lock free, the next ceremony completes");

    let pushed = zjjtvb_git(bare.path(), &["log", "--pretty=%s", ZJJTVB_TRUNK]);
    assert!(
        !pushed.lines().any(|l| l.contains("REFUSED")),
        "a commit the lock refused must never ride onto the store on a later write; remote history was:\n{}",
        pushed
    );
    assert!(
        pushed.lines().any(|l| l.contains("the intended write")),
        "the authorized write must land; remote history was:\n{}",
        pushed
    );
}

#[test]
fn jjtvb_journal_advances_past_a_prior_journaled_entry_before_writing() {
    let (bare, local, config) = zjjtvb_scratch("jjtvb_journal_advance");

    // A second clone runs its own full ceremony first and lands on the
    // remote — simulating a prior write from another station.
    let other = JjkTestDir::new("jjtvb_journal_advance_other");
    zjjtvb_git(bare.path(), &["clone", "-q", "-b", ZJJTVB_TRUNK, &bare.path().to_string_lossy(), &other.path().to_string_lossy()]);
    zjjtvb_git(other.path(), &["config", "user.email", "jjtvb@example.invalid"]);
    zjjtvb_git(other.path(), &["config", "user.name", "jjtvb"]);
    let other_config = jjdb_BlotterConfig {
        local_root: other.path().to_path_buf(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_journal(&jjrfg_PlainGit, &other_config, "guidon-other-station", |root| {
        zjjtvb_write(root, "from-other.txt", "other station's entry");
        (vec![PathBuf::from("from-other.txt")], "from other station".to_string())
    })
    .unwrap();

    // The original clone is now behind; its own ceremony must advance past
    // the other station's entry before adding its own.
    let sha = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-mine", |root| {
        zjjtvb_write(root, "mine.txt", "my entry");
        (vec![PathBuf::from("mine.txt")], "mine".to_string())
    })
    .unwrap();

    assert!(local.path().join("from-other.txt").exists(), "advance must pick up the other station's entry");
    assert!(local.path().join("mine.txt").exists());
    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha);
}

#[test]
fn jjtvb_read_reads_local_content_without_network() {
    let (_bare, local, config) = zjjtvb_scratch("jjtvb_read_local");
    zjjtvb_write(local.path(), "readable.txt", "local content");
    zjjtvb_git(local.path(), &["remote", "set-url", "origin", "/nonexistent/jjtvb-nowhere"]);

    let bytes = jjdb_read(&config, Path::new("readable.txt")).unwrap();

    assert_eq!(bytes, b"local content");
}

#[test]
fn jjtvb_read_reports_absent_content_as_an_ordinary_io_error() {
    let (_bare, _local, config) = zjjtvb_scratch("jjtvb_read_absent");

    let result = jjdb_read(&config, Path::new("never-written.txt"));

    assert!(result.is_err());
}

fn zjjtvb_valid_gallops() -> jjrg_Gallops {
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order: vec![],
        heats: BTreeMap::new(),
        retention_since: None,
    }
}

#[test]
fn jjtvb_gallops_journal_save_then_load_round_trips() {
    let (_bare, _local, config) = zjjtvb_scratch("jjtvb_gallops_roundtrip");
    let gallops = zjjtvb_valid_gallops();

    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-gallops", |current| {
        assert!(current.is_none(), "a store with no gallops tenant yet must hand the transform None");
        zjjtvb_valid_gallops()
    }, "save gallops".to_string())
    .unwrap();

    let loaded = jjdb_gallops_journal_load(&config).unwrap();
    assert_eq!(loaded.inner().next_heat_seed, gallops.next_heat_seed);
    assert!(loaded.inner().heats.is_empty());
}

/// Mutate-as-transform (JJr_b52): the transform receives the gallops as the
/// locked tip holds it — never a value the caller composed from a pre-lock
/// read — so a second write derives from what the first actually landed.
#[test]
fn jjtvb_gallops_journal_save_hands_the_transform_the_locked_tip_state() {
    let (_bare, _local, config) = zjjtvb_scratch("jjtvb_gallops_transform");
    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-seed", |_| {
        let mut g = zjjtvb_valid_gallops();
        g.next_heat_seed = "AZ".to_string();
        g
    }, "seed".to_string())
    .unwrap();

    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-derive", |current| {
        let current = current.expect("the seeded tenant must be handed in").into_inner();
        assert_eq!(current.next_heat_seed, "AZ", "the transform must see what the prior write landed");
        current
    }, "derive".to_string())
    .unwrap();
}

// ---- The WRITE seam's fallible ceremony (jjdb_gallops_journal_try_save) ----
// The command surface's write half routes every mutating jjx command's mutation
// through this when the seam is on. These prove the seam-on guarantees the
// done-when names; the seam-off command paths are proven byte-identical by the
// rest of the suite (the const is compiled false).

/// Seam-ON happy path: a command-style mutation journals to the studbook and a
/// read-after-write within the session sees it — the seam's core promise
/// ("lands each mutating command as a journaled studbook commit"; "read-after-write
/// within a session sees the write").
#[test]
fn jjtvb_gallops_journal_try_save_round_trips_a_command_mutation() {
    let (_bare, _local, config) = zjjtvb_scratch("jjtvb_try_save_roundtrip");
    // Seed a tenant so the transform is handed Some(tip), as a live command is.
    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-seed", |_| {
        let mut g = zjjtvb_valid_gallops();
        g.next_heat_seed = "AA".to_string();
        g
    }, "seed".to_string())
    .unwrap();

    // A command handler mutates the locked tip and hands back its record + message.
    jjdb_gallops_journal_try_save(&jjrfg_PlainGit, &config, "guidon-cmd", |tip| {
        let mut g = tip.expect("the seeded tenant must be handed in").into_inner();
        g.next_heat_seed = "AB".to_string();
        Ok((g, "command mutation".to_string()))
    })
    .unwrap();

    // Read-after-write within the session sees the landed mutation.
    let loaded = jjdb_gallops_journal_load(&config).unwrap();
    assert_eq!(loaded.inner().next_heat_seed, "AB");
}

/// Seam-ON abort: a declining transform (a command handler whose precondition no
/// longer holds on the locked tip) lands NOTHING and releases the lock — the
/// pre-seam invariant "a failed command commits nothing", made structural. The
/// remote tip is unchanged and a following ceremony succeeds against the freed lock.
#[test]
fn jjtvb_gallops_journal_try_save_abort_commits_nothing_and_releases() {
    let (bare, _local, config) = zjjtvb_scratch("jjtvb_try_save_abort");
    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-seed", |_| {
        let mut g = zjjtvb_valid_gallops();
        g.next_heat_seed = "AA".to_string();
        g
    }, "seed".to_string())
    .unwrap();
    let after_seed = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);

    // The transform declines (a stale-precondition command handler).
    let result = jjdb_gallops_journal_try_save(&jjrfg_PlainGit, &config, "guidon-declines", |_tip| {
        Err::<(jjrg_Gallops, String), String>("precondition no longer holds on the tip".to_string())
    });
    match result {
        Err(jjdb_JournalReject::Abort(msg)) => assert!(msg.contains("precondition no longer holds")),
        other => panic!("a declining transform must abort with its own message, got {:?}", other),
    }

    // Nothing landed: the remote tip is exactly where the seed left it.
    assert_eq!(
        zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]),
        after_seed,
        "an aborted transform must land nothing on the remote"
    );

    // The lock was released — a following ceremony acquires it and succeeds.
    jjdb_gallops_journal_try_save(&jjrfg_PlainGit, &config, "guidon-after", |tip| {
        let mut g = tip.expect("seeded tenant").into_inner();
        g.next_heat_seed = "AC".to_string();
        Ok((g, "after abort".to_string()))
    })
    .expect("the lock must be free after an abort");
    assert_eq!(jjdb_gallops_journal_load(&config).unwrap().inner().next_heat_seed, "AC");
}

/// Seam-ON contention: a foreign station holding the lock makes the ceremony
/// refuse loud (Ceremony/LockHeld) and never run the transform ("contention ...
/// refuse loud").
#[test]
fn jjtvb_gallops_journal_try_save_refuses_loud_on_contention() {
    let (_bare, local, config) = zjjtvb_scratch("jjtvb_try_save_contention");
    jjrfg_PlainGit.jjrfr_stake(local.path(), "guidon-another-station").unwrap();

    let called = Cell::new(false);
    let result = jjdb_gallops_journal_try_save(&jjrfg_PlainGit, &config, "guidon-contender", |tip| {
        called.set(true);
        let g = tip.map(|v| v.into_inner()).unwrap_or_else(zjjtvb_valid_gallops);
        Ok((g, "should not land".to_string()))
    });

    match result {
        Err(jjdb_JournalReject::Ceremony(r)) => assert_eq!(r.kind, jjrfr_RejectionKind::LockHeld),
        other => panic!("a held lock must refuse loud as a Ceremony rejection, got {:?}", other),
    }
    assert!(!called.get(), "the transform must not run when the lock cannot be acquired");
}

/// Seam-ON message-from-transform: the commit subject carries the message the
/// transform RETURNS (derived from the locked tip), never a caller pre-composed
/// one — the guard against a minted-result message (a relocate's new coronet)
/// composing from a divergent pre-lock session read.
#[test]
fn jjtvb_gallops_journal_try_save_bakes_the_transform_returned_message() {
    let (bare, _local, config) = zjjtvb_scratch("jjtvb_try_save_message");
    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-seed", |_| {
        let mut g = zjjtvb_valid_gallops();
        g.next_heat_seed = "AA".to_string();
        g
    }, "seed".to_string())
    .unwrap();

    // The message is derived from the TIP the transform is handed, not passed in.
    jjdb_gallops_journal_try_save(&jjrfg_PlainGit, &config, "guidon-msg", |tip| {
        let g = tip.expect("seeded tenant").into_inner();
        let message = format!("derived from tip seed {}", g.next_heat_seed);
        Ok((g, message))
    })
    .unwrap();

    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--format=%s", ZJJTVB_TRUNK]);
    assert!(
        subject.contains("derived from tip seed AA"),
        "the commit subject must carry the transform-returned message, got '{}'",
        subject
    );
}

/// The read path is ref-read, not a working-tree read: `jjdb_gallops_journal_load`
/// resolves the pinned `origin/trunk` snapshot and reads the blob from its object
/// database, so a divergent (even corrupt) studbook working tree is invisible to
/// it. This is the `jjdk_lockless_reads` strengthening — a read touches only the
/// object database — and the guarantee the cutover leans on: the studbook working
/// tree is writer-only scratch a reader never sees.
#[test]
fn jjtvb_gallops_journal_load_reads_the_pin_never_the_working_tree() {
    let (_bare, local, config) = zjjtvb_scratch("jjtvb_gallops_pin_not_worktree");
    let gallops = zjjtvb_valid_gallops();
    jjdb_gallops_journal_save(&jjrfg_PlainGit, &config, "guidon-pin", |_| zjjtvb_valid_gallops(), "save gallops".to_string()).unwrap();

    // Corrupt the working-tree copy after the committed write. A working-tree
    // read would choke on this; the ref-read never looks at it.
    zjjtvb_write(local.path(), "gallops.json", "{ not even json");

    let loaded = jjdb_gallops_journal_load(&config).unwrap();
    assert_eq!(loaded.inner().next_heat_seed, gallops.next_heat_seed);
    assert!(loaded.inner().heats.is_empty());
}

#[test]
fn jjtvb_gallops_over_studbook_enablement_seam_is_live() {
    assert!(JJDB_GALLOPS_OVER_STUDBOOK_ENABLED, "the studbook-backed surface is live post-cutover (₣B3 founding-and-cutover); a revert to false would silently repoint every read and write back at the in-repo gallops");
}

// ---- Founding ceremony ----

fn zjjtvb_valid_gallops_seed() -> String {
    // Pretty-printed to match the canonical on-disk form jjdr_save always produces — a
    // compact seed only round-tripped through jjdr_load's canonical check by accident of
    // the now-stripped V3→V4 episode (empty heat_order also read as pre-V4 residue, so
    // migration mode stood the round-trip gate down as a side effect unrelated to formatting).
    serde_json::to_string_pretty(&zjjtvb_valid_gallops()).expect("a fresh Gallops must serialize")
}

#[test]
fn jjtvb_found_stands_a_fresh_instance_up_from_nothing_against_a_bare_remote() {
    let infield = JjkTestDir::new("jjtvb_found_infield");
    let bare = JjkTestDir::new("jjtvb_found_bare");
    zjjtvb_init_bare(bare.path());
    // local_root does not exist yet — jjdb_found must create it, exactly as
    // it would for a not-yet-founded instance's infield-resident path.
    let local_root = infield.path().join("scratch_studbook");
    let config = jjdb_BlotterConfig {
        local_root: local_root.clone(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    let sha = jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &zjjtvb_valid_gallops_seed());
        (vec![PathBuf::from("gallops.json")], "found".to_string())
    });

    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha, "the founding commit must land on the remote's trunk");
    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--pretty=%s", ZJJTVB_TRUNK]);
    assert_eq!(
        subject,
        zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING, "found"),
        "the genesis commit must take the store's founding ordinal, no zeroth special case"
    );
    assert!(local_root.join("gallops.json").exists());
}

#[test]
fn jjtvb_found_instance_is_immediately_ready_for_the_journal_ceremony() {
    let infield = JjkTestDir::new("jjtvb_found_ready_infield");
    let bare = JjkTestDir::new("jjtvb_found_ready_bare");
    zjjtvb_init_bare(bare.path());
    let config = jjdb_BlotterConfig {
        local_root: infield.path().join("scratch_studbook"),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &zjjtvb_valid_gallops_seed());
        (vec![PathBuf::from("gallops.json")], "found".to_string())
    });

    // The rehearsal proper (two-station lock contention, stale-lock break,
    // atomic-push lease failure, dispatch round-trip) is ₢BrAAW's own pace;
    // this asserts the founded instance is a normal, live blotter the
    // ordinary journal ceremony can already write through.
    let sha = jjdb_journal(&jjrfg_PlainGit, &config, "guidon-rehearsal", |root| {
        zjjtvb_write(root, "entry.txt", "post-founding entry");
        (vec![PathBuf::from("entry.txt")], "rehearsal entry".to_string())
    })
    .unwrap();

    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha);
    let subject = zjjtvb_git(bare.path(), &["log", "-1", "--pretty=%s", ZJJTVB_TRUNK]);
    assert_eq!(
        subject,
        zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 1, "rehearsal entry"),
        "the first journaled write after founding must advance one past the genesis ordinal"
    );
    let loaded = jjdb_gallops_journal_load(&config).unwrap();
    assert_eq!(loaded.inner().next_heat_seed, zjjtvb_valid_gallops().next_heat_seed);
}

#[test]
fn jjtvb_journal_bakes_a_monotonically_advancing_ordinal_across_writes() {
    let infield = JjkTestDir::new("jjtvb_ordinal_monotonic_infield");
    let bare = JjkTestDir::new("jjtvb_ordinal_monotonic_bare");
    zjjtvb_init_bare(bare.path());
    let config = jjdb_BlotterConfig {
        local_root: infield.path().join("scratch_studbook"),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    jjdb_found(&config, |root| {
        zjjtvb_write(root, "gallops.json", &zjjtvb_valid_gallops_seed());
        (vec![PathBuf::from("gallops.json")], "found".to_string())
    });

    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-first", |root| {
        zjjtvb_write(root, "first.txt", "first");
        (vec![PathBuf::from("first.txt")], "first entry".to_string())
    })
    .unwrap();
    jjdb_journal(&jjrfg_PlainGit, &config, "guidon-second", |root| {
        zjjtvb_write(root, "second.txt", "second");
        (vec![PathBuf::from("second.txt")], "second entry".to_string())
    })
    .unwrap();

    let subjects = zjjtvb_git(bare.path(), &["log", "--pretty=%s", "--reverse", ZJJTVB_TRUNK]);
    let lines: Vec<&str> = subjects.lines().collect();
    assert_eq!(
        lines,
        vec![
            zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING, "found"),
            zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 1, "first entry"),
            zjjtvb_baked_subject(JJDB_CATCHWORD_FOUNDING + 2, "second entry"),
        ],
        "each journaled write must advance the ordinal by exactly one past the genesis value"
    );
}

/// Found the REAL production `jjqs_studbook` against its real GitHub remote at
/// the station's real infield root — the env-var-driven integration path over
/// the SAME engine (`jjdb_found_studbook`) the operator's `jjx_found` door
/// composes, so exactly one writer and one seed byte-shape exist. The sanctioned
/// operator entrypoint is the door (the `jjw-bf` tabtarget); this is its
/// `cargo test` sibling for a scripted real found. Runs under the recreate-clean
/// ruling (₣B3 paddock, recorded 260719): the operator deletes and recreates the
/// bare remote and removes local clones first (the engine's own already-founded
/// guard refuses otherwise). Real network, real remote, not run by the ordinary
/// suite: `--ignored`, and both env vars must name their roots explicitly so a
/// stray `--include-ignored` sweep cannot land it anywhere by accident.
///
/// The sire is DERIVED by identifying the hippodrome (as the door does), never a
/// hand-typed constant — the seeded address is the key dispatch derives.
#[test]
#[ignore]
fn jjtvb_found_the_real_studbook_at_its_real_infield_root() {
    use super::jjrfr_farrier::jjrfr_FarrierCore;
    use super::jjrvb_blotter::{jjdb_found_studbook, jjdb_SireSeed};

    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT")
        .expect("set JJTVB_REAL_INFIELD_ROOT to the real infield root to run this ceremony");
    let hippodrome_root = std::env::var("JJTVB_REAL_HIPPODROME_ROOT")
        .expect("set JJTVB_REAL_HIPPODROME_ROOT to the hippodrome whose live gallops seeds the founding");
    let config = jjdb_studbook_config(Path::new(&infield_root));

    let live_path = Path::new(&hippodrome_root).join(".claude/jjm/jjg_gallops.json");
    let live_bytes = std::fs::read(&live_path)
        .unwrap_or_else(|e| panic!("could not read the live gallops at {}: {}", live_path.display(), e));
    let live = crate::jjri_io::jjdr_hark(&live_bytes).expect("the live gallops must validate before it can seed a founding");

    let identity = jjrfg_PlainGit
        .jjrfr_identify(Path::new(&hippodrome_root))
        .expect("the hippodrome must identify to seed the sire");
    let address = identity.upstream_key.expect("the hippodrome must have an origin remote to seed the sire");
    let sire = jjdb_SireSeed {
        kind: JJRDS_KIND_PLAIN_GIT.to_string(),
        address,
        trunk: "main".to_string(),
    };

    let sha = jjdb_found_studbook(&config, live.inner(), &sire).expect("the real founding must compose and land");

    println!("founded jjqs_studbook at {} ({})", sha, config.local_root.display());
}

// ---- Founding import (scratch — the collision-refusal demonstration) ----

/// A scratch heat under the given status: `₢`-keyed paces each carrying one
/// minimal tack, mirroring the stored shape.
fn zjjtvb_import_heat(heat_id: &str, status: jjrg_HeatStatus, pace_bodies: &[&str]) -> (String, jjrg_Heat) {
    let heat_key = format!("₣{}", heat_id);
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();
    for body in pace_bodies {
        let pace_key = format!("₢{}", body);
        paces.insert(
            pace_key.clone(),
            jjrg_Pace {
                tacks: vec![jjrg_Tack {
                    ts: "260719-1200".to_string(),
                    state: jjrg_PaceState::Rough,
                    tier: None,
                    effort: None,
                    text: vec!["scratch docket".to_string()],
                    silks: "scratch-pace".to_string(),
                    basis: "0000000".to_string(),
                }],
                ..Default::default()
            },
        );
        order.push(pace_key);
    }
    let heat = jjrg_Heat {
        silks: format!("scratch-heat-{}", heat_id),
        creation_time: "260719".to_string(),
        status,
        order,
        paces,
    };
    (heat_key, heat)
}

/// A scratch gallops from (key, heat) pairs, heat_order in the given sequence.
fn zjjtvb_import_gallops(heat_seed: &str, pace_seed: &str, heats_in: Vec<(String, jjrg_Heat)>) -> jjrg_Gallops {
    let mut heats = BTreeMap::new();
    let mut heat_order = Vec::new();
    for (k, h) in heats_in {
        heat_order.push(k.clone());
        heats.insert(k, h);
    }
    jjrg_Gallops {
        next_heat_seed: heat_seed.to_string(),
        next_pace_seed: pace_seed.to_string(),
        heat_order,
        heats,
        retention_since: None,
    }
}

/// The founding case (no target): racing and stabled heats ride, the retired
/// heat stays behind — dropped from both the heats map and heat_order — and
/// the source's own mint seeds carry through untouched (JJSAS: live state
/// only).
#[test]
fn jjtvb_founding_import_carries_live_state_only() {
    let source = zjjtvb_import_gallops(
        "AD",
        "CAAAD",
        vec![
            zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAA"]),
            zjjtvb_import_heat("AB", jjrg_HeatStatus::Stabled, &["CAAAB"]),
            zjjtvb_import_heat("AC", jjrg_HeatStatus::Retired, &["CAAAC"]),
        ],
    );

    let seed = jjdb_founding_import(&source, None).expect("a lone lineage must compose");

    assert_eq!(seed.heat_order, vec!["₣AA".to_string(), "₣AB".to_string()]);
    assert!(seed.heats.contains_key("₣AA") && seed.heats.contains_key("₣AB"));
    assert!(!seed.heats.contains_key("₣AC"), "a retired heat must stay behind");
    assert_eq!(seed.next_heat_seed, "AD");
    assert_eq!(seed.next_pace_seed, "CAAAD");
}

/// A firemark the target already holds refuses the import by name (JJSAS
/// multiproject import discipline: independent installs minted from
/// independent seeds, so cross-project imports can collide).
#[test]
fn jjtvb_founding_import_refuses_a_firemark_collision() {
    let target = zjjtvb_import_gallops(
        "AB",
        "CAAAB",
        vec![zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAA"])],
    );
    let source = zjjtvb_import_gallops(
        "AB",
        "CAAAC",
        vec![zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAB"])],
    );

    let err = jjdb_founding_import(&source, Some(&target)).expect_err("a colliding firemark must refuse");
    assert!(err.contains("firemark ₣AA"), "the refusal must name the colliding firemark, got: {}", err);
}

/// A coronet collision refuses even across distinct firemarks — coronets are
/// flat global ids, so the same body under two heats is exactly the 260719
/// live specimen (one coronet independently minted by two clones, surfacing
/// only at convergence).
#[test]
fn jjtvb_founding_import_refuses_a_coronet_collision_across_firemarks() {
    let target = zjjtvb_import_gallops(
        "AB",
        "CAAAG",
        vec![zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAF"])],
    );
    let source = zjjtvb_import_gallops(
        "AC",
        "CAAAG",
        vec![zjjtvb_import_heat("AB", jjrg_HeatStatus::Racing, &["CAAAF"])],
    );

    let err = jjdb_founding_import(&source, Some(&target)).expect_err("a colliding coronet must refuse");
    assert!(err.contains("coronet ₢CAAAF"), "the refusal must name the colliding coronet, got: {}", err);
}

/// The clean-merge case: disjoint lineages append in order, the mint seeds
/// take the elementwise maximum (here each side is ahead on a different seed),
/// and the live-state filter runs BEFORE the collision check — a retired
/// source heat whose key collides with the target neither rides nor refuses.
#[test]
fn jjtvb_founding_import_merges_disjoint_lineages_with_seed_maximum() {
    let target = zjjtvb_import_gallops(
        "AC",
        "CAAAD",
        vec![zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAA"])],
    );
    let source = zjjtvb_import_gallops(
        "AB",
        "CAAAZ",
        vec![
            zjjtvb_import_heat("AB", jjrg_HeatStatus::Racing, &["CAAAB"]),
            zjjtvb_import_heat("AA", jjrg_HeatStatus::Retired, &["CAAAC"]),
        ],
    );

    let merged = jjdb_founding_import(&source, Some(&target)).expect("disjoint live lineages must merge");

    assert_eq!(merged.heat_order, vec!["₣AA".to_string(), "₣AB".to_string()]);
    assert_eq!(
        merged.heats["₣AA"].silks, "scratch-heat-AA",
        "the target's own ₣AA must survive; the source's retired ₣AA neither rides nor refuses"
    );
    assert!(merged.heats["₣AA"].paces.contains_key("₢CAAAA"), "the target's pace stands");
    assert!(!merged.heats["₣AA"].paces.contains_key("₢CAAAC"), "the retired source heat's pace stays behind");
    assert_eq!(merged.next_heat_seed, "AC", "heat seed takes the target side (the later)");
    assert_eq!(merged.next_pace_seed, "CAAAZ", "pace seed takes the source side (the later)");
}

/// The ₢BrAAW rehearsal's pedigree payload — one sire for the recipemuster
/// hippodrome — composed through the production writer (`jjrds_seed_pedigrees_json`)
/// so this rehearsal backfill and the founding share the one seed byte-shape.
/// The literal address is this frozen rehearsal artifact's, targeting the
/// specific standing studbook; the founding path derives its sire from the
/// hippodrome's identity instead. This ignored rehearsal set is retargeted or
/// retired at cutover (₣B3 paddock).
fn zjjtvb_real_pedigrees_json() -> String {
    crate::jjrds_spine::jjrds_seed_pedigrees_json(vec![crate::jjrds_spine::jjrds_Pedigree {
        kind: JJRDS_KIND_PLAIN_GIT.to_string(),
        addresses: vec!["git@github.com:bhyslop/recipemuster".to_string()],
        trunk: "main".to_string(),
    }])
    .expect("the rehearsal seed pedigree must serialize")
}

/// Rehearsal, and the substrate proof every other rehearsal item stands on
/// (₢BrAAW): carry the seed pedigree onto the REAL standing studbook through
/// the full journal ceremony, against the real GitHub remote — the ₢BrAAU
/// founding seeded `gallops.json` alone, and the remote cannot be emptied for
/// a re-found without leaving plain git (GitHub refuses to delete a default
/// branch), so the correction arrives as a journaled write.
///
/// One write exercises the whole bracket where no scratch bare repo can reach
/// it: stake's compare-and-swap on the custom `refs/jjv/guidon` ref, sight's
/// ls-remote-then-fetch read-back, advance, the atomic two-ref proffer
/// under lease, the ordinal bake, and pluck — all against a real server whose
/// receive-pack is a Palisade we do not govern. A refusal here (a rejected ref
/// namespace, an unsupported `--atomic` with `--force-with-lease`) is the
/// finding, not a failure to route around.
///
/// Idempotent by refusal: it asserts the pedigree is absent first, so a second
/// run says so plainly instead of journaling a no-op commit.
#[test]
#[ignore]
fn jjtvb_journal_the_real_pedigree_onto_the_standing_studbook() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT")
        .expect("set JJTVB_REAL_INFIELD_ROOT to the real infield root to run this rehearsal");
    let config = jjdb_studbook_config(Path::new(&infield_root));
    let farrier = jjrfg_PlainGit;

    assert!(
        jjdb_read(&config, Path::new(JJRDS_PEDIGREES_REL_PATH)).is_err(),
        "the standing studbook already carries a pedigrees file — this write has already run"
    );

    let pedigrees_json = zjjtvb_real_pedigrees_json();
    let guidon = crate::jjrvg_guidon::jjdb_guidon_compose(
        "₢BrAAW-rehearsal",
        &crate::jjrvg_guidon::jjdb_station_name(),
        chrono::Utc::now(),
        "journal-pedigree",
    );

    let sha = jjdb_journal(&farrier, &config, &guidon, |root| {
        zjjtvb_write(root, JJRDS_PEDIGREES_REL_PATH, &pedigrees_json);
        (vec![PathBuf::from(JJRDS_PEDIGREES_REL_PATH)], "seed the station pedigree".to_string())
    })
    .expect("the journal ceremony must carry the pedigree onto the real remote");

    // The lock must be released, and the content must be live on the remote —
    // consign is atomic, so a local commit alone would be a lie.
    assert_eq!(farrier.jjrfr_sight(&config.local_root).unwrap(), None, "the guidon must be plucked on the way out");
    println!("journaled the station pedigree onto jjqs_studbook at {}", sha);
}

// ---- Two-station rehearsal against the real remote (₢BrAAW) ----
//
// Every scenario below stands up its OWN clones of the real studbook remote
// and never touches the station's standing clone: a station is exactly a clone,
// so two temp clones are two honest stations, and the real remote stays the
// sole arbiter — which is the whole point. The in-process lock registry cannot
// be what refuses anything here (it keys on root, and these roots differ), so a
// LockHeld can only have come off GitHub's own compare-and-swap.
//
// These share one real remote lock, so they MUST run single-threaded
// (`--test-threads=1`); run in parallel they would contend with each other
// rather than with the scenario under test.

/// A per-run mark for every byte a rehearsal writes. The real studbook is a real
/// store and it ACCUMULATES — the scenarios below are re-run against a trunk that
/// already holds what their last run left. Fixed content would therefore write
/// NOTHING NEW on a second run: advance carries the clone to the remote tip
/// (JJr_b52), and the file is already there byte-identical. Found the honest
/// way — under the retired lodge-based ceremony the first re-run died in
/// lodge, on content its own previous run had put there.
fn zjjtvb_run_mark() -> String {
    chrono::Utc::now().format("%Y%m%dT%H%M%S%.6fZ").to_string()
}

/// Stand a rehearsal station up: a fresh clone of the REAL studbook remote in a
/// temp dir, plus the blotter config pointed at it.
fn zjjtvb_real_station(infield_root: &str, name: &str) -> (JjkTestDir, jjdb_BlotterConfig) {
    let real = jjdb_studbook_config(Path::new(infield_root));
    let dir = JjkTestDir::new(name);
    zjjtvb_git(
        Path::new("/"),
        &["clone", "-q", "-b", &real.trunk, &real.remote_url, &dir.path().to_string_lossy()],
    );
    zjjtvb_git(dir.path(), &["config", "user.email", "jjtvb@example.invalid"]);
    zjjtvb_git(dir.path(), &["config", "user.name", "jjtvb-rehearsal"]);
    let config = jjdb_BlotterConfig { local_root: dir.path().to_path_buf(), ..real };
    (dir, config)
}

/// Every rehearsal below stakes the ONE real lock, so each must begin against a
/// free one — and a scenario that panics mid-ceremony leaves it flying (we
/// learned this the honest way: a harness bug stranded a lock on the real
/// remote and every later scenario then read a genuine LockHeld). Refuse early
/// and name the remedy rather than let a leftover lock masquerade as the
/// contention under test.
fn zjjtvb_require_free_lock<F: jjrfr_FarrierLock>(farrier: &F, station: &jjdb_BlotterConfig) {
    if let Some(held) = farrier.jjrfr_sight(&station.local_root).unwrap() {
        panic!(
            "the real studbook's lock is already held ({:?}) — a prior run left it flying. \
             Clear it at the operator's door: tt/jjw-dc.SightLocks.sh to see it, \
             tt/jjw-dC.Cashier.sh to cashier it (JJSVD jjdd_cashier).",
            held
        );
    }
}

/// Rehearsal 1 — two-station lock contention. Alpha holds the lock; bravo, a
/// wholly separate clone, attempts its own ceremony and must be refused by the
/// remote's compare-and-swap, with its mutate never run.
#[test]
#[ignore]
fn jjtvb_rehearsal_two_station_lock_contention() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT").expect("set JJTVB_REAL_INFIELD_ROOT");
    let farrier = jjrfg_PlainGit;
    let (_alpha_dir, alpha) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_contention_alpha");
    let (_bravo_dir, bravo) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_contention_bravo");
    zjjtvb_require_free_lock(&farrier, &alpha);

    farrier.jjrfr_stake(&alpha.local_root, "station=alpha op=rehearsal-contention").unwrap();

    let called = Cell::new(false);
    let result = jjdb_journal(&farrier, &bravo, "station=bravo op=rehearsal-contention", |root| {
        called.set(true);
        zjjtvb_write(root, "bravo-must-not-land.txt", "x");
        (vec![PathBuf::from("bravo-must-not-land.txt")], "bravo must not land".to_string())
    });

    let kind = result.expect_err("bravo must be refused while alpha holds the lock").kind;
    assert_eq!(kind, jjrfr_RejectionKind::LockHeld, "the refusal must name the lock, not some transport accident");
    assert!(!called.get(), "bravo's mutate must never run: the lock gates the write, not the push");

    // Alpha, still the rightful holder, releases.
    farrier.jjrfr_pluck(&alpha.local_root, "station=alpha op=rehearsal-contention").unwrap();
    assert_eq!(farrier.jjrfr_sight(&alpha.local_root).unwrap(), None);
    println!("REHEARSAL contention: bravo refused with LockHeld off the real remote; mutate never ran");
}

/// Rehearsal 2 — stale-lock break. Alpha stakes and then vanishes (a staked
/// lock with no live guard IS the crashed station: that is precisely the state a
/// killed process leaves behind). Bravo sights the foreign guidon, breaks it,
/// and completes its own ceremony.
#[test]
#[ignore]
fn jjtvb_rehearsal_stale_lock_break() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT").expect("set JJTVB_REAL_INFIELD_ROOT");
    let farrier = jjrfg_PlainGit;
    let (_alpha_dir, alpha) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_break_alpha");
    let (_bravo_dir, bravo) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_break_bravo");
    zjjtvb_require_free_lock(&farrier, &alpha);

    let abandoned = "station=alpha op=rehearsal-break state=crashed";
    farrier.jjrfr_stake(&alpha.local_root, abandoned).unwrap();

    // Bravo sees a lock it does not own, and can read WHO holds it — the whole
    // reason a guidon carries text rather than a bare flag.
    let sighted = farrier.jjrfr_sight(&bravo.local_root).unwrap();
    assert_eq!(sighted.as_deref(), Some(abandoned), "bravo must read the abandoned holder's mark");
    assert_eq!(
        jjdb_journal(&farrier, &bravo, "station=bravo op=blocked", |_| unreachable!("mutate must not run"))
            .expect_err("a stale lock blocks bravo exactly as a live one does")
            .kind,
        jjrfr_RejectionKind::LockHeld,
        "a stale lock is indistinguishable from a held one — which is why the break is a deliberate operator act"
    );

    let broken = jjrfr_break(&farrier, &bravo.local_root).unwrap();
    assert_eq!(broken.as_deref(), Some(abandoned), "the break must report whose lock it cleared");

    let mark = zjjtvb_run_mark();
    let sha = jjdb_journal(&farrier, &bravo, "station=bravo op=rehearsal-break", |root| {
        zjjtvb_write(root, "rehearsal.txt", &format!("bravo wrote this after breaking a stale lock ({})", mark));
        (vec![PathBuf::from("rehearsal.txt")], "rehearsal: bravo writes after a stale-lock break".to_string())
    })
    .expect("after the break, bravo's ceremony must complete");

    assert_eq!(farrier.jjrfr_sight(&bravo.local_root).unwrap(), None, "bravo must release on the way out");
    println!("REHEARSAL break: bravo read alpha's abandoned guidon, broke it, and journaled at {}", sha);
}

/// Rehearsal 3 — atomic-push lease failure, the invariant the whole design
/// turns on. Bravo breaks alpha's lock while alpha is mid-ceremony; alpha's
/// consign must fail its lease and land NOTHING on the remote — no half-applied
/// write, no content pushed without the lock that authorized it.
#[test]
#[ignore]
fn jjtvb_rehearsal_atomic_push_lease_failure() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT").expect("set JJTVB_REAL_INFIELD_ROOT");
    let farrier = jjrfg_PlainGit;
    let (_alpha_dir, alpha) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_lease_alpha");
    let (bravo_dir, bravo) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_lease_bravo");
    zjjtvb_require_free_lock(&farrier, &alpha);

    let baseline = zjjtvb_git(&alpha.local_root, &["rev-parse", &format!("origin/{}", alpha.trunk)]);

    let usurper = "station=bravo op=usurper";
    let mark = zjjtvb_run_mark();
    let result = jjdb_journal(&farrier, &alpha, "station=alpha op=rehearsal-lease", |root| {
        // Bravo — a different clone, i.e. a different machine — breaks alpha's
        // lock and takes it, while alpha sits between sight and consign.
        let cleared = jjrfr_break(&farrier, bravo_dir.path()).unwrap();
        assert!(cleared.is_some(), "bravo must find alpha's lock to break");
        farrier.jjrfr_stake(bravo_dir.path(), usurper).unwrap();
        zjjtvb_write(root, "stranded.txt", &format!("refused by the lock, never authorized ({})", mark));
        (vec![PathBuf::from("stranded.txt")], "must never reach the remote".to_string())
    });

    assert_eq!(
        result.expect_err("alpha's consign must fail once its lock is gone").kind,
        jjrfr_RejectionKind::LockBroken,
        "the refusal must name the broken lock, not a content race"
    );

    // The invariant: the remote is untouched, and alpha's refused commit never
    // rode its local branch — nothing it wrote reached the shared store.
    let remote_tip = zjjtvb_git(&alpha.local_root, &["ls-remote", "origin", &alpha.trunk]);
    let remote_sha = remote_tip.split_whitespace().next().unwrap();
    assert_eq!(remote_sha, baseline, "a lease-failed consign must land NOTHING on the real remote");
    assert_eq!(
        farrier.jjrfr_sight(&bravo.local_root).unwrap().as_deref(),
        Some(usurper),
        "the usurper's lock must survive alpha's failed consign and alpha's own guard release"
    );

    farrier.jjrfr_pluck(&bravo.local_root, usurper).unwrap();
    println!("REHEARSAL lease: alpha's consign was refused LockBroken; real remote still at {}", baseline);
}

/// Rehearsal 3b — the aftermath, against the real remote. This probe once ran as
/// a finding-recorder and found one: a lease-failed ceremony stranded its commit
/// locally, and the station's next authorized ceremony pushed it onto the shared
/// store, unauthorized. JJr_b52 is the answer that finding bought — first as
/// equalize-and-retrench, now as compose-then-push: a refused proffer never puts
/// the commit on the branch at all — and this probe asserts the invariant where
/// the finding was made, against real GitHub rather than a scratch bare repo.
/// Ignored and env-gated like its rehearsal siblings, so it does not run in the
/// standing suite; the every-run guard is the scratch sibling
/// `jjtvb_journal_refused_proffer_leaves_branch_and_record_untouched`.
#[test]
#[ignore]
fn jjtvb_rehearsal_stranded_commit_aftermath() {
    let infield_root = std::env::var("JJTVB_REAL_INFIELD_ROOT").expect("set JJTVB_REAL_INFIELD_ROOT");
    let farrier = jjrfg_PlainGit;
    let (alpha_dir, alpha) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_aftermath_alpha");
    let (bravo_dir, _bravo) = zjjtvb_real_station(&infield_root, "jjtvb_rehearsal_aftermath_bravo");
    zjjtvb_require_free_lock(&farrier, &alpha);

    // The store's tip BEFORE this run wrote anything. The assertion below reads
    // only what this run pushed: the remote's standing history already carries a
    // STRANDED commit — the one the original defect pushed, before JJr_b52 — and
    // a whole-history scan would read that old evidence as a fresh violation
    // forever. What the invariant claims is that no commit THIS run's lock
    // refused reached the store.
    let baseline = zjjtvb_git(alpha_dir.path(), &["rev-parse", &format!("origin/{}", alpha.trunk)]);

    // Drive alpha into the stranded state exactly as rehearsal 3 does.
    let usurper = "station=bravo op=aftermath-usurper";
    let mark = zjjtvb_run_mark();
    let stranded_subject = format!("STRANDED {}: refused by the lock", mark);
    let err = jjdb_journal(&farrier, &alpha, "station=alpha op=aftermath", |root| {
        jjrfr_break(&farrier, bravo_dir.path()).unwrap();
        farrier.jjrfr_stake(bravo_dir.path(), usurper).unwrap();
        zjjtvb_write(root, "stranded.txt", &format!("refused by the lock, never authorized ({})", mark));
        (vec![PathBuf::from("stranded.txt")], stranded_subject.clone())
    })
    .expect_err("the lease must fail");
    assert_eq!(err.kind, jjrfr_RejectionKind::LockBroken);
    farrier.jjrfr_pluck(bravo_dir.path(), usurper).unwrap();

    // Alpha now re-journals something entirely unrelated, with the lock free.
    let sha = jjdb_journal(&farrier, &alpha, "station=alpha op=aftermath-retry", |root| {
        zjjtvb_write(root, "aftermath.txt", &format!("the write alpha actually intended ({})", mark));
        (vec![PathBuf::from("aftermath.txt")], "rehearsal: alpha's next authorized write".to_string())
    })
    .expect("with the lock free, alpha's next ceremony completes");

    // What reached the remote? Read back exactly what this run pushed and look
    // for the stranded subject riding along.
    let pushed = zjjtvb_git(
        alpha_dir.path(),
        &["log", "--pretty=%s", &format!("{}..origin/{}", baseline, alpha.trunk)],
    );
    let stranded_landed = pushed.lines().any(|l| l.contains(&stranded_subject));
    println!("REHEARSAL aftermath: alpha's retry landed at {}", sha);
    println!("REHEARSAL aftermath: stranded commit reached the remote? {}", stranded_landed);
    println!("REHEARSAL aftermath: this run pushed:\n{}", pushed);
    assert!(
        !stranded_landed,
        "JJr_b52 violated on the real remote: a commit the lock REFUSED rode onto the shared store \
         on the station's next write. A refused proffer must leave the local branch untouched, so \
         no refused commit can exist for a later ceremony to carry."
    );
}

// ---- Founding orchestrator (the studbook founding ceremony, fixture-only) ----

/// The founding orchestrator end-to-end against a SCRATCH studbook: one
/// deterministic found+import+seed act stands a studbook up from nothing,
/// seeding BOTH tenants in the genesis commit — the imported live-state gallops
/// (retired filtered out, proving the import ran rather than a raw copy) and the
/// sire pedigree (the founding-seed gap `jjrds_plan` reads first) — such that the
/// dispatch reader resolves the seeded sire. The real found against the real
/// remote is the cutover ceremony's own act (JJSAS Founding-and-cutover,
/// recreate-clean ruling); the collision refusal the import carries is exercised
/// on its own above (`jjtvb_founding_import_refuses_*`), unreachable on this
/// target-none founding path.
#[test]
fn jjtvb_found_studbook_seeds_both_tenants_and_dispatch_resolves_the_sire() {
    use super::jjrds_spine::jjrds_pedigree_lookup;
    use super::jjrvb_blotter::{jjdb_found_studbook, jjdb_gallops_journal_load, jjdb_SireSeed};

    let infield = JjkTestDir::new("jjtvb_found_studbook_infield");
    let bare = JjkTestDir::new("jjtvb_found_studbook_bare");
    zjjtvb_init_bare(bare.path());
    let config = jjdb_BlotterConfig {
        local_root: infield.path().join("jjqs_studbook"),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };

    // Live in-repo state to import: racing and stabled ride, retired stays behind.
    let live = zjjtvb_import_gallops(
        "AD",
        "CAAAD",
        vec![
            zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAA"]),
            zjjtvb_import_heat("AB", jjrg_HeatStatus::Stabled, &["CAAAB"]),
            zjjtvb_import_heat("AC", jjrg_HeatStatus::Retired, &["CAAAC"]),
        ],
    );

    const SCRATCH_SIRE: &str = "git@github.com:bhyslop/scratch-hippodrome";
    let sire = jjdb_SireSeed {
        kind: JJRDS_KIND_PLAIN_GIT.to_string(),
        address: SCRATCH_SIRE.to_string(),
        trunk: "main".to_string(),
    };

    let sha = jjdb_found_studbook(&config, &live, &sire).expect("the founding must compose and land");

    // One genesis commit on the remote carries BOTH tenants.
    let remote_tip = zjjtvb_git(bare.path(), &["rev-parse", ZJJTVB_TRUNK]);
    assert_eq!(remote_tip, sha, "the founding commit must land on the remote's trunk");
    let files = zjjtvb_git(bare.path(), &["show", "--name-only", "--pretty=format:", ZJJTVB_TRUNK]);
    let names: Vec<&str> = files.lines().filter(|l| !l.is_empty()).collect();
    assert!(names.contains(&"gallops.json"), "the genesis commit must seed gallops.json, got: {:?}", names);
    assert!(
        names.contains(&JJRDS_PEDIGREES_REL_PATH),
        "the genesis commit must seed pedigrees.json (the founding-seed gap), got: {:?}",
        names
    );

    // The seeded gallops is the live-state IMPORT, not a raw copy: retired gone,
    // racing and stabled ride, the source's mint seeds carry through.
    let founded = jjdb_gallops_journal_load(&config).expect("the founded gallops must read back through the production path");
    assert_eq!(founded.inner().heat_order, vec!["₣AA".to_string(), "₣AB".to_string()]);
    assert!(!founded.inner().heats.contains_key("₣AC"), "the retired heat must stay behind (live-state import)");
    assert_eq!(founded.inner().next_heat_seed, "AD");
    assert_eq!(founded.inner().next_pace_seed, "CAAAD");

    // The dispatch reader resolves the seeded sire — the founded store is
    // servable (a station without pedigrees.json could not dispatch at all).
    let pedigree = jjrds_pedigree_lookup(&config, SCRATCH_SIRE, JJRDS_KIND_PLAIN_GIT)
        .expect("dispatch must resolve the seeded sire from the founded studbook");
    assert_eq!(pedigree.kind, JJRDS_KIND_PLAIN_GIT);
    assert_eq!(pedigree.addresses, vec![SCRATCH_SIRE.to_string()]);
    assert_eq!(pedigree.trunk, "main");
}

/// The already-founded guard: founding refuses a studbook clone that already
/// stands rather than re-init'ing it and clobbering its tenants. The refusal
/// fires before any git touch — the remote here is unreachable and never
/// reached — so recreate-clean is enforced even under a confirm-skipped run.
#[test]
fn jjtvb_found_studbook_refuses_a_clone_that_already_stands() {
    use super::jjrvb_blotter::{jjdb_found_studbook, jjdb_SireSeed};

    let infield = JjkTestDir::new("jjtvb_found_studbook_stands_infield");
    let local_root = infield.path().join("jjqs_studbook");
    std::fs::create_dir_all(&local_root).unwrap();
    let config = jjdb_BlotterConfig {
        local_root,
        remote_url: "unreachable://never-touched".to_string(),
        trunk: ZJJTVB_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    let live = zjjtvb_import_gallops("AB", "CAAAB", vec![zjjtvb_import_heat("AA", jjrg_HeatStatus::Racing, &["CAAAA"])]);
    let sire = jjdb_SireSeed {
        kind: JJRDS_KIND_PLAIN_GIT.to_string(),
        address: "git@github.com:bhyslop/scratch-hippodrome".to_string(),
        trunk: "main".to_string(),
    };

    let err = jjdb_found_studbook(&config, &live, &sire).expect_err("founding must refuse a standing clone");
    assert!(err.contains("already stands"), "the refusal must name the standing clone, got: {}", err);
}

// ============================================================================
// Per-command seam-ON proofs (the CAAAv coverage-gap closure)
// ============================================================================
// The mechanism-level ceremony tests above prove the shared journal core; these
// prove each mutating command's OWN seam-ON path against a fixture studbook + a
// real temp consumer repo — the write paths that were compile-checked but had
// never executed (memo-20260720-journal-writes-pace-retrospective; nominate
// closed separately as CAAAw, the CAAAv census's cutover-blocking find). The
// jjri_persist five (slate/relabel/drop/rail/furlough) ride the shared
// `zjjrm_write_gallops_over` with each command's own mutation closure; the
// machine_commit five (retire/nominate/draft/restring/wrap) drive their
// extracted `_over` on-branch. The seam const stays false throughout —
// reachability comes from the explicit `over_studbook`/studbook parameters,
// never a flip.

/// Serializes every proving-ground test. A REAL `vvcc_CommitLock` is acquired
/// (the cinch: no test-only constructor on the production lock — the
/// interface-contamination guard), and both the acquire and retire's consumer
/// `machine_commit` ride the PROCESS cwd, which a ground repoints to its temp
/// consumer repo — so two grounds must never overlap in the parallel runner.
static ZJJTVB_GROUND_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// A per-command seam-ON proving ground: a founded scratch studbook seeded with
/// `seed` beside a real temp consumer repo, with the process cwd pointed at that
/// consumer repo (so a real commit lock acquires against it and any consumer-side
/// commit lands there). Never the real studbook remote — the config points at
/// scratch dirs alone (the write-seam pace's standing cinch). The serialization
/// guard drops last, after cwd is restored.
struct ZjjtvbGround {
    consumer: JjkTestDir,
    _bare: JjkTestDir,
    _local: JjkTestDir,
    studbook: jjdb_BlotterConfig,
    prior_cwd: PathBuf,
    _serial: std::sync::MutexGuard<'static, ()>,
}

impl ZjjtvbGround {
    fn new(name: &str, seed: jjrg_Gallops) -> Self {
        let serial = ZJJTVB_GROUND_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        // Scratch studbook (bare + clone), seeded with the caller's gallops through
        // the journal ceremony so the tenant lands exactly where a live journal reads
        // it. Path-based (git -C), so cwd-independent — built before the cwd hop.
        let (bare, local, studbook) = zjjtvb_scratch(&format!("{}_sb", name));
        jjdb_gallops_journal_save(&jjrfg_PlainGit, &studbook, "guidon-seed", move |_| seed, "seed".to_string())
            .expect("seeding the fixture studbook must land the gallops tenant");
        // Temp consumer repo with a base commit so HEAD exists for the lock ref.
        let consumer = JjkTestDir::new(&format!("{}_consumer", name));
        zjjtvb_init_local(consumer.path());
        zjjtvb_commit_all(consumer.path(), "base.txt", "base", "init");
        let prior_cwd = std::env::current_dir().expect("a cwd to restore");
        std::env::set_current_dir(consumer.path()).expect("point the process cwd at the consumer repo");
        ZjjtvbGround { consumer, _bare: bare, _local: local, studbook, prior_cwd, _serial: serial }
    }

    fn studbook(&self) -> &jjdb_BlotterConfig {
        &self.studbook
    }

    fn consumer_path(&self) -> &Path {
        self.consumer.path()
    }

    /// Acquire a REAL commit lock against the consumer repo (the process cwd),
    /// exactly as a live command does — the jjri_persist family's on path takes it
    /// (unused on the studbook branch, but real, per the cinch).
    fn lock(&self) -> vvc::vvcc_CommitLock {
        vvc::vvcc_CommitLock::vvcc_acquire().expect("the fixture consumer repo must yield the commit lock")
    }
}

impl Drop for ZjjtvbGround {
    fn drop(&mut self) {
        // Restore cwd here (runs before the fields drop), while the serialization
        // guard is still held — any lock a test acquired has already released
        // against this still-current cwd.
        let _ = std::env::set_current_dir(&self.prior_cwd);
    }
}

/// One tack, Rough, for a seed pace.
fn zjjtvb_seed_pace(heat_id: &str, pace_suffix: &str) -> (String, jjrg_Pace) {
    let pace_key = format!("\u{20A2}{}{}", heat_id, pace_suffix);
    let tack = jjrg_Tack {
        ts: "260101-1200".to_string(),
        state: jjrg_PaceState::Rough,
        tier: None,
        effort: None,
        text: vec!["docket".to_string()],
        silks: format!("pace-{}{}", heat_id, pace_suffix),
        basis: JJRG_UNKNOWN_BASIS.to_string(),
    };
    (pace_key, jjrg_Pace { tacks: vec![tack], ..Default::default() })
}

/// A racing/stabled heat with the given pace suffixes (coronet body = heat_id +
/// suffix), in order.
fn zjjtvb_seed_heat(heat_id: &str, status: jjrg_HeatStatus, pace_suffixes: &[&str]) -> (String, jjrg_Heat) {
    let heat_key = format!("\u{20A3}{}", heat_id);
    let mut paces = BTreeMap::new();
    let mut order = Vec::new();
    for s in pace_suffixes {
        let (pk, pace) = zjjtvb_seed_pace(heat_id, s);
        order.push(pk.clone());
        paces.insert(pk, pace);
    }
    (heat_key, jjrg_Heat {
        silks: format!("heat-{}", heat_id),
        creation_time: "260101".to_string(),
        status,
        order,
        paces,
    })
}

/// A gallops carrying the given heats (heat_order tracks them), with mint seeds
/// clear of the fixtures' own ids.
fn zjjtvb_seed_gallops(heats: Vec<(String, jjrg_Heat)>) -> jjrg_Gallops {
    let mut heat_order = Vec::new();
    let mut heat_map = BTreeMap::new();
    for (k, h) in heats {
        heat_order.push(k.clone());
        heat_map.insert(k, h);
    }
    jjrg_Gallops {
        next_heat_seed: "AB".to_string(),
        next_pace_seed: "CAAAA".to_string(),
        heat_order,
        heats: heat_map,
        retention_since: None,
    }
}

/// Drive the jjri_persist family's seam-ON path: `zjjrm_write_gallops_over` with
/// `over_studbook = true` and the command's own mutation closure, against the
/// ground's fixture studbook. A real lock is acquired (unused on this branch, per
/// the cinch) and released when this returns; `path`/`firemark`/`size_limit`/
/// `session_gallops` are the branch's ignored args.
fn zjjtvb_drive_write_over<R>(
    ground: &ZjjtvbGround,
    firemark: &str,
    message: &str,
    mutate: impl FnOnce(&mut jjrg_Gallops) -> Result<R, String>,
) -> Result<(R, String), crate::jjrm_mcp::zjjrm_WriteRefusal> {
    let lock = ground.lock();
    let mut output = vvc::vvco_Output::buffer();
    let fm = jjrf_Firemark::jjrf_parse(firemark).expect("valid fixture firemark");
    crate::jjrm_mcp::zjjrm_write_gallops_over(
        &jjrfg_PlainGit,
        true,
        &lock,
        Path::new("unused-on-the-studbook-path"),
        &fm,
        message.to_string(),
        vvc::VVCG_SIZE_LIMIT,
        &mut output,
        ground.studbook(),
        "guidon-command",
        zjjtvb_valid_gallops(),
        mutate,
    )
}

// ---- jjri_persist family (B1: the shared over-seam with each command's closure) ----

#[test]
fn jjtvb_seam_on_slate_journals_the_new_pace() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_slate",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZS", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let slate_args = jjrg_SlateArgs {
        firemark: "ZS".to_string(),
        silks: "slated-pace".to_string(),
        text: "docket".to_string(),
        dictation: None,
        precis: None,
        slated: "260101-1200".to_string(),
        before: None,
        after: None,
        first: false,
    };
    let res = zjjtvb_drive_write_over(&ground, "ZS", "slate msg", |g| g.jjrg_slate(slate_args));
    assert!(res.is_ok(), "slate must journal to the studbook");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    assert_eq!(
        loaded.inner().heats.get("\u{20A3}ZS").unwrap().order.len(),
        2,
        "slate must add a pace to the studbook tip"
    );
}

#[test]
fn jjtvb_seam_on_relabel_journals_the_new_silks() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_relabel",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZR", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let tally_args = jjrg_TallyArgs {
        coronet: "ZRAAA".to_string(),
        state: None,
        text: None,
        silks: Some("relabeled".to_string()),
    };
    let res = zjjtvb_drive_write_over(&ground, "ZR", "relabel msg", |g| g.jjrg_tally(tally_args));
    assert!(res.is_ok(), "relabel must journal to the studbook");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    let pace = loaded.inner().heats.get("\u{20A3}ZR").unwrap().paces.get("\u{20A2}ZRAAA").unwrap();
    assert!(
        pace.tacks.iter().any(|t| t.silks == "relabeled"),
        "relabel must journal the new silks to the studbook tip"
    );
}

#[test]
fn jjtvb_seam_on_drop_journals_the_abandonment() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_drop",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZD", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let tally_args = jjrg_TallyArgs {
        coronet: "ZDAAA".to_string(),
        state: Some(jjrg_PaceState::Abandoned),
        text: None,
        silks: None,
    };
    let res = zjjtvb_drive_write_over(&ground, "ZD", "drop msg", |g| g.jjrg_tally(tally_args));
    assert!(res.is_ok(), "drop must journal to the studbook");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    let pace = loaded.inner().heats.get("\u{20A3}ZD").unwrap().paces.get("\u{20A2}ZDAAA").unwrap();
    assert!(
        pace.tacks.iter().any(|t| matches!(t.state, jjrg_PaceState::Abandoned)),
        "drop must journal the Abandoned state to the studbook tip"
    );
}

#[test]
fn jjtvb_seam_on_rail_journals_the_reorder() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_rail",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZL", jjrg_HeatStatus::Racing, &["AAA", "AAB"])]),
    );
    // Move mode (order mode is retired): move the second pace to first, which
    // swaps the two-pace sequence.
    let rail_args = jjrg_RailArgs {
        firemark: "ZL".to_string(),
        order: vec![],
        move_coronet: Some("ZLAAB".to_string()),
        before: None,
        after: None,
        first: true,
        last: false,
    };
    let res = zjjtvb_drive_write_over(&ground, "ZL", "rail msg", |g| g.jjrg_rail(rail_args));
    assert!(res.is_ok(), "rail must journal to the studbook");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    assert_eq!(
        loaded.inner().heats.get("\u{20A3}ZL").unwrap().order,
        vec!["\u{20A2}ZLAAB".to_string(), "\u{20A2}ZLAAA".to_string()],
        "rail must journal the reordered sequence to the studbook tip"
    );
}

#[test]
fn jjtvb_seam_on_furlough_journals_the_status_change() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_furlough",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZF", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let furlough_args = jjrg_FurloughArgs {
        firemark: "ZF".to_string(),
        racing: false,
        stabled: true,
        silks: None,
    };
    let res = zjjtvb_drive_write_over(&ground, "ZF", "furlough msg", |g| g.jjrg_furlough(furlough_args));
    assert!(res.is_ok(), "furlough must journal to the studbook");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    assert!(
        matches!(loaded.inner().heats.get("\u{20A3}ZF").unwrap().status, jjrg_HeatStatus::Stabled),
        "furlough must journal the Stabled status to the studbook tip"
    );
}

// ---- machine_commit family (B2: each command's extracted `_over` on-branch) ----

#[test]
fn jjtvb_seam_on_retire_journals_excision_and_lands_the_trophy_on_the_consumer() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_retire",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZT", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let fm = jjrf_Firemark::jjrf_parse("ZT").unwrap();
    // The paddock is consumer-side: retire reads it, then its fs tail deletes it and
    // machine_commits [trophy, paddock] to the consumer repo. Seed + track it.
    let paddock_rel = crate::jjri_io::jjri_paddock_path(fm.jjrf_as_str());
    let paddock_abs = ground.consumer_path().join(&paddock_rel);
    std::fs::create_dir_all(paddock_abs.parent().unwrap()).unwrap();
    zjjtvb_commit_all(ground.consumer_path(), &paddock_rel, "## Shape\nfixture paddock\n", "seed paddock");

    let mut output = vvc::vvco_Output::buffer();
    let code = crate::jjrrt_retire::jjrrt_retire_over(
        &jjrfg_PlainGit,
        ground.studbook(),
        "guidon-retire",
        &fm,
        "ZT",
        ground.consumer_path(),
        &[],
        vvc::VVCG_SIZE_LIMIT,
        &mut output,
        "jjx_archive",
    );
    assert_eq!(code, 0, "retire_over must succeed on the two-store path");

    // Studbook half: the excision journaled — the tip no longer carries the heat.
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    assert!(
        !loaded.inner().heats.contains_key("\u{20A3}ZT"),
        "retire must excise the heat from the studbook tip"
    );

    // Consumer half: the fs tail deleted the paddock, and the last commit added a
    // trophy (an addition on the consumer path).
    assert!(!paddock_abs.exists(), "retire's fs tail must delete the consumer paddock");
    let name_status = zjjtvb_git(ground.consumer_path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.lines().any(|l| l.starts_with('A')),
        "the retire commit must add the trophy on the consumer path, name-status: {}",
        name_status
    );
}

#[test]
fn jjtvb_seam_on_nominate_journals_the_new_heat_and_lands_the_paddock_on_the_consumer() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_nominate",
        zjjtvb_seed_gallops(vec![]),
    );
    let nominate_args = jjrg_NominateArgs {
        silks: "nominated-heat".to_string(),
        created: "260101".to_string(),
    };
    let mut output = vvc::vvco_Output::buffer();
    let code = crate::jjrno_nominate::jjrno_nominate_over(
        &jjrfg_PlainGit,
        ground.studbook(),
        "guidon-nominate",
        nominate_args,
        ground.consumer_path(),
        &mut output,
        "jjx_create",
    );
    assert_eq!(code, 0, "nominate_over must succeed on the two-store path");

    // Studbook half: the new heat journaled, minted from the tip's own
    // next_heat_seed ("AB", the seeded gallops' seed), which also advanced.
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    let g = loaded.inner();
    assert!(g.heats.contains_key("\u{20A3}AB"), "nominate must journal the new heat to the studbook tip");
    assert_eq!(g.next_heat_seed, "AC", "nominate must advance next_heat_seed on the tip");

    // Consumer half: the fs tail wrote the paddock template, committed on the
    // consumer path (an addition, mirroring retire's trophy commit).
    let fm = jjrf_Firemark::jjrf_parse("AB").unwrap();
    let paddock_rel = crate::jjri_io::jjri_paddock_path(fm.jjrf_as_str());
    let paddock_abs = ground.consumer_path().join(&paddock_rel);
    assert!(paddock_abs.exists(), "nominate's fs tail must write the consumer paddock");
    let content = std::fs::read_to_string(&paddock_abs).unwrap();
    assert!(content.contains("nominated-heat"), "the paddock template must name the heat's silks");
    let name_status = zjjtvb_git(ground.consumer_path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.lines().any(|l| l.starts_with('A')),
        "the nominate commit must add the paddock on the consumer path, name-status: {}",
        name_status
    );
}

#[test]
fn jjtvb_seam_on_draft_journals_the_relocate() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_draft",
        zjjtvb_seed_gallops(vec![
            zjjtvb_seed_heat("ZG", jjrg_HeatStatus::Racing, &["AAA"]),
            zjjtvb_seed_heat("ZH", jjrg_HeatStatus::Racing, &[]),
        ]),
    );
    let mut output = vvc::vvco_Output::buffer();
    let draft_args = jjrg_DraftArgs {
        coronet: "ZGAAA".to_string(),
        to: "ZH".to_string(),
        before: None,
        after: None,
        first: false,
    };
    let code = crate::jjrdr_draft::jjrdr_draft_over(
        &jjrfg_PlainGit,
        ground.studbook(),
        "guidon-draft",
        draft_args,
        "ZGAAA",
        "ZH",
        &mut output,
        "jjx_redocket",
    );
    assert_eq!(code, 0, "draft_over must journal the relocate");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    let g = loaded.inner();
    assert!(g.heats.get("\u{20A3}ZG").unwrap().order.is_empty(), "draft must empty the source heat on the tip");
    assert_eq!(g.heats.get("\u{20A3}ZH").unwrap().order.len(), 1, "draft must land the pace in the destination heat");
}

#[test]
fn jjtvb_seam_on_restring_journals_the_bulk_relocate() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_restring",
        zjjtvb_seed_gallops(vec![
            zjjtvb_seed_heat("ZI", jjrg_HeatStatus::Racing, &["AAA"]),
            zjjtvb_seed_heat("ZJ", jjrg_HeatStatus::Racing, &[]),
        ]),
    );
    let mut output = vvc::vvco_Output::buffer();
    let restring_args = jjrg_RestringArgs {
        source_firemark: "ZI".to_string(),
        dest_firemark: "ZJ".to_string(),
        coronets: vec!["ZIAAA".to_string()],
    };
    let result = crate::jjrrs_restring::jjrrs_restring_over(
        &jjrfg_PlainGit,
        ground.studbook(),
        "guidon-restring",
        restring_args,
        &mut output,
        "jjx_transfer",
    );
    assert!(result.is_ok(), "restring_over must journal the bulk relocate");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    let g = loaded.inner();
    assert!(g.heats.get("\u{20A3}ZI").unwrap().order.is_empty(), "restring must empty the source heat on the tip");
    assert_eq!(g.heats.get("\u{20A3}ZJ").unwrap().order.len(), 1, "restring must land the transferred pace in the destination heat");
}

#[test]
fn jjtvb_seam_on_wrap_journals_the_completion() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_wrap",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZW", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let mut output = vvc::vvco_Output::buffer();
    let tally_args = jjrg_TallyArgs {
        coronet: "ZWAAA".to_string(),
        state: Some(jjrg_PaceState::Complete),
        text: None,
        silks: None,
    };
    let result = crate::jjrwp_wrap::jjrwp_wrap_over(
        &jjrfg_PlainGit,
        ground.studbook(),
        "guidon-wrap",
        tally_args,
        "wrap chalk".to_string(),
        &mut output,
        "jjx_close",
    );
    assert!(result.is_ok(), "wrap_over must journal the completion");
    let loaded = jjdb_gallops_journal_load(ground.studbook()).unwrap();
    let pace = loaded.inner().heats.get("\u{20A3}ZW").unwrap().paces.get("\u{20A2}ZWAAA").unwrap();
    assert!(
        pace.tacks.iter().any(|t| matches!(t.state, jjrg_PaceState::Complete)),
        "wrap must journal the Complete state to the studbook tip"
    );
}

// ---- refusal (the done-when's "refusal paths refuse loud") ----

#[test]
fn jjtvb_seam_on_refuses_loud_on_a_declining_mutation() {
    let ground = ZjjtvbGround::new(
        "jjtvb_seam_on_refusal",
        zjjtvb_seed_gallops(vec![zjjtvb_seed_heat("ZX", jjrg_HeatStatus::Racing, &["AAA"])]),
    );
    let before = jjdb_gallops_journal_load(ground.studbook()).unwrap().inner().heats.len();
    // A mutation whose precondition fails on the locked tip declines — the seam must
    // surface it as a loud Handler refusal, journaling nothing.
    let res = zjjtvb_drive_write_over(&ground, "ZX", "declining msg", |_g| {
        Err::<(), String>("precondition fails on the tip".to_string())
    });
    match res {
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            assert!(e.contains("precondition fails"), "the refusal must carry the mutation's own message, got: {}", e)
        }
        _ => panic!("a declining mutation must refuse loud as a Handler"),
    }
    let after = jjdb_gallops_journal_load(ground.studbook()).unwrap().inner().heats.len();
    assert_eq!(before, after, "a refused mutation must journal nothing");
}

// ---- residue: the shared cwd→studbook+guidon derivation the ON writers delegate to ----

/// The derivation `zjjrm_studbook_and_guidon` composes (cwd → infield_root →
/// studbook config + guidon) is otherwise reachable only through the const-gated
/// ON writers, so it would first execute at flip-time. Driving it here against a
/// real infield cwd (a hippodrome git repo under an infield dir, the primary-seat
/// shape) closes that residue: the composition executes, and it points at the
/// infield's `jjqs_studbook`. cwd-serialized like the grounds above.
#[test]
fn jjtvb_studbook_and_guidon_derives_from_a_real_infield_cwd() {
    let _serial = ZJJTVB_GROUND_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let infield = JjkTestDir::new("jjtvb_derive_infield");
    let hippodrome = infield.path().join("hippodrome");
    std::fs::create_dir_all(&hippodrome).unwrap();
    zjjtvb_init_local(&hippodrome);
    zjjtvb_commit_all(&hippodrome, "a.txt", "hello", "init");

    let prior_cwd = std::env::current_dir().expect("a cwd to restore");
    std::env::set_current_dir(&hippodrome).expect("point cwd at the hippodrome");
    let derived = crate::jjrm_mcp::zjjrm_studbook_and_guidon("\u{2609}260101-0000-test", "jjx_test");
    // Restore cwd before asserting, so a failed assertion never leaves the runner adrift.
    std::env::set_current_dir(&prior_cwd).expect("restore cwd");

    let (studbook, guidon) = derived.expect("a valid infield cwd must derive the studbook + guidon");
    assert!(
        studbook.local_root.ends_with("jjqs_studbook"),
        "the derived studbook must live at the infield's jjqs_studbook, got {}",
        studbook.local_root.display()
    );
    assert!(!guidon.is_empty(), "the derivation must compose a non-empty guidon");
}