// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVTES — the practice seat's layout, short of the kennel: over a scratch
// maintainer repository, the seat clone stands two levels below the practice
// home at the maintainer's checked-out branch, its only remotes its origin and
// the practice base remote with the push side neutered; its station file names
// a relative tool store that resolves to the station's own, beside an empty
// secrets directory; a second stand-up over the same home refuses; and the
// door's election refuses the agora, a missing or relative home and an extra
// argument. The scratch stands under the crate's own ignored build directory,
// and nothing reaches a network.

use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

use crate::rbvces_seat::{
    rbvces_elect,
    rbvces_stand,
    rbvces_Station,
    RBVCES_CLONE_SUBDIR,
    RBVCES_SEAT_SUBDIR, //
};
use crate::rbvcep_repo::{
    RBVCEP_SECRETS_SUBDIR,
    RBVCEP_STATION_FILE,
    RBVCEP_STATION_SUBDIR, //
};
use crate::rbvcew_world::{
    rbvcew_word,
    rbvcew_World,
    RBVCEW_PALAESTRA, //
};

/// The pace-like branch the scratch maintainer stands on, apart from main.
const ZRBVTES_BRANCH: &str = "pace-branch";

/// The verb the election is told it serves.
const ZRBVTES_VERB: &str = "encamp";

fn zrbvtes_git(dir: &Path, args: &[&str], global: &Path) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", global)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ALLOW_PROTOCOL", "file")
        .output()
        .expect("test harness git invocation must spawn");
    assert!(
        out.status.success(),
        "git {:?} in {} failed: {}",
        args,
        dir.display(),
        String::from_utf8_lossy(&out.stderr), //
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn zrbvtes_scratch_root() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after the epoch")
        .as_nanos();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!(
            "rbvtes_seat_{}_{}",
            std::process::id(),
            nanos, //
        ));
    std::fs::create_dir_all(&root).expect("scratch root must be creatable");
    std::fs::canonicalize(&root).expect("scratch root must resolve")
}

fn zrbvtes_args(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| w.to_string()).collect()
}

/// The seat stands as the memo lays it out, and a second stand-up refuses.
#[test]
fn rbvtes_seat_layout_over_a_scratch_maintainer() {
    let root = zrbvtes_scratch_root();
    let top = root.join("maintainer");
    let tackroom = root.join("stall").join("tackroom-buk");
    let home = root.join("practice-home");
    std::fs::create_dir_all(&top).expect("scratch maintainer must be creatable");
    std::fs::create_dir_all(&tackroom).expect("scratch tackroom must be creatable");
    let global = root.join("global.gitconfig");
    std::fs::write(
        &global,
        "[user]\n\tname = rbvtes\n\temail = rbvtes@example.invalid\n",
    )
    .expect("scratch global must be writable");
    zrbvtes_git(&top, &["init", "-q", "-b", "main"], &global);
    std::fs::write(top.join("README.md"), "maintainer\n").expect("seed file must be writable");
    zrbvtes_git(&top, &["add", "README.md"], &global);
    zrbvtes_git(&top, &["commit", "-q", "-m", "maintainer"], &global);
    zrbvtes_git(&top, &["checkout", "-q", "-b", ZRBVTES_BRANCH], &global);
    let tip = zrbvtes_git(&top, &["rev-parse", "HEAD"], &global);

    let station = rbvces_Station {
        user: "seatuser".to_string(),
        tincture: "tst".to_string(),
        tackroom: std::fs::canonicalize(&tackroom).expect("scratch tackroom must resolve"),
    };
    let seat = rbvces_stand(&top, &home, &RBVCEW_PALAESTRA, &station)
        .unwrap_or_else(|e| panic!("the seat must stand: {}", e));

    // Two levels below the home, at the maintainer's branch and tip.
    let seat_dir = home.join(RBVCES_SEAT_SUBDIR);
    assert_eq!(seat.clone_dir, seat_dir.join(RBVCES_CLONE_SUBDIR));
    assert_eq!(
        zrbvtes_git(&seat.clone_dir, &["rev-parse", "--abbrev-ref", "HEAD"], &global).trim(),
        ZRBVTES_BRANCH,
    );
    assert_eq!(zrbvtes_git(&seat.clone_dir, &["rev-parse", "HEAD"], &global), tip);

    // Origin and the practice base alone, its push side the sentinel.
    let remotes = zrbvtes_git(&seat.clone_dir, &["remote", "-v"], &global);
    let lines: Vec<&str> = remotes.lines().collect();
    let top_str = top.to_str().expect("scratch path is UTF-8");
    let expected = [
        format!(
            "origin\t{} (fetch)",
            top_str, //
        ),
        format!(
            "origin\t{} (push)",
            top_str, //
        ),
        format!(
            "{}\t{} (fetch)",
            RBVCEW_PALAESTRA.base_remote,
            RBVCEW_PALAESTRA.base_url, //
        ),
        format!(
            "{}\t{} (push)",
            RBVCEW_PALAESTRA.base_remote,
            RBVCEW_PALAESTRA.base_push_disabled, //
        ),
    ];
    assert_eq!(lines.len(), 4, "the seat names other remotes:\n{}", remotes);
    for want in &expected {
        assert!(
            lines.contains(&want.as_str()),
            "the seat's remotes lack '{}':\n{}",
            want,
            remotes, //
        );
    }
    assert_eq!(
        RBVCEW_PALAESTRA.base_push_disabled,
        "DISABLED-PALAESTRA_UPSTREAM-IS-READ-ONLY", //
    );
    assert!(
        zrbvtes_git(&seat.clone_dir, &["status", "--porcelain"], &global)
            .trim()
            .is_empty(),
        "the seat clone is not clean", //
    );

    // The station file where the clone's regime looks, its stores relative.
    let station_dir = seat_dir.join(RBVCEP_STATION_SUBDIR);
    assert_eq!(seat.station_file, station_dir.join(RBVCEP_STATION_FILE));
    let burs = std::fs::read_to_string(&seat.station_file).expect("the BURS file must be readable");
    let value = |key: &str| -> String {
        burs.lines()
            .find_map(|l| l.strip_prefix(&format!("{}=", key)).map(|v| v.to_string()))
            .unwrap_or_else(|| panic!("the BURS file names no {}:\n{}", key, burs))
    };
    assert_eq!(value("BURS_USER"), "seatuser");
    assert_eq!(value("BURS_TINCTURE"), "tst");
    assert_eq!(value("BURS_LOG_DIR"), "../logs-buk");
    let rel = value("BURS_TACKROOM");
    assert!(!rel.starts_with('/'), "BURS_TACKROOM is absolute: {}", rel);
    assert_eq!(
        std::fs::canonicalize(seat.clone_dir.join(&rel)).expect("the relative tool store must resolve"),
        station.tackroom,
        "BURS_TACKROOM does not reach the tool store", //
    );

    // The empty secrets directory beside it.
    assert_eq!(seat.secrets_dir, station_dir.join(RBVCEP_SECRETS_SUBDIR));
    assert_eq!(
        std::fs::read_dir(&seat.secrets_dir)
            .expect("secrets directory must stand")
            .count(),
        0,
        "the secrets directory is not empty", //
    );

    // A seat already standing refuses, touching nothing.
    let again = rbvces_stand(&top, &home, &RBVCEW_PALAESTRA, &station);
    assert!(again.is_err(), "a second stand-up over a standing seat landed");

    std::fs::remove_dir_all(&root).expect("scratch root must be removable");
}

/// The election admits the palaestra and an absolute home alone.
#[test]
fn rbvtes_seat_election_refuses_all_but_palaestra_and_an_absolute_home() {
    let palaestra = rbvcew_word(rbvcew_World::Palaestra);
    let agora = rbvcew_word(rbvcew_World::Agora);
    assert_eq!(
        rbvces_elect(ZRBVTES_VERB, &zrbvtes_args(&[palaestra, "/abs/home"])),
        Ok(PathBuf::from("/abs/home")),
    );
    for args in [
        vec![],
        vec![agora, "/abs/home"],
        vec!["practice", "/abs/home"],
        vec![palaestra],
        vec![palaestra, "relative/home"],
        vec![palaestra, "/abs/home", "extra"],
    ] {
        assert!(
            rbvces_elect(ZRBVTES_VERB, &zrbvtes_args(&args)).is_err(),
            "the election admitted {:?}",
            args, //
        );
    }
}
