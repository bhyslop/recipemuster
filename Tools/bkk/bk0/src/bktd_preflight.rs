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

//! The preflight's hurdles, and the door lookup's.
//!
//! NOTHING HERE WRITES THE PROCESS ENVIRONMENT. Every reading under test takes
//! its station fact as a value, which is why it can be posed: a hurdle that
//! wrote the environment would be deciding the answers of every neighbour the
//! harness is running beside it, and that has already reddened a hurdle three
//! files away that passed alone and failed in company.

use super::bkcd_preflight::{
    bkcd_Absence,
    bkcd_preflight_at,
    bkcd_unready,
    bkcd_unsupplied, //
};
use super::bkce_election::{
    bkce_Geography,
    bkce_seat, //
};
use super::bkco_output::{
    bkco_call,
    bkco_colophon,
    bkco_tabtarget_in,
    BKCO_COLOPHON_DERBY,
    BKCO_COLOPHON_HEEL,
    BKCO_COLOPHON_REBED,
    BKCO_VERB_HEEL,
    BKCO_VERB_REBED, //
};
use super::bkcr_resolve::bkcr_Collar;
use super::bkcx_python::{
    bkcx_Collar,
    bkcx_Seat,
    bkcx_resolve,
    bkcx_seat_at,
    BKCX_UV, //
};
use super::bkte_election::bkte_plant_at;
use super::bkth_heel::bkth_feedroom;
use super::bktr_resolve::bktr_lured;
use super::bktu_lure::{
    bktu_Bent,
    bktu_Lure, //
};
use bkp::bkprc_catena::bkprc_admit;
use std::path::{
    Path,
    PathBuf, //
};

/// A conforming app collar whose residence a hurdle can put a file at or leave
/// empty. Spelled here rather than composed by the lure surface because what is
/// under test is the RESIDENCE READING, and the collar is the shortest honest
/// statement of one.
const ZBKTD_APP: &str = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Cargo.toml\"
BKRR_TARGET=\"crib\"
BKRR_ROOTS=\"src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_STRIKE=\"vow-b\"
BKRR_RESIDENCE=\"kennel/bin\"
BKRR_BYNAME=\"manger\"
";

/// The same collar declaring no striking door, so the preflight's fallback arm
/// can be read.
const ZBKTD_APP_UNSTRUCK: &str = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Cargo.toml\"
BKRR_TARGET=\"crib\"
BKRR_ROOTS=\"src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"kennel/bin\"
BKRR_BYNAME=\"manger\"
";

fn zbktd_collar(declared: &str) -> bkcr_Collar {
    bkcr_Collar {
        name: "app-lure".to_string(),
        instance: PathBuf::from("app-lure"),
        regime: bkprc_admit(declared, "composed").expect("the collar stands inside the subset"),
    }
}

/// The geography every reading below is posed under: a main checkout, one
/// residence. The dispatched arm is the election's own hurdles' to drive; what
/// is under test here is the preflight's framing of the verdict, and one
/// geography is enough to reach every shape of it.
static ZBKTD_SELF: bkce_Geography = bkce_Geography::Undispatched;

/// A suite collar declaring a struck exergue, so the exergue arm is read over a
/// collar the residence arm never touches. The file it declares stands under the
/// elected root and is ignored by the lure, as every struck exergue in the estate
/// is: generated, never committed, and so never part of the tree the concord
/// compares.
const ZBKTD_SUITE_STRUCK: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"src/gen_exergue.rs\"
BKRR_STRIKE=\"tt/lure-b.Strike.sh\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
";

/// The exergue the suite collar above declares, and the door that strikes it.
const ZBKTD_STRUCK_FILE: &str = "src/gen_exergue.rs";
const ZBKTD_STRIKE_DOOR: &str = "tt/lure-b.Strike.sh";

fn zbktd_suite(declared: &str) -> bkcr_Collar {
    bkcr_Collar {
        name: "suite-lure".to_string(),
        instance: PathBuf::from("suite-lure"),
        regime: bkprc_admit(declared, "composed").expect("the collar stands inside the subset"),
    }
}

/// A position the lure resolves: `HEAD`, or a remote-tracking ref.
fn zbktd_rev(lure: &bktu_Lure, rev: &str) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(lure.bktu_root())
        .arg("rev-parse")
        .arg(rev)
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "could not read the lure's {}: {}",
                rev,
                err, //
            )
        });

    assert!(
        out.status.success(),
        "git would not read the lure's {}: {}",
        rev,
        String::from_utf8_lossy(&out.stderr).trim(), //
    );

    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Write an exergue file in the striker's own shape — `Tools/buk/bue_exergue.sh`'s
/// — struck at the position given over the roots given.
///
/// THE SHAPE IS COPIED AND NOT THE STRIKER, because the striker reads the real
/// station and dies on a repository whose trunk never touched the roots; what
/// this hurdle proves is the READING of a file the striker wrote, so the file is
/// composed to the shape the reading consumes. The constant's name is the
/// caller's, as it is in the estate, and this caller spells it under its own
/// family head so it reads as a seated family head rather than a stray; the ancestry
/// block stands so that a reading that mistook it for the roots would fail here
/// rather than in the field.
///
/// THE ELECTION'S HURDLES STRIKE A DELIVERED ROOT THIS WAY, posing the file a
/// borrow candidate's build left beside it, which is why the path is a parameter
/// and the composer a face of this file: one strike serves both, so what a posed
/// exergue says is composed in one place.
pub(crate) fn bktd_strike_at(lure: &bktu_Lure, relative: &str, sha: &str, roots: &[&str]) {
    let mut body = String::from(
        "// GENERATED by a hurdle, in the striker's shape.\n\n\
         /// The newest landed source touch this build was struck from.\n\
         pub const BKTD_STRUCK_EXERGUE: &str = \"",
    );
    body.push_str(sha);
    body.push_str(
        "\";\n\n/// The elected source roots the sha above was measured over.\n\
         pub const BKTD_STRUCK_EXERGUE_ROOTS: &[&str] = &[\n",
    );
    for root in roots {
        body.push_str(&format!(
            "    \"{}\",\n",
            root, //
        ));
    }
    body.push_str("];\n\npub const BKTD_STRUCK_EXERGUE_ANCESTRY: &[&str] = &[\n");
    body.push_str(&format!(
        "    \"{}\",\n",
        sha, //
    ));
    body.push_str("];\n\n// eof\n");

    lure.bktu_write(relative, &body);
}

/// THE CONTROL, AND IT IS THE HURDLE'S OTHER HALF. An absence proven against a
/// tree nothing was ever placed in proves an absence rather than a reading, so
/// every arm below drives this shape first: a lure whose kibble stands at the
/// residence the declaration composes, read as ready.
#[test]
fn bktd_a_kibble_standing_at_its_residence_is_no_absence() {
    let lure = bktu_Lure::bktu_compose("preflight-placed");
    let haynet = lure.bktu_haynet();
    let feedroom = bkth_feedroom(&haynet);
    haynet.bktu_kibble_seat(&lure, ZBKTD_KIBBLE, &lure.bktu_kibble_seal());

    let residence = haynet.bktu_kibble_residence(ZBKTD_KIBBLE);
    std::fs::create_dir_all(residence.parent().expect("a residence has a parent"))
        .expect("the residence directory composes");
    std::fs::write(&residence, "placed").expect("the placed binary writes");

    let absent = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        &[],
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert!(
        absent.is_empty(),
        "a kibble standing at its residence was reported absent: {:?}",
        absent, //
    );
}

/// The same lure with the residence never written: one absence, naming the
/// converge and the tabtarget that performs it.
#[test]
fn bktd_an_unplaced_kibble_is_an_absence_naming_the_converge() {
    let lure = bktu_Lure::bktu_compose("preflight-unplaced");
    let haynet = lure.bktu_haynet();
    let feedroom = bkth_feedroom(&haynet);
    haynet.bktu_kibble_seat(&lure, ZBKTD_KIBBLE, &lure.bktu_kibble_seal());

    let residence = haynet.bktu_kibble_residence(ZBKTD_KIBBLE);
    assert!(
        !residence.is_file(),
        "the control: nothing may stand at {} before the reading",
        residence.display(), //
    );

    let absent = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        &[],
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert_eq!(
        absent.len(),
        1,
        "one kibble stands unplaced and the preflight found {:?}",
        absent, //
    );
    assert_eq!(
        absent[0].at, residence,
        "the absence names a residence other than the one the declaration composes", //
    );
    assert!(
        absent[0].supply.contains(BKCO_VERB_HEEL),
        "the absence does not name the door that places a kibble: {}",
        absent[0].supply, //
    );
    assert_eq!(
        absent[0].heel.as_deref(),
        Some(ZBKTD_KIBBLE),
        "a heel is the whole of a kibble's supply, so the absence hands rebed its name", //
    );
}

/// THE ESTATE'S OWN uv KIBBLE, POSED UNPLACED THROUGH THE EXISTING POSER, which
/// is what parts this arm from the lure-local one above: that one proves the
/// reading over a kibble this surface composed, and this one proves it over the
/// declaration the STATION actually carries, copied rather than restated.
///
/// THE DECLARATION IS THE STATION'S AND THE ROOT IS THE HURDLE'S OWN. The
/// estate's uv is laid through a station seat because it is the station's
/// declaration, and it is read beneath a haynet so that the placed file this arm
/// writes as its control stands nowhere the station looks; moving the version
/// then changes only the path the pin composes, which IS the thing under test.
#[test]
fn bktd_the_estates_uv_kibble_at_a_version_nothing_placed_is_an_absence() {
    let lure = bktu_Lure::bktu_compose("preflight-uv-unplaced");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let station = lure.bktu_stationed();

    station.bktu_uv_seat(&lure);

    let standing = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        &[],
        &[],
    )
    .expect("the preflight reads the estate's own declaration")
    .absent;

    assert_eq!(
        standing.len(),
        1,
        "one kibble stands declared and the preflight found {:?}",
        standing, //
    );

    // THE CONTROL IS COMPOSED FROM WHAT THE READING ITSELF NAMED, which is what
    // keeps it station-independent: the residence is whatever the declaration
    // composes under this lure's own store, so the arm proves a READING rather
    // than the emptiness of a directory nothing ever wrote to.
    let residence = standing[0].at.clone();
    std::fs::create_dir_all(residence.parent().expect("a residence has a parent"))
        .expect("the residence directory composes");
    std::fs::write(&residence, "placed").expect("the placed binary writes");

    let ready = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        &[],
        &[],
    )
    .expect("the preflight reads the estate's own declaration")
    .absent;

    assert!(
        ready.is_empty(),
        "the control: a kibble standing at its residence was reported absent: {:?}",
        ready, //
    );

    station.bktu_uv_seat_unplaced(&lure, ZBKTD_UV_UNPLACED);

    let moved = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        &[],
        &[],
    )
    .expect("the preflight reads a moved declaration")
    .absent;

    assert_eq!(
        moved.len(),
        1,
        "a version nothing has placed was read as standing: {:?}",
        moved, //
    );
    assert!(
        moved[0].at.to_string_lossy().contains(ZBKTD_UV_UNPLACED),
        "the absence names a residence other than the one the moved declaration composes: {}",
        moved[0].at.display(), //
    );
    assert!(
        moved[0].supply.contains(BKCO_VERB_HEEL),
        "the absence does not name the door that places a kibble: {}",
        moved[0].supply, //
    );
}

/// A version the estate does NOT declare, so the residence the moved
/// declaration composes is one nothing has ever placed at.
const ZBKTD_UV_UNPLACED: &str = "0.0.0-unplaced";

/// An app collar whose residence stands nowhere is an absence, and the act it
/// names is the COLLAR'S OWN striking door rather than the kennel's converge.
#[test]
fn bktd_an_absent_app_residence_names_the_collars_own_strike_door() {
    let lure = bktu_Lure::bktu_compose("preflight-app");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let collar = zbktd_collar(ZBKTD_APP);

    let absent = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert_eq!(
        absent.len(),
        1,
        "one app residence stands nowhere and the preflight found {:?}",
        absent, //
    );
    assert_eq!(
        absent[0].at,
        lure.bktu_root().join("kennel/bin/manger"),
        "the absence joins the residence and the byname", //
    );
    assert_eq!(
        absent[0].supply, "vow-b",
        "the act is the collar's declared striking door, read as it stands", //
    );
    assert_eq!(
        absent[0].heel, None,
        "a collar declaring a strike is never handed a heel, so rebed runs its door instead", //
    );
    assert_eq!(
        absent[0].strike,
        vec!["vow-b".to_string()],
        "the strike doors are the collar's declaration, handed to rebed to run", //
    );

    // THE CONTROL: the same collar over a residence that DOES stand, and answers
    // the seat's own position, reads ready — so what the arm above proved is the
    // reading and not the lure's emptiness. A file that merely stood was the
    // control before the preflight asked currency, and it is exactly the
    // artifact the outrun arm below refuses.
    lure.bktu_write("src/one.rs", "// the elected root\n");
    lure.bktu_commit("plant the elected root");
    let seat = bkce_seat(lure.bktu_root(), "src").expect("the elected root has been touched");
    bkte_plant_at(&lure, "kennel/bin/manger", &seat);

    let after = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert!(
        after.is_empty(),
        "an app residence that stands was reported absent: {:?}",
        after, //
    );
}

/// A collar declaring no striking door is handed the kennel's own converge,
/// which is the one act this kit can promise.
#[test]
fn bktd_an_app_declaring_no_strike_door_is_handed_the_converge() {
    let lure = bktu_Lure::bktu_compose("preflight-app-unstruck");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let collar = zbktd_collar(ZBKTD_APP_UNSTRUCK);

    let absent = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert_eq!(
        absent.len(),
        1,
        "one app residence stands nowhere", //
    );
    assert!(
        absent[0].supply.contains(BKCO_VERB_HEEL),
        "a collar declaring no striking door was handed no act at all: {}",
        absent[0].supply, //
    );
    assert_eq!(
        absent[0].heel.as_deref(),
        Some("app-lure"),
        "the converge a strikeless collar is handed is a heel rebed performs", //
    );
    assert!(
        absent[0].strike.is_empty(),
        "a strikeless collar hands rebed no door to run", //
    );
}

/// AN APP RESIDENCE THAT STANDS AND IS OUTRUN IS AN ABSENCE, and its report says
/// it stands. This is the shape the field found: a pull moved the collar's
/// elected roots beneath a binary that still stood, an existence test waved it
/// through, and the hurdles met that binary's own staleness fence instead.
///
/// THE CONTROL IS DRIVEN FIRST, AND IT IS THE SAME BINARY: planted answering the
/// seat's own position it reads ready, so what the arm below proves is the
/// election at work rather than a residence the preflight never liked.
#[test]
fn bktd_an_app_residence_that_stands_outrun_names_the_collars_own_strike_door() {
    let lure = bktu_Lure::bktu_compose("preflight-app-outrun");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let collar = zbktd_collar(ZBKTD_APP);
    let residence = lure.bktu_root().join("kennel/bin/manger");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let struck = bkce_seat(lure.bktu_root(), "src").expect("the elected root has been touched");
    bkte_plant_at(&lure, "kennel/bin/manger", &struck);

    let current = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert!(
        current.is_empty(),
        "the control: a residence answering the seat's position is no absence: {:?}",
        current, //
    );

    lure.bktu_write("src/two.rs", "// the seat moves an elected root\n");
    lure.bktu_commit("move an elected root past the standing binary");

    let outrun = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert_eq!(
        outrun.len(),
        1,
        "one residence stands outrun and the preflight found {:?}",
        outrun, //
    );
    assert_eq!(
        outrun[0].at, residence,
        "the absence names the residence that stands", //
    );
    assert_eq!(
        outrun[0].supply, "vow-b",
        "the act is the collar's declared striking door, as for a residence that is absent", //
    );

    let why = outrun[0]
        .stands
        .as_deref()
        .expect("the absence says the residence stands");
    assert!(
        why.contains("src/two.rs"),
        "the plaint names what moved beneath the binary: {}",
        why, //
    );

    let said = bkcd_unready(&outrun, "tt/bkw-d.Derby.sh", None);
    assert!(
        said.contains("stands at") && !said.contains("nothing stands at"),
        "the report parts the outrun shape from the swept one:\n{}",
        said, //
    );
}

/// A STRUCK EXERGUE THE TRUNK HAS OUTRUN IS AN ABSENCE, judged against the trunk
/// counterpart and never `HEAD`. The file carries the landed reading, so what
/// stales it is a landing on the trunk beneath its roots — and a seat that has
/// not moved at all must see it, which is what the checkout at the end poses:
/// `HEAD` holds the very tree the file was struck from, and the file is outrun
/// regardless.
///
/// THREE READINGS OVER ONE LURE: the file absent, which names the strike doors;
/// the file struck at the trunk's own landing, which is the control and reads
/// ready; and the trunk moved beneath the roots while the seat stands still,
/// which is the arm.
#[test]
fn bktd_a_struck_exergue_the_trunk_has_outrun_names_the_strike_doors() {
    let lure = bktu_Lure::bktu_compose("preflight-exergue-outrun");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let collar = zbktd_suite(ZBKTD_SUITE_STRUCK);
    let struck_at = lure.bktu_root().join(ZBKTD_STRUCK_FILE);

    lure.bktu_git(&["branch", "-M", "trunk"]);
    lure.bktu_write(
        ".gitignore",
        &format!(
            "{}\n",
            ZBKTD_STRUCK_FILE, //
        ),
    );
    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");

    // THE TRUNK COUNTERPART IS A REAL REMOTE — a bare repository beside the lure
    // with its head recorded — because the reading resolves the remote's
    // recorded head exactly as the striker does, and a posed local branch would
    // prove a different reading than the one the estate takes.
    let bare = lure.bktu_temp().join("counterpart.git");
    let spelled = bare.display().to_string();
    lure.bktu_git(&["init", "--quiet", "--bare", &spelled]);
    lure.bktu_git(&["remote", "add", "origin", &spelled]);
    lure.bktu_git(&["push", "--quiet", "origin", "trunk"]);
    lure.bktu_git(&["remote", "set-head", "origin", "trunk"]);

    let absent = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert_eq!(
        absent.len(),
        1,
        "one declared exergue stands nowhere and the preflight found {:?}",
        absent, //
    );
    assert_eq!(
        absent[0].at, struck_at,
        "the absence names the declared file", //
    );
    assert!(
        absent[0].stands.is_none(),
        "an absent file is the swept shape and not the outrun one: {:?}",
        absent[0].stands, //
    );
    assert_eq!(
        absent[0].supply, ZBKTD_STRIKE_DOOR,
        "the act is the collar's declared striking door", //
    );
    assert_eq!(
        absent[0].heel, None,
        "a heel strikes no exergue, so rebed is handed no name for one", //
    );
    assert_eq!(
        absent[0].strike,
        vec![ZBKTD_STRIKE_DOOR.to_string()],
        "the exergue's strike door is handed to rebed to run", //
    );

    let landed = zbktd_rev(&lure, "origin/trunk");
    bktd_strike_at(&lure, ZBKTD_STRUCK_FILE, &landed, &["src"]);

    let current = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert!(
        current.is_empty(),
        "the control: a file struck at the trunk's own landing is no absence: {:?}",
        current, //
    );

    // THE TRUNK MOVES AND THE SEAT DOES NOT. The landing is made on trunk and
    // pushed, and the seat is put back on the branch it stood on, so `HEAD`
    // holds the tree the file was struck from and only the counterpart has
    // moved — a reading against `HEAD` would call this file current.
    lure.bktu_git(&["checkout", "--quiet", "-b", "seat"]);
    lure.bktu_git(&["checkout", "--quiet", "trunk"]);
    lure.bktu_write("src/two.rs", "// the trunk moves an elected root\n");
    lure.bktu_commit("the trunk moves an elected root");
    lure.bktu_git(&["push", "--quiet", "origin", "trunk"]);
    lure.bktu_git(&["checkout", "--quiet", "seat"]);

    let outrun = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        std::slice::from_ref(&collar),
        &[],
    )
    .expect("the preflight reads a conforming tree")
    .absent;

    assert_eq!(
        outrun.len(),
        1,
        "one struck exergue is outrun by the trunk and the preflight found {:?}",
        outrun, //
    );
    assert_eq!(
        outrun[0].at, struck_at,
        "the absence names the file that stands", //
    );
    assert_eq!(
        outrun[0].supply, ZBKTD_STRIKE_DOOR,
        "the act is the collar's declared striking door, as for an absent file", //
    );

    let why = outrun[0]
        .stands
        .as_deref()
        .expect("the absence says the file stands");
    assert!(
        why.contains("origin/trunk") && why.contains("src/two.rs"),
        "the plaint names the counterpart and what moved beneath the roots: {}",
        why, //
    );
}

/// A hand act no converge performs: a moved lock's re-derivation and its
/// notch, the one shape rebed hands back whole.
const ZBKTD_HAND_ACT: &str = "gangline lure-py, and the lock it writes taken into the tree";

/// THE REPORT NAMES EVERY ABSENCE, THEN REBED AS THE ONE CONVERGE, THEN THE
/// RE-RUN, which is what parts it from a first-absence refusal that hands its
/// reader one repair at a time — and from a list of acts to type.
///
/// EVERY ABSENCE KEEPS THE TRUTH OF ITS SUPPLY: a heel by its verb and collar,
/// a strike by the doors the collar names, each as rebed's to perform, and a
/// hand act no converge performs handed over whole. No heel door is handed to
/// type.
///
/// AND IT RENDERS THE TWO SHAPES AS TWO SENTENCES, so a reader who meets the
/// swept one every morning cannot read the outrun one as more of the same.
#[test]
fn bktd_the_unready_report_names_every_absence_then_rebed_then_the_rerun() {
    let absent = vec![
        bkcd_Absence {
            subject: "the kibble 'bki_uv'".to_string(),
            at: PathBuf::from("/tackroom/bkk/bki_uv/0.9.9/uv"),
            stands: None,
            supply: bkco_call(None, BKCO_VERB_HEEL, Some("bki_uv")),
            heel: Some("bki_uv".to_string()),
            strike: Vec::new(),
        },
        bkcd_Absence {
            subject: "the app collar 'app-lure'".to_string(),
            at: PathBuf::from("/seat/kennel/bin/manger"),
            stands: None,
            supply: "tt/vow-b.Build.sh".to_string(),
            heel: None,
            strike: vec!["tt/vow-b.Build.sh".to_string()],
        },
        bkcd_Absence {
            subject: "the app collar 'app-outrun'".to_string(),
            at: PathBuf::from("/seat/kennel/bin/outrun"),
            stands: Some("the trees differ under the elected roots: src/two.rs".to_string()),
            supply: "tt/vow-b.Build.sh".to_string(),
            heel: None,
            strike: vec!["tt/vow-b.Build.sh".to_string()],
        },
        bkcd_Absence {
            subject: "the environment of the python collar 'lure-py'".to_string(),
            at: PathBuf::from("/stall/bkk/lure-py"),
            stands: Some("its lock no longer answers its manifest".to_string()),
            supply: ZBKTD_HAND_ACT.to_string(),
            heel: None,
            strike: Vec::new(),
        },
    ];

    let said = bkcd_unready(&absent, "tt/bkw-d.Derby.sh", None);

    for wanted in [
        "bki_uv",
        "app-lure",
        "/tackroom/bkk/bki_uv/0.9.9/uv",
        "/seat/kennel/bin/manger",
        "tt/bkw-d.Derby.sh",
    ] {
        assert!(
            said.contains(wanted),
            "the report does not carry '{}':\n{}",
            wanted,
            said, //
        );
    }

    assert!(
        said.contains(&format!(
            "supplied by: {}, which performs {} bki_uv",
            BKCO_VERB_REBED,
            BKCO_VERB_HEEL, //
        )),
        "a heel's absence names the heel as rebed's:\n{}",
        said, //
    );
    assert!(
        said.contains(&format!(
            "supplied by: {}, which runs tt/vow-b.Build.sh",
            BKCO_VERB_REBED, //
        )),
        "a strike's absence names the collar's door as rebed's to run:\n{}",
        said, //
    );
    assert!(
        said.contains(&format!(
            "supplied by: {}",
            ZBKTD_HAND_ACT, //
        )),
        "an act no converge performs is handed over whole:\n{}",
        said, //
    );
    assert!(
        said.contains(&bkco_call(None, BKCO_VERB_REBED, None))
            && said.contains(BKCO_COLOPHON_REBED),
        "the closing names rebed's door as the one converge:\n{}",
        said, //
    );
    assert!(
        !said.contains(BKCO_COLOPHON_HEEL),
        "no heel door is handed to type:\n{}",
        said, //
    );
    assert!(
        said.contains("nothing stands at /seat/kennel/bin/manger"),
        "the swept shape renders as nothing standing:\n{}",
        said, //
    );
    assert!(
        said.contains(
            "stands at /seat/kennel/bin/outrun, and may not be presumed current: the trees differ"
        ),
        "the outrun shape renders as standing, with its plaint:\n{}",
        said, //
    );
}

/// REBED'S REPORT IS THE SAME LIST UNDER ITS OWN LEAD. What it names after its
/// heels and strikes is what neither supplies, so it carries every absence, its
/// act and the re-run exactly as the derby's does — and never the derby's claim
/// that the walk never provisions, which a converge that just provisioned
/// cannot make.
#[test]
fn bktd_the_unsupplied_report_names_every_absence_under_its_own_lead() {
    let absent = vec![bkcd_Absence {
        subject: "the environment of the python collar 'lure-py'".to_string(),
        at: PathBuf::from("/stall/bkk/lure-py"),
        stands: None,
        supply: ZBKTD_HAND_ACT.to_string(),
        heel: None,
        strike: Vec::new(),
    }];

    let rebed = bkco_call(None, BKCO_VERB_REBED, None);
    let said = bkcd_unsupplied(&absent, &rebed);
    let derby = bkcd_unready(&absent, "tt/bkw-d.Derby.sh", None);

    let wanted = [
        "lure-py".to_string(),
        "nothing stands at /stall/bkk/lure-py".to_string(),
        format!(
            "supplied by: {}",
            ZBKTD_HAND_ACT, //
        ),
        format!(
            "then, once every one of them stands: {}",
            rebed, //
        ),
    ];
    for wanted in &wanted {
        assert!(
            said.contains(wanted.as_str()),
            "the report does not carry '{}':\n{}",
            wanted,
            said, //
        );
    }
    assert!(
        said.contains("after every heel and strike"),
        "the lead says what remains is past every heel and strike:\n{}",
        said, //
    );
    assert!(
        derby.contains("never provisions") && !said.contains("never provisions"),
        "only the derby's lead says the walk never provisions:\n{}\n{}",
        derby,
        said, //
    );
}

/// THE LOOKUP CARRIES THE COLOPHON AND NEVER A FRONTISPIECE, which is the whole
/// reason it exists: the colophon is the durable half of a tabtarget's name and
/// the frontispiece is prose that may be reworded.
#[test]
fn bktd_the_door_lookup_carries_the_colophon_through_either_arm() {
    let lure = bktu_Lure::bktu_compose("preflight-lookup");
    let tabtargets = lure.bktu_temp().join("tt");
    std::fs::create_dir_all(&tabtargets).expect("the tabtarget directory composes");

    // THE DEGRADED ARM FIRST, over a directory holding nothing.
    assert_eq!(
        bkco_tabtarget_in(&tabtargets, BKCO_COLOPHON_HEEL),
        None,
        "a colophon resolved against an empty directory named a file", //
    );

    std::fs::write(tabtargets.join("bkw-h.Heel.sh"), "#!/bin/bash\n").expect("the door writes");
    std::fs::write(tabtargets.join("bkw-d.Derby.sh"), "#!/bin/bash\n").expect("the door writes");
    std::fs::write(tabtargets.join("bkw-h.txt"), "not a door\n").expect("the decoy writes");

    let resolved =
        bkco_tabtarget_in(&tabtargets, BKCO_COLOPHON_HEEL).expect("the standing door resolves");

    assert!(
        resolved.starts_with(&format!(
            "{}.",
            BKCO_COLOPHON_HEEL, //
        )),
        "the resolved name does not lead with the colophon: {}",
        resolved, //
    );
    assert!(
        resolved.ends_with(".sh"),
        "the resolved name is no tabtarget: {}",
        resolved, //
    );

    // TWO MATCHES IS AN AMBIGUITY NO CALLER MAY RESOLVE on the reader's behalf.
    std::fs::write(tabtargets.join("bkw-h.Heeled.sh"), "#!/bin/bash\n").expect("the twin writes");

    assert_eq!(
        bkco_tabtarget_in(&tabtargets, BKCO_COLOPHON_HEEL),
        None,
        "a colophon standing at two doors resolved to one of them", //
    );

    // AND A NEIGHBOUR IS NOT REACHED BY IT.
    assert_eq!(
        bkco_tabtarget_in(&tabtargets, BKCO_COLOPHON_DERBY).as_deref(),
        Some("bkw-d.Derby.sh"),
        "the derby's colophon resolved to something else", //
    );
}

/// Every verb the roster carries wears a colophon, and a word that is no door
/// of this kennel wears none.
#[test]
fn bktd_every_rostered_verb_wears_a_colophon() {
    assert_eq!(
        bkco_colophon(BKCO_VERB_HEEL),
        Some(BKCO_COLOPHON_HEEL),
        "the converge's verb resolves to no colophon", //
    );
    assert_eq!(
        bkco_colophon("fetch"),
        None,
        "a word this kennel does not dispatch resolved to a door", //
    );

    // A VERB NO ROSTER CARRIES RENDERS AS ITSELF rather than dying: this
    // composes operator-facing text, and a refusal that died while explaining a
    // refusal would lose the condition it was called to report.
    assert_eq!(
        bkco_call(None, "fetch", Some("bki_uv")),
        "fetch",
        "an unrostered verb composed something other than itself", //
    );
}

/// The composed act carries the verb, the tabtarget and the imprint, in that
/// order — the verb so a reader who knows the kennel reads the word, the
/// tabtarget so a reader who does not has a file to run.
#[test]
fn bktd_a_composed_act_carries_the_verb_beside_the_door() {
    let said = bkco_call(None, BKCO_VERB_HEEL, Some("bki_uv"));

    assert!(
        said.starts_with(BKCO_VERB_HEEL),
        "the composed act does not lead with the verb: {}",
        said, //
    );
    assert!(
        said.contains(BKCO_COLOPHON_HEEL),
        "the composed act does not carry the colophon: {}",
        said, //
    );
    assert!(
        said.ends_with("bki_uv"),
        "the composed act does not carry the imprint: {}",
        said, //
    );

    let bare = bkco_call(None, BKCO_VERB_HEEL, None);
    assert!(
        bare.contains(BKCO_COLOPHON_HEEL) && !bare.ends_with(' '),
        "an act taking no imprint composed a trailing column: {}",
        bare, //
    );
}

/// The reach a composed act names is repository-relative and never the absolute
/// directory the dispatch exported: what a reader types is `tt/<file>`.
#[test]
fn bktd_a_composed_act_names_a_reach_a_reader_can_type() {
    let said = bkco_call(None, BKCO_VERB_HEEL, Some("bki_uv"));
    let door = said
        .split_whitespace()
        .find(|word| word.contains(BKCO_COLOPHON_HEEL))
        .expect("the composed act carries a door");

    assert!(
        door.starts_with("tt/") && !Path::new(door).is_absolute(),
        "the composed act names an absolute path rather than a reach: {}",
        door, //
    );
}

/// The python collar the environment arms below compose.
const ZBKTD_POSED_COLLAR: &str = "suite-lure";

/// A python course over a conforming project, its seat posed under the lure's
/// own loosebox where nothing has ever been converged.
fn zbktd_python_course(lure: &bktu_Lure) -> (bkcx_Collar, bkcx_Seat) {
    lure.bktu_project("project", ZBKTD_POSED_COLLAR, &bktu_Bent::default());

    let collar = bkcx_resolve(lure.bktu_root(), &bktr_lured(lure), ZBKTD_POSED_COLLAR)
        .expect("the composed python collar resolves")
        .collar;

    let seat = bkcx_seat_at(
        &lure.bktu_loosebox().join("posed"),
        &lure.bktu_root().join("tackroom"),
        &collar,
    );

    (collar, seat)
}

/// Pose an environment's own record, carrying the interpreter version given.
fn zbktd_marqued(seat: &bkcx_Seat, version: &str) {
    std::fs::create_dir_all(&seat.environment).expect("the posed environment composes");
    std::fs::write(
        seat.environment.join("pyvenv.cfg"),
        format!(
            "home = /posed\nversion_info = {}\n",
            version, //
        ),
    )
    .expect("the posed record writes");
}

/// A python course whose environment stands nowhere is one absence: the
/// environment named where the converge would write it, nothing standing there,
/// and the converge over that collar named as the supplying act. The reading
/// takes the environment's own record and spawns nothing to learn it.
#[test]
fn bktd_an_absent_python_environment_is_an_absence_naming_the_converge() {
    let lure = bktu_Lure::bktu_compose("preflight-python-absent");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let (collar, seat) = zbktd_python_course(&lure);

    let absent = bkcd_preflight_at(
        lure.bktu_root(),
        &bktr_lured(&lure),
        &feedroom,
        &ZBKTD_SELF,
        &[],
        &[(&collar, &seat)],
    )
    .expect("the preflight reads a tree carrying a python course")
    .absent;

    assert_eq!(
        absent.len(),
        1,
        "one environment stands nowhere and the preflight found {:?}",
        absent, //
    );
    assert_eq!(
        absent[0].at, seat.environment,
        "the absence names a place other than where the converge writes the environment", //
    );
    assert_eq!(
        absent[0].stands,
        None,
        "nothing stands, so nothing is said to stand: {:?}",
        absent[0], //
    );
    assert_eq!(
        absent[0].supply,
        bkco_call(None, BKCO_VERB_HEEL, Some(ZBKTD_POSED_COLLAR)),
        "the absence does not name the converge over its own collar", //
    );
    assert_eq!(
        absent[0].heel.as_deref(),
        Some(ZBKTD_POSED_COLLAR),
        "an environment whose lock still answers is supplied by its heel alone", //
    );
}

/// WHERE UV ITSELF IS UNPLACED, THE PYTHON READING STOPS AT THE ENVIRONMENT'S
/// RECORD. Every reading past it reaches uv at its residence, and uv's absence
/// already stands in the report, so the lock and the environment are left for
/// the drive after uv is placed rather than refused as though the fault were
/// theirs.
///
/// THE CONTROL IS A RECORD THAT DISAGREES WITH THE PIN: read and reported beside
/// uv's absence, so the reading is proven to run. A record that answers the pin
/// then leaves uv's absence standing alone — where a reading that went on to
/// reach uv would have reported the lock it could not have read.
#[test]
fn bktd_an_unplaced_uv_leaves_the_python_reading_at_the_environments_record() {
    let lure = bktu_Lure::bktu_compose("preflight-python-uvless");
    let feedroom = bkth_feedroom(&lure.bktu_haynet());
    let (collar, seat) = zbktd_python_course(&lure);
    lure.bktu_stationed().bktu_uv_seat(&lure);

    let read = || {
        bkcd_preflight_at(
            lure.bktu_root(),
            &bktr_lured(&lure),
            &feedroom,
            &ZBKTD_SELF,
            &[],
            &[(&collar, &seat)],
        )
        .expect("the preflight reads a tree carrying a python course")
        .absent
    };

    zbktd_marqued(&seat, "3.11.9");
    let moved = read();

    assert_eq!(
        moved.len(),
        2,
        "uv and the moved environment were each to be reported: {:?}",
        moved, //
    );
    assert!(
        moved
            .iter()
            .any(|absence| absence.subject.contains(BKCX_UV)),
        "uv's own absence is not in the report: {:?}",
        moved, //
    );
    assert!(
        moved.iter().any(|absence| absence.at == seat.environment
            && absence
                .stands
                .as_deref()
                .is_some_and(|why| why.contains("3.11.9"))),
        "the control: the environment's record was not read: {:?}",
        moved, //
    );

    zbktd_marqued(&seat, "3.12.7");
    let answering = read();

    assert_eq!(
        answering.len(),
        1,
        "a record answering the pin, with uv unplaced, reported more than uv's absence: {:?}",
        answering, //
    );
    assert!(
        answering[0].subject.contains(BKCX_UV),
        "the one absence is not uv's: {:?}",
        answering[0], //
    );
}

/// The kibble name the haynet arms pose, which keys nothing: the residence is
/// keyed on the seat.
const ZBKTD_KIBBLE: &str = "kibble-haynet";

// eof
