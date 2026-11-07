// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Hurdles for bale: selection by age alone, the inclusive last day and the
//! later-day refusal, the output folder stepped over, the read-back that gates
//! every removal, links kept as links, and no archive ever overwritten.
//!
//! Every hurdle composes its own yard, confines the library's folders to it,
//! and fixes today rather than reading the clock.

use crate::bkhlrb_bale::{
    bkhlrb_Member,
    bkhlrb_Outcome,
    bkhlrb_bale,
    bkhlrb_latest,
    bkhlrb_name,
    bkhlrb_read_back, //
};
use crate::bkhlrd_day::bkhlrd_Day;
use crate::bkhlrg_ground::{
    bkhlrg_Ground,
    bkhlrg_read, //
};
use crate::bkhlty_yard::{
    bkhlty_Yard,
    bkhlty_confine,
    bkhlty_date,
    bkhlty_mtime,
    BKHLTY_OUTPUT_APART,
    BKHLTY_OUTPUT_WITHIN,
    BKHLTY_TINCTURE, //
};
use flate2::read::GzDecoder;
use std::path::{
    Path,
    PathBuf, //
};
use tar::{
    Archive,
    EntryType, //
};

/// The day every bale hurdle stands on.
fn zbkhltb_today() -> bkhlrd_Day {
    bkhlrd_Day::bkhlrd_parse("20260315").expect("the fixed day parses")
}

/// A yard read and confined, ready to bale.
fn zbkhltb_ground(yard: &bkhlty_Yard) -> bkhlrg_Ground {
    let ground = bkhlrg_read(&yard.bkhlty_project()).expect("the composed project reads");
    bkhlty_confine(yard, &ground);
    ground
}

/// The archive a bale wrote, or a panic naming what it did instead.
fn zbkhltb_wrote(outcome: bkhlrb_Outcome) -> crate::bkhlrb_bale::bkhlrb_Baled {
    match outcome {
        bkhlrb_Outcome::bkhlrb_Wrote(baled) => baled,
        bkhlrb_Outcome::bkhlrb_Nothing(last) => panic!(
            "nothing was baled on or before {}",
            last.bkhlrd_digits(), //
        ),
    }
}

/// Every member of an archive: its name, its kind and, for a link, its target.
fn zbkhltb_members(archive: &Path) -> Vec<(PathBuf, EntryType, Option<PathBuf>)> {
    let opened = std::fs::File::open(archive).expect("the archive opens");
    let mut reader = Archive::new(GzDecoder::new(opened));
    reader
        .entries()
        .expect("the archive reads")
        .map(|entry| {
            let entry = entry.expect("a member reads");
            (
                entry.path().expect("a name reads").into_owned(),
                entry.header().entry_type(),
                entry
                    .link_name()
                    .expect("a link target reads")
                    .map(|target| target.into_owned()),
            )
        })
        .collect()
}

/// The names of an archive's members.
fn zbkhltb_names(archive: &Path) -> Vec<PathBuf> {
    zbkhltb_members(archive)
        .into_iter()
        .map(|(name, _, _)| name)
        .collect()
}

#[test]
fn bkhltb_the_last_day_is_included_and_the_day_after_is_not() {
    let yard = bkhlty_Yard::bkhlty_compose("inclusive", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let last = today.bkhlrd_back(5);
    let on = yard.bkhlty_plant("../quillery/on-the-day.txt", b"brindle", last);
    let after = yard.bkhlty_plant(
        "../quillery/the-day-after.txt",
        b"grizzle",
        last.bkhlrd_back(-1),
    );
    let ground = zbkhltb_ground(&yard);

    let baled =
        zbkhltb_wrote(bkhlrb_bale(&ground, last, today, &bkhlrb_read_back).expect("the bale runs"));

    assert_eq!(baled.first, last);
    assert_eq!(baled.last, last);
    assert_eq!(
        baled.archive.file_name().and_then(|n| n.to_str()),
        Some(bkhlrb_name(BKHLTY_TINCTURE, last, last).as_str()), //
    );
    assert_eq!(
        zbkhltb_names(&baled.archive),
        vec![PathBuf::from("quillery/on-the-day.txt")], //
    );
    assert!(!on.exists());
    assert!(after.exists());
}

#[test]
fn bkhltb_a_day_later_than_two_days_ago_is_refused_and_nothing_moves() {
    let yard = bkhlty_Yard::bkhlty_compose("later", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let planted = yard.bkhlty_plant("../murkwell/ancient.txt", b"dapple", today.bkhlrd_back(40));
    let ground = zbkhltb_ground(&yard);

    for later in [today.bkhlrd_back(1), today, today.bkhlrd_back(-1)] {
        let why = bkhlrb_bale(&ground, later, today, &bkhlrb_read_back)
            .expect_err("a later day is refused");
        assert!(why.contains("later than"), "{}", why);
    }

    assert!(planted.exists());
    let hayloft = ground
        .hayloft
        .as_ref()
        .expect("the archive folder is named");
    assert!(!hayloft.folder.exists(), "a refused bale writes nothing");
}

#[test]
fn bkhltb_the_latest_day_taken_is_two_days_ago() {
    let yard = bkhlty_Yard::bkhlty_compose("latest", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let two = yard.bkhlty_plant("../quillery/two-back.txt", b"roan", today.bkhlrd_back(2));
    let one = yard.bkhlty_plant("../quillery/one-back.txt", b"bay", today.bkhlrd_back(1));
    let ground = zbkhltb_ground(&yard);

    let baled = zbkhltb_wrote(
        bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back).expect("runs"),
    );

    assert_eq!(baled.last, today.bkhlrd_back(2));
    assert!(!two.exists());
    assert!(one.exists());
}

#[test]
fn bkhltb_both_folders_are_swept_by_age_whatever_their_entries_are_named() {
    let yard = bkhlty_Yard::bkhlty_compose("byage", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let old = today.bkhlrd_back(30);
    let new = today.bkhlrd_back(1);
    let gone = [
        yard.bkhlty_plant("../quillery/last.txt", b"a", old),
        yard.bkhlty_plant("../quillery/hist-zz-20260101.txt", b"b", old),
        yard.bkhlty_plant("../quillery/whatever", b"c", old),
        yard.bkhlty_plant("../murkwell/temp-20260101-1/burx.env", b"d", old),
        yard.bkhlty_plant("../murkwell/oddly named", b"e", old),
    ];
    let kept = [
        yard.bkhlty_plant("../quillery/hist-zz-20260313.txt", b"f", new),
        yard.bkhlty_plant("../quillery/ancient-sounding-name.txt", b"g", new),
        yard.bkhlty_plant("../murkwell/temp-19990101-1/burx.env", b"h", new),
    ];
    let ground = zbkhltb_ground(&yard);

    let baled = zbkhltb_wrote(
        bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back).expect("runs"),
    );

    assert_eq!(baled.units, 5);
    for path in gone {
        assert!(!path.exists(), "{} should have gone", path.display());
    }
    for path in kept {
        assert!(path.exists(), "{} should have stayed", path.display());
    }
    assert!(ground.log.is_dir(), "the swept folder itself stays");
    assert!(ground.temp.is_dir(), "the swept folder itself stays");

    let names = zbkhltb_names(&baled.archive);
    assert!(names.contains(&PathBuf::from("murkwell/temp-20260101-1/burx.env")));
    assert!(names.contains(&PathBuf::from("quillery/whatever")));
}

#[test]
fn bkhltb_a_run_folder_is_judged_by_its_newest_file() {
    let yard = bkhlty_Yard::bkhlty_compose("newest", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let old = today.bkhlrd_back(20);
    let mixed_old = yard.bkhlty_plant("../murkwell/mixed/deep/early.txt", b"piebald", old);
    let mixed_new = yard.bkhlty_plant("../murkwell/mixed/late.txt", b"skewbald", today);
    let aged = yard.bkhlty_plant("../murkwell/aged/deep/only.txt", b"palomino", old);
    let bare = yard.bkhlty_at("../murkwell/bare");
    std::fs::create_dir_all(&bare).expect("the bare folder composes");
    bkhlty_date(&bare, old);
    let ground = zbkhltb_ground(&yard);

    let baled = zbkhltb_wrote(
        bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back).expect("runs"),
    );

    assert_eq!(
        baled.units, 2,
        "the aged folder and the bare one go; the mixed one stays"
    );
    assert!(
        mixed_old.exists(),
        "a folder holding one new file stays whole"
    );
    assert!(mixed_new.exists());
    assert!(!aged.exists());
    assert!(
        !yard.bkhlty_at("../murkwell/aged").exists(),
        "the emptied folder goes"
    );
    assert!(
        !bare.exists(),
        "a folder holding no file goes by its own time"
    );
    assert_eq!(baled.first, old);
}

#[test]
fn bkhltb_the_first_day_is_the_oldest_file_taken() {
    let yard = bkhlty_Yard::bkhlty_compose("firstday", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let oldest = today.bkhlrd_back(90);
    yard.bkhlty_plant("../murkwell/run/a.txt", b"x", oldest);
    yard.bkhlty_plant("../murkwell/run/b.txt", b"y", today.bkhlrd_back(10));
    yard.bkhlty_plant("../quillery/c.txt", b"z", today.bkhlrd_back(50));
    let ground = zbkhltb_ground(&yard);

    let baled = zbkhltb_wrote(
        bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back).expect("runs"),
    );

    assert_eq!(baled.first, oldest);
    assert_eq!(
        baled.archive.file_name().and_then(|n| n.to_str()),
        Some(bkhlrb_name(BKHLTY_TINCTURE, oldest, today.bkhlrd_back(2)).as_str()), //
    );
}

#[test]
fn bkhltb_the_output_folder_is_never_touched() {
    let yard = bkhlty_Yard::bkhlty_compose("output", true, BKHLTY_OUTPUT_WITHIN);
    let today = zbkhltb_today();
    let old = today.bkhlrd_back(30);
    let held = yard.bkhlty_plant("../murkwell/outfold/current/result.txt", b"chestnut", old);
    bkhlty_date(&yard.bkhlty_at("../murkwell/outfold/current"), old);
    bkhlty_date(&yard.bkhlty_at("../murkwell/outfold"), old);
    let swept = yard.bkhlty_plant("../murkwell/run/result.txt", b"sorrel", old);
    let ground = zbkhltb_ground(&yard);

    let baled = zbkhltb_wrote(
        bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back).expect("runs"),
    );

    assert!(held.exists(), "the output folder's contents stay");
    assert!(!swept.exists());
    assert!(zbkhltb_names(&baled.archive)
        .iter()
        .all(|name| !name.starts_with("murkwell/outfold")));
}

#[test]
fn bkhltb_a_failed_check_removes_nothing() {
    let yard = bkhlty_Yard::bkhlty_compose("failed", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let old = today.bkhlrd_back(30);
    let planted = [
        yard.bkhlty_plant("../quillery/one.txt", b"cremello", old),
        yard.bkhlty_plant("../murkwell/run/two.txt", b"perlino", old),
    ];
    let ground = zbkhltb_ground(&yard);
    let refusing = |_: &Path, _: &[bkhlrb_Member]| -> Result<(), String> {
        Err("a planted refusal".to_string())
    };

    let why = bkhlrb_bale(&ground, bkhlrb_latest(today), today, &refusing)
        .expect_err("the check refuses");

    assert!(why.contains("a planted refusal"), "{}", why);
    assert!(why.contains("nothing was"), "{}", why);
    for path in planted {
        assert!(path.exists(), "{} should still stand", path.display());
    }
    let hayloft = &ground.hayloft.as_ref().expect("named").folder;
    let name = bkhlrb_name(BKHLTY_TINCTURE, old, today.bkhlrd_back(2));
    assert!(
        !hayloft.join(&name).exists(),
        "no archive takes the final name"
    );
    assert!(
        hayloft.join(format!(".{}", name)).exists(),
        "the archive stands for inspection under its temporary name"
    );
}

#[test]
fn bkhltb_the_read_back_catches_an_original_that_differs() {
    let yard = bkhlty_Yard::bkhlty_compose("differs", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let old = today.bkhlrd_back(30);
    let planted = yard.bkhlty_plant("../quillery/one.txt", b"cremello", old);
    let ground = zbkhltb_ground(&yard);
    let tampering = |archive: &Path, members: &[bkhlrb_Member]| -> Result<(), String> {
        std::fs::write(&planted, b"cremellO").map_err(|err| err.to_string())?;
        bkhlty_date(&planted, old);
        bkhlrb_read_back(archive, members)
    };

    let why = bkhlrb_bale(&ground, bkhlrb_latest(today), today, &tampering)
        .expect_err("the bytes differ");

    assert!(why.contains("differ"), "{}", why);
    assert!(planted.exists());
}

#[test]
fn bkhltb_links_are_kept_as_links_and_never_followed() {
    let yard = bkhlty_Yard::bkhlty_compose("links", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let old = today.bkhlrd_back(30);
    let outside = yard.bkhlty_plant("../elsewhere/kept.txt", b"buckskin", today);
    yard.bkhlty_plant("../murkwell/run/real.txt", b"dun", old);
    let link = yard.bkhlty_at("../murkwell/run/pointer");
    std::os::unix::fs::symlink(&outside, &link).expect("the link composes");
    let folder_link = yard.bkhlty_at("../murkwell/run/folder-pointer");
    std::os::unix::fs::symlink("../../elsewhere", &folder_link).expect("the link composes");
    let ground = zbkhltb_ground(&yard);

    let baled = zbkhltb_wrote(
        bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back).expect("runs"),
    );

    let members = zbkhltb_members(&baled.archive);
    let pointer = members
        .iter()
        .find(|(name, _, _)| name == Path::new("murkwell/run/pointer"))
        .expect("the link is a member");
    assert_eq!(pointer.1, EntryType::Symlink);
    assert_eq!(pointer.2.as_deref(), Some(outside.as_path()));
    let folder_pointer = members
        .iter()
        .find(|(name, _, _)| name == Path::new("murkwell/run/folder-pointer"))
        .expect("the folder link is a member");
    assert_eq!(folder_pointer.1, EntryType::Symlink);
    assert!(
        members.iter().all(
            |(name, _, _)| !name.starts_with("murkwell/run/folder-pointer")
                || name == Path::new("murkwell/run/folder-pointer")
        ),
        "a linked folder is never descended"
    );
    assert!(!link.exists() && std::fs::symlink_metadata(&link).is_err());
    assert!(outside.exists(), "a link's target is never removed");
    assert!(yard.bkhlty_at("../elsewhere").is_dir());
}

#[test]
fn bkhltb_nothing_old_enough_writes_nothing() {
    let yard = bkhlty_Yard::bkhlty_compose("nothing", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltb_today();
    let fresh = yard.bkhlty_plant("../quillery/fresh.txt", b"grullo", today);
    let ground = zbkhltb_ground(&yard);

    let outcome = bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back)
        .expect("the bale runs");

    assert_eq!(
        outcome,
        bkhlrb_Outcome::bkhlrb_Nothing(today.bkhlrd_back(2))
    );
    assert!(fresh.exists());
    assert!(!ground.hayloft.as_ref().expect("named").folder.exists());
}

#[test]
fn bkhltb_an_archive_or_its_temporary_already_standing_is_never_overwritten() {
    for mark in ["", "."] {
        let yard = bkhlty_Yard::bkhlty_compose("overwrite", true, BKHLTY_OUTPUT_APART);
        let today = zbkhltb_today();
        let old = today.bkhlrd_back(30);
        let planted = yard.bkhlty_plant("../quillery/one.txt", b"overo", old);
        let name = bkhlrb_name(BKHLTY_TINCTURE, old, today.bkhlrd_back(2));
        let standing = yard.bkhlty_plant(
            &format!(
                "../strawloft/{}{}",
                mark,
                name, //
            ),
            b"tobiano",
            today,
        );
        let before = bkhlty_mtime(&standing);
        let ground = zbkhltb_ground(&yard);

        let why = bkhlrb_bale(&ground, bkhlrb_latest(today), today, &bkhlrb_read_back)
            .expect_err("a standing archive refuses");

        assert!(why.contains("never overwritten"), "{}", why);
        assert!(planted.exists());
        assert_eq!(std::fs::read(&standing).expect("still reads"), b"tobiano");
        assert_eq!(bkhlty_mtime(&standing), before);
    }
}

// eof
