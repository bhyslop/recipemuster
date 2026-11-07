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

//! Hurdles for unbale: the paste-ready listing, the empty-or-absent target and
//! the live folders refused as targets, and one archive unpacked both by this
//! crate and by the machine's own `tar`, a long name and a link compared.

use crate::bkhlrb_bale::{
    bkhlrb_Outcome,
    bkhlrb_bale,
    bkhlrb_latest,
    bkhlrb_read_back, //
};
use crate::bkhlrd_day::bkhlrd_Day;
use crate::bkhlrg_ground::{
    bkhlrg_Ground,
    bkhlrg_read,
    BKHLRG_HAYLOFT_KEY, //
};
use crate::bkhlru_unbale::{
    bkhlru_listing,
    bkhlru_unbale, //
};
use crate::bkhlty_yard::{
    bkhlty_Yard,
    bkhlty_confine,
    bkhlty_mtime,
    BKHLTY_HAYLOFT,
    BKHLTY_OUTPUT_APART, //
};
use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

/// The day every unbale hurdle stands on.
fn zbkhltu_today() -> bkhlrd_Day {
    bkhlrd_Day::bkhlrd_parse("20260315").expect("the fixed day parses")
}

/// A yard read and confined.
fn zbkhltu_ground(yard: &bkhlty_Yard) -> bkhlrg_Ground {
    let ground = bkhlrg_read(&yard.bkhlty_project()).expect("the composed project reads");
    bkhlty_confine(yard, &ground);
    ground
}

/// A member name past a hundred characters, folders and a file.
fn zbkhltu_long_name() -> String {
    format!(
        "run-long/{}/{}/{}.txt",
        "a".repeat(60),
        "b".repeat(60),
        "c".repeat(60), //
    )
}

/// Bale a tree holding a long name, a link and a plain file, answering the
/// archive.
fn zbkhltu_baled(yard: &bkhlty_Yard, ground: &bkhlrg_Ground) -> PathBuf {
    let today = zbkhltu_today();
    let old = today.bkhlrd_back(30);
    yard.bkhlty_plant(
        &format!(
            "../murkwell/{}",
            zbkhltu_long_name(), //
        ),
        b"appaloosa\n",
        old,
    );
    yard.bkhlty_plant(
        "../murkwell/run-long/plain.txt",
        &[0u8, 1, 2, 255, 254],
        old,
    );
    std::os::unix::fs::symlink("plain.txt", yard.bkhlty_at("../murkwell/run-long/pointer"))
        .expect("the link composes");
    yard.bkhlty_plant("../quillery/hist-zz.txt", b"marengo", old);

    match bkhlrb_bale(ground, bkhlrb_latest(today), today, &bkhlrb_read_back)
        .expect("the bale runs")
    {
        bkhlrb_Outcome::bkhlrb_Wrote(baled) => baled.archive,
        bkhlrb_Outcome::bkhlrb_Nothing(_) => panic!("nothing was baled"),
    }
}

/// Assert an unpacked tree carries the long name, the plain bytes and the link.
fn zbkhltu_compare(unpacked: &Path) {
    let long = unpacked.join("murkwell").join(zbkhltu_long_name());
    assert_eq!(
        std::fs::read(&long).expect("the long name unpacks"),
        b"appaloosa\n"
    );
    assert_eq!(
        std::fs::read(unpacked.join("murkwell/run-long/plain.txt")).expect("plain unpacks"),
        vec![0u8, 1, 2, 255, 254], //
    );
    let pointer = unpacked.join("murkwell/run-long/pointer");
    assert!(std::fs::symlink_metadata(&pointer)
        .expect("the link unpacks")
        .file_type()
        .is_symlink());
    assert_eq!(
        std::fs::read_link(&pointer).expect("the link reads"),
        PathBuf::from("plain.txt"), //
    );
    assert_eq!(
        std::fs::read(unpacked.join("quillery/hist-zz.txt")).expect("the other folder unpacks"),
        b"marengo", //
    );
}

#[test]
fn bkhltu_the_listing_is_ready_to_paste_from_the_project_root() {
    let yard = bkhlty_Yard::bkhlty_compose("listing", true, BKHLTY_OUTPUT_APART);
    let today = zbkhltu_today();
    for name in [
        "zq-20260101-20260131.tar.gz",
        "zq-20251201-20251231.tar.gz",
        ".zq-20260201-20260228.tar.gz",
        "notes.txt",
    ] {
        yard.bkhlty_plant(
            &format!(
                "{}/{}",
                BKHLTY_HAYLOFT,
                name, //
            ),
            b"x",
            today,
        );
    }
    let ground = zbkhltu_ground(&yard);

    let listing = bkhlru_listing(&ground).expect("the listing reads");

    assert_eq!(
        listing,
        vec![
            format!(
                "{}/zq-20251201-20251231.tar.gz",
                BKHLTY_HAYLOFT, //
            ),
            format!(
                "{}/zq-20260101-20260131.tar.gz",
                BKHLTY_HAYLOFT, //
            ),
        ]
    );
    for line in &listing {
        assert!(
            yard.bkhlty_at(line).is_file(),
            "{} reads from the project root",
            line
        );
    }
}

#[test]
fn bkhltu_an_absent_archive_folder_lists_nothing_and_an_unnamed_one_refuses() {
    let absent = bkhlty_Yard::bkhlty_compose("listabsent", true, BKHLTY_OUTPUT_APART);
    let ground = zbkhltu_ground(&absent);
    assert_eq!(
        bkhlru_listing(&ground).expect("the listing reads"),
        Vec::<String>::new()
    );
    drop(absent);

    let unnamed = bkhlty_Yard::bkhlty_compose("listunnamed", false, BKHLTY_OUTPUT_APART);
    let ground = zbkhltu_ground(&unnamed);
    let why = bkhlru_listing(&ground).expect_err("no archive folder is named");
    assert!(why.contains(BKHLRG_HAYLOFT_KEY), "{}", why);
}

#[test]
fn bkhltu_an_archive_unpacks_into_an_absent_target_keeping_times_and_links() {
    let yard = bkhlty_Yard::bkhlty_compose("unpack", true, BKHLTY_OUTPUT_APART);
    let ground = zbkhltu_ground(&yard);
    let archive = zbkhltu_baled(&yard, &ground);
    let target = yard.bkhlty_at("../unpacked/fresh");

    let unpacked =
        bkhlru_unbale(&ground, &archive, Path::new("../unpacked/fresh")).expect("the unpack runs");

    assert_eq!(
        unpacked,
        std::fs::canonicalize(&target).expect("the target stands")
    );
    zbkhltu_compare(&unpacked);
    let day =
        bkhlrd_Day::bkhlrd_from_time(bkhlty_mtime(&unpacked.join("murkwell/run-long/plain.txt")));
    assert_eq!(
        day,
        zbkhltu_today().bkhlrd_back(30),
        "the modification time survives"
    );
}

#[test]
fn bkhltu_an_empty_target_is_taken_and_a_full_one_refused() {
    let yard = bkhlty_Yard::bkhlty_compose("targets", true, BKHLTY_OUTPUT_APART);
    let ground = zbkhltu_ground(&yard);
    let archive = zbkhltu_baled(&yard, &ground);

    std::fs::create_dir_all(yard.bkhlty_at("../empty")).expect("the empty target composes");
    bkhlru_unbale(&ground, &archive, Path::new("../empty")).expect("an empty target is taken");
    zbkhltu_compare(&yard.bkhlty_at("../empty"));

    let why =
        bkhlru_unbale(&ground, &archive, Path::new("../empty")).expect_err("a full target refuses");
    assert!(why.contains("not empty"), "{}", why);

    yard.bkhlty_plant("../afile", b"x", zbkhltu_today());
    let why =
        bkhlru_unbale(&ground, &archive, Path::new("../afile")).expect_err("a file is no target");
    assert!(why.contains("no folder"), "{}", why);
}

#[test]
fn bkhltu_a_target_inside_a_live_folder_is_refused() {
    let yard = bkhlty_Yard::bkhlty_compose("livetarget", true, BKHLTY_OUTPUT_APART);
    let ground = zbkhltu_ground(&yard);
    let archive = zbkhltu_baled(&yard, &ground);

    for target in ["../quillery/out", "../murkwell", "../murkwell/deeper/out"] {
        let why = bkhlru_unbale(&ground, &archive, Path::new(target))
            .expect_err("a live folder is refused");
        assert!(why.contains("live folders"), "{}", why);
        assert!(!yard.bkhlty_at(target).exists() || target == "../murkwell");
    }
}

#[test]
fn bkhltu_the_machines_own_tar_unpacks_an_archive_whole() {
    let yard = bkhlty_Yard::bkhlty_compose("othertar", true, BKHLTY_OUTPUT_APART);
    let ground = zbkhltu_ground(&yard);
    let archive = zbkhltu_baled(&yard, &ground);
    let target = yard.bkhlty_at("../by-tar");
    std::fs::create_dir_all(&target).expect("the target composes");

    let status = Command::new("tar")
        .arg("-xzf")
        .arg(&archive)
        .arg("-C")
        .arg(&target)
        .status()
        .expect("the machine's tar runs");

    assert!(status.success(), "tar exited {:?}", status.code());
    zbkhltu_compare(&target);
}

// eof
