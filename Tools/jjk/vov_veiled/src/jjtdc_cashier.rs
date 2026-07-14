// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the cashier door (`jjrdc_cashier`, JJSVD `jjdd_cashier`).
//!
//! The door adds no mechanism to the break sequence, so what is under test here
//! is the door's own contract: that an unfounded station reports nothing to
//! sight rather than failing, that the report shows what the operator must judge
//! from (age, liveness, the differentiated consequence, the store), and that a
//! break clears the lock and names its victim.

use crate::jjrdc_cashier::{
    jjrdc_any_held,
    jjrdc_held_stores,
    jjrdc_report,
    jjrdc_sight_store,
    jjrdc_State,
    jjrdc_Store,
    JJRDC_STORE_STUDBOOK,
};
use crate::jjrfg_plaingit::jjrfg_PlainGit;
use crate::jjrfr_farrier::{
    jjrfr_break,
    jjrfr_FarrierLock,
};
use crate::jjrvb_blotter::{
    jjdb_BlotterConfig,
    JJDB_CATCHWORD_FOUNDING,
    JJDB_CATCHWORD_SIGIL,
};
use crate::jjrvg_guidon::{
    jjdb_guidon_compose,
    JJDB_LIVENESS_WARN_SECONDS,
};
use crate::jjtu_testdir::JjkTestDir;
use chrono::{
    Duration,
    Utc,
};
use std::path::{
    Path,
    PathBuf,
};

const ZJJTDC_TRUNK: &str = "jjtdc-trunk";

fn zjjtdc_git(dir: &Path, args: &[&str]) {
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
}

/// A bare remote plus a local clone tracking it — a blotter this door can sight.
fn zjjtdc_scratch(name: &str) -> (JjkTestDir, JjkTestDir, jjdb_BlotterConfig) {
    let bare = JjkTestDir::new(&format!("{}_bare", name));
    zjjtdc_git(bare.path(), &["init", "-q", "--bare", "-b", ZJJTDC_TRUNK]);
    let local = JjkTestDir::new(&format!("{}_local", name));
    zjjtdc_git(local.path(), &["init", "-q", "-b", ZJJTDC_TRUNK]);
    zjjtdc_git(local.path(), &["config", "user.email", "jjtdc@example.invalid"]);
    zjjtdc_git(local.path(), &["config", "user.name", "jjtdc"]);
    std::fs::write(local.path().join("base.txt"), "base").unwrap();
    zjjtdc_git(local.path(), &["add", "--", "base.txt"]);
    zjjtdc_git(local.path(), &["commit", "-q", "-m", "init"]);
    zjjtdc_git(local.path(), &["remote", "add", "origin", &bare.path().to_string_lossy()]);
    zjjtdc_git(local.path(), &["push", "-q", "-u", "origin", ZJJTDC_TRUNK]);
    let config = jjdb_BlotterConfig {
        local_root: local.path().to_path_buf(),
        remote_url: bare.path().to_string_lossy().into_owned(),
        trunk: ZJJTDC_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    };
    (bare, local, config)
}

fn zjjtdc_store(config: jjdb_BlotterConfig) -> jjrdc_Store {
    jjrdc_Store {
        name: JJRDC_STORE_STUDBOOK,
        config,
    }
}

/// The orientation hazard this door was built against: a station that never
/// founded its studbook has no clone to sight. That is "nothing to sight", not a
/// failure — a blind sight would raise an unclassifiable git error at exactly
/// the moment the operator is trying to diagnose one.
#[test]
fn jjtdc_an_unfounded_station_reports_nothing_to_sight() {
    let store = zjjtdc_store(jjdb_BlotterConfig {
        local_root: PathBuf::from("/nonexistent/jjqs_studbook"),
        remote_url: "git@example.invalid:nothing.git".to_string(),
        trunk: ZJJTDC_TRUNK.to_string(),
        ordinal_sigil: JJDB_CATCHWORD_SIGIL,
        ordinal_founding: JJDB_CATCHWORD_FOUNDING,
    });

    let sighting = jjrdc_sight_store(&jjrfg_PlainGit, &store);

    assert_eq!(sighting.state, jjrdc_State::Unfounded);
    let report = jjrdc_report(std::slice::from_ref(&sighting));
    assert!(report.contains("nothing to sight"), "report was: {}", report);
    assert!(!jjrdc_any_held(std::slice::from_ref(&sighting)));
}

#[test]
fn jjtdc_a_free_store_reports_no_lock_held() {
    let (_bare, _local, config) = zjjtdc_scratch("jjtdc_free");
    let sighting = jjrdc_sight_store(&jjrfg_PlainGit, &zjjtdc_store(config));

    assert_eq!(sighting.state, jjrdc_State::Free);
    assert!(jjrdc_report(std::slice::from_ref(&sighting)).contains("no lock held"));
    assert!(!jjrdc_any_held(std::slice::from_ref(&sighting)));
}

/// The report is what the operator decides from, so the gate's four required
/// showings are asserted here: the guidon's fields, acquire time as an AGE, the
/// store the lock guards, and the DIFFERENTIATED consequence (JJSVD, The confirm
/// gate).
#[test]
fn jjtdc_a_held_store_reports_the_fields_the_gate_must_show() {
    let (_bare, _local, config) = zjjtdc_scratch("jjtdc_held");
    let farrier = jjrfg_PlainGit;
    let guidon = jjdb_guidon_compose(
        "☉260714-1006",
        "beast",
        Utc::now() - Duration::minutes(20),
        "journal",
    );
    farrier.jjrfr_stake(&config.local_root, &guidon).unwrap();

    let sighting = jjrdc_sight_store(&farrier, &zjjtdc_store(config));
    assert!(matches!(sighting.state, jjrdc_State::Held(_)));
    assert!(jjrdc_any_held(std::slice::from_ref(&sighting)));
    assert_eq!(jjrdc_held_stores(std::slice::from_ref(&sighting)), vec![JJRDC_STORE_STUDBOOK.to_string()]);

    let report = jjrdc_report(std::slice::from_ref(&sighting));
    assert!(report.contains("☉260714-1006"), "the officium must be named: {}", report);
    assert!(report.contains("beast"), "the station must be named: {}", report);
    assert!(report.contains("journal"), "the operation must be named: {}", report);
    assert!(report.contains("20m ago"), "acquire time must render as an AGE, not a timestamp: {}", report);
    assert!(report.contains(JJRDC_STORE_STUDBOOK), "the store the lock guards must be named: {}", report);
    assert!(report.contains("live WRITER"), "the writer consequence must be shown: {}", report);
    assert!(report.contains("live READER"), "the reader consequence must be shown: {}", report);
    assert!(
        !report.contains("WARNING"),
        "a 20-minute-old lock must earn no liveness warning: {}",
        report
    );
}

/// The liveness warning warns and never blocks: a young lock is probably a live
/// writer, and the report says so — but the door still offers the break, because
/// the operator may know something the clock does not.
#[test]
fn jjtdc_a_young_lock_earns_the_liveness_warning() {
    let (_bare, _local, config) = zjjtdc_scratch("jjtdc_young");
    let farrier = jjrfg_PlainGit;
    let guidon = jjdb_guidon_compose(
        "☉260714-1006",
        "beast",
        Utc::now() - Duration::seconds(JJDB_LIVENESS_WARN_SECONDS / 2),
        "journal",
    );
    farrier.jjrfr_stake(&config.local_root, &guidon).unwrap();

    let sighting = jjrdc_sight_store(&farrier, &zjjtdc_store(config));
    let report = jjrdc_report(std::slice::from_ref(&sighting));

    assert!(report.contains("YOUNG"), "a fresh lock must be flagged live: {}", report);
    assert!(jjrdc_any_held(std::slice::from_ref(&sighting)), "and it must still be offered for breaking");
}

/// A mark this engine did not compose is STILL a lock (the mechanics compare
/// blob hashes, never text). It must be sightable, it must render, and it must
/// break — so the report shows it verbatim and says why.
#[test]
fn jjtdc_a_malformed_guidon_is_still_a_lock_and_still_breaks() {
    let (_bare, _local, config) = zjjtdc_scratch("jjtdc_malformed");
    let farrier = jjrfg_PlainGit;
    let foreign = "some-other-engine-wrote-this";
    farrier.jjrfr_stake(&config.local_root, foreign).unwrap();

    let sighting = jjrdc_sight_store(&farrier, &zjjtdc_store(config.clone()));
    let report = jjrdc_report(std::slice::from_ref(&sighting));
    assert!(report.contains(foreign), "an unreadable mark must be shown verbatim: {}", report);
    assert!(report.contains("still a lock"), "and the operator must be told it breaks like any other");

    let cleared = jjrfr_break(&farrier, &config.local_root).unwrap();
    assert_eq!(cleared.as_deref(), Some(foreign), "the break must clear it and name what it cleared");
    assert_eq!(farrier.jjrfr_sight(&config.local_root).unwrap(), None);
}

/// The whole point of the door: after the break, the lock is gone and the next
/// ceremony can take it.
#[test]
fn jjtdc_the_break_clears_the_lock_and_names_its_victim() {
    let (_bare, _local, config) = zjjtdc_scratch("jjtdc_break");
    let farrier = jjrfg_PlainGit;
    let guidon = jjdb_guidon_compose("☉260714-1006", "beast", Utc::now(), "journal");
    farrier.jjrfr_stake(&config.local_root, &guidon).unwrap();

    let cleared = jjrfr_break(&farrier, &config.local_root).unwrap();

    assert_eq!(cleared.as_deref(), Some(guidon.as_str()));
    assert_eq!(
        jjrdc_sight_store(&farrier, &zjjtdc_store(config)).state,
        jjrdc_State::Free,
        "the store must sight free after the break"
    );
}
